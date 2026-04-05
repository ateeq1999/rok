//! CORS middleware helpers.

use axum::http::{HeaderValue, Method};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

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

/// A strict CORS layer that only allows the given single origin.
pub fn cors_layer_for(origin: &str) -> CorsLayer {
    let origin: HeaderValue = origin.parse().expect("invalid origin header value");
    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}

/// A strict CORS layer that allows any of the given origins.
///
/// Useful for allowing both `http://localhost:3000` and a production domain.
///
/// ```rust
/// use rok_http::middleware::cors_layer_for_origins;
///
/// let layer = cors_layer_for_origins(&["https://example.com", "https://app.example.com"]);
/// ```
pub fn cors_layer_for_origins(origins: &[&str]) -> CorsLayer {
    let values: Vec<HeaderValue> = origins
        .iter()
        .map(|o| o.parse().expect("invalid origin header value"))
        .collect();
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(values))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}
