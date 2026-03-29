use crate::schema::{StepResult, StepTypeResult};
use std::fs;
use std::time::Instant;

pub fn run(path: &str, pick: &[String], cwd: &std::path::Path) -> StepResult {
    let start = Instant::now();

    let full_path = cwd.join(path);
    let content = fs::read_to_string(&full_path).unwrap_or_default();

    let data = if path.ends_with(".json") {
        extract_json(&content, pick)
    } else if path.ends_with(".toml") {
        extract_toml(&content, pick)
    } else if path.ends_with(".yaml") || path.ends_with(".yml") {
        extract_yaml(&content, pick)
    } else if path.ends_with(".env") {
        extract_env(&content, pick)
    } else {
        serde_json::json!({ "raw": content })
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Extract {
            path: path.to_string(),
            data,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}

fn extract_json(content: &str, pick: &[String]) -> serde_json::Value {
    let value: serde_json::Value = match serde_json::from_str(content) {
        Ok(v) => v,
        Err(_) => return serde_json::json!({ "error": "invalid json" }),
    };

    if pick.is_empty() {
        return value;
    }

    let mut result = serde_json::Map::new();
    for key in pick {
        if let Some(v) = value.get(key) {
            result.insert(key.clone(), v.clone());
        }
    }
    serde_json::Value::Object(result)
}

fn extract_toml(content: &str, pick: &[String]) -> serde_json::Value {
    let value: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return serde_json::json!({ "error": "invalid toml" }),
    };

    if pick.is_empty() {
        return serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    }

    let mut result = serde_json::Map::new();
    for key in pick {
        if let Some(v) = value.get(key) {
            if let Ok(json_val) = serde_json::to_value(v) {
                result.insert(key.clone(), json_val);
            }
        }
    }
    serde_json::Value::Object(result)
}

fn extract_yaml(content: &str, pick: &[String]) -> serde_json::Value {
    let value: serde_json::Value = match serde_yaml::from_str(content) {
        Ok(v) => v,
        Err(_) => return serde_json::json!({ "error": "invalid yaml" }),
    };

    if pick.is_empty() {
        return value;
    }

    let mut result = serde_json::Map::new();
    for key in pick {
        if let Some(v) = value.get(key) {
            result.insert(key.clone(), v.clone());
        }
    }
    serde_json::Value::Object(result)
}

fn extract_env(content: &str, pick: &[String]) -> serde_json::Value {
    let mut result = serde_json::Map::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            if pick.is_empty() || pick.contains(&key.to_string()) {
                result.insert(
                    key.to_string(),
                    serde_json::Value::String(value.to_string()),
                );
            }
        }
    }

    serde_json::Value::Object(result)
}
