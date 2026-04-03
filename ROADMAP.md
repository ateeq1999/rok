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

## Release Policy

- **Patch** (`0.1.x`): bug fixes, doc improvements, no API change
- **Minor** (`0.x.0`): additive API changes, new optional features
- **Major** (`x.0.0`): breaking changes, require RFC + deprecation period

Every release must pass the acceptance gates in Milestone 1.
Each published version is tagged `<crate>-v<version>` in git (e.g. `rok-utils-v0.1.0`).
