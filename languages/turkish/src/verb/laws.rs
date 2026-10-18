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
        law0 = [("E", " ") => "e", ("E", "a") => "a", ("E", "e") => "e", ("E", "i") => "e", ("E", "o") => "a", ("E", "u") => "a", ("E", "ı") => "a", ("E", "") => "e"];
        law1 = [("I", " ") => "i", ("I", "a") => "ı", ("I", "e") => "i", ("I", "i") => "i", ("I", "o") => "u", ("I", "u") => "u", ("I", "ö") => "ü", ("I", "ü") => "ü", ("I", "ı") => "ı", ("I", "") => "i"];
        law2 = [("A", "o") => "o", ("A", "u") => "a", ("A", "") => "a"];
    }
    rules {
        // E → e after  , e, i, none; a after a, o, u, ı (the nearest of  , a, e, i, o, u, ı before it, as written): 2,368 words (ekmek, oynamak, silmek, ezberlemek), 2,548 forms wrong.
        law0[iterative]: "{x:[E]}" => "{x|law0[t]}" / "{t:[ |a|e|i|o|u|ı]?}{g:![ |a|e|i|o|u|ı]*}" _ "";
        // I → i after  , e, i, none; ı after a, ı; u after o, u; ü after ö, ü (the nearest of  , a, e, i, o, u, ö, ü, ı before it, as written): 2,368 words (ekmek, oynamak, silmek, ezberlemek), 24 forms wrong.
        law1[iterative]: "{x:[I]}" => "{x|law1[t]}" / "{t:[ |a|e|i|o|u|ö|ü|ı]?}{g:![ |a|e|i|o|u|ö|ü|ı]*}" _ "";
        // A → o after o; a after u, none (the nearest of o, u before it, as written): 56 words (pişman olmak, tıraş olmak, ıslık çalmak, kış uykusuna yatmak), 0 forms wrong.
        law2[iterative]: "{x:[A]}" => "{x|law2[t]}" / "{t:[o|u]?}{g:![o|u]*}" _ "";
        // [ş|t|k|p|ç|s]+d → +t: 470 words (ekmek, bitmek, vazgeçmek, yapışmak), 0 forms wrong.
        law3: "{x:[ş|t|k|p|ç|s]}+d" => "{x}+t";
        // [r|l|y|ğ|z|n|v|ı]+tı → +dı: 29 words (yazmak, kaymak, anmak, saymak), 0 forms wrong.
        law4: "{x:[r|l|y|ğ|z|n|v|ı]}+tı" => "{x}+dı";
        // [y|r|ğ|n|m|l|z|v|u]+tu → +du: 34 words (kurmak, donmak, sormak, duymak), 0 forms wrong.
        law5: "{x:[y|r|ğ|n|m|l|z|v|u]}+tu" => "{x}+du";
        // [s|ç|ğ]+ır → +ar: 18 words (basmak, asmak, kısmak, tırsmak), 0 forms wrong.
        law6: "{x:[s|ç|ğ]}+ır" => "{x}+ar";
        // z+ir → z+er: 7 words (çizmek, gezmek, altını çizmek, dizmek), 0 forms wrong.
        law7: "z+ir" => "z+er";
    }
};
