//! [`Migration`] — a single versioned database migration.

/// A single forward (and optionally reversible) database migration.
#[derive(Debug, Clone)]
pub struct Migration {
    /// Monotonically increasing version number.
    pub version: u64,
    /// Human-readable name (e.g. `"create_users"`).
    pub name: String,
    /// SQL to apply (run on `migrate up`).
    pub up_sql: String,
    /// SQL to revert (run on `migrate down`).  `None` for irreversible migrations.
    pub down_sql: Option<String>,
}

impl Migration {
    pub fn new(
        version: u64,
        name: impl Into<String>,
        up_sql: impl Into<String>,
        down_sql: impl Into<Option<String>>,
    ) -> Self {
        Self {
            version,
            name: name.into(),
            up_sql: up_sql.into(),
            down_sql: down_sql.into(),
        }
    }

    /// Return `true` if this migration can be rolled back.
    pub fn is_reversible(&self) -> bool {
        self.down_sql.is_some()
    }
}

impl std::fmt::Display for Migration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}_{}", self.version, self.name)
    }
}
