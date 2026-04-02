//! Configuration file support for rok
//!
//! Loads configuration from .rokrc or rok.toml files

use crate::schema::Options;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RokConfig {
    #[serde(default)]
    pub defaults: ConfigDefaults,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigDefaults {
    pub output: Option<String>,
    pub verbose: Option<bool>,
    pub quiet: Option<bool>,
    pub cache: Option<bool>,
    pub stop_on_error: Option<bool>,
    pub timeout_ms: Option<u64>,
}

impl RokConfig {
    /// Load configuration from .rokrc or rok.toml in the given directory
    pub fn load(dir: &Path) -> Option<Self> {
        // Try .rokrc first (TOML format)
        let rokrc_path = dir.join(".rokrc");
        if rokrc_path.exists() {
            if let Ok(content) = fs::read_to_string(&rokrc_path) {
                if let Ok(config) = toml::from_str(&content) {
                    log_info(&format!("Loaded config from {}", rokrc_path.display()));
                    return Some(config);
                }
            }
        }

        // Try rok.toml
        let toml_path = dir.join("rok.toml");
        if toml_path.exists() {
            if let Ok(content) = fs::read_to_string(&toml_path) {
                if let Ok(config) = toml::from_str(&content) {
                    log_info(&format!("Loaded config from {}", toml_path.display()));
                    return Some(config);
                }
            }
        }

        // Try .rok/config.toml
        let config_path = dir.join(".rok").join("config.toml");
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str(&content) {
                    log_info(&format!("Loaded config from {}", config_path.display()));
                    return Some(config);
                }
            }
        }

        None
    }

    /// Merge configuration with command-line options
    pub fn merge_with_options(&self, mut options: Options) -> Options {
        // Apply defaults from config
        if let Some(output) = &self.defaults.output {
            // Output is handled at CLI level
            let _ = output;
        }
        if let Some(verbose) = self.defaults.verbose {
            // Verbose is handled at CLI level
            let _ = verbose;
        }
        if let Some(quiet) = self.defaults.quiet {
            // Quiet is handled at CLI level
            let _ = quiet;
        }
        if let Some(cache) = self.defaults.cache {
            options.cache = cache;
        }
        if let Some(stop_on_error) = self.defaults.stop_on_error {
            options.stop_on_error = stop_on_error;
        }
        if let Some(timeout_ms) = self.defaults.timeout_ms {
            options.timeout_ms = timeout_ms;
        }

        // Merge environment variables
        for (key, value) in &self.env {
            options.env.insert(key.clone(), value.clone());
        }

        options
    }
}

/// Apply configuration to options
pub fn apply_config(options: Options) -> Options {
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(config) = RokConfig::load(&cwd) {
            return config.merge_with_options(options);
        }
    }
    options
}

fn log_info(msg: &str) {
    eprintln!("[rok] {}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_config_from_rokrc() {
        let temp_dir = TempDir::new().unwrap();
        let rokrc_path = temp_dir.path().join(".rokrc");

        let content = r#"
[defaults]
cache = true
stopOnError = false
timeoutMs = 60000

[env]
NODE_ENV = "test"
API_URL = "http://localhost:3000"

[aliases]
build = "cargo build --release"
test = "cargo test --all"
"#;
        fs::write(&rokrc_path, content).unwrap();

        let config = RokConfig::load(temp_dir.path());
        assert!(config.is_some());
        let config = config.unwrap();

        assert!(config.defaults.cache.unwrap());
        assert!(!config.defaults.stop_on_error.unwrap());
        assert_eq!(config.defaults.timeout_ms, Some(60000));
        assert_eq!(config.env.get("NODE_ENV"), Some(&"test".to_string()));
    }

    #[test]
    fn test_merge_with_options() {
        let config = RokConfig {
            defaults: ConfigDefaults {
                cache: Some(true),
                stop_on_error: Some(false),
                timeout_ms: Some(60000),
                ..Default::default()
            },
            env: HashMap::new(),
        };

        let options = Options::default();
        let merged = config.merge_with_options(options);

        assert!(merged.cache);
        assert!(!merged.stop_on_error);
        assert_eq!(merged.timeout_ms, 60000);
    }

    #[test]
    fn test_no_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let config = RokConfig::load(temp_dir.path());
        assert!(config.is_none());
    }
}
