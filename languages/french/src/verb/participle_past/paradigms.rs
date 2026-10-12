//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `imperfect_first` (pl)
/// `imperfect_second` (pl)
/// `past_historic_third` (pl)
/// `participle` (past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     imperfect_first: $c0:tt;
     imperfect_second: $c1:tt;
     past_historic_third: $c2:tt;
     participle: $c3:tt;
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
    /// Like `passé`: 6,519 words.
    Passé,
    /// Like `établi`: 342 words.
    Établi,
    /// Like `rendu`: 72 words.
    Rendu,
    /// Like `senti`: 35 words.
    Senti,
    /// Like `réduit`: 31 words.
    Réduit,
    /// Like `tenu`: 30 words.
    Tenu,
    /// Like `point`: 30 words.
    Point,
    /// Like `remis`: 25 words.
    Remis,
    /// Like `valu`: 25 words.
    Valu,
    /// Like `inscrit`: 16 words.
    Inscrit,
    /// Like `fait`: 12 words.
    Fait,
    /// Like `pris`: 12 words.
    Pris,
    /// Like `soustrait`: 11 words.
    Soustrait,
    /// Like `dit`: 11 words.
    Dit,
    /// Like `connu`: 11 words.
    Connu,
    /// Like `ouvert`: 10 words.
    Ouvert,
    /// Like `conçu`: 7 words.
    Conçu,
    /// Like `conclu`: 6 words.
    Conclu,
    /// Like `lui`: 6 words.
    Lui,
    /// Like `requis`: 6 words.
    Requis,
    /// Like `dissous`: 6 words.
    Dissous,
    /// Like `promu`: 5 words.
    Promu,
    /// Like `pourvu`: 5 words.
    Pourvu,
    /// Like `complu`: 5 words.
    Complu,
    /// Like `prévu`: 4 words.
    Prévu,
    /// Like `élu`: 4 words.
    Élu,
    /// Like `embu`: 3 words.
    Embu,
    /// Like `survécu`: 3 words.
    Survécu,
    /// Like `fui`: 3 words.
    Fui,
    /// Like `ouï`: 3 words.
    Ouï,
    /// Like `inclus`: 3 words.
    Inclus,
    /// Like `assis`: 3 words.
    Assis,
    /// Like `heu`: 3 words.
    Heu,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 33] = [
        Conjugation::Passé,
        Conjugation::Établi,
        Conjugation::Rendu,
        Conjugation::Senti,
        Conjugation::Réduit,
        Conjugation::Tenu,
        Conjugation::Point,
        Conjugation::Remis,
        Conjugation::Valu,
        Conjugation::Inscrit,
        Conjugation::Fait,
        Conjugation::Pris,
        Conjugation::Soustrait,
        Conjugation::Dit,
        Conjugation::Connu,
        Conjugation::Ouvert,
        Conjugation::Conçu,
        Conjugation::Conclu,
        Conjugation::Lui,
        Conjugation::Requis,
        Conjugation::Dissous,
        Conjugation::Promu,
        Conjugation::Pourvu,
        Conjugation::Complu,
        Conjugation::Prévu,
        Conjugation::Élu,
        Conjugation::Embu,
        Conjugation::Survécu,
        Conjugation::Fui,
        Conjugation::Ouï,
        Conjugation::Inclus,
        Conjugation::Assis,
        Conjugation::Heu,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 33] = [
    // Passé: 6,519 words (abhorré, abjuré, abscondé, absenté, absorbé, abusé).
    conjugation!("passé", 6519, "~é";
        imperfect_first: "~ions";
        imperfect_second: "~iez";
        past_historic_third: "~èrent";
        participle: "~é";
    ),
    // Établi: 342 words (eu, poli, aboli, mort, haï, tapi).
    conjugation!("établi", 342, "~";
        imperfect_first: "~ssions";
        imperfect_second: "~ssiez";
        past_historic_third: "~rent";
        participle: "~";
    ),
    // Rendu: 72 words (vendu, foutu, mordu, descendu, battu, entendu).
    conjugation!("rendu", 72, "~u";
        imperfect_first: "~ions";
        imperfect_second: "~iez";
        past_historic_third: "~irent";
        participle: "~u";
    ),
    // Senti: 35 words (dormi, parti, menti, senti, accueilli, sorti).
    conjugation!("senti", 35, "~";
        imperfect_first: "~ons";
        imperfect_second: "~ez";
        past_historic_third: "~rent";
        participle: "~";
    ),
    // Réduit: 31 words (construit, cuit, traduit, produit, détruit, conduit).
    conjugation!("réduit", 31, "~t";
        imperfect_first: "~sions";
        imperfect_second: "~siez";
        past_historic_third: "~sirent";
        participle: "~t";
    ),
    // Tenu: 30 words (souvenu, devenu, venu, obtenu, tenu, intervenu).
    conjugation!("tenu", 30, "~~u";
        imperfect_first: "~~ions";
        imperfect_second: "~~iez";
        past_historic_third: "~inr~t";
        participle: "~~u";
    ),
    // Point: 30 words (plaint, éteint, joint, craint, peint, atteint).
    conjugation!("point", 30, "~~t";
        imperfect_first: "~g~ions";
        imperfect_second: "~g~iez";
        past_historic_third: "~g~irent";
        participle: "~~t";
    ),
    // Remis: 25 words (remis, mis, sis, émis, promis, clos).
    conjugation!("remis", 25, "~~s";
        imperfect_first: "~ett~ons";
        imperfect_second: "~ett~ez";
        past_historic_third: "~~rent";
        participle: "~~s";
    ),
    // Valu: 25 words (chu, voulu, couru, valu, fallu, moulu).
    conjugation!("valu", 25, "~u";
        imperfect_first: "~ions";
        imperfect_second: "~iez";
        past_historic_third: "~urent";
        participle: "~u";
    ),
    // Inscrit: 16 words (écrit, décrit, frit, prescrit, récrit, souscrit).
    conjugation!("inscrit", 16, "~t";
        imperfect_first: "~vions";
        imperfect_second: "~viez";
        past_historic_third: "~virent";
        participle: "~t";
    ),
    // Fait: 12 words (fait, refait, contrefait, défait, satisfait, parfait).
    conjugation!("fait", 12, "~a~t";
        imperfect_first: "~a~sions";
        imperfect_second: "~a~siez";
        past_historic_third: "~~rent";
        participle: "~a~t";
    ),
    // Pris: 12 words (pris, compris, appris, surpris, repris, entrepris).
    conjugation!("pris", 12, "~~s";
        imperfect_first: "~en~ons";
        imperfect_second: "~en~ez";
        past_historic_third: "~~rent";
        participle: "~~s";
    ),
    // Soustrait: 11 words (trait, distrait, soustrait, abstrait, extrait, brait).
    conjugation!("soustrait", 11, "~it";
        imperfect_first: "~yions";
        imperfect_second: "~yiez";
        past_historic_third: "~yèrent";
        participle: "~it";
    ),
    // Dit: 11 words (dit, contredit, redit, interdit, maudit, médit).
    conjugation!("dit", 11, "~t";
        imperfect_first: "~sions";
        imperfect_second: "~siez";
        past_historic_third: "~rent";
        participle: "~t";
    ),
    // Connu: 11 words (connu, reconnu, paru, disparu, apparu, comparu).
    conjugation!("connu", 11, "~u";
        imperfect_first: "~aissions";
        imperfect_second: "~aissiez";
        past_historic_third: "~urent";
        participle: "~u";
    ),
    // Ouvert: 10 words (découvert, ouvert, offert, souffert, couvert, rouvert).
    conjugation!("ouvert", 10, "~e~t";
        imperfect_first: "~~ions";
        imperfect_second: "~~iez";
        past_historic_third: "~~irent";
        participle: "~e~t";
    ),
    // Conçu: 7 words (reçu, aperçu, déçu, conçu, perçu, reperçu).
    conjugation!("conçu", 7, "~çu";
        imperfect_first: "~cevions";
        imperfect_second: "~ceviez";
        past_historic_third: "~çurent";
        participle: "~çu";
    ),
    // Conclu: 6 words (ri, souri, conclu, circoncis, occis, exclu).
    conjugation!("conclu", 6, "~";
        imperfect_first: "~ions";
        imperfect_second: "~iez";
        past_historic_third: "~rent";
        participle: "~";
    ),
    // Lui: 6 words (suffi, nui, lui, relui, tralui, parnui).
    conjugation!("lui", 6, "~";
        imperfect_first: "~sions";
        imperfect_second: "~siez";
        past_historic_third: "~rent";
        participle: "~";
    ),
    // Requis: 6 words (requis, acquis, conquis, enquis, reconquis, quis).
    conjugation!("requis", 6, "~~s";
        imperfect_first: "~ér~ons";
        imperfect_second: "~ér~ez";
        past_historic_third: "~~rent";
        participle: "~~s";
    ),
    // Dissous: 6 words (absous, dissous, autodissous, sous, resous, arsous).
    conjugation!("dissous", 6, "~us";
        imperfect_first: "~lvions";
        imperfect_second: "~lviez";
        past_historic_third: "~lurent";
        participle: "~us";
    ),
    // Promu: 5 words (pu, ému, promu, repu, remu).
    conjugation!("promu", 5, "~~";
        imperfect_first: "~o~vions";
        imperfect_second: "~o~viez";
        past_historic_third: "~~rent";
        participle: "~~";
    ),
    // Pourvu: 5 words (cru, pourvu, dépourvu, recru, mécru).
    conjugation!("pourvu", 5, "~u";
        imperfect_first: "~oyions";
        imperfect_second: "~oyiez";
        past_historic_third: "~urent";
        participle: "~u";
    ),
    // Complu: 5 words (plu, tu, déplu, replu, complu).
    conjugation!("complu", 5, "~u";
        imperfect_first: "~aisions";
        imperfect_second: "~aisiez";
        past_historic_third: "~urent";
        participle: "~u";
    ),
    // Prévu: 4 words (vu, revu, prévu, entrevu).
    conjugation!("prévu", 4, "~u";
        imperfect_first: "~oyions";
        imperfect_second: "~oyiez";
        past_historic_third: "~irent";
        participle: "~u";
    ),
    // Élu: 4 words (lu, relu, élu, réélu).
    conjugation!("élu", 4, "~u";
        imperfect_first: "~isions";
        imperfect_second: "~isiez";
        past_historic_third: "~urent";
        participle: "~u";
    ),
    // Embu: 3 words (bu, rebu, embu).
    conjugation!("embu", 3, "~";
        imperfect_first: "~vions";
        imperfect_second: "~viez";
        past_historic_third: "~rent";
        participle: "~";
    ),
    // Survécu: 3 words (vécu, survécu, revécu).
    conjugation!("survécu", 3, "~écu";
        imperfect_first: "~ivions";
        imperfect_second: "~iviez";
        past_historic_third: "~écurent";
        participle: "~écu";
    ),
    // Fui: 3 words (fui, enfui, refui).
    conjugation!("fui", 3, "~~";
        imperfect_first: "~y~ons";
        imperfect_second: "~y~ez";
        past_historic_third: "~~rent";
        participle: "~~";
    ),
    // Ouï: 3 words (ouï, réouï, inouï).
    conjugation!("ouï", 3, "~uï";
        imperfect_first: "~uïssions|~yions";
        imperfect_second: "~uïssiez|~yiez";
        past_historic_third: "~uïrent";
        participle: "~uï";
    ),
    // Inclus: 3 words (inclus, transclus, occlus).
    conjugation!("inclus", 3, "~s";
        imperfect_first: "~ions";
        imperfect_second: "~iez";
        past_historic_third: "~rent";
        participle: "~s";
    ),
    // Assis: 3 words (assis, rassis, sursis).
    conjugation!("assis", 3, "~~s";
        imperfect_first: "~oy~ons|~ey~ons";
        imperfect_second: "~oy~ez|~ey~ez";
        past_historic_third: "~~rent";
        participle: "~~s";
    ),
    // Heu: 3 words (reu, heu, réeu).
    conjugation!("heu", 3, "~eu";
        imperfect_first: "~avions";
        imperfect_second: "~aviez";
        past_historic_third: "~eurent";
        participle: "~eu";
    ),
];
