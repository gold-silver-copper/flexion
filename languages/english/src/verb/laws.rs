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
        law0 = [("T", "d") => "d", ("T", "g") => "g", ("T", "m") => "m", ("T", "n") => "n", ("T", "t") => "t", ("T", "") => "t"];
        law1 = [("S", "s") => "s", ("S", "x") => "x", ("S", "z") => "z", ("S", "") => "s"];
    }
    rules {
        // T → that letter; t after none (the letter just before it, if one of d, g, m, n, t): 1,607 words (cat, abet, abut, het), 0 forms wrong.
        law0[iterative]: "{x:[T]}" => "{x|law0[t]}" / "{t:[d|g|m|n|t]?}{g:[+|ˇ|¦|°|‹]*}" _ "";
        // S → that letter; s after none (the letter just before it, if one of s, x, z): 16,457 words (free, portmanteau, cat, word), 9 forms wrong.
        law1[iterative]: "{x:[S]}" => "{x|law1[t]}" / "{t:[s|x|z]?}{g:[+|ˇ|¦|°|‹]*}" _ "";
        // t+d → t+: 100 words (forecast, cast, broadcast, burst), 0 forms wrong.
        law2: "t+d" => "t+";
        // b+t → b+b: 182 words (ab, lob, dub, bib), 0 forms wrong.
        law3: "b+t" => "b+b";
        // l+p → l+l: 88 words (canal, control, appel, halal), 0 forms wrong.
        law4: "l+p" => "l+l";
        // [v|s|z|u|i]+te → +e: 35 words (arise, drive, rise, strive), 0 forms wrong.
        law5: "{x:[v|s|z|u|i]}+te" => "{x}+e";
        // [c|k]+d → +ked: 34 words (metric, music, hydraulic, traffic), 2 forms wrong.
        law6: "{x:[c|k]}+d" => "{x}+ked";
        // [c|k]+t → +k: 52 words (metric, grok, music, hydraulic), 0 forms wrong.
        law7: "{x:[c|k]}+t" => "{x}+k";
        // nd+d → n+t: 54 words (send, bend, lend, spend), 0 forms wrong.
        law8: "nd+d" => "n+t";
        // ld+d → l+t: 13 words (build, rebuild, jerrybuild, overbuild), 0 forms wrong.
        law9: "ld+d" => "l+t";
        // n+d → n+t: 11 words (born, uplean, mean, bemean), 0 forms wrong.
        law10: "n+d" => "n+t";
        // f+t → f+f: 10 words (ref, chef, crossref, coif), 0 forms wrong.
        law11: "f+t" => "f+f";
        // v+p → v+v: 7 words (shiv, rev, nav, improv), 0 forms wrong.
        law12: "v+p" => "v+v";
        // ll+d → l+t: 5 words (dwell, misspell, foresmell, outdwell), 0 forms wrong.
        law13: "ll+d" => "l+t";
        // p+d → p+t: 5 words (lowp, outleap, forthleap, beleap), 0 forms wrong.
        law14: "p+d" => "p+t";
        // ll+ → +old after [s|t]: 23 words (foretell, outsell, oversell, resell), 0 forms wrong.
        law15: "ll+" => "+old" / "{[s|t]}" _ "";
    }
};
