use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "user_role"))]
pub enum Role {
    Root,
    Admin,
    Staff,
    RegularUser,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RoleAccess {
    pub role: Role,
}
