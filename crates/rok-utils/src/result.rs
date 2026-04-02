//! Result / error extension traits.

use std::fmt::Display;

/// Extension methods on `Option<T>`.
pub trait OptionExt<T> {
    /// Convert `None` into an [`anyhow::Error`] with the given message.
    fn context(self, msg: impl Display) -> anyhow::Result<T>;

    /// Like [`OptionExt::context`] but the message is lazily constructed.
    fn with_context<F, M>(self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce() -> M,
        M: Display;
}

impl<T> OptionExt<T> for Option<T> {
    fn context(self, msg: impl Display) -> anyhow::Result<T> {
        self.ok_or_else(|| anyhow::anyhow!("{}", msg))
    }

    fn with_context<F, M>(self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce() -> M,
        M: Display,
    {
        self.ok_or_else(|| anyhow::anyhow!("{}", f()))
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_becomes_error() {
        let r: anyhow::Result<i32> = None::<i32>.context("missing value");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().to_string(), "missing value");
    }

    #[test]
    fn some_passes_through() {
        let r: anyhow::Result<i32> = Some(42).context("unreachable");
        assert_eq!(r.unwrap(), 42);
    }
}
