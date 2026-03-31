# rok Implementation Todo

> **Status**: v5 In Progress | **Next**: v5 completion + v6 Intelligence Layer

## v5: Agent Efficiency Features

Focus: Reduce token usage, accelerate repetitive coding tasks, enable agents to work more efficiently.

### Phase 1: Smart Code Operations

- [x] **5.1** Batch Multi-file Edit (2026-03-31)
  - Single step to find/replace across 100s of files
  - Support glob patterns, regex, whole-word matching
  - Added `glob` and `whole_word` fields to replace step
  
- [x] **5.2** Import Management (2026-03-31)
  - Auto-add missing imports
  - Remove unused imports  
  - Organize imports (sort, group by type)
  - Support: JS/TS, Python, Rust, Go

- [x] **5.3** Symbol Refactoring (2026-03-31)
  - Rename functions/variables across entire codebase
  - Update all references automatically
  - Preview changes before applying (dry_run mode)
  - Parallel execution via rayon, whole-word regex matching

- [ ] **5.4** Boilerplate Auto-fill
  - Detect file type, auto-add standard imports/headers
  - Add copyright headers, license blocks
  - Template-based scaffolding

### Phase 2: Context Optimization

- [x] **5.5** Selective File Loading (2026-03-31)
  - Only load relevant files (by imports, exports, git diff)
  - Filter by file type, directory, patterns
  - Added filter_imports, filter_exports, since to read step

- [x] **5.6** Incremental Mode (2026-03-31)
  - Only process files changed since last run
  - Track state in .rok/incremental_state.json (mtime-based)
  - Supports: read, grep, replace, scan, refactor, deps steps

- [x] **5.7** Context Compression (2026-03-31)
  - Summarize large files before passing to agent
  - Extract key functions/classes (already available via summarize step)

### Phase 3: Agent Workflow

- [ ] **5.8** Task Chaining
  - Pass output of one step as input to next
  - Share state between task runs
  
- [x] **5.9** Checkpoint/Resume (2026-03-31)
  - Save progress mid-execution via `checkpoint` step
  - Stored in .rok/checkpoints/{id}.json
  - CLI: `rok checkpoints --list`, `rok checkpoints --delete`

- [x] **5.10** Result Caching (2026-03-31)
  - Cache expensive operations via `"cache": true` in options
  - Skip re-runs if step inputs unchanged (hash-based)
  - CLI: `rok cache --stats`, `rok cache --clear`

### Phase 4: Code Intelligence

- [x] **5.11** Dependency Graph (2026-03-31)
  - Map file relationships (imports/exports) via `deps` step
  - Supports JS/TS, Python, Rust, Go
  - Cycle detection, focus mode for single-file analysis

- [x] **5.12** Export/Import Scanner (2026-03-31)
  - `deps` step maps all imports/exports across codebase
  - `scan` step with output:"imports"|"exports" for targeted scan
  
- [ ] **5.13** Dead Code Detection
  - Find unused functions/variables
  - Identify unreachable code

### Phase 5: Generation & Templates

- [ ] **5.14** Example-based Generation
  - Agent provides 2-3 examples
  - Rok infers pattern and generates the rest
  
- [ ] **5.15** Scaffold from Spec
  - Generate entire file structures from minimal JSON
  - Multi-file template expansion

---

## v4: One JSON. All Changes (COMPLETE)

All tasks 4.1-4.24 completed.

### Phase 1: Task Files
- [x] 4.1 .rok.json support
- [x] 4.2 save command
- [x] 4.3 run command
- [x] 4.4 list command
- [x] 4.5 edit command

### Phase 2: Enhanced Steps
- [x] 4.6 id field
- [x] 4.7 max_bytes
- [x] 4.8 create_dirs
- [x] 4.9 case_sensitive
- [x] 4.10 context_lines
- [x] 4.11 encoding

### Phase 3: CLI Enhancements
- [x] 4.12 watch
- [x] 4.13 history
- [x] 4.14 replay

### Phase 4: Advanced Features
- [x] 4.15 step dependencies
- [x] 4.16 env var expansion
- [x] 4.17 timeout override
- [x] 4.18 retry logic

### Phase 5: Polish
- [x] 4.19 colored output
- [x] 4.20 progress indicator
- [x] 4.21 verbose flag
- [x] 4.22 quiet flag
- [x] 4.23 shell completions
- [x] 4.24 man pages

---

## v3: Bring Your Own Templates (COMPLETE)

All tasks 3.1-3.11 completed.

---

## Quick Reference

### Current Version
```
v0.6.0 (commit 71d475e)
```

### Testing Commands

```bash
# Test step dependencies
echo '{"steps":[{"id":"step1","type":"bash","cmd":"echo step1"},{"id":"step2","depends_on":["step1"],"type":"bash","cmd":"echo step2"}]}' | rok
```
