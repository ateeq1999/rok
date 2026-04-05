pub mod cache;
pub mod deploy;
pub mod generate;
pub mod history;
pub mod lint;
pub mod migrate;
pub mod serve;
pub mod tasks;
pub mod templates;
pub mod watch;

/// Read a line from stdin and return the trimmed result.
pub fn read_input() -> String {
    use std::io::Write;
    let mut input = String::new();
    std::io::stdout().flush().ok();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}
