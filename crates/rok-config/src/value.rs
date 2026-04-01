//! Intermediate value type used during config file parsing.

use std::collections::HashMap;

/// A loosely typed config value returned from parsed files.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Value {
    /// Flatten a [`Value::Map`] into dot-separated `key → string` pairs.
    pub fn flatten(self, prefix: &str) -> HashMap<String, String> {
        let mut out = HashMap::new();
        self.flatten_into(prefix, &mut out);
        out
    }

    fn flatten_into(self, prefix: &str, out: &mut HashMap<String, String>) {
        match self {
            Value::Map(map) => {
                for (k, v) in map {
                    let key = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{prefix}.{k}")
                    };
                    v.flatten_into(&key, out);
                }
            }
            Value::Array(arr) => {
                for (i, v) in arr.into_iter().enumerate() {
                    let key = format!("{prefix}.{i}");
                    v.flatten_into(&key, out);
                }
            }
            other => {
                out.insert(prefix.to_string(), other.to_string_value());
            }
        }
    }

    fn to_string_value(self) -> String {
        match self {
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Int(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s,
            Value::Array(_) | Value::Map(_) => unreachable!("flattened before reaching here"),
        }
    }
}
