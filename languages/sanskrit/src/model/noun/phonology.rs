//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        S0 = ["च", "ज", "त", "ध", "न", "भ", "ष", "स", "ा", "ि", "ी", "\u{941}", "\u{942}", "ौ"];
        S1 = ["च", "ज", "द", "ध", "भ", "ष", "ि", "\u{941}", "\u{942}", "\u{943}", "ौ"];
        TRIGGER = ["\u{e000}", "\u{e001}"];
    }
    maps {
        M0 = ["च" => "ग\u{94d}भ", "ज" => "ग\u{94d}भ", "त" => "द\u{94d}भ", "ध" => "द\u{94d}भ", "न" => "भ", "भ" => "ब\u{94d}भ", "ष" => "ड\u{94d}भ", "स" => "ोभ", "ा" => "य", "ि" => "ीन", "ी" => "िय", "\u{941}" => "\u{942}न", "\u{942}" => "\u{941}व", "ौ" => "ाव"];
        M1 = ["च" => "क\u{94d}", "ज" => "क\u{94d}", "द" => "त\u{94d}", "ध" => "त\u{94d}", "भ" => "प\u{94d}", "ष" => "ट\u{94d}", "ि" => "ी", "\u{941}" => "\u{942}", "\u{942}" => "\u{941}", "\u{943}" => "\u{944}", "ौ" => "ा"];
    }
    rules {
        end_0: "{x:S0}" => "{x|M0}" / "" _ "\u{e000}";
        end_1: "{x:S1}" => "{x|M1}" / "" _ "\u{e001}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
