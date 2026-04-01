//! Application error type that converts into HTTP responses.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// A unified application error that implements [`axum::response::IntoResponse`].
///
/// Handlers return `Result<T, AppError>` and Axum converts the error into a
/// JSON body with the appropriate HTTP status code.
///
/// ```rust
/// use rok_http::{AppError, Json};
/// use axum::extract::Path;
///
/// async fn get_user(Path(id): Path<u64>) -> Result<Json<serde_json::Value>, AppError> {
///     if id == 0 {
///         return Err(AppError::not_found("user not found"));
///     }
///     Ok(Json(serde_json::json!({"id": id})))
/// }
/// ```
#[derive(Debug, Error)]
pub enum AppError {
    /// 400 Bad Request
    #[error("{0}")]
    BadRequest(String),

    /// 401 Unauthorized
    #[error("{0}")]
    Unauthorized(String),

    /// 403 Forbidden
    #[error("{0}")]
    Forbidden(String),

    /// 404 Not Found
    #[error("{0}")]
    NotFound(String),

    /// 409 Conflict
    #[error("{0}")]
    Conflict(String),

    /// 422 Unprocessable Entity
    #[error("{0}")]
    UnprocessableEntity(String),

    /// 500 Internal Server Error
    #[error("{0}")]
    Internal(String),
}

impl AppError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }
    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::Forbidden(msg.into())
    }
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }
    pub fn unprocessable(msg: impl Into<String>) -> Self {
        Self::UnprocessableEntity(msg.into())
    }
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

/// Convert any `anyhow::Error` into a 500.
impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        Self::Internal(e.to_string())
    }
}
