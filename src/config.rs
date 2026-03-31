use crate::error::RokError;
use crate::schema::Options;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub options: Options,
    pub cwd: std::path::PathBuf,
    pub env: std::collections::HashMap<String, String>,
}

impl Config {
    pub fn from_options(options: Options) -> Result<Self, RokError> {
        let cwd_str = options.cwd.clone();
        let cwd = Path::new(&cwd_str);

        if !cwd.exists() {
            return Err(RokError::startup(format!(
                "Working directory does not exist: {}",
                cwd_str
            )));
        }

        if !cwd.is_dir() {
            return Err(RokError::startup(format!(
                "Working directory is not a directory: {}",
                cwd_str
            )));
        }

        let mut env = std::env::vars().collect::<std::collections::HashMap<_, _>>();
        env.extend(options.env.clone());

        Ok(Self {
            options,
            cwd: cwd.to_path_buf(),
            env,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn test_config_from_options_valid() {
        let temp_dir = TempDir::new().unwrap();
        let options = Options {
            cwd: temp_dir.path().to_string_lossy().to_string(),
            stop_on_error: true,
            timeout_ms: 30000,
            env: HashMap::new(),
            cache: false,
            cache_dir: None,
        };

        let config = Config::from_options(options);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.cwd, temp_dir.path());
    }

    #[test]
    fn test_config_from_options_nonexistent_dir() {
        let options = Options {
            cwd: "/nonexistent/path/that/does/not/exist".to_string(),
            stop_on_error: true,
            timeout_ms: 30000,
            env: HashMap::new(),
            cache: false,
            cache_dir: None,
        };

        let config = Config::from_options(options);
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(err.message.contains("Working directory does not exist"));
    }

    #[test]
    fn test_config_from_options_file_not_dir() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let options = Options {
            cwd: file_path.to_string_lossy().to_string(),
            stop_on_error: true,
            timeout_ms: 30000,
            env: HashMap::new(),
            cache: false,
            cache_dir: None,
        };

        let config = Config::from_options(options);
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(err.message.contains("is not a directory"));
    }

    #[test]
    fn test_config_merges_env_vars() {
        let temp_dir = TempDir::new().unwrap();
        let mut custom_env = HashMap::new();
        custom_env.insert("CUSTOM_VAR".to_string(), "custom_value".to_string());

        let options = Options {
            cwd: temp_dir.path().to_string_lossy().to_string(),
            stop_on_error: true,
            timeout_ms: 30000,
            env: custom_env,
            cache: false,
            cache_dir: None,
        };

        let config = Config::from_options(options);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(
            config.env.get("CUSTOM_VAR"),
            Some(&"custom_value".to_string())
        );
    }

    #[test]
    fn test_config_default_cwd() {
        let options = Options::default();
        assert_eq!(options.cwd, ".");
    }
}

use std::fs;
