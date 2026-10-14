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
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `nom` (sg, du, pl)
/// `acc` (sg, du, pl)
/// `ins` (sg, du, pl)
/// `dat` (sg, du, pl)
/// `abl` (sg, du, pl)
/// `gen_` (sg, du, pl)
/// `loc` (sg, du, pl)
/// `voc` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun: $s0:tt;)?
     $(stem noun_nom_du: $s1:tt;)?
     $(stem noun_acc_pl: $s2:tt;)?
     $(nom: $c0:tt, $c1:tt, $c2:tt;)?
     $(acc: $c3:tt, $c4:tt, $c5:tt;)?
     $(ins: $c6:tt, $c7:tt, $c8:tt;)?
     $(dat: $c9:tt, $c10:tt, $c11:tt;)?
     $(abl: $c12:tt, $c13:tt, $c14:tt;)?
     $(gen_: $c15:tt, $c16:tt, $c17:tt;)?
     $(loc: $c18:tt, $c19:tt, $c20:tt;)?
     $(voc: $c21:tt, $c22:tt, $c23:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `योग`: 2,089 words.
    योग,
    /// Like `साधन`: 1,091 words.
    साधन,
    /// Like `कला`: 506 words.
    कला,
    /// Like `गति`: 261 words.
    गति,
    /// Like `नाडी`: 233 words.
    नाडी,
    /// Like `संधि`: 134 words.
    संधि,
    /// Like `केतु`: 128 words.
    केतु,
    /// Like `अप्सस्`: 103 words.
    अपसस,
    /// Like `दातृ`: 56 words.
    दातृ,
    /// Like `मूर्धन्`: 51 words.
    मूरधन,
    /// Like `स्वामिन्`: 49 words.
    सवामिन,
    /// Like `धामन्`: 46 words.
    धामन,
    /// Like `जतु`: 19 words.
    जतु,
    /// Like `मरुत्`: 19 words.
    मरुत,
    /// Like `तनु`: 17 words.
    तनु,
    /// Like `चमू`: 16 words.
    चमू,
    /// Like `सम्पद्`: 15 words.
    समपद,
    /// Like `युज्`: 13 words.
    युज,
    /// Like `ज्योतिस्`: 10 words.
    जयोतिस,
    /// Like `धी`: 10 words.
    धी,
    /// Like `त्वच्`: 9 words.
    तवच,
    /// Like `स्पृध्`: 8 words.
    सपृध,
    /// Like `चन्द्रमस्`: 8 words.
    चनदरमस,
    /// Like `आत्मन्`: 8 words.
    आतमन,
    /// Like `द्रुह्`: 8 words.
    दरुह,
    /// Like `गोपा`: 7 words.
    गोपा,
    /// Like `महत्`: 7 words.
    महत,
    /// Like `सेवि`: 7 words.
    सेवि,
    /// Like `स्वर्`: 7 words.
    सवर,
    /// Like `त्विष्`: 6 words.
    तविष,
    /// Like `उसृ`: 6 words.
    उसृ,
    /// Like `भू`: 6 words.
    भू,
    /// Like `पितृ`: 6 words.
    पितृ,
    /// Like `चक्षुस्`: 6 words.
    चकषुस,
    /// Like `तृ`: 5 words.
    तृ,
    /// Like `अर्हत्`: 5 words.
    अरहत,
    /// Like `राड्राजन्`: 4 words.
    राडराजन,
    /// Like `रिप्`: 4 words.
    रिप,
    /// Like `दिश्`: 4 words.
    दिश,
    /// Like `जनुस्`: 4 words.
    जनुस,
    /// Like `शुभ्`: 4 words.
    शुभ,
    /// Like `जगत्`: 4 words.
    जगत,
    /// Like `ग्लौ`: 3 words.
    गलौ,
    /// Like `मधुलिह्`: 3 words.
    मधुलिह,
    /// Like `लक्ष्मी`: 3 words.
    लकषमी,
    /// Like `विराज्`: 3 words.
    विराज,
    /// Like `उषस्`: 3 words.
    उषस,
    /// Like `आक्रमण`: 1 words.
    आकरमण,
    /// Like `आस्थान`: 1 words.
    आसथान,
    /// Like `कण्ठ्य`: 1 words.
    कणठय,
    /// Like `प्रबन्ध`: 1 words.
    परबनध,
    /// Like `असंख्येय`: 1 words.
    असंखयेय,
    /// Like `तृतीया`: 1 words.
    तृतीया,
    /// Like `प्राचीनावीत`: 1 words.
    पराचीनावीत,
    /// Like `निर्ऋति`: 1 words.
    निरऋति,
    /// Like `नैर्ऋत`: 1 words.
    नैरऋत,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 56] = [
        Declension::योग,
        Declension::साधन,
        Declension::कला,
        Declension::गति,
        Declension::नाडी,
        Declension::संधि,
        Declension::केतु,
        Declension::अपसस,
        Declension::दातृ,
        Declension::मूरधन,
        Declension::सवामिन,
        Declension::धामन,
        Declension::जतु,
        Declension::मरुत,
        Declension::तनु,
        Declension::चमू,
        Declension::समपद,
        Declension::युज,
        Declension::जयोतिस,
        Declension::धी,
        Declension::तवच,
        Declension::सपृध,
        Declension::चनदरमस,
        Declension::आतमन,
        Declension::दरुह,
        Declension::गोपा,
        Declension::महत,
        Declension::सेवि,
        Declension::सवर,
        Declension::तविष,
        Declension::उसृ,
        Declension::भू,
        Declension::पितृ,
        Declension::चकषुस,
        Declension::तृ,
        Declension::अरहत,
        Declension::राडराजन,
        Declension::रिप,
        Declension::दिश,
        Declension::जनुस,
        Declension::शुभ,
        Declension::जगत,
        Declension::गलौ,
        Declension::मधुलिह,
        Declension::लकषमी,
        Declension::विराज,
        Declension::उषस,
        Declension::आकरमण,
        Declension::आसथान,
        Declension::कणठय,
        Declension::परबनध,
        Declension::असंखयेय,
        Declension::तृतीया,
        Declension::पराचीनावीत,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 56] = [
    // योग: 2,089 words (क, ङ, ज, ञ, ट, ण).
    declension!("योग", 2089, "~", [gender: Masculine];
    nom: "~ः", "~ौ", "~ाः";
    acc: "~म्", _, "~ान्";
    ins: "~ेन", "~ाभ्याम्", "~ैः";
    dat: "~ाय", _, "~ेभ्यः";
    abl: "~ात्", _, _;
    gen_: "~स्य", "~योः", "~ानाम्";
    loc: "~े", _, "~ेषु";
    voc: "~", _, _;
    ),
    // साधन: 1,091 words (विश्व, ख, साम्य, तृण, आसन, तीर). Like योग, except in 4 cells: nom sg, nom du, nom pl, acc pl.
    declension!("साधन", 1091, "~", [gender: Neuter], like योग;
    nom: "~म्", "~े", "~ानि";
    acc: _, _, "~ानि";
    ),
    // कला: 506 words (देवता, काला, घृणा, भाषा, जरा, अहिंसा).
    declension!("कला", 506, "~ा", [gender: Feminine];
    nom: "~ा", "~े", "~ाः";
    acc: "~ाम्", _, "~ाः";
    ins: "~या", "~ाभ्याम्", "~ाभिः";
    dat: "~ायै", _, "~ाभ्यः";
    abl: "~ायाः", _, _;
    gen_: "~ायाः", "~योः", "~ानाम्";
    loc: "~ायाम्", _, "~ासु";
    voc: "~े", _, _;
    ),
    // गति: 261 words (मुक्ति, संस्कृति, बुद्धि, नाभि, राजनीति, कृषि).
    declension!("गति", 261, "~ि", [gender: Feminine];
    nom: "~िः", "~ी", "~यः";
    acc: "~िम्", _, "~ीः";
    ins: "~्या", "~िभ्याम्", "~िभिः";
    dat: "~ये|~्यै", _, "~िभ्यः";
    abl: "~ेः|~्याः", _, _;
    gen_: "~ेः|~्याः", "~्योः", "~ीनाम्";
    loc: "~ौ|~्याम्", _, "~िषु";
    voc: "~े", _, _;
    ),
    // नाडी: 233 words (वली, ताली, पत्नी, रोटी, नदी, पृथ्वी).
    declension!("नाडी", 233, "~ी", [gender: Feminine];
    nom: "~ी", "~्यौ", "~्यः";
    acc: "~ीम्", _, "~ीः";
    ins: "~्या", "~ीभ्याम्", "~ीभिः";
    dat: "~्यै", _, "~ीभ्यः";
    abl: "~्याः", _, _;
    gen_: "~्याः", "~्योः", "~ीनाम्";
    loc: "~्याम्", _, "~ीषु";
    voc: "~ि", _, _;
    ),
    // संधि: 134 words (संधि, अग्नि, रयि, गणपति, आदि, समाधि). Like गति, except in 6 cells: acc pl, ins sg, dat sg, abl sg, gen_ sg, loc sg.
    declension!("संधि", 134, "~ि", [gender: Masculine], like गति;
    acc: _, _, "~ीन्";
    ins: "~िना", _, _;
    dat: "~ये", _, _;
    abl: "~ेः", _, _;
    gen_: "~ेः", _, _;
    loc: "~ौ", _, _;
    ),
    // केतु: 128 words (सूनु, पशु, परमाणु, गुरु, अणु, ऋतु).
    declension!("केतु", 128, "~ु", [gender: Masculine];
    nom: "~ुः", "~ू", "~वः";
    acc: "~ुम्", _, "~ून्";
    ins: "~ुना", "~ुभ्याम्", "~ुभिः";
    dat: "~वे", _, "~ुभ्यः";
    abl: "~ोः", _, _;
    gen_: "~ोः", "~्वोः", "~ूनाम्";
    loc: "~ौ", _, "~ुषु";
    voc: "~ो", _, _;
    ),
    // अपसस: 103 words (मनस्, शिरस्, अयस्, आस्, पसस्, नभस्).
    declension!("अप्सस्", 103, "~स्", [gender: Neuter];
    nom: "~ः", "~सी", "~ांसि";
    acc: "~ः", _, "~ांसि";
    ins: "~सा", "~ोभ्याम्", "~ोभिः";
    dat: "~से", _, "~ोभ्यः";
    abl: "~सः", _, _;
    gen_: "~सः", "~सोः", "~साम्";
    loc: "~सि", _, "~ःसु|~स्सु";
    voc: "~ः", _, _;
    ),
    // दातृ: 56 words (दातृ, सवितृ, नप्तृ, होतृ, उद्गातृ, धातृ).
    declension!("दातृ", 56, "~ृ", [gender: Masculine];
    nom: "~ा", "~ारौ", "~ारः";
    acc: "~ारम्", _, "~ॄन्";
    ins: "~्रा", "~ृभ्याम्", "~ृभिः";
    dat: "~्रे", _, "~ृभ्यः";
    abl: "~ुः", _, _;
    gen_: "~ुः", "~्रोः", "~ॄणाम्";
    loc: "~रि", _, "~ृषु";
    voc: "~ः", _, _;
    ),
    // मूरधन: 51 words (अश्मन्, ग्रावन्, श्वन्, क्लोमन्, युवन्, रक्षोहन्).
    declension!("मूर्धन्", 51, "~न्", [gender: Masculine];
    nom: "~ा", "~ानौ", "~ानः";
    acc: "~ानम्", _, "~्नः";
    ins: "~्ना", "~भ्याम्", "~भिः";
    dat: "~्ने", _, "~भ्यः";
    abl: "~्नः", _, _;
    gen_: "~्नः", "~्नोः", "~्नाम्";
    loc: "~्नि|~नि", _, "~सु";
    voc: "~न्", _, _;
    ),
    // सवामिन: 49 words (हस्तिन्, मन्त्रिन्, फणिन्, अन्तर्यामिन्, केशिन्, रुधिरोद्गारिन्). Like संधि, except in 12 cells: nom sg, nom du, nom pl, acc sg, acc pl, dat sg, ….
    declension!("स्वामिन्", 49, "~िन्", [gender: Masculine], like संधि;
    nom: "~ी", "~िनौ", "~िनः";
    acc: "~िनम्", _, "~िनः";
    dat: "~िने", _, _;
    abl: "~िनः", _, _;
    gen_: "~िनः", "~िनोः", "~िनाम्";
    loc: "~िनि", _, _;
    voc: "~िन्", _, _;
    ),
    // धामन: 46 words (चर्मन्, नामन्, उदन्, अकर्मन्, ब्रह्मन्, कर्मन्). Like मूरधन, except in 6 cells: nom sg, nom du, nom pl, acc sg, acc pl, voc sg.
    declension!("धामन्", 46, "~न्", [gender: Neuter], like मूरधन;
    nom: "~", "~्नी|~नी", "~ानि";
    acc: "~", _, "~ानि";
    voc: "~न्|~", _, _;
    ),
    // जतु: 19 words (वस्तु, आयु, मधु, अश्रु, दारु, जानु). Like केतु, except in 11 cells: nom sg, nom du, nom pl, acc sg, acc pl, dat sg, ….
    declension!("जतु", 19, "~ु", like केतु;
    nom: "~ु", "~ुनी", "~ूनि";
    acc: "~ु", _, "~ूनि";
    dat: "~ुने", _, _;
    abl: "~ुनः", _, _;
    gen_: "~ुनः", "~ुनोः", _;
    loc: "~ुनि", _, _;
    voc: "~ु|~ो", _, _;
    ),
    // मरुत: 19 words (मरुत्, नपात्, विद्युत्, दशत्, कृत्, हरित्).
    declension!("मरुत्", 19, "~त्";
    nom: "~त्", "~तौ", "~तः";
    acc: "~तम्", _, "~तः";
    ins: "~ता", "~द्भ्याम्", "~द्भिः";
    dat: "~ते", _, "~द्भ्यः";
    abl: "~तः", _, _;
    gen_: "~तः", "~तोः", "~ताम्";
    loc: "~ति", _, "~त्सु";
    voc: "~त्", _, _;
    ),
    // तनु: 17 words (आलु, हनु, लघु, धेनु, तनु, कु). Like केतु, except in 6 cells: acc pl, ins sg, dat sg, abl sg, gen_ sg, loc sg.
    declension!("तनु", 17, "~ु", like केतु;
    acc: _, _, "~ूः";
    ins: "~्वा", _, _;
    dat: "~वे|~्वै", _, _;
    abl: "~ोः|~्वाः", _, _;
    gen_: "~ोः|~्वाः", _, _;
    loc: "~ौ|~्वाम्", _, _;
    ),
    // चमू: 16 words (वधू, तनू, श्वश्रू, जुहू, निम्बू, जनू).
    declension!("चमू", 16, "~ू";
    nom: "~ूः", "~्वौ", "~्वः";
    acc: "~ूम्", _, "~ूः";
    ins: "~्वा", "~ूभ्याम्", "~ूभिः";
    dat: "~्वै", _, "~ूभ्यः";
    abl: "~्वाः", _, _;
    gen_: "~्वाः", "~्वोः", "~ूनाम्";
    loc: "~्वाम्", _, "~ूषु";
    voc: "~ु", _, _;
    ),
    // समपद: 15 words (हृद्, पद्, विद्, शरद्, मृद्, परिषद्). Like मरुत, except in 11 cells: nom du, nom pl, acc sg, acc pl, ins sg, dat sg, ….
    declension!("सम्पद्", 15, "~द्", like मरुत;
    nom: _, "~दौ", "~दः";
    acc: "~दम्", _, "~दः";
    ins: "~दा", _, _;
    dat: "~दे", _, _;
    abl: "~दः", _, _;
    gen_: "~दः", "~दोः", "~दाम्";
    loc: "~दि", _, _;
    ),
    // युज: 13 words (असृज्, बणिज्, वणिज्, भुज्, भिषज्, उशिज्).
    declension!("युज्", 13, "~ज्";
    nom: "~क्", "~जौ", "~जः";
    acc: "~जम्", _, "~जः";
    ins: "~जा", "~ग्भ्याम्", "~ग्भिः";
    dat: "~जे", _, "~ग्भ्यः";
    abl: "~जः", _, _;
    gen_: "~जः", "~जोः", "~जाम्";
    loc: "~जि", _, "~क्षु";
    voc: "~क्", _, _;
    ),
    // जयोतिस: 10 words (दोस्, क्रविस्, सर्पिस्, बर्हिस्, हविस्, रोचिस्).
    declension!("ज्योतिस्", 10, "~~स्";
    nom: "~~ः", "~~षी", "~ींष~";
    acc: "~~ः", _, "~ींष~";
    ins: "~~षा", "~~र्भ्याम्", "~~र्भिः";
    dat: "~~षे", _, "~~र्भ्यः";
    abl: "~~षः", _, _;
    gen_: "~~षः", "~~षोः", "~~षाम्";
    loc: "~~षि", _, "~~ःषु|~~ष्षु";
    voc: "~~ः", _, _;
    ),
    // धी: 10 words (ह्री, स्त्री, धी, श्री, भी, ली).
    declension!("धी", 10, "~ी";
    nom: "~ीः", "~ियौ", "~ियः";
    acc: "~ियम्", _, "~ियः";
    ins: "~िया", "~ीभ्याम्", "~ीभिः";
    dat: "~िये", _, "~ीभ्यः";
    abl: "~ियः", _, _;
    gen_: "~ियः", "~ियोः", "~ियाम्|~ीनाम्";
    loc: "~ियि", _, "~ीषु";
    voc: "~ीः", _, _;
    ),
    // तवच: 9 words (त्वच्, ऋच्, रुच्, शुच्, मुच्, स्फिच्). Like युज, except in 11 cells: nom du, nom pl, acc sg, acc pl, ins sg, dat sg, ….
    declension!("त्वच्", 9, "~च्", like युज;
    nom: _, "~चौ", "~चः";
    acc: "~चम्", _, "~चः";
    ins: "~चा", _, _;
    dat: "~चे", _, _;
    abl: "~चः", _, _;
    gen_: "~चः", "~चोः", "~चाम्";
    loc: "~चि", _, _;
    ),
    // सपृध: 8 words (प्रबुध्, स्पृध्, क्षुध्, युध्, वीरुध्, श्वाविध्). Like मरुत, except in 11 cells: nom du, nom pl, acc sg, acc pl, ins sg, dat sg, ….
    declension!("स्पृध्", 8, "~ध्", like मरुत;
    nom: _, "~धौ", "~धः";
    acc: "~धम्", _, "~धः";
    ins: "~धा", _, _;
    dat: "~धे", _, _;
    abl: "~धः", _, _;
    gen_: "~धः", "~धोः", "~धाम्";
    loc: "~धि", _, _;
    ),
    // चनदरमस: 8 words (ऊधस्, श्रेयस्, अप्सरस्, पुंस्, चन्द्रमस्, विद्वांस्). Like अपसस, except in 5 cells: nom sg, nom du, nom pl, acc sg, acc pl.
    declension!("चन्द्रमस्", 8, "~स्", like अपसस;
    nom: "~ाः", "~सौ", "~सः";
    acc: "~सम्", _, "~सः";
    ),
    // आतमन: 8 words (अथर्वन्, पर्वन्, वन्, धन्वन्, अध्वन्, भुर्वन्). Like मूरधन, except in 8 cells: acc pl, ins sg, dat sg, abl sg, gen_ sg, gen_ du, ….
    declension!("आत्मन्", 8, "~न्", like मूरधन;
    acc: _, _, "~नः";
    ins: "~ना", _, _;
    dat: "~ने", _, _;
    abl: "~नः", _, _;
    gen_: "~नः", "~नोः", "~नाम्";
    loc: "~नि", _, _;
    ),
    // दरुह: 8 words (स्निह्, द्रुह्, मिह्, गुह्, संदिह्, उपानह्). Like युज, except in 11 cells: nom du, nom pl, acc sg, acc pl, ins sg, dat sg, ….
    declension!("द्रुह्", 8, "~ह्", like युज;
    nom: _, "~हौ", "~हः";
    acc: "~हम्", _, "~हः";
    ins: "~हा", _, _;
    dat: "~हे", _, _;
    abl: "~हः", _, _;
    gen_: "~हः", "~होः", "~हाम्";
    loc: "~हि", _, _;
    ),
    // गोपा: 7 words (जा, पन्था, प्रतिमा, दा, गोपा, रथेष्ठा). Like कला, except in 10 cells: nom sg, nom du, acc pl, ins sg, dat sg, abl sg, ….
    declension!("गोपा", 7, "~ा", [gender: Masculine], like कला;
    nom: "~ाः", "~ौ", _;
    acc: _, _, "~ः";
    ins: "~ा", _, _;
    dat: "~े", _, _;
    abl: "~ः", _, _;
    gen_: "~ः", "~ोः", _;
    loc: "~ि", _, _;
    voc: "~ाः", _, _;
    ),
    // महत: 7 words (हिमवत्, महत्, परस्वत्, श्रीमत्, अर्वत्, इरावत्). Like मरुत, except in 5 cells: nom sg, nom du, nom pl, acc sg, voc sg.
    declension!("महत्", 7, "~त्", [gender: Masculine], like मरुत;
    nom: "~ान्", "~न्तौ", "~न्तः";
    acc: "~न्तम्", _, _;
    voc: "~न्", _, _;
    ),
    // सेवि: 7 words (वारि, अस्थि, अक्षि, इदानि, सेवि, दधि). Like सवामिन, except in 7 cells: nom sg, nom du, nom pl, acc sg, acc pl, gen_ pl, ….
    declension!("सेवि", 7, "~ि", [gender: Neuter], like सवामिन;
    nom: "~ि", "~िनी", "~ीनि";
    acc: "~ि", _, "~ीनि";
    gen_: _, _, "~ीनाम्";
    voc: "~ि|~े", _, _;
    ),
    // सवर: 7 words (अहर्, द्वार्, पुर्, स्वर्, वार्, गिर्).
    declension!("स्वर्", 7, "~र्";
    nom: "~ः", "~री", "~रि";
    acc: "~ः", _, "~रि";
    ins: "~रा", "~र्भ्याम्", "~र्भिः";
    dat: "~रे", _, "~र्भ्यः";
    abl: "~रः", _, _;
    gen_: "~रः", "~रोः", "~राम्";
    loc: "~रि", _, "~र्षु";
    voc: "~ः", _, _;
    ),
    // तविष: 6 words (त्विष्, विष्, प्रेष्, प्रावृष्, द्विष्, रुष्).
    declension!("त्विष्", 6, "~ष्", [gender: Feminine];
    nom: "~ट्", "~षौ", "~षः";
    acc: "~षम्", _, "~षः";
    ins: "~षा", "~ड्भ्याम्", "~ड्भिः";
    dat: "~षे", _, "~ड्भ्यः";
    abl: "~षः", _, _;
    gen_: "~षः", "~षोः", "~षाम्";
    loc: "~षि", _, "~ट्सु";
    voc: "~ट्", _, _;
    ),
    // उसृ: 6 words (मातृ, दुहितृ, यातृ, ननान्दृ, उसृ, विमातृ). Like दातृ, except in 4 cells: nom du, nom pl, acc sg, acc pl.
    declension!("उसृ", 6, "~ृ", [gender: Feminine], like दातृ;
    nom: _, "~रौ", "~रः";
    acc: "~रम्", _, "~ॄः";
    ),
    // भू: 6 words (भ्रू, भू, स्वपू, सू, जू, मू). Like चमू, except in 12 cells: nom du, nom pl, acc sg, acc pl, ins sg, dat sg, ….
    declension!("भू", 6, "~ू", like चमू;
    nom: _, "~ुवौ", "~ुवः";
    acc: "~ुवम्", _, "~ुवः";
    ins: "~ुवा", _, _;
    dat: "~ुवे", _, _;
    abl: "~ुवः", _, _;
    gen_: "~ुवः", "~ुवोः", "~ुवाम्|~ूनाम्";
    loc: "~ुवि", _, _;
    voc: "~ूः", _, _;
    ),
    // पितृ: 6 words (भ्रातृ, पितृ, देवृ, नृ, जामातृ, मातापितृ). Like उसृ, except in 1 cell: acc pl.
    declension!("पितृ", 6, "~ृ", [gender: Masculine], like उसृ;
    acc: _, _, "~ॄन्";
    ),
    // चकषुस: 6 words (यजुस्, आयुस्, वपुस्, तरुस्, चक्षुस्, धनुस्).
    declension!("चक्षुस्", 6, "~ुस्";
    nom: "~ुः", "~ुषी", "~ूंषि";
    acc: "~ुः", _, "~ूंषि";
    ins: "~ुषा", "~ुर्भ्याम्", "~ुर्भिः";
    dat: "~ुषे", _, "~ुर्भ्यः";
    abl: "~ुषः", _, _;
    gen_: "~ुषः", "~ुषोः", "~ुषाम्";
    loc: "~ुषि", _, "~ुःषु|~ुष्षु";
    voc: "~ुः", _, _;
    ),
    // तृ: 5 words (स्वसृ, स्तृ, तृ, पितृष्वसृ, पितुःष्वसृ). Like दातृ, except in 1 cell: acc pl.
    declension!("तृ", 5, "~ृ", [gender: Feminine], like दातृ;
    acc: _, _, "~ॄः";
    ),
    // अरहत: 5 words (दत्, सत्, पृषत्, अर्हत्, सरत्). Like महत, except in 1 cell: nom sg.
    declension!("अर्हत्", 5, "~त्", [gender: Masculine], like महत;
    nom: "~न्", _, _;
    ),
    // राडराजन: 4 words (राजन्, मज्जन्, महाराजन्, राड्राजन्). Like मूरधन, except in 8 cells: acc pl, ins sg, dat sg, abl sg, gen_ sg, gen_ du, ….
    declension!("राड्राजन्", 4, "~न्", [gender: Masculine], like मूरधन;
    acc: _, _, "~्ञः";
    ins: "~्ञा", _, _;
    dat: "~्ञे", _, _;
    abl: "~्ञः", _, _;
    gen_: "~्ञः", "~्ञोः", "~्ञाम्";
    loc: "~्ञि|~नि", _, _;
    ),
    // रिप: 4 words (अप्, क्षप्, कृप्, रिप्).
    declension!("रिप्", 4, "~प्", [gender: Feminine];
    nom: "~प्", "~पौ", "~पः";
    acc: "~पम्", _, "~पः";
    ins: "~पा", "~ब्भ्याम्", "~ब्भिः";
    dat: "~पे", _, "~ब्भ्यः";
    abl: "~पः", _, _;
    gen_: "~पः", "~पोः", "~पाम्";
    loc: "~पि", _, "~प्सु";
    voc: "~प्", _, _;
    ),
    // दिश: 4 words (विश्, स्पश्, दिश्, व्रिश्). Like तविष, except in 11 cells: nom du, nom pl, acc sg, acc pl, ins sg, dat sg, ….
    declension!("दिश्", 4, "~श्", like तविष;
    nom: _, "~शौ", "~शः";
    acc: "~शम्", _, "~शः";
    ins: "~शा", _, _;
    dat: "~शे", _, _;
    abl: "~शः", _, _;
    gen_: "~शः", "~शोः", "~शाम्";
    loc: "~शि", _, _;
    ),
    // जनुस: 4 words (मनुस्, जनुस्, आशिस्, इन्द्रधनुस्). Like तविष, except in 6 cells: nom sg, ins du, ins pl, dat pl, loc pl, voc sg.
    declension!("जनुस्", 4, "~स्", like तविष;
    nom: "~ः", _, _;
    ins: _, "~र्भ्याम्", "~र्भिः";
    dat: _, _, "~र्भ्यः";
    loc: _, _, "~ःषु";
    voc: "~ः", _, _;
    ),
    // शुभ: 4 words (गृभ्, क्षुभ्, शुभ्, अनुष्टुभ्). Like रिप, except in 11 cells: nom du, nom pl, acc sg, acc pl, ins sg, dat sg, ….
    declension!("शुभ्", 4, "~भ्", [gender: Feminine], like रिप;
    nom: _, "~भौ", "~भः";
    acc: "~भम्", _, "~भः";
    ins: "~भा", _, _;
    dat: "~भे", _, _;
    abl: "~भः", _, _;
    gen_: "~भः", "~भोः", "~भाम्";
    loc: "~भि", _, _;
    ),
    // जगत: 4 words (जगत्, यकृत्, बृहत्, असत्). Like मरुत, except in 4 cells: nom du, nom pl, acc sg, acc pl.
    declension!("जगत्", 4, "~त्", [gender: Neuter], like मरुत;
    nom: _, "~ती", "~न्ति";
    acc: "~त्", _, "~न्ति";
    ),
    // गलौ: 3 words (नौ, ज्यौ, ग्लौ).
    declension!("ग्लौ", 3, "~ौ";
    nom: "~ौः", "~ावौ", "~ावः";
    acc: "~ावम्", _, "~ावः";
    ins: "~ावा", "~ौभ्याम्", "~ौभिः";
    dat: "~ावे", _, "~ौभ्यः";
    abl: "~ावः", _, _;
    gen_: "~ावः", "~ावोः", "~ावाम्";
    loc: "~ावि", _, "~ौषु";
    voc: "~ौः", _, _;
    ),
    // मधुलिह: 3 words (मधुलिह्, आरुह्, प्रष्ठवाह्). Like दरुह, except in 6 cells: nom sg, ins du, ins pl, dat pl, loc pl, voc sg.
    declension!("मधुलिह्", 3, "~ह्", like दरुह;
    nom: "~ट्", _, _;
    ins: _, "~ड्भ्याम्", "~ड्भिः";
    dat: _, _, "~ड्भ्यः";
    loc: _, _, "~ट्सु";
    voc: "~ट्", _, _;
    ),
    // लकषमी: 3 words (लक्ष्मी, तन्त्री, तन्द्री). Like नाडी, except in 2 cells: nom sg, gen_ pl.
    declension!("लक्ष्मी", 3, "~ी", [gender: Feminine], like नाडी;
    nom: "~ीः|~ी", _, _;
    gen_: _, _, "~ीणाम्";
    ),
    // विराज: 3 words (विराज्, राज्, सम्राज्). Like युज, except in 6 cells: nom sg, ins du, ins pl, dat pl, loc pl, voc sg.
    declension!("विराज्", 3, "~ज्", like युज;
    nom: "~ट्", _, _;
    ins: _, "~ड्भ्याम्", "~ड्भिः";
    dat: _, _, "~ड्भ्यः";
    loc: _, _, "~ट्सु";
    voc: "~ट्", _, _;
    ),
    // उषस: 3 words (उषस्, भास्, मास्). Like अपसस, except in 5 cells: nom pl, acc pl, ins du, ins pl, dat pl.
    declension!("उषस्", 3, "~स्", like अपसस;
    nom: _, _, "~ंसि";
    acc: _, _, "~ंसि";
    ins: _, "~द्भ्याम्", "~द्भिः";
    dat: _, _, "~द्भ्यः";
    ),
    // आकरमण: 1 words (आक्रमण).
    declension!("आक्रमण", 1, "आ~";
    nom: "अति~म्", "अति~े", "अति~ानि";
    acc: "अति~म्", _, "अति~ानि";
    ins: "अति~ेन", "अति~ाभ्याम्", "अति~ैः";
    dat: "अति~ाय", _, "अति~ेभ्यः";
    abl: "अति~ात्", _, _;
    gen_: "अति~स्य", "अति~योः", "अति~ानाम्";
    loc: "अति~े", _, "अति~ेषु";
    voc: "अति~", _, _;
    ),
    // आसथान: 1 words (आस्थान).
    declension!("आस्थान", 1, "आ~";
    nom: "अ~म्", "अ~े", "अ~ानि";
    acc: "अ~म्", _, "अ~ानि";
    ins: "अ~ेन", "अ~ाभ्याम्", "अ~ैः";
    dat: "अ~ाय", _, "अ~ेभ्यः";
    abl: "अ~ात्", _, _;
    gen_: "अ~स्य", "अ~योः", "अ~ानाम्";
    loc: "अ~े", _, "अ~ेषु";
    voc: "अ~", _, _;
    ),
    // कणठय: 1 words (कण्ठ्य). Like चनदरमस, except in 5 cells: nom sg, nom du, nom pl, acc sg, voc sg.
    declension!("कण्ठ्य", 1, "~", like चनदरमस;
    nom: "~ान्", "~ांसौ", "~ांसः";
    acc: "~ांसम्", _, _;
    voc: "~न्", _, _;
    ),
    // परबनध: 1 words (प्रबन्ध). Like योग, except in 0 cells: .
    declension!("प्रबन्ध", 1, "प्र~", like योग;
    ),
    // असंखयेय: 1 words (असंख्येय).
    declension!("असंख्येय", 1, "~ं~";
    nom: "~ङ्~म्", "~ङ्~े", "~ङ्~ानि";
    acc: "~ङ्~म्", _, "~ङ्~ानि";
    ins: "~ङ्~ेन", "~ङ्~ाभ्याम्", "~ङ्~ैः";
    dat: "~ङ्~ाय", _, "~ङ्~ेभ्यः";
    abl: "~ङ्~ात्", _, _;
    gen_: "~ङ्~स्य", "~ङ्~योः", "~ङ्~ानाम्";
    loc: "~ङ्~े", _, "~ङ्~ेषु";
    voc: "~ङ्~", _, _;
    ),
    // तृतीया: 1 words (तृतीया).
    declension!("तृतीया", 1, "~ृ~ा";
    nom: "~्रि~ा", "~्रि~े", "~्रि~ाः";
    acc: "~्रि~ाम्", _, "~्रि~ाः";
    ins: "~्रि~या", "~्रि~ाभ्याम्", "~्रि~ाभिः";
    dat: "~्रि~ायै", _, "~्रि~ाभ्यः";
    abl: "~्रि~ायाः", _, _;
    gen_: "~्रि~ायाः", "~्रि~योः", "~्रि~ानाम्";
    loc: "~्रि~ायाम्", _, "~्रि~ासु";
    voc: "~्रि~े", _, _;
    ),
    // पराचीनावीत: 1 words (प्राचीनावीत).
    declension!("प्राचीनावीत", 1, "~ी~";
    nom: "~ि~म्", "~ि~े", "~ि~ानि";
    acc: "~ि~म्", _, "~ि~ानि";
    ins: "~ि~ेन", "~ि~ाभ्याम्", "~ि~ैः";
    dat: "~ि~ाय", _, "~ि~ेभ्यः";
    abl: "~ि~ात्", _, _;
    gen_: "~ि~स्य", "~ि~योः", "~ि~ानाम्";
    loc: "~ि~े", _, "~ि~ेषु";
    voc: "~ि~", _, _;
    ),
    // निरऋति: 1 words (निर्ऋति).
    declension!("निर्ऋति", 1, "~्ऋ~ि";
    nom: "~ृ~िः", "~ृ~ी", "~ृ~यः";
    acc: "~ृ~िम्", _, "~ृ~ीः";
    ins: "~ृ~्या", "~ृ~िभ्याम्", "~ृ~िभिः";
    dat: "~ृ~ये|~ृ~्यै", _, "~ृ~िभ्यः";
    abl: "~ृ~ेः|~ृ~्याः", _, _;
    gen_: "~ृ~ेः|~ृ~्याः", "~ृ~्योः", "~ृ~ीनाम्";
    loc: "~ृ~ौ|~ृ~्याम्", _, "~ृ~िषु";
    voc: "~ृ~े", _, _;
    ),
    // नैरऋत: 1 words (नैर्ऋत).
    declension!("नैर्ऋत", 1, "~्ऋ~";
    nom: "~ृ~ः", "~ृ~ौ", "~ृ~ाः";
    acc: "~ृ~म्", _, "~ृ~ान्";
    ins: "~ृ~ेन", "~ृ~ाभ्याम्", "~ृ~ैः";
    dat: "~ृ~ाय", _, "~ृ~ेभ्यः";
    abl: "~ृ~ात्", _, _;
    gen_: "~ृ~स्य", "~ृ~योः", "~ृ~ानाम्";
    loc: "~ृ~े", _, "~ृ~ेषु";
    voc: "~ृ~", _, _;
    ),
];
