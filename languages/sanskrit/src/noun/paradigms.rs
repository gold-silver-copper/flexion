//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's gender says):
/// `nominative` (sg, du, pl)
/// `accusative` (sg, du, pl)
/// `instrumental` (sg, du, pl)
/// `dative` (sg, du, pl)
/// `ablative` (sg, du, pl)
/// `genitive` (sg, du, pl)
/// `locative` (sg, du, pl)
/// `vocative` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `योग`: 1,486 words.
    योग,
    /// Like `साधन`: 859 words.
    साधन,
    /// Like `विषय`: 634 words.
    विषय,
    /// Like `कला`: 385 words.
    कला,
    /// Like `गति`: 244 words.
    गति,
    /// Like `गौरव`: 232 words.
    गौरव,
    /// Like `नाडी`: 184 words.
    नाडी,
    /// Like `परा`: 121 words.
    परा,
    /// Like `संधि`: 107 words.
    संधि,
    /// Like `अप्सस्`: 104 words.
    अपसस,
    /// Like `केतु`: 92 words.
    केतु,
    /// Like `दातृ`: 56 words.
    दातृ,
    /// Like `पुरी`: 49 words.
    पुरी,
    /// Like `मरु`: 36 words.
    मरु,
    /// Like `स्वामिन्`: 35 words.
    सवामिन,
    /// Like `हरि`: 27 words.
    हरि,
    /// Like `मूर्धन्`: 24 words.
    मूरधन,
    /// Like `मरुत्`: 19 words.
    मरुत,
    /// Like `धामन्`: 18 words.
    धामन,
    /// Like `मषि`: 17 words.
    मषि,
    /// Like `तनु`: 17 words.
    तनु,
    /// Like `पर्वन्`: 15 words.
    परवन,
    /// Like `सम्पद्`: 15 words.
    समपद,
    /// Like `जतु`: 14 words.
    जतु,
    /// Like `वारकिन्`: 14 words.
    वारकिन,
    /// Like `ज्योतिस्`: 14 words.
    जयोतिस,
    /// Like `युज्`: 13 words.
    युज,
    /// Like `आत्मन्`: 13 words.
    आतमन,
    /// Like `ग्रावन्`: 11 words.
    गरावन,
    /// Like `चमू`: 11 words.
    चमू,
    /// Like `धी`: 10 words.
    धी,
    /// Like `पुंस्`: 10 words.
    पुंस,
    /// Like `त्वच्`: 9 words.
    तवच,
    /// Like `जन्मन्`: 9 words.
    जनमन,
    /// Like `ऊष्मन्`: 8 words.
    ऊषमन,
    /// Like `स्पृध्`: 8 words.
    सपृध,
    /// Like `गोपा`: 7 words.
    गोपा,
    /// Like `वारि`: 7 words.
    वारि,
    /// Like `भरीमन्`: 7 words.
    भरीमन,
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
    /// Like `दारु`: 5 words.
    दारु,
    /// Like `पितृभू`: 5 words.
    पितृभू,
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
    pub const ALL: [Declension; 56] = [
        Declension::योग,
        Declension::साधन,
        Declension::विषय,
        Declension::कला,
        Declension::गति,
        Declension::गौरव,
        Declension::नाडी,
        Declension::परा,
        Declension::संधि,
        Declension::अपसस,
        Declension::केतु,
        Declension::दातृ,
        Declension::पुरी,
        Declension::मरु,
        Declension::सवामिन,
        Declension::हरि,
        Declension::मूरधन,
        Declension::मरुत,
        Declension::धामन,
        Declension::मषि,
        Declension::तनु,
        Declension::परवन,
        Declension::समपद,
        Declension::जतु,
        Declension::वारकिन,
        Declension::जयोतिस,
        Declension::युज,
        Declension::आतमन,
        Declension::गरावन,
        Declension::चमू,
        Declension::धी,
        Declension::पुंस,
        Declension::तवच,
        Declension::जनमन,
        Declension::ऊषमन,
        Declension::सपृध,
        Declension::गोपा,
        Declension::वारि,
        Declension::भरीमन,
        Declension::तविष,
        Declension::उसृ,
        Declension::भू,
        Declension::पितृ,
        Declension::अरहत,
        Declension::चकषुस,
        Declension::महत,
        Declension::तृ,
        Declension::दारु,
        Declension::पितृभू,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 56] = [
    // योग: 1,486 words (क, ङ, ज, ञ, ट, ण).
    declension!("योग", 1486, "~";
        nominative: "~ः", "~ौ", "~ाः";
        accusative: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        instrumental: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        locative: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative: "~", referred, "~ाः";
    ),
    // साधन: 859 words (विश्व, ख, साम्य, तृण, आसन, वन).
    declension!("साधन", 859, "~";
        nominative: "~म\u{94d}", "~\u{947}", "~ानि";
        accusative: "~म\u{94d}", "~\u{947}", "~ानि";
        instrumental: "~\u{947}न", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive: "~स\u{94d}य", "~योः", "~ानाम\u{94d}";
        locative: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative: "~", referred, "~ानि";
    ),
    // विषय: 634 words (वृक्ष, समुद्र, तरंग, गृह, मूर्ख, मधुर).
    declension!("विषय", 634, "~";
        nominative: "~ः", "~ौ", "~ाः";
        accusative: "~म\u{94d}", "~ौ", "~ान\u{94d}";
        instrumental: "~\u{947}ण", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive: "~स\u{94d}य", "~योः", "~ाणाम\u{94d}";
        locative: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative: "~", referred, "~ाः";
    ),
    // कला: 385 words (देवता, काला, घृणा, अहिंसा, दया, तुला).
    declension!("कला", 385, "~ा";
        nominative: "~ा", "~\u{947}", "~ाः";
        accusative: "~ाम\u{94d}", "~\u{947}", "~ाः";
        instrumental: "~या", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        dative: "~ाय\u{948}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative: "~ायाः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        genitive: "~ायाः", "~योः", "~ानाम\u{94d}";
        locative: "~ायाम\u{94d}", "~योः", "~ास\u{941}";
        vocative: "~\u{947}", referred, "~ाः";
    ),
    // गति: 244 words (मुक्ति, संस्कृति, बुद्धि, नाभि, राजनीति, गति).
    declension!("गति", 244, "~ि";
        nominative: "~िः", "~ी", "~यः";
        accusative: "~िम\u{94d}", "~ी", "~ीः";
        instrumental: "~\u{94d}या", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative: "~य\u{947}|~\u{94d}य\u{948}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative: "~\u{947}ः|~\u{94d}याः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive: "~\u{947}ः|~\u{94d}याः", "~\u{94d}योः", "~ीनाम\u{94d}";
        locative: "~ौ|~\u{94d}याम\u{94d}", "~\u{94d}योः", "~िष\u{941}";
        vocative: "~\u{947}", referred, "~यः";
    ),
    // गौरव: 232 words (तीर, ताम्र, द्वार, अभ्र, कार्य, क्षेत्र).
    declension!("गौरव", 232, "~";
        nominative: "~म\u{94d}", "~\u{947}", "~ाणि";
        accusative: "~म\u{94d}", "~\u{947}", "~ाणि";
        instrumental: "~\u{947}ण", "~ाभ\u{94d}याम\u{94d}", "~\u{948}ः";
        dative: "~ाय", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        ablative: "~ात\u{94d}", "~ाभ\u{94d}याम\u{94d}", "~\u{947}भ\u{94d}यः";
        genitive: "~स\u{94d}य", "~योः", "~ाणाम\u{94d}";
        locative: "~\u{947}", "~योः", "~\u{947}ष\u{941}";
        vocative: "~", referred, "~ाणि";
    ),
    // नाडी: 184 words (वली, ताली, पत्नी, रोटी, नदी, पृथ्वी).
    declension!("नाडी", 184, "~ी";
        nominative: "~ी", "~\u{94d}यौ", "~\u{94d}यः";
        accusative: "~ीम\u{94d}", "~\u{94d}यौ", "~ीः";
        instrumental: "~\u{94d}या", "~ीभ\u{94d}याम\u{94d}", "~ीभिः";
        dative: "~\u{94d}य\u{948}", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        ablative: "~\u{94d}याः", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        genitive: "~\u{94d}याः", "~\u{94d}योः", "~ीनाम\u{94d}";
        locative: "~\u{94d}याम\u{94d}", "~\u{94d}योः", "~ीष\u{941}";
        vocative: "~ि", referred, "~\u{94d}यः";
    ),
    // परा: 121 words (भाषा, जरा, हीरा, क्रिया, ईर्षा, तारा).
    declension!("परा", 121, "~ा";
        nominative: "~ा", "~\u{947}", "~ाः";
        accusative: "~ाम\u{94d}", "~\u{947}", "~ाः";
        instrumental: "~या", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        dative: "~ाय\u{948}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative: "~ायाः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        genitive: "~ायाः", "~योः", "~ाणाम\u{94d}";
        locative: "~ायाम\u{94d}", "~योः", "~ास\u{941}";
        vocative: "~\u{947}", referred, "~ाः";
    ),
    // संधि: 107 words (संधि, अग्नि, गणपति, आदि, समाधि, पति).
    declension!("स\u{902}धि", 107, "~ि";
        nominative: "~िः", "~ी", "~यः";
        accusative: "~िम\u{94d}", "~ी", "~ीन\u{94d}";
        instrumental: "~िना", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative: "~य\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative: "~\u{947}ः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive: "~\u{947}ः", "~\u{94d}योः", "~ीनाम\u{94d}";
        locative: "~ौ", "~\u{94d}योः", "~िष\u{941}";
        vocative: "~\u{947}", referred, "~यः";
    ),
    // अपसस: 104 words (मनस्, शिरस्, अयस्, आस्, पसस्, नभस्).
    declension!("अप\u{94d}सस\u{94d}", 104, "~स\u{94d}";
        nominative: "~ः", "~सी", "~ा\u{902}सि";
        accusative: "~ः", "~सी", "~ा\u{902}सि";
        instrumental: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        dative: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        genitive: "~सः", "~सोः", "~साम\u{94d}";
        locative: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        vocative: "~ः", referred, "~ा\u{902}सि";
    ),
    // केतु: 92 words (सूनु, पशु, परमाणु, अणु, ऋतु, परशु).
    declension!("क\u{947}त\u{941}", 92, "~\u{941}";
        nominative: "~\u{941}ः", "~\u{942}", "~वः";
        accusative: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}न\u{94d}";
        instrumental: "~\u{941}ना", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative: "~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative: "~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive: "~ोः", "~\u{94d}वोः", "~\u{942}नाम\u{94d}";
        locative: "~ौ", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        vocative: "~ो", referred, "~वः";
    ),
    // दातृ: 56 words (दातृ, सवितृ, नप्तृ, होतृ, उद्गातृ, धातृ).
    declension!("दात\u{943}", 56, "~\u{943}";
        nominative: "~ा", "~ारौ", "~ारः";
        accusative: "~ारम\u{94d}", "~ारौ", "~\u{944}न\u{94d}";
        instrumental: "~\u{94d}रा", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भिः";
        dative: "~\u{94d}र\u{947}", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        ablative: "~\u{941}ः", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        genitive: "~\u{941}ः", "~\u{94d}रोः", "~\u{944}णाम\u{94d}";
        locative: "~रि", "~\u{94d}रोः", "~\u{943}ष\u{941}";
        vocative: "~ः", referred, "~ारः";
    ),
    // पुरी: 49 words (पुत्री, किशोरी, क्षौरी, नारी, धरित्री, पुरी).
    declension!("प\u{941}री", 49, "~ी";
        nominative: "~ी", "~\u{94d}यौ", "~\u{94d}यः";
        accusative: "~ीम\u{94d}", "~\u{94d}यौ", "~ीः";
        instrumental: "~\u{94d}या", "~ीभ\u{94d}याम\u{94d}", "~ीभिः";
        dative: "~\u{94d}य\u{948}", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        ablative: "~\u{94d}याः", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        genitive: "~\u{94d}याः", "~\u{94d}योः", "~ीणाम\u{94d}";
        locative: "~\u{94d}याम\u{94d}", "~\u{94d}योः", "~ीष\u{941}";
        vocative: "~ि", referred, "~\u{94d}यः";
    ),
    // मरु: 36 words (गुरु, अगुरु, तर्कु, तरु, प्रियङ्गु, द्रु).
    declension!("मर\u{941}", 36, "~\u{941}";
        nominative: "~\u{941}ः", "~\u{942}", "~वः";
        accusative: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}न\u{94d}";
        instrumental: "~\u{941}णा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative: "~व\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative: "~ोः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive: "~ोः", "~\u{94d}वोः", "~\u{942}णाम\u{94d}";
        locative: "~ौ", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        vocative: "~ो", referred, "~वः";
    ),
    // सवामिन: 35 words (हस्तिन्, फणिन्, केशिन्, ज्ञानिन्, घातिन्, स्वामिन्).
    declension!("स\u{94d}वामिन\u{94d}", 35, "~िन\u{94d}";
        nominative: "~ी", "~िनौ", "~िनः";
        accusative: "~िनम\u{94d}", "~िनौ", "~िनः";
        instrumental: "~िना", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative: "~िन\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative: "~िनः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive: "~िनः", "~िनोः", "~िनाम\u{94d}";
        locative: "~िनि", "~िनोः", "~िष\u{941}";
        vocative: "~िन\u{94d}", referred, "~िनः";
    ),
    // हरि: 27 words (रयि, गिरि, हरि, कृमि, अंह्रि, बहुव्रीहि).
    declension!("हरि", 27, "~ि";
        nominative: "~िः", "~ी", "~यः";
        accusative: "~िम\u{94d}", "~ी", "~ीन\u{94d}";
        instrumental: "~िणा", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative: "~य\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative: "~\u{947}ः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive: "~\u{947}ः", "~\u{94d}योः", "~ीणाम\u{94d}";
        locative: "~ौ", "~\u{94d}योः", "~िष\u{941}";
        vocative: "~\u{947}", referred, "~यः";
    ),
    // मूरधन: 24 words (श्वन्, क्लोमन्, युवन्, रक्षोहन्, व्योमन्, प्लीहन्).
    declension!("म\u{942}र\u{94d}धन\u{94d}", 24, "~न\u{94d}";
        nominative: "~ा", "~ानौ", "~ानः";
        accusative: "~ानम\u{94d}", "~ानौ", "~\u{94d}नः";
        instrumental: "~\u{94d}ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~\u{94d}न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~\u{94d}नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~\u{94d}नः", "~\u{94d}नोः", "~\u{94d}नाम\u{94d}";
        locative: "~\u{94d}नि|~नि", "~\u{94d}नोः", "~स\u{941}";
        vocative: "~न\u{94d}", referred, "~ानः";
    ),
    // मरुत: 19 words (मरुत्, नपात्, विद्युत्, दशत्, कृत्, हरित्).
    declension!("मर\u{941}त\u{94d}", 19, "~त\u{94d}";
        nominative: "~त\u{94d}", "~तौ", "~तः";
        accusative: "~तम\u{94d}", "~तौ", "~तः";
        instrumental: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive: "~तः", "~तोः", "~ताम\u{94d}";
        locative: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        vocative: "~त\u{94d}", referred, "~तः";
    ),
    // धामन: 18 words (नामन्, उदन्, सामन्, भूमन्, जनिमन्, स्नावन्).
    declension!("धामन\u{94d}", 18, "~न\u{94d}";
        nominative: "~", "~\u{94d}नी|~नी", "~ानि";
        accusative: "~", "~\u{94d}नी|~नी", "~ानि";
        instrumental: "~\u{94d}ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~\u{94d}न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~\u{94d}नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~\u{94d}नः", "~\u{94d}नोः", "~\u{94d}नाम\u{94d}";
        locative: "~\u{94d}नि|~नि", "~\u{94d}नोः", "~स\u{941}";
        vocative: "~न\u{94d}|~", referred, "~ानि";
    ),
    // मषि: 17 words (कृषि, रात्रि, अश्रि, शारि, अङ्गुरि, मषि).
    declension!("मषि", 17, "~ि";
        nominative: "~िः", "~ी", "~यः";
        accusative: "~िम\u{94d}", "~ी", "~ीः";
        instrumental: "~\u{94d}या", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative: "~य\u{947}|~\u{94d}य\u{948}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative: "~\u{947}ः|~\u{94d}याः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive: "~\u{947}ः|~\u{94d}याः", "~\u{94d}योः", "~ीणाम\u{94d}";
        locative: "~ौ|~\u{94d}याम\u{94d}", "~\u{94d}योः", "~िष\u{941}";
        vocative: "~\u{947}", referred, "~यः";
    ),
    // तनु: 17 words (आलु, हनु, लघु, धेनु, तनु, कु).
    declension!("तन\u{941}", 17, "~\u{941}";
        nominative: "~\u{941}ः", "~\u{942}", "~वः";
        accusative: "~\u{941}म\u{94d}", "~\u{942}", "~\u{942}ः";
        instrumental: "~\u{94d}वा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative: "~व\u{947}|~\u{94d}व\u{948}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative: "~ोः|~\u{94d}वाः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive: "~ोः|~\u{94d}वाः", "~\u{94d}वोः", "~\u{942}नाम\u{94d}";
        locative: "~ौ|~\u{94d}वाम\u{94d}", "~\u{94d}वोः", "~\u{941}ष\u{941}";
        vocative: "~ो", referred, "~वः";
    ),
    // परवन: 15 words (चर्मन्, अकर्मन्, ब्रह्मन्, कर्मन्, मर्मन्, शर्मन्).
    declension!("पर\u{94d}वन\u{94d}", 15, "~न\u{94d}";
        nominative: "~", "~णी", "~ाणि";
        accusative: "~", "~णी", "~ाणि";
        instrumental: "~णा", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~ण\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~णः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~णः", "~णोः", "~णाम\u{94d}";
        locative: "~णि", "~णोः", "~स\u{941}";
        vocative: "~न\u{94d}|~", referred, "~ाणि";
    ),
    // समपद: 15 words (हृद्, पद्, विद्, शरद्, मृद्, परिषद्).
    declension!("सम\u{94d}पद\u{94d}", 15, "~द\u{94d}";
        nominative: "~त\u{94d}", "~दौ", "~दः";
        accusative: "~दम\u{94d}", "~दौ", "~दः";
        instrumental: "~दा", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative: "~द\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative: "~दः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive: "~दः", "~दोः", "~दाम\u{94d}";
        locative: "~दि", "~दोः", "~त\u{94d}स\u{941}";
        vocative: "~त\u{94d}", referred, "~दः";
    ),
    // जतु: 14 words (वस्तु, आयु, मधु, जानु, अंहु, वास्तु).
    declension!("जत\u{941}", 14, "~\u{941}";
        nominative: "~\u{941}", "~\u{941}नी", "~\u{942}नि";
        accusative: "~\u{941}", "~\u{941}नी", "~\u{942}नि";
        instrumental: "~\u{941}ना", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative: "~\u{941}न\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative: "~\u{941}नः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive: "~\u{941}नः", "~\u{941}नोः", "~\u{942}नाम\u{94d}";
        locative: "~\u{941}नि", "~\u{941}नोः", "~\u{941}ष\u{941}";
        vocative: "~\u{941}|~ो", referred, "~\u{942}नि";
    ),
    // वारकिन: 14 words (मन्त्रिन्, अन्तर्यामिन्, रुधिरोद्गारिन्, शास्त्रिन्, रोगिन्, साक्षिन्).
    declension!("वारकिन\u{94d}", 14, "~िन\u{94d}";
        nominative: "~ी", "~िणौ", "~िणः";
        accusative: "~िणम\u{94d}", "~िणौ", "~िणः";
        instrumental: "~िणा", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative: "~िण\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative: "~िणः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive: "~िणः", "~िणोः", "~िणाम\u{94d}";
        locative: "~िणि", "~िणोः", "~िष\u{941}";
        vocative: "~िन\u{94d}", referred, "~िणः";
    ),
    // जयोतिस: 14 words (दोस्, क्रविस्, सर्पिस्, बर्हिस्, हविस्, रोचिस्).
    declension!("ज\u{94d}योतिस\u{94d}", 14, "~~स\u{94d}";
        nominative: "~~ः", "~~षी", "~ी\u{902}ष~";
        accusative: "~~ः", "~~षी", "~ी\u{902}ष~";
        instrumental: "~~षा", "~~र\u{94d}भ\u{94d}याम\u{94d}", "~~र\u{94d}भिः";
        dative: "~~ष\u{947}", "~~र\u{94d}भ\u{94d}याम\u{94d}", "~~र\u{94d}भ\u{94d}यः";
        ablative: "~~षः", "~~र\u{94d}भ\u{94d}याम\u{94d}", "~~र\u{94d}भ\u{94d}यः";
        genitive: "~~षः", "~~षोः", "~~षाम\u{94d}";
        locative: "~~षि", "~~षोः", "~~ःष\u{941}|~~ष\u{94d}ष\u{941}";
        vocative: "~~ः", referred, "~ी\u{902}ष~";
    ),
    // युज: 13 words (असृज्, बणिज्, वणिज्, भुज्, भिषज्, उशिज्).
    declension!("य\u{941}ज\u{94d}", 13, "~ज\u{94d}";
        nominative: "~क\u{94d}", "~जौ", "~जः";
        accusative: "~जम\u{94d}", "~जौ", "~जः";
        instrumental: "~जा", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भिः";
        dative: "~ज\u{947}", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        ablative: "~जः", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        genitive: "~जः", "~जोः", "~जाम\u{94d}";
        locative: "~जि", "~जोः", "~क\u{94d}ष\u{941}";
        vocative: "~क\u{94d}", referred, "~जः";
    ),
    // आतमन: 13 words (अश्मन्, तक्मन्, आत्मन्, महात्मन्, वन्, रश्मन्).
    declension!("आत\u{94d}मन\u{94d}", 13, "~न\u{94d}";
        nominative: "~ा", "~ानौ", "~ानः";
        accusative: "~ानम\u{94d}", "~ानौ", "~नः";
        instrumental: "~ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~नः", "~नोः", "~नाम\u{94d}";
        locative: "~नि", "~नोः", "~स\u{941}";
        vocative: "~न\u{94d}", referred, "~ानः";
    ),
    // गरावन: 11 words (ग्रावन्, वृषन्, अर्यमन्, उक्षन्, शीर्षन्, तक्षन्).
    declension!("ग\u{94d}रावन\u{94d}", 11, "~न\u{94d}";
        nominative: "~ा", "~ाणौ", "~ाणः";
        accusative: "~ाणम\u{94d}", "~ाणौ", "~\u{94d}णः";
        instrumental: "~\u{94d}णा", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~\u{94d}ण\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~\u{94d}णः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~\u{94d}णः", "~\u{94d}णोः", "~\u{94d}णाम\u{94d}";
        locative: "~\u{94d}णि|~णि", "~\u{94d}णोः", "~स\u{941}";
        vocative: "~न\u{94d}", referred, "~ाणः";
    ),
    // चमू: 11 words (वधू, तनू, जुहू, निम्बू, जनू, चमू).
    declension!("चम\u{942}", 11, "~\u{942}";
        nominative: "~\u{942}ः", "~\u{94d}वौ", "~\u{94d}वः";
        accusative: "~\u{942}म\u{94d}", "~\u{94d}वौ", "~\u{942}ः";
        instrumental: "~\u{94d}वा", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भिः";
        dative: "~\u{94d}व\u{948}", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भ\u{94d}यः";
        ablative: "~\u{94d}वाः", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भ\u{94d}यः";
        genitive: "~\u{94d}वाः", "~\u{94d}वोः", "~\u{942}नाम\u{94d}";
        locative: "~\u{94d}वाम\u{94d}", "~\u{94d}वोः", "~\u{942}ष\u{941}";
        vocative: "~\u{941}", referred, "~\u{94d}वः";
    ),
    // धी: 10 words (ह्री, स्त्री, धी, श्री, भी, ली).
    declension!("धी", 10, "~ी";
        nominative: "~ीः", "~ियौ", "~ियः";
        accusative: "~ियम\u{94d}", "~ियौ", "~ियः";
        instrumental: "~िया", "~ीभ\u{94d}याम\u{94d}", "~ीभिः";
        dative: "~िय\u{947}", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        ablative: "~ियः", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        genitive: "~ियः", "~ियोः", "~ियाम\u{94d}|~ीनाम\u{94d}";
        locative: "~ियि", "~ियोः", "~ीष\u{941}";
        vocative: "~ीः", referred, "~ियः";
    ),
    // पुंस: 10 words (ऊधस्, उषस्, श्रेयस्, मास्, अप्सरस्, पुंस्).
    declension!("प\u{941}\u{902}स\u{94d}", 10, "~स\u{94d}";
        nominative: "~ाः", "~सौ", "~सः";
        accusative: "~सम\u{94d}", "~सौ", "~सः";
        instrumental: "~सा", "~ोभ\u{94d}याम\u{94d}", "~ोभिः";
        dative: "~स\u{947}", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        ablative: "~सः", "~ोभ\u{94d}याम\u{94d}", "~ोभ\u{94d}यः";
        genitive: "~सः", "~सोः", "~साम\u{94d}";
        locative: "~सि", "~सोः", "~ःस\u{941}|~स\u{94d}स\u{941}";
        vocative: "~ः", referred, "~सः";
    ),
    // तवच: 9 words (त्वच्, ऋच्, रुच्, शुच्, मुच्, स्फिच्).
    declension!("त\u{94d}वच\u{94d}", 9, "~च\u{94d}";
        nominative: "~क\u{94d}", "~चौ", "~चः";
        accusative: "~चम\u{94d}", "~चौ", "~चः";
        instrumental: "~चा", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भिः";
        dative: "~च\u{947}", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        ablative: "~चः", "~ग\u{94d}भ\u{94d}याम\u{94d}", "~ग\u{94d}भ\u{94d}यः";
        genitive: "~चः", "~चोः", "~चाम\u{94d}";
        locative: "~चि", "~चोः", "~क\u{94d}ष\u{941}";
        vocative: "~क\u{94d}", referred, "~चः";
    ),
    // जनमन: 9 words (वस्मन्, वर्त्मन्, जन्मन्, भस्मन्, धन्वन्, छद्मन्).
    declension!("जन\u{94d}मन\u{94d}", 9, "~न\u{94d}";
        nominative: "~", "~नी", "~ानि";
        accusative: "~", "~नी", "~ानि";
        instrumental: "~ना", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~न\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~नः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~नः", "~नोः", "~नाम\u{94d}";
        locative: "~नि", "~नोः", "~स\u{941}";
        vocative: "~न\u{94d}|~", referred, "~ानि";
    ),
    // ऊषमन: 8 words (अथर्वन्, धर्मन्, दर्मन्, भार्मन्, द्राघ्मन्, ऊष्मन्).
    declension!("ऊष\u{94d}मन\u{94d}", 8, "~न\u{94d}";
        nominative: "~ा", "~ाणौ", "~ाणः";
        accusative: "~ाणम\u{94d}", "~ाणौ", "~णः";
        instrumental: "~णा", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~ण\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~णः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~णः", "~णोः", "~णाम\u{94d}";
        locative: "~णि", "~णोः", "~स\u{941}";
        vocative: "~न\u{94d}", referred, "~ाणः";
    ),
    // सपृध: 8 words (प्रबुध्, स्पृध्, क्षुध्, युध्, वीरुध्, श्वाविध्).
    declension!("स\u{94d}प\u{943}ध\u{94d}", 8, "~ध\u{94d}";
        nominative: "~त\u{94d}", "~धौ", "~धः";
        accusative: "~धम\u{94d}", "~धौ", "~धः";
        instrumental: "~धा", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative: "~ध\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative: "~धः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive: "~धः", "~धोः", "~धाम\u{94d}";
        locative: "~धि", "~धोः", "~त\u{94d}स\u{941}";
        vocative: "~त\u{94d}", referred, "~धः";
    ),
    // गोपा: 7 words (जा, पन्था, प्रतिमा, दा, गोपा, रथेष्ठा).
    declension!("गोपा", 7, "~ा";
        nominative: "~ाः", "~ौ", "~ाः";
        accusative: "~ाम\u{94d}", "~ौ", "~ः";
        instrumental: "~ा", "~ाभ\u{94d}याम\u{94d}", "~ाभिः";
        dative: "~\u{947}", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        ablative: "~ः", "~ाभ\u{94d}याम\u{94d}", "~ाभ\u{94d}यः";
        genitive: "~ः", "~ोः", "~ानाम\u{94d}";
        locative: "~ि", "~ोः", "~ास\u{941}";
        vocative: "~ाः", referred, "~ाः";
    ),
    // वारि: 7 words (वारि, अस्थि, अक्षि, इदानि, सेवि, दधि).
    declension!("वारि", 7, "~ि";
        nominative: "~ि", "~िनी", "~ीनि";
        accusative: "~ि", "~िनी", "~ीनि";
        instrumental: "~िना", "~िभ\u{94d}याम\u{94d}", "~िभिः";
        dative: "~िन\u{947}", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        ablative: "~िनः", "~िभ\u{94d}याम\u{94d}", "~िभ\u{94d}यः";
        genitive: "~िनः", "~िनोः", "~ीनाम\u{94d}";
        locative: "~िनि", "~िनोः", "~िष\u{941}";
        vocative: "~ि|~\u{947}", referred, "~ीनि";
    ),
    // भरीमन: 7 words (रोमन्, दरीमन्, भरीमन्, स्तरीमन्, परीमन्, परेमन्).
    declension!("भरीमन\u{94d}", 7, "~न\u{94d}";
        nominative: "~", "~\u{94d}णी|~णी", "~ाणि";
        accusative: "~", "~\u{94d}णी|~णी", "~ाणि";
        instrumental: "~\u{94d}णा", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~\u{94d}ण\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~\u{94d}णः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~\u{94d}णः", "~\u{94d}णोः", "~\u{94d}णाम\u{94d}";
        locative: "~\u{94d}णि|~णि", "~\u{94d}णोः", "~स\u{941}";
        vocative: "~न\u{94d}|~", referred, "~ाणि";
    ),
    // तविष: 6 words (त्विष्, विष्, प्रेष्, प्रावृष्, द्विष्, रुष्).
    declension!("त\u{94d}विष\u{94d}", 6, "~ष\u{94d}";
        nominative: "~ट\u{94d}", "~षौ", "~षः";
        accusative: "~षम\u{94d}", "~षौ", "~षः";
        instrumental: "~षा", "~ड\u{94d}भ\u{94d}याम\u{94d}", "~ड\u{94d}भिः";
        dative: "~ष\u{947}", "~ड\u{94d}भ\u{94d}याम\u{94d}", "~ड\u{94d}भ\u{94d}यः";
        ablative: "~षः", "~ड\u{94d}भ\u{94d}याम\u{94d}", "~ड\u{94d}भ\u{94d}यः";
        genitive: "~षः", "~षोः", "~षाम\u{94d}";
        locative: "~षि", "~षोः", "~ट\u{94d}स\u{941}";
        vocative: "~ट\u{94d}", referred, "~षः";
    ),
    // उसृ: 6 words (मातृ, दुहितृ, यातृ, ननान्दृ, उसृ, विमातृ).
    declension!("उस\u{943}", 6, "~\u{943}";
        nominative: "~ा", "~रौ", "~रः";
        accusative: "~रम\u{94d}", "~रौ", "~\u{944}ः";
        instrumental: "~\u{94d}रा", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भिः";
        dative: "~\u{94d}र\u{947}", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        ablative: "~\u{941}ः", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        genitive: "~\u{941}ः", "~\u{94d}रोः", "~\u{944}णाम\u{94d}";
        locative: "~रि", "~\u{94d}रोः", "~\u{943}ष\u{941}";
        vocative: "~ः", referred, "~रः";
    ),
    // भू: 6 words (भ्रू, भू, स्वपू, सू, जू, मू).
    declension!("भ\u{942}", 6, "~\u{942}";
        nominative: "~\u{942}ः", "~\u{941}वौ", "~\u{941}वः";
        accusative: "~\u{941}वम\u{94d}", "~\u{941}वौ", "~\u{941}वः";
        instrumental: "~\u{941}वा", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भिः";
        dative: "~\u{941}व\u{947}", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भ\u{94d}यः";
        ablative: "~\u{941}वः", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भ\u{94d}यः";
        genitive: "~\u{941}वः", "~\u{941}वोः", "~\u{941}वाम\u{94d}|~\u{942}नाम\u{94d}";
        locative: "~\u{941}वि", "~\u{941}वोः", "~\u{942}ष\u{941}";
        vocative: "~\u{942}ः", referred, "~\u{941}वः";
    ),
    // पितृ: 6 words (भ्रातृ, पितृ, देवृ, नृ, जामातृ, मातापितृ).
    declension!("पित\u{943}", 6, "~\u{943}";
        nominative: "~ा", "~रौ", "~रः";
        accusative: "~रम\u{94d}", "~रौ", "~\u{944}न\u{94d}";
        instrumental: "~\u{94d}रा", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भिः";
        dative: "~\u{94d}र\u{947}", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        ablative: "~\u{941}ः", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        genitive: "~\u{941}ः", "~\u{94d}रोः", "~\u{944}णाम\u{94d}";
        locative: "~रि", "~\u{94d}रोः", "~\u{943}ष\u{941}";
        vocative: "~ः", referred, "~रः";
    ),
    // अरहत: 6 words (दत्, सत्, पृषत्, अर्हत्, सरत्, अर्वत्).
    declension!("अर\u{94d}हत\u{94d}", 6, "~त\u{94d}";
        nominative: "~न\u{94d}", "~न\u{94d}तौ", "~न\u{94d}तः";
        accusative: "~न\u{94d}तम\u{94d}", "~न\u{94d}तौ", "~तः";
        instrumental: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive: "~तः", "~तोः", "~ताम\u{94d}";
        locative: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        vocative: "~न\u{94d}", referred, "~न\u{94d}तः";
    ),
    // चकषुस: 6 words (यजुस्, आयुस्, वपुस्, तरुस्, चक्षुस्, धनुस्).
    declension!("चक\u{94d}ष\u{941}स\u{94d}", 6, "~\u{941}स\u{94d}";
        nominative: "~\u{941}ः", "~\u{941}षी", "~\u{942}\u{902}षि";
        accusative: "~\u{941}ः", "~\u{941}षी", "~\u{942}\u{902}षि";
        instrumental: "~\u{941}षा", "~\u{941}र\u{94d}भ\u{94d}याम\u{94d}", "~\u{941}र\u{94d}भिः";
        dative: "~\u{941}ष\u{947}", "~\u{941}र\u{94d}भ\u{94d}याम\u{94d}", "~\u{941}र\u{94d}भ\u{94d}यः";
        ablative: "~\u{941}षः", "~\u{941}र\u{94d}भ\u{94d}याम\u{94d}", "~\u{941}र\u{94d}भ\u{94d}यः";
        genitive: "~\u{941}षः", "~\u{941}षोः", "~\u{941}षाम\u{94d}";
        locative: "~\u{941}षि", "~\u{941}षोः", "~\u{941}ःष\u{941}|~\u{941}ष\u{94d}ष\u{941}";
        vocative: "~\u{941}ः", referred, "~\u{942}\u{902}षि";
    ),
    // महत: 6 words (हिमवत्, महत्, परस्वत्, श्रीमत्, इरावत्, बलवत्).
    declension!("महत\u{94d}", 6, "~त\u{94d}";
        nominative: "~ान\u{94d}", "~न\u{94d}तौ", "~न\u{94d}तः";
        accusative: "~न\u{94d}तम\u{94d}", "~न\u{94d}तौ", "~तः";
        instrumental: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive: "~तः", "~तोः", "~ताम\u{94d}";
        locative: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        vocative: "~न\u{94d}", referred, "~न\u{94d}तः";
    ),
    // तृ: 5 words (स्वसृ, स्तृ, तृ, पितृष्वसृ, पितुःष्वसृ).
    declension!("त\u{943}", 5, "~\u{943}";
        nominative: "~ा", "~ारौ", "~ारः";
        accusative: "~ारम\u{94d}", "~ारौ", "~\u{944}ः";
        instrumental: "~\u{94d}रा", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भिः";
        dative: "~\u{94d}र\u{947}", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        ablative: "~\u{941}ः", "~\u{943}भ\u{94d}याम\u{94d}", "~\u{943}भ\u{94d}यः";
        genitive: "~\u{941}ः", "~\u{94d}रोः", "~\u{944}णाम\u{94d}";
        locative: "~रि", "~\u{94d}रोः", "~\u{943}ष\u{941}";
        vocative: "~ः", referred, "~ारः";
    ),
    // दारु: 5 words (अश्रु, दारु, त्रपु, श्मश्रु, क्षु).
    declension!("दार\u{941}", 5, "~\u{941}";
        nominative: "~\u{941}", "~\u{941}णी", "~\u{942}णि";
        accusative: "~\u{941}", "~\u{941}णी", "~\u{942}णि";
        instrumental: "~\u{941}णा", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भिः";
        dative: "~\u{941}ण\u{947}", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        ablative: "~\u{941}णः", "~\u{941}भ\u{94d}याम\u{94d}", "~\u{941}भ\u{94d}यः";
        genitive: "~\u{941}णः", "~\u{941}णोः", "~\u{942}णाम\u{94d}";
        locative: "~\u{941}णि", "~\u{941}णोः", "~\u{941}ष\u{941}";
        vocative: "~\u{941}|~ो", referred, "~\u{942}णि";
    ),
    // पितृभू: 5 words (श्वश्रू, अग्रू, वर्षाहू, पितृभू, उर्वारू).
    declension!("पित\u{943}भ\u{942}", 5, "~\u{942}";
        nominative: "~\u{942}ः", "~\u{94d}वौ", "~\u{94d}वः";
        accusative: "~\u{942}म\u{94d}", "~\u{94d}वौ", "~\u{942}ः";
        instrumental: "~\u{94d}वा", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भिः";
        dative: "~\u{94d}व\u{948}", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भ\u{94d}यः";
        ablative: "~\u{94d}वाः", "~\u{942}भ\u{94d}याम\u{94d}", "~\u{942}भ\u{94d}यः";
        genitive: "~\u{94d}वाः", "~\u{94d}वोः", "~\u{942}णाम\u{94d}";
        locative: "~\u{94d}वाम\u{94d}", "~\u{94d}वोः", "~\u{942}ष\u{941}";
        vocative: "~\u{941}", referred, "~\u{94d}वः";
    ),
    // जगत: 4 words (जगत्, यकृत्, बृहत्, असत्).
    declension!("जगत\u{94d}", 4, "~त\u{94d}";
        nominative: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        accusative: "~त\u{94d}", "~ती", "~न\u{94d}ति";
        instrumental: "~ता", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भिः";
        dative: "~त\u{947}", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        ablative: "~तः", "~द\u{94d}भ\u{94d}याम\u{94d}", "~द\u{94d}भ\u{94d}यः";
        genitive: "~तः", "~तोः", "~ताम\u{94d}";
        locative: "~ति", "~तोः", "~त\u{94d}स\u{941}";
        vocative: "~त\u{94d}", referred, "~न\u{94d}ति";
    ),
    // राडराजन: 4 words (राजन्, मज्जन्, महाराजन्, राड्राजन्).
    declension!("राड\u{94d}राजन\u{94d}", 4, "~न\u{94d}";
        nominative: "~ा", "~ानौ", "~ानः";
        accusative: "~ानम\u{94d}", "~ानौ", "~\u{94d}ञः";
        instrumental: "~\u{94d}ञा", "~भ\u{94d}याम\u{94d}", "~भिः";
        dative: "~\u{94d}ञ\u{947}", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        ablative: "~\u{94d}ञः", "~भ\u{94d}याम\u{94d}", "~भ\u{94d}यः";
        genitive: "~\u{94d}ञः", "~\u{94d}ञोः", "~\u{94d}ञाम\u{94d}";
        locative: "~\u{94d}ञि|~नि", "~\u{94d}ञोः", "~स\u{941}";
        vocative: "~न\u{94d}", referred, "~ानः";
    ),
    // रिप: 4 words (अप्, क्षप्, कृप्, रिप्).
    declension!("रिप\u{94d}", 4, "~प\u{94d}";
        nominative: "~प\u{94d}", "~पौ", "~पः";
        accusative: "~पम\u{94d}", "~पौ", "~पः";
        instrumental: "~पा", "~ब\u{94d}भ\u{94d}याम\u{94d}", "~ब\u{94d}भिः";
        dative: "~प\u{947}", "~ब\u{94d}भ\u{94d}याम\u{94d}", "~ब\u{94d}भ\u{94d}यः";
        ablative: "~पः", "~ब\u{94d}भ\u{94d}याम\u{94d}", "~ब\u{94d}भ\u{94d}यः";
        genitive: "~पः", "~पोः", "~पाम\u{94d}";
        locative: "~पि", "~पोः", "~प\u{94d}स\u{941}";
        vocative: "~प\u{94d}", referred, "~पः";
    ),
    // शुभ: 4 words (गृभ्, क्षुभ्, शुभ्, अनुष्टुभ्).
    declension!("श\u{941}भ\u{94d}", 4, "~भ\u{94d}";
        nominative: "~प\u{94d}", "~भौ", "~भः";
        accusative: "~भम\u{94d}", "~भौ", "~भः";
        instrumental: "~भा", "~ब\u{94d}भ\u{94d}याम\u{94d}", "~ब\u{94d}भिः";
        dative: "~भ\u{947}", "~ब\u{94d}भ\u{94d}याम\u{94d}", "~ब\u{94d}भ\u{94d}यः";
        ablative: "~भः", "~ब\u{94d}भ\u{94d}याम\u{94d}", "~ब\u{94d}भ\u{94d}यः";
        genitive: "~भः", "~भोः", "~भाम\u{94d}";
        locative: "~भि", "~भोः", "~प\u{94d}स\u{941}";
        vocative: "~प\u{94d}", referred, "~भः";
    ),
    // गलौ: 3 words (नौ, ज्यौ, ग्लौ).
    declension!("ग\u{94d}लौ", 3, "~ौ";
        nominative: "~ौः", "~ावौ", "~ावः";
        accusative: "~ावम\u{94d}", "~ावौ", "~ावः";
        instrumental: "~ावा", "~ौभ\u{94d}याम\u{94d}", "~ौभिः";
        dative: "~ाव\u{947}", "~ौभ\u{94d}याम\u{94d}", "~ौभ\u{94d}यः";
        ablative: "~ावः", "~ौभ\u{94d}याम\u{94d}", "~ौभ\u{94d}यः";
        genitive: "~ावः", "~ावोः", "~ावाम\u{94d}";
        locative: "~ावि", "~ावोः", "~ौष\u{941}";
        vocative: "~ौः", referred, "~ावः";
    ),
    // लकषमी: 3 words (लक्ष्मी, तन्त्री, तन्द्री).
    declension!("लक\u{94d}ष\u{94d}मी", 3, "~ी";
        nominative: "~ीः|~ी", "~\u{94d}यौ", "~\u{94d}यः";
        accusative: "~ीम\u{94d}", "~\u{94d}यौ", "~ीः";
        instrumental: "~\u{94d}या", "~ीभ\u{94d}याम\u{94d}", "~ीभिः";
        dative: "~\u{94d}य\u{948}", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        ablative: "~\u{94d}याः", "~ीभ\u{94d}याम\u{94d}", "~ीभ\u{94d}यः";
        genitive: "~\u{94d}याः", "~\u{94d}योः", "~ीणाम\u{94d}";
        locative: "~\u{94d}याम\u{94d}", "~\u{94d}योः", "~ीष\u{941}";
        vocative: "~ि", referred, "~\u{94d}यः";
    ),
    // विराज: 3 words (विराज्, राज्, सम्राज्).
    declension!("विराज\u{94d}", 3, "~ज\u{94d}";
        nominative: "~ट\u{94d}", "~जौ", "~जः";
        accusative: "~जम\u{94d}", "~जौ", "~जः";
        instrumental: "~जा", "~ड\u{94d}भ\u{94d}याम\u{94d}", "~ड\u{94d}भिः";
        dative: "~ज\u{947}", "~ड\u{94d}भ\u{94d}याम\u{94d}", "~ड\u{94d}भ\u{94d}यः";
        ablative: "~जः", "~ड\u{94d}भ\u{94d}याम\u{94d}", "~ड\u{94d}भ\u{94d}यः";
        genitive: "~जः", "~जोः", "~जाम\u{94d}";
        locative: "~जि", "~जोः", "~ट\u{94d}स\u{941}";
        vocative: "~ट\u{94d}", referred, "~जः";
    ),
];
