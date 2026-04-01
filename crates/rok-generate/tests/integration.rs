use rok_generate::Generator;
use std::collections::HashMap;

fn vars(pairs: &[(&str, serde_json::Value)]) -> HashMap<String, serde_json::Value> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
}

#[test]
fn model_template_renders_struct() {
    let gen = Generator::new();
    let code = gen
        .render(
            "model",
            &vars(&[
                ("name", serde_json::json!("User")),
                ("fields", serde_json::json!(["id", "email", "name"])),
            ]),
        )
        .unwrap();
    assert!(code.contains("struct User"));
    assert!(code.contains("pub id: String"));
    assert!(code.contains("pub email: String"));
}

#[test]
fn handler_template_renders_functions() {
    let gen = Generator::new();
    let code = gen
        .render("handler", &vars(&[("name", serde_json::json!("Post"))]))
        .unwrap();
    assert!(code.contains("list_post"));
    assert!(code.contains("get_post"));
}

#[test]
fn migration_template_renders_sql() {
    let gen = Generator::new();
    let code = gen
        .render(
            "migration",
            &vars(&[
                ("name", serde_json::json!("Product")),
                ("fields", serde_json::json!(["title", "price"])),
                ("created_at", serde_json::json!("2026-01-01")),
            ]),
        )
        .unwrap();
    assert!(code.contains("CREATE TABLE IF NOT EXISTS products"));
    assert!(code.contains("title TEXT NOT NULL"));
}

#[test]
fn repository_template_renders_struct() {
    let gen = Generator::new();
    let code = gen
        .render("repository", &vars(&[("name", serde_json::json!("Order"))]))
        .unwrap();
    assert!(code.contains("OrderRepository"));
    assert!(code.contains("find_all"));
    assert!(code.contains("find_by_id"));
}

#[test]
fn unknown_template_returns_error() {
    let gen = Generator::new();
    let result = gen.render("no-such-template", &HashMap::new());
    assert!(result.is_err());
}

#[test]
fn render_to_file() {
    let dir = tempfile::tempdir().unwrap();
    let dest = dir.path().join("user.rs");
    let gen = Generator::new();
    gen.render_to_file(
        "model",
        &vars(&[
            ("name", serde_json::json!("Post")),
            ("fields", serde_json::json!(["title", "body"])),
        ]),
        &dest,
    )
    .unwrap();
    let content = std::fs::read_to_string(&dest).unwrap();
    assert!(content.contains("struct Post"));
}

#[test]
fn custom_template() {
    let mut gen = Generator::new();
    gen.add_template("greeting", "Hello, {{ name }}!").unwrap();
    let out = gen
        .render("greeting", &vars(&[("name", serde_json::json!("rok"))]))
        .unwrap();
    assert_eq!(out.trim(), "Hello, rok!");
}
