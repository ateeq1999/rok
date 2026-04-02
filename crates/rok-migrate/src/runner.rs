//! Async PostgreSQL runner — executes [`MigrationPlan`]s against a live pool.
//!
//! Requires the `postgres` feature:
//!
//! ```toml
//! rok-migrate = { version = "0.1", features = ["postgres"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use rok_migrate::{loader::load_from_dir, Migrator, runner};
//!
//! let pool = sqlx::PgPool::connect(&database_url).await?;
//! runner::ensure_history_table(&pool).await?;
//!
//! let mut migrator = Migrator::new();
//! for m in load_from_dir("migrations")? {
//!     migrator.add(m)?;
//! }
//!
//! let applied = runner::applied_versions(&pool).await?;
//! let ran = runner::run_up(&pool, &migrator, &applied).await?;
//! println!("applied {} migration(s)", ran.len());
//! ```

use sqlx::PgPool;

use crate::error::MigrateError;
use crate::migrator::Migrator;

// ── helpers ───────────────────────────────────────────────────────────────────

fn to_db_err(e: sqlx::Error) -> MigrateError {
    MigrateError::Database(e.to_string())
}

// ── public API ────────────────────────────────────────────────────────────────

/// Create the `_rok_migrations` history table if it does not already exist.
pub async fn ensure_history_table(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::query(Migrator::history_table_sql())
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(to_db_err)
}

/// Return the list of migration versions already recorded in `_rok_migrations`.
pub async fn applied_versions(pool: &PgPool) -> Result<Vec<u64>, MigrateError> {
    let rows: Vec<(i64,)> = sqlx::query_as("SELECT version FROM _rok_migrations ORDER BY version")
        .fetch_all(pool)
        .await
        .map_err(to_db_err)?;
    Ok(rows.into_iter().map(|(v,)| v as u64).collect())
}

/// Apply all pending migrations in version order.
///
/// Each migration runs inside its own transaction so a failure rolls back
/// only that migration. Returns the list of versions successfully applied.
pub async fn run_up(
    pool: &PgPool,
    migrator: &Migrator,
    applied: &[u64],
) -> Result<Vec<u64>, MigrateError> {
    let plans = migrator.plan_up(applied);
    let mut ran = Vec::with_capacity(plans.len());

    for plan in &plans {
        let mut tx = pool.begin().await.map_err(to_db_err)?;
        sqlx::query(&plan.sql)
            .execute(&mut *tx)
            .await
            .map_err(to_db_err)?;
        tx.commit().await.map_err(to_db_err)?;
        ran.push(plan.migration.version);
    }

    Ok(ran)
}

/// Roll back applied migrations in reverse version order.
///
/// `steps` caps the number of migrations to revert; `None` reverts all applied
/// migrations tracked in `applied`. Each rollback runs inside its own
/// transaction. Returns the list of versions successfully reverted.
///
/// # Errors
///
/// Propagates [`MigrateError::Irreversible`] if any targeted migration has no
/// `down_sql`.
pub async fn run_down(
    pool: &PgPool,
    migrator: &Migrator,
    applied: &[u64],
    steps: Option<usize>,
) -> Result<Vec<u64>, MigrateError> {
    let plans = migrator.plan_down(applied, steps)?;
    let mut reverted = Vec::with_capacity(plans.len());

    for plan in &plans {
        let mut tx = pool.begin().await.map_err(to_db_err)?;
        sqlx::query(&plan.sql)
            .execute(&mut *tx)
            .await
            .map_err(to_db_err)?;
        tx.commit().await.map_err(to_db_err)?;
        reverted.push(plan.migration.version);
    }

    Ok(reverted)
}
