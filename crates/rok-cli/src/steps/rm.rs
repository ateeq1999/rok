use crate::schema::{StepResult, StepTypeResult};
use std::fs;
use std::time::Instant;

pub fn run(path: &str, recursive: bool, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let target_path = cwd.join(path);

    let result = if recursive {
        fs::remove_dir_all(&target_path)
    } else {
        fs::remove_file(&target_path)
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Rm {
                path: path.to_string(),
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(_e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Rm {
                path: path.to_string(),
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}
