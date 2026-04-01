//! Environment-variable isolation for tests.

use std::cell::RefCell;
use std::collections::HashMap;

/// Sets environment variables for the duration of the test and removes (or
/// restores) them on drop.
///
/// ```rust
/// use rok_test::TestEnv;
///
/// let env = TestEnv::new();
/// env.set("MY_VAR", "hello");
/// assert_eq!(std::env::var("MY_VAR").unwrap(), "hello");
/// // Restored when `env` is dropped.
/// ```
pub struct TestEnv {
    original: RefCell<HashMap<String, Option<String>>>,
}

impl TestEnv {
    pub fn new() -> Self {
        Self {
            original: RefCell::new(HashMap::new()),
        }
    }

    /// Set an environment variable, remembering the original value.
    pub fn set(&self, key: &str, value: &str) {
        self.remember(key);
        // SAFETY: env mutation is inherently unsound in multi-threaded code;
        // this helper is intended for single-threaded test binaries only.
        unsafe { std::env::set_var(key, value) };
    }

    /// Remove an environment variable for the test duration.
    pub fn remove(&self, key: &str) {
        self.remember(key);
        unsafe { std::env::remove_var(key) };
    }

    fn remember(&self, key: &str) {
        self.original
            .borrow_mut()
            .entry(key.to_string())
            .or_insert_with(|| std::env::var(key).ok());
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        for (key, original) in self.original.borrow().iter() {
            match original {
                Some(v) => unsafe { std::env::set_var(key, v) },
                None => unsafe { std::env::remove_var(key) },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_restore() {
        let key = "_ROK_TEST_ENV_VAR";
        unsafe { std::env::remove_var(key) };
        {
            let env = TestEnv::new();
            env.set(key, "test_value");
            assert_eq!(std::env::var(key).unwrap(), "test_value");
        }
        assert!(std::env::var(key).is_err());
    }

    #[test]
    fn preserves_existing_value() {
        let key = "_ROK_TEST_ENV_PRESERVE";
        unsafe { std::env::set_var(key, "original") };
        {
            let env = TestEnv::new();
            env.set(key, "overridden");
            assert_eq!(std::env::var(key).unwrap(), "overridden");
        }
        assert_eq!(std::env::var(key).unwrap(), "original");
        unsafe { std::env::remove_var(key) };
    }
}
