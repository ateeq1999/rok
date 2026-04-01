//! rok-migrate — SQL migration runner for the rok ecosystem.
//!
//! ```rust
//! use rok_migrate::{Migration, Migrator};
//!
//! let mut m = Migrator::new();
//! m.add(Migration::new(1, "create_users", "CREATE TABLE users (id INT);", Some("DROP TABLE users;".to_string()))).unwrap();
//! m.add(Migration::new(2, "add_email",    "ALTER TABLE users ADD email TEXT;", None::<String>)).unwrap();
//!
//! let pending = m.pending(&[]);          // nothing applied yet
//! assert_eq!(pending.len(), 2);
//!
//! let pending = m.pending(&[1]);         // version 1 already applied
//! assert_eq!(pending.len(), 1);
//! assert_eq!(pending[0].version, 2);
//! ```

pub mod error;
pub mod loader;
pub mod migration;
pub mod migrator;

pub use error::MigrateError;
pub use loader::load_from_dir;
pub use migration::Migration;
pub use migrator::Migrator;
