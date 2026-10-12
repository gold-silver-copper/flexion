//! Learning a part of speech built on principal parts (see
//! [`flexion::Systems`]): which part's system each cell belongs to, each
//! system's grammar (inflecting its cells from its part), and each part's
//! grammar (giving the part of a lemma).

use crate::HashMap;
use crate::choose::Settings;
use crate::extract::Lexeme;
use crate::learn::{self, Learned, Shape};
use crate::skeleton;
use flexion::Orthography;

/// A learned part of speech built on principal parts.
#[derive(Debug, Clone)]
pub struct LearnedSystems<'s> {
    pub cells: usize,
    /// See [`flexion::Systems::sharpness`].
    pub sharpness: f32,
    /// Per principal part, the lemma's first.
    pub systems: Vec<LearnedSystem<'s>>,
}

/// One principal part's system.
#[derive(Debug, Clone)]
pub struct LearnedSystem<'s> {
    /// The cell the part is; `None` for the lemma's.
    pub part: Option<usize>,
    /// Its cells, in the part of speech's numbering.
    pub cells: Vec<usize>,
    /// Gives the part of a lemma.
    pub from_lemma: Option<Learned<'s>>,
    /// Inflects the cells from the part.
    pub grammar: Learned<'s>,
}

/// The fewest words with both forms that say which part a cell is built on;
/// with fewer, it is the lemma's.
const MIN_PAIRS: usize = 20;

impl LearnedSystems<'_> {
    /// Run `f` with the runtime systems these grammars make.
    pub fn with_runtime<R>(&self, f: impl FnOnce(&flexion::Systems<'_, u8>) -> R) -> R {
        let cells: Vec<Vec<u16>> = self
            .systems
            .iter()
            .map(|s| {
                s.cells
                    .iter()
                    .filter_map(|&c| u16::try_from(c).ok())
                    .collect()
            })
            .collect();
        let parts: Vec<flexion::System<'_, u8>> = self
            .systems
            .iter()
            .zip(&cells)
            .map(|(s, cells)| flexion::System {
                name: "",
                part: s.part.and_then(|c| u16::try_from(c).ok()),
                cells,
                from_lemma: s
                    .from_lemma
                    .as_ref()
                    .map(|l| &l.grammar as &(dyn flexion::Inflect<u8> + Sync)),
                grammar: &s.grammar.grammar,
            })
            .collect();
        f(&flexion::Systems {
            cells: self.cells,
            parts: &parts,
            sharpness: self.sharpness,
        })
    }
}

/// Learn `lexemes` (of a part of speech of `shape`) as systems of the
/// principal parts `parts` (their cells).
pub fn learn<'s>(
    orthography: Orthography<'s>,
    shape: &Shape,
    parts: &[usize],
    lexemes: &[Lexeme],
    settings: Settings,
) -> LearnedSystems<'s> {
    let read = |f: &str| orthography.read(f).letters;
    let form = |l: &Lexeme, c: usize| l.cells.get(c).and_then(|fs| fs.first()).map(|f| read(f));
    // Each cell's system: the part its form is told from most regularly,
    // the lemma's unless enough words say otherwise.
    let system_of: Vec<usize> = (0..shape.cells)
        .map(|c| {
            if let Some(k) = parts.iter().position(|&p| p == c) {
                return k + 1;
            }
            let concentration = |k: usize| -> f64 {
                let mut changes: HashMap<(String, String), usize> = HashMap::default();
                let mut n = 0usize;
                for l in lexemes {
                    let source = match k {
                        0 => Some(read(&l.full)),
                        _ => parts.get(k - 1).and_then(|&p| form(l, p)),
                    };
                    let (Some(source), Some(target)) = (source, form(l, c)) else {
                        continue;
                    };
                    let a = skeleton::analyze(&source, &[target.as_str()]);
                    let target = a.forms.into_iter().next().flatten().unwrap_or_default();
                    *changes.entry((a.lemma, target)).or_default() += 1;
                    n += 1;
                }
                if n < MIN_PAIRS {
                    return 0.0;
                }
                #[allow(clippy::cast_precision_loss)]
                let share = changes.values().map(|&x| (x * x) as f64).sum::<f64>() / (n * n) as f64;
                share
            };
            let scores: Vec<(usize, f64)> =
                (0..=parts.len()).map(|k| (k, concentration(k))).collect();
            if std::env::var_os("FLEXION_DEBUG").is_some() {
                eprintln!("cell {c}: {scores:?}");
            }
            scores
                .into_iter()
                .fold(
                    (0, -1.0),
                    |best, (k, x)| if x > best.1 { (k, x) } else { best },
                )
                .0
        })
        .collect();
    let systems = (0..=parts.len())
        .filter_map(|k| {
            let cells: Vec<usize> = (0..shape.cells)
                .filter(|&c| system_of.get(c) == Some(&k))
                .collect();
            let part = k.checked_sub(1).and_then(|i| parts.get(i).copied());
            if cells.is_empty() {
                return None;
            }
            let local = |c: usize| cells.iter().position(|&x| x == c);
            let sub_shape = Shape {
                cells: cells.len(),
                lemma_cell: match part {
                    Some(p) => local(p),
                    None => shape.lemma_cell.and_then(local),
                },
                fallback: cells
                    .iter()
                    .map(|&c| shape.fallback.get(c).copied().flatten().and_then(local))
                    .collect(),
                capitalized: shape.capitalized,
                inherent: shape.inherent.clone(),
            };
            let restrict = |l: &Lexeme| -> Vec<Vec<String>> {
                cells
                    .iter()
                    .map(|&c| l.cells.get(c).cloned().unwrap_or_default())
                    .collect()
            };
            let (grammar, from_lemma) = match part {
                None => {
                    let sub: Vec<Lexeme> = lexemes
                        .iter()
                        .map(|l| Lexeme {
                            cells: restrict(l),
                            ..l.clone()
                        })
                        .collect();
                    (learn::learn(orthography, &sub_shape, &sub, settings), None)
                }
                Some(p) => {
                    // The system, from each word's part.
                    let sub: Vec<Lexeme> = lexemes
                        .iter()
                        .filter_map(|l| {
                            let full = l.cells.get(p)?.first()?.clone();
                            let key = orthography.key(&read(&full));
                            Some(Lexeme {
                                lemma: full.clone(),
                                full,
                                key,
                                cells: restrict(l),
                                ..l.clone()
                            })
                        })
                        .collect();
                    // The part, from each word's lemma.
                    let parts_of: Vec<Lexeme> = lexemes
                        .iter()
                        .filter(|l| l.cells.get(p).is_some_and(|f| !f.is_empty()))
                        .map(|l| Lexeme {
                            cells: vec![l.cells.get(p).cloned().unwrap_or_default()],
                            ..l.clone()
                        })
                        .collect();
                    let one = Shape {
                        cells: 1,
                        lemma_cell: None,
                        fallback: vec![None],
                        capitalized: shape.capitalized,
                        inherent: shape.inherent.clone(),
                    };
                    (
                        learn::learn(orthography, &sub_shape, &sub, settings),
                        Some(learn::learn(orthography, &one, &parts_of, settings)),
                    )
                }
            };
            Some(LearnedSystem {
                part,
                cells,
                from_lemma,
                grammar,
            })
        })
        .collect();
    LearnedSystems {
        cells: shape.cells,
        sharpness: 1.0,
        systems,
    }
}
