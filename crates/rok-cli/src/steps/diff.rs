use crate::schema::{DiffFormat, StepResult, StepTypeResult};
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::time::Instant;

pub fn run(a: &str, b: &str, format: &DiffFormat, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let a_path = cwd.join(a);
    let b_path = cwd.join(b);

    let a_content = fs::read_to_string(&a_path).unwrap_or_default();
    let b_content = fs::read_to_string(&b_path).unwrap_or_default();

    let diff = TextDiff::from_lines(&a_content, &b_content);

    let mut added = 0;
    let mut removed = 0;
    let mut changed_sections = Vec::new();

    let mut in_section = false;
    for group in diff.grouped_ops(3) {
        for op in group {
            for change in diff.iter_changes(&op) {
                match change.tag() {
                    ChangeTag::Insert => added += 1,
                    ChangeTag::Delete => removed += 1,
                    ChangeTag::Equal => {}
                }
            }
            if !in_section {
                changed_sections.push(format!("line {}", op.old_range().start));
                in_section = true;
            }
        }
    }

    let is_identical = added == 0 && removed == 0;

    let unified_diff = match format {
        DiffFormat::Unified => Some(diff.unified_diff().header(a, b).to_string()),
        _ => None,
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Diff {
            a: a.to_string(),
            b: b.to_string(),
            added,
            removed,
            changed_sections,
            is_identical,
            unified_diff,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}
