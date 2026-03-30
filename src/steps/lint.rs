use crate::schema::{LintError, LintTool, StepResult, StepTypeResult};
use std::process::Command;
use std::time::Instant;
use walkdir::WalkDir;

pub fn run(path: &str, tool: &LintTool, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let detected_tool = if matches!(tool, LintTool::Auto) {
        detect_linter(cwd)
    } else {
        tool.clone()
    };

    let (errors, warnings, error_list) = match detected_tool {
        LintTool::Eslint => run_eslint(path, cwd),
        LintTool::Biome => run_biome(path, cwd),
        LintTool::Clippy => run_clippy(path, cwd),
        LintTool::Ruff => run_ruff(path, cwd),
        _ => (0, 0, Vec::new()),
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Lint {
            errors_count: errors,
            warnings_count: warnings,
            errors: error_list,
        },
        status: if errors > 0 {
            "error".to_string()
        } else {
            "ok".to_string()
        },
        duration_ms,
        stopped_pipeline: None,
    }
}

fn detect_linter(cwd: &std::path::Path) -> LintTool {
    let files: Vec<_> = WalkDir::new(cwd)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.path()
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
        })
        .collect();

    if files
        .iter()
        .any(|f| f == "biome.json" || f == "biome.jsonc")
    {
        return LintTool::Biome;
    }
    if files
        .iter()
        .any(|f| f == ".eslintrc.json" || f == ".eslintrc.js" || f == "eslint.config.js")
    {
        return LintTool::Eslint;
    }
    if files
        .iter()
        .any(|f| f == "pyproject.toml" || f == "ruff.toml")
    {
        return LintTool::Ruff;
    }
    if files.iter().any(|f| f == "Cargo.toml") {
        return LintTool::Clippy;
    }

    LintTool::Biome
}

fn run_eslint(path: &str, cwd: &std::path::Path) -> (usize, usize, Vec<LintError>) {
    let output = Command::new("npx")
        .args(["eslint", path, "--format", "json"])
        .current_dir(cwd)
        .output();

    parse_eslint_output(output)
}

fn run_biome(path: &str, cwd: &std::path::Path) -> (usize, usize, Vec<LintError>) {
    let output = Command::new("npx")
        .args(["@biomejs/biome", "lint", path, "--formatter-enabled=false"])
        .current_dir(cwd)
        .output();

    parse_biome_output(output)
}

fn run_clippy(_path: &str, cwd: &std::path::Path) -> (usize, usize, Vec<LintError>) {
    let output = Command::new("cargo")
        .args(["clippy", "--message-format=json"])
        .current_dir(cwd)
        .output();

    parse_clippy_output(output)
}

fn run_ruff(path: &str, cwd: &std::path::Path) -> (usize, usize, Vec<LintError>) {
    let output = Command::new("ruff")
        .args(["check", path, "--output-format", "json"])
        .current_dir(cwd)
        .output();

    parse_ruff_output(output)
}

fn parse_eslint_output(
    output: Result<std::process::Output, std::io::Error>,
) -> (usize, usize, Vec<LintError>) {
    let mut errors = 0;
    let mut warnings = 0;
    let mut error_list = Vec::new();

    if let Ok(output) = output {
        if let Ok(json) =
            serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&output.stdout))
        {
            if let Some(files) = json.as_array() {
                for file in files {
                    if let Some(messages) = file.get("messages").and_then(|m| m.as_array()) {
                        for msg in messages {
                            let severity =
                                msg.get("severity").and_then(|s| s.as_u64()).unwrap_or(0);
                            let line =
                                msg.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
                            let message = msg.get("message").and_then(|m| m.as_str()).unwrap_or("");
                            let rule = msg
                                .get("ruleId")
                                .and_then(|r| r.as_str())
                                .unwrap_or("unknown");

                            let lint_error = LintError {
                                file: msg
                                    .get("filePath")
                                    .map(|p| p.to_string())
                                    .unwrap_or_default(),
                                line,
                                rule: rule.to_string(),
                                message: message.to_string(),
                                severity: if severity >= 2 {
                                    "error".to_string()
                                } else {
                                    "warning".to_string()
                                },
                            };

                            if severity >= 2 {
                                errors += 1;
                            } else {
                                warnings += 1;
                            }
                            error_list.push(lint_error);
                        }
                    }
                }
            }
        }
    }

    (errors, warnings, error_list)
}

fn parse_biome_output(
    output: Result<std::process::Output, std::io::Error>,
) -> (usize, usize, Vec<LintError>) {
    let mut errors = 0;
    let mut warnings = 0;
    let mut error_list = Vec::new();

    if let Ok(output) = output {
        if let Ok(json) =
            serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&output.stdout))
        {
            if let Some(diagnostics) = json.get("diagnostics").and_then(|d| d.as_array()) {
                for diag in diagnostics {
                    let severity = diag
                        .get("severity")
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown");
                    let line = diag
                        .get("span")
                        .and_then(|s| s.get("start"))
                        .and_then(|p| p.get("line"))
                        .and_then(|l| l.as_u64())
                        .unwrap_or(0) as usize;
                    let message = diag.get("message").and_then(|m| m.as_str()).unwrap_or("");
                    let code = diag
                        .get("code")
                        .and_then(|c| c.as_str())
                        .unwrap_or("unknown");

                    let lint_error = LintError {
                        file: diag
                            .get("file_path")
                            .map(|p| p.to_string())
                            .unwrap_or_default(),
                        line,
                        rule: code.to_string(),
                        message: message.to_string(),
                        severity: if severity == "error" {
                            "error".to_string()
                        } else {
                            "warning".to_string()
                        },
                    };

                    if severity == "error" {
                        errors += 1;
                    } else {
                        warnings += 1;
                    }
                    error_list.push(lint_error);
                }
            }
        }
    }

    (errors, warnings, error_list)
}

fn parse_clippy_output(
    output: Result<std::process::Output, std::io::Error>,
) -> (usize, usize, Vec<LintError>) {
    let errors = 0;
    let mut warnings = 0;
    let mut error_list = Vec::new();

    if let Ok(output) = output {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                let reason = json.get("reason").and_then(|r| r.as_str()).unwrap_or("");
                if reason == "compiler-message" {
                    if let Some(span) = json.get("span") {
                        let line =
                            span.get("line_start").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
                        let message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");
                        let code = json.get("code").and_then(|c| c.as_str()).unwrap_or("");

                        error_list.push(LintError {
                            file: span
                                .get("file_name")
                                .map(|p| p.to_string())
                                .unwrap_or_default(),
                            line,
                            rule: code.to_string(),
                            message: message.to_string(),
                            severity: "warning".to_string(),
                        });
                        warnings += 1;
                    }
                }
            }
        }
    }

    (errors, warnings, error_list)
}

fn parse_ruff_output(
    output: Result<std::process::Output, std::io::Error>,
) -> (usize, usize, Vec<LintError>) {
    let mut errors = 0;
    let warnings = 0;
    let mut error_list = Vec::new();

    if let Ok(output) = output {
        if let Ok(json) =
            serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&output.stdout))
        {
            if let Some(items) = json.as_array() {
                for item in items {
                    let filename = item.get("filename").and_then(|f| f.as_str()).unwrap_or("");
                    if let Some(violations) = item.get("violations").and_then(|v| v.as_array()) {
                        for v in violations {
                            let line = v
                                .get("location")
                                .and_then(|l| l.get("row"))
                                .and_then(|r| r.as_u64())
                                .unwrap_or(0) as usize;
                            let message = v.get("message").and_then(|m| m.as_str()).unwrap_or("");
                            let code = v.get("code").and_then(|c| c.as_str()).unwrap_or("");

                            error_list.push(LintError {
                                file: filename.to_string(),
                                line,
                                rule: code.to_string(),
                                message: message.to_string(),
                                severity: "error".to_string(),
                            });
                            errors += 1;
                        }
                    }
                }
            }
        }
    }

    (errors, warnings, error_list)
}
