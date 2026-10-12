//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `non_past_first` (pl)
/// `non_past_second` (sg, pl)
/// `non_past_third` (sg)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     non_past_first: $c0:tt;
     non_past_second: $c1:tt, $c2:tt;
     non_past_third: $c3:tt;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `ciągniesz`: 11,857 words.
    Ciągniesz,
    /// Like `rozklekoczesz`: 56 words.
    Rozklekoczesz,
    /// Like `chlasz`: 20 words.
    Chlasz,
    /// Like `kołaczesz`: 12 words.
    Kołaczesz,
    /// Like `zrównujesz`: 8 words.
    Zrównujesz,
    /// Like `tkasz`: 6 words.
    Tkasz,
    /// Like `zaorzesz`: 6 words.
    Zaorzesz,
    /// Like `chlaszczesz`: 6 words.
    Chlaszczesz,
    /// Like `podajesz`: 4 words.
    Podajesz,
    /// Like `haratasz`: 4 words.
    Haratasz,
    /// Like `biegniesz`: 3 words.
    Biegniesz,
    /// Like `zwiesz`: 3 words.
    Zwiesz,
    /// Like `zaprzysiężesz`: 3 words.
    Zaprzysiężesz,
    /// Like `jesteś`: 1 words.
    Jesteś,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 14] = [
        Conjugation::Ciągniesz,
        Conjugation::Rozklekoczesz,
        Conjugation::Chlasz,
        Conjugation::Kołaczesz,
        Conjugation::Zrównujesz,
        Conjugation::Tkasz,
        Conjugation::Zaorzesz,
        Conjugation::Chlaszczesz,
        Conjugation::Podajesz,
        Conjugation::Haratasz,
        Conjugation::Biegniesz,
        Conjugation::Zwiesz,
        Conjugation::Zaprzysiężesz,
        Conjugation::Jesteś,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 14] = [
    // Ciągniesz: 11,857 words (wymagasz, wymożesz, googlujesz, pieprzysz, mulczujesz, śpiewasz).
    conjugation!("ciągniesz", 11857, "~sz";
        non_past_first: "~my";
        non_past_second: "~sz", "~cie";
        non_past_third: "~";
    ),
    // Rozklekoczesz: 56 words (bełkoczesz, trzepoczesz, chichoczesz, dygoczesz, burkoczesz, łaskoczesz).
    conjugation!("rozklekoczesz", 56, "~z~sz";
        non_past_first: "~z~my|~~my";
        non_past_second: "~z~sz|~~sz", "~z~cie|~~cie";
        non_past_third: "~z~|~~";
    ),
    // Chlasz: 20 words (bajasz, chlasz, szczasz, kajasz, guzdrasz, przemyśliwasz).
    conjugation!("chlasz", 20, "~asz";
        non_past_first: "~amy";
        non_past_second: "~asz", "~acie";
        non_past_third: "~a";
    ),
    // Kołaczesz: 12 words (szepczesz, chłepczesz, plączesz, depczesz, drepczesz, łechczesz).
    conjugation!("kołaczesz", 12, "~czesz";
        non_past_first: "~czemy|~cemy|~tamy";
        non_past_second: "~czesz|~cesz|~tasz", "~czecie|~cecie|~tacie";
        non_past_third: "~cze|~ce|~ta";
    ),
    // Zrównujesz: 8 words (porównujesz, wykonujesz, dorównujesz, przyrównujesz, wyrównujesz, zrównujesz).
    conjugation!("zrównujesz", 8, "~ujesz";
        non_past_first: "~ujemy|~ywamy";
        non_past_second: "~ujesz|~ywasz", "~ujecie|~ywacie";
        non_past_third: "~uje|~ywa";
    ),
    // Tkasz: 6 words (tkasz, iskasz, mlaskasz, kwokasz, pluskasz, oklaskasz).
    conjugation!("tkasz", 6, "~kasz";
        non_past_first: "~kamy|~zczemy";
        non_past_second: "~kasz|~zczesz", "~kacie|~zczecie";
        non_past_third: "~ka|~zcze";
    ),
    // Zaorzesz: 6 words (orzesz, zaorzesz, babrzesz, zakaszesz, worzesz, poorzesz).
    conjugation!("zaorzesz", 6, "~zesz";
        non_past_first: "~zemy|~amy";
        non_past_second: "~zesz|~asz", "~zecie|~acie";
        non_past_third: "~ze|~a";
    ),
    // Chlaszczesz: 6 words (głaszczesz, klaszczesz, chłoszczesz, chlaszczesz, wiszczesz, wychłoszczesz).
    conjugation!("chlaszczesz", 6, "~zczesz";
        non_past_first: "~zczemy";
        non_past_second: "~zczesz", "~zczecie";
        non_past_third: "~zcze";
    ),
    // Podajesz: 4 words (stajesz, podajesz, gorejesz, nastajesz).
    conjugation!("podajesz", 4, "~jesz";
        non_past_first: "~jemy|~wamy";
        non_past_second: "~jesz|~wasz", "~jecie|~wacie";
        non_past_third: "~je|~wa";
    ),
    // Haratasz: 4 words (bełtasz, chlustasz, zbełtasz, haratasz).
    conjugation!("haratasz", 4, "~tasz";
        non_past_first: "~tamy";
        non_past_second: "~tasz", "~tacie";
        non_past_third: "~ta";
    ),
    // Biegniesz: 3 words (biegniesz, lęgniesz, poprzysięgniesz).
    conjugation!("biegniesz", 3, "~gni~sz";
        non_past_first: "~gni~my|~ż~my";
        non_past_second: "~gni~sz|~ż~sz", "~gni~cie|~ż~cie";
        non_past_third: "~gni~|~ż~";
    ),
    // Zwiesz: 3 words (zwiesz, druzgocesz, zdruzgocesz).
    conjugation!("zwiesz", 3, "~~sz";
        non_past_first: "~~my|~z~my";
        non_past_second: "~~sz|~z~sz", "~~cie|~z~cie";
        non_past_third: "~~|~z~";
    ),
    // Zaprzysiężesz: 3 words (zaprzężesz, zaprzysiężesz, zalężesz).
    conjugation!("zaprzysiężesz", 3, "~ż~sz";
        non_past_first: "~ż~my|~gni~my";
        non_past_second: "~ż~sz|~gni~sz", "~ż~cie|~gni~cie";
        non_past_third: "~ż~|~gni~";
    ),
    // Jesteś: 1 words (jesteś).
    conjugation!("jesteś", 1, "j~steś";
        non_past_first: "j~steśmy|będzi~my";
        non_past_second: "j~steś|będzi~sz", "j~steście|będzi~cie";
        non_past_third: "j~st|będzi~";
    ),
];
