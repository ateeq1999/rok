//! Password hashing and verification via Argon2id.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::AuthError;

/// Hash `password` with Argon2id + a random salt.
///
/// The returned string is a PHC-encoded hash that can be stored in a database
/// and later verified with [`verify`].
pub fn hash(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AuthError::HashError(e.to_string()))
}

/// Return `true` if `password` matches the stored `hash`.
pub fn verify(password: &str, hash: &str) -> Result<bool, AuthError> {
    let parsed = PasswordHash::new(hash).map_err(|e| AuthError::HashError(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

/// Async version of [`hash`] — offloads the CPU-intensive work to a blocking
/// thread so the async runtime is not stalled.
///
/// ```rust,no_run
/// use rok_auth::password;
///
/// #[tokio::main]
/// async fn main() {
///     let h = password::hash_async("my-password".to_string()).await.unwrap();
///     assert!(password::verify_async("my-password".to_string(), h).await.unwrap());
/// }
/// ```
pub async fn hash_async(password: String) -> Result<String, AuthError> {
    tokio::task::spawn_blocking(move || hash(&password))
        .await
        .map_err(|e| AuthError::HashError(e.to_string()))?
}

/// Async version of [`verify`] — offloads the CPU-intensive work to a blocking
/// thread so the async runtime is not stalled.
pub async fn verify_async(password: String, hash: String) -> Result<bool, AuthError> {
    tokio::task::spawn_blocking(move || verify(&password, &hash))
        .await
        .map_err(|e| AuthError::HashError(e.to_string()))?
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify() {
        let h = hash("correct-horse-battery-staple").unwrap();
        assert!(verify("correct-horse-battery-staple", &h).unwrap());
        assert!(!verify("wrong-password", &h).unwrap());
    }

    #[test]
    fn hashes_are_unique() {
        let h1 = hash("same").unwrap();
        let h2 = hash("same").unwrap();
        // Different salts → different hashes even for the same input.
        assert_ne!(h1, h2);
    }
}
