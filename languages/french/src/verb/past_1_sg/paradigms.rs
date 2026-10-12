//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `present_first` (pl)
/// `imperfect_first` (sg)
/// `imperfect_second` (sg)
/// `imperfect_third` (sg, pl)
/// `past_historic_first` (sg, pl)
/// `past_historic_second` (sg, pl)
/// `past_historic_third` (sg)
/// `imperfect_subjunctive_first` (sg, pl)
/// `imperfect_subjunctive_second` (sg, pl)
/// `imperfect_subjunctive_third` (sg, pl)
/// `imperative` (nous)
/// `participle` (pres)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     present_first: $c0:tt;
     imperfect_first: $c1:tt;
     imperfect_second: $c2:tt;
     imperfect_third: $c3:tt, $c4:tt;
     past_historic_first: $c5:tt, $c6:tt;
     past_historic_second: $c7:tt, $c8:tt;
     past_historic_third: $c9:tt;
     imperfect_subjunctive_first: $c10:tt, $c11:tt;
     imperfect_subjunctive_second: $c12:tt, $c13:tt;
     imperfect_subjunctive_third: $c14:tt, $c15:tt;
     imperative: $c16:tt;
     participle: $c17:tt;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `passai`: 6,504 words.
    Passai,
    /// Like `établis`: 328 words.
    Établis,
    /// Like `rendis`: 195 words.
    Rendis,
    /// Like `eus`: 29 words.
    Eus,
    /// Like `tins`: 29 words.
    Tins,
    /// Like `remis`: 18 words.
    Remis,
    /// Like `dis`: 13 words.
    Dis,
    /// Like `fis`: 12 words.
    Fis,
    /// Like `pris`: 12 words.
    Pris,
    /// Like `connus`: 11 words.
    Connus,
    /// Like `prévis`: 7 words.
    Prévis,
    /// Like `pourvus`: 7 words.
    Pourvus,
    /// Like `conçus`: 7 words.
    Conçus,
    /// Like `pus`: 6 words.
    Pus,
    /// Like `requis`: 6 words.
    Requis,
    /// Like `dissolus`: 6 words.
    Dissolus,
    /// Like `resvoi`: 6 words.
    Resvoi,
    /// Like `inclus`: 5 words.
    Inclus,
    /// Like `lus`: 4 words.
    Lus,
    /// Like `tus`: 4 words.
    Tus,
    /// Like `ouïs`: 4 words.
    Ouïs,
    /// Like `embus`: 3 words.
    Embus,
    /// Like `survécus`: 3 words.
    Survécus,
    /// Like `dus`: 3 words.
    Dus,
    /// Like `fuis`: 3 words.
    Fuis,
    /// Like `luis`: 3 words.
    Luis,
    /// Like `heus`: 3 words.
    Heus,
    /// Like `crûs`: 2 words.
    Crûs,
    /// Like `haïs`: 1 words.
    Haïs,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 29] = [
        Conjugation::Passai,
        Conjugation::Établis,
        Conjugation::Rendis,
        Conjugation::Eus,
        Conjugation::Tins,
        Conjugation::Remis,
        Conjugation::Dis,
        Conjugation::Fis,
        Conjugation::Pris,
        Conjugation::Connus,
        Conjugation::Prévis,
        Conjugation::Pourvus,
        Conjugation::Conçus,
        Conjugation::Pus,
        Conjugation::Requis,
        Conjugation::Dissolus,
        Conjugation::Resvoi,
        Conjugation::Inclus,
        Conjugation::Lus,
        Conjugation::Tus,
        Conjugation::Ouïs,
        Conjugation::Embus,
        Conjugation::Survécus,
        Conjugation::Dus,
        Conjugation::Fuis,
        Conjugation::Luis,
        Conjugation::Heus,
        Conjugation::Crûs,
        Conjugation::Haïs,
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
    // Passai: 6,504 words (abhorrai, abjurai, abscondai, absentai, absorbai, abusai).
    conjugation!("passai", 6504, "~ai";
        present_first: "~ons";
        imperfect_first: "~ais";
        imperfect_second: "~ais";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~ai", "~âmes";
        past_historic_second: "~as", "~âtes";
        past_historic_third: "~a";
        imperfect_subjunctive_first: "~asse", "~assions";
        imperfect_subjunctive_second: "~asses", "~assiez";
        imperfect_subjunctive_third: "~ât", "~assent";
        imperative: "~ons";
        participle: "~ant";
    ),
    // Établis: 328 words (polis, abolis, naquis, renaquis, tapis, approfondis).
    conjugation!("établis", 328, "~is";
        present_first: "~issons";
        imperfect_first: "~issais";
        imperfect_second: "~issais";
        imperfect_third: "~issait", "~issaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~issons";
        participle: "~issant";
    ),
    // Rendis: 195 words (dormis, partis, mentis, sentis, vendis, construisis).
    conjugation!("rendis", 195, "~is";
        present_first: "~ons";
        imperfect_first: "~ais";
        imperfect_second: "~ais";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~ons";
        participle: "~ant";
    ),
    // Eus: 29 words (fus, eus, mourus, voulus, sus, courus).
    conjugation!("eus", 29, "~us";
        present_first: "~ons";
        imperfect_first: "~ais";
        imperfect_second: "~ais";
        imperfect_third: "~ait", "~aient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ons";
        participle: "~ant";
    ),
    // Tins: 29 words (souvins, devins, vins, obtins, tins, intervins).
    conjugation!("tins", 29, "~i~s";
        present_first: "~e~ons";
        imperfect_first: "~e~ais";
        imperfect_second: "~e~ais";
        imperfect_third: "~e~ait", "~e~aient";
        past_historic_first: "~i~s", "~î~mes";
        past_historic_second: "~i~s", "~î~tes";
        past_historic_third: "~i~t";
        imperfect_subjunctive_first: "~i~sse", "~i~ssions";
        imperfect_subjunctive_second: "~i~sses", "~i~ssiez";
        imperfect_subjunctive_third: "~î~t", "~i~ssent";
        imperative: "~e~ons";
        participle: "~e~ant";
    ),
    // Remis: 18 words (remis, mis, émis, promis, permis, commis).
    conjugation!("remis", 18, "~is";
        present_first: "~ettons";
        imperfect_first: "~ettais";
        imperfect_second: "~ettais";
        imperfect_third: "~ettait", "~ettaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~ettons";
        participle: "~ettant";
    ),
    // Dis: 13 words (dis, contredis, redis, suffis, interdis, médis).
    conjugation!("dis", 13, "~is";
        present_first: "~isons";
        imperfect_first: "~isais";
        imperfect_second: "~isais";
        imperfect_third: "~isait", "~isaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~isons";
        participle: "~isant";
    ),
    // Fis: 12 words (fis, refis, contrefis, défis, satisfis, parfis).
    conjugation!("fis", 12, "~is";
        present_first: "~aisons";
        imperfect_first: "~aisais";
        imperfect_second: "~aisais";
        imperfect_third: "~aisait", "~aisaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~aisons";
        participle: "~aisant";
    ),
    // Pris: 12 words (pris, compris, appris, surpris, repris, entrepris).
    conjugation!("pris", 12, "~is";
        present_first: "~enons";
        imperfect_first: "~enais";
        imperfect_second: "~enais";
        imperfect_third: "~enait", "~enaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~enons";
        participle: "~enant";
    ),
    // Connus: 11 words (connus, reconnus, parus, disparus, apparus, comparus).
    conjugation!("connus", 11, "~us";
        present_first: "~aissons";
        imperfect_first: "~aissais";
        imperfect_second: "~aissais";
        imperfect_third: "~aissait", "~aissaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~aissons";
        participle: "~aissant";
    ),
    // Prévis: 7 words (vis, revis, assis, prévis, entrevis, rassis).
    conjugation!("prévis", 7, "~is";
        present_first: "~oyons";
        imperfect_first: "~oyais";
        imperfect_second: "~oyais";
        imperfect_third: "~oyait", "~oyaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~oyons";
        participle: "~oyant";
    ),
    // Pourvus: 7 words (chus, crus, pourvus, déchus, dépourvus, recrus).
    conjugation!("pourvus", 7, "~us";
        present_first: "~oyons";
        imperfect_first: "~oyais";
        imperfect_second: "~oyais";
        imperfect_third: "~oyait", "~oyaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~oyons";
        participle: "~oyant";
    ),
    // Conçus: 7 words (reçus, aperçus, déçus, conçus, perçus, reperçus).
    conjugation!("conçus", 7, "~çus";
        present_first: "~cevons";
        imperfect_first: "~cevais";
        imperfect_second: "~cevais";
        imperfect_third: "~cevait", "~cevaient";
        past_historic_first: "~çus", "~çûmes";
        past_historic_second: "~çus", "~çûtes";
        past_historic_third: "~çut";
        imperfect_subjunctive_first: "~çusse", "~çussions";
        imperfect_subjunctive_second: "~çusses", "~çussiez";
        imperfect_subjunctive_third: "~çût", "~çussent";
        imperative: "~cevons";
        participle: "~cevant";
    ),
    // Pus: 6 words (pus, mus, émus, promus, repus, remus).
    conjugation!("pus", 6, "~us";
        present_first: "~ouvons";
        imperfect_first: "~ouvais";
        imperfect_second: "~ouvais";
        imperfect_third: "~ouvait", "~ouvaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~ouvons";
        participle: "~ouvant";
    ),
    // Requis: 6 words (requis, acquis, conquis, enquis, reconquis, quis).
    conjugation!("requis", 6, "~is";
        present_first: "~érons";
        imperfect_first: "~érais";
        imperfect_second: "~érais";
        imperfect_third: "~érait", "~éraient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~érons";
        participle: "~érant";
    ),
    // Dissolus: 6 words (résolus, absolus, dissolus, autodissolus, resolus, arsolus).
    conjugation!("dissolus", 6, "~us";
        present_first: "~vons";
        imperfect_first: "~vais";
        imperfect_second: "~vais";
        imperfect_third: "~vait", "~vaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~vons";
        participle: "~vant";
    ),
    // Resvoi: 6 words (resvoi, esloignoi, brusloi, esposoi, gaschoi, ruisselloi).
    conjugation!("resvoi", 6, "~oi";
        present_first: "~ons";
        imperfect_first: "~ois";
        imperfect_second: "~ois";
        imperfect_third: "~oit", "~oient";
        past_historic_first: "~oi", "~asmes";
        past_historic_second: "~as", "~astes";
        past_historic_third: "~a";
        imperfect_subjunctive_first: "~asse", "~assions";
        imperfect_subjunctive_second: "~asses", "~assiez";
        imperfect_subjunctive_third: "~ast", "~assent";
        imperative: "~ons";
        participle: "~ant";
    ),
    // Inclus: 5 words (inclus, conclus, exclus, transclus, occlus).
    conjugation!("inclus", 5, "~us";
        present_first: "~uons";
        imperfect_first: "~uais";
        imperfect_second: "~uais";
        imperfect_third: "~uait", "~uaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~uons";
        participle: "~uant";
    ),
    // Lus: 4 words (lus, relus, élus, réélus).
    conjugation!("lus", 4, "~us";
        present_first: "~isons";
        imperfect_first: "~isais";
        imperfect_second: "~isais";
        imperfect_third: "~isait", "~isaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~isons";
        participle: "~isant";
    ),
    // Tus: 4 words (plus, tus, déplus, complus).
    conjugation!("tus", 4, "~us";
        present_first: "~aisons";
        imperfect_first: "~aisais";
        imperfect_second: "~aisais";
        imperfect_third: "~aisait", "~aisaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~aisons";
        participle: "~aisant";
    ),
    // Ouïs: 4 words (ouïs, amuïs, réouïs, inouïs).
    conjugation!("ouïs", 4, "~uïs";
        present_first: "~uïssons|~yons";
        imperfect_first: "~uïssais|~yais";
        imperfect_second: "~uïssais|~yais";
        imperfect_third: "~uïssait|~yait", "~uïssaient|~yaient";
        past_historic_first: "~uïs", "~uïmes";
        past_historic_second: "~uïs", "~uïtes";
        past_historic_third: "~uït";
        imperfect_subjunctive_first: "~uïsse", "~uïssions";
        imperfect_subjunctive_second: "~uïsses", "~uïssiez";
        imperfect_subjunctive_third: "~uït", "~uïssent";
        imperative: "~uïssons|~yons";
        participle: "~uïssant|~yant";
    ),
    // Embus: 3 words (bus, rebus, embus).
    conjugation!("embus", 3, "~us";
        present_first: "~uvons";
        imperfect_first: "~uvais";
        imperfect_second: "~uvais";
        imperfect_third: "~uvait", "~uvaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~uvons";
        participle: "~uvant";
    ),
    // Survécus: 3 words (vécus, survécus, revécus).
    conjugation!("survécus", 3, "~écus";
        present_first: "~ivons";
        imperfect_first: "~ivais";
        imperfect_second: "~ivais";
        imperfect_third: "~ivait", "~ivaient";
        past_historic_first: "~écus", "~écûmes";
        past_historic_second: "~écus", "~écûtes";
        past_historic_third: "~écut";
        imperfect_subjunctive_first: "~écusse", "~écussions";
        imperfect_subjunctive_second: "~écusses", "~écussiez";
        imperfect_subjunctive_third: "~écût", "~écussent";
        imperative: "~ivons";
        participle: "~ivant";
    ),
    // Dus: 3 words (dus, redus, ramentus).
    conjugation!("dus", 3, "~us";
        present_first: "~evons";
        imperfect_first: "~evais";
        imperfect_second: "~evais";
        imperfect_third: "~evait", "~evaient";
        past_historic_first: "~us", "~ûmes";
        past_historic_second: "~us", "~ûtes";
        past_historic_third: "~ut";
        imperfect_subjunctive_first: "~usse", "~ussions";
        imperfect_subjunctive_second: "~usses", "~ussiez";
        imperfect_subjunctive_third: "~ût", "~ussent";
        imperative: "~evons";
        participle: "~evant";
    ),
    // Fuis: 3 words (fuis, enfuis, refuis).
    conjugation!("fuis", 3, "~is";
        present_first: "~yons";
        imperfect_first: "~yais";
        imperfect_second: "~yais";
        imperfect_third: "~yait", "~yaient";
        past_historic_first: "~is", "~îmes";
        past_historic_second: "~is", "~îtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isse", "~issions";
        imperfect_subjunctive_second: "~isses", "~issiez";
        imperfect_subjunctive_third: "~ît", "~issent";
        imperative: "~yons";
        participle: "~yant";
    ),
    // Luis: 3 words (luis, reluis, traluis).
    conjugation!("luis", 3, "~is";
        present_first: "~isons";
        imperfect_first: "~isais";
        imperfect_second: "~isais";
        imperfect_third: "~isait", "~isaient";
        past_historic_first: "~is|~isis", "~îmes|~isîmes";
        past_historic_second: "~is|~isis", "~îtes|~isîtes";
        past_historic_third: "~it";
        imperfect_subjunctive_first: "~isisse", "~isissions";
        imperfect_subjunctive_second: "~isisses", "~isissiez";
        imperfect_subjunctive_third: "~isît", "~isissent";
        imperative: "~isons";
        participle: "~isant";
    ),
    // Heus: 3 words (reus, heus, réeus).
    conjugation!("heus", 3, "~eus";
        present_first: "~avons";
        imperfect_first: "~avais";
        imperfect_second: "~avais";
        imperfect_third: "~avait", "~avaient";
        past_historic_first: "~eus", "~eûmes";
        past_historic_second: "~eus", "~eûtes";
        past_historic_third: "~eut";
        imperfect_subjunctive_first: "~eusse", "~eussions";
        imperfect_subjunctive_second: "~eusses", "~eussiez";
        imperfect_subjunctive_third: "~eût", "~eussent";
        imperative: "~ayons";
        participle: "~ayant";
    ),
    // Crûs: 2 words (crûs, recrûs).
    conjugation!("crûs", 2, "~ûs";
        present_first: "~oissons";
        imperfect_first: "~oissais";
        imperfect_second: "~oissais";
        imperfect_third: "~oissait", "~oissaient";
        past_historic_first: "~ûs", "~ûmes";
        past_historic_second: "~ûs", "~ûtes";
        past_historic_third: "~ût";
        imperfect_subjunctive_first: "~ûsse", "~ûssions";
        imperfect_subjunctive_second: "~ûsses", "~ûssiez";
        imperfect_subjunctive_third: "~ût", "~ûssent";
        imperative: "~oissons";
        participle: "~oissant";
    ),
    // Haïs: 1 words (haïs).
    conjugation!("haïs", 1, "~s";
        present_first: "~ssons";
        imperfect_first: "~ssais";
        imperfect_second: "~ssais";
        imperfect_third: "~ssait", "~ssaient";
        past_historic_first: "~s", "~mes";
        past_historic_second: "~s", "~tes";
        past_historic_third: "~t";
        imperfect_subjunctive_first: "~sse", "~ssions";
        imperfect_subjunctive_second: "~sses", "~ssiez";
        imperfect_subjunctive_third: "~t", "~ssent";
        imperative: "~ssons";
        participle: "~ssant";
    ),
];
