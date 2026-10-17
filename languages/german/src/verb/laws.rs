//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`), on a letter a `%` copies (`°`), on a class
//! letter an ending writes (harmony, by the nearest letter of a tier before
//! it as written), or inside a stem that alternates (`ˇ`, by what follows
//! it): every form a conjugation builds goes through them, in order, so one conjugation
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
        // [s|z|ß|x]+s → +: 321 words (pflanzen, aasen, pissen, boxen), 26 forms wrong.
        law0: "{x:[s|z|ß|x]}+s" => "{x}+";
        // [t|d]+t → +et: 356 words (listen, bitten, rotten, arbeiten), 256 forms wrong.
        law1: "{x:[t|d]}+t" => "{x}+et";
        // d+s → d+es: 98 words (landen, baden, laden, reden), 2 forms wrong.
        law2: "d+s" => "d+es";
        // [h|i]+f → +: 5 words (weichen, schleichen, streichen, gleichen), 0 forms wrong.
        law3: "{x:[h|i]}+f" => "{x}+";
    }
};
