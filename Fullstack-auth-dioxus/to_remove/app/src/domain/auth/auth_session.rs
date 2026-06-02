use axum_session_sqlx::SessionPgPool;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::auth::_users::data::user::User;

pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionPgPool, PgPool>;
