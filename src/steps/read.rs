use crate::schema::{FileContent, StepResult, StepTypeResult};
use globset::Glob;
use std::fs;
use std::time::Instant;
use walkdir::WalkDir;

pub fn run(path: &str, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let full_path = cwd.join(path);

    let mut files = Vec::new();
    let mut read_error = None;

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
        match fs::read_to_string(&entry_path) {
            Ok(content) => {
                let rel_path = entry_path
                    .strip_prefix(cwd)
                    .unwrap_or(&entry_path)
                    .to_string_lossy()
                    .to_string();
                files.push(FileContent {
                    path: rel_path,
                    content,
                });
            }
            Err(e) => {
                read_error = Some(e.to_string());
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Read {
            path: path.to_string(),
            files,
        },
        status: if read_error.is_some() {
            "error".to_string()
        } else {
            "ok".to_string()
        },
        duration_ms,
        stopped_pipeline: None,
    }
}
