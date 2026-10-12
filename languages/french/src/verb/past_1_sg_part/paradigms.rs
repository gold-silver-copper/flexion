//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `past_historic_first` (sg)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     past_historic_first: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `passer`: 6,178 words.
    Passer,
    /// Like `ouvrir`: 380 words.
    Ouvrir,
    /// Like `arranger`: 193 words.
    Arranger,
    /// Like `amorcer`: 124 words.
    Amorcer,
    /// Like `rendre`: 78 words.
    Rendre,
    /// Like `réduire`: 33 words.
    Réduire,
    /// Like `poindre`: 30 words.
    Poindre,
    /// Like `tenir`: 29 words.
    Tenir,
    /// Like `dire`: 25 words.
    Dire,
    /// Like `remettre`: 18 words.
    Remettre,
    /// Like `inscrire`: 15 words.
    Inscrire,
    /// Like `pourvoir`: 12 words.
    Pourvoir,
    /// Like `faire`: 12 words.
    Faire,
    /// Like `prendre`: 12 words.
    Prendre,
    /// Like `soustraire`: 11 words.
    Soustraire,
    /// Like `connaître`: 11 words.
    Connaître,
    /// Like `paraitre`: 11 words.
    Paraitre,
    /// Like `courir`: 10 words.
    Courir,
    /// Like `concevoir`: 7 words.
    Concevoir,
    /// Like `croire`: 6 words.
    Croire,
    /// Like `pouvoir`: 6 words.
    Pouvoir,
    /// Like `requérir`: 6 words.
    Requérir,
    /// Like `dissoudre`: 6 words.
    Dissoudre,
    /// Like `resver`: 6 words.
    Resver,
    /// Like `prévoir`: 5 words.
    Prévoir,
    /// Like `lire`: 4 words.
    Lire,
    /// Like `taire`: 4 words.
    Taire,
    /// Like `moudre`: 4 words.
    Moudre,
    /// Like `seoir`: 4 words.
    Seoir,
    /// Like `accroître`: 4 words.
    Accroître,
    /// Like `survivre`: 3 words.
    Survivre,
    /// Like `devoir`: 3 words.
    Devoir,
    /// Like `recoudre`: 3 words.
    Recoudre,
    /// Like `accroitre`: 3 words.
    Accroitre,
    /// Like `havoir`: 3 words.
    Havoir,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 35] = [
        Conjugation::Passer,
        Conjugation::Ouvrir,
        Conjugation::Arranger,
        Conjugation::Amorcer,
        Conjugation::Rendre,
        Conjugation::Réduire,
        Conjugation::Poindre,
        Conjugation::Tenir,
        Conjugation::Dire,
        Conjugation::Remettre,
        Conjugation::Inscrire,
        Conjugation::Pourvoir,
        Conjugation::Faire,
        Conjugation::Prendre,
        Conjugation::Soustraire,
        Conjugation::Connaître,
        Conjugation::Paraitre,
        Conjugation::Courir,
        Conjugation::Concevoir,
        Conjugation::Croire,
        Conjugation::Pouvoir,
        Conjugation::Requérir,
        Conjugation::Dissoudre,
        Conjugation::Resver,
        Conjugation::Prévoir,
        Conjugation::Lire,
        Conjugation::Taire,
        Conjugation::Moudre,
        Conjugation::Seoir,
        Conjugation::Accroître,
        Conjugation::Survivre,
        Conjugation::Devoir,
        Conjugation::Recoudre,
        Conjugation::Accroitre,
        Conjugation::Havoir,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 35] = [
    // Passer: 6,178 words (abhorrer, abjurer, absconder, absenter, absorber, abuser).
    conjugation!("passer", 6178, "~er";
        past_historic_first: "~ai";
    ),
    // Ouvrir: 380 words (avoir, dormir, partir, polir, abolir, mentir).
    conjugation!("ouvrir", 380, "~r";
        past_historic_first: "~s";
    ),
    // Arranger: 193 words (badger, voyager, manager, singer, manger, bouger).
    conjugation!("arranger", 193, "~r";
        past_historic_first: "~ai";
    ),
    // Amorcer: 124 words (tracer, placer, agacer, forcer, commencer, coercer).
    conjugation!("amorcer", 124, "~cer";
        past_historic_first: "~çai";
    ),
    // Rendre: 78 words (être, naître, renaître, vendre, foutre, mordre).
    conjugation!("rendre", 78, "~re";
        past_historic_first: "~is";
    ),
    // Réduire: 33 words (construire, cuire, traduire, produire, détruire, conduire).
    conjugation!("réduire", 33, "~re";
        past_historic_first: "~sis";
    ),
    // Poindre: 30 words (plaindre, éteindre, joindre, craindre, peindre, atteindre).
    conjugation!("poindre", 30, "~~dre";
        past_historic_first: "~g~is";
    ),
    // Tenir: 29 words (souvenir, devenir, venir, obtenir, tenir, intervenir).
    conjugation!("tenir", 29, "~en~r";
        past_historic_first: "~~ns";
    ),
    // Dire: 25 words (dire, contredire, rire, redire, inclure, sourire).
    conjugation!("dire", 25, "~re";
        past_historic_first: "~s";
    ),
    // Remettre: 18 words (remettre, mettre, émettre, promettre, permettre, commettre).
    conjugation!("remettre", 18, "~ettre";
        past_historic_first: "~is";
    ),
    // Inscrire: 15 words (écrire, décrire, prescrire, récrire, souscrire, inscrire).
    conjugation!("inscrire", 15, "~re";
        past_historic_first: "~vis";
    ),
    // Pourvoir: 12 words (choir, vouloir, valoir, pourvoir, déchoir, prévaloir).
    conjugation!("pourvoir", 12, "~oir";
        past_historic_first: "~us";
    ),
    // Faire: 12 words (faire, refaire, contrefaire, défaire, satisfaire, parfaire).
    conjugation!("faire", 12, "~a~re";
        past_historic_first: "~~s";
    ),
    // Prendre: 12 words (prendre, comprendre, apprendre, surprendre, reprendre, entreprendre).
    conjugation!("prendre", 12, "~endre";
        past_historic_first: "~is";
    ),
    // Soustraire: 11 words (traire, distraire, soustraire, abstraire, extraire, braire).
    conjugation!("soustraire", 11, "~~re";
        past_historic_first: "~ya~";
    ),
    // Connaître: 11 words (connaître, reconnaître, paraître, disparaître, apparaître, comparaître).
    conjugation!("connaître", 11, "~aître";
        past_historic_first: "~us";
    ),
    // Paraitre: 11 words (reconnaitre, paraitre, disparaitre, connaitre, apparaitre, comparaitre).
    conjugation!("paraitre", 11, "~aitre";
        past_historic_first: "~us";
    ),
    // Courir: 10 words (mourir, courir, secourir, recourir, accourir, encourir).
    conjugation!("courir", 10, "~ir";
        past_historic_first: "~us";
    ),
    // Concevoir: 7 words (recevoir, apercevoir, décevoir, concevoir, percevoir, repercevoir).
    conjugation!("concevoir", 7, "~cevoir";
        past_historic_first: "~çus";
    ),
    // Croire: 6 words (boire, croire, reboire, recroire, mécroire, emboire).
    conjugation!("croire", 6, "~oire";
        past_historic_first: "~us";
    ),
    // Pouvoir: 6 words (pouvoir, mouvoir, émouvoir, promouvoir, repouvoir, remouvoir).
    conjugation!("pouvoir", 6, "~o~voir";
        past_historic_first: "~~s";
    ),
    // Requérir: 6 words (requérir, acquérir, conquérir, enquérir, reconquérir, quérir).
    conjugation!("requérir", 6, "~ér~r";
        past_historic_first: "~~s";
    ),
    // Dissoudre: 6 words (résoudre, absoudre, dissoudre, autodissoudre, resoudre, arsoudre).
    conjugation!("dissoudre", 6, "~~dre";
        past_historic_first: "~l~s";
    ),
    // Resver: 6 words (resver, esloigner, brusler, esposer, gascher, ruisseller).
    conjugation!("resver", 6, "~er";
        past_historic_first: "~oi";
    ),
    // Prévoir: 5 words (voir, soir, revoir, prévoir, entrevoir).
    conjugation!("prévoir", 5, "~o~r";
        past_historic_first: "~~s";
    ),
    // Lire: 4 words (lire, relire, élire, réélire).
    conjugation!("lire", 4, "~ire";
        past_historic_first: "~us";
    ),
    // Taire: 4 words (plaire, taire, déplaire, complaire).
    conjugation!("taire", 4, "~aire";
        past_historic_first: "~us";
    ),
    // Moudre: 4 words (moudre, remoudre, émoudre, vermoudre).
    conjugation!("moudre", 4, "~dre";
        past_historic_first: "~lus";
    ),
    // Seoir: 4 words (seoir, asseoir, rasseoir, surseoir).
    conjugation!("seoir", 4, "~eo~r";
        past_historic_first: "~~s";
    ),
    // Accroître: 4 words (décroître, connoître, accroître, reconnoître).
    conjugation!("accroître", 4, "~oître";
        past_historic_first: "~us";
    ),
    // Survivre: 3 words (vivre, survivre, revivre).
    conjugation!("survivre", 3, "~ivre";
        past_historic_first: "~écus";
    ),
    // Devoir: 3 words (devoir, redevoir, ramentevoir).
    conjugation!("devoir", 3, "~evoir";
        past_historic_first: "~us";
    ),
    // Recoudre: 3 words (coudre, recoudre, découdre).
    conjugation!("recoudre", 3, "~dre";
        past_historic_first: "~sis";
    ),
    // Accroitre: 3 words (décroitre, accroitre, recroitre).
    conjugation!("accroitre", 3, "~oitre";
        past_historic_first: "~us";
    ),
    // Havoir: 3 words (ravoir, havoir, réavoir).
    conjugation!("havoir", 3, "~avoir";
        past_historic_first: "~eus";
    ),
];
