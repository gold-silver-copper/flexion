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
    let literals: Vec<&str> = pattern.split('~').collect();
    let (first, rest) = literals.split_first()?;
    let Some((last, middle)) = rest.split_last() else {
        // No stem piece: the pattern is the word itself.
        return (*first == word).then(Vec::new);
    };
    let inner = word.strip_prefix(first)?.strip_suffix(last)?;
    let mut stems = Vec::with_capacity(rest.len());
    place(inner, middle, &mut stems)?;
    Some(stems)
}

/// Split `text` into stem pieces around `literals` (each stem piece
/// non-empty), rightmost literal placement first, appending the pieces to
/// `out` in order.
fn place<'w>(text: &'w str, literals: &[&str], out: &mut Stems<'w>) -> Option<()> {
    let Some((lit, before)) = literals.split_last() else {
        if text.is_empty() {
            return None;
        }
        out.push(text);
        return Some(());
    };
    // Each placement of `lit`, rightmost first: every place it occurs, or
    // (an empty literal between two stem pieces) every letter boundary.
    let places: Vec<usize> = if lit.is_empty() {
        text.char_indices().map(|(i, _)| i).collect()
    } else {
        text.match_indices(lit).map(|(i, _)| i).collect()
    };
    for &at in places.iter().rev() {
        let (Some(head), Some(after)) = (text.get(..at), text.get(at.saturating_add(lit.len())..))
        else {
            continue;
        };
        // The stem piece after the literal is never empty.
        if after.is_empty() {
            continue;
        }
        let mark = out.len();
        if place(head, before, out).is_some() {
            out.push(after);
            return Some(());
        }
        out.truncate(mark);
    }
    None
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
    }
}
