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
        law5 = [("s", "a") => "ś¦", ("s", "e") => "ś¦", ("s", "i") => "ś¦", ("s", "o") => "ś¦", ("s", "u") => "ś¦", ("s", "y") => "ś¦", ("s", "ę") => "ś¦", ("z", "a") => "ź¦", ("z", "e") => "z", ("z", "i") => "z", ("z", "o") => "z", ("z", "u") => "z", ("z", "y") => "z", ("z", "ę") => "z"];
    }
    rules {
        // sz+i → s+i: 798 words (pierwszy, pieszy, lepszy, teraźniejszy), 0 forms wrong.
        law0: "sz+i" => "s+i";
        // c+i → c+y: 620 words (mulczujący, lśniący, zwierzęcy, obcy), 0 forms wrong.
        law1: "c+i" => "c+y";
        // t+i → +ci: 424 words (party, wart, złoty, pity), 0 forms wrong.
        law2: "t+i" => "+ci";
        // on+i → +eni: 440 words (spragniony, zmęczony, ograniczony, uzależniony), 30 forms wrong.
        law3: "on+i" => "+eni";
        // z+i → z+y: 331 words (uroczy, tajemniczy, rolniczy, oszukańczy), 0 forms wrong.
        law4: "z+i" => "z+y";
        // s → ś after [a|e|i|o|u|y|ę]; z → ź after a in alternating stems (ˇ), before +.e..., +.i, .+i: 251 words (prosty, pusty, ciasny, jedenasty), 0 forms wrong.
        law5: "{x:[s|z]}" => "{x|law5[l]}" / "{l:[a|e|i|o|u|y|ę]}" _ "+{![+|ˇ|¦|°|‹]}{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{![+|ˇ|¦|°|‹]}{[i]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![ˇ]*}ˇ$";
        // ł+i → +li: 248 words (ciepły, miły, oziębły, prostopadły), 0 forms wrong.
        law6: "ł+i" => "+li";
        // s+ci → +ści: 9 words (przezroczysty, szósty, czysty, przejrzysty), 0 forms wrong.
        law7: "s+ci" => "+ści";
        // ∅ → z after [d|r] in alternating stems (ˇ), before +e..., +i: 17 words (hardy, młody, twardy, chudy), 0 forms wrong.
        law8: "" => "z¦" / "{[d|r]}" _ "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[i]}{![ˇ]*}ˇ$";
        // r+i → r+zy: 30 words (bury, dobry, chory, stary), 0 forms wrong.
        law9: "r+i" => "r+zy";
        // s+l → +śl: 6 words (dorosły, wzniosły, rosły, wyniosły), 0 forms wrong.
        law10: "s+l" => "+śl";
        // ł+s → +lejs: 16 words (ciepły, okrągły, szczupły, rychły), 0 forms wrong.
        law11: "ł+s" => "+lejs";
        // ch+i → +si: 16 words (cichy, suchy, głuchy, kruchy), 0 forms wrong.
        law12: "ch+i" => "+si";
        // os+l → o+śl: 6 words (dorosły, wzniosły, rosły, wyniosły), 0 forms wrong.
        law13: "os+l" => "o+śl";
        // ż+i → ż+y: 13 words (świeży, boży, hoży, chyży), 0 forms wrong.
        law14: "ż+i" => "ż+y";
        // [t|w|n|d]+m → +ym: 31 words (syt, odkryty, szyty, uszyty), 0 forms wrong.
        law15: "{x:[t|w|n|d]}+m" => "{x}+ym";
        // n+s → +ńs: 6 words (tani, szczwany, cwany, wyrafinowany), 0 forms wrong.
        law16: "n+s" => "+ńs";
    }
};
