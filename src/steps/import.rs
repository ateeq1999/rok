use crate::schema::{StepResult, StepTypeResult};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Instant;

pub fn run(
    path: &str,
    add: &[String],
    remove: &[String],
    organize: bool,
    cwd: &Path,
) -> StepResult {
    let start = Instant::now();
    let full_path = cwd.join(path);

    if !full_path.exists() {
        return StepResult {
            index: 0,
            step_type: StepTypeResult::Import {
                path: full_path.to_string_lossy().to_string(),
                added: vec![],
                removed: vec![],
                organized: false,
            },
            status: "error".to_string(),
            duration_ms: 0,
            stopped_pipeline: None,
        };
    }

    let mut added = Vec::new();
    let mut removed = Vec::new();

    if !add.is_empty() || !remove.is_empty() || organize {
        if let Ok(content) = fs::read_to_string(&full_path) {
            let ext = full_path
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            let (new_content, added_imports, removed_imports) = match ext.as_str() {
                "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => {
                    process_js_ts(&content, add, remove, organize)
                }
                "py" => process_python(&content, add, remove, organize),
                "rs" => process_rust(&content, add, remove, organize),
                "go" => process_go(&content, add, remove, organize),
                _ => (content.clone(), vec![], vec![]),
            };

            if new_content != content {
                let _ = fs::write(&full_path, &new_content);
            }

            added = added_imports;
            removed = removed_imports;
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Import {
            path: full_path.to_string_lossy().to_string(),
            added,
            removed,
            organized: organize,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}

fn process_js_ts(
    content: &str,
    add: &[String],
    remove: &[String],
    organize: bool,
) -> (String, Vec<String>, Vec<String>) {
    let mut lines: Vec<&str> = content.lines().collect();
    let mut added = Vec::new();
    let mut removed = Vec::new();

    let import_prefixes = ["import ", "export "];
    let import_indices: HashSet<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| import_prefixes.iter().any(|p| l.starts_with(p)))
        .map(|(i, _)| i)
        .collect();

    let mut sorted_imports: Vec<usize> = import_indices.iter().copied().collect();

    for imp in remove {
        if let Some(&idx) = sorted_imports.iter().find(|&&i| lines[i].contains(imp)) {
            removed.push(lines[idx].to_string());
            lines.remove(idx);
            sorted_imports = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| import_prefixes.iter().any(|p| l.starts_with(p)))
                .map(|(i, _)| i)
                .collect();
        }
    }

    if let Some(mut idx) = sorted_imports.last().copied() {
        for imp in add {
            if !lines.iter().any(|l| l.contains(imp)) {
                let import_line = if imp.starts_with("import ") || imp.starts_with("export ") {
                    imp.clone()
                } else {
                    format!("import {}", imp)
                };
                let import_clone = import_line.clone();
                lines.insert(idx + 1, Box::leak(import_line.into_boxed_str()));
                added.push(import_clone);
                idx += 1;
            }
        }
    } else if !add.is_empty() {
        let first_non_empty = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
        for imp in add {
            let import_line = if imp.starts_with("import ") || imp.starts_with("export ") {
                imp.clone()
            } else {
                format!("import {}", imp)
            };
            let import_clone = import_line.clone();
            lines.insert(first_non_empty, Box::leak(import_line.into_boxed_str()));
            added.push(import_clone);
        }
    }

    if organize {
        let import_section: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| import_prefixes.iter().any(|p| l.starts_with(p)))
            .map(|(i, _)| i)
            .collect();

        let mut sorted_imports: Vec<usize> = import_section.clone();
        sorted_imports.sort_by(|a, b| {
            let a_external = lines[*a].contains("from '") || lines[*a].contains("from \"");
            let b_external = lines[*b].contains("from '") || lines[*b].contains("from \"");
            match (a_external, b_external) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => lines[*a].cmp(lines[*b]),
            }
        });

        let import_set: HashSet<usize> = import_section.iter().copied().collect();
        let mut new_lines: Vec<&str> = sorted_imports.iter().map(|&i| lines[i]).collect();

        for (i, line) in lines.iter().enumerate() {
            if !import_set.contains(&i) {
                new_lines.push(line);
            }
        }

        lines = new_lines;
    }

    (lines.join("\n"), added, removed)
}

fn process_python(
    content: &str,
    add: &[String],
    remove: &[String],
    organize: bool,
) -> (String, Vec<String>, Vec<String>) {
    let mut lines: Vec<&str> = content.lines().collect();
    let mut added = Vec::new();
    let mut removed = Vec::new();

    let import_indices: HashSet<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.starts_with("import ") || l.starts_with("from "))
        .map(|(i, _)| i)
        .collect();

    let mut sorted_imports: Vec<usize> = import_indices.iter().copied().collect();

    for imp in remove {
        if let Some(&idx) = sorted_imports.iter().find(|&&i| lines[i].contains(imp)) {
            removed.push(lines[idx].to_string());
            lines.remove(idx);
            sorted_imports = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.starts_with("import ") || l.starts_with("from "))
                .map(|(i, _)| i)
                .collect();
        }
    }

    if let Some(mut idx) = sorted_imports.last().copied() {
        for imp in add {
            if !lines.iter().any(|l| l.contains(imp)) {
                let import_line = if imp.starts_with("import ") || imp.starts_with("from ") {
                    imp.clone()
                } else {
                    format!("import {}", imp)
                };
                let import_clone = import_line.clone();
                lines.insert(idx + 1, Box::leak(import_line.into_boxed_str()));
                added.push(import_clone);
                idx += 1;
            }
        }
    }

    if organize {
        let import_section: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.starts_with("import ") || l.starts_with("from "))
            .map(|(i, _)| i)
            .collect();

        let mut sorted_imports: Vec<usize> = import_section.clone();
        sorted_imports.sort_by(|a, b| lines[*a].cmp(lines[*b]));

        let import_set: HashSet<usize> = import_section.iter().copied().collect();
        let mut new_lines: Vec<&str> = sorted_imports.iter().map(|&i| lines[i]).collect();

        for (i, line) in lines.iter().enumerate() {
            if !import_set.contains(&i) {
                new_lines.push(line);
            }
        }

        lines = new_lines;
    }

    (lines.join("\n"), added, removed)
}

fn process_rust(
    content: &str,
    add: &[String],
    remove: &[String],
    organize: bool,
) -> (String, Vec<String>, Vec<String>) {
    let mut lines: Vec<&str> = content.lines().collect();
    let mut added = Vec::new();
    let mut removed = Vec::new();

    let import_prefixes = ["use ", "pub use "];
    let import_indices: HashSet<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| import_prefixes.iter().any(|p| l.starts_with(p)))
        .map(|(i, _)| i)
        .collect();

    let mut sorted_imports: Vec<usize> = import_indices.iter().copied().collect();

    for imp in remove {
        if let Some(&idx) = sorted_imports.iter().find(|&&i| lines[i].contains(imp)) {
            removed.push(lines[idx].to_string());
            lines.remove(idx);
            sorted_imports = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| import_prefixes.iter().any(|p| l.starts_with(p)))
                .map(|(i, _)| i)
                .collect();
        }
    }

    if let Some(mut idx) = sorted_imports.last().copied() {
        for imp in add {
            if !lines.iter().any(|l| l.contains(imp)) {
                let import_line = if imp.starts_with("use ") || imp.starts_with("pub use ") {
                    imp.clone()
                } else {
                    format!("use {}", imp)
                };
                let import_clone = import_line.clone();
                lines.insert(idx + 1, Box::leak(import_line.into_boxed_str()));
                added.push(import_clone);
                idx += 1;
            }
        }
    }

    if organize {
        let import_section: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| import_prefixes.iter().any(|p| l.starts_with(p)))
            .map(|(i, _)| i)
            .collect();

        let mut sorted_imports: Vec<usize> = import_section.clone();
        sorted_imports.sort_by(|a, b| lines[*a].cmp(lines[*b]));

        let import_set: HashSet<usize> = import_section.iter().copied().collect();
        let mut new_lines: Vec<&str> = sorted_imports.iter().map(|&i| lines[i]).collect();

        for (i, line) in lines.iter().enumerate() {
            if !import_set.contains(&i) {
                new_lines.push(line);
            }
        }

        lines = new_lines;
    }

    (lines.join("\n"), added, removed)
}

fn process_go(
    content: &str,
    add: &[String],
    remove: &[String],
    organize: bool,
) -> (String, Vec<String>, Vec<String>) {
    let mut lines: Vec<&str> = content.lines().collect();
    let mut added = Vec::new();
    let mut removed = Vec::new();

    let import_indices: HashSet<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.starts_with("\t\"") || l.starts_with("\""))
        .map(|(i, _)| i)
        .collect();

    let mut sorted_imports: Vec<usize> = import_indices.iter().copied().collect();

    for imp in remove {
        if let Some(&idx) = sorted_imports.iter().find(|&&i| lines[i].contains(imp)) {
            removed.push(lines[idx].to_string());
            lines.remove(idx);
            sorted_imports = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.starts_with("\t\"") || l.starts_with("\""))
                .map(|(i, _)| i)
                .collect();
        }
    }

    if !add.is_empty() {
        if let Some(&block_start) = sorted_imports.first() {
            let block_end = lines.iter().skip(block_start).position(|l| *l == ")");

            if let Some(end_offset) = block_end {
                let end = block_start + end_offset;
                for imp in add {
                    if !lines.iter().any(|l| l.contains(imp)) {
                        let import_line = format!("\t\"{}\"", imp);
                        let import_clone = import_line.clone();
                        lines.insert(end, Box::leak(import_line.into_boxed_str()));
                        added.push(import_clone);
                    }
                }
            }
        }
    }

    if organize {
        let import_section: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.starts_with("\t\"") || l.starts_with("\""))
            .map(|(i, _)| i)
            .collect();

        let mut sorted_imports: Vec<usize> = import_section.clone();
        sorted_imports.sort_by(|a, b| lines[*a].cmp(lines[*b]));

        let import_set: HashSet<usize> = import_section.iter().copied().collect();
        let mut new_lines: Vec<&str> = sorted_imports.iter().map(|&i| lines[i]).collect();

        for (i, line) in lines.iter().enumerate() {
            if !import_set.contains(&i) {
                new_lines.push(line);
            }
        }

        lines = new_lines;
    }

    (lines.join("\n"), added, removed)
}
