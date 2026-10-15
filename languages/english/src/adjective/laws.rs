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
        // y+e → +ie: 1,861 words (craggy, country, early, baby), 30 forms wrong.
        law0: "y+e" => "+ie";
        // e+e → e+: 251 words (free, minute, able, abstruse), 0 forms wrong.
        law1: "e+e" => "e+";
        // e+i → +i: 120 words (smiley, horsey, dykey, limey), 4 forms wrong.
        law2: "e+i" => "+i";
        // b+l → b+b: 5 words (fab, sib, drab, glib), 0 forms wrong.
        law3: "b+l" => "b+b";
    }
};
