//! Config file loading (JSON, TOML, YAML → flattened map).

use std::collections::HashMap;
use std::path::Path;

use crate::error::ConfigError;
use crate::value::Value;

/// Supported configuration file formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}

impl ConfigFormat {
    /// Detect the format from a file extension.  Returns `None` if unknown.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        match path.as_ref().extension()?.to_str()? {
            "json" => Some(Self::Json),
            "toml" => Some(Self::Toml),
            "yaml" | "yml" => Some(Self::Yaml),
            _ => None,
        }
    }
}

impl std::fmt::Display for ConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "JSON"),
            Self::Toml => write!(f, "TOML"),
            Self::Yaml => write!(f, "YAML"),
        }
    }
}

/// Load a config file and return its entries as a flat `key → value` map.
pub fn load_file<P: AsRef<Path>>(
    path: P,
    format: ConfigFormat,
) -> Result<HashMap<String, String>, ConfigError> {
    let path = path.as_ref();
    let raw = std::fs::read_to_string(path).map_err(|e| ConfigError::IoError {
        path: path.display().to_string(),
        source: e,
    })?;

    let value = parse_str(&raw, format, path)?;
    Ok(value.flatten(""))
}

fn parse_str(raw: &str, format: ConfigFormat, path: &Path) -> Result<Value, ConfigError> {
    match format {
        ConfigFormat::Json => {
            let v: serde_json::Value =
                serde_json::from_str(raw).map_err(|e| ConfigError::FormatError {
                    path: path.display().to_string(),
                    format: "JSON".into(),
                    reason: e.to_string(),
                })?;
            Ok(from_json(v))
        }
        ConfigFormat::Toml => {
            let v: toml::Value = toml::from_str(raw).map_err(|e| ConfigError::FormatError {
                path: path.display().to_string(),
                format: "TOML".into(),
                reason: e.to_string(),
            })?;
            Ok(from_toml(v))
        }
        ConfigFormat::Yaml => {
            let v: serde_yaml::Value =
                serde_yaml::from_str(raw).map_err(|e| ConfigError::FormatError {
                    path: path.display().to_string(),
                    format: "YAML".into(),
                    reason: e.to_string(),
                })?;
            Ok(from_yaml(v))
        }
    }
}

// ── adapter functions ─────────────────────────────────────────────────────────

fn from_json(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else {
                Value::Float(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => Value::Array(arr.into_iter().map(from_json).collect()),
        serde_json::Value::Object(obj) => {
            Value::Map(obj.into_iter().map(|(k, v)| (k, from_json(v))).collect())
        }
    }
}

fn from_toml(v: toml::Value) -> Value {
    match v {
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Integer(n) => Value::Int(n),
        toml::Value::Float(f) => Value::Float(f),
        toml::Value::String(s) => Value::String(s),
        toml::Value::Datetime(dt) => Value::String(dt.to_string()),
        toml::Value::Array(arr) => Value::Array(arr.into_iter().map(from_toml).collect()),
        toml::Value::Table(tbl) => {
            Value::Map(tbl.into_iter().map(|(k, v)| (k, from_toml(v))).collect())
        }
    }
}

fn from_yaml(v: serde_yaml::Value) -> Value {
    match v {
        serde_yaml::Value::Null => Value::Null,
        serde_yaml::Value::Bool(b) => Value::Bool(b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else {
                Value::Float(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_yaml::Value::String(s) => Value::String(s),
        serde_yaml::Value::Sequence(arr) => Value::Array(arr.into_iter().map(from_yaml).collect()),
        serde_yaml::Value::Mapping(map) => Value::Map(
            map.into_iter()
                .filter_map(|(k, v)| k.as_str().map(|s| (s.to_string(), from_yaml(v))))
                .collect(),
        ),
        serde_yaml::Value::Tagged(tagged) => from_yaml(tagged.value),
    }
}
