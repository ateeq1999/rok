//! rok-lint — custom linting rules for rok projects.
//!
//! ```rust
//! use rok_lint::{Linter, rules};
//!
//! let mut linter = Linter::new();
//! linter.add_rule(rules::TodoCommentRule::default());
//! linter.add_rule(rules::LongLineRule::new(120));
//! linter.add_rule(rules::TrailingWhitespaceRule::default());
//!
//! let diags = linter.check_str("example.rs", "let x = 1; // TODO: fix this\n");
//! assert_eq!(diags.len(), 1);
//! ```

pub mod diagnostic;
pub mod linter;
pub mod rules;

pub use diagnostic::{Diagnostic, Severity};
pub use linter::Linter;
