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
        law0 = [("A", "ं") => "न", ("A", "च") => "न", ("A", "ज") => "न", ("A", "ट") => "न", ("A", "ठ") => "न", ("A", "ड") => "न", ("A", "ढ") => "न", ("A", "ण") => "न", ("A", "त") => "न", ("A", "थ") => "न", ("A", "द") => "न", ("A", "ध") => "न", ("A", "न") => "न", ("A", "र") => "ण", ("A", "ल") => "न", ("A", "ळ") => "न", ("A", "श") => "न", ("A", "ष") => "ण", ("A", "स") => "न", ("A", "ृ") => "ण", ("A", "ॄ") => "ण", ("A", "") => "न"];
        law1 = [("D", "क") => "ि", ("D", "ख") => "ि", ("D", "ण") => "ि", ("D", "न") => "ि", ("D", "म") => "ि", ("D", "य") => "ि", ("D", "र") => "ि", ("D", "ल") => "ि", ("D", "श") => "ि", ("D", "ा") => "ि", ("D", "ो") => "ि", ("D", "") => "ि"];
    }
    rules {
        // A → न after ं, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ल, ळ, श, स, none; ण after र, ष, ृ, ॄ (the nearest of ं, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, र, ल, ळ, श, ष, स, ृ, ॄ before it, as written): 1,863 words (विश्व, ज, द, कमल), 0 forms wrong.
        law0[iterative]: "{x:[A]}" => "{x|law0[t]}" / "{t:[ं|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|र|ल|ळ|श|ष|स|ृ|ॄ]?}{g:![ं|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|र|ल|ळ|श|ष|स|ृ|ॄ]*}" _ "";
        // D → ि after क, ख, ण, न, म, य, र, ल, श, ा, ो, none (the nearest of क, ख, ण, न, म, य, र, ल, श, ा, ो before it, as written): 466 words (स्निह्, भारत, हस्तिन्, देव), 0 forms wrong.
        law1[iterative]: "{x:[D]}" => "{x|law1[t]}" / "{t:[क|ख|ण|न|म|य|र|ल|श|ा|ो]?}{g:![क|ख|ण|न|म|य|र|ल|श|ा|ो]*}" _ "";
    }
};
