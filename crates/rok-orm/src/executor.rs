//! Async PostgreSQL executor — runs [`QueryBuilder`] output against a live pool.
//!
//! Requires the `postgres` feature:
//!
//! ```toml
//! rok-orm = { version = "0.1", features = ["postgres"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use rok_orm::{Model, executor};
//!
//! #[derive(Model, sqlx::FromRow)]
//! pub struct User { pub id: i64, pub name: String }
//!
//! let pool = sqlx::PgPool::connect(&database_url).await?;
//!
//! let users: Vec<User> = executor::fetch_all(&pool, User::query().where_eq("active", true)).await?;
//! let count: i64       = executor::count(&pool, &User::query()).await?;
//! executor::delete(&pool, User::find(42i64)).await?;
//! ```

use rok_orm_core::{sqlx_pg, Model, QueryBuilder};
use sqlx::postgres::PgRow;
use sqlx::PgPool;

/// Fetch all rows matching the query.
pub async fn fetch_all<T>(pool: &PgPool, builder: QueryBuilder<T>) -> Result<Vec<T>, sqlx::Error>
where
    T: Model + for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    let (sql, params) = builder.to_sql();
    sqlx_pg::fetch_all_as::<T>(pool, &sql, params).await
}

/// Fetch at most one row matching the query.  Returns `None` if no rows match.
pub async fn fetch_optional<T>(
    pool: &PgPool,
    builder: QueryBuilder<T>,
) -> Result<Option<T>, sqlx::Error>
where
    T: Model + for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    let (sql, params) = builder.to_sql();
    sqlx_pg::fetch_optional_as::<T>(pool, &sql, params).await
}

/// Return the row count matching the query's WHERE clause.
pub async fn count<T>(pool: &PgPool, builder: &QueryBuilder<T>) -> Result<i64, sqlx::Error> {
    let (sql, params) = builder.to_count_sql();
    let row = sqlx_pg::build_query(&sql, params).fetch_one(pool).await?;
    use sqlx::Row;
    row.try_get::<i64, _>(0)
}

/// Execute a raw SQL string with positional parameters and return rows affected.
pub async fn execute_raw(
    pool: &PgPool,
    sql: &str,
    params: Vec<rok_orm_core::SqlValue>,
) -> Result<u64, sqlx::Error> {
    sqlx_pg::execute(pool, sql, params).await
}

/// Insert a row using the column-value pairs and return rows affected.
pub async fn insert<T>(
    pool: &PgPool,
    table: &str,
    data: &[(&str, rok_orm_core::SqlValue)],
) -> Result<u64, sqlx::Error> {
    let (sql, params) = QueryBuilder::<T>::insert_sql(table, data);
    execute_raw(pool, &sql, params).await
}

/// Delete rows matching the builder's conditions and return rows affected.
pub async fn delete<T>(pool: &PgPool, builder: QueryBuilder<T>) -> Result<u64, sqlx::Error> {
    let (sql, params) = builder.to_delete_sql();
    execute_raw(pool, &sql, params).await
}
