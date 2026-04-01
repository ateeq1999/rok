//! Integration tests for rok CLI
//!
//! These tests verify end-to-end functionality of the rok CLI tool.

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use std::fs;
use tempfile::TempDir;

/// Helper to create a rok command
fn rok_cmd() -> Command {
    Command::cargo_bin("rok").unwrap()
}

#[test]
fn test_version_flag() {
    let mut cmd = rok_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rok"));
}

#[test]
fn test_help_flag() {
    let mut cmd = rok_cmd();
    cmd.arg("--help");
    let assert = cmd.assert().success();
    // Just verify it runs successfully, output may vary
    assert.stdout(predicate::str::contains("rok"));
}

#[test]
fn test_run_from_json_inline() {
    let mut cmd = rok_cmd();
    cmd.arg("-j")
        .arg(r#"{"steps":[{"type":"bash","cmd":"echo hello"}]}"#);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_run_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let task_file = temp_dir.path().join("task.json");

    let json = json!({
        "steps": [
            {"type": "bash", "cmd": "echo test-from-file"}
        ]
    });

    fs::write(&task_file, json.to_string()).unwrap();

    let mut cmd = rok_cmd();
    cmd.arg("-f").arg(task_file);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-from-file"));
}

#[test]
fn test_dry_run() {
    let mut cmd = rok_cmd();
    cmd.arg("--dry-run")
        .arg("-j")
        .arg(r#"{"steps":[{"type":"bash","cmd":"echo hello"}]}"#);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Dry run"));
}

#[test]
fn test_multiple_steps() {
    let mut cmd = rok_cmd();
    let json = json!({
        "steps": [
            {"type": "bash", "cmd": "echo step1"},
            {"type": "bash", "cmd": "echo step2"},
            {"type": "bash", "cmd": "echo step3"}
        ]
    });
    cmd.arg("-j").arg(json.to_string());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("step1"))
        .stdout(predicate::str::contains("step2"));
}

#[test]
fn test_step_with_id() {
    let mut cmd = rok_cmd();
    let json = json!({
        "steps": [
            {"type": "bash", "id": "my-step", "cmd": "echo hello"}
        ]
    });
    cmd.arg("-j").arg(json.to_string());
    cmd.assert().success();
}

#[test]
fn test_step_dependencies() {
    let mut cmd = rok_cmd();
    let json = json!({
        "steps": [
            {"type": "bash", "id": "step1", "cmd": "echo first"},
            {"type": "bash", "id": "step2", "depends_on": ["step1"], "cmd": "echo second"}
        ]
    });
    cmd.arg("-j").arg(json.to_string());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("first"))
        .stdout(predicate::str::contains("second"));
}

#[test]
fn test_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    let cwd = temp_dir.path().to_string_lossy().to_string();

    let json = json!({
        "options": {"cwd": cwd},
        "steps": [
            {"type": "write", "path": "test.txt", "content": "hello world"},
            {"type": "read", "path": "test.txt"}
        ]
    });

    let mut cmd = rok_cmd();
    cmd.arg("-j").arg(json.to_string());
    cmd.assert().success();

    // Verify file was written
    assert!(test_file.exists());
    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "hello world");
}

#[test]
fn test_mkdir() {
    let temp_dir = TempDir::new().unwrap();
    let new_dir = temp_dir.path().join("new-directory");
    let cwd = temp_dir.path().to_string_lossy().to_string();

    let json = json!({
        "options": {"cwd": cwd},
        "steps": [
            {"type": "mkdir", "path": "new-directory"}
        ]
    });

    let mut cmd = rok_cmd();
    cmd.arg("-j").arg(json.to_string());
    cmd.assert().success();

    assert!(new_dir.exists());
    assert!(new_dir.is_dir());
}

#[test]
fn test_grep_step() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(
        &test_file,
        "TODO: implement this\nSome other text\nTODO: fix bug",
    )
    .unwrap();
    let cwd = temp_dir.path().to_string_lossy().to_string();

    let json = json!({
        "options": {"cwd": cwd},
        "steps": [
            {"type": "grep", "pattern": "TODO", "path": "test.txt"}
        ]
    });

    let mut cmd = rok_cmd();
    cmd.arg("-j").arg(json.to_string());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("matches"));
}

#[test]
fn test_if_step_condition_true() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("exists.txt");
    fs::write(&test_file, "content").unwrap();
    let cwd = temp_dir.path().to_string_lossy().to_string();

    let json = json!({
        "options": {"cwd": cwd},
        "steps": [
            {
                "type": "if",
                "condition": {"type": "exists", "path": "exists.txt"},
                "then": [
                    {"type": "bash", "cmd": "echo file exists"}
                ],
                "else": [
                    {"type": "bash", "cmd": "echo file missing"}
                ]
            }
        ]
    });

    let mut cmd = rok_cmd();
    cmd.arg("-j").arg(json.to_string());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("then"));
}

#[test]
fn test_if_step_condition_false() {
    let temp_dir = TempDir::new().unwrap();
    let cwd = temp_dir.path().to_string_lossy().to_string();

    let json = json!({
        "options": {"cwd": cwd},
        "steps": [
            {
                "type": "if",
                "condition": {"type": "exists", "path": "nonexistent.txt"},
                "then": [
                    {"type": "bash", "cmd": "echo should not run"}
                ],
                "else": [
                    {"type": "bash", "cmd": "echo file not found"}
                ]
            }
        ]
    });

    let mut cmd = rok_cmd();
    cmd.arg("-j").arg(json.to_string());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("else"));
}

#[test]
fn test_each_step() {
    let json = json!({
        "steps": [
            {
                "type": "each",
                "over": ["item1", "item2", "item3"],
                "as": "item",
                "step": {"type": "bash", "cmd": "echo {{item}}"}
            }
        ]
    });

    let mut cmd = rok_cmd();
    cmd.arg("-j").arg(json.to_string());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("item1"))
        .stdout(predicate::str::contains("item2"))
        .stdout(predicate::str::contains("item3"));
}

#[test]
fn test_invalid_json() {
    let mut cmd = rok_cmd();
    cmd.arg("-j").arg(r#"{"steps": invalid json}"#);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid JSON"));
}

#[test]
fn test_missing_input() {
    let mut cmd = rok_cmd();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No input provided"));
}

#[test]
fn test_output_format_json() {
    let mut cmd = rok_cmd();
    cmd.arg("-o")
        .arg("json")
        .arg("-j")
        .arg(r#"{"steps":[{"type":"bash","cmd":"echo test"}]}"#);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"status\""));
}

#[test]
fn test_verbose_flag() {
    let mut cmd = rok_cmd();
    cmd.arg("--verbose")
        .arg("-j")
        .arg(r#"{"steps":[{"type":"bash","cmd":"echo test"}]}"#);
    cmd.assert().success();
}

#[test]
fn test_quiet_flag() {
    let mut cmd = rok_cmd();
    cmd.arg("--quiet")
        .arg("-j")
        .arg(r#"{"steps":[{"type":"bash","cmd":"echo test"}]}"#);
    cmd.assert().success();
}

#[test]
fn test_templates_command() {
    let mut cmd = rok_cmd();
    cmd.arg("templates");
    cmd.assert().success();
}

#[test]
fn test_list_command_empty() {
    // Create a temp directory with empty .rok/tasks
    let temp_dir = TempDir::new().unwrap();
    let tasks_dir = temp_dir.path().join(".rok").join("tasks");
    fs::create_dir_all(&tasks_dir).unwrap();

    let mut cmd = rok_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No saved tasks"));
}

#[test]
fn test_history_command_empty() {
    // Create a temp directory without history
    let temp_dir = TempDir::new().unwrap();
    fs::create_dir_all(temp_dir.path().join(".rok")).unwrap();

    let mut cmd = rok_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg("history");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No execution history"));
}
