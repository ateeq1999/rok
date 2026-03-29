# rok Implementation Progress

## Status: ✅ v2 Complete

## Implemented Features (v2)

### Phase 1: Core Infrastructure
- [x] `Cargo.toml` - Project configuration with dependencies
- [x] `error.rs` - Error types and exit codes
- [x] `schema.rs` - Step enum with all v2 types
- [x] `cli.rs` - Clap argument parsing (--json, --file, --output, --dry-run)
- [x] `config.rs` - Options struct with validation

### Phase 2: Step Implementations
- [x] `steps/bash.rs` - Shell command execution
- [x] `steps/read.rs` - Glob expand and file reading
- [x] `steps/write.rs` - Write file contents (returns diff)
- [x] `steps/patch.rs` - Surgical find-and-replace edits
- [x] `steps/mv.rs` - Move/rename files
- [x] `steps/cp.rs` - Copy files (with recursive support)
- [x] `steps/rm.rs` - Remove files/directories
- [x] `steps/mkdir.rs` - Create directories
- [x] `steps/grep.rs` - Regex search with match results
- [x] `steps/replace.rs` - Parallel search and replace
- [x] `steps/scan.rs` - Full project intelligence
- [x] `steps/summarize.rs` - File compression
- [x] `steps/extract.rs` - Config file key picker
- [x] `steps/diff.rs` - Compare files
- [x] `steps/lint.rs` - Linter adapter
- [x] `steps/template.rs` - Scaffold files
- [x] `steps/snapshot.rs` - Checkpoint/restore
- [x] `steps/git.rs` - Git operations
- [x] `steps/http.rs` - HTTP client

### Phase 3: Control Flow
- [x] `refs.rs` - JSONPath-lite resolver
- [x] `runner.rs` - Pipeline executor
- [x] `if` step - Conditional execution
- [x] `each` step - Map over list with `as` variable
- [x] `parallel` step - Concurrent execution

### Phase 4: Testing
- [x] Basic bash command execution
- [x] File operations (mkdir, write, read, cp, mv, rm, patch)
- [x] Grep and replace functionality
- [x] Scan step (project intelligence)
- [x] Extract step (config parsing)
- [x] Diff step (file comparison)
- [x] Template step (scaffolding)
- [x] Snapshot/restore (checkpoints)
- [x] Git operations
- [x] HTTP requests
- [x] Lint step
- [x] If/else conditional execution
- [x] Each with list and ref
- [x] Parallel step execution
- [x] Variable substitution in steps
- [x] JSONPath ref resolution
- [x] Output format options (json, pretty, silent)
- [x] Dry-run mode

## Project Structure

```
rok/
├── Cargo.toml
├── plan.md
├── PROGRESS.md
├── README.md
└── src/
    ├── main.rs
    ├── cli.rs
    ├── config.rs
    ├── error.rs
    ├── output.rs
    ├── runner.rs
    ├── schema.rs
    ├── refs.rs
    └── steps/
        ├── mod.rs
        ├── bash.rs
        ├── cp.rs
        ├── diff.rs
        ├── extract.rs
        ├── git.rs
        ├── grep.rs
        ├── http.rs
        ├── lint.rs
        ├── mkdir.rs
        ├── mv.rs
        ├── patch.rs
        ├── read.rs
        ├── replace.rs
        ├── rm.rs
        ├── scan.rs
        ├── snapshot.rs
        ├── summarize.rs
        ├── template.rs
        └── write.rs
```

## Verified Working

- `scan` - Full project intelligence with stack detection, entry points, tree, exports, imports
- `summarize` - File compression with imports, exports, functions, types
- `extract` - JSON/TOML/YAML/ENV key picker
- `diff` - File comparison with stat/unified/json formats
- `patch` - Surgical edits with unified diff output
- `write` - Returns diff (not full content)
- `lint` - Auto-detect and run linter (eslint/biome/clippy/ruff)
- `template` - Built-in templates (react-route, react-component, etc.)
- `snapshot`/`restore` - Checkpoint and rollback
- `git` - Structured git operations (status, diff, log, add, commit, branch)
- `http` - HTTP client with headers and expect_status
- `if` - Conditional execution with all condition types
- `each` - Map over list with custom variable name (`as`)
- `ref` - JSONPath resolution for chaining steps
- Variable substitution (`{{var}}` syntax)

## Usage Examples

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

# Example: Scan project and grep
{
  "steps": [
    { "type": "scan", "path": "./src", "depth": 3, "include": ["rs"] },
    { "type": "grep", "pattern": "fn main", "path": "./src", "ext": ["rs"] }
  ]
}

# Example: Conditional execution
{
  "steps": [
    { "type": "if", "condition": { "type": "exists", "path": "./Cargo.toml" },
      "then": [ { "type": "bash", "cmd": "echo has Cargo.toml" } ],
      "else": [ { "type": "bash", "cmd": "echo no Cargo.toml" } ]
    }
  ]
}

# Example: Each with grep results
{
  "steps": [
    { "type": "grep", "pattern": "TODO", "path": "./src", "ext": ["rs"] },
    { "type": "each", "over": { "ref": 0, "pick": "matches[*].path" },
      "as": "file",
      "step": { "type": "bash", "cmd": "echo {{file}}" }
    }
  ]
}
```
