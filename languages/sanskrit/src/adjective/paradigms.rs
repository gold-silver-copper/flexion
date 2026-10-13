//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the declension's it is like,
/// or, in a declension like no other, derived):
/// `nom_m` (sg, du, pl)
/// `nom_f` (sg, du, pl)
/// `nom_n` (sg, du, pl)
/// `acc_m` (sg, du, pl)
/// `acc_f` (sg, du, pl)
/// `acc_n` (sg, du, pl)
/// `ins_m` (sg, du, pl)
/// `ins_f` (sg, du, pl)
/// `ins_n` (sg, du, pl)
/// `dat_m` (sg, du, pl)
/// `dat_f` (sg, du, pl)
/// `dat_n` (sg, du, pl)
/// `abl_m` (sg, du, pl)
/// `abl_f` (sg, du, pl)
/// `abl_n` (sg, du, pl)
/// `gen_m` (sg, du, pl)
/// `gen_f` (sg, du, pl)
/// `gen_n` (sg, du, pl)
/// `loc_m` (sg, du, pl)
/// `loc_f` (sg, du, pl)
/// `loc_n` (sg, du, pl)
/// `voc_m` (sg, du, pl)
/// `voc_f` (sg, du, pl)
/// `voc_n` (sg, du, pl)
/// `comparative`
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     $(nom_m: $c0:tt, $c1:tt, $c2:tt;)?
     $(nom_f: $c3:tt, $c4:tt, $c5:tt;)?
     $(nom_n: $c6:tt, $c7:tt, $c8:tt;)?
     $(acc_m: $c9:tt, $c10:tt, $c11:tt;)?
     $(acc_f: $c12:tt, $c13:tt, $c14:tt;)?
     $(acc_n: $c15:tt, $c16:tt, $c17:tt;)?
     $(ins_m: $c18:tt, $c19:tt, $c20:tt;)?
     $(ins_f: $c21:tt, $c22:tt, $c23:tt;)?
     $(ins_n: $c24:tt, $c25:tt, $c26:tt;)?
     $(dat_m: $c27:tt, $c28:tt, $c29:tt;)?
     $(dat_f: $c30:tt, $c31:tt, $c32:tt;)?
     $(dat_n: $c33:tt, $c34:tt, $c35:tt;)?
     $(abl_m: $c36:tt, $c37:tt, $c38:tt;)?
     $(abl_f: $c39:tt, $c40:tt, $c41:tt;)?
     $(abl_n: $c42:tt, $c43:tt, $c44:tt;)?
     $(gen_m: $c45:tt, $c46:tt, $c47:tt;)?
     $(gen_f: $c48:tt, $c49:tt, $c50:tt;)?
     $(gen_n: $c51:tt, $c52:tt, $c53:tt;)?
     $(loc_m: $c54:tt, $c55:tt, $c56:tt;)?
     $(loc_f: $c57:tt, $c58:tt, $c59:tt;)?
     $(loc_n: $c60:tt, $c61:tt, $c62:tt;)?
     $(voc_m: $c63:tt, $c64:tt, $c65:tt;)?
     $(voc_f: $c66:tt, $c67:tt, $c68:tt;)?
     $(voc_n: $c69:tt, $c70:tt, $c71:tt;)?
     $(comparative: $c72:tt;)?
     $(superlative: $c73:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
                $(flexion::cell!($c48),)? "\t",
                $(flexion::cell!($c49),)? "\t",
                $(flexion::cell!($c50),)? "\t",
                $(flexion::cell!($c51),)? "\t",
                $(flexion::cell!($c52),)? "\t",
                $(flexion::cell!($c53),)? "\t",
                $(flexion::cell!($c54),)? "\t",
                $(flexion::cell!($c55),)? "\t",
                $(flexion::cell!($c56),)? "\t",
                $(flexion::cell!($c57),)? "\t",
                $(flexion::cell!($c58),)? "\t",
                $(flexion::cell!($c59),)? "\t",
                $(flexion::cell!($c60),)? "\t",
                $(flexion::cell!($c61),)? "\t",
                $(flexion::cell!($c62),)? "\t",
                $(flexion::cell!($c63),)? "\t",
                $(flexion::cell!($c64),)? "\t",
                $(flexion::cell!($c65),)? "\t",
                $(flexion::cell!($c66),)? "\t",
                $(flexion::cell!($c67),)? "\t",
                $(flexion::cell!($c68),)? "\t",
                $(flexion::cell!($c69),)? "\t",
                $(flexion::cell!($c70),)? "\t",
                $(flexion::cell!($c71),)? "\t",
                $(flexion::cell!($c72),)? "\t",
                $(flexion::cell!($c73),)? "\t",
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
    /// Like `धृत`: 1,072 words.
    धृत,
    /// Like `अवर`: 259 words.
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
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 33] = [
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 33] = [
    // धृत: 1,072 words (विश्व, ज, द, कमल, बुद्ध, स्निह्).
    declension!("धृत", 1072, "~";
    nom_m: "~ः", "~ौ", "~ाः";
    nom_f: "~ा", "~े", "~ाः";
    nom_n: "~म्", "~े", "~ानि";
    acc_m: "~म्", _, "~ान्";
    acc_f: "~ाम्", _, "~ाः";
    ins_m: "~ेन", "~ाभ्याम्", "~ैः";
    ins_f: "~या", "~ाभ्याम्", "~ाभिः";
    ins_n: "~ेन", "~ाभ्याम्", "~ैः";
    dat_m: "~ाय", _, "~ेभ्यः";
    dat_f: "~ायै", _, "~ाभ्यः";
    dat_n: "~ाय", _, "~ेभ्यः";
    abl_m: "~ात्", _, _;
    abl_f: "~ायाः", _, _;
    abl_n: "~ात्", _, _;
    gen_m: "~स्य", "~योः", "~ानाम्";
    gen_f: _, "~योः", "~ानाम्";
    gen_n: "~स्य", "~योः", "~ानाम्";
    loc_m: "~े", _, "~ेषु";
    loc_f: "~ायाम्", _, "~ासु";
    loc_n: "~े", _, "~ेषु";
    voc_m: "~", _, _;
    voc_f: "~े", _, _;
    voc_n: "~", _, _;
    comparative: "~तर";
    superlative: "~तम";
    ),
    // अवर: 259 words (मूर्ख, मधुर, ताम्र, सूर्य, कार्य, निर्मम). Like धृत, except in 6 cells: nom_n pl, ins_m sg, ins_n sg, gen_m pl, gen_f pl, gen_n pl.
    declension!("अवर", 259, "~", like धृत;
    nom_n: _, _, "~ाणि";
    ins_m: "~ेण", _, _;
    ins_n: "~ेण", _, _;
    gen_m: _, _, "~ाणाम्";
    gen_f: _, _, "~ाणाम्";
    gen_n: _, _, "~ाणाम्";
    ),
    // मूढ: 183 words (भारत, देव, गणित, विशेषण, लेखन, दैव). Like धृत, except in 17 cells: nom_f sg, nom_f du, nom_f pl, acc_f sg, acc_f pl, ins_f sg, ….
    declension!("मूढ", 183, "~", like धृत;
    nom_f: "~ी", "~्यौ", "~्यः";
    acc_f: "~ीम्", _, "~ीः";
    ins_f: "~्या", "~ीभ्याम्", "~ीभिः";
    dat_f: "~्यै", _, "~ीभ्यः";
    abl_f: "~्याः", _, _;
    gen_f: _, "~्योः", "~ीनाम्";
    loc_f: "~्याम्", _, "~ीषु";
    voc_f: "~ि", _, _;
    comparative: derived;
    ),
    // हार: 60 words (सर्प, कार, क्षौर, गौर, अक्ष, ऋक्ष). Like मूढ, except in 7 cells: nom_n pl, ins_m sg, ins_n sg, gen_m pl, gen_f pl, gen_n pl, ….
    declension!("हार", 60, "~", like मूढ;
    nom_n: _, _, "~ाणि";
    ins_m: "~ेण", _, _;
    ins_n: "~ेण", _, _;
    gen_m: _, _, "~ाणाम्";
    gen_f: _, _, "~ीणाम्";
    gen_n: _, _, "~ाणाम्";
    superlative: derived;
    ),
    // भाविन: 59 words (हस्तिन्, मायिन्, केशिन्, एकाकिन्, ज्ञानिन्, घातिन्).
    declension!("भाविन्", 59, "~िन्";
    nom_m: "~ी", "~िनौ", "~िनः";
    nom_f: "~िनी", "~िन्यौ", "~िन्यः";
    nom_n: "~ि", "~िनी", "~ीनि";
    acc_m: "~िनम्", _, "~िनः";
    acc_f: "~िनीम्", _, "~िनीः";
    ins_m: "~िना", "~िभ्याम्", "~िभिः";
    ins_f: "~िन्या", "~िनीभ्याम्", "~िनीभिः";
    ins_n: "~िना", "~िभ्याम्", "~िभिः";
    dat_m: "~िने", _, "~िभ्यः";
    dat_f: "~िन्यै", _, "~िनीभ्यः";
    dat_n: "~िने", _, "~िभ्यः";
    abl_m: "~िनः", _, _;
    abl_f: "~िन्याः", _, _;
    abl_n: "~िनः", _, _;
    gen_m: "~िनः", "~िनोः", "~िनाम्";
    gen_f: _, "~िन्योः", "~िनीनाम्";
    gen_n: "~िनः", "~िनोः", "~िनाम्";
    loc_m: "~िनि", _, "~िषु";
    loc_f: "~िन्याम्", _, "~िनीषु";
    loc_n: "~िनि", _, "~िषु";
    voc_m: "~िन्", _, _;
    voc_f: "~िनि", _, _;
    voc_n: "~ि|~िन्", _, _;
    ),
    // मनु: 34 words (वायु, पृदाकुसानु, मनु, स्थाणु, पाण्डु, सेतु).
    declension!("मनु", 34, "~ु";
    nom_m: "~ुः", "~ू", "~वः";
    nom_f: "~ुः", "~ू", "~वः";
    nom_n: "~ु", "~ुनी", "~ूनि";
    acc_m: "~ुम्", _, "~ून्";
    acc_f: "~ुम्", _, "~ूः";
    ins_m: "~ुना", "~ुभ्याम्", "~ुभिः";
    ins_f: "~्वा", "~ुभ्याम्", "~ुभिः";
    ins_n: "~ुना", "~ुभ्याम्", "~ुभिः";
    dat_m: "~वे", _, "~ुभ्यः";
    dat_f: "~वे|~्वै", _, "~ुभ्यः";
    dat_n: "~ुने|~वे", _, "~ुभ्यः";
    abl_m: "~ोः", _, _;
    abl_f: "~ोः|~्वाः", _, _;
    abl_n: "~ुनः|~ोः", _, _;
    gen_m: "~ोः", "~्वोः", "~ूनाम्";
    gen_f: "~ोः|~्वाः", "~्वोः", "~ूनाम्";
    gen_n: "~ुनः|~ोः", "~ुनोः|~्वोः", "~ूनाम्";
    loc_m: "~ौ", _, "~ुषु";
    loc_f: "~ौ|~्वाम्", _, "~ुषु";
    loc_n: "~ुनि|~ौ", "~ुनोः|~्वोः", "~ुषु";
    voc_m: "~ो", _, _;
    voc_f: "~ो", _, _;
    voc_n: "~ु|~ो", _, _;
    ),
    // शुचि: 32 words (संधि, कवि, अभि, गभस्ति, पुलस्ति, वसति). Like भाविन, except in 37 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("शुचि", 32, "~ि", like भाविन;
    nom_m: "~िः", "~ी", "~यः";
    nom_f: "~िः", "~ी", "~यः";
    acc_m: "~िम्", _, "~ीन्";
    acc_f: "~िम्", _, "~ीः";
    ins_f: "~्या", "~िभ्याम्", "~िभिः";
    dat_m: "~ये", _, _;
    dat_f: "~ये|~्यै", _, "~िभ्यः";
    dat_n: "~िने|~ये", _, _;
    abl_m: "~ेः", _, _;
    abl_f: "~ेः|~्याः", _, _;
    abl_n: "~िनः|~ेः", _, _;
    gen_m: "~ेः", "~्योः", "~ीनाम्";
    gen_f: "~ेः|~्याः", "~्योः", "~ीनाम्";
    gen_n: "~िनः|~ेः", "~िनोः|~्योः", "~ीनाम्";
    loc_m: "~ौ", _, _;
    loc_f: "~ौ|~्याम्", _, "~िषु";
    loc_n: "~िनि|~ौ", "~िनोः|~्योः", _;
    voc_m: "~े", _, _;
    voc_f: "~े", _, _;
    voc_n: "~ि|~े", _, _;
    ),
    // रसवत: 31 words (जगत्, अंशुमत्, हिमवत्, महत्, भगवत्, अन्तवत्).
    declension!("रसवत्", 31, "~त्";
    nom_m: "~ान्", "~न्तौ", "~न्तः";
    nom_f: "~ती", "~त्यौ", "~त्यः";
    nom_n: "~त्", "~ती", "~न्ति";
    acc_m: "~न्तम्", _, "~तः";
    acc_f: "~तीम्", _, "~तीः";
    ins_m: "~ता", "~द्भ्याम्", "~द्भिः";
    ins_f: "~त्या", "~तीभ्याम्", "~तीभिः";
    ins_n: "~ता", "~द्भ्याम्", "~द्भिः";
    dat_m: "~ते", _, "~द्भ्यः";
    dat_f: "~त्यै", _, "~तीभ्यः";
    dat_n: "~ते", _, "~द्भ्यः";
    abl_m: "~तः", _, _;
    abl_f: "~त्याः", _, _;
    abl_n: "~तः", _, _;
    gen_m: "~तः", "~तोः", "~ताम्";
    gen_f: _, "~त्योः", "~तीनाम्";
    gen_n: "~तः", "~तोः", "~ताम्";
    loc_m: "~ति", _, "~त्सु";
    loc_f: "~त्याम्", _, "~तीषु";
    loc_n: "~ति", _, "~त्सु";
    voc_m: "~न्", _, _;
    voc_f: "~ति", _, _;
    voc_n: "~त्", _, _;
    ),
    // हितकर: 23 words (अरित्र, अग्निहोत्र, चित्र, सिंहदंष्ट्र, दरिद्र, विचित्र).
    declension!("हितकर", 23, "~ि~~र";
    nom_m: "~ि~~रः", "~ि~~रौ", "~ि~~राः";
    nom_f: "~ि~~रा", "~ि~~रे", "~ि~~राः";
    nom_n: "~ि~~रम्", "~ि~~रे", "~ि~~राणि";
    acc_m: "~ि~~रम्", _, "~ि~~रान्";
    acc_f: "~ि~~राम्", _, "~ि~~राः";
    ins_m: "~ि~~रेण", "~ि~~राभ्याम्", "~ि~~रैः";
    ins_f: "~ि~~रया", "~ि~~राभ्याम्", "~ि~~राभिः";
    ins_n: "~ि~~रेण", "~ि~~राभ्याम्", "~ि~~रैः";
    dat_m: "~ि~~राय", _, "~ि~~रेभ्यः";
    dat_f: "~ि~~रायै", _, "~ि~~राभ्यः";
    dat_n: "~ि~~राय", _, "~ि~~रेभ्यः";
    abl_m: "~ि~~रात्", _, _;
    abl_f: "~ि~~रायाः", _, _;
    abl_n: "~ि~~रात्", _, _;
    gen_m: "~ि~~रस्य", "~ि~~रयोः", "~ि~~राणाम्";
    gen_f: _, "~ि~~रयोः", "~ि~~राणाम्";
    gen_n: "~ि~~रस्य", "~ि~~रयोः", "~ि~~राणाम्";
    loc_m: "~ि~~रे", _, "~ि~~रेषु";
    loc_f: "~ि~~रायाम्", _, "~ि~~रासु";
    loc_n: "~ि~~रे", _, "~ि~~रेषु";
    voc_m: "~ि~~र", _, _;
    voc_f: "~ि~~रे", _, _;
    voc_n: "~ि~~र", _, _;
    comparative: "~े~ीयस~";
    superlative: "~े~िष~ठ";
    ),
    // घटक: 22 words (आवश्यक, अंशक, जनक, नन्दक, खानक, पालक).
    declension!("घटक", 22, "~~";
    nom_m: "~~ः", "~~ौ", "~~ाः";
    nom_f: "~ि~ा", "~ि~े", "~ि~ाः";
    nom_n: "~~म्", "~~े", "~~ानि";
    acc_m: "~~म्", _, "~~ान्";
    acc_f: "~ि~ाम्", _, "~ि~ाः";
    ins_m: "~~ेन", "~~ाभ्याम्", "~~ैः";
    ins_f: "~ि~या", "~ि~ाभ्याम्", "~ि~ाभिः";
    ins_n: "~~ेन", "~~ाभ्याम्", "~~ैः";
    dat_m: "~~ाय", _, "~~ेभ्यः";
    dat_f: "~ि~ायै", _, "~ि~ाभ्यः";
    dat_n: "~~ाय", _, "~~ेभ्यः";
    abl_m: "~~ात्", _, _;
    abl_f: "~ि~ायाः", _, _;
    abl_n: "~~ात्", _, _;
    gen_m: "~~स्य", "~~योः", "~~ानाम्";
    gen_f: _, "~ि~योः", "~ि~ानाम्";
    gen_n: "~~स्य", "~~योः", "~~ानाम्";
    loc_m: "~~े", _, "~~ेषु";
    loc_f: "~ि~ायाम्", _, "~ि~ासु";
    loc_n: "~~े", _, "~~ेषु";
    voc_m: "~~", _, _;
    voc_f: "~ि~े", _, _;
    voc_n: "~~", _, _;
    ),
    // धायस: 19 words (रिशादस्, स्वपस्, सुश्रवस्, सुशेवस्, वीतिराधस्, वपुस्).
    declension!("धायस्", 19, "~स्";
    nom_m: "~ाः", "~सौ", "~सः";
    nom_f: "~ाः", "~सौ", "~सः";
    nom_n: "~ः", "~सी", "~ांसि";
    acc_m: "~सम्", _, "~सः";
    acc_f: "~सम्", _, "~सः";
    ins_m: "~सा", "~ोभ्याम्", "~ोभिः";
    ins_f: "~सा", "~ोभ्याम्", "~ोभिः";
    ins_n: "~सा", "~ोभ्याम्", "~ोभिः";
    dat_m: "~से", _, "~ोभ्यः";
    dat_f: "~से", _, "~ोभ्यः";
    dat_n: "~से", _, "~ोभ्यः";
    abl_m: "~सः", _, _;
    abl_f: "~सः", _, _;
    abl_n: "~सः", _, _;
    gen_m: "~सः", "~सोः", "~साम्";
    gen_f: _, "~सोः", "~साम्";
    gen_n: "~सः", "~सोः", "~साम्";
    loc_m: "~सि", _, "~ःसु|~स्सु";
    loc_f: "~सि", _, "~ःसु|~स्सु";
    loc_n: "~सि", _, "~ःसु|~स्सु";
    voc_m: "~ः", _, _;
    voc_f: "~ः", _, _;
    voc_n: "~ः", _, _;
    comparative: "~ीयस्";
    superlative: "~स्तम";
    ),
    // भारिन: 18 words (मन्त्रिन्, त्सारिन्, अवत्रापिन्, अहंकारिन्, रङ्गिन्, फिरङ्गिन्). Like भाविन, except in 36 cells: nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, nom_n du, ….
    declension!("भारिन्", 18, "~िन्", like भाविन;
    nom_m: _, "~िणौ", "~िणः";
    nom_f: "~िणी", "~िण्यौ", "~िण्यः";
    nom_n: _, "~िणी", "~ीणि";
    acc_m: "~िणम्", _, "~िणः";
    acc_f: "~िणीम्", _, "~िणीः";
    ins_m: "~िणा", _, _;
    ins_f: "~िण्या", "~िणीभ्याम्", "~िणीभिः";
    ins_n: "~िणा", _, _;
    dat_m: "~िणे", _, _;
    dat_f: "~िण्यै", _, "~िणीभ्यः";
    dat_n: "~िणे", _, _;
    abl_m: "~िणः", _, _;
    abl_f: "~िण्याः", _, _;
    abl_n: "~िणः", _, _;
    gen_m: "~िणः", "~िणोः", "~िणाम्";
    gen_f: _, "~िण्योः", "~िणीनाम्";
    gen_n: "~िणः", "~िणोः", "~िणाम्";
    loc_m: "~िणि", _, _;
    loc_f: "~िण्याम्", _, "~िणीषु";
    loc_n: "~िणि", _, _;
    voc_f: "~िणि", _, _;
    ),
    // ऋतावन: 15 words (वृत्रहन्, ऋतावन्, रक्षोहन्, वृषन्, हन्, यावन्).
    declension!("ऋतावन्", 15, "~न्";
    nom_m: "~ा", "~ानौ", "~ानः";
    nom_f: "~री", "~र्यौ", "~र्यः";
    nom_n: "~", "~्नी|~नी", "~ानि";
    acc_m: "~ानम्", _, "~्नः";
    acc_f: "~रीम्", _, "~रीः";
    acc_n: _, "~्नी|~नी", _;
    ins_m: "~्ना", "~भ्याम्", "~भिः";
    ins_f: "~र्या", "~रीभ्याम्", "~रीभिः";
    ins_n: "~्ना", "~भ्याम्", "~भिः";
    dat_m: "~्ने", _, "~भ्यः";
    dat_f: "~र्यै", _, "~रीभ्यः";
    dat_n: "~्ने", _, "~भ्यः";
    abl_m: "~्नः", _, _;
    abl_f: "~र्याः", _, _;
    abl_n: "~्नः", _, _;
    gen_m: "~्नः", "~्नोः", "~्नाम्";
    gen_f: _, "~र्योः", "~रीणाम्";
    gen_n: "~्नः", "~्नोः", "~्नाम्";
    loc_m: "~्नि|~नि", _, "~सु";
    loc_f: "~र्याम्", _, "~रीषु";
    loc_n: "~्नि|~नि", _, "~सु";
    voc_m: "~न्", _, _;
    voc_f: "~रि", _, _;
    voc_n: "~न्|~", "~्नी|~नी", _;
    ),
    // जयायस: 15 words (श्रेयस्, वस्यस्, मलीयस्, ज्यायस्, ओजीयस्, कनीयस्). Like धायस, except in 23 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("ज्यायस्", 15, "~स्", like धायस;
    nom_m: "~ान्", "~ांसौ", "~ांसः";
    nom_f: "~सी", "~स्यौ", "~स्यः";
    acc_m: "~ांसम्", _, _;
    acc_f: "~सीम्", _, "~सीः";
    ins_f: "~स्या", "~सीभ्याम्", "~सीभिः";
    dat_f: "~स्यै", _, "~सीभ्यः";
    abl_f: "~स्याः", _, _;
    gen_f: _, "~स्योः", "~सीनाम्";
    loc_f: "~स्याम्", _, "~सीषु";
    voc_m: "~न्", _, _;
    voc_f: "~सि", _, _;
    comparative: derived;
    superlative: derived;
    ),
    // धृषु: 13 words (दारु, तरु, पुरु, दीर्घायु, धृषु, रिपु). Like मनु, except in 13 cells: nom_n du, nom_n pl, ins_m sg, ins_n sg, dat_n sg, abl_n sg, ….
    declension!("धृषु", 13, "~ु", like मनु;
    nom_n: _, "~ुणी", "~ूणि";
    ins_m: "~ुणा", _, _;
    ins_n: "~ुणा", _, _;
    dat_n: "~ुणे|~वे", _, _;
    abl_n: "~ुणः|~ोः", _, _;
    gen_m: _, _, "~ूणाम्";
    gen_f: _, _, "~ूणाम्";
    gen_n: "~ुणः|~ोः", "~ुणोः|~्वोः", "~ूणाम्";
    loc_n: "~ुणि|~ौ", "~ुणोः|~्वोः", _;
    ),
    // लघु: 11 words (अणु, बहु, लघु, अंहु, आशु, वसु). Like मनु, except in 19 cells: nom_f sg, nom_f du, nom_f pl, acc_f sg, acc_f pl, ins_f sg, ….
    declension!("लघु", 11, "~ु", like मनु;
    nom_f: "~्वी", "~्व्यौ", "~्व्यः";
    acc_f: "~्वीम्", _, "~्वीः";
    ins_f: "~्व्या", "~्वीभ्याम्", "~्वीभिः";
    dat_f: "~्व्यै", _, "~्वीभ्यः";
    abl_f: "~्व्याः", _, _;
    gen_f: derived, "~्व्योः", "~्वीनाम्";
    loc_f: "~्व्याम्", _, "~्वीषु";
    voc_f: "~्वि", _, _;
    comparative: "~ीयस्";
    superlative: "~िष्ठ";
    ),
    // छिद: 9 words (उपस्थसद्, पुरुषाद्, सद्, विद्, छिद्, भिद्).
    declension!("छिद्", 9, "~द्";
    nom_m: "~त्", "~दौ", "~दः";
    nom_f: "~त्", "~दौ", "~दः";
    nom_n: "~त्", "~दी", "~न्दि";
    acc_m: "~दम्", _, "~दः";
    acc_f: "~दम्", _, "~दः";
    ins_m: "~दा", "~द्भ्याम्", "~द्भिः";
    ins_f: "~दा", "~द्भ्याम्", "~द्भिः";
    ins_n: "~दा", "~द्भ्याम्", "~द्भिः";
    dat_m: "~दे", _, "~द्भ्यः";
    dat_f: "~दे", _, "~द्भ्यः";
    dat_n: "~दे", _, "~द्भ्यः";
    abl_m: "~दः", _, _;
    abl_f: "~दः", _, _;
    abl_n: "~दः", _, _;
    gen_m: "~दः", "~दोः", "~दाम्";
    gen_f: _, "~दोः", "~दाम्";
    gen_n: "~दः", "~दोः", "~दाम्";
    loc_m: "~दि", _, "~त्सु";
    loc_f: "~दि", _, "~त्सु";
    loc_n: "~दि", _, "~त्सु";
    voc_m: "~त्", _, _;
    voc_f: "~त्", _, _;
    voc_n: "~त्", _, _;
    comparative: "~त्तर";
    superlative: "~त्तम";
    ),
    // तरुतृ: 8 words (यातृ, नेतृ, तरुतृ, गन्तृ, यन्तृ, पक्तृ).
    declension!("तरुतृ", 8, "~ृ";
    nom_m: "~ा", "~ारौ", "~ारः";
    nom_f: "~्री", "~्र्यौ", "~्र्यः";
    nom_n: "~ृ", "~ृणी", "~ॄणि";
    acc_m: "~ारम्", _, "~ॄन्";
    acc_f: "~्रीम्", _, "~्रीः";
    ins_m: "~्रा", "~ृभ्याम्", "~ृभिः";
    ins_f: "~्र्या", "~्रीभ्याम्", "~्रीभिः";
    ins_n: "~ृणा|~्रा", "~ृभ्याम्", "~ृभिः";
    dat_m: "~्रे", _, "~ृभ्यः";
    dat_f: "~्र्यै", _, "~्रीभ्यः";
    dat_n: "~ृणे|~्रे", _, "~ृभ्यः";
    abl_m: "~ुः", _, _;
    abl_f: "~्र्याः", _, _;
    abl_n: "~ृणः|~ुः", _, _;
    gen_m: "~ुः", "~्रोः", "~ॄणाम्";
    gen_f: _, "~्र्योः", "~्रीणाम्";
    gen_n: "~ृणः|~ुः", "~ृणोः|~्रोः", "~ॄणाम्";
    loc_m: "~रि", _, "~ृषु";
    loc_f: "~्र्याम्", _, "~्रीषु";
    loc_n: "~ृणि|~रि", "~ृणोः|~्रोः", "~ृषु";
    voc_m: "~ः", _, _;
    voc_f: "~्रि", _, _;
    voc_n: "~ृ|~ः", _, _;
    ),
    // कारक: 8 words (विदूषक, कारक, तारक, सारक, प्रियकारक, क्षुद्रक). Like घटक, except in 6 cells: nom_n pl, ins_m sg, ins_n sg, gen_m pl, gen_f pl, gen_n pl.
    declension!("कारक", 8, "~~", like घटक;
    nom_n: _, _, "~~ाणि";
    ins_m: "~~ेण", _, _;
    ins_n: "~~ेण", _, _;
    gen_m: _, _, "~~ाणाम्";
    gen_f: _, _, "~ि~ाणाम्";
    gen_n: _, _, "~~ाणाम्";
    ),
    // रथी: 8 words (श्री, प्री, वी, वीतभी, रथी, शी). Like शुचि, except in 34 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("रथी", 8, "~ी", like शुचि;
    nom_m: "~ीः", "~ियौ", "~ियः";
    nom_f: "~ीः", "~ियौ", "~ियः";
    acc_m: "~ियम्", _, "~ियः";
    acc_f: "~ियम्", _, "~ियः";
    ins_m: "~िया", "~ीभ्याम्", "~ीभिः";
    ins_f: "~िया", "~ीभ्याम्", "~ीभिः";
    dat_m: "~िये", _, "~ीभ्यः";
    dat_f: "~िये", _, "~ीभ्यः";
    abl_m: "~ियः", _, _;
    abl_f: "~ियः", _, _;
    gen_m: "~ियः", "~ियोः", "~ियाम्";
    gen_f: derived, "~ियोः", "~ियाम्";
    loc_m: "~ियि", _, "~ीषु";
    loc_f: "~ियि", _, "~ीषु";
    voc_m: "~ीः", _, _;
    voc_f: "~ीः", _, _;
    ),
    // सरवजित: 7 words (विद्युत्, दशत्, हरित्, सर्वजित्, च्युत्, श्चुत्). Like रसवत, except in 21 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("सर्वजित्", 7, "~त्", like रसवत;
    nom_m: "~त्", "~तौ", "~तः";
    nom_f: "~त्", "~तौ", "~तः";
    acc_m: "~तम्", _, _;
    acc_f: "~तम्", _, "~तः";
    ins_f: "~ता", "~द्भ्याम्", "~द्भिः";
    dat_f: "~ते", _, "~द्भ्यः";
    abl_f: "~तः", _, _;
    gen_f: _, "~तोः", "~ताम्";
    loc_f: "~ति", _, "~त्सु";
    voc_m: "~त्", _, _;
    voc_f: "~त्", _, _;
    ),
    // परिय: 6 words (प्रिय, अन्ताराष्ट्रिय, क्षत्रिय, इन्द्रिय, नष्टक्रिय, अनिन्द्रिय).
    declension!("प्रिय", 6, "~िय";
    nom_m: "~ियः", "~ियौ", "~ियाः";
    nom_f: "~िया", "~िये", "~ियाः";
    nom_n: "~ियम्", "~िये", "~ियाणि";
    acc_m: "~ियम्", _, "~ियान्";
    acc_f: "~ियाम्", _, "~ियाः";
    ins_m: "~ियेण", "~ियाभ्याम्", "~ियैः";
    ins_f: "~ियया", "~ियाभ्याम्", "~ियाभिः";
    ins_n: "~ियेण", "~ियाभ्याम्", "~ियैः";
    dat_m: "~ियाय", _, "~ियेभ्यः";
    dat_f: "~ियायै", _, "~ियाभ्यः";
    dat_n: "~ियाय", _, "~ियेभ्यः";
    abl_m: "~ियात्", _, _;
    abl_f: "~ियायाः", _, _;
    abl_n: "~ियात्", _, _;
    gen_m: "~ियस्य", "~िययोः", "~ियाणाम्";
    gen_f: _, "~िययोः", "~ियाणाम्";
    gen_n: "~ियस्य", "~िययोः", "~ियाणाम्";
    loc_m: "~िये", _, "~ियेषु";
    loc_f: "~ियायाम्", _, "~ियासु";
    loc_n: "~िये", _, "~ियेषु";
    voc_m: "~िय", _, _;
    voc_f: "~िये", _, _;
    voc_n: "~िय", _, _;
    comparative: "~ेयस्";
    superlative: "~ेष्ठ";
    ),
    // असत: 6 words (बृहत्, पृषत्, अर्हत्, असत्, ऋहत्, अजानत्). Like रसवत, except in 1 cell: nom_m sg.
    declension!("असत्", 6, "~त्", like रसवत;
    nom_m: "~न्", _, _;
    ),
    // महातमन: 6 words (प्रज्ञात्मन्, शर्मन्, महात्मन्, भस्मन्, युग्मन्, अकृतात्मन्). Like ऋतावन, except in 34 cells: nom_f sg, nom_f du, nom_f pl, nom_n du, acc_m pl, acc_f sg, ….
    declension!("महात्मन्", 6, "~न्", like ऋतावन;
    nom_f: "~ा", "~ानौ", "~ानः";
    nom_n: _, "~नी", _;
    acc_m: _, _, "~नः";
    acc_f: "~ानम्", _, "~नः";
    acc_n: _, derived, _;
    ins_m: "~ना", _, _;
    ins_f: "~ना", "~भ्याम्", "~भिः";
    ins_n: "~ना", _, _;
    dat_m: "~ने", _, _;
    dat_f: "~ने", _, "~भ्यः";
    dat_n: "~ने", _, _;
    abl_m: "~नः", _, _;
    abl_f: "~नः", _, _;
    abl_n: "~नः", _, _;
    gen_m: "~नः", "~नोः", "~नाम्";
    gen_f: _, "~नोः", "~नाम्";
    gen_n: "~नः", "~नोः", "~नाम्";
    loc_m: "~नि", _, _;
    loc_f: "~नि", _, "~सु";
    loc_n: "~नि", _, _;
    voc_f: "~न्", _, _;
    voc_n: _, derived, _;
    ),
    // उशिज: 6 words (विराज्, भुज्, उशिज्, त्यज्, रुज्, सयुज्).
    declension!("उशिज्", 6, "~ज्";
    nom_m: "~क्", "~जौ", "~जः";
    nom_f: "~क्", "~जौ", "~जः";
    nom_n: "~क्", "~जी", "~ञ्जि";
    acc_m: "~जम्", _, "~जः";
    acc_f: "~जम्", _, "~जः";
    ins_m: "~जा", "~ग्भ्याम्", "~ग्भिः";
    ins_f: "~जा", "~ग्भ्याम्", "~ग्भिः";
    ins_n: "~जा", "~ग्भ्याम्", "~ग्भिः";
    dat_m: "~जे", _, "~ग्भ्यः";
    dat_f: "~जे", _, "~ग्भ्यः";
    dat_n: "~जे", _, "~ग्भ्यः";
    abl_m: "~जः", _, _;
    abl_f: "~जः", _, _;
    abl_n: "~जः", _, _;
    gen_m: "~जः", "~जोः", "~जाम्";
    gen_f: _, "~जोः", "~जाम्";
    gen_n: "~जः", "~जोः", "~जाम्";
    loc_m: "~जि", _, "~क्षु";
    loc_f: "~जि", _, "~क्षु";
    loc_n: "~जि", _, "~क्षु";
    voc_m: "~क्", _, _;
    voc_f: "~क्", _, _;
    voc_n: "~क्", _, _;
    ),
    // पा: 6 words (सा, वर्चोदा, वर्चोधा, पा, रथेष्ठा, विश्वपा). Like धृत, except in 15 cells: nom_m sg, acc_m sg, acc_m pl, ins_m sg, ins_m pl, dat_m sg, ….
    declension!("पा", 6, "~ा", like धृत;
    nom_m: "~ाः", _, _;
    acc_m: "~ाम्", _, "~ः";
    ins_m: "~ा", _, "~ाभिः";
    dat_m: "~े", _, "~ाभ्यः";
    abl_m: "~ः", _, _;
    gen_m: "~ः", "~ोः", _;
    loc_m: "~ि", _, "~ासु";
    voc_m: "~ाः", _, _;
    comparative: derived;
    superlative: derived;
    ),
    // हरि: 6 words (हरि, अरि, करि, वध्रि, भूरि, सहुरि).
    declension!("हरि", 6, "~रि";
    nom_m: "~रिः", "~री", "~रयः";
    nom_f: "~रिः", "~री", "~रयः";
    nom_n: "~रि", "~रिणी", "~रीणि";
    acc_m: "~रिम्", _, "~रीन्";
    acc_f: "~रिम्", _, "~रीः";
    ins_m: "~रिणा", "~रिभ्याम्", "~रिभिः";
    ins_f: "~र्या", "~रिभ्याम्", "~रिभिः";
    ins_n: "~रिणा", "~रिभ्याम्", "~रिभिः";
    dat_m: "~रये", _, "~रिभ्यः";
    dat_f: "~रये", _, "~रिभ्यः";
    dat_n: "~रिणे", _, "~रिभ्यः";
    abl_m: "~रेः", _, _;
    abl_f: "~रेः", _, _;
    abl_n: "~रिणः", _, _;
    gen_m: "~रेः", "~र्योः", "~रीणाम्";
    gen_f: _, "~र्योः", "~रीणाम्";
    gen_n: "~रिणः", "~रिणोः", "~रीणाम्";
    loc_m: "~रौ", _, "~रिषु";
    loc_f: "~रौ", _, "~रिषु";
    loc_n: "~रिणि", _, "~रिषु";
    voc_m: "~रे", _, _;
    voc_f: "~रे", _, _;
    voc_n: "~रि", _, _;
    comparative: "~यस्";
    superlative: "~यिष्ठ";
    ),
    // सपृश: 5 words (सदृश्, स्पृश्, हृदिस्पृश्, कीदृश्, ईदृश्). Like उशिज, except in 30 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n du, nom_n pl, ….
    declension!("स्पृश्", 5, "~श्", like उशिज;
    nom_m: _, "~शौ", "~शः";
    nom_f: _, "~शौ", "~शः";
    nom_n: _, "~शी", "~ंशि";
    acc_m: "~शम्", _, "~शः";
    acc_f: "~शम्", _, "~शः";
    ins_m: "~शा", _, _;
    ins_f: "~शा", _, _;
    ins_n: "~शा", _, _;
    dat_m: "~शे", _, _;
    dat_f: "~शे", _, _;
    dat_n: "~शे", _, _;
    abl_m: "~शः", _, _;
    abl_f: "~शः", _, _;
    abl_n: "~शः", _, _;
    gen_m: "~शः", "~शोः", "~शाम्";
    gen_f: _, "~शोः", "~शाम्";
    gen_n: "~शः", "~शोः", "~शाम्";
    loc_m: "~शि", _, _;
    loc_f: "~शि", _, _;
    loc_n: "~शि", _, _;
    ),
    // नीरोग: 4 words (दीर्घ, शीर्षक, धीर्य, नीरोग).
    declension!("नीरोग", 4, "~ीर~~";
    nom_m: "~ीर~~ः", "~ीर~~ौ", "~ीर~~ाः";
    nom_f: "~ीर~~ा", "~ीर~~े", "~ीर~~ाः";
    nom_n: "~ीर~~म्", "~ीर~~े", "~ीर~~ाणि";
    acc_m: "~ीर~~म्", _, "~ीर~~ान्";
    acc_f: "~ीर~~ाम्", _, "~ीर~~ाः";
    ins_m: "~ीर~~ेण", "~ीर~~ाभ्याम्", "~ीर~~ैः";
    ins_f: "~ीर~~या", "~ीर~~ाभ्याम्", "~ीर~~ाभिः";
    ins_n: "~ीर~~ेण", "~ीर~~ाभ्याम्", "~ीर~~ैः";
    dat_m: "~ीर~~ाय", _, "~ीर~~ेभ्यः";
    dat_f: "~ीर~~ायै", _, "~ीर~~ाभ्यः";
    dat_n: "~ीर~~ाय", _, "~ीर~~ेभ्यः";
    abl_m: "~ीर~~ात्", _, _;
    abl_f: "~ीर~~ायाः", _, _;
    abl_n: "~ीर~~ात्", _, _;
    gen_m: "~ीर~~स्य", "~ीर~~योः", "~ीर~~ाणाम्";
    gen_f: _, "~ीर~~योः", "~ीर~~ाणाम्";
    gen_n: "~ीर~~स्य", "~ीर~~योः", "~ीर~~ाणाम्";
    loc_m: "~ीर~~े", _, "~ीर~~ेषु";
    loc_f: "~ीर~~ायाम्", _, "~ीर~~ासु";
    loc_n: "~ीर~~े", _, "~ीर~~ेषु";
    voc_m: "~ीर~~", _, _;
    voc_f: "~ीर~~े", _, _;
    voc_n: "~ीर~~", _, _;
    comparative: "~~रा~ीयस्";
    superlative: "~~रा~िष्ठ";
    ),
    // अमनस: 4 words (अमनस्, क्लान्तमनस्, रंहस्, तरस्). Like जयायस, except in 5 cells: nom_m sg, nom_m du, nom_m pl, acc_m sg, voc_m sg.
    declension!("अमनस्", 4, "~स्", like जयायस;
    nom_m: "~ाः", "~सौ", "~सः";
    acc_m: "~सम्", _, _;
    voc_m: "~ः", _, _;
    ),
    // वामन: 3 words (नव, घन, पक्व). Like धृत, except in 2 cells: comparative, superlative.
    declension!("वामन", 3, "~", like धृत;
    comparative: "~ीयस्";
    superlative: "~िष्ठ";
    ),
    // बहुवरीहि: 3 words (ब्राह्मि, बहुव्रीहि, द्रापि). Like शुचि, except in 13 cells: nom_n du, nom_n pl, ins_m sg, ins_n sg, dat_n sg, abl_n sg, ….
    declension!("बहुव्रीहि", 3, "~ि", like शुचि;
    nom_n: _, "~िणी", "~ीणि";
    ins_m: "~िणा", _, _;
    ins_n: "~िणा", _, _;
    dat_n: "~िणे|~ये", _, _;
    abl_n: "~िणः|~ेः", _, _;
    gen_m: _, _, "~ीणाम्";
    gen_f: _, _, "~ीणाम्";
    gen_n: "~िणः|~ेः", "~िणोः|~्योः", "~ीणाम्";
    loc_n: "~िणि|~ौ", "~िणोः|~्योः", _;
    ),
    // रघु: 3 words (अगुरु, प्रभु, रघु). Like लघु, except in 13 cells: nom_n du, nom_n pl, ins_m sg, ins_n sg, dat_n sg, abl_n sg, ….
    declension!("रघु", 3, "~ु", like लघु;
    nom_n: _, "~ुणी", "~ूणि";
    ins_m: "~ुणा", _, _;
    ins_n: "~ुणा", _, _;
    dat_n: "~ुणे|~वे", _, _;
    abl_n: "~ुणः|~ोः", _, _;
    gen_m: _, _, "~ूणाम्";
    gen_f: _, _, "~्वीणाम्";
    gen_n: "~ुणः|~ोः", "~ुणोः|~्वोः", "~ूणाम्";
    loc_n: "~ुणि|~ौ", "~ुणोः|~्वोः", _;
    ),
];
