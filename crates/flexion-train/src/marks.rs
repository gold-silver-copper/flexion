//! Spelling tables derived from Unicode decomposition. A config names
//! combining marks (the acute that writes stress, the breathings lookup
//! ignores, the length marks a setting drops); this module finds every
//! precomposed character carrying them, so the runtime needs no Unicode
//! tables of its own.

use crate::spec::Spelling;
use unicode_normalization::UnicodeNormalization;

/// Tables for [`flexion::tables::SpellingSpec`].
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Derived {
    /// Triples: character, plain letter, kind digit.
    pub stress_table: String,
    /// Vowels, with every letter that is a vowel plus folded or dropped marks.
    pub vowels: String,
    /// `(letter, folded)` pairs; `'\0'`: left out of lookup keys.
    pub fold: Vec<(char, char)>,
    /// `(letter, plain)` pairs; `'\0'`: not written.
    pub unmark: Vec<(char, char)>,
    /// Letters written with the hiatus mark.
    pub hiatus: String,
}

/// Characters scanned: Latin, Greek, Cyrillic, Indic and Latin/Greek Extended.
const SCAN: std::ops::RangeInclusive<u32> = 0xc0..=0x1fff;

fn compose(chars: &[char]) -> Option<char> {
    let s: String = chars.iter().collect::<String>().nfc().collect();
    let mut it = s.chars();
    match (it.next(), it.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

pub fn derive(sp: &Spelling) -> Derived {
    let stress: Vec<char> = sp.stress_marks.chars().collect();
    let kind_of = |m: char| -> Option<usize> {
        let m = sp
            .stress_aliases
            .iter()
            .find(|a| a.0 == m)
            .map_or(m, |a| a.1);
        stress.iter().position(|&s| s == m)
    };
    let mut out = Derived {
        vowels: sp.vowels.to_string(),
        fold: sp.fold.clone(),
        unmark: sp.unmark.clone(),
        ..Derived::default()
    };
    for &(alias, target) in &sp.stress_aliases {
        if let Some(k) = kind_of(target) {
            push_alias(&mut out.stress_table, alias, '\0', k);
        }
    }
    for m in sp.fold_marks.chars() {
        out.fold.push((m, '\0'));
    }
    for m in sp.unmark_marks.chars() {
        out.unmark.push((m, '\0'));
    }
    let is_vowel = |c: char| sp.vowels.contains(c);
    let known: String = [sp.fold_marks, sp.unmark_marks, sp.stress_marks]
        .concat()
        .chars()
        .chain(sp.stress_aliases.iter().map(|a| a.0))
        .collect();
    for u in SCAN {
        let Some(c) = char::from_u32(u) else {
            continue;
        };
        let d: Vec<char> = c.nfd().collect();
        let Some((&first, marks)) = d.split_first() else {
            continue;
        };
        if let Some(h) = sp.hiatus_mark
            && marks.contains(&h)
            && is_vowel(first)
        {
            out.hiatus.push(c);
        }
        if marks.is_empty() || !is_vowel(first) || !marks.iter().all(|m| known.contains(*m)) {
            continue;
        }
        let without = |drop: &dyn Fn(char) -> bool| -> Option<char> {
            let kept: Vec<char> = std::iter::once(first)
                .chain(marks.iter().copied().filter(|&m| !drop(m)))
                .collect();
            compose(&kept)
        };
        let stressed = marks.iter().find_map(|&m| kind_of(m));
        match stressed {
            Some(k) => {
                if !stress.is_empty()
                    && let Some(plain) = without(&|m| kind_of(m).is_some())
                {
                    // Written only if it is how the letter and the kind's own
                    // mark compose (not a grave read as an acute, not a
                    // compatibility duplicate).
                    let canonical = stress
                        .get(k)
                        .and_then(|&m| compose(&[plain, m]))
                        .is_some_and(|x| x == c);
                    if canonical {
                        push_stress(&mut out.stress_table, c, plain, k);
                    } else {
                        push_alias(&mut out.stress_table, c, plain, k);
                    }
                }
            }
            None => {
                if !out.vowels.contains(c) {
                    out.vowels.push(c);
                }
                if marks.iter().any(|m| sp.fold_marks.contains(*m))
                    && let Some(f) =
                        without(&|m| sp.fold_marks.contains(m) || sp.unmark_marks.contains(m))
                {
                    out.fold.push((c, f));
                }
                if marks.iter().any(|m| sp.unmark_marks.contains(*m))
                    && let Some(f) = without(&|m| sp.unmark_marks.contains(m))
                {
                    out.unmark.push((c, f));
                }
            }
        }
    }
    // Marks a caller may leave out are ignored for lookup too.
    for &(c, plain) in &out.unmark.clone() {
        if !out.fold.iter().any(|f| f.0 == c) {
            let folded = out
                .fold
                .iter()
                .find(|f| f.0 == plain)
                .map_or(plain, |f| f.1);
            out.fold.push((c, folded));
        }
    }
    out
}

/// A character read as stress of `kind` but never written (kinds as letters
/// `a`, `b`, …).
fn push_alias(table: &mut String, c: char, plain: char, kind: usize) {
    table.push(c);
    table.push(plain);
    table.push(
        char::from_u32(u32::from('a').saturating_add(u32::try_from(kind).unwrap_or(0)))
            .unwrap_or('a'),
    );
}

fn push_stress(table: &mut String, c: char, plain: char, kind: usize) {
    table.push(c);
    table.push(plain);
    table.push(char::from_digit(u32::try_from(kind).unwrap_or(0), 10).unwrap_or('0'));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_greek_tables() {
        let sp = Spelling {
            stress_marks: "\u{301}\u{342}",
            stress_aliases: vec![('\u{300}', '\u{301}')],
            vowels: "αεηιουω",
            fold_marks: "\u{313}\u{314}\u{345}\u{308}",
            unmark_marks: "\u{304}\u{306}",
            ..Spelling::default()
        };
        let d = derive(&sp);
        let triples: Vec<(char, char, char)> = {
            let c: Vec<char> = d.stress_table.chars().collect();
            c.chunks(3).map(|t| (t[0], t[1], t[2])).collect()
        };
        assert!(triples.contains(&('ἄ', 'ἀ', '0')));
        assert!(triples.contains(&('ῶ', 'ω', '1')));
        assert!(triples.contains(&('ὰ', 'α', 'a')));
        assert!(d.fold.contains(&('ἀ', 'α')));
        assert!(d.fold.contains(&('ᾳ', 'α')));
        assert!(d.fold.contains(&('ᾰ', 'α')));
        assert!(d.unmark.contains(&('ῡ', 'υ')));
        assert!(d.vowels.contains('ἀ'));
    }
}
