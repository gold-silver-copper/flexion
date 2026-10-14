//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the conjugation's it is like, or, in a conjugation like
/// no other, derived):
/// `pres_1` (sg, pl)
/// `pres_2` (sg, pl)
/// `pres_3` (sg, pl)
/// `impf_1` (sg, pl)
/// `impf_2` (sg, pl)
/// `impf_3` (sg, pl)
/// `pret_1` (sg, pl)
/// `pret_2` (sg, pl)
/// `pret_3` (sg, pl)
/// `fut_1` (sg, pl)
/// `fut_2` (sg, pl)
/// `fut_3` (sg, pl)
/// `cond_1` (sg, pl)
/// `cond_2` (sg, pl)
/// `cond_3` (sg, pl)
/// `subj_1` (sg, pl)
/// `subj_2` (sg, pl)
/// `subj_3` (sg, pl)
/// `subj_impf_1` (sg, pl)
/// `subj_impf_2` (sg, pl)
/// `subj_impf_3` (sg, pl)
/// `subj_impf_se_1` (sg, pl)
/// `subj_impf_se_2` (sg, pl)
/// `subj_impf_se_3` (sg, pl)
/// `subj_fut_1` (sg, pl)
/// `subj_fut_2` (sg, pl)
/// `subj_fut_3` (sg, pl)
/// `vos` (ind, subj, imp)
/// `imperative` (tu, usted, nosotros, vosotros, ustedes)
/// `negative_imperative` (tu, usted, nosotros, vosotros, ustedes)
/// `infinitive`
/// `gerund`
/// `participle_m` (sg, pl)
/// `participle_f` (sg, pl)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem verb_pres_1_sg: $s0:tt;)?
     $(stem verb_pres_1_pl: $s1:tt;)?
     $(stem verb: $s2:tt;)?
     $(stem verb_fut_1_sg: $s3:tt;)?
     $(stem verb_subj_1_pl: $s4:tt;)?
     $(stem participle: $s5:tt;)?
     $(pres_1: $c0:tt, $c1:tt;)?
     $(pres_2: $c2:tt, $c3:tt;)?
     $(pres_3: $c4:tt, $c5:tt;)?
     $(impf_1: $c6:tt, $c7:tt;)?
     $(impf_2: $c8:tt, $c9:tt;)?
     $(impf_3: $c10:tt, $c11:tt;)?
     $(pret_1: $c12:tt, $c13:tt;)?
     $(pret_2: $c14:tt, $c15:tt;)?
     $(pret_3: $c16:tt, $c17:tt;)?
     $(fut_1: $c18:tt, $c19:tt;)?
     $(fut_2: $c20:tt, $c21:tt;)?
     $(fut_3: $c22:tt, $c23:tt;)?
     $(cond_1: $c24:tt, $c25:tt;)?
     $(cond_2: $c26:tt, $c27:tt;)?
     $(cond_3: $c28:tt, $c29:tt;)?
     $(subj_1: $c30:tt, $c31:tt;)?
     $(subj_2: $c32:tt, $c33:tt;)?
     $(subj_3: $c34:tt, $c35:tt;)?
     $(subj_impf_1: $c36:tt, $c37:tt;)?
     $(subj_impf_2: $c38:tt, $c39:tt;)?
     $(subj_impf_3: $c40:tt, $c41:tt;)?
     $(subj_impf_se_1: $c42:tt, $c43:tt;)?
     $(subj_impf_se_2: $c44:tt, $c45:tt;)?
     $(subj_impf_se_3: $c46:tt, $c47:tt;)?
     $(subj_fut_1: $c48:tt, $c49:tt;)?
     $(subj_fut_2: $c50:tt, $c51:tt;)?
     $(subj_fut_3: $c52:tt, $c53:tt;)?
     $(vos: $c54:tt, $c55:tt, $c56:tt;)?
     $(imperative: $c57:tt, $c58:tt, $c59:tt, $c60:tt, $c61:tt;)?
     $(negative_imperative: $c62:tt, $c63:tt, $c64:tt, $c65:tt, $c66:tt;)?
     $(infinitive: $c67:tt;)?
     $(gerund: $c68:tt;)?
     $(participle_m: $c69:tt, $c70:tt;)?
     $(participle_f: $c71:tt, $c72:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
                $(flexion::cell!($c48),)? "\t",
                $(flexion::cell!($c49),)? "\t",
                $(flexion::cell!($c50),)? "\t",
                $(flexion::cell!($c51),)? "\t",
                $(flexion::cell!($c52),)? "\t",
                $(flexion::cell!($c53),)? "\t",
                $(flexion::cell!($c54),)? "\t",
                $(flexion::cell!($c55),)? "\t",
                $(flexion::cell!($c56),)? "\t",
                $(flexion::cell!($c57),)? "\t",
                $(flexion::cell!($c58),)? "\t",
                $(flexion::cell!($c59),)? "\t",
                $(flexion::cell!($c60),)? "\t",
                $(flexion::cell!($c61),)? "\t",
                $(flexion::cell!($c62),)? "\t",
                $(flexion::cell!($c63),)? "\t",
                $(flexion::cell!($c64),)? "\t",
                $(flexion::cell!($c65),)? "\t",
                $(flexion::cell!($c66),)? "\t",
                $(flexion::cell!($c67),)? "\t",
                $(flexion::cell!($c68),)? "\t",
                $(flexion::cell!($c69),)? "\t",
                $(flexion::cell!($c70),)? "\t",
                $(flexion::cell!($c71),)? "\t",
                $(flexion::cell!($c72),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
            )),
        }
    };
}

/// The conjugation a conjugation is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Conjugation::$parent as u16)
    };
}

/// The cells a conjugation reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pret_as_pres`: one value read as another).
const PRET_AS_PRES: [u16; 73] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1, 2, 3, 4, 5, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `echar`: 9,691 words.
    Echar,
    /// Like `realizar`: 1,202 words.
    Realizar,
    /// Like `incautarse`: 724 words.
    Incautarse,
    /// Like `acomodarse`: 591 words.
    Acomodarse,
    /// Like `quejarse`: 564 words.
    Quejarse,
    /// Like `rilarse`: 548 words.
    Rilarse,
    /// Like `emputarse`: 398 words.
    Emputarse,
    /// Like `regodearse`: 358 words.
    Regodearse,
    /// Like `conocer`: 334 words.
    Conocer,
    /// Like `curtir`: 327 words.
    Curtir,
    /// Like `sacar`: 252 words.
    Sacar,
    /// Like `fosilizarse`: 249 words.
    Fosilizarse,
    /// Like `autoabastecerse`: 176 words.
    Autoabastecerse,
    /// Like `cerrar`: 157 words.
    Cerrar,
    /// Like `meter`: 145 words.
    Meter,
    /// Like `osificarse`: 134 words.
    Osificarse,
    /// Like `volar`: 117 words.
    Volar,
    /// Like `darse`: 115 words.
    Darse,
    /// Like `desviar`: 109 words.
    Desviar,
    /// Like `instruir`: 105 words.
    Instruir,
    /// Like `atravesarse`: 70 words.
    Atravesarse,
    /// Like `sentir`: 56 words.
    Sentir,
    /// Like `situar`: 56 words.
    Situar,
    /// Like `desencontrarse`: 54 words.
    Desencontrarse,
    /// Like `musirse`: 54 words.
    Musirse,
    /// Like `dirigir`: 46 words.
    Dirigir,
    /// Like `resfriarse`: 40 words.
    Resfriarse,
    /// Like `poner`: 37 words.
    Poner,
    /// Like `fraguar`: 32 words.
    Fraguar,
    /// Like `rendir`: 32 words.
    Rendir,
    /// Like `descocarse`: 31 words.
    Descocarse,
    /// Like `desvivirse`: 30 words.
    Desvivirse,
    /// Like `situarse`: 27 words.
    Situarse,
    /// Like `embazarse`: 27 words.
    Embazarse,
    /// Like `destoserse`: 27 words.
    Destoserse,
    /// Like `venir`: 24 words.
    Venir,
    /// Like `reducir`: 24 words.
    Reducir,
    /// Like `escribir`: 23 words.
    Escribir,
    /// Like `sacarse`: 23 words.
    Sacarse,
    /// Like `meliar`: 23 words.
    Meliar,
    /// Like `coger`: 22 words.
    Coger,
    /// Like `descomponerse`: 22 words.
    Descomponerse,
    /// Like `ahijar`: 22 words.
    Ahijar,
    /// Like `alzarse`: 22 words.
    Alzarse,
    /// Like `entremeterse`: 21 words.
    Entremeterse,
    /// Like `inmiscuirse`: 21 words.
    Inmiscuirse,
    /// Like `aunar`: 20 words.
    Aunar,
    /// Like `abatirse`: 20 words.
    Abatirse,
    /// Like `fruncir`: 19 words.
    Fruncir,
    /// Like `chuzarse`: 19 words.
    Chuzarse,
    /// Like `resentirse`: 19 words.
    Resentirse,
    /// Like `calzarse`: 18 words.
    Calzarse,
    /// Like `desentenderse`: 17 words.
    Desentenderse,
    /// Like `creer`: 17 words.
    Creer,
    /// Like `traer`: 16 words.
    Traer,
    /// Like `entecarse`: 16 words.
    Entecarse,
    /// Like `aborrascarse`: 14 words.
    Aborrascarse,
    /// Like `revolver`: 14 words.
    Revolver,
    /// Like `rozarse`: 14 words.
    Rozarse,
    /// Like `vencer`: 13 words.
    Vencer,
    /// Like `tener`: 13 words.
    Tener,
    /// Like `ceñir`: 13 words.
    Ceñir,
    /// Like `enfoscarse`: 12 words.
    Enfoscarse,
    /// Like `enraizar`: 12 words.
    Enraizar,
    /// Like `ahilarse`: 12 words.
    Ahilarse,
    /// Like `armarse`: 12 words.
    Armarse,
    /// Like `hacer`: 12 words.
    Hacer,
    /// Like `empezar`: 11 words.
    Empezar,
    /// Like `escabullirse`: 11 words.
    Escabullirse,
    /// Like `apanucarse`: 11 words.
    Apanucarse,
    /// Like `suscribirse`: 10 words.
    Suscribirse,
    /// Like `retraerse`: 10 words.
    Retraerse,
    /// Like `lucir`: 10 words.
    Lucir,
    /// Like `enamoriscarse`: 9 words.
    Enamoriscarse,
    /// Like `apuñuscarse`: 9 words.
    Apuñuscarse,
    /// Like `abrir`: 9 words.
    Abrir,
    /// Like `trocar`: 9 words.
    Trocar,
    /// Like `resolverse`: 9 words.
    Resolverse,
    /// Like `detenerse`: 9 words.
    Detenerse,
    /// Like `santiguarse`: 9 words.
    Santiguarse,
    /// Like `refreír`: 9 words.
    Refreír,
    /// Like `guiar`: 8 words.
    Guiar,
    /// Like `reducirse`: 8 words.
    Reducirse,
    /// Like `desmedirse`: 8 words.
    Desmedirse,
    /// Like `creerse`: 8 words.
    Creerse,
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
    /// Like `aterirse`: 7 words.
    Aterirse,
    /// Like `jetiarse`: 7 words.
    Jetiarse,
    /// Like `deshacerse`: 7 words.
    Deshacerse,
    /// Like `relamerse`: 7 words.
    Relamerse,
    /// Like `seguir`: 6 words.
    Seguir,
    /// Like `agorar`: 6 words.
    Agorar,
    /// Like `apajuilar`: 6 words.
    Apajuilar,
    /// Like `recogerse`: 6 words.
    Recogerse,
    /// Like `oír`: 6 words.
    Oír,
    /// Like `desdecir`: 6 words.
    Desdecir,
    /// Like `entredormirse`: 6 words.
    Entredormirse,
    /// Like `tender`: 5 words.
    Tender,
    /// Like `caer`: 5 words.
    Caer,
    /// Like `querer`: 5 words.
    Querer,
    /// Like `huir`: 5 words.
    Huir,
    /// Like `cocer`: 5 words.
    Cocer,
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
    /// Like `revestirse`: 5 words.
    Revestirse,
    /// Like `freírse`: 5 words.
    Freírse,
    /// Like `dormir`: 5 words.
    Dormir,
    /// Like `salir`: 5 words.
    Salir,
    /// Like `decir`: 5 words.
    Decir,
    /// Like `saber`: 4 words.
    Saber,
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
    /// Like `desforzarse`: 4 words.
    Desforzarse,
    /// Like `regirse`: 4 words.
    Regirse,
    /// Like `fraguarse`: 4 words.
    Fraguarse,
    /// Like `moverse`: 4 words.
    Moverse,
    /// Like `verse`: 4 words.
    Verse,
    /// Like `criarse`: 4 words.
    Criarse,
    /// Like `punzarse`: 4 words.
    Punzarse,
    /// Like `traslucirse`: 4 words.
    Traslucirse,
    /// Like `torcerse`: 4 words.
    Torcerse,
    /// Like `servirse`: 4 words.
    Servirse,
    /// Like `vencerse`: 4 words.
    Vencerse,
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
    /// Like `apucuyar`: 3 words.
    Apucuyar,
    /// Like `antojar`: 3 words.
    Antojar,
    /// Like `enraizarse`: 3 words.
    Enraizarse,
    /// Like `escocerse`: 3 words.
    Escocerse,
    /// Like `seguirse`: 3 words.
    Seguirse,
    /// Like `azoarse`: 3 words.
    Azoarse,
    /// Like `nacerse`: 3 words.
    Nacerse,
    /// Like `trenzarse`: 3 words.
    Trenzarse,
    /// Like `cubrirse`: 3 words.
    Cubrirse,
    /// Like `derretirse`: 3 words.
    Derretirse,
    /// Like `fingirse`: 3 words.
    Fingirse,
    /// Like `azonzarse`: 3 words.
    Azonzarse,
    /// Like `asirse`: 2 words.
    Asirse,
    /// Like `reírse`: 1 words.
    Reírse,
    /// Like `ir`: 1 words.
    Ir,
    /// Like `ingrirse`: 1 words.
    Ingrirse,
    /// Like `irse`: 1 words.
    Irse,
    /// Like `unirse`: 1 words.
    Unirse,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 159] = [
        Conjugation::Echar,
        Conjugation::Realizar,
        Conjugation::Incautarse,
        Conjugation::Acomodarse,
        Conjugation::Quejarse,
        Conjugation::Rilarse,
        Conjugation::Emputarse,
        Conjugation::Regodearse,
        Conjugation::Conocer,
        Conjugation::Curtir,
        Conjugation::Sacar,
        Conjugation::Fosilizarse,
        Conjugation::Autoabastecerse,
        Conjugation::Cerrar,
        Conjugation::Meter,
        Conjugation::Osificarse,
        Conjugation::Volar,
        Conjugation::Darse,
        Conjugation::Desviar,
        Conjugation::Instruir,
        Conjugation::Atravesarse,
        Conjugation::Sentir,
        Conjugation::Situar,
        Conjugation::Desencontrarse,
        Conjugation::Musirse,
        Conjugation::Dirigir,
        Conjugation::Resfriarse,
        Conjugation::Poner,
        Conjugation::Fraguar,
        Conjugation::Rendir,
        Conjugation::Descocarse,
        Conjugation::Desvivirse,
        Conjugation::Situarse,
        Conjugation::Embazarse,
        Conjugation::Destoserse,
        Conjugation::Venir,
        Conjugation::Reducir,
        Conjugation::Escribir,
        Conjugation::Sacarse,
        Conjugation::Meliar,
        Conjugation::Coger,
        Conjugation::Descomponerse,
        Conjugation::Ahijar,
        Conjugation::Alzarse,
        Conjugation::Entremeterse,
        Conjugation::Inmiscuirse,
        Conjugation::Aunar,
        Conjugation::Abatirse,
        Conjugation::Fruncir,
        Conjugation::Chuzarse,
        Conjugation::Resentirse,
        Conjugation::Calzarse,
        Conjugation::Desentenderse,
        Conjugation::Creer,
        Conjugation::Traer,
        Conjugation::Entecarse,
        Conjugation::Aborrascarse,
        Conjugation::Revolver,
        Conjugation::Rozarse,
        Conjugation::Vencer,
        Conjugation::Tener,
        Conjugation::Ceñir,
        Conjugation::Enfoscarse,
        Conjugation::Enraizar,
        Conjugation::Ahilarse,
        Conjugation::Armarse,
        Conjugation::Hacer,
        Conjugation::Empezar,
        Conjugation::Escabullirse,
        Conjugation::Apanucarse,
        Conjugation::Suscribirse,
        Conjugation::Retraerse,
        Conjugation::Lucir,
        Conjugation::Enamoriscarse,
        Conjugation::Apuñuscarse,
        Conjugation::Abrir,
        Conjugation::Trocar,
        Conjugation::Resolverse,
        Conjugation::Detenerse,
        Conjugation::Santiguarse,
        Conjugation::Refreír,
        Conjugation::Guiar,
        Conjugation::Reducirse,
        Conjugation::Desmedirse,
        Conjugation::Creerse,
        Conjugation::Moler,
        Conjugation::Regir,
        Conjugation::Mover,
        Conjugation::Ver,
        Conjugation::Tañer,
        Conjugation::Ahumarse,
        Conjugation::Venirse,
        Conjugation::Aterirse,
        Conjugation::Jetiarse,
        Conjugation::Deshacerse,
        Conjugation::Relamerse,
        Conjugation::Seguir,
        Conjugation::Agorar,
        Conjugation::Apajuilar,
        Conjugation::Recogerse,
        Conjugation::Oír,
        Conjugation::Desdecir,
        Conjugation::Entredormirse,
        Conjugation::Tender,
        Conjugation::Caer,
        Conjugation::Querer,
        Conjugation::Huir,
        Conjugation::Cocer,
        Conjugation::Delinquir,
        Conjugation::Afligirse,
        Conjugation::Reconocerse,
        Conjugation::Teñirse,
        Conjugation::Trocarse,
        Conjugation::Revestirse,
        Conjugation::Freírse,
        Conjugation::Dormir,
        Conjugation::Salir,
        Conjugation::Decir,
        Conjugation::Saber,
        Conjugation::Forzar,
        Conjugation::Adquirir,
        Conjugation::Cohibir,
        Conjugation::Morir,
        Conjugation::Distinguir,
        Conjugation::Torcer,
        Conjugation::Desforzarse,
        Conjugation::Regirse,
        Conjugation::Fraguarse,
        Conjugation::Moverse,
        Conjugation::Verse,
        Conjugation::Criarse,
        Conjugation::Punzarse,
        Conjugation::Traslucirse,
        Conjugation::Torcerse,
        Conjugation::Servirse,
        Conjugation::Vencerse,
        Conjugation::Yacer,
        Conjugation::Argüir,
        Conjugation::Jugar,
        Conjugation::Romper,
        Conjugation::Valer,
        Conjugation::Apucuyar,
        Conjugation::Antojar,
        Conjugation::Enraizarse,
        Conjugation::Escocerse,
        Conjugation::Seguirse,
        Conjugation::Azoarse,
        Conjugation::Nacerse,
        Conjugation::Trenzarse,
        Conjugation::Cubrirse,
        Conjugation::Derretirse,
        Conjugation::Fingirse,
        Conjugation::Azonzarse,
        Conjugation::Asirse,
        Conjugation::Reírse,
        Conjugation::Ir,
        Conjugation::Ingrirse,
        Conjugation::Irse,
        Conjugation::Unirse,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 159] = [
    // Echar: 9,691 words (estar, amar, generar, crear, dar, abandonar).
    conjugation!("echar", 9691, "~ar";
    stem participle: "~ad- like echar";
    pres_1: "~o", "~amos";
    pres_2: "~as", "~áis";
    pres_3: "~a", "~an";
    impf_1: "~aba", "~ábamos";
    impf_2: "~abas", "~abais";
    impf_3: _, "~aban";
    pret_1: "~é", "~amos";
    pret_2: "~aste", "~asteis";
    pret_3: "~ó", "~aron";
    fut_1: "~aré", "~aremos";
    fut_2: "~arás", "~aréis";
    fut_3: "~ará", "~arán";
    cond_1: "~aría", "~aríamos";
    cond_2: "~arías", "~aríais";
    cond_3: _, "~arían";
    subj_1: "~e", "~emos";
    subj_2: "~es", "~éis";
    subj_3: _, "~en";
    subj_impf_1: "~ara", "~áramos";
    subj_impf_2: "~aras", "~arais";
    subj_impf_3: _, "~aran";
    subj_impf_se_1: "~ase", "~ásemos";
    subj_impf_se_2: "~ases", "~aseis";
    subj_impf_se_3: _, "~asen";
    subj_fut_1: "~are", "~áremos";
    subj_fut_2: "~ares", "~areis";
    subj_fut_3: _, "~aren";
    vos: "~ás", "~és", "~á";
    imperative: "~a", "~e", "~emos", "~ad", "~en";
    infinitive: "~ar";
    gerund: "~ando";
    ),
    // Realizar: 1,202 words (profundizar, utilizar, realzar, avergonzar, lanzar, alcanzar).
    conjugation!("realizar", 1202, "~zar";
    stem participle: "~zad- like echar";
    pres_1: "~zo", "~zamos";
    pres_2: "~zas", "~záis";
    pres_3: "~za", "~zan";
    impf_1: "~zaba", "~zábamos";
    impf_2: "~zabas", "~zabais";
    impf_3: _, "~zaban";
    pret_1: "~cé", "~zamos";
    pret_2: "~zaste", "~zasteis";
    pret_3: "~zó", "~zaron";
    fut_1: "~zaré", "~zaremos";
    fut_2: "~zarás", "~zaréis";
    fut_3: "~zará", "~zarán";
    cond_1: "~zaría", "~zaríamos";
    cond_2: "~zarías", "~zaríais";
    cond_3: _, "~zarían";
    subj_1: "~ce", "~cemos";
    subj_2: "~ces", "~céis";
    subj_3: _, "~cen";
    subj_impf_1: "~zara", "~záramos";
    subj_impf_2: "~zaras", "~zarais";
    subj_impf_3: _, "~zaran";
    subj_impf_se_1: "~zase", "~zásemos";
    subj_impf_se_2: "~zases", "~zaseis";
    subj_impf_se_3: _, "~zasen";
    subj_fut_1: "~zare", "~záremos";
    subj_fut_2: "~zares", "~zareis";
    subj_fut_3: _, "~zaren";
    vos: "~zás", "~cés", "~zá";
    imperative: "~za", "~ce", "~cemos", "~zad", "~cen";
    infinitive: "~zar";
    gerund: "~zando";
    ),
    // Incautarse: 724 words (llamarse, jambarse, amoratarse, infartarse, autoengañarse, ahorcajarse).
    conjugation!("incautarse", 724, "~a~arse";
    stem participle: "~a~ad- like echar";
    pres_1: "~a~o", "~a~amos";
    pres_2: "~a~as", "~a~áis";
    pres_3: "~a~a", "~a~an";
    impf_1: "~a~aba", "~a~ábamos";
    impf_2: "~a~abas", "~a~abais";
    impf_3: _, "~a~aban";
    pret_1: "~a~é", "~a~amos";
    pret_2: "~a~aste", "~a~asteis";
    pret_3: "~a~ó", "~a~aron";
    fut_1: "~a~aré", "~a~aremos";
    fut_2: "~a~arás", "~a~aréis";
    fut_3: "~a~ará", "~a~arán";
    cond_1: "~a~aría", "~a~aríamos";
    cond_2: "~a~arías", "~a~aríais";
    cond_3: _, "~a~arían";
    subj_1: "~a~e", "~a~emos";
    subj_2: "~a~es", "~a~éis";
    subj_3: _, "~a~en";
    subj_impf_1: "~a~ara", "~a~áramos";
    subj_impf_2: "~a~aras", "~a~arais";
    subj_impf_3: _, "~a~aran";
    subj_impf_se_1: "~a~ase", "~a~ásemos";
    subj_impf_se_2: "~a~ases", "~a~aseis";
    subj_impf_se_3: _, "~a~asen";
    subj_fut_1: "~a~are", "~a~áremos";
    subj_fut_2: "~a~ares", "~a~areis";
    subj_fut_3: _, "~a~aren";
    vos: "~a~ás", "~a~és", "~a~ate";
    imperative: "~á~ate", "~á~ese", "~a~émonos", "~a~aos", "~á~ense";
    infinitive: "~a~arse";
    gerund: "~a~ándose";
    ),
    // Acomodarse: 591 words (acodarse, acomodarse, endrogarse, personarse, autocontrolarse, amonarse).
    conjugation!("acomodarse", 591, "~o~arse";
    stem participle: "~o~ad- like echar";
    pres_1: "~o~o", "~o~amos";
    pres_2: "~o~as", "~o~áis";
    pres_3: "~o~a", "~o~an";
    impf_1: "~o~aba", "~o~ábamos";
    impf_2: "~o~abas", "~o~abais";
    impf_3: _, "~o~aban";
    pret_1: "~o~é", "~o~amos";
    pret_2: "~o~aste", "~o~asteis";
    pret_3: "~o~ó", "~o~aron";
    fut_1: "~o~aré", "~o~aremos";
    fut_2: "~o~arás", "~o~aréis";
    fut_3: "~o~ará", "~o~arán";
    cond_1: "~o~aría", "~o~aríamos";
    cond_2: "~o~arías", "~o~aríais";
    cond_3: _, "~o~arían";
    subj_1: "~o~e", "~o~emos";
    subj_2: "~o~es", "~o~éis";
    subj_3: _, "~o~en";
    subj_impf_1: "~o~ara", "~o~áramos";
    subj_impf_2: "~o~aras", "~o~arais";
    subj_impf_3: _, "~o~aran";
    subj_impf_se_1: "~o~ase", "~o~ásemos";
    subj_impf_se_2: "~o~ases", "~o~aseis";
    subj_impf_se_3: _, "~o~asen";
    subj_fut_1: "~o~are", "~o~áremos";
    subj_fut_2: "~o~ares", "~o~areis";
    subj_fut_3: _, "~o~aren";
    vos: "~o~ás", "~o~és", "~o~ate";
    imperative: "~ó~ate", "~ó~ese", "~o~émonos", "~o~aos", "~ó~ense";
    infinitive: "~o~arse";
    gerund: "~o~ándose";
    ),
    // Quejarse: 564 words (quejarse, querellarse, despertarse, pegarse, autocremarse, afiebrarse).
    conjugation!("quejarse", 564, "~e~arse";
    stem participle: "~e~ad- like echar";
    pres_1: "~e~o", "~e~amos";
    pres_2: "~e~as", "~e~áis";
    pres_3: "~e~a", "~e~an";
    impf_1: "~e~aba", "~e~ábamos";
    impf_2: "~e~abas", "~e~abais";
    impf_3: _, "~e~aban";
    pret_1: "~e~é", "~e~amos";
    pret_2: "~e~aste", "~e~asteis";
    pret_3: "~e~ó", "~e~aron";
    fut_1: "~e~aré", "~e~aremos";
    fut_2: "~e~arás", "~e~aréis";
    fut_3: "~e~ará", "~e~arán";
    cond_1: "~e~aría", "~e~aríamos";
    cond_2: "~e~arías", "~e~aríais";
    cond_3: _, "~e~arían";
    subj_1: "~e~e", "~e~emos";
    subj_2: "~e~es", "~e~éis";
    subj_3: _, "~e~en";
    subj_impf_1: "~e~ara", "~e~áramos";
    subj_impf_2: "~e~aras", "~e~arais";
    subj_impf_3: _, "~e~aran";
    subj_impf_se_1: "~e~ase", "~e~ásemos";
    subj_impf_se_2: "~e~ases", "~e~aseis";
    subj_impf_se_3: _, "~e~asen";
    subj_fut_1: "~e~are", "~e~áremos";
    subj_fut_2: "~e~ares", "~e~areis";
    subj_fut_3: _, "~e~aren";
    vos: "~e~ás", "~e~és", "~e~ate";
    imperative: "~é~ate", "~é~ese", "~e~émonos", "~e~aos", "~é~ense";
    infinitive: "~e~arse";
    gerund: "~e~ándose";
    ),
    // Rilarse: 548 words (fijarse, chisparse, ababillarse, agüitarse, refanfinflarse, encapricharse).
    conjugation!("rilarse", 548, "~i~arse";
    stem participle: "~i~ad- like echar";
    pres_1: "~i~o", "~i~amos";
    pres_2: "~i~as", "~i~áis";
    pres_3: "~i~a", "~i~an";
    impf_1: "~i~aba", "~i~ábamos";
    impf_2: "~i~abas", "~i~abais";
    impf_3: _, "~i~aban";
    pret_1: "~i~é", "~i~amos";
    pret_2: "~i~aste", "~i~asteis";
    pret_3: "~i~ó", "~i~aron";
    fut_1: "~i~aré", "~i~aremos";
    fut_2: "~i~arás", "~i~aréis";
    fut_3: "~i~ará", "~i~arán";
    cond_1: "~i~aría", "~i~aríamos";
    cond_2: "~i~arías", "~i~aríais";
    cond_3: _, "~i~arían";
    subj_1: "~i~e", "~i~emos";
    subj_2: "~i~es", "~i~éis";
    subj_3: _, "~i~en";
    subj_impf_1: "~i~ara", "~i~áramos";
    subj_impf_2: "~i~aras", "~i~arais";
    subj_impf_3: _, "~i~aran";
    subj_impf_se_1: "~i~ase", "~i~ásemos";
    subj_impf_se_2: "~i~ases", "~i~aseis";
    subj_impf_se_3: _, "~i~asen";
    subj_fut_1: "~i~are", "~i~áremos";
    subj_fut_2: "~i~ares", "~i~areis";
    subj_fut_3: _, "~i~aren";
    vos: "~i~ás", "~i~és", "~i~ate";
    imperative: "~í~ate", "~í~ese", "~i~émonos", "~i~aos", "~í~ense";
    infinitive: "~i~arse";
    gerund: "~i~ándose";
    ),
    // Emputarse: 398 words (acalenturarse, emputarse, confabularse, autoinculparse, enfurruñarse, despreocuparse).
    conjugation!("emputarse", 398, "~u~arse";
    stem participle: "~u~ad- like echar";
    pres_1: "~u~o", "~u~amos";
    pres_2: "~u~as", "~u~áis";
    pres_3: "~u~a", "~u~an";
    impf_1: "~u~aba", "~u~ábamos";
    impf_2: "~u~abas", "~u~abais";
    impf_3: _, "~u~aban";
    pret_1: "~u~é", "~u~amos";
    pret_2: "~u~aste", "~u~asteis";
    pret_3: "~u~ó", "~u~aron";
    fut_1: "~u~aré", "~u~aremos";
    fut_2: "~u~arás", "~u~aréis";
    fut_3: "~u~ará", "~u~arán";
    cond_1: "~u~aría", "~u~aríamos";
    cond_2: "~u~arías", "~u~aríais";
    cond_3: _, "~u~arían";
    subj_1: "~u~e", "~u~emos";
    subj_2: "~u~es", "~u~éis";
    subj_3: _, "~u~en";
    subj_impf_1: "~u~ara", "~u~áramos";
    subj_impf_2: "~u~aras", "~u~arais";
    subj_impf_3: _, "~u~aran";
    subj_impf_se_1: "~u~ase", "~u~ásemos";
    subj_impf_se_2: "~u~ases", "~u~aseis";
    subj_impf_se_3: _, "~u~asen";
    subj_fut_1: "~u~are", "~u~áremos";
    subj_fut_2: "~u~ares", "~u~areis";
    subj_fut_3: _, "~u~aren";
    vos: "~u~ás", "~u~és", "~u~ate";
    imperative: "~ú~ate", "~ú~ese", "~u~émonos", "~u~aos", "~ú~ense";
    infinitive: "~u~arse";
    gerund: "~u~ándose";
    ),
    // Regodearse: 358 words (carcajearse, petatearse, autobombearse, regodearse, mamonearse, cachiporrearse).
    conjugation!("regodearse", 358, "~~arse";
    stem participle: "~~ad- like echar";
    pres_1: "~~o", "~~amos";
    pres_2: "~~as", "~~áis";
    pres_3: "~~a", "~~an";
    impf_1: "~~aba", "~~ábamos";
    impf_2: "~~abas", "~~abais";
    impf_3: _, "~~aban";
    pret_1: "~~é", "~~amos";
    pret_2: "~~aste", "~~asteis";
    pret_3: "~~ó", "~~aron";
    fut_1: "~~aré", "~~aremos";
    fut_2: "~~arás", "~~aréis";
    fut_3: "~~ará", "~~arán";
    cond_1: "~~aría", "~~aríamos";
    cond_2: "~~arías", "~~aríais";
    cond_3: _, "~~arían";
    subj_1: "~~e", "~~emos";
    subj_2: "~~es", "~~éis";
    subj_3: _, "~~en";
    subj_impf_1: "~~ara", "~~áramos";
    subj_impf_2: "~~aras", "~~arais";
    subj_impf_3: _, "~~aran";
    subj_impf_se_1: "~~ase", "~~ásemos";
    subj_impf_se_2: "~~ases", "~~aseis";
    subj_impf_se_3: _, "~~asen";
    subj_fut_1: "~~are", "~~áremos";
    subj_fut_2: "~~ares", "~~areis";
    subj_fut_3: _, "~~aren";
    vos: "~~ás", "~~és", "~~ate";
    imperative: "~éat~", "~é~se", "~~émonos", "~~aos", "~é~nse";
    infinitive: "~~arse";
    gerund: "~~ándose";
    ),
    // Conocer: 334 words (conocer, tender, placer, perder, encender, nacer).
    conjugation!("conocer", 334, "~~er";
    stem participle: "~~id- like echar";
    pres_1: "~z~o", "~~emos";
    pres_2: "~~es", "~~éis";
    pres_3: "~~e", "~~en";
    impf_1: "~~ía", "~~íamos";
    impf_2: "~~ías", "~~íais";
    impf_3: _, "~~ían";
    pret_1: "~~í", "~~imos";
    pret_2: "~~iste", "~~isteis";
    pret_3: "~~ió", "~~ieron";
    fut_1: "~~eré", "~~eremos";
    fut_2: "~~erás", "~~eréis";
    fut_3: "~~erá", "~~erán";
    cond_1: "~~ería", "~~eríamos";
    cond_2: "~~erías", "~~eríais";
    cond_3: _, "~~erían";
    subj_1: "~z~a", "~z~amos";
    subj_2: "~z~as", "~z~áis";
    subj_3: _, "~z~an";
    subj_impf_1: "~~iera", "~~iéramos";
    subj_impf_2: "~~ieras", "~~ierais";
    subj_impf_3: _, "~~ieran";
    subj_impf_se_1: "~~iese", "~~iésemos";
    subj_impf_se_2: "~~ieses", "~~ieseis";
    subj_impf_se_3: _, "~~iesen";
    subj_fut_1: "~~iere", "~~iéremos";
    subj_fut_2: "~~ieres", "~~iereis";
    subj_fut_3: _, "~~ieren";
    vos: "~~és", "~z~ás", "~~é";
    imperative: "~~e", "~z~a", "~z~amos", "~~ed", "~z~an";
    infinitive: "~~er";
    gerund: "~~iendo";
    ),
    // Curtir: 327 words (partir, polir, abolir, cumplir, finir, vivir).
    conjugation!("curtir", 327, "~ir";
    stem participle: "~id- like echar";
    pres_1: "~o", "~imos";
    pres_2: "~es", "~ís";
    pres_3: "~e", "~en";
    impf_1: "~ía", "~íamos";
    impf_2: "~ías", "~íais";
    impf_3: _, "~ían";
    pret_1: "~í", "~imos";
    pret_2: "~iste", "~isteis";
    pret_3: "~ió", "~ieron";
    fut_1: "~iré", "~iremos";
    fut_2: "~irás", "~iréis";
    fut_3: "~irá", "~irán";
    cond_1: "~iría", "~iríamos";
    cond_2: "~irías", "~iríais";
    cond_3: _, "~irían";
    subj_1: "~a", "~amos";
    subj_2: "~as", "~áis";
    subj_3: _, "~an";
    subj_impf_1: "~iera", "~iéramos";
    subj_impf_2: "~ieras", "~ierais";
    subj_impf_3: _, "~ieran";
    subj_impf_se_1: "~iese", "~iésemos";
    subj_impf_se_2: "~ieses", "~ieseis";
    subj_impf_se_3: _, "~iesen";
    subj_fut_1: "~iere", "~iéremos";
    subj_fut_2: "~ieres", "~iereis";
    subj_fut_3: _, "~ieren";
    vos: "~ís", "~ás", "~í";
    imperative: "~e", "~a", "~amos", "~id", "~an";
    infinitive: "~ir";
    gerund: "~iendo";
    ),
    // Sacar: 252 words (buscar, sacar, rascar, pecar, tocar, atacar).
    conjugation!("sacar", 252, "~car";
    stem participle: "~cad- like echar";
    pres_1: "~co", "~camos";
    pres_2: "~cas", "~cáis";
    pres_3: "~ca", "~can";
    impf_1: "~caba", "~cábamos";
    impf_2: "~cabas", "~cabais";
    impf_3: _, "~caban";
    pret_1: "~qué", "~camos";
    pret_2: "~caste", "~casteis";
    pret_3: "~có", "~caron";
    fut_1: "~caré", "~caremos";
    fut_2: "~carás", "~caréis";
    fut_3: "~cará", "~carán";
    cond_1: "~caría", "~caríamos";
    cond_2: "~carías", "~caríais";
    cond_3: _, "~carían";
    subj_1: "~que", "~quemos";
    subj_2: "~ques", "~quéis";
    subj_3: _, "~quen";
    subj_impf_1: "~cara", "~cáramos";
    subj_impf_2: "~caras", "~carais";
    subj_impf_3: _, "~caran";
    subj_impf_se_1: "~case", "~cásemos";
    subj_impf_se_2: "~cases", "~caseis";
    subj_impf_se_3: _, "~casen";
    subj_fut_1: "~care", "~cáremos";
    subj_fut_2: "~cares", "~careis";
    subj_fut_3: _, "~caren";
    vos: "~cás", "~qués", "~cá";
    imperative: "~ca", "~que", "~quemos", "~cad", "~quen";
    infinitive: "~car";
    gerund: "~cando";
    ),
    // Fosilizarse: 249 words (fosilizarse, autorrealizarse, autobautizarse, autoanalizarse, autopolinizarse, autoorganizarse).
    conjugation!("fosilizarse", 249, "~izarse";
    stem participle: "~izad- like echar";
    pres_1: "~izo", "~izamos";
    pres_2: "~izas", "~izáis";
    pres_3: "~iza", "~izan";
    impf_1: "~izaba", "~izábamos";
    impf_2: "~izabas", "~izabais";
    impf_3: _, "~izaban";
    pret_1: "~icé", "~izamos";
    pret_2: "~izaste", "~izasteis";
    pret_3: "~izó", "~izaron";
    fut_1: "~izaré", "~izaremos";
    fut_2: "~izarás", "~izaréis";
    fut_3: "~izará", "~izarán";
    cond_1: "~izaría", "~izaríamos";
    cond_2: "~izarías", "~izaríais";
    cond_3: _, "~izarían";
    subj_1: "~ice", "~icemos";
    subj_2: "~ices", "~icéis";
    subj_3: _, "~icen";
    subj_impf_1: "~izara", "~izáramos";
    subj_impf_2: "~izaras", "~izarais";
    subj_impf_3: _, "~izaran";
    subj_impf_se_1: "~izase", "~izásemos";
    subj_impf_se_2: "~izases", "~izaseis";
    subj_impf_se_3: _, "~izasen";
    subj_fut_1: "~izare", "~izáremos";
    subj_fut_2: "~izares", "~izareis";
    subj_fut_3: _, "~izaren";
    vos: "~izás", "~icés", "~izate";
    imperative: "~ízate", "~ícese", "~icémonos", "~izaos", "~ícense";
    infinitive: "~izarse";
    gerund: "~izándose";
    ),
    // Autoabastecerse: 176 words (autoabastecerse, entrometerse, entremeterse, autoprotegerse, entigrecerse, ensilvecerse).
    conjugation!("autoabastecerse", 176, "~e~erse";
    stem participle: "~e~id- like echar";
    pres_1: "~ez~o", "~e~emos";
    pres_2: "~e~es", "~e~éis";
    pres_3: "~e~e", "~e~en";
    impf_1: "~e~ía", "~e~íamos";
    impf_2: "~e~ías", "~e~íais";
    impf_3: _, "~e~ían";
    pret_1: "~e~í", "~e~imos";
    pret_2: "~e~iste", "~e~isteis";
    pret_3: "~e~ió", "~e~ieron";
    fut_1: "~e~eré", "~e~eremos";
    fut_2: "~e~erás", "~e~eréis";
    fut_3: "~e~erá", "~e~erán";
    cond_1: "~e~ería", "~e~eríamos";
    cond_2: "~e~erías", "~e~eríais";
    cond_3: _, "~e~erían";
    subj_1: "~ez~a", "~ez~amos";
    subj_2: "~ez~as", "~ez~áis";
    subj_3: _, "~ez~an";
    subj_impf_1: "~e~iera", "~e~iéramos";
    subj_impf_2: "~e~ieras", "~e~ierais";
    subj_impf_3: _, "~e~ieran";
    subj_impf_se_1: "~e~iese", "~e~iésemos";
    subj_impf_se_2: "~e~ieses", "~e~ieseis";
    subj_impf_se_3: _, "~e~iesen";
    subj_fut_1: "~e~iere", "~e~iéremos";
    subj_fut_2: "~e~ieres", "~e~iereis";
    subj_fut_3: _, "~e~ieren";
    vos: "~e~és", "~ez~ás", "~e~ete";
    imperative: "~é~ete", "~éz~ase", "~ez~ámonos", "~e~eos", "~éz~anse";
    infinitive: "~e~erse";
    gerund: "~e~iéndose";
    ),
    // Cerrar: 157 words (pensar, quebrar, helar, negar, gobernar, confesar). Like Regodearse, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("cerrar", 157, "~~ar", like Regodearse;
    pres_1: "~i~o", _;
    pres_2: "~i~as", _;
    pres_3: "~i~a", "~i~an";
    subj_1: "~i~e", _;
    subj_2: "~i~es", _;
    subj_3: _, "~i~en";
    vos: _, _, "~~á";
    imperative: "~i~a", "~i~e", "~~emos", "~~ad", "~i~en";
    infinitive: "~~ar";
    gerund: "~~ando";
    ),
    // Meter: 145 words (absconder, absorber, her, meter, acceder, beber). Like Curtir, except in 17 cells: pres_1 pl, pres_2 pl, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, ….
    conjugation!("meter", 145, "~er", like Curtir;
    pres_1: _, "~emos";
    pres_2: _, "~éis";
    fut_1: "~eré", "~eremos";
    fut_2: "~erás", "~eréis";
    fut_3: "~erá", "~erán";
    cond_1: "~ería", "~eríamos";
    cond_2: "~erías", "~eríais";
    cond_3: _, "~erían";
    vos: "~és", _, "~é";
    imperative: _, _, _, "~ed", _;
    infinitive: "~er";
    ),
    // Osificarse: 134 words (gelificarse, automedicarse, autocalificarse, autoaplicarse, osificarse, autoidentificarse).
    conjugation!("osificarse", 134, "~icarse";
    stem participle: "~icad- like echar";
    pres_1: "~ico", "~icamos";
    pres_2: "~icas", "~icáis";
    pres_3: "~ica", "~ican";
    impf_1: "~icaba", "~icábamos";
    impf_2: "~icabas", "~icabais";
    impf_3: _, "~icaban";
    pret_1: "~iqué", "~icamos";
    pret_2: "~icaste", "~icasteis";
    pret_3: "~icó", "~icaron";
    fut_1: "~icaré", "~icaremos";
    fut_2: "~icarás", "~icaréis";
    fut_3: "~icará", "~icarán";
    cond_1: "~icaría", "~icaríamos";
    cond_2: "~icarías", "~icaríais";
    cond_3: _, "~icarían";
    subj_1: "~ique", "~iquemos";
    subj_2: "~iques", "~iquéis";
    subj_3: _, "~iquen";
    subj_impf_1: "~icara", "~icáramos";
    subj_impf_2: "~icaras", "~icarais";
    subj_impf_3: _, "~icaran";
    subj_impf_se_1: "~icase", "~icásemos";
    subj_impf_se_2: "~icases", "~icaseis";
    subj_impf_se_3: _, "~icasen";
    subj_fut_1: "~icare", "~icáremos";
    subj_fut_2: "~icares", "~icareis";
    subj_fut_3: _, "~icaren";
    vos: "~icás", "~iqués", "~icate";
    imperative: "~ícate", "~íquese", "~iquémonos", "~icaos", "~íquense";
    infinitive: "~icarse";
    gerund: "~icándose";
    ),
    // Volar: 117 words (sonar, soltar, encontrar, volar, solar, acostar). Like Acomodarse, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("volar", 117, "~o~ar", like Acomodarse;
    pres_1: "~ue~o", _;
    pres_2: "~ue~as", _;
    pres_3: "~ue~a", "~ue~an";
    subj_1: "~ue~e", _;
    subj_2: "~ue~es", _;
    subj_3: _, "~ue~en";
    vos: _, _, "~o~á";
    imperative: "~ue~a", "~ue~e", "~o~emos", "~o~ad", "~ue~en";
    infinitive: "~o~ar";
    gerund: "~o~ando";
    ),
    // Darse: 115 words (acovacharse, abicharse, abrocharse, arreguincharse, reguincharse, emborracharse). Like Echar, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("darse", 115, "~arse", like Echar;
    vos: _, _, "~áte";
    imperative: "~ate", "~ese", "~émonos", "~aos", "~ense";
    infinitive: "~arse";
    gerund: "~ándose";
    ),
    // Desviar: 109 words (enviar, enfriar, ampliar, aviar, confiar, agriar).
    conjugation!("desviar", 109, "~iar";
    stem participle: "~iad- like echar";
    pres_1: "~ío", "~iamos";
    pres_2: "~ías", "~iáis";
    pres_3: "~ía", "~ían";
    impf_1: "~iaba", "~iábamos";
    impf_2: "~iabas", "~iabais";
    impf_3: _, "~iaban";
    pret_1: "~ié", "~iamos";
    pret_2: "~iaste", "~iasteis";
    pret_3: "~ió", "~iaron";
    fut_1: "~iaré", "~iaremos";
    fut_2: "~iarás", "~iaréis";
    fut_3: "~iará", "~iarán";
    cond_1: "~iaría", "~iaríamos";
    cond_2: "~iarías", "~iaríais";
    cond_3: _, "~iarían";
    subj_1: "~íe", "~iemos";
    subj_2: "~íes", "~iéis";
    subj_3: _, "~íen";
    subj_impf_1: "~iara", "~iáramos";
    subj_impf_2: "~iaras", "~iarais";
    subj_impf_3: _, "~iaran";
    subj_impf_se_1: "~iase", "~iásemos";
    subj_impf_se_2: "~iases", "~iaseis";
    subj_impf_se_3: _, "~iasen";
    subj_fut_1: "~iare", "~iáremos";
    subj_fut_2: "~iares", "~iareis";
    subj_fut_3: _, "~iaren";
    vos: "~iás", "~iés", "~iá";
    imperative: "~ía", "~íe", "~iemos", "~iad", "~íen";
    infinitive: "~iar";
    gerund: "~iando";
    ),
    // Instruir: 105 words (construir, destruir, bullir, incluir, excluir, sustituir). Like Curtir, except in 32 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, pret_3 sg, pret_3 pl, ….
    conjugation!("instruir", 105, "~ir", like Curtir;
    pres_1: "~yo", _;
    pres_2: "~yes", _;
    pres_3: "~ye", "~yen";
    pret_3: "~yó", "~yeron";
    subj_1: "~ya", "~yamos";
    subj_2: "~yas", "~yáis";
    subj_3: _, "~yan";
    subj_impf_1: "~yera", "~yéramos";
    subj_impf_2: "~yeras", "~yerais";
    subj_impf_3: _, "~yeran";
    subj_impf_se_1: "~yese", "~yésemos";
    subj_impf_se_2: "~yeses", "~yeseis";
    subj_impf_se_3: _, "~yesen";
    subj_fut_1: "~yere", "~yéremos";
    subj_fut_2: "~yeres", "~yereis";
    subj_fut_3: _, "~yeren";
    vos: _, "~yás", _;
    imperative: "~ye", "~ya", "~yamos", _, "~yan";
    gerund: "~yendo";
    ),
    // Atravesarse: 70 words (autogobernarse, quebrarse, abnegarse, sentarse, regarse, atravesarse). Like Quejarse, except in 10 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("atravesarse", 70, "~e~arse", like Quejarse;
    pres_1: "~ie~o", _;
    pres_2: "~ie~as", _;
    pres_3: "~ie~a", "~ie~an";
    subj_1: "~ie~e", _;
    subj_2: "~ie~es", _;
    subj_3: _, "~ie~en";
    imperative: "~ié~ate", "~ié~ese", _, _, "~ié~ense";
    ),
    // Sentir: 56 words (mentir, sentir, zaherir, herir, advertir, sugerir).
    conjugation!("sentir", 56, "~e~ir";
    stem participle: "~e~id- like echar";
    pres_1: "~ie~o", "~e~imos";
    pres_2: "~ie~es", "~e~ís";
    pres_3: "~ie~e", "~ie~en";
    impf_1: "~e~ía", "~e~íamos";
    impf_2: "~e~ías", "~e~íais";
    impf_3: _, "~e~ían";
    pret_1: "~e~í", "~e~imos";
    pret_2: "~e~iste", "~e~isteis";
    pret_3: "~i~ió", "~i~ieron";
    fut_1: "~e~iré", "~e~iremos";
    fut_2: "~e~irás", "~e~iréis";
    fut_3: "~e~irá", "~e~irán";
    cond_1: "~e~iría", "~e~iríamos";
    cond_2: "~e~irías", "~e~iríais";
    cond_3: _, "~e~irían";
    subj_1: "~ie~a", "~i~amos";
    subj_2: "~ie~as", "~i~áis";
    subj_3: _, "~ie~an";
    subj_impf_1: "~i~iera", "~i~iéramos";
    subj_impf_2: "~i~ieras", "~i~ierais";
    subj_impf_3: _, "~i~ieran";
    subj_impf_se_1: "~i~iese", "~i~iésemos";
    subj_impf_se_2: "~i~ieses", "~i~ieseis";
    subj_impf_se_3: _, "~i~iesen";
    subj_fut_1: "~i~iere", "~i~iéremos";
    subj_fut_2: "~i~ieres", "~i~iereis";
    subj_fut_3: _, "~i~ieren";
    vos: "~e~ís", "~i~ás", "~e~í";
    imperative: "~ie~e", "~ie~a", "~i~amos", "~e~id", "~ie~an";
    infinitive: "~e~ir";
    gerund: "~i~iendo";
    ),
    // Situar: 56 words (puntuar, continuar, efectuar, actuar, situar, menstruar).
    conjugation!("situar", 56, "~uar";
    stem participle: "~uad- like echar";
    pres_1: "~úo", "~uamos";
    pres_2: "~úas", "~uáis";
    pres_3: "~úa", "~úan";
    impf_1: "~uaba", "~uábamos";
    impf_2: "~uabas", "~uabais";
    impf_3: _, "~uaban";
    pret_1: "~ué", "~uamos";
    pret_2: "~uaste", "~uasteis";
    pret_3: "~uó", "~uaron";
    fut_1: "~uaré", "~uaremos";
    fut_2: "~uarás", "~uaréis";
    fut_3: "~uará", "~uarán";
    cond_1: "~uaría", "~uaríamos";
    cond_2: "~uarías", "~uaríais";
    cond_3: _, "~uarían";
    subj_1: "~úe", "~uemos";
    subj_2: "~úes", "~uéis";
    subj_3: _, "~úen";
    subj_impf_1: "~uara", "~uáramos";
    subj_impf_2: "~uaras", "~uarais";
    subj_impf_3: _, "~uaran";
    subj_impf_se_1: "~uase", "~uásemos";
    subj_impf_se_2: "~uases", "~uaseis";
    subj_impf_se_3: _, "~uasen";
    subj_fut_1: "~uare", "~uáremos";
    subj_fut_2: "~uares", "~uareis";
    subj_fut_3: _, "~uaren";
    vos: "~uás", "~ués", "~uá";
    imperative: "~úa", "~úe", "~uemos", "~uad", "~úen";
    infinitive: "~uar";
    gerund: "~uando";
    ),
    // Desencontrarse: 54 words (desencontrarse, colarse, encontrarse, poblarse, encordarse, acostarse). Like Volar, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("desencontrarse", 54, "~o~arse", like Volar;
    vos: _, _, "~o~ate";
    imperative: "~ué~ate", "~ué~ese", "~o~émonos", "~o~aos", "~ué~ense";
    infinitive: "~o~arse";
    gerund: "~o~ándose";
    ),
    // Musirse: 54 words (desnutrirse, musirse, entumirse, ocurrirse, consumirse, eludirse).
    conjugation!("musirse", 54, "~u~irse";
    stem participle: "~u~id- like echar";
    pres_1: "~u~o", "~u~imos";
    pres_2: "~u~es", "~u~ís";
    pres_3: "~u~e", "~u~en";
    impf_1: "~u~ía", "~u~íamos";
    impf_2: "~u~ías", "~u~íais";
    impf_3: _, "~u~ían";
    pret_1: "~u~í", "~u~imos";
    pret_2: "~u~iste", "~u~isteis";
    pret_3: "~u~ió", "~u~ieron";
    fut_1: "~u~iré", "~u~iremos";
    fut_2: "~u~irás", "~u~iréis";
    fut_3: "~u~irá", "~u~irán";
    cond_1: "~u~iría", "~u~iríamos";
    cond_2: "~u~irías", "~u~iríais";
    cond_3: _, "~u~irían";
    subj_1: "~u~a", "~u~amos";
    subj_2: "~u~as", "~u~áis";
    subj_3: _, "~u~an";
    subj_impf_1: "~u~iera", "~u~iéramos";
    subj_impf_2: "~u~ieras", "~u~ierais";
    subj_impf_3: _, "~u~ieran";
    subj_impf_se_1: "~u~iese", "~u~iésemos";
    subj_impf_se_2: "~u~ieses", "~u~ieseis";
    subj_impf_se_3: _, "~u~iesen";
    subj_fut_1: "~u~iere", "~u~iéremos";
    subj_fut_2: "~u~ieres", "~u~iereis";
    subj_fut_3: _, "~u~ieren";
    vos: "~u~ís", "~u~ás", "~u~ite";
    imperative: "~ú~ete", "~ú~ase", "~u~ámonos", "~u~íos", "~ú~anse";
    infinitive: "~u~irse";
    gerund: "~u~iéndose";
    ),
    // Dirigir: 46 words (urgir, mugir, exigir, fingir, agir, dirigir).
    conjugation!("dirigir", 46, "~gir";
    stem participle: "~gid- like echar";
    pres_1: "~jo", "~gimos";
    pres_2: "~ges", "~gís";
    pres_3: "~ge", "~gen";
    impf_1: "~gía", "~gíamos";
    impf_2: "~gías", "~gíais";
    impf_3: _, "~gían";
    pret_1: "~gí", "~gimos";
    pret_2: "~giste", "~gisteis";
    pret_3: "~gió", "~gieron";
    fut_1: "~giré", "~giremos";
    fut_2: "~girás", "~giréis";
    fut_3: "~girá", "~girán";
    cond_1: "~giría", "~giríamos";
    cond_2: "~girías", "~giríais";
    cond_3: _, "~girían";
    subj_1: "~ja", "~jamos";
    subj_2: "~jas", "~jáis";
    subj_3: _, "~jan";
    subj_impf_1: "~giera", "~giéramos";
    subj_impf_2: "~gieras", "~gierais";
    subj_impf_3: _, "~gieran";
    subj_impf_se_1: "~giese", "~giésemos";
    subj_impf_se_2: "~gieses", "~gieseis";
    subj_impf_se_3: _, "~giesen";
    subj_fut_1: "~giere", "~giéremos";
    subj_fut_2: "~gieres", "~giereis";
    subj_fut_3: _, "~gieren";
    vos: "~gís", "~jás", "~gí";
    imperative: "~ge", "~ja", "~jamos", "~gid", "~jan";
    infinitive: "~gir";
    gerund: "~giendo";
    ),
    // Resfriarse: 40 words (resfriarse, guiarse, confiarse, demasiarse, enfriarse, aliarse). Like Desviar, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("resfriarse", 40, "~iarse", like Desviar;
    vos: _, _, "~iate";
    imperative: "~íate", "~íese", "~iémonos", "~iaos", "~íense";
    infinitive: "~iarse";
    gerund: "~iándose";
    ),
    // Poner: 37 words (poner, yuxtaponer, imponer, suponer, exponer, posponer).
    conjugation!("poner", 37, "~oner";
    stem participle: "~uest- like echar";
    pres_1: "~ongo", "~onemos";
    pres_2: "~ones", "~onéis";
    pres_3: "~one", "~onen";
    impf_1: "~onía", "~oníamos";
    impf_2: "~onías", "~oníais";
    impf_3: _, "~onían";
    pret_1: "~use", "~usimos";
    pret_2: "~usiste", "~usisteis";
    pret_3: "~uso", "~usieron";
    fut_1: "~ondré", "~ondremos";
    fut_2: "~ondrás", "~ondréis";
    fut_3: "~ondrá", "~ondrán";
    cond_1: "~ondría", "~ondríamos";
    cond_2: "~ondrías", "~ondríais";
    cond_3: _, "~ondrían";
    subj_1: "~onga", "~ongamos";
    subj_2: "~ongas", "~ongáis";
    subj_3: _, "~ongan";
    subj_impf_1: "~usiera", "~usiéramos";
    subj_impf_2: "~usieras", "~usierais";
    subj_impf_3: _, "~usieran";
    subj_impf_se_1: "~usiese", "~usiésemos";
    subj_impf_se_2: "~usieses", "~usieseis";
    subj_impf_se_3: _, "~usiesen";
    subj_fut_1: "~usiere", "~usiéremos";
    subj_fut_2: "~usieres", "~usiereis";
    subj_fut_3: _, "~usieren";
    vos: "~onés", "~ongás", "~oné";
    imperative: "~ón", "~onga", "~ongamos", "~oned", "~ongan";
    infinitive: "~oner";
    gerund: "~oniendo";
    ),
    // Fraguar: 32 words (averiguar, atestiguar, menguar, aguar, testiguar, apaciguar). Like Situar, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, pret_1 sg, subj_1 sg, ….
    conjugation!("fraguar", 32, "~uar", like Situar;
    pres_1: "~uo", _;
    pres_2: "~uas", _;
    pres_3: "~ua", "~uan";
    pret_1: "~üé", _;
    subj_1: "~üe", "~üemos";
    subj_2: "~ües", "~üéis";
    subj_3: _, "~üen";
    vos: _, "~üés", _;
    imperative: "~ua", "~üe", "~üemos", _, "~üen";
    ),
    // Rendir: 32 words (pedir, servir, despedir, vestir, investir, medir). Like Sentir, except in 10 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("rendir", 32, "~e~ir", like Sentir;
    pres_1: "~i~o", _;
    pres_2: "~i~es", _;
    pres_3: "~i~e", "~i~en";
    subj_1: "~i~a", _;
    subj_2: "~i~as", _;
    subj_3: _, "~i~an";
    imperative: "~i~e", "~i~a", _, _, "~i~an";
    ),
    // Descocarse: 31 words (autoconvocarse, descocarse, abocarse, apocarse, equivocarse, enfocarse).
    conjugation!("descocarse", 31, "~ocarse";
    stem participle: "~ocad- like echar";
    pres_1: "~oco", "~ocamos";
    pres_2: "~ocas", "~ocáis";
    pres_3: "~oca", "~ocan";
    impf_1: "~ocaba", "~ocábamos";
    impf_2: "~ocabas", "~ocabais";
    impf_3: _, "~ocaban";
    pret_1: "~oqué", "~ocamos";
    pret_2: "~ocaste", "~ocasteis";
    pret_3: "~ocó", "~ocaron";
    fut_1: "~ocaré", "~ocaremos";
    fut_2: "~ocarás", "~ocaréis";
    fut_3: "~ocará", "~ocarán";
    cond_1: "~ocaría", "~ocaríamos";
    cond_2: "~ocarías", "~ocaríais";
    cond_3: _, "~ocarían";
    subj_1: "~oque", "~oquemos";
    subj_2: "~oques", "~oquéis";
    subj_3: _, "~oquen";
    subj_impf_1: "~ocara", "~ocáramos";
    subj_impf_2: "~ocaras", "~ocarais";
    subj_impf_3: _, "~ocaran";
    subj_impf_se_1: "~ocase", "~ocásemos";
    subj_impf_se_2: "~ocases", "~ocaseis";
    subj_impf_se_3: _, "~ocasen";
    subj_fut_1: "~ocare", "~ocáremos";
    subj_fut_2: "~ocares", "~ocareis";
    subj_fut_3: _, "~ocaren";
    vos: "~ocás", "~oqués", "~ocate";
    imperative: "~ócate", "~óquese", "~oquémonos", "~ocaos", "~óquense";
    infinitive: "~ocarse";
    gerund: "~ocándose";
    ),
    // Desvivirse: 30 words (desvivirse, autodefinirse, autopercibirse, inhibirse, extinguirse, distinguirse). Like Rendir, except in 31 cells: pres_1 pl, pres_2 pl, impf_1 sg, impf_1 pl, impf_2 sg, impf_2 pl, ….
    conjugation!("desvivirse", 30, "~i~irse", like Rendir;
    stem participle: "~i~id- like echar";
    pres_1: _, "~i~imos";
    pres_2: _, "~i~ís";
    impf_1: "~i~ía", "~i~íamos";
    impf_2: "~i~ías", "~i~íais";
    impf_3: _, "~i~ían";
    pret_1: "~i~í", "~i~imos";
    pret_2: "~i~iste", "~i~isteis";
    fut_1: "~i~iré", "~i~iremos";
    fut_2: "~i~irás", "~i~iréis";
    fut_3: "~i~irá", "~i~irán";
    cond_1: "~i~iría", "~i~iríamos";
    cond_2: "~i~irías", "~i~iríais";
    cond_3: _, "~i~irían";
    vos: "~i~ís", _, "~i~ite";
    imperative: "~í~ete", "~í~ase", "~i~ámonos", "~i~íos", "~í~anse";
    infinitive: "~i~irse";
    gerund: "~i~iéndose";
    ),
    // Situarse: 27 words (autoevaluarse, adecuarse, atenuarse, extenuarse, situarse, insinuarse). Like Situar, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("situarse", 27, "~uarse", like Situar;
    vos: _, _, "~uate";
    imperative: "~úate", "~úese", "~uémonos", "~uaos", "~úense";
    infinitive: "~uarse";
    gerund: "~uándose";
    ),
    // Embazarse: 27 words (embazarse, abrazarse, acarrazarse, amostazarse, desplazarse, tarazarse).
    conjugation!("embazarse", 27, "~azarse";
    stem participle: "~azad- like echar";
    pres_1: "~azo", "~azamos";
    pres_2: "~azas", "~azáis";
    pres_3: "~aza", "~azan";
    impf_1: "~azaba", "~azábamos";
    impf_2: "~azabas", "~azabais";
    impf_3: _, "~azaban";
    pret_1: "~acé", "~azamos";
    pret_2: "~azaste", "~azasteis";
    pret_3: "~azó", "~azaron";
    fut_1: "~azaré", "~azaremos";
    fut_2: "~azarás", "~azaréis";
    fut_3: "~azará", "~azarán";
    cond_1: "~azaría", "~azaríamos";
    cond_2: "~azarías", "~azaríais";
    cond_3: _, "~azarían";
    subj_1: "~ace", "~acemos";
    subj_2: "~aces", "~acéis";
    subj_3: _, "~acen";
    subj_impf_1: "~azara", "~azáramos";
    subj_impf_2: "~azaras", "~azarais";
    subj_impf_3: _, "~azaran";
    subj_impf_se_1: "~azase", "~azásemos";
    subj_impf_se_2: "~azases", "~azaseis";
    subj_impf_se_3: _, "~azasen";
    subj_fut_1: "~azare", "~azáremos";
    subj_fut_2: "~azares", "~azareis";
    subj_fut_3: _, "~azaren";
    vos: "~azás", "~acés", "~azate";
    imperative: "~ázate", "~ácese", "~acémonos", "~azaos", "~ácense";
    infinitive: "~azarse";
    gerund: "~azándose";
    ),
    // Destoserse: 27 words (destoserse, correrse, coserse, corresponderse, dolerse, carcomerse).
    conjugation!("destoserse", 27, "~o~erse";
    stem participle: "~o~id- like echar";
    pres_1: "~o~o", "~o~emos";
    pres_2: "~o~es", "~o~éis";
    pres_3: "~o~e", "~o~en";
    impf_1: "~o~ía", "~o~íamos";
    impf_2: "~o~ías", "~o~íais";
    impf_3: _, "~o~ían";
    pret_1: "~o~í", "~o~imos";
    pret_2: "~o~iste", "~o~isteis";
    pret_3: "~o~ió", "~o~ieron";
    fut_1: "~o~eré", "~o~eremos";
    fut_2: "~o~erás", "~o~eréis";
    fut_3: "~o~erá", "~o~erán";
    cond_1: "~o~ería", "~o~eríamos";
    cond_2: "~o~erías", "~o~eríais";
    cond_3: _, "~o~erían";
    subj_1: "~o~a", "~o~amos";
    subj_2: "~o~as", "~o~áis";
    subj_3: _, "~o~an";
    subj_impf_1: "~o~iera", "~o~iéramos";
    subj_impf_2: "~o~ieras", "~o~ierais";
    subj_impf_3: _, "~o~ieran";
    subj_impf_se_1: "~o~iese", "~o~iésemos";
    subj_impf_se_2: "~o~ieses", "~o~ieseis";
    subj_impf_se_3: _, "~o~iesen";
    subj_fut_1: "~o~iere", "~o~iéremos";
    subj_fut_2: "~o~ieres", "~o~iereis";
    subj_fut_3: _, "~o~ieren";
    vos: "~o~és", "~o~ás", "~o~ete";
    imperative: "~ó~ete", "~ó~ase", "~o~ámonos", "~o~eos", "~ó~anse";
    infinitive: "~o~erse";
    gerund: "~o~iéndose";
    ),
    // Venir: 24 words (devenir, venir, intervenir, avenir, revenir, convenir). Like Sentir, except in 27 cells: pres_1 sg, pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, pret_3 sg, ….
    conjugation!("venir", 24, "~e~ir", like Sentir;
    pres_1: "~e~go", _;
    pret_1: "~i~e", "~i~imos";
    pret_2: "~i~iste", "~i~isteis";
    pret_3: "~i~o", _;
    fut_1: "~e~dré", "~e~dremos";
    fut_2: "~e~drás", "~e~dréis";
    fut_3: "~e~drá", "~e~drán";
    cond_1: "~e~dría", "~e~dríamos";
    cond_2: "~e~drías", "~e~dríais";
    cond_3: _, "~e~drían";
    subj_1: "~e~ga", "~e~gamos";
    subj_2: "~e~gas", "~e~gáis";
    subj_3: _, "~e~gan";
    vos: _, "~e~gás", _;
    imperative: "~é~", "~e~ga", "~e~gamos", _, "~e~gan";
    ),
    // Reducir: 24 words (traducir, producir, reducir, conducir, introducir, inducir).
    conjugation!("reducir", 24, "~cir";
    stem participle: "~cid- like echar";
    pres_1: "~zco", "~cimos";
    pres_2: "~ces", "~cís";
    pres_3: "~ce", "~cen";
    impf_1: "~cía", "~cíamos";
    impf_2: "~cías", "~cíais";
    impf_3: _, "~cían";
    pret_1: "~je", "~jimos";
    pret_2: "~jiste", "~jisteis";
    pret_3: "~jo", "~jeron";
    fut_1: "~ciré", "~ciremos";
    fut_2: "~cirás", "~ciréis";
    fut_3: "~cirá", "~cirán";
    cond_1: "~ciría", "~ciríamos";
    cond_2: "~cirías", "~ciríais";
    cond_3: _, "~cirían";
    subj_1: "~zca", "~zcamos";
    subj_2: "~zcas", "~zcáis";
    subj_3: _, "~zcan";
    subj_impf_1: "~jera", "~jéramos";
    subj_impf_2: "~jeras", "~jerais";
    subj_impf_3: _, "~jeran";
    subj_impf_se_1: "~jese", "~jésemos";
    subj_impf_se_2: "~jeses", "~jeseis";
    subj_impf_se_3: _, "~jesen";
    subj_fut_1: "~jere", "~jéremos";
    subj_fut_2: "~jeres", "~jereis";
    subj_fut_3: _, "~jeren";
    vos: "~cís", "~zcás", "~cí";
    imperative: "~ce", "~zca", "~zcamos", "~cid", "~zcan";
    infinitive: "~cir";
    gerund: "~ciendo";
    ),
    // Escribir: 23 words (escribir, describir, inscribir, transcribir, prescribir, proscribir).
    conjugation!("escribir", 23, "~bir";
    stem participle: "~- like escribir";
    pres_1: "~bo", "~bimos";
    pres_2: "~bes", "~bís";
    pres_3: "~be", "~ben";
    impf_1: "~bía", "~bíamos";
    impf_2: "~bías", "~bíais";
    impf_3: _, "~bían";
    pret_1: "~bí", "~bimos";
    pret_2: "~biste", "~bisteis";
    pret_3: "~bió", "~bieron";
    fut_1: "~biré", "~biremos";
    fut_2: "~birás", "~biréis";
    fut_3: "~birá", "~birán";
    cond_1: "~biría", "~biríamos";
    cond_2: "~birías", "~biríais";
    cond_3: _, "~birían";
    subj_1: "~ba", "~bamos";
    subj_2: "~bas", "~báis";
    subj_3: _, "~ban";
    subj_impf_1: "~biera", "~biéramos";
    subj_impf_2: "~bieras", "~bierais";
    subj_impf_3: _, "~bieran";
    subj_impf_se_1: "~biese", "~biésemos";
    subj_impf_se_2: "~bieses", "~bieseis";
    subj_impf_se_3: _, "~biesen";
    subj_fut_1: "~biere", "~biéremos";
    subj_fut_2: "~bieres", "~biereis";
    subj_fut_3: _, "~bieren";
    vos: "~bís", "~bás", "~bí";
    imperative: "~be", "~ba", "~bamos", "~bid", "~ban";
    infinitive: "~bir";
    gerund: "~biendo";
    ),
    // Sacarse: 23 words (abarracarse, abellacarse, desajacarse, trelacarse, atracarse, enflacarse).
    conjugation!("sacarse", 23, "~acarse";
    stem participle: "~acad- like echar";
    pres_1: "~aco", "~acamos";
    pres_2: "~acas", "~acáis";
    pres_3: "~aca", "~acan";
    impf_1: "~acaba", "~acábamos";
    impf_2: "~acabas", "~acabais";
    impf_3: _, "~acaban";
    pret_1: "~aqué", "~acamos";
    pret_2: "~acaste", "~acasteis";
    pret_3: "~acó", "~acaron";
    fut_1: "~acaré", "~acaremos";
    fut_2: "~acarás", "~acaréis";
    fut_3: "~acará", "~acarán";
    cond_1: "~acaría", "~acaríamos";
    cond_2: "~acarías", "~acaríais";
    cond_3: _, "~acarían";
    subj_1: "~aque", "~aquemos";
    subj_2: "~aques", "~aquéis";
    subj_3: _, "~aquen";
    subj_impf_1: "~acara", "~acáramos";
    subj_impf_2: "~acaras", "~acarais";
    subj_impf_3: _, "~acaran";
    subj_impf_se_1: "~acase", "~acásemos";
    subj_impf_se_2: "~acases", "~acaseis";
    subj_impf_se_3: _, "~acasen";
    subj_fut_1: "~acare", "~acáremos";
    subj_fut_2: "~acares", "~acareis";
    subj_fut_3: _, "~acaren";
    vos: "~acás", "~aqués", "~acate";
    imperative: "~ácate", "~áquese", "~aquémonos", "~acaos", "~áquense";
    infinitive: "~acarse";
    gerund: "~acándose";
    ),
    // Meliar: 23 words (gilidiar, chalequiar, meliar, cangrejiar, jariniar, jonjoliar). Like Desviar, except in 10 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("meliar", 23, "~iar", like Desviar;
    pres_1: "~eo", _;
    pres_2: "~eas", _;
    pres_3: "~ea", "~ean";
    subj_1: "~ee", _;
    subj_2: "~ees", _;
    subj_3: _, "~een";
    imperative: "~ea", "~ee", _, _, "~een";
    ),
    // Coger: 22 words (proteger, coger, recoger, escoger, acoger, emerger). Like Dirigir, except in 17 cells: pres_1 pl, pres_2 pl, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, ….
    conjugation!("coger", 22, "~ger", like Dirigir;
    pres_1: _, "~gemos";
    pres_2: _, "~géis";
    fut_1: "~geré", "~geremos";
    fut_2: "~gerás", "~geréis";
    fut_3: "~gerá", "~gerán";
    cond_1: "~gería", "~geríamos";
    cond_2: "~gerías", "~geríais";
    cond_3: _, "~gerían";
    vos: "~gés", _, "~gé";
    imperative: _, _, _, "~ged", _;
    infinitive: "~ger";
    ),
    // Descomponerse: 22 words (autoimponerse, ponerse, imponerse, disponerse, exponerse, reponerse). Like Poner, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("descomponerse", 22, "~onerse", like Poner;
    vos: _, _, "~onete";
    imperative: "~onte", "~óngase", "~ongámonos", "~oneos", "~ónganse";
    infinitive: "~onerse";
    gerund: "~oniéndose";
    ),
    // Ahijar: 22 words (prohijar, descafeinar, aislar, ahincar, amohinar, cabrahigar). Like Rilarse, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("ahijar", 22, "~i~ar", like Rilarse;
    pres_1: "~í~o", _;
    pres_2: "~í~as", _;
    pres_3: "~í~a", "~í~an";
    subj_1: "~í~e", _;
    subj_2: "~í~es", _;
    subj_3: _, "~í~en";
    vos: _, _, "~i~á";
    imperative: "~í~a", "~í~e", "~i~emos", "~i~ad", "~í~en";
    infinitive: "~i~ar";
    gerund: "~i~ando";
    ),
    // Alzarse: 22 words (emperezarse, esperezarse, alzarse, vezarse, avezarse, avergonzarse).
    conjugation!("alzarse", 22, "~~zarse";
    stem participle: "~~zad- like echar";
    pres_1: "~~zo", "~~zamos";
    pres_2: "~~zas", "~~záis";
    pres_3: "~~za", "~~zan";
    impf_1: "~~zaba", "~~zábamos";
    impf_2: "~~zabas", "~~zabais";
    impf_3: _, "~~zaban";
    pret_1: "~~cé", "~~zamos";
    pret_2: "~~zaste", "~~zasteis";
    pret_3: "~~zó", "~~zaron";
    fut_1: "~~zaré", "~~zaremos";
    fut_2: "~~zarás", "~~zaréis";
    fut_3: "~~zará", "~~zarán";
    cond_1: "~~zaría", "~~zaríamos";
    cond_2: "~~zarías", "~~zaríais";
    cond_3: _, "~~zarían";
    subj_1: "~~ce", "~~cemos";
    subj_2: "~~ces", "~~céis";
    subj_3: _, "~~cen";
    subj_impf_1: "~~zara", "~~záramos";
    subj_impf_2: "~~zaras", "~~zarais";
    subj_impf_3: _, "~~zaran";
    subj_impf_se_1: "~~zase", "~~zásemos";
    subj_impf_se_2: "~~zases", "~~zaseis";
    subj_impf_se_3: _, "~~zasen";
    subj_fut_1: "~~zare", "~~záremos";
    subj_fut_2: "~~zares", "~~zareis";
    subj_fut_3: _, "~~zaren";
    vos: "~~zás", "~~cés", "~~zate";
    imperative: "~ézat~", "~éc~se", "~~cémonos", "~~zaos", "~éc~nse";
    infinitive: "~~zarse";
    gerund: "~~zándose";
    ),
    // Entremeterse: 21 words (prenderse, comprenderse, tejerse, sorprenderse, emperescerse, venderse). Like Autoabastecerse, except in 10 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("entremeterse", 21, "~e~erse", like Autoabastecerse;
    pres_1: "~e~o", _;
    subj_1: "~e~a", "~e~amos";
    subj_2: "~e~as", "~e~áis";
    subj_3: _, "~e~an";
    vos: _, "~e~ás", _;
    imperative: _, "~é~ase", "~e~ámonos", _, "~é~anse";
    ),
    // Inmiscuirse: 21 words (autodestruirse, inmiscuirse, autoexcluirse, autoconstruirse, rehuirse, atribuirse).
    conjugation!("inmiscuirse", 21, "~uirse";
    stem participle: "~uid- like echar";
    pres_1: "~uyo", "~uimos";
    pres_2: "~uyes", "~uís";
    pres_3: "~uye", "~uyen";
    impf_1: "~uía", "~uíamos";
    impf_2: "~uías", "~uíais";
    impf_3: _, "~uían";
    pret_1: "~uí", "~uimos";
    pret_2: "~uiste", "~uisteis";
    pret_3: "~uyó", "~uyeron";
    fut_1: "~uiré", "~uiremos";
    fut_2: "~uirás", "~uiréis";
    fut_3: "~uirá", "~uirán";
    cond_1: "~uiría", "~uiríamos";
    cond_2: "~uirías", "~uiríais";
    cond_3: _, "~uirían";
    subj_1: "~uya", "~uyamos";
    subj_2: "~uyas", "~uyáis";
    subj_3: _, "~uyan";
    subj_impf_1: "~uyera", "~uyéramos";
    subj_impf_2: "~uyeras", "~uyerais";
    subj_impf_3: _, "~uyeran";
    subj_impf_se_1: "~uyese", "~uyésemos";
    subj_impf_se_2: "~uyeses", "~uyeseis";
    subj_impf_se_3: _, "~uyesen";
    subj_fut_1: "~uyere", "~uyéremos";
    subj_fut_2: "~uyeres", "~uyereis";
    subj_fut_3: _, "~uyeren";
    vos: "~uís", "~uyás", "~uite";
    imperative: "~úyete", "~úyase", "~uyámonos", "~uíos", "~úyanse";
    infinitive: "~uirse";
    gerund: "~uyéndose";
    ),
    // Aunar: 20 words (aunar, rehusar, ahumar, maullar, aupar, aullar). Like Emputarse, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("aunar", 20, "~u~ar", like Emputarse;
    pres_1: "~ú~o", _;
    pres_2: "~ú~as", _;
    pres_3: "~ú~a", "~ú~an";
    subj_1: "~ú~e", _;
    subj_2: "~ú~es", _;
    subj_3: _, "~ú~en";
    vos: _, _, "~u~á";
    imperative: "~ú~a", "~ú~e", "~u~emos", "~u~ad", "~ú~en";
    infinitive: "~u~ar";
    gerund: "~u~ando";
    ),
    // Abatirse: 20 words (batirse, partirse, persuadirse, abatirse, debatirse, expandirse).
    conjugation!("abatirse", 20, "~a~irse";
    stem participle: "~a~id- like echar";
    pres_1: "~a~o", "~a~imos";
    pres_2: "~a~es", "~a~ís";
    pres_3: "~a~e", "~a~en";
    impf_1: "~a~ía", "~a~íamos";
    impf_2: "~a~ías", "~a~íais";
    impf_3: _, "~a~ían";
    pret_1: "~a~í", "~a~imos";
    pret_2: "~a~iste", "~a~isteis";
    pret_3: "~a~ió", "~a~ieron";
    fut_1: "~a~iré", "~a~iremos";
    fut_2: "~a~irás", "~a~iréis";
    fut_3: "~a~irá", "~a~irán";
    cond_1: "~a~iría", "~a~iríamos";
    cond_2: "~a~irías", "~a~iríais";
    cond_3: _, "~a~irían";
    subj_1: "~a~a", "~a~amos";
    subj_2: "~a~as", "~a~áis";
    subj_3: _, "~a~an";
    subj_impf_1: "~a~iera", "~a~iéramos";
    subj_impf_2: "~a~ieras", "~a~ierais";
    subj_impf_3: _, "~a~ieran";
    subj_impf_se_1: "~a~iese", "~a~iésemos";
    subj_impf_se_2: "~a~ieses", "~a~ieseis";
    subj_impf_se_3: _, "~a~iesen";
    subj_fut_1: "~a~iere", "~a~iéremos";
    subj_fut_2: "~a~ieres", "~a~iereis";
    subj_fut_3: _, "~a~ieren";
    vos: "~a~ís", "~a~ás", "~a~ite";
    imperative: "~á~ete", "~á~ase", "~a~ámonos", "~a~íos", "~á~anse";
    infinitive: "~a~irse";
    gerund: "~a~iéndose";
    ),
    // Fruncir: 19 words (zurcir, uncir, surcir, esparcir, resarcir, balbucir). Like Reducir, except in 30 cells: pres_1 sg, pret_1 sg, pret_2 sg, pret_2 pl, pret_3 sg, pret_3 pl, ….
    conjugation!("fruncir", 19, "~cir", like Reducir as PRET_AS_PRES;
    pres_1: "~zo", _;
    pret_1: "~cí", _;
    pret_2: "~ciste", "~cisteis";
    pret_3: "~ció", "~cieron";
    subj_1: "~za", "~zamos";
    subj_2: "~zas", "~záis";
    subj_3: _, "~zan";
    subj_impf_1: "~ciera", "~ciéramos";
    subj_impf_2: "~cieras", "~cierais";
    subj_impf_3: _, "~cieran";
    subj_impf_se_1: "~ciese", "~ciésemos";
    subj_impf_se_2: "~cieses", "~cieseis";
    subj_impf_se_3: _, "~ciesen";
    subj_fut_1: "~ciere", "~ciéremos";
    subj_fut_2: "~cieres", "~ciereis";
    subj_fut_3: _, "~cieren";
    vos: _, "~zás", _;
    imperative: _, "~za", "~zamos", _, "~zan";
    ),
    // Chuzarse: 19 words (chapuzarse, abruzarse, abuzarse, escagarruzarse, aliruzarse, atapuzarse).
    conjugation!("chuzarse", 19, "~uzarse";
    stem participle: "~uzad- like echar";
    pres_1: "~uzo", "~uzamos";
    pres_2: "~uzas", "~uzáis";
    pres_3: "~uza", "~uzan";
    impf_1: "~uzaba", "~uzábamos";
    impf_2: "~uzabas", "~uzabais";
    impf_3: _, "~uzaban";
    pret_1: "~ucé", "~uzamos";
    pret_2: "~uzaste", "~uzasteis";
    pret_3: "~uzó", "~uzaron";
    fut_1: "~uzaré", "~uzaremos";
    fut_2: "~uzarás", "~uzaréis";
    fut_3: "~uzará", "~uzarán";
    cond_1: "~uzaría", "~uzaríamos";
    cond_2: "~uzarías", "~uzaríais";
    cond_3: _, "~uzarían";
    subj_1: "~uce", "~ucemos";
    subj_2: "~uces", "~ucéis";
    subj_3: _, "~ucen";
    subj_impf_1: "~uzara", "~uzáramos";
    subj_impf_2: "~uzaras", "~uzarais";
    subj_impf_3: _, "~uzaran";
    subj_impf_se_1: "~uzase", "~uzásemos";
    subj_impf_se_2: "~uzases", "~uzaseis";
    subj_impf_se_3: _, "~uzasen";
    subj_fut_1: "~uzare", "~uzáremos";
    subj_fut_2: "~uzares", "~uzareis";
    subj_fut_3: _, "~uzaren";
    vos: "~uzás", "~ucés", "~uzate";
    imperative: "~úzate", "~úcese", "~ucémonos", "~uzaos", "~úcense";
    infinitive: "~uzarse";
    gerund: "~uzándose";
    ),
    // Resentirse: 19 words (resentirse, arrepentirse, repentirse, divertirse, sentirse, referirse). Like Sentir, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("resentirse", 19, "~e~irse", like Sentir;
    vos: _, _, "~e~ite";
    imperative: "~ié~ete", "~ié~ase", "~i~ámonos", "~e~íos", "~ié~anse";
    infinitive: "~e~irse";
    gerund: "~i~iéndose";
    ),
    // Calzarse: 18 words (abalanzarse, ensalzarse, esperanzarse, alianzarse, alcanzarse, desalzarse).
    conjugation!("calzarse", 18, "~a~zarse";
    stem participle: "~a~zad- like echar";
    pres_1: "~a~zo", "~a~zamos";
    pres_2: "~a~zas", "~a~záis";
    pres_3: "~a~za", "~a~zan";
    impf_1: "~a~zaba", "~a~zábamos";
    impf_2: "~a~zabas", "~a~zabais";
    impf_3: _, "~a~zaban";
    pret_1: "~a~cé", "~a~zamos";
    pret_2: "~a~zaste", "~a~zasteis";
    pret_3: "~a~zó", "~a~zaron";
    fut_1: "~a~zaré", "~a~zaremos";
    fut_2: "~a~zarás", "~a~zaréis";
    fut_3: "~a~zará", "~a~zarán";
    cond_1: "~a~zaría", "~a~zaríamos";
    cond_2: "~a~zarías", "~a~zaríais";
    cond_3: _, "~a~zarían";
    subj_1: "~a~ce", "~a~cemos";
    subj_2: "~a~ces", "~a~céis";
    subj_3: _, "~a~cen";
    subj_impf_1: "~a~zara", "~a~záramos";
    subj_impf_2: "~a~zaras", "~a~zarais";
    subj_impf_3: _, "~a~zaran";
    subj_impf_se_1: "~a~zase", "~a~zásemos";
    subj_impf_se_2: "~a~zases", "~a~zaseis";
    subj_impf_se_3: _, "~a~zasen";
    subj_fut_1: "~a~zare", "~a~záremos";
    subj_fut_2: "~a~zares", "~a~zareis";
    subj_fut_3: _, "~a~zaren";
    vos: "~a~zás", "~a~cés", "~a~zate";
    imperative: "~á~zate", "~á~cese", "~a~cémonos", "~a~zaos", "~á~cense";
    infinitive: "~a~zarse";
    gerund: "~a~zándose";
    ),
    // Desentenderse: 17 words (desentenderse, autodefenderse, perderse, defenderse, extenderse, sobrentenderse). Like Entremeterse, except in 10 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("desentenderse", 17, "~e~erse", like Entremeterse;
    pres_1: "~ie~o", _;
    pres_2: "~ie~es", _;
    pres_3: "~ie~e", "~ie~en";
    subj_1: "~ie~a", _;
    subj_2: "~ie~as", _;
    subj_3: _, "~ie~an";
    imperative: "~ié~ete", "~ié~ase", _, _, "~ié~anse";
    ),
    // Creer: 17 words (peer, creer, leer, veer, seer, roer). Like Meter, except in 21 cells: pret_1 pl, pret_2 sg, pret_2 pl, pret_3 sg, pret_3 pl, subj_impf_1 sg, ….
    conjugation!("creer", 17, "~er", like Meter;
    stem participle: "~íd- like echar";
    pret_1: _, "~ímos";
    pret_2: "~íste", "~ísteis";
    pret_3: "~yó", "~yeron";
    subj_impf_1: "~yera", "~yéramos";
    subj_impf_2: "~yeras", "~yerais";
    subj_impf_3: _, "~yeran";
    subj_impf_se_1: "~yese", "~yésemos";
    subj_impf_se_2: "~yeses", "~yeseis";
    subj_impf_se_3: _, "~yesen";
    subj_fut_1: "~yere", "~yéremos";
    subj_fut_2: "~yeres", "~yereis";
    subj_fut_3: _, "~yeren";
    gerund: "~yendo";
    ),
    // Traer: 16 words (traer, contraer, atraer, extraer, substraer, abstraer). Like Creer, except in 31 cells: pres_1 sg, pret_1 sg, pret_1 pl, pret_2 sg, pret_2 pl, pret_3 sg, ….
    conjugation!("traer", 16, "~er", like Creer;
    pres_1: "~igo", _;
    pret_1: "~je", "~jimos";
    pret_2: "~jiste", "~jisteis";
    pret_3: "~jo", "~jeron";
    subj_1: "~iga", "~igamos";
    subj_2: "~igas", "~igáis";
    subj_3: _, "~igan";
    subj_impf_1: "~jera", "~jéramos";
    subj_impf_2: "~jeras", "~jerais";
    subj_impf_3: _, "~jeran";
    subj_impf_se_1: "~jese", "~jésemos";
    subj_impf_se_2: "~jeses", "~jeseis";
    subj_impf_se_3: _, "~jesen";
    subj_fut_1: "~jere", "~jéremos";
    subj_fut_2: "~jeres", "~jereis";
    subj_fut_3: _, "~jeren";
    vos: _, "~igás", _;
    imperative: _, "~iga", "~igamos", _, "~igan";
    ),
    // Entecarse: 16 words (entecarse, enchuecarse, obcecarse, intersecarse, ahuecarse, deprecarse).
    conjugation!("entecarse", 16, "~~carse";
    stem participle: "~~cad- like echar";
    pres_1: "~~co", "~~camos";
    pres_2: "~~cas", "~~cáis";
    pres_3: "~~ca", "~~can";
    impf_1: "~~caba", "~~cábamos";
    impf_2: "~~cabas", "~~cabais";
    impf_3: _, "~~caban";
    pret_1: "~~qué", "~~camos";
    pret_2: "~~caste", "~~casteis";
    pret_3: "~~có", "~~caron";
    fut_1: "~~caré", "~~caremos";
    fut_2: "~~carás", "~~caréis";
    fut_3: "~~cará", "~~carán";
    cond_1: "~~caría", "~~caríamos";
    cond_2: "~~carías", "~~caríais";
    cond_3: _, "~~carían";
    subj_1: "~~que", "~~quemos";
    subj_2: "~~ques", "~~quéis";
    subj_3: _, "~~quen";
    subj_impf_1: "~~cara", "~~cáramos";
    subj_impf_2: "~~caras", "~~carais";
    subj_impf_3: _, "~~caran";
    subj_impf_se_1: "~~case", "~~cásemos";
    subj_impf_se_2: "~~cases", "~~caseis";
    subj_impf_se_3: _, "~~casen";
    subj_fut_1: "~~care", "~~cáremos";
    subj_fut_2: "~~cares", "~~careis";
    subj_fut_3: _, "~~caren";
    vos: "~~cás", "~~qués", "~~cate";
    imperative: "~écat~", "~équ~se", "~~quémonos", "~~caos", "~équ~nse";
    infinitive: "~~carse";
    gerund: "~~cándose";
    ),
    // Aborrascarse: 14 words (aborrascarse, atascarse, rascarse, achubascarse, enfrascarse, emborrascarse).
    conjugation!("aborrascarse", 14, "~a~carse";
    stem participle: "~a~cad- like echar";
    pres_1: "~a~co", "~a~camos";
    pres_2: "~a~cas", "~a~cáis";
    pres_3: "~a~ca", "~a~can";
    impf_1: "~a~caba", "~a~cábamos";
    impf_2: "~a~cabas", "~a~cabais";
    impf_3: _, "~a~caban";
    pret_1: "~a~qué", "~a~camos";
    pret_2: "~a~caste", "~a~casteis";
    pret_3: "~a~có", "~a~caron";
    fut_1: "~a~caré", "~a~caremos";
    fut_2: "~a~carás", "~a~caréis";
    fut_3: "~a~cará", "~a~carán";
    cond_1: "~a~caría", "~a~caríamos";
    cond_2: "~a~carías", "~a~caríais";
    cond_3: _, "~a~carían";
    subj_1: "~a~que", "~a~quemos";
    subj_2: "~a~ques", "~a~quéis";
    subj_3: _, "~a~quen";
    subj_impf_1: "~a~cara", "~a~cáramos";
    subj_impf_2: "~a~caras", "~a~carais";
    subj_impf_3: _, "~a~caran";
    subj_impf_se_1: "~a~case", "~a~cásemos";
    subj_impf_se_2: "~a~cases", "~a~caseis";
    subj_impf_se_3: _, "~a~casen";
    subj_fut_1: "~a~care", "~a~cáremos";
    subj_fut_2: "~a~cares", "~a~careis";
    subj_fut_3: _, "~a~caren";
    vos: "~a~cás", "~a~qués", "~a~cate";
    imperative: "~á~cate", "~á~quese", "~a~quémonos", "~a~caos", "~á~quense";
    infinitive: "~a~carse";
    gerund: "~a~cándose";
    ),
    // Revolver: 14 words (absolver, revolver, resolver, disolver, volver, solver).
    conjugation!("revolver", 14, "~o~ver";
    stem participle: "~ue~t- like echar";
    pres_1: "~ue~vo", "~o~vemos";
    pres_2: "~ue~ves", "~o~véis";
    pres_3: "~ue~ve", "~ue~ven";
    impf_1: "~o~vía", "~o~víamos";
    impf_2: "~o~vías", "~o~víais";
    impf_3: _, "~o~vían";
    pret_1: "~o~ví", "~o~vimos";
    pret_2: "~o~viste", "~o~visteis";
    pret_3: "~o~vió", "~o~vieron";
    fut_1: "~o~veré", "~o~veremos";
    fut_2: "~o~verás", "~o~veréis";
    fut_3: "~o~verá", "~o~verán";
    cond_1: "~o~vería", "~o~veríamos";
    cond_2: "~o~verías", "~o~veríais";
    cond_3: _, "~o~verían";
    subj_1: "~ue~va", "~o~vamos";
    subj_2: "~ue~vas", "~o~váis";
    subj_3: _, "~ue~van";
    subj_impf_1: "~o~viera", "~o~viéramos";
    subj_impf_2: "~o~vieras", "~o~vierais";
    subj_impf_3: _, "~o~vieran";
    subj_impf_se_1: "~o~viese", "~o~viésemos";
    subj_impf_se_2: "~o~vieses", "~o~vieseis";
    subj_impf_se_3: _, "~o~viesen";
    subj_fut_1: "~o~viere", "~o~viéremos";
    subj_fut_2: "~o~vieres", "~o~viereis";
    subj_fut_3: _, "~o~vieren";
    vos: "~o~vés", "~o~vás", "~o~vé";
    imperative: "~ue~ve", "~ue~va", "~o~vamos", "~o~ved", "~ue~van";
    infinitive: "~o~ver";
    gerund: "~o~viendo";
    ),
    // Rozarse: 14 words (apozarse, escozarse, arrebozarse, empozarse, alborozarse, desarrebozarse).
    conjugation!("rozarse", 14, "~ozarse";
    stem participle: "~ozad- like echar";
    pres_1: "~ozo", "~ozamos";
    pres_2: "~ozas", "~ozáis";
    pres_3: "~oza", "~ozan";
    impf_1: "~ozaba", "~ozábamos";
    impf_2: "~ozabas", "~ozabais";
    impf_3: _, "~ozaban";
    pret_1: "~océ", "~ozamos";
    pret_2: "~ozaste", "~ozasteis";
    pret_3: "~ozó", "~ozaron";
    fut_1: "~ozaré", "~ozaremos";
    fut_2: "~ozarás", "~ozaréis";
    fut_3: "~ozará", "~ozarán";
    cond_1: "~ozaría", "~ozaríamos";
    cond_2: "~ozarías", "~ozaríais";
    cond_3: _, "~ozarían";
    subj_1: "~oce", "~ocemos";
    subj_2: "~oces", "~océis";
    subj_3: _, "~ocen";
    subj_impf_1: "~ozara", "~ozáramos";
    subj_impf_2: "~ozaras", "~ozarais";
    subj_impf_3: _, "~ozaran";
    subj_impf_se_1: "~ozase", "~ozásemos";
    subj_impf_se_2: "~ozases", "~ozaseis";
    subj_impf_se_3: _, "~ozasen";
    subj_fut_1: "~ozare", "~ozáremos";
    subj_fut_2: "~ozares", "~ozareis";
    subj_fut_3: _, "~ozaren";
    vos: "~ozás", "~océs", "~ozate";
    imperative: "~ózate", "~ócese", "~océmonos", "~ozaos", "~ócense";
    infinitive: "~ozarse";
    gerund: "~ozándose";
    ),
    // Vencer: 13 words (coercer, vencer, ejercer, convencer, mecer, mulcer). Like Fruncir, except in 17 cells: pres_1 pl, pres_2 pl, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, ….
    conjugation!("vencer", 13, "~cer", like Fruncir;
    pres_1: _, "~cemos";
    pres_2: _, "~céis";
    fut_1: "~ceré", "~ceremos";
    fut_2: "~cerás", "~ceréis";
    fut_3: "~cerá", "~cerán";
    cond_1: "~cería", "~ceríamos";
    cond_2: "~cerías", "~ceríais";
    cond_3: _, "~cerían";
    vos: "~cés", _, "~cé";
    imperative: _, _, _, "~ced", _;
    infinitive: "~cer";
    ),
    // Tener: 13 words (tener, sostener, obtener, detener, mantener, retener).
    conjugation!("tener", 13, "~ener";
    stem participle: "~enid- like echar";
    pres_1: "~engo", "~enemos";
    pres_2: "~ienes", "~enéis";
    pres_3: "~iene", "~ienen";
    impf_1: "~enía", "~eníamos";
    impf_2: "~enías", "~eníais";
    impf_3: _, "~enían";
    pret_1: "~uve", "~uvimos";
    pret_2: "~uviste", "~uvisteis";
    pret_3: "~uvo", "~uvieron";
    fut_1: "~endré", "~endremos";
    fut_2: "~endrás", "~endréis";
    fut_3: "~endrá", "~endrán";
    cond_1: "~endría", "~endríamos";
    cond_2: "~endrías", "~endríais";
    cond_3: _, "~endrían";
    subj_1: "~enga", "~engamos";
    subj_2: "~engas", "~engáis";
    subj_3: _, "~engan";
    subj_impf_1: "~uviera", "~uviéramos";
    subj_impf_2: "~uvieras", "~uvierais";
    subj_impf_3: _, "~uvieran";
    subj_impf_se_1: "~uviese", "~uviésemos";
    subj_impf_se_2: "~uvieses", "~uvieseis";
    subj_impf_se_3: _, "~uviesen";
    subj_fut_1: "~uviere", "~uviéremos";
    subj_fut_2: "~uvieres", "~uviereis";
    subj_fut_3: _, "~uvieren";
    vos: "~enés", "~engás", "~ené";
    imperative: "~én", "~enga", "~engamos", "~ened", "~engan";
    infinitive: "~ener";
    gerund: "~eniendo";
    ),
    // Ceñir: 13 words (teñir, reñir, ceñir, desteñir, estreñir, astreñir).
    conjugation!("ceñir", 13, "~eñir";
    stem participle: "~eñid- like echar";
    pres_1: "~iño", "~eñimos";
    pres_2: "~iñes", "~eñís";
    pres_3: "~iñe", "~iñen";
    impf_1: "~eñía", "~eñíamos";
    impf_2: "~eñías", "~eñíais";
    impf_3: _, "~eñían";
    pret_1: "~eñí", "~eñimos";
    pret_2: "~eñiste", "~eñisteis";
    pret_3: "~iñó", "~iñeron";
    fut_1: "~eñiré", "~eñiremos";
    fut_2: "~eñirás", "~eñiréis";
    fut_3: "~eñirá", "~eñirán";
    cond_1: "~eñiría", "~eñiríamos";
    cond_2: "~eñirías", "~eñiríais";
    cond_3: _, "~eñirían";
    subj_1: "~iña", "~iñamos";
    subj_2: "~iñas", "~iñáis";
    subj_3: _, "~iñan";
    subj_impf_1: "~iñera", "~iñéramos";
    subj_impf_2: "~iñeras", "~iñerais";
    subj_impf_3: _, "~iñeran";
    subj_impf_se_1: "~iñese", "~iñésemos";
    subj_impf_se_2: "~iñeses", "~iñeseis";
    subj_impf_se_3: _, "~iñesen";
    subj_fut_1: "~iñere", "~iñéremos";
    subj_fut_2: "~iñeres", "~iñereis";
    subj_fut_3: _, "~iñeren";
    vos: "~eñís", "~iñás", "~eñí";
    imperative: "~iñe", "~iña", "~iñamos", "~eñid", "~iñan";
    infinitive: "~eñir";
    gerund: "~iñendo";
    ),
    // Enfoscarse: 12 words (enfoscarse, afoscarse, coscarse, desemboscarse, enfroscarse, trasroscarse).
    conjugation!("enfoscarse", 12, "~o~carse";
    stem participle: "~o~cad- like echar";
    pres_1: "~o~co", "~o~camos";
    pres_2: "~o~cas", "~o~cáis";
    pres_3: "~o~ca", "~o~can";
    impf_1: "~o~caba", "~o~cábamos";
    impf_2: "~o~cabas", "~o~cabais";
    impf_3: _, "~o~caban";
    pret_1: "~o~qué", "~o~camos";
    pret_2: "~o~caste", "~o~casteis";
    pret_3: "~o~có", "~o~caron";
    fut_1: "~o~caré", "~o~caremos";
    fut_2: "~o~carás", "~o~caréis";
    fut_3: "~o~cará", "~o~carán";
    cond_1: "~o~caría", "~o~caríamos";
    cond_2: "~o~carías", "~o~caríais";
    cond_3: _, "~o~carían";
    subj_1: "~o~que", "~o~quemos";
    subj_2: "~o~ques", "~o~quéis";
    subj_3: _, "~o~quen";
    subj_impf_1: "~o~cara", "~o~cáramos";
    subj_impf_2: "~o~caras", "~o~carais";
    subj_impf_3: _, "~o~caran";
    subj_impf_se_1: "~o~case", "~o~cásemos";
    subj_impf_se_2: "~o~cases", "~o~caseis";
    subj_impf_se_3: _, "~o~casen";
    subj_fut_1: "~o~care", "~o~cáremos";
    subj_fut_2: "~o~cares", "~o~careis";
    subj_fut_3: _, "~o~caren";
    vos: "~o~cás", "~o~qués", "~o~cate";
    imperative: "~ó~cate", "~ó~quese", "~o~quémonos", "~o~caos", "~ó~quense";
    infinitive: "~o~carse";
    gerund: "~o~cándose";
    ),
    // Enraizar: 12 words (enraizar, europeizar, arcaizar, descorreizar, judaizar, raizar). Like Fosilizarse, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("enraizar", 12, "~izar", like Fosilizarse;
    pres_1: "~ízo", _;
    pres_2: "~ízas", _;
    pres_3: "~íza", "~ízan";
    subj_1: "~íce", _;
    subj_2: "~íces", _;
    subj_3: _, "~ícen";
    vos: _, _, "~izá";
    imperative: "~íza", "~íce", "~icemos", "~izad", "~ícen";
    infinitive: "~izar";
    gerund: "~izando";
    ),
    // Ahilarse: 12 words (autoaislarse, aislarse, ahitarse, ahilarse, airarse, desaislarse). Like Rilarse, except in 7 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("ahilarse", 12, "~i~arse", like Rilarse;
    pres_1: "~í~o", _;
    pres_2: "~í~as", _;
    pres_3: "~í~a", "~í~an";
    subj_1: "~í~e", _;
    subj_2: "~í~es", _;
    subj_3: _, "~í~en";
    ),
    // Armarse: 12 words (abarse, armarse, asmarse, alarse, andarse, asparse).
    conjugation!("armarse", 12, "a~arse";
    stem participle: "~ad- like armarse";
    pres_1: "a~o", "a~amos";
    pres_2: "a~as", "a~áis";
    pres_3: "a~a", "a~an";
    impf_1: "a~aba", "a~ábamos";
    impf_2: "a~abas", "a~abais";
    impf_3: _, "a~aban";
    pret_1: "a~é", "a~amos";
    pret_2: "a~aste", "a~asteis";
    pret_3: "a~ó", "a~aron";
    fut_1: "a~aré", "a~aremos";
    fut_2: "a~arás", "a~aréis";
    fut_3: "a~ará", "a~arán";
    cond_1: "a~aría", "a~aríamos";
    cond_2: "a~arías", "a~aríais";
    cond_3: _, "a~arían";
    subj_1: "a~e", "a~emos";
    subj_2: "a~es", "a~éis";
    subj_3: _, "a~en";
    subj_impf_1: "a~ara", "a~áramos";
    subj_impf_2: "a~aras", "a~arais";
    subj_impf_3: _, "a~aran";
    subj_impf_se_1: "a~ase", "a~ásemos";
    subj_impf_se_2: "a~ases", "a~aseis";
    subj_impf_se_3: _, "a~asen";
    subj_fut_1: "a~are", "a~áremos";
    subj_fut_2: "a~ares", "a~areis";
    subj_fut_3: _, "a~aren";
    vos: "a~ás", "a~és", "a~ate";
    imperative: "á~ate", "á~ese", "a~émonos", "a~aos", "á~ense";
    infinitive: "a~arse";
    gerund: "a~ándose";
    ),
    // Hacer: 12 words (hacer, satisfacer, facer, deshacer, rehacer, contrahacer).
    conjugation!("hacer", 12, "~acer";
    stem participle: "~ech- like echar";
    pres_1: "~ago", "~acemos";
    pres_2: "~aces", "~acéis";
    pres_3: "~ace", "~acen";
    impf_1: "~acía", "~acíamos";
    impf_2: "~acías", "~acíais";
    impf_3: _, "~acían";
    pret_1: "~ice", "~icimos";
    pret_2: "~iciste", "~icisteis";
    pret_3: "~izo", "~icieron";
    fut_1: "~aré", "~aremos";
    fut_2: "~arás", "~aréis";
    fut_3: "~ará", "~arán";
    cond_1: "~aría", "~aríamos";
    cond_2: "~arías", "~aríais";
    cond_3: _, "~arían";
    subj_1: "~aga", "~agamos";
    subj_2: "~agas", "~agáis";
    subj_3: _, "~agan";
    subj_impf_1: "~iciera", "~iciéramos";
    subj_impf_2: "~icieras", "~icierais";
    subj_impf_3: _, "~icieran";
    subj_impf_se_1: "~iciese", "~iciésemos";
    subj_impf_se_2: "~icieses", "~icieseis";
    subj_impf_se_3: _, "~iciesen";
    subj_fut_1: "~iciere", "~iciéremos";
    subj_fut_2: "~icieres", "~iciereis";
    subj_fut_3: _, "~icieren";
    vos: "~acés", "~agás", "~acé";
    imperative: "~az", "~aga", "~agamos", "~aced", "~agan";
    infinitive: "~acer";
    gerund: "~aciendo";
    ),
    // Empezar: 11 words (empezar, comenzar, tropezar, despezar, enlenzar, entropezar). Like Alzarse, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("empezar", 11, "~~zar", like Alzarse;
    pres_1: "~i~zo", _;
    pres_2: "~i~zas", _;
    pres_3: "~i~za", "~i~zan";
    subj_1: "~i~ce", _;
    subj_2: "~i~ces", _;
    subj_3: _, "~i~cen";
    vos: _, _, "~~zá";
    imperative: "~i~za", "~i~ce", "~~cemos", "~~zad", "~i~cen";
    infinitive: "~~zar";
    gerund: "~~zando";
    ),
    // Escabullirse: 11 words (escabullirse, zambullirse, rebullirse, tullirse, escullirse, descabullirse). Like Musirse, except in 18 cells: pret_3 sg, pret_3 pl, subj_impf_1 sg, subj_impf_1 pl, subj_impf_2 sg, subj_impf_2 pl, ….
    conjugation!("escabullirse", 11, "~u~irse", like Musirse;
    pret_3: "~u~ó", "~u~eron";
    subj_impf_1: "~u~era", "~u~éramos";
    subj_impf_2: "~u~eras", "~u~erais";
    subj_impf_3: _, "~u~eran";
    subj_impf_se_1: "~u~ese", "~u~ésemos";
    subj_impf_se_2: "~u~eses", "~u~eseis";
    subj_impf_se_3: _, "~u~esen";
    subj_fut_1: "~u~ere", "~u~éremos";
    subj_fut_2: "~u~eres", "~u~ereis";
    subj_fut_3: _, "~u~eren";
    gerund: "~u~éndose";
    ),
    // Apanucarse: 11 words (acurrucarse, acorrucarse, embejucarse, apitucarse, emborucarse, desnucarse).
    conjugation!("apanucarse", 11, "~ucarse";
    stem participle: "~ucad- like echar";
    pres_1: "~uco", "~ucamos";
    pres_2: "~ucas", "~ucáis";
    pres_3: "~uca", "~ucan";
    impf_1: "~ucaba", "~ucábamos";
    impf_2: "~ucabas", "~ucabais";
    impf_3: _, "~ucaban";
    pret_1: "~uqué", "~ucamos";
    pret_2: "~ucaste", "~ucasteis";
    pret_3: "~ucó", "~ucaron";
    fut_1: "~ucaré", "~ucaremos";
    fut_2: "~ucarás", "~ucaréis";
    fut_3: "~ucará", "~ucarán";
    cond_1: "~ucaría", "~ucaríamos";
    cond_2: "~ucarías", "~ucaríais";
    cond_3: _, "~ucarían";
    subj_1: "~uque", "~uquemos";
    subj_2: "~uques", "~uquéis";
    subj_3: _, "~uquen";
    subj_impf_1: "~ucara", "~ucáramos";
    subj_impf_2: "~ucaras", "~ucarais";
    subj_impf_3: _, "~ucaran";
    subj_impf_se_1: "~ucase", "~ucásemos";
    subj_impf_se_2: "~ucases", "~ucaseis";
    subj_impf_se_3: _, "~ucasen";
    subj_fut_1: "~ucare", "~ucáremos";
    subj_fut_2: "~ucares", "~ucareis";
    subj_fut_3: _, "~ucaren";
    vos: "~ucás", "~uqués", "~ucate";
    imperative: "~úcate", "~úquese", "~uquémonos", "~ucaos", "~úquense";
    infinitive: "~ucarse";
    gerund: "~ucándose";
    ),
    // Suscribirse: 10 words (autodescribirse, escribirse, adscribirse, inscribirse, subscribirse, suscribirse).
    conjugation!("suscribirse", 10, "~ibirse";
    stem participle: "~i- like escribir";
    pres_1: "~ibo", "~ibimos";
    pres_2: "~ibes", "~ibís";
    pres_3: "~ibe", "~iben";
    impf_1: "~ibía", "~ibíamos";
    impf_2: "~ibías", "~ibíais";
    impf_3: _, "~ibían";
    pret_1: "~ibí", "~ibimos";
    pret_2: "~ibiste", "~ibisteis";
    pret_3: "~ibió", "~ibieron";
    fut_1: "~ibiré", "~ibiremos";
    fut_2: "~ibirás", "~ibiréis";
    fut_3: "~ibirá", "~ibirán";
    cond_1: "~ibiría", "~ibiríamos";
    cond_2: "~ibirías", "~ibiríais";
    cond_3: _, "~ibirían";
    subj_1: "~iba", "~ibamos";
    subj_2: "~ibas", "~ibáis";
    subj_3: _, "~iban";
    subj_impf_1: "~ibiera", "~ibiéramos";
    subj_impf_2: "~ibieras", "~ibierais";
    subj_impf_3: _, "~ibieran";
    subj_impf_se_1: "~ibiese", "~ibiésemos";
    subj_impf_se_2: "~ibieses", "~ibieseis";
    subj_impf_se_3: _, "~ibiesen";
    subj_fut_1: "~ibiere", "~ibiéremos";
    subj_fut_2: "~ibieres", "~ibiereis";
    subj_fut_3: _, "~ibieren";
    vos: "~ibís", "~ibás", "~ibite";
    imperative: "~íbete", "~íbase", "~ibámonos", "~ibíos", "~íbanse";
    infinitive: "~ibirse";
    gerund: "~ibiéndose";
    ),
    // Retraerse: 10 words (contraerse, traerse, atraerse, sustraerse, retrotraerse, retraerse).
    conjugation!("retraerse", 10, "~aerse";
    stem participle: "~aíd- like echar";
    pres_1: "~aigo", "~aemos";
    pres_2: "~aes", "~aéis";
    pres_3: "~ae", "~aen";
    impf_1: "~aía", "~aíamos";
    impf_2: "~aías", "~aíais";
    impf_3: _, "~aían";
    pret_1: "~aje", "~ajimos";
    pret_2: "~ajiste", "~ajisteis";
    pret_3: "~ajo", "~ajeron";
    fut_1: "~aeré", "~aeremos";
    fut_2: "~aerás", "~aeréis";
    fut_3: "~aerá", "~aerán";
    cond_1: "~aería", "~aeríamos";
    cond_2: "~aerías", "~aeríais";
    cond_3: _, "~aerían";
    subj_1: "~aiga", "~aigamos";
    subj_2: "~aigas", "~aigáis";
    subj_3: _, "~aigan";
    subj_impf_1: "~ajera", "~ajéramos";
    subj_impf_2: "~ajeras", "~ajerais";
    subj_impf_3: _, "~ajeran";
    subj_impf_se_1: "~ajese", "~ajésemos";
    subj_impf_se_2: "~ajeses", "~ajeseis";
    subj_impf_se_3: _, "~ajesen";
    subj_fut_1: "~ajere", "~ajéremos";
    subj_fut_2: "~ajeres", "~ajereis";
    subj_fut_3: _, "~ajeren";
    vos: "~aés", "~aigás", "~aete";
    imperative: "~áete", "~áigase", "~aigámonos", "~aeos", "~áiganse";
    infinitive: "~aerse";
    gerund: "~ayéndose";
    ),
    // Lucir: 10 words (lucir, relucir, deslucir, enlucir, hendir, traslucir). Like Conocer, except in 17 cells: pres_1 pl, pres_2 pl, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, ….
    conjugation!("lucir", 10, "~~ir", like Conocer;
    pres_1: _, "~~imos";
    pres_2: _, "~~ís";
    fut_1: "~~iré", "~~iremos";
    fut_2: "~~irás", "~~iréis";
    fut_3: "~~irá", "~~irán";
    cond_1: "~~iría", "~~iríamos";
    cond_2: "~~irías", "~~iríais";
    cond_3: _, "~~irían";
    vos: "~~ís", _, "~~í";
    imperative: _, _, _, "~~id", _;
    infinitive: "~~ir";
    ),
    // Enamoriscarse: 9 words (enamoriscarse, ciscarse, ariscarse, enriscarse, apriscarse, arriscarse).
    conjugation!("enamoriscarse", 9, "~i~carse";
    stem participle: "~i~cad- like echar";
    pres_1: "~i~co", "~i~camos";
    pres_2: "~i~cas", "~i~cáis";
    pres_3: "~i~ca", "~i~can";
    impf_1: "~i~caba", "~i~cábamos";
    impf_2: "~i~cabas", "~i~cabais";
    impf_3: _, "~i~caban";
    pret_1: "~i~qué", "~i~camos";
    pret_2: "~i~caste", "~i~casteis";
    pret_3: "~i~có", "~i~caron";
    fut_1: "~i~caré", "~i~caremos";
    fut_2: "~i~carás", "~i~caréis";
    fut_3: "~i~cará", "~i~carán";
    cond_1: "~i~caría", "~i~caríamos";
    cond_2: "~i~carías", "~i~caríais";
    cond_3: _, "~i~carían";
    subj_1: "~i~que", "~i~quemos";
    subj_2: "~i~ques", "~i~quéis";
    subj_3: _, "~i~quen";
    subj_impf_1: "~i~cara", "~i~cáramos";
    subj_impf_2: "~i~caras", "~i~carais";
    subj_impf_3: _, "~i~caran";
    subj_impf_se_1: "~i~case", "~i~cásemos";
    subj_impf_se_2: "~i~cases", "~i~caseis";
    subj_impf_se_3: _, "~i~casen";
    subj_fut_1: "~i~care", "~i~cáremos";
    subj_fut_2: "~i~cares", "~i~careis";
    subj_fut_3: _, "~i~caren";
    vos: "~i~cás", "~i~qués", "~i~cate";
    imperative: "~í~cate", "~í~quese", "~i~quémonos", "~i~caos", "~í~quense";
    infinitive: "~i~carse";
    gerund: "~i~cándose";
    ),
    // Apuñuscarse: 9 words (apuñuscarse, buscarse, rebuscarse, zurruscarse, enfurruscarse, apeñuscarse).
    conjugation!("apuñuscarse", 9, "~u~carse";
    stem participle: "~u~cad- like echar";
    pres_1: "~u~co", "~u~camos";
    pres_2: "~u~cas", "~u~cáis";
    pres_3: "~u~ca", "~u~can";
    impf_1: "~u~caba", "~u~cábamos";
    impf_2: "~u~cabas", "~u~cabais";
    impf_3: _, "~u~caban";
    pret_1: "~u~qué", "~u~camos";
    pret_2: "~u~caste", "~u~casteis";
    pret_3: "~u~có", "~u~caron";
    fut_1: "~u~caré", "~u~caremos";
    fut_2: "~u~carás", "~u~caréis";
    fut_3: "~u~cará", "~u~carán";
    cond_1: "~u~caría", "~u~caríamos";
    cond_2: "~u~carías", "~u~caríais";
    cond_3: _, "~u~carían";
    subj_1: "~u~que", "~u~quemos";
    subj_2: "~u~ques", "~u~quéis";
    subj_3: _, "~u~quen";
    subj_impf_1: "~u~cara", "~u~cáramos";
    subj_impf_2: "~u~caras", "~u~carais";
    subj_impf_3: _, "~u~caran";
    subj_impf_se_1: "~u~case", "~u~cásemos";
    subj_impf_se_2: "~u~cases", "~u~caseis";
    subj_impf_se_3: _, "~u~casen";
    subj_fut_1: "~u~care", "~u~cáremos";
    subj_fut_2: "~u~cares", "~u~careis";
    subj_fut_3: _, "~u~caren";
    vos: "~u~cás", "~u~qués", "~u~cate";
    imperative: "~ú~cate", "~ú~quese", "~u~quémonos", "~u~caos", "~ú~quense";
    infinitive: "~u~carse";
    gerund: "~u~cándose";
    ),
    // Abrir: 9 words (cubrir, abrir, reabrir, descubrir, encubrir, entreabrir). Like Lucir, except in 10 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("abrir", 9, "~~ir", like Lucir;
    stem participle: "~ie~t- like echar";
    pres_1: "~~o", _;
    subj_1: "~~a", "~~amos";
    subj_2: "~~as", "~~áis";
    subj_3: _, "~~an";
    vos: _, "~~ás", _;
    imperative: _, "~~a", "~~amos", _, "~~an";
    ),
    // Trocar: 9 words (trocar, trastrocar, clocar, retrocar, destrocar, aclocar). Like Descocarse, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("trocar", 9, "~ocar", like Descocarse;
    pres_1: "~ueco", _;
    pres_2: "~uecas", _;
    pres_3: "~ueca", "~uecan";
    subj_1: "~ueque", _;
    subj_2: "~ueques", _;
    subj_3: _, "~uequen";
    vos: _, _, "~ocá";
    imperative: "~ueca", "~ueque", "~oquemos", "~ocad", "~uequen";
    infinitive: "~ocar";
    gerund: "~ocando";
    ),
    // Resolverse: 9 words (autodisolverse, resolverse, volverse, devolverse, revolverse, desenvolverse). Like Revolver, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("resolverse", 9, "~o~verse", like Revolver;
    vos: _, _, "~o~vete";
    imperative: "~ué~vete", "~ué~vase", "~o~vámonos", "~o~veos", "~ué~vanse";
    infinitive: "~o~verse";
    gerund: "~o~viéndose";
    ),
    // Detenerse: 9 words (abstenerse, contenerse, sostenerse, mantenerse, detenerse, entretenerse). Like Tener, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("detenerse", 9, "~enerse", like Tener;
    vos: _, _, "~enete";
    imperative: "~ente", "~éngase", "~engámonos", "~eneos", "~énganse";
    infinitive: "~enerse";
    gerund: "~eniéndose";
    ),
    // Santiguarse: 9 words (achiguarse, apaciguarse, santiguarse, enmaniguarse, amortiguarse, averiguarse).
    conjugation!("santiguarse", 9, "~i~uarse";
    stem participle: "~i~uad- like echar";
    pres_1: "~i~uo", "~i~uamos";
    pres_2: "~i~uas", "~i~uáis";
    pres_3: "~i~ua", "~i~uan";
    impf_1: "~i~uaba", "~i~uábamos";
    impf_2: "~i~uabas", "~i~uabais";
    impf_3: _, "~i~uaban";
    pret_1: "~i~üé", "~i~uamos";
    pret_2: "~i~uaste", "~i~uasteis";
    pret_3: "~i~uó", "~i~uaron";
    fut_1: "~i~uaré", "~i~uaremos";
    fut_2: "~i~uarás", "~i~uaréis";
    fut_3: "~i~uará", "~i~uarán";
    cond_1: "~i~uaría", "~i~uaríamos";
    cond_2: "~i~uarías", "~i~uaríais";
    cond_3: _, "~i~uarían";
    subj_1: "~i~üe", "~i~üemos";
    subj_2: "~i~ües", "~i~üéis";
    subj_3: _, "~i~üen";
    subj_impf_1: "~i~uara", "~i~uáramos";
    subj_impf_2: "~i~uaras", "~i~uarais";
    subj_impf_3: _, "~i~uaran";
    subj_impf_se_1: "~i~uase", "~i~uásemos";
    subj_impf_se_2: "~i~uases", "~i~uaseis";
    subj_impf_se_3: _, "~i~uasen";
    subj_fut_1: "~i~uare", "~i~uáremos";
    subj_fut_2: "~i~uares", "~i~uareis";
    subj_fut_3: _, "~i~uaren";
    vos: "~i~uás", "~i~üés", "~i~uate";
    imperative: "~í~uate", "~í~üese", "~i~üémonos", "~i~uaos", "~í~üense";
    infinitive: "~i~uarse";
    gerund: "~i~uándose";
    ),
    // Refreír: 9 words (reír, freír, sonreír, engreír, desleír, esleír).
    conjugation!("refreír", 9, "~eír";
    stem participle: "~eíd- like echar";
    pres_1: "~ío", "~eímos";
    pres_2: "~íes", "~eís";
    pres_3: "~íe", "~íen";
    impf_1: "~eía", "~eíamos";
    impf_2: "~eías", "~eíais";
    impf_3: _, "~eían";
    pret_1: "~eí", "~eímos";
    pret_2: "~eíste", "~eísteis";
    pret_3: "~ió", "~ieron";
    fut_1: "~eiré", "~eiremos";
    fut_2: "~eirás", "~eiréis";
    fut_3: "~eirá", "~eirán";
    cond_1: "~eiría", "~eiríamos";
    cond_2: "~eirías", "~eiríais";
    cond_3: _, "~eirían";
    subj_1: "~ía", "~iamos";
    subj_2: "~ías", "~iáis";
    subj_3: _, "~ían";
    subj_impf_1: "~iera", "~iéramos";
    subj_impf_2: "~ieras", "~ierais";
    subj_impf_3: _, "~ieran";
    subj_impf_se_1: "~iese", "~iésemos";
    subj_impf_se_2: "~ieses", "~ieseis";
    subj_impf_se_3: _, "~iesen";
    subj_fut_1: "~iere", "~iéremos";
    subj_fut_2: "~ieres", "~iereis";
    subj_fut_3: _, "~ieren";
    vos: "~eís", "~iás", "~eí";
    imperative: "~íe", "~ía", "~iamos", "~eíd", "~ían";
    infinitive: "~eír";
    gerund: "~iendo";
    ),
    // Guiar: 8 words (liar, fiar, criar, guiar, ciar, piar). Like Desviar, except in 7 cells: pres_2 pl, pret_1 sg, pret_3 sg, subj_2 pl, vos ind, vos subj, ….
    conjugation!("guiar", 8, "~iar", like Desviar;
    pres_2: _, "~iais";
    pret_1: "~ie", _;
    pret_3: "~io", _;
    subj_2: _, "~ieis";
    vos: "~ias", "~ies", "~ia";
    ),
    // Reducirse: 8 words (autorreproducirse, autoconducirse, autoproducirse, reducirse, reproducirse, introducirse).
    conjugation!("reducirse", 8, "~ucirse";
    stem participle: "~ucid- like echar";
    pres_1: "~uzco", "~ucimos";
    pres_2: "~uces", "~ucís";
    pres_3: "~uce", "~ucen";
    impf_1: "~ucía", "~ucíamos";
    impf_2: "~ucías", "~ucíais";
    impf_3: _, "~ucían";
    pret_1: "~uje", "~ujimos";
    pret_2: "~ujiste", "~ujisteis";
    pret_3: "~ujo", "~ujeron";
    fut_1: "~uciré", "~uciremos";
    fut_2: "~ucirás", "~uciréis";
    fut_3: "~ucirá", "~ucirán";
    cond_1: "~uciría", "~uciríamos";
    cond_2: "~ucirías", "~uciríais";
    cond_3: _, "~ucirían";
    subj_1: "~uzca", "~uzcamos";
    subj_2: "~uzcas", "~uzcáis";
    subj_3: _, "~uzcan";
    subj_impf_1: "~ujera", "~ujéramos";
    subj_impf_2: "~ujeras", "~ujerais";
    subj_impf_3: _, "~ujeran";
    subj_impf_se_1: "~ujese", "~ujésemos";
    subj_impf_se_2: "~ujeses", "~ujeseis";
    subj_impf_se_3: _, "~ujesen";
    subj_fut_1: "~ujere", "~ujéremos";
    subj_fut_2: "~ujeres", "~ujereis";
    subj_fut_3: _, "~ujeren";
    vos: "~ucís", "~uzcás", "~ucite";
    imperative: "~úcete", "~úzcase", "~uzcámonos", "~ucíos", "~úzcanse";
    infinitive: "~ucirse";
    gerund: "~uciéndose";
    ),
    // Desmedirse: 8 words (descomedirse, desmedirse, despedirse, acomedirse, medirse, expedirse).
    conjugation!("desmedirse", 8, "~edirse";
    stem participle: "~edid- like echar";
    pres_1: "~ido", "~edimos";
    pres_2: "~ides", "~edís";
    pres_3: "~ide", "~iden";
    impf_1: "~edía", "~edíamos";
    impf_2: "~edías", "~edíais";
    impf_3: _, "~edían";
    pret_1: "~edí", "~edimos";
    pret_2: "~ediste", "~edisteis";
    pret_3: "~idió", "~idieron";
    fut_1: "~ediré", "~ediremos";
    fut_2: "~edirás", "~ediréis";
    fut_3: "~edirá", "~edirán";
    cond_1: "~ediría", "~ediríamos";
    cond_2: "~edirías", "~ediríais";
    cond_3: _, "~edirían";
    subj_1: "~ida", "~idamos";
    subj_2: "~idas", "~idáis";
    subj_3: _, "~idan";
    subj_impf_1: "~idiera", "~idiéramos";
    subj_impf_2: "~idieras", "~idierais";
    subj_impf_3: _, "~idieran";
    subj_impf_se_1: "~idiese", "~idiésemos";
    subj_impf_se_2: "~idieses", "~idieseis";
    subj_impf_se_3: _, "~idiesen";
    subj_fut_1: "~idiere", "~idiéremos";
    subj_fut_2: "~idieres", "~idiereis";
    subj_fut_3: _, "~idieren";
    vos: "~edís", "~idás", "~edite";
    imperative: "~ídete", "~ídase", "~idámonos", "~edíos", "~ídanse";
    infinitive: "~edirse";
    gerund: "~idiéndose";
    ),
    // Creerse: 8 words (caerse, creerse, peerse, proveerse, corroerse, desposeerse).
    conjugation!("creerse", 8, "~~erse";
    stem participle: "~~íd- like echar";
    pres_1: "~~o", "~~emos";
    pres_2: "~~es", "~~éis";
    pres_3: "~~e", "~~en";
    impf_1: "~~ía", "~~íamos";
    impf_2: "~~ías", "~~íais";
    impf_3: _, "~~ían";
    pret_1: "~~í", "~~ímos";
    pret_2: "~~íste", "~~ísteis";
    pret_3: "~~yó", "~~yeron";
    fut_1: "~~eré", "~~eremos";
    fut_2: "~~erás", "~~eréis";
    fut_3: "~~erá", "~~erán";
    cond_1: "~~ería", "~~eríamos";
    cond_2: "~~erías", "~~eríais";
    cond_3: _, "~~erían";
    subj_1: "~~a", "~~amos";
    subj_2: "~~as", "~~áis";
    subj_3: _, "~~an";
    subj_impf_1: "~~yera", "~~yéramos";
    subj_impf_2: "~~yeras", "~~yerais";
    subj_impf_3: _, "~~yeran";
    subj_impf_se_1: "~~yese", "~~yésemos";
    subj_impf_se_2: "~~yeses", "~~yeseis";
    subj_impf_se_3: _, "~~yesen";
    subj_fut_1: "~~yere", "~~yéremos";
    subj_fut_2: "~~yeres", "~~yereis";
    subj_fut_3: _, "~~yeren";
    vos: "~~és", "~~ás", "~~ete";
    imperative: "~é~te", "~éas~", "~~ámonos", "~~eos", "~éans~";
    infinitive: "~~erse";
    gerund: "~~yéndose";
    ),
    // Moler: 7 words (doler, soler, moler, demoler, condoler, remoler).
    conjugation!("moler", 7, "~oler";
    stem participle: "~olid- like echar";
    pres_1: "~uelo", "~olemos";
    pres_2: "~ueles", "~oléis";
    pres_3: "~uele", "~uelen";
    impf_1: "~olía", "~olíamos";
    impf_2: "~olías", "~olíais";
    impf_3: _, "~olían";
    pret_1: "~olí", "~olimos";
    pret_2: "~oliste", "~olisteis";
    pret_3: "~olió", "~olieron";
    fut_1: "~oleré", "~oleremos";
    fut_2: "~olerás", "~oleréis";
    fut_3: "~olerá", "~olerán";
    cond_1: "~olería", "~oleríamos";
    cond_2: "~olerías", "~oleríais";
    cond_3: _, "~olerían";
    subj_1: "~uela", "~olamos";
    subj_2: "~uelas", "~oláis";
    subj_3: _, "~uelan";
    subj_impf_1: "~oliera", "~oliéramos";
    subj_impf_2: "~olieras", "~olierais";
    subj_impf_3: _, "~olieran";
    subj_impf_se_1: "~oliese", "~oliésemos";
    subj_impf_se_2: "~olieses", "~olieseis";
    subj_impf_se_3: _, "~oliesen";
    subj_fut_1: "~oliere", "~oliéremos";
    subj_fut_2: "~olieres", "~oliereis";
    subj_fut_3: _, "~olieren";
    vos: "~olés", "~olás", "~olé";
    imperative: "~uele", "~uela", "~olamos", "~oled", "~uelan";
    infinitive: "~oler";
    gerund: "~oliendo";
    ),
    // Regir: 7 words (elegir, corregir, colegir, reelegir, regir, recolegir).
    conjugation!("regir", 7, "~egir";
    stem participle: "~egid- like echar";
    pres_1: "~ijo", "~egimos";
    pres_2: "~iges", "~egís";
    pres_3: "~ige", "~igen";
    impf_1: "~egía", "~egíamos";
    impf_2: "~egías", "~egíais";
    impf_3: _, "~egían";
    pret_1: "~egí", "~egimos";
    pret_2: "~egiste", "~egisteis";
    pret_3: "~igió", "~igieron";
    fut_1: "~egiré", "~egiremos";
    fut_2: "~egirás", "~egiréis";
    fut_3: "~egirá", "~egirán";
    cond_1: "~egiría", "~egiríamos";
    cond_2: "~egirías", "~egiríais";
    cond_3: _, "~egirían";
    subj_1: "~ija", "~ijamos";
    subj_2: "~ijas", "~ijáis";
    subj_3: _, "~ijan";
    subj_impf_1: "~igiera", "~igiéramos";
    subj_impf_2: "~igieras", "~igierais";
    subj_impf_3: _, "~igieran";
    subj_impf_se_1: "~igiese", "~igiésemos";
    subj_impf_se_2: "~igieses", "~igieseis";
    subj_impf_se_3: _, "~igiesen";
    subj_fut_1: "~igiere", "~igiéremos";
    subj_fut_2: "~igieres", "~igiereis";
    subj_fut_3: _, "~igieren";
    vos: "~egís", "~ijás", "~egí";
    imperative: "~ige", "~ija", "~ijamos", "~egid", "~ijan";
    infinitive: "~egir";
    gerund: "~igiendo";
    ),
    // Mover: 7 words (llover, promover, mover, remover, conmover, amover).
    conjugation!("mover", 7, "~over";
    stem participle: "~ovid- like echar";
    pres_1: "~uevo", "~ovemos";
    pres_2: "~ueves", "~ovéis";
    pres_3: "~ueve", "~ueven";
    impf_1: "~ovía", "~ovíamos";
    impf_2: "~ovías", "~ovíais";
    impf_3: _, "~ovían";
    pret_1: "~oví", "~ovimos";
    pret_2: "~oviste", "~ovisteis";
    pret_3: "~ovió", "~ovieron";
    fut_1: "~overé", "~overemos";
    fut_2: "~overás", "~overéis";
    fut_3: "~overá", "~overán";
    cond_1: "~overía", "~overíamos";
    cond_2: "~overías", "~overíais";
    cond_3: _, "~overían";
    subj_1: "~ueva", "~ovamos";
    subj_2: "~uevas", "~ováis";
    subj_3: _, "~uevan";
    subj_impf_1: "~oviera", "~oviéramos";
    subj_impf_2: "~ovieras", "~ovierais";
    subj_impf_3: _, "~ovieran";
    subj_impf_se_1: "~oviese", "~oviésemos";
    subj_impf_se_2: "~ovieses", "~ovieseis";
    subj_impf_se_3: _, "~oviesen";
    subj_fut_1: "~oviere", "~oviéremos";
    subj_fut_2: "~ovieres", "~oviereis";
    subj_fut_3: _, "~ovieren";
    vos: "~ovés", "~ovás", "~ové";
    imperative: "~ueve", "~ueva", "~ovamos", "~oved", "~uevan";
    infinitive: "~over";
    gerund: "~oviendo";
    ),
    // Ver: 7 words (ver, ser, prever, entrever, antever, rever). Like Meter, except in 21 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("ver", 7, "~er", like Meter;
    stem participle: "~ist- like echar";
    pres_1: "~eo", _;
    pres_2: "~és", _;
    pres_3: "~é", "~én";
    impf_1: "~eía", "~eíamos";
    impf_2: "~eías", "~eíais";
    impf_3: _, "~eían";
    subj_1: "~ea", "~eamos";
    subj_2: "~eas", "~eáis";
    subj_3: _, "~ean";
    vos: derived, "~eás", derived;
    imperative: "~é", "~ea", "~eamos", _, "~ean";
    ),
    // Tañer: 7 words (jier, atañer, tañer, empeller, toller, creyer). Like Meter, except in 18 cells: pret_3 sg, pret_3 pl, subj_impf_1 sg, subj_impf_1 pl, subj_impf_2 sg, subj_impf_2 pl, ….
    conjugation!("tañer", 7, "~er", like Meter;
    pret_3: "~ó", "~eron";
    subj_impf_1: "~era", "~éramos";
    subj_impf_2: "~eras", "~erais";
    subj_impf_3: _, "~eran";
    subj_impf_se_1: "~ese", "~ésemos";
    subj_impf_se_2: "~eses", "~eseis";
    subj_impf_se_3: _, "~esen";
    subj_fut_1: "~ere", "~éremos";
    subj_fut_2: "~eres", "~ereis";
    subj_fut_3: _, "~eren";
    gerund: "~endo";
    ),
    // Ahumarse: 7 words (ahusarse, ahumarse, aunarse, rehusarse, rehurtarse, sahumarse). Like Emputarse, except in 7 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("ahumarse", 7, "~u~arse", like Emputarse;
    pres_1: "~ú~o", _;
    pres_2: "~ú~as", _;
    pres_3: "~ú~a", "~ú~an";
    subj_1: "~ú~e", _;
    subj_2: "~ú~es", _;
    subj_3: _, "~ú~en";
    ),
    // Venirse: 7 words (venirse, desconvenirse, avenirse, desavenirse, convenirse, prevenirse). Like Venir, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("venirse", 7, "~e~irse", like Venir;
    vos: _, _, "~e~ite";
    imperative: "~e~te", "~é~gase", "~e~gámonos", "~e~íos", "~é~ganse";
    infinitive: "~e~irse";
    gerund: "~i~iéndose";
    ),
    // Aterirse: 7 words (aterirse, empedernirse, arrecirse, hespirse, denegrirse, sumergirse). Like Entremeterse, except in 17 cells: pres_1 pl, pres_2 pl, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, ….
    conjugation!("aterirse", 7, "~e~irse", like Entremeterse;
    pres_1: _, "~e~imos";
    pres_2: _, "~e~ís";
    fut_1: "~e~iré", "~e~iremos";
    fut_2: "~e~irás", "~e~iréis";
    fut_3: "~e~irá", "~e~irán";
    cond_1: "~e~iría", "~e~iríamos";
    cond_2: "~e~irías", "~e~iríais";
    cond_3: _, "~e~irían";
    vos: "~e~ís", _, "~e~ite";
    imperative: _, _, _, "~e~íos", _;
    infinitive: "~e~irse";
    ),
    // Jetiarse: 7 words (mamoniarse, patrasiarse, friquiarse, paniquiarse, jetiarse, penquiarse). Like Meliar, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("jetiarse", 7, "~iarse", like Meliar;
    vos: _, _, "~iate";
    imperative: "~éate", "~éese", "~iémonos", "~iaos", "~éense";
    infinitive: "~iarse";
    gerund: "~iándose";
    ),
    // Deshacerse: 7 words (autosatisfacerse, hacerse, rehacerse, desfacerse, deshacerse, contrahacerse). Like Hacer, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("deshacerse", 7, "~acerse", like Hacer;
    vos: _, _, "~acete";
    imperative: "~azte", "~ágase", "~agámonos", "~aceos", "~áganse";
    infinitive: "~acerse";
    gerund: "~aciéndose";
    ),
    // Relamerse: 7 words (valerse, prevalerse, relamerse, lamerse, precaverse, barrerse). Like Abatirse, except in 17 cells: pres_1 pl, pres_2 pl, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, ….
    conjugation!("relamerse", 7, "~a~erse", like Abatirse;
    pres_1: _, "~a~emos";
    pres_2: _, "~a~éis";
    fut_1: "~a~eré", "~a~eremos";
    fut_2: "~a~erás", "~a~eréis";
    fut_3: "~a~erá", "~a~erán";
    cond_1: "~a~ería", "~a~eríamos";
    cond_2: "~a~erías", "~a~eríais";
    cond_3: _, "~a~erían";
    vos: "~a~és", _, "~a~ete";
    imperative: _, _, _, "~a~eos", _;
    infinitive: "~a~erse";
    ),
    // Seguir: 6 words (proseguir, perseguir, seguir, conseguir, subseguir, reseguir).
    conjugation!("seguir", 6, "~eguir";
    stem participle: "~eguid- like echar";
    pres_1: "~igo", "~eguimos";
    pres_2: "~igues", "~eguís";
    pres_3: "~igue", "~iguen";
    impf_1: "~eguía", "~eguíamos";
    impf_2: "~eguías", "~eguíais";
    impf_3: _, "~eguían";
    pret_1: "~eguí", "~eguimos";
    pret_2: "~eguiste", "~eguisteis";
    pret_3: "~iguió", "~iguieron";
    fut_1: "~eguiré", "~eguiremos";
    fut_2: "~eguirás", "~eguiréis";
    fut_3: "~eguirá", "~eguirán";
    cond_1: "~eguiría", "~eguiríamos";
    cond_2: "~eguirías", "~eguiríais";
    cond_3: _, "~eguirían";
    subj_1: "~iga", "~igamos";
    subj_2: "~igas", "~igáis";
    subj_3: _, "~igan";
    subj_impf_1: "~iguiera", "~iguiéramos";
    subj_impf_2: "~iguieras", "~iguierais";
    subj_impf_3: _, "~iguieran";
    subj_impf_se_1: "~iguiese", "~iguiésemos";
    subj_impf_se_2: "~iguieses", "~iguieseis";
    subj_impf_se_3: _, "~iguiesen";
    subj_fut_1: "~iguiere", "~iguiéremos";
    subj_fut_2: "~iguieres", "~iguiereis";
    subj_fut_3: _, "~iguieren";
    vos: "~eguís", "~igás", "~eguí";
    imperative: "~igue", "~iga", "~igamos", "~eguid", "~igan";
    infinitive: "~eguir";
    gerund: "~iguiendo";
    ),
    // Agorar: 6 words (degollar, agorar, regoldar, engorar, cogolmar, gorar). Like Volar, except in 10 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("agorar", 6, "~o~ar", like Volar;
    pres_1: "~üe~o", _;
    pres_2: "~üe~as", _;
    pres_3: "~üe~a", "~üe~an";
    subj_1: "~üe~e", _;
    subj_2: "~üe~es", _;
    subj_3: _, "~üe~en";
    imperative: "~üe~a", "~üe~e", _, _, "~üe~en";
    ),
    // Apajuilar: 6 words (acuclillar, encuclillar, apajuilar, apulismar, aculillar, encuitar). Like Rilarse, except in 0 cells: .
    conjugation!("apajuilar", 6, "~i~ar", like Rilarse;
    ),
    // Recogerse: 6 words (encogerse, desencogerse, sobrecogerse, recogerse, cogerse, acogerse).
    conjugation!("recogerse", 6, "~ogerse";
    stem participle: "~ogid- like echar";
    pres_1: "~ojo", "~ogemos";
    pres_2: "~oges", "~ogéis";
    pres_3: "~oge", "~ogen";
    impf_1: "~ogía", "~ogíamos";
    impf_2: "~ogías", "~ogíais";
    impf_3: _, "~ogían";
    pret_1: "~ogí", "~ogimos";
    pret_2: "~ogiste", "~ogisteis";
    pret_3: "~ogió", "~ogieron";
    fut_1: "~ogeré", "~ogeremos";
    fut_2: "~ogerás", "~ogeréis";
    fut_3: "~ogerá", "~ogerán";
    cond_1: "~ogería", "~ogeríamos";
    cond_2: "~ogerías", "~ogeríais";
    cond_3: _, "~ogerían";
    subj_1: "~oja", "~ojamos";
    subj_2: "~ojas", "~ojáis";
    subj_3: _, "~ojan";
    subj_impf_1: "~ogiera", "~ogiéramos";
    subj_impf_2: "~ogieras", "~ogierais";
    subj_impf_3: _, "~ogieran";
    subj_impf_se_1: "~ogiese", "~ogiésemos";
    subj_impf_se_2: "~ogieses", "~ogieseis";
    subj_impf_se_3: _, "~ogiesen";
    subj_fut_1: "~ogiere", "~ogiéremos";
    subj_fut_2: "~ogieres", "~ogiereis";
    subj_fut_3: _, "~ogieren";
    vos: "~ogés", "~ojás", "~ogete";
    imperative: "~ógete", "~ójase", "~ojámonos", "~ogeos", "~ójanse";
    infinitive: "~ogerse";
    gerund: "~ogiéndose";
    ),
    // Oír: 6 words (oír, embaír, desvaír, desoír, trasoír, entreoír). Like Instruir, except in 16 cells: pres_1 sg, pres_1 pl, pret_1 pl, pret_2 sg, pret_2 pl, subj_1 sg, ….
    conjugation!("oír", 6, "~ír", like Instruir;
    stem participle: "~íd- like echar";
    pres_1: "~igo", "~ímos";
    pret_1: _, "~ímos";
    pret_2: "~íste", "~ísteis";
    subj_1: "~iga", "~igamos";
    subj_2: "~igas", "~igáis";
    subj_3: _, "~igan";
    vos: _, "~igás", _;
    imperative: _, "~iga", "~igamos", "~íd", "~igan";
    infinitive: "~ír";
    ),
    // Desdecir: 6 words (bendecir, maldecir, contradecir, desdecir, predecir, condecir).
    conjugation!("desdecir", 6, "~ecir";
    stem participle: "~ich- like echar";
    pres_1: "~igo", "~ecimos";
    pres_2: "~ices", "~ecís";
    pres_3: "~ice", "~icen";
    impf_1: "~ecía", "~ecíamos";
    impf_2: "~ecías", "~ecíais";
    impf_3: _, "~ecían";
    pret_1: "~ije", "~ijimos";
    pret_2: "~ijiste", "~ijisteis";
    pret_3: "~ijo", "~ijeron";
    fut_1: "~eciré|~iré", "~eciremos|~iremos";
    fut_2: "~ecirás|~irás", "~eciréis|~iréis";
    fut_3: "~ecirá|~irá", "~ecirán|~irán";
    cond_1: "~eciría|~iría", "~eciríamos|~iríamos";
    cond_2: "~ecirías|~irías", "~eciríais|~iríais";
    cond_3: _, "~ecirían|~irían";
    subj_1: "~iga", "~igamos";
    subj_2: "~igas", "~igáis";
    subj_3: _, "~igan";
    subj_impf_1: "~ijera", "~ijéramos";
    subj_impf_2: "~ijeras", "~ijerais";
    subj_impf_3: _, "~ijeran";
    subj_impf_se_1: "~ijese", "~ijésemos";
    subj_impf_se_2: "~ijeses", "~ijeseis";
    subj_impf_se_3: _, "~ijesen";
    subj_fut_1: "~ijere", "~ijéremos";
    subj_fut_2: "~ijeres", "~ijereis";
    subj_fut_3: _, "~ijeren";
    vos: "~ecís", "~igás", "~ecí";
    imperative: "~ice", "~iga", "~igamos", "~ecid", "~igan";
    infinitive: "~ecir";
    gerund: "~iciendo";
    ),
    // Entredormirse: 6 words (entredormirse, podrirse, adormirse, dormirse, morirse, repodrirse).
    conjugation!("entredormirse", 6, "~o~irse";
    stem participle: "~o~id- like echar";
    pres_1: "~ue~o", "~o~imos";
    pres_2: "~ue~es", "~o~ís";
    pres_3: "~ue~e", "~ue~en";
    impf_1: "~o~ía", "~o~íamos";
    impf_2: "~o~ías", "~o~íais";
    impf_3: _, "~o~ían";
    pret_1: "~o~í", "~o~imos";
    pret_2: "~o~iste", "~o~isteis";
    pret_3: "~u~ió", "~u~ieron";
    fut_1: "~o~iré", "~o~iremos";
    fut_2: "~o~irás", "~o~iréis";
    fut_3: "~o~irá", "~o~irán";
    cond_1: "~o~iría", "~o~iríamos";
    cond_2: "~o~irías", "~o~iríais";
    cond_3: _, "~o~irían";
    subj_1: "~ue~a", "~u~amos";
    subj_2: "~ue~as", "~u~áis";
    subj_3: _, "~ue~an";
    subj_impf_1: "~u~iera", "~u~iéramos";
    subj_impf_2: "~u~ieras", "~u~ierais";
    subj_impf_3: _, "~u~ieran";
    subj_impf_se_1: "~u~iese", "~u~iésemos";
    subj_impf_se_2: "~u~ieses", "~u~ieseis";
    subj_impf_se_3: _, "~u~iesen";
    subj_fut_1: "~u~iere", "~u~iéremos";
    subj_fut_2: "~u~ieres", "~u~iereis";
    subj_fut_3: _, "~u~ieren";
    vos: "~o~ís", "~u~ás", "~o~ite";
    imperative: "~ué~ete", "~ué~ase", "~u~ámonos", "~o~íos", "~ué~anse";
    infinitive: "~o~irse";
    gerund: "~u~iéndose";
    ),
    // Tender: 5 words (discerner, cerner, jerver, herver, jerber). Like Conocer, except in 14 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_1 pl, ….
    conjugation!("tender", 5, "~~er", like Conocer;
    pres_1: "~i~o", _;
    pres_2: "~i~es", _;
    pres_3: "~i~e", "~i~en";
    subj_1: "~i~a", "~~amos";
    subj_2: "~i~as", "~~áis";
    subj_3: _, "~i~an";
    vos: _, "~~ás", _;
    imperative: "~i~e", "~i~a", "~~amos", _, "~i~an";
    ),
    // Caer: 5 words (caer, raer, recaer, decaer, descaer). Like Creer, except in 10 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("caer", 5, "~er", like Creer;
    pres_1: "~igo", _;
    subj_1: "~iga", "~igamos";
    subj_2: "~igas", "~igáis";
    subj_3: _, "~igan";
    vos: _, "~igás", _;
    imperative: _, "~iga", "~igamos", _, "~igan";
    ),
    // Querer: 5 words (querer, malquerer, bienquerer, desquerer, requerer).
    conjugation!("querer", 5, "~erer";
    stem participle: "~erid- like echar";
    pres_1: "~iero", "~eremos";
    pres_2: "~ieres", "~eréis";
    pres_3: "~iere", "~ieren";
    impf_1: "~ería", "~eríamos";
    impf_2: "~erías", "~eríais";
    impf_3: _, "~erían";
    pret_1: "~ise", "~isimos";
    pret_2: "~isiste", "~isisteis";
    pret_3: "~iso", "~isieron";
    fut_1: "~erré", "~erremos";
    fut_2: "~errás", "~erréis";
    fut_3: "~errá", "~errán";
    cond_1: "~erría", "~erríamos";
    cond_2: "~errías", "~erríais";
    cond_3: _, "~errían";
    subj_1: "~iera", "~eramos";
    subj_2: "~ieras", "~eráis";
    subj_3: _, "~ieran";
    subj_impf_1: "~isiera", "~isiéramos";
    subj_impf_2: "~isieras", "~isierais";
    subj_impf_3: _, "~isieran";
    subj_impf_se_1: "~isiese", "~isiésemos";
    subj_impf_se_2: "~isieses", "~isieseis";
    subj_impf_se_3: _, "~isiesen";
    subj_fut_1: "~isiere", "~isiéremos";
    subj_fut_2: "~isieres", "~isiereis";
    subj_fut_3: _, "~isieren";
    vos: "~erés", "~erás", "~eré";
    imperative: "~iere", "~iera", "~eramos", "~ered", "~ieran";
    infinitive: "~erer";
    gerund: "~eriendo";
    ),
    // Huir: 5 words (fuir, huir, fluir, fruir, luir). Like Instruir, except in 4 cells: pres_2 pl, pret_1 sg, vos ind, vos imp.
    conjugation!("huir", 5, "~ir", like Instruir;
    pres_2: _, "~is";
    pret_1: "~i", _;
    vos: "~is", _, "~i";
    ),
    // Cocer: 5 words (cocer, escocer, recocer, descocer, precocer).
    conjugation!("cocer", 5, "~ocer";
    stem participle: "~ocid- like echar";
    pres_1: "~uezo", "~ocemos";
    pres_2: "~ueces", "~océis";
    pres_3: "~uece", "~uecen";
    impf_1: "~ocía", "~ocíamos";
    impf_2: "~ocías", "~ocíais";
    impf_3: _, "~ocían";
    pret_1: "~ocí", "~ocimos";
    pret_2: "~ociste", "~ocisteis";
    pret_3: "~oció", "~ocieron";
    fut_1: "~oceré", "~oceremos";
    fut_2: "~ocerás", "~oceréis";
    fut_3: "~ocerá", "~ocerán";
    cond_1: "~ocería", "~oceríamos";
    cond_2: "~ocerías", "~oceríais";
    cond_3: _, "~ocerían";
    subj_1: "~ueza", "~ozamos";
    subj_2: "~uezas", "~ozáis";
    subj_3: _, "~uezan";
    subj_impf_1: "~ociera", "~ociéramos";
    subj_impf_2: "~ocieras", "~ocierais";
    subj_impf_3: _, "~ocieran";
    subj_impf_se_1: "~ociese", "~ociésemos";
    subj_impf_se_2: "~ocieses", "~ocieseis";
    subj_impf_se_3: _, "~ociesen";
    subj_fut_1: "~ociere", "~ociéremos";
    subj_fut_2: "~ocieres", "~ociereis";
    subj_fut_3: _, "~ocieren";
    vos: "~océs", "~ozás", "~océ";
    imperative: "~uece", "~ueza", "~ozamos", "~oced", "~uezan";
    infinitive: "~ocer";
    gerund: "~ociendo";
    ),
    // Delinquir: 5 words (delinquir, derelinquir, jaquir, derrelinquir, relinquir).
    conjugation!("delinquir", 5, "~quir";
    stem participle: "~quid- like echar";
    pres_1: "~co", "~quimos";
    pres_2: "~ques", "~quís";
    pres_3: "~que", "~quen";
    impf_1: "~quía", "~quíamos";
    impf_2: "~quías", "~quíais";
    impf_3: _, "~quían";
    pret_1: "~quí", "~quimos";
    pret_2: "~quiste", "~quisteis";
    pret_3: "~quió", "~quieron";
    fut_1: "~quiré", "~quiremos";
    fut_2: "~quirás", "~quiréis";
    fut_3: "~quirá", "~quirán";
    cond_1: "~quiría", "~quiríamos";
    cond_2: "~quirías", "~quiríais";
    cond_3: _, "~quirían";
    subj_1: "~ca", "~camos";
    subj_2: "~cas", "~cáis";
    subj_3: _, "~can";
    subj_impf_1: "~quiera", "~quiéramos";
    subj_impf_2: "~quieras", "~quierais";
    subj_impf_3: _, "~quieran";
    subj_impf_se_1: "~quiese", "~quiésemos";
    subj_impf_se_2: "~quieses", "~quieseis";
    subj_impf_se_3: _, "~quiesen";
    subj_fut_1: "~quiere", "~quiéremos";
    subj_fut_2: "~quieres", "~quiereis";
    subj_fut_3: _, "~quieren";
    vos: "~quís", "~cás", "~quí";
    imperative: "~que", "~ca", "~camos", "~quid", "~can";
    infinitive: "~quir";
    gerund: "~quiendo";
    ),
    // Afligirse: 5 words (autoexigirse, afligirse, arrigirse, dirigirse, erigirse). Like Regir, except in 31 cells: pres_1 pl, pres_2 pl, impf_1 sg, impf_1 pl, impf_2 sg, impf_2 pl, ….
    conjugation!("afligirse", 5, "~igirse", like Regir;
    stem participle: "~igid- like echar";
    pres_1: _, "~igimos";
    pres_2: _, "~igís";
    impf_1: "~igía", "~igíamos";
    impf_2: "~igías", "~igíais";
    impf_3: _, "~igían";
    pret_1: "~igí", "~igimos";
    pret_2: "~igiste", "~igisteis";
    fut_1: "~igiré", "~igiremos";
    fut_2: "~igirás", "~igiréis";
    fut_3: "~igirá", "~igirán";
    cond_1: "~igiría", "~igiríamos";
    cond_2: "~igirías", "~igiríais";
    cond_3: _, "~igirían";
    vos: "~igís", _, "~igite";
    imperative: "~ígete", "~íjase", "~ijámonos", "~igíos", "~íjanse";
    infinitive: "~igirse";
    gerund: "~igiéndose";
    ),
    // Reconocerse: 5 words (autoreconocerse, autorreconocerse, conocerse, desconocerse, reconocerse). Like Destoserse, except in 10 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("reconocerse", 5, "~o~erse", like Destoserse;
    pres_1: "~oz~o", _;
    subj_1: "~oz~a", "~oz~amos";
    subj_2: "~oz~as", "~oz~áis";
    subj_3: _, "~oz~an";
    vos: _, "~oz~ás", _;
    imperative: _, "~óz~ase", "~oz~ámonos", _, "~óz~anse";
    ),
    // Teñirse: 5 words (teñirse, ceñirse, desceñirse, desteñirse, estreñirse). Like Ceñir, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("teñirse", 5, "~eñirse", like Ceñir;
    vos: _, _, "~eñite";
    imperative: "~íñete", "~íñase", "~iñámonos", "~eñíos", "~íñanse";
    infinitive: "~eñirse";
    gerund: "~iñéndose";
    ),
    // Trocarse: 5 words (aclocarse, trocarse, enclocarse, enllocarse, trastrocarse). Like Trocar, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("trocarse", 5, "~ocarse", like Trocar;
    vos: _, _, "~ocate";
    imperative: "~uécate", "~uéquese", "~oquémonos", "~ocaos", "~uéquense";
    infinitive: "~ocarse";
    gerund: "~ocándose";
    ),
    // Revestirse: 5 words (desvestirse, revestirse, sobrevestirse, travestirse, vestirse).
    conjugation!("revestirse", 5, "~e~tirse";
    stem participle: "~e~tid- like echar";
    pres_1: "~i~to", "~e~timos";
    pres_2: "~i~tes", "~e~tís";
    pres_3: "~i~te", "~i~ten";
    impf_1: "~e~tía", "~e~tíamos";
    impf_2: "~e~tías", "~e~tíais";
    impf_3: _, "~e~tían";
    pret_1: "~e~tí", "~e~timos";
    pret_2: "~e~tiste", "~e~tisteis";
    pret_3: "~i~tió", "~i~tieron";
    fut_1: "~e~tiré", "~e~tiremos";
    fut_2: "~e~tirás", "~e~tiréis";
    fut_3: "~e~tirá", "~e~tirán";
    cond_1: "~e~tiría", "~e~tiríamos";
    cond_2: "~e~tirías", "~e~tiríais";
    cond_3: _, "~e~tirían";
    subj_1: "~i~ta", "~i~tamos";
    subj_2: "~i~tas", "~i~táis";
    subj_3: _, "~i~tan";
    subj_impf_1: "~i~tiera", "~i~tiéramos";
    subj_impf_2: "~i~tieras", "~i~tierais";
    subj_impf_3: _, "~i~tieran";
    subj_impf_se_1: "~i~tiese", "~i~tiésemos";
    subj_impf_se_2: "~i~tieses", "~i~tieseis";
    subj_impf_se_3: _, "~i~tiesen";
    subj_fut_1: "~i~tiere", "~i~tiéremos";
    subj_fut_2: "~i~tieres", "~i~tiereis";
    subj_fut_3: _, "~i~tieren";
    vos: "~e~tís", "~i~tás", "~e~tite";
    imperative: "~í~tete", "~í~tase", "~i~támonos", "~e~tíos", "~í~tanse";
    infinitive: "~e~tirse";
    gerund: "~i~tiéndose";
    ),
    // Freírse: 5 words (reírse, desleírse, engreírse, sonreírse, freírse). Like Refreír, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("freírse", 5, "~eírse", like Refreír;
    vos: _, _, "~eíte";
    imperative: "~íete", "~íase", "~iámonos", "~eíos", "~íanse";
    infinitive: "~eírse";
    gerund: "~iéndose";
    ),
    // Dormir: 5 words (dormir, adormir, maldormir, podrir, repodrir). Like Entredormirse, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("dormir", 5, "~o~ir", like Entredormirse;
    vos: _, _, "~o~í";
    imperative: "~ue~e", "~ue~a", "~u~amos", "~o~id", "~ue~an";
    infinitive: "~o~ir";
    gerund: "~u~iendo";
    ),
    // Salir: 5 words (salir, asir, desasir, sobresalir, resalir). Like Curtir, except in 22 cells: pres_1 sg, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, ….
    conjugation!("salir", 5, "~ir", like Curtir;
    pres_1: "~go", _;
    fut_1: "~dré", "~dremos";
    fut_2: "~drás", "~dréis";
    fut_3: "~drá", "~drán";
    cond_1: "~dría", "~dríamos";
    cond_2: "~drías", "~dríais";
    cond_3: _, "~drían";
    subj_1: "~ga", "~gamos";
    subj_2: "~gas", "~gáis";
    subj_3: _, "~gan";
    vos: _, "~gás", _;
    imperative: "~", "~ga", "~gamos", _, "~gan";
    ),
    // Decir: 5 words (decir, antedecir, redecir, interdecir, entredecir). Like Desdecir, except in 11 cells: fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, fut_3 pl, ….
    conjugation!("decir", 5, "~ecir", like Desdecir;
    fut_1: "~iré", "~iremos";
    fut_2: "~irás", "~iréis";
    fut_3: "~irá", "~irán";
    cond_1: "~iría", "~iríamos";
    cond_2: "~irías", "~iríais";
    cond_3: _, "~irían";
    ),
    // Saber: 4 words (saber, caber, haber, resaber).
    conjugation!("saber", 4, "~aber";
    stem participle: "~abid- like echar";
    pres_1: "~é", "~abemos";
    pres_2: "~abes", "~abéis";
    pres_3: "~abe", "~aben";
    impf_1: "~abía", "~abíamos";
    impf_2: "~abías", "~abíais";
    impf_3: _, "~abían";
    pret_1: "~upe", "~upimos";
    pret_2: "~upiste", "~upisteis";
    pret_3: "~upo", "~upieron";
    fut_1: "~abré", "~abremos";
    fut_2: "~abrás", "~abréis";
    fut_3: "~abrá", "~abrán";
    cond_1: "~abría", "~abríamos";
    cond_2: "~abrías", "~abríais";
    cond_3: _, "~abrían";
    subj_1: "~epa", "~epamos";
    subj_2: "~epas", "~epáis";
    subj_3: _, "~epan";
    subj_impf_1: "~upiera", "~upiéramos";
    subj_impf_2: "~upieras", "~upierais";
    subj_impf_3: _, "~upieran";
    subj_impf_se_1: "~upiese", "~upiésemos";
    subj_impf_se_2: "~upieses", "~upieseis";
    subj_impf_se_3: _, "~upiesen";
    subj_fut_1: "~upiere", "~upiéremos";
    subj_fut_2: "~upieres", "~upiereis";
    subj_fut_3: _, "~upieren";
    vos: "~abés", "~epás", "~abé";
    imperative: "~abe", "~epa", "~epamos", "~abed", "~epan";
    infinitive: "~aber";
    gerund: "~abiendo";
    ),
    // Forzar: 4 words (almorzar, forzar, esforzar, reforzar).
    conjugation!("forzar", 4, "~o~zar";
    stem participle: "~o~zad- like echar";
    pres_1: "~ue~zo", "~o~zamos";
    pres_2: "~ue~zas", "~o~záis";
    pres_3: "~ue~za", "~ue~zan";
    impf_1: "~o~zaba", "~o~zábamos";
    impf_2: "~o~zabas", "~o~zabais";
    impf_3: _, "~o~zaban";
    pret_1: "~o~cé", "~o~zamos";
    pret_2: "~o~zaste", "~o~zasteis";
    pret_3: "~o~zó", "~o~zaron";
    fut_1: "~o~zaré", "~o~zaremos";
    fut_2: "~o~zarás", "~o~zaréis";
    fut_3: "~o~zará", "~o~zarán";
    cond_1: "~o~zaría", "~o~zaríamos";
    cond_2: "~o~zarías", "~o~zaríais";
    cond_3: _, "~o~zarían";
    subj_1: "~ue~ce", "~o~cemos";
    subj_2: "~ue~ces", "~o~céis";
    subj_3: _, "~ue~cen";
    subj_impf_1: "~o~zara", "~o~záramos";
    subj_impf_2: "~o~zaras", "~o~zarais";
    subj_impf_3: _, "~o~zaran";
    subj_impf_se_1: "~o~zase", "~o~zásemos";
    subj_impf_se_2: "~o~zases", "~o~zaseis";
    subj_impf_se_3: _, "~o~zasen";
    subj_fut_1: "~o~zare", "~o~záremos";
    subj_fut_2: "~o~zares", "~o~zareis";
    subj_fut_3: _, "~o~zaren";
    vos: "~o~zás", "~o~cés", "~o~zá";
    imperative: "~ue~za", "~ue~ce", "~o~cemos", "~o~zad", "~ue~cen";
    infinitive: "~o~zar";
    gerund: "~o~zando";
    ),
    // Adquirir: 4 words (adquirir, inquirir, perquirir, readquirir). Like Lucir, except in 14 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_1 pl, ….
    conjugation!("adquirir", 4, "~~ir", like Lucir;
    pres_1: "~e~o", _;
    pres_2: "~e~es", _;
    pres_3: "~e~e", "~e~en";
    subj_1: "~e~a", "~~amos";
    subj_2: "~e~as", "~~áis";
    subj_3: _, "~e~an";
    vos: _, "~~ás", _;
    imperative: "~e~e", "~e~a", "~~amos", _, "~e~an";
    ),
    // Cohibir: 4 words (prohibir, cohibir, rehinchir, desprohibir). Like Desvivirse, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("cohibir", 4, "~i~ir", like Desvivirse;
    pres_1: "~í~o", _;
    pres_2: "~í~es", _;
    pres_3: "~í~e", "~í~en";
    subj_1: "~í~a", _;
    subj_2: "~í~as", _;
    subj_3: _, "~í~an";
    vos: _, _, "~i~í";
    imperative: "~í~e", "~í~a", "~i~amos", "~i~id", "~í~an";
    infinitive: "~i~ir";
    gerund: "~i~iendo";
    ),
    // Morir: 4 words (morir, premorir, desmorir, entremorir). Like Dormir, except in 0 cells: .
    conjugation!("morir", 4, "~o~ir", like Dormir;
    stem participle: "~ue~t- like echar";
    ),
    // Distinguir: 4 words (distinguir, extinguir, subdistinguir, desextinguir). Like Inmiscuirse, except in 35 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, pret_3 sg, pret_3 pl, ….
    conjugation!("distinguir", 4, "~uir", like Inmiscuirse;
    pres_1: "~o", _;
    pres_2: "~ues", _;
    pres_3: "~ue", "~uen";
    pret_3: "~uió", "~uieron";
    subj_1: "~a", "~amos";
    subj_2: "~as", "~áis";
    subj_3: _, "~an";
    subj_impf_1: "~uiera", "~uiéramos";
    subj_impf_2: "~uieras", "~uierais";
    subj_impf_3: _, "~uieran";
    subj_impf_se_1: "~uiese", "~uiésemos";
    subj_impf_se_2: "~uieses", "~uieseis";
    subj_impf_se_3: _, "~uiesen";
    subj_fut_1: "~uiere", "~uiéremos";
    subj_fut_2: "~uieres", "~uiereis";
    subj_fut_3: _, "~uieren";
    vos: _, "~ás", "~uí";
    imperative: "~ue", "~a", "~amos", "~uid", "~an";
    infinitive: "~uir";
    gerund: "~uiendo";
    ),
    // Torcer: 4 words (torcer, retorcer, destorcer, contorcer).
    conjugation!("torcer", 4, "~o~cer";
    stem participle: "~o~cid- like echar";
    pres_1: "~ue~zo", "~o~cemos";
    pres_2: "~ue~ces", "~o~céis";
    pres_3: "~ue~ce", "~ue~cen";
    impf_1: "~o~cía", "~o~cíamos";
    impf_2: "~o~cías", "~o~cíais";
    impf_3: _, "~o~cían";
    pret_1: "~o~cí", "~o~cimos";
    pret_2: "~o~ciste", "~o~cisteis";
    pret_3: "~o~ció", "~o~cieron";
    fut_1: "~o~ceré", "~o~ceremos";
    fut_2: "~o~cerás", "~o~ceréis";
    fut_3: "~o~cerá", "~o~cerán";
    cond_1: "~o~cería", "~o~ceríamos";
    cond_2: "~o~cerías", "~o~ceríais";
    cond_3: _, "~o~cerían";
    subj_1: "~ue~za", "~o~zamos";
    subj_2: "~ue~zas", "~o~záis";
    subj_3: _, "~ue~zan";
    subj_impf_1: "~o~ciera", "~o~ciéramos";
    subj_impf_2: "~o~cieras", "~o~cierais";
    subj_impf_3: _, "~o~cieran";
    subj_impf_se_1: "~o~ciese", "~o~ciésemos";
    subj_impf_se_2: "~o~cieses", "~o~cieseis";
    subj_impf_se_3: _, "~o~ciesen";
    subj_fut_1: "~o~ciere", "~o~ciéremos";
    subj_fut_2: "~o~cieres", "~o~ciereis";
    subj_fut_3: _, "~o~cieren";
    vos: "~o~cés", "~o~zás", "~o~cé";
    imperative: "~ue~ce", "~ue~za", "~o~zamos", "~o~ced", "~ue~zan";
    infinitive: "~o~cer";
    gerund: "~o~ciendo";
    ),
    // Desforzarse: 4 words (desforzarse, forzarse, esforzarse, reforzarse). Like Forzar, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("desforzarse", 4, "~o~zarse", like Forzar;
    vos: _, _, "~o~zate";
    imperative: "~ué~zate", "~ué~cese", "~o~cémonos", "~o~zaos", "~ué~cense";
    infinitive: "~o~zarse";
    gerund: "~o~zándose";
    ),
    // Regirse: 4 words (autocorregirse, autoelegirse, corregirse, regirse). Like Regir, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("regirse", 4, "~egirse", like Regir;
    vos: _, _, "~egite";
    imperative: "~ígete", "~íjase", "~ijámonos", "~egíos", "~íjanse";
    infinitive: "~egirse";
    gerund: "~igiéndose";
    ),
    // Fraguarse: 4 words (paniaguarse, sobreaguarse, desaguarse, fraguarse).
    conjugation!("fraguarse", 4, "~a~uarse";
    stem participle: "~a~uad- like echar";
    pres_1: "~a~uo", "~a~uamos";
    pres_2: "~a~uas", "~a~uáis";
    pres_3: "~a~ua", "~a~uan";
    impf_1: "~a~uaba", "~a~uábamos";
    impf_2: "~a~uabas", "~a~uabais";
    impf_3: _, "~a~uaban";
    pret_1: "~a~üé", "~a~uamos";
    pret_2: "~a~uaste", "~a~uasteis";
    pret_3: "~a~uó", "~a~uaron";
    fut_1: "~a~uaré", "~a~uaremos";
    fut_2: "~a~uarás", "~a~uaréis";
    fut_3: "~a~uará", "~a~uarán";
    cond_1: "~a~uaría", "~a~uaríamos";
    cond_2: "~a~uarías", "~a~uaríais";
    cond_3: _, "~a~uarían";
    subj_1: "~a~üe", "~a~üemos";
    subj_2: "~a~ües", "~a~üéis";
    subj_3: _, "~a~üen";
    subj_impf_1: "~a~uara", "~a~uáramos";
    subj_impf_2: "~a~uaras", "~a~uarais";
    subj_impf_3: _, "~a~uaran";
    subj_impf_se_1: "~a~uase", "~a~uásemos";
    subj_impf_se_2: "~a~uases", "~a~uaseis";
    subj_impf_se_3: _, "~a~uasen";
    subj_fut_1: "~a~uare", "~a~uáremos";
    subj_fut_2: "~a~uares", "~a~uareis";
    subj_fut_3: _, "~a~uaren";
    vos: "~a~uás", "~a~üés", "~a~uate";
    imperative: "~á~uate", "~á~üese", "~a~üémonos", "~a~uaos", "~á~üense";
    infinitive: "~a~uarse";
    gerund: "~a~uándose";
    ),
    // Moverse: 4 words (lloverse, conmoverse, removerse, moverse). Like Mover, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("moverse", 4, "~overse", like Mover;
    vos: _, _, "~ovete";
    imperative: "~uévete", "~uévase", "~ovámonos", "~oveos", "~uévanse";
    infinitive: "~overse";
    gerund: "~oviéndose";
    ),
    // Verse: 4 words (reverse, verse, arderse, entreverse). Like Ver, except in 9 cells: vos ind, vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, ….
    conjugation!("verse", 4, "~erse", like Ver;
    vos: "~és", _, "~ete";
    imperative: "~ete", "~éase", "~eámonos", "~eos", "~éanse";
    infinitive: "~erse";
    gerund: "~iéndose";
    ),
    // Criarse: 4 words (criarse, liarse, triarse, fiarse). Like Resfriarse, except in 6 cells: pres_2 pl, pret_1 sg, pret_3 sg, subj_2 pl, vos ind, vos subj.
    conjugation!("criarse", 4, "~iarse", like Resfriarse;
    pres_2: _, "~iais";
    pret_1: "~ie", _;
    pret_3: "~io", _;
    subj_2: _, "~ieis";
    vos: "~ias", "~ies", _;
    ),
    // Punzarse: 4 words (anarquizarse, barroquizarse, endulzarse, punzarse).
    conjugation!("punzarse", 4, "~u~zarse";
    stem participle: "~u~zad- like echar";
    pres_1: "~u~zo", "~u~zamos";
    pres_2: "~u~zas", "~u~záis";
    pres_3: "~u~za", "~u~zan";
    impf_1: "~u~zaba", "~u~zábamos";
    impf_2: "~u~zabas", "~u~zabais";
    impf_3: _, "~u~zaban";
    pret_1: "~u~cé", "~u~zamos";
    pret_2: "~u~zaste", "~u~zasteis";
    pret_3: "~u~zó", "~u~zaron";
    fut_1: "~u~zaré", "~u~zaremos";
    fut_2: "~u~zarás", "~u~zaréis";
    fut_3: "~u~zará", "~u~zarán";
    cond_1: "~u~zaría", "~u~zaríamos";
    cond_2: "~u~zarías", "~u~zaríais";
    cond_3: _, "~u~zarían";
    subj_1: "~u~ce", "~u~cemos";
    subj_2: "~u~ces", "~u~céis";
    subj_3: _, "~u~cen";
    subj_impf_1: "~u~zara", "~u~záramos";
    subj_impf_2: "~u~zaras", "~u~zarais";
    subj_impf_3: _, "~u~zaran";
    subj_impf_se_1: "~u~zase", "~u~zásemos";
    subj_impf_se_2: "~u~zases", "~u~zaseis";
    subj_impf_se_3: _, "~u~zasen";
    subj_fut_1: "~u~zare", "~u~záremos";
    subj_fut_2: "~u~zares", "~u~zareis";
    subj_fut_3: _, "~u~zaren";
    vos: "~u~zás", "~u~cés", "~u~zate";
    imperative: "~ú~zate", "~ú~cese", "~u~cémonos", "~u~zaos", "~ú~cense";
    infinitive: "~u~zarse";
    gerund: "~u~zándose";
    ),
    // Traslucirse: 4 words (traslucirse, translucirse, deslucirse, lucirse). Like Musirse, except in 10 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("traslucirse", 4, "~u~irse", like Musirse;
    pres_1: "~uz~o", _;
    subj_1: "~uz~a", "~uz~amos";
    subj_2: "~uz~as", "~uz~áis";
    subj_3: _, "~uz~an";
    vos: _, "~uz~ás", _;
    imperative: _, "~úz~ase", "~uz~ámonos", _, "~úz~anse";
    ),
    // Torcerse: 4 words (contorcerse, retorcerse, torcerse, destorcerse). Like Torcer, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("torcerse", 4, "~o~cerse", like Torcer;
    vos: _, _, "~o~cete";
    imperative: "~ué~cete", "~ué~zase", "~o~zámonos", "~o~ceos", "~ué~zanse";
    infinitive: "~o~cerse";
    gerund: "~o~ciéndose";
    ),
    // Servirse: 4 words (rendirse, rehenchirse, henchirse, servirse). Like Rendir, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("servirse", 4, "~e~irse", like Rendir;
    vos: _, _, "~e~íte";
    imperative: "~i~ete", "~i~ase", "~i~ámonos", "~e~íos", "~i~anse";
    infinitive: "~e~irse";
    gerund: "~i~iéndose";
    ),
    // Vencerse: 4 words (remecerse, convencerse, vencerse, desconvencerse).
    conjugation!("vencerse", 4, "~e~cerse";
    stem participle: "~e~cid- like echar";
    pres_1: "~e~zo", "~e~cemos";
    pres_2: "~e~ces", "~e~céis";
    pres_3: "~e~ce", "~e~cen";
    impf_1: "~e~cía", "~e~cíamos";
    impf_2: "~e~cías", "~e~cíais";
    impf_3: _, "~e~cían";
    pret_1: "~e~cí", "~e~cimos";
    pret_2: "~e~ciste", "~e~cisteis";
    pret_3: "~e~ció", "~e~cieron";
    fut_1: "~e~ceré", "~e~ceremos";
    fut_2: "~e~cerás", "~e~ceréis";
    fut_3: "~e~cerá", "~e~cerán";
    cond_1: "~e~cería", "~e~ceríamos";
    cond_2: "~e~cerías", "~e~ceríais";
    cond_3: _, "~e~cerían";
    subj_1: "~e~za", "~e~zamos";
    subj_2: "~e~zas", "~e~záis";
    subj_3: _, "~e~zan";
    subj_impf_1: "~e~ciera", "~e~ciéramos";
    subj_impf_2: "~e~cieras", "~e~cierais";
    subj_impf_3: _, "~e~cieran";
    subj_impf_se_1: "~e~ciese", "~e~ciésemos";
    subj_impf_se_2: "~e~cieses", "~e~cieseis";
    subj_impf_se_3: _, "~e~ciesen";
    subj_fut_1: "~e~ciere", "~e~ciéremos";
    subj_fut_2: "~e~cieres", "~e~ciereis";
    subj_fut_3: _, "~e~cieren";
    vos: "~e~cés", "~e~zás", "~e~cete";
    imperative: "~é~cete", "~é~zase", "~e~zámonos", "~e~ceos", "~é~zanse";
    infinitive: "~e~cerse";
    gerund: "~e~ciéndose";
    ),
    // Yacer: 3 words (yacer, subyacer, adyacer). Like Vencer, except in 12 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("yacer", 3, "~cer", like Vencer;
    pres_1: "~zco|~zgo|~go", _;
    subj_1: "~zca|~zga|~ga", "~zcamos|~zgamos|~gamos";
    subj_2: "~zcas|~zgas|~gas|~zcás|~zgás", "~zcáis|~zgáis|~gáis";
    subj_3: _, "~zcan|~zgan|~gan";
    vos: _, "~gás", _;
    imperative: "~ce|~z", "~zca|~zga|~ga", "~zcamos|~zgamos|~gamos", _, "~zcan|~zgan|~gan";
    negative_imperative: "~zcas|~zgas|~gas", _, _, _, _;
    ),
    // Argüir: 3 words (argüir, reargüir, redargüir). Like Inmiscuirse, except in 31 cells: pres_1 pl, pres_2 pl, impf_1 sg, impf_1 pl, impf_2 sg, impf_2 pl, ….
    conjugation!("argüir", 3, "~üir", like Inmiscuirse;
    stem participle: "~üid- like echar";
    pres_1: _, "~üimos";
    pres_2: _, "~üís";
    impf_1: "~üía", "~üíamos";
    impf_2: "~üías", "~üíais";
    impf_3: _, "~üían";
    pret_1: "~üí", "~üimos";
    pret_2: "~üiste", "~üisteis";
    fut_1: "~üiré", "~üiremos";
    fut_2: "~üirás", "~üiréis";
    fut_3: "~üirá", "~üirán";
    cond_1: "~üiría", "~üiríamos";
    cond_2: "~üirías", "~üiríais";
    cond_3: _, "~üirían";
    vos: "~üís", _, "~üí";
    imperative: "~uye", "~uya", "~uyamos", "~üid", "~uyan";
    infinitive: "~üir";
    gerund: "~uyendo";
    ),
    // Jugar: 3 words (jugar, rejugar, sobrejugar). Like Cerrar, except in 15 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, pret_1 sg, subj_1 sg, ….
    conjugation!("jugar", 3, "~~ar", like Cerrar;
    pres_1: "~e~o", _;
    pres_2: "~e~as", _;
    pres_3: "~e~a", "~e~an";
    pret_1: "~~ué", _;
    subj_1: "~e~ue", "~~uemos";
    subj_2: "~e~ues", "~~uéis";
    subj_3: _, "~e~uen";
    vos: _, "~~ués", _;
    imperative: "~e~a", "~e~ue", "~~uemos", _, "~e~uen";
    ),
    // Romper: 3 words (romper, derromper, arromper).
    conjugation!("romper", 3, "~mper";
    stem participle: "~t- like echar";
    pres_1: "~mpo", "~mpemos";
    pres_2: "~mpes", "~mpéis";
    pres_3: "~mpe", "~mpen";
    impf_1: "~mpía", "~mpíamos";
    impf_2: "~mpías", "~mpíais";
    impf_3: _, "~mpían";
    pret_1: "~mpí", "~mpimos";
    pret_2: "~mpiste", "~mpisteis";
    pret_3: "~mpió", "~mpieron";
    fut_1: "~mperé", "~mperemos";
    fut_2: "~mperás", "~mperéis";
    fut_3: "~mperá", "~mperán";
    cond_1: "~mpería", "~mperíamos";
    cond_2: "~mperías", "~mperíais";
    cond_3: _, "~mperían";
    subj_1: "~mpa", "~mpamos";
    subj_2: "~mpas", "~mpáis";
    subj_3: _, "~mpan";
    subj_impf_1: "~mpiera", "~mpiéramos";
    subj_impf_2: "~mpieras", "~mpierais";
    subj_impf_3: _, "~mpieran";
    subj_impf_se_1: "~mpiese", "~mpiésemos";
    subj_impf_se_2: "~mpieses", "~mpieseis";
    subj_impf_se_3: _, "~mpiesen";
    subj_fut_1: "~mpiere", "~mpiéremos";
    subj_fut_2: "~mpieres", "~mpiereis";
    subj_fut_3: _, "~mpieren";
    vos: "~mpés", "~mpás", "~mpé";
    imperative: "~mpe", "~mpa", "~mpamos", "~mped", "~mpan";
    infinitive: "~mper";
    gerund: "~mpiendo";
    ),
    // Valer: 3 words (valer, equivaler, prevaler). Like Salir, except in 7 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative tu, imperative vosotros, ….
    conjugation!("valer", 3, "~er", like Salir;
    pres_1: _, "~emos";
    pres_2: _, "~éis";
    vos: "~és", _, "~é";
    imperative: "~e", _, _, "~ed", _;
    infinitive: "~er";
    ),
    // Apucuyar: 3 words (intrusar, apucuyar, empuyar). Like Emputarse, except in 0 cells: .
    conjugation!("apucuyar", 3, "~u~ar", like Emputarse;
    ),
    // Antojar: 3 words (antojar, hopar, ñangotar). Like Acomodarse, except in 0 cells: .
    conjugation!("antojar", 3, "~o~ar", like Acomodarse;
    ),
    // Enraizarse: 3 words (enraizarse, europeizarse, corporeizarse). Like Fosilizarse, except in 7 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("enraizarse", 3, "~izarse", like Fosilizarse;
    pres_1: "~ízo", _;
    pres_2: "~ízas", _;
    pres_3: "~íza", "~ízan";
    subj_1: "~íce", _;
    subj_2: "~íces", _;
    subj_3: _, "~ícen";
    ),
    // Escocerse: 3 words (escocerse, recocerse, cocerse). Like Cocer, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("escocerse", 3, "~ocerse", like Cocer;
    stem participle: "~oc- like escocerse";
    vos: _, _, "~ocete";
    imperative: "~uécete", "~uézase", "~ozámonos", "~oceos", "~uézanse";
    infinitive: "~ocerse";
    gerund: "~ociéndose";
    ),
    // Seguirse: 3 words (conseguirse, subseguirse, seguirse). Like Seguir, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("seguirse", 3, "~eguirse", like Seguir;
    vos: _, _, "~eguite";
    imperative: "~íguete", "~ígase", "~igámonos", "~eguíos", "~íganse";
    infinitive: "~eguirse";
    gerund: "~iguiéndose";
    ),
    // Azoarse: 3 words (abarloarse, azoarse, barloarse).
    conjugation!("azoarse", 3, "~oarse";
    stem participle: "~oad- like echar";
    pres_1: "~oo", "~oamos";
    pres_2: "~oas", "~oáis";
    pres_3: "~oa", "~oan";
    impf_1: "~oaba", "~oábamos";
    impf_2: "~oabas", "~oabais";
    impf_3: _, "~oaban";
    pret_1: "~oé", "~oamos";
    pret_2: "~oaste", "~oasteis";
    pret_3: "~oó", "~oaron";
    fut_1: "~oaré", "~oaremos";
    fut_2: "~oarás", "~oaréis";
    fut_3: "~oará", "~oarán";
    cond_1: "~oaría", "~oaríamos";
    cond_2: "~oarías", "~oaríais";
    cond_3: _, "~oarían";
    subj_1: "~oe", "~oemos";
    subj_2: "~oes", "~oéis";
    subj_3: _, "~oen";
    subj_impf_1: "~oara", "~oáramos";
    subj_impf_2: "~oaras", "~oarais";
    subj_impf_3: _, "~oaran";
    subj_impf_se_1: "~oase", "~oásemos";
    subj_impf_se_2: "~oases", "~oaseis";
    subj_impf_se_3: _, "~oasen";
    subj_fut_1: "~oare", "~oáremos";
    subj_fut_2: "~oares", "~oareis";
    subj_fut_3: _, "~oaren";
    vos: "~oás", "~oés", "~oate";
    imperative: "~óate", "~óese", "~oémonos", "~oaos", "~óense";
    infinitive: "~oarse";
    gerund: "~oándose";
    ),
    // Nacerse: 3 words (complacerse, aplacerse, nacerse).
    conjugation!("nacerse", 3, "~acerse";
    stem verb: "~- like nacerse";
    stem participle: "~acid- like echar";
    pres_1: "~azco", "~acemos";
    pres_2: "~aces", "~acéis";
    pres_3: "~ace", "~acen";
    impf_1: "~acía", "~acíamos";
    impf_2: "~acías", "~acíais";
    impf_3: _, "~acían";
    fut_1: "~aceré", "~aceremos";
    fut_2: "~acerás", "~aceréis";
    fut_3: "~acerá", "~acerán";
    cond_1: "~acería", "~aceríamos";
    cond_2: "~acerías", "~aceríais";
    cond_3: _, "~acerían";
    subj_1: "~azca", "~azcamos";
    subj_2: "~azcas", "~azcáis";
    subj_3: _, "~azcan";
    vos: "~acés", "~azcás", "~acete";
    imperative: "~ácete", "~ázcase", "~azcámonos", "~aceos", "~ázcanse";
    infinitive: "~acerse";
    gerund: "~aciéndose";
    ),
    // Trenzarse: 3 words (trenzarse, destrenzarse, homogeneizarse).
    conjugation!("trenzarse", 3, "~e~zarse";
    stem participle: "~e~zad- like echar";
    pres_1: "~e~zo", "~e~zamos";
    pres_2: "~e~zas", "~e~záis";
    pres_3: "~e~za", "~e~zan";
    impf_1: "~e~zaba", "~e~zábamos";
    impf_2: "~e~zabas", "~e~zabais";
    impf_3: _, "~e~zaban";
    pret_1: "~e~cé", "~e~zamos";
    pret_2: "~e~zaste", "~e~zasteis";
    pret_3: "~e~zó", "~e~zaron";
    fut_1: "~e~zaré", "~e~zaremos";
    fut_2: "~e~zarás", "~e~zaréis";
    fut_3: "~e~zará", "~e~zarán";
    cond_1: "~e~zaría", "~e~zaríamos";
    cond_2: "~e~zarías", "~e~zaríais";
    cond_3: _, "~e~zarían";
    subj_1: "~e~ce", "~e~cemos";
    subj_2: "~e~ces", "~e~céis";
    subj_3: _, "~e~cen";
    subj_impf_1: "~e~zara", "~e~záramos";
    subj_impf_2: "~e~zaras", "~e~zarais";
    subj_impf_3: _, "~e~zaran";
    subj_impf_se_1: "~e~zase", "~e~zásemos";
    subj_impf_se_2: "~e~zases", "~e~zaseis";
    subj_impf_se_3: _, "~e~zasen";
    subj_fut_1: "~e~zare", "~e~záremos";
    subj_fut_2: "~e~zares", "~e~zareis";
    subj_fut_3: _, "~e~zaren";
    vos: "~e~zás", "~e~cés", "~e~zate";
    imperative: "~é~zate", "~é~cese", "~e~cémonos", "~e~zaos", "~é~cense";
    infinitive: "~e~zarse";
    gerund: "~e~zándose";
    ),
    // Cubrirse: 3 words (descubrirse, cubrirse, encubrirse).
    conjugation!("cubrirse", 3, "~u~~irse";
    stem participle: "~u~ie~t- like echar";
    pres_1: "~u~~o", "~u~~imos";
    pres_2: "~u~~es", "~u~~ís";
    pres_3: "~u~~e", "~u~~en";
    impf_1: "~u~~ía", "~u~~íamos";
    impf_2: "~u~~ías", "~u~~íais";
    impf_3: _, "~u~~ían";
    pret_1: "~u~~í", "~u~~imos";
    pret_2: "~u~~iste", "~u~~isteis";
    pret_3: "~u~~ió", "~u~~ieron";
    fut_1: "~u~~iré", "~u~~iremos";
    fut_2: "~u~~irás", "~u~~iréis";
    fut_3: "~u~~irá", "~u~~irán";
    cond_1: "~u~~iría", "~u~~iríamos";
    cond_2: "~u~~irías", "~u~~iríais";
    cond_3: _, "~u~~irían";
    subj_1: "~u~~a", "~u~~amos";
    subj_2: "~u~~as", "~u~~áis";
    subj_3: _, "~u~~an";
    subj_impf_1: "~u~~iera", "~u~~iéramos";
    subj_impf_2: "~u~~ieras", "~u~~ierais";
    subj_impf_3: _, "~u~~ieran";
    subj_impf_se_1: "~u~~iese", "~u~~iésemos";
    subj_impf_se_2: "~u~~ieses", "~u~~ieseis";
    subj_impf_se_3: _, "~u~~iesen";
    subj_fut_1: "~u~~iere", "~u~~iéremos";
    subj_fut_2: "~u~~ieres", "~u~~iereis";
    subj_fut_3: _, "~u~~ieren";
    vos: "~u~~ís", "~u~~ás", "~u~~ite";
    imperative: "~ú~~ete", "~ú~~ase", "~u~~ámonos", "~u~~íos", "~ú~~anse";
    infinitive: "~u~~irse";
    gerund: "~u~~iéndose";
    ),
    // Derretirse: 3 words (derretirse, competirse, repetirse).
    conjugation!("derretirse", 3, "~etirse";
    stem participle: "~etid- like echar";
    pres_1: "~ito", "~etimos";
    pres_2: "~ites", "~etís";
    pres_3: "~ite", "~iten";
    impf_1: "~etía", "~etíamos";
    impf_2: "~etías", "~etíais";
    impf_3: _, "~etían";
    pret_1: "~etí", "~etimos";
    pret_2: "~etiste", "~etisteis";
    pret_3: "~itió", "~itieron";
    fut_1: "~etiré", "~etiremos";
    fut_2: "~etirás", "~etiréis";
    fut_3: "~etirá", "~etirán";
    cond_1: "~etiría", "~etiríamos";
    cond_2: "~etirías", "~etiríais";
    cond_3: _, "~etirían";
    subj_1: "~ita", "~itamos";
    subj_2: "~itas", "~itáis";
    subj_3: _, "~itan";
    subj_impf_1: "~itiera", "~itiéramos";
    subj_impf_2: "~itieras", "~itierais";
    subj_impf_3: _, "~itieran";
    subj_impf_se_1: "~itiese", "~itiésemos";
    subj_impf_se_2: "~itieses", "~itieseis";
    subj_impf_se_3: _, "~itiesen";
    subj_fut_1: "~itiere", "~itiéremos";
    subj_fut_2: "~itieres", "~itiereis";
    subj_fut_3: _, "~itieren";
    vos: "~etís", "~itás", "~etite";
    imperative: "~ítete", "~ítase", "~itámonos", "~etíos", "~ítanse";
    infinitive: "~etirse";
    gerund: "~itiéndose";
    ),
    // Fingirse: 3 words (refringirse, fingirse, atingirse).
    conjugation!("fingirse", 3, "~i~girse";
    stem participle: "~i~gid- like echar";
    pres_1: "~i~jo", "~i~gimos";
    pres_2: "~i~ges", "~i~gís";
    pres_3: "~i~ge", "~i~gen";
    impf_1: "~i~gía", "~i~gíamos";
    impf_2: "~i~gías", "~i~gíais";
    impf_3: _, "~i~gían";
    pret_1: "~i~gí", "~i~gimos";
    pret_2: "~i~giste", "~i~gisteis";
    pret_3: "~i~gió", "~i~gieron";
    fut_1: "~i~giré", "~i~giremos";
    fut_2: "~i~girás", "~i~giréis";
    fut_3: "~i~girá", "~i~girán";
    cond_1: "~i~giría", "~i~giríamos";
    cond_2: "~i~girías", "~i~giríais";
    cond_3: _, "~i~girían";
    subj_1: "~i~ja", "~i~jamos";
    subj_2: "~i~jas", "~i~jáis";
    subj_3: _, "~i~jan";
    subj_impf_1: "~i~giera", "~i~giéramos";
    subj_impf_2: "~i~gieras", "~i~gierais";
    subj_impf_3: _, "~i~gieran";
    subj_impf_se_1: "~i~giese", "~i~giésemos";
    subj_impf_se_2: "~i~gieses", "~i~gieseis";
    subj_impf_se_3: _, "~i~giesen";
    subj_fut_1: "~i~giere", "~i~giéremos";
    subj_fut_2: "~i~gieres", "~i~giereis";
    subj_fut_3: _, "~i~gieren";
    vos: "~i~gís", "~i~jás", "~i~gite";
    imperative: "~í~gete", "~í~jase", "~i~jámonos", "~i~gíos", "~í~janse";
    infinitive: "~i~girse";
    gerund: "~i~giéndose";
    ),
    // Azonzarse: 3 words (azonzarse, desgonzarse, tronzarse). Like Desforzarse, except in 10 cells: pres_1 sg, pres_2 sg, pres_3 sg, pres_3 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("azonzarse", 3, "~o~zarse", like Desforzarse;
    pres_1: "~o~zo", _;
    pres_2: "~o~zas", _;
    pres_3: "~o~za", "~o~zan";
    subj_1: "~o~ce", _;
    subj_2: "~o~ces", _;
    subj_3: _, "~o~cen";
    imperative: "~ó~zate", "~ó~cese", _, _, "~ó~cense";
    ),
    // Asirse: 2 words (abrirse, asirse).
    conjugation!("asirse", 2, "a~irse";
    stem participle: "~id- like armarse";
    pres_1: "a~go", "a~imos";
    pres_2: "a~es", "a~ís";
    pres_3: "a~e", "a~en";
    impf_1: "a~ía", "a~íamos";
    impf_2: "a~ías", "a~íais";
    impf_3: _, "a~ían";
    pret_1: "a~í", "a~imos";
    pret_2: "a~iste", "a~isteis";
    pret_3: "a~ió", "a~ieron";
    fut_1: "a~iré", "a~iremos";
    fut_2: "a~irás", "a~iréis";
    fut_3: "a~irá", "a~irán";
    cond_1: "a~iría", "a~iríamos";
    cond_2: "a~irías", "a~iríais";
    cond_3: _, "a~irían";
    subj_1: "a~ga", "a~gamos";
    subj_2: "a~gas", "a~gáis";
    subj_3: _, "a~gan";
    subj_impf_1: "a~iera", "a~iéramos";
    subj_impf_2: "a~ieras", "a~ierais";
    subj_impf_3: _, "a~ieran";
    subj_impf_se_1: "a~iese", "a~iésemos";
    subj_impf_se_2: "a~ieses", "a~ieseis";
    subj_impf_se_3: _, "a~iesen";
    subj_fut_1: "a~iere", "a~iéremos";
    subj_fut_2: "a~ieres", "a~iereis";
    subj_fut_3: _, "a~ieren";
    vos: "a~ís", "a~gás", "a~ite";
    imperative: "á~ete", "á~gase", "a~gámonos", "a~íos", "á~ganse";
    infinitive: "a~irse";
    gerund: "a~iéndose";
    ),
    // Reírse: 1 words (embaírse). Like Creerse, except in 17 cells: pres_1 pl, pres_2 pl, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, ….
    conjugation!("reírse", 1, "~~írse", like Creerse;
    pres_1: _, "~~ímos";
    pres_2: _, "~~ís";
    fut_1: "~~iré", "~~iremos";
    fut_2: "~~irás", "~~iréis";
    fut_3: "~~irá", "~~irán";
    cond_1: "~~iría", "~~iríamos";
    cond_2: "~~irías", "~~iríais";
    cond_3: _, "~~irían";
    vos: "~~ís", _, "~~íte";
    imperative: _, _, _, "~~íos", _;
    infinitive: "~~írse";
    ),
    // Ir: 1 words (ir).
    conjugation!("ir", 1, "ir";
    pres_1: "voy", "vamos";
    pres_2: "vas", "vais";
    pres_3: "va", "van";
    impf_1: "iba", "íbamos";
    impf_2: "ibas", "ibais";
    impf_3: _, "iban";
    pret_1: "fui", "fuimos";
    pret_2: "fuiste", "fuisteis";
    pret_3: "fue", "fueron";
    fut_1: "iré", "iremos";
    fut_2: "irás", "iréis";
    fut_3: "irá", "irán";
    cond_1: "iría", "iríamos";
    cond_2: "irías", "iríais";
    cond_3: _, "irían";
    subj_1: "vaya", "vayamos";
    subj_2: "vayas", "vayáis";
    subj_3: _, "vayan";
    subj_impf_1: "fuera", "fuéramos";
    subj_impf_2: "fueras", "fuerais";
    subj_impf_3: _, "fueran";
    subj_impf_se_1: "fuese", "fuésemos";
    subj_impf_se_2: "fueses", "fueseis";
    subj_impf_se_3: _, "fuesen";
    subj_fut_1: "fuere", "fuéremos";
    subj_fut_2: "fueres", "fuereis";
    subj_fut_3: _, "fueren";
    vos: _, "vayás", "andá";
    imperative: "ve", "vaya", "vamos|vayamos", "id", "vayan";
    infinitive: "ir";
    gerund: "yendo";
    participle_m: "ido", "idos";
    participle_f: "ida", "idas";
    ),
    // Ingrirse: 1 words (ingrirse).
    conjugation!("ingrirse", 1, "i~irse";
    stem participle: "~id- like ingrirse";
    pres_1: "i~o", "i~imos";
    pres_2: "i~es", "i~ís";
    pres_3: "i~e", "i~en";
    impf_1: "i~ía", "i~íamos";
    impf_2: "i~ías", "i~íais";
    impf_3: _, "i~ían";
    pret_1: "i~í", "i~imos";
    pret_2: "i~iste", "i~isteis";
    pret_3: "i~ió", "i~ieron";
    fut_1: "i~iré", "i~iremos";
    fut_2: "i~irás", "i~iréis";
    fut_3: "i~irá", "i~irán";
    cond_1: "i~iría", "i~iríamos";
    cond_2: "i~irías", "i~iríais";
    cond_3: _, "i~irían";
    subj_1: "i~a", "i~amos";
    subj_2: "i~as", "i~áis";
    subj_3: _, "i~an";
    subj_impf_1: "i~iera", "i~iéramos";
    subj_impf_2: "i~ieras", "i~ierais";
    subj_impf_3: _, "i~ieran";
    subj_impf_se_1: "i~iese", "i~iésemos";
    subj_impf_se_2: "i~ieses", "i~ieseis";
    subj_impf_se_3: _, "i~iesen";
    subj_fut_1: "i~iere", "i~iéremos";
    subj_fut_2: "i~ieres", "i~iereis";
    subj_fut_3: _, "i~ieren";
    vos: "i~ís", "i~ás", "i~ite";
    imperative: "í~ete", "í~ase", "i~ámonos", "i~íos", "í~anse";
    infinitive: "i~irse";
    gerund: "i~iéndose";
    ),
    // Irse: 1 words (irse). Like Ir, except in 9 cells: vos ind, vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, ….
    conjugation!("irse", 1, "irse", like Ir;
    vos: "vas", _, "andate";
    imperative: "vete|ándate", "váyase", "vayámonos|vámonos", "idos", "váyanse";
    infinitive: "irse";
    gerund: "yéndose";
    ),
    // Unirse: 1 words (unirse).
    conjugation!("unirse", 1, "u~irse";
    stem participle: "~id- like unirse";
    pres_1: "u~o", "u~imos";
    pres_2: "u~es", "u~ís";
    pres_3: "u~e", "u~en";
    impf_1: "u~ía", "u~íamos";
    impf_2: "u~ías", "u~íais";
    impf_3: _, "u~ían";
    pret_1: "u~í", "u~imos";
    pret_2: "u~iste", "u~isteis";
    pret_3: "u~ió", "u~ieron";
    fut_1: "u~iré", "u~iremos";
    fut_2: "u~irás", "u~iréis";
    fut_3: "u~irá", "u~irán";
    cond_1: "u~iría", "u~iríamos";
    cond_2: "u~irías", "u~iríais";
    cond_3: _, "u~irían";
    subj_1: "u~a", "u~amos";
    subj_2: "u~as", "u~áis";
    subj_3: _, "u~an";
    subj_impf_1: "u~iera", "u~iéramos";
    subj_impf_2: "u~ieras", "u~ierais";
    subj_impf_3: _, "u~ieran";
    subj_impf_se_1: "u~iese", "u~iésemos";
    subj_impf_se_2: "u~ieses", "u~ieseis";
    subj_impf_se_3: _, "u~iesen";
    subj_fut_1: "u~iere", "u~iéremos";
    subj_fut_2: "u~ieres", "u~iereis";
    subj_fut_3: _, "u~ieren";
    vos: "u~ís", "u~ás", "u~ite";
    imperative: "ú~ete", "ú~ase", "u~ámonos", "u~íos", "ú~anse";
    infinitive: "u~irse";
    gerund: "u~iéndose";
    ),
];
