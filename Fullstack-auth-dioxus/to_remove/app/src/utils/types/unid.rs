use std::fmt::Display;
use std::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{
    Database, Type,
    decode::Decode,
    encode::Encode,
    error::BoxDynError,
    postgres::{PgHasArrayType, Postgres},
};
use uuid::Uuid;

use super::identifiable::Identifiable;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unid<Pk> {
    uuid: Uuid,
    entity_type: PhantomData<Pk>,
}

impl<Pk> AsRef<Uuid> for Unid<Pk> {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}

impl<Pk> AsMut<Uuid> for Unid<Pk> {
    fn as_mut(&mut self) -> &mut Uuid {
        &mut self.uuid
    }
}

impl<Pk> From<Uuid> for Unid<Pk> {
    fn from(uuid: Uuid) -> Self {
        Self {
            uuid,
            entity_type: PhantomData,
        }
    }
}

impl<Pk> Unid<Pk> {
    /// Create a new Unid with a randomly generated UUID v4
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            entity_type: PhantomData,
        }
    }

    /// Check if this is a nil UUID
    pub fn is_nil(&self) -> bool {
        self.uuid.is_nil()
    }

    /// Create a nil Unid
    pub fn nil() -> Self {
        Self {
            uuid: Uuid::nil(),
            entity_type: PhantomData,
        }
    }

    /// Create a Unid from a string representation of a UUID
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        let uuid = Uuid::parse_str(s)?;
        Ok(Self {
            uuid,
            entity_type: PhantomData,
        })
    }

    pub fn to_uuid(self) -> Uuid {
        self.uuid
    }
}

impl<Pk> Display for Unid<Pk> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

impl<Pk> Serialize for Unid<Pk> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Uuid::serialize(&self.uuid, serializer)
    }
}

impl<'de, Pk> Deserialize<'de> for Unid<Pk> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Uuid::deserialize(deserializer).map(|uuid| uuid.into())
    }
}

impl<EPk> Identifiable for Unid<EPk> {
    type Pk = EPk;
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.uuid
    }
}

impl<Pk> Default for Unid<Pk> {
    fn default() -> Self {
        Self::new()
    }
}
pub trait UnidVecExt<Pk> {
    fn to_uuids(self) -> Vec<Uuid>;

    fn contains_uuid(&self, uuid: Uuid) -> bool;
}

impl<Pk> UnidVecExt<Pk> for Vec<Unid<Pk>> {
    fn to_uuids(self) -> Vec<Uuid> {
        self.into_iter().map(|unid| unid.uuid).collect()
    }

    fn contains_uuid(&self, uuid: Uuid) -> bool {
        self.iter().any(|unid| *unid.as_ref() == uuid)
    }
}

// ==== SQLX support ====
#[cfg(feature = "ssr")]
impl<Pk> Type<Postgres> for Unid<Pk> {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <Uuid as Type<Postgres>>::type_info()
    }
}

#[cfg(feature = "ssr")]
impl<Pk> PgHasArrayType for Unid<Pk> {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <Uuid as PgHasArrayType>::array_type_info()
    }
}

#[cfg(feature = "ssr")]
impl<'r, Pk> Decode<'r, Postgres> for Unid<Pk>
where
    Uuid: Decode<'r, Postgres>,
{
    fn decode(value: <Postgres as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let uuid = <Uuid as Decode<'r, Postgres>>::decode(value)?;
        Ok(Unid {
            uuid,
            entity_type: PhantomData,
        })
    }
}

#[cfg(feature = "ssr")]
impl<'r, Pk> Encode<'r, Postgres> for Unid<Pk> {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'r>,
    ) -> Result<sqlx::encode::IsNull, BoxDynError> {
        let uuid = &self.uuid;
        <&Uuid as Encode<Postgres>>::encode(uuid, buf)
    }
}

