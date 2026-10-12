//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's auxiliary says):
/// `present_first` (sg, pl)
/// `present_second` (sg, pl)
/// `present_third` (sg, pl)
/// `imperfect_first` (sg, pl)
/// `imperfect_second` (sg, pl)
/// `imperfect_third` (sg, pl)
/// `past_historic_first` (sg, pl)
/// `past_historic_second` (sg, pl)
/// `past_historic_third` (sg, pl)
/// `future_first` (sg, pl)
/// `future_second` (sg, pl)
/// `future_third` (sg, pl)
/// `conditional_first` (sg, pl)
/// `conditional_second` (sg, pl)
/// `conditional_third` (sg, pl)
/// `present_subjunctive_first` (sg, pl)
/// `present_subjunctive_second` (sg, pl)
/// `present_subjunctive_third` (sg, pl)
/// `imperfect_subjunctive_first` (sg, pl)
/// `imperfect_subjunctive_second` (sg, pl)
/// `imperfect_subjunctive_third` (sg, pl)
/// `imperative` (tu, lei, noi, voi, loro)
/// `negative_imperative` (tu, lei, noi, voi, loro)
/// `infinitive`
/// `gerund`
/// `participle` (pres, past)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     present_first: $c0:tt, $c1:tt;
     present_second: $c2:tt, $c3:tt;
     present_third: $c4:tt, $c5:tt;
     imperfect_first: $c6:tt, $c7:tt;
     imperfect_second: $c8:tt, $c9:tt;
     imperfect_third: $c10:tt, $c11:tt;
     past_historic_first: $c12:tt, $c13:tt;
     past_historic_second: $c14:tt, $c15:tt;
     past_historic_third: $c16:tt, $c17:tt;
     future_first: $c18:tt, $c19:tt;
     future_second: $c20:tt, $c21:tt;
     future_third: $c22:tt, $c23:tt;
     conditional_first: $c24:tt, $c25:tt;
     conditional_second: $c26:tt, $c27:tt;
     conditional_third: $c28:tt, $c29:tt;
     present_subjunctive_first: $c30:tt, $c31:tt;
     present_subjunctive_second: $c32:tt, $c33:tt;
     present_subjunctive_third: $c34:tt, $c35:tt;
     imperfect_subjunctive_first: $c36:tt, $c37:tt;
     imperfect_subjunctive_second: $c38:tt, $c39:tt;
     imperfect_subjunctive_third: $c40:tt, $c41:tt;
     imperative: $c42:tt, $c43:tt, $c44:tt, $c45:tt, $c46:tt;
     negative_imperative: $c47:tt, $c48:tt, $c49:tt, $c50:tt, $c51:tt;
     infinitive: $c52:tt;
     gerund: $c53:tt;
     participle: $c54:tt, $c55:tt;
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
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
                flexion::form!($c48),
                flexion::form!($c49),
                flexion::form!($c50),
                flexion::form!($c51),
                flexion::form!($c52),
                flexion::form!($c53),
                flexion::form!($c54),
                flexion::form!($c55),
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `anzare`: 3,764 words.
    Anzare,
    /// Like `tenarsi`: 994 words.
    Tenarsi,
    /// Like `ocare`: 623 words.
    Ocare,
    /// Like `uscire`: 562 words.
    Uscire,
    /// Like `doppiare`: 516 words.
    Doppiare,
    /// Like `nciare`: 484 words.
    Nciare,
    /// Like `oscarsi`: 206 words.
    Oscarsi,
    /// Like `rognirsi`: 195 words.
    Rognirsi,
    /// Like `pigliarsi`: 136 words.
    Pigliarsi,
    /// Like `sciarsi`: 107 words.
    Sciarsi,
    /// Like `essere`: 85 words.
    Essere,
    /// Like `ingere`: 52 words.
    Ingere,
    /// Like `ndere`: 46 words.
    Ndere,
    /// Like `dersi`: 43 words.
    Dersi,
    /// Like `tire`: 40 words.
    Tire,
    /// Like `cendere`: 35 words.
    Cendere,
    /// Like `ndersi`: 28 words.
    Ndersi,
    /// Like `tingersi`: 25 words.
    Tingersi,
    /// Like `mettere`: 23 words.
    Mettere,
    /// Like `urgere`: 23 words.
    Urgere,
    /// Like `reggere`: 22 words.
    Reggere,
    /// Like `fare`: 22 words.
    Fare,
    /// Like `porre`: 21 words.
    Porre,
    /// Like `dipartirsi`: 19 words.
    Dipartirsi,
    /// Like `farsi`: 19 words.
    Farsi,
    /// Like `battere`: 18 words.
    Battere,
    /// Like `sviare`: 15 words.
    Sviare,
    /// Like `giostrare`: 15 words.
    Giostrare,
    /// Like `dire`: 14 words.
    Dire,
    /// Like `adere`: 14 words.
    Adere,
    /// Like `aurire`: 13 words.
    Aurire,
    /// Like `ncere`: 13 words.
    Ncere,
    /// Like `arsi`: 13 words.
    Arsi,
    /// Like `ogliere`: 12 words.
    Ogliere,
    /// Like `orrere`: 11 words.
    Orrere,
    /// Like `argere`: 11 words.
    Argere,
    /// Like `uggersi`: 11 words.
    Uggersi,
    /// Like `onoscere`: 10 words.
    Onoscere,
    /// Like `rivere`: 10 words.
    Rivere,
    /// Like `frire`: 10 words.
    Frire,
    /// Like `venire`: 9 words.
    Venire,
    /// Like `piersi`: 9 words.
    Piersi,
    /// Like `sistere`: 9 words.
    Sistere,
    /// Like `tenere`: 8 words.
    Tenere,
    /// Like `prefiggere`: 8 words.
    Prefiggere,
    /// Like `endersi`: 8 words.
    Endersi,
    /// Like `durre`: 8 words.
    Durre,
    /// Like `arre`: 8 words.
    Arre,
    /// Like `aurirsi`: 7 words.
    Aurirsi,
    /// Like `egnere`: 7 words.
    Egnere,
    /// Like `nettere`: 6 words.
    Nettere,
    /// Like `tacere`: 6 words.
    Tacere,
    /// Like `vedere`: 6 words.
    Vedere,
    /// Like `rispondere`: 5 words.
    Rispondere,
    /// Like `risolvere`: 5 words.
    Risolvere,
    /// Like `dirsi`: 5 words.
    Dirsi,
    /// Like `rigere`: 5 words.
    Rigere,
    /// Like `sapere`: 5 words.
    Sapere,
    /// Like `ridare`: 5 words.
    Ridare,
    /// Like `egliersi`: 5 words.
    Egliersi,
    /// Like `ettersi`: 5 words.
    Ettersi,
    /// Like `sviarsi`: 5 words.
    Sviarsi,
    /// Like `overe`: 5 words.
    Overe,
    /// Like `tinguere`: 5 words.
    Tinguere,
    /// Like `cutere`: 5 words.
    Cutere,
    /// Like `primere`: 5 words.
    Primere,
    /// Like `ncersi`: 5 words.
    Ncersi,
    /// Like `fondere`: 4 words.
    Fondere,
    /// Like `creare`: 4 words.
    Creare,
    /// Like `rolare`: 4 words.
    Rolare,
    /// Like `ttenersi`: 4 words.
    Ttenersi,
    /// Like `mettersi`: 4 words.
    Mettersi,
    /// Like `oire`: 4 words.
    Oire,
    /// Like `riversi`: 4 words.
    Riversi,
    /// Like `retrocedere`: 4 words.
    Retrocedere,
    /// Like `vvedere`: 4 words.
    Vvedere,
    /// Like `ompire`: 4 words.
    Ompire,
    /// Like `nascondersi`: 4 words.
    Nascondersi,
    /// Like `scere`: 4 words.
    Scere,
    /// Like `noiare`: 4 words.
    Noiare,
    /// Like `arire`: 4 words.
    Arire,
    /// Like `lire`: 4 words.
    Lire,
    /// Like `sedere`: 4 words.
    Sedere,
    /// Like `oscersi`: 4 words.
    Oscersi,
    /// Like `mergersi`: 4 words.
    Mergersi,
    /// Like `andere`: 4 words.
    Andere,
    /// Like `dursi`: 4 words.
    Dursi,
    /// Like `prirsi`: 4 words.
    Prirsi,
    /// Like `ducere`: 3 words.
    Ducere,
    /// Like `hiedere`: 3 words.
    Hiedere,
    /// Like `scuotere`: 3 words.
    Scuotere,
    /// Like `tringere`: 3 words.
    Tringere,
    /// Like `ercepire`: 3 words.
    Ercepire,
    /// Like `pellere`: 3 words.
    Pellere,
    /// Like `acersi`: 3 words.
    Acersi,
    /// Like `scuotersi`: 3 words.
    Scuotersi,
    /// Like `ire`: 1 words.
    Ire,
    /// Like `are`: 1 words.
    Are,
    /// Like `ere`: 1 words.
    Ere,
    /// Like `rre`: 1 words.
    Rre,
    /// Like `rsi`: 1 words.
    Rsi,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 101] = [
        Conjugation::Anzare,
        Conjugation::Tenarsi,
        Conjugation::Ocare,
        Conjugation::Uscire,
        Conjugation::Doppiare,
        Conjugation::Nciare,
        Conjugation::Oscarsi,
        Conjugation::Rognirsi,
        Conjugation::Pigliarsi,
        Conjugation::Sciarsi,
        Conjugation::Essere,
        Conjugation::Ingere,
        Conjugation::Ndere,
        Conjugation::Dersi,
        Conjugation::Tire,
        Conjugation::Cendere,
        Conjugation::Ndersi,
        Conjugation::Tingersi,
        Conjugation::Mettere,
        Conjugation::Urgere,
        Conjugation::Reggere,
        Conjugation::Fare,
        Conjugation::Porre,
        Conjugation::Dipartirsi,
        Conjugation::Farsi,
        Conjugation::Battere,
        Conjugation::Sviare,
        Conjugation::Giostrare,
        Conjugation::Dire,
        Conjugation::Adere,
        Conjugation::Aurire,
        Conjugation::Ncere,
        Conjugation::Arsi,
        Conjugation::Ogliere,
        Conjugation::Orrere,
        Conjugation::Argere,
        Conjugation::Uggersi,
        Conjugation::Onoscere,
        Conjugation::Rivere,
        Conjugation::Frire,
        Conjugation::Venire,
        Conjugation::Piersi,
        Conjugation::Sistere,
        Conjugation::Tenere,
        Conjugation::Prefiggere,
        Conjugation::Endersi,
        Conjugation::Durre,
        Conjugation::Arre,
        Conjugation::Aurirsi,
        Conjugation::Egnere,
        Conjugation::Nettere,
        Conjugation::Tacere,
        Conjugation::Vedere,
        Conjugation::Rispondere,
        Conjugation::Risolvere,
        Conjugation::Dirsi,
        Conjugation::Rigere,
        Conjugation::Sapere,
        Conjugation::Ridare,
        Conjugation::Egliersi,
        Conjugation::Ettersi,
        Conjugation::Sviarsi,
        Conjugation::Overe,
        Conjugation::Tinguere,
        Conjugation::Cutere,
        Conjugation::Primere,
        Conjugation::Ncersi,
        Conjugation::Fondere,
        Conjugation::Creare,
        Conjugation::Rolare,
        Conjugation::Ttenersi,
        Conjugation::Mettersi,
        Conjugation::Oire,
        Conjugation::Riversi,
        Conjugation::Retrocedere,
        Conjugation::Vvedere,
        Conjugation::Ompire,
        Conjugation::Nascondersi,
        Conjugation::Scere,
        Conjugation::Noiare,
        Conjugation::Arire,
        Conjugation::Lire,
        Conjugation::Sedere,
        Conjugation::Oscersi,
        Conjugation::Mergersi,
        Conjugation::Andere,
        Conjugation::Dursi,
        Conjugation::Prirsi,
        Conjugation::Ducere,
        Conjugation::Hiedere,
        Conjugation::Scuotere,
        Conjugation::Tringere,
        Conjugation::Ercepire,
        Conjugation::Pellere,
        Conjugation::Acersi,
        Conjugation::Scuotersi,
        Conjugation::Ire,
        Conjugation::Are,
        Conjugation::Ere,
        Conjugation::Rre,
        Conjugation::Rsi,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 101] = [
    // Anzare: 3,764 words (vulnerare, nuotare, ordinare, rubare, volare, ballare).
    conjugation!("anzare", 3764, "~are";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~i", "~iamo";
        present_subjunctive_second: "~i", "~iate";
        present_subjunctive_third: "~i", "~ino";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~a", referred, "~iamo", "~ate", "~ino";
        negative_imperative: "~are", referred, "~iamo", "~ate", referred;
        infinitive: "~are";
        gerund: "~ando";
        participle: "~ante", "~ato";
    ),
    // Tenarsi: 994 words (tirarsi, celarsi, cresimarsi, incontrarsi, organizzarsi, svilupparsi).
    conjugation!("tenarsi", 994, "~arsi";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~i", "~iamo";
        present_subjunctive_second: "~i", "~iate";
        present_subjunctive_third: "~i", "~ino";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~ati", referred, "~iamoci", "~atevi", "~ino";
        negative_imperative: "~arti|~are", referred, "~iamoci|~iamo", "~atevi|~ate", referred;
        infinitive: "~arsi";
        gerund: "~andosi";
        participle: "~antesi", "~atosi";
    ),
    // Ocare: 623 words (vocare, nevicare, zigzagare, zincare, zoppicare, duplicare).
    conjugation!("ocare", 623, "~are";
        present_first: "~o", "~hiamo";
        present_second: "~hi", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~herò", "~heremo";
        future_second: "~herai", "~herete";
        future_third: "~herà", "~heranno";
        conditional_first: "~herei", "~heremmo";
        conditional_second: "~heresti", "~hereste";
        conditional_third: "~herebbe", "~herebbero";
        present_subjunctive_first: "~hi", "~hiamo";
        present_subjunctive_second: "~hi", "~hiate";
        present_subjunctive_third: "~hi", "~hino";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~a", referred, "~hiamo", "~ate", "~hino";
        negative_imperative: "~are", referred, "~hiamo", "~ate", referred;
        infinitive: "~are";
        gerund: "~ando";
        participle: "~ante", "~ato";
    ),
    // Uscire: 562 words (fire, fallire, punire, morire, zittire, agire).
    conjugation!("uscire", 562, "~ire";
        present_first: "~isco", "~iamo";
        present_second: "~isci", "~ite";
        present_third: "~isce", "~iscono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~ì", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~isca", "~iamo";
        present_subjunctive_second: "~isca", "~iate";
        present_subjunctive_third: "~isca", "~iscano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~isci", referred, "~iamo", "~ite", "~iscano";
        negative_imperative: "~ire", referred, "~iamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~endo";
        participle: "~ente", "~ito";
    ),
    // Doppiare: 516 words (odiare, tagliare, variare, gonfiare, copiare, mediare).
    conjugation!("doppiare", 516, "~are";
        present_first: "~o", "~amo";
        present_second: "~", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~", "~amo";
        present_subjunctive_second: "~", "~ate";
        present_subjunctive_third: "~", "~no";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~a", referred, "~amo", "~ate", "~no";
        negative_imperative: "~are", referred, "~amo", "~ate", referred;
        infinitive: "~are";
        gerund: "~ando";
        participle: "~ante", "~ato";
    ),
    // Nciare: 484 words (lasciare, baciare, elogiare, enunciare, guerreggiare, gareggiare).
    conjugation!("nciare", 484, "~iare";
        present_first: "~io", "~iamo";
        present_second: "~i", "~iate";
        present_third: "~ia", "~iano";
        imperfect_first: "~iavo", "~iavamo";
        imperfect_second: "~iavi", "~iavate";
        imperfect_third: "~iava", "~iavano";
        past_historic_first: "~iai", "~iammo";
        past_historic_second: "~iasti", "~iaste";
        past_historic_third: "~iò", "~iarono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~i", "~iamo";
        present_subjunctive_second: "~i", "~iate";
        present_subjunctive_third: "~i", "~ino";
        imperfect_subjunctive_first: "~iassi", "~iassimo";
        imperfect_subjunctive_second: "~iassi", "~iaste";
        imperfect_subjunctive_third: "~iasse", "~iassero";
        imperative: "~ia", referred, "~iamo", "~iate", "~ino";
        negative_imperative: "~iare", referred, "~iamo", "~iate", referred;
        infinitive: "~iare";
        gerund: "~iando";
        participle: "~iante", "~iato";
    ),
    // Oscarsi: 206 words (obbligarsi, drogarsi, purificarsi, classificarsi, identificarsi, mummificarsi).
    conjugation!("oscarsi", 206, "~arsi";
        present_first: "~o", "~hiamo";
        present_second: "~hi", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~herò", "~heremo";
        future_second: "~herai", "~herete";
        future_third: "~herà", "~heranno";
        conditional_first: "~herei", "~heremmo";
        conditional_second: "~heresti", "~hereste";
        conditional_third: "~herebbe", "~herebbero";
        present_subjunctive_first: "~hi", "~hiamo";
        present_subjunctive_second: "~hi", "~hiate";
        present_subjunctive_third: "~hi", "~hino";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~ati", referred, "~hiamoci", "~atevi", "~hino";
        negative_imperative: "~arti|~are", referred, "~hiamoci|~hiamo", "~atevi|~ate", referred;
        infinitive: "~arsi";
        gerund: "~andosi";
        participle: "~antesi", "~atosi";
    ),
    // Rognirsi: 195 words (ferirsi, unirsi, garantirsi, zittirsi, pulirsi, capirsi).
    conjugation!("rognirsi", 195, "~irsi";
        present_first: "~isco", "~iamo";
        present_second: "~isci", "~ite";
        present_third: "~isce", "~iscono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~ì", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~isca", "~iamo";
        present_subjunctive_second: "~isca", "~iate";
        present_subjunctive_third: "~isca", "~iscano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~isciti", referred, "~iamoci", "~itevi", "~iscano";
        negative_imperative: "~irti|~ire", referred, "~iamoci|~iamo", "~itevi|~ite", referred;
        infinitive: "~irsi";
        gerund: "~endosi";
        participle: "~entesi", "~itosi";
    ),
    // Pigliarsi: 136 words (svegliarsi, meravigliarsi, tagliarsi, umiliarsi, odiarsi, squagliarsi).
    conjugation!("pigliarsi", 136, "~arsi";
        present_first: "~o", "~amo";
        present_second: "~", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~", "~amo";
        present_subjunctive_second: "~", "~ate";
        present_subjunctive_third: "~", "~no";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~ati", referred, "~amoci", "~atevi", "~no";
        negative_imperative: "~arti|~are", referred, "~amoci|~amo", "~atevi|~ate", referred;
        infinitive: "~arsi";
        gerund: "~andosi";
        participle: "~antesi", "~atosi";
    ),
    // Sciarsi: 107 words (baciarsi, ormeggiarsi, slanciarsi, squarciarsi, danneggiarsi, ciucciarsi).
    conjugation!("sciarsi", 107, "~iarsi";
        present_first: "~io", "~iamo";
        present_second: "~i", "~iate";
        present_third: "~ia", "~iano";
        imperfect_first: "~iavo", "~iavamo";
        imperfect_second: "~iavi", "~iavate";
        imperfect_third: "~iava", "~iavano";
        past_historic_first: "~iai", "~iammo";
        past_historic_second: "~iasti", "~iaste";
        past_historic_third: "~iò", "~iarono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~i", "~iamo";
        present_subjunctive_second: "~i", "~iate";
        present_subjunctive_third: "~i", "~ino";
        imperfect_subjunctive_first: "~iassi", "~iassimo";
        imperfect_subjunctive_second: "~iassi", "~iaste";
        imperfect_subjunctive_third: "~iasse", "~iassero";
        imperative: "~iati", referred, "~iamoci", "~iatevi", "~ino";
        negative_imperative: "~iarti|~iare", referred, "~iamoci|~iamo", "~iatevi|~iate", referred;
        infinitive: "~iarsi";
        gerund: "~iandosi";
        participle: "~iantesi", "~iatosi";
    ),
    // Essere: 85 words (essere, rompere, bere, volere, nascere, dicere).
    conjugation!("essere", 85, "~ere";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~etti", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~ette", "~ettero";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", referred, "~iamo", "~ete", "~ano";
        negative_imperative: "~ere", referred, "~iamo", "~ete", referred;
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~uto";
    ),
    // Ingere: 52 words (convergere, volgere, ungere, cingere, svolgere, tingere).
    conjugation!("ingere", 52, "~gere";
        present_first: "~go", "~giamo";
        present_second: "~gi", "~gete";
        present_third: "~ge", "~gono";
        imperfect_first: "~gevo", "~gevamo";
        imperfect_second: "~gevi", "~gevate";
        imperfect_third: "~geva", "~gevano";
        past_historic_first: "~si", "~gemmo";
        past_historic_second: "~gesti", "~geste";
        past_historic_third: "~se", "~sero";
        future_first: "~gerò", "~geremo";
        future_second: "~gerai", "~gerete";
        future_third: "~gerà", "~geranno";
        conditional_first: "~gerei", "~geremmo";
        conditional_second: "~geresti", "~gereste";
        conditional_third: "~gerebbe", "~gerebbero";
        present_subjunctive_first: "~ga", "~giamo";
        present_subjunctive_second: "~ga", "~giate";
        present_subjunctive_third: "~ga", "~gano";
        imperfect_subjunctive_first: "~gessi", "~gessimo";
        imperfect_subjunctive_second: "~gessi", "~geste";
        imperfect_subjunctive_third: "~gesse", "~gessero";
        imperative: "~gi", referred, "~giamo", "~gete", "~gano";
        negative_imperative: "~gere", referred, "~giamo", "~gete", referred;
        infinitive: "~gere";
        gerund: "~gendo";
        participle: "~gente", "~to";
    ),
    // Ndere: 46 words (uccidere, mordere, radere, ledere, evadere, occidere).
    conjugation!("ndere", 46, "~dere";
        present_first: "~do", "~diamo";
        present_second: "~di", "~dete";
        present_third: "~de", "~dono";
        imperfect_first: "~devo", "~devamo";
        imperfect_second: "~devi", "~devate";
        imperfect_third: "~deva", "~devano";
        past_historic_first: "~si", "~demmo";
        past_historic_second: "~desti", "~deste";
        past_historic_third: "~se", "~sero";
        future_first: "~derò", "~deremo";
        future_second: "~derai", "~derete";
        future_third: "~derà", "~deranno";
        conditional_first: "~derei", "~deremmo";
        conditional_second: "~deresti", "~dereste";
        conditional_third: "~derebbe", "~derebbero";
        present_subjunctive_first: "~da", "~diamo";
        present_subjunctive_second: "~da", "~diate";
        present_subjunctive_third: "~da", "~dano";
        imperfect_subjunctive_first: "~dessi", "~dessimo";
        imperfect_subjunctive_second: "~dessi", "~deste";
        imperfect_subjunctive_third: "~desse", "~dessero";
        imperative: "~di", referred, "~diamo", "~dete", "~dano";
        negative_imperative: "~dere", referred, "~diamo", "~dete", referred;
        infinitive: "~dere";
        gerund: "~dendo";
        participle: "~dente", "~so";
    ),
    // Dersi: 43 words (muoversi, credersi, godersi, rompersi, sedersi, vendersi).
    conjugation!("dersi", 43, "~ersi";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~etti", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~ette", "~ettero";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~iti", referred, "~iamoci", "~etevi", "~ano";
        negative_imperative: "~erti", referred, "~iamoci", "~etevi", referred;
        infinitive: "~ersi";
        gerund: "~endosi";
        participle: "~entesi", "~utosi";
    ),
    // Tire: 40 words (empire, cucire, fuggire, vestire, bollire, mentire).
    conjugation!("tire", 40, "~ire";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ite";
        present_third: "~e", "~ono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~ì", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~i", referred, "~iamo", "~ite", "~ano";
        negative_imperative: "~ire", referred, "~iamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~endo";
        participle: "~ente", "~ito";
    ),
    // Cendere: 35 words (dipendere, offendere, tendere, dispendere, soppendere, endere).
    conjugation!("cendere", 35, "~ndere";
        present_first: "~ndo", "~ndiamo";
        present_second: "~ndi", "~ndete";
        present_third: "~nde", "~ndono";
        imperfect_first: "~ndevo", "~ndevamo";
        imperfect_second: "~ndevi", "~ndevate";
        imperfect_third: "~ndeva", "~ndevano";
        past_historic_first: "~si", "~ndemmo";
        past_historic_second: "~ndesti", "~ndeste";
        past_historic_third: "~se", "~sero";
        future_first: "~nderò", "~nderemo";
        future_second: "~nderai", "~nderete";
        future_third: "~nderà", "~nderanno";
        conditional_first: "~nderei", "~nderemmo";
        conditional_second: "~nderesti", "~ndereste";
        conditional_third: "~nderebbe", "~nderebbero";
        present_subjunctive_first: "~nda", "~ndiamo";
        present_subjunctive_second: "~nda", "~ndiate";
        present_subjunctive_third: "~nda", "~ndano";
        imperfect_subjunctive_first: "~ndessi", "~ndessimo";
        imperfect_subjunctive_second: "~ndessi", "~ndeste";
        imperfect_subjunctive_third: "~ndesse", "~ndessero";
        imperative: "~ndi", referred, "~ndiamo", "~ndete", "~ndano";
        negative_imperative: "~ndere", referred, "~ndiamo", "~ndete", referred;
        infinitive: "~ndere";
        gerund: "~ndendo";
        participle: "~ndente", "~so";
    ),
    // Ndersi: 28 words (vedersi, uccidersi, radersi, mordersi, fondersi, elidersi).
    conjugation!("ndersi", 28, "~dersi";
        present_first: "~do", "~diamo";
        present_second: "~di", "~dete";
        present_third: "~de", "~dono";
        imperfect_first: "~devo", "~devamo";
        imperfect_second: "~devi", "~devate";
        imperfect_third: "~deva", "~devano";
        past_historic_first: "~si", "~demmo";
        past_historic_second: "~desti", "~deste";
        past_historic_third: "~se", "~sero";
        future_first: "~derò", "~deremo";
        future_second: "~derai", "~derete";
        future_third: "~derà", "~deranno";
        conditional_first: "~derei", "~deremmo";
        conditional_second: "~deresti", "~dereste";
        conditional_third: "~derebbe", "~derebbero";
        present_subjunctive_first: "~da", "~diamo";
        present_subjunctive_second: "~da", "~diate";
        present_subjunctive_third: "~da", "~dano";
        imperfect_subjunctive_first: "~dessi", "~dessimo";
        imperfect_subjunctive_second: "~dessi", "~deste";
        imperfect_subjunctive_third: "~desse", "~dessero";
        imperative: "~diti", referred, "~diamoci", "~detevi", "~dano";
        negative_imperative: "~derti|~dere", referred, "~diamoci|~diamo", "~detevi|~dete", referred;
        infinitive: "~dersi";
        gerund: "~dendosi";
        participle: "~dentesi", "~sosi";
    ),
    // Tingersi: 25 words (svolgersi, ergersi, ungersi, volgersi, erigersi, pungersi).
    conjugation!("tingersi", 25, "~gersi";
        present_first: "~go", "~giamo";
        present_second: "~gi", "~gete";
        present_third: "~ge", "~gono";
        imperfect_first: "~gevo", "~gevamo";
        imperfect_second: "~gevi", "~gevate";
        imperfect_third: "~geva", "~gevano";
        past_historic_first: "~si", "~gemmo";
        past_historic_second: "~gesti", "~geste";
        past_historic_third: "~se", "~sero";
        future_first: "~gerò", "~geremo";
        future_second: "~gerai", "~gerete";
        future_third: "~gerà", "~geranno";
        conditional_first: "~gerei", "~geremmo";
        conditional_second: "~geresti", "~gereste";
        conditional_third: "~gerebbe", "~gerebbero";
        present_subjunctive_first: "~ga", "~giamo";
        present_subjunctive_second: "~ga", "~giate";
        present_subjunctive_third: "~ga", "~gano";
        imperfect_subjunctive_first: "~gessi", "~gessimo";
        imperfect_subjunctive_second: "~gessi", "~geste";
        imperfect_subjunctive_third: "~gesse", "~gessero";
        imperative: "~giti", referred, "~giamoci", "~getevi", "~gano";
        negative_imperative: "~gerti|~gere", referred, "~giamoci|~giamo", "~getevi|~gete", referred;
        infinitive: "~gersi";
        gerund: "~gendosi";
        participle: "~gentesi", "~tosi";
    ),
    // Mettere: 23 words (mettere, fedecommettere, emettere, omettere, ommettere, promettere).
    conjugation!("mettere", 23, "~ettere";
        present_first: "~etto", "~ettiamo";
        present_second: "~etti", "~ettete";
        present_third: "~ette", "~ettono";
        imperfect_first: "~ettevo", "~ettevamo";
        imperfect_second: "~ettevi", "~ettevate";
        imperfect_third: "~etteva", "~ettevano";
        past_historic_first: "~isi", "~ettemmo";
        past_historic_second: "~ettesti", "~etteste";
        past_historic_third: "~ise", "~isero";
        future_first: "~etterò", "~etteremo";
        future_second: "~etterai", "~etterete";
        future_third: "~etterà", "~etteranno";
        conditional_first: "~etterei", "~etteremmo";
        conditional_second: "~etteresti", "~ettereste";
        conditional_third: "~etterebbe", "~etterebbero";
        present_subjunctive_first: "~etta", "~ettiamo";
        present_subjunctive_second: "~etta", "~ettiate";
        present_subjunctive_third: "~etta", "~ettano";
        imperfect_subjunctive_first: "~ettessi", "~ettessimo";
        imperfect_subjunctive_second: "~ettessi", "~etteste";
        imperfect_subjunctive_third: "~ettesse", "~ettessero";
        imperative: "~etti", referred, "~ettiamo", "~ettete", "~ettano";
        negative_imperative: "~ettere", referred, "~ettiamo", "~ettete", referred;
        infinitive: "~ettere";
        gerund: "~ettendo";
        participle: "~ettente", "~esso";
    ),
    // Urgere: 23 words (tangere, volvere, vertere, pedere, erompere, fervere).
    conjugation!("urgere", 23, "~ere";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~etti", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~ette", "~ettero";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", "~a", "~iamo", "~ete", "~ano";
        negative_imperative: "~ere", "~a", "~iamo", "~ete", "~ano";
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~uto";
    ),
    // Reggere: 22 words (leggere, eleggere, ereggere, iggere, teggere, uggere).
    conjugation!("reggere", 22, "~ggere";
        present_first: "~ggo", "~ggiamo";
        present_second: "~ggi", "~ggete";
        present_third: "~gge", "~ggono";
        imperfect_first: "~ggevo", "~ggevamo";
        imperfect_second: "~ggevi", "~ggevate";
        imperfect_third: "~ggeva", "~ggevano";
        past_historic_first: "~ssi", "~ggemmo";
        past_historic_second: "~ggesti", "~ggeste";
        past_historic_third: "~sse", "~ssero";
        future_first: "~ggerò", "~ggeremo";
        future_second: "~ggerai", "~ggerete";
        future_third: "~ggerà", "~ggeranno";
        conditional_first: "~ggerei", "~ggeremmo";
        conditional_second: "~ggeresti", "~ggereste";
        conditional_third: "~ggerebbe", "~ggerebbero";
        present_subjunctive_first: "~gga", "~ggiamo";
        present_subjunctive_second: "~gga", "~ggiate";
        present_subjunctive_third: "~gga", "~ggano";
        imperfect_subjunctive_first: "~ggessi", "~ggessimo";
        imperfect_subjunctive_second: "~ggessi", "~ggeste";
        imperfect_subjunctive_third: "~ggesse", "~ggessero";
        imperative: "~ggi", referred, "~ggiamo", "~ggete", "~ggano";
        negative_imperative: "~ggere", referred, "~ggiamo", "~ggete", referred;
        infinitive: "~ggere";
        gerund: "~ggendo";
        participle: "~ggente", "~tto";
    ),
    // Fare: 22 words (fare, disfare, rifare, tumefare, strafare, putrefare).
    conjugation!("fare", 22, "~are";
        present_first: "~accio", "~acciamo";
        present_second: "~ai", "~ate";
        present_third: "~à", "~anno";
        imperfect_first: "~acevo", "~acevamo";
        imperfect_second: "~acevi", "~acevate";
        imperfect_third: "~aceva", "~acevano";
        past_historic_first: "~eci", "~acemmo";
        past_historic_second: "~acesti", "~aceste";
        past_historic_third: "~ece", "~ecero";
        future_first: "~arò", "~aremo";
        future_second: "~arai", "~arete";
        future_third: "~arà", "~aranno";
        conditional_first: "~arei", "~aremmo";
        conditional_second: "~aresti", "~areste";
        conditional_third: "~arebbe", "~arebbero";
        present_subjunctive_first: "~accia", "~acciamo";
        present_subjunctive_second: "~accia", "~acciate";
        present_subjunctive_third: "~accia", "~acciano";
        imperfect_subjunctive_first: "~acessi", "~acessimo";
        imperfect_subjunctive_second: "~acessi", "~aceste";
        imperfect_subjunctive_third: "~acesse", "~acessero";
        imperative: "~ai|~a'", referred, "~acciamo", "~ate", "~acciano";
        negative_imperative: "~are", referred, "~acciamo", "~ate", referred;
        infinitive: "~are";
        gerund: "~acendo";
        participle: "~acente", "~atto";
    ),
    // Porre: 21 words (torre, ovideporre, porre, comporre, pporre, proporre).
    conjugation!("porre", 21, "~rre";
        present_first: "~ngo", "~niamo";
        present_second: "~ni", "~nete";
        present_third: "~ne", "~ngono";
        imperfect_first: "~nevo", "~nevamo";
        imperfect_second: "~nevi", "~nevate";
        imperfect_third: "~neva", "~nevano";
        past_historic_first: "~si", "~nemmo";
        past_historic_second: "~nesti", "~neste";
        past_historic_third: "~se", "~sero";
        future_first: "~rrò", "~rremo";
        future_second: "~rrai", "~rrete";
        future_third: "~rrà", "~rranno";
        conditional_first: "~rrei", "~rremmo";
        conditional_second: "~rresti", "~rreste";
        conditional_third: "~rrebbe", "~rrebbero";
        present_subjunctive_first: "~nga", "~niamo";
        present_subjunctive_second: "~nga", "~niate";
        present_subjunctive_third: "~nga", "~ngano";
        imperfect_subjunctive_first: "~nessi", "~nessimo";
        imperfect_subjunctive_second: "~nessi", "~neste";
        imperfect_subjunctive_third: "~nesse", "~nessero";
        imperative: "~ni", referred, "~niamo", "~nete", "~ngano";
        negative_imperative: "~rre", referred, "~niamo", "~nete", referred;
        infinitive: "~rre";
        gerund: "~nendo";
        participle: "~nente", "~sto";
    ),
    // Dipartirsi: 19 words (vestirsi, nutrirsi, pentirsi, mentirsi, cucirsi, svestirsi).
    conjugation!("dipartirsi", 19, "~irsi";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ite";
        present_third: "~e", "~ono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~ì", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~iti", referred, "~iamoci", "~itevi", "~ano";
        negative_imperative: "~irti|~ire", referred, "~iamoci|~iamo", "~itevi|~ite", referred;
        infinitive: "~irsi";
        gerund: "~endosi";
        participle: "~entesi", "~itosi";
    ),
    // Farsi: 19 words (farsi, darsi, rifarsi, ridarsi, disfarsi, confarsi).
    conjugation!("farsi", 19, "~arsi";
        present_first: "~accio", "~acciamo";
        present_second: "~ai", "~ate";
        present_third: "~à", "~anno";
        imperfect_first: "~acevo", "~acevamo";
        imperfect_second: "~acevi", "~acevate";
        imperfect_third: "~aceva", "~acevano";
        past_historic_first: "~eci", "~acemmo";
        past_historic_second: "~acesti", "~aceste";
        past_historic_third: "~ece", "~ecero";
        future_first: "~arò", "~aremo";
        future_second: "~arai", "~arete";
        future_third: "~arà", "~aranno";
        conditional_first: "~arei", "~aremmo";
        conditional_second: "~aresti", "~areste";
        conditional_third: "~arebbe", "~arebbero";
        present_subjunctive_first: "~accia", "~acciamo";
        present_subjunctive_second: "~accia", "~acciate";
        present_subjunctive_third: "~accia", "~acciano";
        imperfect_subjunctive_first: "~acessi", "~acessimo";
        imperfect_subjunctive_second: "~acessi", "~aceste";
        imperfect_subjunctive_third: "~acesse", "~acessero";
        imperative: "~atti", referred, "~acciamoci", "~atevi", "~acciano";
        negative_imperative: "~arti|~are", referred, "~acciamoci|~acciamo", "~atevi|~ate", referred;
        infinitive: "~arsi";
        gerund: "~acendosi";
        participle: "~acentesi", "~attosi";
    ),
    // Battere: 18 words (battere, adempiere, fetere, pretessere, ritessere, scontessere).
    conjugation!("battere", 18, "~ere";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~ei", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~é", "~erono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", referred, "~iamo", "~ete", "~ano";
        negative_imperative: "~ere", referred, "~iamo", "~ete", referred;
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~uto";
    ),
    // Sviare: 15 words (notare, espiare, sviare, fuorviare, obliare, obbliare).
    conjugation!("sviare", 15, "~are";
        present_first: "~o", "~amo";
        present_second: "~i", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~i", "~amo";
        present_subjunctive_second: "~i", "~ate";
        present_subjunctive_third: "~i", "~ino";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~a", referred, "~amo", "~ate", "~ino";
        negative_imperative: "~are", referred, "~amo", "~ate", referred;
        infinitive: "~are";
        gerund: "~ando";
        participle: "~ante", "~ato";
    ),
    // Giostrare: 15 words (obstare, slinguazzare, sitare, tombare, neare, distare).
    conjugation!("giostrare", 15, "~are";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~i", "~iamo";
        present_subjunctive_second: "~i", "~iate";
        present_subjunctive_third: "~i", "~ino";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~a", "~i", "~iamo", "~ate", "~ino";
        negative_imperative: "~are", "~i", "~iamo", "~ate", "~ino";
        infinitive: "~are";
        gerund: "~ando";
        participle: "~ante", "~ato";
    ),
    // Dire: 14 words (dire, interdire, predire, indire, addire, disdire).
    conjugation!("dire", 14, "~ire";
        present_first: "~ico", "~iciamo";
        present_second: "~ici", "~ite";
        present_third: "~ice", "~icono";
        imperfect_first: "~icevo", "~icevamo";
        imperfect_second: "~icevi", "~icevate";
        imperfect_third: "~iceva", "~icevano";
        past_historic_first: "~issi", "~icemmo";
        past_historic_second: "~icesti", "~iceste";
        past_historic_third: "~isse", "~issero";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~ica", "~iciamo";
        present_subjunctive_second: "~ica", "~iciate";
        present_subjunctive_third: "~ica", "~icano";
        imperfect_subjunctive_first: "~icessi", "~icessimo";
        imperfect_subjunctive_second: "~icessi", "~iceste";
        imperfect_subjunctive_third: "~icesse", "~icessero";
        imperative: "~ici", referred, "~iciamo", "~ite", "~icano";
        negative_imperative: "~ire", referred, "~iciamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~icendo";
        participle: "~icente", "~etto";
    ),
    // Adere: 14 words (avere, potere, dovere, godere, parere, riavere).
    conjugation!("adere", 14, "~ere";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~di", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~de", "~dero";
        future_first: "~rò", "~remo";
        future_second: "~rai", "~rete";
        future_third: "~rà", "~ranno";
        conditional_first: "~rei", "~remmo";
        conditional_second: "~resti", "~reste";
        conditional_third: "~rebbe", "~rebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", referred, "~iamo", "~ete", "~ano";
        negative_imperative: "~ere", referred, "~iamo", "~ete", referred;
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~uto";
    ),
    // Aurire: 13 words (esordire, munire, obbedire, ubbidire, lenire, esaudire).
    conjugation!("aurire", 13, "~ire";
        present_first: "~isco", "~iamo";
        present_second: "~isci", "~ite";
        present_third: "~isce", "~iscono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~ì", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~isca", "~iamo";
        present_subjunctive_second: "~isca", "~iate";
        present_subjunctive_third: "~isca", "~iscano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~isci", referred, "~iamo", "~ite", "~iscano";
        negative_imperative: "~ire", referred, "~iamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~endo";
        participle: "~iente", "~ito";
    ),
    // Ncere: 13 words (evincere, molcere, ncere, vincere, orcere, torcere).
    conjugation!("ncere", 13, "~cere";
        present_first: "~co", "~ciamo";
        present_second: "~ci", "~cete";
        present_third: "~ce", "~cono";
        imperfect_first: "~cevo", "~cevamo";
        imperfect_second: "~cevi", "~cevate";
        imperfect_third: "~ceva", "~cevano";
        past_historic_first: "~si", "~cemmo";
        past_historic_second: "~cesti", "~ceste";
        past_historic_third: "~se", "~sero";
        future_first: "~cerò", "~ceremo";
        future_second: "~cerai", "~cerete";
        future_third: "~cerà", "~ceranno";
        conditional_first: "~cerei", "~ceremmo";
        conditional_second: "~ceresti", "~cereste";
        conditional_third: "~cerebbe", "~cerebbero";
        present_subjunctive_first: "~ca", "~ciamo";
        present_subjunctive_second: "~ca", "~ciate";
        present_subjunctive_third: "~ca", "~cano";
        imperfect_subjunctive_first: "~cessi", "~cessimo";
        imperfect_subjunctive_second: "~cessi", "~ceste";
        imperfect_subjunctive_third: "~cesse", "~cessero";
        imperative: "~ci", referred, "~ciamo", "~cete", "~cano";
        negative_imperative: "~cere", referred, "~ciamo", "~cete", referred;
        infinitive: "~cere";
        gerund: "~cendo";
        participle: "~cente", "~to";
    ),
    // Arsi: 13 words (bersi, arsi, porsi, trarsi, irsi, omporsi).
    conjugation!("arsi", 13, "~rsi";
        present_first: "~ngo", "~niamo";
        present_second: "~ni", "~nete";
        present_third: "~ne", "~ngono";
        imperfect_first: "~nevo", "~nevamo";
        imperfect_second: "~nevi", "~nevate";
        imperfect_third: "~neva", "~nevano";
        past_historic_first: "~si", "~nemmo";
        past_historic_second: "~nesti", "~neste";
        past_historic_third: "~se", "~sero";
        future_first: "~rrò", "~rremo";
        future_second: "~rrai", "~rrete";
        future_third: "~rrà", "~rranno";
        conditional_first: "~rrei", "~rremmo";
        conditional_second: "~rresti", "~rreste";
        conditional_third: "~rrebbe", "~rrebbero";
        present_subjunctive_first: "~nga", "~niamo";
        present_subjunctive_second: "~nga", "~niate";
        present_subjunctive_third: "~nga", "~ngano";
        imperfect_subjunctive_first: "~nessi", "~nessimo";
        imperfect_subjunctive_second: "~nessi", "~neste";
        imperfect_subjunctive_third: "~nesse", "~nessero";
        imperative: "~niti", referred, "~niamoci", "~netevi", "~ngano";
        negative_imperative: "~rti|~rre", referred, "~niamoci|~niamo", "~netevi|~nete", referred;
        infinitive: "~rsi";
        gerund: "~nendosi";
        participle: "~nentesi", "~stosi";
    ),
    // Ogliere: 12 words (togliere, svegliere, ogliere, egliere, cogliere, sciogliere).
    conjugation!("ogliere", 12, "~g~iere";
        present_first: "~~go", "~g~iamo";
        present_second: "~g~i", "~g~iete";
        present_third: "~g~ie", "~~gono";
        imperfect_first: "~g~ievo", "~g~ievamo";
        imperfect_second: "~g~ievi", "~g~ievate";
        imperfect_third: "~g~ieva", "~g~ievano";
        past_historic_first: "~~si", "~g~iemmo";
        past_historic_second: "~g~iesti", "~g~ieste";
        past_historic_third: "~~se", "~~sero";
        future_first: "~g~ierò", "~g~ieremo";
        future_second: "~g~ierai", "~g~ierete";
        future_third: "~g~ierà", "~g~ieranno";
        conditional_first: "~g~ierei", "~g~ieremmo";
        conditional_second: "~g~ieresti", "~g~iereste";
        conditional_third: "~g~ierebbe", "~g~ierebbero";
        present_subjunctive_first: "~~ga", "~g~iamo";
        present_subjunctive_second: "~~ga", "~g~iate";
        present_subjunctive_third: "~~ga", "~~gano";
        imperfect_subjunctive_first: "~g~iessi", "~g~iessimo";
        imperfect_subjunctive_second: "~g~iessi", "~g~ieste";
        imperfect_subjunctive_third: "~g~iesse", "~g~iessero";
        imperative: "~g~i", referred, "~g~iamo", "~g~iete", "~~gano";
        negative_imperative: "~g~iere", referred, "~g~iamo", "~g~iete", referred;
        infinitive: "~g~iere";
        gerund: "~g~iendo";
        participle: "~g~iente", "~~to";
    ),
    // Orrere: 11 words (occorrere, cherere, correre, orrere, ccorrere, percorrere).
    conjugation!("orrere", 11, "~rere";
        present_first: "~ro", "~riamo";
        present_second: "~ri", "~rete";
        present_third: "~re", "~rono";
        imperfect_first: "~revo", "~revamo";
        imperfect_second: "~revi", "~revate";
        imperfect_third: "~reva", "~revano";
        past_historic_first: "~si", "~remmo";
        past_historic_second: "~resti", "~reste";
        past_historic_third: "~se", "~sero";
        future_first: "~rerò", "~reremo";
        future_second: "~rerai", "~rerete";
        future_third: "~rerà", "~reranno";
        conditional_first: "~rerei", "~reremmo";
        conditional_second: "~reresti", "~rereste";
        conditional_third: "~rerebbe", "~rerebbero";
        present_subjunctive_first: "~ra", "~riamo";
        present_subjunctive_second: "~ra", "~riate";
        present_subjunctive_third: "~ra", "~rano";
        imperfect_subjunctive_first: "~ressi", "~ressimo";
        imperfect_subjunctive_second: "~ressi", "~reste";
        imperfect_subjunctive_third: "~resse", "~ressero";
        imperative: "~ri", referred, "~riamo", "~rete", "~rano";
        negative_imperative: "~rere", referred, "~riamo", "~rete", referred;
        infinitive: "~rere";
        gerund: "~rendo";
        participle: "~rente", "~so";
    ),
    // Argere: 11 words (emergere, mergere, spergere, tergere, pergere, immergere).
    conjugation!("argere", 11, "~gere";
        present_first: "~go", "~giamo";
        present_second: "~gi", "~gete";
        present_third: "~ge", "~gono";
        imperfect_first: "~gevo", "~gevamo";
        imperfect_second: "~gevi", "~gevate";
        imperfect_third: "~geva", "~gevano";
        past_historic_first: "~si", "~gemmo";
        past_historic_second: "~gesti", "~geste";
        past_historic_third: "~se", "~sero";
        future_first: "~gerò", "~geremo";
        future_second: "~gerai", "~gerete";
        future_third: "~gerà", "~geranno";
        conditional_first: "~gerei", "~geremmo";
        conditional_second: "~geresti", "~gereste";
        conditional_third: "~gerebbe", "~gerebbero";
        present_subjunctive_first: "~ga", "~giamo";
        present_subjunctive_second: "~ga", "~giate";
        present_subjunctive_third: "~ga", "~gano";
        imperfect_subjunctive_first: "~gessi", "~gessimo";
        imperfect_subjunctive_second: "~gessi", "~geste";
        imperfect_subjunctive_third: "~gesse", "~gessero";
        imperative: "~gi", referred, "~giamo", "~gete", "~gano";
        negative_imperative: "~gere", referred, "~giamo", "~gete", referred;
        infinitive: "~gere";
        gerund: "~gendo";
        participle: "~gente", "~so";
    ),
    // Uggersi: 11 words (leggersi, crocifiggersi, teggersi, truggersi, reggersi, fliggersi).
    conjugation!("uggersi", 11, "~ggersi";
        present_first: "~ggo", "~ggiamo";
        present_second: "~ggi", "~ggete";
        present_third: "~gge", "~ggono";
        imperfect_first: "~ggevo", "~ggevamo";
        imperfect_second: "~ggevi", "~ggevate";
        imperfect_third: "~ggeva", "~ggevano";
        past_historic_first: "~ssi", "~ggemmo";
        past_historic_second: "~ggesti", "~ggeste";
        past_historic_third: "~sse", "~ssero";
        future_first: "~ggerò", "~ggeremo";
        future_second: "~ggerai", "~ggerete";
        future_third: "~ggerà", "~ggeranno";
        conditional_first: "~ggerei", "~ggeremmo";
        conditional_second: "~ggeresti", "~ggereste";
        conditional_third: "~ggerebbe", "~ggerebbero";
        present_subjunctive_first: "~gga", "~ggiamo";
        present_subjunctive_second: "~gga", "~ggiate";
        present_subjunctive_third: "~gga", "~ggano";
        imperfect_subjunctive_first: "~ggessi", "~ggessimo";
        imperfect_subjunctive_second: "~ggessi", "~ggeste";
        imperfect_subjunctive_third: "~ggesse", "~ggessero";
        imperative: "~ggiti", referred, "~ggiamoci", "~ggetevi", "~ggano";
        negative_imperative: "~ggerti|~ggere", referred, "~ggiamoci|~ggiamo", "~ggetevi|~ggete", referred;
        infinitive: "~ggersi";
        gerund: "~ggendosi";
        participle: "~ggentesi", "~ttosi";
    ),
    // Onoscere: 10 words (crescere, gnoscere, oscere, onoscere, conoscere, sconoscere).
    conjugation!("onoscere", 10, "~scere";
        present_first: "~sco", "~sciamo";
        present_second: "~sci", "~scete";
        present_third: "~sce", "~scono";
        imperfect_first: "~scevo", "~scevamo";
        imperfect_second: "~scevi", "~scevate";
        imperfect_third: "~sceva", "~scevano";
        past_historic_first: "~bbi", "~scemmo";
        past_historic_second: "~scesti", "~sceste";
        past_historic_third: "~bbe", "~bbero";
        future_first: "~scerò", "~sceremo";
        future_second: "~scerai", "~scerete";
        future_third: "~scerà", "~sceranno";
        conditional_first: "~scerei", "~sceremmo";
        conditional_second: "~sceresti", "~scereste";
        conditional_third: "~scerebbe", "~scerebbero";
        present_subjunctive_first: "~sca", "~sciamo";
        present_subjunctive_second: "~sca", "~sciate";
        present_subjunctive_third: "~sca", "~scano";
        imperfect_subjunctive_first: "~scessi", "~scessimo";
        imperfect_subjunctive_second: "~scessi", "~sceste";
        imperfect_subjunctive_third: "~scesse", "~scessero";
        imperative: "~sci", referred, "~sciamo", "~scete", "~scano";
        negative_imperative: "~scere", referred, "~sciamo", "~scete", referred;
        infinitive: "~scere";
        gerund: "~scendo";
        participle: "~scente", "~sciuto";
    ),
    // Rivere: 10 words (iscrivere, dattiloscrivere, rivere, scrivere, coscrivere, crivere).
    conjugation!("rivere", 10, "~vere";
        present_first: "~vo", "~viamo";
        present_second: "~vi", "~vete";
        present_third: "~ve", "~vono";
        imperfect_first: "~vevo", "~vevamo";
        imperfect_second: "~vevi", "~vevate";
        imperfect_third: "~veva", "~vevano";
        past_historic_first: "~ssi", "~vemmo";
        past_historic_second: "~vesti", "~veste";
        past_historic_third: "~sse", "~ssero";
        future_first: "~verò", "~veremo";
        future_second: "~verai", "~verete";
        future_third: "~verà", "~veranno";
        conditional_first: "~verei", "~veremmo";
        conditional_second: "~veresti", "~vereste";
        conditional_third: "~verebbe", "~verebbero";
        present_subjunctive_first: "~va", "~viamo";
        present_subjunctive_second: "~va", "~viate";
        present_subjunctive_third: "~va", "~vano";
        imperfect_subjunctive_first: "~vessi", "~vessimo";
        imperfect_subjunctive_second: "~vessi", "~veste";
        imperfect_subjunctive_third: "~vesse", "~vessero";
        imperative: "~vi", referred, "~viamo", "~vete", "~vano";
        negative_imperative: "~vere", referred, "~viamo", "~vete", referred;
        infinitive: "~vere";
        gerund: "~vendo";
        participle: "~vente", "~tto";
    ),
    // Frire: 10 words (offrire, uprire, prire, frire, coprire, aprire).
    conjugation!("frire", 10, "~~ire";
        present_first: "~~o", "~~iamo";
        present_second: "~~i", "~~ite";
        present_third: "~~e", "~~ono";
        imperfect_first: "~~ivo", "~~ivamo";
        imperfect_second: "~~ivi", "~~ivate";
        imperfect_third: "~~iva", "~~ivano";
        past_historic_first: "~~ii|~e~si", "~~immo";
        past_historic_second: "~~isti", "~~iste";
        past_historic_third: "~~ì|~e~se", "~~irono|~e~sero";
        future_first: "~~irò", "~~iremo";
        future_second: "~~irai", "~~irete";
        future_third: "~~irà", "~~iranno";
        conditional_first: "~~irei", "~~iremmo";
        conditional_second: "~~iresti", "~~ireste";
        conditional_third: "~~irebbe", "~~irebbero";
        present_subjunctive_first: "~~a", "~~iamo";
        present_subjunctive_second: "~~a", "~~iate";
        present_subjunctive_third: "~~a", "~~ano";
        imperfect_subjunctive_first: "~~issi", "~~issimo";
        imperfect_subjunctive_second: "~~issi", "~~iste";
        imperfect_subjunctive_third: "~~isse", "~~issero";
        imperative: "~~i", referred, "~~iamo", "~~ite", "~~ano";
        negative_imperative: "~~ire", referred, "~~iamo", "~~ite", referred;
        infinitive: "~~ire";
        gerund: "~~endo";
        participle: "~~ente", "~e~to";
    ),
    // Venire: 9 words (venire, svenire, divenire, onvenire, sovvenire, avvenire).
    conjugation!("venire", 9, "~~nire";
        present_first: "~~ngo", "~~niamo";
        present_second: "~i~ni", "~~nite";
        present_third: "~i~ne", "~~ngono";
        imperfect_first: "~~nivo", "~~nivamo";
        imperfect_second: "~~nivi", "~~nivate";
        imperfect_third: "~~niva", "~~nivano";
        past_historic_first: "~~nni", "~~nimmo";
        past_historic_second: "~~nisti", "~~niste";
        past_historic_third: "~~nne", "~~nnero";
        future_first: "~~rrò", "~~rremo";
        future_second: "~~rrai", "~~rrete";
        future_third: "~~rrà", "~~rranno";
        conditional_first: "~~rrei", "~~rremmo";
        conditional_second: "~~rresti", "~~rreste";
        conditional_third: "~~rrebbe", "~~rrebbero";
        present_subjunctive_first: "~~nga", "~~niamo";
        present_subjunctive_second: "~~nga", "~~niate";
        present_subjunctive_third: "~~nga", "~~ngano";
        imperfect_subjunctive_first: "~~nissi", "~~nissimo";
        imperfect_subjunctive_second: "~~nissi", "~~niste";
        imperfect_subjunctive_third: "~~nisse", "~~nissero";
        imperative: "~i~ni", referred, "~~niamo", "~~nite", "~~ngano";
        negative_imperative: "~~nire", referred, "~~niamo", "~~nite", referred;
        infinitive: "~~nire";
        gerund: "~~nendo";
        participle: "~~niente", "~~nuto";
    ),
    // Piersi: 9 words (battersi, attersi, ttersi, petersi, ottersi, piersi).
    conjugation!("piersi", 9, "~ersi";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~ei", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~é", "~erono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~iti", referred, "~iamoci", "~etevi", "~ano";
        negative_imperative: "~erti|~ere", referred, "~iamoci|~iamo", "~etevi|~ete", referred;
        infinitive: "~ersi";
        gerund: "~endosi";
        participle: "~entesi", "~utosi";
    ),
    // Sistere: 9 words (cernere, istere, sistere, esistere, assistere, rvoassistere).
    conjugation!("sistere", 9, "~ere";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~etti|~ei", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~ette|~é", "~ettero|~erono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", referred, "~iamo", "~ete", "~ano";
        negative_imperative: "~ere", referred, "~iamo", "~ete", referred;
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~ito";
    ),
    // Tenere: 8 words (tenere, artenere, ttenere, stenere, manere, ottenere).
    conjugation!("tenere", 8, "~~nere";
        present_first: "~~ngo", "~~niamo";
        present_second: "~i~ni", "~~nete";
        present_third: "~i~ne", "~~ngono";
        imperfect_first: "~~nevo", "~~nevamo";
        imperfect_second: "~~nevi", "~~nevate";
        imperfect_third: "~~neva", "~~nevano";
        past_historic_first: "~~nni", "~~nemmo";
        past_historic_second: "~~nesti", "~~neste";
        past_historic_third: "~~nne", "~~nnero";
        future_first: "~~rrò", "~~rremo";
        future_second: "~~rrai", "~~rrete";
        future_third: "~~rrà", "~~rranno";
        conditional_first: "~~rrei", "~~rremmo";
        conditional_second: "~~rresti", "~~rreste";
        conditional_third: "~~rrebbe", "~~rrebbero";
        present_subjunctive_first: "~~nga", "~~niamo";
        present_subjunctive_second: "~~nga", "~~niate";
        present_subjunctive_third: "~~nga", "~~ngano";
        imperfect_subjunctive_first: "~~nessi", "~~nessimo";
        imperfect_subjunctive_second: "~~nessi", "~~neste";
        imperfect_subjunctive_third: "~~nesse", "~~nessero";
        imperative: "~i~ni", referred, "~~niamo", "~~nete", "~~ngano";
        negative_imperative: "~~nere", referred, "~~niamo", "~~nete", referred;
        infinitive: "~~nere";
        gerund: "~~nendo";
        participle: "~~nente", "~~nuto";
    ),
    // Prefiggere: 8 words (affiggere, crocefiggere, crocifiggere, crucifiggere, defiggere, prefiggere).
    conjugation!("prefiggere", 8, "~ggere";
        present_first: "~ggo", "~ggiamo";
        present_second: "~ggi", "~ggete";
        present_third: "~gge", "~ggono";
        imperfect_first: "~ggevo", "~ggevamo";
        imperfect_second: "~ggevi", "~ggevate";
        imperfect_third: "~ggeva", "~ggevano";
        past_historic_first: "~ssi", "~ggemmo";
        past_historic_second: "~ggesti", "~ggeste";
        past_historic_third: "~sse", "~ssero";
        future_first: "~ggerò", "~ggeremo";
        future_second: "~ggerai", "~ggerete";
        future_third: "~ggerà", "~ggeranno";
        conditional_first: "~ggerei", "~ggeremmo";
        conditional_second: "~ggeresti", "~ggereste";
        conditional_third: "~ggerebbe", "~ggerebbero";
        present_subjunctive_first: "~gga", "~ggiamo";
        present_subjunctive_second: "~gga", "~ggiate";
        present_subjunctive_third: "~gga", "~ggano";
        imperfect_subjunctive_first: "~ggessi", "~ggessimo";
        imperfect_subjunctive_second: "~ggessi", "~ggeste";
        imperfect_subjunctive_third: "~ggesse", "~ggessero";
        imperative: "~ggi", referred, "~ggiamo", "~ggete", "~ggano";
        negative_imperative: "~ggere", referred, "~ggiamo", "~ggete", referred;
        infinitive: "~ggere";
        gerund: "~ggendo";
        participle: "~ggente", "~sso";
    ),
    // Endersi: 8 words (offendersi, prendersi, endersi, rendersi, intendersi, tendersi).
    conjugation!("endersi", 8, "~ndersi";
        present_first: "~ndo", "~ndiamo";
        present_second: "~ndi", "~ndete";
        present_third: "~nde", "~ndono";
        imperfect_first: "~ndevo", "~ndevamo";
        imperfect_second: "~ndevi", "~ndevate";
        imperfect_third: "~ndeva", "~ndevano";
        past_historic_first: "~si", "~ndemmo";
        past_historic_second: "~ndesti", "~ndeste";
        past_historic_third: "~se", "~sero";
        future_first: "~nderò", "~nderemo";
        future_second: "~nderai", "~nderete";
        future_third: "~nderà", "~nderanno";
        conditional_first: "~nderei", "~nderemmo";
        conditional_second: "~nderesti", "~ndereste";
        conditional_third: "~nderebbe", "~nderebbero";
        present_subjunctive_first: "~nda", "~ndiamo";
        present_subjunctive_second: "~nda", "~ndiate";
        present_subjunctive_third: "~nda", "~ndano";
        imperfect_subjunctive_first: "~ndessi", "~ndessimo";
        imperfect_subjunctive_second: "~ndessi", "~ndeste";
        imperfect_subjunctive_third: "~ndesse", "~ndessero";
        imperative: "~nditi", referred, "~ndiamoci", "~ndetevi", "~ndano";
        negative_imperative: "~nderti|~ndere", referred, "~ndiamoci|~ndiamo", "~ndetevi|~ndete", referred;
        infinitive: "~ndersi";
        gerund: "~ndendosi";
        participle: "~ndentesi", "~sosi";
    ),
    // Durre: 8 words (edurre, durre, produrre, ondurre, tradurre, introdurre).
    conjugation!("durre", 8, "~urre";
        present_first: "~uco", "~uciamo";
        present_second: "~uci", "~ucete";
        present_third: "~uce", "~ucono";
        imperfect_first: "~ucevo", "~ucevamo";
        imperfect_second: "~ucevi", "~ucevate";
        imperfect_third: "~uceva", "~ucevano";
        past_historic_first: "~ussi", "~ucemmo";
        past_historic_second: "~ucesti", "~uceste";
        past_historic_third: "~usse", "~ussero";
        future_first: "~urrò", "~urremo";
        future_second: "~urrai", "~urrete";
        future_third: "~urrà", "~urranno";
        conditional_first: "~urrei", "~urremmo";
        conditional_second: "~urresti", "~urreste";
        conditional_third: "~urrebbe", "~urrebbero";
        present_subjunctive_first: "~uca", "~uciamo";
        present_subjunctive_second: "~uca", "~uciate";
        present_subjunctive_third: "~uca", "~ucano";
        imperfect_subjunctive_first: "~ucessi", "~ucessimo";
        imperfect_subjunctive_second: "~ucessi", "~uceste";
        imperfect_subjunctive_third: "~ucesse", "~ucessero";
        imperative: "~uci", referred, "~uciamo", "~ucete", "~ucano";
        negative_imperative: "~urre", referred, "~uciamo", "~ucete", referred;
        infinitive: "~urre";
        gerund: "~ucendo";
        participle: "~ucente", "~otto";
    ),
    // Arre: 8 words (rarre, trarre, arre, ontrarre, urre, contrarre).
    conjugation!("arre", 8, "~rre";
        present_first: "~ggo", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ggono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~ssi", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~sse", "~ssero";
        future_first: "~rrò", "~rremo";
        future_second: "~rrai", "~rrete";
        future_third: "~rrà", "~rranno";
        conditional_first: "~rrei", "~rremmo";
        conditional_second: "~rresti", "~rreste";
        conditional_third: "~rrebbe", "~rrebbero";
        present_subjunctive_first: "~gga", "~iamo";
        present_subjunctive_second: "~gga", "~iate";
        present_subjunctive_third: "~gga", "~ggano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", referred, "~iamo", "~ete", "~ggano";
        negative_imperative: "~rre", referred, "~iamo", "~ete", referred;
        infinitive: "~rre";
        gerund: "~endo";
        participle: "~ente", "~tto";
    ),
    // Aurirsi: 7 words (munirsi, pedirsi, aurirsi, orgoglirsi, venirsi, sovvenirsi).
    conjugation!("aurirsi", 7, "~irsi";
        present_first: "~isco", "~iamo";
        present_second: "~isci", "~ite";
        present_third: "~isce", "~iscono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~ì", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~isca", "~iamo";
        present_subjunctive_second: "~isca", "~iate";
        present_subjunctive_third: "~isca", "~iscano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~isciti", referred, "~iamoci", "~itevi", "~iscano";
        negative_imperative: "~irti|~ire", referred, "~iamoci|~iamo", "~itevi|~ite", referred;
        infinitive: "~irsi";
        gerund: "~endosi";
        participle: "~ientesi", "~itosi";
    ),
    // Egnere: 7 words (ugnere, egnere, pegnere, spegnere, ignere, pignere).
    conjugation!("egnere", 7, "~g~ere";
        present_first: "~~go", "~g~iamo";
        present_second: "~g~i", "~g~ete";
        present_third: "~g~e", "~~gono";
        imperfect_first: "~g~evo", "~g~evamo";
        imperfect_second: "~g~evi", "~g~evate";
        imperfect_third: "~g~eva", "~g~evano";
        past_historic_first: "~~si", "~g~emmo";
        past_historic_second: "~g~esti", "~g~este";
        past_historic_third: "~~se", "~~sero";
        future_first: "~g~erò", "~g~eremo";
        future_second: "~g~erai", "~g~erete";
        future_third: "~g~erà", "~g~eranno";
        conditional_first: "~g~erei", "~g~eremmo";
        conditional_second: "~g~eresti", "~g~ereste";
        conditional_third: "~g~erebbe", "~g~erebbero";
        present_subjunctive_first: "~~ga", "~g~iamo";
        present_subjunctive_second: "~~ga", "~g~iate";
        present_subjunctive_third: "~~ga", "~~gano";
        imperfect_subjunctive_first: "~g~essi", "~g~essimo";
        imperfect_subjunctive_second: "~g~essi", "~g~este";
        imperfect_subjunctive_third: "~g~esse", "~g~essero";
        imperative: "~g~i", referred, "~g~iamo", "~g~ete", "~~gano";
        negative_imperative: "~g~ere", referred, "~g~iamo", "~g~ete", referred;
        infinitive: "~g~ere";
        gerund: "~g~endo";
        participle: "~g~ente", "~~to";
    ),
    // Nettere: 6 words (flettere, ettere, nettere, connettere, onnettere, annettere).
    conjugation!("nettere", 6, "~ttere";
        present_first: "~tto", "~ttiamo";
        present_second: "~tti", "~ttete";
        present_third: "~tte", "~ttono";
        imperfect_first: "~ttevo", "~ttevamo";
        imperfect_second: "~ttevi", "~ttevate";
        imperfect_third: "~tteva", "~ttevano";
        past_historic_first: "~ttei", "~ttemmo";
        past_historic_second: "~ttesti", "~tteste";
        past_historic_third: "~tté", "~tterono";
        future_first: "~tterò", "~tteremo";
        future_second: "~tterai", "~tterete";
        future_third: "~tterà", "~tteranno";
        conditional_first: "~tterei", "~tteremmo";
        conditional_second: "~tteresti", "~ttereste";
        conditional_third: "~tterebbe", "~tterebbero";
        present_subjunctive_first: "~tta", "~ttiamo";
        present_subjunctive_second: "~tta", "~ttiate";
        present_subjunctive_third: "~tta", "~ttano";
        imperfect_subjunctive_first: "~ttessi", "~ttessimo";
        imperfect_subjunctive_second: "~ttessi", "~tteste";
        imperfect_subjunctive_third: "~ttesse", "~ttessero";
        imperative: "~tti", referred, "~ttiamo", "~ttete", "~ttano";
        negative_imperative: "~ttere", referred, "~ttiamo", "~ttete", referred;
        infinitive: "~ttere";
        gerund: "~ttendo";
        participle: "~ttente", "~sso";
    ),
    // Tacere: 6 words (iacere, tacere, spiacere, acere, piacere, giacere).
    conjugation!("tacere", 6, "~ere";
        present_first: "~cio", "~ciamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ciono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~qui", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~que", "~quero";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~cia", "~ciamo";
        present_subjunctive_second: "~cia", "~ciate";
        present_subjunctive_third: "~cia", "~ciano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", referred, "~ciamo", "~ete", "~ciano";
        negative_imperative: "~ere", referred, "~ciamo", "~ete", referred;
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~iuto";
    ),
    // Vedere: 6 words (vedere, antivedere, stravedere, transvedere, travedere, trasvedere).
    conjugation!("vedere", 6, "~edere";
        present_first: "~edo", "~ediamo";
        present_second: "~edi", "~edete";
        present_third: "~ede", "~edono";
        imperfect_first: "~edevo", "~edevamo";
        imperfect_second: "~edevi", "~edevate";
        imperfect_third: "~edeva", "~edevano";
        past_historic_first: "~idi", "~edemmo";
        past_historic_second: "~edesti", "~edeste";
        past_historic_third: "~ide", "~idero";
        future_first: "~edrò", "~edremo";
        future_second: "~edrai", "~edrete";
        future_third: "~edrà", "~edranno";
        conditional_first: "~edrei", "~edremmo";
        conditional_second: "~edresti", "~edreste";
        conditional_third: "~edrebbe", "~edrebbero";
        present_subjunctive_first: "~eda", "~ediamo";
        present_subjunctive_second: "~eda", "~ediate";
        present_subjunctive_third: "~eda", "~edano";
        imperfect_subjunctive_first: "~edessi", "~edessimo";
        imperfect_subjunctive_second: "~edessi", "~edeste";
        imperfect_subjunctive_third: "~edesse", "~edessero";
        imperative: "~edi", referred, "~ediamo", "~edete", "~edano";
        negative_imperative: "~edere", referred, "~ediamo", "~edete", referred;
        infinitive: "~edere";
        gerund: "~edendo";
        participle: "~edente|~eggente", "~eduto";
    ),
    // Rispondere: 5 words (nascondere, rispondere, spondere, ascondere, condere).
    conjugation!("rispondere", 5, "~ndere";
        present_first: "~ndo", "~ndiamo";
        present_second: "~ndi", "~ndete";
        present_third: "~nde", "~ndono";
        imperfect_first: "~ndevo", "~ndevamo";
        imperfect_second: "~ndevi", "~ndevate";
        imperfect_third: "~ndeva", "~ndevano";
        past_historic_first: "~si", "~ndemmo";
        past_historic_second: "~ndesti", "~ndeste";
        past_historic_third: "~se", "~sero";
        future_first: "~nderò", "~nderemo";
        future_second: "~nderai", "~nderete";
        future_third: "~nderà", "~nderanno";
        conditional_first: "~nderei", "~nderemmo";
        conditional_second: "~nderesti", "~ndereste";
        conditional_third: "~nderebbe", "~nderebbero";
        present_subjunctive_first: "~nda", "~ndiamo";
        present_subjunctive_second: "~nda", "~ndiate";
        present_subjunctive_third: "~nda", "~ndano";
        imperfect_subjunctive_first: "~ndessi", "~ndessimo";
        imperfect_subjunctive_second: "~ndessi", "~ndeste";
        imperfect_subjunctive_third: "~ndesse", "~ndessero";
        imperative: "~ndi", referred, "~ndiamo", "~ndete", "~ndano";
        negative_imperative: "~ndere", referred, "~ndiamo", "~ndete", referred;
        infinitive: "~ndere";
        gerund: "~ndendo";
        participle: "~ndente", "~sto";
    ),
    // Risolvere: 5 words (risolvere, evolvere, resolvere, dissolvere, assolvere).
    conjugation!("risolvere", 5, "~vere";
        present_first: "~vo", "~viamo";
        present_second: "~vi", "~vete";
        present_third: "~ve", "~vono";
        imperfect_first: "~vevo", "~vevamo";
        imperfect_second: "~vevi", "~vevate";
        imperfect_third: "~veva", "~vevano";
        past_historic_first: "~si", "~vemmo";
        past_historic_second: "~vesti", "~veste";
        past_historic_third: "~se", "~sero";
        future_first: "~verò", "~veremo";
        future_second: "~verai", "~verete";
        future_third: "~verà", "~veranno";
        conditional_first: "~verei", "~veremmo";
        conditional_second: "~veresti", "~vereste";
        conditional_third: "~verebbe", "~verebbero";
        present_subjunctive_first: "~va", "~viamo";
        present_subjunctive_second: "~va", "~viate";
        present_subjunctive_third: "~va", "~vano";
        imperfect_subjunctive_first: "~vessi", "~vessimo";
        imperfect_subjunctive_second: "~vessi", "~veste";
        imperfect_subjunctive_third: "~vesse", "~vessero";
        imperative: "~vi", referred, "~viamo", "~vete", "~vano";
        negative_imperative: "~vere", referred, "~viamo", "~vete", referred;
        infinitive: "~vere";
        gerund: "~vendo";
        participle: "~vente", "~to";
    ),
    // Dirsi: 5 words (addirsi, dirsi, disdirsi, edirsi, contraddirsi).
    conjugation!("dirsi", 5, "~~rsi";
        present_first: "~~co", "~~ciamo";
        present_second: "~~ci", "~~te";
        present_third: "~~ce", "~~cono";
        imperfect_first: "~~cevo", "~~cevamo";
        imperfect_second: "~~cevi", "~~cevate";
        imperfect_third: "~~ceva", "~~cevano";
        past_historic_first: "~~ssi", "~~cemmo";
        past_historic_second: "~~cesti", "~~ceste";
        past_historic_third: "~~sse", "~~ssero";
        future_first: "~~rò", "~~remo";
        future_second: "~~rai", "~~rete";
        future_third: "~~rà", "~~ranno";
        conditional_first: "~~rei", "~~remmo";
        conditional_second: "~~resti", "~~reste";
        conditional_third: "~~rebbe", "~~rebbero";
        present_subjunctive_first: "~~ca", "~~ciamo";
        present_subjunctive_second: "~~ca", "~~ciate";
        present_subjunctive_third: "~~ca", "~~cano";
        imperfect_subjunctive_first: "~~cessi", "~~cessimo";
        imperfect_subjunctive_second: "~~cessi", "~~ceste";
        imperfect_subjunctive_third: "~~cesse", "~~cessero";
        imperative: "~~citi", referred, "~~ciamoci", "~~tevi", "~~cano";
        negative_imperative: "~~rti|~~re", referred, "~~ciamoci|~~ciamo", "~~tevi|~~te", referred;
        infinitive: "~~rsi";
        gerund: "~~cendosi";
        participle: "~~centesi", "~ettos~";
    ),
    // Rigere: 5 words (erigere, negligere, diligere, rigere, ligere).
    conjugation!("rigere", 5, "~igere";
        present_first: "~igo", "~igiamo";
        present_second: "~igi", "~igete";
        present_third: "~ige", "~igono";
        imperfect_first: "~igevo", "~igevamo";
        imperfect_second: "~igevi", "~igevate";
        imperfect_third: "~igeva", "~igevano";
        past_historic_first: "~essi", "~igemmo";
        past_historic_second: "~igesti", "~igeste";
        past_historic_third: "~esse", "~essero";
        future_first: "~igerò", "~igeremo";
        future_second: "~igerai", "~igerete";
        future_third: "~igerà", "~igeranno";
        conditional_first: "~igerei", "~igeremmo";
        conditional_second: "~igeresti", "~igereste";
        conditional_third: "~igerebbe", "~igerebbero";
        present_subjunctive_first: "~iga", "~igiamo";
        present_subjunctive_second: "~iga", "~igiate";
        present_subjunctive_third: "~iga", "~igano";
        imperfect_subjunctive_first: "~igessi", "~igessimo";
        imperfect_subjunctive_second: "~igessi", "~igeste";
        imperfect_subjunctive_third: "~igesse", "~igessero";
        imperative: "~igi", referred, "~igiamo", "~igete", "~igano";
        negative_imperative: "~igere", referred, "~igiamo", "~igete", referred;
        infinitive: "~igere";
        gerund: "~igendo";
        participle: "~igente", "~etto";
    ),
    // Sapere: 5 words (sapere, strasapere, presapere, risapere, ssapere).
    conjugation!("sapere", 5, "~apere";
        present_first: "~ò", "~appiamo";
        present_second: "~ai", "~apete";
        present_third: "~à", "~anno";
        imperfect_first: "~apevo", "~apevamo";
        imperfect_second: "~apevi", "~apevate";
        imperfect_third: "~apeva", "~apevano";
        past_historic_first: "~eppi", "~apemmo";
        past_historic_second: "~apesti", "~apeste";
        past_historic_third: "~eppe", "~eppero";
        future_first: "~aprò", "~apremo";
        future_second: "~aprai", "~aprete";
        future_third: "~aprà", "~apranno";
        conditional_first: "~aprei", "~apremmo";
        conditional_second: "~apresti", "~apreste";
        conditional_third: "~aprebbe", "~aprebbero";
        present_subjunctive_first: "~appia", "~appiamo";
        present_subjunctive_second: "~appia", "~appiate";
        present_subjunctive_third: "~appia", "~appiano";
        imperfect_subjunctive_first: "~apessi", "~apessimo";
        imperfect_subjunctive_second: "~apessi", "~apeste";
        imperfect_subjunctive_third: "~apesse", "~apessero";
        imperative: "~appi", referred, "~appiamo", "~appiate", "~appiano";
        negative_imperative: "~apere", referred, "~appiamo", "~appiate", referred;
        infinitive: "~apere";
        gerund: "~apendo";
        participle: "~ante", "~aputo";
    ),
    // Ridare: 5 words (dare, ridare, sottostare, ristare, antistare).
    conjugation!("ridare", 5, "~are";
        present_first: "~ò", "~iamo";
        present_second: "~ai", "~ate";
        present_third: "~à", "~anno";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~etti", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~ette", "~ettero";
        future_first: "~arò", "~aremo";
        future_second: "~arai", "~arete";
        future_third: "~arà", "~aranno";
        conditional_first: "~arei", "~aremmo";
        conditional_second: "~aresti", "~areste";
        conditional_third: "~arebbe", "~arebbero";
        present_subjunctive_first: "~ia", "~iamo";
        present_subjunctive_second: "~ia", "~iate";
        present_subjunctive_third: "~ia", "~iano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~ai|~a'", referred, "~iamo", "~ate", "~iano";
        negative_imperative: "~are", referred, "~iamo", "~ate", referred;
        infinitive: "~are";
        gerund: "~ando";
        participle: "~ante", "~ato";
    ),
    // Egliersi: 5 words (togliersi, egliersi, ogliersi, ciogliersi, cogliersi).
    conjugation!("egliersi", 5, "~g~iersi";
        present_first: "~~go", "~g~iamo";
        present_second: "~g~i", "~g~iete";
        present_third: "~g~ie", "~~gono";
        imperfect_first: "~g~ievo", "~g~ievamo";
        imperfect_second: "~g~ievi", "~g~ievate";
        imperfect_third: "~g~ieva", "~g~ievano";
        past_historic_first: "~~si", "~g~iemmo";
        past_historic_second: "~g~iesti", "~g~ieste";
        past_historic_third: "~~se", "~~sero";
        future_first: "~g~ierò", "~g~ieremo";
        future_second: "~g~ierai", "~g~ierete";
        future_third: "~g~ierà", "~g~ieranno";
        conditional_first: "~g~ierei", "~g~ieremmo";
        conditional_second: "~g~ieresti", "~g~iereste";
        conditional_third: "~g~ierebbe", "~g~ierebbero";
        present_subjunctive_first: "~~ga", "~g~iamo";
        present_subjunctive_second: "~~ga", "~g~iate";
        present_subjunctive_third: "~~ga", "~~gano";
        imperfect_subjunctive_first: "~g~iessi", "~g~iessimo";
        imperfect_subjunctive_second: "~g~iessi", "~g~ieste";
        imperfect_subjunctive_third: "~g~iesse", "~g~iessero";
        imperative: "~g~iti", referred, "~g~iamoci", "~g~ietevi", "~~gano";
        negative_imperative: "~g~ierti|~g~iere", referred, "~g~iamoci|~g~iamo", "~g~ietevi|~g~iete", referred;
        infinitive: "~g~iersi";
        gerund: "~g~iendosi";
        participle: "~g~ientesi", "~~tosi";
    ),
    // Ettersi: 5 words (genuflettersi, flettersi, nettersi, ettersi, connettersi).
    conjugation!("ettersi", 5, "~ttersi";
        present_first: "~tto", "~ttiamo";
        present_second: "~tti", "~ttete";
        present_third: "~tte", "~ttono";
        imperfect_first: "~ttevo", "~ttevamo";
        imperfect_second: "~ttevi", "~ttevate";
        imperfect_third: "~tteva", "~ttevano";
        past_historic_first: "~ttei", "~ttemmo";
        past_historic_second: "~ttesti", "~tteste";
        past_historic_third: "~tté", "~tterono";
        future_first: "~tterò", "~tteremo";
        future_second: "~tterai", "~tterete";
        future_third: "~tterà", "~tteranno";
        conditional_first: "~tterei", "~tteremmo";
        conditional_second: "~tteresti", "~ttereste";
        conditional_third: "~tterebbe", "~tterebbero";
        present_subjunctive_first: "~tta", "~ttiamo";
        present_subjunctive_second: "~tta", "~ttiate";
        present_subjunctive_third: "~tta", "~ttano";
        imperfect_subjunctive_first: "~ttessi", "~ttessimo";
        imperfect_subjunctive_second: "~ttessi", "~tteste";
        imperfect_subjunctive_third: "~ttesse", "~ttessero";
        imperative: "~ttiti", referred, "~ttiamoci", "~ttetevi", "~ttano";
        negative_imperative: "~tterti|~ttere", referred, "~ttiamoci|~ttiamo", "~ttetevi|~ttete", referred;
        infinitive: "~ttersi";
        gerund: "~ttendosi";
        participle: "~ttentesi", "~ssosi";
    ),
    // Sviarsi: 5 words (obliarsi, sviarsi, avviarsi, viarsi, miarsi).
    conjugation!("sviarsi", 5, "~arsi";
        present_first: "~o", "~amo";
        present_second: "~i", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~ò", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~i", "~amo";
        present_subjunctive_second: "~i", "~ate";
        present_subjunctive_third: "~i", "~ino";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~ati", referred, "~amoci", "~atevi", "~ino";
        negative_imperative: "~arti|~are", referred, "~amoci|~amo", "~atevi|~ate", referred;
        infinitive: "~arsi";
        gerund: "~andosi";
        participle: "~antesi", "~atosi";
    ),
    // Overe: 5 words (bevere, overe, piovere, iovere, bbevere).
    conjugation!("overe", 5, "~ere";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~vi", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~ve", "~vero";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", referred, "~iamo", "~ete", "~ano";
        negative_imperative: "~ere", referred, "~iamo", "~ete", referred;
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~uto";
    ),
    // Tinguere: 5 words (tinguere, addistinguere, distinguere, inguere, estinguere).
    conjugation!("tinguere", 5, "~guere";
        present_first: "~guo", "~guiamo";
        present_second: "~gui", "~guete";
        present_third: "~gue", "~guono";
        imperfect_first: "~guevo", "~guevamo";
        imperfect_second: "~guevi", "~guevate";
        imperfect_third: "~gueva", "~guevano";
        past_historic_first: "~si", "~guemmo";
        past_historic_second: "~guesti", "~gueste";
        past_historic_third: "~se", "~sero";
        future_first: "~guerò", "~gueremo";
        future_second: "~guerai", "~guerete";
        future_third: "~guerà", "~gueranno";
        conditional_first: "~guerei", "~gueremmo";
        conditional_second: "~gueresti", "~guereste";
        conditional_third: "~guerebbe", "~guerebbero";
        present_subjunctive_first: "~gua", "~guiamo";
        present_subjunctive_second: "~gua", "~guiate";
        present_subjunctive_third: "~gua", "~guano";
        imperfect_subjunctive_first: "~guessi", "~guessimo";
        imperfect_subjunctive_second: "~guessi", "~gueste";
        imperfect_subjunctive_third: "~guesse", "~guessero";
        imperative: "~gui", referred, "~guiamo", "~guete", "~guano";
        negative_imperative: "~guere", referred, "~guiamo", "~guete", referred;
        infinitive: "~guere";
        gerund: "~guendo";
        participle: "~guente", "~to";
    ),
    // Cutere: 5 words (cutere, discutere, scotere, cotere, percotere).
    conjugation!("cutere", 5, "~tere";
        present_first: "~to", "~tiamo";
        present_second: "~ti", "~tete";
        present_third: "~te", "~tono";
        imperfect_first: "~tevo", "~tevamo";
        imperfect_second: "~tevi", "~tevate";
        imperfect_third: "~teva", "~tevano";
        past_historic_first: "~ssi", "~temmo";
        past_historic_second: "~testi", "~teste";
        past_historic_third: "~sse", "~ssero";
        future_first: "~terò", "~teremo";
        future_second: "~terai", "~terete";
        future_third: "~terà", "~teranno";
        conditional_first: "~terei", "~teremmo";
        conditional_second: "~teresti", "~tereste";
        conditional_third: "~terebbe", "~terebbero";
        present_subjunctive_first: "~ta", "~tiamo";
        present_subjunctive_second: "~ta", "~tiate";
        present_subjunctive_third: "~ta", "~tano";
        imperfect_subjunctive_first: "~tessi", "~tessimo";
        imperfect_subjunctive_second: "~tessi", "~teste";
        imperfect_subjunctive_third: "~tesse", "~tessero";
        imperative: "~ti", referred, "~tiamo", "~tete", "~tano";
        negative_imperative: "~tere", referred, "~tiamo", "~tete", referred;
        infinitive: "~tere";
        gerund: "~tendo";
        participle: "~tente", "~sso";
    ),
    // Primere: 5 words (primere, comprimere, imprimere, omprimere, esprimere).
    conjugation!("primere", 5, "~imere";
        present_first: "~imo", "~imiamo";
        present_second: "~imi", "~imete";
        present_third: "~ime", "~imono";
        imperfect_first: "~imevo", "~imevamo";
        imperfect_second: "~imevi", "~imevate";
        imperfect_third: "~imeva", "~imevano";
        past_historic_first: "~essi", "~imemmo";
        past_historic_second: "~imesti", "~imeste";
        past_historic_third: "~esse", "~essero";
        future_first: "~imerò", "~imeremo";
        future_second: "~imerai", "~imerete";
        future_third: "~imerà", "~imeranno";
        conditional_first: "~imerei", "~imeremmo";
        conditional_second: "~imeresti", "~imereste";
        conditional_third: "~imerebbe", "~imerebbero";
        present_subjunctive_first: "~ima", "~imiamo";
        present_subjunctive_second: "~ima", "~imiate";
        present_subjunctive_third: "~ima", "~imano";
        imperfect_subjunctive_first: "~imessi", "~imessimo";
        imperfect_subjunctive_second: "~imessi", "~imeste";
        imperfect_subjunctive_third: "~imesse", "~imessero";
        imperative: "~imi", referred, "~imiamo", "~imete", "~imano";
        negative_imperative: "~imere", referred, "~imiamo", "~imete", referred;
        infinitive: "~imere";
        gerund: "~imendo";
        participle: "~imente", "~esso";
    ),
    // Ncersi: 5 words (vincersi, ncersi, orcersi, convincersi, torcersi).
    conjugation!("ncersi", 5, "~cersi";
        present_first: "~co", "~ciamo";
        present_second: "~ci", "~cete";
        present_third: "~ce", "~cono";
        imperfect_first: "~cevo", "~cevamo";
        imperfect_second: "~cevi", "~cevate";
        imperfect_third: "~ceva", "~cevano";
        past_historic_first: "~si", "~cemmo";
        past_historic_second: "~cesti", "~ceste";
        past_historic_third: "~se", "~sero";
        future_first: "~cerò", "~ceremo";
        future_second: "~cerai", "~cerete";
        future_third: "~cerà", "~ceranno";
        conditional_first: "~cerei", "~ceremmo";
        conditional_second: "~ceresti", "~cereste";
        conditional_third: "~cerebbe", "~cerebbero";
        present_subjunctive_first: "~ca", "~ciamo";
        present_subjunctive_second: "~ca", "~ciate";
        present_subjunctive_third: "~ca", "~cano";
        imperfect_subjunctive_first: "~cessi", "~cessimo";
        imperfect_subjunctive_second: "~cessi", "~ceste";
        imperfect_subjunctive_third: "~cesse", "~cessero";
        imperative: "~citi", referred, "~ciamoci", "~cetevi", "~cano";
        negative_imperative: "~certi|~cere", referred, "~ciamoci|~ciamo", "~cetevi|~cete", referred;
        infinitive: "~cersi";
        gerund: "~cendosi";
        participle: "~centesi", "~tosi";
    ),
    // Fondere: 4 words (fondere, diffondere, onfondere, ffondere).
    conjugation!("fondere", 4, "~ondere";
        present_first: "~ondo", "~ondiamo";
        present_second: "~ondi", "~ondete";
        present_third: "~onde", "~ondono";
        imperfect_first: "~ondevo", "~ondevamo";
        imperfect_second: "~ondevi", "~ondevate";
        imperfect_third: "~ondeva", "~ondevano";
        past_historic_first: "~usi", "~ondemmo";
        past_historic_second: "~ondesti", "~ondeste";
        past_historic_third: "~use", "~usero";
        future_first: "~onderò", "~onderemo";
        future_second: "~onderai", "~onderete";
        future_third: "~onderà", "~onderanno";
        conditional_first: "~onderei", "~onderemmo";
        conditional_second: "~onderesti", "~ondereste";
        conditional_third: "~onderebbe", "~onderebbero";
        present_subjunctive_first: "~onda", "~ondiamo";
        present_subjunctive_second: "~onda", "~ondiate";
        present_subjunctive_third: "~onda", "~ondano";
        imperfect_subjunctive_first: "~ondessi", "~ondessimo";
        imperfect_subjunctive_second: "~ondessi", "~ondeste";
        imperfect_subjunctive_third: "~ondesse", "~ondessero";
        imperative: "~ondi", referred, "~ondiamo", "~ondete", "~ondano";
        negative_imperative: "~ondere", referred, "~ondiamo", "~ondete", referred;
        infinitive: "~ondere";
        gerund: "~ondendo";
        participle: "~ondente", "~uso";
    ),
    // Creare: 4 words (creare, meare, beare, buare).
    conjugation!("creare", 4, "~are";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~o", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~i", "~iamo";
        present_subjunctive_second: "~i", "~iate";
        present_subjunctive_third: "~i", "~ino";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~a", referred, "~iamo", "~ate", "~ino";
        negative_imperative: "~are", referred, "~iamo", "~ate", referred;
        infinitive: "~are";
        gerund: "~ando";
        participle: "~ante", "~ato";
    ),
    // Rolare: 4 words (sonare, rotare, scorare, rolare).
    conjugation!("rolare", 4, "~~are";
        present_first: "~u~o", "~~iamo";
        present_second: "~u~i", "~~ate";
        present_third: "~u~a", "~u~ano";
        imperfect_first: "~~avo", "~~avamo";
        imperfect_second: "~~avi", "~~avate";
        imperfect_third: "~~ava", "~~avano";
        past_historic_first: "~~ai", "~~ammo";
        past_historic_second: "~~asti", "~~aste";
        past_historic_third: "~~ò", "~~arono";
        future_first: "~~erò", "~~eremo";
        future_second: "~~erai", "~~erete";
        future_third: "~~erà", "~~eranno";
        conditional_first: "~~erei", "~~eremmo";
        conditional_second: "~~eresti", "~~ereste";
        conditional_third: "~~erebbe", "~~erebbero";
        present_subjunctive_first: "~u~i", "~~iamo";
        present_subjunctive_second: "~u~i", "~~iate";
        present_subjunctive_third: "~u~i", "~u~ino";
        imperfect_subjunctive_first: "~~assi", "~~assimo";
        imperfect_subjunctive_second: "~~assi", "~~aste";
        imperfect_subjunctive_third: "~~asse", "~~assero";
        imperative: "~u~a", referred, "~~iamo", "~~ate", "~u~ino";
        negative_imperative: "~~are", referred, "~~iamo", "~~ate", referred;
        infinitive: "~~are";
        gerund: "~~ando";
        participle: "~~ante", "~~ato";
    ),
    // Ttenersi: 4 words (tenersi, ttenersi, stenersi, sostenersi).
    conjugation!("ttenersi", 4, "~~nersi";
        present_first: "~~ngo", "~~niamo";
        present_second: "~i~ni", "~~nete";
        present_third: "~i~ne", "~~ngono";
        imperfect_first: "~~nevo", "~~nevamo";
        imperfect_second: "~~nevi", "~~nevate";
        imperfect_third: "~~neva", "~~nevano";
        past_historic_first: "~~nni", "~~nemmo";
        past_historic_second: "~~nesti", "~~neste";
        past_historic_third: "~~nne", "~~nnero";
        future_first: "~~rrò", "~~rremo";
        future_second: "~~rrai", "~~rrete";
        future_third: "~~rrà", "~~rranno";
        conditional_first: "~~rrei", "~~rremmo";
        conditional_second: "~~rresti", "~~rreste";
        conditional_third: "~~rrebbe", "~~rrebbero";
        present_subjunctive_first: "~~nga", "~~niamo";
        present_subjunctive_second: "~~nga", "~~niate";
        present_subjunctive_third: "~~nga", "~~ngano";
        imperfect_subjunctive_first: "~~nessi", "~~nessimo";
        imperfect_subjunctive_second: "~~nessi", "~~neste";
        imperfect_subjunctive_third: "~~nesse", "~~nessero";
        imperative: "~i~niti", referred, "~~niamoci", "~~netevi", "~~ngano";
        negative_imperative: "~~nerti|~~nere", referred, "~~niamoci|~~niamo", "~~netevi|~~nete", referred;
        infinitive: "~~nersi";
        gerund: "~~nendosi";
        participle: "~~nentesi", "~~nutosi";
    ),
    // Mettersi: 4 words (mettersi, promettersi, frammettersi, tromettersi).
    conjugation!("mettersi", 4, "~ettersi";
        present_first: "~etto", "~ettiamo";
        present_second: "~etti", "~ettete";
        present_third: "~ette", "~ettono";
        imperfect_first: "~ettevo", "~ettevamo";
        imperfect_second: "~ettevi", "~ettevate";
        imperfect_third: "~etteva", "~ettevano";
        past_historic_first: "~isi", "~ettemmo";
        past_historic_second: "~ettesti", "~etteste";
        past_historic_third: "~ise", "~isero";
        future_first: "~etterò", "~etteremo";
        future_second: "~etterai", "~etterete";
        future_third: "~etterà", "~etteranno";
        conditional_first: "~etterei", "~etteremmo";
        conditional_second: "~etteresti", "~ettereste";
        conditional_third: "~etterebbe", "~etterebbero";
        present_subjunctive_first: "~etta", "~ettiamo";
        present_subjunctive_second: "~etta", "~ettiate";
        present_subjunctive_third: "~etta", "~ettano";
        imperfect_subjunctive_first: "~ettessi", "~ettessimo";
        imperfect_subjunctive_second: "~ettessi", "~etteste";
        imperfect_subjunctive_third: "~ettesse", "~ettessero";
        imperative: "~ettiti", referred, "~ettiamoci", "~ettetevi", "~ettano";
        negative_imperative: "~etterti|~ettere", referred, "~ettiamoci|~ettiamo", "~ettetevi|~ettete", referred;
        infinitive: "~ettersi";
        gerund: "~ettendosi";
        participle: "~ettentesi", "~essosi";
    ),
    // Oire: 4 words (guaire, fluire, gire, oire).
    conjugation!("oire", 4, "~ire";
        present_first: "~isco", "~iamo";
        present_second: "~isci", "~ite";
        present_third: "~isce", "~iscono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~i", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~isca", "~iamo";
        present_subjunctive_second: "~isca", "~iate";
        present_subjunctive_third: "~isca", "~iscano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~isci", referred, "~iamo", "~ite", "~iscano";
        negative_imperative: "~ire", referred, "~iamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~endo";
        participle: "~ente", "~ito";
    ),
    // Riversi: 4 words (iscriversi, riversi, scriversi, criversi).
    conjugation!("riversi", 4, "~versi";
        present_first: "~vo", "~viamo";
        present_second: "~vi", "~vete";
        present_third: "~ve", "~vono";
        imperfect_first: "~vevo", "~vevamo";
        imperfect_second: "~vevi", "~vevate";
        imperfect_third: "~veva", "~vevano";
        past_historic_first: "~ssi", "~vemmo";
        past_historic_second: "~vesti", "~veste";
        past_historic_third: "~sse", "~ssero";
        future_first: "~verò", "~veremo";
        future_second: "~verai", "~verete";
        future_third: "~verà", "~veranno";
        conditional_first: "~verei", "~veremmo";
        conditional_second: "~veresti", "~vereste";
        conditional_third: "~verebbe", "~verebbero";
        present_subjunctive_first: "~va", "~viamo";
        present_subjunctive_second: "~va", "~viate";
        present_subjunctive_third: "~va", "~vano";
        imperfect_subjunctive_first: "~vessi", "~vessimo";
        imperfect_subjunctive_second: "~vessi", "~veste";
        imperfect_subjunctive_third: "~vesse", "~vessero";
        imperative: "~viti", referred, "~viamoci", "~vetevi", "~vano";
        negative_imperative: "~verti|~vere", referred, "~viamoci|~viamo", "~vetevi|~vete", referred;
        infinitive: "~versi";
        gerund: "~vendosi";
        participle: "~ventesi", "~ttosi";
    ),
    // Retrocedere: 4 words (concedere, retrocedere, ccedere, oncedere).
    conjugation!("retrocedere", 4, "~dere";
        present_first: "~do", "~diamo";
        present_second: "~di", "~dete";
        present_third: "~de", "~dono";
        imperfect_first: "~devo", "~devamo";
        imperfect_second: "~devi", "~devate";
        imperfect_third: "~deva", "~devano";
        past_historic_first: "~ssi|~detti|~dei", "~demmo";
        past_historic_second: "~desti", "~deste";
        past_historic_third: "~sse|~dette|~dé", "~ssero|~dettero|~derono";
        future_first: "~derò", "~deremo";
        future_second: "~derai", "~derete";
        future_third: "~derà", "~deranno";
        conditional_first: "~derei", "~deremmo";
        conditional_second: "~deresti", "~dereste";
        conditional_third: "~derebbe", "~derebbero";
        present_subjunctive_first: "~da", "~diamo";
        present_subjunctive_second: "~da", "~diate";
        present_subjunctive_third: "~da", "~dano";
        imperfect_subjunctive_first: "~dessi", "~dessimo";
        imperfect_subjunctive_second: "~dessi", "~deste";
        imperfect_subjunctive_third: "~desse", "~dessero";
        imperative: "~di", referred, "~diamo", "~dete", "~dano";
        negative_imperative: "~dere", referred, "~diamo", "~dete", referred;
        infinitive: "~dere";
        gerund: "~dendo";
        participle: "~dente", "~sso";
    ),
    // Vvedere: 4 words (provedere, vvedere, provvedere, rovvedere).
    conjugation!("vvedere", 4, "~edere";
        present_first: "~edo", "~ediamo";
        present_second: "~edi", "~edete";
        present_third: "~ede", "~edono";
        imperfect_first: "~edevo", "~edevamo";
        imperfect_second: "~edevi", "~edevate";
        imperfect_third: "~edeva", "~edevano";
        past_historic_first: "~idi", "~edemmo";
        past_historic_second: "~edesti", "~edeste";
        past_historic_third: "~ide", "~idero";
        future_first: "~ederò", "~ederemo";
        future_second: "~ederai", "~ederete";
        future_third: "~ederà", "~ederanno";
        conditional_first: "~ederei", "~ederemmo";
        conditional_second: "~ederesti", "~edereste";
        conditional_third: "~ederebbe", "~ederebbero";
        present_subjunctive_first: "~eda", "~ediamo";
        present_subjunctive_second: "~eda", "~ediate";
        present_subjunctive_third: "~eda", "~edano";
        imperfect_subjunctive_first: "~edessi", "~edessimo";
        imperfect_subjunctive_second: "~edessi", "~edeste";
        imperfect_subjunctive_third: "~edesse", "~edessero";
        imperative: "~edi", referred, "~ediamo", "~edete", "~edano";
        negative_imperative: "~edere", referred, "~ediamo", "~edete", referred;
        infinitive: "~edere";
        gerund: "~edendo";
        participle: "~edente|~eggente", "~eduto|~isto";
    ),
    // Ompire: 4 words (adempire, orgoglire, ompire, naglire).
    conjugation!("ompire", 4, "~ire";
        present_first: "~isco", "~iamo";
        present_second: "~isci", "~ite";
        present_third: "~isce", "~iscono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~ì", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~isca", "~iamo";
        present_subjunctive_second: "~isca", "~iate";
        present_subjunctive_third: "~isca", "~iscano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~isci", referred, "~iamo", "~ite", "~iscano";
        negative_imperative: "~ire", referred, "~iamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~iendo";
        participle: "~iente", "~ito";
    ),
    // Nascondersi: 4 words (nascondersi, spondersi, condersi, rispondersi).
    conjugation!("nascondersi", 4, "~ndersi";
        present_first: "~ndo", "~ndiamo";
        present_second: "~ndi", "~ndete";
        present_third: "~nde", "~ndono";
        imperfect_first: "~ndevo", "~ndevamo";
        imperfect_second: "~ndevi", "~ndevate";
        imperfect_third: "~ndeva", "~ndevano";
        past_historic_first: "~si", "~ndemmo";
        past_historic_second: "~ndesti", "~ndeste";
        past_historic_third: "~se", "~sero";
        future_first: "~nderò", "~nderemo";
        future_second: "~nderai", "~nderete";
        future_third: "~nderà", "~nderanno";
        conditional_first: "~nderei", "~nderemmo";
        conditional_second: "~nderesti", "~ndereste";
        conditional_third: "~nderebbe", "~nderebbero";
        present_subjunctive_first: "~nda", "~ndiamo";
        present_subjunctive_second: "~nda", "~ndiate";
        present_subjunctive_third: "~nda", "~ndano";
        imperfect_subjunctive_first: "~ndessi", "~ndessimo";
        imperfect_subjunctive_second: "~ndessi", "~ndeste";
        imperfect_subjunctive_third: "~ndesse", "~ndessero";
        imperative: "~nditi", referred, "~ndiamoci", "~ndetevi", "~ndano";
        negative_imperative: "~nderti|~ndere", referred, "~ndiamoci|~ndiamo", "~ndetevi|~ndete", referred;
        infinitive: "~ndersi";
        gerund: "~ndendosi";
        participle: "~ndentesi", "~stosi";
    ),
    // Scere: 4 words (mescere, scere, escere, pascere).
    conjugation!("scere", 4, "~ere";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~etti|~ei", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~ette|~é", "~ettero|~erono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~a", "~iamo";
        present_subjunctive_second: "~a", "~iate";
        present_subjunctive_third: "~a", "~ano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~i", referred, "~iamo", "~ete", "~ano";
        negative_imperative: "~ere", referred, "~iamo", "~ete", referred;
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~iuto";
    ),
    // Noiare: 4 words (noiare, vaiare, raiare, uoiare).
    conjugation!("noiare", 4, "~are";
        present_first: "~o", "~amo";
        present_second: "~", "~ate";
        present_third: "~a", "~ano";
        imperfect_first: "~avo", "~avamo";
        imperfect_second: "~avi", "~avate";
        imperfect_third: "~ava", "~avano";
        past_historic_first: "~ai", "~ammo";
        past_historic_second: "~asti", "~aste";
        past_historic_third: "~o", "~arono";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~", "~amo";
        present_subjunctive_second: "~", "~ate";
        present_subjunctive_third: "~", "~no";
        imperfect_subjunctive_first: "~assi", "~assimo";
        imperfect_subjunctive_second: "~assi", "~aste";
        imperfect_subjunctive_third: "~asse", "~assero";
        imperative: "~a", referred, "~amo", "~ate", "~no";
        negative_imperative: "~are", referred, "~amo", "~ate", referred;
        infinitive: "~are";
        gerund: "~ando";
        participle: "~ante", "~ato";
    ),
    // Arire: 4 words (arire, parire, apparire, omparire).
    conjugation!("arire", 4, "~rire";
        present_first: "~io|~risco", "~riamo";
        present_second: "~ri|~risci", "~rite";
        present_third: "~re|~risce", "~iono|~riscono";
        imperfect_first: "~rivo", "~rivamo";
        imperfect_second: "~rivi", "~rivate";
        imperfect_third: "~riva", "~rivano";
        past_historic_first: "~rvi", "~rimmo";
        past_historic_second: "~risti", "~riste";
        past_historic_third: "~rve", "~rvero";
        future_first: "~rirò", "~riremo";
        future_second: "~rirai", "~rirete";
        future_third: "~rirà", "~riranno";
        conditional_first: "~rirei", "~riremmo";
        conditional_second: "~riresti", "~rireste";
        conditional_third: "~rirebbe", "~rirebbero";
        present_subjunctive_first: "~ia|~risca", "~riamo";
        present_subjunctive_second: "~ia|~risca", "~riate";
        present_subjunctive_third: "~ia|~risca", "~iano|~riscano";
        imperfect_subjunctive_first: "~rissi", "~rissimo";
        imperfect_subjunctive_second: "~rissi", "~riste";
        imperfect_subjunctive_third: "~risse", "~rissero";
        imperative: "~ri|~risci", referred, "~riamo", "~rite", "~iano|~riscano";
        negative_imperative: "~rire", referred, "~riamo", "~rite", referred;
        infinitive: "~rire";
        gerund: "~rendo";
        participle: "~rente", "~rso";
    ),
    // Lire: 4 words (intravienire, lire, salire, assalire).
    conjugation!("lire", 4, "~ire";
        present_first: "~go", "~iamo";
        present_second: "~i", "~ite";
        present_third: "~e", "~gono";
        imperfect_first: "~ivo", "~ivamo";
        imperfect_second: "~ivi", "~ivate";
        imperfect_third: "~iva", "~ivano";
        past_historic_first: "~ii", "~immo";
        past_historic_second: "~isti", "~iste";
        past_historic_third: "~ì", "~irono";
        future_first: "~irò", "~iremo";
        future_second: "~irai", "~irete";
        future_third: "~irà", "~iranno";
        conditional_first: "~irei", "~iremmo";
        conditional_second: "~iresti", "~ireste";
        conditional_third: "~irebbe", "~irebbero";
        present_subjunctive_first: "~ga", "~iamo";
        present_subjunctive_second: "~ga", "~iate";
        present_subjunctive_third: "~ga", "~gano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~i", referred, "~iamo", "~ite", "~gano";
        negative_imperative: "~ire", referred, "~iamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~endo";
        participle: "~ente", "~ito";
    ),
    // Sedere: 4 words (sedere, ssedere, possedere, ossedere).
    conjugation!("sedere", 4, "~~dere";
        present_first: "~i~do|~~ggo", "~~diamo";
        present_second: "~i~di", "~~dete";
        present_third: "~i~de", "~i~dono|~~ggono";
        imperfect_first: "~~devo", "~~devamo";
        imperfect_second: "~~devi", "~~devate";
        imperfect_third: "~~deva", "~~devano";
        past_historic_first: "~~detti|~~dei", "~~demmo";
        past_historic_second: "~~desti", "~~deste";
        past_historic_third: "~~dette|~~dé", "~~dettero|~~derono";
        future_first: "~i~derò|~~derò", "~i~deremo|~~deremo";
        future_second: "~i~derai|~~derai", "~i~derete|~~derete";
        future_third: "~i~derà|~~derà", "~i~deranno|~~deranno";
        conditional_first: "~i~derei|~~derei", "~i~deremmo|~~deremmo";
        conditional_second: "~i~deresti|~~deresti", "~i~dereste|~~dereste";
        conditional_third: "~i~derebbe|~~derebbe", "~i~derebbero|~~derebbero";
        present_subjunctive_first: "~i~da|~~gga", "~~diamo";
        present_subjunctive_second: "~i~da|~~gga", "~~diate";
        present_subjunctive_third: "~i~da|~~gga", "~i~dano|~~ggano";
        imperfect_subjunctive_first: "~~dessi", "~~dessimo";
        imperfect_subjunctive_second: "~~dessi", "~~deste";
        imperfect_subjunctive_third: "~~desse", "~~dessero";
        imperative: "~i~di", referred, "~~diamo", "~~dete", "~i~dano|~~ggano";
        negative_imperative: "~~dere", referred, "~~diamo", "~~dete", referred;
        infinitive: "~~dere";
        gerund: "~~dendo";
        participle: "~~dente", "~~duto";
    ),
    // Oscersi: 4 words (oscersi, onoscersi, rescersi, crescersi).
    conjugation!("oscersi", 4, "~scersi";
        present_first: "~sco", "~sciamo";
        present_second: "~sci", "~scete";
        present_third: "~sce", "~scono";
        imperfect_first: "~scevo", "~scevamo";
        imperfect_second: "~scevi", "~scevate";
        imperfect_third: "~sceva", "~scevano";
        past_historic_first: "~bbi", "~scemmo";
        past_historic_second: "~scesti", "~sceste";
        past_historic_third: "~bbe", "~bbero";
        future_first: "~scerò", "~sceremo";
        future_second: "~scerai", "~scerete";
        future_third: "~scerà", "~sceranno";
        conditional_first: "~scerei", "~sceremmo";
        conditional_second: "~sceresti", "~scereste";
        conditional_third: "~scerebbe", "~scerebbero";
        present_subjunctive_first: "~sca", "~sciamo";
        present_subjunctive_second: "~sca", "~sciate";
        present_subjunctive_third: "~sca", "~scano";
        imperfect_subjunctive_first: "~scessi", "~scessimo";
        imperfect_subjunctive_second: "~scessi", "~sceste";
        imperfect_subjunctive_third: "~scesse", "~scessero";
        imperative: "~sciti", referred, "~sciamoci", "~scetevi", "~scano";
        negative_imperative: "~scerti|~scere", referred, "~sciamoci|~sciamo", "~scetevi|~scete", referred;
        infinitive: "~scersi";
        gerund: "~scendosi";
        participle: "~scentesi", "~sciutosi";
    ),
    // Mergersi: 4 words (mergersi, argersi, immergersi, spargersi).
    conjugation!("mergersi", 4, "~gersi";
        present_first: "~go", "~giamo";
        present_second: "~gi", "~gete";
        present_third: "~ge", "~gono";
        imperfect_first: "~gevo", "~gevamo";
        imperfect_second: "~gevi", "~gevate";
        imperfect_third: "~geva", "~gevano";
        past_historic_first: "~si", "~gemmo";
        past_historic_second: "~gesti", "~geste";
        past_historic_third: "~se", "~sero";
        future_first: "~gerò", "~geremo";
        future_second: "~gerai", "~gerete";
        future_third: "~gerà", "~geranno";
        conditional_first: "~gerei", "~geremmo";
        conditional_second: "~geresti", "~gereste";
        conditional_third: "~gerebbe", "~gerebbero";
        present_subjunctive_first: "~ga", "~giamo";
        present_subjunctive_second: "~ga", "~giate";
        present_subjunctive_third: "~ga", "~gano";
        imperfect_subjunctive_first: "~gessi", "~gessimo";
        imperfect_subjunctive_second: "~gessi", "~geste";
        imperfect_subjunctive_third: "~gesse", "~gessero";
        imperative: "~giti", referred, "~giamoci", "~getevi", "~gano";
        negative_imperative: "~gerti|~gere", referred, "~giamoci|~giamo", "~getevi|~gete", referred;
        infinitive: "~gersi";
        gerund: "~gendosi";
        participle: "~gentesi", "~sosi";
    ),
    // Andere: 4 words (andere, pandere, espandere, spandere).
    conjugation!("andere", 4, "~dere";
        present_first: "~do", "~diamo";
        present_second: "~di", "~dete";
        present_third: "~de", "~dono";
        imperfect_first: "~devo", "~devamo";
        imperfect_second: "~devi", "~devate";
        imperfect_third: "~deva", "~devano";
        past_historic_first: "~si", "~demmo";
        past_historic_second: "~desti", "~deste";
        past_historic_third: "~se", "~sero";
        future_first: "~derò", "~deremo";
        future_second: "~derai", "~derete";
        future_third: "~derà", "~deranno";
        conditional_first: "~derei", "~deremmo";
        conditional_second: "~deresti", "~dereste";
        conditional_third: "~derebbe", "~derebbero";
        present_subjunctive_first: "~da", "~diamo";
        present_subjunctive_second: "~da", "~diate";
        present_subjunctive_third: "~da", "~dano";
        imperfect_subjunctive_first: "~dessi", "~dessimo";
        imperfect_subjunctive_second: "~dessi", "~deste";
        imperfect_subjunctive_third: "~desse", "~dessero";
        imperative: "~di", referred, "~diamo", "~dete", "~dano";
        negative_imperative: "~dere", referred, "~diamo", "~dete", referred;
        infinitive: "~dere";
        gerund: "~dendo";
        participle: "~dente", "~to|~so";
    ),
    // Dursi: 4 words (dursi, prodursi, introdursi, ondursi).
    conjugation!("dursi", 4, "~ursi";
        present_first: "~uco", "~uciamo";
        present_second: "~uci", "~ucete";
        present_third: "~uce", "~ucono";
        imperfect_first: "~ucevo", "~ucevamo";
        imperfect_second: "~ucevi", "~ucevate";
        imperfect_third: "~uceva", "~ucevano";
        past_historic_first: "~ussi", "~ucemmo";
        past_historic_second: "~ucesti", "~uceste";
        past_historic_third: "~usse", "~ussero";
        future_first: "~urrò", "~urremo";
        future_second: "~urrai", "~urrete";
        future_third: "~urrà", "~urranno";
        conditional_first: "~urrei", "~urremmo";
        conditional_second: "~urresti", "~urreste";
        conditional_third: "~urrebbe", "~urrebbero";
        present_subjunctive_first: "~uca", "~uciamo";
        present_subjunctive_second: "~uca", "~uciate";
        present_subjunctive_third: "~uca", "~ucano";
        imperfect_subjunctive_first: "~ucessi", "~ucessimo";
        imperfect_subjunctive_second: "~ucessi", "~uceste";
        imperfect_subjunctive_third: "~ucesse", "~ucessero";
        imperative: "~uciti", referred, "~uciamoci", "~ucetevi", "~ucano";
        negative_imperative: "~urti|~urre", referred, "~uciamoci|~uciamo", "~ucetevi|~ucete", referred;
        infinitive: "~ursi";
        gerund: "~ucendosi";
        participle: "~ucentesi", "~ottosi";
    ),
    // Prirsi: 4 words (offrirsi, prirsi, oprirsi, aprirsi).
    conjugation!("prirsi", 4, "~~irsi";
        present_first: "~~o", "~~iamo";
        present_second: "~~i", "~~ite";
        present_third: "~~e", "~~ono";
        imperfect_first: "~~ivo", "~~ivamo";
        imperfect_second: "~~ivi", "~~ivate";
        imperfect_third: "~~iva", "~~ivano";
        past_historic_first: "~~ii|~e~si", "~~immo";
        past_historic_second: "~~isti", "~~iste";
        past_historic_third: "~~ì|~e~se", "~~irono|~e~sero";
        future_first: "~~irò", "~~iremo";
        future_second: "~~irai", "~~irete";
        future_third: "~~irà", "~~iranno";
        conditional_first: "~~irei", "~~iremmo";
        conditional_second: "~~iresti", "~~ireste";
        conditional_third: "~~irebbe", "~~irebbero";
        present_subjunctive_first: "~~a", "~~iamo";
        present_subjunctive_second: "~~a", "~~iate";
        present_subjunctive_third: "~~a", "~~ano";
        imperfect_subjunctive_first: "~~issi", "~~issimo";
        imperfect_subjunctive_second: "~~issi", "~~iste";
        imperfect_subjunctive_third: "~~isse", "~~issero";
        imperative: "~~iti", referred, "~~iamoci", "~~itevi", "~~ano";
        negative_imperative: "~~irti|~~ire", referred, "~~iamoci|~~iamo", "~~itevi|~~ite", referred;
        infinitive: "~~irsi";
        gerund: "~~endosi";
        participle: "~~entesi", "~e~tosi";
    ),
    // Ducere: 3 words (ducere, educere, onducere).
    conjugation!("ducere", 3, "~ucere";
        present_first: "~uco", "~uciamo";
        present_second: "~uci", "~ucete";
        present_third: "~uce", "~ucono";
        imperfect_first: "~ucevo", "~ucevamo";
        imperfect_second: "~ucevi", "~ucevate";
        imperfect_third: "~uceva", "~ucevano";
        past_historic_first: "~ussi", "~ucemmo";
        past_historic_second: "~ucesti", "~uceste";
        past_historic_third: "~usse", "~ussero";
        future_first: "~urrò", "~urremo";
        future_second: "~urrai", "~urrete";
        future_third: "~urrà", "~urranno";
        conditional_first: "~urrei", "~urremmo";
        conditional_second: "~urresti", "~urreste";
        conditional_third: "~urrebbe", "~urrebbero";
        present_subjunctive_first: "~uca", "~uciamo";
        present_subjunctive_second: "~uca", "~uciate";
        present_subjunctive_third: "~uca", "~ucano";
        imperfect_subjunctive_first: "~ucessi", "~ucessimo";
        imperfect_subjunctive_second: "~ucessi", "~uceste";
        imperfect_subjunctive_third: "~ucesse", "~ucessero";
        imperative: "~uci", referred, "~uciamo", "~ucete", "~ucano";
        negative_imperative: "~ucere", referred, "~uciamo", "~ucete", referred;
        infinitive: "~ucere";
        gerund: "~ucendo";
        participle: "~ucente", "~otto";
    ),
    // Hiedere: 3 words (edere, hiedere, chiedere).
    conjugation!("hiedere", 3, "~dere";
        present_first: "~do", "~diamo";
        present_second: "~di", "~dete";
        present_third: "~de", "~dono";
        imperfect_first: "~devo", "~devamo";
        imperfect_second: "~devi", "~devate";
        imperfect_third: "~deva", "~devano";
        past_historic_first: "~si", "~demmo";
        past_historic_second: "~desti", "~deste";
        past_historic_third: "~se", "~sero";
        future_first: "~derò", "~deremo";
        future_second: "~derai", "~derete";
        future_third: "~derà", "~deranno";
        conditional_first: "~derei", "~deremmo";
        conditional_second: "~deresti", "~dereste";
        conditional_third: "~derebbe", "~derebbero";
        present_subjunctive_first: "~da", "~diamo";
        present_subjunctive_second: "~da", "~diate";
        present_subjunctive_third: "~da", "~dano";
        imperfect_subjunctive_first: "~dessi", "~dessimo";
        imperfect_subjunctive_second: "~dessi", "~deste";
        imperfect_subjunctive_third: "~desse", "~dessero";
        imperative: "~di", referred, "~diamo", "~dete", "~dano";
        negative_imperative: "~dere", referred, "~diamo", "~dete", referred;
        infinitive: "~dere";
        gerund: "~dendo";
        participle: "~dente", "~sto";
    ),
    // Scuotere: 3 words (cuotere, scuotere, percuotere).
    conjugation!("scuotere", 3, "~u~tere";
        present_first: "~u~to", "~u~tiamo";
        present_second: "~u~ti", "~u~tete";
        present_third: "~u~te", "~u~tono";
        imperfect_first: "~u~tevo", "~u~tevamo";
        imperfect_second: "~u~tevi", "~u~tevate";
        imperfect_third: "~u~teva", "~u~tevano";
        past_historic_first: "~~ssi", "~u~temmo";
        past_historic_second: "~u~testi", "~u~teste";
        past_historic_third: "~~sse", "~~ssero";
        future_first: "~u~terò", "~u~teremo";
        future_second: "~u~terai", "~u~terete";
        future_third: "~u~terà", "~u~teranno";
        conditional_first: "~u~terei", "~u~teremmo";
        conditional_second: "~u~teresti", "~u~tereste";
        conditional_third: "~u~terebbe", "~u~terebbero";
        present_subjunctive_first: "~u~ta", "~u~tiamo";
        present_subjunctive_second: "~u~ta", "~u~tiate";
        present_subjunctive_third: "~u~ta", "~u~tano";
        imperfect_subjunctive_first: "~u~tessi", "~u~tessimo";
        imperfect_subjunctive_second: "~u~tessi", "~u~teste";
        imperfect_subjunctive_third: "~u~tesse", "~u~tessero";
        imperative: "~u~ti", referred, "~u~tiamo", "~u~tete", "~u~tano";
        negative_imperative: "~u~tere", referred, "~u~tiamo", "~u~tete", referred;
        infinitive: "~u~tere";
        gerund: "~u~tendo";
        participle: "~u~tente", "~~sso";
    ),
    // Tringere: 3 words (stringere, tringere, ostringere).
    conjugation!("tringere", 3, "~ingere";
        present_first: "~ingo", "~ingiamo";
        present_second: "~ingi", "~ingete";
        present_third: "~inge", "~ingono";
        imperfect_first: "~ingevo", "~ingevamo";
        imperfect_second: "~ingevi", "~ingevate";
        imperfect_third: "~ingeva", "~ingevano";
        past_historic_first: "~insi", "~ingemmo";
        past_historic_second: "~ingesti", "~ingeste";
        past_historic_third: "~inse", "~insero";
        future_first: "~ingerò", "~ingeremo";
        future_second: "~ingerai", "~ingerete";
        future_third: "~ingerà", "~ingeranno";
        conditional_first: "~ingerei", "~ingeremmo";
        conditional_second: "~ingeresti", "~ingereste";
        conditional_third: "~ingerebbe", "~ingerebbero";
        present_subjunctive_first: "~inga", "~ingiamo";
        present_subjunctive_second: "~inga", "~ingiate";
        present_subjunctive_third: "~inga", "~ingano";
        imperfect_subjunctive_first: "~ingessi", "~ingessimo";
        imperfect_subjunctive_second: "~ingessi", "~ingeste";
        imperfect_subjunctive_third: "~ingesse", "~ingessero";
        imperative: "~ingi", referred, "~ingiamo", "~ingete", "~ingano";
        negative_imperative: "~ingere", referred, "~ingiamo", "~ingete", referred;
        infinitive: "~ingere";
        gerund: "~ingendo";
        participle: "~ingente", "~etto";
    ),
    // Ercepire: 3 words (cepire, ercepire, oncepire).
    conjugation!("ercepire", 3, "~e~ire";
        present_first: "~e~isco", "~e~iamo";
        present_second: "~e~isci", "~e~ite";
        present_third: "~e~isce", "~e~iscono";
        imperfect_first: "~e~ivo", "~e~ivamo";
        imperfect_second: "~e~ivi", "~e~ivate";
        imperfect_third: "~e~iva", "~e~ivano";
        past_historic_first: "~e~ii", "~e~immo";
        past_historic_second: "~e~isti", "~e~iste";
        past_historic_third: "~e~ì", "~e~irono";
        future_first: "~e~irò", "~e~iremo";
        future_second: "~e~irai", "~e~irete";
        future_third: "~e~irà", "~e~iranno";
        conditional_first: "~e~irei", "~e~iremmo";
        conditional_second: "~e~iresti", "~e~ireste";
        conditional_third: "~e~irebbe", "~e~irebbero";
        present_subjunctive_first: "~e~isca", "~e~iamo";
        present_subjunctive_second: "~e~isca", "~e~iate";
        present_subjunctive_third: "~e~isca", "~e~iscano";
        imperfect_subjunctive_first: "~e~issi", "~e~issimo";
        imperfect_subjunctive_second: "~e~issi", "~e~iste";
        imperfect_subjunctive_third: "~e~isse", "~e~issero";
        imperative: "~e~isci", referred, "~e~iamo", "~e~ite", "~e~iscano";
        negative_imperative: "~e~ire", referred, "~e~iamo", "~e~ite", referred;
        infinitive: "~e~ire";
        gerund: "~e~endo";
        participle: "~i~iente", "~e~ito";
    ),
    // Pellere: 3 words (evellere, pellere, espellere).
    conjugation!("pellere", 3, "~e~lere";
        present_first: "~e~lo", "~e~liamo";
        present_second: "~e~li", "~e~lete";
        present_third: "~e~le", "~e~lono";
        imperfect_first: "~e~levo", "~e~levamo";
        imperfect_second: "~e~levi", "~e~levate";
        imperfect_third: "~e~leva", "~e~levano";
        past_historic_first: "~u~si", "~e~lemmo";
        past_historic_second: "~e~lesti", "~e~leste";
        past_historic_third: "~u~se", "~u~sero";
        future_first: "~e~lerò", "~e~leremo";
        future_second: "~e~lerai", "~e~lerete";
        future_third: "~e~lerà", "~e~leranno";
        conditional_first: "~e~lerei", "~e~leremmo";
        conditional_second: "~e~leresti", "~e~lereste";
        conditional_third: "~e~lerebbe", "~e~lerebbero";
        present_subjunctive_first: "~e~la", "~e~liamo";
        present_subjunctive_second: "~e~la", "~e~liate";
        present_subjunctive_third: "~e~la", "~e~lano";
        imperfect_subjunctive_first: "~e~lessi", "~e~lessimo";
        imperfect_subjunctive_second: "~e~lessi", "~e~leste";
        imperfect_subjunctive_third: "~e~lesse", "~e~lessero";
        imperative: "~e~li", referred, "~e~liamo", "~e~lete", "~e~lano";
        negative_imperative: "~e~lere", referred, "~e~liamo", "~e~lete", referred;
        infinitive: "~e~lere";
        gerund: "~e~lendo";
        participle: "~e~lente", "~u~so";
    ),
    // Acersi: 3 words (acersi, piacersi, iacersi).
    conjugation!("acersi", 3, "~ersi";
        present_first: "~cio", "~ciamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ciono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~qui", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~que", "~quero";
        future_first: "~erò", "~eremo";
        future_second: "~erai", "~erete";
        future_third: "~erà", "~eranno";
        conditional_first: "~erei", "~eremmo";
        conditional_second: "~eresti", "~ereste";
        conditional_third: "~erebbe", "~erebbero";
        present_subjunctive_first: "~cia", "~ciamo";
        present_subjunctive_second: "~cia", "~ciate";
        present_subjunctive_third: "~cia", "~ciano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~iti", referred, "~ciamoci", "~etevi", "~ciano";
        negative_imperative: "~erti|~ere", referred, "~ciamoci|~ciamo", "~etevi|~ete", referred;
        infinitive: "~ersi";
        gerund: "~endosi";
        participle: "~entesi", "~iutosi";
    ),
    // Scuotersi: 3 words (cuotersi, percuotersi, scuotersi).
    conjugation!("scuotersi", 3, "~u~tersi";
        present_first: "~u~to", "~u~tiamo";
        present_second: "~u~ti", "~u~tete";
        present_third: "~u~te", "~u~tono";
        imperfect_first: "~u~tevo", "~u~tevamo";
        imperfect_second: "~u~tevi", "~u~tevate";
        imperfect_third: "~u~teva", "~u~tevano";
        past_historic_first: "~~ssi", "~u~temmo";
        past_historic_second: "~u~testi", "~u~teste";
        past_historic_third: "~~sse", "~~ssero";
        future_first: "~u~terò", "~u~teremo";
        future_second: "~u~terai", "~u~terete";
        future_third: "~u~terà", "~u~teranno";
        conditional_first: "~u~terei", "~u~teremmo";
        conditional_second: "~u~teresti", "~u~tereste";
        conditional_third: "~u~terebbe", "~u~terebbero";
        present_subjunctive_first: "~u~ta", "~u~tiamo";
        present_subjunctive_second: "~u~ta", "~u~tiate";
        present_subjunctive_third: "~u~ta", "~u~tano";
        imperfect_subjunctive_first: "~u~tessi", "~u~tessimo";
        imperfect_subjunctive_second: "~u~tessi", "~u~teste";
        imperfect_subjunctive_third: "~u~tesse", "~u~tessero";
        imperative: "~u~titi", referred, "~u~tiamoci", "~u~tetevi", "~u~tano";
        negative_imperative: "~u~terti|~u~tere", referred, "~u~tiamoci|~u~tiamo", "~u~tetevi|~u~tete", referred;
        infinitive: "~u~tersi";
        gerund: "~u~tendosi";
        participle: "~u~tentesi", "~~ssosi";
    ),
    // Ire: 1 words (ire).
    conjugation!("ire", 1, "ire";
        present_first: "ngo", "imo";
        present_second: "ni", "ite";
        present_third: "ne", "ngono";
        imperfect_first: "ivo", "ivamo";
        imperfect_second: "ivi", "ivate";
        imperfect_third: "iva|gia", "ivano";
        past_historic_first: "ii", "immo";
        past_historic_second: "isti", "iste";
        past_historic_third: "i", "irono";
        future_first: "irò", "iremo";
        future_second: "irai", "irete";
        future_third: "irà", "iranno";
        conditional_first: "irei", "iremmo";
        conditional_second: "iresti", "ireste";
        conditional_third: "irebbe", "irebbero";
        present_subjunctive_first: "nga", "niamo";
        present_subjunctive_second: "nga", "niate";
        present_subjunctive_third: "nga", "ngano";
        imperfect_subjunctive_first: "issi", "issimo";
        imperfect_subjunctive_second: "issi", "iste";
        imperfect_subjunctive_third: "isse", "issero";
        imperative: "niti", referred, "niamoci", "ite", "ngano";
        negative_imperative: "rti", referred, "niamoci", "netevi", referred;
        infinitive: "ire";
        gerund: "endo";
        participle: "nentesi", "ito";
    ),
    // Are: 1 words (are).
    conjugation!("are", 1, "are";
        present_first: "o", "iamo";
        present_second: "ai", "ate";
        present_third: "a", "anno";
        imperfect_first: "avo", "avamo";
        imperfect_second: "avi", "avate";
        imperfect_third: "ava", "avano";
        past_historic_first: "etti", "emmo";
        past_historic_second: "esti", "este";
        past_historic_third: "ette", "ettero";
        future_first: "arò", "aremo";
        future_second: "arai", "arete";
        future_third: "arà", "aranno";
        conditional_first: "arei", "aremmo";
        conditional_second: "aresti", "areste";
        conditional_third: "arebbe", "arebbero";
        present_subjunctive_first: "ia", "iamo";
        present_subjunctive_second: "ia", "iate";
        present_subjunctive_third: "ia", "iano";
        imperfect_subjunctive_first: "essi", "essimo";
        imperfect_subjunctive_second: "essi", "este";
        imperfect_subjunctive_third: "esse", "essero";
        imperative: "ai|a'", referred, "iamo", "ate", "iano";
        negative_imperative: "are", referred, "iamo", "ate", referred;
        infinitive: "are";
        gerund: "ando";
        participle: "ante", "ato";
    ),
    // Ere: 1 words (ere).
    conjugation!("ere", 1, "ere";
        present_first: "o", "iamo";
        present_second: "i", "ate";
        present_third: "e", "anno";
        imperfect_first: "avo", "avamo";
        imperfect_second: "avi", "avate";
        imperfect_third: "ava", "avano";
        past_historic_first: "etti", "emmo";
        past_historic_second: "esti", "este";
        past_historic_third: "ette", "ettero";
        future_first: "arò", "aremo";
        future_second: "arai", "arete";
        future_third: "arà", "aranno";
        conditional_first: "arei", "aremmo";
        conditional_second: "aresti", "areste";
        conditional_third: "arebbe", "arebbero";
        present_subjunctive_first: "ia", "iamo";
        present_subjunctive_second: "ia", "iate";
        present_subjunctive_third: "ia", "iano";
        imperfect_subjunctive_first: "essi", "essimo";
        imperfect_subjunctive_second: "essi", "este";
        imperfect_subjunctive_third: "esse", "essero";
        imperative: "ai", referred, "iamo", "ate", "iano";
        negative_imperative: "are", referred, "iamo", "ate", referred;
        infinitive: "ere";
        gerund: "ando";
        participle: "ante", "ato";
    ),
    // Rre: 1 words (rre).
    conjugation!("rre", 1, "rre";
        present_first: "ngo", "niamo";
        present_second: "ni", "nete";
        present_third: "ne", "ngono";
        imperfect_first: "nevo", "nevamo";
        imperfect_second: "nevi", "nevate";
        imperfect_third: "neva", "nevano";
        past_historic_first: "si", "nemmo";
        past_historic_second: "nesti", "neste";
        past_historic_third: "se", "sero";
        future_first: "rrò", "rremo";
        future_second: "rrai", "rrete";
        future_third: "rrà", "rranno";
        conditional_first: "rrei", "rremmo";
        conditional_second: "rresti", "rreste";
        conditional_third: "rrebbe", "rrebbero";
        present_subjunctive_first: "nga", "niamo";
        present_subjunctive_second: "nga", "niate";
        present_subjunctive_third: "nga", "ngano";
        imperfect_subjunctive_first: "nessi", "nessimo";
        imperfect_subjunctive_second: "nessi", "neste";
        imperfect_subjunctive_third: "nesse", "nessero";
        imperative: "ni", referred, "niamo", "nete", "ngano";
        negative_imperative: "rre", referred, "niamo", "nete", referred;
        infinitive: "rre";
        gerund: "nendo";
        participle: "nente", "sto";
    ),
    // Rsi: 1 words (rsi).
    conjugation!("rsi", 1, "rsi";
        present_first: "ngo", "niamo";
        present_second: "ni", "nete";
        present_third: "ne", "ngono";
        imperfect_first: "nevo", "nevamo";
        imperfect_second: "nevi", "nevate";
        imperfect_third: "neva", "nevano";
        past_historic_first: "si", "nemmo";
        past_historic_second: "nesti", "neste";
        past_historic_third: "se", "sero";
        future_first: "rrò", "rremo";
        future_second: "rrai", "rrete";
        future_third: "rrà", "rranno";
        conditional_first: "rrei", "rremmo";
        conditional_second: "rresti", "rreste";
        conditional_third: "rrebbe", "rrebbero";
        present_subjunctive_first: "nga", "niamo";
        present_subjunctive_second: "nga", "niate";
        present_subjunctive_third: "nga", "ngano";
        imperfect_subjunctive_first: "nessi", "nessimo";
        imperfect_subjunctive_second: "nessi", "neste";
        imperfect_subjunctive_third: "nesse", "nessero";
        imperative: "niti", referred, "niamoci", "netevi", "ngano";
        negative_imperative: "rti|rre", referred, "niamoci|niamo", "netevi|nete", referred;
        infinitive: "rsi";
        gerund: "nendosi";
        participle: "nentesi", "stosi";
    ),
];
