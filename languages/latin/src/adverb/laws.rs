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
    agreements {
        law0 = [("L", "l") => "l", ("L", "r") => "r", ("L", "") => "l"];
    }
    rules {
        // L → that letter; l after none (the letter just before it, if one of l, r): 5 words (facile, ūtile, difficile, miserē), 0 forms wrong.
        law0[iterative]: "{x:[L]}" => "{x|law0[t]}" / "{t:[l|r]?}{g:[+|ˇ|¦|°|‹]*}" _ "";
    }
};
