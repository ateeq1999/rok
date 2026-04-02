use rok_orm::Model;

// ── basic derive ──────────────────────────────────────────────────────────────

#[derive(Model)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Model)]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// snake_case + "s"
#[derive(Model)]
pub struct OrderItem {
    pub id: i64,
    pub quantity: i32,
}

// ── table names ───────────────────────────────────────────────────────────────

#[test]
fn table_name_simple() {
    assert_eq!(User::table_name(), "users");
}

#[test]
fn table_name_multi_word() {
    assert_eq!(BlogPost::table_name(), "blog_posts");
    assert_eq!(OrderItem::table_name(), "order_items");
}

// ── columns ───────────────────────────────────────────────────────────────────

#[test]
fn columns_list() {
    assert_eq!(User::columns(), &["id", "name", "email"]);
    assert_eq!(BlogPost::columns(), &["id", "title", "body", "published"]);
}

// ── query builder through Model trait ────────────────────────────────────────

#[test]
fn query_select_all() {
    let (sql, params) = User::query().to_sql();
    assert_eq!(sql, "SELECT * FROM users");
    assert!(params.is_empty());
}

#[test]
fn query_where_eq() {
    let (sql, params) = User::query().where_eq("id", 1i64).to_sql();
    assert!(sql.contains("WHERE id = $1"));
    assert_eq!(params.len(), 1);
}

#[test]
fn query_find() {
    let (sql, params) = User::find(42i64).to_sql();
    assert!(sql.contains("WHERE id = $1"));
    assert_eq!(params[0], rok_orm::SqlValue::Integer(42));
}

#[test]
fn query_chaining() {
    let (sql, params) = BlogPost::query()
        .where_eq("published", true)
        .where_like("title", "%rust%")
        .order_by_desc("id")
        .limit(5)
        .offset(10)
        .to_sql();

    assert!(sql.contains("FROM blog_posts"));
    assert!(sql.contains("WHERE published = $1 AND title LIKE $2"));
    assert!(sql.contains("ORDER BY id DESC"));
    assert!(sql.contains("LIMIT 5"));
    assert!(sql.contains("OFFSET 10"));
    assert_eq!(params.len(), 2);
}

#[test]
fn count_sql() {
    let (sql, _) = User::query().where_not_null("email").to_count_sql();
    assert!(sql.starts_with("SELECT COUNT(*) FROM users"));
    assert!(sql.contains("email IS NOT NULL"));
}

#[test]
fn insert_sql() {
    use rok_orm::SqlValue;
    let (sql, params) = rok_orm::QueryBuilder::<User>::insert_sql(
        "users",
        &[
            ("name", "Alice".into()),
            ("email", "alice@example.com".into()),
        ],
    );
    assert!(sql.contains("INSERT INTO users (name, email) VALUES ($1, $2)"));
    assert_eq!(
        params,
        vec![
            SqlValue::Text("Alice".into()),
            SqlValue::Text("alice@example.com".into()),
        ]
    );
}
