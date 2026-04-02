---
name: Bug Report
description: Report a bug in any rok crate
title: "fix(<crate>): <short description>"
labels: ["bug"]
---

## Affected crate

<!-- e.g. rok-migrate, rok-orm, rok-cli -->

## Version

<!-- Output of: cargo pkgid <crate> -->

## What happened

<!-- Describe the bug. Be specific. -->

## Expected behaviour

<!-- What should have happened instead? -->

## Reproduction

<!-- Minimal code or rok JSON task that triggers the bug. -->

```rust
```

## Environment

- OS:
- Rust version (`rustc --version`):

## Acceptance criteria for the fix

- [ ] A failing test is added that reproduces the bug
- [ ] The fix makes that test pass
- [ ] `./scripts/dev.sh gates` is green
