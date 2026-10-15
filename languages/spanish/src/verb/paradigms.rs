//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
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
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// What a conjugation adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
macro_rules! affix {
    () => {
        None
    };
    ($affix:literal) => {
        affix!($affix, $affix, "")
    };
    ($affix:literal, $other:literal, $letters:literal) => {
        Some(flexion::Affix {
            pattern: ::std::borrow::Cow::Borrowed($affix),
            other: ::std::borrow::Cow::Borrowed($other),
            letters: ::std::borrow::Cow::Borrowed($letters),
        })
    };
}

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
    /// Like `conocer`: 330 words.
    Conocer,
    /// Like `curtir`: 328 words.
    Curtir,
    /// Like `sacar`: 250 words.
    Sacar,
    /// Like `fosilizarse`: 249 words.
    Fosilizarse,
    /// Like `autoabastecerse`: 176 words.
    Autoabastecerse,
    /// Like `meter`: 145 words.
    Meter,
    /// Like `osificarse`: 134 words.
    Osificarse,
    /// Like `cerrar`: 132 words.
    Cerrar,
    /// Like `darse`: 115 words.
    Darse,
    /// Like `desviar`: 109 words.
    Desviar,
    /// Like `instruir`: 105 words.
    Instruir,
    /// Like `sonar`: 80 words.
    Sonar,
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
    /// Like `volar`: 41 words.
    Volar,
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
    /// Like `regar`: 25 words.
    Regar,
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
    /// Like `chuzarse`: 19 words.
    Chuzarse,
    /// Like `resentirse`: 19 words.
    Resentirse,
    /// Like `fruncir`: 18 words.
    Fruncir,
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
    /// Like `moler`: 12 words.
    Moler,
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
    /// Like `empezar`: 7 words.
    Empezar,
    /// Like `regir`: 7 words.
    Regir,
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
    /// Like `mover`: 6 words.
    Mover,
    /// Like `seguir`: 6 words.
    Seguir,
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
    /// Like `agorar`: 4 words.
    Agorar,
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
    /// Like `comenzar`: 4 words.
    Comenzar,
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
    pub const ALL: [Conjugation; 162] = [
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
        Conjugation::Meter,
        Conjugation::Osificarse,
        Conjugation::Cerrar,
        Conjugation::Darse,
        Conjugation::Desviar,
        Conjugation::Instruir,
        Conjugation::Sonar,
        Conjugation::Atravesarse,
        Conjugation::Sentir,
        Conjugation::Situar,
        Conjugation::Desencontrarse,
        Conjugation::Musirse,
        Conjugation::Dirigir,
        Conjugation::Volar,
        Conjugation::Resfriarse,
        Conjugation::Poner,
        Conjugation::Fraguar,
        Conjugation::Rendir,
        Conjugation::Descocarse,
        Conjugation::Desvivirse,
        Conjugation::Situarse,
        Conjugation::Embazarse,
        Conjugation::Destoserse,
        Conjugation::Regar,
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
        Conjugation::Chuzarse,
        Conjugation::Resentirse,
        Conjugation::Fruncir,
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
        Conjugation::Moler,
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
        Conjugation::Empezar,
        Conjugation::Regir,
        Conjugation::Ver,
        Conjugation::Tañer,
        Conjugation::Ahumarse,
        Conjugation::Venirse,
        Conjugation::Aterirse,
        Conjugation::Jetiarse,
        Conjugation::Deshacerse,
        Conjugation::Relamerse,
        Conjugation::Mover,
        Conjugation::Seguir,
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
        Conjugation::Agorar,
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
        Conjugation::Comenzar,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 162] = [
    // Echar: 9,691 words (estar, amar, generar, crear, dar, abandonar).
    conjugation!("echar", 9691, "~ar";
    stem verb: "~- like echar";
    stem verb_fut_1_sg: "~ar- like echar";
    pres_1: "~o", "~amos";
    pres_2: "~as", "~áis";
    impf_1: "~aba", "~ábamos";
    subj_1: "~e", "~emos";
    subj_2: "~es", "~éis";
    subj_3: _, "~en";
    vos: "~ás", "~és", "~á";
    imperative: "~a", "~e", "~emos", "~ad", "~en";
    infinitive: "~ar";
    gerund: "~ando";
    participle_m: "~ado", _;
    ),
    // Realizar: 1,202 words (profundizar, utilizar, realzar, avergonzar, lanzar, alcanzar). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("realizar", 1202, "~zar", like Echar;
    stem verb: "~- like realizar";
    stem verb_fut_1_sg: "~zar- like echar";
    pres_1: "~zo", "~zamos";
    pres_2: "~zas", "~záis";
    impf_1: "~zaba", "~zábamos";
    subj_1: "~ce", "~cemos";
    subj_2: "~ces", "~céis";
    subj_3: _, "~cen";
    vos: "~zás", "~cés", "~zá";
    imperative: "~za", "~ce", "~cemos", "~zad", "~cen";
    infinitive: "~zar";
    gerund: "~zando";
    participle_m: "~zado", _;
    ),
    // Incautarse: 724 words (llamarse, jambarse, amoratarse, infartarse, autoengañarse, ahorcajarse).
    conjugation!("incautarse", 724, "~a~arse";
    stem verb: "~a~- like echar";
    stem verb_fut_1_sg: "~a~ar- like echar";
    pres_1: "~a~o", "~a~amos";
    pres_2: "~a~as", "~a~áis";
    impf_1: "~a~aba", "~a~ábamos";
    subj_1: "~a~e", "~a~emos";
    subj_2: "~a~es", "~a~éis";
    subj_3: _, "~a~en";
    vos: "~a~ás", "~a~és", "~a~ate";
    imperative: "~á~ate", "~á~ese", "~a~émonos", "~a~aos", "~á~ense";
    infinitive: "~a~arse";
    gerund: "~a~ándose";
    participle_m: "~a~ado", _;
    ),
    // Acomodarse: 591 words (acodarse, acomodarse, endrogarse, personarse, autocontrolarse, amonarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("acomodarse", 591, "~o~arse", like Incautarse;
    stem verb: "~o~- like echar";
    stem verb_fut_1_sg: "~o~ar- like echar";
    pres_1: "~o~o", "~o~amos";
    pres_2: "~o~as", "~o~áis";
    impf_1: "~o~aba", "~o~ábamos";
    subj_1: "~o~e", "~o~emos";
    subj_2: "~o~es", "~o~éis";
    subj_3: _, "~o~en";
    vos: "~o~ás", "~o~és", "~o~ate";
    imperative: "~ó~ate", "~ó~ese", "~o~émonos", "~o~aos", "~ó~ense";
    infinitive: "~o~arse";
    gerund: "~o~ándose";
    participle_m: "~o~ado", _;
    ),
    // Quejarse: 564 words (quejarse, querellarse, despertarse, pegarse, autocremarse, afiebrarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("quejarse", 564, "~e~arse", like Incautarse;
    stem verb: "~e~- like echar";
    stem verb_fut_1_sg: "~e~ar- like echar";
    pres_1: "~e~o", "~e~amos";
    pres_2: "~e~as", "~e~áis";
    impf_1: "~e~aba", "~e~ábamos";
    subj_1: "~e~e", "~e~emos";
    subj_2: "~e~es", "~e~éis";
    subj_3: _, "~e~en";
    vos: "~e~ás", "~e~és", "~e~ate";
    imperative: "~é~ate", "~é~ese", "~e~émonos", "~e~aos", "~é~ense";
    infinitive: "~e~arse";
    gerund: "~e~ándose";
    participle_m: "~e~ado", _;
    ),
    // Rilarse: 548 words (fijarse, chisparse, ababillarse, agüitarse, refanfinflarse, encapricharse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("rilarse", 548, "~i~arse", like Incautarse;
    stem verb: "~i~- like echar";
    stem verb_fut_1_sg: "~i~ar- like echar";
    pres_1: "~i~o", "~i~amos";
    pres_2: "~i~as", "~i~áis";
    impf_1: "~i~aba", "~i~ábamos";
    subj_1: "~i~e", "~i~emos";
    subj_2: "~i~es", "~i~éis";
    subj_3: _, "~i~en";
    vos: "~i~ás", "~i~és", "~i~ate";
    imperative: "~í~ate", "~í~ese", "~i~émonos", "~i~aos", "~í~ense";
    infinitive: "~i~arse";
    gerund: "~i~ándose";
    participle_m: "~i~ado", _;
    ),
    // Emputarse: 398 words (acalenturarse, emputarse, confabularse, autoinculparse, enfurruñarse, despreocuparse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("emputarse", 398, "~u~arse", like Incautarse;
    stem verb: "~u~- like echar";
    stem verb_fut_1_sg: "~u~ar- like echar";
    pres_1: "~u~o", "~u~amos";
    pres_2: "~u~as", "~u~áis";
    impf_1: "~u~aba", "~u~ábamos";
    subj_1: "~u~e", "~u~emos";
    subj_2: "~u~es", "~u~éis";
    subj_3: _, "~u~en";
    vos: "~u~ás", "~u~és", "~u~ate";
    imperative: "~ú~ate", "~ú~ese", "~u~émonos", "~u~aos", "~ú~ense";
    infinitive: "~u~arse";
    gerund: "~u~ándose";
    participle_m: "~u~ado", _;
    ),
    // Regodearse: 358 words (carcajearse, petatearse, autobombearse, regodearse, mamonearse, cachiporrearse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("regodearse", 358, "~~arse", like Incautarse;
    stem verb: "~~- like echar";
    stem verb_fut_1_sg: "~~ar- like echar";
    pres_1: "~~o", "~~amos";
    pres_2: "~~as", "~~áis";
    impf_1: "~~aba", "~~ábamos";
    subj_1: "~~e", "~~emos";
    subj_2: "~~es", "~~éis";
    subj_3: _, "~~en";
    vos: "~~ás", "~~és", "~~ate";
    imperative: "~éat~", "~é~se", "~~émonos", "~~aos", "~é~nse";
    infinitive: "~~arse";
    gerund: "~~ándose";
    participle_m: "~~ado", _;
    ),
    // Conocer: 330 words (conocer, tender, placer, perder, encender, nacer). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("conocer", 330, "~~er", like Incautarse;
    stem verb: "~~- like conocer";
    stem verb_fut_1_sg: "~~er- like echar";
    pres_1: "~z~o", "~~emos";
    pres_2: "~~es", "~~éis";
    impf_1: "~~ía", "~~íamos";
    subj_1: "~z~a", "~z~amos";
    subj_2: "~z~as", "~z~áis";
    subj_3: _, "~z~an";
    vos: "~~és", "~z~ás", "~~é";
    imperative: "~~e", "~z~a", "~z~amos", "~~ed", "~z~an";
    infinitive: "~~er";
    gerund: "~~iendo";
    participle_m: "~~ido", _;
    ),
    // Curtir: 328 words (partir, polir, abolir, cumplir, finir, vivir). Like Echar, except in 21 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("curtir", 328, "~ir", like Echar;
    stem verb: "~- like conocer";
    stem verb_fut_1_sg: "~ir- like echar";
    pres_1: _, "~imos";
    pres_2: "~es", "~ís";
    impf_1: "~ía", "~íamos";
    subj_1: "~a", "~amos";
    subj_2: "~as", "~áis";
    subj_3: _, "~an";
    vos: "~ís", "~ás", "~í";
    imperative: "~e", "~a", "~amos", "~id", "~an";
    infinitive: "~ir";
    gerund: "~iendo";
    participle_m: "~ido", _;
    ),
    // Sacar: 250 words (buscar, sacar, rascar, pecar, tocar, atacar). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("sacar", 250, "~car", like Echar;
    stem verb: "~- like sacar";
    stem verb_fut_1_sg: "~car- like echar";
    pres_1: "~co", "~camos";
    pres_2: "~cas", "~cáis";
    impf_1: "~caba", "~cábamos";
    subj_1: "~que", "~quemos";
    subj_2: "~ques", "~quéis";
    subj_3: _, "~quen";
    vos: "~cás", "~qués", "~cá";
    imperative: "~ca", "~que", "~quemos", "~cad", "~quen";
    infinitive: "~car";
    gerund: "~cando";
    participle_m: "~cado", _;
    ),
    // Fosilizarse: 249 words (fosilizarse, autorrealizarse, autobautizarse, autoanalizarse, autopolinizarse, autoorganizarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("fosilizarse", 249, "~izarse", like Echar;
    stem verb: "~i- like realizar";
    stem verb_fut_1_sg: "~izar- like echar";
    pres_1: "~izo", "~izamos";
    pres_2: "~izas", "~izáis";
    impf_1: "~izaba", "~izábamos";
    subj_1: "~ice", "~icemos";
    subj_2: "~ices", "~icéis";
    subj_3: _, "~icen";
    vos: "~izás", "~icés", "~izate";
    imperative: "~ízate", "~ícese", "~icémonos", "~izaos", "~ícense";
    infinitive: "~izarse";
    gerund: "~izándose";
    participle_m: "~izado", _;
    ),
    // Autoabastecerse: 176 words (autoabastecerse, entrometerse, entremeterse, autoprotegerse, entigrecerse, ensilvecerse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("autoabastecerse", 176, "~e~erse", like Incautarse;
    stem verb: "~e~- like conocer";
    stem verb_fut_1_sg: "~e~er- like echar";
    pres_1: "~ez~o", "~e~emos";
    pres_2: "~e~es", "~e~éis";
    impf_1: "~e~ía", "~e~íamos";
    subj_1: "~ez~a", "~ez~amos";
    subj_2: "~ez~as", "~ez~áis";
    subj_3: _, "~ez~an";
    vos: "~e~és", "~ez~ás", "~e~ete";
    imperative: "~é~ete", "~éz~ase", "~ez~ámonos", "~e~eos", "~éz~anse";
    infinitive: "~e~erse";
    gerund: "~e~iéndose";
    participle_m: "~e~ido", _;
    ),
    // Meter: 145 words (absconder, absorber, her, meter, acceder, beber). Like Curtir, except in 6 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative vosotros, infinitive.
    conjugation!("meter", 145, "~er", like Curtir;
    stem verb_fut_1_sg: "~er- like echar";
    pres_1: _, "~emos";
    pres_2: _, "~éis";
    vos: "~és", _, "~é";
    imperative: _, _, _, "~ed", _;
    infinitive: "~er";
    ),
    // Osificarse: 134 words (gelificarse, automedicarse, autocalificarse, autoaplicarse, osificarse, autoidentificarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("osificarse", 134, "~icarse", like Echar;
    stem verb: "~i- like sacar";
    stem verb_fut_1_sg: "~icar- like echar";
    pres_1: "~ico", "~icamos";
    pres_2: "~icas", "~icáis";
    impf_1: "~icaba", "~icábamos";
    subj_1: "~ique", "~iquemos";
    subj_2: "~iques", "~iquéis";
    subj_3: _, "~iquen";
    vos: "~icás", "~iqués", "~icate";
    imperative: "~ícate", "~íquese", "~iquémonos", "~icaos", "~íquense";
    infinitive: "~icarse";
    gerund: "~icándose";
    participle_m: "~icado", _;
    ),
    // Cerrar: 132 words (pensar, quebrar, helar, gobernar, confesar, atestar). Like Regodearse, except in 13 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, vos imp, ….
    conjugation!("cerrar", 132, "~~ar", like Regodearse;
    pres_1: "~i~o", _;
    pres_2: "~i~as", _;
    subj_1: "~i~e", _;
    subj_2: "~i~es", _;
    subj_3: _, "~i~en";
    vos: _, _, "~~á";
    imperative: "~i~a", "~i~e", "~~emos", "~~ad", "~i~en";
    infinitive: "~~ar";
    gerund: "~~ando";
    ),
    // Darse: 115 words (acovacharse, abicharse, abrocharse, arreguincharse, reguincharse, emborracharse). Like Echar with `` after each pattern (`se` after e, n, r), except in 10 cells: pret_3 pl, subj_1 sg, subj_3 pl, subj_impf_se_3 sg, subj_fut_3 sg, vos imp, ….
    conjugation!("darse", 115, "~arse", like Echar with "", "se" after "enr";
    pret_3: _, "~aron";
    subj_1: "~e", _;
    subj_3: _, "~en";
    subj_impf_se_3: "~ase", _;
    subj_fut_3: "~are", _;
    vos: _, _, "~áte";
    imperative: "~ate", _, "~émonos", "~aos", _;
    gerund: "~ándose";
    ),
    // Desviar: 109 words (enviar, enfriar, ampliar, aviar, confiar, agriar). Like Curtir, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("desviar", 109, "~iar", like Curtir;
    stem verb: "~i- like echar";
    stem verb_fut_1_sg: "~iar- like echar";
    pres_1: "~ío", "~iamos";
    pres_2: "~ías", "~iáis";
    impf_1: "~iaba", "~iábamos";
    subj_1: "~íe", "~iemos";
    subj_2: "~íes", "~iéis";
    subj_3: _, "~íen";
    vos: "~iás", "~iés", "~iá";
    imperative: "~ía", "~íe", "~iemos", "~iad", "~íen";
    infinitive: "~iar";
    gerund: "~iando";
    participle_m: "~iado", _;
    ),
    // Instruir: 105 words (construir, destruir, bullir, incluir, excluir, sustituir). Like Curtir, except in 13 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, ….
    conjugation!("instruir", 105, "~ir", like Curtir;
    stem verb: "~- like instruir";
    pres_1: "~yo", _;
    pres_2: "~yes", _;
    subj_1: "~ya", "~yamos";
    subj_2: "~yas", "~yáis";
    subj_3: _, "~yan";
    vos: _, "~yás", _;
    imperative: "~ye", "~ya", "~yamos", _, "~yan";
    gerund: "~yendo";
    ),
    // Sonar: 80 words (sonar, volar, solar, acostar, acordar, probar). Like Echar with `ˇ` after each pattern.
    conjugation!("sonar", 80, "~arˇ", like Echar with "ˇ";
    ),
    // Atravesarse: 70 words (autogobernarse, quebrarse, abnegarse, sentarse, regarse, atravesarse). Like Quejarse, except in 8 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("atravesarse", 70, "~e~arse", like Quejarse;
    pres_1: "~ie~o", _;
    pres_2: "~ie~as", _;
    subj_1: "~ie~e", _;
    subj_2: "~ie~es", _;
    subj_3: _, "~ie~en";
    imperative: "~ié~ate", "~ié~ese", _, _, "~ié~ense";
    ),
    // Sentir: 56 words (mentir, sentir, zaherir, herir, advertir, sugerir). Like Autoabastecerse, except in 25 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, pret_3 sg, pret_3 pl, ….
    conjugation!("sentir", 56, "~e~ir", like Autoabastecerse;
    stem verb_fut_1_sg: "~e~ir- like echar";
    pres_1: "~ie~o", "~e~imos";
    pres_2: "~ie~es", "~e~ís";
    pret_3: "~i~ió", "~i~ieron";
    subj_1: "~ie~a", "~i~amos";
    subj_2: "~ie~as", "~i~áis";
    subj_3: _, "~ie~an";
    subj_impf_1: _, "~i~iéramos";
    subj_impf_3: "~i~iera", _;
    subj_impf_se_3: "~i~iese", _;
    subj_fut_3: "~i~iere", _;
    vos: "~e~ís", "~i~ás", "~e~í";
    imperative: "~ie~e", "~ie~a", "~i~amos", "~e~id", "~ie~an";
    infinitive: "~e~ir";
    gerund: "~i~iendo";
    ),
    // Situar: 56 words (puntuar, continuar, efectuar, actuar, situar, menstruar). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("situar", 56, "~uar", like Echar;
    stem verb: "~u- like echar";
    stem verb_fut_1_sg: "~uar- like echar";
    pres_1: "~úo", "~uamos";
    pres_2: "~úas", "~uáis";
    impf_1: "~uaba", "~uábamos";
    subj_1: "~úe", "~uemos";
    subj_2: "~úes", "~uéis";
    subj_3: _, "~úen";
    vos: "~uás", "~ués", "~uá";
    imperative: "~úa", "~úe", "~uemos", "~uad", "~úen";
    infinitive: "~uar";
    gerund: "~uando";
    participle_m: "~uado", _;
    ),
    // Desencontrarse: 54 words (desencontrarse, colarse, encontrarse, poblarse, encordarse, acostarse). Like Acomodarse, except in 8 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("desencontrarse", 54, "~o~arse", like Acomodarse;
    pres_1: "~ue~o", _;
    pres_2: "~ue~as", _;
    subj_1: "~ue~e", _;
    subj_2: "~ue~es", _;
    subj_3: _, "~ue~en";
    imperative: "~ué~ate", "~ué~ese", _, _, "~ué~ense";
    ),
    // Musirse: 54 words (desnutrirse, musirse, entumirse, ocurrirse, consumirse, eludirse). Like Emputarse, except in 21 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("musirse", 54, "~u~irse", like Emputarse;
    stem verb: "~u~- like conocer";
    stem verb_fut_1_sg: "~u~ir- like echar";
    pres_1: _, "~u~imos";
    pres_2: "~u~es", "~u~ís";
    impf_1: "~u~ía", "~u~íamos";
    subj_1: "~u~a", "~u~amos";
    subj_2: "~u~as", "~u~áis";
    subj_3: _, "~u~an";
    vos: "~u~ís", "~u~ás", "~u~ite";
    imperative: "~ú~ete", "~ú~ase", "~u~ámonos", "~u~íos", "~ú~anse";
    infinitive: "~u~irse";
    gerund: "~u~iéndose";
    participle_m: "~u~ido", _;
    ),
    // Dirigir: 46 words (urgir, mugir, exigir, fingir, agir, dirigir). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("dirigir", 46, "~gir", like Echar;
    stem verb: "~g- like conocer";
    stem verb_fut_1_sg: "~gir- like echar";
    pres_1: "~jo", "~gimos";
    pres_2: "~ges", "~gís";
    impf_1: "~gía", "~gíamos";
    subj_1: "~ja", "~jamos";
    subj_2: "~jas", "~jáis";
    subj_3: _, "~jan";
    vos: "~gís", "~jás", "~gí";
    imperative: "~ge", "~ja", "~jamos", "~gid", "~jan";
    infinitive: "~gir";
    gerund: "~giendo";
    participle_m: "~gido", _;
    ),
    // Volar: 41 words (soltar, encontrar, demostrar, soñar, mostrar, colgar). Like Desencontrarse, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("volar", 41, "~o~ar", like Desencontrarse;
    vos: _, _, "~o~á";
    imperative: "~ue~a", "~ue~e", "~o~emos", "~o~ad", "~ue~en";
    infinitive: "~o~ar";
    gerund: "~o~ando";
    ),
    // Resfriarse: 40 words (resfriarse, guiarse, confiarse, demasiarse, enfriarse, aliarse). Like Desviar with `` after each pattern (`se` after e, n, r), except in 10 cells: pret_3 pl, subj_1 sg, subj_3 pl, subj_impf_se_3 sg, subj_fut_3 sg, vos imp, ….
    conjugation!("resfriarse", 40, "~iarse", like Desviar with "", "se" after "enr";
    pret_3: _, "~iaron";
    subj_1: "~íe", _;
    subj_3: _, "~íen";
    subj_impf_se_3: "~iase", _;
    subj_fut_3: "~iare", _;
    vos: _, _, "~iate";
    imperative: "~íate", _, "~iémonos", "~iaos", _;
    gerund: "~iándose";
    ),
    // Poner: 37 words (poner, yuxtaponer, imponer, suponer, exponer, posponer). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("poner", 37, "~oner", like Echar;
    stem verb: "~us- like poner";
    stem verb_fut_1_sg: "~ondr- like echar";
    pres_1: "~ongo", "~onemos";
    pres_2: "~ones", "~onéis";
    impf_1: "~onía", "~oníamos";
    subj_1: "~onga", "~ongamos";
    subj_2: "~ongas", "~ongáis";
    subj_3: _, "~ongan";
    vos: "~onés", "~ongás", "~oné";
    imperative: "~ón", "~onga", "~ongamos", "~oned", "~ongan";
    infinitive: "~oner";
    gerund: "~oniendo";
    participle_m: "~uesto", _;
    ),
    // Fraguar: 32 words (averiguar, atestiguar, menguar, aguar, testiguar, apaciguar). Like Situar, except in 12 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, ….
    conjugation!("fraguar", 32, "~uar", like Situar;
    stem verb: "~- like fraguar";
    pres_1: "~uo", _;
    pres_2: "~uas", _;
    subj_1: "~üe", "~üemos";
    subj_2: "~ües", "~üéis";
    subj_3: _, "~üen";
    vos: _, "~üés", _;
    imperative: "~ua", "~üe", "~üemos", _, "~üen";
    ),
    // Rendir: 32 words (pedir, servir, despedir, vestir, investir, medir). Like Sentir, except in 8 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("rendir", 32, "~e~ir", like Sentir;
    pres_1: "~i~o", _;
    pres_2: "~i~es", _;
    subj_1: "~i~a", _;
    subj_2: "~i~as", _;
    subj_3: _, "~i~an";
    imperative: "~i~e", "~i~a", _, _, "~i~an";
    ),
    // Descocarse: 31 words (autoconvocarse, descocarse, abocarse, apocarse, equivocarse, enfocarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("descocarse", 31, "~ocarse", like Echar;
    stem verb: "~o- like sacar";
    stem verb_fut_1_sg: "~ocar- like echar";
    pres_1: "~oco", "~ocamos";
    pres_2: "~ocas", "~ocáis";
    impf_1: "~ocaba", "~ocábamos";
    subj_1: "~oque", "~oquemos";
    subj_2: "~oques", "~oquéis";
    subj_3: _, "~oquen";
    vos: "~ocás", "~oqués", "~ocate";
    imperative: "~ócate", "~óquese", "~oquémonos", "~ocaos", "~óquense";
    infinitive: "~ocarse";
    gerund: "~ocándose";
    participle_m: "~ocado", _;
    ),
    // Desvivirse: 30 words (desvivirse, autodefinirse, autopercibirse, inhibirse, extinguirse, distinguirse). Like Rendir, except in 14 cells: pres_1 pl, pres_2 pl, impf_1 sg, impf_1 pl, vos ind, vos imp, ….
    conjugation!("desvivirse", 30, "~i~irse", like Rendir;
    stem verb: "~i~- like conocer";
    stem verb_fut_1_sg: "~i~ir- like echar";
    pres_1: _, "~i~imos";
    pres_2: _, "~i~ís";
    impf_1: "~i~ía", "~i~íamos";
    vos: "~i~ís", _, "~i~ite";
    imperative: "~í~ete", "~í~ase", "~i~ámonos", "~i~íos", "~í~anse";
    infinitive: "~i~irse";
    gerund: "~i~iéndose";
    participle_m: "~i~ido", _;
    ),
    // Situarse: 27 words (autoevaluarse, adecuarse, atenuarse, extenuarse, situarse, insinuarse). Like Situar with `` after each pattern (`se` after e, n, r), except in 10 cells: pret_3 pl, subj_1 sg, subj_3 pl, subj_impf_se_3 sg, subj_fut_3 sg, vos imp, ….
    conjugation!("situarse", 27, "~uarse", like Situar with "", "se" after "enr";
    pret_3: _, "~uaron";
    subj_1: "~úe", _;
    subj_3: _, "~úen";
    subj_impf_se_3: "~uase", _;
    subj_fut_3: "~uare", _;
    vos: _, _, "~uate";
    imperative: "~úate", _, "~uémonos", "~uaos", _;
    gerund: "~uándose";
    ),
    // Embazarse: 27 words (embazarse, abrazarse, acarrazarse, amostazarse, desplazarse, tarazarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("embazarse", 27, "~azarse", like Echar;
    stem verb: "~a- like realizar";
    stem verb_fut_1_sg: "~azar- like echar";
    pres_1: "~azo", "~azamos";
    pres_2: "~azas", "~azáis";
    impf_1: "~azaba", "~azábamos";
    subj_1: "~ace", "~acemos";
    subj_2: "~aces", "~acéis";
    subj_3: _, "~acen";
    vos: "~azás", "~acés", "~azate";
    imperative: "~ázate", "~ácese", "~acémonos", "~azaos", "~ácense";
    infinitive: "~azarse";
    gerund: "~azándose";
    participle_m: "~azado", _;
    ),
    // Destoserse: 27 words (destoserse, correrse, coserse, corresponderse, dolerse, carcomerse). Like Acomodarse, except in 21 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("destoserse", 27, "~o~erse", like Acomodarse;
    stem verb: "~o~- like conocer";
    stem verb_fut_1_sg: "~o~er- like echar";
    pres_1: _, "~o~emos";
    pres_2: "~o~es", "~o~éis";
    impf_1: "~o~ía", "~o~íamos";
    subj_1: "~o~a", "~o~amos";
    subj_2: "~o~as", "~o~áis";
    subj_3: _, "~o~an";
    vos: "~o~és", "~o~ás", "~o~ete";
    imperative: "~ó~ete", "~ó~ase", "~o~ámonos", "~o~eos", "~ó~anse";
    infinitive: "~o~erse";
    gerund: "~o~iéndose";
    participle_m: "~o~ido", _;
    ),
    // Regar: 25 words (negar, renegar, fregar, regar, segar, sosegar). Like Echar with `ˇ` after each pattern, except in 10 cells: pret_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("regar", 25, "~arˇ", like Echar with "ˇ";
    pret_1: "~uéˇ", _;
    subj_1: "~ueˇ", "~uemosˇ";
    subj_2: "~uesˇ", "~uéisˇ";
    subj_3: _, "~uenˇ";
    vos: _, "~uésˇ", _;
    imperative: _, "~ueˇ", "~uemosˇ", _, "~uenˇ";
    ),
    // Venir: 24 words (devenir, venir, intervenir, avenir, revenir, convenir). Like Sentir, except in 11 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("venir", 24, "~e~ir", like Sentir;
    stem verb: "~i~- like poner";
    stem verb_fut_1_sg: "~e~dr- like echar";
    pres_1: "~e~go", _;
    subj_1: "~e~ga", "~e~gamos";
    subj_2: "~e~gas", "~e~gáis";
    subj_3: _, "~e~gan";
    vos: _, "~e~gás", _;
    imperative: "~é~", "~e~ga", "~e~gamos", _, "~e~gan";
    ),
    // Reducir: 24 words (traducir, producir, reducir, conducir, introducir, inducir). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("reducir", 24, "~cir", like Echar;
    stem verb: "~j- like reducir";
    stem verb_fut_1_sg: "~cir- like echar";
    pres_1: "~zco", "~cimos";
    pres_2: "~ces", "~cís";
    impf_1: "~cía", "~cíamos";
    subj_1: "~zca", "~zcamos";
    subj_2: "~zcas", "~zcáis";
    subj_3: _, "~zcan";
    vos: "~cís", "~zcás", "~cí";
    imperative: "~ce", "~zca", "~zcamos", "~cid", "~zcan";
    infinitive: "~cir";
    gerund: "~ciendo";
    participle_m: "~cido", _;
    ),
    // Escribir: 23 words (escribir, describir, inscribir, transcribir, prescribir, proscribir). Like Echar, except in 21 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("escribir", 23, "~bir", like Echar;
    stem verb: "~b- like conocer";
    stem verb_fut_1_sg: "~bir- like echar";
    stem participle: "~- like escribir";
    pres_1: "~bo", "~bimos";
    pres_2: "~bes", "~bís";
    impf_1: "~bía", "~bíamos";
    subj_1: "~ba", "~bamos";
    subj_2: "~bas", "~báis";
    subj_3: _, "~ban";
    vos: "~bís", "~bás", "~bí";
    imperative: "~be", "~ba", "~bamos", "~bid", "~ban";
    infinitive: "~bir";
    gerund: "~biendo";
    ),
    // Sacarse: 23 words (abarracarse, abellacarse, desajacarse, trelacarse, atracarse, enflacarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("sacarse", 23, "~acarse", like Echar;
    stem verb: "~a- like sacar";
    stem verb_fut_1_sg: "~acar- like echar";
    pres_1: "~aco", "~acamos";
    pres_2: "~acas", "~acáis";
    impf_1: "~acaba", "~acábamos";
    subj_1: "~aque", "~aquemos";
    subj_2: "~aques", "~aquéis";
    subj_3: _, "~aquen";
    vos: "~acás", "~aqués", "~acate";
    imperative: "~ácate", "~áquese", "~aquémonos", "~acaos", "~áquense";
    infinitive: "~acarse";
    gerund: "~acándose";
    participle_m: "~acado", _;
    ),
    // Meliar: 23 words (gilidiar, chalequiar, meliar, cangrejiar, jariniar, jonjoliar). Like Desviar, except in 8 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("meliar", 23, "~iar", like Desviar;
    pres_1: "~eo", _;
    pres_2: "~eas", _;
    subj_1: "~ee", _;
    subj_2: "~ees", _;
    subj_3: _, "~een";
    imperative: "~ea", "~ee", _, _, "~een";
    ),
    // Coger: 22 words (proteger, coger, recoger, escoger, acoger, emerger). Like Dirigir, except in 6 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative vosotros, infinitive.
    conjugation!("coger", 22, "~ger", like Dirigir;
    stem verb_fut_1_sg: "~ger- like echar";
    pres_1: _, "~gemos";
    pres_2: _, "~géis";
    vos: "~gés", _, "~gé";
    imperative: _, _, _, "~ged", _;
    infinitive: "~ger";
    ),
    // Descomponerse: 22 words (autoimponerse, ponerse, imponerse, disponerse, exponerse, reponerse). Like Poner with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("descomponerse", 22, "~onerse", like Poner with "", "se" after "r";
    vos: _, _, "~onete";
    imperative: "~onte", "~óngase", "~ongámonos", "~oneos", "~ónganse";
    gerund: "~oniéndose";
    ),
    // Ahijar: 22 words (prohijar, descafeinar, aislar, ahincar, amohinar, cabrahigar). Like Rilarse, except in 13 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, vos imp, ….
    conjugation!("ahijar", 22, "~i~ar", like Rilarse;
    pres_1: "~í~o", _;
    pres_2: "~í~as", _;
    subj_1: "~í~e", _;
    subj_2: "~í~es", _;
    subj_3: _, "~í~en";
    vos: _, _, "~i~á";
    imperative: "~í~a", "~í~e", "~i~emos", "~i~ad", "~í~en";
    infinitive: "~i~ar";
    gerund: "~i~ando";
    ),
    // Alzarse: 22 words (emperezarse, esperezarse, alzarse, vezarse, avezarse, avergonzarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("alzarse", 22, "~~zarse", like Incautarse;
    stem verb: "~~- like realizar";
    stem verb_fut_1_sg: "~~zar- like echar";
    pres_1: "~~zo", "~~zamos";
    pres_2: "~~zas", "~~záis";
    impf_1: "~~zaba", "~~zábamos";
    subj_1: "~~ce", "~~cemos";
    subj_2: "~~ces", "~~céis";
    subj_3: _, "~~cen";
    vos: "~~zás", "~~cés", "~~zate";
    imperative: "~ézat~", "~éc~se", "~~cémonos", "~~zaos", "~éc~nse";
    infinitive: "~~zarse";
    gerund: "~~zándose";
    participle_m: "~~zado", _;
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
    // Inmiscuirse: 21 words (autodestruirse, inmiscuirse, autoexcluirse, autoconstruirse, rehuirse, atribuirse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("inmiscuirse", 21, "~uirse", like Echar;
    stem verb: "~u- like instruir";
    stem verb_fut_1_sg: "~uir- like echar";
    pres_1: "~uyo", "~uimos";
    pres_2: "~uyes", "~uís";
    impf_1: "~uía", "~uíamos";
    subj_1: "~uya", "~uyamos";
    subj_2: "~uyas", "~uyáis";
    subj_3: _, "~uyan";
    vos: "~uís", "~uyás", "~uite";
    imperative: "~úyete", "~úyase", "~uyámonos", "~uíos", "~úyanse";
    infinitive: "~uirse";
    gerund: "~uyéndose";
    participle_m: "~uido", _;
    ),
    // Aunar: 20 words (aunar, rehusar, ahumar, maullar, aupar, aullar). Like Emputarse, except in 13 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, vos imp, ….
    conjugation!("aunar", 20, "~u~ar", like Emputarse;
    pres_1: "~ú~o", _;
    pres_2: "~ú~as", _;
    subj_1: "~ú~e", _;
    subj_2: "~ú~es", _;
    subj_3: _, "~ú~en";
    vos: _, _, "~u~á";
    imperative: "~ú~a", "~ú~e", "~u~emos", "~u~ad", "~ú~en";
    infinitive: "~u~ar";
    gerund: "~u~ando";
    ),
    // Abatirse: 20 words (batirse, partirse, persuadirse, abatirse, debatirse, expandirse). Like Incautarse, except in 21 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("abatirse", 20, "~a~irse", like Incautarse;
    stem verb: "~a~- like conocer";
    stem verb_fut_1_sg: "~a~ir- like echar";
    pres_1: _, "~a~imos";
    pres_2: "~a~es", "~a~ís";
    impf_1: "~a~ía", "~a~íamos";
    subj_1: "~a~a", "~a~amos";
    subj_2: "~a~as", "~a~áis";
    subj_3: _, "~a~an";
    vos: "~a~ís", "~a~ás", "~a~ite";
    imperative: "~á~ete", "~á~ase", "~a~ámonos", "~a~íos", "~á~anse";
    infinitive: "~a~irse";
    gerund: "~a~iéndose";
    participle_m: "~a~ido", _;
    ),
    // Chuzarse: 19 words (chapuzarse, abruzarse, abuzarse, escagarruzarse, aliruzarse, atapuzarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("chuzarse", 19, "~uzarse", like Echar;
    stem verb: "~u- like realizar";
    stem verb_fut_1_sg: "~uzar- like echar";
    pres_1: "~uzo", "~uzamos";
    pres_2: "~uzas", "~uzáis";
    impf_1: "~uzaba", "~uzábamos";
    subj_1: "~uce", "~ucemos";
    subj_2: "~uces", "~ucéis";
    subj_3: _, "~ucen";
    vos: "~uzás", "~ucés", "~uzate";
    imperative: "~úzate", "~úcese", "~ucémonos", "~uzaos", "~úcense";
    infinitive: "~uzarse";
    gerund: "~uzándose";
    participle_m: "~uzado", _;
    ),
    // Resentirse: 19 words (resentirse, arrepentirse, repentirse, divertirse, sentirse, referirse). Like Sentir with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("resentirse", 19, "~e~irse", like Sentir with "", "se" after "r";
    vos: _, _, "~e~ite";
    imperative: "~ié~ete", "~ié~ase", "~i~ámonos", "~e~íos", "~ié~anse";
    gerund: "~i~iéndose";
    ),
    // Fruncir: 18 words (zurcir, uncir, surcir, esparcir, resarcir, fruncir). Like Reducir, except in 10 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("fruncir", 18, "~cir", like Reducir;
    stem verb: "~c- like conocer";
    pres_1: "~zo", _;
    subj_1: "~za", "~zamos";
    subj_2: "~zas", "~záis";
    subj_3: _, "~zan";
    vos: _, "~zás", _;
    imperative: _, "~za", "~zamos", _, "~zan";
    ),
    // Calzarse: 18 words (abalanzarse, ensalzarse, esperanzarse, alianzarse, alcanzarse, desalzarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("calzarse", 18, "~a~zarse", like Incautarse;
    stem verb: "~a~- like realizar";
    stem verb_fut_1_sg: "~a~zar- like echar";
    pres_1: "~a~zo", "~a~zamos";
    pres_2: "~a~zas", "~a~záis";
    impf_1: "~a~zaba", "~a~zábamos";
    subj_1: "~a~ce", "~a~cemos";
    subj_2: "~a~ces", "~a~céis";
    subj_3: _, "~a~cen";
    vos: "~a~zás", "~a~cés", "~a~zate";
    imperative: "~á~zate", "~á~cese", "~a~cémonos", "~a~zaos", "~á~cense";
    infinitive: "~a~zarse";
    gerund: "~a~zándose";
    participle_m: "~a~zado", _;
    ),
    // Desentenderse: 17 words (desentenderse, autodefenderse, perderse, defenderse, extenderse, sobrentenderse). Like Entremeterse, except in 8 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("desentenderse", 17, "~e~erse", like Entremeterse;
    pres_1: "~ie~o", _;
    pres_2: "~ie~es", _;
    subj_1: "~ie~a", _;
    subj_2: "~ie~as", _;
    subj_3: _, "~ie~an";
    imperative: "~ié~ete", "~ié~ase", _, _, "~ié~anse";
    ),
    // Creer: 17 words (peer, creer, leer, veer, seer, roer). Like Meter, except in 2 cells: gerund, participle_m sg.
    conjugation!("creer", 17, "~er", like Meter;
    stem verb: "~- like creer";
    gerund: "~yendo";
    participle_m: "~ído", _;
    ),
    // Traer: 16 words (traer, contraer, atraer, extraer, substraer, abstraer). Like Creer, except in 10 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("traer", 16, "~er", like Creer;
    stem verb: "~j- like reducir";
    pres_1: "~igo", _;
    subj_1: "~iga", "~igamos";
    subj_2: "~igas", "~igáis";
    subj_3: _, "~igan";
    vos: _, "~igás", _;
    imperative: _, "~iga", "~igamos", _, "~igan";
    ),
    // Entecarse: 16 words (entecarse, enchuecarse, obcecarse, intersecarse, ahuecarse, deprecarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("entecarse", 16, "~~carse", like Incautarse;
    stem verb: "~~- like sacar";
    stem verb_fut_1_sg: "~~car- like echar";
    pres_1: "~~co", "~~camos";
    pres_2: "~~cas", "~~cáis";
    impf_1: "~~caba", "~~cábamos";
    subj_1: "~~que", "~~quemos";
    subj_2: "~~ques", "~~quéis";
    subj_3: _, "~~quen";
    vos: "~~cás", "~~qués", "~~cate";
    imperative: "~écat~", "~équ~se", "~~quémonos", "~~caos", "~équ~nse";
    infinitive: "~~carse";
    gerund: "~~cándose";
    participle_m: "~~cado", _;
    ),
    // Aborrascarse: 14 words (aborrascarse, atascarse, rascarse, achubascarse, enfrascarse, emborrascarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("aborrascarse", 14, "~a~carse", like Incautarse;
    stem verb: "~a~- like sacar";
    stem verb_fut_1_sg: "~a~car- like echar";
    pres_1: "~a~co", "~a~camos";
    pres_2: "~a~cas", "~a~cáis";
    impf_1: "~a~caba", "~a~cábamos";
    subj_1: "~a~que", "~a~quemos";
    subj_2: "~a~ques", "~a~quéis";
    subj_3: _, "~a~quen";
    vos: "~a~cás", "~a~qués", "~a~cate";
    imperative: "~á~cate", "~á~quese", "~a~quémonos", "~a~caos", "~á~quense";
    infinitive: "~a~carse";
    gerund: "~a~cándose";
    participle_m: "~a~cado", _;
    ),
    // Revolver: 14 words (absolver, revolver, resolver, disolver, volver, solver). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("revolver", 14, "~o~ver", like Incautarse;
    stem verb: "~o~v- like conocer";
    stem verb_fut_1_sg: "~o~ver- like echar";
    pres_1: "~ue~vo", "~o~vemos";
    pres_2: "~ue~ves", "~o~véis";
    impf_1: "~o~vía", "~o~víamos";
    subj_1: "~ue~va", "~o~vamos";
    subj_2: "~ue~vas", "~o~váis";
    subj_3: _, "~ue~van";
    vos: "~o~vés", "~o~vás", "~o~vé";
    imperative: "~ue~ve", "~ue~va", "~o~vamos", "~o~ved", "~ue~van";
    infinitive: "~o~ver";
    gerund: "~o~viendo";
    participle_m: "~ue~to", _;
    ),
    // Rozarse: 14 words (apozarse, escozarse, arrebozarse, empozarse, alborozarse, desarrebozarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("rozarse", 14, "~ozarse", like Echar;
    stem verb: "~o- like realizar";
    stem verb_fut_1_sg: "~ozar- like echar";
    pres_1: "~ozo", "~ozamos";
    pres_2: "~ozas", "~ozáis";
    impf_1: "~ozaba", "~ozábamos";
    subj_1: "~oce", "~ocemos";
    subj_2: "~oces", "~océis";
    subj_3: _, "~ocen";
    vos: "~ozás", "~océs", "~ozate";
    imperative: "~ózate", "~ócese", "~océmonos", "~ozaos", "~ócense";
    infinitive: "~ozarse";
    gerund: "~ozándose";
    participle_m: "~ozado", _;
    ),
    // Vencer: 13 words (coercer, vencer, ejercer, convencer, mecer, mulcer). Like Fruncir, except in 6 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative vosotros, infinitive.
    conjugation!("vencer", 13, "~cer", like Fruncir;
    stem verb_fut_1_sg: "~cer- like echar";
    pres_1: _, "~cemos";
    pres_2: _, "~céis";
    vos: "~cés", _, "~cé";
    imperative: _, _, _, "~ced", _;
    infinitive: "~cer";
    ),
    // Tener: 13 words (tener, sostener, obtener, detener, mantener, retener). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("tener", 13, "~ener", like Echar;
    stem verb: "~uv- like poner";
    stem verb_fut_1_sg: "~endr- like echar";
    pres_1: "~engo", "~enemos";
    pres_2: "~ienes", "~enéis";
    impf_1: "~enía", "~eníamos";
    subj_1: "~enga", "~engamos";
    subj_2: "~engas", "~engáis";
    subj_3: _, "~engan";
    vos: "~enés", "~engás", "~ené";
    imperative: "~én", "~enga", "~engamos", "~ened", "~engan";
    infinitive: "~ener";
    gerund: "~eniendo";
    participle_m: "~enido", _;
    ),
    // Ceñir: 13 words (teñir, reñir, ceñir, desteñir, estreñir, astreñir). Like Echar, except in 30 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("ceñir", 13, "~eñir", like Echar;
    stem verb_fut_1_sg: "~eñir- like echar";
    pres_1: "~iño", "~eñimos";
    pres_2: "~iñes", "~eñís";
    impf_1: "~eñía", "~eñíamos";
    pret_1: "~eñí", "~eñimos";
    pret_3: "~iñó", "~iñeron";
    subj_1: "~iña", "~iñamos";
    subj_2: "~iñas", "~iñáis";
    subj_3: _, "~iñan";
    subj_impf_1: _, "~iñéramos";
    subj_impf_3: "~iñera", _;
    subj_impf_se_3: "~iñese", _;
    subj_fut_3: "~iñere", _;
    vos: "~eñís", "~iñás", "~eñí";
    imperative: "~iñe", "~iña", "~iñamos", "~eñid", "~iñan";
    infinitive: "~eñir";
    gerund: "~iñendo";
    participle_m: "~eñido", _;
    ),
    // Enfoscarse: 12 words (enfoscarse, afoscarse, coscarse, desemboscarse, enfroscarse, trasroscarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("enfoscarse", 12, "~o~carse", like Incautarse;
    stem verb: "~o~- like sacar";
    stem verb_fut_1_sg: "~o~car- like echar";
    pres_1: "~o~co", "~o~camos";
    pres_2: "~o~cas", "~o~cáis";
    impf_1: "~o~caba", "~o~cábamos";
    subj_1: "~o~que", "~o~quemos";
    subj_2: "~o~ques", "~o~quéis";
    subj_3: _, "~o~quen";
    vos: "~o~cás", "~o~qués", "~o~cate";
    imperative: "~ó~cate", "~ó~quese", "~o~quémonos", "~o~caos", "~ó~quense";
    infinitive: "~o~carse";
    gerund: "~o~cándose";
    participle_m: "~o~cado", _;
    ),
    // Enraizar: 12 words (enraizar, europeizar, arcaizar, descorreizar, judaizar, raizar). Like Fosilizarse, except in 13 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, vos imp, ….
    conjugation!("enraizar", 12, "~izar", like Fosilizarse;
    pres_1: "~ízo", _;
    pres_2: "~ízas", _;
    subj_1: "~íce", _;
    subj_2: "~íces", _;
    subj_3: _, "~ícen";
    vos: _, _, "~izá";
    imperative: "~íza", "~íce", "~icemos", "~izad", "~ícen";
    infinitive: "~izar";
    gerund: "~izando";
    ),
    // Ahilarse: 12 words (autoaislarse, aislarse, ahitarse, ahilarse, airarse, desaislarse). Like Ahijar with `` after each pattern (`se` after e, n, r), except in 10 cells: pret_3 pl, subj_1 sg, subj_3 pl, subj_impf_se_3 sg, subj_fut_3 sg, vos imp, ….
    conjugation!("ahilarse", 12, "~i~arse", like Ahijar with "", "se" after "enr";
    pret_3: _, "~i~aron";
    subj_1: "~í~e", _;
    subj_3: _, "~í~en";
    subj_impf_se_3: "~i~ase", _;
    subj_fut_3: "~i~are", _;
    vos: _, _, "~i~ate";
    imperative: "~í~ate", _, "~i~émonos", "~i~aos", _;
    gerund: "~i~ándose";
    ),
    // Armarse: 12 words (abarse, armarse, asmarse, alarse, andarse, asparse). Like Echar, except in 32 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("armarse", 12, "a~arse", like Echar;
    pres_1: "a~o", "a~amos";
    pres_2: "a~as", "a~áis";
    impf_1: "a~aba", "a~ábamos";
    pret_1: "a~é", "a~amos";
    pret_3: "a~ó", "a~aron";
    fut_1: "a~aré", _;
    cond_3: "a~aría", _;
    subj_1: "a~e", "a~emos";
    subj_2: "a~es", "a~éis";
    subj_3: _, "a~en";
    subj_impf_1: _, "a~áramos";
    subj_impf_3: "a~ara", _;
    subj_impf_se_3: "a~ase", _;
    subj_fut_3: "a~are", _;
    vos: "a~ás", "a~és", "a~ate";
    imperative: "á~ate", "á~ese", "a~émonos", "a~aos", "á~ense";
    infinitive: "a~arse";
    gerund: "a~ándose";
    participle_m: "a~ado", _;
    ),
    // Hacer: 12 words (hacer, satisfacer, facer, deshacer, rehacer, contrahacer). Like Echar, except in 30 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("hacer", 12, "~acer", like Echar;
    pres_1: "~ago", "~acemos";
    pres_2: "~aces", "~acéis";
    impf_1: "~acía", "~acíamos";
    pret_1: "~ice", "~icimos";
    pret_3: "~izo", "~icieron";
    subj_1: "~aga", "~agamos";
    subj_2: "~agas", "~agáis";
    subj_3: _, "~agan";
    subj_impf_1: _, "~iciéramos";
    subj_impf_3: "~iciera", _;
    subj_impf_se_3: "~iciese", _;
    subj_fut_3: "~iciere", _;
    vos: "~acés", "~agás", "~acé";
    imperative: "~az", "~aga", "~agamos", "~aced", "~agan";
    infinitive: "~acer";
    gerund: "~aciendo";
    participle_m: "~echo", _;
    ),
    // Moler: 12 words (llover, doler, poder, morder, soler, moler). Like Meter with `ˇ` after each pattern.
    conjugation!("moler", 12, "~erˇ", like Meter with "ˇ";
    ),
    // Escabullirse: 11 words (escabullirse, zambullirse, rebullirse, tullirse, escullirse, descabullirse). Like Musirse, except in 1 cell: gerund.
    conjugation!("escabullirse", 11, "~u~irse", like Musirse;
    stem verb: "~u~- like escabullirse";
    gerund: "~u~éndose";
    ),
    // Apanucarse: 11 words (acurrucarse, acorrucarse, embejucarse, apitucarse, emborucarse, desnucarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("apanucarse", 11, "~ucarse", like Echar;
    stem verb: "~u- like sacar";
    stem verb_fut_1_sg: "~ucar- like echar";
    pres_1: "~uco", "~ucamos";
    pres_2: "~ucas", "~ucáis";
    impf_1: "~ucaba", "~ucábamos";
    subj_1: "~uque", "~uquemos";
    subj_2: "~uques", "~uquéis";
    subj_3: _, "~uquen";
    vos: "~ucás", "~uqués", "~ucate";
    imperative: "~úcate", "~úquese", "~uquémonos", "~ucaos", "~úquense";
    infinitive: "~ucarse";
    gerund: "~ucándose";
    participle_m: "~ucado", _;
    ),
    // Suscribirse: 10 words (autodescribirse, escribirse, adscribirse, inscribirse, subscribirse, suscribirse). Like Echar, except in 21 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("suscribirse", 10, "~ibirse", like Echar;
    stem verb: "~ib- like conocer";
    stem verb_fut_1_sg: "~ibir- like echar";
    stem participle: "~i- like escribir";
    pres_1: "~ibo", "~ibimos";
    pres_2: "~ibes", "~ibís";
    impf_1: "~ibía", "~ibíamos";
    subj_1: "~iba", "~ibamos";
    subj_2: "~ibas", "~ibáis";
    subj_3: _, "~iban";
    vos: "~ibís", "~ibás", "~ibite";
    imperative: "~íbete", "~íbase", "~ibámonos", "~ibíos", "~íbanse";
    infinitive: "~ibirse";
    gerund: "~ibiéndose";
    ),
    // Retraerse: 10 words (contraerse, traerse, atraerse, sustraerse, retrotraerse, retraerse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("retraerse", 10, "~aerse", like Echar;
    stem verb: "~aj- like reducir";
    stem verb_fut_1_sg: "~aer- like echar";
    pres_1: "~aigo", "~aemos";
    pres_2: "~aes", "~aéis";
    impf_1: "~aía", "~aíamos";
    subj_1: "~aiga", "~aigamos";
    subj_2: "~aigas", "~aigáis";
    subj_3: _, "~aigan";
    vos: "~aés", "~aigás", "~aete";
    imperative: "~áete", "~áigase", "~aigámonos", "~aeos", "~áiganse";
    infinitive: "~aerse";
    gerund: "~ayéndose";
    participle_m: "~aído", _;
    ),
    // Lucir: 10 words (lucir, relucir, deslucir, enlucir, hendir, traslucir). Like Conocer, except in 6 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative vosotros, infinitive.
    conjugation!("lucir", 10, "~~ir", like Conocer;
    stem verb_fut_1_sg: "~~ir- like echar";
    pres_1: _, "~~imos";
    pres_2: _, "~~ís";
    vos: "~~ís", _, "~~í";
    imperative: _, _, _, "~~id", _;
    infinitive: "~~ir";
    ),
    // Enamoriscarse: 9 words (enamoriscarse, ciscarse, ariscarse, enriscarse, apriscarse, arriscarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("enamoriscarse", 9, "~i~carse", like Incautarse;
    stem verb: "~i~- like sacar";
    stem verb_fut_1_sg: "~i~car- like echar";
    pres_1: "~i~co", "~i~camos";
    pres_2: "~i~cas", "~i~cáis";
    impf_1: "~i~caba", "~i~cábamos";
    subj_1: "~i~que", "~i~quemos";
    subj_2: "~i~ques", "~i~quéis";
    subj_3: _, "~i~quen";
    vos: "~i~cás", "~i~qués", "~i~cate";
    imperative: "~í~cate", "~í~quese", "~i~quémonos", "~i~caos", "~í~quense";
    infinitive: "~i~carse";
    gerund: "~i~cándose";
    participle_m: "~i~cado", _;
    ),
    // Apuñuscarse: 9 words (apuñuscarse, buscarse, rebuscarse, zurruscarse, enfurruscarse, apeñuscarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("apuñuscarse", 9, "~u~carse", like Incautarse;
    stem verb: "~u~- like sacar";
    stem verb_fut_1_sg: "~u~car- like echar";
    pres_1: "~u~co", "~u~camos";
    pres_2: "~u~cas", "~u~cáis";
    impf_1: "~u~caba", "~u~cábamos";
    subj_1: "~u~que", "~u~quemos";
    subj_2: "~u~ques", "~u~quéis";
    subj_3: _, "~u~quen";
    vos: "~u~cás", "~u~qués", "~u~cate";
    imperative: "~ú~cate", "~ú~quese", "~u~quémonos", "~u~caos", "~ú~quense";
    infinitive: "~u~carse";
    gerund: "~u~cándose";
    participle_m: "~u~cado", _;
    ),
    // Abrir: 9 words (cubrir, abrir, reabrir, descubrir, encubrir, entreabrir). Like Lucir, except in 11 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("abrir", 9, "~~ir", like Lucir;
    pres_1: "~~o", _;
    subj_1: "~~a", "~~amos";
    subj_2: "~~as", "~~áis";
    subj_3: _, "~~an";
    vos: _, "~~ás", _;
    imperative: _, "~~a", "~~amos", _, "~~an";
    participle_m: "~ie~to", _;
    ),
    // Trocar: 9 words (trocar, trastrocar, clocar, retrocar, destrocar, aclocar). Like Descocarse, except in 13 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, vos imp, ….
    conjugation!("trocar", 9, "~ocar", like Descocarse;
    pres_1: "~ueco", _;
    pres_2: "~uecas", _;
    subj_1: "~ueque", _;
    subj_2: "~ueques", _;
    subj_3: _, "~uequen";
    vos: _, _, "~ocá";
    imperative: "~ueca", "~ueque", "~oquemos", "~ocad", "~uequen";
    infinitive: "~ocar";
    gerund: "~ocando";
    ),
    // Resolverse: 9 words (autodisolverse, resolverse, volverse, devolverse, revolverse, desenvolverse). Like Revolver with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("resolverse", 9, "~o~verse", like Revolver with "", "se" after "r";
    vos: _, _, "~o~vete";
    imperative: "~ué~vete", "~ué~vase", "~o~vámonos", "~o~veos", "~ué~vanse";
    gerund: "~o~viéndose";
    ),
    // Detenerse: 9 words (abstenerse, contenerse, sostenerse, mantenerse, detenerse, entretenerse). Like Tener with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("detenerse", 9, "~enerse", like Tener with "", "se" after "r";
    vos: _, _, "~enete";
    imperative: "~ente", "~éngase", "~engámonos", "~eneos", "~énganse";
    gerund: "~eniéndose";
    ),
    // Santiguarse: 9 words (achiguarse, apaciguarse, santiguarse, enmaniguarse, amortiguarse, averiguarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("santiguarse", 9, "~i~uarse", like Incautarse;
    stem verb: "~i~- like fraguar";
    stem verb_fut_1_sg: "~i~uar- like echar";
    pres_1: "~i~uo", "~i~uamos";
    pres_2: "~i~uas", "~i~uáis";
    impf_1: "~i~uaba", "~i~uábamos";
    subj_1: "~i~üe", "~i~üemos";
    subj_2: "~i~ües", "~i~üéis";
    subj_3: _, "~i~üen";
    vos: "~i~uás", "~i~üés", "~i~uate";
    imperative: "~í~uate", "~í~üese", "~i~üémonos", "~i~uaos", "~í~üense";
    infinitive: "~i~uarse";
    gerund: "~i~uándose";
    participle_m: "~i~uado", _;
    ),
    // Refreír: 9 words (reír, freír, sonreír, engreír, desleír, esleír). Like Curtir, except in 23 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("refreír", 9, "~eír", like Curtir;
    stem verb_fut_1_sg: "~eir- like echar";
    pres_1: "~ío", "~eímos";
    pres_2: "~íes", "~eís";
    impf_1: "~eía", "~eíamos";
    pret_1: "~eí", "~eímos";
    subj_1: "~ía", "~iamos";
    subj_2: "~ías", "~iáis";
    subj_3: _, "~ían";
    vos: "~eís", "~iás", "~eí";
    imperative: "~íe", "~ía", "~iamos", "~eíd", "~ían";
    infinitive: "~eír";
    participle_m: "~eído", _;
    ),
    // Guiar: 8 words (liar, fiar, criar, guiar, ciar, piar). Like Desviar, except in 7 cells: pres_2 pl, pret_1 sg, pret_3 sg, subj_2 pl, vos ind, vos subj, ….
    conjugation!("guiar", 8, "~iar", like Desviar;
    pres_2: _, "~iais";
    pret_1: "~ie", _;
    pret_3: "~io", _;
    subj_2: _, "~ieis";
    vos: "~ias", "~ies", "~ia";
    ),
    // Reducirse: 8 words (autorreproducirse, autoconducirse, autoproducirse, reducirse, reproducirse, introducirse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("reducirse", 8, "~ucirse", like Echar;
    stem verb: "~uj- like reducir";
    stem verb_fut_1_sg: "~ucir- like echar";
    pres_1: "~uzco", "~ucimos";
    pres_2: "~uces", "~ucís";
    impf_1: "~ucía", "~ucíamos";
    subj_1: "~uzca", "~uzcamos";
    subj_2: "~uzcas", "~uzcáis";
    subj_3: _, "~uzcan";
    vos: "~ucís", "~uzcás", "~ucite";
    imperative: "~úcete", "~úzcase", "~uzcámonos", "~ucíos", "~úzcanse";
    infinitive: "~ucirse";
    gerund: "~uciéndose";
    participle_m: "~ucido", _;
    ),
    // Desmedirse: 8 words (descomedirse, desmedirse, despedirse, acomedirse, medirse, expedirse). Like Echar, except in 30 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("desmedirse", 8, "~edirse", like Echar;
    stem verb_fut_1_sg: "~edir- like echar";
    pres_1: "~ido", "~edimos";
    pres_2: "~ides", "~edís";
    impf_1: "~edía", "~edíamos";
    pret_1: "~edí", "~edimos";
    pret_3: "~idió", "~idieron";
    subj_1: "~ida", "~idamos";
    subj_2: "~idas", "~idáis";
    subj_3: _, "~idan";
    subj_impf_1: _, "~idiéramos";
    subj_impf_3: "~idiera", _;
    subj_impf_se_3: "~idiese", _;
    subj_fut_3: "~idiere", _;
    vos: "~edís", "~idás", "~edite";
    imperative: "~ídete", "~ídase", "~idámonos", "~edíos", "~ídanse";
    infinitive: "~edirse";
    gerund: "~idiéndose";
    participle_m: "~edido", _;
    ),
    // Creerse: 8 words (caerse, creerse, peerse, proveerse, corroerse, desposeerse). Like Abrir, except in 12 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative tu, imperative usted, ….
    conjugation!("creerse", 8, "~~erse", like Abrir;
    stem verb: "~~- like creer";
    stem verb_fut_1_sg: "~~er- like echar";
    pres_1: _, "~~emos";
    pres_2: _, "~~éis";
    vos: "~~és", _, "~~ete";
    imperative: "~é~te", "~éas~", "~~ámonos", "~~eos", "~éans~";
    infinitive: "~~erse";
    gerund: "~~yéndose";
    participle_m: "~~ído", _;
    ),
    // Empezar: 7 words (empezar, tropezar, despezar, entropezar, trompezar, desentropezar). Like Alzarse, except in 13 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, vos imp, ….
    conjugation!("empezar", 7, "~~zar", like Alzarse;
    pres_1: "~i~zo", _;
    pres_2: "~i~zas", _;
    subj_1: "~i~ce", _;
    subj_2: "~i~ces", _;
    subj_3: _, "~i~cen";
    vos: _, _, "~~zá";
    imperative: "~i~za", "~i~ce", "~~cemos", "~~zad", "~i~cen";
    infinitive: "~~zar";
    gerund: "~~zando";
    ),
    // Regir: 7 words (elegir, corregir, colegir, reelegir, regir, recolegir). Like Echar, except in 30 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("regir", 7, "~egir", like Echar;
    stem verb_fut_1_sg: "~egir- like echar";
    pres_1: "~ijo", "~egimos";
    pres_2: "~iges", "~egís";
    impf_1: "~egía", "~egíamos";
    pret_1: "~egí", "~egimos";
    pret_3: "~igió", "~igieron";
    subj_1: "~ija", "~ijamos";
    subj_2: "~ijas", "~ijáis";
    subj_3: _, "~ijan";
    subj_impf_1: _, "~igiéramos";
    subj_impf_3: "~igiera", _;
    subj_impf_se_3: "~igiese", _;
    subj_fut_3: "~igiere", _;
    vos: "~egís", "~ijás", "~egí";
    imperative: "~ige", "~ija", "~ijamos", "~egid", "~ijan";
    infinitive: "~egir";
    gerund: "~igiendo";
    participle_m: "~egido", _;
    ),
    // Ver: 7 words (ver, ser, prever, entrever, antever, rever). Like Meter, except in 17 cells: pres_1 sg, pres_2 sg, impf_1 sg, impf_1 pl, subj_1 sg, subj_1 pl, ….
    conjugation!("ver", 7, "~er", like Meter;
    pres_1: "~eo", _;
    pres_2: "~és", _;
    impf_1: "~eía", "~eíamos";
    subj_1: "~ea", "~eamos";
    subj_2: "~eas", "~eáis";
    subj_3: _, "~ean";
    vos: derived, "~eás", derived;
    imperative: "~é", "~ea", "~eamos", _, "~ean";
    participle_m: "~isto", _;
    ),
    // Tañer: 7 words (jier, atañer, tañer, empeller, toller, creyer). Like Meter, except in 1 cell: gerund.
    conjugation!("tañer", 7, "~er", like Meter;
    stem verb: "~- like escabullirse";
    gerund: "~endo";
    ),
    // Ahumarse: 7 words (ahusarse, ahumarse, aunarse, rehusarse, rehurtarse, sahumarse). Like Aunar with `` after each pattern (`se` after e, n, r), except in 10 cells: pret_3 pl, subj_1 sg, subj_3 pl, subj_impf_se_3 sg, subj_fut_3 sg, vos imp, ….
    conjugation!("ahumarse", 7, "~u~arse", like Aunar with "", "se" after "enr";
    pret_3: _, "~u~aron";
    subj_1: "~ú~e", _;
    subj_3: _, "~ú~en";
    subj_impf_se_3: "~u~ase", _;
    subj_fut_3: "~u~are", _;
    vos: _, _, "~u~ate";
    imperative: "~ú~ate", _, "~u~émonos", "~u~aos", _;
    gerund: "~u~ándose";
    ),
    // Venirse: 7 words (venirse, desconvenirse, avenirse, desavenirse, convenirse, prevenirse). Like Venir with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("venirse", 7, "~e~irse", like Venir with "", "se" after "r";
    vos: _, _, "~e~ite";
    imperative: "~e~te", "~é~gase", "~e~gámonos", "~e~íos", "~é~ganse";
    gerund: "~i~iéndose";
    ),
    // Aterirse: 7 words (aterirse, empedernirse, arrecirse, hespirse, denegrirse, sumergirse). Like Entremeterse, except in 6 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative vosotros, infinitive.
    conjugation!("aterirse", 7, "~e~irse", like Entremeterse;
    stem verb_fut_1_sg: "~e~ir- like echar";
    pres_1: _, "~e~imos";
    pres_2: _, "~e~ís";
    vos: "~e~ís", _, "~e~ite";
    imperative: _, _, _, "~e~íos", _;
    infinitive: "~e~irse";
    ),
    // Jetiarse: 7 words (mamoniarse, patrasiarse, friquiarse, paniquiarse, jetiarse, penquiarse). Like Meliar with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("jetiarse", 7, "~iarse", like Meliar with "", "se" after "r";
    vos: _, _, "~iate";
    imperative: "~éate", "~éese", "~iémonos", "~iaos", "~éense";
    gerund: "~iándose";
    ),
    // Deshacerse: 7 words (autosatisfacerse, hacerse, rehacerse, desfacerse, deshacerse, contrahacerse). Like Hacer with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("deshacerse", 7, "~acerse", like Hacer with "", "se" after "r";
    vos: _, _, "~acete";
    imperative: "~azte", "~ágase", "~agámonos", "~aceos", "~áganse";
    gerund: "~aciéndose";
    ),
    // Relamerse: 7 words (valerse, prevalerse, relamerse, lamerse, precaverse, barrerse). Like Abatirse, except in 6 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative vosotros, infinitive.
    conjugation!("relamerse", 7, "~a~erse", like Abatirse;
    stem verb_fut_1_sg: "~a~er- like echar";
    pres_1: _, "~a~emos";
    pres_2: _, "~a~éis";
    vos: "~a~és", _, "~a~ete";
    imperative: _, _, _, "~a~eos", _;
    infinitive: "~a~erse";
    ),
    // Mover: 6 words (promover, mover, remover, conmover, amover, commover). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("mover", 6, "~over", like Echar;
    stem verb: "~ov- like conocer";
    stem verb_fut_1_sg: "~over- like echar";
    pres_1: "~uevo", "~ovemos";
    pres_2: "~ueves", "~ovéis";
    impf_1: "~ovía", "~ovíamos";
    subj_1: "~ueva", "~ovamos";
    subj_2: "~uevas", "~ováis";
    subj_3: _, "~uevan";
    vos: "~ovés", "~ovás", "~ové";
    imperative: "~ueve", "~ueva", "~ovamos", "~oved", "~uevan";
    infinitive: "~over";
    gerund: "~oviendo";
    participle_m: "~ovido", _;
    ),
    // Seguir: 6 words (proseguir, perseguir, seguir, conseguir, subseguir, reseguir). Like Traer, except in 20 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, pret_1 sg, ….
    conjugation!("seguir", 6, "~eguir", like Traer;
    stem verb_fut_1_sg: "~eguir- like echar";
    pres_1: _, "~eguimos";
    pres_2: "~igues", "~eguís";
    impf_1: "~eguía", "~eguíamos";
    pret_1: "~eguí", "~eguimos";
    pret_3: "~iguió", "~iguieron";
    subj_impf_1: _, "~iguiéramos";
    subj_impf_3: "~iguiera", _;
    subj_impf_se_3: "~iguiese", _;
    subj_fut_3: "~iguiere", _;
    vos: "~eguís", _, "~eguí";
    imperative: "~igue", _, _, "~eguid", _;
    infinitive: "~eguir";
    gerund: "~iguiendo";
    participle_m: "~eguido", _;
    ),
    // Apajuilar: 6 words (acuclillar, encuclillar, apajuilar, apulismar, aculillar, encuitar). Like Rilarse.
    conjugation!("apajuilar", 6, "~i~ar", like Rilarse;
    ),
    // Recogerse: 6 words (encogerse, desencogerse, sobrecogerse, recogerse, cogerse, acogerse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("recogerse", 6, "~ogerse", like Echar;
    stem verb: "~og- like conocer";
    stem verb_fut_1_sg: "~oger- like echar";
    pres_1: "~ojo", "~ogemos";
    pres_2: "~oges", "~ogéis";
    impf_1: "~ogía", "~ogíamos";
    subj_1: "~oja", "~ojamos";
    subj_2: "~ojas", "~ojáis";
    subj_3: _, "~ojan";
    vos: "~ogés", "~ojás", "~ogete";
    imperative: "~ógete", "~ójase", "~ojámonos", "~ogeos", "~ójanse";
    infinitive: "~ogerse";
    gerund: "~ogiéndose";
    participle_m: "~ogido", _;
    ),
    // Oír: 6 words (oír, embaír, desvaír, desoír, trasoír, entreoír). Like Instruir, except in 14 cells: pres_1 sg, pres_1 pl, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, ….
    conjugation!("oír", 6, "~ír", like Instruir;
    stem verb: "~- like creer";
    pres_1: "~igo", "~ímos";
    subj_1: "~iga", "~igamos";
    subj_2: "~igas", "~igáis";
    subj_3: _, "~igan";
    vos: _, "~igás", _;
    imperative: _, "~iga", "~igamos", "~íd", "~igan";
    infinitive: "~ír";
    participle_m: "~ído", _;
    ),
    // Desdecir: 6 words (bendecir, maldecir, contradecir, desdecir, predecir, condecir). Like Traer, except in 24 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, fut_1 sg, ….
    conjugation!("desdecir", 6, "~ecir", like Traer;
    stem verb: "~ij- like reducir";
    pres_1: _, "~ecimos";
    pres_2: "~ices", "~ecís";
    impf_1: "~ecía", "~ecíamos";
    fut_1: "~eciré|~iré", "~eciremos|~iremos";
    fut_2: "~ecirás|~irás", "~eciréis|~iréis";
    fut_3: "~ecirá|~irá", "~ecirán|~irán";
    cond_1: "~eciría|~iría", "~eciríamos|~iríamos";
    cond_2: "~ecirías|~irías", "~eciríais|~iríais";
    cond_3: derived, "~ecirían|~irían";
    vos: "~ecís", _, "~ecí";
    imperative: "~ice", _, _, "~ecid", _;
    infinitive: "~ecir";
    gerund: "~iciendo";
    participle_m: "~icho", _;
    ),
    // Entredormirse: 6 words (entredormirse, podrirse, adormirse, dormirse, morirse, repodrirse). Like Musirse, except in 19 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("entredormirse", 6, "~o~irse", like Musirse;
    stem verb_fut_1_sg: "~o~ir- like echar";
    pres_1: "~ue~o", "~o~imos";
    pres_2: "~ue~es", "~o~ís";
    impf_1: "~o~ía", "~o~íamos";
    pret_1: "~o~í", "~o~imos";
    subj_1: "~ue~a", _;
    subj_2: "~ue~as", _;
    subj_3: _, "~ue~an";
    vos: "~o~ís", _, "~o~ite";
    imperative: "~ué~ete", "~ué~ase", _, "~o~íos", "~ué~anse";
    infinitive: "~o~irse";
    participle_m: "~o~ido", _;
    ),
    // Tender: 5 words (discerner, cerner, jerver, herver, jerber). Like Conocer, except in 12 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, ….
    conjugation!("tender", 5, "~~er", like Conocer;
    pres_1: "~i~o", _;
    pres_2: "~i~es", _;
    subj_1: "~i~a", "~~amos";
    subj_2: "~i~as", "~~áis";
    subj_3: _, "~i~an";
    vos: _, "~~ás", _;
    imperative: "~i~e", "~i~a", "~~amos", _, "~i~an";
    ),
    // Caer: 5 words (caer, raer, recaer, decaer, descaer). Like Traer.
    conjugation!("caer", 5, "~er", like Traer;
    stem verb: "~- like creer";
    ),
    // Querer: 5 words (querer, malquerer, bienquerer, desquerer, requerer). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("querer", 5, "~erer", like Echar;
    stem verb: "~is- like poner";
    stem verb_fut_1_sg: "~err- like echar";
    pres_1: "~iero", "~eremos";
    pres_2: "~ieres", "~eréis";
    impf_1: "~ería", "~eríamos";
    subj_1: "~iera", "~eramos";
    subj_2: "~ieras", "~eráis";
    subj_3: _, "~ieran";
    vos: "~erés", "~erás", "~eré";
    imperative: "~iere", "~iera", "~eramos", "~ered", "~ieran";
    infinitive: "~erer";
    gerund: "~eriendo";
    participle_m: "~erido", _;
    ),
    // Huir: 5 words (fuir, huir, fluir, fruir, luir). Like Instruir, except in 4 cells: pres_2 pl, pret_1 sg, vos ind, vos imp.
    conjugation!("huir", 5, "~ir", like Instruir;
    pres_2: _, "~is";
    pret_1: "~i", _;
    vos: "~is", _, "~i";
    ),
    // Cocer: 5 words (cocer, escocer, recocer, descocer, precocer). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("cocer", 5, "~ocer", like Echar;
    stem verb: "~oc- like conocer";
    stem verb_fut_1_sg: "~ocer- like echar";
    pres_1: "~uezo", "~ocemos";
    pres_2: "~ueces", "~océis";
    impf_1: "~ocía", "~ocíamos";
    subj_1: "~ueza", "~ozamos";
    subj_2: "~uezas", "~ozáis";
    subj_3: _, "~uezan";
    vos: "~océs", "~ozás", "~océ";
    imperative: "~uece", "~ueza", "~ozamos", "~oced", "~uezan";
    infinitive: "~ocer";
    gerund: "~ociendo";
    participle_m: "~ocido", _;
    ),
    // Delinquir: 5 words (delinquir, derelinquir, jaquir, derrelinquir, relinquir). Like Sacar, except in 21 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("delinquir", 5, "~quir", like Sacar;
    stem verb: "~qu- like conocer";
    stem verb_fut_1_sg: "~quir- like echar";
    pres_1: _, "~quimos";
    pres_2: "~ques", "~quís";
    impf_1: "~quía", "~quíamos";
    subj_1: "~ca", "~camos";
    subj_2: "~cas", "~cáis";
    subj_3: _, "~can";
    vos: "~quís", "~cás", "~quí";
    imperative: "~que", "~ca", "~camos", "~quid", "~can";
    infinitive: "~quir";
    gerund: "~quiendo";
    participle_m: "~quido", _;
    ),
    // Afligirse: 5 words (autoexigirse, afligirse, arrigirse, dirigirse, erigirse). Like Regir, except in 14 cells: pres_1 pl, pres_2 pl, impf_1 sg, impf_1 pl, vos ind, vos imp, ….
    conjugation!("afligirse", 5, "~igirse", like Regir;
    stem verb: "~ig- like conocer";
    stem verb_fut_1_sg: "~igir- like echar";
    pres_1: _, "~igimos";
    pres_2: _, "~igís";
    impf_1: "~igía", "~igíamos";
    vos: "~igís", _, "~igite";
    imperative: "~ígete", "~íjase", "~ijámonos", "~igíos", "~íjanse";
    infinitive: "~igirse";
    gerund: "~igiéndose";
    participle_m: "~igido", _;
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
    // Teñirse: 5 words (teñirse, ceñirse, desceñirse, desteñirse, estreñirse). Like Ceñir with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("teñirse", 5, "~eñirse", like Ceñir with "", "se" after "r";
    vos: _, _, "~eñite";
    imperative: "~íñete", "~íñase", "~iñámonos", "~eñíos", "~íñanse";
    gerund: "~iñéndose";
    ),
    // Trocarse: 5 words (aclocarse, trocarse, enclocarse, enllocarse, trastrocarse). Like Trocar with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("trocarse", 5, "~ocarse", like Trocar with "", "se" after "r";
    vos: _, _, "~ocate";
    imperative: "~uécate", "~uéquese", "~oquémonos", "~ocaos", "~uéquense";
    gerund: "~ocándose";
    ),
    // Revestirse: 5 words (desvestirse, revestirse, sobrevestirse, travestirse, vestirse). Like Incautarse, except in 30 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("revestirse", 5, "~e~tirse", like Incautarse;
    stem verb_fut_1_sg: "~e~tir- like echar";
    pres_1: "~i~to", "~e~timos";
    pres_2: "~i~tes", "~e~tís";
    impf_1: "~e~tía", "~e~tíamos";
    pret_1: "~e~tí", "~e~timos";
    pret_3: "~i~tió", "~i~tieron";
    subj_1: "~i~ta", "~i~tamos";
    subj_2: "~i~tas", "~i~táis";
    subj_3: _, "~i~tan";
    subj_impf_1: _, "~i~tiéramos";
    subj_impf_3: "~i~tiera", _;
    subj_impf_se_3: "~i~tiese", _;
    subj_fut_3: "~i~tiere", _;
    vos: "~e~tís", "~i~tás", "~e~tite";
    imperative: "~í~tete", "~í~tase", "~i~támonos", "~e~tíos", "~í~tanse";
    infinitive: "~e~tirse";
    gerund: "~i~tiéndose";
    participle_m: "~e~tido", _;
    ),
    // Freírse: 5 words (reírse, desleírse, engreírse, sonreírse, freírse). Like Refreír with `` after each pattern (`se` after a, n, r), except in 11 cells: impf_1 sg, pret_3 pl, cond_3 sg, subj_1 sg, subj_3 pl, subj_impf_3 sg, ….
    conjugation!("freírse", 5, "~eírse", like Refreír with "", "se" after "anr";
    impf_1: "~eía", _;
    pret_3: _, "~ieron";
    cond_3: "~eiría", _;
    subj_1: "~ía", _;
    subj_3: _, "~ían";
    subj_impf_3: "~iera", _;
    vos: _, _, "~eíte";
    imperative: "~íete", _, "~iámonos", "~eíos", _;
    gerund: "~iéndose";
    ),
    // Dormir: 5 words (dormir, adormir, maldormir, podrir, repodrir). Like Entredormirse, except in 8 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("dormir", 5, "~o~ir", like Entredormirse;
    vos: _, _, "~o~í";
    imperative: "~ue~e", "~ue~a", "~u~amos", "~o~id", "~ue~an";
    infinitive: "~o~ir";
    gerund: "~u~iendo";
    ),
    // Salir: 5 words (salir, asir, desasir, sobresalir, resalir). Like Curtir, except in 11 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("salir", 5, "~ir", like Curtir;
    stem verb_fut_1_sg: "~dr- like echar";
    pres_1: "~go", _;
    subj_1: "~ga", "~gamos";
    subj_2: "~gas", "~gáis";
    subj_3: _, "~gan";
    vos: _, "~gás", _;
    imperative: "~", "~ga", "~gamos", _, "~gan";
    ),
    // Decir: 5 words (decir, antedecir, redecir, interdecir, entredecir). Like Desdecir, except in 10 cells: fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, fut_3 pl, cond_1 sg, ….
    conjugation!("decir", 5, "~ecir", like Desdecir;
    stem verb_fut_1_sg: "~ir- like echar";
    fut_1: _, derived;
    fut_2: derived, derived;
    fut_3: derived, derived;
    cond_1: derived, derived;
    cond_2: derived, derived;
    cond_3: _, derived;
    ),
    // Agorar: 4 words (agorar, engorar, cogolmar, gorar). Like Volar, except in 8 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("agorar", 4, "~o~ar", like Volar;
    pres_1: "~üe~o", _;
    pres_2: "~üe~as", _;
    subj_1: "~üe~e", _;
    subj_2: "~üe~es", _;
    subj_3: _, "~üe~en";
    imperative: "~üe~a", "~üe~e", _, _, "~üe~en";
    ),
    // Saber: 4 words (saber, caber, haber, resaber). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("saber", 4, "~aber", like Echar;
    stem verb: "~up- like poner";
    stem verb_fut_1_sg: "~abr- like echar";
    pres_1: "~é", "~abemos";
    pres_2: "~abes", "~abéis";
    impf_1: "~abía", "~abíamos";
    subj_1: "~epa", "~epamos";
    subj_2: "~epas", "~epáis";
    subj_3: _, "~epan";
    vos: "~abés", "~epás", "~abé";
    imperative: "~abe", "~epa", "~epamos", "~abed", "~epan";
    infinitive: "~aber";
    gerund: "~abiendo";
    participle_m: "~abido", _;
    ),
    // Forzar: 4 words (almorzar, forzar, esforzar, reforzar). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("forzar", 4, "~o~zar", like Incautarse;
    stem verb: "~o~- like realizar";
    stem verb_fut_1_sg: "~o~zar- like echar";
    pres_1: "~ue~zo", "~o~zamos";
    pres_2: "~ue~zas", "~o~záis";
    impf_1: "~o~zaba", "~o~zábamos";
    subj_1: "~ue~ce", "~o~cemos";
    subj_2: "~ue~ces", "~o~céis";
    subj_3: _, "~ue~cen";
    vos: "~o~zás", "~o~cés", "~o~zá";
    imperative: "~ue~za", "~ue~ce", "~o~cemos", "~o~zad", "~ue~cen";
    infinitive: "~o~zar";
    gerund: "~o~zando";
    participle_m: "~o~zado", _;
    ),
    // Adquirir: 4 words (adquirir, inquirir, perquirir, readquirir). Like Abrir, except in 9 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("adquirir", 4, "~~ir", like Abrir;
    pres_1: "~e~o", _;
    pres_2: "~e~es", _;
    subj_1: "~e~a", _;
    subj_2: "~e~as", _;
    subj_3: _, "~e~an";
    imperative: "~e~e", "~e~a", _, _, "~e~an";
    participle_m: "~~ido", _;
    ),
    // Cohibir: 4 words (prohibir, cohibir, rehinchir, desprohibir). Like Desvivirse, except in 13 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, vos imp, ….
    conjugation!("cohibir", 4, "~i~ir", like Desvivirse;
    pres_1: "~í~o", _;
    pres_2: "~í~es", _;
    subj_1: "~í~a", _;
    subj_2: "~í~as", _;
    subj_3: _, "~í~an";
    vos: _, _, "~i~í";
    imperative: "~í~e", "~í~a", "~i~amos", "~i~id", "~í~an";
    infinitive: "~i~ir";
    gerund: "~i~iendo";
    ),
    // Morir: 4 words (morir, premorir, desmorir, entremorir). Like Dormir, except in 1 cell: participle_m sg.
    conjugation!("morir", 4, "~o~ir", like Dormir;
    participle_m: "~ue~to", _;
    ),
    // Distinguir: 4 words (distinguir, extinguir, subdistinguir, desextinguir). Like Curtir, except in 12 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, vos ind, ….
    conjugation!("distinguir", 4, "~uir", like Curtir;
    stem verb: "~u- like conocer";
    stem verb_fut_1_sg: "~uir- like echar";
    pres_1: _, "~uimos";
    pres_2: "~ues", "~uís";
    impf_1: "~uía", "~uíamos";
    vos: "~uís", _, "~uí";
    imperative: "~ue", _, _, "~uid", _;
    infinitive: "~uir";
    gerund: "~uiendo";
    participle_m: "~uido", _;
    ),
    // Torcer: 4 words (torcer, retorcer, destorcer, contorcer). Like Forzar, except in 21 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("torcer", 4, "~o~cer", like Forzar;
    stem verb: "~o~c- like conocer";
    stem verb_fut_1_sg: "~o~cer- like echar";
    pres_1: _, "~o~cemos";
    pres_2: "~ue~ces", "~o~céis";
    impf_1: "~o~cía", "~o~cíamos";
    subj_1: "~ue~za", "~o~zamos";
    subj_2: "~ue~zas", "~o~záis";
    subj_3: _, "~ue~zan";
    vos: "~o~cés", "~o~zás", "~o~cé";
    imperative: "~ue~ce", "~ue~za", "~o~zamos", "~o~ced", "~ue~zan";
    infinitive: "~o~cer";
    gerund: "~o~ciendo";
    participle_m: "~o~cido", _;
    ),
    // Desforzarse: 4 words (desforzarse, forzarse, esforzarse, reforzarse). Like Forzar with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("desforzarse", 4, "~o~zarse", like Forzar with "", "se" after "r";
    vos: _, _, "~o~zate";
    imperative: "~ué~zate", "~ué~cese", "~o~cémonos", "~o~zaos", "~ué~cense";
    gerund: "~o~zándose";
    ),
    // Regirse: 4 words (autocorregirse, autoelegirse, corregirse, regirse). Like Regir with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("regirse", 4, "~egirse", like Regir with "", "se" after "r";
    vos: _, _, "~egite";
    imperative: "~ígete", "~íjase", "~ijámonos", "~egíos", "~íjanse";
    gerund: "~igiéndose";
    ),
    // Fraguarse: 4 words (paniaguarse, sobreaguarse, desaguarse, fraguarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("fraguarse", 4, "~a~uarse", like Incautarse;
    stem verb: "~a~- like fraguar";
    stem verb_fut_1_sg: "~a~uar- like echar";
    pres_1: "~a~uo", "~a~uamos";
    pres_2: "~a~uas", "~a~uáis";
    impf_1: "~a~uaba", "~a~uábamos";
    subj_1: "~a~üe", "~a~üemos";
    subj_2: "~a~ües", "~a~üéis";
    subj_3: _, "~a~üen";
    vos: "~a~uás", "~a~üés", "~a~uate";
    imperative: "~á~uate", "~á~üese", "~a~üémonos", "~a~uaos", "~á~üense";
    infinitive: "~a~uarse";
    gerund: "~a~uándose";
    participle_m: "~a~uado", _;
    ),
    // Moverse: 4 words (lloverse, conmoverse, removerse, moverse). Like Mover with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("moverse", 4, "~overse", like Mover with "", "se" after "r";
    vos: _, _, "~ovete";
    imperative: "~uévete", "~uévase", "~ovámonos", "~oveos", "~uévanse";
    gerund: "~oviéndose";
    ),
    // Verse: 4 words (reverse, verse, arderse, entreverse). Like Ver with `` after each pattern (`se` after r), except in 8 cells: vos ind, vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, ….
    conjugation!("verse", 4, "~erse", like Ver with "", "se" after "r";
    vos: "~és", _, "~ete";
    imperative: "~ete", "~éase", "~eámonos", "~eos", "~éanse";
    gerund: "~iéndose";
    ),
    // Criarse: 4 words (criarse, liarse, triarse, fiarse). Like Guiar with `` after each pattern (`se` after e, n, r), except in 11 cells: pret_1 sg, pret_3 pl, subj_1 sg, subj_3 pl, subj_impf_se_3 sg, subj_fut_3 sg, ….
    conjugation!("criarse", 4, "~iarse", like Guiar with "", "se" after "enr";
    pret_1: "~ie", _;
    pret_3: _, "~iaron";
    subj_1: "~íe", _;
    subj_3: _, "~íen";
    subj_impf_se_3: "~iase", _;
    subj_fut_3: "~iare", _;
    vos: _, _, "~iate";
    imperative: "~íate", _, "~iémonos", "~iaos", _;
    gerund: "~iándose";
    ),
    // Punzarse: 4 words (anarquizarse, barroquizarse, endulzarse, punzarse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("punzarse", 4, "~u~zarse", like Incautarse;
    stem verb: "~u~- like realizar";
    stem verb_fut_1_sg: "~u~zar- like echar";
    pres_1: "~u~zo", "~u~zamos";
    pres_2: "~u~zas", "~u~záis";
    impf_1: "~u~zaba", "~u~zábamos";
    subj_1: "~u~ce", "~u~cemos";
    subj_2: "~u~ces", "~u~céis";
    subj_3: _, "~u~cen";
    vos: "~u~zás", "~u~cés", "~u~zate";
    imperative: "~ú~zate", "~ú~cese", "~u~cémonos", "~u~zaos", "~ú~cense";
    infinitive: "~u~zarse";
    gerund: "~u~zándose";
    participle_m: "~u~zado", _;
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
    // Torcerse: 4 words (contorcerse, retorcerse, torcerse, destorcerse). Like Torcer with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("torcerse", 4, "~o~cerse", like Torcer with "", "se" after "r";
    vos: _, _, "~o~cete";
    imperative: "~ué~cete", "~ué~zase", "~o~zámonos", "~o~ceos", "~ué~zanse";
    gerund: "~o~ciéndose";
    ),
    // Servirse: 4 words (rendirse, rehenchirse, henchirse, servirse). Like Rendir with `` after each pattern (`se` after a, n, r), except in 11 cells: impf_1 sg, pret_3 pl, cond_3 sg, subj_1 sg, subj_3 pl, subj_impf_3 sg, ….
    conjugation!("servirse", 4, "~e~irse", like Rendir with "", "se" after "anr";
    impf_1: "~e~ía", _;
    pret_3: _, "~i~ieron";
    cond_3: "~e~iría", _;
    subj_1: "~i~a", _;
    subj_3: _, "~i~an";
    subj_impf_3: "~i~iera", _;
    vos: _, _, "~e~íte";
    imperative: "~i~ete", _, "~i~ámonos", "~e~íos", _;
    gerund: "~i~iéndose";
    ),
    // Comenzar: 4 words (comenzar, enlenzar, recomenzar, encomenzar). Like Realizar with `ˇ` after each pattern.
    conjugation!("comenzar", 4, "~zarˇ", like Realizar with "ˇ";
    ),
    // Vencerse: 4 words (remecerse, convencerse, vencerse, desconvencerse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("vencerse", 4, "~e~cerse", like Incautarse;
    stem verb: "~e~c- like conocer";
    stem verb_fut_1_sg: "~e~cer- like echar";
    pres_1: "~e~zo", "~e~cemos";
    pres_2: "~e~ces", "~e~céis";
    impf_1: "~e~cía", "~e~cíamos";
    subj_1: "~e~za", "~e~zamos";
    subj_2: "~e~zas", "~e~záis";
    subj_3: _, "~e~zan";
    vos: "~e~cés", "~e~zás", "~e~cete";
    imperative: "~é~cete", "~é~zase", "~e~zámonos", "~e~ceos", "~é~zanse";
    infinitive: "~e~cerse";
    gerund: "~e~ciéndose";
    participle_m: "~e~cido", _;
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
    // Argüir: 3 words (argüir, reargüir, redargüir). Like Inmiscuirse, except in 16 cells: pres_1 pl, pres_2 pl, impf_1 sg, impf_1 pl, pret_1 sg, pret_1 pl, ….
    conjugation!("argüir", 3, "~üir", like Inmiscuirse;
    stem verb_fut_1_sg: "~üir- like echar";
    pres_1: _, "~üimos";
    pres_2: _, "~üís";
    impf_1: "~üía", "~üíamos";
    pret_1: "~üí", "~üimos";
    vos: "~üís", _, "~üí";
    imperative: "~uye", "~uya", "~uyamos", "~üid", "~uyan";
    infinitive: "~üir";
    gerund: "~uyendo";
    participle_m: "~üido", _;
    ),
    // Jugar: 3 words (jugar, rejugar, sobrejugar). Like Cerrar, except in 13 cells: pres_1 sg, pres_2 sg, pret_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, ….
    conjugation!("jugar", 3, "~~ar", like Cerrar;
    pres_1: "~e~o", _;
    pres_2: "~e~as", _;
    pret_1: "~~ué", _;
    subj_1: "~e~ue", "~~uemos";
    subj_2: "~e~ues", "~~uéis";
    subj_3: _, "~e~uen";
    vos: _, "~~ués", _;
    imperative: "~e~a", "~e~ue", "~~uemos", _, "~e~uen";
    ),
    // Romper: 3 words (romper, derromper, arromper). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("romper", 3, "~mper", like Echar;
    stem verb: "~mp- like conocer";
    stem verb_fut_1_sg: "~mper- like echar";
    pres_1: "~mpo", "~mpemos";
    pres_2: "~mpes", "~mpéis";
    impf_1: "~mpía", "~mpíamos";
    subj_1: "~mpa", "~mpamos";
    subj_2: "~mpas", "~mpáis";
    subj_3: _, "~mpan";
    vos: "~mpés", "~mpás", "~mpé";
    imperative: "~mpe", "~mpa", "~mpamos", "~mped", "~mpan";
    infinitive: "~mper";
    gerund: "~mpiendo";
    participle_m: "~to", _;
    ),
    // Valer: 3 words (valer, equivaler, prevaler). Like Salir, except in 7 cells: pres_1 pl, pres_2 pl, vos ind, vos imp, imperative tu, imperative vosotros, ….
    conjugation!("valer", 3, "~er", like Salir;
    pres_1: _, "~emos";
    pres_2: _, "~éis";
    vos: "~és", _, "~é";
    imperative: "~e", _, _, "~ed", _;
    infinitive: "~er";
    ),
    // Apucuyar: 3 words (intrusar, apucuyar, empuyar). Like Emputarse.
    conjugation!("apucuyar", 3, "~u~ar", like Emputarse;
    ),
    // Antojar: 3 words (antojar, hopar, ñangotar). Like Acomodarse.
    conjugation!("antojar", 3, "~o~ar", like Acomodarse;
    ),
    // Enraizarse: 3 words (enraizarse, europeizarse, corporeizarse). Like Enraizar with `` after each pattern (`se` after e, n, r), except in 10 cells: pret_3 pl, subj_1 sg, subj_3 pl, subj_impf_se_3 sg, subj_fut_3 sg, vos imp, ….
    conjugation!("enraizarse", 3, "~izarse", like Enraizar with "", "se" after "enr";
    pret_3: _, "~izaron";
    subj_1: "~íce", _;
    subj_3: _, "~ícen";
    subj_impf_se_3: "~izase", _;
    subj_fut_3: "~izare", _;
    vos: _, _, "~izate";
    imperative: "~ízate", _, "~icémonos", "~izaos", _;
    gerund: "~izándose";
    ),
    // Escocerse: 3 words (escocerse, recocerse, cocerse). Like Cocer with `` after each pattern (`se` after r), except in 11 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("escocerse", 3, "~ocerse", like Cocer with "", "se" after "r";
    vos: _, _, "~ocete";
    imperative: "~uécete", "~uézase", "~ozámonos", "~oceos", "~uézanse";
    gerund: "~ociéndose";
    participle_m: "~ocido|~ocho", "~ocidos|~ochos";
    participle_f: "~ocida|~ocha", "~ocidas|~ochas";
    ),
    // Seguirse: 3 words (conseguirse, subseguirse, seguirse). Like Seguir with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("seguirse", 3, "~eguirse", like Seguir with "", "se" after "r";
    vos: _, _, "~eguite";
    imperative: "~íguete", "~ígase", "~igámonos", "~eguíos", "~íganse";
    gerund: "~iguiéndose";
    ),
    // Azoarse: 3 words (abarloarse, azoarse, barloarse). Like Echar, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("azoarse", 3, "~oarse", like Echar;
    stem verb: "~o- like echar";
    stem verb_fut_1_sg: "~oar- like echar";
    pres_1: "~oo", "~oamos";
    pres_2: "~oas", "~oáis";
    impf_1: "~oaba", "~oábamos";
    subj_1: "~oe", "~oemos";
    subj_2: "~oes", "~oéis";
    subj_3: _, "~oen";
    vos: "~oás", "~oés", "~oate";
    imperative: "~óate", "~óese", "~oémonos", "~oaos", "~óense";
    infinitive: "~oarse";
    gerund: "~oándose";
    participle_m: "~oado", _;
    ),
    // Nacerse: 3 words (complacerse, aplacerse, nacerse). Like Deshacerse, except in 20 cells: pres_1 sg, pret_1 sg, pret_1 pl, pret_3 sg, pret_3 pl, subj_1 sg, ….
    conjugation!("nacerse", 3, "~acerse", like Deshacerse;
    stem verb_fut_1_sg: "~acer- like echar";
    pres_1: "~azco", _;
    pret_1: "~ací", "~acimos";
    pret_3: "~ació|~ugo", "~acieron|~uguieron";
    subj_1: "~azca", "~azcamos";
    subj_2: "~azcas", "~azcáis";
    subj_3: _, "~azcan";
    subj_impf_1: _, "~aciéramos";
    subj_impf_3: "~aciera|~uguiera", _;
    subj_impf_se_3: "~aciese|~uguiese", _;
    subj_fut_3: "~aciere|~uguiere", _;
    vos: _, "~azcás", _;
    imperative: "~ácete", "~ázcase", "~azcámonos", _, "~ázcanse";
    participle_m: "~acido", _;
    ),
    // Trenzarse: 3 words (trenzarse, destrenzarse, homogeneizarse). Like Vencerse, except in 21 cells: pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, subj_1 sg, ….
    conjugation!("trenzarse", 3, "~e~zarse", like Vencerse;
    stem verb: "~e~- like realizar";
    stem verb_fut_1_sg: "~e~zar- like echar";
    pres_1: _, "~e~zamos";
    pres_2: "~e~zas", "~e~záis";
    impf_1: "~e~zaba", "~e~zábamos";
    subj_1: "~e~ce", "~e~cemos";
    subj_2: "~e~ces", "~e~céis";
    subj_3: _, "~e~cen";
    vos: "~e~zás", "~e~cés", "~e~zate";
    imperative: "~é~zate", "~é~cese", "~e~cémonos", "~e~zaos", "~é~cense";
    infinitive: "~e~zarse";
    gerund: "~e~zándose";
    participle_m: "~e~zado", _;
    ),
    // Cubrirse: 3 words (descubrirse, cubrirse, encubrirse).
    conjugation!("cubrirse", 3, "~u~~irse";
    stem verb: "~u~~- like conocer";
    stem verb_fut_1_sg: "~u~~ir- like echar";
    pres_1: "~u~~o", "~u~~imos";
    pres_2: "~u~~es", "~u~~ís";
    impf_1: "~u~~ía", "~u~~íamos";
    subj_1: "~u~~a", "~u~~amos";
    subj_2: "~u~~as", "~u~~áis";
    subj_3: _, "~u~~an";
    vos: "~u~~ís", "~u~~ás", "~u~~ite";
    imperative: "~ú~~ete", "~ú~~ase", "~u~~ámonos", "~u~~íos", "~ú~~anse";
    infinitive: "~u~~irse";
    gerund: "~u~~iéndose";
    participle_m: "~u~ie~to", _;
    ),
    // Derretirse: 3 words (derretirse, competirse, repetirse). Like Echar, except in 30 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("derretirse", 3, "~etirse", like Echar;
    stem verb_fut_1_sg: "~etir- like echar";
    pres_1: "~ito", "~etimos";
    pres_2: "~ites", "~etís";
    impf_1: "~etía", "~etíamos";
    pret_1: "~etí", "~etimos";
    pret_3: "~itió", "~itieron";
    subj_1: "~ita", "~itamos";
    subj_2: "~itas", "~itáis";
    subj_3: _, "~itan";
    subj_impf_1: _, "~itiéramos";
    subj_impf_3: "~itiera", _;
    subj_impf_se_3: "~itiese", _;
    subj_fut_3: "~itiere", _;
    vos: "~etís", "~itás", "~etite";
    imperative: "~ítete", "~ítase", "~itámonos", "~etíos", "~ítanse";
    infinitive: "~etirse";
    gerund: "~itiéndose";
    participle_m: "~etido", _;
    ),
    // Fingirse: 3 words (refringirse, fingirse, atingirse). Like Incautarse, except in 22 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("fingirse", 3, "~i~girse", like Incautarse;
    stem verb: "~i~g- like conocer";
    stem verb_fut_1_sg: "~i~gir- like echar";
    pres_1: "~i~jo", "~i~gimos";
    pres_2: "~i~ges", "~i~gís";
    impf_1: "~i~gía", "~i~gíamos";
    subj_1: "~i~ja", "~i~jamos";
    subj_2: "~i~jas", "~i~jáis";
    subj_3: _, "~i~jan";
    vos: "~i~gís", "~i~jás", "~i~gite";
    imperative: "~í~gete", "~í~jase", "~i~jámonos", "~i~gíos", "~í~janse";
    infinitive: "~i~girse";
    gerund: "~i~giéndose";
    participle_m: "~i~gido", _;
    ),
    // Azonzarse: 3 words (azonzarse, desgonzarse, tronzarse). Like Forzar with `` after each pattern (`se` after r), except in 12 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, vos imp, ….
    conjugation!("azonzarse", 3, "~o~zarse", like Forzar with "", "se" after "r";
    pres_1: "~o~zo", _;
    pres_2: "~o~zas", _;
    subj_1: "~o~ce", _;
    subj_2: "~o~ces", _;
    subj_3: _, "~o~cen";
    vos: _, _, "~o~zate";
    imperative: "~ó~zate", "~ó~cese", "~o~cémonos", "~o~zaos", "~ó~cense";
    gerund: "~o~zándose";
    ),
    // Asirse: 2 words (abrirse, asirse). Like Echar, except in 32 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("asirse", 2, "a~irse", like Echar;
    pres_1: "a~go", "a~imos";
    pres_2: "a~es", "a~ís";
    impf_1: "a~ía", "a~íamos";
    pret_1: "a~í", "a~imos";
    pret_3: "a~ió", "a~ieron";
    fut_1: "a~iré", _;
    cond_3: "a~iría", _;
    subj_1: "a~ga", "a~gamos";
    subj_2: "a~gas", "a~gáis";
    subj_3: _, "a~gan";
    subj_impf_1: _, "a~iéramos";
    subj_impf_3: "a~iera", _;
    subj_impf_se_3: "a~iese", _;
    subj_fut_3: "a~iere", _;
    vos: "a~ís", "a~gás", "a~ite";
    imperative: "á~ete", "á~gase", "a~gámonos", "a~íos", "á~ganse";
    infinitive: "a~irse";
    gerund: "a~iéndose";
    participle_m: "a~ido", _;
    ),
    // Reírse: 1 words (embaírse). Like Creerse, except in 25 cells: pres_1 pl, pres_2 pl, pres_3 sg, pres_3 pl, subj_impf_1 sg, subj_impf_2 sg, ….
    conjugation!("reírse", 1, "~~írse", like Creerse;
    stem verb_fut_1_sg: "~~ir- like echar";
    pres_1: _, "~~ímos";
    pres_2: _, "~~ís";
    pres_3: "~~e", "~~en";
    subj_impf_1: "~~yera", _;
    subj_impf_2: "~~yeras", "~~yerais";
    subj_impf_3: derived, "~~yeran";
    subj_impf_se_1: "~~yese", "~~yésemos";
    subj_impf_se_2: "~~yeses", "~~yeseis";
    subj_impf_se_3: derived, "~~yesen";
    subj_fut_1: "~~yere", "~~yéremos";
    subj_fut_2: "~~yeres", "~~yereis";
    subj_fut_3: derived, "~~yeren";
    vos: "~~ís", _, "~~íte";
    imperative: _, _, _, "~~íos", _;
    infinitive: "~~írse";
    ),
    // Ir: 1 words (ir).
    conjugation!("ir", 1, "ir";
    pres_1: "voy", "vamos";
    pres_2: "vas", "vais";
    impf_1: "iba", "íbamos";
    pret_1: "fui", "fuimos";
    pret_3: "fue", "fueron";
    fut_1: "iré", _;
    cond_3: "iría", _;
    subj_1: "vaya", "vayamos";
    subj_2: "vayas", "vayáis";
    subj_3: _, "vayan";
    subj_impf_1: _, "fuéramos";
    subj_impf_3: "fuera", _;
    subj_impf_se_3: "fuese", _;
    subj_fut_3: "fuere", _;
    vos: _, "vayás", "andá";
    imperative: "ve", "vaya", "vamos|vayamos", "id", "vayan";
    infinitive: "ir";
    gerund: "yendo";
    participle_m: "ido", _;
    ),
    // Ingrirse: 1 words (ingrirse). Like Echar, except in 32 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("ingrirse", 1, "i~irse", like Echar;
    pres_1: "i~o", "i~imos";
    pres_2: "i~es", "i~ís";
    impf_1: "i~ía", "i~íamos";
    pret_1: "i~í", "i~imos";
    pret_3: "i~ió", "i~ieron";
    fut_1: "i~iré", _;
    cond_3: "i~iría", _;
    subj_1: "i~a", "i~amos";
    subj_2: "i~as", "i~áis";
    subj_3: _, "i~an";
    subj_impf_1: _, "i~iéramos";
    subj_impf_3: "i~iera", _;
    subj_impf_se_3: "i~iese", _;
    subj_fut_3: "i~iere", _;
    vos: "i~ís", "i~ás", "i~ite";
    imperative: "í~ete", "í~ase", "i~ámonos", "i~íos", "í~anse";
    infinitive: "i~irse";
    gerund: "i~iéndose";
    participle_m: "i~ido", _;
    ),
    // Irse: 1 words (irse). Like Ir, except in 9 cells: vos ind, vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, ….
    conjugation!("irse", 1, "irse", like Ir;
    vos: "vas", _, "andate";
    imperative: "vete|ándate", "váyase", "vayámonos|vámonos", "idos", "váyanse";
    infinitive: "irse";
    gerund: "yéndose";
    ),
    // Unirse: 1 words (unirse). Like Echar, except in 32 cells: pres_1 sg, pres_1 pl, pres_2 sg, pres_2 pl, impf_1 sg, impf_1 pl, ….
    conjugation!("unirse", 1, "u~irse", like Echar;
    pres_1: "u~o", "u~imos";
    pres_2: "u~es", "u~ís";
    impf_1: "u~ía", "u~íamos";
    pret_1: "u~í", "u~imos";
    pret_3: "u~ió", "u~ieron";
    fut_1: "u~iré", _;
    cond_3: "u~iría", _;
    subj_1: "u~a", "u~amos";
    subj_2: "u~as", "u~áis";
    subj_3: _, "u~an";
    subj_impf_1: _, "u~iéramos";
    subj_impf_3: "u~iera", _;
    subj_impf_se_3: "u~iese", _;
    subj_fut_3: "u~iere", _;
    vos: "u~ís", "u~ás", "u~ite";
    imperative: "ú~ete", "ú~ase", "u~ámonos", "u~íos", "ú~anse";
    infinitive: "u~irse";
    gerund: "u~iéndose";
    participle_m: "u~ido", _;
    ),
];
