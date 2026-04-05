use crate::cli::GenerateCommands;
use rok_generate::Generator;
use std::collections::HashMap;

pub fn run(command: &GenerateCommands) -> i32 {
    match command {
        GenerateCommands::Model {
            name,
            output,
            force,
            fields,
        } => generate_model(name, output, *force, fields.as_deref()),

        GenerateCommands::Api {
            name,
            output,
            force,
            crud,
        } => generate_api(name, output, *force, *crud),

        GenerateCommands::Migration {
            name,
            output,
            fields,
        } => generate_migration(name, output, fields.as_deref()),

        GenerateCommands::Repository {
            name,
            output,
            force,
        } => generate_repository(name, output, *force),
    }
}

fn generate_model(
    name: &str,
    output: &std::path::Path,
    force: bool,
    fields: Option<&str>,
) -> i32 {
    const TYPED_MODEL: &str = r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ name }} {
{% for field in fields %}    pub {{ field.name }}: {{ field.type }},
{% endfor %}}
"#;

    let field_objects: Vec<serde_json::Value> = fields
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|pair| {
            let mut parts = pair.splitn(2, ':');
            let fname = parts.next().unwrap_or("field").trim();
            let ftype = parts.next().unwrap_or("String").trim();
            serde_json::json!({ "name": fname, "type": ftype })
        })
        .collect();

    std::fs::create_dir_all(output).unwrap_or_default();
    let dest = output.join(format!("{}.rs", rok_utils::string::to_snake_case(name)));

    if dest.exists() && !force {
        println!("skip  {} (already exists; use --force to overwrite)", dest.display());
        return 0;
    }

    let mut gen = Generator::new();
    gen.add_template("model", TYPED_MODEL).expect("registering model template");

    let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
    vars.insert("name".into(), serde_json::json!(name));
    vars.insert("fields".into(), serde_json::Value::Array(field_objects));

    gen.render_to_file("model", &vars, &dest).expect("rendering model");
    println!("wrote {}", dest.display());
    0
}

fn generate_api(name: &str, output: &std::path::Path, force: bool, crud: bool) -> i32 {
    let (tpl_key, template) = if crud {
        ("crud_handler", rok_generate::templates::CRUD_HANDLER)
    } else {
        ("handler", rok_generate::templates::HANDLER)
    };

    std::fs::create_dir_all(output).unwrap_or_default();
    let dest = output.join(format!("{}.rs", rok_utils::string::to_snake_case(name)));

    if dest.exists() && !force {
        println!("skip  {} (already exists; use --force to overwrite)", dest.display());
        return 0;
    }

    let mut gen = Generator::new();
    gen.add_template(tpl_key, template).expect("registering handler template");

    let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
    vars.insert("name".into(), serde_json::json!(name));

    gen.render_to_file(tpl_key, &vars, &dest).expect("rendering handler");
    println!("wrote {}", dest.display());
    0
}

fn generate_migration(
    name: &str,
    output: &std::path::Path,
    fields: Option<&str>,
) -> i32 {
    let snake_name = rok_utils::string::to_snake_case(name);
    let next_version = rok_migrate::loader::load_from_dir(output)
        .unwrap_or_default()
        .iter()
        .map(|m| m.version)
        .max()
        .unwrap_or(0)
        + 1;

    std::fs::create_dir_all(output).unwrap_or_default();
    let dest = output.join(format!("{:04}_{}.sql", next_version, snake_name));

    let field_list: Vec<&str> = fields
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let mut gen = Generator::new();
    gen.add_template("migration", rok_generate::templates::MIGRATION)
        .expect("registering migration template");

    let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
    vars.insert("name".into(), serde_json::json!(name));
    vars.insert(
        "fields".into(),
        serde_json::Value::Array(field_list.iter().map(|f| serde_json::json!(f)).collect()),
    );
    vars.insert(
        "created_at".into(),
        serde_json::json!(chrono::Local::now().format("%Y-%m-%d").to_string()),
    );

    gen.render_to_file("migration", &vars, &dest).expect("rendering migration");
    println!("wrote {}", dest.display());
    0
}

fn generate_repository(name: &str, output: &std::path::Path, force: bool) -> i32 {
    std::fs::create_dir_all(output).unwrap_or_default();
    let dest = output.join(format!(
        "{}_repository.rs",
        rok_utils::string::to_snake_case(name)
    ));

    if dest.exists() && !force {
        println!("skip  {} (already exists; use --force to overwrite)", dest.display());
        return 0;
    }

    let mut gen = Generator::new();
    gen.add_template("repository", rok_generate::templates::REPOSITORY)
        .expect("registering repository template");

    let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
    vars.insert("name".into(), serde_json::json!(name));

    gen.render_to_file("repository", &vars, &dest).expect("rendering repository");
    println!("wrote {}", dest.display());
    0
}
