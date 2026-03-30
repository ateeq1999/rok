use crate::config::Config;
use crate::refs;
use crate::schema::{Condition, EachOver, Output, Payload, Step, StepResult, StepTypeResult};
use chrono::Utc;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub struct Runner {
    config: Config,
    payload: Payload,
}

impl Runner {
    pub fn new(config: Config, payload: Payload) -> Self {
        Self { config, payload }
    }

    pub fn run(&self) -> Output {
        let start = Instant::now();

        let execution_order = self.build_execution_order();

        let mut results_map: HashMap<usize, StepResult> = HashMap::new();
        let mut steps_ok = 0;
        let mut steps_failed = 0;
        let mut stopped = false;

        for (original_index, step) in execution_order {
            if stopped {
                results_map.insert(
                    original_index,
                    StepResult {
                        index: original_index,
                        step_type: StepTypeResult::Bash {
                            cmd: "".to_string(),
                            stdout: "".to_string(),
                            stderr: "Skipped due to previous error".to_string(),
                            exit_code: -1,
                        },
                        status: "skipped".to_string(),
                        duration_ms: 0,
                        stopped_pipeline: Some(true),
                    },
                );
                continue;
            }

            let results_vec: Vec<StepResult> = results_map.values().cloned().collect();
            let result = self.execute_step(step, original_index, &results_vec);
            if result.status == "ok" {
                steps_ok += 1;
            } else {
                steps_failed += 1;
                if self.config.options.stop_on_error {
                    stopped = true;
                }
            }
            results_map.insert(original_index, result);
        }

        let results: Vec<StepResult> = (0..self.payload.steps.len())
            .map(|i| results_map.remove(&i).unwrap())
            .collect();

        let duration_ms = start.elapsed().as_millis() as u64;
        let status = if stopped || steps_failed > 0 {
            if steps_ok == results.len() {
                "ok".to_string()
            } else {
                "partial".to_string()
            }
        } else {
            "ok".to_string()
        };

        let output = Output {
            status,
            steps_total: self.payload.steps.len(),
            steps_ok,
            steps_failed,
            duration_ms,
            results,
        };

        self.save_history(&output);

        output
    }

    fn build_execution_order(&self) -> Vec<(usize, &Step)> {
        let mut id_to_index: HashMap<String, usize> = HashMap::new();
        for (i, step) in self.payload.steps.iter().enumerate() {
            let id = step.get_id();
            if !id.is_empty() {
                id_to_index.insert(id.to_string(), i);
            }
        }

        let mut in_degree: HashMap<usize, usize> = HashMap::new();
        let mut deps: HashMap<usize, Vec<usize>> = HashMap::new();

        for (i, step) in self.payload.steps.iter().enumerate() {
            in_degree.insert(i, 0);
            let mut step_deps = Vec::new();
            for dep_id in step.get_depends_on() {
                if let Some(&dep_index) = id_to_index.get(dep_id) {
                    step_deps.push(dep_index);
                    *in_degree.entry(i).or_insert(0) += 1;
                }
            }
            deps.insert(i, step_deps);
        }

        let mut queue: Vec<usize> = in_degree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&k, _)| k)
            .collect();

        queue.sort();

        let mut execution_order = Vec::new();
        let mut visited = vec![false; self.payload.steps.len()];

        while let Some(i) = queue.pop() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            execution_order.push((i, &self.payload.steps[i]));

            for (j, _) in self.payload.steps.iter().enumerate() {
                if let Some(j_deps) = deps.get(&j) {
                    if j_deps.contains(&i) {
                        if let Some(deg) = in_degree.get_mut(&j) {
                            *deg -= 1;
                            if *deg == 0 {
                                queue.push(j);
                                queue.sort();
                            }
                        }
                    }
                }
            }
        }

        for (i, step) in self.payload.steps.iter().enumerate() {
            if !step.get_depends_on().is_empty() && !visited[i] {
                eprintln!("Warning: step {} has unmet dependencies", step.get_id());
                execution_order.push((i, step));
            }
        }

        execution_order
    }

    fn save_history(&self, output: &Output) {
        let run_id = format!("{}", Utc::now().format("%Y%m%d-%H%M%S"));

        let history_entry = serde_json::json!({
            "run_id": run_id,
            "status": output.status,
            "steps_total": output.steps_total,
            "steps_ok": output.steps_ok,
            "steps_failed": output.steps_failed,
            "duration_ms": output.duration_ms,
            "payload": serde_json::to_string(&self.payload).unwrap_or_default(),
            "timestamp": Utc::now().to_rfc3339(),
        });

        let history_file = std::path::Path::new(".rok/history.json");
        let mut history: Vec<serde_json::Value> = if history_file.exists() {
            serde_json::from_str(&std::fs::read_to_string(history_file).unwrap_or_default())
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        history.insert(0, history_entry);

        if history.len() > 100 {
            history.truncate(100);
        }

        if let Some(parent) = history_file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let _ = std::fs::write(
            history_file,
            serde_json::to_string_pretty(&history).unwrap_or_default(),
        );
    }

    fn execute_step(&self, step: &Step, index: usize, prev_results: &[StepResult]) -> StepResult {
        match step {
            Step::Bash {
                cmd,
                timeout_ms,
                retry,
                ..
            } => {
                let cmd_with_env = refs::substitute_env_vars(cmd);

                let result = if let Some(retry_config) = retry {
                    self.run_with_retry(&cmd_with_env, timeout_ms, retry_config)
                } else if let Some(timeout) = timeout_ms {
                    self.run_with_timeout(&cmd_with_env, *timeout)
                } else {
                    crate::steps::bash::run(&cmd_with_env, &self.config.cwd, &self.config.env)
                };

                let mut result = result;
                result.index = index;
                result
            }
            Step::Read { path, .. } => {
                let path_with_env = refs::substitute_env_vars(path);
                let mut result = crate::steps::read::run(&path_with_env, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Write { path, content, .. } => {
                let path_with_env = refs::substitute_env_vars(path);
                let content_with_env = refs::substitute_env_vars(content);
                let mut result =
                    crate::steps::write::run(&path_with_env, &content_with_env, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Patch { path, edits, .. } => {
                let mut result = crate::steps::patch::run(path, edits, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Mv { from, to, .. } => {
                let mut result = crate::steps::mv::run(from, to, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Cp {
                from,
                to,
                recursive,
                ..
            } => {
                let mut result = crate::steps::cp::run(from, to, *recursive, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Rm {
                path, recursive, ..
            } => {
                let mut result = crate::steps::rm::run(path, *recursive, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Mkdir { path, .. } => {
                let mut result = crate::steps::mkdir::run(path, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Grep {
                pattern,
                path,
                ext,
                regex,
                ..
            } => {
                let pattern_with_env = refs::substitute_env_vars(pattern);
                let path_with_env = refs::substitute_env_vars(path);
                let mut result = crate::steps::grep::run(
                    &pattern_with_env,
                    &path_with_env,
                    ext,
                    *regex,
                    &self.config.cwd,
                );
                result.index = index;
                result
            }
            Step::Replace {
                pattern,
                replacement,
                path,
                ext,
                regex,
                ..
            } => {
                let pattern_with_env = refs::substitute_env_vars(pattern);
                let replacement_with_env = refs::substitute_env_vars(replacement);
                let path_with_env = refs::substitute_env_vars(path);
                let mut result = crate::steps::replace::run(
                    &pattern_with_env,
                    &replacement_with_env,
                    &path_with_env,
                    ext,
                    *regex,
                    &self.config.cwd,
                );
                result.index = index;
                result
            }
            Step::Scan {
                path,
                depth,
                include,
                output,
                ..
            } => {
                let mut result =
                    crate::steps::scan::run(path, *depth, include, output, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Summarize { path, focus, .. } => {
                let mut result = crate::steps::summarize::run(path, focus, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Extract { path, pick, .. } => {
                let mut result = crate::steps::extract::run(path, pick, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Diff { a, b, format, .. } => {
                let mut result = crate::steps::diff::run(a, b, format, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Lint { path, tool, .. } => {
                let mut result = crate::steps::lint::run(path, tool, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Template {
                name,
                builtin,
                source,
                output,
                vars,
                ..
            } => {
                let mut result = crate::steps::template::run(
                    name,
                    builtin,
                    source,
                    output,
                    vars,
                    &self.config.cwd,
                );
                result.index = index;
                result
            }
            Step::Snapshot {
                path, snapshot_id, ..
            } => {
                let mut result =
                    crate::steps::snapshot::snapshot(path, snapshot_id, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Restore { snapshot_id, .. } => {
                let mut result = crate::steps::snapshot::restore(snapshot_id, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Git { op, args, .. } => {
                let mut result = crate::steps::git::run(op, args, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Http {
                method,
                url,
                headers,
                expect_status,
                body,
                ..
            } => {
                let mut result = crate::steps::http::run(
                    method,
                    url,
                    headers,
                    *expect_status,
                    body,
                    &self.config.cwd,
                );
                result.index = index;
                result
            }
            Step::If {
                condition,
                then,
                else_,
                ..
            } => self.run_if(condition, then, else_, index, prev_results),
            Step::Each {
                over,
                as_,
                parallel,
                step,
                ..
            } => self.run_each(over, step, as_, *parallel, index, prev_results),
            Step::Parallel { steps, .. } => self.run_parallel(steps, index, prev_results),
        }
    }

    fn run_with_timeout(&self, cmd: &str, timeout_ms: u64) -> crate::schema::StepResult {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();
        let cmd_owned = cmd.to_string();
        let cwd = self.config.cwd.clone();
        let env = self.config.env.clone();

        thread::spawn(move || {
            let result = crate::steps::bash::run(&cmd_owned, &cwd, &env);
            let _ = tx.send(result);
        });

        match rx.recv_timeout(Duration::from_millis(timeout_ms)) {
            Ok(result) => result,
            Err(_) => crate::schema::StepResult {
                index: 0,
                step_type: crate::schema::StepTypeResult::Bash {
                    cmd: cmd.to_string(),
                    stdout: String::new(),
                    stderr: format!("Command timed out after {}ms", timeout_ms),
                    exit_code: -1,
                },
                status: "error".to_string(),
                duration_ms: timeout_ms,
                stopped_pipeline: None,
            },
        }
    }

    fn run_with_retry(
        &self,
        cmd: &str,
        timeout_ms: &Option<u64>,
        retry_config: &crate::schema::RetryConfig,
    ) -> crate::schema::StepResult {
        let mut last_result: Option<crate::schema::StepResult> = None;
        let mut delay = retry_config.delay_ms;

        for attempt in 0..retry_config.count {
            if attempt > 0 {
                std::thread::sleep(std::time::Duration::from_millis(delay));
                if retry_config.backoff {
                    delay *= 2;
                }
            }

            let result = if let Some(timeout) = timeout_ms {
                self.run_with_timeout(cmd, *timeout)
            } else {
                let cmd_with_env = refs::substitute_env_vars(cmd);
                crate::steps::bash::run(&cmd_with_env, &self.config.cwd, &self.config.env)
            };

            if result.status == "ok" {
                return result;
            }

            last_result = Some(result);
        }

        last_result.unwrap_or_else(|| {
            let cmd_with_env = refs::substitute_env_vars(cmd);
            crate::steps::bash::run(&cmd_with_env, &self.config.cwd, &self.config.env)
        })
    }

    fn run_if(
        &self,
        condition: &Condition,
        then: &[Step],
        else_: &[Step],
        index: usize,
        prev_results: &[StepResult],
    ) -> StepResult {
        let start = Instant::now();
        let condition_met = self.eval_condition(condition, prev_results);

        let branch_steps = if condition_met { then } else { else_ };
        let branch_name = if condition_met {
            "then".to_string()
        } else {
            "else".to_string()
        };

        let mut results = Vec::new();
        for (i, step) in branch_steps.iter().enumerate() {
            let result = self.execute_step(step, i, prev_results);
            results.push(result);
        }

        let duration_ms = start.elapsed().as_millis() as u64;
        let all_ok = results.iter().all(|r| r.status == "ok");

        StepResult {
            index,
            step_type: StepTypeResult::If {
                condition_met,
                branch: branch_name,
                results,
            },
            status: if all_ok {
                "ok".to_string()
            } else {
                "partial".to_string()
            },
            duration_ms,
            stopped_pipeline: None,
        }
    }

    fn eval_condition(&self, condition: &Condition, prev_results: &[StepResult]) -> bool {
        match condition {
            Condition::Exists { path } => {
                let full_path = self.config.cwd.join(path);
                full_path.exists()
            }
            Condition::Contains { path, pattern, .. } => {
                let full_path = self.config.cwd.join(path);
                if let Ok(content) = std::fs::read_to_string(full_path) {
                    content.contains(pattern)
                } else {
                    false
                }
            }
            Condition::GrepHasResults { ref_ } => refs::has_grep_results(*ref_, prev_results),
            Condition::StepOk { ref_ } => refs::step_ok(*ref_, prev_results),
            Condition::StepFailed { ref_ } => refs::step_failed(*ref_, prev_results),
            Condition::FileChanged { path, since } => {
                let full_path = self.config.cwd.join(path);
                if let Ok(metadata) = std::fs::metadata(full_path) {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(since_date) = chrono::DateTime::parse_from_rfc3339(since) {
                            let modified_dt: chrono::DateTime<chrono::Utc> = modified.into();
                            return modified_dt > since_date.with_timezone(&chrono::Utc);
                        }
                    }
                }
                false
            }
            Condition::Not { condition } => !self.eval_condition(condition, prev_results),
            Condition::And { conditions } => conditions
                .iter()
                .all(|c| self.eval_condition(c, prev_results)),
            Condition::Or { conditions } => conditions
                .iter()
                .any(|c| self.eval_condition(c, prev_results)),
        }
    }

    fn run_each(
        &self,
        over: &EachOver,
        step: &Step,
        as_var: &str,
        parallel: bool,
        index: usize,
        prev_results: &[StepResult],
    ) -> StepResult {
        let start = Instant::now();

        let items: Vec<String> = match over {
            EachOver::List(list) => list.clone(),
            EachOver::Ref(r) => {
                let json = refs::resolve_ref(r.ref_, &r.pick, prev_results);
                json.and_then(|v| {
                    v.as_array().map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(String::from))
                            .collect()
                    })
                })
                .unwrap_or_default()
            }
        };

        let step_closure = step.clone();

        let results: Vec<_> = if parallel {
            items
                .par_iter()
                .enumerate()
                .map(|(i, item)| {
                    let substituted = self.substitute_step(&step_closure, as_var, item);
                    self.execute_step(&substituted, i, prev_results)
                })
                .collect()
        } else {
            items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let substituted = self.substitute_step(&step_closure, as_var, item);
                    self.execute_step(&substituted, i, prev_results)
                })
                .collect()
        };

        let duration_ms = start.elapsed().as_millis() as u64;
        let all_ok = results.iter().all(|r| r.status == "ok");

        StepResult {
            index,
            step_type: StepTypeResult::Each { items, results },
            status: if all_ok {
                "ok".to_string()
            } else {
                "partial".to_string()
            },
            duration_ms,
            stopped_pipeline: None,
        }
    }

    fn substitute_step(&self, step: &Step, var_name: &str, value: &str) -> Step {
        fn sub(step: &Step, var: &str, val: &str) -> Step {
            match step {
                Step::Bash {
                    cmd,
                    id,
                    timeout_ms,
                    retry,
                    depends_on,
                    ..
                } => Step::Bash {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    cmd: refs::substitute_vars(cmd, var, val),
                    timeout_ms: *timeout_ms,
                    retry: retry.clone(),
                },
                Step::Read {
                    path,
                    id,
                    depends_on,
                    ..
                } => Step::Read {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    max_bytes: None,
                    encoding: None,
                },
                Step::Write {
                    path,
                    content,
                    id,
                    depends_on,
                    ..
                } => Step::Write {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    content: refs::substitute_vars(content, var, val),
                    create_dirs: true,
                },
                Step::Patch {
                    path,
                    edits,
                    id,
                    depends_on,
                    ..
                } => Step::Patch {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    edits: edits.clone(),
                },
                Step::Mv {
                    from,
                    to,
                    id,
                    depends_on,
                    ..
                } => Step::Mv {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    from: refs::substitute_vars(from, var, val),
                    to: refs::substitute_vars(to, var, val),
                },
                Step::Cp {
                    from,
                    to,
                    recursive,
                    id,
                    depends_on,
                    ..
                } => Step::Cp {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    from: refs::substitute_vars(from, var, val),
                    to: refs::substitute_vars(to, var, val),
                    recursive: *recursive,
                },
                Step::Rm {
                    path,
                    recursive,
                    id,
                    depends_on,
                    ..
                } => Step::Rm {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    recursive: *recursive,
                },
                Step::Mkdir {
                    path,
                    id,
                    depends_on,
                    ..
                } => Step::Mkdir {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                },
                Step::Grep {
                    pattern,
                    path,
                    ext,
                    regex,
                    id,
                    depends_on,
                    ..
                } => Step::Grep {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    pattern: refs::substitute_vars(pattern, var, val),
                    path: refs::substitute_vars(path, var, val),
                    ext: ext.clone(),
                    regex: *regex,
                    context_lines: None,
                },
                Step::Replace {
                    pattern,
                    replacement,
                    path,
                    ext,
                    regex,
                    id,
                    depends_on,
                    ..
                } => Step::Replace {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    pattern: refs::substitute_vars(pattern, var, val),
                    replacement: refs::substitute_vars(replacement, var, val),
                    path: refs::substitute_vars(path, var, val),
                    ext: ext.clone(),
                    regex: *regex,
                    case_sensitive: true,
                },
                Step::Scan {
                    path,
                    depth,
                    include,
                    output,
                    id,
                    depends_on,
                    ..
                } => Step::Scan {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    depth: *depth,
                    include: include.clone(),
                    output: output.clone(),
                },
                Step::Summarize {
                    path,
                    focus,
                    id,
                    depends_on,
                    ..
                } => Step::Summarize {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    focus: focus.clone(),
                },
                Step::Extract {
                    path,
                    pick,
                    id,
                    depends_on,
                    ..
                } => Step::Extract {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    pick: pick.clone(),
                },
                Step::Diff {
                    a,
                    b,
                    format,
                    id,
                    depends_on,
                    ..
                } => Step::Diff {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    a: refs::substitute_vars(a, var, val),
                    b: refs::substitute_vars(b, var, val),
                    format: format.clone(),
                },
                Step::Lint {
                    path,
                    tool,
                    id,
                    depends_on,
                    ..
                } => Step::Lint {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    tool: tool.clone(),
                },
                Step::Template {
                    name,
                    builtin,
                    source,
                    output,
                    vars,
                    id,
                    depends_on,
                    ..
                } => Step::Template {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    name: name.clone(),
                    builtin: builtin.clone(),
                    source: source.clone(),
                    output: refs::substitute_vars(output, var, val),
                    vars: vars.clone(),
                },
                Step::Snapshot {
                    path,
                    snapshot_id,
                    id,
                    depends_on,
                    ..
                } => Step::Snapshot {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    path: refs::substitute_vars(path, var, val),
                    snapshot_id: snapshot_id.clone(),
                },
                Step::Restore {
                    snapshot_id,
                    id,
                    depends_on,
                    ..
                } => Step::Restore {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    snapshot_id: refs::substitute_vars(snapshot_id, var, val),
                },
                Step::Git {
                    op,
                    args,
                    id,
                    depends_on,
                    ..
                } => Step::Git {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    op: op.clone(),
                    args: args
                        .iter()
                        .map(|a| refs::substitute_vars(a, var, val))
                        .collect(),
                },
                Step::Http {
                    method,
                    url,
                    headers,
                    expect_status,
                    body,
                    id,
                    depends_on,
                    ..
                } => Step::Http {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    method: method.clone(),
                    url: refs::substitute_vars(url, var, val),
                    headers: headers.clone(),
                    expect_status: *expect_status,
                    body: body.clone(),
                },
                Step::If {
                    condition,
                    then,
                    else_,
                    id,
                    depends_on,
                    ..
                } => Step::If {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    condition: condition.clone(),
                    then: then.iter().map(|s| sub(s, var, val)).collect(),
                    else_: else_.iter().map(|s| sub(s, var, val)).collect(),
                },
                Step::Each {
                    over,
                    as_,
                    parallel,
                    step,
                    id,
                    depends_on,
                    ..
                } => Step::Each {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    over: over.clone(),
                    as_: as_.clone(),
                    parallel: *parallel,
                    step: Box::new(sub(step, var, val)),
                },
                Step::Parallel {
                    steps,
                    id,
                    depends_on,
                    ..
                } => Step::Parallel {
                    id: id.clone(),
                    depends_on: depends_on.clone(),
                    steps: steps.iter().map(|s| sub(s, var, val)).collect(),
                },
            }
        }

        sub(step, var_name, value)
    }

    fn run_parallel(
        &self,
        steps: &[Step],
        parent_index: usize,
        prev_results: &[StepResult],
    ) -> StepResult {
        let start = Instant::now();

        let results: Vec<_> = steps
            .par_iter()
            .enumerate()
            .map(|(i, step)| self.execute_step(step, i, prev_results))
            .collect();

        let duration_ms = start.elapsed().as_millis() as u64;
        let all_ok = results.iter().all(|r| r.status == "ok");

        StepResult {
            index: parent_index,
            step_type: StepTypeResult::Parallel { results },
            status: if all_ok {
                "ok".to_string()
            } else {
                "partial".to_string()
            },
            duration_ms,
            stopped_pipeline: None,
        }
    }
}
