# rok Implementation Todo

> **Status**: v2 Complete | **Next**: v3 - Bring Your Own Templates

## Agent Instructions

### How to Work on Tasks

1. **Pick a task** from the Todo list below
2. **Create a branch** for each major feature: `git checkout -b feat/<feature-name>`
3. **Implement the feature** following the acceptance criteria
4. **Test your implementation** with example payloads
5. **Commit** using the commit template below
6. **Update this file** with progress (mark completed items)

### Commit Template

```bash
git add <changed-files>
git commit -m "feat: <feature name>

- Added <specific change 1>
- Added <specific change 2>
- Fixed <bug if any>

Closes #<issue-number>"
```

### Progress Update Template

When completing a task, update the todo.md:
- [x] Task name (date completed)
- Add implementation notes if needed
- Link to commit hash

---

## v3: Bring Your Own Templates

### Phase 1: Template Engine Core

- [x] **3.1** Create template discovery system (2026-03-30)
  - Search paths: `./.rok/templates/`, `~/.rok/templates/`, built-in
  - Accepts: `.rok-template.json` schema files
  - Returns: list of available templates with metadata
  - Added: `rok templates` CLI command
  - Added: `name` field to template step for custom template lookup
  
- [x] **3.2** Define `.rok-template.json` schema (2026-03-30)
  - `name`, `description`, `version`, `author`, `tags`
  - `output[]`: `{ from, to, condition? }`
  - `props`: prop definitions with types (string, enum, boolean, path, array)
  - Added: hooks, post_generate support
  - Added: derive_from, derive, pattern, min, max for props

- [x] **3.3** Implement prop type system (2026-03-30)
  - `string`: basic text input with pattern, min, max validation
  - `enum`: predefined values validation
  - `boolean`: true/false toggle validation
  - `path`: relative path validation
  - `array`: JSON array or comma-separated validation
  - Added: validate_prop function for all types

- [x] **3.4** Build template syntax parser (Tera-based) (2026-03-30)
  - Basic slot: `{{prop_name}}`
  - Derived transforms: `{{prop_name | pluralize}}`, `{{prop_name | camelcase}}`, etc.
  - Conditional block: `{% if prop %}...{% endif %}`
  - Each block: `{% for item in array_prop %}...{% endfor %}` (requires JSON arrays in vars)

### Phase 2: Template CLI Commands

- [x] **3.5** Add `rok init-template` command (2026-03-30)
  - Interactive wizard to create new templates
  - Generates `.rok-template.json` and template files
  - Supports all prop types

- [x] **3.6** Add `rok validate-template` command (2026-03-30)
  - Validates `.rok-template.json` schema
  - Checks template syntax
  - Reports errors with line numbers

- [x] **3.7** Add `rok list-templates` command (2026-03-30)
  - Lists all available templates via `rok templates`
  - Shows: name, description, tags
  - Filter by tags (via grep)

### Phase 3: Template Execution

- [x] **3.8** Update `template` step to use user templates (2026-03-30)
  - New field: `name` (template name)
  - New field: `props` (prop values)
  - Merge with existing `builtin` support

- [x] **3.9** Implement template inheritance (2026-03-30)
  - `extends` field in schema
  - Override specific outputs/props
  - Chain inheritance (A extends B extends C)

- [x] **3.10** Add derived transforms (2026-03-30)
  - `pluralize`, `singularize`
  - `camelcase`, `snakecase`, `kebabcase`, `pascalcase`
  - `uppercase`, `lowercase`
  - `capitalize`
  - All filters work in templates

### Phase 4: Template Discovery API

- [x] **3.11** Add `rok templates` JSON endpoint (2026-03-30)
  - Returns all discoverable templates
  - Includes: name, description, tags, props, outputs
  - Used by agents for template discovery

---

## v4: One JSON. All Changes

### Phase 1: Task Files

- [ ] **4.1** Implement `.rok.json` task file support
  - Top-level schema: name, description, version, author
  - `options`: cwd, stop_on_error, timeout_ms, dry_run, env
  - `props`: reusable variables
  - `steps`: the actual workflow

- [ ] **4.2** Add `rok save <name>` command
  - Saves current payload as named task
  - Stores in `.rok/tasks/<name>.json`

- [ ] **4.3** Add `rok run <task-name>` command
  - Loads and executes saved task
  - Merges with CLI args

- [ ] **4.4** Add `rok list` command
  - Lists saved tasks
  - Shows name, description, last run

- [ ] **4.5** Add `rok edit <task-name>` command
  - Opens task in editor
  - Validates on save

### Phase 2: Enhanced Step Features

- [ ] **4.6** Add `id` field to all steps
  - Referenceable identifier
  - Useful for debugging and logging

- [ ] **4.7** Add `max_bytes` to `read` step
  - Limit file read size
  - Prevents huge file issues

- [ ] **4.8** Add `create_dirs` to `write` step
  - Auto-create parent directories
  - Default: true

- [ ] **4.9** Add `case_sensitive` to `replace` step
  - Option for case-insensitive replace
  - Default: true

- [ ] **4.10** Add `context_lines` to `grep` step
  - Include N lines before/after match
  - Default: 0

- [ ] **4.11** Add `encoding` to `read` step
  - Support different encodings
  - Default: UTF-8

### Phase 3: CLI Enhancements

- [ ] **4.12** Add `rok watch` command
  - Watch for file changes
  - Re-run on change

- [ ] **4.13** Add `rok history` command
  - Show execution history
  - With timing and status

- [ ] **4.14** Add `rok replay <run-id>` command
  - Re-run a previous execution
  - Useful for debugging

### Phase 4: Advanced Features

- [ ] **4.15** Implement step dependencies
  - `depends_on`: [step_id, ...]
  - Wait for dependencies before running

- [ ] **4.16** Add environment variable expansion
  - `{{env.VAR_NAME}}` in any string field
  - Load from `.env` automatically

- [ ] **4.17** Add step timeout override
  - Per-step timeout_ms
  - Override global timeout

- [ ] **4.18** Add step retry logic
  - `retry`: { count, delay_ms, backoff }
  - Auto-retry failed steps

### Phase 5: Polish

- [ ] **4.19** Add colored output for errors/warnings
- [ ] **4.20** Add progress indicator for long operations
- [ ] **4.21** Add `--verbose` flag for debug output
- [ ] **4.22** Add `--quiet` flag to suppress output
- [ ] **4.23** Generate shell completions (bash, zsh, fish)
- [ ] **4.24** Add man pages

---

## Dependencies to Add (v3/v4)

```toml
# Already present
tera = "1"        # Template rendering
inflector = "0.11" # Word transformations
slug = "0.5"      # URL-safe slugs

# New for v3/v4
# (none required - using existing deps)
```

---

## Acceptance Criteria

### Template Engine (3.1-3.4)

| Feature | Criteria |
|---------|----------|
| Template discovery | Finds templates in `.rok/templates/` and `~/.rok/templates/` |
| Schema validation | Validates `.rok-template.json` against spec |
| Prop types | All 5 types (string, enum, boolean, path, array) work |
| Syntax parser | Slots, conditionals, and loops render correctly |

### Template CLI (3.5-3.7)

| Feature | Criteria |
|---------|----------|
| init-template | Creates valid template structure interactively |
| validate-template | Reports errors with line numbers |
| list-templates | Shows all templates with metadata |

### Template Execution (3.8-3.11)

| Feature | Criteria |
|---------|----------|
| User templates | `template` step works with custom templates |
| Inheritance | Child templates inherit from parent correctly |
| Derived transforms | All transforms (pluralize, camelcase, etc.) work |
| Discovery API | `rok templates` returns valid JSON |

### Task Files (4.1-4.5)

| Feature | Criteria |
|---------|----------|
| .rok.json | Parses and executes task files correctly |
| save | Saves current payload to named task |
| run | Loads and executes saved task |
| list | Shows all saved tasks |
| edit | Opens task in editor |

### Enhanced Steps (4.6-4.11)

| Feature | Criteria |
|---------|----------|
| id field | All steps accept and return id |
| max_bytes | read limits file size correctly |
| create_dirs | write creates parent dirs |
| case_sensitive | replace respects case flag |
| context_lines | grep shows surrounding lines |
| encoding | read handles different encodings |

### CLI Enhancements (4.12-4.24)

| Feature | Criteria |
|---------|----------|
| watch | Re-runs on file changes |
| history | Shows past executions |
| replay | Re-runs previous execution |
| Shell completions | Works in bash/zsh/fish |

---

## Progress Log

```
v2.0.0 - 2026-03-30 - feat: Complete rok v2 implementation
  - scan, summarize, extract, diff, patch steps
  - lint, template, snapshot/restore, git, http steps
  - if/each control flow with ref system
  - Fixed JSONPath resolver
  - Added 'as' parameter for each step

v1.0.0 - <date> - Initial release
  - Basic steps: bash, read, write, mv, cp, rm, mkdir
  - grep, replace
  - Parallel execution
```

---

## Quick Reference

### Current Version
```
v0.3.0 (commit e87d7b8)
```

### Scripts Added
- `scripts/publish.sh` - Publish to crates.io and GitHub
- `scripts/install.sh` - Install locally
- `scripts/dev.sh` - Development helper (build, test, clippy, fmt)

### Files Added
- `AGENT.md` - Documentation for AI agents

### Testing Commands

```bash
# Test scan
echo '{"steps":[{"type":"scan","path":"./src","depth":2}]}' | rok -f -

# Test template
echo '{"steps":[{"type":"template","builtin":"react-component","output":"./Test.tsx","vars":{"name":"Test"}}]}' | rok -f -

# Test if/each
echo '{"steps":[{"type":"each","over":["a","b"],"step":{"type":"bash","cmd":"echo {{item}}"}}]}' | rok -f -
```
