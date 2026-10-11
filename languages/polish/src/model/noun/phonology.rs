//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        CONSONANT = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z", "á", "ä", "å", "é", "ë", "ï", "ö", "ü", "ć", "ł", "ń", "ō", "ś", "š", "ź", "ż"];
        S0 = ["c", "g", "k", "n", "s", "st", "sł", "t", "x", "z", "zd", "ć", "ł", "ń", "ś", "ź"];
        S1 = ["c", "ch", "n", "s", "st", "t", "z", "ć", "ł", "ń", "ś", "ź"];
        S2 = ["ó", "ą"];
        S3 = ["s", "z"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}", "\u{e003}"];
    }
    maps {
        M0 = ["c" => "ć", "g" => "dz", "k" => "c", "n" => "ń", "s" => "ś", "st" => "ści", "sł" => "śl", "t" => "ci", "x" => "ks", "z" => "ź", "zd" => "źdz", "ć" => "ci", "ł" => "l", "ń" => "ni", "ś" => "si", "ź" => "zi"];
        M1 = ["c" => "ćc", "ch" => "s", "n" => "ńc", "s" => "śc", "st" => "śc", "t" => "c", "z" => "źc", "ć" => "c", "ł" => "l", "ń" => "n", "ś" => "s", "ź" => "z"];
        M2 = ["ó" => "o", "ą" => "ę"];
        M3 = ["s" => "śn", "z" => "n"];
    }
    rules {
        end_0: "{x:S0}" => "{x|M0}" / "" _ "\u{e000}";
        end_1: "{x:S1}" => "{x|M1}" / "" _ "\u{e001}";
        across_2: "{x:S2}" => "{x|M2}" / "" _ "{k:CONSONANT*}\u{e002}";
        end_3: "{x:S3}" => "{x|M3}" / "" _ "\u{e003}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
