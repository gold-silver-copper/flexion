//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        CONSONANT = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z", "â", "ä", "ç", "ê", "î", "ñ"];
        S0 = ["a", "o"];
        TRIGGER = ["\u{e000}"];
    }
    maps {
        M0 = ["a" => "á", "o" => "ó"];
    }
    rules {
        across_0: "{x:S0}" => "{x|M0}" / "" _ "{k:CONSONANT*}\u{e000}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
