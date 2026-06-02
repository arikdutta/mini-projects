use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::app_error::AppError;

pub trait AccessController {
    fn check_permission(_user: &User) -> Result<(), AppError> {
        Err(AppError::Unauthorized)
    }
}
