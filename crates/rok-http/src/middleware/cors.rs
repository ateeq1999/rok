//! CORS middleware helpers.

use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

/// A permissive CORS layer suitable for development.
///
/// Allows any origin, GET / POST / PUT / DELETE / OPTIONS, and any header.
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
}

/// A stricter CORS layer that only allows the given origin.
pub fn cors_layer_for(origin: &str) -> CorsLayer {
    let origin: HeaderValue = origin
        .parse()
        .expect("invalid origin header value");
    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}
