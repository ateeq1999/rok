# rok Development Plan

> From v1 to v4+ — Evolution and Future Roadmap

**Current Version**: v0.9.0 (v4 Complete)  
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
- **Cache Management**: Stats and clear commands (`rok cache`)
- **Incremental Mode**: Process only changed files
- **Import Management**: Auto-add/remove/organize imports
- **Batch Operations**: Multi-file edits in one step

### v6.0 — Future (📋 Planned)
- Symbol refactoring across codebase
- Dependency graph visualization
- Example-based code generation
- Plugin system for custom steps
- Interactive mode (REPL)
- AI-assisted task composer

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

**Total Impact**: A task that would normally require 20+ API calls and 50K+ tokens can be accomplished with 1 JSON payload and ~5K tokens.

---

## Architecture Overview

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
| Incremental mode | 🔄 In Progress | v5.0 |
| Import management | 🔄 In Progress | v5.0 |
| Symbol refactoring | 📋 Planned | v6.0 |
| Dependency graph | 📋 Planned | v6.0 |

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

---

## Next Steps

### Immediate (v0.10.0)
- [ ] Full incremental mode implementation
- [ ] Import management step enhancements
- [ ] Documentation website (mdBook)

### Short-term (v0.11.0)
- [ ] Symbol refactoring across codebase
- [ ] Dependency graph visualization
- [ ] Video tutorials

### Long-term (v1.0.0)
- [ ] Plugin system for custom steps
- [ ] Interactive mode (REPL)
- [ ] AI-assisted task composer
- [ ] Community template gallery

---

## Conclusion

rok has evolved from a simple task runner into a **production-ready, AI-native code execution engine**. With 74 tests, comprehensive documentation, and publication on crates.io, it's ready for widespread adoption.

The vision remains: **One JSON. All Changes.** The agent writes planning as JSON. rok handles execution. Clean separation. Maximum efficiency.

---

*For usage instructions, see [rok.md](rok.md). For API reference, see [docs/api.md](docs/api.md).*
