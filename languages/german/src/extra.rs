//! Personal and reflexive pronouns. Wiktionary keeps all personal pronouns in
//! one table whose second-person forms are tagged `error-unrecognized-form`,
//! so this is a small fixed table rather than learned data.

use crate::{Case, Gender, Number, Person};

/// A personal pronoun. `gender` matters only in the third person singular
/// (`er`, `sie`, `es`).
///
/// ```
/// use german::*;
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, Gender::Masculine, Case::Accusative), "mich");
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Dative), "ihr");
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, Gender::Neuter, Case::Dative), "ihnen");
/// ```
pub fn personal_pronoun(
    person: Person,
    number: Number,
    gender: Gender,
    case: Case,
) -> &'static str {
    // nominative, genitive, dative, accusative
    let row: [&'static str; 4] = match (person, number, gender) {
        (Person::First, Number::Singular, _) => ["ich", "meiner", "mir", "mich"],
        (Person::Second, Number::Singular, _) => ["du", "deiner", "dir", "dich"],
        (Person::Third, Number::Singular, Gender::Masculine) => ["er", "seiner", "ihm", "ihn"],
        (Person::Third, Number::Singular, Gender::Feminine) => ["sie", "ihrer", "ihr", "sie"],
        (Person::Third, Number::Singular, Gender::Neuter) => ["es", "seiner", "ihm", "es"],
        (Person::First, Number::Plural, _) => ["wir", "unser", "uns", "uns"],
        (Person::Second, Number::Plural, _) => ["ihr", "euer", "euch", "euch"],
        (Person::Third, Number::Plural, _) => ["sie", "ihrer", "ihnen", "sie"],
    };
    let [nom, gen_, dat, acc] = row;
    match case {
        Case::Nominative => nom,
        Case::Genitive => gen_,
        Case::Dative => dat,
        Case::Accusative => acc,
    }
}

/// The polite `Sie` (singular and plural alike).
pub fn formal_pronoun(case: Case) -> &'static str {
    match case {
        Case::Nominative | Case::Accusative => "Sie",
        Case::Genitive => "Ihrer",
        Case::Dative => "Ihnen",
    }
}

/// The reflexive pronoun (`sich`, `mich`, `uns`), which has only a dative and
/// an accusative; `None` for the other cases.
pub fn reflexive_pronoun(person: Person, number: Number, case: Case) -> Option<&'static str> {
    let (dat, acc) = match (person, number) {
        (Person::First, Number::Singular) => ("mir", "mich"),
        (Person::Second, Number::Singular) => ("dir", "dich"),
        (Person::First, Number::Plural) => ("uns", "uns"),
        (Person::Second, Number::Plural) => ("euch", "euch"),
        (Person::Third, _) => ("sich", "sich"),
    };
    match case {
        Case::Dative => Some(dat),
        Case::Accusative => Some(acc),
        Case::Nominative | Case::Genitive => None,
    }
}
