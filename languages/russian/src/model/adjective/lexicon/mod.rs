//! Lemmas written in full that another part of speech attests, by lookup
//! key (the lemma folded as callers may type it).

mod c0;
mod c1;

pub(crate) fn find(key: &str) -> Option<&'static str> {
    c0::find(key).or_else(|| c1::find(key))
}
