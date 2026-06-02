use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::response::{IntoResponse, Redirect, Response};
use http::Request;
use tower::{Layer, Service};

use crate::AuthSession;

const PROTECTED_PATHS: [&str; 1] = ["/"]; // This protects ALL routes since every path starts with /
const PUBLIC_PATHS: [&str; 2] = ["/login", "/api"]; // Works only when we just have /api, not /api/login

#[derive(Clone, Default)]
pub struct AuthMiddlewareLayer;

impl AuthMiddlewareLayer {
    pub const fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for AuthMiddlewareLayer {
    type Service = AuthMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            let path = req.uri().path();

            let is_authenticated = match req.extensions().get::<AuthSession>() {
                Some(session) => session.is_authenticated(),
                None => false,
            };

            if should_redirect_to_home(path, is_authenticated) {
                return Ok(Redirect::to("/").into_response());
            }

            if should_redirect_to_auth(path, is_authenticated) {
                return Ok(Redirect::to("/login").into_response());
            }

            inner.call(req).await
        })
    }
}

fn should_redirect_to_home(path: &str, is_authenticated: bool) -> bool {
    path == "/login" && is_authenticated
}

fn should_redirect_to_auth(path: &str, is_authenticated: bool) -> bool {
    if is_authenticated {
        return false;
    }

    // Allow homepage access without authentication
    if path == "/" {
        return false;
    }

    !PUBLIC_PATHS
        .iter()
        .any(|public_path| path.starts_with(public_path))
        && PROTECTED_PATHS
            .iter()
            .any(|protected_path| path.starts_with(protected_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_redirect_to_home() {
        // Not authorized
        assert!(!should_redirect_to_home("/login", false));

        // Authorized
        assert!(should_redirect_to_home("/login", true));

        // Not `login` url
        assert!(!should_redirect_to_home("/valid_url", false));
        assert!(!should_redirect_to_home("/valid_url", true));
    }

    #[test]
    fn test_should_redirect_to_auth() {
        // Not authorized
        assert!(should_redirect_to_auth("/", false));
        assert!(should_redirect_to_auth("/valid_url", false));

        // Authorized
        assert!(!should_redirect_to_auth("/", true));
        assert!(
            !should_redirect_to_auth("/login", false),
            "Unauthenticated users should not redirect from login"
        );

        // Test public paths - unauthenticated users should NOT be redirected from public paths
        assert!(!should_redirect_to_auth("/api/user", false));

        // Test non-public API paths - unauthenticated users SHOULD be redirected
        assert!(should_redirect_to_auth("/api/protected", false));
    }
}
