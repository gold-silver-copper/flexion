//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        CONSONANT = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z", "æ", "ë", "ü", "ŭ"];
        S0 = ["a", "e", "eo", "io", "o"];
        S1 = ["b", "c", "ct", "d", "h", "l", "mb", "mp", "n", "nc", "nd", "ng", "nq", "q", "r", "sc", "t", "tt", "v"];
        S2 = ["b", "c", "ct", "d", "g", "m", "mb", "nd", "ng", "r", "rr", "sc", "t", "v"];
        S3 = ["b", "c", "ct", "d", "g", "mb", "nd", "ng", "r", "sc", "v"];
        S4 = ["b", "c", "ct", "d", "g", "mb", "nd", "ng", "r", "sc", "v"];
        S5 = ["a", "e", "i", "o", "u"];
        S6 = ["b", "c", "d", "g", "r", "sc", "v"];
        S7 = ["b", "c", "d", "g", "r", "v"];
        S8 = ["b", "c", "d", "g", "r", "v"];
        S9 = ["ei", "o", "ī", "ū"];
        S10 = ["b", "c", "ct", "h", "r", "sc", "t"];
        S11 = ["b", "c", "d", "g", "r", "sc", "t"];
        S12 = ["a", "e", "i", "o"];
        S13 = ["b", "c", "d", "r", "t"];
        S14 = ["c", "r", "t"];
        S15 = ["c", "r", "t"];
        S16 = ["c", "r", "t"];
        S17 = ["d", "t"];
        S18 = ["p", "t"];
        S19 = ["m", "p", "t"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}", "\u{e003}", "\u{e004}", "\u{e005}", "\u{e006}", "\u{e007}", "\u{e008}", "\u{e009}", "\u{e00a}", "\u{e00b}", "\u{e00c}", "\u{e00d}", "\u{e00e}", "\u{e00f}", "\u{e010}", "\u{e011}", "\u{e012}", "\u{e013}"];
    }
    maps {
        M0 = ["a" => "ē", "e" => "ē", "eo" => "ē", "io" => "ī", "o" => "ā"];
        M1 = ["b" => "pt", "c" => "xer", "ct" => "x", "d" => "s", "h" => "ct", "l" => "s", "mb" => "bit", "mp" => "pt", "n" => "t", "nc" => "ct", "nd" => "ss", "ng" => "ct", "nq" => "c", "q" => "c", "r" => "ser", "sc" => "t", "t" => "s", "tt" => "ss", "v" => "ct"];
        M2 = ["b" => "pse", "c" => "xe", "ct" => "xe", "d" => "se", "g" => "xe", "m" => "n", "mb" => "bue", "nd" => "de", "ng" => "ge", "r" => "sse", "rr" => "cur", "sc" => "ve", "t" => "n", "v" => "xe"];
        M3 = ["b" => "psi", "c" => "xi", "ct" => "xi", "d" => "si", "g" => "xi", "mb" => "bui", "nd" => "di", "ng" => "gi", "r" => "ssi", "sc" => "vi", "v" => "xi"];
        M4 = ["b" => "psē", "c" => "xē", "ct" => "xē", "d" => "sē", "g" => "xē", "mb" => "buē", "nd" => "dē", "ng" => "gē", "r" => "sē", "sc" => "vē", "v" => "xē"];
        M5 = ["a" => "ē", "e" => "ē", "i" => "ē", "o" => "ō", "u" => "ū"];
        M6 = ["b" => "ss", "c" => "x", "d" => "s", "g" => "s", "r" => "s", "sc" => "xt", "v" => "bu"];
        M7 = ["b" => "ssi", "c" => "xi", "d" => "si", "g" => "si", "r" => "si", "v" => "bui"];
        M8 = ["b" => "ssē", "c" => "xē", "d" => "sē", "g" => "sē", "r" => "sē", "v" => "buē"];
        M9 = ["ei" => "i", "o" => "u", "ī" => "i", "ū" => "u"];
        M10 = ["b" => "psū", "c" => "xēr", "ct" => "xū", "h" => "ctū", "r" => "sēr", "sc" => "tū", "t" => "sū"];
        M11 = ["b" => "p", "c" => "t", "d" => "ss", "g" => "c", "r" => "st", "sc" => "x", "t" => "ss"];
        M12 = ["a" => "ā", "e" => "ē", "i" => "ē", "o" => "ō"];
        M13 = ["b" => "s", "c" => "s", "d" => "s", "r" => "s", "t" => "ss"];
        M14 = ["c" => "si", "r" => "si", "t" => "ssi"];
        M15 = ["c" => "se", "r" => "se", "t" => "sse"];
        M16 = ["c" => "sē", "r" => "sē", "t" => "ssē"];
        M17 = ["d" => "ssū", "t" => "nsū"];
        M18 = ["p" => "p", "t" => "t"];
        M19 = ["m" => "m", "p" => "p", "t" => "t"];
    }
    rules {
        across_0: "{x:S0}" => "{x|M0}" / "" _ "{k:CONSONANT*}\u{e000}";
        end_1: "{x:S1}" => "{x|M1}" / "" _ "\u{e001}";
        end_2: "{x:S2}" => "{x|M2}" / "" _ "\u{e002}";
        end_3: "{x:S3}" => "{x|M3}" / "" _ "\u{e003}";
        end_4: "{x:S4}" => "{x|M4}" / "" _ "\u{e004}";
        across_5: "{x:S5}" => "{x|M5}" / "" _ "{k:CONSONANT*}\u{e005}";
        end_6: "{x:S6}" => "{x|M6}" / "" _ "\u{e006}";
        end_7: "{x:S7}" => "{x|M7}" / "" _ "\u{e007}";
        end_8: "{x:S8}" => "{x|M8}" / "" _ "\u{e008}";
        across_9: "{x:S9}" => "{x|M9}" / "" _ "{k:CONSONANT*}\u{e009}";
        end_10: "{x:S10}" => "{x|M10}" / "" _ "\u{e00a}";
        end_11: "{x:S11}" => "{x|M11}" / "" _ "\u{e00b}";
        across_12: "{x:S12}" => "{x|M12}" / "" _ "{k:CONSONANT*}\u{e00c}";
        end_13: "{x:S13}" => "{x|M13}" / "" _ "\u{e00d}";
        end_14: "{x:S14}" => "{x|M14}" / "" _ "\u{e00e}";
        end_15: "{x:S15}" => "{x|M15}" / "" _ "\u{e00f}";
        end_16: "{x:S16}" => "{x|M16}" / "" _ "\u{e010}";
        end_17: "{x:S17}" => "{x|M17}" / "" _ "\u{e011}";
        start_18: "\u{e012}e{x:S18}" => "{x|M18}e{x}";
        start_19: "\u{e013}o{x:S19}" => "{x|M19}o{x}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
