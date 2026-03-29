use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Ok = 0,
    Partial = 1,
    SchemaError = 2,
    StartupError = 3,
    Timeout = 4,
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
}

impl RokError {
    pub fn new(code: ExitCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn schema(message: impl Into<String>) -> Self {
        Self::new(ExitCode::SchemaError, message)
    }

    pub fn startup(message: impl Into<String>) -> Self {
        Self::new(ExitCode::StartupError, message)
    }

    #[allow(dead_code)]
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::new(ExitCode::Timeout, message)
    }
}

impl fmt::Display for RokError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RokError {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct StepError {
    pub step_index: usize,
    pub message: String,
}

impl StepError {
    #[allow(dead_code)]
    pub fn new(step_index: usize, message: impl Into<String>) -> Self {
        Self {
            step_index,
            message: message.into(),
        }
    }
}

impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Step {}: {}", self.step_index, self.message)
    }
}

impl std::error::Error for StepError {}
