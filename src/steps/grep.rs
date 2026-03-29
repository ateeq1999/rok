use crate::schema::{GrepMatch, StepResult, StepTypeResult};
use regex::Regex;
use std::fs;
use std::time::Instant;
use walkdir::WalkDir;

pub fn run(
    pattern: &str,
    path: &str,
    ext: &[String],
    regex_mode: bool,
    cwd: &std::path::Path,
) -> StepResult {
    let start = Instant::now();

    let full_path = cwd.join(path);
    let pattern = if regex_mode {
        pattern.to_string()
    } else {
        regex::escape(pattern)
    };

    let re = Regex::new(&pattern).unwrap_or_else(|_| Regex::new(".").unwrap());

    let mut matches = Vec::new();

    for entry in WalkDir::new(&full_path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();

        if !entry_path.is_file() {
            continue;
        }

        if !ext.is_empty() {
            if let Some(ext_match) = entry_path.extension() {
                let ext_str = ext_match.to_string_lossy().to_lowercase();
                if !ext.iter().any(|e| e.to_lowercase() == ext_str) {
                    continue;
                }
            } else {
                continue;
            }
        }

        if let Ok(content) = fs::read_to_string(entry_path) {
            for (line_num, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    let rel_path = entry_path
                        .strip_prefix(cwd)
                        .unwrap_or(entry_path)
                        .to_string_lossy()
                        .to_string();
                    matches.push(GrepMatch {
                        path: rel_path,
                        line: line_num + 1,
                        text: line.to_string(),
                    });
                }
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Grep {
            pattern: pattern.to_string(),
            matches,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}
