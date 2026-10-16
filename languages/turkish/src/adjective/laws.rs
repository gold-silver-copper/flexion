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
        law0 = [("I", "a") => "ı", ("I", "e") => "i", ("I", "i") => "i", ("I", "o") => "u", ("I", "ü") => "ü", ("I", "") => "ü", ("E", "a") => "a", ("E", "e") => "e", ("E", "i") => "e", ("E", "o") => "a", ("E", "ü") => "e", ("E", "") => "e"];
    }
    rules {
        // I → ı after a; i after e, i; u after o; ü after ü, none; E → a after a, o; e after e, i, ü, none (the nearest of a, e, i, o, ü before it, as written): 55 words (sik, bay, alp, fino), 0 forms wrong.
        law0[iterative]: "{x:[I|E]}" => "{x|law0[t]}" / "{t:[a|e|i|o|ü]?}{g:![a|e|i|o|ü]*}" _ "";
        // [t|p|k|ç|s]+d → +t: 8 words (sik, alp, iç, çift), 0 forms wrong.
        law1: "{x:[t|p|k|ç|s]}+d" => "{x}+t";
    }
};
