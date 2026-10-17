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
        law0 = [("I", "a") => "ı", ("I", "e") => "i", ("I", "i") => "i", ("I", "o") => "u", ("I", "u") => "u", ("I", "â") => "ı", ("I", "î") => "i", ("I", "ö") => "ü", ("I", "û") => "u", ("I", "ü") => "ü", ("I", "ı") => "ı", ("I", "") => "ı", ("A", "a") => "a", ("A", "e") => "e", ("A", "i") => "e", ("A", "o") => "a", ("A", "u") => "a", ("A", "â") => "a", ("A", "î") => "e", ("A", "ö") => "e", ("A", "û") => "a", ("A", "ü") => "e", ("A", "ı") => "a", ("A", "") => "a"];
        law1 = [("g", "a") => "g", ("g", "e") => "g", ("g", "i") => "g", ("g", "n") => "g", ("g", "o") => "ğ¦", ("g", "r") => "g", ("g", "u") => "g", ("g", "ü") => "g", ("g", "ı") => "g", ("k", "a") => "ğ¦", ("k", "e") => "ğ¦", ("k", "i") => "ğ¦", ("k", "n") => "g¦", ("k", "o") => "k", ("k", "r") => "k", ("k", "u") => "ğ¦", ("k", "ü") => "ğ¦", ("k", "ı") => "ğ¦", ("p", "a") => "b¦", ("p", "e") => "b¦", ("p", "i") => "b¦", ("p", "n") => "p", ("p", "o") => "b¦", ("p", "r") => "p", ("p", "u") => "b¦", ("p", "ü") => "p", ("p", "ı") => "p", ("t", "a") => "d¦", ("t", "e") => "d¦", ("t", "i") => "d¦", ("t", "n") => "d¦", ("t", "o") => "t", ("t", "r") => "d¦", ("t", "u") => "d¦", ("t", "ü") => "d¦", ("t", "ı") => "d¦", ("ç", "a") => "c¦", ("ç", "e") => "c¦", ("ç", "i") => "c¦", ("ç", "n") => "c¦", ("ç", "o") => "ç", ("ç", "r") => "ç", ("ç", "u") => "c¦", ("ç", "ü") => "ç", ("ç", "ı") => "c¦"];
    }
    rules {
        // I → ı after a, â, ı, none; i after e, i, î; u after o, u, û; ü after ö, ü; A → a after a, o, u, â, û, ı, none; e after e, i, î, ö, ü (the nearest of a, e, i, o, u, â, î, ö, û, ü, ı before it, as written): 10,370 words (pound, ab, abdal, abdest), 1,374 forms wrong.
        law0[iterative]: "{x:[I|A]}" => "{x|law0[t]}" / "{t:[a|e|i|o|u|â|î|ö|û|ü|ı]?}{g:![a|e|i|o|u|â|î|ö|û|ü|ı]*}" _ "";
        // k → ğ after [a|e|i|u|ü|ı]; t → d after [a|e|i|n|r|u|ü|ı]; ç → c after [a|e|i|n|u|ı]; p → b after [a|e|i|o|u]; g → ğ after o; k → g after n in alternating stems (ˇ), before +.$, +.[^a|e|i|o|u|â|î|ö|û|ü|ı]: 1,737 words (ant, art, bap, şibidak), 0 forms wrong.
        law1: "{x:[g|k|p|t|ç]}" => "{x|law1[l]}" / "{l:[a|e|i|n|o|r|u|ü|ı]}" _ "+{![+|ˇ|¦|°|‹]}ˇ$" | "+{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|â|î|ö|û|ü|ı]}{![ˇ]*}ˇ$";
        // n+ğ → n+g: 12 words (renk, cenk, pelenk, denk), 0 forms wrong.
        law2: "n+ğ" => "n+g";
        // [ol|ul]+i → +ü: 10 words (alkol, metropol, alveol, megapol), 0 forms wrong.
        law3: "{x:[ol|ul]}+i" => "{x}+ü";
    }
};
