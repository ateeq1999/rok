# rok API Reference

## Step Types

### bash

Run shell commands.

```json
{
  "type": "bash",
  "id": "build",
  "cmd": "npm run build",
  "timeout_ms": 60000,
  "retry": {
    "count": 3,
    "delay_ms": 1000,
    "backoff": true
  }
}
```

**Fields:**
- `cmd` (required): Shell command to execute
- `id`: Step identifier for references
- `timeout_ms`: Timeout in milliseconds
- `retry`: Retry configuration

### read

Read file contents.

```json
{
  "type": "read",
  "path": "src/main.rs",
  "max_bytes": 1048576,
  "encoding": "utf-8"
}
```

**Fields:**
- `path` (required): File path to read
- `max_bytes`: Maximum bytes to read
- `encoding`: File encoding (default: utf-8)

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
- `path` (required): Output file path
- `content` (required): Content to write
- `create_dirs`: Create parent directories (default: true)

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

### patch

Apply edits to a file.

```json
{
  "type": "patch",
  "path": "src/main.rs",
  "edits": [
    { "find": "fn old()", "replace": "fn new()" }
  ]
}
```

### grep

Search for patterns in files.

```json
{
  "type": "grep",
  "pattern": "TODO",
  "path": "./src",
  "ext": ["rs", "js"],
  "regex": true,
  "context_lines": 2
}
```

**Fields:**
- `pattern` (required): Search pattern
- `path` (required): Directory to search
- `ext`: File extensions to filter
- `regex`: Use regex (default: false)
- `context_lines`: Lines before/after match

### scan

Scan directory tree.

```json
{
  "type": "scan",
  "path": "./src",
  "depth": 3,
  "include": ["*.rs", "*.toml"]
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

### summarize

Summarize code structure.

```json
{
  "type": "summarize",
  "path": "./src",
  "focus": "exports"
}
```

### lint

Lint code files.

```json
{
  "type": "lint",
  "path": "./src",
  "tool": "clippy"
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

### template

Render a template file.

```json
{
  "type": "template",
  "builtin": "react-component",
  "output": "Component.tsx",
  "vars": {
    "name": "Button"
  }
}
```

### git

Run git commands.

```json
{
  "type": "git",
  "op": "commit",
  "args": ["-m", "commit message"]
}
```

### http

Make HTTP requests.

```json
{
  "type": "http",
  "method": "GET",
  "url": "https://api.example.com/data",
  "headers": { "Authorization": "Bearer token" },
  "expect_status": 200
}
```

### snapshot

Save a snapshot of files.

```json
{
  "type": "snapshot",
  "path": ".",
  "snapshot_id": "before-change"
}
```

### restore

Restore from a snapshot.

```json
{
  "type": "restore",
  "snapshot_id": "before-change"
}
```

### if

Conditional step execution.

```json
{
  "type": "if",
  "condition": { "type": "grep_has_results", "ref": 0 },
  "then": [
    { "type": "bash", "cmd": "echo found" }
  ],
  "else": [
    { "type": "bash", "cmd": "echo not found" }
  ]
}
```

### each

Iterate over items.

```json
{
  "type": "each",
  "over": ["a", "b", "c"],
  "as": "item",
  "parallel": false,
  "step": {
    "type": "bash",
    "cmd": "echo {{item}}"
  }
}
```

### parallel

Run steps in parallel.

```json
{
  "type": "parallel",
  "steps": [
    { "type": "bash", "cmd": "task1" },
    { "type": "bash", "cmd": "task2" }
  ]
}
```

## Condition Types

- `exists`: Check if path exists
- `contains`: Check if file contains pattern
- `grep_has_results`: Check if grep found results
- `step_ok`: Check if step succeeded
- `step_failed`: Check if step failed
- `file_changed`: Check if file changed

## Derived Transforms

Available in templates:

- `{{name | uppercase}}`
- `{{name | lowercase}}`
- `{{name | capitalize}}`
- `{{name | pluralize}}`
- `{{name | singularize}}`
- `{{name | camelcase}}`
- `{{name | snakecase}}`
- `{{name | kebabcase}}`
- `{{name | pascalcase}}`
