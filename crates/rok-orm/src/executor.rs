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
//! use rok_orm::{Model, executor::pg};
//!
//! #[derive(Model, sqlx::FromRow)]
//! pub struct User { pub id: i64, pub name: String }
//!
//! let pool = sqlx::PgPool::connect(&database_url).await?;
//!
//! let users: Vec<User> = pg::fetch_all(&pool, User::query().where_eq("active", true)).await?;
//! let count: i64       = pg::count(&pool, &User::query()).await?;
//! pg::delete(&pool, User::find(42i64)).await?;
//! ```

use rok_orm_core::{Model, QueryBuilder, SqlValue};
use sqlx::{postgres::PgRow, PgPool, Row};

// ── internal binding helpers ──────────────────────────────────────────────────

fn bind_query<'q>(
    sql: &'q str,
    params: Vec<SqlValue>,
) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    let mut q = sqlx::query(sql);
    for param in params {
        q = match param {
            SqlValue::Text(s) => q.bind(s),
            SqlValue::Integer(n) => q.bind(n),
            SqlValue::Float(f) => q.bind(f),
            SqlValue::Bool(b) => q.bind(b),
            SqlValue::Null => q.bind(Option::<String>::None),
        };
    }
    q
}

fn bind_query_as<'q, T>(
    sql: &'q str,
    params: Vec<SqlValue>,
) -> sqlx::query::QueryAs<'q, sqlx::Postgres, T, sqlx::postgres::PgArguments>
where
    T: for<'r> sqlx::FromRow<'r, PgRow>,
{
    let mut q = sqlx::query_as::<_, T>(sql);
    for param in params {
        q = match param {
            SqlValue::Text(s) => q.bind(s),
            SqlValue::Integer(n) => q.bind(n),
            SqlValue::Float(f) => q.bind(f),
            SqlValue::Bool(b) => q.bind(b),
            SqlValue::Null => q.bind(Option::<String>::None),
        };
    }
    q
}

// ── public API ────────────────────────────────────────────────────────────────

/// Fetch all rows matching the query.
pub async fn fetch_all<T>(pool: &PgPool, builder: QueryBuilder<T>) -> Result<Vec<T>, sqlx::Error>
where
    T: Model + for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    let (sql, params) = builder.to_sql();
    bind_query_as::<T>(&sql, params).fetch_all(pool).await
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
    bind_query_as::<T>(&sql, params).fetch_optional(pool).await
}

/// Return the row count matching the query's WHERE clause.
pub async fn count<T>(pool: &PgPool, builder: &QueryBuilder<T>) -> Result<i64, sqlx::Error> {
    let (sql, params) = builder.to_count_sql();
    let row = bind_query(&sql, params).fetch_one(pool).await?;
    row.try_get::<i64, _>(0)
}

/// Execute a raw SQL string with positional parameters and return rows affected.
pub async fn execute_raw(
    pool: &PgPool,
    sql: &str,
    params: Vec<SqlValue>,
) -> Result<u64, sqlx::Error> {
    let result = bind_query(sql, params).execute(pool).await?;
    Ok(result.rows_affected())
}

/// Insert a row using the column-value pairs and return rows affected.
pub async fn insert<T>(
    pool: &PgPool,
    table: &str,
    data: &[(&str, SqlValue)],
) -> Result<u64, sqlx::Error> {
    let (sql, params) = QueryBuilder::<T>::insert_sql(table, data);
    execute_raw(pool, &sql, params).await
}

/// Delete rows matching the builder's conditions and return rows affected.
pub async fn delete<T>(pool: &PgPool, builder: QueryBuilder<T>) -> Result<u64, sqlx::Error> {
    let (sql, params) = builder.to_delete_sql();
    execute_raw(pool, &sql, params).await
}
