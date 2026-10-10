//! One language description per module. A config is everything the trainer
//! knows about a language; adding a language means adding a module here.

pub mod english;
pub mod latin;
pub mod polish;
pub mod russian;

use flexion_train::spec::Language;

/// Every configured language, by crate name.
pub fn all() -> Vec<Language> {
    vec![
        english::language(),
        russian::language(),
        latin::language(),
        polish::language(),
    ]
}

pub fn by_name(name: &str) -> Option<Language> {
    all().into_iter().find(|l| l.name == name)
}
