//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative_masculine` (sg, du, pl)
/// `nominative_feminine` (sg, du, pl)
/// `nominative_neuter` (sg, du, pl)
/// `accusative_masculine` (sg, du, pl)
/// `accusative_feminine` (sg, du, pl)
/// `accusative_neuter` (sg, du, pl)
/// `instrumental_masculine` (sg, du, pl)
/// `instrumental_feminine` (sg, du, pl)
/// `instrumental_neuter` (sg, du, pl)
/// `dative_masculine` (sg, du, pl)
/// `dative_feminine` (sg, du, pl)
/// `dative_neuter` (sg, du, pl)
/// `ablative_masculine` (sg, du, pl)
/// `ablative_feminine` (sg, du, pl)
/// `ablative_neuter` (sg, du, pl)
/// `genitive_masculine` (sg, du, pl)
/// `genitive_feminine` (sg, du, pl)
/// `genitive_neuter` (sg, du, pl)
/// `locative_masculine` (sg, du, pl)
/// `locative_feminine` (sg, du, pl)
/// `locative_neuter` (sg, du, pl)
/// `vocative_masculine` (sg, du, pl)
/// `vocative_feminine` (sg, du, pl)
/// `vocative_neuter` (sg, du, pl)
/// `comparative`
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative_masculine: $c0:tt, $c1:tt, $c2:tt;
     nominative_feminine: $c3:tt, $c4:tt, $c5:tt;
     nominative_neuter: $c6:tt, $c7:tt, $c8:tt;
     accusative_masculine: $c9:tt, $c10:tt, $c11:tt;
     accusative_feminine: $c12:tt, $c13:tt, $c14:tt;
     accusative_neuter: $c15:tt, $c16:tt, $c17:tt;
     instrumental_masculine: $c18:tt, $c19:tt, $c20:tt;
     instrumental_feminine: $c21:tt, $c22:tt, $c23:tt;
     instrumental_neuter: $c24:tt, $c25:tt, $c26:tt;
     dative_masculine: $c27:tt, $c28:tt, $c29:tt;
     dative_feminine: $c30:tt, $c31:tt, $c32:tt;
     dative_neuter: $c33:tt, $c34:tt, $c35:tt;
     ablative_masculine: $c36:tt, $c37:tt, $c38:tt;
     ablative_feminine: $c39:tt, $c40:tt, $c41:tt;
     ablative_neuter: $c42:tt, $c43:tt, $c44:tt;
     genitive_masculine: $c45:tt, $c46:tt, $c47:tt;
     genitive_feminine: $c48:tt, $c49:tt, $c50:tt;
     genitive_neuter: $c51:tt, $c52:tt, $c53:tt;
     locative_masculine: $c54:tt, $c55:tt, $c56:tt;
     locative_feminine: $c57:tt, $c58:tt, $c59:tt;
     locative_neuter: $c60:tt, $c61:tt, $c62:tt;
     vocative_masculine: $c63:tt, $c64:tt, $c65:tt;
     vocative_feminine: $c66:tt, $c67:tt, $c68:tt;
     vocative_neuter: $c69:tt, $c70:tt, $c71:tt;
     comparative: $c72:tt;
     superlative: $c73:tt;
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
                flexion::form!($c60),
                flexion::form!($c61),
                flexion::form!($c62),
                flexion::form!($c63),
                flexion::form!($c64),
                flexion::form!($c65),
                flexion::form!($c66),
                flexion::form!($c67),
                flexion::form!($c68),
                flexion::form!($c69),
                flexion::form!($c70),
                flexion::form!($c71),
                flexion::form!($c72),
                flexion::form!($c73),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `धृत`: 907 words.
    धृत,
    /// Like `अवर`: 245 words.
    अवर,
    /// Like `मूढ`: 181 words.
    मूढ,
    /// Like `वामन`: 110 words.
    वामन,
    /// Like `हार`: 60 words.
    हार,
    /// Like `भाविन्`: 59 words.
    भाविन,
    /// Like `ांशु`: 34 words.
    ांशु,
    /// Like `शुचि`: 32 words.
    शुचि,
    /// Like `रसवत्`: 31 words.
    रसवत,
    /// Like `काशक`: 22 words.
    काशक,
    /// Like `मनस्`: 19 words.
    मनस,
    /// Like `भारिन्`: 18 words.
    भारिन,
    /// Like `हितकर`: 18 words.
    हितकर,
    /// Like `ऋतावन्`: 15 words.
    ऋतावन,
    /// Like `ज्यायस्`: 15 words.
    जयायस,
    /// Like `धृषु`: 13 words.
    धृषु,
    /// Like `थिर`: 13 words.
    थिर,
    /// Like `लघु`: 11 words.
    लघु,
    /// Like `लीन`: 11 words.
    लीन,
    /// Like `छिद्`: 9 words.
    छिद,
    /// Like `तरुतृ`: 8 words.
    तरुतृ,
    /// Like `अनूप`: 8 words.
    अनूप,
    /// Like `कारक`: 8 words.
    कारक,
    /// Like `रथी`: 8 words.
    रथी,
    /// Like `सर्वजित्`: 7 words.
    सरवजित,
    /// Like `प्रिय`: 6 words.
    परिय,
    /// Like `असत्`: 6 words.
    असत,
    /// Like `महात्मन्`: 6 words.
    महातमन,
    /// Like `राज्`: 6 words.
    राज,
    /// Like `पा`: 6 words.
    पा,
    /// Like `हरि`: 6 words.
    हरि,
    /// Like `पृश्`: 5 words.
    पृश,
    /// Like `नीरोग`: 4 words.
    नीरोग,
    /// Like `अमनस्`: 4 words.
    अमनस,
    /// Like `बहुव्रीहि`: 3 words.
    बहुवरीहि,
    /// Like `रघु`: 3 words.
    रघु,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 36] = [
        Declension::धृत,
        Declension::अवर,
        Declension::मूढ,
        Declension::वामन,
        Declension::हार,
        Declension::भाविन,
        Declension::ांशु,
        Declension::शुचि,
        Declension::रसवत,
        Declension::काशक,
        Declension::मनस,
        Declension::भारिन,
        Declension::हितकर,
        Declension::ऋतावन,
        Declension::जयायस,
        Declension::धृषु,
        Declension::थिर,
        Declension::लघु,
        Declension::लीन,
        Declension::छिद,
        Declension::तरुतृ,
        Declension::अनूप,
        Declension::कारक,
        Declension::रथी,
        Declension::सरवजित,
        Declension::परिय,
        Declension::असत,
        Declension::महातमन,
        Declension::राज,
        Declension::पा,
        Declension::हरि,
        Declension::पृश,
        Declension::नीरोग,
        Declension::अमनस,
        Declension::बहुवरीहि,
        Declension::रघु,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 36] = [
    // धृत: 907 words (ज, द, कमल, बुद्ध, सह, आद).
    declension!("ध\u{943}त", 907, "~";
        nominative_masculine: "~ः", "~ौ", "~ाः";
        nominative_feminine: "~ा", "~\u{947}", "~ाः";
        nominative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        accusative_masculine: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        accusative_feminine: "~ाम\u{94d}", "~\u{947}", "~ाः";
        accusative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        instrumental_masculine: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        instrumental_feminine: "~या", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        instrumental_neuter: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative_masculine: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        dative_feminine: "~ाय\u{948}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        dative_neuter: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_masculine: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_feminine: "~ायाः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative_neuter: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive_masculine: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        genitive_feminine: "~ायाः", "~योः", "~ानाम\u{94d}";
        genitive_neuter: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        locative_masculine: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        locative_feminine: "~ायाम\u{94d}", "~योः", "~ास\u{941}";
        locative_neuter: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative_masculine: "~", "~ौ", "~ाः";
        vocative_feminine: "~\u{947}", "~\u{947}", "~ाः";
        vocative_neuter: "~", "~\u{947}", "~ानि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // अवर: 245 words (मूर्ख, मधुर, ताम्र, सूर्य, कार्य, मनुष्य).
    declension!("अवर", 245, "~";
        nominative_masculine: "~ः", "~ौ", "~ाः";
        nominative_feminine: "~ा", "~\u{947}", "~ाः";
        nominative_neuter: "~म\u{94d}", "~\u{947}", "~ाणि";
        accusative_masculine: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        accusative_feminine: "~ाम\u{94d}", "~\u{947}", "~ाः";
        accusative_neuter: "~म\u{94d}", "~\u{947}", "~ाणि";
        instrumental_masculine: "~\u{947}ण", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        instrumental_feminine: "~या", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        instrumental_neuter: "~\u{947}ण", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative_masculine: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        dative_feminine: "~ाय\u{948}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        dative_neuter: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_masculine: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_feminine: "~ायाः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative_neuter: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive_masculine: "~स\u{94d}य", "~योः", "~ाणाम\u{94d}";
        genitive_feminine: "~ायाः", "~योः", "~ाणाम\u{94d}";
        genitive_neuter: "~स\u{94d}य", "~योः", "~ाणाम\u{94d}";
        locative_masculine: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        locative_feminine: "~ायाम\u{94d}", "~योः", "~ास\u{941}";
        locative_neuter: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative_masculine: "~", "~ौ", "~ाः";
        vocative_feminine: "~\u{947}", "~\u{947}", "~ाः";
        vocative_neuter: "~", "~\u{947}", "~ाणि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // मूढ: 181 words (भारत, देव, गणित, लेखन, दैव, मर्दन).
    declension!("म\u{942}ढ", 181, "~";
        nominative_masculine: "~ः", "~ौ", "~ाः";
        nominative_feminine: "~ी", "~\u{94d}यौ", "~\u{94d}यः";
        nominative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        accusative_masculine: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        accusative_feminine: "~ीम\u{94d}", "~\u{94d}यौ", "~ीः";
        accusative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        instrumental_masculine: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        instrumental_feminine: "~\u{94d}या", "~ीभ\u{94d}याम\u{94d}", "~ीभिः";
        instrumental_neuter: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative_masculine: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        dative_feminine: "~\u{94d}य\u{948}", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        dative_neuter: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_masculine: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_feminine: "~\u{94d}याः", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        ablative_neuter: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive_masculine: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        genitive_feminine: "~\u{94d}याः", "~\u{94d}योः", "~ीनाम\u{94d}";
        genitive_neuter: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        locative_masculine: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        locative_feminine: "~\u{94d}याम\u{94d}", "~\u{94d}योः", "~ीष\u{941}";
        locative_neuter: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative_masculine: "~", "~ौ", "~ाः";
        vocative_feminine: "~ि", "~\u{94d}यौ", "~\u{94d}यः";
        vocative_neuter: "~", "~\u{947}", "~ानि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // वामन: 110 words (जन, इन, दूरदर्शन, दिन, कठिन, बुध्न).
    declension!("वामन", 110, "~";
        nominative_masculine: "~ः", "~ौ", "~ाः";
        nominative_feminine: "~ा", "~\u{947}", "~ाः";
        nominative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        accusative_masculine: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        accusative_feminine: "~ाम\u{94d}", "~\u{947}", "~ाः";
        accusative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        instrumental_masculine: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        instrumental_feminine: "~या", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        instrumental_neuter: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative_masculine: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        dative_feminine: "~ाय\u{948}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        dative_neuter: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_masculine: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_feminine: "~ायाः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative_neuter: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive_masculine: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        genitive_feminine: "~ायाः", "~योः", "~ानाम\u{94d}";
        genitive_neuter: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        locative_masculine: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        locative_feminine: "~ायाम\u{94d}", "~योः", "~ास\u{941}";
        locative_neuter: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative_masculine: "~", "~ौ", "~ाः";
        vocative_feminine: "~\u{947}", "~\u{947}", "~ाः";
        vocative_neuter: "~", "~\u{947}", "~ानि";
        comparative: "~ीयस\u{94d}";
        superlative: "~िष\u{94d}ठ";
    ),
    // हार: 60 words (सर्प, कार, क्षौर, गौर, अक्ष, ऋक्ष).
    declension!("हार", 60, "~";
        nominative_masculine: "~ः", "~ौ", "~ाः";
        nominative_feminine: "~ी", "~\u{94d}यौ", "~\u{94d}यः";
        nominative_neuter: "~म\u{94d}", "~\u{947}", "~ाणि";
        accusative_masculine: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        accusative_feminine: "~ीम\u{94d}", "~\u{94d}यौ", "~ीः";
        accusative_neuter: "~म\u{94d}", "~\u{947}", "~ाणि";
        instrumental_masculine: "~\u{947}ण", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        instrumental_feminine: "~\u{94d}या", "~ीभ\u{94d}याम\u{94d}", "~ीभिः";
        instrumental_neuter: "~\u{947}ण", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative_masculine: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        dative_feminine: "~\u{94d}य\u{948}", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        dative_neuter: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_masculine: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_feminine: "~\u{94d}याः", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        ablative_neuter: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive_masculine: "~स\u{94d}य", "~योः", "~ाणाम\u{94d}";
        genitive_feminine: "~\u{94d}याः", "~\u{94d}योः", "~ीणाम\u{94d}";
        genitive_neuter: "~स\u{94d}य", "~योः", "~ाणाम\u{94d}";
        locative_masculine: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        locative_feminine: "~\u{94d}याम\u{94d}", "~\u{94d}योः", "~ीष\u{941}";
        locative_neuter: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative_masculine: "~", "~ौ", "~ाः";
        vocative_feminine: "~ि", "~\u{94d}यौ", "~\u{94d}यः";
        vocative_neuter: "~", "~\u{947}", "~ाणि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // भाविन: 59 words (हस्तिन्, मायिन्, केशिन्, एकाकिन्, ज्ञानिन्, घातिन्).
    declension!("भाविन\u{94d}", 59, "~िन\u{94d}";
        nominative_masculine: "~ी", "~िनौ", "~िनः";
        nominative_feminine: "~िनी", "~िन\u{94d}यौ", "~िन\u{94d}यः";
        nominative_neuter: "~ि", "~िनी", "~ीनि";
        accusative_masculine: "~िनम\u{94d}", "~िनौ", "~िनः";
        accusative_feminine: "~िनीम\u{94d}", "~िन\u{94d}यौ", "~िनीः";
        accusative_neuter: "~ि", "~िनी", "~ीनि";
        instrumental_masculine: "~िना", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        instrumental_feminine: "~िन\u{94d}या", "~िनीभ\u{94d}याम\u{94d}", "~िनीभिः";
        instrumental_neuter: "~िना", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative_masculine: "~िन\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        dative_feminine: "~िन\u{94d}य\u{948}", "~िनीभ\u{94d}याम\u{94d}", "~िनीभ\u{94d}यः";
        dative_neuter: "~िन\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_masculine: "~िनः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_feminine: "~िन\u{94d}याः", "~िनीभ\u{94d}याम\u{94d}", "~िनीभ\u{94d}यः";
        ablative_neuter: "~िनः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive_masculine: "~िनः", "~िनोः", "~िनाम\u{94d}";
        genitive_feminine: "~िन\u{94d}याः", "~िन\u{94d}योः", "~िनीनाम\u{94d}";
        genitive_neuter: "~िनः", "~िनोः", "~िनाम\u{94d}";
        locative_masculine: "~िनि", "~िनोः", "~िष\u{941}";
        locative_feminine: "~िन\u{94d}याम\u{94d}", "~िन\u{94d}योः", "~िनीष\u{941}";
        locative_neuter: "~िनि", "~िनोः", "~िष\u{941}";
        vocative_masculine: "~िन\u{94d}", "~िनौ", "~िनः";
        vocative_feminine: "~िनि", "~िन\u{94d}यौ", "~िन\u{94d}यः";
        vocative_neuter: "~ि|~िन\u{94d}", "~िनी", "~ीनि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // ांशु: 34 words (वायु, पृदाकुसानु, मनु, पाण्डु, सेतु, शम्भु).
    declension!("ा\u{902}श\u{941}", 34, "~\u{941}";
        nominative_masculine: "~\u{941}ः", "~\u{942}", "~वः";
        nominative_feminine: "~\u{941}ः", "~\u{942}", "~वः";
        nominative_neuter: "~\u{941}", "~\u{941}नी", "~\u{942}नि";
        accusative_masculine: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}न\u{94d}";
        accusative_feminine: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}ः";
        accusative_neuter: "~\u{941}", "~\u{941}नी", "~\u{942}नि";
        instrumental_masculine: "~\u{941}ना", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        instrumental_feminine: "~\u{94d}वा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        instrumental_neuter: "~\u{941}ना", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative_masculine: "~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        dative_feminine: "~व\u{947}|~\u{94d}व\u{948}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        dative_neuter: "~\u{941}न\u{947}|~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_masculine: "~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_feminine: "~ोः|~\u{94d}वाः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_neuter: "~\u{941}नः|~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive_masculine: "~ोः", "~\u{94d}वोः", "~\u{942}नाम\u{94d}";
        genitive_feminine: "~ोः|~\u{94d}वाः", "~\u{94d}वोः", "~\u{942}नाम\u{94d}";
        genitive_neuter: "~\u{941}नः|~ोः", "~\u{941}नोः|~\u{94d}वोः", "~\u{942}नाम\u{94d}";
        locative_masculine: "~ौ", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        locative_feminine: "~ौ|~\u{94d}वाम\u{94d}", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        locative_neuter: "~\u{941}नि|~ौ", "~\u{941}नोः|~\u{94d}वोः", "~\u{941}ष\u{941}";
        vocative_masculine: "~ो", "~\u{942}", "~वः";
        vocative_feminine: "~ो", "~\u{942}", "~वः";
        vocative_neuter: "~\u{941}|~ो", "~\u{941}नी", "~\u{942}नि";
        comparative: "~ीयस\u{94d}";
        superlative: "~िष\u{94d}ठ";
    ),
    // शुचि: 32 words (संधि, कवि, अभि, गभस्ति, पुलस्ति, वसति).
    declension!("श\u{941}चि", 32, "~ि";
        nominative_masculine: "~िः", "~ी", "~यः";
        nominative_feminine: "~िः", "~ी", "~यः";
        nominative_neuter: "~ि", "~िनी", "~ीनि";
        accusative_masculine: "~िम\u{94d}", "~ी", "~ीन\u{94d}";
        accusative_feminine: "~िम\u{94d}", "~ी", "~ीः";
        accusative_neuter: "~ि", "~िनी", "~ीनि";
        instrumental_masculine: "~िना", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        instrumental_feminine: "~\u{94d}या", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        instrumental_neuter: "~िना", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative_masculine: "~य\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        dative_feminine: "~य\u{947}|~\u{94d}य\u{948}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        dative_neuter: "~िन\u{947}|~य\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_masculine: "~\u{947}ः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_feminine: "~\u{947}ः|~\u{94d}याः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_neuter: "~िनः|~\u{947}ः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive_masculine: "~\u{947}ः", "~\u{94d}योः", "~ीनाम\u{94d}";
        genitive_feminine: "~\u{947}ः|~\u{94d}याः", "~\u{94d}योः", "~ीनाम\u{94d}";
        genitive_neuter: "~िनः|~\u{947}ः", "~िनोः|~\u{94d}योः", "~ीनाम\u{94d}";
        locative_masculine: "~ौ", "~\u{94d}योः", "~िष\u{941}";
        locative_feminine: "~ौ|~\u{94d}याम\u{94d}", "~\u{94d}योः", "~िष\u{941}";
        locative_neuter: "~िनि|~ौ", "~िनोः|~\u{94d}योः", "~िष\u{941}";
        vocative_masculine: "~\u{947}", "~ी", "~यः";
        vocative_feminine: "~\u{947}", "~ी", "~यः";
        vocative_neuter: "~ि|~\u{947}", "~िनी", "~ीनि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // रसवत: 31 words (जगत्, अंशुमत्, हिमवत्, महत्, भगवत्, अन्तवत्).
    declension!("रसवत\u{94d}", 31, "~त\u{94d}";
        nominative_masculine: "~ान\u{94d}", "~न\u{94d}तौ", "~न\u{94d}तः";
        nominative_feminine: "~ती", "~त\u{94d}यौ", "~त\u{94d}यः";
        nominative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        accusative_masculine: "~न\u{94d}तम\u{94d}", "~न\u{94d}तौ", "~तः";
        accusative_feminine: "~तीम\u{94d}", "~त\u{94d}यौ", "~तीः";
        accusative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        instrumental_masculine: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        instrumental_feminine: "~त\u{94d}या", "~तीभ\u{94d}याम\u{94d}", "~तीभिः";
        instrumental_neuter: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative_masculine: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        dative_feminine: "~त\u{94d}य\u{948}", "~तीभ\u{94d}याम\u{94d}", "~तीभ\u{94d}यः";
        dative_neuter: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_masculine: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_feminine: "~त\u{94d}याः", "~तीभ\u{94d}याम\u{94d}", "~तीभ\u{94d}यः";
        ablative_neuter: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive_masculine: "~तः", "~तोः", "~ताम\u{94d}";
        genitive_feminine: "~त\u{94d}याः", "~त\u{94d}योः", "~तीनाम\u{94d}";
        genitive_neuter: "~तः", "~तोः", "~ताम\u{94d}";
        locative_masculine: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        locative_feminine: "~त\u{94d}याम\u{94d}", "~त\u{94d}योः", "~तीष\u{941}";
        locative_neuter: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        vocative_masculine: "~न\u{94d}", "~न\u{94d}तौ", "~न\u{94d}तः";
        vocative_feminine: "~ति", "~त\u{94d}यौ", "~त\u{94d}यः";
        vocative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        comparative: "~त\u{94d}तर";
        superlative: "~त\u{94d}तम";
    ),
    // काशक: 22 words (आवश्यक, अंशक, जनक, नन्दक, खानक, पालक).
    declension!("काशक", 22, "~~";
        nominative_masculine: "~~ः", "~~ौ", "~~ाः";
        nominative_feminine: "~ि~ा", "~ि~\u{947}", "~ि~ाः";
        nominative_neuter: "~~म\u{94d}", "~~\u{947}", "~~ानि";
        accusative_masculine: "~~म\u{94d}", "~~ौ", "~~ान\u{94d}";
        accusative_feminine: "~ि~ाम\u{94d}", "~ि~\u{947}", "~ि~ाः";
        accusative_neuter: "~~म\u{94d}", "~~\u{947}", "~~ानि";
        instrumental_masculine: "~~\u{947}न", "~~ाभ\u{94d}याम\u{94d}", "~~\u{948}ः";
        instrumental_feminine: "~ि~या", "~ि~ाभ\u{94d}याम\u{94d}", "~ि~ाभिः";
        instrumental_neuter: "~~\u{947}न", "~~ाभ\u{94d}याम\u{94d}", "~~\u{948}ः";
        dative_masculine: "~~ाय", "~~ाभ\u{94d}याम\u{94d}", "~~\u{947}भ\u{94d}यः";
        dative_feminine: "~ि~ाय\u{948}", "~ि~ाभ\u{94d}याम\u{94d}", "~ि~ाभ\u{94d}यः";
        dative_neuter: "~~ाय", "~~ाभ\u{94d}याम\u{94d}", "~~\u{947}भ\u{94d}यः";
        ablative_masculine: "~~ात\u{94d}", "~~ाभ\u{94d}याम\u{94d}", "~~\u{947}भ\u{94d}यः";
        ablative_feminine: "~ि~ायाः", "~ि~ाभ\u{94d}याम\u{94d}", "~ि~ाभ\u{94d}यः";
        ablative_neuter: "~~ात\u{94d}", "~~ाभ\u{94d}याम\u{94d}", "~~\u{947}भ\u{94d}यः";
        genitive_masculine: "~~स\u{94d}य", "~~योः", "~~ानाम\u{94d}";
        genitive_feminine: "~ि~ायाः", "~ि~योः", "~ि~ानाम\u{94d}";
        genitive_neuter: "~~स\u{94d}य", "~~योः", "~~ानाम\u{94d}";
        locative_masculine: "~~\u{947}", "~~योः", "~~\u{947}ष\u{941}";
        locative_feminine: "~ि~ायाम\u{94d}", "~ि~योः", "~ि~ास\u{941}";
        locative_neuter: "~~\u{947}", "~~योः", "~~\u{947}ष\u{941}";
        vocative_masculine: "~~", "~~ौ", "~~ाः";
        vocative_feminine: "~ि~\u{947}", "~ि~\u{947}", "~ि~ाः";
        vocative_neuter: "~~", "~~\u{947}", "~~ानि";
        comparative: "~~";
        superlative: "~~";
    ),
    // मनस: 19 words (रिशादस्, वीतिराधस्, वपुस्, तवस्, दुर्मनस्, अपस्).
    declension!("मनस\u{94d}", 19, "~स\u{94d}";
        nominative_masculine: "~ाः", "~सौ", "~सः";
        nominative_feminine: "~ाः", "~सौ", "~सः";
        nominative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        accusative_masculine: "~सम\u{94d}", "~सौ", "~सः";
        accusative_feminine: "~सम\u{94d}", "~सौ", "~सः";
        accusative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        instrumental_masculine: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        instrumental_feminine: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        instrumental_neuter: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        dative_masculine: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        dative_feminine: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        dative_neuter: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative_masculine: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative_feminine: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative_neuter: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        genitive_masculine: "~सः", "~सोः", "~साम\u{94d}";
        genitive_feminine: "~सः", "~सोः", "~साम\u{94d}";
        genitive_neuter: "~सः", "~सोः", "~साम\u{94d}";
        locative_masculine: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        locative_feminine: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        locative_neuter: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        vocative_masculine: "~ः", "~सौ", "~सः";
        vocative_feminine: "~ः", "~सौ", "~सः";
        vocative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        comparative: "~ीयस\u{94d}";
        superlative: "~स\u{94d}तम";
    ),
    // भारिन: 18 words (मन्त्रिन्, त्सारिन्, अवत्रापिन्, अहंकारिन्, रङ्गिन्, फिरङ्गिन्).
    declension!("भारिन\u{94d}", 18, "~िन\u{94d}";
        nominative_masculine: "~ी", "~िणौ", "~िणः";
        nominative_feminine: "~िणी", "~िण\u{94d}यौ", "~िण\u{94d}यः";
        nominative_neuter: "~ि", "~िणी", "~ीणि";
        accusative_masculine: "~िणम\u{94d}", "~िणौ", "~िणः";
        accusative_feminine: "~िणीम\u{94d}", "~िण\u{94d}यौ", "~िणीः";
        accusative_neuter: "~ि", "~िणी", "~ीणि";
        instrumental_masculine: "~िणा", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        instrumental_feminine: "~िण\u{94d}या", "~िणीभ\u{94d}याम\u{94d}", "~िणीभिः";
        instrumental_neuter: "~िणा", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative_masculine: "~िण\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        dative_feminine: "~िण\u{94d}य\u{948}", "~िणीभ\u{94d}याम\u{94d}", "~िणीभ\u{94d}यः";
        dative_neuter: "~िण\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_masculine: "~िणः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_feminine: "~िण\u{94d}याः", "~िणीभ\u{94d}याम\u{94d}", "~िणीभ\u{94d}यः";
        ablative_neuter: "~िणः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive_masculine: "~िणः", "~िणोः", "~िणाम\u{94d}";
        genitive_feminine: "~िण\u{94d}याः", "~िण\u{94d}योः", "~िणीनाम\u{94d}";
        genitive_neuter: "~िणः", "~िणोः", "~िणाम\u{94d}";
        locative_masculine: "~िणि", "~िणोः", "~िष\u{941}";
        locative_feminine: "~िण\u{94d}याम\u{94d}", "~िण\u{94d}योः", "~िणीष\u{941}";
        locative_neuter: "~िणि", "~िणोः", "~िष\u{941}";
        vocative_masculine: "~िन\u{94d}", "~िणौ", "~िणः";
        vocative_feminine: "~िणि", "~िण\u{94d}यौ", "~िण\u{94d}यः";
        vocative_neuter: "~ि|~िन\u{94d}", "~िणी", "~ीणि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // हितकर: 18 words (अरित्र, अग्निहोत्र, चित्र, सिंहदंष्ट्र, दरिद्र, अमित्र).
    declension!("हितकर", 18, "~ि~~र";
        nominative_masculine: "~ि~~रः", "~ि~~रौ", "~ि~~राः";
        nominative_feminine: "~ि~~रा", "~ि~~र\u{947}", "~ि~~राः";
        nominative_neuter: "~ि~~रम\u{94d}", "~ि~~र\u{947}", "~ि~~राणि";
        accusative_masculine: "~ि~~रम\u{94d}", "~ि~~रौ", "~ि~~रान\u{94d}";
        accusative_feminine: "~ि~~राम\u{94d}", "~ि~~र\u{947}", "~ि~~राः";
        accusative_neuter: "~ि~~रम\u{94d}", "~ि~~र\u{947}", "~ि~~राणि";
        instrumental_masculine: "~ि~~र\u{947}ण", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~र\u{948}ः";
        instrumental_feminine: "~ि~~रया", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~राभिः";
        instrumental_neuter: "~ि~~र\u{947}ण", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~र\u{948}ः";
        dative_masculine: "~ि~~राय", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~र\u{947}भ\u{94d}यः";
        dative_feminine: "~ि~~राय\u{948}", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~राभ\u{94d}यः";
        dative_neuter: "~ि~~राय", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~र\u{947}भ\u{94d}यः";
        ablative_masculine: "~ि~~रात\u{94d}", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~र\u{947}भ\u{94d}यः";
        ablative_feminine: "~ि~~रायाः", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~राभ\u{94d}यः";
        ablative_neuter: "~ि~~रात\u{94d}", "~ि~~राभ\u{94d}याम\u{94d}", "~ि~~र\u{947}भ\u{94d}यः";
        genitive_masculine: "~ि~~रस\u{94d}य", "~ि~~रयोः", "~ि~~राणाम\u{94d}";
        genitive_feminine: "~ि~~रायाः", "~ि~~रयोः", "~ि~~राणाम\u{94d}";
        genitive_neuter: "~ि~~रस\u{94d}य", "~ि~~रयोः", "~ि~~राणाम\u{94d}";
        locative_masculine: "~ि~~र\u{947}", "~ि~~रयोः", "~ि~~र\u{947}ष\u{941}";
        locative_feminine: "~ि~~रायाम\u{94d}", "~ि~~रयोः", "~ि~~रास\u{941}";
        locative_neuter: "~ि~~र\u{947}", "~ि~~रयोः", "~ि~~र\u{947}ष\u{941}";
        vocative_masculine: "~ि~~र", "~ि~~रौ", "~ि~~राः";
        vocative_feminine: "~ि~~र\u{947}", "~ि~~र\u{947}", "~ि~~राः";
        vocative_neuter: "~ि~~र", "~ि~~र\u{947}", "~ि~~राणि";
        comparative: "~\u{947}~ीयस~";
        superlative: "~\u{947}~िष~ठ";
    ),
    // ऋतावन: 15 words (वृत्रहन्, ऋतावन्, रक्षोहन्, वृषन्, हन्, यावन्).
    declension!("ऋतावन\u{94d}", 15, "~न\u{94d}";
        nominative_masculine: "~ा", "~ानौ", "~ानः";
        nominative_feminine: "~री", "~र\u{94d}यौ", "~र\u{94d}यः";
        nominative_neuter: "~", "~\u{94d}नी|~नी", "~ानि";
        accusative_masculine: "~ानम\u{94d}", "~ानौ", "~\u{94d}नः";
        accusative_feminine: "~रीम\u{94d}", "~र\u{94d}यौ", "~रीः";
        accusative_neuter: "~", "~\u{94d}नी|~नी", "~ानि";
        instrumental_masculine: "~\u{94d}ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        instrumental_feminine: "~र\u{94d}या", "~रीभ\u{94d}याम\u{94d}", "~रीभिः";
        instrumental_neuter: "~\u{94d}ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative_masculine: "~\u{94d}न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        dative_feminine: "~र\u{94d}य\u{948}", "~रीभ\u{94d}याम\u{94d}", "~रीभ\u{94d}यः";
        dative_neuter: "~\u{94d}न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative_masculine: "~\u{94d}नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative_feminine: "~र\u{94d}याः", "~रीभ\u{94d}याम\u{94d}", "~रीभ\u{94d}यः";
        ablative_neuter: "~\u{94d}नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive_masculine: "~\u{94d}नः", "~\u{94d}नोः", "~\u{94d}नाम\u{94d}";
        genitive_feminine: "~र\u{94d}याः", "~र\u{94d}योः", "~रीणाम\u{94d}";
        genitive_neuter: "~\u{94d}नः", "~\u{94d}नोः", "~\u{94d}नाम\u{94d}";
        locative_masculine: "~\u{94d}नि|~नि", "~\u{94d}नोः", "~स\u{941}";
        locative_feminine: "~र\u{94d}याम\u{94d}", "~र\u{94d}योः", "~रीष\u{941}";
        locative_neuter: "~\u{94d}नि|~नि", "~\u{94d}नोः", "~स\u{941}";
        vocative_masculine: "~न\u{94d}", "~ानौ", "~ानः";
        vocative_feminine: "~रि", "~र\u{94d}यौ", "~र\u{94d}यः";
        vocative_neuter: "~न\u{94d}|~", "~\u{94d}नी|~नी", "~ानि";
        comparative: "~यस\u{94d}";
        superlative: "~यिष\u{94d}ठ";
    ),
    // जयायस: 15 words (श्रेयस्, वस्यस्, मलीयस्, ज्यायस्, ओजीयस्, कनीयस्).
    declension!("ज\u{94d}यायस\u{94d}", 15, "~स\u{94d}";
        nominative_masculine: "~ान\u{94d}", "~ा\u{902}सौ", "~ा\u{902}सः";
        nominative_feminine: "~सी", "~स\u{94d}यौ", "~स\u{94d}यः";
        nominative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        accusative_masculine: "~ा\u{902}सम\u{94d}", "~ा\u{902}सौ", "~सः";
        accusative_feminine: "~सीम\u{94d}", "~स\u{94d}यौ", "~सीः";
        accusative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        instrumental_masculine: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        instrumental_feminine: "~स\u{94d}या", "~सीभ\u{94d}याम\u{94d}", "~सीभिः";
        instrumental_neuter: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        dative_masculine: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        dative_feminine: "~स\u{94d}य\u{948}", "~सीभ\u{94d}याम\u{94d}", "~सीभ\u{94d}यः";
        dative_neuter: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative_masculine: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative_feminine: "~स\u{94d}याः", "~सीभ\u{94d}याम\u{94d}", "~सीभ\u{94d}यः";
        ablative_neuter: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        genitive_masculine: "~सः", "~सोः", "~साम\u{94d}";
        genitive_feminine: "~स\u{94d}याः", "~स\u{94d}योः", "~सीनाम\u{94d}";
        genitive_neuter: "~सः", "~सोः", "~साम\u{94d}";
        locative_masculine: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        locative_feminine: "~स\u{94d}याम\u{94d}", "~स\u{94d}योः", "~सीष\u{941}";
        locative_neuter: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        vocative_masculine: "~न\u{94d}", "~ा\u{902}सौ", "~ा\u{902}सः";
        vocative_feminine: "~सि", "~स\u{94d}यौ", "~स\u{94d}यः";
        vocative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        comparative: "~ीयस\u{94d}";
        superlative: "~स\u{94d}तम";
    ),
    // धृषु: 13 words (दारु, तरु, पुरु, दीर्घायु, धृषु, रिपु).
    declension!("ध\u{943}ष\u{941}", 13, "~\u{941}";
        nominative_masculine: "~\u{941}ः", "~\u{942}", "~वः";
        nominative_feminine: "~\u{941}ः", "~\u{942}", "~वः";
        nominative_neuter: "~\u{941}", "~\u{941}णी", "~\u{942}णि";
        accusative_masculine: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}न\u{94d}";
        accusative_feminine: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}ः";
        accusative_neuter: "~\u{941}", "~\u{941}णी", "~\u{942}णि";
        instrumental_masculine: "~\u{941}णा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        instrumental_feminine: "~\u{94d}वा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        instrumental_neuter: "~\u{941}णा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative_masculine: "~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        dative_feminine: "~व\u{947}|~\u{94d}व\u{948}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        dative_neuter: "~\u{941}ण\u{947}|~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_masculine: "~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_feminine: "~ोः|~\u{94d}वाः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_neuter: "~\u{941}णः|~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive_masculine: "~ोः", "~\u{94d}वोः", "~\u{942}णाम\u{94d}";
        genitive_feminine: "~ोः|~\u{94d}वाः", "~\u{94d}वोः", "~\u{942}णाम\u{94d}";
        genitive_neuter: "~\u{941}णः|~ोः", "~\u{941}णोः|~\u{94d}वोः", "~\u{942}णाम\u{94d}";
        locative_masculine: "~ौ", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        locative_feminine: "~ौ|~\u{94d}वाम\u{94d}", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        locative_neuter: "~\u{941}णि|~ौ", "~\u{941}णोः|~\u{94d}वोः", "~\u{941}ष\u{941}";
        vocative_masculine: "~ो", "~\u{942}", "~वः";
        vocative_feminine: "~ो", "~\u{942}", "~वः";
        vocative_neuter: "~\u{941}|~ो", "~\u{941}णी", "~\u{942}णि";
        comparative: "~ीयस\u{94d}";
        superlative: "~िष\u{94d}ठ";
    ),
    // थिर: 13 words (शिशिर, अजिर, रुधिर, बधिर, मेधिर, अस्थिर).
    declension!("थिर", 13, "~िर";
        nominative_masculine: "~िरः", "~िरौ", "~िराः";
        nominative_feminine: "~िरा", "~िर\u{947}", "~िराः";
        nominative_neuter: "~िरम\u{94d}", "~िर\u{947}", "~िराणि";
        accusative_masculine: "~िरम\u{94d}", "~िरौ", "~िरान\u{94d}";
        accusative_feminine: "~िराम\u{94d}", "~िर\u{947}", "~िराः";
        accusative_neuter: "~िरम\u{94d}", "~िर\u{947}", "~िराणि";
        instrumental_masculine: "~िर\u{947}ण", "~िराभ\u{94d}याम\u{94d}", "~िर\u{948}ः";
        instrumental_feminine: "~िरया", "~िराभ\u{94d}याम\u{94d}", "~िराभिः";
        instrumental_neuter: "~िर\u{947}ण", "~िराभ\u{94d}याम\u{94d}", "~िर\u{948}ः";
        dative_masculine: "~िराय", "~िराभ\u{94d}याम\u{94d}", "~िर\u{947}भ\u{94d}यः";
        dative_feminine: "~िराय\u{948}", "~िराभ\u{94d}याम\u{94d}", "~िराभ\u{94d}यः";
        dative_neuter: "~िराय", "~िराभ\u{94d}याम\u{94d}", "~िर\u{947}भ\u{94d}यः";
        ablative_masculine: "~िरात\u{94d}", "~िराभ\u{94d}याम\u{94d}", "~िर\u{947}भ\u{94d}यः";
        ablative_feminine: "~िरायाः", "~िराभ\u{94d}याम\u{94d}", "~िराभ\u{94d}यः";
        ablative_neuter: "~िरात\u{94d}", "~िराभ\u{94d}याम\u{94d}", "~िर\u{947}भ\u{94d}यः";
        genitive_masculine: "~िरस\u{94d}य", "~िरयोः", "~िराणाम\u{94d}";
        genitive_feminine: "~िरायाः", "~िरयोः", "~िराणाम\u{94d}";
        genitive_neuter: "~िरस\u{94d}य", "~िरयोः", "~िराणाम\u{94d}";
        locative_masculine: "~िर\u{947}", "~िरयोः", "~िर\u{947}ष\u{941}";
        locative_feminine: "~िरायाम\u{94d}", "~िरयोः", "~िरास\u{941}";
        locative_neuter: "~िर\u{947}", "~िरयोः", "~िर\u{947}ष\u{941}";
        vocative_masculine: "~िर", "~िरौ", "~िराः";
        vocative_feminine: "~िर\u{947}", "~िर\u{947}", "~िराः";
        vocative_neuter: "~िर", "~िर\u{947}", "~िराणि";
        comparative: "~\u{947}यस\u{94d}";
        superlative: "~\u{947}ष\u{94d}ठ";
    ),
    // लघु: 11 words (अणु, बहु, लघु, अंहु, आशु, वसु).
    declension!("लघ\u{941}", 11, "~\u{941}";
        nominative_masculine: "~\u{941}ः", "~\u{942}", "~वः";
        nominative_feminine: "~\u{94d}वी", "~\u{94d}व\u{94d}यौ", "~\u{94d}व\u{94d}यः";
        nominative_neuter: "~\u{941}", "~\u{941}नी", "~\u{942}नि";
        accusative_masculine: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}न\u{94d}";
        accusative_feminine: "~\u{94d}वीम\u{94d}", "~\u{94d}व\u{94d}यौ", "~\u{94d}वीः";
        accusative_neuter: "~\u{941}", "~\u{941}नी", "~\u{942}नि";
        instrumental_masculine: "~\u{941}ना", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        instrumental_feminine: "~\u{94d}व\u{94d}या", "~\u{94d}वीभ\u{94d}याम\u{94d}", "~\u{94d}वीभिः";
        instrumental_neuter: "~\u{941}ना", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative_masculine: "~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        dative_feminine: "~\u{94d}व\u{94d}य\u{948}", "~\u{94d}वीभ\u{94d}याम\u{94d}", "~\u{94d}वीभ\u{94d}यः";
        dative_neuter: "~\u{941}न\u{947}|~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_masculine: "~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_feminine: "~\u{94d}व\u{94d}याः", "~\u{94d}वीभ\u{94d}याम\u{94d}", "~\u{94d}वीभ\u{94d}यः";
        ablative_neuter: "~\u{941}नः|~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive_masculine: "~ोः", "~\u{94d}वोः", "~\u{942}नाम\u{94d}";
        genitive_feminine: "~\u{94d}व\u{94d}याः", "~\u{94d}व\u{94d}योः", "~\u{94d}वीनाम\u{94d}";
        genitive_neuter: "~\u{941}नः|~ोः", "~\u{941}नोः|~\u{94d}वोः", "~\u{942}नाम\u{94d}";
        locative_masculine: "~ौ", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        locative_feminine: "~\u{94d}व\u{94d}याम\u{94d}", "~\u{94d}व\u{94d}योः", "~\u{94d}वीष\u{941}";
        locative_neuter: "~\u{941}नि|~ौ", "~\u{941}नोः|~\u{94d}वोः", "~\u{941}ष\u{941}";
        vocative_masculine: "~ो", "~\u{942}", "~वः";
        vocative_feminine: "~\u{94d}वि", "~\u{94d}व\u{94d}यौ", "~\u{94d}व\u{94d}यः";
        vocative_neuter: "~\u{941}|~ो", "~\u{941}नी", "~\u{942}नि";
        comparative: "~ीयस\u{94d}";
        superlative: "~िष\u{94d}ठ";
    ),
    // लीन: 11 words (दीन, कनीन, हीन, कालीन, कुलीन, पीन).
    declension!("लीन", 11, "~ीन";
        nominative_masculine: "~ीनः", "~ीनौ", "~ीनाः";
        nominative_feminine: "~ीना", "~ीन\u{947}", "~ीनाः";
        nominative_neuter: "~ीनम\u{94d}", "~ीन\u{947}", "~ीनानि";
        accusative_masculine: "~ीनम\u{94d}", "~ीनौ", "~ीनान\u{94d}";
        accusative_feminine: "~ीनाम\u{94d}", "~ीन\u{947}", "~ीनाः";
        accusative_neuter: "~ीनम\u{94d}", "~ीन\u{947}", "~ीनानि";
        instrumental_masculine: "~ीन\u{947}न", "~ीनाभ\u{94d}याम\u{94d}", "~ीन\u{948}ः";
        instrumental_feminine: "~ीनया", "~ीनाभ\u{94d}याम\u{94d}", "~ीनाभिः";
        instrumental_neuter: "~ीन\u{947}न", "~ीनाभ\u{94d}याम\u{94d}", "~ीन\u{948}ः";
        dative_masculine: "~ीनाय", "~ीनाभ\u{94d}याम\u{94d}", "~ीन\u{947}भ\u{94d}यः";
        dative_feminine: "~ीनाय\u{948}", "~ीनाभ\u{94d}याम\u{94d}", "~ीनाभ\u{94d}यः";
        dative_neuter: "~ीनाय", "~ीनाभ\u{94d}याम\u{94d}", "~ीन\u{947}भ\u{94d}यः";
        ablative_masculine: "~ीनात\u{94d}", "~ीनाभ\u{94d}याम\u{94d}", "~ीन\u{947}भ\u{94d}यः";
        ablative_feminine: "~ीनायाः", "~ीनाभ\u{94d}याम\u{94d}", "~ीनाभ\u{94d}यः";
        ablative_neuter: "~ीनात\u{94d}", "~ीनाभ\u{94d}याम\u{94d}", "~ीन\u{947}भ\u{94d}यः";
        genitive_masculine: "~ीनस\u{94d}य", "~ीनयोः", "~ीनानाम\u{94d}";
        genitive_feminine: "~ीनायाः", "~ीनयोः", "~ीनानाम\u{94d}";
        genitive_neuter: "~ीनस\u{94d}य", "~ीनयोः", "~ीनानाम\u{94d}";
        locative_masculine: "~ीन\u{947}", "~ीनयोः", "~ीन\u{947}ष\u{941}";
        locative_feminine: "~ीनायाम\u{94d}", "~ीनयोः", "~ीनास\u{941}";
        locative_neuter: "~ीन\u{947}", "~ीनयोः", "~ीन\u{947}ष\u{941}";
        vocative_masculine: "~ीन", "~ीनौ", "~ीनाः";
        vocative_feminine: "~ीन\u{947}", "~ीन\u{947}", "~ीनाः";
        vocative_neuter: "~ीन", "~ीन\u{947}", "~ीनानि";
        comparative: "~ीयस\u{94d}";
        superlative: "~िष\u{94d}ठ";
    ),
    // छिद: 9 words (उपस्थसद्, पुरुषाद्, सद्, विद्, छिद्, भिद्).
    declension!("छिद\u{94d}", 9, "~द\u{94d}";
        nominative_masculine: "~त\u{94d}", "~दौ", "~दः";
        nominative_feminine: "~त\u{94d}", "~दौ", "~दः";
        nominative_neuter: "~त\u{94d}", "~दी", "~न\u{94d}दि";
        accusative_masculine: "~दम\u{94d}", "~दौ", "~दः";
        accusative_feminine: "~दम\u{94d}", "~दौ", "~दः";
        accusative_neuter: "~त\u{94d}", "~दी", "~न\u{94d}दि";
        instrumental_masculine: "~दा", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        instrumental_feminine: "~दा", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        instrumental_neuter: "~दा", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative_masculine: "~द\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        dative_feminine: "~द\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        dative_neuter: "~द\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_masculine: "~दः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_feminine: "~दः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_neuter: "~दः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive_masculine: "~दः", "~दोः", "~दाम\u{94d}";
        genitive_feminine: "~दः", "~दोः", "~दाम\u{94d}";
        genitive_neuter: "~दः", "~दोः", "~दाम\u{94d}";
        locative_masculine: "~दि", "~दोः", "~त\u{94d}स\u{941}";
        locative_feminine: "~दि", "~दोः", "~त\u{94d}स\u{941}";
        locative_neuter: "~दि", "~दोः", "~त\u{94d}स\u{941}";
        vocative_masculine: "~त\u{94d}", "~दौ", "~दः";
        vocative_feminine: "~त\u{94d}", "~दौ", "~दः";
        vocative_neuter: "~त\u{94d}", "~दी", "~न\u{94d}दि";
        comparative: "~त\u{94d}तर";
        superlative: "~त\u{94d}तम";
    ),
    // तरुतृ: 8 words (यातृ, नेतृ, तरुतृ, गन्तृ, यन्तृ, पक्तृ).
    declension!("तर\u{941}त\u{943}", 8, "~\u{943}";
        nominative_masculine: "~ा", "~ारौ", "~ारः";
        nominative_feminine: "~\u{94d}री", "~\u{94d}र\u{94d}यौ", "~\u{94d}र\u{94d}यः";
        nominative_neuter: "~\u{943}", "~\u{943}णी", "~\u{944}णि";
        accusative_masculine: "~ारम\u{94d}", "~ारौ", "~\u{944}न\u{94d}";
        accusative_feminine: "~\u{94d}रीम\u{94d}", "~\u{94d}र\u{94d}यौ", "~\u{94d}रीः";
        accusative_neuter: "~\u{943}", "~\u{943}णी", "~\u{944}णि";
        instrumental_masculine: "~\u{94d}रा", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भिः";
        instrumental_feminine: "~\u{94d}र\u{94d}या", "~\u{94d}रीभ\u{94d}याम\u{94d}", "~\u{94d}रीभिः";
        instrumental_neuter: "~\u{943}णा|~\u{94d}रा", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भिः";
        dative_masculine: "~\u{94d}र\u{947}", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        dative_feminine: "~\u{94d}र\u{94d}य\u{948}", "~\u{94d}रीभ\u{94d}याम\u{94d}", "~\u{94d}रीभ\u{94d}यः";
        dative_neuter: "~\u{943}ण\u{947}|~\u{94d}र\u{947}", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        ablative_masculine: "~\u{941}ः", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        ablative_feminine: "~\u{94d}र\u{94d}याः", "~\u{94d}रीभ\u{94d}याम\u{94d}", "~\u{94d}रीभ\u{94d}यः";
        ablative_neuter: "~\u{943}णः|~\u{941}ः", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        genitive_masculine: "~\u{941}ः", "~\u{94d}रोः", "~\u{944}णाम\u{94d}";
        genitive_feminine: "~\u{94d}र\u{94d}याः", "~\u{94d}र\u{94d}योः", "~\u{94d}रीणाम\u{94d}";
        genitive_neuter: "~\u{943}णः|~\u{941}ः", "~\u{943}णोः|~\u{94d}रोः", "~\u{944}णाम\u{94d}";
        locative_masculine: "~रि", "~\u{94d}रोः", "~\u{943}ष\u{941}";
        locative_feminine: "~\u{94d}र\u{94d}याम\u{94d}", "~\u{94d}र\u{94d}योः", "~\u{94d}रीष\u{941}";
        locative_neuter: "~\u{943}णि|~रि", "~\u{943}णोः|~\u{94d}रोः", "~\u{943}ष\u{941}";
        vocative_masculine: "~ः", "~ारौ", "~ारः";
        vocative_feminine: "~\u{94d}रि", "~\u{94d}र\u{94d}यौ", "~\u{94d}र\u{94d}यः";
        vocative_neuter: "~\u{943}|~ः", "~\u{943}णी", "~\u{944}णि";
        comparative: "~ीयस\u{94d}";
        superlative: "~स\u{94d}तम";
    ),
    // अनूप: 8 words (अल्प, तप, सत्संकल्प, कश्यप, कल्प, अनूप).
    declension!("अन\u{942}प", 8, "~";
        nominative_masculine: "~ः", "~ौ", "~ाः";
        nominative_feminine: "~ा", "~\u{947}", "~ाः";
        nominative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        accusative_masculine: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        accusative_feminine: "~ाम\u{94d}", "~\u{947}", "~ाः";
        accusative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        instrumental_masculine: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        instrumental_feminine: "~या", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        instrumental_neuter: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative_masculine: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        dative_feminine: "~ाय\u{948}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        dative_neuter: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_masculine: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_feminine: "~ायाः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative_neuter: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive_masculine: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        genitive_feminine: "~ायाः", "~योः", "~ानाम\u{94d}";
        genitive_neuter: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        locative_masculine: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        locative_feminine: "~ायाम\u{94d}", "~योः", "~ास\u{941}";
        locative_neuter: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative_masculine: "~", "~ौ", "~ाः";
        vocative_feminine: "~\u{947}", "~\u{947}", "~ाः";
        vocative_neuter: "~", "~\u{947}", "~ानि";
        comparative: "~तर";
        superlative: "~िष\u{94d}ठ";
    ),
    // कारक: 8 words (कारक, तारक, सारक, क्षुद्रक, टेरक, दारक).
    declension!("कारक", 8, "~~";
        nominative_masculine: "~~ः", "~~ौ", "~~ाः";
        nominative_feminine: "~ि~ा", "~ि~\u{947}", "~ि~ाः";
        nominative_neuter: "~~म\u{94d}", "~~\u{947}", "~~ाणि";
        accusative_masculine: "~~म\u{94d}", "~~ौ", "~~ान\u{94d}";
        accusative_feminine: "~ि~ाम\u{94d}", "~ि~\u{947}", "~ि~ाः";
        accusative_neuter: "~~म\u{94d}", "~~\u{947}", "~~ाणि";
        instrumental_masculine: "~~\u{947}ण", "~~ाभ\u{94d}याम\u{94d}", "~~\u{948}ः";
        instrumental_feminine: "~ि~या", "~ि~ाभ\u{94d}याम\u{94d}", "~ि~ाभिः";
        instrumental_neuter: "~~\u{947}ण", "~~ाभ\u{94d}याम\u{94d}", "~~\u{948}ः";
        dative_masculine: "~~ाय", "~~ाभ\u{94d}याम\u{94d}", "~~\u{947}भ\u{94d}यः";
        dative_feminine: "~ि~ाय\u{948}", "~ि~ाभ\u{94d}याम\u{94d}", "~ि~ाभ\u{94d}यः";
        dative_neuter: "~~ाय", "~~ाभ\u{94d}याम\u{94d}", "~~\u{947}भ\u{94d}यः";
        ablative_masculine: "~~ात\u{94d}", "~~ाभ\u{94d}याम\u{94d}", "~~\u{947}भ\u{94d}यः";
        ablative_feminine: "~ि~ायाः", "~ि~ाभ\u{94d}याम\u{94d}", "~ि~ाभ\u{94d}यः";
        ablative_neuter: "~~ात\u{94d}", "~~ाभ\u{94d}याम\u{94d}", "~~\u{947}भ\u{94d}यः";
        genitive_masculine: "~~स\u{94d}य", "~~योः", "~~ाणाम\u{94d}";
        genitive_feminine: "~ि~ायाः", "~ि~योः", "~ि~ाणाम\u{94d}";
        genitive_neuter: "~~स\u{94d}य", "~~योः", "~~ाणाम\u{94d}";
        locative_masculine: "~~\u{947}", "~~योः", "~~\u{947}ष\u{941}";
        locative_feminine: "~ि~ायाम\u{94d}", "~ि~योः", "~ि~ास\u{941}";
        locative_neuter: "~~\u{947}", "~~योः", "~~\u{947}ष\u{941}";
        vocative_masculine: "~~", "~~ौ", "~~ाः";
        vocative_feminine: "~ि~\u{947}", "~ि~\u{947}", "~ि~ाः";
        vocative_neuter: "~~", "~~\u{947}", "~~ाणि";
        comparative: "~~";
        superlative: "~~";
    ),
    // रथी: 8 words (श्री, प्री, वी, वीतभी, रथी, शी).
    declension!("रथी", 8, "~ी";
        nominative_masculine: "~ीः", "~ियौ", "~ियः";
        nominative_feminine: "~ीः", "~ियौ", "~ियः";
        nominative_neuter: "~ि", "~िनी", "~ीनि";
        accusative_masculine: "~ियम\u{94d}", "~ियौ", "~ियः";
        accusative_feminine: "~ियम\u{94d}", "~ियौ", "~ियः";
        accusative_neuter: "~ि", "~िनी", "~ीनि";
        instrumental_masculine: "~िया", "~ीभ\u{94d}याम\u{94d}", "~ीभिः";
        instrumental_feminine: "~िया", "~ीभ\u{94d}याम\u{94d}", "~ीभिः";
        instrumental_neuter: "~िना", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative_masculine: "~िय\u{947}", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        dative_feminine: "~िय\u{947}", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        dative_neuter: "~िन\u{947}|~य\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_masculine: "~ियः", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        ablative_feminine: "~ियः", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        ablative_neuter: "~िनः|~\u{947}ः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive_masculine: "~ियः", "~ियोः", "~ियाम\u{94d}";
        genitive_feminine: "~ियः", "~ियोः", "~ियाम\u{94d}";
        genitive_neuter: "~िनः|~\u{947}ः", "~िनोः|~\u{94d}योः", "~ीनाम\u{94d}";
        locative_masculine: "~ियि", "~ियोः", "~ीष\u{941}";
        locative_feminine: "~ियि", "~ियोः", "~ीष\u{941}";
        locative_neuter: "~िनि|~ौ", "~िनोः|~\u{94d}योः", "~िष\u{941}";
        vocative_masculine: "~ीः", "~ियौ", "~ियः";
        vocative_feminine: "~ीः", "~ियौ", "~ियः";
        vocative_neuter: "~ि|~\u{947}", "~िनी", "~ीनि";
        comparative: "~\u{947}यस\u{94d}";
        superlative: "~तम";
    ),
    // सरवजित: 7 words (दशत्, हरित्, सर्वजित्, च्युत्, श्चुत्, रोहित्).
    declension!("सर\u{94d}वजित\u{94d}", 7, "~त\u{94d}";
        nominative_masculine: "~त\u{94d}", "~तौ", "~तः";
        nominative_feminine: "~त\u{94d}", "~तौ", "~तः";
        nominative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        accusative_masculine: "~तम\u{94d}", "~तौ", "~तः";
        accusative_feminine: "~तम\u{94d}", "~तौ", "~तः";
        accusative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        instrumental_masculine: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        instrumental_feminine: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        instrumental_neuter: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative_masculine: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        dative_feminine: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        dative_neuter: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_masculine: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_feminine: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_neuter: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive_masculine: "~तः", "~तोः", "~ताम\u{94d}";
        genitive_feminine: "~तः", "~तोः", "~ताम\u{94d}";
        genitive_neuter: "~तः", "~तोः", "~ताम\u{94d}";
        locative_masculine: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        locative_feminine: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        locative_neuter: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        vocative_masculine: "~त\u{94d}", "~तौ", "~तः";
        vocative_feminine: "~त\u{94d}", "~तौ", "~तः";
        vocative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        comparative: "~त\u{94d}तर";
        superlative: "~त\u{94d}तम";
    ),
    // परिय: 6 words (प्रिय, अन्ताराष्ट्रिय, क्षत्रिय, इन्द्रिय, नष्टक्रिय, अनिन्द्रिय).
    declension!("प\u{94d}रिय", 6, "~िय";
        nominative_masculine: "~ियः", "~ियौ", "~ियाः";
        nominative_feminine: "~िया", "~िय\u{947}", "~ियाः";
        nominative_neuter: "~ियम\u{94d}", "~िय\u{947}", "~ियाणि";
        accusative_masculine: "~ियम\u{94d}", "~ियौ", "~ियान\u{94d}";
        accusative_feminine: "~ियाम\u{94d}", "~िय\u{947}", "~ियाः";
        accusative_neuter: "~ियम\u{94d}", "~िय\u{947}", "~ियाणि";
        instrumental_masculine: "~िय\u{947}ण", "~ियाभ\u{94d}याम\u{94d}", "~िय\u{948}ः";
        instrumental_feminine: "~ियया", "~ियाभ\u{94d}याम\u{94d}", "~ियाभिः";
        instrumental_neuter: "~िय\u{947}ण", "~ियाभ\u{94d}याम\u{94d}", "~िय\u{948}ः";
        dative_masculine: "~ियाय", "~ियाभ\u{94d}याम\u{94d}", "~िय\u{947}भ\u{94d}यः";
        dative_feminine: "~ियाय\u{948}", "~ियाभ\u{94d}याम\u{94d}", "~ियाभ\u{94d}यः";
        dative_neuter: "~ियाय", "~ियाभ\u{94d}याम\u{94d}", "~िय\u{947}भ\u{94d}यः";
        ablative_masculine: "~ियात\u{94d}", "~ियाभ\u{94d}याम\u{94d}", "~िय\u{947}भ\u{94d}यः";
        ablative_feminine: "~ियायाः", "~ियाभ\u{94d}याम\u{94d}", "~ियाभ\u{94d}यः";
        ablative_neuter: "~ियात\u{94d}", "~ियाभ\u{94d}याम\u{94d}", "~िय\u{947}भ\u{94d}यः";
        genitive_masculine: "~ियस\u{94d}य", "~िययोः", "~ियाणाम\u{94d}";
        genitive_feminine: "~ियायाः", "~िययोः", "~ियाणाम\u{94d}";
        genitive_neuter: "~ियस\u{94d}य", "~िययोः", "~ियाणाम\u{94d}";
        locative_masculine: "~िय\u{947}", "~िययोः", "~िय\u{947}ष\u{941}";
        locative_feminine: "~ियायाम\u{94d}", "~िययोः", "~ियास\u{941}";
        locative_neuter: "~िय\u{947}", "~िययोः", "~िय\u{947}ष\u{941}";
        vocative_masculine: "~िय", "~ियौ", "~ियाः";
        vocative_feminine: "~िय\u{947}", "~िय\u{947}", "~ियाः";
        vocative_neuter: "~िय", "~िय\u{947}", "~ियाणि";
        comparative: "~\u{947}यस\u{94d}";
        superlative: "~\u{947}ष\u{94d}ठ";
    ),
    // असत: 6 words (बृहत्, पृषत्, अर्हत्, असत्, ऋहत्, अजानत्).
    declension!("असत\u{94d}", 6, "~त\u{94d}";
        nominative_masculine: "~न\u{94d}", "~न\u{94d}तौ", "~न\u{94d}तः";
        nominative_feminine: "~ती", "~त\u{94d}यौ", "~त\u{94d}यः";
        nominative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        accusative_masculine: "~न\u{94d}तम\u{94d}", "~न\u{94d}तौ", "~तः";
        accusative_feminine: "~तीम\u{94d}", "~त\u{94d}यौ", "~तीः";
        accusative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        instrumental_masculine: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        instrumental_feminine: "~त\u{94d}या", "~तीभ\u{94d}याम\u{94d}", "~तीभिः";
        instrumental_neuter: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative_masculine: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        dative_feminine: "~त\u{94d}य\u{948}", "~तीभ\u{94d}याम\u{94d}", "~तीभ\u{94d}यः";
        dative_neuter: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_masculine: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative_feminine: "~त\u{94d}याः", "~तीभ\u{94d}याम\u{94d}", "~तीभ\u{94d}यः";
        ablative_neuter: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive_masculine: "~तः", "~तोः", "~ताम\u{94d}";
        genitive_feminine: "~त\u{94d}याः", "~त\u{94d}योः", "~तीनाम\u{94d}";
        genitive_neuter: "~तः", "~तोः", "~ताम\u{94d}";
        locative_masculine: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        locative_feminine: "~त\u{94d}याम\u{94d}", "~त\u{94d}योः", "~तीष\u{941}";
        locative_neuter: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        vocative_masculine: "~न\u{94d}", "~न\u{94d}तौ", "~न\u{94d}तः";
        vocative_feminine: "~ति", "~त\u{94d}यौ", "~त\u{94d}यः";
        vocative_neuter: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        comparative: "~त\u{94d}तर";
        superlative: "~त\u{94d}तम";
    ),
    // महातमन: 6 words (शर्मन्, महात्मन्, भस्मन्, युग्मन्, अकृतात्मन्, ज्ञात्मन्).
    declension!("महात\u{94d}मन\u{94d}", 6, "~न\u{94d}";
        nominative_masculine: "~ा", "~ानौ", "~ानः";
        nominative_feminine: "~ा", "~ानौ", "~ानः";
        nominative_neuter: "~", "~नी", "~ानि";
        accusative_masculine: "~ानम\u{94d}", "~ानौ", "~नः";
        accusative_feminine: "~ानम\u{94d}", "~ानौ", "~नः";
        accusative_neuter: "~", "~नी", "~ानि";
        instrumental_masculine: "~ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        instrumental_feminine: "~ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        instrumental_neuter: "~ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative_masculine: "~न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        dative_feminine: "~न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        dative_neuter: "~न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative_masculine: "~नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative_feminine: "~नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative_neuter: "~नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive_masculine: "~नः", "~नोः", "~नाम\u{94d}";
        genitive_feminine: "~नः", "~नोः", "~नाम\u{94d}";
        genitive_neuter: "~नः", "~नोः", "~नाम\u{94d}";
        locative_masculine: "~नि", "~नोः", "~स\u{941}";
        locative_feminine: "~नि", "~नोः", "~स\u{941}";
        locative_neuter: "~नि", "~नोः", "~स\u{941}";
        vocative_masculine: "~न\u{94d}", "~ानौ", "~ानः";
        vocative_feminine: "~न\u{94d}", "~ानौ", "~ानः";
        vocative_neuter: "~न\u{94d}|~", "~नी", "~ानि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // राज: 6 words (भुज्, उशिज्, त्यज्, रुज्, सयुज्, राज्).
    declension!("राज\u{94d}", 6, "~ज\u{94d}";
        nominative_masculine: "~क\u{94d}", "~जौ", "~जः";
        nominative_feminine: "~क\u{94d}", "~जौ", "~जः";
        nominative_neuter: "~क\u{94d}", "~जी", "~ञ\u{94d}जि";
        accusative_masculine: "~जम\u{94d}", "~जौ", "~जः";
        accusative_feminine: "~जम\u{94d}", "~जौ", "~जः";
        accusative_neuter: "~क\u{94d}", "~जी", "~ञ\u{94d}जि";
        instrumental_masculine: "~जा", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भिः";
        instrumental_feminine: "~जा", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भिः";
        instrumental_neuter: "~जा", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भिः";
        dative_masculine: "~ज\u{947}", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        dative_feminine: "~ज\u{947}", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        dative_neuter: "~ज\u{947}", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        ablative_masculine: "~जः", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        ablative_feminine: "~जः", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        ablative_neuter: "~जः", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        genitive_masculine: "~जः", "~जोः", "~जाम\u{94d}";
        genitive_feminine: "~जः", "~जोः", "~जाम\u{94d}";
        genitive_neuter: "~जः", "~जोः", "~जाम\u{94d}";
        locative_masculine: "~जि", "~जोः", "~क\u{94d}ष\u{941}";
        locative_feminine: "~जि", "~जोः", "~क\u{94d}ष\u{941}";
        locative_neuter: "~जि", "~जोः", "~क\u{94d}ष\u{941}";
        vocative_masculine: "~क\u{94d}", "~जौ", "~जः";
        vocative_feminine: "~क\u{94d}", "~जौ", "~जः";
        vocative_neuter: "~क\u{94d}", "~जी", "~ञ\u{94d}जि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // पा: 6 words (सा, वर्चोदा, वर्चोधा, पा, रथेष्ठा, श्वपा).
    declension!("पा", 6, "~ा";
        nominative_masculine: "~ाः", "~ौ", "~ाः";
        nominative_feminine: "~ा", "~\u{947}", "~ाः";
        nominative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        accusative_masculine: "~ाम\u{94d}", "~ौ", "~ः";
        accusative_feminine: "~ाम\u{94d}", "~\u{947}", "~ाः";
        accusative_neuter: "~म\u{94d}", "~\u{947}", "~ानि";
        instrumental_masculine: "~ा", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        instrumental_feminine: "~या", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        instrumental_neuter: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative_masculine: "~\u{947}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        dative_feminine: "~ाय\u{948}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        dative_neuter: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative_masculine: "~ः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative_feminine: "~ायाः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative_neuter: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive_masculine: "~ः", "~ोः", "~ानाम\u{94d}";
        genitive_feminine: "~ायाः", "~योः", "~ानाम\u{94d}";
        genitive_neuter: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        locative_masculine: "~ि", "~ोः", "~ास\u{941}";
        locative_feminine: "~ायाम\u{94d}", "~योः", "~ास\u{941}";
        locative_neuter: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative_masculine: "~ाः", "~ौ", "~ाः";
        vocative_feminine: "~\u{947}", "~\u{947}", "~ाः";
        vocative_neuter: "~", "~\u{947}", "~ानि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // हरि: 6 words (हरि, अरि, करि, वध्रि, भूरि, सहुरि).
    declension!("हरि", 6, "~रि";
        nominative_masculine: "~रिः", "~री", "~रयः";
        nominative_feminine: "~रिः", "~री", "~रयः";
        nominative_neuter: "~रि", "~रिणी", "~रीणि";
        accusative_masculine: "~रिम\u{94d}", "~री", "~रीन\u{94d}";
        accusative_feminine: "~रिम\u{94d}", "~री", "~रीः";
        accusative_neuter: "~रि", "~रिणी", "~रीणि";
        instrumental_masculine: "~रिणा", "~रिभ\u{94d}याम\u{94d}", "~रिभिः";
        instrumental_feminine: "~र\u{94d}या", "~रिभ\u{94d}याम\u{94d}", "~रिभिः";
        instrumental_neuter: "~रिणा", "~रिभ\u{94d}याम\u{94d}", "~रिभिः";
        dative_masculine: "~रय\u{947}", "~रिभ\u{94d}याम\u{94d}", "~रिभ\u{94d}यः";
        dative_feminine: "~रय\u{947}", "~रिभ\u{94d}याम\u{94d}", "~रिभ\u{94d}यः";
        dative_neuter: "~रिण\u{947}", "~रिभ\u{94d}याम\u{94d}", "~रिभ\u{94d}यः";
        ablative_masculine: "~र\u{947}ः", "~रिभ\u{94d}याम\u{94d}", "~रिभ\u{94d}यः";
        ablative_feminine: "~र\u{947}ः", "~रिभ\u{94d}याम\u{94d}", "~रिभ\u{94d}यः";
        ablative_neuter: "~रिणः", "~रिभ\u{94d}याम\u{94d}", "~रिभ\u{94d}यः";
        genitive_masculine: "~र\u{947}ः", "~र\u{94d}योः", "~रीणाम\u{94d}";
        genitive_feminine: "~र\u{947}ः", "~र\u{94d}योः", "~रीणाम\u{94d}";
        genitive_neuter: "~रिणः", "~रिणोः", "~रीणाम\u{94d}";
        locative_masculine: "~रौ", "~र\u{94d}योः", "~रिष\u{941}";
        locative_feminine: "~रौ", "~र\u{94d}योः", "~रिष\u{941}";
        locative_neuter: "~रिणि", "~रिणोः", "~रिष\u{941}";
        vocative_masculine: "~र\u{947}", "~री", "~रयः";
        vocative_feminine: "~र\u{947}", "~री", "~रयः";
        vocative_neuter: "~रि", "~रिणी", "~रीणि";
        comparative: "~यस\u{94d}";
        superlative: "~यिष\u{94d}ठ";
    ),
    // पृश: 5 words (सदृश्, हृदिस्पृश्, कीदृश्, ईदृश्, पृश्).
    declension!("प\u{943}श\u{94d}", 5, "~श\u{94d}";
        nominative_masculine: "~क\u{94d}", "~शौ", "~शः";
        nominative_feminine: "~क\u{94d}", "~शौ", "~शः";
        nominative_neuter: "~क\u{94d}", "~शी", "~\u{902}शि";
        accusative_masculine: "~शम\u{94d}", "~शौ", "~शः";
        accusative_feminine: "~शम\u{94d}", "~शौ", "~शः";
        accusative_neuter: "~क\u{94d}", "~शी", "~\u{902}शि";
        instrumental_masculine: "~शा", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भिः";
        instrumental_feminine: "~शा", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भिः";
        instrumental_neuter: "~शा", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भिः";
        dative_masculine: "~श\u{947}", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        dative_feminine: "~श\u{947}", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        dative_neuter: "~श\u{947}", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        ablative_masculine: "~शः", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        ablative_feminine: "~शः", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        ablative_neuter: "~शः", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        genitive_masculine: "~शः", "~शोः", "~शाम\u{94d}";
        genitive_feminine: "~शः", "~शोः", "~शाम\u{94d}";
        genitive_neuter: "~शः", "~शोः", "~शाम\u{94d}";
        locative_masculine: "~शि", "~शोः", "~क\u{94d}ष\u{941}";
        locative_feminine: "~शि", "~शोः", "~क\u{94d}ष\u{941}";
        locative_neuter: "~शि", "~शोः", "~क\u{94d}ष\u{941}";
        vocative_masculine: "~क\u{94d}", "~शौ", "~शः";
        vocative_feminine: "~क\u{94d}", "~शौ", "~शः";
        vocative_neuter: "~क\u{94d}", "~शी", "~\u{902}शि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // नीरोग: 4 words (दीर्घ, शीर्षक, धीर्य, नीरोग).
    declension!("नीरोग", 4, "~ीर~~";
        nominative_masculine: "~ीर~~ः", "~ीर~~ौ", "~ीर~~ाः";
        nominative_feminine: "~ीर~~ा", "~ीर~~\u{947}", "~ीर~~ाः";
        nominative_neuter: "~ीर~~म\u{94d}", "~ीर~~\u{947}", "~ीर~~ाणि";
        accusative_masculine: "~ीर~~म\u{94d}", "~ीर~~ौ", "~ीर~~ान\u{94d}";
        accusative_feminine: "~ीर~~ाम\u{94d}", "~ीर~~\u{947}", "~ीर~~ाः";
        accusative_neuter: "~ीर~~म\u{94d}", "~ीर~~\u{947}", "~ीर~~ाणि";
        instrumental_masculine: "~ीर~~\u{947}ण", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~\u{948}ः";
        instrumental_feminine: "~ीर~~या", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~ाभिः";
        instrumental_neuter: "~ीर~~\u{947}ण", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~\u{948}ः";
        dative_masculine: "~ीर~~ाय", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~\u{947}भ\u{94d}यः";
        dative_feminine: "~ीर~~ाय\u{948}", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~ाभ\u{94d}यः";
        dative_neuter: "~ीर~~ाय", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~\u{947}भ\u{94d}यः";
        ablative_masculine: "~ीर~~ात\u{94d}", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~\u{947}भ\u{94d}यः";
        ablative_feminine: "~ीर~~ायाः", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~ाभ\u{94d}यः";
        ablative_neuter: "~ीर~~ात\u{94d}", "~ीर~~ाभ\u{94d}याम\u{94d}", "~ीर~~\u{947}भ\u{94d}यः";
        genitive_masculine: "~ीर~~स\u{94d}य", "~ीर~~योः", "~ीर~~ाणाम\u{94d}";
        genitive_feminine: "~ीर~~ायाः", "~ीर~~योः", "~ीर~~ाणाम\u{94d}";
        genitive_neuter: "~ीर~~स\u{94d}य", "~ीर~~योः", "~ीर~~ाणाम\u{94d}";
        locative_masculine: "~ीर~~\u{947}", "~ीर~~योः", "~ीर~~\u{947}ष\u{941}";
        locative_feminine: "~ीर~~ायाम\u{94d}", "~ीर~~योः", "~ीर~~ास\u{941}";
        locative_neuter: "~ीर~~\u{947}", "~ीर~~योः", "~ीर~~\u{947}ष\u{941}";
        vocative_masculine: "~ीर~~", "~ीर~~ौ", "~ीर~~ाः";
        vocative_feminine: "~ीर~~\u{947}", "~ीर~~\u{947}", "~ीर~~ाः";
        vocative_neuter: "~ीर~~", "~ीर~~\u{947}", "~ीर~~ाणि";
        comparative: "~~रा~ीयस\u{94d}";
        superlative: "~~रा~िष\u{94d}ठ";
    ),
    // अमनस: 4 words (अमनस्, क्लान्तमनस्, रंहस्, तरस्).
    declension!("अमनस\u{94d}", 4, "~स\u{94d}";
        nominative_masculine: "~ाः", "~सौ", "~सः";
        nominative_feminine: "~सी", "~स\u{94d}यौ", "~स\u{94d}यः";
        nominative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        accusative_masculine: "~सम\u{94d}", "~सौ", "~सः";
        accusative_feminine: "~सीम\u{94d}", "~स\u{94d}यौ", "~सीः";
        accusative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        instrumental_masculine: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        instrumental_feminine: "~स\u{94d}या", "~सीभ\u{94d}याम\u{94d}", "~सीभिः";
        instrumental_neuter: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        dative_masculine: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        dative_feminine: "~स\u{94d}य\u{948}", "~सीभ\u{94d}याम\u{94d}", "~सीभ\u{94d}यः";
        dative_neuter: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative_masculine: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative_feminine: "~स\u{94d}याः", "~सीभ\u{94d}याम\u{94d}", "~सीभ\u{94d}यः";
        ablative_neuter: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        genitive_masculine: "~सः", "~सोः", "~साम\u{94d}";
        genitive_feminine: "~स\u{94d}याः", "~स\u{94d}योः", "~सीनाम\u{94d}";
        genitive_neuter: "~सः", "~सोः", "~साम\u{94d}";
        locative_masculine: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        locative_feminine: "~स\u{94d}याम\u{94d}", "~स\u{94d}योः", "~सीष\u{941}";
        locative_neuter: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        vocative_masculine: "~ः", "~सौ", "~सः";
        vocative_feminine: "~सि", "~स\u{94d}यौ", "~स\u{94d}यः";
        vocative_neuter: "~ः", "~सी", "~ा\u{902}सि";
        comparative: "~ीयस\u{94d}";
        superlative: "~स\u{94d}तम";
    ),
    // बहुवरीहि: 3 words (ब्राह्मि, बहुव्रीहि, द्रापि).
    declension!("बह\u{941}व\u{94d}रीहि", 3, "~ि";
        nominative_masculine: "~िः", "~ी", "~यः";
        nominative_feminine: "~िः", "~ी", "~यः";
        nominative_neuter: "~ि", "~िणी", "~ीणि";
        accusative_masculine: "~िम\u{94d}", "~ी", "~ीन\u{94d}";
        accusative_feminine: "~िम\u{94d}", "~ी", "~ीः";
        accusative_neuter: "~ि", "~िणी", "~ीणि";
        instrumental_masculine: "~िणा", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        instrumental_feminine: "~\u{94d}या", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        instrumental_neuter: "~िणा", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative_masculine: "~य\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        dative_feminine: "~य\u{947}|~\u{94d}य\u{948}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        dative_neuter: "~िण\u{947}|~य\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_masculine: "~\u{947}ः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_feminine: "~\u{947}ः|~\u{94d}याः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative_neuter: "~िणः|~\u{947}ः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive_masculine: "~\u{947}ः", "~\u{94d}योः", "~ीणाम\u{94d}";
        genitive_feminine: "~\u{947}ः|~\u{94d}याः", "~\u{94d}योः", "~ीणाम\u{94d}";
        genitive_neuter: "~िणः|~\u{947}ः", "~िणोः|~\u{94d}योः", "~ीणाम\u{94d}";
        locative_masculine: "~ौ", "~\u{94d}योः", "~िष\u{941}";
        locative_feminine: "~ौ|~\u{94d}याम\u{94d}", "~\u{94d}योः", "~िष\u{941}";
        locative_neuter: "~िणि|~ौ", "~िणोः|~\u{94d}योः", "~िष\u{941}";
        vocative_masculine: "~\u{947}", "~ी", "~यः";
        vocative_feminine: "~\u{947}", "~ी", "~यः";
        vocative_neuter: "~ि|~\u{947}", "~िणी", "~ीणि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // रघु: 3 words (अगुरु, प्रभु, रघु).
    declension!("रघ\u{941}", 3, "~\u{941}";
        nominative_masculine: "~\u{941}ः", "~\u{942}", "~वः";
        nominative_feminine: "~\u{94d}वी", "~\u{94d}व\u{94d}यौ", "~\u{94d}व\u{94d}यः";
        nominative_neuter: "~\u{941}", "~\u{941}णी", "~\u{942}णि";
        accusative_masculine: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}न\u{94d}";
        accusative_feminine: "~\u{94d}वीम\u{94d}", "~\u{94d}व\u{94d}यौ", "~\u{94d}वीः";
        accusative_neuter: "~\u{941}", "~\u{941}णी", "~\u{942}णि";
        instrumental_masculine: "~\u{941}णा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        instrumental_feminine: "~\u{94d}व\u{94d}या", "~\u{94d}वीभ\u{94d}याम\u{94d}", "~\u{94d}वीभिः";
        instrumental_neuter: "~\u{941}णा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative_masculine: "~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        dative_feminine: "~\u{94d}व\u{94d}य\u{948}", "~\u{94d}वीभ\u{94d}याम\u{94d}", "~\u{94d}वीभ\u{94d}यः";
        dative_neuter: "~\u{941}ण\u{947}|~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_masculine: "~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative_feminine: "~\u{94d}व\u{94d}याः", "~\u{94d}वीभ\u{94d}याम\u{94d}", "~\u{94d}वीभ\u{94d}यः";
        ablative_neuter: "~\u{941}णः|~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive_masculine: "~ोः", "~\u{94d}वोः", "~\u{942}णाम\u{94d}";
        genitive_feminine: "~\u{94d}व\u{94d}याः", "~\u{94d}व\u{94d}योः", "~\u{94d}वीणाम\u{94d}";
        genitive_neuter: "~\u{941}णः|~ोः", "~\u{941}णोः|~\u{94d}वोः", "~\u{942}णाम\u{94d}";
        locative_masculine: "~ौ", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        locative_feminine: "~\u{94d}व\u{94d}याम\u{94d}", "~\u{94d}व\u{94d}योः", "~\u{94d}वीष\u{941}";
        locative_neuter: "~\u{941}णि|~ौ", "~\u{941}णोः|~\u{94d}वोः", "~\u{941}ष\u{941}";
        vocative_masculine: "~ो", "~\u{942}", "~वः";
        vocative_feminine: "~\u{94d}वि", "~\u{94d}व\u{94d}यौ", "~\u{94d}व\u{94d}यः";
        vocative_neuter: "~\u{941}|~ो", "~\u{941}णी", "~\u{942}णि";
        comparative: "~ीयस\u{94d}";
        superlative: "~िष\u{94d}ठ";
    ),
];
