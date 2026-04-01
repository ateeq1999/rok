//! Temporary file/directory fixtures.

use std::path::{Path, PathBuf};

/// A temporary directory that is deleted when dropped.
///
/// Thin wrapper around [`tempfile::TempDir`] with convenience helpers.
pub struct TempFixture {
    inner: tempfile::TempDir,
}

impl TempFixture {
    /// Create a new temporary directory.
    pub fn new() -> Self {
        Self {
            inner: tempfile::tempdir().expect("failed to create temp dir"),
        }
    }

    /// Return the path to the temporary directory.
    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    /// Write `content` to `filename` inside the temp dir.  Returns the full path.
    pub fn write(&self, filename: &str, content: &str) -> PathBuf {
        let path = self.inner.path().join(filename);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create_dir_all failed");
        }
        std::fs::write(&path, content).expect("write failed");
        path
    }

    /// Read a file inside the temp dir.
    pub fn read(&self, filename: &str) -> String {
        std::fs::read_to_string(self.inner.path().join(filename))
            .expect("read failed")
    }

    /// Return `true` if `filename` exists inside the temp dir.
    pub fn exists(&self, filename: &str) -> bool {
        self.inner.path().join(filename).exists()
    }
}

impl Default for TempFixture {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read() {
        let fix = TempFixture::new();
        fix.write("hello.txt", "world");
        assert_eq!(fix.read("hello.txt"), "world");
    }

    #[test]
    fn exists_check() {
        let fix = TempFixture::new();
        assert!(!fix.exists("nope.txt"));
        fix.write("yes.txt", "");
        assert!(fix.exists("yes.txt"));
    }

    #[test]
    fn nested_path() {
        let fix = TempFixture::new();
        fix.write("a/b/c.txt", "deep");
        assert!(fix.exists("a/b/c.txt"));
    }
}
