use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("unknown template `{0}`")]
    UnknownTemplate(String),

    #[error("template render failed for `{template}`: {reason}")]
    RenderError { template: String, reason: String },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
