//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`A`, `B`, `C`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem adjective_m: $s0:tt;)?
     $(stem adjective_f: $s1:tt;)?
     $(stem adjective_n: $s2:tt;)?
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
            reads: view!($($($view)?)?),
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
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
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

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`superlative_as_comparative`: one value read as another).
const SUPERLATIVE_AS_COMPARATIVE: [u16; 74] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 72,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`f_as_m`: one value read as another).
const F_AS_M: [u16; 74] = [
    0, 1, 2, 0, 1, 2, 6, 7, 8, 9, 10, 11, 9, 10, 11, 15, 16, 17, 18, 19, 20, 18, 19, 20, 24, 25,
    26, 27, 28, 29, 27, 28, 29, 33, 34, 35, 36, 37, 38, 36, 37, 38, 42, 43, 44, 45, 46, 47, 45, 46,
    47, 51, 52, 53, 54, 55, 56, 54, 55, 56, 60, 61, 62, 63, 64, 65, 63, 64, 65, 69, 70, 71, 72, 73,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`m_as_f`: one value read as another).
const M_AS_F: [u16; 74] = [
    3, 4, 5, 3, 4, 5, 6, 7, 8, 12, 13, 14, 12, 13, 14, 15, 16, 17, 21, 22, 23, 21, 22, 23, 24, 25,
    26, 30, 31, 32, 30, 31, 32, 33, 34, 35, 39, 40, 41, 39, 40, 41, 42, 43, 44, 48, 49, 50, 48, 49,
    50, 51, 52, 53, 57, 58, 59, 57, 58, 59, 60, 61, 62, 66, 67, 68, 66, 67, 68, 69, 70, 71, 72, 73,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`loc_as_voc`: one value read as another).
const LOC_AS_VOC: [u16; 74] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 63, 64, 65, 66, 67, 68, 69, 70, 71, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `धृत`: 1,057 words.
    धृत,
    /// Like `अवर`: 254 words.
    अवर,
    /// Like `मूढ`: 183 words.
    मूढ,
    /// Like `भाविन्`: 62 words.
    भाविन,
    /// Like `हार`: 60 words.
    हार,
    /// Like `मनु`: 34 words.
    मनु,
    /// Like `शुचि`: 32 words.
    शुचि,
    /// Like `रसवत्`: 31 words.
    रसवत,
    /// Like `घटक`: 30 words.
    घटक,
    /// Like `हितकर`: 23 words.
    हितकर,
    /// Like `धायस्`: 19 words.
    धायस,
    /// Like `भारिन्`: 15 words.
    भारिन,
    /// Like `ज्यायस्`: 15 words.
    जयायस,
    /// Like `ऋतावन्`: 15 words.
    ऋतावन,
    /// Like `धृषु`: 13 words.
    धृषु,
    /// Like `चिर`: 13 words.
    चिर,
    /// Like `लघु`: 11 words.
    लघु,
    /// Like `छिद्`: 9 words.
    छिद,
    /// Like `बहुव्रीहि`: 9 words.
    बहुवरीहि,
    /// Like `तरुतृ`: 8 words.
    तरुतृ,
    /// Like `सर्वजित्`: 7 words.
    सरवजित,
    /// Like `असत्`: 6 words.
    असत,
    /// Like `उशिज्`: 6 words.
    उशिज,
    /// Like `पा`: 6 words.
    पा,
    /// Like `महात्मन्`: 6 words.
    महातमन,
    /// Like `स्पृश्`: 5 words.
    सपृश,
    /// Like `वी`: 5 words.
    वी,
    /// Like `अमनस्`: 4 words.
    अमनस,
    /// Like `न्यञ्च्`: 4 words.
    नयञच,
    /// Like `वामन`: 3 words.
    वामन,
    /// Like `इतर`: 3 words.
    इतर,
    /// Like `रघु`: 3 words.
    रघु,
    /// Like `दुह्`: 3 words.
    दुह,
    /// Like `रथी`: 3 words.
    रथी,
    /// Like `मुच्`: 2 words.
    मुच,
    /// Like `प्रबुध्`: 1 words.
    परबुध,
    /// Like `एवंगत`: 1 words.
    एवंगत,
    /// Like `द्विष्`: 1 words.
    दविष,
    /// Like `निर्ऋति`: 1 words.
    निरऋति,
    /// Like `नैर्ऋत`: 1 words.
    नैरऋत,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 40] = [
        Declension::धृत,
        Declension::अवर,
        Declension::मूढ,
        Declension::भाविन,
        Declension::हार,
        Declension::मनु,
        Declension::शुचि,
        Declension::रसवत,
        Declension::घटक,
        Declension::हितकर,
        Declension::धायस,
        Declension::भारिन,
        Declension::जयायस,
        Declension::ऋतावन,
        Declension::धृषु,
        Declension::चिर,
        Declension::लघु,
        Declension::छिद,
        Declension::बहुवरीहि,
        Declension::तरुतृ,
        Declension::सरवजित,
        Declension::असत,
        Declension::उशिज,
        Declension::पा,
        Declension::महातमन,
        Declension::सपृश,
        Declension::वी,
        Declension::अमनस,
        Declension::नयञच,
        Declension::वामन,
        Declension::इतर,
        Declension::रघु,
        Declension::दुह,
        Declension::रथी,
        Declension::मुच,
        Declension::परबुध,
        Declension::एवंगत,
        Declension::दविष,
        Declension::निरऋति,
        Declension::नैरऋत,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 40] = [
    // धृत: 1,057 words (ज, द, कमल, बुद्ध, सह, आद).
    declension!("धृत", 1057, "~";
    nom_m: "~ः", "~ौ", "~ाः";
    nom_f: "~ा", "~े", "~ाः";
    nom_n: "~म्", "~े", "~ाAि";
    acc_m: "~म्", _, "~ान्";
    acc_f: "~ाम्", _, "~ाः";
    ins_m: "~ेA", "~ाभ्याम्", "~ैः";
    ins_f: "~या", "~ाभ्याम्", "~ाभिः";
    dat_m: "~ाय", _, "~ेभ्यः";
    dat_f: "~ायै", _, "~ाभ्यः";
    dat_n: "~ाय", _, _;
    abl_m: "~ात्", _, _;
    abl_f: "~ायाः", _, _;
    abl_n: "~ात्", _, _;
    gen_m: "~स्य", "~योः", "~ाAाम्";
    gen_f: _, "~योः", "~ाAाम्";
    gen_n: "~स्य", "~योः", _;
    loc_m: "~े", _, "~ेषु";
    loc_f: "~ायाम्", _, "~ासु";
    loc_n: "~े", _, _;
    voc_m: "~", _, _;
    voc_f: "~े", _, _;
    voc_n: "~", _, _;
    comparative: "~तर";
    superlative: "~तम";
    ),
    // अवर: 254 words (मूर्ख, मधुर, ताम्र, प्रिय, सूर्य, कार्य). Like धृत, except in 4 cells: nom_n pl, ins_m sg, gen_m pl, gen_f pl.
    declension!("अवर", 254, "~", like धृत;
    nom_n: _, _, "~ाणि";
    ins_m: "~ेण", _, _;
    gen_m: _, _, "~ाणाम्";
    gen_f: _, _, "~ाणाम्";
    ),
    // मूढ: 183 words (भारत, देव, गणित, विशेषण, लेखन, दैव). Like धृत, except in 17 cells: nom_f sg, nom_f du, nom_f pl, acc_f sg, acc_f pl, ins_f sg, ….
    declension!("मूढ", 183, "~", like धृत;
    nom_f: "~ी", "~्यौ", "~्यः";
    acc_f: "~ीम्", _, "~ीः";
    ins_f: "~्या", "~ीभ्याम्", "~ीभिः";
    dat_f: "~्यै", _, "~ीभ्यः";
    abl_f: "~्याः", _, _;
    gen_f: _, "~्योः", "~ीAाम्";
    loc_f: "~्याम्", _, "~ीषु";
    voc_f: "~ि", _, _;
    comparative: derived;
    ),
    // भाविन: 62 words (हस्तिन्, मायिन्, केशिन्, एकाकिन्, ज्ञानिन्, घातिन्).
    declension!("भाविन्", 62, "~िन्";
    nom_m: "~ी", "~िAौ", "~िAः";
    nom_f: "~िAी", "~िन्यौ", "~िन्यः";
    nom_n: "~ि", "~िAी", "~ीAि";
    acc_m: "~िAम्", _, "~िAः";
    acc_f: "~िAीम्", _, "~िAीः";
    ins_m: "~िAा", "~िभ्याम्", "~िभिः";
    ins_f: "~िन्या", "~िAीभ्याम्", "~िAीभिः";
    dat_m: "~िAे", _, "~िभ्यः";
    dat_f: "~िन्यै", _, "~िAीभ्यः";
    dat_n: "~िAे", _, _;
    abl_m: "~िAः", _, _;
    abl_f: "~िन्याः", _, _;
    abl_n: "~िAः", _, _;
    gen_m: "~िAः", "~िAोः", "~िAाम्";
    gen_f: _, "~िन्योः", "~िAीनाम्";
    gen_n: "~िAः", "~िAोः", _;
    loc_m: "~िAि", _, "~िषु";
    loc_f: "~िन्याम्", _, "~िAीषु";
    loc_n: "~िAि", _, _;
    voc_m: "~िन्", _, _;
    voc_f: "~िAि", _, _;
    voc_n: "~ि|~िन्", _, _;
    ),
    // हार: 60 words (सर्प, कार, क्षौर, गौर, अक्ष, ऋक्ष). Like मूढ, except in 4 cells: nom_n pl, ins_m sg, gen_m pl, gen_f pl.
    declension!("हार", 60, "~", like मूढ as SUPERLATIVE_AS_COMPARATIVE;
    nom_n: _, _, "~ाणि";
    ins_m: "~ेण", _, _;
    gen_m: _, _, "~ाणाम्";
    gen_f: _, _, "~ीणाम्";
    ),
    // मनु: 34 words (वायु, पृदाकुसानु, मनु, स्थाणु, पाण्डु, सेतु).
    declension!("मनु", 34, "~ु";
    nom_m: "~ुः", "~ू", "~वः";
    nom_f: "~ुः", "~ू", "~वः";
    nom_n: "~ु", "~ुAी", "~ूAि";
    acc_m: "~ुम्", _, "~ूA्";
    acc_f: "~ुम्", _, "~ूः";
    ins_m: "~ुAा", "~ुभ्याम्", "~ुभिः";
    ins_f: "~्वा", "~ुभ्याम्", "~ुभिः";
    dat_m: "~वे", _, "~ुभ्यः";
    dat_f: "~वे|~्वै", _, "~ुभ्यः";
    dat_n: "~ुAे|~वे", _, _;
    abl_m: "~ोः", _, _;
    abl_f: "~ोः|~्वाः", _, _;
    abl_n: "~ुAः|~ोः", _, _;
    gen_m: "~ोः", "~्वोः", "~ूAाम्";
    gen_f: _, "~्वोः", "~ूAाम्";
    gen_n: "~ुAः|~ोः", "~ुAोः|~्वोः", _;
    loc_m: "~ौ", _, "~ुषु";
    loc_f: "~ौ|~्वाम्", _, "~ुषु";
    loc_n: "~ुAि|~ौ", _, _;
    voc_m: "~ो", _, _;
    voc_f: "~ो", _, _;
    voc_n: "~ु|~ो", _, _;
    ),
    // शुचि: 32 words (संधि, कवि, अभि, गभस्ति, पुलस्ति, वसति). Like भाविन, except in 31 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("शुचि", 32, "~ि", like भाविन as F_AS_M;
    nom_m: "~िः", "~ी", "~यः";
    nom_f: "~िः", "~ी", "~यः";
    acc_m: "~िम्", _, "~ीA्";
    acc_f: "~िम्", _, "~ीः";
    ins_f: "~्या", _, _;
    dat_m: "~ये", _, _;
    dat_f: "~ये|~्यै", _, _;
    dat_n: "~िAे|~ये", _, _;
    abl_m: "~ेः", _, _;
    abl_f: "~ेः|~्याः", _, _;
    abl_n: "~िAः|~ेः", _, _;
    gen_m: "~ेः", "~्योः", "~ीAाम्";
    gen_f: derived, "~्योः", "~ीAाम्";
    gen_n: "~िAः|~ेः", "~िAोः|~्योः", _;
    loc_m: "~ौ", _, _;
    loc_f: "~ौ|~्याम्", _, _;
    loc_n: "~िAि|~ौ", _, _;
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
    dat_m: "~ते", _, "~द्भ्यः";
    dat_f: "~त्यै", _, "~तीभ्यः";
    dat_n: "~ते", _, _;
    abl_m: "~तः", _, _;
    abl_f: "~त्याः", _, _;
    abl_n: "~तः", _, _;
    gen_m: "~तः", "~तोः", "~ताम्";
    gen_f: _, "~त्योः", "~तीAाम्";
    gen_n: "~तः", "~तोः", _;
    loc_m: "~ति", _, "~त्सु";
    loc_f: "~त्याम्", _, "~तीषु";
    loc_n: "~ति", _, _;
    voc_m: "~न्", _, _;
    voc_f: "~ति", _, _;
    voc_n: "~त्", _, _;
    ),
    // घटक: 30 words (आवश्यक, अंशक, विदूषक, जनक, नन्दक, खानक).
    declension!("घटक", 30, "~~";
    nom_m: "~~ः", "~~ौ", "~~ाः";
    nom_f: "~ि~ा", "~ि~े", "~ि~ाः";
    nom_n: "~~म्", "~~े", "~~ाAि";
    acc_m: "~~म्", _, "~~ान्";
    acc_f: "~ि~ाम्", _, "~ि~ाः";
    ins_m: "~~ेA", "~~ाभ्याम्", "~~ैः";
    ins_f: "~ि~या", "~ि~ाभ्याम्", "~ि~ाभिः";
    dat_m: "~~ाय", _, "~~ेभ्यः";
    dat_f: "~ि~ायै", _, "~ि~ाभ्यः";
    dat_n: "~~ाय", _, _;
    abl_m: "~~ात्", _, _;
    abl_f: "~ि~ायाः", _, _;
    abl_n: "~~ात्", _, _;
    gen_m: "~~स्य", "~~योः", "~~ाAाम्";
    gen_f: _, "~ि~योः", "~ि~ाAाम्";
    gen_n: "~~स्य", "~~योः", _;
    loc_m: "~~े", _, "~~ेषु";
    loc_f: "~ि~ायाम्", _, "~ि~ासु";
    loc_n: "~~े", _, _;
    voc_m: "~~", _, _;
    voc_f: "~ि~े", _, _;
    voc_n: "~~", _, _;
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
    dat_m: "~ि~~राय", _, "~ि~~रेभ्यः";
    dat_f: "~ि~~रायै", _, "~ि~~राभ्यः";
    dat_n: "~ि~~राय", _, _;
    abl_m: "~ि~~रात्", _, _;
    abl_f: "~ि~~रायाः", _, _;
    abl_n: "~ि~~रात्", _, _;
    gen_m: "~ि~~रस्य", "~ि~~रयोः", "~ि~~राणाम्";
    gen_f: _, "~ि~~रयोः", "~ि~~राणाम्";
    gen_n: "~ि~~रस्य", "~ि~~रयोः", _;
    loc_m: "~ि~~रे", _, "~ि~~रेषु";
    loc_f: "~ि~~रायाम्", _, "~ि~~रासु";
    loc_n: "~ि~~रे", _, _;
    voc_m: "~ि~~र", _, _;
    voc_f: "~ि~~रे", _, _;
    voc_n: "~ि~~र", _, _;
    comparative: "~े~ीयस~";
    superlative: "~े~िष~ठ";
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
    dat_m: "~से", _, "~ोभ्यः";
    dat_f: "~से", _, "~ोभ्यः";
    dat_n: "~से", _, _;
    abl_m: "~सः", _, _;
    abl_f: "~सः", _, _;
    abl_n: "~सः", _, _;
    gen_m: "~सः", "~सोः", "~साम्";
    gen_f: _, "~सोः", "~साम्";
    gen_n: "~सः", "~सोः", _;
    loc_m: "~सि", _, "~ःसु|~स्सु";
    loc_f: "~सि", _, "~ःसु|~स्सु";
    loc_n: "~सि", _, _;
    voc_m: "~ः", _, _;
    voc_f: "~ः", _, _;
    voc_n: "~ः", _, _;
    comparative: "~ीयस्";
    superlative: "~स्तम";
    ),
    // भारिन: 15 words (मन्त्रिन्, अवत्रापिन्, अहंकारिन्, रङ्गिन्, फिरङ्गिन्, करिन्). Like भाविन, except in 34 cells: nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, nom_n du, ….
    declension!("भारिन्", 15, "~िन्", like भाविन;
    nom_m: _, "~िणौ", "~िणः";
    nom_f: "~िणी", "~िण्यौ", "~िण्यः";
    nom_n: _, "~िणी", "~ीणि";
    acc_m: "~िणम्", _, "~िणः";
    acc_f: "~िणीम्", _, "~िणीः";
    ins_m: "~िणा", _, _;
    ins_f: "~िण्या", "~िणीभ्याम्", "~िणीभिः";
    dat_m: "~िणे", _, _;
    dat_f: "~िण्यै", _, "~िणीभ्यः";
    dat_n: "~िणे", _, _;
    abl_m: "~िणः", _, _;
    abl_f: "~िण्याः", _, _;
    abl_n: "~िणः", _, _;
    gen_m: "~िणः", "~िणोः", "~िणाम्";
    gen_f: _, "~िण्योः", "~िणीनाम्";
    gen_n: "~िणः", "~िणोः", _;
    loc_m: "~िणि", _, _;
    loc_f: "~िण्याम्", _, "~िणीषु";
    loc_n: "~िणि", _, _;
    voc_f: "~िणि", _, _;
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
    gen_f: _, "~स्योः", "~सीAाम्";
    loc_f: "~स्याम्", _, "~सीषु";
    voc_m: "~न्", _, _;
    voc_f: "~सि", _, _;
    comparative: derived;
    superlative: derived;
    ),
    // ऋतावन: 15 words (वृत्रहन्, ऋतावन्, रक्षोहन्, वृषन्, हन्, यावन्).
    declension!("ऋतावन्", 15, "~न्";
    nom_m: "~ा", "~ाAौ", "~ाAः";
    nom_f: "~री", "~र्यौ", "~र्यः";
    nom_n: "~", "~्Aी|~नी", "~ाAि";
    acc_m: "~ाAम्", _, "~्Aः";
    acc_f: "~रीम्", _, "~रीः";
    ins_m: "~्Aा", "~भ्याम्", "~भिः";
    ins_f: "~र्या", "~रीभ्याम्", "~रीभिः";
    dat_m: "~्Aे", _, "~भ्यः";
    dat_f: "~र्यै", _, "~रीभ्यः";
    dat_n: "~्Aे", _, _;
    abl_m: "~्Aः", _, _;
    abl_f: "~र्याः", _, _;
    abl_n: "~्Aः", _, _;
    gen_m: "~्Aः", "~्Aोः", "~्Aाम्";
    gen_f: _, "~र्योः", "~रीAाम्";
    gen_n: "~्Aः", "~्Aोः", _;
    loc_m: "~्Aि|~नि", _, "~सु";
    loc_f: "~र्याम्", _, "~रीषु";
    loc_n: "~्Aि|~नि", _, _;
    voc_m: "~न्", _, _;
    voc_f: "~रि", _, _;
    voc_n: "~न्|~", _, _;
    ),
    // धृषु: 13 words (दारु, तरु, पुरु, दीर्घायु, धृषु, रिपु). Like मनु, except in 1 cell: acc_m pl.
    declension!("धृषु", 13, "~ु", like मनु;
    acc_m: _, _, "~ून्";
    ),
    // चिर: 13 words (शिशिर, अजिर, रुधिर, स्थिर, बधिर, मेधिर).
    declension!("चिर", 13, "~िर";
    nom_m: "~िरः", "~िरौ", "~िराः";
    nom_f: "~िरा", "~िरे", "~िराः";
    nom_n: "~िरम्", "~िरे", "~िराAि";
    acc_m: "~िरम्", _, "~िरान्";
    acc_f: "~िराम्", _, "~िराः";
    ins_m: "~िरेA", "~िराभ्याम्", "~िरैः";
    ins_f: "~िरया", "~िराभ्याम्", "~िराभिः";
    dat_m: "~िराय", _, "~िरेभ्यः";
    dat_f: "~िरायै", _, "~िराभ्यः";
    dat_n: "~िराय", _, _;
    abl_m: "~िरात्", _, _;
    abl_f: "~िरायाः", _, _;
    abl_n: "~िरात्", _, _;
    gen_m: "~िरस्य", "~िरयोः", "~िराAाम्";
    gen_f: _, "~िरयोः", "~िराAाम्";
    gen_n: "~िरस्य", "~िरयोः", _;
    loc_m: "~िरे", _, "~िरेषु";
    loc_f: "~िरायाम्", _, "~िरासु";
    loc_n: "~िरे", _, _;
    voc_m: "~िर", _, _;
    voc_f: "~िरे", _, _;
    voc_n: "~िर", _, _;
    comparative: "~ेयस्";
    superlative: "~ेष्ठ";
    ),
    // लघु: 11 words (अणु, बहु, लघु, अंहु, आशु, वसु). Like मनु, except in 18 cells: nom_f sg, nom_f du, nom_f pl, acc_f sg, acc_f pl, ins_f sg, ….
    declension!("लघु", 11, "~ु", like मनु;
    nom_f: "~्वी", "~्व्यौ", "~्व्यः";
    acc_f: "~्वीम्", _, "~्वीः";
    ins_f: "~्व्या", "~्वीभ्याम्", "~्वीभिः";
    dat_f: "~्व्यै", _, "~्वीभ्यः";
    abl_f: "~्व्याः", _, _;
    gen_f: _, "~्व्योः", "~्वीAाम्";
    loc_f: "~्व्याम्", _, "~्वीषु";
    voc_f: "~्वि", _, _;
    comparative: "~ीयस्";
    superlative: "~िष्ठ";
    ),
    // छिद: 9 words (उपस्थसद्, पुरुषाद्, सद्, विद्, छिद्, भिद्). Like रसवत, except in 35 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("छिद्", 9, "~द्", like रसवत as F_AS_M;
    nom_m: "~त्", "~दौ", "~दः";
    nom_f: "~त्", "~दौ", "~दः";
    nom_n: _, "~दी", "~न्दि";
    acc_m: "~दम्", _, "~दः";
    acc_f: "~दम्", _, "~दः";
    ins_m: "~दा", _, _;
    ins_f: "~दा", _, _;
    dat_m: "~दे", _, _;
    dat_f: "~दे", _, _;
    dat_n: "~दे", _, _;
    abl_m: "~दः", _, _;
    abl_f: "~दः", _, _;
    abl_n: "~दः", _, _;
    gen_m: "~दः", "~दोः", "~दाम्";
    gen_f: derived, "~दोः", "~दाम्";
    gen_n: "~दः", "~दोः", _;
    loc_m: "~दि", _, _;
    loc_f: "~दि", _, _;
    loc_n: "~दि", _, _;
    voc_m: "~त्", _, _;
    voc_f: "~त्", _, _;
    comparative: "~त्तर";
    superlative: "~त्तम";
    ),
    // बहुवरीहि: 9 words (ब्राह्मि, हरि, बहुव्रीहि, अरि, द्रापि, करि). Like शुचि, except in 1 cell: acc_m pl.
    declension!("बहुव्रीहि", 9, "~ि", like शुचि;
    acc_m: _, _, "~ीन्";
    ),
    // तरुतृ: 8 words (यातृ, नेतृ, तरुतृ, गन्तृ, यन्तृ, पक्तृ).
    declension!("तरुतृ", 8, "~ृ";
    nom_m: "~ा", "~ारौ", "~ारः";
    nom_f: "~्री", "~्र्यौ", "~्र्यः";
    nom_n: "~ृ", "~ृAी", "~ॄAि";
    acc_m: "~ारम्", _, "~ॄन्";
    acc_f: "~्रीम्", _, "~्रीः";
    ins_m: "~्रा", "~ृभ्याम्", "~ृभिः";
    ins_f: "~्र्या", "~्रीभ्याम्", "~्रीभिः";
    ins_n: "~ृAा|~्रा", _, _;
    dat_m: "~्रे", _, "~ृभ्यः";
    dat_f: "~्र्यै", _, "~्रीभ्यः";
    dat_n: "~ृAे|~्रे", _, _;
    abl_m: "~ुः", _, _;
    abl_f: "~्र्याः", _, _;
    abl_n: "~ृAः|~ुः", _, _;
    gen_m: "~ुः", "~्रोः", "~ॄAाम्";
    gen_f: _, "~्र्योः", "~्रीAाम्";
    gen_n: "~ृAः|~ुः", "~ृAोः|~्रोः", _;
    loc_m: "~रि", _, "~ृषु";
    loc_f: "~्र्याम्", _, "~्रीषु";
    loc_n: "~ृAि|~रि", _, _;
    voc_m: "~ः", _, _;
    voc_f: "~्रि", _, _;
    voc_n: "~ृ|~ः", _, _;
    ),
    // सरवजित: 7 words (विद्युत्, दशत्, हरित्, सर्वजित्, च्युत्, श्चुत्). Like रसवत, except in 11 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("सर्वजित्", 7, "~त्", like रसवत as F_AS_M;
    nom_m: "~त्", "~तौ", "~तः";
    nom_f: "~त्", "~तौ", "~तः";
    acc_m: "~तम्", _, _;
    acc_f: "~तम्", _, _;
    gen_f: derived, _, _;
    voc_m: "~त्", _, _;
    voc_f: "~त्", _, _;
    ),
    // असत: 6 words (बृहत्, पृषत्, अर्हत्, असत्, ऋहत्, अजानत्). Like रसवत, except in 1 cell: nom_m sg.
    declension!("असत्", 6, "~त्", like रसवत;
    nom_m: "~न्", _, _;
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
    dat_m: "~जे", _, "~ग्भ्यः";
    dat_f: "~जे", _, "~ग्भ्यः";
    dat_n: "~जे", _, _;
    abl_m: "~जः", _, _;
    abl_f: "~जः", _, _;
    abl_n: "~जः", _, _;
    gen_m: "~जः", "~जोः", "~जाम्";
    gen_f: _, "~जोः", "~जाम्";
    gen_n: "~जः", "~जोः", _;
    loc_m: "~जि", _, "~क्षु";
    loc_f: "~जि", _, "~क्षु";
    loc_n: "~जि", _, _;
    voc_m: "~क्", _, _;
    voc_f: "~क्", _, _;
    voc_n: "~क्", _, _;
    ),
    // पा: 6 words (सा, वर्चोदा, वर्चोधा, पा, रथेष्ठा, विश्वपा). Like धृत, except in 17 cells: nom_m sg, nom_m du, acc_m pl, ins_m sg, ins_n sg, ins_n pl, ….
    declension!("पा", 6, "~ा", like धृत as M_AS_F;
    nom_m: "~ाः", "~ौ", _;
    acc_m: _, _, "~ः";
    ins_m: "~ा", _, _;
    ins_n: "~ेन", _, "~ैः";
    dat_m: "~े", _, _;
    dat_n: _, _, "~ेभ्यः";
    abl_m: "~ः", _, _;
    abl_n: _, _, "~ेभ्यः";
    gen_m: "~ः", "~ोः", _;
    loc_m: "~ि", _, _;
    loc_n: _, _, "~ेषु";
    voc_m: "~ाः", _, _;
    comparative: derived;
    superlative: derived;
    ),
    // महातमन: 6 words (प्रज्ञात्मन्, शर्मन्, महात्मन्, भस्मन्, युग्मन्, अकृतात्मन्). Like ऋतावन, except in 22 cells: nom_n du, acc_m pl, acc_f pl, ins_m sg, ins_f sg, dat_m sg, ….
    declension!("महात्मन्", 6, "~न्", like ऋतावन as F_AS_M;
    nom_n: _, "~Aी", _;
    acc_m: _, _, "~Aः";
    acc_f: _, _, "~Aः";
    ins_m: "~Aा", _, _;
    ins_f: "~Aा", _, _;
    dat_m: "~Aे", _, _;
    dat_f: "~Aे", _, _;
    dat_n: "~Aे", _, _;
    abl_m: "~Aः", _, _;
    abl_f: "~Aः", _, _;
    abl_n: "~Aः", _, _;
    gen_m: "~Aः", "~Aोः", "~Aाम्";
    gen_f: derived, "~Aोः", "~Aाम्";
    gen_n: "~Aः", "~Aोः", _;
    loc_m: "~Aि", _, _;
    loc_f: "~Aि", _, _;
    loc_n: "~Aि", _, _;
    ),
    // सपृश: 5 words (सदृश्, स्पृश्, हृदिस्पृश्, कीदृश्, ईदृश्). Like उशिज, except in 28 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n du, nom_n pl, ….
    declension!("स्पृश्", 5, "~श्", like उशिज;
    nom_m: _, "~शौ", "~शः";
    nom_f: _, "~शौ", "~शः";
    nom_n: _, "~शी", "~ंशि";
    acc_m: "~शम्", _, "~शः";
    acc_f: "~शम्", _, "~शः";
    ins_m: "~शा", _, _;
    ins_f: "~शा", _, _;
    dat_m: "~शे", _, _;
    dat_f: "~शे", _, _;
    dat_n: "~शे", _, _;
    abl_m: "~शः", _, _;
    abl_f: "~शः", _, _;
    abl_n: "~शः", _, _;
    gen_m: "~शः", "~शोः", "~शाम्";
    gen_f: _, "~शोः", "~शाम्";
    gen_n: "~शः", "~शोः", _;
    loc_m: "~शि", _, _;
    loc_f: "~शि", _, _;
    loc_n: "~शि", _, _;
    ),
    // वी: 5 words (श्री, प्री, वी, शी, दुर्धी).
    declension!("वी", 5, "~ी";
    nom_m: "~ीः", "~ियौ", "~ियः";
    nom_f: "~ीः", "~ियौ", "~ियः";
    nom_n: "~ि", "~िAी", "~ीAि";
    acc_m: "~ियम्", _, "~ियः";
    acc_f: "~ियम्", _, "~ियः";
    ins_m: "~िया", "~ीभ्याम्", "~ीभिः";
    ins_f: "~िया", "~ीभ्याम्", "~ीभिः";
    ins_n: "~िAा", "~िभ्याम्", "~िभिः";
    dat_m: "~िये", _, "~ीभ्यः";
    dat_f: "~िये", _, "~ीभ्यः";
    dat_n: "~िAे|~ये", "~िभ्याम्", "~िभ्यः";
    abl_m: "~ियः", _, _;
    abl_f: "~ियः", _, _;
    abl_n: "~िAः|~ेः", "~िभ्याम्", "~िभ्यः";
    gen_m: "~ियः", "~ियोः", "~ियाम्|~ीनाम्";
    gen_f: _, "~ियोः", "~ियाम्|~ीनाम्";
    gen_n: "~िAः|~ेः", "~िAोः|~्योः", "~ीAाम्";
    loc_m: "~ियि", _, "~ीषु";
    loc_f: "~ियि", _, "~ीषु";
    loc_n: "~िAि|~ौ", _, "~िषु";
    voc_m: "~ीः", _, _;
    voc_f: "~ीः", _, _;
    voc_n: "~ि|~े", _, _;
    ),
    // अमनस: 4 words (अमनस्, क्लान्तमनस्, रंहस्, तरस्). Like जयायस, except in 5 cells: nom_m sg, nom_m du, nom_m pl, acc_m sg, voc_m sg.
    declension!("अमनस्", 4, "~स्", like जयायस;
    nom_m: "~ाः", "~सौ", "~सः";
    acc_m: "~सम्", _, _;
    voc_m: "~ः", _, _;
    ),
    // नयञच: 4 words (प्राञ्च्, अवाञ्च्, न्यञ्च्, उदञ्च्). Like उशिज, except in 36 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("न्यञ्च्", 4, "~ञ्च्", like उशिज;
    nom_m: "~ङ्", "~ञ्चौ", "~ञ्चः";
    nom_f: "~ची", "~च्यौ", "~च्यः";
    nom_n: _, "~ची", "~ञ्चि";
    acc_m: "~ञ्चम्", _, "~चः";
    acc_f: "~चीम्", _, "~चीः";
    ins_m: "~चा", _, _;
    ins_f: "~च्या", "~चीभ्याम्", "~चीभिः";
    dat_m: "~चे", _, _;
    dat_f: "~च्यै", _, "~चीभ्यः";
    dat_n: "~चे", _, _;
    abl_m: "~चः", _, _;
    abl_f: "~च्याः", _, _;
    abl_n: "~चः", _, _;
    gen_m: "~चः", "~चोः", "~चाम्";
    gen_f: _, "~च्योः", "~चीनाम्";
    gen_n: "~चः", "~चोः", _;
    loc_m: "~चि", _, _;
    loc_f: "~च्याम्", _, "~चीषु";
    loc_n: "~चि", _, _;
    voc_m: "~ङ्", _, _;
    voc_f: "~चि", _, _;
    ),
    // वामन: 3 words (नव, घन, पक्व). Like धृत, except in 2 cells: comparative, superlative.
    declension!("वामन", 3, "~", like धृत;
    comparative: "~ीयस्";
    superlative: "~िष्ठ";
    ),
    // इतर: 3 words (विश्व, इतर, एकैक). Like धृत, except in 14 cells: nom_m pl, dat_m sg, dat_f sg, dat_n sg, abl_m sg, abl_f sg, ….
    declension!("इतर", 3, "~", like धृत;
    nom_m: _, _, "~े";
    dat_m: "~स्मै", _, _;
    dat_f: "~स्यै", _, _;
    dat_n: "~स्मै", _, _;
    abl_m: "~स्मात्", _, _;
    abl_f: "~स्याः", _, _;
    abl_n: "~स्मात्", _, _;
    gen_m: _, _, "~ेषाम्";
    gen_f: _, _, "~ासाम्";
    loc_m: "~स्मिन्", _, _;
    loc_f: "~स्याम्", _, _;
    loc_n: "~स्मिन्", _, _;
    comparative: derived;
    superlative: derived;
    ),
    // रघु: 3 words (अगुरु, प्रभु, रघु). Like लघु, except in 11 cells: nom_n du, nom_n pl, acc_m pl, ins_m sg, dat_n sg, abl_n sg, ….
    declension!("रघु", 3, "~ु", like लघु;
    nom_n: _, "~ुणी", "~ूणि";
    acc_m: _, _, "~ून्";
    ins_m: "~ुणा", _, _;
    dat_n: "~ुणे|~वे", _, _;
    abl_n: "~ुणः|~ोः", _, _;
    gen_m: _, _, "~ूणाम्";
    gen_f: _, _, "~्वीणाम्";
    gen_n: "~ुणः|~ोः", "~ुणोः|~्वोः", _;
    loc_n: "~ुणि|~ौ", _, _;
    ),
    // दुह: 3 words (स्निह्, दुह्, लिह्).
    declension!("दुह्", 3, "~ह्";
    nom_m: "~ट्", "~हौ", "~हः";
    nom_f: "~ट्", "~हौ", "~हः";
    nom_n: "~ट्", "~ही", "~ंहि";
    acc_m: "~हम्", _, "~हः";
    acc_f: "~हम्", _, "~हः";
    ins_m: "~हा", "~ड्भ्याम्", "~ड्भिः";
    ins_f: "~हा", "~ड्भ्याम्", "~ड्भिः";
    dat_m: "~हे", _, "~ड्भ्यः";
    dat_f: "~हे", _, "~ड्भ्यः";
    dat_n: "~हे", _, _;
    abl_m: "~हः", _, _;
    abl_f: "~हः", _, _;
    abl_n: "~हः", _, _;
    gen_m: "~हः", "~होः", "~हाम्";
    gen_f: _, "~होः", "~हाम्";
    gen_n: "~हः", "~होः", _;
    loc_m: "~हि", _, "~ट्सु";
    loc_f: "~हि", _, "~ट्सु";
    loc_n: "~हि", _, _;
    voc_m: "~ट्", _, _;
    voc_f: "~ट्", _, _;
    voc_n: "~ट्", _, _;
    ),
    // रथी: 3 words (वीतभी, रथी, दूढी). Like वी, except in 21 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, acc_m sg, acc_m pl, ….
    declension!("रथी", 3, "~ी", like वी;
    nom_m: _, "~्यौ", "~्यः";
    nom_f: _, "~्यौ", "~्यः";
    acc_m: "~्यम्", _, "~्यः";
    acc_f: "~्यम्", _, "~्यः";
    ins_m: "~्या", _, _;
    ins_f: "~्या", _, _;
    dat_m: "~्ये", _, _;
    dat_f: "~्ये", _, _;
    abl_m: "~्यः", _, _;
    abl_f: "~्यः", _, _;
    gen_m: "~्यः", "~्योः", "~्याम्|~ीनाम्";
    gen_f: _, "~्योः", "~्याम्|~ीनाम्";
    loc_m: "~्यि", _, _;
    loc_f: "~्यि", _, _;
    ),
    // मुच: 2 words (अर्च्, मुच्). Like नयञच, except in 11 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("मुच्", 2, "~च्", like नयञच as F_AS_M;
    nom_m: "~क्", "~चौ", "~चः";
    nom_f: "~क्", "~चौ", "~चः";
    acc_m: "~चम्", _, _;
    acc_f: "~चम्", _, _;
    gen_f: derived, _, _;
    voc_m: "~क्", _, _;
    voc_f: "~क्", _, _;
    ),
    // परबुध: 1 words (प्रबुध्).
    declension!("प्रबुध्", 1, "~ब~ध्";
    nom_m: "~भ~त्", "~ब~धौ", "~ब~धः";
    nom_f: "~भ~त्", "~ब~धौ", "~ब~धः";
    nom_n: "~भ~त्", "~ब~धी", "~ब~न्धि";
    acc_m: "~ब~धम्", _, "~ब~धः";
    acc_f: "~ब~धम्", _, "~ब~धः";
    ins_m: "~ब~धा", "~भ~द्भ्याम्", "~भ~द्भिः";
    ins_f: "~ब~धा", "~भ~द्भ्याम्", "~भ~द्भिः";
    dat_m: "~ब~धे", _, "~भ~द्भ्यः";
    dat_f: "~ब~धे", _, "~भ~द्भ्यः";
    dat_n: "~ब~धे", _, _;
    abl_m: "~ब~धः", _, _;
    abl_f: "~ब~धः", _, _;
    abl_n: "~ब~धः", _, _;
    gen_m: "~ब~धः", "~ब~धोः", "~ब~धाम्";
    gen_f: _, "~ब~धोः", "~ब~धाम्";
    gen_n: "~ब~धः", "~ब~धोः", _;
    loc_m: "~ब~धि", _, "~भ~त्सु";
    loc_f: "~ब~धि", _, "~भ~त्सु";
    loc_n: "~ब~धि", _, _;
    voc_m: "~भ~त्", _, _;
    voc_f: "~भ~त्", _, _;
    voc_n: "~भ~त्", _, _;
    ),
    // एवंगत: 1 words (एवंगत).
    declension!("एवंगत", 1, "~ं~";
    nom_m: "~ङ्~ः", "~ङ्~ौ", "~ङ्~ाः";
    nom_f: "~ङ्~ा", "~ङ्~े", "~ङ्~ाः";
    nom_n: "~ङ्~म्", "~ङ्~े", "~ङ्~ानि";
    acc_m: "~ङ्~म्", _, "~ङ्~ान्";
    acc_f: "~ङ्~ाम्", _, "~ङ्~ाः";
    ins_m: "~ङ्~ेन", "~ङ्~ाभ्याम्", "~ङ्~ैः";
    ins_f: "~ङ्~या", "~ङ्~ाभ्याम्", "~ङ्~ाभिः";
    dat_m: "~ङ्~ाय", _, "~ङ्~ेभ्यः";
    dat_f: "~ङ्~ायै", _, "~ङ्~ाभ्यः";
    dat_n: "~ङ्~ाय", _, _;
    abl_m: "~ङ्~ात्", _, _;
    abl_f: "~ङ्~ायाः", _, _;
    abl_n: "~ङ्~ात्", _, _;
    gen_m: "~ङ्~स्य", "~ङ्~योः", "~ङ्~ानाम्";
    gen_f: _, "~ङ्~योः", "~ङ्~ानाम्";
    gen_n: "~ङ्~स्य", "~ङ्~योः", _;
    loc_m: "~ङ्~े", _, "~ङ्~ेषु";
    loc_f: "~ङ्~ायाम्", _, "~ङ्~ासु";
    loc_n: "~ङ्~े", _, _;
    voc_m: "~ङ्~", _, _;
    voc_f: "~ङ्~े", _, _;
    voc_n: "~ङ्~", _, _;
    ),
    // दविष: 1 words (द्विष्). Like दुह, except in 28 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n du, nom_n pl, ….
    declension!("द्विष्", 1, "~ष्", like दुह;
    nom_m: _, "~षौ", "~षः";
    nom_f: _, "~षौ", "~षः";
    nom_n: _, "~षी", "~ंषि";
    acc_m: "~षम्", _, "~षः";
    acc_f: "~षम्", _, "~षः";
    ins_m: "~षा", _, _;
    ins_f: "~षा", _, _;
    dat_m: "~षे", _, _;
    dat_f: "~षे", _, _;
    dat_n: "~षे", _, _;
    abl_m: "~षः", _, _;
    abl_f: "~षः", _, _;
    abl_n: "~षः", _, _;
    gen_m: "~षः", "~षोः", "~षाम्";
    gen_f: _, "~षोः", "~षाम्";
    gen_n: "~षः", "~षोः", _;
    loc_m: "~षि", _, _;
    loc_f: "~षि", _, _;
    loc_n: "~षि", _, _;
    ),
    // निरऋति: 1 words (निर्ऋति).
    declension!("निर्ऋति", 1, "~्ऋ~ि";
    nom_m: "~ृ~िः", "~ृ~ी", "~ृ~यः";
    nom_f: "~ृ~िः", "~ृ~ी", "~ृ~यः";
    nom_n: "~ृ~ि", "~ृ~िनी", "~ृ~ीनि";
    acc_m: "~ृ~िम्", _, "~ृ~ीन्";
    acc_f: "~ृ~िम्", _, "~ृ~ीः";
    ins_m: "~ृ~िना", "~ृ~िभ्याम्", "~ृ~िभिः";
    ins_f: "~ृ~्या", "~ृ~िभ्याम्", "~ृ~िभिः";
    dat_m: "~ृ~ये", _, "~ृ~िभ्यः";
    dat_f: "~ृ~ये|~ृ~्यै", _, "~ृ~िभ्यः";
    dat_n: "~ृ~िने|~ृ~ये", _, _;
    abl_m: "~ृ~ेः", _, _;
    abl_f: "~ृ~ेः|~ृ~्याः", _, _;
    abl_n: "~ृ~िनः|~ृ~ेः", _, _;
    gen_m: "~ृ~ेः", "~ृ~्योः", "~ृ~ीनाम्";
    gen_f: _, "~ृ~्योः", "~ृ~ीनाम्";
    gen_n: "~ृ~िनः|~ृ~ेः", "~ृ~िनोः|~ृ~्योः", _;
    loc_m: "~ृ~ौ", _, "~ृ~िषु";
    loc_f: "~ृ~ौ|~ृ~्याम्", _, "~ृ~िषु";
    loc_n: "~ृ~िनि|~ृ~ौ", _, _;
    voc_m: "~ृ~े", _, _;
    voc_f: "~ृ~े", _, _;
    voc_n: "~ृ~ि|~ृ~े", _, _;
    ),
    // नैरऋत: 1 words (नैर्ऋत). Like निरऋति, except in 37 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("नैर्ऋत", 1, "~्ऋ~", like निरऋति as LOC_AS_VOC;
    nom_m: "~ृ~ः", "~ृ~ौ", "~ृ~ाः";
    nom_f: "~ृ~ी", "~ृ~्यौ", "~ृ~्यः";
    nom_n: "~ृ~म्", "~ृ~े", "~ृ~ानि";
    acc_m: "~ृ~म्", _, "~ृ~ान्";
    acc_f: "~ृ~ीम्", _, _;
    ins_m: "~ृ~ेन", "~ृ~ाभ्याम्", "~ृ~ैः";
    ins_f: _, "~ृ~ीभ्याम्", "~ृ~ीभिः";
    dat_m: "~ृ~ाय", _, "~ृ~ेभ्यः";
    dat_f: "~ृ~्यै", _, "~ृ~ीभ्यः";
    dat_n: "~ृ~ाय", _, _;
    abl_m: "~ृ~ात्", _, _;
    abl_f: "~ृ~्याः", _, _;
    abl_n: "~ृ~ात्", _, _;
    gen_m: "~ृ~स्य", "~ृ~योः", "~ृ~ानाम्";
    gen_n: "~ृ~स्य", "~ृ~योः", _;
    loc_m: _, _, "~ृ~ेषु";
    loc_f: "~ृ~्याम्", _, "~ृ~ीषु";
    loc_n: "~ृ~े", _, _;
    voc_m: "~ृ~", _, _;
    voc_f: "~ृ~ि", _, _;
    voc_n: "~ृ~", _, _;
    ),
];
