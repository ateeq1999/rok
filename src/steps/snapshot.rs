use crate::schema::StepResult;
use crate::schema::StepTypeResult;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

pub fn snapshot(path: &str, id: &str, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let snapshot_dir = cwd.join(".rok").join("snapshots");
    let snapshot_path: PathBuf = snapshot_dir.join(format!("{}.tar.gz", id));

    let result = create_tarball(cwd.join(path), &snapshot_path);

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Snapshot {
                path: path.to_string(),
                id: id.to_string(),
                archived: true,
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(_e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Snapshot {
                path: path.to_string(),
                id: id.to_string(),
                archived: false,
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}

pub fn restore(id: &str, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let snapshot_path: PathBuf = cwd
        .join(".rok")
        .join("snapshots")
        .join(format!("{}.tar.gz", id));

    let result = extract_tarball(&snapshot_path, cwd);

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => StepResult {
            index: 0,
            step_type: StepTypeResult::Restore {
                id: id.to_string(),
                restored: true,
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
        Err(_e) => StepResult {
            index: 0,
            step_type: StepTypeResult::Restore {
                id: id.to_string(),
                restored: false,
            },
            status: "error".to_string(),
            duration_ms,
            stopped_pipeline: None,
        },
    }
}

fn create_tarball(source: std::path::PathBuf, dest: &PathBuf) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(dest.parent().unwrap())?;
    let file = File::create(dest)?;
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for entry in WalkDir::new(&source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative = path.strip_prefix(&source).unwrap_or(path);

        if path.is_file() {
            let _ = tar.append_path_with_name(path, relative);
        } else if path.is_dir() && !relative.as_os_str().is_empty() {
            let _ = tar.append_dir(relative, path);
        }
    }

    tar.finish()?;
    Ok(())
}

fn extract_tarball(source: &PathBuf, dest: &std::path::Path) -> Result<(), std::io::Error> {
    let file = File::open(source)?;
    let dec = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(dec);

    archive.unpack(dest)?;
    Ok(())
}
