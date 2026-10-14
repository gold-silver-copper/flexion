//! Personal pronouns and participle agreement.
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
    /// The subject (`je`, `elle`).
    Subject,
    /// The direct object, before the verb (`me`, `la`).
    DirectObject,
    /// The indirect object, before the verb (`me`, `lui`).
    IndirectObject,
    /// The stressed form, alone or after a preposition (`moi`, `avec elle`).
    Disjunctive,
    /// The reflexive object (`je me lave`, `il se lave`).
    Reflexive,
}

/// A personal pronoun, before elision (`je`, not `j'`). `gender` matters in
/// the third person.
///
/// ```
/// use french::*;
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, Gender::Masculine, PronounCase::Disjunctive), "moi");
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, PronounCase::IndirectObject), "lui");
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, Gender::Masculine, PronounCase::Disjunctive), "eux");
/// ```
pub fn personal_pronoun(
    person: Person,
    number: Number,
    gender: Gender,
    case: PronounCase,
) -> &'static str {
    let feminine = gender == Gender::Feminine;
    // subject, direct object, indirect object, disjunctive, reflexive
    let row: [&'static str; 5] = match (person, number) {
        (Person::First, Number::Singular) => ["je", "me", "me", "moi", "me"],
        (Person::Second, Number::Singular) => ["tu", "te", "te", "toi", "te"],
        (Person::Third, Number::Singular) if feminine => ["elle", "la", "lui", "elle", "se"],
        (Person::Third, Number::Singular) => ["il", "le", "lui", "lui", "se"],
        (Person::First, Number::Plural) => ["nous", "nous", "nous", "nous", "nous"],
        (Person::Second, Number::Plural) => ["vous", "vous", "vous", "vous", "vous"],
        (Person::Third, Number::Plural) if feminine => ["elles", "les", "leur", "elles", "se"],
        (Person::Third, Number::Plural) => ["ils", "les", "leur", "eux", "se"],
    };
    let [subject, direct, indirect, disjunctive, reflexive] = row;
    match case {
        PronounCase::Subject => subject,
        PronounCase::DirectObject => direct,
        PronounCase::IndirectObject => indirect,
        PronounCase::Disjunctive => disjunctive,
        PronounCase::Reflexive => reflexive,
    }
}

/// A verb's past participle agreeing in gender and number (`prendre` →
/// `prises`): [`participle`] in the masculine singular, then
/// [`past_participle`].
///
/// ```
/// use french::*;
/// assert_eq!(agreeing_participle("finir", Gender::Feminine, Number::Singular), "finie");
/// ```
pub fn agreeing_participle(verb: &str, gender: Gender, number: Number) -> String {
    past_participle(&participle(verb, Participle::Past), gender, number)
}
