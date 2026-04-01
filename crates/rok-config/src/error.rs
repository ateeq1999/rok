use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing configuration key: `{0}`")]
    MissingKey(String),

    #[error("failed to parse `{0}`: {1}")]
    ParseError(String, String),

    #[error("failed to read config file `{path}`: {source}")]
    IoError {
        path: String,
        source: std::io::Error,
    },

    #[error("failed to parse config file `{path}` as {format}: {reason}")]
    FormatError {
        path: String,
        format: String,
        reason: String,
    },
}
