mod cache;
mod cli;
mod cmd;
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
use cli::{Cli, Commands};
use config::Config;
use config_file::apply_config;
use error::ExitCode;
use output::format_output;
use progress::ProgressReporter;
use runner::Runner;

fn main() {
    let cli = Cli::parse();
    std::process::exit(dispatch(&cli));
}

fn dispatch(cli: &Cli) -> i32 {
    match &cli.command {
        // ── template management ───────────────────────────────────────────
        Some(Commands::Templates) => cmd::templates::list(),
        Some(Commands::InitTemplate { name }) => cmd::templates::init(name.clone()),
        Some(Commands::ValidateTemplate { path }) => cmd::templates::validate(path.clone()),

        // ── saved tasks ───────────────────────────────────────────────────
        Some(Commands::Run { name }) => cmd::tasks::run(name, &cli.output),
        Some(Commands::Save { name, description }) => {
            let payload = match cli.parse_payload() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: {}", e.message);
                    return i32::from(e.code);
                }
            };
            cmd::tasks::save(name, description.as_deref(), payload)
        }
        Some(Commands::List) => cmd::tasks::list(),
        Some(Commands::Edit { name }) => cmd::tasks::edit(name),

        // ── watch ─────────────────────────────────────────────────────────
        Some(Commands::Watch { file, watch, interval }) => cmd::watch::watch(
            file.clone(),
            watch.clone().unwrap_or_else(|| vec![".".to_string()]),
            *interval,
            &cli.output,
        ),

        // ── history ───────────────────────────────────────────────────────
        Some(Commands::History { count }) => cmd::history::show(*count),
        Some(Commands::Replay { run_id }) => cmd::history::replay(run_id.clone(), &cli.output),

        // ── cache & checkpoints ───────────────────────────────────────────
        Some(Commands::Cache { stats, clear }) => cmd::cache::manage(*stats, *clear),
        Some(Commands::Checkpoints { list, delete }) => {
            cmd::cache::checkpoints(*list, delete.as_deref())
        }

        // ── serve ─────────────────────────────────────────────────────────
        Some(Commands::Serve { port }) => cmd::serve::serve(port),

        // ── code generation ───────────────────────────────────────────────
        Some(Commands::Generate { command }) => cmd::generate::run(command),

        // ── migrations ────────────────────────────────────────────────────
        Some(Commands::Migrate { command }) => cmd::migrate::run(command),

        // ── lint ──────────────────────────────────────────────────────────
        Some(Commands::Lint { path, max_line_length }) => cmd::lint::run(path, *max_line_length),

        // ── deploy ────────────────────────────────────────────────────────
        Some(Commands::Deploy { command }) => cmd::deploy::run(command),

        // ── default: run a payload from --file / --json / stdin ───────────
        None => run_payload(cli),
    }
}

fn run_payload(cli: &Cli) -> i32 {
    let payload = match cli.parse_payload() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e.message);
            return i32::from(e.code);
        }
    };

    let options = apply_config(payload.options.clone());
    let config = match Config::from_options(options) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e.message);
            return i32::from(e.code);
        }
    };

    if cli.dry_run {
        return dry_run(&payload);
    }

    let steps_count = payload.steps.len();
    let runner = Runner::new(config, payload);

    if cli.verbose && progress::should_show_progress() {
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

    match output.status.as_str() {
        "ok" => i32::from(ExitCode::Ok),
        _ => i32::from(ExitCode::Partial),
    }
}

fn dry_run(payload: &schema::Payload) -> i32 {
    println!("Dry run - would execute {} steps:", payload.steps.len());
    for (i, step) in payload.steps.iter().enumerate() {
        println!("{}", describe_step(i, step));
    }
    0
}

fn describe_step(i: usize, step: &schema::Step) -> String {
    match step {
        schema::Step::Bash { cmd, .. } => format!("  {i}: bash {cmd}"),
        schema::Step::Read { path, filter_imports, filter_exports, since, .. } => {
            let mut s = format!("  {i}: read {path}");
            if let Some(fi) = filter_imports { s.push_str(&format!(" import:{fi}")); }
            if let Some(fe) = filter_exports { s.push_str(&format!(" export:{fe}")); }
            if let Some(t) = since { s.push_str(&format!(" since:{t}")); }
            s
        }
        schema::Step::Write { path, .. } => format!("  {i}: write {path}"),
        schema::Step::Patch { path, .. } => format!("  {i}: patch {path}"),
        schema::Step::Mv { from, to, .. } => format!("  {i}: mv {from} → {to}"),
        schema::Step::Cp { from, to, .. } => format!("  {i}: cp {from} → {to}"),
        schema::Step::Rm { path, .. } => format!("  {i}: rm {path}"),
        schema::Step::Mkdir { path, .. } => format!("  {i}: mkdir {path}"),
        schema::Step::Grep { pattern, path, .. } => format!("  {i}: grep {pattern} in {path}"),
        schema::Step::Replace { pattern, path, .. } => {
            format!("  {i}: replace {pattern} in {path}")
        }
        schema::Step::Scan { path, .. } => format!("  {i}: scan {path}"),
        schema::Step::Summarize { path, .. } => format!("  {i}: summarize {path}"),
        schema::Step::Extract { path, .. } => format!("  {i}: extract {path}"),
        schema::Step::Diff { a, b, .. } => format!("  {i}: diff {a} vs {b}"),
        schema::Step::Lint { path, .. } => format!("  {i}: lint {path}"),
        schema::Step::Template { builtin, source, output, .. } => {
            let src = if builtin.is_empty() { source } else { builtin };
            format!("  {i}: template {src} -> {output}")
        }
        schema::Step::Snapshot { path, snapshot_id, .. } => {
            format!("  {i}: snapshot {path} @ {snapshot_id}")
        }
        schema::Step::Restore { snapshot_id, .. } => format!("  {i}: restore {snapshot_id}"),
        schema::Step::Git { op, .. } => format!("  {i}: git {op:?}"),
        schema::Step::Http { method, url, .. } => format!("  {i}: http {method} {url}"),
        schema::Step::Import { path, add, remove, organize, .. } => {
            format!(
                "  {i}: import {path} (add: {}, remove: {}, organize: {organize})",
                add.len(),
                remove.len()
            )
        }
        schema::Step::Refactor { symbol, rename_to, path, dry_run, .. } => {
            format!(
                "  {i}: refactor '{symbol}' -> '{rename_to}' in {path}{}",
                if *dry_run { " [dry-run]" } else { "" }
            )
        }
        schema::Step::Deps { path, depth, .. } => format!("  {i}: deps {path} (depth: {depth})"),
        schema::Step::Checkpoint { checkpoint_id, restore, .. } => {
            format!(
                "  {i}: checkpoint {checkpoint_id} ({})",
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
            let mut s = format!("  {i}: boilerplate {path}");
            if add_header.is_some() { s.push_str(" +header"); }
            if add_license.is_some() { s.push_str(" +license"); }
            if add_shebang.is_some() { s.push_str(" +shebang"); }
            if *auto_imports { s.push_str(" +auto-imports"); }
            s
        }
        schema::Step::DeadCode { path, include, .. } => {
            format!("  {i}: dead_code {path} (include: {include:?})")
        }
        schema::Step::If { condition, .. } => format!("  {i}: if {condition:?}"),
        schema::Step::Each { over, .. } => format!("  {i}: each over {over:?}"),
        schema::Step::Parallel { steps, .. } => {
            format!("  {i}: parallel ({} steps)", steps.len())
        }
    }
}
