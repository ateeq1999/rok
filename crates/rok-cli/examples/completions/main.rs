//! Generate shell completions for rok
//!
//! Usage: cargo run --example completions <shell>
//! Where <shell> is one of: bash, zsh, fish, powershell, elvish

use clap::CommandFactory;
use clap_complete::{generate, Generator, Shell};
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: completions <shell>");
        eprintln!("Supported shells: bash, zsh, fish, powershell, elvish");
        std::process::exit(1);
    }

    let shell = &args[1];
    let gen = match shell.as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => {
            eprintln!("Unknown shell: {}", shell);
            eprintln!("Supported shells: bash, zsh, fish, powershell, elvish");
            std::process::exit(1);
        }
    };

    print_completions(gen);
}

fn print_completions<G: Generator>(gen: G) {
    let mut cmd = rok_cli::cli::Cli::command();
    generate(gen, &mut cmd, "rok", &mut io::stdout());
}
