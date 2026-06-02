use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::auth::_users::permission::Permission;
use crate::domain::auth::_users::pk::UserPk;
use crate::domain::auth::_users::role::{Role, RoleAccess};
use crate::domain::auth::app_error::AppError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub unid: UserPk,
    pub created: OffsetDateTime,
    pub first_name: String,
    pub last_name: String,
    pub last_password_change: OffsetDateTime,
    pub email: String,
    pub password: String,
    pub roles: HashSet<RoleAccess>,
}

impl User {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn has_any_role(&self, roles: &[Role]) -> bool {
        self.roles.iter().any(|ra| roles.contains(&ra.role))
    }

    pub fn check_permission<P: Permission>(&self, permission: &P) -> Result<(), AppError> {
        permission.check_permission(self)
    }
}
