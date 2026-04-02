use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MigrateError {
    #[error("duplicate migration version {0}")]
    DuplicateVersion(u64),

    #[error("invalid migration filename `{0}`: expected `{{version}}_{{name}}.sql`")]
    InvalidFilename(String),

    #[error("migration version {0} not found")]
    NotFound(u64),

    #[error("migration version {0} is irreversible (no down SQL)")]
    Irreversible(u64),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("database error: {0}")]
    Database(String),
}
