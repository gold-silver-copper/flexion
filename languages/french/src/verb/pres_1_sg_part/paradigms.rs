//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `present_first` (sg)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     present_first: $c0:tt;
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
    /// Like `passer`: 6,017 words.
    Passer,
    /// Like `établir`: 334 words.
    Établir,
    /// Like `récupérer`: 244 words.
    Récupérer,
    /// Like `dire`: 196 words.
    Dire,
    /// Like `relever`: 90 words.
    Relever,
    /// Like `renvoyer`: 58 words.
    Renvoyer,
    /// Like `remettre`: 48 words.
    Remettre,
    /// Like `appeler`: 41 words.
    Appeler,
    /// Like `poindre`: 36 words.
    Poindre,
    /// Like `payer`: 31 words.
    Payer,
    /// Like `becqueter`: 31 words.
    Becqueter,
    /// Like `tenir`: 29 words.
    Tenir,
    /// Like `connaître`: 19 words.
    Connaître,
    /// Like `ouvrir`: 16 words.
    Ouvrir,
    /// Like `sentir`: 13 words.
    Sentir,
    /// Like `revêtir`: 12 words.
    Revêtir,
    /// Like `concevoir`: 7 words.
    Concevoir,
    /// Like `requérir`: 6 words.
    Requérir,
    /// Like `poursuivre`: 5 words.
    Poursuivre,
    /// Like `avoir`: 4 words.
    Avoir,
    /// Like `endormir`: 4 words.
    Endormir,
    /// Like `vouloir`: 4 words.
    Vouloir,
    /// Like `valoir`: 4 words.
    Valoir,
    /// Like `jeter`: 4 words.
    Jeter,
    /// Like `mouvoir`: 4 words.
    Mouvoir,
    /// Like `devoir`: 3 words.
    Devoir,
    /// Like `servir`: 3 words.
    Servir,
    /// Like `ouïr`: 3 words.
    Ouïr,
    /// Like `bouillir`: 3 words.
    Bouillir,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 29] = [
        Conjugation::Passer,
        Conjugation::Établir,
        Conjugation::Récupérer,
        Conjugation::Dire,
        Conjugation::Relever,
        Conjugation::Renvoyer,
        Conjugation::Remettre,
        Conjugation::Appeler,
        Conjugation::Poindre,
        Conjugation::Payer,
        Conjugation::Becqueter,
        Conjugation::Tenir,
        Conjugation::Connaître,
        Conjugation::Ouvrir,
        Conjugation::Sentir,
        Conjugation::Revêtir,
        Conjugation::Concevoir,
        Conjugation::Requérir,
        Conjugation::Poursuivre,
        Conjugation::Avoir,
        Conjugation::Endormir,
        Conjugation::Vouloir,
        Conjugation::Valoir,
        Conjugation::Jeter,
        Conjugation::Mouvoir,
        Conjugation::Devoir,
        Conjugation::Servir,
        Conjugation::Ouïr,
        Conjugation::Bouillir,
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
    // Passer: 6,017 words (abhorrer, abjurer, absconder, absenter, absorber, abuser).
    conjugation!("passer", 6017, "~r";
        present_first: "~";
    ),
    // Établir: 334 words (choir, voir, polir, abolir, tapir, approfondir).
    conjugation!("établir", 334, "~r";
        present_first: "~s";
    ),
    // Récupérer: 244 words (pécher, accélérer, espérer, péter, sécher, protéger).
    conjugation!("récupérer", 244, "~é~r";
        present_first: "~è~";
    ),
    // Dire: 196 words (être, lire, boire, faire, traire, plaire).
    conjugation!("dire", 196, "~re";
        present_first: "~s";
    ),
    // Relever: 90 words (lever, élever, peser, acheter, déchiqueter, celer).
    conjugation!("relever", 90, "~e~r";
        present_first: "~è~";
    ),
    // Renvoyer: 58 words (tutoyer, vouvoyer, flyer, nettoyer, employer, envoyer).
    conjugation!("renvoyer", 58, "~y~r";
        present_first: "~i~";
    ),
    // Remettre: 48 words (remettre, mettre, foutre, naitre, battre, émettre).
    conjugation!("remettre", 48, "~tre";
        present_first: "~s";
    ),
    // Appeler: 41 words (épeler, appeler, modeler, rappeler, renouveler, ficeler).
    conjugation!("appeler", 41, "~e~~r";
        present_first: "~e~l~|~è~~";
    ),
    // Poindre: 36 words (plaindre, éteindre, joindre, craindre, peindre, atteindre).
    conjugation!("poindre", 36, "~dre";
        present_first: "~s";
    ),
    // Payer: 31 words (layer, brayer, payer, essayer, balayer, effrayer).
    conjugation!("payer", 31, "~y~r";
        present_first: "~y~|~i~";
    ),
    // Becqueter: 31 words (banqueter, décolleter, riveter, feuilleter, hoqueter, becqueter).
    conjugation!("becqueter", 31, "~e~~r";
        present_first: "~e~t~|~è~~";
    ),
    // Tenir: 29 words (souvenir, devenir, venir, obtenir, tenir, intervenir).
    conjugation!("tenir", 29, "~~ir";
        present_first: "~i~s";
    ),
    // Connaître: 19 words (naître, renaître, connaître, reconnaître, paraître, disparaître).
    conjugation!("connaître", 19, "~ître";
        present_first: "~is";
    ),
    // Ouvrir: 16 words (accueillir, découvrir, ouvrir, offrir, cueillir, souffrir).
    conjugation!("ouvrir", 16, "~ir";
        present_first: "~e";
    ),
    // Sentir: 13 words (partir, mentir, sentir, sortir, repartir, repentir).
    conjugation!("sentir", 13, "~tir";
        present_first: "~s";
    ),
    // Revêtir: 12 words (courir, secourir, recourir, accourir, encourir, parcourir).
    conjugation!("revêtir", 12, "~ir";
        present_first: "~s";
    ),
    // Concevoir: 7 words (recevoir, apercevoir, décevoir, concevoir, percevoir, repercevoir).
    conjugation!("concevoir", 7, "~cev~r";
        present_first: "~ç~s";
    ),
    // Requérir: 6 words (requérir, acquérir, conquérir, enquérir, reconquérir, quérir).
    conjugation!("requérir", 6, "~ér~~";
        present_first: "~~e~s";
    ),
    // Poursuivre: 5 words (vivre, suivre, poursuivre, survivre, revivre).
    conjugation!("poursuivre", 5, "~vre";
        present_first: "~s";
    ),
    // Avoir: 4 words (avoir, ravoir, havoir, réavoir).
    conjugation!("avoir", 4, "~vo~r";
        present_first: "~~";
    ),
    // Endormir: 4 words (dormir, endormir, rendormir, redormir).
    conjugation!("endormir", 4, "~mir";
        present_first: "~s";
    ),
    // Vouloir: 4 words (vouloir, revouloir, douloir, souloir).
    conjugation!("vouloir", 4, "~o~loir";
        present_first: "~e~x";
    ),
    // Valoir: 4 words (valoir, prévaloir, équivaloir, revaloir).
    conjugation!("valoir", 4, "~loir";
        present_first: "~ux";
    ),
    // Jeter: 4 words (jeter, projeter, rejeter, interjeter).
    conjugation!("jeter", 4, "~~r";
        present_first: "~t~";
    ),
    // Mouvoir: 4 words (mouvoir, émouvoir, promouvoir, remouvoir).
    conjugation!("mouvoir", 4, "~o~voir";
        present_first: "~e~s";
    ),
    // Devoir: 3 words (devoir, redevoir, ramentevoir).
    conjugation!("devoir", 3, "~ev~r";
        present_first: "~~s";
    ),
    // Servir: 3 words (servir, resservir, desservir).
    conjugation!("servir", 3, "~vir";
        present_first: "~s";
    ),
    // Ouïr: 3 words (ouïr, réouïr, inouïr).
    conjugation!("ouïr", 3, "~uïr";
        present_first: "~uïs|~is";
    ),
    // Bouillir: 3 words (bouillir, rebouillir, débouillir).
    conjugation!("bouillir", 3, "~illir";
        present_first: "~s";
    ),
];
