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
        // [ι|ητ|κτ|ικ|πτ|ηρ|ιν|ωτ|ηλ|ργ|υρ|ων|δν|ωρ|δρ|κρ|υλ|λλ|ρτ|σσ|υτ|ωπ|ῡμ|ημ|ην|θρ|ιρ|μν|ντ|πν|τρ|ιμ|γγ|πρ|ρφ|υκ|ωγ|ῡσ|ιτ|λμ|μβ|νδ|χν|ωλ|ᾱτ|ῡλ|ῡχ]+ωτ → +οτ: 719 words (ἀρχαιος, ὁμοιος, ῥωμαιος, βῐαιος), 0 forms wrong.
        law0: "{x:[ι|ητ|κτ|ικ|πτ|ηρ|ιν|ωτ|ηλ|ργ|υρ|ων|δν|ωρ|δρ|κρ|υλ|λλ|ρτ|σσ|υτ|ωπ|ῡμ|ημ|ην|θρ|ιρ|μν|ντ|πν|τρ|ιμ|γγ|πρ|ρφ|υκ|ωγ|ῡσ|ιτ|λμ|μβ|νδ|χν|ωλ|ᾱτ|ῡλ|ῡχ]}+ωτ" => "{x}+οτ";
        // ου+δ → ο+δ: 8 words (τετρᾰπους, ὀκτωπους, τρῐπους, ὀκτᾰπους), 0 forms wrong.
        law1: "ου+δ" => "ο+δ";
        // ου+σ → ο+σ: 8 words (τετρᾰπους, ὀκτωπους, τρῐπους, ὀκτᾰπους), 0 forms wrong.
        law2: "ου+σ" => "ο+σ";
    }
};
