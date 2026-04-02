//! [`Model`] trait — implemented automatically by `#[derive(Model)]`.

use crate::query::QueryBuilder;

/// The core ORM trait.  Implemented via `#[derive(Model)]` from `rok-orm`.
///
/// ```rust,ignore
/// use rok_orm::Model;
///
/// #[derive(Model)]
/// pub struct Post {
///     pub id: i64,
///     pub title: String,
///     pub body: String,
/// }
///
/// let q = Post::query()
///     .where_eq("title", "Hello")
///     .order_by_desc("id")
///     .limit(10);
///
/// let (sql, params) = q.to_sql();
/// assert!(sql.starts_with("SELECT * FROM posts"));
/// ```
pub trait Model: Sized {
    /// SQL table name (e.g. `"users"`).
    fn table_name() -> &'static str;

    /// Primary-key column.  Defaults to `"id"`.
    fn primary_key() -> &'static str {
        "id"
    }

    /// All column names in declaration order.
    fn columns() -> &'static [&'static str];

    /// Start a new [`QueryBuilder`] scoped to this model.
    fn query() -> QueryBuilder<Self> {
        QueryBuilder::new(Self::table_name())
    }

    /// Build a `SELECT … WHERE <pk> = $1` query.
    fn find(id: impl Into<crate::condition::SqlValue>) -> QueryBuilder<Self> {
        Self::query().where_eq(Self::primary_key(), id)
    }
}
