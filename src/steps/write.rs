use crate::schema::{StepResult, StepTypeResult};
use similar::TextDiff;
use std::fs;
use std::time::Instant;

pub fn run(path: &str, content: &str, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let target_path = cwd.join(path);

    let original = fs::read_to_string(&target_path).unwrap_or_default();
    let new_content = content.to_string();
    let diff = if original != new_content {
        let d = TextDiff::from_lines(&original, &new_content);
        Some(d.unified_diff().header("a", "b").to_string())
    } else {
        None
    };

    let result = if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
    } else {
        Ok(())
    }
    .and_then(|_| fs::write(&target_path, content));

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Write {
                path: path.to_string(),
                diff,
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(_e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Write {
                path: path.to_string(),
                diff,
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}
