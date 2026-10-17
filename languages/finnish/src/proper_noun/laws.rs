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
        law0 = [("A", "a") => "a", ("A", "o") => "a", ("A", "u") => "a", ("A", "y") => "ä", ("A", "z") => "ä", ("A", "ä") => "ä", ("A", "å") => "a", ("A", "ö") => "ä", ("A", "ž") => "a", ("A", "") => "ä"];
        law1 = [("V", "a") => "a", ("V", "e") => "e", ("V", "i") => "i", ("V", "o") => "o", ("V", "u") => "u", ("V", "y") => "y", ("V", "ä") => "ä", ("V", "å") => "å", ("V", "ö") => "ö", ("V", "") => "o"];
        law2 = [("E", "a") => "a", ("E", "e") => "e", ("E", "i") => "i", ("E", "m") => "m", ("E", "ä") => "ä", ("E", "") => "a"];
        law3 = [("k", "a") => "k", ("k", "e") => "k", ("k", "h") => "k", ("k", "i") => "k", ("k", "k") => "¦", ("k", "l") => "¦", ("k", "m") => "k", ("k", "n") => "g¦", ("k", "o") => "¦", ("k", "p") => "k", ("k", "r") => "k", ("k", "u") => "k", ("k", "ä") => "¦", ("p", "a") => "v¦", ("p", "e") => "p", ("p", "h") => "p", ("p", "i") => "p", ("p", "k") => "p", ("p", "l") => "p", ("p", "m") => "m¦", ("p", "n") => "p", ("p", "o") => "p", ("p", "p") => "¦", ("p", "r") => "v¦", ("p", "u") => "p", ("p", "ä") => "p", ("t", "a") => "t", ("t", "e") => "d¦", ("t", "h") => "d¦", ("t", "i") => "d¦", ("t", "k") => "t", ("t", "l") => "l¦", ("t", "m") => "t", ("t", "n") => "n¦", ("t", "o") => "d¦", ("t", "p") => "t", ("t", "r") => "r¦", ("t", "u") => "d¦", ("t", "ä") => "t"];
        law7 = [("g", "n") => "k¦", ("g", "r") => "g", ("n", "n") => "t¦", ("n", "r") => "n", ("r", "n") => "r", ("r", "r") => "t¦"];
    }
    rules {
        // A → a after a, o, u, å, ž; ä after y, z, ä, ö, none (the nearest of a, o, u, y, z, ä, å, ö, ž before it, as written): 7,796 words (israel, bangladesh, bahrain, burundi), 1,123 forms wrong.
        law0[iterative]: "{x:[A]}" => "{x|law0[t]}" / "{t:[a|o|u|y|z|ä|å|ö|ž]?}{g:![a|o|u|y|z|ä|å|ö|ž]*}" _ "";
        // V → that letter; o after none (the nearest of a, e, i, o, u, y, ä, å, ö before it, as written): 8,985 words (israel, latvia, algeria, tunisia), 0 forms wrong.
        law1[iterative]: "{x:[V]}" => "{x|law1[t]}" / "{t:[a|e|i|o|u|y|ä|å|ö]?}{g:![a|e|i|o|u|y|ä|å|ö]*}" _ "";
        // E → that letter; a after none (the nearest of a, e, i, m, ä before it, as written): 166 words (susi, krapu, kuopio, tapio), 10 forms wrong.
        law2[iterative]: "{x:[E]}" => "{x|law2[t]}" / "{t:[a|e|i|m|ä]?}{g:![a|e|i|m|ä]*}" _ "";
        // k → ∅ after [k|l|o|ä]; t → d after [e|h|i|o|u]; t → n after n; p → ∅ after p; k → g after n; t → r after r; p → m after m; p → v after [a|r]; t → l after l in alternating stems (ˇ), before +[a|i|ä][k|l|t], +[a|i|ä]n$, +[a|i|ä]s.., +ei.$, +ei.[^a|e|i|o|u|y|ä|å|ö], +e[k|l|t], +en$, +es.., +o.[d|k|l|s|t], +o.n$, +ö.[d|h|k|l|s|t], i+[k|l|t], i+s.., [o|u|ö]+i.$, [o|u|ö]+i.[^a|e|i|o|u|y|ä|å|ö], [o|u|ö]+[k|l|t], [o|u|ö]+n$, [o|u|ö]+s..: 1,149 words (helsinki, ahti, ahto, lähi-itä), 94 forms wrong.
        law3: "{x:[k|p|t]}" => "{x|law3[l]}" / "{l:[a|e|h|i|k|l|m|n|o|p|r|u|ä]}" _ "+{[a|i|ä]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[a|i|ä]}{[n]}ˇ$" | "+{[a|i|ä]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "+{[e]}{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$" | "+{[e]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[e]}{[n]}ˇ$" | "+{[e]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[d|k|l|s|t]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "+{[ö]}{![+|ˇ|¦|°|‹]}{[d|h|k|l|s|t]}{![ˇ]*}ˇ$" | "{[i]}+{[k|l|t]}{![ˇ]*}ˇ$" | "{[i]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[o|u|ö]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[k|l|t]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[n]}ˇ$" | "{[o|u|ö]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // t → ∅ after t in alternating stems (ˇ), before +[a|i][k|l|t], +[a|i]n$, +[a|i]s.., +e.[^a|e|i|o|u|y|ä|å|ö]$, +e.[^a|e|i|o|u|y|ä|å|ö][^a|e|i|o|u|y|ä|å|ö], +o.[d|k|l|s|t], +o.n$, +ä[k|l|t], +äs.., .+i.$, .+i.[^a|e|i|o|u|y|ä|å|ö], .+[k|l|t], .+n$, .+s..: 180 words (raamattu, kettu, otto, korintti), 0 forms wrong.
        law4: "t" => "¦" / "{[t]}" _ "+{[a|i]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[a|i]}{[n]}ˇ$" | "+{[a|i]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[d|k|l|s|t]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "+{[ä]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[ä]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[k|l|t]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[n]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // em+t → e+nt: 17 words (hyväntoivonniemi, rovaniemi, kirkkoniemi, pelkosenniemi), 0 forms wrong.
        law5: "em+t" => "e+nt";
        // io+a → io+ta: 29 words (ontario, ohio, tokio, kolmio), 0 forms wrong.
        law6: "io+a" => "io+ta";
        // g → k after n; n → t after n; r → t after r in alternating stems (ˇ), before .+[a|e|i|o|u|y|ä|å|ö]: 81 words (rinne, päijänne, selänne, kangas), 0 forms wrong.
        law7: "{x:[g|n|r]}" => "{x|law7[l]}" / "{l:[n|r]}" _ "{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$";
        // iö+ä → iö+tä: 10 words (suomi-yhtiö, kihniö, enontekiö, närpiö), 0 forms wrong.
        law8: "iö+ä" => "iö+tä";
        // är+ej → är+eit: 12 words (mänttäri, kähäri, kämäri, spännäri), 0 forms wrong.
        law9: "är+ej" => "är+eit";
        // ai+i → ai+hi: 6 words (mumbai, kai, nikolai, iisai), 0 forms wrong.
        law10: "ai+i" => "ai+hi";
        // au+a → au+ta: 5 words (tokelau, palau, guinea-bissau, chisinau), 0 forms wrong.
        law11: "au+a" => "au+ta";
        // au+u → au+hu: 5 words (tokelau, palau, guinea-bissau, chisinau), 0 forms wrong.
        law12: "au+u" => "au+hu";
        // iv+ea → iv+eä: 5 words (paasikivi, koirikivi, papinkivi, saarikivi), 0 forms wrong.
        law13: "iv+ea" => "iv+eä";
        // ∅ → p after p in alternating stems (ˇ), before a+[a|e|i|o|u|y|ä|å|ö]: 5 words (opas, kempas, laapas, valpas), 0 forms wrong.
        law14: "" => "p¦" / "{[p]}" _ "{[a]}+{[a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$";
    }
};
