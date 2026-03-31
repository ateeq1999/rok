//! Benchmarks for rok runner
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn benchmark_json_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("JSON Parsing");

    group.bench_function("parse simple bash step", |b| {
        b.iter(|| {
            let json = r#"{
                "type": "bash",
                "cmd": "echo hello",
                "timeout_ms": 5000
            }"#;
            let _step: rok_cli::schema::Step = serde_json::from_str(json).unwrap();
        })
    });

    group.bench_function("parse complex payload", |b| {
        b.iter(|| {
            let json = r#"{
                "name": "test-task",
                "steps": [
                    {"type": "bash", "cmd": "echo 1"},
                    {"type": "bash", "cmd": "echo 2"},
                    {"type": "bash", "cmd": "echo 3"},
                    {"type": "read", "path": "./src"},
                    {"type": "write", "path": "out.txt", "content": "hello"}
                ]
            }"#;
            let _payload: rok_cli::schema::Payload = serde_json::from_str(json).unwrap();
        })
    });

    group.finish();
}

fn benchmark_file_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("File Operations");
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    for i in 0..10 {
        let file_path = temp_dir.path().join(format!("file_{}.txt", i));
        fs::write(&file_path, format!("content {}", i)).unwrap();
    }

    group.bench_function("read single file", |b| {
        b.iter(|| {
            let file_path = temp_dir.path().join("file_0.txt");
            let _content = fs::read_to_string(&file_path).unwrap();
        })
    });

    group.bench_function("read multiple files", |b| {
        b.iter(|| {
            for i in 0..10 {
                let file_path = temp_dir.path().join(format!("file_{}.txt", i));
                let _content = fs::read_to_string(&file_path).unwrap();
            }
        })
    });

    group.bench_function("write file", |b| {
        b.iter(|| {
            let file_path = temp_dir.path().join("write_test.txt");
            fs::write(&file_path, "test content").unwrap();
        })
    });

    group.finish();
}

fn benchmark_reference_resolution(c: &mut Criterion) {
    use rok_cli::schema::{GrepMatch, StepResult, StepTypeResult};

    let mut group = c.benchmark_group("Reference Resolution");

    let results = vec![StepResult {
        index: 0,
        step_type: StepTypeResult::Grep {
            pattern: "TODO".to_string(),
            matches: (0..100)
                .map(|i| GrepMatch {
                    path: format!("src/file_{}.rs", i),
                    line: i,
                    text: "// TODO: implement".to_string(),
                })
                .collect(),
        },
        status: "ok".to_string(),
        duration_ms: 100,
        stopped_pipeline: None,
    }];

    group.bench_function("resolve simple ref", |b| {
        b.iter(|| {
            let _value = rok_cli::refs::resolve_ref(0, "*", &results);
        })
    });

    group.bench_function("resolve nested ref", |b| {
        b.iter(|| {
            let _value = rok_cli::refs::resolve_ref(0, "matches[*].path", &results);
        })
    });

    group.finish();
}

fn benchmark_regex_operations(c: &mut Criterion) {
    use regex::Regex;

    let mut group = c.benchmark_group("Regex Operations");

    let text =
        "TODO: implement this\nFIXME: fix bug\nTODO: add tests\nNOTE: important\nTODO: refactor";
    let re = Regex::new(r"TODO:.*").unwrap();

    group.bench_function("regex find all", |b| {
        b.iter(|| {
            let matches: Vec<_> = re.find_iter(black_box(text)).collect();
            assert_eq!(matches.len(), 3);
        })
    });

    group.bench_function("regex replace all", |b| {
        b.iter(|| {
            let result = re.replace_all(black_box(text), "DONE: completed");
            assert!(result.contains("DONE:"));
        })
    });

    group.finish();
}

fn benchmark_execution_order(c: &mut Criterion) {
    use rok_cli::config::Config;
    use rok_cli::schema::{Options, Payload, Step};

    let mut group = c.benchmark_group("Execution Order");

    group.bench_function("build order 10 steps", |b| {
        b.iter(|| {
            let steps: Vec<Step> = (0..10)
                .map(|i| Step::Bash {
                    id: format!("step_{}", i),
                    depends_on: if i > 0 {
                        vec![format!("step_{}", i - 1)]
                    } else {
                        vec![]
                    },
                    cmd: format!("echo {}", i),
                    timeout_ms: None,
                    retry: None,
                })
                .collect();

            let payload = Payload {
                name: None,
                description: None,
                version: None,
                author: None,
                options: Options::default(),
                props: std::collections::HashMap::new(),
                steps,
            };

            let config = Config::from_options(payload.options.clone()).unwrap();
            let runner = rok_cli::runner::Runner::new(config, payload);
            // Note: We can't directly test build_execution_order as it's private
            // but we can test the full run
            let _output = runner.run();
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_json_parsing,
    benchmark_file_operations,
    benchmark_reference_resolution,
    benchmark_regex_operations,
    benchmark_execution_order,
);

criterion_main!(benches);
