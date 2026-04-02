//! JWT sign / verify via [`jsonwebtoken`].

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::{AuthConfig, AuthError, Claims};

/// The main auth handle — holds keys derived from [`AuthConfig`].
#[derive(Clone)]
pub struct Auth {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    config: AuthConfig,
}

impl Auth {
    /// Build an [`Auth`] from the given config.
    ///
    /// # Panics
    ///
    /// Panics if `config.secret` is empty.
    pub fn new(config: AuthConfig) -> Self {
        assert!(
            !config.secret.is_empty(),
            "AuthConfig.secret must not be empty"
        );
        let encoding_key = EncodingKey::from_secret(config.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.secret.as_bytes());
        Self {
            encoding_key,
            decoding_key,
            config,
        }
    }

    /// Sign `claims` and return a compact JWT string.
    pub fn sign(&self, claims: &Claims) -> Result<String, AuthError> {
        let mut claims = claims.clone();
        // Apply issuer from config if not already set.
        if claims.iss.is_none() {
            claims.iss = self.config.issuer.clone();
        }
        // Honour token_ttl from config.
        let now = chrono::Utc::now().timestamp();
        claims.iat = now;
        claims.exp = now + self.config.token_ttl.as_secs() as i64;

        jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| AuthError::Internal(e.to_string()))
    }

    /// Verify `token` and return the decoded [`Claims`].
    ///
    /// Returns [`AuthError::TokenExpired`] when the `exp` claim is in the past,
    /// and [`AuthError::InvalidToken`] for all other validation failures.
    pub fn verify(&self, token: &str) -> Result<Claims, AuthError> {
        let mut validation = Validation::new(Algorithm::HS256);
        if let Some(iss) = &self.config.issuer {
            validation.set_issuer(&[iss]);
        }

        jsonwebtoken::decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::InvalidToken,
            })
    }

    /// Return a reference to the [`AuthConfig`].
    pub fn config(&self) -> &AuthConfig {
        &self.config
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_auth() -> Auth {
        Auth::new(AuthConfig {
            secret: "test-secret-key-1234".to_string(),
            ..Default::default()
        })
    }

    #[test]
    fn sign_and_verify() {
        let auth = make_auth();
        let claims = Claims::new("alice", vec!["admin", "user"]);
        let token = auth.sign(&claims).unwrap();
        let decoded = auth.verify(&token).unwrap();
        assert_eq!(decoded.sub, "alice");
        assert!(decoded.has_role("admin"));
        assert!(decoded.has_role("user"));
        assert!(!decoded.has_role("superuser"));
    }

    #[test]
    fn invalid_token_rejected() {
        let auth = make_auth();
        let result = auth.verify("not.a.valid.jwt");
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[test]
    fn wrong_secret_rejected() {
        let signer = make_auth();
        let verifier = Auth::new(AuthConfig {
            secret: "different-secret".to_string(),
            ..Default::default()
        });
        let token = signer
            .sign(&Claims::new("bob", vec![] as Vec<&str>))
            .unwrap();
        assert!(matches!(
            verifier.verify(&token),
            Err(AuthError::InvalidToken)
        ));
    }
}
