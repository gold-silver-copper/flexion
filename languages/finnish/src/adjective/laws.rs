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
        law0 = [("A", "-") => "ä", ("A", "a") => "a", ("A", "o") => "a", ("A", "u") => "a", ("A", "y") => "ä", ("A", "z") => "ä", ("A", "ä") => "ä", ("A", "ö") => "ä", ("A", "") => "ä"];
        law1 = [("I", "a") => "a", ("I", "e") => "ä", ("I", "i") => "i", ("I", "p") => "i", ("I", "s") => "ä", ("I", "t") => "i", ("I", "u") => "a", ("I", "y") => "ä", ("I", "ä") => "ä", ("I", "") => "i"];
        law3 = [("p", "l") => "p", ("p", "p") => "¦", ("p", "t") => "p", ("t", "l") => "l¦", ("t", "p") => "t", ("t", "t") => "¦"];
        law5 = [("k", "h") => "k", ("k", "i") => "k", ("k", "k") => "¦", ("k", "r") => "¦", ("k", "y") => "k", ("k", "ä") => "k", ("p", "h") => "p", ("p", "i") => "v¦", ("p", "k") => "p", ("p", "r") => "p", ("p", "y") => "v¦", ("p", "ä") => "p", ("t", "h") => "d¦", ("t", "i") => "d¦", ("t", "k") => "t", ("t", "r") => "t", ("t", "y") => "t", ("t", "ä") => "d¦"];
    }
    rules {
        // A → ä after -, y, z, ä, ö, none; a after a, o, u (the nearest of -, a, o, u, y, z, ä, ö before it, as written): 14,176 words (beige, indigo, khaki, vapaa), 687 forms wrong.
        law0[iterative]: "{x:[A]}" => "{x|law0[t]}" / "{t:[-|a|o|u|y|z|ä|ö]?}{g:![-|a|o|u|y|z|ä|ö]*}" _ "";
        // I → a after a, u; ä after e, s, y, ä; i after i, p, t, none (the nearest of a, e, i, p, s, t, u, y, ä before it, as written): 5,078 words (khaki, pisin, halal, leveä), 9 forms wrong.
        law1[iterative]: "{x:[I]}" => "{x|law1[t]}" / "{t:[a|e|i|p|s|t|u|y|ä]?}{g:![a|e|i|p|s|t|u|y|ä]*}" _ "";
        // ä+j → +i: 16 words (tuliterä, aikaavievä, kaikkinielevä, hapanimelä), 0 forms wrong.
        law2: "ä+j" => "+i";
        // t → ∅ after t; t → l after l; p → ∅ after p in alternating stems (ˇ), before +[a|i][^a|e|i|o|u|y|ä|ö]$, +[a|i][^a|e|i|o|u|y|ä|ö].., +e.[^a|e|i|o|u|y|ä|ö]$, +e.[^a|e|i|o|u|y|ä|ö].[a|e|i|o|u|y|ä|ö], +o...[a|e|i|o|u|y|ä|ö], .+i.$, .+i..[a|e|i|o|u|y|ä|ö], .+[k|l|m|s|t], .+n$: 444 words (violetti, rutto, loppu, helppo), 2 forms wrong.
        law3: "{x:[p|t]}" => "{x|law3[l]}" / "{l:[l|p|t]}" _ "+{[a|i]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{[a|i]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[k|l|m|s|t]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[n]}ˇ$";
        // i+e → +e: 9 words (makaaberi, obskuuri, populaari, primaari), 0 forms wrong.
        law4: "i+e" => "+e";
        // k → ∅ after [k|r]; t → d after [h|i|ä]; p → v after [i|y] in alternating stems (ˇ), before +[a|i|ä][^a|e|i|o|u|y|ä|ö]$, +[a|i|ä][^a|e|i|o|u|y|ä|ö].., +e.[^a|e|i|o|u|y|ä|ö]$, +e.[^a|e|i|o|u|y|ä|ö]i$, +e.[^a|e|i|o|u|y|ä|ö][l|s|t], +o.[d|k|l|s|t], +o.n$, +ö.[d|h|k|l|s|t], +ö.n$, .+i.$, .+i..[a|e|i|o|u|y|ä|ö], .+[k|l|s|t], .+n$: 139 words (märkä, pinkki, mätä, mahti), 28 forms wrong.
        law5: "{x:[k|p|t]}" => "{x|law5[l]}" / "{l:[h|i|k|r|y|ä]}" _ "+{[a|i|ä]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{[a|i|ä]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{[i]}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{[l|s|t]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[d|k|l|s|t]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "+{[ö]}{![+|ˇ|¦|°|‹]}{[d|h|k|l|s|t]}{![ˇ]*}ˇ$" | "+{[ö]}{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[k|l|s|t]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[n]}ˇ$";
        // copy of k → s: 14 words (kahdeskymmenes, yhdeksäskymmenes, neljäskymmenes, kolmaskymmenes), 0 forms wrong.
        law6: "°k" => "s";
        // t → n after n in alternating stems (ˇ), before +.[^a|e|i|o|u|y|ä|ö]$, +.[^a|e|i|o|u|y|ä|ö].., .+[a|e|i|o|u|y|ä|ö][^a|e|i|o|u|y|ä|ö].[a|e|i|o|u|y|ä|ö], .+[^a|e|i|o|u|y|ä|ö].[a|e|i|o|u|y|ä|ö]: 7 words (ikivihanta, lyhyenläntä, vihanta, vähänläntä), 0 forms wrong.
        law7: "t" => "n¦" / "{[n]}" _ "+{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$";
        // da+a → +taa: 9 words (hidas, puhdas, ahdas, epäpuhdas), 0 forms wrong.
        law8: "da+a" => "+taa";
        // da+i → +tai: 9 words (hidas, puhdas, ahdas, epäpuhdas), 0 forms wrong.
        law9: "da+i" => "+tai";
        // mi+mi → +pimi: 8 words (lämmin, uunilämmin, käyttölämmin, lypsylämmin), 0 forms wrong.
        law10: "mi+mi" => "+pimi";
        // mi+mä → +pimä: 8 words (lämmin, uunilämmin, käyttölämmin, lypsylämmin), 0 forms wrong.
        law11: "mi+mä" => "+pimä";
    }
};
