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
    #[serde(default)]
    pub cache: bool,
    #[serde(default)]
    pub cache_dir: Option<String>,
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
            cache: false,
            cache_dir: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryConfig {
    #[serde(default = "default_retry_count")]
    pub count: usize,
    #[serde(default = "default_retry_delay")]
    pub delay_ms: u64,
    #[serde(default)]
    pub backoff: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Step {
    Bash {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        cmd: String,
        #[serde(default)]
        timeout_ms: Option<u64>,
        #[serde(default)]
        retry: Option<RetryConfig>,
    },
    Read {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        #[serde(default)]
        max_bytes: Option<usize>,
        #[serde(default)]
        encoding: Option<String>,
        #[serde(default, alias = "filter_imports")]
        filter_imports: Option<String>,
        #[serde(default, alias = "filter_exports")]
        filter_exports: Option<String>,
        #[serde(default)]
        since: Option<String>,
    },
    Write {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        content: String,
        #[serde(default = "default_true")]
        create_dirs: bool,
    },
    Patch {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        edits: Vec<PatchEdit>,
    },
    Mv {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        from: String,
        to: String,
    },
    Cp {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        from: String,
        to: String,
        #[serde(default)]
        recursive: bool,
    },
    Rm {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        #[serde(default)]
        recursive: bool,
    },
    Mkdir {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
    },
    Grep {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
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
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        pattern: String,
        replacement: String,
        path: String,
        #[serde(default)]
        ext: Vec<String>,
        #[serde(default = "default_regex")]
        regex: bool,
        #[serde(default = "default_true")]
        case_sensitive: bool,
        #[serde(default)]
        glob: Option<String>,
        #[serde(default)]
        whole_word: bool,
    },
    Scan {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
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
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        #[serde(default)]
        focus: String,
    },
    Extract {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        #[serde(default)]
        pick: Vec<String>,
    },
    Diff {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        a: String,
        b: String,
        #[serde(default = "default_diff_format")]
        format: DiffFormat,
    },
    Lint {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        #[serde(default = "default_lint_tool")]
        tool: LintTool,
    },
    Template {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
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
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        snapshot_id: String,
    },
    Restore {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        snapshot_id: String,
    },
    Git {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        op: GitOp,
        #[serde(default)]
        args: Vec<String>,
    },
    Http {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        method: String,
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
        #[serde(default = "default_expect_status")]
        expect_status: u16,
        #[serde(default)]
        body: Option<String>,
    },
    Import {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        path: String,
        #[serde(default)]
        add: Vec<String>,
        #[serde(default)]
        remove: Vec<String>,
        #[serde(default)]
        organize: bool,
    },
    If {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        condition: Condition,
        then: Vec<Step>,
        #[serde(default, rename = "else")]
        else_: Vec<Step>,
    },
    Each {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
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
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        steps: Vec<Step>,
    },
}

impl Step {
    pub fn get_id(&self) -> &str {
        match self {
            Step::Bash { id, .. } => id,
            Step::Read { id, .. } => id,
            Step::Write { id, .. } => id,
            Step::Patch { id, .. } => id,
            Step::Mv { id, .. } => id,
            Step::Cp { id, .. } => id,
            Step::Rm { id, .. } => id,
            Step::Mkdir { id, .. } => id,
            Step::Grep { id, .. } => id,
            Step::Replace { id, .. } => id,
            Step::Scan { id, .. } => id,
            Step::Summarize { id, .. } => id,
            Step::Extract { id, .. } => id,
            Step::Diff { id, .. } => id,
            Step::Lint { id, .. } => id,
            Step::Template { id, .. } => id,
            Step::Snapshot { id, .. } => id,
            Step::Restore { id, .. } => id,
            Step::Git { id, .. } => id,
            Step::Http { id, .. } => id,
            Step::Import { id, .. } => id,
            Step::If { id, .. } => id,
            Step::Each { id, .. } => id,
            Step::Parallel { id, .. } => id,
        }
    }

    pub fn get_depends_on(&self) -> &[String] {
        match self {
            Step::Bash { depends_on, .. } => depends_on,
            Step::Read { depends_on, .. } => depends_on,
            Step::Write { depends_on, .. } => depends_on,
            Step::Patch { depends_on, .. } => depends_on,
            Step::Mv { depends_on, .. } => depends_on,
            Step::Cp { depends_on, .. } => depends_on,
            Step::Rm { depends_on, .. } => depends_on,
            Step::Mkdir { depends_on, .. } => depends_on,
            Step::Grep { depends_on, .. } => depends_on,
            Step::Replace { depends_on, .. } => depends_on,
            Step::Scan { depends_on, .. } => depends_on,
            Step::Summarize { depends_on, .. } => depends_on,
            Step::Extract { depends_on, .. } => depends_on,
            Step::Diff { depends_on, .. } => depends_on,
            Step::Lint { depends_on, .. } => depends_on,
            Step::Template { depends_on, .. } => depends_on,
            Step::Snapshot { depends_on, .. } => depends_on,
            Step::Restore { depends_on, .. } => depends_on,
            Step::Git { depends_on, .. } => depends_on,
            Step::Http { depends_on, .. } => depends_on,
            Step::Import { depends_on, .. } => depends_on,
            Step::If { depends_on, .. } => depends_on,
            Step::Each { depends_on, .. } => depends_on,
            Step::Parallel { depends_on, .. } => depends_on,
        }
    }
}

fn default_regex() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_retry_count() -> usize {
    3
}

fn default_retry_delay() -> u64 {
    1000
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

fn default_depends_on() -> Vec<String> {
    Vec::new()
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
        files_filtered: usize,
        filter_reason: Option<String>,
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
    Import {
        path: String,
        added: Vec<String>,
        removed: Vec<String>,
        organized: bool,
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
            Self::Import { .. } => "import",
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_parse_bash_step() {
        let json = r#"{
            "type": "bash",
            "cmd": "echo hello",
            "timeout_ms": 5000
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Bash {
                cmd, timeout_ms, ..
            } => {
                assert_eq!(cmd, "echo hello");
                assert_eq!(timeout_ms, Some(5000));
            }
            _ => panic!("Expected Bash step"),
        }
    }

    #[test]
    fn test_parse_read_step() {
        let json = r#"{
            "type": "read",
            "path": "src/main.rs",
            "max_bytes": 1048576
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Read {
                path, max_bytes, ..
            } => {
                assert_eq!(path, "src/main.rs");
                assert_eq!(max_bytes, Some(1048576));
            }
            _ => panic!("Expected Read step"),
        }
    }

    #[test]
    fn test_parse_write_step() {
        let json = r#"{
            "type": "write",
            "path": "output.txt",
            "content": "Hello World",
            "create_dirs": true
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Write {
                path,
                content,
                create_dirs,
                ..
            } => {
                assert_eq!(path, "output.txt");
                assert_eq!(content, "Hello World");
                assert!(create_dirs);
            }
            _ => panic!("Expected Write step"),
        }
    }

    #[test]
    fn test_parse_grep_step() {
        let json = r#"{
            "type": "grep",
            "pattern": "TODO",
            "path": "./src",
            "ext": ["rs", "ts"],
            "regex": true,
            "context_lines": 2
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Grep {
                pattern,
                path,
                ext,
                regex,
                ..
            } => {
                assert_eq!(pattern, "TODO");
                assert_eq!(path, "./src");
                assert_eq!(ext, vec!["rs", "ts"]);
                assert!(regex);
            }
            _ => panic!("Expected Grep step"),
        }
    }

    #[test]
    fn test_parse_replace_step_with_glob() {
        let json = r#"{
            "type": "replace",
            "pattern": "foo",
            "replacement": "bar",
            "path": "./src",
            "glob": "**/*.rs",
            "whole_word": true
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Replace {
                pattern,
                replacement,
                glob,
                whole_word,
                ..
            } => {
                assert_eq!(pattern, "foo");
                assert_eq!(replacement, "bar");
                assert_eq!(glob, Some("**/*.rs".to_string()));
                assert!(whole_word);
            }
            _ => panic!("Expected Replace step"),
        }
    }

    #[test]
    fn test_parse_if_step() {
        let json = r#"{
            "type": "if",
            "condition": { "type": "exists", "path": "./Cargo.toml" },
            "then": [
                { "type": "bash", "cmd": "echo exists" }
            ],
            "else": [
                { "type": "bash", "cmd": "echo not found" }
            ]
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::If {
                condition,
                then,
                else_,
                ..
            } => {
                match condition {
                    Condition::Exists { path } => {
                        assert_eq!(path, "./Cargo.toml");
                    }
                    _ => panic!("Expected Exists condition"),
                }
                assert_eq!(then.len(), 1);
                assert_eq!(else_.len(), 1);
            }
            _ => panic!("Expected If step"),
        }
    }

    #[test]
    fn test_parse_each_step() {
        let json = r#"{
            "type": "each",
            "over": ["item1", "item2", "item3"],
            "as": "file",
            "parallel": true,
            "step": { "type": "bash", "cmd": "echo {{file}}" }
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Each {
                over,
                as_,
                parallel,
                step,
                ..
            } => {
                match over {
                    EachOver::List(items) => {
                        assert_eq!(items.len(), 3);
                        assert_eq!(items[0], "item1");
                    }
                    _ => panic!("Expected List over"),
                }
                assert_eq!(as_, "file");
                assert!(parallel);
                match *step {
                    Step::Bash { cmd, .. } => assert_eq!(cmd, "echo {{file}}"),
                    _ => panic!("Expected Bash step"),
                }
            }
            _ => panic!("Expected Each step"),
        }
    }

    #[test]
    fn test_parse_parallel_step() {
        let json = r#"{
            "type": "parallel",
            "steps": [
                { "type": "bash", "cmd": "echo 1" },
                { "type": "bash", "cmd": "echo 2" }
            ]
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Parallel { steps, .. } => {
                assert_eq!(steps.len(), 2);
            }
            _ => panic!("Expected Parallel step"),
        }
    }

    #[test]
    fn test_parse_payload_with_options() {
        let json = r#"{
            "name": "test-task",
            "description": "A test task",
            "version": "1.0.0",
            "options": {
                "cwd": "./src",
                "stopOnError": false,
                "timeoutMs": 60000,
                "env": {
                    "NODE_ENV": "production"
                }
            },
            "steps": [
                { "type": "bash", "cmd": "echo hello" }
            ]
        }"#;

        let payload: Payload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name, Some("test-task".to_string()));
        assert_eq!(payload.description, Some("A test task".to_string()));
        assert_eq!(payload.version, Some("1.0.0".to_string()));
        assert_eq!(payload.options.cwd, "./src");
        assert!(!payload.options.stop_on_error);
        assert_eq!(payload.options.timeout_ms, 60000);
        assert_eq!(
            payload.options.env.get("NODE_ENV"),
            Some(&"production".to_string())
        );
        assert_eq!(payload.steps.len(), 1);
    }

    #[test]
    fn test_parse_step_with_id_and_depends() {
        let json = r#"{
            "type": "bash",
            "id": "step2",
            "depends_on": ["step1"],
            "cmd": "echo hello"
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Bash {
                id,
                depends_on,
                cmd,
                ..
            } => {
                assert_eq!(id, "step2");
                assert_eq!(depends_on, vec!["step1"]);
                assert_eq!(cmd, "echo hello");
            }
            _ => panic!("Expected Bash step"),
        }
    }

    #[test]
    fn test_parse_retry_config() {
        let json = r#"{
            "type": "bash",
            "cmd": "flaky-command",
            "retry": {
                "count": 3,
                "delayMs": 2000,
                "backoff": true
            }
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Bash { retry, .. } => {
                assert!(retry.is_some());
                let retry = retry.unwrap();
                assert_eq!(retry.count, 3);
                assert_eq!(retry.delay_ms, 2000);
                assert!(retry.backoff);
            }
            _ => panic!("Expected Bash step"),
        }
    }

    #[test]
    fn test_parse_import_step() {
        let json = r#"{
            "type": "import",
            "path": "src/main.ts",
            "add": ["import { foo } from './foo';"],
            "remove": ["import { bar } from './bar';"],
            "organize": true
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Import {
                path,
                add,
                remove,
                organize,
                ..
            } => {
                assert_eq!(path, "src/main.ts");
                assert_eq!(add.len(), 1);
                assert_eq!(add[0], "import { foo } from './foo';");
                assert_eq!(remove.len(), 1);
                assert!(organize);
            }
            _ => panic!("Expected Import step"),
        }
    }

    #[test]
    fn test_parse_scan_step() {
        let json = r#"{
            "type": "scan",
            "path": "./src",
            "depth": 3,
            "include": ["rs", "toml"],
            "output": "full"
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Scan {
                path,
                depth,
                include,
                output,
                ..
            } => {
                assert_eq!(path, "./src");
                assert_eq!(depth, 3);
                assert_eq!(include, vec!["rs", "toml"]);
                match output {
                    ScanOutput::Full => {}
                    _ => panic!("Expected Full output"),
                }
            }
            _ => panic!("Expected Scan step"),
        }
    }

    #[test]
    fn test_parse_http_step() {
        let json = r#"{
            "type": "http",
            "method": "POST",
            "url": "https://api.example.com/data",
            "headers": {
                "Authorization": "Bearer token",
                "Content-Type": "application/json"
            },
            "expect_status": 201,
            "body": "{\"key\": \"value\"}"
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::Http {
                method,
                url,
                headers,
                expect_status,
                body,
                ..
            } => {
                assert_eq!(method, "POST");
                assert_eq!(url, "https://api.example.com/data");
                assert_eq!(
                    headers.get("Authorization"),
                    Some(&"Bearer token".to_string())
                );
                assert_eq!(expect_status, 201);
                assert_eq!(body, Some("{\"key\": \"value\"}".to_string()));
            }
            _ => panic!("Expected Http step"),
        }
    }

    #[test]
    fn test_parse_complex_condition() {
        let json = r#"{
            "type": "if",
            "condition": {
                "type": "and",
                "conditions": [
                    { "type": "exists", "path": "./Cargo.toml" },
                    { "type": "not", "condition": { "type": "exists", "path": "./dist" } }
                ]
            },
            "then": [
                { "type": "bash", "cmd": "cargo build" }
            ],
            "else": []
        }"#;

        let step: Step = serde_json::from_str(json).unwrap();
        match step {
            Step::If {
                condition, then, ..
            } => {
                match condition {
                    Condition::And { conditions } => {
                        assert_eq!(conditions.len(), 2);
                        match &conditions[0] {
                            Condition::Exists { path } => assert_eq!(path, "./Cargo.toml"),
                            _ => panic!("Expected Exists"),
                        }
                        match &conditions[1] {
                            Condition::Not { condition } => match &**condition {
                                Condition::Exists { path } => assert_eq!(path, "./dist"),
                                _ => panic!("Expected Exists inside Not"),
                            },
                            _ => panic!("Expected Not"),
                        }
                    }
                    _ => panic!("Expected And condition"),
                }
                assert_eq!(then.len(), 1);
            }
            _ => panic!("Expected If step"),
        }
    }

    #[test]
    fn test_step_get_id() {
        let step = Step::Bash {
            id: "my-step".to_string(),
            depends_on: vec![],
            cmd: "echo hello".to_string(),
            timeout_ms: None,
            retry: None,
        };
        assert_eq!(step.get_id(), "my-step");
    }

    #[test]
    fn test_step_get_depends_on() {
        let step = Step::Bash {
            id: "step2".to_string(),
            depends_on: vec!["step1".to_string()],
            cmd: "echo hello".to_string(),
            timeout_ms: None,
            retry: None,
        };
        assert_eq!(step.get_depends_on(), &["step1".to_string()]);
    }

    #[test]
    fn test_default_options() {
        let options = Options::default();
        assert_eq!(options.cwd, ".");
        assert!(options.stop_on_error);
        assert_eq!(options.timeout_ms, 30000);
        assert!(!options.cache);
        assert!(options.cache_dir.is_none());
    }

    #[test]
    fn test_serialize_output() {
        let output = Output {
            status: "ok".to_string(),
            steps_total: 5,
            steps_ok: 5,
            steps_failed: 0,
            duration_ms: 1234,
            results: vec![],
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("\"status\":\"ok\""));
        assert!(json.contains("\"stepsTotal\":5"));
        assert!(json.contains("\"durationMs\":1234"));
    }
}
