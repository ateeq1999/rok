use crate::{
    cli::OutputFormat,
    config::Config,
    config_file::apply_config,
    output::format_output,
    runner::Runner,
    schema,
};

pub fn show(count: usize) -> i32 {
    let history_file = std::path::Path::new(".rok/history.json");
    if !history_file.exists() {
        println!("No execution history found.");
        return 0;
    }

    let history: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(history_file).unwrap_or_default())
            .unwrap_or_default();

    if history.is_empty() {
        println!("No execution history found.");
    } else {
        println!("Execution history (last {}):", count);
        for entry in history.iter().take(count) {
            println!(
                "  {} - {} - {} steps - {}ms",
                entry["run_id"].as_str().unwrap_or("?"),
                entry["status"].as_str().unwrap_or("?"),
                entry["steps_total"].as_u64().unwrap_or(0),
                entry["duration_ms"].as_u64().unwrap_or(0),
            );
        }
    }
    0
}

pub fn replay(run_id: Option<String>, output: &OutputFormat) -> i32 {
    let history_file = std::path::Path::new(".rok/history.json");
    if !history_file.exists() {
        eprintln!("No execution history found.");
        return 1;
    }

    let history: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(history_file).unwrap_or_default())
            .unwrap_or_default();

    let target_id = run_id.unwrap_or_else(|| {
        history
            .first()
            .map(|e| e["run_id"].as_str().unwrap_or("").to_string())
            .unwrap_or_default()
    });

    let entry = history
        .iter()
        .find(|e| e["run_id"].as_str() == Some(&target_id));

    match entry {
        Some(entry) => {
            let payload_json = entry["payload"].as_str().unwrap_or("{}");
            let payload: schema::Payload = match serde_json::from_str(payload_json) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse payload: {}", e);
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

            println!("Replaying run {}...", target_id);
            let result = Runner::new(config, payload).run();
            let formatted = format_output(&result, output);
            if !formatted.is_empty() {
                println!("{}", formatted);
            }
            0
        }
        None => {
            eprintln!("Run {} not found in history.", target_id);
            1
        }
    }
}
