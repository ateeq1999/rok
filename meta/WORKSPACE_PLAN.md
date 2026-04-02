# rok Workspace Implementation Plan

> Tracks the phased build-out of the rok crate ecosystem.

---

## Workspace Structure

```
rok/
├── Cargo.toml              # Workspace root (resolver = "2")
├── README.md
├── LICENSE
├── ECOSYSTEM.md
├── ROADMAP.md
├── WORKSPACE_PLAN.md       # this file
└── crates/
    ├── rok-cli/            # ✅ v0.10.0  — binary (rok)
    ├── rok-utils/          # ✅ v0.1.0  — Phase 1
    ├── rok-config/         # ✅ v0.1.0  — Phase 1
    ├── rok-http/           # ✅ v0.1.0  — Phase 2
    └── rok-auth/           # ✅ v0.1.0  — Phase 2
```

---

## Phase 1 — Foundation ✅

### rok-utils v0.1.0

Shared utilities for all rok crates.

| Module | API |
|--------|-----|
| `fs` | `read_to_string`, `read_bytes`, `write_atomic`, `ensure_dir`, `is_file`, `is_dir` |
| `path` | `normalize`, `stem_ext`, `with_extension` |
| `string` | `to_camel_case`, `to_pascal_case`, `to_snake_case`, `to_kebab_case`, `to_screaming_snake` |
| `result` | `OptionExt` — `.context(msg)` / `.with_context(|| msg)` on `Option<T>` |

```rust
use rok_utils::{fs, path, string};

fs::write_atomic("out.txt", "hello")?;
let s = fs::read_to_string("out.txt")?;
let norm = path::normalize("./foo/../bar");
let name = string::to_camel_case("hello_world"); // "helloWorld"
```

---

### rok-config v0.1.0

Multi-format configuration with env-var merging.

```rust
use rok_config::{Config, ConfigFormat};

let config = Config::builder()
    .file("config.toml", ConfigFormat::Toml)
    .env("APP_")          // APP_SERVER__PORT=9000 → "server.port"
    .build()?;

let port: u16 = config.get("server.port")?;
```

**Features**
- File loading: JSON, TOML, YAML (auto-detect from extension via `file_auto`)
- Flat dot-path key access (`"server.port"`, `"database.url"`)
- `env("PREFIX_")` merging — later sources override earlier ones
- `defaults(hashmap)` for hard-coded fallbacks
- Typed `get::<T>()` + untyped `get_str()`

---

## Phase 2 — HTTP & Auth ✅

### rok-http v0.1.0

Axum 0.8+ based web framework layer.

```rust
use rok_http::{App, Router};
use axum::{routing::get, Json};

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

#[tokio::main]
async fn main() {
    App::new()
        .router(Router::new().route("/health", get(health)))
        .serve("0.0.0.0:3000")
        .await
        .unwrap();
}
```

**Planned features**
- `App` builder with sensible defaults
- Middleware stack (CORS, tracing, compression)
- Typed `AppError` → HTTP response mapping
- `serve` / `serve_tls` helpers

---

### rok-auth v0.1.0

JWT + session authentication, OAuth providers, RBAC.

```rust
use rok_auth::{Auth, AuthConfig, Claims};

let auth = Auth::new(AuthConfig {
    secret: std::env::var("AUTH_SECRET")?,
    token_ttl: std::time::Duration::from_secs(3600),
    ..Default::default()
});

let token = auth.sign(&Claims::new("user-123", vec!["admin"]))?;
let claims = auth.verify(&token)?;
```

**Planned features**
- JWT sign / verify (HS256)
- `Claims` with subject, roles, expiry
- Session token generation (opaque)
- `AuthLayer` (Tower middleware) for Axum
- RBAC role checks

---

## Implementation Checklist

### Phase 1 ✅

- [x] Cargo workspace (`resolver = "2"`, shared deps)
- [x] `rok-cli` migrated to `crates/rok-cli/`
- [x] `rok-utils` — fs, path, string, result modules
- [x] `rok-config` — builder, JSON/TOML/YAML loading, env merging
- [x] Unit + integration tests for both crates
- [ ] Publish to crates.io

### Phase 2 ✅

- [x] `rok-http`
  - [x] `App` struct + builder
  - [x] `Router` re-export + helpers
  - [x] Middleware stack (CORS, tracing, compression, request-id)
  - [x] `AppError` → `IntoResponse`
- [x] `rok-auth`
  - [x] JWT sign / verify (HS256 via `jsonwebtoken`)
  - [x] `Claims` struct with RBAC role checks
  - [x] Session token generation (256-bit random hex)
  - [x] Password hashing / verification (Argon2id)
- [ ] Integration tests (HTTP)
- [ ] Documentation
- [ ] Publish to crates.io

---

## Testing Strategy

| Level | Scope |
|-------|-------|
| Unit tests (`#[cfg(test)]`) | Each module independently |
| Integration tests (`tests/`) | Cross-module and crate API surface |
| Doc-tests | Public API examples compile and run |

Target: 80 %+ line coverage per crate.

---

## Publishing Order

```
rok-utils → rok-config → rok-http → rok-auth → rok-cli
```

Each crate depends only on crates published before it in this list.

---

*See also: ECOSYSTEM.md for the full multi-crate vision.*
