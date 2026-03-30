use crate::config::Config;
use crate::refs;
use crate::schema::{Condition, EachOver, Output, Payload, Step, StepResult, StepTypeResult};
use rayon::prelude::*;
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
        let mut results = Vec::new();
        let mut steps_ok = 0;
        let mut steps_failed = 0;
        let mut stopped = false;

        for (index, step) in self.payload.steps.iter().enumerate() {
            if stopped {
                results.push(StepResult {
                    index,
                    step_type: StepTypeResult::Bash {
                        cmd: "".to_string(),
                        stdout: "".to_string(),
                        stderr: "Skipped due to previous error".to_string(),
                        exit_code: -1,
                    },
                    status: "skipped".to_string(),
                    duration_ms: 0,
                    stopped_pipeline: Some(true),
                });
                continue;
            }

            let result = self.execute_step(step, index, &results);
            if result.status == "ok" {
                steps_ok += 1;
            } else {
                steps_failed += 1;
                if self.config.options.stop_on_error {
                    stopped = true;
                }
            }
            results.push(result);
        }

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

        Output {
            status,
            steps_total: self.payload.steps.len(),
            steps_ok,
            steps_failed,
            duration_ms,
            results,
        }
    }

    fn execute_step(&self, step: &Step, index: usize, prev_results: &[StepResult]) -> StepResult {
        match step {
            Step::Bash { cmd } => {
                let mut result = crate::steps::bash::run(cmd, &self.config.cwd, &self.config.env);
                result.index = index;
                result
            }
            Step::Read { path } => {
                let mut result = crate::steps::read::run(path, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Write { path, content } => {
                let mut result = crate::steps::write::run(path, content, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Patch { path, edits } => {
                let mut result = crate::steps::patch::run(path, edits, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Mv { from, to } => {
                let mut result = crate::steps::mv::run(from, to, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Cp {
                from,
                to,
                recursive,
            } => {
                let mut result = crate::steps::cp::run(from, to, *recursive, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Rm { path, recursive } => {
                let mut result = crate::steps::rm::run(path, *recursive, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Mkdir { path } => {
                let mut result = crate::steps::mkdir::run(path, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Grep {
                pattern,
                path,
                ext,
                regex,
            } => {
                let mut result =
                    crate::steps::grep::run(pattern, path, ext, *regex, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Replace {
                pattern,
                replacement,
                path,
                ext,
                regex,
            } => {
                let mut result = crate::steps::replace::run(
                    pattern,
                    replacement,
                    path,
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
            } => {
                let mut result =
                    crate::steps::scan::run(path, *depth, include, output, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Summarize { path, focus } => {
                let mut result = crate::steps::summarize::run(path, focus, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Extract { path, pick } => {
                let mut result = crate::steps::extract::run(path, pick, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Diff { a, b, format } => {
                let mut result = crate::steps::diff::run(a, b, format, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Lint { path, tool } => {
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
            } => {
                let mut result = crate::steps::template::run(
                    &name,
                    builtin,
                    source,
                    output,
                    vars,
                    &self.config.cwd,
                );
                result.index = index;
                result
            }
            Step::Snapshot { path, id } => {
                let mut result = crate::steps::snapshot::snapshot(path, id, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Restore { id } => {
                let mut result = crate::steps::snapshot::restore(id, &self.config.cwd);
                result.index = index;
                result
            }
            Step::Git { op, args } => {
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
            } => self.run_if(condition, then, else_, index, prev_results),
            Step::Each {
                over,
                as_,
                parallel,
                step,
            } => self.run_each(over, step, as_, *parallel, index, prev_results),
            Step::Parallel { steps } => self.run_parallel(steps, index, prev_results),
        }
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
                Step::Bash { cmd } => Step::Bash {
                    cmd: refs::substitute_vars(cmd, var, val),
                },
                Step::Read { path } => Step::Read {
                    path: refs::substitute_vars(path, var, val),
                },
                Step::Write { path, content } => Step::Write {
                    path: refs::substitute_vars(path, var, val),
                    content: refs::substitute_vars(content, var, val),
                },
                Step::Patch { path, edits } => Step::Patch {
                    path: refs::substitute_vars(path, var, val),
                    edits: edits.clone(),
                },
                Step::Mv { from, to } => Step::Mv {
                    from: refs::substitute_vars(from, var, val),
                    to: refs::substitute_vars(to, var, val),
                },
                Step::Cp {
                    from,
                    to,
                    recursive,
                } => Step::Cp {
                    from: refs::substitute_vars(from, var, val),
                    to: refs::substitute_vars(to, var, val),
                    recursive: *recursive,
                },
                Step::Rm { path, recursive } => Step::Rm {
                    path: refs::substitute_vars(path, var, val),
                    recursive: *recursive,
                },
                Step::Mkdir { path } => Step::Mkdir {
                    path: refs::substitute_vars(path, var, val),
                },
                Step::Grep {
                    pattern,
                    path,
                    ext,
                    regex,
                } => Step::Grep {
                    pattern: refs::substitute_vars(pattern, var, val),
                    path: refs::substitute_vars(path, var, val),
                    ext: ext.clone(),
                    regex: *regex,
                },
                Step::Replace {
                    pattern,
                    replacement,
                    path,
                    ext,
                    regex,
                } => Step::Replace {
                    pattern: refs::substitute_vars(pattern, var, val),
                    replacement: refs::substitute_vars(replacement, var, val),
                    path: refs::substitute_vars(path, var, val),
                    ext: ext.clone(),
                    regex: *regex,
                },
                Step::Scan {
                    path,
                    depth,
                    include,
                    output,
                } => Step::Scan {
                    path: refs::substitute_vars(path, var, val),
                    depth: *depth,
                    include: include.clone(),
                    output: output.clone(),
                },
                Step::Summarize { path, focus } => Step::Summarize {
                    path: refs::substitute_vars(path, var, val),
                    focus: focus.clone(),
                },
                Step::Extract { path, pick } => Step::Extract {
                    path: refs::substitute_vars(path, var, val),
                    pick: pick.clone(),
                },
                Step::Diff { a, b, format } => Step::Diff {
                    a: refs::substitute_vars(a, var, val),
                    b: refs::substitute_vars(b, var, val),
                    format: format.clone(),
                },
                Step::Lint { path, tool } => Step::Lint {
                    path: refs::substitute_vars(path, var, val),
                    tool: tool.clone(),
                },
                Step::Template {
                    name,
                    builtin,
                    source,
                    output,
                    vars,
                } => Step::Template {
                    name: name.clone(),
                    builtin: builtin.clone(),
                    source: source.clone(),
                    output: refs::substitute_vars(output, var, val),
                    vars: vars.clone(),
                },
                Step::Snapshot { path, id } => Step::Snapshot {
                    path: refs::substitute_vars(path, var, val),
                    id: id.clone(),
                },
                Step::Restore { id } => Step::Restore {
                    id: refs::substitute_vars(id, var, val),
                },
                Step::Git { op, args } => Step::Git {
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
                } => Step::Http {
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
                } => Step::If {
                    condition: condition.clone(),
                    then: then.iter().map(|s| sub(s, var, val)).collect(),
                    else_: else_.iter().map(|s| sub(s, var, val)).collect(),
                },
                Step::Each {
                    over,
                    as_,
                    parallel,
                    step,
                } => Step::Each {
                    over: over.clone(),
                    as_: as_.clone(),
                    parallel: *parallel,
                    step: Box::new(sub(step, var, val)),
                },
                Step::Parallel { steps } => Step::Parallel {
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
