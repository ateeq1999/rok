//! Lint diagnostic types.

use std::fmt;

/// The severity of a lint diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Hint,
    Warning,
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hint => write!(f, "hint"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// A single lint finding.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Rule that produced this diagnostic.
    pub rule: String,
    /// File path (or `"<stdin>"` for in-memory checks).
    pub file: String,
    /// 1-based line number.
    pub line: usize,
    /// Severity level.
    pub severity: Severity,
    /// Human-readable message.
    pub message: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}:{} — {} ({})",
            self.severity, self.file, self.line, self.message, self.rule
        )
    }
}
