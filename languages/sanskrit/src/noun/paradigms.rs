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
/// `nominative` (sg, du, pl)
/// `accusative` (sg, du, pl)
/// `instrumental` (sg, du, pl)
/// `dative` (sg, du, pl)
/// `ablative` (sg, du, pl)
/// `genitive` (sg, du, pl)
/// `locative` (sg, du, pl)
/// `vocative` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident)?;
     nominative: $c0:tt, $c1:tt, $c2:tt;
     accusative: $c3:tt, $c4:tt, $c5:tt;
     instrumental: $c6:tt, $c7:tt, $c8:tt;
     dative: $c9:tt, $c10:tt, $c11:tt;
     ablative: $c12:tt, $c13:tt, $c14:tt;
     genitive: $c15:tt, $c16:tt, $c17:tt;
     locative: $c18:tt, $c19:tt, $c20:tt;
     vocative: $c21:tt, $c22:tt, $c23:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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
    /// Like `केतु`: 127 words.
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
    /// Like `आत्मन्`: 5 words.
    आतमन,
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
    /// Like `पर्वन्`: 2 words.
    परवन,
    /// Like `मरु`: 1 words.
    मरु,
    /// Like `जन्मन्`: 1 words.
    जनमन,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 44] = [
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
        Declension::गोपा,
        Declension::वारि,
        Declension::तविष,
        Declension::उसृ,
        Declension::भू,
        Declension::पितृ,
        Declension::अरहत,
        Declension::चकषुस,
        Declension::महत,
        Declension::आतमन,
        Declension::तृ,
        Declension::जगत,
        Declension::राडराजन,
        Declension::रिप,
        Declension::शुभ,
        Declension::गलौ,
        Declension::लकषमी,
        Declension::विराज,
        Declension::परवन,
        Declension::मरु,
        Declension::जनमन,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 44] = [
    // योग: 2,120 words (क, ङ, ज, ञ, ट, ण).
    declension!("योग", 2120, "~", [gender: Masculine];
        nominative: "~ः", "~ौ", "~ाः";
        accusative: "~म्", "~ौ", "~ान्";
        instrumental: "~ेन", "~ाभ्याम्", "~ैः";
        dative: "~ाय", "~ाभ्याम्", "~ेभ्यः";
        ablative: "~ात्", "~ाभ्याम्", "~ेभ्यः";
        genitive: "~स्य", "~योः", "~ानाम्";
        locative: "~े", "~योः", "~ेषु";
        vocative: "~", derived, "~ाः";
    ),
    // साधन: 1,091 words (विश्व, ख, साम्य, तृण, आसन, तीर). Like योग, except in 6 cells: nominative sg, nominative du, nominative pl, accusative du, accusative pl, vocative pl.
    declension!("साधन", 1091, "~", [gender: Neuter], like योग;
        nominative: "~म्", "~े", "~ानि";
        accusative: _, "~े", "~ानि";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: _, _, "~ानि";
    ),
    // कला: 506 words (देवता, काला, घृणा, भाषा, जरा, अहिंसा).
    declension!("कला", 506, "~ा", [gender: Feminine];
        nominative: "~ा", "~े", "~ाः";
        accusative: "~ाम्", "~े", "~ाः";
        instrumental: "~या", "~ाभ्याम्", "~ाभिः";
        dative: "~ायै", "~ाभ्याम्", "~ाभ्यः";
        ablative: "~ायाः", "~ाभ्याम्", "~ाभ्यः";
        genitive: "~ायाः", "~योः", "~ानाम्";
        locative: "~ायाम्", "~योः", "~ासु";
        vocative: "~े", derived, "~ाः";
    ),
    // गति: 261 words (मुक्ति, संस्कृति, बुद्धि, नाभि, राजनीति, कृषि).
    declension!("गति", 261, "~ि", [gender: Feminine];
        nominative: "~िः", "~ी", "~यः";
        accusative: "~िम्", "~ी", "~ीः";
        instrumental: "~्या", "~िभ्याम्", "~िभिः";
        dative: "~ये|~्यै", "~िभ्याम्", "~िभ्यः";
        ablative: "~ेः|~्याः", "~िभ्याम्", "~िभ्यः";
        genitive: "~ेः|~्याः", "~्योः", "~ीनाम्";
        locative: "~ौ|~्याम्", "~्योः", "~िषु";
        vocative: "~े", derived, "~यः";
    ),
    // नाडी: 233 words (वली, ताली, पत्नी, रोटी, नदी, पृथ्वी).
    declension!("नाडी", 233, "~ी", [gender: Feminine];
        nominative: "~ी", "~्यौ", "~्यः";
        accusative: "~ीम्", "~्यौ", "~ीः";
        instrumental: "~्या", "~ीभ्याम्", "~ीभिः";
        dative: "~्यै", "~ीभ्याम्", "~ीभ्यः";
        ablative: "~्याः", "~ीभ्याम्", "~ीभ्यः";
        genitive: "~्याः", "~्योः", "~ीनाम्";
        locative: "~्याम्", "~्योः", "~ीषु";
        vocative: "~ि", derived, "~्यः";
    ),
    // संधि: 134 words (संधि, अग्नि, रयि, गणपति, आदि, समाधि). Like गति, except in 6 cells: accusative pl, instrumental sg, dative sg, ablative sg, genitive sg, locative sg.
    declension!("संधि", 134, "~ि", [gender: Masculine], like गति;
        nominative: _, _, _;
        accusative: _, _, "~ीन्";
        instrumental: "~िना", _, _;
        dative: "~ये", _, _;
        ablative: "~ेः", _, _;
        genitive: "~ेः", _, _;
        locative: "~ौ", _, _;
        vocative: _, _, _;
    ),
    // केतु: 127 words (सूनु, पशु, परमाणु, गुरु, अणु, ऋतु).
    declension!("केतु", 127, "~ु", [gender: Masculine];
        nominative: "~ुः", "~ू", "~वः";
        accusative: "~ुम्", "~ू", "~ून्";
        instrumental: "~ुना", "~ुभ्याम्", "~ुभिः";
        dative: "~वे", "~ुभ्याम्", "~ुभ्यः";
        ablative: "~ोः", "~ुभ्याम्", "~ुभ्यः";
        genitive: "~ोः", "~्वोः", "~ूनाम्";
        locative: "~ौ", "~्वोः", "~ुषु";
        vocative: "~ो", derived, "~वः";
    ),
    // अपसस: 104 words (मनस्, शिरस्, अयस्, आस्, पसस्, नभस्).
    declension!("अप्सस्", 104, "~स्", [gender: Neuter];
        nominative: "~ः", "~सी", "~ांसि";
        accusative: "~ः", "~सी", "~ांसि";
        instrumental: "~सा", "~ोभ्याम्", "~ोभिः";
        dative: "~से", "~ोभ्याम्", "~ोभ्यः";
        ablative: "~सः", "~ोभ्याम्", "~ोभ्यः";
        genitive: "~सः", "~सोः", "~साम्";
        locative: "~सि", "~सोः", "~ःसु|~स्सु";
        vocative: "~ः", derived, "~ांसि";
    ),
    // दातृ: 56 words (दातृ, सवितृ, नप्तृ, होतृ, उद्गातृ, धातृ).
    declension!("दातृ", 56, "~ृ", [gender: Masculine];
        nominative: "~ा", "~ारौ", "~ारः";
        accusative: "~ारम्", "~ारौ", "~ॄन्";
        instrumental: "~्रा", "~ृभ्याम्", "~ृभिः";
        dative: "~्रे", "~ृभ्याम्", "~ृभ्यः";
        ablative: "~ुः", "~ृभ्याम्", "~ृभ्यः";
        genitive: "~ुः", "~्रोः", "~ॄणाम्";
        locative: "~रि", "~्रोः", "~ृषु";
        vocative: "~ः", derived, "~ारः";
    ),
    // मूरधन: 51 words (अश्मन्, ग्रावन्, श्वन्, क्लोमन्, युवन्, रक्षोहन्).
    declension!("मूर्धन्", 51, "~न्", [gender: Masculine];
        nominative: "~ा", "~ानौ", "~ानः";
        accusative: "~ानम्", "~ानौ", "~्नः";
        instrumental: "~्ना", "~भ्याम्", "~भिः";
        dative: "~्ने", "~भ्याम्", "~भ्यः";
        ablative: "~्नः", "~भ्याम्", "~भ्यः";
        genitive: "~्नः", "~्नोः", "~्नाम्";
        locative: "~्नि|~नि", "~्नोः", "~सु";
        vocative: "~न्", derived, "~ानः";
    ),
    // सवामिन: 49 words (हस्तिन्, मन्त्रिन्, फणिन्, अन्तर्यामिन्, केशिन्, रुधिरोद्गारिन्).
    declension!("स्वामिन्", 49, "~िन्", [gender: Masculine];
        nominative: "~ी", "~िनौ", "~िनः";
        accusative: "~िनम्", "~िनौ", "~िनः";
        instrumental: "~िना", "~िभ्याम्", "~िभिः";
        dative: "~िने", "~िभ्याम्", "~िभ्यः";
        ablative: "~िनः", "~िभ्याम्", "~िभ्यः";
        genitive: "~िनः", "~िनोः", "~िनाम्";
        locative: "~िनि", "~िनोः", "~िषु";
        vocative: "~िन्", derived, "~िनः";
    ),
    // धामन: 46 words (चर्मन्, नामन्, उदन्, अकर्मन्, ब्रह्मन्, कर्मन्). Like मूरधन, except in 8 cells: nominative sg, nominative du, nominative pl, accusative sg, accusative du, accusative pl, ….
    declension!("धामन्", 46, "~न्", [gender: Neuter], like मूरधन;
        nominative: "~", "~्नी|~नी", "~ानि";
        accusative: "~", "~्नी|~नी", "~ानि";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: "~न्|~", _, "~ानि";
    ),
    // मरुत: 19 words (मरुत्, नपात्, विद्युत्, दशत्, कृत्, हरित्).
    declension!("मरुत्", 19, "~त्";
        nominative: "~त्", "~तौ", "~तः";
        accusative: "~तम्", "~तौ", "~तः";
        instrumental: "~ता", "~द्भ्याम्", "~द्भिः";
        dative: "~ते", "~द्भ्याम्", "~द्भ्यः";
        ablative: "~तः", "~द्भ्याम्", "~द्भ्यः";
        genitive: "~तः", "~तोः", "~ताम्";
        locative: "~ति", "~तोः", "~त्सु";
        vocative: "~त्", derived, "~तः";
    ),
    // जतु: 19 words (वस्तु, आयु, मधु, अश्रु, दारु, जानु).
    declension!("जतु", 19, "~ु";
        nominative: "~ु", "~ुनी", "~ूनि";
        accusative: "~ु", "~ुनी", "~ूनि";
        instrumental: "~ुना", "~ुभ्याम्", "~ुभिः";
        dative: "~ुने", "~ुभ्याम्", "~ुभ्यः";
        ablative: "~ुनः", "~ुभ्याम्", "~ुभ्यः";
        genitive: "~ुनः", "~ुनोः", "~ूनाम्";
        locative: "~ुनि", "~ुनोः", "~ुषु";
        vocative: "~ु|~ो", derived, "~ूनि";
    ),
    // तनु: 17 words (आलु, हनु, लघु, धेनु, तनु, कु). Like केतु, except in 6 cells: accusative pl, instrumental sg, dative sg, ablative sg, genitive sg, locative sg.
    declension!("तनु", 17, "~ु", like केतु;
        nominative: _, _, _;
        accusative: _, _, "~ूः";
        instrumental: "~्वा", _, _;
        dative: "~वे|~्वै", _, _;
        ablative: "~ोः|~्वाः", _, _;
        genitive: "~ोः|~्वाः", _, _;
        locative: "~ौ|~्वाम्", _, _;
        vocative: _, _, _;
    ),
    // चमू: 16 words (वधू, तनू, श्वश्रू, जुहू, निम्बू, जनू).
    declension!("चमू", 16, "~ू";
        nominative: "~ूः", "~्वौ", "~्वः";
        accusative: "~ूम्", "~्वौ", "~ूः";
        instrumental: "~्वा", "~ूभ्याम्", "~ूभिः";
        dative: "~्वै", "~ूभ्याम्", "~ूभ्यः";
        ablative: "~्वाः", "~ूभ्याम्", "~ूभ्यः";
        genitive: "~्वाः", "~्वोः", "~ूनाम्";
        locative: "~्वाम्", "~्वोः", "~ूषु";
        vocative: "~ु", derived, "~्वः";
    ),
    // समपद: 15 words (हृद्, पद्, विद्, शरद्, मृद्, परिषद्).
    declension!("सम्पद्", 15, "~द्";
        nominative: "~त्", "~दौ", "~दः";
        accusative: "~दम्", "~दौ", "~दः";
        instrumental: "~दा", "~द्भ्याम्", "~द्भिः";
        dative: "~दे", "~द्भ्याम्", "~द्भ्यः";
        ablative: "~दः", "~द्भ्याम्", "~द्भ्यः";
        genitive: "~दः", "~दोः", "~दाम्";
        locative: "~दि", "~दोः", "~त्सु";
        vocative: "~त्", derived, "~दः";
    ),
    // जयोतिस: 14 words (दोस्, क्रविस्, सर्पिस्, बर्हिस्, हविस्, रोचिस्).
    declension!("ज्योतिस्", 14, "~~स्";
        nominative: "~~ः", "~~षी", "~ींष~";
        accusative: "~~ः", "~~षी", "~ींष~";
        instrumental: "~~षा", "~~र्भ्याम्", "~~र्भिः";
        dative: "~~षे", "~~र्भ्याम्", "~~र्भ्यः";
        ablative: "~~षः", "~~र्भ्याम्", "~~र्भ्यः";
        genitive: "~~षः", "~~षोः", "~~षाम्";
        locative: "~~षि", "~~षोः", "~~ःषु|~~ष्षु";
        vocative: "~~ः", derived, "~ींष~";
    ),
    // युज: 13 words (असृज्, बणिज्, वणिज्, भुज्, भिषज्, उशिज्).
    declension!("युज्", 13, "~ज्";
        nominative: "~क्", "~जौ", "~जः";
        accusative: "~जम्", "~जौ", "~जः";
        instrumental: "~जा", "~ग्भ्याम्", "~ग्भिः";
        dative: "~जे", "~ग्भ्याम्", "~ग्भ्यः";
        ablative: "~जः", "~ग्भ्याम्", "~ग्भ्यः";
        genitive: "~जः", "~जोः", "~जाम्";
        locative: "~जि", "~जोः", "~क्षु";
        vocative: "~क्", derived, "~जः";
    ),
    // धी: 10 words (ह्री, स्त्री, धी, श्री, भी, ली).
    declension!("धी", 10, "~ी";
        nominative: "~ीः", "~ियौ", "~ियः";
        accusative: "~ियम्", "~ियौ", "~ियः";
        instrumental: "~िया", "~ीभ्याम्", "~ीभिः";
        dative: "~िये", "~ीभ्याम्", "~ीभ्यः";
        ablative: "~ियः", "~ीभ्याम्", "~ीभ्यः";
        genitive: "~ियः", "~ियोः", "~ियाम्|~ीनाम्";
        locative: "~ियि", "~ियोः", "~ीषु";
        vocative: "~ीः", derived, "~ियः";
    ),
    // पुंस: 10 words (ऊधस्, उषस्, श्रेयस्, मास्, अप्सरस्, पुंस्). Like अपसस, except in 7 cells: nominative sg, nominative du, nominative pl, accusative sg, accusative du, accusative pl, ….
    declension!("पुंस्", 10, "~स्", like अपसस;
        nominative: "~ाः", "~सौ", "~सः";
        accusative: "~सम्", "~सौ", "~सः";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: _, _, "~सः";
    ),
    // तवच: 9 words (त्वच्, ऋच्, रुच्, शुच्, मुच्, स्फिच्).
    declension!("त्वच्", 9, "~च्";
        nominative: "~क्", "~चौ", "~चः";
        accusative: "~चम्", "~चौ", "~चः";
        instrumental: "~चा", "~ग्भ्याम्", "~ग्भिः";
        dative: "~चे", "~ग्भ्याम्", "~ग्भ्यः";
        ablative: "~चः", "~ग्भ्याम्", "~ग्भ्यः";
        genitive: "~चः", "~चोः", "~चाम्";
        locative: "~चि", "~चोः", "~क्षु";
        vocative: "~क्", derived, "~चः";
    ),
    // सपृध: 8 words (प्रबुध्, स्पृध्, क्षुध्, युध्, वीरुध्, श्वाविध्).
    declension!("स्पृध्", 8, "~ध्";
        nominative: "~त्", "~धौ", "~धः";
        accusative: "~धम्", "~धौ", "~धः";
        instrumental: "~धा", "~द्भ्याम्", "~द्भिः";
        dative: "~धे", "~द्भ्याम्", "~द्भ्यः";
        ablative: "~धः", "~द्भ्याम्", "~द्भ्यः";
        genitive: "~धः", "~धोः", "~धाम्";
        locative: "~धि", "~धोः", "~त्सु";
        vocative: "~त्", derived, "~धः";
    ),
    // गोपा: 7 words (जा, पन्था, प्रतिमा, दा, गोपा, रथेष्ठा). Like कला, except in 12 cells: nominative sg, nominative du, accusative du, accusative pl, instrumental sg, dative sg, ….
    declension!("गोपा", 7, "~ा", [gender: Masculine], like कला;
        nominative: "~ाः", "~ौ", _;
        accusative: _, "~ौ", "~ः";
        instrumental: "~ा", _, _;
        dative: "~े", _, _;
        ablative: "~ः", _, _;
        genitive: "~ः", "~ोः", _;
        locative: "~ि", "~ोः", _;
        vocative: "~ाः", _, _;
    ),
    // वारि: 7 words (वारि, अस्थि, अक्षि, इदानि, सेवि, दधि). Like सवामिन, except in 9 cells: nominative sg, nominative du, nominative pl, accusative sg, accusative du, accusative pl, ….
    declension!("वारि", 7, "~ि", [gender: Neuter], like सवामिन;
        nominative: "~ि", "~िनी", "~ीनि";
        accusative: "~ि", "~िनी", "~ीनि";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, "~ीनाम्";
        locative: _, _, _;
        vocative: "~ि|~े", _, "~ीनि";
    ),
    // तविष: 6 words (त्विष्, विष्, प्रेष्, प्रावृष्, द्विष्, रुष्).
    declension!("त्विष्", 6, "~ष्", [gender: Feminine];
        nominative: "~ट्", "~षौ", "~षः";
        accusative: "~षम्", "~षौ", "~षः";
        instrumental: "~षा", "~ड्भ्याम्", "~ड्भिः";
        dative: "~षे", "~ड्भ्याम्", "~ड्भ्यः";
        ablative: "~षः", "~ड्भ्याम्", "~ड्भ्यः";
        genitive: "~षः", "~षोः", "~षाम्";
        locative: "~षि", "~षोः", "~ट्सु";
        vocative: "~ट्", derived, "~षः";
    ),
    // उसृ: 6 words (मातृ, दुहितृ, यातृ, ननान्दृ, उसृ, विमातृ). Like दातृ, except in 6 cells: nominative du, nominative pl, accusative sg, accusative du, accusative pl, vocative pl.
    declension!("उसृ", 6, "~ृ", [gender: Feminine], like दातृ;
        nominative: _, "~रौ", "~रः";
        accusative: "~रम्", "~रौ", "~ॄः";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: _, _, "~रः";
    ),
    // भू: 6 words (भ्रू, भू, स्वपू, सू, जू, मू).
    declension!("भू", 6, "~ू";
        nominative: "~ूः", "~ुवौ", "~ुवः";
        accusative: "~ुवम्", "~ुवौ", "~ुवः";
        instrumental: "~ुवा", "~ूभ्याम्", "~ूभिः";
        dative: "~ुवे", "~ूभ्याम्", "~ूभ्यः";
        ablative: "~ुवः", "~ूभ्याम्", "~ूभ्यः";
        genitive: "~ुवः", "~ुवोः", "~ुवाम्|~ूनाम्";
        locative: "~ुवि", "~ुवोः", "~ूषु";
        vocative: "~ूः", derived, "~ुवः";
    ),
    // पितृ: 6 words (भ्रातृ, पितृ, देवृ, नृ, जामातृ, मातापितृ). Like उसृ, except in 1 cell: accusative pl.
    declension!("पितृ", 6, "~ृ", [gender: Masculine], like उसृ;
        nominative: _, _, _;
        accusative: _, _, "~ॄन्";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: _, _, _;
    ),
    // अरहत: 6 words (दत्, सत्, पृषत्, अर्हत्, सरत्, अर्वत्). Like मरुत, except in 7 cells: nominative sg, nominative du, nominative pl, accusative sg, accusative du, vocative sg, ….
    declension!("अर्हत्", 6, "~त्", [gender: Masculine], like मरुत;
        nominative: "~न्", "~न्तौ", "~न्तः";
        accusative: "~न्तम्", "~न्तौ", _;
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: "~न्", _, "~न्तः";
    ),
    // चकषुस: 6 words (यजुस्, आयुस्, वपुस्, तरुस्, चक्षुस्, धनुस्).
    declension!("चक्षुस्", 6, "~ुस्";
        nominative: "~ुः", "~ुषी", "~ूंषि";
        accusative: "~ुः", "~ुषी", "~ूंषि";
        instrumental: "~ुषा", "~ुर्भ्याम्", "~ुर्भिः";
        dative: "~ुषे", "~ुर्भ्याम्", "~ुर्भ्यः";
        ablative: "~ुषः", "~ुर्भ्याम्", "~ुर्भ्यः";
        genitive: "~ुषः", "~ुषोः", "~ुषाम्";
        locative: "~ुषि", "~ुषोः", "~ुःषु|~ुष्षु";
        vocative: "~ुः", derived, "~ूंषि";
    ),
    // महत: 6 words (हिमवत्, महत्, परस्वत्, श्रीमत्, इरावत्, बलवत्). Like अरहत, except in 1 cell: nominative sg.
    declension!("महत्", 6, "~त्", [gender: Masculine], like अरहत;
        nominative: "~ान्", _, _;
        accusative: _, _, _;
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: _, _, _;
    ),
    // आतमन: 5 words (अथर्वन्, वन्, अध्वन्, यज्वन्, प्रातरित्वन्). Like मूरधन, except in 9 cells: accusative pl, instrumental sg, dative sg, ablative sg, genitive sg, genitive du, ….
    declension!("आत्मन्", 5, "~न्", [gender: Masculine], like मूरधन;
        nominative: _, _, _;
        accusative: _, _, "~नः";
        instrumental: "~ना", _, _;
        dative: "~ने", _, _;
        ablative: "~नः", _, _;
        genitive: "~नः", "~नोः", "~नाम्";
        locative: "~नि", "~नोः", _;
        vocative: _, _, _;
    ),
    // तृ: 5 words (स्वसृ, स्तृ, तृ, पितृष्वसृ, पितुःष्वसृ). Like दातृ, except in 1 cell: accusative pl.
    declension!("तृ", 5, "~ृ", [gender: Feminine], like दातृ;
        nominative: _, _, _;
        accusative: _, _, "~ॄः";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: _, _, _;
    ),
    // जगत: 4 words (जगत्, यकृत्, बृहत्, असत्). Like मरुत, except in 6 cells: nominative du, nominative pl, accusative sg, accusative du, accusative pl, vocative pl.
    declension!("जगत्", 4, "~त्", [gender: Neuter], like मरुत;
        nominative: _, "~ती", "~न्ति";
        accusative: "~त्", "~ती", "~न्ति";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: _, _, "~न्ति";
    ),
    // राडराजन: 4 words (राजन्, मज्जन्, महाराजन्, राड्राजन्). Like मूरधन, except in 9 cells: accusative pl, instrumental sg, dative sg, ablative sg, genitive sg, genitive du, ….
    declension!("राड्राजन्", 4, "~न्", [gender: Masculine], like मूरधन;
        nominative: _, _, _;
        accusative: _, _, "~्ञः";
        instrumental: "~्ञा", _, _;
        dative: "~्ञे", _, _;
        ablative: "~्ञः", _, _;
        genitive: "~्ञः", "~्ञोः", "~्ञाम्";
        locative: "~्ञि|~नि", "~्ञोः", _;
        vocative: _, _, _;
    ),
    // रिप: 4 words (अप्, क्षप्, कृप्, रिप्).
    declension!("रिप्", 4, "~प्", [gender: Feminine];
        nominative: "~प्", "~पौ", "~पः";
        accusative: "~पम्", "~पौ", "~पः";
        instrumental: "~पा", "~ब्भ्याम्", "~ब्भिः";
        dative: "~पे", "~ब्भ्याम्", "~ब्भ्यः";
        ablative: "~पः", "~ब्भ्याम्", "~ब्भ्यः";
        genitive: "~पः", "~पोः", "~पाम्";
        locative: "~पि", "~पोः", "~प्सु";
        vocative: "~प्", derived, "~पः";
    ),
    // शुभ: 4 words (गृभ्, क्षुभ्, शुभ्, अनुष्टुभ्).
    declension!("शुभ्", 4, "~भ्", [gender: Feminine];
        nominative: "~प्", "~भौ", "~भः";
        accusative: "~भम्", "~भौ", "~भः";
        instrumental: "~भा", "~ब्भ्याम्", "~ब्भिः";
        dative: "~भे", "~ब्भ्याम्", "~ब्भ्यः";
        ablative: "~भः", "~ब्भ्याम्", "~ब्भ्यः";
        genitive: "~भः", "~भोः", "~भाम्";
        locative: "~भि", "~भोः", "~प्सु";
        vocative: "~प्", derived, "~भः";
    ),
    // गलौ: 3 words (नौ, ज्यौ, ग्लौ).
    declension!("ग्लौ", 3, "~ौ";
        nominative: "~ौः", "~ावौ", "~ावः";
        accusative: "~ावम्", "~ावौ", "~ावः";
        instrumental: "~ावा", "~ौभ्याम्", "~ौभिः";
        dative: "~ावे", "~ौभ्याम्", "~ौभ्यः";
        ablative: "~ावः", "~ौभ्याम्", "~ौभ्यः";
        genitive: "~ावः", "~ावोः", "~ावाम्";
        locative: "~ावि", "~ावोः", "~ौषु";
        vocative: "~ौः", derived, "~ावः";
    ),
    // लकषमी: 3 words (लक्ष्मी, तन्त्री, तन्द्री). Like नाडी, except in 2 cells: nominative sg, genitive pl.
    declension!("लक्ष्मी", 3, "~ी", [gender: Feminine], like नाडी;
        nominative: "~ीः|~ी", _, _;
        accusative: _, _, _;
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, "~ीणाम्";
        locative: _, _, _;
        vocative: _, _, _;
    ),
    // विराज: 3 words (विराज्, राज्, सम्राज्). Like युज, except in 9 cells: nominative sg, instrumental du, instrumental pl, dative du, dative pl, ablative du, ….
    declension!("विराज्", 3, "~ज्", like युज;
        nominative: "~ट्", _, _;
        accusative: _, _, _;
        instrumental: _, "~ड्भ्याम्", "~ड्भिः";
        dative: _, "~ड्भ्याम्", "~ड्भ्यः";
        ablative: _, "~ड्भ्याम्", "~ड्भ्यः";
        genitive: _, _, _;
        locative: _, _, "~ट्सु";
        vocative: "~ट्", _, _;
    ),
    // परवन: 2 words (पर्वन्, भुर्वन्).
    declension!("पर्वन्", 2, "~न्";
        nominative: "~", "~णी", "~ाणि";
        accusative: "~", "~णी", "~ाणि";
        instrumental: "~णा", "~भ्याम्", "~भिः";
        dative: "~णे", "~भ्याम्", "~भ्यः";
        ablative: "~णः", "~भ्याम्", "~भ्यः";
        genitive: "~णः", "~णोः", "~णाम्";
        locative: "~णि", "~णोः", "~सु";
        vocative: "~न्|~", derived, "~ाणि";
    ),
    // मरु: 1 words (ऋभु). Like केतु, except in 2 cells: instrumental sg, genitive pl.
    declension!("मरु", 1, "~ु", like केतु;
        nominative: _, _, _;
        accusative: _, _, _;
        instrumental: "~ुणा", _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, "~ूणाम्";
        locative: _, _, _;
        vocative: _, _, _;
    ),
    // जनमन: 1 words (धन्वन्). Like आतमन, except in 8 cells: nominative sg, nominative du, nominative pl, accusative sg, accusative du, accusative pl, ….
    declension!("जन्मन्", 1, "~न्", like आतमन;
        nominative: "~", "~नी", "~ानि";
        accusative: "~", "~नी", "~ानि";
        instrumental: _, _, _;
        dative: _, _, _;
        ablative: _, _, _;
        genitive: _, _, _;
        locative: _, _, _;
        vocative: "~न्|~", _, "~ानि";
    ),
];
