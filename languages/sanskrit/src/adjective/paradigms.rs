//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ु`, `मनु`: the stem and `ु`). A stem in two pieces (`~~`, `घटक`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A capital (`A`, `B`, `C`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem adjective: \"~- like धृत\"`: the slot's cells are `धृत`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
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
     $(stem adjective: $s0:tt;)?
     $(stem adjective_f: $s1:tt;)?
     $(stem adjective_nom_n_du: $s2:tt;)?
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
            affix: None,
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `धृत`: 1,057 words.
    धृत,
    /// Like `अवर`: 254 words.
    अवर,
    /// Like `मूढ`: 183 words.
    मूढ,
    /// Like `भाविन्`: 61 words.
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
    /// Like `भारिन्`: 16 words.
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
    /// Like `हरि`: 6 words.
    हरि,
    /// Like `महात्मन्`: 6 words.
    महातमन,
    /// Like `स्पृश्`: 5 words.
    सपृश,
    /// Like `वी`: 5 words.
    वी,
    /// Like `अमनस्`: 4 words.
    अमनस,
    /// Like `प्राञ्च्`: 4 words.
    पराञच,
    /// Like `वामन`: 3 words.
    वामन,
    /// Like `एकैक`: 3 words.
    एकैक,
    /// Like `बहुव्रीहि`: 3 words.
    बहुवरीहि,
    /// Like `रघु`: 3 words.
    रघु,
    /// Like `लिह्`: 3 words.
    लिह,
    /// Like `दूढी`: 3 words.
    दूढी,
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
    pub const ALL: [Declension; 41] = [
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
        Declension::तरुतृ,
        Declension::सरवजित,
        Declension::असत,
        Declension::उशिज,
        Declension::पा,
        Declension::हरि,
        Declension::महातमन,
        Declension::सपृश,
        Declension::वी,
        Declension::अमनस,
        Declension::पराञच,
        Declension::वामन,
        Declension::एकैक,
        Declension::बहुवरीहि,
        Declension::रघु,
        Declension::लिह,
        Declension::दूढी,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 41] = [
    // धृत: 1,057 words (ज, द, कमल, बुद्ध, सह, आद).
    declension!("धृत", 1057, "~";
    stem adjective: "~- like धृत";
    stem adjective_f: "~- like धृत";
    nom_n: _, "~े", _;
    acc_m: _, derived, "~ान्";
    acc_f: _, derived, _;
    acc_n: derived, _, derived;
    ins_m: "~ेA", _, _;
    ins_n: _, derived, derived;
    dat_m: "~ाय", derived, _;
    dat_n: "~ाय", derived, derived;
    abl_m: "~ात्", derived, derived;
    abl_n: "~ात्", derived, derived;
    gen_m: "~स्य", "~योः", "~ाAाम्";
    gen_f: derived, _, _;
    gen_n: "~स्य", "~योः", _;
    loc_m: "~े", _, _;
    loc_n: "~े", _, derived;
    voc_m: _, derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, _, derived;
    comparative: "~तर";
    superlative: "~तम";
    ),
    // अवर: 254 words (मूर्ख, मधुर, ताम्र, प्रिय, सूर्य, कार्य). Like धृत, except in 7 cells: nom_n pl, acc_f du, ins_m sg, gen_m pl, gen_f sg, voc_f du, ….
    declension!("अवर", 254, "~", like धृत;
    stem adjective_f: "~- like अवर";
    nom_n: _, _, "~ाणि";
    acc_f: _, derived, _;
    ins_m: "~ेण", _, _;
    gen_m: _, _, "~ाणाम्";
    gen_f: derived, _, _;
    voc_f: _, derived, derived;
    ),
    // मूढ: 183 words (भारत, देव, गणित, विशेषण, लेखन, दैव). Like धृत, except in 5 cells: acc_f du, gen_f sg, voc_f du, voc_f pl, comparative.
    declension!("मूढ", 183, "~", like धृत;
    stem adjective_f: "~- like मूढ";
    acc_f: _, derived, _;
    gen_f: derived, _, _;
    voc_f: _, derived, derived;
    comparative: derived;
    ),
    // भाविन: 61 words (हस्तिन्, मायिन्, केशिन्, एकाकिन्, ज्ञानिन्, घातिन्).
    declension!("भाविन्", 61, "~िन्";
    stem adjective_nom_n_du: "~CA- like भाविन्";
    nom_m: "~ी", "~CAौ", "~CAः";
    nom_f: "~िAी", "~िन्यौ", "~िन्यः";
    nom_n: "~C", _, "~ीAि";
    acc_m: "~CAम्", _, _;
    acc_f: "~िAीम्", _, "~िAीः";
    acc_n: _, derived, _;
    ins_m: _, "~Cभ्याम्", "~Cभिः";
    ins_f: "~िन्या", _, "~CAीभिः";
    ins_n: derived, _, _;
    dat_m: _, _, "~Cभ्यः";
    dat_f: "~िन्यै", "~CAीभ्याम्", _;
    abl_f: "~िन्याः", "~CAीभ्याम्", "~CAीभ्यः";
    gen_f: _, _, "~िAीनाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, "~Cषु";
    loc_f: "~िन्याम्", "~िन्योः", "~िAीषु";
    loc_n: _, derived, _;
    voc_m: "~िन्", _, _;
    voc_f: "~िAि", _, _;
    voc_n: "~ि|~िन्", derived, _;
    ),
    // हार: 60 words (सर्प, कार, क्षौर, गौर, अक्ष, ऋक्ष). Like मूढ, except in 5 cells: nom_n pl, ins_m sg, gen_m pl, gen_f pl, superlative.
    declension!("हार", 60, "~", like मूढ;
    nom_n: _, _, "~ाणि";
    ins_m: "~ेण", _, _;
    gen_m: _, _, "~ाणाम्";
    gen_f: _, _, "~ीणाम्";
    superlative: derived;
    ),
    // मनु: 34 words (वायु, पृदाकुसानु, मनु, स्थाणु, पाण्डु, सेतु).
    declension!("मनु", 34, "~ु";
    nom_m: "~ुः", "~ू", "~वः";
    nom_f: "~ुः", "~ू", "~वः";
    nom_n: "~ु", "~ुAी", "~ूAि";
    acc_m: "~ुम्", _, "~ूA्";
    acc_f: "~ुम्", _, "~ूः";
    ins_m: "~ुAा", "~ुभ्याम्", "~ुभिः";
    ins_f: "~्वा", _, "~ुभिः";
    dat_m: "~वे", _, "~ुभ्यः";
    dat_f: "~वे|~्वै", "~ुभ्याम्", _;
    dat_n: "~ुAे|~वे", _, _;
    abl_m: "~ोः", _, _;
    abl_f: "~ोः|~्वाः", "~ुभ्याम्", "~ुभ्यः";
    abl_n: "~ुAः|~ोः", _, _;
    gen_m: "~ोः", "~्वोः", "~ूAाम्";
    gen_f: _, _, "~ूAाम्";
    gen_n: "~ुAः|~ोः", "~ुAोः|~्वोः", _;
    loc_m: "~ौ", _, "~ुषु";
    loc_f: "~ौ|~्वाम्", "~्वोः", "~ुषु";
    loc_n: "~ुAि|~ौ", _, _;
    voc_m: "~ो", _, _;
    voc_f: "~ो", _, _;
    voc_n: "~ु|~ो", _, _;
    ),
    // शुचि: 32 words (संधि, कवि, अभि, गभस्ति, पुलस्ति, वसति).
    declension!("शुचि", 32, "~ि";
    nom_m: "~िः", "~ी", "~यः";
    nom_f: "~िः", "~ी", "~यः";
    nom_n: "~ि", "~िAी", "~ीAि";
    acc_m: "~िम्", _, "~ीA्";
    acc_f: "~िम्", _, "~ीः";
    ins_m: "~िAा", "~िभ्याम्", "~िभिः";
    ins_f: "~्या", _, "~िभिः";
    dat_m: "~ये", _, "~िभ्यः";
    dat_f: "~ये|~्यै", "~िभ्याम्", _;
    dat_n: "~िAे|~ये", _, _;
    abl_m: "~ेः", _, _;
    abl_f: "~ेः|~्याः", "~िभ्याम्", "~िभ्यः";
    abl_n: "~िAः|~ेः", _, _;
    gen_m: "~ेः", "~्योः", "~ीAाम्";
    gen_f: _, _, "~ीAाम्";
    gen_n: "~िAः|~ेः", "~िAोः|~्योः", _;
    loc_m: "~ौ", _, "~िषु";
    loc_f: "~ौ|~्याम्", "~्योः", "~िषु";
    loc_n: "~िAि|~ौ", _, _;
    voc_m: "~े", _, _;
    voc_f: "~े", _, _;
    voc_n: "~ि|~े", _, _;
    ),
    // रसवत: 31 words (जगत्, अंशुमत्, हिमवत्, महत्, भगवत्, अन्तवत्).
    declension!("रसवत्", 31, "~त्";
    stem adjective_f: "~त- like मूढ";
    stem adjective_nom_n_du: "~त- like रसवत्";
    nom_m: "~ान्", "~न्तौ", "~न्तः";
    nom_n: "~त्", _, "~न्ति";
    acc_m: "~न्तम्", _, _;
    acc_f: _, derived, _;
    acc_n: _, derived, _;
    ins_m: _, "~द्भ्याम्", "~द्भिः";
    ins_n: derived, _, _;
    dat_m: _, _, "~द्भ्यः";
    gen_f: derived, _, _;
    gen_n: _, _, derived;
    loc_m: _, derived, "~त्सु";
    loc_n: _, derived, _;
    voc_m: "~न्", _, _;
    voc_f: _, derived, derived;
    voc_n: "~त्", derived, _;
    ),
    // घटक: 30 words (आवश्यक, अंशक, विदूषक, जनक, नन्दक, खानक).
    declension!("घटक", 30, "~~";
    stem adjective: "~~- like धृत";
    stem adjective_f: "~ि~- like धृत";
    nom_n: _, "~~े", _;
    acc_m: _, derived, "~~ान्";
    acc_f: _, derived, _;
    acc_n: derived, _, derived;
    ins_m: "~~ेA", _, _;
    ins_n: _, derived, derived;
    dat_m: "~~ाय", derived, _;
    dat_n: "~~ाय", derived, derived;
    abl_m: "~~ात्", derived, derived;
    abl_n: "~~ात्", derived, derived;
    gen_m: "~~स्य", "~~योः", "~~ाAाम्";
    gen_f: derived, _, _;
    gen_n: "~~स्य", "~~योः", _;
    loc_m: "~~े", _, _;
    loc_n: "~~े", _, derived;
    voc_m: _, derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, _, derived;
    ),
    // हितकर: 23 words (अरित्र, अग्निहोत्र, चित्र, सिंहदंष्ट्र, दरिद्र, विचित्र).
    declension!("हितकर", 23, "~ि~~र";
    stem adjective_f: "~ि~~र- like अवर";
    nom_m: "~ि~~रः", "~ि~~रौ", "~ि~~राः";
    nom_n: "~ि~~रम्", "~ि~~रे", "~ि~~राणि";
    acc_m: "~ि~~रम्", _, "~ि~~रान्";
    acc_f: _, derived, _;
    ins_m: "~ि~~रेण", "~ि~~राभ्याम्", "~ि~~रैः";
    dat_m: "~ि~~राय", _, "~ि~~रेभ्यः";
    dat_n: "~ि~~राय", _, _;
    abl_m: "~ि~~रात्", _, _;
    abl_n: "~ि~~रात्", _, _;
    gen_m: "~ि~~रस्य", "~ि~~रयोः", "~ि~~राणाम्";
    gen_f: derived, _, _;
    gen_n: "~ि~~रस्य", "~ि~~रयोः", _;
    loc_m: "~ि~~रे", _, "~ि~~रेषु";
    loc_n: "~ि~~रे", _, _;
    voc_m: "~ि~~र", _, _;
    voc_f: _, derived, derived;
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
    ins_m: "~सा", "~ोभ्याम्", "~ोभCः";
    ins_f: "~सा", _, "~ोभिः";
    dat_m: "~से", _, "~ोभ्यः";
    dat_f: "~से", "~ोभ्याम्", _;
    dat_n: "~से", _, _;
    abl_m: "~सः", _, _;
    abl_f: "~सः", "~ोभ्याम्", "~ोभ्यः";
    abl_n: "~सः", _, _;
    gen_m: "~सः", "~सोः", "~साम्";
    gen_f: _, _, "~साम्";
    gen_n: "~सः", "~सोः", _;
    loc_m: "~सC", _, "~ःसु|~स्सु";
    loc_f: "~सC", "~सोः", "~ःसु|~स्सु";
    loc_n: "~सि", _, _;
    voc_m: "~ः", _, _;
    voc_f: "~ः", _, _;
    voc_n: "~ः", _, _;
    comparative: "~ीयस्";
    superlative: "~स्तम";
    ),
    // भारिन: 16 words (मन्त्रिन्, अवत्रापिन्, अहंकारिन्, रङ्गिन्, फिरङ्गिन्, करिन्). Like भाविन, except in 27 cells: nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, nom_n pl, ….
    declension!("भारिन्", 16, "~िन्", like भाविन;
    stem adjective_nom_n_du: "~Cण- like भाविन्";
    nom_m: _, "~Cणौ", "~Cणः";
    nom_f: "~िणी", "~िण्यौ", "~िण्यः";
    nom_n: _, _, "~ीणि";
    acc_m: "~Cणम्", _, _;
    acc_f: "~िणीम्", _, "~िणीः";
    acc_n: _, derived, _;
    ins_f: "~िण्या", _, "~Cणीभिः";
    ins_n: derived, _, _;
    dat_f: "~िण्यै", "~Cणीभ्याम्", _;
    abl_f: "~िण्याः", "~Cणीभ्याम्", "~Cणीभ्यः";
    gen_f: _, _, "~िणीनाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, _;
    loc_f: "~िण्याम्", "~िण्योः", "~िणीषु";
    loc_n: _, derived, _;
    voc_f: "~िणि", _, _;
    voc_n: _, derived, _;
    ),
    // जयायस: 15 words (श्रेयस्, वस्यस्, मलीयस्, ज्यायस्, ओजीयस्, कनीयस्). Like धायस, except in 11 cells: nom_m sg, nom_m du, nom_m pl, acc_m sg, acc_f du, gen_f sg, ….
    declension!("ज्यायस्", 15, "~स्", like धायस;
    stem adjective_f: "~स- like ज्यायस्";
    nom_m: "~ान्", "~ांसौ", "~ांसः";
    acc_m: "~ांसम्", _, _;
    acc_f: _, derived, _;
    gen_f: derived, _, _;
    voc_m: "~न्", _, _;
    voc_f: _, derived, derived;
    comparative: derived;
    superlative: derived;
    ),
    // ऋतावन: 15 words (वृत्रहन्, ऋतावन्, रक्षोहन्, वृषन्, हन्, यावन्).
    declension!("ऋतावन्", 15, "~न्";
    stem adjective_f: "~र- like मूढ";
    nom_m: "~ा", "~ाAौ", "~ाAः";
    nom_n: "~", "~्Aी|~Aी", "~ाAि";
    acc_m: "~ाAम्", _, "~्Aः";
    acc_f: _, derived, _;
    ins_m: "~्Aा", "~भ्याम्", "~भिः";
    dat_m: "~्Aे", _, "~भ्यः";
    dat_n: "~्Aे", _, _;
    abl_m: "~्Aः", _, _;
    abl_n: "~्Aः", _, _;
    gen_m: "~्Aः", "~्Aोः", "~्Aाम्";
    gen_f: derived, _, _;
    gen_n: "~्Aः", "~्Aोः", _;
    loc_m: "~्AC|~Aि", _, "~सु";
    loc_n: "~्AC|~Aि", _, _;
    voc_m: "~न्", _, _;
    voc_f: _, derived, derived;
    voc_n: "~न्|~", _, _;
    ),
    // धृषु: 13 words (दारु, तरु, पुरु, दीर्घायु, धृषु, रिपु). Like मनु, except in 8 cells: nom_n du, acc_m pl, ins_m sg, dat_n sg, abl_n sg, gen_n sg, ….
    declension!("धृषु", 13, "~ु", like मनु;
    nom_n: _, "~ुणी", _;
    acc_m: _, _, "~ून्";
    ins_m: "~ुणा", _, _;
    dat_n: "~ुणे|~वे", _, _;
    abl_n: "~ुणः|~ोः", _, _;
    gen_n: "~ुणः|~ोः", "~ुणोः|~्वोः", _;
    loc_n: "~ुणि|~ौ", _, _;
    ),
    // चिर: 13 words (शिशिर, अजिर, रुधिर, स्थिर, बधिर, मेधिर).
    declension!("चिर", 13, "~िर";
    stem adjective: "~िर- like धृत";
    stem adjective_f: "~िर- like धृत";
    nom_n: _, "~िरे", _;
    acc_m: _, derived, "~िरान्";
    acc_f: _, derived, _;
    acc_n: derived, _, derived;
    ins_m: "~िरेA", _, _;
    ins_n: _, derived, derived;
    dat_m: "~िराय", derived, _;
    dat_n: "~िराय", derived, derived;
    abl_m: "~िरात्", derived, derived;
    abl_n: "~िरात्", derived, derived;
    gen_m: "~िरस्य", "~िरयोः", "~िराAाम्";
    gen_f: derived, _, _;
    gen_n: "~िरस्य", "~िरयोः", _;
    loc_m: "~िरे", _, _;
    loc_n: "~िरे", _, derived;
    voc_m: _, derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, _, derived;
    comparative: "~ेयस्";
    superlative: "~ेष्ठ";
    ),
    // लघु: 11 words (अणु, बहु, लघु, अंहु, आशु, वसु). Like मनु, except in 6 cells: acc_f du, gen_f sg, voc_f du, voc_f pl, comparative, superlative.
    declension!("लघु", 11, "~ु", like मनु;
    stem adjective_f: "~्व- like लघु";
    acc_f: _, derived, _;
    gen_f: derived, _, _;
    voc_f: _, derived, derived;
    comparative: "~ीयस्";
    superlative: "~Cष्ठ";
    ),
    // छिद: 9 words (उपस्थसद्, पुरुषाद्, सद्, विद्, छिद्, भिद्).
    declension!("छिद्", 9, "~द्";
    nom_m: "~त्", "~दौ", "~दः";
    nom_f: "~त्", "~दौ", "~दः";
    nom_n: "~त्", "~दी", "~न्दि";
    acc_m: "~दम्", _, "~दः";
    acc_f: "~दम्", _, "~दः";
    ins_m: "~दा", "~द्भ्याम्", "~द्भिः";
    ins_f: "~दा", _, "~द्भिः";
    dat_m: "~दे", _, "~द्भ्यः";
    dat_f: "~दे", "~द्भ्याम्", _;
    dat_n: "~दे", _, _;
    abl_m: "~दः", _, _;
    abl_f: "~दः", "~द्भ्याम्", "~द्भ्यः";
    abl_n: "~दः", _, _;
    gen_m: "~दः", "~दोः", "~दाम्";
    gen_f: _, _, "~दाम्";
    gen_n: "~दः", "~दोः", _;
    loc_m: "~दC", _, "~त्सु";
    loc_f: "~दC", "~दोः", "~त्सु";
    loc_n: "~दि", _, _;
    voc_m: "~त्", _, _;
    voc_f: "~त्", _, _;
    voc_n: "~त्", _, _;
    comparative: "~त्तर";
    superlative: "~त्तम";
    ),
    // तरुतृ: 8 words (यातृ, नेतृ, तरुतृ, गन्तृ, यन्तृ, पक्तृ).
    declension!("तरुतृ", 8, "~ृ";
    stem adjective_f: "~्र- like लघु";
    nom_m: "~ा", "~ारौ", "~ारः";
    nom_n: "~ृ", "~ृAी", "~ॄAि";
    acc_m: "~ारम्", _, "~ॄन्";
    acc_f: _, derived, _;
    ins_m: "~्रा", "~ृभ्याम्", "~ृभिः";
    ins_n: "~ृAा|~्रा", _, _;
    dat_m: "~्रे", _, "~ृभ्यः";
    dat_n: "~ृAे|~्रे", _, _;
    abl_m: "~ुः", _, _;
    abl_n: "~ृAः|~ुः", _, _;
    gen_m: "~ुः", "~्रोः", "~ॄAाम्";
    gen_f: derived, _, _;
    gen_n: "~ृAः|~ुः", "~ृAोः|~्रोः", _;
    loc_m: "~रि", _, "~ृषु";
    loc_n: "~ृAि|~रि", _, _;
    voc_m: "~ः", _, _;
    voc_f: _, derived, derived;
    voc_n: "~ृ|~ः", _, _;
    ),
    // सरवजित: 7 words (विद्युत्, दशत्, हरित्, सर्वजित्, च्युत्, श्चुत्). Like रसवत, except in 22 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("सर्वजित्", 7, "~त्", like रसवत;
    nom_m: "~त्", "~तौ", "~तः";
    nom_f: "~त्", "~तौ", "~तः";
    acc_m: "~तम्", _, _;
    acc_f: "~तम्", _, "~तः";
    ins_f: "~ता", _, "~द्भिः";
    dat_f: "~ते", "~द्भ्याम्", _;
    abl_f: "~तः", "~द्भ्याम्", "~द्भ्यः";
    gen_f: _, _, "~ताम्";
    loc_f: "~तC", "~तोः", "~त्सु";
    voc_m: "~त्", _, _;
    voc_f: "~त्", _, _;
    ),
    // असत: 6 words (बृहत्, पृषत्, अर्हत्, असत्, ऋहत्, अजानत्). Like रसवत, except in 1 cell: nom_m sg.
    declension!("असत्", 6, "~त्", like रसवत;
    nom_m: "~न्", _, _;
    ),
    // उशिज: 6 words (विराज्, भुज्, उशिज्, त्यज्, रुज्, सयुज्).
    declension!("उशिज्", 6, "~ज्";
    stem adjective_nom_n_du: "~ज- like रसवत्";
    nom_m: "~क्", "~जौ", "~जः";
    nom_f: "~क्", "~जौ", "~जः";
    nom_n: "~क्", _, "~ञ्जि";
    acc_m: "~जम्", _, _;
    acc_f: "~जम्", _, "~जः";
    acc_n: _, derived, _;
    ins_m: _, "~ग्भ्याम्", "~ग्भिः";
    ins_f: "~जा", _, "~ग्भिः";
    ins_n: derived, _, _;
    dat_m: _, _, "~ग्भ्यः";
    dat_f: "~जे", "~ग्भ्याम्", _;
    abl_f: "~जः", "~ग्भ्याम्", "~ग्भ्यः";
    gen_f: _, _, "~जाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, "~क्षु";
    loc_f: "~जC", "~जोः", "~क्षु";
    loc_n: _, derived, _;
    voc_m: "~क्", _, _;
    voc_f: "~क्", _, _;
    voc_n: "~क्", derived, _;
    ),
    // पा: 6 words (सा, वर्चोदा, वर्चोधा, पा, रथेष्ठा, विश्वपा). Like धृत, except in 20 cells: nom_m sg, acc_m sg, acc_m pl, ins_m sg, ins_m pl, ins_n sg, ….
    declension!("पा", 6, "~ा", like धृत;
    nom_m: "~ाः", _, _;
    acc_m: "~ाम्", _, "~ः";
    ins_m: "~ा", _, "~ाभिः";
    ins_n: "~ेन", _, "~ैः";
    dat_m: "~े", _, "~ाभ्यः";
    dat_n: _, _, "~ेभ्यः";
    abl_m: "~ः", _, _;
    abl_n: _, _, "~ेभ्यः";
    gen_m: "~ः", "~ोः", _;
    loc_m: "~C", _, "~ासु";
    loc_n: _, _, "~ेषु";
    voc_m: "~ाः", _, _;
    comparative: derived;
    superlative: derived;
    ),
    // हरि: 6 words (हरि, अरि, करि, वध्रि, भूरि, सहुरि).
    declension!("हरि", 6, "~रि";
    stem adjective_f: "~र- like हरि";
    nom_m: "~रिः", "~री", "~रयः";
    nom_n: "~रि", "~रिणी", "~रीAि";
    acc_m: "~रिम्", _, "~रीन्";
    acc_f: _, derived, _;
    ins_m: "~रिAा", "~रिभ्याम्", "~रिभिः";
    dat_m: "~रये", _, "~रिभ्यः";
    dat_n: "~रिAे", _, _;
    abl_m: "~रेः", _, _;
    abl_n: "~रिणः", _, _;
    gen_m: "~रेः", "~र्योः", "~रीणाम्";
    gen_f: derived, _, _;
    gen_n: "~रिणः", "~रिAोः", _;
    loc_m: "~रौ", _, "~रिषु";
    loc_n: "~रिणि", _, _;
    voc_m: "~रे", _, _;
    voc_f: _, derived, derived;
    voc_n: "~रि", _, _;
    comparative: "~यस्";
    superlative: "~यिष्ठ";
    ),
    // महातमन: 6 words (प्रज्ञात्मन्, शर्मन्, महात्मन्, भस्मन्, युग्मन्, अकृतात्मन्). Like ऋतावन, except in 23 cells: nom_f sg, nom_f du, nom_f pl, acc_f sg, acc_f pl, acc_n du, ….
    declension!("महात्मन्", 6, "~न्", like ऋतावन;
    stem adjective_nom_n_du: "~A- like रसवत्";
    nom_f: "~ा", "~ाAौ", "~ाAः";
    acc_f: "~ाAम्", _, "~Aः";
    acc_n: _, derived, _;
    ins_f: "~Aा", _, "~भिः";
    ins_n: derived, _, _;
    dat_f: "~Aे", "~भ्याम्", _;
    abl_f: "~Aः", "~भ्याम्", "~भ्यः";
    gen_f: _, _, "~Aाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, _;
    loc_f: "~AC", "~Aोः", "~सु";
    loc_n: _, derived, _;
    voc_f: "~न्", _, _;
    voc_n: _, derived, _;
    ),
    // सपृश: 5 words (सदृश्, स्पृश्, हृदिस्पृश्, कीदृश्, ईदृश्). Like उशिज, except in 20 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n pl, acc_m sg, ….
    declension!("स्पृश्", 5, "~श्", like उशिज;
    stem adjective_nom_n_du: "~श- like रसवत्";
    nom_m: _, "~शौ", "~शः";
    nom_f: _, "~शौ", "~शः";
    nom_n: _, _, "~ंशि";
    acc_m: "~शम्", _, _;
    acc_f: "~शम्", _, "~शः";
    acc_n: _, derived, _;
    ins_f: "~शा", _, _;
    ins_n: derived, _, _;
    dat_f: "~शे", _, _;
    abl_f: "~शः", _, _;
    gen_f: _, _, "~शाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, _;
    loc_f: "~शC", "~शोः", _;
    loc_n: _, derived, _;
    voc_n: _, derived, _;
    ),
    // वी: 5 words (श्री, प्री, वी, शी, दुर्धी).
    declension!("वी", 5, "~ी";
    nom_m: "~ीः", "~Cयौ", "~Cयः";
    nom_f: "~ीः", "~Cयौ", "~Cयः";
    nom_n: "~C", "~CAी", "~ीAि";
    acc_m: "~Cयम्", _, "~Cयः";
    acc_f: "~Cयम्", _, "~Cयः";
    ins_m: "~Cया", "~ीभ्याम्", "~ीभिः";
    ins_f: "~Cया", _, "~ीभिः";
    ins_n: "~CAा", "~Cभ्याम्", "~Cभिः";
    dat_m: "~Cये", _, "~ीभ्यः";
    dat_f: "~Cये", "~ीभ्याम्", _;
    dat_n: "~CAे|~ये", "~Cभ्याम्", "~Cभ्यः";
    abl_m: "~Cयः", _, _;
    abl_f: "~Cयः", "~ीभ्याम्", "~ीभ्यः";
    abl_n: "~CAः|~ेः", "~Cभ्याम्", "~Cभ्यः";
    gen_m: "~Cयः", "~Cयोः", "~Cयाम्|~ीनाम्";
    gen_f: _, _, "~Cयाम्|~ीनाम्";
    gen_n: "~CAः|~ेः", "~CAोः|~्योः", "~ीAाम्";
    loc_m: "~Cयि", _, "~ीषु";
    loc_f: "~Cयि", "~Cयोः", "~ीषु";
    loc_n: "~CAि|~ौ", _, "~Cषु";
    voc_m: "~ीः", _, _;
    voc_f: "~ीः", _, _;
    voc_n: "~C|~े", _, _;
    ),
    // अमनस: 4 words (अमनस्, क्लान्तमनस्, रंहस्, तरस्). Like जयायस, except in 5 cells: nom_m sg, nom_m du, nom_m pl, acc_m sg, voc_m sg.
    declension!("अमनस्", 4, "~स्", like जयायस;
    nom_m: "~ाः", "~सौ", "~सः";
    acc_m: "~सम्", _, _;
    voc_m: "~ः", _, _;
    ),
    // पराञच: 4 words (प्राञ्च्, अवाञ्च्, न्यञ्च्, उदञ्च्). Like उशिज, except in 16 cells: nom_m sg, nom_m du, nom_m pl, nom_n pl, acc_m sg, acc_f du, ….
    declension!("प्राञ्च्", 4, "~ञ्च्", like उशिज;
    stem adjective_f: "~च- like न्यञ्च्";
    stem adjective_nom_n_du: "~च- like भाविन्";
    nom_m: "~ङ्", "~ञ्चौ", "~ञ्चः";
    nom_n: _, _, "~ञ्चि";
    acc_m: "~ञ्चम्", _, _;
    acc_f: _, derived, _;
    acc_n: _, derived, _;
    ins_n: derived, _, _;
    gen_f: derived, _, _;
    gen_n: _, _, derived;
    loc_m: _, derived, _;
    loc_n: _, derived, _;
    voc_m: "~ङ्", _, _;
    voc_f: _, derived, derived;
    voc_n: _, derived, _;
    ),
    // वामन: 3 words (नव, घन, पक्व). Like धृत, except in 2 cells: comparative, superlative.
    declension!("वामन", 3, "~", like धृत;
    comparative: "~ीयस्";
    superlative: "~िष्ठ";
    ),
    // एकैक: 3 words (विश्व, इतर, एकैक). Like धृत, except in 14 cells: nom_m pl, dat_m sg, dat_f sg, dat_n sg, abl_m sg, abl_f sg, ….
    declension!("एकैक", 3, "~", like धृत;
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
    // बहुवरीहि: 3 words (ब्राह्मि, बहुव्रीहि, द्रापि). Like शुचि, except in 11 cells: nom_n du, nom_n pl, acc_m pl, ins_m sg, dat_n sg, abl_n sg, ….
    declension!("बहुव्रीहि", 3, "~ि", like शुचि;
    nom_n: _, "~िणी", "~ीणि";
    acc_m: _, _, "~ीन्";
    ins_m: "~िणा", _, _;
    dat_n: "~िणे|~ये", _, _;
    abl_n: "~िणः|~ेः", _, _;
    gen_m: _, _, "~ीणाम्";
    gen_f: _, _, "~ीणाम्";
    gen_n: "~िणः|~ेः", "~िणोः|~्योः", _;
    loc_n: "~िणि|~ौ", _, _;
    ),
    // रघु: 3 words (अगुरु, प्रभु, रघु). Like लघु, except in 9 cells: nom_n du, acc_m pl, ins_m sg, dat_n sg, abl_n sg, gen_f pl, ….
    declension!("रघु", 3, "~ु", like लघु;
    nom_n: _, "~ुणी", _;
    acc_m: _, _, "~ून्";
    ins_m: "~ुणा", _, _;
    dat_n: "~ुणे|~वे", _, _;
    abl_n: "~ुणः|~ोः", _, _;
    gen_f: _, _, "~्वीणाम्";
    gen_n: "~ुणः|~ोः", "~ुणोः|~्वोः", _;
    loc_n: "~ुणि|~ौ", _, _;
    ),
    // लिह: 3 words (स्निह्, दुह्, लिह्).
    declension!("लिह्", 3, "~ह्";
    stem adjective_nom_n_du: "~ह- like रसवत्";
    nom_m: "~ट्", "~हौ", "~हः";
    nom_f: "~ट्", "~हौ", "~हः";
    nom_n: "~ट्", _, "~ंहि";
    acc_m: "~हम्", _, _;
    acc_f: "~हम्", _, "~हः";
    acc_n: _, derived, _;
    ins_m: _, "~ड्भ्याम्", "~ड्भिः";
    ins_f: "~हा", _, "~ड्भिः";
    ins_n: derived, _, _;
    dat_m: _, _, "~ड्भ्यः";
    dat_f: "~हे", "~ड्भ्याम्", _;
    abl_f: "~हः", "~ड्भ्याम्", "~ड्भ्यः";
    gen_f: _, _, "~हाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, "~ट्सु";
    loc_f: "~हC", "~होः", "~ट्सु";
    loc_n: _, derived, _;
    voc_m: "~ट्", _, _;
    voc_f: "~ट्", _, _;
    voc_n: "~ट्", derived, _;
    ),
    // दूढी: 3 words (वीतभी, रथी, दूढी). Like वी, except in 21 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, acc_m sg, acc_m pl, ….
    declension!("दूढी", 3, "~ी", like वी;
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
    gen_f: _, _, "~्याम्|~ीनाम्";
    loc_m: "~्यि", _, _;
    loc_f: "~्यि", "~्योः", _;
    ),
    // मुच: 2 words (अर्च्, मुच्). Like पराञच, except in 28 cells: nom_m sg, nom_m du, nom_m pl, nom_f sg, nom_f du, nom_f pl, ….
    declension!("मुच्", 2, "~च्", like पराञच;
    stem adjective_nom_n_du: "~च- like रसवत्";
    nom_m: "~क्", "~चौ", "~चः";
    nom_f: "~क्", "~चौ", "~चः";
    acc_m: "~चम्", _, _;
    acc_f: "~चम्", _, "~चः";
    acc_n: _, derived, _;
    ins_f: "~चा", _, "~ग्भिः";
    ins_n: derived, _, _;
    dat_f: "~चे", "~ग्भ्याम्", _;
    abl_f: "~चः", "~ग्भ्याम्", "~ग्भ्यः";
    gen_f: _, _, "~चाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, _;
    loc_f: "~चC", "~चोः", "~क्षु";
    loc_n: _, derived, _;
    voc_m: "~क्", _, _;
    voc_f: "~क्", _, _;
    voc_n: _, derived, _;
    ),
    // परबुध: 1 words (प्रबुध्).
    declension!("प्रबुध्", 1, "~ब~ध्";
    stem adjective_nom_n_du: "~ब~ध- like भाविन्";
    nom_m: "~भ~त्", "~ब~धौ", "~ब~धः";
    nom_f: "~भ~त्", "~ब~धौ", "~ब~धः";
    nom_n: "~भ~त्", _, "~ब~न्धि";
    acc_m: "~ब~धम्", _, _;
    acc_f: "~ब~धम्", _, "~ब~धः";
    acc_n: _, derived, _;
    ins_m: _, "~भ~द्भ्याम्", "~भ~द्भिः";
    ins_f: "~ब~धा", _, "~भ~द्भिः";
    ins_n: derived, _, _;
    dat_m: _, _, "~भ~द्भ्यः";
    dat_f: "~ब~धे", "~भ~द्भ्याम्", _;
    abl_f: "~ब~धः", "~भ~द्भ्याम्", "~भ~द्भ्यः";
    gen_f: _, _, "~ब~धाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, "~भ~त्सु";
    loc_f: "~ब~धि", "~ब~धोः", "~भ~त्सु";
    loc_n: _, derived, _;
    voc_m: "~भ~त्", _, _;
    voc_f: "~भ~त्", _, _;
    voc_n: "~भ~त्", derived, _;
    ),
    // एवंगत: 1 words (एवंगत).
    declension!("एवंगत", 1, "~ं~";
    stem adjective: "~ङ्~- like नैर्ऋत";
    stem adjective_f: "~ङ्~- like एवंगत";
    stem adjective_nom_n_du: "~ङ्~- like नैर्ऋत";
    acc_m: _, derived, _;
    acc_f: _, derived, _;
    acc_n: derived, derived, derived;
    ins_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_n: _, derived, derived;
    abl_m: _, derived, derived;
    abl_n: _, derived, derived;
    gen_f: derived, _, _;
    gen_n: _, _, derived;
    loc_m: _, derived, _;
    loc_n: _, derived, derived;
    voc_m: _, derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, derived, derived;
    ),
    // दविष: 1 words (द्विष्). Like लिह, except in 20 cells: nom_m du, nom_m pl, nom_f du, nom_f pl, nom_n pl, acc_m sg, ….
    declension!("द्विष्", 1, "~ष्", like लिह;
    stem adjective_nom_n_du: "~ष- like रसवत्";
    nom_m: _, "~षौ", "~षः";
    nom_f: _, "~षौ", "~षः";
    nom_n: _, _, "~ंषि";
    acc_m: "~षम्", _, _;
    acc_f: "~षम्", _, "~षः";
    acc_n: _, derived, _;
    ins_f: "~षा", _, _;
    ins_n: derived, _, _;
    dat_f: "~षे", _, _;
    abl_f: "~षः", _, _;
    gen_f: _, _, "~षाम्";
    gen_n: _, _, derived;
    loc_m: _, derived, _;
    loc_f: "~षC", "~षोः", _;
    loc_n: _, derived, _;
    voc_n: _, derived, _;
    ),
    // निरऋति: 1 words (निर्ऋति).
    declension!("निर्ऋति", 1, "~्ऋ~ि";
    stem adjective_f: "~ृ~- like निर्ऋति";
    nom_m: "~ृ~िः", "~ृ~ी", "~ृ~यः";
    nom_n: "~ृ~ि", "~ृ~िनी", "~ृ~ीनि";
    acc_m: "~ृ~िम्", _, "~ृ~ीन्";
    acc_f: _, derived, _;
    ins_m: "~ृ~िना", "~ृ~िभ्याम्", "~ृ~िभिः";
    dat_m: "~ृ~ये", _, "~ृ~िभ्यः";
    dat_n: "~ृ~िने|~ृ~ये", _, _;
    abl_m: "~ृ~ेः", _, _;
    abl_n: "~ृ~िनः|~ृ~ेः", _, _;
    gen_m: "~ृ~ेः", "~ृ~्योः", "~ृ~ीनाम्";
    gen_f: derived, _, _;
    gen_n: "~ृ~िनः|~ृ~ेः", "~ृ~िनोः|~ृ~्योः", _;
    loc_m: "~ृ~ौ", _, "~ृ~िषु";
    loc_n: "~ृ~िनि|~ृ~ौ", _, _;
    voc_m: "~ृ~े", _, _;
    voc_f: _, derived, derived;
    voc_n: "~ृ~ि|~ृ~े", _, _;
    ),
    // नैरऋत: 1 words (नैर्ऋत).
    declension!("नैर्ऋत", 1, "~्ऋ~";
    stem adjective: "~ृ~- like नैर्ऋत";
    stem adjective_f: "~ृ~- like न्यञ्च्";
    stem adjective_nom_n_du: "~ृ~- like नैर्ऋत";
    acc_m: _, derived, _;
    acc_f: _, derived, _;
    acc_n: derived, derived, derived;
    ins_n: derived, derived, derived;
    dat_m: _, derived, _;
    dat_n: _, derived, derived;
    abl_m: _, derived, derived;
    abl_n: _, derived, derived;
    gen_f: derived, _, _;
    gen_n: _, _, derived;
    loc_m: _, derived, _;
    loc_n: _, derived, derived;
    voc_m: _, derived, derived;
    voc_f: _, derived, derived;
    voc_n: _, derived, derived;
    ),
];
