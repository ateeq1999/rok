use crate::schema::{PatchEdit, StepResult, StepTypeResult};
use similar::TextDiff;
use std::fs;
use std::time::Instant;

pub fn run(path: &str, edits: &[PatchEdit], cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let full_path = cwd.join(path);
    let original = fs::read_to_string(&full_path).unwrap_or_default();
    let mut content = original.clone();

    let mut edits_applied = 0;

    for edit in edits {
        if content.contains(&edit.find) {
            content = content.replace(&edit.find, &edit.replace);
            edits_applied += 1;
        }
    }

    let diff = TextDiff::from_lines(&original, &content);
    let unified_diff = diff.unified_diff().header("a", "b").to_string();

    let result = if content != original {
        fs::write(&full_path, &content)
    } else {
        Ok(())
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Patch {
                path: path.to_string(),
                edits_applied,
                diff: Some(unified_diff),
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(_e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Patch {
                path: path.to_string(),
                edits_applied,
                diff: Some(unified_diff),
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}
