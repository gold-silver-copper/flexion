//! Patterns: how a form is built from a word's stems.
//!
//! A pattern is literal text with a `~` where each piece of the stem goes:
//! `~а` is the stem and `а` (`книга`), `~о~` a stem with an `о` inside it
//! (`лесок`, whose `о` drops out in `~~а`, `леска`), `ge~t` a prefix and an
//! ending (`gesagt`). Every form of a paradigm has the same number of stem
//! pieces, in the same order: they are the letters the word keeps throughout.
//! A cell with more than one form lists them with `|` (`~ой|~ою`); the first
//! is the usual one.

/// The stem pieces of one word, in order.
pub type Stems<'w> = Vec<&'w str>;

/// The pattern's alternatives (`~ой|~ою` → `~ой`, `~ою`), the usual one first.
pub fn variants(pattern: &str) -> impl Iterator<Item = &str> {
    pattern.split('|')
}

/// How many stem pieces a pattern has.
pub fn pieces(pattern: &str) -> usize {
    pattern.matches('~').count()
}

/// Read the stem pieces of `word` off `pattern`: `word` must be the pattern's
/// literal text with a non-empty piece of stem at every `~`. Where a literal
/// could match in more than one place, the rightmost reading wins (a word
/// changes at its end). `None` when the word doesn't fit the pattern.
pub fn bind<'w>(pattern: &str, word: &'w str) -> Option<Stems<'w>> {
    bind_by(pattern, word, |a, b| a == b)
}

/// [`bind`], with `same` saying which letters of the word match which of
/// the pattern (a lookup that reads `ā` as `a`). The stems are the word's
/// own letters.
pub fn bind_by<'w>(
    pattern: &str,
    word: &'w str,
    same: impl Fn(char, char) -> bool + Copy,
) -> Option<Stems<'w>> {
    let literals: Vec<&str> = pattern.split('~').collect();
    let (first, rest) = literals.split_first()?;
    let Some((last, middle)) = rest.split_last() else {
        // No stem piece: the pattern is the word itself.
        let n = prefix(word, first, same)?;
        return (n == word.len()).then(Vec::new);
    };
    let inner = word.get(prefix(word, first, same)?..)?;
    let inner = inner.get(..suffix(inner, last, same)?)?;
    let mut stems = Vec::with_capacity(rest.len());
    place(inner, middle, same, &mut stems)?;
    Some(stems)
}

/// The length in bytes of `text`'s beginning that matches `literal`.
fn prefix(text: &str, literal: &str, same: impl Fn(char, char) -> bool) -> Option<usize> {
    let mut chars = text.char_indices();
    for l in literal.chars() {
        let (_, c) = chars.next()?;
        if !same(c, l) {
            return None;
        }
    }
    Some(chars.next().map_or(text.len(), |(i, _)| i))
}

/// Where `text`'s end that matches `literal` starts.
fn suffix(text: &str, literal: &str, same: impl Fn(char, char) -> bool) -> Option<usize> {
    let mut chars = text.char_indices().rev();
    let mut start = text.len();
    for l in literal.chars().rev() {
        let (i, c) = chars.next()?;
        if !same(c, l) {
            return None;
        }
        start = i;
    }
    Some(start)
}

/// Split `text` into stem pieces around `literals` (each stem piece
/// non-empty), rightmost literal placement first, appending the pieces to
/// `out` in order.
fn place<'w>(
    text: &'w str,
    literals: &[&str],
    same: impl Fn(char, char) -> bool + Copy,
    out: &mut Stems<'w>,
) -> Option<()> {
    let Some((lit, before)) = literals.split_last() else {
        if text.is_empty() {
            return None;
        }
        out.push(text);
        return Some(());
    };
    // Each placement of `lit`, rightmost first: every place it occurs, or
    // (an empty literal between two stem pieces) every letter boundary.
    let places: Vec<(usize, usize)> = text
        .char_indices()
        .filter_map(|(i, _)| {
            let n = prefix(text.get(i..)?, lit, same)?;
            Some((i, i.saturating_add(n)))
        })
        .collect();
    for &(at, end) in places.iter().rev() {
        let (Some(head), Some(after)) = (text.get(..at), text.get(end..)) else {
            continue;
        };
        // The stem piece after the literal is never empty.
        if after.is_empty() {
            continue;
        }
        let mark = out.len();
        if place(head, before, same, out).is_some() {
            out.push(after);
            return Some(());
        }
        out.truncate(mark);
    }
    None
}

/// Where a built form's stem ends and its ending begins, for sound laws to
/// see (`ε+ο` → `+ου`). Never part of a word.
pub const BOUNDARY: char = '+';

/// [`fill`], with [`BOUNDARY`] after the last stem piece (at the start, for
/// a pattern with none).
pub fn fill_marked(pattern: &str, stems: &[&str]) -> String {
    match pattern.rfind('~') {
        Some(at) => {
            let (head, tail) = pattern.split_at(at + 1);
            format!("{}{BOUNDARY}{tail}", fill(head, stems))
        }
        None => format!("{BOUNDARY}{pattern}"),
    }
}

/// Build a form: `pattern` (one alternative) with `stems` at its `~`s.
/// Missing stem pieces leave their place empty.
pub fn fill(pattern: &str, stems: &[&str]) -> String {
    let mut out = String::with_capacity(pattern.len().saturating_add(16));
    let mut stems = stems.iter();
    for (i, lit) in pattern.split('~').enumerate() {
        if i > 0
            && let Some(s) = stems.next()
        {
            out.push_str(s);
        }
        out.push_str(lit);
    }
    out
}

/// Where the ending starts in a form `pattern` builds: the byte after the
/// last stem piece (the start of the form when it has none).
pub fn ending_start(pattern: &str, stems: &[&str]) -> usize {
    let Some((head, _)) = pattern.rsplit_once('~') else {
        return 0;
    };
    let mut at = 0usize;
    let mut stems = stems.iter();
    for (i, lit) in head.split('~').enumerate() {
        if i > 0
            && let Some(s) = stems.next()
        {
            at = at.saturating_add(s.len());
        }
        at = at.saturating_add(lit.len());
    }
    // The last stem piece.
    stems.next().map_or(at, |s| at.saturating_add(s.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binds_and_fills() {
        assert_eq!(bind("~а", "книга"), Some(vec!["книг"]));
        assert_eq!(fill("~и", &["книг"]), "книги");
        // A fleeting vowel: the rightmost о is the one inside the stem.
        assert_eq!(bind("~о~", "молоток"), Some(vec!["молот", "к"]));
        assert_eq!(fill("~~а", &["молот", "к"]), "молотка");
        assert_eq!(bind("ge~t", "gesagt"), Some(vec!["sag"]));
        assert_eq!(bind("~а", "рука"), Some(vec!["рук"]));
        // Stem pieces are never empty; the word must fit.
        assert_eq!(bind("~а", "а"), None);
        assert_eq!(bind("~ость", "книга"), None);
        assert_eq!(bind("путь", "путь"), Some(vec![]));
        assert_eq!(ending_start("~~ам", &["молот", "к"]), "молотк".len());
        assert_eq!(ending_start("ge~t", &["sag"]), "gesag".len());
        assert_eq!(variants("~ой|~ою").collect::<Vec<_>>(), vec!["~ой", "~ою"]);
        // Matching letters a lookup reads alike, the stems are the word's.
        let plain = |a: char, b: char| a == b || (a, b) == ('o', 'ō') || (a, b) == ('ō', 'o');
        assert_eq!(bind_by("~tiō", "ratio", plain), Some(vec!["ra"]));
        assert_eq!(bind_by("~x", "fōx", plain), Some(vec!["fō"]));
    }
}
