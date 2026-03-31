use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Ok = 0,
    Partial = 1,
    SchemaError = 2,
    StartupError = 3,
    Timeout = 4,
    ConfigError = 5,
    IoError = 6,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code as i32
    }
}

#[derive(Debug)]
pub struct RokError {
    pub code: ExitCode,
    pub message: String,
    pub context: Option<String>,
}

impl RokError {
    pub fn new(code: ExitCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn schema(message: impl Into<String>) -> Self {
        Self::new(ExitCode::SchemaError, message)
    }

    pub fn startup(message: impl Into<String>) -> Self {
        Self::new(ExitCode::StartupError, message)
    }

    pub fn config(message: impl Into<String>) -> Self {
        Self::new(ExitCode::ConfigError, message)
    }

    pub fn io(message: impl Into<String>) -> Self {
        Self::new(ExitCode::IoError, message)
    }

    #[allow(dead_code)]
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::new(ExitCode::Timeout, message)
    }
}

impl fmt::Display for RokError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ref ctx) = self.context {
            write!(f, " (context: {})", ctx)?;
        }
        Ok(())
    }
}

impl std::error::Error for RokError {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct StepError {
    pub step_index: usize,
    pub step_id: Option<String>,
    pub message: String,
}

impl StepError {
    #[allow(dead_code)]
    pub fn new(step_index: usize, message: impl Into<String>) -> Self {
        Self {
            step_index,
            step_id: None,
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    pub fn with_id(step_index: usize, step_id: String, message: impl Into<String>) -> Self {
        Self {
            step_index,
            step_id: Some(step_id),
            message: message.into(),
        }
    }
}

impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref id) = self.step_id {
            write!(f, "Step '{}' (index {}): {}", id, self.step_index, self.message)
        } else {
            write!(f, "Step {}: {}", self.step_index, self.message)
        }
    }
}

impl std::error::Error for StepError {}
