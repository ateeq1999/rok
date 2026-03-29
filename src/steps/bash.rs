use crate::schema::{StepResult, StepTypeResult};
use std::process::Command;
use std::time::Instant;

pub fn run(
    cmd: &str,
    cwd: &std::path::Path,
    env: &std::collections::HashMap<String, String>,
) -> StepResult {
    let start = Instant::now();

    let mut command = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(["/C", cmd]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", cmd]);
        c
    };

    command.current_dir(cwd).envs(env);

    let output = command.output();

    let duration_ms = start.elapsed().as_millis() as u64;

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);

            StepResult {
                index: 0,
                step_type: StepTypeResult::Bash {
                    cmd: cmd.to_string(),
                    stdout,
                    stderr,
                    exit_code,
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
            step_type: StepTypeResult::Bash {
                cmd: cmd.to_string(),
                stdout: String::new(),
                stderr: e.to_string(),
                exit_code: -1,
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}
