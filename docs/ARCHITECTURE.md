# rok Architecture

> Technical architecture documentation for developers

## Overview

rok is a JSON-driven task execution engine built in Rust. It parses JSON payloads and executes multi-step operations with support for control flow, references, and error handling.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Layer (cli.rs)                      │
│  - Argument parsing (clap)                                   │
│  - Input handling (file, stdin, inline JSON)                 │
│  - Command routing (run, save, list, watch, etc.)            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Configuration (config.rs)                  │
│  - Validate options                                          │
│  - Resolve working directory                                 │
│  - Merge environment variables                               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Schema Layer (schema.rs)                  │
│  - Payload deserialization (serde)                           │
│  - Step type definitions                                     │
│  - Type-safe JSON structures                                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Runner Engine (runner.rs)                  │
│  - Build execution order (dependency resolution)             │
│  - Execute steps with caching                                │
│  - Handle control flow (if/each/parallel)                    │
│  - Manage step references                                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Step Implementations                       │
│  ┌─────────┬─────────┬─────────┬─────────┬─────────┐        │
│  │  bash   │  read   │  write  │  grep   │ replace │ ...    │
│  └─────────┴─────────┴─────────┴─────────┴─────────┘        │
│  Each step implements: run(...) -> StepResult                │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Output Layer (output.rs)                   │
│  - Format results (JSON, pretty, silent)                     │
│  - Colored terminal output                                   │
│  - Error reporting                                           │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. CLI Layer (`cli.rs`)

**Responsibilities:**
- Parse command-line arguments using `clap`
- Handle multiple input sources (file, stdin, inline JSON)
- Route to appropriate command handler

**Key Types:**
```rust
pub struct Cli {
    pub command: Option<Commands>,
    pub json: Option<String>,
    pub file: Option<String>,
    pub output: OutputFormat,
    pub verbose: bool,
    pub quiet: bool,
}
```

**Commands:**
- `run` - Execute a saved task
- `save` - Save current payload
- `list` - List saved tasks
- `watch` - Watch files and re-run
- `history` - Show execution history
- `replay` - Replay previous run
- `templates` - List templates
- `init-template` - Create new template
- `validate-template` - Validate template

### 2. Schema Layer (`schema.rs`)

**Responsibilities:**
- Define type-safe JSON structures
- Serialize/deserialize payloads
- Define all step types

**Key Structures:**
```rust
pub struct Payload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub options: Options,
    pub props: HashMap<String, Value>,
    pub steps: Vec<Step>,
}

pub enum Step {
    Bash { cmd: String, ... },
    Read { path: String, ... },
    Write { path: String, content: String, ... },
    // ... 20+ step types
}
```

### 3. Runner Engine (`runner.rs`)

**Responsibilities:**
- Build execution order with dependency resolution
- Execute steps sequentially or in parallel
- Handle control flow (if/each/parallel)
- Manage step caching
- Track execution history

**Execution Flow:**
```
1. Build execution order (topological sort for dependencies)
2. For each step:
   a. Check cache (if enabled)
   b. Resolve references from previous steps
   c. Execute step implementation
   d. Save result to cache
   e. Update execution state
3. Format and return output
```

**Key Methods:**
```rust
pub fn run(&self) -> Output
fn build_execution_order(&self) -> Vec<(usize, &Step)>
fn execute_step(&self, step: &Step, index: usize, prev_results: &[StepResult]) -> StepResult
fn run_if(...) -> StepResult
fn run_each(...) -> StepResult
fn run_parallel(...) -> StepResult
```

### 4. Reference Resolution (`refs.rs`)

**Responsibilities:**
- Resolve references to previous step results
- Support JSONPath-like syntax for nested access
- Substitute variables in strings

**Reference Syntax:**
```json
{"ref": 0, "pick": "matches[*].path"}
```

**Functions:**
```rust
pub fn resolve_ref(step_index: usize, pick: &str, results: &[StepResult]) -> Option<Value>
pub fn has_grep_results(step_index: usize, results: &[StepResult]) -> bool
pub fn step_ok(step_index: usize, results: &[StepResult]) -> bool
pub fn substitute_vars(template: &str, var_name: &str, value: &str) -> String
pub fn substitute_env_vars(template: &str) -> String
```

### 5. Step Implementations (`steps/`)

Each step module implements a `run` function:

```rust
pub fn run(args..., cwd: &Path) -> StepResult {
    let start = Instant::now();
    
    // Implementation
    
    StepResult {
        index: 0,
        step_type: StepTypeResult::YourStep { ... },
        status: "ok".to_string(),
        duration_ms: start.elapsed().as_millis() as u64,
        stopped_pipeline: None,
    }
}
```

**Step Categories:**

| Category | Steps |
|----------|-------|
| File Operations | `read`, `write`, `mv`, `cp`, `rm`, `mkdir`, `patch` |
| Search | `grep`, `replace`, `scan` |
| Code Analysis | `summarize`, `extract`, `lint`, `diff` |
| Templates | `template` |
| Version Control | `git`, `snapshot`, `restore` |
| Network | `http` |
| Control Flow | `if`, `each`, `parallel` |
| Import Management | `import` |

## Data Flow

### 1. Input Processing

```
User Input (JSON)
       │
       ▼
┌──────────────────┐
│  cli.parse_payload()  │
└──────────────────┘
       │
       ▼
┌──────────────────┐
│  Payload struct   │
└──────────────────┘
```

### 2. Execution

```
Payload
   │
   ▼
┌─────────────────────────┐
│  Runner::new(config, payload) │
└─────────────────────────┘
   │
   ▼
┌─────────────────────────┐
│  Runner::run()               │
│  - Build execution order     │
│  - Execute each step         │
│  - Collect results           │
└─────────────────────────┘
```

### 3. Reference Resolution

```
Step N references Step 0
       │
       ▼
┌──────────────────────┐
│  refs::resolve_ref()  │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│  Extract from result  │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│  Substitute in step   │
└──────────────────────┘
```

## Execution Order

rok uses topological sorting to determine execution order based on dependencies:

```rust
fn build_execution_order(&self) -> Vec<(usize, &Step)> {
    // 1. Build id-to-index map
    // 2. Calculate in-degrees (dependency count)
    // 3. Topological sort (Kahn's algorithm)
    // 4. Handle unmet dependencies
}
```

## Caching System

When `options.cache` is enabled:

```
Step Execution
      │
      ▼
┌──────────────┐
│ Generate key │ (hash of step JSON)
└──────────────┘
      │
      ▼
┌──────────────┐
│ Check cache  │
└──────────────┘
      │
  ┌───┴───┐
  │ Hit   │ Miss
  ▼       ▼
Return  Execute
Cached  and Save
```

## Error Handling

rok uses a custom error type:

```rust
pub struct RokError {
    pub code: ExitCode,
    pub message: String,
}

pub enum ExitCode {
    Ok = 0,
    Partial = 1,
    SchemaError = 2,
    StartupError = 3,
    Timeout = 4,
}
```

**Error Propagation:**
- Step errors are captured in `StepResult`
- Runner continues or stops based on `stop_on_error`
- Final output includes error details

## Configuration

### Options Structure

```rust
pub struct Options {
    pub cwd: String,              // Working directory
    pub stop_on_error: bool,      // Stop on first error
    pub timeout_ms: u64,          // Default timeout
    pub env: HashMap<String, String>,  // Environment variables
    pub cache: bool,              // Enable caching
    pub cache_dir: Option<String>, // Cache directory
}
```

### Runtime Data

```
.rok/
├── tasks/           # Saved task files
├── snapshots/       # File snapshots for restore
├── templates/       # Custom templates
├── cache/           # Step execution cache
└── history.json     # Execution history
```

## Performance Considerations

### Parallelism

- `parallel` step executes sub-steps concurrently
- `each` step can run in parallel mode
- File operations use `rayon` for parallel iteration

### Caching

- Expensive operations can be cached
- Cache key is hash of step JSON
- Cache stored in `.rok/cache/`

### Memory

- Large files handled with `max_bytes` option
- Streaming for HTTP responses
- Lazy evaluation where possible

## Extension Points

### Adding New Steps

See `CONTRIBUTING.md` for detailed guide.

### Custom Templates

Templates are stored in `.rok/templates/` with schema:

```json
{
  "name": "template-name",
  "description": "...",
  "output": [{"from": "file.txt", "to": "{{name}}.txt"}],
  "props": {...}
}
```

### Custom Conditions

Extend `Condition` enum in `schema.rs`:

```rust
pub enum Condition {
    Exists { path: String },
    Contains { path: String, pattern: String },
    // Add new condition types here
}
```

## Testing Strategy

### Unit Tests

Test individual step implementations:

```rust
#[test]
fn test_read_file() {
    let result = read::run("test.txt", None, None, None, &Path::new("."));
    assert_eq!(result.status, "ok");
}
```

### Integration Tests

Test full task execution:

```rust
#[test]
fn test_full_workflow() {
    let payload = serde_json::json!({
        "steps": [
            {"type": "mkdir", "path": "test"},
            {"type": "write", "path": "test/file.txt", "content": "hello"}
        ]
    });
    // Execute and verify
}
```

## Future Enhancements

See `TODO.md` for planned features:

- Symbol refactoring
- Dependency graph analysis
- Incremental mode
- Example-based generation
- Task chaining
- Checkpoint/resume

## Debugging

### Verbose Mode

```bash
rok -f task.json --verbose
```

### Dry Run

```bash
rok -f task.json --dry-run
```

### History Replay

```bash
rok history
rok replay <run_id>
```

## Related Files

| File | Purpose |
|------|---------|
| `src/main.rs` | Entry point, CLI handling |
| `src/cli.rs` | Argument parsing |
| `src/schema.rs` | Type definitions |
| `src/runner.rs` | Execution engine |
| `src/refs.rs` | Reference resolution |
| `src/output.rs` | Output formatting |
| `src/error.rs` | Error types |
| `src/config.rs` | Configuration |
| `src/steps/*` | Step implementations |
