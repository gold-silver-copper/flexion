//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ा`, `कला`: the stem and `ा`). A stem in two pieces (`~~स्`, `ज्योतिस्`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem rest: \"~- like योग\"`: the slot's cells are `योग`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
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
     $(stem rest: $s0:tt;)?
     $(stem rest__: $s1:tt;)?
     $(stem rest_: $s2:tt;)?
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
    /// Like `अप्सस्`: 104 words.
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
    /// Like `चन्द्रमस्`: 10 words.
    चनदरमस,
    /// Like `त्वच्`: 9 words.
    तवच,
    /// Like `स्पृध्`: 8 words.
    सपृध,
    /// Like `आत्मन्`: 8 words.
    आतमन,
    /// Like `मिह्`: 8 words.
    मिह,
    /// Like `गोपा`: 7 words.
    गोपा,
    /// Like `इरावत्`: 7 words.
    इरावत,
    /// Like `सेवि`: 7 words.
    सेवि,
    /// Like `वार्`: 7 words.
    वार,
    /// Like `त्विष्`: 6 words.
    तविष,
    /// Like `मातृ`: 6 words.
    मातृ,
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
    /// Like `स्पश्`: 4 words.
    सपश,
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
    pub const ALL: [Declension; 55] = [
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
        Declension::चनदरमस,
        Declension::तवच,
        Declension::सपृध,
        Declension::आतमन,
        Declension::मिह,
        Declension::गोपा,
        Declension::इरावत,
        Declension::सेवि,
        Declension::वार,
        Declension::तविष,
        Declension::मातृ,
        Declension::भू,
        Declension::पितृ,
        Declension::चकषुस,
        Declension::तृ,
        Declension::अरहत,
        Declension::राडराजन,
        Declension::रिप,
        Declension::सपश,
        Declension::जनुस,
        Declension::शुभ,
        Declension::जगत,
        Declension::गलौ,
        Declension::मधुलिह,
        Declension::लकषमी,
        Declension::विराज,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 55] = [
    // योग: 2,089 words (क, ङ, ज, ञ, ट, ण).
    declension!("योग", 2089, "~", [gender: Masculine];
    stem rest: "~- like योग";
    stem rest__: "~- like योग";
    stem rest_: "~- like योग";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // साधन: 1,091 words (विश्व, ख, साम्य, तृण, आसन, तीर). Like योग, except in 7 cells: acc du, dat du, abl du, abl pl, loc du, voc du, ….
    declension!("साधन", 1091, "~", [gender: Neuter], like योग;
    stem rest: "~- like साधन";
    stem rest__: "~- like साधन";
    stem rest_: "~- like साधन";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // कला: 506 words (देवता, काला, घृणा, भाषा, जरा, अहिंसा).
    declension!("कला", 506, "~ा", [gender: Feminine];
    stem rest: "~- like कला";
    stem rest__: "~- like कला";
    stem rest_: "~- like कला";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // गति: 261 words (मुक्ति, संस्कृति, बुद्धि, नाभि, राजनीति, कृषि).
    declension!("गति", 261, "~ि", [gender: Feminine];
    stem rest: "~- like गति";
    stem rest__: "~- like गति";
    stem rest_: "~- like गति";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
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
    // अपसस: 104 words (मनस्, शिरस्, अयस्, आस्, पसस्, नभस्).
    declension!("अप्सस्", 104, "~स्", [gender: Neuter];
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
    stem rest__: "~ार- like दातृ";
    nom: "~ा", _, _;
    acc: _, derived, "~ॄन्";
    ins: "~्रा", "~ृभ्याम्", "~ृभिः";
    dat: "~्रे", _, "~ृभ्यः";
    abl: "~ुः", _, _;
    gen_: "~ुः", "~्रोः", "~ॄणाम्";
    loc: "~रि", _, "~ृषु";
    voc: "~ः", derived, derived;
    ),
    // मूरधन: 51 words (अश्मन्, ग्रावन्, श्वन्, क्लोमन्, युवन्, रक्षोहन्).
    declension!("मूर्धन्", 51, "~न्", [gender: Masculine];
    stem rest__: "~ान- like दातृ";
    nom: "~ा", _, _;
    acc: _, derived, "~्नः";
    ins: "~्ना", "~भ्याम्", "~भिः";
    dat: "~्ने", _, "~भ्यः";
    abl: "~्नः", _, _;
    gen_: "~्नः", "~्नोः", "~्नाम्";
    loc: "~्नि|~नि", _, "~सु";
    voc: "~न्", derived, derived;
    ),
    // सवामिन: 49 words (हस्तिन्, मन्त्रिन्, फणिन्, अन्तर्यामिन्, केशिन्, रुधिरोद्गारिन्). Like संधि, except in 6 cells: nom sg, acc du, loc du, voc sg, voc du, voc pl.
    declension!("स्वामिन्", 49, "~िन्", [gender: Masculine], like संधि;
    stem rest__: "~िन- like दातृ";
    stem rest_: "~िन- like स्वामिन्";
    nom: "~ी", _, _;
    acc: _, derived, _;
    loc: _, derived, _;
    voc: "~िन्", derived, derived;
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
    stem rest: "~- like मरुत्";
    stem rest__: "~त- like दातृ";
    stem rest_: "~त- like स्वामिन्";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
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
    // समपद: 15 words (हृद्, पद्, विद्, शरद्, मृद्, परिषद्). Like मरुत, except in 4 cells: acc du, loc du, voc du, voc pl.
    declension!("सम्पद्", 15, "~द्", like मरुत;
    stem rest__: "~द- like दातृ";
    stem rest_: "~द- like स्वामिन्";
    acc: _, derived, _;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // युज: 13 words (असृज्, बणिज्, वणिज्, भुज्, भिषज्, उशिज्).
    declension!("युज्", 13, "~ज्";
    stem rest: "~- like युज्";
    stem rest__: "~ज- like दातृ";
    stem rest_: "~ज- like स्वामिन्";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // जयोतिस: 10 words (दोस्, क्रविस्, सर्पिस्, बर्हिस्, हविस्, रोचिस्).
    declension!("ज्योतिस्", 10, "~~स्";
    stem rest: "~~- like ज्योतिस्";
    nom: _, "~~षी", "~ींष~";
    acc: "~~ः", _, "~ींष~";
    ins: "~~षा", _, _;
    dat: "~~षे", derived, _;
    abl: "~~षः", derived, derived;
    gen_: "~~षः", "~~षोः", "~~षाम्";
    loc: "~~षि", _, _;
    ),
    // धी: 10 words (ह्री, स्त्री, धी, श्री, भी, ली).
    declension!("धी", 10, "~ी";
    stem rest: "~ी- like धी";
    stem rest__: "~िय- like दातृ";
    acc: _, derived, "~ियः";
    ins: "~िया", _, _;
    dat: "~िये", derived, _;
    abl: "~ियः", derived, derived;
    gen_: "~ियः", "~ियोः", "~ियाम्|~ीनाम्";
    loc: "~ियि", _, _;
    voc: _, derived, derived;
    ),
    // चनदरमस: 10 words (ऊधस्, उषस्, श्रेयस्, मास्, अप्सरस्, पुंस्). Like अपसस, except in 5 cells: nom sg, acc du, loc du, voc du, voc pl.
    declension!("चन्द्रमस्", 10, "~स्", like अपसस;
    stem rest__: "~स- like दातृ";
    stem rest_: "~स- like स्वामिन्";
    nom: "~ाः", _, _;
    acc: _, derived, _;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // तवच: 9 words (त्वच्, ऋच्, रुच्, शुच्, मुच्, स्फिच्). Like युज, except in 4 cells: acc du, loc du, voc du, voc pl.
    declension!("त्वच्", 9, "~च्", like युज;
    stem rest__: "~च- like दातृ";
    stem rest_: "~च- like स्वामिन्";
    acc: _, derived, _;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // सपृध: 8 words (प्रबुध्, स्पृध्, क्षुध्, युध्, वीरुध्, श्वाविध्). Like मरुत, except in 4 cells: acc du, loc du, voc du, voc pl.
    declension!("स्पृध्", 8, "~ध्", like मरुत;
    stem rest__: "~ध- like दातृ";
    stem rest_: "~ध- like स्वामिन्";
    acc: _, derived, _;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // आतमन: 8 words (अथर्वन्, पर्वन्, वन्, धन्वन्, अध्वन्, भुर्वन्). Like मूरधन, except in 1 cell: loc du.
    declension!("आत्मन्", 8, "~न्", like मूरधन;
    stem rest_: "~न- like स्वामिन्";
    loc: _, derived, _;
    ),
    // मिह: 8 words (स्निह्, द्रुह्, मिह्, गुह्, संदिह्, उपानह्). Like युज, except in 4 cells: acc du, loc du, voc du, voc pl.
    declension!("मिह्", 8, "~ह्", like युज;
    stem rest__: "~ह- like दातृ";
    stem rest_: "~ह- like स्वामिन्";
    acc: _, derived, _;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // गोपा: 7 words (जा, पन्था, प्रतिमा, दा, गोपा, रथेष्ठा). Like कला, except in 11 cells: nom du, acc pl, ins sg, dat sg, dat du, abl sg, ….
    declension!("गोपा", 7, "~ा", [gender: Masculine], like कला;
    stem rest: "~ा- like गोपा";
    nom: _, "~ौ", _;
    acc: _, _, "~ः";
    ins: "~ा", _, _;
    dat: "~े", derived, _;
    abl: "~ः", derived, derived;
    gen_: "~ः", "~ोः", _;
    loc: "~ि", _, _;
    ),
    // इरावत: 7 words (हिमवत्, महत्, परस्वत्, श्रीमत्, अर्वत्, इरावत्). Like मरुत, except in 6 cells: acc du, dat du, abl du, abl pl, voc du, voc pl.
    declension!("इरावत्", 7, "~त्", [gender: Masculine], like मरुत;
    stem rest: "~- like महत्";
    stem rest__: "~न्त- like दातृ";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    voc: _, derived, derived;
    ),
    // सेवि: 7 words (वारि, अस्थि, अक्षि, इदानि, सेवि, दधि). Like सवामिन, except in 7 cells: nom sg, nom du, nom pl, acc sg, acc pl, gen_ pl, ….
    declension!("सेवि", 7, "~ि", [gender: Neuter], like सवामिन;
    nom: "~ि", "~िनी", "~ीनि";
    acc: "~ि", _, "~ीनि";
    gen_: _, _, "~ीनाम्";
    voc: "~ि|~े", _, _;
    ),
    // वार: 7 words (अहर्, द्वार्, पुर्, स्वर्, वार्, गिर्).
    declension!("वार्", 7, "~र्";
    stem rest: "~- like स्वर्";
    nom: _, "~री", "~रि";
    acc: "~ः", _, "~रि";
    ins: "~रा", _, _;
    dat: "~रे", derived, _;
    abl: "~रः", derived, derived;
    gen_: "~रः", "~रोः", "~राम्";
    loc: "~रि", _, _;
    ),
    // तविष: 6 words (त्विष्, विष्, प्रेष्, प्रावृष्, द्विष्, रुष्).
    declension!("त्विष्", 6, "~ष्", [gender: Feminine];
    stem rest: "~- like त्विष्";
    stem rest__: "~ष- like दातृ";
    stem rest_: "~ष- like स्वामिन्";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // मातृ: 6 words (मातृ, दुहितृ, यातृ, ननान्दृ, उसृ, विमातृ). Like दातृ, except in 4 cells: acc du, acc pl, voc du, voc pl.
    declension!("मातृ", 6, "~ृ", [gender: Feminine], like दातृ;
    stem rest__: "~र- like दातृ";
    acc: _, derived, "~ॄः";
    voc: _, derived, derived;
    ),
    // भू: 6 words (भ्रू, भू, स्वपू, सू, जू, मू). Like चमू, except in 14 cells: acc du, acc pl, ins sg, dat sg, dat du, abl sg, ….
    declension!("भू", 6, "~ू", like चमू;
    stem rest: "~ू- like धी";
    stem rest__: "~ुव- like दातृ";
    acc: _, derived, "~ुवः";
    ins: "~ुवा", _, _;
    dat: "~ुवे", derived, _;
    abl: "~ुवः", derived, derived;
    gen_: "~ुवः", "~ुवोः", "~ुवाम्|~ूनाम्";
    loc: "~ुवि", _, _;
    voc: _, derived, derived;
    ),
    // पितृ: 6 words (भ्रातृ, पितृ, देवृ, नृ, जामातृ, मातापितृ). Like मातृ, except in 1 cell: acc pl.
    declension!("पितृ", 6, "~ृ", [gender: Masculine], like मातृ;
    acc: _, _, "~ॄन्";
    ),
    // चकषुस: 6 words (यजुस्, आयुस्, वपुस्, तरुस्, चक्षुस्, धनुस्).
    declension!("चक्षुस्", 6, "~ुस्";
    stem rest: "~ु- like ज्योतिस्";
    nom: _, "~ुषी", "~ूंषि";
    acc: "~ुः", _, "~ूंषि";
    ins: "~ुषा", _, _;
    dat: "~ुषे", derived, _;
    abl: "~ुषः", derived, derived;
    gen_: "~ुषः", "~ुषोः", "~ुषाम्";
    loc: "~ुषि", _, _;
    ),
    // तृ: 5 words (स्वसृ, स्तृ, तृ, पितृष्वसृ, पितुःष्वसृ). Like दातृ, except in 1 cell: acc pl.
    declension!("तृ", 5, "~ृ", [gender: Feminine], like दातृ;
    acc: _, _, "~ॄः";
    ),
    // अरहत: 5 words (दत्, सत्, पृषत्, अर्हत्, सरत्). Like इरावत, except in 1 cell: nom sg.
    declension!("अर्हत्", 5, "~त्", [gender: Masculine], like इरावत;
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
    stem rest__: "~प- like दातृ";
    stem rest_: "~प- like स्वामिन्";
    nom: "~प्", _, _;
    acc: _, derived, _;
    ins: _, "~ब्भ्याम्", "~ब्भिः";
    dat: _, _, "~ब्भ्यः";
    loc: _, derived, "~प्सु";
    voc: "~प्", derived, derived;
    ),
    // सपश: 4 words (विश्, स्पश्, दिश्, व्रिश्). Like तविष, except in 4 cells: acc du, loc du, voc du, voc pl.
    declension!("स्पश्", 4, "~श्", like तविष;
    stem rest__: "~श- like दातृ";
    stem rest_: "~श- like स्वामिन्";
    acc: _, derived, _;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // जनुस: 4 words (मनुस्, जनुस्, आशिस्, इन्द्रधनुस्). Like तविष, except in 6 cells: nom sg, ins du, ins pl, dat pl, loc pl, voc sg.
    declension!("जनुस्", 4, "~स्", like तविष;
    nom: "~ः", _, _;
    ins: _, "~र्भ्याम्", "~र्भिः";
    dat: _, _, "~र्भ्यः";
    loc: _, _, "~ःषु";
    voc: "~ः", _, _;
    ),
    // शुभ: 4 words (गृभ्, क्षुभ्, शुभ्, अनुष्टुभ्). Like रिप, except in 4 cells: acc du, loc du, voc du, voc pl.
    declension!("शुभ्", 4, "~भ्", [gender: Feminine], like रिप;
    stem rest__: "~भ- like दातृ";
    stem rest_: "~भ- like स्वामिन्";
    acc: _, derived, _;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // जगत: 4 words (जगत्, यकृत्, बृहत्, असत्). Like मरुत, except in 4 cells: nom du, nom pl, acc sg, acc pl.
    declension!("जगत्", 4, "~त्", [gender: Neuter], like मरुत;
    nom: _, "~ती", "~न्ति";
    acc: "~त्", _, "~न्ति";
    ),
    // गलौ: 3 words (नौ, ज्यौ, ग्लौ).
    declension!("ग्लौ", 3, "~ौ";
    stem rest: "~ौ- like धी";
    stem rest__: "~ाव- like दातृ";
    stem rest_: "~ाव- like स्वामिन्";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // मधुलिह: 3 words (मधुलिह्, आरुह्, प्रष्ठवाह्). Like मिह, except in 3 cells: dat du, abl du, abl pl.
    declension!("मधुलिह्", 3, "~ह्", like मिह;
    stem rest: "~- like त्विष्";
    dat: _, derived, _;
    abl: _, derived, derived;
    ),
    // लकषमी: 3 words (लक्ष्मी, तन्त्री, तन्द्री). Like नाडी, except in 2 cells: nom sg, gen_ pl.
    declension!("लक्ष्मी", 3, "~ी", [gender: Feminine], like नाडी;
    nom: "~ीः|~ी", _, _;
    gen_: _, _, "~ीणाम्";
    ),
    // विराज: 3 words (विराज्, राज्, सम्राज्). Like युज, except in 3 cells: dat du, abl du, abl pl.
    declension!("विराज्", 3, "~ज्", like युज;
    stem rest: "~- like त्विष्";
    dat: _, derived, _;
    abl: _, derived, derived;
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
    // कणठय: 1 words (कण्ठ्य). Like चनदरमस, except in 5 cells: nom sg, acc du, voc sg, voc du, voc pl.
    declension!("कण्ठ्य", 1, "~", like चनदरमस;
    stem rest__: "~ांस- like दातृ";
    nom: "~ान्", _, _;
    acc: _, derived, _;
    voc: "~न्", derived, derived;
    ),
    // परबनध: 1 words (प्रबन्ध). Like योग.
    declension!("प्रबन्ध", 1, "प्र~", like योग;
    ),
    // असंखयेय: 1 words (असंख्येय).
    declension!("असंख्येय", 1, "~ं~";
    stem rest: "~ङ्~- like साधन";
    stem rest__: "~ङ्~- like साधन";
    stem rest_: "~ङ्~- like साधन";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // तृतीया: 1 words (तृतीया).
    declension!("तृतीया", 1, "~ृ~ा";
    stem rest: "~्रि~- like कला";
    stem rest__: "~्रि~- like कला";
    stem rest_: "~्रि~- like कला";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // पराचीनावीत: 1 words (प्राचीनावीत).
    declension!("प्राचीनावीत", 1, "~ी~";
    stem rest: "~ि~- like साधन";
    stem rest__: "~ि~- like साधन";
    stem rest_: "~ि~- like साधन";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // निरऋति: 1 words (निर्ऋति).
    declension!("निर्ऋति", 1, "~्ऋ~ि";
    stem rest: "~ृ~- like गति";
    stem rest__: "~ृ~- like गति";
    stem rest_: "~ृ~- like गति";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // नैरऋत: 1 words (नैर्ऋत).
    declension!("नैर्ऋत", 1, "~्ऋ~";
    stem rest: "~ृ~- like योग";
    stem rest__: "~ृ~- like योग";
    stem rest_: "~ृ~- like योग";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
];
