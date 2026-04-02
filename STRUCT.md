# Project Structure Proposal

> A structural refactor to make the workspace easier to navigate, maintain, and grow.

---

## 1. Current Pain Points

### Root-level markdown clutter
Nine planning/design documents sit at the root alongside `README.md`:

```
AGENT.md            IMPROVEMENT_PLAN.md   rok-orm-cli.md
CHANGELOG.md        PLAN.md               rok-orm-plan.md
CONTRIBUTING.md     ROADMAP.md            rok.md
ECOSYSTEM.md        TODO.md
README.md           WORKSPACE_PLAN.md
```

First-time contributors see a wall of text. Most are stale planning docs that no longer reflect the code.

### Deeply-nested ORM sub-crates
`rok-orm-core` and `rok-orm-macros` live *inside* the `rok-orm` directory:

```
crates/rok-orm/
├── Cargo.toml       ← facade
├── rok-orm-core/    ← nested workspace member
└── rok-orm-macros/  ← nested workspace member
```

Every workspace member reference becomes `"crates/rok-orm/rok-orm-core"` — three levels deep, and IDEs show confusing duplicate `rok-orm` folders.

### Mixed example content
`crates/rok-cli/examples/` mixes two unrelated things:
- Rust binary examples (`docs_server.rs`, `completions/main.rs`)
- rok JSON task files (`generate-crud.json`, `ci-pipeline.json`)

### Stray files at root
- `logo.svg` — brand asset loose at root
- `migrations/0001_create_posts.sql` — accidentally generated, not owned by any crate
- `.rokrc.example` — should be near the docs it documents

### No clear separation of concerns
Looking at the root you cannot tell at a glance what is user-facing vs internal vs a sub-project.

---

## 2. Proposed Structure

```
rok/
│
├── Cargo.toml                  # Workspace root
├── Cargo.lock
├── README.md                   # User-facing: install + quick-start
├── CHANGELOG.md                # User-facing: version history
├── CONTRIBUTING.md             # User-facing: how to contribute
├── ROADMAP.md                  # User-facing: what is planned
├── .gitignore
├── .rokrc.example              # Keep at root — users copy this
│
├── .github/
│   ├── workflows/
│   │   └── ci.yml
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
│
├── crates/                     # Publishable library crates
│   ├── rok-cli/                # Main CLI (also a library)
│   ├── rok-utils/
│   ├── rok-config/
│   ├── rok-http/
│   ├── rok-auth/
│   ├── rok-lint/
│   ├── rok-test/
│   ├── rok-generate/
│   ├── rok-deploy/
│   ├── rok-migrate/
│   ├── rok-orm/                # Facade re-export only
│   ├── rok-orm-core/           # ← MOVED from crates/rok-orm/rok-orm-core
│   └── rok-orm-macros/         # ← MOVED from crates/rok-orm/rok-orm-macros
│
├── tools/                      # Standalone binary tools (not published as libs)
│   ├── rok-gen-model/
│   ├── rok-gen-api/
│   └── rok-docs/
│
├── docs/                       # TanStack Start documentation site (JS/TS)
│   ├── src/
│   ├── public/
│   │   └── brand/              # ← logo.svg, favicon, logos moved here
│   └── ...
│
├── examples/                   # End-to-end rok JSON task examples
│   ├── generate-crud.json      # ← MOVED from crates/rok-cli/examples/
│   ├── ci-pipeline.json
│   ├── batch-imports.json
│   ├── full-stack-feature.json
│   └── refactor-rename.json
│
├── scripts/                    # Dev & release helper scripts
│   ├── install.sh
│   ├── publish.sh
│   ├── dev.sh
│   └── completions.sh
│
└── meta/                       # Internal design docs (not user-facing)
    ├── ECOSYSTEM.md            # ← MOVED from root
    ├── WORKSPACE_PLAN.md       # ← MOVED from root
    ├── AGENT.md                # ← MOVED from root
    ├── rok.md                  # ← MOVED from root (AI agent guide)
    └── archive/                # Stale planning docs kept for reference
        ├── PLAN.md
        ├── IMPROVEMENT_PLAN.md
        ├── TODO.md
        ├── rok-orm-cli.md
        └── rok-orm-plan.md
```

---

## 3. Key Changes and Rationale

### 3.1 Flatten ORM sub-crates

**Before**
```toml
# Cargo.toml
members = [
    "crates/rok-orm/rok-orm-core",
    "crates/rok-orm/rok-orm-macros",
    "crates/rok-orm",
]
```

**After**
```toml
members = [
    "crates/rok-orm-core",
    "crates/rok-orm-macros",
    "crates/rok-orm",
]
```

- All three crates sit side-by-side in `crates/` — same pattern as every other crate
- Cross-crate `path` references become `../rok-orm-core` instead of `../rok-orm-core` (same depth)
- IDE project trees no longer show a confusing nested `rok-orm/rok-orm/…` path

### 3.2 Promote `examples/` to root

rok JSON task files demonstrate the tool to users. They belong at the workspace root (like `examples/` in any Rust workspace), not buried in a single crate's folder.

Rust binary examples (`docs_server.rs`, `completions/`) stay in `crates/rok-cli/examples/` where `cargo run --example` can find them.

### 3.3 Move brand assets into `docs/public/brand/`

`logo.svg` is consumed by the docs site, not the Rust code. It lives closer to where it is actually used and keeps the workspace root clean.

### 3.4 Separate user-facing from internal docs

| Root (user-facing) | `meta/` (internal) | `meta/archive/` (stale) |
|---|---|---|
| `README.md` | `ECOSYSTEM.md` | `PLAN.md` |
| `CHANGELOG.md` | `WORKSPACE_PLAN.md` | `IMPROVEMENT_PLAN.md` |
| `CONTRIBUTING.md` | `AGENT.md` | `TODO.md` |
| `ROADMAP.md` | `rok.md` | `rok-orm-cli.md` |
| | | `rok-orm-plan.md` |

The rule: if a new contributor needs to read it, it stays at root. If it's an internal design decision, it goes to `meta/`.

### 3.5 Delete the stray `migrations/` at root

`migrations/0001_create_posts.sql` was generated accidentally by the `rok-gen-api scaffold` test run. Migration files belong inside the project that owns them (typically an `examples/` project or an app crate), not loose at the workspace root.

---

## 4. Migration Checklist

```
[ ] Move  crates/rok-orm/rok-orm-core/   → crates/rok-orm-core/
[ ] Move  crates/rok-orm/rok-orm-macros/ → crates/rok-orm-macros/
[ ] Update  Cargo.toml  workspace members (3 paths)
[ ] Update  crates/rok-orm/Cargo.toml    path deps
[ ] Update  crates/rok-migrate/Cargo.toml  (if it paths to rok-orm-core)

[ ] Move  crates/rok-cli/examples/*.json  → examples/
[ ] Leave crates/rok-cli/examples/*.rs    in place

[ ] Move  logo.svg        → docs/public/brand/logo.svg
[ ] Move  ECOSYSTEM.md    → meta/ECOSYSTEM.md
[ ] Move  WORKSPACE_PLAN.md → meta/WORKSPACE_PLAN.md
[ ] Move  AGENT.md        → meta/AGENT.md
[ ] Move  rok.md          → meta/rok.md
[ ] Move  PLAN.md         → meta/archive/PLAN.md
[ ] Move  IMPROVEMENT_PLAN.md → meta/archive/
[ ] Move  TODO.md         → meta/archive/
[ ] Move  rok-orm-cli.md  → meta/archive/
[ ] Move  rok-orm-plan.md → meta/archive/

[ ] Delete  migrations/0001_create_posts.sql  (test artefact)

[ ] Update  docs/ any hardcoded paths to logo.svg
[ ] Update  .github/favicon.svg  reference if pointing to root logo
[ ] Update  README.md  links to moved docs
```

---

## 5. What Does Not Change

- `crates/` — the top-level directory and every crate name stay the same
- `tools/` — the CLI tools directory stays as-is
- `docs/` — TanStack Start site stays at root
- `scripts/` — stays at root
- `.github/` — stays at root
- All crate `src/` trees — zero code changes required

The refactor is purely a file-move operation. Cargo workspace paths, crate names, and `pub` APIs are unaffected (except for flattening the ORM sub-crates, which only touches three `path = "…"` strings).

---

## 6. Result at a Glance

```
rok/
├── Cargo.toml / Cargo.lock      ← workspace
├── README.md                    ← start here
├── CHANGELOG.md / CONTRIBUTING.md / ROADMAP.md
├── .gitignore / .rokrc.example
├── .github/                     ← CI + templates
├── crates/          (13 items)  ← library crates
├── tools/           (3 items)   ← CLI binaries
├── docs/                        ← JS docs site
├── examples/        (5 items)   ← rok task JSON files
├── scripts/         (4 items)   ← shell helpers
└── meta/                        ← internal planning
    └── archive/                 ← stale docs
```

Nine items at the root (plus dotfiles). Every directory has a single, obvious responsibility.
