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
    let mut i = 0;

    while i < parts.len() {
        let part = parts[i];
        if part.is_empty() {
            i += 1;
            continue;
        }

        if let Some((key, rest)) = part.split_once('[') {
            if !key.is_empty() {
                current = current.get(key)?.clone();
            }

            let bracket_part = rest.trim_end_matches(']');
            if bracket_part == "*" {
                if let Some(arr) = current.as_array() {
                    current = serde_json::json!(arr.clone());
                }
            } else if let Ok(arr_idx) = bracket_part.parse::<usize>() {
                current = current.get(arr_idx)?.clone();
            }
            i += 1;
            if !rest.ends_with(']') && i < parts.len() {
                let remaining = parts[i..].join(".");
                if let Some((arr_key, arr_rest)) = remaining.split_once('[') {
                    if arr_key == "*" {
                        if let Some(arr) = current.as_array() {
                            let extracted: Vec<_> = arr
                                .iter()
                                .filter_map(|v| v.get(arr_rest.trim_end_matches(']')))
                                .cloned()
                                .collect();
                            return Some(serde_json::Value::Array(extracted));
                        }
                    }
                }
            }
        } else if part == "*" {
            if let Some(arr) = current.as_array() {
                let extracted: Vec<_> = arr
                    .iter()
                    .map(|v| {
                        if let Some(obj) = v.as_object() {
                            serde_json::Value::Object(obj.clone())
                        } else {
                            v.clone()
                        }
                    })
                    .collect();
                return Some(serde_json::Value::Array(extracted));
            }
        } else if part.contains('[') {
            if let Some((arr_key, arr_idx_str)) = part.split_once('[') {
                let arr_idx = arr_idx_str.trim_end_matches(']');
                if arr_key.is_empty() {
                    if let Some(arr) = current.as_array() {
                        if arr_idx == "*" {
                            let extracted: Vec<_> = arr
                                .iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect();
                            return Some(serde_json::Value::Array(
                                extracted
                                    .into_iter()
                                    .map(serde_json::Value::String)
                                    .collect(),
                            ));
                        }
                    }
                }
            }
        } else {
            current = current.get(part)?.clone();
        }
        i += 1;
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
