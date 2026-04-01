use crate::schema::{StepResult, StepTypeResult};
use std::fs;
use std::time::Instant;
use walkdir::WalkDir;

pub fn run(from: &str, to: &str, recursive: bool, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let from_path = cwd.join(from);
    let to_path = cwd.join(to);

    let result = if recursive && from_path.is_dir() {
        copy_dir_recursive(&from_path, &to_path)
    } else if from_path.is_file() {
        if let Some(parent) = to_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::copy(&from_path, &to_path).map(|_| ())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Source not found",
        ))
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Cp {
                from: from.to_string(),
                to: to.to_string(),
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(_e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Cp {
                from: from.to_string(),
                to: to.to_string(),
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let src_path = entry.path();
        let dst_path = dst.join(src_path.strip_prefix(src).unwrap());

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(src_path, dst_path)?;
        }
    }

    Ok(())
}
