//! rok-http — Axum 0.8 HTTP layer for the rok ecosystem.
//!
//! ```rust,no_run
//! use rok_http::App;
//! use axum::{routing::get, Json};
//!
//! async fn health() -> Json<serde_json::Value> {
//!     Json(serde_json::json!({"status": "ok"}))
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     App::new()
//!         .route("/health", axum::routing::get(health))
//!         .serve("0.0.0.0:3000")
//!         .await
//!         .unwrap();
//! }
//! ```

pub mod app;
pub mod error;
pub mod middleware;

pub use app::App;
pub use axum::{
    extract::{Json, Path, Query, State},
    routing, Router,
};
pub use error::AppError;
pub use middleware::AuthLayer;
pub use rok_auth::{AuthConfig, Claims};
