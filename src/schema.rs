use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    #[serde(default = "default_cwd")]
    pub cwd: String,
    #[serde(default = "default_stop_on_error")]
    pub stop_on_error: bool,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

fn default_cwd() -> String {
    ".".to_string()
}

fn default_stop_on_error() -> bool {
    true
}

fn default_timeout_ms() -> u64 {
    30000
}

impl Default for Options {
    fn default() -> Self {
        Self {
            cwd: default_cwd(),
            stop_on_error: default_stop_on_error(),
            timeout_ms: default_timeout_ms(),
            env: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Step {
    Bash {
        cmd: String,
    },
    Read {
        path: String,
    },
    Write {
        path: String,
        content: String,
    },
    Mv {
        from: String,
        to: String,
    },
    Cp {
        from: String,
        to: String,
        #[serde(default)]
        recursive: bool,
    },
    Rm {
        path: String,
        #[serde(default)]
        recursive: bool,
    },
    Mkdir {
        path: String,
    },
    Grep {
        pattern: String,
        path: String,
        #[serde(default)]
        ext: Vec<String>,
        #[serde(default = "default_regex")]
        regex: bool,
    },
    Replace {
        pattern: String,
        replacement: String,
        path: String,
        #[serde(default)]
        ext: Vec<String>,
        #[serde(default = "default_regex")]
        regex: bool,
    },
    Parallel {
        steps: Vec<Step>,
    },
}

fn default_regex() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    #[serde(default)]
    pub options: Options,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrepMatch {
    pub path: String,
    pub line: usize,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileContent {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepResult {
    pub index: usize,
    #[serde(flatten)]
    pub step_type: StepTypeResult,
    pub status: String,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_pipeline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum StepTypeResult {
    Bash {
        cmd: String,
        stdout: String,
        stderr: String,
        exit_code: i32,
    },
    Read {
        path: String,
        files: Vec<FileContent>,
    },
    Write {
        path: String,
    },
    Mv {
        from: String,
        to: String,
    },
    Cp {
        from: String,
        to: String,
    },
    Rm {
        path: String,
    },
    Mkdir {
        path: String,
    },
    Grep {
        pattern: String,
        matches: Vec<GrepMatch>,
    },
    Replace {
        pattern: String,
        replacement: String,
        files_scanned: usize,
        files_modified: usize,
        total_replacements: usize,
    },
    Parallel {
        results: Vec<StepResult>,
    },
}

impl StepTypeResult {
    #[allow(dead_code)]
    pub fn step_type_name(&self) -> &'static str {
        match self {
            Self::Bash { .. } => "bash",
            Self::Read { .. } => "read",
            Self::Write { .. } => "write",
            Self::Mv { .. } => "mv",
            Self::Cp { .. } => "cp",
            Self::Rm { .. } => "rm",
            Self::Mkdir { .. } => "mkdir",
            Self::Grep { .. } => "grep",
            Self::Replace { .. } => "replace",
            Self::Parallel { .. } => "parallel",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub status: String,
    pub steps_total: usize,
    pub steps_ok: usize,
    pub steps_failed: usize,
    pub duration_ms: u64,
    pub results: Vec<StepResult>,
}
