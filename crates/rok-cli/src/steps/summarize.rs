use crate::schema::{FileSummary, StepResult, StepTypeResult};
use regex::Regex;
use std::fs;
use std::time::Instant;

pub fn run(path: &str, _focus: &str, cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let full_path = cwd.join(path);
    let content = fs::read_to_string(&full_path).unwrap_or_default();
    let line_count = content.lines().count();

    let metadata = fs::metadata(&full_path).ok();
    let last_modified = metadata
        .and_then(|m| m.modified().ok())
        .map(|t| {
            let datetime: chrono::DateTime<chrono::Utc> = t.into();
            datetime.to_rfc3339()
        })
        .unwrap_or_else(|| "unknown".to_string());

    let ext = full_path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let imports = extract_imports(&content, &ext);
    let exports = extract_exports(&content, &ext);
    let functions = extract_functions(&content, &ext);
    let types_used = extract_types(&content, &ext);

    let summary = FileSummary {
        imports,
        exports,
        functions,
        types_used,
        line_count,
        last_modified,
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Summarize {
            path: path.to_string(),
            summary,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}

fn extract_imports(content: &str, ext: &str) -> Vec<String> {
    let mut imports = Vec::new();

    match ext {
        "ts" | "tsx" | "js" | "jsx" => {
            let re = Regex::new(r#"import\s+(?:[\w{}\s,*]+\s+from\s+)?['"]([^'"]+)['"]"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        imports.push(m.as_str().to_string());
                    }
                }
            }
        }
        "rs" => {
            let re = Regex::new(r#"use\s+([\w:]+)"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        imports.push(m.as_str().to_string());
                    }
                }
            }
        }
        _ => {}
    }

    imports
}

fn extract_exports(content: &str, ext: &str) -> Vec<String> {
    let mut exports = Vec::new();

    match ext {
        "ts" | "tsx" | "js" | "jsx" => {
            let re = Regex::new(r#"export\s+(?:default\s+)?(?:const|let|var|function|class|interface|type)\s+(\w+)"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        exports.push(m.as_str().to_string());
                    }
                }
            }
            let default_re = Regex::new(r#"export\s+default\s+(?:function\s+)?(\w+)"#).ok();
            if let Some(re) = default_re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        exports.push(format!("{} (default)", m.as_str()));
                    }
                }
            }
        }
        "rs" => {
            let re =
                Regex::new(r#"(?:pub\s+)?(?:fn|struct|enum|trait|impl|const|mod)\s+(\w+)"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        exports.push(m.as_str().to_string());
                    }
                }
            }
        }
        _ => {}
    }

    exports
}

fn extract_functions(content: &str, ext: &str) -> Vec<String> {
    let mut functions = Vec::new();

    match ext {
        "ts" | "tsx" | "js" | "jsx" => {
            let re = Regex::new(r#"(?:function\s+(\w+)|(?:const|let|var)\s+(\w+)\s*=\s*(?:async\s*)?\(|(\w+)\s*:\s*(?:Promise<)?[^>]*>?\s*\(|\b(\w+)\s*\([^)]*\)\s*\{)"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    for i in 1..=4 {
                        if let Some(m) = cap.get(i) {
                            let name = m.as_str();
                            if !name.is_empty()
                                && !["if", "for", "while", "switch", "return"].contains(&name)
                            {
                                functions.push(format!("{}()", name));
                            }
                        }
                    }
                }
            }
        }
        "rs" => {
            let re = Regex::new(r#"(?:pub\s+)?fn\s+(\w+)"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        functions.push(format!("{}()", m.as_str()));
                    }
                }
            }
        }
        _ => {}
    }

    functions
}

fn extract_types(content: &str, ext: &str) -> Vec<String> {
    let mut types = Vec::new();

    match ext {
        "ts" | "tsx" => {
            let re = Regex::new(r#"(?:interface|type)\s+(\w+)"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        types.push(m.as_str().to_string());
                    }
                }
            }
        }
        "rs" => {
            let re = Regex::new(r#"(?:struct|enum)\s+(\w+)"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        types.push(m.as_str().to_string());
                    }
                }
            }
        }
        _ => {}
    }

    types
}
