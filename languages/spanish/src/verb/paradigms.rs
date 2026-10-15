//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ar`, `echar`: the stem and `ar`). A stem in two pieces (`~a~arse`, `incautarse`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~arˇ`: `sonar`).

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem rest_: \"~- like echar\"`: the slot's cells are `echar`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
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
     $(stem rest_: $s0:tt;)?
     $(stem lemma: $s1:tt;)?
     $(stem rest: $s2:tt;)?
     $(stem rest__: $s3:tt;)?
     $(stem rest___: $s4:tt;)?
     $(stem gerund: $s5:tt;)?
     $(stem participle: $s6:tt;)?
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
                $(flexion::cell!($s6),)? "\t",
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

/// What a conjugation adds after each pattern of the one it is like (`with "", "se" after "enr"`: ``, or `se`
/// where that pattern ends in `e`, `n`, `r`).
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
    /// Like `echar`: 9,689 words.
    Echar,
    /// Like `realizar`: 1,200 words.
    Realizar,
    /// Like `incautarse`: 724 words.
    Incautarse,
    /// Like `acomodarse`: 591 words.
    Acomodarse,
    /// Like `quejarse`: 564 words.
    Quejarse,
    /// Like `rilarse`: 548 words.
    Rilarse,
    /// Like `atipujarse`: 398 words.
    Atipujarse,
    /// Like `regodearse`: 358 words.
    Regodearse,
    /// Like `conocer`: 330 words.
    Conocer,
    /// Like `curtir`: 328 words.
    Curtir,
    /// Like `sacar`: 249 words.
    Sacar,
    /// Like `fosilizarse`: 249 words.
    Fosilizarse,
    /// Like `autoabastecerse`: 176 words.
    Autoabastecerse,
    /// Like `meter`: 145 words.
    Meter,
    /// Like `osificarse`: 134 words.
    Osificarse,
    /// Like `recalentar`: 132 words.
    Recalentar,
    /// Like `agacharse`: 115 words.
    Agacharse,
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
    /// Like `volar`: 40 words.
    Volar,
    /// Like `resfriarse`: 40 words.
    Resfriarse,
    /// Like `suponer`: 37 words.
    Suponer,
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
    /// Like `trasegar`: 25 words.
    Trasegar,
    /// Like `convenir`: 24 words.
    Convenir,
    /// Like `reducir`: 24 words.
    Reducir,
    /// Like `escribir`: 23 words.
    Escribir,
    /// Like `sacarse`: 23 words.
    Sacarse,
    /// Like `meliar`: 23 words.
    Meliar,
    /// Like `recoger`: 22 words.
    Recoger,
    /// Like `descomponerse`: 22 words.
    Descomponerse,
    /// Like `ahijar`: 22 words.
    Ahijar,
    /// Like `enderezarse`: 22 words.
    Enderezarse,
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
    /// Like `mantener`: 13 words.
    Mantener,
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
    /// Like `prever`: 7 words.
    Prever,
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
    /// Like `desleírse`: 5 words.
    Desleírse,
    /// Like `dormir`: 5 words.
    Dormir,
    /// Like `salir`: 5 words.
    Salir,
    /// Like `interdecir`: 5 words.
    Interdecir,
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
    /// Like `reverse`: 4 words.
    Reverse,
    /// Like `criarse`: 4 words.
    Criarse,
    /// Like `punzarse`: 4 words.
    Punzarse,
    /// Like `traslucirse`: 4 words.
    Traslucirse,
    /// Like `torcerse`: 4 words.
    Torcerse,
    /// Like `henchirse`: 4 words.
    Henchirse,
    /// Like `comenzar`: 4 words.
    Comenzar,
    /// Like `vencerse`: 4 words.
    Vencerse,
    /// Like `adyacer`: 3 words.
    Adyacer,
    /// Like `argüir`: 3 words.
    Argüir,
    /// Like `jugar`: 3 words.
    Jugar,
    /// Like `romper`: 3 words.
    Romper,
    /// Like `valer`: 3 words.
    Valer,
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
        Conjugation::Atipujarse,
        Conjugation::Regodearse,
        Conjugation::Conocer,
        Conjugation::Curtir,
        Conjugation::Sacar,
        Conjugation::Fosilizarse,
        Conjugation::Autoabastecerse,
        Conjugation::Meter,
        Conjugation::Osificarse,
        Conjugation::Recalentar,
        Conjugation::Agacharse,
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
        Conjugation::Suponer,
        Conjugation::Fraguar,
        Conjugation::Rendir,
        Conjugation::Descocarse,
        Conjugation::Desvivirse,
        Conjugation::Situarse,
        Conjugation::Embazarse,
        Conjugation::Destoserse,
        Conjugation::Trasegar,
        Conjugation::Convenir,
        Conjugation::Reducir,
        Conjugation::Escribir,
        Conjugation::Sacarse,
        Conjugation::Meliar,
        Conjugation::Recoger,
        Conjugation::Descomponerse,
        Conjugation::Ahijar,
        Conjugation::Enderezarse,
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
        Conjugation::Mantener,
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
        Conjugation::Prever,
        Conjugation::Tañer,
        Conjugation::Ahumarse,
        Conjugation::Venirse,
        Conjugation::Aterirse,
        Conjugation::Jetiarse,
        Conjugation::Deshacerse,
        Conjugation::Relamerse,
        Conjugation::Mover,
        Conjugation::Seguir,
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
        Conjugation::Desleírse,
        Conjugation::Dormir,
        Conjugation::Salir,
        Conjugation::Interdecir,
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
        Conjugation::Reverse,
        Conjugation::Criarse,
        Conjugation::Punzarse,
        Conjugation::Traslucirse,
        Conjugation::Torcerse,
        Conjugation::Henchirse,
        Conjugation::Comenzar,
        Conjugation::Vencerse,
        Conjugation::Adyacer,
        Conjugation::Argüir,
        Conjugation::Jugar,
        Conjugation::Romper,
        Conjugation::Valer,
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
    // Echar: 9,689 words (estar, amar, generar, crear, dar, abandonar).
    conjugation!("echar", 9689, "~ar";
    stem rest_: "~- like echar";
    stem lemma: "~- like echar";
    stem rest: "~- like echar";
    stem rest__: "~ar- like echar";
    stem rest___: "~- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~ando";
    participle_m: "~ado", _;
    ),
    // Realizar: 1,200 words (profundizar, utilizar, realzar, avergonzar, lanzar, alcanzar). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("realizar", 1200, "~zar", like Echar;
    stem rest_: "~- like realizar";
    stem lemma: "~z- like echar";
    stem rest: "~- like realizar";
    stem rest__: "~zar- like echar";
    stem rest___: "~c- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~zando";
    participle_m: "~zado", _;
    ),
    // Incautarse: 724 words (llamarse, jambarse, amoratarse, infartarse, autoengañarse, ahorcajarse).
    conjugation!("incautarse", 724, "~a~arse";
    stem lemma: "~a~- like incautarse";
    stem rest: "~a~- like echar";
    stem rest__: "~a~ar- like echar";
    stem rest___: "~a~- like incautarse";
    pres_1: "~a~o", _;
    pres_2: "~a~as", _;
    impf_3: derived, _;
    subj_1: "~a~e", _;
    subj_2: "~a~es", _;
    subj_3: _, "~a~en";
    imperative: "~á~ate", "~á~ese", _, _, "~á~ense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~a~ándose";
    participle_m: "~a~ado", _;
    ),
    // Acomodarse: 591 words (acodarse, acomodarse, endrogarse, personarse, autocontrolarse, amonarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("acomodarse", 591, "~o~arse", like Incautarse;
    stem lemma: "~o~- like incautarse";
    stem rest: "~o~- like echar";
    stem rest__: "~o~ar- like echar";
    stem rest___: "~o~- like incautarse";
    pres_1: "~o~o", _;
    pres_2: "~o~as", _;
    impf_3: derived, _;
    subj_1: "~o~e", _;
    subj_2: "~o~es", _;
    subj_3: _, "~o~en";
    imperative: "~ó~ate", "~ó~ese", _, _, "~ó~ense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~o~ándose";
    participle_m: "~o~ado", _;
    ),
    // Quejarse: 564 words (quejarse, querellarse, despertarse, pegarse, autocremarse, afiebrarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("quejarse", 564, "~e~arse", like Incautarse;
    stem lemma: "~e~- like incautarse";
    stem rest: "~e~- like echar";
    stem rest__: "~e~ar- like echar";
    stem rest___: "~e~- like incautarse";
    pres_1: "~e~o", _;
    pres_2: "~e~as", _;
    impf_3: derived, _;
    subj_1: "~e~e", _;
    subj_2: "~e~es", _;
    subj_3: _, "~e~en";
    imperative: "~é~ate", "~é~ese", _, _, "~é~ense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~e~ándose";
    participle_m: "~e~ado", _;
    ),
    // Rilarse: 548 words (fijarse, chisparse, ababillarse, agüitarse, refanfinflarse, encapricharse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("rilarse", 548, "~i~arse", like Incautarse;
    stem lemma: "~i~- like incautarse";
    stem rest: "~i~- like echar";
    stem rest__: "~i~ar- like echar";
    stem rest___: "~i~- like incautarse";
    pres_1: "~i~o", _;
    pres_2: "~i~as", _;
    impf_3: derived, _;
    subj_1: "~i~e", _;
    subj_2: "~i~es", _;
    subj_3: _, "~i~en";
    imperative: "~í~ate", "~í~ese", _, _, "~í~ense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~i~ándose";
    participle_m: "~i~ado", _;
    ),
    // Atipujarse: 398 words (acalenturarse, emputarse, confabularse, autoinculparse, enfurruñarse, despreocuparse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("atipujarse", 398, "~u~arse", like Incautarse;
    stem lemma: "~u~- like incautarse";
    stem rest: "~u~- like echar";
    stem rest__: "~u~ar- like echar";
    stem rest___: "~u~- like incautarse";
    pres_1: "~u~o", _;
    pres_2: "~u~as", _;
    impf_3: derived, _;
    subj_1: "~u~e", _;
    subj_2: "~u~es", _;
    subj_3: _, "~u~en";
    imperative: "~ú~ate", "~ú~ese", _, _, "~ú~ense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~u~ándose";
    participle_m: "~u~ado", _;
    ),
    // Regodearse: 358 words (carcajearse, petatearse, autobombearse, regodearse, mamonearse, cachiporrearse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("regodearse", 358, "~~arse", like Incautarse;
    stem lemma: "~~- like incautarse";
    stem rest: "~~- like echar";
    stem rest__: "~~ar- like echar";
    stem rest___: "~~- like incautarse";
    pres_1: "~~o", _;
    pres_2: "~~as", _;
    impf_3: derived, _;
    subj_1: "~~e", _;
    subj_2: "~~es", _;
    subj_3: _, "~~en";
    imperative: "~éat~", "~é~se", _, _, "~é~nse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~~ándose";
    participle_m: "~~ado", _;
    ),
    // Conocer: 330 words (conocer, tender, placer, perder, encender, nacer). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("conocer", 330, "~~er", like Incautarse;
    stem lemma: "~~- like conocer";
    stem rest: "~~- like conocer";
    stem rest__: "~~er- like echar";
    stem rest___: "~z~- like conocer";
    pres_1: "~z~o", _;
    pres_2: "~~es", _;
    impf_3: derived, _;
    subj_1: "~z~a", _;
    subj_2: "~z~as", _;
    subj_3: _, "~z~an";
    imperative: "~~e", "~z~a", _, _, "~z~an";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~~iendo";
    participle_m: "~~ido", _;
    ),
    // Curtir: 328 words (partir, polir, abolir, cumplir, finir, vivir). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("curtir", 328, "~ir", like Echar;
    stem rest_: "~- like curtir";
    stem lemma: "~- like curtir";
    stem rest: "~- like conocer";
    stem rest__: "~ir- like echar";
    stem rest___: "~- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~iendo";
    participle_m: "~ido", _;
    ),
    // Sacar: 249 words (buscar, sacar, rascar, pecar, tocar, atacar). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("sacar", 249, "~car", like Echar;
    stem rest_: "~- like sacar";
    stem lemma: "~c- like echar";
    stem rest: "~- like sacar";
    stem rest__: "~car- like echar";
    stem rest___: "~qu- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~cando";
    participle_m: "~cado", _;
    ),
    // Fosilizarse: 249 words (fosilizarse, autorrealizarse, autobautizarse, autoanalizarse, autopolinizarse, autoorganizarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("fosilizarse", 249, "~izarse", like Echar;
    stem lemma: "~iz- like incautarse";
    stem rest: "~i- like realizar";
    stem rest__: "~izar- like echar";
    stem rest___: "~ic- like incautarse";
    pres_1: "~izo", _;
    pres_2: "~izas", _;
    impf_3: derived, _;
    subj_1: "~ice", _;
    subj_2: "~ices", _;
    subj_3: _, "~icen";
    imperative: "~ízate", "~ícese", _, _, "~ícense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~izándose";
    participle_m: "~izado", _;
    ),
    // Autoabastecerse: 176 words (autoabastecerse, entrometerse, entremeterse, autoprotegerse, entigrecerse, ensilvecerse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("autoabastecerse", 176, "~e~erse", like Incautarse;
    stem lemma: "~e~- like autoabastecerse";
    stem rest: "~e~- like conocer";
    stem rest__: "~e~er- like echar";
    stem rest___: "~ez~- like autoabastecerse";
    pres_1: "~ez~o", _;
    pres_2: "~e~es", _;
    impf_3: derived, _;
    subj_1: "~ez~a", _;
    subj_2: "~ez~as", _;
    subj_3: _, "~ez~an";
    imperative: "~é~ete", "~éz~ase", _, _, "~éz~anse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~e~iéndose";
    participle_m: "~e~ido", _;
    ),
    // Meter: 145 words (absconder, absorber, her, meter, acceder, beber). Like Curtir, except in 1 cell: impf_3 sg.
    conjugation!("meter", 145, "~er", like Curtir;
    stem lemma: "~- like conocer";
    stem rest__: "~er- like echar";
    impf_3: derived, _;
    ),
    // Osificarse: 134 words (gelificarse, automedicarse, autocalificarse, autoaplicarse, osificarse, autoidentificarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("osificarse", 134, "~icarse", like Echar;
    stem lemma: "~ic- like incautarse";
    stem rest: "~i- like sacar";
    stem rest__: "~icar- like echar";
    stem rest___: "~iqu- like incautarse";
    pres_1: "~ico", _;
    pres_2: "~icas", _;
    impf_3: derived, _;
    subj_1: "~ique", _;
    subj_2: "~iques", _;
    subj_3: _, "~iquen";
    imperative: "~ícate", "~íquese", _, _, "~íquense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~icándose";
    participle_m: "~icado", _;
    ),
    // Recalentar: 132 words (pensar, quebrar, helar, gobernar, confesar, atestar). Like Regodearse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("recalentar", 132, "~~ar", like Regodearse;
    stem rest_: "~i~- like echar";
    stem lemma: "~~- like echar";
    stem rest___: "~~- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~~ando";
    ),
    // Agacharse: 115 words (acovacharse, abicharse, abrocharse, arreguincharse, reguincharse, emborracharse). Like Echar with `` after each pattern (`se` after e, n, r), except in 10 cells: pret_3 pl, subj_1 sg, subj_3 pl, subj_impf_se_3 sg, subj_fut_3 sg, vos imp, ….
    conjugation!("agacharse", 115, "~arse", like Echar with "", "se" after "enr";
    pret_3: _, "~aron";
    subj_1: "~e", _;
    subj_3: _, "~en";
    subj_impf_se_3: "~ase", _;
    subj_fut_3: "~are", _;
    vos: _, _, "~áte";
    imperative: "~ate", _, "~émonos", "~aos", _;
    gerund: "~ándose";
    ),
    // Desviar: 109 words (enviar, enfriar, ampliar, aviar, confiar, agriar). Like Curtir, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("desviar", 109, "~iar", like Curtir;
    stem rest_: "~í- like echar";
    stem lemma: "~i- like echar";
    stem rest: "~i- like echar";
    stem rest__: "~iar- like echar";
    stem rest___: "~i- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~iando";
    participle_m: "~iado", _;
    ),
    // Instruir: 105 words (construir, destruir, bullir, incluir, excluir, sustituir). Like Curtir, except in 7 cells: subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, negative_imperative ustedes, ….
    conjugation!("instruir", 105, "~ir", like Curtir;
    stem rest_: "~y- like curtir";
    stem rest: "~- like instruir";
    stem rest___: "~y- like conocer";
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
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
    // Sentir: 56 words (mentir, sentir, zaherir, herir, advertir, sugerir). Like Autoabastecerse, except in 14 cells: impf_3 sg, pret_3 sg, pret_3 pl, subj_3 sg, subj_impf_1 pl, subj_impf_3 sg, ….
    conjugation!("sentir", 56, "~e~ir", like Autoabastecerse;
    stem rest_: "~ie~- like curtir";
    stem lemma: "~e~- like curtir";
    stem rest__: "~e~ir- like echar";
    stem rest___: "~i~- like conocer";
    impf_3: derived, _;
    pret_3: "~i~ió", "~i~ieron";
    subj_3: derived, _;
    subj_impf_1: _, "~i~iéramos";
    subj_impf_3: "~i~iera", _;
    subj_impf_se_3: "~i~iese", _;
    subj_fut_3: "~i~iere", _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~i~iendo";
    ),
    // Situar: 56 words (puntuar, continuar, efectuar, actuar, situar, menstruar). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("situar", 56, "~uar", like Echar;
    stem rest_: "~ú- like echar";
    stem lemma: "~u- like echar";
    stem rest: "~u- like echar";
    stem rest__: "~uar- like echar";
    stem rest___: "~u- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
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
    // Musirse: 54 words (desnutrirse, musirse, entumirse, ocurrirse, consumirse, eludirse). Like Atipujarse, except in 12 cells: pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("musirse", 54, "~u~irse", like Atipujarse;
    stem lemma: "~u~- like musirse";
    stem rest: "~u~- like conocer";
    stem rest__: "~u~ir- like echar";
    stem rest___: "~u~- like autoabastecerse";
    pres_2: "~u~es", _;
    impf_3: derived, _;
    subj_1: "~u~a", _;
    subj_2: "~u~as", _;
    subj_3: _, "~u~an";
    imperative: "~ú~ete", "~ú~ase", _, _, "~ú~anse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~u~iéndose";
    participle_m: "~u~ido", _;
    ),
    // Dirigir: 46 words (urgir, mugir, exigir, fingir, agir, dirigir). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("dirigir", 46, "~gir", like Echar;
    stem rest_: "~- like dirigir";
    stem lemma: "~g- like curtir";
    stem rest: "~g- like conocer";
    stem rest__: "~gir- like echar";
    stem rest___: "~j- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~giendo";
    participle_m: "~gido", _;
    ),
    // Volar: 40 words (soltar, encontrar, demostrar, soñar, mostrar, colgar). Like Desencontrarse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("volar", 40, "~o~ar", like Desencontrarse;
    stem rest_: "~ue~- like echar";
    stem lemma: "~o~- like echar";
    stem rest___: "~o~- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
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
    // Suponer: 37 words (poner, yuxtaponer, imponer, suponer, exponer, posponer). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("suponer", 37, "~oner", like Echar;
    stem lemma: "~on- like conocer";
    stem rest: "~us- like poner";
    stem rest__: "~ondr- like echar";
    stem rest___: "~ong- like conocer";
    pres_1: "~ongo", _;
    pres_2: "~ones", _;
    impf_3: derived, _;
    subj_1: "~onga", _;
    subj_2: "~ongas", _;
    subj_3: _, "~ongan";
    imperative: "~ón", "~onga", _, _, "~ongan";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~oniendo";
    participle_m: "~uesto", _;
    ),
    // Fraguar: 32 words (averiguar, atestiguar, menguar, aguar, testiguar, apaciguar). Like Situar, except in 10 cells: pres_1 sg, pres_2 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("fraguar", 32, "~uar", like Situar;
    stem rest: "~- like fraguar";
    stem rest___: "~ü- like echar";
    pres_1: "~uo", _;
    pres_2: "~uas", _;
    subj_1: "~üe", _;
    subj_2: "~ües", _;
    subj_3: _, "~üen";
    imperative: "~ua", "~üe", _, _, "~üen";
    negative_imperative: _, _, derived, derived, _;
    ),
    // Rendir: 32 words (pedir, servir, despedir, vestir, investir, medir). Like Sentir, except in 4 cells: subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative ustedes.
    conjugation!("rendir", 32, "~e~ir", like Sentir;
    stem rest_: "~i~- like curtir";
    subj_3: derived, _;
    negative_imperative: derived, derived, _, _, derived;
    ),
    // Descocarse: 31 words (autoconvocarse, descocarse, abocarse, apocarse, equivocarse, enfocarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("descocarse", 31, "~ocarse", like Echar;
    stem lemma: "~oc- like incautarse";
    stem rest: "~o- like sacar";
    stem rest__: "~ocar- like echar";
    stem rest___: "~oqu- like incautarse";
    pres_1: "~oco", _;
    pres_2: "~ocas", _;
    impf_3: derived, _;
    subj_1: "~oque", _;
    subj_2: "~oques", _;
    subj_3: _, "~oquen";
    imperative: "~ócate", "~óquese", _, _, "~óquense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~ocándose";
    participle_m: "~ocado", _;
    ),
    // Desvivirse: 30 words (desvivirse, autodefinirse, autopercibirse, inhibirse, extinguirse, distinguirse). Like Rendir, except in 8 cells: impf_3 sg, imperative tu, imperative usted, imperative ustedes, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("desvivirse", 30, "~i~irse", like Rendir;
    stem lemma: "~i~- like musirse";
    stem rest: "~i~- like conocer";
    stem rest__: "~i~ir- like echar";
    stem rest___: "~i~- like autoabastecerse";
    impf_3: derived, _;
    imperative: "~í~ete", "~í~ase", _, _, "~í~anse";
    negative_imperative: _, _, derived, derived, _;
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
    // Embazarse: 27 words (embazarse, abrazarse, acarrazarse, amostazarse, desplazarse, tarazarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("embazarse", 27, "~azarse", like Echar;
    stem lemma: "~az- like incautarse";
    stem rest: "~a- like realizar";
    stem rest__: "~azar- like echar";
    stem rest___: "~ac- like incautarse";
    pres_1: "~azo", _;
    pres_2: "~azas", _;
    impf_3: derived, _;
    subj_1: "~ace", _;
    subj_2: "~aces", _;
    subj_3: _, "~acen";
    imperative: "~ázate", "~ácese", _, _, "~ácense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~azándose";
    participle_m: "~azado", _;
    ),
    // Destoserse: 27 words (destoserse, correrse, coserse, corresponderse, dolerse, carcomerse). Like Acomodarse, except in 12 cells: pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("destoserse", 27, "~o~erse", like Acomodarse;
    stem lemma: "~o~- like autoabastecerse";
    stem rest: "~o~- like conocer";
    stem rest__: "~o~er- like echar";
    stem rest___: "~o~- like autoabastecerse";
    pres_2: "~o~es", _;
    impf_3: derived, _;
    subj_1: "~o~a", _;
    subj_2: "~o~as", _;
    subj_3: _, "~o~an";
    imperative: "~ó~ete", "~ó~ase", _, _, "~ó~anse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~o~iéndose";
    participle_m: "~o~ido", _;
    ),
    // Trasegar: 25 words (negar, renegar, fregar, regar, segar, sosegar). Like Echar with `ˇ` after each pattern, except in 10 cells: pret_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("trasegar", 25, "~arˇ", like Echar with "ˇ";
    pret_1: "~uéˇ", _;
    subj_1: "~ueˇ", "~uemosˇ";
    subj_2: "~uesˇ", "~uéisˇ";
    subj_3: _, "~uenˇ";
    vos: _, "~uésˇ", _;
    imperative: _, "~ueˇ", "~uemosˇ", _, "~uenˇ";
    ),
    // Convenir: 24 words (devenir, venir, intervenir, avenir, revenir, convenir). Like Sentir, except in 9 cells: pres_1 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, imperative usted, ….
    conjugation!("convenir", 24, "~e~ir", like Sentir;
    stem rest: "~i~- like poner";
    stem rest__: "~e~dr- like echar";
    stem rest___: "~e~g- like conocer";
    pres_1: "~e~go", _;
    subj_1: "~e~ga", _;
    subj_2: "~e~gas", _;
    subj_3: _, "~e~gan";
    imperative: "~é~", "~e~ga", _, _, "~e~gan";
    negative_imperative: _, _, derived, derived, _;
    ),
    // Reducir: 24 words (traducir, producir, reducir, conducir, introducir, inducir). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("reducir", 24, "~cir", like Echar;
    stem lemma: "~c- like curtir";
    stem rest: "~j- like reducir";
    stem rest__: "~cir- like echar";
    stem rest___: "~zc- like conocer";
    pres_1: "~zco", _;
    pres_2: "~ces", _;
    impf_3: derived, _;
    subj_1: "~zca", _;
    subj_2: "~zcas", _;
    subj_3: _, "~zcan";
    imperative: "~ce", "~zca", _, _, "~zcan";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~ciendo";
    participle_m: "~cido", _;
    ),
    // Escribir: 23 words (escribir, describir, inscribir, transcribir, prescribir, proscribir). Like Echar, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("escribir", 23, "~bir", like Echar;
    stem rest_: "~b- like curtir";
    stem lemma: "~b- like curtir";
    stem rest: "~b- like conocer";
    stem rest__: "~bir- like echar";
    stem rest___: "~b- like conocer";
    stem participle: "~- like escribir";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~biendo";
    ),
    // Sacarse: 23 words (abarracarse, abellacarse, desajacarse, trelacarse, atracarse, enflacarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("sacarse", 23, "~acarse", like Echar;
    stem lemma: "~ac- like incautarse";
    stem rest: "~a- like sacar";
    stem rest__: "~acar- like echar";
    stem rest___: "~aqu- like incautarse";
    pres_1: "~aco", _;
    pres_2: "~acas", _;
    impf_3: derived, _;
    subj_1: "~aque", _;
    subj_2: "~aques", _;
    subj_3: _, "~aquen";
    imperative: "~ácate", "~áquese", _, _, "~áquense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~acándose";
    participle_m: "~acado", _;
    ),
    // Meliar: 23 words (gilidiar, chalequiar, meliar, cangrejiar, jariniar, jonjoliar). Like Desviar, except in 4 cells: subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative ustedes.
    conjugation!("meliar", 23, "~iar", like Desviar;
    stem rest_: "~e- like echar";
    subj_3: derived, _;
    negative_imperative: derived, derived, _, _, derived;
    ),
    // Recoger: 22 words (proteger, coger, recoger, escoger, acoger, emerger). Like Dirigir, except in 1 cell: impf_3 sg.
    conjugation!("recoger", 22, "~ger", like Dirigir;
    stem lemma: "~g- like conocer";
    stem rest__: "~ger- like echar";
    impf_3: derived, _;
    ),
    // Descomponerse: 22 words (autoimponerse, ponerse, imponerse, disponerse, exponerse, reponerse). Like Suponer with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("descomponerse", 22, "~onerse", like Suponer with "", "se" after "r";
    vos: _, _, "~onete";
    imperative: "~onte", "~óngase", "~ongámonos", "~oneos", "~ónganse";
    gerund: "~oniéndose";
    ),
    // Ahijar: 22 words (prohijar, descafeinar, aislar, ahincar, amohinar, cabrahigar). Like Rilarse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("ahijar", 22, "~i~ar", like Rilarse;
    stem rest_: "~í~- like echar";
    stem lemma: "~i~- like echar";
    stem rest___: "~i~- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~i~ando";
    ),
    // Enderezarse: 22 words (emperezarse, esperezarse, alzarse, vezarse, avezarse, avergonzarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("enderezarse", 22, "~~zarse", like Incautarse;
    stem lemma: "~~z- like incautarse";
    stem rest: "~~- like realizar";
    stem rest__: "~~zar- like echar";
    stem rest___: "~~c- like incautarse";
    pres_1: "~~zo", _;
    pres_2: "~~zas", _;
    impf_3: derived, _;
    subj_1: "~~ce", _;
    subj_2: "~~ces", _;
    subj_3: _, "~~cen";
    imperative: "~ézat~", "~éc~se", _, _, "~éc~nse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~~zándose";
    participle_m: "~~zado", _;
    ),
    // Entremeterse: 21 words (prenderse, comprenderse, tejerse, sorprenderse, emperescerse, venderse). Like Autoabastecerse, except in 8 cells: pres_1 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative usted, imperative ustedes, ….
    conjugation!("entremeterse", 21, "~e~erse", like Autoabastecerse;
    stem rest___: "~e~- like autoabastecerse";
    pres_1: "~e~o", _;
    subj_1: "~e~a", _;
    subj_2: "~e~as", _;
    subj_3: _, "~e~an";
    imperative: _, "~é~ase", _, _, "~é~anse";
    negative_imperative: _, _, derived, derived, _;
    ),
    // Inmiscuirse: 21 words (autodestruirse, inmiscuirse, autoexcluirse, autoconstruirse, rehuirse, atribuirse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("inmiscuirse", 21, "~uirse", like Echar;
    stem lemma: "~u- like musirse";
    stem rest: "~u- like instruir";
    stem rest__: "~uir- like echar";
    stem rest___: "~uy- like autoabastecerse";
    pres_1: "~uyo", _;
    pres_2: "~uyes", _;
    impf_3: derived, _;
    subj_1: "~uya", _;
    subj_2: "~uyas", _;
    subj_3: _, "~uyan";
    imperative: "~úyete", "~úyase", _, _, "~úyanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~uyéndose";
    participle_m: "~uido", _;
    ),
    // Aunar: 20 words (aunar, rehusar, ahumar, maullar, aupar, aullar). Like Atipujarse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("aunar", 20, "~u~ar", like Atipujarse;
    stem rest_: "~ú~- like echar";
    stem lemma: "~u~- like echar";
    stem rest___: "~u~- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~u~ando";
    ),
    // Abatirse: 20 words (batirse, partirse, persuadirse, abatirse, debatirse, expandirse). Like Incautarse, except in 12 cells: pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("abatirse", 20, "~a~irse", like Incautarse;
    stem lemma: "~a~- like musirse";
    stem rest: "~a~- like conocer";
    stem rest__: "~a~ir- like echar";
    stem rest___: "~a~- like autoabastecerse";
    pres_2: "~a~es", _;
    impf_3: derived, _;
    subj_1: "~a~a", _;
    subj_2: "~a~as", _;
    subj_3: _, "~a~an";
    imperative: "~á~ete", "~á~ase", _, _, "~á~anse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~a~iéndose";
    participle_m: "~a~ido", _;
    ),
    // Chuzarse: 19 words (chapuzarse, abruzarse, abuzarse, escagarruzarse, aliruzarse, atapuzarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("chuzarse", 19, "~uzarse", like Echar;
    stem lemma: "~uz- like incautarse";
    stem rest: "~u- like realizar";
    stem rest__: "~uzar- like echar";
    stem rest___: "~uc- like incautarse";
    pres_1: "~uzo", _;
    pres_2: "~uzas", _;
    impf_3: derived, _;
    subj_1: "~uce", _;
    subj_2: "~uces", _;
    subj_3: _, "~ucen";
    imperative: "~úzate", "~úcese", _, _, "~úcense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~uzándose";
    participle_m: "~uzado", _;
    ),
    // Resentirse: 19 words (resentirse, arrepentirse, repentirse, divertirse, sentirse, referirse). Like Sentir with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("resentirse", 19, "~e~irse", like Sentir with "", "se" after "r";
    vos: _, _, "~e~ite";
    imperative: "~ié~ete", "~ié~ase", "~i~ámonos", "~e~íos", "~ié~anse";
    gerund: "~i~iéndose";
    ),
    // Fruncir: 18 words (zurcir, uncir, surcir, esparcir, resarcir, fruncir). Like Reducir, except in 6 cells: subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, negative_imperative ustedes.
    conjugation!("fruncir", 18, "~cir", like Reducir;
    stem rest_: "~- like fruncir";
    stem rest: "~c- like conocer";
    stem rest___: "~z- like conocer";
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    ),
    // Calzarse: 18 words (abalanzarse, ensalzarse, esperanzarse, alianzarse, alcanzarse, desalzarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("calzarse", 18, "~a~zarse", like Incautarse;
    stem lemma: "~a~z- like incautarse";
    stem rest: "~a~- like realizar";
    stem rest__: "~a~zar- like echar";
    stem rest___: "~a~c- like incautarse";
    pres_1: "~a~zo", _;
    pres_2: "~a~zas", _;
    impf_3: derived, _;
    subj_1: "~a~ce", _;
    subj_2: "~a~ces", _;
    subj_3: _, "~a~cen";
    imperative: "~á~zate", "~á~cese", _, _, "~á~cense";
    negative_imperative: _, _, derived, derived, _;
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
    stem rest: "~- like creer";
    gerund: "~yendo";
    participle_m: "~ído", _;
    ),
    // Traer: 16 words (traer, contraer, atraer, extraer, substraer, abstraer). Like Creer, except in 8 cells: pres_1 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative usted, imperative ustedes, ….
    conjugation!("traer", 16, "~er", like Creer;
    stem rest: "~j- like reducir";
    stem rest___: "~ig- like conocer";
    pres_1: "~igo", _;
    subj_1: "~iga", _;
    subj_2: "~igas", _;
    subj_3: _, "~igan";
    imperative: _, "~iga", _, _, "~igan";
    negative_imperative: _, _, derived, derived, _;
    ),
    // Entecarse: 16 words (entecarse, enchuecarse, obcecarse, intersecarse, ahuecarse, deprecarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("entecarse", 16, "~~carse", like Incautarse;
    stem lemma: "~~c- like incautarse";
    stem rest: "~~- like sacar";
    stem rest__: "~~car- like echar";
    stem rest___: "~~qu- like incautarse";
    pres_1: "~~co", _;
    pres_2: "~~cas", _;
    impf_3: derived, _;
    subj_1: "~~que", _;
    subj_2: "~~ques", _;
    subj_3: _, "~~quen";
    imperative: "~écat~", "~équ~se", _, _, "~équ~nse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~~cándose";
    participle_m: "~~cado", _;
    ),
    // Aborrascarse: 14 words (aborrascarse, atascarse, rascarse, achubascarse, enfrascarse, emborrascarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("aborrascarse", 14, "~a~carse", like Incautarse;
    stem lemma: "~a~c- like incautarse";
    stem rest: "~a~- like sacar";
    stem rest__: "~a~car- like echar";
    stem rest___: "~a~qu- like incautarse";
    pres_1: "~a~co", _;
    pres_2: "~a~cas", _;
    impf_3: derived, _;
    subj_1: "~a~que", _;
    subj_2: "~a~ques", _;
    subj_3: _, "~a~quen";
    imperative: "~á~cate", "~á~quese", _, _, "~á~quense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~a~cándose";
    participle_m: "~a~cado", _;
    ),
    // Revolver: 14 words (absolver, revolver, resolver, disolver, volver, solver). Like Incautarse, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("revolver", 14, "~o~ver", like Incautarse;
    stem rest_: "~ue~v- like curtir";
    stem lemma: "~o~v- like conocer";
    stem rest: "~o~v- like conocer";
    stem rest__: "~o~ver- like echar";
    stem rest___: "~o~v- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~o~viendo";
    participle_m: "~ue~to", _;
    ),
    // Rozarse: 14 words (apozarse, escozarse, arrebozarse, empozarse, alborozarse, desarrebozarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("rozarse", 14, "~ozarse", like Echar;
    stem lemma: "~oz- like incautarse";
    stem rest: "~o- like realizar";
    stem rest__: "~ozar- like echar";
    stem rest___: "~oc- like incautarse";
    pres_1: "~ozo", _;
    pres_2: "~ozas", _;
    impf_3: derived, _;
    subj_1: "~oce", _;
    subj_2: "~oces", _;
    subj_3: _, "~ocen";
    imperative: "~ózate", "~ócese", _, _, "~ócense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~ozándose";
    participle_m: "~ozado", _;
    ),
    // Vencer: 13 words (coercer, vencer, ejercer, convencer, mecer, mulcer). Like Fruncir, except in 1 cell: impf_3 sg.
    conjugation!("vencer", 13, "~cer", like Fruncir;
    stem lemma: "~c- like conocer";
    stem rest__: "~cer- like echar";
    impf_3: derived, _;
    ),
    // Mantener: 13 words (tener, sostener, obtener, detener, mantener, retener). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("mantener", 13, "~ener", like Echar;
    stem lemma: "~en- like conocer";
    stem rest: "~uv- like poner";
    stem rest__: "~endr- like echar";
    stem rest___: "~eng- like conocer";
    pres_1: "~engo", _;
    pres_2: "~ienes", _;
    impf_3: derived, _;
    subj_1: "~enga", _;
    subj_2: "~engas", _;
    subj_3: _, "~engan";
    imperative: "~én", "~enga", _, _, "~engan";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~eniendo";
    participle_m: "~enido", _;
    ),
    // Ceñir: 13 words (teñir, reñir, ceñir, desteñir, estreñir, astreñir). Like Echar, except in 17 cells: impf_3 sg, pret_1 sg, pret_1 pl, pret_3 sg, pret_3 pl, subj_3 sg, ….
    conjugation!("ceñir", 13, "~eñir", like Echar;
    stem rest_: "~iñ- like curtir";
    stem lemma: "~eñ- like curtir";
    stem rest__: "~eñir- like echar";
    stem rest___: "~iñ- like conocer";
    impf_3: derived, _;
    pret_1: "~eñí", "~eñimos";
    pret_3: "~iñó", "~iñeron";
    subj_3: derived, _;
    subj_impf_1: _, "~iñéramos";
    subj_impf_3: "~iñera", _;
    subj_impf_se_3: "~iñese", _;
    subj_fut_3: "~iñere", _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~iñendo";
    participle_m: "~eñido", _;
    ),
    // Enfoscarse: 12 words (enfoscarse, afoscarse, coscarse, desemboscarse, enfroscarse, trasroscarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("enfoscarse", 12, "~o~carse", like Incautarse;
    stem lemma: "~o~c- like incautarse";
    stem rest: "~o~- like sacar";
    stem rest__: "~o~car- like echar";
    stem rest___: "~o~qu- like incautarse";
    pres_1: "~o~co", _;
    pres_2: "~o~cas", _;
    impf_3: derived, _;
    subj_1: "~o~que", _;
    subj_2: "~o~ques", _;
    subj_3: _, "~o~quen";
    imperative: "~ó~cate", "~ó~quese", _, _, "~ó~quense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~o~cándose";
    participle_m: "~o~cado", _;
    ),
    // Enraizar: 12 words (enraizar, europeizar, arcaizar, descorreizar, judaizar, raizar). Like Fosilizarse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("enraizar", 12, "~izar", like Fosilizarse;
    stem rest_: "~í- like realizar";
    stem lemma: "~iz- like echar";
    stem rest___: "~ic- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
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
    // Hacer: 12 words (hacer, satisfacer, facer, deshacer, rehacer, contrahacer). Like Echar, except in 21 cells: pres_1 sg, pres_2 sg, impf_3 sg, pret_1 sg, pret_1 pl, pret_3 sg, ….
    conjugation!("hacer", 12, "~acer", like Echar;
    stem lemma: "~ac- like conocer";
    stem rest___: "~ag- like conocer";
    pres_1: "~ago", _;
    pres_2: "~aces", _;
    impf_3: derived, _;
    pret_1: "~ice", "~icimos";
    pret_3: "~izo", "~icieron";
    subj_1: "~aga", _;
    subj_2: "~agas", _;
    subj_3: _, "~agan";
    subj_impf_1: _, "~iciéramos";
    subj_impf_3: "~iciera", _;
    subj_impf_se_3: "~iciese", _;
    subj_fut_3: "~iciere", _;
    imperative: "~az", "~aga", _, _, "~agan";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~aciendo";
    participle_m: "~echo", _;
    ),
    // Moler: 12 words (llover, doler, poder, morder, soler, moler). Like Meter with `ˇ` after each pattern.
    conjugation!("moler", 12, "~erˇ", like Meter with "ˇ";
    ),
    // Escabullirse: 11 words (escabullirse, zambullirse, rebullirse, tullirse, escullirse, descabullirse). Like Musirse, except in 1 cell: gerund.
    conjugation!("escabullirse", 11, "~u~irse", like Musirse;
    stem rest: "~u~- like escabullirse";
    gerund: "~u~éndose";
    ),
    // Apanucarse: 11 words (acurrucarse, acorrucarse, embejucarse, apitucarse, emborucarse, desnucarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("apanucarse", 11, "~ucarse", like Echar;
    stem lemma: "~uc- like incautarse";
    stem rest: "~u- like sacar";
    stem rest__: "~ucar- like echar";
    stem rest___: "~uqu- like incautarse";
    pres_1: "~uco", _;
    pres_2: "~ucas", _;
    impf_3: derived, _;
    subj_1: "~uque", _;
    subj_2: "~uques", _;
    subj_3: _, "~uquen";
    imperative: "~úcate", "~úquese", _, _, "~úquense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~ucándose";
    participle_m: "~ucado", _;
    ),
    // Suscribirse: 10 words (autodescribirse, escribirse, adscribirse, inscribirse, subscribirse, suscribirse). Like Echar, except in 12 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("suscribirse", 10, "~ibirse", like Echar;
    stem lemma: "~ib- like musirse";
    stem rest: "~ib- like conocer";
    stem rest__: "~ibir- like echar";
    stem rest___: "~ib- like autoabastecerse";
    stem participle: "~i- like escribir";
    pres_1: "~ibo", _;
    pres_2: "~ibes", _;
    impf_3: derived, _;
    subj_1: "~iba", _;
    subj_2: "~ibas", _;
    subj_3: _, "~iban";
    imperative: "~íbete", "~íbase", _, _, "~íbanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~ibiéndose";
    ),
    // Retraerse: 10 words (contraerse, traerse, atraerse, sustraerse, retrotraerse, retraerse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("retraerse", 10, "~aerse", like Echar;
    stem lemma: "~a- like autoabastecerse";
    stem rest: "~aj- like reducir";
    stem rest__: "~aer- like echar";
    stem rest___: "~aig- like autoabastecerse";
    pres_1: "~aigo", _;
    pres_2: "~aes", _;
    impf_3: derived, _;
    subj_1: "~aiga", _;
    subj_2: "~aigas", _;
    subj_3: _, "~aigan";
    imperative: "~áete", "~áigase", _, _, "~áiganse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~ayéndose";
    participle_m: "~aído", _;
    ),
    // Lucir: 10 words (lucir, relucir, deslucir, enlucir, hendir, traslucir). Like Conocer, except in 1 cell: impf_3 sg.
    conjugation!("lucir", 10, "~~ir", like Conocer;
    stem lemma: "~~- like curtir";
    stem rest__: "~~ir- like echar";
    impf_3: derived, _;
    ),
    // Enamoriscarse: 9 words (enamoriscarse, ciscarse, ariscarse, enriscarse, apriscarse, arriscarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("enamoriscarse", 9, "~i~carse", like Incautarse;
    stem lemma: "~i~c- like incautarse";
    stem rest: "~i~- like sacar";
    stem rest__: "~i~car- like echar";
    stem rest___: "~i~qu- like incautarse";
    pres_1: "~i~co", _;
    pres_2: "~i~cas", _;
    impf_3: derived, _;
    subj_1: "~i~que", _;
    subj_2: "~i~ques", _;
    subj_3: _, "~i~quen";
    imperative: "~í~cate", "~í~quese", _, _, "~í~quense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~i~cándose";
    participle_m: "~i~cado", _;
    ),
    // Apuñuscarse: 9 words (apuñuscarse, buscarse, rebuscarse, zurruscarse, enfurruscarse, apeñuscarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("apuñuscarse", 9, "~u~carse", like Incautarse;
    stem lemma: "~u~c- like incautarse";
    stem rest: "~u~- like sacar";
    stem rest__: "~u~car- like echar";
    stem rest___: "~u~qu- like incautarse";
    pres_1: "~u~co", _;
    pres_2: "~u~cas", _;
    impf_3: derived, _;
    subj_1: "~u~que", _;
    subj_2: "~u~ques", _;
    subj_3: _, "~u~quen";
    imperative: "~ú~cate", "~ú~quese", _, _, "~ú~quense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~u~cándose";
    participle_m: "~u~cado", _;
    ),
    // Abrir: 9 words (cubrir, abrir, reabrir, descubrir, encubrir, entreabrir). Like Lucir, except in 7 cells: subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, negative_imperative ustedes, ….
    conjugation!("abrir", 9, "~~ir", like Lucir;
    stem rest_: "~~- like curtir";
    stem rest___: "~~- like conocer";
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    participle_m: "~ie~to", _;
    ),
    // Trocar: 9 words (trocar, trastrocar, clocar, retrocar, destrocar, aclocar). Like Descocarse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("trocar", 9, "~ocar", like Descocarse;
    stem rest_: "~ue- like sacar";
    stem lemma: "~oc- like echar";
    stem rest___: "~oqu- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~ocando";
    ),
    // Resolverse: 9 words (autodisolverse, resolverse, volverse, devolverse, revolverse, desenvolverse). Like Revolver with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("resolverse", 9, "~o~verse", like Revolver with "", "se" after "r";
    vos: _, _, "~o~vete";
    imperative: "~ué~vete", "~ué~vase", "~o~vámonos", "~o~veos", "~ué~vanse";
    gerund: "~o~viéndose";
    ),
    // Detenerse: 9 words (abstenerse, contenerse, sostenerse, mantenerse, detenerse, entretenerse). Like Mantener with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("detenerse", 9, "~enerse", like Mantener with "", "se" after "r";
    vos: _, _, "~enete";
    imperative: "~ente", "~éngase", "~engámonos", "~eneos", "~énganse";
    gerund: "~eniéndose";
    ),
    // Santiguarse: 9 words (achiguarse, apaciguarse, santiguarse, enmaniguarse, amortiguarse, averiguarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("santiguarse", 9, "~i~uarse", like Incautarse;
    stem lemma: "~i~u- like incautarse";
    stem rest: "~i~- like fraguar";
    stem rest__: "~i~uar- like echar";
    stem rest___: "~i~ü- like incautarse";
    pres_1: "~i~uo", _;
    pres_2: "~i~uas", _;
    impf_3: derived, _;
    subj_1: "~i~üe", _;
    subj_2: "~i~ües", _;
    subj_3: _, "~i~üen";
    imperative: "~í~uate", "~í~üese", _, _, "~í~üense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~i~uándose";
    participle_m: "~i~uado", _;
    ),
    // Refreír: 9 words (reír, freír, sonreír, engreír, desleír, esleír). Like Curtir, except in 10 cells: impf_3 sg, pret_1 sg, pret_1 pl, subj_3 sg, negative_imperative tu, negative_imperative usted, ….
    conjugation!("refreír", 9, "~eír", like Curtir;
    stem rest_: "~í- like curtir";
    stem lemma: "~eí- like refreír";
    stem rest__: "~eir- like echar";
    stem rest___: "~i- like conocer";
    impf_3: derived, _;
    pret_1: "~eí", "~eímos";
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    participle_m: "~eído", _;
    ),
    // Guiar: 8 words (liar, fiar, criar, guiar, ciar, piar). Like Desviar, except in 5 cells: impf_3 sg, pret_1 sg, pret_3 sg, negative_imperative nosotros, negative_imperative vosotros.
    conjugation!("guiar", 8, "~iar", like Desviar;
    stem lemma: "~i- like guiar";
    stem rest___: "~ie- like guiar";
    impf_3: derived, _;
    pret_1: "~ie", _;
    pret_3: "~io", _;
    negative_imperative: _, _, derived, derived, _;
    ),
    // Reducirse: 8 words (autorreproducirse, autoconducirse, autoproducirse, reducirse, reproducirse, introducirse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("reducirse", 8, "~ucirse", like Echar;
    stem lemma: "~uc- like musirse";
    stem rest: "~uj- like reducir";
    stem rest__: "~ucir- like echar";
    stem rest___: "~uzc- like autoabastecerse";
    pres_1: "~uzco", _;
    pres_2: "~uces", _;
    impf_3: derived, _;
    subj_1: "~uzca", _;
    subj_2: "~uzcas", _;
    subj_3: _, "~uzcan";
    imperative: "~úcete", "~úzcase", _, _, "~úzcanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~uciéndose";
    participle_m: "~ucido", _;
    ),
    // Desmedirse: 8 words (descomedirse, desmedirse, despedirse, acomedirse, medirse, expedirse). Like Echar, except in 21 cells: pres_1 sg, pres_2 sg, impf_3 sg, pret_1 sg, pret_1 pl, pret_3 sg, ….
    conjugation!("desmedirse", 8, "~edirse", like Echar;
    stem lemma: "~ed- like musirse";
    stem rest__: "~edir- like echar";
    stem rest___: "~id- like autoabastecerse";
    pres_1: "~ido", _;
    pres_2: "~ides", _;
    impf_3: derived, _;
    pret_1: "~edí", "~edimos";
    pret_3: "~idió", "~idieron";
    subj_1: "~ida", _;
    subj_2: "~idas", _;
    subj_3: _, "~idan";
    subj_impf_1: _, "~idiéramos";
    subj_impf_3: "~idiera", _;
    subj_impf_se_3: "~idiese", _;
    subj_fut_3: "~idiere", _;
    imperative: "~ídete", "~ídase", _, _, "~ídanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~idiéndose";
    participle_m: "~edido", _;
    ),
    // Creerse: 8 words (caerse, creerse, peerse, proveerse, corroerse, desposeerse). Like Abrir, except in 8 cells: impf_3 sg, imperative tu, imperative usted, imperative ustedes, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("creerse", 8, "~~erse", like Abrir;
    stem lemma: "~~- like autoabastecerse";
    stem rest: "~~- like creer";
    stem rest__: "~~er- like echar";
    stem rest___: "~~- like autoabastecerse";
    impf_3: derived, _;
    imperative: "~é~te", "~éas~", _, _, "~éans~";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~~yéndose";
    participle_m: "~~ído", _;
    ),
    // Empezar: 7 words (empezar, tropezar, despezar, entropezar, trompezar, desentropezar). Like Enderezarse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("empezar", 7, "~~zar", like Enderezarse;
    stem rest_: "~i~- like realizar";
    stem lemma: "~~z- like echar";
    stem rest___: "~~c- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~~zando";
    ),
    // Regir: 7 words (elegir, corregir, colegir, reelegir, regir, recolegir). Like Echar, except in 17 cells: impf_3 sg, pret_1 sg, pret_1 pl, pret_3 sg, pret_3 pl, subj_3 sg, ….
    conjugation!("regir", 7, "~egir", like Echar;
    stem rest_: "~i- like dirigir";
    stem lemma: "~eg- like curtir";
    stem rest__: "~egir- like echar";
    stem rest___: "~ij- like conocer";
    impf_3: derived, _;
    pret_1: "~egí", "~egimos";
    pret_3: "~igió", "~igieron";
    subj_3: derived, _;
    subj_impf_1: _, "~igiéramos";
    subj_impf_3: "~igiera", _;
    subj_impf_se_3: "~igiese", _;
    subj_fut_3: "~igiere", _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~igiendo";
    participle_m: "~egido", _;
    ),
    // Prever: 7 words (ver, ser, prever, entrever, antever, rever). Like Meter, except in 15 cells: pres_1 sg, pres_2 sg, impf_1 sg, impf_1 pl, subj_1 sg, subj_2 sg, ….
    conjugation!("prever", 7, "~er", like Meter;
    stem rest___: "~e- like conocer";
    pres_1: "~eo", _;
    pres_2: "~és", _;
    impf_1: "~eía", "~eíamos";
    subj_1: "~ea", _;
    subj_2: "~eas", _;
    subj_3: _, "~ean";
    vos: derived, _, derived;
    imperative: "~é", "~ea", _, _, "~ean";
    negative_imperative: _, _, derived, derived, _;
    participle_m: "~isto", _;
    ),
    // Tañer: 7 words (jier, atañer, tañer, empeller, toller, creyer). Like Meter, except in 1 cell: gerund.
    conjugation!("tañer", 7, "~er", like Meter;
    stem rest: "~- like escabullirse";
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
    // Venirse: 7 words (venirse, desconvenirse, avenirse, desavenirse, convenirse, prevenirse). Like Convenir with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("venirse", 7, "~e~irse", like Convenir with "", "se" after "r";
    vos: _, _, "~e~ite";
    imperative: "~e~te", "~é~gase", "~e~gámonos", "~e~íos", "~é~ganse";
    gerund: "~i~iéndose";
    ),
    // Aterirse: 7 words (aterirse, empedernirse, arrecirse, hespirse, denegrirse, sumergirse). Like Entremeterse, except in 1 cell: impf_3 sg.
    conjugation!("aterirse", 7, "~e~irse", like Entremeterse;
    stem lemma: "~e~- like musirse";
    stem rest__: "~e~ir- like echar";
    impf_3: derived, _;
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
    // Relamerse: 7 words (valerse, prevalerse, relamerse, lamerse, precaverse, barrerse). Like Abatirse, except in 1 cell: impf_3 sg.
    conjugation!("relamerse", 7, "~a~erse", like Abatirse;
    stem lemma: "~a~- like autoabastecerse";
    stem rest__: "~a~er- like echar";
    impf_3: derived, _;
    ),
    // Mover: 6 words (promover, mover, remover, conmover, amover, commover). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("mover", 6, "~over", like Echar;
    stem rest_: "~uev- like curtir";
    stem lemma: "~ov- like conocer";
    stem rest: "~ov- like conocer";
    stem rest__: "~over- like echar";
    stem rest___: "~ov- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~oviendo";
    participle_m: "~ovido", _;
    ),
    // Seguir: 6 words (proseguir, perseguir, seguir, conseguir, subseguir, reseguir). Like Traer, except in 13 cells: pres_2 sg, impf_3 sg, pret_1 sg, pret_1 pl, pret_3 sg, pret_3 pl, ….
    conjugation!("seguir", 6, "~eguir", like Traer;
    stem lemma: "~egu- like curtir";
    stem rest__: "~eguir- like echar";
    pres_2: "~igues", _;
    impf_3: derived, _;
    pret_1: "~eguí", "~eguimos";
    pret_3: "~iguió", "~iguieron";
    subj_impf_1: _, "~iguiéramos";
    subj_impf_3: "~iguiera", _;
    subj_impf_se_3: "~iguiese", _;
    subj_fut_3: "~iguiere", _;
    imperative: "~igue", _, _, _, _;
    gerund: "~iguiendo";
    participle_m: "~eguido", _;
    ),
    // Recogerse: 6 words (encogerse, desencogerse, sobrecogerse, recogerse, cogerse, acogerse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("recogerse", 6, "~ogerse", like Echar;
    stem lemma: "~og- like autoabastecerse";
    stem rest: "~og- like conocer";
    stem rest__: "~oger- like echar";
    stem rest___: "~oj- like autoabastecerse";
    pres_1: "~ojo", _;
    pres_2: "~oges", _;
    impf_3: derived, _;
    subj_1: "~oja", _;
    subj_2: "~ojas", _;
    subj_3: _, "~ojan";
    imperative: "~ógete", "~ójase", _, _, "~ójanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~ogiéndose";
    participle_m: "~ogido", _;
    ),
    // Oír: 6 words (oír, embaír, desvaír, desoír, trasoír, entreoír). Like Instruir, except in 10 cells: pres_1 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative usted, ….
    conjugation!("oír", 6, "~ír", like Instruir;
    stem lemma: "~í- like refreír";
    stem rest: "~- like creer";
    stem rest___: "~ig- like conocer";
    pres_1: "~igo", _;
    impf_3: derived, _;
    subj_1: "~iga", _;
    subj_2: "~igas", _;
    subj_3: _, "~igan";
    imperative: _, "~iga", _, _, "~igan";
    negative_imperative: _, _, derived, derived, _;
    participle_m: "~ído", _;
    ),
    // Desdecir: 6 words (bendecir, maldecir, contradecir, desdecir, predecir, condecir). Like Traer, except in 17 cells: pres_2 sg, impf_3 sg, fut_1 sg, fut_1 pl, fut_2 sg, fut_2 pl, ….
    conjugation!("desdecir", 6, "~ecir", like Traer;
    stem lemma: "~ec- like curtir";
    stem rest: "~ij- like reducir";
    pres_2: "~ices", _;
    impf_3: derived, _;
    fut_1: "~eciré|~iré", "~eciremos|~iremos";
    fut_2: "~ecirás|~irás", "~eciréis|~iréis";
    fut_3: "~ecirá|~irá", "~ecirán|~irán";
    cond_1: "~eciría|~iría", "~eciríamos|~iríamos";
    cond_2: "~ecirías|~irías", "~eciríais|~iríais";
    cond_3: derived, "~ecirían|~irían";
    imperative: "~ice", _, _, _, _;
    gerund: "~iciendo";
    participle_m: "~icho", _;
    ),
    // Entredormirse: 6 words (entredormirse, podrirse, adormirse, dormirse, morirse, repodrirse). Like Musirse, except in 12 cells: pres_1 sg, pres_2 sg, impf_3 sg, pret_1 sg, pret_1 pl, subj_1 sg, ….
    conjugation!("entredormirse", 6, "~o~irse", like Musirse;
    stem lemma: "~o~- like musirse";
    stem rest__: "~o~ir- like echar";
    pres_1: "~ue~o", _;
    pres_2: "~ue~es", _;
    impf_3: derived, _;
    pret_1: "~o~í", "~o~imos";
    subj_1: "~ue~a", _;
    subj_2: "~ue~as", _;
    subj_3: _, "~ue~an";
    imperative: "~ué~ete", "~ué~ase", _, _, "~ué~anse";
    participle_m: "~o~ido", _;
    ),
    // Tender: 5 words (discerner, cerner, jerver, herver, jerber). Like Conocer, except in 6 cells: subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, negative_imperative ustedes.
    conjugation!("tender", 5, "~~er", like Conocer;
    stem rest_: "~i~- like curtir";
    stem rest___: "~~- like conocer";
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    ),
    // Caer: 5 words (caer, raer, recaer, decaer, descaer). Like Traer.
    conjugation!("caer", 5, "~er", like Traer;
    stem rest: "~- like creer";
    ),
    // Querer: 5 words (querer, malquerer, bienquerer, desquerer, requerer). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("querer", 5, "~erer", like Echar;
    stem rest_: "~ier- like curtir";
    stem lemma: "~er- like conocer";
    stem rest: "~is- like poner";
    stem rest__: "~err- like echar";
    stem rest___: "~er- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~eriendo";
    participle_m: "~erido", _;
    ),
    // Huir: 5 words (fuir, huir, fluir, fruir, luir). Like Instruir, except in 4 cells: pres_2 pl, pret_1 sg, vos ind, vos imp.
    conjugation!("huir", 5, "~ir", like Instruir;
    pres_2: _, "~is";
    pret_1: "~i", _;
    vos: "~is", _, "~i";
    ),
    // Cocer: 5 words (cocer, escocer, recocer, descocer, precocer). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("cocer", 5, "~ocer", like Echar;
    stem rest_: "~ue- like fruncir";
    stem lemma: "~oc- like conocer";
    stem rest: "~oc- like conocer";
    stem rest__: "~ocer- like echar";
    stem rest___: "~oz- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~ociendo";
    participle_m: "~ocido", _;
    ),
    // Delinquir: 5 words (delinquir, derelinquir, jaquir, derrelinquir, relinquir). Like Sacar, except in 12 cells: pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("delinquir", 5, "~quir", like Sacar;
    stem lemma: "~qu- like curtir";
    stem rest: "~qu- like conocer";
    stem rest__: "~quir- like echar";
    stem rest___: "~c- like conocer";
    pres_2: "~ques", _;
    impf_3: derived, _;
    subj_1: "~ca", _;
    subj_2: "~cas", _;
    subj_3: _, "~can";
    imperative: "~que", "~ca", _, _, "~can";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~quiendo";
    participle_m: "~quido", _;
    ),
    // Afligirse: 5 words (autoexigirse, afligirse, arrigirse, dirigirse, erigirse). Like Regir, except in 8 cells: impf_3 sg, imperative tu, imperative usted, imperative ustedes, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("afligirse", 5, "~igirse", like Regir;
    stem lemma: "~ig- like musirse";
    stem rest: "~ig- like conocer";
    stem rest__: "~igir- like echar";
    stem rest___: "~ij- like autoabastecerse";
    impf_3: derived, _;
    imperative: "~ígete", "~íjase", _, _, "~íjanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~igiéndose";
    participle_m: "~igido", _;
    ),
    // Reconocerse: 5 words (autoreconocerse, autorreconocerse, conocerse, desconocerse, reconocerse). Like Destoserse, except in 8 cells: pres_1 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative usted, imperative ustedes, ….
    conjugation!("reconocerse", 5, "~o~erse", like Destoserse;
    stem rest___: "~oz~- like autoabastecerse";
    pres_1: "~oz~o", _;
    subj_1: "~oz~a", _;
    subj_2: "~oz~as", _;
    subj_3: _, "~oz~an";
    imperative: _, "~óz~ase", _, _, "~óz~anse";
    negative_imperative: _, _, derived, derived, _;
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
    // Revestirse: 5 words (desvestirse, revestirse, sobrevestirse, travestirse, vestirse). Like Incautarse, except in 21 cells: pres_1 sg, pres_2 sg, impf_3 sg, pret_1 sg, pret_1 pl, pret_3 sg, ….
    conjugation!("revestirse", 5, "~e~tirse", like Incautarse;
    stem lemma: "~e~t- like musirse";
    stem rest__: "~e~tir- like echar";
    stem rest___: "~i~t- like autoabastecerse";
    pres_1: "~i~to", _;
    pres_2: "~i~tes", _;
    impf_3: derived, _;
    pret_1: "~e~tí", "~e~timos";
    pret_3: "~i~tió", "~i~tieron";
    subj_1: "~i~ta", _;
    subj_2: "~i~tas", _;
    subj_3: _, "~i~tan";
    subj_impf_1: _, "~i~tiéramos";
    subj_impf_3: "~i~tiera", _;
    subj_impf_se_3: "~i~tiese", _;
    subj_fut_3: "~i~tiere", _;
    imperative: "~í~tete", "~í~tase", _, _, "~í~tanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~i~tiéndose";
    participle_m: "~e~tido", _;
    ),
    // Desleírse: 5 words (reírse, desleírse, engreírse, sonreírse, freírse). Like Refreír with `` after each pattern (`se` after a, n, r), except in 11 cells: impf_1 sg, pret_3 pl, cond_3 sg, subj_1 sg, subj_3 pl, subj_impf_3 sg, ….
    conjugation!("desleírse", 5, "~eírse", like Refreír with "", "se" after "anr";
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
    // Dormir: 5 words (dormir, adormir, maldormir, podrir, repodrir). Like Entredormirse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("dormir", 5, "~o~ir", like Entredormirse;
    stem rest_: "~ue~- like curtir";
    stem lemma: "~o~- like curtir";
    stem rest___: "~u~- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~u~iendo";
    ),
    // Salir: 5 words (salir, asir, desasir, sobresalir, resalir). Like Curtir, except in 9 cells: pres_1 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, imperative usted, ….
    conjugation!("salir", 5, "~ir", like Curtir;
    stem rest__: "~dr- like echar";
    stem rest___: "~g- like conocer";
    pres_1: "~go", _;
    subj_1: "~ga", _;
    subj_2: "~gas", _;
    subj_3: _, "~gan";
    imperative: "~", "~ga", _, _, "~gan";
    negative_imperative: _, _, derived, derived, _;
    ),
    // Interdecir: 5 words (decir, antedecir, redecir, interdecir, entredecir). Like Desdecir, except in 10 cells: fut_1 pl, fut_2 sg, fut_2 pl, fut_3 sg, fut_3 pl, cond_1 sg, ….
    conjugation!("interdecir", 5, "~ecir", like Desdecir;
    stem rest__: "~ir- like echar";
    fut_1: _, derived;
    fut_2: derived, derived;
    fut_3: derived, derived;
    cond_1: derived, derived;
    cond_2: derived, derived;
    cond_3: _, derived;
    ),
    // Agorar: 4 words (agorar, engorar, cogolmar, gorar). Like Volar, except in 4 cells: subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative ustedes.
    conjugation!("agorar", 4, "~o~ar", like Volar;
    stem rest_: "~üe~- like echar";
    subj_3: derived, _;
    negative_imperative: derived, derived, _, _, derived;
    ),
    // Saber: 4 words (saber, caber, haber, resaber). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("saber", 4, "~aber", like Echar;
    stem lemma: "~ab- like conocer";
    stem rest: "~up- like poner";
    stem rest__: "~abr- like echar";
    stem rest___: "~ep- like conocer";
    pres_1: "~é", _;
    pres_2: "~abes", _;
    impf_3: derived, _;
    subj_1: "~epa", _;
    subj_2: "~epas", _;
    subj_3: _, "~epan";
    imperative: "~abe", "~epa", _, _, "~epan";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~abiendo";
    participle_m: "~abido", _;
    ),
    // Forzar: 4 words (almorzar, forzar, esforzar, reforzar). Like Incautarse, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("forzar", 4, "~o~zar", like Incautarse;
    stem rest_: "~ue~- like realizar";
    stem lemma: "~o~z- like echar";
    stem rest: "~o~- like realizar";
    stem rest__: "~o~zar- like echar";
    stem rest___: "~o~c- like echar";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~o~zando";
    participle_m: "~o~zado", _;
    ),
    // Adquirir: 4 words (adquirir, inquirir, perquirir, readquirir). Like Abrir, except in 5 cells: subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative ustedes, participle_m sg.
    conjugation!("adquirir", 4, "~~ir", like Abrir;
    stem rest_: "~e~- like curtir";
    subj_3: derived, _;
    negative_imperative: derived, derived, _, _, derived;
    participle_m: "~~ido", _;
    ),
    // Cohibir: 4 words (prohibir, cohibir, rehinchir, desprohibir). Like Desvivirse, except in 8 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("cohibir", 4, "~i~ir", like Desvivirse;
    stem rest_: "~í~- like curtir";
    stem lemma: "~i~- like curtir";
    stem rest___: "~i~- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~i~iendo";
    ),
    // Morir: 4 words (morir, premorir, desmorir, entremorir). Like Dormir, except in 1 cell: participle_m sg.
    conjugation!("morir", 4, "~o~ir", like Dormir;
    participle_m: "~ue~to", _;
    ),
    // Distinguir: 4 words (distinguir, extinguir, subdistinguir, desextinguir). Like Curtir, except in 5 cells: pres_2 sg, impf_3 sg, imperative tu, gerund, participle_m sg.
    conjugation!("distinguir", 4, "~uir", like Curtir;
    stem lemma: "~u- like curtir";
    stem rest: "~u- like conocer";
    stem rest__: "~uir- like echar";
    pres_2: "~ues", _;
    impf_3: derived, _;
    imperative: "~ue", _, _, _, _;
    gerund: "~uiendo";
    participle_m: "~uido", _;
    ),
    // Torcer: 4 words (torcer, retorcer, destorcer, contorcer). Like Forzar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("torcer", 4, "~o~cer", like Forzar;
    stem rest_: "~ue~- like fruncir";
    stem lemma: "~o~c- like conocer";
    stem rest: "~o~c- like conocer";
    stem rest__: "~o~cer- like echar";
    stem rest___: "~o~z- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
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
    // Fraguarse: 4 words (paniaguarse, sobreaguarse, desaguarse, fraguarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("fraguarse", 4, "~a~uarse", like Incautarse;
    stem lemma: "~a~u- like incautarse";
    stem rest: "~a~- like fraguar";
    stem rest__: "~a~uar- like echar";
    stem rest___: "~a~ü- like incautarse";
    pres_1: "~a~uo", _;
    pres_2: "~a~uas", _;
    impf_3: derived, _;
    subj_1: "~a~üe", _;
    subj_2: "~a~ües", _;
    subj_3: _, "~a~üen";
    imperative: "~á~uate", "~á~üese", _, _, "~á~üense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~a~uándose";
    participle_m: "~a~uado", _;
    ),
    // Moverse: 4 words (lloverse, conmoverse, removerse, moverse). Like Mover with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("moverse", 4, "~overse", like Mover with "", "se" after "r";
    vos: _, _, "~ovete";
    imperative: "~uévete", "~uévase", "~ovámonos", "~oveos", "~uévanse";
    gerund: "~oviéndose";
    ),
    // Reverse: 4 words (reverse, verse, arderse, entreverse). Like Prever with `` after each pattern (`se` after r), except in 8 cells: vos ind, vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, ….
    conjugation!("reverse", 4, "~erse", like Prever with "", "se" after "r";
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
    // Punzarse: 4 words (anarquizarse, barroquizarse, endulzarse, punzarse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("punzarse", 4, "~u~zarse", like Incautarse;
    stem lemma: "~u~z- like incautarse";
    stem rest: "~u~- like realizar";
    stem rest__: "~u~zar- like echar";
    stem rest___: "~u~c- like incautarse";
    pres_1: "~u~zo", _;
    pres_2: "~u~zas", _;
    impf_3: derived, _;
    subj_1: "~u~ce", _;
    subj_2: "~u~ces", _;
    subj_3: _, "~u~cen";
    imperative: "~ú~zate", "~ú~cese", _, _, "~ú~cense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~u~zándose";
    participle_m: "~u~zado", _;
    ),
    // Traslucirse: 4 words (traslucirse, translucirse, deslucirse, lucirse). Like Musirse, except in 8 cells: pres_1 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative usted, imperative ustedes, ….
    conjugation!("traslucirse", 4, "~u~irse", like Musirse;
    stem rest___: "~uz~- like autoabastecerse";
    pres_1: "~uz~o", _;
    subj_1: "~uz~a", _;
    subj_2: "~uz~as", _;
    subj_3: _, "~uz~an";
    imperative: _, "~úz~ase", _, _, "~úz~anse";
    negative_imperative: _, _, derived, derived, _;
    ),
    // Torcerse: 4 words (contorcerse, retorcerse, torcerse, destorcerse). Like Torcer with `` after each pattern (`se` after r), except in 7 cells: vos imp, imperative tu, imperative usted, imperative nosotros, imperative vosotros, imperative ustedes, ….
    conjugation!("torcerse", 4, "~o~cerse", like Torcer with "", "se" after "r";
    vos: _, _, "~o~cete";
    imperative: "~ué~cete", "~ué~zase", "~o~zámonos", "~o~ceos", "~ué~zanse";
    gerund: "~o~ciéndose";
    ),
    // Henchirse: 4 words (rendirse, rehenchirse, henchirse, servirse). Like Rendir with `` after each pattern (`se` after a, n, r), except in 11 cells: impf_1 sg, pret_3 pl, cond_3 sg, subj_1 sg, subj_3 pl, subj_impf_3 sg, ….
    conjugation!("henchirse", 4, "~e~irse", like Rendir with "", "se" after "anr";
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
    // Vencerse: 4 words (remecerse, convencerse, vencerse, desconvencerse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("vencerse", 4, "~e~cerse", like Incautarse;
    stem lemma: "~e~c- like autoabastecerse";
    stem rest: "~e~c- like conocer";
    stem rest__: "~e~cer- like echar";
    stem rest___: "~e~z- like autoabastecerse";
    pres_1: "~e~zo", _;
    pres_2: "~e~ces", _;
    impf_3: derived, _;
    subj_1: "~e~za", _;
    subj_2: "~e~zas", _;
    subj_3: _, "~e~zan";
    imperative: "~é~cete", "~é~zase", _, _, "~é~zanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~e~ciéndose";
    participle_m: "~e~cido", _;
    ),
    // Adyacer: 3 words (yacer, subyacer, adyacer). Like Vencer, except in 12 cells: pres_1 sg, subj_1 sg, subj_1 pl, subj_2 sg, subj_2 pl, subj_3 pl, ….
    conjugation!("adyacer", 3, "~cer", like Vencer;
    pres_1: "~zco|~zgo|~go", _;
    subj_1: "~zca|~zga|~ga", "~zcamos|~zgamos|~gamos";
    subj_2: "~zcas|~zgas|~gas|~zcás|~zgás", "~zcáis|~zgáis|~gáis";
    subj_3: _, "~zcan|~zgan|~gan";
    vos: _, "~gás", _;
    imperative: "~ce|~z", "~zca|~zga|~ga", "~zcamos|~zgamos|~gamos", _, "~zcan|~zgan|~gan";
    negative_imperative: "~zcas|~zgas|~gas", _, _, _, _;
    ),
    // Argüir: 3 words (argüir, reargüir, redargüir). Like Inmiscuirse, except in 11 cells: impf_3 sg, pret_1 sg, pret_1 pl, subj_3 sg, negative_imperative tu, negative_imperative usted, ….
    conjugation!("argüir", 3, "~üir", like Inmiscuirse;
    stem rest_: "~uy- like curtir";
    stem lemma: "~ü- like curtir";
    stem rest__: "~üir- like echar";
    stem rest___: "~uy- like conocer";
    impf_3: derived, _;
    pret_1: "~üí", "~üimos";
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~uyendo";
    participle_m: "~üido", _;
    ),
    // Jugar: 3 words (jugar, rejugar, sobrejugar). Like Recalentar, except in 11 cells: pres_1 sg, pres_2 sg, pret_1 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("jugar", 3, "~~ar", like Recalentar;
    stem rest___: "~~u- like echar";
    pres_1: "~e~o", _;
    pres_2: "~e~as", _;
    pret_1: "~~ué", _;
    subj_1: "~e~ue", _;
    subj_2: "~e~ues", _;
    subj_3: _, "~e~uen";
    imperative: "~e~a", "~e~ue", _, _, "~e~uen";
    negative_imperative: _, _, derived, derived, _;
    ),
    // Romper: 3 words (romper, derromper, arromper). Like Echar, except in 9 cells: impf_3 sg, subj_3 sg, negative_imperative tu, negative_imperative usted, negative_imperative nosotros, negative_imperative vosotros, ….
    conjugation!("romper", 3, "~mper", like Echar;
    stem rest_: "~mp- like curtir";
    stem lemma: "~mp- like conocer";
    stem rest: "~mp- like conocer";
    stem rest__: "~mper- like echar";
    stem rest___: "~mp- like conocer";
    impf_3: derived, _;
    subj_3: derived, _;
    negative_imperative: derived, derived, derived, derived, derived;
    gerund: "~mpiendo";
    participle_m: "~to", _;
    ),
    // Valer: 3 words (valer, equivaler, prevaler). Like Salir, except in 2 cells: impf_3 sg, imperative tu.
    conjugation!("valer", 3, "~er", like Salir;
    stem lemma: "~- like conocer";
    impf_3: derived, _;
    imperative: "~e", _, _, _, _;
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
    // Azoarse: 3 words (abarloarse, azoarse, barloarse). Like Echar, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("azoarse", 3, "~oarse", like Echar;
    stem lemma: "~o- like incautarse";
    stem rest: "~o- like echar";
    stem rest__: "~oar- like echar";
    stem rest___: "~o- like incautarse";
    pres_1: "~oo", _;
    pres_2: "~oas", _;
    impf_3: derived, _;
    subj_1: "~oe", _;
    subj_2: "~oes", _;
    subj_3: _, "~oen";
    imperative: "~óate", "~óese", _, _, "~óense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~oándose";
    participle_m: "~oado", _;
    ),
    // Nacerse: 3 words (complacerse, aplacerse, nacerse). Like Deshacerse, except in 18 cells: pres_1 sg, pret_1 sg, pret_1 pl, pret_3 sg, pret_3 pl, subj_1 sg, ….
    conjugation!("nacerse", 3, "~acerse", like Deshacerse;
    stem rest__: "~acer- like echar";
    stem rest___: "~azc- like autoabastecerse";
    pres_1: "~azco", _;
    pret_1: "~ací", "~acimos";
    pret_3: "~ació|~ugo", "~acieron|~uguieron";
    subj_1: "~azca", _;
    subj_2: "~azcas", _;
    subj_3: _, "~azcan";
    subj_impf_1: _, "~aciéramos";
    subj_impf_3: "~aciera|~uguiera", _;
    subj_impf_se_3: "~aciese|~uguiese", _;
    subj_fut_3: "~aciere|~uguiere", _;
    imperative: "~ácete", "~ázcase", _, _, "~ázcanse";
    negative_imperative: _, _, derived, derived, _;
    participle_m: "~acido", _;
    ),
    // Trenzarse: 3 words (trenzarse, destrenzarse, homogeneizarse). Like Vencerse, except in 12 cells: pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, imperative tu, ….
    conjugation!("trenzarse", 3, "~e~zarse", like Vencerse;
    stem lemma: "~e~z- like incautarse";
    stem rest: "~e~- like realizar";
    stem rest__: "~e~zar- like echar";
    stem rest___: "~e~c- like incautarse";
    pres_2: "~e~zas", _;
    impf_3: derived, _;
    subj_1: "~e~ce", _;
    subj_2: "~e~ces", _;
    subj_3: _, "~e~cen";
    imperative: "~é~zate", "~é~cese", _, _, "~é~cense";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~e~zándose";
    participle_m: "~e~zado", _;
    ),
    // Cubrirse: 3 words (descubrirse, cubrirse, encubrirse).
    conjugation!("cubrirse", 3, "~u~~irse";
    stem lemma: "~u~~- like musirse";
    stem rest: "~u~~- like conocer";
    stem rest__: "~u~~ir- like echar";
    stem rest___: "~u~~- like autoabastecerse";
    pres_1: "~u~~o", _;
    pres_2: "~u~~es", _;
    impf_3: derived, _;
    subj_1: "~u~~a", _;
    subj_2: "~u~~as", _;
    subj_3: _, "~u~~an";
    imperative: "~ú~~ete", "~ú~~ase", _, _, "~ú~~anse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~u~~iéndose";
    participle_m: "~u~ie~to", _;
    ),
    // Derretirse: 3 words (derretirse, competirse, repetirse). Like Echar, except in 21 cells: pres_1 sg, pres_2 sg, impf_3 sg, pret_1 sg, pret_1 pl, pret_3 sg, ….
    conjugation!("derretirse", 3, "~etirse", like Echar;
    stem lemma: "~et- like musirse";
    stem rest__: "~etir- like echar";
    stem rest___: "~it- like autoabastecerse";
    pres_1: "~ito", _;
    pres_2: "~ites", _;
    impf_3: derived, _;
    pret_1: "~etí", "~etimos";
    pret_3: "~itió", "~itieron";
    subj_1: "~ita", _;
    subj_2: "~itas", _;
    subj_3: _, "~itan";
    subj_impf_1: _, "~itiéramos";
    subj_impf_3: "~itiera", _;
    subj_impf_se_3: "~itiese", _;
    subj_fut_3: "~itiere", _;
    imperative: "~ítete", "~ítase", _, _, "~ítanse";
    negative_imperative: _, _, derived, derived, _;
    gerund: "~itiéndose";
    participle_m: "~etido", _;
    ),
    // Fingirse: 3 words (refringirse, fingirse, atingirse). Like Incautarse, except in 13 cells: pres_1 sg, pres_2 sg, impf_3 sg, subj_1 sg, subj_2 sg, subj_3 pl, ….
    conjugation!("fingirse", 3, "~i~girse", like Incautarse;
    stem lemma: "~i~g- like musirse";
    stem rest: "~i~g- like conocer";
    stem rest__: "~i~gir- like echar";
    stem rest___: "~i~j- like autoabastecerse";
    pres_1: "~i~jo", _;
    pres_2: "~i~ges", _;
    impf_3: derived, _;
    subj_1: "~i~ja", _;
    subj_2: "~i~jas", _;
    subj_3: _, "~i~jan";
    imperative: "~í~gete", "~í~jase", _, _, "~í~janse";
    negative_imperative: _, _, derived, derived, _;
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
    // Reírse: 1 words (embaírse). Like Creerse, except in 20 cells: pres_3 sg, pres_3 pl, impf_3 sg, subj_impf_1 sg, subj_impf_2 sg, subj_impf_2 pl, ….
    conjugation!("reírse", 1, "~~írse", like Creerse;
    stem lemma: "~~í- like freírse";
    stem rest__: "~~ir- like echar";
    pres_3: "~~e", "~~en";
    impf_3: derived, _;
    subj_impf_1: "~~yera", _;
    subj_impf_2: "~~yeras", "~~yerais";
    subj_impf_3: derived, "~~yeran";
    subj_impf_se_1: "~~yese", "~~yésemos";
    subj_impf_se_2: "~~yeses", "~~yeseis";
    subj_impf_se_3: derived, "~~yesen";
    subj_fut_1: "~~yere", "~~yéremos";
    subj_fut_2: "~~yeres", "~~yereis";
    subj_fut_3: derived, "~~yeren";
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
