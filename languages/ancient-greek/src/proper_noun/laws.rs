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
        law0 = [("Ε", "α") => "ε", ("Ε", "ε") => "ε", ("Ε", "ο") => "ε", ("Ε", "σ") => "ε", ("Ε", "ἀ") => "ε", ("Ε", "ἑ") => "ε", ("Ε", "ῐ") => "ε", ("Ε", "") => "ῠ"];
    }
    rules {
        // Ε → ε after α, ε, ο, σ, ἀ, ἑ, ῐ; ῠ after none (the nearest of α, ε, ο, σ, ἀ, ἑ, ῐ before it, as written): 140 words (μωϋσης, ἀχαιμενης, αἰγῐᾰλευς, ἀλκᾰμενης), 7 forms wrong.
        law0[iterative]: "{x:[Ε]}" => "{x|law0[t]}" / "{t:[α|ε|ο|σ|ἀ|ἑ|ῐ]?}{g:![α|ε|ο|σ|ἀ|ἑ|ῐ]*}" _ "";
    }
};
