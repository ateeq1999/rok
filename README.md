# rok - Run One, Know All

> Execute multi-step tasks from JSON - the ultimate automation tool for developers and AI agents.

[![Crates.io](https://img.shields.io/crates/v/rok-cli)](https://crates.io/crates/rok-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/ateeq1999/rok/actions/workflows/ci.yml/badge.svg)](https://github.com/ateeq1999/rok/actions/workflows/ci.yml)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen)](https://github.com/ateeq1999/rok)

## What is rok?

**rok** is a CLI tool that executes multi-step tasks defined in JSON format. It's designed for:
- **Developer workflows** - Automate repetitive tasks
- **AI agents** - Self-contained task execution
- **CI/CD pipelines** - Complex build and deployment scripts
- **Self-evolution** - Use rok to improve rok

## Installation

```bash
# From crates.io
cargo install rok-cli

# From source
git clone https://github.com/ateeq1999/rok
cd rok
cargo install --path .

# Local development
cargo install --path . --force
```

## Quick Start

### Basic Usage

```bash
# Run from file
rok -f task.json

# Run from stdin
echo '{"steps":[{"type":"bash","cmd":"echo hello"}]}' | rok

# Run from inline JSON
rok -j '{"steps":[{"type":"bash","cmd":"echo hello"}]}'
```

### Example: File Operations

```json
{
  "steps": [
    { "type": "mkdir", "path": "src/components" },
    { "type": "write", "path": "src/components/Hello.tsx", "content": "export const Hello = () => <div>Hello World</div>" },
    { "type": "bash", "cmd": "npm run build" }
  ]
}
```

## Core Concepts

### Steps

rok provides 20+ built-in step types:

| Category | Steps |
|----------|-------|
| **File Operations** | `read`, `write`, `mv`, `cp`, `rm`, `mkdir`, `patch` |
| **Search** | `grep`, `scan`, `extract` |
| **Code Analysis** | `summarize`, `lint`, `diff` |
| **Templates** | `template` |
| **Version Control** | `git`, `snapshot`, `restore` |
| **Network** | `http` |
| **Control Flow** | `if`, `each`, `parallel` |

### References

Pass data between steps using refs:

```json
{
  "steps": [
    { "type": "scan", "path": "./src", "depth": 2 },
    { "type": "each", "over": {"ref": 0, "pick": "tree.*"}, "step": { "type": "bash", "cmd": "echo {{item}}" }}
  ]
}
```

### Conditionals

```json
{
  "steps": [
    { "type": "grep", "pattern": "TODO", "path": "./src" },
    {
      "type": "if",
      "condition": { "type": "grep_has_results", "ref": 0 },
      "then": [
        { "type": "bash", "cmd": "echo Found TODOs!" }
      ]
    }
  ]
}
```

## Advanced Features

### Templates

Create reusable templates:

```bash
rok init-template my-component
```

Templates support:
- Custom props with validation
- Inheritance (`extends` field)
- Derived transforms (pluralize, camelcase, etc.)

### Task Files

Save and reuse tasks:

```bash
# Save current payload as a named task
rok -f task.json save my-task -d "Build the project"

# Run saved task
rok run my-task

# List all tasks
rok list
```

### Environment Variables

Use `{{env.VAR_NAME}}` in any string field:

```json
{
  "steps": [
    { "type": "bash", "cmd": "echo {{env.USER}}" }
  ]
}
```

### Timeouts & Retries

```json
{
  "steps": [
    {
      "type": "bash",
      "cmd": "make build",
      "timeout_ms": 60000,
      "retry": {
        "count": 3,
        "delay_ms": 2000,
        "backoff": true
      }
    }
  ]
}
```

### Step Enhancements

All steps support:
- `id` - Referenceable identifier
- Custom fields per step type (max_bytes, create_dirs, case_sensitive, context_lines, encoding)

## CLI Commands

```
rok - AI Agent Task Runner

Commands:
  templates          List available templates
  init-template      Create a new template
  validate-template  Validate a template schema
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
```

## Configuration

### Options

```json
{
  "name": "my-task",
  "description": "Build and deploy",
  "version": "1.0.0",
  "options": {
    "cwd": ".",
    "stop_on_error": true,
    "timeout_ms": 30000,
    "env": { "NODE_ENV": "production" }
  },
  "props": {
    "version": "1.0.0"
  },
  "steps": []
}
```

## Self-Evolution

rok is self-hosting - use it to improve itself:

```bash
# Generate a new step module
rok -f - <<'EOF'
{
  "steps": [
    { "type": "template", "builtin": "rust-module", "output": "./src/steps/new_step.rs", "vars": { "name": "new_step" } }
  ]
}
EOF
```

See [usage.md](usage.md) for more self-evolution patterns.

## Development

```bash
# Build
cargo build --release

# Test
cargo test

# Run all tests with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_test

# Run benchmarks
cargo bench

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# Generate documentation
cargo doc --open
```

### Project Structure

```
rok/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli.rs               # Argument parsing
│   ├── config.rs            # Configuration
│   ├── error.rs             # Error types
│   ├── output.rs            # Output formatting
│   ├── refs.rs              # Reference resolution (with tests)
│   ├── runner.rs            # Execution engine
│   ├── schema.rs            # JSON schema (with tests)
│   └── steps/               # Step implementations
├── tests/
│   └── integration_test.rs  # Integration tests
├── benches/
│   └── runner_bench.rs      # Benchmarks
├── docs/
│   ├── ARCHITECTURE.md      # Architecture documentation
│   ├── api.md               # API reference
│   └── ...
├── examples/                # Example task files
└── scripts/                 # Utility scripts
```

For more information, see:
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [CHANGELOG.md](CHANGELOG.md) - Version history
- [IMPROVEMENT_PLAN.md](IMPROVEMENT_PLAN.md) - Roadmap and improvements
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - Technical architecture

## License

MIT License - see [LICENSE](LICENSE) for details.
