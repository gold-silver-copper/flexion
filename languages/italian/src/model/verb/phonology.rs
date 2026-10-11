//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        CONSONANT = ["b", "c", "d", "f", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z"];
        S0 = ["c", "d", "g", "gg", "gn", "l", "m", "n", "nd", "r", "rr", "rs", "s", "sc", "t", "tt", "v"];
        S1 = ["a", "e", "ia", "o", "uo"];
        S2 = ["c", "d", "g", "gg", "l", "n", "nd", "r", "rr", "sc"];
        S3 = ["c", "d", "gg", "gn", "l", "n", "r", "rr", "v"];
        S4 = ["c", "gg", "l", "n", "v"];
        S5 = ["l", "n"];
        S6 = ["l", "t", "v"];
        S7 = ["l", "v"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}", "\u{e003}", "\u{e004}", "\u{e005}", "\u{e006}", "\u{e007}"];
    }
    maps {
        M0 = ["c" => "s", "d" => "s", "g" => "s", "gg" => "ss", "gn" => "ns", "l" => "s", "m" => "ns", "n" => "s", "nd" => "s", "r" => "s", "rr" => "nes", "rs" => "c", "s" => "r", "sc" => "bb", "t" => "ss", "tt" => "ss", "v" => "ss"];
        M1 = ["a" => "e", "e" => "ie", "ia" => "e", "o" => "uo", "uo" => "o"];
        M2 = ["c" => "se", "d" => "se", "g" => "se", "gg" => "sse", "l" => "rr", "n" => "rr", "nd" => "se", "r" => "c", "rr" => "n", "sc" => "bbe"];
        M3 = ["c" => "rre", "d" => "g", "gg" => "rre", "gn" => "n", "l" => "rre", "n" => "rre", "r" => "v", "rr" => "n", "v" => "rre"];
        M4 = ["c" => "rr", "gg" => "rr", "l" => "rr", "n" => "rr", "v" => "rr"];
        M5 = ["l" => "r", "n" => "r"];
        M6 = ["l" => "gli", "t" => "ssi", "v" => "bbi"];
        M7 = ["l" => "gl", "v" => "bb"];
    }
    rules {
        end_0: "{x:S0}" => "{x|M0}" / "" _ "\u{e000}";
        across_1: "{x:S1}" => "{x|M1}" / "" _ "{k:CONSONANT*}\u{e001}";
        end_2: "{x:S2}" => "{x|M2}" / "" _ "\u{e002}";
        end_3: "{x:S3}" => "{x|M3}" / "" _ "\u{e003}";
        end_4: "{x:S4}" => "{x|M4}" / "" _ "\u{e004}";
        end_5: "{x:S5}" => "{x|M5}" / "" _ "\u{e005}";
        end_6: "{x:S6}" => "{x|M6}" / "" _ "\u{e006}";
        end_7: "{x:S7}" => "{x|M7}" / "" _ "\u{e007}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
