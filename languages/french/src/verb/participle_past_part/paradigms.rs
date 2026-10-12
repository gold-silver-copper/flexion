//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `participle` (past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     participle: $c0:tt;
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
    /// Like `passer`: 6,514 words.
    Passer,
    /// Like `sentir`: 375 words.
    Sentir,
    /// Like `dire`: 92 words.
    Dire,
    /// Like `rendre`: 66 words.
    Rendre,
    /// Like `tenir`: 45 words.
    Tenir,
    /// Like `poindre`: 30 words.
    Poindre,
    /// Like `prévoir`: 18 words.
    Prévoir,
    /// Like `remettre`: 18 words.
    Remettre,
    /// Like `connaître`: 13 words.
    Connaître,
    /// Like `paitre`: 13 words.
    Paitre,
    /// Like `prendre`: 12 words.
    Prendre,
    /// Like `bruire`: 11 words.
    Bruire,
    /// Like `éclore`: 11 words.
    Éclore,
    /// Like `ouvrir`: 10 words.
    Ouvrir,
    /// Like `concevoir`: 7 words.
    Concevoir,
    /// Like `croire`: 6 words.
    Croire,
    /// Like `requérir`: 6 words.
    Requérir,
    /// Like `dissoudre`: 6 words.
    Dissoudre,
    /// Like `pouvoir`: 5 words.
    Pouvoir,
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
    /// Like `poursuivre`: 3 words.
    Poursuivre,
    /// Like `recoudre`: 3 words.
    Recoudre,
    /// Like `accroitre`: 3 words.
    Accroitre,
    /// Like `havoir`: 3 words.
    Havoir,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 29] = [
        Conjugation::Passer,
        Conjugation::Sentir,
        Conjugation::Dire,
        Conjugation::Rendre,
        Conjugation::Tenir,
        Conjugation::Poindre,
        Conjugation::Prévoir,
        Conjugation::Remettre,
        Conjugation::Connaître,
        Conjugation::Paitre,
        Conjugation::Prendre,
        Conjugation::Bruire,
        Conjugation::Éclore,
        Conjugation::Ouvrir,
        Conjugation::Concevoir,
        Conjugation::Croire,
        Conjugation::Requérir,
        Conjugation::Dissoudre,
        Conjugation::Pouvoir,
        Conjugation::Lire,
        Conjugation::Taire,
        Conjugation::Moudre,
        Conjugation::Seoir,
        Conjugation::Accroître,
        Conjugation::Survivre,
        Conjugation::Poursuivre,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 29] = [
    // Passer: 6,514 words (abhorrer, abjurer, absconder, absenter, absorber, abuser).
    conjugation!("passer", 6514, "~er";
        participle: "~é";
    ),
    // Sentir: 375 words (pleuvoir, avoir, dormir, partir, polir, abolir).
    conjugation!("sentir", 375, "~r";
        participle: "~";
    ),
    // Dire: 92 words (être, faire, traire, naître, renaître, dire).
    conjugation!("dire", 92, "~re";
        participle: "~t";
    ),
    // Rendre: 66 words (vendre, foutre, mordre, descendre, battre, entendre).
    conjugation!("rendre", 66, "~re";
        participle: "~u";
    ),
    // Tenir: 45 words (souvenir, devenir, courir, venir, obtenir, tenir).
    conjugation!("tenir", 45, "~ir";
        participle: "~u";
    ),
    // Poindre: 30 words (plaindre, éteindre, joindre, craindre, peindre, atteindre).
    conjugation!("poindre", 30, "~dre";
        participle: "~t";
    ),
    // Prévoir: 18 words (choir, voir, vouloir, valoir, falloir, revoir).
    conjugation!("prévoir", 18, "~oir";
        participle: "~u";
    ),
    // Remettre: 18 words (remettre, mettre, émettre, promettre, permettre, commettre).
    conjugation!("remettre", 18, "~ettre";
        participle: "~is";
    ),
    // Connaître: 13 words (connaître, reconnaître, paraître, disparaître, apparaître, comparaître).
    conjugation!("connaître", 13, "~aître";
        participle: "~u";
    ),
    // Paitre: 13 words (reconnaitre, paraitre, disparaitre, connaitre, paitre, apparaitre).
    conjugation!("paitre", 13, "~aitre";
        participle: "~u";
    ),
    // Prendre: 12 words (prendre, comprendre, apprendre, surprendre, reprendre, entreprendre).
    conjugation!("prendre", 12, "~endre";
        participle: "~is";
    ),
    // Bruire: 11 words (rire, sourire, suffire, nuire, luire, conclure).
    conjugation!("bruire", 11, "~re";
        participle: "~";
    ),
    // Éclore: 11 words (inclure, clore, éclore, circoncire, occire, transclure).
    conjugation!("éclore", 11, "~re";
        participle: "~s";
    ),
    // Ouvrir: 10 words (découvrir, ouvrir, offrir, souffrir, couvrir, rouvrir).
    conjugation!("ouvrir", 10, "~~ir";
        participle: "~e~t";
    ),
    // Concevoir: 7 words (recevoir, apercevoir, décevoir, concevoir, percevoir, repercevoir).
    conjugation!("concevoir", 7, "~cevoir";
        participle: "~çu";
    ),
    // Croire: 6 words (boire, croire, reboire, recroire, mécroire, emboire).
    conjugation!("croire", 6, "~oire";
        participle: "~u";
    ),
    // Requérir: 6 words (requérir, acquérir, conquérir, enquérir, reconquérir, quérir).
    conjugation!("requérir", 6, "~ér~r";
        participle: "~~s";
    ),
    // Dissoudre: 6 words (absoudre, dissoudre, autodissoudre, soudre, resoudre, arsoudre).
    conjugation!("dissoudre", 6, "~dre";
        participle: "~s";
    ),
    // Pouvoir: 5 words (pouvoir, émouvoir, promouvoir, repouvoir, remouvoir).
    conjugation!("pouvoir", 5, "~o~voir";
        participle: "~~";
    ),
    // Lire: 4 words (lire, relire, élire, réélire).
    conjugation!("lire", 4, "~ire";
        participle: "~u";
    ),
    // Taire: 4 words (plaire, taire, déplaire, complaire).
    conjugation!("taire", 4, "~aire";
        participle: "~u";
    ),
    // Moudre: 4 words (moudre, remoudre, émoudre, vermoudre).
    conjugation!("moudre", 4, "~dre";
        participle: "~lu";
    ),
    // Seoir: 4 words (seoir, asseoir, rasseoir, surseoir).
    conjugation!("seoir", 4, "~eo~r";
        participle: "~~s";
    ),
    // Accroître: 4 words (décroître, connoître, accroître, reconnoître).
    conjugation!("accroître", 4, "~oître";
        participle: "~u";
    ),
    // Survivre: 3 words (vivre, survivre, revivre).
    conjugation!("survivre", 3, "~ivre";
        participle: "~écu";
    ),
    // Poursuivre: 3 words (suivre, poursuivre, ensuivre).
    conjugation!("poursuivre", 3, "~re";
        participle: "~i";
    ),
    // Recoudre: 3 words (coudre, recoudre, découdre).
    conjugation!("recoudre", 3, "~dre";
        participle: "~su";
    ),
    // Accroitre: 3 words (décroitre, accroitre, recroitre).
    conjugation!("accroitre", 3, "~oitre";
        participle: "~u";
    ),
    // Havoir: 3 words (ravoir, havoir, réavoir).
    conjugation!("havoir", 3, "~avoir";
        participle: "~eu";
    ),
];
