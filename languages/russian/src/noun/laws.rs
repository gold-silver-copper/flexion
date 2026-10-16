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
        // к+ы → к+и: 9,373 words (собака, зонтик, бортник, бредник), 5 forms wrong.
        law0: "к+ы" => "к+и";
        // иц+о → иц+е: 2,564 words (былица, масленица, царица, лисица), 7 forms wrong.
        law1: "иц+о" => "иц+е";
        // г+ы → г+и: 2,035 words (бочаг, гонг, четверг, носорог), 1 forms wrong.
        law2: "г+ы" => "г+и";
        // х+ы → х+и: 837 words (стреха, петух, шлюха, грех), 7 forms wrong.
        law3: "х+ы" => "х+и";
        // ш+ы → ш+и: 446 words (ладоша, чаша, душа, дикуша), 1 forms wrong.
        law4: "ш+ы" => "ш+и";
        // ч+ы → ч+и: 126 words (мульча, тысяча, туча, дача), 1 forms wrong.
        law5: "ч+ы" => "ч+и";
        // [щ|ц|ч|ш|ж|к|г|х]+ям → +ам: 84 words (ночь, помочь, печь, рожь), 6 forms wrong.
        law6: "{x:[щ|ц|ч|ш|ж|к|г|х]}+ям" => "{x}+ам";
        // ш+ой → ш+ей: 440 words (ладоша, чаша, дикуша, папаша), 20 forms wrong.
        law7: "ш+ой" => "ш+ей";
        // ж+ы → ж+и: 82 words (кожа, грыжа, прихожая, мрежа), 2 forms wrong.
        law8: "ж+ы" => "ж+и";
        // [к|щ|ш|ч|ж|г|х]+ых → +их: 28 words (телеведущая, неверующая, составляющая, заведующая), 0 forms wrong.
        law9: "{x:[к|щ|ш|ч|ж|г|х]}+ых" => "{x}+их";
        // щ+ы → щ+и: 101 words (тёща, пища, гуща, роща), 0 forms wrong.
        law10: "щ+ы" => "щ+и";
        // ч+я → ч+а: 46 words (ночь, помочь, печь, речь), 1 forms wrong.
        law11: "ч+я" => "ч+а";
        // ч+ог → ч+ег: 20 words (лесничий, рабочий, ничто, городничий), 0 forms wrong.
        law12: "ч+ог" => "ч+ег";
        // ж+я → ж+а: 22 words (рожь, ложь, вожжи, дрожжи), 0 forms wrong.
        law13: "ж+я" => "ж+а";
        // оч+ом → оч+ем: 5 words (рабочий, чернорабочий, горнорабочий, разнорабочий), 0 forms wrong.
        law14: "оч+ом" => "оч+ем";
        // зж+о → зж+е: 6 words (съезжая, приезжий, заезжий, новоприезжий), 0 forms wrong.
        law15: "зж+о" => "зж+е";
        // ьб+и → ьб+ы: 5 words (судьба, свадьба, усадьба, шмальба), 0 forms wrong.
        law16: "ьб+и" => "ьб+ы";
    }
};
