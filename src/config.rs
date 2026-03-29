use crate::error::RokError;
use crate::schema::Options;
use std::path::Path;

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
