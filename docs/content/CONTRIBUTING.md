# Contributing to rok

Thank you for your interest in contributing to rok! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Adding New Steps](#adding-new-steps)
- [Testing Guidelines](#testing-guidelines)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Welcome newcomers and help them learn

## Getting Started

1. **Fork the repository**
2. **Clone your fork**: `git clone https://github.com/YOUR_USERNAME/rok.git`
3. **Create a branch**: `git checkout -b feature/your-feature-name`
4. **Make your changes**
5. **Test thoroughly**
6. **Submit a PR**

## Development Setup

### Prerequisites

- Rust 1.70 or later (`rustup install stable`)
- Git
- A code editor (VS Code, RustRover, etc.)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/ateeq1999/rok.git
cd rok

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with verbose output
cargo run -- -f examples/task.json --verbose
```

### Development Commands

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Run all tests
cargo test --all

# Check documentation
cargo doc --no-deps
```

## Project Structure

```
rok/
├── src/
│   ├── main.rs              # CLI entry point and command handling
│   ├── cli.rs               # CLI argument parsing (clap)
│   ├── config.rs            # Configuration management
│   ├── error.rs             # Error types and handling
│   ├── output.rs            # Output formatting
│   ├── refs.rs              # Reference resolution between steps
│   ├── runner.rs            # Core execution engine
│   ├── schema.rs            # JSON schema definitions (serde)
│   └── steps/               # Step implementations
│       ├── mod.rs           # Step module exports
│       ├── bash.rs          # Bash command execution
│       ├── read.rs          # File reading
│       ├── write.rs         # File writing
│       ├── grep.rs          # Pattern searching
│       ├── replace.rs       # Find/replace across files
│       ├── scan.rs          # Directory scanning
│       ├── template.rs      # Template rendering
│       └── ...              # Other step types
├── docs/                    # Documentation files
├── examples/                # Example task files
├── scripts/                 # Utility scripts
├── .rok/                    # Runtime data (tasks, snapshots, cache)
│   ├── tasks/               # Saved tasks
│   ├── snapshots/           # File snapshots
│   └── templates/           # Custom templates
└── tests/                   # Integration tests
```

## Adding New Steps

### Step 1: Define the Schema

Add your step variant to `src/schema.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Step {
    // ... existing steps ...
    
    YourStep {
        #[serde(default)]
        id: String,
        #[serde(default = "default_depends_on")]
        depends_on: Vec<String>,
        // Your fields here
        path: String,
        option: Option<String>,
    },
}
```

### Step 2: Implement the Step

Create `src/steps/your_step.rs`:

```rust
use crate::schema::{StepResult, StepTypeResult};
use std::time::Instant;

pub fn run(path: &str, option: Option<&str>, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();
    
    // Your implementation here
    
    let duration_ms = start.elapsed().as_millis() as u64;
    
    StepResult {
        index: 0,
        step_type: StepTypeResult::YourStep {
            path: path.to_string(),
            // Your result fields
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}
```

### Step 3: Register the Step

Update `src/steps/mod.rs`:

```rust
pub mod your_step;
```

Update `src/runner.rs` in the `execute_step` method:

```rust
Step::YourStep { path, option, .. } => {
    let mut result = crate::steps::your_step::run(path, option.as_deref(), &self.config.cwd);
    result.index = index;
    result
}
```

Update `src/main.rs` in the dry-run section:

```rust
schema::Step::YourStep { path, .. } => {
    format!("  {}: your_step {}", i, path)
}
```

### Step 4: Add Documentation

Update `docs/api.md` with your step's documentation.

### Step 5: Add Tests

Create tests in `src/steps/your_step.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_basic_functionality() {
        // Your test here
    }
}
```

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_success_case() {
        assert_eq!(some_function(), expected_result);
    }
    
    #[test]
    fn test_error_case() {
        let result = some_function();
        assert!(result.is_err());
    }
}
```

### Integration Tests

Create `tests/your_feature.rs`:

```rust
use assert_cmd::Command;
use std::fs;

#[test]
fn test_your_feature() {
    let mut cmd = Command::cargo_bin("rok").unwrap();
    cmd.arg("-j")
        .arg(r#"{"steps":[{"type":"your_step","path":"."}]}"#);
    cmd.assert().success();
}
```

## Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Build/config changes

### Examples

```
feat(steps): add new your_step type
fix(runner): handle edge case in parallel execution
docs: update API documentation
refactor(schema): simplify step enum structure
test: add unit tests for refs module
```

## Pull Request Process

1. **Update documentation** if adding features
2. **Add tests** for new functionality
3. **Run clippy**: `cargo clippy -- -D warnings`
4. **Format code**: `cargo fmt`
5. **Ensure tests pass**: `cargo test`
6. **Update CHANGELOG.md** if applicable
7. **Request review** from maintainers

## Release Process

Releases follow semantic versioning:

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Update documentation
- [ ] Run all tests
- [ ] Create git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
- [ ] Push tag: `git push origin v0.1.0`
- [ ] Publish to crates.io: `cargo publish`

## Questions?

- Open an issue for bugs or feature requests
- Join discussions in existing issues
- Check existing documentation first

Thank you for contributing to rok! 🚀
