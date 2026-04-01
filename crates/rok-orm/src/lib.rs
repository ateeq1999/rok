//! rok-orm — Eloquent-inspired ORM for the rok ecosystem.
//!
//! # Quick start
//!
//! ```rust
//! use rok_orm::Model;
//!
//! #[derive(Model)]
//! pub struct User {
//!     pub id: i64,
//!     pub name: String,
//!     pub email: String,
//! }
//!
//! // Generated table name and columns
//! assert_eq!(User::table_name(), "users");
//! assert_eq!(User::columns(), &["id", "name", "email"]);
//!
//! // Build a query
//! let (sql, params) = User::query()
//!     .where_eq("active", true)
//!     .order_by_desc("created_at")
//!     .limit(10)
//!     .to_sql();
//!
//! assert!(sql.contains("FROM users"));
//! assert!(sql.contains("LIMIT 10"));
//! ```

// Re-export core types
pub use rok_orm_core::{Condition, Model, OrderDir, QueryBuilder, SqlValue};

// Re-export the derive macro
pub use rok_orm_macros::Model as Model_derive;

// Convenience re-export of the derive macro under the canonical name so users
// only need `use rok_orm::Model;` in combination with `#[derive(Model)]`.
pub use rok_orm_macros::Model;
