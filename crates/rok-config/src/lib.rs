//! rok-config — configuration management for the rok ecosystem.
//!
//! ```rust,no_run
//! use rok_config::{Config, ConfigFormat};
//!
//! let config = Config::builder()
//!     .file("config.toml", ConfigFormat::Toml)
//!     .env("APP_")
//!     .build()
//!     .unwrap();
//!
//! let port: u16 = config.get("server.port").unwrap();
//! ```

mod builder;
mod env;
mod error;
mod loader;
mod value;

pub use builder::ConfigBuilder;
pub use error::ConfigError;
pub use loader::ConfigFormat;
pub use value::Value;

use std::collections::HashMap;

/// A resolved configuration object.
///
/// Values are stored as a flat map using dot-separated keys, e.g.
/// `"server.port"` → `"3000"`.
#[derive(Debug, Clone, Default)]
pub struct Config {
    data: HashMap<String, String>,
}

impl Config {
    /// Start building a [`Config`].
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Retrieve a value by its dot-separated key and parse it into `T`.
    pub fn get<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let raw = self
            .data
            .get(key)
            .ok_or_else(|| ConfigError::MissingKey(key.to_string()))?;
        raw.parse::<T>()
            .map_err(|e| ConfigError::ParseError(key.to_string(), e.to_string()))
    }

    /// Retrieve a value as a `&str`, returning `None` if the key is absent.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    /// Returns `true` if the key exists in the configuration.
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Return the number of entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // ── internal ──────────────────────────────────────────────────────────

    pub(crate) fn from_map(data: HashMap<String, String>) -> Self {
        Self { data }
    }
}
