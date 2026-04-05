//! [`App`] — the main application builder.

use axum::Router;
use rok_auth::AuthConfig;
use std::future::Future;
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::middleware::{cors_layer, cors_layer_for_origins, request_id_layer, AuthLayer};

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
    auth: Option<AuthConfig>,
    allowed_origins: Option<Vec<String>>,
}

impl App {
    /// Create a new [`App`] with the default middleware stack.
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            auth: None,
            allowed_origins: None,
        }
    }

    /// Enable JWT bearer-token authentication for all routes added after this call.
    ///
    /// Requests missing or carrying an invalid `Authorization: Bearer <token>` header
    /// will receive `401 Unauthorized` before reaching any handler.
    pub fn with_auth(mut self, config: AuthConfig) -> Self {
        self.auth = Some(config);
        self
    }

    /// Restrict CORS to specific allowed origins.
    ///
    /// When set, only the listed origins will be accepted; all others will be
    /// rejected.  Overrides the default permissive CORS policy.
    ///
    /// ```rust,no_run
    /// use rok_http::App;
    ///
    /// App::new()
    ///     .with_cors(&["https://example.com", "https://app.example.com"]);
    /// ```
    pub fn with_cors(mut self, origins: &[&str]) -> Self {
        self.allowed_origins = Some(origins.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Inject shared application state into all routes via [`axum::Extension`].
    ///
    /// Handlers access the value with `Extension(val): Extension<T>`.  For
    /// large state objects wrap in [`std::sync::Arc`] before passing.
    ///
    /// ```rust,no_run
    /// use rok_http::App;
    /// use axum::{routing::get, Extension};
    /// use std::sync::Arc;
    ///
    /// #[derive(Clone)]
    /// struct Db { url: String }
    ///
    /// async fn health(Extension(db): Extension<Arc<Db>>) -> String {
    ///     format!("connected to {}", db.url)
    /// }
    ///
    /// App::new()
    ///     .route("/health", get(health))
    ///     .with_state(Arc::new(Db { url: "postgres://...".into() }));
    /// ```
    ///
    /// If you need typed `State<S>` extractors, build an `axum::Router<S>`,
    /// call `.with_state(s)` on it yourself, then pass it to [`App::router`].
    pub fn with_state<S>(mut self, state: S) -> Self
    where
        S: Clone + Send + Sync + 'static,
    {
        self.router = self.router.layer(axum::Extension(state));
        self
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
        let mut router = self.router;
        if let Some(config) = self.auth {
            router = router.layer(AuthLayer::new(config));
        }
        let cors = if let Some(ref origins) = self.allowed_origins {
            let refs: Vec<&str> = origins.iter().map(String::as_str).collect();
            cors_layer_for_origins(&refs)
        } else {
            cors_layer()
        };
        router
            .layer(CompressionLayer::new())
            .layer(cors)
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

    /// Bind and serve on `addr`, shutting down gracefully when `signal` resolves.
    ///
    /// ```rust,no_run
    /// use rok_http::App;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     App::new()
    ///         .serve_with_shutdown("127.0.0.1:3000", async {
    ///             tokio::signal::ctrl_c().await.unwrap();
    ///         })
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn serve_with_shutdown<F>(self, addr: &str, signal: F) -> anyhow::Result<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let app = self.build();
        let addr: SocketAddr = addr.parse()?;
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("listening on {addr}");
        axum::serve(listener, app).with_graceful_shutdown(signal).await?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
