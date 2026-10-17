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
        law0 = [("A", " ") => "ä", ("A", "a") => "a", ("A", "o") => "a", ("A", "u") => "a", ("A", "y") => "ä", ("A", "ä") => "ä", ("A", "ö") => "ä", ("A", "") => "ä", ("T", " ") => "ä", ("T", "a") => "t", ("T", "o") => "a", ("T", "u") => "a", ("T", "y") => "ä", ("T", "ä") => "t", ("T", "ö") => "ä", ("T", "") => "ä", ("U", " ") => "y", ("U", "a") => "u", ("U", "o") => "u", ("U", "u") => "u", ("U", "y") => "y", ("U", "ä") => "y", ("U", "ö") => "y", ("U", "") => "y", ("O", " ") => "ö", ("O", "a") => "o", ("O", "o") => "o", ("O", "u") => "o", ("O", "y") => "ö", ("O", "ä") => "ö", ("O", "ö") => "ö", ("O", "") => "ö"];
        law1 = [("V", "a") => "a", ("V", "e") => "e", ("V", "i") => "i", ("V", "o") => "o", ("V", "u") => "u", ("V", "y") => "y", ("V", "ä") => "ä", ("V", "ö") => "ö", ("V", "") => "u"];
        law2 = [("k", "a") => "k", ("k", "h") => "k", ("k", "i") => "k", ("k", "l") => "k", ("k", "n") => "g¦", ("k", "o") => "k", ("k", "r") => "k", ("k", "t") => "k", ("k", "u") => "k", ("k", "y") => "k", ("k", "ä") => "k", ("p", "a") => "p", ("p", "h") => "p", ("p", "i") => "v¦", ("p", "l") => "p", ("p", "n") => "p", ("p", "o") => "v¦", ("p", "r") => "v¦", ("p", "t") => "p", ("p", "u") => "p", ("p", "y") => "p", ("p", "ä") => "p", ("t", "a") => "d¦", ("t", "h") => "d¦", ("t", "i") => "d¦", ("t", "l") => "l¦", ("t", "n") => "n¦", ("t", "o") => "d¦", ("t", "r") => "r¦", ("t", "t") => "¦", ("t", "u") => "d¦", ("t", "y") => "d¦", ("t", "ä") => "d¦"];
        law9 = [("d", "a") => "t¦", ("d", "h") => "t¦", ("d", "i") => "t¦", ("d", "l") => "d", ("d", "m") => "d", ("d", "n") => "d", ("d", "r") => "d", ("d", "u") => "t¦", ("g", "a") => "g", ("g", "h") => "g", ("g", "i") => "g", ("g", "l") => "g", ("g", "m") => "g", ("g", "n") => "k¦", ("g", "r") => "g", ("g", "u") => "g", ("j", "a") => "j", ("j", "h") => "j", ("j", "i") => "j", ("j", "l") => "k¦", ("j", "m") => "j", ("j", "n") => "j", ("j", "r") => "k¦", ("j", "u") => "j", ("l", "a") => "l", ("l", "h") => "l", ("l", "i") => "l", ("l", "l") => "t¦", ("l", "m") => "l", ("l", "n") => "l", ("l", "r") => "l", ("l", "u") => "l", ("m", "a") => "m", ("m", "h") => "m", ("m", "i") => "m", ("m", "l") => "m", ("m", "m") => "p¦", ("m", "n") => "m", ("m", "r") => "m", ("m", "u") => "m", ("n", "a") => "n", ("n", "h") => "n", ("n", "i") => "n", ("n", "l") => "n", ("n", "m") => "n", ("n", "n") => "t¦", ("n", "r") => "n", ("n", "u") => "n", ("r", "a") => "r", ("r", "h") => "r", ("r", "i") => "r", ("r", "l") => "r", ("r", "m") => "r", ("r", "n") => "r", ("r", "r") => "t¦", ("r", "u") => "r", ("v", "a") => "p¦", ("v", "h") => "v", ("v", "i") => "p¦", ("v", "l") => "p¦", ("v", "m") => "v", ("v", "n") => "v", ("v", "r") => "v", ("v", "u") => "v"];
        law11 = [("k", "a") => "¦", ("k", "i") => "¦", ("k", "k") => "¦", ("k", "l") => "¦", ("k", "m") => "k", ("k", "r") => "¦", ("k", "ä") => "¦", ("p", "a") => "p", ("p", "i") => "p", ("p", "k") => "p", ("p", "l") => "p", ("p", "m") => "m¦", ("p", "r") => "p", ("p", "ä") => "p"];
    }
    rules {
        // A → ä after  , y, ä, ö, none; a after a, o, u; T → ä after  , y, ö, none; t after a, ä; a after o, u; U → y after  , y, ä, ö, none; u after a, o, u; O → ö after  , y, ä, ö, none; o after a, o, u (the nearest of  , a, o, u, y, ä, ö before it, as written): 12,046 words (anoa, naida, tuulla, taikoa), 2,254 forms wrong.
        law0[iterative]: "{x:[A|T|U|O]}" => "{x|law0[t]}" / "{t:[ |a|o|u|y|ä|ö]?}{g:![ |a|o|u|y|ä|ö]*}" _ "";
        // V → that letter; u after none (the nearest of a, e, i, o, u, y, ä, ö before it, as written): 3,096 words (naida, tuulla, uida, kävellä), 2 forms wrong.
        law1[iterative]: "{x:[V]}" => "{x|law1[t]}" / "{t:[a|e|i|o|u|y|ä|ö]?}{g:![a|e|i|o|u|y|ä|ö]*}" _ "";
        // t → ∅ after t; t → d after [a|h|i|o|u|y|ä]; t → n after n; t → r after r; t → l after l; p → v after [i|o|r]; k → g after n in alternating stems (ˇ), before +[a|ä]$, +[a|ä][^a|e|i|o|u|y|ä|ö]$, +[a|ä][^a|e|i|o|u|y|ä|ö][^a|e|i|o|u|y|ä|ö], +e, +i.$, +i..[a|e|i|o|u|y|ä|ö], i+m.[a|e|i|o|u|y|ä|ö]$, i+n$, i+t, [u|y]+$, [u|y]+i.$, [u|y]+i..[a|e|i|o|u|y|ä|ö], [u|y]+m.[a|e|i|o|u|y|ä|ö]$, [u|y]+n$, [u|y]+t: 4,484 words (piirtää, hiihtää, kirjoittaa, heittää), 498 forms wrong.
        law2: "{x:[k|p|t]}" => "{x|law2[l]}" / "{l:[a|h|i|l|n|o|r|t|u|y|ä]}" _ "+{[a|ä]}ˇ$" | "+{[a|ä]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{[a|ä]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "+{[e]}{![ˇ]*}ˇ$" | "+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[i]}+{[m]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}ˇ$" | "{[i]}+{[n]}ˇ$" | "{[i]}+{[t]}{![ˇ]*}ˇ$" | "{[u|y]}+ˇ$" | "{[u|y]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[u|y]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[u|y]}+{[m]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}ˇ$" | "{[u|y]}+{[n]}ˇ$" | "{[u|y]}+{[t]}{![ˇ]*}ˇ$";
        // [n|r|l|y|u|i|e]+di → +si: 224 words (pyytää, löytää, lentää, lyhentää), 24 forms wrong.
        law3: "{x:[n|r|l|y|u|i|e]}+di" => "{x}+si";
        // n+d → n+n: 225 words (antaa, lentää, lyhentää, suomentaa), 0 forms wrong.
        law4: "n+d" => "n+n";
        // n+ti → n+si: 246 words (lentää, lyhentää, suomentaa, ruotsintaa), 8 forms wrong.
        law5: "n+ti" => "n+si";
        // ∅ → k after [a|i|k|l|o|r|u] in alternating stems (ˇ), before e+[a|s|ä], e+n[a|e|i|o|u|y|ä|ö], ä+.$, ä+.[i|m|s|v], ä+.n$, ä+.t$, ä+.t[^a|e|i|o|u|y|ä|ö]: 70 words (paeta, aueta, pelätä, hyökätä), 0 forms wrong.
        law6: "" => "k¦" / "{[a|i|k|l|o|r|u]}" _ "{[e]}+{[a|s|ä]}{![ˇ]*}ˇ$" | "{[e]}+{[n]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}{[i|m|s|v]}{![ˇ]*}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}{[t]}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}{[t]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$";
        // [s|t]+tt → +t: 7 words (juosta, syöstä, piestä, rangaista), 0 forms wrong.
        law7: "{x:[s|t]}+tt" => "{x}+t";
        // s+nn → s+s: 7 words (juosta, syöstä, piestä, rangaista), 0 forms wrong.
        law8: "s+nn" => "s+s";
        // d → t after [a|h|i|u]; n → t after n; r → t after r; l → t after l; j → k after [l|r]; v → p after [a|i|l]; g → k after n; m → p after m in alternating stems (ˇ), before .+[a|i|s|ä], .+n[a|e|i|o|u|y|ä|ö], ..+[a|e|i|o|u|y|ä|ö]: 333 words (valehdella, kuunnella, ladata, kimmota), 0 forms wrong.
        law9: "{x:[d|g|j|l|m|n|r|v]}" => "{x|law9[l]}" / "{l:[a|h|i|l|m|n|r|u]}" _ "{![+|ˇ|¦|°|‹]}+{[a|i|s|ä]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[n]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$";
        // s+tk → s+k: 7 words (juosta, syöstä, piestä, rangaista), 0 forms wrong.
        law10: "s+tk" => "s+k";
        // k → ∅ after [a|i|k|l|r|ä]; p → m after m in alternating stems (ˇ), before a+.nu$, i+m.[a|e|i|o|u|y|ä|ö]$, i+n$, i+t, [o|u]+$, [o|u]+i.$, [o|u]+i..[a|e|i|o|u|y|ä|ö], [o|u]+m.[a|e|i|o|u|y|ä|ö]$, [o|u]+n$, [o|u]+t$, [o|u]+t.e, y+$, y+i.$, y+i..[a|e|i|o|u|y|ä|ö], y+m.[a|e|i|o|u|y|ä|ö]$, y+n$, y+t: 14 words (näkyä, säikkyä, rääkyä, särkyä), 0 forms wrong.
        law11: "{x:[k|p]}" => "{x|law11[l]}" / "{l:[a|i|k|l|m|r|ä]}" _ "{[a]}+{![+|ˇ|¦|°|‹]}{[n]}{[u]}ˇ$" | "{[i]}+{[m]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}ˇ$" | "{[i]}+{[n]}ˇ$" | "{[i]}+{[t]}{![ˇ]*}ˇ$" | "{[o|u]}+ˇ$" | "{[o|u]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[o|u]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[o|u]}+{[m]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}ˇ$" | "{[o|u]}+{[n]}ˇ$" | "{[o|u]}+{[t]}ˇ$" | "{[o|u]}+{[t]}{![+|ˇ|¦|°|‹]}{[e]}{![ˇ]*}ˇ$" | "{[y]}+ˇ$" | "{[y]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[y]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[y]}+{[m]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}ˇ$" | "{[y]}+{[n]}ˇ$" | "{[y]}+{[t]}{![ˇ]*}ˇ$";
        // ∅ → p after p in alternating stems (ˇ), before a+[a|i|s], a+n[a|e|i|o|u|y|ä|ö], e+n[a|e|i|o|u|y|ä|ö], o+a.$, o+a.[a|e|i|o|u|y|ä|ö], o+a.[^a|e|i|o|u|y|ä|ö]i, o+nem[a|e|i|o|u|y|ä|ö], o+ni, o+s, u+a[a|e|i|o|u|y|ä|ö], u+a[^a|e|i|o|u|y|ä|ö][a|e|i|o|u|y|ä|ö], u+s, ä+.$, ä+.[i|m|s|v], ä+.n$, ä+.t$, ä+.t[^a|e|i|o|u|y|ä|ö]: 101 words (hypätä, flopata, teipata, pumpata), 41 forms wrong.
        law12: "" => "p¦" / "{[p]}" _ "{[a]}+{[a|i|s]}{![ˇ]*}ˇ$" | "{[a]}+{[n]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[e]}+{[n]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[o]}+{[a]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[o]}+{[a]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[o]}+{[a]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{[i]}{![ˇ]*}ˇ$" | "{[o]}+{[n]}{[e]}{[m]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[o]}+{[n]}{[i]}{![ˇ]*}ˇ$" | "{[o]}+{[s]}{![ˇ]*}ˇ$" | "{[u]}+{[a]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[u]}+{[a]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[u]}+{[s]}{![ˇ]*}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}{[i|m|s|v]}{![ˇ]*}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}{[t]}ˇ$" | "{[ä]}+{![+|ˇ|¦|°|‹]}{[t]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$";
        // ∅ → t after t in alternating stems (ˇ), before [a|ä]+.$, [a|ä]+.[i|m|s|v], [a|ä]+.n$, [a|ä]+.t$, [a|ä]+.t[^a|e|i|o|u|y|ä|ö], e.+[a|e|i|o|u|y|ä|ö], o+a.$, o+a.[a|e|i|o|u|y|ä|ö], o+a.[^a|e|i|o|u|y|ä|ö]i, o+s, u+a[a|e|i|o|u|y|ä|ö], u+a[^a|e|i|o|u|y|ä|ö][a|e|i|o|u|y|ä|ö], u+s: 569 words (ajatella, kuvitella, homotella, huoritella), 0 forms wrong.
        law13: "" => "t¦" / "{[t]}" _ "{[a|ä]}+{![+|ˇ|¦|°|‹]}ˇ$" | "{[a|ä]}+{![+|ˇ|¦|°|‹]}{[i|m|s|v]}{![ˇ]*}ˇ$" | "{[a|ä]}+{![+|ˇ|¦|°|‹]}{[n]}ˇ$" | "{[a|ä]}+{![+|ˇ|¦|°|‹]}{[t]}ˇ$" | "{[a|ä]}+{![+|ˇ|¦|°|‹]}{[t]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[e]}{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[o]}+{[a]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[o]}+{[a]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[o]}+{[a]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{[i]}{![ˇ]*}ˇ$" | "{[o]}+{[s]}{![ˇ]*}ˇ$" | "{[u]}+{[a]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[u]}+{[a]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[u]}+{[s]}{![ˇ]*}ˇ$";
        // n+ä → n+nä: 8 words (iirintää, filmintää, siintää, tekstintää), 0 forms wrong.
        law14: "n+ä" => "n+nä";
        // p → ∅ after p in alternating stems (ˇ), before +a$, +a[^a|e|i|o|u|y|ä|ö]$, +a[^a|e|i|o|u|y|ä|ö][^a|e|i|o|u|y|ä|ö], +e, i+m.[a|e|i|o|u|y|ä|ö]$, i+n$, i+t, [o|y]+$, [o|y]+t, u+$, u+i.$, u+i..[a|e|i|o|u|y|ä|ö], u+m.[a|e|i|o|u|y|ä|ö]$, u+n$, u+t: 11 words (riippua, loppua, tippua, hoippua), 0 forms wrong.
        law15: "p" => "¦" / "{[p]}" _ "+{[a]}ˇ$" | "+{[a]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}ˇ$" | "+{[a]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "+{[e]}{![ˇ]*}ˇ$" | "{[i]}+{[m]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}ˇ$" | "{[i]}+{[n]}ˇ$" | "{[i]}+{[t]}{![ˇ]*}ˇ$" | "{[o|y]}+ˇ$" | "{[o|y]}+{[t]}{![ˇ]*}ˇ$" | "{[u]}+ˇ$" | "{[u]}+{[i]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[u]}+{[i]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}{![ˇ]*}ˇ$" | "{[u]}+{[m]}{![+|ˇ|¦|°|‹]}{[a|e|i|o|u|y|ä|ö]}ˇ$" | "{[u]}+{[n]}ˇ$" | "{[u]}+{[t]}{![ˇ]*}ˇ$";
        // [in|on]+i → +si: 25 words (iirintää, filmintää, siintää, tekstintää), 0 forms wrong.
        law16: "{x:[in|on]}+i" => "{x}+si";
        // on+a → on+na: 17 words (hienontaa, huonontaa, isontaa, jalontaa), 0 forms wrong.
        law17: "on+a" => "on+na";
        // t+n → t+: 8 words (auttaa, laittaa, maittaa, malttaa), 0 forms wrong.
        law18: "t+n" => "t+";
        // t+s → t+o: 8 words (auttaa, laittaa, maittaa, malttaa), 0 forms wrong.
        law19: "t+s" => "t+o";
        // aa+a → a+kaa: 5 words (maata, taata, väkisinmaata, laata), 0 forms wrong.
        law20: "aa+a" => "a+kaa";
        // aa+i → a+kai: 5 words (maata, taata, väkisinmaata, laata), 0 forms wrong.
        law21: "aa+i" => "a+kai";
        // aa+s → a+kas: 5 words (maata, taata, väkisinmaata, laata), 0 forms wrong.
        law22: "aa+s" => "a+kas";
        // o+a → +koa after [a|i]: 6 words (liota, hiota, laota, saota), 6 forms wrong.
        law23: "o+a" => "+koa" / "{[a|i]}" _ "";
        // o+s → +kos after [a|i]: 6 words (liota, hiota, laota, saota), 0 forms wrong.
        law24: "o+s" => "+kos" / "{[a|i]}" _ "";
        // pi+ → +i before ˇ: 12 words (nyppiä, hyppiä, hörppiä, jelppiä), 0 forms wrong.
        law25: "pi+" => "+i" / "" _ "{[ˇ]}";
    }
};
