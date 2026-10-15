//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`), on a letter a `%` copies (`°`), on a class
//! letter an ending writes (harmony, by the nearest letter of a tier before
//! it as written), or inside a stem that alternates (`ˇ`, by what follows
//! it): every form a conjugation builds goes through them, in order, so one conjugation
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
        law2 = [("iw", "a") => "iw", ("iw", "b") => "iw", ("iw", "c") => "iw", ("iw", "d") => "iw", ("iw", "e") => "iw", ("iw", "f") => "iw", ("iw", "g") => "uj¦", ("iw", "h") => "uj¦", ("iw", "i") => "iw", ("iw", "j") => "iw", ("iw", "k") => "uj¦", ("iw", "l") => "iw", ("iw", "m") => "iw", ("iw", "n") => "iw", ("iw", "o") => "iw", ("iw", "p") => "iw", ("iw", "r") => "iw", ("iw", "s") => "iw", ("iw", "t") => "iw", ("iw", "u") => "iw", ("iw", "w") => "iw", ("iw", "y") => "iw", ("iw", "z") => "iw", ("iw", "ą") => "iw", ("iw", "ł") => "iw", ("iw", "ż") => "iw", ("k", "a") => "cz¦", ("k", "b") => "k", ("k", "c") => "k", ("k", "d") => "k", ("k", "e") => "k", ("k", "f") => "k", ("k", "g") => "k", ("k", "h") => "k", ("k", "i") => "k", ("k", "j") => "k", ("k", "k") => "k", ("k", "l") => "k", ("k", "m") => "k", ("k", "n") => "k", ("k", "o") => "k", ("k", "p") => "k", ("k", "r") => "k", ("k", "s") => "k", ("k", "t") => "k", ("k", "u") => "cz¦", ("k", "w") => "k", ("k", "y") => "k", ("k", "z") => "k", ("k", "ą") => "k", ("k", "ł") => "k", ("k", "ż") => "k", ("n", "a") => "ń¦", ("n", "b") => "n", ("n", "c") => "n", ("n", "d") => "n", ("n", "e") => "ń¦", ("n", "f") => "n", ("n", "g") => "n", ("n", "h") => "n", ("n", "i") => "ń¦", ("n", "j") => "n", ("n", "k") => "n", ("n", "l") => "n", ("n", "m") => "n", ("n", "n") => "n", ("n", "o") => "ń¦", ("n", "p") => "n", ("n", "r") => "n", ("n", "s") => "n", ("n", "t") => "ń¦", ("n", "u") => "n", ("n", "w") => "n", ("n", "y") => "ń¦", ("n", "z") => "n", ("n", "ą") => "n", ("n", "ł") => "n", ("n", "ż") => "n", ("ow", "a") => "ow", ("ow", "b") => "uj¦", ("ow", "c") => "uj¦", ("ow", "d") => "uj¦", ("ow", "e") => "ow", ("ow", "f") => "uj¦", ("ow", "g") => "uj¦", ("ow", "h") => "uj¦", ("ow", "i") => "uj¦", ("ow", "j") => "uj¦", ("ow", "k") => "uj¦", ("ow", "l") => "uj¦", ("ow", "m") => "uj¦", ("ow", "n") => "uj¦", ("ow", "o") => "ow", ("ow", "p") => "uj¦", ("ow", "r") => "uj¦", ("ow", "s") => "uj¦", ("ow", "t") => "uj¦", ("ow", "u") => "uj¦", ("ow", "w") => "uj¦", ("ow", "y") => "ow", ("ow", "z") => "uj¦", ("ow", "ą") => "ow", ("ow", "ł") => "uj¦", ("ow", "ż") => "uj¦", ("t", "a") => "t", ("t", "b") => "t", ("t", "c") => "t", ("t", "d") => "t", ("t", "e") => "t", ("t", "f") => "t", ("t", "g") => "t", ("t", "h") => "t", ("t", "i") => "t", ("t", "j") => "t", ("t", "k") => "t", ("t", "l") => "t", ("t", "m") => "t", ("t", "n") => "t", ("t", "o") => "cz¦", ("t", "p") => "cz¦", ("t", "r") => "t", ("t", "s") => "t", ("t", "t") => "t", ("t", "u") => "t", ("t", "w") => "t", ("t", "y") => "t", ("t", "z") => "t", ("t", "ą") => "cz¦", ("t", "ł") => "t", ("t", "ż") => "t", ("yw", "a") => "yw", ("yw", "b") => "uj¦", ("yw", "c") => "uj¦", ("yw", "d") => "uj¦", ("yw", "e") => "yw", ("yw", "f") => "yw", ("yw", "g") => "yw", ("yw", "h") => "yw", ("yw", "i") => "yw", ("yw", "j") => "yw", ("yw", "k") => "yw", ("yw", "l") => "yw", ("yw", "m") => "uj¦", ("yw", "n") => "uj¦", ("yw", "o") => "yw", ("yw", "p") => "uj¦", ("yw", "r") => "uj¦", ("yw", "s") => "uj¦", ("yw", "t") => "uj¦", ("yw", "u") => "yw", ("yw", "w") => "uj¦", ("yw", "y") => "yw", ("yw", "z") => "uj¦", ("yw", "ą") => "yw", ("yw", "ł") => "uj¦", ("yw", "ż") => "yw"];
        law10 = [("o", "d") => "ó¦", ("o", "g") => "ó¦", ("o", "i") => "o", ("o", "k") => "ó¦", ("o", "l") => "o", ("o", "m") => "ó¦", ("o", "n") => "ó¦", ("o", "p") => "ó¦", ("o", "r") => "ó¦", ("o", "s") => "ó¦", ("o", "w") => "ó¦", ("o", "ł") => "ó¦", ("ę", "d") => "ę", ("ę", "g") => "ę", ("ę", "i") => "ą¦", ("ę", "k") => "ę", ("ę", "l") => "ą¦", ("ę", "m") => "ę", ("ę", "n") => "ę", ("ę", "p") => "ę", ("ę", "r") => "ę", ("ę", "s") => "ę", ("ę", "w") => "ę", ("ę", "ł") => "ę"];
        law12 = [("w", "a") => "j¦", ("w", "i") => "w", ("w", "o") => "w", ("w", "ą") => "w", ("w", "ę") => "w", ("z", "a") => "ż¦", ("z", "i") => "ż¦", ("z", "o") => "ż¦", ("z", "ą") => "ż¦", ("z", "ę") => "ż¦"];
    }
    rules {
        // ow+e → +uje: 2,652 words (googlować, mulczować, dziękować, potrzebować), 9 forms wrong.
        law0: "ow+e" => "+uje";
        // ow+ą → +ują: 2,652 words (googlować, mulczować, dziękować, potrzebować), 0 forms wrong.
        law1: "ow+ą" => "+ują";
        // ow → uj after [b|c|d|f|g|h|i|j|k|l|m|n|p|r|s|t|u|w|z|ł|ż]; yw → uj after [b|c|d|m|n|p|r|s|t|w|z|ł]; n → ń after [a|e|i|o|t|y]; iw → uj after [g|h|k]; t → cz after [o|p|ą]; k → cz after [a|u] in alternating stems (ˇ), before +$, +[c|m|ę]: 3,043 words (googlować, mulczować, dziękować, potrzebować), 0 forms wrong.
        law2: "{x:[iw|k|n|ow|t|yw]}" => "{x|law2[l]}" / "{l:[a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|r|s|t|u|w|y|z|ą|ł|ż]}" _ "+ˇ$" | "+{[c|m|ę]}{![ˇ]*}ˇ$";
        // a+e → a+: 197 words (wychowywać, wylogowywać, przygotowywać, opodatkowywać), 0 forms wrong.
        law3: "a+e" => "a+";
        // a+o → a+: 197 words (wychowywać, wylogowywać, przygotowywać, opodatkowywać), 0 forms wrong.
        law4: "a+o" => "a+";
        // yw+e → +uje: 281 words (wskazywać, pokazywać, obiecywać, opisywać), 0 forms wrong.
        law5: "yw+e" => "+uje";
        // yw+ą → +ują: 281 words (wskazywać, pokazywać, obiecywać, opisywać), 0 forms wrong.
        law6: "yw+ą" => "+ują";
        // [l|z]+io → +o: 252 words (dzielić, pierdolić, mylić, palić), 0 forms wrong.
        law7: "{x:[l|z]}+io" => "{x}+o";
        // l+ię → l+ę: 256 words (dzielić, pierdolić, mylić, palić), 0 forms wrong.
        law8: "l+ię" => "l+ę";
        // l+ią → l+ą: 256 words (dzielić, pierdolić, mylić, palić), 0 forms wrong.
        law9: "l+ią" => "l+ą";
        // o → ó after [d|g|k|m|n|p|r|s|w|ł]; ę → ą after [i|l] in alternating stems (ˇ), before +[^a|e|i|o|u|y|ó|ą|ę]$, +[^a|e|i|o|u|y|ó|ą|ę][c|m], +[^a|e|i|o|u|y|ó|ą|ę]ł$, +[^a|e|i|o|u|y|ó|ą|ę]ł.[y|z], [^a|e|i|o|u|y|ó|ą|ę]+$, [^a|e|i|o|u|y|ó|ą|ę]+c..$, [^a|e|i|o|u|y|ó|ą|ę]+m, [^a|e|i|o|u|y|ó|ą|ę]+ł$, [^a|e|i|o|u|y|ó|ą|ę]+ł.[y|z], [^a|e|i|o|u|y|ó|ą|ę][^a|e|i|o|u|y|ó|ą|ę]+$, [^a|e|i|o|u|y|ó|ą|ę][^a|e|i|o|u|y|ó|ą|ę]+.i[a|e|i|o|u|y|ó|ą|ę], [^a|e|i|o|u|y|ó|ą|ę][^a|e|i|o|u|y|ó|ą|ę]+.y: 106 words (robić, zrobić, solić, tworzyć), 0 forms wrong.
        law10: "{x:[o|ę]}" => "{x|law10[l]}" / "{l:[d|g|i|k|l|m|n|p|r|s|w|ł]}" _ "+{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}ˇ$" | "+{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}{[c|m]}{![ˇ]*}ˇ$" | "+{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}{[ł]}ˇ$" | "+{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}{[ł]}{![+|ˇ|¦|°|‹]}{[y|z]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+{[c]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+{[m]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+{[ł]}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+{[ł]}{![+|ˇ|¦|°|‹]}{[y|z]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+{![+|ˇ|¦|°|‹]}{[i]}{[a|e|i|o|u|y|ó|ą|ę]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+{![+|ˇ|¦|°|‹]}{[y]}{![ˇ]*}ˇ$";
        // s → ś after [a|e|i|o|u|y|ą] in alternating stems (ˇ), before +$, +.i, .+i: 74 words (zasnąć, rosnąć, usnąć, kisnąć), 9 forms wrong.
        law11: "s" => "ś¦" / "{[a|e|i|o|u|y|ą]}" _ "+ˇ$" | "+{![+|ˇ|¦|°|‹]}{[i]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[i]}{![ˇ]*}ˇ$";
        // z → ż after [a|i|o|ą|ę]; w → j after a in alternating stems (ˇ), before +$, +c..$, +[e|m|o|ą|ę]: 66 words (lizać, obrazić, grozić, wiązać), 0 forms wrong.
        law12: "{x:[w|z]}" => "{x|law12[l]}" / "{l:[a|i|o|ą|ę]}" _ "+ˇ$" | "+{[c]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}ˇ$" | "+{[e|m|o|ą|ę]}{![ˇ]*}ˇ$";
        // t+z → +cz: 88 words (szeptać, bełkotać, chłeptać, mamrotać), 0 forms wrong.
        law13: "t+z" => "+cz";
        // i+y → i+: 5 words (rozbrzmieć, zagrzmieć, zabrzmieć, nabrzmieć), 0 forms wrong.
        law14: "i+y" => "i+";
        // o+j → +ój: 49 words (poić, doić, koić, kroić), 6 forms wrong.
        law15: "o+j" => "+ój";
        // [s|r]+ej → +z: 40 words (napisać, podpisać, zaorać, przepisać), 11 forms wrong.
        law16: "{x:[s|r]}+ej" => "{x}+z";
        // s+c → s+zc: 5 words (świstać, chłostać, chlastać, zaświstać), 0 forms wrong.
        law17: "s+c" => "s+zc";
        // e → a after [i|l] in alternating stems (ˇ), before +ł, [^a|e|i|o|u|y|ó|ą|ę]+ł: 7 words (dnieć, blednąć, odechcieć, zachcieć), 0 forms wrong.
        law18: "e" => "a¦" / "{[i|l]}" _ "+{[ł]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ó|ą|ę]}+{[ł]}{![ˇ]*}ˇ$";
        // j+i → j+: 12 words (łajać, tajać, krajać, złajać), 0 forms wrong.
        law19: "j+i" => "j+";
        // un+ij → u+ń: 21 words (usunąć, sunąć, splunąć, runąć), 3 forms wrong.
        law20: "un+ij" => "u+ń";
        // k+z → +cz: 15 words (płakać, gdakać, skakać, krakać), 0 forms wrong.
        law21: "k+z" => "+cz";
        // zy+n → zy+t: 12 words (przyszyć, uszyć, doszyć, naszyć), 0 forms wrong.
        law22: "zy+n" => "zy+t";
        // źd+ż → +żdż: 13 words (jeździć, ujeździć, gnieździć, zagnieździć), 0 forms wrong.
        law23: "źd+ż" => "+żdż";
        // c+n → +tn: 9 words (przeciąć, naciąć, wyciąć, pociąć), 0 forms wrong.
        law24: "c+n" => "+tn";
        // [i|z|l]+z → +ej: 7 words (piać, śmiać, siać, wiać), 0 forms wrong.
        law25: "{x:[i|z|l]}+z" => "{x}+ej";
        // ss+z → ss+: 7 words (ssać, wyssać, wessać, przyssać), 0 forms wrong.
        law26: "ss+z" => "ss+";
        // [d|z|w|b]+t → +et: 6 words (odciąć, rozciąć, wciąć, obciąć), 0 forms wrong.
        law27: "{x:[d|z|w|b]}+t" => "{x}+et";
        // fi+j → f+: 5 words (trafić, trefić, przytrafić, natrafić), 0 forms wrong.
        law28: "fi+j" => "f+";
        // y+cz → y+ż: 5 words (strzyc, ostrzyc, podstrzyc, przystrzyc), 0 forms wrong.
        law29: "y+cz" => "y+ż";
        // y+k → y+g: 5 words (strzyc, ostrzyc, podstrzyc, przystrzyc), 0 forms wrong.
        law30: "y+k" => "y+g";
    }
};
