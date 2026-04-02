# rok-cli-orm — ORM Codegen Command Reference

> All rok commands for generating, managing, and evolving rok-orm models, migrations, queries, and repositories. One JSON. All Changes.

**rok version**: v0.10.0+  
**rok-orm version**: v0.1.0+  
**Built-in template prefix**: `rok-orm-*`

---

## Table of Contents

1. [Command Overview](#1-command-overview)
2. [rok orm — Command Group](#2-rok-orm--command-group)
3. [Model Generation Commands](#3-model-generation-commands)
4. [Migration Commands](#4-migration-commands)
5. [Query & Repository Commands](#5-query--repository-commands)
6. [Relation Commands](#6-relation-commands)
7. [Scaffold Commands](#7-scaffold-commands)
8. [Inspection Commands](#8-inspection-commands)
9. [Refactor Commands](#9-refactor-commands)
10. [Validation Commands](#10-validation-commands)
11. [rok JSON Payload Reference](#11-rok-json-payload-reference)
12. [Built-in Templates Reference](#12-built-in-templates-reference)
13. [Config File (.rokorm)](#13-config-file-rokorm)
14. [Full Workflow Examples](#14-full-workflow-examples)
15. [Agent Golden Rules for rok-orm](#15-agent-golden-rules-for-rok-orm)

---

## 1. Command Overview

All ORM codegen is driven by rok. There are three modes:

**Mode A — Direct CLI shorthand** (`rok orm <subcommand>`)
Thin wrappers that build and execute rok JSON payloads internally. For humans.

**Mode B — rok JSON payload** (`rok -f payload.json` or `rok -j '...'`)
The raw rok execution engine. For agents and automation.

**Mode C — rok saved tasks** (`rok run orm:<task>`)
Pre-saved named tasks for common workflows. For teams and CI.

---

## 2. rok orm — Command Group

### Help

```bash
rok orm --help
rok orm <subcommand> --help
```

### Subcommand Index

| Subcommand | Description |
|---|---|
| `rok orm model` | Model generation commands |
| `rok orm migrate` | Migration run/rollback/status |
| `rok orm query` | Generate typed query methods |
| `rok orm repo` | Generate repository structs |
| `rok orm relation` | Add relations to existing models |
| `rok orm scaffold` | Full CRUD scaffold (model + repo + handler) |
| `rok orm inspect` | Inspect DB schema or model state |
| `rok orm refactor` | Rename models, columns, tables |
| `rok orm validate` | Validate JSON schema or generated code |

---

## 3. Model Generation Commands

### 3.1 `rok orm model generate`

Generate a complete Rust model struct with `#[derive(Model)]` from a JSON schema.

```bash
rok orm model generate \
  --from schema.json \
  --out src/domain/post/model.rs \
  --soft-delete \
  --timestamps \
  --derives "Debug,Clone,Serialize,Deserialize" \
  --pool PgPool \
  --error OrmError
```

**Flags:**

| Flag | Default | Description |
|---|---|---|
| `--from <file>` | stdin | JSON schema file path |
| `--out <file>` | stdout | Output Rust file path |
| `--soft-delete` | false | Add `#[orm(soft_delete)]` |
| `--timestamps` | true | Add `#[orm(timestamps)]` |
| `--derives <list>` | `Debug,Clone,Serialize,Deserialize` | Comma-separated derive list |
| `--pool <type>` | `PgPool` | Pool type: PgPool, MySqlPool, SqlitePool |
| `--error <type>` | `OrmError` | Error type name |
| `--no-async-trait` | false | Skip `#[async_trait]` |
| `--namespace <mod>` | none | Wrap in `pub mod <namespace>` |
| `--fmt` | true | Run `cargo fmt` after generation |
| `--check` | false | Dry-run: print to stdout, no file write |

**Underlying rok payload:**

```json
{
  "name": "orm:model:generate",
  "steps": [
    {
      "type": "template",
      "builtin": "rok-orm-model",
      "props": {
        "name": "Post",
        "table": "posts",
        "soft_delete": true,
        "timestamps": true,
        "derives": ["Debug", "Clone", "Serialize", "Deserialize"],
        "pool_type": "PgPool",
        "error_type": "OrmError",
        "columns": [],
        "relations": []
      },
      "output": "src/domain/post/model.rs"
    },
    { "type": "bash", "cmd": "cargo fmt -- src/domain/post/model.rs" },
    { "type": "bash", "cmd": "cargo check --quiet" }
  ]
}
```

---

### 3.2 `rok orm model from-json`

Generate directly from inline JSON without a schema file:

```bash
rok orm model from-json \
  --json '{"name":"User","table":"users","columns":[{"name":"id","type":"Uuid","primary":true}]}' \
  --out src/domain/user/model.rs
```

---

### 3.3 `rok orm model from-db`

**Introspect a live database** and generate the model from an existing table:

```bash
rok orm model from-db \
  --table users \
  --database-url $DATABASE_URL \
  --out src/domain/user/model.rs \
  --soft-delete
```

**What it does:**
1. Connects to the DB via `DATABASE_URL`
2. Runs `SELECT column_name, data_type, is_nullable, column_default FROM information_schema.columns`
3. Maps PostgreSQL types → Rust types
4. Detects `deleted_at` column → suggests `--soft-delete`
5. Generates the full model struct

**Underlying rok payload:**

```json
{
  "name": "orm:model:from-db",
  "steps": [
    {
      "type": "bash",
      "cmd": "rok-orm introspect --table users --url $DATABASE_URL --format json",
      "id": "introspect"
    },
    {
      "type": "template",
      "builtin": "rok-orm-model",
      "props": { "ref": 0, "pick": "schema" },
      "output": "src/domain/user/model.rs"
    },
    { "type": "bash", "cmd": "cargo fmt -- src/domain/user/model.rs" }
  ]
}
```

---

### 3.4 `rok orm model add-column`

Add a column to an existing model and generate the corresponding migration:

```bash
rok orm model add-column \
  --model src/domain/post/model.rs \
  --name view_count \
  --type i64 \
  --default 0 \
  --nullable false \
  --migrate
```

**Flags:**

| Flag | Description |
|---|---|
| `--model <path>` | Path to existing model.rs |
| `--name <col>` | Column name |
| `--type <type>` | Rust type |
| `--default <val>` | Default value |
| `--nullable` | Allow NULL |
| `--migrate` | Also generate + run migration |

**Underlying rok payload:**

```json
{
  "name": "orm:model:add-column",
  "steps": [
    {
      "type": "patch",
      "path": "src/domain/post/model.rs",
      "edits": [
        {
          "find": "pub deleted_at: Option<DateTime<Utc>>,",
          "replace": "pub view_count: i64,\n    pub deleted_at: Option<DateTime<Utc>>,"
        }
      ]
    },
    {
      "type": "template",
      "builtin": "rok-orm-migration-add-column",
      "props": {
        "table": "posts",
        "column": "view_count",
        "pg_type": "BIGINT",
        "default": "0",
        "nullable": false
      },
      "output": "migrations/{{ timestamp }}_add_view_count_to_posts.sql"
    },
    { "type": "bash", "cmd": "cargo fmt -- src/domain/post/model.rs" },
    { "type": "bash", "cmd": "cargo check --quiet" }
  ]
}
```

---

### 3.5 `rok orm model remove-column`

```bash
rok orm model remove-column \
  --model src/domain/post/model.rs \
  --name view_count \
  --migrate
```

---

### 3.6 `rok orm model list`

List all detected model files in the project:

```bash
rok orm model list
rok orm model list --path src/domain
rok orm model list --json    # machine-readable output
```

**Output:**
```
rok-orm models detected (4):
  src/domain/user/model.rs        User        → users
  src/domain/post/model.rs        Post        → posts
  src/domain/comment/model.rs     Comment     → comments
  src/domain/tag/model.rs         Tag         → tags
```

---

## 4. Migration Commands

### 4.1 `rok orm migrate generate`

Generate a migration file from a model JSON schema without running it:

```bash
rok orm migrate generate \
  --from schema.json \
  --out migrations/ \
  --driver postgres
```

**Flags:**

| Flag | Default | Description |
|---|---|---|
| `--from <file>` | stdin | JSON schema file |
| `--out <dir>` | `./migrations` | Output directory |
| `--driver <db>` | `postgres` | postgres, mysql, sqlite |
| `--down` | true | Also generate DOWN migration |
| `--timestamp` | true | Prefix file with timestamp |

**Generated files:**
```
migrations/
  20240101_120000_create_posts_table.sql
  20240101_120000_create_posts_table.down.sql
```

---

### 4.2 `rok orm migrate run`

Run all pending migrations:

```bash
rok orm migrate run
rok orm migrate run --database-url $DATABASE_URL
rok orm migrate run --path migrations/
rok orm migrate run --dry-run    # print SQL, don't execute
```

**Underlying rok payload:**

```json
{
  "name": "orm:migrate:run",
  "steps": [
    { "type": "snapshot", "path": ".", "snapshot_id": "pre-migrate" },
    {
      "type": "bash",
      "cmd": "sqlx migrate run --database-url $DATABASE_URL",
      "id": "migrate"
    },
    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": 1 },
      "then": [
        { "type": "restore", "snapshot_id": "pre-migrate" }
      ]
    }
  ]
}
```

---

### 4.3 `rok orm migrate rollback`

```bash
rok orm migrate rollback              # rollback last 1
rok orm migrate rollback --steps 3   # rollback last 3
rok orm migrate rollback --all        # rollback everything (fresh state)
```

---

### 4.4 `rok orm migrate status`

```bash
rok orm migrate status
```

**Output:**
```
Migration Status (DATABASE_URL=postgres://...)
──────────────────────────────────────────────────────────
  ✓  20240101_000000_create_users_table.sql       applied  2024-01-01 10:00:00
  ✓  20240102_000000_create_posts_table.sql       applied  2024-01-02 11:00:00
  ✗  20240103_000000_add_view_count_to_posts.sql  pending
──────────────────────────────────────────────────────────
  2 applied · 1 pending
```

---

### 4.5 `rok orm migrate fresh`

Drop all tables and re-run all migrations from scratch:

```bash
rok orm migrate fresh
rok orm migrate fresh --seed    # also run seeders after
```

---

### 4.6 `rok orm migrate seed`

```bash
rok orm migrate seed
rok orm migrate seed --file seeds/users.json
rok orm migrate seed --model User --count 50    # generate N fake rows
```

---

## 5. Query & Repository Commands

### 5.1 `rok orm query generate`

Generate typed query methods for a model — custom finders beyond the default `find()` and `query()`:

```bash
rok orm query generate \
  --model Post \
  --out src/domain/post/queries.rs \
  --methods find_by_slug,find_published,find_by_user
```

**JSON schema for query definitions:**

```json
{
  "model": "Post",
  "table": "posts",
  "queries": [
    {
      "name": "find_by_slug",
      "args": [{ "name": "slug", "type": "String" }],
      "filters": [{ "col": "slug", "op": "eq", "arg": "slug" }],
      "returns": "Option<Post>"
    },
    {
      "name": "find_published",
      "args": [{ "name": "page", "type": "u64" }],
      "filters": [
        { "col": "published", "op": "eq", "value": true },
        { "col": "deleted_at", "op": "is_null" }
      ],
      "order": [{ "col": "created_at", "dir": "desc" }],
      "paginate": 20,
      "returns": "PaginatedResult<Post>"
    },
    {
      "name": "count_by_user",
      "args": [{ "name": "user_id", "type": "Uuid" }],
      "filters": [{ "col": "user_id", "op": "eq", "arg": "user_id" }],
      "returns": "i64",
      "aggregate": "count"
    }
  ]
}
```

**Generated code:**

```rust
// src/domain/post/queries.rs — generated by rok-orm

use sqlx::PgPool;
use uuid::Uuid;
use rok_orm::{OrmError, PaginatedResult};
use super::model::Post;

impl Post {
    pub async fn find_by_slug(
        slug: &str,
        pool: &PgPool,
    ) -> Result<Option<Post>, OrmError> {
        Post::query()
            .where_eq("slug", slug)
            .first(pool)
            .await
    }

    pub async fn find_published(
        page: u64,
        pool: &PgPool,
    ) -> Result<PaginatedResult<Post>, OrmError> {
        Post::query()
            .where_eq("published", true)
            .order_by_desc("created_at")
            .paginate(20)
            .fetch_page(page, pool)
            .await
    }

    pub async fn count_by_user(
        user_id: Uuid,
        pool: &PgPool,
    ) -> Result<i64, OrmError> {
        Post::query()
            .where_eq("user_id", user_id)
            .count(pool)
            .await
    }
}
```

---

### 5.2 `rok orm repo generate`

Generate a repository struct + trait (for SOLID dependency inversion):

```bash
rok orm repo generate \
  --model Post \
  --out src/infrastructure/repositories/post_repo.rs \
  --trait src/domain/post/repository.rs \
  --methods "find_by_id,find_published,create,update,delete"
```

**Generated trait:**

```rust
// src/domain/post/repository.rs — generated by rok-orm

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, OrmError>;
    async fn find_published(&self, page: u64) -> Result<PaginatedResult<Post>, OrmError>;
    async fn create(&self, dto: CreatePostDto) -> Result<Post, OrmError>;
    async fn update(&self, id: Uuid, dto: UpdatePostDto) -> Result<Post, OrmError>;
    async fn delete(&self, id: Uuid) -> Result<(), OrmError>;
}
```

**Generated implementation:**

```rust
// src/infrastructure/repositories/post_repo.rs — generated by rok-orm

pub struct PgPostRepository {
    pool: PgPool,
}

impl PgPostRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl PostRepository for PgPostRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, OrmError> {
        Post::find(id, &self.pool).await
    }

    async fn find_published(&self, page: u64) -> Result<PaginatedResult<Post>, OrmError> {
        Post::find_published(page, &self.pool).await
    }

    async fn create(&self, dto: CreatePostDto) -> Result<Post, OrmError> {
        let mut post = Post {
            id: Uuid::new_v4(),
            title: dto.title,
            body: dto.body,
            published: dto.published.unwrap_or(false),
            user_id: dto.user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };
        post.save(&self.pool).await?;
        Ok(post)
    }

    async fn update(&self, id: Uuid, dto: UpdatePostDto) -> Result<Post, OrmError> {
        let mut post = Post::find_or_fail(id, &self.pool).await?;
        if let Some(title) = dto.title { post.title = title; }
        if let Some(body) = dto.body { post.body = Some(body); }
        if let Some(published) = dto.published { post.published = published; }
        post.save(&self.pool).await?;
        Ok(post)
    }

    async fn delete(&self, id: Uuid) -> Result<(), OrmError> {
        let post = Post::find_or_fail(id, &self.pool).await?;
        post.delete(&self.pool).await
    }
}
```

---

### 5.3 `rok orm repo mock`

Generate a `MockRepository` for use in unit tests:

```bash
rok orm repo mock \
  --trait src/domain/post/repository.rs \
  --out src/domain/post/mock_repository.rs
```

**Generated mock:**

```rust
// src/domain/post/mock_repository.rs — generated by rok-orm

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct MockPostRepository {
    store: Arc<Mutex<HashMap<Uuid, Post>>>,
}

impl MockPostRepository {
    pub fn new() -> Self {
        Self { store: Arc::new(Mutex::new(HashMap::new())) }
    }
    pub fn seed(self, posts: Vec<Post>) -> Self {
        let mut store = self.store.lock().unwrap();
        for post in posts { store.insert(post.id, post); }
        drop(store); self
    }
}

#[async_trait]
impl PostRepository for MockPostRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, OrmError> {
        Ok(self.store.lock().unwrap().get(&id).cloned())
    }
    // ... all methods generated
}
```

---

## 6. Relation Commands

### 6.1 `rok orm relation add`

Add a relation to an existing model file:

```bash
rok orm relation add \
  --model src/domain/user/model.rs \
  --kind has_many \
  --related Post \
  --fk user_id

rok orm relation add \
  --model src/domain/post/model.rs \
  --kind belongs_to \
  --related User \
  --fk user_id

rok orm relation add \
  --model src/domain/user/model.rs \
  --kind has_one \
  --related Profile \
  --fk user_id

rok orm relation add \
  --model src/domain/post/model.rs \
  --kind many_to_many \
  --related Tag \
  --pivot post_tags \
  --local-fk post_id \
  --foreign-fk tag_id
```

**Underlying rok payload for `has_many`:**

```json
{
  "name": "orm:relation:add",
  "steps": [
    {
      "type": "patch",
      "path": "src/domain/user/model.rs",
      "edits": [
        {
          "find": "#[orm(table = \"users\")]",
          "replace": "#[orm(table = \"users\")]\n#[has_many(Post, fk = \"user_id\")]"
        }
      ]
    },
    { "type": "bash", "cmd": "cargo fmt -- src/domain/user/model.rs" },
    { "type": "bash", "cmd": "cargo check --quiet" }
  ]
}
```

---

### 6.2 `rok orm relation list`

List all relations detected across all models:

```bash
rok orm relation list
rok orm relation list --model User
rok orm relation list --json
```

**Output:**
```
rok-orm relations detected:
  User      has_many    → Post        (fk: user_id)
  User      has_one     → Profile     (fk: user_id)
  Post      belongs_to  → User        (fk: user_id)
  Post      has_many    → Comment     (fk: post_id)
  Post      many_to_many → Tag        (pivot: post_tags)
  Comment   belongs_to  → Post        (fk: post_id)
  Comment   belongs_to  → User        (fk: user_id)
```

---

## 7. Scaffold Commands

### 7.1 `rok orm scaffold full`

Generate the **complete vertical slice** for a model: struct + migration + repository trait + repository impl + mock + Axum handler + DTOs:

```bash
rok orm scaffold full \
  --from schema.json \
  --domain src/domain \
  --infra src/infrastructure \
  --api src/api/v1 \
  --soft-delete \
  --timestamps \
  --migrate
```

**What gets generated:**

```
src/
├── domain/<model_snake>/
│   ├── model.rs                 # #[derive(Model)] struct
│   ├── repository.rs            # PostRepository trait
│   ├── service.rs               # PostService (business logic stub)
│   ├── queries.rs               # Typed query methods
│   └── mock_repository.rs       # MockPostRepository for tests
│
├── infrastructure/repositories/
│   └── post_repo.rs             # PgPostRepository impl
│
└── api/v1/<model_snake>/
    ├── mod.rs
    ├── handlers.rs              # Axum handlers (CRUD)
    └── dto.rs                   # CreatePostDto, UpdatePostDto, PostResponse
│
migrations/
└── {{ timestamp }}_create_posts_table.sql
```

**Underlying rok payload:**

```json
{
  "name": "orm:scaffold:full",
  "steps": [
    { "type": "snapshot", "path": ".", "snapshot_id": "pre-scaffold" },
    {
      "type": "template",
      "builtin": "rok-orm-model",
      "props": { "ref": "schema" },
      "output": "src/domain/{{ model_snake }}/model.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-repository-trait",
      "props": { "ref": "schema" },
      "output": "src/domain/{{ model_snake }}/repository.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-service-stub",
      "props": { "ref": "schema" },
      "output": "src/domain/{{ model_snake }}/service.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-queries",
      "props": { "ref": "schema" },
      "output": "src/domain/{{ model_snake }}/queries.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-mock-repository",
      "props": { "ref": "schema" },
      "output": "src/domain/{{ model_snake }}/mock_repository.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-pg-repository",
      "props": { "ref": "schema" },
      "output": "src/infrastructure/repositories/{{ model_snake }}_repo.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-migration",
      "props": { "ref": "schema" },
      "output": "migrations/{{ timestamp }}_create_{{ table }}_table.sql"
    },
    {
      "type": "template",
      "builtin": "rok-orm-axum-handler",
      "props": { "ref": "schema" },
      "output": "src/api/v1/{{ model_snake }}/handlers.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-dto",
      "props": { "ref": "schema" },
      "output": "src/api/v1/{{ model_snake }}/dto.rs"
    },
    { "type": "bash", "cmd": "cargo fmt" },
    { "type": "bash", "cmd": "cargo check --quiet" },
    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": 10 },
      "then": [{ "type": "restore", "snapshot_id": "pre-scaffold" }]
    }
  ]
}
```

---

### 7.2 `rok orm scaffold model-only`

Struct + migration only — no handlers, no repo:

```bash
rok orm scaffold model-only --from schema.json --out src/domain/
```

---

### 7.3 `rok orm scaffold repo-only`

Repository trait + impl + mock for an existing model:

```bash
rok orm scaffold repo-only \
  --model src/domain/post/model.rs \
  --out-trait src/domain/post/repository.rs \
  --out-impl src/infrastructure/repositories/post_repo.rs \
  --out-mock src/domain/post/mock_repository.rs
```

---

### 7.4 `rok orm scaffold handler`

Axum CRUD handlers + DTOs for an existing model:

```bash
rok orm scaffold handler \
  --model src/domain/post/model.rs \
  --out src/api/v1/posts/ \
  --operations "list,get,create,update,delete" \
  --auth-guard
```

**Generated handlers:**

```rust
// GET    /posts          → list_posts
// GET    /posts/:id      → get_post
// POST   /posts          → create_post
// PUT    /posts/:id      → update_post
// DELETE /posts/:id      → delete_post
```

---

## 8. Inspection Commands

### 8.1 `rok orm inspect model`

Print a structured summary of a model file:

```bash
rok orm inspect model src/domain/post/model.rs
rok orm inspect model src/domain/post/model.rs --json
```

**Output:**
```
Model: Post
  Table:       posts
  Primary Key: id (Uuid)
  Soft Delete: yes (deleted_at)
  Timestamps:  yes (created_at, updated_at)
  Columns (8):
    id           Uuid              NOT NULL  PK
    user_id      Uuid              NOT NULL
    title        String            NOT NULL
    body         Option<String>    NULLABLE
    published    bool              NOT NULL
    created_at   DateTime<Utc>     NOT NULL
    updated_at   DateTime<Utc>     NOT NULL
    deleted_at   Option<DateTime>  NULLABLE
  Relations (1):
    belongs_to User (fk: user_id)
```

---

### 8.2 `rok orm inspect db`

Inspect the live database and show table + column info:

```bash
rok orm inspect db --table posts
rok orm inspect db --all
rok orm inspect db --diff src/domain/post/model.rs    # compare model vs DB
```

**Diff output:**
```
Diff: Post model vs posts table (DATABASE_URL)
  ✓  id           UUID         matches
  ✓  user_id      UUID         matches
  ✓  title        VARCHAR(255) matches
  ✗  slug         TEXT         in DB but NOT in model  ← add column
  ✗  view_count   BIGINT       in model but NOT in DB  ← generate migration
```

---

### 8.3 `rok orm inspect relations`

```bash
rok orm inspect relations --path src/domain/
rok orm inspect relations --model User
rok orm inspect relations --detect-missing    # find FK columns with no declared relation
```

---

### 8.4 `rok orm inspect migrations`

```bash
rok orm inspect migrations --path migrations/
rok orm inspect migrations --detect-orphans   # migrations with no matching model
```

---

## 9. Refactor Commands

### 9.1 `rok orm refactor rename-model`

Rename a model across all files — struct, table name, file, imports, references:

```bash
rok orm refactor rename-model \
  --from BlogPost \
  --to Article \
  --path src/ \
  --rename-file \
  --rename-table
```

**Underlying rok payload:**

```json
{
  "name": "orm:refactor:rename-model",
  "steps": [
    { "type": "snapshot", "path": ".", "snapshot_id": "pre-rename" },
    { "type": "refactor", "symbol": "BlogPost", "rename_to": "Article", "path": "./src" },
    {
      "type": "replace",
      "path": "src/**/*.rs",
      "pattern": "blog_posts",
      "replacement": "articles"
    },
    {
      "type": "mv",
      "from": "src/domain/blog_post",
      "to": "src/domain/article"
    },
    {
      "type": "template",
      "builtin": "rok-orm-migration-rename-table",
      "props": { "from_table": "blog_posts", "to_table": "articles" },
      "output": "migrations/{{ timestamp }}_rename_blog_posts_to_articles.sql"
    },
    { "type": "bash", "cmd": "cargo fmt" },
    { "type": "bash", "cmd": "cargo check --quiet" },
    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": 5 },
      "then": [{ "type": "restore", "snapshot_id": "pre-rename" }]
    }
  ]
}
```

---

### 9.2 `rok orm refactor rename-column`

```bash
rok orm refactor rename-column \
  --model src/domain/post/model.rs \
  --from body \
  --to content \
  --migrate
```

---

### 9.3 `rok orm refactor rename-table`

```bash
rok orm refactor rename-table \
  --model src/domain/post/model.rs \
  --from posts \
  --to articles \
  --migrate
```

---

### 9.4 `rok orm refactor change-type`

```bash
rok orm refactor change-type \
  --model src/domain/post/model.rs \
  --column status \
  --from String \
  --to PostStatus \
  --migrate
```

---

## 10. Validation Commands

### 10.1 `rok orm validate schema`

Validate a JSON schema file against the rok-orm schema specification:

```bash
rok orm validate schema schema.json
rok orm validate schema schema.json --strict    # also check naming conventions
```

**Output:**
```
Validating schema.json...
  ✓  name: "Post" (valid)
  ✓  table: "posts" (valid)
  ✓  columns: 6 columns defined
  ✗  column "user_id": type "uuid" should be "Uuid" (Rust casing)
  ✗  relation "belongs_to": missing required field "fk"
  ✓  soft_delete compatible with schema
Validation: 2 errors found
```

---

### 10.2 `rok orm validate model`

Validate that a Rust model file compiles and satisfies the `Model` trait correctly:

```bash
rok orm validate model src/domain/post/model.rs
```

**Underlying rok payload:**

```json
{
  "name": "orm:validate:model",
  "steps": [
    { "type": "bash", "cmd": "cargo check --quiet 2>&1", "id": "check" },
    { "type": "bash", "cmd": "cargo clippy -- -D warnings 2>&1", "id": "clippy" },
    {
      "type": "summarize",
      "path": "src/domain/post/model.rs",
      "filter_exports": true
    }
  ]
}
```

---

### 10.3 `rok orm validate sync`

Check that all models are in sync with the database schema:

```bash
rok orm validate sync --database-url $DATABASE_URL --path src/domain/
```

**Output:**
```
rok-orm sync validation (4 models):
  ✓  User      → users       in sync
  ✓  Post      → posts       in sync
  ✗  Comment   → comments    OUT OF SYNC — column "score" in model missing in DB
  ✓  Tag       → tags        in sync
Run `rok orm migrate run` to apply pending migrations.
```

---

## 11. rok JSON Payload Reference

### Standard Model Generation Payload

```json
{
  "name": "generate-post-model",
  "description": "Generate Post ORM model with migration",
  "options": {
    "stop_on_error": true
  },
  "steps": [
    {
      "type": "snapshot",
      "path": ".",
      "snapshot_id": "pre-generate"
    },
    {
      "type": "template",
      "builtin": "rok-orm-model",
      "props": {
        "name": "Post",
        "table": "posts",
        "soft_delete": true,
        "timestamps": true,
        "pool_type": "PgPool",
        "error_type": "OrmError",
        "derives": ["Debug", "Clone", "Serialize", "Deserialize"],
        "columns": [
          { "name": "id",        "type": "Uuid",            "primary": true },
          { "name": "user_id",   "type": "Uuid",            "nullable": false },
          { "name": "title",     "type": "String",          "nullable": false },
          { "name": "body",      "type": "Option<String>",  "nullable": true },
          { "name": "published", "type": "bool",            "default": "false" },
          { "name": "slug",      "type": "String",          "nullable": false, "unique": true }
        ],
        "relations": [
          { "kind": "belongs_to", "model": "User", "fk": "user_id" },
          { "kind": "has_many",   "model": "Comment", "fk": "post_id" },
          { "kind": "many_to_many", "model": "Tag", "pivot": "post_tags", "local_fk": "post_id", "foreign_fk": "tag_id" }
        ]
      },
      "output": "src/domain/post/model.rs"
    },
    {
      "type": "template",
      "builtin": "rok-orm-migration",
      "props": {
        "name": "Post",
        "table": "posts",
        "driver": "postgres",
        "columns": { "ref": 1, "pick": "props.columns" },
        "indexes": [
          { "name": "idx_posts_slug",       "columns": ["slug"],       "unique": true },
          { "name": "idx_posts_user_id",    "columns": ["user_id"] },
          { "name": "idx_posts_published",  "columns": ["published", "created_at"] }
        ]
      },
      "output": "migrations/{{ timestamp }}_create_posts_table.sql"
    },
    {
      "type": "bash",
      "cmd": "cargo fmt",
      "id": "fmt"
    },
    {
      "type": "bash",
      "cmd": "cargo check --quiet",
      "id": "check"
    },
    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": 4 },
      "then": [
        { "type": "restore", "snapshot_id": "pre-generate" }
      ]
    }
  ]
}
```

---

### Full Scaffold Payload (Agent Use)

```json
{
  "name": "scaffold-post-full",
  "description": "Full vertical slice: model + migration + repo + handler",
  "props": {
    "model_name":   "Post",
    "model_snake":  "post",
    "table":        "posts",
    "soft_delete":  true,
    "timestamps":   true
  },
  "steps": [
    { "type": "snapshot", "path": ".", "snapshot_id": "pre-scaffold" },

    { "type": "template", "builtin": "rok-orm-model",
      "props": { "ref": "props" },
      "output": "src/domain/{{ model_snake }}/model.rs" },

    { "type": "template", "builtin": "rok-orm-migration",
      "props": { "ref": "props" },
      "output": "migrations/{{ timestamp }}_create_{{ table }}_table.sql" },

    { "type": "template", "builtin": "rok-orm-repository-trait",
      "props": { "ref": "props" },
      "output": "src/domain/{{ model_snake }}/repository.rs" },

    { "type": "template", "builtin": "rok-orm-pg-repository",
      "props": { "ref": "props" },
      "output": "src/infrastructure/repositories/{{ model_snake }}_repo.rs" },

    { "type": "template", "builtin": "rok-orm-mock-repository",
      "props": { "ref": "props" },
      "output": "src/domain/{{ model_snake }}/mock_repository.rs" },

    { "type": "template", "builtin": "rok-orm-service-stub",
      "props": { "ref": "props" },
      "output": "src/domain/{{ model_snake }}/service.rs" },

    { "type": "template", "builtin": "rok-orm-axum-handler",
      "props": { "ref": "props" },
      "output": "src/api/v1/{{ model_snake }}/handlers.rs" },

    { "type": "template", "builtin": "rok-orm-dto",
      "props": { "ref": "props" },
      "output": "src/api/v1/{{ model_snake }}/dto.rs" },

    { "type": "bash", "cmd": "cargo fmt" },
    { "type": "bash", "cmd": "cargo check --quiet", "id": "check" },
    { "type": "bash", "cmd": "cargo test --quiet",  "id": "test" },

    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": "check" },
      "then": [
        { "type": "restore", "snapshot_id": "pre-scaffold" }
      ]
    }
  ]
}
```

---

## 12. Built-in Templates Reference

These are rok built-in templates prefixed `rok-orm-*`. Use them with `"builtin": "rok-orm-*"` in any rok payload.

| Template | Output | Description |
|---|---|---|
| `rok-orm-model` | `model.rs` | Full `#[derive(Model)]` struct |
| `rok-orm-migration` | `.sql` | `CREATE TABLE` with indexes |
| `rok-orm-migration-add-column` | `.sql` | `ALTER TABLE ADD COLUMN` |
| `rok-orm-migration-remove-column` | `.sql` | `ALTER TABLE DROP COLUMN` |
| `rok-orm-migration-rename-table` | `.sql` | `ALTER TABLE RENAME TO` |
| `rok-orm-migration-rename-column` | `.sql` | `ALTER TABLE RENAME COLUMN` |
| `rok-orm-repository-trait` | `repository.rs` | `trait PostRepository` |
| `rok-orm-pg-repository` | `post_repo.rs` | `PgPostRepository` impl |
| `rok-orm-mock-repository` | `mock_repository.rs` | `MockPostRepository` |
| `rok-orm-queries` | `queries.rs` | Typed `impl Post` query methods |
| `rok-orm-service-stub` | `service.rs` | Service struct with dependency injection |
| `rok-orm-axum-handler` | `handlers.rs` | Axum CRUD handlers |
| `rok-orm-dto` | `dto.rs` | `CreateDto`, `UpdateDto`, `ResponseDto` |
| `rok-orm-seeder` | `seeder.rs` | DB seeder struct |

### Template Props Schema

All templates accept the same base props, extended per template:

```json
{
  "name":        "Post",           // PascalCase model name (required)
  "model_snake": "post",           // snake_case (auto-derived if omitted)
  "table":       "posts",          // DB table name (required)
  "soft_delete": false,            // enable soft delete
  "timestamps":  true,             // auto timestamps
  "pool_type":   "PgPool",         // sqlx pool type
  "error_type":  "OrmError",       // error type
  "derives":     ["Debug","Clone","Serialize","Deserialize"],
  "columns":     [],               // column definitions
  "relations":   [],               // relation definitions
  "indexes":     [],               // index definitions (migration only)
  "queries":     [],               // query method definitions (queries only)
  "driver":      "postgres"        // migration SQL dialect
}
```

---

## 13. Config File (.rokorm)

Place `.rokorm` (TOML) at the project root to set defaults for all `rok orm` commands:

```toml
# .rokorm — rok-orm project configuration

[defaults]
pool_type   = "PgPool"
error_type  = "OrmError"
soft_delete = false
timestamps  = true
driver      = "postgres"
derives     = ["Debug", "Clone", "Serialize", "Deserialize"]

[paths]
domain       = "src/domain"
infra        = "src/infrastructure/repositories"
api          = "src/api/v1"
migrations   = "migrations"

[naming]
model_suffix  = ""            # e.g. "Model" → UserModel
repo_suffix   = "Repository"  # e.g. "Repository" → UserRepository
handler_file  = "handlers"    # handlers.rs
dto_file      = "dto"         # dto.rs

[database]
url_env = "DATABASE_URL"      # env var to read for DB URL

[format]
auto_fmt    = true             # run cargo fmt after generation
auto_check  = true             # run cargo check after generation
```

---

## 14. Full Workflow Examples

### Example 1 — Greenfield Model (Agent Workflow)

```bash
# Step 1: write schema to file
cat > post.schema.json << 'EOF'
{
  "name": "Post",
  "table": "posts",
  "soft_delete": true,
  "timestamps": true,
  "columns": [
    { "name": "id",        "type": "Uuid",    "primary": true },
    { "name": "user_id",   "type": "Uuid" },
    { "name": "title",     "type": "String" },
    { "name": "published", "type": "bool",    "default": "false" }
  ],
  "relations": [
    { "kind": "belongs_to", "model": "User", "fk": "user_id" }
  ]
}
EOF

# Step 2: full scaffold
rok orm scaffold full --from post.schema.json --migrate

# Step 3: run migrations
rok orm migrate run

# Step 4: validate sync
rok orm validate sync
```

---

### Example 2 — Add Column to Existing Model (Agent Workflow)

```json
{
  "name": "add-slug-to-posts",
  "steps": [
    { "type": "snapshot", "path": ".", "snapshot_id": "pre-add-slug" },
    {
      "type": "patch",
      "path": "src/domain/post/model.rs",
      "edits": [
        {
          "find": "pub title: String,",
          "replace": "pub title: String,\n    pub slug: String,"
        }
      ]
    },
    {
      "type": "template",
      "builtin": "rok-orm-migration-add-column",
      "props": {
        "table": "posts",
        "column": "slug",
        "pg_type": "TEXT",
        "nullable": false,
        "unique": true,
        "after": "title"
      },
      "output": "migrations/{{ timestamp }}_add_slug_to_posts.sql"
    },
    { "type": "bash", "cmd": "cargo fmt -- src/domain/post/model.rs" },
    { "type": "bash", "cmd": "cargo check --quiet", "id": "check" },
    { "type": "bash", "cmd": "sqlx migrate run" },
    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": "check" },
      "then": [{ "type": "restore", "snapshot_id": "pre-add-slug" }]
    }
  ]
}
```

---

### Example 3 — Rename Model (Agent Workflow)

```bash
rok orm refactor rename-model \
  --from BlogPost \
  --to Article \
  --path src/ \
  --rename-file \
  --rename-table \
  --migrate
```

---

### Example 4 — Inspect Before Generating

```bash
# Check what's in the DB before generating a model
rok orm inspect db --table users
rok orm inspect db --diff src/domain/user/model.rs

# Validate schema before generating
rok orm validate schema user.schema.json --strict

# Then generate
rok orm scaffold full --from user.schema.json
```

---

## 15. Agent Golden Rules for rok-orm

These rules extend the rok Golden Rules specifically for ORM operations.

### ORM Rule 1 — Always Snapshot Before Any Model Change

```json
{ "type": "snapshot", "path": ".", "snapshot_id": "pre-orm-change" }
```

Never modify a model, add a column, rename a table, or run a migration without a snapshot first.

---

### ORM Rule 2 — Always Validate After Generation

Every scaffold or generate payload MUST end with:

```json
{ "type": "bash", "cmd": "cargo check --quiet" }
```

Optionally also:

```json
{ "type": "bash", "cmd": "cargo test --quiet" }
```

---

### ORM Rule 3 — One JSON for Model + Migration + Repo

Never generate a model in one payload and a migration in a separate manual step. One payload generates the full slice:

```
model.rs → migration.sql → repository.rs → mock_repository.rs
```

---

### ORM Rule 4 — Never Manually Edit Generated Files

Use `rok orm model add-column`, `rok orm refactor rename-column`, or `rok orm relation add` instead of directly patching model files. These commands keep the JSON schema, Rust struct, migration, and DB in sync.

Exception: business logic hooks (`before_save`, `after_save`) and custom `impl` blocks are user territory and are never overwritten by rok.

---

### ORM Rule 5 — Validate Sync Before Release

Before any release or deploy, always run:

```bash
rok orm validate sync --database-url $DATABASE_URL --path src/domain/
```

If any model is out of sync, stop and generate the missing migration.

---

### ORM Rule 6 — Use `from-db` for Existing Tables

When adding rok-orm to an existing project with an existing database, never hand-write the struct. Always:

```bash
rok orm model from-db --table existing_table --database-url $DATABASE_URL
```

This guarantees the struct exactly matches the DB.

---

*rok-cli-orm — part of the rok ecosystem. Run One. Know All.*
