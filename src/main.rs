mod cli;
mod config;
mod error;
mod output;
mod refs;
mod runner;
mod schema;
mod steps;

use clap::Parser;
use cli::Cli;
use config::Config;
use error::ExitCode;
use output::format_output;
use runner::Runner;

fn main() {
    let cli = Cli::parse();

    if let Some(cli::Commands::Templates) = cli.command {
        let templates = steps::template_discovery::list_templates(std::path::Path::new("."));
        let json = serde_json::to_string_pretty(&templates).unwrap_or_default();
        println!("{}", json);
        std::process::exit(0);
    }

    if let Some(cli::Commands::InitTemplate { name }) = &cli.command {
        let template_name = name.clone().unwrap_or_else(|| {
            println!("Enter template name: ");
            read_input()
        });

        println!("Creating template: {}", template_name);

        let template_dir = std::path::Path::new(".rok/templates").join(&template_name);
        std::fs::create_dir_all(&template_dir).expect("Failed to create template directory");

        let schema = steps::template_discovery::TemplateSchema {
            name: template_name.clone(),
            description: String::new(),
            version: "1.0.0".to_string(),
            author: String::new(),
            tags: vec![],
            extends: None,
            output: vec![steps::template_discovery::TemplateOutput {
                from: "template.txt".to_string(),
                to: "{{name}}.txt".to_string(),
                condition: None,
            }],
            props: std::collections::HashMap::new(),
            hooks: None,
            post_generate: vec![],
        };

        let schema_path = template_dir.join(".rok-template.json");
        let schema_json =
            serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");
        std::fs::write(&schema_path, &schema_json).expect("Failed to write schema");

        std::fs::write(template_dir.join("template.txt"), "{{ name }}\n")
            .expect("Failed to write template");

        println!("✅ Template created at: {}", template_dir.display());
        println!("   Edit .rok-template.json to add props");
        std::process::exit(0);
    }

    if let Some(cli::Commands::ValidateTemplate { path }) = &cli.command {
        let template_path = path.clone().unwrap_or_else(|| ".".to_string());
        let cwd = std::path::Path::new(&template_path);

        let template_file = if cwd.is_file() {
            cwd.to_path_buf()
        } else {
            cwd.join(".rok-template.json")
        };

        if !template_file.exists() {
            eprintln!("❌ Template schema not found: {}", template_file.display());
            std::process::exit(1);
        }

        match steps::template_discovery::validate_template(&template_file) {
            Ok(_) => {
                println!("✅ Template is valid!");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("❌ Template validation failed:");
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }

    fn read_input() -> String {
        use std::io::Write;
        let mut input = String::new();
        std::io::stdout().flush().ok();
        std::io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    }

    let payload = match cli.parse_payload() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e.message);
            std::process::exit(i32::from(e.code));
        }
    };

    let config = match Config::from_options(payload.options.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e.message);
            std::process::exit(i32::from(e.code));
        }
    };

    if cli.dry_run {
        println!("Dry run - would execute {} steps:", payload.steps.len());
        for (i, step) in payload.steps.iter().enumerate() {
            let step_str = match step {
                schema::Step::Bash { cmd } => format!("  {}: bash {}", i, cmd),
                schema::Step::Read { path } => format!("  {}: read {}", i, path),
                schema::Step::Write { path, .. } => format!("  {}: write {}", i, path),
                schema::Step::Patch { path, .. } => format!("  {}: patch {}", i, path),
                schema::Step::Mv { from, to } => format!("  {}: mv {} → {}", i, from, to),
                schema::Step::Cp { from, to, .. } => format!("  {}: cp {} → {}", i, from, to),
                schema::Step::Rm { path, .. } => format!("  {}: rm {}", i, path),
                schema::Step::Mkdir { path } => format!("  {}: mkdir {}", i, path),
                schema::Step::Grep { pattern, path, .. } => {
                    format!("  {}: grep {} in {}", i, pattern, path)
                }
                schema::Step::Replace { pattern, path, .. } => {
                    format!("  {}: replace {} in {}", i, pattern, path)
                }
                schema::Step::Scan { path, .. } => format!("  {}: scan {}", i, path),
                schema::Step::Summarize { path, .. } => format!("  {}: summarize {}", i, path),
                schema::Step::Extract { path, .. } => format!("  {}: extract {}", i, path),
                schema::Step::Diff { a, b, .. } => format!("  {}: diff {} vs {}", i, a, b),
                schema::Step::Lint { path, .. } => format!("  {}: lint {}", i, path),
                schema::Step::Template {
                    builtin,
                    source,
                    output,
                    ..
                } => {
                    format!(
                        "  {}: template {} -> {}",
                        i,
                        if builtin.is_empty() { source } else { builtin },
                        output
                    )
                }
                schema::Step::Snapshot { path, id } => {
                    format!("  {}: snapshot {} @ {}", i, path, id)
                }
                schema::Step::Restore { id } => format!("  {}: restore {}", i, id),
                schema::Step::Git { op, .. } => format!("  {}: git {:?}", i, op),
                schema::Step::Http { method, url, .. } => {
                    format!("  {}: http {} {}", i, method, url)
                }
                schema::Step::If { condition, .. } => format!("  {}: if {:?}", i, condition),
                schema::Step::Each { over, .. } => format!("  {}: each over {:?}", i, over),
                schema::Step::Parallel { steps } => {
                    format!("  {}: parallel ({} steps)", i, steps.len())
                }
            };
            println!("{}", step_str);
        }
        std::process::exit(0);
    }

    let runner = Runner::new(config, payload);
    let output = runner.run();

    let formatted = format_output(&output, &cli.output);

    if !formatted.is_empty() {
        println!("{}", formatted);
    }

    let exit_code = match output.status.as_str() {
        "ok" => ExitCode::Ok,
        _ => ExitCode::Partial,
    };

    std::process::exit(i32::from(exit_code));
}
