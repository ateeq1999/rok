//! JSON comparison helpers.

use serde_json::Value;

/// Return a pretty-printed diff string between two JSON values.
pub fn diff(left: &Value, right: &Value) -> String {
    let l = serde_json::to_string_pretty(left).unwrap_or_default();
    let r = serde_json::to_string_pretty(right).unwrap_or_default();
    if l == r {
        return String::from("(values are equal)");
    }
    format!("left:\n{l}\n\nright:\n{r}")
}

/// Return `true` if `subset` is a structural subset of `superset` — every
/// key/value in `subset` is present and equal in `superset`.
pub fn is_subset(subset: &Value, superset: &Value) -> bool {
    match (subset, superset) {
        (Value::Object(sub), Value::Object(sup)) => {
            sub.iter().all(|(k, v)| {
                sup.get(k).map(|sv| is_subset(v, sv)).unwrap_or(false)
            })
        }
        (a, b) => a == b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn subset_passes() {
        assert!(is_subset(&json!({"a": 1}), &json!({"a": 1, "b": 2})));
    }

    #[test]
    fn subset_fails_on_mismatch() {
        assert!(!is_subset(&json!({"a": 2}), &json!({"a": 1, "b": 2})));
    }

    #[test]
    fn diff_equal_values() {
        let v = json!({"x": 1});
        assert!(diff(&v, &v).contains("equal"));
    }
}
