//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        CONSONANT = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z", "ç", "ñ"];
        S0 = ["a", "ai", "au", "e", "ei", "eu", "eí", "i", "o", "u", "ui"];
        S1 = ["c", "z"];
        S2 = ["c", "z"];
        S3 = ["e", "eí", "i", "o"];
        S4 = ["e", "i", "o", "u"];
        S5 = ["c", "g"];
        S6 = ["c", "g"];
        S7 = ["e", "o"];
        S8 = ["c", "g"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}", "\u{e003}", "\u{e004}", "\u{e005}", "\u{e006}", "\u{e007}", "\u{e008}"];
    }
    maps {
        M0 = ["a" => "á", "ai" => "ái", "au" => "áu", "e" => "é", "ei" => "éi", "eu" => "éu", "eí" => "ié", "i" => "í", "o" => "ó", "u" => "ú", "ui" => "úi"];
        M1 = ["c" => "qu", "z" => "c"];
        M2 = ["c" => "qué", "z" => "cé"];
        M3 = ["e" => "ie", "eí" => "ie", "i" => "e", "o" => "ue"];
        M4 = ["e" => "ie", "i" => "í", "o" => "ue", "u" => "ú"];
        M5 = ["c" => "zc", "g" => "j"];
        M6 = ["c" => "zcá", "g" => "já"];
        M7 = ["e" => "i", "o" => "u"];
        M8 = ["c" => "zca", "g" => "ja"];
    }
    rules {
        across_0: "{x:S0}" => "{x|M0}" / "" _ "{k:CONSONANT*}\u{e000}";
        end_1: "{x:S1}" => "{x|M1}" / "" _ "\u{e001}";
        end_2: "{x:S2}" => "{x|M2}" / "" _ "\u{e002}";
        across_3: "{x:S3}" => "{x|M3}" / "" _ "{k:CONSONANT*}\u{e003}";
        across_4: "{x:S4}" => "{x|M4}" / "" _ "{k:CONSONANT*}\u{e004}";
        end_5: "{x:S5}" => "{x|M5}" / "" _ "\u{e005}";
        end_6: "{x:S6}" => "{x|M6}" / "" _ "\u{e006}";
        across_7: "{x:S7}" => "{x|M7}" / "" _ "{k:CONSONANT*}\u{e007}";
        end_8: "{x:S8}" => "{x|M8}" / "" _ "\u{e008}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
