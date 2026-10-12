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
/// `present_second` (sg)
/// `present_third` (sg, pl)
/// `future_first` (sg, pl)
/// `future_second` (sg, pl)
/// `future_third` (sg, pl)
/// `conditional_first` (sg, pl)
/// `conditional_second` (sg, pl)
/// `conditional_third` (sg, pl)
/// `present_subjunctive_first` (sg)
/// `present_subjunctive_second` (sg)
/// `present_subjunctive_third` (sg, pl)
/// `imperative` (tu)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     present_first: $c0:tt;
     present_second: $c1:tt;
     present_third: $c2:tt, $c3:tt;
     future_first: $c4:tt, $c5:tt;
     future_second: $c6:tt, $c7:tt;
     future_third: $c8:tt, $c9:tt;
     conditional_first: $c10:tt, $c11:tt;
     conditional_second: $c12:tt, $c13:tt;
     conditional_third: $c14:tt, $c15:tt;
     present_subjunctive_first: $c16:tt;
     present_subjunctive_second: $c17:tt;
     present_subjunctive_third: $c18:tt, $c19:tt;
     imperative: $c20:tt;
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
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `passe`: 6,147 words.
    Passe,
    /// Like `établis`: 329 words.
    Établis,
    /// Like `récupère`: 225 words.
    Récupère,
    /// Like `dis`: 55 words.
    Dis,
    /// Like `rends`: 54 words.
    Rends,
    /// Like `prévois`: 46 words.
    Prévois,
    /// Like `chancelle`: 38 words.
    Chancelle,
    /// Like `remets`: 31 words.
    Remets,
    /// Like `paye`: 31 words.
    Paye,
    /// Like `becquette`: 31 words.
    Becquette,
    /// Like `poins`: 30 words.
    Poins,
    /// Like `tiens`: 29 words.
    Tiens,
    /// Like `inscris`: 18 words.
    Inscris,
    /// Like `connais`: 17 words.
    Connais,
    /// Like `ouvre`: 15 words.
    Ouvre,
    /// Like `agrège`: 14 words.
    Agrège,
    /// Like `sens`: 13 words.
    Sens,
    /// Like `fais`: 13 words.
    Fais,
    /// Like `prends`: 12 words.
    Prends,
    /// Like `conçois`: 7 words.
    Conçois,
    /// Like `peux`: 6 words.
    Peux,
    /// Like `éclos`: 6 words.
    Éclos,
    /// Like `requiers`: 6 words.
    Requiers,
    /// Like `dissous`: 6 words.
    Dissous,
    /// Like `resve`: 6 words.
    Resve,
    /// Like `vaux`: 5 words.
    Vaux,
    /// Like `endors`: 4 words.
    Endors,
    /// Like `poursuis`: 4 words.
    Poursuis,
    /// Like `mouds`: 4 words.
    Mouds,
    /// Like `ouïs`: 4 words.
    Ouïs,
    /// Like `meus`: 4 words.
    Meus,
    /// Like `revêts`: 4 words.
    Revêts,
    /// Like `vois`: 3 words.
    Vois,
    /// Like `complais`: 3 words.
    Complais,
    /// Like `dois`: 3 words.
    Dois,
    /// Like `sers`: 3 words.
    Sers,
    /// Like `recouds`: 3 words.
    Recouds,
    /// Like `bous`: 3 words.
    Bous,
    /// Like `hai`: 3 words.
    Hai,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 39] = [
        Conjugation::Passe,
        Conjugation::Établis,
        Conjugation::Récupère,
        Conjugation::Dis,
        Conjugation::Rends,
        Conjugation::Prévois,
        Conjugation::Chancelle,
        Conjugation::Remets,
        Conjugation::Paye,
        Conjugation::Becquette,
        Conjugation::Poins,
        Conjugation::Tiens,
        Conjugation::Inscris,
        Conjugation::Connais,
        Conjugation::Ouvre,
        Conjugation::Agrège,
        Conjugation::Sens,
        Conjugation::Fais,
        Conjugation::Prends,
        Conjugation::Conçois,
        Conjugation::Peux,
        Conjugation::Éclos,
        Conjugation::Requiers,
        Conjugation::Dissous,
        Conjugation::Resve,
        Conjugation::Vaux,
        Conjugation::Endors,
        Conjugation::Poursuis,
        Conjugation::Mouds,
        Conjugation::Ouïs,
        Conjugation::Meus,
        Conjugation::Revêts,
        Conjugation::Vois,
        Conjugation::Complais,
        Conjugation::Dois,
        Conjugation::Sers,
        Conjugation::Recouds,
        Conjugation::Bous,
        Conjugation::Hai,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 39] = [
    // Passe: 6,147 words (abhorre, abjure, absconde, absente, absorbe, abuse).
    conjugation!("passe", 6147, "~";
        present_first: "~";
        present_second: "~s";
        present_third: "~", "~nt";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~";
        present_subjunctive_second: "~s";
        present_subjunctive_third: "~", "~nt";
        imperative: "~";
    ),
    // Établis: 329 words (polis, abolis, hais, tapis, approfondis, finis).
    conjugation!("établis", 329, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~ssent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~sse";
        present_subjunctive_second: "~sses";
        present_subjunctive_third: "~sse", "~ssent";
        imperative: "~s";
    ),
    // Récupère: 225 words (pèche, accélère, espère, pète, sèche, succède).
    conjugation!("récupère", 225, "~è~";
        present_first: "~è~";
        present_second: "~è~s";
        present_third: "~è~", "~è~nt";
        future_first: "~è~rai|~é~rai", "~è~rons|~é~rons";
        future_second: "~è~ras|~é~ras", "~è~rez|~é~rez";
        future_third: "~è~ra|~é~ra", "~è~ront|~é~ront";
        conditional_first: "~è~rais|~é~rais", "~è~rions|~é~rions";
        conditional_second: "~è~rais|~é~rais", "~è~riez|~é~riez";
        conditional_third: "~è~rait|~é~rait", "~è~raient|~é~raient";
        present_subjunctive_first: "~è~";
        present_subjunctive_second: "~è~s";
        present_subjunctive_third: "~è~", "~è~nt";
        imperative: "~è~";
    ),
    // Dis: 55 words (lis, dis, construis, cuis, contredis, traduis).
    conjugation!("dis", 55, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~sent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~se";
        present_subjunctive_second: "~ses";
        present_subjunctive_third: "~se", "~sent";
        imperative: "~s";
    ),
    // Rends: 54 words (vends, sieds, mords, descends, entends, vaincs).
    conjugation!("rends", 54, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~", "~ent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~e";
        present_subjunctive_second: "~es";
        present_subjunctive_third: "~e", "~ent";
        imperative: "~s";
    ),
    // Prévois: 46 words (chois, trais, meurs, cours, fuis, ris).
    conjugation!("prévois", 46, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~ent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~e";
        present_subjunctive_second: "~es";
        present_subjunctive_third: "~e", "~ent";
        imperative: "~s";
    ),
    // Chancelle: 38 words (épelle, modelle, renouvelle, ficelle, attelle, dételle).
    conjugation!("chancelle", 38, "~e~l~";
        present_first: "~e~l~|~è~~";
        present_second: "~e~l~s|~è~~s";
        present_third: "~e~l~|~è~~", "~e~l~nt|~è~~nt";
        future_first: "~e~l~rai|~è~~rai", "~e~l~rons|~è~~rons";
        future_second: "~e~l~ras|~è~~ras", "~e~l~rez|~è~~rez";
        future_third: "~e~l~ra|~è~~ra", "~e~l~ront|~è~~ront";
        conditional_first: "~e~l~rais|~è~~rais", "~e~l~rions|~è~~rions";
        conditional_second: "~e~l~rais|~è~~rais", "~e~l~riez|~è~~riez";
        conditional_third: "~e~l~rait|~è~~rait", "~e~l~raient|~è~~raient";
        present_subjunctive_first: "~e~l~|~è~~";
        present_subjunctive_second: "~e~l~s|~è~~s";
        present_subjunctive_third: "~e~l~|~è~~", "~e~l~nt|~è~~nt";
        imperative: "~e~l~|~è~~";
    ),
    // Remets: 31 words (remets, mets, fous, bats, émets, promets).
    conjugation!("remets", 31, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~", "~tent";
        future_first: "~trai", "~trons";
        future_second: "~tras", "~trez";
        future_third: "~tra", "~tront";
        conditional_first: "~trais", "~trions";
        conditional_second: "~trais", "~triez";
        conditional_third: "~trait", "~traient";
        present_subjunctive_first: "~te";
        present_subjunctive_second: "~tes";
        present_subjunctive_third: "~te", "~tent";
        imperative: "~s";
    ),
    // Paye: 31 words (laye, braye, paye, essaye, balaye, effraye).
    conjugation!("paye", 31, "~y~";
        present_first: "~y~|~i~";
        present_second: "~y~s|~i~s";
        present_third: "~y~|~i~", "~y~nt|~i~nt";
        future_first: "~y~rai|~i~rai", "~y~rons|~i~rons";
        future_second: "~y~ras|~i~ras", "~y~rez|~i~rez";
        future_third: "~y~ra|~i~ra", "~y~ront|~i~ront";
        conditional_first: "~y~rais|~i~rais", "~y~rions|~i~rions";
        conditional_second: "~y~rais|~i~rais", "~y~riez|~i~riez";
        conditional_third: "~y~rait|~i~rait", "~y~raient|~i~raient";
        present_subjunctive_first: "~y~|~i~";
        present_subjunctive_second: "~y~s|~i~s";
        present_subjunctive_third: "~y~|~i~", "~y~nt|~i~nt";
        imperative: "~y~|~i~";
    ),
    // Becquette: 31 words (banquette, décollette, rivette, feuillette, hoquette, becquette).
    conjugation!("becquette", 31, "~e~t~";
        present_first: "~e~t~|~è~~";
        present_second: "~e~t~s|~è~~s";
        present_third: "~e~t~|~è~~", "~e~t~nt|~è~~nt";
        future_first: "~e~t~rai|~è~~rai", "~e~t~rons|~è~~rons";
        future_second: "~e~t~ras|~è~~ras", "~e~t~rez|~è~~rez";
        future_third: "~e~t~ra|~è~~ra", "~e~t~ront|~è~~ront";
        conditional_first: "~e~t~rais|~è~~rais", "~e~t~rions|~è~~rions";
        conditional_second: "~e~t~rais|~è~~rais", "~e~t~riez|~è~~riez";
        conditional_third: "~e~t~rait|~è~~rait", "~e~t~raient|~è~~raient";
        present_subjunctive_first: "~e~t~|~è~~";
        present_subjunctive_second: "~e~t~s|~è~~s";
        present_subjunctive_third: "~e~t~|~è~~", "~e~t~nt|~è~~nt";
        imperative: "~e~t~|~è~~";
    ),
    // Poins: 30 words (plains, éteins, joins, crains, peins, atteins).
    conjugation!("poins", 30, "~~s";
        present_first: "~~s";
        present_second: "~~s";
        present_third: "~~t", "~g~ent";
        future_first: "~~drai", "~~drons";
        future_second: "~~dras", "~~drez";
        future_third: "~~dra", "~~dront";
        conditional_first: "~~drais", "~~drions";
        conditional_second: "~~drais", "~~driez";
        conditional_third: "~~drait", "~~draient";
        present_subjunctive_first: "~g~e";
        present_subjunctive_second: "~g~es";
        present_subjunctive_third: "~g~e", "~g~ent";
        imperative: "~~s";
    ),
    // Tiens: 29 words (souviens, deviens, viens, obtiens, tiens, interviens).
    conjugation!("tiens", 29, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~nent";
        future_first: "~drai", "~drons";
        future_second: "~dras", "~drez";
        future_third: "~dra", "~dront";
        conditional_first: "~drais", "~drions";
        conditional_second: "~drais", "~driez";
        conditional_third: "~drait", "~draient";
        present_subjunctive_first: "~ne";
        present_subjunctive_second: "~nes";
        present_subjunctive_third: "~ne", "~nent";
        imperative: "~s";
    ),
    // Inscris: 18 words (bois, écris, décris, prescris, récris, souscris).
    conjugation!("inscris", 18, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~vent";
        future_first: "~rai", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rais", "~rions";
        conditional_second: "~rais", "~riez";
        conditional_third: "~rait", "~raient";
        present_subjunctive_first: "~ve";
        present_subjunctive_second: "~ves";
        present_subjunctive_third: "~ve", "~vent";
        imperative: "~s";
    ),
    // Connais: 17 words (nais, renais, connais, reconnais, disparais, apparais).
    conjugation!("connais", 17, "~is";
        present_first: "~is";
        present_second: "~is";
        present_third: "~ît", "~issent";
        future_first: "~îtrai", "~îtrons";
        future_second: "~îtras", "~îtrez";
        future_third: "~îtra", "~îtront";
        conditional_first: "~îtrais", "~îtrions";
        conditional_second: "~îtrais", "~îtriez";
        conditional_third: "~îtrait", "~îtraient";
        present_subjunctive_first: "~isse";
        present_subjunctive_second: "~isses";
        present_subjunctive_third: "~isse", "~issent";
        imperative: "~is";
    ),
    // Ouvre: 15 words (accueille, découvre, ouvre, offre, cueille, souffre).
    conjugation!("ouvre", 15, "~e";
        present_first: "~e";
        present_second: "~es";
        present_third: "~e", "~ent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~e";
        present_subjunctive_second: "~es";
        present_subjunctive_third: "~e", "~ent";
        imperative: "~e";
    ),
    // Agrège: 14 words (protège, abrège, piège, assiège, allège, siège).
    conjugation!("agrège", 14, "~è~";
        present_first: "~è~";
        present_second: "~è~s";
        present_third: "~è~", "~è~nt";
        future_first: "~é~rai|~è~rai", "~é~rons|~è~rons";
        future_second: "~é~ras|~è~ras", "~é~rez|~è~rez";
        future_third: "~é~ra|~è~ra", "~é~ront|~è~ront";
        conditional_first: "~é~rais|~è~rais", "~é~rions|~è~rions";
        conditional_second: "~é~rais|~è~rais", "~é~riez|~è~riez";
        conditional_third: "~é~rait|~è~rait", "~é~raient|~è~raient";
        present_subjunctive_first: "~è~";
        present_subjunctive_second: "~è~s";
        present_subjunctive_third: "~è~", "~è~nt";
        imperative: "~è~";
    ),
    // Sens: 13 words (pars, mens, sens, sors, repars, repens).
    conjugation!("sens", 13, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~tent";
        future_first: "~tirai", "~tirons";
        future_second: "~tiras", "~tirez";
        future_third: "~tira", "~tiront";
        conditional_first: "~tirais", "~tirions";
        conditional_second: "~tirais", "~tiriez";
        conditional_third: "~tirait", "~tiraient";
        present_subjunctive_first: "~te";
        present_subjunctive_second: "~tes";
        present_subjunctive_third: "~te", "~tent";
        imperative: "~s";
    ),
    // Fais: 13 words (fais, vais, refais, contrefais, défais, satisfais).
    conjugation!("fais", 13, "~ais";
        present_first: "~ais";
        present_second: "~ais";
        present_third: "~ait", "~ont";
        future_first: "~erai", "~erons";
        future_second: "~eras", "~erez";
        future_third: "~era", "~eront";
        conditional_first: "~erais", "~erions";
        conditional_second: "~erais", "~eriez";
        conditional_third: "~erait", "~eraient";
        present_subjunctive_first: "~asse";
        present_subjunctive_second: "~asses";
        present_subjunctive_third: "~asse", "~assent";
        imperative: "~ais";
    ),
    // Prends: 12 words (prends, comprends, apprends, surprends, reprends, entreprends).
    conjugation!("prends", 12, "~ds";
        present_first: "~ds";
        present_second: "~ds";
        present_third: "~d", "~nent";
        future_first: "~drai", "~drons";
        future_second: "~dras", "~drez";
        future_third: "~dra", "~dront";
        conditional_first: "~drais", "~drions";
        conditional_second: "~drais", "~driez";
        conditional_third: "~drait", "~draient";
        present_subjunctive_first: "~ne";
        present_subjunctive_second: "~nes";
        present_subjunctive_third: "~ne", "~nent";
        imperative: "~ds";
    ),
    // Conçois: 7 words (reçois, aperçois, déçois, conçois, perçois, reperçois).
    conjugation!("conçois", 7, "~çois";
        present_first: "~çois";
        present_second: "~çois";
        present_third: "~çoit", "~çoivent";
        future_first: "~cevrai", "~cevrons";
        future_second: "~cevras", "~cevrez";
        future_third: "~cevra", "~cevront";
        conditional_first: "~cevrais", "~cevrions";
        conditional_second: "~cevrais", "~cevriez";
        conditional_third: "~cevrait", "~cevraient";
        present_subjunctive_first: "~çoive";
        present_subjunctive_second: "~çoives";
        present_subjunctive_third: "~çoive", "~çoivent";
        imperative: "~çois";
    ),
    // Peux: 6 words (peux, veux, reveux, repeux, deux, seux).
    conjugation!("peux", 6, "~e~x";
        present_first: "~e~x";
        present_second: "~e~x";
        present_third: "~e~t", "~e~lent";
        future_first: "~o~drai", "~o~drons";
        future_second: "~o~dras", "~o~drez";
        future_third: "~o~dra", "~o~dront";
        conditional_first: "~o~drais", "~o~drions";
        conditional_second: "~o~drais", "~o~driez";
        conditional_third: "~o~drait", "~o~draient";
        present_subjunctive_first: "~e~ille";
        present_subjunctive_second: "~e~illes";
        present_subjunctive_third: "~e~ille", "~e~illent";
        imperative: "~e~x";
    ),
    // Éclos: 6 words (clos, éclos, déclos, enclos, forclos, reclos).
    conjugation!("éclos", 6, "~os";
        present_first: "~os";
        present_second: "~os";
        present_third: "~ôt", "~osent";
        future_first: "~orai", "~orons";
        future_second: "~oras", "~orez";
        future_third: "~ora", "~oront";
        conditional_first: "~orais", "~orions";
        conditional_second: "~orais", "~oriez";
        conditional_third: "~orait", "~oraient";
        present_subjunctive_first: "~ose";
        present_subjunctive_second: "~oses";
        present_subjunctive_third: "~ose", "~osent";
        imperative: "~os";
    ),
    // Requiers: 6 words (requiers, acquiers, conquiers, enquiers, reconquiers, quiers).
    conjugation!("requiers", 6, "~ie~s";
        present_first: "~ie~s";
        present_second: "~ie~s";
        present_third: "~ie~t", "~iè~ent";
        future_first: "~e~rai", "~e~rons";
        future_second: "~e~ras", "~e~rez";
        future_third: "~e~ra", "~e~ront";
        conditional_first: "~e~rais", "~e~rions";
        conditional_second: "~e~rais", "~e~riez";
        conditional_third: "~e~rait", "~e~raient";
        present_subjunctive_first: "~iè~e";
        present_subjunctive_second: "~iè~es";
        present_subjunctive_third: "~iè~e", "~iè~ent";
        imperative: "~ie~s";
    ),
    // Dissous: 6 words (résous, absous, dissous, autodissous, resous, arsous).
    conjugation!("dissous", 6, "~us";
        present_first: "~us";
        present_second: "~us";
        present_third: "~ut", "~lvent";
        future_first: "~udrai", "~udrons";
        future_second: "~udras", "~udrez";
        future_third: "~udra", "~udront";
        conditional_first: "~udrais", "~udrions";
        conditional_second: "~udrais", "~udriez";
        conditional_third: "~udrait", "~udraient";
        present_subjunctive_first: "~lve";
        present_subjunctive_second: "~lves";
        present_subjunctive_third: "~lve", "~lvent";
        imperative: "~us";
    ),
    // Resve: 6 words (resve, esloigne, brusle, espose, gasche, ruisselle).
    conjugation!("resve", 6, "~";
        present_first: "~";
        present_second: "~s";
        present_third: "~", "~nt";
        future_first: "~roi", "~rons";
        future_second: "~ras", "~rez";
        future_third: "~ra", "~ront";
        conditional_first: "~rois", "~rions";
        conditional_second: "~rois", "~riez";
        conditional_third: "~roit", "~roient";
        present_subjunctive_first: "~";
        present_subjunctive_second: "~s";
        present_subjunctive_third: "~", "~nt";
        imperative: "~";
    ),
    // Vaux: 5 words (vaux, faux, prévaux, équivaux, revaux).
    conjugation!("vaux", 5, "~ux";
        present_first: "~ux";
        present_second: "~ux";
        present_third: "~ut", "~lent";
        future_first: "~udrai", "~udrons";
        future_second: "~udras", "~udrez";
        future_third: "~udra", "~udront";
        conditional_first: "~udrais", "~udrions";
        conditional_second: "~udrais", "~udriez";
        conditional_third: "~udrait", "~udraient";
        present_subjunctive_first: "~ille";
        present_subjunctive_second: "~illes";
        present_subjunctive_third: "~ille", "~illent";
        imperative: "~ux";
    ),
    // Endors: 4 words (dors, endors, rendors, redors).
    conjugation!("endors", 4, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~ment";
        future_first: "~mirai", "~mirons";
        future_second: "~miras", "~mirez";
        future_third: "~mira", "~miront";
        conditional_first: "~mirais", "~mirions";
        conditional_second: "~mirais", "~miriez";
        conditional_third: "~mirait", "~miraient";
        present_subjunctive_first: "~me";
        present_subjunctive_second: "~mes";
        present_subjunctive_third: "~me", "~ment";
        imperative: "~s";
    ),
    // Poursuis: 4 words (vis, poursuis, survis, revis).
    conjugation!("poursuis", 4, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~vent";
        future_first: "~vrai", "~vrons";
        future_second: "~vras", "~vrez";
        future_third: "~vra", "~vront";
        conditional_first: "~vrais", "~vrions";
        conditional_second: "~vrais", "~vriez";
        conditional_third: "~vrait", "~vraient";
        present_subjunctive_first: "~ve";
        present_subjunctive_second: "~ves";
        present_subjunctive_third: "~ve", "~vent";
        imperative: "~s";
    ),
    // Mouds: 4 words (mouds, remouds, émouds, vermouds).
    conjugation!("mouds", 4, "~ds";
        present_first: "~ds";
        present_second: "~ds";
        present_third: "~d", "~lent";
        future_first: "~drai", "~drons";
        future_second: "~dras", "~drez";
        future_third: "~dra", "~dront";
        conditional_first: "~drais", "~drions";
        conditional_second: "~drais", "~driez";
        conditional_third: "~drait", "~draient";
        present_subjunctive_first: "~le";
        present_subjunctive_second: "~les";
        present_subjunctive_third: "~le", "~lent";
        imperative: "~ds";
    ),
    // Ouïs: 4 words (ouïs, amuïs, réouïs, inouïs).
    conjugation!("ouïs", 4, "~uïs";
        present_first: "~uïs|~is";
        present_second: "~uïs|~is";
        present_third: "~uït|~it", "~uïssent|~ient";
        future_first: "~uïrai|~irai|~rrai", "~uïrons|~irons|~rrons";
        future_second: "~uïras|~iras|~rras", "~uïrez|~irez|~rrez";
        future_third: "~uïra|~ira|~rra", "~uïront|~iront|~rront";
        conditional_first: "~uïrais|~irais|~rrais", "~uïrions|~irions|~rrions";
        conditional_second: "~uïrais|~irais|~rrais", "~uïriez|~iriez|~rriez";
        conditional_third: "~uïrait|~irait|~rrait", "~uïraient|~iraient|~rraient";
        present_subjunctive_first: "~uïsse|~ie";
        present_subjunctive_second: "~uïsses|~ies";
        present_subjunctive_third: "~uïsse|~ie", "~uïssent|~ient";
        imperative: "~uïs|~is";
    ),
    // Meus: 4 words (meus, émeus, promeus, remeus).
    conjugation!("meus", 4, "~e~s";
        present_first: "~e~s";
        present_second: "~e~s";
        present_third: "~e~t", "~e~vent";
        future_first: "~o~vrai", "~o~vrons";
        future_second: "~o~vras", "~o~vrez";
        future_third: "~o~vra", "~o~vront";
        conditional_first: "~o~vrais", "~o~vrions";
        conditional_second: "~o~vrais", "~o~vriez";
        conditional_third: "~o~vrait", "~o~vraient";
        present_subjunctive_first: "~e~ve";
        present_subjunctive_second: "~e~ves";
        present_subjunctive_third: "~e~ve", "~e~vent";
        imperative: "~e~s";
    ),
    // Revêts: 4 words (vêts, dévêts, revêts, survêts).
    conjugation!("revêts", 4, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~", "~ent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~e";
        present_subjunctive_second: "~es";
        present_subjunctive_third: "~e", "~ent";
        imperative: "~s";
    ),
    // Vois: 3 words (vois, revois, entrevois).
    conjugation!("vois", 3, "~ois";
        present_first: "~ois";
        present_second: "~ois";
        present_third: "~oit", "~oient";
        future_first: "~errai", "~errons";
        future_second: "~erras", "~errez";
        future_third: "~erra", "~erront";
        conditional_first: "~errais", "~errions";
        conditional_second: "~errais", "~erriez";
        conditional_third: "~errait", "~erraient";
        present_subjunctive_first: "~oie";
        present_subjunctive_second: "~oies";
        present_subjunctive_third: "~oie", "~oient";
        imperative: "~ois";
    ),
    // Complais: 3 words (plais, déplais, complais).
    conjugation!("complais", 3, "~is";
        present_first: "~is";
        present_second: "~is";
        present_third: "~ît|~it", "~isent";
        future_first: "~irai", "~irons";
        future_second: "~iras", "~irez";
        future_third: "~ira", "~iront";
        conditional_first: "~irais", "~irions";
        conditional_second: "~irais", "~iriez";
        conditional_third: "~irait", "~iraient";
        present_subjunctive_first: "~ise";
        present_subjunctive_second: "~ises";
        present_subjunctive_third: "~ise", "~isent";
        imperative: "~is";
    ),
    // Dois: 3 words (dois, redois, ramentois).
    conjugation!("dois", 3, "~ois";
        present_first: "~ois";
        present_second: "~ois";
        present_third: "~oit", "~oivent";
        future_first: "~evrai", "~evrons";
        future_second: "~evras", "~evrez";
        future_third: "~evra", "~evront";
        conditional_first: "~evrais", "~evrions";
        conditional_second: "~evrais", "~evriez";
        conditional_third: "~evrait", "~evraient";
        present_subjunctive_first: "~oive";
        present_subjunctive_second: "~oives";
        present_subjunctive_third: "~oive", "~oivent";
        imperative: "~ois";
    ),
    // Sers: 3 words (sers, ressers, dessers).
    conjugation!("sers", 3, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~vent";
        future_first: "~virai", "~virons";
        future_second: "~viras", "~virez";
        future_third: "~vira", "~viront";
        conditional_first: "~virais", "~virions";
        conditional_second: "~virais", "~viriez";
        conditional_third: "~virait", "~viraient";
        present_subjunctive_first: "~ve";
        present_subjunctive_second: "~ves";
        present_subjunctive_third: "~ve", "~vent";
        imperative: "~s";
    ),
    // Recouds: 3 words (couds, recouds, découds).
    conjugation!("recouds", 3, "~ds";
        present_first: "~ds";
        present_second: "~ds";
        present_third: "~d", "~sent";
        future_first: "~drai", "~drons";
        future_second: "~dras", "~drez";
        future_third: "~dra", "~dront";
        conditional_first: "~drais", "~drions";
        conditional_second: "~drais", "~driez";
        conditional_third: "~drait", "~draient";
        present_subjunctive_first: "~se";
        present_subjunctive_second: "~ses";
        present_subjunctive_third: "~se", "~sent";
        imperative: "~ds";
    ),
    // Bous: 3 words (bous, rebous, débous).
    conjugation!("bous", 3, "~s";
        present_first: "~s";
        present_second: "~s";
        present_third: "~t", "~illent";
        future_first: "~illirai", "~illirons";
        future_second: "~illiras", "~illirez";
        future_third: "~illira", "~illiront";
        conditional_first: "~illirais", "~illirions";
        conditional_second: "~illirais", "~illiriez";
        conditional_third: "~illirait", "~illiraient";
        present_subjunctive_first: "~ille";
        present_subjunctive_second: "~illes";
        present_subjunctive_third: "~ille", "~illent";
        imperative: "~s";
    ),
    // Hai: 3 words (rai, hai, réai).
    conjugation!("hai", 3, "~ai";
        present_first: "~ai";
        present_second: "~as";
        present_third: "~a", "~ont";
        future_first: "~aurai", "~aurons";
        future_second: "~auras", "~aurez";
        future_third: "~aura", "~auront";
        conditional_first: "~aurais", "~aurions";
        conditional_second: "~aurais", "~auriez";
        conditional_third: "~aurait", "~auraient";
        present_subjunctive_first: "~aie";
        present_subjunctive_second: "~aies";
        present_subjunctive_third: "~ait", "~aient";
        imperative: "~aie";
    ),
];
