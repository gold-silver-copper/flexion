//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        S0 = ["ν", "ς"];
        TRIGGER = ["\u{e000}"];
    }
    maps {
        M0 = ["ν" => "σ", "ς" => "σ"];
    }
    rules {
        end_0: "{x:S0}" => "{x|M0}" / "" _ "\u{e000}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
