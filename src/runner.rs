use crate::config::Config;
use crate::schema::{Output, Payload, Step, StepResult, StepTypeResult};
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

            let result = self.execute_step(step, index);
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

    fn execute_step(&self, step: &Step, index: usize) -> StepResult {
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
            Step::Parallel { steps } => self.run_parallel(steps, index),
        }
    }

    fn run_parallel(&self, steps: &[Step], parent_index: usize) -> StepResult {
        let start = Instant::now();

        let results: Vec<_> = steps
            .par_iter()
            .enumerate()
            .map(|(i, step)| self.execute_step(step, i))
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
