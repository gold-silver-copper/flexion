//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        S0 = ["c", "ch", "g", "k", "n", "sn", "sp", "sł", "t", "w", "z", "zd", "zn", "ć", "ść"];
        S1 = ["c", "ch", "k", "n", "sn", "sp", "t", "w", "z", "zd", "zn", "ć", "ść", "źć"];
        S2 = ["c", "k", "n", "s", "sp", "sł", "t", "z", "ć", "ść", "źć"];
        S3 = ["c", "k", "n", "s", "sn", "t", "z", "zn", "ć", "źć"];
        S4 = ["c", "s", "z", "ść", "źć"];
        S5 = ["c", "ć", "ść", "źć"];
        S6 = ["c", "źć"];
        S7 = ["c", "ść"];
        S8 = ["c", "ść"];
        S9 = ["c", "z"];
        S10 = ["c", "z"];
        TRIGGER = ["\u{e000}", "\u{e001}", "\u{e002}", "\u{e003}", "\u{e004}", "\u{e005}", "\u{e006}", "\u{e007}", "\u{e008}", "\u{e009}", "\u{e00a}"];
    }
    maps {
        M0 = ["c" => "kl", "ch" => "dzi", "g" => "ż", "k" => "cz", "n" => "ły", "sn" => "śli", "sp" => "śpi", "sł" => "śl", "t" => "cz", "w" => "j", "z" => "ż", "zd" => "żdż", "zn" => "źli", "ć" => "l", "ść" => "dl"];
        M1 = ["c" => "kł", "ch" => "dź", "k" => "czą", "n" => "ńci", "sn" => "śni", "sp" => "śpi", "t" => "czą", "w" => "ją", "z" => "żą", "zd" => "żdż", "zn" => "źni", "ć" => "ł", "ść" => "dł", "źć" => "zł"];
        M2 = ["c" => "ć", "k" => "c", "n" => "ń", "s" => "ś", "sp" => "śp", "sł" => "śl", "t" => "c", "z" => "ź", "ć" => "j", "ść" => "d", "źć" => "z"];
        M3 = ["c" => "ż", "k" => "z", "n" => "l", "s" => "śni", "sn" => "śl", "t" => "z", "z" => "źni", "zn" => "źl", "ć" => "j", "źć" => "zi"];
        M4 = ["c" => "k", "s" => "śn", "z" => "źn", "ść" => "d", "źć" => "z"];
        M5 = ["c" => "gn", "ć" => "n", "ść" => "dłb", "źć" => "złb"];
        M6 = ["c" => "żo", "źć" => "zio"];
        M7 = ["c" => "gn", "ść" => "dn"];
        M8 = ["c" => "g", "ść" => "d"];
        M9 = ["c" => "ćci", "z" => "źci"];
        M10 = ["c" => "ćm", "z" => "źm"];
    }
    rules {
        end_0: "{x:S0}" => "{x|M0}" / "" _ "\u{e000}";
        end_1: "{x:S1}" => "{x|M1}" / "" _ "\u{e001}";
        end_2: "{x:S2}" => "{x|M2}" / "" _ "\u{e002}";
        end_3: "{x:S3}" => "{x|M3}" / "" _ "\u{e003}";
        end_4: "{x:S4}" => "{x|M4}" / "" _ "\u{e004}";
        end_5: "{x:S5}" => "{x|M5}" / "" _ "\u{e005}";
        end_6: "{x:S6}" => "{x|M6}" / "" _ "\u{e006}";
        end_7: "{x:S7}" => "{x|M7}" / "" _ "\u{e007}";
        end_8: "{x:S8}" => "{x|M8}" / "" _ "\u{e008}";
        end_9: "{x:S9}" => "{x|M9}" / "" _ "\u{e009}";
        end_10: "{x:S10}" => "{x|M10}" / "" _ "\u{e00a}";
        cleanup: "{t:TRIGGER}" => "";
    }
};

pub(crate) fn program() -> Option<&'static Program> {
    static PROGRAM: OnceLock<Option<Program>> = OnceLock::new();
    PROGRAM.get_or_init(|| LAWS.compile().ok()).as_ref()
}
