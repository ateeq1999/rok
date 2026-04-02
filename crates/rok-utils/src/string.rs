//! String case-conversion helpers.
//!
//! All functions are pure and allocation-free when the input already matches
//! the target convention.

/// Convert a string to `camelCase`.
///
/// Words are split on `_`, `-`, spaces, and existing case boundaries.
pub fn to_camel_case(s: &str) -> String {
    let words = split_words(s);
    words
        .iter()
        .enumerate()
        .map(|(i, w)| {
            if i == 0 {
                w.to_lowercase()
            } else {
                capitalize(w)
            }
        })
        .collect()
}

/// Convert a string to `PascalCase`.
pub fn to_pascal_case(s: &str) -> String {
    split_words(s).iter().map(|w| capitalize(w)).collect()
}

/// Convert a string to `snake_case`.
pub fn to_snake_case(s: &str) -> String {
    split_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

/// Convert a string to `kebab-case`.
pub fn to_kebab_case(s: &str) -> String {
    split_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

/// Convert a string to `SCREAMING_SNAKE_CASE`.
pub fn to_screaming_snake(s: &str) -> String {
    split_words(s)
        .iter()
        .map(|w| w.to_uppercase())
        .collect::<Vec<_>>()
        .join("_")
}

// ── internal helpers ─────────────────────────────────────────────────────────

/// Split a string into lowercase words by:
///   - `_` / `-` / space separators
///   - camelCase / PascalCase boundaries
fn split_words(s: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();

    let chars: Vec<char> = s.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == '-' || c == ' ' {
            if !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
        } else if c.is_uppercase() {
            // Start a new word on uppercase unless it follows another uppercase
            // (acronym) or is the first character.
            let prev_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_lower = chars
                .get(i + 1)
                .map(|ch| ch.is_lowercase())
                .unwrap_or(false);
            let acronym_end = i > 0 && chars[i - 1].is_uppercase() && next_lower;
            if !current.is_empty() && (prev_lower || acronym_end) {
                words.push(current.clone());
                current.clear();
            }
            current.push(c);
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_to_camel() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("foo_bar_baz"), "fooBarBaz");
    }

    #[test]
    fn snake_to_pascal() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    }

    #[test]
    fn camel_to_snake() {
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("FooBarBaz"), "foo_bar_baz");
    }

    #[test]
    fn camel_to_kebab() {
        assert_eq!(to_kebab_case("helloWorld"), "hello-world");
    }

    #[test]
    fn screaming() {
        assert_eq!(to_screaming_snake("hello_world"), "HELLO_WORLD");
    }

    #[test]
    fn passthrough_unchanged() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
        assert_eq!(to_pascal_case("AlreadyPascal"), "AlreadyPascal");
    }
}
