# rok Ecosystem — Master Plan

> **One JSON. All Changes. Every Tool.**

**Version**: v1.0.0 (Ecosystem Plan)  
**Last Updated**: April 1, 2026  
**Status**: Planning Phase

---

## 1. Vision

The rok ecosystem is a **comprehensive suite of Rust tools and libraries** that work together to eliminate repetitive development tasks. Each crate is:

- ✅ **Independently usable** — Install only what you need
- ✅ **Interoperable** — Crates compose seamlessly
- ✅ **Published separately** — Individual versioning on crates.io
- ✅ **rok-powered** — All use rok for self-development

### Core Philosophy

```
┌─────────────────────────────────────────────────────────┐
│                    rok CLI (Orchestrator)                │
│         The single entry point for all operations        │
└─────────────────────────────────────────────────────────┘
              │              │              │
              ▼              ▼              ▼
    ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
    │  rok-orm    │  │ rok-lint    │  │ rok-test    │
    │  (ORM)      │  │ (Linter)    │  │ (Testing)   │
    └─────────────┘  └─────────────┘  └─────────────┘
              │              │              │
              ▼              ▼              ▼
    ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
    │ rok-migrate │  │ rok-generate│  │ rok-deploy  │
    │ (Migrations)│  │ (Codegen)   │  │ (Deployment)│
    └─────────────┘  └─────────────┘  └─────────────┘
```

---

## 2. Workspace Architecture

### Root Structure

```
rok-workspace/
├── Cargo.toml                  # Workspace root
├── README.md
├── ECOSYSTEM.md               # This file
├── .github/
│   └── workflows/
│       └── ci.yml             # Unified CI for all crates
│
├── crates/
│   ├── rok-cli/               # Main CLI (existing) ✅ v0.10.0
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   ├── rok-orm/               # ORM layer 📋 v0.1.0
│   │   ├── Cargo.toml
│   │   ├── rok-orm-macros/    # Proc-macros
│   │   └── rok-orm-core/      # Core traits
│   │
│   ├── rok-http/              # Axum 0.8+ web framework 📋 v0.1.0
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   ├── rok-auth/              # Better-auth compatible auth 📋 v0.1.0
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   ├── rok-lint/              # Linting rules 📋 v0.1.0
│   ├── rok-test/              # Testing framework 📋 v0.1.0
│   ├── rok-migrate/           # Database migrations 📋 v0.1.0
│   ├── rok-generate/          # Code generation 📋 v0.1.0
│   ├── rok-deploy/            # Deployment automation 📋 v0.1.0
│   ├── rok-config/             # Configuration management 📋 v0.1.0
│   └── rok-utils/             # Shared utilities 📋 v0.1.0
│
└── tools/
    ├── rok-gen-model/         # Model generator CLI 📋 v0.1.0
    ├── rok-gen-api/           # API generator CLI 📋 v0.1.0
    └── rok-docs/               # Documentation generator 📋 v0.1.0
```

### Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "crates/rok-cli",
    "crates/rok-orm/rok-orm-core",
    "crates/rok-orm/rok-orm-macros",
    "crates/rok-orm",
    "crates/rok-http",
    "crates/rok-auth",
    "crates/rok-lint",
    "crates/rok-test",
    "crates/rok-migrate",
    "crates/rok-generate",
    "crates/rok-deploy",
    "crates/rok-config",
    "crates/rok-utils",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
repository = "https://github.com/ateeq1999/rok"
homepage = "https://github.com/ateeq1999/rok"
keywords = ["cli", "json", "automation", "ai"]
categories = ["command-line-utilities", "development-tools"]

[workspace.dependencies]
# Shared dependencies across all crates
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio-rustls"] }
async-trait = "0.1"
thiserror = "1"
anyhow = "1"
tracing = "0.1"
clap = { version = "4", features = ["derive"] }

# HTTP & Web (rok-http)
axum = { version = "0.8", features = ["macros", "http2"] }
tower = { version = "0.5", features = ["full"] }
tower-http = { version = "0.6", features = ["cors", "compression", "trace"] }
hyper = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
jsonwebtoken = "9"

# Auth (rok-auth)
argon2 = "0.5"
bcrypt = "0.15"
totp-rs = "5"
rand = "0.8"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# Code generation
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
darling = "0.20"
heck = "0.5"
tera = "1"

# Testing
assert_cmd = "2"
predicates = "3"
tokio-test = "0.4"
```

---

## 3. Crate Catalog

### 3.1 Core Crates (Foundation)

#### `rok-cli` ✅ v0.10.0

**Purpose**: Main CLI orchestrator  
**Install**: `cargo install rok-cli`  
**Dependencies**: None (standalone)

```toml
[package]
name = "rok-cli"
version = "0.10.0"
description = "Run One, Know All - Execute multi-step tasks from JSON"
```

**Key Features**:
- 26 step types
- Task system
- Caching + incremental mode
- Template engine
- Control flow (if/each/parallel)

---

#### `rok-utils` 📋 Planned v0.1.0

**Purpose**: Shared utilities for all rok crates  
**Install**: `cargo add rok-utils`  
**Dependencies**: None

```toml
[package]
name = "rok-utils"
version = "0.1.0"
description = "Shared utilities for the rok ecosystem"
```

**Exports**:
- `fs` — File operations
- `path` — Path utilities
- `string` — String manipulation
- `result` — Result extensions
- `async` — Async helpers

---

#### `rok-config` 📋 Planned v0.1.0

**Purpose**: Configuration management  
**Install**: `cargo add rok-config`  
**Dependencies**: `rok-utils`, `toml`, `serde_yaml`

```toml
[package]
name = "rok-config"
version = "0.1.0"
description = "Configuration management for rok ecosystem"
```

**Features**:
- Multi-format config (JSON, TOML, YAML)
- Environment variable merging
- Schema validation
- Hot reloading

---

### 3.2 Database Crates

#### `rok-orm` 📋 Planned v0.1.0

**Purpose**: Eloquent-inspired async ORM  
**Install**: `cargo add rok-orm`  
**Dependencies**: `sqlx`, `tokio`, `rok-orm-macros`, `rok-orm-core`

```toml
[package]
name = "rok-orm"
version = "0.1.0"
description = "Async ORM for Rust, powered by sqlx"
```

**Workspace**:
```
rok-orm/
├── Cargo.toml              # Facade crate
├── rok-orm-core/           # Traits, QueryBuilder
└── rok-orm-macros/         # Derive macros
```

**Key Features**:
- `#[derive(Model)]` macro
- QueryBuilder with Eloquent syntax
- Relations (HasMany, BelongsTo, ManyToMany)
- Migrations
- Soft deletes
- Pagination
- Transactions

**Example**:
```rust
use rok_orm::Model;

#[derive(Model)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

// Usage
let users = User::query()
    .where_eq("active", true)
    .order_by_desc("created_at")
    .limit(10)
    .all(&pool)
    .await?;
```

---

#### `rok-migrate` 📋 Planned v0.1.0

**Purpose**: Database migration runner  
**Install**: `cargo add rok-migrate`  
**Dependencies**: `sqlx`, `rok-orm-core`

```toml
[package]
name = "rok-migrate"
version = "0.1.0"
description = "Database migration runner for rok ecosystem"
```

**Features**:
- SQL migration files
- Rollback support
- Migration tracking table
- Seed data
- CLI integration

---

### 3.3 Development Tools

#### `rok-lint` 📋 Planned v0.1.0

**Purpose**: Custom linting rules  
**Install**: `cargo install rok-lint`  
**Dependencies**: `syn`, `quote`, `proc-macro2`

```toml
[package]
name = "rok-lint"
version = "0.1.0"
description = "Custom linting rules for Rust projects"
```

**Features**:
- Project-specific rules
- Auto-fix suggestions
- Integration with `cargo clippy`
- Custom rule plugins

---

#### `rok-test` 📋 Planned v0.1.0

**Purpose**: Testing framework  
**Install**: `cargo add rok-test --dev`  
**Dependencies**: `tokio`, `sqlx`, `assert_cmd`

```toml
[package]
name = "rok-test"
version = "0.1.0"
description = "Testing framework for rok ecosystem"
```

**Features**:
- Database test helpers
- Mock generators
- Integration test scaffolding
- Fixture management

---

#### `rok-generate` 📋 Planned v0.1.0

**Purpose**: Code generation  
**Install**: `cargo install rok-generate`  
**Dependencies**: `tera`, `rok-orm`

```toml
[package]
name = "rok-generate"
version = "0.1.0"
description = "Code generation for rok ecosystem"
```

**Features**:
- Model generators
- API scaffolding
- Repository pattern
- DTO generation

---

#### `rok-deploy` 📋 Planned v0.1.0

**Purpose**: Deployment automation  
**Install**: `cargo install rok-deploy`  
**Dependencies**: `tokio`, `serde_json`

```toml
[package]
name = "rok-deploy"
version = "0.1.0"
description = "Deployment automation for rok ecosystem"
```

**Features**:
- Docker builds
- Kubernetes manifests
- Cloud deployments (AWS, GCP, Azure)
- Rollback support

---

### 3.4 HTTP & Network

#### `rok-http` 📋 Planned v0.1.0

**Purpose**: Axum-based web framework with rok conventions  
**Install**: `cargo add rok-http`  
**Dependencies**: `axum 0.8+`, `tokio`, `tower`, `hyper`

```toml
[package]
name = "rok-http"
version = "0.1.0"
description = "Axum-based web framework with rok conventions"
```

**Features**:
- **Axum 0.8+** — Latest version with Rustls, HTTP/2
- **Starter Templates** — Production-ready project scaffolding
- **Middleware Stack** — CORS, compression, request ID, timing
- **Error Handling** — Unified error types with JSON responses
- **Request Validation** — Type-safe request parsing
- **Response Helpers** — Paginated responses, streaming

**Starter Templates**:
```bash
rok-http new my-api --template minimal
rok-http new my-api --template full --with-auth --with-db
rok-http new my-api --template microservice
```

**Example**:
```rust
use rok_http::{App, Router, get, post};

#[get("/users/:id")]
async fn get_user(Path(id): Path<Uuid>) -> Result<Json<User>> {
    let user = User::find(id, &pool).await?;
    Ok(Json(user))
}

#[post("/users")]
async fn create_user(Json(req): Json<CreateUserRequest>) -> Result<Json<User>> {
    let user = User::create(req, &pool).await?;
    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    App::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user))
        .serve("0.0.0.0:3000")
        .await
        .unwrap();
}
```

**Project Structure** (full template):
```
my-api/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── health.rs
│   │   └── users.rs
│   ├── handlers/
│   │   └── mod.rs
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── logging.rs
│   └── extractors/
│       └── mod.rs
├── migrations/
├── tests/
└── Cargo.toml
```

---

#### `rok-auth` 📋 Planned v0.1.0

**Purpose**: Better-auth compatible authentication system  
**Install**: `cargo add rok-auth`  
**Dependencies**: `rok-http`, `jsonwebtoken`, `argon2`, `sqlx`

```toml
[package]
name = "rok-auth"
version = "0.1.0"
description = "Better-auth compatible authentication for rok ecosystem"
```

**Features**:
- **Better-Auth Compatible** — Drop-in replacement for better-auth
- **Session Management** — Database + JWT sessions
- **OAuth Providers** — Google, GitHub, Discord, Microsoft
- **Multi-Factor Auth** — TOTP, SMS, Email codes
- **Password Reset** — Secure token-based flows
- **Account Linking** — Merge multiple auth providers
- **Role-Based Access** — RBAC with permissions
- **Rate Limiting** — Built-in brute force protection

**Example**:
```rust
use rok_auth::{Auth, AuthConfig, Provider};

#[tokio::main]
async fn main() {
    let auth = Auth::new(AuthConfig {
        secret: std::env::var("AUTH_SECRET").unwrap(),
        session: SessionConfig {
            expires_in: Duration::days(7),
            ..Default::default()
        },
        providers: vec![
            Provider::google(client_id, client_secret),
            Provider::github(client_id, client_secret),
        ],
        ..Default::default()
    });

    App::new()
        .route("/auth/signin", post(signin))
        .route("/auth/signup", post(signup))
        .route("/auth/signout", post(signout))
        .route("/auth/oauth/:provider", get(oauth_callback))
        .route("/protected", get(protected_route))
        .layer(auth.middleware())
        .serve("0.0.0.0:3000")
        .await
        .unwrap();
}

// Protected route handler
async fn protected_route(
    user: AuthUser,  // Extractor automatically validates session
) -> Result<Json<UserProfile>> {
    Ok(Json(user.profile))
}
```

**Database Schema** (auto-generated):
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    password_hash VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    provider_account_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    UNIQUE(provider, provider_account_id)
);

CREATE TABLE verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identifier VARCHAR(255) NOT NULL,
    value VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**rok Integration**:
```json
{
  "name": "setup-auth",
  "steps": [
    {
      "type": "template",
      "builtin": "rok-auth-init",
      "props": {
        "providers": ["google", "github"],
        "session_type": "database",
        "mfa_enabled": true
      }
    },
    {
      "type": "bash",
      "cmd": "cargo add rok-auth rok-http"
    },
    {
      "type": "bash",
      "cmd": "rok migrate run"
    }
  ]
}
```

---

### 3.5 CLI Tools (Binaries)

#### `rok-gen-model` 📋 Planned v0.1.0

**Purpose**: Generate models from database  
**Install**: `cargo install rok-gen-model`  
**Dependencies**: `rok-orm`, `sqlx`, `clap`

**Commands**:
```bash
rok-gen-model from-db --url postgres://localhost/mydb
rok-gen-model from-json --file schema.json
rok-gen-model migrate --up
```

---

#### `rok-gen-api` 📋 Planned v0.1.0

**Purpose**: Generate API endpoints  
**Install**: `cargo install rok-gen-api`  
**Dependencies**: `rok-generate`, `axum`

**Commands**:
```bash
rok-gen-api scaffold User --crud
rok-gen-api handler CreateUser
rok-gen-api dto UserRequest
```

---

#### `rok-docs` 📋 Planned v0.1.0

**Purpose**: Generate documentation  
**Install**: `cargo install rok-docs`  
**Dependencies**: `pulldown-cmark`, `tera`

**Commands**:
```bash
rok-docs generate
rok-docs serve
rok-docs deploy
```

---

## 4. Dependency Graph

```
┌────────────────────────────────────────────────────────────┐
│                     rok-cli (v0.10.0)                      │
│                    (Orchestrator)                          │
└────────────────────────────────────────────────────────────┘
         │                    │                    │
         │ uses               │ uses               │ uses
         ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   rok-orm       │  │   rok-generate  │  │   rok-deploy    │
│   (v0.1.0)      │  │   (v0.1.0)      │  │   (v0.1.0)      │
└─────────────────┘  └─────────────────┘  └─────────────────┘
         │                    │                    │
         │ depends on         │ depends on         │ depends on
         ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ rok-orm-core    │  │   rok-utils     │  │   rok-config    │
│ rok-orm-macros  │  │   rok-config    │  │   rok-utils     │
└─────────────────┘  └─────────────────┘  └─────────────────┘
         │
         │ uses
         ▼
┌─────────────────┐
│   sqlx          │
│   tokio         │
└─────────────────┘

┌────────────────────────────────────────────────────────────┐
│                  rok-http (v0.1.0)                         │
│                  (Axum 0.8+ Base)                          │
└────────────────────────────────────────────────────────────┘
         │                    │
         │ uses               │ uses
         ▼                    ▼
┌─────────────────┐  ┌─────────────────┐
│   rok-auth      │  │   rok-orm       │
│   (v0.1.0)      │  │   (v0.1.0)      │
└─────────────────┘  └─────────────────┘
         │                    │
         │ depends on         │ depends on
         ▼                    ▼
┌─────────────────┐  ┌─────────────────┐
│ jsonwebtoken    │  │ rok-orm-core    │
│ argon2          │  │ sqlx            │
│ totp-rs         │  │                 │
└─────────────────┘  └─────────────────┘
```

---

## 5. Publishing Strategy

### Individual Versioning

Each crate has **independent versioning**:

| Crate | Current | Next | Breaking Changes |
|-------|---------|------|------------------|
| `rok-cli` | v0.10.0 | v0.11.0 | Minor |
| `rok-http` | — | v0.1.0 | N/A (new) |
| `rok-auth` | — | v0.1.0 | N/A (new) |
| `rok-orm` | — | v0.1.0 | N/A (new) |
| `rok-utils` | — | v0.1.0 | N/A (new) |
| `rok-config` | — | v0.1.0 | N/A (new) |

### Publishing Order

```bash
# 1. Foundation crates first
cargo publish -p rok-utils
cargo publish -p rok-config

# 2. HTTP layer
cargo publish -p rok-http

# 3. Auth (depends on rok-http)
cargo publish -p rok-auth

# 4. ORM layer
cargo publish -p rok-orm-core
cargo publish -p rok-orm-macros
cargo publish -p rok-orm

# 5. Tool crates
cargo publish -p rok-lint
cargo publish -p rok-test
cargo publish -p rok-generate
cargo publish -p rok-deploy

# 6. CLI tools
cargo publish -p rok-cli
cargo publish -p rok-gen-model
cargo publish -p rok-gen-api
cargo publish -p rok-docs
```

### Cross-Crate Dependencies

Use workspace paths for development, version numbers for publishing:

```toml
# Development (workspace)
[dependencies]
rok-utils = { path = "../rok-utils" }

# Publishing (crates.io)
[dependencies]
rok-utils = "0.1"
```

---

## 6. Installation Patterns

### Install Single Crate

```bash
# Just the ORM
cargo add rok-orm

# Just testing helpers
cargo add rok-test --dev

# Just the CLI
cargo install rok-cli
```

### Install Full Suite

```bash
# All development tools
cargo install rok-cli rok-gen-model rok-gen-api rok-docs

# All runtime dependencies
cargo add rok-orm rok-config rok-utils
```

### Use rok to Install rok

```json
{
  "steps": [
    { "type": "bash", "cmd": "cargo install rok-cli --force" },
    { "type": "bash", "cmd": "cargo install rok-gen-model" },
    { "type": "bash", "cmd": "cargo install rok-gen-api" }
  ]
}
```

---

## 7. Self-Evolution Model

The rok ecosystem **builds itself** using rok.

### Example: Adding a New Crate

```json
{
  "name": "add-rok-lint",
  "steps": [
    { "type": "bash", "cmd": "mkdir -p crates/rok-lint/src" },
    
    { "type": "write", "path": "crates/rok-lint/Cargo.toml", "content": "[package]\nname = \"rok-lint\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nsyn = { version = \"2\", features = [\"full\"] }\nquote = \"1\"\nproc-macro2 = \"1\"\n" },
    
    { "type": "write", "path": "crates/rok-lint/src/lib.rs", "content": "pub mod rules;\npub mod runner;\n" },
    
    { "type": "patch", "path": "Cargo.toml", "edits": [{"find": "members = [", "replace": "members = [\n    \"crates/rok-lint\","}] },
    
    { "type": "bash", "cmd": "cargo check -p rok-lint" },
    { "type": "bash", "cmd": "cargo test -p rok-lint" },
    
    { "type": "git", "op": "add", "args": ["crates/rok-lint"] },
    { "type": "git", "op": "commit", "args": ["-m", "feat: Add rok-lint crate"] }
  ]
}
```

---

## 8. CI/CD Pipeline

### GitHub Actions Workflow

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test-workspace:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
      
      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
      
      - name: Format check
        run: cargo fmt --all -- --check
      
      - name: Clippy (workspace)
        run: cargo clippy --workspace -- -D warnings
      
      - name: Test (workspace)
        run: cargo test --workspace
      
      - name: Build all crates
        run: cargo build --workspace --release

  publish-crates:
    needs: test-workspace
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
      
      - name: Publish changed crates
        run: |
          cargo login ${{ secrets.CRATES_IO_TOKEN }}
          ./scripts/publish-workspace.sh
```

---

## 9. Implementation Roadmap

### Phase 1 — Foundation (Weeks 1-2)

- [ ] Set up workspace structure
- [ ] Create `rok-utils` crate
- [ ] Create `rok-config` crate
- [ ] Migrate shared code from `rok-cli`

### Phase 2 — ORM Core (Weeks 3-6)

- [ ] Implement `rok-orm-core` traits
- [ ] Build `rok-orm-macros` proc-macro
- [ ] Create facade `rok-orm` crate
- [ ] Add QueryBuilder
- [ ] Implement relations

### Phase 3 — Development Tools (Weeks 7-10)

- [ ] Build `rok-lint`
- [ ] Build `rok-test`
- [ ] Build `rok-generate`
- [ ] Create CLI tools (`rok-gen-model`, `rok-gen-api`)

### Phase 4 — Deployment (Weeks 11-12)

- [ ] Build `rok-deploy`
- [ ] Build `rok-http`
- [ ] Build `rok-docs`
- [ ] Integration testing

### Phase 5 — Polish & Release (Weeks 13-14)

- [ ] Documentation website
- [ ] Example projects
- [ ] Performance benchmarks
- [ ] Publish all crates

---

## 10. Naming Conventions

### Crate Names

- Lowercase with hyphens: `rok-orm`, `rok-utils`
- Prefix with `rok-` for ecosystem crates
- Suffix with purpose: `-core`, `-macros`, `-cli`

### Version Numbers

- Follow semver: `MAJOR.MINOR.PATCH`
- `0.x.y` for initial development
- `1.0.0` when API stabilizes

### Feature Flags

```toml
[features]
default = ["postgres"]
postgres = ["sqlx/postgres"]
mysql = ["sqlx/mysql"]
sqlite = ["sqlx/sqlite"]
axum = ["dep:axum"]
```

---

## 11. Documentation Strategy

### Per-Crate Docs

- README.md with examples
- rustdoc comments on all public APIs
- Integration test examples

### Central Documentation

- **Website**: `rok.dev` (or `rok-cli.dev`)
- **Guides**: Installation, tutorials, recipes
- **API Reference**: Auto-generated from rustdoc
- **Blog**: Release notes, announcements

### Example Projects

```
examples/
├── basic-orm/           # Simple CRUD app
├── api-with-auth/       # Full API with JWT
├── microservice/        # Multi-service setup
└── workspace-demo/      # Workspace usage example
```

---

## 12. Community & Contribution

### Contribution Guidelines

1. Fork the workspace
2. Create feature branch
3. Add tests
4. Run `cargo clippy --workspace`
5. Submit PR

### Crate Maintenance

Each crate has a **maintainer**:

| Crate | Maintainer | Status |
|-------|------------|--------|
| `rok-cli` | @ateeq1999 | ✅ Active |
| `rok-orm` | TBD | 📋 Seeking |
| `rok-utils` | TBD | 📋 Seeking |

### RFC Process

For major changes:

1. Open GitHub Issue
2. Write RFC document
3. Community discussion (1 week)
4. Implementation
5. Merge + Publish

---

## 13. Success Metrics

| Metric | Target | Timeline |
|--------|--------|----------|
| Crates Published | 12 | 3 months |
| Total Downloads | 10K | 6 months |
| GitHub Stars | 500 | 6 months |
| Contributors | 10 | 6 months |
| Documentation Coverage | 90% | 3 months |
| Test Coverage | 80% | 3 months |

---

## 14. Future Extensions

### Potential Crates (v2.0)

- `rok-cache` — Redis/Memcached integration
- `rok-queue` — Job queue (RabbitMQ, SQS)
- `rok-auth` — Authentication helpers
- `rok-graphql` — GraphQL generation
- `rok-websocket` — WebSocket utilities
- `rok-metrics` — Prometheus/OpenTelemetry
- `rok-logging` — Structured logging
- `rok-validation` — Form validation

### Platform Extensions

- **rok cloud** — Hosted execution
- **rok studio** — Visual task builder
- **rok marketplace** — Template sharing
- **rok academy** — Learning platform

---

## 15. Quick Reference

### Install All Tools

```bash
cargo install rok-cli rok-gen-model rok-gen-api rok-docs
```

### Add Runtime Dependencies

```bash
cargo add rok-orm rok-config rok-utils
cargo add rok-test --dev
```

### Build Workspace

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace
```

### Publish Crate

```bash
cargo publish -p rok-utils
cargo publish -p rok-orm
```

---

## Final Principle

> **Every crate stands alone. Together, they form an ecosystem.**

---

**Build Once. Use Everywhere.**

---

*For implementation details, see individual crate READMEs. For rok CLI usage, see [rok.md](rok.md).*
