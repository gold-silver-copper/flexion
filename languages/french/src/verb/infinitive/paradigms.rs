//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `present_second` (pl)
/// `present_subjunctive_first` (pl)
/// `present_subjunctive_second` (pl)
/// `imperative` (vous)
/// `infinitive`
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     present_second: $c0:tt;
     present_subjunctive_first: $c1:tt;
     present_subjunctive_second: $c2:tt;
     imperative: $c3:tt;
     infinitive: $c4:tt;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `passer`: 6,515 words.
    Passer,
    /// Like `établir`: 333 words.
    Établir,
    /// Like `rendre`: 103 words.
    Rendre,
    /// Like `tenir`: 91 words.
    Tenir,
    /// Like `dire`: 64 words.
    Dire,
    /// Like `devoir`: 31 words.
    Devoir,
    /// Like `poindre`: 30 words.
    Poindre,
    /// Like `connaître`: 21 words.
    Connaître,
    /// Like `naitre`: 19 words.
    Naitre,
    /// Like `inscrire`: 15 words.
    Inscrire,
    /// Like `croire`: 14 words.
    Croire,
    /// Like `faire`: 12 words.
    Faire,
    /// Like `prendre`: 12 words.
    Prendre,
    /// Like `prévoir`: 11 words.
    Prévoir,
    /// Like `dissoudre`: 6 words.
    Dissoudre,
    /// Like `avoir`: 4 words.
    Avoir,
    /// Like `moudre`: 4 words.
    Moudre,
    /// Like `ouïr`: 4 words.
    Ouïr,
    /// Like `emboire`: 3 words.
    Emboire,
    /// Like `recoudre`: 3 words.
    Recoudre,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 20] = [
        Conjugation::Passer,
        Conjugation::Établir,
        Conjugation::Rendre,
        Conjugation::Tenir,
        Conjugation::Dire,
        Conjugation::Devoir,
        Conjugation::Poindre,
        Conjugation::Connaître,
        Conjugation::Naitre,
        Conjugation::Inscrire,
        Conjugation::Croire,
        Conjugation::Faire,
        Conjugation::Prendre,
        Conjugation::Prévoir,
        Conjugation::Dissoudre,
        Conjugation::Avoir,
        Conjugation::Moudre,
        Conjugation::Ouïr,
        Conjugation::Emboire,
        Conjugation::Recoudre,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 20] = [
    // Passer: 6,515 words (abhorrer, abjurer, absconder, absenter, absorber, abuser).
    conjugation!("passer", 6515, "~er";
        present_second: "~ez";
        present_subjunctive_first: "~ions";
        present_subjunctive_second: "~iez";
        imperative: "~ez";
        infinitive: "~er";
    ),
    // Établir: 333 words (polir, abolir, haïr, tapir, approfondir, finir).
    conjugation!("établir", 333, "~r";
        present_second: "~ssez";
        present_subjunctive_first: "~ssions";
        present_subjunctive_second: "~ssiez";
        imperative: "~ssez";
        infinitive: "~r";
    ),
    // Rendre: 103 words (être, remettre, vivre, mettre, vendre, foutre).
    conjugation!("rendre", 103, "~re";
        present_second: "~ez";
        present_subjunctive_first: "~ions";
        present_subjunctive_second: "~iez";
        imperative: "~ez";
        infinitive: "~re";
    ),
    // Tenir: 91 words (souvenir, dormir, partir, mentir, sentir, mourir).
    conjugation!("tenir", 91, "~ir";
        present_second: "~ez";
        present_subjunctive_first: "~ions";
        present_subjunctive_second: "~iez";
        imperative: "~ez";
        infinitive: "~ir";
    ),
    // Dire: 64 words (lire, plaire, dire, construire, cuire, contredire).
    conjugation!("dire", 64, "~re";
        present_second: "~sez";
        present_subjunctive_first: "~sions";
        present_subjunctive_second: "~siez";
        imperative: "~sez";
        infinitive: "~re";
    ),
    // Devoir: 31 words (pleuvoir, pouvoir, vouloir, devoir, savoir, valoir).
    conjugation!("devoir", 31, "~oir";
        present_second: "~ez";
        present_subjunctive_first: "~ions";
        present_subjunctive_second: "~iez";
        imperative: "~ez";
        infinitive: "~oir";
    ),
    // Poindre: 30 words (plaindre, éteindre, joindre, craindre, peindre, atteindre).
    conjugation!("poindre", 30, "~~dre";
        present_second: "~g~ez";
        present_subjunctive_first: "~g~ions";
        present_subjunctive_second: "~g~iez";
        imperative: "~g~ez";
        infinitive: "~~dre";
    ),
    // Connaître: 21 words (naître, renaître, connaître, croître, reconnaître, paraître).
    conjugation!("connaître", 21, "~ître";
        present_second: "~issez";
        present_subjunctive_first: "~issions";
        present_subjunctive_second: "~issiez";
        imperative: "~issez";
        infinitive: "~ître";
    ),
    // Naitre: 19 words (naitre, reconnaitre, paraitre, disparaitre, connaitre, croitre).
    conjugation!("naitre", 19, "~tre";
        present_second: "~ssez";
        present_subjunctive_first: "~ssions";
        present_subjunctive_second: "~ssiez";
        imperative: "~ssez";
        infinitive: "~tre";
    ),
    // Inscrire: 15 words (écrire, décrire, prescrire, récrire, souscrire, inscrire).
    conjugation!("inscrire", 15, "~re";
        present_second: "~vez";
        present_subjunctive_first: "~vions";
        present_subjunctive_second: "~viez";
        imperative: "~vez";
        infinitive: "~re";
    ),
    // Croire: 14 words (traire, croire, distraire, soustraire, abstraire, extraire).
    conjugation!("croire", 14, "~ire";
        present_second: "~yez";
        present_subjunctive_first: "~yions";
        present_subjunctive_second: "~yiez";
        imperative: "~yez";
        infinitive: "~ire";
    ),
    // Faire: 12 words (faire, refaire, contrefaire, défaire, satisfaire, parfaire).
    conjugation!("faire", 12, "~~re";
        present_second: "~~tes";
        present_subjunctive_first: "~ss~ons";
        present_subjunctive_second: "~ss~ez";
        imperative: "~~tes";
        infinitive: "~~re";
    ),
    // Prendre: 12 words (prendre, comprendre, apprendre, surprendre, reprendre, entreprendre).
    conjugation!("prendre", 12, "~dre";
        present_second: "~ez";
        present_subjunctive_first: "~ions";
        present_subjunctive_second: "~iez";
        imperative: "~ez";
        infinitive: "~dre";
    ),
    // Prévoir: 11 words (choir, voir, fuir, revoir, pourvoir, prévoir).
    conjugation!("prévoir", 11, "~ir";
        present_second: "~yez";
        present_subjunctive_first: "~yions";
        present_subjunctive_second: "~yiez";
        imperative: "~yez";
        infinitive: "~ir";
    ),
    // Dissoudre: 6 words (résoudre, absoudre, dissoudre, autodissoudre, resoudre, arsoudre).
    conjugation!("dissoudre", 6, "~udre";
        present_second: "~lvez";
        present_subjunctive_first: "~lvions";
        present_subjunctive_second: "~lviez";
        imperative: "~lvez";
        infinitive: "~udre";
    ),
    // Avoir: 4 words (avoir, ravoir, havoir, réavoir).
    conjugation!("avoir", 4, "~voir";
        present_second: "~vez";
        present_subjunctive_first: "~yons";
        present_subjunctive_second: "~yez";
        imperative: "~yez";
        infinitive: "~voir";
    ),
    // Moudre: 4 words (moudre, remoudre, émoudre, vermoudre).
    conjugation!("moudre", 4, "~dre";
        present_second: "~lez";
        present_subjunctive_first: "~lions";
        present_subjunctive_second: "~liez";
        imperative: "~lez";
        infinitive: "~dre";
    ),
    // Ouïr: 4 words (ouïr, amuïr, réouïr, inouïr).
    conjugation!("ouïr", 4, "~uïr";
        present_second: "~uïssez|~yez";
        present_subjunctive_first: "~uïssions|~yions";
        present_subjunctive_second: "~uïssiez|~yiez";
        imperative: "~uïssez|~yez";
        infinitive: "~uïr";
    ),
    // Emboire: 3 words (boire, reboire, emboire).
    conjugation!("emboire", 3, "~oire";
        present_second: "~uvez";
        present_subjunctive_first: "~uvions";
        present_subjunctive_second: "~uviez";
        imperative: "~uvez";
        infinitive: "~oire";
    ),
    // Recoudre: 3 words (coudre, recoudre, découdre).
    conjugation!("recoudre", 3, "~dre";
        present_second: "~sez";
        present_subjunctive_first: "~sions";
        present_subjunctive_second: "~siez";
        imperative: "~sez";
        infinitive: "~dre";
    ),
];
