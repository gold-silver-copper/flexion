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
        law0 = [("A", "ं") => "न", ("A", "च") => "न", ("A", "ज") => "न", ("A", "ट") => "न", ("A", "ठ") => "न", ("A", "ड") => "न", ("A", "ढ") => "न", ("A", "ण") => "न", ("A", "त") => "न", ("A", "थ") => "न", ("A", "द") => "न", ("A", "ध") => "न", ("A", "न") => "न", ("A", "र") => "ण", ("A", "ल") => "न", ("A", "ळ") => "न", ("A", "व") => "न", ("A", "श") => "न", ("A", "ष") => "ण", ("A", "ह") => "न", ("A", "ु") => "न", ("A", "ृ") => "ण", ("A", "ॄ") => "ण", ("A", "ो") => "न", ("A", "") => "न", ("B", "ं") => "न", ("B", "च") => "न", ("B", "ज") => "न", ("B", "ट") => "न", ("B", "ठ") => "न", ("B", "ड") => "न", ("B", "ढ") => "न", ("B", "ण") => "न", ("B", "त") => "न", ("B", "थ") => "न", ("B", "द") => "न", ("B", "ध") => "न", ("B", "न") => "न", ("B", "र") => "ण", ("B", "ल") => "न", ("B", "ळ") => "न", ("B", "व") => "न", ("B", "श") => "न", ("B", "ष") => "ण", ("B", "ह") => "न", ("B", "ु") => "न", ("B", "ृ") => "ण", ("B", "ॄ") => "न", ("B", "ो") => "न", ("B", "") => "न", ("C", "ं") => "ि", ("C", "च") => "ि", ("C", "ज") => "ि", ("C", "ट") => "ि", ("C", "ठ") => "ि", ("C", "ड") => "ि", ("C", "ढ") => "ि", ("C", "ण") => "ि", ("C", "त") => "ि", ("C", "थ") => "ि", ("C", "द") => "ि", ("C", "ध") => "ि", ("C", "न") => "ि", ("C", "र") => "ि", ("C", "ल") => "ि", ("C", "ळ") => "ि", ("C", "व") => "ि", ("C", "श") => "ि", ("C", "ष") => "ि", ("C", "ह") => "ि", ("C", "ु") => "ि", ("C", "ृ") => "ि", ("C", "ॄ") => "ि", ("C", "ो") => "ि", ("C", "") => "ि"];
    }
    rules {
        // A → न after ं, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ल, ळ, व, श, ह, ु, ो, none; ण after र, ष, ृ, ॄ; B → न after ं, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ल, ळ, व, श, ह, ु, ॄ, ो, none; ण after र, ष, ृ; C → ि after ं, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, र, ल, ळ, व, श, ष, ह, ु, ृ, ॄ, ो, none (the nearest of ं, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, र, ल, ळ, व, श, ष, ह, ु, ृ, ॄ, ो before it, as written): 1,660 words (विश्व, ज, द, कमल), 64 forms wrong.
        law0[iterative]: "{x:[A|B|C]}" => "{x|law0[t]}" / "{t:[ं|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|र|ल|ळ|व|श|ष|ह|ु|ृ|ॄ|ो]?}{g:![ं|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|र|ल|ळ|व|श|ष|ह|ु|ृ|ॄ|ो]*}" _ "";
        // ण → न after ं, अ, आ, ऐ, च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ब, ल, श, स, ै unless र, ष, ृ, ॄ comes between: 8 words (अथर्वाङ्गिरस, सरस, राक्षस, रसिक), 0 forms wrong.
        law1: "ण" => "न" / "{t:[ं|अ|आ|ऐ|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|ब|ल|श|स|ै]}{a:![ं|अ|आ|ऐ|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|ब|ल|श|स|ै|र|ष|ृ|ॄ]*}+{b:![ं|अ|आ|ऐ|च|ज|ट|ठ|ड|ढ|ण|त|थ|द|ध|न|ब|ल|श|स|ै|र|ष|ृ|ॄ]*}" _ "";
    }
};
