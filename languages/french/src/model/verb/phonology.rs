//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        CONSONANT = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z", "ç", "ø", "ɛ", "ɥ", "ʁ"];
        S0 = ["e", "oi", "é"];
        S1 = ["dr", "r", "tr"];
        S2 = ["e", "ou"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}"];
    }
    maps {
        M0 = ["e" => "è", "oi" => "u", "é" => "è"];
        M1 = ["dr" => "s", "r" => "s", "tr" => "ss"];
        M2 = ["e" => "oi", "ou" => "eu"];
    }
    rules {
        across_0: "{x:S0}" => "{x|M0}" / "" _ "{k:CONSONANT*}\u{e000}";
        end_1: "{x:S1}" => "{x|M1}" / "" _ "\u{e001}";
        across_2: "{x:S2}" => "{x|M2}" / "" _ "{k:CONSONANT*}\u{e002}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
