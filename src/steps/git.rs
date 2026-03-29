use crate::schema::{GitOp, StepResult, StepTypeResult};
use std::process::Command;
use std::time::Instant;

pub fn run(op: &GitOp, args: &[String], cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let op_str = match op {
        GitOp::Status => "status",
        GitOp::Diff => "diff",
        GitOp::Log => "log",
        GitOp::Add => "add",
        GitOp::Commit => "commit",
        GitOp::Branch => "branch",
    };

    let output = match op {
        GitOp::Status => {
            let mut cmd = Command::new("git");
            cmd.args(["status", "--porcelain"]).current_dir(cwd);
            cmd.output()
        }
        GitOp::Diff => {
            let mut cmd_args = vec!["diff".to_string()];
            cmd_args.extend(args.iter().cloned());
            let mut cmd = Command::new("git");
            cmd.args(&cmd_args).current_dir(cwd);
            cmd.output()
        }
        GitOp::Log => {
            let mut cmd_args = vec!["log".to_string()];
            cmd_args.extend(args.iter().cloned());
            let mut cmd = Command::new("git");
            cmd.args(&cmd_args).current_dir(cwd);
            cmd.output()
        }
        GitOp::Add => {
            let mut cmd_args = vec!["add".to_string()];
            cmd_args.extend(args.iter().cloned());
            let mut cmd = Command::new("git");
            cmd.args(&cmd_args).current_dir(cwd);
            cmd.output()
        }
        GitOp::Commit => {
            let mut cmd_args = vec!["commit".to_string()];
            cmd_args.extend(args.iter().cloned());
            let mut cmd = Command::new("git");
            cmd.args(&cmd_args).current_dir(cwd);
            cmd.output()
        }
        GitOp::Branch => {
            let mut cmd_args = vec!["branch".to_string()];
            cmd_args.extend(args.iter().cloned());
            let mut cmd = Command::new("git");
            cmd.args(&cmd_args).current_dir(cwd);
            cmd.output()
        }
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);

            let json_output = if exit_code == 0 {
                match op {
                    GitOp::Status => {
                        let files: Vec<&str> = stdout.lines().collect();
                        serde_json::json!({
                            "files": files,
                            "clean": files.is_empty()
                        })
                    }
                    GitOp::Branch => {
                        let branches: Vec<&str> = stdout.lines().collect();
                        serde_json::json!({ "branches": branches })
                    }
                    GitOp::Log => {
                        let commits: Vec<&str> = stdout.lines().collect();
                        serde_json::json!({ "commits": commits })
                    }
                    _ => serde_json::json!({ "output": stdout }),
                }
            } else {
                serde_json::json!({
                    "error": stderr,
                    "exit_code": exit_code
                })
            };

            StepResult {
                index: 0,
                step_type: StepTypeResult::Git {
                    op: op_str.to_string(),
                    output: json_output,
                },
                status: if exit_code == 0 {
                    "ok".to_string()
                } else {
                    "error".to_string()
                },
                duration_ms,
                stopped_pipeline: None,
            }
        }
        Err(e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Git {
                op: op_str.to_string(),
                output: serde_json::json!({ "error": e.to_string() }),
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}
