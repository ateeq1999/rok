use crate::schema::{StepResult, StepTypeResult};
use std::fs;
use std::time::Instant;

pub fn run(path: &str, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let target_path = cwd.join(path);
    let result = fs::create_dir_all(&target_path);

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Mkdir {
                path: path.to_string(),
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Mkdir {
                path: path.to_string(),
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}
