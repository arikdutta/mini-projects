use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Trait for strongly-typed UUID-based identifiers
///
/// This trait provides a common interface for all entity primary key types
/// that are based on UUIDs. Implementors should be newtype wrappers around `Uuid`.
pub trait Unid:
    Clone
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + PartialEq
    + Eq
    + std::hash::Hash
    + PartialOrd
    + Ord
    + Serialize
    + for<'de> Deserialize<'de>
    + From<Uuid>
    + Into<Uuid>
    + AsRef<Uuid>
    + Default
    + Send
    + Sync
    + 'static
{
    /// Create a new ID with a randomly generated UUID v4
    fn new() -> Self {
        Self::from(Uuid::new_v4())
    }

    /// Check if this is a nil UUID
    fn is_nil(&self) -> bool {
        self.as_ref().is_nil()
    }

    /// Create a nil ID
    fn nil() -> Self {
        Self::from(Uuid::nil())
    }

    /// Create an ID from a string representation of a UUID
    fn from_string(s: &str) -> Result<Self, uuid::Error> {
        let uuid = Uuid::parse_str(s)?;
        Ok(Self::from(uuid))
    }

    /// Get the inner UUID value
    fn to_uuid(self) -> Uuid {
        self.into()
    }

    /// Create a vector of IDs from a vector of UUIDs
    #[cfg(feature = "ssr")]
    fn vec_from_uuids(uuids: Vec<Uuid>) -> Vec<Self> {
        uuids.into_iter().map(Self::from).collect()
    }
}

/// Macro to implement Unid trait for newtype wrappers around Uuid
///
/// This macro generates all the necessary trait implementations for a type
/// to be used as a UUID-based identifier with full sqlx support.
///
/// # Example
/// ```
/// unid_newtype!(CompanyPk);
/// unid_newtype!(UserPk);
/// ```
#[macro_export]
macro_rules! unid_newtype {
    ($name:ident) => {
        #[derive(
            Copy,
            Clone,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(::uuid::Uuid);

        impl $crate::utils::types::unid_access_control::Unid for $name {}

        impl Default for $name {
            fn default() -> Self {
                <Self as $crate::utils::types::unid_access_control::Unid>::new()
            }
        }

        impl From<::uuid::Uuid> for $name {
            fn from(uuid: ::uuid::Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<$name> for ::uuid::Uuid {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl AsRef<::uuid::Uuid> for $name {
            fn as_ref(&self) -> &::uuid::Uuid {
                &self.0
            }
        }

        impl AsMut<::uuid::Uuid> for $name {
            fn as_mut(&mut self) -> &mut ::uuid::Uuid {
                &mut self.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl $crate::utils::types::identifiable::Identifiable for $name {
            type Pk = $name;
            type Id = ::uuid::Uuid;

            fn id(&self) -> Self::Id {
                self.0
            }
        }

        // ==== SQLX support ====
        #[cfg(feature = "ssr")]
        impl ::sqlx::Type<::sqlx::Postgres> for $name {
            fn type_info() -> <::sqlx::Postgres as ::sqlx::Database>::TypeInfo {
                <::uuid::Uuid as ::sqlx::Type<::sqlx::Postgres>>::type_info()
            }
        }

        #[cfg(feature = "ssr")]
        impl ::sqlx::postgres::PgHasArrayType for $name {
            fn array_type_info() -> ::sqlx::postgres::PgTypeInfo {
                <::uuid::Uuid as ::sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }

        #[cfg(feature = "ssr")]
        impl<'r> ::sqlx::Decode<'r, ::sqlx::Postgres> for $name {
            fn decode(
                value: <::sqlx::Postgres as ::sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, ::sqlx::error::BoxDynError> {
                let uuid = <::uuid::Uuid as ::sqlx::Decode<'r, ::sqlx::Postgres>>::decode(value)?;
                Ok($name(uuid))
            }
        }

        #[cfg(feature = "ssr")]
        impl<'r> ::sqlx::Encode<'r, ::sqlx::Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut <::sqlx::Postgres as ::sqlx::Database>::ArgumentBuffer<'r>,
            ) -> Result<::sqlx::encode::IsNull, ::sqlx::error::BoxDynError> {
                <::uuid::Uuid as ::sqlx::Encode<::sqlx::Postgres>>::encode_by_ref(&self.0, buf)
            }
        }
    };
}

