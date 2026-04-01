//! [`Generator`] — renders named templates with Tera.

use std::collections::HashMap;
use std::path::Path;

use serde_json::Value;
use tera::Tera;

use crate::error::GenerateError;
use crate::templates;

/// Code generator backed by Tera templates.
///
/// Register custom templates with [`Generator::add_template`] or use one of
/// the built-in templates: `"model"`, `"handler"`, `"migration"`, `"repository"`.
pub struct Generator {
    tera: Tera,
}

impl Generator {
    /// Create a generator pre-loaded with built-in templates.
    pub fn new() -> Self {
        let mut tera = Tera::default();
        tera.add_raw_template("model", templates::MODEL).unwrap();
        tera.add_raw_template("handler", templates::HANDLER).unwrap();
        tera.add_raw_template("migration", templates::MIGRATION).unwrap();
        tera.add_raw_template("repository", templates::REPOSITORY).unwrap();
        Self { tera }
    }

    /// Register an additional template from a string.
    pub fn add_template(&mut self, name: &str, content: &str) -> Result<(), GenerateError> {
        self.tera
            .add_raw_template(name, content)
            .map_err(|e| GenerateError::RenderError {
                template: name.to_string(),
                reason: e.to_string(),
            })
    }

    /// Register templates from a directory (files named `<name>.tera`).
    pub fn add_template_dir<P: AsRef<Path>>(&mut self, dir: P) -> Result<(), GenerateError> {
        let pattern = format!("{}/**/*.tera", dir.as_ref().display());
        self.tera
            .add_template_files(
                self.tera
                    .get_template_names()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|_| ("", None::<&str>)),
            )
            .ok();
        // Load via glob pattern.
        Tera::new(&pattern)
            .map(|extra| {
                for name in extra.get_template_names() {
                    if let Ok(src) = extra.render(name, &tera::Context::new()) {
                        let _ = self.tera.add_raw_template(name, &src);
                    }
                }
            })
            .map_err(|e| GenerateError::RenderError {
                template: pattern,
                reason: e.to_string(),
            })
    }

    /// Render `template_name` with `vars` and return the generated string.
    pub fn render(
        &self,
        template_name: &str,
        vars: &HashMap<String, Value>,
    ) -> Result<String, GenerateError> {
        if !self.tera.get_template_names().any(|n| n == template_name) {
            return Err(GenerateError::UnknownTemplate(template_name.to_string()));
        }
        let mut ctx = tera::Context::new();
        for (k, v) in vars {
            ctx.insert(k, v);
        }
        self.tera
            .render(template_name, &ctx)
            .map_err(|e| GenerateError::RenderError {
                template: template_name.to_string(),
                reason: e.to_string(),
            })
    }

    /// Render and write the output to `dest_path`.
    pub fn render_to_file<P: AsRef<Path>>(
        &self,
        template_name: &str,
        vars: &HashMap<String, Value>,
        dest_path: P,
    ) -> Result<(), GenerateError> {
        let output = self.render(template_name, vars)?;
        if let Some(parent) = dest_path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(dest_path, output)?;
        Ok(())
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
