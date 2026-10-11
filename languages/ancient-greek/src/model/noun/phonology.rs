//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        S0 = ["ξ", "ρ", "ς", "ψ"];
        S1 = ["ν", "ς"];
        S2 = ["ξ", "ς", "ψ"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}"];
    }
    maps {
        M0 = ["ξ" => "κ", "ρ" => "τ", "ς" => "δ", "ψ" => "π"];
        M1 = ["ν" => "σ", "ς" => "σ"];
        M2 = ["ξ" => "κ", "ς" => "τ", "ψ" => "π"];
    }
    rules {
        end_0: "{x:S0}" => "{x|M0}" / "" _ "\u{e000}";
        end_1: "{x:S1}" => "{x|M1}" / "" _ "\u{e001}";
        end_2: "{x:S2}" => "{x|M2}" / "" _ "\u{e002}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
