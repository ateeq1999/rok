//! [`Linter`] — runs a set of rules against source files.

use std::path::Path;

use crate::diagnostic::Diagnostic;
use crate::rules::Rule;

/// Runs registered [`Rule`]s against source files or strings.
#[derive(Default)]
pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a rule.
    pub fn add_rule<R: Rule + 'static>(&mut self, rule: R) -> &mut Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// Check an in-memory string, using `file` as the reported path.
    pub fn check_str(&self, file: &str, content: &str) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for rule in &self.rules {
            out.extend(rule.check(file, content));
        }
        out
    }

    /// Check a file on disk.
    pub fn check_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Vec<Diagnostic>> {
        let content = std::fs::read_to_string(&path)?;
        let file = path.as_ref().display().to_string();
        Ok(self.check_str(&file, &content))
    }

    /// Check every `*.rs` file under `dir` recursively.
    pub fn check_dir<P: AsRef<Path>>(&self, dir: P) -> anyhow::Result<Vec<Diagnostic>> {
        let mut all = Vec::new();
        for entry in walkdir(dir.as_ref())? {
            all.extend(self.check_file(&entry)?);
        }
        Ok(all)
    }
}

fn walkdir(dir: &Path) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(walkdir(&path)?);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(files)
}
