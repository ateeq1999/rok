use crate::schema::{ScanOutput, StepResult, StepTypeResult};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use walkdir::WalkDir;

pub fn run(
    path: &str,
    depth: usize,
    include: &[String],
    output: &ScanOutput,
    cwd: &std::path::Path,
) -> StepResult {
    let start = Instant::now();

    let full_path = cwd.join(path);

    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    let mut exports: HashMap<String, Vec<String>> = HashMap::new();
    let mut imports_graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut stack = Vec::new();
    let mut entry_points = Vec::new();
    let mut file_count = 0;

    for entry in WalkDir::new(&full_path)
        .max_depth(depth)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        if !entry_path.is_file() {
            continue;
        }

        if let Some(ext) = entry_path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !include.is_empty() && !include.iter().any(|e| e.to_lowercase() == ext_str) {
                continue;
            }

            file_count += 1;

            let rel_path = entry_path
                .strip_prefix(&full_path)
                .unwrap_or(entry_path)
                .to_string_lossy()
                .to_string();

            if let Some(parent) = entry_path.parent() {
                let parent_rel = parent
                    .strip_prefix(&full_path)
                    .unwrap_or(parent)
                    .to_string_lossy()
                    .to_string();
                let parent_key = if parent_rel.is_empty() {
                    ".".to_string()
                } else {
                    parent_rel
                };

                let file_name = entry_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                tree.entry(parent_key).or_default().push(file_name);
            }

            detect_stack(&rel_path, &mut stack);

            if is_entry_point(&rel_path, &ext_str) {
                entry_points.push(rel_path.clone());
            }

            if matches!(output, ScanOutput::Exports | ScanOutput::Full) {
                if let Ok(content) = fs::read_to_string(entry_path) {
                    let file_exports = extract_exports(&content, &ext_str);
                    if !file_exports.is_empty() {
                        exports.insert(rel_path.clone(), file_exports);
                    }
                }
            }

            if matches!(output, ScanOutput::Imports | ScanOutput::Full) {
                if let Ok(content) = fs::read_to_string(entry_path) {
                    let imports = extract_imports(&content, &ext_str);
                    if !imports.is_empty() {
                        imports_graph.insert(rel_path.clone(), imports);
                    }
                }
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Scan {
            path: path.to_string(),
            stack,
            entry_points,
            file_count,
            tree,
            exports,
            imports_graph,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}

fn detect_stack(path: &str, stack: &mut Vec<String>) {
    let path_lower = path.to_lowercase();

    if path_lower.contains("node_modules") {
        return;
    }

    let detectors = [
        ("react", "react"),
        ("vue", "vue"),
        ("svelte", "svelte"),
        ("angular", "angular"),
        ("next", "next.js"),
        ("nuxt", "nuxt"),
        ("vite", "vite"),
        ("webpack", "webpack"),
        ("typescript", "typescript"),
        ("tailwind", "tailwind"),
        ("tanstack", "tanstack"),
        ("trpc", "trpc"),
        ("prisma", "prisma"),
        ("deno", "deno"),
        ("tauri", "tauri"),
        ("electron", "electron"),
    ];

    for (keyword, name) in detectors {
        if path_lower.contains(keyword) && !stack.contains(&name.to_string()) {
            stack.push(name.to_string());
        }
    }
}

fn is_entry_point(path: &str, ext: &str) -> bool {
    let name = std::path::Path::new(path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    matches!(
        (ext, name.as_str()),
        ("ts", "index" | "main" | "app" | "server")
            | ("tsx", "index" | "main" | "app" | "_app")
            | ("js", "index" | "main" | "app")
            | ("rs", "main" | "lib" | "mod")
            | ("go", "main")
            | ("py", "__init__" | "main")
    )
}

fn extract_exports(content: &str, ext: &str) -> Vec<String> {
    let mut exports = Vec::new();

    match ext {
        "ts" | "tsx" | "js" | "jsx" => {
            let re = Regex::new(r#"(?:export\s+(?:default\s+)?(?:const|let|var|function|class|interface|type)\s+(\w+)|export\s+\{\s*([^}]+)\s*\})"#).ok();
            if let Some(re) = re {
                for cap in re.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        exports.push(m.as_str().to_string());
                    } else if let Some(m) = cap.get(2) {
                        for item in m.as_str().split(',') {
                            let item = item.trim();
                            if !item.is_empty() {
                                exports.push(item.to_string());
                            }
                        }
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
