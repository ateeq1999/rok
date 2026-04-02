# rok — Development Procedure

> This document is the single source of truth for how work gets done in this repository.
> It is written for **coding agents** first. Every section should be unambiguous enough
> that an AI can execute it without asking for clarification.

---

## 1. Mental Model

```
Issue (spec)  →  Branch  →  Code + Docs  →  Gates  →  PR  →  Merge  →  Release
```

- **Issues define work.** Nothing gets built without a closed issue that specifies exactly what to build.
- **Rustdoc is the documentation.** There is no separate docs folder to maintain. `///` and `//!` comments in source code are the API reference. `rok-docs generate` turns them into the site.
- **Gates are the quality bar.** If `./scripts/dev.sh gates` is green, the work is releasable.
- **Agents do all the coding.** Procedures are written so an agent can execute them top-to-bottom.

---

## 2. Repository Layout (quick reference)

```
crates/          library crates — published to crates.io
tools/           binary-only CLI tools — published to crates.io
docs/            TanStack Start docs site (JS/TS)
examples/        rok JSON task files demonstrating features
scripts/         dev.sh, publish.sh, install.sh, serve-docs.sh
meta/            internal guides (this file)
.github/         CI, issue templates, PR template
```

Workspace root: `Cargo.toml` lists all 16 members.
Acceptance gates: `./scripts/dev.sh gates`
Publishing: `./scripts/publish.sh [--dry-run] [crate-name]`

---

## 3. Maintenance Routine

Run these on a regular cadence, not just before releases.

### Weekly

```bash
# Update Cargo.lock to latest compatible versions
cargo update

# Re-run gates to confirm nothing broke
./scripts/dev.sh gates
```

### Before every release

```bash
# Security audit (install once: cargo install cargo-audit)
cargo audit

# Dry-run the full publish to catch packaging issues early
./scripts/publish.sh --dry-run
```

### Dependency version bumps

Edit the version in `[workspace.dependencies]` in the root `Cargo.toml`, then
re-run gates. Never bump a dependency in an individual crate's `Cargo.toml` if
it belongs in the workspace table.

---

## 4. Adding a Feature — Full Procedure

### Step 1 — Open an issue with a structured spec

Use the **Feature Spec** issue template (`.github/ISSUE_TEMPLATE/feature_spec.md`).
The spec must answer:

| Field | Description |
|-------|-------------|
| **Affected crates** | Which crates change. List by name. |
| **Public API** | New/changed function signatures (Rust code block). |
| **Behaviour** | What the feature does, inputs and outputs. |
| **Acceptance criteria** | Concrete, checkable statements. Use `[ ]` checkboxes. |
| **Out of scope** | What is explicitly NOT being built in this issue. |
| **Example usage** | A short working code snippet or JSON task. |

A spec is complete when an agent can implement it without asking follow-up questions.

### Step 2 — Create a branch

```bash
git checkout -b feat/<crate>/<short-name>
# Examples:
#   feat/rok-orm/soft-deletes
#   feat/rok-auth/refresh-tokens
#   fix/rok-migrate/duplicate-version-error
```

Branch naming:
- `feat/<crate>/<name>` — new capability
- `fix/<crate>/<name>` — bug fix
- `chore/<name>` — tooling, deps, CI
- `docs/<name>` — documentation only

### Step 3 — Read before writing

Before touching any file, read:
1. The affected crate's `src/lib.rs` (public API surface)
2. The affected crate's existing tests
3. The issue spec

### Step 4 — Implement

Rules:
- **One concern per commit.** Do not mix feature code with formatting fixes.
- **No dead code.** If something is unused, do not add it.
- **No feature flags for speculative future use.** Only add what the issue asks for.
- **Minimum viable implementation.** If the issue asks for `X`, implement `X` exactly.

### Step 5 — Write the documentation (in source)

Every public item added or changed must have a rustdoc comment.

```rust
// Crate overview — goes in src/lib.rs
//! rok-example — one-line description.
//!
//! ```rust
//! // A short working example that compiles as a doc-test.
//! ```

// Function doc
/// Returns the widget count for `owner`.
///
/// # Errors
///
/// Returns [`WidgetError::NotFound`] if `owner` does not exist.
pub fn widget_count(owner: &str) -> Result<usize, WidgetError> { ... }
```

Rules:
- Every `pub fn`, `pub struct`, `pub enum`, `pub trait` gets a `///` comment.
- Every crate's `lib.rs` gets a `//!` module doc with at least one doc-test.
- Doc-tests must compile and pass (`cargo test --doc`).
- Do not write a separate markdown file. The rustdoc comment IS the documentation.

### Step 6 — Run the acceptance gates

```bash
./scripts/dev.sh gates
```

All five gates must be green:

| Gate | Command | Pass condition |
|------|---------|----------------|
| Formatting | `cargo fmt --all -- --check` | Zero diffs |
| Lints | `cargo clippy --workspace -- -D warnings` | Zero warnings |
| Tests | `cargo test --workspace` | Zero failures |
| Docs | `cargo doc --workspace --no-deps` | Zero errors |
| Clean tree | `git status --porcelain` | Empty output |

If any gate fails, fix it before proceeding. Do not open a PR with failing gates.

### Step 7 — Commit

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

[optional body — explain WHY, not WHAT]
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`
Scope: the crate name (`rok-orm`, `rok-migrate`, etc.)

Examples:
```
feat(rok-orm): add soft-delete support via deleted_at column
fix(rok-migrate): reject duplicate migration versions at load time
docs(rok-auth): add doc-tests for Claims::has_role
chore: update serde to 1.0.230
```

### Step 8 — Open a pull request

Use the PR template (`.github/PULL_REQUEST_TEMPLATE.md`).

The PR title = the commit message of the most significant commit.
The PR body must include:
- Link to the issue (`Closes #N`)
- The exact `./scripts/dev.sh gates` output (copy-paste or CI link)
- Any decisions made that deviate from the spec (and why)

---

## 5. Documentation Strategy

The documentation pipeline is:

```
source code (//! and ///)
        │
        ▼
cargo doc --workspace --no-deps      ← generates rustdoc HTML
        │
        ▼
rok-docs generate --workspace Cargo.toml --output docs/content/crates
        │                              ← extracts //! comments → markdown
        ▼
docs/ TanStack Start site            ← renders markdown for the web
```

### Rules for agents

1. **Never create a separate `.md` file to document a crate's API.** Put it in the source.
2. **After adding public items, re-run** `cargo doc --workspace --no-deps` to confirm everything builds.
3. **Doc-tests are tests.** They run in CI. Keep them minimal but correct.
4. **`examples/` is for rok JSON tasks**, not Rust code. Rust examples go in `crates/<name>/examples/`.

### Regenerating the docs site content

```bash
./scripts/serve-docs.sh              # generate + serve locally
# or
rok-docs generate --workspace Cargo.toml --output docs/content/crates
```

This is done automatically in CI before deploying the docs site.

---

## 6. Adding a New Crate

1. Decide whether it belongs in `crates/` (library) or `tools/` (binary-only CLI tool).
2. Create `crates/<name>/Cargo.toml` using workspace inheritance:

```toml
[package]
name = "<name>"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
homepage.workspace = true
description = "One sentence."
keywords = ["rok"]
categories = ["development-tools"]

[dependencies]
# workspace deps only — no version numbers here
anyhow = { workspace = true }
```

3. Add the path to the `members` array in the root `Cargo.toml`.
4. Add the crate to the `PUBLISH_ORDER` array in `scripts/publish.sh` (after all its dependencies).
5. Add a row to the crate table in `ROADMAP.md`.
6. Write `src/lib.rs` with a `//!` doc comment and at least one doc-test.
7. Run `./scripts/dev.sh gates`.

---

## 7. Fixing a Bug

1. Open a bug issue — describe observed vs expected behaviour and affected crate.
2. Branch: `fix/<crate>/<short-name>`
3. Write a failing test that reproduces the bug **first**.
4. Fix the code until the test passes.
5. Run `./scripts/dev.sh gates`.
6. Commit: `fix(<crate>): <description>`
7. Open PR, link the issue.

---

## 8. Releasing a Crate

Only publish from a clean `main` branch after merging the relevant PR.

```bash
# 1. Make sure you are on main with a clean tree
git checkout main && git pull

# 2. Bump the version (workspace version or crate-specific)
#    Edit [workspace.package] version in Cargo.toml for a coordinated release,
#    or edit the individual crate's Cargo.toml for an independent bump.

# 3. Commit the version bump
git add Cargo.toml   # or the specific crate
git commit -m "chore: bump <crate> to v<version>"

# 4. Dry-run first
./scripts/publish.sh --dry-run

# 5. Publish
./scripts/publish.sh <crate-name>
# Or publish all that are behind:
./scripts/publish.sh
```

`publish.sh` runs all gates, skips crates already at the current version,
and tags each release `<crate>-v<version>`.

---

## 9. CI (GitHub Actions)

The workflow at `.github/workflows/ci.yml` runs on every push and PR:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace -- -D warnings`
- `cargo test --workspace`
- `cargo doc --workspace --no-deps`

A PR cannot be merged until CI is green. Do not bypass CI.

---

## 10. What agents must never do

- Merge to `main` directly — always use a PR.
- Publish a crate without `--dry-run` passing first.
- Add `#[allow(dead_code)]` or `#[allow(unused)]` as a shortcut.
- Create a documentation markdown file instead of rustdoc comments.
- Add code that is not covered by the issue spec ("while I'm here..." additions).
- Add dependencies that are not in `[workspace.dependencies]`.
- Use `unwrap()` in library code outside of tests.
