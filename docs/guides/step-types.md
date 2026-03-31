# Step Types Reference

> Complete reference for all rok step types

## Overview

rok provides 20+ built-in step types organized into categories:

- **File Operations** - Read, write, move, copy, delete files
- **Search & Replace** - Find patterns, replace text
- **Code Analysis** - Summarize, extract, lint code
- **Templates** - Generate code from templates
- **Version Control** - Git operations, snapshots
- **Network** - HTTP requests
- **Control Flow** - Conditional execution, loops, parallel

---

## File Operations

### bash

Execute shell commands.

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

**Fields:**
- `cmd` (required) - Shell command to execute
- `id` - Step identifier for references
- `timeout_ms` - Timeout in milliseconds
- `retry` - Retry configuration

**Output:**
```json
{
  "type": "bash",
  "cmd": "npm run build",
  "stdout": "...",
  "stderr": "...",
  "exit_code": 0
}
```

### read

Read file contents.

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

**Fields:**
- `path` (required) - File path or glob pattern
- `max_bytes` - Maximum bytes to read per file
- `encoding` - File encoding (default: utf-8)
- `filter_imports` - Only files containing this import
- `filter_exports` - Only files containing this export
- `since` - Only files modified since this date

**Output:**
```json
{
  "type": "read",
  "path": "src/**/*.rs",
  "files": [
    {"path": "src/main.rs", "content": "..."}
  ],
  "files_filtered": 5
}
```

### write

Write content to a file.

```json
{
  "type": "write",
  "path": "output.txt",
  "content": "Hello World",
  "create_dirs": true
}
```

**Fields:**
- `path` (required) - Output file path
- `content` (required) - Content to write
- `create_dirs` - Create parent directories (default: true)

### patch

Apply edits to a file.

```json
{
  "type": "patch",
  "path": "src/main.rs",
  "edits": [
    {"find": "fn old()", "replace": "fn new()"}
  ]
}
```

**Fields:**
- `path` (required) - File to patch
- `edits` (required) - Array of find/replace pairs

### mv

Move/rename a file or directory.

```json
{
  "type": "mv",
  "from": "old.txt",
  "to": "new.txt"
}
```

### cp

Copy files or directories.

```json
{
  "type": "cp",
  "from": "source.txt",
  "to": "dest.txt",
  "recursive": true
}
```

### rm

Remove files or directories.

```json
{
  "type": "rm",
  "path": "temp/",
  "recursive": true
}
```

### mkdir

Create directories.

```json
{
  "type": "mkdir",
  "path": "src/components"
}
```

---

## Search & Replace

### grep

Search for patterns in files.

```json
{
  "type": "grep",
  "pattern": "TODO",
  "path": "./src",
  "ext": ["rs", "ts"],
  "regex": false,
  "case_sensitive": true,
  "context_lines": 2
}
```

**Fields:**
- `pattern` (required) - Search pattern
- `path` (required) - Directory to search
- `ext` - File extensions to filter
- `regex` - Use regex (default: false)
- `case_sensitive` - Case-sensitive search
- `context_lines` - Lines before/after match

**Output:**
```json
{
  "type": "grep",
  "pattern": "TODO",
  "matches": [
    {"path": "src/main.rs", "line": 10, "text": "// TODO: fix"}
  ]
}
```

### replace

Find and replace across files.

```json
{
  "type": "replace",
  "pattern": "foo",
  "replacement": "bar",
  "path": "./src",
  "ext": ["ts", "js"],
  "glob": "**/*.ts",
  "whole_word": true,
  "regex": false
}
```

**Fields:**
- `pattern` (required) - Pattern to find
- `replacement` (required) - Replacement text
- `path` (required) - Directory to search
- `ext` - File extensions to filter
- `glob` - Glob pattern for files
- `whole_word` - Match whole words only
- `regex` - Use regex

**Output:**
```json
{
  "type": "replace",
  "pattern": "foo",
  "replacement": "bar",
  "files_scanned": 50,
  "files_modified": 10,
  "total_replacements": 25
}
```

---

## Code Analysis

### scan

Scan directory tree.

```json
{
  "type": "scan",
  "path": "./src",
  "depth": 3,
  "include": ["rs", "toml"],
  "output": "full"
}
```

**Fields:**
- `path` (required) - Directory to scan
- `depth` - Maximum depth (default: 3)
- `include` - File extensions to include
- `output` - Output format: summary, full, imports, exports

**Output:**
```json
{
  "type": "scan",
  "path": "./src",
  "file_count": 42,
  "tree": {"src": ["main.rs", "lib.rs"]},
  "exports": {"main": ["main", "helper"]},
  "imports_graph": {"main": ["lib", "utils"]}
}
```

### summarize

Summarize code structure.

```json
{
  "type": "summarize",
  "path": "./src/main.rs",
  "focus": "exports"
}
```

**Fields:**
- `path` (required) - File to summarize
- `focus` - Focus area: imports, exports, functions, types

**Output:**
```json
{
  "type": "summarize",
  "path": "./src/main.rs",
  "summary": {
    "imports": ["std::fs", "std::io"],
    "exports": ["main", "helper"],
    "functions": ["main", "helper", "util"],
    "types_used": ["String", "Vec"],
    "line_count": 250
  }
}
```

### extract

Extract data from files using JSONPath.

```json
{
  "type": "extract",
  "path": "package.json",
  "pick": ["name", "version", "dependencies"]
}
```

**Fields:**
- `path` (required) - File to extract from
- `pick` - Keys to extract

**Output:**
```json
{
  "type": "extract",
  "path": "package.json",
  "data": {
    "name": "my-app",
    "version": "1.0.0"
  }
}
```

### lint

Lint code files.

```json
{
  "type": "lint",
  "path": "./src",
  "tool": "auto"
}
```

**Fields:**
- `path` (required) - Directory to lint
- `tool` - Linter: auto, eslint, biome, clippy, ruff

**Output:**
```json
{
  "type": "lint",
  "errors_count": 0,
  "warnings_count": 2,
  "errors": []
}
```

### diff

Show differences between files.

```json
{
  "type": "diff",
  "a": "old.txt",
  "b": "new.txt",
  "format": "unified"
}
```

**Fields:**
- `a` (required) - First file
- `b` (required) - Second file
- `format` - Output format: unified, json, stat

---

## Templates

### template

Render a template file.

```json
{
  "type": "template",
  "builtin": "react-component",
  "output": "Button.tsx",
  "vars": {
    "name": "Button"
  }
}
```

**Fields:**
- `builtin` - Built-in template name
- `name` - Custom template name
- `source` - Template source (inline)
- `output` - Output file path
- `vars` - Template variables

---

## Version Control

### git

Run git commands.

```json
{
  "type": "git",
  "op": "commit",
  "args": ["-m", "feat: add feature"]
}
```

**Fields:**
- `op` (required) - Operation: status, diff, log, add, commit, branch
- `args` - Command arguments

### snapshot

Save a snapshot of files.

```json
{
  "type": "snapshot",
  "path": ".",
  "snapshot_id": "before-refactor"
}
```

**Fields:**
- `path` (required) - Directory to snapshot
- `snapshot_id` (required) - Snapshot identifier

### restore

Restore from a snapshot.

```json
{
  "type": "restore",
  "snapshot_id": "before-refactor"
}
```

---

## Network

### http

Make HTTP requests.

```json
{
  "type": "http",
  "method": "POST",
  "url": "https://api.example.com/data",
  "headers": {"Authorization": "Bearer token"},
  "expect_status": 201,
  "body": "{\"key\": \"value\"}"
}
```

**Fields:**
- `method` (required) - HTTP method
- `url` (required) - Request URL
- `headers` - Request headers
- `expect_status` - Expected status code (default: 200)
- `body` - Request body

---

## Import Management

### import

Manage imports in code files.

```json
{
  "type": "import",
  "path": "src/main.ts",
  "add": ["import { foo } from './foo';"],
  "remove": ["import { bar } from './bar';"],
  "organize": true
}
```

**Fields:**
- `path` (required) - File to modify
- `add` - Imports to add
- `remove` - Imports to remove
- `organize` - Sort and organize imports

---

## Control Flow

### if

Conditional step execution.

```json
{
  "type": "if",
  "condition": {"type": "exists", "path": "./Cargo.toml"},
  "then": [
    {"type": "bash", "cmd": "cargo build"}
  ],
  "else": [
    {"type": "bash", "cmd": "echo No Cargo.toml"}
  ]
}
```

**Fields:**
- `condition` (required) - Condition to evaluate
- `then` - Steps to run if true
- `else` - Steps to run if false

**Condition Types:**
- `exists` - Check if path exists
- `contains` - Check if file contains pattern
- `grepHasResults` - Check if grep found results
- `stepOk` - Check if step succeeded
- `stepFailed` - Check if step failed
- `fileChanged` - Check if file changed
- `not` - Negate condition
- `and` - All conditions must be true
- `or` - Any condition must be true

### each

Iterate over items.

```json
{
  "type": "each",
  "over": ["item1", "item2", "item3"],
  "as": "item",
  "parallel": true,
  "step": {"type": "bash", "cmd": "echo {{item}}"}
}
```

**Fields:**
- `over` (required) - Items to iterate (array or ref)
- `as` - Variable name (default: "item")
- `parallel` - Run in parallel (default: true)
- `step` (required) - Step to execute for each item

### parallel

Run steps in parallel.

```json
{
  "type": "parallel",
  "steps": [
    {"type": "bash", "cmd": "task1"},
    {"type": "bash", "cmd": "task2"},
    {"type": "bash", "cmd": "task3"}
  ]
}
```

**Fields:**
- `steps` (required) - Steps to run in parallel

---

## Common Field Patterns

All steps support these common fields:

```json
{
  "type": "...",
  "id": "unique-id",
  "depends_on": ["step-id-1", "step-id-2"]
}
```

- `id` - Unique identifier for referencing
- `depends_on` - Step IDs that must complete first

---

## Related Guides

- [Getting Started](getting-started.md) - First steps with rok
- [Control Flow](control-flow.md) - Master if/each/parallel
- [Templates](templates.md) - Create reusable templates
