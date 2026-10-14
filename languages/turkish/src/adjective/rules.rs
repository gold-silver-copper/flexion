//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 10 rules and 27 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Faydalı 26, Sivil 17, Alp 12, Dazlak 5],
        "i" => [Faydalı 8] regular,
        "li" => [Faydalı 7] regular,
        "k" => [Dazlak 5, Faydalı 1, Sivil 1] regular,
        "ak" => [Dazlak 5] regular,
        "l" => [Sivil 8, Alp 4] regular,
        "el" => [Sivil 4] regular,
        "t" => [Sivil 2] regular,
        "y" => [Alp 2] regular,
        "ı" => [Faydalı 8] regular,
    }
    listed {
        "#alp" => Alp,
        "#apocu" => Faydalı,
        "#ayna" => Faydalı,
        "#biricik" => Faydalı,
        "#biyokimyasal" => Alp,
        "#bön" => Alp,
        "#derin" => Sivil,
        "#fino" => Faydalı,
        "#fizikokimyasal" => Alp,
        "#hamile" => Faydalı,
        "#hâkim" => Sivil,
        "#hüzünlü" => Faydalı,
        "#ihtiyar" => Alp,
        "#interseks" => Sivil,
        "#iç" => Sivil,
        "#maço" => Faydalı,
        "#mağmum" => Alp,
        "#müntehap" => Faydalı,
        "#müslüman" => Alp,
        "#otçul" => Alp,
        "#rantabl" => Alp,
        "#sik" => Sivil,
        "#yapma" => Faydalı,
        "#yeğ" => Sivil,
        "#yüzüncü" => Faydalı,
        "#âlem" => Sivil,
        "#öksüz" => Alp,
    }
};
