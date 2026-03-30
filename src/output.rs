use crate::cli::OutputFormat;
use crate::schema::Output;
use colored::*;

pub fn format_output(output: &Output, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string(output).unwrap_or_default(),
        OutputFormat::Pretty => format_pretty(output),
        OutputFormat::Silent => String::new(),
    }
}

fn format_pretty(output: &Output) -> String {
    let mut s = String::new();

    let status_color = match output.status.as_str() {
        "ok" => "green",
        "partial" => "yellow",
        _ => "red",
    };

    s.push_str(&format!(
        "{} [{}] {} steps ({} ok, {} failed) in {}ms\n\n",
        "●".color(status_color).bold(),
        output.status.to_uppercase().color(status_color).bold(),
        output.steps_total,
        output.steps_ok,
        output.steps_failed,
        output.duration_ms
    ));

    for result in &output.results {
        let status_color = match result.status.as_str() {
            "ok" => "green",
            "error" => "red",
            "skipped" => "yellow",
            _ => "white",
        };

        s.push_str(&format!(
            "  [{}{:>3}{}] {}",
            result.status.color(status_color),
            result.index,
            "ms".color("dimmed"),
            format_step_type(result)
        ));

        s.push('\n');
    }

    s
}

fn format_step_type(result: &crate::schema::StepResult) -> String {
    use crate::schema::StepTypeResult::*;

    match &result.step_type {
        Bash {
            cmd,
            stdout,
            exit_code,
            ..
        } => {
            let mut s = format!("bash: {}", cmd.bold());
            if !stdout.is_empty() {
                s.push_str(&format!("\n        {}", stdout.trim().color("dimmed")));
            }
            if *exit_code != 0 {
                s.push_str(&format!(" [exit: {}]", exit_code));
            }
            s
        }
        Read { path, files, .. } => {
            format!("read: {} → {} files", path.bold(), files.len())
        }
        Write { path, .. } => {
            format!("write: {}", path.bold())
        }
        Patch {
            path,
            edits_applied,
            ..
        } => {
            format!("patch: {} → {} edits", path.bold(), edits_applied)
        }
        Mv { from, to } => {
            format!("mv: {} → {}", from.bold(), to.bold())
        }
        Cp { from, to, .. } => {
            format!("cp: {} → {}", from.bold(), to.bold())
        }
        Rm { path } => {
            format!("rm: {}", path.bold())
        }
        Mkdir { path } => {
            format!("mkdir: {}", path.bold())
        }
        Grep {
            pattern, matches, ..
        } => {
            format!("grep: {} → {} matches", pattern.bold(), matches.len())
        }
        Replace {
            pattern,
            files_modified,
            total_replacements,
            ..
        } => {
            format!(
                "replace: {} → {} files, {} replacements",
                pattern.bold(),
                files_modified,
                total_replacements
            )
        }
        Scan {
            path,
            file_count,
            stack,
            ..
        } => {
            format!(
                "scan: {} → {} files, stack: {}",
                path.bold(),
                file_count,
                stack.join(", ")
            )
        }
        Summarize { path, summary, .. } => {
            format!(
                "summarize: {} → {} exports, {} functions",
                path.bold(),
                summary.exports.len(),
                summary.functions.len()
            )
        }
        Extract { path, data, .. } => {
            let keys = data
                .as_object()
                .map(|o| o.keys().cloned().collect::<Vec<_>>().join(", "))
                .unwrap_or_default();
            format!("extract: {} → keys: {}", path.bold(), keys)
        }
        Diff {
            a,
            b,
            added,
            removed,
            ..
        } => {
            format!("diff: {} → {} +{} -{}", a.bold(), b, added, removed)
        }
        Lint {
            errors_count,
            warnings_count,
            ..
        } => {
            format!("lint: {} errors, {} warnings", errors_count, warnings_count)
        }
        Template {
            output, rendered, ..
        } => {
            format!(
                "template: {} → {}",
                output.bold(),
                if *rendered {
                    "ok".to_string()
                } else {
                    "failed".to_string()
                }
            )
        }
        Snapshot { path, id, .. } => {
            format!("snapshot: {} → {}", path.bold(), id)
        }
        Restore { id, restored } => {
            format!(
                "restore: {} → {}",
                id,
                if *restored {
                    "ok".to_string()
                } else {
                    "failed".to_string()
                }
            )
        }
        Git { op, output, .. } => {
            let status = output
                .get("error")
                .map(|e| e.to_string())
                .unwrap_or_else(|| "ok".to_string());
            format!("git {:?} → {}", op, status)
        }
        Http {
            method,
            url,
            status,
            ..
        } => {
            format!("http: {} {} → {}", method, url, status)
        }
        If {
            condition_met,
            branch,
            results,
            ..
        } => {
            format!(
                "if ({}) → {} ({} steps)",
                if *condition_met { "true" } else { "false" },
                branch,
                results.len()
            )
        }
        Each { items, results, .. } => {
            format!("each: {} items → {} results", items.len(), results.len())
        }
        Parallel { results, .. } => {
            format!("parallel: {} sub-steps", results.len())
        }
    }
}
