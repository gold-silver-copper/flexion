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
    /// Like `girare`: 5,878 words.
    Girare,
    /// Like `tapparsi`: 1,289 words.
    Tapparsi,
    /// Like `giocare`: 992 words.
    Giocare,
    /// Like `uscire`: 770 words.
    Uscire,
    /// Like `raddoppiare`: 727 words.
    Raddoppiare,
    /// Like `lanciare`: 694 words.
    Lanciare,
    /// Like `imboscarsi`: 260 words.
    Imboscarsi,
    /// Like `incarognirsi`: 220 words.
    Incarognirsi,
    /// Like `ripigliarsi`: 170 words.
    Ripigliarsi,
    /// Like `volgere`: 130 words.
    Volgere,
    /// Like `lasciarsi`: 129 words.
    Lasciarsi,
    /// Like `ardere`: 95 words.
    Ardere,
    /// Like `essere`: 93 words.
    Essere,
    /// Like `prendere`: 79 words.
    Prendere,
    /// Like `partire`: 62 words.
    Partire,
    /// Like `mettere`: 56 words.
    Mettere,
    /// Like `disporre`: 53 words.
    Disporre,
    /// Like `reggere`: 37 words.
    Reggere,
    /// Like `tingersi`: 36 words.
    Tingersi,
    /// Like `perdersi`: 36 words.
    Perdersi,
    /// Like `urgere`: 30 words.
    Urgere,
    /// Like `venire`: 29 words.
    Venire,
    /// Like `torcere`: 26 words.
    Torcere,
    /// Like `scrivere`: 26 words.
    Scrivere,
    /// Like `dipartirsi`: 26 words.
    Dipartirsi,
    /// Like `ricorrere`: 25 words.
    Ricorrere,
    /// Like `spargere`: 25 words.
    Spargere,
    /// Like `sbattersi`: 25 words.
    Sbattersi,
    /// Like `rigiostrare`: 25 words.
    Rigiostrare,
    /// Like `battere`: 24 words.
    Battere,
    /// Like `condurre`: 24 words.
    Condurre,
    /// Like `fare`: 24 words.
    Fare,
    /// Like `tenere`: 22 words.
    Tenere,
    /// Like `deviare`: 22 words.
    Deviare,
    /// Like `prendersi`: 22 words.
    Prendersi,
    /// Like `riconoscere`: 21 words.
    Riconoscere,
    /// Like `sciogliere`: 21 words.
    Sciogliere,
    /// Like `imporsi`: 20 words.
    Imporsi,
    /// Like `farsi`: 20 words.
    Farsi,
    /// Like `esaurire`: 19 words.
    Esaurire,
    /// Like `fondere`: 18 words.
    Fondere,
    /// Like `dire`: 18 words.
    Dire,
    /// Like `assistere`: 17 words.
    Assistere,
    /// Like `ritrarre`: 16 words.
    Ritrarre,
    /// Like `mettersi`: 15 words.
    Mettersi,
    /// Like `reggersi`: 15 words.
    Reggersi,
    /// Like `imprimere`: 14 words.
    Imprimere,
    /// Like `tacere`: 13 words.
    Tacere,
    /// Like `connettere`: 13 words.
    Connettere,
    /// Like `spegnere`: 13 words.
    Spegnere,
    /// Like `ricadere`: 12 words.
    Ricadere,
    /// Like `vendersi`: 12 words.
    Vendersi,
    /// Like `ridursi`: 12 words.
    Ridursi,
    /// Like `presumere`: 11 words.
    Presumere,
    /// Like `trattenersi`: 11 words.
    Trattenersi,
    /// Like `aprire`: 10 words.
    Aprire,
    /// Like `prefiggere`: 10 words.
    Prefiggere,
    /// Like `contrarsi`: 10 words.
    Contrarsi,
    /// Like `vedere`: 9 words.
    Vedere,
    /// Like `abbonare`: 9 words.
    Abbonare,
    /// Like `vincersi`: 9 words.
    Vincersi,
    /// Like `sviarsi`: 9 words.
    Sviarsi,
    /// Like `rompere`: 8 words.
    Rompere,
    /// Like `muovere`: 8 words.
    Muovere,
    /// Like `stringere`: 8 words.
    Stringere,
    /// Like `decuocere`: 8 words.
    Decuocere,
    /// Like `corrispondere`: 8 words.
    Corrispondere,
    /// Like `flettersi`: 8 words.
    Flettersi,
    /// Like `noiare`: 8 words.
    Noiare,
    /// Like `valere`: 7 words.
    Valere,
    /// Like `apparire`: 7 words.
    Apparire,
    /// Like `estinguere`: 7 words.
    Estinguere,
    /// Like `risolvere`: 7 words.
    Risolvere,
    /// Like `escutere`: 7 words.
    Escutere,
    /// Like `espellere`: 7 words.
    Espellere,
    /// Like `stare`: 7 words.
    Stare,
    /// Like `sciogliersi`: 7 words.
    Sciogliersi,
    /// Like `rivolere`: 6 words.
    Rivolere,
    /// Like `ducere`: 6 words.
    Ducere,
    /// Like `risalire`: 6 words.
    Risalire,
    /// Like `fulgere`: 6 words.
    Fulgere,
    /// Like `compire`: 6 words.
    Compire,
    /// Like `consentire`: 6 words.
    Consentire,
    /// Like `scriversi`: 6 words.
    Scriversi,
    /// Like `dirsi`: 6 words.
    Dirsi,
    /// Like `coprirsi`: 6 words.
    Coprirsi,
    /// Like `esaurirsi`: 6 words.
    Esaurirsi,
    /// Like `rivalersi`: 6 words.
    Rivalersi,
    /// Like `ire`: 5 words.
    Ire,
    /// Like `richiedere`: 5 words.
    Richiedere,
    /// Like `riscuotere`: 5 words.
    Riscuotere,
    /// Like `godere`: 5 words.
    Godere,
    /// Like `dirigere`: 5 words.
    Dirigere,
    /// Like `sedere`: 5 words.
    Sedere,
    /// Like `retrocedere`: 5 words.
    Retrocedere,
    /// Like `sviare`: 5 words.
    Sviare,
    /// Like `rimuoversi`: 5 words.
    Rimuoversi,
    /// Like `fruire`: 5 words.
    Fruire,
    /// Like `provvedere`: 5 words.
    Provvedere,
    /// Like `travedere`: 5 words.
    Travedere,
    /// Like `sapere`: 5 words.
    Sapere,
    /// Like `diffondersi`: 5 words.
    Diffondersi,
    /// Like `vedersi`: 5 words.
    Vedersi,
    /// Like `evolversi`: 5 words.
    Evolversi,
    /// Like `traggere`: 5 words.
    Traggere,
    /// Like `inerire`: 5 words.
    Inerire,
    /// Like `empire`: 4 words.
    Empire,
    /// Like `morire`: 4 words.
    Morire,
    /// Like `vivere`: 4 words.
    Vivere,
    /// Like `ricucire`: 4 words.
    Ricucire,
    /// Like `disfare`: 4 words.
    Disfare,
    /// Like `creare`: 4 words.
    Creare,
    /// Like `competere`: 4 words.
    Competere,
    /// Like `torre`: 4 words.
    Torre,
    /// Like `conoscersi`: 4 words.
    Conoscersi,
    /// Like `immergersi`: 4 words.
    Immergersi,
    /// Like `scindere`: 4 words.
    Scindere,
    /// Like `spandere`: 4 words.
    Spandere,
    /// Like `nascondersi`: 4 words.
    Nascondersi,
    /// Like `piacersi`: 4 words.
    Piacersi,
    /// Like `stringersi`: 4 words.
    Stringersi,
    /// Like `imprimersi`: 4 words.
    Imprimersi,
    /// Like `riscuotersi`: 4 words.
    Riscuotersi,
    /// Like `mescere`: 4 words.
    Mescere,
    /// Like `piovere`: 3 words.
    Piovere,
    /// Like `potere`: 3 words.
    Potere,
    /// Like `rinascere`: 3 words.
    Rinascere,
    /// Like `dolere`: 3 words.
    Dolere,
    /// Like `soffrire`: 3 words.
    Soffrire,
    /// Like `disseppellire`: 3 words.
    Disseppellire,
    /// Like `divellere`: 3 words.
    Divellere,
    /// Like `redigere`: 3 words.
    Redigere,
    /// Like `possedere`: 3 words.
    Possedere,
    /// Like `rompersi`: 3 words.
    Rompersi,
    /// Like `appercepire`: 3 words.
    Appercepire,
    /// Like `fendere`: 3 words.
    Fendere,
    /// Like `rifrangere`: 3 words.
    Rifrangere,
    /// Like `estinguersi`: 3 words.
    Estinguersi,
    /// Like `disfarsi`: 3 words.
    Disfarsi,
    /// Like `imbevere`: 3 words.
    Imbevere,
    /// Like `affocare`: 3 words.
    Affocare,
    /// Like `convenirsi`: 3 words.
    Convenirsi,
    /// Like `smucciare`: 3 words.
    Smucciare,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 143] = [
        Conjugation::Girare,
        Conjugation::Tapparsi,
        Conjugation::Giocare,
        Conjugation::Uscire,
        Conjugation::Raddoppiare,
        Conjugation::Lanciare,
        Conjugation::Imboscarsi,
        Conjugation::Incarognirsi,
        Conjugation::Ripigliarsi,
        Conjugation::Volgere,
        Conjugation::Lasciarsi,
        Conjugation::Ardere,
        Conjugation::Essere,
        Conjugation::Prendere,
        Conjugation::Partire,
        Conjugation::Mettere,
        Conjugation::Disporre,
        Conjugation::Reggere,
        Conjugation::Tingersi,
        Conjugation::Perdersi,
        Conjugation::Urgere,
        Conjugation::Venire,
        Conjugation::Torcere,
        Conjugation::Scrivere,
        Conjugation::Dipartirsi,
        Conjugation::Ricorrere,
        Conjugation::Spargere,
        Conjugation::Sbattersi,
        Conjugation::Rigiostrare,
        Conjugation::Battere,
        Conjugation::Condurre,
        Conjugation::Fare,
        Conjugation::Tenere,
        Conjugation::Deviare,
        Conjugation::Prendersi,
        Conjugation::Riconoscere,
        Conjugation::Sciogliere,
        Conjugation::Imporsi,
        Conjugation::Farsi,
        Conjugation::Esaurire,
        Conjugation::Fondere,
        Conjugation::Dire,
        Conjugation::Assistere,
        Conjugation::Ritrarre,
        Conjugation::Mettersi,
        Conjugation::Reggersi,
        Conjugation::Imprimere,
        Conjugation::Tacere,
        Conjugation::Connettere,
        Conjugation::Spegnere,
        Conjugation::Ricadere,
        Conjugation::Vendersi,
        Conjugation::Ridursi,
        Conjugation::Presumere,
        Conjugation::Trattenersi,
        Conjugation::Aprire,
        Conjugation::Prefiggere,
        Conjugation::Contrarsi,
        Conjugation::Vedere,
        Conjugation::Abbonare,
        Conjugation::Vincersi,
        Conjugation::Sviarsi,
        Conjugation::Rompere,
        Conjugation::Muovere,
        Conjugation::Stringere,
        Conjugation::Decuocere,
        Conjugation::Corrispondere,
        Conjugation::Flettersi,
        Conjugation::Noiare,
        Conjugation::Valere,
        Conjugation::Apparire,
        Conjugation::Estinguere,
        Conjugation::Risolvere,
        Conjugation::Escutere,
        Conjugation::Espellere,
        Conjugation::Stare,
        Conjugation::Sciogliersi,
        Conjugation::Rivolere,
        Conjugation::Ducere,
        Conjugation::Risalire,
        Conjugation::Fulgere,
        Conjugation::Compire,
        Conjugation::Consentire,
        Conjugation::Scriversi,
        Conjugation::Dirsi,
        Conjugation::Coprirsi,
        Conjugation::Esaurirsi,
        Conjugation::Rivalersi,
        Conjugation::Ire,
        Conjugation::Richiedere,
        Conjugation::Riscuotere,
        Conjugation::Godere,
        Conjugation::Dirigere,
        Conjugation::Sedere,
        Conjugation::Retrocedere,
        Conjugation::Sviare,
        Conjugation::Rimuoversi,
        Conjugation::Fruire,
        Conjugation::Provvedere,
        Conjugation::Travedere,
        Conjugation::Sapere,
        Conjugation::Diffondersi,
        Conjugation::Vedersi,
        Conjugation::Evolversi,
        Conjugation::Traggere,
        Conjugation::Inerire,
        Conjugation::Empire,
        Conjugation::Morire,
        Conjugation::Vivere,
        Conjugation::Ricucire,
        Conjugation::Disfare,
        Conjugation::Creare,
        Conjugation::Competere,
        Conjugation::Torre,
        Conjugation::Conoscersi,
        Conjugation::Immergersi,
        Conjugation::Scindere,
        Conjugation::Spandere,
        Conjugation::Nascondersi,
        Conjugation::Piacersi,
        Conjugation::Stringersi,
        Conjugation::Imprimersi,
        Conjugation::Riscuotersi,
        Conjugation::Mescere,
        Conjugation::Piovere,
        Conjugation::Potere,
        Conjugation::Rinascere,
        Conjugation::Dolere,
        Conjugation::Soffrire,
        Conjugation::Disseppellire,
        Conjugation::Divellere,
        Conjugation::Redigere,
        Conjugation::Possedere,
        Conjugation::Rompersi,
        Conjugation::Appercepire,
        Conjugation::Fendere,
        Conjugation::Rifrangere,
        Conjugation::Estinguersi,
        Conjugation::Disfarsi,
        Conjugation::Imbevere,
        Conjugation::Affocare,
        Conjugation::Convenirsi,
        Conjugation::Smucciare,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 143] = [
    // Girare: 5,878 words (accusare, accreditare, vulnerare, lavare, nuotare, ordinare).
    conjugation!("girare", 5878, "~are";
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
    // Tapparsi: 1,289 words (lavarsi, abbandonarsi, abbassarsi, tirarsi, abbronzarsi, ossidarsi).
    conjugation!("tapparsi", 1289, "~arsi";
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
    // Giocare: 992 words (pagare, pizzicare, vocare, nevicare, rogare, interrogare).
    conjugation!("giocare", 992, "~are";
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
    // Uscire: 770 words (fire, fallire, finire, punire, approfondire, zittire).
    conjugation!("uscire", 770, "~ire";
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
    // Raddoppiare: 727 words (odiare, studiare, invecchiare, scoppiare, abbaiare, abbreviare).
    conjugation!("raddoppiare", 727, "~are";
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
    // Lanciare: 694 words (mangiare, lasciare, abbracciare, lanciare, baciare, pronunciare).
    conjugation!("lanciare", 694, "~iare";
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
    // Imboscarsi: 260 words (calcificarsi, coniugarsi, impiegarsi, obbligarsi, asciugarsi, impiccarsi).
    conjugation!("imboscarsi", 260, "~arsi";
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
    // Incarognirsi: 220 words (ferirsi, riferirsi, stupirsi, trasferirsi, riunirsi, unirsi).
    conjugation!("incarognirsi", 220, "~irsi";
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
    // Ripigliarsi: 170 words (abbacchiarsi, annoiarsi, arrabbiarsi, svegliarsi, scambiarsi, risparmiarsi).
    conjugation!("ripigliarsi", 170, "~arsi";
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
    // Volgere: 130 words (dipingere, piangere, giungere, fingere, volgere, ungere).
    conjugation!("volgere", 130, "~gere";
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
    // Lasciarsi: 129 words (affacciarsi, cacciarsi, bruciarsi, appoggiarsi, baciarsi, abbracciarsi).
    conjugation!("lasciarsi", 129, "~iarsi";
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
    // Ardere: 95 words (chiudere, dividere, perdere, rodere, ardere, decidere).
    conjugation!("ardere", 95, "~dere";
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
    // Essere: 93 words (essere, colere, dicere, temere, cedere, vendere).
    conjugation!("essere", 93, "~ere";
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
        participle: "~ente", "~uto";
    ),
    // Prendere: 79 words (prendere, dipendere, offendere, accendere, contundere, tendere).
    conjugation!("prendere", 79, "~ndere";
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
    // Partire: 62 words (dormire, sentire, partire, fuggire, divertire, servire).
    conjugation!("partire", 62, "~ire";
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
    // Mettere: 56 words (mettere, smettere, commettere, promettere, ammettere, dimettere).
    conjugation!("mettere", 56, "~ettere";
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
    // Disporre: 53 words (porre, deporre, decomporre, comporre, opporre, proporre).
    conjugation!("disporre", 53, "~rre";
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
    // Reggere: 37 words (leggere, friggere, proteggere, figgere, struggere, correggere).
    conjugation!("reggere", 37, "~ggere";
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
    // Tingersi: 36 words (accorgersi, rivolgersi, aggiungersi, dirigersi, ricongiungersi, spingersi).
    conjugation!("tingersi", 36, "~gersi";
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
    // Perdersi: 36 words (concludersi, dividersi, chiedersi, concedersi, perdersi, uccidersi).
    conjugation!("perdersi", 36, "~dersi";
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
    // Urgere: 30 words (serpere, tangere, volvere, pedere, acquiescere, concernere).
    conjugation!("urgere", 30, "~ere";
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
    // Venire: 29 words (venire, avvenire, convenire, pervenire, divenire, provenire).
    conjugation!("venire", 29, "~~nire";
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
    // Torcere: 26 words (vincere, convincere, torcere, attorcere, contorcere, detorcere).
    conjugation!("torcere", 26, "~cere";
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
    // Scrivere: 26 words (scrivere, iscrivere, descrivere, prescrivere, circonscrivere, circoscrivere).
    conjugation!("scrivere", 26, "~vere";
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
    // Dipartirsi: 26 words (riempirsi, vestirsi, nutrirsi, divertirsi, sentirsi, servirsi).
    conjugation!("dipartirsi", 26, "~irsi";
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
    // Ricorrere: 25 words (correre, scorrere, trascorrere, occorrere, ricorrere, decorrere).
    conjugation!("ricorrere", 25, "~rere";
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
    // Spargere: 25 words (aspergere, immergere, detergere, emergere, reimmergere, risommergere).
    conjugation!("spargere", 25, "~gere";
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
    // Sbattersi: 25 words (abbattersi, assumersi, imbattersi, battersi, ripetersi, sbattersi).
    conjugation!("sbattersi", 25, "~ersi";
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
        negative_imperative: "~erti|~ere", referred, "~iamoci", "~etevi", referred;
        infinitive: "~ersi";
        gerund: "~endosi";
        participle: "~entesi", "~utosi";
    ),
    // Rigiostrare: 25 words (obstare, torneare, equidistare, retrogradare, protuberare, strapiombare).
    conjugation!("rigiostrare", 25, "~are";
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
    // Battere: 24 words (battere, fottere, compiere, abbattere, combattere, ripetere).
    conjugation!("battere", 24, "~ere";
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
    // Condurre: 24 words (dedurre, condurre, sedurre, tradurre, produrre, ridurre).
    conjugation!("condurre", 24, "~urre";
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
    // Fare: 24 words (fare, rifare, tumefare, contraffare, stupefare, sopraffare).
    conjugation!("fare", 24, "~are";
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
    // Tenere: 22 words (tenere, rimanere, contenere, appartenere, ritenere, mantenere).
    conjugation!("tenere", 22, "~~nere";
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
    // Deviare: 22 words (avviare, notare, inviare, deviare, rinviare, espiare).
    conjugation!("deviare", 22, "~are";
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
    // Prendersi: 22 words (sorprendersi, riprendersi, intendersi, difendersi, prendersi, arrendersi).
    conjugation!("prendersi", 22, "~ndersi";
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
    // Riconoscere: 21 words (cognoscere, crescere, conoscere, decrescere, riconoscere, anticonoscere).
    conjugation!("riconoscere", 21, "~scere";
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
    // Sciogliere: 21 words (sciogliere, cogliere, scegliere, togliere, accogliere, raccogliere).
    conjugation!("sciogliere", 21, "~g~iere";
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
    // Imporsi: 20 words (porsi, disporsi, esporsi, imporsi, bersi, opporsi).
    conjugation!("imporsi", 20, "~rsi";
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
    // Farsi: 20 words (farsi, darsi, rifarsi, ridarsi, confarsi, putrefarsi).
    conjugation!("farsi", 20, "~arsi";
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
    // Esaurire: 19 words (premunire, esaurire, esordire, impedire, munire, obbedire).
    conjugation!("esaurire", 19, "~ire";
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
    // Fondere: 18 words (fondere, confondere, circonfondere, diffondere, radiodiffondere, telediffondere).
    conjugation!("fondere", 18, "~ondere";
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
    // Dire: 18 words (dire, benedire, ridire, interdire, predire, maledire).
    conjugation!("dire", 18, "~ire";
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
    // Assistere: 17 words (esistere, assistere, insistere, resistere, cernere, consistere).
    conjugation!("assistere", 17, "~ere";
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
    // Ritrarre: 16 words (trarre, attrarre, distrarre, detrarre, estrarre, contrarre).
    conjugation!("ritrarre", 16, "~rre";
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
    // Mettersi: 15 words (mettersi, dimettersi, rimettersi, permettersi, trasmettersi, sottomettersi).
    conjugation!("mettersi", 15, "~ettersi";
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
    // Reggersi: 15 words (reggersi, proteggersi, leggersi, distruggersi, crocifiggersi, correggersi).
    conjugation!("reggersi", 15, "~ggersi";
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
    // Imprimere: 14 words (opprimere, esprimere, deprimere, imprimere, comprimere, decomprimere).
    conjugation!("imprimere", 14, "~imere";
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
    // Tacere: 13 words (iacere, piacere, tacere, giacere, dispiacere, rigiacere).
    conjugation!("tacere", 13, "~ere";
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
    // Connettere: 13 words (flettere, annettere, riflettere, connettere, disconnettere, sconnettere).
    conjugation!("connettere", 13, "~ttere";
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
    // Spegnere: 13 words (spegnere, dispegnere, rispegnere, accignere, strignere, impignere).
    conjugation!("spegnere", 13, "~g~ere";
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
    // Ricadere: 12 words (cadere, avere, parere, accadere, decadere, discadere).
    conjugation!("ricadere", 12, "~ere";
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
    // Vendersi: 12 words (credersi, godersi, sedersi, vendersi, ricredersi, assistersi).
    conjugation!("vendersi", 12, "~ersi";
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
        imperative: "~iti", referred, "~iamoci", "~etevi", "~ano";
        negative_imperative: "~erti|~ere", referred, "~iamoci|~iamo", "~etevi|~ete", referred;
        infinitive: "~ersi";
        gerund: "~endosi";
        participle: "~entesi", "~utosi";
    ),
    // Ridursi: 12 words (ridursi, condursi, dedursi, indursi, introdursi, riprodursi).
    conjugation!("ridursi", 12, "~ursi";
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
    // Presumere: 11 words (consumere, presumere, assumere, sumere, desumere, riassumere).
    conjugation!("presumere", 11, "~mere";
        present_first: "~mo", "~miamo";
        present_second: "~mi", "~mete";
        present_third: "~me", "~mono";
        imperfect_first: "~mevo", "~mevamo";
        imperfect_second: "~mevi", "~mevate";
        imperfect_third: "~meva", "~mevano";
        past_historic_first: "~nsi", "~memmo";
        past_historic_second: "~mesti", "~meste";
        past_historic_third: "~nse", "~nsero";
        future_first: "~merò", "~meremo";
        future_second: "~merai", "~merete";
        future_third: "~merà", "~meranno";
        conditional_first: "~merei", "~meremmo";
        conditional_second: "~meresti", "~mereste";
        conditional_third: "~merebbe", "~merebbero";
        present_subjunctive_first: "~ma", "~miamo";
        present_subjunctive_second: "~ma", "~miate";
        present_subjunctive_third: "~ma", "~mano";
        imperfect_subjunctive_first: "~messi", "~messimo";
        imperfect_subjunctive_second: "~messi", "~meste";
        imperfect_subjunctive_third: "~messe", "~messero";
        imperative: "~mi", referred, "~miamo", "~mete", "~mano";
        negative_imperative: "~mere", referred, "~miamo", "~mete", referred;
        infinitive: "~mere";
        gerund: "~mendo";
        participle: "~mente", "~nto";
    ),
    // Trattenersi: 11 words (tenersi, ritenersi, trattenersi, sostenersi, astenersi, rimanersi).
    conjugation!("trattenersi", 11, "~~nersi";
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
    // Aprire: 10 words (scoprire, aprire, coprire, discoprire, riaprire, ricoprire).
    conjugation!("aprire", 10, "~~ire";
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
    // Prefiggere: 10 words (affiggere, crocefiggere, crocifiggere, crucifiggere, defiggere, infiggere).
    conjugation!("prefiggere", 10, "~ggere";
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
    // Contrarsi: 10 words (distrarsi, sottrarsi, trarsi, protrarsi, contrarsi, astrarsi).
    conjugation!("contrarsi", 10, "~rsi";
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
        imperative: "~iti", referred, "~iamoci", "~etevi", "~ggano";
        negative_imperative: "~rti|~rre", referred, "~iamoci|~iamo", "~etevi|~ete", referred;
        infinitive: "~rsi";
        gerund: "~endosi";
        participle: "~entesi", "~ttosi";
    ),
    // Vedere: 9 words (rivedere, vedere, prevedere, disvedere, intravedere, intravvedere).
    conjugation!("vedere", 9, "~edere";
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
        participle: "~edente|~eggente", "~isto";
    ),
    // Abbonare: 9 words (abbonare, sonare, rotare, risonare, risolare, scorare).
    conjugation!("abbonare", 9, "~~are";
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
    // Vincersi: 9 words (convincersi, vincersi, contorcersi, autoconvincersi, torcersi, distorcersi).
    conjugation!("vincersi", 9, "~cersi";
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
    // Sviarsi: 9 words (riavviarsi, disviarsi, avviarsi, inviarsi, striarsi, obliarsi).
    conjugation!("sviarsi", 9, "~arsi";
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
    // Rompere: 8 words (rompere, interrompere, corrompere, dirompere, disrompere, prorompere).
    conjugation!("rompere", 8, "~ompere";
        present_first: "~ompo", "~ompiamo";
        present_second: "~ompi", "~ompete";
        present_third: "~ompe", "~ompono";
        imperfect_first: "~ompevo", "~ompevamo";
        imperfect_second: "~ompevi", "~ompevate";
        imperfect_third: "~ompeva", "~ompevano";
        past_historic_first: "~uppi", "~ompemmo";
        past_historic_second: "~ompesti", "~ompeste";
        past_historic_third: "~uppe", "~uppero";
        future_first: "~omperò", "~omperemo";
        future_second: "~omperai", "~omperete";
        future_third: "~omperà", "~omperanno";
        conditional_first: "~omperei", "~omperemmo";
        conditional_second: "~omperesti", "~ompereste";
        conditional_third: "~omperebbe", "~omperebbero";
        present_subjunctive_first: "~ompa", "~ompiamo";
        present_subjunctive_second: "~ompa", "~ompiate";
        present_subjunctive_third: "~ompa", "~ompano";
        imperfect_subjunctive_first: "~ompessi", "~ompessimo";
        imperfect_subjunctive_second: "~ompessi", "~ompeste";
        imperfect_subjunctive_third: "~ompesse", "~ompessero";
        imperative: "~ompi", referred, "~ompiamo", "~ompete", "~ompano";
        negative_imperative: "~ompere", referred, "~ompiamo", "~ompete", referred;
        infinitive: "~ompere";
        gerund: "~ompendo";
        participle: "~ompente", "~otto";
    ),
    // Muovere: 8 words (muovere, rimuovere, commuovere, promuovere, sommuovere, smuovere).
    conjugation!("muovere", 8, "~u~vere";
        present_first: "~u~vo", "~u~viamo";
        present_second: "~u~vi", "~u~vete";
        present_third: "~u~ve", "~u~vono";
        imperfect_first: "~u~vevo", "~u~vevamo";
        imperfect_second: "~u~vevi", "~u~vevate";
        imperfect_third: "~u~veva", "~u~vevano";
        past_historic_first: "~~ssi", "~u~vemmo";
        past_historic_second: "~u~vesti", "~u~veste";
        past_historic_third: "~~sse", "~~ssero";
        future_first: "~u~verò", "~u~veremo";
        future_second: "~u~verai", "~u~verete";
        future_third: "~u~verà", "~u~veranno";
        conditional_first: "~u~verei", "~u~veremmo";
        conditional_second: "~u~veresti", "~u~vereste";
        conditional_third: "~u~verebbe", "~u~verebbero";
        present_subjunctive_first: "~u~va", "~u~viamo";
        present_subjunctive_second: "~u~va", "~u~viate";
        present_subjunctive_third: "~u~va", "~u~vano";
        imperfect_subjunctive_first: "~u~vessi", "~u~vessimo";
        imperfect_subjunctive_second: "~u~vessi", "~u~veste";
        imperfect_subjunctive_third: "~u~vesse", "~u~vessero";
        imperative: "~u~vi", referred, "~u~viamo", "~u~vete", "~u~vano";
        negative_imperative: "~u~vere", referred, "~u~viamo", "~u~vete", referred;
        infinitive: "~u~vere";
        gerund: "~u~vendo";
        participle: "~u~vente", "~~sso";
    ),
    // Stringere: 8 words (stringere, costringere, astringere, constringere, distringere, restringere).
    conjugation!("stringere", 8, "~ingere";
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
    // Decuocere: 8 words (cuocere, concuocere, decuocere, incuocere, ricuocere, stracuocere).
    conjugation!("decuocere", 8, "~u~cere";
        present_first: "~u~cio", "~u~ciamo";
        present_second: "~u~ci", "~u~cete";
        present_third: "~u~ce", "~u~ciono";
        imperfect_first: "~u~cevo", "~u~cevamo";
        imperfect_second: "~u~cevi", "~u~cevate";
        imperfect_third: "~u~ceva", "~u~cevano";
        past_historic_first: "~~ssi", "~u~cemmo";
        past_historic_second: "~u~cesti", "~u~ceste";
        past_historic_third: "~~sse", "~~ssero";
        future_first: "~u~cerò", "~u~ceremo";
        future_second: "~u~cerai", "~u~cerete";
        future_third: "~u~cerà", "~u~ceranno";
        conditional_first: "~u~cerei", "~u~ceremmo";
        conditional_second: "~u~ceresti", "~u~cereste";
        conditional_third: "~u~cerebbe", "~u~cerebbero";
        present_subjunctive_first: "~u~cia", "~u~ciamo";
        present_subjunctive_second: "~u~cia", "~u~ciate";
        present_subjunctive_third: "~u~cia", "~u~ciano";
        imperfect_subjunctive_first: "~u~cessi", "~u~cessimo";
        imperfect_subjunctive_second: "~u~cessi", "~u~ceste";
        imperfect_subjunctive_third: "~u~cesse", "~u~cessero";
        imperative: "~u~ci", referred, "~u~ciamo", "~u~cete", "~u~ciano";
        negative_imperative: "~u~cere", referred, "~u~ciamo", "~u~cete", referred;
        infinitive: "~u~cere";
        gerund: "~u~cendo";
        participle: "~u~cente", "~~tto";
    ),
    // Corrispondere: 8 words (corrispondere, rispondere, nascondere, respondere, rinascondere, disascondere).
    conjugation!("corrispondere", 8, "~ndere";
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
    // Flettersi: 8 words (connettersi, riflettersi, disconnettersi, interconnettersi, annettersi, genuflettersi).
    conjugation!("flettersi", 8, "~ttersi";
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
    // Noiare: 8 words (sdraiare, spaiare, scuoiare, scoiare, noiare, sghiaiare).
    conjugation!("noiare", 8, "~are";
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
    // Valere: 7 words (valere, contravvalere, disvalere, equivalere, prevalere, invalere).
    conjugation!("valere", 7, "~lere";
        present_first: "~lgo", "~liamo";
        present_second: "~li", "~lete";
        present_third: "~le", "~lgono";
        imperfect_first: "~levo", "~levamo";
        imperfect_second: "~levi", "~levate";
        imperfect_third: "~leva", "~levano";
        past_historic_first: "~lsi", "~lemmo";
        past_historic_second: "~lesti", "~leste";
        past_historic_third: "~lse", "~lsero";
        future_first: "~rrò", "~rremo";
        future_second: "~rrai", "~rrete";
        future_third: "~rrà", "~rranno";
        conditional_first: "~rrei", "~rremmo";
        conditional_second: "~rresti", "~rreste";
        conditional_third: "~rrebbe", "~rrebbero";
        present_subjunctive_first: "~lga", "~liamo";
        present_subjunctive_second: "~lga", "~liate";
        present_subjunctive_third: "~lga", "~lgano";
        imperfect_subjunctive_first: "~lessi", "~lessimo";
        imperfect_subjunctive_second: "~lessi", "~leste";
        imperfect_subjunctive_third: "~lesse", "~lessero";
        imperative: "~li", referred, "~liamo", "~lete", "~lgano";
        negative_imperative: "~lere", referred, "~liamo", "~lete", referred;
        infinitive: "~lere";
        gerund: "~lendo";
        participle: "~lente", "~lso";
    ),
    // Apparire: 7 words (apparire, comparire, scomparire, riapparire, disparire, ricomparire).
    conjugation!("apparire", 7, "~rire";
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
    // Estinguere: 7 words (distinguere, contraddistinguere, suddistinguere, ridistinguere, estinguere, stinguere).
    conjugation!("estinguere", 7, "~guere";
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
    // Risolvere: 7 words (risolvere, evolvere, resolvere, dissolvere, assolvere, devolvere).
    conjugation!("risolvere", 7, "~vere";
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
    // Escutere: 7 words (discutere, escutere, incutere, ridiscutere, riscotere, percotere).
    conjugation!("escutere", 7, "~tere";
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
    // Espellere: 7 words (espellere, depellere, propellere, repellere, riespellere, impellere).
    conjugation!("espellere", 7, "~e~lere";
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
    // Stare: 7 words (dare, stare, ridare, sottostare, ostare, ristare).
    conjugation!("stare", 7, "~are";
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
    // Sciogliersi: 7 words (togliersi, scegliersi, accogliersi, disciogliersi, sciogliersi, raccogliersi).
    conjugation!("sciogliersi", 7, "~g~iersi";
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
    // Rivolere: 6 words (volere, disvolere, stravolere, svolere, rivolere, solere).
    conjugation!("rivolere", 6, "~~lere";
        present_first: "~~glio", "~~gliamo";
        present_second: "~u~i", "~~lete";
        present_third: "~u~le", "~~gliono";
        imperfect_first: "~~levo", "~~levamo";
        imperfect_second: "~~levi", "~~levate";
        imperfect_third: "~~leva", "~~levano";
        past_historic_first: "~~lli", "~~lemmo";
        past_historic_second: "~~lesti", "~~leste";
        past_historic_third: "~~lle", "~~llero";
        future_first: "~~rrò", "~~rremo";
        future_second: "~~rrai", "~~rrete";
        future_third: "~~rrà", "~~rranno";
        conditional_first: "~~rrei", "~~rremmo";
        conditional_second: "~~rresti", "~~rreste";
        conditional_third: "~~rrebbe", "~~rrebbero";
        present_subjunctive_first: "~~glia", "~~gliamo";
        present_subjunctive_second: "~~glia", "~~gliate";
        present_subjunctive_third: "~~glia", "~~gliano";
        imperfect_subjunctive_first: "~~lessi", "~~lessimo";
        imperfect_subjunctive_second: "~~lessi", "~~leste";
        imperfect_subjunctive_third: "~~lesse", "~~lessero";
        imperative: "~~gli", referred, "~~gliamo", "~~gliate", "~~gliano";
        negative_imperative: "~~lere", referred, "~~gliamo", "~~gliate", referred;
        infinitive: "~~lere";
        gerund: "~~lendo";
        participle: "~~lente", "~~luto";
    ),
    // Ducere: 6 words (ducere, conducere, educere, reducere, riconducere, riducere).
    conjugation!("ducere", 6, "~ucere";
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
    // Risalire: 6 words (salire, assalire, riassalire, risalire, vienire, intravienire).
    conjugation!("risalire", 6, "~ire";
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
    // Fulgere: 6 words (convergere, mingere, rifulgere, circonfulgere, refulgere, fulgere).
    conjugation!("fulgere", 6, "~gere";
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
        imperative: "~gi", "~ga", "~giamo", "~gete", "~gano";
        negative_imperative: "~gere", "~ga", "~giamo", "~gete", "~gano";
        infinitive: "~gere";
        gerund: "~gendo";
        participle: "~gente", "~to";
    ),
    // Compire: 6 words (compire, ricompire, adempire, inorgoglire, orgoglire, incanaglire).
    conjugation!("compire", 6, "~ire";
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
    // Consentire: 6 words (consentire, acconsentire, assentire, dissentire, riconsentire, sconsentire).
    conjugation!("consentire", 6, "~~ire";
        present_first: "~~o", "~~iamo";
        present_second: "~~i", "~~ite";
        present_third: "~~e", "~~ono";
        imperfect_first: "~~ivo", "~~ivamo";
        imperfect_second: "~~ivi", "~~ivate";
        imperfect_third: "~~iva", "~~ivano";
        past_historic_first: "~~ii", "~~immo";
        past_historic_second: "~~isti", "~~iste";
        past_historic_third: "~~ì", "~~irono";
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
        participle: "~zien~e", "~~ito";
    ),
    // Scriversi: 6 words (iscriversi, viversi, scriversi, descriversi, prescriversi, ascriversi).
    conjugation!("scriversi", 6, "~versi";
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
    // Dirsi: 6 words (addirsi, dirsi, contraddirsi, maledirsi, autocontraddirsi, disdirsi).
    conjugation!("dirsi", 6, "~~rsi";
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
    // Coprirsi: 6 words (aprirsi, offrirsi, coprirsi, scoprirsi, ricoprirsi, riaprirsi).
    conjugation!("coprirsi", 6, "~~irsi";
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
    // Esaurirsi: 6 words (impedirsi, munirsi, esaurirsi, inorgoglirsi, premunirsi, incanaglirsi).
    conjugation!("esaurirsi", 6, "~irsi";
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
    // Rivalersi: 6 words (volersi, avvalersi, valersi, dolersi, rivalersi, condolersi).
    conjugation!("rivalersi", 6, "~lersi";
        present_first: "~lgo", "~liamo";
        present_second: "~li", "~lete";
        present_third: "~le", "~lgono";
        imperfect_first: "~levo", "~levamo";
        imperfect_second: "~levi", "~levate";
        imperfect_third: "~leva", "~levano";
        past_historic_first: "~lsi", "~lemmo";
        past_historic_second: "~lesti", "~leste";
        past_historic_third: "~lse", "~lsero";
        future_first: "~rrò", "~rremo";
        future_second: "~rrai", "~rrete";
        future_third: "~rrà", "~rranno";
        conditional_first: "~rrei", "~rremmo";
        conditional_second: "~rresti", "~rreste";
        conditional_third: "~rrebbe", "~rrebbero";
        present_subjunctive_first: "~lga", "~liamo";
        present_subjunctive_second: "~lga", "~liate";
        present_subjunctive_third: "~lga", "~lgano";
        imperfect_subjunctive_first: "~lessi", "~lessimo";
        imperfect_subjunctive_second: "~lessi", "~leste";
        imperfect_subjunctive_third: "~lesse", "~lessero";
        imperative: "~liti", referred, "~liamoci", "~letevi", "~lgano";
        negative_imperative: "~lerti", referred, "~liamoci", "~letevi", referred;
        infinitive: "~lersi";
        gerund: "~lendosi";
        participle: "~lentesi", "~lsosi";
    ),
    // Ire: 5 words (bere, ire, ribere, strabere, trabere).
    conjugation!("ire", 5, "~re";
        present_first: "~vo", "~viamo";
        present_second: "~vi", "~vete";
        present_third: "~ve", "~vono";
        imperfect_first: "~vevo", "~vevamo";
        imperfect_second: "~vevi", "~vevate";
        imperfect_third: "~veva", "~vevano";
        past_historic_first: "~vvi|~vetti", "~vemmo";
        past_historic_second: "~vesti", "~veste";
        past_historic_third: "~vve|~vette", "~vvero|~vettero";
        future_first: "~rrò", "~rremo";
        future_second: "~rrai", "~rrete";
        future_third: "~rrà", "~rranno";
        conditional_first: "~rrei", "~rremmo";
        conditional_second: "~rresti", "~rreste";
        conditional_third: "~rrebbe", "~rrebbero";
        present_subjunctive_first: "~va", "~viamo";
        present_subjunctive_second: "~va", "~viate";
        present_subjunctive_third: "~va", "~vano";
        imperfect_subjunctive_first: "~vessi", "~vessimo";
        imperfect_subjunctive_second: "~vessi", "~veste";
        imperfect_subjunctive_third: "~vesse", "~vessero";
        imperative: "~vi", referred, "~viamo", "~vete", "~vano";
        negative_imperative: "~re", referred, "~viamo", "~vete", referred;
        infinitive: "~re";
        gerund: "~vendo";
        participle: "~vente", "~vuto";
    ),
    // Richiedere: 5 words (chiedere, richiedere, dischiedere, inchiedere, rinchiedere).
    conjugation!("richiedere", 5, "~dere";
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
    // Riscuotere: 5 words (percuotere, scuotere, riscuotere, escuotere, ripercuotere).
    conjugation!("riscuotere", 5, "~u~tere";
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
    // Godere: 5 words (dovere, godere, rigodere, stragodere, ridovere).
    conjugation!("godere", 5, "~ere";
        present_first: "~o", "~iamo";
        present_second: "~i", "~ete";
        present_third: "~e", "~ono";
        imperfect_first: "~evo", "~evamo";
        imperfect_second: "~evi", "~evate";
        imperfect_third: "~eva", "~evano";
        past_historic_first: "~etti|~ei", "~emmo";
        past_historic_second: "~esti", "~este";
        past_historic_third: "~ette|~é", "~ettero|~erono";
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
    // Dirigere: 5 words (prediligere, dirigere, diligere, erigere, negligere).
    conjugation!("dirigere", 5, "~igere";
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
    // Sedere: 5 words (sedere, ossedere, presedere, soprassedere, risedere).
    conjugation!("sedere", 5, "~~ere";
        present_first: "~i~o", "~~iamo";
        present_second: "~i~i", "~~ete";
        present_third: "~i~e", "~i~ono";
        imperfect_first: "~~evo", "~~evamo";
        imperfect_second: "~~evi", "~~evate";
        imperfect_third: "~~eva", "~~evano";
        past_historic_first: "~~etti|~~ei", "~~emmo";
        past_historic_second: "~~esti", "~~este";
        past_historic_third: "~~ette|~~é", "~~ettero|~~erono";
        future_first: "~i~erò|~~erò", "~i~eremo|~~eremo";
        future_second: "~i~erai|~~erai", "~i~erete|~~erete";
        future_third: "~i~erà|~~erà", "~i~eranno|~~eranno";
        conditional_first: "~i~erei|~~erei", "~i~eremmo|~~eremmo";
        conditional_second: "~i~eresti|~~eresti", "~i~ereste|~~ereste";
        conditional_third: "~i~erebbe|~~erebbe", "~i~erebbero|~~erebbero";
        present_subjunctive_first: "~i~a", "~~iamo";
        present_subjunctive_second: "~i~a", "~~iate";
        present_subjunctive_third: "~i~a", "~i~ano";
        imperfect_subjunctive_first: "~~essi", "~~essimo";
        imperfect_subjunctive_second: "~~essi", "~~este";
        imperfect_subjunctive_third: "~~esse", "~~essero";
        imperative: "~i~i", referred, "~~iamo", "~~ete", "~i~ano";
        negative_imperative: "~~ere", referred, "~~iamo", "~~ete", referred;
        infinitive: "~~ere";
        gerund: "~~endo";
        participle: "~~ente", "~~uto";
    ),
    // Retrocedere: 5 words (succedere, concedere, retrocedere, riconcedere, subconcedere).
    conjugation!("retrocedere", 5, "~dere";
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
    // Sviare: 5 words (sciare, spiare, sviare, piare, striare).
    conjugation!("sviare", 5, "~are";
        present_first: "~o", "~amo";
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
    // Rimuoversi: 5 words (commuoversi, muoversi, rimuoversi, smuoversi, autopromuoversi).
    conjugation!("rimuoversi", 5, "~u~versi";
        present_first: "~u~vo", "~u~viamo";
        present_second: "~u~vi", "~u~vete";
        present_third: "~u~ve", "~u~vono";
        imperfect_first: "~u~vevo", "~u~vevamo";
        imperfect_second: "~u~vevi", "~u~vevate";
        imperfect_third: "~u~veva", "~u~vevano";
        past_historic_first: "~~ssi", "~u~vemmo";
        past_historic_second: "~u~vesti", "~u~veste";
        past_historic_third: "~~sse", "~~ssero";
        future_first: "~u~verò", "~u~veremo";
        future_second: "~u~verai", "~u~verete";
        future_third: "~u~verà", "~u~veranno";
        conditional_first: "~u~verei", "~u~veremmo";
        conditional_second: "~u~veresti", "~u~vereste";
        conditional_third: "~u~verebbe", "~u~verebbero";
        present_subjunctive_first: "~u~va", "~u~viamo";
        present_subjunctive_second: "~u~va", "~u~viate";
        present_subjunctive_third: "~u~va", "~u~vano";
        imperfect_subjunctive_first: "~u~vessi", "~u~vessimo";
        imperfect_subjunctive_second: "~u~vessi", "~u~veste";
        imperfect_subjunctive_third: "~u~vesse", "~u~vessero";
        imperative: "~u~viti", referred, "~u~viamoci", "~u~vetevi", "~u~vano";
        negative_imperative: "~u~verti|~u~vere", referred, "~u~viamoci|~u~viamo", "~u~vetevi|~u~vete", referred;
        infinitive: "~u~versi";
        gerund: "~u~vendosi";
        participle: "~u~ventesi", "~~ssosi";
    ),
    // Fruire: 5 words (guaire, fluire, gioire, gire, fruire).
    conjugation!("fruire", 5, "~ire";
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
    // Provvedere: 5 words (provvedere, provedere, riprovvedere, sprovvedere, riprovedere).
    conjugation!("provvedere", 5, "~edere";
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
    // Travedere: 5 words (antivedere, stravedere, transvedere, travedere, trasvedere).
    conjugation!("travedere", 5, "~edere";
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
    // Sapere: 5 words (sapere, strasapere, soprassapere, presapere, risapere).
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
    // Diffondersi: 5 words (diffondersi, confondersi, fondersi, profondersi, effondersi).
    conjugation!("diffondersi", 5, "~ondersi";
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
        imperative: "~onditi", referred, "~ondiamoci", "~ondetevi", "~ondano";
        negative_imperative: "~onderti|~ondere", referred, "~ondiamoci|~ondiamo", "~ondetevi|~ondete", referred;
        infinitive: "~ondersi";
        gerund: "~ondendosi";
        participle: "~ondentesi", "~usosi";
    ),
    // Vedersi: 5 words (vedersi, rivedersi, ravvedersi, avvedersi, intravedersi).
    conjugation!("vedersi", 5, "~edersi";
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
        imperative: "~editi", referred, "~ediamoci", "~edetevi", "~edano";
        negative_imperative: "~ederti|~edere", referred, "~ediamoci|~ediamo", "~edetevi|~edete", referred;
        infinitive: "~edersi";
        gerund: "~edendosi";
        participle: "~edentesi|~eggentesi", "~istosi";
    ),
    // Evolversi: 5 words (evolversi, dissolversi, risolversi, autoassolversi, assolversi).
    conjugation!("evolversi", 5, "~versi";
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
        imperative: "~viti", referred, "~viamoci", "~vetevi", "~vano";
        negative_imperative: "~verti|~vere", referred, "~viamoci|~viamo", "~vetevi|~vete", referred;
        infinitive: "~versi";
        gerund: "~vendosi";
        participle: "~ventesi", "~tosi";
    ),
    // Traggere: 5 words (sottraggere, astraggere, distraggere, ritraggere, traggere).
    conjugation!("traggere", 5, "~ggere";
        present_first: "~ggo", "~ggiamo";
        present_second: "~ggi", "~ete";
        present_third: "~gge", "~ggono";
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
        present_subjunctive_first: "~gga", "~ggiamo";
        present_subjunctive_second: "~gga", "~ggiate";
        present_subjunctive_third: "~gga", "~ggano";
        imperfect_subjunctive_first: "~essi", "~essimo";
        imperfect_subjunctive_second: "~essi", "~este";
        imperfect_subjunctive_third: "~esse", "~essero";
        imperative: "~ggi", referred, "~ggiamo", "~ete", "~ggano";
        negative_imperative: "~ggere", referred, "~ggiamo", "~ete", referred;
        infinitive: "~ggere";
        gerund: "~endo";
        participle: "~ente", "~tto";
    ),
    // Inerire: 5 words (bruire, inerire, olire, reddire, ripire).
    conjugation!("inerire", 5, "~ire";
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
        imperative: "~isci", "~isca", "~iamo", "~ite", "~iscano";
        negative_imperative: "~ire", "~isca", "~iamo", "~ite", "~iscano";
        infinitive: "~ire";
        gerund: "~endo";
        participle: "~ente", "~ito";
    ),
    // Empire: 4 words (empire, riempire, saglire, boglire).
    conjugation!("empire", 4, "~ire";
        present_first: "~io", "~iamo";
        present_second: "~i", "~ite";
        present_third: "~ie", "~iono";
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
        present_subjunctive_first: "~ia", "~iamo";
        present_subjunctive_second: "~ia", "~iate";
        present_subjunctive_third: "~ia", "~iano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~i", referred, "~iamo", "~ite", "~iano";
        negative_imperative: "~ire", referred, "~iamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~iendo";
        participle: "~iente", "~ito";
    ),
    // Morire: 4 words (morire, premorire, rimorire, smorire).
    conjugation!("morire", 4, "~~rire";
        present_first: "~u~io", "~~riamo";
        present_second: "~u~ri", "~~rite";
        present_third: "~u~re", "~u~iono";
        imperfect_first: "~~rivo", "~~rivamo";
        imperfect_second: "~~rivi", "~~rivate";
        imperfect_third: "~~riva", "~~rivano";
        past_historic_first: "~~rii", "~~rimmo";
        past_historic_second: "~~risti", "~~riste";
        past_historic_third: "~~rì", "~~rirono";
        future_first: "~~rirò", "~~riremo";
        future_second: "~~rirai", "~~rirete";
        future_third: "~~rirà", "~~riranno";
        conditional_first: "~~rirei", "~~riremmo";
        conditional_second: "~~riresti", "~~rireste";
        conditional_third: "~~rirebbe", "~~rirebbero";
        present_subjunctive_first: "~u~ia", "~~riamo";
        present_subjunctive_second: "~u~ia", "~~riate";
        present_subjunctive_third: "~u~ia", "~u~iano";
        imperfect_subjunctive_first: "~~rissi", "~~rissimo";
        imperfect_subjunctive_second: "~~rissi", "~~riste";
        imperfect_subjunctive_third: "~~risse", "~~rissero";
        imperative: "~u~ri", referred, "~~riamo", "~~rite", "~u~iano";
        negative_imperative: "~~rire", referred, "~~riamo", "~~rite", referred;
        infinitive: "~~rire";
        gerund: "~~rendo";
        participle: "~~rente", "~~rto";
    ),
    // Vivere: 4 words (vivere, rivivere, convivere, sopravvivere).
    conjugation!("vivere", 4, "~vere";
        present_first: "~vo", "~viamo";
        present_second: "~vi", "~vete";
        present_third: "~ve", "~vono";
        imperfect_first: "~vevo", "~vevamo";
        imperfect_second: "~vevi", "~vevate";
        imperfect_third: "~veva", "~vevano";
        past_historic_first: "~ssi", "~vemmo";
        past_historic_second: "~vesti", "~veste";
        past_historic_third: "~sse", "~ssero";
        future_first: "~vrò", "~vremo";
        future_second: "~vrai", "~vrete";
        future_third: "~vrà", "~vranno";
        conditional_first: "~vrei", "~vremmo";
        conditional_second: "~vresti", "~vreste";
        conditional_third: "~vrebbe", "~vrebbero";
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
        participle: "~vente", "~ssuto";
    ),
    // Ricucire: 4 words (cucire, ricucire, scucire, discucire).
    conjugation!("ricucire", 4, "~ire";
        present_first: "~io", "~iamo";
        present_second: "~i", "~ite";
        present_third: "~e", "~iono";
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
        present_subjunctive_first: "~ia", "~iamo";
        present_subjunctive_second: "~ia", "~iate";
        present_subjunctive_third: "~ia", "~iano";
        imperfect_subjunctive_first: "~issi", "~issimo";
        imperfect_subjunctive_second: "~issi", "~iste";
        imperfect_subjunctive_third: "~isse", "~issero";
        imperative: "~i", referred, "~iamo", "~ite", "~iano";
        negative_imperative: "~ire", referred, "~iamo", "~ite", referred;
        infinitive: "~ire";
        gerund: "~endo";
        participle: "~ente", "~ito";
    ),
    // Disfare: 4 words (soddisfare, disfare, sodisfare, ridisfare).
    conjugation!("disfare", 4, "~are";
        present_first: "~ò", "~acciamo";
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
        imperative: "~ai", referred, "~acciamo", "~ate", "~acciano";
        negative_imperative: "~are", referred, "~acciamo", "~ate", referred;
        infinitive: "~are";
        gerund: "~acendo";
        participle: "~acente", "~atto";
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
    // Competere: 4 words (vertere, competere, vergere, ricompetere).
    conjugation!("competere", 4, "~ere";
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
        imperative: "~i", "~a", "~iamo", "~ete", "~ano";
        negative_imperative: "~ere", "~a", "~iamo", "~ete", "~ano";
        infinitive: "~ere";
        gerund: "~endo";
        participle: "~ente", "~uto";
    ),
    // Torre: 4 words (torre, accorre, sciorre, disverre).
    conjugation!("torre", 4, "~rre";
        present_first: "~lgo", "~gliamo";
        present_second: "~gli", "~gliete";
        present_third: "~glie", "~lgono";
        imperfect_first: "~glievo", "~glievamo";
        imperfect_second: "~glievi", "~glievate";
        imperfect_third: "~glieva", "~glievano";
        past_historic_first: "~lsi", "~gliemmo";
        past_historic_second: "~gliesti", "~glieste";
        past_historic_third: "~lse", "~lsero";
        future_first: "~rrò", "~rremo";
        future_second: "~rrai", "~rrete";
        future_third: "~rrà", "~rranno";
        conditional_first: "~rrei", "~rremmo";
        conditional_second: "~rresti", "~rreste";
        conditional_third: "~rrebbe", "~rrebbero";
        present_subjunctive_first: "~lga", "~gliamo";
        present_subjunctive_second: "~lga", "~gliate";
        present_subjunctive_third: "~lga", "~lgano";
        imperfect_subjunctive_first: "~gliessi", "~gliessimo";
        imperfect_subjunctive_second: "~gliessi", "~glieste";
        imperfect_subjunctive_third: "~gliesse", "~gliessero";
        imperative: "~gli", referred, "~gliamo", "~gliete", "~lgano";
        negative_imperative: "~rre", referred, "~gliamo", "~gliete", referred;
        infinitive: "~rre";
        gerund: "~gliendo";
        participle: "~gliente", "~lto";
    ),
    // Conoscersi: 4 words (conoscersi, riconoscersi, accrescersi, rincrescersi).
    conjugation!("conoscersi", 4, "~scersi";
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
    // Immergersi: 4 words (immergersi, spargersi, reimmergersi, cospargersi).
    conjugation!("immergersi", 4, "~gersi";
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
    // Scindere: 4 words (scindere, prescindere, rescindere, piroscindere).
    conjugation!("scindere", 4, "~ndere";
        present_first: "~ndo", "~ndiamo";
        present_second: "~ndi", "~ndete";
        present_third: "~nde", "~ndono";
        imperfect_first: "~ndevo", "~ndevamo";
        imperfect_second: "~ndevi", "~ndevate";
        imperfect_third: "~ndeva", "~ndevano";
        past_historic_first: "~ssi", "~ndemmo";
        past_historic_second: "~ndesti", "~ndeste";
        past_historic_third: "~sse", "~ssero";
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
        participle: "~ndente", "~sso";
    ),
    // Spandere: 4 words (spandere, espandere, riespandere, rispandere).
    conjugation!("spandere", 4, "~dere";
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
    // Nascondersi: 4 words (nascondersi, rispondersi, ascondersi, corrispondersi).
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
    // Piacersi: 4 words (piacersi, dispiacersi, compiacersi, spiacersi).
    conjugation!("piacersi", 4, "~ersi";
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
    // Stringersi: 4 words (costringersi, stringersi, restringersi, ristringersi).
    conjugation!("stringersi", 4, "~~ngersi";
        present_first: "~~ngo", "~~ngiamo";
        present_second: "~~ngi", "~~ngete";
        present_third: "~~nge", "~~ngono";
        imperfect_first: "~~ngevo", "~~ngevamo";
        imperfect_second: "~~ngevi", "~~ngevate";
        imperfect_third: "~~ngeva", "~~ngevano";
        past_historic_first: "~~nsi", "~~ngemmo";
        past_historic_second: "~~ngesti", "~~ngeste";
        past_historic_third: "~~nse", "~~nsero";
        future_first: "~~ngerò", "~~ngeremo";
        future_second: "~~ngerai", "~~ngerete";
        future_third: "~~ngerà", "~~ngeranno";
        conditional_first: "~~ngerei", "~~ngeremmo";
        conditional_second: "~~ngeresti", "~~ngereste";
        conditional_third: "~~ngerebbe", "~~ngerebbero";
        present_subjunctive_first: "~~nga", "~~ngiamo";
        present_subjunctive_second: "~~nga", "~~ngiate";
        present_subjunctive_third: "~~nga", "~~ngano";
        imperfect_subjunctive_first: "~~ngessi", "~~ngessimo";
        imperfect_subjunctive_second: "~~ngessi", "~~ngeste";
        imperfect_subjunctive_third: "~~ngesse", "~~ngessero";
        imperative: "~~ngiti", referred, "~~ngiamoci", "~~ngetevi", "~~ngano";
        negative_imperative: "~~ngerti|~~ngere", referred, "~~ngiamoci|~~ngiamo", "~~ngetevi|~~ngete", referred;
        infinitive: "~~ngersi";
        gerund: "~~ngendosi";
        participle: "~~ngentesi", "~ettos~";
    ),
    // Imprimersi: 4 words (esprimersi, deprimersi, reprimersi, imprimersi).
    conjugation!("imprimersi", 4, "~imersi";
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
        imperative: "~imiti", referred, "~imiamoci", "~imetevi", "~imano";
        negative_imperative: "~imerti|~imere", referred, "~imiamoci|~imiamo", "~imetevi|~imete", referred;
        infinitive: "~imersi";
        gerund: "~imendosi";
        participle: "~imentesi", "~essosi";
    ),
    // Riscuotersi: 4 words (percuotersi, ripercuotersi, scuotersi, riscuotersi).
    conjugation!("riscuotersi", 4, "~u~tersi";
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
    // Mescere: 4 words (pascere, mescere, rimescere, ripascere).
    conjugation!("mescere", 4, "~ere";
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
    // Piovere: 3 words (piovere, ripiovere, spiovere).
    conjugation!("piovere", 3, "~ere";
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
    // Potere: 3 words (potere, prepotere, ripotere).
    conjugation!("potere", 3, "~otere";
        present_first: "~osso", "~ossiamo";
        present_second: "~uoi", "~otete";
        present_third: "~uò", "~ossono";
        imperfect_first: "~otevo", "~otevamo";
        imperfect_second: "~otevi", "~otevate";
        imperfect_third: "~oteva", "~otevano";
        past_historic_first: "~otei", "~otemmo";
        past_historic_second: "~otesti", "~oteste";
        past_historic_third: "~oté", "~oterono";
        future_first: "~otrò", "~otremo";
        future_second: "~otrai", "~otrete";
        future_third: "~otrà", "~otranno";
        conditional_first: "~otrei", "~otremmo";
        conditional_second: "~otresti", "~otreste";
        conditional_third: "~otrebbe", "~otrebbero";
        present_subjunctive_first: "~ossa", "~ossiamo";
        present_subjunctive_second: "~ossa", "~ossiate";
        present_subjunctive_third: "~ossa", "~ossano";
        imperfect_subjunctive_first: "~otessi", "~otessimo";
        imperfect_subjunctive_second: "~otessi", "~oteste";
        imperfect_subjunctive_third: "~otesse", "~otessero";
        imperative: "~a", referred, "~iamo", "~ate", "~ino";
        negative_imperative: "~are", referred, "~iamo", "~ate", referred;
        infinitive: "~otere";
        gerund: "~otendo";
        participle: "~otente", "~otuto";
    ),
    // Rinascere: 3 words (nascere, rinascere, prenascere).
    conjugation!("rinascere", 3, "~scere";
        present_first: "~sco", "~sciamo";
        present_second: "~sci", "~scete";
        present_third: "~sce", "~scono";
        imperfect_first: "~scevo", "~scevamo";
        imperfect_second: "~scevi", "~scevate";
        imperfect_third: "~sceva", "~scevano";
        past_historic_first: "~cqui", "~scemmo";
        past_historic_second: "~scesti", "~sceste";
        past_historic_third: "~cque", "~cquero";
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
        participle: "~scente", "~to";
    ),
    // Dolere: 3 words (dolere, ridolere, sdolere).
    conjugation!("dolere", 3, "~~lere";
        present_first: "~~lgo", "~~liamo";
        present_second: "~u~li", "~~lete";
        present_third: "~u~le", "~~lgono";
        imperfect_first: "~~levo", "~~levamo";
        imperfect_second: "~~levi", "~~levate";
        imperfect_third: "~~leva", "~~levano";
        past_historic_first: "~~lsi", "~~lemmo";
        past_historic_second: "~~lesti", "~~leste";
        past_historic_third: "~~lse", "~~lsero";
        future_first: "~~rrò", "~~rremo";
        future_second: "~~rrai", "~~rrete";
        future_third: "~~rrà", "~~rranno";
        conditional_first: "~~rrei", "~~rremmo";
        conditional_second: "~~rresti", "~~rreste";
        conditional_third: "~~rrebbe", "~~rrebbero";
        present_subjunctive_first: "~~lga", "~~liamo";
        present_subjunctive_second: "~~lga", "~~liate";
        present_subjunctive_third: "~~lga", "~~lgano";
        imperfect_subjunctive_first: "~~lessi", "~~lessimo";
        imperfect_subjunctive_second: "~~lessi", "~~leste";
        imperfect_subjunctive_third: "~~lesse", "~~lessero";
        imperative: "~u~li", referred, "~~liamo", "~~lete", "~~lgano";
        negative_imperative: "~~lere", referred, "~~liamo", "~~lete", referred;
        infinitive: "~~lere";
        gerund: "~~lendo";
        participle: "~~lente", "~~luto";
    ),
    // Soffrire: 3 words (offrire, soffrire, rioffrire).
    conjugation!("soffrire", 3, "~~ire";
        present_first: "~~o", "~~iamo";
        present_second: "~~i", "~~ite";
        present_third: "~~e", "~~ono";
        imperfect_first: "~~ivo", "~~ivamo";
        imperfect_second: "~~ivi", "~~ivate";
        imperfect_third: "~~iva", "~~ivano";
        past_historic_first: "~~ii", "~~immo";
        past_historic_second: "~~isti", "~~iste";
        past_historic_third: "~~ì", "~~irono";
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
        participle: "~e~ente", "~e~to";
    ),
    // Disseppellire: 3 words (seppellire, riseppellire, disseppellire).
    conjugation!("disseppellire", 3, "~pe~lire";
        present_first: "~pe~lisco", "~pe~liamo";
        present_second: "~pe~lisci", "~pe~lite";
        present_third: "~pe~lisce", "~pe~liscono";
        imperfect_first: "~pe~livo", "~pe~livamo";
        imperfect_second: "~pe~livi", "~pe~livate";
        imperfect_third: "~pe~liva", "~pe~livano";
        past_historic_first: "~pe~lii", "~pe~limmo";
        past_historic_second: "~pe~listi", "~pe~liste";
        past_historic_third: "~pe~lì", "~pe~lirono";
        future_first: "~pe~lirò", "~pe~liremo";
        future_second: "~pe~lirai", "~pe~lirete";
        future_third: "~pe~lirà", "~pe~liranno";
        conditional_first: "~pe~lirei", "~pe~liremmo";
        conditional_second: "~pe~liresti", "~pe~lireste";
        conditional_third: "~pe~lirebbe", "~pe~lirebbero";
        present_subjunctive_first: "~pe~lisca", "~pe~liamo";
        present_subjunctive_second: "~pe~lisca", "~pe~liate";
        present_subjunctive_third: "~pe~lisca", "~pe~liscano";
        imperfect_subjunctive_first: "~pe~lissi", "~pe~lissimo";
        imperfect_subjunctive_second: "~pe~lissi", "~pe~liste";
        imperfect_subjunctive_third: "~pe~lisse", "~pe~lissero";
        imperative: "~pe~lisci", referred, "~pe~liamo", "~pe~lite", "~pe~liscano";
        negative_imperative: "~pe~lire", referred, "~pe~liamo", "~pe~lite", referred;
        infinitive: "~pe~lire";
        gerund: "~pe~lendo";
        participle: "~pe~lente", "~o~to|~pe~lito";
    ),
    // Divellere: 3 words (svellere, divellere, disvellere).
    conjugation!("divellere", 3, "~lere";
        present_first: "~lo|~go", "~liamo";
        present_second: "~li", "~lete";
        present_third: "~le", "~lono|~gono";
        imperfect_first: "~levo", "~levamo";
        imperfect_second: "~levi", "~levate";
        imperfect_third: "~leva", "~levano";
        past_historic_first: "~si", "~lemmo";
        past_historic_second: "~lesti", "~leste";
        past_historic_third: "~se", "~sero";
        future_first: "~lerò", "~leremo";
        future_second: "~lerai", "~lerete";
        future_third: "~lerà", "~leranno";
        conditional_first: "~lerei", "~leremmo";
        conditional_second: "~leresti", "~lereste";
        conditional_third: "~lerebbe", "~lerebbero";
        present_subjunctive_first: "~la|~ga", "~liamo";
        present_subjunctive_second: "~la|~ga", "~liate";
        present_subjunctive_third: "~la|~ga", "~lano|~gano";
        imperfect_subjunctive_first: "~lessi", "~lessimo";
        imperfect_subjunctive_second: "~lessi", "~leste";
        imperfect_subjunctive_third: "~lesse", "~lessero";
        imperative: "~li", referred, "~liamo", "~lete", "~lano|~gano";
        negative_imperative: "~lere", referred, "~liamo", "~lete", referred;
        infinitive: "~lere";
        gerund: "~lendo";
        participle: "~lente", "~to";
    ),
    // Redigere: 3 words (redigere, esigere, transigere).
    conjugation!("redigere", 3, "~igere";
        present_first: "~igo", "~igiamo";
        present_second: "~igi", "~igete";
        present_third: "~ige", "~igono";
        imperfect_first: "~igevo", "~igevamo";
        imperfect_second: "~igevi", "~igevate";
        imperfect_third: "~igeva", "~igevano";
        past_historic_first: "~assi", "~igemmo";
        past_historic_second: "~igesti", "~igeste";
        past_historic_third: "~asse", "~assero";
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
        participle: "~igente", "~atto";
    ),
    // Possedere: 3 words (possedere, ripossedere, spossedere).
    conjugation!("possedere", 3, "~~dere";
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
    // Rompersi: 3 words (interrompersi, rompersi, corrompersi).
    conjugation!("rompersi", 3, "~ompersi";
        present_first: "~ompo", "~ompiamo";
        present_second: "~ompi", "~ompete";
        present_third: "~ompe", "~ompono";
        imperfect_first: "~ompevo", "~ompevamo";
        imperfect_second: "~ompevi", "~ompevate";
        imperfect_third: "~ompeva", "~ompevano";
        past_historic_first: "~uppi", "~ompemmo";
        past_historic_second: "~ompesti", "~ompeste";
        past_historic_third: "~uppe", "~uppero";
        future_first: "~omperò", "~omperemo";
        future_second: "~omperai", "~omperete";
        future_third: "~omperà", "~omperanno";
        conditional_first: "~omperei", "~omperemmo";
        conditional_second: "~omperesti", "~ompereste";
        conditional_third: "~omperebbe", "~omperebbero";
        present_subjunctive_first: "~ompa", "~ompiamo";
        present_subjunctive_second: "~ompa", "~ompiate";
        present_subjunctive_third: "~ompa", "~ompano";
        imperfect_subjunctive_first: "~ompessi", "~ompessimo";
        imperfect_subjunctive_second: "~ompessi", "~ompeste";
        imperfect_subjunctive_third: "~ompesse", "~ompessero";
        imperative: "~ompiti", referred, "~ompiamoci", "~ompetevi", "~ompano";
        negative_imperative: "~omperti|~ompere", referred, "~ompiamoci|~ompiamo", "~ompetevi|~ompete", referred;
        infinitive: "~ompersi";
        gerund: "~ompendosi";
        participle: "~ompentesi", "~ottosi";
    ),
    // Appercepire: 3 words (concepire, appercepire, riconcepire).
    conjugation!("appercepire", 3, "~e~ire";
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
    // Fendere: 3 words (fendere, rifendere, sfendere).
    conjugation!("fendere", 3, "~ndere";
        present_first: "~ndo", "~ndiamo";
        present_second: "~ndi", "~ndete";
        present_third: "~nde", "~ndono";
        imperfect_first: "~ndevo", "~ndevamo";
        imperfect_second: "~ndevi", "~ndevate";
        imperfect_third: "~ndeva", "~ndevano";
        past_historic_first: "~ndetti|~ndei", "~ndemmo";
        past_historic_second: "~ndesti", "~ndeste";
        past_historic_third: "~ndette|~ndé", "~ndettero|~nderono";
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
        participle: "~ndente", "~nduto|~sso";
    ),
    // Rifrangere: 3 words (diffrangere, refrangere, rifrangere).
    conjugation!("rifrangere", 3, "~ngere";
        present_first: "~ngo", "~ngiamo";
        present_second: "~ngi", "~ngete";
        present_third: "~nge", "~ngono";
        imperfect_first: "~ngevo", "~ngevamo";
        imperfect_second: "~ngevi", "~ngevate";
        imperfect_third: "~ngeva", "~ngevano";
        past_historic_first: "~nsi", "~ngemmo";
        past_historic_second: "~ngesti", "~ngeste";
        past_historic_third: "~nse", "~nsero";
        future_first: "~ngerò", "~ngeremo";
        future_second: "~ngerai", "~ngerete";
        future_third: "~ngerà", "~ngeranno";
        conditional_first: "~ngerei", "~ngeremmo";
        conditional_second: "~ngeresti", "~ngereste";
        conditional_third: "~ngerebbe", "~ngerebbero";
        present_subjunctive_first: "~nga", "~ngiamo";
        present_subjunctive_second: "~nga", "~ngiate";
        present_subjunctive_third: "~nga", "~ngano";
        imperfect_subjunctive_first: "~ngessi", "~ngessimo";
        imperfect_subjunctive_second: "~ngessi", "~ngeste";
        imperfect_subjunctive_third: "~ngesse", "~ngessero";
        imperative: "~ngi", referred, "~ngiamo", "~ngete", "~ngano";
        negative_imperative: "~ngere", referred, "~ngiamo", "~ngete", referred;
        infinitive: "~ngere";
        gerund: "~ngendo";
        participle: "~ngente", "~nto|~tto";
    ),
    // Estinguersi: 3 words (distinguersi, estinguersi, contraddistinguersi).
    conjugation!("estinguersi", 3, "~guersi";
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
        imperative: "~guiti", referred, "~guiamoci", "~guetevi", "~guano";
        negative_imperative: "~guerti|~guere", referred, "~guiamoci|~guiamo", "~guetevi|~guete", referred;
        infinitive: "~guersi";
        gerund: "~guendosi";
        participle: "~guentesi", "~tosi";
    ),
    // Disfarsi: 3 words (disfarsi, ridisfarsi, soddisfarsi).
    conjugation!("disfarsi", 3, "~arsi";
        present_first: "~ò", "~acciamo";
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
        negative_imperative: "~arti|~are", referred, "~acciamoci", "~atevi|~ate", referred;
        infinitive: "~arsi";
        gerund: "~acendosi";
        participle: "~acentesi", "~attosi";
    ),
    // Imbevere: 3 words (imbevere, bevere, soprabbevere).
    conjugation!("imbevere", 3, "~vere";
        present_first: "~vo", "~viamo";
        present_second: "~vi", "~vete";
        present_third: "~ve", "~vono";
        imperfect_first: "~vevo", "~vevamo";
        imperfect_second: "~vevi", "~vevate";
        imperfect_third: "~veva", "~vevano";
        past_historic_first: "~vvi|~vetti", "~vemmo";
        past_historic_second: "~vesti", "~veste";
        past_historic_third: "~vve|~vette", "~vvero|~vettero";
        future_first: "~rrò", "~rremo";
        future_second: "~rrai", "~rrete";
        future_third: "~rrà", "~rranno";
        conditional_first: "~rrei", "~rremmo";
        conditional_second: "~rresti", "~rreste";
        conditional_third: "~rrebbe", "~rrebbero";
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
        participle: "~vente", "~vuto";
    ),
    // Affocare: 3 words (infocare, affocare, rinfocare).
    conjugation!("affocare", 3, "~~are";
        present_first: "~u~o", "~~hiamo";
        present_second: "~u~hi", "~~ate";
        present_third: "~u~a", "~u~ano";
        imperfect_first: "~~avo", "~~avamo";
        imperfect_second: "~~avi", "~~avate";
        imperfect_third: "~~ava", "~~avano";
        past_historic_first: "~~ai", "~~ammo";
        past_historic_second: "~~asti", "~~aste";
        past_historic_third: "~~ò", "~~arono";
        future_first: "~~herò", "~~heremo";
        future_second: "~~herai", "~~herete";
        future_third: "~~herà", "~~heranno";
        conditional_first: "~~herei", "~~heremmo";
        conditional_second: "~~heresti", "~~hereste";
        conditional_third: "~~herebbe", "~~herebbero";
        present_subjunctive_first: "~u~hi", "~~hiamo";
        present_subjunctive_second: "~u~hi", "~~hiate";
        present_subjunctive_third: "~u~hi", "~u~hino";
        imperfect_subjunctive_first: "~~assi", "~~assimo";
        imperfect_subjunctive_second: "~~assi", "~~aste";
        imperfect_subjunctive_third: "~~asse", "~~assero";
        imperative: "~u~a", referred, "~~hiamo", "~~ate", "~u~hino";
        negative_imperative: "~~are", referred, "~~hiamo", "~~ate", referred;
        infinitive: "~~are";
        gerund: "~~ando";
        participle: "~~ante", "~~ato";
    ),
    // Convenirsi: 3 words (convenirsi, risovvenirsi, rinvenirsi).
    conjugation!("convenirsi", 3, "~~nirsi";
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
        imperative: "~i~niti", referred, "~~niamoci", "~~nitevi", "~~ngano";
        negative_imperative: "~~nirti|~~nire", referred, "~~niamoci|~~niamo", "~~nitevi|~~nite", referred;
        infinitive: "~~nirsi";
        gerund: "~~nendosi";
        participle: "~~nientesi", "~~nutosi";
    ),
    // Smucciare: 3 words (rilisciare, ristrisciare, smucciare).
    conjugation!("smucciare", 3, "~iare";
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
        imperative: "~ia", "~i", "~iamo", "~iate", "~ino";
        negative_imperative: "~iare", "~i", "~iamo", "~iate", "~ino";
        infinitive: "~iare";
        gerund: "~iando";
        participle: "~iante", "~iato";
    ),
];
