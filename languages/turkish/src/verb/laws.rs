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
        law0 = [("E", "a") => "a", ("E", "b") => "e", ("E", "d") => "e", ("E", "g") => "e", ("E", "i") => "e", ("E", "o") => "a", ("E", "u") => "a", ("E", "v") => "e", ("E", "ü") => "e", ("E", "ı") => "a", ("E", "") => "e"];
        law1 = [("I", " ") => "i", ("I", "a") => "ı", ("I", "e") => "i", ("I", "i") => "i", ("I", "o") => "u", ("I", "u") => "u", ("I", "ö") => "ü", ("I", "ü") => "ü", ("I", "ı") => "ı", ("I", "") => "i"];
    }
    rules {
        // E → a after a, o, u, ı; e after b, d, g, i, v, ü, none (the nearest of a, b, d, g, i, o, u, v, ü, ı before it, as written): 2,368 words (ekmek, oynamak, silmek, ezberlemek), 7,470 forms wrong.
        law0[iterative]: "{x:[E]}" => "{x|law0[t]}" / "{t:[a|b|d|g|i|o|u|v|ü|ı]?}{g:![a|b|d|g|i|o|u|v|ü|ı]*}" _ "";
        // I → i after  , e, i, none; ı after a, ı; u after o, u; ü after ö, ü (the nearest of  , a, e, i, o, u, ö, ü, ı before it, as written): 2,368 words (ekmek, oynamak, silmek, ezberlemek), 24 forms wrong.
        law1[iterative]: "{x:[I]}" => "{x|law1[t]}" / "{t:[ |a|e|i|o|u|ö|ü|ı]?}{g:![ |a|e|i|o|u|ö|ü|ı]*}" _ "";
        // [ş|t|k|ç|p|s]+d → +t: 152 words (tüketmek, erişmek, fotoğraf çekmek, ilerletmek), 0 forms wrong.
        law2: "{x:[ş|t|k|ç|p|s]}+d" => "{x}+t";
        // [y|a|r|l|ğ|z|n|ı]+tı → +dı: 79 words (yazmak, almak, acımak, kaymak), 0 forms wrong.
        law3: "{x:[y|a|r|l|ğ|z|n|ı]}+tı" => "{x}+dı";
        // [z|r|n|l|v|y|m|e|ğ|d]+ti → +di: 28 words (silmek, yenmek, binmek, ermek), 0 forms wrong.
        law4: "{x:[z|r|n|l|v|y|m|e|ğ|d]}+ti" => "{x}+di";
        // [y|r|ğ|n|m|l|z|u]+tu → +du: 63 words (kurmak, donmak, sormak, duymak), 0 forms wrong.
        law5: "{x:[y|r|ğ|n|m|l|z|u]}+tu" => "{x}+du";
        // [z|r|y|v|n|l|m]+tü → +dü: 22 words (gülmek, övmek, dönmek, sürmek), 0 forms wrong.
        law6: "{x:[z|r|y|v|n|l|m]}+tü" => "{x}+dü";
        // [a|ı|u]+a → +ya: 13 words (acımak, ağrımak, tanımak, taşımak), 0 forms wrong.
        law7: "{x:[a|ı|u]}+a" => "{x}+ya";
        // [a|ı]+ı → +: 13 words (acımak, ağrımak, tanımak, taşımak), 0 forms wrong.
        law8: "{x:[a|ı]}+ı" => "{x}+";
        // [s|ç|ğ]+ır → +ar: 18 words (basmak, asmak, kısmak, tırsmak), 0 forms wrong.
        law9: "{x:[s|ç|ğ]}+ır" => "{x}+ar";
        // z+ir → z+er: 7 words (çizmek, gezmek, altını çizmek, dizmek), 0 forms wrong.
        law10: "z+ir" => "z+er";
        // en+a → en+e: 5 words (sahnelenmek, yelpazelenmek, lanetlenmek, taharetlenmek), 0 forms wrong.
        law11: "en+a" => "en+e";
        // en+ma → en+me: 5 words (sahnelenmek, yelpazelenmek, lanetlenmek, taharetlenmek), 0 forms wrong.
        law12: "en+ma" => "en+me";
        // en+sa → en+se: 5 words (sahnelenmek, yelpazelenmek, lanetlenmek, taharetlenmek), 0 forms wrong.
        law13: "en+sa" => "en+se";
    }
};
