//! [`App`] — the main application builder.

use axum::Router;
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::middleware::{cors_layer, request_id_layer};

/// Application builder.
///
/// Wraps an Axum [`Router`] and wires the default middleware stack:
/// request-id → tracing → CORS → gzip compression.
///
/// ```rust,no_run
/// use rok_http::App;
/// use axum::{routing::get, Json};
///
/// async fn ping() -> &'static str { "pong" }
///
/// #[tokio::main]
/// async fn main() {
///     App::new()
///         .route("/ping", axum::routing::get(ping))
///         .serve("127.0.0.1:3000")
///         .await
///         .unwrap();
/// }
/// ```
pub struct App {
    router: Router,
}

impl App {
    /// Create a new [`App`] with the default middleware stack.
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    /// Add a route (forwarded to the inner [`Router`]).
    pub fn route(mut self, path: &str, method_router: axum::routing::MethodRouter) -> Self {
        self.router = self.router.route(path, method_router);
        self
    }

    /// Merge an external [`Router`] into this app.
    pub fn router(mut self, router: Router) -> Self {
        self.router = self.router.merge(router);
        self
    }

    /// Build the final [`Router`] with the default middleware applied.
    pub fn build(self) -> Router {
        self.router
            .layer(CompressionLayer::new())
            .layer(cors_layer())
            .layer(TraceLayer::new_for_http())
            .layer(request_id_layer())
    }

    /// Bind and serve on `addr`, blocking until shutdown.
    pub async fn serve(self, addr: &str) -> anyhow::Result<()> {
        let app = self.build();
        let addr: SocketAddr = addr.parse()?;
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("listening on {addr}");
        axum::serve(listener, app).await?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
