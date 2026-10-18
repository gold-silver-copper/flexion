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
    maps {
        law2 = ["अ" => "आ", "इ" => "ऐ", "उ" => "औ", "उद" => "उदा", "उप" => "उपा", "ऊ" => "औ", "ऋ" => "आर्", "ऋच" => "अर्च", "ओ" => "औ", "सम" => "समा"];
    }
    agreements {
        law0 = [("A", "क") => "न", ("A", "ग") => "न", ("A", "च") => "न", ("A", "ज") => "न", ("A", "ट") => "न", ("A", "ठ") => "न", ("A", "ण") => "न", ("A", "त") => "न", ("A", "द") => "न", ("A", "ध") => "न", ("A", "र") => "ण", ("A", "श") => "न", ("A", "ष") => "ण", ("A", "स") => "न", ("A", "ृ") => "ण", ("A", "") => "न"];
        law1 = [("B", "क") => "ष", ("B", "त") => "स", ("B", "द") => "ध", ("B", "म") => "स", ("B", "र") => "ष", ("B", "व") => "स", ("B", "ष") => "ज", ("B", "स") => "स", ("B", "ा") => "स", ("B", "ि") => "ष", ("B", "ी") => "ष", ("B", "ु") => "ष", ("B", "ृ") => "ज", ("B", "े") => "ष", ("B", "ो") => "ष", ("B", "") => "ज"];
    }
    rules {
        // A → न after क, ग, च, ज, ट, ठ, ण, त, द, ध, श, स, none; ण after र, ष, ृ (the nearest of क, ग, च, ज, ट, ठ, ण, त, द, ध, र, श, ष, स, ृ before it, as written): 642 words (आद, आप, निन्दति, खादति), 16 forms wrong.
        law0[iterative]: "{x:[A]}" => "{x|law0[t]}" / "{t:[क|ग|च|ज|ट|ठ|ण|त|द|ध|र|श|ष|स|ृ]?}{g:![क|ग|च|ज|ट|ठ|ण|त|द|ध|र|श|ष|स|ृ]*}" _ "";
        // B → ष after क, र, ि, ी, ु, े, ो; स after त, म, व, स, ा; ध after द; ज after ष, ृ, none (the nearest of क, त, द, म, र, व, ष, स, ा, ि, ी, ु, ृ, े, ो before it, as written): 139 words (भाति, द्राति, धृष्णोति, जानाति), 0 forms wrong.
        law1[iterative]: "{x:[B]}" => "{x|law1[t]}" / "{t:[क|त|द|म|र|व|ष|स|ा|ि|ी|ु|ृ|े|ो]?}{g:![क|त|द|म|र|व|ष|स|ा|ि|ी|ु|ृ|े|ो]*}" _ "";
        // अ‹अ → आ, इ → ऐ, उ → औ, उद → उदा, उप → उपा, ऊ → औ, ऋ → आर्, ऋच → अर्च, ओ → औ, सम → समा: 34 words (इच्छति, ओषति, अस्ति, अत्ति), 0 forms wrong.
        law2: "^अ‹{x:[अ|इ|उ|उद|उप|ऊ|ऋ|ऋच|ओ|सम]}" => "{x|law2}";
        // [ा|े|ो|ू|ए]+ा → +: 6 words (एति, अश्रोत्, अभूत्, अगात्), 0 forms wrong.
        law3: "{x:[ा|े|ो|ू|ए]}+ा" => "{x}+";
        // [ष|स]+ीष → +: 5 words (अताप्सीत्, अभार्षीत्, अजैषीत्, अवाक्षीत्), 0 forms wrong.
        law4: "{x:[ष|स]}+ीष" => "{x}+";
    }
};
