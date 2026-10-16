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
        law0 = [("s", "d") => "s", ("s", "e") => "¦", ("s", "g") => "s", ("s", "l") => "s", ("s", "m") => "s", ("s", "n") => "s", ("s", "p") => "¦", ("s", "ā") => "¦", ("s", "ē") => "¦", ("s", "ō") => "¦", ("s", "ū") => "¦", ("us", "d") => "e¦", ("us", "e") => "us", ("us", "g") => "us", ("us", "l") => "e¦", ("us", "m") => "e¦", ("us", "n") => "e¦", ("us", "p") => "us", ("us", "ā") => "us", ("us", "ē") => "us", ("us", "ō") => "us", ("us", "ū") => "us", ("ō", "d") => "i¦", ("ō", "e") => "ō", ("ō", "g") => "i¦", ("ō", "l") => "ō", ("ō", "m") => "ō", ("ō", "n") => "ō", ("ō", "p") => "ō", ("ō", "ā") => "ō", ("ō", "ē") => "ō", ("ō", "ō") => "ō", ("ō", "ū") => "ō"];
    }
    rules {
        // s → ∅ after [e|p|ā|ē|ō|ū]; ō → i after [d|g]; us → e after [d|l|m|n] in alternating stems (ˇ), before +[e|i|n|r|t|u|ē]: 862 words (ōs, genus, onus, cōs), 0 forms wrong.
        law0: "{x:[s|us|ō]}" => "{x|law0[l]}" / "{l:[d|e|g|l|m|n|p|ā|ē|ō|ū]}" _ "+{[e|i|n|r|t|u|ē]}{![ˇ]*}ˇ$";
        // s+t → +t: 7 words (erysipelas, mārs, rinoceros, quirīs), 0 forms wrong.
        law1: "s+t" => "+t";
        // x+e → +ce: 328 words (bōx, lynx, hallux, generātrīx), 18 forms wrong.
        law2: "x+e" => "+ce";
        // x+ē → +cē: 326 words (bōx, lynx, hallux, generātrīx), 36 forms wrong.
        law3: "x+ē" => "+cē";
        // i+i → i+: 15 words (axis, anguis, orexis, unguis), 0 forms wrong.
        law4: "i+i" => "i+";
        // [r|n|l|x]+n → +: 124 words (sōl, martyr, ēr, massētēr), 10 forms wrong.
        law5: "{x:[r|n|l|x]}+n" => "{x}+";
        // a+e → a+: 25 words (addax, anthrax, manna, victuma), 0 forms wrong.
        law6: "a+e" => "a+";
        // [p|b|ō|m|y]+tē → +ē: 25 words (urbs, cyclōps, grȳps, thrīps), 2 forms wrong.
        law7: "{x:[p|b|ō|m|y]}+tē" => "{x}+ē";
        // ēn+t → +ent: 29 words (lēns, dēns, mēns, gēns), 0 forms wrong.
        law8: "ēn+t" => "+ent";
        // us+r → +er: 5 words (opus, latus, ulcus, vīscus), 0 forms wrong.
        law9: "us+r" => "+er";
        // p+t → p+: 20 words (cyclōps, grȳps, thrīps, ops), 0 forms wrong.
        law10: "p+t" => "p+";
        // ān+t → +ant: 16 words (accūsāns, īnfāns, elephāns, quadrāns), 0 forms wrong.
        law11: "ān+t" => "+ant";
        // iī+ → i+: 11 words (aquāriī, eximiī, stadiī, rōrāriī), 0 forms wrong.
        law12: "iī+" => "i+";
        // xi+ē → x+ē: 9 words (axis, orexis, syntaxis, rīxis), 0 forms wrong.
        law13: "xi+ē" => "x+ē";
        // [n|r|t]+b → +ib: 7 words (vetera, tormina, arōmata, posteriōra), 0 forms wrong.
        law14: "{x:[n|r|t]}+b" => "{x}+ib";
        // ui+e → u+e: 6 words (anguis, unguis, ninguis, torquis), 0 forms wrong.
        law15: "ui+e" => "u+e";
        // ui+ē → u+ē: 6 words (anguis, unguis, ninguis, torquis), 0 forms wrong.
        law16: "ui+ē" => "u+ē";
        // b+t → b+: 5 words (urbs, plēbs, arabs, chalybs), 0 forms wrong.
        law17: "b+t" => "b+";
        // in+c → in+g: 5 words (sphinx, mēninx, sȳrinx, salpinx), 0 forms wrong.
        law18: "in+c" => "in+g";
        // xi+e → x+e: 5 words (axis, orexis, rīxis, rhēxis), 0 forms wrong.
        law19: "xi+e" => "x+e";
    }
};
