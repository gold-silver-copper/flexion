//! One language description per module. A config is everything the trainer
//! knows about a language; adding a language means adding a module here.

pub mod ancient_greek;
pub mod english;
pub mod finnish;
pub mod french;
pub mod german;
pub mod italian;
pub mod latin;
pub mod polish;
pub mod romance;
pub mod russian;
pub mod sanskrit;
pub mod spanish;
pub mod turkish;

use flexion_train::spec::Language;

/// The languages whose crates are built and tested.
pub fn all() -> Vec<Language> {
    vec![
        english::language(),
        russian::language(),
        latin::language(),
        ancient_greek::language(),
        sanskrit::language(),
        polish::language(),
        german::language(),
        spanish::language(),
        french::language(),
        italian::language(),
        finnish::language(),
        turkish::language(),
    ]
}

/// A configured language by crate name.
pub fn by_name(name: &str) -> Option<Language> {
    all().into_iter().find(|l| l.name == name)
}
