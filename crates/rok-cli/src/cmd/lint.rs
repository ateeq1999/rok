use rok_lint::{
    rules::{LongLineRule, TodoCommentRule, TrailingWhitespaceRule},
    Linter, Severity,
};

pub fn run(path: &std::path::Path, max_line_length: usize) -> i32 {
    let mut linter = Linter::new();
    linter.add_rule(TodoCommentRule::default());
    linter.add_rule(LongLineRule::new(max_line_length));
    linter.add_rule(TrailingWhitespaceRule);

    let files = collect_files(path);
    if files.is_empty() {
        println!("No source files found in {}", path.display());
        return 0;
    }

    let mut total = 0usize;
    let mut errors = 0usize;

    for file in &files {
        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let diags = linter.check_str(file.to_str().unwrap_or("?"), &content);
        for d in &diags {
            if d.severity == Severity::Error {
                errors += 1;
            }
            println!("{}:{}  [{}]  {}  {}", d.file, d.line, d.severity, d.rule, d.message);
        }
        total += diags.len();
    }

    if total == 0 {
        println!("No issues found in {} file(s).", files.len());
    } else {
        println!("\n{} issue(s) in {} file(s).", total, files.len());
    }

    if errors > 0 { 1 } else { 0 }
}

fn collect_files(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    if path.is_file() {
        return vec![path.to_path_buf()];
    }
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .map(|x| matches!(x, "rs" | "ts" | "js" | "py" | "go"))
                .unwrap_or(false)
        })
        .filter(|e| {
            !e.path()
                .components()
                .any(|c| c.as_os_str() == "target" || c.as_os_str() == "node_modules")
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}
