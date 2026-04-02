//! rok-gen-model — generate Rust model structs from a JSON schema.
//!
//! # Schema Format
//!
//! ```json
//! {
//!   "models": [
//!     {
//!       "name": "User",
//!       "fields": [
//!         { "name": "id",    "type": "i64" },
//!         { "name": "name",  "type": "String" },
//!         { "name": "email", "type": "String" }
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! Each model produces a `<snake_name>.rs` file in the output directory.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use heck::ToSnakeCase;
use rok_generate::Generator;
use serde::Deserialize;

// ── CLI ──────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "rok-gen-model",
    version,
    about = "Generate Rust model structs for the rok ecosystem"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate models from a JSON schema file
    FromJson {
        /// Path to the JSON schema file
        #[arg(long, short)]
        file: PathBuf,

        /// Output directory for generated `.rs` files
        #[arg(long, short, default_value = "src/models")]
        output: PathBuf,

        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },

    /// Generate models from a live database (requires DATABASE_URL)
    FromDb {
        /// PostgreSQL connection URL
        #[arg(long)]
        url: String,

        /// Output directory for generated `.rs` files
        #[arg(long, short, default_value = "src/models")]
        output: PathBuf,
    },

    /// Print the SQL for creating or dropping the rok migration history table
    Migrate {
        /// Apply pending migrations (print history-table DDL)
        #[arg(long)]
        up: bool,
    },
}

// ── Schema types ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct Schema {
    models: Vec<ModelDef>,
}

#[derive(Deserialize)]
struct ModelDef {
    name: String,
    #[serde(default)]
    fields: Vec<FieldDef>,
}

#[derive(Deserialize)]
struct FieldDef {
    name: String,
    #[serde(rename = "type", default = "default_type")]
    ty: String,
}

fn default_type() -> String {
    "String".to_string()
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::FromJson {
            file,
            output,
            force,
        } => cmd_from_json(&file, &output, force),

        Commands::FromDb { url, output } => cmd_from_db(&url, &output),

        Commands::Migrate { up } => cmd_migrate(up),
    }
}

// ── Commands ─────────────────────────────────────────────────────────────────

/// Typed model template — uses `field.name` and `field.type` from context.
const TYPED_MODEL: &str = r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ name }} {
{% for field in fields %}    pub {{ field.name }}: {{ field.type }},
{% endfor %}}
"#;

fn cmd_from_json(file: &Path, output: &Path, force: bool) -> Result<()> {
    let content =
        std::fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;

    let schema: Schema =
        serde_json::from_str(&content).with_context(|| format!("parsing {}", file.display()))?;

    if schema.models.is_empty() {
        println!("No models found in schema — nothing to generate.");
        return Ok(());
    }

    std::fs::create_dir_all(output)
        .with_context(|| format!("creating output dir {}", output.display()))?;

    let mut gen = Generator::new();
    gen.add_template("typed_model", TYPED_MODEL)
        .context("registering typed_model template")?;

    let mut generated = 0usize;

    for model in &schema.models {
        let dest = output.join(format!("{}.rs", model.name.to_snake_case()));

        if dest.exists() && !force {
            println!(
                "  skip  {} (already exists; use --force to overwrite)",
                dest.display()
            );
            continue;
        }

        // Build typed field objects for the template.
        let field_objects: Vec<serde_json::Value> = model
            .fields
            .iter()
            .map(|f| serde_json::json!({ "name": f.name, "type": f.ty }))
            .collect();

        let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
        vars.insert("name".into(), serde_json::json!(model.name));
        vars.insert("fields".into(), serde_json::Value::Array(field_objects));

        gen.render_to_file("typed_model", &vars, &dest)
            .with_context(|| format!("rendering model {}", model.name))?;

        println!("  wrote {}", dest.display());
        generated += 1;
    }

    println!("\nGenerated {generated} model(s) into {}", output.display());
    Ok(())
}

fn cmd_from_db(url: &str, output: &Path) -> Result<()> {
    // Introspection requires a live sqlx connection — that would pull in the
    // sqlx runtime.  Print a clear message rather than silently doing nothing.
    println!("DATABASE_URL: {url}");
    println!("output dir  : {}", output.display());
    println!();
    println!("Live database introspection is not yet implemented.");
    println!("To generate models today, export your schema as JSON and use:");
    println!("  rok-gen-model from-json --file schema.json");
    Ok(())
}

fn cmd_migrate(up: bool) -> Result<()> {
    if !up {
        bail!("Pass --up to print the migration history-table DDL.");
    }
    println!("{}", rok_migrate_ddl());
    Ok(())
}

fn rok_migrate_ddl() -> &'static str {
    r#"CREATE TABLE IF NOT EXISTS _rok_migrations (
    version     BIGINT      PRIMARY KEY,
    name        TEXT        NOT NULL,
    applied_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);"#
}
