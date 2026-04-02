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
use heck::{ToPascalCase, ToSnakeCase};
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::FromJson {
            file,
            output,
            force,
        } => cmd_from_json(&file, &output, force),

        Commands::FromDb { url, output } => cmd_from_db(&url, &output).await,

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

async fn cmd_from_db(url: &str, output: &Path) -> Result<()> {
    use sqlx::Row;

    let pool = sqlx::PgPool::connect(url)
        .await
        .with_context(|| format!("connecting to {url}"))?;

    // Query all user-defined tables in the public schema, ordered by
    // table name and column position.
    let rows = sqlx::query(
        "SELECT table_name, column_name, data_type, is_nullable \
         FROM information_schema.columns \
         WHERE table_schema = 'public' \
         ORDER BY table_name, ordinal_position",
    )
    .fetch_all(&pool)
    .await
    .context("querying information_schema.columns")?;

    if rows.is_empty() {
        println!("No tables found in the public schema — nothing to generate.");
        return Ok(());
    }

    // Group columns by table name, preserving insertion order.
    let mut tables: Vec<(String, Vec<(String, String)>)> = Vec::new();
    let mut table_index: HashMap<String, usize> = HashMap::new();

    for row in &rows {
        let table_name: String = row.try_get("table_name")?;
        let column_name: String = row.try_get("column_name")?;
        let data_type: String = row.try_get("data_type")?;
        let is_nullable: String = row.try_get("is_nullable")?;

        // Skip the rok migration history table.
        if table_name.starts_with("_rok_") {
            continue;
        }

        let rust_type = pg_type_to_rust(&data_type, is_nullable == "YES");

        let idx = table_index.entry(table_name.clone()).or_insert_with(|| {
            let idx = tables.len();
            tables.push((table_name.clone(), Vec::new()));
            idx
        });
        tables[*idx].1.push((column_name, rust_type));
    }

    if tables.is_empty() {
        println!("No tables to generate (only _rok_ internal tables found).");
        return Ok(());
    }

    std::fs::create_dir_all(output)
        .with_context(|| format!("creating output dir {}", output.display()))?;

    let mut gen = Generator::new();
    gen.add_template("typed_model", TYPED_MODEL)
        .context("registering typed_model template")?;

    let mut generated = 0usize;

    for (table_name, columns) in &tables {
        let struct_name = table_name.to_pascal_case();
        let dest = output.join(format!("{}.rs", table_name.to_snake_case()));

        let field_objects: Vec<serde_json::Value> = columns
            .iter()
            .map(|(col, ty)| serde_json::json!({ "name": col, "type": ty }))
            .collect();

        let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
        vars.insert("name".into(), serde_json::json!(struct_name));
        vars.insert("fields".into(), serde_json::Value::Array(field_objects));

        gen.render_to_file("typed_model", &vars, &dest)
            .with_context(|| format!("rendering model {struct_name}"))?;

        println!("  wrote {} ({} field(s))", dest.display(), columns.len());
        generated += 1;
    }

    println!("\nGenerated {generated} model(s) into {}", output.display());
    Ok(())
}

/// Map a PostgreSQL `data_type` string to a Rust type name.
///
/// `nullable` wraps the base type in `Option<T>`.
fn pg_type_to_rust(pg_type: &str, nullable: bool) -> String {
    let base = match pg_type {
        "integer" | "int" | "int4" => "i32",
        "bigint" | "int8" => "i64",
        "smallint" | "int2" => "i16",
        "boolean" => "bool",
        "real" | "float4" => "f32",
        "double precision" | "float8" => "f64",
        "numeric" | "decimal" => "f64",
        "text" | "character varying" | "varchar" | "char" | "character" | "name" => "String",
        "bytea" => "Vec<u8>",
        "uuid" => "uuid::Uuid",
        "timestamp with time zone" => "chrono::DateTime<chrono::Utc>",
        "timestamp without time zone" | "timestamp" => "chrono::NaiveDateTime",
        "date" => "chrono::NaiveDate",
        "time without time zone" | "time" => "chrono::NaiveTime",
        "json" | "jsonb" => "serde_json::Value",
        _ => "String",
    };
    if nullable {
        format!("Option<{base}>")
    } else {
        base.to_string()
    }
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
