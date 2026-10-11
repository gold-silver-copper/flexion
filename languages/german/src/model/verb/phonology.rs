//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        CONSONANT = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z", "ß", "è", "é"];
        S0 = ["au", "e", "ei", "i", "ie", "o", "u", "ö", "ü"];
        S1 = ["b", "f", "g", "h", "l", "r", "s"];
        S2 = ["au", "ei"];
        S3 = ["b", "g"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}", "\u{e003}"];
    }
    maps {
        M0 = ["au" => "ie", "e" => "ä", "ei" => "ie", "i" => "ä", "ie" => "ö", "o" => "ie", "u" => "ie", "ö" => "ü", "ü" => "ö"];
        M1 = ["b" => "zub", "f" => "zuf", "g" => "zug", "h" => "zuh", "l" => "zul", "r" => "zur", "s" => "zus"];
        M2 = ["au" => "ö", "ei" => "i"];
        M3 = ["b" => "tt", "g" => "cht"];
    }
    rules {
        across_0: "{x:S0}" => "{x|M0}" / "" _ "{k:CONSONANT*}\u{e000}";
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
