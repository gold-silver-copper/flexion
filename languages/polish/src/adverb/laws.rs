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
        // o+j → +iej: 55 words (rano, bardzo, parno, zimno), 0 forms wrong.
        law0: "o+j" => "+iej";
        // [i|ż]+i → +: 8 words (bezpośrednio, tanio, głupio, chyżo), 0 forms wrong.
        law1: "{x:[i|ż]}+i" => "{x}+";
        // ł+e → +le: 7 words (ciepło, rychło, miło, mdło), 0 forms wrong.
        law2: "ł+e" => "+le";
        // r+i → r+z: 6 words (skoro, ostro, staro, modro), 0 forms wrong.
        law3: "r+i" => "r+z";
        // g+e → +że: 5 words (drogo, długo, ubogo, srogo), 0 forms wrong.
        law4: "g+e" => "+że";
    }
};
