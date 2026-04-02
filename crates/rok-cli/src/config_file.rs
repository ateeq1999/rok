//! Configuration file support for rok
//!
//! Loads configuration from .rokrc or rok.toml files using rok-config.

use crate::schema::Options;
use rok_config::{Config as RokCfg, ConfigFormat};
use std::path::Path;

/// Try to load a rok config file from one of the standard locations.
///
/// Probes `.rokrc`, `rok.toml`, and `.rok/config.toml` in order.
fn try_load_config(dir: &Path) -> Option<RokCfg> {
    let candidates: &[(&str, ConfigFormat)] = &[
        (".rokrc", ConfigFormat::Toml),
        ("rok.toml", ConfigFormat::Toml),
        (".rok/config.toml", ConfigFormat::Toml),
    ];

    for (rel, fmt) in candidates {
        let path = dir.join(rel);
        if path.exists() {
            match RokCfg::builder().file(&path, *fmt).build() {
                Ok(cfg) => {
                    eprintln!("[rok] Loaded config from {}", path.display());
                    return Some(cfg);
                }
                Err(e) => {
                    eprintln!("[rok] warning: failed to parse {}: {}", path.display(), e);
                }
            }
        }
    }
    None
}

/// Apply values from a `rok-config` [`RokCfg`] to [`Options`].
fn apply_rok_config(cfg: &RokCfg, mut options: Options) -> Options {
    // `[defaults]` section is flattened as "defaults.<key>"
    if let Ok(v) = cfg.get::<bool>("defaults.cache") {
        options.cache = v;
    }
    if let Ok(v) = cfg.get::<bool>("defaults.stopOnError") {
        options.stop_on_error = v;
    }
    if let Ok(v) = cfg.get::<u64>("defaults.timeoutMs") {
        options.timeout_ms = v;
    }

    // `[env]` section is flattened as "env.<VAR_NAME>" → "<value>"
    for (key, value) in cfg.iter() {
        if let Some(var) = key.strip_prefix("env.") {
            options.env.insert(var.to_string(), value.to_string());
        }
    }

    options
}

/// Apply configuration file settings to the given options.
pub fn apply_config(options: Options) -> Options {
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(cfg) = try_load_config(&cwd) {
            return apply_rok_config(&cfg, options);
        }
    }
    options
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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
"#;
        fs::write(&rokrc_path, content).unwrap();

        let cfg = try_load_config(temp_dir.path()).expect("config loaded");
        assert_eq!(cfg.get::<bool>("defaults.cache").unwrap(), true);
        assert_eq!(cfg.get::<bool>("defaults.stopOnError").unwrap(), false);
        assert_eq!(cfg.get::<u64>("defaults.timeoutMs").unwrap(), 60000);
        assert_eq!(cfg.get_str("env.NODE_ENV"), Some("test"));
    }

    #[test]
    fn test_apply_rok_config() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join(".rokrc"),
            "[defaults]\ncache = true\nstopOnError = false\ntimeoutMs = 60000\n",
        )
        .unwrap();

        let cfg = try_load_config(temp_dir.path()).unwrap();
        let options = apply_rok_config(&cfg, Options::default());

        assert!(options.cache);
        assert!(!options.stop_on_error);
        assert_eq!(options.timeout_ms, 60000);
    }

    #[test]
    fn test_no_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let cfg = try_load_config(temp_dir.path());
        assert!(cfg.is_none());
    }
}
