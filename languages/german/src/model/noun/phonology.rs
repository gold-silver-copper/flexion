//! The sound laws: edits carry trigger characters that set them off;
//! the last rule deletes any trigger left.

use flexion::sound_law::{Program, RuleSet, sound_laws};
use std::sync::OnceLock;

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        CONSONANT = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z", "ß", "à", "á", "â", "ã", "å", "æ", "ç", "è", "é", "ê", "ë", "í", "ï", "ñ", "ó", "ô", "õ", "ø", "ú", "û", "ć", "č", "ē", "ī", "ł", "ō", "œ", "ş", "ū", "ǃ", "α", "β", "γ"];
        S0 = ["a", "au", "o", "oo", "u"];
        TRIGGER = ["\u{e000}"];
    }
    maps {
        M0 = ["a" => "ä", "au" => "äu", "o" => "ö", "oo" => "ö", "u" => "ü"];
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
