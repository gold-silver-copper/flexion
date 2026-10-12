//! Personal pronouns, participle agreement, and the adjectives that change
//! before a noun.
//!
//! Wiktionary has no table for personal pronouns (each is its own entry,
//! with its other forms as loose head-line words), so they are a small fixed
//! table rather than learned data. A verb's conjugation table gives its past
//! participle only in the masculine singular; [`agreeing_participle`] makes
//! it agree through the learned participle model.

use crate::{Gender, Number, Participle, Person, participle, past_participle};

/// Where a personal pronoun stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PronounCase {
    /// The subject (`io`, `lei`).
    Subject,
    /// The unstressed direct object, before the verb (`mi`, `la`).
    DirectObject,
    /// The unstressed indirect object, before the verb (`mi`, `le`).
    IndirectObject,
    /// The stressed form, after a verb or a preposition (`me`, `con lei`).
    Stressed,
    /// The reflexive object (`mi lavo`, `si lava`).
    Reflexive,
}

/// A personal pronoun. `gender` matters in the third person (`lui`, `lei`;
/// `li`, `le`). `Lei`, the polite `you`, is the third person feminine
/// singular, capitalized.
///
/// ```
/// use italian::*;
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, Gender::Masculine, PronounCase::Stressed), "me");
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, PronounCase::IndirectObject), "le");
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, Gender::Masculine, PronounCase::DirectObject), "li");
/// ```
pub fn personal_pronoun(person: Person, number: Number, gender: Gender, case: PronounCase) -> &'static str {
    let feminine = gender == Gender::Feminine;
    // subject, direct object, indirect object, stressed, reflexive
    let row: [&'static str; 5] = match (person, number) {
        (Person::First, Number::Singular) => ["io", "mi", "mi", "me", "mi"],
        (Person::Second, Number::Singular) => ["tu", "ti", "ti", "te", "ti"],
        (Person::Third, Number::Singular) if feminine => ["lei", "la", "le", "lei", "si"],
        (Person::Third, Number::Singular) => ["lui", "lo", "gli", "lui", "si"],
        (Person::First, Number::Plural) => ["noi", "ci", "ci", "noi", "ci"],
        (Person::Second, Number::Plural) => ["voi", "vi", "vi", "voi", "vi"],
        (Person::Third, Number::Plural) if feminine => ["loro", "le", "gli", "loro", "si"],
        (Person::Third, Number::Plural) => ["loro", "li", "gli", "loro", "si"],
    };
    let [subject, direct, indirect, stressed, reflexive] = row;
    match case {
        PronounCase::Subject => subject,
        PronounCase::DirectObject => direct,
        PronounCase::IndirectObject => indirect,
        PronounCase::Stressed => stressed,
        PronounCase::Reflexive => reflexive,
    }
}

/// A verb's past participle agreeing in gender and number (`prendere` →
/// `prese`): [`participle`] in the masculine singular, then
/// [`past_participle`].
///
/// ```
/// use italian::*;
/// assert_eq!(agreeing_participle("finire", Gender::Feminine, Number::Plural), "finite");
/// ```
pub fn agreeing_participle(verb: &str, gender: Gender, number: Number) -> String {
    past_participle(&participle(verb, Participle::Past), gender, number)
}

/// How the next word begins, for the words whose form depends on it
/// (`bel libro`, `bello specchio`, `bell'albero`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Onset {
    /// A vowel (`albero`, `amica`).
    Vowel,
    /// `s` before a consonant, `z`, `gn`, `ps`, `pn`, `x`, `y`, or `i`
    /// before a vowel (`specchio`, `zaino`, `gnomo`, `psicologo`, `iato`).
    Impure,
    /// Any other consonant (`libro`).
    Plain,
}

fn onset(next: &str) -> Onset {
    let lower = next.to_lowercase();
    let mut chars = lower.chars();
    let first = chars.next();
    let second = chars.next();
    let vowel = |c: char| "aeiouàèéìòóù".contains(c);
    match (first, second) {
        (Some('i' | 'j'), Some(s)) if vowel(s) => Onset::Impure,
        (Some(f), _) if vowel(f) || (f == 'h' && second.is_some_and(vowel)) => Onset::Vowel,
        (Some('s'), Some(s)) if !vowel(s) => Onset::Impure,
        (Some('z' | 'x' | 'y'), _) => Onset::Impure,
        (Some('g'), Some('n')) | (Some('p'), Some('s' | 'n')) => Onset::Impure,
        _ => Onset::Plain,
    }
}

/// The form of `bello`, `quello`, `buono`, `santo` or `grande` right
/// before a noun, which depends on how the noun begins (`bel libro`,
/// `bello specchio`, `bell'albero`, `begli occhi`, `buon amico`, `San
/// Marco`, `gran parte`). Wiktionary lists these forms only for `bello`, in
/// a table it can't read, so they are written out here. `None` for every
/// other adjective, which doesn't change before a noun.
///
/// ```
/// use italian::*;
/// assert_eq!(before_noun("bello", Gender::Masculine, Number::Plural, "occhi").as_deref(), Some("begli"));
/// assert_eq!(before_noun("quello", Gender::Masculine, Number::Singular, "libro").as_deref(), Some("quel"));
/// assert_eq!(before_noun("buono", Gender::Feminine, Number::Singular, "amica").as_deref(), Some("buon'"));
/// assert_eq!(before_noun("rosso", Gender::Masculine, Number::Singular, "libro"), None);
/// ```
pub fn before_noun(adjective: &str, gender: Gender, number: Number, next: &str) -> Option<String> {
    let o = onset(next);
    let masculine = gender == Gender::Masculine;
    let singular = number == Number::Singular;
    let form = match adjective {
        // Like the article (`il`, `lo`, `l'`; `i`, `gli`).
        "bello" | "quello" => {
            let stem = adjective.strip_suffix("llo")?;
            match (masculine, singular, o) {
                (_, true, Onset::Vowel) => format!("{stem}ll'"),
                (true, true, Onset::Plain) => format!("{stem}l"),
                (true, true, Onset::Impure) => format!("{stem}llo"),
                (true, false, Onset::Plain) => format!("{stem}i"),
                (true, false, _) => format!("{stem}gli"),
                (false, true, _) => format!("{stem}lla"),
                (false, false, _) => format!("{stem}lle"),
            }
        }
        // Like the article `un`, `uno`, `un'`.
        "buono" => match (masculine, singular, o) {
            (true, true, Onset::Impure) => "buono".to_string(),
            (true, true, _) => "buon".to_string(),
            (false, true, Onset::Vowel) => "buon'".to_string(),
            (false, true, _) => "buona".to_string(),
            (true, false, _) => "buoni".to_string(),
            (false, false, _) => "buone".to_string(),
        },
        "santo" => match (masculine, singular, o) {
            (_, true, Onset::Vowel) => "sant'".to_string(),
            (true, true, Onset::Plain) => "san".to_string(),
            (true, true, Onset::Impure) => "santo".to_string(),
            (false, true, _) => "santa".to_string(),
            (true, false, _) => "santi".to_string(),
            (false, false, _) => "sante".to_string(),
        },
        "grande" => match (singular, o) {
            (true, Onset::Plain) => "gran".to_string(),
            (true, _) => "grande".to_string(),
            (false, _) => "grandi".to_string(),
        },
        _ => return None,
    };
    Some(form)
}
