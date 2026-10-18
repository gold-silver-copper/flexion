//! Runs side by side (`--lanes N`), a language or one of its parts each, in
//! a process of its own: this binary again, behind a wrapper command if one is
//! given (`--wrap "systemd-run --user --scope -q -p MemoryMax=20G -p
//! MemorySwapMax=0"`, split at spaces).
//!
//! Every build that learns and every `try` records its wall seconds and the
//! process's peak memory in `target/build-cache/runs.tsv`, one line a run
//! (`when`, what ran, language, part of speech or `all`, seconds, peak MB).
//! The scheduler reads each run's latest record: the longest starts first,
//! and runs start beside each other while their measured peaks fit
//! [`ALL_MB`]. A run with no record, or measured above [`RUN_MB`], runs
//! alone, first.

use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

/// The memory one run may use, in MB: what the wrap above caps it at.
pub const RUN_MB: u64 = 20 * 1024;

/// The memory the runs at once may use together, in MB.
pub const ALL_MB: u64 = 40 * 1024;

/// What the records name a run of a whole language.
pub const ALL: &str = "all";

/// A run's measured cost: wall seconds and the process's peak memory.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cost {
    pub secs: f64,
    pub peak_mb: u64,
}

/// The latest record of each run, by what ran, language and part.
#[derive(Default)]
pub struct Records(BTreeMap<(String, String, String), Cost>);

impl Records {
    /// The records in `file`, later lines over earlier ones; none if there
    /// is no file. Lines that don't parse are skipped.
    pub fn read(file: &Path) -> Records {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        let mut latest = BTreeMap::new();
        for line in text.lines() {
            let f: Vec<&str> = line.split('\t').collect();
            let [_, what, lang, part, secs, peak] = f.as_slice() else {
                continue;
            };
            let (Ok(secs), Ok(peak_mb)) = (secs.parse(), peak.parse()) else {
                continue;
            };
            latest.insert(
                (what.to_string(), lang.to_string(), part.to_string()),
                Cost { secs, peak_mb },
            );
        }
        Records(latest)
    }

    /// The latest cost of `what` on `lang`'s `part` (or [`ALL`]).
    pub fn cost(&self, what: &str, lang: &str, part: &str) -> Option<Cost> {
        self.0
            .get(&(what.to_string(), lang.to_string(), part.to_string()))
            .copied()
    }
}

/// Record that `what` on `lang`'s `part` (or [`ALL`]) took `secs`, with the
/// process's peak memory so far: a line added to `file`. A failure to
/// write is printed, not returned: the run itself succeeded. Gives the
/// cost (none where the peak can't be read, and then nothing is recorded).
pub fn record(file: &Path, what: &str, lang: &str, part: &str, secs: f64) -> Option<Cost> {
    let peak_mb = flexion_train::trace::process_peak_mb()?;
    let when = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let line = format!("{when}\t{what}\t{lang}\t{part}\t{secs:.0}\t{peak_mb}\n");
    let written = file
        .parent()
        .map_or(Ok(()), std::fs::create_dir_all)
        .and_then(|()| {
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(file)
        })
        // One write of one short line, so lines from runs side by side
        // don't interleave.
        .and_then(|mut f| f.write_all(line.as_bytes()));
    if let Err(e) = written {
        eprintln!("{}: {e}", file.display());
    }
    Some(Cost { secs, peak_mb })
}

/// Whether a run must run alone: never measured, or measured above what
/// one run may use.
fn alone(cost: Option<Cost>) -> bool {
    cost.is_none_or(|c| c.peak_mb > RUN_MB)
}

/// The order runs start in: those that run alone first (in the order
/// given), then the longest first.
fn order(costs: &[Option<Cost>]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..costs.len()).collect();
    let key = |i: &usize| {
        let c = costs.get(*i).copied().flatten();
        (!alone(c), c.map_or(0.0, |c| -c.secs))
    };
    order.sort_by(|a, b| {
        let (a, b) = (key(a), key(b));
        a.0.cmp(&b.0).then(a.1.total_cmp(&b.1))
    });
    order
}

/// Whether a run costing `next` may start beside `running`, `lanes` at
/// once: anything starts when nothing runs; else only within the lanes,
/// when neither it nor any running one runs alone, and their peaks fit
/// [`ALL_MB`].
fn admits(running: &[Option<Cost>], next: Option<Cost>, lanes: usize) -> bool {
    if running.is_empty() {
        return true;
    }
    let peaks: u64 = running.iter().flatten().map(|c| c.peak_mb).sum();
    running.len() < lanes
        && !alone(next)
        && !running.iter().any(|&c| alone(c))
        && next.is_some_and(|c| peaks + c.peak_mb <= ALL_MB)
}

/// The schedule [`run`] keeps for runs costing `costs`, `lanes` at once,
/// were each to take its recorded time (a run with none, none): each run's
/// index in `costs`, lane (from 1), start and end, in the order they start.
pub fn simulate(costs: &[Option<Cost>], lanes: usize) -> Vec<(usize, usize, f64, f64)> {
    let mut waiting = order(costs);
    let mut running: Vec<(usize, usize, f64)> = Vec::new();
    let mut plan = Vec::new();
    let mut now = 0.0;
    loop {
        loop {
            let busy: Vec<Option<Cost>> = running
                .iter()
                .map(|r| costs.get(r.0).copied().flatten())
                .collect();
            let Some(next) = waiting
                .iter()
                .position(|&i| admits(&busy, costs.get(i).copied().flatten(), lanes))
            else {
                break;
            };
            let i = waiting.remove(next);
            let lane = (1..)
                .find(|l| running.iter().all(|r| r.1 != *l))
                .unwrap_or(1);
            let end = now + costs.get(i).copied().flatten().map_or(0.0, |c| c.secs);
            running.push((i, lane, end));
            plan.push((i, lane, now, end));
        }
        let Some(first) = running.iter().map(|r| r.2).min_by(f64::total_cmp) else {
            return plan;
        };
        now = first;
        running.retain(|r| r.2 > now);
    }
}

/// One run, of a language's part of speech or of all of it ([`ALL`]): this
/// binary's arguments, and where its output goes.
pub struct Job {
    pub lang: &'static str,
    pub part: &'static str,
    pub args: Vec<String>,
    pub log: PathBuf,
}

/// A run's name: the language, and its part where it is one part's run
/// (`german-noun`).
pub fn name(lang: &str, part: &str) -> String {
    if part == ALL {
        lang.to_string()
    } else {
        format!("{lang}-{part}")
    }
}

impl Job {
    fn name(&self) -> String {
        name(self.lang, self.part)
    }
}

/// Run `jobs`, each `what` on its language's part (or all of it), `lanes`
/// at once, as their latest records in `records` admit them (see the
/// module's comment), each with its output in its log; `done` hears of each
/// as it ends (its [`name`], whether it succeeded). Gives the names of the
/// runs that failed.
pub fn run(
    jobs: Vec<Job>,
    records: &Records,
    what: &str,
    lanes: usize,
    wrap: Option<&str>,
    mut done: impl FnMut(&str, bool),
) -> Result<Vec<String>, String> {
    let costs: Vec<Option<Cost>> = jobs
        .iter()
        .map(|j| records.cost(what, j.lang, j.part))
        .collect();
    let plan = simulate(&costs, lanes);
    let unmeasured: Vec<String> = jobs
        .iter()
        .zip(&costs)
        .filter(|(_, c)| c.is_none())
        .map(|(j, _)| j.name())
        .collect();
    eprintln!(
        "{} runs on {lanes} lanes, about {:.0} minutes by their records{}",
        jobs.len(),
        plan.iter().map(|p| p.3).fold(0.0, f64::max) / 60.0,
        if unmeasured.is_empty() {
            String::new()
        } else {
            format!(", and alone: {} (no record)", unmeasured.join(", "))
        }
    );
    let order = order(&costs);
    let mut slots: Vec<Option<(Job, Option<Cost>)>> =
        jobs.into_iter().zip(costs).map(Some).collect();
    let mut waiting: Vec<(Job, Option<Cost>)> = order
        .into_iter()
        .filter_map(|i| slots.get_mut(i).and_then(Option::take))
        .collect();
    let mut running: Vec<(Job, Option<Cost>, Child, Instant)> = Vec::new();
    let mut failed = Vec::new();
    loop {
        loop {
            let busy: Vec<Option<Cost>> = running.iter().map(|r| r.1).collect();
            let Some(next) = waiting.iter().position(|w| admits(&busy, w.1, lanes)) else {
                break;
            };
            let (job, cost) = waiting.remove(next);
            let child = spawn(&job, wrap)?;
            eprintln!("{}: started, output in {}", job.name(), job.log.display());
            running.push((job, cost, child, Instant::now()));
        }
        if running.is_empty() {
            return Ok(failed);
        }
        std::thread::sleep(Duration::from_secs(1));
        let mut still = Vec::new();
        for (job, cost, mut child, start) in running {
            match child
                .try_wait()
                .map_err(|e| format!("{}: {e}", job.name()))?
            {
                None => still.push((job, cost, child, start)),
                Some(status) => {
                    let secs = start.elapsed().as_secs_f64();
                    let name = job.name();
                    if status.success() {
                        eprintln!("{name}: done in {secs:.0}s");
                    } else {
                        eprintln!("{name}: failed ({status}), see {}", job.log.display());
                        failed.push(name.clone());
                    }
                    done(&name, status.success());
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
        .map_err(|e| format!("{}: {e}", job.name()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cost(secs: f64, gb: u64) -> Option<Cost> {
        Some(Cost {
            secs,
            peak_mb: gb * 1024,
        })
    }

    /// Longest first: 100 on one lane, 60 then 50 on the other, done at
    /// 110 (in the order given, 50 and 60 first, it would end at 150).
    #[test]
    fn longest_first() {
        let costs = [cost(50.0, 1), cost(60.0, 1), cost(100.0, 1)];
        let plan = simulate(&costs, 2);
        let end = plan.iter().map(|p| p.3).fold(0.0, f64::max);
        assert_eq!(end, 110.0);
    }

    /// A run with no record runs alone; runs start together only while
    /// their peaks fit the budget.
    #[test]
    fn admission() {
        assert!(!admits(&[cost(10.0, 1)], None, 3));
        assert!(!admits(&[None], cost(10.0, 1), 3));
        assert!(!admits(&[cost(10.0, 1)], cost(10.0, 21), 3));
        assert!(admits(&[cost(10.0, 16)], cost(10.0, 16), 3));
        assert!(!admits(
            &[cost(10.0, 16), cost(10.0, 16)],
            cost(10.0, 16),
            3
        ));
        assert!(admits(&[], None, 1));
    }
}
