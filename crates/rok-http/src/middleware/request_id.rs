//! Request-ID middleware — stamps every request with `x-request-id`.

use axum::http::{HeaderName, HeaderValue, Request};
use std::task::{Context, Poll};
use tower::{Layer, Service};

static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

/// Return a Tower layer that injects `x-request-id` into each request.
pub fn request_id_layer() -> RequestIdLayer {
    RequestIdLayer
}

#[derive(Clone)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdService<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for RequestIdService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        // Use an existing header if present, otherwise generate a simple id.
        if !req.headers().contains_key(&X_REQUEST_ID) {
            let id = new_id();
            if let Ok(v) = HeaderValue::from_str(&id) {
                req.headers_mut().insert(X_REQUEST_ID.clone(), v);
            }
        }
        self.inner.call(req)
    }
}

fn new_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    format!("req-{ns:08x}")
}
