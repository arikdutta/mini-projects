use async_trait::async_trait;
use axum_session_auth::{Authentication, HasPermission};
use leptos::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::data::users_db::UsersDb;
use crate::domain::auth::auth_session::AuthSession;

pub fn auth() -> Result<AuthSession, ServerFnError> {
    use_context::<AuthSession>()
        .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
}

#[async_trait]
impl Authentication<User, Uuid, PgPool> for User {
    async fn load_user(user_unid: Uuid, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        let pool = pool.expect("Pool is None");

        UsersDb::get_by_unid(pool, user_unid.into())
            .await
            .map_err(|e| anyhow::anyhow!("Cannot get user: {}", e))?
            .ok_or_else(|| anyhow::anyhow!("User not found"))
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    // Required by Authentication trait. Cannot be removed.
    // All users are considered active (no status field exists).
    fn is_active(&self) -> bool {
        true
    }

    // Required by Authentication trait. Cannot be removed.
    // All loaded users are considered non-anonymous.
    fn is_anonymous(&self) -> bool {
        false
    }
}

#[async_trait]
impl HasPermission<PgPool> for User {
    async fn has(&self, _role: &str, _pool: &Option<&PgPool>) -> bool {
        false
    }
}

#[server(IsAuthenticated, "/api/user")]
pub async fn is_authenticated() -> Result<bool, ServerFnError> {
    let auth = auth()?;

    Ok(auth.is_authenticated())
}
