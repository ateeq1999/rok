# Getting Started with rok

> Your first steps with rok - Run One, Know All

## What is rok?

**rok** is a CLI tool that executes multi-step tasks defined in JSON format. It's designed to:

- **Automate repetitive tasks** - File operations, builds, deployments
- **Power AI agents** - Self-contained task execution for coding assistants
- **Streamline CI/CD** - Complex build and deployment pipelines
- **Enable self-evolution** - Use rok to improve rok itself

## Installation

### From crates.io (Recommended)

```bash
cargo install rok-cli
```

### From Source

```bash
git clone https://github.com/ateeq1999/rok
cd rok
cargo install --path .
```

### Verify Installation

```bash
rok --version
# Output: rok 0.9.0
```

## Quick Start

### Your First Task

Create a file called `hello.json`:

```json
{
  "steps": [
    {
      "type": "bash",
      "cmd": "echo Hello from rok!"
    }
  ]
}
```

Run it:

```bash
rok -f hello.json
```

### Running from Command Line

```bash
# From inline JSON
rok -j '{"steps":[{"type":"bash","cmd":"echo hello"}]}'

# From stdin
echo '{"steps":[{"type":"bash","cmd":"echo hello"}]}' | rok

# Dry run (preview without executing)
rok -f task.json --dry-run
```

## Core Concepts

### Steps

Tasks are made of **steps**. Each step performs one action:

```json
{
  "steps": [
    {"type": "bash", "cmd": "npm install"},
    {"type": "bash", "cmd": "npm run build"},
    {"type": "bash", "cmd": "npm test"}
  ]
}
```

### Step Types

rok provides 20+ built-in step types:

| Category | Steps |
|----------|-------|
| **File Operations** | `read`, `write`, `mv`, `cp`, `rm`, `mkdir`, `patch` |
| **Search** | `grep`, `replace`, `scan` |
| **Code Analysis** | `summarize`, `extract`, `lint`, `diff` |
| **Templates** | `template` |
| **Version Control** | `git`, `snapshot`, `restore` |
| **Network** | `http` |
| **Control Flow** | `if`, `each`, `parallel` |

### References

Pass data between steps using refs:

```json
{
  "steps": [
    {"type": "grep", "pattern": "TODO", "path": "./src"},
    {
      "type": "bash",
      "cmd": "echo Found {{ref:0.matches.length}} TODOs"
    }
  ]
}
```

### Control Flow

**Conditional execution:**

```json
{
  "steps": [
    {"type": "grep", "pattern": "TODO", "path": "./src"},
    {
      "type": "if",
      "condition": {"type": "grepHasResults", "ref": 0},
      "then": [
        {"type": "bash", "cmd": "echo Found TODOs to fix"}
      ]
    }
  ]
}
```

**Loop over items:**

```json
{
  "steps": [
    {
      "type": "each",
      "over": ["file1.txt", "file2.txt", "file3.txt"],
      "as": "file",
      "step": {"type": "bash", "cmd": "echo Processing {{file}}"}
    }
  ]
}
```

## Configuration

### Configuration File (.rokrc)

Create a `.rokrc` file in your project root:

```toml
[defaults]
cache = true
stopOnError = true
timeoutMs = 60000

[env]
NODE_ENV = "development"
API_URL = "http://localhost:3000"

[aliases]
build = "cargo build --release"
test = "cargo test --all"
```

### Task Options

Options can be specified in the task JSON:

```json
{
  "options": {
    "cwd": ".",
    "stop_on_error": false,
    "timeout_ms": 60000,
    "env": {"NODE_ENV": "production"}
  },
  "steps": []
}
```

## Common Patterns

### File Operations

```json
{
  "steps": [
    {"type": "mkdir", "path": "dist"},
    {"type": "write", "path": "dist/file.txt", "content": "Hello"},
    {"type": "cp", "from": "dist/", "to": "backup/", "recursive": true}
  ]
}
```

### Search and Replace

```json
{
  "steps": [
    {
      "type": "grep",
      "pattern": "oldFunction",
      "path": "./src",
      "ext": ["ts", "js"]
    },
    {
      "type": "replace",
      "pattern": "oldFunction",
      "replacement": "newFunction",
      "path": "./src",
      "ext": ["ts", "js"],
      "whole_word": true
    }
  ]
}
```

### Build and Test

```json
{
  "steps": [
    {"type": "bash", "cmd": "cargo build --release"},
    {"type": "bash", "cmd": "cargo test"},
    {
      "type": "if",
      "condition": {"type": "stepOk", "ref": 1},
      "then": [
        {"type": "bash", "cmd": "echo All tests passed!"}
      ]
    }
  ]
}
```

## CLI Reference

```
rok - AI Agent Task Runner

Commands:
  templates          List available templates
  init-template      Create a new template
  run                Run a saved task
  save               Save current payload as task
  list               List saved tasks
  edit               Edit a saved task
  watch              Watch files and re-run
  history            Show execution history
  replay             Replay a previous run

Options:
  -f, --file FILE    Path to JSON file
  -j, --json JSON    Inline JSON payload
  -o, --output       Output format: json, pretty, silent
  -v, --verbose      Enable verbose output
  -q, --quiet        Suppress output
  --dry-run          Preview without executing
  --help             Show help
  --version          Show version
```

## Next Steps

- [Step Types Guide](step-types.md) - Complete reference for all step types
- [Control Flow Guide](control-flow.md) - Master if/each/parallel
- [Templates Guide](templates.md) - Create reusable templates
- [Best Practices](best-practices.md) - Patterns and anti-patterns

## Getting Help

- Documentation: https://github.com/ateeq1999/rok
- Issues: https://github.com/ateeq1999/rok/issues
- Discussions: https://github.com/ateeq1999/rok/discussions
