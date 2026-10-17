//! Which declension a word takes. The rule with the longest ending the word has
//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each
//! declension with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is
//! kept only when it pays for itself: it must describe the words it covers
//! in fewer bits than the broader rule does, by more than the rule costs.

use super::paradigms::Declension::{self, *};
use flexion::Choice;

/// 35 rules and 49 listed words.
pub(super) const DECLENSION: Choice<'static, Declension> = flexion::choice! {
    rules {
        "" => [Rēctē 293, Singulāriter 136, Continenter 77, Ōcius 22, Sērō 17, Nōminātim 13, Dītissimē 13, Paene 12, Variē 10, Ūber 8, Extrā 4, Integrē 4, Ūtile 3],
        "a" => [Extrā 4] regular,
        "ra" => [Extrā 4] regular,
        "e" => [Rēctē 293, Dītissimē 13, Paene 12, Variē 10, Integrē 4, Ūtile 3] regular,
        "ice" => [Rēctē 18] regular,
        "ide" => [Rēctē 18] regular,
        "ie" => [Variē 9, Rēctē 1] regular,
        "rie" => [Variē 4] regular,
        "le" => [Rēctē 12, Ūtile 3, Paene 1] regular,
        "ile" => [Ūtile 3] regular,
        "ule" => [Rēctē 7] regular,
        "me" => [Dītissimē 13, Rēctē 4] regular,
        "sime" => [Dītissimē 13, Rēctē 1] regular,
        "ne" => [Rēctē 27, Paene 3] regular,
        "ene" => [Rēctē 3, Paene 3] regular,
        "pe" => [Paene 3, Rēctē 1] regular,
        "re" => [Rēctē 26, Integrē 4, Paene 1] regular,
        "are" => [Rēctē 8] regular,
        "gre" => [Integrē 2] regular,
        "se" => [Rēctē 47] regular,
        "te" => [Rēctē 90, Paene 1, Variē 1] regular,
        "ue" => [Rēctē 13] regular,
        "ve" => [Rēctē 9] regular,
        "m" => [Nōminātim 13, Ōcius 2] regular,
        "im" => [Nōminātim 13] regular,
        "um" => [Ōcius 2] regular,
        "o" => [Sērō 17] regular,
        "to" => [Sērō 10] regular,
        "r" => [Singulāriter 136, Continenter 77, Ūber 4],
        "per" => [Ūber 2] regular,
        "iter" => [Singulāriter 133] regular,
        "nter" => [Continenter 76] regular,
        "enter" => [Continenter 49] regular,
        "s" => [Ōcius 19, Ūber 2] regular,
        "us" => [Ōcius 19, Ūber 1] regular,
    }
    listed {
        Rēctē => [
            "#aspere", "#honorifice", "#misere", "#parissume", "#parve", "#pie", "#tenuissime",
        ],
        Singulāriter => [
            "#acriter", "#alacriter", "#audacter", "#celeriter", "#difficiliter", "#difficulter",
            "#indifficulter", "#tenuiter",
        ],
        Continenter => [
            "#inerter",
        ],
        Ōcius => [
            "#interius", "#multum", "#parum", "#secus", "#subdiu", "#superius",
        ],
        Sērō => [
            "#derecto", "#merito",
        ],
        Paene => [
            "#ante", "#bene", "#dulce", "#grande", "#lene", "#magnopere", "#male", "#paene",
            "#prae", "#prope", "#saepe", "#sepe",
        ],
        Variē => [
            "#elate",
        ],
        Ūber => [
            "#diu", "#dius", "#satis", "#subter", "#super", "#temperi", "#uber",
        ],
        Extrā => [
            "#supra",
        ],
        Integrē => [
            "#aegre", "#fabre", "#integre", "#pulchre",
        ],
    }
};
