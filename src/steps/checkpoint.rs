use crate::schema::{StepResult, StepTypeResult};
use std::fs;
use std::path::Path;
use std::time::Instant;

pub fn run(checkpoint_id: &str, restore: bool, cwd: &Path) -> StepResult {
    let start = Instant::now();
    let checkpoint_dir = cwd.join(".rok/checkpoints");
    let checkpoint_file = checkpoint_dir.join(format!("{}.json", checkpoint_id));

    if restore {
        // Restore checkpoint
        if !checkpoint_file.exists() {
            return StepResult {
                index: 0,
                step_type: StepTypeResult::Checkpoint {
                    checkpoint_id: checkpoint_id.to_string(),
                    action: "restore".to_string(),
                    steps_saved: 0,
                },
                status: format!("error: checkpoint '{}' not found", checkpoint_id),
                duration_ms: 0,
                stopped_pipeline: None,
            };
        }

        match fs::read_to_string(&checkpoint_file) {
            Ok(content) => {
                let data: serde_json::Value =
                    serde_json::from_str(&content).unwrap_or(serde_json::json!({}));
                let steps_saved = data["steps_saved"].as_u64().unwrap_or(0) as usize;

                let duration_ms = start.elapsed().as_millis() as u64;
                StepResult {
                    index: 0,
                    step_type: StepTypeResult::Checkpoint {
                        checkpoint_id: checkpoint_id.to_string(),
                        action: "restored".to_string(),
                        steps_saved,
                    },
                    status: "ok".to_string(),
                    duration_ms,
                    stopped_pipeline: None,
                }
            }
            Err(e) => StepResult {
                index: 0,
                step_type: StepTypeResult::Checkpoint {
                    checkpoint_id: checkpoint_id.to_string(),
                    action: "restore".to_string(),
                    steps_saved: 0,
                },
                status: format!("error: {}", e),
                duration_ms: start.elapsed().as_millis() as u64,
                stopped_pipeline: None,
            },
        }
    } else {
        // Save checkpoint
        let _ = fs::create_dir_all(&checkpoint_dir);

        let timestamp = chrono::Utc::now().to_rfc3339();
        let checkpoint_data = serde_json::json!({
            "checkpoint_id": checkpoint_id,
            "created_at": timestamp,
            "steps_saved": 0,
            "cwd": cwd.to_string_lossy(),
        });

        match fs::write(&checkpoint_file, serde_json::to_string_pretty(&checkpoint_data).unwrap_or_default()) {
            Ok(_) => {
                let duration_ms = start.elapsed().as_millis() as u64;
                StepResult {
                    index: 0,
                    step_type: StepTypeResult::Checkpoint {
                        checkpoint_id: checkpoint_id.to_string(),
                        action: "saved".to_string(),
                        steps_saved: 0,
                    },
                    status: "ok".to_string(),
                    duration_ms,
                    stopped_pipeline: None,
                }
            }
            Err(e) => StepResult {
                index: 0,
                step_type: StepTypeResult::Checkpoint {
                    checkpoint_id: checkpoint_id.to_string(),
                    action: "save".to_string(),
                    steps_saved: 0,
                },
                status: format!("error: {}", e),
                duration_ms: start.elapsed().as_millis() as u64,
                stopped_pipeline: None,
            },
        }
    }
}
