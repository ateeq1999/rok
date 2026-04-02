use crate::schema::{RefactorChange, StepResult, StepTypeResult};
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use walkdir::WalkDir;

pub fn run(
    symbol: &str,
    rename_to: &str,
    path: &str,
    ext: &[String],
    dry_run: bool,
    whole_word: bool,
    cwd: &std::path::Path,
) -> StepResult {
    let start = Instant::now();
    let full_path = cwd.join(path);

    let pattern_str = if whole_word {
        format!(r"\b{}\b", regex::escape(symbol))
    } else {
        regex::escape(symbol)
    };

    let re = match Regex::new(&pattern_str) {
        Ok(r) => r,
        Err(e) => {
            return StepResult {
                index: 0,
                step_type: StepTypeResult::Refactor {
                    symbol: symbol.to_string(),
                    rename_to: rename_to.to_string(),
                    files_scanned: 0,
                    files_modified: 0,
                    total_replacements: 0,
                    dry_run,
                    changes: vec![],
                },
                status: format!("error: invalid pattern: {}", e),
                duration_ms: 0,
                stopped_pipeline: None,
            };
        }
    };

    let files: Vec<_> = WalkDir::new(&full_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            if ext.is_empty() {
                return true;
            }
            if let Some(file_ext) = e.path().extension() {
                let ext_str = file_ext.to_string_lossy().to_lowercase();
                return ext.iter().any(|ex| ex.to_lowercase() == ext_str);
            }
            false
        })
        .collect();

    let files_scanned = files.len();
    let all_changes: Arc<Mutex<Vec<RefactorChange>>> = Arc::new(Mutex::new(Vec::new()));
    let files_modified = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let total_replacements = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    files.par_iter().for_each(|entry| {
        let entry_path = entry.path();
        if let Ok(content) = fs::read_to_string(entry_path) {
            let matches: Vec<_> = re.find_iter(&content).collect();
            if matches.is_empty() {
                return;
            }

            let relative_path = entry_path
                .strip_prefix(cwd)
                .unwrap_or(entry_path)
                .to_string_lossy()
                .to_string();

            let mut changes = Vec::new();
            for (line_idx, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    let new_line = re.replace_all(line, rename_to).to_string();
                    changes.push(RefactorChange {
                        path: relative_path.clone(),
                        line: line_idx + 1,
                        old_text: line.to_string(),
                        new_text: new_line,
                    });
                }
            }

            total_replacements.fetch_add(matches.len(), std::sync::atomic::Ordering::Relaxed);

            if !dry_run {
                let new_content = re.replace_all(&content, rename_to).to_string();
                if fs::write(entry_path, &new_content).is_ok() {
                    files_modified.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            } else {
                files_modified.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }

            if let Ok(mut all) = all_changes.lock() {
                all.extend(changes);
            }
        }
    });

    let duration_ms = start.elapsed().as_millis() as u64;
    let changes = Arc::try_unwrap(all_changes)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();

    StepResult {
        index: 0,
        step_type: StepTypeResult::Refactor {
            symbol: symbol.to_string(),
            rename_to: rename_to.to_string(),
            files_scanned,
            files_modified: files_modified.load(std::sync::atomic::Ordering::Relaxed),
            total_replacements: total_replacements.load(std::sync::atomic::Ordering::Relaxed),
            dry_run,
            changes,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}
