//! The sound-law stage's results kept on disk for experiments (`xtask try`
//! and `sweep`), by a hash of all it reads and of the source a build keys
//! (see [`crate::saved`]): a rerun that changes only the printer skips it.
//! Off unless [`keep_in`] turned it on, so never for a build.

use crate::Hashing;
use crate::laws::Law;
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use flexion::Paradigm;
use std::fmt::{Debug, Write as _};
use std::path::PathBuf;
use std::sync::OnceLock;

static DIR: OnceLock<PathBuf> = OnceLock::new();

/// Keep the stage's results in `dir` from now on.
pub fn keep_in(dir: PathBuf) {
    let _ = DIR.set(dir);
}

/// The key of the stage's results for `inputs` (their `Debug` text), if
/// the stage is kept at all.
pub(crate) fn key(inputs: &[&dyn Debug]) -> Option<u64> {
    DIR.get()?;
    let mut hash = Hashing::new();
    // The source of all a build keeps, not only the stage's: what the
    // stage calls (the hasher that sets `HashMap` order, the runtime) is
    // read from it.
    for (path, text) in crate::saved::SOURCE {
        write!(hash, "{path}\u{1f}{text}\u{1f}").ok()?;
    }
    for input in inputs {
        write!(hash, "{input:?}\u{1f}").ok()?;
    }
    Some(hash.finish())
}

fn path(key: u64) -> Option<PathBuf> {
    Some(DIR.get()?.join(format!("laws-{key:016x}.json.gz")))
}

/// What the stage gave: its laws, the program they compile to, the
/// paradigms and the words' assignment after it.
pub(crate) type Results = (
    Vec<Law>,
    Option<sound_law::Program<'static>>,
    Vec<Paradigm<'static>>,
    Vec<u16>,
);

/// The results kept under `key`, if there are.
pub(crate) fn load(key: u64) -> Option<Results> {
    let file = std::fs::File::open(path(key)?).ok()?;
    serde_json::from_reader(GzDecoder::new(file)).ok()
}

/// Keep the results under `key`.
pub(crate) fn save(
    key: u64,
    laws: &[Law],
    program: Option<&sound_law::Program<'static>>,
    paradigms: &[Paradigm<'_>],
    assignment: &[u16],
) {
    let Some(path) = path(key) else {
        return;
    };
    let partial = path.with_extension("partial");
    let written = std::fs::create_dir_all(path.parent().unwrap_or(&path))
        .and_then(|()| std::fs::File::create(&partial))
        .map_err(|e| e.to_string())
        .and_then(|file| {
            let mut gz = GzEncoder::new(file, Compression::fast());
            serde_json::to_writer(&mut gz, &(laws, program, paradigms, assignment))
                .map_err(|e| e.to_string())?;
            gz.finish().map_err(|e| e.to_string())
        });
    if written.is_ok() {
        let _ = std::fs::rename(&partial, &path);
    }
}
