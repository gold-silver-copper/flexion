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
        // T → that letter; t after none (the nearest of d, g, m, n, t before it, as written): 1,557 words (cat, abet, abut, het), 0 forms wrong.
        law0[iterative]: "{x:[T]}" => "{x|law0[t]}" / "{t:[d|g|m|n|t]?}{g:![d|g|m|n|t]*}" _ "";
        // S → that letter; s after none (the nearest of s, x, z before it, as written): 16,368 words (free, portmanteau, cat, word), 281 forms wrong.
        law1[iterative]: "{x:[S]}" => "{x|law1[t]}" / "{t:[s|x|z]?}{g:![s|x|z]*}" _ "";
        // t+d → t+: 100 words (forecast, cast, broadcast, burst), 0 forms wrong.
        law2: "t+d" => "t+";
        // b+p → b+b: 25 words (dub, dab, drib, drab), 0 forms wrong.
        law3: "b+p" => "b+b";
        // b+t → b+b: 103 words (ab, lob, bib, stab), 0 forms wrong.
        law4: "b+t" => "b+b";
        // l+p → l+l: 71 words (control, appel, halal, compel), 0 forms wrong.
        law5: "l+p" => "l+l";
        // p+n → p+p: 62 words (nap, nip, kneecap, snap), 0 forms wrong.
        law6: "p+n" => "p+p";
        // nd+d → n+t: 54 words (send, bend, lend, spend), 0 forms wrong.
        law7: "nd+d" => "n+t";
        // p+d → p+p: 54 words (handicap, drop, dip, snowdrop), 0 forms wrong.
        law8: "p+d" => "p+p";
        // p+m → p+p: 34 words (map, mop, membership, mishap), 0 forms wrong.
        law9: "p+m" => "p+p";
        // [c|k]+t → +k: 26 words (metric, yak, traffic, kek), 0 forms wrong.
        law10: "{x:[c|k]}+t" => "{x}+k";
        // b+n → b+b: 27 words (nab, hobnob, nib, nob), 0 forms wrong.
        law11: "b+n" => "b+b";
        // b+g → b+b: 21 words (gab, grub, grab, gib), 0 forms wrong.
        law12: "b+g" => "b+b";
        // [v|s|z]+te → +e: 20 words (arise, rise, strive, uprise), 0 forms wrong.
        law13: "{x:[v|s|z]}+te" => "{x}+e";
        // iv+d → iv+: 14 words (drive, overdrive, wardrive, outdrive), 0 forms wrong.
        law14: "iv+d" => "iv+";
        // er+n → er+r: 13 words (infer, transfer, confer, cotransfer), 0 forms wrong.
        law15: "er+n" => "er+r";
        // ld+d → l+t: 13 words (build, rebuild, jerrybuild, overbuild), 0 forms wrong.
        law16: "ld+d" => "l+t";
        // [c|k]+m → +k: 11 words (music, mimic, tarmac, mosaic), 0 forms wrong.
        law17: "{x:[c|k]}+m" => "{x}+k";
        // l+n → l+l: 11 words (canal, annul, enrol, disannul), 0 forms wrong.
        law18: "l+n" => "l+l";
        // [n|al]+d → +t: 18 words (born, uplean, mean, bemean), 0 forms wrong.
        law19: "{x:[n|al]}+d" => "{x}+t";
        // ur+n → ur+r: 10 words (incur, concur, unblur, encur), 0 forms wrong.
        law20: "ur+n" => "ur+r";
        // ay+ne → a+i: 9 words (gainsay, unlay, inlay, naysay), 0 forms wrong.
        law21: "ay+ne" => "a+i";
        // y+n → y+: 9 words (gainsay, unlay, inlay, naysay), 0 forms wrong.
        law22: "y+n" => "y+";
        // ar+n → ar+r: 7 words (gnar, unbar, snar, unspar), 0 forms wrong.
        law23: "ar+n" => "ar+r";
        // f+t → f+f: 7 words (ref, chef, crossref, coif), 0 forms wrong.
        law24: "f+t" => "f+f";
        // v+p → v+v: 7 words (shiv, rev, nav, improv), 0 forms wrong.
        law25: "v+p" => "v+v";
        // b+m → b+b: 6 words (demob, mob, mab, simulpub), 0 forms wrong.
        law26: "b+m" => "b+b";
        // [c|k]+g → +k: 6 words (grok, logic, magic, garlic), 0 forms wrong.
        law27: "{x:[c|k]}+g" => "{x}+k";
        // [c|k]+n → +k: 5 words (panic, tonic, picnic, nuk), 0 forms wrong.
        law28: "{x:[c|k]}+n" => "{x}+k";
        // ll+d → l+t: 5 words (dwell, misspell, foresmell, outdwell), 0 forms wrong.
        law29: "ll+d" => "l+t";
    }
};
