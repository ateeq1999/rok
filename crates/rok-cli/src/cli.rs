use crate::error::RokError;
use crate::schema::Payload;
use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(name = "rok")]
#[command(version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.2.0"))]
#[command(about = "Run One, Know All - Execute multi-step tasks from JSON")]
#[command(long_about = "rok - AI Agent Task Runner

A CLI tool that collapses multi-step operations into a single JSON invocation.

EXAMPLES:
  rok -f task.json                    Run from file
  echo '{\"steps\":[{\"type\":\"bash\",\"cmd\":\"echo hello\"}]}' | rok    Run from stdin
  rok templates                       List available templates
  rok --help                          Show this help
  rok --verbose -f task.json          Run with verbose output
  rok -q -f task.json                 Run quietly (suppress output)

SHELL COMPLETIONS:
  Generate completions for your shell:
  - Bash: cargo run --quiet --example completion bash > /etc/bash_completion.d/rok
  - Zsh:  cargo run --quiet --example completion zsh  > ~/.zsh/_rok
  - Fish: cargo run --quiet --example completion fish > ~/.config/fish/completions/rok.fish

For more info, see: https://github.com/ateeq1999/rok")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(
        short = 'j',
        long = "json",
        conflicts_with = "file",
        help = "JSON payload inline"
    )]
    pub json: Option<String>,

    #[arg(
        short = 'f',
        long = "file",
        conflicts_with = "json",
        help = "Path to JSON file"
    )]
    pub file: Option<String>,

    #[arg(
        short = 'o',
        long = "output",
        default_value = "json",
        help = "Output format: json, pretty, silent"
    )]
    pub output: OutputFormat,

    #[arg(long = "dry-run", help = "Preview steps without executing")]
    pub dry_run: bool,

    #[arg(long = "verbose", short = 'v', help = "Enable verbose output")]
    pub verbose: bool,

    #[arg(long = "quiet", short = 'q', help = "Suppress output (except errors)")]
    pub quiet: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "List available templates")]
    Templates,

    #[command(about = "Create a new template interactively")]
    InitTemplate {
        #[arg(help = "Template name")]
        name: Option<String>,
    },

    #[command(about = "Validate a template schema")]
    ValidateTemplate {
        #[arg(help = "Path to template directory or .rok-template.json")]
        path: Option<String>,
    },

    #[command(about = "Run a saved task")]
    Run {
        #[arg(help = "Task name")]
        name: String,
    },

    #[command(about = "Save current payload as a named task")]
    Save {
        #[arg(help = "Task name")]
        name: String,

        #[arg(short = 'd', long = "description", help = "Task description")]
        description: Option<String>,
    },

    #[command(about = "List saved tasks")]
    List,

    #[command(about = "Edit a saved task")]
    Edit {
        #[arg(help = "Task name")]
        name: String,
    },

    #[command(about = "Watch files and re-run on changes")]
    Watch {
        #[arg(help = "Path to JSON file")]
        file: Option<String>,

        #[arg(short = 'w', long = "watch", help = "Files/dirs to watch")]
        watch: Option<Vec<String>>,

        #[arg(
            short = 'i',
            long = "interval",
            default_value = "1000",
            help = "Polling interval in ms"
        )]
        interval: u64,
    },

    #[command(about = "Show execution history")]
    History {
        #[arg(
            short = 'n',
            long = "count",
            default_value = "10",
            help = "Number of entries to show"
        )]
        count: usize,
    },

    #[command(about = "Replay a previous execution")]
    Replay {
        #[arg(help = "Run ID")]
        run_id: Option<String>,
    },

    #[command(about = "Show cache statistics")]
    Cache {
        #[arg(short = 's', long = "stats", help = "Show cache statistics")]
        stats: bool,

        #[arg(long = "clear", help = "Clear cache")]
        clear: bool,
    },

    #[command(about = "Manage checkpoints")]
    Checkpoints {
        #[arg(short = 'l', long = "list", help = "List all checkpoints")]
        list: bool,

        #[arg(long = "delete", help = "Delete a checkpoint by ID")]
        delete: Option<String>,
    },

    #[command(about = "Serve documentation website")]
    Serve {
        #[arg(
            short = 'p',
            long = "port",
            default_value = "8080",
            help = "Port to serve on"
        )]
        port: String,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Pretty,
    Silent,
}

impl Cli {
    pub fn parse_payload(&self) -> Result<Payload, RokError> {
        let json_str = if let Some(ref json) = self.json {
            json.clone()
        } else if let Some(ref file) = self.file {
            fs::read_to_string(file)
                .map_err(|e| RokError::schema(format!("Failed to read file: {}", e)))?
        } else {
            let mut stdin_content = String::new();
            io::stdin()
                .read_to_string(&mut stdin_content)
                .map_err(|e| RokError::schema(format!("Failed to read stdin: {}", e)))?;
            if stdin_content.trim().is_empty() {
                return Err(RokError::schema(
                    "No input provided. Use --json, --file, or pipe JSON to stdin.",
                ));
            }
            stdin_content
        };

        serde_json::from_str(&json_str)
            .map_err(|e| RokError::schema(format!("Invalid JSON: {}", e)))
    }
}
