//! A language's [`Orthography`], built from its config: every letter that is
//! a vowel, and every precomposed letter that carries a stress mark, found
//! from Unicode.

use crate::spec::Spelling;
use flexion::Orthography;
use unicode_normalization::UnicodeNormalization;

/// The letters an orthography is built from, owned.
#[derive(Debug, Clone)]
pub struct Script {
    pub lowercase: bool,
    pub vowels: String,
    pub stress_marks: String,
    pub stressed: Vec<(char, char, u8)>,
    pub implicit: String,
    pub dropped: String,
    pub mark_monosyllables: bool,
    /// Letters a lookup reads as another, sorted (see
    /// [`flexion::Orthography::folded`]).
    pub folded: Vec<(char, char)>,
    /// The law of limitation's facts, where the language has one.
    pub prosody: Option<Prosody>,
}

/// What [`flexion::Prosody`] needs, owned.
#[derive(Debug, Clone)]
pub struct Prosody {
    pub diphthongs: Vec<&'static str>,
    pub hiatus: String,
    pub long: String,
    pub short_finals: Vec<&'static str>,
    pub acute: u8,
    pub circumflex: u8,
}

/// The Unicode blocks the ten languages write in.
const BLOCKS: &[(u32, u32)] = &[
    (0x0041, 0x024F),
    (0x0370, 0x03FF),
    (0x0400, 0x052F),
    (0x0900, 0x097F),
    (0x1E00, 0x1EFF),
    (0x1F00, 0x1FFF),
];

impl Script {
    pub fn new(s: &Spelling) -> Script {
        let marks: Vec<char> = s.stress_marks.chars().collect();
        let base_vowels: Vec<char> = s.vowels.chars().collect();
        let mut vowels = String::new();
        let mut stressed: Vec<(bool, char, char, u8)> = Vec::new();
        let mut folded: Vec<(char, char)> = s.fold.iter().chain(&s.unmark).copied().collect();
        let mut long = String::new();
        let mut hiatus = String::new();
        for c in BLOCKS
            .iter()
            .flat_map(|&(a, b)| (a..=b).filter_map(char::from_u32))
        {
            if c.is_uppercase() && s.fold_case {
                continue;
            }
            let decomposed: Vec<char> = c.to_string().nfd().collect();
            let Some((&base, rest)) = decomposed.split_first() else {
                continue;
            };
            // A letter whose marks a lookup ignores reads as the letter
            // without them (Greek `ᾰ` as `α`, `ἀ` as `α`).
            let ignored = |m: &char| s.fold_marks.contains(*m) || s.unmark_marks.contains(*m);
            let stress =
                |m: &char| marks.contains(m) || s.stress_aliases.iter().any(|(f, _)| f == m);
            if rest.iter().any(ignored)
                && !rest.iter().any(|m| marks.contains(m))
                && let Some(plain) = compose(base, rest.iter().filter(|m| !ignored(m)))
                && plain != c
                && !folded.iter().any(|e| e.0 == c)
            {
                folded.push((c, plain));
            }
            // A vowel: listed, or a listed vowel with marks the language
            // writes on vowels (Greek breathings and length; stress marks).
            let vowel_marks = |m: &char| ignored(m) || stress(m);
            if !s.vowels.contains(c)
                && !(base_vowels.contains(&base) && rest.iter().all(vowel_marks))
            {
                continue;
            }
            // The stress mark it carries, if any (or one read as one).
            let kind = rest.iter().find_map(|m| {
                let m = s
                    .stress_aliases
                    .iter()
                    .find(|(from, _)| from == m)
                    .map_or(*m, |&(_, to)| to);
                marks.iter().position(|&x| x == m)
            });
            match kind {
                Some(kind) => {
                    let without: Vec<&char> = rest.iter().filter(|m| !stress(m)).collect();
                    // A circumflex stands on a long vowel: `ᾶ` reads `ᾱ`
                    // (where the long letter is one character).
                    let circumflexed = s.circumflex.is_some_and(|m| rest.contains(&m));
                    let lengthened = (circumflexed && !s.long_vowels.contains(base))
                        .then(|| {
                            compose(
                                base,
                                std::iter::once(&'\u{304}').chain(without.iter().copied()),
                            )
                        })
                        .flatten();
                    if let Some(plain) =
                        lengthened.or_else(|| compose(base, without.iter().copied()))
                    {
                        // A letter whose mark is only read as a stress mark (the
                        // grave) is never written: it goes after the others.
                        let aliased = !rest.iter().any(|m| marks.contains(m));
                        stressed.push((aliased, c, plain, u8::try_from(kind).unwrap_or(0)));
                    }
                }
                None => {
                    vowels.push(c);
                    if s.long_vowels.contains(base)
                        || rest.iter().any(|m| s.long_marks.contains(*m))
                    {
                        long.push(c);
                    }
                    if s.hiatus_mark.is_some_and(|h| rest.contains(&h)) {
                        hiatus.push(c);
                    }
                }
            }
        }
        let mark_kind = |m: char| {
            s.stress_marks
                .chars()
                .position(|x| x == m)
                .and_then(|k| u8::try_from(k).ok())
        };
        let prosody = (!s.long_vowels.is_empty()).then(|| Prosody {
            diphthongs: s.diphthongs.clone(),
            hiatus,
            long,
            short_finals: s.short_finals.clone(),
            acute: 0,
            circumflex: s.circumflex.and_then(mark_kind).unwrap_or(0),
        });
        stressed.sort_by_key(|e| e.0);
        folded.sort_unstable();
        folded.dedup_by_key(|e| e.0);
        Script {
            folded,
            prosody,
            lowercase: s.fold_case,
            vowels,
            stress_marks: s.stress_marks.to_string(),
            stressed: stressed.into_iter().map(|(_, c, p, k)| (c, p, k)).collect(),
            implicit: s.implicit.to_string(),
            dropped: s.strip_marks.to_string(),
            mark_monosyllables: s.mark_monosyllables,
        }
    }

    pub fn orthography(&self) -> Orthography<'_> {
        Orthography {
            lowercase: self.lowercase,
            vowels: &self.vowels,
            stress_marks: &self.stress_marks,
            stressed: &self.stressed,
            implicit: &self.implicit,
            dropped: &self.dropped,
            mark_monosyllables: self.mark_monosyllables,
            folded: &self.folded,
            prosody: self.prosody.as_ref().map(|p| flexion::Prosody {
                diphthongs: &p.diphthongs,
                hiatus: &p.hiatus,
                long: &p.long,
                short_finals: &p.short_finals,
                acute: p.acute,
                circumflex: p.circumflex,
            }),
        }
    }
}

/// The one character `base` with `marks` composes to, if it is one.
fn compose<'m>(base: char, marks: impl Iterator<Item = &'m char>) -> Option<char> {
    let composed: String = std::iter::once(base)
        .chain(marks.copied())
        .collect::<String>()
        .nfc()
        .collect();
    let mut chars = composed.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}
