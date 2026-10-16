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
        // i+i → i+: 1,614 words (odiare, mangiare, studiare, lasciare), 288 forms wrong.
        law0: "i+i" => "i+";
        // ic+e → ic+he: 429 words (pizzicare, nevicare, zoppicare, abdicare), 2 forms wrong.
        law1: "ic+e" => "ic+he";
        // ic+i → ic+hi: 429 words (pizzicare, nevicare, zoppicare, abdicare), 0 forms wrong.
        law2: "ic+i" => "ic+hi";
        // gi+e → g+e: 398 words (mangiare, viaggiare, contagiare, plagiare), 0 forms wrong.
        law3: "gi+e" => "g+e";
        // ci+e → c+e: 361 words (lasciare, abbracciare, lanciare, baciare), 12 forms wrong.
        law4: "ci+e" => "c+e";
        // g+in → g+hin: 40 words (rogare, interrogare, affogare, derogare), 0 forms wrong.
        law5: "g+in" => "g+hin";
        // [d|g|v|r|m|t|l|n]+gg → +: 14 words (valere, vivere, sfriggere, contravvalere), 0 forms wrong.
        law6: "{x:[d|g|v|r|m|t|l|n]}+gg" => "{x}+";
        // g+r → g+er: 14 words (tangere, convergere, contingere, mingere), 0 forms wrong.
        law7: "g+r" => "g+er";
        // g+d → +s: 7 words (convergere, mingere, fungere, defungere), 0 forms wrong.
        law8: "g+d" => "+s";
        // eg+i → eg+hi: 55 words (pregare, piegare, negare, segare), 0 forms wrong.
        law9: "eg+i" => "eg+hi";
        // g+s → +ss: 10 words (affiggere, crocefiggere, crocifiggere, crucifiggere), 0 forms wrong.
        law10: "g+s" => "+ss";
        // og+e → og+he: 40 words (rogare, interrogare, affogare, derogare), 12 forms wrong.
        law11: "og+e" => "og+he";
        // og+i → og+hi: 40 words (rogare, interrogare, affogare, derogare), 0 forms wrong.
        law12: "og+i" => "og+hi";
        // ec+e → ec+he after a vowel earlier in the stem: 21 words (defecare, deprecare, arrecare, accecare), 12 forms wrong.
        law13: "ec+e" => "ec+he" / "{[a|e|i|o|u]}{![a|e|i|o|u]*}" _ "";
        // ec+i → ec+hi: 26 words (defecare, deprecare, arrecare, accecare), 1 forms wrong.
        law14: "ec+i" => "ec+hi";
        // [l|n]+ss → +s: 8 words (valere, volvere, contravvalere, disvalere), 0 forms wrong.
        law15: "{x:[l|n]}+ss" => "{x}+s";
        // on+s → +us: 20 words (fondere, confondere, circonfondere, diffondere), 0 forms wrong.
        law16: "on+s" => "+us";
        // ug+i → ug+hi: 19 words (asciugare, coniugare, coniugarsi, asciugarsi), 0 forms wrong.
        law17: "ug+i" => "ug+hi";
        // uo+s → +os: 9 words (muovere, percuotere, scuotere, commuovere), 0 forms wrong.
        law18: "uo+s" => "+os";
        // gl+en → gl+ien: 6 words (inorgoglire, inorgoglirsi, incanaglirsi, orgoglire), 0 forms wrong.
        law19: "gl+en" => "gl+ien";
        // al+r → a+r: 6 words (valere, contravvalere, disvalere, equivalere), 0 forms wrong.
        law20: "al+r" => "a+r";
    }
};
