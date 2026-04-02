use crate::schema::{StepResult, StepTypeResult};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use walkdir::WalkDir;

pub fn run(
    path: &str,
    depth: usize,
    include: &[String],
    focus: Option<&str>,
    cwd: &std::path::Path,
) -> StepResult {
    let start = Instant::now();
    let full_path = cwd.join(path);

    let files: Vec<_> = WalkDir::new(&full_path)
        .max_depth(depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            let ext = e
                .path()
                .extension()
                .map(|ex| ex.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            if include.is_empty() {
                matches!(
                    ext.as_str(),
                    "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" | "py" | "rs" | "go"
                )
            } else {
                include.iter().any(|i| i.to_lowercase() == ext)
            }
        })
        .collect();

    let file_count = files.len();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

    for entry in &files {
        let entry_path = entry.path();
        let relative = entry_path
            .strip_prefix(cwd)
            .unwrap_or(entry_path)
            .to_string_lossy()
            .to_string()
            .replace('\\', "/");

        graph.entry(relative.clone()).or_default();

        if let Ok(content) = fs::read_to_string(entry_path) {
            let ext = entry_path
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            let imports = match ext.as_str() {
                "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => extract_js_imports(&content),
                "py" => extract_python_imports(&content),
                "rs" => extract_rust_imports(&content),
                "go" => extract_go_imports(&content),
                _ => vec![],
            };

            for imp in imports {
                graph.entry(relative.clone()).or_default().push(imp.clone());
                dependents.entry(imp).or_default().push(relative.clone());
            }
        }
    }

    // Detect cycles using DFS
    let cycles = detect_cycles(&graph);

    // If focus is specified, filter to only show relevant files
    if let Some(focus_file) = focus {
        let focus_str = focus_file.replace('\\', "/");
        let direct_deps = graph.get(&focus_str).cloned().unwrap_or_default();
        let direct_dependents = dependents.get(&focus_str).cloned().unwrap_or_default();

        let mut filtered_graph: HashMap<String, Vec<String>> = HashMap::new();
        filtered_graph.insert(focus_str.clone(), direct_deps);
        for dep in &direct_dependents {
            filtered_graph.insert(dep.clone(), vec![focus_str.clone()]);
        }

        let duration_ms = start.elapsed().as_millis() as u64;
        return StepResult {
            index: 0,
            step_type: StepTypeResult::Deps {
                path: path.to_string(),
                graph: filtered_graph,
                dependents: dependents
                    .into_iter()
                    .filter(|(k, _)| k == &focus_str)
                    .collect(),
                file_count,
                cycles,
            },
            status: "ok".to_string(),
            duration_ms,
            stopped_pipeline: None,
        };
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Deps {
            path: path.to_string(),
            graph,
            dependents,
            file_count,
            cycles,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}

fn extract_js_imports(content: &str) -> Vec<String> {
    let re = Regex::new(r#"(?:import|from)\s+['"]([^'"]+)['"]"#).unwrap();
    re.captures_iter(content)
        .map(|c| c[1].to_string())
        .collect()
}

fn extract_python_imports(content: &str) -> Vec<String> {
    let re = Regex::new(r"(?:^import\s+(\S+)|^from\s+(\S+)\s+import)").unwrap();
    content
        .lines()
        .flat_map(|line| {
            re.captures(line).map(|c| {
                c.get(1)
                    .or_else(|| c.get(2))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default()
            })
        })
        .filter(|s| !s.is_empty())
        .collect()
}

fn extract_rust_imports(content: &str) -> Vec<String> {
    let re = Regex::new(r"^use\s+([a-zA-Z_][a-zA-Z0-9_:]*(?:::[a-zA-Z_*{][^;]*)?)").unwrap();
    content
        .lines()
        .flat_map(|line| {
            re.captures(line)
                .map(|c| c[1].split("::").next().unwrap_or("").to_string())
        })
        .filter(|s| !s.is_empty() && s != "crate" && s != "super" && s != "self")
        .collect()
}

fn extract_go_imports(content: &str) -> Vec<String> {
    let re = Regex::new(r#"^\s*(?:\w+\s+)?["']([^"']+)["']"#).unwrap();
    content
        .lines()
        .flat_map(|line| re.captures(line).map(|c| c[1].to_string()))
        .collect()
}

fn detect_cycles(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut cycles = Vec::new();
    let mut visited: HashMap<&str, bool> = HashMap::new();
    let mut path: Vec<String> = Vec::new();

    for node in graph.keys() {
        if !visited.contains_key(node.as_str()) {
            dfs_cycle(node, graph, &mut visited, &mut path, &mut cycles);
        }
    }

    cycles
}

fn dfs_cycle<'a>(
    node: &'a str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashMap<&'a str, bool>,
    path: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    // Only detect simple 2-node cycles (A imports B and B imports A) for efficiency
    if let Some(deps) = graph.get(node) {
        for dep in deps {
            if let Some(dep_deps) = graph.get(dep.as_str()) {
                if dep_deps.contains(&node.to_string())
                    && !cycles
                        .iter()
                        .any(|c: &Vec<String>| c.contains(&node.to_string()) && c.contains(dep))
                {
                    cycles.push(vec![node.to_string(), dep.clone()]);
                }
            }
        }
    }
    visited.insert(node, true);
    let _ = path;
}
