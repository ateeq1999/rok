# rok — Roadmap

> **Run One. Know All.** — One JSON. All Changes.

**Repo**: [github.com/ateeq1999/rok](https://github.com/ateeq1999/rok)

---

## Current State

The workspace contains **16 crates** across two categories:

| Crate | Type | Version | Status |
|-------|------|---------|--------|
| `rok-cli` | binary + lib | 0.10.0 | published |
| `rok-utils` | lib | 0.1.0 | implemented, unpublished |
| `rok-config` | lib | 0.1.0 | implemented, unpublished |
| `rok-orm-core` | lib | 0.1.0 | implemented, unpublished |
| `rok-orm-macros` | proc-macro | 0.1.0 | implemented, unpublished |
| `rok-orm` | lib (facade) | 0.1.0 | implemented, unpublished |
| `rok-migrate` | lib | 0.1.0 | implemented, unpublished |
| `rok-http` | lib | 0.1.0 | implemented, unpublished |
| `rok-auth` | lib | 0.1.0 | implemented, unpublished |
| `rok-lint` | lib | 0.1.0 | implemented, unpublished |
| `rok-test` | lib | 0.1.0 | implemented, unpublished |
| `rok-generate` | lib | 0.1.0 | implemented, unpublished |
| `rok-deploy` | lib | 0.1.0 | implemented, unpublished |
| `rok-gen-model` | binary | 0.1.0 | implemented, unpublished |
| `rok-gen-api` | binary | 0.1.0 | implemented, unpublished |
| `rok-docs` | binary | 0.1.0 | implemented, unpublished |

---

## Milestone 1 — Initial Ecosystem Publish

**Goal**: All 15 new crates published to crates.io at v0.1.0.

### Acceptance gates (must pass before any publish)

```bash
./scripts/dev.sh gates
```

1. No uncommitted changes in the working tree
2. `cargo fmt --all -- --check` — zero formatting issues
3. `cargo clippy --workspace -- -D warnings` — zero warnings
4. `cargo test --workspace` — all unit and integration tests green
5. `cargo doc --workspace --no-deps` — all documentation compiles

### Publish order

Dependencies must be published before dependents.

```
rok-utils          (no workspace deps)
rok-config         (no workspace deps)
rok-orm-core       (no workspace deps)
rok-orm-macros     (no workspace deps)
rok-orm            (rok-orm-core, rok-orm-macros)
rok-migrate        (no workspace deps)
rok-http           (no workspace deps)
rok-auth           (no workspace deps)
rok-lint           (no workspace deps)
rok-test           (no workspace deps)
rok-generate       (no workspace deps)
rok-deploy         (no workspace deps)
rok-gen-model      (rok-generate)
rok-gen-api        (rok-generate)
rok-docs           (no workspace deps)
```

Run:

```bash
./scripts/publish.sh          # publish all in order
./scripts/publish.sh rok-utils  # publish a single crate
./scripts/publish.sh --dry-run  # validate without uploading
```

---

## Milestone 2 — v0.11.0 rok-cli ✅

Align rok-cli with the new ecosystem crates.

- [x] Replace internal string utilities with `rok-utils`
- [x] Replace internal config loading with `rok-config`
- [x] Add `rok generate model` / `rok generate api` subcommands (wrapping `rok-generate`)
- [x] Add `rok migrate` subcommand (wrapping `rok-migrate`)
- [x] Resolve existing dead-code warnings

---

## Milestone 3 — sqlx Integration ✅

Give `rok-orm` a real database backend.

- [x] Add `sqlx` to `rok-orm-core` behind a feature flag (`sqlx-postgres`, `sqlx-sqlite`)
- [x] Implement `Model::find`, `Model::all`, `Model::create`, `Model::delete` against a live pool
- [x] Implement `rok-migrate` execution (apply / rollback against a database)
- [x] Integration tests against a Postgres container (GitHub Actions `services:`)

---

## Milestone 4 — Docs Site ✅

- [x] Fill `docs/content/` via `rok-docs generate`
- [x] Deploy docs site to Vercel / Netlify on `main` push
- [x] Add per-crate API reference pages

---

## Milestone 5 — v1.0.0

Stable public APIs across all crates.

- [x] Audit all `pub` APIs — remove anything unstable or incomplete
- [x] Add `#[non_exhaustive]` where appropriate
- [ ] Publish `v1.0.0` across the workspace (requires crates.io token — run `./scripts/publish.sh`)
- [ ] Announce on crates.io, Reddit (`/r/rust`), Discord

---

## Milestone 6 — rok-orm Macro DSL

**Goal**: Collapse model definition, querying, and migration generation into a
single, ergonomic macro surface. Users should need zero boilerplate structs and
zero `impl` blocks for the common case.

---

### 6.1 — `model!` declarative schema macro

Replace the two-step `#[derive(Model, sqlx::FromRow)] + attribute soup` with a
single invocation that generates the struct, trait impls, and a `FromRow` derive
in one shot.

```rust
// Before
#[derive(Debug, Clone, Model, sqlx::FromRow)]
#[rok_orm(table = "articles")]
pub struct Article {
    #[rok_orm(primary_key)]
    pub id:         i64,
    pub title:      String,
    pub published:  bool,
    #[rok_orm(skip)]
    pub draft_notes: String,
}

// After
rok_orm::model! {
    Article {
        table:   "articles",
        pk:      id: i64,

        title:      String,
        published:  bool,

        #[skip]
        draft_notes: String,
    }
}
```

**What the macro generates:**
- `pub struct Article { … }` with `#[derive(Debug, Clone)]`
- `impl Model for Article { … }` (table, pk, columns)
- `impl sqlx::FromRow for Article` (or `#[derive(sqlx::FromRow)]`)
- Associated constant `Article::COLUMNS` (`&'static [&'static str]`)
- `Article::query()` already works via blanket `Model` impl

**Acceptance criteria:**
- [ ] `model!` macro defined in `rok-orm-macros`
- [ ] Generates identical output to equivalent `#[derive(Model)]` usage
- [ ] Struct visibility (`pub`, `pub(crate)`, private) configurable
- [ ] Optional `derives = [Serialize, Deserialize, PartialEq]` key for extra derives

---

### 6.2 — `query!` compile-time query builder

A declarative macro for building queries without chaining methods — readable at
a glance, validated at compile time where possible.

```rust
// Method-chain style (still supported)
let q = User::query()
    .where_eq("active", true)
    .or_where_eq("role", "admin")
    .order_by_desc("created_at")
    .limit(10);

// Macro style
let q = rok_orm::query!(User {
    where: active = true OR role = "admin",
    order: created_at DESC,
    limit: 10,
});
```

**What the macro generates:** Equivalent `QueryBuilder` method calls. No runtime
overhead — expands to exactly the same code as the chain.

**Acceptance criteria:**
- [ ] `query!` macro defined in `rok-orm-macros`
- [ ] Supports: `where`, `or_where`, `select`, `distinct`, `order`, `limit`, `offset`
- [ ] Column names checked against `Model::columns()` at compile time
- [ ] Degrades gracefully — unknown operators fall back to `where_raw`

---

### 6.3 — `migrate!` inline migration DSL

Define migrations directly in Rust source with the DDL embedded as string
literals — no separate `.sql` files needed for simple cases.

```rust
rok_orm::migrate! {
    version: 1,
    name:    "create_users",

    up: "
        CREATE TABLE users (
            id    BIGSERIAL PRIMARY KEY,
            name  TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );
    ",
    down: "DROP TABLE users;",
}
```

**What the macro generates:** A `static MIGRATION_0001: rok_migrate::Migration`
constant that can be collected with `migrations![]` and applied via the executor.

```rust
// Collect and apply
rok_orm::migrations![MIGRATION_0001, MIGRATION_0002, MIGRATION_0003];

Migrator::run(&pool, &ALL_MIGRATIONS).await?;
```

**Acceptance criteria:**
- [ ] `migrate!` macro emits a `Migration` static
- [ ] `migrations![…]` macro collects them into `&'static [Migration]`
- [ ] Version uniqueness validated at compile time (via const assertions)
- [ ] Compatible with file-based migrations from `rok-migrate::load_from_dir`

---

### 6.4 — `scope!` named query scopes

Attach reusable, named query filters to a model — eliminating repeated
`.where_eq("deleted_at", SqlValue::Null)` chains everywhere.

```rust
rok_orm::scopes! {
    User {
        active:  |q| q.where_eq("active", true),
        admins:  |q| q.where_eq("role", "admin"),
        recent:  |q| q.order_by_desc("created_at").limit(20),
    }
}

// Usage — scopes are free functions on the model
let users = User::active().recent().all(&pool).await?;
```

**What the macro generates:** `impl User { fn active() -> QueryBuilder<User> { … } … }`

**Acceptance criteria:**
- [ ] `scopes!` macro generates inherent methods returning `QueryBuilder<Self>`
- [ ] Scopes are composable (chainable)
- [ ] Zero-cost: each scope is just a function call into the builder chain

---

### 6.5 — Soft deletes via attribute

```rust
rok_orm::model! {
    Post {
        table:      "posts",
        soft_delete: deleted_at,   // enables soft-delete mode

        pk:      id: i64,
        title:   String,
        deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    }
}

// delete_by_pk now sets deleted_at = NOW() instead of DELETE
Post::delete_by_pk(&pool, 1i64).await?;

// all() and find_where() automatically add WHERE deleted_at IS NULL
Post::all(&pool).await?;             // only non-deleted rows

// opt-in to include deleted rows
Post::query().include_deleted().all(&pool).await?;
```

**Acceptance criteria:**
- [ ] `soft_delete` key in `model!` enables the behaviour
- [ ] `delete_by_pk` / `delete_where` issue `UPDATE … SET deleted_at = NOW()`
- [ ] `all()` / `find_by_pk()` / `find_where()` filter `WHERE deleted_at IS NULL`
- [ ] `QueryBuilder` gets `.include_deleted()` method to opt out of the filter

---

### 6.6 — Transactions

```rust
let mut tx = pool.begin().await?;

User::create_in(&mut tx, &[("name", "Alice".into())]).await?;
Post::create_in(&mut tx, &[("user_id", 1i64.into())]).await?;

tx.commit().await?;
```

**Acceptance criteria:**
- [ ] All `PgModel` CRUD methods have `_in(tx: &mut PgTransaction, …)` variants
- [ ] `executor` module exposes matching `fetch_all_in`, `insert_in`, `update_in`, `delete_in`
- [ ] Rollback on drop (standard sqlx behaviour preserved)

---

### 6.7 — Query logging

```rust
rok-orm = { version = "0.1", features = ["postgres", "tracing"] }
```

When the `tracing` feature is enabled every generated SQL statement is emitted
at `DEBUG` level via the `tracing` crate:

```
DEBUG rok_orm: SELECT * FROM users WHERE active = $1 [true] (3.2ms)
```

**Acceptance criteria:**
- [ ] `tracing` feature added to `rok-orm` and `rok-orm-core`
- [ ] SQL + params + duration logged after each executor call
- [ ] Zero overhead when feature is disabled (conditional compilation)

---

- **Patch** (`0.1.x`): bug fixes, doc improvements, no API change
- **Minor** (`0.x.0`): additive API changes, new optional features
- **Major** (`x.0.0`): breaking changes, require RFC + deprecation period

Every release must pass the acceptance gates in Milestone 1.
Each published version is tagged `<crate>-v<version>` in git (e.g. `rok-utils-v0.1.0`).
