//! Environment-variable merging.

use std::collections::HashMap;

/// Read all environment variables whose names start with `prefix`, strip the
/// prefix, convert to `lower.dot.notation`, and return as a flat map.
///
/// `APP_SERVER__PORT=3000` with prefix `"APP_"` becomes `"server.port"`.
///
/// Double-underscore (`__`) is used as the nested-key separator so that single
/// underscores can appear inside key names.
pub fn load_env(prefix: &str) -> HashMap<String, String> {
    let prefix_upper = prefix.to_uppercase();
    std::env::vars()
        .filter_map(|(k, v)| {
            let upper = k.to_uppercase();
            upper.strip_prefix(&prefix_upper).map(|rest| {
                let key = rest
                    .split("__")
                    .map(|seg| seg.to_lowercase())
                    .collect::<Vec<_>>()
                    .join(".");
                (key, v)
            })
        })
        .collect()
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_prefix_and_normalises() {
        std::env::set_var("_ROK_TEST_SERVER__PORT", "9000");
        let map = load_env("_ROK_TEST_");
        assert_eq!(map.get("server.port").map(String::as_str), Some("9000"));
        std::env::remove_var("_ROK_TEST_SERVER__PORT");
    }
}
