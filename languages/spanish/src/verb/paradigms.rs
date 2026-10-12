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
    /// Like `echar`: 8,816 words.
    Echar,
    /// Like `realizar`: 1,202 words.
    Realizar,
    /// Like `sacar`: 739 words.
    Sacar,
    /// Like `incautarse`: 630 words.
    Incautarse,
    /// Like `acomodarse`: 567 words.
    Acomodarse,
    /// Like `quejarse`: 528 words.
    Quejarse,
    /// Like `rilarse`: 496 words.
    Rilarse,
    /// Like `pegar`: 387 words.
    Pegar,
    /// Like `emputarse`: 367 words.
    Emputarse,
    /// Like `regodearse`: 358 words.
    Regodearse,
    /// Like `curtir`: 321 words.
    Curtir,
    /// Like `saber`: 300 words.
    Saber,
    /// Like `fosilizarse`: 252 words.
    Fosilizarse,
    /// Like `autoabastecerse`: 165 words.
    Autoabastecerse,
    /// Like `meter`: 149 words.
    Meter,
    /// Like `osificarse`: 134 words.
    Osificarse,
    /// Like `cerrar`: 132 words.
    Cerrar,
    /// Like `darse`: 115 words.
    Darse,
    /// Like `desviar`: 109 words.
    Desviar,
    /// Like `volar`: 105 words.
    Volar,
    /// Like `instruir`: 62 words.
    Instruir,
    /// Like `encenagarse`: 61 words.
    Encenagarse,
    /// Like `sentir`: 56 words.
    Sentir,
    /// Like `situar`: 56 words.
    Situar,
    /// Like `atravesarse`: 56 words.
    Atravesarse,
    /// Like `musirse`: 54 words.
    Musirse,
    /// Like `aborrascarse`: 47 words.
    Aborrascarse,
    /// Like `dirigir`: 46 words.
    Dirigir,
    /// Like `bullir`: 43 words.
    Bullir,
    /// Like `coligarse`: 40 words.
    Coligarse,
    /// Like `resfriarse`: 40 words.
    Resfriarse,
    /// Like `tender`: 39 words.
    Tender,
    /// Like `poner`: 37 words.
    Poner,
    /// Like `entremeterse`: 35 words.
    Entremeterse,
    /// Like `desencontrarse`: 35 words.
    Desencontrarse,
    /// Like `fraguar`: 32 words.
    Fraguar,
    /// Like `descocarse`: 31 words.
    Descocarse,
    /// Like `pegarse`: 30 words.
    Pegarse,
    /// Like `desvivirse`: 30 words.
    Desvivirse,
    /// Like `embazarse`: 27 words.
    Embazarse,
    /// Like `destoserse`: 26 words.
    Destoserse,
    /// Like `regar`: 25 words.
    Regar,
    /// Like `arrugarse`: 25 words.
    Arrugarse,
    /// Like `venir`: 24 words.
    Venir,
    /// Like `reducir`: 24 words.
    Reducir,
    /// Like `escribir`: 23 words.
    Escribir,
    /// Like `situarse`: 23 words.
    Situarse,
    /// Like `sacarse`: 23 words.
    Sacarse,
    /// Like `meliar`: 23 words.
    Meliar,
    /// Like `coger`: 22 words.
    Coger,
    /// Like `descomponerse`: 22 words.
    Descomponerse,
    /// Like `alzarse`: 22 words.
    Alzarse,
    /// Like `inmiscuirse`: 21 words.
    Inmiscuirse,
    /// Like `enamoriscarse`: 21 words.
    Enamoriscarse,
    /// Like `fruncir`: 20 words.
    Fruncir,
    /// Like `aunar`: 20 words.
    Aunar,
    /// Like `abatirse`: 20 words.
    Abatirse,
    /// Like `chuzarse`: 19 words.
    Chuzarse,
    /// Like `enfoscarse`: 19 words.
    Enfoscarse,
    /// Like `ahijar`: 18 words.
    Ahijar,
    /// Like `calzarse`: 18 words.
    Calzarse,
    /// Like `proveer`: 17 words.
    Proveer,
    /// Like `endrogarse`: 17 words.
    Endrogarse,
    /// Like `desentenderse`: 17 words.
    Desentenderse,
    /// Like `traer`: 16 words.
    Traer,
    /// Like `resentirse`: 16 words.
    Resentirse,
    /// Like `rendir`: 15 words.
    Rendir,
    /// Like `apuñuscarse`: 15 words.
    Apuñuscarse,
    /// Like `revolver`: 14 words.
    Revolver,
    /// Like `entecarse`: 14 words.
    Entecarse,
    /// Like `regarse`: 14 words.
    Regarse,
    /// Like `rozarse`: 14 words.
    Rozarse,
    /// Like `vencer`: 13 words.
    Vencer,
    /// Like `tener`: 13 words.
    Tener,
    /// Like `ceñir`: 13 words.
    Ceñir,
    /// Like `hacer`: 12 words.
    Hacer,
    /// Like `enraizar`: 12 words.
    Enraizar,
    /// Like `ahilarse`: 12 words.
    Ahilarse,
    /// Like `armarse`: 12 words.
    Armarse,
    /// Like `empezar`: 11 words.
    Empezar,
    /// Like `decir`: 11 words.
    Decir,
    /// Like `escabullirse`: 11 words.
    Escabullirse,
    /// Like `apanucarse`: 11 words.
    Apanucarse,
    /// Like `suscribirse`: 10 words.
    Suscribirse,
    /// Like `retraerse`: 10 words.
    Retraerse,
    /// Like `relamerse`: 10 words.
    Relamerse,
    /// Like `pedir`: 9 words.
    Pedir,
    /// Like `abrir`: 9 words.
    Abrir,
    /// Like `lucir`: 9 words.
    Lucir,
    /// Like `refreír`: 9 words.
    Refreír,
    /// Like `colgar`: 9 words.
    Colgar,
    /// Like `trocar`: 9 words.
    Trocar,
    /// Like `resolverse`: 9 words.
    Resolverse,
    /// Like `colarse`: 9 words.
    Colarse,
    /// Like `detenerse`: 9 words.
    Detenerse,
    /// Like `creerse`: 9 words.
    Creerse,
    /// Like `santiguarse`: 9 words.
    Santiguarse,
    /// Like `guiar`: 8 words.
    Guiar,
    /// Like `reducirse`: 8 words.
    Reducirse,
    /// Like `entercarse`: 8 words.
    Entercarse,
    /// Like `desmedirse`: 8 words.
    Desmedirse,
    /// Like `moler`: 7 words.
    Moler,
    /// Like `regir`: 7 words.
    Regir,
    /// Like `mover`: 7 words.
    Mover,
    /// Like `ver`: 7 words.
    Ver,
    /// Like `tañer`: 7 words.
    Tañer,
    /// Like `ahumarse`: 7 words.
    Ahumarse,
    /// Like `venirse`: 7 words.
    Venirse,
    /// Like `deshacerse`: 7 words.
    Deshacerse,
    /// Like `referirse`: 7 words.
    Referirse,
    /// Like `aterirse`: 7 words.
    Aterirse,
    /// Like `jetiarse`: 7 words.
    Jetiarse,
    /// Like `oír`: 6 words.
    Oír,
    /// Like `seguir`: 6 words.
    Seguir,
    /// Like `agorar`: 6 words.
    Agorar,
    /// Like `apajuilar`: 6 words.
    Apajuilar,
    /// Like `entredormirse`: 6 words.
    Entredormirse,
    /// Like `recogerse`: 6 words.
    Recogerse,
    /// Like `caer`: 5 words.
    Caer,
    /// Like `dormir`: 5 words.
    Dormir,
    /// Like `querer`: 5 words.
    Querer,
    /// Like `salir`: 5 words.
    Salir,
    /// Like `huir`: 5 words.
    Huir,
    /// Like `cocer`: 5 words.
    Cocer,
    /// Like `evacuar`: 5 words.
    Evacuar,
    /// Like `delinquir`: 5 words.
    Delinquir,
    /// Like `afligirse`: 5 words.
    Afligirse,
    /// Like `reconocerse`: 5 words.
    Reconocerse,
    /// Like `teñirse`: 5 words.
    Teñirse,
    /// Like `trocarse`: 5 words.
    Trocarse,
    /// Like `reírse`: 5 words.
    Reírse,
    /// Like `revestirse`: 5 words.
    Revestirse,
    /// Like `forzar`: 4 words.
    Forzar,
    /// Like `adquirir`: 4 words.
    Adquirir,
    /// Like `cohibir`: 4 words.
    Cohibir,
    /// Like `morir`: 4 words.
    Morir,
    /// Like `distinguir`: 4 words.
    Distinguir,
    /// Like `torcer`: 4 words.
    Torcer,
    /// Like `repetir`: 4 words.
    Repetir,
    /// Like `concebir`: 4 words.
    Concebir,
    /// Like `desforzarse`: 4 words.
    Desforzarse,
    /// Like `regirse`: 4 words.
    Regirse,
    /// Like `fraguarse`: 4 words.
    Fraguarse,
    /// Like `moverse`: 4 words.
    Moverse,
    /// Like `criarse`: 4 words.
    Criarse,
    /// Like `punzarse`: 4 words.
    Punzarse,
    /// Like `traslucirse`: 4 words.
    Traslucirse,
    /// Like `torcerse`: 4 words.
    Torcerse,
    /// Like `licuarse`: 4 words.
    Licuarse,
    /// Like `vencerse`: 4 words.
    Vencerse,
    /// Like `sonarse`: 4 words.
    Sonarse,
    /// Like `yacer`: 3 words.
    Yacer,
    /// Like `argüir`: 3 words.
    Argüir,
    /// Like `jugar`: 3 words.
    Jugar,
    /// Like `romper`: 3 words.
    Romper,
    /// Like `valer`: 3 words.
    Valer,
    /// Like `imprimir`: 3 words.
    Imprimir,
    /// Like `cernir`: 3 words.
    Cernir,
    /// Like `volcar`: 3 words.
    Volcar,
    /// Like `apucuyar`: 3 words.
    Apucuyar,
    /// Like `antojar`: 3 words.
    Antojar,
    /// Like `colgarse`: 3 words.
    Colgarse,
    /// Like `escocerse`: 3 words.
    Escocerse,
    /// Like `seguirse`: 3 words.
    Seguirse,
    /// Like `azoarse`: 3 words.
    Azoarse,
    /// Like `trenzarse`: 3 words.
    Trenzarse,
    /// Like `cubrirse`: 3 words.
    Cubrirse,
    /// Like `derretirse`: 3 words.
    Derretirse,
    /// Like `volcarse`: 3 words.
    Volcarse,
    /// Like `fingirse`: 3 words.
    Fingirse,
    /// Like `azonzarse`: 3 words.
    Azonzarse,
    /// Like `ir`: 1 words.
    Ir,
    /// Like `embaírse`: 1 words.
    Embaírse,
    /// Like `ingrirse`: 1 words.
    Ingrirse,
    /// Like `irse`: 1 words.
    Irse,
    /// Like `unirse`: 1 words.
    Unirse,
    /// Like `verse`: 1 words.
    Verse,
    /// Like `abrirse`: 1 words.
    Abrirse,
    /// Like `asirse`: 1 words.
    Asirse,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 179] = [
        Conjugation::Echar,
        Conjugation::Realizar,
        Conjugation::Sacar,
        Conjugation::Incautarse,
        Conjugation::Acomodarse,
        Conjugation::Quejarse,
        Conjugation::Rilarse,
        Conjugation::Pegar,
        Conjugation::Emputarse,
        Conjugation::Regodearse,
        Conjugation::Curtir,
        Conjugation::Saber,
        Conjugation::Fosilizarse,
        Conjugation::Autoabastecerse,
        Conjugation::Meter,
        Conjugation::Osificarse,
        Conjugation::Cerrar,
        Conjugation::Darse,
        Conjugation::Desviar,
        Conjugation::Volar,
        Conjugation::Instruir,
        Conjugation::Encenagarse,
        Conjugation::Sentir,
        Conjugation::Situar,
        Conjugation::Atravesarse,
        Conjugation::Musirse,
        Conjugation::Aborrascarse,
        Conjugation::Dirigir,
        Conjugation::Bullir,
        Conjugation::Coligarse,
        Conjugation::Resfriarse,
        Conjugation::Tender,
        Conjugation::Poner,
        Conjugation::Entremeterse,
        Conjugation::Desencontrarse,
        Conjugation::Fraguar,
        Conjugation::Descocarse,
        Conjugation::Pegarse,
        Conjugation::Desvivirse,
        Conjugation::Embazarse,
        Conjugation::Destoserse,
        Conjugation::Regar,
        Conjugation::Arrugarse,
        Conjugation::Venir,
        Conjugation::Reducir,
        Conjugation::Escribir,
        Conjugation::Situarse,
        Conjugation::Sacarse,
        Conjugation::Meliar,
        Conjugation::Coger,
        Conjugation::Descomponerse,
        Conjugation::Alzarse,
        Conjugation::Inmiscuirse,
        Conjugation::Enamoriscarse,
        Conjugation::Fruncir,
        Conjugation::Aunar,
        Conjugation::Abatirse,
        Conjugation::Chuzarse,
        Conjugation::Enfoscarse,
        Conjugation::Ahijar,
        Conjugation::Calzarse,
        Conjugation::Proveer,
        Conjugation::Endrogarse,
        Conjugation::Desentenderse,
        Conjugation::Traer,
        Conjugation::Resentirse,
        Conjugation::Rendir,
        Conjugation::Apuñuscarse,
        Conjugation::Revolver,
        Conjugation::Entecarse,
        Conjugation::Regarse,
        Conjugation::Rozarse,
        Conjugation::Vencer,
        Conjugation::Tener,
        Conjugation::Ceñir,
        Conjugation::Hacer,
        Conjugation::Enraizar,
        Conjugation::Ahilarse,
        Conjugation::Armarse,
        Conjugation::Empezar,
        Conjugation::Decir,
        Conjugation::Escabullirse,
        Conjugation::Apanucarse,
        Conjugation::Suscribirse,
        Conjugation::Retraerse,
        Conjugation::Relamerse,
        Conjugation::Pedir,
        Conjugation::Abrir,
        Conjugation::Lucir,
        Conjugation::Refreír,
        Conjugation::Colgar,
        Conjugation::Trocar,
        Conjugation::Resolverse,
        Conjugation::Colarse,
        Conjugation::Detenerse,
        Conjugation::Creerse,
        Conjugation::Santiguarse,
        Conjugation::Guiar,
        Conjugation::Reducirse,
        Conjugation::Entercarse,
        Conjugation::Desmedirse,
        Conjugation::Moler,
        Conjugation::Regir,
        Conjugation::Mover,
        Conjugation::Ver,
        Conjugation::Tañer,
        Conjugation::Ahumarse,
        Conjugation::Venirse,
        Conjugation::Deshacerse,
        Conjugation::Referirse,
        Conjugation::Aterirse,
        Conjugation::Jetiarse,
        Conjugation::Oír,
        Conjugation::Seguir,
        Conjugation::Agorar,
        Conjugation::Apajuilar,
        Conjugation::Entredormirse,
        Conjugation::Recogerse,
        Conjugation::Caer,
        Conjugation::Dormir,
        Conjugation::Querer,
        Conjugation::Salir,
        Conjugation::Huir,
        Conjugation::Cocer,
        Conjugation::Evacuar,
        Conjugation::Delinquir,
        Conjugation::Afligirse,
        Conjugation::Reconocerse,
        Conjugation::Teñirse,
        Conjugation::Trocarse,
        Conjugation::Reírse,
        Conjugation::Revestirse,
        Conjugation::Forzar,
        Conjugation::Adquirir,
        Conjugation::Cohibir,
        Conjugation::Morir,
        Conjugation::Distinguir,
        Conjugation::Torcer,
        Conjugation::Repetir,
        Conjugation::Concebir,
        Conjugation::Desforzarse,
        Conjugation::Regirse,
        Conjugation::Fraguarse,
        Conjugation::Moverse,
        Conjugation::Criarse,
        Conjugation::Punzarse,
        Conjugation::Traslucirse,
        Conjugation::Torcerse,
        Conjugation::Licuarse,
        Conjugation::Vencerse,
        Conjugation::Sonarse,
        Conjugation::Yacer,
        Conjugation::Argüir,
        Conjugation::Jugar,
        Conjugation::Romper,
        Conjugation::Valer,
        Conjugation::Imprimir,
        Conjugation::Cernir,
        Conjugation::Volcar,
        Conjugation::Apucuyar,
        Conjugation::Antojar,
        Conjugation::Colgarse,
        Conjugation::Escocerse,
        Conjugation::Seguirse,
        Conjugation::Azoarse,
        Conjugation::Trenzarse,
        Conjugation::Cubrirse,
        Conjugation::Derretirse,
        Conjugation::Volcarse,
        Conjugation::Fingirse,
        Conjugation::Azonzarse,
        Conjugation::Ir,
        Conjugation::Embaírse,
        Conjugation::Ingrirse,
        Conjugation::Irse,
        Conjugation::Unirse,
        Conjugation::Verse,
        Conjugation::Abrirse,
        Conjugation::Asirse,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 179] = [
    // Echar: 8,816 words (estar, amar, generar, crear, dar, abandonar).
    conjugation!("echar", 8816, "~ar";
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
    // Realizar: 1,202 words (profundizar, utilizar, realzar, avergonzar, lanzar, alcanzar).
    conjugation!("realizar", 1202, "~zar";
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
    // Sacar: 739 words (buscar, sacar, rascar, pecar, tocar, ubicar).
    conjugation!("sacar", 739, "~car";
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
    // Incautarse: 630 words (llamarse, jambarse, amoratarse, infartarse, autoengañarse, ahorcajarse).
    conjugation!("incautarse", 630, "~a~arse";
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
    // Acomodarse: 567 words (acodarse, acomodarse, personarse, autocontrolarse, amonarse, apersonarse).
    conjugation!("acomodarse", 567, "~o~arse";
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
    // Quejarse: 528 words (quejarse, querellarse, despertarse, autocremarse, afiebrarse, desmonterarse).
    conjugation!("quejarse", 528, "~e~arse";
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
    // Rilarse: 496 words (fijarse, chisparse, ababillarse, agüitarse, refanfinflarse, encapricharse).
    conjugation!("rilarse", 496, "~i~arse";
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
    // Pegar: 387 words (llegar, delegar, fatigar, cabalgar, juzgar, apagar).
    conjugation!("pegar", 387, "~ar";
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
    // Emputarse: 367 words (acalenturarse, emputarse, confabularse, autoinculparse, enfurruñarse, despreocuparse).
    conjugation!("emputarse", 367, "~u~arse";
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
    // Regodearse: 358 words (carcajearse, petatearse, autobombearse, regodearse, mamonearse, cachiporrearse).
    conjugation!("regodearse", 358, "~~arse";
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
    // Curtir: 321 words (partir, polir, abolir, cumplir, finir, vivir).
    conjugation!("curtir", 321, "~ir";
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
    // Saber: 300 words (saber, conocer, caber, placer, haber, nacer).
    conjugation!("saber", 300, "~~er";
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
    // Fosilizarse: 252 words (fosilizarse, autorrealizarse, autobautizarse, autoanalizarse, autopolinizarse, autoorganizarse).
    conjugation!("fosilizarse", 252, "~izarse";
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
    // Autoabastecerse: 165 words (autoabastecerse, autoprotegerse, entigrecerse, reverse, ensilvecerse, establecerse).
    conjugation!("autoabastecerse", 165, "~e~erse";
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
    // Meter: 149 words (absconder, absorber, her, meter, acceder, beber).
    conjugation!("meter", 149, "~er";
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
    // Osificarse: 134 words (gelificarse, automedicarse, autocalificarse, autoaplicarse, osificarse, autoidentificarse).
    conjugation!("osificarse", 134, "~icarse";
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
    // Cerrar: 132 words (pensar, quebrar, helar, gobernar, confesar, atestar).
    conjugation!("cerrar", 132, "~~ar";
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
    // Darse: 115 words (acovacharse, abicharse, abrocharse, arreguincharse, reguincharse, emborracharse).
    conjugation!("darse", 115, "~arse";
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
    // Desviar: 109 words (enviar, enfriar, ampliar, aviar, confiar, agriar).
    conjugation!("desviar", 109, "~iar";
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
    // Volar: 105 words (sonar, soltar, encontrar, volar, solar, acostar).
    conjugation!("volar", 105, "~o~ar";
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
    // Instruir: 62 words (construir, destruir, incluir, excluir, sustituir, disminuir).
    conjugation!("instruir", 62, "~ir";
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
    // Encenagarse: 61 words (autopropagarse, autopagarse, encenagarse, apangarse, cargarse, largarse).
    conjugation!("encenagarse", 61, "~a~arse";
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
    // Sentir: 56 words (mentir, sentir, zaherir, herir, advertir, sugerir).
    conjugation!("sentir", 56, "~e~ir";
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
    // Situar: 56 words (puntuar, continuar, efectuar, actuar, situar, menstruar).
    conjugation!("situar", 56, "~uar";
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
    // Atravesarse: 56 words (autogobernarse, quebrarse, sentarse, atravesarse, gobernarse, acimentarse).
    conjugation!("atravesarse", 56, "~e~arse";
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
    // Musirse: 54 words (desnutrirse, musirse, entumirse, ocurrirse, consumirse, eludirse).
    conjugation!("musirse", 54, "~u~irse";
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
    // Aborrascarse: 47 words (aborrascarse, trancarse, atrancarse, apalancarse, atascarse, bancarse).
    conjugation!("aborrascarse", 47, "~a~carse";
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
    // Dirigir: 46 words (urgir, mugir, exigir, fingir, agir, dirigir).
    conjugation!("dirigir", 46, "~gir";
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
    // Bullir: 43 words (bullir, plañir, mullir, muñir, gruñir, fuñir).
    conjugation!("bullir", 43, "~ir";
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
    // Coligarse: 40 words (endomingarse, coaligarse, coligarse, chingarse, desabrigarse, descuajaringarse).
    conjugation!("coligarse", 40, "~i~arse";
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
    // Resfriarse: 40 words (resfriarse, guiarse, confiarse, demasiarse, enfriarse, aliarse).
    conjugation!("resfriarse", 40, "~iarse";
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
    // Tender: 39 words (tender, perder, encender, defender, ascender, entender).
    conjugation!("tender", 39, "~~er";
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
    // Poner: 37 words (poner, yuxtaponer, imponer, suponer, exponer, posponer).
    conjugation!("poner", 37, "~oner";
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
    // Entremeterse: 35 words (entrometerse, entremeterse, prenderse, comprenderse, tejerse, sorprenderse).
    conjugation!("entremeterse", 35, "~e~erse";
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
    // Desencontrarse: 35 words (desencontrarse, encontrarse, poblarse, encordarse, acostarse, acordarse).
    conjugation!("desencontrarse", 35, "~o~arse";
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
    // Fraguar: 32 words (averiguar, atestiguar, menguar, aguar, testiguar, apaciguar).
    conjugation!("fraguar", 32, "~uar";
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
    // Descocarse: 31 words (autoconvocarse, descocarse, abocarse, apocarse, equivocarse, enfocarse).
    conjugation!("descocarse", 31, "~ocarse";
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
    // Pegarse: 30 words (pegarse, aborregarse, entregarse, llegarse, vengarse, apegarse).
    conjugation!("pegarse", 30, "~e~arse";
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
    // Desvivirse: 30 words (desvivirse, autodefinirse, autopercibirse, inhibirse, extinguirse, distinguirse).
    conjugation!("desvivirse", 30, "~i~irse";
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
    // Embazarse: 27 words (embazarse, abrazarse, acarrazarse, amostazarse, desplazarse, tarazarse).
    conjugation!("embazarse", 27, "~azarse";
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
    // Destoserse: 26 words (destoserse, correrse, coserse, corresponderse, dolerse, carcomerse).
    conjugation!("destoserse", 26, "~o~erse";
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
    // Regar: 25 words (negar, renegar, fregar, regar, segar, sosegar).
    conjugation!("regar", 25, "~~ar";
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
    // Arrugarse: 25 words (añusgarse, destungarse, purgarse, enjugarse, jugarse, fugarse).
    conjugation!("arrugarse", 25, "~u~arse";
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
    // Venir: 24 words (devenir, venir, intervenir, avenir, revenir, convenir).
    conjugation!("venir", 24, "~e~ir";
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
    // Reducir: 24 words (traducir, producir, reducir, conducir, introducir, inducir).
    conjugation!("reducir", 24, "~cir";
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
    // Escribir: 23 words (escribir, describir, inscribir, transcribir, prescribir, proscribir).
    conjugation!("escribir", 23, "~bir";
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
    // Situarse: 23 words (autoevaluarse, adecuarse, atenuarse, extenuarse, situarse, insinuarse).
    conjugation!("situarse", 23, "~uarse";
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
    // Sacarse: 23 words (abarracarse, abellacarse, desajacarse, trelacarse, atracarse, enflacarse).
    conjugation!("sacarse", 23, "~acarse";
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
    // Meliar: 23 words (gilidiar, chalequiar, meliar, cangrejiar, jariniar, jonjoliar).
    conjugation!("meliar", 23, "~iar";
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
    // Coger: 22 words (proteger, coger, recoger, escoger, acoger, emerger).
    conjugation!("coger", 22, "~ger";
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
    // Descomponerse: 22 words (autoimponerse, ponerse, imponerse, disponerse, exponerse, reponerse).
    conjugation!("descomponerse", 22, "~onerse";
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
    // Alzarse: 22 words (emperezarse, esperezarse, alzarse, vezarse, avezarse, avergonzarse).
    conjugation!("alzarse", 22, "~~zarse";
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
    // Inmiscuirse: 21 words (autodestruirse, inmiscuirse, autoexcluirse, autoconstruirse, rehuirse, atribuirse).
    conjugation!("inmiscuirse", 21, "~uirse";
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
    // Enamoriscarse: 21 words (enamoriscarse, ahincarse, afincarse, ciscarse, ariscarse, enriscarse).
    conjugation!("enamoriscarse", 21, "~i~carse";
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
    // Fruncir: 20 words (zurcir, uncir, surcir, esparcir, resarcir, balbucir).
    conjugation!("fruncir", 20, "~cir";
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
    // Aunar: 20 words (aunar, rehusar, ahumar, maullar, aupar, aullar).
    conjugation!("aunar", 20, "~u~ar";
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
    // Abatirse: 20 words (batirse, partirse, persuadirse, abatirse, debatirse, expandirse).
    conjugation!("abatirse", 20, "~a~irse";
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
    // Chuzarse: 19 words (chapuzarse, abruzarse, abuzarse, escagarruzarse, aliruzarse, atapuzarse).
    conjugation!("chuzarse", 19, "~uzarse";
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
    // Enfoscarse: 19 words (embroncarse, abroncarse, enfoscarse, afoscarse, corcarse, coscarse).
    conjugation!("enfoscarse", 19, "~o~carse";
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
    // Ahijar: 18 words (prohijar, descafeinar, aislar, amohinar, ahijar, airar).
    conjugation!("ahijar", 18, "~i~ar";
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
    // Calzarse: 18 words (abalanzarse, ensalzarse, esperanzarse, alianzarse, alcanzarse, desalzarse).
    conjugation!("calzarse", 18, "~a~zarse";
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
    // Proveer: 17 words (peer, creer, leer, veer, seer, roer).
    conjugation!("proveer", 17, "~er";
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
    // Endrogarse: 17 words (endrogarse, togarse, afogarse, prolongarse, logarse, ahogarse).
    conjugation!("endrogarse", 17, "~o~arse";
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
    // Desentenderse: 17 words (desentenderse, autodefenderse, perderse, defenderse, extenderse, sobrentenderse).
    conjugation!("desentenderse", 17, "~e~erse";
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
    // Traer: 16 words (traer, contraer, atraer, extraer, substraer, abstraer).
    conjugation!("traer", 16, "~er";
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
    // Resentirse: 16 words (resentirse, arrepentirse, repentirse, divertirse, sentirse, consentirse).
    conjugation!("resentirse", 16, "~e~irse";
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
    // Rendir: 15 words (servir, vestir, investir, gemir, rendir, desvestir).
    conjugation!("rendir", 15, "~e~ir";
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
    // Apuñuscarse: 15 words (apuñuscarse, bifurcarse, inculcarse, buscarse, rebuscarse, zurruscarse).
    conjugation!("apuñuscarse", 15, "~u~carse";
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
    // Revolver: 14 words (absolver, revolver, resolver, disolver, volver, solver).
    conjugation!("revolver", 14, "~o~ver";
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
    // Entecarse: 14 words (entecarse, enchuecarse, obcecarse, intersecarse, ahuecarse, deprecarse).
    conjugation!("entecarse", 14, "~~carse";
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
    // Regarse: 14 words (abnegarse, regarse, sosegarse, negarse, replegarse, estregarse).
    conjugation!("regarse", 14, "~e~arse";
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
    // Rozarse: 14 words (apozarse, escozarse, arrebozarse, empozarse, alborozarse, desarrebozarse).
    conjugation!("rozarse", 14, "~ozarse";
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
    // Vencer: 13 words (coercer, vencer, ejercer, convencer, mecer, mulcer).
    conjugation!("vencer", 13, "~cer";
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
    // Tener: 13 words (tener, sostener, obtener, detener, mantener, retener).
    conjugation!("tener", 13, "~ener";
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
    // Ceñir: 13 words (teñir, reñir, ceñir, desteñir, estreñir, astreñir).
    conjugation!("ceñir", 13, "~eñir";
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
    // Hacer: 12 words (hacer, satisfacer, facer, deshacer, rehacer, contrahacer).
    conjugation!("hacer", 12, "~acer";
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
    // Enraizar: 12 words (enraizar, europeizar, arcaizar, descorreizar, judaizar, raizar).
    conjugation!("enraizar", 12, "~izar";
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
    // Ahilarse: 12 words (autoaislarse, aislarse, ahitarse, ahilarse, airarse, desaislarse).
    conjugation!("ahilarse", 12, "~i~arse";
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
    // Armarse: 12 words (abarse, armarse, asmarse, alarse, andarse, asparse).
    conjugation!("armarse", 12, "a~arse";
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
    // Empezar: 11 words (empezar, comenzar, tropezar, despezar, enlenzar, entropezar).
    conjugation!("empezar", 11, "~~zar";
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
    // Decir: 11 words (decir, bendecir, maldecir, contradecir, desdecir, predecir).
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
    // Escabullirse: 11 words (escabullirse, zambullirse, rebullirse, tullirse, escullirse, descabullirse).
    conjugation!("escabullirse", 11, "~u~irse";
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
    // Apanucarse: 11 words (acurrucarse, acorrucarse, embejucarse, apitucarse, emborucarse, desnucarse).
    conjugation!("apanucarse", 11, "~ucarse";
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
    // Suscribirse: 10 words (autodescribirse, escribirse, adscribirse, inscribirse, subscribirse, suscribirse).
    conjugation!("suscribirse", 10, "~ibirse";
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
    // Retraerse: 10 words (contraerse, traerse, atraerse, sustraerse, retrotraerse, retraerse).
    conjugation!("retraerse", 10, "~aerse";
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
    // Relamerse: 10 words (valerse, complacerse, prevalerse, aplacerse, relamerse, lamerse).
    conjugation!("relamerse", 10, "~a~erse";
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
    // Pedir: 9 words (pedir, despedir, medir, impedir, expedir, reexpedir).
    conjugation!("pedir", 9, "~edir";
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
    // Abrir: 9 words (cubrir, abrir, reabrir, descubrir, encubrir, entreabrir).
    conjugation!("abrir", 9, "~~ir";
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
    // Lucir: 9 words (lucir, relucir, deslucir, enlucir, traslucir, translucir).
    conjugation!("lucir", 9, "~~ir";
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
    // Refreír: 9 words (reír, freír, sonreír, engreír, desleír, esleír).
    conjugation!("refreír", 9, "~eír";
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
    // Colgar: 9 words (colgar, rogar, alongar, descolgar, holgar, logar).
    conjugation!("colgar", 9, "~o~ar";
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
    // Trocar: 9 words (trocar, trastrocar, clocar, retrocar, destrocar, aclocar).
    conjugation!("trocar", 9, "~ocar";
        present_first: "~ueco", "~ocamos";
        present_second: "~uecas", "~ocáis";
        present_third: "~ueca", "~uecan";
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
        present_subjunctive_first: "~ueque", "~oquemos";
        present_subjunctive_second: "~ueques", "~oquéis";
        present_subjunctive_third: "~ueque", "~uequen";
        imperfect_subjunctive_first: "~ocara", "~ocáramos";
        imperfect_subjunctive_second: "~ocaras", "~ocarais";
        imperfect_subjunctive_third: "~ocara", "~ocaran";
        imperfect_subjunctive_se_first: "~ocase", "~ocásemos";
        imperfect_subjunctive_se_second: "~ocases", "~ocaseis";
        imperfect_subjunctive_se_third: "~ocase", "~ocasen";
        future_subjunctive_first: "~ocare", "~ocáremos";
        future_subjunctive_second: "~ocares", "~ocareis";
        future_subjunctive_third: "~ocare", "~ocaren";
        vos: "~ocás", "~oqués", "~ocá";
        imperative: "~ueca", "~ueque", "~oquemos", "~ocad", "~uequen";
        negative_imperative: "~ueques", "~ueque", "~oquemos", "~oquéis", "~uequen";
        infinitive: "~ocar";
        gerund: "~ocando";
        participle_masculine: "~ocado", "~ocados";
        participle_feminine: "~ocada", "~ocadas";
    ),
    // Resolverse: 9 words (autodisolverse, resolverse, volverse, devolverse, revolverse, desenvolverse).
    conjugation!("resolverse", 9, "~o~verse";
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
        vos: "~o~vés", "~o~vás", "~o~vete";
        imperative: "~ué~vete", "~ué~vase", "~o~vámonos", "~o~veos", "~ué~vanse";
        negative_imperative: "~ue~vas", "~ue~va", "~o~vamos", "~o~váis", "~ue~van";
        infinitive: "~o~verse";
        gerund: "~o~viéndose";
        participle_masculine: "~ue~to", "~ue~tos";
        participle_feminine: "~ue~ta", "~ue~tas";
    ),
    // Colarse: 9 words (colarse, volarse, desolarse, amolarse, consolarse, desconsolarse).
    conjugation!("colarse", 9, "~olarse";
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
    // Detenerse: 9 words (abstenerse, contenerse, sostenerse, mantenerse, detenerse, entretenerse).
    conjugation!("detenerse", 9, "~enerse";
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
        vos: "~enés", "~engás", "~enete";
        imperative: "~ente", "~éngase", "~engámonos", "~eneos", "~énganse";
        negative_imperative: "~engas", "~enga", "~engamos", "~engáis", "~engan";
        infinitive: "~enerse";
        gerund: "~eniéndose";
        participle_masculine: "~enido", "~enidos";
        participle_feminine: "~enida", "~enidas";
    ),
    // Creerse: 9 words (caerse, creerse, peerse, proveerse, corroerse, desposeerse).
    conjugation!("creerse", 9, "~~erse";
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
    // Santiguarse: 9 words (achiguarse, apaciguarse, santiguarse, enmaniguarse, amortiguarse, averiguarse).
    conjugation!("santiguarse", 9, "~i~uarse";
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
    // Guiar: 8 words (liar, fiar, criar, guiar, ciar, piar).
    conjugation!("guiar", 8, "~iar";
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
    // Reducirse: 8 words (autorreproducirse, autoconducirse, autoproducirse, reducirse, reproducirse, introducirse).
    conjugation!("reducirse", 8, "~ucirse";
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
    // Entercarse: 8 words (entercarse, acercarse, cercarse, empuercarse, engrescarse, mercarse).
    conjugation!("entercarse", 8, "~e~carse";
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
    // Desmedirse: 8 words (descomedirse, desmedirse, despedirse, acomedirse, medirse, expedirse).
    conjugation!("desmedirse", 8, "~edirse";
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
    // Moler: 7 words (doler, soler, moler, demoler, condoler, remoler).
    conjugation!("moler", 7, "~oler";
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
    // Regir: 7 words (elegir, corregir, colegir, reelegir, regir, recolegir).
    conjugation!("regir", 7, "~egir";
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
    // Mover: 7 words (llover, promover, mover, remover, conmover, amover).
    conjugation!("mover", 7, "~over";
        present_first: "~uevo", "~ovemos";
        present_second: "~ueves", "~ovéis";
        present_third: "~ueve", "~ueven";
        imperfect_first: "~ovía", "~ovíamos";
        imperfect_second: "~ovías", "~ovíais";
        imperfect_third: "~ovía", "~ovían";
        preterite_first: "~oví", "~ovimos";
        preterite_second: "~oviste", "~ovisteis";
        preterite_third: "~ovió", "~ovieron";
        future_first: "~overé", "~overemos";
        future_second: "~overás", "~overéis";
        future_third: "~overá", "~overán";
        conditional_first: "~overía", "~overíamos";
        conditional_second: "~overías", "~overíais";
        conditional_third: "~overía", "~overían";
        present_subjunctive_first: "~ueva", "~ovamos";
        present_subjunctive_second: "~uevas", "~ováis";
        present_subjunctive_third: "~ueva", "~uevan";
        imperfect_subjunctive_first: "~oviera", "~oviéramos";
        imperfect_subjunctive_second: "~ovieras", "~ovierais";
        imperfect_subjunctive_third: "~oviera", "~ovieran";
        imperfect_subjunctive_se_first: "~oviese", "~oviésemos";
        imperfect_subjunctive_se_second: "~ovieses", "~ovieseis";
        imperfect_subjunctive_se_third: "~oviese", "~oviesen";
        future_subjunctive_first: "~oviere", "~oviéremos";
        future_subjunctive_second: "~ovieres", "~oviereis";
        future_subjunctive_third: "~oviere", "~ovieren";
        vos: "~ovés", "~ovás", "~ové";
        imperative: "~ueve", "~ueva", "~ovamos", "~oved", "~uevan";
        negative_imperative: "~uevas", "~ueva", "~ovamos", "~ováis", "~uevan";
        infinitive: "~over";
        gerund: "~oviendo";
        participle_masculine: "~ovido", "~ovidos";
        participle_feminine: "~ovida", "~ovidas";
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
    // Tañer: 7 words (jier, atañer, tañer, empeller, toller, creyer).
    conjugation!("tañer", 7, "~er";
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
    // Ahumarse: 7 words (ahusarse, ahumarse, aunarse, rehusarse, rehurtarse, sahumarse).
    conjugation!("ahumarse", 7, "~u~arse";
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
    // Venirse: 7 words (venirse, desconvenirse, avenirse, desavenirse, convenirse, prevenirse).
    conjugation!("venirse", 7, "~e~irse";
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
    // Deshacerse: 7 words (autosatisfacerse, hacerse, rehacerse, desfacerse, deshacerse, contrahacerse).
    conjugation!("deshacerse", 7, "~acerse";
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
        vos: "~acés", "~agás", "~acete";
        imperative: "~azte", "~ágase", "~agámonos", "~aceos", "~áganse";
        negative_imperative: "~agas", "~aga", "~agamos", "~agáis", "~agan";
        infinitive: "~acerse";
        gerund: "~aciéndose";
        participle_masculine: "~echo", "~echos";
        participle_feminine: "~echa", "~echas";
    ),
    // Referirse: 7 words (referirse, injerirse, proferirse, adherirse, inferirse, interferirse).
    conjugation!("referirse", 7, "~erirse";
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
    // Aterirse: 7 words (aterirse, empedernirse, arrecirse, hespirse, denegrirse, sumergirse).
    conjugation!("aterirse", 7, "~e~irse";
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
    // Jetiarse: 7 words (mamoniarse, patrasiarse, friquiarse, paniquiarse, jetiarse, penquiarse).
    conjugation!("jetiarse", 7, "~iarse";
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
    // Oír: 6 words (oír, embaír, desvaír, desoír, trasoír, entreoír).
    conjugation!("oír", 6, "~ír";
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
    // Seguir: 6 words (proseguir, perseguir, seguir, conseguir, subseguir, reseguir).
    conjugation!("seguir", 6, "~eguir";
        present_first: "~igo", "~eguimos";
        present_second: "~igues", "~eguís";
        present_third: "~igue", "~iguen";
        imperfect_first: "~eguía", "~eguíamos";
        imperfect_second: "~eguías", "~eguíais";
        imperfect_third: "~eguía", "~eguían";
        preterite_first: "~eguí", "~eguimos";
        preterite_second: "~eguiste", "~eguisteis";
        preterite_third: "~iguió", "~iguieron";
        future_first: "~eguiré", "~eguiremos";
        future_second: "~eguirás", "~eguiréis";
        future_third: "~eguirá", "~eguirán";
        conditional_first: "~eguiría", "~eguiríamos";
        conditional_second: "~eguirías", "~eguiríais";
        conditional_third: "~eguiría", "~eguirían";
        present_subjunctive_first: "~iga", "~igamos";
        present_subjunctive_second: "~igas", "~igáis";
        present_subjunctive_third: "~iga", "~igan";
        imperfect_subjunctive_first: "~iguiera", "~iguiéramos";
        imperfect_subjunctive_second: "~iguieras", "~iguierais";
        imperfect_subjunctive_third: "~iguiera", "~iguieran";
        imperfect_subjunctive_se_first: "~iguiese", "~iguiésemos";
        imperfect_subjunctive_se_second: "~iguieses", "~iguieseis";
        imperfect_subjunctive_se_third: "~iguiese", "~iguiesen";
        future_subjunctive_first: "~iguiere", "~iguiéremos";
        future_subjunctive_second: "~iguieres", "~iguiereis";
        future_subjunctive_third: "~iguiere", "~iguieren";
        vos: "~eguís", "~igás", "~eguí";
        imperative: "~igue", "~iga", "~igamos", "~eguid", "~igan";
        negative_imperative: "~igas", "~iga", "~igamos", "~igáis", "~igan";
        infinitive: "~eguir";
        gerund: "~iguiendo";
        participle_masculine: "~eguido", "~eguidos";
        participle_feminine: "~eguida", "~eguidas";
    ),
    // Agorar: 6 words (degollar, agorar, regoldar, engorar, cogolmar, gorar).
    conjugation!("agorar", 6, "~o~ar";
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
    // Apajuilar: 6 words (acuclillar, encuclillar, apajuilar, apulismar, aculillar, encuitar).
    conjugation!("apajuilar", 6, "~i~ar";
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
    // Entredormirse: 6 words (entredormirse, podrirse, adormirse, dormirse, morirse, repodrirse).
    conjugation!("entredormirse", 6, "~o~irse";
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
    // Recogerse: 6 words (encogerse, desencogerse, sobrecogerse, recogerse, cogerse, acogerse).
    conjugation!("recogerse", 6, "~ogerse";
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
    // Caer: 5 words (caer, raer, recaer, decaer, descaer).
    conjugation!("caer", 5, "~er";
        present_first: "~igo", "~emos";
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
        vos: "~és", "~igás", "~é";
        imperative: "~e", "~iga", "~igamos", "~ed", "~igan";
        negative_imperative: "~igas", "~iga", "~igamos", "~igáis", "~igan";
        infinitive: "~er";
        gerund: "~yendo";
        participle_masculine: "~ído", "~ídos";
        participle_feminine: "~ída", "~ídas";
    ),
    // Dormir: 5 words (dormir, adormir, maldormir, podrir, repodrir).
    conjugation!("dormir", 5, "~o~ir";
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
        vos: "~o~ís", "~u~ás", "~o~í";
        imperative: "~ue~e", "~ue~a", "~u~amos", "~o~id", "~ue~an";
        negative_imperative: "~ue~as", "~ue~a", "~u~amos", "~u~áis", "~ue~an";
        infinitive: "~o~ir";
        gerund: "~u~iendo";
        participle_masculine: "~o~ido", "~o~idos";
        participle_feminine: "~o~ida", "~o~idas";
    ),
    // Querer: 5 words (querer, malquerer, bienquerer, desquerer, requerer).
    conjugation!("querer", 5, "~erer";
        present_first: "~iero", "~eremos";
        present_second: "~ieres", "~eréis";
        present_third: "~iere", "~ieren";
        imperfect_first: "~ería", "~eríamos";
        imperfect_second: "~erías", "~eríais";
        imperfect_third: "~ería", "~erían";
        preterite_first: "~ise", "~isimos";
        preterite_second: "~isiste", "~isisteis";
        preterite_third: "~iso", "~isieron";
        future_first: "~erré", "~erremos";
        future_second: "~errás", "~erréis";
        future_third: "~errá", "~errán";
        conditional_first: "~erría", "~erríamos";
        conditional_second: "~errías", "~erríais";
        conditional_third: "~erría", "~errían";
        present_subjunctive_first: "~iera", "~eramos";
        present_subjunctive_second: "~ieras", "~eráis";
        present_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_first: "~isiera", "~isiéramos";
        imperfect_subjunctive_second: "~isieras", "~isierais";
        imperfect_subjunctive_third: "~isiera", "~isieran";
        imperfect_subjunctive_se_first: "~isiese", "~isiésemos";
        imperfect_subjunctive_se_second: "~isieses", "~isieseis";
        imperfect_subjunctive_se_third: "~isiese", "~isiesen";
        future_subjunctive_first: "~isiere", "~isiéremos";
        future_subjunctive_second: "~isieres", "~isiereis";
        future_subjunctive_third: "~isiere", "~isieren";
        vos: "~erés", "~erás", "~eré";
        imperative: "~iere", "~iera", "~eramos", "~ered", "~ieran";
        negative_imperative: "~ieras", "~iera", "~eramos", "~eráis", "~ieran";
        infinitive: "~erer";
        gerund: "~eriendo";
        participle_masculine: "~erido", "~eridos";
        participle_feminine: "~erida", "~eridas";
    ),
    // Salir: 5 words (salir, asir, desasir, sobresalir, resalir).
    conjugation!("salir", 5, "~ir";
        present_first: "~go", "~imos";
        present_second: "~es", "~ís";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~ió", "~ieron";
        future_first: "~dré", "~dremos";
        future_second: "~drás", "~dréis";
        future_third: "~drá", "~drán";
        conditional_first: "~dría", "~dríamos";
        conditional_second: "~drías", "~dríais";
        conditional_third: "~dría", "~drían";
        present_subjunctive_first: "~ga", "~gamos";
        present_subjunctive_second: "~gas", "~gáis";
        present_subjunctive_third: "~ga", "~gan";
        imperfect_subjunctive_first: "~iera", "~iéramos";
        imperfect_subjunctive_second: "~ieras", "~ierais";
        imperfect_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_se_first: "~iese", "~iésemos";
        imperfect_subjunctive_se_second: "~ieses", "~ieseis";
        imperfect_subjunctive_se_third: "~iese", "~iesen";
        future_subjunctive_first: "~iere", "~iéremos";
        future_subjunctive_second: "~ieres", "~iereis";
        future_subjunctive_third: "~iere", "~ieren";
        vos: "~ís", "~gás", "~í";
        imperative: "~", "~ga", "~gamos", "~id", "~gan";
        negative_imperative: "~gas", "~ga", "~gamos", "~gáis", "~gan";
        infinitive: "~ir";
        gerund: "~iendo";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Huir: 5 words (fuir, huir, fluir, fruir, luir).
    conjugation!("huir", 5, "~ir";
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
    // Cocer: 5 words (cocer, escocer, recocer, descocer, precocer).
    conjugation!("cocer", 5, "~ocer";
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
    // Evacuar: 5 words (evacuar, licuar, adecuar, readecuar, promiscuar).
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
    // Delinquir: 5 words (delinquir, derelinquir, jaquir, derrelinquir, relinquir).
    conjugation!("delinquir", 5, "~quir";
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
    // Afligirse: 5 words (autoexigirse, afligirse, arrigirse, dirigirse, erigirse).
    conjugation!("afligirse", 5, "~igirse";
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
    // Reconocerse: 5 words (autoreconocerse, autorreconocerse, conocerse, desconocerse, reconocerse).
    conjugation!("reconocerse", 5, "~o~erse";
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
    // Teñirse: 5 words (teñirse, ceñirse, desceñirse, desteñirse, estreñirse).
    conjugation!("teñirse", 5, "~eñirse";
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
    // Trocarse: 5 words (aclocarse, trocarse, enclocarse, enllocarse, trastrocarse).
    conjugation!("trocarse", 5, "~ocarse";
        present_first: "~ueco", "~ocamos";
        present_second: "~uecas", "~ocáis";
        present_third: "~ueca", "~uecan";
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
        present_subjunctive_first: "~ueque", "~oquemos";
        present_subjunctive_second: "~ueques", "~oquéis";
        present_subjunctive_third: "~ueque", "~uequen";
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
        imperative: "~uécate", "~uéquese", "~oquémonos", "~ocaos", "~uéquense";
        negative_imperative: "~ueques", "~ueque", "~oquemos", "~oquéis", "~uequen";
        infinitive: "~ocarse";
        gerund: "~ocándose";
        participle_masculine: "~ocado", "~ocados";
        participle_feminine: "~ocada", "~ocadas";
    ),
    // Reírse: 5 words (reírse, desleírse, engreírse, sonreírse, freírse).
    conjugation!("reírse", 5, "~eírse";
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
    // Revestirse: 5 words (desvestirse, revestirse, sobrevestirse, travestirse, vestirse).
    conjugation!("revestirse", 5, "~e~tirse";
        present_first: "~i~to", "~e~timos";
        present_second: "~i~tes", "~e~tís";
        present_third: "~i~te", "~i~ten";
        imperfect_first: "~e~tía", "~e~tíamos";
        imperfect_second: "~e~tías", "~e~tíais";
        imperfect_third: "~e~tía", "~e~tían";
        preterite_first: "~e~tí", "~e~timos";
        preterite_second: "~e~tiste", "~e~tisteis";
        preterite_third: "~i~tió", "~i~tieron";
        future_first: "~e~tiré", "~e~tiremos";
        future_second: "~e~tirás", "~e~tiréis";
        future_third: "~e~tirá", "~e~tirán";
        conditional_first: "~e~tiría", "~e~tiríamos";
        conditional_second: "~e~tirías", "~e~tiríais";
        conditional_third: "~e~tiría", "~e~tirían";
        present_subjunctive_first: "~i~ta", "~i~tamos";
        present_subjunctive_second: "~i~tas", "~i~táis";
        present_subjunctive_third: "~i~ta", "~i~tan";
        imperfect_subjunctive_first: "~i~tiera", "~i~tiéramos";
        imperfect_subjunctive_second: "~i~tieras", "~i~tierais";
        imperfect_subjunctive_third: "~i~tiera", "~i~tieran";
        imperfect_subjunctive_se_first: "~i~tiese", "~i~tiésemos";
        imperfect_subjunctive_se_second: "~i~tieses", "~i~tieseis";
        imperfect_subjunctive_se_third: "~i~tiese", "~i~tiesen";
        future_subjunctive_first: "~i~tiere", "~i~tiéremos";
        future_subjunctive_second: "~i~tieres", "~i~tiereis";
        future_subjunctive_third: "~i~tiere", "~i~tieren";
        vos: "~e~tís", "~i~tás", "~e~tite";
        imperative: "~í~tete", "~í~tase", "~i~támonos", "~e~tíos", "~í~tanse";
        negative_imperative: "~i~tas", "~i~ta", "~i~tamos", "~i~táis", "~i~tan";
        infinitive: "~e~tirse";
        gerund: "~i~tiéndose";
        participle_masculine: "~e~tido", "~e~tidos";
        participle_feminine: "~e~tida", "~e~tidas";
    ),
    // Forzar: 4 words (almorzar, forzar, esforzar, reforzar).
    conjugation!("forzar", 4, "~o~zar";
        present_first: "~ue~zo", "~o~zamos";
        present_second: "~ue~zas", "~o~záis";
        present_third: "~ue~za", "~ue~zan";
        imperfect_first: "~o~zaba", "~o~zábamos";
        imperfect_second: "~o~zabas", "~o~zabais";
        imperfect_third: "~o~zaba", "~o~zaban";
        preterite_first: "~o~cé", "~o~zamos";
        preterite_second: "~o~zaste", "~o~zasteis";
        preterite_third: "~o~zó", "~o~zaron";
        future_first: "~o~zaré", "~o~zaremos";
        future_second: "~o~zarás", "~o~zaréis";
        future_third: "~o~zará", "~o~zarán";
        conditional_first: "~o~zaría", "~o~zaríamos";
        conditional_second: "~o~zarías", "~o~zaríais";
        conditional_third: "~o~zaría", "~o~zarían";
        present_subjunctive_first: "~ue~ce", "~o~cemos";
        present_subjunctive_second: "~ue~ces", "~o~céis";
        present_subjunctive_third: "~ue~ce", "~ue~cen";
        imperfect_subjunctive_first: "~o~zara", "~o~záramos";
        imperfect_subjunctive_second: "~o~zaras", "~o~zarais";
        imperfect_subjunctive_third: "~o~zara", "~o~zaran";
        imperfect_subjunctive_se_first: "~o~zase", "~o~zásemos";
        imperfect_subjunctive_se_second: "~o~zases", "~o~zaseis";
        imperfect_subjunctive_se_third: "~o~zase", "~o~zasen";
        future_subjunctive_first: "~o~zare", "~o~záremos";
        future_subjunctive_second: "~o~zares", "~o~zareis";
        future_subjunctive_third: "~o~zare", "~o~zaren";
        vos: "~o~zás", "~o~cés", "~o~zá";
        imperative: "~ue~za", "~ue~ce", "~o~cemos", "~o~zad", "~ue~cen";
        negative_imperative: "~ue~ces", "~ue~ce", "~o~cemos", "~o~céis", "~ue~cen";
        infinitive: "~o~zar";
        gerund: "~o~zando";
        participle_masculine: "~o~zado", "~o~zados";
        participle_feminine: "~o~zada", "~o~zadas";
    ),
    // Adquirir: 4 words (adquirir, inquirir, perquirir, readquirir).
    conjugation!("adquirir", 4, "~~ir";
        present_first: "~e~o", "~~imos";
        present_second: "~e~es", "~~ís";
        present_third: "~e~e", "~e~en";
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
        present_subjunctive_first: "~e~a", "~~amos";
        present_subjunctive_second: "~e~as", "~~áis";
        present_subjunctive_third: "~e~a", "~e~an";
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
        imperative: "~e~e", "~e~a", "~~amos", "~~id", "~e~an";
        negative_imperative: "~e~as", "~e~a", "~~amos", "~~áis", "~e~an";
        infinitive: "~~ir";
        gerund: "~~iendo";
        participle_masculine: "~~ido", "~~idos";
        participle_feminine: "~~ida", "~~idas";
    ),
    // Cohibir: 4 words (prohibir, cohibir, rehinchir, desprohibir).
    conjugation!("cohibir", 4, "~i~ir";
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
    // Morir: 4 words (morir, premorir, desmorir, entremorir).
    conjugation!("morir", 4, "~o~ir";
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
        vos: "~o~ís", "~u~ás", "~o~í";
        imperative: "~ue~e", "~ue~a", "~u~amos", "~o~id", "~ue~an";
        negative_imperative: "~ue~as", "~ue~a", "~u~amos", "~u~áis", "~ue~an";
        infinitive: "~o~ir";
        gerund: "~u~iendo";
        participle_masculine: "~ue~to", "~ue~tos";
        participle_feminine: "~ue~ta", "~ue~tas";
    ),
    // Distinguir: 4 words (distinguir, extinguir, subdistinguir, desextinguir).
    conjugation!("distinguir", 4, "~uir";
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
    // Torcer: 4 words (torcer, retorcer, destorcer, contorcer).
    conjugation!("torcer", 4, "~o~cer";
        present_first: "~ue~zo", "~o~cemos";
        present_second: "~ue~ces", "~o~céis";
        present_third: "~ue~ce", "~ue~cen";
        imperfect_first: "~o~cía", "~o~cíamos";
        imperfect_second: "~o~cías", "~o~cíais";
        imperfect_third: "~o~cía", "~o~cían";
        preterite_first: "~o~cí", "~o~cimos";
        preterite_second: "~o~ciste", "~o~cisteis";
        preterite_third: "~o~ció", "~o~cieron";
        future_first: "~o~ceré", "~o~ceremos";
        future_second: "~o~cerás", "~o~ceréis";
        future_third: "~o~cerá", "~o~cerán";
        conditional_first: "~o~cería", "~o~ceríamos";
        conditional_second: "~o~cerías", "~o~ceríais";
        conditional_third: "~o~cería", "~o~cerían";
        present_subjunctive_first: "~ue~za", "~o~zamos";
        present_subjunctive_second: "~ue~zas", "~o~záis";
        present_subjunctive_third: "~ue~za", "~ue~zan";
        imperfect_subjunctive_first: "~o~ciera", "~o~ciéramos";
        imperfect_subjunctive_second: "~o~cieras", "~o~cierais";
        imperfect_subjunctive_third: "~o~ciera", "~o~cieran";
        imperfect_subjunctive_se_first: "~o~ciese", "~o~ciésemos";
        imperfect_subjunctive_se_second: "~o~cieses", "~o~cieseis";
        imperfect_subjunctive_se_third: "~o~ciese", "~o~ciesen";
        future_subjunctive_first: "~o~ciere", "~o~ciéremos";
        future_subjunctive_second: "~o~cieres", "~o~ciereis";
        future_subjunctive_third: "~o~ciere", "~o~cieren";
        vos: "~o~cés", "~o~zás", "~o~cé";
        imperative: "~ue~ce", "~ue~za", "~o~zamos", "~o~ced", "~ue~zan";
        negative_imperative: "~ue~zas", "~ue~za", "~o~zamos", "~o~záis", "~ue~zan";
        infinitive: "~o~cer";
        gerund: "~o~ciendo";
        participle_masculine: "~o~cido", "~o~cidos";
        participle_feminine: "~o~cida", "~o~cidas";
    ),
    // Repetir: 4 words (repetir, derretir, competir, redetir).
    conjugation!("repetir", 4, "~etir";
        present_first: "~ito", "~etimos";
        present_second: "~ites", "~etís";
        present_third: "~ite", "~iten";
        imperfect_first: "~etía", "~etíamos";
        imperfect_second: "~etías", "~etíais";
        imperfect_third: "~etía", "~etían";
        preterite_first: "~etí", "~etimos";
        preterite_second: "~etiste", "~etisteis";
        preterite_third: "~itió", "~itieron";
        future_first: "~etiré", "~etiremos";
        future_second: "~etirás", "~etiréis";
        future_third: "~etirá", "~etirán";
        conditional_first: "~etiría", "~etiríamos";
        conditional_second: "~etirías", "~etiríais";
        conditional_third: "~etiría", "~etirían";
        present_subjunctive_first: "~ita", "~itamos";
        present_subjunctive_second: "~itas", "~itáis";
        present_subjunctive_third: "~ita", "~itan";
        imperfect_subjunctive_first: "~itiera", "~itiéramos";
        imperfect_subjunctive_second: "~itieras", "~itierais";
        imperfect_subjunctive_third: "~itiera", "~itieran";
        imperfect_subjunctive_se_first: "~itiese", "~itiésemos";
        imperfect_subjunctive_se_second: "~itieses", "~itieseis";
        imperfect_subjunctive_se_third: "~itiese", "~itiesen";
        future_subjunctive_first: "~itiere", "~itiéremos";
        future_subjunctive_second: "~itieres", "~itiereis";
        future_subjunctive_third: "~itiere", "~itieren";
        vos: "~etís", "~itás", "~etí";
        imperative: "~ite", "~ita", "~itamos", "~etid", "~itan";
        negative_imperative: "~itas", "~ita", "~itamos", "~itáis", "~itan";
        infinitive: "~etir";
        gerund: "~itiendo";
        participle_masculine: "~etido", "~etidos";
        participle_feminine: "~etida", "~etidas";
    ),
    // Concebir: 4 words (concebir, decebir, preconcebir, recebir).
    conjugation!("concebir", 4, "~ebir";
        present_first: "~ibo", "~ebimos";
        present_second: "~ibes", "~ebís";
        present_third: "~ibe", "~iben";
        imperfect_first: "~ebía", "~ebíamos";
        imperfect_second: "~ebías", "~ebíais";
        imperfect_third: "~ebía", "~ebían";
        preterite_first: "~ebí", "~ebimos";
        preterite_second: "~ebiste", "~ebisteis";
        preterite_third: "~ibió", "~ibieron";
        future_first: "~ebiré", "~ebiremos";
        future_second: "~ebirás", "~ebiréis";
        future_third: "~ebirá", "~ebirán";
        conditional_first: "~ebiría", "~ebiríamos";
        conditional_second: "~ebirías", "~ebiríais";
        conditional_third: "~ebiría", "~ebirían";
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
        vos: "~ebís", "~ibás", "~ebí";
        imperative: "~ibe", "~iba", "~ibamos", "~ebid", "~iban";
        negative_imperative: "~ibas", "~iba", "~ibamos", "~ibáis", "~iban";
        infinitive: "~ebir";
        gerund: "~ibiendo";
        participle_masculine: "~ebido", "~ebidos";
        participle_feminine: "~ebida", "~ebidas";
    ),
    // Desforzarse: 4 words (desforzarse, forzarse, esforzarse, reforzarse).
    conjugation!("desforzarse", 4, "~o~zarse";
        present_first: "~ue~zo", "~o~zamos";
        present_second: "~ue~zas", "~o~záis";
        present_third: "~ue~za", "~ue~zan";
        imperfect_first: "~o~zaba", "~o~zábamos";
        imperfect_second: "~o~zabas", "~o~zabais";
        imperfect_third: "~o~zaba", "~o~zaban";
        preterite_first: "~o~cé", "~o~zamos";
        preterite_second: "~o~zaste", "~o~zasteis";
        preterite_third: "~o~zó", "~o~zaron";
        future_first: "~o~zaré", "~o~zaremos";
        future_second: "~o~zarás", "~o~zaréis";
        future_third: "~o~zará", "~o~zarán";
        conditional_first: "~o~zaría", "~o~zaríamos";
        conditional_second: "~o~zarías", "~o~zaríais";
        conditional_third: "~o~zaría", "~o~zarían";
        present_subjunctive_first: "~ue~ce", "~o~cemos";
        present_subjunctive_second: "~ue~ces", "~o~céis";
        present_subjunctive_third: "~ue~ce", "~ue~cen";
        imperfect_subjunctive_first: "~o~zara", "~o~záramos";
        imperfect_subjunctive_second: "~o~zaras", "~o~zarais";
        imperfect_subjunctive_third: "~o~zara", "~o~zaran";
        imperfect_subjunctive_se_first: "~o~zase", "~o~zásemos";
        imperfect_subjunctive_se_second: "~o~zases", "~o~zaseis";
        imperfect_subjunctive_se_third: "~o~zase", "~o~zasen";
        future_subjunctive_first: "~o~zare", "~o~záremos";
        future_subjunctive_second: "~o~zares", "~o~zareis";
        future_subjunctive_third: "~o~zare", "~o~zaren";
        vos: "~o~zás", "~o~cés", "~o~zate";
        imperative: "~ué~zate", "~ué~cese", "~o~cémonos", "~o~zaos", "~ué~cense";
        negative_imperative: "~ue~ces", "~ue~ce", "~o~cemos", "~o~céis", "~ue~cen";
        infinitive: "~o~zarse";
        gerund: "~o~zándose";
        participle_masculine: "~o~zado", "~o~zados";
        participle_feminine: "~o~zada", "~o~zadas";
    ),
    // Regirse: 4 words (autocorregirse, autoelegirse, corregirse, regirse).
    conjugation!("regirse", 4, "~egirse";
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
    // Fraguarse: 4 words (paniaguarse, sobreaguarse, desaguarse, fraguarse).
    conjugation!("fraguarse", 4, "~a~uarse";
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
    // Moverse: 4 words (lloverse, conmoverse, removerse, moverse).
    conjugation!("moverse", 4, "~overse";
        present_first: "~uevo", "~ovemos";
        present_second: "~ueves", "~ovéis";
        present_third: "~ueve", "~ueven";
        imperfect_first: "~ovía", "~ovíamos";
        imperfect_second: "~ovías", "~ovíais";
        imperfect_third: "~ovía", "~ovían";
        preterite_first: "~oví", "~ovimos";
        preterite_second: "~oviste", "~ovisteis";
        preterite_third: "~ovió", "~ovieron";
        future_first: "~overé", "~overemos";
        future_second: "~overás", "~overéis";
        future_third: "~overá", "~overán";
        conditional_first: "~overía", "~overíamos";
        conditional_second: "~overías", "~overíais";
        conditional_third: "~overía", "~overían";
        present_subjunctive_first: "~ueva", "~ovamos";
        present_subjunctive_second: "~uevas", "~ováis";
        present_subjunctive_third: "~ueva", "~uevan";
        imperfect_subjunctive_first: "~oviera", "~oviéramos";
        imperfect_subjunctive_second: "~ovieras", "~ovierais";
        imperfect_subjunctive_third: "~oviera", "~ovieran";
        imperfect_subjunctive_se_first: "~oviese", "~oviésemos";
        imperfect_subjunctive_se_second: "~ovieses", "~ovieseis";
        imperfect_subjunctive_se_third: "~oviese", "~oviesen";
        future_subjunctive_first: "~oviere", "~oviéremos";
        future_subjunctive_second: "~ovieres", "~oviereis";
        future_subjunctive_third: "~oviere", "~ovieren";
        vos: "~ovés", "~ovás", "~ovete";
        imperative: "~uévete", "~uévase", "~ovámonos", "~oveos", "~uévanse";
        negative_imperative: "~uevas", "~ueva", "~ovamos", "~ováis", "~uevan";
        infinitive: "~overse";
        gerund: "~oviéndose";
        participle_masculine: "~ovido", "~ovidos";
        participle_feminine: "~ovida", "~ovidas";
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
    // Punzarse: 4 words (anarquizarse, barroquizarse, endulzarse, punzarse).
    conjugation!("punzarse", 4, "~u~zarse";
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
    // Traslucirse: 4 words (traslucirse, translucirse, deslucirse, lucirse).
    conjugation!("traslucirse", 4, "~u~irse";
        present_first: "~uz~o", "~u~imos";
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
        present_subjunctive_first: "~uz~a", "~uz~amos";
        present_subjunctive_second: "~uz~as", "~uz~áis";
        present_subjunctive_third: "~uz~a", "~uz~an";
        imperfect_subjunctive_first: "~u~iera", "~u~iéramos";
        imperfect_subjunctive_second: "~u~ieras", "~u~ierais";
        imperfect_subjunctive_third: "~u~iera", "~u~ieran";
        imperfect_subjunctive_se_first: "~u~iese", "~u~iésemos";
        imperfect_subjunctive_se_second: "~u~ieses", "~u~ieseis";
        imperfect_subjunctive_se_third: "~u~iese", "~u~iesen";
        future_subjunctive_first: "~u~iere", "~u~iéremos";
        future_subjunctive_second: "~u~ieres", "~u~iereis";
        future_subjunctive_third: "~u~iere", "~u~ieren";
        vos: "~u~ís", "~uz~ás", "~u~ite";
        imperative: "~ú~ete", "~úz~ase", "~uz~ámonos", "~u~íos", "~úz~anse";
        negative_imperative: "~uz~as", "~uz~a", "~uz~amos", "~uz~áis", "~uz~an";
        infinitive: "~u~irse";
        gerund: "~u~iéndose";
        participle_masculine: "~u~ido", "~u~idos";
        participle_feminine: "~u~ida", "~u~idas";
    ),
    // Torcerse: 4 words (contorcerse, retorcerse, torcerse, destorcerse).
    conjugation!("torcerse", 4, "~o~cerse";
        present_first: "~ue~zo", "~o~cemos";
        present_second: "~ue~ces", "~o~céis";
        present_third: "~ue~ce", "~ue~cen";
        imperfect_first: "~o~cía", "~o~cíamos";
        imperfect_second: "~o~cías", "~o~cíais";
        imperfect_third: "~o~cía", "~o~cían";
        preterite_first: "~o~cí", "~o~cimos";
        preterite_second: "~o~ciste", "~o~cisteis";
        preterite_third: "~o~ció", "~o~cieron";
        future_first: "~o~ceré", "~o~ceremos";
        future_second: "~o~cerás", "~o~ceréis";
        future_third: "~o~cerá", "~o~cerán";
        conditional_first: "~o~cería", "~o~ceríamos";
        conditional_second: "~o~cerías", "~o~ceríais";
        conditional_third: "~o~cería", "~o~cerían";
        present_subjunctive_first: "~ue~za", "~o~zamos";
        present_subjunctive_second: "~ue~zas", "~o~záis";
        present_subjunctive_third: "~ue~za", "~ue~zan";
        imperfect_subjunctive_first: "~o~ciera", "~o~ciéramos";
        imperfect_subjunctive_second: "~o~cieras", "~o~cierais";
        imperfect_subjunctive_third: "~o~ciera", "~o~cieran";
        imperfect_subjunctive_se_first: "~o~ciese", "~o~ciésemos";
        imperfect_subjunctive_se_second: "~o~cieses", "~o~cieseis";
        imperfect_subjunctive_se_third: "~o~ciese", "~o~ciesen";
        future_subjunctive_first: "~o~ciere", "~o~ciéremos";
        future_subjunctive_second: "~o~cieres", "~o~ciereis";
        future_subjunctive_third: "~o~ciere", "~o~cieren";
        vos: "~o~cés", "~o~zás", "~o~cete";
        imperative: "~ué~cete", "~ué~zase", "~o~zámonos", "~o~ceos", "~ué~zanse";
        negative_imperative: "~ue~zas", "~ue~za", "~o~zamos", "~o~záis", "~ue~zan";
        infinitive: "~o~cerse";
        gerund: "~o~ciéndose";
        participle_masculine: "~o~cido", "~o~cidos";
        participle_feminine: "~o~cida", "~o~cidas";
    ),
    // Licuarse: 4 words (anticuarse, colicuarse, licuarse, apropincuarse).
    conjugation!("licuarse", 4, "~i~uarse";
        present_first: "~i~úo|~i~uo", "~i~uamos";
        present_second: "~i~úas|~i~uas", "~i~uáis";
        present_third: "~i~úa|~i~ua", "~i~úan|~i~uan";
        imperfect_first: "~i~uaba", "~i~uábamos";
        imperfect_second: "~i~uabas", "~i~uabais";
        imperfect_third: "~i~uaba", "~i~uaban";
        preterite_first: "~i~ué", "~i~uamos";
        preterite_second: "~i~uaste", "~i~uasteis";
        preterite_third: "~i~uó", "~i~uaron";
        future_first: "~i~uaré", "~i~uaremos";
        future_second: "~i~uarás", "~i~uaréis";
        future_third: "~i~uará", "~i~uarán";
        conditional_first: "~i~uaría", "~i~uaríamos";
        conditional_second: "~i~uarías", "~i~uaríais";
        conditional_third: "~i~uaría", "~i~uarían";
        present_subjunctive_first: "~i~úe|~i~ue", "~i~uemos";
        present_subjunctive_second: "~i~úes|~i~ues", "~i~uéis";
        present_subjunctive_third: "~i~úe|~i~ue", "~i~úen|~i~uen";
        imperfect_subjunctive_first: "~i~uara", "~i~uáramos";
        imperfect_subjunctive_second: "~i~uaras", "~i~uarais";
        imperfect_subjunctive_third: "~i~uara", "~i~uaran";
        imperfect_subjunctive_se_first: "~i~uase", "~i~uásemos";
        imperfect_subjunctive_se_second: "~i~uases", "~i~uaseis";
        imperfect_subjunctive_se_third: "~i~uase", "~i~uasen";
        future_subjunctive_first: "~i~uare", "~i~uáremos";
        future_subjunctive_second: "~i~uares", "~i~uareis";
        future_subjunctive_third: "~i~uare", "~i~uaren";
        vos: "~i~uás", "~i~ués", "~i~uate";
        imperative: "~i~úate|~í~uate", "~i~úese|~í~uese", "~i~uémonos", "~i~uaos", "~i~úense|~í~uense";
        negative_imperative: "~i~úes|~i~ues", "~i~úe|~i~ue", "~i~uemos", "~i~uéis", "~i~úen|~i~uen";
        infinitive: "~i~uarse";
        gerund: "~i~uándose";
        participle_masculine: "~i~uado", "~i~uados";
        participle_feminine: "~i~uada", "~i~uadas";
    ),
    // Vencerse: 4 words (remecerse, convencerse, vencerse, desconvencerse).
    conjugation!("vencerse", 4, "~e~cerse";
        present_first: "~e~zo", "~e~cemos";
        present_second: "~e~ces", "~e~céis";
        present_third: "~e~ce", "~e~cen";
        imperfect_first: "~e~cía", "~e~cíamos";
        imperfect_second: "~e~cías", "~e~cíais";
        imperfect_third: "~e~cía", "~e~cían";
        preterite_first: "~e~cí", "~e~cimos";
        preterite_second: "~e~ciste", "~e~cisteis";
        preterite_third: "~e~ció", "~e~cieron";
        future_first: "~e~ceré", "~e~ceremos";
        future_second: "~e~cerás", "~e~ceréis";
        future_third: "~e~cerá", "~e~cerán";
        conditional_first: "~e~cería", "~e~ceríamos";
        conditional_second: "~e~cerías", "~e~ceríais";
        conditional_third: "~e~cería", "~e~cerían";
        present_subjunctive_first: "~e~za", "~e~zamos";
        present_subjunctive_second: "~e~zas", "~e~záis";
        present_subjunctive_third: "~e~za", "~e~zan";
        imperfect_subjunctive_first: "~e~ciera", "~e~ciéramos";
        imperfect_subjunctive_second: "~e~cieras", "~e~cierais";
        imperfect_subjunctive_third: "~e~ciera", "~e~cieran";
        imperfect_subjunctive_se_first: "~e~ciese", "~e~ciésemos";
        imperfect_subjunctive_se_second: "~e~cieses", "~e~cieseis";
        imperfect_subjunctive_se_third: "~e~ciese", "~e~ciesen";
        future_subjunctive_first: "~e~ciere", "~e~ciéremos";
        future_subjunctive_second: "~e~cieres", "~e~ciereis";
        future_subjunctive_third: "~e~ciere", "~e~cieren";
        vos: "~e~cés", "~e~zás", "~e~cete";
        imperative: "~é~cete", "~é~zase", "~e~zámonos", "~e~ceos", "~é~zanse";
        negative_imperative: "~e~zas", "~e~za", "~e~zamos", "~e~záis", "~e~zan";
        infinitive: "~e~cerse";
        gerund: "~e~ciéndose";
        participle_masculine: "~e~cido", "~e~cidos";
        participle_feminine: "~e~cida", "~e~cidas";
    ),
    // Sonarse: 4 words (atronarse, asonarse, sonarse, tronarse).
    conjugation!("sonarse", 4, "~onarse";
        present_first: "~ueno", "~onamos";
        present_second: "~uenas", "~onáis";
        present_third: "~uena", "~uenan";
        imperfect_first: "~onaba", "~onábamos";
        imperfect_second: "~onabas", "~onabais";
        imperfect_third: "~onaba", "~onaban";
        preterite_first: "~oné", "~onamos";
        preterite_second: "~onaste", "~onasteis";
        preterite_third: "~onó", "~onaron";
        future_first: "~onaré", "~onaremos";
        future_second: "~onarás", "~onaréis";
        future_third: "~onará", "~onarán";
        conditional_first: "~onaría", "~onaríamos";
        conditional_second: "~onarías", "~onaríais";
        conditional_third: "~onaría", "~onarían";
        present_subjunctive_first: "~uene", "~onemos";
        present_subjunctive_second: "~uenes", "~onéis";
        present_subjunctive_third: "~uene", "~uenen";
        imperfect_subjunctive_first: "~onara", "~onáramos";
        imperfect_subjunctive_second: "~onaras", "~onarais";
        imperfect_subjunctive_third: "~onara", "~onaran";
        imperfect_subjunctive_se_first: "~onase", "~onásemos";
        imperfect_subjunctive_se_second: "~onases", "~onaseis";
        imperfect_subjunctive_se_third: "~onase", "~onasen";
        future_subjunctive_first: "~onare", "~onáremos";
        future_subjunctive_second: "~onares", "~onareis";
        future_subjunctive_third: "~onare", "~onaren";
        vos: "~onás", "~onés", "~onate";
        imperative: "~uénate", "~uénese", "~onémonos", "~onaos", "~uénense";
        negative_imperative: "~uenes", "~uene", "~onemos", "~onéis", "~uenen";
        infinitive: "~onarse";
        gerund: "~onándose";
        participle_masculine: "~onado", "~onados";
        participle_feminine: "~onada", "~onadas";
    ),
    // Yacer: 3 words (yacer, subyacer, adyacer).
    conjugation!("yacer", 3, "~cer";
        present_first: "~zco|~zgo|~go", "~cemos";
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
        present_subjunctive_first: "~zca|~zga|~ga", "~zcamos|~zgamos|~gamos";
        present_subjunctive_second: "~zcas|~zgas|~gas|~zcás|~zgás", "~zcáis|~zgáis|~gáis";
        present_subjunctive_third: "~zca|~zga|~ga", "~zcan|~zgan|~gan";
        imperfect_subjunctive_first: "~ciera", "~ciéramos";
        imperfect_subjunctive_second: "~cieras", "~cierais";
        imperfect_subjunctive_third: "~ciera", "~cieran";
        imperfect_subjunctive_se_first: "~ciese", "~ciésemos";
        imperfect_subjunctive_se_second: "~cieses", "~cieseis";
        imperfect_subjunctive_se_third: "~ciese", "~ciesen";
        future_subjunctive_first: "~ciere", "~ciéremos";
        future_subjunctive_second: "~cieres", "~ciereis";
        future_subjunctive_third: "~ciere", "~cieren";
        vos: "~cés", "~gás", "~cé";
        imperative: "~ce|~z", "~zca|~zga|~ga", "~zcamos|~zgamos|~gamos", "~ced", "~zcan|~zgan|~gan";
        negative_imperative: "~zcas|~zgas|~gas", "~zca|~zga|~ga", "~zcamos|~zgamos|~gamos", "~zcáis|~zgáis|~gáis", "~zcan|~zgan|~gan";
        infinitive: "~cer";
        gerund: "~ciendo";
        participle_masculine: "~cido", "~cidos";
        participle_feminine: "~cida", "~cidas";
    ),
    // Argüir: 3 words (argüir, reargüir, redargüir).
    conjugation!("argüir", 3, "~üir";
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
    // Jugar: 3 words (jugar, rejugar, sobrejugar).
    conjugation!("jugar", 3, "~~ar";
        present_first: "~e~o", "~~amos";
        present_second: "~e~as", "~~áis";
        present_third: "~e~a", "~e~an";
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
        present_subjunctive_first: "~e~ue", "~~uemos";
        present_subjunctive_second: "~e~ues", "~~uéis";
        present_subjunctive_third: "~e~ue", "~e~uen";
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
        imperative: "~e~a", "~e~ue", "~~uemos", "~~ad", "~e~uen";
        negative_imperative: "~e~ues", "~e~ue", "~~uemos", "~~uéis", "~e~uen";
        infinitive: "~~ar";
        gerund: "~~ando";
        participle_masculine: "~~ado", "~~ados";
        participle_feminine: "~~ada", "~~adas";
    ),
    // Romper: 3 words (romper, derromper, arromper).
    conjugation!("romper", 3, "~mper";
        present_first: "~mpo", "~mpemos";
        present_second: "~mpes", "~mpéis";
        present_third: "~mpe", "~mpen";
        imperfect_first: "~mpía", "~mpíamos";
        imperfect_second: "~mpías", "~mpíais";
        imperfect_third: "~mpía", "~mpían";
        preterite_first: "~mpí", "~mpimos";
        preterite_second: "~mpiste", "~mpisteis";
        preterite_third: "~mpió", "~mpieron";
        future_first: "~mperé", "~mperemos";
        future_second: "~mperás", "~mperéis";
        future_third: "~mperá", "~mperán";
        conditional_first: "~mpería", "~mperíamos";
        conditional_second: "~mperías", "~mperíais";
        conditional_third: "~mpería", "~mperían";
        present_subjunctive_first: "~mpa", "~mpamos";
        present_subjunctive_second: "~mpas", "~mpáis";
        present_subjunctive_third: "~mpa", "~mpan";
        imperfect_subjunctive_first: "~mpiera", "~mpiéramos";
        imperfect_subjunctive_second: "~mpieras", "~mpierais";
        imperfect_subjunctive_third: "~mpiera", "~mpieran";
        imperfect_subjunctive_se_first: "~mpiese", "~mpiésemos";
        imperfect_subjunctive_se_second: "~mpieses", "~mpieseis";
        imperfect_subjunctive_se_third: "~mpiese", "~mpiesen";
        future_subjunctive_first: "~mpiere", "~mpiéremos";
        future_subjunctive_second: "~mpieres", "~mpiereis";
        future_subjunctive_third: "~mpiere", "~mpieren";
        vos: "~mpés", "~mpás", "~mpé";
        imperative: "~mpe", "~mpa", "~mpamos", "~mped", "~mpan";
        negative_imperative: "~mpas", "~mpa", "~mpamos", "~mpáis", "~mpan";
        infinitive: "~mper";
        gerund: "~mpiendo";
        participle_masculine: "~to", "~tos";
        participle_feminine: "~ta", "~tas";
    ),
    // Valer: 3 words (valer, equivaler, prevaler).
    conjugation!("valer", 3, "~er";
        present_first: "~go", "~emos";
        present_second: "~es", "~éis";
        present_third: "~e", "~en";
        imperfect_first: "~ía", "~íamos";
        imperfect_second: "~ías", "~íais";
        imperfect_third: "~ía", "~ían";
        preterite_first: "~í", "~imos";
        preterite_second: "~iste", "~isteis";
        preterite_third: "~ió", "~ieron";
        future_first: "~dré", "~dremos";
        future_second: "~drás", "~dréis";
        future_third: "~drá", "~drán";
        conditional_first: "~dría", "~dríamos";
        conditional_second: "~drías", "~dríais";
        conditional_third: "~dría", "~drían";
        present_subjunctive_first: "~ga", "~gamos";
        present_subjunctive_second: "~gas", "~gáis";
        present_subjunctive_third: "~ga", "~gan";
        imperfect_subjunctive_first: "~iera", "~iéramos";
        imperfect_subjunctive_second: "~ieras", "~ierais";
        imperfect_subjunctive_third: "~iera", "~ieran";
        imperfect_subjunctive_se_first: "~iese", "~iésemos";
        imperfect_subjunctive_se_second: "~ieses", "~ieseis";
        imperfect_subjunctive_se_third: "~iese", "~iesen";
        future_subjunctive_first: "~iere", "~iéremos";
        future_subjunctive_second: "~ieres", "~iereis";
        future_subjunctive_third: "~iere", "~ieren";
        vos: "~és", "~gás", "~é";
        imperative: "~e", "~ga", "~gamos", "~ed", "~gan";
        negative_imperative: "~gas", "~ga", "~gamos", "~gáis", "~gan";
        infinitive: "~er";
        gerund: "~iendo";
        participle_masculine: "~ido", "~idos";
        participle_feminine: "~ida", "~idas";
    ),
    // Imprimir: 3 words (imprimir, reimprimir, sobreimprimir).
    conjugation!("imprimir", 3, "~imir";
        present_first: "~imo", "~imimos";
        present_second: "~imes", "~imís";
        present_third: "~ime", "~imen";
        imperfect_first: "~imía", "~imíamos";
        imperfect_second: "~imías", "~imíais";
        imperfect_third: "~imía", "~imían";
        preterite_first: "~imí", "~imimos";
        preterite_second: "~imiste", "~imisteis";
        preterite_third: "~imió", "~imieron";
        future_first: "~imiré", "~imiremos";
        future_second: "~imirás", "~imiréis";
        future_third: "~imirá", "~imirán";
        conditional_first: "~imiría", "~imiríamos";
        conditional_second: "~imirías", "~imiríais";
        conditional_third: "~imiría", "~imirían";
        present_subjunctive_first: "~ima", "~imamos";
        present_subjunctive_second: "~imas", "~imáis";
        present_subjunctive_third: "~ima", "~iman";
        imperfect_subjunctive_first: "~imiera", "~imiéramos";
        imperfect_subjunctive_second: "~imieras", "~imierais";
        imperfect_subjunctive_third: "~imiera", "~imieran";
        imperfect_subjunctive_se_first: "~imiese", "~imiésemos";
        imperfect_subjunctive_se_second: "~imieses", "~imieseis";
        imperfect_subjunctive_se_third: "~imiese", "~imiesen";
        future_subjunctive_first: "~imiere", "~imiéremos";
        future_subjunctive_second: "~imieres", "~imiereis";
        future_subjunctive_third: "~imiere", "~imieren";
        vos: "~imís", "~imás", "~imí";
        imperative: "~ime", "~ima", "~imamos", "~imid", "~iman";
        negative_imperative: "~imas", "~ima", "~imamos", "~imáis", "~iman";
        infinitive: "~imir";
        gerund: "~imiendo";
        participle_masculine: "~imido|~eso", "~imidos|~esos";
        participle_feminine: "~imida|~esa", "~imidas|~esas";
    ),
    // Cernir: 3 words (discernir, cernir, hendir).
    conjugation!("cernir", 3, "~~ir";
        present_first: "~i~o", "~~imos";
        present_second: "~i~es", "~~ís";
        present_third: "~i~e", "~i~en";
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
        vos: "~~ís", "~~ás", "~~í";
        imperative: "~i~e", "~i~a", "~~amos", "~~id", "~i~an";
        negative_imperative: "~i~as", "~i~a", "~~amos", "~~áis", "~i~an";
        infinitive: "~~ir";
        gerund: "~~iendo";
        participle_masculine: "~~ido", "~~idos";
        participle_feminine: "~~ida", "~~idas";
    ),
    // Volcar: 3 words (revolcar, volcar, emporcar).
    conjugation!("volcar", 3, "~o~car";
        present_first: "~ue~co", "~o~camos";
        present_second: "~ue~cas", "~o~cáis";
        present_third: "~ue~ca", "~ue~can";
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
        present_subjunctive_first: "~ue~que", "~o~quemos";
        present_subjunctive_second: "~ue~ques", "~o~quéis";
        present_subjunctive_third: "~ue~que", "~ue~quen";
        imperfect_subjunctive_first: "~o~cara", "~o~cáramos";
        imperfect_subjunctive_second: "~o~caras", "~o~carais";
        imperfect_subjunctive_third: "~o~cara", "~o~caran";
        imperfect_subjunctive_se_first: "~o~case", "~o~cásemos";
        imperfect_subjunctive_se_second: "~o~cases", "~o~caseis";
        imperfect_subjunctive_se_third: "~o~case", "~o~casen";
        future_subjunctive_first: "~o~care", "~o~cáremos";
        future_subjunctive_second: "~o~cares", "~o~careis";
        future_subjunctive_third: "~o~care", "~o~caren";
        vos: "~o~cás", "~o~qués", "~o~cá";
        imperative: "~ue~ca", "~ue~que", "~o~quemos", "~o~cad", "~ue~quen";
        negative_imperative: "~ue~ques", "~ue~que", "~o~quemos", "~o~quéis", "~ue~quen";
        infinitive: "~o~car";
        gerund: "~o~cando";
        participle_masculine: "~o~cado", "~o~cados";
        participle_feminine: "~o~cada", "~o~cadas";
    ),
    // Apucuyar: 3 words (intrusar, apucuyar, empuyar).
    conjugation!("apucuyar", 3, "~u~ar";
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
    // Antojar: 3 words (antojar, hopar, ñangotar).
    conjugation!("antojar", 3, "~o~ar";
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
    // Escocerse: 3 words (escocerse, recocerse, cocerse).
    conjugation!("escocerse", 3, "~ocerse";
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
        vos: "~océs", "~ozás", "~ocete";
        imperative: "~uécete", "~uézase", "~ozámonos", "~oceos", "~uézanse";
        negative_imperative: "~uezas", "~ueza", "~ozamos", "~ozáis", "~uezan";
        infinitive: "~ocerse";
        gerund: "~ociéndose";
        participle_masculine: "~ocido|~ocho", "~ocidos|~ochos";
        participle_feminine: "~ocida|~ocha", "~ocidas|~ochas";
    ),
    // Seguirse: 3 words (conseguirse, subseguirse, seguirse).
    conjugation!("seguirse", 3, "~eguirse";
        present_first: "~igo", "~eguimos";
        present_second: "~igues", "~eguís";
        present_third: "~igue", "~iguen";
        imperfect_first: "~eguía", "~eguíamos";
        imperfect_second: "~eguías", "~eguíais";
        imperfect_third: "~eguía", "~eguían";
        preterite_first: "~eguí", "~eguimos";
        preterite_second: "~eguiste", "~eguisteis";
        preterite_third: "~iguió", "~iguieron";
        future_first: "~eguiré", "~eguiremos";
        future_second: "~eguirás", "~eguiréis";
        future_third: "~eguirá", "~eguirán";
        conditional_first: "~eguiría", "~eguiríamos";
        conditional_second: "~eguirías", "~eguiríais";
        conditional_third: "~eguiría", "~eguirían";
        present_subjunctive_first: "~iga", "~igamos";
        present_subjunctive_second: "~igas", "~igáis";
        present_subjunctive_third: "~iga", "~igan";
        imperfect_subjunctive_first: "~iguiera", "~iguiéramos";
        imperfect_subjunctive_second: "~iguieras", "~iguierais";
        imperfect_subjunctive_third: "~iguiera", "~iguieran";
        imperfect_subjunctive_se_first: "~iguiese", "~iguiésemos";
        imperfect_subjunctive_se_second: "~iguieses", "~iguieseis";
        imperfect_subjunctive_se_third: "~iguiese", "~iguiesen";
        future_subjunctive_first: "~iguiere", "~iguiéremos";
        future_subjunctive_second: "~iguieres", "~iguiereis";
        future_subjunctive_third: "~iguiere", "~iguieren";
        vos: "~eguís", "~igás", "~eguite";
        imperative: "~íguete", "~ígase", "~igámonos", "~eguíos", "~íganse";
        negative_imperative: "~igas", "~iga", "~igamos", "~igáis", "~igan";
        infinitive: "~eguirse";
        gerund: "~iguiéndose";
        participle_masculine: "~eguido", "~eguidos";
        participle_feminine: "~eguida", "~eguidas";
    ),
    // Azoarse: 3 words (abarloarse, azoarse, barloarse).
    conjugation!("azoarse", 3, "~oarse";
        present_first: "~oo", "~oamos";
        present_second: "~oas", "~oáis";
        present_third: "~oa", "~oan";
        imperfect_first: "~oaba", "~oábamos";
        imperfect_second: "~oabas", "~oabais";
        imperfect_third: "~oaba", "~oaban";
        preterite_first: "~oé", "~oamos";
        preterite_second: "~oaste", "~oasteis";
        preterite_third: "~oó", "~oaron";
        future_first: "~oaré", "~oaremos";
        future_second: "~oarás", "~oaréis";
        future_third: "~oará", "~oarán";
        conditional_first: "~oaría", "~oaríamos";
        conditional_second: "~oarías", "~oaríais";
        conditional_third: "~oaría", "~oarían";
        present_subjunctive_first: "~oe", "~oemos";
        present_subjunctive_second: "~oes", "~oéis";
        present_subjunctive_third: "~oe", "~oen";
        imperfect_subjunctive_first: "~oara", "~oáramos";
        imperfect_subjunctive_second: "~oaras", "~oarais";
        imperfect_subjunctive_third: "~oara", "~oaran";
        imperfect_subjunctive_se_first: "~oase", "~oásemos";
        imperfect_subjunctive_se_second: "~oases", "~oaseis";
        imperfect_subjunctive_se_third: "~oase", "~oasen";
        future_subjunctive_first: "~oare", "~oáremos";
        future_subjunctive_second: "~oares", "~oareis";
        future_subjunctive_third: "~oare", "~oaren";
        vos: "~oás", "~oés", "~oate";
        imperative: "~óate", "~óese", "~oémonos", "~oaos", "~óense";
        negative_imperative: "~oes", "~oe", "~oemos", "~oéis", "~oen";
        infinitive: "~oarse";
        gerund: "~oándose";
        participle_masculine: "~oado", "~oados";
        participle_feminine: "~oada", "~oadas";
    ),
    // Trenzarse: 3 words (trenzarse, destrenzarse, homogeneizarse).
    conjugation!("trenzarse", 3, "~e~zarse";
        present_first: "~e~zo", "~e~zamos";
        present_second: "~e~zas", "~e~záis";
        present_third: "~e~za", "~e~zan";
        imperfect_first: "~e~zaba", "~e~zábamos";
        imperfect_second: "~e~zabas", "~e~zabais";
        imperfect_third: "~e~zaba", "~e~zaban";
        preterite_first: "~e~cé", "~e~zamos";
        preterite_second: "~e~zaste", "~e~zasteis";
        preterite_third: "~e~zó", "~e~zaron";
        future_first: "~e~zaré", "~e~zaremos";
        future_second: "~e~zarás", "~e~zaréis";
        future_third: "~e~zará", "~e~zarán";
        conditional_first: "~e~zaría", "~e~zaríamos";
        conditional_second: "~e~zarías", "~e~zaríais";
        conditional_third: "~e~zaría", "~e~zarían";
        present_subjunctive_first: "~e~ce", "~e~cemos";
        present_subjunctive_second: "~e~ces", "~e~céis";
        present_subjunctive_third: "~e~ce", "~e~cen";
        imperfect_subjunctive_first: "~e~zara", "~e~záramos";
        imperfect_subjunctive_second: "~e~zaras", "~e~zarais";
        imperfect_subjunctive_third: "~e~zara", "~e~zaran";
        imperfect_subjunctive_se_first: "~e~zase", "~e~zásemos";
        imperfect_subjunctive_se_second: "~e~zases", "~e~zaseis";
        imperfect_subjunctive_se_third: "~e~zase", "~e~zasen";
        future_subjunctive_first: "~e~zare", "~e~záremos";
        future_subjunctive_second: "~e~zares", "~e~zareis";
        future_subjunctive_third: "~e~zare", "~e~zaren";
        vos: "~e~zás", "~e~cés", "~e~zate";
        imperative: "~é~zate", "~é~cese", "~e~cémonos", "~e~zaos", "~é~cense";
        negative_imperative: "~e~ces", "~e~ce", "~e~cemos", "~e~céis", "~e~cen";
        infinitive: "~e~zarse";
        gerund: "~e~zándose";
        participle_masculine: "~e~zado", "~e~zados";
        participle_feminine: "~e~zada", "~e~zadas";
    ),
    // Cubrirse: 3 words (descubrirse, cubrirse, encubrirse).
    conjugation!("cubrirse", 3, "~u~~irse";
        present_first: "~u~~o", "~u~~imos";
        present_second: "~u~~es", "~u~~ís";
        present_third: "~u~~e", "~u~~en";
        imperfect_first: "~u~~ía", "~u~~íamos";
        imperfect_second: "~u~~ías", "~u~~íais";
        imperfect_third: "~u~~ía", "~u~~ían";
        preterite_first: "~u~~í", "~u~~imos";
        preterite_second: "~u~~iste", "~u~~isteis";
        preterite_third: "~u~~ió", "~u~~ieron";
        future_first: "~u~~iré", "~u~~iremos";
        future_second: "~u~~irás", "~u~~iréis";
        future_third: "~u~~irá", "~u~~irán";
        conditional_first: "~u~~iría", "~u~~iríamos";
        conditional_second: "~u~~irías", "~u~~iríais";
        conditional_third: "~u~~iría", "~u~~irían";
        present_subjunctive_first: "~u~~a", "~u~~amos";
        present_subjunctive_second: "~u~~as", "~u~~áis";
        present_subjunctive_third: "~u~~a", "~u~~an";
        imperfect_subjunctive_first: "~u~~iera", "~u~~iéramos";
        imperfect_subjunctive_second: "~u~~ieras", "~u~~ierais";
        imperfect_subjunctive_third: "~u~~iera", "~u~~ieran";
        imperfect_subjunctive_se_first: "~u~~iese", "~u~~iésemos";
        imperfect_subjunctive_se_second: "~u~~ieses", "~u~~ieseis";
        imperfect_subjunctive_se_third: "~u~~iese", "~u~~iesen";
        future_subjunctive_first: "~u~~iere", "~u~~iéremos";
        future_subjunctive_second: "~u~~ieres", "~u~~iereis";
        future_subjunctive_third: "~u~~iere", "~u~~ieren";
        vos: "~u~~ís", "~u~~ás", "~u~~ite";
        imperative: "~ú~~ete", "~ú~~ase", "~u~~ámonos", "~u~~íos", "~ú~~anse";
        negative_imperative: "~u~~as", "~u~~a", "~u~~amos", "~u~~áis", "~u~~an";
        infinitive: "~u~~irse";
        gerund: "~u~~iéndose";
        participle_masculine: "~u~ie~to", "~u~ie~tos";
        participle_feminine: "~u~ie~ta", "~u~ie~tas";
    ),
    // Derretirse: 3 words (derretirse, competirse, repetirse).
    conjugation!("derretirse", 3, "~etirse";
        present_first: "~ito", "~etimos";
        present_second: "~ites", "~etís";
        present_third: "~ite", "~iten";
        imperfect_first: "~etía", "~etíamos";
        imperfect_second: "~etías", "~etíais";
        imperfect_third: "~etía", "~etían";
        preterite_first: "~etí", "~etimos";
        preterite_second: "~etiste", "~etisteis";
        preterite_third: "~itió", "~itieron";
        future_first: "~etiré", "~etiremos";
        future_second: "~etirás", "~etiréis";
        future_third: "~etirá", "~etirán";
        conditional_first: "~etiría", "~etiríamos";
        conditional_second: "~etirías", "~etiríais";
        conditional_third: "~etiría", "~etirían";
        present_subjunctive_first: "~ita", "~itamos";
        present_subjunctive_second: "~itas", "~itáis";
        present_subjunctive_third: "~ita", "~itan";
        imperfect_subjunctive_first: "~itiera", "~itiéramos";
        imperfect_subjunctive_second: "~itieras", "~itierais";
        imperfect_subjunctive_third: "~itiera", "~itieran";
        imperfect_subjunctive_se_first: "~itiese", "~itiésemos";
        imperfect_subjunctive_se_second: "~itieses", "~itieseis";
        imperfect_subjunctive_se_third: "~itiese", "~itiesen";
        future_subjunctive_first: "~itiere", "~itiéremos";
        future_subjunctive_second: "~itieres", "~itiereis";
        future_subjunctive_third: "~itiere", "~itieren";
        vos: "~etís", "~itás", "~etite";
        imperative: "~ítete", "~ítase", "~itámonos", "~etíos", "~ítanse";
        negative_imperative: "~itas", "~ita", "~itamos", "~itáis", "~itan";
        infinitive: "~etirse";
        gerund: "~itiéndose";
        participle_masculine: "~etido", "~etidos";
        participle_feminine: "~etida", "~etidas";
    ),
    // Volcarse: 3 words (volcarse, revolcarse, emporcarse).
    conjugation!("volcarse", 3, "~o~carse";
        present_first: "~ue~co", "~o~camos";
        present_second: "~ue~cas", "~o~cáis";
        present_third: "~ue~ca", "~ue~can";
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
        present_subjunctive_first: "~ue~que", "~o~quemos";
        present_subjunctive_second: "~ue~ques", "~o~quéis";
        present_subjunctive_third: "~ue~que", "~ue~quen";
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
        imperative: "~ué~cate", "~ué~quese", "~o~quémonos", "~o~caos", "~ué~quense";
        negative_imperative: "~ue~ques", "~ue~que", "~o~quemos", "~o~quéis", "~ue~quen";
        infinitive: "~o~carse";
        gerund: "~o~cándose";
        participle_masculine: "~o~cado", "~o~cados";
        participle_feminine: "~o~cada", "~o~cadas";
    ),
    // Fingirse: 3 words (refringirse, fingirse, atingirse).
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
    // Azonzarse: 3 words (azonzarse, desgonzarse, tronzarse).
    conjugation!("azonzarse", 3, "~o~zarse";
        present_first: "~o~zo", "~o~zamos";
        present_second: "~o~zas", "~o~záis";
        present_third: "~o~za", "~o~zan";
        imperfect_first: "~o~zaba", "~o~zábamos";
        imperfect_second: "~o~zabas", "~o~zabais";
        imperfect_third: "~o~zaba", "~o~zaban";
        preterite_first: "~o~cé", "~o~zamos";
        preterite_second: "~o~zaste", "~o~zasteis";
        preterite_third: "~o~zó", "~o~zaron";
        future_first: "~o~zaré", "~o~zaremos";
        future_second: "~o~zarás", "~o~zaréis";
        future_third: "~o~zará", "~o~zarán";
        conditional_first: "~o~zaría", "~o~zaríamos";
        conditional_second: "~o~zarías", "~o~zaríais";
        conditional_third: "~o~zaría", "~o~zarían";
        present_subjunctive_first: "~o~ce", "~o~cemos";
        present_subjunctive_second: "~o~ces", "~o~céis";
        present_subjunctive_third: "~o~ce", "~o~cen";
        imperfect_subjunctive_first: "~o~zara", "~o~záramos";
        imperfect_subjunctive_second: "~o~zaras", "~o~zarais";
        imperfect_subjunctive_third: "~o~zara", "~o~zaran";
        imperfect_subjunctive_se_first: "~o~zase", "~o~zásemos";
        imperfect_subjunctive_se_second: "~o~zases", "~o~zaseis";
        imperfect_subjunctive_se_third: "~o~zase", "~o~zasen";
        future_subjunctive_first: "~o~zare", "~o~záremos";
        future_subjunctive_second: "~o~zares", "~o~zareis";
        future_subjunctive_third: "~o~zare", "~o~zaren";
        vos: "~o~zás", "~o~cés", "~o~zate";
        imperative: "~ó~zate", "~ó~cese", "~o~cémonos", "~o~zaos", "~ó~cense";
        negative_imperative: "~o~ces", "~o~ce", "~o~cemos", "~o~céis", "~o~cen";
        infinitive: "~o~zarse";
        gerund: "~o~zándose";
        participle_masculine: "~o~zado", "~o~zados";
        participle_feminine: "~o~zada", "~o~zadas";
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
    // Embaírse: 1 words (embaírse).
    conjugation!("embaírse", 1, "~~írse";
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
    // Ingrirse: 1 words (ingrirse).
    conjugation!("ingrirse", 1, "i~irse";
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
    // Unirse: 1 words (unirse).
    conjugation!("unirse", 1, "u~irse";
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
    // Abrirse: 1 words (abrirse).
    conjugation!("abrirse", 1, "a~~irse";
        present_first: "a~~o", "a~~imos";
        present_second: "a~~es", "a~~ís";
        present_third: "a~~e", "a~~en";
        imperfect_first: "a~~ía", "a~~íamos";
        imperfect_second: "a~~ías", "a~~íais";
        imperfect_third: "a~~ía", "a~~ían";
        preterite_first: "a~~í", "a~~imos";
        preterite_second: "a~~iste", "a~~isteis";
        preterite_third: "a~~ió", "a~~ieron";
        future_first: "a~~iré", "a~~iremos";
        future_second: "a~~irás", "a~~iréis";
        future_third: "a~~irá", "a~~irán";
        conditional_first: "a~~iría", "a~~iríamos";
        conditional_second: "a~~irías", "a~~iríais";
        conditional_third: "a~~iría", "a~~irían";
        present_subjunctive_first: "a~~a", "a~~amos";
        present_subjunctive_second: "a~~as", "a~~áis";
        present_subjunctive_third: "a~~a", "a~~an";
        imperfect_subjunctive_first: "a~~iera", "a~~iéramos";
        imperfect_subjunctive_second: "a~~ieras", "a~~ierais";
        imperfect_subjunctive_third: "a~~iera", "a~~ieran";
        imperfect_subjunctive_se_first: "a~~iese", "a~~iésemos";
        imperfect_subjunctive_se_second: "a~~ieses", "a~~ieseis";
        imperfect_subjunctive_se_third: "a~~iese", "a~~iesen";
        future_subjunctive_first: "a~~iere", "a~~iéremos";
        future_subjunctive_second: "a~~ieres", "a~~iereis";
        future_subjunctive_third: "a~~iere", "a~~ieren";
        vos: "a~~ís", "a~~ás", "a~~ite";
        imperative: "á~~ete", "á~~ase", "a~~ámonos", "a~~íos", "á~~anse";
        negative_imperative: "a~~as", "a~~a", "a~~amos", "a~~áis", "a~~an";
        infinitive: "a~~irse";
        gerund: "a~~iéndose";
        participle_masculine: "a~ie~to", "a~ie~tos";
        participle_feminine: "a~ie~ta", "a~ie~tas";
    ),
    // Asirse: 1 words (asirse).
    conjugation!("asirse", 1, "a~irse";
        present_first: "a~go", "a~imos";
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
        present_subjunctive_first: "a~ga", "a~gamos";
        present_subjunctive_second: "a~gas", "a~gáis";
        present_subjunctive_third: "a~ga", "a~gan";
        imperfect_subjunctive_first: "a~iera", "a~iéramos";
        imperfect_subjunctive_second: "a~ieras", "a~ierais";
        imperfect_subjunctive_third: "a~iera", "a~ieran";
        imperfect_subjunctive_se_first: "a~iese", "a~iésemos";
        imperfect_subjunctive_se_second: "a~ieses", "a~ieseis";
        imperfect_subjunctive_se_third: "a~iese", "a~iesen";
        future_subjunctive_first: "a~iere", "a~iéremos";
        future_subjunctive_second: "a~ieres", "a~iereis";
        future_subjunctive_third: "a~iere", "a~ieren";
        vos: "a~ís", "a~gás", "a~ite";
        imperative: "á~ete", "á~gase", "a~gámonos", "a~íos", "á~ganse";
        negative_imperative: "a~gas", "a~ga", "a~gamos", "a~gáis", "a~gan";
        infinitive: "a~irse";
        gerund: "a~iéndose";
        participle_masculine: "a~ido", "a~idos";
        participle_feminine: "a~ida", "a~idas";
    ),
];
