# Contributing to rok

## Quick start

```bash
git clone https://github.com/ateeq1999/rok.git
cd rok
./scripts/dev.sh gates   # all checks must be green before any PR
```

## Full procedure

See **[meta/DEVELOPMENT.md](meta/DEVELOPMENT.md)** — the definitive guide covering:

- Maintenance routine (weekly, pre-release)
- Adding a feature end-to-end
- Documentation strategy (rustdoc as source of truth)
- Adding a new crate
- Fixing a bug
- Releasing a crate
- What agents must never do

## At a glance

| Task | Command |
|------|---------|
| Run all quality gates | `./scripts/dev.sh gates` |
| Fix formatting | `./scripts/dev.sh fix` |
| Run tests only | `./scripts/dev.sh test` |
| Publish one crate (dry run) | `./scripts/publish.sh --dry-run rok-utils` |
| Publish one crate | `./scripts/publish.sh rok-utils` |
| Serve docs locally | `./scripts/serve-docs.sh` |

## Commit style

[Conventional Commits](https://www.conventionalcommits.org/) with crate scope:

```
feat(rok-orm): add soft-delete support
fix(rok-migrate): reject duplicate versions at load time
docs(rok-auth): add doc-tests for Claims::has_role
chore: update serde to 1.0.230
```

## Branch naming

```
feat/<crate>/<name>    new capability
fix/<crate>/<name>     bug fix
chore/<name>           tooling / deps / CI
docs/<name>            documentation only
```
