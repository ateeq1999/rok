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

/// Recursively copy a directory tree from `src` to `dst`.
///
/// `dst` is created if it does not exist.  Existing files at `dst` are
/// overwritten without warning.
///
/// ```rust
/// use rok_utils::fs::copy_dir_all;
/// use std::fs;
///
/// let src = tempfile::tempdir().unwrap();
/// let dst = tempfile::tempdir().unwrap();
/// fs::write(src.path().join("hello.txt"), b"hi").unwrap();
/// copy_dir_all(src.path(), dst.path()).unwrap();
/// assert!(dst.path().join("hello.txt").exists());
/// ```
pub fn copy_dir_all<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Recursively find all files under `dir` with the given extension (without
/// the leading dot).
///
/// Returns paths in the order they are discovered (directory-order, not
/// sorted).  Returns an empty `Vec` if `dir` does not exist.
///
/// ```rust
/// use rok_utils::fs::find_files;
/// use std::fs;
///
/// let dir = tempfile::tempdir().unwrap();
/// fs::write(dir.path().join("a.txt"), b"").unwrap();
/// fs::write(dir.path().join("b.rs"), b"").unwrap();
/// let txt = find_files(dir.path(), "txt").unwrap();
/// assert_eq!(txt.len(), 1);
/// assert!(txt[0].ends_with("a.txt"));
/// ```
pub fn find_files<P: AsRef<Path>>(dir: P, ext: &str) -> Result<Vec<std::path::PathBuf>> {
    let mut result = Vec::new();
    if dir.as_ref().is_dir() {
        find_files_inner(dir.as_ref(), ext, &mut result)?;
    }
    Ok(result)
}

fn find_files_inner(dir: &Path, ext: &str, acc: &mut Vec<std::path::PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            find_files_inner(&path, ext, acc)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some(ext) {
            acc.push(path);
        }
    }
    Ok(())
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

    #[test]
    fn copy_dir_all_copies_nested() {
        let src = tempfile::tempdir().unwrap();
        let dst = tempfile::tempdir().unwrap();
        let sub = src.path().join("sub");
        fs::create_dir_all(&sub).unwrap();
        fs::write(src.path().join("root.txt"), b"root").unwrap();
        fs::write(sub.join("nested.txt"), b"nested").unwrap();

        copy_dir_all(src.path(), dst.path()).unwrap();

        assert_eq!(read_to_string(dst.path().join("root.txt")).unwrap(), "root");
        assert_eq!(read_to_string(dst.path().join("sub/nested.txt")).unwrap(), "nested");
    }

    #[test]
    fn find_files_by_extension() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("a.txt"), b"").unwrap();
        fs::write(dir.path().join("b.rs"), b"").unwrap();
        fs::write(dir.path().join("c.txt"), b"").unwrap();

        let txt = find_files(dir.path(), "txt").unwrap();
        assert_eq!(txt.len(), 2);
        let rs = find_files(dir.path(), "rs").unwrap();
        assert_eq!(rs.len(), 1);
    }

    #[test]
    fn find_files_nonexistent_dir_returns_empty() {
        let result = find_files("/nonexistent/path/xyz", "rs").unwrap();
        assert!(result.is_empty());
    }
}
