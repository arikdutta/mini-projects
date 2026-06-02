use leptos::prelude::*;

use crate::domain::auth::_users::data::user::User;

#[server(GetUser, "/api/user")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::domain::auth::auth_context::auth;

    let auth = auth()?;

    Ok(auth.current_user)
}
