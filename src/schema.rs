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
        #[serde(default)]
        id: String,
        cmd: String,
    },
    Read {
        #[serde(default)]
        id: String,
        path: String,
        #[serde(default)]
        max_bytes: Option<usize>,
        #[serde(default)]
        encoding: Option<String>,
    },
    Write {
        #[serde(default)]
        id: String,
        path: String,
        content: String,
        #[serde(default = "default_true")]
        create_dirs: bool,
    },
    Patch {
        #[serde(default)]
        id: String,
        path: String,
        edits: Vec<PatchEdit>,
    },
    Mv {
        #[serde(default)]
        id: String,
        from: String,
        to: String,
    },
    Cp {
        #[serde(default)]
        id: String,
        from: String,
        to: String,
        #[serde(default)]
        recursive: bool,
    },
    Rm {
        #[serde(default)]
        id: String,
        path: String,
        #[serde(default)]
        recursive: bool,
    },
    Mkdir {
        #[serde(default)]
        id: String,
        path: String,
    },
    Grep {
        #[serde(default)]
        id: String,
        pattern: String,
        path: String,
        #[serde(default)]
        ext: Vec<String>,
        #[serde(default = "default_regex")]
        regex: bool,
        #[serde(default)]
        context_lines: Option<usize>,
    },
    Replace {
        #[serde(default)]
        id: String,
        pattern: String,
        replacement: String,
        path: String,
        #[serde(default)]
        ext: Vec<String>,
        #[serde(default = "default_regex")]
        regex: bool,
        #[serde(default = "default_true")]
        case_sensitive: bool,
    },
    Scan {
        #[serde(default)]
        id: String,
        path: String,
        #[serde(default = "default_depth")]
        depth: usize,
        #[serde(default)]
        include: Vec<String>,
        #[serde(default = "default_scan_output")]
        output: ScanOutput,
    },
    Summarize {
        #[serde(default)]
        id: String,
        path: String,
        #[serde(default)]
        focus: String,
    },
    Extract {
        #[serde(default)]
        id: String,
        path: String,
        #[serde(default)]
        pick: Vec<String>,
    },
    Diff {
        #[serde(default)]
        id: String,
        a: String,
        b: String,
        #[serde(default = "default_diff_format")]
        format: DiffFormat,
    },
    Lint {
        #[serde(default)]
        id: String,
        path: String,
        #[serde(default = "default_lint_tool")]
        tool: LintTool,
    },
    Template {
        #[serde(default)]
        id: String,
        #[serde(default)]
        name: String,
        #[serde(default)]
        builtin: String,
        #[serde(default)]
        source: String,
        output: String,
        #[serde(default)]
        vars: HashMap<String, String>,
    },
    Snapshot {
        #[serde(default)]
        id: String,
        path: String,
        snapshot_id: String,
    },
    Restore {
        #[serde(default)]
        id: String,
        snapshot_id: String,
    },
    Git {
        #[serde(default)]
        id: String,
        op: GitOp,
        #[serde(default)]
        args: Vec<String>,
    },
    Http {
        #[serde(default)]
        id: String,
        method: String,
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
        #[serde(default = "default_expect_status")]
        expect_status: u16,
        #[serde(default)]
        body: Option<String>,
    },
    If {
        #[serde(default)]
        id: String,
        condition: Condition,
        then: Vec<Step>,
        #[serde(default)]
        else_: Vec<Step>,
    },
    Each {
        #[serde(default)]
        id: String,
        over: EachOver,
        #[serde(default = "default_each_as", rename = "as")]
        as_: String,
        #[serde(default = "default_each_parallel")]
        parallel: bool,
        step: Box<Step>,
    },
    Parallel {
        #[serde(default)]
        id: String,
        steps: Vec<Step>,
    },
}

fn default_regex() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_depth() -> usize {
    3
}

fn default_scan_output() -> ScanOutput {
    ScanOutput::Summary
}

fn default_diff_format() -> DiffFormat {
    DiffFormat::Stat
}

fn default_lint_tool() -> LintTool {
    LintTool::Auto
}

fn default_expect_status() -> u16 {
    200
}

fn default_each_parallel() -> bool {
    true
}

fn default_each_as() -> String {
    "item".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScanOutput {
    Summary,
    Full,
    Imports,
    Exports,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DiffFormat {
    Unified,
    Json,
    Stat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LintTool {
    Auto,
    Eslint,
    Biome,
    Clippy,
    Ruff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GitOp {
    Status,
    Diff,
    Log,
    Add,
    Commit,
    Branch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchEdit {
    pub find: String,
    pub replace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EachOver {
    List(Vec<String>),
    Ref(EachRef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EachRef {
    pub ref_: usize,
    #[serde(default = "default_pick")]
    pub pick: String,
}

fn default_pick() -> String {
    "*".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Condition {
    Exists {
        path: String,
    },
    Contains {
        path: String,
        pattern: String,
        #[serde(default)]
        regex: bool,
    },
    GrepHasResults {
        ref_: usize,
    },
    StepOk {
        ref_: usize,
    },
    StepFailed {
        ref_: usize,
    },
    FileChanged {
        path: String,
        since: String,
    },
    Not {
        condition: Box<Condition>,
    },
    And {
        conditions: Vec<Condition>,
    },
    Or {
        conditions: Vec<Condition>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub options: Options,
    #[serde(default)]
    pub props: HashMap<String, serde_json::Value>,
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
        diff: Option<String>,
    },
    Patch {
        path: String,
        edits_applied: usize,
        diff: Option<String>,
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
    Scan {
        path: String,
        stack: Vec<String>,
        entry_points: Vec<String>,
        file_count: usize,
        tree: HashMap<String, Vec<String>>,
        exports: HashMap<String, Vec<String>>,
        imports_graph: HashMap<String, Vec<String>>,
    },
    Summarize {
        path: String,
        summary: FileSummary,
    },
    Extract {
        path: String,
        data: serde_json::Value,
    },
    Diff {
        a: String,
        b: String,
        added: usize,
        removed: usize,
        changed_sections: Vec<String>,
        is_identical: bool,
        unified_diff: Option<String>,
    },
    Lint {
        errors_count: usize,
        warnings_count: usize,
        errors: Vec<LintError>,
    },
    Template {
        output: String,
        rendered: bool,
    },
    Snapshot {
        path: String,
        id: String,
        archived: bool,
    },
    Restore {
        id: String,
        restored: bool,
    },
    Git {
        op: String,
        output: serde_json::Value,
    },
    Http {
        method: String,
        url: String,
        status: u16,
        body: Option<String>,
    },
    If {
        condition_met: bool,
        branch: String,
        results: Vec<StepResult>,
    },
    Each {
        items: Vec<String>,
        results: Vec<StepResult>,
    },
    Parallel {
        results: Vec<StepResult>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSummary {
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub functions: Vec<String>,
    pub types_used: Vec<String>,
    pub line_count: usize,
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LintError {
    pub file: String,
    pub line: usize,
    pub rule: String,
    pub message: String,
    pub severity: String,
}

impl StepTypeResult {
    #[allow(dead_code)]
    pub fn step_type_name(&self) -> &'static str {
        match self {
            Self::Bash { .. } => "bash",
            Self::Read { .. } => "read",
            Self::Write { .. } => "write",
            Self::Patch { .. } => "patch",
            Self::Mv { .. } => "mv",
            Self::Cp { .. } => "cp",
            Self::Rm { .. } => "rm",
            Self::Mkdir { .. } => "mkdir",
            Self::Grep { .. } => "grep",
            Self::Replace { .. } => "replace",
            Self::Scan { .. } => "scan",
            Self::Summarize { .. } => "summarize",
            Self::Extract { .. } => "extract",
            Self::Diff { .. } => "diff",
            Self::Lint { .. } => "lint",
            Self::Template { .. } => "template",
            Self::Snapshot { .. } => "snapshot",
            Self::Restore { .. } => "restore",
            Self::Git { .. } => "git",
            Self::Http { .. } => "http",
            Self::If { .. } => "if",
            Self::Each { .. } => "each",
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
