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
        // [к|г|х|ш|ч|ж|щ]+ы → +и: 12,996 words (собака, зонтик, бортник, бредник), 17 forms wrong.
        law0: "{x:[к|г|х|ш|ч|ж|щ]}+ы" => "{x}+и";
        // иц+о → иц+е: 2,563 words (былица, масленица, царица, лисица), 6 forms wrong.
        law1: "иц+о" => "иц+е";
        // [щ|ц|ч|ш|ж|к|г|х]+ям → +ам: 84 words (ночь, помочь, печь, рожь), 6 forms wrong.
        law2: "{x:[щ|ц|ч|ш|ж|к|г|х]}+ям" => "{x}+ам";
        // ш+ой → ш+ей: 440 words (ладоша, чаша, дикуша, папаша), 20 forms wrong.
        law3: "ш+ой" => "ш+ей";
        // [к|щ|ш|ч|ж|г|х]+ых → +их: 28 words (телеведущая, неверующая, составляющая, заведующая), 0 forms wrong.
        law4: "{x:[к|щ|ш|ч|ж|г|х]}+ых" => "{x}+их";
        // [ч|ж]+я → +а: 68 words (ночь, помочь, печь, речь), 1 forms wrong.
        law5: "{x:[ч|ж]}+я" => "{x}+а";
        // ч+ог → ч+ег: 20 words (лесничий, рабочий, ничто, городничий), 0 forms wrong.
        law6: "ч+ог" => "ч+ег";
        // оч+ом → оч+ем: 5 words (рабочий, чернорабочий, горнорабочий, разнорабочий), 0 forms wrong.
        law7: "оч+ом" => "оч+ем";
        // зж+о → зж+е: 6 words (съезжая, приезжий, заезжий, новоприезжий), 0 forms wrong.
        law8: "зж+о" => "зж+е";
        // ьб+и → ьб+ы: 5 words (судьба, свадьба, усадьба, шмальба), 0 forms wrong.
        law9: "ьб+и" => "ьб+ы";
    }
};
