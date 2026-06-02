use serde::{Deserialize, Serialize};

use crate::common::errors::app_error::AppError;
use crate::domain::auth::_users::data::user::User;

pub trait AccessController {
    type StateParam: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    fn check_permission(_user: &User) -> Result<(), AppError> {
        Err(AppError::Unauthorized)
    }

    #[allow(async_fn_in_trait)]
    async fn check_permission_with_state(
        _user: &User,
        _entity_id: Self::StateParam,
    ) -> Result<(), AppError> {
        Err(AppError::Unauthorized)
    }
}
