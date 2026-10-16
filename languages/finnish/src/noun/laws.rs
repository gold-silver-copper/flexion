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
        law0 = [("A", "a") => "a", ("A", "o") => "a", ("A", "u") => "a", ("A", "y") => "ä", ("A", "ä") => "ä", ("A", "ö") => "ä", ("A", "") => "ä"];
        law1 = [("V", "a") => "a", ("V", "e") => "e", ("V", "i") => "i", ("V", "o") => "o", ("V", "u") => "u", ("V", "y") => "y", ("V", "ä") => "ä", ("V", "ö") => "ö", ("V", "") => "e"];
        law2 = [("Ä", "a") => "a", ("Ä", "i") => "i", ("Ä", "m") => "m", ("Ä", "ä") => "ä", ("Ä", "") => "a"];
        law3 = [("T", "a") => "a", ("T", "t") => "t", ("T", "ä") => "ä", ("T", "") => "a"];
        law4 = [("k", "a") => "¦", ("k", "e") => "k", ("k", "h") => "k", ("k", "i") => "¦", ("k", "k") => "¦", ("k", "l") => "¦", ("k", "m") => "k", ("k", "n") => "g¦", ("k", "o") => "¦", ("k", "p") => "k", ("k", "r") => "¦", ("k", "t") => "k", ("k", "u") => "¦", ("k", "y") => "k", ("k", "ä") => "¦", ("p", "a") => "v¦", ("p", "e") => "p", ("p", "h") => "p", ("p", "i") => "v¦", ("p", "k") => "p", ("p", "l") => "p", ("p", "m") => "m¦", ("p", "n") => "p", ("p", "o") => "p", ("p", "p") => "¦", ("p", "r") => "v¦", ("p", "t") => "p", ("p", "u") => "v¦", ("p", "y") => "p", ("p", "ä") => "v¦", ("t", "a") => "d¦", ("t", "e") => "d¦", ("t", "h") => "d¦", ("t", "i") => "d¦", ("t", "k") => "t", ("t", "l") => "l¦", ("t", "m") => "t", ("t", "n") => "n¦", ("t", "o") => "d¦", ("t", "p") => "t", ("t", "r") => "r¦", ("t", "t") => "¦", ("t", "u") => "d¦", ("t", "y") => "d¦", ("t", "ä") => "d¦"];
        law14 = [("l", "l") => "t¦", ("l", "n") => "l", ("l", "r") => "l", ("n", "l") => "n", ("n", "n") => "t¦", ("n", "r") => "n", ("r", "l") => "r", ("r", "n") => "r", ("r", "r") => "t¦"];
    }
    rules {
        // A → a after a, o, u; ä after y, ä, ö, none (the nearest of a, o, u, y, ä, ö before it, as written): 5,359 words (swap, avatar, koira, luu), 1,078 forms wrong.
        law0[iterative]: "{x:[A]}" => "{x|law0[t]}" / "{t:[a|o|u|y|ä|ö]?}{g:![a|o|u|y|ä|ö]*}" _ "";
        // V → that letter; e after none (the nearest of a, e, i, o, u, y, ä, ö before it, as written): 5,988 words (swap, avatar, koira, luu), 30 forms wrong.
        law1[iterative]: "{x:[V]}" => "{x|law1[t]}" / "{t:[a|e|i|o|u|y|ä|ö]?}{g:![a|e|i|o|u|y|ä|ö]*}" _ "";
        // Ä → that letter; a after none (the nearest of a, i, m, ä before it, as written): 137 words (radio, body, valtio, mies), 18 forms wrong.
        law2[iterative]: "{x:[Ä]}" => "{x|law2[t]}" / "{t:[a|i|m|ä]?}{g:![a|i|m|ä]*}" _ "";
        // T → that letter; a after none (the nearest of a, t, ä before it, as written): 424 words (pata, rata, kita, body), 44 forms wrong.
        law3[iterative]: "{x:[T]}" => "{x|law3[t]}" / "{t:[a|t|ä]?}{g:![a|t|ä]*}" _ "";
        // k → ∅ after [a|i|k|l|o|r|u|ä]; t → ∅ after t; t → d after [a|e|h|i|o|u|y|ä]; t → n after n; p → ∅ after p; p → v after [a|i|r|u|ä]; k → g after n; t → l after l; t → r after r; p → m after m in alternating stems (ˇ), before +[a|i|ä][k|l|t], +[a|i|ä]n$, +[a|i|ä]s.., +ei.$, +ei.[^a|e|i|o|u|y|ä|ö], +e[k|l|t], +es.., +o.[d|k|l|s|t], +o.n$, +ö.[d|h|k|l|s|t], e+i.[^a|e|i|o|u|y|ä|ö], e+[k|l], e+s.., [o|u|ö]+i.$, [o|u|ö]+i.[^a|e|i|o|u|y|ä|ö], [o|u|ö]+[k|l|t], [o|u|ö]+n$, [o|u|ö]+s.., y+i.[^a|e|i|o|u|y|ä|ö], y+[k|l|t], y+s..: 1,059 words (pato, pata, itu, suka), 44 forms wrong.
        law4: "{x:[k|p|t]}" => "{x|law4[l]}" / "{l:[a|e|h|i|k|l|m|n|o|p|r|t|u|y|ä]}" _ "+{[a|i|ä]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[a|i|ä]}{[n]}ˇ$" | "+{[a|i|ä]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[e]}{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "+{[e]}{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "+{[e]}{[k|l|t]}{![ˇ]*}ˇ$" | "+{[e]}{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[d|k|l|s|t]}{![ˇ]*}ˇ$" | "+{[o]}{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "+{[ö]}{![+|ˇ|¦|°|‹]}{[d|h|k|l|s|t]}{![ˇ]*}ˇ$" | "{[e]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[e]}+{[k|l]}{![ˇ]*}ˇ$" | "{[e]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[o|u|ö]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[k|l|t]}{![ˇ]*}ˇ$" | "{[o|u|ö]}+{[n]}ˇ$" | "{[o|u|ö]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[y]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[y]}+{[k|l|t]}{![ˇ]*}ˇ$" | "{[y]}+{[s]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // ∅ → t after t in alternating stems (ˇ), before [a|e]+[a|e|i|o|u|y|ä|ö], i+.e, i+.i.: 17 words (ratas, kiihdytin, jäähdytin, syöte), 0 forms wrong.
        law5: "" => "t¦" / "{[t]}" _ "{[a|e]}+{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[i]}+{![+|ˇ|¦|°|‹]}{[e]}{![ˇ]*}ˇ$" | "{[i]}+{![+|ˇ|¦|°|‹]}{[i]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // [e|ä|a]+ii → +isii: 39 words (rike, liike, ärsyke, levyke), 0 forms wrong.
        law6: "{x:[e|ä|a]}+ii" => "{x}+isii";
        // e+ee → e+esee: 37 words (rike, liike, ärsyke, levyke), 0 forms wrong.
        law7: "e+ee" => "e+esee";
        // ke+te → k+keide: 37 words (rike, liike, ärsyke, levyke), 0 forms wrong.
        law8: "ke+te" => "k+keide";
        // de+e → +tee: 32 words (side, sade, luode, lähde), 6 forms wrong.
        law9: "de+e" => "+tee";
        // de+i → +tei: 32 words (side, sade, luode, lähde), 60 forms wrong.
        law10: "de+i" => "+tei";
        // [e|a]+ia → +ita: 22 words (ohdake, uloke, tarvike, vahvike), 0 forms wrong.
        law11: "{x:[e|a]}+ia" => "{x}+ita";
        // n+d → n+n: 8 words (kynsi, länsi, kansi, alakansi), 0 forms wrong.
        law12: "n+d" => "n+n";
        // e+ta → e+tta: 21 words (ohdake, uloke, tarvike, vahvike), 6 forms wrong.
        law13: "e+ta" => "e+tta";
        // n → t after n; r → t after r; l → t after l in alternating stems (ˇ), before .+[a|e|i|o|u|y|ä|ö], .+[^a|e|i|o|u|y|ä|ö][a|e|i|o|u|y|ä|ö], ..+[a|e|i|o|u|y|ä|ö]: 41 words (manner, tanner, aarre, anne), 0 forms wrong.
        law14: "{x:[l|n|r]}" => "{x|law14[l]}" / "{l:[l|n|r]}" _ "{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$";
        // [e|ä]+iä → +itä: 17 words (rike, liike, ärsyke, levyke), 0 forms wrong.
        law15: "{x:[e|ä]}+iä" => "{x}+itä";
        // r+d → r+r: 10 words (hirsi, varsi, pursi, orsi), 0 forms wrong.
        law16: "r+d" => "r+r";
        // ul+ej → ul+eit after a vowel earlier in the stem: 10 words (hangul, valkosipuli, sopuli, konsuli), 0 forms wrong.
        law17: "ul+ej" => "ul+eit" / "{[a|e|i|o|u|y|ä|ö]}{![a|e|i|o|u|y|ä|ö]*}" _ "";
        // ga+ → +ka: 7 words (kuningas, kangas, rengas, valkokangas), 49 forms wrong.
        law18: "ga+" => "+ka";
        // i+i → i+hi: 7 words (thai, pii, koi, täi), 0 forms wrong.
        law19: "i+i" => "i+hi";
        // us+ea → us+ta: 5 words (kuusi, jousi, joulukuusi, hevonkuusi), 0 forms wrong.
        law20: "us+ea" => "us+ta";
    }
};
