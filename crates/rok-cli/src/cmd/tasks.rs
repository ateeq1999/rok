use crate::{
    cli::OutputFormat,
    config::Config,
    config_file::apply_config,
    output::format_output,
    runner::Runner,
    schema,
};

pub fn run(name: &str, output: &OutputFormat) -> i32 {
    let task_path = std::path::Path::new(".rok/tasks").join(format!("{}.json", name));
    if !task_path.exists() {
        eprintln!("Task not found: {}", name);
        return 1;
    }

    let task_json = match std::fs::read_to_string(&task_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read task file: {}", e);
            return 1;
        }
    };
    let payload: schema::Payload = match serde_json::from_str(&task_json) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to parse task file: {}", e);
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

    let result = Runner::new(config, payload).run();
    println!("{}", format_output(&result, output));
    0
}

pub fn save(name: &str, description: Option<&str>, mut payload: schema::Payload) -> i32 {
    if payload.name.is_none() {
        payload.name = Some(name.to_string());
    }
    if let Some(desc) = description {
        payload.description = Some(desc.to_string());
    }

    let tasks_dir = std::path::Path::new(".rok/tasks");
    if let Err(e) = std::fs::create_dir_all(tasks_dir) {
        eprintln!("Failed to create tasks directory: {}", e);
        return 1;
    }

    let task_path = tasks_dir.join(format!("{}.json", name));
    let task_json = serde_json::to_string_pretty(&payload).expect("Failed to serialize task");
    std::fs::write(&task_path, task_json).expect("Failed to write task file");
    println!("Task saved: {}", name);
    0
}

pub fn list() -> i32 {
    let tasks_dir = std::path::Path::new(".rok/tasks");
    if !tasks_dir.exists() {
        println!("No saved tasks found.");
        return 0;
    }

    let mut tasks: Vec<serde_json::Value> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(tasks_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(task) = serde_json::from_str::<schema::Payload>(&content) {
                        let name = task.name.unwrap_or_else(|| {
                            entry
                                .path()
                                .file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string()
                        });
                        tasks.push(serde_json::json!({
                            "name": name,
                            "description": task.description.unwrap_or_default(),
                            "steps": task.steps.len(),
                        }));
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
            let desc = task["description"].as_str().unwrap_or("");
            println!(
                "  {} - {} ({} steps)",
                task["name"],
                if desc.is_empty() { "no description" } else { desc },
                task["steps"],
            );
        }
    }
    0
}

#[allow(clippy::zombie_processes)]
pub fn edit(name: &str) -> i32 {
    let task_path = std::path::Path::new(".rok/tasks").join(format!("{}.json", name));
    if !task_path.exists() {
        eprintln!("Task not found: {}", name);
        return 1;
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
    0
}
