# Usage.md - How to Use rok to Evolve Itself

> This document showcases how to use rok to evolve and improve itself

## Overview

rok is a self-hosting tool - it can use itself to implement new features. This document shows examples of using rok for self-improvement workflows.

## Showcase: How rok Uses Itself

### Example 1: Add a New Step Type

```bash
# Generate a new step implementation using template
rok -f - <<'EOF'
{
  "steps": [
    {
      "type": "template",
      "builtin": "rust-module",
      "output": "./src/steps/new_step.rs",
      "vars": { "name": "new_step" }
    }
  ]
}
EOF
```

### Example 2: Refactor Multiple Files

```bash
# Rename a function across all Rust files
rok -f - <<'EOF'
{
  "steps": [
    {
      "type": "grep",
      "pattern": "old_function_name",
      "path": "./src",
      "ext": ["rs"]
    },
    {
      "type": "if",
      "condition": { "type": "grep_has_results", "ref": 0 },
      "then": [
        {
          "type": "replace",
          "pattern": "old_function_name",
          "replacement": "new_function_name",
          "path": "./src",
          "ext": ["rs"]
        }
      ]
    }
  ]
}
EOF
```

### Example 3: Generate Test Files

```bash
# Generate tests for all modules
rok -f - <<'EOF'
{
  "steps": [
    {
      "type": "scan",
      "path": "./src",
      "depth": 2,
      "include": ["rs"]
    },
    {
      "type": "each",
      "over": {
        "ref": 0,
        "pick": "tree.*"
      },
      "as": "file",
      "step": {
        "type": "template",
        "builtin": "test-file",
        "output": "./tests/{{file}}.test.rs",
        "vars": { "name": "{{file}}" }
      }
    }
  ]
}
EOF
```

### Example 4: Batch Update Documentation

```bash
# Update all markdown files with new content
rok -f - <<'EOF'
{
  "steps": [
    {
      "type": "grep",
      "pattern": "old_version",
      "path": ".",
      "ext": ["md"]
    },
    {
      "type": "replace",
      "pattern": "old_version",
      "replacement": "v0.3.0",
      "path": ".",
      "ext": ["md"]
    }
  ]
}
EOF
```

### Example 5: Full Development Workflow

```bash
# Complete: snapshot -> make changes -> test -> commit
rok -f - <<'EOF'
{
  "steps": [
    { "type": "snapshot", "path": ".", "id": "before-feature" },
    { "type": "mkdir", "path": "./src/steps/new_feature" },
    { "type": "write", "path": "./src/steps/new_feature/mod.rs", "content": "..." },
    { "type": "write", "path": "./src/steps/new_feature/impl.rs", "content": "..." },
    { "type": "bash", "cmd": "cargo build --release" },
    { "type": "bash", "cmd": "cargo test" },
    {
      "type": "if",
      "condition": { "type": "step_failed", "ref": 5 },
      "then": [
        { "type": "restore", "id": "before-feature" },
        { "type": "bash", "cmd": "echo 'Build failed, restored snapshot'" }
      ]
    }
  ]
}
EOF
```

## Advanced Self-Evolution Patterns

### Pattern 1: Iterative Refactoring

```json
{
  "name": "Iterative Refactoring",
  "steps": [
    { "type": "scan", "path": "./src", "depth": 2 },
    {
      "type": "each",
      "over": { "ref": 0, "pick": "tree.*" },
      "as": "file",
      "step": {
        "type": "summarize",
        "path": "{{file}}"
      }
    },
    {
      "type": "if",
      "condition": { "type": "exists", "path": "./refactor_plan.json" },
      "then": [
        { "type": "read", "path": "./refactor_plan.json" }
      ]
    }
  ]
}
```

### Pattern 2: Automated Code Quality

```json
{
  "steps": [
    { "type": "lint", "path": "./src", "tool": "auto" },
    {
      "type": "if",
      "condition": { "type": "contains", "path": "./src", "pattern": "TODO" },
      "then": [
        { "type": "grep", "pattern": "TODO", "path": "./src" }
      ]
    }
  ]
}
```

### Pattern 3: Dependency Updates

```json
{
  "steps": [
    { "type": "extract", "path": "./Cargo.toml", "pick": ["dependencies"] },
    { "type": "bash", "cmd": "cargo outdated || echo 'No outdated deps'" }
  ]
}
```

## Real-World Self-Improvement

### Building a New Feature

```bash
# 1. Understand current codebase
rok -f - <<'EOF'
{
  "steps": [
    { "type": "scan", "path": "./src/steps", "depth": 1 }
  ]
}
EOF

# 2. Add new step to schema
rok -f - <<'EOF'
{
  "steps": [
    {
      "type": "patch",
      "path": "./src/schema.rs",
      "edits": [
        { "find": "// Add new steps above", "replace": "// Add new steps above\n    NewStep { ... }," }
      ]
    }
  ]
}
EOF

# 3. Create implementation
rok -f - <<'EOF'
{
  "steps": [
    {
      "type": "template",
      "builtin": "rust-module",
      "output": "./src/steps/new_step.rs",
      "vars": { "name": "new_step" }
    }
  ]
}
EOF

# 4. Test it works
rok -f - <<'EOF'
{
  "steps": [
    { "type": "bash", "cmd": "cargo build --release" }
  ]
}
EOF
```

## Best Practices for Self-Evolution

1. **Always snapshot before major changes**
2. **Run tests after each change**
3. **Use conditional steps to avoid errors**
4. **Chain steps with refs to avoid re-reading data**
5. **Use templates for repetitive code generation**

## Integration with CI/CD

```bash
# Pre-commit check
rok -f - <<'EOF'
{
  "steps": [
    { "type": "bash", "cmd": "cargo fmt -- --check" },
    { "type": "bash", "cmd": "cargo clippy -- -D warnings" },
    { "type": "bash", "cmd": "cargo test" }
  ]
}
EOF
```

## Summary

rok is designed to be self-hosting:
- All features are implemented as rok steps
- Use `scan` to understand the codebase
- Use `each` for batch operations
- Use `if` for conditional logic
- Use `snapshot`/`restore` for safety
- Use templates for code generation

This enables powerful self-improvement workflows!
