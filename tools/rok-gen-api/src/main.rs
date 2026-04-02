//! rok-gen-api — scaffold Axum API handlers, DTOs, and full CRUD endpoints.
//!
//! # Commands
//!
//! ```text
//! rok-gen-api scaffold User --crud
//! rok-gen-api handler CreateUser
//! rok-gen-api dto UserRequest
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use heck::{ToSnakeCase, ToUpperCamelCase};
use rok_generate::Generator;

// ── CLI ──────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "rok-gen-api",
    version,
    about = "Scaffold Axum API handlers and DTOs for the rok ecosystem"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a full CRUD scaffold (model + handlers + repository + migration)
    Scaffold {
        /// Model name in PascalCase (e.g. `User`, `BlogPost`)
        name: String,

        /// Include full CRUD handler set
        #[arg(long)]
        crud: bool,

        /// Root source directory
        #[arg(long, default_value = "src")]
        output: PathBuf,

        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },

    /// Generate a single Axum handler file
    Handler {
        /// Handler name in PascalCase (e.g. `CreateUser`)
        name: String,

        /// Output directory
        #[arg(long, default_value = "src/handlers")]
        output: PathBuf,

        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },

    /// Generate a DTO (Data Transfer Object) struct
    Dto {
        /// DTO name in PascalCase (e.g. `UserRequest`)
        name: String,

        /// Output directory
        #[arg(long, default_value = "src/dto")]
        output: PathBuf,

        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scaffold {
            name,
            crud,
            output,
            force,
        } => cmd_scaffold(&name, crud, &output, force),

        Commands::Handler {
            name,
            output,
            force,
        } => cmd_handler(&name, &output, force),

        Commands::Dto {
            name,
            output,
            force,
        } => cmd_dto(&name, &output, force),
    }
}

// ── Commands ─────────────────────────────────────────────────────────────────

fn cmd_scaffold(name: &str, crud: bool, output: &Path, force: bool) -> Result<()> {
    let pascal = name.to_upper_camel_case();
    let snake = pascal.to_snake_case();
    let gen = Generator::new();

    let base_fields = vec![serde_json::json!("name"), serde_json::json!("created_at")];

    // 1. Model
    write_file(
        force,
        &output.join("models").join(format!("{snake}.rs")),
        || {
            let mut vars = HashMap::new();
            vars.insert("name".into(), serde_json::json!(&pascal));
            vars.insert(
                "fields".into(),
                serde_json::Value::Array(base_fields.clone()),
            );
            gen.render("model", &vars)
        },
    )?;

    // 2. Handler
    write_file(
        force,
        &output.join("handlers").join(format!("{snake}.rs")),
        || {
            let mut vars = HashMap::new();
            vars.insert("name".into(), serde_json::json!(&pascal));
            gen.render("handler", &vars)
        },
    )?;

    // 3. Repository
    write_file(
        force,
        &output
            .join("repositories")
            .join(format!("{snake}_repository.rs")),
        || {
            let mut vars = HashMap::new();
            vars.insert("name".into(), serde_json::json!(&pascal));
            gen.render("repository", &vars)
        },
    )?;

    // 4. Migration SQL
    let migration_path = PathBuf::from("migrations").join(format!("0001_create_{snake}s.sql"));
    write_file(force, &migration_path, || {
        let mut vars = HashMap::new();
        vars.insert("name".into(), serde_json::json!(&pascal));
        vars.insert("created_at".into(), serde_json::json!(chrono_now()));
        vars.insert(
            "fields".into(),
            serde_json::Value::Array(base_fields.clone()),
        );
        gen.render("migration", &vars)
    })?;

    if crud {
        println!("\nCRUD scaffold generated for `{pascal}`:");
        println!("  {}/models/{snake}.rs", output.display());
        println!("  {}/handlers/{snake}.rs", output.display());
        println!("  {}/repositories/{snake}_repository.rs", output.display());
        println!("  migrations/0001_create_{snake}s.sql");
        println!();
        println!(
            "Next steps:\n  1. Add the handler routes to your Router\n  2. Run `rok-migrate up`"
        );
    } else {
        println!("Scaffold generated for `{pascal}` (pass --crud for full CRUD handlers).");
    }

    Ok(())
}

fn cmd_handler(name: &str, output: &Path, force: bool) -> Result<()> {
    let pascal = name.to_upper_camel_case();
    let snake = pascal.to_snake_case();
    let gen = Generator::new();

    let dest = output.join(format!("{snake}.rs"));
    write_file(force, &dest, || {
        let mut vars = HashMap::new();
        vars.insert("name".into(), serde_json::json!(&pascal));
        gen.render("handler", &vars)
    })?;

    println!("Handler written to {}", dest.display());
    Ok(())
}

fn cmd_dto(name: &str, output: &Path, force: bool) -> Result<()> {
    let pascal = name.to_upper_camel_case();
    let snake = pascal.to_snake_case();

    let dest = output.join(format!("{snake}.rs"));

    write_file(force, &dest, || Ok(dto_template(&pascal)))?;
    println!("DTO written to {}", dest.display());
    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn write_file<F>(force: bool, dest: &Path, render: F) -> Result<()>
where
    F: FnOnce() -> Result<String, rok_generate::GenerateError>,
{
    if dest.exists() && !force {
        println!("  skip  {} (use --force to overwrite)", dest.display());
        return Ok(());
    }

    let content = render().with_context(|| format!("rendering {}", dest.display()))?;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating dir {}", parent.display()))?;
    }

    std::fs::write(dest, content).with_context(|| format!("writing {}", dest.display()))?;

    println!("  wrote {}", dest.display());
    Ok(())
}

fn dto_template(name: &str) -> String {
    format!(
        r#"use serde::{{Deserialize, Serialize}};

/// Data Transfer Object for `{name}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {name} {{
    pub name: String,
}}
"#
    )
}

fn chrono_now() -> String {
    // Use a fixed-format timestamp without pulling in chrono directly.
    // The precise value doesn't matter — it's a comment in the migration file.
    "{{now}}".to_string()
}
