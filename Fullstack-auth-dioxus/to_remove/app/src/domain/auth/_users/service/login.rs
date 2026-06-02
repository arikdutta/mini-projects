use leptos::prelude::*;

#[server(Login, "/api/user")]
pub async fn login(
    user_login: String,
    password: String,
) -> Result<(), ServerFnError> {
    use crate::common::app_state::use_app_state;
    use crate::domain::auth::_users::data::users_db::UsersDb;
    use crate::domain::auth::auth_context::auth;

    let app_state = use_app_state()?;

    let user = match UsersDb::get_from_email(user_login, &app_state.pool).await {
        Some(user) => user,
        None => {
            return Err(ServerFnError::ServerError(
                "User does not exist.".to_string(),
            ));
        }
    };

    match pwhash::unix::verify(password, user.password.as_str()) {
        true => {
            let auth = auth()?;
            auth.login_user(user.unid);
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}
