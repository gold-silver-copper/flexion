//! Patterns: how a form is built from a word's stems.
//!
//! A pattern is literal text with a `~` where each piece of the stem goes:
//! `~а` is the stem and `а` (`книга`), `~о~` a stem with an `о` inside it
//! (`лесок`, whose `о` drops out in `~~а`, `леска`), `ge~t` a prefix and an
//! ending (`gesagt`). Every form of a paradigm has the same number of stem
//! pieces, in the same order: they are the letters the word keeps throughout.
//! A cell with more than one form lists them with `|` (`~ой|~ою`); the first
//! is the usual one. A [`COPY`] in a pattern is the first letter of the stem
//! piece after it: reduplication (`%ε~κα` is `λέλυκα` of `λυ`, `πεπαίδευκα`
//! of `παιδευ`).

/// The stem pieces of one word, in order.
pub type Stems<'w> = Vec<&'w str>;

/// In a pattern, the first letter of the stem piece after it (see the module
/// documentation).
pub const COPY: char = '%';

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
    let pattern = pattern.strip_suffix(ALTERNATES).unwrap_or(pattern);
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
    // A copied letter must be the one its stem piece begins with.
    if pattern.contains(COPY) {
        let built = fill(pattern, &stems);
        let agree = built.chars().count() == word.chars().count()
            && built.chars().zip(word.chars()).all(|(b, w)| same(w, b));
        return agree.then_some(stems);
    }
    Some(stems)
}

/// The length in bytes of `text`'s beginning that matches `literal`.
fn prefix(text: &str, literal: &str, same: impl Fn(char, char) -> bool) -> Option<usize> {
    let mut chars = text.char_indices();
    for l in literal.chars() {
        let (_, c) = chars.next()?;
        if l != COPY && !same(c, l) {
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
        if l != COPY && !same(c, l) {
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

/// Before a letter a [`COPY`] wrote, in a built form, for sound laws to see
/// (`°φεφιλ+ηκα` → `πεφιλ+ηκα`: the copy of an aspirate is plain). Never
/// part of a word.
pub const COPIED: char = '°';

/// Where a pattern's first literal meets its first stem piece, in a built
/// form, for sound laws to see (`ἐ‹ἀγ+ον` → `ἠγ+ον`: the augment lengthens
/// the stem's vowel). Never part of a word.
pub const FRONT: char = '‹';

/// At the end of a pattern, and of the form it builds: the paradigm's stems
/// alternate, as the sound laws that hold only for such stems change them
/// (Finnish consonant gradation: `katu`, `kadun`; a rule feature of the
/// paradigm, Chomsky & Halle 1968 ch. 8). Never part of a word.
pub const ALTERNATES: char = 'ˇ';

/// After letters a law for alternating stems has written: no other such
/// law changes them again (they apply at once: Finnish `voitto`, `voiton`,
/// is not `voidon`). Never part of a word.
pub const CHANGED: char = '¦';

/// Every mark of a built form (see [`fill_marked`]), none of them a letter
/// of it. Sound laws read them in this order (`[+|ˇ|¦|°|‹]`).
pub const MARKS: [char; 5] = [BOUNDARY, ALTERNATES, CHANGED, COPIED, FRONT];

/// Whether `c` marks a built form (one of [`MARKS`]) rather than being a
/// letter of it.
pub fn is_mark(c: char) -> bool {
    MARKS.contains(&c)
}

/// [`fill`], with [`BOUNDARY`] after the last stem piece (at the start, for
/// a pattern with none), [`COPIED`] before each copied letter, and [`FRONT`]
/// before the first stem piece where a literal comes before it.
pub fn fill_marked(pattern: &str, stems: &[&str]) -> String {
    match pattern.rfind('~') {
        Some(at) => {
            let (head, tail) = pattern.split_at(at + 1);
            format!("{}{BOUNDARY}{tail}", build(head, stems, true))
        }
        None => format!("{BOUNDARY}{pattern}"),
    }
}

/// Build a form: `pattern` (one alternative) with `stems` at its `~`s, and
/// the first letter of the stem piece after each [`COPY`]. A pattern with
/// more `~`s than stem pieces takes them again from the first: a form
/// written twice (Turkish `gele gele` of `gele`, by `~ ~`).
pub fn fill(pattern: &str, stems: &[&str]) -> String {
    build(pattern, stems, false)
}

/// [`fill`], with [`COPIED`] before each copied letter and [`FRONT`] after
/// a first literal if `marked` (a pattern may write it: a relation's change
/// said through the laws where a literal meets the stem).
fn build(pattern: &str, stems: &[&str], marked: bool) -> String {
    let mut out = String::with_capacity(pattern.len().saturating_add(16));
    let piece = |i: usize| {
        stems
            .get(i)
            .or_else(|| stems.get(i.checked_rem(stems.len())?))
    };
    for (i, lit) in pattern.split('~').enumerate() {
        if let Some(s) = i.checked_sub(1).and_then(piece) {
            if marked && i == 1 && !out.is_empty() && !out.ends_with(FRONT) {
                out.push(FRONT);
            }
            out.push_str(s);
        }
        let next = piece(i).and_then(|s| s.chars().next());
        for c in lit.chars() {
            match (c, next) {
                (COPY, Some(n)) => {
                    if marked {
                        out.push(COPIED);
                    }
                    out.push(n);
                }
                (COPY, None) => {}
                (ALTERNATES | FRONT, _) if !marked => {}
                _ => out.push(c),
            }
        }
    }
    out
}

/// A form built marked (see [`fill_marked`]) as a word: its marks taken
/// out, and where its ending starts.
pub fn unmarked(built: &str) -> (String, usize) {
    // Every mark but the boundary, which tells where the ending starts.
    let word: String = built
        .chars()
        .filter(|&c| !is_mark(c) || c == BOUNDARY)
        .collect();
    let ending = word.find(BOUNDARY).unwrap_or(0);
    (word.replace(BOUNDARY, ""), ending)
}

/// Where the ending starts in a form `pattern` builds: the byte after the
/// last stem piece (the start of the form when it has none).
pub fn ending_start(pattern: &str, stems: &[&str]) -> usize {
    let head = pattern.rfind('~').and_then(|at| pattern.get(..=at));
    head.map_or(0, |head| fill(head, stems).len())
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
        // Reduplication copies the stem's first letter.
        assert_eq!(fill("%ε~κα", &["λυ"]), "λελυκα");
        assert_eq!(fill("ἐ%ε~κη", &["λυ"]), "ἐλελυκη");
        assert_eq!(bind("%ε~κα", "πεπαιδευκα"), Some(vec!["παιδευ"]));
        assert_eq!(bind("%ε~κα", "πεπαιδευκ"), None);
        assert_eq!(bind("%ε~κα", "τεπαιδευκα"), None);
        // Marked for sound laws: where the stem ends, and what was copied.
        // and where a literal before the stem meets it.
        assert_eq!(fill_marked("ἐ%ε~κη", &["φιλη"]), "ἐ°φε‹φιλη+κη");
        assert_eq!(
            unmarked("ἐ°φε‹φιλη+κη"),
            ("ἐφεφιληκη".to_string(), "ἐφεφιλη".len())
        );
        assert_eq!(fill_marked("ἐ~ον", &["ἀγ"]), "ἐ‹ἀγ+ον");
        assert_eq!(fill_marked("~ον", &["ἀγ"]), "ἀγ+ον");
        // A paradigm whose stems alternate: marked for the laws only.
        assert_eq!(bind("~ˇ", "veto"), Some(vec!["veto"]));
        assert_eq!(fill("~nˇ", &["veto"]), "veton");
        assert_eq!(fill_marked("~nˇ", &["veto"]), "veto+nˇ");
        assert_eq!(unmarked("vedo+nˇ"), ("vedon".to_string(), "vedo".len()));
        // Matching letters a lookup reads alike, the stems are the word's.
        let plain = |a: char, b: char| a == b || (a, b) == ('o', 'ō') || (a, b) == ('ō', 'o');
        assert_eq!(bind_by("~tiō", "ratio", plain), Some(vec!["ra"]));
        assert_eq!(bind_by("~x", "fōx", plain), Some(vec!["fō"]));
    }
}
