use rok_lint::{diagnostic::Severity, rules, Linter};

fn linter() -> Linter {
    let mut l = Linter::new();
    l.add_rule(rules::TodoCommentRule::default());
    l.add_rule(rules::LongLineRule::new(80));
    l.add_rule(rules::TrailingWhitespaceRule::default());
    l
}

#[test]
fn detects_todo_comment() {
    let l = linter();
    let diags = l.check_str("f.rs", "fn foo() {} // TODO: fix me\n");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].rule, "todo-comment");
    assert_eq!(diags[0].line, 1);
}

#[test]
fn detects_fixme() {
    let l = linter();
    let diags = l.check_str("f.rs", "// FIXME: this is broken\n");
    assert!(diags.iter().any(|d| d.rule == "todo-comment"));
}

#[test]
fn detects_long_line() {
    let l = linter();
    let long = "x".repeat(81);
    let diags = l.check_str("f.rs", &long);
    assert!(diags.iter().any(|d| d.rule == "long-line"));
}

#[test]
fn no_diagnostic_for_clean_code() {
    let l = linter();
    let diags = l.check_str("f.rs", "fn main() {}\n");
    assert!(diags.is_empty());
}

#[test]
fn detects_trailing_whitespace() {
    let l = linter();
    let diags = l.check_str("f.rs", "let x = 1;   \n");
    assert!(diags.iter().any(|d| d.rule == "trailing-whitespace"));
}

#[test]
fn severity_ordering() {
    assert!(Severity::Error > Severity::Warning);
    assert!(Severity::Warning > Severity::Hint);
}

#[test]
fn check_file_on_disk() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.rs");
    std::fs::write(&path, "// TODO: remove\nfn clean() {}\n").unwrap();
    let l = linter();
    let diags = l.check_file(&path).unwrap();
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].rule, "todo-comment");
}
