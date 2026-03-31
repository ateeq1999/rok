use crate::schema::{StepResult, StepTypeResult};
use std::fs;
use std::path::Path;
use std::time::Instant;

pub fn run(
    path: &str,
    add_header: &Option<String>,
    add_license: &Option<String>,
    add_shebang: &Option<String>,
    auto_imports: bool,
    cwd: &Path,
) -> StepResult {
    let start = Instant::now();
    let full_path = cwd.join(path);

    if !full_path.exists() {
        return StepResult {
            index: 0,
            step_type: StepTypeResult::Boilerplate {
                path: full_path.to_string_lossy().to_string(),
                header_added: false,
                license_added: false,
                shebang_added: false,
                imports_added: vec![],
            },
            status: "error".to_string(),
            duration_ms: 0,
            stopped_pipeline: None,
        };
    }

    let mut header_added = false;
    let mut license_added = false;
    let mut shebang_added = false;
    let mut imports_added = Vec::new();

    if let Ok(mut content) = fs::read_to_string(&full_path) {
        let ext = full_path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        // Add shebang if specified
        if let Some(shebang) = add_shebang {
            if !content.starts_with("#!") {
                content = format!("{}\n{}", shebang, content);
                shebang_added = true;
            }
        }

        // Add header comment if specified
        if let Some(header) = add_header {
            let header_comment = format_comment(header, &ext);
            if !content.contains(&header_comment) {
                content = format!("{}\n{}", header_comment, content);
                header_added = true;
            }
        }

        // Add license if specified
        if let Some(license) = add_license {
            let license_text = get_license_text(license);
            let license_comment = format_comment(&license_text, &ext);
            if !content.contains(&license_comment) {
                content = format!("{}\n{}", license_comment, content);
                license_added = true;
            }
        }

        // Auto-add standard imports for known file types
        if auto_imports {
            let (new_content, added_imports) = add_standard_imports(&content, &ext);
            content = new_content;
            imports_added = added_imports;
        }

        let _ = fs::write(&full_path, &content);
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    StepResult {
        index: 0,
        step_type: StepTypeResult::Boilerplate {
            path: full_path.to_string_lossy().to_string(),
            header_added,
            license_added,
            shebang_added,
            imports_added,
        },
        status: "ok".to_string(),
        duration_ms,
        stopped_pipeline: None,
    }
}

fn format_comment(text: &str, ext: &str) -> String {
    match ext {
        "py" => text
            .lines()
            .map(|l| format!("# {}", l))
            .collect::<Vec<_>>()
            .join("\n"),
        "rs" | "go" | "ts" | "tsx" | "js" | "jsx" | "c" | "cpp" | "h" | "hpp" => {
            if text.lines().count() > 1 {
                format!(
                    "/*\n{}\n */",
                    text.lines()
                        .map(|l| format!(" * {}", l))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            } else {
                format!("// {}", text)
            }
        }
        "html" | "xml" => format!("<!-- {} -->", text),
        "css" | "scss" | "less" => format!("/* {} */", text),
        "lua" => text
            .lines()
            .map(|l| format!("-- {}", l))
            .collect::<Vec<_>>()
            .join("\n"),
        "sql" => text
            .lines()
            .map(|l| format!("-- {}", l))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => text.to_string(),
    }
}

fn get_license_text(license: &str) -> String {
    match license.to_lowercase().as_str() {
        "mit" => "MIT License\n\nCopyright (c) 2024\n\nPermission is hereby granted...".to_string(),
        "apache" => "Apache License 2.0\n\nLicensed under the Apache License...".to_string(),
        "gpl" => "GNU General Public License v3.0\n\nThis program is free software...".to_string(),
        "bsd" => "BSD 3-Clause License\n\nCopyright (c) 2024...".to_string(),
        "isc" => "ISC License\n\nCopyright (c) 2024...".to_string(),
        "unlicense" => {
            "This is free and unencumbered software released into the public domain.".to_string()
        }
        custom => custom.to_string(),
    }
}

fn add_standard_imports(content: &str, ext: &str) -> (String, Vec<String>) {
    let mut added = Vec::new();
    let mut lines: Vec<&str> = content.lines().collect();

    match ext {
        "js" | "jsx" | "ts" | "tsx" => {
            // Check for common patterns and suggest imports
            if content.contains("useState")
                && !content.contains("import")
                && !content.contains("import { useState")
            {
                lines.insert(0, "import { useState } from 'react';");
                added.push("useState".to_string());
            }
            if content.contains("useEffect")
                && !content.contains("import { useEffect")
                && !content.contains("import { useEffect")
            {
                lines.insert(0, "import { useEffect } from 'react';");
                added.push("useEffect".to_string());
            }
            if content.contains("useCallback") && !content.contains("import { useCallback") {
                lines.insert(0, "import { useCallback } from 'react';");
                added.push("useCallback".to_string());
            }
        }
        "py" => {
            if content.contains("typing") && !content.contains("from typing import") {
                lines.insert(0, "from typing import List, Dict, Optional");
                added.push("typing".to_string());
            }
        }
        "rs" => {
            if content.contains("std::") && !content.contains("use std::") {
                lines.insert(0, "use std::collections::HashMap;");
                added.push("std::collections::HashMap".to_string());
            }
        }
        _ => {}
    }

    (lines.join("\n"), added)
}
