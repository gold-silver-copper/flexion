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
        // e+e → e+: 3,430 words (abominable, abrogable, acceptable, libre), 4 forms wrong.
        law0: "e+e" => "e+";
        // a+e → a+: 1,512 words (panista, foxista, plata, simba), 0 forms wrong.
        law1: "a+e" => "a+";
        // és+e → +ese: 262 words (japonés, luxemburgués, inglés, francés), 0 forms wrong.
        law2: "és+e" => "+ese";
        // és+a → +esa: 260 words (japonés, luxemburgués, inglés, francés), 0 forms wrong.
        law3: "és+a" => "+esa";
        // ón+e → +one: 250 words (chingón, juguetón, cabrón, llorón), 0 forms wrong.
        law4: "ón+e" => "+one";
        // ón+a → +ona: 247 words (chingón, juguetón, cabrón, llorón), 0 forms wrong.
        law5: "ón+a" => "+ona";
        // s+s → s+: 5 words (ambos, exprés, entrambos, antiestrés), 0 forms wrong.
        law6: "s+s" => "s+";
        // i+e → i+: 88 words (yanqui, nazi, mini, yuki), 0 forms wrong.
        law7: "i+e" => "i+";
        // z+e → +ce: 48 words (rapaz, fugaz, andaluz, feliz), 0 forms wrong.
        law8: "z+e" => "+ce";
        // án+e → +ane: 38 words (catalán, musulmán, alemán, charlatán), 0 forms wrong.
        law9: "án+e" => "+ane";
        // án+a → +ana: 37 words (catalán, musulmán, alemán, charlatán), 0 forms wrong.
        law10: "án+a" => "+ana";
        // ín+e → +ine: 17 words (bailarín, mandarín, saltarín, afín), 0 forms wrong.
        law11: "ín+e" => "+ine";
        // ao+a → +á: 14 words (culiao, pringao, aweonao, ahueonao), 0 forms wrong.
        law12: "ao+a" => "+á";
        // é+e → é+: 14 words (touché, calé, paré, culé), 1 forms wrong.
        law13: "é+e" => "é+";
        // ún+e → +une: 12 words (común, pastún, euscaldún, euskaldún), 0 forms wrong.
        law14: "ún+e" => "+une";
        // x+es → x+: 9 words (unisex, redox, sioux, réflex), 0 forms wrong.
        law15: "x+es" => "x+";
        // h+s → h+: 6 words (touch, kitsch, british, amish), 0 forms wrong.
        law16: "h+s" => "h+";
        // ó+e → ó+: 5 words (bordó, rococó, sirionó, guató), 0 forms wrong.
        law17: "ó+e" => "ó+";
        // ún+a → +una: 5 words (pastún, euscaldún, euskaldún, pashtún), 0 forms wrong.
        law18: "ún+a" => "+una";
    }
};
