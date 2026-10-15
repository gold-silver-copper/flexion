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
        // न → ण after र, ष, ृ unless च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ल, श, स comes between, not before ्: 1,181 words (वृक्ष, समुद्र, तरंग, तीर), 1 forms wrong.
        law0: "न" => "ण" / "{t:[र|ष|ृ]}{a:![र|ष|ृ|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|ल|श|स]*}+{b:![र|ष|ृ|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|ल|श|स]*}" _ "" unless "" _ "{[्]}";
        // ्म+् → ्म+ after a vowel earlier in the stem: 7 words (अश्मन्, अकर्मन्, आत्मन्, ऊष्मन्), 2 forms wrong.
        law1: "्म+्" => "्म+" / "{[अ|आ|इ|ई|उ|ऊ|ऋ|ए|ऐ|ओ|औ]}{![अ|आ|इ|ई|उ|ऊ|ऋ|ए|ऐ|ओ|औ]*}" _ "";
        // ्म+्न → ्म+न: 13 words (तक्मन्, महात्मन्, वस्मन्, वर्त्मन्), 0 forms wrong.
        law2: "्म+्न" => "्म+न";
        // ्म+्ण → ्म+ण: 17 words (चर्मन्, ब्रह्मन्, कर्मन्, मर्मन्), 0 forms wrong.
        law3: "्म+्ण" => "्म+ण";
    }
};
