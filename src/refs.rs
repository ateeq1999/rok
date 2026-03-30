use crate::schema::{StepResult, StepTypeResult};
use serde_json::Value;

pub fn resolve_ref(step_index: usize, pick: &str, results: &[StepResult]) -> Option<Value> {
    let step = results.get(step_index)?;

    let value = match &step.step_type {
        StepTypeResult::Bash {
            stdout,
            stderr,
            exit_code,
            ..
        } => {
            serde_json::json!({
                "stdout": stdout,
                "stderr": stderr,
                "exit_code": exit_code,
            })
        }
        StepTypeResult::Read { files, .. } => {
            serde_json::json!({ "files": files })
        }
        StepTypeResult::Grep { matches, .. } => {
            serde_json::json!({ "matches": matches })
        }
        StepTypeResult::Replace {
            files_modified,
            total_replacements,
            ..
        } => {
            serde_json::json!({
                "files_modified": files_modified,
                "total_replacements": total_replacements,
            })
        }
        StepTypeResult::Scan { .. } => {
            serde_json::json!({ "scan": true })
        }
        StepTypeResult::Summarize { summary, .. } => {
            serde_json::json!({ "summary": summary })
        }
        StepTypeResult::Extract { data, .. } => data.clone(),
        StepTypeResult::Diff {
            added,
            removed,
            is_identical,
            ..
        } => {
            serde_json::json!({
                "added": added,
                "removed": removed,
                "is_identical": is_identical,
            })
        }
        StepTypeResult::Lint {
            errors_count,
            warnings_count,
            errors,
            ..
        } => {
            serde_json::json!({
                "errors_count": errors_count,
                "warnings_count": warnings_count,
                "errors": errors,
            })
        }
        StepTypeResult::Http { status, body, .. } => {
            serde_json::json!({ "status": status, "body": body })
        }
        StepTypeResult::If {
            condition_met,
            results,
            ..
        } => {
            serde_json::json!({
                "condition_met": condition_met,
                "results": results,
            })
        }
        StepTypeResult::Each { items, results, .. } => {
            serde_json::json!({
                "items": items,
                "results": results,
            })
        }
        StepTypeResult::Parallel { results, .. } => {
            serde_json::json!({ "results": results })
        }
        _ => serde_json::json!({ "result": step }),
    };

    if pick.is_empty() || pick == "*" {
        return Some(value);
    }

    resolve_jsonpath(&value, pick)
}

fn resolve_jsonpath(value: &Value, path: &str) -> Option<Value> {
    let mut current = value.clone();

    let parts: Vec<&str> = path.split('.').collect();

    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }

        if part.contains('[') || part.contains('*') {
            let key = part.split('[').next().unwrap_or("");
            let rest = part.strip_prefix(key).unwrap_or("");

            if !key.is_empty() {
                if let Some(v) = current.get(key) {
                    current = v.clone();
                } else {
                    return None;
                }
            }

            if rest.is_empty() {
                continue;
            }

            let bracket_content = rest.trim_start_matches('[').trim_end_matches(']');

            if current.is_array() {
                if bracket_content == "*" {
                    let arr = current.as_array().unwrap();
                    let mut extracted = Vec::new();

                    if i + 1 < parts.len() {
                        let next_part = parts[i + 1];
                        for item in arr {
                            if let Some(v) = item.get(next_part) {
                                extracted.push(v.clone());
                            }
                        }
                        return Some(serde_json::Value::Array(extracted));
                    } else {
                        return Some(current.clone());
                    }
                } else if let Ok(idx) = bracket_content.parse::<usize>() {
                    if let Some(item) = current.get(idx) {
                        current = item.clone();
                    } else {
                        return None;
                    }
                }
            }
        } else if let Some(v) = current.get(*part) {
            current = v.clone();
        } else {
            return None;
        }
    }

    Some(current)
}

pub fn has_grep_results(step_index: usize, results: &[StepResult]) -> bool {
    if let Some(step) = results.get(step_index) {
        if let StepTypeResult::Grep { matches, .. } = &step.step_type {
            return !matches.is_empty();
        }
    }
    false
}

pub fn step_ok(step_index: usize, results: &[StepResult]) -> bool {
    results
        .get(step_index)
        .map(|s| s.status == "ok")
        .unwrap_or(false)
}

pub fn step_failed(step_index: usize, results: &[StepResult]) -> bool {
    results
        .get(step_index)
        .map(|s| s.status == "error")
        .unwrap_or(false)
}

pub fn substitute_vars(template: &str, var_name: &str, value: &str) -> String {
    let pattern = format!("{{{{{}}}}}", var_name);
    template.replace(&pattern, value)
}

pub fn substitute_env_vars(template: &str) -> String {
    let re = regex::Regex::new(r"\{\{env\.(\w+)\}\}").unwrap();
    re.replace_all(template, |caps: &regex::Captures| {
        let var_name = &caps[1];
        std::env::var(var_name).unwrap_or_else(|_| caps[0].to_string())
    })
    .to_string()
}
