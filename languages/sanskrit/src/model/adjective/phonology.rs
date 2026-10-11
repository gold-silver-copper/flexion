//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        S0 = ["क", "ज", "त", "न", "श", "स", "हन", "ि", "\u{941}", "\u{943}"];
        S1 = ["ज", "द", "न", "हन", "ि", "िन", "\u{941}"];
        S2 = ["ज", "न", "श", "हन"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}"];
    }
    maps {
        M0 = ["क" => "िका", "ज" => "ग\u{94d}भ", "त" => "द\u{94d}भ", "न" => "भ", "श" => "ग\u{94d}भ", "स" => "ोभ", "हन" => "घ\u{94d}न", "ि" => "ी", "\u{941}" => "\u{942}", "\u{943}" => "\u{944}"];
        M1 = ["ज" => "क\u{94d}", "द" => "त\u{94d}", "न" => "ा", "हन" => "घ\u{94d}", "ि" => "ीन", "िन" => "ी", "\u{941}" => "\u{942}न"];
        M2 = ["ज" => "क", "न" => "ण", "श" => "क", "हन" => "घ\u{94d}न"];
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
