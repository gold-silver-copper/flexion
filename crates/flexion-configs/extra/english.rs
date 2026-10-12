//! Hand-written helpers on top of the learned model: agreement, personal
//! pronouns, possessives and counting. Wiktionary has no pronoun paradigm
//! tables for English and doesn't tag `be`'s person forms as cells, so these
//! are small fixed tables rather than learned data.

use crate::{Number, VerbForm, noun, verb};

/// Grammatical person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Person {
    First,
    Second,
    Third,
}

/// Pronoun gender (third person singular).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

/// Pronoun case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Case {
    /// `I`, `he`, `they`
    Nominative,
    /// `me`, `him`, `them`
    Accusative,
    /// `myself`, `himself`, `themselves`
    Reflexive,
    /// `mine`, `his`, `theirs`
    Possessive,
    /// `my`, `his`, `their`
    PersonalPossessive,
}

/// Simple tenses English marks on the verb itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tense {
    Present,
    Past,
}

/// A finite verb agreeing with its subject: `walk` → `walks` in the third
/// person singular present; `be` → `am`/`are`/`is`/`was`/`were`.
///
/// ```
/// use english::*;
/// assert_eq!(conjugate("be", Tense::Past, Person::Second, Number::Singular), "were");
/// assert_eq!(conjugate("go", Tense::Past, Person::First, Number::Plural), "went");
/// ```
pub fn conjugate(word: &str, tense: Tense, person: Person, number: Number) -> String {
    let lemma = word.split(',').next().unwrap_or(word).trim();
    if lemma.eq_ignore_ascii_case("be") {
        let form = match (tense, person, number) {
            (Tense::Present, Person::First, Number::Singular) => "am",
            (Tense::Present, Person::Third, Number::Singular) => "is",
            (Tense::Present, _, _) => "are",
            (Tense::Past, Person::First | Person::Third, Number::Singular) => "was",
            (Tense::Past, _, _) => "were",
        };
        return match_case(lemma, form);
    }
    match (tense, person, number) {
        (Tense::Present, Person::Third, Number::Singular) => verb(word, VerbForm::ThirdSingular),
        (Tense::Present, _, _) => verb(word, VerbForm::Infinitive),
        (Tense::Past, _, _) => verb(word, VerbForm::Past),
    }
}

fn match_case(model: &str, form: &str) -> String {
    let mut letters = model.chars().filter(|c| c.is_alphabetic());
    match (letters.next(), letters.next()) {
        (Some(a), Some(b)) if a.is_uppercase() && b.is_uppercase() => form.to_uppercase(),
        (Some(a), _) if a.is_uppercase() => capitalize_first(form),
        _ => form.to_string(),
    }
}

/// The personal pronoun for the given features. Gender only matters in the
/// third person singular.
///
/// ```
/// use english::*;
/// assert_eq!(pronoun(Person::First, Number::Singular, Gender::Neuter, Case::Nominative), "I");
/// assert_eq!(pronoun(Person::Third, Number::Plural, Gender::Neuter, Case::Reflexive), "themselves");
/// ```
pub fn pronoun(person: Person, number: Number, gender: Gender, case: Case) -> &'static str {
    use Case::*;
    let row: [&'static str; 5] = match (number, person, gender) {
        (Number::Singular, Person::First, _) => ["I", "me", "myself", "mine", "my"],
        (Number::Singular, Person::Second, _) => ["you", "you", "yourself", "yours", "your"],
        (Number::Singular, Person::Third, Gender::Masculine) => {
            ["he", "him", "himself", "his", "his"]
        }
        (Number::Singular, Person::Third, Gender::Feminine) => {
            ["she", "her", "herself", "hers", "her"]
        }
        (Number::Singular, Person::Third, Gender::Neuter) => ["it", "it", "itself", "its", "its"],
        (Number::Plural, Person::First, _) => ["we", "us", "ourselves", "ours", "our"],
        (Number::Plural, Person::Second, _) => ["you", "you", "yourselves", "yours", "your"],
        (Number::Plural, Person::Third, _) => ["they", "them", "themselves", "theirs", "their"],
    };
    let [nom, acc, refl, poss, det] = row;
    match case {
        Nominative => nom,
        Accusative => acc,
        Reflexive => refl,
        Possessive => poss,
        PersonalPossessive => det,
    }
}

/// `'s`, or a bare `'` after a final `s` (`dog's`, `dogs'`, `James'`).
pub fn add_possessive(word: &str) -> String {
    if word.ends_with('s') {
        format!("{word}'")
    } else {
        format!("{word}'s")
    }
}

/// The first letter uppercased.
pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

/// The noun for a count: singular for exactly 1, plural otherwise.
pub fn count(word: &str, count: u32) -> String {
    noun(
        word,
        if count == 1 {
            Number::Singular
        } else {
            Number::Plural
        },
    )
}

/// `"3 mice"`.
pub fn count_with_number(word: &str, amount: u32) -> String {
    format!("{amount} {}", count(word, amount))
}
