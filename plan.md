# `rok` — Run One, Know All
## Agent Task Runner — Implementation Plan

> **Concept:** An AI coding agent often wastes 80% of its tokens on sequential
> tool calls — one bash, wait, one read, wait, one move, wait. `rok` collapses
> an entire multi-step task into **a single invocation with a JSON payload**,
> executes all steps in the optimal order, and returns a single structured JSON
> report. One round-trip. Full context.

---

## Name

| Name | Meaning |
|---|---|
| **`rok`** | **R**un **O**ne **K**now-all — short, memorable, CLI-natural |
| alt: `rox` | Run Operations eXpress |
| alt: `jex` | JSON EXecutor |

→ **Recommended: `rok`**. Three letters, rolls off the keyboard, no conflicts
with common unix tools.

---

## Core Concept

Instead of an agent doing this (7 round-trips, ~1400 tokens of overhead):

```
agent → Bash(find ...)          → result
agent → Bash(grep ...)          → result
agent → Bash(mv a b)            → result
agent → Bash(mv c d)            → result
agent → Read(some/path/**)      → result
agent → Bash(sed ...)           → result
agent → Bash(grep -r ...)       → result
```

The agent does **one** call:

```bash
rok --json '{ "steps": [...] }'
```

And gets back **one** structured JSON result with all outputs, timings, and
errors — ready to parse in a single context injection.

---

## Interface

### Invocation

```bash
# JSON inline
rok --json '{ "steps": [...], "options": {} }'

# JSON from file
rok --file ./task.json

# JSON from stdin (pipe-friendly, great for agents)
echo '{ "steps": [...] }' | rok

# Dry run — validate and print plan without executing
rok --json '...' --dry-run

# Output format
rok --json '...' --output json      # default: machine-readable
rok --json '...' --output pretty    # human terminal
rok --json '...' --output silent    # exit code only
```

---

## JSON Payload Schema

### Input

```jsonc
{
  // optional global settings
  "options": {
    "cwd": "/d/ZNZ/projects/karibu_pass_proj",  // working dir for all steps
    "stop_on_error": true,                       // default: true
    "timeout_ms": 30000,                         // per-step timeout
    "env": { "NODE_ENV": "development" }         // extra env vars
  },

  // ordered list of steps
  "steps": [
    // --- STEP TYPES ---

    // 1. shell command
    { "type": "bash", "cmd": "find . -path '*routes/events.ts'" },

    // 2. read file(s) — globs supported
    { "type": "read", "path": "./.claude/worktrees/agent-a042efcf/**" },

    // 3. move / rename
    { "type": "mv", "from": "events/server-fns", "to": "events/server" },

    // 4. search & replace across files (rsr built-in)
    {
      "type": "replace",
      "pattern": "server-fns",
      "replacement": "server",
      "path": "./apps/web/src",
      "ext": ["ts", "tsx"],
      "regex": false
    },

    // 5. grep — structured match results
    {
      "type": "grep",
      "pattern": "@/components/loader\\b",
      "path": "./apps/web/src",
      "ext": ["ts", "tsx"],
      "regex": true
    },

    // 6. write file
    {
      "type": "write",
      "path": "./apps/web/src/config.ts",
      "content": "export const VERSION = '2.0';"
    },

    // 7. make directory
    { "type": "mkdir", "path": "./apps/web/src/new-module" },

    // 8. delete file or directory
    { "type": "rm", "path": "./apps/web/src/old-module", "recursive": true },

    // 9. copy
    { "type": "cp", "from": "./template", "to": "./new-feature", "recursive": true },

    // 10. run steps in parallel (when order doesn't matter)
    {
      "type": "parallel",
      "steps": [
        { "type": "mv", "from": "events/server-fns",     "to": "events/server"     },
        { "type": "mv", "from": "orders/server-fns",     "to": "orders/server"     },
        { "type": "mv", "from": "tickets/server-fns",    "to": "tickets/server"    },
        { "type": "mv", "from": "organizers/server-fns", "to": "organizers/server" },
        { "type": "mv", "from": "promo-codes/server-fns","to": "promo-codes/server"}
      ]
    }
  ]
}
```

---

## Output Schema

A single JSON object is always returned on stdout.

```jsonc
{
  "status": "ok",            // "ok" | "partial" | "error"
  "steps_total": 7,
  "steps_ok": 7,
  "steps_failed": 0,
  "duration_ms": 112,

  "results": [
    {
      "index": 0,
      "type": "bash",
      "cmd": "find . -path '*routes/events.ts'",
      "status": "ok",
      "stdout": "./apps/api/src/routes/events.ts\n./apps/web/src/routes/events.ts",
      "stderr": "",
      "exit_code": 0,
      "duration_ms": 14
    },
    {
      "index": 1,
      "type": "read",
      "path": "./.claude/worktrees/agent-a042efcf/**",
      "status": "ok",
      "files": [
        { "path": "./.claude/worktrees/agent-a042efcf/config.ts", "content": "..." },
        { "path": "./.claude/worktrees/agent-a042efcf/README.md", "content": "..." }
      ],
      "duration_ms": 8
    },
    {
      "index": 2,
      "type": "parallel",
      "status": "ok",
      "results": [
        { "index": 0, "type": "mv", "from": "events/server-fns", "to": "events/server", "status": "ok" },
        { "index": 1, "type": "mv", "from": "orders/server-fns",  "to": "orders/server",  "status": "ok" }
      ],
      "duration_ms": 3
    },
    {
      "index": 3,
      "type": "replace",
      "status": "ok",
      "files_scanned": 30,
      "files_modified": 12,
      "total_replacements": 47,
      "duration_ms": 38
    },
    {
      "index": 4,
      "type": "grep",
      "status": "ok",
      "matches": [
        { "path": "src/routes/dashboard.tsx", "line": 3,  "text": "import Loader from '@/components/loader'" },
        { "path": "src/routes/events.tsx",    "line": 11, "text": "import Logo from '@/components/Logo'" }
      ],
      "duration_ms": 9
    },
    {
      "index": 5,
      "type": "bash",
      "status": "error",
      "stdout": "",
      "stderr": "sed: no input files",
      "exit_code": 1,
      "duration_ms": 2,
      "stopped_pipeline": true    // stop_on_error was true — remaining steps skipped
    }
  ]
}
```

---

## Project Structure

```
rok/
├── Cargo.toml
├── plan.md
└── src/
    ├── main.rs          ← entry: parse input → run pipeline → emit output
    ├── cli.rs           ← clap: --json / --file / stdin + --output + --dry-run
    ├── config.rs        ← Options struct + global defaults
    ├── schema.rs        ← Step enum (serde) — the payload types
    ├── runner.rs        ← pipeline executor: sequential + parallel dispatch
    ├── steps/
    │   ├── bash.rs      ← std::process::Command
    │   ├── read.rs      ← glob expand + fs::read_to_string
    │   ├── write.rs     ← fs::write with parent mkdir
    │   ├── mv.rs        ← fs::rename (+ cross-device fallback)
    │   ├── cp.rs        ← recursive copy
    │   ├── rm.rs        ← fs::remove_file / remove_dir_all
    │   ├── mkdir.rs     ← fs::create_dir_all
    │   ├── grep.rs      ← regex::Regex over walkdir (returns match list)
    │   └── replace.rs   ← rayon parallel replace (rsr logic, reused here)
    ├── output.rs        ← JSON / pretty / silent formatters
    └── error.rs         ← RokError, StepError, exit codes
```

---

## Dependencies

```toml
[dependencies]
clap       = { version = "4", features = ["derive"] }
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
rayon      = "1"          # parallel steps + parallel replace
walkdir    = "2"          # glob/recursive read + grep
regex      = "1"          # grep + replace
globset    = "0.4"        # glob patterns in read/exclude
anyhow     = "1"          # error chaining
colored    = "2"          # pretty output
```

---

## Step Execution Model

```
parse JSON payload
      │
      ▼
validate all steps (schema check, path safety)   ← --dry-run stops here
      │
      ▼
  for each step (in order):
      │
      ├── type == "parallel" → rayon::scope → spawn child steps concurrently
      │                        wait for all → collect results
      │
      ├── type == "bash"     → Command::new("sh").arg("-c").arg(cmd)
      │                        capture stdout + stderr + exit_code
      │
      ├── type == "read"     → glob::expand(path) → par_iter → read_to_string
      │
      ├── type == "replace"  → walkdir + rayon (same as rsr core)
      │
      ├── type == "grep"     → walkdir + Regex::find_iter → match list
      │
      └── type == "mv/cp/rm/mkdir/write" → std::fs ops
      │
      ▼
  step failed?
      ├── stop_on_error: true  → mark remaining as "skipped", emit output
      └── stop_on_error: false → continue, accumulate errors
      │
      ▼
emit single JSON result → stdout
exit code
```

---

## Real-World Example — The Exact Task That Inspired rok

Rename `server-fns` → `server` across a monorepo, scan for stale imports,
read worktree context — all in one shot:

```json
{
  "options": {
    "cwd": "/d/ZNZ/projects/karibu_pass_proj",
    "stop_on_error": false
  },
  "steps": [
    {
      "type": "bash",
      "cmd": "find . -path '*routes/events.ts' -o -path '*api/src/routes*' -type f"
    },
    {
      "type": "parallel",
      "steps": [
        { "type": "mv", "from": "events/server-fns",     "to": "events/server"     },
        { "type": "mv", "from": "orders/server-fns",     "to": "orders/server"     },
        { "type": "mv", "from": "organizers/server-fns", "to": "organizers/server" },
        { "type": "mv", "from": "promo-codes/server-fns","to": "promo-codes/server"},
        { "type": "mv", "from": "tickets/server-fns",    "to": "tickets/server"    }
      ]
    },
    {
      "type": "replace",
      "pattern": "server-fns",
      "replacement": "server",
      "path": "./apps/web/src",
      "ext": ["ts", "tsx"]
    },
    {
      "type": "grep",
      "pattern": "@/components/loader\\b|@/components/ThemeToggle\\b|@/components/error-boundary\\b|@/components/Logo\\b|@/components/public-nav\\b|@/components/user-menu\\b|@/components/dashboard/|@/components/events/|@/components/admin/|@/components/landing/",
      "path": "./apps/web/src",
      "ext": ["ts", "tsx"],
      "regex": true
    },
    {
      "type": "read",
      "path": "./.claude/worktrees/agent-a042efcf/**"
    }
  ]
}
```

**Before `rok`:** 9 separate tool calls, ~1800 tokens of round-trip overhead.
**With `rok`:** 1 call, 1 result, ~180 tokens. **10× token reduction.**

---

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | All steps completed successfully |
| `1` | One or more steps failed (`partial`) |
| `2` | Invalid JSON payload / schema error |
| `3` | Fatal startup error (bad cwd, permission denied) |
| `4` | Timeout exceeded |

---

## Implementation Order for OpenCode

1. **`error.rs`** + **`schema.rs`** — types first, everything depends on these
2. **`cli.rs`** — parse `--json` / `--file` / stdin into raw `serde_json::Value`
3. **`config.rs`** — deserialize `Options`, validate `cwd` exists
4. **`steps/bash.rs`** — simplest step, test the result struct shape
5. **`steps/read.rs`** — glob expand, test with `**` patterns
6. **`steps/mv|cp|rm|mkdir|write.rs`** — thin `std::fs` wrappers
7. **`steps/grep.rs`** — walkdir + regex, return `Vec<GrepMatch>`
8. **`steps/replace.rs`** — port rsr core logic here
9. **`runner.rs`** — sequential pipeline + `parallel` dispatch via rayon
10. **`output.rs`** — JSON serializer + pretty printer
11. **`main.rs`** — wire everything, exit codes
12. **Integration tests** — use `tempfile` crate for isolated fs fixtures

---

## Agent Integration Note

The agent should always use `--output json` and inject the result directly into
its context. Recommended agent pattern:

```
Run this task with rok and return only the JSON result:
rok --json '<payload>'

Parse: if status == "ok" → proceed.
If status == "partial" or "error" → inspect results[*].status == "error"
entries and decide whether to retry or escalate.
```

**One plan. One call. One result.**
