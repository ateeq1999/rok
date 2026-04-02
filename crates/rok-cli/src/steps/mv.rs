use crate::schema::{StepResult, StepTypeResult};
use std::fs;
use std::time::Instant;

pub fn run(from: &str, to: &str, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let from_path = cwd.join(from);
    let to_path = cwd.join(to);

    let result = if to_path.is_dir() {
        let file_name = from_path.file_name().unwrap_or_default();
        let dest = to_path.join(file_name);
        if let Some(parent) = dest.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::rename(&from_path, &dest)
    } else {
        if let Some(parent) = to_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::rename(&from_path, &to_path)
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Mv {
                from: from.to_string(),
                to: to.to_string(),
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(_e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Mv {
                from: from.to_string(),
                to: to.to_string(),
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}
