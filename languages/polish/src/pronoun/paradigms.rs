//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative_masculine` (anim, inan)
/// `nominative_feminine` (anim, inan)
/// `nominative_neuter` (anim, inan)
/// `nominative_virile` (anim, inan)
/// `nominative_nonvirile` (anim, inan)
/// `genitive_masculine` (anim, inan)
/// `genitive_feminine` (anim, inan)
/// `genitive_neuter` (anim, inan)
/// `genitive_virile` (anim, inan)
/// `genitive_nonvirile` (anim, inan)
/// `dative_masculine` (anim, inan)
/// `dative_feminine` (anim, inan)
/// `dative_neuter` (anim, inan)
/// `dative_virile` (anim, inan)
/// `dative_nonvirile` (anim, inan)
/// `accusative_masculine` (anim, inan)
/// `accusative_feminine` (anim, inan)
/// `accusative_neuter` (anim, inan)
/// `accusative_virile` (anim, inan)
/// `accusative_nonvirile` (anim, inan)
/// `instrumental_masculine` (anim, inan)
/// `instrumental_feminine` (anim, inan)
/// `instrumental_neuter` (anim, inan)
/// `instrumental_virile` (anim, inan)
/// `instrumental_nonvirile` (anim, inan)
/// `locative_masculine` (anim, inan)
/// `locative_feminine` (anim, inan)
/// `locative_neuter` (anim, inan)
/// `locative_virile` (anim, inan)
/// `locative_nonvirile` (anim, inan)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative_masculine: $c0:tt, $c1:tt;
     nominative_feminine: $c2:tt, $c3:tt;
     nominative_neuter: $c4:tt, $c5:tt;
     nominative_virile: $c6:tt, $c7:tt;
     nominative_nonvirile: $c8:tt, $c9:tt;
     genitive_masculine: $c10:tt, $c11:tt;
     genitive_feminine: $c12:tt, $c13:tt;
     genitive_neuter: $c14:tt, $c15:tt;
     genitive_virile: $c16:tt, $c17:tt;
     genitive_nonvirile: $c18:tt, $c19:tt;
     dative_masculine: $c20:tt, $c21:tt;
     dative_feminine: $c22:tt, $c23:tt;
     dative_neuter: $c24:tt, $c25:tt;
     dative_virile: $c26:tt, $c27:tt;
     dative_nonvirile: $c28:tt, $c29:tt;
     accusative_masculine: $c30:tt, $c31:tt;
     accusative_feminine: $c32:tt, $c33:tt;
     accusative_neuter: $c34:tt, $c35:tt;
     accusative_virile: $c36:tt, $c37:tt;
     accusative_nonvirile: $c38:tt, $c39:tt;
     instrumental_masculine: $c40:tt, $c41:tt;
     instrumental_feminine: $c42:tt, $c43:tt;
     instrumental_neuter: $c44:tt, $c45:tt;
     instrumental_virile: $c46:tt, $c47:tt;
     instrumental_nonvirile: $c48:tt, $c49:tt;
     locative_masculine: $c50:tt, $c51:tt;
     locative_feminine: $c52:tt, $c53:tt;
     locative_neuter: $c54:tt, $c55:tt;
     locative_virile: $c56:tt, $c57:tt;
     locative_nonvirile: $c58:tt, $c59:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
                flexion::form!($c48),
                flexion::form!($c49),
                flexion::form!($c50),
                flexion::form!($c51),
                flexion::form!($c52),
                flexion::form!($c53),
                flexion::form!($c54),
                flexion::form!($c55),
                flexion::form!($c56),
                flexion::form!($c57),
                flexion::form!($c58),
                flexion::form!($c59),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `każdy`: 6 words.
    Każdy,
    /// Like `taki`: 3 words.
    Taki,
    /// Like `mój`: 3 words.
    Mój,
    /// Like `nasz`: 2 words.
    Nasz,
    /// Like `czyj`: 2 words.
    Czyj,
    /// Like `czyjś`: 2 words.
    Czyjś,
    /// Like `on`: 1 words.
    On,
    /// Like `ile`: 1 words.
    Ile,
    /// Like `ki`: 1 words.
    Ki,
    /// Like `tamten`: 1 words.
    Tamten,
    /// Like `pewien`: 1 words.
    Pewien,
    /// Like `jakikolwiek`: 1 words.
    Jakikolwiek,
    /// Like `którykolwiek`: 1 words.
    Którykolwiek,
    /// Like `ów`: 1 words.
    Ów,
    /// Like `niejeden`: 1 words.
    Niejeden,
    /// Like `ilekolwiek`: 1 words.
    Ilekolwiek,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 16] = [
        Declension::Każdy,
        Declension::Taki,
        Declension::Mój,
        Declension::Nasz,
        Declension::Czyj,
        Declension::Czyjś,
        Declension::On,
        Declension::Ile,
        Declension::Ki,
        Declension::Tamten,
        Declension::Pewien,
        Declension::Jakikolwiek,
        Declension::Którykolwiek,
        Declension::Ów,
        Declension::Niejeden,
        Declension::Ilekolwiek,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        DECLENSIONS.get(self as usize)
    }
}

impl flexion::Label for Declension {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static DECLENSIONS: [Paradigm<'static>; 16] = [
    // Każdy: 6 words (który, każdy, niektóry, owy, takowy, poniektóry).
    declension!("każdy", 6, "~y";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~zy", "~zy";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~y";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Taki: 3 words (taki, sraki, każdziutki).
    declension!("taki", 3, "~ki";
        nominative_masculine: "~ki", "~ki";
        nominative_feminine: "~ka", "~ka";
        nominative_neuter: "~kie", "~kie";
        nominative_virile: "~cy", "~cy";
        nominative_nonvirile: "~kie", "~kie";
        genitive_masculine: "~kiego", "~kiego";
        genitive_feminine: "~kiej", "~kiej";
        genitive_neuter: "~kiego", "~kiego";
        genitive_virile: "~kich", "~kich";
        genitive_nonvirile: "~kich", "~kich";
        dative_masculine: "~kiemu", "~kiemu";
        dative_feminine: "~kiej", "~kiej";
        dative_neuter: "~kiemu", "~kiemu";
        dative_virile: "~kim", "~kim";
        dative_nonvirile: "~kim", "~kim";
        accusative_masculine: "~kiego", "~ki";
        accusative_feminine: "~ką", "~ką";
        accusative_neuter: "~kie", "~kie";
        accusative_virile: "~kich", "~kich";
        accusative_nonvirile: "~kie", "~kie";
        instrumental_masculine: "~kim", "~kim";
        instrumental_feminine: "~ką", "~ką";
        instrumental_neuter: "~kim", "~kim";
        instrumental_virile: "~kimi", "~kimi";
        instrumental_nonvirile: "~kimi", "~kimi";
        locative_masculine: "~kim", "~kim";
        locative_feminine: "~kiej", "~kiej";
        locative_neuter: "~kim", "~kim";
        locative_virile: "~kich", "~kich";
        locative_nonvirile: "~kich", "~kich";
    ),
    // Mój: 3 words (mój, twój, swój).
    declension!("mój", 3, "~ój";
        nominative_masculine: "~ój", "~ój";
        nominative_feminine: "~oja", "~oja";
        nominative_neuter: "~oje", "~oje";
        nominative_virile: "~oi", "~oi";
        nominative_nonvirile: "~oje", "~oje";
        genitive_masculine: "~ojego", "~ojego";
        genitive_feminine: "~ojej", "~ojej";
        genitive_neuter: "~ojego", "~ojego";
        genitive_virile: "~oich", "~oich";
        genitive_nonvirile: "~oich", "~oich";
        dative_masculine: "~ojemu", "~ojemu";
        dative_feminine: "~ojej", "~ojej";
        dative_neuter: "~ojemu", "~ojemu";
        dative_virile: "~oim", "~oim";
        dative_nonvirile: "~oim", "~oim";
        accusative_masculine: "~ojego", "~ój";
        accusative_feminine: "~oją", "~oją";
        accusative_neuter: "~oje", "~oje";
        accusative_virile: "~oich", "~oich";
        accusative_nonvirile: "~oje", "~oje";
        instrumental_masculine: "~oim", "~oim";
        instrumental_feminine: "~oją", "~oją";
        instrumental_neuter: "~oim", "~oim";
        instrumental_virile: "~oimi", "~oimi";
        instrumental_nonvirile: "~oimi", "~oimi";
        locative_masculine: "~oim", "~oim";
        locative_feminine: "~ojej", "~ojej";
        locative_neuter: "~oim", "~oim";
        locative_virile: "~oich", "~oich";
        locative_nonvirile: "~oich", "~oich";
    ),
    // Nasz: 2 words (nasz, wasz).
    declension!("nasz", 2, "~z";
        nominative_masculine: "~z", "~z";
        nominative_feminine: "~za", "~za";
        nominative_neuter: "~ze", "~ze";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~ze", "~ze";
        genitive_masculine: "~zego", "~zego";
        genitive_feminine: "~zej", "~zej";
        genitive_neuter: "~zego", "~zego";
        genitive_virile: "~zych", "~zych";
        genitive_nonvirile: "~zych", "~zych";
        dative_masculine: "~zemu", "~zemu";
        dative_feminine: "~zej", "~zej";
        dative_neuter: "~zemu", "~zemu";
        dative_virile: "~zym", "~zym";
        dative_nonvirile: "~zym", "~zym";
        accusative_masculine: "~zego", "~z";
        accusative_feminine: "~zą", "~zą";
        accusative_neuter: "~ze", "~ze";
        accusative_virile: "~zych", "~zych";
        accusative_nonvirile: "~ze", "~ze";
        instrumental_masculine: "~zym", "~zym";
        instrumental_feminine: "~zą", "~zą";
        instrumental_neuter: "~zym", "~zym";
        instrumental_virile: "~zymi", "~zymi";
        instrumental_nonvirile: "~zymi", "~zymi";
        locative_masculine: "~zym", "~zym";
        locative_feminine: "~zej", "~zej";
        locative_neuter: "~zym", "~zym";
        locative_virile: "~zych", "~zych";
        locative_nonvirile: "~zych", "~zych";
    ),
    // Czyj: 2 words (czyj, niczyj).
    declension!("czyj", 2, "~j";
        nominative_masculine: "~j", "~j";
        nominative_feminine: "~ja", "~ja";
        nominative_neuter: "~je", "~je";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~je", "~je";
        genitive_masculine: "~jego", "~jego";
        genitive_feminine: "~jej", "~jej";
        genitive_neuter: "~jego", "~jego";
        genitive_virile: "~ich", "~ich";
        genitive_nonvirile: "~ich", "~ich";
        dative_masculine: "~jemu", "~jemu";
        dative_feminine: "~jej", "~jej";
        dative_neuter: "~jemu", "~jemu";
        dative_virile: "~im", "~im";
        dative_nonvirile: "~im", "~im";
        accusative_masculine: "~jego", "~j";
        accusative_feminine: "~ją", "~ją";
        accusative_neuter: "~je", "~je";
        accusative_virile: "~ich", "~ich";
        accusative_nonvirile: "~je", "~je";
        instrumental_masculine: "~im", "~im";
        instrumental_feminine: "~ją", "~ją";
        instrumental_neuter: "~im", "~im";
        instrumental_virile: "~imi", "~imi";
        instrumental_nonvirile: "~imi", "~imi";
        locative_masculine: "~im", "~im";
        locative_feminine: "~jej", "~jej";
        locative_neuter: "~im", "~im";
        locative_virile: "~ich", "~ich";
        locative_nonvirile: "~ich", "~ich";
    ),
    // Czyjś: 2 words (czyjś, czyjkolwiek).
    declension!("czyjś", 2, "~j~";
        nominative_masculine: "~j~", "~j~";
        nominative_feminine: "~ja~", "~ja~";
        nominative_neuter: "~je~", "~je~";
        nominative_virile: "~i~", "~i~";
        nominative_nonvirile: "~je~", "~je~";
        genitive_masculine: "~jego~", "~jego~";
        genitive_feminine: "~jej~", "~jej~";
        genitive_neuter: "~jego~", "~jego~";
        genitive_virile: "~ich~", "~ich~";
        genitive_nonvirile: "~ich~", "~ich~";
        dative_masculine: "~jemu~", "~jemu~";
        dative_feminine: "~jej~", "~jej~";
        dative_neuter: "~jemu~", "~jemu~";
        dative_virile: "~im~", "~im~";
        dative_nonvirile: "~im~", "~im~";
        accusative_masculine: "~jego~", "~j~";
        accusative_feminine: "~ją~", "~ją~";
        accusative_neuter: "~je~", "~je~";
        accusative_virile: "~ich~", "~ich~";
        accusative_nonvirile: "~je~", "~je~";
        instrumental_masculine: "~im~", "~im~";
        instrumental_feminine: "~ją~", "~ją~";
        instrumental_neuter: "~im~", "~im~";
        instrumental_virile: "~imi~", "~imi~";
        instrumental_nonvirile: "~imi~", "~imi~";
        locative_masculine: "~im~", "~im~";
        locative_feminine: "~jej~", "~jej~";
        locative_neuter: "~im~", "~im~";
        locative_virile: "~ich~", "~ich~";
        locative_nonvirile: "~ich~", "~ich~";
    ),
    // On: 1 words (on).
    declension!("on", 1, "~";
        nominative_masculine: "~", "~";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~o", "~o";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~o", "~o";
        accusative_virile: "~ych", "~ych";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Ile: 1 words (ile).
    declension!("ile", 1, "~e";
        nominative_masculine: "~y", "~y";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~e", "~e";
        nominative_virile: "~u", "~u";
        nominative_nonvirile: "~e", "~e";
        genitive_masculine: "~ego", "~ego";
        genitive_feminine: "~ej", "~ej";
        genitive_neuter: "~ego", "~ego";
        genitive_virile: "~ych", "~ych";
        genitive_nonvirile: "~ych", "~ych";
        dative_masculine: "~emu", "~emu";
        dative_feminine: "~ej", "~ej";
        dative_neuter: "~emu", "~emu";
        dative_virile: "~ym", "~ym";
        dative_nonvirile: "~ym", "~ym";
        accusative_masculine: "~ego", "~y";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~e", "~e";
        accusative_virile: "~u", "~u";
        accusative_nonvirile: "~e", "~e";
        instrumental_masculine: "~ym", "~ym";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~ym", "~ym";
        instrumental_virile: "~ymi", "~ymi";
        instrumental_nonvirile: "~ymi", "~ymi";
        locative_masculine: "~ym", "~ym";
        locative_feminine: "~ej", "~ej";
        locative_neuter: "~ym", "~ym";
        locative_virile: "~ych", "~ych";
        locative_nonvirile: "~ych", "~ych";
    ),
    // Ki: 1 words (ki).
    declension!("ki", 1, "~i";
        nominative_masculine: "~i", "~i";
        nominative_feminine: "~a", "~a";
        nominative_neuter: "~ie", "~ie";
        nominative_virile: "~i", "~i";
        nominative_nonvirile: "~ie", "~ie";
        genitive_masculine: "~iego", "~iego";
        genitive_feminine: "~iej", "~iej";
        genitive_neuter: "~iego", "~iego";
        genitive_virile: "~ich", "~ich";
        genitive_nonvirile: "~ich", "~ich";
        dative_masculine: "~iemu", "~iemu";
        dative_feminine: "~iej", "~iej";
        dative_neuter: "~iemu", "~iemu";
        dative_virile: "~im", "~im";
        dative_nonvirile: "~im", "~im";
        accusative_masculine: "~iego", "~i";
        accusative_feminine: "~ą", "~ą";
        accusative_neuter: "~ie", "~ie";
        accusative_virile: "~ich", "~ich";
        accusative_nonvirile: "~ie", "~ie";
        instrumental_masculine: "~im", "~im";
        instrumental_feminine: "~ą", "~ą";
        instrumental_neuter: "~im", "~im";
        instrumental_virile: "~imi", "~imi";
        instrumental_nonvirile: "~imi", "~imi";
        locative_masculine: "~im", "~im";
        locative_feminine: "~iej", "~iej";
        locative_neuter: "~im", "~im";
        locative_virile: "~ich", "~ich";
        locative_nonvirile: "~ich", "~ich";
    ),
    // Tamten: 1 words (tamten).
    declension!("tamten", 1, "~ten";
        nominative_masculine: "~ten", "~ten";
        nominative_feminine: "~ta", "~ta";
        nominative_neuter: "~to", "~to";
        nominative_virile: "~ci", "~ci";
        nominative_nonvirile: "~te", "~te";
        genitive_masculine: "~tego", "~tego";
        genitive_feminine: "~tej", "~tej";
        genitive_neuter: "~tego", "~tego";
        genitive_virile: "~tych", "~tych";
        genitive_nonvirile: "~tych", "~tych";
        dative_masculine: "~temu", "~temu";
        dative_feminine: "~tej", "~tej";
        dative_neuter: "~temu", "~temu";
        dative_virile: "~tym", "~tym";
        dative_nonvirile: "~tym", "~tym";
        accusative_masculine: "~tego", "~ten";
        accusative_feminine: "~tę|~tą", "~tę|~tą";
        accusative_neuter: "~to", "~to";
        accusative_virile: "~tych", "~tych";
        accusative_nonvirile: "~te", "~te";
        instrumental_masculine: "~tym", "~tym";
        instrumental_feminine: "~tą", "~tą";
        instrumental_neuter: "~tym", "~tym";
        instrumental_virile: "~tymi", "~tymi";
        instrumental_nonvirile: "~tymi", "~tymi";
        locative_masculine: "~tym", "~tym";
        locative_feminine: "~tej", "~tej";
        locative_neuter: "~tym", "~tym";
        locative_virile: "~tych", "~tych";
        locative_nonvirile: "~tych", "~tych";
    ),
    // Pewien: 1 words (pewien).
    declension!("pewien", 1, "~ie~";
        nominative_masculine: "~ie~", "~ie~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~e", "~~e";
        nominative_virile: "~~i", "~~i";
        nominative_nonvirile: "~~e", "~~e";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~ie~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~e", "~~e";
        accusative_virile: "~~ych", "~~ych";
        accusative_nonvirile: "~~e", "~~e";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
    ),
    // Jakikolwiek: 1 words (jakikolwiek).
    declension!("jakikolwiek", 1, "~~ik~~";
        nominative_masculine: "~~ik~~", "~~ik~~";
        nominative_feminine: "~~ak~~", "~~ak~~";
        nominative_neuter: "~~iek~~", "~~iek~~";
        nominative_virile: "~cy~~~", "~cy~~~";
        nominative_nonvirile: "~~iek~~", "~~iek~~";
        genitive_masculine: "~~ieg~ko~", "~~ieg~ko~";
        genitive_feminine: "~~iejk~~", "~~iejk~~";
        genitive_neuter: "~~ieg~ko~", "~~ieg~ko~";
        genitive_virile: "~~ichk~~", "~~ichk~~";
        genitive_nonvirile: "~~ichk~~", "~~ichk~~";
        dative_masculine: "~~iemuk~~", "~~iemuk~~";
        dative_feminine: "~~iejk~~", "~~iejk~~";
        dative_neuter: "~~iemuk~~", "~~iemuk~~";
        dative_virile: "~~imk~~", "~~imk~~";
        dative_nonvirile: "~~imk~~", "~~imk~~";
        accusative_masculine: "~~ieg~ko~", "~~ik~~";
        accusative_feminine: "~~ąk~~", "~~ąk~~";
        accusative_neuter: "~~iek~~", "~~iek~~";
        accusative_virile: "~~ichk~~", "~~ichk~~";
        accusative_nonvirile: "~~iek~~", "~~iek~~";
        instrumental_masculine: "~~imk~~", "~~imk~~";
        instrumental_feminine: "~~ąk~~", "~~ąk~~";
        instrumental_neuter: "~~imk~~", "~~imk~~";
        instrumental_virile: "~~imik~~", "~~imik~~";
        instrumental_nonvirile: "~~imik~~", "~~imik~~";
        locative_masculine: "~~imk~~", "~~imk~~";
        locative_feminine: "~~iejk~~", "~~iejk~~";
        locative_neuter: "~~imk~~", "~~imk~~";
        locative_virile: "~~ichk~~", "~~ichk~~";
        locative_nonvirile: "~~ichk~~", "~~ichk~~";
    ),
    // Którykolwiek: 1 words (którykolwiek).
    declension!("którykolwiek", 1, "~y~";
        nominative_masculine: "~y~", "~y~";
        nominative_feminine: "~a~", "~a~";
        nominative_neuter: "~e~", "~e~";
        nominative_virile: "~zy~", "~zy~";
        nominative_nonvirile: "~e~", "~e~";
        genitive_masculine: "~ego~", "~ego~";
        genitive_feminine: "~ej~", "~ej~";
        genitive_neuter: "~ego~", "~ego~";
        genitive_virile: "~ych~", "~ych~";
        genitive_nonvirile: "~ych~", "~ych~";
        dative_masculine: "~emu~", "~emu~";
        dative_feminine: "~ej~", "~ej~";
        dative_neuter: "~emu~", "~emu~";
        dative_virile: "~ym~", "~ym~";
        dative_nonvirile: "~ym~", "~ym~";
        accusative_masculine: "~ego~", "~y~";
        accusative_feminine: "~ą~", "~ą~";
        accusative_neuter: "~e~", "~e~";
        accusative_virile: "~ych~", "~ych~";
        accusative_nonvirile: "~e~", "~e~";
        instrumental_masculine: "~ym~", "~ym~";
        instrumental_feminine: "~ą~", "~ą~";
        instrumental_neuter: "~ym~", "~ym~";
        instrumental_virile: "~ymi~", "~ymi~";
        instrumental_nonvirile: "~ymi~", "~ymi~";
        locative_masculine: "~ym~", "~ym~";
        locative_feminine: "~ej~", "~ej~";
        locative_neuter: "~ym~", "~ym~";
        locative_virile: "~ych~", "~ych~";
        locative_nonvirile: "~ych~", "~ych~";
    ),
    // Ów: 1 words (ów).
    declension!("ów", 1, "ó~";
        nominative_masculine: "ó~", "ó~";
        nominative_feminine: "o~a", "o~a";
        nominative_neuter: "o~o", "o~o";
        nominative_virile: "o~i", "o~i";
        nominative_nonvirile: "o~e", "o~e";
        genitive_masculine: "o~ego", "o~ego";
        genitive_feminine: "o~ej", "o~ej";
        genitive_neuter: "o~ego", "o~ego";
        genitive_virile: "o~ych", "o~ych";
        genitive_nonvirile: "o~ych", "o~ych";
        dative_masculine: "o~emu", "o~emu";
        dative_feminine: "o~ej", "o~ej";
        dative_neuter: "o~emu", "o~emu";
        dative_virile: "o~ym", "o~ym";
        dative_nonvirile: "o~ym", "o~ym";
        accusative_masculine: "o~ego", "ó~";
        accusative_feminine: "o~ą", "o~ą";
        accusative_neuter: "o~o", "o~o";
        accusative_virile: "o~ych", "o~ych";
        accusative_nonvirile: "o~e", "o~e";
        instrumental_masculine: "o~ym", "o~ym";
        instrumental_feminine: "o~ą", "o~ą";
        instrumental_neuter: "o~ym", "o~ym";
        instrumental_virile: "o~ymi", "o~ymi";
        instrumental_nonvirile: "o~ymi", "o~ymi";
        locative_masculine: "o~ym", "o~ym";
        locative_feminine: "o~ej", "o~ej";
        locative_neuter: "o~ym", "o~ym";
        locative_virile: "o~ych", "o~ych";
        locative_nonvirile: "o~ych", "o~ych";
    ),
    // Niejeden: 1 words (niejeden).
    declension!("niejeden", 1, "~e~";
        nominative_masculine: "~e~", "~e~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~o", "~~o";
        nominative_virile: "~~i", "~~i";
        nominative_nonvirile: "~~e", "~~e";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~e~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~o", "~~o";
        accusative_virile: "~~ych", "~~ych";
        accusative_nonvirile: "~~e", "~~e";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
    ),
    // Ilekolwiek: 1 words (ilekolwiek).
    declension!("ilekolwiek", 1, "~e~";
        nominative_masculine: "~e~", "~e~";
        nominative_feminine: "~~a", "~~a";
        nominative_neuter: "~~o", "~~o";
        nominative_virile: "~u~", "~u~";
        nominative_nonvirile: "~e~", "~e~";
        genitive_masculine: "~~ego", "~~ego";
        genitive_feminine: "~~ej", "~~ej";
        genitive_neuter: "~~ego", "~~ego";
        genitive_virile: "~~ych", "~~ych";
        genitive_nonvirile: "~~ych", "~~ych";
        dative_masculine: "~~emu", "~~emu";
        dative_feminine: "~~ej", "~~ej";
        dative_neuter: "~~emu", "~~emu";
        dative_virile: "~~ym", "~~ym";
        dative_nonvirile: "~~ym", "~~ym";
        accusative_masculine: "~~ego", "~e~";
        accusative_feminine: "~~ą", "~~ą";
        accusative_neuter: "~~o", "~~o";
        accusative_virile: "~u~", "~u~";
        accusative_nonvirile: "~e~", "~e~";
        instrumental_masculine: "~~ym", "~~ym";
        instrumental_feminine: "~~ą", "~~ą";
        instrumental_neuter: "~~ym", "~~ym";
        instrumental_virile: "~~ymi", "~~ymi";
        instrumental_nonvirile: "~~ymi", "~~ymi";
        locative_masculine: "~~ym", "~~ym";
        locative_feminine: "~~ej", "~~ej";
        locative_neuter: "~~ym", "~~ym";
        locative_virile: "~~ych", "~~ych";
        locative_nonvirile: "~~ych", "~~ych";
    ),
];
