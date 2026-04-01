use crate::schema::{DeadCodeIssue, StepResult, StepTypeResult};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::Instant;

pub fn run(path: &str, include: &[String], cwd: &Path) -> StepResult {
    let start = Instant::now();
    let full_path = cwd.join(path);

    if !full_path.exists() {
        return StepResult {
            index: 0,
            step_type: StepTypeResult::DeadCode {
                path: full_path.to_string_lossy().to_string(),
                unused_functions: vec![],
                unused_variables: vec![],
                unused_imports: vec![],
                unreachable_code: vec![],
            },
            status: "error".to_string(),
            duration_ms: 0,
            stopped_pipeline: None,
        };
    }

    let mut unused_functions = Vec::new();
    let unused_variables = Vec::new();
    let mut unused_imports = Vec::new();
    let mut unreachable_code = Vec::new();

    // Collect all files
    let mut files = Vec::new();
    collect_files(&full_path, include, &mut files);

    // Parse all files to build a symbol table
    let mut defined_symbols: HashMap<String, Vec<Symbol>> = HashMap::new();
    let mut used_symbols: HashSet<String> = HashSet::new();
    let mut all_imports: HashMap<String, Vec<ImportInfo>> = HashMap::new();

    for file in &files {
        if let Ok(content) = fs::read_to_string(file) {
            let ext = file
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            let rel_path = file
                .strip_prefix(cwd)
                .unwrap_or(file)
                .to_string_lossy()
                .to_string();

            // Extract definitions
            let symbols = extract_definitions(&content, &ext, &rel_path);
            for symbol in symbols {
                defined_symbols
                    .entry(symbol.name.clone())
                    .or_default()
                    .push(symbol);
            }

            // Extract usages
            let usages = extract_usages(&content, &ext);
            used_symbols.extend(usages);

            // Extract imports
            let imports = extract_imports(&content, &ext, &rel_path);
            all_imports.insert(rel_path, imports);
        }
    }

    // Find unused functions
    for (name, definitions) in &defined_symbols {
        if definitions.iter().any(|d| d.kind == "function") && !used_symbols.contains(name) {
            for def in definitions {
                if def.kind == "function" {
                    unused_functions.push(DeadCodeIssue {
                        file: def.file.clone(),
                        line: def.line,
                        symbol: name.clone(),
                        kind: "function".to_string(),
                        message: format!("Function '{}' is defined but never used", name),
                    });
                }
            }
        }
    }

    // Find unused imports
    for (file, imports) in &all_imports {
        for import in imports {
            if !used_symbols.contains(&import.symbol) {
                unused_imports.push(DeadCodeIssue {
                    file: file.clone(),
                    line: import.line,
                    symbol: import.symbol.clone(),
                    kind: "import".to_string(),
                    message: format!("Import '{}' is unused", import.symbol),
                });
            }
        }
    }

    // Find unreachable code patterns
    for file in &files {
        if let Ok(content) = fs::read_to_string(file) {
            let rel_path = file
                .strip_prefix(cwd)
                .unwrap_or(file)
                .to_string_lossy()
                .to_string();

            let issues = find_unreachable_code(&content, &rel_path);
            unreachable_code.extend(issues);
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::DeadCode {
            path: full_path.to_string_lossy().to_string(),
            unused_functions,
            unused_variables,
            unused_imports,
            unreachable_code,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    kind: String,
    file: String,
    line: usize,
}

#[derive(Debug, Clone)]
struct ImportInfo {
    symbol: String,
    line: usize,
}

fn collect_files(path: &Path, include: &[String], files: &mut Vec<std::path::PathBuf>) {
    if path.is_file() {
        files.push(path.to_path_buf());
        return;
    }

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let ext = entry
            .path()
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        if include.is_empty() || include.iter().any(|i| i == &ext) {
            files.push(entry.path().to_path_buf());
        }
    }
}

fn extract_definitions(content: &str, ext: &str, file: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    match ext {
        "js" | "jsx" | "ts" | "tsx" => {
            // Function declarations
            let func_re = Regex::new(r"(?m)^(?:export\s+)?(?:async\s+)?function\s+(\w+)").unwrap();
            for cap in func_re.captures_iter(content) {
                let line = content[..cap.get(0).unwrap().start()].lines().count() + 1;
                symbols.push(Symbol {
                    name: cap[1].to_string(),
                    kind: "function".to_string(),
                    file: file.to_string(),
                    line,
                });
            }

            // Arrow functions and const
            let const_re = Regex::new(r"(?m)^(?:export\s+)?const\s+(\w+)\s*=").unwrap();
            for cap in const_re.captures_iter(content) {
                let line = content[..cap.get(0).unwrap().start()].lines().count() + 1;
                symbols.push(Symbol {
                    name: cap[1].to_string(),
                    kind: "variable".to_string(),
                    file: file.to_string(),
                    line,
                });
            }
        }
        "py" => {
            let func_re = Regex::new(r"(?m)^def\s+(\w+)\s*\(").unwrap();
            for cap in func_re.captures_iter(content) {
                let line = content[..cap.get(0).unwrap().start()].lines().count() + 1;
                symbols.push(Symbol {
                    name: cap[1].to_string(),
                    kind: "function".to_string(),
                    file: file.to_string(),
                    line,
                });
            }
        }
        "rs" => {
            let func_re = Regex::new(r"(?m)^(?:pub\s+)?fn\s+(\w+)").unwrap();
            for cap in func_re.captures_iter(content) {
                let line = content[..cap.get(0).unwrap().start()].lines().count() + 1;
                symbols.push(Symbol {
                    name: cap[1].to_string(),
                    kind: "function".to_string(),
                    file: file.to_string(),
                    line,
                });
            }
        }
        "go" => {
            let func_re = Regex::new(r"(?m)^func\s+(?:\([^)]+\)\s+)?(\w+)").unwrap();
            for cap in func_re.captures_iter(content) {
                let line = content[..cap.get(0).unwrap().start()].lines().count() + 1;
                symbols.push(Symbol {
                    name: cap[1].to_string(),
                    kind: "function".to_string(),
                    file: file.to_string(),
                    line,
                });
            }
        }
        _ => {}
    }

    symbols
}

fn extract_usages(content: &str, _ext: &str) -> Vec<String> {
    let mut usages = HashSet::new();

    // Simple word boundary matching for identifiers
    let word_re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    for cap in word_re.captures_iter(content) {
        let word = &cap[1];
        // Skip keywords
        if !is_keyword(word) {
            usages.insert(word.to_string());
        }
    }

    usages.into_iter().collect()
}

fn extract_imports(content: &str, ext: &str, _file: &str) -> Vec<ImportInfo> {
    let mut imports = Vec::new();

    match ext {
        "js" | "jsx" | "ts" | "tsx" => {
            let import_re = Regex::new(r"import\s+(?:\{([^}]+)\}|(\w+))\s+from").unwrap();
            for cap in import_re.captures_iter(content) {
                let line = content[..cap.get(0).unwrap().start()].lines().count() + 1;
                if let Some(named) = cap.get(1) {
                    for symbol in named.as_str().split(',') {
                        let symbol = symbol.split_whitespace().next().unwrap_or("").to_string();
                        if !symbol.is_empty() {
                            imports.push(ImportInfo { symbol, line });
                        }
                    }
                }
                if let Some(default) = cap.get(2) {
                    imports.push(ImportInfo {
                        symbol: default.as_str().to_string(),
                        line,
                    });
                }
            }
        }
        "py" => {
            let import_re = Regex::new(r"(?:from\s+\S+\s+)?import\s+(.+)").unwrap();
            for cap in import_re.captures_iter(content) {
                let line = content[..cap.get(0).unwrap().start()].lines().count() + 1;
                for symbol in cap[1].split(',') {
                    let symbol = symbol.split_whitespace().next().unwrap_or("").to_string();
                    if !symbol.is_empty() && symbol != "import" {
                        imports.push(ImportInfo { symbol, line });
                    }
                }
            }
        }
        "rs" => {
            let use_re = Regex::new(r"use\s+(?:crate::)?(?:\{([^}]+)\}|(\w+))").unwrap();
            for cap in use_re.captures_iter(content) {
                let line = content[..cap.get(0).unwrap().start()].lines().count() + 1;
                if let Some(named) = cap.get(1) {
                    for symbol in named.as_str().split(',') {
                        let symbol = symbol.split_whitespace().next().unwrap_or("").to_string();
                        if !symbol.is_empty() {
                            imports.push(ImportInfo { symbol, line });
                        }
                    }
                }
            }
        }
        _ => {}
    }

    imports
}

fn find_unreachable_code(content: &str, file: &str) -> Vec<DeadCodeIssue> {
    let mut issues = Vec::new();

    // Check for code after return statements
    let return_re = Regex::new(r"(?m)^\s*return\b[^;]*;").unwrap();
    for cap in return_re.captures_iter(content) {
        let m = cap.get(0).unwrap();
        let line = content[..m.start()].lines().count() + 1;
        let after_return = &content[m.end()..];
        if let Some(next_line) = after_return.lines().next() {
            let trimmed: &str = next_line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("//") {
                issues.push(DeadCodeIssue {
                    file: file.to_string(),
                    line: line + 1,
                    symbol: "unreachable".to_string(),
                    kind: "unreachable".to_string(),
                    message: "Code after return statement may be unreachable".to_string(),
                });
            }
        }
    }

    // Check for code after throw/panic
    let throw_re = Regex::new(r"(?m)^\s*(?:throw|panic|panic!)\b[^;]*;").unwrap();
    for cap in throw_re.captures_iter(content) {
        let m = cap.get(0).unwrap();
        let line = content[..m.start()].lines().count() + 1;
        let after_throw = &content[m.end()..];
        if let Some(next_line) = after_throw.lines().next() {
            let trimmed: &str = next_line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("//") {
                issues.push(DeadCodeIssue {
                    file: file.to_string(),
                    line: line + 1,
                    symbol: "unreachable".to_string(),
                    kind: "unreachable".to_string(),
                    message: "Code after throw/panic may be unreachable".to_string(),
                });
            }
        }
    }

    issues
}

fn is_keyword(word: &str) -> bool {
    let keywords = [
        // JavaScript/TypeScript
        "if",
        "else",
        "for",
        "while",
        "return",
        "function",
        "const",
        "let",
        "var",
        "import",
        "export",
        "from",
        "class",
        "extends",
        "new",
        "this",
        "typeof",
        "instanceof",
        "in",
        "of",
        "try",
        "catch",
        "finally",
        "throw",
        "async",
        "await",
        "yield",
        "switch",
        "case",
        "default",
        "break",
        "continue",
        "do",
        "with",
        "void",
        "delete",
        "true",
        "false",
        "null",
        "undefined",
        // Python
        "def",
        "as",
        "pass",
        "raise",
        "except",
        "lambda",
        // Rust
        "fn",
        "mut",
        "pub",
        "mod",
        "use",
        "struct",
        "enum",
        "impl",
        "trait",
        "match",
        "loop",
        "Some",
        "None",
        "Ok",
        "Err",
        // Go
        "func",
        "package",
        "type",
        "interface",
        "map",
        "range",
    ];
    keywords.contains(&word)
}
