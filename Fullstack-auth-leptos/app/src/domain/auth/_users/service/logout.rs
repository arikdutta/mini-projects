use leptos::prelude::*;

#[server(Logout, "/api/user")]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::domain::auth::auth_context::auth;

    let auth = auth()?;
    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
