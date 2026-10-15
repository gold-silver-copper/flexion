//! The sound-law stage's results kept on disk for experiments (`xtask try`
//! and `sweep`), by a hash of all it reads and of its own source: a rerun
//! that changes only later stages skips it. Off unless [`keep_in`] turned
//! it on, so never for a build.

use crate::Fnv;
use crate::laws::{Change, Law};
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use flexion::{Affix, Paradigm};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Write as _};
use std::hash::Hasher;
use std::path::PathBuf;
use std::sync::OnceLock;

static DIR: OnceLock<PathBuf> = OnceLock::new();

/// Keep the stage's results in `dir` from now on.
pub fn keep_in(dir: PathBuf) {
    let _ = DIR.set(dir);
}

/// The source of the stage and of all it calls: a change to any of it is a
/// new key.
const SOURCE: &[&str] = &[
    include_str!("laws.rs"),
    include_str!("laws/harmony.rs"),
    include_str!("learn.rs"),
    include_str!("bits.rs"),
    include_str!("eval.rs"),
    include_str!("par.rs"),
    include_str!("stage.rs"),
    include_str!("../../flexion/src/orthography.rs"),
    include_str!("../../flexion/src/paradigm.rs"),
    include_str!("../../flexion/src/pattern.rs"),
    include_str!("../../flexion/src/stems.rs"),
    include_str!("../../sound-law/src/lib.rs"),
    include_str!("../../sound-law/src/compile.rs"),
    include_str!("../../sound-law/src/engine.rs"),
];

/// What the stage gave: its laws, the paradigms and the words' assignment
/// after it, and whether it compiled its laws to a program.
#[derive(Serialize, Deserialize)]
struct Stored {
    laws: Vec<Law>,
    paradigms: Vec<Owned>,
    assignment: Vec<u16>,
    compiled: bool,
}

/// A paradigm's fields, owned.
#[derive(Serialize, Deserialize)]
struct Owned {
    name: String,
    words: u32,
    lemma: String,
    parent: Option<u16>,
    inherent: Vec<(u8, u8)>,
    table: String,
    stems: String,
    reads: Option<Vec<u16>>,
    affix: Option<(String, String, String)>,
}

/// The key of the stage's results for `inputs` (their `Debug` text), if
/// the stage is kept at all.
pub(crate) fn key(inputs: &[&dyn Debug]) -> Option<u64> {
    DIR.get()?;
    // From FNV's own offset basis, whatever a test reseeded.
    let mut hash = Hashing(Fnv(0xcbf2_9ce4_8422_2325));
    for source in SOURCE {
        hash.0.write(source.as_bytes());
    }
    for input in inputs {
        write!(hash, "{input:?}\u{1f}").ok()?;
    }
    Some(hash.0.finish())
}

/// A hasher written to as text.
struct Hashing(Fnv);

impl std::fmt::Write for Hashing {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}

fn path(key: u64) -> Option<PathBuf> {
    Some(DIR.get()?.join(format!("laws-{key:016x}.json.gz")))
}

/// The results kept under `key`, with the program their laws compile to.
pub(crate) type Results = (
    Vec<Law>,
    Option<sound_law::Program<'static>>,
    Vec<Paradigm<'static>>,
    Vec<u16>,
);

/// The results kept under `key`, if there are.
pub(crate) fn load(key: u64) -> Option<Results> {
    let file = std::fs::File::open(path(key)?).ok()?;
    let stored: Stored = serde_json::from_reader(GzDecoder::new(file)).ok()?;
    let program = program(&stored.laws, stored.compiled);
    let paradigms = stored.paradigms.into_iter().map(paradigm).collect();
    Some((stored.laws, program, paradigms, stored.assignment))
}

/// Keep the results under `key`: only where their laws compile to the
/// program the stage gave, so that loading them gives the same.
pub(crate) fn save(
    key: u64,
    laws: &[Law],
    compiled: Option<&sound_law::Program<'static>>,
    paradigms: &[Paradigm<'_>],
    assignment: &[u16],
) {
    if program(laws, compiled.is_some()).as_ref() != compiled {
        return;
    }
    let stored = Stored {
        laws: laws.to_vec(),
        paradigms: paradigms.iter().map(owned).collect(),
        assignment: assignment.to_vec(),
        compiled: compiled.is_some(),
    };
    let Some(path) = path(key) else {
        return;
    };
    let partial = path.with_extension("partial");
    let written = std::fs::create_dir_all(path.parent().unwrap_or(&path))
        .and_then(|()| std::fs::File::create(&partial))
        .map_err(|e| e.to_string())
        .and_then(|file| {
            let mut gz = GzEncoder::new(file, Compression::fast());
            serde_json::to_writer(&mut gz, &stored).map_err(|e| e.to_string())?;
            gz.finish().map_err(|e| e.to_string())
        });
    if written.is_ok() {
        let _ = std::fs::rename(&partial, &path);
    }
}

fn program(laws: &[Law], compiled: bool) -> Option<sound_law::Program<'static>> {
    let changes: Vec<Change> = laws.iter().map(|l| l.change.clone()).collect();
    compiled.then(|| crate::laws::compile(&changes)).flatten()
}

fn owned(p: &Paradigm<'_>) -> Owned {
    Owned {
        name: p.name.to_string(),
        words: p.words,
        lemma: p.lemma.to_string(),
        parent: p.parent,
        inherent: p.inherent.to_vec(),
        table: p.table.to_string(),
        stems: p.stems.to_string(),
        reads: p.reads.as_ref().map(|r| r.to_vec()),
        affix: p.affix.as_ref().map(|a| {
            (
                a.pattern.to_string(),
                a.other.to_string(),
                a.letters.to_string(),
            )
        }),
    }
}

fn paradigm(o: Owned) -> Paradigm<'static> {
    Paradigm {
        name: Cow::Owned(o.name),
        words: o.words,
        lemma: Cow::Owned(o.lemma),
        parent: o.parent,
        inherent: Cow::Owned(o.inherent),
        table: Cow::Owned(o.table),
        stems: Cow::Owned(o.stems),
        reads: o.reads.map(Cow::Owned),
        affix: o.affix.map(|(pattern, other, letters)| Affix {
            pattern: Cow::Owned(pattern),
            other: Cow::Owned(other),
            letters: Cow::Owned(letters),
        }),
    }
}
