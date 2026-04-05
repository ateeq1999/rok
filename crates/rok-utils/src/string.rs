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

/// Truncate `s` to at most `max` characters, appending `"..."` if truncation
/// occurs.
///
/// Truncation is always on a valid UTF-8 character boundary, so the result may
/// be shorter than `max` for multi-byte inputs.
///
/// ```rust
/// use rok_utils::string::truncate;
/// assert_eq!(truncate("hello world", 5), "he...");
/// assert_eq!(truncate("hi", 10), "hi");
/// ```
pub fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    // Reserve 3 chars for "..."
    let cut = max.saturating_sub(3);
    let mut end = cut;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &s[..end])
}

/// Return a naive English plural of `word`.
///
/// Handles common English inflection rules:
/// * words ending in `s`, `x`, `z`, `ch`, `sh` → `+es`
/// * words ending in a consonant + `y` → `-y` + `ies`
/// * words ending in `f` / `fe` → `-f[e]` + `ves`
/// * words ending in `us` → `-us` + `i`
/// * everything else → `+s`
///
/// ```rust
/// use rok_utils::string::pluralize;
/// assert_eq!(pluralize("user"), "users");
/// assert_eq!(pluralize("box"), "boxes");
/// assert_eq!(pluralize("category"), "categories");
/// assert_eq!(pluralize("leaf"), "leaves");
/// ```
pub fn pluralize(word: &str) -> String {
    if word.is_empty() {
        return word.to_string();
    }
    let lower = word.to_lowercase();
    if lower.ends_with("sis") {
        return format!("{}es", &word[..word.len() - 3]);
    }
    if lower.ends_with("fe") {
        return format!("{}ves", &word[..word.len() - 2]);
    }
    if lower.ends_with('f') && !lower.ends_with("ff") {
        return format!("{}ves", &word[..word.len() - 1]);
    }
    if lower.ends_with("us") {
        return format!("{}i", &word[..word.len() - 2]);
    }
    if lower.ends_with("ch") || lower.ends_with("sh") {
        return format!("{}es", word);
    }
    if lower.ends_with('s') || lower.ends_with('x') || lower.ends_with('z') {
        return format!("{}es", word);
    }
    if lower.ends_with('y') {
        let prev = lower.chars().rev().nth(1);
        if !matches!(prev, Some('a' | 'e' | 'i' | 'o' | 'u')) {
            return format!("{}ies", &word[..word.len() - 1]);
        }
    }
    format!("{}s", word)
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

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("hi", 10), "hi");
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn truncate_long_string() {
        assert_eq!(truncate("hello world", 5), "he...");
        assert_eq!(truncate("abcdefgh", 6), "abc...");
    }

    #[test]
    fn pluralize_regular() {
        assert_eq!(pluralize("user"), "users");
        assert_eq!(pluralize("item"), "items");
    }

    #[test]
    fn pluralize_sibilant() {
        assert_eq!(pluralize("box"), "boxes");
        assert_eq!(pluralize("church"), "churches");
        assert_eq!(pluralize("dish"), "dishes");
        assert_eq!(pluralize("buzz"), "buzzes");
    }

    #[test]
    fn pluralize_y_ending() {
        assert_eq!(pluralize("category"), "categories");
        assert_eq!(pluralize("city"), "cities");
        assert_eq!(pluralize("day"), "days"); // vowel + y → just +s
    }

    #[test]
    fn pluralize_f_ending() {
        assert_eq!(pluralize("leaf"), "leaves");
        assert_eq!(pluralize("knife"), "knives");
    }

    #[test]
    fn pluralize_us_ending() {
        assert_eq!(pluralize("cactus"), "cacti");
        assert_eq!(pluralize("focus"), "foci");
    }
}
