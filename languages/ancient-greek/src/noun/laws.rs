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
        // [η|ω]+δ → +τ: 76 words (μῠκης, κρης, σκληροτης, θης), 0 forms wrong.
        law0: "{x:[η|ω]}+δ" => "{x}+τ";
        // [γ|ρῠ]+κ → +γ: 35 words (μηνῐγξ, φᾰλᾰγξ, σπηλῠγξ, λᾰρῠγξ), 12 forms wrong.
        law1: "{x:[γ|ρῠ]}+κ" => "{x}+γ";
        // [ῑ|ῡ|ἰ]+τ → +ν: 10 words (ῥῑς, ἰς, ἀκτῑς, θῑς), 0 forms wrong.
        law2: "{x:[ῑ|ῡ|ἰ]}+τ" => "{x}+ν";
        // [ε|ᾰ]+ν → +ς after a vowel earlier in the stem: 7 words (τερᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς, περᾰς), 0 forms wrong.
        law3: "{x:[ε|ᾰ]}+ν" => "{x}+ς" / "{[α|ε|η|ι|ο|υ|ω|ϊ|ϋ|ἀ|ἁ|ἐ|ἑ|ἠ|ἡ|ἰ|ἱ|ὀ|ὁ|ὐ|ὑ|ὠ|ὡ|ᾀ|ᾠ|ᾰ|ᾱ|ᾳ|ῃ|ῐ|ῑ|ῠ|ῡ|ῳ]}{![α|ε|η|ι|ο|υ|ω|ϊ|ϋ|ἀ|ἁ|ἐ|ἑ|ἠ|ἡ|ἰ|ἱ|ὀ|ὁ|ὐ|ὑ|ὠ|ὡ|ᾀ|ᾠ|ᾰ|ᾱ|ᾳ|ῃ|ῐ|ῑ|ῠ|ῡ|ῳ]*}" _ "";
    }
};
