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
/// `nom` (sg, du, pl)
/// `acc` (sg, du, pl)
/// `ins` (sg, du, pl)
/// `dat` (sg, du, pl)
/// `abl` (sg, du, pl)
/// `gen_` (sg, du, pl)
/// `loc` (sg, du, pl)
/// `voc` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `योग`: 2,120 words.
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
    /// Like `मरुत्`: 19 words.
    मरुत,
    /// Like `जतु`: 19 words.
    जतु,
    /// Like `तनु`: 17 words.
    तनु,
    /// Like `चमू`: 16 words.
    चमू,
    /// Like `सम्पद्`: 15 words.
    समपद,
    /// Like `ज्योतिस्`: 14 words.
    जयोतिस,
    /// Like `युज्`: 13 words.
    युज,
    /// Like `धी`: 10 words.
    धी,
    /// Like `पुंस्`: 10 words.
    पुंस,
    /// Like `त्वच्`: 9 words.
    तवच,
    /// Like `स्पृध्`: 8 words.
    सपृध,
    /// Like `आत्मन्`: 8 words.
    आतमन,
    /// Like `गोपा`: 7 words.
    गोपा,
    /// Like `वारि`: 7 words.
    वारि,
    /// Like `त्विष्`: 6 words.
    तविष,
    /// Like `उसृ`: 6 words.
    उसृ,
    /// Like `भू`: 6 words.
    भू,
    /// Like `पितृ`: 6 words.
    पितृ,
    /// Like `अर्हत्`: 6 words.
    अरहत,
    /// Like `चक्षुस्`: 6 words.
    चकषुस,
    /// Like `महत्`: 6 words.
    महत,
    /// Like `तृ`: 5 words.
    तृ,
    /// Like `जगत्`: 4 words.
    जगत,
    /// Like `राड्राजन्`: 4 words.
    राडराजन,
    /// Like `रिप्`: 4 words.
    रिप,
    /// Like `शुभ्`: 4 words.
    शुभ,
    /// Like `ग्लौ`: 3 words.
    गलौ,
    /// Like `लक्ष्मी`: 3 words.
    लकषमी,
    /// Like `विराज्`: 3 words.
    विराज,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 41] = [
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
        Declension::मरुत,
        Declension::जतु,
        Declension::तनु,
        Declension::चमू,
        Declension::समपद,
        Declension::जयोतिस,
        Declension::युज,
        Declension::धी,
        Declension::पुंस,
        Declension::तवच,
        Declension::सपृध,
        Declension::आतमन,
        Declension::गोपा,
        Declension::वारि,
        Declension::तविष,
        Declension::उसृ,
        Declension::भू,
        Declension::पितृ,
        Declension::अरहत,
        Declension::चकषुस,
        Declension::महत,
        Declension::तृ,
        Declension::जगत,
        Declension::राडराजन,
        Declension::रिप,
        Declension::शुभ,
        Declension::गलौ,
        Declension::लकषमी,
        Declension::विराज,
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
    // योग: 2,120 words (क, ङ, ज, ञ, ट, ण).
    declension!("योग", 2120, "~", [gender: Masculine];
    nom: "~ः", _, "~ाः";
    acc: "~म्", "~ौ", "~ान्";
    ins: "~ेन", "~ाभ्याम्", "~ैः";
    dat: "~ाय", _, "~ेभ्यः";
    abl: "~ात्", _, _;
    gen_: "~स्य", "~योः", "~ानाम्";
    loc: "~े", _, "~ेषु";
    voc: "~", _, _;
    ),
    // साधन: 1,091 words (विश्व, ख, साम्य, तृण, आसन, तीर). Like योग, except in 4 cells: nom sg, nom pl, acc du, acc pl.
    declension!("साधन", 1091, "~", [gender: Neuter], like योग;
    nom: "~म्", _, "~ानि";
    acc: _, "~े", "~ानि";
    ),
    // कला: 506 words (देवता, काला, घृणा, भाषा, जरा, अहिंसा).
    declension!("कला", 506, "~ा", [gender: Feminine];
    nom: "~ा", _, "~ाः";
    acc: "~ाम्", "~े", "~ाः";
    ins: "~या", "~ाभ्याम्", "~ाभिः";
    dat: "~ायै", _, "~ाभ्यः";
    abl: "~ायाः", _, _;
    gen_: "~ायाः", "~योः", "~ानाम्";
    loc: "~ायाम्", _, "~ासु";
    voc: "~े", _, _;
    ),
    // गति: 261 words (मुक्ति, संस्कृति, बुद्धि, नाभि, राजनीति, कृषि).
    declension!("गति", 261, "~ि", [gender: Feminine];
    nom: "~िः", _, "~यः";
    acc: "~िम्", "~ी", "~ीः";
    ins: "~्या", "~िभ्याम्", "~िभिः";
    dat: "~ये|~्यै", _, "~िभ्यः";
    abl: "~ेः|~्याः", _, _;
    gen_: "~ेः|~्याः", "~्योः", "~ीनाम्";
    loc: "~ौ|~्याम्", _, "~िषु";
    voc: "~े", _, _;
    ),
    // नाडी: 233 words (वली, ताली, पत्नी, रोटी, नदी, पृथ्वी).
    declension!("नाडी", 233, "~ी", [gender: Feminine];
    nom: "~ी", _, "~्यः";
    acc: "~ीम्", "~्यौ", "~ीः";
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
    nom: "~ुः", _, "~वः";
    acc: "~ुम्", "~ू", "~ून्";
    ins: "~ुना", "~ुभ्याम्", "~ुभिः";
    dat: "~वे", _, "~ुभ्यः";
    abl: "~ोः", _, _;
    gen_: "~ोः", "~्वोः", "~ूनाम्";
    loc: "~ौ", _, "~ुषु";
    voc: "~ो", _, _;
    ),
    // अपसस: 104 words (मनस्, शिरस्, अयस्, आस्, पसस्, नभस्).
    declension!("अप्सस्", 104, "~स्", [gender: Neuter];
    nom: "~ः", _, "~ांसि";
    acc: "~ः", "~सी", "~ांसि";
    ins: "~सा", "~ोभ्याम्", "~ोभिः";
    dat: "~से", _, "~ोभ्यः";
    abl: "~सः", _, _;
    gen_: "~सः", "~सोः", "~साम्";
    loc: "~सि", _, "~ःसु|~स्सु";
    voc: "~ः", _, _;
    ),
    // दातृ: 56 words (दातृ, सवितृ, नप्तृ, होतृ, उद्गातृ, धातृ).
    declension!("दातृ", 56, "~ृ", [gender: Masculine];
    nom: "~ा", _, "~ारः";
    acc: "~ारम्", "~ारौ", "~ॄन्";
    ins: "~्रा", "~ृभ्याम्", "~ृभिः";
    dat: "~्रे", _, "~ृभ्यः";
    abl: "~ुः", _, _;
    gen_: "~ुः", "~्रोः", "~ॄणाम्";
    loc: "~रि", _, "~ृषु";
    voc: "~ः", _, _;
    ),
    // मूरधन: 51 words (अश्मन्, ग्रावन्, श्वन्, क्लोमन्, युवन्, रक्षोहन्).
    declension!("मूर्धन्", 51, "~न्", [gender: Masculine];
    nom: "~ा", _, "~ानः";
    acc: "~ानम्", "~ानौ", "~्नः";
    ins: "~्ना", "~भ्याम्", "~भिः";
    dat: "~्ने", _, "~भ्यः";
    abl: "~्नः", _, _;
    gen_: "~्नः", "~्नोः", "~्नाम्";
    loc: "~्नि|~नि", _, "~सु";
    voc: "~न्", _, _;
    ),
    // सवामिन: 49 words (हस्तिन्, मन्त्रिन्, फणिन्, अन्तर्यामिन्, केशिन्, रुधिरोद्गारिन्). Like संधि, except in 12 cells: nom sg, nom pl, acc sg, acc du, acc pl, dat sg, ….
    declension!("स्वामिन्", 49, "~िन्", [gender: Masculine], like संधि;
    nom: "~ी", _, "~िनः";
    acc: "~िनम्", "~िनौ", "~िनः";
    dat: "~िने", _, _;
    abl: "~िनः", _, _;
    gen_: "~िनः", "~िनोः", "~िनाम्";
    loc: "~िनि", _, _;
    voc: "~िन्", _, _;
    ),
    // धामन: 46 words (चर्मन्, नामन्, उदन्, अकर्मन्, ब्रह्मन्, कर्मन्). Like मूरधन, except in 7 cells: nom sg, nom du, nom pl, acc sg, acc du, acc pl, ….
    declension!("धामन्", 46, "~न्", [gender: Neuter], like मूरधन;
    nom: "~", "~्नी|~नी", "~ानि";
    acc: "~", "~्नी|~नी", "~ानि";
    voc: "~न्|~", _, _;
    ),
    // मरुत: 19 words (मरुत्, नपात्, विद्युत्, दशत्, कृत्, हरित्).
    declension!("मरुत्", 19, "~त्";
    nom: "~त्", _, "~तः";
    acc: "~तम्", "~तौ", "~तः";
    ins: "~ता", "~द्भ्याम्", "~द्भिः";
    dat: "~ते", _, "~द्भ्यः";
    abl: "~तः", _, _;
    gen_: "~तः", "~तोः", "~ताम्";
    loc: "~ति", _, "~त्सु";
    voc: "~त्", _, _;
    ),
    // जतु: 19 words (वस्तु, आयु, मधु, अश्रु, दारु, जानु). Like केतु, except in 11 cells: nom sg, nom pl, acc sg, acc du, acc pl, dat sg, ….
    declension!("जतु", 19, "~ु", like केतु;
    nom: "~ु", _, "~ूनि";
    acc: "~ु", "~ुनी", "~ूनि";
    dat: "~ुने", _, _;
    abl: "~ुनः", _, _;
    gen_: "~ुनः", "~ुनोः", _;
    loc: "~ुनि", _, _;
    voc: "~ु|~ो", _, _;
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
    nom: "~ूः", _, "~्वः";
    acc: "~ूम्", "~्वौ", "~ूः";
    ins: "~्वा", "~ूभ्याम्", "~ूभिः";
    dat: "~्वै", _, "~ूभ्यः";
    abl: "~्वाः", _, _;
    gen_: "~्वाः", "~्वोः", "~ूनाम्";
    loc: "~्वाम्", _, "~ूषु";
    voc: "~ु", _, _;
    ),
    // समपद: 15 words (हृद्, पद्, विद्, शरद्, मृद्, परिषद्). Like मरुत, except in 11 cells: nom pl, acc sg, acc du, acc pl, ins sg, dat sg, ….
    declension!("सम्पद्", 15, "~द्", like मरुत;
    nom: _, _, "~दः";
    acc: "~दम्", "~दौ", "~दः";
    ins: "~दा", _, _;
    dat: "~दे", _, _;
    abl: "~दः", _, _;
    gen_: "~दः", "~दोः", "~दाम्";
    loc: "~दि", _, _;
    ),
    // जयोतिस: 14 words (दोस्, क्रविस्, सर्पिस्, बर्हिस्, हविस्, रोचिस्).
    declension!("ज्योतिस्", 14, "~~स्";
    nom: "~~ः", _, "~ींष~";
    acc: "~~ः", "~~षी", "~ींष~";
    ins: "~~षा", "~~र्भ्याम्", "~~र्भिः";
    dat: "~~षे", _, "~~र्भ्यः";
    abl: "~~षः", _, _;
    gen_: "~~षः", "~~षोः", "~~षाम्";
    loc: "~~षि", _, "~~ःषु|~~ष्षु";
    voc: "~~ः", _, _;
    ),
    // युज: 13 words (असृज्, बणिज्, वणिज्, भुज्, भिषज्, उशिज्).
    declension!("युज्", 13, "~ज्";
    nom: "~क्", _, "~जः";
    acc: "~जम्", "~जौ", "~जः";
    ins: "~जा", "~ग्भ्याम्", "~ग्भिः";
    dat: "~जे", _, "~ग्भ्यः";
    abl: "~जः", _, _;
    gen_: "~जः", "~जोः", "~जाम्";
    loc: "~जि", _, "~क्षु";
    voc: "~क्", _, _;
    ),
    // धी: 10 words (ह्री, स्त्री, धी, श्री, भी, ली).
    declension!("धी", 10, "~ी";
    nom: "~ीः", _, "~ियः";
    acc: "~ियम्", "~ियौ", "~ियः";
    ins: "~िया", "~ीभ्याम्", "~ीभिः";
    dat: "~िये", _, "~ीभ्यः";
    abl: "~ियः", _, _;
    gen_: "~ियः", "~ियोः", "~ियाम्|~ीनाम्";
    loc: "~ियि", _, "~ीषु";
    voc: "~ीः", _, _;
    ),
    // पुंस: 10 words (ऊधस्, उषस्, श्रेयस्, मास्, अप्सरस्, पुंस्). Like अपसस, except in 5 cells: nom sg, nom pl, acc sg, acc du, acc pl.
    declension!("पुंस्", 10, "~स्", like अपसस;
    nom: "~ाः", _, "~सः";
    acc: "~सम्", "~सौ", "~सः";
    ),
    // तवच: 9 words (त्वच्, ऋच्, रुच्, शुच्, मुच्, स्फिच्). Like युज, except in 11 cells: nom pl, acc sg, acc du, acc pl, ins sg, dat sg, ….
    declension!("त्वच्", 9, "~च्", like युज;
    nom: _, _, "~चः";
    acc: "~चम्", "~चौ", "~चः";
    ins: "~चा", _, _;
    dat: "~चे", _, _;
    abl: "~चः", _, _;
    gen_: "~चः", "~चोः", "~चाम्";
    loc: "~चि", _, _;
    ),
    // सपृध: 8 words (प्रबुध्, स्पृध्, क्षुध्, युध्, वीरुध्, श्वाविध्). Like मरुत, except in 11 cells: nom pl, acc sg, acc du, acc pl, ins sg, dat sg, ….
    declension!("स्पृध्", 8, "~ध्", like मरुत;
    nom: _, _, "~धः";
    acc: "~धम्", "~धौ", "~धः";
    ins: "~धा", _, _;
    dat: "~धे", _, _;
    abl: "~धः", _, _;
    gen_: "~धः", "~धोः", "~धाम्";
    loc: "~धि", _, _;
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
    // गोपा: 7 words (जा, पन्था, प्रतिमा, दा, गोपा, रथेष्ठा). Like कला, except in 10 cells: nom sg, acc du, acc pl, ins sg, dat sg, abl sg, ….
    declension!("गोपा", 7, "~ा", [gender: Masculine], like कला;
    nom: "~ाः", _, _;
    acc: _, "~ौ", "~ः";
    ins: "~ा", _, _;
    dat: "~े", _, _;
    abl: "~ः", _, _;
    gen_: "~ः", "~ोः", _;
    loc: "~ि", _, _;
    voc: "~ाः", _, _;
    ),
    // वारि: 7 words (वारि, अस्थि, अक्षि, इदानि, सेवि, दधि). Like सवामिन, except in 7 cells: nom sg, nom pl, acc sg, acc du, acc pl, gen_ pl, ….
    declension!("वारि", 7, "~ि", [gender: Neuter], like सवामिन;
    nom: "~ि", _, "~ीनि";
    acc: "~ि", "~िनी", "~ीनि";
    gen_: _, _, "~ीनाम्";
    voc: "~ि|~े", _, _;
    ),
    // तविष: 6 words (त्विष्, विष्, प्रेष्, प्रावृष्, द्विष्, रुष्).
    declension!("त्विष्", 6, "~ष्", [gender: Feminine];
    nom: "~ट्", _, "~षः";
    acc: "~षम्", "~षौ", "~षः";
    ins: "~षा", "~ड्भ्याम्", "~ड्भिः";
    dat: "~षे", _, "~ड्भ्यः";
    abl: "~षः", _, _;
    gen_: "~षः", "~षोः", "~षाम्";
    loc: "~षि", _, "~ट्सु";
    voc: "~ट्", _, _;
    ),
    // उसृ: 6 words (मातृ, दुहितृ, यातृ, ननान्दृ, उसृ, विमातृ). Like दातृ, except in 4 cells: nom pl, acc sg, acc du, acc pl.
    declension!("उसृ", 6, "~ृ", [gender: Feminine], like दातृ;
    nom: _, _, "~रः";
    acc: "~रम्", "~रौ", "~ॄः";
    ),
    // भू: 6 words (भ्रू, भू, स्वपू, सू, जू, मू). Like चमू, except in 12 cells: nom pl, acc sg, acc du, acc pl, ins sg, dat sg, ….
    declension!("भू", 6, "~ू", like चमू;
    nom: _, _, "~ुवः";
    acc: "~ुवम्", "~ुवौ", "~ुवः";
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
    // अरहत: 6 words (दत्, सत्, पृषत्, अर्हत्, सरत्, अर्वत्). Like मरुत, except in 5 cells: nom sg, nom pl, acc sg, acc du, voc sg.
    declension!("अर्हत्", 6, "~त्", [gender: Masculine], like मरुत;
    nom: "~न्", _, "~न्तः";
    acc: "~न्तम्", "~न्तौ", _;
    voc: "~न्", _, _;
    ),
    // चकषुस: 6 words (यजुस्, आयुस्, वपुस्, तरुस्, चक्षुस्, धनुस्).
    declension!("चक्षुस्", 6, "~ुस्";
    nom: "~ुः", _, "~ूंषि";
    acc: "~ुः", "~ुषी", "~ूंषि";
    ins: "~ुषा", "~ुर्भ्याम्", "~ुर्भिः";
    dat: "~ुषे", _, "~ुर्भ्यः";
    abl: "~ुषः", _, _;
    gen_: "~ुषः", "~ुषोः", "~ुषाम्";
    loc: "~ुषि", _, "~ुःषु|~ुष्षु";
    voc: "~ुः", _, _;
    ),
    // महत: 6 words (हिमवत्, महत्, परस्वत्, श्रीमत्, इरावत्, बलवत्). Like अरहत, except in 1 cell: nom sg.
    declension!("महत्", 6, "~त्", [gender: Masculine], like अरहत;
    nom: "~ान्", _, _;
    ),
    // तृ: 5 words (स्वसृ, स्तृ, तृ, पितृष्वसृ, पितुःष्वसृ). Like दातृ, except in 1 cell: acc pl.
    declension!("तृ", 5, "~ृ", [gender: Feminine], like दातृ;
    acc: _, _, "~ॄः";
    ),
    // जगत: 4 words (जगत्, यकृत्, बृहत्, असत्). Like मरुत, except in 4 cells: nom pl, acc sg, acc du, acc pl.
    declension!("जगत्", 4, "~त्", [gender: Neuter], like मरुत;
    nom: _, _, "~न्ति";
    acc: "~त्", "~ती", "~न्ति";
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
    nom: "~प्", _, "~पः";
    acc: "~पम्", "~पौ", "~पः";
    ins: "~पा", "~ब्भ्याम्", "~ब्भिः";
    dat: "~पे", _, "~ब्भ्यः";
    abl: "~पः", _, _;
    gen_: "~पः", "~पोः", "~पाम्";
    loc: "~पि", _, "~प्सु";
    voc: "~प्", _, _;
    ),
    // शुभ: 4 words (गृभ्, क्षुभ्, शुभ्, अनुष्टुभ्). Like रिप, except in 11 cells: nom pl, acc sg, acc du, acc pl, ins sg, dat sg, ….
    declension!("शुभ्", 4, "~भ्", [gender: Feminine], like रिप;
    nom: _, _, "~भः";
    acc: "~भम्", "~भौ", "~भः";
    ins: "~भा", _, _;
    dat: "~भे", _, _;
    abl: "~भः", _, _;
    gen_: "~भः", "~भोः", "~भाम्";
    loc: "~भि", _, _;
    ),
    // गलौ: 3 words (नौ, ज्यौ, ग्लौ).
    declension!("ग्लौ", 3, "~ौ";
    nom: "~ौः", _, "~ावः";
    acc: "~ावम्", "~ावौ", "~ावः";
    ins: "~ावा", "~ौभ्याम्", "~ौभिः";
    dat: "~ावे", _, "~ौभ्यः";
    abl: "~ावः", _, _;
    gen_: "~ावः", "~ावोः", "~ावाम्";
    loc: "~ावि", _, "~ौषु";
    voc: "~ौः", _, _;
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
];
