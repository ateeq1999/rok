//! JWT bearer-token auth guard middleware.
//!
//! Extracts `Authorization: Bearer <token>`, verifies it with [`rok_auth::Auth`],
//! and injects the decoded [`rok_auth::Claims`] as a request extension.
//! Requests without a valid token receive `401 Unauthorized`.
//!
//! # Example
//!
//! ```rust,no_run
//! use rok_http::App;
//! use rok_auth::AuthConfig;
//!
//! App::new()
//!     .route("/private", axum::routing::get(|| async { "secret" }))
//!     .with_auth(AuthConfig {
//!         secret: "my-secret".to_string(),
//!         ..Default::default()
//!     });
//! ```

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use rok_auth::{Auth, AuthConfig};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};

// ── Layer ────────────────────────────────────────────────────────────────────

/// Tower [`Layer`] that attaches the auth guard to every request.
#[derive(Clone)]
pub struct AuthLayer {
    auth: Arc<Auth>,
}

impl AuthLayer {
    /// Create from an [`AuthConfig`].
    pub fn new(config: AuthConfig) -> Self {
        Self {
            auth: Arc::new(Auth::new(config)),
        }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            auth: Arc::clone(&self.auth),
        }
    }
}

// ── Service ──────────────────────────────────────────────────────────────────

/// Tower [`Service`] that validates the Bearer token and forwards the request.
#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    auth: Arc<Auth>,
}

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let auth = Arc::clone(&self.auth);
        // `take_inner` pattern: replace inner so the cloned service is ready.
        let mut inner = self.inner.clone();
        std::mem::swap(&mut inner, &mut self.inner);

        Box::pin(async move {
            let token = extract_bearer(req.headers());

            match token.and_then(|t| auth.verify(t).ok()) {
                Some(claims) => {
                    req.extensions_mut().insert(claims);
                    inner.call(req).await
                }
                None => Ok(unauthorized()),
            }
        })
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn extract_bearer(headers: &axum::http::HeaderMap) -> Option<&str> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
}

fn unauthorized() -> Response {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized"))
        .expect("static response is valid")
}
