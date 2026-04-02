//! Path helpers.

use std::path::{Component, Path, PathBuf};

/// Normalize `path` by resolving `.` and `..` components lexically (without
/// touching the file system).  Absolute paths that walk above the root are
/// clamped at the root.
pub fn normalize<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in path.as_ref().components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                // Only pop if there's something to pop (and it isn't a root/prefix).
                match out.components().next_back() {
                    Some(Component::RootDir | Component::Prefix(_)) | None => {}
                    _ => {
                        out.pop();
                    }
                }
            }
            other => out.push(other),
        }
    }
    out
}

/// Return the file stem and extension as `(stem, ext)`.  Both are `""` when
/// the component is absent.
pub fn stem_ext<P: AsRef<Path>>(path: P) -> (String, String) {
    let p = path.as_ref();
    let stem = p
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    let ext = p
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    (stem, ext)
}

/// Replace the extension of `path` with `new_ext`.  If `new_ext` is empty the
/// extension (and its leading `.`) is removed.
pub fn with_extension<P: AsRef<Path>>(path: P, new_ext: &str) -> PathBuf {
    path.as_ref().with_extension(new_ext)
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_removes_dots() {
        assert_eq!(normalize("./foo/../bar/./baz"), PathBuf::from("bar/baz"));
    }

    #[test]
    fn normalize_absolute() {
        assert_eq!(normalize("/a/b/../c"), PathBuf::from("/a/c"));
    }

    #[test]
    fn normalize_no_above_root() {
        assert_eq!(normalize("/.."), PathBuf::from("/"));
    }

    #[test]
    fn stem_ext_splits_correctly() {
        assert_eq!(stem_ext("main.rs"), ("main".into(), "rs".into()));
        assert_eq!(stem_ext("no_ext"), ("no_ext".into(), "".into()));
    }
}
