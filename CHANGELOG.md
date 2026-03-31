# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation (CONTRIBUTING.md, ARCHITECTURE.md)
- Unit tests for core modules (refs, schema validation)
- Integration test examples
- Shell completions generation script
- Enhanced error messages with context

### Changed
- Improved error handling throughout the codebase
- Enhanced output formatting with better colors
- Updated dependencies to latest versions

### Fixed
- Edge cases in reference resolution
- Memory leaks in parallel execution
- File path handling on Windows

## [0.8.0] - 2026-03-31

### Added
- **Batch Multi-file Edit**: `replace` step now supports `glob` and `whole_word` fields
- **Import Management**: New `import` step for managing imports in code files
- **Selective File Loading**: `read` step supports `filter_imports`, `filter_exports`, and `since` fields
- **Context Compression**: `summarize` step for extracting key information from files

### Changed
- Enhanced `Replace` step with glob pattern support
- Improved file filtering in `Read` step

## [0.7.0] - 2026-02-15

### Added
- Step dependencies with `depends_on` field
- Environment variable expansion in all string fields
- Timeout override per step
- Retry logic with configurable count, delay, and backoff
- Colored output in pretty mode
- Progress indicator
- Verbose and quiet flags
- Shell completions (bash, zsh, fish)

### Changed
- All steps now support `id` field for referencing
- Enhanced CLI with more intuitive commands

## [0.6.0] - 2026-01-20

### Added
- **Task Files**: Save and reuse tasks
  - `rok save <name>` - Save current payload
  - `rok run <name>` - Run saved task
  - `rok list` - List all tasks
  - `rok edit <name>` - Edit task file
- **Watch Mode**: `rok watch -f file.json` - Watch files and re-run on changes
- **History**: `rok history` - Show execution history
- **Replay**: `rok replay [run_id]` - Replay previous execution
- **Serve**: `rok serve` - Serve documentation locally

### Changed
- Improved execution order with topological sorting
- Enhanced caching system

## [0.5.0] - 2025-12-10

### Added
- **Template System**: Create and use custom templates
  - Built-in templates for common patterns
  - Custom templates in `.rok/templates/`
  - Template props with validation
  - Derived transforms (camelcase, snakecase, etc.)
- **Template Commands**:
  - `rok templates` - List available templates
  - `rok init-template <name>` - Create new template
  - `rok validate-template <path>` - Validate template schema

### Changed
- Refactored template handling into `template_discovery` module

## [0.4.0] - 2025-11-05

### Added
- **Control Flow Steps**:
  - `if` - Conditional execution with conditions
  - `each` - Loop over items
  - `parallel` - Execute steps in parallel
- **Condition Types**:
  - `exists`, `contains`, `grep_has_results`
  - `step_ok`, `step_failed`, `file_changed`
  - `not`, `and`, `or` for compound conditions

### Changed
- Enhanced reference resolution with JSONPath-like syntax
- Improved error handling in control flow

## [0.3.0] - 2025-10-01

### Added
- **Version Control Steps**:
  - `git` - Run git commands
  - `snapshot` - Save file state
  - `restore` - Restore from snapshot
- **Network Step**:
  - `http` - Make HTTP requests
- **Code Analysis**:
  - `lint` - Lint code files
  - `diff` - Compare files
  - `extract` - Extract data from JSON/YAML

### Changed
- Added retry logic for bash commands
- Enhanced output formatting

## [0.2.0] - 2025-09-01

### Added
- **Search Steps**:
  - `grep` - Search for patterns
  - `replace` - Find and replace across files
  - `scan` - Scan directory tree
- **Code Intelligence**:
  - `summarize` - Summarize code structure

### Changed
- Improved file operation performance with rayon
- Enhanced glob pattern matching

## [0.1.0] - 2025-08-01

### Added
- Initial release
- **Core Steps**:
  - `bash` - Execute shell commands
  - `read` - Read files
  - `write` - Write files
  - `mv` - Move files
  - `cp` - Copy files
  - `rm` - Remove files
  - `mkdir` - Create directories
  - `patch` - Apply edits to files

### Changed
- Basic CLI with file and JSON input support

---

## Version History Summary

| Version | Release Date | Key Features |
|---------|-------------|--------------|
| 0.1.0 | 2025-08-01 | Initial release, basic file ops |
| 0.2.0 | 2025-09-01 | Search and code intelligence |
| 0.3.0 | 2025-10-01 | Version control, network, linting |
| 0.4.0 | 2025-11-05 | Control flow (if/each/parallel) |
| 0.5.0 | 2025-12-10 | Template system |
| 0.6.0 | 2026-01-20 | Task files, watch, history |
| 0.7.0 | 2026-02-15 | Dependencies, retries, enhanced CLI |
| 0.8.0 | 2026-03-31 | Import management, batch edits |

## Upcoming Features (v0.9.0)

- [ ] Symbol refactoring across codebase
- [ ] Dependency graph visualization
- [ ] Incremental execution mode
- [ ] Example-based code generation
- [ ] Task chaining and composition
- [ ] Checkpoint/resume for long tasks

## Breaking Changes

### v0.7.0
- All steps now require explicit `id` for referencing (backward compatible with empty string)
- `depends_on` field added to all steps

### v0.5.0
- Template system introduced with new CLI commands
- `.rok/` directory structure changed

---

*For more information, see [README.md](README.md) and [docs/](docs/).*
