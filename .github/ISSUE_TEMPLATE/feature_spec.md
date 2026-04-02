---
name: Feature Spec
description: Structured spec for a new feature — readable by coding agents
title: "feat(<crate>): <short description>"
labels: ["enhancement"]
---

## Affected crates

<!-- List every crate that will change. -->

- `rok-`

## Problem

<!-- One paragraph. What is missing or broken today? -->

## Public API

<!-- New or changed Rust signatures. Use a code block.
     This is what the agent will implement exactly. -->

```rust
// Example — replace with actual signatures
pub fn new_function(arg: &str) -> Result<Output, Error>;
```

## Behaviour

<!-- Describe inputs, outputs, side effects, and edge cases.
     Be precise enough that no follow-up questions are needed. -->

## Acceptance criteria

<!-- Each item must be a concrete, verifiable statement.
     The agent ticks these off as it implements. -->

- [ ] `./scripts/dev.sh gates` passes with no modifications to the gate commands
- [ ] Every new `pub` item has a `///` doc comment
- [ ] At least one unit test covers the happy path
- [ ] At least one unit test covers the primary error case

## Example usage

<!-- A short working Rust snippet or JSON task that exercises the feature. -->

```rust
```

## Out of scope

<!-- Explicitly list what is NOT being built in this issue. -->

-
