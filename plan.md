# `rok` v2 — Agent Token Killer
## Full Implementation Plan

> v1 collapsed sequential tool calls into one round-trip.
> v2 goes further: it gives the agent **pre-digested context** instead of raw
> file dumps, **conditional logic** so the agent doesn't need a second call to
> react to results, and **project intelligence** so the agent understands a
> codebase without reading every file.

---

## What Burns Agent Tokens (and how rok v2 kills each one)

| Token Waste | Root Cause | rok v2 Fix |
|---|---|---|
| Reading 40 files to find 3 relevant ones | Agent reads everything | `scan` step — ranked relevance, summaries only |
| Agent reads a file, asks Claude to summarize | Two round-trips | `summarize` step — built-in |
| `find` + `grep` + `read` to understand a codebase | 10+ calls | `scan` step — one call, full project map |
| Agent re-reads files it already saw | No memory | `write` returns diff + hash |
| Agent writes a file, reads it back to verify | Unnecessary | `write` returns diff + hash |
| Conditional logic needs a second call | No branching | `if` step — conditional execution in payload |
| Agent loops over a list manually | One call per item | `each` step — map over list |
| Agent asks "does X exist?" before acting | Defensive calls | `exists` condition built into `if` |
| Diffing two files requires reading both | 2 reads + agent diff | `diff` step |
| Agent reads package.json + tsconfig + ... | Config archaeology | `extract` step — key picker |

---

## New Step Types (v2 additions)

### `scan` — Full Project Intelligence (biggest token saver)

Instead of the agent reading 50 files to understand a codebase, `scan` returns
a structured map: file tree, exports, imports graph, detected stack, entry points.

```json
{
  "type": "scan",
  "path": "./apps/web/src",
  "depth": 3,
  "include": ["ts", "tsx"],
  "output": "summary"
}
```

`"output"` modes: `"summary"` | `"full"` | `"imports"` | `"exports"`

Result:
```json
{
  "type": "scan",
  "status": "ok",
  "stack": ["react", "typescript", "tanstack-router", "tailwind"],
  "entry_points": ["src/main.tsx", "src/routes/__root.tsx"],
  "file_count": 84,
  "tree": {
    "src/routes":     ["__root.tsx", "index.tsx", "dashboard/", "events/"],
    "src/components": ["ui/", "dashboard/", "events/"],
    "src/lib":        ["api.ts", "auth.ts", "utils.ts"]
  },
  "exports": {
    "src/lib/api.ts":  ["fetchEvents", "fetchOrders", "apiClient"],
    "src/lib/auth.ts": ["useAuth", "AuthProvider"]
  },
  "imports_graph": {
    "src/routes/dashboard/events/index.tsx": [
      "src/lib/api.ts",
      "src/components/events/EventCard.tsx"
    ]
  }
}
```

**Token savings:** replaces 20–50 file reads with one structured summary.

---

### `summarize` — Compress File Content

Instead of dumping a 400-line file into agent context, summarize it.

```json
{
  "type": "summarize",
  "path": "./src/routes/events.tsx",
  "focus": "exports, types, function signatures"
}
```

Result:
```json
{
  "type": "summarize",
  "status": "ok",
  "path": "src/routes/events.tsx",
  "summary": {
    "imports":   ["React", "useAuth from @/lib/auth", "EventCard from @/components/events"],
    "exports":   ["EventsPage (default)", "EventsLoader"],
    "functions": [
      "EventsLoader(): Promise<Event[]>",
      "EventsPage({ data }: { data: Event[] }): JSX.Element"
    ],
    "types_used":    ["Event", "LoaderData"],
    "line_count":    412,
    "last_modified": "2025-03-28T14:22:00Z"
  }
}
```

**Token savings:** replaces a 400-line read with a 20-line summary.

---

### `diff` — Compare Files

```json
{
  "type": "diff",
  "a": "./src/old/events.ts",
  "b": "./src/new/events.ts",
  "format": "stat"
}
```

`"format"` modes: `"unified"` | `"json"` | `"stat"` (minimal tokens)

`"stat"` result:
```json
{
  "type": "diff",
  "added": 12,
  "removed": 4,
  "changed_sections": ["EventsLoader function", "import block"],
  "is_identical": false
}
```

---

### `patch` — Surgical File Edit

Instead of the agent reading a file, modifying it mentally, then writing the
whole thing back — `patch` makes targeted edits and returns only the diff.

```json
{
  "type": "patch",
  "path": "./src/lib/api.ts",
  "edits": [
    {
      "find":    "import { oldClient } from './old-client'",
      "replace": "import { newClient } from './new-client'"
    },
    {
      "find":    "export const VERSION = '1.0'",
      "replace": "export const VERSION = '2.0'"
    }
  ]
}
```

Returns a unified diff — not the whole file.

---

### `if` — Conditional Execution (no second round-trip)

Run steps only if a condition holds. The agent doesn't need to receive a result,
decide, and send another payload.

```json
{
  "type": "if",
  "condition": { "type": "exists", "path": "./src/routes/events.tsx" },
  "then": [
    { "type": "read",  "path": "./src/routes/events.tsx" }
  ],
  "else": [
    { "type": "write", "path": "./src/routes/events.tsx", "content": "// new file" }
  ]
}
```

All supported conditions:

```jsonc
{ "type": "exists",           "path": "..." }
{ "type": "contains",         "path": "...", "pattern": "...", "regex": false }
{ "type": "grep_has_results", "ref": 2 }       // step index 2 returned matches
{ "type": "step_ok",          "ref": 1 }       // step index 1 succeeded
{ "type": "step_failed",      "ref": 1 }
{ "type": "file_changed",     "path": "...", "since": "2025-03-01" }
{ "type": "not",              "condition": { ... } }
{ "type": "and",              "conditions": [ {...}, {...} ] }
{ "type": "or",               "conditions": [ {...}, {...} ] }
```

---

### `each` — Map Over a List

Run a step template over multiple values — in parallel if order doesn't matter.

```json
{
  "type": "each",
  "over": ["events", "orders", "tickets", "organizers", "promo-codes"],
  "as": "module",
  "parallel": true,
  "step": {
    "type": "mv",
    "from": "{{module}}/server-fns",
    "to":   "{{module}}/server"
  }
}
```

Or map over files returned by a previous step using `ref`:

```json
{
  "type": "each",
  "over": { "ref": 0, "pick": "matches[*].path" },
  "as": "file",
  "parallel": true,
  "step": { "type": "summarize", "path": "{{file}}" }
}
```

`"ref": 0` pulls the file list from step 0's output.
`"pick"` is a simple JSONPath to extract the array.

---

### `extract` — Pull Specific Keys From Config Files

```json
{
  "type": "extract",
  "path": "./package.json",
  "pick": ["name", "version", "dependencies", "scripts"]
}
```

Works with: `package.json`, `tsconfig.json`, `Cargo.toml`, `.env`, any JSON / TOML / YAML / INI.

Result:
```json
{
  "type": "extract",
  "status": "ok",
  "data": {
    "name": "karibu-pass",
    "version": "0.4.2",
    "scripts": { "dev": "vite", "build": "tsc && vite build" }
  }
}
```

---

### `lint` — Run Linter, Return Structured Errors

```json
{
  "type": "lint",
  "path": "./apps/web/src",
  "tool": "auto"
}
```

`"tool"` values: `"auto"` | `"eslint"` | `"biome"` | `"clippy"` | `"ruff"`

`"auto"` detects the linter from project config files.

Result:
```json
{
  "type": "lint",
  "status": "ok",
  "errors_count": 1,
  "warnings_count": 3,
  "errors": [
    {
      "file": "src/routes/events.tsx",
      "line": 42,
      "rule": "no-unused-vars",
      "message": "'loader' is defined but never used",
      "severity": "error"
    }
  ]
}
```

**Token savings:** agent gets structured errors instead of parsing raw linter output.

---

### `template` — Scaffold Files

```json
{
  "type": "template",
  "builtin": "react-route",
  "output": "./src/routes/invoices/index.tsx",
  "vars": { "name": "Invoices", "model": "Invoice" }
}
```

Or use a custom template from the project:
```json
{
  "type": "template",
  "source": "./.rok/templates/route.tsx.tmpl",
  "output": "./src/routes/{{name}}/index.tsx",
  "vars": { "name": "invoices" }
}
```

Built-in templates: `react-route`, `react-component`, `api-handler`, `rust-module`, `test-file`

---

### `snapshot` / `restore` — Safe Checkpoints

```json
{ "type": "snapshot", "path": "./src", "id": "before-refactor" }
```

```json
{ "type": "restore", "id": "before-refactor" }
```

Stored in `.rok/snapshots/`. Agent can roll back without git.

---

### `git` — Structured Git Operations

```jsonc
{ "type": "git", "op": "status" }
{ "type": "git", "op": "diff",   "args": ["--stat", "HEAD~1"] }
{ "type": "git", "op": "log",    "args": ["--oneline", "-10"] }
{ "type": "git", "op": "add",    "args": ["."] }
{ "type": "git", "op": "commit", "args": ["-m", "refactor: rename server-fns to server"] }
{ "type": "git", "op": "branch", "args": ["--list"] }
```

Always returns structured JSON, not raw git text.

---

### `http` — Call an API

```json
{
  "type": "http",
  "method": "GET",
  "url": "http://localhost:3000/api/health",
  "headers": { "Authorization": "Bearer {{env.API_TOKEN}}" },
  "expect_status": 200
}
```

---

## `ref` System — Chain Steps Without a Round-Trip

Any step value can reference a previous step's output using `{ "ref": N, "pick": "..." }`.
This is what makes `if` and `each` powerful.

```json
{
  "steps": [
    // step 0: grep for stale imports
    { "type": "grep", "pattern": "server-fns", "path": "./src", "ext": ["ts","tsx"] },

    // step 1: only run replace if grep found something
    {
      "type": "if",
      "condition": { "type": "grep_has_results", "ref": 0 },
      "then": [
        { "type": "replace", "pattern": "server-fns", "replacement": "server", "path": "./src" }
      ]
    },

    // step 2: summarize every file that had a match
    {
      "type": "each",
      "over": { "ref": 0, "pick": "matches[*].path" },
      "as": "f",
      "step": { "type": "summarize", "path": "{{f}}" }
    }
  ]
}
```

Three logical operations. One round-trip. Zero intermediate calls.

---

## Full Step Reference (v2)

| Type | What it does |
|---|---|
| `bash` | Run shell command |
| `read` | Read file(s) with glob |
| `write` | Write file (returns diff) |
| `patch` | Surgical find-and-replace edits in one file |
| `replace` | Parallel search & replace across many files |
| `mv` | Move / rename |
| `cp` | Copy |
| `rm` | Delete |
| `mkdir` | Create directory |
| `grep` | Search pattern, return structured match list |
| `scan` | Full project structure + imports/exports map |
| `summarize` | Compress file to signatures + structure |
| `extract` | Pull specific keys from config files |
| `diff` | Compare two files |
| `lint` | Run linter, return structured errors |
| `template` | Scaffold files from built-in or custom template |
| `snapshot` | Checkpoint directory to `.rok/snapshots/` |
| `restore` | Roll back to a snapshot |
| `git` | Git operations with structured output |
| `http` | HTTP request |
| `if` | Conditional step execution |
| `each` | Map a step over a list |
| `parallel` | Run steps concurrently |

---

## Updated Project Structure

```
rok/
├── Cargo.toml
├── plan.md
└── src/
    ├── main.rs
    ├── cli.rs
    ├── config.rs
    ├── schema.rs          ← Step enum, now with all v2 types
    ├── runner.rs          ← pipeline + if/each/parallel + ref resolver
    ├── refs.rs            ← ref resolution engine (JSONPath-lite)
    ├── steps/
    │   ├── bash.rs
    │   ├── read.rs
    │   ├── write.rs       ← now returns unified diff
    │   ├── patch.rs       ← NEW: surgical edits + diff output
    │   ├── mv.rs
    │   ├── cp.rs
    │   ├── rm.rs
    │   ├── mkdir.rs
    │   ├── grep.rs
    │   ├── replace.rs
    │   ├── scan.rs        ← NEW: project intelligence
    │   ├── summarize.rs   ← NEW: file compression
    │   ├── extract.rs     ← NEW: config file key picker
    │   ├── diff.rs        ← NEW
    │   ├── lint.rs        ← NEW: eslint/biome/clippy/ruff adapter
    │   ├── template.rs    ← NEW
    │   ├── snapshot.rs    ← NEW
    │   ├── git.rs         ← NEW
    │   └── http.rs        ← NEW
    ├── output.rs
    └── error.rs
```

---

## Additional Dependencies (v2)

```toml
similar     = "2"        # unified diffs (write + diff + patch)
toml        = "0.8"      # extract: Cargo.toml
serde_yaml  = "0.9"      # extract: YAML configs
dotenvy     = "0.15"     # extract: .env files
tera        = "1"        # template rendering ({{var}} syntax)
reqwest     = { version = "0.12", features = ["blocking", "json"] }
git2        = "0.19"     # git step
tree-sitter = "0.22"     # summarize + scan: parse imports/exports
```

---

## Implementation Order for OpenCode

### Phase 1 — v1 core
`error` → `schema` → `cli` → `config` → `bash` → `read` → `mv/cp/rm/mkdir/write` → `grep` → `replace` → `runner` (sequential + parallel) → `output`

### Phase 2 — ref system + control flow
1. **`refs.rs`** — JSONPath-lite resolver (`matches[*].path`, `stdout`, etc.)
2. **`runner.rs`** — `if` step + all condition types
3. **`runner.rs`** — `each` step + `{{var}}` template substitution

### Phase 3 — project intelligence
4. **`scan.rs`** — walkdir + tree-sitter imports/exports parsing
5. **`summarize.rs`** — function/type signature extraction
6. **`extract.rs`** — JSON / TOML / YAML / ENV key picker

### Phase 4 — surgical editing
7. **`patch.rs`** — find/replace with diff output
8. **`diff.rs`** — unified diff between two files
9. **`write.rs`** — upgrade to always return diff

### Phase 5 — dev tooling
10. **`lint.rs`** — auto-detect + run linter + parse to JSON
11. **`git.rs`** — structured git output
12. **`http.rs`** — reqwest blocking client
13. **`template.rs`** — tera rendering + built-in scaffolds
14. **`snapshot.rs`** — tar.gz checkpoint to `.rok/snapshots/`

---

## Token Reduction Summary

| Scenario | Without rok | With rok v2 | Savings |
|---|---|---|---|
| Understand a new codebase | 30–50 file reads | 1 `scan` call | ~90% |
| Refactor + verify + summarize | 15+ calls | 1 payload with `if` + `each` | ~85% |
| Read config files before acting | 5–8 reads | 1 `extract` call | ~80% |
| Write file + verify content | 2 calls | 1 `write` (returns diff) | ~50% |
| Conditional rename workflow | 3–4 calls | 1 `if` step | ~70% |
| Rename 10 dirs | 10 `mv` calls | 1 `each` step | ~90% |
| Check linter before committing | bash + parse | 1 `lint` call | ~60% |
| Scaffold a new route | 5–8 writes | 1 `template` call | ~80% |
