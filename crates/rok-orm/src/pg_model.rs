//! [`PgModel`] — ergonomic async CRUD methods for any [`Model`] + [`sqlx::FromRow`] type.
//!
//! All methods are provided as defaults; no manual implementation is required.
//!
//! # Example
//!
//! ```rust,ignore
//! use rok_orm::{Model, pg_model::PgModel, SqlValue};
//!
//! #[derive(Model, sqlx::FromRow)]
//! pub struct User {
//!     pub id: i64,
//!     pub name: String,
//! }
//!
//! let pool = sqlx::PgPool::connect(&url).await?;
//!
//! let all: Vec<User>    = User::all(&pool).await?;
//! let one: Option<User> = User::find_by_pk(&pool, 1i64).await?;
//! User::create(&pool, &[("name", "Alice".into())]).await?;
//! User::delete_by_pk(&pool, 1i64).await?;
//! ```

use rok_orm_core::{Model, SqlValue};
use sqlx::{postgres::PgRow, PgPool};

use crate::executor;

/// Blanket async CRUD extension for any type that implements [`Model`] and
/// [`sqlx::FromRow`].
///
/// Implemented automatically for every such type — no `impl` block needed.
pub trait PgModel: Model + for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin {
    /// Fetch every row from the model's table.
    fn all(
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::fetch_all(pool, Self::query())
    }

    /// Fetch a single row by primary key.  Returns `None` if not found.
    fn find_by_pk(
        pool: &PgPool,
        id: impl Into<SqlValue> + Send,
    ) -> impl std::future::Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::fetch_optional(pool, Self::find(id))
    }

    /// Insert a row using the given column-value pairs and return rows affected.
    fn create(
        pool: &PgPool,
        data: &[(&str, SqlValue)],
    ) -> impl std::future::Future<Output = Result<u64, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::insert::<Self>(pool, Self::table_name(), data)
    }

    /// Delete the row with the given primary key and return rows affected.
    fn delete_by_pk(
        pool: &PgPool,
        id: impl Into<SqlValue> + Send,
    ) -> impl std::future::Future<Output = Result<u64, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::delete(pool, Self::find(id))
    }
}

/// Blanket implementation — every `Model + FromRow + Send + Unpin` gets
/// `PgModel` for free.
impl<T> PgModel for T where T: Model + for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin {}
