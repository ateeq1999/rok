# rok-orm — Comprehensive Implementation Plan

> **rok-orm** is an Eloquent-inspired async ORM for Rust, built on top of `sqlx`, designed as a first-class citizen of the **rok** ecosystem. One JSON payload generates your entire model layer.

**Version Target**: v0.1.0  
**Depends on**: rok v0.10.0+, sqlx 0.8+, Tokio, Axum  
**Published as**: `rok-orm` + `rok-orm-macros` on crates.io

---

## Table of Contents

1. [Vision & Design Principles](#1-vision--design-principles)
2. [Crate Architecture](#2-crate-architecture)
3. [Core Trait System](#3-core-trait-system)
4. [QueryBuilder Engine](#4-querybuilder-engine)
5. [Derive Macro System](#5-derive-macro-system)
6. [Relations Layer](#6-relations-layer)
7. [Migration Engine](#7-migration-engine)
8. [Transaction System](#8-transaction-system)
9. [Soft Delete & Scopes](#9-soft-delete--scopes)
10. [Pagination](#10-pagination)
11. [Error Handling](#11-error-handling)
12. [Folder Structure](#12-folder-structure)
13. [Cargo.toml & Dependencies](#13-cargotoml--dependencies)
14. [Implementation Roadmap](#14-implementation-roadmap)
15. [rok Integration Layer](#15-rok-integration-layer)
16. [Testing Strategy](#16-testing-strategy)
17. [Full API Reference](#17-full-api-reference)

---

## 1. Vision & Design Principles

### The Goal

rok-orm eliminates the gap between a JSON schema description and a fully working, production-grade Rust data layer. An agent or developer describes their model in JSON — rok-orm generates, validates, and evolves it.

### Design Pillars

**Pillar 1 — rok-native first.** Every model, migration, and query is generatable from a single rok JSON payload. rok-orm is the execution target; the rok CLI is the driver.

**Pillar 2 — Eloquent ergonomics, Rust safety.** The API surface should feel as natural as Laravel's Eloquent — `User::find(id)`, `Post::where("published", true).paginate(20)` — but with full Rust type safety and zero runtime surprises.

**Pillar 3 — sqlx as the bedrock.** rok-orm never replaces sqlx. It wraps it. Every query compiles to a raw sqlx call. The full power of sqlx (prepared statements, `LISTEN/NOTIFY`, `COPY`, transactions) is always accessible underneath.

**Pillar 4 — Zero magic overhead.** The macro generates code you could have written by hand. No runtime reflection, no dynamic dispatch where avoidable. Generated code is readable, auditable, and overridable.

**Pillar 5 — Separation of concerns.** The ORM layer never touches HTTP. Domain models never import Axum. The boundary is enforced by crate structure.

---

## 2. Crate Architecture

### Workspace Layout

```
rok-orm/
├── Cargo.toml                  # workspace root
├── rok-orm-macros/             # proc-macro crate (MUST be separate)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # proc_macro entry points
│       ├── model_derive.rs     # #[derive(Model)] impl
│       ├── relation_derive.rs  # #[has_many], #[belongs_to] etc
│       └── attr_parser.rs      # darling-based attribute parsing
│
├── rok-orm-core/               # traits, types, QueryBuilder, runtime
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── model.rs            # Model trait definition
│       ├── query_builder.rs    # QueryBuilder<T>
│       ├── paginator.rs        # Paginator<T>
│       ├── relation.rs         # Relation traits + loaders
│       ├── scope.rs            # Global scopes
│       ├── transaction.rs      # Transaction wrapper
│       ├── migration.rs        # Migration engine
│       ├── executor.rs         # Unified Executor abstraction
│       ├── value.rs            # SqlValue enum
│       └── error.rs            # OrmError
│
└── rok-orm/                    # facade crate — public API
    ├── Cargo.toml
    └── src/
        └── lib.rs              # re-exports everything
```

### Dependency Graph

```
rok-orm (facade)
  └── rok-orm-core
        └── sqlx
  └── rok-orm-macros
        └── syn + quote + darling
```

The user depends on `rok-orm` only. The proc-macro crate is a build dependency pulled in automatically.

---

## 3. Core Trait System

### 3.1 The `Model` Trait

This is the heart of the entire ORM. Every generated struct implements it.

```rust
// rok-orm-core/src/model.rs

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{QueryBuilder, OrmError, SqlValue, Executor};

#[async_trait]
pub trait Model: Sized + Send + Sync + Unpin
    + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow>
{
    // ── Identity ─────────────────────────────────────────
    fn table_name() -> &'static str;
    fn primary_key() -> &'static str { "id" }
    fn columns() -> &'static [&'static str];

    // ── Query entrypoints ────────────────────────────────
    fn query() -> QueryBuilder<Self> {
        QueryBuilder::new(Self::table_name())
    }

    // ── Static finders ───────────────────────────────────
    async fn find(id: impl Into<SqlValue> + Send, pool: &PgPool)
        -> Result<Option<Self>, OrmError>;

    async fn find_or_fail(id: impl Into<SqlValue> + Send, pool: &PgPool)
        -> Result<Self, OrmError>
    {
        Self::find(id, pool).await?.ok_or(OrmError::NotFound {
            model: Self::table_name(),
        })
    }

    async fn all(pool: &PgPool) -> Result<Vec<Self>, OrmError> {
        Self::query().get(pool).await
    }

    // ── Persistence ──────────────────────────────────────
    async fn save(&mut self, pool: &PgPool) -> Result<(), OrmError>;
    async fn delete(self, pool: &PgPool) -> Result<(), OrmError>;

    // ── Aggregates ───────────────────────────────────────
    async fn count(pool: &PgPool) -> Result<i64, OrmError>;
    async fn exists(id: impl Into<SqlValue> + Send, pool: &PgPool)
        -> Result<bool, OrmError>;

    // ── Hooks (overridable) ──────────────────────────────
    async fn before_save(&mut self) -> Result<(), OrmError> { Ok(()) }
    async fn after_save(&mut self) -> Result<(), OrmError> { Ok(()) }
    async fn before_delete(&mut self) -> Result<(), OrmError> { Ok(()) }

    // ── Global scope (overridable) ───────────────────────
    fn apply_global_scope(qb: QueryBuilder<Self>) -> QueryBuilder<Self> { qb }
}
```

### 3.2 The `SoftDelete` Trait

Opt-in. When implemented, `delete()` sets `deleted_at` instead of issuing `DELETE`.

```rust
#[async_trait]
pub trait SoftDelete: Model {
    fn deleted_at_column() -> &'static str { "deleted_at" }

    async fn restore(&mut self, pool: &PgPool) -> Result<(), OrmError>;
    async fn force_delete(self, pool: &PgPool) -> Result<(), OrmError>;

    fn with_trashed() -> QueryBuilder<Self> {
        QueryBuilder::new_unscoped(Self::table_name())
    }
    fn only_trashed() -> QueryBuilder<Self> {
        QueryBuilder::new_unscoped(Self::table_name())
            .where_not_null(Self::deleted_at_column())
    }
}
```

### 3.3 The `Timestamps` Trait

```rust
pub trait Timestamps: Model {
    fn created_at_column() -> &'static str { "created_at" }
    fn updated_at_column() -> &'static str { "updated_at" }
}
```

---

## 4. QueryBuilder Engine

The `QueryBuilder<T>` is the fluent query composition layer. It accumulates clauses and compiles them into a parameterized SQL string only at execution time.

### 4.1 Internal Structure

```rust
// rok-orm-core/src/query_builder.rs

pub enum Direction { Asc, Desc }
pub enum WhereOp { Eq, Ne, Lt, Lte, Gt, Gte, Like, ILike, In, NotIn, IsNull, IsNotNull }

pub struct WhereClause {
    pub column: String,
    pub op: WhereOp,
    pub value: Option<SqlValue>,
    pub raw: Option<String>,         // for raw SQL fragments
    pub connector: Connector,        // AND / OR
}

pub struct JoinClause {
    pub kind: JoinKind,              // INNER / LEFT / RIGHT
    pub table: String,
    pub on: String,
}

pub struct QueryBuilder<T: Model> {
    table:        &'static str,
    wheres:       Vec<WhereClause>,
    joins:        Vec<JoinClause>,
    order_by:     Vec<(String, Direction)>,
    group_by:     Vec<String>,
    having:       Vec<WhereClause>,
    limit:        Option<u64>,
    offset:       Option<u64>,
    select_cols:  Option<Vec<String>>,  // None = SELECT *
    with_trashed: bool,                 // bypass soft-delete scope
    _phantom:     PhantomData<T>,
}
```

### 4.2 Builder API

```rust
impl<T: Model> QueryBuilder<T> {
    // ── Filters ──────────────────────────────────────────
    pub fn where_eq(self, col: &str, val: impl Into<SqlValue>) -> Self;
    pub fn where_ne(self, col: &str, val: impl Into<SqlValue>) -> Self;
    pub fn where_gt(self, col: &str, val: impl Into<SqlValue>) -> Self;
    pub fn where_gte(self, col: &str, val: impl Into<SqlValue>) -> Self;
    pub fn where_lt(self, col: &str, val: impl Into<SqlValue>) -> Self;
    pub fn where_lte(self, col: &str, val: impl Into<SqlValue>) -> Self;
    pub fn where_like(self, col: &str, pattern: &str) -> Self;
    pub fn where_ilike(self, col: &str, pattern: &str) -> Self;
    pub fn where_in(self, col: &str, vals: Vec<impl Into<SqlValue>>) -> Self;
    pub fn where_not_in(self, col: &str, vals: Vec<impl Into<SqlValue>>) -> Self;
    pub fn where_null(self, col: &str) -> Self;
    pub fn where_not_null(self, col: &str) -> Self;
    pub fn where_raw(self, sql: &str) -> Self;
    pub fn or_where(self, col: &str, op: WhereOp, val: impl Into<SqlValue>) -> Self;

    // ── Joins ────────────────────────────────────────────
    pub fn join(self, table: &str, on: &str) -> Self;
    pub fn left_join(self, table: &str, on: &str) -> Self;
    pub fn right_join(self, table: &str, on: &str) -> Self;

    // ── Ordering / Grouping ──────────────────────────────
    pub fn order_by(self, col: &str, dir: Direction) -> Self;
    pub fn order_by_asc(self, col: &str) -> Self;
    pub fn order_by_desc(self, col: &str) -> Self;
    pub fn group_by(self, col: &str) -> Self;
    pub fn having(self, col: &str, op: WhereOp, val: impl Into<SqlValue>) -> Self;

    // ── Limiting ─────────────────────────────────────────
    pub fn limit(self, n: u64) -> Self;
    pub fn offset(self, n: u64) -> Self;

    // ── Column selection ─────────────────────────────────
    pub fn select(self, cols: Vec<&str>) -> Self;

    // ── Eager loading ────────────────────────────────────
    pub fn with(self, relation: &str) -> Self;
    pub fn with_count(self, relation: &str) -> Self;

    // ── Pagination ───────────────────────────────────────
    pub fn paginate(self, per_page: u64) -> Paginator<T>;

    // ── Terminals ────────────────────────────────────────
    pub async fn get(self, pool: &PgPool) -> Result<Vec<T>, OrmError>;
    pub async fn first(self, pool: &PgPool) -> Result<Option<T>, OrmError>;
    pub async fn first_or_fail(self, pool: &PgPool) -> Result<T, OrmError>;
    pub async fn count(self, pool: &PgPool) -> Result<i64, OrmError>;
    pub async fn exists(self, pool: &PgPool) -> Result<bool, OrmError>;
    pub async fn pluck(self, col: &str, pool: &PgPool) -> Result<Vec<SqlValue>, OrmError>;

    // ── SQL inspection ───────────────────────────────────
    pub fn to_sql(&self) -> (String, Vec<SqlValue>);  // for debugging
}
```

### 4.3 SQL Compilation

The `to_sql()` method compiles the builder into a parameterized query:

```rust
pub fn to_sql(&self) -> (String, Vec<SqlValue>) {
    let mut sql = format!("SELECT {} FROM {}", self.select_clause(), self.table);
    let mut params: Vec<SqlValue> = vec![];
    let mut idx = 1usize;

    // JOINs
    for join in &self.joins {
        sql.push_str(&format!(" {} JOIN {} ON {}", join.kind, join.table, join.on));
    }

    // WHERE
    let where_clauses = self.build_where_clauses(&mut params, &mut idx);
    if !where_clauses.is_empty() {
        sql.push_str(&format!(" WHERE {}", where_clauses));
    }

    // GROUP BY / HAVING / ORDER BY / LIMIT / OFFSET
    // ... each appended as needed

    (sql, params)
}
```

All values are positional `$1`, `$2`, ... placeholders — never string interpolation. SQL injection is structurally impossible.

---

## 5. Derive Macro System

### 5.1 Usage

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[orm(table = "posts", soft_delete, timestamps)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: Option<String>,
    pub published: bool,
    #[orm(skip)]           // excluded from INSERT/UPDATE
    pub view_count: i64,
    #[orm(rename = "ts_created")]   // maps to different DB column
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
```

### 5.2 Supported Attributes

| Attribute | Level | Description |
|---|---|---|
| `#[orm(table = "...")]` | struct | Override table name (default: snake_case plural) |
| `#[orm(soft_delete)]` | struct | Generate soft-delete impl |
| `#[orm(timestamps)]` | struct | Auto-set `created_at` / `updated_at` on save |
| `#[orm(pk = "...")]` | struct | Override primary key column name |
| `#[orm(skip)]` | field | Exclude from INSERT/UPDATE |
| `#[orm(rename = "...")]` | field | Map to different column name |
| `#[orm(readonly)]` | field | Include in SELECT, exclude from INSERT/UPDATE |
| `#[orm(json)]` | field | Serialize/deserialize as JSONB |
| `#[has_many(Post, fk = "user_id")]` | struct | Declare has-many relation |
| `#[belongs_to(User, fk = "user_id")]` | struct | Declare belongs-to relation |
| `#[has_one(Profile, fk = "user_id")]` | struct | Declare has-one relation |

### 5.3 What the Macro Generates

Given the `Post` struct above, the macro emits:

```rust
// auto-generated by #[derive(Model)]

#[async_trait]
impl Model for Post {
    fn table_name() -> &'static str { "posts" }
    fn primary_key() -> &'static str { "id" }
    fn columns() -> &'static [&'static str] {
        &["id", "user_id", "title", "body", "published", "ts_created", "updated_at", "deleted_at"]
    }

    fn apply_global_scope(qb: QueryBuilder<Self>) -> QueryBuilder<Self> {
        qb.where_null("deleted_at")   // injected because soft_delete is set
    }

    fn query() -> QueryBuilder<Self> {
        Self::apply_global_scope(QueryBuilder::new("posts"))
    }

    async fn find(id: impl Into<SqlValue> + Send, pool: &PgPool)
        -> Result<Option<Self>, OrmError>
    {
        sqlx::query_as::<_, Self>(
            "SELECT id, user_id, title, body, published, ts_created, updated_at, deleted_at
             FROM posts WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id.into().as_uuid())
        .fetch_optional(pool)
        .await
        .map_err(OrmError::Database)
    }

    async fn save(&mut self, pool: &PgPool) -> Result<(), OrmError> {
        self.before_save().await?;
        self.updated_at = Utc::now();   // timestamps feature
        sqlx::query(
            "INSERT INTO posts (id, user_id, title, body, published, ts_created, updated_at, deleted_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET
               user_id = $2, title = $3, body = $4, published = $5,
               ts_created = $6, updated_at = $7, deleted_at = $8"
        )
        .bind(self.id)
        .bind(self.user_id)
        .bind(&self.title)
        .bind(&self.body)
        .bind(self.published)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(self.deleted_at)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(OrmError::Database)?;
        self.after_save().await
    }

    async fn delete(self, pool: &PgPool) -> Result<(), OrmError> {
        // soft_delete: UPDATE instead of DELETE
        sqlx::query("UPDATE posts SET deleted_at = NOW() WHERE id = $1")
            .bind(self.id)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(OrmError::Database)
    }

    async fn count(pool: &PgPool) -> Result<i64, OrmError> {
        sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE deleted_at IS NULL")
            .fetch_one(pool)
            .await
            .map_err(OrmError::Database)
    }

    async fn exists(id: impl Into<SqlValue> + Send, pool: &PgPool)
        -> Result<bool, OrmError>
    {
        let n: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM posts WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id.into().as_uuid())
        .fetch_one(pool)
        .await
        .map_err(OrmError::Database)?;
        Ok(n > 0)
    }
}
```

### 5.4 Macro Internals (proc-macro crate)

```
rok-orm-macros/src/
├── lib.rs
│     #[proc_macro_derive(Model, attributes(orm, has_many, belongs_to, has_one))]
│     pub fn derive_model(input: TokenStream) -> TokenStream
│
├── model_derive.rs
│     parse_struct_attrs()  →  ModelConfig { table, pk, soft_delete, timestamps }
│     parse_field_attrs()   →  Vec<FieldConfig { name, col, skip, readonly, json }>
│     generate_impl()       →  TokenStream (the full impl Model block)
│
├── relation_derive.rs
│     parse_relations()     →  Vec<RelationConfig>
│     generate_relation_methods() → TokenStream
│
└── attr_parser.rs
      uses `darling` crate for ergonomic attribute parsing
```

---

## 6. Relations Layer

### 6.1 Relation Types

```rust
// rok-orm-core/src/relation.rs

pub enum RelationKind { HasMany, HasOne, BelongsTo, ManyToMany }

pub struct RelationDef {
    pub kind: RelationKind,
    pub related_table: &'static str,
    pub foreign_key: &'static str,
    pub local_key: &'static str,
    pub pivot_table: Option<&'static str>,   // for ManyToMany
}
```

### 6.2 Generated Relation Methods

```rust
// For: #[has_many(Post, fk = "user_id")]
impl User {
    pub async fn posts(&self, pool: &PgPool) -> Result<Vec<Post>, OrmError> {
        Post::query()
            .where_eq("user_id", self.id)
            .get(pool)
            .await
    }

    pub async fn published_posts(&self, pool: &PgPool) -> Result<Vec<Post>, OrmError> {
        Post::query()
            .where_eq("user_id", self.id)
            .where_eq("published", true)
            .order_by_desc("created_at")
            .get(pool)
            .await
    }
}

// For: #[belongs_to(User, fk = "user_id")]
impl Post {
    pub async fn user(&self, pool: &PgPool) -> Result<Option<User>, OrmError> {
        User::find(self.user_id, pool).await
    }
}

// For: #[has_one(Profile, fk = "user_id")]
impl User {
    pub async fn profile(&self, pool: &PgPool) -> Result<Option<Profile>, OrmError> {
        Profile::query()
            .where_eq("user_id", self.id)
            .first(pool)
            .await
    }
}
```

### 6.3 Eager Loading — N+1 Prevention

The `with()` clause on `QueryBuilder` triggers a batch loader:

```rust
// This fires 2 queries, not N+1
let posts = Post::query()
    .where_eq("published", true)
    .with("user")           // collects all user_ids, fires one IN query
    .get(&pool)
    .await?;

// posts[i].user is populated (Option<User>)
```

Internal batch loader:

```rust
// rok-orm-core/src/relation.rs

pub struct EagerLoader<Parent, Child> {
    relation: RelationDef,
    _p: PhantomData<(Parent, Child)>,
}

impl<Parent: Model, Child: Model> EagerLoader<Parent, Child> {
    pub async fn load(
        parents: &mut Vec<Parent>,
        pool: &PgPool,
    ) -> Result<(), OrmError> {
        // 1. collect all FK values from parents
        let fk_values: Vec<SqlValue> = parents.iter()
            .map(|p| p.get_field(self.relation.local_key))
            .collect();

        // 2. fire one IN query
        let children = Child::query()
            .where_in(self.relation.foreign_key, fk_values)
            .get(pool)
            .await?;

        // 3. merge back by FK
        // ... group children by FK, assign to parent
        Ok(())
    }
}
```

---

## 7. Migration Engine

### 7.1 From rok JSON to Migration

rok-orm generates migrations from the same JSON schema used for model generation:

```json
{
  "name": "CreatePostsTable",
  "table": "posts",
  "columns": [
    { "name": "id",         "type": "Uuid",            "primary": true },
    { "name": "user_id",    "type": "Uuid",             "nullable": false, "references": { "table": "users", "column": "id" } },
    { "name": "title",      "type": "String",           "nullable": false, "max_length": 255 },
    { "name": "body",       "type": "Option<String>",   "nullable": true },
    { "name": "published",  "type": "bool",             "default": "false" },
    { "name": "created_at", "type": "DateTime<Utc>",    "default": "NOW()" },
    { "name": "updated_at", "type": "DateTime<Utc>",    "default": "NOW()" },
    { "name": "deleted_at", "type": "Option<DateTime<Utc>>", "nullable": true }
  ],
  "indexes": [
    { "name": "idx_posts_user_id", "columns": ["user_id"] },
    { "name": "idx_posts_published", "columns": ["published", "created_at"] }
  ]
}
```

### 7.2 Type Mapping

| Rust Type | PostgreSQL | MySQL | SQLite |
|---|---|---|---|
| `Uuid` | `UUID` | `CHAR(36)` | `TEXT` |
| `String` | `TEXT` | `TEXT` | `TEXT` |
| `String` (max_length) | `VARCHAR(n)` | `VARCHAR(n)` | `TEXT` |
| `i32` | `INTEGER` | `INT` | `INTEGER` |
| `i64` | `BIGINT` | `BIGINT` | `INTEGER` |
| `f64` | `FLOAT8` | `DOUBLE` | `REAL` |
| `bool` | `BOOLEAN` | `TINYINT(1)` | `INTEGER` |
| `DateTime<Utc>` | `TIMESTAMPTZ` | `DATETIME` | `TEXT` |
| `Option<T>` | `T NULL` | `T NULL` | `T NULL` |
| `serde_json::Value` | `JSONB` | `JSON` | `TEXT` |
| `Vec<String>` | `TEXT[]` | `JSON` | `TEXT` |

### 7.3 Generated Migration SQL

```sql
-- migrations/20240101_000000_create_posts_table.sql
-- Generated by rok-orm — DO NOT EDIT MANUALLY

CREATE TABLE IF NOT EXISTS posts (
    id          UUID        PRIMARY KEY,
    user_id     UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title       VARCHAR(255) NOT NULL,
    body        TEXT,
    published   BOOLEAN     NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ
);

CREATE INDEX idx_posts_user_id      ON posts(user_id);
CREATE INDEX idx_posts_published    ON posts(published, created_at);

-- Down migration (auto-generated)
-- DROP TABLE IF EXISTS posts;
```

### 7.4 Migration Runner

```rust
// rok-orm-core/src/migration.rs

pub struct MigrationRunner {
    pool: PgPool,
    migrations_dir: PathBuf,
}

impl MigrationRunner {
    pub async fn run_pending(&self) -> Result<Vec<String>, OrmError>;
    pub async fn rollback_last(&self) -> Result<String, OrmError>;
    pub async fn rollback_n(&self, n: usize) -> Result<Vec<String>, OrmError>;
    pub async fn status(&self) -> Result<Vec<MigrationStatus>, OrmError>;
    pub async fn fresh(&self) -> Result<(), OrmError>;   // drop all + re-run
    pub async fn seed(&self, seeder: impl Seeder) -> Result<(), OrmError>;
}
```

---

## 8. Transaction System

### 8.1 Unified `Executor` Abstraction

Both `&PgPool` and `&mut PgTransaction` implement `OrmExecutor`, so all model methods work transparently in either context:

```rust
// rok-orm-core/src/executor.rs

pub trait OrmExecutor: Send {
    // marker trait — sqlx::Executor bounds applied per method
}

impl OrmExecutor for &PgPool {}
impl OrmExecutor for &mut sqlx::Transaction<'_, sqlx::Postgres> {}
```

### 8.2 Usage

```rust
// Atomic — if any step fails, everything rolls back
let mut tx = pool.begin().await?;

user.save(&mut tx).await?;
post.save(&mut tx).await?;
notification.save(&mut tx).await?;

tx.commit().await?;
```

### 8.3 Convenience Wrapper

```rust
// rok-orm-core/src/transaction.rs

pub async fn transaction<F, T>(pool: &PgPool, f: F) -> Result<T, OrmError>
where
    F: for<'tx> FnOnce(&'tx mut sqlx::Transaction<'_, sqlx::Postgres>)
        -> BoxFuture<'tx, Result<T, OrmError>> + Send,
    T: Send,
{
    let mut tx = pool.begin().await.map_err(OrmError::Database)?;
    match f(&mut tx).await {
        Ok(result) => { tx.commit().await.map_err(OrmError::Database)?; Ok(result) }
        Err(e) => { let _ = tx.rollback().await; Err(e) }
    }
}

// Usage:
let result = transaction(&pool, |tx| Box::pin(async move {
    user.save(tx).await?;
    post.save(tx).await?;
    Ok(post.id)
})).await?;
```

---

## 9. Soft Delete & Scopes

### 9.1 Soft Delete Flow

When `#[orm(soft_delete)]` is set:

- `Model::query()` automatically appends `WHERE deleted_at IS NULL` (via `apply_global_scope`)
- `delete()` issues `UPDATE SET deleted_at = NOW()` not `DELETE FROM`
- `find()` includes `AND deleted_at IS NULL`
- `SoftDelete::restore()` sets `deleted_at = NULL`
- `SoftDelete::force_delete()` issues actual `DELETE FROM`
- `SoftDelete::with_trashed()` returns unscoped `QueryBuilder` that sees all rows
- `SoftDelete::only_trashed()` returns builder with `WHERE deleted_at IS NOT NULL`

### 9.2 Global Scopes

Beyond soft delete, users can define custom always-on filters:

```rust
impl Model for Post {
    fn apply_global_scope(qb: QueryBuilder<Self>) -> QueryBuilder<Self> {
        // automatically calls parent (soft_delete) scope first
        let qb = <Self as SoftDelete>::apply_soft_delete_scope(qb);
        // then user-defined scope
        qb.where_eq("tenant_id", current_tenant_id())
    }
}
```

---

## 10. Pagination

### 10.1 Offset Pagination

```rust
// rok-orm-core/src/paginator.rs

pub struct Paginator<T> {
    builder: QueryBuilder<T>,
    per_page: u64,
}

pub struct PaginatedResult<T> {
    pub data:         Vec<T>,
    pub total:        i64,
    pub per_page:     u64,
    pub current_page: u64,
    pub last_page:    u64,
    pub from:         u64,
    pub to:           u64,
    pub has_next:     bool,
    pub has_prev:     bool,
}

impl<T: Model> Paginator<T> {
    pub async fn fetch_page(
        self,
        page: u64,
        pool: &PgPool,
    ) -> Result<PaginatedResult<T>, OrmError> {
        let total = self.builder.clone().count(pool).await?;
        let offset = page * self.per_page;
        let data = self.builder
            .limit(self.per_page)
            .offset(offset)
            .get(pool)
            .await?;
        // compute metadata...
        Ok(PaginatedResult { data, total, per_page: self.per_page, current_page: page, ... })
    }
}
```

### 10.2 Cursor Pagination (for large datasets)

```rust
pub struct CursorPaginator<T> {
    builder: QueryBuilder<T>,
    per_page: u64,
    cursor_col: String,
}

pub struct CursorResult<T> {
    pub data:        Vec<T>,
    pub next_cursor: Option<String>,   // base64 encoded cursor value
    pub has_more:    bool,
}

impl<T: Model> CursorPaginator<T> {
    pub async fn fetch_after(
        self,
        cursor: Option<&str>,
        pool: &PgPool,
    ) -> Result<CursorResult<T>, OrmError>;
}
```

### 10.3 Usage

```rust
// Offset pagination — for UI with page numbers
let page = Post::query()
    .where_eq("published", true)
    .order_by_desc("created_at")
    .paginate(20)
    .fetch_page(0, &pool)
    .await?;

// Cursor pagination — for infinite scroll / APIs
let result = Post::query()
    .order_by_desc("created_at")
    .cursor_paginate(20, "created_at")
    .fetch_after(req.cursor.as_deref(), &pool)
    .await?;
```

---

## 11. Error Handling

### 11.1 `OrmError` Type

```rust
// rok-orm-core/src/error.rs

#[derive(Debug, thiserror::Error)]
pub enum OrmError {
    #[error("Record not found in table `{model}`")]
    NotFound { model: &'static str },

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Unique constraint violation on column `{column}`")]
    UniqueViolation { column: String },

    #[error("Foreign key constraint violation")]
    ForeignKeyViolation,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Relation `{name}` not loaded — call .with(\"{name}\") on the query")]
    RelationNotLoaded { name: &'static str },

    #[error("Transaction failed: {0}")]
    Transaction(String),
}

impl OrmError {
    pub fn is_not_found(&self) -> bool { matches!(self, Self::NotFound { .. }) }
    pub fn is_unique_violation(&self) -> bool { matches!(self, Self::UniqueViolation { .. }) }

    // Parse sqlx::Error into typed OrmError
    pub fn from_sqlx(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref db_err) = err {
            if let Some(code) = db_err.code() {
                match code.as_ref() {
                    "23505" => return Self::UniqueViolation {
                        column: db_err.constraint().unwrap_or("unknown").into()
                    },
                    "23503" => return Self::ForeignKeyViolation,
                    _ => {}
                }
            }
        }
        Self::Database(err)
    }
}
```

### 11.2 Axum Integration

```rust
// In your API layer (rok-orm provides this as an optional feature)
#[cfg(feature = "axum")]
impl axum::response::IntoResponse for OrmError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            OrmError::NotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            OrmError::UniqueViolation { .. } => (StatusCode::CONFLICT, self.to_string()),
            OrmError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            _ => {
                tracing::error!("ORM error: {self}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into())
            }
        };
        (status, axum::Json(serde_json::json!({ "error": message }))).into_response()
    }
}
```

---

## 12. Folder Structure

### Final Production Layout

```
rok-orm/                                   # workspace root
├── Cargo.toml                             # [workspace] members
├── README.md
├── CHANGELOG.md
├── rok-orm-plan.md                        # this file
│
├── rok-orm-macros/                        # proc-macro crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                         # proc_macro entry points
│       ├── model_derive.rs                # Model derive logic
│       ├── relation_derive.rs             # Relation attr processing
│       └── attr_parser.rs                 # darling wrappers
│
├── rok-orm-core/                          # core library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── model.rs                       # Model trait
│       ├── query_builder.rs               # QueryBuilder<T>
│       ├── paginator.rs                   # Paginator<T> + CursorPaginator<T>
│       ├── relation.rs                    # RelationDef + EagerLoader
│       ├── scope.rs                       # GlobalScope + SoftDelete
│       ├── transaction.rs                 # transaction() helper
│       ├── executor.rs                    # OrmExecutor trait
│       ├── migration.rs                   # MigrationRunner
│       ├── value.rs                       # SqlValue enum
│       └── error.rs                       # OrmError
│
├── rok-orm/                               # public facade
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                         # pub use rok_orm_core::*; pub use rok_orm_macros::*;
│
├── tests/
│   ├── unit/
│   │   ├── query_builder_test.rs
│   │   ├── sql_compilation_test.rs
│   │   ├── error_parsing_test.rs
│   │   └── macro_expansion_test.rs
│   └── integration/
│       ├── model_crud_test.rs
│       ├── relations_test.rs
│       ├── pagination_test.rs
│       ├── soft_delete_test.rs
│       ├── transaction_test.rs
│       └── migration_test.rs
│
└── examples/
    ├── basic_crud.rs
    ├── relations_eager.rs
    ├── pagination.rs
    ├── transactions.rs
    └── soft_delete.rs
```

---

## 13. Cargo.toml & Dependencies

### Workspace Root

```toml
[workspace]
members = ["rok-orm-macros", "rok-orm-core", "rok-orm"]
resolver = "2"
```

### rok-orm-macros

```toml
[package]
name = "rok-orm-macros"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn       = { version = "2", features = ["full", "extra-traits"] }
quote     = "1"
proc-macro2 = "1"
darling   = "0.20"
heck      = "0.5"    # case conversion (snake_case, PascalCase)
```

### rok-orm-core

```toml
[package]
name = "rok-orm-core"
version = "0.1.0"
edition = "2021"

[dependencies]
sqlx        = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono", "migrate", "json"] }
tokio       = { version = "1",   features = ["full"] }
async-trait = "0.1"
serde       = { version = "1",   features = ["derive"] }
serde_json  = "1"
uuid        = { version = "1",   features = ["v4", "serde"] }
chrono      = { version = "0.4", features = ["serde"] }
thiserror   = "1"
futures     = "0.3"
base64      = "0.22"  # cursor pagination

[features]
default = ["postgres"]
postgres = []
mysql    = []
sqlite   = []
axum     = ["dep:axum", "dep:http"]

[dev-dependencies]
tokio-test  = "0.4"
sqlx        = { version = "0.8", features = ["test-utils"] }
```

### rok-orm (facade)

```toml
[package]
name = "rok-orm"
version = "0.1.0"
edition = "2021"

[dependencies]
rok-orm-core   = { version = "0.1", path = "../rok-orm-core" }
rok-orm-macros = { version = "0.1", path = "../rok-orm-macros" }

[features]
default  = ["postgres"]
postgres = ["rok-orm-core/postgres"]
mysql    = ["rok-orm-core/mysql"]
sqlite   = ["rok-orm-core/sqlite"]
axum     = ["rok-orm-core/axum"]
```

---

## 14. Implementation Roadmap

### Phase 1 — Core Skeleton (Week 1)

- [ ] Workspace setup, crate scaffolding
- [ ] `OrmError` with all variants + sqlx parsing
- [ ] `SqlValue` enum covering all DB-compatible Rust types
- [ ] `OrmExecutor` abstraction (Pool + Transaction)
- [ ] `Model` trait (no macro yet) — write manual impl for one test model
- [ ] `QueryBuilder<T>` — where, order, limit, offset, `to_sql()`, `get()`, `first()`
- [ ] Basic unit tests for SQL compilation
- [ ] Docker Compose with Postgres for integration tests

### Phase 2 — Proc-Macro (Week 2)

- [ ] `rok-orm-macros` crate — `lib.rs` proc_macro entry
- [ ] `attr_parser.rs` — darling wrappers for struct + field attrs
- [ ] `model_derive.rs` — generate `impl Model` for basic struct
- [ ] Generates: `table_name`, `primary_key`, `columns`, `find`, `save`, `delete`, `count`, `exists`
- [ ] `#[orm(skip)]`, `#[orm(rename)]`, `#[orm(table)]` support
- [ ] `cargo expand` validation — inspect generated code
- [ ] Macro expansion tests with `trybuild` crate

### Phase 3 — Features & Scopes (Week 3)

- [ ] `#[orm(soft_delete)]` — generates `SoftDelete` impl, rewrites `delete()`, injects global scope
- [ ] `#[orm(timestamps)]` — auto-fills `updated_at` on `save()`
- [ ] Global scope system (`apply_global_scope`)
- [ ] `SoftDelete` trait: `restore()`, `force_delete()`, `with_trashed()`, `only_trashed()`
- [ ] Soft delete integration tests

### Phase 4 — Relations (Week 4)

- [ ] `RelationDef` structure
- [ ] `#[has_many(Model, fk = "...")]` macro attribute + code gen
- [ ] `#[belongs_to(Model, fk = "...")]` + `#[has_one(Model, fk = "...")]`
- [ ] `EagerLoader` batch query (N+1 prevention)
- [ ] `QueryBuilder::with("relation")` wiring
- [ ] Relations integration tests (with real FK constraints)

### Phase 5 — Pagination & Transactions (Week 5)

- [ ] `Paginator<T>` — offset pagination, `PaginatedResult<T>` with full metadata
- [ ] `CursorPaginator<T>` — base64-encoded cursor, `CursorResult<T>`
- [ ] `QueryBuilder::paginate()` and `cursor_paginate()` terminals
- [ ] `transaction()` convenience wrapper
- [ ] Transaction integration tests (rollback scenarios)

### Phase 6 — Migration Engine (Week 6)

- [ ] `MigrationRunner` — detect pending, run, rollback
- [ ] JSON schema → SQL migration generator
- [ ] Type mapping table (Postgres / MySQL / SQLite)
- [ ] Down migration auto-generation
- [ ] `rok-orm migrate` CLI integration (bridges to rok `bash` step)

### Phase 7 — Polish & Release (Week 7)

- [ ] Axum feature flag — `OrmError: IntoResponse`
- [ ] `QueryBuilder::to_sql()` debug output
- [ ] `tracing` spans on all DB operations
- [ ] Full doc-comments, `#[must_use]` on builder methods
- [ ] README with end-to-end quickstart
- [ ] Publish `rok-orm-macros`, `rok-orm-core`, `rok-orm` to crates.io
- [ ] Add rok JSON templates for model generation (bridges to rok-cli-orm)

---

## 15. rok Integration Layer

rok-orm is designed to be driven entirely by rok. These are the rok step payloads that power the full workflow:

### Generate a Model from JSON

```json
{
  "name": "generate-post-model",
  "steps": [
    {
      "type": "template",
      "builtin": "rok-orm-model",
      "props": {
        "name": "Post",
        "table": "posts",
        "soft_delete": true,
        "timestamps": true,
        "columns": [
          { "name": "id",        "type": "Uuid",           "primary": true },
          { "name": "user_id",   "type": "Uuid" },
          { "name": "title",     "type": "String" },
          { "name": "published", "type": "bool" }
        ],
        "relations": [
          { "kind": "belongs_to", "model": "User", "fk": "user_id" }
        ]
      },
      "output": "src/domain/post/model.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-migration",
      "props": { "ref": 0 },
      "output": "migrations/{{ timestamp }}_create_posts.sql"
    },
    { "type": "bash", "cmd": "cargo fmt" },
    { "type": "bash", "cmd": "cargo check" }
  ]
}
```

### Run Migrations via rok

```json
{
  "name": "migrate",
  "steps": [
    { "type": "bash", "cmd": "rok-orm migrate run", "env": { "DATABASE_URL": "${DATABASE_URL}" } },
    { "type": "bash", "cmd": "cargo test -- --test-threads=1" }
  ]
}
```

---

## 16. Testing Strategy

### Unit Tests

Every SQL-producing method is unit-tested by calling `to_sql()` and asserting the string and params vector:

```rust
#[test]
fn test_where_eq_compiles() {
    let (sql, params) = Post::query()
        .where_eq("published", true)
        .order_by_desc("created_at")
        .limit(10)
        .to_sql();

    assert_eq!(sql,
        "SELECT id, user_id, title, published, created_at, updated_at, deleted_at \
         FROM posts WHERE deleted_at IS NULL AND published = $1 \
         ORDER BY created_at DESC LIMIT 10"
    );
    assert_eq!(params, vec![SqlValue::Bool(true)]);
}
```

### Integration Tests (real DB)

```rust
#[sqlx::test]
async fn test_create_and_find(pool: PgPool) {
    let mut post = Post {
        id: Uuid::new_v4(),
        title: "Hello".into(),
        published: false,
        ..Default::default()
    };
    post.save(&pool).await.unwrap();

    let found = Post::find(post.id, &pool).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().title, "Hello");
}

#[sqlx::test]
async fn test_soft_delete_invisible(pool: PgPool) {
    let mut post = create_test_post(&pool).await;
    post.clone().delete(&pool).await.unwrap();

    let found = Post::find(post.id, &pool).await.unwrap();
    assert!(found.is_none());  // soft-deleted — not visible

    let found_trashed = Post::with_trashed()
        .where_eq("id", post.id)
        .first(&pool)
        .await.unwrap();
    assert!(found_trashed.is_some());  // visible with with_trashed()
}
```

### Test Infrastructure

```toml
# .env.test
DATABASE_URL=postgres://postgres:password@localhost:5432/rok_orm_test
```

```yaml
# docker-compose.test.yml
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: rok_orm_test
      POSTGRES_PASSWORD: password
    ports: ["5432:5432"]
```

---

## 17. Full API Reference

### Model — Static Methods

| Method | Signature | Description |
|---|---|---|
| `find` | `async fn find(id, pool) -> Result<Option<Self>>` | Find by primary key |
| `find_or_fail` | `async fn find_or_fail(id, pool) -> Result<Self>` | Find or return NotFound |
| `all` | `async fn all(pool) -> Result<Vec<Self>>` | Fetch all rows |
| `query` | `fn query() -> QueryBuilder<Self>` | Start a fluent query |
| `count` | `async fn count(pool) -> Result<i64>` | Count all rows |
| `exists` | `async fn exists(id, pool) -> Result<bool>` | Check existence |

### Model — Instance Methods

| Method | Signature | Description |
|---|---|---|
| `save` | `async fn save(&mut self, pool) -> Result<()>` | INSERT or UPDATE (upsert) |
| `delete` | `async fn delete(self, pool) -> Result<()>` | DELETE (or soft-delete) |
| `before_save` | `async fn before_save(&mut self) -> Result<()>` | Hook — override to add logic |
| `after_save` | `async fn after_save(&mut self) -> Result<()>` | Hook — override to add logic |

### QueryBuilder — Filters

| Method | Description |
|---|---|
| `where_eq(col, val)` | WHERE col = $n |
| `where_ne(col, val)` | WHERE col != $n |
| `where_gt / gte / lt / lte` | Comparison operators |
| `where_like(col, pattern)` | WHERE col LIKE $n |
| `where_ilike(col, pattern)` | WHERE col ILIKE $n (case-insensitive) |
| `where_in(col, vals)` | WHERE col IN ($1, $2, ...) |
| `where_not_in(col, vals)` | WHERE col NOT IN (...) |
| `where_null(col)` | WHERE col IS NULL |
| `where_not_null(col)` | WHERE col IS NOT NULL |
| `where_raw(sql)` | Raw SQL fragment (no binding) |
| `or_where(col, op, val)` | OR connector |

### QueryBuilder — Terminals

| Method | Description |
|---|---|
| `get(pool)` | Fetch Vec<T> |
| `first(pool)` | Fetch Option<T> |
| `first_or_fail(pool)` | Fetch T or NotFound |
| `count(pool)` | COUNT(*) |
| `exists(pool)` | COUNT(*) > 0 |
| `pluck(col, pool)` | Fetch single column as Vec<SqlValue> |
| `paginate(n)` | Returns Paginator<T> |
| `cursor_paginate(n, col)` | Returns CursorPaginator<T> |
| `to_sql()` | Debug — returns (String, Vec<SqlValue>) |

### SoftDelete — Additional Methods

| Method | Description |
|---|---|
| `restore(&mut self, pool)` | Set deleted_at = NULL |
| `force_delete(self, pool)` | Hard DELETE FROM |
| `with_trashed()` | QueryBuilder bypassing soft-delete scope |
| `only_trashed()` | QueryBuilder filtering to deleted rows only |

---

*rok-orm — part of the rok ecosystem. Run One. Know All.*
