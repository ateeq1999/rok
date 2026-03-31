# rok for AI Agents

> **Prompt**: You are an AI coding agent with access to the `rok` CLI tool. Use this guide to construct effective rok payloads.

---

## Quick Reference

```bash
# Run from inline JSON
rok -j '{"steps":[{"type":"bash","cmd":"echo hello"}]}'

# Run from file
rok -f task.json

# Run saved task
rok run my-task

# List tasks
rok list

# Show cache stats
rok cache --stats

# Generate shell completions
cargo run --example completions bash
```

---

## Core Principle

**Your job is planning. rok's job is execution.**

Write **one JSON payload** that describes the entire task. rok executes all steps and returns one structured result.

**Token Efficiency**: A task requiring 20+ API calls and 50K+ tokens becomes 1 JSON payload and ~5K tokens.

---

## Payload Structure

```json
{
  "name": "task-name",
  "description": "What this task does",
  "version": "1.0.0",
  "options": {
    "cwd": ".",
    "stop_on_error": false,
    "timeout_ms": 60000,
    "cache": true,
    "incremental": false,
    "env": {
      "NODE_ENV": "production"
    }
  },
  "props": {
    "feature_name": "users"
  },
  "steps": [
    // Your steps here
  ]
}
```

---

## Step Types

### File Operations

#### `bash` — Run Shell Command
```json
{
  "type": "bash",
  "id": "build",
  "cmd": "npm run build",
  "timeout_ms": 60000,
  "retry": {
    "count": 3,
    "delay_ms": 2000,
    "backoff": true
  }
}
```

#### `read` — Read Files
```json
{
  "type": "read",
  "path": "src/**/*.rs",
  "max_bytes": 1048576,
  "filter_imports": "React",
  "filter_exports": "Component",
  "since": "2024-01-01"
}
```

#### `write` — Write File
```json
{
  "type": "write",
  "path": "src/components/Button.tsx",
  "content": "export const Button = () => <button>Click</button>;",
  "create_dirs": true
}
```

#### `patch` — Surgical Edits
```json
{
  "type": "patch",
  "path": "src/main.rs",
  "edits": [
    { "find": "fn old_name()", "replace": "fn new_name()" },
    { "find": "let x = 1;", "replace": "let x = 2;" }
  ]
}
```

#### `mv` / `cp` / `rm` / `mkdir`
```json
{ "type": "mv", "from": "old.txt", "to": "new.txt" }
{ "type": "cp", "from": "src/", "to": "backup/", "recursive": true }
{ "type": "rm", "path": "dist/", "recursive": true }
{ "type": "mkdir", "path": "src/components" }
```

---

### Search & Replace

#### `grep` — Search Pattern
```json
{
  "type": "grep",
  "pattern": "TODO",
  "path": "./src",
  "ext": ["ts", "js"],
  "regex": false,
  "context_lines": 2
}
```

**Output**: `{ "matches": [{"path": "...", "line": N, "text": "..."}] }`

#### `replace` — Find/Replace Across Files
```json
{
  "type": "replace",
  "pattern": "oldFunction",
  "replacement": "newFunction",
  "path": "./src",
  "ext": ["ts", "js"],
  "glob": "**/*.{ts,js}",
  "whole_word": true
}
```

---

### Code Intelligence

#### `scan` — Project Map
```json
{
  "type": "scan",
  "path": "./src",
  "depth": 3,
  "include": ["ts", "tsx"],
  "output": "full"
}
```

**Output**: `{ "tree": {...}, "exports": {...}, "imports_graph": {...}, "file_count": N }`

#### `summarize` — File Structure
```json
{
  "type": "summarize",
  "path": "./src/App.tsx",
  "focus": "exports"
}
```

**Output**: `{ "imports": [...], "exports": [...], "functions": [...], "line_count": N }`

#### `extract` — Pull Keys from Config
```json
{
  "type": "extract",
  "path": "package.json",
  "pick": ["name", "version", "dependencies"]
}
```

#### `lint` — Run Linter
```json
{
  "type": "lint",
  "path": "./src",
  "tool": "auto"
}
```

**Output**: `{ "errors_count": N, "warnings_count": M, "errors": [...] }`

#### `diff` — Compare Files
```json
{
  "type": "diff",
  "a": "old.txt",
  "b": "new.txt",
  "format": "unified"
}
```

---

### Control Flow

#### `if` — Conditional Execution
```json
{
  "type": "if",
  "condition": { "type": "exists", "path": "./Cargo.toml" },
  "then": [
    { "type": "bash", "cmd": "cargo build" }
  ],
  "else": [
    { "type": "bash", "cmd": "echo No Cargo.toml" }
  ]
}
```

**Condition Types**:
- `{ "type": "exists", "path": "..." }`
- `{ "type": "contains", "path": "...", "pattern": "..." }`
- `{ "type": "grepHasResults", "ref": 0 }`
- `{ "type": "stepOk", "ref": 0 }`
- `{ "type": "stepFailed", "ref": 0 }`
- `{ "type": "fileChanged", "path": "...", "since": "2024-01-01" }`
- `{ "type": "not", "condition": {...} }`
- `{ "type": "and", "conditions": [...] }`
- `{ "type": "or", "conditions": [...] }`

#### `each` — Loop Over Items
```json
{
  "type": "each",
  "over": ["file1.txt", "file2.txt", "file3.txt"],
  "as": "file",
  "parallel": true,
  "step": {
    "type": "read",
    "path": "{{file}}"
  }
}
```

**With ref**:
```json
{
  "type": "each",
  "over": { "ref": 0, "pick": "matches[*].path" },
  "as": "file",
  "step": { "type": "summarize", "path": "{{file}}" }
}
```

#### `parallel` — Run Concurrently
```json
{
  "type": "parallel",
  "steps": [
    { "type": "bash", "cmd": "npm run build:client" },
    { "type": "bash", "cmd": "npm run build:server" }
  ]
}
```

---

### Version Control

#### `git` — Git Operations
```json
{
  "type": "git",
  "op": "commit",
  "args": ["-m", "feat: add new feature"]
}
```

**Operations**: `status`, `diff`, `log`, `add`, `commit`, `branch`

#### `snapshot` — Checkpoint
```json
{
  "type": "snapshot",
  "path": ".",
  "snapshot_id": "before-refactor"
}
```

#### `restore` — Rollback
```json
{
  "type": "restore",
  "snapshot_id": "before-refactor"
}
```

---

### Templates

#### `template` — Generate from Template
```json
{
  "type": "template",
  "builtin": "react-component",
  "output": "src/components/Button.tsx",
  "vars": {
    "name": "Button",
    "props": ["onClick", "disabled"]
  }
}
```

**Custom template**:
```json
{
  "type": "template",
  "name": "my-template",
  "output": "output.txt",
  "vars": {
    "name": "value"
  }
}
```

---

### Import Management

#### `import` — Manage Imports
```json
{
  "type": "import",
  "path": "src/main.ts",
  "add": [
    "import { useState } from 'react';",
    "import { useCallback } from 'react';"
  ],
  "remove": [
    "import { oldHook } from 'react';"
  ],
  "organize": true
}
```

---

### Network

#### `http` — HTTP Request
```json
{
  "type": "http",
  "method": "POST",
  "url": "https://api.example.com/data",
  "headers": {
    "Authorization": "Bearer token",
    "Content-Type": "application/json"
  },
  "expect_status": 201,
  "body": "{\"key\": \"value\"}"
}
```

---

## Reference System

Use `{ "ref": N }` to access previous step outputs:

```json
{
  "steps": [
    { "type": "grep", "pattern": "TODO", "path": "./src" },
    {
      "type": "bash",
      "cmd": "echo Found {{ref:0.matches.length}} TODOs"
    },
    {
      "type": "each",
      "over": { "ref": 0, "pick": "matches[*].path" },
      "as": "file",
      "step": { "type": "read", "path": "{{file}}" }
    }
  ]
}
```

**Common ref paths**:
- `ref:0` — Full output of step 0
- `ref:0.stdout` — Bash stdout
- `ref:0.matches` — Grep matches array
- `ref:0.matches[*].path` — All match paths
- `ref:0.files` — Read files array
- `ref:0.summary` — Summarize output

---

## Variable Substitution

### In Strings
```json
{
  "props": { "name": "Button" },
  "steps": [
    { "type": "write", "path": "{{props.name}}.tsx", "content": "..." }
  ]
}
```

### Environment Variables
```json
{
  "steps": [
    { "type": "bash", "cmd": "echo {{env.USER}}" },
    { "type": "write", "path": "{{env.HOME}}/.config/app.json", "content": "..." }
  ]
}
```

---

## Complete Examples

### 1. Rename Function Across Codebase
```json
{
  "name": "rename-function",
  "steps": [
    { "type": "snapshot", "path": "src", "snapshot_id": "before-rename" },
    { "type": "grep", "pattern": "function oldName", "path": "./src", "ext": ["ts"] },
    {
      "type": "if",
      "condition": { "type": "grepHasResults", "ref": 1 },
      "then": [
        {
          "type": "replace",
          "pattern": "oldName",
          "replacement": "newName",
          "path": "./src",
          "ext": ["ts"],
          "whole_word": true
        },
        { "type": "bash", "cmd": "echo Renamed successfully" }
      ],
      "else": [
        { "type": "bash", "cmd": "echo oldName not found" }
      ]
    }
  ]
}
```

### 2. Scaffold Feature
```json
{
  "name": "scaffold-feature",
  "props": { "feature": "users" },
  "steps": [
    { "type": "mkdir", "path": "src/features/{{props.feature}}" },
    {
      "type": "template",
      "builtin": "react-component",
      "output": "src/features/{{props.feature}}/index.tsx",
      "vars": { "name": "{{props.feature | pascalcase}}" }
    },
    {
      "type": "write",
      "path": "src/features/{{props.feature}}/types.ts",
      "content": "export interface {{props.feature | pascalcase}} { id: string; }\n"
    },
    { "type": "bash", "cmd": "echo Scaffolded {{props.feature}} feature" }
  ]
}
```

### 3. CI/CD Pipeline
```json
{
  "name": "ci-pipeline",
  "options": { "stop_on_error": true, "timeout_ms": 300000 },
  "steps": [
    { "type": "bash", "cmd": "cargo fmt -- --check", "id": "fmt" },
    { "type": "bash", "cmd": "cargo clippy -- -D warnings", "id": "lint" },
    { "type": "bash", "cmd": "cargo test --all", "id": "test" },
    { "type": "bash", "cmd": "cargo build --release", "id": "build" },
    {
      "type": "if",
      "condition": { "type": "and", "conditions": [
        { "type": "stepOk", "ref": "fmt" },
        { "type": "stepOk", "ref": "lint" },
        { "type": "stepOk", "ref": "test" },
        { "type": "stepOk", "ref": "build" }
      ]},
      "then": [
        { "type": "bash", "cmd": "echo ✓ All checks passed" }
      ]
    }
  ]
}
```

### 4. Batch Import Management
```json
{
  "name": "add-react-imports",
  "steps": [
    {
      "type": "read",
      "path": "src/**/*.tsx",
      "filter_imports": "React"
    },
    {
      "type": "each",
      "over": { "ref": 0, "pick": "files[*].path" },
      "as": "file",
      "step": {
        "type": "import",
        "path": "{{file}}",
        "add": [
          "import { useState, useEffect } from 'react';",
          "import { useCallback } from 'react';"
        ],
        "organize": true
      }
    }
  ]
}
```

---

## Best Practices

### 1. Snapshot Before Risky Operations
```json
{
  "steps": [
    { "type": "snapshot", "path": ".", "snapshot_id": "before-refactor" },
    // ... risky changes ...
    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": "lint" },
      "then": [
        { "type": "restore", "snapshot_id": "before-refactor" }
      ]
    }
  ]
}
```

### 2. Use IDs for References
```json
{
  "steps": [
    { "type": "grep", "id": "find-todos", "pattern": "TODO", "path": "./src" },
    { "type": "bash", "cmd": "echo Found {{ref:find-todos.matches.length}}" }
  ]
}
```

### 3. Chain with Refs
```json
{
  "steps": [
    { "type": "scan", "path": "./src", "depth": 2 },
    {
      "type": "each",
      "over": { "ref": 0, "pick": "tree.*" },
      "as": "file",
      "step": { "type": "summarize", "path": "{{file}}" }
    }
  ]
}
```

### 4. Use Conditionals
```json
{
  "steps": [
    { "type": "grep", "pattern": "console.log", "path": "./src" },
    {
      "type": "if",
      "condition": { "type": "grepHasResults", "ref": 0 },
      "then": [
        {
          "type": "replace",
          "pattern": "console.log\\(.*\\)",
          "replacement": "",
          "path": "./src"
        }
      ]
    }
  ]
}
```

### 5. Enable Caching
```json
{
  "options": {
    "cache": true,
    "cache_dir": ".rok/cache"
  },
  "steps": [...]
}
```

---

## CLI Commands

```bash
# Run task
rok -f task.json
rok -j '{"steps":[...]}'
rok run my-task

# Task management
rok save my-task -d "Description"
rok list
rok edit my-task

# Watch mode
rok watch -f task.json -w src/

# History
rok history
rok replay <run_id>

# Cache
rok cache --stats
rok cache --clear

# Templates
rok templates
rok init-template my-template
rok validate-template ./template-dir

# Help
rok --help
rok --version
```

---

## Output Schema

```json
{
  "status": "ok | partial | error",
  "steps_total": 10,
  "steps_ok": 9,
  "steps_failed": 1,
  "duration_ms": 1234,
  "results": [
    {
      "index": 0,
      "type": "bash",
      "status": "ok",
      "duration_ms": 100,
      "cmd": "echo hello",
      "stdout": "hello\n",
      "stderr": "",
      "exit_code": 0
    }
  ]
}
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All steps succeeded |
| 1 | Some steps failed |
| 2 | Invalid JSON |
| 3 | Startup error |

---

## Token Efficiency Comparison

| Task | Without rok | With rok | Savings |
|------|-------------|----------|---------|
| Rename function | 20 calls, 50K tokens | 1 payload, 5K tokens | 90% |
| Scaffold feature | 15 calls, 40K tokens | 1 payload, 4K tokens | 90% |
| Add imports to 10 files | 30 calls, 60K tokens | 1 payload, 6K tokens | 90% |
| CI/CD pipeline | 10 calls, 25K tokens | 1 payload, 3K tokens | 88% |

---

## Quick Start Template

Copy this as a starting point:

```json
{
  "name": "my-task",
  "description": "What this task does",
  "options": {
    "cwd": ".",
    "stop_on_error": false,
    "cache": true
  },
  "steps": [
    {
      "type": "bash",
      "id": "step1",
      "cmd": "echo hello"
    }
  ]
}
```

---

## Resources

- [PLAN.md](PLAN.md) — Development roadmap
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — Technical architecture
- [docs/guides/getting-started.md](docs/guides/getting-started.md) — User guide
- [docs/guides/step-types.md](docs/guides/step-types.md) — Step reference
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines

---

**Remember**: Write one JSON. rok does the rest. Run One, Know All.
