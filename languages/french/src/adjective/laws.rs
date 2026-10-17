//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`), on a letter a `%` copies (`°`), on a class
//! letter an ending writes (harmony, by the nearest letter of a tier before
//! it as written), or inside a stem that alternates (`ˇ`, by what follows
//! it): every form a declension builds goes through them, in order, so one declension
//! serves words whose stems change its endings. Each with the words whose
//! forms it gives, and how many of the dictionary's forms it gets wrong
//! (those are listed in `irregular.rs`). Written in `sound-law`'s notation
//! (`"target" => "replacement" / "before" _ "after"`; `{x|law3}`, a
//! captured letter through that law's map) and compiled when this crate is
//! built.

use flexion::sound_law::Program;

/// The laws, in order.
pub(super) const LAWS: Program<'static> = flexion::laws! {
    classes {}
    rules {
        // [s|x]+s → +: 530 words (gratis, japonais, bourgeois, anglais), 20 forms wrong.
        law0: "{x:[s|x]}+s" => "{x}+";
        // [t|i|u|o]+n → +t: 11 words (sot, chou, coi, favori), 0 forms wrong.
        law1: "{x:[t|i|u|o]}+n" => "{x}+t";
        // et+e → et+te: 40 words (violet, net, cadet, gringalet), 0 forms wrong.
        law2: "et+e" => "et+te";
        // c+e → +que: 10 words (public, turc, ammoniac, caduc), 0 forms wrong.
        law3: "c+e" => "+que";
        // gu+e → gu+ë: 7 words (aigu, ambigu, contigu, subaigu), 0 forms wrong.
        law4: "gu+e" => "gu+ë";
    }
};
