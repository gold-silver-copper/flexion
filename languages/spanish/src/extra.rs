//! Personal pronouns. Wiktionary has no table for them (each pronoun is its
//! own entry, with its other forms as loose head-line words), so this is a
//! small fixed table rather than learned data.

use crate::{Gender, Number, Person};

/// Where a personal pronoun stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PronounCase {
    /// The subject (`yo`, `ella`).
    Subject,
    /// The direct object, before the verb (`me`, `la`).
    DirectObject,
    /// The indirect object, before the verb (`me`, `le`).
    IndirectObject,
    /// After a preposition (`para mí`, `con ella`).
    Prepositional,
    /// The reflexive object (`me lavo`, `se lava`).
    Reflexive,
}

/// A personal pronoun. `gender` matters in the third person and in the
/// plural (`nosotras`); the second person is `tú`/`vosotros` (see
/// [`formal_pronoun`] for `usted`).
///
/// ```
/// use spanish::*;
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, Gender::Masculine, PronounCase::Prepositional), "mí");
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, PronounCase::DirectObject), "la");
/// assert_eq!(personal_pronoun(Person::First, Number::Plural, Gender::Feminine, PronounCase::Subject), "nosotras");
/// ```
pub fn personal_pronoun(
    person: Person,
    number: Number,
    gender: Gender,
    case: PronounCase,
) -> &'static str {
    let feminine = gender == Gender::Feminine;
    // subject, direct object, indirect object, prepositional, reflexive
    let row: [&'static str; 5] = match (person, number) {
        (Person::First, Number::Singular) => ["yo", "me", "me", "mí", "me"],
        (Person::Second, Number::Singular) => ["tú", "te", "te", "ti", "te"],
        (Person::Third, Number::Singular) if feminine => ["ella", "la", "le", "ella", "se"],
        (Person::Third, Number::Singular) => ["él", "lo", "le", "él", "se"],
        (Person::First, Number::Plural) if feminine => {
            ["nosotras", "nos", "nos", "nosotras", "nos"]
        }
        (Person::First, Number::Plural) => ["nosotros", "nos", "nos", "nosotros", "nos"],
        (Person::Second, Number::Plural) if feminine => ["vosotras", "os", "os", "vosotras", "os"],
        (Person::Second, Number::Plural) => ["vosotros", "os", "os", "vosotros", "os"],
        (Person::Third, Number::Plural) if feminine => ["ellas", "las", "les", "ellas", "se"],
        (Person::Third, Number::Plural) => ["ellos", "los", "les", "ellos", "se"],
    };
    pick(row, case)
}

/// The polite `usted` (plural `ustedes`, which is also the everyday plural
/// in Latin America). It takes third-person verb forms.
pub fn formal_pronoun(number: Number, gender: Gender, case: PronounCase) -> &'static str {
    let feminine = gender == Gender::Feminine;
    let row: [&'static str; 5] = match number {
        Number::Singular if feminine => ["usted", "la", "le", "usted", "se"],
        Number::Singular => ["usted", "lo", "le", "usted", "se"],
        Number::Plural if feminine => ["ustedes", "las", "les", "ustedes", "se"],
        Number::Plural => ["ustedes", "los", "les", "ustedes", "se"],
    };
    pick(row, case)
}

fn pick(row: [&'static str; 5], case: PronounCase) -> &'static str {
    let [subject, direct, indirect, prepositional, reflexive] = row;
    match case {
        PronounCase::Subject => subject,
        PronounCase::DirectObject => direct,
        PronounCase::IndirectObject => indirect,
        PronounCase::Prepositional => prepositional,
        PronounCase::Reflexive => reflexive,
    }
}
