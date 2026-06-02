use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::role::Role;
use crate::domain::auth::app_error::AppError;

pub trait Permission: core::fmt::Debug + Send + Sync {
    fn roles_required(&self) -> Vec<Role>;

    fn has_permission(&self, user: &User) -> bool {
        user.has_any_role(&self.roles_required())
    }

    fn check_permission(&self, user: &User) -> Result<(), AppError> {
        if self.has_permission(user) {
            Ok(())
        } else {
            Err(AppError::Unauthorized)
        }
    }
}
