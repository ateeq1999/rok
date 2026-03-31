//! Progress indicator for long-running tasks

use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Progress tracker for step execution
pub struct ProgressTracker {
    pb: ProgressBar,
    total: usize,
    current: Arc<AtomicUsize>,
}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new(total: usize) -> Self {
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("=>-"),
        );

        Self {
            pb,
            total,
            current: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Get the current progress counter (for sharing with execution threads)
    pub fn get_counter(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.current)
    }

    /// Update progress with step information
    pub fn update(&self, step_name: &str, status: &str) {
        let current = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        self.pb.set_position(current as u64);
        self.pb.set_message(format!("{} ({})", step_name, status));

        if current >= self.total {
            self.pb.finish_with_message("Complete!");
        }
    }

    /// Mark a step as started
    pub fn step_started(&self, step_name: &str) {
        let current = self.current.load(Ordering::SeqCst);
        self.pb.set_position(current as u64);
        self.pb.set_message(format!("Running: {}", step_name));
    }

    /// Mark a step as completed
    pub fn step_completed(&self, step_name: &str, status: &str) {
        let current = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        self.pb.set_position(current as u64);
        let icon = match status {
            "ok" => "✓",
            "error" => "✗",
            "skipped" => "⊘",
            _ => "•",
        };
        self.pb.set_message(format!("{} {}", icon, step_name));

        if current >= self.total {
            self.pb.finish_with_message("✓ All steps complete");
        }
    }

    /// Finish the progress bar
    pub fn finish(&self) {
        self.pb.finish_with_message("✓ Complete");
    }
}

/// Simple progress reporter for verbose mode
#[allow(dead_code)]
pub struct ProgressReporter {
    current: AtomicUsize,
    #[allow(dead_code)]
    total: usize,
}

#[allow(dead_code)]
impl ProgressReporter {
    pub fn new(total: usize) -> Self {
        Self {
            current: AtomicUsize::new(0),
            total,
        }
    }

    pub fn report(&self, _step_index: usize, step_name: &str, status: &str) {
        let current = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        let icon = match status {
            "ok" => "✓",
            "error" => "✗",
            "skipped" => "⊘",
            _ => "•",
        };
        eprintln!("[{}/{}] {} {}", current, self.total, icon, step_name);
    }
}

/// Check if we should show progress (TTY and not quiet mode)
pub fn should_show_progress() -> bool {
    atty::is(atty::Stream::Stdout)
}
