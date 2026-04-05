use crate::{
    cli::OutputFormat,
    config::Config,
    config_file::apply_config,
    output::format_output,
    runner::Runner,
    schema,
};

pub fn watch(
    file: Option<String>,
    _watch_paths: Vec<String>,
    interval_ms: u64,
    output: &OutputFormat,
) -> i32 {
    let file_path = match file {
        Some(f) => f,
        None => {
            eprintln!("Error: --file required for watch mode");
            return 1;
        }
    };

    println!("Watching '{}' for changes (Ctrl+C to stop)...", file_path);

    loop {
        let raw = match std::fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return 1;
            }
        };
        let payload: schema::Payload = match serde_json::from_str(&raw) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                return 1;
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

        println!(
            "\n--- Running at {} ---",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );

        let result = Runner::new(config, payload).run();
        let formatted = format_output(&result, output);
        if !formatted.is_empty() {
            println!("{}", formatted);
        }

        std::thread::sleep(std::time::Duration::from_millis(interval_ms));
    }
}
