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
        // [к|г|х]+е → +о: 166 words (близорукий, великий, высокий, широкий), 0 forms wrong.
        law0: "{x:[к|г|х]}+е" => "{x}+о";
        // [к|х|г|ш|ж|щ]+ы → +и: 73 words (большой, баской, морской, городской), 16 forms wrong.
        law1: "{x:[к|х|г|ш|ж|щ]}+ы" => "{x}+и";
    }
};
