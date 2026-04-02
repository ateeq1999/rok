//! rok-auth — JWT authentication and RBAC for the rok ecosystem.
//!
//! ```rust,no_run
//! use rok_auth::{Auth, AuthConfig, Claims};
//!
//! let auth = Auth::new(AuthConfig {
//!     secret: "super-secret-key".to_string(),
//!     ..Default::default()
//! });
//!
//! // Sign a token
//! let token = auth.sign(&Claims::new("user-123", vec!["admin"])).unwrap();
//!
//! // Verify and decode
//! let claims = auth.verify(&token).unwrap();
//! assert_eq!(claims.sub, "user-123");
//! assert!(claims.has_role("admin"));
//! ```

pub mod claims;
pub mod config;
pub mod error;
pub mod jwt;
pub mod password;
pub mod session;

pub use claims::{Claims, RefreshClaims};
pub use config::AuthConfig;
pub use error::AuthError;
pub use jwt::Auth;
pub use session::SessionToken;
