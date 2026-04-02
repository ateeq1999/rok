//! JWT Claims.

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Claims carried in a refresh token.
///
/// Refresh tokens use a dedicated `typ` field (`"refresh"`) so they cannot be
/// accepted where an access token is expected (and vice-versa).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    /// Subject — same value as in the corresponding access token.
    pub sub: String,

    /// Discriminator — always `"refresh"`.
    pub typ: String,

    /// Expiry (Unix timestamp).
    pub exp: i64,

    /// Issued-at (Unix timestamp).
    pub iat: i64,

    /// Issuer (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
}

impl RefreshClaims {
    pub(crate) fn new(sub: impl Into<String>, exp: i64, iss: Option<String>) -> Self {
        Self {
            sub: sub.into(),
            typ: "refresh".to_string(),
            exp,
            iat: Utc::now().timestamp(),
            iss,
        }
    }
}

/// Standard + custom JWT claims carried in every token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — typically a user ID.
    pub sub: String,

    /// Roles assigned to this subject.
    #[serde(default)]
    pub roles: Vec<String>,

    /// Expiry (Unix timestamp).
    pub exp: i64,

    /// Issued-at (Unix timestamp).
    pub iat: i64,

    /// Issuer (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
}

impl Claims {
    /// Create claims for `subject` with the given `roles`, expiring in 1 hour.
    pub fn new(subject: impl Into<String>, roles: Vec<impl Into<String>>) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub: subject.into(),
            roles: roles.into_iter().map(Into::into).collect(),
            exp: now + 3600,
            iat: now,
            iss: None,
        }
    }

    /// Return `true` if `role` is in the claims' role list.
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Return `true` if the token has not yet expired.
    pub fn is_valid(&self) -> bool {
        Utc::now().timestamp() < self.exp
    }
}
