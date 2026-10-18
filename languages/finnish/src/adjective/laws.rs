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
        law0 = [("A", "-") => "ä", ("A", "a") => "a", ("A", "o") => "a", ("A", "u") => "a", ("A", "y") => "ä", ("A", "ä") => "ä", ("A", "ö") => "ä", ("A", "") => "ä"];
        law1 = [("V", "a") => "a", ("V", "e") => "e", ("V", "i") => "i", ("V", "o") => "o", ("V", "u") => "u", ("V", "y") => "y", ("V", "ä") => "ä", ("V", "ö") => "ö", ("V", "") => "a"];
        law3 = [("p", "l") => "p", ("p", "p") => "¦", ("p", "t") => "p", ("t", "l") => "l¦", ("t", "p") => "t", ("t", "t") => "¦"];
    }
    rules {
        // A → ä after -, y, ä, ö, none; a after a, o, u (the nearest of -, a, o, u, y, ä, ö before it, as written): 14,171 words (beige, indigo, khaki, vapaa), 687 forms wrong.
        law0[iterative]: "{x:[A]}" => "{x|law0[t]}" / "{t:[-|a|o|u|y|ä|ö]?}{g:![-|a|o|u|y|ä|ö]*}" _ "";
        // V → that letter; a after none (the nearest of a, e, i, o, u, y, ä, ö before it, as written): 2,368 words (beige, indigo, khaki, vapaa), 9 forms wrong.
        law1[iterative]: "{x:[V]}" => "{x|law1[t]}" / "{t:[a|e|i|o|u|y|ä|ö]?}{g:![a|e|i|o|u|y|ä|ö]*}" _ "";
        // ä+j → +i: 16 words (tuliterä, aikaavievä, kaikkinielevä, hapanimelä), 0 forms wrong.
        law2: "ä+j" => "+i";
        // t → ∅ after t; t → l after l; p → ∅ after p in alternating stems (ˇ), before +[a|i][^a|e|i|o|u|y|ä|ö]$, +[a|i][^a|e|i|o|u|y|ä|ö].., +e.[^a|e|i|o|u|y|ä|ö]$, +e.[^a|e|i|o|u|y|ä|ö].[a|e|i|o|u|y|ä|ö], +o...[a|e|i|o|u|y|ä|ö], .+i.$, .+i..[a|e|i|o|u|y|ä|ö], .+[k|l|m|s|t], .+n$: 445 words (violetti, rutto, loppu, helppo), 2 forms wrong.
        law3: "{x:[p|t]}" => "{x|law3[l]}" / "{l:[l|p|t]}" _ "+{[a|i]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{[a|i]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[k|l|m|s|t]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[n]}ˇ$";
        // i+e → +e: 9 words (makaaberi, obskuuri, populaari, primaari), 0 forms wrong.
        law4: "i+e" => "+e";
        // copy of k → s: 14 words (kahdeskymmenes, yhdeksäskymmenes, neljäskymmenes, kolmaskymmenes), 0 forms wrong.
        law5: "°k" => "s";
        // da+a → +taa: 9 words (hidas, puhdas, ahdas, epäpuhdas), 0 forms wrong.
        law6: "da+a" => "+taa";
        // da+i → +tai: 9 words (hidas, puhdas, ahdas, epäpuhdas), 0 forms wrong.
        law7: "da+i" => "+tai";
        // mi+mi → +pimi: 8 words (lämmin, uunilämmin, käyttölämmin, lypsylämmin), 0 forms wrong.
        law8: "mi+mi" => "+pimi";
        // mi+mä → +pimä: 8 words (lämmin, uunilämmin, käyttölämmin, lypsylämmin), 0 forms wrong.
        law9: "mi+mä" => "+pimä";
        // kä+i → k+i after t: 5 words (kokopitkä, puolipitkä, tasapitkä, esikoispitkä), 0 forms wrong.
        law10: "kä+i" => "k+i" / "{[t]}" _ "";
    }
};
