//! [`QueryBuilder`] — fluent SQL builder.

use std::marker::PhantomData;

use crate::condition::{Condition, OrderDir, SqlValue};

/// A fluent builder that produces parameterized SQL SELECT statements.
///
/// # Example
///
/// ```rust
/// use rok_orm_core::{QueryBuilder, SqlValue};
///
/// let (sql, params) = QueryBuilder::<()>::new("users")
///     .where_eq("active", true)
///     .where_like("email", "%@example.com")
///     .order_by_desc("created_at")
///     .limit(20)
///     .offset(40)
///     .to_sql();
///
/// assert!(sql.contains("WHERE"));
/// assert!(sql.contains("ORDER BY created_at DESC"));
/// assert!(sql.contains("LIMIT 20"));
/// assert!(sql.contains("OFFSET 40"));
/// assert_eq!(params.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct QueryBuilder<T> {
    table: String,
    select_cols: Option<Vec<String>>,
    conditions: Vec<Condition>,
    order: Vec<(String, OrderDir)>,
    limit_val: Option<usize>,
    offset_val: Option<usize>,
    _marker: PhantomData<T>,
}

impl<T> QueryBuilder<T> {
    pub fn new(table: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            select_cols: None,
            conditions: Vec::new(),
            order: Vec::new(),
            limit_val: None,
            offset_val: None,
            _marker: PhantomData,
        }
    }

    // ── column selection ──────────────────────────────────────────────────

    pub fn select(mut self, cols: &[&str]) -> Self {
        self.select_cols = Some(cols.iter().map(|s| s.to_string()).collect());
        self
    }

    // ── where conditions ──────────────────────────────────────────────────

    pub fn where_eq(mut self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.conditions.push(Condition::Eq(col.into(), val.into()));
        self
    }

    pub fn where_ne(mut self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.conditions.push(Condition::Ne(col.into(), val.into()));
        self
    }

    pub fn where_gt(mut self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.conditions.push(Condition::Gt(col.into(), val.into()));
        self
    }

    pub fn where_gte(mut self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.conditions.push(Condition::Gte(col.into(), val.into()));
        self
    }

    pub fn where_lt(mut self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.conditions.push(Condition::Lt(col.into(), val.into()));
        self
    }

    pub fn where_lte(mut self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.conditions.push(Condition::Lte(col.into(), val.into()));
        self
    }

    pub fn where_like(mut self, col: &str, pattern: &str) -> Self {
        self.conditions
            .push(Condition::Like(col.into(), pattern.into()));
        self
    }

    pub fn where_null(mut self, col: &str) -> Self {
        self.conditions.push(Condition::IsNull(col.into()));
        self
    }

    pub fn where_not_null(mut self, col: &str) -> Self {
        self.conditions.push(Condition::IsNotNull(col.into()));
        self
    }

    pub fn where_in(mut self, col: &str, vals: Vec<impl Into<SqlValue>>) -> Self {
        self.conditions.push(Condition::In(
            col.into(),
            vals.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn where_raw(mut self, sql: &str) -> Self {
        self.conditions.push(Condition::Raw(sql.into()));
        self
    }

    // ── ordering ──────────────────────────────────────────────────────────

    pub fn order_by(mut self, col: &str) -> Self {
        self.order.push((col.into(), OrderDir::Asc));
        self
    }

    pub fn order_by_desc(mut self, col: &str) -> Self {
        self.order.push((col.into(), OrderDir::Desc));
        self
    }

    // ── pagination ────────────────────────────────────────────────────────

    pub fn limit(mut self, n: usize) -> Self {
        self.limit_val = Some(n);
        self
    }

    pub fn offset(mut self, n: usize) -> Self {
        self.offset_val = Some(n);
        self
    }

    // ── SQL generation ────────────────────────────────────────────────────

    /// Build a parameterized `SELECT` statement.
    ///
    /// Returns `(sql, params)` — params are ordered to match `$1`, `$2`, …
    pub fn to_sql(&self) -> (String, Vec<SqlValue>) {
        let cols = self
            .select_cols
            .as_ref()
            .map(|c| c.join(", "))
            .unwrap_or_else(|| "*".into());

        let mut sql = format!("SELECT {cols} FROM {}", self.table);
        let mut params: Vec<SqlValue> = Vec::new();

        sql.push_str(&self.build_where(&mut params));
        sql.push_str(&self.build_order());

        if let Some(n) = self.limit_val {
            sql.push_str(&format!(" LIMIT {n}"));
        }
        if let Some(n) = self.offset_val {
            sql.push_str(&format!(" OFFSET {n}"));
        }

        (sql, params)
    }

    /// Build a `SELECT COUNT(*) FROM …` statement.
    pub fn to_count_sql(&self) -> (String, Vec<SqlValue>) {
        let mut params: Vec<SqlValue> = Vec::new();
        let where_clause = self.build_where(&mut params);
        (
            format!("SELECT COUNT(*) FROM {}{}", self.table, where_clause),
            params,
        )
    }

    /// Build a `DELETE FROM … WHERE …` statement.
    pub fn to_delete_sql(&self) -> (String, Vec<SqlValue>) {
        let mut params: Vec<SqlValue> = Vec::new();
        let where_clause = self.build_where(&mut params);
        (
            format!("DELETE FROM {}{}", self.table, where_clause),
            params,
        )
    }

    // ── static helpers ────────────────────────────────────────────────────

    /// Build an `INSERT INTO` statement from key-value pairs.
    pub fn insert_sql(table: &str, data: &[(&str, SqlValue)]) -> (String, Vec<SqlValue>) {
        let cols: Vec<&str> = data.iter().map(|(c, _)| *c).collect();
        let placeholders: Vec<String> = (1..=data.len()).map(|i| format!("${i}")).collect();
        let params: Vec<SqlValue> = data.iter().map(|(_, v)| v.clone()).collect();
        (
            format!(
                "INSERT INTO {table} ({}) VALUES ({})",
                cols.join(", "),
                placeholders.join(", ")
            ),
            params,
        )
    }

    /// Build an `UPDATE … SET … WHERE …` statement.
    pub fn update_sql(
        table: &str,
        data: &[(&str, SqlValue)],
        conditions: &[Condition],
    ) -> (String, Vec<SqlValue>) {
        let mut params: Vec<SqlValue> = Vec::new();
        let set_clauses: Vec<String> = data
            .iter()
            .enumerate()
            .map(|(i, (col, val))| {
                params.push(val.clone());
                format!("{col} = ${}", i + 1)
            })
            .collect();

        let mut sql = format!("UPDATE {table} SET {}", set_clauses.join(", "));

        if !conditions.is_empty() {
            let where_parts: Vec<String> = conditions
                .iter()
                .map(|c| {
                    let (frag, ps) = c.to_param_sql(params.len() + 1);
                    params.extend(ps);
                    frag
                })
                .collect();
            sql.push_str(&format!(" WHERE {}", where_parts.join(" AND ")));
        }

        (sql, params)
    }

    // ── internals ─────────────────────────────────────────────────────────

    fn build_where(&self, params: &mut Vec<SqlValue>) -> String {
        if self.conditions.is_empty() {
            return String::new();
        }
        let parts: Vec<String> = self
            .conditions
            .iter()
            .map(|c| {
                let (frag, ps) = c.to_param_sql(params.len() + 1);
                params.extend(ps);
                frag
            })
            .collect();
        format!(" WHERE {}", parts.join(" AND "))
    }

    fn build_order(&self) -> String {
        if self.order.is_empty() {
            return String::new();
        }
        let parts: Vec<String> = self
            .order
            .iter()
            .map(|(col, dir)| format!("{col} {dir}"))
            .collect();
        format!(" ORDER BY {}", parts.join(", "))
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_select() {
        let (sql, params) = QueryBuilder::<()>::new("users").to_sql();
        assert_eq!(sql, "SELECT * FROM users");
        assert!(params.is_empty());
    }

    #[test]
    fn where_eq_generates_param() {
        let (sql, params) = QueryBuilder::<()>::new("users")
            .where_eq("id", 42i64)
            .to_sql();
        assert!(sql.contains("WHERE id = $1"));
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], SqlValue::Integer(42));
    }

    #[test]
    fn multiple_conditions() {
        let (sql, params) = QueryBuilder::<()>::new("posts")
            .where_eq("active", true)
            .where_like("title", "%rust%")
            .to_sql();
        assert!(sql.contains("WHERE active = $1 AND title LIKE $2"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn order_limit_offset() {
        let (sql, _) = QueryBuilder::<()>::new("users")
            .order_by_desc("created_at")
            .order_by("name")
            .limit(10)
            .offset(20)
            .to_sql();
        assert!(sql.contains("ORDER BY created_at DESC, name ASC"));
        assert!(sql.contains("LIMIT 10"));
        assert!(sql.contains("OFFSET 20"));
    }

    #[test]
    fn count_sql() {
        let (sql, _) = QueryBuilder::<()>::new("users")
            .where_eq("active", true)
            .to_count_sql();
        assert!(sql.starts_with("SELECT COUNT(*) FROM users"));
    }

    #[test]
    fn delete_sql() {
        let (sql, params) = QueryBuilder::<()>::new("sessions")
            .where_eq("user_id", 5i64)
            .to_delete_sql();
        assert!(sql.contains("DELETE FROM sessions WHERE user_id = $1"));
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn insert_sql() {
        let (sql, params) = QueryBuilder::<()>::insert_sql(
            "users",
            &[("name", "Alice".into()), ("email", "a@a.com".into())],
        );
        assert!(sql.contains("INSERT INTO users (name, email) VALUES ($1, $2)"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn where_in() {
        let (sql, params) = QueryBuilder::<()>::new("users")
            .where_in("id", vec![1i64, 2, 3])
            .to_sql();
        assert!(sql.contains("id IN ($1, $2, $3)"));
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn select_specific_columns() {
        let (sql, _) = QueryBuilder::<()>::new("users")
            .select(&["id", "email"])
            .to_sql();
        assert!(sql.starts_with("SELECT id, email FROM users"));
    }
}
