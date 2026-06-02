#![recursion_limit = "256"]

use std::env;

use axum_session_sqlx::SessionPgPool;
use dotenv::dotenv;
use leptos::prelude::*;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use uuid::Uuid;

mod app_router;
mod fallback;
mod middleware;

use app::domain::auth::_users::data::user::User;
use app_router::build_app_router::build_app_router;

pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionPgPool, PgPool>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    let subscriber = FmtSubscriber::builder()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        // Apply the EnvFilter to use RUST_LOG, defaulting to info level
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Could not set subscriber");
    dotenv().ok();

    let conf = get_configuration(None)?;

    let addr = conf.leptos_options.site_addr;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await?;

    // Run database migrations
    info!("Running database migrations...");
    sqlx::migrate!("../migrations").run(&pool).await?;
    info!("Database migrations completed successfully! ✔️");

    let app = build_app_router(conf, pool).await?;
    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{HeaderValue, Method, Request, StatusCode, Uri, Version};
    use sqlx::PgPool;
    use tower::ServiceExt;

    use super::*;
    use crate::app_router::build_app_router::build_app_router;

    fn create_request(uri: &str) -> Request<Body> {
        Request::builder()
            .method(Method::GET)
            .uri(uri.parse::<Uri>().unwrap())
            .version(Version::HTTP_11)
            .header(
                axum::http::header::ACCEPT,
                HeaderValue::from_static("application/json"),
            )
            .body(Body::empty())
            .unwrap()
    }

    #[sqlx::test]
    async fn test_unauthenticated_user(pool: PgPool) {
        let conf = get_configuration(None).unwrap();
        let response = build_app_router(conf, pool)
            .await
            .unwrap()
            .oneshot(create_request("/nonexistent"))
            .await
            .unwrap();

        // redirecting to authorize: status code 303
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
    }
}
