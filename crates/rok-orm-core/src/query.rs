//! [`QueryBuilder`] — fluent SQL builder.

use std::marker::PhantomData;

use crate::condition::{Condition, JoinOp, OrderDir, SqlValue};

/// A fluent builder that produces parameterized SQL statements.
///
/// Conditions added with `where_*` methods are joined with `AND`.
/// Use `or_where_*` variants to join with `OR`.
///
/// # Example
///
/// ```rust
/// use rok_orm_core::{QueryBuilder, SqlValue};
///
/// let (sql, params) = QueryBuilder::<()>::new("users")
///     .where_eq("active", true)
///     .or_where_eq("role", "admin")
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
    distinct: bool,
    conditions: Vec<(JoinOp, Condition)>,
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
            distinct: false,
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

    /// Emit `SELECT DISTINCT …`.
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    // ── AND conditions ────────────────────────────────────────────────────

    pub fn where_eq(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::And, Condition::Eq(col.into(), val.into()))
    }

    pub fn where_ne(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::And, Condition::Ne(col.into(), val.into()))
    }

    pub fn where_gt(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::And, Condition::Gt(col.into(), val.into()))
    }

    pub fn where_gte(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::And, Condition::Gte(col.into(), val.into()))
    }

    pub fn where_lt(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::And, Condition::Lt(col.into(), val.into()))
    }

    pub fn where_lte(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::And, Condition::Lte(col.into(), val.into()))
    }

    pub fn where_like(self, col: &str, pattern: &str) -> Self {
        self.push(JoinOp::And, Condition::Like(col.into(), pattern.into()))
    }

    pub fn where_not_like(self, col: &str, pattern: &str) -> Self {
        self.push(JoinOp::And, Condition::NotLike(col.into(), pattern.into()))
    }

    pub fn where_null(self, col: &str) -> Self {
        self.push(JoinOp::And, Condition::IsNull(col.into()))
    }

    pub fn where_not_null(self, col: &str) -> Self {
        self.push(JoinOp::And, Condition::IsNotNull(col.into()))
    }

    pub fn where_in(self, col: &str, vals: Vec<impl Into<SqlValue>>) -> Self {
        self.push(
            JoinOp::And,
            Condition::In(col.into(), vals.into_iter().map(Into::into).collect()),
        )
    }

    pub fn where_not_in(self, col: &str, vals: Vec<impl Into<SqlValue>>) -> Self {
        self.push(
            JoinOp::And,
            Condition::NotIn(col.into(), vals.into_iter().map(Into::into).collect()),
        )
    }

    pub fn where_between(self, col: &str, lo: impl Into<SqlValue>, hi: impl Into<SqlValue>) -> Self {
        self.push(
            JoinOp::And,
            Condition::Between(col.into(), lo.into(), hi.into()),
        )
    }

    pub fn where_not_between(
        self,
        col: &str,
        lo: impl Into<SqlValue>,
        hi: impl Into<SqlValue>,
    ) -> Self {
        self.push(
            JoinOp::And,
            Condition::NotBetween(col.into(), lo.into(), hi.into()),
        )
    }

    pub fn where_raw(self, sql: &str) -> Self {
        self.push(JoinOp::And, Condition::Raw(sql.into()))
    }

    // ── OR conditions ─────────────────────────────────────────────────────

    pub fn or_where_eq(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::Or, Condition::Eq(col.into(), val.into()))
    }

    pub fn or_where_ne(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::Or, Condition::Ne(col.into(), val.into()))
    }

    pub fn or_where_gt(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::Or, Condition::Gt(col.into(), val.into()))
    }

    pub fn or_where_gte(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::Or, Condition::Gte(col.into(), val.into()))
    }

    pub fn or_where_lt(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::Or, Condition::Lt(col.into(), val.into()))
    }

    pub fn or_where_lte(self, col: &str, val: impl Into<SqlValue>) -> Self {
        self.push(JoinOp::Or, Condition::Lte(col.into(), val.into()))
    }

    pub fn or_where_like(self, col: &str, pattern: &str) -> Self {
        self.push(JoinOp::Or, Condition::Like(col.into(), pattern.into()))
    }

    pub fn or_where_null(self, col: &str) -> Self {
        self.push(JoinOp::Or, Condition::IsNull(col.into()))
    }

    pub fn or_where_not_null(self, col: &str) -> Self {
        self.push(JoinOp::Or, Condition::IsNotNull(col.into()))
    }

    pub fn or_where_in(self, col: &str, vals: Vec<impl Into<SqlValue>>) -> Self {
        self.push(
            JoinOp::Or,
            Condition::In(col.into(), vals.into_iter().map(Into::into).collect()),
        )
    }

    pub fn or_where_between(
        self,
        col: &str,
        lo: impl Into<SqlValue>,
        hi: impl Into<SqlValue>,
    ) -> Self {
        self.push(
            JoinOp::Or,
            Condition::Between(col.into(), lo.into(), hi.into()),
        )
    }

    pub fn or_where_raw(self, sql: &str) -> Self {
        self.push(JoinOp::Or, Condition::Raw(sql.into()))
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

        let distinct_kw = if self.distinct { "DISTINCT " } else { "" };
        let mut sql = format!("SELECT {distinct_kw}{cols} FROM {}", self.table);
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

    /// Build an `UPDATE … SET … WHERE …` statement using this builder's
    /// table name and WHERE conditions.
    pub fn to_update_sql(&self, data: &[(&str, SqlValue)]) -> (String, Vec<SqlValue>) {
        let mut params: Vec<SqlValue> = Vec::new();
        let set_clauses: Vec<String> = data
            .iter()
            .enumerate()
            .map(|(i, (col, val))| {
                params.push(val.clone());
                format!("{col} = ${}", i + 1)
            })
            .collect();

        let mut sql = format!("UPDATE {} SET {}", self.table, set_clauses.join(", "));
        sql.push_str(&self.build_where(&mut params));
        (sql, params)
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

    /// Build an `UPDATE … SET … WHERE …` statement from explicit conditions.
    ///
    /// Prefer [`to_update_sql`] when you already have a `QueryBuilder`.
    pub fn update_sql(
        table: &str,
        data: &[(&str, SqlValue)],
        conditions: &[(JoinOp, Condition)],
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
            let where_frag = build_where_from(conditions, &mut params);
            sql.push_str(&where_frag);
        }

        (sql, params)
    }

    // ── internals ─────────────────────────────────────────────────────────

    fn push(mut self, op: JoinOp, cond: Condition) -> Self {
        self.conditions.push((op, cond));
        self
    }

    fn build_where(&self, params: &mut Vec<SqlValue>) -> String {
        build_where_from(&self.conditions, params)
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

    /// Expose the raw conditions (useful for callers that need to inspect them).
    pub fn conditions(&self) -> &[(JoinOp, Condition)] {
        &self.conditions
    }
}

fn build_where_from(conditions: &[(JoinOp, Condition)], params: &mut Vec<SqlValue>) -> String {
    if conditions.is_empty() {
        return String::new();
    }
    let mut out = " WHERE ".to_string();
    for (idx, (op, cond)) in conditions.iter().enumerate() {
        let (frag, ps) = cond.to_param_sql(params.len() + 1);
        params.extend(ps);
        if idx > 0 {
            out.push(' ');
            out.push_str(&op.to_string());
            out.push(' ');
        }
        out.push_str(&frag);
    }
    out
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
    fn distinct_select() {
        let (sql, _) = QueryBuilder::<()>::new("users").distinct().to_sql();
        assert!(sql.starts_with("SELECT DISTINCT * FROM users"));
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
    fn or_conditions() {
        let (sql, params) = QueryBuilder::<()>::new("users")
            .where_eq("role", "admin")
            .or_where_eq("role", "moderator")
            .to_sql();
        assert!(sql.contains("WHERE role = $1 OR role = $2"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn where_between() {
        let (sql, params) = QueryBuilder::<()>::new("orders")
            .where_between("amount", 10i64, 100i64)
            .to_sql();
        assert!(sql.contains("amount BETWEEN $1 AND $2"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn where_not_in() {
        let (sql, params) = QueryBuilder::<()>::new("users")
            .where_not_in("status", vec!["banned", "deleted"])
            .to_sql();
        assert!(sql.contains("status NOT IN ($1, $2)"));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn where_not_like() {
        let (sql, _) = QueryBuilder::<()>::new("users")
            .where_not_like("email", "%@spam.com")
            .to_sql();
        assert!(sql.contains("email NOT LIKE $1"));
    }

    #[test]
    fn to_update_sql() {
        let (sql, params) = QueryBuilder::<()>::new("users")
            .where_eq("id", 1i64)
            .to_update_sql(&[("name", "Bob".into()), ("active", true.into())]);
        assert!(sql.starts_with("UPDATE users SET name = $1, active = $2"));
        assert!(sql.contains("WHERE id = $3"));
        assert_eq!(params.len(), 3);
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

    #[test]
    fn option_value_null() {
        let val: SqlValue = Option::<i64>::None.into();
        assert_eq!(val, SqlValue::Null);
    }

    #[test]
    fn option_value_some() {
        let val: SqlValue = Some(42i64).into();
        assert_eq!(val, SqlValue::Integer(42));
    }
}
