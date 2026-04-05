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
//! let n: i64            = User::count(&pool).await?;
//! User::create(&pool, &[("name", "Alice".into())]).await?;
//! User::update_by_pk(&pool, 1i64, &[("name", "Bob".into())]).await?;
//! User::delete_by_pk(&pool, 1i64).await?;
//! ```

use rok_orm_core::{Model, QueryBuilder, SqlValue};
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

    /// Fetch rows matching a custom query.
    fn find_where(
        pool: &PgPool,
        builder: QueryBuilder<Self>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::fetch_all(pool, builder)
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

    /// Return the total number of rows in the model's table.
    fn count(
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<i64, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::count(pool, &Self::query())
    }

    /// Return the number of rows matching a custom query.
    fn count_where(
        pool: &PgPool,
        builder: &QueryBuilder<Self>,
    ) -> impl std::future::Future<Output = Result<i64, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::count(pool, builder)
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

    /// Update the row with the given primary key and return rows affected.
    fn update_by_pk(
        pool: &PgPool,
        id: impl Into<SqlValue> + Send,
        data: &[(&str, SqlValue)],
    ) -> impl std::future::Future<Output = Result<u64, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        let builder = Self::find(id);
        executor::update::<Self>(pool, builder, data)
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

    /// Delete rows matching a custom query and return rows affected.
    fn delete_where(
        pool: &PgPool,
        builder: QueryBuilder<Self>,
    ) -> impl std::future::Future<Output = Result<u64, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::delete(pool, builder)
    }

    /// Update rows matching a custom query and return rows affected.
    ///
    /// ```rust,ignore
    /// User::update_where(
    ///     &pool,
    ///     User::query().where_eq("role", "guest"),
    ///     &[("active", false.into())],
    /// ).await?;
    /// ```
    fn update_where(
        pool: &PgPool,
        builder: QueryBuilder<Self>,
        data: &[(&str, SqlValue)],
    ) -> impl std::future::Future<Output = Result<u64, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::update::<Self>(pool, builder, data)
    }

    /// Insert multiple rows in one statement and return rows affected.
    ///
    /// All rows must have the same column keys in the same order.
    ///
    /// ```rust,ignore
    /// User::bulk_create(&pool, &[
    ///     vec![("name", "Alice".into()), ("email", "a@a.com".into())],
    ///     vec![("name", "Bob".into()),   ("email", "b@b.com".into())],
    /// ]).await?;
    /// ```
    fn bulk_create(
        pool: &PgPool,
        rows: &[Vec<(&str, SqlValue)>],
    ) -> impl std::future::Future<Output = Result<u64, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::bulk_insert::<Self>(pool, Self::table_name(), rows)
    }

    /// Insert a row and return the full inserted row via `RETURNING *`.
    ///
    /// Ideal for getting back generated IDs or server-side default columns.
    ///
    /// ```rust,ignore
    /// let user: User = User::create_returning(
    ///     &pool,
    ///     &[("name", "Alice".into()), ("email", "a@a.com".into())],
    /// ).await?;
    /// println!("new id = {}", user.id);
    /// ```
    fn create_returning(
        pool: &PgPool,
        data: &[(&str, SqlValue)],
    ) -> impl std::future::Future<Output = Result<Self, sqlx::Error>> + Send
    where
        Self: Sized,
    {
        executor::insert_returning::<Self>(pool, Self::table_name(), data)
    }
}

/// Blanket implementation — every `Model + FromRow + Send + Unpin` gets
/// `PgModel` for free.
impl<T> PgModel for T where T: Model + for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin {}
