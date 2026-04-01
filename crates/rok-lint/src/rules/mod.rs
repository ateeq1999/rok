//! Built-in lint rules.

mod long_line;
mod todo_comment;
mod trailing_whitespace;

pub use long_line::LongLineRule;
pub use todo_comment::TodoCommentRule;
pub use trailing_whitespace::TrailingWhitespaceRule;

use crate::diagnostic::Diagnostic;

/// Trait implemented by every lint rule.
pub trait Rule: Send + Sync {
    /// Return a short unique identifier for this rule (e.g. `"todo-comment"`).
    fn id(&self) -> &str;

    /// Check `content` (the full file text) and return all diagnostics found.
    fn check(&self, file: &str, content: &str) -> Vec<Diagnostic>;
}
