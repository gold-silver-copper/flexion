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
    rules {
        // e+e → e+: 265 words (orange, safe, lose, woke), 0 forms wrong.
        law0: "e+e" => "e+";
        // s+s → s+es: 240 words (bourgeois, mies, lass, divers), 0 forms wrong.
        law1: "s+s" => "s+es";
        // nt+s → nt+es: 150 words (aberrant, flatulent, abundant, pedant), 0 forms wrong.
        law2: "nt+s" => "nt+es";
        // ft+s → ft+es: 134 words (schmackhaft, lebhaft, sanft, alptraumhaft), 0 forms wrong.
        law3: "ft+s" => "ft+es";
        // ht+s → ht+es: 75 words (echt, recht, licht, gerecht), 49 forms wrong.
        law4: "ht+s" => "ht+es";
        // st+s → st+es: 69 words (feist, lost, manifest, trist), 0 forms wrong.
        law5: "st+s" => "st+es";
        // kt+s → kt+es: 47 words (nackt, abstrakt, verrückt, geschickt), 0 forms wrong.
        law6: "kt+s" => "kt+es";
        // u+s → u+es: 36 words (blau, neu, mau, grau), 0 forms wrong.
        law7: "u+s" => "u+es";
        // zt+s → zt+es: 22 words (verwanzt, gekürzt, ungeschützt, geschützt), 0 forms wrong.
        law8: "zt+s" => "zt+es";
        // mt+s → mt+es: 20 words (unbestimmt, gehemmt, berühmt, verklemmt), 0 forms wrong.
        law9: "mt+s" => "mt+es";
        // bt+s → bt+es: 19 words (geliebt, versnobt, begabt, beliebt), 0 forms wrong.
        law10: "bt+s" => "bt+es";
        // ß+s → ß+es: 18 words (bloß, weiß, süß, heiß), 0 forms wrong.
        law11: "ß+s" => "ß+es";
        // ut+s → ut+es: 16 words (absolut, laut, akut, resolut), 0 forms wrong.
        law12: "ut+s" => "ut+es";
        // it+s → it+es: 13 words (breit, weit, bereit, gescheit), 0 forms wrong.
        law13: "it+s" => "it+es";
        // id+s → id+es: 12 words (arid, solid, frigid, amyloid), 0 forms wrong.
        law14: "id+s" => "id+es";
        // at+s → at+es: 11 words (privat, delikat, desolat, moderat), 0 forms wrong.
        law15: "at+s" => "at+es";
        // ei+s → ei+es: 10 words (frei, wertfrei, unfrei, störungsfrei), 0 forms wrong.
        law16: "ei+s" => "ei+es";
        // dt+s → dt+es: 9 words (verwandt, redegewandt, beredt, gewandt), 0 forms wrong.
        law17: "dt+s" => "dt+es";
        // sk+s → sk+es: 8 words (pittoresk, brüsk, grotesk, burlesk), 0 forms wrong.
        law18: "sk+s" => "sk+es";
        // x+s → x+es: 8 words (complex, unorthodox, orthodox, fix), 0 forms wrong.
        law19: "x+s" => "x+es";
        // z+s → z+es: 8 words (schwarz, spitz, stolz, nachtschwarz), 0 forms wrong.
        law20: "z+s" => "z+es";
        // md+s → md+es: 7 words (fremd, ortsfremd, praxisfremd, realitätsfremd), 0 forms wrong.
        law21: "md+s" => "md+es";
        // pt+s → pt+es: 7 words (abrupt, prompt, korrupt, verkappt), 0 forms wrong.
        law22: "pt+s" => "pt+es";
        // äh+s → ä+chs after a vowel earlier in the stem: 6 words (körpernah, zeitnah, citynah, äquatornah), 2 forms wrong.
        law23: "äh+s" => "ä+chs" / "{[a|e|i|o|u|y|ä|ö|ü]}{![a|e|i|o|u|y|ä|ö|ü]*}" _ "";
        // w+s → w+es: 6 words (new, blaw, blauw, trew), 0 forms wrong.
        law24: "w+s" => "w+es";
        // lt+s → lt+es after [a|h|i|l|o|u|ä|‹]: 20 words (kalt, alt, gewillt, okkult), 0 forms wrong.
        law25: "lt+s" => "lt+es" / "{[a|h|i|l|o|u|ä|‹]}" _ "";
    }
};
