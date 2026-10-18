//! Where training time and memory go: each stage's wall time and the
//! process's peak memory over it (Linux's `VmHWM`, reset at each stage's
//! start; none elsewhere). The allocator keeps memory a stage freed, so a
//! stage's peak is at least what the stages before it left in use.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// One stage: its name, seconds, and the peak resident memory over it, in
/// MB.
#[derive(Debug, Clone, PartialEq)]
pub struct Stage {
    pub name: &'static str,
    pub secs: f64,
    pub peak_mb: Option<u64>,
}

/// The stages so far, and when the current one began.
#[derive(Debug, Clone)]
pub struct Trace {
    pub stages: Vec<Stage>,
    at: Instant,
}

impl Default for Trace {
    fn default() -> Trace {
        reset_peak();
        Trace {
            stages: Vec::new(),
            at: Instant::now(),
        }
    }
}

impl Trace {
    /// End the current stage as `name`, and start the next.
    pub fn mark(&mut self, name: &'static str) {
        self.stages.push(Stage {
            name,
            secs: self.at.elapsed().as_secs_f64(),
            peak_mb: peak_mb(),
        });
        reset_peak();
        self.at = Instant::now();
    }

    /// Seconds over every stage, and the highest peak.
    pub fn total(&self) -> (f64, Option<u64>) {
        let secs = self.stages.iter().map(|s| s.secs).sum();
        let peak = self.stages.iter().filter_map(|s| s.peak_mb).max();
        (secs, peak)
    }

    /// One line: each stage's seconds and peak.
    pub fn line(&self) -> String {
        self.stages
            .iter()
            .map(|s| match s.peak_mb {
                Some(mb) => format!("{} {:.1}s {mb} MB", s.name, s.secs),
                None => format!("{} {:.1}s", s.name, s.secs),
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// The process's highest peak before the last reset, in MB.
static EARLIER_PEAK: AtomicU64 = AtomicU64::new(0);

/// Start measuring the process's peak memory afresh (Linux: writing 5 to
/// `clear_refs` resets `VmHWM` to the memory in use), keeping the peak so
/// far for [`process_peak_mb`].
pub fn reset_peak() {
    if let Some(mb) = peak_mb() {
        EARLIER_PEAK.fetch_max(mb, Ordering::Relaxed);
    }
    let _ = std::fs::write("/proc/self/clear_refs", "5");
}

/// The process's peak resident memory since it started, through every
/// reset, in MB.
pub fn process_peak_mb() -> Option<u64> {
    Some(peak_mb()?.max(EARLIER_PEAK.load(Ordering::Relaxed)))
}

/// The process's peak resident memory since it was last reset, in MB.
pub fn peak_mb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|l| l.starts_with("VmHWM:"))?;
    let kb: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb / 1024)
}
