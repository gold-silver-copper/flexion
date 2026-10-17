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
        // e+e → e+: 265 words (orange, safe, lose, woke), 0 forms wrong.
        law0: "e+e" => "e+";
        // [s|nt|ft|ht|st|kt|u|zt|mt|bt|ß|ut|it|id|at|ei|dt|sk|x|z|md|pt]+s → +es: 941 words (bourgeois, mies, lass, divers), 49 forms wrong.
        law1: "{x:[s|nt|ft|ht|st|kt|u|zt|mt|bt|ß|ut|it|id|at|ei|dt|sk|x|z|md|pt]}+s" => "{x}+es";
        // äh+s → ä+chs after a vowel earlier in the stem: 6 words (körpernah, zeitnah, citynah, äquatornah), 2 forms wrong.
        law2: "äh+s" => "ä+chs" / "{[a|e|i|o|u|y|ä|ö|ü]}{![a|e|i|o|u|y|ä|ö|ü]*}" _ "";
        // w+s → w+es: 6 words (new, blaw, blauw, trew), 0 forms wrong.
        law3: "w+s" => "w+es";
        // lt+s → lt+es after [a|h|i|l|o|u|ä|‹]: 20 words (kalt, alt, gewillt, okkult), 0 forms wrong.
        law4: "lt+s" => "lt+es" / "{[a|h|i|l|o|u|ä|‹]}" _ "";
    }
};
