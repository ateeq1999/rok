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

For more info, see: https://github.com/ateeq1999/rok")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short = 'j', long = "json", conflicts_with_all = ["file", "stdin"], help = "JSON payload inline")]
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

    #[arg(long = "verbose", help = "Enable verbose output")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "List available templates")]
    Templates,
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
