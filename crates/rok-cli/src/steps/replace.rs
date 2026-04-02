use crate::schema::{StepResult, StepTypeResult};
use glob::Pattern;
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use walkdir::WalkDir;

#[allow(clippy::too_many_arguments)]
pub fn run(
    pattern: &str,
    replacement: &str,
    path: &str,
    ext: &[String],
    regex_mode: bool,
    whole_word: bool,
    glob_pattern: Option<&str>,
    cwd: &std::path::Path,
) -> StepResult {
    let start = Instant::now();

    let full_path = cwd.join(path);

    let pattern_str = if whole_word && !regex_mode {
        format!(r"\b{}\b", regex::escape(pattern))
    } else if regex_mode {
        pattern.to_string()
    } else {
        regex::escape(pattern)
    };

    let re = match Regex::new(&pattern_str) {
        Ok(r) => r,
        Err(_) => {
            return StepResult {
                index: 0,
                step_type: StepTypeResult::Replace {
                    pattern: pattern_str,
                    replacement: replacement.to_string(),
                    files_scanned: 0,
                    files_modified: 0,
                    total_replacements: 0,
                },
                status: "error".to_string(),
                duration_ms: 0,
                stopped_pipeline: None,
            };
        }
    };

    let glob_pattern = glob_pattern.and_then(|g| Pattern::new(g).ok());

    let files: Vec<_> = WalkDir::new(&full_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            if let Some(ref gp) = glob_pattern {
                let relative = e.path().strip_prefix(cwd).unwrap_or(e.path());
                if !gp.matches(&relative.to_string_lossy()) {
                    return false;
                }
            }
            if ext.is_empty() {
                return true;
            }
            if let Some(ext_match) = e.path().extension() {
                let ext_str = ext_match.to_string_lossy().to_lowercase();
                return ext.iter().any(|ex| ex.to_lowercase() == ext_str);
            }
            false
        })
        .collect();

    let files_scanned = files.len();
    let total_replacements = AtomicUsize::new(0);
    let files_modified = AtomicUsize::new(0);

    files.par_iter().for_each(|entry| {
        let entry_path = entry.path();

        if let Ok(content) = fs::read_to_string(entry_path) {
            let matches: Vec<_> = re.find_iter(&content).collect();

            if !matches.is_empty() {
                let new_content = re.replace_all(&content, replacement).to_string();

                if new_content != content {
                    total_replacements.fetch_add(matches.len(), Ordering::Relaxed);

                    if fs::write(entry_path, &new_content).is_ok() {
                        files_modified.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }
    });

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Replace {
            pattern: pattern_str,
            replacement: replacement.to_string(),
            files_scanned,
            files_modified: files_modified.load(Ordering::Relaxed),
            total_replacements: total_replacements.load(Ordering::Relaxed),
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}
