use crate::schema::{FileContent, StepResult, StepTypeResult};
use globset::Glob;
use std::fs;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

pub fn run(
    path: &str,
    filter_imports: Option<&str>,
    filter_exports: Option<&str>,
    since: Option<&str>,
    cwd: &std::path::Path,
) -> StepResult {
    let full_path = cwd.join(path);
    let mut files = Vec::new();
    let mut read_error = None;
    let mut files_filtered = 0;
    let mut filter_reason: Option<String> = None;

    let since_timestamp = since.and_then(|s| {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
            Some(dt.timestamp())
        } else if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
            Some(ndt.and_utc().timestamp())
        } else if let Ok(nd) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Some(nd.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
        } else {
            None
        }
    });

    let paths: Vec<_> = if path.contains('*') {
        if let Ok(glob) = Glob::new(path) {
            let matcher = glob.compile_matcher();
            WalkDir::new(cwd)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .filter(|e| matcher.is_match(e.path()))
                .map(|e| e.path().to_path_buf())
                .collect()
        } else {
            vec![full_path.clone()]
        }
    } else if full_path.is_dir() {
        WalkDir::new(&full_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_path_buf())
            .collect()
    } else if full_path.is_file() {
        vec![full_path]
    } else {
        vec![]
    };

    for entry_path in paths {
        let should_skip = if let Some(filter_ts) = since_timestamp {
            if let Ok(metadata) = entry_path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    let modified_ts = modified
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);
                    modified_ts < filter_ts
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if should_skip {
            files_filtered += 1;
            continue;
        }

        if let Ok(content) = fs::read_to_string(&entry_path) {
            let rel_path = entry_path
                .strip_prefix(cwd)
                .unwrap_or(&entry_path)
                .to_string_lossy()
                .to_string();

            if let Some(import_filter) = filter_imports {
                if !content.contains(import_filter) {
                    files_filtered += 1;
                    continue;
                }
            }

            if let Some(export_filter) = filter_exports {
                let has_export = content.contains(&format!("export {}", export_filter))
                    || content.contains(&format!("export const {}", export_filter))
                    || content.contains(&format!("export function {}", export_filter))
                    || content.contains(&format!("export class {}", export_filter))
                    || content.contains(&format!("export default {}", export_filter))
                    || content.contains(&format!("pub fn {}", export_filter))
                    || content.contains(&format!("pub const {}", export_filter))
                    || content.contains(&format!("pub struct {}", export_filter))
                    || content.contains(&format!("pub enum {}", export_filter))
                    || content.contains(&format!("def {}", export_filter))
                    || content.contains(&format!("func {}", export_filter));

                if !has_export {
                    files_filtered += 1;
                    continue;
                }
            }

            files.push(FileContent {
                path: rel_path,
                content,
            });
        } else {
            read_error = Some("Failed to read file".to_string());
        }
    }

    if files_filtered > 0 {
        filter_reason = Some(format!("{} files filtered by criteria", files_filtered));
    }

    StepResult {
        index: 0,
        step_type: StepTypeResult::Read {
            path: path.to_string(),
            files,
            files_filtered,
            filter_reason,
        },
        status: if read_error.is_some() {
            "error".to_string()
        } else {
            "ok".to_string()
        },
        duration_ms: 0,
        stopped_pipeline: None,
    }
}
