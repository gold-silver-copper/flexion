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
        law0 = [("A", "a") => "a", ("A", "o") => "a", ("A", "u") => "a", ("A", "y") => "ä", ("A", "ä") => "ä", ("A", "å") => "a", ("A", "ö") => "ä", ("A", "") => "ä"];
        law1 = [("I", "-") => "i", ("I", "c") => "i", ("I", "i") => "i", ("I", "p") => "e", ("I", "ä") => "e", ("I", "") => "i"];
        law2 = [("V", "a") => "a", ("V", "e") => "e", ("V", "i") => "i", ("V", "o") => "o", ("V", "u") => "u", ("V", "y") => "y", ("V", "ä") => "ä", ("V", "å") => "å", ("V", "ö") => "ö", ("V", "") => "o"];
        law3 = [("E", "a") => "a", ("E", "e") => "e", ("E", "i") => "i", ("E", "m") => "m", ("E", "ä") => "ä", ("E", "") => "a"];
        law4 = [("k", "a") => "k", ("k", "e") => "k", ("k", "h") => "k", ("k", "i") => "k", ("k", "k") => "¦", ("k", "l") => "¦", ("k", "m") => "k", ("k", "n") => "g¦", ("k", "o") => "¦", ("k", "p") => "k", ("k", "r") => "k", ("k", "u") => "k", ("k", "ä") => "¦", ("p", "a") => "v¦", ("p", "e") => "p", ("p", "h") => "p", ("p", "i") => "p", ("p", "k") => "p", ("p", "l") => "p", ("p", "m") => "m¦", ("p", "n") => "p", ("p", "o") => "p", ("p", "p") => "¦", ("p", "r") => "v¦", ("p", "u") => "p", ("p", "ä") => "p", ("t", "a") => "t", ("t", "e") => "d¦", ("t", "h") => "d¦", ("t", "i") => "d¦", ("t", "k") => "t", ("t", "l") => "l¦", ("t", "m") => "t", ("t", "n") => "n¦", ("t", "o") => "d¦", ("t", "p") => "t", ("t", "r") => "r¦", ("t", "u") => "d¦", ("t", "ä") => "t"];
        law12 = [("g", "n") => "k¦", ("n", "n") => "t¦"];
    }
    rules {
        // A → a after a, o, u, å; ä after y, ä, ö, none (the nearest of a, o, u, y, ä, å, ö before it, as written): 7,796 words (israel, bangladesh, bahrain, burundi), 1,033 forms wrong.
        law0[iterative]: "{x:[A]}" => "{x|law0[t]}" / "{t:[a|o|u|y|ä|å|ö]?}{g:![a|o|u|y|ä|å|ö]*}" _ "";
        // I → i after -, c, i, none; e after p, ä (the nearest of -, c, i, p, ä before it, as written): 730 words (israel, bangladesh, bahrain, bhutan), 731 forms wrong.
        law1[iterative]: "{x:[I]}" => "{x|law1[t]}" / "{t:[-|c|i|p|ä]?}{g:![-|c|i|p|ä]*}" _ "";
        // V → that letter; o after none (the nearest of a, e, i, o, u, y, ä, å, ö before it, as written): 8,985 words (israel, latvia, algeria, tunisia), 0 forms wrong.
        law2[iterative]: "{x:[V]}" => "{x|law2[t]}" / "{t:[a|e|i|o|u|y|ä|å|ö]?}{g:![a|e|i|o|u|y|ä|å|ö]*}" _ "";
        // E → that letter; a after none (the nearest of a, e, i, m, ä before it, as written): 166 words (susi, krapu, kuopio, tapio), 10 forms wrong.
        law3[iterative]: "{x:[E]}" => "{x|law3[t]}" / "{t:[a|e|i|m|ä]?}{g:![a|e|i|m|ä]*}" _ "";
        // k → ∅ after [k|l|o|ä]; t → d after [e|h|i|o|u]; t → n after n; p → ∅ after p; k → g after n; t → r after r; p → m after m; p → v after [a|r]; t → l after l in alternating stems (ˇ), before +[a|i|ä][k|l|t], +[a|i|ä]n$, +[a|i|ä]s.., +ei.$, +ei.[^a|e|i|o|u|y|ä|å|ö], +e[k|l|t], +en$, +es.., +o.[d|k|l|s|t], +o.n$, +ö.[d|h|k|l|s|t], i+[k|l|t], i+s.., [o|u|ö]+i.$, [o|u|ö]+i.[^a|e|i|o|u|y|ä|å|ö], [o|u|ö]+[k|l|t], [o|u|ö]+n$, [o|u|ö]+s..: 1,149 words (helsinki, ahti, ahto, lähi-itä), 94 forms wrong.
        law4: "{x:[k|p|t]}" => "{x|law4[l]}" / "{l:[a|e|h|i|k|l|m|n|o|p|r|u|ä]}" _ "+{[a|i|ä]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[a|i|ä]}{[n]}ˇ$" | "+{[a|i|ä]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "+{[e]}{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$" | "+{[e]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[e]}{[n]}ˇ$" | "+{[e]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[d|k|l|s|t]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "+{[ö]}{![+|ˇ|¦|°|‹]}{[d|h|k|l|s|t]}{![ˇ]*}ˇ$" | "{[i]}+{[k|l|t]}{![ˇ]*}ˇ$" | "{[i]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[o|u|ö]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[k|l|t]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[n]}ˇ$" | "{[o|u|ö]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // [n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž|j]+ks → +iks: 8 words (nepal, paul, pjongjang, palander), 0 forms wrong.
        law5: "{x:[n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž|j]}+ks" => "{x}+iks";
        // [n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]+mm → +imm: 8 words (nepal, paul, pjongjang, palander), 0 forms wrong.
        law6: "{x:[n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]}+mm" => "{x}+imm";
        // [n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]+nn → +inn: 8 words (nepal, paul, pjongjang, palander), 0 forms wrong.
        law7: "{x:[n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]}+nn" => "{x}+inn";
        // [n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]+ns → +ins: 8 words (nepal, paul, pjongjang, palander), 0 forms wrong.
        law8: "{x:[n|r|t|l|k|d|s|m|v|g|h|š|x|f|c|b|ž]}+ns" => "{x}+ins";
        // t → ∅ after t in alternating stems (ˇ), before +[a|i][k|l|t], +[a|i]n$, +[a|i]s.., +e.[^a|e|i|o|u|y|ä|å|ö]$, +e.[^a|e|i|o|u|y|ä|å|ö][^a|e|i|o|u|y|ä|å|ö], +o.[d|k|l|s|t], +o.n$, +ä[k|l|t], +äs.., .+i.$, .+i.[^a|e|i|o|u|y|ä|å|ö], .+[k|l|t], .+n$, .+s..: 180 words (raamattu, kettu, otto, korintti), 0 forms wrong.
        law9: "t" => "¦" / "{[t]}" _ "+{[a|i]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[a|i]}{[n]}ˇ$" | "+{[a|i]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}ˇ$" | "+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[d|k|l|s|t]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "+{[ä]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[ä]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[k|l|t]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[n]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // em+t → e+nt: 17 words (hyväntoivonniemi, rovaniemi, kirkkoniemi, pelkosenniemi), 0 forms wrong.
        law10: "em+t" => "e+nt";
        // io+a → io+ta: 29 words (ontario, ohio, tokio, kolmio), 0 forms wrong.
        law11: "io+a" => "io+ta";
        // g → k after n; n → t after n in alternating stems (ˇ), before .+[a|e|i|o|u|y|ä|å|ö]: 75 words (rinne, päijänne, selänne, kangas), 0 forms wrong.
        law12: "{x:[g|n]}" => "{x|law12[l]}" / "{l:[n]}" _ "{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$";
        // iö+ä → iö+tä: 10 words (suomi-yhtiö, kihniö, enontekiö, närpiö), 0 forms wrong.
        law13: "iö+ä" => "iö+tä";
        // [ni|si|ti|mi|li]+h → +: 52 words (delfiini, japani, berliini, vatikaani), 0 forms wrong.
        law14: "{x:[ni|si|ti|mi|li]}+h" => "{x}+";
        // är+ej → är+eit: 12 words (mänttäri, kähäri, kämäri, spännäri), 0 forms wrong.
        law15: "är+ej" => "är+eit";
        // au+a → au+ta: 5 words (tokelau, palau, guinea-bissau, chisinau), 0 forms wrong.
        law16: "au+a" => "au+ta";
        // au+u → au+hu: 5 words (tokelau, palau, guinea-bissau, chisinau), 0 forms wrong.
        law17: "au+u" => "au+hu";
        // iv+ea → iv+eä: 5 words (paasikivi, koirikivi, papinkivi, saarikivi), 0 forms wrong.
        law18: "iv+ea" => "iv+eä";
        // [ji|vi]+h → +: 10 words (havaiji, savoiji, bastilji, pompeji), 0 forms wrong.
        law19: "{x:[ji|vi]}+h" => "{x}+";
        // kä+e → kä+i: 5 words (äikäs, kivekäs, kärkäs, hyrkäs), 0 forms wrong.
        law20: "kä+e" => "kä+i";
        // ∅ → p after p in alternating stems (ˇ), before a+[a|e|i|o|u|y|ä|å|ö]: 5 words (opas, kempas, laapas, valpas), 0 forms wrong.
        law21: "" => "p¦" / "{[p]}" _ "{[a]}+{[a|e|i|o|u|y|ä|å|ö]}{![ˇ]*}ˇ$";
        // [r|n|d|l|s|p|m|h|g]+ei → +ii before [d|m]: 6 words (tigris, artemis, kauris, akropolis), 0 forms wrong.
        law22: "{x:[r|n|d|l|s|p|m|h|g]}+ei" => "{x}+ii" / "" _ "{[d|m]}";
    }
};
