//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ा`, `लोला`: the stem and `ा`). A stem in two pieces (`~्ऋ~ि`, `निर्ऋति`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem nom_sg: \"~- like वामन\"`: the slot's cells are `वामन`'s table's patterns
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
     $(stem nom_sg: $s0:tt;)?
     $(stem nom_du: $s1:tt;)?
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
    /// Like `वामन`: 247 words.
    वामन,
    /// Like `तारक`: 101 words.
    तारक,
    /// Like `लोला`: 46 words.
    लोला,
    /// Like `काली`: 44 words.
    काली,
    /// Like `विधि`: 35 words.
    विधि,
    /// Like `वीरा`: 21 words.
    वीरा,
    /// Like `वायु`: 20 words.
    वायु,
    /// Like `पुराण`: 20 words.
    पुराण,
    /// Like `रुधिर`: 11 words.
    रुधिर,
    /// Like `अङ्गिरस्`: 9 words.
    अङगिरस,
    /// Like `दिति`: 8 words.
    दिति,
    /// Like `भैरवी`: 8 words.
    भैरवी,
    /// Like `रघु`: 8 words.
    रघु,
    /// Like `अश्विन्`: 7 words.
    अशविन,
    /// Like `अथर्वन्`: 7 words.
    अथरवन,
    /// Like `भगवत्`: 5 words.
    भगवत,
    /// Like `सवितृ`: 3 words.
    सवितृ,
    /// Like `सर्वजित्`: 3 words.
    सरवजित,
    /// Like `असृज्`: 1 words.
    असृज,
    /// Like `वाच्`: 1 words.
    वाच,
    /// Like `उपनिषद्`: 1 words.
    उपनिषद,
    /// Like `विपाश्`: 1 words.
    विपाश,
    /// Like `कुरुष्`: 1 words.
    कुरुष,
    /// Like `निर्ऋति`: 1 words.
    निरऋति,
    /// Like `नैर्ऋत`: 1 words.
    नैरऋत,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 25] = [
        Declension::वामन,
        Declension::तारक,
        Declension::लोला,
        Declension::काली,
        Declension::विधि,
        Declension::वीरा,
        Declension::वायु,
        Declension::पुराण,
        Declension::रुधिर,
        Declension::अङगिरस,
        Declension::दिति,
        Declension::भैरवी,
        Declension::रघु,
        Declension::अशविन,
        Declension::अथरवन,
        Declension::भगवत,
        Declension::सवितृ,
        Declension::सरवजित,
        Declension::असृज,
        Declension::वाच,
        Declension::उपनिषद,
        Declension::विपाश,
        Declension::कुरुष,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 25] = [
    // वामन: 247 words (बुद्ध, सह, स्वस्तिक, कृष्ण, गणेश, मत्स्य).
    declension!("वामन", 247, "~", [gender: Masculine];
    stem nom_sg: "~- like वामन";
    stem nom_du: "~- like वामन";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
    // तारक: 101 words (मित्र, धृतराष्ट्र, इन्द्र, कालचक्र, भर्ग, वृत्र). Like वामन, except in 2 cells: ins sg, gen_ pl.
    declension!("तारक", 101, "~", [gender: Masculine], like वामन;
    ins: "~ेण", _, _;
    gen_: _, _, "~ाणाम्";
    ),
    // लोला: 46 words (कला, गंगा, उमा, माया, अजा, लङ्का).
    declension!("लोला", 46, "~ा", [gender: Feminine];
    nom: "~ा", "~े", "~ाः";
    acc: "~ाम्", _, "~ाः";
    ins: "~या", "~ाभ्याम्", "~ाभिः";
    dat: "~ायै", _, "~ाभ्यः";
    abl: "~ायाः", _, _;
    gen_: "~ायाः", "~योः", "~ानाम्";
    loc: "~ायाम्", _, "~ासु";
    voc: "~े", _, _;
    ),
    // काली: 44 words (हिन्दी, जाह्नवी, सरस्वती, काली, यमी, अमरावती).
    declension!("काली", 44, "~ी", [gender: Feminine];
    nom: "~ी", "~्यौ", "~्यः";
    acc: "~ीम्", _, "~ीः";
    ins: "~्या", "~ीभ्याम्", "~ीभिः";
    dat: "~्यै", _, "~ीभ्यः";
    abl: "~्याः", _, _;
    gen_: "~्याः", "~्योः", "~ीनाम्";
    loc: "~्याम्", _, "~ीषु";
    voc: "~ि", _, _;
    ),
    // विधि: 35 words (अग्नि, संस्कृति, गणपति, पाणिनि, शनि, राजि).
    declension!("विधि", 35, "~ि", [gender: Masculine];
    stem nom_sg: "~- like विधि";
    nom: _, "~ी", _;
    acc: "~िम्", _, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, _, derived;
    ),
    // वीरा: 21 words (हीरा, वीरा, देवभाषा, उषा, इन्दिरा, सुभद्रा). Like लोला, except in 1 cell: gen_ pl.
    declension!("वीरा", 21, "~ा", [gender: Feminine], like लोला;
    gen_: _, _, "~ाणाम्";
    ),
    // वायु: 20 words (मधु, वायु, वास्तु, विष्णु, अभिमन्यु, पाण्डु).
    declension!("वायु", 20, "~ु", [gender: Masculine];
    nom: "~ुः", "~ू", "~वः";
    acc: "~ुम्", _, "~ून्";
    ins: "~ुना", "~ुभ्याम्", "~ुभिः";
    dat: "~वे", _, "~ुभ्यः";
    abl: "~ोः", _, _;
    gen_: "~ोः", "~्वोः", "~ूनाम्";
    loc: "~ौ", _, "~ुषु";
    voc: "~ो", _, _;
    ),
    // पुराण: 20 words (पुराण, क्रोश, गवामयन, कर्मकाण्ड, मधुकाण्ड, रामायण). Like वामन, except in 4 cells: nom sg, nom du, nom pl, acc pl.
    declension!("पुराण", 20, "~", [gender: Neuter], like वामन;
    nom: "~म्", "~े", "~ानि";
    acc: _, _, "~ानि";
    ),
    // रुधिर: 11 words (दीर्घ, कामसूत्र, रुधिर, ब्रह्म, सिंहपुर, अर्थशास्त्र). Like तारक, except in 4 cells: nom sg, nom du, nom pl, acc pl.
    declension!("रुधिर", 11, "~", [gender: Neuter], like तारक;
    nom: "~म्", "~े", "~ाणि";
    acc: _, _, "~ाणि";
    ),
    // अङगिरस: 9 words (उषस्, अङ्गिरस्, आयुस्, रंहस्, वसुश्रवस्, उच्चैःश्रवस्).
    declension!("अङ्गिरस्", 9, "~स्";
    stem nom_du: "~स- like वामन";
    nom: "~ाः", _, "~सः";
    acc: _, derived, "~सः";
    ins: "~सा", "~ोभ्याम्", "~ोभिः";
    dat: "~से", _, "~ोभ्यः";
    abl: "~सः", _, _;
    gen_: "~सः", "~सोः", "~साम्";
    loc: "~सि", _, "~ःसु|~स्सु";
    voc: "~ः", derived, _;
    ),
    // दिति: 8 words (संधि, असि, ऋद्धि, अदिति, दिति, अरमति). Like विधि, except in 6 cells: acc pl, ins sg, dat sg, abl sg, gen_ sg, loc sg.
    declension!("दिति", 8, "~ि", [gender: Feminine], like विधि;
    acc: _, _, "~ीः";
    ins: "~्या", _, _;
    dat: "~ये|~्यै", _, _;
    abl: "~ेः|~्याः", _, _;
    gen_: "~ेः|~्याः", _, _;
    loc: "~ौ|~्याम्", _, _;
    ),
    // भैरवी: 8 words (देवनागरी, भैरवी, गोदावरी, हरी, गान्धारी, शुतुद्री). Like काली, except in 1 cell: gen_ pl.
    declension!("भैरवी", 8, "~ी", [gender: Feminine], like काली;
    gen_: _, _, "~ीणाम्";
    ),
    // रघु: 8 words (रघु, कश्यपमेरु, मेरु, इक्ष्वाकु, भृगु, क्रुमु). Like वायु, except in 2 cells: ins sg, gen_ pl.
    declension!("रघु", 8, "~ु", like वायु;
    ins: "~ुणा", _, _;
    gen_: _, _, "~ूणाम्";
    ),
    // अशविन: 7 words (हस्तिन्, प्रियदर्शिन्, अश्विन्, वज्राङ्गबलिन्, श्मशानवासिन्, मनस्विन्). Like विधि, except in 12 cells: nom sg, nom pl, acc du, acc pl, dat sg, abl sg, ….
    declension!("अश्विन्", 7, "~िन्", [gender: Masculine], like विधि;
    stem nom_du: "~िन- like वामन";
    nom: "~ी", _, "~िनः";
    acc: _, derived, "~िनः";
    dat: "~िने", _, _;
    abl: "~िनः", _, _;
    gen_: "~िनः", "~िनोः", "~िनाम्";
    loc: "~िनि", _, _;
    voc: "~िन्", derived, _;
    ),
    // अथरवन: 7 words (अथर्वन्, ब्रह्मन्, अर्यमन्, अश्वत्थामन्, विश्वकर्मन्, पूषन्).
    declension!("अथर्वन्", 7, "~न्";
    stem nom_du: "~ाण- like वामन";
    nom: "~ा", _, "~ाणः";
    acc: _, derived, "~णः";
    ins: "~णा", "~भ्याम्", "~भिः";
    dat: "~णे", _, "~भ्यः";
    abl: "~णः", _, _;
    gen_: "~णः", "~णोः", "~णाम्";
    loc: "~णि", _, "~सु";
    voc: "~न्", derived, _;
    ),
    // भगवत: 5 words (हिमवत्, भगवत्, अनन्तवत्, हनुमत्, विवस्वत्).
    declension!("भगवत्", 5, "~त्", [gender: Masculine];
    stem nom_du: "~न्त- like वामन";
    nom: "~ान्", _, "~न्तः";
    acc: _, derived, "~तः";
    ins: "~ता", "~द्भ्याम्", "~द्भिः";
    dat: "~ते", _, "~द्भ्यः";
    abl: "~तः", _, _;
    gen_: "~तः", "~तोः", "~ताम्";
    loc: "~ति", _, "~त्सु";
    voc: "~न्", derived, _;
    ),
    // सवितृ: 3 words (सवितृ, त्वष्टृ, विधातृ).
    declension!("सवितृ", 3, "~ृ", [gender: Masculine];
    stem nom_du: "~ार- like वामन";
    nom: "~ा", _, "~ारः";
    acc: _, derived, "~ॄन्";
    ins: "~्रा", "~ृभ्याम्", "~ृभिः";
    dat: "~्रे", _, "~ृभ्यः";
    abl: "~ुः", _, _;
    gen_: "~ुः", "~्रोः", "~ॄणाम्";
    loc: "~रि", _, "~ृषु";
    voc: "~ः", derived, _;
    ),
    // सरवजित: 3 words (सर्वजित्, परीक्षित्, आस्फुजित्). Like भगवत, except in 5 cells: nom sg, nom pl, acc du, voc sg, voc du.
    declension!("सर्वजित्", 3, "~त्", [gender: Masculine], like भगवत;
    stem nom_du: "~त- like वामन";
    nom: "~त्", _, "~तः";
    acc: _, derived, _;
    voc: "~त्", derived, _;
    ),
    // असृज: 1 words (असृज्).
    declension!("असृज्", 1, "~ज्";
    stem nom_du: "~ज- like वामन";
    nom: "~क्", _, "~जः";
    acc: _, derived, "~जः";
    ins: "~जा", "~ग्भ्याम्", "~ग्भिः";
    dat: "~जे", _, "~ग्भ्यः";
    abl: "~जः", _, _;
    gen_: "~जः", "~जोः", "~जाम्";
    loc: "~जि", _, "~क्षु";
    voc: "~क्", derived, _;
    ),
    // वाच: 1 words (वाच्). Like असृज, except in 11 cells: nom pl, acc du, acc pl, ins sg, dat sg, abl sg, ….
    declension!("वाच्", 1, "~च्", like असृज;
    stem nom_du: "~च- like वामन";
    nom: _, _, "~चः";
    acc: _, derived, "~चः";
    ins: "~चा", _, _;
    dat: "~चे", _, _;
    abl: "~चः", _, _;
    gen_: "~चः", "~चोः", "~चाम्";
    loc: "~चि", _, _;
    voc: _, derived, _;
    ),
    // उपनिषद: 1 words (उपनिषद्). Like सरवजित, except in 11 cells: nom pl, acc du, acc pl, ins sg, dat sg, abl sg, ….
    declension!("उपनिषद्", 1, "~द्", like सरवजित;
    stem nom_du: "~द- like वामन";
    nom: _, _, "~दः";
    acc: _, derived, "~दः";
    ins: "~दा", _, _;
    dat: "~दे", _, _;
    abl: "~दः", _, _;
    gen_: "~दः", "~दोः", "~दाम्";
    loc: "~दि", _, _;
    voc: _, derived, _;
    ),
    // विपाश: 1 words (विपाश्).
    declension!("विपाश्", 1, "~श्";
    stem nom_du: "~श- like वामन";
    nom: "~ट्", _, "~शः";
    acc: _, derived, "~शः";
    ins: "~शा", "~ड्भ्याम्", "~ड्भिः";
    dat: "~शे", _, "~ड्भ्यः";
    abl: "~शः", _, _;
    gen_: "~शः", "~शोः", "~शाम्";
    loc: "~शि", _, "~ट्सु";
    voc: "~ट्", derived, _;
    ),
    // कुरुष: 1 words (कुरुष्). Like विपाश, except in 11 cells: nom pl, acc du, acc pl, ins sg, dat sg, abl sg, ….
    declension!("कुरुष्", 1, "~ष्", like विपाश;
    stem nom_du: "~ष- like वामन";
    nom: _, _, "~षः";
    acc: _, derived, "~षः";
    ins: "~षा", _, _;
    dat: "~षे", _, _;
    abl: "~षः", _, _;
    gen_: "~षः", "~षोः", "~षाम्";
    loc: "~षि", _, _;
    voc: _, derived, _;
    ),
    // निरऋति: 1 words (निर्ऋति).
    declension!("निर्ऋति", 1, "~्ऋ~ि";
    stem nom_sg: "~ृ~- like विधि";
    nom: _, "~ृ~ी", _;
    acc: "~ृ~िम्", _, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, _, derived;
    ),
    // नैरऋत: 1 words (नैर्ऋत).
    declension!("नैर्ऋत", 1, "~्ऋ~";
    stem nom_sg: "~ृ~- like वामन";
    stem nom_du: "~ृ~- like वामन";
    acc: _, derived, _;
    dat: _, derived, _;
    abl: _, derived, derived;
    loc: _, derived, _;
    voc: _, derived, derived;
    ),
];
