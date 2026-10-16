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
        // ē → e after [c|d|i|l|m|p|r|t|u] in alternating stems (ˇ), before +.., .+..: 186 words (quadrupēs, volucripēs, recēns, clēmēns), 2 forms wrong.
        law0: "ē" => "e¦" / "{[c|d|i|l|m|p|r|t|u]}" _ "+{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$" | "{![+|ˇ|¦|°|‹]}+{![+|ˇ|¦|°|‹]}{![+|ˇ|¦|°|‹]}{![ˇ]*}ˇ$";
        // ēn+t → +ent: 13 words (ēns, libēns, lubēns, ingēns), 0 forms wrong.
        law1: "ēn+t" => "+ent";
        // x+i → +ci: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 6 forms wrong.
        law2: "x+i" => "+ci";
        // x+ē → +cē: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 0 forms wrong.
        law3: "x+ē" => "+cē";
        // x+em → +cem: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 0 forms wrong.
        law4: "x+em" => "+cem";
        // ān+t → +ant: 53 words (īnfāns, ignicomāns, ēlegāns, auricomāns), 0 forms wrong.
        law5: "ān+t" => "+ant";
        // e+c → +ic: 34 words (complex, simplex, duplex, multiplex), 29 forms wrong.
        law6: "e+c" => "+ic";
        // ōn+t → +ont: 14 words (sōns, īnsōns, latifrōns, nigrifrōns), 0 forms wrong.
        law7: "ōn+t" => "+ont";
        // p+t → p+: 5 words (inops, cōps, mīcrōps, macrōps), 0 forms wrong.
        law8: "p+t" => "p+";
        // x+ī → +cī before ˇ: 5 words (praecox, bilanx, sēminex, hystrix), 0 forms wrong.
        law9: "x+ī" => "+cī" / "" _ "{[ˇ]}";
        // x+īs → +cīs before ˇ: 5 words (praecox, bilanx, sēminex, hystrix), 0 forms wrong.
        law10: "x+īs" => "+cīs" / "" _ "{[ˇ]}";
    }
};
