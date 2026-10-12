//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the declension's it is like):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
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
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
                flexion::cell!($c2), "\t",
                flexion::cell!($c3), "\t",
                flexion::cell!($c4), "\t",
                flexion::cell!($c5), "\t",
                flexion::cell!($c6), "\t",
                flexion::cell!($c7), "\t",
                flexion::cell!($c8), "\t",
                flexion::cell!($c9), "\t",
                flexion::cell!($c10), "\t",
                flexion::cell!($c11), "\t",
                flexion::cell!($c12), "\t",
                flexion::cell!($c13), "\t",
                flexion::cell!($c14), "\t",
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
                flexion::cell!($c19), "\t",
                flexion::cell!($c20), "\t",
                flexion::cell!($c21), "\t",
                flexion::cell!($c22), "\t",
                flexion::cell!($c23), "\t",
                flexion::cell!($c24), "\t",
                flexion::cell!($c25), "\t",
                flexion::cell!($c26), "\t",
                flexion::cell!($c27), "\t",
                flexion::cell!($c28), "\t",
                flexion::cell!($c29), "\t",
                flexion::cell!($c30), "\t",
                flexion::cell!($c31), "\t",
                flexion::cell!($c32), "\t",
                flexion::cell!($c33), "\t",
                flexion::cell!($c34), "\t",
                flexion::cell!($c35), "\t",
                flexion::cell!($c36), "\t",
                flexion::cell!($c37), "\t",
                flexion::cell!($c38), "\t",
                flexion::cell!($c39), "\t",
                flexion::cell!($c40), "\t",
                flexion::cell!($c41), "\t",
                flexion::cell!($c42), "\t",
                flexion::cell!($c43), "\t",
                flexion::cell!($c44), "\t",
                flexion::cell!($c45), "\t",
                flexion::cell!($c46), "\t",
                flexion::cell!($c47), "\t",
                flexion::cell!($c48), "\t",
                flexion::cell!($c49), "\t",
                flexion::cell!($c50), "\t",
                flexion::cell!($c51), "\t",
                flexion::cell!($c52), "\t",
                flexion::cell!($c53), "\t",
                flexion::cell!($c54), "\t",
                flexion::cell!($c55), "\t",
                flexion::cell!($c56), "\t",
                flexion::cell!($c57), "\t",
                flexion::cell!($c58), "\t",
                flexion::cell!($c59), "\t",
                flexion::cell!($c60), "\t",
                flexion::cell!($c61), "\t",
                flexion::cell!($c62), "\t",
                flexion::cell!($c63), "\t",
                flexion::cell!($c64), "\t",
                flexion::cell!($c65), "\t",
                flexion::cell!($c66), "\t",
                flexion::cell!($c67), "\t",
                flexion::cell!($c68), "\t",
                flexion::cell!($c69), "\t",
                flexion::cell!($c70), "\t",
                flexion::cell!($c71), "\t",
                flexion::cell!($c72), "\t",
                flexion::cell!($c73), "\t",
            )),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `धृत`: 1,070 words.
    धृत,
    /// Like `अवर`: 258 words.
    अवर,
    /// Like `मूढ`: 183 words.
    मूढ,
    /// Like `हार`: 60 words.
    हार,
    /// Like `भाविन्`: 59 words.
    भाविन,
    /// Like `मनु`: 34 words.
    मनु,
    /// Like `शुचि`: 32 words.
    शुचि,
    /// Like `रसवत्`: 31 words.
    रसवत,
    /// Like `हितकर`: 23 words.
    हितकर,
    /// Like `घटक`: 22 words.
    घटक,
    /// Like `धायस्`: 19 words.
    धायस,
    /// Like `भारिन्`: 18 words.
    भारिन,
    /// Like `ऋतावन्`: 15 words.
    ऋतावन,
    /// Like `ज्यायस्`: 15 words.
    जयायस,
    /// Like `धृषु`: 13 words.
    धृषु,
    /// Like `लघु`: 11 words.
    लघु,
    /// Like `छिद्`: 9 words.
    छिद,
    /// Like `तरुतृ`: 8 words.
    तरुतृ,
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
    /// Like `उशिज्`: 6 words.
    उशिज,
    /// Like `पा`: 6 words.
    पा,
    /// Like `हरि`: 6 words.
    हरि,
    /// Like `स्पृश्`: 5 words.
    सपृश,
    /// Like `नीरोग`: 4 words.
    नीरोग,
    /// Like `अमनस्`: 4 words.
    अमनस,
    /// Like `वामन`: 3 words.
    वामन,
    /// Like `बहुव्रीहि`: 3 words.
    बहुवरीहि,
    /// Like `रघु`: 3 words.
    रघु,
    /// Like `चिर`: 1 words.
    चिर,
    /// Like `लीन`: 1 words.
    लीन,
    /// Like `अनूप`: 1 words.
    अनूप,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 36] = [
        Declension::धृत,
        Declension::अवर,
        Declension::मूढ,
        Declension::हार,
        Declension::भाविन,
        Declension::मनु,
        Declension::शुचि,
        Declension::रसवत,
        Declension::हितकर,
        Declension::घटक,
        Declension::धायस,
        Declension::भारिन,
        Declension::ऋतावन,
        Declension::जयायस,
        Declension::धृषु,
        Declension::लघु,
        Declension::छिद,
        Declension::तरुतृ,
        Declension::कारक,
        Declension::रथी,
        Declension::सरवजित,
        Declension::परिय,
        Declension::असत,
        Declension::महातमन,
        Declension::उशिज,
        Declension::पा,
        Declension::हरि,
        Declension::सपृश,
        Declension::नीरोग,
        Declension::अमनस,
        Declension::वामन,
        Declension::बहुवरीहि,
        Declension::रघु,
        Declension::चिर,
        Declension::लीन,
        Declension::अनूप,
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
    // धृत: 1,070 words (विश्व, ज, द, कमल, बुद्ध, स्निह्).
    declension!("धृत", 1070, "~";
        nominative_masculine: "~ः", "~ौ", "~ाः";
        nominative_feminine: "~ा", "~े", "~ाः";
        nominative_neuter: "~म्", "~े", "~ानि";
        accusative_masculine: "~म्", "~ौ", "~ान्";
        accusative_feminine: "~ाम्", "~े", "~ाः";
        accusative_neuter: "~म्", "~े", "~ानि";
        instrumental_masculine: "~ेन", "~ाभ्याम्", "~ैः";
        instrumental_feminine: "~या", "~ाभ्याम्", "~ाभिः";
        instrumental_neuter: "~ेन", "~ाभ्याम्", "~ैः";
        dative_masculine: "~ाय", "~ाभ्याम्", "~ेभ्यः";
        dative_feminine: "~ायै", "~ाभ्याम्", "~ाभ्यः";
        dative_neuter: "~ाय", "~ाभ्याम्", "~ेभ्यः";
        ablative_masculine: "~ात्", "~ाभ्याम्", "~ेभ्यः";
        ablative_feminine: "~ायाः", "~ाभ्याम्", "~ाभ्यः";
        ablative_neuter: "~ात्", "~ाभ्याम्", "~ेभ्यः";
        genitive_masculine: "~स्य", "~योः", "~ानाम्";
        genitive_feminine: "~ायाः", "~योः", "~ानाम्";
        genitive_neuter: "~स्य", "~योः", "~ानाम्";
        locative_masculine: "~े", "~योः", "~ेषु";
        locative_feminine: "~ायाम्", "~योः", "~ासु";
        locative_neuter: "~े", "~योः", "~ेषु";
        vocative_masculine: "~", "~ौ", "~ाः";
        vocative_feminine: "~े", "~े", "~ाः";
        vocative_neuter: "~", "~े", "~ानि";
        comparative: "~तर";
        superlative: "~तम";
    ),
    // अवर: 258 words (मूर्ख, मधुर, ताम्र, सूर्य, कार्य, निर्मम). Like धृत, except in 8 cells: nominative_neuter pl, accusative_neuter pl, instrumental_masculine sg, instrumental_neuter sg, genitive_masculine pl, genitive_feminine pl, ….
    declension!("अवर", 258, "~", like धृत;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, "~ाणि";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, "~ाणि";
        instrumental_masculine: "~ेण", _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: "~ेण", _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, "~ाणाम्";
        genitive_feminine: _, _, "~ाणाम्";
        genitive_neuter: _, _, "~ाणाम्";
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, "~ाणि";
        comparative: _;
        superlative: _;
    ),
    // मूढ: 183 words (भारत, देव, गणित, विशेषण, लेखन, दैव). Like धृत, except in 25 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, accusative_feminine sg, accusative_feminine du, accusative_feminine pl, ….
    declension!("मूढ", 183, "~", like धृत;
        nominative_masculine: _, _, _;
        nominative_feminine: "~ी", "~्यौ", "~्यः";
        nominative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~ीम्", "~्यौ", "~ीः";
        accusative_neuter: _, _, _;
        instrumental_masculine: _, _, _;
        instrumental_feminine: "~्या", "~ीभ्याम्", "~ीभिः";
        instrumental_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~्यै", "~ीभ्याम्", "~ीभ्यः";
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: "~्याः", "~ीभ्याम्", "~ीभ्यः";
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~्याः", "~्योः", "~ीनाम्";
        genitive_neuter: _, _, _;
        locative_masculine: _, _, _;
        locative_feminine: "~्याम्", "~्योः", "~ीषु";
        locative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~ि", "~्यौ", "~्यः";
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: _;
    ),
    // हार: 60 words (सर्प, कार, क्षौर, गौर, अक्ष, ऋक्ष). Like मूढ, except in 9 cells: nominative_neuter pl, accusative_neuter pl, instrumental_masculine sg, instrumental_neuter sg, genitive_masculine pl, genitive_feminine pl, ….
    declension!("हार", 60, "~", like मूढ;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, "~ाणि";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, "~ाणि";
        instrumental_masculine: "~ेण", _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: "~ेण", _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, "~ाणाम्";
        genitive_feminine: _, _, "~ीणाम्";
        genitive_neuter: _, _, "~ाणाम्";
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, "~ाणि";
        comparative: _;
        superlative: derived;
    ),
    // भाविन: 59 words (हस्तिन्, मायिन्, केशिन्, एकाकिन्, ज्ञानिन्, घातिन्).
    declension!("भाविन्", 59, "~िन्";
        nominative_masculine: "~ी", "~िनौ", "~िनः";
        nominative_feminine: "~िनी", "~िन्यौ", "~िन्यः";
        nominative_neuter: "~ि", "~िनी", "~ीनि";
        accusative_masculine: "~िनम्", "~िनौ", "~िनः";
        accusative_feminine: "~िनीम्", "~िन्यौ", "~िनीः";
        accusative_neuter: "~ि", "~िनी", "~ीनि";
        instrumental_masculine: "~िना", "~िभ्याम्", "~िभिः";
        instrumental_feminine: "~िन्या", "~िनीभ्याम्", "~िनीभिः";
        instrumental_neuter: "~िना", "~िभ्याम्", "~िभिः";
        dative_masculine: "~िने", "~िभ्याम्", "~िभ्यः";
        dative_feminine: "~िन्यै", "~िनीभ्याम्", "~िनीभ्यः";
        dative_neuter: "~िने", "~िभ्याम्", "~िभ्यः";
        ablative_masculine: "~िनः", "~िभ्याम्", "~िभ्यः";
        ablative_feminine: "~िन्याः", "~िनीभ्याम्", "~िनीभ्यः";
        ablative_neuter: "~िनः", "~िभ्याम्", "~िभ्यः";
        genitive_masculine: "~िनः", "~िनोः", "~िनाम्";
        genitive_feminine: "~िन्याः", "~िन्योः", "~िनीनाम्";
        genitive_neuter: "~िनः", "~िनोः", "~िनाम्";
        locative_masculine: "~िनि", "~िनोः", "~िषु";
        locative_feminine: "~िन्याम्", "~िन्योः", "~िनीषु";
        locative_neuter: "~िनि", "~िनोः", "~िषु";
        vocative_masculine: "~िन्", "~िनौ", "~िनः";
        vocative_feminine: "~िनि", "~िन्यौ", "~िन्यः";
        vocative_neuter: "~ि|~िन्", "~िनी", "~ीनि";
        comparative: derived;
        superlative: derived;
    ),
    // मनु: 34 words (वायु, पृदाकुसानु, मनु, स्थाणु, पाण्डु, सेतु).
    declension!("मनु", 34, "~ु";
        nominative_masculine: "~ुः", "~ू", "~वः";
        nominative_feminine: "~ुः", "~ू", "~वः";
        nominative_neuter: "~ु", "~ुनी", "~ूनि";
        accusative_masculine: "~ुम्", "~ू", "~ून्";
        accusative_feminine: "~ुम्", "~ू", "~ूः";
        accusative_neuter: "~ु", "~ुनी", "~ूनि";
        instrumental_masculine: "~ुना", "~ुभ्याम्", "~ुभिः";
        instrumental_feminine: "~्वा", "~ुभ्याम्", "~ुभिः";
        instrumental_neuter: "~ुना", "~ुभ्याम्", "~ुभिः";
        dative_masculine: "~वे", "~ुभ्याम्", "~ुभ्यः";
        dative_feminine: "~वे|~्वै", "~ुभ्याम्", "~ुभ्यः";
        dative_neuter: "~ुने|~वे", "~ुभ्याम्", "~ुभ्यः";
        ablative_masculine: "~ोः", "~ुभ्याम्", "~ुभ्यः";
        ablative_feminine: "~ोः|~्वाः", "~ुभ्याम्", "~ुभ्यः";
        ablative_neuter: "~ुनः|~ोः", "~ुभ्याम्", "~ुभ्यः";
        genitive_masculine: "~ोः", "~्वोः", "~ूनाम्";
        genitive_feminine: "~ोः|~्वाः", "~्वोः", "~ूनाम्";
        genitive_neuter: "~ुनः|~ोः", "~ुनोः|~्वोः", "~ूनाम्";
        locative_masculine: "~ौ", "~्वोः", "~ुषु";
        locative_feminine: "~ौ|~्वाम्", "~्वोः", "~ुषु";
        locative_neuter: "~ुनि|~ौ", "~ुनोः|~्वोः", "~ुषु";
        vocative_masculine: "~ो", "~ू", "~वः";
        vocative_feminine: "~ो", "~ू", "~वः";
        vocative_neuter: "~ु|~ो", "~ुनी", "~ूनि";
        comparative: derived;
        superlative: derived;
    ),
    // शुचि: 32 words (संधि, कवि, अभि, गभस्ति, पुलस्ति, वसति).
    declension!("शुचि", 32, "~ि";
        nominative_masculine: "~िः", "~ी", "~यः";
        nominative_feminine: "~िः", "~ी", "~यः";
        nominative_neuter: "~ि", "~िनी", "~ीनि";
        accusative_masculine: "~िम्", "~ी", "~ीन्";
        accusative_feminine: "~िम्", "~ी", "~ीः";
        accusative_neuter: "~ि", "~िनी", "~ीनि";
        instrumental_masculine: "~िना", "~िभ्याम्", "~िभिः";
        instrumental_feminine: "~्या", "~िभ्याम्", "~िभिः";
        instrumental_neuter: "~िना", "~िभ्याम्", "~िभिः";
        dative_masculine: "~ये", "~िभ्याम्", "~िभ्यः";
        dative_feminine: "~ये|~्यै", "~िभ्याम्", "~िभ्यः";
        dative_neuter: "~िने|~ये", "~िभ्याम्", "~िभ्यः";
        ablative_masculine: "~ेः", "~िभ्याम्", "~िभ्यः";
        ablative_feminine: "~ेः|~्याः", "~िभ्याम्", "~िभ्यः";
        ablative_neuter: "~िनः|~ेः", "~िभ्याम्", "~िभ्यः";
        genitive_masculine: "~ेः", "~्योः", "~ीनाम्";
        genitive_feminine: "~ेः|~्याः", "~्योः", "~ीनाम्";
        genitive_neuter: "~िनः|~ेः", "~िनोः|~्योः", "~ीनाम्";
        locative_masculine: "~ौ", "~्योः", "~िषु";
        locative_feminine: "~ौ|~्याम्", "~्योः", "~िषु";
        locative_neuter: "~िनि|~ौ", "~िनोः|~्योः", "~िषु";
        vocative_masculine: "~े", "~ी", "~यः";
        vocative_feminine: "~े", "~ी", "~यः";
        vocative_neuter: "~ि|~े", "~िनी", "~ीनि";
        comparative: derived;
        superlative: derived;
    ),
    // रसवत: 31 words (जगत्, अंशुमत्, हिमवत्, महत्, भगवत्, अन्तवत्).
    declension!("रसवत्", 31, "~त्";
        nominative_masculine: "~ान्", "~न्तौ", "~न्तः";
        nominative_feminine: "~ती", "~त्यौ", "~त्यः";
        nominative_neuter: "~त्", "~ती", "~न्ति";
        accusative_masculine: "~न्तम्", "~न्तौ", "~तः";
        accusative_feminine: "~तीम्", "~त्यौ", "~तीः";
        accusative_neuter: "~त्", "~ती", "~न्ति";
        instrumental_masculine: "~ता", "~द्भ्याम्", "~द्भिः";
        instrumental_feminine: "~त्या", "~तीभ्याम्", "~तीभिः";
        instrumental_neuter: "~ता", "~द्भ्याम्", "~द्भिः";
        dative_masculine: "~ते", "~द्भ्याम्", "~द्भ्यः";
        dative_feminine: "~त्यै", "~तीभ्याम्", "~तीभ्यः";
        dative_neuter: "~ते", "~द्भ्याम्", "~द्भ्यः";
        ablative_masculine: "~तः", "~द्भ्याम्", "~द्भ्यः";
        ablative_feminine: "~त्याः", "~तीभ्याम्", "~तीभ्यः";
        ablative_neuter: "~तः", "~द्भ्याम्", "~द्भ्यः";
        genitive_masculine: "~तः", "~तोः", "~ताम्";
        genitive_feminine: "~त्याः", "~त्योः", "~तीनाम्";
        genitive_neuter: "~तः", "~तोः", "~ताम्";
        locative_masculine: "~ति", "~तोः", "~त्सु";
        locative_feminine: "~त्याम्", "~त्योः", "~तीषु";
        locative_neuter: "~ति", "~तोः", "~त्सु";
        vocative_masculine: "~न्", "~न्तौ", "~न्तः";
        vocative_feminine: "~ति", "~त्यौ", "~त्यः";
        vocative_neuter: "~त्", "~ती", "~न्ति";
        comparative: derived;
        superlative: derived;
    ),
    // हितकर: 23 words (अरित्र, अग्निहोत्र, चित्र, सिंहदंष्ट्र, दरिद्र, विचित्र).
    declension!("हितकर", 23, "~ि~~र";
        nominative_masculine: "~ि~~रः", "~ि~~रौ", "~ि~~राः";
        nominative_feminine: "~ि~~रा", "~ि~~रे", "~ि~~राः";
        nominative_neuter: "~ि~~रम्", "~ि~~रे", "~ि~~राणि";
        accusative_masculine: "~ि~~रम्", "~ि~~रौ", "~ि~~रान्";
        accusative_feminine: "~ि~~राम्", "~ि~~रे", "~ि~~राः";
        accusative_neuter: "~ि~~रम्", "~ि~~रे", "~ि~~राणि";
        instrumental_masculine: "~ि~~रेण", "~ि~~राभ्याम्", "~ि~~रैः";
        instrumental_feminine: "~ि~~रया", "~ि~~राभ्याम्", "~ि~~राभिः";
        instrumental_neuter: "~ि~~रेण", "~ि~~राभ्याम्", "~ि~~रैः";
        dative_masculine: "~ि~~राय", "~ि~~राभ्याम्", "~ि~~रेभ्यः";
        dative_feminine: "~ि~~रायै", "~ि~~राभ्याम्", "~ि~~राभ्यः";
        dative_neuter: "~ि~~राय", "~ि~~राभ्याम्", "~ि~~रेभ्यः";
        ablative_masculine: "~ि~~रात्", "~ि~~राभ्याम्", "~ि~~रेभ्यः";
        ablative_feminine: "~ि~~रायाः", "~ि~~राभ्याम्", "~ि~~राभ्यः";
        ablative_neuter: "~ि~~रात्", "~ि~~राभ्याम्", "~ि~~रेभ्यः";
        genitive_masculine: "~ि~~रस्य", "~ि~~रयोः", "~ि~~राणाम्";
        genitive_feminine: "~ि~~रायाः", "~ि~~रयोः", "~ि~~राणाम्";
        genitive_neuter: "~ि~~रस्य", "~ि~~रयोः", "~ि~~राणाम्";
        locative_masculine: "~ि~~रे", "~ि~~रयोः", "~ि~~रेषु";
        locative_feminine: "~ि~~रायाम्", "~ि~~रयोः", "~ि~~रासु";
        locative_neuter: "~ि~~रे", "~ि~~रयोः", "~ि~~रेषु";
        vocative_masculine: "~ि~~र", "~ि~~रौ", "~ि~~राः";
        vocative_feminine: "~ि~~रे", "~ि~~रे", "~ि~~राः";
        vocative_neuter: "~ि~~र", "~ि~~रे", "~ि~~राणि";
        comparative: "~े~ीयस~";
        superlative: "~े~िष~ठ";
    ),
    // घटक: 22 words (आवश्यक, अंशक, जनक, नन्दक, खानक, पालक).
    declension!("घटक", 22, "~~";
        nominative_masculine: "~~ः", "~~ौ", "~~ाः";
        nominative_feminine: "~ि~ा", "~ि~े", "~ि~ाः";
        nominative_neuter: "~~म्", "~~े", "~~ानि";
        accusative_masculine: "~~म्", "~~ौ", "~~ान्";
        accusative_feminine: "~ि~ाम्", "~ि~े", "~ि~ाः";
        accusative_neuter: "~~म्", "~~े", "~~ानि";
        instrumental_masculine: "~~ेन", "~~ाभ्याम्", "~~ैः";
        instrumental_feminine: "~ि~या", "~ि~ाभ्याम्", "~ि~ाभिः";
        instrumental_neuter: "~~ेन", "~~ाभ्याम्", "~~ैः";
        dative_masculine: "~~ाय", "~~ाभ्याम्", "~~ेभ्यः";
        dative_feminine: "~ि~ायै", "~ि~ाभ्याम्", "~ि~ाभ्यः";
        dative_neuter: "~~ाय", "~~ाभ्याम्", "~~ेभ्यः";
        ablative_masculine: "~~ात्", "~~ाभ्याम्", "~~ेभ्यः";
        ablative_feminine: "~ि~ायाः", "~ि~ाभ्याम्", "~ि~ाभ्यः";
        ablative_neuter: "~~ात्", "~~ाभ्याम्", "~~ेभ्यः";
        genitive_masculine: "~~स्य", "~~योः", "~~ानाम्";
        genitive_feminine: "~ि~ायाः", "~ि~योः", "~ि~ानाम्";
        genitive_neuter: "~~स्य", "~~योः", "~~ानाम्";
        locative_masculine: "~~े", "~~योः", "~~ेषु";
        locative_feminine: "~ि~ायाम्", "~ि~योः", "~ि~ासु";
        locative_neuter: "~~े", "~~योः", "~~ेषु";
        vocative_masculine: "~~", "~~ौ", "~~ाः";
        vocative_feminine: "~ि~े", "~ि~े", "~ि~ाः";
        vocative_neuter: "~~", "~~े", "~~ानि";
        comparative: derived;
        superlative: derived;
    ),
    // धायस: 19 words (रिशादस्, स्वपस्, सुश्रवस्, सुशेवस्, वीतिराधस्, वपुस्).
    declension!("धायस्", 19, "~स्";
        nominative_masculine: "~ाः", "~सौ", "~सः";
        nominative_feminine: "~ाः", "~सौ", "~सः";
        nominative_neuter: "~ः", "~सी", "~ांसि";
        accusative_masculine: "~सम्", "~सौ", "~सः";
        accusative_feminine: "~सम्", "~सौ", "~सः";
        accusative_neuter: "~ः", "~सी", "~ांसि";
        instrumental_masculine: "~सा", "~ोभ्याम्", "~ोभिः";
        instrumental_feminine: "~सा", "~ोभ्याम्", "~ोभिः";
        instrumental_neuter: "~सा", "~ोभ्याम्", "~ोभिः";
        dative_masculine: "~से", "~ोभ्याम्", "~ोभ्यः";
        dative_feminine: "~से", "~ोभ्याम्", "~ोभ्यः";
        dative_neuter: "~से", "~ोभ्याम्", "~ोभ्यः";
        ablative_masculine: "~सः", "~ोभ्याम्", "~ोभ्यः";
        ablative_feminine: "~सः", "~ोभ्याम्", "~ोभ्यः";
        ablative_neuter: "~सः", "~ोभ्याम्", "~ोभ्यः";
        genitive_masculine: "~सः", "~सोः", "~साम्";
        genitive_feminine: "~सः", "~सोः", "~साम्";
        genitive_neuter: "~सः", "~सोः", "~साम्";
        locative_masculine: "~सि", "~सोः", "~ःसु|~स्सु";
        locative_feminine: "~सि", "~सोः", "~ःसु|~स्सु";
        locative_neuter: "~सि", "~सोः", "~ःसु|~स्सु";
        vocative_masculine: "~ः", "~सौ", "~सः";
        vocative_feminine: "~ः", "~सौ", "~सः";
        vocative_neuter: "~ः", "~सी", "~ांसि";
        comparative: "~ीयस्";
        superlative: "~स्तम";
    ),
    // भारिन: 18 words (मन्त्रिन्, त्सारिन्, अवत्रापिन्, अहंकारिन्, रङ्गिन्, फिरङ्गिन्).
    declension!("भारिन्", 18, "~िन्";
        nominative_masculine: "~ी", "~िणौ", "~िणः";
        nominative_feminine: "~िणी", "~िण्यौ", "~िण्यः";
        nominative_neuter: "~ि", "~िणी", "~ीणि";
        accusative_masculine: "~िणम्", "~िणौ", "~िणः";
        accusative_feminine: "~िणीम्", "~िण्यौ", "~िणीः";
        accusative_neuter: "~ि", "~िणी", "~ीणि";
        instrumental_masculine: "~िणा", "~िभ्याम्", "~िभिः";
        instrumental_feminine: "~िण्या", "~िणीभ्याम्", "~िणीभिः";
        instrumental_neuter: "~िणा", "~िभ्याम्", "~िभिः";
        dative_masculine: "~िणे", "~िभ्याम्", "~िभ्यः";
        dative_feminine: "~िण्यै", "~िणीभ्याम्", "~िणीभ्यः";
        dative_neuter: "~िणे", "~िभ्याम्", "~िभ्यः";
        ablative_masculine: "~िणः", "~िभ्याम्", "~िभ्यः";
        ablative_feminine: "~िण्याः", "~िणीभ्याम्", "~िणीभ्यः";
        ablative_neuter: "~िणः", "~िभ्याम्", "~िभ्यः";
        genitive_masculine: "~िणः", "~िणोः", "~िणाम्";
        genitive_feminine: "~िण्याः", "~िण्योः", "~िणीनाम्";
        genitive_neuter: "~िणः", "~िणोः", "~िणाम्";
        locative_masculine: "~िणि", "~िणोः", "~िषु";
        locative_feminine: "~िण्याम्", "~िण्योः", "~िणीषु";
        locative_neuter: "~िणि", "~िणोः", "~िषु";
        vocative_masculine: "~िन्", "~िणौ", "~िणः";
        vocative_feminine: "~िणि", "~िण्यौ", "~िण्यः";
        vocative_neuter: "~ि|~िन्", "~िणी", "~ीणि";
        comparative: derived;
        superlative: derived;
    ),
    // ऋतावन: 15 words (वृत्रहन्, ऋतावन्, रक्षोहन्, वृषन्, हन्, यावन्).
    declension!("ऋतावन्", 15, "~न्";
        nominative_masculine: "~ा", "~ानौ", "~ानः";
        nominative_feminine: "~री", "~र्यौ", "~र्यः";
        nominative_neuter: "~", "~्नी|~नी", "~ानि";
        accusative_masculine: "~ानम्", "~ानौ", "~्नः";
        accusative_feminine: "~रीम्", "~र्यौ", "~रीः";
        accusative_neuter: "~", "~्नी|~नी", "~ानि";
        instrumental_masculine: "~्ना", "~भ्याम्", "~भिः";
        instrumental_feminine: "~र्या", "~रीभ्याम्", "~रीभिः";
        instrumental_neuter: "~्ना", "~भ्याम्", "~भिः";
        dative_masculine: "~्ने", "~भ्याम्", "~भ्यः";
        dative_feminine: "~र्यै", "~रीभ्याम्", "~रीभ्यः";
        dative_neuter: "~्ने", "~भ्याम्", "~भ्यः";
        ablative_masculine: "~्नः", "~भ्याम्", "~भ्यः";
        ablative_feminine: "~र्याः", "~रीभ्याम्", "~रीभ्यः";
        ablative_neuter: "~्नः", "~भ्याम्", "~भ्यः";
        genitive_masculine: "~्नः", "~्नोः", "~्नाम्";
        genitive_feminine: "~र्याः", "~र्योः", "~रीणाम्";
        genitive_neuter: "~्नः", "~्नोः", "~्नाम्";
        locative_masculine: "~्नि|~नि", "~्नोः", "~सु";
        locative_feminine: "~र्याम्", "~र्योः", "~रीषु";
        locative_neuter: "~्नि|~नि", "~्नोः", "~सु";
        vocative_masculine: "~न्", "~ानौ", "~ानः";
        vocative_feminine: "~रि", "~र्यौ", "~र्यः";
        vocative_neuter: "~न्|~", "~्नी|~नी", "~ानि";
        comparative: derived;
        superlative: derived;
    ),
    // जयायस: 15 words (श्रेयस्, वस्यस्, मलीयस्, ज्यायस्, ओजीयस्, कनीयस्). Like धायस, except in 34 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, ….
    declension!("ज्यायस्", 15, "~स्", like धायस;
        nominative_masculine: "~ान्", "~ांसौ", "~ांसः";
        nominative_feminine: "~सी", "~स्यौ", "~स्यः";
        nominative_neuter: _, _, _;
        accusative_masculine: "~ांसम्", "~ांसौ", _;
        accusative_feminine: "~सीम्", "~स्यौ", "~सीः";
        accusative_neuter: _, _, _;
        instrumental_masculine: _, _, _;
        instrumental_feminine: "~स्या", "~सीभ्याम्", "~सीभिः";
        instrumental_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~स्यै", "~सीभ्याम्", "~सीभ्यः";
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: "~स्याः", "~सीभ्याम्", "~सीभ्यः";
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~स्याः", "~स्योः", "~सीनाम्";
        genitive_neuter: _, _, _;
        locative_masculine: _, _, _;
        locative_feminine: "~स्याम्", "~स्योः", "~सीषु";
        locative_neuter: _, _, _;
        vocative_masculine: "~न्", "~ांसौ", "~ांसः";
        vocative_feminine: "~सि", "~स्यौ", "~स्यः";
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: derived;
    ),
    // धृषु: 13 words (दारु, तरु, पुरु, दीर्घायु, धृषु, रिपु). Like मनु, except in 17 cells: nominative_neuter du, nominative_neuter pl, accusative_neuter du, accusative_neuter pl, instrumental_masculine sg, instrumental_neuter sg, ….
    declension!("धृषु", 13, "~ु", like मनु;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, "~ुणी", "~ूणि";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, "~ुणी", "~ूणि";
        instrumental_masculine: "~ुणा", _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: "~ुणा", _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: "~ुणे|~वे", _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: "~ुणः|~ोः", _, _;
        genitive_masculine: _, _, "~ूणाम्";
        genitive_feminine: _, _, "~ूणाम्";
        genitive_neuter: "~ुणः|~ोः", "~ुणोः|~्वोः", "~ूणाम्";
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: "~ुणि|~ौ", "~ुणोः|~्वोः", _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, "~ुणी", "~ूणि";
        comparative: _;
        superlative: _;
    ),
    // लघु: 11 words (अणु, बहु, लघु, अंहु, आशु, वसु). Like मनु, except in 26 cells: nominative_feminine sg, nominative_feminine du, nominative_feminine pl, accusative_feminine sg, accusative_feminine du, accusative_feminine pl, ….
    declension!("लघु", 11, "~ु", like मनु;
        nominative_masculine: _, _, _;
        nominative_feminine: "~्वी", "~्व्यौ", "~्व्यः";
        nominative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: "~्वीम्", "~्व्यौ", "~्वीः";
        accusative_neuter: _, _, _;
        instrumental_masculine: _, _, _;
        instrumental_feminine: "~्व्या", "~्वीभ्याम्", "~्वीभिः";
        instrumental_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~्व्यै", "~्वीभ्याम्", "~्वीभ्यः";
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: "~्व्याः", "~्वीभ्याम्", "~्वीभ्यः";
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~्व्याः", "~्व्योः", "~्वीनाम्";
        genitive_neuter: _, _, _;
        locative_masculine: _, _, _;
        locative_feminine: "~्व्याम्", "~्व्योः", "~्वीषु";
        locative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: "~्वि", "~्व्यौ", "~्व्यः";
        vocative_neuter: _, _, _;
        comparative: "~ीयस्";
        superlative: "~िष्ठ";
    ),
    // छिद: 9 words (उपस्थसद्, पुरुषाद्, सद्, विद्, छिद्, भिद्).
    declension!("छिद्", 9, "~द्";
        nominative_masculine: "~त्", "~दौ", "~दः";
        nominative_feminine: "~त्", "~दौ", "~दः";
        nominative_neuter: "~त्", "~दी", "~न्दि";
        accusative_masculine: "~दम्", "~दौ", "~दः";
        accusative_feminine: "~दम्", "~दौ", "~दः";
        accusative_neuter: "~त्", "~दी", "~न्दि";
        instrumental_masculine: "~दा", "~द्भ्याम्", "~द्भिः";
        instrumental_feminine: "~दा", "~द्भ्याम्", "~द्भिः";
        instrumental_neuter: "~दा", "~द्भ्याम्", "~द्भिः";
        dative_masculine: "~दे", "~द्भ्याम्", "~द्भ्यः";
        dative_feminine: "~दे", "~द्भ्याम्", "~द्भ्यः";
        dative_neuter: "~दे", "~द्भ्याम्", "~द्भ्यः";
        ablative_masculine: "~दः", "~द्भ्याम्", "~द्भ्यः";
        ablative_feminine: "~दः", "~द्भ्याम्", "~द्भ्यः";
        ablative_neuter: "~दः", "~द्भ्याम्", "~द्भ्यः";
        genitive_masculine: "~दः", "~दोः", "~दाम्";
        genitive_feminine: "~दः", "~दोः", "~दाम्";
        genitive_neuter: "~दः", "~दोः", "~दाम्";
        locative_masculine: "~दि", "~दोः", "~त्सु";
        locative_feminine: "~दि", "~दोः", "~त्सु";
        locative_neuter: "~दि", "~दोः", "~त्सु";
        vocative_masculine: "~त्", "~दौ", "~दः";
        vocative_feminine: "~त्", "~दौ", "~दः";
        vocative_neuter: "~त्", "~दी", "~न्दि";
        comparative: "~त्तर";
        superlative: "~त्तम";
    ),
    // तरुतृ: 8 words (यातृ, नेतृ, तरुतृ, गन्तृ, यन्तृ, पक्तृ).
    declension!("तरुतृ", 8, "~ृ";
        nominative_masculine: "~ा", "~ारौ", "~ारः";
        nominative_feminine: "~्री", "~्र्यौ", "~्र्यः";
        nominative_neuter: "~ृ", "~ृणी", "~ॄणि";
        accusative_masculine: "~ारम्", "~ारौ", "~ॄन्";
        accusative_feminine: "~्रीम्", "~्र्यौ", "~्रीः";
        accusative_neuter: "~ृ", "~ृणी", "~ॄणि";
        instrumental_masculine: "~्रा", "~ृभ्याम्", "~ृभिः";
        instrumental_feminine: "~्र्या", "~्रीभ्याम्", "~्रीभिः";
        instrumental_neuter: "~ृणा|~्रा", "~ृभ्याम्", "~ृभिः";
        dative_masculine: "~्रे", "~ृभ्याम्", "~ृभ्यः";
        dative_feminine: "~्र्यै", "~्रीभ्याम्", "~्रीभ्यः";
        dative_neuter: "~ृणे|~्रे", "~ृभ्याम्", "~ृभ्यः";
        ablative_masculine: "~ुः", "~ृभ्याम्", "~ृभ्यः";
        ablative_feminine: "~्र्याः", "~्रीभ्याम्", "~्रीभ्यः";
        ablative_neuter: "~ृणः|~ुः", "~ृभ्याम्", "~ृभ्यः";
        genitive_masculine: "~ुः", "~्रोः", "~ॄणाम्";
        genitive_feminine: "~्र्याः", "~्र्योः", "~्रीणाम्";
        genitive_neuter: "~ृणः|~ुः", "~ृणोः|~्रोः", "~ॄणाम्";
        locative_masculine: "~रि", "~्रोः", "~ृषु";
        locative_feminine: "~्र्याम्", "~्र्योः", "~्रीषु";
        locative_neuter: "~ृणि|~रि", "~ृणोः|~्रोः", "~ृषु";
        vocative_masculine: "~ः", "~ारौ", "~ारः";
        vocative_feminine: "~्रि", "~्र्यौ", "~्र्यः";
        vocative_neuter: "~ृ|~ः", "~ृणी", "~ॄणि";
        comparative: derived;
        superlative: derived;
    ),
    // कारक: 8 words (विदूषक, कारक, तारक, सारक, प्रियकारक, क्षुद्रक). Like घटक, except in 8 cells: nominative_neuter pl, accusative_neuter pl, instrumental_masculine sg, instrumental_neuter sg, genitive_masculine pl, genitive_feminine pl, ….
    declension!("कारक", 8, "~~", like घटक;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, "~~ाणि";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, "~~ाणि";
        instrumental_masculine: "~~ेण", _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: "~~ेण", _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, "~~ाणाम्";
        genitive_feminine: _, _, "~ि~ाणाम्";
        genitive_neuter: _, _, "~~ाणाम्";
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, "~~ाणि";
        comparative: _;
        superlative: _;
    ),
    // रथी: 8 words (श्री, प्री, वी, वीतभी, रथी, शी).
    declension!("रथी", 8, "~ी";
        nominative_masculine: "~ीः", "~ियौ", "~ियः";
        nominative_feminine: "~ीः", "~ियौ", "~ियः";
        nominative_neuter: "~ि", "~िनी", "~ीनि";
        accusative_masculine: "~ियम्", "~ियौ", "~ियः";
        accusative_feminine: "~ियम्", "~ियौ", "~ियः";
        accusative_neuter: "~ि", "~िनी", "~ीनि";
        instrumental_masculine: "~िया", "~ीभ्याम्", "~ीभिः";
        instrumental_feminine: "~िया", "~ीभ्याम्", "~ीभिः";
        instrumental_neuter: "~िना", "~िभ्याम्", "~िभिः";
        dative_masculine: "~िये", "~ीभ्याम्", "~ीभ्यः";
        dative_feminine: "~िये", "~ीभ्याम्", "~ीभ्यः";
        dative_neuter: "~िने|~ये", "~िभ्याम्", "~िभ्यः";
        ablative_masculine: "~ियः", "~ीभ्याम्", "~ीभ्यः";
        ablative_feminine: "~ियः", "~ीभ्याम्", "~ीभ्यः";
        ablative_neuter: "~िनः|~ेः", "~िभ्याम्", "~िभ्यः";
        genitive_masculine: "~ियः", "~ियोः", "~ियाम्";
        genitive_feminine: "~ियः", "~ियोः", "~ियाम्";
        genitive_neuter: "~िनः|~ेः", "~िनोः|~्योः", "~ीनाम्";
        locative_masculine: "~ियि", "~ियोः", "~ीषु";
        locative_feminine: "~ियि", "~ियोः", "~ीषु";
        locative_neuter: "~िनि|~ौ", "~िनोः|~्योः", "~िषु";
        vocative_masculine: "~ीः", "~ियौ", "~ियः";
        vocative_feminine: "~ीः", "~ियौ", "~ियः";
        vocative_neuter: "~ि|~े", "~िनी", "~ीनि";
        comparative: derived;
        superlative: derived;
    ),
    // सरवजित: 7 words (विद्युत्, दशत्, हरित्, सर्वजित्, च्युत्, श्चुत्). Like रसवत, except in 32 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, nominative_feminine sg, nominative_feminine du, nominative_feminine pl, ….
    declension!("सर्वजित्", 7, "~त्", like रसवत;
        nominative_masculine: "~त्", "~तौ", "~तः";
        nominative_feminine: "~त्", "~तौ", "~तः";
        nominative_neuter: _, _, _;
        accusative_masculine: "~तम्", "~तौ", _;
        accusative_feminine: "~तम्", "~तौ", "~तः";
        accusative_neuter: _, _, _;
        instrumental_masculine: _, _, _;
        instrumental_feminine: "~ता", "~द्भ्याम्", "~द्भिः";
        instrumental_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: "~ते", "~द्भ्याम्", "~द्भ्यः";
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: "~तः", "~द्भ्याम्", "~द्भ्यः";
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: "~तः", "~तोः", "~ताम्";
        genitive_neuter: _, _, _;
        locative_masculine: _, _, _;
        locative_feminine: "~ति", "~तोः", "~त्सु";
        locative_neuter: _, _, _;
        vocative_masculine: "~त्", "~तौ", "~तः";
        vocative_feminine: "~त्", "~तौ", "~तः";
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
    ),
    // परिय: 6 words (प्रिय, अन्ताराष्ट्रिय, क्षत्रिय, इन्द्रिय, नष्टक्रिय, अनिन्द्रिय).
    declension!("प्रिय", 6, "~िय";
        nominative_masculine: "~ियः", "~ियौ", "~ियाः";
        nominative_feminine: "~िया", "~िये", "~ियाः";
        nominative_neuter: "~ियम्", "~िये", "~ियाणि";
        accusative_masculine: "~ियम्", "~ियौ", "~ियान्";
        accusative_feminine: "~ियाम्", "~िये", "~ियाः";
        accusative_neuter: "~ियम्", "~िये", "~ियाणि";
        instrumental_masculine: "~ियेण", "~ियाभ्याम्", "~ियैः";
        instrumental_feminine: "~ियया", "~ियाभ्याम्", "~ियाभिः";
        instrumental_neuter: "~ियेण", "~ियाभ्याम्", "~ियैः";
        dative_masculine: "~ियाय", "~ियाभ्याम्", "~ियेभ्यः";
        dative_feminine: "~ियायै", "~ियाभ्याम्", "~ियाभ्यः";
        dative_neuter: "~ियाय", "~ियाभ्याम्", "~ियेभ्यः";
        ablative_masculine: "~ियात्", "~ियाभ्याम्", "~ियेभ्यः";
        ablative_feminine: "~ियायाः", "~ियाभ्याम्", "~ियाभ्यः";
        ablative_neuter: "~ियात्", "~ियाभ्याम्", "~ियेभ्यः";
        genitive_masculine: "~ियस्य", "~िययोः", "~ियाणाम्";
        genitive_feminine: "~ियायाः", "~िययोः", "~ियाणाम्";
        genitive_neuter: "~ियस्य", "~िययोः", "~ियाणाम्";
        locative_masculine: "~िये", "~िययोः", "~ियेषु";
        locative_feminine: "~ियायाम्", "~िययोः", "~ियासु";
        locative_neuter: "~िये", "~िययोः", "~ियेषु";
        vocative_masculine: "~िय", "~ियौ", "~ियाः";
        vocative_feminine: "~िये", "~िये", "~ियाः";
        vocative_neuter: "~िय", "~िये", "~ियाणि";
        comparative: "~ेयस्";
        superlative: "~ेष्ठ";
    ),
    // असत: 6 words (बृहत्, पृषत्, अर्हत्, असत्, ऋहत्, अजानत्). Like रसवत, except in 1 cell: nominative_masculine sg.
    declension!("असत्", 6, "~त्", like रसवत;
        nominative_masculine: "~न्", _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        instrumental_masculine: _, _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
    ),
    // महातमन: 6 words (प्रज्ञात्मन्, शर्मन्, महात्मन्, भस्मन्, युग्मन्, अकृतात्मन्).
    declension!("महात्मन्", 6, "~न्";
        nominative_masculine: "~ा", "~ानौ", "~ानः";
        nominative_feminine: "~ा", "~ानौ", "~ानः";
        nominative_neuter: "~", "~नी", "~ानि";
        accusative_masculine: "~ानम्", "~ानौ", "~नः";
        accusative_feminine: "~ानम्", "~ानौ", "~नः";
        accusative_neuter: "~", "~नी", "~ानि";
        instrumental_masculine: "~ना", "~भ्याम्", "~भिः";
        instrumental_feminine: "~ना", "~भ्याम्", "~भिः";
        instrumental_neuter: "~ना", "~भ्याम्", "~भिः";
        dative_masculine: "~ने", "~भ्याम्", "~भ्यः";
        dative_feminine: "~ने", "~भ्याम्", "~भ्यः";
        dative_neuter: "~ने", "~भ्याम्", "~भ्यः";
        ablative_masculine: "~नः", "~भ्याम्", "~भ्यः";
        ablative_feminine: "~नः", "~भ्याम्", "~भ्यः";
        ablative_neuter: "~नः", "~भ्याम्", "~भ्यः";
        genitive_masculine: "~नः", "~नोः", "~नाम्";
        genitive_feminine: "~नः", "~नोः", "~नाम्";
        genitive_neuter: "~नः", "~नोः", "~नाम्";
        locative_masculine: "~नि", "~नोः", "~सु";
        locative_feminine: "~नि", "~नोः", "~सु";
        locative_neuter: "~नि", "~नोः", "~सु";
        vocative_masculine: "~न्", "~ानौ", "~ानः";
        vocative_feminine: "~न्", "~ानौ", "~ानः";
        vocative_neuter: "~न्|~", "~नी", "~ानि";
        comparative: derived;
        superlative: derived;
    ),
    // उशिज: 6 words (विराज्, भुज्, उशिज्, त्यज्, रुज्, सयुज्).
    declension!("उशिज्", 6, "~ज्";
        nominative_masculine: "~क्", "~जौ", "~जः";
        nominative_feminine: "~क्", "~जौ", "~जः";
        nominative_neuter: "~क्", "~जी", "~ञ्जि";
        accusative_masculine: "~जम्", "~जौ", "~जः";
        accusative_feminine: "~जम्", "~जौ", "~जः";
        accusative_neuter: "~क्", "~जी", "~ञ्जि";
        instrumental_masculine: "~जा", "~ग्भ्याम्", "~ग्भिः";
        instrumental_feminine: "~जा", "~ग्भ्याम्", "~ग्भिः";
        instrumental_neuter: "~जा", "~ग्भ्याम्", "~ग्भिः";
        dative_masculine: "~जे", "~ग्भ्याम्", "~ग्भ्यः";
        dative_feminine: "~जे", "~ग्भ्याम्", "~ग्भ्यः";
        dative_neuter: "~जे", "~ग्भ्याम्", "~ग्भ्यः";
        ablative_masculine: "~जः", "~ग्भ्याम्", "~ग्भ्यः";
        ablative_feminine: "~जः", "~ग्भ्याम्", "~ग्भ्यः";
        ablative_neuter: "~जः", "~ग्भ्याम्", "~ग्भ्यः";
        genitive_masculine: "~जः", "~जोः", "~जाम्";
        genitive_feminine: "~जः", "~जोः", "~जाम्";
        genitive_neuter: "~जः", "~जोः", "~जाम्";
        locative_masculine: "~जि", "~जोः", "~क्षु";
        locative_feminine: "~जि", "~जोः", "~क्षु";
        locative_neuter: "~जि", "~जोः", "~क्षु";
        vocative_masculine: "~क्", "~जौ", "~जः";
        vocative_feminine: "~क्", "~जौ", "~जः";
        vocative_neuter: "~क्", "~जी", "~ञ्जि";
        comparative: derived;
        superlative: derived;
    ),
    // पा: 6 words (सा, वर्चोदा, वर्चोधा, पा, रथेष्ठा, विश्वपा). Like धृत, except in 17 cells: nominative_masculine sg, accusative_masculine sg, accusative_masculine pl, instrumental_masculine sg, instrumental_masculine pl, dative_masculine sg, ….
    declension!("पा", 6, "~ा", like धृत;
        nominative_masculine: "~ाः", _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        accusative_masculine: "~ाम्", _, "~ः";
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        instrumental_masculine: "~ा", _, "~ाभिः";
        instrumental_feminine: _, _, _;
        instrumental_neuter: _, _, _;
        dative_masculine: "~े", _, "~ाभ्यः";
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        ablative_masculine: "~ः", _, "~ाभ्यः";
        ablative_feminine: _, _, _;
        ablative_neuter: _, _, _;
        genitive_masculine: "~ः", "~ोः", _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        locative_masculine: "~ि", "~ोः", "~ासु";
        locative_feminine: _, _, _;
        locative_neuter: _, _, _;
        vocative_masculine: "~ाः", _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: derived;
        superlative: derived;
    ),
    // हरि: 6 words (हरि, अरि, करि, वध्रि, भूरि, सहुरि).
    declension!("हरि", 6, "~रि";
        nominative_masculine: "~रिः", "~री", "~रयः";
        nominative_feminine: "~रिः", "~री", "~रयः";
        nominative_neuter: "~रि", "~रिणी", "~रीणि";
        accusative_masculine: "~रिम्", "~री", "~रीन्";
        accusative_feminine: "~रिम्", "~री", "~रीः";
        accusative_neuter: "~रि", "~रिणी", "~रीणि";
        instrumental_masculine: "~रिणा", "~रिभ्याम्", "~रिभिः";
        instrumental_feminine: "~र्या", "~रिभ्याम्", "~रिभिः";
        instrumental_neuter: "~रिणा", "~रिभ्याम्", "~रिभिः";
        dative_masculine: "~रये", "~रिभ्याम्", "~रिभ्यः";
        dative_feminine: "~रये", "~रिभ्याम्", "~रिभ्यः";
        dative_neuter: "~रिणे", "~रिभ्याम्", "~रिभ्यः";
        ablative_masculine: "~रेः", "~रिभ्याम्", "~रिभ्यः";
        ablative_feminine: "~रेः", "~रिभ्याम्", "~रिभ्यः";
        ablative_neuter: "~रिणः", "~रिभ्याम्", "~रिभ्यः";
        genitive_masculine: "~रेः", "~र्योः", "~रीणाम्";
        genitive_feminine: "~रेः", "~र्योः", "~रीणाम्";
        genitive_neuter: "~रिणः", "~रिणोः", "~रीणाम्";
        locative_masculine: "~रौ", "~र्योः", "~रिषु";
        locative_feminine: "~रौ", "~र्योः", "~रिषु";
        locative_neuter: "~रिणि", "~रिणोः", "~रिषु";
        vocative_masculine: "~रे", "~री", "~रयः";
        vocative_feminine: "~रे", "~री", "~रयः";
        vocative_neuter: "~रि", "~रिणी", "~रीणि";
        comparative: "~यस्";
        superlative: "~यिष्ठ";
    ),
    // सपृश: 5 words (सदृश्, स्पृश्, हृदिस्पृश्, कीदृश्, ईदृश्).
    declension!("स्पृश्", 5, "~श्";
        nominative_masculine: "~क्", "~शौ", "~शः";
        nominative_feminine: "~क्", "~शौ", "~शः";
        nominative_neuter: "~क्", "~शी", "~ंशि";
        accusative_masculine: "~शम्", "~शौ", "~शः";
        accusative_feminine: "~शम्", "~शौ", "~शः";
        accusative_neuter: "~क्", "~शी", "~ंशि";
        instrumental_masculine: "~शा", "~ग्भ्याम्", "~ग्भिः";
        instrumental_feminine: "~शा", "~ग्भ्याम्", "~ग्भिः";
        instrumental_neuter: "~शा", "~ग्भ्याम्", "~ग्भिः";
        dative_masculine: "~शे", "~ग्भ्याम्", "~ग्भ्यः";
        dative_feminine: "~शे", "~ग्भ्याम्", "~ग्भ्यः";
        dative_neuter: "~शे", "~ग्भ्याम्", "~ग्भ्यः";
        ablative_masculine: "~शः", "~ग्भ्याम्", "~ग्भ्यः";
        ablative_feminine: "~शः", "~ग्भ्याम्", "~ग्भ्यः";
        ablative_neuter: "~शः", "~ग्भ्याम्", "~ग्भ्यः";
        genitive_masculine: "~शः", "~शोः", "~शाम्";
        genitive_feminine: "~शः", "~शोः", "~शाम्";
        genitive_neuter: "~शः", "~शोः", "~शाम्";
        locative_masculine: "~शि", "~शोः", "~क्षु";
        locative_feminine: "~शि", "~शोः", "~क्षु";
        locative_neuter: "~शि", "~शोः", "~क्षु";
        vocative_masculine: "~क्", "~शौ", "~शः";
        vocative_feminine: "~क्", "~शौ", "~शः";
        vocative_neuter: "~क्", "~शी", "~ंशि";
        comparative: derived;
        superlative: derived;
    ),
    // नीरोग: 4 words (दीर्घ, शीर्षक, धीर्य, नीरोग).
    declension!("नीरोग", 4, "~ीर~~";
        nominative_masculine: "~ीर~~ः", "~ीर~~ौ", "~ीर~~ाः";
        nominative_feminine: "~ीर~~ा", "~ीर~~े", "~ीर~~ाः";
        nominative_neuter: "~ीर~~म्", "~ीर~~े", "~ीर~~ाणि";
        accusative_masculine: "~ीर~~म्", "~ीर~~ौ", "~ीर~~ान्";
        accusative_feminine: "~ीर~~ाम्", "~ीर~~े", "~ीर~~ाः";
        accusative_neuter: "~ीर~~म्", "~ीर~~े", "~ीर~~ाणि";
        instrumental_masculine: "~ीर~~ेण", "~ीर~~ाभ्याम्", "~ीर~~ैः";
        instrumental_feminine: "~ीर~~या", "~ीर~~ाभ्याम्", "~ीर~~ाभिः";
        instrumental_neuter: "~ीर~~ेण", "~ीर~~ाभ्याम्", "~ीर~~ैः";
        dative_masculine: "~ीर~~ाय", "~ीर~~ाभ्याम्", "~ीर~~ेभ्यः";
        dative_feminine: "~ीर~~ायै", "~ीर~~ाभ्याम्", "~ीर~~ाभ्यः";
        dative_neuter: "~ीर~~ाय", "~ीर~~ाभ्याम्", "~ीर~~ेभ्यः";
        ablative_masculine: "~ीर~~ात्", "~ीर~~ाभ्याम्", "~ीर~~ेभ्यः";
        ablative_feminine: "~ीर~~ायाः", "~ीर~~ाभ्याम्", "~ीर~~ाभ्यः";
        ablative_neuter: "~ीर~~ात्", "~ीर~~ाभ्याम्", "~ीर~~ेभ्यः";
        genitive_masculine: "~ीर~~स्य", "~ीर~~योः", "~ीर~~ाणाम्";
        genitive_feminine: "~ीर~~ायाः", "~ीर~~योः", "~ीर~~ाणाम्";
        genitive_neuter: "~ीर~~स्य", "~ीर~~योः", "~ीर~~ाणाम्";
        locative_masculine: "~ीर~~े", "~ीर~~योः", "~ीर~~ेषु";
        locative_feminine: "~ीर~~ायाम्", "~ीर~~योः", "~ीर~~ासु";
        locative_neuter: "~ीर~~े", "~ीर~~योः", "~ीर~~ेषु";
        vocative_masculine: "~ीर~~", "~ीर~~ौ", "~ीर~~ाः";
        vocative_feminine: "~ीर~~े", "~ीर~~े", "~ीर~~ाः";
        vocative_neuter: "~ीर~~", "~ीर~~े", "~ीर~~ाणि";
        comparative: "~~रा~ीयस्";
        superlative: "~~रा~िष्ठ";
    ),
    // अमनस: 4 words (अमनस्, क्लान्तमनस्, रंहस्, तरस्). Like जयायस, except in 8 cells: nominative_masculine sg, nominative_masculine du, nominative_masculine pl, accusative_masculine sg, accusative_masculine du, vocative_masculine sg, ….
    declension!("अमनस्", 4, "~स्", like जयायस;
        nominative_masculine: "~ाः", "~सौ", "~सः";
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        accusative_masculine: "~सम्", "~सौ", _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        instrumental_masculine: _, _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: _, _, _;
        vocative_masculine: "~ः", "~सौ", "~सः";
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: _;
    ),
    // वामन: 3 words (नव, घन, पक्व). Like धृत, except in 2 cells: comparative, superlative.
    declension!("वामन", 3, "~", like धृत;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        instrumental_masculine: _, _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: "~ीयस्";
        superlative: "~िष्ठ";
    ),
    // बहुवरीहि: 3 words (ब्राह्मि, बहुव्रीहि, द्रापि). Like शुचि, except in 17 cells: nominative_neuter du, nominative_neuter pl, accusative_neuter du, accusative_neuter pl, instrumental_masculine sg, instrumental_neuter sg, ….
    declension!("बहुव्रीहि", 3, "~ि", like शुचि;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, "~िणी", "~ीणि";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, "~िणी", "~ीणि";
        instrumental_masculine: "~िणा", _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: "~िणा", _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: "~िणे|~ये", _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: "~िणः|~ेः", _, _;
        genitive_masculine: _, _, "~ीणाम्";
        genitive_feminine: _, _, "~ीणाम्";
        genitive_neuter: "~िणः|~ेः", "~िणोः|~्योः", "~ीणाम्";
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: "~िणि|~ौ", "~िणोः|~्योः", _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, "~िणी", "~ीणि";
        comparative: _;
        superlative: _;
    ),
    // रघु: 3 words (अगुरु, प्रभु, रघु). Like लघु, except in 17 cells: nominative_neuter du, nominative_neuter pl, accusative_neuter du, accusative_neuter pl, instrumental_masculine sg, instrumental_neuter sg, ….
    declension!("रघु", 3, "~ु", like लघु;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, "~ुणी", "~ूणि";
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, "~ुणी", "~ूणि";
        instrumental_masculine: "~ुणा", _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: "~ुणा", _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: "~ुणे|~वे", _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: "~ुणः|~ोः", _, _;
        genitive_masculine: _, _, "~ूणाम्";
        genitive_feminine: _, _, "~्वीणाम्";
        genitive_neuter: "~ुणः|~ोः", "~ुणोः|~्वोः", "~ूणाम्";
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: "~ुणि|~ौ", "~ुणोः|~्वोः", _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, "~ुणी", "~ूणि";
        comparative: _;
        superlative: _;
    ),
    // चिर: 1 words (स्फिर).
    declension!("चिर", 1, "~िर";
        nominative_masculine: "~िरः", "~िरौ", "~िराः";
        nominative_feminine: "~िरा", "~िरे", "~िराः";
        nominative_neuter: "~िरम्", "~िरे", "~िराणि";
        accusative_masculine: "~िरम्", "~िरौ", "~िरान्";
        accusative_feminine: "~िराम्", "~िरे", "~िराः";
        accusative_neuter: "~िरम्", "~िरे", "~िराणि";
        instrumental_masculine: "~िरेण", "~िराभ्याम्", "~िरैः";
        instrumental_feminine: "~िरया", "~िराभ्याम्", "~िराभिः";
        instrumental_neuter: "~िरेण", "~िराभ्याम्", "~िरैः";
        dative_masculine: "~िराय", "~िराभ्याम्", "~िरेभ्यः";
        dative_feminine: "~िरायै", "~िराभ्याम्", "~िराभ्यः";
        dative_neuter: "~िराय", "~िराभ्याम्", "~िरेभ्यः";
        ablative_masculine: "~िरात्", "~िराभ्याम्", "~िरेभ्यः";
        ablative_feminine: "~िरायाः", "~िराभ्याम्", "~िराभ्यः";
        ablative_neuter: "~िरात्", "~िराभ्याम्", "~िरेभ्यः";
        genitive_masculine: "~िरस्य", "~िरयोः", "~िराणाम्";
        genitive_feminine: "~िरायाः", "~िरयोः", "~िराणाम्";
        genitive_neuter: "~िरस्य", "~िरयोः", "~िराणाम्";
        locative_masculine: "~िरे", "~िरयोः", "~िरेषु";
        locative_feminine: "~िरायाम्", "~िरयोः", "~िरासु";
        locative_neuter: "~िरे", "~िरयोः", "~िरेषु";
        vocative_masculine: "~िर", "~िरौ", "~िराः";
        vocative_feminine: "~िरे", "~िरे", "~िराः";
        vocative_neuter: "~िर", "~िरे", "~िराणि";
        comparative: "~ेयस्";
        superlative: "~ेष्ठ";
    ),
    // लीन: 1 words (कनीन).
    declension!("लीन", 1, "~ीन";
        nominative_masculine: "~ीनः", "~ीनौ", "~ीनाः";
        nominative_feminine: "~ीना", "~ीने", "~ीनाः";
        nominative_neuter: "~ीनम्", "~ीने", "~ीनानि";
        accusative_masculine: "~ीनम्", "~ीनौ", "~ीनान्";
        accusative_feminine: "~ीनाम्", "~ीने", "~ीनाः";
        accusative_neuter: "~ीनम्", "~ीने", "~ीनानि";
        instrumental_masculine: "~ीनेन", "~ीनाभ्याम्", "~ीनैः";
        instrumental_feminine: "~ीनया", "~ीनाभ्याम्", "~ीनाभिः";
        instrumental_neuter: "~ीनेन", "~ीनाभ्याम्", "~ीनैः";
        dative_masculine: "~ीनाय", "~ीनाभ्याम्", "~ीनेभ्यः";
        dative_feminine: "~ीनायै", "~ीनाभ्याम्", "~ीनाभ्यः";
        dative_neuter: "~ीनाय", "~ीनाभ्याम्", "~ीनेभ्यः";
        ablative_masculine: "~ीनात्", "~ीनाभ्याम्", "~ीनेभ्यः";
        ablative_feminine: "~ीनायाः", "~ीनाभ्याम्", "~ीनाभ्यः";
        ablative_neuter: "~ीनात्", "~ीनाभ्याम्", "~ीनेभ्यः";
        genitive_masculine: "~ीनस्य", "~ीनयोः", "~ीनानाम्";
        genitive_feminine: "~ीनायाः", "~ीनयोः", "~ीनानाम्";
        genitive_neuter: "~ीनस्य", "~ीनयोः", "~ीनानाम्";
        locative_masculine: "~ीने", "~ीनयोः", "~ीनेषु";
        locative_feminine: "~ीनायाम्", "~ीनयोः", "~ीनासु";
        locative_neuter: "~ीने", "~ीनयोः", "~ीनेषु";
        vocative_masculine: "~ीन", "~ीनौ", "~ीनाः";
        vocative_feminine: "~ीने", "~ीने", "~ीनाः";
        vocative_neuter: "~ीन", "~ीने", "~ीनानि";
        comparative: "~ीयस्";
        superlative: "~िष्ठ";
    ),
    // अनूप: 1 words (अल्प). Like धृत, except in 1 cell: superlative.
    declension!("अनूप", 1, "~", like धृत;
        nominative_masculine: _, _, _;
        nominative_feminine: _, _, _;
        nominative_neuter: _, _, _;
        accusative_masculine: _, _, _;
        accusative_feminine: _, _, _;
        accusative_neuter: _, _, _;
        instrumental_masculine: _, _, _;
        instrumental_feminine: _, _, _;
        instrumental_neuter: _, _, _;
        dative_masculine: _, _, _;
        dative_feminine: _, _, _;
        dative_neuter: _, _, _;
        ablative_masculine: _, _, _;
        ablative_feminine: _, _, _;
        ablative_neuter: _, _, _;
        genitive_masculine: _, _, _;
        genitive_feminine: _, _, _;
        genitive_neuter: _, _, _;
        locative_masculine: _, _, _;
        locative_feminine: _, _, _;
        locative_neuter: _, _, _;
        vocative_masculine: _, _, _;
        vocative_feminine: _, _, _;
        vocative_neuter: _, _, _;
        comparative: _;
        superlative: "~िष्ठ";
    ),
];
