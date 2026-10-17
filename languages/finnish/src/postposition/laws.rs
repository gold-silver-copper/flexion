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
        law0 = [("V", "a") => "a", ("V", "e") => "e", ("V", "i") => "i", ("V", "o") => "o", ("V", "u") => "u", ("V", "y") => "y", ("V", "ä") => "ä", ("V", "ö") => "ö", ("V", "") => "a"];
    }
    rules {
        // V → that letter; a after none (the nearest of a, e, i, o, u, y, ä, ö before it, as written): 32 words (kanssa, myötä, lävitse, kautta), 0 forms wrong.
        law0[iterative]: "{x:[V]}" => "{x|law0[t]}" / "{t:[a|e|i|o|u|y|ä|ö]?}{g:![a|e|i|o|u|y|ä|ö]*}" _ "";
    }
};
