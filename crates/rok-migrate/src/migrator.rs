//! [`Migrator`] — manages an ordered set of migrations.

use crate::error::MigrateError;
use crate::migration::Migration;

/// Holds a set of [`Migration`]s and answers queries about their state.
#[derive(Default)]
pub struct Migrator {
    migrations: Vec<Migration>,
}

impl Migrator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a migration.  Migrations are automatically sorted by version.
    ///
    /// # Errors
    ///
    /// Returns [`MigrateError::DuplicateVersion`] if a migration with the same
    /// version is already registered.
    pub fn add(&mut self, migration: Migration) -> Result<(), MigrateError> {
        if self.migrations.iter().any(|m| m.version == migration.version) {
            return Err(MigrateError::DuplicateVersion(migration.version));
        }
        self.migrations.push(migration);
        self.migrations.sort_by_key(|m| m.version);
        Ok(())
    }

    /// Return all registered migrations in version order.
    pub fn all(&self) -> &[Migration] {
        &self.migrations
    }

    /// Return migrations whose version is **not** in `applied`.
    pub fn pending<'a>(&'a self, applied: &[u64]) -> Vec<&'a Migration> {
        self.migrations
            .iter()
            .filter(|m| !applied.contains(&m.version))
            .collect()
    }

    /// Return migrations that have been applied (version is in `applied`).
    pub fn applied<'a>(&'a self, applied: &[u64]) -> Vec<&'a Migration> {
        self.migrations
            .iter()
            .filter(|m| applied.contains(&m.version))
            .collect()
    }

    /// Return the SQL for creating the migration-history tracking table.
    pub fn history_table_sql() -> &'static str {
        r#"CREATE TABLE IF NOT EXISTS _rok_migrations (
    version     BIGINT      PRIMARY KEY,
    name        TEXT        NOT NULL,
    applied_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);"#
    }

    /// Return the INSERT SQL for recording that a migration was applied.
    pub fn record_sql(migration: &Migration) -> String {
        format!(
            "INSERT INTO _rok_migrations (version, name) VALUES ({}, '{}');",
            migration.version,
            migration.name.replace('\'', "''")
        )
    }

    /// Return the DELETE SQL for removing a migration record (rollback).
    pub fn revert_record_sql(version: u64) -> String {
        format!("DELETE FROM _rok_migrations WHERE version = {version};")
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> Migrator {
        let mut m = Migrator::new();
        m.add(Migration::new(1, "create_users", "CREATE TABLE users (id INT);", None))
            .unwrap();
        m.add(Migration::new(2, "add_email", "ALTER TABLE users ADD email TEXT;", None))
            .unwrap();
        m.add(Migration::new(3, "add_index", "CREATE INDEX ON users(email);", None))
            .unwrap();
        m
    }

    #[test]
    fn pending_all_when_none_applied() {
        let m = make();
        assert_eq!(m.pending(&[]).len(), 3);
    }

    #[test]
    fn pending_skips_applied() {
        let m = make();
        let p = m.pending(&[1, 3]);
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].version, 2);
    }

    #[test]
    fn duplicate_version_rejected() {
        let mut m = Migrator::new();
        m.add(Migration::new(1, "a", "SQL;", None)).unwrap();
        let err = m.add(Migration::new(1, "b", "SQL;", None));
        assert!(matches!(err, Err(MigrateError::DuplicateVersion(1))));
    }

    #[test]
    fn sorted_by_version() {
        let mut m = Migrator::new();
        m.add(Migration::new(3, "c", "SQL;", None)).unwrap();
        m.add(Migration::new(1, "a", "SQL;", None)).unwrap();
        m.add(Migration::new(2, "b", "SQL;", None)).unwrap();
        let versions: Vec<u64> = m.all().iter().map(|m| m.version).collect();
        assert_eq!(versions, vec![1, 2, 3]);
    }

    #[test]
    fn history_table_sql_has_version_col() {
        assert!(Migrator::history_table_sql().contains("version"));
        assert!(Migrator::history_table_sql().contains("_rok_migrations"));
    }
}
