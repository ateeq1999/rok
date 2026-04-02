//! Load migrations from a directory of `{version}_{name}.sql` files.
//!
//! ## File format
//!
//! ```sql
//! -- UP
//! CREATE TABLE users (id BIGINT PRIMARY KEY, name TEXT NOT NULL);
//!
//! -- DOWN
//! DROP TABLE users;
//! ```
//!
//! The `-- DOWN` section and everything after it is treated as the rollback
//! SQL.  If no `-- DOWN` marker is present the migration is irreversible.
//!
//! Filenames must be `{version}_{name}.sql`, e.g. `0001_create_users.sql`.

use std::path::Path;

use crate::error::MigrateError;
use crate::migration::Migration;

/// Load all `*.sql` files from `dir` as [`Migration`]s, sorted by version.
///
/// ```rust
/// use rok_migrate::loader::load_from_dir;
///
/// // An empty directory returns an empty list.
/// let dir = tempfile::tempdir().unwrap();
/// let migrations = load_from_dir(dir.path()).unwrap();
/// assert!(migrations.is_empty());
/// ```
pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Vec<Migration>, MigrateError> {
    let mut migrations = Vec::new();

    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("sql") {
            continue;
        }

        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let (version, name) = parse_filename(&filename, &path)?;
        let content = std::fs::read_to_string(&path)?;
        let (up_sql, down_sql) = parse_content(&content);

        migrations.push(Migration::new(version, name, up_sql, down_sql));
    }

    migrations.sort_by_key(|m| m.version);
    Ok(migrations)
}

// ── internal helpers ─────────────────────────────────────────────────────────

fn parse_filename(stem: &str, path: &Path) -> Result<(u64, String), MigrateError> {
    let mut parts = stem.splitn(2, '_');
    let version_str = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MigrateError::InvalidFilename(path.display().to_string()))?;
    let name = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MigrateError::InvalidFilename(path.display().to_string()))?;

    let version = version_str
        .parse::<u64>()
        .map_err(|_| MigrateError::InvalidFilename(path.display().to_string()))?;

    Ok((version, name.to_string()))
}

fn parse_content(content: &str) -> (String, Option<String>) {
    // Strip optional `-- UP` header.
    let content = content.trim();
    let content = content
        .strip_prefix("-- UP")
        .map(|s| s.trim_start())
        .unwrap_or(content);

    // Split on `-- DOWN`.
    if let Some((up, down)) = content.split_once("-- DOWN") {
        let up = up.trim().to_string();
        let down = down.trim().to_string();
        (up, if down.is_empty() { None } else { Some(down) })
    } else {
        (content.to_string(), None)
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, name: &str, content: &str) {
        std::fs::write(dir.join(name), content).unwrap();
    }

    #[test]
    fn loads_up_and_down() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "0001_create_users.sql",
            "-- UP\nCREATE TABLE users (id INT);\n\n-- DOWN\nDROP TABLE users;\n",
        );
        let ms = load_from_dir(dir.path()).unwrap();
        assert_eq!(ms.len(), 1);
        assert_eq!(ms[0].version, 1);
        assert_eq!(ms[0].name, "create_users");
        assert!(ms[0].up_sql.contains("CREATE TABLE"));
        assert!(ms[0].down_sql.as_deref().unwrap().contains("DROP TABLE"));
    }

    #[test]
    fn loads_up_only() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "0002_add_index.sql",
            "CREATE INDEX ON users(email);",
        );
        let ms = load_from_dir(dir.path()).unwrap();
        assert_eq!(ms[0].version, 2);
        assert!(ms[0].down_sql.is_none());
    }

    #[test]
    fn sorted_by_version() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "0003_c.sql", "-- c");
        write(dir.path(), "0001_a.sql", "-- a");
        write(dir.path(), "0002_b.sql", "-- b");
        let ms = load_from_dir(dir.path()).unwrap();
        let versions: Vec<u64> = ms.iter().map(|m| m.version).collect();
        assert_eq!(versions, vec![1, 2, 3]);
    }

    #[test]
    fn non_sql_files_ignored() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "0001_init.sql", "SELECT 1;");
        write(dir.path(), "README.md", "# migrations");
        let ms = load_from_dir(dir.path()).unwrap();
        assert_eq!(ms.len(), 1);
    }

    #[test]
    fn empty_dir_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let ms = load_from_dir(dir.path()).unwrap();
        assert!(ms.is_empty());
    }
}
