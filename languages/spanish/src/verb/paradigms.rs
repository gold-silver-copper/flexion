//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `present_first` (sg, pl)
/// `present_second` (sg, pl)
/// `present_third` (sg, pl)
/// `imperfect_first` (sg, pl)
/// `imperfect_second` (sg, pl)
/// `imperfect_third` (sg, pl)
/// `preterite_first` (sg, pl)
/// `preterite_second` (sg, pl)
/// `preterite_third` (sg, pl)
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
/// `imperfect_subjunctive_se_first` (sg, pl)
/// `imperfect_subjunctive_se_second` (sg, pl)
/// `imperfect_subjunctive_se_third` (sg, pl)
/// `future_subjunctive_first` (sg, pl)
/// `future_subjunctive_second` (sg, pl)
/// `future_subjunctive_third` (sg, pl)
/// `vos` (ind, subj, imp)
/// `imperative` (tu, usted, nosotros, vosotros, ustedes)
/// `negative_imperative` (tu, usted, nosotros, vosotros, ustedes)
/// `infinitive`
/// `gerund`
/// `participle_masculine` (sg, pl)
/// `participle_feminine` (sg, pl)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     present_first: $c0:tt, $c1:tt;
     present_second: $c2:tt, $c3:tt;
     present_third: $c4:tt, $c5:tt;
     imperfect_first: $c6:tt, $c7:tt;
     imperfect_second: $c8:tt, $c9:tt;
     imperfect_third: $c10:tt, $c11:tt;
     preterite_first: $c12:tt, $c13:tt;
     preterite_second: $c14:tt, $c15:tt;
     preterite_third: $c16:tt, $c17:tt;
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
     imperfect_subjunctive_se_first: $c42:tt, $c43:tt;
     imperfect_subjunctive_se_second: $c44:tt, $c45:tt;
     imperfect_subjunctive_se_third: $c46:tt, $c47:tt;
     future_subjunctive_first: $c48:tt, $c49:tt;
     future_subjunctive_second: $c50:tt, $c51:tt;
     future_subjunctive_third: $c52:tt, $c53:tt;
     vos: $c54:tt, $c55:tt, $c56:tt;
     imperative: $c57:tt, $c58:tt, $c59:tt, $c60:tt, $c61:tt;
     negative_imperative: $c62:tt, $c63:tt, $c64:tt, $c65:tt, $c66:tt;
     infinitive: $c67:tt;
     gerund: $c68:tt;
     participle_masculine: $c69:tt, $c70:tt;
     participle_feminine: $c71:tt, $c72:tt;
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
                flexion::form!($c56),
                flexion::form!($c57),
                flexion::form!($c58),
                flexion::form!($c59),
                flexion::form!($c60),
                flexion::form!($c61),
                flexion::form!($c62),
                flexion::form!($c63),
                flexion::form!($c64),
                flexion::form!($c65),
                flexion::form!($c66),
                flexion::form!($c67),
                flexion::form!($c68),
                flexion::form!($c69),
                flexion::form!($c70),
                flexion::form!($c71),
                flexion::form!($c72),
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `car`: 4,817 words.
    Car,
    /// Like `ezar`: 865 words.
    Ezar,
    /// Like `cautarse`: 424 words.
    Cautarse,
    /// Like `ercar`: 404 words.
    Ercar,
    /// Like `quejarse`: 359 words.
    Quejarse,
    /// Like `modarse`: 348 words.
    Modarse,
    /// Like `rilarse`: 329 words.
    Rilarse,
    /// Like `odearse`: 237 words.
    Odearse,
    /// Like `gar`: 221 words.
    Gar,
    /// Like `silizarse`: 217 words.
    Silizarse,
    /// Like `rir`: 212 words.
    Rir,
    /// Like `saber`: 205 words.
    Saber,
    /// Like `pujarse`: 203 words.
    Pujarse,
    /// Like `abastecerse`: 133 words.
    Abastecerse,
    /// Like `medicarse`: 101 words.
    Medicarse,
    /// Like `ger`: 95 words.
    Ger,
    /// Like `cerrar`: 69 words.
    Cerrar,
    /// Like `pliar`: 67 words.
    Pliar,
    /// Like `earse`: 64 words.
    Earse,
    /// Like `volar`: 54 words.
    Volar,
    /// Like `adrarse`: 45 words.
    Adrarse,
    /// Like `situar`: 44 words.
    Situar,
    /// Like `ularse`: 44 words.
    Ularse,
    /// Like `onarse`: 43 words.
    Onarse,
    /// Like `enagarse`: 40 words.
    Enagarse,
    /// Like `ecarse`: 40 words.
    Ecarse,
    /// Like `struir`: 38 words.
    Struir,
    /// Like `ravesarse`: 37 words.
    Ravesarse,
    /// Like `icharse`: 33 words.
    Icharse,
    /// Like `jerir`: 30 words.
    Jerir,
    /// Like `musirse`: 29 words.
    Musirse,
    /// Like `rigir`: 27 words.
    Rigir,
    /// Like `omingarse`: 26 words.
    Omingarse,
    /// Like `trancarse`: 26 words.
    Trancarse,
    /// Like `toserse`: 26 words.
    Toserse,
    /// Like `friarse`: 25 words.
    Friarse,
    /// Like `contrarse`: 24 words.
    Contrarse,
    /// Like `poner`: 23 words.
    Poner,
    /// Like `inuarse`: 23 words.
    Inuarse,
    /// Like `llir`: 22 words.
    Llir,
    /// Like `venir`: 21 words.
    Venir,
    /// Like `guar`: 21 words.
    Guar,
    /// Like `pegarse`: 20 words.
    Pegarse,
    /// Like `venderse`: 19 words.
    Venderse,
    /// Like `ujiar`: 19 words.
    Ujiar,
    /// Like `orzarse`: 19 words.
    Orzarse,
    /// Like `ogerse`: 18 words.
    Ogerse,
    /// Like `miscuirse`: 18 words.
    Miscuirse,
    /// Like `aular`: 17 words.
    Aular,
    /// Like `descocarse`: 17 words.
    Descocarse,
    /// Like `lugarse`: 17 words.
    Lugarse,
    /// Like `ercarse`: 17 words.
    Ercarse,
    /// Like `tender`: 16 words.
    Tender,
    /// Like `ullirse`: 15 words.
    Ullirse,
    /// Like `definirse`: 15 words.
    Definirse,
    /// Like `lazarse`: 14 words.
    Lazarse,
    /// Like `calzarse`: 14 words.
    Calzarse,
    /// Like `caer`: 13 words.
    Caer,
    /// Like `pentirse`: 13 words.
    Pentirse,
    /// Like `sacarse`: 13 words.
    Sacarse,
    /// Like `rcir`: 12 words.
    Rcir,
    /// Like `regar`: 12 words.
    Regar,
    /// Like `seguir`: 12 words.
    Seguir,
    /// Like `aizar`: 12 words.
    Aizar,
    /// Like `oger`: 12 words.
    Oger,
    /// Like `decir`: 11 words.
    Decir,
    /// Like `perderse`: 11 words.
    Perderse,
    /// Like `hacerse`: 11 words.
    Hacerse,
    /// Like `chuzarse`: 11 words.
    Chuzarse,
    /// Like `tener`: 10 words.
    Tener,
    /// Like `onducir`: 10 words.
    Onducir,
    /// Like `partirse`: 10 words.
    Partirse,
    /// Like `uñuscarse`: 10 words.
    Uñuscarse,
    /// Like `torcer`: 9 words.
    Torcer,
    /// Like `rehilar`: 9 words.
    Rehilar,
    /// Like `rogarse`: 9 words.
    Rogarse,
    /// Like `ntiguarse`: 9 words.
    Ntiguarse,
    /// Like `rozarse`: 9 words.
    Rozarse,
    /// Like `volcarse`: 9 words.
    Volcarse,
    /// Like `hacer`: 8 words.
    Hacer,
    /// Like `ivirse`: 8 words.
    Ivirse,
    /// Like `amoriscarse`: 8 words.
    Amoriscarse,
    /// Like `ribir`: 8 words.
    Ribir,
    /// Like `bucarse`: 8 words.
    Bucarse,
    /// Like `atirse`: 8 words.
    Atirse,
    /// Like `refreír`: 7 words.
    Refreír,
    /// Like `colgar`: 7 words.
    Colgar,
    /// Like `ver`: 7 words.
    Ver,
    /// Like `opezar`: 7 words.
    Opezar,
    /// Like `aislarse`: 7 words.
    Aislarse,
    /// Like `mponerse`: 7 words.
    Mponerse,
    /// Like `regarse`: 7 words.
    Regarse,
    /// Like `lfiarse`: 7 words.
    Lfiarse,
    /// Like `rcunscribirse`: 7 words.
    Rcunscribirse,
    /// Like `enderse`: 7 words.
    Enderse,
    /// Like `aer`: 6 words.
    Aer,
    /// Like `aunarse`: 6 words.
    Aunarse,
    /// Like `eblar`: 6 words.
    Eblar,
    /// Like `ducirse`: 6 words.
    Ducirse,
    /// Like `egarse`: 6 words.
    Egarse,
    /// Like `argarse`: 6 words.
    Argarse,
    /// Like `fiar`: 5 words.
    Fiar,
    /// Like `evacuar`: 5 words.
    Evacuar,
    /// Like `ceñir`: 5 words.
    Ceñir,
    /// Like `agorar`: 5 words.
    Agorar,
    /// Like `ñer`: 5 words.
    Ñer,
    /// Like `frescarse`: 5 words.
    Frescarse,
    /// Like `cernirse`: 5 words.
    Cernirse,
    /// Like `juilar`: 5 words.
    Juilar,
    /// Like `pedir`: 4 words.
    Pedir,
    /// Like `moler`: 4 words.
    Moler,
    /// Like `huir`: 4 words.
    Huir,
    /// Like `volver`: 4 words.
    Volver,
    /// Like `colarse`: 4 words.
    Colarse,
    /// Like `tullirse`: 4 words.
    Tullirse,
    /// Like `contraerse`: 4 words.
    Contraerse,
    /// Like `criarse`: 4 words.
    Criarse,
    /// Like `brir`: 4 words.
    Brir,
    /// Like `onocerse`: 4 words.
    Onocerse,
    /// Like `regir`: 3 words.
    Regir,
    /// Like `lucir`: 3 words.
    Lucir,
    /// Like `cocer`: 3 words.
    Cocer,
    /// Like `venirse`: 3 words.
    Venirse,
    /// Like `hibir`: 3 words.
    Hibir,
    /// Like `colgarse`: 3 words.
    Colgarse,
    /// Like `teñirse`: 3 words.
    Teñirse,
    /// Like `despedirse`: 3 words.
    Despedirse,
    /// Like `acogerse`: 3 words.
    Acogerse,
    /// Like `regirse`: 3 words.
    Regirse,
    /// Like `fingirse`: 3 words.
    Fingirse,
    /// Like `fraguarse`: 3 words.
    Fraguarse,
    /// Like `rigirse`: 3 words.
    Rigirse,
    /// Like `punzarse`: 3 words.
    Punzarse,
    /// Like `güir`: 3 words.
    Güir,
    /// Like `tinguir`: 3 words.
    Tinguir,
    /// Like `linquir`: 3 words.
    Linquir,
    /// Like `ancarse`: 3 words.
    Ancarse,
    /// Like `ferirse`: 3 words.
    Ferirse,
    /// Like `ugarse`: 3 words.
    Ugarse,
    /// Like `engreírse`: 2 words.
    Engreírse,
    /// Like `ir`: 1 words.
    Ir,
    /// Like `oír`: 1 words.
    Oír,
    /// Like `reírse`: 1 words.
    Reírse,
    /// Like `irse`: 1 words.
    Irse,
    /// Like `cirse`: 1 words.
    Cirse,
    /// Like `verse`: 1 words.
    Verse,
    /// Like `podrirse`: 1 words.
    Podrirse,
    /// Like `dormirse`: 1 words.
    Dormirse,
    /// Like `morirse`: 1 words.
    Morirse,
    /// Like `eír`: 1 words.
    Eír,
    /// Like `aír`: 1 words.
    Aír,
    /// Like `entirse`: 1 words.
    Entirse,
    /// Like `edirse`: 1 words.
    Edirse,
    /// Like `arse`: 1 words.
    Arse,
    /// Like `etirse`: 1 words.
    Etirse,
    /// Like `eñirse`: 1 words.
    Eñirse,
    /// Like `ormirse`: 1 words.
    Ormirse,
    /// Like `eírse`: 1 words.
    Eírse,
    /// Like `estirse`: 1 words.
    Estirse,
    /// Like `eerse`: 1 words.
    Eerse,
    /// Like `aírse`: 1 words.
    Aírse,
    /// Like `odrirse`: 1 words.
    Odrirse,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 162] = [
        Conjugation::Car,
        Conjugation::Ezar,
        Conjugation::Cautarse,
        Conjugation::Ercar,
        Conjugation::Quejarse,
        Conjugation::Modarse,
        Conjugation::Rilarse,
        Conjugation::Odearse,
        Conjugation::Gar,
        Conjugation::Silizarse,
        Conjugation::Rir,
        Conjugation::Saber,
        Conjugation::Pujarse,
        Conjugation::Abastecerse,
        Conjugation::Medicarse,
        Conjugation::Ger,
        Conjugation::Cerrar,
        Conjugation::Pliar,
        Conjugation::Earse,
        Conjugation::Volar,
        Conjugation::Adrarse,
        Conjugation::Situar,
        Conjugation::Ularse,
        Conjugation::Onarse,
        Conjugation::Enagarse,
        Conjugation::Ecarse,
        Conjugation::Struir,
        Conjugation::Ravesarse,
        Conjugation::Icharse,
        Conjugation::Jerir,
        Conjugation::Musirse,
        Conjugation::Rigir,
        Conjugation::Omingarse,
        Conjugation::Trancarse,
        Conjugation::Toserse,
        Conjugation::Friarse,
        Conjugation::Contrarse,
        Conjugation::Poner,
        Conjugation::Inuarse,
        Conjugation::Llir,
        Conjugation::Venir,
        Conjugation::Guar,
        Conjugation::Pegarse,
        Conjugation::Venderse,
        Conjugation::Ujiar,
        Conjugation::Orzarse,
        Conjugation::Ogerse,
        Conjugation::Miscuirse,
        Conjugation::Aular,
        Conjugation::Descocarse,
        Conjugation::Lugarse,
        Conjugation::Ercarse,
        Conjugation::Tender,
        Conjugation::Ullirse,
        Conjugation::Definirse,
        Conjugation::Lazarse,
        Conjugation::Calzarse,
        Conjugation::Caer,
        Conjugation::Pentirse,
        Conjugation::Sacarse,
        Conjugation::Rcir,
        Conjugation::Regar,
        Conjugation::Seguir,
        Conjugation::Aizar,
        Conjugation::Oger,
        Conjugation::Decir,
        Conjugation::Perderse,
        Conjugation::Hacerse,
        Conjugation::Chuzarse,
        Conjugation::Tener,
        Conjugation::Onducir,
        Conjugation::Partirse,
        Conjugation::Uñuscarse,
        Conjugation::Torcer,
        Conjugation::Rehilar,
        Conjugation::Rogarse,
        Conjugation::Ntiguarse,
        Conjugation::Rozarse,
        Conjugation::Volcarse,
        Conjugation::Hacer,
        Conjugation::Ivirse,
        Conjugation::Amoriscarse,
        Conjugation::Ribir,
        Conjugation::Bucarse,
        Conjugation::Atirse,
        Conjugation::Refreír,
        Conjugation::Colgar,
        Conjugation::Ver,
        Conjugation::Opezar,
        Conjugation::Aislarse,
        Conjugation::Mponerse,
        Conjugation::Regarse,
        Conjugation::Lfiarse,
        Conjugation::Rcunscribirse,
        Conjugation::Enderse,
        Conjugation::Aer,
        Conjugation::Aunarse,
        Conjugation::Eblar,
        Conjugation::Ducirse,
        Conjugation::Egarse,
        Conjugation::Argarse,
        Conjugation::Fiar,
        Conjugation::Evacuar,
        Conjugation::Ceñir,
        Conjugation::Agorar,
        Conjugation::Ñer,
        Conjugation::Frescarse,
        Conjugation::Cernirse,
        Conjugation::Juilar,
        Conjugation::Pedir,
        Conjugation::Moler,
        Conjugation::Huir,
        Conjugation::Volver,
        Conjugation::Colarse,
        Conjugation::Tullirse,
        Conjugation::Contraerse,
        Conjugation::Criarse,
        Conjugation::Brir,
        Conjugation::Onocerse,
        Conjugation::Regir,
        Conjugation::Lucir,
        Conjugation::Cocer,
        Conjugation::Venirse,
        Conjugation::Hibir,
        Conjugation::Colgarse,
        Conjugation::Teñirse,
        Conjugation::Despedirse,
        Conjugation::Acogerse,
        Conjugation::Regirse,
        Conjugation::Fingirse,
        Conjugation::Fraguarse,
        Conjugation::Rigirse,
        Conjugation::Punzarse,
        Conjugation::Güir,
        Conjugation::Tinguir,
        Conjugation::Linquir,
        Conjugation::Ancarse,
        Conjugation::Ferirse,
        Conjugation::Ugarse,
        Conjugation::Engreírse,
        Conjugation::Ir,
        Conjugation::Oír,
        Conjugation::Reírse,
        Conjugation::Irse,
        Conjugation::Cirse,
        Conjugation::Verse,
        Conjugation::Podrirse,
        Conjugation::Dormirse,
        Conjugation::Morirse,
        Conjugation::Eír,
        Conjugation::Aír,
        Conjugation::Entirse,
        Conjugation::Edirse,
        Conjugation::Arse,
        Conjugation::Etirse,
        Conjugation::Eñirse,
        Conjugation::Ormirse,
        Conjugation::Eírse,
        Conjugation::Estirse,
        Conjugation::Eerse,
        Conjugation::Aírse,
        Conjugation::Odrirse,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 162] = [
    // Car: 4,817 words (estar, amar, crear, dar, far, nuclear).
    conjugation!("car", 4817, "~ar";
        present_first: "~o", "~amos";
        present_second: "~as", "~áis";
        present_third: "~a", "~an";
        imperfect_first: "~aba", "~ábamos";
        imperfect_second: "~abas", "~abais";
        imperfect_third: "~aba", "~aban";
        preterite_first: "~é", "~amos";
        preterite_second: "~aste", "~asteis";
        preterite_third: "~ó", "~aron";
        future_first: "~aré", "~aremos";
        future_second: "~arás", "~aréis";
        future_third: "~ará", "~arán";
        conditional_first: "~aría", "~aríamos";
        conditional_second: "~arías", "~aríais";
        conditional_third: "~aría", "~arían";
        present_subjunctive_first: "~e", "~emos";
        present_subjunctive_second: "~es", "~éis";
        present_subjunctive_third: "~e", "~en";
        imperfect_subjunctive_first: "~ara", "~áramos";
        imperfect_subjunctive_second: "~aras", "~arais";
        imperfect_subjunctive_third: "~ara", "~aran";
        imperfect_subjunctive_se_first: "~ase", "~ásemos";
        imperfect_subjunctive_se_second: "~ases", "~aseis";
        imperfect_subjunctive_se_third: "~ase", "~asen";
        future_subjunctive_first: "~are", "~áremos";
        future_subjunctive_second: "~ares", "~areis";
        future_subjunctive_third: "~are", "~aren";
        vos: "~ás", "~és", "~á";
        imperative: "~a", "~e", "~emos", "~ad", "~en";
        negative_imperative: "~es", "~e", "~emos", "~éis", "~en";
        infinitive: "~ar";
        gerund: "~ando";
        participle_masculine: "~ado", "~ados";
        participle_feminine: "~ada", "~adas";
    ),
    // Ezar: 865 words (utilizar, avergonzar, oficializar, gozar, cruzar, forzar).
    conjugation!("ezar", 865, "~zar";
        present_first: "~zo", "~zamos";
        present_second: "~zas", "~záis";
        present_third: "~za", "~zan";
        imperfect_first: "~zaba", "~zábamos";
        imperfect_second: "~zabas", "~zabais";
        imperfect_third: "~zaba", "~zaban";
        preterite_first: "~cé", "~zamos";
        preterite_second: "~zaste", "~zasteis";
        preterite_third: "~zó", "~zaron";
        future_first: "~zaré", "~zaremos";
        future_second: "~zarás", "~zaréis";
        future_third: "~zará", "~zarán";
        conditional_first: "~zaría", "~zaríamos";
        conditional_second: "~zarías", "~zaríais";
        conditional_third: "~zaría", "~zarían";
        present_subjunctive_first: "~ce", "~cemos";
        present_subjunctive_second: "~ces", "~céis";
        present_subjunctive_third: "~ce", "~cen";
        imperfect_subjunctive_first: "~zara", "~záramos";
        imperfect_subjunctive_second: "~zaras", "~zarais";
        imperfect_subjunctive_third: "~zara", "~zaran";
        imperfect_subjunctive_se_first: "~zase", "~zásemos";
        imperfect_subjunctive_se_second: "~zases", "~zaseis";
        imperfect_subjunctive_se_third: "~zase", "~zasen";
        future_subjunctive_first: "~zare", "~záremos";
        future_subjunctive_second: "~zares", "~zareis";
        future_subjunctive_third: "~zare", "~zaren";
        vos: "~zás", "~cés", "~zá";
        imperative: "~za", "~ce", "~cemos", "~zad", "~cen";
        negative_imperative: "~ces", "~ce", "~cemos", "~céis", "~cen";
        infinitive: "~zar";
        gerund: "~zando";
        participle_masculine: "~zado", "~zados";
        participle_feminine: "~zada", "~zadas";
    ),
    // Cautarse: 424 words (jambarse, descararse, desbandarse, agarbarse, entanarse, ufanarse).
    conjugation!("cautarse", 424, "~a~arse";
        present_first: "~a~o", "~a~amos";
        present_second: "~a~as", "~a~áis";
        present_third: "~a~a", "~a~an";
        imperfect_first: "~a~aba", "~a~ábamos";
        imperfect_second: "~a~abas", "~a~abais";
        imperfect_third: "~a~aba", "~a~aban";
        preterite_first: "~a~é", "~a~amos";
        preterite_second: "~a~aste", "~a~asteis";
        preterite_third: "~a~ó", "~a~aron";
        future_first: "~a~aré", "~a~aremos";
        future_second: "~a~arás", "~a~aréis";
        future_third: "~a~ará", "~a~arán";
        conditional_first: "~a~aría", "~a~aríamos";
        conditional_second: "~a~arías", "~a~aríais";
        conditional_third: "~a~aría", "~a~arían";
        present_subjunctive_first: "~a~e", "~a~emos";
        present_subjunctive_second: "~a~es", "~a~éis";
        present_subjunctive_third: "~a~e", "~a~en";
        imperfect_subjunctive_first: "~a~ara", "~a~áramos";
        imperfect_subjunctive_second: "~a~aras", "~a~arais";
        imperfect_subjunctive_third: "~a~ara", "~a~aran";
        imperfect_subjunctive_se_first: "~a~ase", "~a~ásemos";
        imperfect_subjunctive_se_second: "~a~ases", "~a~aseis";
        imperfect_subjunctive_se_third: "~a~ase", "~a~asen";
        future_subjunctive_first: "~a~are", "~a~áremos";
        future_subjunctive_second: "~a~ares", "~a~areis";
        future_subjunctive_third: "~a~are", "~a~aren";
        vos: "~a~ás", "~a~és", "~a~ate";
        imperative: "~á~ate", "~á~ese", "~a~émonos", "~a~aos", "~á~ense";
        negative_imperative: "~a~es", "~a~e", "~a~emos", "~a~éis", "~a~en";
        infinitive: "~a~arse";
        gerund: "~a~ándose";
        participle_masculine: "~a~ado", "~a~ados";
        participle_feminine: "~a~ada", "~a~adas";
    ),
    // Ercar: 404 words (ubicar, justificar, identificar, significar, publicar, educar).
    conjugation!("ercar", 404, "~car";
        present_first: "~co", "~camos";
        present_second: "~cas", "~cáis";
        present_third: "~ca", "~can";
        imperfect_first: "~caba", "~cábamos";
        imperfect_second: "~cabas", "~cabais";
        imperfect_third: "~caba", "~caban";
        preterite_first: "~qué", "~camos";
        preterite_second: "~caste", "~casteis";
        preterite_third: "~có", "~caron";
        future_first: "~caré", "~caremos";
        future_second: "~carás", "~caréis";
        future_third: "~cará", "~carán";
        conditional_first: "~caría", "~caríamos";
        conditional_second: "~carías", "~caríais";
        conditional_third: "~caría", "~carían";
        present_subjunctive_first: "~que", "~quemos";
        present_subjunctive_second: "~ques", "~quéis";
        present_subjunctive_third: "~que", "~quen";
        imperfect_subjunctive_first: "~cara", "~cáramos";
        imperfect_subjunctive_second: "~caras", "~carais";
        imperfect_subjunctive_third: "~cara", "~caran";
        imperfect_subjunctive_se_first: "~case", "~cásemos";
        imperfect_subjunctive_se_second: "~cases", "~caseis";
        imperfect_subjunctive_se_third: "~case", "~casen";
        future_subjunctive_first: "~care", "~cáremos";
        future_subjunctive_second: "~cares", "~careis";
        future_subjunctive_third: "~care", "~caren";
        vos: "~cás", "~qués", "~cá";
        imperative: "~ca", "~que", "~quemos", "~cad", "~quen";
        negative_imperative: "~ques", "~que", "~quemos", "~quéis", "~quen";
        infinitive: "~car";
        gerund: "~cando";
        participle_masculine: "~cado", "~cados";
        participle_feminine: "~cada", "~cadas";
    ),
    // Quejarse: 359 words (quejarse, querellarse, despertarse, aquerenciarse, herniarse, dejarse).
    conjugation!("quejarse", 359, "~e~arse";
        present_first: "~e~o", "~e~amos";
        present_second: "~e~as", "~e~áis";
        present_third: "~e~a", "~e~an";
        imperfect_first: "~e~aba", "~e~ábamos";
        imperfect_second: "~e~abas", "~e~abais";
        imperfect_third: "~e~aba", "~e~aban";
        preterite_first: "~e~é", "~e~amos";
        preterite_second: "~e~aste", "~e~asteis";
        preterite_third: "~e~ó", "~e~aron";
        future_first: "~e~aré", "~e~aremos";
        future_second: "~e~arás", "~e~aréis";
        future_third: "~e~ará", "~e~arán";
        conditional_first: "~e~aría", "~e~aríamos";
        conditional_second: "~e~arías", "~e~aríais";
        conditional_third: "~e~aría", "~e~arían";
        present_subjunctive_first: "~e~e", "~e~emos";
        present_subjunctive_second: "~e~es", "~e~éis";
        present_subjunctive_third: "~e~e", "~e~en";
        imperfect_subjunctive_first: "~e~ara", "~e~áramos";
        imperfect_subjunctive_second: "~e~aras", "~e~arais";
        imperfect_subjunctive_third: "~e~ara", "~e~aran";
        imperfect_subjunctive_se_first: "~e~ase", "~e~ásemos";
        imperfect_subjunctive_se_second: "~e~ases", "~e~aseis";
        imperfect_subjunctive_se_third: "~e~ase", "~e~asen";
        future_subjunctive_first: "~e~are", "~e~áremos";
        future_subjunctive_second: "~e~ares", "~e~areis";
        future_subjunctive_third: "~e~are", "~e~aren";
        vos: "~e~ás", "~e~és", "~e~ate";
        imperative: "~é~ate", "~é~ese", "~e~émonos", "~e~aos", "~é~ense";
        negative_imperative: "~e~es", "~e~e", "~e~emos", "~e~éis", "~e~en";
        infinitive: "~e~arse";
        gerund: "~e~ándose";
        participle_masculine: "~e~ado", "~e~ados";
        participle_feminine: "~e~ada", "~e~adas";
    ),
    // Modarse: 348 words (acodarse, personarse, encostarse, enconcharse, encoñarse, acorcharse).
    conjugation!("modarse", 348, "~o~arse";
        present_first: "~o~o", "~o~amos";
        present_second: "~o~as", "~o~áis";
        present_third: "~o~a", "~o~an";
        imperfect_first: "~o~aba", "~o~ábamos";
        imperfect_second: "~o~abas", "~o~abais";
        imperfect_third: "~o~aba", "~o~aban";
        preterite_first: "~o~é", "~o~amos";
        preterite_second: "~o~aste", "~o~asteis";
        preterite_third: "~o~ó", "~o~aron";
        future_first: "~o~aré", "~o~aremos";
        future_second: "~o~arás", "~o~aréis";
        future_third: "~o~ará", "~o~arán";
        conditional_first: "~o~aría", "~o~aríamos";
        conditional_second: "~o~arías", "~o~aríais";
        conditional_third: "~o~aría", "~o~arían";
        present_subjunctive_first: "~o~e", "~o~emos";
        present_subjunctive_second: "~o~es", "~o~éis";
        present_subjunctive_third: "~o~e", "~o~en";
        imperfect_subjunctive_first: "~o~ara", "~o~áramos";
        imperfect_subjunctive_second: "~o~aras", "~o~arais";
        imperfect_subjunctive_third: "~o~ara", "~o~aran";
        imperfect_subjunctive_se_first: "~o~ase", "~o~ásemos";
        imperfect_subjunctive_se_second: "~o~ases", "~o~aseis";
        imperfect_subjunctive_se_third: "~o~ase", "~o~asen";
        future_subjunctive_first: "~o~are", "~o~áremos";
        future_subjunctive_second: "~o~ares", "~o~areis";
        future_subjunctive_third: "~o~are", "~o~aren";
        vos: "~o~ás", "~o~és", "~o~ate";
        imperative: "~ó~ate", "~ó~ese", "~o~émonos", "~o~aos", "~ó~ense";
        negative_imperative: "~o~es", "~o~e", "~o~emos", "~o~éis", "~o~en";
        infinitive: "~o~arse";
        gerund: "~o~ándose";
        participle_masculine: "~o~ado", "~o~ados";
        participle_feminine: "~o~ada", "~o~adas";
    ),
    // Rilarse: 329 words (fijarse, chisparse, compincharse, dignarse, rilarse, pirrarse).
    conjugation!("rilarse", 329, "~i~arse";
        present_first: "~i~o", "~i~amos";
        present_second: "~i~as", "~i~áis";
        present_third: "~i~a", "~i~an";
        imperfect_first: "~i~aba", "~i~ábamos";
        imperfect_second: "~i~abas", "~i~abais";
        imperfect_third: "~i~aba", "~i~aban";
        preterite_first: "~i~é", "~i~amos";
        preterite_second: "~i~aste", "~i~asteis";
        preterite_third: "~i~ó", "~i~aron";
        future_first: "~i~aré", "~i~aremos";
        future_second: "~i~arás", "~i~aréis";
        future_third: "~i~ará", "~i~arán";
        conditional_first: "~i~aría", "~i~aríamos";
        conditional_second: "~i~arías", "~i~aríais";
        conditional_third: "~i~aría", "~i~arían";
        present_subjunctive_first: "~i~e", "~i~emos";
        present_subjunctive_second: "~i~es", "~i~éis";
        present_subjunctive_third: "~i~e", "~i~en";
        imperfect_subjunctive_first: "~i~ara", "~i~áramos";
        imperfect_subjunctive_second: "~i~aras", "~i~arais";
        imperfect_subjunctive_third: "~i~ara", "~i~aran";
        imperfect_subjunctive_se_first: "~i~ase", "~i~ásemos";
        imperfect_subjunctive_se_second: "~i~ases", "~i~aseis";
        imperfect_subjunctive_se_third: "~i~ase", "~i~asen";
        future_subjunctive_first: "~i~are", "~i~áremos";
        future_subjunctive_second: "~i~ares", "~i~areis";
        future_subjunctive_third: "~i~are", "~i~aren";
        vos: "~i~ás", "~i~és", "~i~ate";
        imperative: "~í~ate", "~í~ese", "~i~émonos", "~i~aos", "~í~ense";
        negative_imperative: "~i~es", "~i~e", "~i~emos", "~i~éis", "~i~en";
        infinitive: "~i~arse";
        gerund: "~i~ándose";
        participle_masculine: "~i~ado", "~i~ados";
        participle_feminine: "~i~ada", "~i~adas";
    ),
    // Odearse: 237 words (farrearse, olfatearse, despearse, zocatearse, volquearse, falopearse).
    conjugation!("odearse", 237, "~~arse";
        present_first: "~~o", "~~amos";
        present_second: "~~as", "~~áis";
        present_third: "~~a", "~~an";
        imperfect_first: "~~aba", "~~ábamos";
        imperfect_second: "~~abas", "~~abais";
        imperfect_third: "~~aba", "~~aban";
        preterite_first: "~~é", "~~amos";
        preterite_second: "~~aste", "~~asteis";
        preterite_third: "~~ó", "~~aron";
        future_first: "~~aré", "~~aremos";
        future_second: "~~arás", "~~aréis";
        future_third: "~~ará", "~~arán";
        conditional_first: "~~aría", "~~aríamos";
        conditional_second: "~~arías", "~~aríais";
        conditional_third: "~~aría", "~~arían";
        present_subjunctive_first: "~~e", "~~emos";
        present_subjunctive_second: "~~es", "~~éis";
        present_subjunctive_third: "~~e", "~~en";
        imperfect_subjunctive_first: "~~ara", "~~áramos";
        imperfect_subjunctive_second: "~~aras", "~~arais";
        imperfect_subjunctive_third: "~~ara", "~~aran";
        imperfect_subjunctive_se_first: "~~ase", "~~ásemos";
        imperfect_subjunctive_se_second: "~~ases", "~~aseis";
        imperfect_subjunctive_se_third: "~~ase", "~~asen";
        future_subjunctive_first: "~~are", "~~áremos";
        future_subjunctive_second: "~~ares", "~~areis";
        future_subjunctive_third: "~~are", "~~aren";
        vos: "~~ás", "~~és", "~~ate";
        imperative: "~éat~", "~é~se", "~~émonos", "~~aos", "~é~nse";
        negative_imperative: "~~es", "~~e", "~~emos", "~~éis", "~~en";
        infinitive: "~~arse";
        gerund: "~~ándose";
        participle_masculine: "~~ado", "~~ados";
        participle_feminine: "~~ada", "~~adas";
    ),
    // Gar: 221 words (fatigar, juzgar, jugar, agregar, navegar, otorgar).
    conjugation!("gar", 221, "~ar";
        present_first: "~o", "~amos";
        present_second: "~as", "~áis";
        present_third: "~a", "~an";
        imperfect_first: "~aba", "~ábamos";
        imperfect_second: "~abas", "~abais";
        imperfect_third: "~aba", "~aban";
        preterite_first: "~ué", "~amos";
        preterite_second: "~aste", "~asteis";
        preterite_third: "~ó", "~aron";
        future_first: "~aré", "~aremos";
        future_second: "~arás", "~aréis";
        future_third: "~ará", "~arán";
        conditional_first: "~aría", "~aríamos";
        conditional_second: "~arías", "~aríais";
        conditional_third: "~aría", "~arían";
        present_subjunctive_first: "~ue", "~uemos";
        present_subjunctive_second: "~ues", "~uéis";
        present_subjunctive_third: "~ue", "~uen";
        imperfect_subjunctive_first: "~ara", "~áramos";
        imperfect_subjunctive_second: "~aras", "~arais";
        imperfect_subjunctive_third: "~ara", "~aran";
        imperfect_subjunctive_se_first: "~ase", "~ásemos";
        imperfect_subjunctive_se_second: "~ases", "~aseis";
        imperfect_subjunctive_se_third: "~ase", "~asen";
        future_subjunctive_first: "~are", "~áremos";
        future_subjunctive_second: "~ares", "~areis";
        future_subjunctive_third: "~are", "~aren";
        vos: "~ás", "~ués", "~á";
        imperative: "~a", "~ue", "~uemos", "~ad", "~uen";
        negative_imperative: "~ues", "~ue", "~uemos", "~uéis", "~uen";
        infinitive: "~ar";
        gerund: "~ando";
        participle_masculine: "~ado", "~ados";
        participle_feminine: "~ada", "~adas";
    ),
    // Silizarse: 217 words (nebulizarse, atizarse, finalizarse, velarizarse, urbanizarse, helenizarse).
    conjugation!("silizarse", 217, "~izarse";
        present_first: "~izo", "~izamos";
        present_second: "~izas", "~izáis";
        present_third: "~iza", "~izan";
        imperfect_first: "~izaba", "~izábamos";
        imperfect_second: "~izabas", "~izabais";
        imperfect_third: "~izaba", "~izaban";
        preterite_first: "~icé", "~izamos";
        preterite_second: "~izaste", "~izasteis";
        preterite_third: "~izó", "~izaron";
        future_first: "~izaré", "~izaremos";
        future_second: "~izarás", "~izaréis";
        future_third: "~izará", "~izarán";
        conditional_first: "~izaría", "~izaríamos";
        conditional_second: "~izarías", "~izaríais";
        conditional_third: "~izaría", "~izarían";
        present_subjunctive_first: "~ice", "~icemos";
        present_subjunctive_second: "~ices", "~icéis";
        present_subjunctive_third: "~ice", "~icen";
        imperfect_subjunctive_first: "~izara", "~izáramos";
        imperfect_subjunctive_second: "~izaras", "~izarais";
        imperfect_subjunctive_third: "~izara", "~izaran";
        imperfect_subjunctive_se_first: "~izase", "~izásemos";
        imperfect_subjunctive_se_second: "~izases", "~izaseis";
        imperfect_subjunctive_se_third: "~izase", "~izasen";
        future_subjunctive_first: "~izare", "~izáremos";
        future_subjunctive_second: "~izares", "~izareis";
        future_subjunctive_third: "~izare", "~izaren";
        vos: "~izás", "~icés", "~izate";
        imperative: "~ízate", "~ícese", "~icémonos", "~izaos", "~ícense";
        negative_imperative: "~ices", "~ice", "~icemos", "~icéis", "~icen";
        infinitive: "~izarse";
        gerund: "~izándose";
        participle_masculine: "~izado", "~izados";
        participle_feminine: "~izada", "~izadas";
    ),
    // Rir: 212 words (dormir, finir, asir, unir, reunir, morir).
    conjugation!("rir", 212, "~ir";
        present_first: "~o", "~imos";
        present_second: "~es", "~ís";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~ió", "~ieron";
        future_first: "~iré", "~iremos";
        future_second: "~irás", "~iréis";
        future_third: "~irá", "~irán";
        conditional_first: "~iría", "~iríamos";
        conditional_second: "~irías", "~iríais";
        conditional_third: "~iría", "~irían";
        present_subjunctive_first: "~a", "~amos";
        present_subjunctive_second: "~as", "~áis";
        present_subjunctive_third: "~a", "~an";
        imperfect_subjunctive_first: "~iera", "~iéramos";
        imperfect_subjunctive_second: "~ieras", "~ierais";
        imperfect_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_se_first: "~iese", "~iésemos";
        imperfect_subjunctive_se_second: "~ieses", "~ieseis";
        imperfect_subjunctive_se_third: "~iese", "~iesen";
        future_subjunctive_first: "~iere", "~iéremos";
        future_subjunctive_second: "~ieres", "~iereis";
        future_subjunctive_third: "~iere", "~ieren";
        vos: "~ís", "~ás", "~í";
        imperative: "~e", "~a", "~amos", "~id", "~an";
        negative_imperative: "~as", "~a", "~amos", "~áis", "~an";
        infinitive: "~ir";
        gerund: "~iendo";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Saber: 205 words (saber, caber, haber, yacer, nacer, crecer).
    conjugation!("saber", 205, "~~er";
        present_first: "~z~o", "~~emos";
        present_second: "~~es", "~~éis";
        present_third: "~~e", "~~en";
        imperfect_first: "~~ía", "~~íamos";
        imperfect_second: "~~ías", "~~íais";
        imperfect_third: "~~ía", "~~ían";
        preterite_first: "~~í", "~~imos";
        preterite_second: "~~iste", "~~isteis";
        preterite_third: "~~ió", "~~ieron";
        future_first: "~~eré", "~~eremos";
        future_second: "~~erás", "~~eréis";
        future_third: "~~erá", "~~erán";
        conditional_first: "~~ería", "~~eríamos";
        conditional_second: "~~erías", "~~eríais";
        conditional_third: "~~ería", "~~erían";
        present_subjunctive_first: "~z~a", "~z~amos";
        present_subjunctive_second: "~z~as", "~z~áis";
        present_subjunctive_third: "~z~a", "~z~an";
        imperfect_subjunctive_first: "~~iera", "~~iéramos";
        imperfect_subjunctive_second: "~~ieras", "~~ierais";
        imperfect_subjunctive_third: "~~iera", "~~ieran";
        imperfect_subjunctive_se_first: "~~iese", "~~iésemos";
        imperfect_subjunctive_se_second: "~~ieses", "~~ieseis";
        imperfect_subjunctive_se_third: "~~iese", "~~iesen";
        future_subjunctive_first: "~~iere", "~~iéremos";
        future_subjunctive_second: "~~ieres", "~~iereis";
        future_subjunctive_third: "~~iere", "~~ieren";
        vos: "~~és", "~z~ás", "~~é";
        imperative: "~~e", "~z~a", "~z~amos", "~~ed", "~z~an";
        negative_imperative: "~z~as", "~z~a", "~z~amos", "~z~áis", "~z~an";
        infinitive: "~~er";
        gerund: "~~iendo";
        participle_masculine: "~~ido", "~~idos";
        participle_feminine: "~~ida", "~~idas";
    ),
    // Pujarse: 203 words (jumarse, ñublarse, juntarse, acuitarse, zullarse, cuidarse).
    conjugation!("pujarse", 203, "~u~arse";
        present_first: "~u~o", "~u~amos";
        present_second: "~u~as", "~u~áis";
        present_third: "~u~a", "~u~an";
        imperfect_first: "~u~aba", "~u~ábamos";
        imperfect_second: "~u~abas", "~u~abais";
        imperfect_third: "~u~aba", "~u~aban";
        preterite_first: "~u~é", "~u~amos";
        preterite_second: "~u~aste", "~u~asteis";
        preterite_third: "~u~ó", "~u~aron";
        future_first: "~u~aré", "~u~aremos";
        future_second: "~u~arás", "~u~aréis";
        future_third: "~u~ará", "~u~arán";
        conditional_first: "~u~aría", "~u~aríamos";
        conditional_second: "~u~arías", "~u~aríais";
        conditional_third: "~u~aría", "~u~arían";
        present_subjunctive_first: "~u~e", "~u~emos";
        present_subjunctive_second: "~u~es", "~u~éis";
        present_subjunctive_third: "~u~e", "~u~en";
        imperfect_subjunctive_first: "~u~ara", "~u~áramos";
        imperfect_subjunctive_second: "~u~aras", "~u~arais";
        imperfect_subjunctive_third: "~u~ara", "~u~aran";
        imperfect_subjunctive_se_first: "~u~ase", "~u~ásemos";
        imperfect_subjunctive_se_second: "~u~ases", "~u~aseis";
        imperfect_subjunctive_se_third: "~u~ase", "~u~asen";
        future_subjunctive_first: "~u~are", "~u~áremos";
        future_subjunctive_second: "~u~ares", "~u~areis";
        future_subjunctive_third: "~u~are", "~u~aren";
        vos: "~u~ás", "~u~és", "~u~ate";
        imperative: "~ú~ate", "~ú~ese", "~u~émonos", "~u~aos", "~ú~ense";
        negative_imperative: "~u~es", "~u~e", "~u~emos", "~u~éis", "~u~en";
        infinitive: "~u~arse";
        gerund: "~u~ándose";
        participle_masculine: "~u~ado", "~u~ados";
        participle_feminine: "~u~ada", "~u~adas";
    ),
    // Abastecerse: 133 words (reverse, rehacerse, oscurecerse, favorecerse, evanecerse, estremecerse).
    conjugation!("abastecerse", 133, "~e~erse";
        present_first: "~ez~o", "~e~emos";
        present_second: "~e~es", "~e~éis";
        present_third: "~e~e", "~e~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~e~í", "~e~imos";
        preterite_second: "~e~iste", "~e~isteis";
        preterite_third: "~e~ió", "~e~ieron";
        future_first: "~e~eré", "~e~eremos";
        future_second: "~e~erás", "~e~eréis";
        future_third: "~e~erá", "~e~erán";
        conditional_first: "~e~ería", "~e~eríamos";
        conditional_second: "~e~erías", "~e~eríais";
        conditional_third: "~e~ería", "~e~erían";
        present_subjunctive_first: "~ez~a", "~ez~amos";
        present_subjunctive_second: "~ez~as", "~ez~áis";
        present_subjunctive_third: "~ez~a", "~ez~an";
        imperfect_subjunctive_first: "~e~iera", "~e~iéramos";
        imperfect_subjunctive_second: "~e~ieras", "~e~ierais";
        imperfect_subjunctive_third: "~e~iera", "~e~ieran";
        imperfect_subjunctive_se_first: "~e~iese", "~e~iésemos";
        imperfect_subjunctive_se_second: "~e~ieses", "~e~ieseis";
        imperfect_subjunctive_se_third: "~e~iese", "~e~iesen";
        future_subjunctive_first: "~e~iere", "~e~iéremos";
        future_subjunctive_second: "~e~ieres", "~e~iereis";
        future_subjunctive_third: "~e~iere", "~e~ieren";
        vos: "~e~és", "~ez~ás", "~e~ete";
        imperative: "~é~ete", "~éz~ase", "~ez~ámonos", "~e~eos", "~éz~anse";
        negative_imperative: "~ez~as", "~ez~a", "~ez~amos", "~ez~áis", "~ez~an";
        infinitive: "~e~erse";
        gerund: "~e~iéndose";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Medicarse: 101 words (osificarse, picarse, ubicarse, identificarse, sindicarse, justificarse).
    conjugation!("medicarse", 101, "~icarse";
        present_first: "~ico", "~icamos";
        present_second: "~icas", "~icáis";
        present_third: "~ica", "~ican";
        imperfect_first: "~icaba", "~icábamos";
        imperfect_second: "~icabas", "~icabais";
        imperfect_third: "~icaba", "~icaban";
        preterite_first: "~iqué", "~icamos";
        preterite_second: "~icaste", "~icasteis";
        preterite_third: "~icó", "~icaron";
        future_first: "~icaré", "~icaremos";
        future_second: "~icarás", "~icaréis";
        future_third: "~icará", "~icarán";
        conditional_first: "~icaría", "~icaríamos";
        conditional_second: "~icarías", "~icaríais";
        conditional_third: "~icaría", "~icarían";
        present_subjunctive_first: "~ique", "~iquemos";
        present_subjunctive_second: "~iques", "~iquéis";
        present_subjunctive_third: "~ique", "~iquen";
        imperfect_subjunctive_first: "~icara", "~icáramos";
        imperfect_subjunctive_second: "~icaras", "~icarais";
        imperfect_subjunctive_third: "~icara", "~icaran";
        imperfect_subjunctive_se_first: "~icase", "~icásemos";
        imperfect_subjunctive_se_second: "~icases", "~icaseis";
        imperfect_subjunctive_se_third: "~icase", "~icasen";
        future_subjunctive_first: "~icare", "~icáremos";
        future_subjunctive_second: "~icares", "~icareis";
        future_subjunctive_third: "~icare", "~icaren";
        vos: "~icás", "~iqués", "~icate";
        imperative: "~ícate", "~íquese", "~iquémonos", "~icaos", "~íquense";
        negative_imperative: "~iques", "~ique", "~iquemos", "~iquéis", "~iquen";
        infinitive: "~icarse";
        gerund: "~icándose";
        participle_masculine: "~icado", "~icados";
        participle_feminine: "~icada", "~icadas";
    ),
    // Ger: 95 words (her, querer, vender, pretender, oler, temer).
    conjugation!("ger", 95, "~er";
        present_first: "~o", "~emos";
        present_second: "~es", "~éis";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~ió", "~ieron";
        future_first: "~eré", "~eremos";
        future_second: "~erás", "~eréis";
        future_third: "~erá", "~erán";
        conditional_first: "~ería", "~eríamos";
        conditional_second: "~erías", "~eríais";
        conditional_third: "~ería", "~erían";
        present_subjunctive_first: "~a", "~amos";
        present_subjunctive_second: "~as", "~áis";
        present_subjunctive_third: "~a", "~an";
        imperfect_subjunctive_first: "~iera", "~iéramos";
        imperfect_subjunctive_second: "~ieras", "~ierais";
        imperfect_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_se_first: "~iese", "~iésemos";
        imperfect_subjunctive_se_second: "~ieses", "~ieseis";
        imperfect_subjunctive_se_third: "~iese", "~iesen";
        future_subjunctive_first: "~iere", "~iéremos";
        future_subjunctive_second: "~ieres", "~iereis";
        future_subjunctive_third: "~iere", "~ieren";
        vos: "~és", "~ás", "~é";
        imperative: "~e", "~a", "~amos", "~ed", "~an";
        negative_imperative: "~as", "~a", "~amos", "~áis", "~an";
        infinitive: "~er";
        gerund: "~iendo";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Cerrar: 69 words (pensar, quebrar, helar, gobernar, atestar, apretar).
    conjugation!("cerrar", 69, "~~ar";
        present_first: "~i~o", "~~amos";
        present_second: "~i~as", "~~áis";
        present_third: "~i~a", "~i~an";
        imperfect_first: "~~aba", "~~ábamos";
        imperfect_second: "~~abas", "~~abais";
        imperfect_third: "~~aba", "~~aban";
        preterite_first: "~~é", "~~amos";
        preterite_second: "~~aste", "~~asteis";
        preterite_third: "~~ó", "~~aron";
        future_first: "~~aré", "~~aremos";
        future_second: "~~arás", "~~aréis";
        future_third: "~~ará", "~~arán";
        conditional_first: "~~aría", "~~aríamos";
        conditional_second: "~~arías", "~~aríais";
        conditional_third: "~~aría", "~~arían";
        present_subjunctive_first: "~i~e", "~~emos";
        present_subjunctive_second: "~i~es", "~~éis";
        present_subjunctive_third: "~i~e", "~i~en";
        imperfect_subjunctive_first: "~~ara", "~~áramos";
        imperfect_subjunctive_second: "~~aras", "~~arais";
        imperfect_subjunctive_third: "~~ara", "~~aran";
        imperfect_subjunctive_se_first: "~~ase", "~~ásemos";
        imperfect_subjunctive_se_second: "~~ases", "~~aseis";
        imperfect_subjunctive_se_third: "~~ase", "~~asen";
        future_subjunctive_first: "~~are", "~~áremos";
        future_subjunctive_second: "~~ares", "~~areis";
        future_subjunctive_third: "~~are", "~~aren";
        vos: "~~ás", "~~és", "~~á";
        imperative: "~i~a", "~i~e", "~~emos", "~~ad", "~i~en";
        negative_imperative: "~i~es", "~i~e", "~~emos", "~~éis", "~i~en";
        infinitive: "~~ar";
        gerund: "~~ando";
        participle_masculine: "~~ado", "~~ados";
        participle_feminine: "~~ada", "~~adas";
    ),
    // Pliar: 67 words (aviar, confiar, vaciar, rociar, desafiar, culiar).
    conjugation!("pliar", 67, "~iar";
        present_first: "~ío", "~iamos";
        present_second: "~ías", "~iáis";
        present_third: "~ía", "~ían";
        imperfect_first: "~iaba", "~iábamos";
        imperfect_second: "~iabas", "~iabais";
        imperfect_third: "~iaba", "~iaban";
        preterite_first: "~ié", "~iamos";
        preterite_second: "~iaste", "~iasteis";
        preterite_third: "~ió", "~iaron";
        future_first: "~iaré", "~iaremos";
        future_second: "~iarás", "~iaréis";
        future_third: "~iará", "~iarán";
        conditional_first: "~iaría", "~iaríamos";
        conditional_second: "~iarías", "~iaríais";
        conditional_third: "~iaría", "~iarían";
        present_subjunctive_first: "~íe", "~iemos";
        present_subjunctive_second: "~íes", "~iéis";
        present_subjunctive_third: "~íe", "~íen";
        imperfect_subjunctive_first: "~iara", "~iáramos";
        imperfect_subjunctive_second: "~iaras", "~iarais";
        imperfect_subjunctive_third: "~iara", "~iaran";
        imperfect_subjunctive_se_first: "~iase", "~iásemos";
        imperfect_subjunctive_se_second: "~iases", "~iaseis";
        imperfect_subjunctive_se_third: "~iase", "~iasen";
        future_subjunctive_first: "~iare", "~iáremos";
        future_subjunctive_second: "~iares", "~iareis";
        future_subjunctive_third: "~iare", "~iaren";
        vos: "~iás", "~iés", "~iá";
        imperative: "~ía", "~íe", "~iemos", "~iad", "~íen";
        negative_imperative: "~íes", "~íe", "~iemos", "~iéis", "~íen";
        infinitive: "~iar";
        gerund: "~iando";
        participle_masculine: "~iado", "~iados";
        participle_feminine: "~iada", "~iadas";
    ),
    // Earse: 64 words (echarse, darse, ducharse, faracharse, fincharse, earse).
    conjugation!("earse", 64, "~arse";
        present_first: "~o", "~amos";
        present_second: "~as", "~áis";
        present_third: "~a", "~an";
        imperfect_first: "~aba", "~ábamos";
        imperfect_second: "~abas", "~abais";
        imperfect_third: "~aba", "~aban";
        preterite_first: "~é", "~amos";
        preterite_second: "~aste", "~asteis";
        preterite_third: "~ó", "~aron";
        future_first: "~aré", "~aremos";
        future_second: "~arás", "~aréis";
        future_third: "~ará", "~arán";
        conditional_first: "~aría", "~aríamos";
        conditional_second: "~arías", "~aríais";
        conditional_third: "~aría", "~arían";
        present_subjunctive_first: "~e", "~emos";
        present_subjunctive_second: "~es", "~éis";
        present_subjunctive_third: "~e", "~en";
        imperfect_subjunctive_first: "~ara", "~áramos";
        imperfect_subjunctive_second: "~aras", "~arais";
        imperfect_subjunctive_third: "~ara", "~aran";
        imperfect_subjunctive_se_first: "~ase", "~ásemos";
        imperfect_subjunctive_se_second: "~ases", "~aseis";
        imperfect_subjunctive_se_third: "~ase", "~asen";
        future_subjunctive_first: "~are", "~áremos";
        future_subjunctive_second: "~ares", "~areis";
        future_subjunctive_third: "~are", "~aren";
        vos: "~ás", "~és", "~áte";
        imperative: "~ate", "~ese", "~émonos", "~aos", "~ense";
        negative_imperative: "~es", "~e", "~emos", "~éis", "~en";
        infinitive: "~arse";
        gerund: "~ándose";
        participle_masculine: "~ado", "~ados";
        participle_feminine: "~ada", "~adas";
    ),
    // Volar: 54 words (sonar, soltar, encontrar, volar, solar, acostar).
    conjugation!("volar", 54, "~o~ar";
        present_first: "~ue~o", "~o~amos";
        present_second: "~ue~as", "~o~áis";
        present_third: "~ue~a", "~ue~an";
        imperfect_first: "~o~aba", "~o~ábamos";
        imperfect_second: "~o~abas", "~o~abais";
        imperfect_third: "~o~aba", "~o~aban";
        preterite_first: "~o~é", "~o~amos";
        preterite_second: "~o~aste", "~o~asteis";
        preterite_third: "~o~ó", "~o~aron";
        future_first: "~o~aré", "~o~aremos";
        future_second: "~o~arás", "~o~aréis";
        future_third: "~o~ará", "~o~arán";
        conditional_first: "~o~aría", "~o~aríamos";
        conditional_second: "~o~arías", "~o~aríais";
        conditional_third: "~o~aría", "~o~arían";
        present_subjunctive_first: "~ue~e", "~o~emos";
        present_subjunctive_second: "~ue~es", "~o~éis";
        present_subjunctive_third: "~ue~e", "~ue~en";
        imperfect_subjunctive_first: "~o~ara", "~o~áramos";
        imperfect_subjunctive_second: "~o~aras", "~o~arais";
        imperfect_subjunctive_third: "~o~ara", "~o~aran";
        imperfect_subjunctive_se_first: "~o~ase", "~o~ásemos";
        imperfect_subjunctive_se_second: "~o~ases", "~o~aseis";
        imperfect_subjunctive_se_third: "~o~ase", "~o~asen";
        future_subjunctive_first: "~o~are", "~o~áremos";
        future_subjunctive_second: "~o~ares", "~o~areis";
        future_subjunctive_third: "~o~are", "~o~aren";
        vos: "~o~ás", "~o~és", "~o~á";
        imperative: "~ue~a", "~ue~e", "~o~emos", "~o~ad", "~ue~en";
        negative_imperative: "~ue~es", "~ue~e", "~o~emos", "~o~éis", "~ue~en";
        infinitive: "~o~ar";
        gerund: "~o~ando";
        participle_masculine: "~o~ado", "~o~ados";
        participle_feminine: "~o~ada", "~o~adas";
    ),
    // Adrarse: 45 words (abarse, armarse, asmarse, alarse, andarse, asparse).
    conjugation!("adrarse", 45, "a~arse";
        present_first: "a~o", "a~amos";
        present_second: "a~as", "a~áis";
        present_third: "a~a", "a~an";
        imperfect_first: "a~aba", "a~ábamos";
        imperfect_second: "a~abas", "a~abais";
        imperfect_third: "a~aba", "a~aban";
        preterite_first: "a~é", "a~amos";
        preterite_second: "a~aste", "a~asteis";
        preterite_third: "a~ó", "a~aron";
        future_first: "a~aré", "a~aremos";
        future_second: "a~arás", "a~aréis";
        future_third: "a~ará", "a~arán";
        conditional_first: "a~aría", "a~aríamos";
        conditional_second: "a~arías", "a~aríais";
        conditional_third: "a~aría", "a~arían";
        present_subjunctive_first: "a~e", "a~emos";
        present_subjunctive_second: "a~es", "a~éis";
        present_subjunctive_third: "a~e", "a~en";
        imperfect_subjunctive_first: "a~ara", "a~áramos";
        imperfect_subjunctive_second: "a~aras", "a~arais";
        imperfect_subjunctive_third: "a~ara", "a~aran";
        imperfect_subjunctive_se_first: "a~ase", "a~ásemos";
        imperfect_subjunctive_se_second: "a~ases", "a~aseis";
        imperfect_subjunctive_se_third: "a~ase", "a~asen";
        future_subjunctive_first: "a~are", "a~áremos";
        future_subjunctive_second: "a~ares", "a~areis";
        future_subjunctive_third: "a~are", "a~aren";
        vos: "a~ás", "a~és", "a~ate";
        imperative: "á~ate", "á~ese", "a~émonos", "a~aos", "á~ense";
        negative_imperative: "a~es", "a~e", "a~emos", "a~éis", "a~en";
        infinitive: "a~arse";
        gerund: "a~ándose";
        participle_masculine: "a~ado", "a~ados";
        participle_feminine: "a~ada", "a~adas";
    ),
    // Situar: 44 words (puntuar, efectuar, situar, evaluar, ruar, tumultuar).
    conjugation!("situar", 44, "~uar";
        present_first: "~úo", "~uamos";
        present_second: "~úas", "~uáis";
        present_third: "~úa", "~úan";
        imperfect_first: "~uaba", "~uábamos";
        imperfect_second: "~uabas", "~uabais";
        imperfect_third: "~uaba", "~uaban";
        preterite_first: "~ué", "~uamos";
        preterite_second: "~uaste", "~uasteis";
        preterite_third: "~uó", "~uaron";
        future_first: "~uaré", "~uaremos";
        future_second: "~uarás", "~uaréis";
        future_third: "~uará", "~uarán";
        conditional_first: "~uaría", "~uaríamos";
        conditional_second: "~uarías", "~uaríais";
        conditional_third: "~uaría", "~uarían";
        present_subjunctive_first: "~úe", "~uemos";
        present_subjunctive_second: "~úes", "~uéis";
        present_subjunctive_third: "~úe", "~úen";
        imperfect_subjunctive_first: "~uara", "~uáramos";
        imperfect_subjunctive_second: "~uaras", "~uarais";
        imperfect_subjunctive_third: "~uara", "~uaran";
        imperfect_subjunctive_se_first: "~uase", "~uásemos";
        imperfect_subjunctive_se_second: "~uases", "~uaseis";
        imperfect_subjunctive_se_third: "~uase", "~uasen";
        future_subjunctive_first: "~uare", "~uáremos";
        future_subjunctive_second: "~uares", "~uareis";
        future_subjunctive_third: "~uare", "~uaren";
        vos: "~uás", "~ués", "~uá";
        imperative: "~úa", "~úe", "~uemos", "~uad", "~úen";
        negative_imperative: "~úes", "~úe", "~uemos", "~uéis", "~úen";
        infinitive: "~uar";
        gerund: "~uando";
        participle_masculine: "~uado", "~uados";
        participle_feminine: "~uada", "~uadas";
    ),
    // Ularse: 44 words (usarse, untarse, utarse, ularse, urrarse, umarse).
    conjugation!("ularse", 44, "u~arse";
        present_first: "u~o", "u~amos";
        present_second: "u~as", "u~áis";
        present_third: "u~a", "u~an";
        imperfect_first: "u~aba", "u~ábamos";
        imperfect_second: "u~abas", "u~abais";
        imperfect_third: "u~aba", "u~aban";
        preterite_first: "u~é", "u~amos";
        preterite_second: "u~aste", "u~asteis";
        preterite_third: "u~ó", "u~aron";
        future_first: "u~aré", "u~aremos";
        future_second: "u~arás", "u~aréis";
        future_third: "u~ará", "u~arán";
        conditional_first: "u~aría", "u~aríamos";
        conditional_second: "u~arías", "u~aríais";
        conditional_third: "u~aría", "u~arían";
        present_subjunctive_first: "u~e", "u~emos";
        present_subjunctive_second: "u~es", "u~éis";
        present_subjunctive_third: "u~e", "u~en";
        imperfect_subjunctive_first: "u~ara", "u~áramos";
        imperfect_subjunctive_second: "u~aras", "u~arais";
        imperfect_subjunctive_third: "u~ara", "u~aran";
        imperfect_subjunctive_se_first: "u~ase", "u~ásemos";
        imperfect_subjunctive_se_second: "u~ases", "u~aseis";
        imperfect_subjunctive_se_third: "u~ase", "u~asen";
        future_subjunctive_first: "u~are", "u~áremos";
        future_subjunctive_second: "u~ares", "u~areis";
        future_subjunctive_third: "u~are", "u~aren";
        vos: "u~ás", "u~és", "u~ate";
        imperative: "ú~ate", "ú~ese", "u~émonos", "u~aos", "ú~ense";
        negative_imperative: "u~es", "u~e", "u~emos", "u~éis", "u~en";
        infinitive: "u~arse";
        gerund: "u~ándose";
        participle_masculine: "u~ado", "u~ados";
        participle_feminine: "u~ada", "u~adas";
    ),
    // Onarse: 43 words (ociarse, ornarse, onarse, orciarse, ollarse, orarse).
    conjugation!("onarse", 43, "o~arse";
        present_first: "o~o", "o~amos";
        present_second: "o~as", "o~áis";
        present_third: "o~a", "o~an";
        imperfect_first: "o~aba", "o~ábamos";
        imperfect_second: "o~abas", "o~abais";
        imperfect_third: "o~aba", "o~aban";
        preterite_first: "o~é", "o~amos";
        preterite_second: "o~aste", "o~asteis";
        preterite_third: "o~ó", "o~aron";
        future_first: "o~aré", "o~aremos";
        future_second: "o~arás", "o~aréis";
        future_third: "o~ará", "o~arán";
        conditional_first: "o~aría", "o~aríamos";
        conditional_second: "o~arías", "o~aríais";
        conditional_third: "o~aría", "o~arían";
        present_subjunctive_first: "o~e", "o~emos";
        present_subjunctive_second: "o~es", "o~éis";
        present_subjunctive_third: "o~e", "o~en";
        imperfect_subjunctive_first: "o~ara", "o~áramos";
        imperfect_subjunctive_second: "o~aras", "o~arais";
        imperfect_subjunctive_third: "o~ara", "o~aran";
        imperfect_subjunctive_se_first: "o~ase", "o~ásemos";
        imperfect_subjunctive_se_second: "o~ases", "o~aseis";
        imperfect_subjunctive_se_third: "o~ase", "o~asen";
        future_subjunctive_first: "o~are", "o~áremos";
        future_subjunctive_second: "o~ares", "o~areis";
        future_subjunctive_third: "o~are", "o~aren";
        vos: "o~ás", "o~és", "o~ate";
        imperative: "ó~ate", "ó~ese", "o~émonos", "o~aos", "ó~ense";
        negative_imperative: "o~es", "o~e", "o~emos", "o~éis", "o~en";
        infinitive: "o~arse";
        gerund: "o~ándose";
        participle_masculine: "o~ado", "o~ados";
        participle_feminine: "o~ada", "o~adas";
    ),
    // Enagarse: 40 words (apangarse, cargarse, largarse, encargarse, pagarse, rasgarse).
    conjugation!("enagarse", 40, "~a~arse";
        present_first: "~a~o", "~a~amos";
        present_second: "~a~as", "~a~áis";
        present_third: "~a~a", "~a~an";
        imperfect_first: "~a~aba", "~a~ábamos";
        imperfect_second: "~a~abas", "~a~abais";
        imperfect_third: "~a~aba", "~a~aban";
        preterite_first: "~a~ué", "~a~amos";
        preterite_second: "~a~aste", "~a~asteis";
        preterite_third: "~a~ó", "~a~aron";
        future_first: "~a~aré", "~a~aremos";
        future_second: "~a~arás", "~a~aréis";
        future_third: "~a~ará", "~a~arán";
        conditional_first: "~a~aría", "~a~aríamos";
        conditional_second: "~a~arías", "~a~aríais";
        conditional_third: "~a~aría", "~a~arían";
        present_subjunctive_first: "~a~ue", "~a~uemos";
        present_subjunctive_second: "~a~ues", "~a~uéis";
        present_subjunctive_third: "~a~ue", "~a~uen";
        imperfect_subjunctive_first: "~a~ara", "~a~áramos";
        imperfect_subjunctive_second: "~a~aras", "~a~arais";
        imperfect_subjunctive_third: "~a~ara", "~a~aran";
        imperfect_subjunctive_se_first: "~a~ase", "~a~ásemos";
        imperfect_subjunctive_se_second: "~a~ases", "~a~aseis";
        imperfect_subjunctive_se_third: "~a~ase", "~a~asen";
        future_subjunctive_first: "~a~are", "~a~áremos";
        future_subjunctive_second: "~a~ares", "~a~areis";
        future_subjunctive_third: "~a~are", "~a~aren";
        vos: "~a~ás", "~a~ués", "~a~ate";
        imperative: "~á~ate", "~á~uese", "~a~uémonos", "~a~aos", "~á~uense";
        negative_imperative: "~a~ues", "~a~ue", "~a~uemos", "~a~uéis", "~a~uen";
        infinitive: "~a~arse";
        gerund: "~a~ándose";
        participle_masculine: "~a~ado", "~a~ados";
        participle_feminine: "~a~ada", "~a~adas";
    ),
    // Ecarse: 40 words (entrarse, ecarse, entarse, evarse, egiarse, erarse).
    conjugation!("ecarse", 40, "e~arse";
        present_first: "e~o", "e~amos";
        present_second: "e~as", "e~áis";
        present_third: "e~a", "e~an";
        imperfect_first: "e~aba", "e~ábamos";
        imperfect_second: "e~abas", "e~abais";
        imperfect_third: "e~aba", "e~aban";
        preterite_first: "e~é", "e~amos";
        preterite_second: "e~aste", "e~asteis";
        preterite_third: "e~ó", "e~aron";
        future_first: "e~aré", "e~aremos";
        future_second: "e~arás", "e~aréis";
        future_third: "e~ará", "e~arán";
        conditional_first: "e~aría", "e~aríamos";
        conditional_second: "e~arías", "e~aríais";
        conditional_third: "e~aría", "e~arían";
        present_subjunctive_first: "e~e", "e~emos";
        present_subjunctive_second: "e~es", "e~éis";
        present_subjunctive_third: "e~e", "e~en";
        imperfect_subjunctive_first: "e~ara", "e~áramos";
        imperfect_subjunctive_second: "e~aras", "e~arais";
        imperfect_subjunctive_third: "e~ara", "e~aran";
        imperfect_subjunctive_se_first: "e~ase", "e~ásemos";
        imperfect_subjunctive_se_second: "e~ases", "e~aseis";
        imperfect_subjunctive_se_third: "e~ase", "e~asen";
        future_subjunctive_first: "e~are", "e~áremos";
        future_subjunctive_second: "e~ares", "e~areis";
        future_subjunctive_third: "e~are", "e~aren";
        vos: "e~ás", "e~és", "e~ate";
        imperative: "é~ate", "é~ese", "e~émonos", "e~aos", "é~ense";
        negative_imperative: "e~es", "e~e", "e~emos", "e~éis", "e~en";
        infinitive: "e~arse";
        gerund: "e~ándose";
        participle_masculine: "e~ado", "e~ados";
        participle_feminine: "e~ada", "e~adas";
    ),
    // Struir: 38 words (excluir, rehuir, influir, diluir, confluir, recluir).
    conjugation!("struir", 38, "~ir";
        present_first: "~yo", "~imos";
        present_second: "~yes", "~ís";
        present_third: "~ye", "~yen";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~yó", "~yeron";
        future_first: "~iré", "~iremos";
        future_second: "~irás", "~iréis";
        future_third: "~irá", "~irán";
        conditional_first: "~iría", "~iríamos";
        conditional_second: "~irías", "~iríais";
        conditional_third: "~iría", "~irían";
        present_subjunctive_first: "~ya", "~yamos";
        present_subjunctive_second: "~yas", "~yáis";
        present_subjunctive_third: "~ya", "~yan";
        imperfect_subjunctive_first: "~yera", "~yéramos";
        imperfect_subjunctive_second: "~yeras", "~yerais";
        imperfect_subjunctive_third: "~yera", "~yeran";
        imperfect_subjunctive_se_first: "~yese", "~yésemos";
        imperfect_subjunctive_se_second: "~yeses", "~yeseis";
        imperfect_subjunctive_se_third: "~yese", "~yesen";
        future_subjunctive_first: "~yere", "~yéremos";
        future_subjunctive_second: "~yeres", "~yereis";
        future_subjunctive_third: "~yere", "~yeren";
        vos: "~ís", "~yás", "~í";
        imperative: "~ye", "~ya", "~yamos", "~id", "~yan";
        negative_imperative: "~yas", "~ya", "~yamos", "~yáis", "~yan";
        infinitive: "~ir";
        gerund: "~yendo";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Ravesarse: 37 words (quebrarse, sentarse, gobernarse, helarse, aventarse, cerrarse).
    conjugation!("ravesarse", 37, "~e~arse";
        present_first: "~ie~o", "~e~amos";
        present_second: "~ie~as", "~e~áis";
        present_third: "~ie~a", "~ie~an";
        imperfect_first: "~e~aba", "~e~ábamos";
        imperfect_second: "~e~abas", "~e~abais";
        imperfect_third: "~e~aba", "~e~aban";
        preterite_first: "~e~é", "~e~amos";
        preterite_second: "~e~aste", "~e~asteis";
        preterite_third: "~e~ó", "~e~aron";
        future_first: "~e~aré", "~e~aremos";
        future_second: "~e~arás", "~e~aréis";
        future_third: "~e~ará", "~e~arán";
        conditional_first: "~e~aría", "~e~aríamos";
        conditional_second: "~e~arías", "~e~aríais";
        conditional_third: "~e~aría", "~e~arían";
        present_subjunctive_first: "~ie~e", "~e~emos";
        present_subjunctive_second: "~ie~es", "~e~éis";
        present_subjunctive_third: "~ie~e", "~ie~en";
        imperfect_subjunctive_first: "~e~ara", "~e~áramos";
        imperfect_subjunctive_second: "~e~aras", "~e~arais";
        imperfect_subjunctive_third: "~e~ara", "~e~aran";
        imperfect_subjunctive_se_first: "~e~ase", "~e~ásemos";
        imperfect_subjunctive_se_second: "~e~ases", "~e~aseis";
        imperfect_subjunctive_se_third: "~e~ase", "~e~asen";
        future_subjunctive_first: "~e~are", "~e~áremos";
        future_subjunctive_second: "~e~ares", "~e~areis";
        future_subjunctive_third: "~e~are", "~e~aren";
        vos: "~e~ás", "~e~és", "~e~ate";
        imperative: "~ié~ate", "~ié~ese", "~e~émonos", "~e~aos", "~ié~ense";
        negative_imperative: "~ie~es", "~ie~e", "~e~emos", "~e~éis", "~ie~en";
        infinitive: "~e~arse";
        gerund: "~e~ándose";
        participle_masculine: "~e~ado", "~e~ados";
        participle_feminine: "~e~ada", "~e~adas";
    ),
    // Icharse: 33 words (inflarse, iciarse, igarse, iparse, icharse, ilarse).
    conjugation!("icharse", 33, "i~arse";
        present_first: "i~o", "i~amos";
        present_second: "i~as", "i~áis";
        present_third: "i~a", "i~an";
        imperfect_first: "i~aba", "i~ábamos";
        imperfect_second: "i~abas", "i~abais";
        imperfect_third: "i~aba", "i~aban";
        preterite_first: "i~é", "i~amos";
        preterite_second: "i~aste", "i~asteis";
        preterite_third: "i~ó", "i~aron";
        future_first: "i~aré", "i~aremos";
        future_second: "i~arás", "i~aréis";
        future_third: "i~ará", "i~arán";
        conditional_first: "i~aría", "i~aríamos";
        conditional_second: "i~arías", "i~aríais";
        conditional_third: "i~aría", "i~arían";
        present_subjunctive_first: "i~e", "i~emos";
        present_subjunctive_second: "i~es", "i~éis";
        present_subjunctive_third: "i~e", "i~en";
        imperfect_subjunctive_first: "i~ara", "i~áramos";
        imperfect_subjunctive_second: "i~aras", "i~arais";
        imperfect_subjunctive_third: "i~ara", "i~aran";
        imperfect_subjunctive_se_first: "i~ase", "i~ásemos";
        imperfect_subjunctive_se_second: "i~ases", "i~aseis";
        imperfect_subjunctive_se_third: "i~ase", "i~asen";
        future_subjunctive_first: "i~are", "i~áremos";
        future_subjunctive_second: "i~ares", "i~areis";
        future_subjunctive_third: "i~are", "i~aren";
        vos: "i~ás", "i~és", "i~ate";
        imperative: "í~ate", "í~ese", "i~émonos", "i~aos", "í~ense";
        negative_imperative: "i~es", "i~e", "i~emos", "i~éis", "i~en";
        infinitive: "i~arse";
        gerund: "i~ándose";
        participle_masculine: "i~ado", "i~ados";
        participle_feminine: "i~ada", "i~adas";
    ),
    // Jerir: 30 words (mentir, sentir, herir, hervir, preferir, referir).
    conjugation!("jerir", 30, "~e~ir";
        present_first: "~ie~o", "~e~imos";
        present_second: "~ie~es", "~e~ís";
        present_third: "~ie~e", "~ie~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~e~í", "~e~imos";
        preterite_second: "~e~iste", "~e~isteis";
        preterite_third: "~i~ió", "~i~ieron";
        future_first: "~e~iré", "~e~iremos";
        future_second: "~e~irás", "~e~iréis";
        future_third: "~e~irá", "~e~irán";
        conditional_first: "~e~iría", "~e~iríamos";
        conditional_second: "~e~irías", "~e~iríais";
        conditional_third: "~e~iría", "~e~irían";
        present_subjunctive_first: "~ie~a", "~i~amos";
        present_subjunctive_second: "~ie~as", "~i~áis";
        present_subjunctive_third: "~ie~a", "~ie~an";
        imperfect_subjunctive_first: "~i~iera", "~i~iéramos";
        imperfect_subjunctive_second: "~i~ieras", "~i~ierais";
        imperfect_subjunctive_third: "~i~iera", "~i~ieran";
        imperfect_subjunctive_se_first: "~i~iese", "~i~iésemos";
        imperfect_subjunctive_se_second: "~i~ieses", "~i~ieseis";
        imperfect_subjunctive_se_third: "~i~iese", "~i~iesen";
        future_subjunctive_first: "~i~iere", "~i~iéremos";
        future_subjunctive_second: "~i~ieres", "~i~iereis";
        future_subjunctive_third: "~i~iere", "~i~ieren";
        vos: "~e~ís", "~i~ás", "~e~í";
        imperative: "~ie~e", "~ie~a", "~i~amos", "~e~id", "~ie~an";
        negative_imperative: "~ie~as", "~ie~a", "~i~amos", "~i~áis", "~ie~an";
        infinitive: "~e~ir";
        gerund: "~i~iendo";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Musirse: 29 words (musirse, eludirse, pulirse, reunirse, tupirse, subirse).
    conjugation!("musirse", 29, "~u~irse";
        present_first: "~u~o", "~u~imos";
        present_second: "~u~es", "~u~ís";
        present_third: "~u~e", "~u~en";
        imperfect_first: "~u~ía", "~u~íamos";
        imperfect_second: "~u~ías", "~u~íais";
        imperfect_third: "~u~ía", "~u~ían";
        preterite_first: "~u~í", "~u~imos";
        preterite_second: "~u~iste", "~u~isteis";
        preterite_third: "~u~ió", "~u~ieron";
        future_first: "~u~iré", "~u~iremos";
        future_second: "~u~irás", "~u~iréis";
        future_third: "~u~irá", "~u~irán";
        conditional_first: "~u~iría", "~u~iríamos";
        conditional_second: "~u~irías", "~u~iríais";
        conditional_third: "~u~iría", "~u~irían";
        present_subjunctive_first: "~u~a", "~u~amos";
        present_subjunctive_second: "~u~as", "~u~áis";
        present_subjunctive_third: "~u~a", "~u~an";
        imperfect_subjunctive_first: "~u~iera", "~u~iéramos";
        imperfect_subjunctive_second: "~u~ieras", "~u~ierais";
        imperfect_subjunctive_third: "~u~iera", "~u~ieran";
        imperfect_subjunctive_se_first: "~u~iese", "~u~iésemos";
        imperfect_subjunctive_se_second: "~u~ieses", "~u~ieseis";
        imperfect_subjunctive_se_third: "~u~iese", "~u~iesen";
        future_subjunctive_first: "~u~iere", "~u~iéremos";
        future_subjunctive_second: "~u~ieres", "~u~iereis";
        future_subjunctive_third: "~u~iere", "~u~ieren";
        vos: "~u~ís", "~u~ás", "~u~ite";
        imperative: "~ú~ete", "~ú~ase", "~u~ámonos", "~u~íos", "~ú~anse";
        negative_imperative: "~u~as", "~u~a", "~u~amos", "~u~áis", "~u~an";
        infinitive: "~u~irse";
        gerund: "~u~iéndose";
        participle_masculine: "~u~ido", "~u~idos";
        participle_feminine: "~u~ida", "~u~idas";
    ),
    // Rigir: 27 words (urgir, fingir, agir, rugir, erigir, fugir).
    conjugation!("rigir", 27, "~gir";
        present_first: "~jo", "~gimos";
        present_second: "~ges", "~gís";
        present_third: "~ge", "~gen";
        imperfect_first: "~gía", "~gíamos";
        imperfect_second: "~gías", "~gíais";
        imperfect_third: "~gía", "~gían";
        preterite_first: "~gí", "~gimos";
        preterite_second: "~giste", "~gisteis";
        preterite_third: "~gió", "~gieron";
        future_first: "~giré", "~giremos";
        future_second: "~girás", "~giréis";
        future_third: "~girá", "~girán";
        conditional_first: "~giría", "~giríamos";
        conditional_second: "~girías", "~giríais";
        conditional_third: "~giría", "~girían";
        present_subjunctive_first: "~ja", "~jamos";
        present_subjunctive_second: "~jas", "~jáis";
        present_subjunctive_third: "~ja", "~jan";
        imperfect_subjunctive_first: "~giera", "~giéramos";
        imperfect_subjunctive_second: "~gieras", "~gierais";
        imperfect_subjunctive_third: "~giera", "~gieran";
        imperfect_subjunctive_se_first: "~giese", "~giésemos";
        imperfect_subjunctive_se_second: "~gieses", "~gieseis";
        imperfect_subjunctive_se_third: "~giese", "~giesen";
        future_subjunctive_first: "~giere", "~giéremos";
        future_subjunctive_second: "~gieres", "~giereis";
        future_subjunctive_third: "~giere", "~gieren";
        vos: "~gís", "~jás", "~gí";
        imperative: "~ge", "~ja", "~jamos", "~gid", "~jan";
        negative_imperative: "~jas", "~ja", "~jamos", "~jáis", "~jan";
        infinitive: "~gir";
        gerund: "~giendo";
        participle_masculine: "~gido", "~gidos";
        participle_feminine: "~gida", "~gidas";
    ),
    // Omingarse: 26 words (chingarse, fatigarse, ligarse, aligarse, fisgarse, avejigarse).
    conjugation!("omingarse", 26, "~i~arse";
        present_first: "~i~o", "~i~amos";
        present_second: "~i~as", "~i~áis";
        present_third: "~i~a", "~i~an";
        imperfect_first: "~i~aba", "~i~ábamos";
        imperfect_second: "~i~abas", "~i~abais";
        imperfect_third: "~i~aba", "~i~aban";
        preterite_first: "~i~ué", "~i~amos";
        preterite_second: "~i~aste", "~i~asteis";
        preterite_third: "~i~ó", "~i~aron";
        future_first: "~i~aré", "~i~aremos";
        future_second: "~i~arás", "~i~aréis";
        future_third: "~i~ará", "~i~arán";
        conditional_first: "~i~aría", "~i~aríamos";
        conditional_second: "~i~arías", "~i~aríais";
        conditional_third: "~i~aría", "~i~arían";
        present_subjunctive_first: "~i~ue", "~i~uemos";
        present_subjunctive_second: "~i~ues", "~i~uéis";
        present_subjunctive_third: "~i~ue", "~i~uen";
        imperfect_subjunctive_first: "~i~ara", "~i~áramos";
        imperfect_subjunctive_second: "~i~aras", "~i~arais";
        imperfect_subjunctive_third: "~i~ara", "~i~aran";
        imperfect_subjunctive_se_first: "~i~ase", "~i~ásemos";
        imperfect_subjunctive_se_second: "~i~ases", "~i~aseis";
        imperfect_subjunctive_se_third: "~i~ase", "~i~asen";
        future_subjunctive_first: "~i~are", "~i~áremos";
        future_subjunctive_second: "~i~ares", "~i~areis";
        future_subjunctive_third: "~i~are", "~i~aren";
        vos: "~i~ás", "~i~ués", "~i~ate";
        imperative: "~í~ate", "~í~uese", "~i~uémonos", "~i~aos", "~í~uense";
        negative_imperative: "~i~ues", "~i~ue", "~i~uemos", "~i~uéis", "~i~uen";
        infinitive: "~i~arse";
        gerund: "~i~ándose";
        participle_masculine: "~i~ado", "~i~ados";
        participle_feminine: "~i~ada", "~i~adas";
    ),
    // Trancarse: 26 words (trancarse, atascarse, bancarse, rascarse, cascarse, embarcarse).
    conjugation!("trancarse", 26, "~a~carse";
        present_first: "~a~co", "~a~camos";
        present_second: "~a~cas", "~a~cáis";
        present_third: "~a~ca", "~a~can";
        imperfect_first: "~a~caba", "~a~cábamos";
        imperfect_second: "~a~cabas", "~a~cabais";
        imperfect_third: "~a~caba", "~a~caban";
        preterite_first: "~a~qué", "~a~camos";
        preterite_second: "~a~caste", "~a~casteis";
        preterite_third: "~a~có", "~a~caron";
        future_first: "~a~caré", "~a~caremos";
        future_second: "~a~carás", "~a~caréis";
        future_third: "~a~cará", "~a~carán";
        conditional_first: "~a~caría", "~a~caríamos";
        conditional_second: "~a~carías", "~a~caríais";
        conditional_third: "~a~caría", "~a~carían";
        present_subjunctive_first: "~a~que", "~a~quemos";
        present_subjunctive_second: "~a~ques", "~a~quéis";
        present_subjunctive_third: "~a~que", "~a~quen";
        imperfect_subjunctive_first: "~a~cara", "~a~cáramos";
        imperfect_subjunctive_second: "~a~caras", "~a~carais";
        imperfect_subjunctive_third: "~a~cara", "~a~caran";
        imperfect_subjunctive_se_first: "~a~case", "~a~cásemos";
        imperfect_subjunctive_se_second: "~a~cases", "~a~caseis";
        imperfect_subjunctive_se_third: "~a~case", "~a~casen";
        future_subjunctive_first: "~a~care", "~a~cáremos";
        future_subjunctive_second: "~a~cares", "~a~careis";
        future_subjunctive_third: "~a~care", "~a~caren";
        vos: "~a~cás", "~a~qués", "~a~cate";
        imperative: "~á~cate", "~á~quese", "~a~quémonos", "~a~caos", "~á~quense";
        negative_imperative: "~a~ques", "~a~que", "~a~quemos", "~a~quéis", "~a~quen";
        infinitive: "~a~carse";
        gerund: "~a~cándose";
        participle_masculine: "~a~cado", "~a~cados";
        participle_feminine: "~a~cada", "~a~cadas";
    ),
    // Toserse: 26 words (correrse, coserse, dolerse, escocerse, volverse, acorrerse).
    conjugation!("toserse", 26, "~o~erse";
        present_first: "~o~o", "~o~emos";
        present_second: "~o~es", "~o~éis";
        present_third: "~o~e", "~o~en";
        imperfect_first: "~o~ía", "~o~íamos";
        imperfect_second: "~o~ías", "~o~íais";
        imperfect_third: "~o~ía", "~o~ían";
        preterite_first: "~o~í", "~o~imos";
        preterite_second: "~o~iste", "~o~isteis";
        preterite_third: "~o~ió", "~o~ieron";
        future_first: "~o~eré", "~o~eremos";
        future_second: "~o~erás", "~o~eréis";
        future_third: "~o~erá", "~o~erán";
        conditional_first: "~o~ería", "~o~eríamos";
        conditional_second: "~o~erías", "~o~eríais";
        conditional_third: "~o~ería", "~o~erían";
        present_subjunctive_first: "~o~a", "~o~amos";
        present_subjunctive_second: "~o~as", "~o~áis";
        present_subjunctive_third: "~o~a", "~o~an";
        imperfect_subjunctive_first: "~o~iera", "~o~iéramos";
        imperfect_subjunctive_second: "~o~ieras", "~o~ierais";
        imperfect_subjunctive_third: "~o~iera", "~o~ieran";
        imperfect_subjunctive_se_first: "~o~iese", "~o~iésemos";
        imperfect_subjunctive_se_second: "~o~ieses", "~o~ieseis";
        imperfect_subjunctive_se_third: "~o~iese", "~o~iesen";
        future_subjunctive_first: "~o~iere", "~o~iéremos";
        future_subjunctive_second: "~o~ieres", "~o~iereis";
        future_subjunctive_third: "~o~iere", "~o~ieren";
        vos: "~o~és", "~o~ás", "~o~ete";
        imperative: "~ó~ete", "~ó~ase", "~o~ámonos", "~o~eos", "~ó~anse";
        negative_imperative: "~o~as", "~o~a", "~o~amos", "~o~áis", "~o~an";
        infinitive: "~o~erse";
        gerund: "~o~iéndose";
        participle_masculine: "~o~ido", "~o~idos";
        participle_feminine: "~o~ida", "~o~idas";
    ),
    // Friarse: 25 words (confiarse, aliarse, averiarse, entrecriarse, aindiarse, ejecutoriarse).
    conjugation!("friarse", 25, "~iarse";
        present_first: "~ío", "~iamos";
        present_second: "~ías", "~iáis";
        present_third: "~ía", "~ían";
        imperfect_first: "~iaba", "~iábamos";
        imperfect_second: "~iabas", "~iabais";
        imperfect_third: "~iaba", "~iaban";
        preterite_first: "~ié", "~iamos";
        preterite_second: "~iaste", "~iasteis";
        preterite_third: "~ió", "~iaron";
        future_first: "~iaré", "~iaremos";
        future_second: "~iarás", "~iaréis";
        future_third: "~iará", "~iarán";
        conditional_first: "~iaría", "~iaríamos";
        conditional_second: "~iarías", "~iaríais";
        conditional_third: "~iaría", "~iarían";
        present_subjunctive_first: "~íe", "~iemos";
        present_subjunctive_second: "~íes", "~iéis";
        present_subjunctive_third: "~íe", "~íen";
        imperfect_subjunctive_first: "~iara", "~iáramos";
        imperfect_subjunctive_second: "~iaras", "~iarais";
        imperfect_subjunctive_third: "~iara", "~iaran";
        imperfect_subjunctive_se_first: "~iase", "~iásemos";
        imperfect_subjunctive_se_second: "~iases", "~iaseis";
        imperfect_subjunctive_se_third: "~iase", "~iasen";
        future_subjunctive_first: "~iare", "~iáremos";
        future_subjunctive_second: "~iares", "~iareis";
        future_subjunctive_third: "~iare", "~iaren";
        vos: "~iás", "~iés", "~iate";
        imperative: "~íate", "~íese", "~iémonos", "~iaos", "~íense";
        negative_imperative: "~íes", "~íe", "~iemos", "~iéis", "~íen";
        infinitive: "~iarse";
        gerund: "~iándose";
        participle_masculine: "~iado", "~iados";
        participle_feminine: "~iada", "~iadas";
    ),
    // Contrarse: 24 words (encontrarse, poblarse, encordarse, acostarse, acordarse, apostarse).
    conjugation!("contrarse", 24, "~o~arse";
        present_first: "~ue~o", "~o~amos";
        present_second: "~ue~as", "~o~áis";
        present_third: "~ue~a", "~ue~an";
        imperfect_first: "~o~aba", "~o~ábamos";
        imperfect_second: "~o~abas", "~o~abais";
        imperfect_third: "~o~aba", "~o~aban";
        preterite_first: "~o~é", "~o~amos";
        preterite_second: "~o~aste", "~o~asteis";
        preterite_third: "~o~ó", "~o~aron";
        future_first: "~o~aré", "~o~aremos";
        future_second: "~o~arás", "~o~aréis";
        future_third: "~o~ará", "~o~arán";
        conditional_first: "~o~aría", "~o~aríamos";
        conditional_second: "~o~arías", "~o~aríais";
        conditional_third: "~o~aría", "~o~arían";
        present_subjunctive_first: "~ue~e", "~o~emos";
        present_subjunctive_second: "~ue~es", "~o~éis";
        present_subjunctive_third: "~ue~e", "~ue~en";
        imperfect_subjunctive_first: "~o~ara", "~o~áramos";
        imperfect_subjunctive_second: "~o~aras", "~o~arais";
        imperfect_subjunctive_third: "~o~ara", "~o~aran";
        imperfect_subjunctive_se_first: "~o~ase", "~o~ásemos";
        imperfect_subjunctive_se_second: "~o~ases", "~o~aseis";
        imperfect_subjunctive_se_third: "~o~ase", "~o~asen";
        future_subjunctive_first: "~o~are", "~o~áremos";
        future_subjunctive_second: "~o~ares", "~o~areis";
        future_subjunctive_third: "~o~are", "~o~aren";
        vos: "~o~ás", "~o~és", "~o~ate";
        imperative: "~ué~ate", "~ué~ese", "~o~émonos", "~o~aos", "~ué~ense";
        negative_imperative: "~ue~es", "~ue~e", "~o~emos", "~o~éis", "~ue~en";
        infinitive: "~o~arse";
        gerund: "~o~ándose";
        participle_masculine: "~o~ado", "~o~ados";
        participle_feminine: "~o~ada", "~o~adas";
    ),
    // Poner: 23 words (poner, yuxtaponer, imponer, suponer, posponer, componer).
    conjugation!("poner", 23, "~oner";
        present_first: "~ongo", "~onemos";
        present_second: "~ones", "~onéis";
        present_third: "~one", "~onen";
        imperfect_first: "~onía", "~oníamos";
        imperfect_second: "~onías", "~oníais";
        imperfect_third: "~onía", "~onían";
        preterite_first: "~use", "~usimos";
        preterite_second: "~usiste", "~usisteis";
        preterite_third: "~uso", "~usieron";
        future_first: "~ondré", "~ondremos";
        future_second: "~ondrás", "~ondréis";
        future_third: "~ondrá", "~ondrán";
        conditional_first: "~ondría", "~ondríamos";
        conditional_second: "~ondrías", "~ondríais";
        conditional_third: "~ondría", "~ondrían";
        present_subjunctive_first: "~onga", "~ongamos";
        present_subjunctive_second: "~ongas", "~ongáis";
        present_subjunctive_third: "~onga", "~ongan";
        imperfect_subjunctive_first: "~usiera", "~usiéramos";
        imperfect_subjunctive_second: "~usieras", "~usierais";
        imperfect_subjunctive_third: "~usiera", "~usieran";
        imperfect_subjunctive_se_first: "~usiese", "~usiésemos";
        imperfect_subjunctive_se_second: "~usieses", "~usieseis";
        imperfect_subjunctive_se_third: "~usiese", "~usiesen";
        future_subjunctive_first: "~usiere", "~usiéremos";
        future_subjunctive_second: "~usieres", "~usiereis";
        future_subjunctive_third: "~usiere", "~usieren";
        vos: "~onés", "~ongás", "~oné";
        imperative: "~ón", "~onga", "~ongamos", "~oned", "~ongan";
        negative_imperative: "~ongas", "~onga", "~ongamos", "~ongáis", "~ongan";
        infinitive: "~oner";
        gerund: "~oniendo";
        participle_masculine: "~uesto", "~uestos";
        participle_feminine: "~uesta", "~uestas";
    ),
    // Inuarse: 23 words (situarse, evaluarse, efectuarse, ecuarse, enuarse, tenuarse).
    conjugation!("inuarse", 23, "~uarse";
        present_first: "~úo", "~uamos";
        present_second: "~úas", "~uáis";
        present_third: "~úa", "~úan";
        imperfect_first: "~uaba", "~uábamos";
        imperfect_second: "~uabas", "~uabais";
        imperfect_third: "~uaba", "~uaban";
        preterite_first: "~ué", "~uamos";
        preterite_second: "~uaste", "~uasteis";
        preterite_third: "~uó", "~uaron";
        future_first: "~uaré", "~uaremos";
        future_second: "~uarás", "~uaréis";
        future_third: "~uará", "~uarán";
        conditional_first: "~uaría", "~uaríamos";
        conditional_second: "~uarías", "~uaríais";
        conditional_third: "~uaría", "~uarían";
        present_subjunctive_first: "~úe", "~uemos";
        present_subjunctive_second: "~úes", "~uéis";
        present_subjunctive_third: "~úe", "~úen";
        imperfect_subjunctive_first: "~uara", "~uáramos";
        imperfect_subjunctive_second: "~uaras", "~uarais";
        imperfect_subjunctive_third: "~uara", "~uaran";
        imperfect_subjunctive_se_first: "~uase", "~uásemos";
        imperfect_subjunctive_se_second: "~uases", "~uaseis";
        imperfect_subjunctive_se_third: "~uase", "~uasen";
        future_subjunctive_first: "~uare", "~uáremos";
        future_subjunctive_second: "~uares", "~uareis";
        future_subjunctive_third: "~uare", "~uaren";
        vos: "~uás", "~ués", "~uate";
        imperative: "~úate", "~úese", "~uémonos", "~uaos", "~úense";
        negative_imperative: "~úes", "~úe", "~uemos", "~uéis", "~úen";
        infinitive: "~uarse";
        gerund: "~uándose";
        participle_masculine: "~uado", "~uados";
        participle_feminine: "~uada", "~uadas";
    ),
    // Llir: 22 words (fuñir, çabullir, tullir, fallir, puñir, uñir).
    conjugation!("llir", 22, "~ir";
        present_first: "~o", "~imos";
        present_second: "~es", "~ís";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~ó", "~eron";
        future_first: "~iré", "~iremos";
        future_second: "~irás", "~iréis";
        future_third: "~irá", "~irán";
        conditional_first: "~iría", "~iríamos";
        conditional_second: "~irías", "~iríais";
        conditional_third: "~iría", "~irían";
        present_subjunctive_first: "~a", "~amos";
        present_subjunctive_second: "~as", "~áis";
        present_subjunctive_third: "~a", "~an";
        imperfect_subjunctive_first: "~era", "~éramos";
        imperfect_subjunctive_second: "~eras", "~erais";
        imperfect_subjunctive_third: "~era", "~eran";
        imperfect_subjunctive_se_first: "~ese", "~ésemos";
        imperfect_subjunctive_se_second: "~eses", "~eseis";
        imperfect_subjunctive_se_third: "~ese", "~esen";
        future_subjunctive_first: "~ere", "~éremos";
        future_subjunctive_second: "~eres", "~ereis";
        future_subjunctive_third: "~ere", "~eren";
        vos: "~ís", "~ás", "~í";
        imperative: "~e", "~a", "~amos", "~id", "~an";
        negative_imperative: "~as", "~a", "~amos", "~áis", "~an";
        infinitive: "~ir";
        gerund: "~endo";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Venir: 21 words (devenir, venir, intervenir, avenir, revenir, convenir).
    conjugation!("venir", 21, "~e~ir";
        present_first: "~e~go", "~e~imos";
        present_second: "~ie~es", "~e~ís";
        present_third: "~ie~e", "~ie~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~i~e", "~i~imos";
        preterite_second: "~i~iste", "~i~isteis";
        preterite_third: "~i~o", "~i~ieron";
        future_first: "~e~dré", "~e~dremos";
        future_second: "~e~drás", "~e~dréis";
        future_third: "~e~drá", "~e~drán";
        conditional_first: "~e~dría", "~e~dríamos";
        conditional_second: "~e~drías", "~e~dríais";
        conditional_third: "~e~dría", "~e~drían";
        present_subjunctive_first: "~e~ga", "~e~gamos";
        present_subjunctive_second: "~e~gas", "~e~gáis";
        present_subjunctive_third: "~e~ga", "~e~gan";
        imperfect_subjunctive_first: "~i~iera", "~i~iéramos";
        imperfect_subjunctive_second: "~i~ieras", "~i~ierais";
        imperfect_subjunctive_third: "~i~iera", "~i~ieran";
        imperfect_subjunctive_se_first: "~i~iese", "~i~iésemos";
        imperfect_subjunctive_se_second: "~i~ieses", "~i~ieseis";
        imperfect_subjunctive_se_third: "~i~iese", "~i~iesen";
        future_subjunctive_first: "~i~iere", "~i~iéremos";
        future_subjunctive_second: "~i~ieres", "~i~iereis";
        future_subjunctive_third: "~i~iere", "~i~ieren";
        vos: "~e~ís", "~e~gás", "~e~í";
        imperative: "~é~", "~e~ga", "~e~gamos", "~e~id", "~e~gan";
        negative_imperative: "~e~gas", "~e~ga", "~e~gamos", "~e~gáis", "~e~gan";
        infinitive: "~e~ir";
        gerund: "~i~iendo";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Guar: 21 words (averiguar, testiguar, iguar, estiguar, nguar, ciguar).
    conjugation!("guar", 21, "~uar";
        present_first: "~uo", "~uamos";
        present_second: "~uas", "~uáis";
        present_third: "~ua", "~uan";
        imperfect_first: "~uaba", "~uábamos";
        imperfect_second: "~uabas", "~uabais";
        imperfect_third: "~uaba", "~uaban";
        preterite_first: "~üé", "~uamos";
        preterite_second: "~uaste", "~uasteis";
        preterite_third: "~uó", "~uaron";
        future_first: "~uaré", "~uaremos";
        future_second: "~uarás", "~uaréis";
        future_third: "~uará", "~uarán";
        conditional_first: "~uaría", "~uaríamos";
        conditional_second: "~uarías", "~uaríais";
        conditional_third: "~uaría", "~uarían";
        present_subjunctive_first: "~üe", "~üemos";
        present_subjunctive_second: "~ües", "~üéis";
        present_subjunctive_third: "~üe", "~üen";
        imperfect_subjunctive_first: "~uara", "~uáramos";
        imperfect_subjunctive_second: "~uaras", "~uarais";
        imperfect_subjunctive_third: "~uara", "~uaran";
        imperfect_subjunctive_se_first: "~uase", "~uásemos";
        imperfect_subjunctive_se_second: "~uases", "~uaseis";
        imperfect_subjunctive_se_third: "~uase", "~uasen";
        future_subjunctive_first: "~uare", "~uáremos";
        future_subjunctive_second: "~uares", "~uareis";
        future_subjunctive_third: "~uare", "~uaren";
        vos: "~uás", "~üés", "~uá";
        imperative: "~ua", "~üe", "~üemos", "~uad", "~üen";
        negative_imperative: "~ües", "~üe", "~üemos", "~üéis", "~üen";
        infinitive: "~uar";
        gerund: "~uando";
        participle_masculine: "~uado", "~uados";
        participle_feminine: "~uada", "~uadas";
    ),
    // Pegarse: 20 words (pegarse, aborregarse, entregarse, vengarse, doblegarse, embregarse).
    conjugation!("pegarse", 20, "~e~arse";
        present_first: "~e~o", "~e~amos";
        present_second: "~e~as", "~e~áis";
        present_third: "~e~a", "~e~an";
        imperfect_first: "~e~aba", "~e~ábamos";
        imperfect_second: "~e~abas", "~e~abais";
        imperfect_third: "~e~aba", "~e~aban";
        preterite_first: "~e~ué", "~e~amos";
        preterite_second: "~e~aste", "~e~asteis";
        preterite_third: "~e~ó", "~e~aron";
        future_first: "~e~aré", "~e~aremos";
        future_second: "~e~arás", "~e~aréis";
        future_third: "~e~ará", "~e~arán";
        conditional_first: "~e~aría", "~e~aríamos";
        conditional_second: "~e~arías", "~e~aríais";
        conditional_third: "~e~aría", "~e~arían";
        present_subjunctive_first: "~e~ue", "~e~uemos";
        present_subjunctive_second: "~e~ues", "~e~uéis";
        present_subjunctive_third: "~e~ue", "~e~uen";
        imperfect_subjunctive_first: "~e~ara", "~e~áramos";
        imperfect_subjunctive_second: "~e~aras", "~e~arais";
        imperfect_subjunctive_third: "~e~ara", "~e~aran";
        imperfect_subjunctive_se_first: "~e~ase", "~e~ásemos";
        imperfect_subjunctive_se_second: "~e~ases", "~e~aseis";
        imperfect_subjunctive_se_third: "~e~ase", "~e~asen";
        future_subjunctive_first: "~e~are", "~e~áremos";
        future_subjunctive_second: "~e~ares", "~e~areis";
        future_subjunctive_third: "~e~are", "~e~aren";
        vos: "~e~ás", "~e~ués", "~e~ate";
        imperative: "~é~ate", "~é~uese", "~e~uémonos", "~e~aos", "~é~uense";
        negative_imperative: "~e~ues", "~e~ue", "~e~uemos", "~e~uéis", "~e~uen";
        infinitive: "~e~arse";
        gerund: "~e~ándose";
        participle_masculine: "~e~ado", "~e~ados";
        participle_feminine: "~e~ada", "~e~adas";
    ),
    // Venderse: 19 words (prenderse, tejerse, venderse, atreverse, evanescerse, pederse).
    conjugation!("venderse", 19, "~e~erse";
        present_first: "~e~o", "~e~emos";
        present_second: "~e~es", "~e~éis";
        present_third: "~e~e", "~e~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~e~í", "~e~imos";
        preterite_second: "~e~iste", "~e~isteis";
        preterite_third: "~e~ió", "~e~ieron";
        future_first: "~e~eré", "~e~eremos";
        future_second: "~e~erás", "~e~eréis";
        future_third: "~e~erá", "~e~erán";
        conditional_first: "~e~ería", "~e~eríamos";
        conditional_second: "~e~erías", "~e~eríais";
        conditional_third: "~e~ería", "~e~erían";
        present_subjunctive_first: "~e~a", "~e~amos";
        present_subjunctive_second: "~e~as", "~e~áis";
        present_subjunctive_third: "~e~a", "~e~an";
        imperfect_subjunctive_first: "~e~iera", "~e~iéramos";
        imperfect_subjunctive_second: "~e~ieras", "~e~ierais";
        imperfect_subjunctive_third: "~e~iera", "~e~ieran";
        imperfect_subjunctive_se_first: "~e~iese", "~e~iésemos";
        imperfect_subjunctive_se_second: "~e~ieses", "~e~ieseis";
        imperfect_subjunctive_se_third: "~e~iese", "~e~iesen";
        future_subjunctive_first: "~e~iere", "~e~iéremos";
        future_subjunctive_second: "~e~ieres", "~e~iereis";
        future_subjunctive_third: "~e~iere", "~e~ieren";
        vos: "~e~és", "~e~ás", "~e~ete";
        imperative: "~é~ete", "~é~ase", "~e~ámonos", "~e~eos", "~é~anse";
        negative_imperative: "~e~as", "~e~a", "~e~amos", "~e~áis", "~e~an";
        infinitive: "~e~erse";
        gerund: "~e~iéndose";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Ujiar: 19 words (gilidiar, meliar, luquiar, mirujiar, jetiar, noveleriar).
    conjugation!("ujiar", 19, "~iar";
        present_first: "~eo", "~iamos";
        present_second: "~eas", "~iáis";
        present_third: "~ea", "~ean";
        imperfect_first: "~iaba", "~iábamos";
        imperfect_second: "~iabas", "~iabais";
        imperfect_third: "~iaba", "~iaban";
        preterite_first: "~ié", "~iamos";
        preterite_second: "~iaste", "~iasteis";
        preterite_third: "~ió", "~iaron";
        future_first: "~iaré", "~iaremos";
        future_second: "~iarás", "~iaréis";
        future_third: "~iará", "~iarán";
        conditional_first: "~iaría", "~iaríamos";
        conditional_second: "~iarías", "~iaríais";
        conditional_third: "~iaría", "~iarían";
        present_subjunctive_first: "~ee", "~iemos";
        present_subjunctive_second: "~ees", "~iéis";
        present_subjunctive_third: "~ee", "~een";
        imperfect_subjunctive_first: "~iara", "~iáramos";
        imperfect_subjunctive_second: "~iaras", "~iarais";
        imperfect_subjunctive_third: "~iara", "~iaran";
        imperfect_subjunctive_se_first: "~iase", "~iásemos";
        imperfect_subjunctive_se_second: "~iases", "~iaseis";
        imperfect_subjunctive_se_third: "~iase", "~iasen";
        future_subjunctive_first: "~iare", "~iáremos";
        future_subjunctive_second: "~iares", "~iareis";
        future_subjunctive_third: "~iare", "~iaren";
        vos: "~iás", "~iés", "~iá";
        imperative: "~ea", "~ee", "~iemos", "~iad", "~een";
        negative_imperative: "~ees", "~ee", "~iemos", "~iéis", "~een";
        infinitive: "~iar";
        gerund: "~iando";
        participle_masculine: "~iado", "~iados";
        participle_feminine: "~iada", "~iadas";
    ),
    // Orzarse: 19 words (forzarse, alzarse, vezarse, avezarse, trenzarse, avergonzarse).
    conjugation!("orzarse", 19, "~~zarse";
        present_first: "~~zo", "~~zamos";
        present_second: "~~zas", "~~záis";
        present_third: "~~za", "~~zan";
        imperfect_first: "~~zaba", "~~zábamos";
        imperfect_second: "~~zabas", "~~zabais";
        imperfect_third: "~~zaba", "~~zaban";
        preterite_first: "~~cé", "~~zamos";
        preterite_second: "~~zaste", "~~zasteis";
        preterite_third: "~~zó", "~~zaron";
        future_first: "~~zaré", "~~zaremos";
        future_second: "~~zarás", "~~zaréis";
        future_third: "~~zará", "~~zarán";
        conditional_first: "~~zaría", "~~zaríamos";
        conditional_second: "~~zarías", "~~zaríais";
        conditional_third: "~~zaría", "~~zarían";
        present_subjunctive_first: "~~ce", "~~cemos";
        present_subjunctive_second: "~~ces", "~~céis";
        present_subjunctive_third: "~~ce", "~~cen";
        imperfect_subjunctive_first: "~~zara", "~~záramos";
        imperfect_subjunctive_second: "~~zaras", "~~zarais";
        imperfect_subjunctive_third: "~~zara", "~~zaran";
        imperfect_subjunctive_se_first: "~~zase", "~~zásemos";
        imperfect_subjunctive_se_second: "~~zases", "~~zaseis";
        imperfect_subjunctive_se_third: "~~zase", "~~zasen";
        future_subjunctive_first: "~~zare", "~~záremos";
        future_subjunctive_second: "~~zares", "~~zareis";
        future_subjunctive_third: "~~zare", "~~zaren";
        vos: "~~zás", "~~cés", "~~zate";
        imperative: "~ézat~", "~éc~se", "~~cémonos", "~~zaos", "~éc~nse";
        negative_imperative: "~~ces", "~~ce", "~~cemos", "~~céis", "~~cen";
        infinitive: "~~zarse";
        gerund: "~~zándose";
        participle_masculine: "~~zado", "~~zados";
        participle_feminine: "~~zada", "~~zadas";
    ),
    // Ogerse: 18 words (caerse, creerse, peerse, arderse, leerse, overse).
    conjugation!("ogerse", 18, "~~erse";
        present_first: "~~o", "~~emos";
        present_second: "~~es", "~~éis";
        present_third: "~~e", "~~en";
        imperfect_first: "~~ía", "~~íamos";
        imperfect_second: "~~ías", "~~íais";
        imperfect_third: "~~ía", "~~ían";
        preterite_first: "~~í", "~~ímos";
        preterite_second: "~~íste", "~~ísteis";
        preterite_third: "~~yó", "~~yeron";
        future_first: "~~eré", "~~eremos";
        future_second: "~~erás", "~~eréis";
        future_third: "~~erá", "~~erán";
        conditional_first: "~~ería", "~~eríamos";
        conditional_second: "~~erías", "~~eríais";
        conditional_third: "~~ería", "~~erían";
        present_subjunctive_first: "~~a", "~~amos";
        present_subjunctive_second: "~~as", "~~áis";
        present_subjunctive_third: "~~a", "~~an";
        imperfect_subjunctive_first: "~~yera", "~~yéramos";
        imperfect_subjunctive_second: "~~yeras", "~~yerais";
        imperfect_subjunctive_third: "~~yera", "~~yeran";
        imperfect_subjunctive_se_first: "~~yese", "~~yésemos";
        imperfect_subjunctive_se_second: "~~yeses", "~~yeseis";
        imperfect_subjunctive_se_third: "~~yese", "~~yesen";
        future_subjunctive_first: "~~yere", "~~yéremos";
        future_subjunctive_second: "~~yeres", "~~yereis";
        future_subjunctive_third: "~~yere", "~~yeren";
        vos: "~~és", "~~ás", "~~ete";
        imperative: "~é~te", "~éas~", "~~ámonos", "~~eos", "~éans~";
        negative_imperative: "~~as", "~~a", "~~amos", "~~áis", "~~an";
        infinitive: "~~erse";
        gerund: "~~yéndose";
        participle_masculine: "~~ído", "~~ídos";
        participle_feminine: "~~ída", "~~ídas";
    ),
    // Miscuirse: 18 words (rehuirse, erguirse, huirse, destruirse, miscuirse, excluirse).
    conjugation!("miscuirse", 18, "~uirse";
        present_first: "~uyo", "~uimos";
        present_second: "~uyes", "~uís";
        present_third: "~uye", "~uyen";
        imperfect_first: "~uía", "~uíamos";
        imperfect_second: "~uías", "~uíais";
        imperfect_third: "~uía", "~uían";
        preterite_first: "~uí", "~uimos";
        preterite_second: "~uiste", "~uisteis";
        preterite_third: "~uyó", "~uyeron";
        future_first: "~uiré", "~uiremos";
        future_second: "~uirás", "~uiréis";
        future_third: "~uirá", "~uirán";
        conditional_first: "~uiría", "~uiríamos";
        conditional_second: "~uirías", "~uiríais";
        conditional_third: "~uiría", "~uirían";
        present_subjunctive_first: "~uya", "~uyamos";
        present_subjunctive_second: "~uyas", "~uyáis";
        present_subjunctive_third: "~uya", "~uyan";
        imperfect_subjunctive_first: "~uyera", "~uyéramos";
        imperfect_subjunctive_second: "~uyeras", "~uyerais";
        imperfect_subjunctive_third: "~uyera", "~uyeran";
        imperfect_subjunctive_se_first: "~uyese", "~uyésemos";
        imperfect_subjunctive_se_second: "~uyeses", "~uyeseis";
        imperfect_subjunctive_se_third: "~uyese", "~uyesen";
        future_subjunctive_first: "~uyere", "~uyéremos";
        future_subjunctive_second: "~uyeres", "~uyereis";
        future_subjunctive_third: "~uyere", "~uyeren";
        vos: "~uís", "~uyás", "~uite";
        imperative: "~úyete", "~úyase", "~uyámonos", "~uíos", "~úyanse";
        negative_imperative: "~uyas", "~uya", "~uyamos", "~uyáis", "~uyan";
        infinitive: "~uirse";
        gerund: "~uyéndose";
        participle_masculine: "~uido", "~uidos";
        participle_feminine: "~uida", "~uidas";
    ),
    // Aular: 17 words (aunar, aupar, aullar, reusar, ahusar, sahumar).
    conjugation!("aular", 17, "~u~ar";
        present_first: "~ú~o", "~u~amos";
        present_second: "~ú~as", "~u~áis";
        present_third: "~ú~a", "~ú~an";
        imperfect_first: "~u~aba", "~u~ábamos";
        imperfect_second: "~u~abas", "~u~abais";
        imperfect_third: "~u~aba", "~u~aban";
        preterite_first: "~u~é", "~u~amos";
        preterite_second: "~u~aste", "~u~asteis";
        preterite_third: "~u~ó", "~u~aron";
        future_first: "~u~aré", "~u~aremos";
        future_second: "~u~arás", "~u~aréis";
        future_third: "~u~ará", "~u~arán";
        conditional_first: "~u~aría", "~u~aríamos";
        conditional_second: "~u~arías", "~u~aríais";
        conditional_third: "~u~aría", "~u~arían";
        present_subjunctive_first: "~ú~e", "~u~emos";
        present_subjunctive_second: "~ú~es", "~u~éis";
        present_subjunctive_third: "~ú~e", "~ú~en";
        imperfect_subjunctive_first: "~u~ara", "~u~áramos";
        imperfect_subjunctive_second: "~u~aras", "~u~arais";
        imperfect_subjunctive_third: "~u~ara", "~u~aran";
        imperfect_subjunctive_se_first: "~u~ase", "~u~ásemos";
        imperfect_subjunctive_se_second: "~u~ases", "~u~aseis";
        imperfect_subjunctive_se_third: "~u~ase", "~u~asen";
        future_subjunctive_first: "~u~are", "~u~áremos";
        future_subjunctive_second: "~u~ares", "~u~areis";
        future_subjunctive_third: "~u~are", "~u~aren";
        vos: "~u~ás", "~u~és", "~u~á";
        imperative: "~ú~a", "~ú~e", "~u~emos", "~u~ad", "~ú~en";
        negative_imperative: "~ú~es", "~ú~e", "~u~emos", "~u~éis", "~ú~en";
        infinitive: "~u~ar";
        gerund: "~u~ando";
        participle_masculine: "~u~ado", "~u~ados";
        participle_feminine: "~u~ada", "~u~adas";
    ),
    // Descocarse: 17 words (descocarse, abocarse, apocarse, zocarse, acocarse, embocarse).
    conjugation!("descocarse", 17, "~ocarse";
        present_first: "~oco", "~ocamos";
        present_second: "~ocas", "~ocáis";
        present_third: "~oca", "~ocan";
        imperfect_first: "~ocaba", "~ocábamos";
        imperfect_second: "~ocabas", "~ocabais";
        imperfect_third: "~ocaba", "~ocaban";
        preterite_first: "~oqué", "~ocamos";
        preterite_second: "~ocaste", "~ocasteis";
        preterite_third: "~ocó", "~ocaron";
        future_first: "~ocaré", "~ocaremos";
        future_second: "~ocarás", "~ocaréis";
        future_third: "~ocará", "~ocarán";
        conditional_first: "~ocaría", "~ocaríamos";
        conditional_second: "~ocarías", "~ocaríais";
        conditional_third: "~ocaría", "~ocarían";
        present_subjunctive_first: "~oque", "~oquemos";
        present_subjunctive_second: "~oques", "~oquéis";
        present_subjunctive_third: "~oque", "~oquen";
        imperfect_subjunctive_first: "~ocara", "~ocáramos";
        imperfect_subjunctive_second: "~ocaras", "~ocarais";
        imperfect_subjunctive_third: "~ocara", "~ocaran";
        imperfect_subjunctive_se_first: "~ocase", "~ocásemos";
        imperfect_subjunctive_se_second: "~ocases", "~ocaseis";
        imperfect_subjunctive_se_third: "~ocase", "~ocasen";
        future_subjunctive_first: "~ocare", "~ocáremos";
        future_subjunctive_second: "~ocares", "~ocareis";
        future_subjunctive_third: "~ocare", "~ocaren";
        vos: "~ocás", "~oqués", "~ocate";
        imperative: "~ócate", "~óquese", "~oquémonos", "~ocaos", "~óquense";
        negative_imperative: "~oques", "~oque", "~oquemos", "~oquéis", "~oquen";
        infinitive: "~ocarse";
        gerund: "~ocándose";
        participle_masculine: "~ocado", "~ocados";
        participle_feminine: "~ocada", "~ocadas";
    ),
    // Lugarse: 17 words (añusgarse, purgarse, enjugarse, jugarse, fugarse, rugarse).
    conjugation!("lugarse", 17, "~u~arse";
        present_first: "~u~o", "~u~amos";
        present_second: "~u~as", "~u~áis";
        present_third: "~u~a", "~u~an";
        imperfect_first: "~u~aba", "~u~ábamos";
        imperfect_second: "~u~abas", "~u~abais";
        imperfect_third: "~u~aba", "~u~aban";
        preterite_first: "~u~ué", "~u~amos";
        preterite_second: "~u~aste", "~u~asteis";
        preterite_third: "~u~ó", "~u~aron";
        future_first: "~u~aré", "~u~aremos";
        future_second: "~u~arás", "~u~aréis";
        future_third: "~u~ará", "~u~arán";
        conditional_first: "~u~aría", "~u~aríamos";
        conditional_second: "~u~arías", "~u~aríais";
        conditional_third: "~u~aría", "~u~arían";
        present_subjunctive_first: "~u~ue", "~u~uemos";
        present_subjunctive_second: "~u~ues", "~u~uéis";
        present_subjunctive_third: "~u~ue", "~u~uen";
        imperfect_subjunctive_first: "~u~ara", "~u~áramos";
        imperfect_subjunctive_second: "~u~aras", "~u~arais";
        imperfect_subjunctive_third: "~u~ara", "~u~aran";
        imperfect_subjunctive_se_first: "~u~ase", "~u~ásemos";
        imperfect_subjunctive_se_second: "~u~ases", "~u~aseis";
        imperfect_subjunctive_se_third: "~u~ase", "~u~asen";
        future_subjunctive_first: "~u~are", "~u~áremos";
        future_subjunctive_second: "~u~ares", "~u~areis";
        future_subjunctive_third: "~u~are", "~u~aren";
        vos: "~u~ás", "~u~ués", "~u~ate";
        imperative: "~ú~ate", "~ú~uese", "~u~uémonos", "~u~aos", "~ú~uense";
        negative_imperative: "~u~ues", "~u~ue", "~u~uemos", "~u~uéis", "~u~uen";
        infinitive: "~u~arse";
        gerund: "~u~ándose";
        participle_masculine: "~u~ado", "~u~ados";
        participle_feminine: "~u~ada", "~u~adas";
    ),
    // Ercarse: 17 words (puntisecarse, secarse, ercarse, incarse, uecarse, cecarse).
    conjugation!("ercarse", 17, "~~carse";
        present_first: "~~co", "~~camos";
        present_second: "~~cas", "~~cáis";
        present_third: "~~ca", "~~can";
        imperfect_first: "~~caba", "~~cábamos";
        imperfect_second: "~~cabas", "~~cabais";
        imperfect_third: "~~caba", "~~caban";
        preterite_first: "~~qué", "~~camos";
        preterite_second: "~~caste", "~~casteis";
        preterite_third: "~~có", "~~caron";
        future_first: "~~caré", "~~caremos";
        future_second: "~~carás", "~~caréis";
        future_third: "~~cará", "~~carán";
        conditional_first: "~~caría", "~~caríamos";
        conditional_second: "~~carías", "~~caríais";
        conditional_third: "~~caría", "~~carían";
        present_subjunctive_first: "~~que", "~~quemos";
        present_subjunctive_second: "~~ques", "~~quéis";
        present_subjunctive_third: "~~que", "~~quen";
        imperfect_subjunctive_first: "~~cara", "~~cáramos";
        imperfect_subjunctive_second: "~~caras", "~~carais";
        imperfect_subjunctive_third: "~~cara", "~~caran";
        imperfect_subjunctive_se_first: "~~case", "~~cásemos";
        imperfect_subjunctive_se_second: "~~cases", "~~caseis";
        imperfect_subjunctive_se_third: "~~case", "~~casen";
        future_subjunctive_first: "~~care", "~~cáremos";
        future_subjunctive_second: "~~cares", "~~careis";
        future_subjunctive_third: "~~care", "~~caren";
        vos: "~~cás", "~~qués", "~~cate";
        imperative: "~écat~", "~équ~se", "~~quémonos", "~~caos", "~équ~nse";
        negative_imperative: "~~ques", "~~que", "~~quemos", "~~quéis", "~~quen";
        infinitive: "~~carse";
        gerund: "~~cándose";
        participle_masculine: "~~cado", "~~cados";
        participle_feminine: "~~cada", "~~cadas";
    ),
    // Tender: 16 words (tender, perder, heder, hender, jeder, cerner).
    conjugation!("tender", 16, "~~er";
        present_first: "~i~o", "~~emos";
        present_second: "~i~es", "~~éis";
        present_third: "~i~e", "~i~en";
        imperfect_first: "~~ía", "~~íamos";
        imperfect_second: "~~ías", "~~íais";
        imperfect_third: "~~ía", "~~ían";
        preterite_first: "~~í", "~~imos";
        preterite_second: "~~iste", "~~isteis";
        preterite_third: "~~ió", "~~ieron";
        future_first: "~~eré", "~~eremos";
        future_second: "~~erás", "~~eréis";
        future_third: "~~erá", "~~erán";
        conditional_first: "~~ería", "~~eríamos";
        conditional_second: "~~erías", "~~eríais";
        conditional_third: "~~ería", "~~erían";
        present_subjunctive_first: "~i~a", "~~amos";
        present_subjunctive_second: "~i~as", "~~áis";
        present_subjunctive_third: "~i~a", "~i~an";
        imperfect_subjunctive_first: "~~iera", "~~iéramos";
        imperfect_subjunctive_second: "~~ieras", "~~ierais";
        imperfect_subjunctive_third: "~~iera", "~~ieran";
        imperfect_subjunctive_se_first: "~~iese", "~~iésemos";
        imperfect_subjunctive_se_second: "~~ieses", "~~ieseis";
        imperfect_subjunctive_se_third: "~~iese", "~~iesen";
        future_subjunctive_first: "~~iere", "~~iéremos";
        future_subjunctive_second: "~~ieres", "~~iereis";
        future_subjunctive_third: "~~iere", "~~ieren";
        vos: "~~és", "~~ás", "~~é";
        imperative: "~i~e", "~i~a", "~~amos", "~~ed", "~i~an";
        negative_imperative: "~i~as", "~i~a", "~~amos", "~~áis", "~i~an";
        infinitive: "~~er";
        gerund: "~~iendo";
        participle_masculine: "~~ido", "~~idos";
        participle_feminine: "~~ida", "~~idas";
    ),
    // Ullirse: 15 words (unirse, ullirse, umirse, urrirse, uncirse, ubrirse).
    conjugation!("ullirse", 15, "u~irse";
        present_first: "u~o", "u~imos";
        present_second: "u~es", "u~ís";
        present_third: "u~e", "u~en";
        imperfect_first: "u~ía", "u~íamos";
        imperfect_second: "u~ías", "u~íais";
        imperfect_third: "u~ía", "u~ían";
        preterite_first: "u~í", "u~imos";
        preterite_second: "u~iste", "u~isteis";
        preterite_third: "u~ió", "u~ieron";
        future_first: "u~iré", "u~iremos";
        future_second: "u~irás", "u~iréis";
        future_third: "u~irá", "u~irán";
        conditional_first: "u~iría", "u~iríamos";
        conditional_second: "u~irías", "u~iríais";
        conditional_third: "u~iría", "u~irían";
        present_subjunctive_first: "u~a", "u~amos";
        present_subjunctive_second: "u~as", "u~áis";
        present_subjunctive_third: "u~a", "u~an";
        imperfect_subjunctive_first: "u~iera", "u~iéramos";
        imperfect_subjunctive_second: "u~ieras", "u~ierais";
        imperfect_subjunctive_third: "u~iera", "u~ieran";
        imperfect_subjunctive_se_first: "u~iese", "u~iésemos";
        imperfect_subjunctive_se_second: "u~ieses", "u~ieseis";
        imperfect_subjunctive_se_third: "u~iese", "u~iesen";
        future_subjunctive_first: "u~iere", "u~iéremos";
        future_subjunctive_second: "u~ieres", "u~iereis";
        future_subjunctive_third: "u~iere", "u~ieren";
        vos: "u~ís", "u~ás", "u~ite";
        imperative: "ú~ete", "ú~ase", "u~ámonos", "u~íos", "ú~anse";
        negative_imperative: "u~as", "u~a", "u~amos", "u~áis", "u~an";
        infinitive: "u~irse";
        gerund: "u~iéndose";
        participle_masculine: "u~ido", "u~idos";
        participle_feminine: "u~ida", "u~idas";
    ),
    // Definirse: 15 words (hispirse, omitirse, definirse, percibirse, hibirse, tinguirse).
    conjugation!("definirse", 15, "~i~irse";
        present_first: "~i~o", "~i~imos";
        present_second: "~i~es", "~i~ís";
        present_third: "~i~e", "~i~en";
        imperfect_first: "~i~ía", "~i~íamos";
        imperfect_second: "~i~ías", "~i~íais";
        imperfect_third: "~i~ía", "~i~ían";
        preterite_first: "~i~í", "~i~imos";
        preterite_second: "~i~iste", "~i~isteis";
        preterite_third: "~i~ió", "~i~ieron";
        future_first: "~i~iré", "~i~iremos";
        future_second: "~i~irás", "~i~iréis";
        future_third: "~i~irá", "~i~irán";
        conditional_first: "~i~iría", "~i~iríamos";
        conditional_second: "~i~irías", "~i~iríais";
        conditional_third: "~i~iría", "~i~irían";
        present_subjunctive_first: "~i~a", "~i~amos";
        present_subjunctive_second: "~i~as", "~i~áis";
        present_subjunctive_third: "~i~a", "~i~an";
        imperfect_subjunctive_first: "~i~iera", "~i~iéramos";
        imperfect_subjunctive_second: "~i~ieras", "~i~ierais";
        imperfect_subjunctive_third: "~i~iera", "~i~ieran";
        imperfect_subjunctive_se_first: "~i~iese", "~i~iésemos";
        imperfect_subjunctive_se_second: "~i~ieses", "~i~ieseis";
        imperfect_subjunctive_se_third: "~i~iese", "~i~iesen";
        future_subjunctive_first: "~i~iere", "~i~iéremos";
        future_subjunctive_second: "~i~ieres", "~i~iereis";
        future_subjunctive_third: "~i~iere", "~i~ieren";
        vos: "~i~ís", "~i~ás", "~i~ite";
        imperative: "~í~ete", "~í~ase", "~i~ámonos", "~i~íos", "~í~anse";
        negative_imperative: "~i~as", "~i~a", "~i~amos", "~i~áis", "~i~an";
        infinitive: "~i~irse";
        gerund: "~i~iéndose";
        participle_masculine: "~i~ido", "~i~idos";
        participle_feminine: "~i~ida", "~i~idas";
    ),
    // Lazarse: 14 words (aquebrazarse, avorazarse, tazarse, razarse, ostazarse, lazarse).
    conjugation!("lazarse", 14, "~azarse";
        present_first: "~azo", "~azamos";
        present_second: "~azas", "~azáis";
        present_third: "~aza", "~azan";
        imperfect_first: "~azaba", "~azábamos";
        imperfect_second: "~azabas", "~azabais";
        imperfect_third: "~azaba", "~azaban";
        preterite_first: "~acé", "~azamos";
        preterite_second: "~azaste", "~azasteis";
        preterite_third: "~azó", "~azaron";
        future_first: "~azaré", "~azaremos";
        future_second: "~azarás", "~azaréis";
        future_third: "~azará", "~azarán";
        conditional_first: "~azaría", "~azaríamos";
        conditional_second: "~azarías", "~azaríais";
        conditional_third: "~azaría", "~azarían";
        present_subjunctive_first: "~ace", "~acemos";
        present_subjunctive_second: "~aces", "~acéis";
        present_subjunctive_third: "~ace", "~acen";
        imperfect_subjunctive_first: "~azara", "~azáramos";
        imperfect_subjunctive_second: "~azaras", "~azarais";
        imperfect_subjunctive_third: "~azara", "~azaran";
        imperfect_subjunctive_se_first: "~azase", "~azásemos";
        imperfect_subjunctive_se_second: "~azases", "~azaseis";
        imperfect_subjunctive_se_third: "~azase", "~azasen";
        future_subjunctive_first: "~azare", "~azáremos";
        future_subjunctive_second: "~azares", "~azareis";
        future_subjunctive_third: "~azare", "~azaren";
        vos: "~azás", "~acés", "~azate";
        imperative: "~ázate", "~ácese", "~acémonos", "~azaos", "~ácense";
        negative_imperative: "~aces", "~ace", "~acemos", "~acéis", "~acen";
        infinitive: "~azarse";
        gerund: "~azándose";
        participle_masculine: "~azado", "~azados";
        participle_feminine: "~azada", "~azadas";
    ),
    // Calzarse: 14 words (desalzarse, empanzarse, descalzarse, avanzarse, calzarse, lanzarse).
    conjugation!("calzarse", 14, "~a~zarse";
        present_first: "~a~zo", "~a~zamos";
        present_second: "~a~zas", "~a~záis";
        present_third: "~a~za", "~a~zan";
        imperfect_first: "~a~zaba", "~a~zábamos";
        imperfect_second: "~a~zabas", "~a~zabais";
        imperfect_third: "~a~zaba", "~a~zaban";
        preterite_first: "~a~cé", "~a~zamos";
        preterite_second: "~a~zaste", "~a~zasteis";
        preterite_third: "~a~zó", "~a~zaron";
        future_first: "~a~zaré", "~a~zaremos";
        future_second: "~a~zarás", "~a~zaréis";
        future_third: "~a~zará", "~a~zarán";
        conditional_first: "~a~zaría", "~a~zaríamos";
        conditional_second: "~a~zarías", "~a~zaríais";
        conditional_third: "~a~zaría", "~a~zarían";
        present_subjunctive_first: "~a~ce", "~a~cemos";
        present_subjunctive_second: "~a~ces", "~a~céis";
        present_subjunctive_third: "~a~ce", "~a~cen";
        imperfect_subjunctive_first: "~a~zara", "~a~záramos";
        imperfect_subjunctive_second: "~a~zaras", "~a~zarais";
        imperfect_subjunctive_third: "~a~zara", "~a~zaran";
        imperfect_subjunctive_se_first: "~a~zase", "~a~zásemos";
        imperfect_subjunctive_se_second: "~a~zases", "~a~zaseis";
        imperfect_subjunctive_se_third: "~a~zase", "~a~zasen";
        future_subjunctive_first: "~a~zare", "~a~záremos";
        future_subjunctive_second: "~a~zares", "~a~zareis";
        future_subjunctive_third: "~a~zare", "~a~zaren";
        vos: "~a~zás", "~a~cés", "~a~zate";
        imperative: "~á~zate", "~á~cese", "~a~cémonos", "~a~zaos", "~á~cense";
        negative_imperative: "~a~ces", "~a~ce", "~a~cemos", "~a~céis", "~a~cen";
        infinitive: "~a~zarse";
        gerund: "~a~zándose";
        participle_masculine: "~a~zado", "~a~zados";
        participle_feminine: "~a~zada", "~a~zadas";
    ),
    // Caer: 13 words (peer, creer, caer, leer, veer, seer).
    conjugation!("caer", 13, "~er";
        present_first: "~o", "~emos";
        present_second: "~es", "~éis";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~ímos";
        preterite_second: "~íste", "~ísteis";
        preterite_third: "~yó", "~yeron";
        future_first: "~eré", "~eremos";
        future_second: "~erás", "~eréis";
        future_third: "~erá", "~erán";
        conditional_first: "~ería", "~eríamos";
        conditional_second: "~erías", "~eríais";
        conditional_third: "~ería", "~erían";
        present_subjunctive_first: "~a", "~amos";
        present_subjunctive_second: "~as", "~áis";
        present_subjunctive_third: "~a", "~an";
        imperfect_subjunctive_first: "~yera", "~yéramos";
        imperfect_subjunctive_second: "~yeras", "~yerais";
        imperfect_subjunctive_third: "~yera", "~yeran";
        imperfect_subjunctive_se_first: "~yese", "~yésemos";
        imperfect_subjunctive_se_second: "~yeses", "~yeseis";
        imperfect_subjunctive_se_third: "~yese", "~yesen";
        future_subjunctive_first: "~yere", "~yéremos";
        future_subjunctive_second: "~yeres", "~yereis";
        future_subjunctive_third: "~yere", "~yeren";
        vos: "~és", "~ás", "~é";
        imperative: "~e", "~a", "~amos", "~ed", "~an";
        negative_imperative: "~as", "~a", "~amos", "~áis", "~an";
        infinitive: "~er";
        gerund: "~yendo";
        participle_masculine: "~ído", "~ídos";
        participle_feminine: "~ída", "~ídas";
    ),
    // Pentirse: 13 words (sentirse, rendirse, rehenchirse, mentirse, henchirse, servirse).
    conjugation!("pentirse", 13, "~e~irse";
        present_first: "~ie~o", "~e~imos";
        present_second: "~ie~es", "~e~ís";
        present_third: "~ie~e", "~ie~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~e~í", "~e~imos";
        preterite_second: "~e~iste", "~e~isteis";
        preterite_third: "~i~ió", "~i~ieron";
        future_first: "~e~iré", "~e~iremos";
        future_second: "~e~irás", "~e~iréis";
        future_third: "~e~irá", "~e~irán";
        conditional_first: "~e~iría", "~e~iríamos";
        conditional_second: "~e~irías", "~e~iríais";
        conditional_third: "~e~iría", "~e~irían";
        present_subjunctive_first: "~ie~a", "~i~amos";
        present_subjunctive_second: "~ie~as", "~i~áis";
        present_subjunctive_third: "~ie~a", "~ie~an";
        imperfect_subjunctive_first: "~i~iera", "~i~iéramos";
        imperfect_subjunctive_second: "~i~ieras", "~i~ierais";
        imperfect_subjunctive_third: "~i~iera", "~i~ieran";
        imperfect_subjunctive_se_first: "~i~iese", "~i~iésemos";
        imperfect_subjunctive_se_second: "~i~ieses", "~i~ieseis";
        imperfect_subjunctive_se_third: "~i~iese", "~i~iesen";
        future_subjunctive_first: "~i~iere", "~i~iéremos";
        future_subjunctive_second: "~i~ieres", "~i~iereis";
        future_subjunctive_third: "~i~iere", "~i~ieren";
        vos: "~e~ís", "~i~ás", "~e~ite";
        imperative: "~ié~ete", "~ié~ase", "~i~ámonos", "~e~íos", "~ié~anse";
        negative_imperative: "~ie~as", "~ie~a", "~i~amos", "~i~áis", "~ie~an";
        infinitive: "~e~irse";
        gerund: "~i~iéndose";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Sacarse: 13 words (sacarse, empacarse, atacarse, opacarse, macarse, racarse).
    conjugation!("sacarse", 13, "~acarse";
        present_first: "~aco", "~acamos";
        present_second: "~acas", "~acáis";
        present_third: "~aca", "~acan";
        imperfect_first: "~acaba", "~acábamos";
        imperfect_second: "~acabas", "~acabais";
        imperfect_third: "~acaba", "~acaban";
        preterite_first: "~aqué", "~acamos";
        preterite_second: "~acaste", "~acasteis";
        preterite_third: "~acó", "~acaron";
        future_first: "~acaré", "~acaremos";
        future_second: "~acarás", "~acaréis";
        future_third: "~acará", "~acarán";
        conditional_first: "~acaría", "~acaríamos";
        conditional_second: "~acarías", "~acaríais";
        conditional_third: "~acaría", "~acarían";
        present_subjunctive_first: "~aque", "~aquemos";
        present_subjunctive_second: "~aques", "~aquéis";
        present_subjunctive_third: "~aque", "~aquen";
        imperfect_subjunctive_first: "~acara", "~acáramos";
        imperfect_subjunctive_second: "~acaras", "~acarais";
        imperfect_subjunctive_third: "~acara", "~acaran";
        imperfect_subjunctive_se_first: "~acase", "~acásemos";
        imperfect_subjunctive_se_second: "~acases", "~acaseis";
        imperfect_subjunctive_se_third: "~acase", "~acasen";
        future_subjunctive_first: "~acare", "~acáremos";
        future_subjunctive_second: "~acares", "~acareis";
        future_subjunctive_third: "~acare", "~acaren";
        vos: "~acás", "~aqués", "~acate";
        imperative: "~ácate", "~áquese", "~aquémonos", "~acaos", "~áquense";
        negative_imperative: "~aques", "~aque", "~aquemos", "~aquéis", "~aquen";
        infinitive: "~acarse";
        gerund: "~acándose";
        participle_masculine: "~acado", "~acados";
        participle_feminine: "~acada", "~acadas";
    ),
    // Rcir: 12 words (uncir, juncir, fulcir, rcir, arcir, bucir).
    conjugation!("rcir", 12, "~cir";
        present_first: "~zo", "~cimos";
        present_second: "~ces", "~cís";
        present_third: "~ce", "~cen";
        imperfect_first: "~cía", "~cíamos";
        imperfect_second: "~cías", "~cíais";
        imperfect_third: "~cía", "~cían";
        preterite_first: "~cí", "~cimos";
        preterite_second: "~ciste", "~cisteis";
        preterite_third: "~ció", "~cieron";
        future_first: "~ciré", "~ciremos";
        future_second: "~cirás", "~ciréis";
        future_third: "~cirá", "~cirán";
        conditional_first: "~ciría", "~ciríamos";
        conditional_second: "~cirías", "~ciríais";
        conditional_third: "~ciría", "~cirían";
        present_subjunctive_first: "~za", "~zamos";
        present_subjunctive_second: "~zas", "~záis";
        present_subjunctive_third: "~za", "~zan";
        imperfect_subjunctive_first: "~ciera", "~ciéramos";
        imperfect_subjunctive_second: "~cieras", "~cierais";
        imperfect_subjunctive_third: "~ciera", "~cieran";
        imperfect_subjunctive_se_first: "~ciese", "~ciésemos";
        imperfect_subjunctive_se_second: "~cieses", "~cieseis";
        imperfect_subjunctive_se_third: "~ciese", "~ciesen";
        future_subjunctive_first: "~ciere", "~ciéremos";
        future_subjunctive_second: "~cieres", "~ciereis";
        future_subjunctive_third: "~ciere", "~cieren";
        vos: "~cís", "~zás", "~cí";
        imperative: "~ce", "~za", "~zamos", "~cid", "~zan";
        negative_imperative: "~zas", "~za", "~zamos", "~záis", "~zan";
        infinitive: "~cir";
        gerund: "~ciendo";
        participle_masculine: "~cido", "~cidos";
        participle_feminine: "~cida", "~cidas";
    ),
    // Regar: 12 words (negar, regar, segar, desplegar, replegar, cegar).
    conjugation!("regar", 12, "~~ar";
        present_first: "~i~o", "~~amos";
        present_second: "~i~as", "~~áis";
        present_third: "~i~a", "~i~an";
        imperfect_first: "~~aba", "~~ábamos";
        imperfect_second: "~~abas", "~~abais";
        imperfect_third: "~~aba", "~~aban";
        preterite_first: "~~ué", "~~amos";
        preterite_second: "~~aste", "~~asteis";
        preterite_third: "~~ó", "~~aron";
        future_first: "~~aré", "~~aremos";
        future_second: "~~arás", "~~aréis";
        future_third: "~~ará", "~~arán";
        conditional_first: "~~aría", "~~aríamos";
        conditional_second: "~~arías", "~~aríais";
        conditional_third: "~~aría", "~~arían";
        present_subjunctive_first: "~i~ue", "~~uemos";
        present_subjunctive_second: "~i~ues", "~~uéis";
        present_subjunctive_third: "~i~ue", "~i~uen";
        imperfect_subjunctive_first: "~~ara", "~~áramos";
        imperfect_subjunctive_second: "~~aras", "~~arais";
        imperfect_subjunctive_third: "~~ara", "~~aran";
        imperfect_subjunctive_se_first: "~~ase", "~~ásemos";
        imperfect_subjunctive_se_second: "~~ases", "~~aseis";
        imperfect_subjunctive_se_third: "~~ase", "~~asen";
        future_subjunctive_first: "~~are", "~~áremos";
        future_subjunctive_second: "~~ares", "~~areis";
        future_subjunctive_third: "~~are", "~~aren";
        vos: "~~ás", "~~ués", "~~á";
        imperative: "~i~a", "~i~ue", "~~uemos", "~~ad", "~i~uen";
        negative_imperative: "~i~ues", "~i~ue", "~~uemos", "~~uéis", "~i~uen";
        infinitive: "~~ar";
        gerund: "~~ando";
        participle_masculine: "~~ado", "~~ados";
        participle_feminine: "~~ada", "~~adas";
    ),
    // Seguir: 12 words (servir, seguir, vestir, gemir, rendir, henchir).
    conjugation!("seguir", 12, "~e~ir";
        present_first: "~i~o", "~e~imos";
        present_second: "~i~es", "~e~ís";
        present_third: "~i~e", "~i~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~e~í", "~e~imos";
        preterite_second: "~e~iste", "~e~isteis";
        preterite_third: "~i~ió", "~i~ieron";
        future_first: "~e~iré", "~e~iremos";
        future_second: "~e~irás", "~e~iréis";
        future_third: "~e~irá", "~e~irán";
        conditional_first: "~e~iría", "~e~iríamos";
        conditional_second: "~e~irías", "~e~iríais";
        conditional_third: "~e~iría", "~e~irían";
        present_subjunctive_first: "~i~a", "~i~amos";
        present_subjunctive_second: "~i~as", "~i~áis";
        present_subjunctive_third: "~i~a", "~i~an";
        imperfect_subjunctive_first: "~i~iera", "~i~iéramos";
        imperfect_subjunctive_second: "~i~ieras", "~i~ierais";
        imperfect_subjunctive_third: "~i~iera", "~i~ieran";
        imperfect_subjunctive_se_first: "~i~iese", "~i~iésemos";
        imperfect_subjunctive_se_second: "~i~ieses", "~i~ieseis";
        imperfect_subjunctive_se_third: "~i~iese", "~i~iesen";
        future_subjunctive_first: "~i~iere", "~i~iéremos";
        future_subjunctive_second: "~i~ieres", "~i~iereis";
        future_subjunctive_third: "~i~iere", "~i~ieren";
        vos: "~e~ís", "~i~ás", "~e~í";
        imperative: "~i~e", "~i~a", "~i~amos", "~e~id", "~i~an";
        negative_imperative: "~i~as", "~i~a", "~i~amos", "~i~áis", "~i~an";
        infinitive: "~e~ir";
        gerund: "~i~iendo";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Aizar: 12 words (europeizar, judaizar, raizar, hebraizar, amaizar, aizar).
    conjugation!("aizar", 12, "~izar";
        present_first: "~ízo", "~izamos";
        present_second: "~ízas", "~izáis";
        present_third: "~íza", "~ízan";
        imperfect_first: "~izaba", "~izábamos";
        imperfect_second: "~izabas", "~izabais";
        imperfect_third: "~izaba", "~izaban";
        preterite_first: "~icé", "~izamos";
        preterite_second: "~izaste", "~izasteis";
        preterite_third: "~izó", "~izaron";
        future_first: "~izaré", "~izaremos";
        future_second: "~izarás", "~izaréis";
        future_third: "~izará", "~izarán";
        conditional_first: "~izaría", "~izaríamos";
        conditional_second: "~izarías", "~izaríais";
        conditional_third: "~izaría", "~izarían";
        present_subjunctive_first: "~íce", "~icemos";
        present_subjunctive_second: "~íces", "~icéis";
        present_subjunctive_third: "~íce", "~ícen";
        imperfect_subjunctive_first: "~izara", "~izáramos";
        imperfect_subjunctive_second: "~izaras", "~izarais";
        imperfect_subjunctive_third: "~izara", "~izaran";
        imperfect_subjunctive_se_first: "~izase", "~izásemos";
        imperfect_subjunctive_se_second: "~izases", "~izaseis";
        imperfect_subjunctive_se_third: "~izase", "~izasen";
        future_subjunctive_first: "~izare", "~izáremos";
        future_subjunctive_second: "~izares", "~izareis";
        future_subjunctive_third: "~izare", "~izaren";
        vos: "~izás", "~icés", "~izá";
        imperative: "~íza", "~íce", "~icemos", "~izad", "~ícen";
        negative_imperative: "~íces", "~íce", "~icemos", "~icéis", "~ícen";
        infinitive: "~izar";
        gerund: "~izando";
        participle_masculine: "~izado", "~izados";
        participle_feminine: "~izada", "~izadas";
    ),
    // Oger: 12 words (teger, oger, erger, coger, terger, verger).
    conjugation!("oger", 12, "~ger";
        present_first: "~jo", "~gemos";
        present_second: "~ges", "~géis";
        present_third: "~ge", "~gen";
        imperfect_first: "~gía", "~gíamos";
        imperfect_second: "~gías", "~gíais";
        imperfect_third: "~gía", "~gían";
        preterite_first: "~gí", "~gimos";
        preterite_second: "~giste", "~gisteis";
        preterite_third: "~gió", "~gieron";
        future_first: "~geré", "~geremos";
        future_second: "~gerás", "~geréis";
        future_third: "~gerá", "~gerán";
        conditional_first: "~gería", "~geríamos";
        conditional_second: "~gerías", "~geríais";
        conditional_third: "~gería", "~gerían";
        present_subjunctive_first: "~ja", "~jamos";
        present_subjunctive_second: "~jas", "~jáis";
        present_subjunctive_third: "~ja", "~jan";
        imperfect_subjunctive_first: "~giera", "~giéramos";
        imperfect_subjunctive_second: "~gieras", "~gierais";
        imperfect_subjunctive_third: "~giera", "~gieran";
        imperfect_subjunctive_se_first: "~giese", "~giésemos";
        imperfect_subjunctive_se_second: "~gieses", "~gieseis";
        imperfect_subjunctive_se_third: "~giese", "~giesen";
        future_subjunctive_first: "~giere", "~giéremos";
        future_subjunctive_second: "~gieres", "~giereis";
        future_subjunctive_third: "~giere", "~gieren";
        vos: "~gés", "~jás", "~gé";
        imperative: "~ge", "~ja", "~jamos", "~ged", "~jan";
        negative_imperative: "~jas", "~ja", "~jamos", "~jáis", "~jan";
        infinitive: "~ger";
        gerund: "~giendo";
        participle_masculine: "~gido", "~gidos";
        participle_feminine: "~gida", "~gidas";
    ),
    // Decir: 11 words (decir, maldecir, contradecir, desdecir, predecir, antedecir).
    conjugation!("decir", 11, "~ecir";
        present_first: "~igo", "~ecimos";
        present_second: "~ices", "~ecís";
        present_third: "~ice", "~icen";
        imperfect_first: "~ecía", "~ecíamos";
        imperfect_second: "~ecías", "~ecíais";
        imperfect_third: "~ecía", "~ecían";
        preterite_first: "~ije", "~ijimos";
        preterite_second: "~ijiste", "~ijisteis";
        preterite_third: "~ijo", "~ijeron";
        future_first: "~eciré", "~eciremos";
        future_second: "~ecirás", "~eciréis";
        future_third: "~ecirá", "~ecirán";
        conditional_first: "~eciría", "~eciríamos";
        conditional_second: "~ecirías", "~eciríais";
        conditional_third: "~eciría", "~ecirían";
        present_subjunctive_first: "~iga", "~igamos";
        present_subjunctive_second: "~igas", "~igáis";
        present_subjunctive_third: "~iga", "~igan";
        imperfect_subjunctive_first: "~ijera", "~ijéramos";
        imperfect_subjunctive_second: "~ijeras", "~ijerais";
        imperfect_subjunctive_third: "~ijera", "~ijeran";
        imperfect_subjunctive_se_first: "~ijese", "~ijésemos";
        imperfect_subjunctive_se_second: "~ijeses", "~ijeseis";
        imperfect_subjunctive_se_third: "~ijese", "~ijesen";
        future_subjunctive_first: "~ijere", "~ijéremos";
        future_subjunctive_second: "~ijeres", "~ijereis";
        future_subjunctive_third: "~ijere", "~ijeren";
        vos: "~ecís", "~igás", "~ecí";
        imperative: "~ice", "~iga", "~igamos", "~ecid", "~igan";
        negative_imperative: "~igas", "~iga", "~igamos", "~igáis", "~igan";
        infinitive: "~ecir";
        gerund: "~iciendo";
        participle_masculine: "~icho", "~ichos";
        participle_feminine: "~icha", "~ichas";
    ),
    // Perderse: 11 words (perderse, tenderse, cernerse, tenerse, henderse, verterse).
    conjugation!("perderse", 11, "~e~erse";
        present_first: "~ie~o", "~e~emos";
        present_second: "~ie~es", "~e~éis";
        present_third: "~ie~e", "~ie~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~e~í", "~e~imos";
        preterite_second: "~e~iste", "~e~isteis";
        preterite_third: "~e~ió", "~e~ieron";
        future_first: "~e~eré", "~e~eremos";
        future_second: "~e~erás", "~e~eréis";
        future_third: "~e~erá", "~e~erán";
        conditional_first: "~e~ería", "~e~eríamos";
        conditional_second: "~e~erías", "~e~eríais";
        conditional_third: "~e~ería", "~e~erían";
        present_subjunctive_first: "~ie~a", "~e~amos";
        present_subjunctive_second: "~ie~as", "~e~áis";
        present_subjunctive_third: "~ie~a", "~ie~an";
        imperfect_subjunctive_first: "~e~iera", "~e~iéramos";
        imperfect_subjunctive_second: "~e~ieras", "~e~ierais";
        imperfect_subjunctive_third: "~e~iera", "~e~ieran";
        imperfect_subjunctive_se_first: "~e~iese", "~e~iésemos";
        imperfect_subjunctive_se_second: "~e~ieses", "~e~ieseis";
        imperfect_subjunctive_se_third: "~e~iese", "~e~iesen";
        future_subjunctive_first: "~e~iere", "~e~iéremos";
        future_subjunctive_second: "~e~ieres", "~e~iereis";
        future_subjunctive_third: "~e~iere", "~e~ieren";
        vos: "~e~és", "~e~ás", "~e~ete";
        imperative: "~ié~ete", "~ié~ase", "~e~ámonos", "~e~eos", "~ié~anse";
        negative_imperative: "~ie~as", "~ie~a", "~e~amos", "~e~áis", "~ie~an";
        infinitive: "~e~erse";
        gerund: "~e~iéndose";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Hacerse: 11 words (autosatisfacerse, hacerse, valerse, lamerse, nacerse, barrerse).
    conjugation!("hacerse", 11, "~a~erse";
        present_first: "~a~o", "~a~emos";
        present_second: "~a~es", "~a~éis";
        present_third: "~a~e", "~a~en";
        imperfect_first: "~a~ía", "~a~íamos";
        imperfect_second: "~a~ías", "~a~íais";
        imperfect_third: "~a~ía", "~a~ían";
        preterite_first: "~a~í", "~a~imos";
        preterite_second: "~a~iste", "~a~isteis";
        preterite_third: "~a~ió", "~a~ieron";
        future_first: "~a~eré", "~a~eremos";
        future_second: "~a~erás", "~a~eréis";
        future_third: "~a~erá", "~a~erán";
        conditional_first: "~a~ería", "~a~eríamos";
        conditional_second: "~a~erías", "~a~eríais";
        conditional_third: "~a~ería", "~a~erían";
        present_subjunctive_first: "~a~a", "~a~amos";
        present_subjunctive_second: "~a~as", "~a~áis";
        present_subjunctive_third: "~a~a", "~a~an";
        imperfect_subjunctive_first: "~a~iera", "~a~iéramos";
        imperfect_subjunctive_second: "~a~ieras", "~a~ierais";
        imperfect_subjunctive_third: "~a~iera", "~a~ieran";
        imperfect_subjunctive_se_first: "~a~iese", "~a~iésemos";
        imperfect_subjunctive_se_second: "~a~ieses", "~a~ieseis";
        imperfect_subjunctive_se_third: "~a~iese", "~a~iesen";
        future_subjunctive_first: "~a~iere", "~a~iéremos";
        future_subjunctive_second: "~a~ieres", "~a~iereis";
        future_subjunctive_third: "~a~iere", "~a~ieren";
        vos: "~a~és", "~a~ás", "~a~ete";
        imperative: "~á~ete", "~á~ase", "~a~ámonos", "~a~eos", "~á~anse";
        negative_imperative: "~a~as", "~a~a", "~a~amos", "~a~áis", "~a~an";
        infinitive: "~a~erse";
        gerund: "~a~iéndose";
        participle_masculine: "~a~ido", "~a~idos";
        participle_feminine: "~a~ida", "~a~idas";
    ),
    // Chuzarse: 11 words (cruzarse, chuzarse, puzarse, ruzarse, garruzarse, bruzarse).
    conjugation!("chuzarse", 11, "~uzarse";
        present_first: "~uzo", "~uzamos";
        present_second: "~uzas", "~uzáis";
        present_third: "~uza", "~uzan";
        imperfect_first: "~uzaba", "~uzábamos";
        imperfect_second: "~uzabas", "~uzabais";
        imperfect_third: "~uzaba", "~uzaban";
        preterite_first: "~ucé", "~uzamos";
        preterite_second: "~uzaste", "~uzasteis";
        preterite_third: "~uzó", "~uzaron";
        future_first: "~uzaré", "~uzaremos";
        future_second: "~uzarás", "~uzaréis";
        future_third: "~uzará", "~uzarán";
        conditional_first: "~uzaría", "~uzaríamos";
        conditional_second: "~uzarías", "~uzaríais";
        conditional_third: "~uzaría", "~uzarían";
        present_subjunctive_first: "~uce", "~ucemos";
        present_subjunctive_second: "~uces", "~ucéis";
        present_subjunctive_third: "~uce", "~ucen";
        imperfect_subjunctive_first: "~uzara", "~uzáramos";
        imperfect_subjunctive_second: "~uzaras", "~uzarais";
        imperfect_subjunctive_third: "~uzara", "~uzaran";
        imperfect_subjunctive_se_first: "~uzase", "~uzásemos";
        imperfect_subjunctive_se_second: "~uzases", "~uzaseis";
        imperfect_subjunctive_se_third: "~uzase", "~uzasen";
        future_subjunctive_first: "~uzare", "~uzáremos";
        future_subjunctive_second: "~uzares", "~uzareis";
        future_subjunctive_third: "~uzare", "~uzaren";
        vos: "~uzás", "~ucés", "~uzate";
        imperative: "~úzate", "~úcese", "~ucémonos", "~uzaos", "~úcense";
        negative_imperative: "~uces", "~uce", "~ucemos", "~ucéis", "~ucen";
        infinitive: "~uzarse";
        gerund: "~uzándose";
        participle_masculine: "~uzado", "~uzados";
        participle_feminine: "~uzada", "~uzadas";
    ),
    // Tener: 10 words (tener, sostener, obtener, detener, mantener, entretener).
    conjugation!("tener", 10, "~ener";
        present_first: "~engo", "~enemos";
        present_second: "~ienes", "~enéis";
        present_third: "~iene", "~ienen";
        imperfect_first: "~enía", "~eníamos";
        imperfect_second: "~enías", "~eníais";
        imperfect_third: "~enía", "~enían";
        preterite_first: "~uve", "~uvimos";
        preterite_second: "~uviste", "~uvisteis";
        preterite_third: "~uvo", "~uvieron";
        future_first: "~endré", "~endremos";
        future_second: "~endrás", "~endréis";
        future_third: "~endrá", "~endrán";
        conditional_first: "~endría", "~endríamos";
        conditional_second: "~endrías", "~endríais";
        conditional_third: "~endría", "~endrían";
        present_subjunctive_first: "~enga", "~engamos";
        present_subjunctive_second: "~engas", "~engáis";
        present_subjunctive_third: "~enga", "~engan";
        imperfect_subjunctive_first: "~uviera", "~uviéramos";
        imperfect_subjunctive_second: "~uvieras", "~uvierais";
        imperfect_subjunctive_third: "~uviera", "~uvieran";
        imperfect_subjunctive_se_first: "~uviese", "~uviésemos";
        imperfect_subjunctive_se_second: "~uvieses", "~uvieseis";
        imperfect_subjunctive_se_third: "~uviese", "~uviesen";
        future_subjunctive_first: "~uviere", "~uviéremos";
        future_subjunctive_second: "~uvieres", "~uviereis";
        future_subjunctive_third: "~uviere", "~uvieren";
        vos: "~enés", "~engás", "~ené";
        imperative: "~én", "~enga", "~engamos", "~ened", "~engan";
        negative_imperative: "~engas", "~enga", "~engamos", "~engáis", "~engan";
        infinitive: "~ener";
        gerund: "~eniendo";
        participle_masculine: "~enido", "~enidos";
        participle_feminine: "~enida", "~enidas";
    ),
    // Onducir: 10 words (educir, ducir, troducir, roducir, raducir, producir).
    conjugation!("onducir", 10, "~cir";
        present_first: "~zco", "~cimos";
        present_second: "~ces", "~cís";
        present_third: "~ce", "~cen";
        imperfect_first: "~cía", "~cíamos";
        imperfect_second: "~cías", "~cíais";
        imperfect_third: "~cía", "~cían";
        preterite_first: "~je", "~jimos";
        preterite_second: "~jiste", "~jisteis";
        preterite_third: "~jo", "~jeron";
        future_first: "~ciré", "~ciremos";
        future_second: "~cirás", "~ciréis";
        future_third: "~cirá", "~cirán";
        conditional_first: "~ciría", "~ciríamos";
        conditional_second: "~cirías", "~ciríais";
        conditional_third: "~ciría", "~cirían";
        present_subjunctive_first: "~zca", "~zcamos";
        present_subjunctive_second: "~zcas", "~zcáis";
        present_subjunctive_third: "~zca", "~zcan";
        imperfect_subjunctive_first: "~jera", "~jéramos";
        imperfect_subjunctive_second: "~jeras", "~jerais";
        imperfect_subjunctive_third: "~jera", "~jeran";
        imperfect_subjunctive_se_first: "~jese", "~jésemos";
        imperfect_subjunctive_se_second: "~jeses", "~jeseis";
        imperfect_subjunctive_se_third: "~jese", "~jesen";
        future_subjunctive_first: "~jere", "~jéremos";
        future_subjunctive_second: "~jeres", "~jereis";
        future_subjunctive_third: "~jere", "~jeren";
        vos: "~cís", "~zcás", "~cí";
        imperative: "~ce", "~zca", "~zcamos", "~cid", "~zcan";
        negative_imperative: "~zcas", "~zca", "~zcamos", "~zcáis", "~zcan";
        infinitive: "~cir";
        gerund: "~ciendo";
        participle_masculine: "~cido", "~cidos";
        participle_feminine: "~cida", "~cidas";
    ),
    // Partirse: 10 words (batirse, partirse, esparcirse, salirse, blandirse, carpirse).
    conjugation!("partirse", 10, "~a~irse";
        present_first: "~a~o", "~a~imos";
        present_second: "~a~es", "~a~ís";
        present_third: "~a~e", "~a~en";
        imperfect_first: "~a~ía", "~a~íamos";
        imperfect_second: "~a~ías", "~a~íais";
        imperfect_third: "~a~ía", "~a~ían";
        preterite_first: "~a~í", "~a~imos";
        preterite_second: "~a~iste", "~a~isteis";
        preterite_third: "~a~ió", "~a~ieron";
        future_first: "~a~iré", "~a~iremos";
        future_second: "~a~irás", "~a~iréis";
        future_third: "~a~irá", "~a~irán";
        conditional_first: "~a~iría", "~a~iríamos";
        conditional_second: "~a~irías", "~a~iríais";
        conditional_third: "~a~iría", "~a~irían";
        present_subjunctive_first: "~a~a", "~a~amos";
        present_subjunctive_second: "~a~as", "~a~áis";
        present_subjunctive_third: "~a~a", "~a~an";
        imperfect_subjunctive_first: "~a~iera", "~a~iéramos";
        imperfect_subjunctive_second: "~a~ieras", "~a~ierais";
        imperfect_subjunctive_third: "~a~iera", "~a~ieran";
        imperfect_subjunctive_se_first: "~a~iese", "~a~iésemos";
        imperfect_subjunctive_se_second: "~a~ieses", "~a~ieseis";
        imperfect_subjunctive_se_third: "~a~iese", "~a~iesen";
        future_subjunctive_first: "~a~iere", "~a~iéremos";
        future_subjunctive_second: "~a~ieres", "~a~iereis";
        future_subjunctive_third: "~a~iere", "~a~ieren";
        vos: "~a~ís", "~a~ás", "~a~ite";
        imperative: "~á~ete", "~á~ase", "~a~ámonos", "~a~íos", "~á~anse";
        negative_imperative: "~a~as", "~a~a", "~a~amos", "~a~áis", "~a~an";
        infinitive: "~a~irse";
        gerund: "~a~iéndose";
        participle_masculine: "~a~ido", "~a~idos";
        participle_feminine: "~a~ida", "~a~idas";
    ),
    // Uñuscarse: 10 words (buscarse, ofuscarse, uñuscarse, furcarse, culcarse, rruscarse).
    conjugation!("uñuscarse", 10, "~u~carse";
        present_first: "~u~co", "~u~camos";
        present_second: "~u~cas", "~u~cáis";
        present_third: "~u~ca", "~u~can";
        imperfect_first: "~u~caba", "~u~cábamos";
        imperfect_second: "~u~cabas", "~u~cabais";
        imperfect_third: "~u~caba", "~u~caban";
        preterite_first: "~u~qué", "~u~camos";
        preterite_second: "~u~caste", "~u~casteis";
        preterite_third: "~u~có", "~u~caron";
        future_first: "~u~caré", "~u~caremos";
        future_second: "~u~carás", "~u~caréis";
        future_third: "~u~cará", "~u~carán";
        conditional_first: "~u~caría", "~u~caríamos";
        conditional_second: "~u~carías", "~u~caríais";
        conditional_third: "~u~caría", "~u~carían";
        present_subjunctive_first: "~u~que", "~u~quemos";
        present_subjunctive_second: "~u~ques", "~u~quéis";
        present_subjunctive_third: "~u~que", "~u~quen";
        imperfect_subjunctive_first: "~u~cara", "~u~cáramos";
        imperfect_subjunctive_second: "~u~caras", "~u~carais";
        imperfect_subjunctive_third: "~u~cara", "~u~caran";
        imperfect_subjunctive_se_first: "~u~case", "~u~cásemos";
        imperfect_subjunctive_se_second: "~u~cases", "~u~caseis";
        imperfect_subjunctive_se_third: "~u~case", "~u~casen";
        future_subjunctive_first: "~u~care", "~u~cáremos";
        future_subjunctive_second: "~u~cares", "~u~careis";
        future_subjunctive_third: "~u~care", "~u~caren";
        vos: "~u~cás", "~u~qués", "~u~cate";
        imperative: "~ú~cate", "~ú~quese", "~u~quémonos", "~u~caos", "~ú~quense";
        negative_imperative: "~u~ques", "~u~que", "~u~quemos", "~u~quéis", "~u~quen";
        infinitive: "~u~carse";
        gerund: "~u~cándose";
        participle_masculine: "~u~cado", "~u~cados";
        participle_feminine: "~u~cada", "~u~cadas";
    ),
    // Torcer: 9 words (coercer, vencer, ejercer, torcer, ercer, orcer).
    conjugation!("torcer", 9, "~cer";
        present_first: "~zo", "~cemos";
        present_second: "~ces", "~céis";
        present_third: "~ce", "~cen";
        imperfect_first: "~cía", "~cíamos";
        imperfect_second: "~cías", "~cíais";
        imperfect_third: "~cía", "~cían";
        preterite_first: "~cí", "~cimos";
        preterite_second: "~ciste", "~cisteis";
        preterite_third: "~ció", "~cieron";
        future_first: "~ceré", "~ceremos";
        future_second: "~cerás", "~ceréis";
        future_third: "~cerá", "~cerán";
        conditional_first: "~cería", "~ceríamos";
        conditional_second: "~cerías", "~ceríais";
        conditional_third: "~cería", "~cerían";
        present_subjunctive_first: "~za", "~zamos";
        present_subjunctive_second: "~zas", "~záis";
        present_subjunctive_third: "~za", "~zan";
        imperfect_subjunctive_first: "~ciera", "~ciéramos";
        imperfect_subjunctive_second: "~cieras", "~cierais";
        imperfect_subjunctive_third: "~ciera", "~cieran";
        imperfect_subjunctive_se_first: "~ciese", "~ciésemos";
        imperfect_subjunctive_se_second: "~cieses", "~cieseis";
        imperfect_subjunctive_se_third: "~ciese", "~ciesen";
        future_subjunctive_first: "~ciere", "~ciéremos";
        future_subjunctive_second: "~cieres", "~ciereis";
        future_subjunctive_third: "~ciere", "~cieren";
        vos: "~cés", "~zás", "~cé";
        imperative: "~ce", "~za", "~zamos", "~ced", "~zan";
        negative_imperative: "~zas", "~za", "~zamos", "~záis", "~zan";
        infinitive: "~cer";
        gerund: "~ciendo";
        participle_masculine: "~cido", "~cidos";
        participle_feminine: "~cida", "~cidas";
    ),
    // Rehilar: 9 words (aislar, airar, rehilar, sobrehilar, hijar, feinar).
    conjugation!("rehilar", 9, "~i~ar";
        present_first: "~í~o", "~i~amos";
        present_second: "~í~as", "~i~áis";
        present_third: "~í~a", "~í~an";
        imperfect_first: "~i~aba", "~i~ábamos";
        imperfect_second: "~i~abas", "~i~abais";
        imperfect_third: "~i~aba", "~i~aban";
        preterite_first: "~i~é", "~i~amos";
        preterite_second: "~i~aste", "~i~asteis";
        preterite_third: "~i~ó", "~i~aron";
        future_first: "~i~aré", "~i~aremos";
        future_second: "~i~arás", "~i~aréis";
        future_third: "~i~ará", "~i~arán";
        conditional_first: "~i~aría", "~i~aríamos";
        conditional_second: "~i~arías", "~i~aríais";
        conditional_third: "~i~aría", "~i~arían";
        present_subjunctive_first: "~í~e", "~i~emos";
        present_subjunctive_second: "~í~es", "~i~éis";
        present_subjunctive_third: "~í~e", "~í~en";
        imperfect_subjunctive_first: "~i~ara", "~i~áramos";
        imperfect_subjunctive_second: "~i~aras", "~i~arais";
        imperfect_subjunctive_third: "~i~ara", "~i~aran";
        imperfect_subjunctive_se_first: "~i~ase", "~i~ásemos";
        imperfect_subjunctive_se_second: "~i~ases", "~i~aseis";
        imperfect_subjunctive_se_third: "~i~ase", "~i~asen";
        future_subjunctive_first: "~i~are", "~i~áremos";
        future_subjunctive_second: "~i~ares", "~i~areis";
        future_subjunctive_third: "~i~are", "~i~aren";
        vos: "~i~ás", "~i~és", "~i~á";
        imperative: "~í~a", "~í~e", "~i~emos", "~i~ad", "~í~en";
        negative_imperative: "~í~es", "~í~e", "~i~emos", "~i~éis", "~í~en";
        infinitive: "~i~ar";
        gerund: "~i~ando";
        participle_masculine: "~i~ado", "~i~ados";
        participle_feminine: "~i~ada", "~i~adas";
    ),
    // Rogarse: 9 words (togarse, logarse, drogarse, rogarse, longarse, ersogarse).
    conjugation!("rogarse", 9, "~o~arse";
        present_first: "~o~o", "~o~amos";
        present_second: "~o~as", "~o~áis";
        present_third: "~o~a", "~o~an";
        imperfect_first: "~o~aba", "~o~ábamos";
        imperfect_second: "~o~abas", "~o~abais";
        imperfect_third: "~o~aba", "~o~aban";
        preterite_first: "~o~ué", "~o~amos";
        preterite_second: "~o~aste", "~o~asteis";
        preterite_third: "~o~ó", "~o~aron";
        future_first: "~o~aré", "~o~aremos";
        future_second: "~o~arás", "~o~aréis";
        future_third: "~o~ará", "~o~arán";
        conditional_first: "~o~aría", "~o~aríamos";
        conditional_second: "~o~arías", "~o~aríais";
        conditional_third: "~o~aría", "~o~arían";
        present_subjunctive_first: "~o~ue", "~o~uemos";
        present_subjunctive_second: "~o~ues", "~o~uéis";
        present_subjunctive_third: "~o~ue", "~o~uen";
        imperfect_subjunctive_first: "~o~ara", "~o~áramos";
        imperfect_subjunctive_second: "~o~aras", "~o~arais";
        imperfect_subjunctive_third: "~o~ara", "~o~aran";
        imperfect_subjunctive_se_first: "~o~ase", "~o~ásemos";
        imperfect_subjunctive_se_second: "~o~ases", "~o~aseis";
        imperfect_subjunctive_se_third: "~o~ase", "~o~asen";
        future_subjunctive_first: "~o~are", "~o~áremos";
        future_subjunctive_second: "~o~ares", "~o~areis";
        future_subjunctive_third: "~o~are", "~o~aren";
        vos: "~o~ás", "~o~ués", "~o~ate";
        imperative: "~ó~ate", "~ó~uese", "~o~uémonos", "~o~aos", "~ó~uense";
        negative_imperative: "~o~ues", "~o~ue", "~o~uemos", "~o~uéis", "~o~uen";
        infinitive: "~o~arse";
        gerund: "~o~ándose";
        participle_masculine: "~o~ado", "~o~ados";
        participle_feminine: "~o~ada", "~o~adas";
    ),
    // Ntiguarse: 9 words (achiguarse, averiguarse, ciguarse, ntiguarse, aniguarse, tiguarse).
    conjugation!("ntiguarse", 9, "~i~uarse";
        present_first: "~i~uo", "~i~uamos";
        present_second: "~i~uas", "~i~uáis";
        present_third: "~i~ua", "~i~uan";
        imperfect_first: "~i~uaba", "~i~uábamos";
        imperfect_second: "~i~uabas", "~i~uabais";
        imperfect_third: "~i~uaba", "~i~uaban";
        preterite_first: "~i~üé", "~i~uamos";
        preterite_second: "~i~uaste", "~i~uasteis";
        preterite_third: "~i~uó", "~i~uaron";
        future_first: "~i~uaré", "~i~uaremos";
        future_second: "~i~uarás", "~i~uaréis";
        future_third: "~i~uará", "~i~uarán";
        conditional_first: "~i~uaría", "~i~uaríamos";
        conditional_second: "~i~uarías", "~i~uaríais";
        conditional_third: "~i~uaría", "~i~uarían";
        present_subjunctive_first: "~i~üe", "~i~üemos";
        present_subjunctive_second: "~i~ües", "~i~üéis";
        present_subjunctive_third: "~i~üe", "~i~üen";
        imperfect_subjunctive_first: "~i~uara", "~i~uáramos";
        imperfect_subjunctive_second: "~i~uaras", "~i~uarais";
        imperfect_subjunctive_third: "~i~uara", "~i~uaran";
        imperfect_subjunctive_se_first: "~i~uase", "~i~uásemos";
        imperfect_subjunctive_se_second: "~i~uases", "~i~uaseis";
        imperfect_subjunctive_se_third: "~i~uase", "~i~uasen";
        future_subjunctive_first: "~i~uare", "~i~uáremos";
        future_subjunctive_second: "~i~uares", "~i~uareis";
        future_subjunctive_third: "~i~uare", "~i~uaren";
        vos: "~i~uás", "~i~üés", "~i~uate";
        imperative: "~í~uate", "~í~üese", "~i~üémonos", "~i~uaos", "~í~üense";
        negative_imperative: "~i~ües", "~i~üe", "~i~üemos", "~i~üéis", "~i~üen";
        infinitive: "~i~uarse";
        gerund: "~i~uándose";
        participle_masculine: "~i~uado", "~i~uados";
        participle_feminine: "~i~uada", "~i~uadas";
    ),
    // Rozarse: 9 words (apozarse, escozarse, embozarse, gozarse, rozarse, bozarse).
    conjugation!("rozarse", 9, "~ozarse";
        present_first: "~ozo", "~ozamos";
        present_second: "~ozas", "~ozáis";
        present_third: "~oza", "~ozan";
        imperfect_first: "~ozaba", "~ozábamos";
        imperfect_second: "~ozabas", "~ozabais";
        imperfect_third: "~ozaba", "~ozaban";
        preterite_first: "~océ", "~ozamos";
        preterite_second: "~ozaste", "~ozasteis";
        preterite_third: "~ozó", "~ozaron";
        future_first: "~ozaré", "~ozaremos";
        future_second: "~ozarás", "~ozaréis";
        future_third: "~ozará", "~ozarán";
        conditional_first: "~ozaría", "~ozaríamos";
        conditional_second: "~ozarías", "~ozaríais";
        conditional_third: "~ozaría", "~ozarían";
        present_subjunctive_first: "~oce", "~ocemos";
        present_subjunctive_second: "~oces", "~océis";
        present_subjunctive_third: "~oce", "~ocen";
        imperfect_subjunctive_first: "~ozara", "~ozáramos";
        imperfect_subjunctive_second: "~ozaras", "~ozarais";
        imperfect_subjunctive_third: "~ozara", "~ozaran";
        imperfect_subjunctive_se_first: "~ozase", "~ozásemos";
        imperfect_subjunctive_se_second: "~ozases", "~ozaseis";
        imperfect_subjunctive_se_third: "~ozase", "~ozasen";
        future_subjunctive_first: "~ozare", "~ozáremos";
        future_subjunctive_second: "~ozares", "~ozareis";
        future_subjunctive_third: "~ozare", "~ozaren";
        vos: "~ozás", "~océs", "~ozate";
        imperative: "~ózate", "~ócese", "~océmonos", "~ozaos", "~ócense";
        negative_imperative: "~oces", "~oce", "~ocemos", "~océis", "~ocen";
        infinitive: "~ozarse";
        gerund: "~ozándose";
        participle_masculine: "~ozado", "~ozados";
        participle_feminine: "~ozada", "~ozadas";
    ),
    // Volcarse: 9 words (volcarse, corcarse, coscarse, escoscarse, emboscarse, entroncarse).
    conjugation!("volcarse", 9, "~o~carse";
        present_first: "~o~co", "~o~camos";
        present_second: "~o~cas", "~o~cáis";
        present_third: "~o~ca", "~o~can";
        imperfect_first: "~o~caba", "~o~cábamos";
        imperfect_second: "~o~cabas", "~o~cabais";
        imperfect_third: "~o~caba", "~o~caban";
        preterite_first: "~o~qué", "~o~camos";
        preterite_second: "~o~caste", "~o~casteis";
        preterite_third: "~o~có", "~o~caron";
        future_first: "~o~caré", "~o~caremos";
        future_second: "~o~carás", "~o~caréis";
        future_third: "~o~cará", "~o~carán";
        conditional_first: "~o~caría", "~o~caríamos";
        conditional_second: "~o~carías", "~o~caríais";
        conditional_third: "~o~caría", "~o~carían";
        present_subjunctive_first: "~o~que", "~o~quemos";
        present_subjunctive_second: "~o~ques", "~o~quéis";
        present_subjunctive_third: "~o~que", "~o~quen";
        imperfect_subjunctive_first: "~o~cara", "~o~cáramos";
        imperfect_subjunctive_second: "~o~caras", "~o~carais";
        imperfect_subjunctive_third: "~o~cara", "~o~caran";
        imperfect_subjunctive_se_first: "~o~case", "~o~cásemos";
        imperfect_subjunctive_se_second: "~o~cases", "~o~caseis";
        imperfect_subjunctive_se_third: "~o~case", "~o~casen";
        future_subjunctive_first: "~o~care", "~o~cáremos";
        future_subjunctive_second: "~o~cares", "~o~careis";
        future_subjunctive_third: "~o~care", "~o~caren";
        vos: "~o~cás", "~o~qués", "~o~cate";
        imperative: "~ó~cate", "~ó~quese", "~o~quémonos", "~o~caos", "~ó~quense";
        negative_imperative: "~o~ques", "~o~que", "~o~quemos", "~o~quéis", "~o~quen";
        infinitive: "~o~carse";
        gerund: "~o~cándose";
        participle_masculine: "~o~cado", "~o~cados";
        participle_feminine: "~o~cada", "~o~cadas";
    ),
    // Hacer: 8 words (hacer, facer, rehacer, contrahacer, jacer, tisfacer).
    conjugation!("hacer", 8, "~acer";
        present_first: "~ago", "~acemos";
        present_second: "~aces", "~acéis";
        present_third: "~ace", "~acen";
        imperfect_first: "~acía", "~acíamos";
        imperfect_second: "~acías", "~acíais";
        imperfect_third: "~acía", "~acían";
        preterite_first: "~ice", "~icimos";
        preterite_second: "~iciste", "~icisteis";
        preterite_third: "~izo", "~icieron";
        future_first: "~aré", "~aremos";
        future_second: "~arás", "~aréis";
        future_third: "~ará", "~arán";
        conditional_first: "~aría", "~aríamos";
        conditional_second: "~arías", "~aríais";
        conditional_third: "~aría", "~arían";
        present_subjunctive_first: "~aga", "~agamos";
        present_subjunctive_second: "~agas", "~agáis";
        present_subjunctive_third: "~aga", "~agan";
        imperfect_subjunctive_first: "~iciera", "~iciéramos";
        imperfect_subjunctive_second: "~icieras", "~icierais";
        imperfect_subjunctive_third: "~iciera", "~icieran";
        imperfect_subjunctive_se_first: "~iciese", "~iciésemos";
        imperfect_subjunctive_se_second: "~icieses", "~icieseis";
        imperfect_subjunctive_se_third: "~iciese", "~iciesen";
        future_subjunctive_first: "~iciere", "~iciéremos";
        future_subjunctive_second: "~icieres", "~iciereis";
        future_subjunctive_third: "~iciere", "~icieren";
        vos: "~acés", "~agás", "~acé";
        imperative: "~az", "~aga", "~agamos", "~aced", "~agan";
        negative_imperative: "~agas", "~aga", "~agamos", "~agáis", "~agan";
        infinitive: "~acer";
        gerund: "~aciendo";
        participle_masculine: "~echo", "~echos";
        participle_feminine: "~echa", "~echas";
    ),
    // Ivirse: 8 words (ingrirse, ivirse, igirse, istirse, ibirse, idirse).
    conjugation!("ivirse", 8, "i~irse";
        present_first: "i~o", "i~imos";
        present_second: "i~es", "i~ís";
        present_third: "i~e", "i~en";
        imperfect_first: "i~ía", "i~íamos";
        imperfect_second: "i~ías", "i~íais";
        imperfect_third: "i~ía", "i~ían";
        preterite_first: "i~í", "i~imos";
        preterite_second: "i~iste", "i~isteis";
        preterite_third: "i~ió", "i~ieron";
        future_first: "i~iré", "i~iremos";
        future_second: "i~irás", "i~iréis";
        future_third: "i~irá", "i~irán";
        conditional_first: "i~iría", "i~iríamos";
        conditional_second: "i~irías", "i~iríais";
        conditional_third: "i~iría", "i~irían";
        present_subjunctive_first: "i~a", "i~amos";
        present_subjunctive_second: "i~as", "i~áis";
        present_subjunctive_third: "i~a", "i~an";
        imperfect_subjunctive_first: "i~iera", "i~iéramos";
        imperfect_subjunctive_second: "i~ieras", "i~ierais";
        imperfect_subjunctive_third: "i~iera", "i~ieran";
        imperfect_subjunctive_se_first: "i~iese", "i~iésemos";
        imperfect_subjunctive_se_second: "i~ieses", "i~ieseis";
        imperfect_subjunctive_se_third: "i~iese", "i~iesen";
        future_subjunctive_first: "i~iere", "i~iéremos";
        future_subjunctive_second: "i~ieres", "i~iereis";
        future_subjunctive_third: "i~iere", "i~ieren";
        vos: "i~ís", "i~ás", "i~ite";
        imperative: "í~ete", "í~ase", "i~ámonos", "i~íos", "í~anse";
        negative_imperative: "i~as", "i~a", "i~amos", "i~áis", "i~an";
        infinitive: "i~irse";
        gerund: "i~iéndose";
        participle_masculine: "i~ido", "i~idos";
        participle_feminine: "i~ida", "i~idas";
    ),
    // Amoriscarse: 8 words (ciscarse, fincarse, trincarse, triscarse, hincarse, amoriscarse).
    conjugation!("amoriscarse", 8, "~i~carse";
        present_first: "~i~co", "~i~camos";
        present_second: "~i~cas", "~i~cáis";
        present_third: "~i~ca", "~i~can";
        imperfect_first: "~i~caba", "~i~cábamos";
        imperfect_second: "~i~cabas", "~i~cabais";
        imperfect_third: "~i~caba", "~i~caban";
        preterite_first: "~i~qué", "~i~camos";
        preterite_second: "~i~caste", "~i~casteis";
        preterite_third: "~i~có", "~i~caron";
        future_first: "~i~caré", "~i~caremos";
        future_second: "~i~carás", "~i~caréis";
        future_third: "~i~cará", "~i~carán";
        conditional_first: "~i~caría", "~i~caríamos";
        conditional_second: "~i~carías", "~i~caríais";
        conditional_third: "~i~caría", "~i~carían";
        present_subjunctive_first: "~i~que", "~i~quemos";
        present_subjunctive_second: "~i~ques", "~i~quéis";
        present_subjunctive_third: "~i~que", "~i~quen";
        imperfect_subjunctive_first: "~i~cara", "~i~cáramos";
        imperfect_subjunctive_second: "~i~caras", "~i~carais";
        imperfect_subjunctive_third: "~i~cara", "~i~caran";
        imperfect_subjunctive_se_first: "~i~case", "~i~cásemos";
        imperfect_subjunctive_se_second: "~i~cases", "~i~caseis";
        imperfect_subjunctive_se_third: "~i~case", "~i~casen";
        future_subjunctive_first: "~i~care", "~i~cáremos";
        future_subjunctive_second: "~i~cares", "~i~careis";
        future_subjunctive_third: "~i~care", "~i~caren";
        vos: "~i~cás", "~i~qués", "~i~cate";
        imperative: "~í~cate", "~í~quese", "~i~quémonos", "~i~caos", "~í~quense";
        negative_imperative: "~i~ques", "~i~que", "~i~quemos", "~i~quéis", "~i~quen";
        infinitive: "~i~carse";
        gerund: "~i~cándose";
        participle_masculine: "~i~cado", "~i~cados";
        participle_feminine: "~i~cada", "~i~cadas";
    ),
    // Ribir: 8 words (ribir, cribir, scribir, escribir, rcunscribir, inscribir).
    conjugation!("ribir", 8, "~bir";
        present_first: "~bo", "~bimos";
        present_second: "~bes", "~bís";
        present_third: "~be", "~ben";
        imperfect_first: "~bía", "~bíamos";
        imperfect_second: "~bías", "~bíais";
        imperfect_third: "~bía", "~bían";
        preterite_first: "~bí", "~bimos";
        preterite_second: "~biste", "~bisteis";
        preterite_third: "~bió", "~bieron";
        future_first: "~biré", "~biremos";
        future_second: "~birás", "~biréis";
        future_third: "~birá", "~birán";
        conditional_first: "~biría", "~biríamos";
        conditional_second: "~birías", "~biríais";
        conditional_third: "~biría", "~birían";
        present_subjunctive_first: "~ba", "~bamos";
        present_subjunctive_second: "~bas", "~báis";
        present_subjunctive_third: "~ba", "~ban";
        imperfect_subjunctive_first: "~biera", "~biéramos";
        imperfect_subjunctive_second: "~bieras", "~bierais";
        imperfect_subjunctive_third: "~biera", "~bieran";
        imperfect_subjunctive_se_first: "~biese", "~biésemos";
        imperfect_subjunctive_se_second: "~bieses", "~bieseis";
        imperfect_subjunctive_se_third: "~biese", "~biesen";
        future_subjunctive_first: "~biere", "~biéremos";
        future_subjunctive_second: "~bieres", "~biereis";
        future_subjunctive_third: "~biere", "~bieren";
        vos: "~bís", "~bás", "~bí";
        imperative: "~be", "~ba", "~bamos", "~bid", "~ban";
        negative_imperative: "~bas", "~ba", "~bamos", "~báis", "~ban";
        infinitive: "~bir";
        gerund: "~biendo";
        participle_masculine: "~to|~pto", "~tos|~ptos";
        participle_feminine: "~ta|~pta", "~tas|~ptas";
    ),
    // Bucarse: 8 words (rrucarse, ejucarse, itucarse, nucarse, lucarse, tucarse).
    conjugation!("bucarse", 8, "~ucarse";
        present_first: "~uco", "~ucamos";
        present_second: "~ucas", "~ucáis";
        present_third: "~uca", "~ucan";
        imperfect_first: "~ucaba", "~ucábamos";
        imperfect_second: "~ucabas", "~ucabais";
        imperfect_third: "~ucaba", "~ucaban";
        preterite_first: "~uqué", "~ucamos";
        preterite_second: "~ucaste", "~ucasteis";
        preterite_third: "~ucó", "~ucaron";
        future_first: "~ucaré", "~ucaremos";
        future_second: "~ucarás", "~ucaréis";
        future_third: "~ucará", "~ucarán";
        conditional_first: "~ucaría", "~ucaríamos";
        conditional_second: "~ucarías", "~ucaríais";
        conditional_third: "~ucaría", "~ucarían";
        present_subjunctive_first: "~uque", "~uquemos";
        present_subjunctive_second: "~uques", "~uquéis";
        present_subjunctive_third: "~uque", "~uquen";
        imperfect_subjunctive_first: "~ucara", "~ucáramos";
        imperfect_subjunctive_second: "~ucaras", "~ucarais";
        imperfect_subjunctive_third: "~ucara", "~ucaran";
        imperfect_subjunctive_se_first: "~ucase", "~ucásemos";
        imperfect_subjunctive_se_second: "~ucases", "~ucaseis";
        imperfect_subjunctive_se_third: "~ucase", "~ucasen";
        future_subjunctive_first: "~ucare", "~ucáremos";
        future_subjunctive_second: "~ucares", "~ucareis";
        future_subjunctive_third: "~ucare", "~ucaren";
        vos: "~ucás", "~uqués", "~ucate";
        imperative: "~úcate", "~úquese", "~uquémonos", "~ucaos", "~úquense";
        negative_imperative: "~uques", "~uque", "~uquemos", "~uquéis", "~uquen";
        infinitive: "~ucarse";
        gerund: "~ucándose";
        participle_masculine: "~ucado", "~ucados";
        participle_feminine: "~ucada", "~ucadas";
    ),
    // Atirse: 8 words (aterirse, abrirse, asirse, atirse, andirse, arcirse).
    conjugation!("atirse", 8, "a~irse";
        present_first: "a~o", "a~imos";
        present_second: "a~es", "a~ís";
        present_third: "a~e", "a~en";
        imperfect_first: "a~ía", "a~íamos";
        imperfect_second: "a~ías", "a~íais";
        imperfect_third: "a~ía", "a~ían";
        preterite_first: "a~í", "a~imos";
        preterite_second: "a~iste", "a~isteis";
        preterite_third: "a~ió", "a~ieron";
        future_first: "a~iré", "a~iremos";
        future_second: "a~irás", "a~iréis";
        future_third: "a~irá", "a~irán";
        conditional_first: "a~iría", "a~iríamos";
        conditional_second: "a~irías", "a~iríais";
        conditional_third: "a~iría", "a~irían";
        present_subjunctive_first: "a~a", "a~amos";
        present_subjunctive_second: "a~as", "a~áis";
        present_subjunctive_third: "a~a", "a~an";
        imperfect_subjunctive_first: "a~iera", "a~iéramos";
        imperfect_subjunctive_second: "a~ieras", "a~ierais";
        imperfect_subjunctive_third: "a~iera", "a~ieran";
        imperfect_subjunctive_se_first: "a~iese", "a~iésemos";
        imperfect_subjunctive_se_second: "a~ieses", "a~ieseis";
        imperfect_subjunctive_se_third: "a~iese", "a~iesen";
        future_subjunctive_first: "a~iere", "a~iéremos";
        future_subjunctive_second: "a~ieres", "a~iereis";
        future_subjunctive_third: "a~iere", "a~ieren";
        vos: "a~ís", "a~ás", "a~ite";
        imperative: "á~ete", "á~ase", "a~ámonos", "a~íos", "á~anse";
        negative_imperative: "a~as", "a~a", "a~amos", "a~áis", "a~an";
        infinitive: "a~irse";
        gerund: "a~iéndose";
        participle_masculine: "a~ido", "a~idos";
        participle_feminine: "a~ida", "a~idas";
    ),
    // Refreír: 7 words (reír, sonreír, engreír, refreír, sofreír, peír).
    conjugation!("refreír", 7, "~eír";
        present_first: "~ío", "~eímos";
        present_second: "~íes", "~eís";
        present_third: "~íe", "~íen";
        imperfect_first: "~eía", "~eíamos";
        imperfect_second: "~eías", "~eíais";
        imperfect_third: "~eía", "~eían";
        preterite_first: "~eí", "~eímos";
        preterite_second: "~eíste", "~eísteis";
        preterite_third: "~ió", "~ieron";
        future_first: "~eiré", "~eiremos";
        future_second: "~eirás", "~eiréis";
        future_third: "~eirá", "~eirán";
        conditional_first: "~eiría", "~eiríamos";
        conditional_second: "~eirías", "~eiríais";
        conditional_third: "~eiría", "~eirían";
        present_subjunctive_first: "~ía", "~iamos";
        present_subjunctive_second: "~ías", "~iáis";
        present_subjunctive_third: "~ía", "~ían";
        imperfect_subjunctive_first: "~iera", "~iéramos";
        imperfect_subjunctive_second: "~ieras", "~ierais";
        imperfect_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_se_first: "~iese", "~iésemos";
        imperfect_subjunctive_se_second: "~ieses", "~ieseis";
        imperfect_subjunctive_se_third: "~iese", "~iesen";
        future_subjunctive_first: "~iere", "~iéremos";
        future_subjunctive_second: "~ieres", "~iereis";
        future_subjunctive_third: "~iere", "~ieren";
        vos: "~eís", "~iás", "~eí";
        imperative: "~íe", "~ía", "~iamos", "~eíd", "~ían";
        negative_imperative: "~ías", "~ía", "~iamos", "~iáis", "~ían";
        infinitive: "~eír";
        gerund: "~iendo";
        participle_masculine: "~eído", "~eídos";
        participle_feminine: "~eída", "~eídas";
    ),
    // Colgar: 7 words (colgar, rogar, descolgar, holgar, logar, folgar).
    conjugation!("colgar", 7, "~o~ar";
        present_first: "~ue~o", "~o~amos";
        present_second: "~ue~as", "~o~áis";
        present_third: "~ue~a", "~ue~an";
        imperfect_first: "~o~aba", "~o~ábamos";
        imperfect_second: "~o~abas", "~o~abais";
        imperfect_third: "~o~aba", "~o~aban";
        preterite_first: "~o~ué", "~o~amos";
        preterite_second: "~o~aste", "~o~asteis";
        preterite_third: "~o~ó", "~o~aron";
        future_first: "~o~aré", "~o~aremos";
        future_second: "~o~arás", "~o~aréis";
        future_third: "~o~ará", "~o~arán";
        conditional_first: "~o~aría", "~o~aríamos";
        conditional_second: "~o~arías", "~o~aríais";
        conditional_third: "~o~aría", "~o~arían";
        present_subjunctive_first: "~ue~ue", "~o~uemos";
        present_subjunctive_second: "~ue~ues", "~o~uéis";
        present_subjunctive_third: "~ue~ue", "~ue~uen";
        imperfect_subjunctive_first: "~o~ara", "~o~áramos";
        imperfect_subjunctive_second: "~o~aras", "~o~arais";
        imperfect_subjunctive_third: "~o~ara", "~o~aran";
        imperfect_subjunctive_se_first: "~o~ase", "~o~ásemos";
        imperfect_subjunctive_se_second: "~o~ases", "~o~aseis";
        imperfect_subjunctive_se_third: "~o~ase", "~o~asen";
        future_subjunctive_first: "~o~are", "~o~áremos";
        future_subjunctive_second: "~o~ares", "~o~areis";
        future_subjunctive_third: "~o~are", "~o~aren";
        vos: "~o~ás", "~o~ués", "~o~á";
        imperative: "~ue~a", "~ue~ue", "~o~uemos", "~o~ad", "~ue~uen";
        negative_imperative: "~ue~ues", "~ue~ue", "~o~uemos", "~o~uéis", "~ue~uen";
        infinitive: "~o~ar";
        gerund: "~o~ando";
        participle_masculine: "~o~ado", "~o~ados";
        participle_feminine: "~o~ada", "~o~adas";
    ),
    // Ver: 7 words (ver, ser, prever, entrever, antever, rever).
    conjugation!("ver", 7, "~er";
        present_first: "~eo", "~emos";
        present_second: "~és", "~éis";
        present_third: "~é", "~én";
        imperfect_first: "~eía", "~eíamos";
        imperfect_second: "~eías", "~eíais";
        imperfect_third: "~eía", "~eían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~ió", "~ieron";
        future_first: "~eré", "~eremos";
        future_second: "~erás", "~eréis";
        future_third: "~erá", "~erán";
        conditional_first: "~ería", "~eríamos";
        conditional_second: "~erías", "~eríais";
        conditional_third: "~ería", "~erían";
        present_subjunctive_first: "~ea", "~eamos";
        present_subjunctive_second: "~eas", "~eáis";
        present_subjunctive_third: "~ea", "~ean";
        imperfect_subjunctive_first: "~iera", "~iéramos";
        imperfect_subjunctive_second: "~ieras", "~ierais";
        imperfect_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_se_first: "~iese", "~iésemos";
        imperfect_subjunctive_se_second: "~ieses", "~ieseis";
        imperfect_subjunctive_se_third: "~iese", "~iesen";
        future_subjunctive_first: "~iere", "~iéremos";
        future_subjunctive_second: "~ieres", "~iereis";
        future_subjunctive_third: "~iere", "~ieren";
        vos: "~és", "~eás", "~é";
        imperative: "~é", "~ea", "~eamos", "~ed", "~ean";
        negative_imperative: "~eas", "~ea", "~eamos", "~eáis", "~ean";
        infinitive: "~er";
        gerund: "~iendo";
        participle_masculine: "~isto", "~istos";
        participle_feminine: "~ista", "~istas";
    ),
    // Opezar: 7 words (despezar, opezar, pezar, omenzar, menzar, tropezar).
    conjugation!("opezar", 7, "~~zar";
        present_first: "~i~zo", "~~zamos";
        present_second: "~i~zas", "~~záis";
        present_third: "~i~za", "~i~zan";
        imperfect_first: "~~zaba", "~~zábamos";
        imperfect_second: "~~zabas", "~~zabais";
        imperfect_third: "~~zaba", "~~zaban";
        preterite_first: "~~cé", "~~zamos";
        preterite_second: "~~zaste", "~~zasteis";
        preterite_third: "~~zó", "~~zaron";
        future_first: "~~zaré", "~~zaremos";
        future_second: "~~zarás", "~~zaréis";
        future_third: "~~zará", "~~zarán";
        conditional_first: "~~zaría", "~~zaríamos";
        conditional_second: "~~zarías", "~~zaríais";
        conditional_third: "~~zaría", "~~zarían";
        present_subjunctive_first: "~i~ce", "~~cemos";
        present_subjunctive_second: "~i~ces", "~~céis";
        present_subjunctive_third: "~i~ce", "~i~cen";
        imperfect_subjunctive_first: "~~zara", "~~záramos";
        imperfect_subjunctive_second: "~~zaras", "~~zarais";
        imperfect_subjunctive_third: "~~zara", "~~zaran";
        imperfect_subjunctive_se_first: "~~zase", "~~zásemos";
        imperfect_subjunctive_se_second: "~~zases", "~~zaseis";
        imperfect_subjunctive_se_third: "~~zase", "~~zasen";
        future_subjunctive_first: "~~zare", "~~záremos";
        future_subjunctive_second: "~~zares", "~~zareis";
        future_subjunctive_third: "~~zare", "~~zaren";
        vos: "~~zás", "~~cés", "~~zá";
        imperative: "~i~za", "~i~ce", "~~cemos", "~~zad", "~i~cen";
        negative_imperative: "~i~ces", "~i~ce", "~~cemos", "~~céis", "~i~cen";
        infinitive: "~~zar";
        gerund: "~~zando";
        participle_masculine: "~~zado", "~~zados";
        participle_feminine: "~~zada", "~~zadas";
    ),
    // Aislarse: 7 words (aislarse, airarse, hitarse, hijarse, ohinarse, raillarse).
    conjugation!("aislarse", 7, "~i~arse";
        present_first: "~í~o", "~i~amos";
        present_second: "~í~as", "~i~áis";
        present_third: "~í~a", "~í~an";
        imperfect_first: "~i~aba", "~i~ábamos";
        imperfect_second: "~i~abas", "~i~abais";
        imperfect_third: "~i~aba", "~i~aban";
        preterite_first: "~i~é", "~i~amos";
        preterite_second: "~i~aste", "~i~asteis";
        preterite_third: "~i~ó", "~i~aron";
        future_first: "~i~aré", "~i~aremos";
        future_second: "~i~arás", "~i~aréis";
        future_third: "~i~ará", "~i~arán";
        conditional_first: "~i~aría", "~i~aríamos";
        conditional_second: "~i~arías", "~i~aríais";
        conditional_third: "~i~aría", "~i~arían";
        present_subjunctive_first: "~í~e", "~i~emos";
        present_subjunctive_second: "~í~es", "~i~éis";
        present_subjunctive_third: "~í~e", "~í~en";
        imperfect_subjunctive_first: "~i~ara", "~i~áramos";
        imperfect_subjunctive_second: "~i~aras", "~i~arais";
        imperfect_subjunctive_third: "~i~ara", "~i~aran";
        imperfect_subjunctive_se_first: "~i~ase", "~i~ásemos";
        imperfect_subjunctive_se_second: "~i~ases", "~i~aseis";
        imperfect_subjunctive_se_third: "~i~ase", "~i~asen";
        future_subjunctive_first: "~i~are", "~i~áremos";
        future_subjunctive_second: "~i~ares", "~i~areis";
        future_subjunctive_third: "~i~are", "~i~aren";
        vos: "~i~ás", "~i~és", "~i~ate";
        imperative: "~í~ate", "~í~ese", "~i~émonos", "~i~aos", "~í~ense";
        negative_imperative: "~í~es", "~í~e", "~i~emos", "~i~éis", "~í~en";
        infinitive: "~i~arse";
        gerund: "~i~ándose";
        participle_masculine: "~i~ado", "~i~ados";
        participle_feminine: "~i~ada", "~i~adas";
    ),
    // Mponerse: 7 words (ponerse, yuxtaponerse, oponerse, aponerse, imponerse, mponerse).
    conjugation!("mponerse", 7, "~onerse";
        present_first: "~ongo", "~onemos";
        present_second: "~ones", "~onéis";
        present_third: "~one", "~onen";
        imperfect_first: "~onía", "~oníamos";
        imperfect_second: "~onías", "~oníais";
        imperfect_third: "~onía", "~onían";
        preterite_first: "~use", "~usimos";
        preterite_second: "~usiste", "~usisteis";
        preterite_third: "~uso", "~usieron";
        future_first: "~ondré", "~ondremos";
        future_second: "~ondrás", "~ondréis";
        future_third: "~ondrá", "~ondrán";
        conditional_first: "~ondría", "~ondríamos";
        conditional_second: "~ondrías", "~ondríais";
        conditional_third: "~ondría", "~ondrían";
        present_subjunctive_first: "~onga", "~ongamos";
        present_subjunctive_second: "~ongas", "~ongáis";
        present_subjunctive_third: "~onga", "~ongan";
        imperfect_subjunctive_first: "~usiera", "~usiéramos";
        imperfect_subjunctive_second: "~usieras", "~usierais";
        imperfect_subjunctive_third: "~usiera", "~usieran";
        imperfect_subjunctive_se_first: "~usiese", "~usiésemos";
        imperfect_subjunctive_se_second: "~usieses", "~usieseis";
        imperfect_subjunctive_se_third: "~usiese", "~usiesen";
        future_subjunctive_first: "~usiere", "~usiéremos";
        future_subjunctive_second: "~usieres", "~usiereis";
        future_subjunctive_third: "~usiere", "~usieren";
        vos: "~onés", "~ongás", "~onete";
        imperative: "~onte", "~óngase", "~ongámonos", "~oneos", "~ónganse";
        negative_imperative: "~ongas", "~onga", "~ongamos", "~ongáis", "~ongan";
        infinitive: "~onerse";
        gerund: "~oniéndose";
        participle_masculine: "~uesto", "~uestos";
        participle_feminine: "~uesta", "~uestas";
    ),
    // Regarse: 7 words (regarse, negarse, cegarse, legarse, osegarse, fregarse).
    conjugation!("regarse", 7, "~e~arse";
        present_first: "~ie~o", "~e~amos";
        present_second: "~ie~as", "~e~áis";
        present_third: "~ie~a", "~ie~an";
        imperfect_first: "~e~aba", "~e~ábamos";
        imperfect_second: "~e~abas", "~e~abais";
        imperfect_third: "~e~aba", "~e~aban";
        preterite_first: "~e~ué", "~e~amos";
        preterite_second: "~e~aste", "~e~asteis";
        preterite_third: "~e~ó", "~e~aron";
        future_first: "~e~aré", "~e~aremos";
        future_second: "~e~arás", "~e~aréis";
        future_third: "~e~ará", "~e~arán";
        conditional_first: "~e~aría", "~e~aríamos";
        conditional_second: "~e~arías", "~e~aríais";
        conditional_third: "~e~aría", "~e~arían";
        present_subjunctive_first: "~ie~ue", "~e~uemos";
        present_subjunctive_second: "~ie~ues", "~e~uéis";
        present_subjunctive_third: "~ie~ue", "~ie~uen";
        imperfect_subjunctive_first: "~e~ara", "~e~áramos";
        imperfect_subjunctive_second: "~e~aras", "~e~arais";
        imperfect_subjunctive_third: "~e~ara", "~e~aran";
        imperfect_subjunctive_se_first: "~e~ase", "~e~ásemos";
        imperfect_subjunctive_se_second: "~e~ases", "~e~aseis";
        imperfect_subjunctive_se_third: "~e~ase", "~e~asen";
        future_subjunctive_first: "~e~are", "~e~áremos";
        future_subjunctive_second: "~e~ares", "~e~areis";
        future_subjunctive_third: "~e~are", "~e~aren";
        vos: "~e~ás", "~e~ués", "~e~ate";
        imperative: "~ié~ate", "~ié~uese", "~e~uémonos", "~e~aos", "~ié~uense";
        negative_imperative: "~ie~ues", "~ie~ue", "~e~uemos", "~e~uéis", "~ie~uen";
        infinitive: "~e~arse";
        gerund: "~e~ándose";
        participle_masculine: "~e~ado", "~e~ados";
        participle_feminine: "~e~ada", "~e~adas";
    ),
    // Lfiarse: 7 words (jetiarse, moniarse, rasiarse, iquiarse, niquiarse, nquiarse).
    conjugation!("lfiarse", 7, "~iarse";
        present_first: "~eo", "~iamos";
        present_second: "~eas", "~iáis";
        present_third: "~ea", "~ean";
        imperfect_first: "~iaba", "~iábamos";
        imperfect_second: "~iabas", "~iabais";
        imperfect_third: "~iaba", "~iaban";
        preterite_first: "~ié", "~iamos";
        preterite_second: "~iaste", "~iasteis";
        preterite_third: "~ió", "~iaron";
        future_first: "~iaré", "~iaremos";
        future_second: "~iarás", "~iaréis";
        future_third: "~iará", "~iarán";
        conditional_first: "~iaría", "~iaríamos";
        conditional_second: "~iarías", "~iaríais";
        conditional_third: "~iaría", "~iarían";
        present_subjunctive_first: "~ee", "~iemos";
        present_subjunctive_second: "~ees", "~iéis";
        present_subjunctive_third: "~ee", "~een";
        imperfect_subjunctive_first: "~iara", "~iáramos";
        imperfect_subjunctive_second: "~iaras", "~iarais";
        imperfect_subjunctive_third: "~iara", "~iaran";
        imperfect_subjunctive_se_first: "~iase", "~iásemos";
        imperfect_subjunctive_se_second: "~iases", "~iaseis";
        imperfect_subjunctive_se_third: "~iase", "~iasen";
        future_subjunctive_first: "~iare", "~iáremos";
        future_subjunctive_second: "~iares", "~iareis";
        future_subjunctive_third: "~iare", "~iaren";
        vos: "~iás", "~iés", "~iate";
        imperative: "~éate", "~éese", "~iémonos", "~iaos", "~éense";
        negative_imperative: "~ees", "~ee", "~iemos", "~iéis", "~een";
        infinitive: "~iarse";
        gerund: "~iándose";
        participle_masculine: "~iado", "~iados";
        participle_feminine: "~iada", "~iadas";
    ),
    // Rcunscribirse: 7 words (describirse, ribirse, scribirse, cribirse, rcunscribirse, inscribirse).
    conjugation!("rcunscribirse", 7, "~ibirse";
        present_first: "~ibo", "~ibimos";
        present_second: "~ibes", "~ibís";
        present_third: "~ibe", "~iben";
        imperfect_first: "~ibía", "~ibíamos";
        imperfect_second: "~ibías", "~ibíais";
        imperfect_third: "~ibía", "~ibían";
        preterite_first: "~ibí", "~ibimos";
        preterite_second: "~ibiste", "~ibisteis";
        preterite_third: "~ibió", "~ibieron";
        future_first: "~ibiré", "~ibiremos";
        future_second: "~ibirás", "~ibiréis";
        future_third: "~ibirá", "~ibirán";
        conditional_first: "~ibiría", "~ibiríamos";
        conditional_second: "~ibirías", "~ibiríais";
        conditional_third: "~ibiría", "~ibirían";
        present_subjunctive_first: "~iba", "~ibamos";
        present_subjunctive_second: "~ibas", "~ibáis";
        present_subjunctive_third: "~iba", "~iban";
        imperfect_subjunctive_first: "~ibiera", "~ibiéramos";
        imperfect_subjunctive_second: "~ibieras", "~ibierais";
        imperfect_subjunctive_third: "~ibiera", "~ibieran";
        imperfect_subjunctive_se_first: "~ibiese", "~ibiésemos";
        imperfect_subjunctive_se_second: "~ibieses", "~ibieseis";
        imperfect_subjunctive_se_third: "~ibiese", "~ibiesen";
        future_subjunctive_first: "~ibiere", "~ibiéremos";
        future_subjunctive_second: "~ibieres", "~ibiereis";
        future_subjunctive_third: "~ibiere", "~ibieren";
        vos: "~ibís", "~ibás", "~ibite";
        imperative: "~íbete", "~íbase", "~ibámonos", "~ibíos", "~íbanse";
        negative_imperative: "~ibas", "~iba", "~ibamos", "~ibáis", "~iban";
        infinitive: "~ibirse";
        gerund: "~ibiéndose";
        participle_masculine: "~ito|~ipto", "~itos|~iptos";
        participle_feminine: "~ita|~ipta", "~itas|~iptas";
    ),
    // Enderse: 7 words (enerse, ecerse, eterse, enderse, ederse, elerse).
    conjugation!("enderse", 7, "e~erse";
        present_first: "e~o", "e~emos";
        present_second: "e~es", "e~éis";
        present_third: "e~e", "e~en";
        imperfect_first: "e~ía", "e~íamos";
        imperfect_second: "e~ías", "e~íais";
        imperfect_third: "e~ía", "e~ían";
        preterite_first: "e~í", "e~imos";
        preterite_second: "e~iste", "e~isteis";
        preterite_third: "e~ió", "e~ieron";
        future_first: "e~eré", "e~eremos";
        future_second: "e~erás", "e~eréis";
        future_third: "e~erá", "e~erán";
        conditional_first: "e~ería", "e~eríamos";
        conditional_second: "e~erías", "e~eríais";
        conditional_third: "e~ería", "e~erían";
        present_subjunctive_first: "e~a", "e~amos";
        present_subjunctive_second: "e~as", "e~áis";
        present_subjunctive_third: "e~a", "e~an";
        imperfect_subjunctive_first: "e~iera", "e~iéramos";
        imperfect_subjunctive_second: "e~ieras", "e~ierais";
        imperfect_subjunctive_third: "e~iera", "e~ieran";
        imperfect_subjunctive_se_first: "e~iese", "e~iésemos";
        imperfect_subjunctive_se_second: "e~ieses", "e~ieseis";
        imperfect_subjunctive_se_third: "e~iese", "e~iesen";
        future_subjunctive_first: "e~iere", "e~iéremos";
        future_subjunctive_second: "e~ieres", "e~iereis";
        future_subjunctive_third: "e~iere", "e~ieren";
        vos: "e~és", "e~ás", "e~ete";
        imperative: "é~ete", "é~ase", "e~ámonos", "e~eos", "é~anse";
        negative_imperative: "e~as", "e~a", "e~amos", "e~áis", "e~an";
        infinitive: "e~erse";
        gerund: "e~iéndose";
        participle_masculine: "e~ido", "e~idos";
        participle_feminine: "e~ida", "e~idas";
    ),
    // Aer: 6 words (atraer, retraer, aer, straer, traer, ntrotraer).
    conjugation!("aer", 6, "~er";
        present_first: "~igo", "~emos";
        present_second: "~es", "~éis";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~je", "~jimos";
        preterite_second: "~jiste", "~jisteis";
        preterite_third: "~jo", "~jeron";
        future_first: "~eré", "~eremos";
        future_second: "~erás", "~eréis";
        future_third: "~erá", "~erán";
        conditional_first: "~ería", "~eríamos";
        conditional_second: "~erías", "~eríais";
        conditional_third: "~ería", "~erían";
        present_subjunctive_first: "~iga", "~igamos";
        present_subjunctive_second: "~igas", "~igáis";
        present_subjunctive_third: "~iga", "~igan";
        imperfect_subjunctive_first: "~jera", "~jéramos";
        imperfect_subjunctive_second: "~jeras", "~jerais";
        imperfect_subjunctive_third: "~jera", "~jeran";
        imperfect_subjunctive_se_first: "~jese", "~jésemos";
        imperfect_subjunctive_se_second: "~jeses", "~jeseis";
        imperfect_subjunctive_se_third: "~jese", "~jesen";
        future_subjunctive_first: "~jere", "~jéremos";
        future_subjunctive_second: "~jeres", "~jereis";
        future_subjunctive_third: "~jere", "~jeren";
        vos: "~és", "~igás", "~é";
        imperative: "~e", "~iga", "~igamos", "~ed", "~igan";
        negative_imperative: "~igas", "~iga", "~igamos", "~igáis", "~igan";
        infinitive: "~er";
        gerund: "~yendo";
        participle_masculine: "~ído", "~ídos";
        participle_feminine: "~ída", "~ídas";
    ),
    // Aunarse: 6 words (ahusarse, aunarse, rehurtarse, auparse, husarse, humarse).
    conjugation!("aunarse", 6, "~u~arse";
        present_first: "~ú~o", "~u~amos";
        present_second: "~ú~as", "~u~áis";
        present_third: "~ú~a", "~ú~an";
        imperfect_first: "~u~aba", "~u~ábamos";
        imperfect_second: "~u~abas", "~u~abais";
        imperfect_third: "~u~aba", "~u~aban";
        preterite_first: "~u~é", "~u~amos";
        preterite_second: "~u~aste", "~u~asteis";
        preterite_third: "~u~ó", "~u~aron";
        future_first: "~u~aré", "~u~aremos";
        future_second: "~u~arás", "~u~aréis";
        future_third: "~u~ará", "~u~arán";
        conditional_first: "~u~aría", "~u~aríamos";
        conditional_second: "~u~arías", "~u~aríais";
        conditional_third: "~u~aría", "~u~arían";
        present_subjunctive_first: "~ú~e", "~u~emos";
        present_subjunctive_second: "~ú~es", "~u~éis";
        present_subjunctive_third: "~ú~e", "~ú~en";
        imperfect_subjunctive_first: "~u~ara", "~u~áramos";
        imperfect_subjunctive_second: "~u~aras", "~u~arais";
        imperfect_subjunctive_third: "~u~ara", "~u~aran";
        imperfect_subjunctive_se_first: "~u~ase", "~u~ásemos";
        imperfect_subjunctive_se_second: "~u~ases", "~u~aseis";
        imperfect_subjunctive_se_third: "~u~ase", "~u~asen";
        future_subjunctive_first: "~u~are", "~u~áremos";
        future_subjunctive_second: "~u~ares", "~u~areis";
        future_subjunctive_third: "~u~are", "~u~aren";
        vos: "~u~ás", "~u~és", "~u~ate";
        imperative: "~ú~ate", "~ú~ese", "~u~émonos", "~u~aos", "~ú~ense";
        negative_imperative: "~ú~es", "~ú~e", "~u~emos", "~u~éis", "~ú~en";
        infinitive: "~u~arse";
        gerund: "~u~ándose";
        participle_masculine: "~u~ado", "~u~ados";
        participle_feminine: "~u~ada", "~u~adas";
    ),
    // Eblar: 6 words (ertar, endar, embrar, eldar, eblar, erdar).
    conjugation!("eblar", 6, "~ar";
        present_first: "i~o", "~amos";
        present_second: "i~as", "~áis";
        present_third: "i~a", "i~an";
        imperfect_first: "~aba", "~ábamos";
        imperfect_second: "~abas", "~abais";
        imperfect_third: "~aba", "~aban";
        preterite_first: "~é", "~amos";
        preterite_second: "~aste", "~asteis";
        preterite_third: "~ó", "~aron";
        future_first: "~aré", "~aremos";
        future_second: "~arás", "~aréis";
        future_third: "~ará", "~arán";
        conditional_first: "~aría", "~aríamos";
        conditional_second: "~arías", "~aríais";
        conditional_third: "~aría", "~arían";
        present_subjunctive_first: "i~e", "~emos";
        present_subjunctive_second: "i~es", "~éis";
        present_subjunctive_third: "i~e", "i~en";
        imperfect_subjunctive_first: "~ara", "~áramos";
        imperfect_subjunctive_second: "~aras", "~arais";
        imperfect_subjunctive_third: "~ara", "~aran";
        imperfect_subjunctive_se_first: "~ase", "~ásemos";
        imperfect_subjunctive_se_second: "~ases", "~aseis";
        imperfect_subjunctive_se_third: "~ase", "~asen";
        future_subjunctive_first: "~are", "~áremos";
        future_subjunctive_second: "~ares", "~areis";
        future_subjunctive_third: "~are", "~aren";
        vos: "~ás", "~és", "~á";
        imperative: "i~a", "i~e", "~emos", "~ad", "i~en";
        negative_imperative: "i~es", "i~e", "~emos", "~éis", "i~en";
        infinitive: "~ar";
        gerund: "~ando";
        participle_masculine: "~ado", "~ados";
        participle_feminine: "~ada", "~adas";
    ),
    // Ducirse: 6 words (reproducirse, conducirse, producirse, ducirse, roducirse, troducirse).
    conjugation!("ducirse", 6, "~ucirse";
        present_first: "~uzco", "~ucimos";
        present_second: "~uces", "~ucís";
        present_third: "~uce", "~ucen";
        imperfect_first: "~ucía", "~ucíamos";
        imperfect_second: "~ucías", "~ucíais";
        imperfect_third: "~ucía", "~ucían";
        preterite_first: "~uje", "~ujimos";
        preterite_second: "~ujiste", "~ujisteis";
        preterite_third: "~ujo", "~ujeron";
        future_first: "~uciré", "~uciremos";
        future_second: "~ucirás", "~uciréis";
        future_third: "~ucirá", "~ucirán";
        conditional_first: "~uciría", "~uciríamos";
        conditional_second: "~ucirías", "~uciríais";
        conditional_third: "~uciría", "~ucirían";
        present_subjunctive_first: "~uzca", "~uzcamos";
        present_subjunctive_second: "~uzcas", "~uzcáis";
        present_subjunctive_third: "~uzca", "~uzcan";
        imperfect_subjunctive_first: "~ujera", "~ujéramos";
        imperfect_subjunctive_second: "~ujeras", "~ujerais";
        imperfect_subjunctive_third: "~ujera", "~ujeran";
        imperfect_subjunctive_se_first: "~ujese", "~ujésemos";
        imperfect_subjunctive_se_second: "~ujeses", "~ujeseis";
        imperfect_subjunctive_se_third: "~ujese", "~ujesen";
        future_subjunctive_first: "~ujere", "~ujéremos";
        future_subjunctive_second: "~ujeres", "~ujereis";
        future_subjunctive_third: "~ujere", "~ujeren";
        vos: "~ucís", "~uzcás", "~ucite";
        imperative: "~úcete", "~úzcase", "~uzcámonos", "~ucíos", "~úzcanse";
        negative_imperative: "~uzcas", "~uzca", "~uzcamos", "~uzcáis", "~uzcan";
        infinitive: "~ucirse";
        gerund: "~uciéndose";
        participle_masculine: "~ucido", "~ucidos";
        participle_feminine: "~ucida", "~ucidas";
    ),
    // Egarse: 6 words (egarse, endarse, ebrarse, erdarse, embrarse, eblarse).
    conjugation!("egarse", 6, "e~arse";
        present_first: "ie~o", "e~amos";
        present_second: "ie~as", "e~áis";
        present_third: "ie~a", "ie~an";
        imperfect_first: "e~aba", "e~ábamos";
        imperfect_second: "e~abas", "e~abais";
        imperfect_third: "e~aba", "e~aban";
        preterite_first: "e~é", "e~amos";
        preterite_second: "e~aste", "e~asteis";
        preterite_third: "e~ó", "e~aron";
        future_first: "e~aré", "e~aremos";
        future_second: "e~arás", "e~aréis";
        future_third: "e~ará", "e~arán";
        conditional_first: "e~aría", "e~aríamos";
        conditional_second: "e~arías", "e~aríais";
        conditional_third: "e~aría", "e~arían";
        present_subjunctive_first: "ie~e", "e~emos";
        present_subjunctive_second: "ie~es", "e~éis";
        present_subjunctive_third: "ie~e", "ie~en";
        imperfect_subjunctive_first: "e~ara", "e~áramos";
        imperfect_subjunctive_second: "e~aras", "e~arais";
        imperfect_subjunctive_third: "e~ara", "e~aran";
        imperfect_subjunctive_se_first: "e~ase", "e~ásemos";
        imperfect_subjunctive_se_second: "e~ases", "e~aseis";
        imperfect_subjunctive_se_third: "e~ase", "e~asen";
        future_subjunctive_first: "e~are", "e~áremos";
        future_subjunctive_second: "e~ares", "e~areis";
        future_subjunctive_third: "e~are", "e~aren";
        vos: "e~ás", "e~és", "e~ate";
        imperative: "ié~ate", "ié~ese", "e~émonos", "e~aos", "ié~ense";
        negative_imperative: "ie~es", "ie~e", "e~emos", "e~éis", "ie~en";
        infinitive: "e~arse";
        gerund: "e~ándose";
        participle_masculine: "e~ado", "e~ados";
        participle_feminine: "e~ada", "e~adas";
    ),
    // Argarse: 6 words (asgarse, argarse, angarse, agarse, aigarse, algarse).
    conjugation!("argarse", 6, "a~arse";
        present_first: "a~o", "a~amos";
        present_second: "a~as", "a~áis";
        present_third: "a~a", "a~an";
        imperfect_first: "a~aba", "a~ábamos";
        imperfect_second: "a~abas", "a~abais";
        imperfect_third: "a~aba", "a~aban";
        preterite_first: "a~ué", "a~amos";
        preterite_second: "a~aste", "a~asteis";
        preterite_third: "a~ó", "a~aron";
        future_first: "a~aré", "a~aremos";
        future_second: "a~arás", "a~aréis";
        future_third: "a~ará", "a~arán";
        conditional_first: "a~aría", "a~aríamos";
        conditional_second: "a~arías", "a~aríais";
        conditional_third: "a~aría", "a~arían";
        present_subjunctive_first: "a~ue", "a~uemos";
        present_subjunctive_second: "a~ues", "a~uéis";
        present_subjunctive_third: "a~ue", "a~uen";
        imperfect_subjunctive_first: "a~ara", "a~áramos";
        imperfect_subjunctive_second: "a~aras", "a~arais";
        imperfect_subjunctive_third: "a~ara", "a~aran";
        imperfect_subjunctive_se_first: "a~ase", "a~ásemos";
        imperfect_subjunctive_se_second: "a~ases", "a~aseis";
        imperfect_subjunctive_se_third: "a~ase", "a~asen";
        future_subjunctive_first: "a~are", "a~áremos";
        future_subjunctive_second: "a~ares", "a~areis";
        future_subjunctive_third: "a~are", "a~aren";
        vos: "a~ás", "a~ués", "a~ate";
        imperative: "á~ate", "á~uese", "a~uémonos", "a~aos", "á~uense";
        negative_imperative: "a~ues", "a~ue", "a~uemos", "a~uéis", "a~uen";
        infinitive: "a~arse";
        gerund: "a~ándose";
        participle_masculine: "a~ado", "a~ados";
        participle_feminine: "a~ada", "a~adas";
    ),
    // Fiar: 5 words (liar, fiar, criar, ciar, piar).
    conjugation!("fiar", 5, "~iar";
        present_first: "~ío", "~iamos";
        present_second: "~ías", "~iais";
        present_third: "~ía", "~ían";
        imperfect_first: "~iaba", "~iábamos";
        imperfect_second: "~iabas", "~iabais";
        imperfect_third: "~iaba", "~iaban";
        preterite_first: "~ie", "~iamos";
        preterite_second: "~iaste", "~iasteis";
        preterite_third: "~io", "~iaron";
        future_first: "~iaré", "~iaremos";
        future_second: "~iarás", "~iaréis";
        future_third: "~iará", "~iarán";
        conditional_first: "~iaría", "~iaríamos";
        conditional_second: "~iarías", "~iaríais";
        conditional_third: "~iaría", "~iarían";
        present_subjunctive_first: "~íe", "~iemos";
        present_subjunctive_second: "~íes", "~ieis";
        present_subjunctive_third: "~íe", "~íen";
        imperfect_subjunctive_first: "~iara", "~iáramos";
        imperfect_subjunctive_second: "~iaras", "~iarais";
        imperfect_subjunctive_third: "~iara", "~iaran";
        imperfect_subjunctive_se_first: "~iase", "~iásemos";
        imperfect_subjunctive_se_second: "~iases", "~iaseis";
        imperfect_subjunctive_se_third: "~iase", "~iasen";
        future_subjunctive_first: "~iare", "~iáremos";
        future_subjunctive_second: "~iares", "~iareis";
        future_subjunctive_third: "~iare", "~iaren";
        vos: "~ias", "~ies", "~ia";
        imperative: "~ía", "~íe", "~iemos", "~iad", "~íen";
        negative_imperative: "~íes", "~íe", "~iemos", "~ieis", "~íen";
        infinitive: "~iar";
        gerund: "~iando";
        participle_masculine: "~iado", "~iados";
        participle_feminine: "~iada", "~iadas";
    ),
    // Evacuar: 5 words (evacuar, cuar, ecuar, adecuar, miscuar).
    conjugation!("evacuar", 5, "~uar";
        present_first: "~uo|~úo", "~uamos";
        present_second: "~uas|~úas", "~uáis";
        present_third: "~ua|~úa", "~uan|~úan";
        imperfect_first: "~uaba", "~uábamos";
        imperfect_second: "~uabas", "~uabais";
        imperfect_third: "~uaba", "~uaban";
        preterite_first: "~ué", "~uamos";
        preterite_second: "~uaste", "~uasteis";
        preterite_third: "~uó", "~uaron";
        future_first: "~uaré", "~uaremos";
        future_second: "~uarás", "~uaréis";
        future_third: "~uará", "~uarán";
        conditional_first: "~uaría", "~uaríamos";
        conditional_second: "~uarías", "~uaríais";
        conditional_third: "~uaría", "~uarían";
        present_subjunctive_first: "~ue|~úe", "~uemos";
        present_subjunctive_second: "~ues|~úes", "~uéis";
        present_subjunctive_third: "~ue|~úe", "~uen|~úen";
        imperfect_subjunctive_first: "~uara", "~uáramos";
        imperfect_subjunctive_second: "~uaras", "~uarais";
        imperfect_subjunctive_third: "~uara", "~uaran";
        imperfect_subjunctive_se_first: "~uase", "~uásemos";
        imperfect_subjunctive_se_second: "~uases", "~uaseis";
        imperfect_subjunctive_se_third: "~uase", "~uasen";
        future_subjunctive_first: "~uare", "~uáremos";
        future_subjunctive_second: "~uares", "~uareis";
        future_subjunctive_third: "~uare", "~uaren";
        vos: "~uás", "~ués", "~uá";
        imperative: "~ua|~úa", "~ue|~úe", "~uemos", "~uad", "~uen|~úen";
        negative_imperative: "~ues|~úes", "~ue|~úe", "~uemos", "~uéis", "~uen|~úen";
        infinitive: "~uar";
        gerund: "~uando";
        participle_masculine: "~uado", "~uados";
        participle_feminine: "~uada", "~uadas";
    ),
    // Ceñir: 5 words (teñir, reñir, ceñir, heñir, treñir).
    conjugation!("ceñir", 5, "~eñir";
        present_first: "~iño", "~eñimos";
        present_second: "~iñes", "~eñís";
        present_third: "~iñe", "~iñen";
        imperfect_first: "~eñía", "~eñíamos";
        imperfect_second: "~eñías", "~eñíais";
        imperfect_third: "~eñía", "~eñían";
        preterite_first: "~eñí", "~eñimos";
        preterite_second: "~eñiste", "~eñisteis";
        preterite_third: "~iñó", "~iñeron";
        future_first: "~eñiré", "~eñiremos";
        future_second: "~eñirás", "~eñiréis";
        future_third: "~eñirá", "~eñirán";
        conditional_first: "~eñiría", "~eñiríamos";
        conditional_second: "~eñirías", "~eñiríais";
        conditional_third: "~eñiría", "~eñirían";
        present_subjunctive_first: "~iña", "~iñamos";
        present_subjunctive_second: "~iñas", "~iñáis";
        present_subjunctive_third: "~iña", "~iñan";
        imperfect_subjunctive_first: "~iñera", "~iñéramos";
        imperfect_subjunctive_second: "~iñeras", "~iñerais";
        imperfect_subjunctive_third: "~iñera", "~iñeran";
        imperfect_subjunctive_se_first: "~iñese", "~iñésemos";
        imperfect_subjunctive_se_second: "~iñeses", "~iñeseis";
        imperfect_subjunctive_se_third: "~iñese", "~iñesen";
        future_subjunctive_first: "~iñere", "~iñéremos";
        future_subjunctive_second: "~iñeres", "~iñereis";
        future_subjunctive_third: "~iñere", "~iñeren";
        vos: "~eñís", "~iñás", "~eñí";
        imperative: "~iñe", "~iña", "~iñamos", "~eñid", "~iñan";
        negative_imperative: "~iñas", "~iña", "~iñamos", "~iñáis", "~iñan";
        infinitive: "~eñir";
        gerund: "~iñendo";
        participle_masculine: "~eñido", "~eñidos";
        participle_feminine: "~eñida", "~eñidas";
    ),
    // Agorar: 5 words (agorar, engorar, gorar, gollar, golmar).
    conjugation!("agorar", 5, "~o~ar";
        present_first: "~üe~o", "~o~amos";
        present_second: "~üe~as", "~o~áis";
        present_third: "~üe~a", "~üe~an";
        imperfect_first: "~o~aba", "~o~ábamos";
        imperfect_second: "~o~abas", "~o~abais";
        imperfect_third: "~o~aba", "~o~aban";
        preterite_first: "~o~é", "~o~amos";
        preterite_second: "~o~aste", "~o~asteis";
        preterite_third: "~o~ó", "~o~aron";
        future_first: "~o~aré", "~o~aremos";
        future_second: "~o~arás", "~o~aréis";
        future_third: "~o~ará", "~o~arán";
        conditional_first: "~o~aría", "~o~aríamos";
        conditional_second: "~o~arías", "~o~aríais";
        conditional_third: "~o~aría", "~o~arían";
        present_subjunctive_first: "~üe~e", "~o~emos";
        present_subjunctive_second: "~üe~es", "~o~éis";
        present_subjunctive_third: "~üe~e", "~üe~en";
        imperfect_subjunctive_first: "~o~ara", "~o~áramos";
        imperfect_subjunctive_second: "~o~aras", "~o~arais";
        imperfect_subjunctive_third: "~o~ara", "~o~aran";
        imperfect_subjunctive_se_first: "~o~ase", "~o~ásemos";
        imperfect_subjunctive_se_second: "~o~ases", "~o~aseis";
        imperfect_subjunctive_se_third: "~o~ase", "~o~asen";
        future_subjunctive_first: "~o~are", "~o~áremos";
        future_subjunctive_second: "~o~ares", "~o~areis";
        future_subjunctive_third: "~o~are", "~o~aren";
        vos: "~o~ás", "~o~és", "~o~á";
        imperative: "~üe~a", "~üe~e", "~o~emos", "~o~ad", "~üe~en";
        negative_imperative: "~üe~es", "~üe~e", "~o~emos", "~o~éis", "~üe~en";
        infinitive: "~o~ar";
        gerund: "~o~ando";
        participle_masculine: "~o~ado", "~o~ados";
        participle_feminine: "~o~ada", "~o~adas";
    ),
    // Ñer: 5 words (jier, creyer, ñer, eller, ller).
    conjugation!("ñer", 5, "~er";
        present_first: "~o", "~emos";
        present_second: "~es", "~éis";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~ó", "~eron";
        future_first: "~eré", "~eremos";
        future_second: "~erás", "~eréis";
        future_third: "~erá", "~erán";
        conditional_first: "~ería", "~eríamos";
        conditional_second: "~erías", "~eríais";
        conditional_third: "~ería", "~erían";
        present_subjunctive_first: "~a", "~amos";
        present_subjunctive_second: "~as", "~áis";
        present_subjunctive_third: "~a", "~an";
        imperfect_subjunctive_first: "~era", "~éramos";
        imperfect_subjunctive_second: "~eras", "~erais";
        imperfect_subjunctive_third: "~era", "~eran";
        imperfect_subjunctive_se_first: "~ese", "~ésemos";
        imperfect_subjunctive_se_second: "~eses", "~eseis";
        imperfect_subjunctive_se_third: "~ese", "~esen";
        future_subjunctive_first: "~ere", "~éremos";
        future_subjunctive_second: "~eres", "~ereis";
        future_subjunctive_third: "~ere", "~eren";
        vos: "~és", "~ás", "~é";
        imperative: "~e", "~a", "~amos", "~ed", "~an";
        negative_imperative: "~as", "~a", "~amos", "~áis", "~an";
        infinitive: "~er";
        gerund: "~endo";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Frescarse: 5 words (cercarse, mercarse, uercarse, rescarse, frescarse).
    conjugation!("frescarse", 5, "~e~carse";
        present_first: "~e~co", "~e~camos";
        present_second: "~e~cas", "~e~cáis";
        present_third: "~e~ca", "~e~can";
        imperfect_first: "~e~caba", "~e~cábamos";
        imperfect_second: "~e~cabas", "~e~cabais";
        imperfect_third: "~e~caba", "~e~caban";
        preterite_first: "~e~qué", "~e~camos";
        preterite_second: "~e~caste", "~e~casteis";
        preterite_third: "~e~có", "~e~caron";
        future_first: "~e~caré", "~e~caremos";
        future_second: "~e~carás", "~e~caréis";
        future_third: "~e~cará", "~e~carán";
        conditional_first: "~e~caría", "~e~caríamos";
        conditional_second: "~e~carías", "~e~caríais";
        conditional_third: "~e~caría", "~e~carían";
        present_subjunctive_first: "~e~que", "~e~quemos";
        present_subjunctive_second: "~e~ques", "~e~quéis";
        present_subjunctive_third: "~e~que", "~e~quen";
        imperfect_subjunctive_first: "~e~cara", "~e~cáramos";
        imperfect_subjunctive_second: "~e~caras", "~e~carais";
        imperfect_subjunctive_third: "~e~cara", "~e~caran";
        imperfect_subjunctive_se_first: "~e~case", "~e~cásemos";
        imperfect_subjunctive_se_second: "~e~cases", "~e~caseis";
        imperfect_subjunctive_se_third: "~e~case", "~e~casen";
        future_subjunctive_first: "~e~care", "~e~cáremos";
        future_subjunctive_second: "~e~cares", "~e~careis";
        future_subjunctive_third: "~e~care", "~e~caren";
        vos: "~e~cás", "~e~qués", "~e~cate";
        imperative: "~é~cate", "~é~quese", "~e~quémonos", "~e~caos", "~é~quense";
        negative_imperative: "~e~ques", "~e~que", "~e~quemos", "~e~quéis", "~e~quen";
        infinitive: "~e~carse";
        gerund: "~e~cándose";
        participle_masculine: "~e~cado", "~e~cados";
        participle_feminine: "~e~cada", "~e~cadas";
    ),
    // Cernirse: 5 words (hespirse, cernirse, edernirse, negrirse, mergirse).
    conjugation!("cernirse", 5, "~e~irse";
        present_first: "~e~o", "~e~imos";
        present_second: "~e~es", "~e~ís";
        present_third: "~e~e", "~e~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~e~í", "~e~imos";
        preterite_second: "~e~iste", "~e~isteis";
        preterite_third: "~e~ió", "~e~ieron";
        future_first: "~e~iré", "~e~iremos";
        future_second: "~e~irás", "~e~iréis";
        future_third: "~e~irá", "~e~irán";
        conditional_first: "~e~iría", "~e~iríamos";
        conditional_second: "~e~irías", "~e~iríais";
        conditional_third: "~e~iría", "~e~irían";
        present_subjunctive_first: "~e~a", "~e~amos";
        present_subjunctive_second: "~e~as", "~e~áis";
        present_subjunctive_third: "~e~a", "~e~an";
        imperfect_subjunctive_first: "~e~iera", "~e~iéramos";
        imperfect_subjunctive_second: "~e~ieras", "~e~ierais";
        imperfect_subjunctive_third: "~e~iera", "~e~ieran";
        imperfect_subjunctive_se_first: "~e~iese", "~e~iésemos";
        imperfect_subjunctive_se_second: "~e~ieses", "~e~ieseis";
        imperfect_subjunctive_se_third: "~e~iese", "~e~iesen";
        future_subjunctive_first: "~e~iere", "~e~iéremos";
        future_subjunctive_second: "~e~ieres", "~e~iereis";
        future_subjunctive_third: "~e~iere", "~e~ieren";
        vos: "~e~ís", "~e~ás", "~e~ite";
        imperative: "~é~ete", "~é~ase", "~e~ámonos", "~e~íos", "~é~anse";
        negative_imperative: "~e~as", "~e~a", "~e~amos", "~e~áis", "~e~an";
        infinitive: "~e~irse";
        gerund: "~e~iéndose";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Juilar: 5 words (clillar, uclillar, juilar, ulismar, uitar).
    conjugation!("juilar", 5, "~i~ar";
        present_first: "~i~o", "~i~amos";
        present_second: "~i~as", "~i~áis";
        present_third: "~i~a", "~i~an";
        imperfect_first: "~i~aba", "~i~ábamos";
        imperfect_second: "~i~abas", "~i~abais";
        imperfect_third: "~i~aba", "~i~aban";
        preterite_first: "~i~é", "~i~amos";
        preterite_second: "~i~aste", "~i~asteis";
        preterite_third: "~i~ó", "~i~aron";
        future_first: "~i~aré", "~i~aremos";
        future_second: "~i~arás", "~i~aréis";
        future_third: "~i~ará", "~i~arán";
        conditional_first: "~i~aría", "~i~aríamos";
        conditional_second: "~i~arías", "~i~aríais";
        conditional_third: "~i~aría", "~i~arían";
        present_subjunctive_first: "~i~e", "~i~emos";
        present_subjunctive_second: "~i~es", "~i~éis";
        present_subjunctive_third: "~i~e", "~i~en";
        imperfect_subjunctive_first: "~i~ara", "~i~áramos";
        imperfect_subjunctive_second: "~i~aras", "~i~arais";
        imperfect_subjunctive_third: "~i~ara", "~i~aran";
        imperfect_subjunctive_se_first: "~i~ase", "~i~ásemos";
        imperfect_subjunctive_se_second: "~i~ases", "~i~aseis";
        imperfect_subjunctive_se_third: "~i~ase", "~i~asen";
        future_subjunctive_first: "~i~are", "~i~áremos";
        future_subjunctive_second: "~i~ares", "~i~areis";
        future_subjunctive_third: "~i~are", "~i~aren";
        vos: "~i~ás", "~i~és", "~i~ate";
        imperative: "~í~ate", "~í~ese", "~i~émonos", "~i~aos", "~í~ense";
        negative_imperative: "~i~es", "~i~e", "~i~emos", "~i~éis", "~i~en";
        infinitive: "~i~arse";
        gerund: "~i~ándose";
        participle_masculine: "~i~ado", "~i~ados";
        participle_feminine: "~i~ada", "~i~adas";
    ),
    // Pedir: 4 words (pedir, despedir, medir, expedir).
    conjugation!("pedir", 4, "~edir";
        present_first: "~ido", "~edimos";
        present_second: "~ides", "~edís";
        present_third: "~ide", "~iden";
        imperfect_first: "~edía", "~edíamos";
        imperfect_second: "~edías", "~edíais";
        imperfect_third: "~edía", "~edían";
        preterite_first: "~edí", "~edimos";
        preterite_second: "~ediste", "~edisteis";
        preterite_third: "~idió", "~idieron";
        future_first: "~ediré", "~ediremos";
        future_second: "~edirás", "~ediréis";
        future_third: "~edirá", "~edirán";
        conditional_first: "~ediría", "~ediríamos";
        conditional_second: "~edirías", "~ediríais";
        conditional_third: "~ediría", "~edirían";
        present_subjunctive_first: "~ida", "~idamos";
        present_subjunctive_second: "~idas", "~idáis";
        present_subjunctive_third: "~ida", "~idan";
        imperfect_subjunctive_first: "~idiera", "~idiéramos";
        imperfect_subjunctive_second: "~idieras", "~idierais";
        imperfect_subjunctive_third: "~idiera", "~idieran";
        imperfect_subjunctive_se_first: "~idiese", "~idiésemos";
        imperfect_subjunctive_se_second: "~idieses", "~idieseis";
        imperfect_subjunctive_se_third: "~idiese", "~idiesen";
        future_subjunctive_first: "~idiere", "~idiéremos";
        future_subjunctive_second: "~idieres", "~idiereis";
        future_subjunctive_third: "~idiere", "~idieren";
        vos: "~edís", "~idás", "~edí";
        imperative: "~ide", "~ida", "~idamos", "~edid", "~idan";
        negative_imperative: "~idas", "~ida", "~idamos", "~idáis", "~idan";
        infinitive: "~edir";
        gerund: "~idiendo";
        participle_masculine: "~edido", "~edidos";
        participle_feminine: "~edida", "~edidas";
    ),
    // Moler: 4 words (doler, soler, moler, desmoler).
    conjugation!("moler", 4, "~oler";
        present_first: "~uelo", "~olemos";
        present_second: "~ueles", "~oléis";
        present_third: "~uele", "~uelen";
        imperfect_first: "~olía", "~olíamos";
        imperfect_second: "~olías", "~olíais";
        imperfect_third: "~olía", "~olían";
        preterite_first: "~olí", "~olimos";
        preterite_second: "~oliste", "~olisteis";
        preterite_third: "~olió", "~olieron";
        future_first: "~oleré", "~oleremos";
        future_second: "~olerás", "~oleréis";
        future_third: "~olerá", "~olerán";
        conditional_first: "~olería", "~oleríamos";
        conditional_second: "~olerías", "~oleríais";
        conditional_third: "~olería", "~olerían";
        present_subjunctive_first: "~uela", "~olamos";
        present_subjunctive_second: "~uelas", "~oláis";
        present_subjunctive_third: "~uela", "~uelan";
        imperfect_subjunctive_first: "~oliera", "~oliéramos";
        imperfect_subjunctive_second: "~olieras", "~olierais";
        imperfect_subjunctive_third: "~oliera", "~olieran";
        imperfect_subjunctive_se_first: "~oliese", "~oliésemos";
        imperfect_subjunctive_se_second: "~olieses", "~olieseis";
        imperfect_subjunctive_se_third: "~oliese", "~oliesen";
        future_subjunctive_first: "~oliere", "~oliéremos";
        future_subjunctive_second: "~olieres", "~oliereis";
        future_subjunctive_third: "~oliere", "~olieren";
        vos: "~olés", "~olás", "~olé";
        imperative: "~uele", "~uela", "~olamos", "~oled", "~uelan";
        negative_imperative: "~uelas", "~uela", "~olamos", "~oláis", "~uelan";
        infinitive: "~oler";
        gerund: "~oliendo";
        participle_masculine: "~olido", "~olidos";
        participle_feminine: "~olida", "~olidas";
    ),
    // Huir: 4 words (fuir, huir, luir, uir).
    conjugation!("huir", 4, "~ir";
        present_first: "~yo", "~imos";
        present_second: "~yes", "~is";
        present_third: "~ye", "~yen";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~i", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~yó", "~yeron";
        future_first: "~iré", "~iremos";
        future_second: "~irás", "~iréis";
        future_third: "~irá", "~irán";
        conditional_first: "~iría", "~iríamos";
        conditional_second: "~irías", "~iríais";
        conditional_third: "~iría", "~irían";
        present_subjunctive_first: "~ya", "~yamos";
        present_subjunctive_second: "~yas", "~yáis";
        present_subjunctive_third: "~ya", "~yan";
        imperfect_subjunctive_first: "~yera", "~yéramos";
        imperfect_subjunctive_second: "~yeras", "~yerais";
        imperfect_subjunctive_third: "~yera", "~yeran";
        imperfect_subjunctive_se_first: "~yese", "~yésemos";
        imperfect_subjunctive_se_second: "~yeses", "~yeseis";
        imperfect_subjunctive_se_third: "~yese", "~yesen";
        future_subjunctive_first: "~yere", "~yéremos";
        future_subjunctive_second: "~yeres", "~yereis";
        future_subjunctive_third: "~yere", "~yeren";
        vos: "~is", "~yás", "~i";
        imperative: "~ye", "~ya", "~yamos", "~id", "~yan";
        negative_imperative: "~yas", "~ya", "~yamos", "~yáis", "~yan";
        infinitive: "~ir";
        gerund: "~yendo";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Volver: 4 words (volver, solver, bolver, rrevolver).
    conjugation!("volver", 4, "~o~ver";
        present_first: "~ue~vo", "~o~vemos";
        present_second: "~ue~ves", "~o~véis";
        present_third: "~ue~ve", "~ue~ven";
        imperfect_first: "~o~vía", "~o~víamos";
        imperfect_second: "~o~vías", "~o~víais";
        imperfect_third: "~o~vía", "~o~vían";
        preterite_first: "~o~ví", "~o~vimos";
        preterite_second: "~o~viste", "~o~visteis";
        preterite_third: "~o~vió", "~o~vieron";
        future_first: "~o~veré", "~o~veremos";
        future_second: "~o~verás", "~o~veréis";
        future_third: "~o~verá", "~o~verán";
        conditional_first: "~o~vería", "~o~veríamos";
        conditional_second: "~o~verías", "~o~veríais";
        conditional_third: "~o~vería", "~o~verían";
        present_subjunctive_first: "~ue~va", "~o~vamos";
        present_subjunctive_second: "~ue~vas", "~o~váis";
        present_subjunctive_third: "~ue~va", "~ue~van";
        imperfect_subjunctive_first: "~o~viera", "~o~viéramos";
        imperfect_subjunctive_second: "~o~vieras", "~o~vierais";
        imperfect_subjunctive_third: "~o~viera", "~o~vieran";
        imperfect_subjunctive_se_first: "~o~viese", "~o~viésemos";
        imperfect_subjunctive_se_second: "~o~vieses", "~o~vieseis";
        imperfect_subjunctive_se_third: "~o~viese", "~o~viesen";
        future_subjunctive_first: "~o~viere", "~o~viéremos";
        future_subjunctive_second: "~o~vieres", "~o~viereis";
        future_subjunctive_third: "~o~viere", "~o~vieren";
        vos: "~o~vés", "~o~vás", "~o~vé";
        imperative: "~ue~ve", "~ue~va", "~o~vamos", "~o~ved", "~ue~van";
        negative_imperative: "~ue~vas", "~ue~va", "~o~vamos", "~o~váis", "~ue~van";
        infinitive: "~o~ver";
        gerund: "~o~viendo";
        participle_masculine: "~ue~to", "~ue~tos";
        participle_feminine: "~ue~ta", "~ue~tas";
    ),
    // Colarse: 4 words (colarse, volarse, escolarse, nsolarse).
    conjugation!("colarse", 4, "~olarse";
        present_first: "~uelo", "~olamos";
        present_second: "~uelas", "~oláis";
        present_third: "~uela", "~uelan";
        imperfect_first: "~olaba", "~olábamos";
        imperfect_second: "~olabas", "~olabais";
        imperfect_third: "~olaba", "~olaban";
        preterite_first: "~olé", "~olamos";
        preterite_second: "~olaste", "~olasteis";
        preterite_third: "~oló", "~olaron";
        future_first: "~olaré", "~olaremos";
        future_second: "~olarás", "~olaréis";
        future_third: "~olará", "~olarán";
        conditional_first: "~olaría", "~olaríamos";
        conditional_second: "~olarías", "~olaríais";
        conditional_third: "~olaría", "~olarían";
        present_subjunctive_first: "~uele", "~olemos";
        present_subjunctive_second: "~ueles", "~oléis";
        present_subjunctive_third: "~uele", "~uelen";
        imperfect_subjunctive_first: "~olara", "~oláramos";
        imperfect_subjunctive_second: "~olaras", "~olarais";
        imperfect_subjunctive_third: "~olara", "~olaran";
        imperfect_subjunctive_se_first: "~olase", "~olásemos";
        imperfect_subjunctive_se_second: "~olases", "~olaseis";
        imperfect_subjunctive_se_third: "~olase", "~olasen";
        future_subjunctive_first: "~olare", "~oláremos";
        future_subjunctive_second: "~olares", "~olareis";
        future_subjunctive_third: "~olare", "~olaren";
        vos: "~olás", "~olés", "~olate";
        imperative: "~uélate", "~uélese", "~olémonos", "~olaos", "~uélense";
        negative_imperative: "~ueles", "~uele", "~olemos", "~oléis", "~uelen";
        infinitive: "~olarse";
        gerund: "~olándose";
        participle_masculine: "~olado", "~olados";
        participle_feminine: "~olada", "~oladas";
    ),
    // Tullirse: 4 words (tullirse, bullirse, rruñirse, rpullirse).
    conjugation!("tullirse", 4, "~u~irse";
        present_first: "~u~o", "~u~imos";
        present_second: "~u~es", "~u~ís";
        present_third: "~u~e", "~u~en";
        imperfect_first: "~u~ía", "~u~íamos";
        imperfect_second: "~u~ías", "~u~íais";
        imperfect_third: "~u~ía", "~u~ían";
        preterite_first: "~u~í", "~u~imos";
        preterite_second: "~u~iste", "~u~isteis";
        preterite_third: "~u~ó", "~u~eron";
        future_first: "~u~iré", "~u~iremos";
        future_second: "~u~irás", "~u~iréis";
        future_third: "~u~irá", "~u~irán";
        conditional_first: "~u~iría", "~u~iríamos";
        conditional_second: "~u~irías", "~u~iríais";
        conditional_third: "~u~iría", "~u~irían";
        present_subjunctive_first: "~u~a", "~u~amos";
        present_subjunctive_second: "~u~as", "~u~áis";
        present_subjunctive_third: "~u~a", "~u~an";
        imperfect_subjunctive_first: "~u~era", "~u~éramos";
        imperfect_subjunctive_second: "~u~eras", "~u~erais";
        imperfect_subjunctive_third: "~u~era", "~u~eran";
        imperfect_subjunctive_se_first: "~u~ese", "~u~ésemos";
        imperfect_subjunctive_se_second: "~u~eses", "~u~eseis";
        imperfect_subjunctive_se_third: "~u~ese", "~u~esen";
        future_subjunctive_first: "~u~ere", "~u~éremos";
        future_subjunctive_second: "~u~eres", "~u~ereis";
        future_subjunctive_third: "~u~ere", "~u~eren";
        vos: "~u~ís", "~u~ás", "~u~ite";
        imperative: "~ú~ete", "~ú~ase", "~u~ámonos", "~u~íos", "~ú~anse";
        negative_imperative: "~u~as", "~u~a", "~u~amos", "~u~áis", "~u~an";
        infinitive: "~u~irse";
        gerund: "~u~éndose";
        participle_masculine: "~u~ido", "~u~idos";
        participle_feminine: "~u~ida", "~u~idas";
    ),
    // Contraerse: 4 words (contraerse, traerse, raerse, straerse).
    conjugation!("contraerse", 4, "~aerse";
        present_first: "~aigo", "~aemos";
        present_second: "~aes", "~aéis";
        present_third: "~ae", "~aen";
        imperfect_first: "~aía", "~aíamos";
        imperfect_second: "~aías", "~aíais";
        imperfect_third: "~aía", "~aían";
        preterite_first: "~aje", "~ajimos";
        preterite_second: "~ajiste", "~ajisteis";
        preterite_third: "~ajo", "~ajeron";
        future_first: "~aeré", "~aeremos";
        future_second: "~aerás", "~aeréis";
        future_third: "~aerá", "~aerán";
        conditional_first: "~aería", "~aeríamos";
        conditional_second: "~aerías", "~aeríais";
        conditional_third: "~aería", "~aerían";
        present_subjunctive_first: "~aiga", "~aigamos";
        present_subjunctive_second: "~aigas", "~aigáis";
        present_subjunctive_third: "~aiga", "~aigan";
        imperfect_subjunctive_first: "~ajera", "~ajéramos";
        imperfect_subjunctive_second: "~ajeras", "~ajerais";
        imperfect_subjunctive_third: "~ajera", "~ajeran";
        imperfect_subjunctive_se_first: "~ajese", "~ajésemos";
        imperfect_subjunctive_se_second: "~ajeses", "~ajeseis";
        imperfect_subjunctive_se_third: "~ajese", "~ajesen";
        future_subjunctive_first: "~ajere", "~ajéremos";
        future_subjunctive_second: "~ajeres", "~ajereis";
        future_subjunctive_third: "~ajere", "~ajeren";
        vos: "~aés", "~aigás", "~aete";
        imperative: "~áete", "~áigase", "~aigámonos", "~aeos", "~áiganse";
        negative_imperative: "~aigas", "~aiga", "~aigamos", "~aigáis", "~aigan";
        infinitive: "~aerse";
        gerund: "~ayéndose";
        participle_masculine: "~aído", "~aídos";
        participle_feminine: "~aída", "~aídas";
    ),
    // Criarse: 4 words (criarse, liarse, triarse, fiarse).
    conjugation!("criarse", 4, "~iarse";
        present_first: "~ío", "~iamos";
        present_second: "~ías", "~iais";
        present_third: "~ía", "~ían";
        imperfect_first: "~iaba", "~iábamos";
        imperfect_second: "~iabas", "~iabais";
        imperfect_third: "~iaba", "~iaban";
        preterite_first: "~ie", "~iamos";
        preterite_second: "~iaste", "~iasteis";
        preterite_third: "~io", "~iaron";
        future_first: "~iaré", "~iaremos";
        future_second: "~iarás", "~iaréis";
        future_third: "~iará", "~iarán";
        conditional_first: "~iaría", "~iaríamos";
        conditional_second: "~iarías", "~iaríais";
        conditional_third: "~iaría", "~iarían";
        present_subjunctive_first: "~íe", "~iemos";
        present_subjunctive_second: "~íes", "~ieis";
        present_subjunctive_third: "~íe", "~íen";
        imperfect_subjunctive_first: "~iara", "~iáramos";
        imperfect_subjunctive_second: "~iaras", "~iarais";
        imperfect_subjunctive_third: "~iara", "~iaran";
        imperfect_subjunctive_se_first: "~iase", "~iásemos";
        imperfect_subjunctive_se_second: "~iases", "~iaseis";
        imperfect_subjunctive_se_third: "~iase", "~iasen";
        future_subjunctive_first: "~iare", "~iáremos";
        future_subjunctive_second: "~iares", "~iareis";
        future_subjunctive_third: "~iare", "~iaren";
        vos: "~ias", "~ies", "~iate";
        imperative: "~íate", "~íese", "~iémonos", "~iaos", "~íense";
        negative_imperative: "~íes", "~íe", "~iemos", "~ieis", "~íen";
        infinitive: "~iarse";
        gerund: "~iándose";
        participle_masculine: "~iado", "~iados";
        participle_feminine: "~iada", "~iadas";
    ),
    // Brir: 4 words (brir, abrir, ubrir, descubrir).
    conjugation!("brir", 4, "~~ir";
        present_first: "~~o", "~~imos";
        present_second: "~~es", "~~ís";
        present_third: "~~e", "~~en";
        imperfect_first: "~~ía", "~~íamos";
        imperfect_second: "~~ías", "~~íais";
        imperfect_third: "~~ía", "~~ían";
        preterite_first: "~~í", "~~imos";
        preterite_second: "~~iste", "~~isteis";
        preterite_third: "~~ió", "~~ieron";
        future_first: "~~iré", "~~iremos";
        future_second: "~~irás", "~~iréis";
        future_third: "~~irá", "~~irán";
        conditional_first: "~~iría", "~~iríamos";
        conditional_second: "~~irías", "~~iríais";
        conditional_third: "~~iría", "~~irían";
        present_subjunctive_first: "~~a", "~~amos";
        present_subjunctive_second: "~~as", "~~áis";
        present_subjunctive_third: "~~a", "~~an";
        imperfect_subjunctive_first: "~~iera", "~~iéramos";
        imperfect_subjunctive_second: "~~ieras", "~~ierais";
        imperfect_subjunctive_third: "~~iera", "~~ieran";
        imperfect_subjunctive_se_first: "~~iese", "~~iésemos";
        imperfect_subjunctive_se_second: "~~ieses", "~~ieseis";
        imperfect_subjunctive_se_third: "~~iese", "~~iesen";
        future_subjunctive_first: "~~iere", "~~iéremos";
        future_subjunctive_second: "~~ieres", "~~iereis";
        future_subjunctive_third: "~~iere", "~~ieren";
        vos: "~~ís", "~~ás", "~~í";
        imperative: "~~e", "~~a", "~~amos", "~~id", "~~an";
        negative_imperative: "~~as", "~~a", "~~amos", "~~áis", "~~an";
        infinitive: "~~ir";
        gerund: "~~iendo";
        participle_masculine: "~ie~to", "~ie~tos";
        participle_feminine: "~ie~ta", "~ie~tas";
    ),
    // Onocerse: 4 words (econocerse, reconocerse, nocerse, onocerse).
    conjugation!("onocerse", 4, "~o~erse";
        present_first: "~oz~o", "~o~emos";
        present_second: "~o~es", "~o~éis";
        present_third: "~o~e", "~o~en";
        imperfect_first: "~o~ía", "~o~íamos";
        imperfect_second: "~o~ías", "~o~íais";
        imperfect_third: "~o~ía", "~o~ían";
        preterite_first: "~o~í", "~o~imos";
        preterite_second: "~o~iste", "~o~isteis";
        preterite_third: "~o~ió", "~o~ieron";
        future_first: "~o~eré", "~o~eremos";
        future_second: "~o~erás", "~o~eréis";
        future_third: "~o~erá", "~o~erán";
        conditional_first: "~o~ería", "~o~eríamos";
        conditional_second: "~o~erías", "~o~eríais";
        conditional_third: "~o~ería", "~o~erían";
        present_subjunctive_first: "~oz~a", "~oz~amos";
        present_subjunctive_second: "~oz~as", "~oz~áis";
        present_subjunctive_third: "~oz~a", "~oz~an";
        imperfect_subjunctive_first: "~o~iera", "~o~iéramos";
        imperfect_subjunctive_second: "~o~ieras", "~o~ierais";
        imperfect_subjunctive_third: "~o~iera", "~o~ieran";
        imperfect_subjunctive_se_first: "~o~iese", "~o~iésemos";
        imperfect_subjunctive_se_second: "~o~ieses", "~o~ieseis";
        imperfect_subjunctive_se_third: "~o~iese", "~o~iesen";
        future_subjunctive_first: "~o~iere", "~o~iéremos";
        future_subjunctive_second: "~o~ieres", "~o~iereis";
        future_subjunctive_third: "~o~iere", "~o~ieren";
        vos: "~o~és", "~oz~ás", "~o~ete";
        imperative: "~ó~ete", "~óz~ase", "~oz~ámonos", "~o~eos", "~óz~anse";
        negative_imperative: "~oz~as", "~oz~a", "~oz~amos", "~oz~áis", "~oz~an";
        infinitive: "~o~erse";
        gerund: "~o~iéndose";
        participle_masculine: "~o~ido", "~o~idos";
        participle_feminine: "~o~ida", "~o~idas";
    ),
    // Regir: 3 words (elegir, regir, olegir).
    conjugation!("regir", 3, "~egir";
        present_first: "~ijo", "~egimos";
        present_second: "~iges", "~egís";
        present_third: "~ige", "~igen";
        imperfect_first: "~egía", "~egíamos";
        imperfect_second: "~egías", "~egíais";
        imperfect_third: "~egía", "~egían";
        preterite_first: "~egí", "~egimos";
        preterite_second: "~egiste", "~egisteis";
        preterite_third: "~igió", "~igieron";
        future_first: "~egiré", "~egiremos";
        future_second: "~egirás", "~egiréis";
        future_third: "~egirá", "~egirán";
        conditional_first: "~egiría", "~egiríamos";
        conditional_second: "~egirías", "~egiríais";
        conditional_third: "~egiría", "~egirían";
        present_subjunctive_first: "~ija", "~ijamos";
        present_subjunctive_second: "~ijas", "~ijáis";
        present_subjunctive_third: "~ija", "~ijan";
        imperfect_subjunctive_first: "~igiera", "~igiéramos";
        imperfect_subjunctive_second: "~igieras", "~igierais";
        imperfect_subjunctive_third: "~igiera", "~igieran";
        imperfect_subjunctive_se_first: "~igiese", "~igiésemos";
        imperfect_subjunctive_se_second: "~igieses", "~igieseis";
        imperfect_subjunctive_se_third: "~igiese", "~igiesen";
        future_subjunctive_first: "~igiere", "~igiéremos";
        future_subjunctive_second: "~igieres", "~igiereis";
        future_subjunctive_third: "~igiere", "~igieren";
        vos: "~egís", "~ijás", "~egí";
        imperative: "~ige", "~ija", "~ijamos", "~egid", "~ijan";
        negative_imperative: "~ijas", "~ija", "~ijamos", "~ijáis", "~ijan";
        infinitive: "~egir";
        gerund: "~igiendo";
        participle_masculine: "~egido", "~egidos";
        participle_feminine: "~egida", "~egidas";
    ),
    // Lucir: 3 words (lucir, nocir, ucir).
    conjugation!("lucir", 3, "~~ir";
        present_first: "~z~o", "~~imos";
        present_second: "~~es", "~~ís";
        present_third: "~~e", "~~en";
        imperfect_first: "~~ía", "~~íamos";
        imperfect_second: "~~ías", "~~íais";
        imperfect_third: "~~ía", "~~ían";
        preterite_first: "~~í", "~~imos";
        preterite_second: "~~iste", "~~isteis";
        preterite_third: "~~ió", "~~ieron";
        future_first: "~~iré", "~~iremos";
        future_second: "~~irás", "~~iréis";
        future_third: "~~irá", "~~irán";
        conditional_first: "~~iría", "~~iríamos";
        conditional_second: "~~irías", "~~iríais";
        conditional_third: "~~iría", "~~irían";
        present_subjunctive_first: "~z~a", "~z~amos";
        present_subjunctive_second: "~z~as", "~z~áis";
        present_subjunctive_third: "~z~a", "~z~an";
        imperfect_subjunctive_first: "~~iera", "~~iéramos";
        imperfect_subjunctive_second: "~~ieras", "~~ierais";
        imperfect_subjunctive_third: "~~iera", "~~ieran";
        imperfect_subjunctive_se_first: "~~iese", "~~iésemos";
        imperfect_subjunctive_se_second: "~~ieses", "~~ieseis";
        imperfect_subjunctive_se_third: "~~iese", "~~iesen";
        future_subjunctive_first: "~~iere", "~~iéremos";
        future_subjunctive_second: "~~ieres", "~~iereis";
        future_subjunctive_third: "~~iere", "~~ieren";
        vos: "~~ís", "~z~ás", "~~í";
        imperative: "~~e", "~z~a", "~z~amos", "~~id", "~z~an";
        negative_imperative: "~z~as", "~z~a", "~z~amos", "~z~áis", "~z~an";
        infinitive: "~~ir";
        gerund: "~~iendo";
        participle_masculine: "~~ido", "~~idos";
        participle_feminine: "~~ida", "~~idas";
    ),
    // Cocer: 3 words (cocer, escocer, descocer).
    conjugation!("cocer", 3, "~ocer";
        present_first: "~uezo", "~ocemos";
        present_second: "~ueces", "~océis";
        present_third: "~uece", "~uecen";
        imperfect_first: "~ocía", "~ocíamos";
        imperfect_second: "~ocías", "~ocíais";
        imperfect_third: "~ocía", "~ocían";
        preterite_first: "~ocí", "~ocimos";
        preterite_second: "~ociste", "~ocisteis";
        preterite_third: "~oció", "~ocieron";
        future_first: "~oceré", "~oceremos";
        future_second: "~ocerás", "~oceréis";
        future_third: "~ocerá", "~ocerán";
        conditional_first: "~ocería", "~oceríamos";
        conditional_second: "~ocerías", "~oceríais";
        conditional_third: "~ocería", "~ocerían";
        present_subjunctive_first: "~ueza", "~ozamos";
        present_subjunctive_second: "~uezas", "~ozáis";
        present_subjunctive_third: "~ueza", "~uezan";
        imperfect_subjunctive_first: "~ociera", "~ociéramos";
        imperfect_subjunctive_second: "~ocieras", "~ocierais";
        imperfect_subjunctive_third: "~ociera", "~ocieran";
        imperfect_subjunctive_se_first: "~ociese", "~ociésemos";
        imperfect_subjunctive_se_second: "~ocieses", "~ocieseis";
        imperfect_subjunctive_se_third: "~ociese", "~ociesen";
        future_subjunctive_first: "~ociere", "~ociéremos";
        future_subjunctive_second: "~ocieres", "~ociereis";
        future_subjunctive_third: "~ociere", "~ocieren";
        vos: "~océs", "~ozás", "~océ";
        imperative: "~uece", "~ueza", "~ozamos", "~oced", "~uezan";
        negative_imperative: "~uezas", "~ueza", "~ozamos", "~ozáis", "~uezan";
        infinitive: "~ocer";
        gerund: "~ociendo";
        participle_masculine: "~ocido", "~ocidos";
        participle_feminine: "~ocida", "~ocidas";
    ),
    // Venirse: 3 words (venirse, avenirse, nvenirse).
    conjugation!("venirse", 3, "~e~irse";
        present_first: "~e~go", "~e~imos";
        present_second: "~ie~es", "~e~ís";
        present_third: "~ie~e", "~ie~en";
        imperfect_first: "~e~ía", "~e~íamos";
        imperfect_second: "~e~ías", "~e~íais";
        imperfect_third: "~e~ía", "~e~ían";
        preterite_first: "~i~e", "~i~imos";
        preterite_second: "~i~iste", "~i~isteis";
        preterite_third: "~i~o", "~i~ieron";
        future_first: "~e~dré", "~e~dremos";
        future_second: "~e~drás", "~e~dréis";
        future_third: "~e~drá", "~e~drán";
        conditional_first: "~e~dría", "~e~dríamos";
        conditional_second: "~e~drías", "~e~dríais";
        conditional_third: "~e~dría", "~e~drían";
        present_subjunctive_first: "~e~ga", "~e~gamos";
        present_subjunctive_second: "~e~gas", "~e~gáis";
        present_subjunctive_third: "~e~ga", "~e~gan";
        imperfect_subjunctive_first: "~i~iera", "~i~iéramos";
        imperfect_subjunctive_second: "~i~ieras", "~i~ierais";
        imperfect_subjunctive_third: "~i~iera", "~i~ieran";
        imperfect_subjunctive_se_first: "~i~iese", "~i~iésemos";
        imperfect_subjunctive_se_second: "~i~ieses", "~i~ieseis";
        imperfect_subjunctive_se_third: "~i~iese", "~i~iesen";
        future_subjunctive_first: "~i~iere", "~i~iéremos";
        future_subjunctive_second: "~i~ieres", "~i~iereis";
        future_subjunctive_third: "~i~iere", "~i~ieren";
        vos: "~e~ís", "~e~gás", "~e~ite";
        imperative: "~e~te", "~é~gase", "~e~gámonos", "~e~íos", "~é~ganse";
        negative_imperative: "~e~gas", "~e~ga", "~e~gamos", "~e~gáis", "~e~gan";
        infinitive: "~e~irse";
        gerund: "~i~iéndose";
        participle_masculine: "~e~ido", "~e~idos";
        participle_feminine: "~e~ida", "~e~idas";
    ),
    // Hibir: 3 words (rehinchir, hibir, rohibir).
    conjugation!("hibir", 3, "~i~ir";
        present_first: "~í~o", "~i~imos";
        present_second: "~í~es", "~i~ís";
        present_third: "~í~e", "~í~en";
        imperfect_first: "~i~ía", "~i~íamos";
        imperfect_second: "~i~ías", "~i~íais";
        imperfect_third: "~i~ía", "~i~ían";
        preterite_first: "~i~í", "~i~imos";
        preterite_second: "~i~iste", "~i~isteis";
        preterite_third: "~i~ió", "~i~ieron";
        future_first: "~i~iré", "~i~iremos";
        future_second: "~i~irás", "~i~iréis";
        future_third: "~i~irá", "~i~irán";
        conditional_first: "~i~iría", "~i~iríamos";
        conditional_second: "~i~irías", "~i~iríais";
        conditional_third: "~i~iría", "~i~irían";
        present_subjunctive_first: "~í~a", "~i~amos";
        present_subjunctive_second: "~í~as", "~i~áis";
        present_subjunctive_third: "~í~a", "~í~an";
        imperfect_subjunctive_first: "~i~iera", "~i~iéramos";
        imperfect_subjunctive_second: "~i~ieras", "~i~ierais";
        imperfect_subjunctive_third: "~i~iera", "~i~ieran";
        imperfect_subjunctive_se_first: "~i~iese", "~i~iésemos";
        imperfect_subjunctive_se_second: "~i~ieses", "~i~ieseis";
        imperfect_subjunctive_se_third: "~i~iese", "~i~iesen";
        future_subjunctive_first: "~i~iere", "~i~iéremos";
        future_subjunctive_second: "~i~ieres", "~i~iereis";
        future_subjunctive_third: "~i~iere", "~i~ieren";
        vos: "~i~ís", "~i~ás", "~i~í";
        imperative: "~í~e", "~í~a", "~i~amos", "~i~id", "~í~an";
        negative_imperative: "~í~as", "~í~a", "~i~amos", "~i~áis", "~í~an";
        infinitive: "~i~ir";
        gerund: "~i~iendo";
        participle_masculine: "~i~ido", "~i~idos";
        participle_feminine: "~i~ida", "~i~idas";
    ),
    // Colgarse: 3 words (colgarse, holgarse, descolgarse).
    conjugation!("colgarse", 3, "~o~arse";
        present_first: "~ue~o", "~o~amos";
        present_second: "~ue~as", "~o~áis";
        present_third: "~ue~a", "~ue~an";
        imperfect_first: "~o~aba", "~o~ábamos";
        imperfect_second: "~o~abas", "~o~abais";
        imperfect_third: "~o~aba", "~o~aban";
        preterite_first: "~o~ué", "~o~amos";
        preterite_second: "~o~aste", "~o~asteis";
        preterite_third: "~o~ó", "~o~aron";
        future_first: "~o~aré", "~o~aremos";
        future_second: "~o~arás", "~o~aréis";
        future_third: "~o~ará", "~o~arán";
        conditional_first: "~o~aría", "~o~aríamos";
        conditional_second: "~o~arías", "~o~aríais";
        conditional_third: "~o~aría", "~o~arían";
        present_subjunctive_first: "~ue~ue", "~o~uemos";
        present_subjunctive_second: "~ue~ues", "~o~uéis";
        present_subjunctive_third: "~ue~ue", "~ue~uen";
        imperfect_subjunctive_first: "~o~ara", "~o~áramos";
        imperfect_subjunctive_second: "~o~aras", "~o~arais";
        imperfect_subjunctive_third: "~o~ara", "~o~aran";
        imperfect_subjunctive_se_first: "~o~ase", "~o~ásemos";
        imperfect_subjunctive_se_second: "~o~ases", "~o~aseis";
        imperfect_subjunctive_se_third: "~o~ase", "~o~asen";
        future_subjunctive_first: "~o~are", "~o~áremos";
        future_subjunctive_second: "~o~ares", "~o~areis";
        future_subjunctive_third: "~o~are", "~o~aren";
        vos: "~o~ás", "~o~ués", "~o~ate";
        imperative: "~ué~ate", "~ué~uese", "~o~uémonos", "~o~aos", "~ué~uense";
        negative_imperative: "~ue~ues", "~ue~ue", "~o~uemos", "~o~uéis", "~ue~uen";
        infinitive: "~o~arse";
        gerund: "~o~ándose";
        participle_masculine: "~o~ado", "~o~ados";
        participle_feminine: "~o~ada", "~o~adas";
    ),
    // Teñirse: 3 words (teñirse, ceñirse, reñirse).
    conjugation!("teñirse", 3, "~eñirse";
        present_first: "~iño", "~eñimos";
        present_second: "~iñes", "~eñís";
        present_third: "~iñe", "~iñen";
        imperfect_first: "~eñía", "~eñíamos";
        imperfect_second: "~eñías", "~eñíais";
        imperfect_third: "~eñía", "~eñían";
        preterite_first: "~eñí", "~eñimos";
        preterite_second: "~eñiste", "~eñisteis";
        preterite_third: "~iñó", "~iñeron";
        future_first: "~eñiré", "~eñiremos";
        future_second: "~eñirás", "~eñiréis";
        future_third: "~eñirá", "~eñirán";
        conditional_first: "~eñiría", "~eñiríamos";
        conditional_second: "~eñirías", "~eñiríais";
        conditional_third: "~eñiría", "~eñirían";
        present_subjunctive_first: "~iña", "~iñamos";
        present_subjunctive_second: "~iñas", "~iñáis";
        present_subjunctive_third: "~iña", "~iñan";
        imperfect_subjunctive_first: "~iñera", "~iñéramos";
        imperfect_subjunctive_second: "~iñeras", "~iñerais";
        imperfect_subjunctive_third: "~iñera", "~iñeran";
        imperfect_subjunctive_se_first: "~iñese", "~iñésemos";
        imperfect_subjunctive_se_second: "~iñeses", "~iñeseis";
        imperfect_subjunctive_se_third: "~iñese", "~iñesen";
        future_subjunctive_first: "~iñere", "~iñéremos";
        future_subjunctive_second: "~iñeres", "~iñereis";
        future_subjunctive_third: "~iñere", "~iñeren";
        vos: "~eñís", "~iñás", "~eñite";
        imperative: "~íñete", "~íñase", "~iñámonos", "~eñíos", "~íñanse";
        negative_imperative: "~iñas", "~iña", "~iñamos", "~iñáis", "~iñan";
        infinitive: "~eñirse";
        gerund: "~iñéndose";
        participle_masculine: "~eñido", "~eñidos";
        participle_feminine: "~eñida", "~eñidas";
    ),
    // Despedirse: 3 words (despedirse, medirse, pedirse).
    conjugation!("despedirse", 3, "~edirse";
        present_first: "~ido", "~edimos";
        present_second: "~ides", "~edís";
        present_third: "~ide", "~iden";
        imperfect_first: "~edía", "~edíamos";
        imperfect_second: "~edías", "~edíais";
        imperfect_third: "~edía", "~edían";
        preterite_first: "~edí", "~edimos";
        preterite_second: "~ediste", "~edisteis";
        preterite_third: "~idió", "~idieron";
        future_first: "~ediré", "~ediremos";
        future_second: "~edirás", "~ediréis";
        future_third: "~edirá", "~edirán";
        conditional_first: "~ediría", "~ediríamos";
        conditional_second: "~edirías", "~ediríais";
        conditional_third: "~ediría", "~edirían";
        present_subjunctive_first: "~ida", "~idamos";
        present_subjunctive_second: "~idas", "~idáis";
        present_subjunctive_third: "~ida", "~idan";
        imperfect_subjunctive_first: "~idiera", "~idiéramos";
        imperfect_subjunctive_second: "~idieras", "~idierais";
        imperfect_subjunctive_third: "~idiera", "~idieran";
        imperfect_subjunctive_se_first: "~idiese", "~idiésemos";
        imperfect_subjunctive_se_second: "~idieses", "~idieseis";
        imperfect_subjunctive_se_third: "~idiese", "~idiesen";
        future_subjunctive_first: "~idiere", "~idiéremos";
        future_subjunctive_second: "~idieres", "~idiereis";
        future_subjunctive_third: "~idiere", "~idieren";
        vos: "~edís", "~idás", "~edite";
        imperative: "~ídete", "~ídase", "~idámonos", "~edíos", "~ídanse";
        negative_imperative: "~idas", "~ida", "~idamos", "~idáis", "~idan";
        infinitive: "~edirse";
        gerund: "~idiéndose";
        participle_masculine: "~edido", "~edidos";
        participle_feminine: "~edida", "~edidas";
    ),
    // Acogerse: 3 words (encogerse, cogerse, acogerse).
    conjugation!("acogerse", 3, "~ogerse";
        present_first: "~ojo", "~ogemos";
        present_second: "~oges", "~ogéis";
        present_third: "~oge", "~ogen";
        imperfect_first: "~ogía", "~ogíamos";
        imperfect_second: "~ogías", "~ogíais";
        imperfect_third: "~ogía", "~ogían";
        preterite_first: "~ogí", "~ogimos";
        preterite_second: "~ogiste", "~ogisteis";
        preterite_third: "~ogió", "~ogieron";
        future_first: "~ogeré", "~ogeremos";
        future_second: "~ogerás", "~ogeréis";
        future_third: "~ogerá", "~ogerán";
        conditional_first: "~ogería", "~ogeríamos";
        conditional_second: "~ogerías", "~ogeríais";
        conditional_third: "~ogería", "~ogerían";
        present_subjunctive_first: "~oja", "~ojamos";
        present_subjunctive_second: "~ojas", "~ojáis";
        present_subjunctive_third: "~oja", "~ojan";
        imperfect_subjunctive_first: "~ogiera", "~ogiéramos";
        imperfect_subjunctive_second: "~ogieras", "~ogierais";
        imperfect_subjunctive_third: "~ogiera", "~ogieran";
        imperfect_subjunctive_se_first: "~ogiese", "~ogiésemos";
        imperfect_subjunctive_se_second: "~ogieses", "~ogieseis";
        imperfect_subjunctive_se_third: "~ogiese", "~ogiesen";
        future_subjunctive_first: "~ogiere", "~ogiéremos";
        future_subjunctive_second: "~ogieres", "~ogiereis";
        future_subjunctive_third: "~ogiere", "~ogieren";
        vos: "~ogés", "~ojás", "~ogete";
        imperative: "~ógete", "~ójase", "~ojámonos", "~ogeos", "~ójanse";
        negative_imperative: "~ojas", "~oja", "~ojamos", "~ojáis", "~ojan";
        infinitive: "~ogerse";
        gerund: "~ogiéndose";
        participle_masculine: "~ogido", "~ogidos";
        participle_feminine: "~ogida", "~ogidas";
    ),
    // Regirse: 3 words (regirse, corregirse, elegirse).
    conjugation!("regirse", 3, "~egirse";
        present_first: "~ijo", "~egimos";
        present_second: "~iges", "~egís";
        present_third: "~ige", "~igen";
        imperfect_first: "~egía", "~egíamos";
        imperfect_second: "~egías", "~egíais";
        imperfect_third: "~egía", "~egían";
        preterite_first: "~egí", "~egimos";
        preterite_second: "~egiste", "~egisteis";
        preterite_third: "~igió", "~igieron";
        future_first: "~egiré", "~egiremos";
        future_second: "~egirás", "~egiréis";
        future_third: "~egirá", "~egirán";
        conditional_first: "~egiría", "~egiríamos";
        conditional_second: "~egirías", "~egiríais";
        conditional_third: "~egiría", "~egirían";
        present_subjunctive_first: "~ija", "~ijamos";
        present_subjunctive_second: "~ijas", "~ijáis";
        present_subjunctive_third: "~ija", "~ijan";
        imperfect_subjunctive_first: "~igiera", "~igiéramos";
        imperfect_subjunctive_second: "~igieras", "~igierais";
        imperfect_subjunctive_third: "~igiera", "~igieran";
        imperfect_subjunctive_se_first: "~igiese", "~igiésemos";
        imperfect_subjunctive_se_second: "~igieses", "~igieseis";
        imperfect_subjunctive_se_third: "~igiese", "~igiesen";
        future_subjunctive_first: "~igiere", "~igiéremos";
        future_subjunctive_second: "~igieres", "~igiereis";
        future_subjunctive_third: "~igiere", "~igieren";
        vos: "~egís", "~ijás", "~egite";
        imperative: "~ígete", "~íjase", "~ijámonos", "~egíos", "~íjanse";
        negative_imperative: "~ijas", "~ija", "~ijamos", "~ijáis", "~ijan";
        infinitive: "~egirse";
        gerund: "~igiéndose";
        participle_masculine: "~egido", "~egidos";
        participle_feminine: "~egida", "~egidas";
    ),
    // Fingirse: 3 words (fingirse, atingirse, fringirse).
    conjugation!("fingirse", 3, "~i~girse";
        present_first: "~i~jo", "~i~gimos";
        present_second: "~i~ges", "~i~gís";
        present_third: "~i~ge", "~i~gen";
        imperfect_first: "~i~gía", "~i~gíamos";
        imperfect_second: "~i~gías", "~i~gíais";
        imperfect_third: "~i~gía", "~i~gían";
        preterite_first: "~i~gí", "~i~gimos";
        preterite_second: "~i~giste", "~i~gisteis";
        preterite_third: "~i~gió", "~i~gieron";
        future_first: "~i~giré", "~i~giremos";
        future_second: "~i~girás", "~i~giréis";
        future_third: "~i~girá", "~i~girán";
        conditional_first: "~i~giría", "~i~giríamos";
        conditional_second: "~i~girías", "~i~giríais";
        conditional_third: "~i~giría", "~i~girían";
        present_subjunctive_first: "~i~ja", "~i~jamos";
        present_subjunctive_second: "~i~jas", "~i~jáis";
        present_subjunctive_third: "~i~ja", "~i~jan";
        imperfect_subjunctive_first: "~i~giera", "~i~giéramos";
        imperfect_subjunctive_second: "~i~gieras", "~i~gierais";
        imperfect_subjunctive_third: "~i~giera", "~i~gieran";
        imperfect_subjunctive_se_first: "~i~giese", "~i~giésemos";
        imperfect_subjunctive_se_second: "~i~gieses", "~i~gieseis";
        imperfect_subjunctive_se_third: "~i~giese", "~i~giesen";
        future_subjunctive_first: "~i~giere", "~i~giéremos";
        future_subjunctive_second: "~i~gieres", "~i~giereis";
        future_subjunctive_third: "~i~giere", "~i~gieren";
        vos: "~i~gís", "~i~jás", "~i~gite";
        imperative: "~í~gete", "~í~jase", "~i~jámonos", "~i~gíos", "~í~janse";
        negative_imperative: "~i~jas", "~i~ja", "~i~jamos", "~i~jáis", "~i~jan";
        infinitive: "~i~girse";
        gerund: "~i~giéndose";
        participle_masculine: "~i~gido", "~i~gidos";
        participle_feminine: "~i~gida", "~i~gidas";
    ),
    // Fraguarse: 3 words (desaguarse, fraguarse, niaguarse).
    conjugation!("fraguarse", 3, "~a~uarse";
        present_first: "~a~uo", "~a~uamos";
        present_second: "~a~uas", "~a~uáis";
        present_third: "~a~ua", "~a~uan";
        imperfect_first: "~a~uaba", "~a~uábamos";
        imperfect_second: "~a~uabas", "~a~uabais";
        imperfect_third: "~a~uaba", "~a~uaban";
        preterite_first: "~a~üé", "~a~uamos";
        preterite_second: "~a~uaste", "~a~uasteis";
        preterite_third: "~a~uó", "~a~uaron";
        future_first: "~a~uaré", "~a~uaremos";
        future_second: "~a~uarás", "~a~uaréis";
        future_third: "~a~uará", "~a~uarán";
        conditional_first: "~a~uaría", "~a~uaríamos";
        conditional_second: "~a~uarías", "~a~uaríais";
        conditional_third: "~a~uaría", "~a~uarían";
        present_subjunctive_first: "~a~üe", "~a~üemos";
        present_subjunctive_second: "~a~ües", "~a~üéis";
        present_subjunctive_third: "~a~üe", "~a~üen";
        imperfect_subjunctive_first: "~a~uara", "~a~uáramos";
        imperfect_subjunctive_second: "~a~uaras", "~a~uarais";
        imperfect_subjunctive_third: "~a~uara", "~a~uaran";
        imperfect_subjunctive_se_first: "~a~uase", "~a~uásemos";
        imperfect_subjunctive_se_second: "~a~uases", "~a~uaseis";
        imperfect_subjunctive_se_third: "~a~uase", "~a~uasen";
        future_subjunctive_first: "~a~uare", "~a~uáremos";
        future_subjunctive_second: "~a~uares", "~a~uareis";
        future_subjunctive_third: "~a~uare", "~a~uaren";
        vos: "~a~uás", "~a~üés", "~a~uate";
        imperative: "~á~uate", "~á~üese", "~a~üémonos", "~a~uaos", "~á~üense";
        negative_imperative: "~a~ües", "~a~üe", "~a~üemos", "~a~üéis", "~a~üen";
        infinitive: "~a~uarse";
        gerund: "~a~uándose";
        participle_masculine: "~a~uado", "~a~uados";
        participle_feminine: "~a~uada", "~a~uadas";
    ),
    // Rigirse: 3 words (erigirse, exigirse, rigirse).
    conjugation!("rigirse", 3, "~igirse";
        present_first: "~ijo", "~igimos";
        present_second: "~iges", "~igís";
        present_third: "~ige", "~igen";
        imperfect_first: "~igía", "~igíamos";
        imperfect_second: "~igías", "~igíais";
        imperfect_third: "~igía", "~igían";
        preterite_first: "~igí", "~igimos";
        preterite_second: "~igiste", "~igisteis";
        preterite_third: "~igió", "~igieron";
        future_first: "~igiré", "~igiremos";
        future_second: "~igirás", "~igiréis";
        future_third: "~igirá", "~igirán";
        conditional_first: "~igiría", "~igiríamos";
        conditional_second: "~igirías", "~igiríais";
        conditional_third: "~igiría", "~igirían";
        present_subjunctive_first: "~ija", "~ijamos";
        present_subjunctive_second: "~ijas", "~ijáis";
        present_subjunctive_third: "~ija", "~ijan";
        imperfect_subjunctive_first: "~igiera", "~igiéramos";
        imperfect_subjunctive_second: "~igieras", "~igierais";
        imperfect_subjunctive_third: "~igiera", "~igieran";
        imperfect_subjunctive_se_first: "~igiese", "~igiésemos";
        imperfect_subjunctive_se_second: "~igieses", "~igieseis";
        imperfect_subjunctive_se_third: "~igiese", "~igiesen";
        future_subjunctive_first: "~igiere", "~igiéremos";
        future_subjunctive_second: "~igieres", "~igiereis";
        future_subjunctive_third: "~igiere", "~igieren";
        vos: "~igís", "~ijás", "~igite";
        imperative: "~ígete", "~íjase", "~ijámonos", "~igíos", "~íjanse";
        negative_imperative: "~ijas", "~ija", "~ijamos", "~ijáis", "~ijan";
        infinitive: "~igirse";
        gerund: "~igiéndose";
        participle_masculine: "~igido", "~igidos";
        participle_feminine: "~igida", "~igidas";
    ),
    // Punzarse: 3 words (punzarse, arquizarse, roquizarse).
    conjugation!("punzarse", 3, "~u~zarse";
        present_first: "~u~zo", "~u~zamos";
        present_second: "~u~zas", "~u~záis";
        present_third: "~u~za", "~u~zan";
        imperfect_first: "~u~zaba", "~u~zábamos";
        imperfect_second: "~u~zabas", "~u~zabais";
        imperfect_third: "~u~zaba", "~u~zaban";
        preterite_first: "~u~cé", "~u~zamos";
        preterite_second: "~u~zaste", "~u~zasteis";
        preterite_third: "~u~zó", "~u~zaron";
        future_first: "~u~zaré", "~u~zaremos";
        future_second: "~u~zarás", "~u~zaréis";
        future_third: "~u~zará", "~u~zarán";
        conditional_first: "~u~zaría", "~u~zaríamos";
        conditional_second: "~u~zarías", "~u~zaríais";
        conditional_third: "~u~zaría", "~u~zarían";
        present_subjunctive_first: "~u~ce", "~u~cemos";
        present_subjunctive_second: "~u~ces", "~u~céis";
        present_subjunctive_third: "~u~ce", "~u~cen";
        imperfect_subjunctive_first: "~u~zara", "~u~záramos";
        imperfect_subjunctive_second: "~u~zaras", "~u~zarais";
        imperfect_subjunctive_third: "~u~zara", "~u~zaran";
        imperfect_subjunctive_se_first: "~u~zase", "~u~zásemos";
        imperfect_subjunctive_se_second: "~u~zases", "~u~zaseis";
        imperfect_subjunctive_se_third: "~u~zase", "~u~zasen";
        future_subjunctive_first: "~u~zare", "~u~záremos";
        future_subjunctive_second: "~u~zares", "~u~zareis";
        future_subjunctive_third: "~u~zare", "~u~zaren";
        vos: "~u~zás", "~u~cés", "~u~zate";
        imperative: "~ú~zate", "~ú~cese", "~u~cémonos", "~u~zaos", "~ú~cense";
        negative_imperative: "~u~ces", "~u~ce", "~u~cemos", "~u~céis", "~u~cen";
        infinitive: "~u~zarse";
        gerund: "~u~zándose";
        participle_masculine: "~u~zado", "~u~zados";
        participle_feminine: "~u~zada", "~u~zadas";
    ),
    // Güir: 3 words (güir, argüir, dargüir).
    conjugation!("güir", 3, "~üir";
        present_first: "~uyo", "~üimos";
        present_second: "~uyes", "~üís";
        present_third: "~uye", "~uyen";
        imperfect_first: "~üía", "~üíamos";
        imperfect_second: "~üías", "~üíais";
        imperfect_third: "~üía", "~üían";
        preterite_first: "~üí", "~üimos";
        preterite_second: "~üiste", "~üisteis";
        preterite_third: "~uyó", "~uyeron";
        future_first: "~üiré", "~üiremos";
        future_second: "~üirás", "~üiréis";
        future_third: "~üirá", "~üirán";
        conditional_first: "~üiría", "~üiríamos";
        conditional_second: "~üirías", "~üiríais";
        conditional_third: "~üiría", "~üirían";
        present_subjunctive_first: "~uya", "~uyamos";
        present_subjunctive_second: "~uyas", "~uyáis";
        present_subjunctive_third: "~uya", "~uyan";
        imperfect_subjunctive_first: "~uyera", "~uyéramos";
        imperfect_subjunctive_second: "~uyeras", "~uyerais";
        imperfect_subjunctive_third: "~uyera", "~uyeran";
        imperfect_subjunctive_se_first: "~uyese", "~uyésemos";
        imperfect_subjunctive_se_second: "~uyeses", "~uyeseis";
        imperfect_subjunctive_se_third: "~uyese", "~uyesen";
        future_subjunctive_first: "~uyere", "~uyéremos";
        future_subjunctive_second: "~uyeres", "~uyereis";
        future_subjunctive_third: "~uyere", "~uyeren";
        vos: "~üís", "~uyás", "~üí";
        imperative: "~uye", "~uya", "~uyamos", "~üid", "~uyan";
        negative_imperative: "~uyas", "~uya", "~uyamos", "~uyáis", "~uyan";
        infinitive: "~üir";
        gerund: "~uyendo";
        participle_masculine: "~üido", "~üidos";
        participle_feminine: "~üida", "~üidas";
    ),
    // Tinguir: 3 words (tinguir, distinguir, extinguir).
    conjugation!("tinguir", 3, "~uir";
        present_first: "~o", "~uimos";
        present_second: "~ues", "~uís";
        present_third: "~ue", "~uen";
        imperfect_first: "~uía", "~uíamos";
        imperfect_second: "~uías", "~uíais";
        imperfect_third: "~uía", "~uían";
        preterite_first: "~uí", "~uimos";
        preterite_second: "~uiste", "~uisteis";
        preterite_third: "~uió", "~uieron";
        future_first: "~uiré", "~uiremos";
        future_second: "~uirás", "~uiréis";
        future_third: "~uirá", "~uirán";
        conditional_first: "~uiría", "~uiríamos";
        conditional_second: "~uirías", "~uiríais";
        conditional_third: "~uiría", "~uirían";
        present_subjunctive_first: "~a", "~amos";
        present_subjunctive_second: "~as", "~áis";
        present_subjunctive_third: "~a", "~an";
        imperfect_subjunctive_first: "~uiera", "~uiéramos";
        imperfect_subjunctive_second: "~uieras", "~uierais";
        imperfect_subjunctive_third: "~uiera", "~uieran";
        imperfect_subjunctive_se_first: "~uiese", "~uiésemos";
        imperfect_subjunctive_se_second: "~uieses", "~uieseis";
        imperfect_subjunctive_se_third: "~uiese", "~uiesen";
        future_subjunctive_first: "~uiere", "~uiéremos";
        future_subjunctive_second: "~uieres", "~uiereis";
        future_subjunctive_third: "~uiere", "~uieren";
        vos: "~uís", "~ás", "~uí";
        imperative: "~ue", "~a", "~amos", "~uid", "~an";
        negative_imperative: "~as", "~a", "~amos", "~áis", "~an";
        infinitive: "~uir";
        gerund: "~uiendo";
        participle_masculine: "~uido", "~uidos";
        participle_feminine: "~uida", "~uidas";
    ),
    // Linquir: 3 words (linquir, elinquir, inquir).
    conjugation!("linquir", 3, "~quir";
        present_first: "~co", "~quimos";
        present_second: "~ques", "~quís";
        present_third: "~que", "~quen";
        imperfect_first: "~quía", "~quíamos";
        imperfect_second: "~quías", "~quíais";
        imperfect_third: "~quía", "~quían";
        preterite_first: "~quí", "~quimos";
        preterite_second: "~quiste", "~quisteis";
        preterite_third: "~quió", "~quieron";
        future_first: "~quiré", "~quiremos";
        future_second: "~quirás", "~quiréis";
        future_third: "~quirá", "~quirán";
        conditional_first: "~quiría", "~quiríamos";
        conditional_second: "~quirías", "~quiríais";
        conditional_third: "~quiría", "~quirían";
        present_subjunctive_first: "~ca", "~camos";
        present_subjunctive_second: "~cas", "~cáis";
        present_subjunctive_third: "~ca", "~can";
        imperfect_subjunctive_first: "~quiera", "~quiéramos";
        imperfect_subjunctive_second: "~quieras", "~quierais";
        imperfect_subjunctive_third: "~quiera", "~quieran";
        imperfect_subjunctive_se_first: "~quiese", "~quiésemos";
        imperfect_subjunctive_se_second: "~quieses", "~quieseis";
        imperfect_subjunctive_se_third: "~quiese", "~quiesen";
        future_subjunctive_first: "~quiere", "~quiéremos";
        future_subjunctive_second: "~quieres", "~quiereis";
        future_subjunctive_third: "~quiere", "~quieren";
        vos: "~quís", "~cás", "~quí";
        imperative: "~que", "~ca", "~camos", "~quid", "~can";
        negative_imperative: "~cas", "~ca", "~camos", "~cáis", "~can";
        infinitive: "~quir";
        gerund: "~quiendo";
        participle_masculine: "~quido", "~quidos";
        participle_feminine: "~quida", "~quidas";
    ),
    // Ancarse: 3 words (ancarse, arcarse, alcarse).
    conjugation!("ancarse", 3, "a~carse";
        present_first: "a~co", "a~camos";
        present_second: "a~cas", "a~cáis";
        present_third: "a~ca", "a~can";
        imperfect_first: "a~caba", "a~cábamos";
        imperfect_second: "a~cabas", "a~cabais";
        imperfect_third: "a~caba", "a~caban";
        preterite_first: "a~qué", "a~camos";
        preterite_second: "a~caste", "a~casteis";
        preterite_third: "a~có", "a~caron";
        future_first: "a~caré", "a~caremos";
        future_second: "a~carás", "a~caréis";
        future_third: "a~cará", "a~carán";
        conditional_first: "a~caría", "a~caríamos";
        conditional_second: "a~carías", "a~caríais";
        conditional_third: "a~caría", "a~carían";
        present_subjunctive_first: "a~que", "a~quemos";
        present_subjunctive_second: "a~ques", "a~quéis";
        present_subjunctive_third: "a~que", "a~quen";
        imperfect_subjunctive_first: "a~cara", "a~cáramos";
        imperfect_subjunctive_second: "a~caras", "a~carais";
        imperfect_subjunctive_third: "a~cara", "a~caran";
        imperfect_subjunctive_se_first: "a~case", "a~cásemos";
        imperfect_subjunctive_se_second: "a~cases", "a~caseis";
        imperfect_subjunctive_se_third: "a~case", "a~casen";
        future_subjunctive_first: "a~care", "a~cáremos";
        future_subjunctive_second: "a~cares", "a~careis";
        future_subjunctive_third: "a~care", "a~caren";
        vos: "a~cás", "a~qués", "a~cate";
        imperative: "á~cate", "á~quese", "a~quémonos", "a~caos", "á~quense";
        negative_imperative: "a~ques", "a~que", "a~quemos", "a~quéis", "a~quen";
        infinitive: "a~carse";
        gerund: "a~cándose";
        participle_masculine: "a~cado", "a~cados";
        participle_feminine: "a~cada", "a~cadas";
    ),
    // Ferirse: 3 words (ferirse, jerirse, herirse).
    conjugation!("ferirse", 3, "~erirse";
        present_first: "~iero", "~erimos";
        present_second: "~ieres", "~erís";
        present_third: "~iere", "~ieren";
        imperfect_first: "~ería", "~eríamos";
        imperfect_second: "~erías", "~eríais";
        imperfect_third: "~ería", "~erían";
        preterite_first: "~erí", "~erimos";
        preterite_second: "~eriste", "~eristeis";
        preterite_third: "~irió", "~irieron";
        future_first: "~eriré", "~eriremos";
        future_second: "~erirás", "~eriréis";
        future_third: "~erirá", "~erirán";
        conditional_first: "~eriría", "~eriríamos";
        conditional_second: "~erirías", "~eriríais";
        conditional_third: "~eriría", "~erirían";
        present_subjunctive_first: "~iera", "~iramos";
        present_subjunctive_second: "~ieras", "~iráis";
        present_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_first: "~iriera", "~iriéramos";
        imperfect_subjunctive_second: "~irieras", "~irierais";
        imperfect_subjunctive_third: "~iriera", "~irieran";
        imperfect_subjunctive_se_first: "~iriese", "~iriésemos";
        imperfect_subjunctive_se_second: "~irieses", "~irieseis";
        imperfect_subjunctive_se_third: "~iriese", "~iriesen";
        future_subjunctive_first: "~iriere", "~iriéremos";
        future_subjunctive_second: "~irieres", "~iriereis";
        future_subjunctive_third: "~iriere", "~irieren";
        vos: "~erís", "~irás", "~erite";
        imperative: "~iérete", "~iérase", "~irámonos", "~eríos", "~iéranse";
        negative_imperative: "~ieras", "~iera", "~iramos", "~iráis", "~ieran";
        infinitive: "~erirse";
        gerund: "~iriéndose";
        participle_masculine: "~erido", "~eridos";
        participle_feminine: "~erida", "~eridas";
    ),
    // Ugarse: 3 words (ugarse, usgarse, ulgarse).
    conjugation!("ugarse", 3, "u~arse";
        present_first: "u~o", "u~amos";
        present_second: "u~as", "u~áis";
        present_third: "u~a", "u~an";
        imperfect_first: "u~aba", "u~ábamos";
        imperfect_second: "u~abas", "u~abais";
        imperfect_third: "u~aba", "u~aban";
        preterite_first: "u~ué", "u~amos";
        preterite_second: "u~aste", "u~asteis";
        preterite_third: "u~ó", "u~aron";
        future_first: "u~aré", "u~aremos";
        future_second: "u~arás", "u~aréis";
        future_third: "u~ará", "u~arán";
        conditional_first: "u~aría", "u~aríamos";
        conditional_second: "u~arías", "u~aríais";
        conditional_third: "u~aría", "u~arían";
        present_subjunctive_first: "u~ue", "u~uemos";
        present_subjunctive_second: "u~ues", "u~uéis";
        present_subjunctive_third: "u~ue", "u~uen";
        imperfect_subjunctive_first: "u~ara", "u~áramos";
        imperfect_subjunctive_second: "u~aras", "u~arais";
        imperfect_subjunctive_third: "u~ara", "u~aran";
        imperfect_subjunctive_se_first: "u~ase", "u~ásemos";
        imperfect_subjunctive_se_second: "u~ases", "u~aseis";
        imperfect_subjunctive_se_third: "u~ase", "u~asen";
        future_subjunctive_first: "u~are", "u~áremos";
        future_subjunctive_second: "u~ares", "u~areis";
        future_subjunctive_third: "u~are", "u~aren";
        vos: "u~ás", "u~ués", "u~ate";
        imperative: "ú~ate", "ú~uese", "u~uémonos", "u~aos", "ú~uense";
        negative_imperative: "u~ues", "u~ue", "u~uemos", "u~uéis", "u~uen";
        infinitive: "u~arse";
        gerund: "u~ándose";
        participle_masculine: "u~ado", "u~ados";
        participle_feminine: "u~ada", "u~adas";
    ),
    // Engreírse: 2 words (engreírse, sonreírse).
    conjugation!("engreírse", 2, "~eírse";
        present_first: "~ío", "~eímos";
        present_second: "~íes", "~eís";
        present_third: "~íe", "~íen";
        imperfect_first: "~eía", "~eíamos";
        imperfect_second: "~eías", "~eíais";
        imperfect_third: "~eía", "~eían";
        preterite_first: "~eí", "~eímos";
        preterite_second: "~eíste", "~eísteis";
        preterite_third: "~ió", "~ieron";
        future_first: "~eiré", "~eiremos";
        future_second: "~eirás", "~eiréis";
        future_third: "~eirá", "~eirán";
        conditional_first: "~eiría", "~eiríamos";
        conditional_second: "~eirías", "~eiríais";
        conditional_third: "~eiría", "~eirían";
        present_subjunctive_first: "~ía", "~iamos";
        present_subjunctive_second: "~ías", "~iáis";
        present_subjunctive_third: "~ía", "~ían";
        imperfect_subjunctive_first: "~iera", "~iéramos";
        imperfect_subjunctive_second: "~ieras", "~ierais";
        imperfect_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_se_first: "~iese", "~iésemos";
        imperfect_subjunctive_se_second: "~ieses", "~ieseis";
        imperfect_subjunctive_se_third: "~iese", "~iesen";
        future_subjunctive_first: "~iere", "~iéremos";
        future_subjunctive_second: "~ieres", "~iereis";
        future_subjunctive_third: "~iere", "~ieren";
        vos: "~eís", "~iás", "~eíte";
        imperative: "~íete", "~íase", "~iámonos", "~eíos", "~íanse";
        negative_imperative: "~ías", "~ía", "~iamos", "~iáis", "~ían";
        infinitive: "~eírse";
        gerund: "~iéndose";
        participle_masculine: "~eído", "~eídos";
        participle_feminine: "~eída", "~eídas";
    ),
    // Ir: 1 words (ir).
    conjugation!("ir", 1, "ir";
        present_first: "voy", "vamos";
        present_second: "vas", "vais";
        present_third: "va", "van";
        imperfect_first: "iba", "íbamos";
        imperfect_second: "ibas", "ibais";
        imperfect_third: "iba", "iban";
        preterite_first: "fui", "fuimos";
        preterite_second: "fuiste", "fuisteis";
        preterite_third: "fue", "fueron";
        future_first: "iré", "iremos";
        future_second: "irás", "iréis";
        future_third: "irá", "irán";
        conditional_first: "iría", "iríamos";
        conditional_second: "irías", "iríais";
        conditional_third: "iría", "irían";
        present_subjunctive_first: "vaya", "vayamos";
        present_subjunctive_second: "vayas", "vayáis";
        present_subjunctive_third: "vaya", "vayan";
        imperfect_subjunctive_first: "fuera", "fuéramos";
        imperfect_subjunctive_second: "fueras", "fuerais";
        imperfect_subjunctive_third: "fuera", "fueran";
        imperfect_subjunctive_se_first: "fuese", "fuésemos";
        imperfect_subjunctive_se_second: "fueses", "fueseis";
        imperfect_subjunctive_se_third: "fuese", "fuesen";
        future_subjunctive_first: "fuere", "fuéremos";
        future_subjunctive_second: "fueres", "fuereis";
        future_subjunctive_third: "fuere", "fueren";
        vos: "vas", "vayás", "andá";
        imperative: "ve", "vaya", "vamos|vayamos", "id", "vayan";
        negative_imperative: "vayas", "vaya", "vayamos", "vayáis", "vayan";
        infinitive: "ir";
        gerund: "yendo";
        participle_masculine: "ido", "idos";
        participle_feminine: "ida", "idas";
    ),
    // Oír: 1 words (oír).
    conjugation!("oír", 1, "~ír";
        present_first: "~igo", "~ímos";
        present_second: "~yes", "~ís";
        present_third: "~ye", "~yen";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~ímos";
        preterite_second: "~íste", "~ísteis";
        preterite_third: "~yó", "~yeron";
        future_first: "~iré", "~iremos";
        future_second: "~irás", "~iréis";
        future_third: "~irá", "~irán";
        conditional_first: "~iría", "~iríamos";
        conditional_second: "~irías", "~iríais";
        conditional_third: "~iría", "~irían";
        present_subjunctive_first: "~iga", "~igamos";
        present_subjunctive_second: "~igas", "~igáis";
        present_subjunctive_third: "~iga", "~igan";
        imperfect_subjunctive_first: "~yera", "~yéramos";
        imperfect_subjunctive_second: "~yeras", "~yerais";
        imperfect_subjunctive_third: "~yera", "~yeran";
        imperfect_subjunctive_se_first: "~yese", "~yésemos";
        imperfect_subjunctive_se_second: "~yeses", "~yeseis";
        imperfect_subjunctive_se_third: "~yese", "~yesen";
        future_subjunctive_first: "~yere", "~yéremos";
        future_subjunctive_second: "~yeres", "~yereis";
        future_subjunctive_third: "~yere", "~yeren";
        vos: "~ís", "~igás", "~í";
        imperative: "~ye", "~iga", "~igamos", "~íd", "~igan";
        negative_imperative: "~igas", "~iga", "~igamos", "~igáis", "~igan";
        infinitive: "~ír";
        gerund: "~yendo";
        participle_masculine: "~ído", "~ídos";
        participle_feminine: "~ída", "~ídas";
    ),
    // Reírse: 1 words (reírse).
    conjugation!("reírse", 1, "~~írse";
        present_first: "~~o", "~~ímos";
        present_second: "~~es", "~~ís";
        present_third: "~~e", "~~en";
        imperfect_first: "~~ía", "~~íamos";
        imperfect_second: "~~ías", "~~íais";
        imperfect_third: "~~ía", "~~ían";
        preterite_first: "~~í", "~~ímos";
        preterite_second: "~~íste", "~~ísteis";
        preterite_third: "~~yó", "~~yeron";
        future_first: "~~iré", "~~iremos";
        future_second: "~~irás", "~~iréis";
        future_third: "~~irá", "~~irán";
        conditional_first: "~~iría", "~~iríamos";
        conditional_second: "~~irías", "~~iríais";
        conditional_third: "~~iría", "~~irían";
        present_subjunctive_first: "~~a", "~~amos";
        present_subjunctive_second: "~~as", "~~áis";
        present_subjunctive_third: "~~a", "~~an";
        imperfect_subjunctive_first: "~~yera", "~~yéramos";
        imperfect_subjunctive_second: "~~yeras", "~~yerais";
        imperfect_subjunctive_third: "~~yera", "~~yeran";
        imperfect_subjunctive_se_first: "~~yese", "~~yésemos";
        imperfect_subjunctive_se_second: "~~yeses", "~~yeseis";
        imperfect_subjunctive_se_third: "~~yese", "~~yesen";
        future_subjunctive_first: "~~yere", "~~yéremos";
        future_subjunctive_second: "~~yeres", "~~yereis";
        future_subjunctive_third: "~~yere", "~~yeren";
        vos: "~~ís", "~~ás", "~~íte";
        imperative: "~é~te", "~éas~", "~~ámonos", "~~íos", "~éans~";
        negative_imperative: "~~as", "~~a", "~~amos", "~~áis", "~~an";
        infinitive: "~~írse";
        gerund: "~~yéndose";
        participle_masculine: "~~ído", "~~ídos";
        participle_feminine: "~~ída", "~~ídas";
    ),
    // Irse: 1 words (irse).
    conjugation!("irse", 1, "irse";
        present_first: "voy", "vamos";
        present_second: "vas", "vais";
        present_third: "va", "van";
        imperfect_first: "iba", "íbamos";
        imperfect_second: "ibas", "ibais";
        imperfect_third: "iba", "iban";
        preterite_first: "fui", "fuimos";
        preterite_second: "fuiste", "fuisteis";
        preterite_third: "fue", "fueron";
        future_first: "iré", "iremos";
        future_second: "irás", "iréis";
        future_third: "irá", "irán";
        conditional_first: "iría", "iríamos";
        conditional_second: "irías", "iríais";
        conditional_third: "iría", "irían";
        present_subjunctive_first: "vaya", "vayamos";
        present_subjunctive_second: "vayas", "vayáis";
        present_subjunctive_third: "vaya", "vayan";
        imperfect_subjunctive_first: "fuera", "fuéramos";
        imperfect_subjunctive_second: "fueras", "fuerais";
        imperfect_subjunctive_third: "fuera", "fueran";
        imperfect_subjunctive_se_first: "fuese", "fuésemos";
        imperfect_subjunctive_se_second: "fueses", "fueseis";
        imperfect_subjunctive_se_third: "fuese", "fuesen";
        future_subjunctive_first: "fuere", "fuéremos";
        future_subjunctive_second: "fueres", "fuereis";
        future_subjunctive_third: "fuere", "fueren";
        vos: "vas", "vayás", "andate";
        imperative: "vete|ándate", "váyase", "vayámonos|vámonos", "idos", "váyanse";
        negative_imperative: "vayas", "vaya", "vayamos", "vayáis", "vayan";
        infinitive: "irse";
        gerund: "yéndose";
        participle_masculine: "ido", "idos";
        participle_feminine: "ida", "idas";
    ),
    // Cirse: 1 words (cirse).
    conjugation!("cirse", 1, "~irse";
        present_first: "~o", "~imos";
        present_second: "~es", "~ís";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~ió", "~ieron";
        future_first: "~iré", "~iremos";
        future_second: "~irás", "~iréis";
        future_third: "~irá", "~irán";
        conditional_first: "~iría", "~iríamos";
        conditional_second: "~irías", "~iríais";
        conditional_third: "~iría", "~irían";
        present_subjunctive_first: "~a", "~amos";
        present_subjunctive_second: "~as", "~áis";
        present_subjunctive_third: "~a", "~an";
        imperfect_subjunctive_first: "~iera", "~iéramos";
        imperfect_subjunctive_second: "~ieras", "~ierais";
        imperfect_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_se_first: "~iese", "~iésemos";
        imperfect_subjunctive_se_second: "~ieses", "~ieseis";
        imperfect_subjunctive_se_third: "~iese", "~iesen";
        future_subjunctive_first: "~iere", "~iéremos";
        future_subjunctive_second: "~ieres", "~iereis";
        future_subjunctive_third: "~iere", "~ieren";
        vos: "~ís", "~ás", "~ite";
        imperative: "~e", "~a", "~amos", "~íos", "~an";
        negative_imperative: "~as", "~a", "~amos", "~áis", "~an";
        infinitive: "~irse";
        gerund: "~iéndose";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Verse: 1 words (verse).
    conjugation!("verse", 1, "verse";
        present_first: "veo", "vemos";
        present_second: "ves", "veis";
        present_third: "ve", "ven";
        imperfect_first: "veía", "veíamos";
        imperfect_second: "veías", "veíais";
        imperfect_third: "veía", "veían";
        preterite_first: "vi", "vimos";
        preterite_second: "viste", "visteis";
        preterite_third: "vio", "vieron";
        future_first: "veré", "veremos";
        future_second: "verás", "veréis";
        future_third: "verá", "verán";
        conditional_first: "vería", "veríamos";
        conditional_second: "verías", "veríais";
        conditional_third: "vería", "verían";
        present_subjunctive_first: "vea", "veamos";
        present_subjunctive_second: "veas", "veáis";
        present_subjunctive_third: "vea", "vean";
        imperfect_subjunctive_first: "viera", "viéramos";
        imperfect_subjunctive_second: "vieras", "vierais";
        imperfect_subjunctive_third: "viera", "vieran";
        imperfect_subjunctive_se_first: "viese", "viésemos";
        imperfect_subjunctive_se_second: "vieses", "vieseis";
        imperfect_subjunctive_se_third: "viese", "viesen";
        future_subjunctive_first: "viere", "viéremos";
        future_subjunctive_second: "vieres", "viereis";
        future_subjunctive_third: "viere", "vieren";
        vos: "ves", "veás", "mirate";
        imperative: "vete", "véase", "veámonos", "veos", "véanse";
        negative_imperative: "veas", "vea", "veamos", "veáis", "vean";
        infinitive: "verse";
        gerund: "viéndose";
        participle_masculine: "visto", "vistos";
        participle_feminine: "vista", "vistas";
    ),
    // Podrirse: 1 words (podrirse).
    conjugation!("podrirse", 1, "~o~irse";
        present_first: "~u~o", "~o~imos";
        present_second: "~u~es", "~o~ís";
        present_third: "~u~e", "~u~en";
        imperfect_first: "~o~ía", "~o~íamos";
        imperfect_second: "~o~ías", "~o~íais";
        imperfect_third: "~o~ía", "~o~ían";
        preterite_first: "~o~í", "~o~imos";
        preterite_second: "~o~iste", "~o~isteis";
        preterite_third: "~u~ió", "~u~ieron";
        future_first: "~o~iré", "~o~iremos";
        future_second: "~o~irás", "~o~iréis";
        future_third: "~o~irá", "~o~irán";
        conditional_first: "~o~iría", "~o~iríamos";
        conditional_second: "~o~irías", "~o~iríais";
        conditional_third: "~o~iría", "~o~irían";
        present_subjunctive_first: "~u~a", "~u~amos";
        present_subjunctive_second: "~u~as", "~u~áis";
        present_subjunctive_third: "~u~a", "~u~an";
        imperfect_subjunctive_first: "~u~iera", "~u~iéramos";
        imperfect_subjunctive_second: "~u~ieras", "~u~ierais";
        imperfect_subjunctive_third: "~u~iera", "~u~ieran";
        imperfect_subjunctive_se_first: "~u~iese", "~u~iésemos";
        imperfect_subjunctive_se_second: "~u~ieses", "~u~ieseis";
        imperfect_subjunctive_se_third: "~u~iese", "~u~iesen";
        future_subjunctive_first: "~u~iere", "~u~iéremos";
        future_subjunctive_second: "~u~ieres", "~u~iereis";
        future_subjunctive_third: "~u~iere", "~u~ieren";
        vos: "~o~ís", "~u~ás", "~o~ite";
        imperative: "~ú~ete", "~ú~ase", "~u~ámonos", "~o~íos", "~ú~anse";
        negative_imperative: "~u~as", "~u~a", "~u~amos", "~u~áis", "~u~an";
        infinitive: "~o~irse";
        gerund: "~u~iéndose";
        participle_masculine: "~o~ido", "~o~idos";
        participle_feminine: "~o~ida", "~o~idas";
    ),
    // Dormirse: 1 words (dormirse).
    conjugation!("dormirse", 1, "~o~irse";
        present_first: "~ue~o", "~o~imos";
        present_second: "~ue~es", "~o~ís";
        present_third: "~ue~e", "~ue~en";
        imperfect_first: "~o~ía", "~o~íamos";
        imperfect_second: "~o~ías", "~o~íais";
        imperfect_third: "~o~ía", "~o~ían";
        preterite_first: "~o~í", "~o~imos";
        preterite_second: "~o~iste", "~o~isteis";
        preterite_third: "~u~ió", "~u~ieron";
        future_first: "~o~iré", "~o~iremos";
        future_second: "~o~irás", "~o~iréis";
        future_third: "~o~irá", "~o~irán";
        conditional_first: "~o~iría", "~o~iríamos";
        conditional_second: "~o~irías", "~o~iríais";
        conditional_third: "~o~iría", "~o~irían";
        present_subjunctive_first: "~ue~a", "~u~amos";
        present_subjunctive_second: "~ue~as", "~u~áis";
        present_subjunctive_third: "~ue~a", "~ue~an";
        imperfect_subjunctive_first: "~u~iera", "~u~iéramos";
        imperfect_subjunctive_second: "~u~ieras", "~u~ierais";
        imperfect_subjunctive_third: "~u~iera", "~u~ieran";
        imperfect_subjunctive_se_first: "~u~iese", "~u~iésemos";
        imperfect_subjunctive_se_second: "~u~ieses", "~u~ieseis";
        imperfect_subjunctive_se_third: "~u~iese", "~u~iesen";
        future_subjunctive_first: "~u~iere", "~u~iéremos";
        future_subjunctive_second: "~u~ieres", "~u~iereis";
        future_subjunctive_third: "~u~iere", "~u~ieren";
        vos: "~o~ís", "~u~ás", "~o~ite";
        imperative: "~ué~ete", "~ué~ase", "~u~ámonos", "~o~íos", "~ué~anse";
        negative_imperative: "~ue~as", "~ue~a", "~u~amos", "~u~áis", "~ue~an";
        infinitive: "~o~irse";
        gerund: "~u~iéndose";
        participle_masculine: "~o~ido", "~o~idos";
        participle_feminine: "~o~ida", "~o~idas";
    ),
    // Morirse: 1 words (morirse).
    conjugation!("morirse", 1, "~orirse";
        present_first: "~uero", "~orimos";
        present_second: "~ueres", "~orís";
        present_third: "~uere", "~ueren";
        imperfect_first: "~oría", "~oríamos";
        imperfect_second: "~orías", "~oríais";
        imperfect_third: "~oría", "~orían";
        preterite_first: "~orí", "~orimos";
        preterite_second: "~oriste", "~oristeis";
        preterite_third: "~urió", "~urieron";
        future_first: "~oriré", "~oriremos";
        future_second: "~orirás", "~oriréis";
        future_third: "~orirá", "~orirán";
        conditional_first: "~oriría", "~oriríamos";
        conditional_second: "~orirías", "~oriríais";
        conditional_third: "~oriría", "~orirían";
        present_subjunctive_first: "~uera", "~uramos";
        present_subjunctive_second: "~ueras", "~uráis";
        present_subjunctive_third: "~uera", "~ueran";
        imperfect_subjunctive_first: "~uriera", "~uriéramos";
        imperfect_subjunctive_second: "~urieras", "~urierais";
        imperfect_subjunctive_third: "~uriera", "~urieran";
        imperfect_subjunctive_se_first: "~uriese", "~uriésemos";
        imperfect_subjunctive_se_second: "~urieses", "~urieseis";
        imperfect_subjunctive_se_third: "~uriese", "~uriesen";
        future_subjunctive_first: "~uriere", "~uriéremos";
        future_subjunctive_second: "~urieres", "~uriereis";
        future_subjunctive_third: "~uriere", "~urieren";
        vos: "~orís", "~urás", "~orite";
        imperative: "~uérete", "~uérase", "~urámonos", "~oríos", "~uéranse";
        negative_imperative: "~ueras", "~uera", "~uramos", "~uráis", "~ueran";
        infinitive: "~orirse";
        gerund: "~uriéndose";
        participle_masculine: "~uerto", "~uertos";
        participle_feminine: "~uerta", "~uertas";
    ),
    // Eír: 1 words (eír).
    conjugation!("eír", 1, "eír";
        present_first: "ío", "eímos";
        present_second: "íes", "eís";
        present_third: "íe", "íen";
        imperfect_first: "eía", "eíamos";
        imperfect_second: "eías", "eíais";
        imperfect_third: "eía", "eían";
        preterite_first: "eí", "eímos";
        preterite_second: "eíste", "eísteis";
        preterite_third: "io", "ieron";
        future_first: "eiré", "eiremos";
        future_second: "eirás", "eiréis";
        future_third: "eirá", "eirán";
        conditional_first: "eiría", "eiríamos";
        conditional_second: "eirías", "eiríais";
        conditional_third: "eiría", "eirían";
        present_subjunctive_first: "ía", "iamos";
        present_subjunctive_second: "ías", "iais";
        present_subjunctive_third: "ía", "ían";
        imperfect_subjunctive_first: "iera", "iéramos";
        imperfect_subjunctive_second: "ieras", "ierais";
        imperfect_subjunctive_third: "iera", "ieran";
        imperfect_subjunctive_se_first: "iese", "iésemos";
        imperfect_subjunctive_se_second: "ieses", "ieseis";
        imperfect_subjunctive_se_third: "iese", "iesen";
        future_subjunctive_first: "iere", "iéremos";
        future_subjunctive_second: "ieres", "iereis";
        future_subjunctive_third: "iere", "ieren";
        vos: "eís", "ias", "eí";
        imperative: "íe", "ía", "iamos", "eíd", "ían";
        negative_imperative: "ías", "ía", "iamos", "iais", "ían";
        infinitive: "eír";
        gerund: "iendo";
        participle_masculine: "eído|ito", "eídos|itos";
        participle_feminine: "eída|ita", "eídas|itas";
    ),
    // Aír: 1 words (aír).
    conjugation!("aír", 1, "~ír";
        present_first: "~yo", "~ímos";
        present_second: "~es", "~ís";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~ímos";
        preterite_second: "~íste", "~ísteis";
        preterite_third: "~yó", "~yeron";
        future_first: "~iré", "~iremos";
        future_second: "~irás", "~iréis";
        future_third: "~irá", "~irán";
        conditional_first: "~iría", "~iríamos";
        conditional_second: "~irías", "~iríais";
        conditional_third: "~iría", "~irían";
        present_subjunctive_first: "~ya", "~yamos";
        present_subjunctive_second: "~yas", "~yáis";
        present_subjunctive_third: "~ya", "~yan";
        imperfect_subjunctive_first: "~yera", "~yéramos";
        imperfect_subjunctive_second: "~yeras", "~yerais";
        imperfect_subjunctive_third: "~yera", "~yeran";
        imperfect_subjunctive_se_first: "~yese", "~yésemos";
        imperfect_subjunctive_se_second: "~yeses", "~yeseis";
        imperfect_subjunctive_se_third: "~yese", "~yesen";
        future_subjunctive_first: "~yere", "~yéremos";
        future_subjunctive_second: "~yeres", "~yereis";
        future_subjunctive_third: "~yere", "~yeren";
        vos: "~ís", "~yás", "~í";
        imperative: "~e", "~ya", "~yamos", "~íd", "~yan";
        negative_imperative: "~yas", "~ya", "~yamos", "~yáis", "~yan";
        infinitive: "~ír";
        gerund: "~yendo";
        participle_masculine: "~ído", "~ídos";
        participle_feminine: "~ída", "~ídas";
    ),
    // Entirse: 1 words (entirse).
    conjugation!("entirse", 1, "e~irse";
        present_first: "ie~o", "e~imos";
        present_second: "ie~es", "e~ís";
        present_third: "ie~e", "ie~en";
        imperfect_first: "e~ía", "e~íamos";
        imperfect_second: "e~ías", "e~íais";
        imperfect_third: "e~ía", "e~ían";
        preterite_first: "e~í", "e~imos";
        preterite_second: "e~iste", "e~isteis";
        preterite_third: "i~ió", "i~ieron";
        future_first: "e~iré", "e~iremos";
        future_second: "e~irás", "e~iréis";
        future_third: "e~irá", "e~irán";
        conditional_first: "e~iría", "e~iríamos";
        conditional_second: "e~irías", "e~iríais";
        conditional_third: "e~iría", "e~irían";
        present_subjunctive_first: "ie~a", "i~amos";
        present_subjunctive_second: "ie~as", "i~áis";
        present_subjunctive_third: "ie~a", "ie~an";
        imperfect_subjunctive_first: "i~iera", "i~iéramos";
        imperfect_subjunctive_second: "i~ieras", "i~ierais";
        imperfect_subjunctive_third: "i~iera", "i~ieran";
        imperfect_subjunctive_se_first: "i~iese", "i~iésemos";
        imperfect_subjunctive_se_second: "i~ieses", "i~ieseis";
        imperfect_subjunctive_se_third: "i~iese", "i~iesen";
        future_subjunctive_first: "i~iere", "i~iéremos";
        future_subjunctive_second: "i~ieres", "i~iereis";
        future_subjunctive_third: "i~iere", "i~ieren";
        vos: "e~ís", "i~ás", "e~ite";
        imperative: "ié~ete", "ié~ase", "i~ámonos", "e~íos", "ié~anse";
        negative_imperative: "ie~as", "ie~a", "i~amos", "i~áis", "ie~an";
        infinitive: "e~irse";
        gerund: "i~iéndose";
        participle_masculine: "e~ido", "e~idos";
        participle_feminine: "e~ida", "e~idas";
    ),
    // Edirse: 1 words (edirse).
    conjugation!("edirse", 1, "edirse";
        present_first: "ido", "edimos";
        present_second: "ides", "edís";
        present_third: "ide", "iden";
        imperfect_first: "edía", "edíamos";
        imperfect_second: "edías", "edíais";
        imperfect_third: "edía", "edían";
        preterite_first: "edí", "edimos";
        preterite_second: "ediste", "edisteis";
        preterite_third: "idió", "idieron";
        future_first: "ediré", "ediremos";
        future_second: "edirás", "ediréis";
        future_third: "edirá", "edirán";
        conditional_first: "ediría", "ediríamos";
        conditional_second: "edirías", "ediríais";
        conditional_third: "ediría", "edirían";
        present_subjunctive_first: "ida", "idamos";
        present_subjunctive_second: "idas", "idáis";
        present_subjunctive_third: "ida", "idan";
        imperfect_subjunctive_first: "idiera", "idiéramos";
        imperfect_subjunctive_second: "idieras", "idierais";
        imperfect_subjunctive_third: "idiera", "idieran";
        imperfect_subjunctive_se_first: "idiese", "idiésemos";
        imperfect_subjunctive_se_second: "idieses", "idieseis";
        imperfect_subjunctive_se_third: "idiese", "idiesen";
        future_subjunctive_first: "idiere", "idiéremos";
        future_subjunctive_second: "idieres", "idiereis";
        future_subjunctive_third: "idiere", "idieren";
        vos: "edís", "idás", "edite";
        imperative: "ídete", "ídase", "idámonos", "edíos", "ídanse";
        negative_imperative: "idas", "ida", "idamos", "idáis", "idan";
        infinitive: "edirse";
        gerund: "idiéndose";
        participle_masculine: "edido", "edidos";
        participle_feminine: "edida", "edidas";
    ),
    // Arse: 1 words (arse).
    conjugation!("arse", 1, "arse";
        present_first: "oy", "amos";
        present_second: "ás", "áis";
        present_third: "á", "án";
        imperfect_first: "aba", "ábamos";
        imperfect_second: "abas", "abais";
        imperfect_third: "aba", "aban";
        preterite_first: "uve", "uvimos";
        preterite_second: "uviste", "uvisteis";
        preterite_third: "uvo", "uvieron";
        future_first: "aré", "aremos";
        future_second: "arás", "aréis";
        future_third: "ará", "arán";
        conditional_first: "aría", "aríamos";
        conditional_second: "arías", "aríais";
        conditional_third: "aría", "arían";
        present_subjunctive_first: "é", "emos";
        present_subjunctive_second: "és", "éis";
        present_subjunctive_third: "é", "én";
        imperfect_subjunctive_first: "uviera", "uviéramos";
        imperfect_subjunctive_second: "uvieras", "uvierais";
        imperfect_subjunctive_third: "uviera", "uvieran";
        imperfect_subjunctive_se_first: "uviese", "uviésemos";
        imperfect_subjunctive_se_second: "uvieses", "uvieseis";
        imperfect_subjunctive_se_third: "uviese", "uviesen";
        future_subjunctive_first: "uviere", "uviéremos";
        future_subjunctive_second: "uvieres", "uviereis";
        future_subjunctive_third: "uviere", "uvieren";
        vos: "ás", "és", "ate";
        imperative: "ate", "ese", "émonos", "aos", "ense";
        negative_imperative: "és", "é", "emos", "éis", "én";
        infinitive: "arse";
        gerund: "ándose";
        participle_masculine: "ado", "ados";
        participle_feminine: "ada", "adas";
    ),
    // Etirse: 1 words (etirse).
    conjugation!("etirse", 1, "etirse";
        present_first: "ito", "etimos";
        present_second: "ites", "etís";
        present_third: "ite", "iten";
        imperfect_first: "etía", "etíamos";
        imperfect_second: "etías", "etíais";
        imperfect_third: "etía", "etían";
        preterite_first: "etí", "etimos";
        preterite_second: "etiste", "etisteis";
        preterite_third: "itió", "itieron";
        future_first: "etiré", "etiremos";
        future_second: "etirás", "etiréis";
        future_third: "etirá", "etirán";
        conditional_first: "etiría", "etiríamos";
        conditional_second: "etirías", "etiríais";
        conditional_third: "etiría", "etirían";
        present_subjunctive_first: "ita", "itamos";
        present_subjunctive_second: "itas", "itáis";
        present_subjunctive_third: "ita", "itan";
        imperfect_subjunctive_first: "itiera", "itiéramos";
        imperfect_subjunctive_second: "itieras", "itierais";
        imperfect_subjunctive_third: "itiera", "itieran";
        imperfect_subjunctive_se_first: "itiese", "itiésemos";
        imperfect_subjunctive_se_second: "itieses", "itieseis";
        imperfect_subjunctive_se_third: "itiese", "itiesen";
        future_subjunctive_first: "itiere", "itiéremos";
        future_subjunctive_second: "itieres", "itiereis";
        future_subjunctive_third: "itiere", "itieren";
        vos: "etís", "itás", "etite";
        imperative: "ítete", "ítase", "itámonos", "etíos", "ítanse";
        negative_imperative: "itas", "ita", "itamos", "itáis", "itan";
        infinitive: "etirse";
        gerund: "itiéndose";
        participle_masculine: "etido", "etidos";
        participle_feminine: "etida", "etidas";
    ),
    // Eñirse: 1 words (eñirse).
    conjugation!("eñirse", 1, "eñirse";
        present_first: "iño", "eñimos";
        present_second: "iñes", "eñís";
        present_third: "iñe", "iñen";
        imperfect_first: "eñía", "eñíamos";
        imperfect_second: "eñías", "eñíais";
        imperfect_third: "eñía", "eñían";
        preterite_first: "eñí", "eñimos";
        preterite_second: "eñiste", "eñisteis";
        preterite_third: "iñó", "iñeron";
        future_first: "eñiré", "eñiremos";
        future_second: "eñirás", "eñiréis";
        future_third: "eñirá", "eñirán";
        conditional_first: "eñiría", "eñiríamos";
        conditional_second: "eñirías", "eñiríais";
        conditional_third: "eñiría", "eñirían";
        present_subjunctive_first: "iña", "iñamos";
        present_subjunctive_second: "iñas", "iñáis";
        present_subjunctive_third: "iña", "iñan";
        imperfect_subjunctive_first: "iñera", "iñéramos";
        imperfect_subjunctive_second: "iñeras", "iñerais";
        imperfect_subjunctive_third: "iñera", "iñeran";
        imperfect_subjunctive_se_first: "iñese", "iñésemos";
        imperfect_subjunctive_se_second: "iñeses", "iñeseis";
        imperfect_subjunctive_se_third: "iñese", "iñesen";
        future_subjunctive_first: "iñere", "iñéremos";
        future_subjunctive_second: "iñeres", "iñereis";
        future_subjunctive_third: "iñere", "iñeren";
        vos: "eñís", "iñás", "eñite";
        imperative: "íñete", "íñase", "iñámonos", "eñíos", "íñanse";
        negative_imperative: "iñas", "iña", "iñamos", "iñáis", "iñan";
        infinitive: "eñirse";
        gerund: "iñéndose";
        participle_masculine: "eñido", "eñidos";
        participle_feminine: "eñida", "eñidas";
    ),
    // Ormirse: 1 words (ormirse).
    conjugation!("ormirse", 1, "o~irse";
        present_first: "ue~o", "o~imos";
        present_second: "ue~es", "o~ís";
        present_third: "ue~e", "ue~en";
        imperfect_first: "o~ía", "o~íamos";
        imperfect_second: "o~ías", "o~íais";
        imperfect_third: "o~ía", "o~ían";
        preterite_first: "o~í", "o~imos";
        preterite_second: "o~iste", "o~isteis";
        preterite_third: "u~ió", "u~ieron";
        future_first: "o~iré", "o~iremos";
        future_second: "o~irás", "o~iréis";
        future_third: "o~irá", "o~irán";
        conditional_first: "o~iría", "o~iríamos";
        conditional_second: "o~irías", "o~iríais";
        conditional_third: "o~iría", "o~irían";
        present_subjunctive_first: "ue~a", "u~amos";
        present_subjunctive_second: "ue~as", "u~áis";
        present_subjunctive_third: "ue~a", "ue~an";
        imperfect_subjunctive_first: "u~iera", "u~iéramos";
        imperfect_subjunctive_second: "u~ieras", "u~ierais";
        imperfect_subjunctive_third: "u~iera", "u~ieran";
        imperfect_subjunctive_se_first: "u~iese", "u~iésemos";
        imperfect_subjunctive_se_second: "u~ieses", "u~ieseis";
        imperfect_subjunctive_se_third: "u~iese", "u~iesen";
        future_subjunctive_first: "u~iere", "u~iéremos";
        future_subjunctive_second: "u~ieres", "u~iereis";
        future_subjunctive_third: "u~iere", "u~ieren";
        vos: "o~ís", "u~ás", "o~ite";
        imperative: "ué~ete", "ué~ase", "u~ámonos", "o~íos", "ué~anse";
        negative_imperative: "ue~as", "ue~a", "u~amos", "u~áis", "ue~an";
        infinitive: "o~irse";
        gerund: "u~iéndose";
        participle_masculine: "o~ido", "o~idos";
        participle_feminine: "o~ida", "o~idas";
    ),
    // Eírse: 1 words (eírse).
    conjugation!("eírse", 1, "eírse";
        present_first: "ío", "eímos";
        present_second: "íes", "eís";
        present_third: "íe", "íen";
        imperfect_first: "eía", "eíamos";
        imperfect_second: "eías", "eíais";
        imperfect_third: "eía", "eían";
        preterite_first: "eí", "eímos";
        preterite_second: "eíste", "eísteis";
        preterite_third: "ió", "ieron";
        future_first: "eiré", "eiremos";
        future_second: "eirás", "eiréis";
        future_third: "eirá", "eirán";
        conditional_first: "eiría", "eiríamos";
        conditional_second: "eirías", "eiríais";
        conditional_third: "eiría", "eirían";
        present_subjunctive_first: "ía", "iamos";
        present_subjunctive_second: "ías", "iáis";
        present_subjunctive_third: "ía", "ían";
        imperfect_subjunctive_first: "iera", "iéramos";
        imperfect_subjunctive_second: "ieras", "ierais";
        imperfect_subjunctive_third: "iera", "ieran";
        imperfect_subjunctive_se_first: "iese", "iésemos";
        imperfect_subjunctive_se_second: "ieses", "ieseis";
        imperfect_subjunctive_se_third: "iese", "iesen";
        future_subjunctive_first: "iere", "iéremos";
        future_subjunctive_second: "ieres", "iereis";
        future_subjunctive_third: "iere", "ieren";
        vos: "eís", "iás", "eíte";
        imperative: "íete", "íase", "iámonos", "eíos", "íanse";
        negative_imperative: "ías", "ía", "iamos", "iáis", "ían";
        infinitive: "eírse";
        gerund: "iéndose";
        participle_masculine: "eído", "eídos";
        participle_feminine: "eída", "eídas";
    ),
    // Estirse: 1 words (estirse).
    conjugation!("estirse", 1, "e~tirse";
        present_first: "i~to", "e~timos";
        present_second: "i~tes", "e~tís";
        present_third: "i~te", "i~ten";
        imperfect_first: "e~tía", "e~tíamos";
        imperfect_second: "e~tías", "e~tíais";
        imperfect_third: "e~tía", "e~tían";
        preterite_first: "e~tí", "e~timos";
        preterite_second: "e~tiste", "e~tisteis";
        preterite_third: "i~tió", "i~tieron";
        future_first: "e~tiré", "e~tiremos";
        future_second: "e~tirás", "e~tiréis";
        future_third: "e~tirá", "e~tirán";
        conditional_first: "e~tiría", "e~tiríamos";
        conditional_second: "e~tirías", "e~tiríais";
        conditional_third: "e~tiría", "e~tirían";
        present_subjunctive_first: "i~ta", "i~tamos";
        present_subjunctive_second: "i~tas", "i~táis";
        present_subjunctive_third: "i~ta", "i~tan";
        imperfect_subjunctive_first: "i~tiera", "i~tiéramos";
        imperfect_subjunctive_second: "i~tieras", "i~tierais";
        imperfect_subjunctive_third: "i~tiera", "i~tieran";
        imperfect_subjunctive_se_first: "i~tiese", "i~tiésemos";
        imperfect_subjunctive_se_second: "i~tieses", "i~tieseis";
        imperfect_subjunctive_se_third: "i~tiese", "i~tiesen";
        future_subjunctive_first: "i~tiere", "i~tiéremos";
        future_subjunctive_second: "i~tieres", "i~tiereis";
        future_subjunctive_third: "i~tiere", "i~tieren";
        vos: "e~tís", "i~tás", "e~tite";
        imperative: "í~tete", "í~tase", "i~támonos", "e~tíos", "í~tanse";
        negative_imperative: "i~tas", "i~ta", "i~tamos", "i~táis", "i~tan";
        infinitive: "e~tirse";
        gerund: "i~tiéndose";
        participle_masculine: "e~tido", "e~tidos";
        participle_feminine: "e~tida", "e~tidas";
    ),
    // Eerse: 1 words (eerse).
    conjugation!("eerse", 1, "~erse";
        present_first: "~o", "~emos";
        present_second: "~es", "~éis";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~ímos";
        preterite_second: "~íste", "~ísteis";
        preterite_third: "~yó", "~yeron";
        future_first: "~eré", "~eremos";
        future_second: "~erás", "~eréis";
        future_third: "~erá", "~erán";
        conditional_first: "~ería", "~eríamos";
        conditional_second: "~erías", "~eríais";
        conditional_third: "~ería", "~erían";
        present_subjunctive_first: "~a", "~amos";
        present_subjunctive_second: "~as", "~áis";
        present_subjunctive_third: "~a", "~an";
        imperfect_subjunctive_first: "~yera", "~yéramos";
        imperfect_subjunctive_second: "~yeras", "~yerais";
        imperfect_subjunctive_third: "~yera", "~yeran";
        imperfect_subjunctive_se_first: "~yese", "~yésemos";
        imperfect_subjunctive_se_second: "~yeses", "~yeseis";
        imperfect_subjunctive_se_third: "~yese", "~yesen";
        future_subjunctive_first: "~yere", "~yéremos";
        future_subjunctive_second: "~yeres", "~yereis";
        future_subjunctive_third: "~yere", "~yeren";
        vos: "~és", "~ás", "~ete";
        imperative: "é~te", "éas~", "~ámonos", "~eos", "éans~";
        negative_imperative: "~as", "~a", "~amos", "~áis", "~an";
        infinitive: "~erse";
        gerund: "~yéndose";
        participle_masculine: "~ído", "~ídos";
        participle_feminine: "~ída", "~ídas";
    ),
    // Aírse: 1 words (aírse).
    conjugation!("aírse", 1, "~írse";
        present_first: "~yo", "~ímos";
        present_second: "~es", "~ís";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~ímos";
        preterite_second: "~íste", "~ísteis";
        preterite_third: "~yó", "~yeron";
        future_first: "~iré", "~iremos";
        future_second: "~irás", "~iréis";
        future_third: "~irá", "~irán";
        conditional_first: "~iría", "~iríamos";
        conditional_second: "~irías", "~iríais";
        conditional_third: "~iría", "~irían";
        present_subjunctive_first: "~ya", "~yamos";
        present_subjunctive_second: "~yas", "~yáis";
        present_subjunctive_third: "~ya", "~yan";
        imperfect_subjunctive_first: "~yera", "~yéramos";
        imperfect_subjunctive_second: "~yeras", "~yerais";
        imperfect_subjunctive_third: "~yera", "~yeran";
        imperfect_subjunctive_se_first: "~yese", "~yésemos";
        imperfect_subjunctive_se_second: "~yeses", "~yeseis";
        imperfect_subjunctive_se_third: "~yese", "~yesen";
        future_subjunctive_first: "~yere", "~yéremos";
        future_subjunctive_second: "~yeres", "~yereis";
        future_subjunctive_third: "~yere", "~yeren";
        vos: "~ís", "~yás", "~íte";
        imperative: "~e", "~ya", "~yamos", "~íos", "~yan";
        negative_imperative: "~yas", "~ya", "~yamos", "~yáis", "~yan";
        infinitive: "~írse";
        gerund: "~yendo";
        participle_masculine: "~ído", "~ídos";
        participle_feminine: "~ída", "~ídas";
    ),
    // Odrirse: 1 words (odrirse).
    conjugation!("odrirse", 1, "o~irse";
        present_first: "u~o", "o~imos";
        present_second: "u~es", "o~ís";
        present_third: "u~e", "u~en";
        imperfect_first: "o~ía", "o~íamos";
        imperfect_second: "o~ías", "o~íais";
        imperfect_third: "o~ía", "o~ían";
        preterite_first: "o~í", "o~imos";
        preterite_second: "o~iste", "o~isteis";
        preterite_third: "u~ió", "u~ieron";
        future_first: "o~iré", "o~iremos";
        future_second: "o~irás", "o~iréis";
        future_third: "o~irá", "o~irán";
        conditional_first: "o~iría", "o~iríamos";
        conditional_second: "o~irías", "o~iríais";
        conditional_third: "o~iría", "o~irían";
        present_subjunctive_first: "u~a", "u~amos";
        present_subjunctive_second: "u~as", "u~áis";
        present_subjunctive_third: "u~a", "u~an";
        imperfect_subjunctive_first: "u~iera", "u~iéramos";
        imperfect_subjunctive_second: "u~ieras", "u~ierais";
        imperfect_subjunctive_third: "u~iera", "u~ieran";
        imperfect_subjunctive_se_first: "u~iese", "u~iésemos";
        imperfect_subjunctive_se_second: "u~ieses", "u~ieseis";
        imperfect_subjunctive_se_third: "u~iese", "u~iesen";
        future_subjunctive_first: "u~iere", "u~iéremos";
        future_subjunctive_second: "u~ieres", "u~iereis";
        future_subjunctive_third: "u~iere", "u~ieren";
        vos: "o~ís", "u~ás", "o~ite";
        imperative: "ú~ete", "ú~ase", "u~ámonos", "o~íos", "ú~anse";
        negative_imperative: "u~as", "u~a", "u~amos", "u~áis", "u~an";
        infinitive: "o~irse";
        gerund: "u~iéndose";
        participle_masculine: "o~ido", "o~idos";
        participle_feminine: "o~ida", "o~idas";
    ),
];
