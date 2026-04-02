//! Progress indicator for long-running tasks

use std::sync::atomic::{AtomicUsize, Ordering};

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
