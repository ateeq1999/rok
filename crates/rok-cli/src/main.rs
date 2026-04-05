mod cache;
mod cli;
mod config;
mod config_file;
mod error;
mod output;
mod progress;
mod refs;
mod runner;
mod schema;
mod steps;

use clap::Parser;
use cli::Cli;
use config::Config;
use config_file::apply_config;
use error::ExitCode;
use output::format_output;
use progress::ProgressReporter;
use runner::Runner;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;

fn serve_docs(port: &str) {
    let addr = format!("0.0.0.0:{}", port);
    let doc_dir = Path::new("docs");

    println!("📖 Serving rok documentation at http://localhost:{}", port);
    println!("Press Ctrl+C to stop\n");

    let listener = TcpListener::bind(&addr).expect("Failed to bind to port");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 2048];
                if let Ok(bytes_read) = stream.read(&mut buffer) {
                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                    let response = if request.starts_with("GET / ")
                        || request.starts_with("GET /index.html")
                    {
                        serve_file(&doc_dir.join("index.html"))
                    } else if request.contains("GET /api") {
                        serve_file(&doc_dir.join("api.html"))
                    } else if request.starts_with("GET /") {
                        let path = request
                            .split_whitespace()
                            .nth(1)
                            .unwrap_or("/")
                            .trim_start_matches('/');

                        let file_path = doc_dir.join(path);
                        if file_path.exists() && file_path.is_file() {
                            serve_file(&file_path)
                        } else {
                            not_found()
                        }
                    } else {
                        not_found()
                    };

                    let _ = stream.write_all(response.as_bytes());
                }
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}

fn serve_file(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let content_type = match ext {
                "css" => "text/css",
                "js" => "application/javascript",
                "json" => "application/json",
                "html" | "md" => "text/html",
                _ => "text/plain",
            };

            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                content_type,
                content.len(),
                content
            )
        }
        Err(_) => not_found(),
    }
}

fn not_found() -> String {
    "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 171\r\n\r\n<!DOCTYPE html><html><head><title>404</title></head><body><h1>404 Not Found</h1><p>The requested file was not found.</p></body></html>".to_string()
}

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

    if let Some(cli::Commands::Run { name }) = &cli.command {
        let task_path = std::path::Path::new(".rok/tasks").join(format!("{}.json", name));
        if !task_path.exists() {
            eprintln!("❌ Task not found: {}", name);
            std::process::exit(1);
        }
        let task_json = std::fs::read_to_string(&task_path).expect("Failed to read task file");
        let task_payload: schema::Payload =
            serde_json::from_str(&task_json).expect("Failed to parse task file");

        let options = apply_config(task_payload.options.clone());
        let config = match Config::from_options(options) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {}", e.message);
                std::process::exit(i32::from(e.code));
            }
        };

        let runner = Runner::new(config, task_payload);
        let result = runner.run();
        let output = format_output(&result, &cli.output);
        println!("{}", output);
        std::process::exit(0);
    }

    if let Some(cli::Commands::Save { name, description }) = &cli.command {
        let payload = match cli.parse_payload() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error: {}", e.message);
                std::process::exit(i32::from(e.code));
            }
        };

        let mut task_payload = payload;
        if task_payload.name.is_none() {
            task_payload.name = Some(name.clone());
        }
        if let Some(desc) = description {
            task_payload.description = Some(desc.clone());
        }

        let tasks_dir = std::path::Path::new(".rok/tasks");
        std::fs::create_dir_all(tasks_dir).expect("Failed to create tasks directory");

        let task_path = tasks_dir.join(format!("{}.json", name));
        let task_json =
            serde_json::to_string_pretty(&task_payload).expect("Failed to serialize task");
        std::fs::write(&task_path, task_json).expect("Failed to write task file");

        println!("✅ Task saved: {}", name);
        std::process::exit(0);
    }

    if let Some(cli::Commands::List) = cli.command {
        let tasks_dir = std::path::Path::new(".rok/tasks");
        if !tasks_dir.exists() {
            println!("No saved tasks found.");
            std::process::exit(0);
        }

        let mut tasks: Vec<serde_json::Value> = Vec::new();
        if let Ok(entries) = std::fs::read_dir(tasks_dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "json" {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            if let Ok(task) = serde_json::from_str::<schema::Payload>(&content) {
                                tasks.push(serde_json::json!({
                                    "name": task.name.unwrap_or_else(|| entry.path().file_stem().unwrap_or_default().to_string_lossy().to_string()),
                                    "description": task.description.unwrap_or_default(),
                                    "version": task.version.unwrap_or_default(),
                                    "author": task.author.unwrap_or_default(),
                                    "steps": task.steps.len()
                                }));
                            }
                        }
                    }
                }
            }
        }

        if tasks.is_empty() {
            println!("No saved tasks found.");
        } else {
            println!("Saved tasks:");
            for task in &tasks {
                println!(
                    "  {} - {} ({} steps)",
                    task["name"],
                    if task["description"].as_str().unwrap_or("").is_empty() {
                        "no description"
                    } else {
                        task["description"].as_str().unwrap()
                    },
                    task["steps"]
                );
            }
        }
        std::process::exit(0);
    }

    #[allow(clippy::zombie_processes)]
    if let Some(cli::Commands::Edit { name }) = &cli.command {
        let task_path = std::path::Path::new(".rok/tasks").join(format!("{}.json", name));
        if !task_path.exists() {
            eprintln!("❌ Task not found: {}", name);
            std::process::exit(1);
        }

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("notepad")
                .arg(&task_path)
                .spawn()
                .expect("Failed to open editor");
        }
        #[cfg(not(target_os = "windows"))]
        {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
            std::process::Command::new(editor)
                .arg(&task_path)
                .spawn()
                .expect("Failed to open editor");
        }

        std::process::exit(0);
    }

    if let Some(cli::Commands::Watch {
        file,
        watch,
        interval,
    }) = &cli.command
    {
        let file_path = file.clone().unwrap_or_else(|| {
            eprintln!("Error: --file required for watch mode");
            std::process::exit(1);
        });

        let watch_paths = watch.clone().unwrap_or_else(|| vec![".".to_string()]);

        println!("Watching {:?} for changes (Ctrl+C to stop)...", watch_paths);

        loop {
            let payload: schema::Payload =
                serde_json::from_str(&std::fs::read_to_string(&file_path).unwrap_or_else(|e| {
                    eprintln!("Error reading file: {}", e);
                    std::process::exit(1);
                }))
                .unwrap_or_else(|e| {
                    eprintln!("Error parsing JSON: {}", e);
                    std::process::exit(1);
                });

            let options = apply_config(payload.options.clone());
            let config = match Config::from_options(options) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e.message);
                    std::process::exit(i32::from(e.code));
                }
            };

            println!(
                "\n--- Running at {} ---",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );
            let runner = Runner::new(config, payload);
            let output = runner.run();
            let formatted = format_output(&output, &cli.output);
            if !formatted.is_empty() {
                println!("{}", formatted);
            }

            std::thread::sleep(std::time::Duration::from_millis(*interval));
        }
    }

    if let Some(cli::Commands::History { count }) = &cli.command {
        let history_file = std::path::Path::new(".rok/history.json");
        if !history_file.exists() {
            println!("No execution history found.");
            std::process::exit(0);
        }

        let history: Vec<serde_json::Value> =
            serde_json::from_str(&std::fs::read_to_string(history_file).unwrap_or_default())
                .unwrap_or_default();

        if history.is_empty() {
            println!("No execution history found.");
        } else {
            println!("Execution history (last {}):", count);
            for entry in history.iter().take(*count) {
                println!(
                    "  {} - {} - {} steps - {}ms",
                    entry["run_id"].as_str().unwrap_or("?"),
                    entry["status"].as_str().unwrap_or("?"),
                    entry["steps_total"].as_u64().unwrap_or(0),
                    entry["duration_ms"].as_u64().unwrap_or(0)
                );
            }
        }
        std::process::exit(0);
    }

    if let Some(cli::Commands::Replay { run_id }) = &cli.command {
        let history_file = std::path::Path::new(".rok/history.json");
        if !history_file.exists() {
            eprintln!("No execution history found.");
            std::process::exit(1);
        }

        let history: Vec<serde_json::Value> =
            serde_json::from_str(&std::fs::read_to_string(history_file).unwrap_or_default())
                .unwrap_or_default();

        let target_id = run_id.clone().unwrap_or_else(|| {
            history
                .first()
                .map(|e| e["run_id"].as_str().unwrap_or("").to_string())
                .unwrap_or_default()
        });

        let entry = history
            .iter()
            .find(|e| e["run_id"].as_str() == Some(&target_id));

        if let Some(entry) = entry {
            let payload_json = entry["payload"].as_str().unwrap_or("{}");
            let payload: schema::Payload =
                serde_json::from_str(payload_json).expect("Failed to parse payload");

            let options = apply_config(payload.options.clone());
            let config = match Config::from_options(options) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e.message);
                    std::process::exit(i32::from(e.code));
                }
            };

            println!("Replaying run {}...", target_id);
            let runner = Runner::new(config, payload);
            let output = runner.run();
            let formatted = format_output(&output, &cli.output);
            if !formatted.is_empty() {
                println!("{}", formatted);
            }
            std::process::exit(0);
        } else {
            eprintln!("Run {} not found in history.", target_id);
            std::process::exit(1);
        }
    }

    if let Some(cli::Commands::Cache { stats, clear }) = &cli.command {
        let cache_dir = std::path::Path::new(".rok/cache");

        if *clear {
            match cache::clear(cache_dir) {
                Ok(count) => {
                    println!("✓ Cleared {} cache entries", count);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error clearing cache: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // Default: show stats
        if *stats || !*clear {
            let cache_enabled = true; // Cache feature is always available
            let cache_stats = cache::get_stats(cache_dir, cache_enabled);

            println!("Cache Statistics:");
            println!("  Enabled: {}", cache_stats.enabled);
            println!("  Directory: {}", cache_stats.cache_dir);
            println!("  Total Entries: {}", cache_stats.total_entries);
            println!(
                "  Total Size: {} ({})",
                cache_stats.total_size_human, cache_stats.total_size_bytes
            );

            if let Some(oldest) = &cache_stats.oldest_entry {
                println!("  Oldest Entry: {}", oldest);
            }
            if let Some(newest) = &cache_stats.newest_entry {
                println!("  Newest Entry: {}", newest);
            }

            if cache_stats.total_entries == 0 {
                println!("\nCache is empty. Run tasks with --cache to populate.");
            }

            std::process::exit(0);
        }
    }

    if let Some(cli::Commands::Checkpoints { list, delete }) = &cli.command {
        let checkpoint_dir = std::path::Path::new(".rok/checkpoints");

        if let Some(id) = delete {
            let file = checkpoint_dir.join(format!("{}.json", id));
            if file.exists() {
                std::fs::remove_file(&file).expect("Failed to delete checkpoint");
                println!("✓ Deleted checkpoint: {}", id);
            } else {
                eprintln!("Checkpoint not found: {}", id);
                std::process::exit(1);
            }
            std::process::exit(0);
        }

        if *list || !delete.is_some() {
            if !checkpoint_dir.exists() {
                println!("No checkpoints found.");
                std::process::exit(0);
            }

            let mut found = false;
            if let Ok(entries) = std::fs::read_dir(checkpoint_dir) {
                for entry in entries.flatten() {
                    if entry
                        .path()
                        .extension()
                        .map(|e| e == "json")
                        .unwrap_or(false)
                    {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                                if !found {
                                    println!("Checkpoints:");
                                    found = true;
                                }
                                println!(
                                    "  {} - created: {}",
                                    data["checkpoint_id"].as_str().unwrap_or("?"),
                                    data["created_at"].as_str().unwrap_or("?")
                                );
                            }
                        }
                    }
                }
            }

            if !found {
                println!("No checkpoints found.");
            }
        }

        std::process::exit(0);
    }

    if let Some(cli::Commands::Serve { port }) = &cli.command {
        serve_docs(port);
        std::process::exit(0);
    }

    if let Some(cli::Commands::Generate { command }) = &cli.command {
        use cli::GenerateCommands;
        use rok_generate::Generator;
        use std::collections::HashMap;

        match command {
            GenerateCommands::Model {
                name,
                output,
                force,
                fields,
            } => {
                const TYPED_MODEL: &str = r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ name }} {
{% for field in fields %}    pub {{ field.name }}: {{ field.type }},
{% endfor %}}
"#;
                let field_objects: Vec<serde_json::Value> = fields
                    .as_deref()
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
                    println!(
                        "skip  {} (already exists; use --force to overwrite)",
                        dest.display()
                    );
                    std::process::exit(0);
                }

                let mut gen = Generator::new();
                gen.add_template("model", TYPED_MODEL)
                    .expect("registering model template");

                let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
                vars.insert("name".into(), serde_json::json!(name));
                vars.insert("fields".into(), serde_json::Value::Array(field_objects));

                gen.render_to_file("model", &vars, &dest)
                    .expect("rendering model");
                println!("wrote {}", dest.display());
            }

            GenerateCommands::Api {
                name,
                output,
                force,
                crud,
            } => {
                let (tpl_key, template) = if *crud {
                    ("crud_handler", rok_generate::templates::CRUD_HANDLER)
                } else {
                    ("handler", rok_generate::templates::HANDLER)
                };

                std::fs::create_dir_all(output).unwrap_or_default();
                let dest = output.join(format!("{}.rs", rok_utils::string::to_snake_case(name)));

                if dest.exists() && !force {
                    println!(
                        "skip  {} (already exists; use --force to overwrite)",
                        dest.display()
                    );
                    std::process::exit(0);
                }

                let mut gen = Generator::new();
                gen.add_template(tpl_key, template)
                    .expect("registering handler template");

                let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
                vars.insert("name".into(), serde_json::json!(name));

                gen.render_to_file(tpl_key, &vars, &dest)
                    .expect("rendering handler");
                println!("wrote {}", dest.display());
            }

            GenerateCommands::Migration { name, output, fields } => {
                let snake_name = rok_utils::string::to_snake_case(name);

                // Determine next version from existing migrations in output dir.
                let next_version = rok_migrate::loader::load_from_dir(output)
                    .unwrap_or_default()
                    .iter()
                    .map(|m| m.version)
                    .max()
                    .unwrap_or(0)
                    + 1;

                std::fs::create_dir_all(output).unwrap_or_default();
                let filename = format!("{:04}_{}.sql", next_version, snake_name);
                let dest = output.join(&filename);

                let field_list: Vec<&str> = fields
                    .as_deref()
                    .unwrap_or("")
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                let mut gen = Generator::new();
                gen.add_template("migration", rok_generate::templates::MIGRATION)
                    .expect("registering migration template");

                let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
                vars.insert("name".into(), serde_json::json!(name));
                vars.insert(
                    "fields".into(),
                    serde_json::Value::Array(
                        field_list.iter().map(|f| serde_json::json!(f)).collect(),
                    ),
                );
                vars.insert(
                    "created_at".into(),
                    serde_json::json!(chrono::Local::now().format("%Y-%m-%d").to_string()),
                );

                gen.render_to_file("migration", &vars, &dest)
                    .expect("rendering migration");
                println!("wrote {}", dest.display());
            }

            GenerateCommands::Repository { name, output, force } => {
                std::fs::create_dir_all(output).unwrap_or_default();
                let dest =
                    output.join(format!("{}_repository.rs", rok_utils::string::to_snake_case(name)));

                if dest.exists() && !force {
                    println!(
                        "skip  {} (already exists; use --force to overwrite)",
                        dest.display()
                    );
                    std::process::exit(0);
                }

                let mut gen = Generator::new();
                gen.add_template("repository", rok_generate::templates::REPOSITORY)
                    .expect("registering repository template");

                let mut vars: HashMap<String, serde_json::Value> = HashMap::new();
                vars.insert("name".into(), serde_json::json!(name));

                gen.render_to_file("repository", &vars, &dest)
                    .expect("rendering repository");
                println!("wrote {}", dest.display());
            }
        }
        std::process::exit(0);
    }

    if let Some(cli::Commands::Migrate { command }) = &cli.command {
        use cli::MigrateCommands;
        use rok_migrate::{load_from_dir, Migrator};

        match command {
            MigrateCommands::Init => {
                println!("{}", Migrator::history_table_sql());
            }

            MigrateCommands::Status { dir } => {
                let migrations = load_from_dir(dir).unwrap_or_else(|e| {
                    eprintln!("Error loading migrations: {e}");
                    std::process::exit(1);
                });

                if migrations.is_empty() {
                    println!("No migrations found in {}", dir.display());
                } else {
                    println!("Migrations in {}:", dir.display());
                    for m in &migrations {
                        let reversible = if m.down_sql.is_some() {
                            "reversible"
                        } else {
                            "irreversible"
                        };
                        println!("  {:04}  {}  [{}]", m.version, m.name, reversible);
                    }
                }
            }

            MigrateCommands::Plan { dir } => {
                let migrations = load_from_dir(dir).unwrap_or_else(|e| {
                    eprintln!("Error loading migrations: {e}");
                    std::process::exit(1);
                });

                let mut migrator = Migrator::new();
                for m in migrations {
                    migrator.add(m).unwrap_or_else(|e| {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    });
                }

                // Show plan assuming nothing is applied yet
                let plan = migrator.plan_up(&[]);
                if plan.is_empty() {
                    println!("No pending migrations.");
                } else {
                    println!("Execution plan ({} migration(s)):\n", plan.len());
                    for step in &plan {
                        println!("-- {:04}: {}", step.migration.version, step.migration.name);
                        println!("{}\n", step.sql);
                    }
                }
            }

            MigrateCommands::New { name, dir } => {
                // Determine next version number from existing migrations.
                let next_version = load_from_dir(dir)
                    .unwrap_or_default()
                    .iter()
                    .map(|m| m.version)
                    .max()
                    .unwrap_or(0)
                    + 1;

                std::fs::create_dir_all(dir).unwrap_or_else(|e| {
                    eprintln!("Error creating migrations directory: {e}");
                    std::process::exit(1);
                });

                let snake_name = rok_utils::string::to_snake_case(name);
                let filename = format!("{:04}_{}.sql", next_version, snake_name);
                let path = dir.join(&filename);

                let content = "-- UP\n\n\n-- DOWN\n\n";
                std::fs::write(&path, content).unwrap_or_else(|e| {
                    eprintln!("Error writing migration file: {e}");
                    std::process::exit(1);
                });

                println!("created  {}", path.display());
            }
        }
        std::process::exit(0);
    }

    if let Some(cli::Commands::Lint {
        path,
        max_line_length,
    }) = &cli.command
    {
        use rok_lint::{
            rules::{LongLineRule, TodoCommentRule, TrailingWhitespaceRule},
            Linter, Severity,
        };

        let mut linter = Linter::new();
        linter.add_rule(TodoCommentRule::default());
        linter.add_rule(LongLineRule::new(*max_line_length));
        linter.add_rule(TrailingWhitespaceRule);

        let files: Vec<std::path::PathBuf> = if path.is_file() {
            vec![path.clone()]
        } else {
            walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|x| x.to_str())
                        .map(|x| matches!(x, "rs" | "ts" | "js" | "py" | "go"))
                        .unwrap_or(false)
                })
                .filter(|e| {
                    !e.path()
                        .components()
                        .any(|c| c.as_os_str() == "target" || c.as_os_str() == "node_modules")
                })
                .map(|e| e.path().to_path_buf())
                .collect()
        };

        if files.is_empty() {
            println!("No source files found in {}", path.display());
            std::process::exit(0);
        }

        let mut total = 0usize;
        let mut errors = 0usize;
        for file in &files {
            let content = match std::fs::read_to_string(file) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let diags = linter.check_str(file.to_str().unwrap_or("?"), &content);
            for d in &diags {
                if d.severity == Severity::Error {
                    errors += 1;
                }
                println!(
                    "{}:{}  [{}]  {}  {}",
                    d.file,
                    d.line,
                    d.severity,
                    d.rule,
                    d.message
                );
            }
            total += diags.len();
        }

        if total == 0 {
            println!("No issues found in {} file(s).", files.len());
        } else {
            println!("\n{} issue(s) in {} file(s).", total, files.len());
        }
        std::process::exit(if errors > 0 { 1 } else { 0 });
    }

    if let Some(cli::Commands::Deploy { command }) = &cli.command {
        use cli::DeployCommands;
        use rok_deploy::{compose, dockerfile, kubernetes, DeployConfig};

        fn make_config(name: &str, port: u16, replicas: u32) -> DeployConfig {
            DeployConfig {
                name: name.to_string(),
                image: name.to_string(),
                port,
                replicas,
                ..Default::default()
            }
        }

        match command {
            DeployCommands::Dockerfile { name, port } => {
                let cfg = make_config(name, *port, 1);
                println!("{}", dockerfile::generate(&cfg));
            }
            DeployCommands::Compose { name, port } => {
                let cfg = make_config(name, *port, 1);
                println!("{}", compose::generate(&cfg));
            }
            DeployCommands::K8s { name, port, replicas } => {
                let cfg = make_config(name, *port, *replicas);
                println!("{}", kubernetes::manifests(&cfg));
            }
            DeployCommands::All { name, port, replicas } => {
                let cfg = make_config(name, *port, *replicas);
                println!("# ── Dockerfile ──────────────────────────────────────────────────────────────");
                println!("{}", dockerfile::generate(&cfg));
                println!("# ── docker-compose.yml ──────────────────────────────────────────────────────");
                println!("{}", compose::generate(&cfg));
                println!("# ── kubernetes manifests ────────────────────────────────────────────────────");
                println!("{}", kubernetes::manifests(&cfg));
            }
        }
        std::process::exit(0);
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

    // Apply configuration file settings
    let options = apply_config(payload.options.clone());
    let config = match Config::from_options(options) {
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
                schema::Step::Bash { cmd, .. } => format!("  {}: bash {}", i, cmd),
                schema::Step::Read {
                    path,
                    filter_imports,
                    filter_exports,
                    since,
                    ..
                } => {
                    let mut s = format!("  {}: read {}", i, path);
                    if let Some(fi) = filter_imports {
                        s.push_str(&format!(" import:{}", fi));
                    }
                    if let Some(fe) = filter_exports {
                        s.push_str(&format!(" export:{}", fe));
                    }
                    if let Some(since_time) = since {
                        s.push_str(&format!(" since:{}", since_time));
                    }
                    s
                }
                schema::Step::Write { path, .. } => format!("  {}: write {}", i, path),
                schema::Step::Patch { path, .. } => format!("  {}: patch {}", i, path),
                schema::Step::Mv { from, to, .. } => format!("  {}: mv {} → {}", i, from, to),
                schema::Step::Cp { from, to, .. } => format!("  {}: cp {} → {}", i, from, to),
                schema::Step::Rm { path, .. } => format!("  {}: rm {}", i, path),
                schema::Step::Mkdir { path, .. } => format!("  {}: mkdir {}", i, path),
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
                schema::Step::Snapshot {
                    path, snapshot_id, ..
                } => {
                    format!("  {}: snapshot {} @ {}", i, path, snapshot_id)
                }
                schema::Step::Restore { snapshot_id, .. } => {
                    format!("  {}: restore {}", i, snapshot_id)
                }
                schema::Step::Git { op, .. } => format!("  {}: git {:?}", i, op),
                schema::Step::Http { method, url, .. } => {
                    format!("  {}: http {} {}", i, method, url)
                }
                schema::Step::Import {
                    path,
                    add,
                    remove,
                    organize,
                    ..
                } => {
                    format!(
                        "  {}: import {} (add: {}, remove: {}, organize: {})",
                        i,
                        path,
                        add.len(),
                        remove.len(),
                        organize
                    )
                }
                schema::Step::Refactor {
                    symbol,
                    rename_to,
                    path,
                    dry_run,
                    ..
                } => {
                    format!(
                        "  {}: refactor '{}' -> '{}' in {}{}",
                        i,
                        symbol,
                        rename_to,
                        path,
                        if *dry_run { " [dry-run]" } else { "" }
                    )
                }
                schema::Step::Deps { path, depth, .. } => {
                    format!("  {}: deps {} (depth: {})", i, path, depth)
                }
                schema::Step::Checkpoint {
                    checkpoint_id,
                    restore,
                    ..
                } => {
                    format!(
                        "  {}: checkpoint {} ({})",
                        i,
                        checkpoint_id,
                        if *restore { "restore" } else { "save" }
                    )
                }
                schema::Step::Boilerplate {
                    path,
                    add_header,
                    add_license,
                    add_shebang,
                    auto_imports,
                    ..
                } => {
                    let mut s = format!("  {}: boilerplate {}", i, path);
                    if add_header.is_some() {
                        s.push_str(" +header");
                    }
                    if add_license.is_some() {
                        s.push_str(" +license");
                    }
                    if add_shebang.is_some() {
                        s.push_str(" +shebang");
                    }
                    if *auto_imports {
                        s.push_str(" +auto-imports");
                    }
                    s
                }
                schema::Step::DeadCode { path, include, .. } => {
                    format!("  {}: dead_code {} (include: {:?})", i, path, include)
                }
                schema::Step::If { condition, .. } => format!("  {}: if {:?}", i, condition),
                schema::Step::Each { over, .. } => format!("  {}: each over {:?}", i, over),
                schema::Step::Parallel { steps, .. } => {
                    format!("  {}: parallel ({} steps)", i, steps.len())
                }
            };
            println!("{}", step_str);
        }
        std::process::exit(0);
    }

    let steps_count = payload.steps.len();
    let runner = Runner::new(config, payload);

    // Show progress in verbose mode or if explicitly enabled
    let show_progress = cli.verbose && progress::should_show_progress();

    if show_progress {
        let _reporter = ProgressReporter::new(steps_count);
        eprintln!("[progress] Starting execution of {} steps...", steps_count);
    }

    let output = runner.run();

    let formatted = format_output(&output, &cli.output);

    if cli.quiet {
        if output.status != "ok" {
            eprintln!("Error: {} steps failed", output.steps_failed);
        }
    } else if !formatted.is_empty() {
        println!("{}", formatted);
    }

    if cli.verbose {
        eprintln!(
            "[verbose] Execution completed: {} ok, {} failed in {}ms",
            output.steps_ok, output.steps_failed, output.duration_ms
        );
    }

    let exit_code = match output.status.as_str() {
        "ok" => ExitCode::Ok,
        _ => ExitCode::Partial,
    };

    std::process::exit(i32::from(exit_code));
}
