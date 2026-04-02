use crate::diagnostic::{Diagnostic, Severity};
use crate::rules::Rule;

/// Flags lines that exceed a configured column limit.
pub struct LongLineRule {
    pub max_columns: usize,
    pub severity: Severity,
}

impl LongLineRule {
    pub fn new(max_columns: usize) -> Self {
        Self {
            max_columns,
            severity: Severity::Warning,
        }
    }
}

impl Default for LongLineRule {
    fn default() -> Self {
        Self::new(100)
    }
}

impl Rule for LongLineRule {
    fn id(&self) -> &str {
        "long-line"
    }

    fn check(&self, file: &str, content: &str) -> Vec<Diagnostic> {
        content
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                let len = line.chars().count();
                if len > self.max_columns {
                    Some(Diagnostic {
                        rule: self.id().to_string(),
                        file: file.to_string(),
                        line: i + 1,
                        severity: self.severity,
                        message: format!("line too long ({len} > {} columns)", self.max_columns),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
