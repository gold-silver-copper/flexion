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
        law4 = [("o", "c") => "ue¦", ("o", "d") => "ue¦", ("o", "f") => "ue¦", ("o", "g") => "üe¦", ("o", "l") => "ue¦", ("o", "m") => "ue¦", ("o", "p") => "ue¦", ("o", "r") => "ue¦", ("o", "s") => "ue¦", ("o", "t") => "ue¦", ("o", "v") => "ue¦"];
    }
    rules {
        // g+e → g+ue: 586 words (llegar, delegar, fatigar, cabalgar), 8 forms wrong.
        law0: "g+e" => "g+ue";
        // g+é → g+ué: 585 words (llegar, delegar, fatigar, cabalgar), 0 forms wrong.
        law1: "g+é" => "g+ué";
        // ic+e → i+que: 347 words (ubicar, justificar, indicar, identificar), 0 forms wrong.
        law2: "ic+e" => "i+que";
        // ic+é → i+qué: 347 words (ubicar, justificar, indicar, identificar), 0 forms wrong.
        law3: "ic+é" => "i+qué";
        // o → ue after [c|d|f|l|m|p|r|s|t|v]; o → üe after g in alternating stems (ˇ), before +.a$, +.a[n|s]$, +.o, b+[a|e|i|o|u|í|ü]$, b+[a|e|i|o|u|í|ü][n|s]$, b.+[a|e|i|o|u|í|ü]$, b.+[a|e|i|o|u|í|ü][n|s]$, [d|ñ]+[a|e|i|o|u|í|ü]$, l+a$, l+a[n|s]$, l+e$, l+e[n|s], l+o, l[d|l]+[a|e|i|o|u|í|ü]$, l[d|l]+[a|e|i|o|u|í|ü][n|s]$, lt+[a|e|i|o|u|í|ü]$, [n|s]+[a|e|i|o|u|í|ü]$, [n|s]+[a|e|i|o|u|í|ü][n|s]$, [n|s]t+[a|e|i|o|u|í|ü]$, [n|s]t+[a|e|i|o|u|í|ü][n|s]$, r+[a|e|i|o|u|í|ü]$, r+[^a|e|i|o|u|í|ü]a$, r+[^a|e|i|o|u|í|ü]a[n|s]$, r+[^a|e|i|o|u|í|ü]o, rd+a$, rd+a[n|s]$, rd+e$, rd+e[n|s], rd+o, rm+[a|e], rn+[a|e|i|o|u|í|ü]$, rn+[a|e|i|o|u|í|ü][n|s]$, rt+[a|e|i|o|u|í|ü]$, v+a$, v+a.$, v+e$: 88 words (sonar, volar, llover, doler), 10 forms wrong.
        law4: "{x:[o]}" => "{x|law4[l]}" / "{l:[c|d|f|g|l|m|p|r|s|t|v]}" _ "+{![+|ˇ|¦|°|‹]}{[a]}ˇ$" | "+{![+|ˇ|¦|°|‹]}{[a]}{[n|s]}ˇ$" | "+{![+|ˇ|¦|°|‹]}{[o]}{![ˇ]*}ˇ$" | "{[b]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[b]}+{[a|e|i|o|u|í|ü]}{[n|s]}ˇ$" | "{[b]}{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[b]}{![+|ˇ|¦|°|‹]}+{[a|e|i|o|u|í|ü]}{[n|s]}ˇ$" | "{[d|ñ]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[l]}+{[a]}ˇ$" | "{[l]}+{[a]}{[n|s]}ˇ$" | "{[l]}+{[e]}ˇ$" | "{[l]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{[l]}+{[o]}{![ˇ]*}ˇ$" | "{[l]}{[d|l]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[l]}{[d|l]}+{[a|e|i|o|u|í|ü]}{[n|s]}ˇ$" | "{[l]}{[t]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[n|s]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[n|s]}+{[a|e|i|o|u|í|ü]}{[n|s]}ˇ$" | "{[n|s]}{[t]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[n|s]}{[t]}+{[a|e|i|o|u|í|ü]}{[n|s]}ˇ$" | "{[r]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[r]}+{![+|ˇ|¦|°|‹|a|e|i|o|u|í|ü]}{[a]}ˇ$" | "{[r]}+{![+|ˇ|¦|°|‹|a|e|i|o|u|í|ü]}{[a]}{[n|s]}ˇ$" | "{[r]}+{![+|ˇ|¦|°|‹|a|e|i|o|u|í|ü]}{[o]}{![ˇ]*}ˇ$" | "{[r]}{[d]}+{[a]}ˇ$" | "{[r]}{[d]}+{[a]}{[n|s]}ˇ$" | "{[r]}{[d]}+{[e]}ˇ$" | "{[r]}{[d]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{[r]}{[d]}+{[o]}{![ˇ]*}ˇ$" | "{[r]}{[m]}+{[a|e]}{![ˇ]*}ˇ$" | "{[r]}{[n]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[r]}{[n]}+{[a|e|i|o|u|í|ü]}{[n|s]}ˇ$" | "{[r]}{[t]}+{[a|e|i|o|u|í|ü]}ˇ$" | "{[v]}+{[a]}ˇ$" | "{[v]}+{[a]}{![+|ˇ|¦|°|‹]}ˇ$" | "{[v]}+{[e]}ˇ$";
        // nc+e → n+que: 104 words (roncar, ahincar, hincar, brincar), 4 forms wrong.
        law5: "nc+e" => "n+que";
        // nc+é → n+qué: 104 words (roncar, ahincar, hincar, brincar), 0 forms wrong.
        law6: "nc+é" => "n+qué";
        // rc+e → r+que: 70 words (embarcar, marcar, acercar, recercar), 0 forms wrong.
        law7: "rc+e" => "r+que";
        // rc+é → r+qué: 70 words (embarcar, marcar, acercar, recercar), 0 forms wrong.
        law8: "rc+é" => "r+qué";
        // [ñ|b|l]+ya → +a: 43 words (bullir, plañir, mullir, muñir), 0 forms wrong.
        law9: "{x:[ñ|b|l]}+ya" => "{x}+a";
        // [ñ|b|l]+yá → +á: 43 words (bullir, plañir, mullir, muñir), 0 forms wrong.
        law10: "{x:[ñ|b|l]}+yá" => "{x}+á";
        // uc+é → u+qué: 38 words (educar, caducar, machucar, acurrucar), 0 forms wrong.
        law11: "uc+é" => "u+qué";
        // [ñ|l|y|i]+yé → +é: 43 words (bullir, plañir, mullir, muñir), 0 forms wrong.
        law12: "{x:[ñ|l|y|i]}+yé" => "{x}+é";
        // gu+é → g+üé: 47 words (averiguar, atestiguar, menguar, aguar), 0 forms wrong.
        law13: "gu+é" => "g+üé";
        // zd+ → +d: 37 words (tender, perder, encender, defender), 0 forms wrong.
        law14: "zd+" => "+d";
        // ñ+y → ñ+: 26 words (atañer, plañir, muñir, gruñir), 0 forms wrong.
        law15: "ñ+y" => "ñ+";
        // l+y → l+: 18 words (bullir, mullir, zabullir, çabullir), 0 forms wrong.
        law16: "l+y" => "l+";
        // lc+e → l+que: 23 words (calcar, revolcar, volcar, remolcar), 0 forms wrong.
        law17: "lc+e" => "l+que";
        // lc+é → l+qué: 23 words (calcar, revolcar, volcar, remolcar), 0 forms wrong.
        law18: "lc+é" => "l+qué";
        // zc+e → z+que: 12 words (pellizcar, pizcar, bizcar, peñizcar), 0 forms wrong.
        law19: "zc+e" => "z+que";
        // zc+é → z+qué: 12 words (pellizcar, pizcar, bizcar, peñizcar), 0 forms wrong.
        law20: "zc+é" => "z+qué";
        // zt+ → +t: 11 words (verter, entrometerse, entremeterse, cometerse), 0 forms wrong.
        law21: "zt+" => "+t";
        // c+e → +que before [p|ˇ]: 187 words (caber, buscar, rascar, tocar), 0 forms wrong.
        law22: "c+e" => "+que" / "" _ "{[p|ˇ]}";
        // eg+ue → +iegue before ˇ: 25 words (negar, renegar, fregar, regar), 0 forms wrong.
        law23: "eg+ue" => "+iegue" / "" _ "{[ˇ]}";
    }
};
