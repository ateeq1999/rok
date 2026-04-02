//! [`Migrator`] — manages an ordered set of migrations.

use crate::error::MigrateError;
use crate::migration::Migration;

// ── Execution plan ────────────────────────────────────────────────────────────

/// Which direction a planned migration step runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

/// A single step in an execution plan: the migration metadata plus the
/// complete SQL to execute (including the history-table bookkeeping statement).
#[derive(Debug)]
pub struct MigrationPlan<'a> {
    pub migration: &'a Migration,
    /// Full SQL to send to the database, ready to execute as a single block.
    pub sql: String,
    pub direction: Direction,
}

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
        if self
            .migrations
            .iter()
            .any(|m| m.version == migration.version)
        {
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

    /// Build the execution plan for applying all pending migrations.
    ///
    /// Returns one [`MigrationPlan`] per pending migration in version order.
    /// Each plan's `sql` is `up_sql` followed by the history-table INSERT,
    /// ready to execute as an atomic transaction.
    pub fn plan_up<'a>(&'a self, applied: &[u64]) -> Vec<MigrationPlan<'a>> {
        self.pending(applied)
            .into_iter()
            .map(|m| MigrationPlan {
                migration: m,
                sql: format!("{}\n{}", m.up_sql, Self::record_sql(m)),
                direction: Direction::Up,
            })
            .collect()
    }

    /// Build the execution plan for rolling back applied migrations.
    ///
    /// Steps are returned in **reverse** version order (newest first).
    /// `steps` caps the number of migrations to roll back; `None` rolls back
    /// all applied migrations.
    ///
    /// # Errors
    ///
    /// Returns [`MigrateError::Irreversible`] if any targeted migration has no
    /// `down_sql`.
    pub fn plan_down<'a>(
        &'a self,
        applied: &[u64],
        steps: Option<usize>,
    ) -> Result<Vec<MigrationPlan<'a>>, MigrateError> {
        let mut to_revert = self.applied(applied);
        to_revert.reverse(); // newest-first
        if let Some(n) = steps {
            to_revert.truncate(n);
        }
        to_revert
            .into_iter()
            .map(|m| {
                let down = m
                    .down_sql
                    .as_ref()
                    .ok_or(MigrateError::Irreversible(m.version))?;
                Ok(MigrationPlan {
                    migration: m,
                    sql: format!("{}\n{}", down, Self::revert_record_sql(m.version)),
                    direction: Direction::Down,
                })
            })
            .collect()
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> Migrator {
        let mut m = Migrator::new();
        m.add(Migration::new(
            1,
            "create_users",
            "CREATE TABLE users (id INT);",
            None,
        ))
        .unwrap();
        m.add(Migration::new(
            2,
            "add_email",
            "ALTER TABLE users ADD email TEXT;",
            None,
        ))
        .unwrap();
        m.add(Migration::new(
            3,
            "add_index",
            "CREATE INDEX ON users(email);",
            None,
        ))
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

    #[test]
    fn plan_up_returns_pending_in_order() {
        let m = make();
        let plans = m.plan_up(&[1]); // version 1 already applied
        assert_eq!(plans.len(), 2);
        assert_eq!(plans[0].migration.version, 2);
        assert_eq!(plans[1].migration.version, 3);
        assert!(plans[0].sql.contains("ALTER TABLE"));
        assert!(plans[0].sql.contains("INSERT INTO _rok_migrations"));
        assert!(matches!(plans[0].direction, Direction::Up));
    }

    #[test]
    fn plan_down_returns_applied_in_reverse() {
        let mut m = Migrator::new();
        m.add(Migration::new(
            1,
            "a",
            "CREATE TABLE a;",
            Some("DROP TABLE a;".to_string()),
        ))
        .unwrap();
        m.add(Migration::new(
            2,
            "b",
            "CREATE TABLE b;",
            Some("DROP TABLE b;".to_string()),
        ))
        .unwrap();
        let plans = m.plan_down(&[1, 2], None).unwrap();
        assert_eq!(plans.len(), 2);
        assert_eq!(plans[0].migration.version, 2); // newest first
        assert_eq!(plans[1].migration.version, 1);
        assert!(plans[0].sql.contains("DROP TABLE b"));
        assert!(plans[0].sql.contains("DELETE FROM _rok_migrations"));
        assert!(matches!(plans[0].direction, Direction::Down));
    }

    #[test]
    fn plan_down_steps_limits_count() {
        let mut m = Migrator::new();
        m.add(Migration::new(1, "a", "SQL;", Some("SQL;".to_string())))
            .unwrap();
        m.add(Migration::new(2, "b", "SQL;", Some("SQL;".to_string())))
            .unwrap();
        m.add(Migration::new(3, "c", "SQL;", Some("SQL;".to_string())))
            .unwrap();
        let plans = m.plan_down(&[1, 2, 3], Some(1)).unwrap();
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].migration.version, 3);
    }

    #[test]
    fn plan_down_errors_on_irreversible() {
        let mut m = Migrator::new();
        m.add(Migration::new(1, "a", "SQL;", None)).unwrap(); // no down_sql
        let err = m.plan_down(&[1], None);
        assert!(matches!(err, Err(MigrateError::Irreversible(1))));
    }
}
