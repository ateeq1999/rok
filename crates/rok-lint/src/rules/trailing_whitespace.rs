use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::Rule;

/// Flags lines that end with whitespace characters.
#[derive(Default)]
pub struct TrailingWhitespaceRule;

impl Rule for TrailingWhitespaceRule {
    fn id(&self) -> &str {
        "trailing-whitespace"
    }

    fn check(&self, file: &str, content: &str) -> Vec<Diagnostic> {
        content
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                if line != line.trim_end() {
                    Some(Diagnostic {
                        rule: self.id().to_string(),
                        file: file.to_string(),
                        line: i + 1,
                        severity: Severity::Warning,
                        message: "trailing whitespace".to_string(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
