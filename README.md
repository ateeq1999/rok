# rok - Run One, Know All

An AI coding agent task runner that collapses multi-step operations into a single JSON invocation.

## Installation

```bash
cargo install rok
```

Or download pre-built binaries from [GitHub Releases](https://github.com/ateeq1999/rok/releases).

## Usage

```bash
# JSON inline
rok --json '{ "steps": [...] }'

# From file
rok --file ./task.json

# Pipe from stdin
echo '{ "steps": [...] }' | rok

# Pretty output
rok --json '...' --output pretty

# Dry run
rok --json '...' --dry-run
```

## JSON Payload

```json
{
  "options": {
    "cwd": ".",
    "stopOnError": true,
    "timeoutMs": 30000,
    "env": {}
  },
  "steps": [
    { "type": "bash", "cmd": "find . -name '*.rs'" },
    { "type": "read", "path": "./src/**/*.rs" },
    { "type": "write", "path": "./output.txt", "content": "..." },
    { "type": "mkdir", "path": "./new-dir" },
    { "type": "mv", "from": "old", "to": "new" },
    { "type": "cp", "from": "src", "to": "dest", "recursive": true },
    { "type": "rm", "path": "./file", "recursive": true },
    { "type": "grep", "pattern": "TODO", "path": ".", "ext": ["rs", "toml"] },
    { "type": "replace", "pattern": "foo", "replacement": "bar", "path": ".", "ext": ["rs"] },
    {
      "type": "parallel",
      "steps": [
        { "type": "mkdir", "path": "dir1" },
        { "type": "mkdir", "path": "dir2" }
      ]
    }
  ]
}
```

## Output

```json
{
  "status": "ok",
  "stepsTotal": 5,
  "stepsOk": 5,
  "stepsFailed": 0,
  "durationMs": 150,
  "results": [...]
}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All steps completed successfully |
| 1 | One or more steps failed |
| 2 | Invalid JSON payload |
| 3 | Startup error |

## License

MIT
