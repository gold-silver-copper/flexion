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
        let mut stressed = Vec::new();
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
            // A vowel: listed, or a listed vowel with marks the language
            // writes on vowels (Greek breathings and length; stress marks).
            let vowel_marks = |m: &char| {
                s.fold_marks.contains(*m)
                    || s.unmark_marks.contains(*m)
                    || marks.contains(m)
                    || s.stress_aliases.iter().any(|(f, _)| f == m)
            };
            if !s.vowels.contains(c) && !(base_vowels.contains(&base) && rest.iter().all(vowel_marks)) {
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
                    let plain: String = std::iter::once(base)
                        .chain(rest.iter().copied().filter(|m| {
                            !marks.contains(m) && !s.stress_aliases.iter().any(|(f, _)| f == m)
                        }))
                        .collect::<String>()
                        .nfc()
                        .collect();
                    let mut p = plain.chars();
                    if let (Some(plain), None) = (p.next(), p.next()) {
                        stressed.push((c, plain, u8::try_from(kind).unwrap_or(0)));
                    }
                }
                None => vowels.push(c),
            }
        }
        Script {
            lowercase: s.fold_case,
            vowels,
            stress_marks: s.stress_marks.to_string(),
            stressed,
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
        }
    }
}
