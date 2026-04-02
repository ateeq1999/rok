//! [`ConfigBuilder`] — fluent builder for [`Config`].

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::error::ConfigError;
use crate::loader::{self, ConfigFormat};
use crate::{env, Config};

/// Source registered with the builder.
enum Source {
    File(PathBuf, ConfigFormat),
    Env(String),
    Defaults(HashMap<String, String>),
}

/// Fluent builder for [`Config`].
///
/// Sources are applied in registration order; later sources override earlier
/// ones (env variables registered last win over file values).
#[derive(Default)]
pub struct ConfigBuilder {
    sources: Vec<Source>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a config file.  If `format` is `None` the format is inferred from
    /// the file extension.
    pub fn file<P: AsRef<Path>>(mut self, path: P, format: ConfigFormat) -> Self {
        self.sources
            .push(Source::File(path.as_ref().to_owned(), format));
        self
    }

    /// Add a config file, auto-detecting the format from its extension.
    /// Returns `self` unchanged (no panic) if the extension is unrecognised —
    /// the file is simply skipped and a warning is printed.
    pub fn file_auto<P: AsRef<Path>>(mut self, path: P) -> Self {
        let p = path.as_ref().to_owned();
        if let Some(fmt) = ConfigFormat::from_path(&p) {
            self.sources.push(Source::File(p, fmt));
        } else {
            eprintln!(
                "[rok-config] warning: unrecognised extension for `{}`, skipping",
                p.display()
            );
        }
        self
    }

    /// Merge environment variables whose names start with `prefix`.
    ///
    /// `APP_SERVER__PORT=3000` → key `"server.port"` (double-underscore as
    /// nesting separator).
    pub fn env(mut self, prefix: impl Into<String>) -> Self {
        self.sources.push(Source::Env(prefix.into()));
        self
    }

    /// Provide hard-coded default values.  These are merged first, so any
    /// file or env source can override them.
    pub fn defaults(mut self, map: HashMap<String, String>) -> Self {
        self.sources.push(Source::Defaults(map));
        self
    }

    /// Build the [`Config`].
    pub fn build(self) -> Result<Config, ConfigError> {
        let mut merged: HashMap<String, String> = HashMap::new();

        for source in self.sources {
            match source {
                Source::Defaults(map) => merged.extend(map),
                Source::File(path, format) => {
                    let data = loader::load_file(&path, format)?;
                    merged.extend(data);
                }
                Source::Env(prefix) => {
                    let data = env::load_env(&prefix);
                    merged.extend(data);
                }
            }
        }

        Ok(Config::from_map(merged))
    }
}
