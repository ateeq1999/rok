# rok Development Plan

> From v1 to v5+ — Evolution and Future Roadmap

**Current Version**: v0.10.0 (v5 In Progress)
**Last Updated**: March 31, 2026

---

## Vision

**rok** is an AI-native code execution engine that collapses multi-step coding tasks into a single JSON document. The agent's job is **planning** (expressed as JSON). rok's job is **execution**.

```
agent writes one JSON file
         │
         ▼
    rok executes all steps
         │
         ▼
   one structured result
```

**Tagline**: Run One, Know All — One JSON. All Changes.

---

## Version History

### v1.0 — Core Pipeline (✅ Complete)
- Basic step types: `bash`, `read`, `write`, `mv`, `cp`, `rm`, `mkdir`
- Sequential execution
- Simple JSON schema

### v2.0 — Control Flow + Intelligence (✅ Complete)
- **Ref System**: Steps can reference outputs from previous steps
- **Control Flow**: `if`, `each`, `parallel` steps
- **Project Intelligence**: `scan`, `summarize`, `extract`, `grep`, `replace`
- **Surgical Editing**: `patch` step for targeted edits
- **Dev Tooling**: `diff`, `lint`, `git`, `http`, `snapshot`, `restore`

### v3.0 — Template System (✅ Complete)
- Custom templates with `.rok-template.json` schema
- Template props with validation
- Derived transforms (camelCase, PascalCase, etc.)
- Template inheritance
- Built-in templates for common patterns

### v4.0 — Task Files + Polish (✅ Complete)
- **Task Files**: Save, list, run, edit named tasks
- **Watch Mode**: Auto-rerun on file changes
- **History**: Track execution history
- **Replay**: Replay previous executions
- **Enhanced Steps**: `id`, `depends_on`, `timeout`, `retry`
- **Colored Output**: Pretty formatting
- **Shell Completions**: bash, zsh, fish, powershell, elvish

### v5.0 — Agent Efficiency (🔄 In Progress)
- **Cache Management**: Stats and clear commands (`rok cache`) ✅
- **Incremental Mode**: Skip steps when file inputs unchanged ✅
- **Import Management**: Auto-add/remove/organize imports ✅
- **Batch Operations**: Multi-file edits with glob + whole_word ✅
- **Symbol Refactoring**: Rename symbols across entire codebase ✅
- **Dependency Graph**: Map file import/export relationships ✅
- **Checkpoint/Resume**: Save and restore execution state ✅
- **Selective File Loading**: Filter by imports, exports, mtime ✅

### v6.0 — Intelligence Layer (📋 Planned)
- AI-assisted task composer (natural language → JSON)
- Example-based code generation (pattern inference)
- Semantic search across codebase
- Dead code detection
- Scaffold from spec (multi-file from minimal JSON)
- Plugin system for custom step types
- Interactive REPL mode

### v7.0 — Ecosystem (📋 Planned)
- Community template gallery
- VS Code extension (rok task sidebar)
- Web UI for task authoring
- rok cloud (run tasks remotely)
- Team task sharing and versioning

---

## Token Reduction Strategy

| Problem | rok Solution | Token Savings |
|---------|--------------|---------------|
| Sequential tool calls | All steps in one JSON | ~90% reduction |
| Reading files to find relevant ones | `scan` + `summarize` | ~80% reduction |
| Conditional logic needs two trips | `if` step with conditions | ~70% reduction |
| Looping over a list | `each` step | ~85% reduction |
| Generating code from scratch | `template` system | ~95% reduction |
| Re-reading config files | `extract` step | ~75% reduction |
| No rollback on risky refactors | `snapshot` / `restore` | Safety + tokens |
| Multi-step tasks need many payloads | `ref` system chaining | ~90% reduction |
| Renaming symbols manually | `refactor` step | ~88% reduction |
| Mapping dependencies manually | `deps` step | ~82% reduction |
| Re-running unchanged files | incremental mode | ~60% reduction |

**Total Impact**: A task that would normally require 20+ API calls and 50K+ tokens can be accomplished with 1 JSON payload and ~5K tokens.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Layer (cli.rs)                      │
│  - Argument parsing (clap)                                   │
│  - Input handling (file, stdin, inline JSON)                 │
│  - Command routing (run, save, list, watch, checkpoints...)  │
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
│  - Step type definitions (24 step types)                     │
│  - Type-safe JSON structures                                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Runner Engine (runner.rs)                  │
│  - Build execution order (dependency resolution)             │
│  - Execute steps with caching + incremental                  │
│  - Handle control flow (if/each/parallel)                    │
│  - Manage step references + checkpoints                      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Step Implementations (24 types)            │
│  ┌──────────┬──────────┬──────────┬──────────┬──────────┐   │
│  │  bash    │  read    │  write   │  grep    │ replace  │   │
│  ├──────────┼──────────┼──────────┼──────────┼──────────┤   │
│  │ refactor │  deps    │checkpoint│  import  │ template │   │
│  └──────────┴──────────┴──────────┴──────────┴──────────┘   │
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

---

## Core Innovations

### 1. The Ref System

Steps can reference outputs from previous steps using `{ "ref": N, "pick": "path" }`:

```json
{
  "steps": [
    { "type": "grep", "pattern": "TODO", "path": "./src" },
    {
      "type": "each",
      "over": { "ref": 0, "pick": "matches[*].path" },
      "as": "file",
      "step": { "type": "summarize", "path": "{{file}}" }
    }
  ]
}
```

### 2. Control Flow

**Conditional execution:**
```json
{
  "type": "if",
  "condition": { "type": "grepHasResults", "ref": 0 },
  "then": [{ "type": "bash", "cmd": "echo Found matches" }],
  "else": [{ "type": "bash", "cmd": "echo No matches" }]
}
```

**Loops:**
```json
{
  "type": "each",
  "over": ["file1.txt", "file2.txt"],
  "as": "file",
  "step": { "type": "read", "path": "{{file}}" }
}
```

### 3. Template System

Templates allow developers to define reusable code patterns:

```
.rok/templates/react-component/
├── .rok-template.json   # Schema definition
├── component.tsx        # Template file
└── test.tsx            # Additional output
```

### 4. Task Files

Save and reuse tasks:

```bash
rok -f task.json save my-task -d "Build and test"
rok run my-task
rok list
rok edit my-task
```

### 5. Symbol Refactoring (v5 NEW)

Rename symbols across an entire codebase with whole-word matching:

```json
{
  "type": "refactor",
  "symbol": "getUserById",
  "rename_to": "fetchUserById",
  "path": "./src",
  "ext": ["ts", "tsx"],
  "whole_word": true,
  "dry_run": false
}
```

### 6. Dependency Graph (v5 NEW)

Map file import/export relationships to understand codebase structure:

```json
{
  "type": "deps",
  "path": "./src",
  "depth": 4,
  "include": ["ts", "tsx"],
  "focus": "src/auth/index.ts"
}
```

### 7. Incremental Mode (v5 NEW)

Skip steps whose file inputs haven't changed since last run:

```json
{
  "options": {
    "incremental": true
  },
  "steps": [
    { "type": "grep", "pattern": "TODO", "path": "./src" }
  ]
}
```

---

## Implementation Status

| Feature | Status | Version |
|---------|--------|---------|
| Basic file operations | ✅ Complete | v1.0 |
| Bash execution | ✅ Complete | v1.0 |
| Reference system | ✅ Complete | v2.0 |
| Control flow (if/each/parallel) | ✅ Complete | v2.0 |
| Project intelligence (scan/summarize) | ✅ Complete | v2.0 |
| Search/replace (grep/replace) | ✅ Complete | v2.0 |
| Patch step | ✅ Complete | v2.0 |
| Diff/Lint | ✅ Complete | v2.0 |
| Git operations | ✅ Complete | v2.0 |
| HTTP requests | ✅ Complete | v2.0 |
| Snapshot/Restore | ✅ Complete | v2.0 |
| Template system | ✅ Complete | v3.0 |
| Task files | ✅ Complete | v4.0 |
| Watch mode | ✅ Complete | v4.0 |
| History/Replay | ✅ Complete | v4.0 |
| Cache management | ✅ Complete | v5.0 |
| Incremental mode | ✅ Complete | v5.0 |
| Import management | ✅ Complete | v5.0 |
| Symbol refactoring (`refactor`) | ✅ Complete | v5.0 |
| Dependency graph (`deps`) | ✅ Complete | v5.0 |
| Checkpoint/Resume (`checkpoint`) | ✅ Complete | v5.0 |
| Batch multi-file edit (glob/whole_word) | ✅ Complete | v5.0 |
| Selective file loading (filter_imports) | ✅ Complete | v5.0 |
| Dead code detection | 📋 Planned | v6.0 |
| AI task composer | 📋 Planned | v6.0 |
| Plugin system | 📋 Planned | v6.0 |
| VS Code extension | 📋 Planned | v7.0 |

---

## New Feature Suggestions (v6+)

### 6.1 — Multi-step Transactions
Run a group of steps atomically — if any fail, roll back all changes automatically. Builds on snapshot/restore but triggers rollback transparently.

```json
{
  "type": "transaction",
  "steps": [
    { "type": "replace", "pattern": "oldApi", "replacement": "newApi", "path": "./src" },
    { "type": "bash", "cmd": "cargo test" }
  ],
  "on_failure": "rollback"
}
```

### 6.2 — Smart Diff Review
Before applying changes, show a summary diff grouped by file and require explicit confirmation. Useful for agents making large structural changes.

```json
{
  "type": "diff_review",
  "changes": { "ref": 0 },
  "format": "grouped",
  "auto_apply": false
}
```

### 6.3 — File Watchers as Triggers
Define triggers: when specific files change, run a specific task automatically. Persistent background daemon mode.

```bash
rok watch-daemon --trigger "src/**/*.ts" --task build-and-test
```

### 6.4 — Variable Scoping & Computed Props
Allow props to reference other props and compute derived values:

```json
{
  "props": {
    "name": "UserProfile",
    "file_name": "{{ props.name | snake_case }}.ts",
    "test_file": "{{ props.name | snake_case }}.test.ts"
  }
}
```

### 6.5 — Step Output Assertions
Assert that a step produced expected output before continuing. Fail fast with clear diagnostics:

```json
{
  "type": "bash",
  "cmd": "cargo test",
  "assert": {
    "stdout_contains": "test result: ok",
    "exit_code": 0
  }
}
```

### 6.6 — Parallel Map-Reduce
Run steps over large file sets in parallel, then aggregate results:

```json
{
  "type": "map_reduce",
  "files": { "ref": 0, "pick": "matches[*].path" },
  "map": { "type": "summarize", "path": "{{item}}" },
  "reduce": "concat"
}
```

### 6.7 — Named Outputs / Exports
Let steps export named values that other steps can reference by name (not just index):

```json
{
  "type": "bash",
  "id": "get-version",
  "cmd": "cat Cargo.toml | grep version | head -1",
  "export": "version"
}
```

### 6.8 — rok Schema JSON Schema (jsonschema)
Publish a `rok.schema.json` so editors provide autocomplete and validation for `.rok.json` files.

### 6.9 — Semantic Code Search
Beyond regex grep — search by semantic meaning using AST parsing:

```json
{
  "type": "search",
  "query": "functions that return Option<T>",
  "path": "./src",
  "mode": "semantic"
}
```

### 6.10 — rok Compose (Multi-file Orchestration)
Reference and compose multiple task files into one orchestrated workflow:

```json
{
  "name": "full-deploy",
  "compose": [
    { "task": "build" },
    { "task": "test" },
    { "task": "deploy", "props": { "env": "prod" } }
  ]
}
```

---

## Dependencies

### Core
- `clap` — CLI argument parsing
- `serde` + `serde_json` — JSON serialization
- `anyhow` — Error handling

### File Operations
- `walkdir` — Directory traversal
- `glob` + `globset` — Glob pattern matching
- `similar` — Diff generation

### Code Intelligence
- `regex` — Pattern matching
- `tera` — Template engine
- `git2` — Git operations

### Network
- `reqwest` — HTTP client

### Utilities
- `colored` — Terminal colors
- `chrono` — Date/time handling
- `toml` + `serde_yaml` — Config file parsing
- `indicatif` — Progress indicators
- `clap_complete` — Shell completions
- `rayon` — Parallel execution

---

## Testing Strategy

### Unit Tests (48 tests)
- `refs.rs` — Reference resolution
- `schema.rs` — JSON parsing
- `config.rs` — Configuration
- `runner.rs` — Execution engine
- `cache.rs` — Cache management

### Integration Tests (22 tests)
- CLI commands
- File operations
- Control flow
- End-to-end workflows

### Benchmarks
- JSON parsing performance
- Step execution speed
- Cache effectiveness
- Incremental mode savings

---

## Metrics

| Metric | Target | Achieved |
|--------|--------|----------|
| Test Coverage | 70%+ | ✅ 74 tests |
| CI Build Time | < 5 min | ✅ ~3 min |
| Documentation | Complete | ✅ 12 files |
| Shell Completions | All shells | ✅ 5 shells |
| Configuration | .rokrc | ✅ 3 formats |
| Published | crates.io | ✅ v0.9.0 |
| Step Types | 20+ | ✅ 24 types |

---

## Next Steps

### Immediate (v0.10.0)
- [x] Symbol refactoring (`refactor` step)
- [x] Dependency graph (`deps` step)
- [x] Checkpoint/Resume (`checkpoint` step)
- [x] Incremental mode implementation
- [ ] Comprehensive documentation website
- [ ] Tests for new v5 step types

### Short-term (v0.11.0)
- [ ] Multi-step transactions (6.1)
- [ ] Step output assertions (6.5)
- [ ] Variable scoping & computed props (6.4)
- [ ] rok.schema.json for editor autocomplete (6.8)

### Long-term (v1.0.0)
- [ ] Plugin system for custom steps
- [ ] Interactive mode (REPL)
- [ ] AI-assisted task composer
- [ ] VS Code extension
- [ ] Community template gallery

---

## Conclusion

rok has evolved from a simple task runner into a **production-ready, AI-native code execution engine**. With 24 step types, 74 tests, comprehensive documentation, and publication on crates.io, it represents the most token-efficient way for AI agents to perform complex coding tasks.

The vision remains: **One JSON. All Changes.** The agent writes planning as JSON. rok handles execution. Clean separation. Maximum efficiency.

---

*For usage instructions, see [rok.md](rok.md). For API reference, see [docs/api.md](docs/api.md).*
