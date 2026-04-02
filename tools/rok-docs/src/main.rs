//! rok-docs — generate and serve documentation for the rok workspace.
//!
//! # Commands
//!
//! ```text
//! rok-docs generate [--output docs/content]
//! rok-docs serve    [--port 8080] [--dir docs/content]
//! rok-docs build    [--output dist]
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use axum::body::Body;
use axum::response::Response;
use clap::{Parser, Subcommand};
use serde::Deserialize;

// ── CLI ──────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "rok-docs",
    version,
    about = "Generate and serve documentation for the rok workspace"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate markdown documentation from workspace crate metadata
    Generate {
        /// Output directory for generated markdown files
        #[arg(long, short, default_value = "docs/content/crates")]
        output: PathBuf,

        /// Path to workspace Cargo.toml (defaults to current directory)
        #[arg(long, default_value = "Cargo.toml")]
        workspace: PathBuf,
    },

    /// Serve generated docs with a local HTTP server
    Serve {
        /// Directory to serve
        #[arg(long, short, default_value = "docs/content")]
        dir: PathBuf,

        /// Port to listen on
        #[arg(long, short, default_value_t = 8080)]
        port: u16,
    },

    /// Build production-ready static docs (alias for generate)
    Build {
        /// Output directory
        #[arg(long, short, default_value = "dist/docs")]
        output: PathBuf,

        /// Path to workspace Cargo.toml
        #[arg(long, default_value = "Cargo.toml")]
        workspace: PathBuf,
    },
}

// ── Main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { output, workspace } => cmd_generate(&workspace, &output),

        Commands::Serve { dir, port } => cmd_serve(&dir, port).await,

        Commands::Build { output, workspace } => cmd_generate(&workspace, &output),
    }
}

// ── Commands ─────────────────────────────────────────────────────────────────

fn cmd_generate(workspace_toml: &Path, output: &Path) -> Result<()> {
    let workspace = load_workspace(workspace_toml)?;

    std::fs::create_dir_all(output)
        .with_context(|| format!("creating output dir {}", output.display()))?;

    let mut generated = 0usize;

    for member in &workspace.workspace.members {
        // Resolve the member's Cargo.toml relative to the workspace root.
        let workspace_root = workspace_toml
            .parent()
            .unwrap_or_else(|| Path::new("."));

        let member_toml = workspace_root.join(member).join("Cargo.toml");
        if !member_toml.exists() {
            continue;
        }

        let pkg = match load_package(&member_toml) {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Read the module-level doc comment from lib.rs or main.rs.
        let doc = read_crate_doc(workspace_root, member);

        let md = render_crate_doc(&pkg.package, &doc);
        let dest = output.join(format!("{}.md", pkg.package.name));
        std::fs::write(&dest, md)
            .with_context(|| format!("writing {}", dest.display()))?;

        println!("  wrote {}", dest.display());
        generated += 1;
    }

    // Also write an index file listing all crates.
    let index = render_index(&workspace, workspace_toml);
    let index_dest = output.join("index.md");
    std::fs::write(&index_dest, index)
        .with_context(|| format!("writing {}", index_dest.display()))?;
    println!("  wrote {}", index_dest.display());

    println!("\nGenerated docs for {generated} crate(s) into {}", output.display());
    Ok(())
}

async fn cmd_serve(dir: &Path, port: u16) -> Result<()> {
    use axum::Router;

    let dir = dir.to_path_buf();

    if !dir.exists() {
        anyhow::bail!(
            "Directory '{}' does not exist. Run `rok-docs generate` first.",
            dir.display()
        );
    }

    let handler = {
        let dir = dir.clone();
        move |req: axum::http::Request<Body>| {
            let dir = dir.clone();
            async move { serve_file(&dir, req.uri().path()).await }
        }
    };

    let app = Router::new().fallback(handler);

    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;
    println!("Serving '{}' at http://localhost:{port}", dir.display());
    println!("Press Ctrl-C to stop.");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn serve_file(dir: &Path, uri_path: &str) -> Response<Body> {

    // Normalise path — strip leading slash and append index.md for root.
    let rel = uri_path.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.md" } else { rel };

    // Security: reject path traversal.
    if rel.contains("..") {
        return Response::builder()
            .status(403)
            .body(Body::from("Forbidden"))
            .unwrap();
    }

    let path = dir.join(rel);

    match std::fs::read_to_string(&path) {
        Ok(content) => Response::builder()
            .status(200)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Body::from(content))
            .unwrap(),
        Err(_) => Response::builder()
            .status(404)
            .body(Body::from(format!("Not found: {rel}")))
            .unwrap(),
    }
}

// ── Workspace / package loading ───────────────────────────────────────────────

#[derive(Deserialize)]
struct WorkspaceToml {
    workspace: WorkspaceSection,
}

#[derive(Deserialize)]
struct WorkspaceSection {
    members: Vec<String>,
    #[serde(default)]
    package: HashMap<String, toml::Value>,
}

#[derive(Deserialize)]
struct PackageToml {
    package: PackageMeta,
}

#[derive(Deserialize)]
struct PackageMeta {
    name: String,
    // `version.workspace = true` serialises as a table, not a plain string.
    // Accept either with `Option<toml::Value>`.
    version: Option<toml::Value>,
    description: Option<toml::Value>,
    #[serde(default)]
    keywords: Vec<String>,
}

impl PackageMeta {
    fn version_str(&self) -> &str {
        self.version.as_ref().and_then(|v| v.as_str()).unwrap_or("workspace")
    }

    fn description_str(&self) -> &str {
        self.description.as_ref().and_then(|v| v.as_str()).unwrap_or("")
    }
}

fn load_workspace(path: &Path) -> Result<WorkspaceToml> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

fn load_package(path: &Path) -> Result<PackageToml> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

// ── Doc extraction ────────────────────────────────────────────────────────────

/// Read the `//!` module-level doc comment from a crate's `src/lib.rs` or `src/main.rs`.
fn read_crate_doc(workspace_root: &Path, member: &str) -> String {
    for candidate in &["src/lib.rs", "src/main.rs"] {
        let path = workspace_root.join(member).join(candidate);
        if let Ok(content) = std::fs::read_to_string(&path) {
            let doc: String = content
                .lines()
                .take_while(|l| l.starts_with("//!"))
                .map(|l| l.trim_start_matches("//!").trim_start_matches(' '))
                .collect::<Vec<_>>()
                .join("\n");
            if !doc.is_empty() {
                return doc;
            }
        }
    }
    String::new()
}

// ── Markdown rendering ────────────────────────────────────────────────────────

fn render_crate_doc(pkg: &PackageMeta, doc: &str) -> String {
    let mut out = format!("# {}\n\n", pkg.name);

    let desc = pkg.description_str();
    if !desc.is_empty() {
        out.push_str(&format!("> {desc}\n\n"));
    }

    out.push_str(&format!("**Version**: `{}`\n\n", pkg.version_str()));

    if !pkg.keywords.is_empty() {
        let kw = pkg.keywords.iter().map(|k| format!("`{k}`")).collect::<Vec<_>>().join(", ");
        out.push_str(&format!("**Keywords**: {kw}\n\n"));
    }

    out.push_str("## Installation\n\n");
    out.push_str("```bash\ncargo add ");
    out.push_str(&pkg.name);
    out.push_str("\n```\n\n");

    if !doc.is_empty() {
        out.push_str("## Overview\n\n");
        out.push_str(doc);
        out.push('\n');
    }

    out
}

fn render_index(workspace: &WorkspaceToml, workspace_toml: &Path) -> String {
    let workspace_root = workspace_toml.parent().unwrap_or_else(|| Path::new("."));
    let mut out = "# rok Ecosystem — Crate Index\n\n".to_string();
    out.push_str("| Crate | Description | Version |\n");
    out.push_str("|-------|-------------|--------|\n");

    for member in &workspace.workspace.members {
        let member_toml = workspace_root.join(member).join("Cargo.toml");
        if let Ok(pkg) = load_package(&member_toml) {
            let link = format!("[`{}`]({}.md)", pkg.package.name, pkg.package.name);
            let version_str = workspace
                .workspace
                .package
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| pkg.package.version_str());
            let desc = pkg.package.description_str();
            out.push_str(&format!("| {link} | {desc} | `{version_str}` |\n"));
        }
    }

    out
}
