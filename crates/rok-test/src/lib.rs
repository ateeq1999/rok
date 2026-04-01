//! rok-test — testing utilities for the rok ecosystem.
//!
//! Provides assertion helpers, environment isolation, temp-dir fixtures,
//! and JSON comparison utilities for integration tests.
//!
//! ```rust
//! use rok_test::{assert_contains, assert_json_eq, TestEnv};
//!
//! let env = TestEnv::new();
//! env.set("APP_ENV", "test");
//!
//! assert_contains!("hello world", "hello");
//! assert_json_eq!(
//!     serde_json::json!({"a": 1}),
//!     serde_json::json!({"a": 1})
//! );
//! ```

pub mod env;
pub mod fixture;
pub mod json;

pub use env::TestEnv;
pub use fixture::TempFixture;

// ── assertion macros ─────────────────────────────────────────────────────────

/// Assert that `$haystack` contains `$needle` (works for `&str` and `String`).
///
/// ```rust
/// rok_test::assert_contains!("hello world", "world");
/// ```
#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            $haystack.contains($needle),
            "expected {:?} to contain {:?}",
            $haystack,
            $needle
        )
    };
    ($haystack:expr, $needle:expr, $($arg:tt)+) => {
        assert!(
            $haystack.contains($needle),
            $($arg)+
        )
    };
}

/// Assert that two [`serde_json::Value`]s are equal, printing a diff on failure.
///
/// ```rust
/// rok_test::assert_json_eq!(
///     serde_json::json!({"status": "ok"}),
///     serde_json::json!({"status": "ok"})
/// );
/// ```
#[macro_export]
macro_rules! assert_json_eq {
    ($left:expr, $right:expr) => {
        let l: serde_json::Value = $left;
        let r: serde_json::Value = $right;
        assert_eq!(
            l, r,
            "\nleft:  {}\nright: {}",
            serde_json::to_string_pretty(&l).unwrap_or_default(),
            serde_json::to_string_pretty(&r).unwrap_or_default(),
        )
    };
}

/// Assert that `$value` matches the given JSON literal.
///
/// Keys must be bare identifiers (no quotes):
///
/// ```rust
/// let v = serde_json::json!({"code": 200, "status": "ok"});
/// rok_test::assert_json_matches!(v, {code: 200, status: "ok"});
/// ```
#[macro_export]
macro_rules! assert_json_matches {
    ($value:expr, {$($key:tt : $val:tt),* $(,)?}) => {{
        let v: &serde_json::Value = &$value;
        $(
            let key = stringify!($key);
            let expected = serde_json::json!($val);
            let actual = v.get(key).unwrap_or(&serde_json::Value::Null);
            assert_eq!(actual, &expected, "key `{key}` mismatch");
        )*
    }};
}
