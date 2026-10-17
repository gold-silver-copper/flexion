//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ō`, `agitō`: the stem and `ō`). A stem in two pieces (`~e~dō`, `comprehendō`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~gōˇ`: `compungō`).

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~- like agitō\"`: the slot's cells are `agitō`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
/// `act_pres_1` (sg, pl)
/// `act_pres_2` (sg, pl)
/// `act_pres_3` (sg, pl)
/// `act_impf_1` (sg, pl)
/// `act_impf_2` (sg, pl)
/// `act_impf_3` (sg, pl)
/// `act_fut_1` (sg, pl)
/// `act_fut_2` (sg, pl)
/// `act_fut_3` (sg, pl)
/// `act_perf_1` (sg, pl)
/// `act_perf_2` (sg, pl)
/// `act_perf_3` (sg, pl)
/// `act_plup_1` (sg, pl)
/// `act_plup_2` (sg, pl)
/// `act_plup_3` (sg, pl)
/// `act_futp_1` (sg, pl)
/// `act_futp_2` (sg, pl)
/// `act_futp_3` (sg, pl)
/// `pass_pres_1` (sg, pl)
/// `pass_pres_2` (sg, pl)
/// `pass_pres_3` (sg, pl)
/// `pass_impf_1` (sg, pl)
/// `pass_impf_2` (sg, pl)
/// `pass_impf_3` (sg, pl)
/// `pass_fut_1` (sg, pl)
/// `pass_fut_2` (sg, pl)
/// `pass_fut_3` (sg, pl)
/// `pass_perf_1` (sg, pl)
/// `pass_perf_2` (sg, pl)
/// `pass_perf_3` (sg, pl)
/// `pass_plup_1` (sg, pl)
/// `pass_plup_2` (sg, pl)
/// `pass_plup_3` (sg, pl)
/// `pass_futp_1` (sg, pl)
/// `pass_futp_2` (sg, pl)
/// `pass_futp_3` (sg, pl)
/// `subjunctive_act_pres_1` (sg, pl)
/// `subjunctive_act_pres_2` (sg, pl)
/// `subjunctive_act_pres_3` (sg, pl)
/// `subjunctive_act_impf_1` (sg, pl)
/// `subjunctive_act_impf_2` (sg, pl)
/// `subjunctive_act_impf_3` (sg, pl)
/// `subjunctive_act_perf_1` (sg, pl)
/// `subjunctive_act_perf_2` (sg, pl)
/// `subjunctive_act_perf_3` (sg, pl)
/// `subjunctive_act_plup_1` (sg, pl)
/// `subjunctive_act_plup_2` (sg, pl)
/// `subjunctive_act_plup_3` (sg, pl)
/// `subjunctive_pass_pres_1` (sg, pl)
/// `subjunctive_pass_pres_2` (sg, pl)
/// `subjunctive_pass_pres_3` (sg, pl)
/// `subjunctive_pass_impf_1` (sg, pl)
/// `subjunctive_pass_impf_2` (sg, pl)
/// `subjunctive_pass_impf_3` (sg, pl)
/// `subjunctive_pass_perf_1` (sg, pl)
/// `subjunctive_pass_perf_2` (sg, pl)
/// `subjunctive_pass_perf_3` (sg, pl)
/// `subjunctive_pass_plup_1` (sg, pl)
/// `subjunctive_pass_plup_2` (sg, pl)
/// `subjunctive_pass_plup_3` (sg, pl)
/// `present_imperative_act` (sg, pl)
/// `present_imperative_pass` (sg, pl)
/// `future_imperative_act_2` (sg, pl)
/// `future_imperative_act_3` (sg, pl)
/// `future_imperative_pass_2` (sg, pl)
/// `future_imperative_pass_3` (sg, pl)
/// `infinitive_act` (pres, perf, fut)
/// `infinitive_pass` (pres, perf, fut)
/// `participle` (pres_act, perf_pass, fut_act, fut_pass)
/// `gerund` (nom, gen_, dat, acc, abl, voc)
/// `supine` (nom, gen_, dat, acc, abl, voc)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem indicative_act_pres_1_pl: $s1:tt;)?
     $(stem indicative_act_perf_1_sg: $s2:tt;)?
     $(stem supine_acc: $s3:tt;)?
     $(act_pres_1: $c0:tt, $c1:tt;)?
     $(act_pres_2: $c2:tt, $c3:tt;)?
     $(act_pres_3: $c4:tt, $c5:tt;)?
     $(act_impf_1: $c6:tt, $c7:tt;)?
     $(act_impf_2: $c8:tt, $c9:tt;)?
     $(act_impf_3: $c10:tt, $c11:tt;)?
     $(act_fut_1: $c12:tt, $c13:tt;)?
     $(act_fut_2: $c14:tt, $c15:tt;)?
     $(act_fut_3: $c16:tt, $c17:tt;)?
     $(act_perf_1: $c18:tt, $c19:tt;)?
     $(act_perf_2: $c20:tt, $c21:tt;)?
     $(act_perf_3: $c22:tt, $c23:tt;)?
     $(act_plup_1: $c24:tt, $c25:tt;)?
     $(act_plup_2: $c26:tt, $c27:tt;)?
     $(act_plup_3: $c28:tt, $c29:tt;)?
     $(act_futp_1: $c30:tt, $c31:tt;)?
     $(act_futp_2: $c32:tt, $c33:tt;)?
     $(act_futp_3: $c34:tt, $c35:tt;)?
     $(pass_pres_1: $c36:tt, $c37:tt;)?
     $(pass_pres_2: $c38:tt, $c39:tt;)?
     $(pass_pres_3: $c40:tt, $c41:tt;)?
     $(pass_impf_1: $c42:tt, $c43:tt;)?
     $(pass_impf_2: $c44:tt, $c45:tt;)?
     $(pass_impf_3: $c46:tt, $c47:tt;)?
     $(pass_fut_1: $c48:tt, $c49:tt;)?
     $(pass_fut_2: $c50:tt, $c51:tt;)?
     $(pass_fut_3: $c52:tt, $c53:tt;)?
     $(pass_perf_1: $c54:tt, $c55:tt;)?
     $(pass_perf_2: $c56:tt, $c57:tt;)?
     $(pass_perf_3: $c58:tt, $c59:tt;)?
     $(pass_plup_1: $c60:tt, $c61:tt;)?
     $(pass_plup_2: $c62:tt, $c63:tt;)?
     $(pass_plup_3: $c64:tt, $c65:tt;)?
     $(pass_futp_1: $c66:tt, $c67:tt;)?
     $(pass_futp_2: $c68:tt, $c69:tt;)?
     $(pass_futp_3: $c70:tt, $c71:tt;)?
     $(subjunctive_act_pres_1: $c72:tt, $c73:tt;)?
     $(subjunctive_act_pres_2: $c74:tt, $c75:tt;)?
     $(subjunctive_act_pres_3: $c76:tt, $c77:tt;)?
     $(subjunctive_act_impf_1: $c78:tt, $c79:tt;)?
     $(subjunctive_act_impf_2: $c80:tt, $c81:tt;)?
     $(subjunctive_act_impf_3: $c82:tt, $c83:tt;)?
     $(subjunctive_act_perf_1: $c84:tt, $c85:tt;)?
     $(subjunctive_act_perf_2: $c86:tt, $c87:tt;)?
     $(subjunctive_act_perf_3: $c88:tt, $c89:tt;)?
     $(subjunctive_act_plup_1: $c90:tt, $c91:tt;)?
     $(subjunctive_act_plup_2: $c92:tt, $c93:tt;)?
     $(subjunctive_act_plup_3: $c94:tt, $c95:tt;)?
     $(subjunctive_pass_pres_1: $c96:tt, $c97:tt;)?
     $(subjunctive_pass_pres_2: $c98:tt, $c99:tt;)?
     $(subjunctive_pass_pres_3: $c100:tt, $c101:tt;)?
     $(subjunctive_pass_impf_1: $c102:tt, $c103:tt;)?
     $(subjunctive_pass_impf_2: $c104:tt, $c105:tt;)?
     $(subjunctive_pass_impf_3: $c106:tt, $c107:tt;)?
     $(subjunctive_pass_perf_1: $c108:tt, $c109:tt;)?
     $(subjunctive_pass_perf_2: $c110:tt, $c111:tt;)?
     $(subjunctive_pass_perf_3: $c112:tt, $c113:tt;)?
     $(subjunctive_pass_plup_1: $c114:tt, $c115:tt;)?
     $(subjunctive_pass_plup_2: $c116:tt, $c117:tt;)?
     $(subjunctive_pass_plup_3: $c118:tt, $c119:tt;)?
     $(present_imperative_act: $c120:tt, $c121:tt;)?
     $(present_imperative_pass: $c122:tt, $c123:tt;)?
     $(future_imperative_act_2: $c124:tt, $c125:tt;)?
     $(future_imperative_act_3: $c126:tt, $c127:tt;)?
     $(future_imperative_pass_2: $c128:tt, $c129:tt;)?
     $(future_imperative_pass_3: $c130:tt, $c131:tt;)?
     $(infinitive_act: $c132:tt, $c133:tt, $c134:tt;)?
     $(infinitive_pass: $c135:tt, $c136:tt, $c137:tt;)?
     $(participle: $c138:tt, $c139:tt, $c140:tt, $c141:tt;)?
     $(gerund: $c142:tt, $c143:tt, $c144:tt, $c145:tt, $c146:tt, $c147:tt;)?
     $(supine: $c148:tt, $c149:tt, $c150:tt, $c151:tt, $c152:tt, $c153:tt;)?
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
                $(flexion::cell!($c73),)? "\t",
                $(flexion::cell!($c74),)? "\t",
                $(flexion::cell!($c75),)? "\t",
                $(flexion::cell!($c76),)? "\t",
                $(flexion::cell!($c77),)? "\t",
                $(flexion::cell!($c78),)? "\t",
                $(flexion::cell!($c79),)? "\t",
                $(flexion::cell!($c80),)? "\t",
                $(flexion::cell!($c81),)? "\t",
                $(flexion::cell!($c82),)? "\t",
                $(flexion::cell!($c83),)? "\t",
                $(flexion::cell!($c84),)? "\t",
                $(flexion::cell!($c85),)? "\t",
                $(flexion::cell!($c86),)? "\t",
                $(flexion::cell!($c87),)? "\t",
                $(flexion::cell!($c88),)? "\t",
                $(flexion::cell!($c89),)? "\t",
                $(flexion::cell!($c90),)? "\t",
                $(flexion::cell!($c91),)? "\t",
                $(flexion::cell!($c92),)? "\t",
                $(flexion::cell!($c93),)? "\t",
                $(flexion::cell!($c94),)? "\t",
                $(flexion::cell!($c95),)? "\t",
                $(flexion::cell!($c96),)? "\t",
                $(flexion::cell!($c97),)? "\t",
                $(flexion::cell!($c98),)? "\t",
                $(flexion::cell!($c99),)? "\t",
                $(flexion::cell!($c100),)? "\t",
                $(flexion::cell!($c101),)? "\t",
                $(flexion::cell!($c102),)? "\t",
                $(flexion::cell!($c103),)? "\t",
                $(flexion::cell!($c104),)? "\t",
                $(flexion::cell!($c105),)? "\t",
                $(flexion::cell!($c106),)? "\t",
                $(flexion::cell!($c107),)? "\t",
                $(flexion::cell!($c108),)? "\t",
                $(flexion::cell!($c109),)? "\t",
                $(flexion::cell!($c110),)? "\t",
                $(flexion::cell!($c111),)? "\t",
                $(flexion::cell!($c112),)? "\t",
                $(flexion::cell!($c113),)? "\t",
                $(flexion::cell!($c114),)? "\t",
                $(flexion::cell!($c115),)? "\t",
                $(flexion::cell!($c116),)? "\t",
                $(flexion::cell!($c117),)? "\t",
                $(flexion::cell!($c118),)? "\t",
                $(flexion::cell!($c119),)? "\t",
                $(flexion::cell!($c120),)? "\t",
                $(flexion::cell!($c121),)? "\t",
                $(flexion::cell!($c122),)? "\t",
                $(flexion::cell!($c123),)? "\t",
                $(flexion::cell!($c124),)? "\t",
                $(flexion::cell!($c125),)? "\t",
                $(flexion::cell!($c126),)? "\t",
                $(flexion::cell!($c127),)? "\t",
                $(flexion::cell!($c128),)? "\t",
                $(flexion::cell!($c129),)? "\t",
                $(flexion::cell!($c130),)? "\t",
                $(flexion::cell!($c131),)? "\t",
                $(flexion::cell!($c132),)? "\t",
                $(flexion::cell!($c133),)? "\t",
                $(flexion::cell!($c134),)? "\t",
                $(flexion::cell!($c135),)? "\t",
                $(flexion::cell!($c136),)? "\t",
                $(flexion::cell!($c137),)? "\t",
                $(flexion::cell!($c138),)? "\t",
                $(flexion::cell!($c139),)? "\t",
                $(flexion::cell!($c140),)? "\t",
                $(flexion::cell!($c141),)? "\t",
                $(flexion::cell!($c142),)? "\t",
                $(flexion::cell!($c143),)? "\t",
                $(flexion::cell!($c144),)? "\t",
                $(flexion::cell!($c145),)? "\t",
                $(flexion::cell!($c146),)? "\t",
                $(flexion::cell!($c147),)? "\t",
                $(flexion::cell!($c148),)? "\t",
                $(flexion::cell!($c149),)? "\t",
                $(flexion::cell!($c150),)? "\t",
                $(flexion::cell!($c151),)? "\t",
                $(flexion::cell!($c152),)? "\t",
                $(flexion::cell!($c153),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
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

/// What a conjugation adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
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
    /// Like `agitō`: 2,609 words.
    Agitō,
    /// Like `modulor`: 357 words.
    Modulor,
    /// Like `sūmō`: 265 words.
    Sūmō,
    /// Like `habeō`: 256 words.
    Habeō,
    /// Like `mūniō`: 219 words.
    Mūniō,
    /// Like `quiēscō`: 152 words.
    Quiēscō,
    /// Like `compercō`: 116 words.
    Compercō,
    /// Like `minuō`: 62 words.
    Minuō,
    /// Like `olfaciō`: 53 words.
    Olfaciō,
    /// Like `comprehendō`: 49 words.
    Comprehendō,
    /// Like `excipiō`: 44 words.
    Excipiō,
    /// Like `teneō`: 41 words.
    Teneō,
    /// Like `cubō`: 39 words.
    Cubō,
    /// Like `compescō`: 38 words.
    Compescō,
    /// Like `prōdō`: 34 words.
    Prōdō,
    /// Like `circumscrībō`: 31 words.
    Circumscrībō,
    /// Like `petō`: 29 words.
    Petō,
    /// Like `vertō`: 28 words.
    Vertō,
    /// Like `mittō`: 28 words.
    Mittō,
    /// Like `impleō`: 28 words.
    Impleō,
    /// Like `labōrō`: 28 words.
    Labōrō,
    /// Like `dēfīgō`: 28 words.
    Dēfīgō,
    /// Like `perstringō`: 27 words.
    Perstringō,
    /// Like `obeō`: 27 words.
    Obeō,
    /// Like `mōlior`: 27 words.
    Mōlior,
    /// Like `dēdūcō`: 26 words.
    Dēdūcō,
    /// Like `moveō`: 26 words.
    Moveō,
    /// Like `repōnō`: 26 words.
    Repōnō,
    /// Like `mergō`: 25 words.
    Mergō,
    /// Like `compungō`: 25 words.
    Compungō,
    /// Like `dēpellō`: 24 words.
    Dēpellō,
    /// Like `tueor`: 23 words.
    Tueor,
    /// Like `exsarciō`: 22 words.
    Exsarciō,
    /// Like `praecīdō`: 22 words.
    Praecīdō,
    /// Like `solvō`: 21 words.
    Solvō,
    /// Like `pāscor`: 21 words.
    Pāscor,
    /// Like `coniciō`: 21 words.
    Coniciō,
    /// Like `nuō`: 20 words.
    Nuō,
    /// Like `cēdō`: 19 words.
    Cēdō,
    /// Like `ēgredior`: 19 words.
    Ēgredior,
    /// Like `prōferō`: 18 words.
    Prōferō,
    /// Like `gerō`: 17 words.
    Gerō,
    /// Like `nectō`: 17 words.
    Nectō,
    /// Like `dispiciō`: 17 words.
    Dispiciō,
    /// Like `trahō`: 17 words.
    Trahō,
    /// Like `compingō`: 17 words.
    Compingō,
    /// Like `exigō`: 17 words.
    Exigō,
    /// Like `effundō`: 16 words.
    Effundō,
    /// Like `corripiō`: 16 words.
    Corripiō,
    /// Like `accumbō`: 16 words.
    Accumbō,
    /// Like `intersaepiō`: 16 words.
    Intersaepiō,
    /// Like `ārdeō`: 15 words.
    Ārdeō,
    /// Like `ērigō`: 15 words.
    Ērigō,
    /// Like `dēsinō`: 14 words.
    Dēsinō,
    /// Like `dīcō`: 14 words.
    Dīcō,
    /// Like `tingō`: 14 words.
    Tingō,
    /// Like `regō`: 14 words.
    Regō,
    /// Like `struō`: 14 words.
    Struō,
    /// Like `rumpō`: 14 words.
    Rumpō,
    /// Like `fodiō`: 14 words.
    Fodiō,
    /// Like `asserviō`: 14 words.
    Asserviō,
    /// Like `autumnāscit`: 14 words.
    Autumnāscit,
    /// Like `illābor`: 13 words.
    Illābor,
    /// Like `ēvehō`: 13 words.
    Ēvehō,
    /// Like `cōnsequor`: 13 words.
    Cōnsequor,
    /// Like `madeō`: 13 words.
    Madeō,
    /// Like `excellō`: 13 words.
    Excellō,
    /// Like `scindō`: 13 words.
    Scindō,
    /// Like `īnsistō`: 13 words.
    Īnsistō,
    /// Like `feriō`: 12 words.
    Feriō,
    /// Like `fugiō`: 12 words.
    Fugiō,
    /// Like `obloquor`: 12 words.
    Obloquor,
    /// Like `possideō`: 12 words.
    Possideō,
    /// Like `intersum`: 12 words.
    Intersum,
    /// Like `cōnsulō`: 11 words.
    Cōnsulō,
    /// Like `contineō`: 11 words.
    Contineō,
    /// Like `opprimō`: 11 words.
    Opprimō,
    /// Like `incidō`: 11 words.
    Incidō,
    /// Like `cernō`: 11 words.
    Cernō,
    /// Like `percutiō`: 11 words.
    Percutiō,
    /// Like `ūtor`: 11 words.
    Ūtor,
    /// Like `abblandior`: 11 words.
    Abblandior,
    /// Like `attondeō`: 11 words.
    Attondeō,
    /// Like `micō`: 11 words.
    Micō,
    /// Like `calvor`: 11 words.
    Calvor,
    /// Like `tergeō`: 10 words.
    Tergeō,
    /// Like `legō`: 10 words.
    Legō,
    /// Like `terō`: 10 words.
    Terō,
    /// Like `contendō`: 10 words.
    Contendō,
    /// Like `vīsō`: 10 words.
    Vīsō,
    /// Like `abnuō`: 10 words.
    Abnuō,
    /// Like `aperiō`: 10 words.
    Aperiō,
    /// Like `contorqueō`: 9 words.
    Contorqueō,
    /// Like `inveniō`: 9 words.
    Inveniō,
    /// Like `sternō`: 9 words.
    Sternō,
    /// Like `sentiō`: 9 words.
    Sentiō,
    /// Like `fluō`: 9 words.
    Fluō,
    /// Like `īnsiliō`: 9 words.
    Īnsiliō,
    /// Like `interstinguō`: 9 words.
    Interstinguō,
    /// Like `dēmorior`: 9 words.
    Dēmorior,
    /// Like `pervincō`: 8 words.
    Pervincō,
    /// Like `coorior`: 8 words.
    Coorior,
    /// Like `immisceō`: 8 words.
    Immisceō,
    /// Like `cohaereō`: 8 words.
    Cohaereō,
    /// Like `dēpudet`: 8 words.
    Dēpudet,
    /// Like `gaudeō`: 7 words.
    Gaudeō,
    /// Like `comedō`: 7 words.
    Comedō,
    /// Like `capiō`: 7 words.
    Capiō,
    /// Like `maneō`: 7 words.
    Maneō,
    /// Like `augeō`: 7 words.
    Augeō,
    /// Like `ferveō`: 7 words.
    Ferveō,
    /// Like `īnsīdō`: 7 words.
    Īnsīdō,
    /// Like `dēpendeō`: 7 words.
    Dēpendeō,
    /// Like `peragō`: 7 words.
    Peragō,
    /// Like `cognōscō`: 7 words.
    Cognōscō,
    /// Like `coquō`: 7 words.
    Coquō,
    /// Like `īnserō`: 7 words.
    Īnserō,
    /// Like `cēnseō`: 6 words.
    Cēnseō,
    /// Like `videō`: 6 words.
    Videō,
    /// Like `pergō`: 6 words.
    Pergō,
    /// Like `advīvō`: 6 words.
    Advīvō,
    /// Like `cōnstō`: 6 words.
    Cōnstō,
    /// Like `innectō`: 6 words.
    Innectō,
    /// Like `eximō`: 6 words.
    Eximō,
    /// Like `vinciō`: 6 words.
    Vinciō,
    /// Like `dēfluō`: 6 words.
    Dēfluō,
    /// Like `exsorbeō`: 6 words.
    Exsorbeō,
    /// Like `cōnfugiō`: 6 words.
    Cōnfugiō,
    /// Like `apīscor`: 6 words.
    Apīscor,
    /// Like `ēdiscō`: 6 words.
    Ēdiscō,
    /// Like `deicō`: 6 words.
    Deicō,
    /// Like `scandō`: 6 words.
    Scandō,
    /// Like `occurrō`: 5 words.
    Occurrō,
    /// Like `sūmō`: 5 words.
    Sūmō2,
    /// Like `vēnumdō`: 5 words.
    Vēnumdō,
    /// Like `serviō`: 5 words.
    Serviō,
    /// Like `adipīscor`: 5 words.
    Adipīscor,
    /// Like `astō`: 5 words.
    Astō,
    /// Like `occinō`: 5 words.
    Occinō,
    /// Like `decet`: 5 words.
    Decet,
    /// Like `supersedeō`: 5 words.
    Supersedeō,
    /// Like `clueō`: 5 words.
    Clueō,
    /// Like `ambigō`: 5 words.
    Ambigō,
    /// Like `contremīscō`: 5 words.
    Contremīscō,
    /// Like `colō`: 5 words.
    Colō,
    /// Like `clangō`: 5 words.
    Clangō,
    /// Like `caveō`: 4 words.
    Caveō,
    /// Like `cieō`: 4 words.
    Cieō,
    /// Like `patior`: 4 words.
    Patior,
    /// Like `fīō`: 4 words.
    Fīō,
    /// Like `relinquō`: 4 words.
    Relinquō,
    /// Like `sapiō`: 4 words.
    Sapiō,
    /// Like `abeō`: 4 words.
    Abeō,
    /// Like `ēvellō`: 4 words.
    Ēvellō,
    /// Like `currō`: 4 words.
    Currō,
    /// Like `nītor`: 4 words.
    Nītor,
    /// Like `obtundō`: 4 words.
    Obtundō,
    /// Like `hauriō`: 4 words.
    Hauriō,
    /// Like `comperiō`: 4 words.
    Comperiō,
    /// Like `circumvehor`: 4 words.
    Circumvehor,
    /// Like `adnītor`: 4 words.
    Adnītor,
    /// Like `extruō`: 4 words.
    Extruō,
    /// Like `frangō`: 4 words.
    Frangō,
    /// Like `fricō`: 4 words.
    Fricō,
    /// Like `mulceō`: 4 words.
    Mulceō,
    /// Like `apage`: 4 words.
    Apage,
    /// Like `ēvādō`: 3 words.
    Ēvādō,
    /// Like `coniungō`: 3 words.
    Coniungō,
    /// Like `quaerō`: 3 words.
    Quaerō,
    /// Like `adliciō`: 3 words.
    Adliciō,
    /// Like `stō`: 3 words.
    Stō,
    /// Like `dīluō`: 3 words.
    Dīluō,
    /// Like `misceō`: 3 words.
    Misceō,
    /// Like `iubeō`: 3 words.
    Iubeō,
    /// Like `fundō`: 3 words.
    Fundō,
    /// Like `scatō`: 3 words.
    Scatō,
    /// Like `explicō`: 3 words.
    Explicō,
    /// Like `dēfungor`: 3 words.
    Dēfungor,
    /// Like `fīdō`: 3 words.
    Fīdō,
    /// Like `libet`: 3 words.
    Libet,
    /// Like `coepī`: 3 words.
    Coepī,
    /// Like `dētergeō`: 3 words.
    Dētergeō,
    /// Like `sufferō`: 3 words.
    Sufferō,
    /// Like `antecapiō`: 3 words.
    Antecapiō,
    /// Like `commeminī`: 3 words.
    Commeminī,
    /// Like `contemnō`: 3 words.
    Contemnō,
    /// Like `dētorreō`: 3 words.
    Dētorreō,
    /// Like `superstō`: 3 words.
    Superstō,
    /// Like `gignō`: 3 words.
    Gignō,
    /// Like `assiliō`: 3 words.
    Assiliō,
    /// Like `convalēscō`: 3 words.
    Convalēscō,
    /// Like `tuor`: 3 words.
    Tuor,
    /// Like `adhērēscō`: 3 words.
    Adhērēscō,
    /// Like `iūxtāpōnō`: 3 words.
    Iūxtāpōnō,
    /// Like `adlavō`: 3 words.
    Adlavō,
    /// Like `effervēscō`: 3 words.
    Effervēscō,
    /// Like `dīlūculat`: 2 words.
    Dīlūculat,
    /// Like `coerceo`: 1 words.
    Coerceo,
    /// Like `rēfert`: 1 words.
    Rēfert,
    /// Like `coemo`: 1 words.
    Coemo,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 200] = [
        Conjugation::Agitō,
        Conjugation::Modulor,
        Conjugation::Sūmō,
        Conjugation::Habeō,
        Conjugation::Mūniō,
        Conjugation::Quiēscō,
        Conjugation::Compercō,
        Conjugation::Minuō,
        Conjugation::Olfaciō,
        Conjugation::Comprehendō,
        Conjugation::Excipiō,
        Conjugation::Teneō,
        Conjugation::Cubō,
        Conjugation::Compescō,
        Conjugation::Prōdō,
        Conjugation::Circumscrībō,
        Conjugation::Petō,
        Conjugation::Vertō,
        Conjugation::Mittō,
        Conjugation::Impleō,
        Conjugation::Labōrō,
        Conjugation::Dēfīgō,
        Conjugation::Perstringō,
        Conjugation::Obeō,
        Conjugation::Mōlior,
        Conjugation::Dēdūcō,
        Conjugation::Moveō,
        Conjugation::Repōnō,
        Conjugation::Mergō,
        Conjugation::Compungō,
        Conjugation::Dēpellō,
        Conjugation::Tueor,
        Conjugation::Exsarciō,
        Conjugation::Praecīdō,
        Conjugation::Solvō,
        Conjugation::Pāscor,
        Conjugation::Coniciō,
        Conjugation::Nuō,
        Conjugation::Cēdō,
        Conjugation::Ēgredior,
        Conjugation::Prōferō,
        Conjugation::Gerō,
        Conjugation::Nectō,
        Conjugation::Dispiciō,
        Conjugation::Trahō,
        Conjugation::Compingō,
        Conjugation::Exigō,
        Conjugation::Effundō,
        Conjugation::Corripiō,
        Conjugation::Accumbō,
        Conjugation::Intersaepiō,
        Conjugation::Ārdeō,
        Conjugation::Ērigō,
        Conjugation::Dēsinō,
        Conjugation::Dīcō,
        Conjugation::Tingō,
        Conjugation::Regō,
        Conjugation::Struō,
        Conjugation::Rumpō,
        Conjugation::Fodiō,
        Conjugation::Asserviō,
        Conjugation::Autumnāscit,
        Conjugation::Illābor,
        Conjugation::Ēvehō,
        Conjugation::Cōnsequor,
        Conjugation::Madeō,
        Conjugation::Excellō,
        Conjugation::Scindō,
        Conjugation::Īnsistō,
        Conjugation::Feriō,
        Conjugation::Fugiō,
        Conjugation::Obloquor,
        Conjugation::Possideō,
        Conjugation::Intersum,
        Conjugation::Cōnsulō,
        Conjugation::Contineō,
        Conjugation::Opprimō,
        Conjugation::Incidō,
        Conjugation::Cernō,
        Conjugation::Percutiō,
        Conjugation::Ūtor,
        Conjugation::Abblandior,
        Conjugation::Attondeō,
        Conjugation::Micō,
        Conjugation::Calvor,
        Conjugation::Tergeō,
        Conjugation::Legō,
        Conjugation::Terō,
        Conjugation::Contendō,
        Conjugation::Vīsō,
        Conjugation::Abnuō,
        Conjugation::Aperiō,
        Conjugation::Contorqueō,
        Conjugation::Inveniō,
        Conjugation::Sternō,
        Conjugation::Sentiō,
        Conjugation::Fluō,
        Conjugation::Īnsiliō,
        Conjugation::Interstinguō,
        Conjugation::Dēmorior,
        Conjugation::Pervincō,
        Conjugation::Coorior,
        Conjugation::Immisceō,
        Conjugation::Cohaereō,
        Conjugation::Dēpudet,
        Conjugation::Gaudeō,
        Conjugation::Comedō,
        Conjugation::Capiō,
        Conjugation::Maneō,
        Conjugation::Augeō,
        Conjugation::Ferveō,
        Conjugation::Īnsīdō,
        Conjugation::Dēpendeō,
        Conjugation::Peragō,
        Conjugation::Cognōscō,
        Conjugation::Coquō,
        Conjugation::Īnserō,
        Conjugation::Cēnseō,
        Conjugation::Videō,
        Conjugation::Pergō,
        Conjugation::Advīvō,
        Conjugation::Cōnstō,
        Conjugation::Innectō,
        Conjugation::Eximō,
        Conjugation::Vinciō,
        Conjugation::Dēfluō,
        Conjugation::Exsorbeō,
        Conjugation::Cōnfugiō,
        Conjugation::Apīscor,
        Conjugation::Ēdiscō,
        Conjugation::Deicō,
        Conjugation::Scandō,
        Conjugation::Occurrō,
        Conjugation::Sūmō2,
        Conjugation::Vēnumdō,
        Conjugation::Serviō,
        Conjugation::Adipīscor,
        Conjugation::Astō,
        Conjugation::Occinō,
        Conjugation::Decet,
        Conjugation::Supersedeō,
        Conjugation::Clueō,
        Conjugation::Ambigō,
        Conjugation::Contremīscō,
        Conjugation::Colō,
        Conjugation::Clangō,
        Conjugation::Caveō,
        Conjugation::Cieō,
        Conjugation::Patior,
        Conjugation::Fīō,
        Conjugation::Relinquō,
        Conjugation::Sapiō,
        Conjugation::Abeō,
        Conjugation::Ēvellō,
        Conjugation::Currō,
        Conjugation::Nītor,
        Conjugation::Obtundō,
        Conjugation::Hauriō,
        Conjugation::Comperiō,
        Conjugation::Circumvehor,
        Conjugation::Adnītor,
        Conjugation::Extruō,
        Conjugation::Frangō,
        Conjugation::Fricō,
        Conjugation::Mulceō,
        Conjugation::Apage,
        Conjugation::Ēvādō,
        Conjugation::Coniungō,
        Conjugation::Quaerō,
        Conjugation::Adliciō,
        Conjugation::Stō,
        Conjugation::Dīluō,
        Conjugation::Misceō,
        Conjugation::Iubeō,
        Conjugation::Fundō,
        Conjugation::Scatō,
        Conjugation::Explicō,
        Conjugation::Dēfungor,
        Conjugation::Fīdō,
        Conjugation::Libet,
        Conjugation::Coepī,
        Conjugation::Dētergeō,
        Conjugation::Sufferō,
        Conjugation::Antecapiō,
        Conjugation::Commeminī,
        Conjugation::Contemnō,
        Conjugation::Dētorreō,
        Conjugation::Superstō,
        Conjugation::Gignō,
        Conjugation::Assiliō,
        Conjugation::Convalēscō,
        Conjugation::Tuor,
        Conjugation::Adhērēscō,
        Conjugation::Iūxtāpōnō,
        Conjugation::Adlavō,
        Conjugation::Effervēscō,
        Conjugation::Dīlūculat,
        Conjugation::Coerceo,
        Conjugation::Rēfert,
        Conjugation::Coemo,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 200] = [
    // Agitō: 2,609 words (līberō, radiō, digitō, mundō, abaliēnō, picō).
    conjugation!("agitō", 2609, "~ō";
    stem lemma: "~- like agitō";
    stem indicative_act_pres_1_pl: "~- like agitō";
    stem indicative_act_perf_1_sg: "~āv- like agitō";
    stem supine_acc: "~āt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Modulor: 357 words (for, cōnor, minor, māchinor, vōciferor, lupor).
    conjugation!("modulor", 357, "~or";
    stem indicative_act_pres_1_pl: "~- like modulor";
    stem indicative_act_perf_1_sg: "~āt- like modulor";
    stem supine_acc: "~āt- like modulor";
    act_pres_1: "~or", _;
    act_pres_2: "~āris|~āre", _;
    act_pres_3: "~ātur", "~antur";
    act_fut_3: _, "~ābuntur";
    subjunctive_act_pres_1: "~er", "~ēmur";
    subjunctive_act_pres_2: "~ēris|~ēre", "~ēminī";
    subjunctive_act_pres_3: _, "~entur";
    subjunctive_act_impf_1: "~ārer", "~ārēmur";
    subjunctive_act_impf_2: "~ārēris|~ārēre", "~ārēminī";
    subjunctive_act_impf_3: "~ārētur", "~ārentur";
    present_imperative_act: "~āre", _;
    future_imperative_act_3: derived, "~antor";
    infinitive_act: "~ārī", _, _;
    participle: _, _, _, "~andus";
    gerund: _, _, _, _, derived, _;
    ),
    // Sūmō: 265 words (ferō, eō, sūmō, edō, agō, frīgēscō).
    conjugation!("sūmō", 265, "~ō";
    stem lemma: "~- like agō";
    stem indicative_act_pres_1_pl: "~- like agō";
    stem indicative_act_perf_1_sg: "~s- like agitō";
    stem supine_acc: "~t- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Habeō: 256 words (abhorreō, moneō, aveō, oleō, stupeō, egeō).
    conjugation!("habeō", 256, "~eō";
    stem lemma: "~- like habeō";
    stem indicative_act_pres_1_pl: "~- like habeō";
    stem indicative_act_perf_1_sg: "~u- like agitō";
    stem supine_acc: "~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Mūniō: 219 words (fīniō, hirriō, bombiō, poliō, audiō, odiō).
    conjugation!("mūniō", 219, "~iō";
    stem lemma: "~- like mūniō";
    stem indicative_act_pres_1_pl: "~- like mūniō";
    stem indicative_act_perf_1_sg: "~īv- like mūniō";
    stem supine_acc: "~īt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Quiēscō: 152 words (albēscō, grandēscō, siccēscō, alēscō, mānēscō, cornēscō).
    conjugation!("quiēscō", 152, "~scō";
    stem lemma: "~sc- like agō";
    stem indicative_act_pres_1_pl: "~sc- like agō";
    stem indicative_act_perf_1_sg: "~v- like mūniō";
    stem supine_acc: "~t- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Compercō: 116 words (crūdēscō, vigēscō, cānēscō, senēscō, dulcēscō, tumēscō).
    conjugation!("compercō", 116, "~cō";
    stem lemma: "~c- like compercō";
    stem indicative_act_pres_1_pl: "~c- like agō";
    stem indicative_act_perf_1_sg: "~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Minuō: 62 words (futuō, spuō, luō, cōnfutuō, acuō, exuō).
    conjugation!("minuō", 62, "~uō";
    stem lemma: "~u- like minuō";
    stem indicative_act_pres_1_pl: "~u- like agō";
    stem indicative_act_perf_1_sg: "~u- like agitō";
    stem supine_acc: "~ūt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Olfaciō: 53 words (faciō, patefaciō, satisfaciō, candefaciō, abolēfaciō, labefaciō).
    conjugation!("olfaciō", 53, "~aciō";
    stem lemma: "~- like faciō";
    stem indicative_act_pres_1_pl: "~aci- like faciō";
    stem indicative_act_perf_1_sg: "~ēc- like agitō";
    stem supine_acc: "~act- like agitō";
    future_imperative_act_3: derived, _;
    future_imperative_pass_2: _, "~ītōte";
    future_imperative_pass_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Comprehendō: 49 words (dēfendō, comprehendō, fendō, dēpendō, pendō, suspendō).
    conjugation!("comprehendō", 49, "~e~dō";
    stem lemma: "~e~d- like compercō";
    stem indicative_act_pres_1_pl: "~e~d- like agō";
    stem indicative_act_perf_1_sg: "~e~d- like agitō";
    stem supine_acc: "~ē~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Excipiō: 44 words (praeficiō, accipiō, excipiō, recipiō, suscipiō, prōficiō).
    conjugation!("excipiō", 44, "~i~iō";
    stem lemma: "~i~- like excipiō";
    stem indicative_act_pres_1_pl: "~i~i- like faciō";
    stem indicative_act_perf_1_sg: "~ē~- like agitō";
    stem supine_acc: "~e~t- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Teneō: 41 words (abarceō, doceō, sorbeō, teneō, taedeō, absorbeō). Like Habeō.
    conjugation!("teneō", 41, "~eō", like Habeō;
    stem supine_acc: "~t- like agitō";
    ),
    // Cubō: 39 words (vetō, domō, incubō, tonō, votō, cubō). Like Agitō, except in 5 cells: pass_pres_2 sg, pass_impf_2 sg, pass_fut_2 sg, subjunctive_pass_pres_2 sg, subjunctive_pass_impf_2 sg.
    conjugation!("cubō", 39, "~ō", like Agitō;
    stem indicative_act_perf_1_sg: "~u- like agitō";
    stem supine_acc: "~it- like agitō";
    pass_pres_2: "~āris", _;
    pass_impf_2: "~ābāris", _;
    pass_fut_2: "~āberis", _;
    subjunctive_pass_pres_2: "~ēris", _;
    subjunctive_pass_impf_2: "~ārēris", _;
    ),
    // Compescō: 38 words (alō, volō, mālō, molō, parcō, tremō). Like Sūmō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("compescō", 38, "~ō", like Sūmō;
    stem lemma: "~- like compercō";
    stem indicative_act_perf_1_sg: "~u- like agitō";
    stem supine_acc: "~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Prōdō: 34 words (abdō, vēndō, dēdō, bibō, dīdō, condō). Like Compescō.
    conjugation!("prōdō", 34, "~ō", like Compescō;
    stem indicative_act_perf_1_sg: "~id- like agitō";
    ),
    // Circumscrībō: 31 words (prōscrībō, bubō, nūbō, scrībō, dēscrībō, trānscrībō).
    conjugation!("circumscrībō", 31, "~bō";
    stem lemma: "~b- like compercō";
    stem indicative_act_pres_1_pl: "~b- like agō";
    stem indicative_act_perf_1_sg: "~ps- like agitō";
    stem supine_acc: "~pt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Petō: 29 words (petō, impetō, competō, rūdō, ābaetō, appetō). Like Compescō.
    conjugation!("petō", 29, "~ō", like Compescō;
    stem indicative_act_perf_1_sg: "~īv- like mūniō";
    stem supine_acc: "~īt- like agitō";
    ),
    // Vertō: 28 words (vortō, vertō, ābītō, bētō, invertō, convertō).
    conjugation!("vertō", 28, "~tō";
    stem lemma: "~t- like compercō";
    stem indicative_act_pres_1_pl: "~t- like agō";
    stem indicative_act_perf_1_sg: "~t- like agitō";
    stem supine_acc: "~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Mittō: 28 words (mittō, prōmittō, āmittō, committō, admittō, remittō).
    conjugation!("mittō", 28, "~ittō";
    stem lemma: "~itt- like compercō";
    stem indicative_act_pres_1_pl: "~itt- like agō";
    stem indicative_act_perf_1_sg: "~īs- like agitō";
    stem supine_acc: "~iss- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Impleō: 28 words (dēleō, expleō, neō, fleō, adimpleō, compleō). Like Habeō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("impleō", 28, "~eō", like Habeō;
    stem lemma: "~- like impleō";
    stem indicative_act_perf_1_sg: "~ēv- like mūniō";
    stem supine_acc: "~ēt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Labōrō: 28 words (abambulō, labōrō, abortō, cursō, abequitō, lāpsō). Like Agitō, except in 26 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("labōrō", 28, "~ō", like Agitō;
    stem supine_acc: "~āt- like labōrō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~ātor", _;
    infinitive_act: "~āre", _, _;
    participle: _, _, _, "~andum";
    ),
    // Dēfīgō: 28 words (frīgō, fīgō, sūgō, discingō, succingō, praefīgō).
    conjugation!("dēfīgō", 28, "~gō";
    stem lemma: "~g- like compercō";
    stem indicative_act_pres_1_pl: "~g- like agō";
    stem indicative_act_perf_1_sg: "~x- like agitō";
    stem supine_acc: "~ct- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Perstringō: 27 words (cōnfingō, dēpingō, effingō, fingō, pingō, stringō).
    conjugation!("perstringō", 27, "~ingō";
    stem lemma: "~ing- like compercō";
    stem indicative_act_pres_1_pl: "~ing- like agō";
    stem indicative_act_perf_1_sg: "~īnx- like agitō";
    stem supine_acc: "~ict- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Obeō: 27 words (adeō, dispereō, exeō, queō, ineō, intereō).
    conjugation!("obeō", 27, "~eō";
    stem lemma: "~- like obeō";
    stem indicative_act_pres_1_pl: "~- like obeō";
    stem indicative_act_perf_1_sg: "~- like obeō";
    stem supine_acc: "~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Mōlior: 27 words (dēmōlior, mōlior, mentior, partior, ōrdior, blandior). Like Modulor, except in 22 cells: act_pres_1 sg, act_pres_2 sg, act_pres_3 sg, act_pres_3 pl, act_fut_3 pl, subjunctive_act_pres_1 sg, ….
    conjugation!("mōlior", 27, "~ior", like Modulor;
    stem indicative_act_pres_1_pl: "~- like mōlior";
    stem indicative_act_perf_1_sg: "~īt- like modulor";
    stem supine_acc: "~īt- like modulor";
    act_pres_1: "~ior", _;
    act_pres_2: "~īris|~īre", _;
    act_pres_3: "~ītur", "~iuntur";
    act_fut_3: _, "~ientur";
    subjunctive_act_pres_1: "~iar", "~iāmur";
    subjunctive_act_pres_2: "~iāris|~iāre", "~iāminī";
    subjunctive_act_pres_3: _, "~iantur";
    subjunctive_act_impf_1: "~īrer", "~īrēmur";
    subjunctive_act_impf_2: "~īrēris|~īrēre", "~īrēminī";
    subjunctive_act_impf_3: "~īrētur", "~īrentur";
    present_imperative_act: "~īre", _;
    future_imperative_act_3: derived, "~iuntor";
    infinitive_act: "~īrī", _, _;
    participle: _, _, _, "~iendus|~iundus";
    gerund: _, _, _, _, derived, _;
    ),
    // Dēdūcō: 26 words (abdūcō, trādūcō, prōdūcō, condūcō, ēdūcō, dēdūcō).
    conjugation!("dēdūcō", 26, "~ūcō";
    stem lemma: "~ūc- like dēdūcō";
    stem indicative_act_pres_1_pl: "~ūc- like agō";
    stem indicative_act_perf_1_sg: "~ūx- like agitō";
    stem supine_acc: "~uct- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Moveō: 26 words (foveō, moveō, voveō, removeō, admoveō, āmoveō).
    conjugation!("moveō", 26, "~oveō";
    stem lemma: "~ov- like impleō";
    stem indicative_act_pres_1_pl: "~ov- like habeō";
    stem indicative_act_perf_1_sg: "~ōv- like agitō";
    stem supine_acc: "~ōt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Repōnō: 26 words (pōnō, expōnō, oppōnō, antepōnō, prōpōnō, dēpōnō).
    conjugation!("repōnō", 26, "~ōnō";
    stem lemma: "~ōn- like compercō";
    stem indicative_act_pres_1_pl: "~ōn- like agō";
    stem indicative_act_perf_1_sg: "~os- like repōnō";
    pass_perf_1: "~ostus sum", _;
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, "~ositūrum esse|~ostūrum esse";
    infinitive_pass: _, "~ositum esse|~ostum esse", "~ositum īrī|~ostum īrī";
    participle: _, "~ositus|~ostus", "~ositūrus|~ostūrus", _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, "~ositum|~ostum", "~ositū|~ostū", _;
    ),
    // Mergō: 25 words (mergō, dēmergō, immergō, ēmergō, spargō, cōnspargō). Like Dēfīgō.
    conjugation!("mergō", 25, "~gō", like Dēfīgō;
    stem indicative_act_perf_1_sg: "~s- like agitō";
    stem supine_acc: "~s- like agitō";
    ),
    // Compungō: 25 words (compungō, interpungō, iungō, abiungō, adiungō, sēiungō). Like Dēfīgō with `ˇ` after each pattern.
    conjugation!("compungō", 25, "~gōˇ", like Dēfīgō with "ˇ";
    ),
    // Dēpellō: 24 words (pellō, vellō, appellō, dēpellō, prōpellō, repellō).
    conjugation!("dēpellō", 24, "~e~lō";
    stem lemma: "~e~l- like compercō";
    stem indicative_act_pres_1_pl: "~e~l- like agō";
    stem indicative_act_perf_1_sg: "~u~- like agitō";
    stem supine_acc: "~u~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Tueor: 23 words (polliceor, fateor, cōnfiteor, vereor, reor, intueor). Like Modulor, except in 22 cells: act_pres_1 sg, act_pres_2 sg, act_pres_3 sg, act_pres_3 pl, act_fut_3 pl, subjunctive_act_pres_1 sg, ….
    conjugation!("tueor", 23, "~eor", like Modulor;
    stem indicative_act_pres_1_pl: "~- like tueor";
    stem indicative_act_perf_1_sg: "~it- like modulor";
    stem supine_acc: "~it- like modulor";
    act_pres_1: "~eor", _;
    act_pres_2: "~ēris|~ēre", _;
    act_pres_3: "~ētur", "~entur";
    act_fut_3: _, "~ēbuntur";
    subjunctive_act_pres_1: "~ear", "~eāmur";
    subjunctive_act_pres_2: "~eāris|~eāre", "~eāminī";
    subjunctive_act_pres_3: _, "~eantur";
    subjunctive_act_impf_1: "~ērer", "~ērēmur";
    subjunctive_act_impf_2: "~ērēris|~ērēre", "~ērēminī";
    subjunctive_act_impf_3: "~ērētur", "~ērentur";
    present_imperative_act: "~ēre", _;
    future_imperative_act_3: derived, "~entor";
    infinitive_act: "~ērī", _, _;
    participle: _, _, _, "~endus";
    gerund: _, _, _, _, derived, _;
    ),
    // Exsarciō: 22 words (crociō, saciō, farciō, micciō, effarciō, sarciō). Like Sūmō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("exsarciō", 22, "~ciō", like Sūmō;
    stem lemma: "~c- like mūniō";
    stem indicative_act_pres_1_pl: "~c- like mūniō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Praecīdō: 22 words (sīdō, comprēndō, expandō, prēndō, reprēndō, apprēndō).
    conjugation!("praecīdō", 22, "~dō";
    stem lemma: "~d- like compercō";
    stem indicative_act_pres_1_pl: "~d- like agō";
    stem indicative_act_perf_1_sg: "~d- like agitō";
    stem supine_acc: "~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Solvō: 21 words (volvō, solvō, ēvolvō, convolvō, dissolvō, resolvō).
    conjugation!("solvō", 21, "~vō";
    stem lemma: "~v- like minuō";
    stem indicative_act_pres_1_pl: "~v- like agō";
    stem indicative_act_perf_1_sg: "~v- like agitō";
    stem supine_acc: "~ūt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Pāscor: 21 words (ulcīscor, īrāscor, nāscor, vēscor, aborīscor, oblīvīscor). Like Modulor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("pāscor", 21, "~scor", like Modulor;
    stem lemma: "~sc- like nāscor";
    stem indicative_act_pres_1_pl: "~sc- like nāscor";
    stem indicative_act_perf_1_sg: "~t- like modulor";
    stem supine_acc: "~t- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Coniciō: 21 words (rabiō, iniciō, abiciō, subiciō, adiciō, circumiciō). Like Excipiō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("coniciō", 21, "~~iō", like Excipiō;
    stem lemma: "~~- like excipiō";
    stem indicative_act_pres_1_pl: "~~i- like faciō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Nuō: 20 words (nuō, sinō, minuō, innuō, annuō, dēsinō). Like Vīsō with `ˇ` after each pattern, except in 5 cells: pass_perf_1 sg, infinitive_act fut, infinitive_pass perf, participle perf_pass, participle fut_pass.
    conjugation!("nuō", 20, "~ōˇ", like Vīsō with "ˇ";
    pass_perf_1: "~tus sumˇ", _;
    infinitive_act: _, _, "~tūrum esseˇ";
    infinitive_pass: _, "~tum esseˇ", _;
    participle: _, "~tusˇ", _, "~endusˇ";
    ),
    // Cēdō: 19 words (cēdō, discēdō, concēdō, excēdō, prōcēdō, retrōcēdō).
    conjugation!("cēdō", 19, "~ēdō";
    stem lemma: "~ēd- like compercō";
    stem indicative_act_pres_1_pl: "~ēd- like agō";
    stem indicative_act_perf_1_sg: "~ess- like agitō";
    stem supine_acc: "~ess- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ēgredior: 19 words (ēgredior, gradior, aggredior, trānsgredior, antegredior, congredior). Like Modulor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("ēgredior", 19, "~dior", like Modulor;
    stem lemma: "~d- like ēgredior";
    stem indicative_act_pres_1_pl: "~di- like ēgredior";
    stem indicative_act_perf_1_sg: "~ss- like modulor";
    stem supine_acc: "~ss- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Prōferō: 18 words (trānsferō, praeferō, īnferō, adferō, auferō, cōnferō).
    conjugation!("prōferō", 18, "~ferō";
    stem lemma: "~fer- like referō";
    stem indicative_act_pres_1_pl: "~fer- like referō";
    stem indicative_act_perf_1_sg: "~tul- like agitō";
    stem supine_acc: "~lāt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Gerō: 17 words (gerō, ēgerō, aggerō, adgerō, regerō, suggerō).
    conjugation!("gerō", 17, "~rō";
    stem lemma: "~r- like compercō";
    stem indicative_act_pres_1_pl: "~r- like agō";
    stem indicative_act_perf_1_sg: "~ss- like agitō";
    stem supine_acc: "~st- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Nectō: 17 words (flectō, īnflectō, genūflectō, reflectō, pectō, plectō).
    conjugation!("nectō", 17, "~ctō";
    stem lemma: "~ct- like compercō";
    stem indicative_act_pres_1_pl: "~ct- like agō";
    stem indicative_act_perf_1_sg: "~x- like agitō";
    stem supine_acc: "~x- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dispiciō: 17 words (perspiciō, retrōspiciō, aspiciō, respiciō, cōnspiciō, suspiciō).
    conjugation!("dispiciō", 17, "~iciō";
    stem lemma: "~ic- like excipiō";
    stem indicative_act_pres_1_pl: "~ici- like faciō";
    stem indicative_act_perf_1_sg: "~ex- like agitō";
    stem supine_acc: "~ect- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Trahō: 17 words (extrahō, trahō, abstrahō, subtrahō, attrahō, circumtrahō).
    conjugation!("trahō", 17, "~ahō";
    stem lemma: "~ah- like compercō";
    stem indicative_act_pres_1_pl: "~ah- like agō";
    stem indicative_act_perf_1_sg: "~āx- like agitō";
    stem supine_acc: "~act- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Compingō: 17 words (attingō, contingō, impingō, adtingō, compingō, cōnfringō). Like Perstringō.
    conjugation!("compingō", 17, "~ingō", like Perstringō;
    stem indicative_act_perf_1_sg: "~ēg- like agitō";
    stem supine_acc: "~āct- like agitō";
    ),
    // Exigō: 17 words (adigō, redigō, prōdigō, abigō, trānsigō, colligō). Like Compingō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("exigō", 17, "~igō", like Compingō;
    stem lemma: "~ig- like compercō";
    stem indicative_act_pres_1_pl: "~ig- like agō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Effundō: 16 words (profundō, cōnfundō, īnfundō, tundō, effundō, circumfundō).
    conjugation!("effundō", 16, "~undō";
    stem lemma: "~und- like compercō";
    stem indicative_act_pres_1_pl: "~und- like agō";
    stem indicative_act_perf_1_sg: "~ūd- like agitō";
    stem supine_acc: "~ūs- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Corripiō: 16 words (ēripiō, abripiō, dēripiō, alliciō, arripiō, dīripiō). Like Excipiō.
    conjugation!("corripiō", 16, "~i~iō", like Excipiō;
    stem indicative_act_perf_1_sg: "~i~u- like agitō";
    ),
    // Accumbō: 16 words (recumbō, dēlambō, accumbō, adcumbō, succumbō, prōcumbō).
    conjugation!("accumbō", 16, "~m~ō";
    stem lemma: "~m~- like compercō";
    stem indicative_act_pres_1_pl: "~m~- like agō";
    stem indicative_act_perf_1_sg: "~~u- like agitō";
    stem supine_acc: "~~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Intersaepiō: 16 words (apiō, saepiō, speciō, sēpiō, reperiō, praesaepiō). Like Mūniō.
    conjugation!("intersaepiō", 16, "~iō", like Mūniō;
    stem indicative_act_perf_1_sg: "~s- like agitō";
    stem supine_acc: "~t- like agitō";
    ),
    // Ārdeō: 15 words (audeō, ārdeō, rīdeō, persuādeō, subrīdeō, suādeō). Like Mergō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("ārdeō", 15, "~deō", like Mergō;
    stem lemma: "~d- like impleō";
    stem indicative_act_pres_1_pl: "~d- like habeō";
    stem indicative_act_perf_1_sg: "~s- like mūniō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ērigō: 15 words (dīrigō, ērigō, dīligō, intelligō, porrigō, negligō). Like Exigō.
    conjugation!("ērigō", 15, "~igō", like Exigō;
    stem indicative_act_perf_1_sg: "~ēx- like agitō";
    stem supine_acc: "~ēct- like agitō";
    ),
    // Dēsinō: 14 words (linō, oblinō, collinō, perlinō, circumlinō, succinō).
    conjugation!("dēsinō", 14, "~inō";
    stem lemma: "~in- like compercō";
    stem indicative_act_pres_1_pl: "~in- like agō";
    stem indicative_act_perf_1_sg: "~ēv- like agitō";
    stem supine_acc: "~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dīcō: 14 words (abdīcō, dīcō, addīcō, interdīcō, maledīcō, benedīcō).
    conjugation!("dīcō", 14, "~īcō";
    stem lemma: "~īc- like dēdūcō";
    stem indicative_act_pres_1_pl: "~īc- like agō";
    stem indicative_act_perf_1_sg: "~īx- like agitō";
    stem supine_acc: "~ict- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Tingō: 14 words (lingō, cingō, accingō, circumcingō, incingō, recingō).
    conjugation!("tingō", 14, "~i~gō";
    stem lemma: "~i~g- like compercō";
    stem indicative_act_pres_1_pl: "~i~g- like agō";
    stem indicative_act_perf_1_sg: "~ī~x- like agitō";
    stem supine_acc: "~ī~ct- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Regō: 14 words (regō, neglegō, tegō, dētegō, intellegō, prōtegō). Like Ērigō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("regō", 14, "~egō", like Ērigō;
    stem lemma: "~eg- like compercō";
    stem indicative_act_pres_1_pl: "~eg- like agō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Struō: 14 words (dēstruō, struō, cōnstruō, īnstruō, exstruō, obstruō). Like Minuō.
    conjugation!("struō", 14, "~uō", like Minuō;
    stem indicative_act_perf_1_sg: "~ūx- like agitō";
    stem supine_acc: "~ūct- like agitō";
    ),
    // Rumpō: 14 words (irrumpō, interrumpō, abrumpō, rumpō, ērumpō, corrumpō).
    conjugation!("rumpō", 14, "~um~ō";
    stem lemma: "~um~- like compercō";
    stem indicative_act_pres_1_pl: "~um~- like agō";
    stem indicative_act_perf_1_sg: "~ū~- like agitō";
    stem supine_acc: "~u~t- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fodiō: 14 words (fodiō, cōnfodiō, affodiō, dēfodiō, effodiō, īnfodiō).
    conjugation!("fodiō", 14, "~odiō";
    stem lemma: "~od- like excipiō";
    stem indicative_act_pres_1_pl: "~odi- like faciō";
    stem indicative_act_perf_1_sg: "~ōd- like agitō";
    stem supine_acc: "~oss- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Asserviō: 14 words (veniō, reveniō, interveniō, perveniō, adveniō, ēveniō).
    conjugation!("asserviō", 14, "~e~iō";
    stem lemma: "~e~- like ēveniō";
    stem indicative_act_pres_1_pl: "~e~- like mūniō";
    stem indicative_act_perf_1_sg: "~ē~- like agitō";
    stem supine_acc: "~e~t- like ēveniō";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Autumnāscit: 14 words (vesperāscit, cōnfit, pluit, advesperāscit, ningit, compluit).
    conjugation!("autumnāscit", 14, "~it";
    act_pres_1: "~it", _;
    act_pres_3: "~it", _;
    act_impf_3: "~ēbat", _;
    act_fut_3: "~et", _;
    pass_pres_3: "~itur", _;
    pass_impf_3: "~ēbātur", _;
    pass_fut_3: "~ētur", _;
    subjunctive_act_pres_3: "~at", _;
    subjunctive_act_impf_3: "~eret", _;
    subjunctive_pass_pres_3: "~ātur", _;
    subjunctive_pass_impf_3: "~erētur", _;
    future_imperative_act_3: "~itō", _;
    future_imperative_pass_3: "~itor", _;
    infinitive_act: "~ere", _, _;
    infinitive_pass: "~ī", _, _;
    participle: "~ēns", _, _, "~endum";
    gerund: _, "~endī", "~endō", "~endum", _, _;
    ),
    // Illābor: 13 words (lābor, collābor, dēlābor, conlābor, dīlābor, ēlābor). Like Modulor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("illābor", 13, "~bor", like Modulor;
    stem lemma: "~b- like nāscor";
    stem indicative_act_pres_1_pl: "~b- like nāscor";
    stem indicative_act_perf_1_sg: "~ps- like modulor";
    stem supine_acc: "~ps- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ēvehō: 13 words (vehō, advehō, āvehō, convehō, dēvehō, prōvehō).
    conjugation!("ēvehō", 13, "~ehō";
    stem lemma: "~eh- like compercō";
    stem indicative_act_pres_1_pl: "~eh- like agō";
    stem indicative_act_perf_1_sg: "~ēx- like agitō";
    stem supine_acc: "~ect- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cōnsequor: 13 words (sequor, līquor, prōsequor, persequor, adsequor, assequor). Like Modulor, except in 6 cells: future_imperative_act_3 sg, infinitive_act fut, participle fut_act, gerund abl, supine acc, supine abl.
    conjugation!("cōnsequor", 13, "~quor", like Modulor;
    stem lemma: "~qu- like cōnsequor";
    stem indicative_act_pres_1_pl: "~qu- like nāscor";
    stem indicative_act_perf_1_sg: "~- like cōnsequor";
    future_imperative_act_3: derived, _;
    infinitive_act: _, _, "~cūtūrum esse|~quūtūrum esse";
    participle: _, _, "~cūtūrus|~quūtūrus", _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, "~cūtum|~quūtum", "~cūtū|~quūtū", _;
    ),
    // Madeō: 13 words (studeō, pāreō, horreō, vegeō, sileō, affleō). Like Habeō, except in 23 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("madeō", 13, "~eō", like Habeō;
    stem lemma: "~- like madeō";
    stem supine_acc: "~it- like labōrō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    ),
    // Excellō: 13 words (fallō, excellō, praecellō, extollō, attollō, adtollō).
    conjugation!("excellō", 13, "~lō";
    stem lemma: "~l- like agō";
    stem indicative_act_pres_1_pl: "~l- like agō";
    stem indicative_act_perf_1_sg: "~lu- like mūniō";
    stem supine_acc: "~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Scindō: 13 words (rescindō, scindō, abscindō, pandō, interscindō, findō).
    conjugation!("scindō", 13, "~ndō";
    stem lemma: "~nd- like compercō";
    stem indicative_act_pres_1_pl: "~nd- like agō";
    stem indicative_act_perf_1_sg: "~d- like agitō";
    stem supine_acc: "~ss- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Īnsistō: 13 words (cōnsistō, dēsistō, existō, persistō, resistō, subsistō).
    conjugation!("īnsistō", 13, "~~s~ō";
    stem lemma: "~~s~- like īnsistō";
    stem indicative_act_pres_1_pl: "~~s~- like agō";
    stem indicative_act_perf_1_sg: "~t~~- like agitō";
    stem supine_acc: "~t~~- like ēveniō";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Feriō: 12 words (ūniō, feriō, absiliō, ambiō, salliō, lascīviō). Like Mūniō.
    conjugation!("feriō", 12, "~iō", like Mūniō;
    stem indicative_act_perf_1_sg: "~i- like agitō";
    ),
    // Fugiō: 12 words (fugiō, subterfugiō, effugiō, trānsfugiō, dēfugiō, supterfugiō).
    conjugation!("fugiō", 12, "~u~iō";
    stem lemma: "~u~- like fugiō";
    stem indicative_act_pres_1_pl: "~u~i- like faciō";
    stem indicative_act_perf_1_sg: "~ū~- like agitō";
    stem supine_acc: "~u~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Obloquor: 12 words (loquor, alloquor, circumloquor, colloquor, ēloquor, interloquor). Like Cōnsequor, except in 1 cell: participle fut_act.
    conjugation!("obloquor", 12, "~quor", like Cōnsequor;
    stem indicative_act_perf_1_sg: "~cūt- like modulor";
    stem supine_acc: "~cūt- like modulor";
    participle: _, _, derived, _;
    ),
    // Possideō: 12 words (resideō, assideō, īnsideō, obsideō, adsideō, dēsideō).
    conjugation!("possideō", 12, "~ideō";
    stem lemma: "~id- like habeō";
    stem indicative_act_pres_1_pl: "~id- like habeō";
    stem indicative_act_perf_1_sg: "~ēd- like agitō";
    stem supine_acc: "~ess- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Intersum: 12 words (possum, absum, adsum, īnsum, dēsum, obsum). Like Modulor, except in 28 cells: act_pres_1 sg, act_pres_2 sg, act_pres_3 sg, act_pres_3 pl, act_fut_3 pl, subjunctive_act_pres_1 sg, ….
    conjugation!("intersum", 12, "~sum", like Modulor;
    stem indicative_act_pres_1_pl: "~- like adsum";
    stem indicative_act_perf_1_sg: "~fu- like agitō";
    act_pres_1: "~sum", _;
    act_pres_2: "~es", _;
    act_pres_3: "~est", "~sunt";
    act_fut_3: _, "~erunt";
    subjunctive_act_pres_1: "~sim|~siem|~fuam", "~sīmus|~siēmus|~fuāmus";
    subjunctive_act_pres_2: "~sīs|~siēs|~fuās", "~sītis|~siētis|~fuātis";
    subjunctive_act_pres_3: _, "~sint|~sient|~fuant";
    subjunctive_act_impf_1: "~essem|~forem", "~essēmus|~forēmus";
    subjunctive_act_impf_2: "~essēs|~forēs", "~essētis|~forētis";
    subjunctive_act_impf_3: "~esset|~foret", "~essent|~forent";
    present_imperative_act: "~es", _;
    future_imperative_act_2: _, "~estōte";
    future_imperative_act_3: derived, "~suntō";
    infinitive_act: "~esse", _, "~futūrum esse|~fore";
    participle: _, _, _, derived;
    gerund: _, derived, derived, derived, _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Cōnsulō: 11 words (cōnsulō, incumbō, furō, occulō, dēserō, exserō). Like Compescō.
    conjugation!("cōnsulō", 11, "~ō", like Compescō;
    stem supine_acc: "~t- like agitō";
    ),
    // Contineō: 11 words (contineō, abstineō, dētineō, attineō, distineō, obtineō). Like Corripiō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("contineō", 11, "~i~eō", like Corripiō;
    stem lemma: "~i~- like impleō";
    stem indicative_act_pres_1_pl: "~i~- like habeō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Opprimō: 11 words (dēprimō, exprimō, reprimō, comprimō, imprimō, opprimō). Like Cēdō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("opprimō", 11, "~imō", like Cēdō;
    stem lemma: "~im- like compercō";
    stem indicative_act_pres_1_pl: "~im- like agō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Incidō: 11 words (dēcidō, accidō, coincidō, incidō, intercidō, occidō).
    conjugation!("incidō", 11, "~idō";
    stem lemma: "~id- like agō";
    stem indicative_act_pres_1_pl: "~id- like agō";
    stem indicative_act_perf_1_sg: "~id- like mūniō";
    stem supine_acc: "~ās- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cernō: 11 words (cernō, concernō, sēcernō, discernō, dēcernō, condēcernō).
    conjugation!("cernō", 11, "~e~nō";
    stem lemma: "~e~n- like compercō";
    stem indicative_act_pres_1_pl: "~e~n- like agō";
    stem indicative_act_perf_1_sg: "~~ēv- like agitō";
    stem supine_acc: "~~ēt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Percutiō: 11 words (patiō, recutiō, excutiō, quatiō, discutiō, percutiō).
    conjugation!("percutiō", 11, "~tiō";
    stem lemma: "~t- like excipiō";
    stem indicative_act_pres_1_pl: "~ti- like faciō";
    stem indicative_act_perf_1_sg: "~ss- like agitō";
    stem supine_acc: "~ss- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ūtor: 11 words (revertor, deūtor, ūtor, abūtor, amplector, complector). Like Modulor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("ūtor", 11, "~tor", like Modulor;
    stem lemma: "~t- like nāscor";
    stem indicative_act_pres_1_pl: "~t- like nāscor";
    stem indicative_act_perf_1_sg: "~s- like modulor";
    stem supine_acc: "~s- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Abblandior: 11 words (admētior, dīmētior, remētior, commētior, conmētior, ēmētior). Like Mōlior.
    conjugation!("abblandior", 11, "~ior", like Mōlior;
    stem indicative_act_perf_1_sg: "~it- like modulor";
    stem supine_acc: "~it- like modulor";
    ),
    // Attondeō: 11 words (tondeō, respondeō, dēspondeō, spondeō, attondeō, dētondeō).
    conjugation!("attondeō", 11, "~o~deō";
    stem lemma: "~o~d- like impleō";
    stem indicative_act_pres_1_pl: "~o~d- like habeō";
    stem indicative_act_perf_1_sg: "~o~d- like agitō";
    stem supine_acc: "~ō~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Micō: 11 words (sonō, personō, plicō, intonō, increpō, affricō). Like Cubō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("micō", 11, "~ō", like Cubō;
    stem lemma: "~- like agitō";
    stem supine_acc: "~āt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Calvor: 11 words (queror, fruor, conqueror, locor, calvor, perdor). Like Tueor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("calvor", 11, "~or", like Tueor;
    stem lemma: "~- like cōnsequor";
    stem indicative_act_pres_1_pl: "~- like nāscor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Tergeō: 10 words (abstergeō, mulgeō, urgeō, perurgeō, extergeō, tergeō). Like Mergō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("tergeō", 10, "~geō", like Mergō;
    stem lemma: "~g- like habeō";
    stem indicative_act_pres_1_pl: "~g- like habeō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Legō: 10 words (legō, allegō, relegō, perlegō, praelegō, sublegō). Like Regō.
    conjugation!("legō", 10, "~egō", like Regō;
    stem indicative_act_perf_1_sg: "~ēg- like agitō";
    ),
    // Terō: 10 words (terō, interō, dēterō, atterō, exterō, prōterō).
    conjugation!("terō", 10, "~e~ō";
    stem lemma: "~e~- like compercō";
    stem indicative_act_pres_1_pl: "~e~- like agō";
    stem indicative_act_perf_1_sg: "~~īv- like agitō";
    stem supine_acc: "~~īt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Contendō: 10 words (ostendō, contendō, attendō, intendō, portendō, distendō). Like Praecīdō.
    conjugation!("contendō", 10, "~dō", like Praecīdō;
    stem supine_acc: "~t- like agitō";
    ),
    // Vīsō: 10 words (revīsō, vīsō, invīsō, incessō, praevertō, intervīsō). Like Compescō.
    conjugation!("vīsō", 10, "~ō", like Compescō;
    stem indicative_act_perf_1_sg: "~- like agitō";
    stem supine_acc: "~- like agitō";
    ),
    // Abnuō: 10 words (ajō, uō, īcō, fervō, abscondō, abnuō). Like Vīsō.
    conjugation!("abnuō", 10, "~ō", like Vīsō;
    stem supine_acc: "~it- like agitō";
    ),
    // Aperiō: 10 words (aperiō, rapiō, operiō, saliō, adaperiō, cooperiō). Like Intersaepiō.
    conjugation!("aperiō", 10, "~iō", like Intersaepiō;
    stem indicative_act_perf_1_sg: "~u- like agitō";
    ),
    // Contorqueō: 9 words (torqueō, extorqueō, dētorqueō, contorqueō, distorqueō, intorqueō). Like Sūmō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("contorqueō", 9, "~queō", like Sūmō;
    stem lemma: "~qu- like impleō";
    stem indicative_act_pres_1_pl: "~qu- like habeō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Inveniō: 9 words (inveniō, conveniō, circumveniō, contrāveniō, dēveniō, praeveniō). Like Asserviō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("inveniō", 9, "~e~iō", like Asserviō;
    stem lemma: "~e~- like mūniō";
    stem supine_acc: "~e~t- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Sternō: 9 words (sternō, cōnsternō, asternō, prōsternō, substernō, dēsternō). Like Cernō.
    conjugation!("sternō", 9, "~e~nō", like Cernō;
    stem indicative_act_perf_1_sg: "~~āv- like agitō";
    stem supine_acc: "~~āt- like agitō";
    ),
    // Sentiō: 9 words (sentiō, cōnsentiō, dissentiō, praesentiō, assentiō, adsentiō).
    conjugation!("sentiō", 9, "~e~tiō";
    stem lemma: "~e~t- like mūniō";
    stem indicative_act_pres_1_pl: "~e~t- like mūniō";
    stem indicative_act_perf_1_sg: "~ē~s- like agitō";
    stem supine_acc: "~ē~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fluō: 9 words (fluō, superfluō, circumfluō, cōnfluō, diffluō, interfluō). Like Struō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("fluō", 9, "~uō", like Struō;
    stem lemma: "~u- like agō";
    stem supine_acc: "~ūx- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Īnsiliō: 9 words (exiliō, trānsiliō, dēsiliō, exsiliō, īnsiliō, subsiliō). Like Corripiō, except in 28 cells: act_pres_2 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_pres_3 sg, pass_impf_2 sg, ….
    conjugation!("īnsiliō", 9, "~i~iō", like Corripiō;
    stem indicative_act_pres_1_pl: "~i~- like mūniō";
    stem indicative_act_perf_1_sg: "~i~u- like mūniō";
    stem supine_acc: "~u~t- like agitō";
    act_pres_2: "~i~īs", _;
    pass_pres_1: _, "~i~īmur";
    pass_pres_2: "~i~īris", "~i~īminī";
    pass_pres_3: "~i~ītur", _;
    pass_impf_2: "~i~iēbāris", _;
    pass_fut_2: "~i~iēris", _;
    subjunctive_act_impf_1: "~i~īrem", "~i~īrēmus";
    subjunctive_act_impf_2: "~i~īrēs", "~i~īrētis";
    subjunctive_act_impf_3: "~i~īret", "~i~īrent";
    subjunctive_pass_pres_2: "~i~iāris", _;
    subjunctive_pass_impf_1: "~i~īrer", "~i~īrēmur";
    subjunctive_pass_impf_2: "~i~īrēris", "~i~īrēminī";
    subjunctive_pass_impf_3: "~i~īrētur", "~i~īrentur";
    present_imperative_act: "~i~ī", _;
    present_imperative_pass: "~i~īre", _;
    future_imperative_act_2: _, "~i~ītōte";
    future_imperative_act_3: derived, _;
    future_imperative_pass_2: "~i~ītor", _;
    infinitive_pass: "~i~īrī", _, _;
    participle: _, _, _, "~i~iendus";
    gerund: _, _, _, _, derived, _;
    ),
    // Interstinguō: 9 words (distinguō, stinguō, extinguō, exstinguō, restinguō, intinguō). Like Tingō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("interstinguō", 9, "~i~guō", like Tingō;
    stem lemma: "~i~gu- like minuō";
    stem indicative_act_pres_1_pl: "~i~gu- like agō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dēmorior: 9 words (morior, ēmorior, commorior, conmorior, intermorior, dēmorior). Like Abblandior, except in 4 cells: future_imperative_act_3 sg, gerund abl, supine acc, supine abl.
    conjugation!("dēmorior", 9, "~ior", like Abblandior;
    stem lemma: "~- like ēgredior";
    stem indicative_act_pres_1_pl: "~i- like ēgredior";
    stem indicative_act_perf_1_sg: "~tu- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, "~tuum", "~tuū", _;
    ),
    // Pervincō: 8 words (vincō, convincō, ēvincō, dēvincō, prōvincō, revincō).
    conjugation!("pervincō", 8, "~in~ō";
    stem lemma: "~in~- like compercō";
    stem indicative_act_pres_1_pl: "~in~- like agō";
    stem indicative_act_perf_1_sg: "~ī~- like agitō";
    stem supine_acc: "~i~t- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Coorior: 8 words (aborior, orior, experior, adorior, coorior, exorior). Like Mōlior.
    conjugation!("coorior", 8, "~ior", like Mōlior;
    stem indicative_act_perf_1_sg: "~t- like modulor";
    stem supine_acc: "~t- like modulor";
    ),
    // Immisceō: 8 words (admisceō, commisceō, immisceō, intermisceō, ammisceō, ēmisceō).
    conjugation!("immisceō", 8, "~sceō";
    stem lemma: "~sc- like impleō";
    stem indicative_act_pres_1_pl: "~sc- like habeō";
    stem indicative_act_perf_1_sg: "~scu- like agitō";
    stem supine_acc: "~xt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cohaereō: 8 words (adhaereō, haereō, adhēreō, cohaereō, inhaereō, cohēreō). Like Mergō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("cohaereō", 8, "~reō", like Mergō;
    stem lemma: "~r- like habeō";
    stem indicative_act_pres_1_pl: "~r- like habeō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dēpudet: 8 words (miseret, oportet, taedet, piget, pertaedet, dēpudet). Like Autumnāscit, except in 13 cells: act_pres_1 sg, act_pres_3 sg, act_fut_3 sg, pass_pres_3 sg, pass_fut_3 sg, subjunctive_act_pres_3 sg, ….
    conjugation!("dēpudet", 8, "~et", like Autumnāscit;
    stem indicative_act_perf_1_sg: "~u- like dēpudet";
    stem supine_acc: "~it- like dēpudet";
    act_pres_1: "~et", _;
    act_pres_3: "~et", _;
    act_fut_3: "~ēbit", _;
    pass_pres_3: "~ētur", _;
    pass_fut_3: "~ēbitur", _;
    subjunctive_act_pres_3: "~eat", _;
    subjunctive_act_impf_3: "~ēret", _;
    subjunctive_pass_pres_3: "~eātur", _;
    subjunctive_pass_impf_3: "~ērētur", _;
    future_imperative_act_3: "~ētō", _;
    future_imperative_pass_3: "~ētor", _;
    infinitive_act: "~ēre", _, _;
    infinitive_pass: "~ērī", _, _;
    ),
    // Gaudeō: 7 words (gaudeō, aggaudeō, adgaudeō, congaudeō, pergaudeō, praegaudeō). Like Modulor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("gaudeō", 7, "~audeō", like Modulor;
    stem lemma: "~aud- like gaudeō";
    stem indicative_act_pres_1_pl: "~aud- like habeō";
    stem indicative_act_perf_1_sg: "~āvīs- like modulor";
    stem supine_acc: "~āvīs- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Comedō: 7 words (comedō, ambedō, adedō, exedō, peredō, subedō).
    conjugation!("comedō", 7, "~edō";
    stem lemma: "~- like comedō";
    stem indicative_act_perf_1_sg: "~ēd- like agitō";
    stem supine_acc: "~ēs- like comedō";
    act_pres_1: _, "~edimus";
    act_pres_2: _, "~editis|~ēstis";
    act_impf_1: "~edēbam", "~edēbāmus";
    act_impf_2: "~edēbās", "~edēbātis";
    act_impf_3: "~edēbat", "~edēbant";
    act_fut_1: "~edam", "~edēmus";
    act_fut_2: "~edēs", "~edētis";
    act_fut_3: "~edet", _;
    subjunctive_act_pres_3: "~edat|~edit|~ēst", _;
    present_imperative_act: _, "~edite|~ēste";
    present_imperative_pass: _, derived;
    future_imperative_act_2: "~editō|~ēstō", _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    participle: "~edēns", _, _, _;
    gerund: _, "~edendī", "~edendō", "~edendum", _, _;
    ),
    // Capiō: 7 words (ūsūcapiō, iaciō, laciō, pariō, capiō, cōnsuēfaciō).
    conjugation!("capiō", 7, "~a~iō";
    stem lemma: "~a~- like excipiō";
    stem indicative_act_pres_1_pl: "~a~i- like faciō";
    stem indicative_act_perf_1_sg: "~ē~- like agitō";
    stem supine_acc: "~a~t- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Maneō: 7 words (remaneō, maneō, permaneō, commaneō, ēmaneō, immaneō).
    conjugation!("maneō", 7, "~a~eō";
    stem lemma: "~a~- like habeō";
    stem indicative_act_pres_1_pl: "~a~- like habeō";
    stem indicative_act_perf_1_sg: "~ā~s- like agitō";
    stem supine_acc: "~ā~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Augeō: 7 words (augeō, lūgeō, adaugeō, ēlūgeō, frīgeō, exaugeō). Like Tergeō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("augeō", 7, "~geō", like Tergeō;
    stem lemma: "~g- like impleō";
    stem indicative_act_perf_1_sg: "~x- like agitō";
    stem supine_acc: "~ct- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Ferveō: 7 words (ferveō, cōnferveō, dēferveō, īnferveō, perferveō, referveō).
    conjugation!("ferveō", 7, "~veō";
    stem lemma: "~v- like ferveō";
    stem indicative_act_pres_1_pl: "~v- like habeō";
    stem indicative_act_perf_1_sg: "~bu- like agitō";
    stem supine_acc: "~vit- like dēpudet";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Īnsīdō: 7 words (resīdō, assīdō, subsīdō, cōnsīdō, īnsīdō, dēsīdō). Like Possideō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("īnsīdō", 7, "~īdō", like Possideō;
    stem lemma: "~īd- like agō";
    stem indicative_act_pres_1_pl: "~īd- like agō";
    stem indicative_act_perf_1_sg: "~ēd- like mūniō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dēpendeō: 7 words (strīdeō, commordeō, remordeō, ammordeō, dēpendeō, prōpendeō). Like Ārdeō.
    conjugation!("dēpendeō", 7, "~deō", like Ārdeō;
    stem indicative_act_perf_1_sg: "~d- like agitō";
    ),
    // Peragō: 7 words (peragō, satagō, reagō, circumagō, abagō, vervagō). Like Compingō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("peragō", 7, "~agō", like Compingō;
    stem lemma: "~ag- like agō";
    stem indicative_act_pres_1_pl: "~ag- like agō";
    stem indicative_act_perf_1_sg: "~ēg- like mūniō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cognōscō: 7 words (conōscō, cognōscō, recognōscō, agnōscō, praecognōscō, adgnōscō). Like Quiēscō with `ˇ` after each pattern.
    conjugation!("cognōscō", 7, "~scōˇ", like Quiēscō with "ˇ";
    ),
    // Coquō: 7 words (coquō, concoquō, discoquō, excoquō, dēcoquō, recoquō). Like Extruō with `ˇ` after each pattern.
    conjugation!("coquō", 7, "~ōˇ", like Extruō with "ˇ";
    ),
    // Īnserō: 7 words (disserō, serō, asserō, īnserō, cōnserō, adserō). Like Dēsinō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("īnserō", 7, "~erō", like Dēsinō;
    stem lemma: "~er- like compercō";
    stem indicative_act_pres_1_pl: "~er- like agō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cēnseō: 6 words (recēnseō, cēnseō, paeniteō, accēnseō, percēnseō, poeniteō). Like Habeō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("cēnseō", 6, "~eō", like Habeō;
    stem lemma: "~- like impleō";
    stem supine_acc: "~- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Videō: 6 words (videō, invideō, revideō, prōvideō, pervideō, praevideō). Like Possideō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("videō", 6, "~ideō", like Possideō;
    stem lemma: "~id- like impleō";
    stem indicative_act_perf_1_sg: "~īd- like agitō";
    stem supine_acc: "~īs- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Pergō: 6 words (surgō, resurgō, pergō, cōnsurgō, exsurgō, exurgō). Like Dēfīgō.
    conjugation!("pergō", 6, "~gō", like Dēfīgō;
    stem indicative_act_perf_1_sg: "~rēx- like agitō";
    stem supine_acc: "~rēct- like agitō";
    ),
    // Advīvō: 6 words (vīvō, convīvō, revīvō, advīvō, supervīvō, fīvō). Like Solvō, except in 32 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_pres_3 pl, pass_impf_1 sg, ….
    conjugation!("advīvō", 6, "~vō", like Solvō;
    stem indicative_act_perf_1_sg: "~x- like agitō";
    stem supine_acc: "~ct- like ēveniō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_pres_3: _, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_impf_3: _, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_fut_3: _, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_pres_3: _, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    subjunctive_pass_impf_3: _, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~vitor", derived;
    infinitive_act: "~vere", _, _;
    participle: _, _, _, "~vendum";
    ),
    // Cōnstō: 6 words (praestō, cōnstō, distō, īnstō, prōstō, antestō). Like Cubō, except in 3 cells: infinitive_act fut, infinitive_pass perf, participle perf_pass.
    conjugation!("cōnstō", 6, "~ō", like Cubō;
    stem indicative_act_perf_1_sg: "~it- like agitō";
    infinitive_act: _, _, "~ātūrum esse";
    infinitive_pass: _, "~ātum esse", _;
    participle: _, "~ātus", _, _;
    ),
    // Innectō: 6 words (cōnectō, connectō, subnectō, innectō, annectō, adnectō). Like Nectō.
    conjugation!("innectō", 6, "~ctō", like Nectō;
    stem indicative_act_perf_1_sg: "~xu- like agitō";
    ),
    // Eximō: 6 words (dirimō, eximō, redimō, adimō, interimō, perimō). Like Excipiō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("eximō", 6, "~i~ō", like Excipiō;
    stem lemma: "~i~- like compercō";
    stem indicative_act_pres_1_pl: "~i~- like agō";
    stem supine_acc: "~ē~pt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Vinciō: 6 words (vinciō, revinciō, dēvinciō, ēvinciō, praevinciō, circumvinciō). Like Tingō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("vinciō", 6, "~i~ciō", like Tingō;
    stem lemma: "~i~c- like mūniō";
    stem indicative_act_pres_1_pl: "~i~c- like mūniō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dēfluō: 6 words (affluō, adfluō, īnfluō, dēfluō, effluō, perfluō). Like Fluō, except in 26 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("dēfluō", 6, "~uō", like Fluō;
    stem supine_acc: "~ūx- like labōrō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~uitor", _;
    infinitive_act: "~uere", _, _;
    participle: _, _, _, "~uendum";
    ),
    // Exsorbeō: 6 words (exsorbeō, exorbeō, obsorbeō, persorbeō, resorbeō, apsorbeō).
    conjugation!("exsorbeō", 6, "~beō";
    stem lemma: "~b- like impleō";
    stem indicative_act_pres_1_pl: "~b- like habeō";
    stem indicative_act_perf_1_sg: "~bu- like agitō";
    stem supine_acc: "~pt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cōnfugiō: 6 words (refugiō, aufugiō, perfugiō, cōnfugiō, diffugiō, profugiō). Like Fugiō, except in 32 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_pres_3 pl, pass_impf_1 sg, ….
    conjugation!("cōnfugiō", 6, "~u~iō", like Fugiō;
    stem supine_acc: "~u~it- like dēpudet";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_pres_3: _, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_impf_3: _, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_fut_3: _, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_pres_3: _, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    subjunctive_pass_impf_3: _, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~u~itor", derived;
    infinitive_act: "~u~ere", _, _;
    participle: _, _, _, "~u~iendum|~u~iundum";
    ),
    // Apīscor: 6 words (apīscor, dēpecīscor, pacīscor, compacīscor, compecīscor, dēpacīscor). Like Pāscor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("apīscor", 6, "~īscor", like Pāscor;
    stem lemma: "~īsc- like nāscor";
    stem indicative_act_pres_1_pl: "~īsc- like nāscor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ēdiscō: 6 words (discō, condiscō, addiscō, ēdiscō, dēdiscō, perdiscō).
    conjugation!("ēdiscō", 6, "~s~ō";
    stem lemma: "~s~- like agō";
    stem indicative_act_pres_1_pl: "~s~- like agō";
    stem indicative_act_perf_1_sg: "~di~- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Deicō: 6 words (deicō, indeicō, addeicō, prodeicō, contrādeicō, interdeicō).
    conjugation!("deicō", 6, "~e~cō";
    stem lemma: "~e~c- like compercō";
    stem indicative_act_pres_1_pl: "~e~c- like agō";
    stem indicative_act_perf_1_sg: "~e~x- like agitō";
    stem supine_acc: "~~ct- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Scandō: 6 words (scandō, retundō, oppandō, superscandō, suprascandō, dispandō). Like Praecīdō with `ˇ` after each pattern.
    conjugation!("scandō", 6, "~dōˇ", like Praecīdō with "ˇ";
    ),
    // Occurrō: 5 words (carrō, antecurrō, circumcurrō, converrō, convorrō). Like Gerō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("occurrō", 5, "~rō", like Gerō;
    stem lemma: "~r- like agō";
    stem indicative_act_perf_1_sg: "~r- like mūniō";
    stem supine_acc: "~s- like occurrō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Sūmō2: 5 words (prōmō, dēmō, cōmō, exprōmō, dēprōmō). Like Compescō.
    conjugation!("sūmō", 5, "~ō", like Compescō;
    stem indicative_act_perf_1_sg: "~ps- like agitō";
    stem supine_acc: "~pt- like agitō";
    ),
    // Vēnumdō: 5 words (dō, circundō, circumdō, vēnundō, vēnumdō).
    conjugation!("vēnumdō", 5, "~ō";
    stem lemma: "~- like vēnumdō";
    stem indicative_act_pres_1_pl: "~- like vēnumdō";
    stem indicative_act_perf_1_sg: "~ed- like agitō";
    stem supine_acc: "~at- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Serviō: 5 words (serviō, dormiō, hymniō, saeviō, īnsāniō). Like Mūniō, except in 33 cells: act_perf_1 sg, pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_pres_3 pl, ….
    conjugation!("serviō", 5, "~iō", like Mūniō;
    stem supine_acc: "~īt- like dēpudet";
    act_perf_1: "~īvī|~iī", _;
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_pres_3: _, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_impf_3: _, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_fut_3: _, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_pres_3: _, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    subjunctive_pass_impf_3: _, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~ītor", derived;
    infinitive_act: "~īre", _, _;
    participle: _, _, _, "~iendum|~iundum";
    ),
    // Adipīscor: 5 words (proficīscor, adipīscor, comminīscor, indipīscor, minīscor).
    conjugation!("adipīscor", 5, "~i~īscor";
    stem lemma: "~i~īsc- like nāscor";
    stem indicative_act_pres_1_pl: "~i~īsc- like nāscor";
    stem indicative_act_perf_1_sg: "~e~t- like modulor";
    stem supine_acc: "~e~t- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Astō: 5 words (restō, obstō, astō, perstō, adstō). Like Cōnstō, except in 34 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_pres_3 pl, pass_impf_1 sg, ….
    conjugation!("astō", 5, "~ō", like Cōnstō;
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_pres_3: _, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_impf_3: _, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_fut_3: _, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_pres_3: _, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    subjunctive_pass_impf_3: _, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~ātor", derived;
    infinitive_act: "~āre", _, _;
    infinitive_pass: _, derived, _;
    participle: _, derived, _, "~andum";
    ),
    // Occinō: 5 words (concinō, accinō, praecinō, incinō, occinō). Like Eximō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("occinō", 5, "~i~ō", like Eximō;
    stem lemma: "~i~- like agō";
    stem indicative_act_perf_1_sg: "~i~u- like mūniō";
    stem supine_acc: "~e~t- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Decet: 5 words (licet, decet, addecet, dēdecet, condecet). Like Dēpudet, except in 26 cells: act_pres_3 pl, act_impf_3 pl, act_fut_3 pl, act_perf_3 pl, act_plup_3 pl, act_futp_3 pl, ….
    conjugation!("decet", 5, "~et", like Dēpudet;
    stem supine_acc: "~it- like modulor";
    act_pres_3: _, "~ent";
    act_impf_3: _, "~ēbant";
    act_fut_3: _, "~ēbunt";
    act_perf_3: _, "~uērunt|~uēre";
    act_plup_3: _, "~uerant";
    act_futp_3: _, "~uerint";
    pass_pres_3: derived, _;
    pass_impf_3: derived, _;
    pass_fut_3: derived, _;
    pass_perf_3: derived, _;
    pass_plup_3: derived, _;
    pass_futp_3: derived, _;
    subjunctive_act_pres_3: _, "~eant";
    subjunctive_act_impf_3: _, "~ērent";
    subjunctive_act_perf_3: _, "~uerint";
    subjunctive_act_plup_3: _, "~uissent";
    subjunctive_pass_pres_3: derived, _;
    subjunctive_pass_impf_3: derived, _;
    subjunctive_pass_perf_3: derived, _;
    subjunctive_pass_plup_3: derived, _;
    future_imperative_act_3: _, "~entō";
    future_imperative_pass_3: derived, _;
    infinitive_pass: derived, derived, _;
    participle: _, derived, _, derived;
    ),
    // Supersedeō: 5 words (sedeō, circumsedeō, persedeō, supersedeō, suprāsedeō). Like Comedō, except in 9 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, infinitive_pass fut, participle fut_act, ….
    conjugation!("supersedeō", 5, "~edeō", like Comedō;
    stem lemma: "~ed- like habeō";
    stem indicative_act_pres_1_pl: "~ed- like habeō";
    stem supine_acc: "~ess- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    infinitive_pass: _, _, derived;
    participle: _, _, derived, _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Clueō: 5 words (clueō, langueō, abnueō, cēveō, connīveō). Like Habeō.
    conjugation!("clueō", 5, "~eō", like Habeō;
    stem indicative_act_perf_1_sg: "~- like mūniō";
    ),
    // Ambigō: 5 words (convergō, assurgō, ambigō, īnsurgō, adsurgō). Like Pergō, except in 26 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("ambigō", 5, "~gō", like Pergō;
    stem supine_acc: "~rēct- like labōrō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~gitor", _;
    infinitive_act: "~gere", _, _;
    participle: _, _, _, "~gendum|~gundum";
    ),
    // Contremīscō: 5 words (resipīscō, ingemīscō, dēlitīscō, conticīscō, contremīscō).
    conjugation!("contremīscō", 5, "~īscō";
    stem lemma: "~īsc- like compercō";
    stem indicative_act_pres_1_pl: "~īsc- like agō";
    stem indicative_act_perf_1_sg: "~u- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Colō: 5 words (colō, recolō, excolō, circumcolō, accolō). Like Cōnsulō with `ˇ` after each pattern.
    conjugation!("colō", 5, "~ōˇ", like Cōnsulō with "ˇ";
    ),
    // Clangō: 5 words (cadō, stertō, clangō, accanō, obtexō). Like Compescō, except in 29 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("clangō", 5, "~ō", like Compescō;
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~itor", _;
    infinitive_act: "~ere", _, derived;
    infinitive_pass: _, derived, _;
    participle: _, derived, _, "~endum|~undum";
    ),
    // Caveō: 4 words (caveō, faveō, paveō, praecaveō).
    conjugation!("caveō", 4, "~aveō";
    stem lemma: "~av- like impleō";
    stem indicative_act_pres_1_pl: "~av- like habeō";
    stem indicative_act_perf_1_sg: "~ā- like caveō";
    stem supine_acc: "~aut- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cieō: 4 words (cieō, excieō, concieō, percieō). Like Mūniō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("cieō", 4, "~ieō", like Mūniō;
    stem lemma: "~i- like impleō";
    stem indicative_act_pres_1_pl: "~i- like habeō";
    stem supine_acc: "~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Patior: 4 words (patior, compatior, perpetior, conpatior). Like Ēgredior, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("patior", 4, "~tior", like Ēgredior;
    stem lemma: "~t- like ēgredior";
    stem indicative_act_pres_1_pl: "~ti- like ēgredior";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fīō: 4 words (fīō, obsolefīō, dēfīō, vacēfīō). Like Modulor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("fīō", 4, "~īō", like Modulor;
    stem lemma: "~- like fīō";
    stem indicative_act_pres_1_pl: "~ī- like faciō";
    stem indicative_act_perf_1_sg: "~act- like modulor";
    stem supine_acc: "~act- like fīō";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Relinquō: 4 words (dēlinquō, relinquō, linquō, dērelinquō).
    conjugation!("relinquō", 4, "~inquō";
    stem lemma: "~inqu- like minuō";
    stem indicative_act_pres_1_pl: "~inqu- like agō";
    stem indicative_act_perf_1_sg: "~īqu- like agitō";
    stem supine_acc: "~ict- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Sapiō: 4 words (cupiō, sapiō, concupiō, supersapiō). Like Mūniō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("sapiō", 4, "~iō", like Mūniō;
    stem lemma: "~- like fugiō";
    stem indicative_act_pres_1_pl: "~i- like faciō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Abeō: 4 words (abeō, prōdeō, pereō, redeō). Like Obeō, except in 32 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_pres_3 pl, pass_impf_1 sg, ….
    conjugation!("abeō", 4, "~eō", like Obeō;
    stem supine_acc: "~it- like dēpudet";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_pres_3: _, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_impf_3: _, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_fut_3: _, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_pres_3: _, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    subjunctive_pass_impf_3: _, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~ītor", derived;
    infinitive_act: "~īre", _, _;
    participle: _, _, _, "~eundum";
    ),
    // Ēvellō: 4 words (dīvellō, convellō, ēvellō, refellō). Like Dēpellō.
    conjugation!("ēvellō", 4, "~e~lō", like Dēpellō;
    stem indicative_act_perf_1_sg: "~e~l- like agitō";
    ),
    // Currō: 4 words (currō, percurrō, prōcurrō, superexcurrō).
    conjugation!("currō", 4, "~~rō";
    stem lemma: "~~r- like compercō";
    stem indicative_act_pres_1_pl: "~~r- like agō";
    stem indicative_act_perf_1_sg: "~cu~r- like mūniō";
    stem supine_acc: "~~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Nītor: 4 words (nītor, ēnītor, cōnītor, connītor). Like Ūtor.
    conjugation!("nītor", 4, "~tor", like Ūtor;
    stem indicative_act_perf_1_sg: "~- like nītor";
    stem supine_acc: "~- like nītor";
    ),
    // Obtundō: 4 words (obtundō, pertundō, contundō, extundō). Like Effundō.
    conjugation!("obtundō", 4, "~undō", like Effundō;
    stem indicative_act_perf_1_sg: "~ud- like agitō";
    ),
    // Hauriō: 4 words (exhauriō, hauriō, dehauriō, perhauriō).
    conjugation!("hauriō", 4, "~riō";
    stem lemma: "~r- like mūniō";
    stem indicative_act_pres_1_pl: "~r- like mūniō";
    stem indicative_act_perf_1_sg: "~s- like agitō";
    stem supine_acc: "~st- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Comperiō: 4 words (comperiō, īciō, repperiō, conperiō). Like Intersaepiō.
    conjugation!("comperiō", 4, "~iō", like Intersaepiō;
    stem indicative_act_perf_1_sg: "~- like agitō";
    ),
    // Circumvehor: 4 words (circumvehor, praetervehor, praevehor, supervehor). Like Modulor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("circumvehor", 4, "~hor", like Modulor;
    stem lemma: "~h- like nāscor";
    stem indicative_act_pres_1_pl: "~h- like nāscor";
    stem indicative_act_perf_1_sg: "~ct- like modulor";
    stem supine_acc: "~ct- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Adnītor: 4 words (innītor, obnītor, adnītor, annītor). Like Ūtor, except in 5 cells: infinitive_act perf, infinitive_act fut, participle fut_act, supine acc, supine abl.
    conjugation!("adnītor", 4, "~tor", like Ūtor;
    infinitive_act: _, "~xum esse|~sum esse", "~xūrum esse|~sūrum esse";
    participle: _, _, "~xūrus|~sūrus", _;
    supine: _, _, _, "~xum|~sum", "~xū|~sū", _;
    ),
    // Extruō: 4 words (tāgō, tragō, extruō, superextruō). Like Compescō, except in 3 cells: present_imperative_pass pl, future_imperative_pass_3 sg, infinitive_act pres.
    conjugation!("extruō", 4, "~ō", like Compescō;
    stem lemma: "~- like minuō";
    stem indicative_act_perf_1_sg: "~x- like agitō";
    stem supine_acc: "~ct- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Frangō: 4 words (frangō, tangō, pangō, dēpangō). Like Peragō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("frangō", 4, "~angō", like Peragō;
    stem lemma: "~ang- like compercō";
    stem indicative_act_pres_1_pl: "~ang- like agō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fricō: 4 words (secō, fricō, dēfricō, praesecō). Like Micō.
    conjugation!("fricō", 4, "~ō", like Micō;
    stem supine_acc: "~- like fricō";
    ),
    // Mulceō: 4 words (mulceō, commulceō, permulceō, remulceō). Like Compercō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("mulceō", 4, "~ceō", like Compercō;
    stem lemma: "~c- like impleō";
    stem indicative_act_pres_1_pl: "~c- like habeō";
    stem supine_acc: "~s- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Apage: 4 words (sum, inquam, apage, coest). Like Adipīscor, except in 74 cells: act_pres_1 sg, act_pres_1 pl, act_pres_2 sg, act_pres_2 pl, act_pres_3 sg, act_pres_3 pl, ….
    conjugation!("apage", 4, "~~", like Adipīscor;
    act_pres_1: "~~", derived;
    act_pres_2: derived, derived;
    act_pres_3: derived, derived;
    act_impf_1: derived, derived;
    act_impf_2: derived, derived;
    act_impf_3: derived, derived;
    act_fut_1: derived, derived;
    act_fut_2: derived, derived;
    act_fut_3: derived, derived;
    act_perf_1: derived, derived;
    act_perf_2: derived, derived;
    act_perf_3: derived, derived;
    act_plup_1: derived, derived;
    act_plup_2: derived, derived;
    act_plup_3: derived, derived;
    act_futp_1: derived, derived;
    act_futp_2: derived, derived;
    act_futp_3: derived, derived;
    subjunctive_act_pres_1: derived, derived;
    subjunctive_act_pres_2: derived, derived;
    subjunctive_act_pres_3: derived, derived;
    subjunctive_act_impf_1: derived, derived;
    subjunctive_act_impf_2: derived, derived;
    subjunctive_act_impf_3: derived, derived;
    subjunctive_act_perf_1: derived, derived;
    subjunctive_act_perf_2: derived, derived;
    subjunctive_act_perf_3: derived, derived;
    subjunctive_act_plup_1: derived, derived;
    subjunctive_act_plup_2: derived, derived;
    subjunctive_act_plup_3: derived, derived;
    present_imperative_act: "~~", "~it~";
    future_imperative_act_2: derived, _;
    future_imperative_act_3: _, derived;
    infinitive_act: derived, derived, derived;
    participle: derived, _, _, derived;
    gerund: _, derived, derived, derived, _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Ēvādō: 3 words (vilipendō, circumsīdō, obsīdō). Like Praecīdō.
    conjugation!("ēvādō", 3, "~dō", like Praecīdō;
    stem indicative_act_perf_1_sg: "~s- like agitō";
    ),
    // Coniungō: 3 words (ēmungō, pungō, suffringō).
    conjugation!("coniungō", 3, "~u~gō";
    stem lemma: "~u~g- like compercō";
    stem indicative_act_pres_1_pl: "~u~g- like agō";
    stem indicative_act_perf_1_sg: "~ū~x- like agitō";
    stem supine_acc: "~ū~ct- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Quaerō: 3 words (quaerō, conquaerō, disquīrō). Like Gerō.
    conjugation!("quaerō", 3, "~rō", like Gerō;
    stem indicative_act_perf_1_sg: "~sīv- like agitō";
    stem supine_acc: "~sīt- like agitō";
    ),
    // Adliciō: 3 words (illiciō, inliciō, adliciō). Like Dispiciō.
    conjugation!("adliciō", 3, "~iciō", like Dispiciō;
    stem indicative_act_perf_1_sg: "~ēx- like agitō";
    ),
    // Stō: 3 words (stō, interstō, substō). Like Astō, except in 7 cells: pass_perf_3 sg, pass_plup_3 sg, pass_futp_3 sg, subjunctive_pass_perf_3 sg, subjunctive_pass_plup_3 sg, infinitive_pass perf, ….
    conjugation!("stō", 3, "~ō", like Astō;
    stem indicative_act_perf_1_sg: "~et- like mūniō";
    pass_perf_3: "~atum est", _;
    pass_plup_3: "~atum erat", _;
    pass_futp_3: "~atum erit", _;
    subjunctive_pass_perf_3: "~atum sit", _;
    subjunctive_pass_plup_3: "~atum esset", _;
    infinitive_pass: _, "~atum esse", _;
    participle: _, "~atum", _, _;
    ),
    // Dīluō: 3 words (suō, tribuō, dīluō). Like Minuō, except in 26 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("dīluō", 3, "~uō", like Minuō;
    stem supine_acc: "~ūt- like labōrō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~uitor", _;
    infinitive_act: "~uere", _, _;
    participle: _, _, _, "~uendum";
    ),
    // Misceō: 3 words (misceō, permisceō, remisceō). Like Immisceō, except in 8 cells: pass_perf_1 sg, infinitive_act fut, infinitive_pass perf, infinitive_pass fut, participle perf_pass, participle fut_act, ….
    conjugation!("misceō", 3, "~sceō", like Immisceō;
    pass_perf_1: "~stus sum", _;
    infinitive_act: _, _, "~xtūrum esse|~stūrum esse";
    infinitive_pass: _, "~xtum esse|~stum esse", "~xtum īrī|~stum īrī";
    participle: _, "~xtus|~stus", "~xtūrus|~stūrus", _;
    supine: _, _, _, "~xtum|~stum", "~xtū|~stū", _;
    ),
    // Iubeō: 3 words (iubeō, fidēiubeō, ioubeō). Like Exsorbeō.
    conjugation!("iubeō", 3, "~beō", like Exsorbeō;
    stem indicative_act_perf_1_sg: "~ss- like agitō";
    stem supine_acc: "~ss- like agitō";
    ),
    // Fundō: 3 words (fundō, refundō, praefundō). Like Effundō, except in 26 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("fundō", 3, "~undō", like Effundō;
    stem supine_acc: "~ūs- like labōrō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    future_imperative_pass_3: "~unditor", _;
    infinitive_act: "~undere", _, _;
    participle: _, _, _, "~undendum|~undundum";
    ),
    // Scatō: 3 words (scatō, battō, abbattō). Like Compescō.
    conjugation!("scatō", 3, "~ō", like Compescō;
    stem supine_acc: "~ūt- like agitō";
    ),
    // Explicō: 3 words (explicō, applicō, adplicō). Like Agitō, except in 8 cells: pass_perf_1 sg, infinitive_act fut, infinitive_pass perf, infinitive_pass fut, participle perf_pass, participle fut_act, ….
    conjugation!("explicō", 3, "~ō", like Agitō;
    stem indicative_act_perf_1_sg: "~- like explicō";
    pass_perf_1: "~itus sum", _;
    infinitive_act: _, _, "~ātūrum esse|~itūrum esse";
    infinitive_pass: _, "~ātum esse|~itum esse", "~ātum īrī|~itum īrī";
    participle: _, "~ātus|~itus", "~ātūrus|~itūrus", _;
    supine: _, _, _, "~ātum|~itum", "~ātū|~itū", _;
    ),
    // Dēfungor: 3 words (fungor, dēfungor, perfungor). Like Adipīscor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("dēfungor", 3, "~u~gor", like Adipīscor;
    stem lemma: "~u~g- like nāscor";
    stem indicative_act_pres_1_pl: "~u~g- like nāscor";
    stem indicative_act_perf_1_sg: "~ū~ct- like modulor";
    stem supine_acc: "~ū~ct- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fīdō: 3 words (fīdō, cōnfīdō, diffīdō). Like Ūtor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("fīdō", 3, "~dō", like Ūtor;
    stem lemma: "~d- like fīdō";
    stem indicative_act_pres_1_pl: "~d- like agō";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Libet: 3 words (libet, lubet, perlibet). Like Decet, except in 21 cells: act_pres_3 pl, act_impf_3 pl, act_fut_3 pl, act_perf_3 sg, act_perf_3 pl, act_plup_3 sg, ….
    conjugation!("libet", 3, "~et", like Decet;
    act_pres_3: _, derived;
    act_impf_3: _, derived;
    act_fut_3: _, derived;
    act_perf_3: "~uit|~itus est", derived;
    act_plup_3: "~uerat|~itus erat", derived;
    act_futp_3: "~uerit|~itus erit", derived;
    subjunctive_act_pres_3: _, derived;
    subjunctive_act_impf_3: _, derived;
    subjunctive_act_perf_3: "~uerit|~itus sit", derived;
    subjunctive_act_plup_3: "~uisset|~itus esset", derived;
    future_imperative_act_3: _, derived;
    infinitive_act: _, "~uisse|~itum esse", "~itum īre";
    gerund: _, derived, derived, derived, _, _;
    ),
    // Coepī: 3 words (ōdī, coepī, recoepī). Like Libet, except in 11 cells: act_pres_1 sg, act_pres_3 sg, act_impf_3 sg, act_fut_3 sg, subjunctive_act_pres_3 sg, subjunctive_act_impf_3 sg, ….
    conjugation!("coepī", 3, "~ī", like Libet;
    stem indicative_act_perf_1_sg: "~- like agitō";
    stem supine_acc: "~t- like agitō";
    act_pres_1: "~ī", _;
    act_pres_3: derived, _;
    act_impf_3: derived, _;
    act_fut_3: derived, _;
    subjunctive_act_pres_3: derived, _;
    subjunctive_act_impf_3: derived, _;
    future_imperative_act_3: derived, _;
    infinitive_act: derived, _, _;
    participle: derived, _, _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Dētergeō: 3 words (dētergeō, immulgeō, inmulgeō). Like Tergeō, except in 23 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("dētergeō", 3, "~geō", like Tergeō;
    stem lemma: "~g- like madeō";
    stem supine_acc: "~s- like labōrō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    ),
    // Sufferō: 3 words (offerō, differō, sufferō).
    conjugation!("sufferō", 3, "~fferō";
    stem lemma: "~ffer- like referō";
    stem indicative_act_pres_1_pl: "~ffer- like referō";
    stem indicative_act_perf_1_sg: "~stul- like mūniō";
    stem supine_acc: "~blāt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Antecapiō: 3 words (superiaciō, interiaciō, antecapiō). Like Capiō.
    conjugation!("antecapiō", 3, "~a~iō", like Capiō;
    stem supine_acc: "~e~t- like agitō";
    ),
    // Commeminī: 3 words (meminī, commeminī, rememinī). Like Apage, except in 36 cells: act_pres_1 sg, act_pres_1 pl, act_pres_2 sg, act_pres_2 pl, act_pres_3 sg, act_pres_3 pl, ….
    conjugation!("commeminī", 3, "~i~ī", like Apage;
    act_pres_1: "~i~ī", "~i~imus";
    act_pres_2: "~i~istī", "~i~istis";
    act_pres_3: "~i~it", "~i~ērunt|~i~ēre";
    act_impf_1: "~i~eram", "~i~erāmus";
    act_impf_2: "~i~erās", "~i~erātis";
    act_impf_3: "~i~erat", "~i~erant";
    act_fut_1: "~i~erō", "~i~erimus";
    act_fut_2: "~i~eris", "~i~eritis";
    act_fut_3: "~i~erit", "~i~erint";
    subjunctive_act_pres_1: "~i~erim", "~i~erīmus";
    subjunctive_act_pres_2: "~i~erīs", "~i~erītis";
    subjunctive_act_pres_3: "~i~erit", "~i~erint";
    subjunctive_act_impf_1: "~i~issem", "~i~issēmus";
    subjunctive_act_impf_2: "~i~issēs", "~i~issētis";
    subjunctive_act_impf_3: "~i~isset", "~i~issent";
    present_imperative_act: derived, derived;
    future_imperative_act_2: "~e~tō", "~e~tōte";
    infinitive_act: "~i~isse", _, _;
    participle: "~i~ēns", _, _, _;
    ),
    // Contemnō: 3 words (vannō, temnō, contemnō).
    conjugation!("contemnō", 3, "~nō";
    stem lemma: "~n- like compercō";
    stem indicative_act_pres_1_pl: "~n- like agō";
    stem indicative_act_perf_1_sg: "~- like contemnō";
    pass_perf_1: "~tus sum", _;
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, "~ptūrum esse|~tūrum esse";
    infinitive_pass: _, "~ptum esse|~tum esse", "~ptum īrī|~tum īrī";
    participle: _, "~ptus|~tus", "~ptūrus|~tūrus", _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, "~ptum|~tum", "~ptū|~tū", _;
    ),
    // Dētorreō: 3 words (contorreō, dētorreō, extorreō).
    conjugation!("dētorreō", 3, "~orreō";
    stem lemma: "~orr- like impleō";
    stem indicative_act_pres_1_pl: "~orr- like habeō";
    stem indicative_act_perf_1_sg: "~orru- like agitō";
    stem supine_acc: "~ōst- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Superstō: 3 words (superstō, circumstō, antistō). Like Cubō, except in 4 cells: pass_perf_1 sg, infinitive_act fut, infinitive_pass perf, participle perf_pass.
    conjugation!("superstō", 3, "~ō", like Cubō;
    stem indicative_act_perf_1_sg: "~et- like agitō";
    pass_perf_1: derived, _;
    infinitive_act: _, _, derived;
    infinitive_pass: _, derived, _;
    participle: _, derived, _, _;
    ),
    // Gignō: 3 words (gignō, ingignō, prōgignō).
    conjugation!("gignō", 3, "~ig~ō";
    stem lemma: "~ig~- like compercō";
    stem indicative_act_pres_1_pl: "~ig~- like agō";
    stem indicative_act_perf_1_sg: "~e~u- like agitō";
    stem supine_acc: "~e~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Assiliō: 3 words (assiliō, prōsiliō, resiliō). Like Īnsiliō, except in 23 cells: pass_pres_1 sg, pass_pres_1 pl, pass_pres_2 sg, pass_pres_2 pl, pass_impf_1 sg, pass_impf_1 pl, ….
    conjugation!("assiliō", 3, "~i~iō", like Īnsiliō;
    stem lemma: "~i~- like ēveniō";
    stem supine_acc: "~u~t- like labōrō";
    pass_pres_1: derived, derived;
    pass_pres_2: derived, derived;
    pass_impf_1: derived, derived;
    pass_impf_2: derived, derived;
    pass_fut_1: derived, derived;
    pass_fut_2: derived, derived;
    pass_perf_1: derived, _;
    subjunctive_pass_pres_1: derived, derived;
    subjunctive_pass_pres_2: derived, derived;
    subjunctive_pass_impf_1: derived, derived;
    subjunctive_pass_impf_2: derived, derived;
    present_imperative_pass: derived, _;
    future_imperative_pass_2: derived, _;
    ),
    // Convalēscō: 3 words (convalēscō, coalēscō, tonēscō). Like Clangō, except in 16 cells: pass_pres_3 sg, pass_pres_3 pl, pass_impf_3 sg, pass_impf_3 pl, pass_fut_3 sg, pass_fut_3 pl, ….
    conjugation!("convalēscō", 3, "~ēscō", like Clangō;
    stem lemma: "~ēsc- like convalēscō";
    stem indicative_act_pres_1_pl: "~ēsc- like agō";
    stem supine_acc: "~it- like modulor";
    pass_pres_3: derived, derived;
    pass_impf_3: derived, derived;
    pass_fut_3: derived, derived;
    subjunctive_pass_pres_3: derived, derived;
    subjunctive_pass_impf_3: derived, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, derived;
    infinitive_pass: derived, _, _;
    participle: _, _, _, derived;
    gerund: _, _, _, _, derived, _;
    ),
    // Tuor: 3 words (compluor, conpluor, tuor). Like Modulor, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("tuor", 3, "~uor", like Modulor;
    stem lemma: "~u- like cōnsequor";
    stem indicative_act_pres_1_pl: "~u- like nāscor";
    stem indicative_act_perf_1_sg: "~ūt- like modulor";
    stem supine_acc: "~ūt- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Adhērēscō: 3 words (adhērēscō, adhaerēscō, cohaerēscō). Like Apage, except in 2 cells: future_imperative_act_3 sg, gerund abl.
    conjugation!("adhērēscō", 3, "~rē~cō", like Apage;
    stem lemma: "~rē~c- like convalēscō";
    stem indicative_act_pres_1_pl: "~rē~c- like agō";
    stem indicative_act_perf_1_sg: "~~- like agitō";
    stem supine_acc: "~~- like modulor";
    future_imperative_act_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Iūxtāpōnō: 3 words (praedispōnō, praesuppōnō, iūxtāpōnō). Like Repōnō, except in 4 cells: infinitive_pass fut, participle fut_act, supine acc, supine abl.
    conjugation!("iūxtāpōnō", 3, "~ōnō", like Repōnō;
    stem indicative_act_perf_1_sg: "~osu- like agitō";
    stem supine_acc: "~osit- like agitō";
    infinitive_pass: _, _, derived;
    participle: _, _, derived, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Adlavō: 3 words (lavō, perlavō, adlavō).
    conjugation!("adlavō", 3, "~a~ō";
    stem lemma: "~a~- like agitō";
    stem indicative_act_pres_1_pl: "~a~- like agitō";
    stem indicative_act_perf_1_sg: "~ā~- like agitō";
    stem supine_acc: "~a~āt- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Effervēscō: 3 words (effervēscō, dēfervēscō, cōnfervēscō).
    conjugation!("effervēscō", 3, "~vēscō";
    stem lemma: "~vēsc- like agō";
    stem indicative_act_pres_1_pl: "~vēsc- like agō";
    stem indicative_act_perf_1_sg: "~- like effervēscō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dīlūculat: 2 words (grandinat, dīlūculat). Like Libet, except in 15 cells: act_pres_1 sg, act_pres_3 sg, act_impf_3 sg, act_fut_3 sg, subjunctive_act_pres_3 sg, subjunctive_act_impf_3 sg, ….
    conjugation!("dīlūculat", 2, "~at", like Libet;
    stem indicative_act_perf_1_sg: "~āv- like dēpudet";
    act_pres_1: "~at", _;
    act_pres_3: "~at", _;
    act_impf_3: "~ābat", _;
    act_fut_3: "~ābit", _;
    subjunctive_act_pres_3: "~et", _;
    subjunctive_act_impf_3: "~āret", _;
    future_imperative_act_3: "~ātō", _;
    infinitive_act: "~āre", _, derived;
    participle: "~āns", _, _, _;
    gerund: _, "~andī", "~andō", "~andum", _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Coerceo: 1 words (coerceo).
    conjugation!("coerceo", 1, "~e~eo";
    stem lemma: "~ë~- like impleō";
    stem indicative_act_pres_1_pl: "~ë~- like habeō";
    stem indicative_act_perf_1_sg: "~ë~u- like agitō";
    stem supine_acc: "~ë~it- like agitō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Rēfert: 1 words (rēfert). Like Dīlūculat, except in 5 cells: act_pres_1 sg, act_pres_3 sg, subjunctive_act_impf_3 sg, infinitive_act pres, gerund abl.
    conjugation!("rēfert", 1, "~fert", like Dīlūculat;
    stem indicative_act_pres_1_pl: "~fer- like rēfert";
    stem indicative_act_perf_1_sg: "~tul- like dēpudet";
    act_pres_1: "~fert", _;
    act_pres_3: "~fert", _;
    subjunctive_act_impf_3: "~ferret", _;
    infinitive_act: "~ferre", _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Coemo: 1 words (coemo). Like Eximō, except in 5 cells: present_imperative_pass pl, future_imperative_act_3 sg, future_imperative_pass_3 sg, infinitive_act pres, gerund abl.
    conjugation!("coemo", 1, "~e~o", like Eximō;
    stem lemma: "~ë~- like compercō";
    stem indicative_act_pres_1_pl: "~ë~- like agō";
    present_imperative_pass: _, derived;
    future_imperative_act_3: derived, _;
    future_imperative_pass_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
];
