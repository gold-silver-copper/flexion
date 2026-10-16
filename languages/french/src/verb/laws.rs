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
    rules {
        // é → è after [b|c|d|f|g|h|i|l|m|n|p|r|s|t] in alternating stems (ˇ), before [c|q].+e$, [c|q].+e[n|s], [c|q].+er., [d|l|n|r|s]+e$, [d|l|n|r|s]+e[n|s], [d|l|n|r|s]+er., g+e$, g+e[n|s], g.+e$, g.+e[n|s], g.+er., m+e$, m+e.., t+e$, t+e[n|s], t+er., t.+e$, t.+e[n|s], t.+er., v.+e$, v.+e..: 227 words (pécher, accélérer, espérer, péter), 0 forms wrong.
        law0: "é" => "è¦" / "{[b|c|d|f|g|h|i|l|m|n|p|r|s|t]}" _ "{[c|q]}{![+|ˇ|¦|°|‹]}+{[e]}ˇ$" | "{[c|q]}{![+|ˇ|¦|°|‹]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{[c|q]}{![+|ˇ|¦|°|‹]}+{[e]}{[r]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[d|l|n|r|s]}+{[e]}ˇ$" | "{[d|l|n|r|s]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{[d|l|n|r|s]}+{[e]}{[r]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[g]}+{[e]}ˇ$" | "{[g]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{[g]}{![+|ˇ|¦|°|‹]}+{[e]}ˇ$" | "{[g]}{![+|ˇ|¦|°|‹]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{[g]}{![+|ˇ|¦|°|‹]}+{[e]}{[r]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[m]}+{[e]}ˇ$" | "{[m]}+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[t]}+{[e]}ˇ$" | "{[t]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{[t]}+{[e]}{[r]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[t]}{![+|ˇ|¦|°|‹]}+{[e]}ˇ$" | "{[t]}{![+|ˇ|¦|°|‹]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{[t]}{![+|ˇ|¦|°|‹]}+{[e]}{[r]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{[v]}{![+|ˇ|¦|°|‹]}+{[e]}ˇ$" | "{[v]}{![+|ˇ|¦|°|‹]}+{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // g+a → g+ea: 194 words (badger, voyager, manager, singer), 39 forms wrong.
        law1: "g+a" => "g+ea";
        // g+â → g+eâ: 194 words (badger, voyager, manager, singer), 9 forms wrong.
        law2: "g+â" => "g+eâ";
        // ∅ → e after g in alternating stems (ˇ), before +.[^a|e|i|o|u|y|â|æ|è|é|ê|ë|î|ï|ô|û|ü|œ]s: 193 words (badger, voyager, manager, singer), 0 forms wrong.
        law3: "" => "e¦" / "{[g]}" _ "+{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹|a|e|i|o|u|y|â|æ|è|é|ê|ë|î|ï|ô|û|ü|œ]}{[s]}{![ˇ]*}ˇ$";
        // c+o → +ço: 131 words (tracer, placer, agacer, forcer), 0 forms wrong.
        law4: "c+o" => "+ço";
        // c+a → +ça: 124 words (tracer, placer, agacer, forcer), 0 forms wrong.
        law5: "c+a" => "+ça";
        // c+â → +çâ: 124 words (tracer, placer, agacer, forcer), 0 forms wrong.
        law6: "c+â" => "+çâ";
        // e → è after [c|g|h|l|m|p|r|s|t|u|v] in alternating stems (ˇ), before +.e$, +.e.., .+e$, .+e[n|s], .+er.: 84 words (lever, élever, peser, acheter), 21 forms wrong.
        law7: "e" => "è¦" / "{[c|g|h|l|m|p|r|s|t|u|v]}" _ "+{![+|ˇ|¦|°|‹]}{[e]}ˇ$" | "+{![+|ˇ|¦|°|‹]}{[e]}{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[e]}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[e]}{[n|s]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{[e]}{[r]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // ri+sa → ri+va: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
        law8: "ri+sa" => "ri+va";
        // ri+se → ri+ve: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
        law9: "ri+se" => "ri+ve";
        // ri+si → ri+vi: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
        law10: "ri+si" => "ri+vi";
        // ri+so → ri+vo: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
        law11: "ri+so" => "ri+vo";
        // ri+sî → ri+vî: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
        law12: "ri+sî" => "ri+vî";
        // rt+s → r+s: 5 words (partir, sortir, repartir, ressortir), 0 forms wrong.
        law13: "rt+s" => "r+s";
        // rt+u → rt+i: 5 words (partir, sortir, repartir, ressortir), 0 forms wrong.
        law14: "rt+u" => "rt+i";
    }
};
