//! Languages run side by side (`--lanes N`), each in a process of its own:
//! this binary again, on one language, behind a wrapper command if one is
//! given (`--wrap "systemd-run --user --scope -q -p MemoryMax=16G -p
//! MemorySwapMax=0"`, split at spaces). The longest run starts first, and two
//! runs that each need much memory never run at once.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// What a language's run costs, when last measured: minutes of a quick
/// `try` (the fourth panel's runs, 2026-10-16, part by part on two lanes),
/// minutes of a build (PR #19's rebuild: Finnish, Greek, Turkish, Russian
/// and German measured, the rest estimated from its 2.7 hours), and peak
/// memory in GB.
pub const COSTS: &[(&str, f64, f64, f64)] = &[
    ("finnish", 27.0, 66.0, 15.0),
    ("ancient-greek", 12.0, 21.0, 4.0),
    ("turkish", 11.5, 20.0, 15.0),
    ("russian", 7.0, 12.0, 4.0),
    ("german", 5.5, 12.0, 4.0),
    ("latin", 3.0, 9.0, 4.0),
    ("polish", 3.0, 8.0, 4.0),
    ("spanish", 1.5, 4.0, 4.0),
    ("english", 1.0, 3.0, 4.0),
    ("italian", 1.0, 3.0, 4.0),
    ("sanskrit", 0.5, 1.0, 4.0),
    ("french", 0.5, 1.0, 4.0),
];

/// A run that peaks above this many GB is heavy: no two heavy runs at once
/// (Finnish and Turkish).
const HEAVY_GB: f64 = 8.0;

/// What a run is: a quick `try` or a build.
#[derive(Clone, Copy)]
pub enum Kind {
    Try,
    Build,
}

/// One language's run: this binary's arguments, and where its output goes.
pub struct Job {
    pub lang: &'static str,
    pub args: Vec<String>,
    pub log: PathBuf,
}

/// A job's minutes (5 for a language not measured) and whether it is heavy.
pub fn cost(lang: &str, kind: Kind) -> (f64, bool) {
    COSTS
        .iter()
        .find(|c| c.0 == lang)
        .map_or((5.0, false), |&(_, quick, build, peak)| {
            let minutes = match kind {
                Kind::Try => quick,
                Kind::Build => build,
            };
            (minutes, peak > HEAVY_GB)
        })
}

/// Run `jobs`, `lanes` at once, longest first, no two heavy ones together,
/// each with its output in its log; `done` hears of each as it ends (its
/// language, whether it succeeded). Gives the languages that failed.
pub fn run(
    mut jobs: Vec<Job>,
    kind: Kind,
    lanes: usize,
    wrap: Option<&str>,
    mut done: impl FnMut(&str, bool),
) -> Result<Vec<&'static str>, String> {
    jobs.sort_by(|a, b| cost(b.lang, kind).0.total_cmp(&cost(a.lang, kind).0));
    let mut running: Vec<(Job, Child, Instant)> = Vec::new();
    let mut failed = Vec::new();
    loop {
        while running.len() < lanes.max(1) {
            let heavy = running.iter().any(|(j, _, _)| cost(j.lang, kind).1);
            let Some(next) = jobs.iter().position(|j| !(heavy && cost(j.lang, kind).1)) else {
                break;
            };
            let job = jobs.remove(next);
            let child = spawn(&job, wrap)?;
            eprintln!("{}: started, output in {}", job.lang, job.log.display());
            running.push((job, child, Instant::now()));
        }
        if running.is_empty() {
            return Ok(failed);
        }
        std::thread::sleep(Duration::from_secs(1));
        let mut still = Vec::new();
        for (job, mut child, start) in running {
            match child.try_wait().map_err(|e| format!("{}: {e}", job.lang))? {
                None => still.push((job, child, start)),
                Some(status) => {
                    let secs = start.elapsed().as_secs_f64();
                    if status.success() {
                        eprintln!("{}: done in {secs:.0}s", job.lang);
                    } else {
                        eprintln!("{}: failed ({status}), see {}", job.lang, job.log.display());
                        failed.push(job.lang);
                    }
                    done(job.lang, status.success());
                }
            }
        }
        running = still;
    }
}

/// Start `job`: this very binary (the file it was started from, even if a
/// build has replaced it since), behind `wrap`.
fn spawn(job: &Job, wrap: Option<&str>) -> Result<Child, String> {
    let me = PathBuf::from(format!("/proc/{}/exe", std::process::id()));
    let me = if me.exists() {
        me
    } else {
        std::env::current_exe().map_err(|e| e.to_string())?
    };
    let mut wrapper = wrap.into_iter().flat_map(str::split_whitespace);
    let mut command = match wrapper.next() {
        Some(program) => {
            let mut c = Command::new(program);
            c.args(wrapper).arg(&me);
            c
        }
        None => Command::new(&me),
    };
    if let Some(dir) = job.log.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let log = std::fs::File::create(&job.log).map_err(|e| format!("{}: {e}", job.log.display()))?;
    let err = log.try_clone().map_err(|e| e.to_string())?;
    command
        .args(&job.args)
        .stdin(Stdio::null())
        .stdout(log)
        .stderr(err)
        .spawn()
        .map_err(|e| format!("{}: {e}", job.lang))
}
