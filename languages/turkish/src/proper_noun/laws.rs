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
        law0 = [("I", "a") => "ı", ("I", "e") => "i", ("I", "i") => "i", ("I", "o") => "u", ("I", "u") => "u", ("I", "â") => "ı", ("I", "ö") => "ü", ("I", "ü") => "ü", ("I", "ı") => "ı", ("I", "") => "ı", ("A", "a") => "a", ("A", "e") => "e", ("A", "i") => "e", ("A", "o") => "a", ("A", "u") => "a", ("A", "â") => "a", ("A", "ö") => "e", ("A", "ü") => "e", ("A", "ı") => "a", ("A", "") => "a"];
        law1 = [("D", "a") => "d", ("D", "e") => "d", ("D", "f") => "t", ("D", "i") => "d", ("D", "k") => "t", ("D", "n") => "d", ("D", "o") => "d", ("D", "p") => "t", ("D", "r") => "d", ("D", "s") => "t", ("D", "t") => "t", ("D", "u") => "d", ("D", "y") => "d", ("D", "ç") => "t", ("D", "ı") => "d", ("D", "ş") => "t", ("D", "") => "d"];
    }
    rules {
        // I → ı after a, â, ı, none; i after e, i; u after o, u; ü after ö, ü; A → a after a, o, u, â, ı, none; e after e, i, ö, ü (the nearest of a, e, i, o, u, â, ö, ü, ı before it, as written): 1,423 words (angola, burundi, laos, nepal), 39 forms wrong.
        law0[iterative]: "{x:[I|A]}" => "{x|law0[t]}" / "{t:[a|e|i|o|u|â|ö|ü|ı]?}{g:![a|e|i|o|u|â|ö|ü|ı]*}" _ "";
        // D → d after a, e, i, n, o, r, u, y, ı, none; t after f, k, p, s, t, ç, ş (the nearest of a, e, f, i, k, n, o, p, r, s, t, u, y, ç, ı, ş before it, as written): 988 words (angola, burundi, laos, nepal), 22 forms wrong.
        law1[iterative]: "{x:[D]}" => "{x|law1[t]}" / "{t:[a|e|f|i|k|n|o|p|r|s|t|u|y|ç|ı|ş]?}{g:![a|e|f|i|k|n|o|p|r|s|t|u|y|ç|ı|ş]*}" _ "";
        // [k|ş|t|s|h]+d → +t: 6 words (kürt, sistemik lupus eritematozus, kerkük, mesih), 0 forms wrong.
        law2: "{x:[k|ş|t|s|h]}+d" => "{x}+t";
    }
};
