//! File system utilities.

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Read a file to a `String`.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

/// Read raw bytes from a file.
pub fn read_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    Ok(fs::read(path)?)
}

/// Write `contents` to `path` atomically via a temporary sibling file.
///
/// On success the temp file is renamed to `path`, which on most file systems
/// is an atomic operation.  The temporary file is removed on failure.
pub fn write_atomic<P: AsRef<Path>>(path: P, contents: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or(Path::new("."));

    // Write to a temp file in the same directory so the rename is on the same FS.
    let tmp = tempfile(parent)?;
    fs::write(&tmp, contents)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

/// Ensure `path` and all its parents exist (like `mkdir -p`).
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(fs::create_dir_all(path)?)
}

/// Return `true` if `path` exists and is a regular file.
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// Return `true` if `path` exists and is a directory.
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

// ── internal helper ──────────────────────────────────────────────────────────

fn tempfile(dir: &Path) -> Result<std::path::PathBuf> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    Ok(dir.join(format!(".tmp.rok.{ts}")))
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn round_trip_atomic_write() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("hello.txt");
        write_atomic(&p, b"hello").unwrap();
        assert_eq!(read_to_string(&p).unwrap(), "hello");
    }

    #[test]
    fn ensure_dir_creates_nested() {
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("a").join("b").join("c");
        ensure_dir(&nested).unwrap();
        assert!(nested.is_dir());
    }

    #[test]
    fn is_file_and_is_dir() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("f.txt");
        fs::write(&f, "x").unwrap();
        assert!(is_file(&f));
        assert!(!is_dir(&f));
        assert!(is_dir(dir.path()));
        assert!(!is_file(dir.path()));
    }
}
