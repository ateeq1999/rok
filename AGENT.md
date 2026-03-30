# rok - AI Agent Documentation

> For coding agents: How to use rok for task automation

## Quick Start

```bash
# Install
cargo install rok-cli

# Run a task from file
rok -f task.json

# Run from stdin
echo '{"steps":[{"type":"bash","cmd":"echo hello"}]}' | rok

# List templates
rok templates
```

## Why Use rok?

rok collapses multiple API calls into one JSON payload:

| Without rok | With rok |
|------------|----------|
| 20+ API calls | 1 payload |
| 20+ round-trips | 1 round-trip |
| High token usage | Minimal tokens |

## Step Types

### File Operations

```json
{"type": "read", "path": "./src/**/*.rs"}
{"type": "write", "path": "./file.txt", "content": "..."}
{"type": "mkdir", "path": "./new-dir"}
{"type": "mv", "from": "old.txt", "to": "new.txt"}
{"type": "cp", "from": "src/", "to": "dest/", "recursive": true}
{"type": "rm", "path": "./file.txt"}
```

### Search & Replace

```json
{"type": "grep", "pattern": "TODO", "path": "./src", "ext": ["rs", "ts"]}
{"type": "replace", "pattern": "foo", "replacement": "bar", "path": "./src"}
```

### Project Intelligence

```json
{"type": "scan", "path": "./src", "depth": 3, "include": ["rs", "ts"]}
{"type": "summarize", "path": "./src/main.rs"}
{"type": "extract", "path": "./package.json", "pick": ["name", "version"]}
```

### Code Generation

```json
{"type": "template", "builtin": "react-component", "output": "./Button.tsx", "vars": {"name": "Button"}}
{"type": "template", "name": "my-template", "output": "./file.txt", "vars": {"key": "value"}}
```

### Version Control

```json
{"type": "git", "op": "status"}
{"type": "git", "op": "commit", "args": ["-m", "fix: bug"]}
{"type": "snapshot", "path": "./src", "id": "before-refactor"}
{"type": "restore", "id": "before-refactor"}
```

### Network

```json
{"type": "http", "method": "GET", "url": "https://api.example.com/data"}
{"type": "lint", "path": "./src", "tool": "auto"}
```

## Control Flow

### If/Else (Conditional)

```json
{
  "type": "if",
  "condition": {"type": "exists", "path": "./Cargo.toml"},
  "then": [{"type": "bash", "cmd": "echo has Cargo.toml"}],
  "else": [{"type": "bash", "cmd": "echo no Cargo.toml"}]
}
```

**Conditions:**
- `{"type": "exists", "path": "..."}`
- `{"type": "contains", "path": "...", "pattern": "..."}`
- `{"type": "grep_has_results", "ref": 0}`
- `{"type": "step_ok", "ref": 0}`
- `{"type": "step_failed", "ref": 0}`
- `{"type": "file_changed", "path": "...", "since": "2025-01-01"}`
- `{"type": "not", "condition": {...}}`
- `{"type": "and", "conditions": [...]}`
- `{"type": "or", "conditions": [...]}`

### Each (Loop)

```json
{
  "type": "each",
  "over": ["item1", "item2", "item3"],
  "as": "item",
  "step": {"type": "bash", "cmd": "echo {{item}}"}
}
```

### Parallel Execution

```json
{
  "type": "parallel",
  "steps": [
    {"type": "bash", "cmd": "echo 1"},
    {"type": "bash", "cmd": "echo 2"},
    {"type": "bash", "cmd": "echo 3"}
  ]
}
```

## Reference Previous Steps

Use `{ "ref": N, "pick": "..." }` to access step results:

```json
[
  {"type": "grep", "pattern": "TODO", "path": "./src"},
  {
    "type": "each",
    "over": {"ref": 0, "pick": "matches[*].path"},
    "as": "file",
    "step": {"type": "summarize", "path": "{{file}}"}
  }
]
```

## Template System

### Built-in Templates

```bash
rok templates  # List available
```

| Template | Description |
|----------|-------------|
| react-component | React functional component |
| react-route | TanStack React Router |
| api-handler | API handler |
| rust-module | Rust module |
| test-file | Vitest test file |

### Custom Templates

Create `.rok/templates/<name>/.rok-template.json`:

```json
{
  "name": "my-template",
  "description": "Description",
  "output": [{"from": "file.txt", "to": "{{name}}.txt"}],
  "props": {
    "name": {"type": "string", "required": true}
  }
}
```

### Filters

```text
{{ name | camelcase }}   # helloWorld
{{ name | snakecase }}   # hello_world
{{ name | kebabcase }}    # hello-world
{{ name | pascalcase }}   # HelloWorld
{{ name | pluralize }}    # items
{{ name | singularize }}  # item
{{ name | uppercase }}    # HELLO
{{ name | lowercase }}    # hello
{{ name | capitalize }}   # Hello
```

## Real-World Examples

### Refactor + Verify

```json
{
  "steps": [
    {"type": "grep", "pattern": "oldFunc", "path": "./src"},
    {
      "type": "if",
      "condition": {"type": "grep_has_results", "ref": 0},
      "then": [
        {"type": "replace", "pattern": "oldFunc", "replacement": "newFunc", "path": "./src"},
        {"type": "lint", "path": "./src", "tool": "auto"}
      ]
    }
  ]
}
```

### Generate Multiple Routes

```json
{
  "steps": [
    {
      "type": "each",
      "over": [
        {"name": "Users", "path": "/users"},
        {"name": "Products", "path": "/products"},
        {"name": "Orders", "path": "/orders"}
      ],
      "as": "route",
      "step": {
        "type": "template",
        "builtin": "react-route",
        "output": "./src/routes{{route.path}}/index.tsx",
        "vars": {"component": "{{route.name | pascalcase}}", "path": "{{route.path}}"}
      }
    }
  ]
}
```

### Full Stack Feature

```json
{
  "options": {"cwd": "./apps/api"},
  "steps": [
    {"type": "snapshot", "path": ".", "id": "feature-start"},
    {"type": "mkdir", "path": "./src/features/users"},
    {"type": "write", "path": "./src/features/users/model.ts", "content": "..."},
    {"type": "write", "path": "./src/features/users/service.ts", "content": "..."},
    {"type": "write", "path": "./src/features/users/routes.ts", "content": "..."},
    {"type": "bash", "cmd": "npm test"},
    {
      "type": "if",
      "condition": {"type": "step_failed", "ref": 5},
      "then": [
        {"type": "restore", "id": "feature-start"}
      ]
    }
  ]
}
```

## CLI Options

```bash
rok --help              # Show help
rok -f file.json        # Run from file
rok -j '{"steps":[...]}']  # Run from argument
rok --output pretty     # Pretty output
rok --dry-run          # Preview without executing
rok templates          # List templates
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Partial/Error |
| 2 | Invalid JSON |
| 3 | Startup Error |

## For Agents

When writing rok payloads:

1. **Minimize round-trips**: Combine related operations
2. **Use control flow**: `if`/`each` instead of multiple calls
3. **Use refs**: Chain steps instead of re-reading data
4. **Use templates**: Generate code instead of writing manually
5. **Use snapshots**: For risky operations, enable easy rollback

## Links

- [GitHub](https://github.com/ateeq1999/rok)
- [Crates.io](https://crates.io/crates/rok-cli)

---

## v5 Roadmap: Agent Efficiency Features

Planned features to reduce token usage and accelerate coding tasks:

### Coming Soon
- **Batch multi-file edit** - Replace across 100s of files in one step
- **Import management** - Auto-add/remove/organize imports
- **Symbol refactoring** - Rename across entire codebase
- **Selective file loading** - Only load relevant files
- **Incremental mode** - Only process changed files
- **Dependency graph** - Understand file relationships
- **Example-based generation** - Generate from 2-3 examples

### Already Available
- Step dependencies - Control execution order with `depends_on`
- Templates - Reusable code patterns
- Control flow - `if`/`each` for dynamic workflows
- History/replay - Debug previous runs
