use std::future::Future;

use axum::extract::FromRef;
use leptos::prelude::{LeptosOptions, *};
use sqlx::Postgres;
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPool;

use crate::common::errors::app_error::AppError;
use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::auth_session::AuthSession;

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
}

impl AppState {
    pub async fn db_exec<R, E, F, Fut>(&self, f: F) -> Result<R, AppError>
    where
        E: Into<AppError>,
        F: FnOnce(PoolConnection<Postgres>, User) -> Fut,
        Fut: Future<Output = Result<R, E>>,
    {
        // Get the authenticated user from session
        let auth_session = get_session()?;
        let user = auth_session.current_user.ok_or(AppError::MissingContext(
            "Failed to retrieve user session".to_string(),
        ))?;

        // Get database connection
        let connection = match self.pool.acquire().await {
            Ok(conn) => conn,
            Err(err) => {
                return Err(AppError::database_error(err.to_string()));
            }
        };

        f(connection, user).await.map_err(|err| err.into())
    }
}

pub fn use_app_state() -> Result<AppState, ServerFnError> {
    use_context::<AppState>().ok_or_else(|| ServerFnError::ServerError("App state missing.".into()))
}

pub fn get_session() -> Result<AuthSession, AppError> {
    use_context::<AuthSession>()
        .ok_or_else(|| AppError::MissingContext("Auth session missing.".into()))
}
