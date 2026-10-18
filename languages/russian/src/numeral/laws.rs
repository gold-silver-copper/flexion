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
        law0 = [("Ы", "е") => "ы", ("Ы", "о") => "и", ("Ы", "") => "е"];
    }
    rules {
        // Ы → ы after е; и after о; е after none (the nearest of е, о before it, as written): 12 words (два, три, тысяча, двое), 0 forms wrong.
        law0[iterative]: "{x:[Ы]}" => "{x|law0[t]}" / "{t:[е|о]?}{g:![е|о]*}" _ "";
        // ь+ом → ь+ю: 19 words (шесть, пять, семь, восемь), 0 forms wrong.
        law1: "ь+ом" => "ь+ю";
        // ь+а → +и: 18 words (шесть, пять, семь, девять), 0 forms wrong.
        law2: "ь+а" => "+и";
        // ь+е → +и: 18 words (шесть, пять, семь, девять), 0 forms wrong.
        law3: "ь+е" => "+и";
        // ь+у → +и: 18 words (шесть, пять, семь, девять), 0 forms wrong.
        law4: "ь+у" => "+и";
    }
};
