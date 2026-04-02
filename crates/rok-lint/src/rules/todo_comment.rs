use regex::Regex;

use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::Rule;

/// Flags `// TODO`, `// FIXME`, and `// HACK` comments.
#[derive(Default)]
pub struct TodoCommentRule {
    pub severity: Option<Severity>,
}

impl Rule for TodoCommentRule {
    fn id(&self) -> &str {
        "todo-comment"
    }

    fn check(&self, file: &str, content: &str) -> Vec<Diagnostic> {
        let re = Regex::new(r"//\s*(TODO|FIXME|HACK)\b").unwrap();
        let severity = self.severity.unwrap_or(Severity::Warning);
        content
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                if re.is_match(line) {
                    Some(Diagnostic {
                        rule: self.id().to_string(),
                        file: file.to_string(),
                        line: i + 1,
                        severity,
                        message: format!("unresolved comment marker: `{}`", line.trim()),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
