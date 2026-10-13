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
/// `ind_act_pres_1` (sg, pl)
/// `ind_act_pres_2` (sg, pl)
/// `ind_act_pres_3` (sg, pl)
/// `ind_act_impf_1` (sg, pl)
/// `ind_act_impf_2` (sg, pl)
/// `ind_act_impf_3` (sg, pl)
/// `ind_act_fut_1` (sg, pl)
/// `ind_act_fut_2` (sg, pl)
/// `ind_act_fut_3` (sg, pl)
/// `ind_act_perf_1` (sg, pl)
/// `ind_act_perf_2` (sg, pl)
/// `ind_act_perf_3` (sg, pl)
/// `ind_act_plup_1` (sg, pl)
/// `ind_act_plup_2` (sg, pl)
/// `ind_act_plup_3` (sg, pl)
/// `ind_act_futp_1` (sg, pl)
/// `ind_act_futp_2` (sg, pl)
/// `ind_act_futp_3` (sg, pl)
/// `ind_pass_pres_1` (sg, pl)
/// `ind_pass_pres_2` (sg, pl)
/// `ind_pass_pres_3` (sg, pl)
/// `ind_pass_impf_1` (sg, pl)
/// `ind_pass_impf_2` (sg, pl)
/// `ind_pass_impf_3` (sg, pl)
/// `ind_pass_fut_1` (sg, pl)
/// `ind_pass_fut_2` (sg, pl)
/// `ind_pass_fut_3` (sg, pl)
/// `ind_pass_perf_1` (sg, pl)
/// `ind_pass_perf_2` (sg, pl)
/// `ind_pass_perf_3` (sg, pl)
/// `ind_pass_plup_1` (sg, pl)
/// `ind_pass_plup_2` (sg, pl)
/// `ind_pass_plup_3` (sg, pl)
/// `ind_pass_futp_1` (sg, pl)
/// `ind_pass_futp_2` (sg, pl)
/// `ind_pass_futp_3` (sg, pl)
/// `subj_act_pres_1` (sg, pl)
/// `subj_act_pres_2` (sg, pl)
/// `subj_act_pres_3` (sg, pl)
/// `subj_act_impf_1` (sg, pl)
/// `subj_act_impf_2` (sg, pl)
/// `subj_act_impf_3` (sg, pl)
/// `subj_act_fut_1` (sg, pl)
/// `subj_act_fut_2` (sg, pl)
/// `subj_act_fut_3` (sg, pl)
/// `subj_act_perf_1` (sg, pl)
/// `subj_act_perf_2` (sg, pl)
/// `subj_act_perf_3` (sg, pl)
/// `subj_act_plup_1` (sg, pl)
/// `subj_act_plup_2` (sg, pl)
/// `subj_act_plup_3` (sg, pl)
/// `subj_act_futp_1` (sg, pl)
/// `subj_act_futp_2` (sg, pl)
/// `subj_act_futp_3` (sg, pl)
/// `subj_pass_pres_1` (sg, pl)
/// `subj_pass_pres_2` (sg, pl)
/// `subj_pass_pres_3` (sg, pl)
/// `subj_pass_impf_1` (sg, pl)
/// `subj_pass_impf_2` (sg, pl)
/// `subj_pass_impf_3` (sg, pl)
/// `subj_pass_fut_1` (sg, pl)
/// `subj_pass_fut_2` (sg, pl)
/// `subj_pass_fut_3` (sg, pl)
/// `subj_pass_perf_1` (sg, pl)
/// `subj_pass_perf_2` (sg, pl)
/// `subj_pass_perf_3` (sg, pl)
/// `subj_pass_plup_1` (sg, pl)
/// `subj_pass_plup_2` (sg, pl)
/// `subj_pass_plup_3` (sg, pl)
/// `subj_pass_futp_1` (sg, pl)
/// `subj_pass_futp_2` (sg, pl)
/// `subj_pass_futp_3` (sg, pl)
/// `imperative_act_pres_1` (sg, pl)
/// `imperative_act_pres_2` (sg, pl)
/// `imperative_act_pres_3` (sg, pl)
/// `imperative_act_fut_1` (sg, pl)
/// `imperative_act_fut_2` (sg, pl)
/// `imperative_act_fut_3` (sg, pl)
/// `imperative_pass_pres_1` (sg, pl)
/// `imperative_pass_pres_2` (sg, pl)
/// `imperative_pass_pres_3` (sg, pl)
/// `imperative_pass_fut_1` (sg, pl)
/// `imperative_pass_fut_2` (sg, pl)
/// `imperative_pass_fut_3` (sg, pl)
/// `infinitive_act` (pres, perf, fut)
/// `infinitive_pass` (pres, perf, fut)
/// `participle` (pres_act, perf_pass, fut_act, fut_pass)
/// `gerund` (nom, gen_, dat, acc, abl, voc)
/// `supine` (nom, gen_, dat, acc, abl, voc)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem verb_act_sg: $s0:tt;)?
     $(stem verb_ind: $s1:tt;)?
     $(stem verb_ind_act: $s2:tt;)?
     $(stem participle_perf_pass: $s3:tt;)?
     $(ind_act_pres_1: $c0:tt, $c1:tt;)?
     $(ind_act_pres_2: $c2:tt, $c3:tt;)?
     $(ind_act_pres_3: $c4:tt, $c5:tt;)?
     $(ind_act_impf_1: $c6:tt, $c7:tt;)?
     $(ind_act_impf_2: $c8:tt, $c9:tt;)?
     $(ind_act_impf_3: $c10:tt, $c11:tt;)?
     $(ind_act_fut_1: $c12:tt, $c13:tt;)?
     $(ind_act_fut_2: $c14:tt, $c15:tt;)?
     $(ind_act_fut_3: $c16:tt, $c17:tt;)?
     $(ind_act_perf_1: $c18:tt, $c19:tt;)?
     $(ind_act_perf_2: $c20:tt, $c21:tt;)?
     $(ind_act_perf_3: $c22:tt, $c23:tt;)?
     $(ind_act_plup_1: $c24:tt, $c25:tt;)?
     $(ind_act_plup_2: $c26:tt, $c27:tt;)?
     $(ind_act_plup_3: $c28:tt, $c29:tt;)?
     $(ind_act_futp_1: $c30:tt, $c31:tt;)?
     $(ind_act_futp_2: $c32:tt, $c33:tt;)?
     $(ind_act_futp_3: $c34:tt, $c35:tt;)?
     $(ind_pass_pres_1: $c36:tt, $c37:tt;)?
     $(ind_pass_pres_2: $c38:tt, $c39:tt;)?
     $(ind_pass_pres_3: $c40:tt, $c41:tt;)?
     $(ind_pass_impf_1: $c42:tt, $c43:tt;)?
     $(ind_pass_impf_2: $c44:tt, $c45:tt;)?
     $(ind_pass_impf_3: $c46:tt, $c47:tt;)?
     $(ind_pass_fut_1: $c48:tt, $c49:tt;)?
     $(ind_pass_fut_2: $c50:tt, $c51:tt;)?
     $(ind_pass_fut_3: $c52:tt, $c53:tt;)?
     $(ind_pass_perf_1: $c54:tt, $c55:tt;)?
     $(ind_pass_perf_2: $c56:tt, $c57:tt;)?
     $(ind_pass_perf_3: $c58:tt, $c59:tt;)?
     $(ind_pass_plup_1: $c60:tt, $c61:tt;)?
     $(ind_pass_plup_2: $c62:tt, $c63:tt;)?
     $(ind_pass_plup_3: $c64:tt, $c65:tt;)?
     $(ind_pass_futp_1: $c66:tt, $c67:tt;)?
     $(ind_pass_futp_2: $c68:tt, $c69:tt;)?
     $(ind_pass_futp_3: $c70:tt, $c71:tt;)?
     $(subj_act_pres_1: $c72:tt, $c73:tt;)?
     $(subj_act_pres_2: $c74:tt, $c75:tt;)?
     $(subj_act_pres_3: $c76:tt, $c77:tt;)?
     $(subj_act_impf_1: $c78:tt, $c79:tt;)?
     $(subj_act_impf_2: $c80:tt, $c81:tt;)?
     $(subj_act_impf_3: $c82:tt, $c83:tt;)?
     $(subj_act_fut_1: $c84:tt, $c85:tt;)?
     $(subj_act_fut_2: $c86:tt, $c87:tt;)?
     $(subj_act_fut_3: $c88:tt, $c89:tt;)?
     $(subj_act_perf_1: $c90:tt, $c91:tt;)?
     $(subj_act_perf_2: $c92:tt, $c93:tt;)?
     $(subj_act_perf_3: $c94:tt, $c95:tt;)?
     $(subj_act_plup_1: $c96:tt, $c97:tt;)?
     $(subj_act_plup_2: $c98:tt, $c99:tt;)?
     $(subj_act_plup_3: $c100:tt, $c101:tt;)?
     $(subj_act_futp_1: $c102:tt, $c103:tt;)?
     $(subj_act_futp_2: $c104:tt, $c105:tt;)?
     $(subj_act_futp_3: $c106:tt, $c107:tt;)?
     $(subj_pass_pres_1: $c108:tt, $c109:tt;)?
     $(subj_pass_pres_2: $c110:tt, $c111:tt;)?
     $(subj_pass_pres_3: $c112:tt, $c113:tt;)?
     $(subj_pass_impf_1: $c114:tt, $c115:tt;)?
     $(subj_pass_impf_2: $c116:tt, $c117:tt;)?
     $(subj_pass_impf_3: $c118:tt, $c119:tt;)?
     $(subj_pass_fut_1: $c120:tt, $c121:tt;)?
     $(subj_pass_fut_2: $c122:tt, $c123:tt;)?
     $(subj_pass_fut_3: $c124:tt, $c125:tt;)?
     $(subj_pass_perf_1: $c126:tt, $c127:tt;)?
     $(subj_pass_perf_2: $c128:tt, $c129:tt;)?
     $(subj_pass_perf_3: $c130:tt, $c131:tt;)?
     $(subj_pass_plup_1: $c132:tt, $c133:tt;)?
     $(subj_pass_plup_2: $c134:tt, $c135:tt;)?
     $(subj_pass_plup_3: $c136:tt, $c137:tt;)?
     $(subj_pass_futp_1: $c138:tt, $c139:tt;)?
     $(subj_pass_futp_2: $c140:tt, $c141:tt;)?
     $(subj_pass_futp_3: $c142:tt, $c143:tt;)?
     $(imperative_act_pres_1: $c144:tt, $c145:tt;)?
     $(imperative_act_pres_2: $c146:tt, $c147:tt;)?
     $(imperative_act_pres_3: $c148:tt, $c149:tt;)?
     $(imperative_act_fut_1: $c150:tt, $c151:tt;)?
     $(imperative_act_fut_2: $c152:tt, $c153:tt;)?
     $(imperative_act_fut_3: $c154:tt, $c155:tt;)?
     $(imperative_pass_pres_1: $c156:tt, $c157:tt;)?
     $(imperative_pass_pres_2: $c158:tt, $c159:tt;)?
     $(imperative_pass_pres_3: $c160:tt, $c161:tt;)?
     $(imperative_pass_fut_1: $c162:tt, $c163:tt;)?
     $(imperative_pass_fut_2: $c164:tt, $c165:tt;)?
     $(imperative_pass_fut_3: $c166:tt, $c167:tt;)?
     $(infinitive_act: $c168:tt, $c169:tt, $c170:tt;)?
     $(infinitive_pass: $c171:tt, $c172:tt, $c173:tt;)?
     $(participle: $c174:tt, $c175:tt, $c176:tt, $c177:tt;)?
     $(gerund: $c178:tt, $c179:tt, $c180:tt, $c181:tt, $c182:tt, $c183:tt;)?
     $(supine: $c184:tt, $c185:tt, $c186:tt, $c187:tt, $c188:tt, $c189:tt;)?
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
                $(flexion::cell!($c154),)? "\t",
                $(flexion::cell!($c155),)? "\t",
                $(flexion::cell!($c156),)? "\t",
                $(flexion::cell!($c157),)? "\t",
                $(flexion::cell!($c158),)? "\t",
                $(flexion::cell!($c159),)? "\t",
                $(flexion::cell!($c160),)? "\t",
                $(flexion::cell!($c161),)? "\t",
                $(flexion::cell!($c162),)? "\t",
                $(flexion::cell!($c163),)? "\t",
                $(flexion::cell!($c164),)? "\t",
                $(flexion::cell!($c165),)? "\t",
                $(flexion::cell!($c166),)? "\t",
                $(flexion::cell!($c167),)? "\t",
                $(flexion::cell!($c168),)? "\t",
                $(flexion::cell!($c169),)? "\t",
                $(flexion::cell!($c170),)? "\t",
                $(flexion::cell!($c171),)? "\t",
                $(flexion::cell!($c172),)? "\t",
                $(flexion::cell!($c173),)? "\t",
                $(flexion::cell!($c174),)? "\t",
                $(flexion::cell!($c175),)? "\t",
                $(flexion::cell!($c176),)? "\t",
                $(flexion::cell!($c177),)? "\t",
                $(flexion::cell!($c178),)? "\t",
                $(flexion::cell!($c179),)? "\t",
                $(flexion::cell!($c180),)? "\t",
                $(flexion::cell!($c181),)? "\t",
                $(flexion::cell!($c182),)? "\t",
                $(flexion::cell!($c183),)? "\t",
                $(flexion::cell!($c184),)? "\t",
                $(flexion::cell!($c185),)? "\t",
                $(flexion::cell!($c186),)? "\t",
                $(flexion::cell!($c187),)? "\t",
                $(flexion::cell!($c188),)? "\t",
                $(flexion::cell!($c189),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
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
/// like that it reads (`act_as_pass`: one value read as another).
const ACT_AS_PASS: [u16; 190] = [
    36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
    108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
    127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 108, 109,
    110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128,
    129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
    167, 171, 172, 173, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185,
    186, 187, 188, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`abl_as_voc`: one value read as another).
const ABL_AS_VOC: [u16; 190] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 183, 183, 184, 185, 186, 187, 189, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pres_as_perf`: one value read as another).
const PRES_AS_PERF: [u16; 190] = [
    18, 19, 20, 21, 22, 23, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 54, 55, 56, 57, 58, 59, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 90,
    91, 92, 93, 94, 95, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 126, 127, 128, 129, 130, 131, 114, 115,
    116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134,
    135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153,
    154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
    173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`perf_as_fut`: one value read as another).
const PERF_AS_FUT: [u16; 190] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 170, 171, 173, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`ind_as_subj`: one value read as another).
const IND_AS_SUBJ: [u16; 190] = [
    72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
    116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134,
    135, 136, 137, 138, 139, 140, 141, 142, 143, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
    84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105,
    106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
    125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162,
    163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181,
    182, 183, 184, 185, 186, 187, 188, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pres_as_fut`: one value read as another).
const PRES_AS_FUT: [u16; 190] = [
    12, 13, 14, 15, 16, 17, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 48, 49, 50, 51, 52, 53, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 84,
    85, 86, 87, 88, 89, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 120, 121, 122, 123, 124, 125, 114, 115,
    116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134,
    135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153,
    154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
    173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`impf_as_futp`: one value read as another).
const IMPF_AS_FUTP: [u16; 190] = [
    0, 1, 2, 3, 4, 5, 30, 31, 32, 33, 34, 35, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 66, 67, 68, 69, 70, 71, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 102, 103, 104, 105, 106, 107, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94,
    95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 138,
    139, 140, 141, 142, 143, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133,
    134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171,
    172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`acc_as_voc`: one value read as another).
const ACC_AS_VOC: [u16; 190] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 183, 182, 183, 184, 185, 186, 189, 188, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pass_as_act`: one value read as another).
const PASS_AS_ACT: [u16; 190] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 72, 73, 74, 75,
    76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
    100, 101, 102, 103, 104, 105, 106, 107, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
    86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 144, 145, 146, 147, 148, 149, 150,
    151, 152, 153, 154, 155, 168, 169, 170, 168, 169, 170, 174, 175, 176, 177, 178, 179, 180, 181,
    182, 183, 184, 185, 186, 187, 188, 189,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`3_as_1`: one value read as another).
const C3_AS_1: [u16; 190] = [
    0, 1, 2, 3, 0, 1, 6, 7, 8, 9, 6, 7, 12, 13, 14, 15, 12, 13, 18, 19, 20, 21, 18, 19, 24, 25, 26,
    27, 24, 25, 30, 31, 32, 33, 30, 31, 36, 37, 38, 39, 36, 37, 42, 43, 44, 45, 42, 43, 48, 49, 50,
    51, 48, 49, 54, 55, 56, 57, 54, 55, 60, 61, 62, 63, 60, 61, 66, 67, 68, 69, 66, 67, 72, 73, 74,
    75, 72, 73, 78, 79, 80, 81, 78, 79, 84, 85, 86, 87, 84, 85, 90, 91, 92, 93, 90, 91, 96, 97, 98,
    99, 96, 97, 102, 103, 104, 105, 102, 103, 108, 109, 110, 111, 108, 109, 114, 115, 116, 117,
    114, 115, 120, 121, 122, 123, 120, 121, 126, 127, 128, 129, 126, 127, 132, 133, 134, 135, 132,
    133, 138, 139, 140, 141, 138, 139, 144, 145, 146, 147, 144, 145, 150, 151, 152, 153, 150, 151,
    156, 157, 158, 159, 156, 157, 162, 163, 164, 165, 162, 163, 168, 169, 170, 171, 172, 173, 174,
    175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `agitō`: 2,609 words.
    Agitō,
    /// Like `modulor`: 357 words.
    Modulor,
    /// Like `habeō`: 274 words.
    Habeō,
    /// Like `agō`: 250 words.
    Agō,
    /// Like `mūniō`: 218 words.
    Mūniō,
    /// Like `quiēscō`: 149 words.
    Quiēscō,
    /// Like `compercō`: 116 words.
    Compercō,
    /// Like `minuō`: 79 words.
    Minuō,
    /// Like `ēvādō`: 63 words.
    Ēvādō,
    /// Like `faciō`: 53 words.
    Faciō,
    /// Like `comprehendō`: 46 words.
    Comprehendō,
    /// Like `teneō`: 45 words.
    Teneō,
    /// Like `petō`: 44 words.
    Petō,
    /// Like `cubō`: 42 words.
    Cubō,
    /// Like `excipiō`: 41 words.
    Excipiō,
    /// Like `compescō`: 36 words.
    Compescō,
    /// Like `impleō`: 29 words.
    Impleō,
    /// Like `repōnō`: 29 words.
    Repōnō,
    /// Like `mittō`: 28 words.
    Mittō,
    /// Like `labōrō`: 28 words.
    Labōrō,
    /// Like `obeō`: 27 words.
    Obeō,
    /// Like `perstringō`: 27 words.
    Perstringō,
    /// Like `coniungō`: 27 words.
    Coniungō,
    /// Like `prōdō`: 26 words.
    Prōdō,
    /// Like `dēdūcō`: 26 words.
    Dēdūcō,
    /// Like `cōnsequor`: 25 words.
    Cōnsequor,
    /// Like `mergō`: 25 words.
    Mergō,
    /// Like `coniciō`: 25 words.
    Coniciō,
    /// Like `gerō`: 25 words.
    Gerō,
    /// Like `mōlior`: 25 words.
    Mōlior,
    /// Like `pellō`: 25 words.
    Pellō,
    /// Like `tueor`: 23 words.
    Tueor,
    /// Like `ārdeō`: 22 words.
    Ārdeō,
    /// Like `praecīdō`: 22 words.
    Praecīdō,
    /// Like `exsarciō`: 22 words.
    Exsarciō,
    /// Like `solvō`: 21 words.
    Solvō,
    /// Like `nāscor`: 21 words.
    Nāscor,
    /// Like `abnuō`: 20 words.
    Abnuō,
    /// Like `cēdō`: 19 words.
    Cēdō,
    /// Like `possideō`: 19 words.
    Possideō,
    /// Like `ēgredior`: 19 words.
    Ēgredior,
    /// Like `referō`: 18 words.
    Referō,
    /// Like `dispiciō`: 17 words.
    Dispiciō,
    /// Like `trahō`: 17 words.
    Trahō,
    /// Like `compingō`: 17 words.
    Compingō,
    /// Like `dēsinō`: 17 words.
    Dēsinō,
    /// Like `nectō`: 16 words.
    Nectō,
    /// Like `corripiō`: 16 words.
    Corripiō,
    /// Like `effundō`: 16 words.
    Effundō,
    /// Like `exigō`: 16 words.
    Exigō,
    /// Like `accumbō`: 16 words.
    Accumbō,
    /// Like `nītor`: 15 words.
    Nītor,
    /// Like `extruō`: 15 words.
    Extruō,
    /// Like `dīcō`: 14 words.
    Dīcō,
    /// Like `regō`: 14 words.
    Regō,
    /// Like `struō`: 14 words.
    Struō,
    /// Like `rumpō`: 14 words.
    Rumpō,
    /// Like `fodiō`: 14 words.
    Fodiō,
    /// Like `ēveniō`: 14 words.
    Ēveniō,
    /// Like `nīvit`: 14 words.
    Nīvit,
    /// Like `tingō`: 13 words.
    Tingō,
    /// Like `illābor`: 13 words.
    Illābor,
    /// Like `ēvehō`: 13 words.
    Ēvehō,
    /// Like `madeō`: 13 words.
    Madeō,
    /// Like `dēfīgō`: 13 words.
    Dēfīgō,
    /// Like `intersaepiō`: 13 words.
    Intersaepiō,
    /// Like `contendō`: 13 words.
    Contendō,
    /// Like `cōnsulō`: 12 words.
    Cōnsulō,
    /// Like `feriō`: 12 words.
    Feriō,
    /// Like `fugiō`: 12 words.
    Fugiō,
    /// Like `scindō`: 12 words.
    Scindō,
    /// Like `fallō`: 12 words.
    Fallō,
    /// Like `adsum`: 12 words.
    Adsum,
    /// Like `misceō`: 11 words.
    Misceō,
    /// Like `contineō`: 11 words.
    Contineō,
    /// Like `opprimō`: 11 words.
    Opprimō,
    /// Like `incidō`: 11 words.
    Incidō,
    /// Like `cernō`: 11 words.
    Cernō,
    /// Like `decet`: 11 words.
    Decet,
    /// Like `percutiō`: 11 words.
    Percutiō,
    /// Like `aperiō`: 11 words.
    Aperiō,
    /// Like `respondeō`: 11 words.
    Respondeō,
    /// Like `tergeō`: 10 words.
    Tergeō,
    /// Like `legō`: 10 words.
    Legō,
    /// Like `terō`: 10 words.
    Terō,
    /// Like `calvor`: 10 words.
    Calvor,
    /// Like `vertō`: 9 words.
    Vertō,
    /// Like `contorqueō`: 9 words.
    Contorqueō,
    /// Like `micō`: 9 words.
    Micō,
    /// Like `inveniō`: 9 words.
    Inveniō,
    /// Like `sternō`: 9 words.
    Sternō,
    /// Like `vīsō`: 9 words.
    Vīsō,
    /// Like `sentiō`: 9 words.
    Sentiō,
    /// Like `mētior`: 9 words.
    Mētior,
    /// Like `īnsiliō`: 9 words.
    Īnsiliō,
    /// Like `interstinguō`: 9 words.
    Interstinguō,
    /// Like `pervincō`: 8 words.
    Pervincō,
    /// Like `concoquō`: 8 words.
    Concoquō,
    /// Like `nūbō`: 8 words.
    Nūbō,
    /// Like `prōfluō`: 8 words.
    Prōfluō,
    /// Like `exorior`: 8 words.
    Exorior,
    /// Like `cohaereō`: 8 words.
    Cohaereō,
    /// Like `recolō`: 7 words.
    Recolō,
    /// Like `gaudeō`: 7 words.
    Gaudeō,
    /// Like `cognōscō`: 7 words.
    Cognōscō,
    /// Like `comedō`: 7 words.
    Comedō,
    /// Like `maneō`: 7 words.
    Maneō,
    /// Like `īnsistō`: 7 words.
    Īnsistō,
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
    /// Like `īnserō`: 7 words.
    Īnserō,
    /// Like `coorior`: 7 words.
    Coorior,
    /// Like `cēnseō`: 6 words.
    Cēnseō,
    /// Like `pergō`: 6 words.
    Pergō,
    /// Like `vīvō`: 6 words.
    Vīvō,
    /// Like `cōnstō`: 6 words.
    Cōnstō,
    /// Like `capiō`: 6 words.
    Capiō,
    /// Like `innectō`: 6 words.
    Innectō,
    /// Like `eximō`: 6 words.
    Eximō,
    /// Like `vinciō`: 6 words.
    Vinciō,
    /// Like `scandō`: 6 words.
    Scandō,
    /// Like `dēfluō`: 6 words.
    Dēfluō,
    /// Like `cōnfugiō`: 6 words.
    Cōnfugiō,
    /// Like `apīscor`: 6 words.
    Apīscor,
    /// Like `sistō`: 6 words.
    Sistō,
    /// Like `discō`: 6 words.
    Discō,
    /// Like `deicō`: 6 words.
    Deicō,
    /// Like `occurrō`: 5 words.
    Occurrō,
    /// Like `sūmō`: 5 words.
    Sūmō,
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
    /// Like `mulceō`: 5 words.
    Mulceō,
    /// Like `sedeō`: 5 words.
    Sedeō,
    /// Like `clueō`: 5 words.
    Clueō,
    /// Like `ambigō`: 5 words.
    Ambigō,
    /// Like `contremīscō`: 5 words.
    Contremīscō,
    /// Like `clangō`: 5 words.
    Clangō,
    /// Like `dēpudet`: 5 words.
    Dēpudet,
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
    /// Like `fruor`: 4 words.
    Fruor,
    /// Like `sum`: 4 words.
    Sum,
    /// Like `secō`: 3 words.
    Secō,
    /// Like `quaerō`: 3 words.
    Quaerō,
    /// Like `tangō`: 3 words.
    Tangō,
    /// Like `adliciō`: 3 words.
    Adliciō,
    /// Like `stō`: 3 words.
    Stō,
    /// Like `dīluō`: 3 words.
    Dīluō,
    /// Like `iubeō`: 3 words.
    Iubeō,
    /// Like `apiō`: 3 words.
    Apiō,
    /// Like `fundō`: 3 words.
    Fundō,
    /// Like `scatō`: 3 words.
    Scatō,
    /// Like `perpōtō`: 3 words.
    Perpōtō,
    /// Like `dēfungor`: 3 words.
    Dēfungor,
    /// Like `pāscō`: 3 words.
    Pāscō,
    /// Like `ōdī`: 3 words.
    Ōdī,
    /// Like `dētergeō`: 3 words.
    Dētergeō,
    /// Like `differō`: 3 words.
    Differō,
    /// Like `antecapiō`: 3 words.
    Antecapiō,
    /// Like `commeminī`: 3 words.
    Commeminī,
    /// Like `contemnō`: 3 words.
    Contemnō,
    /// Like `dētorreō`: 3 words.
    Dētorreō,
    /// Like `orior`: 3 words.
    Orior,
    /// Like `superstō`: 3 words.
    Superstō,
    /// Like `gignō`: 3 words.
    Gignō,
    /// Like `assiliō`: 3 words.
    Assiliō,
    /// Like `convalēscō`: 3 words.
    Convalēscō,
    /// Like `adhērēscō`: 3 words.
    Adhērēscō,
    /// Like `lavō`: 3 words.
    Lavō,
    /// Like `abblandior`: 3 words.
    Abblandior,
    /// Like `effervēscō`: 3 words.
    Effervēscō,
    /// Like `dīlūculat`: 2 words.
    Dīlūculat,
    /// Like `sepultō`: 1 words.
    Sepultō,
    /// Like `alāre`: 1 words.
    Alāre,
    /// Like `coerceo`: 1 words.
    Coerceo,
    /// Like `rēfert`: 1 words.
    Rēfert,
    /// Like `coemo`: 1 words.
    Coemo,
    /// Like `inclāmitō`: 1 words.
    Inclāmitō,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 196] = [
        Conjugation::Agitō,
        Conjugation::Modulor,
        Conjugation::Habeō,
        Conjugation::Agō,
        Conjugation::Mūniō,
        Conjugation::Quiēscō,
        Conjugation::Compercō,
        Conjugation::Minuō,
        Conjugation::Ēvādō,
        Conjugation::Faciō,
        Conjugation::Comprehendō,
        Conjugation::Teneō,
        Conjugation::Petō,
        Conjugation::Cubō,
        Conjugation::Excipiō,
        Conjugation::Compescō,
        Conjugation::Impleō,
        Conjugation::Repōnō,
        Conjugation::Mittō,
        Conjugation::Labōrō,
        Conjugation::Obeō,
        Conjugation::Perstringō,
        Conjugation::Coniungō,
        Conjugation::Prōdō,
        Conjugation::Dēdūcō,
        Conjugation::Cōnsequor,
        Conjugation::Mergō,
        Conjugation::Coniciō,
        Conjugation::Gerō,
        Conjugation::Mōlior,
        Conjugation::Pellō,
        Conjugation::Tueor,
        Conjugation::Ārdeō,
        Conjugation::Praecīdō,
        Conjugation::Exsarciō,
        Conjugation::Solvō,
        Conjugation::Nāscor,
        Conjugation::Abnuō,
        Conjugation::Cēdō,
        Conjugation::Possideō,
        Conjugation::Ēgredior,
        Conjugation::Referō,
        Conjugation::Dispiciō,
        Conjugation::Trahō,
        Conjugation::Compingō,
        Conjugation::Dēsinō,
        Conjugation::Nectō,
        Conjugation::Corripiō,
        Conjugation::Effundō,
        Conjugation::Exigō,
        Conjugation::Accumbō,
        Conjugation::Nītor,
        Conjugation::Extruō,
        Conjugation::Dīcō,
        Conjugation::Regō,
        Conjugation::Struō,
        Conjugation::Rumpō,
        Conjugation::Fodiō,
        Conjugation::Ēveniō,
        Conjugation::Nīvit,
        Conjugation::Tingō,
        Conjugation::Illābor,
        Conjugation::Ēvehō,
        Conjugation::Madeō,
        Conjugation::Dēfīgō,
        Conjugation::Intersaepiō,
        Conjugation::Contendō,
        Conjugation::Cōnsulō,
        Conjugation::Feriō,
        Conjugation::Fugiō,
        Conjugation::Scindō,
        Conjugation::Fallō,
        Conjugation::Adsum,
        Conjugation::Misceō,
        Conjugation::Contineō,
        Conjugation::Opprimō,
        Conjugation::Incidō,
        Conjugation::Cernō,
        Conjugation::Decet,
        Conjugation::Percutiō,
        Conjugation::Aperiō,
        Conjugation::Respondeō,
        Conjugation::Tergeō,
        Conjugation::Legō,
        Conjugation::Terō,
        Conjugation::Calvor,
        Conjugation::Vertō,
        Conjugation::Contorqueō,
        Conjugation::Micō,
        Conjugation::Inveniō,
        Conjugation::Sternō,
        Conjugation::Vīsō,
        Conjugation::Sentiō,
        Conjugation::Mētior,
        Conjugation::Īnsiliō,
        Conjugation::Interstinguō,
        Conjugation::Pervincō,
        Conjugation::Concoquō,
        Conjugation::Nūbō,
        Conjugation::Prōfluō,
        Conjugation::Exorior,
        Conjugation::Cohaereō,
        Conjugation::Recolō,
        Conjugation::Gaudeō,
        Conjugation::Cognōscō,
        Conjugation::Comedō,
        Conjugation::Maneō,
        Conjugation::Īnsistō,
        Conjugation::Augeō,
        Conjugation::Ferveō,
        Conjugation::Īnsīdō,
        Conjugation::Dēpendeō,
        Conjugation::Peragō,
        Conjugation::Īnserō,
        Conjugation::Coorior,
        Conjugation::Cēnseō,
        Conjugation::Pergō,
        Conjugation::Vīvō,
        Conjugation::Cōnstō,
        Conjugation::Capiō,
        Conjugation::Innectō,
        Conjugation::Eximō,
        Conjugation::Vinciō,
        Conjugation::Scandō,
        Conjugation::Dēfluō,
        Conjugation::Cōnfugiō,
        Conjugation::Apīscor,
        Conjugation::Sistō,
        Conjugation::Discō,
        Conjugation::Deicō,
        Conjugation::Occurrō,
        Conjugation::Sūmō,
        Conjugation::Vēnumdō,
        Conjugation::Serviō,
        Conjugation::Adipīscor,
        Conjugation::Astō,
        Conjugation::Occinō,
        Conjugation::Mulceō,
        Conjugation::Sedeō,
        Conjugation::Clueō,
        Conjugation::Ambigō,
        Conjugation::Contremīscō,
        Conjugation::Clangō,
        Conjugation::Dēpudet,
        Conjugation::Caveō,
        Conjugation::Cieō,
        Conjugation::Patior,
        Conjugation::Fīō,
        Conjugation::Relinquō,
        Conjugation::Sapiō,
        Conjugation::Abeō,
        Conjugation::Ēvellō,
        Conjugation::Currō,
        Conjugation::Obtundō,
        Conjugation::Hauriō,
        Conjugation::Comperiō,
        Conjugation::Circumvehor,
        Conjugation::Adnītor,
        Conjugation::Fruor,
        Conjugation::Sum,
        Conjugation::Secō,
        Conjugation::Quaerō,
        Conjugation::Tangō,
        Conjugation::Adliciō,
        Conjugation::Stō,
        Conjugation::Dīluō,
        Conjugation::Iubeō,
        Conjugation::Apiō,
        Conjugation::Fundō,
        Conjugation::Scatō,
        Conjugation::Perpōtō,
        Conjugation::Dēfungor,
        Conjugation::Pāscō,
        Conjugation::Ōdī,
        Conjugation::Dētergeō,
        Conjugation::Differō,
        Conjugation::Antecapiō,
        Conjugation::Commeminī,
        Conjugation::Contemnō,
        Conjugation::Dētorreō,
        Conjugation::Orior,
        Conjugation::Superstō,
        Conjugation::Gignō,
        Conjugation::Assiliō,
        Conjugation::Convalēscō,
        Conjugation::Adhērēscō,
        Conjugation::Lavō,
        Conjugation::Abblandior,
        Conjugation::Effervēscō,
        Conjugation::Dīlūculat,
        Conjugation::Sepultō,
        Conjugation::Alāre,
        Conjugation::Coerceo,
        Conjugation::Rēfert,
        Conjugation::Coemo,
        Conjugation::Inclāmitō,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 196] = [
    // Agitō: 2,609 words (līberō, radiō, digitō, mundō, abaliēnō, picō).
    conjugation!("agitō", 2609, "~ō";
    stem participle_perf_pass: "~āt- like agitō";
    ind_act_pres_1: "~ō", "~āmus";
    ind_act_pres_2: "~ās", "~ātis";
    ind_act_pres_3: "~at", "~ant";
    ind_act_impf_1: "~ābam", "~ābāmus";
    ind_act_impf_2: "~ābās", "~ābātis";
    ind_act_impf_3: "~ābat", "~ābant";
    ind_act_fut_1: "~ābō", "~ābimus";
    ind_act_fut_2: "~ābis", "~ābitis";
    ind_act_fut_3: "~ābit", "~ābunt";
    ind_act_perf_1: "~āvī", "~āvimus";
    ind_act_perf_2: "~āvistī", "~āvistis";
    ind_act_perf_3: "~āvit", "~āvērunt|~āvēre";
    ind_act_plup_1: "~āveram", "~āverāmus";
    ind_act_plup_2: "~āverās", "~āverātis";
    ind_act_plup_3: "~āverat", "~āverant";
    ind_act_futp_1: "~āverō", "~āverimus";
    ind_act_futp_2: "~āveris", "~āveritis";
    ind_act_futp_3: "~āverit", "~āverint";
    ind_pass_pres_1: "~or", "~āmur";
    ind_pass_pres_2: "~āris|~āre", "~āminī";
    ind_pass_pres_3: "~ātur", "~antur";
    ind_pass_impf_1: "~ābar", "~ābāmur";
    ind_pass_impf_2: "~ābāris|~ābāre", "~ābāminī";
    ind_pass_impf_3: "~ābātur", "~ābantur";
    ind_pass_fut_1: "~ābor", "~ābimur";
    ind_pass_fut_2: "~āberis|~ābere", "~ābiminī";
    ind_pass_fut_3: "~ābitur", "~ābuntur";
    subj_act_pres_1: "~em", "~ēmus";
    subj_act_pres_2: "~ēs", "~ētis";
    subj_act_pres_3: "~et", "~ent";
    subj_act_impf_1: "~ārem", "~ārēmus";
    subj_act_impf_2: "~ārēs", "~ārētis";
    subj_act_impf_3: "~āret", "~ārent";
    subj_act_perf_1: "~āverim", "~āverīmus";
    subj_act_perf_2: "~āverīs", "~āverītis";
    subj_act_plup_1: "~āvissem", "~āvissēmus";
    subj_act_plup_2: "~āvissēs", "~āvissētis";
    subj_act_plup_3: "~āvisset", "~āvissent";
    subj_pass_pres_1: "~er", "~ēmur";
    subj_pass_pres_2: "~ēris|~ēre", "~ēminī";
    subj_pass_pres_3: "~ētur", "~entur";
    subj_pass_impf_1: "~ārer", "~ārēmur";
    subj_pass_impf_2: "~ārēris|~ārēre", "~ārēminī";
    subj_pass_impf_3: "~ārētur", "~ārentur";
    imperative_act_pres_2: "~ā", "~āte";
    imperative_act_fut_2: "~ātō", "~ātōte";
    imperative_act_fut_3: _, "~antō";
    imperative_pass_pres_2: "~āre", _;
    imperative_pass_fut_2: "~ātor", _;
    imperative_pass_fut_3: _, "~antor";
    infinitive_act: _, "~āvisse", _;
    infinitive_pass: "~ārī", _, _;
    participle: "~āns", _, _, "~andus";
    gerund: _, "~andī", "~andō", "~andum", _, _;
    ),
    // Modulor: 357 words (for, cōnor, minor, māchinor, vōciferor, lupor). Like Agitō, except in 37 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 sg, ind_pass_pres_3 pl, ….
    conjugation!("modulor", 357, "~or", like Agitō as ACT_AS_PASS;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_act_pres_2: _, "~āminī";
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: _, derived;
    infinitive_act: _, "~āsse", _;
    infinitive_pass: derived, _, _;
    participle: _, derived, _, _;
    ),
    // Habeō: 274 words (abhorreō, moneō, aveō, oleō, stupeō, egeō).
    conjugation!("habeō", 274, "~eō";
    stem participle_perf_pass: "~it- like agitō";
    ind_act_pres_1: "~eō", "~ēmus";
    ind_act_pres_2: "~ēs", "~ētis";
    ind_act_pres_3: "~et", "~ent";
    ind_act_impf_1: "~ēbam", "~ēbāmus";
    ind_act_impf_2: "~ēbās", "~ēbātis";
    ind_act_impf_3: "~ēbat", "~ēbant";
    ind_act_fut_1: "~ēbō", "~ēbimus";
    ind_act_fut_2: "~ēbis", "~ēbitis";
    ind_act_fut_3: "~ēbit", "~ēbunt";
    ind_act_perf_1: "~uī", "~uimus";
    ind_act_perf_2: "~uistī", "~uistis";
    ind_act_perf_3: "~uit", "~uērunt|~uēre";
    ind_act_plup_1: "~ueram", "~uerāmus";
    ind_act_plup_2: "~uerās", "~uerātis";
    ind_act_plup_3: "~uerat", "~uerant";
    ind_act_futp_1: "~uerō", "~uerimus";
    ind_act_futp_2: "~ueris", "~ueritis";
    ind_act_futp_3: "~uerit", "~uerint";
    ind_pass_pres_1: "~eor", "~ēmur";
    ind_pass_pres_2: "~ēris", "~ēminī";
    ind_pass_pres_3: "~ētur", "~entur";
    ind_pass_impf_1: "~ēbar", "~ēbāmur";
    ind_pass_impf_2: "~ēbāris", "~ēbāminī";
    ind_pass_impf_3: "~ēbātur", "~ēbantur";
    ind_pass_fut_1: "~ēbor", "~ēbimur";
    ind_pass_fut_2: "~ēberis", "~ēbiminī";
    ind_pass_fut_3: "~ēbitur", "~ēbuntur";
    subj_act_pres_1: "~eam", "~eāmus";
    subj_act_pres_2: "~eās", "~eātis";
    subj_act_pres_3: "~eat", "~eant";
    subj_act_impf_1: "~ērem", "~ērēmus";
    subj_act_impf_2: "~ērēs", "~ērētis";
    subj_act_impf_3: "~ēret", "~ērent";
    subj_act_perf_1: "~uerim", "~uerīmus";
    subj_act_perf_2: "~uerīs", "~uerītis";
    subj_act_plup_1: "~uissem", "~uissēmus";
    subj_act_plup_2: "~uissēs", "~uissētis";
    subj_act_plup_3: "~uisset", "~uissent";
    subj_pass_pres_1: "~ear", "~eāmur";
    subj_pass_pres_2: "~eāris", "~eāminī";
    subj_pass_pres_3: "~eātur", "~eantur";
    subj_pass_impf_1: "~ērer", "~ērēmur";
    subj_pass_impf_2: "~ērēris", "~ērēminī";
    subj_pass_impf_3: "~ērētur", "~ērentur";
    imperative_act_pres_2: "~ē", "~ēte";
    imperative_act_fut_2: "~ētō", "~ētōte";
    imperative_act_fut_3: _, "~entō";
    imperative_pass_pres_2: "~ēre", _;
    imperative_pass_fut_2: "~ētor", _;
    imperative_pass_fut_3: _, "~entor";
    infinitive_act: _, "~uisse", _;
    infinitive_pass: "~ērī", _, _;
    participle: "~ēns", _, _, "~endus";
    gerund: _, "~endī", "~endō", "~endum", _, _;
    ),
    // Agō: 250 words (ferō, eō, vortō, sūmō, edō, vertō). Like Habeō, except in 86 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("agō", 250, "~ō", like Habeō;
    stem participle_perf_pass: "~t- like agitō";
    ind_act_pres_1: "~ō", "~imus";
    ind_act_pres_2: "~is", "~itis";
    ind_act_pres_3: "~it", "~unt";
    ind_act_fut_1: "~am", "~ēmus";
    ind_act_fut_2: "~ēs", "~ētis";
    ind_act_fut_3: "~et", "~ent";
    ind_act_perf_1: "~sī", "~simus";
    ind_act_perf_2: "~sistī", "~sistis";
    ind_act_perf_3: "~sit", "~sērunt|~sēre";
    ind_act_plup_1: "~seram", "~serāmus";
    ind_act_plup_2: "~serās", "~serātis";
    ind_act_plup_3: "~serat", "~serant";
    ind_act_futp_1: "~serō", "~serimus";
    ind_act_futp_2: "~seris", "~seritis";
    ind_act_futp_3: "~serit", "~serint";
    ind_pass_pres_1: "~or", "~imur";
    ind_pass_pres_2: "~eris", "~iminī";
    ind_pass_pres_3: "~itur", "~untur";
    ind_pass_fut_1: "~ar", "~ēmur";
    ind_pass_fut_2: "~ēris", "~ēminī";
    ind_pass_fut_3: "~ētur", "~entur";
    subj_act_pres_1: "~am", "~āmus";
    subj_act_pres_2: "~ās", "~ātis";
    subj_act_pres_3: "~at", "~ant";
    subj_act_impf_1: "~erem", "~erēmus";
    subj_act_impf_2: "~erēs", "~erētis";
    subj_act_impf_3: "~eret", "~erent";
    subj_act_perf_1: "~serim", "~serīmus";
    subj_act_perf_2: "~serīs", "~serītis";
    subj_act_plup_1: "~sissem", "~sissēmus";
    subj_act_plup_2: "~sissēs", "~sissētis";
    subj_act_plup_3: "~sisset", "~sissent";
    subj_pass_pres_1: "~ar", "~āmur";
    subj_pass_pres_2: "~āris", "~āminī";
    subj_pass_pres_3: "~ātur", "~antur";
    subj_pass_impf_1: "~erer", "~erēmur";
    subj_pass_impf_2: "~erēris", "~erēminī";
    subj_pass_impf_3: "~erētur", "~erentur";
    imperative_act_pres_2: "~e", "~ite";
    imperative_act_fut_2: "~itō", "~itōte";
    imperative_act_fut_3: _, "~untō";
    imperative_pass_pres_2: "~ere", _;
    imperative_pass_fut_2: "~itor", _;
    imperative_pass_fut_3: _, "~untor";
    infinitive_act: _, "~sisse", _;
    infinitive_pass: "~ī", _, _;
    ),
    // Mūniō: 218 words (fīniō, hirriō, bombiō, poliō, audiō, odiō).
    conjugation!("mūniō", 218, "~iō";
    stem participle_perf_pass: "~īt- like agitō";
    ind_act_pres_1: "~iō", "~īmus";
    ind_act_pres_2: "~īs", "~ītis";
    ind_act_pres_3: "~it", "~iunt";
    ind_act_impf_1: "~iēbam", "~iēbāmus";
    ind_act_impf_2: "~iēbās", "~iēbātis";
    ind_act_impf_3: "~iēbat", "~iēbant";
    ind_act_fut_1: "~iam", "~iēmus";
    ind_act_fut_2: "~iēs", "~iētis";
    ind_act_fut_3: "~iet", "~ient";
    ind_act_perf_1: "~īvī", "~īvimus";
    ind_act_perf_2: "~īvistī", "~īvistis";
    ind_act_perf_3: "~īvit", "~īvērunt";
    ind_act_plup_1: "~īveram", "~īverāmus";
    ind_act_plup_2: "~īverās", "~īverātis";
    ind_act_plup_3: "~īverat", "~īverant";
    ind_act_futp_1: "~īverō", "~īverimus";
    ind_act_futp_2: "~īveris", "~īveritis";
    ind_act_futp_3: "~īverit", "~īverint";
    ind_pass_pres_1: "~ior", "~īmur";
    ind_pass_pres_2: "~īris|~īre", "~īminī";
    ind_pass_pres_3: "~ītur", "~iuntur";
    ind_pass_impf_1: "~iēbar", "~iēbāmur";
    ind_pass_impf_2: "~iēbāris|~iēbāre", "~iēbāminī";
    ind_pass_impf_3: "~iēbātur", "~iēbantur";
    ind_pass_fut_1: "~iar", "~iēmur";
    ind_pass_fut_2: "~iēris|~iēre", "~iēminī";
    ind_pass_fut_3: "~iētur", "~ientur";
    subj_act_pres_1: "~iam", "~iāmus";
    subj_act_pres_2: "~iās", "~iātis";
    subj_act_pres_3: "~iat", "~iant";
    subj_act_impf_1: "~īrem", "~īrēmus";
    subj_act_impf_2: "~īrēs", "~īrētis";
    subj_act_impf_3: "~īret", "~īrent";
    subj_act_perf_1: "~īverim", "~īverīmus";
    subj_act_perf_2: "~īverīs", "~īverītis";
    subj_act_plup_1: "~īvissem", "~īvissēmus";
    subj_act_plup_2: "~īvissēs", "~īvissētis";
    subj_act_plup_3: "~īvisset", "~īvissent";
    subj_pass_pres_1: "~iar", "~iāmur";
    subj_pass_pres_2: "~iāris|~iāre", "~iāminī";
    subj_pass_pres_3: "~iātur", "~iantur";
    subj_pass_impf_1: "~īrer", "~īrēmur";
    subj_pass_impf_2: "~īrēris|~īrēre", "~īrēminī";
    subj_pass_impf_3: "~īrētur", "~īrentur";
    imperative_act_pres_2: "~ī", "~īte";
    imperative_act_fut_2: "~ītō", "~ītōte";
    imperative_act_fut_3: _, "~iuntō";
    imperative_pass_pres_2: "~īre", _;
    imperative_pass_fut_2: "~ītor", _;
    imperative_pass_fut_3: _, "~iuntor";
    infinitive_act: _, "~īvisse", _;
    infinitive_pass: "~īrī", _, _;
    participle: "~iēns", _, _, "~iendus|~iundus";
    gerund: _, "~iendī", "~iendō", "~iendum", _, _;
    ),
    // Quiēscō: 149 words (albēscō, grandēscō, siccēscō, alēscō, mānēscō, cornēscō).
    conjugation!("quiēscō", 149, "~scō";
    stem participle_perf_pass: "~t- like agitō";
    ind_act_pres_1: "~scō", "~scimus";
    ind_act_pres_2: "~scis", "~scitis";
    ind_act_pres_3: "~scit", "~scunt";
    ind_act_impf_1: "~scēbam", "~scēbāmus";
    ind_act_impf_2: "~scēbās", "~scēbātis";
    ind_act_impf_3: "~scēbat", "~scēbant";
    ind_act_fut_1: "~scam", "~scēmus";
    ind_act_fut_2: "~scēs", "~scētis";
    ind_act_fut_3: "~scet", "~scent";
    ind_act_perf_1: "~vī", "~vimus";
    ind_act_perf_2: "~vistī", "~vistis";
    ind_act_perf_3: "~vit", "~vērunt";
    ind_act_plup_1: "~veram", "~verāmus";
    ind_act_plup_2: "~verās", "~verātis";
    ind_act_plup_3: "~verat", "~verant";
    ind_act_futp_1: "~verō", "~verimus";
    ind_act_futp_2: "~veris", "~veritis";
    ind_act_futp_3: "~verit", "~verint";
    ind_pass_pres_1: "~scor", "~scimur";
    ind_pass_pres_2: "~sceris", "~sciminī";
    ind_pass_pres_3: "~scitur", "~scuntur";
    ind_pass_impf_1: "~scēbar", "~scēbāmur";
    ind_pass_impf_2: "~scēbāris", "~scēbāminī";
    ind_pass_impf_3: "~scēbātur", "~scēbantur";
    ind_pass_fut_1: "~scar", "~scēmur";
    ind_pass_fut_2: "~scēris", "~scēminī";
    ind_pass_fut_3: "~scētur", "~scentur";
    subj_act_pres_1: "~scam", "~scāmus";
    subj_act_pres_2: "~scās", "~scātis";
    subj_act_pres_3: "~scat", "~scant";
    subj_act_impf_1: "~scerem", "~scerēmus";
    subj_act_impf_2: "~scerēs", "~scerētis";
    subj_act_impf_3: "~sceret", "~scerent";
    subj_act_perf_1: "~verim", "~verīmus";
    subj_act_perf_2: "~verīs", "~verītis";
    subj_act_plup_1: "~vissem", "~vissēmus";
    subj_act_plup_2: "~vissēs", "~vissētis";
    subj_act_plup_3: "~visset", "~vissent";
    subj_pass_pres_1: "~scar", "~scāmur";
    subj_pass_pres_2: "~scāris", "~scāminī";
    subj_pass_pres_3: "~scātur", "~scantur";
    subj_pass_impf_1: "~scerer", "~scerēmur";
    subj_pass_impf_2: "~scerēris", "~scerēminī";
    subj_pass_impf_3: "~scerētur", "~scerentur";
    imperative_act_pres_2: "~sce", "~scite";
    imperative_act_fut_2: "~scitō", "~scitōte";
    imperative_act_fut_3: _, "~scuntō";
    imperative_pass_pres_2: "~scere", _;
    imperative_pass_fut_2: "~scitor", _;
    imperative_pass_fut_3: _, "~scuntor";
    infinitive_act: _, "~visse", _;
    infinitive_pass: "~scī", _, _;
    participle: "~scēns", _, _, "~scendus";
    gerund: _, "~scendī", "~scendō", "~scendum", _, _;
    ),
    // Compercō: 116 words (crūdēscō, vigēscō, cānēscō, senēscō, dulcēscō, tumēscō). Like Agō, except in 77 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("compercō", 116, "~cō", like Agō as ABL_AS_VOC;
    ind_act_pres_1: "~cō", "~cimus";
    ind_act_pres_2: "~cis", "~citis";
    ind_act_pres_3: "~cit", "~cunt";
    ind_act_impf_1: "~cēbam", "~cēbāmus";
    ind_act_impf_2: "~cēbās", "~cēbātis";
    ind_act_impf_3: "~cēbat", "~cēbant";
    ind_act_fut_1: "~cam", "~cēmus";
    ind_act_fut_2: "~cēs", "~cētis";
    ind_act_fut_3: "~cet", "~cent";
    ind_pass_pres_1: "~cor", "~cimur";
    ind_pass_pres_2: "~ceris|~cere", "~ciminī";
    ind_pass_pres_3: "~citur", "~cuntur";
    ind_pass_impf_1: "~cēbar", "~cēbāmur";
    ind_pass_impf_2: "~cēbāris|~cēbāre", "~cēbāminī";
    ind_pass_impf_3: "~cēbātur", "~cēbantur";
    ind_pass_fut_1: "~car", "~cēmur";
    ind_pass_fut_2: "~cēris|~cēre", "~cēminī";
    ind_pass_fut_3: "~cētur", "~centur";
    subj_act_pres_1: "~cam", "~cāmus";
    subj_act_pres_2: "~cās", "~cātis";
    subj_act_pres_3: "~cat", "~cant";
    subj_act_impf_1: "~cerem", "~cerēmus";
    subj_act_impf_2: "~cerēs", "~cerētis";
    subj_act_impf_3: "~ceret", "~cerent";
    subj_pass_pres_1: "~car", "~cāmur";
    subj_pass_pres_2: "~cāris|~cāre", "~cāminī";
    subj_pass_pres_3: "~cātur", "~cantur";
    subj_pass_impf_1: "~cerer", "~cerēmur";
    subj_pass_impf_2: "~cerēris|~cerēre", "~cerēminī";
    subj_pass_impf_3: "~cerētur", "~cerentur";
    imperative_act_pres_2: "~ce", "~cite";
    imperative_act_fut_2: "~citō", "~citōte";
    imperative_act_fut_3: _, "~cuntō";
    imperative_pass_pres_2: "~cere", _;
    imperative_pass_fut_2: "~citor", _;
    imperative_pass_fut_3: _, "~cuntor";
    infinitive_pass: "~cī", _, _;
    participle: "~cēns", derived, derived, "~cendus|~cundus";
    gerund: _, "~cendī", "~cendō", "~cendum", _, _;
    supine: _, _, _, derived, _, _;
    ),
    // Minuō: 79 words (nuō, minuō, futuō, innuō, spuō, luō). Like Habeō, except in 72 cells: ind_act_pres_1 sg, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ….
    conjugation!("minuō", 79, "~uō", like Habeō as PRES_AS_PERF;
    stem participle_perf_pass: "~ūt- like agitō";
    ind_act_pres_1: "~uō", _;
    ind_act_pres_2: "~uis", "~uitis";
    ind_act_pres_3: _, "~uunt";
    ind_act_impf_1: "~uēbam", "~uēbāmus";
    ind_act_impf_2: "~uēbās", "~uēbātis";
    ind_act_impf_3: "~uēbat", "~uēbant";
    ind_act_fut_1: "~uam", "~uēmus";
    ind_act_fut_2: "~uēs", "~uētis";
    ind_act_fut_3: "~uet", "~uent";
    ind_pass_pres_1: "~uor", "~uimur";
    ind_pass_pres_2: "~ueris|~uere", "~uiminī";
    ind_pass_pres_3: "~uitur", "~uuntur";
    ind_pass_impf_1: "~uēbar", "~uēbāmur";
    ind_pass_impf_2: "~uēbāris|~uēbāre", "~uēbāminī";
    ind_pass_impf_3: "~uēbātur", "~uēbantur";
    ind_pass_fut_1: "~uar", "~uēmur";
    ind_pass_fut_2: "~uēris|~uēre", "~uēminī";
    ind_pass_fut_3: "~uētur", "~uentur";
    subj_act_pres_1: "~uam", "~uāmus";
    subj_act_pres_2: "~uās", "~uātis";
    subj_act_pres_3: "~uat", "~uant";
    subj_act_impf_1: "~uerem", "~uerēmus";
    subj_act_impf_2: "~uerēs", "~uerētis";
    subj_act_impf_3: "~ueret", "~uerent";
    subj_pass_pres_1: "~uar", "~uāmur";
    subj_pass_pres_2: "~uāris|~uāre", "~uāminī";
    subj_pass_pres_3: "~uātur", "~uantur";
    subj_pass_impf_1: "~uerer", "~uerēmur";
    subj_pass_impf_2: "~uerēris|~uerēre", "~uerēminī";
    subj_pass_impf_3: "~uerētur", "~uerentur";
    imperative_act_pres_2: "~ue", "~uite";
    imperative_act_fut_2: "~uitō", "~uitōte";
    imperative_act_fut_3: _, "~uuntō";
    imperative_pass_pres_2: "~uere", _;
    imperative_pass_fut_2: "~uitor", _;
    imperative_pass_fut_3: _, "~uuntor";
    infinitive_pass: "~uī", _, _;
    participle: "~uēns", _, _, "~uendus";
    gerund: _, "~uendī", "~uendō", "~uendum", _, _;
    ),
    // Ēvādō: 63 words (lūdō, rādō, ēlūdō, ēvādō, extrūdō, rīdō). Like Agō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("ēvādō", 63, "~dō", like Agō;
    stem participle_perf_pass: "~s- like agitō";
    ind_act_pres_1: "~dō", "~dimus";
    ind_act_pres_2: "~dis", "~ditis";
    ind_act_pres_3: "~dit", "~dunt";
    ind_act_impf_1: "~dēbam", "~dēbāmus";
    ind_act_impf_2: "~dēbās", "~dēbātis";
    ind_act_impf_3: "~dēbat", "~dēbant";
    ind_act_fut_1: "~dam", "~dēmus";
    ind_act_fut_2: "~dēs", "~dētis";
    ind_act_fut_3: "~det", "~dent";
    ind_pass_pres_1: "~dor", "~dimur";
    ind_pass_pres_2: "~deris|~dere", "~diminī";
    ind_pass_pres_3: "~ditur", "~duntur";
    ind_pass_impf_1: "~dēbar", "~dēbāmur";
    ind_pass_impf_2: "~dēbāris|~dēbāre", "~dēbāminī";
    ind_pass_impf_3: "~dēbātur", "~dēbantur";
    ind_pass_fut_1: "~dar", "~dēmur";
    ind_pass_fut_2: "~dēris|~dēre", "~dēminī";
    ind_pass_fut_3: "~dētur", "~dentur";
    subj_act_pres_1: "~dam", "~dāmus";
    subj_act_pres_2: "~dās", "~dātis";
    subj_act_pres_3: "~dat", "~dant";
    subj_act_impf_1: "~derem", "~derēmus";
    subj_act_impf_2: "~derēs", "~derētis";
    subj_act_impf_3: "~deret", "~derent";
    subj_pass_pres_1: "~dar", "~dāmur";
    subj_pass_pres_2: "~dāris|~dāre", "~dāminī";
    subj_pass_pres_3: "~dātur", "~dantur";
    subj_pass_impf_1: "~derer", "~derēmur";
    subj_pass_impf_2: "~derēris|~derēre", "~derēminī";
    subj_pass_impf_3: "~derētur", "~derentur";
    imperative_act_pres_2: "~de", "~dite";
    imperative_act_fut_2: "~ditō", "~ditōte";
    imperative_act_fut_3: _, "~duntō";
    imperative_pass_pres_2: "~dere", _;
    imperative_pass_fut_2: "~ditor", _;
    imperative_pass_fut_3: _, "~duntor";
    infinitive_pass: "~dī", _, _;
    participle: "~dēns", _, _, "~dendus|~dundus";
    gerund: _, "~dendī", "~dendō", "~dendum", _, _;
    ),
    // Faciō: 53 words (faciō, patefaciō, satisfaciō, candefaciō, abolēfaciō, labefaciō).
    conjugation!("faciō", 53, "~aciō";
    stem participle_perf_pass: "~act- like agitō";
    ind_act_pres_1: "~aciō", "~acimus";
    ind_act_pres_2: "~acis", "~acitis";
    ind_act_pres_3: "~acit", "~aciunt";
    ind_act_impf_1: "~aciēbam", "~aciēbāmus";
    ind_act_impf_2: "~aciēbās", "~aciēbātis";
    ind_act_impf_3: "~aciēbat", "~aciēbant";
    ind_act_fut_1: "~aciam", "~aciēmus";
    ind_act_fut_2: "~aciēs", "~aciētis";
    ind_act_fut_3: "~aciet", "~acient";
    ind_act_perf_1: "~ēcī", "~ēcimus";
    ind_act_perf_2: "~ēcistī", "~ēcistis";
    ind_act_perf_3: "~ēcit", "~ēcērunt|~ēcēre";
    ind_act_plup_1: "~ēceram", "~ēcerāmus";
    ind_act_plup_2: "~ēcerās", "~ēcerātis";
    ind_act_plup_3: "~ēcerat", "~ēcerant";
    ind_act_futp_1: "~ēcerō", "~ēcerimus";
    ind_act_futp_2: "~ēceris", "~ēceritis";
    ind_act_futp_3: "~ēcerit", "~ēcerint";
    ind_pass_pres_1: "~īō", "~īmus";
    ind_pass_pres_2: "~īs", "~ītis";
    ind_pass_pres_3: "~it", "~īunt";
    ind_pass_impf_1: "~īēbam", "~īēbāmus";
    ind_pass_impf_2: "~īēbās", "~īēbātis";
    ind_pass_impf_3: "~īēbat", "~īēbant";
    ind_pass_fut_1: "~īam", "~īēmus";
    ind_pass_fut_2: "~īēs", "~īētis";
    ind_pass_fut_3: "~īet", "~īent";
    subj_act_pres_1: "~aciam", "~aciāmus";
    subj_act_pres_2: "~aciās", "~aciātis";
    subj_act_pres_3: "~aciat", "~aciant";
    subj_act_impf_1: "~acerem", "~acerēmus";
    subj_act_impf_2: "~acerēs", "~acerētis";
    subj_act_impf_3: "~aceret", "~acerent";
    subj_act_perf_1: "~ēcerim", "~ēcerīmus";
    subj_act_perf_2: "~ēcerīs", "~ēcerītis";
    subj_act_plup_1: "~ēcissem", "~ēcissēmus";
    subj_act_plup_2: "~ēcissēs", "~ēcissētis";
    subj_act_plup_3: "~ēcisset", "~ēcissent";
    subj_pass_pres_1: "~īam", "~īāmus";
    subj_pass_pres_2: "~īās", "~īātis";
    subj_pass_pres_3: "~īat", "~īant";
    subj_pass_impf_1: "~ierem", "~ierēmus";
    subj_pass_impf_2: "~ierēs", "~ierētis";
    subj_pass_impf_3: "~ieret", "~ierent";
    imperative_act_pres_2: "~ace", "~acite";
    imperative_act_fut_2: "~acitō", "~acitōte";
    imperative_act_fut_3: _, "~aciuntō";
    imperative_pass_pres_2: "~ī", "~īte";
    imperative_pass_fut_2: "~ītō", "~ītōte";
    imperative_pass_fut_3: _, "~īuntō";
    infinitive_act: "~acere", "~ēcisse", _;
    infinitive_pass: "~ierī", _, _;
    participle: "~aciēns", _, _, "~aciendus|~aciundus";
    gerund: _, "~aciendī", "~aciendō", "~aciendum", _, _;
    ),
    // Comprehendō: 46 words (dēfendō, comprehendō, fendō, dēpendō, pendō, suspendō).
    conjugation!("comprehendō", 46, "~e~dō";
    stem participle_perf_pass: "~ē~s- like agitō";
    ind_act_pres_1: "~e~dō", "~e~dimus";
    ind_act_pres_2: "~e~dis", "~e~ditis";
    ind_act_pres_3: "~e~dit", "~e~dunt";
    ind_act_impf_1: "~e~dēbam", "~e~dēbāmus";
    ind_act_impf_2: "~e~dēbās", "~e~dēbātis";
    ind_act_impf_3: "~e~dēbat", "~e~dēbant";
    ind_act_fut_1: "~e~dam", "~e~dēmus";
    ind_act_fut_2: "~e~dēs", "~e~dētis";
    ind_act_fut_3: "~e~det", "~e~dent";
    ind_act_perf_1: "~e~dī", "~e~dimus";
    ind_act_perf_2: "~e~distī", "~e~distis";
    ind_act_perf_3: "~e~dit", "~e~dērunt|~e~dēre";
    ind_act_plup_1: "~e~deram", "~e~derāmus";
    ind_act_plup_2: "~e~derās", "~e~derātis";
    ind_act_plup_3: "~e~derat", "~e~derant";
    ind_act_futp_1: "~e~derō", "~e~derimus";
    ind_act_futp_2: "~e~deris", "~e~deritis";
    ind_act_futp_3: "~e~derit", "~e~derint";
    ind_pass_pres_1: "~e~dor", "~e~dimur";
    ind_pass_pres_2: "~e~deris|~e~dere", "~e~diminī";
    ind_pass_pres_3: "~e~ditur", "~e~duntur";
    ind_pass_impf_1: "~e~dēbar", "~e~dēbāmur";
    ind_pass_impf_2: "~e~dēbāris|~e~dēbāre", "~e~dēbāminī";
    ind_pass_impf_3: "~e~dēbātur", "~e~dēbantur";
    ind_pass_fut_1: "~e~dar", "~e~dēmur";
    ind_pass_fut_2: "~e~dēris|~e~dēre", "~e~dēminī";
    ind_pass_fut_3: "~e~dētur", "~e~dentur";
    subj_act_pres_1: "~e~dam", "~e~dāmus";
    subj_act_pres_2: "~e~dās", "~e~dātis";
    subj_act_pres_3: "~e~dat", "~e~dant";
    subj_act_impf_1: "~e~derem", "~e~derēmus";
    subj_act_impf_2: "~e~derēs", "~e~derētis";
    subj_act_impf_3: "~e~deret", "~e~derent";
    subj_act_perf_1: "~e~derim", "~e~derīmus";
    subj_act_perf_2: "~e~derīs", "~e~derītis";
    subj_act_plup_1: "~e~dissem", "~e~dissēmus";
    subj_act_plup_2: "~e~dissēs", "~e~dissētis";
    subj_act_plup_3: "~e~disset", "~e~dissent";
    subj_pass_pres_1: "~e~dar", "~e~dāmur";
    subj_pass_pres_2: "~e~dāris|~e~dāre", "~e~dāminī";
    subj_pass_pres_3: "~e~dātur", "~e~dantur";
    subj_pass_impf_1: "~e~derer", "~e~derēmur";
    subj_pass_impf_2: "~e~derēris|~e~derēre", "~e~derēminī";
    subj_pass_impf_3: "~e~derētur", "~e~derentur";
    imperative_act_pres_2: "~e~de", "~e~dite";
    imperative_act_fut_2: "~e~ditō", "~e~ditōte";
    imperative_act_fut_3: _, "~e~duntō";
    imperative_pass_pres_2: "~e~dere", _;
    imperative_pass_fut_2: "~e~ditor", _;
    imperative_pass_fut_3: _, "~e~duntor";
    infinitive_act: _, "~e~disse", _;
    infinitive_pass: "~e~dī", _, _;
    participle: "~e~dēns", _, _, "~e~dendus|~e~dundus";
    gerund: _, "~e~dendī", "~e~dendō", "~e~dendum", _, _;
    ),
    // Teneō: 45 words (abarceō, doceō, teneō, addoceō, adiaceō, subiaceō). Like Habeō, except in 0 cells: .
    conjugation!("teneō", 45, "~eō", like Habeō;
    stem participle_perf_pass: "~t- like agitō";
    ),
    // Petō: 44 words (petō, impetō, competō, dīrigō, ērigō, dīligō). Like Agō, except in 35 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("petō", 44, "~ō", like Agō;
    stem participle_perf_pass: "~īt- like agitō";
    ind_act_perf_1: "~īvī", "~īvimus";
    ind_act_perf_2: "~īvistī", "~īvistis";
    ind_act_perf_3: "~īvit", "~īvērunt";
    ind_act_plup_1: "~īveram", "~īverāmus";
    ind_act_plup_2: "~īverās", "~īverātis";
    ind_act_plup_3: "~īverat", "~īverant";
    ind_act_futp_1: "~īverō", "~īverimus";
    ind_act_futp_2: "~īveris", "~īveritis";
    ind_act_futp_3: "~īverit", "~īverint";
    ind_pass_pres_2: "~eris|~ere", _;
    ind_pass_impf_2: "~ēbāris|~ēbāre", _;
    ind_pass_fut_2: "~ēris|~ēre", _;
    subj_act_perf_1: "~īverim", "~īverīmus";
    subj_act_perf_2: "~īverīs", "~īverītis";
    subj_act_plup_1: "~īvissem", "~īvissēmus";
    subj_act_plup_2: "~īvissēs", "~īvissētis";
    subj_act_plup_3: "~īvisset", "~īvissent";
    subj_pass_pres_2: "~āris|~āre", _;
    subj_pass_impf_2: "~erēris|~erēre", _;
    infinitive_act: _, "~īvisse", _;
    participle: _, _, _, "~endus|~undus";
    ),
    // Cubō: 42 words (vetō, sonō, domō, incubō, tonō, votō). Like Agitō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("cubō", 42, "~ō", like Agitō;
    stem participle_perf_pass: "~it- like agitō";
    ind_act_perf_1: "~uī", "~uimus";
    ind_act_perf_2: "~uistī", "~uistis";
    ind_act_perf_3: "~uit", "~uērunt|~uēre";
    ind_act_plup_1: "~ueram", "~uerāmus";
    ind_act_plup_2: "~uerās", "~uerātis";
    ind_act_plup_3: "~uerat", "~uerant";
    ind_act_futp_1: "~uerō", "~uerimus";
    ind_act_futp_2: "~ueris", "~ueritis";
    ind_act_futp_3: "~uerit", "~uerint";
    subj_act_perf_1: "~uerim", "~uerīmus";
    subj_act_perf_2: "~uerīs", "~uerītis";
    subj_act_plup_1: "~uissem", "~uissēmus";
    subj_act_plup_2: "~uissēs", "~uissētis";
    subj_act_plup_3: "~uisset", "~uissent";
    infinitive_act: _, "~uisse", _;
    ),
    // Excipiō: 41 words (praeficiō, accipiō, excipiō, recipiō, suscipiō, interficiō).
    conjugation!("excipiō", 41, "~i~iō";
    stem participle_perf_pass: "~e~t- like agitō";
    ind_act_pres_1: "~i~iō", "~i~imus";
    ind_act_pres_2: "~i~is", "~i~itis";
    ind_act_pres_3: "~i~it", "~i~iunt";
    ind_act_impf_1: "~i~iēbam", "~i~iēbāmus";
    ind_act_impf_2: "~i~iēbās", "~i~iēbātis";
    ind_act_impf_3: "~i~iēbat", "~i~iēbant";
    ind_act_fut_1: "~i~iam", "~i~iēmus";
    ind_act_fut_2: "~i~iēs", "~i~iētis";
    ind_act_fut_3: "~i~iet", "~i~ient";
    ind_act_perf_1: "~ē~ī", "~ē~imus";
    ind_act_perf_2: "~ē~istī", "~ē~istis";
    ind_act_perf_3: "~ē~it", "~ē~ērunt|~ē~ēre";
    ind_act_plup_1: "~ē~eram", "~ē~erāmus";
    ind_act_plup_2: "~ē~erās", "~ē~erātis";
    ind_act_plup_3: "~ē~erat", "~ē~erant";
    ind_act_futp_1: "~ē~erō", "~ē~erimus";
    ind_act_futp_2: "~ē~eris", "~ē~eritis";
    ind_act_futp_3: "~ē~erit", "~ē~erint";
    ind_pass_pres_1: "~i~ior", "~i~imur";
    ind_pass_pres_2: "~i~eris|~i~ere", "~i~iminī";
    ind_pass_pres_3: "~i~itur", "~i~iuntur";
    ind_pass_impf_1: "~i~iēbar", "~i~iēbāmur";
    ind_pass_impf_2: "~i~iēbāris|~i~iēbāre", "~i~iēbāminī";
    ind_pass_impf_3: "~i~iēbātur", "~i~iēbantur";
    ind_pass_fut_1: "~i~iar", "~i~iēmur";
    ind_pass_fut_2: "~i~iēris|~i~iēre", "~i~iēminī";
    ind_pass_fut_3: "~i~iētur", "~i~ientur";
    subj_act_pres_1: "~i~iam", "~i~iāmus";
    subj_act_pres_2: "~i~iās", "~i~iātis";
    subj_act_pres_3: "~i~iat", "~i~iant";
    subj_act_impf_1: "~i~erem", "~i~erēmus";
    subj_act_impf_2: "~i~erēs", "~i~erētis";
    subj_act_impf_3: "~i~eret", "~i~erent";
    subj_act_perf_1: "~ē~erim", "~ē~erīmus";
    subj_act_perf_2: "~ē~erīs", "~ē~erītis";
    subj_act_plup_1: "~ē~issem", "~ē~issēmus";
    subj_act_plup_2: "~ē~issēs", "~ē~issētis";
    subj_act_plup_3: "~ē~isset", "~ē~issent";
    subj_pass_pres_1: "~i~iar", "~i~iāmur";
    subj_pass_pres_2: "~i~iāris|~i~iāre", "~i~iāminī";
    subj_pass_pres_3: "~i~iātur", "~i~iantur";
    subj_pass_impf_1: "~i~erer", "~i~erēmur";
    subj_pass_impf_2: "~i~erēris|~i~erēre", "~i~erēminī";
    subj_pass_impf_3: "~i~erētur", "~i~erentur";
    imperative_act_pres_2: "~i~e", "~i~ite";
    imperative_act_fut_2: "~i~itō", "~i~itōte";
    imperative_act_fut_3: _, "~i~iuntō";
    imperative_pass_pres_2: "~i~ere", _;
    imperative_pass_fut_2: "~i~itor", _;
    imperative_pass_fut_3: _, "~i~iuntor";
    infinitive_act: _, "~ē~isse", _;
    infinitive_pass: "~i~ī", _, _;
    participle: "~i~iēns", _, _, "~i~iendus|~i~iundus";
    gerund: _, "~i~iendī", "~i~iendō", "~i~iendum", _, _;
    ),
    // Compescō: 36 words (volō, mālō, molō, tremō, olō, nōlō). Like Petō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("compescō", 36, "~ō", like Petō;
    stem participle_perf_pass: "~it- like agitō";
    ind_act_perf_1: "~uī", "~uimus";
    ind_act_perf_2: "~uistī", "~uistis";
    ind_act_perf_3: "~uit", "~uērunt|~uēre";
    ind_act_plup_1: "~ueram", "~uerāmus";
    ind_act_plup_2: "~uerās", "~uerātis";
    ind_act_plup_3: "~uerat", "~uerant";
    ind_act_futp_1: "~uerō", "~uerimus";
    ind_act_futp_2: "~ueris", "~ueritis";
    ind_act_futp_3: "~uerit", "~uerint";
    subj_act_perf_1: "~uerim", "~uerīmus";
    subj_act_perf_2: "~uerīs", "~uerītis";
    subj_act_plup_1: "~uissem", "~uissēmus";
    subj_act_plup_2: "~uissēs", "~uissētis";
    subj_act_plup_3: "~uisset", "~uissent";
    infinitive_act: _, "~uisse", _;
    ),
    // Impleō: 29 words (dēleō, expleō, neō, fleō, aboleō, adimpleō). Like Habeō, except in 34 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("impleō", 29, "~eō", like Habeō;
    stem participle_perf_pass: "~ēt- like agitō";
    ind_act_perf_1: "~ēvī", "~ēvimus";
    ind_act_perf_2: "~ēvistī", "~ēvistis";
    ind_act_perf_3: "~ēvit", "~ēvērunt|~ēvēre";
    ind_act_plup_1: "~ēveram", "~ēverāmus";
    ind_act_plup_2: "~ēverās", "~ēverātis";
    ind_act_plup_3: "~ēverat", "~ēverant";
    ind_act_futp_1: "~ēverō", "~ēverimus";
    ind_act_futp_2: "~ēveris", "~ēveritis";
    ind_act_futp_3: "~ēverit", "~ēverint";
    ind_pass_pres_2: "~ēris|~ēre", _;
    ind_pass_impf_2: "~ēbāris|~ēbāre", _;
    ind_pass_fut_2: "~ēberis|~ēbere", _;
    subj_act_perf_1: "~ēverim", "~ēverīmus";
    subj_act_perf_2: "~ēverīs", "~ēverītis";
    subj_act_plup_1: "~ēvissem", "~ēvissēmus";
    subj_act_plup_2: "~ēvissēs", "~ēvissētis";
    subj_act_plup_3: "~ēvisset", "~ēvissent";
    subj_pass_pres_2: "~eāris|~eāre", _;
    subj_pass_impf_2: "~ērēris|~ērēre", _;
    infinitive_act: _, "~ēvisse", _;
    ),
    // Repōnō: 29 words (pōnō, expōnō, oppōnō, antepōnō, prōpōnō, dēpōnō).
    conjugation!("repōnō", 29, "~ōnō";
    stem participle_perf_pass: "~os- like repōnō";
    ind_act_pres_1: "~ōnō", "~ōnimus";
    ind_act_pres_2: "~ōnis", "~ōnitis";
    ind_act_pres_3: "~ōnit", "~ōnunt";
    ind_act_impf_1: "~ōnēbam", "~ōnēbāmus";
    ind_act_impf_2: "~ōnēbās", "~ōnēbātis";
    ind_act_impf_3: "~ōnēbat", "~ōnēbant";
    ind_act_fut_1: "~ōnam", "~ōnēmus";
    ind_act_fut_2: "~ōnēs", "~ōnētis";
    ind_act_fut_3: "~ōnet", "~ōnent";
    ind_act_perf_1: "~osuī|~osīvī", "~osuimus|~osīvimus|~osīmus";
    ind_act_perf_2: "~osuistī|~osīvistī|~osīstī", "~osuistis|~osīvistis|~osīstis";
    ind_act_perf_3: "~osuit|~osīvit|~osīt", "~osuērunt|~osuēre|~osīvērunt|~osīrunt|~osīvēre";
    ind_act_plup_1: "~osueram|~osīveram|~osīram", "~osuerāmus|~osīverāmus|~osīrāmus";
    ind_act_plup_2: "~osuerās|~osīverās|~osīrās", "~osuerātis|~osīverātis|~osīrātis";
    ind_act_plup_3: "~osuerat|~osīverat|~osīrat", "~osuerant|~osīverant|~osīrant";
    ind_act_futp_1: "~osuerō|~osīverō|~osīrō", "~osuerimus|~osīverimus|~osīrimus";
    ind_act_futp_2: "~osueris|~osīveris|~osīris", "~osueritis|~osīveritis|~osīritis";
    ind_act_futp_3: "~osuerit|~osīverit|~osīrit", "~osuerint|~osīverint|~osīrint";
    ind_pass_pres_1: "~ōnor", "~ōnimur";
    ind_pass_pres_2: "~ōneris|~ōnere", "~ōniminī";
    ind_pass_pres_3: "~ōnitur", "~ōnuntur";
    ind_pass_impf_1: "~ōnēbar", "~ōnēbāmur";
    ind_pass_impf_2: "~ōnēbāris|~ōnēbāre", "~ōnēbāminī";
    ind_pass_impf_3: "~ōnēbātur", "~ōnēbantur";
    ind_pass_fut_1: "~ōnar", "~ōnēmur";
    ind_pass_fut_2: "~ōnēris|~ōnēre", "~ōnēminī";
    ind_pass_fut_3: "~ōnētur", "~ōnentur";
    subj_act_pres_1: "~ōnam", "~ōnāmus";
    subj_act_pres_2: "~ōnās", "~ōnātis";
    subj_act_pres_3: "~ōnat", "~ōnant";
    subj_act_impf_1: "~ōnerem", "~ōnerēmus";
    subj_act_impf_2: "~ōnerēs", "~ōnerētis";
    subj_act_impf_3: "~ōneret", "~ōnerent";
    subj_act_perf_1: "~osuerim|~osīverim|~osīrim", "~osuerīmus|~osīverīmus|~osīrīmus";
    subj_act_perf_2: "~osuerīs|~osīverīs|~osīrīs", "~osuerītis|~osīverītis|~osīrītis";
    subj_act_plup_1: "~osuissem|~osīvissem|~osīssem", "~osuissēmus|~osīvissēmus|~osīssēmus";
    subj_act_plup_2: "~osuissēs|~osīvissēs|~osīssēs", "~osuissētis|~osīvissētis|~osīssētis";
    subj_act_plup_3: "~osuisset|~osīvisset|~osīsset", "~osuissent|~osīvissent|~osīssent";
    subj_pass_pres_1: "~ōnar", "~ōnāmur";
    subj_pass_pres_2: "~ōnāris|~ōnāre", "~ōnāminī";
    subj_pass_pres_3: "~ōnātur", "~ōnantur";
    subj_pass_impf_1: "~ōnerer", "~ōnerēmur";
    subj_pass_impf_2: "~ōnerēris|~ōnerēre", "~ōnerēminī";
    subj_pass_impf_3: "~ōnerētur", "~ōnerentur";
    imperative_act_pres_2: "~ōne", "~ōnite";
    imperative_act_fut_2: "~ōnitō", "~ōnitōte";
    imperative_act_fut_3: _, "~ōnuntō";
    imperative_pass_pres_2: "~ōnere", _;
    imperative_pass_fut_2: "~ōnitor", _;
    imperative_pass_fut_3: _, "~ōnuntor";
    infinitive_act: _, "~osuisse|~osīvisse|~osīsse", _;
    infinitive_pass: "~ōnī", _, _;
    participle: "~ōnēns", _, _, "~ōnendus|~ōnundus";
    gerund: _, "~ōnendī", "~ōnendō", "~ōnendum", _, _;
    ),
    // Mittō: 28 words (mittō, prōmittō, āmittō, committō, admittō, remittō).
    conjugation!("mittō", 28, "~ittō";
    stem participle_perf_pass: "~iss- like agitō";
    ind_act_pres_1: "~ittō", "~ittimus";
    ind_act_pres_2: "~ittis", "~ittitis";
    ind_act_pres_3: "~ittit", "~ittunt";
    ind_act_impf_1: "~ittēbam", "~ittēbāmus";
    ind_act_impf_2: "~ittēbās", "~ittēbātis";
    ind_act_impf_3: "~ittēbat", "~ittēbant";
    ind_act_fut_1: "~ittam", "~ittēmus";
    ind_act_fut_2: "~ittēs", "~ittētis";
    ind_act_fut_3: "~ittet", "~ittent";
    ind_act_perf_1: "~īsī", "~īsimus";
    ind_act_perf_2: "~īsistī", "~īsistis";
    ind_act_perf_3: "~īsit", "~īsērunt|~īsēre";
    ind_act_plup_1: "~īseram", "~īserāmus";
    ind_act_plup_2: "~īserās", "~īserātis";
    ind_act_plup_3: "~īserat", "~īserant";
    ind_act_futp_1: "~īserō", "~īserimus";
    ind_act_futp_2: "~īseris", "~īseritis";
    ind_act_futp_3: "~īserit", "~īserint";
    ind_pass_pres_1: "~ittor", "~ittimur";
    ind_pass_pres_2: "~itteris|~ittere", "~ittiminī";
    ind_pass_pres_3: "~ittitur", "~ittuntur";
    ind_pass_impf_1: "~ittēbar", "~ittēbāmur";
    ind_pass_impf_2: "~ittēbāris|~ittēbāre", "~ittēbāminī";
    ind_pass_impf_3: "~ittēbātur", "~ittēbantur";
    ind_pass_fut_1: "~ittar", "~ittēmur";
    ind_pass_fut_2: "~ittēris|~ittēre", "~ittēminī";
    ind_pass_fut_3: "~ittētur", "~ittentur";
    subj_act_pres_1: "~ittam", "~ittāmus";
    subj_act_pres_2: "~ittās", "~ittātis";
    subj_act_pres_3: "~ittat", "~ittant";
    subj_act_impf_1: "~itterem", "~itterēmus";
    subj_act_impf_2: "~itterēs", "~itterētis";
    subj_act_impf_3: "~itteret", "~itterent";
    subj_act_perf_1: "~īserim", "~īserīmus";
    subj_act_perf_2: "~īserīs", "~īserītis";
    subj_act_plup_1: "~īsissem", "~īsissēmus";
    subj_act_plup_2: "~īsissēs", "~īsissētis";
    subj_act_plup_3: "~īsisset", "~īsissent";
    subj_pass_pres_1: "~ittar", "~ittāmur";
    subj_pass_pres_2: "~ittāris|~ittāre", "~ittāminī";
    subj_pass_pres_3: "~ittātur", "~ittantur";
    subj_pass_impf_1: "~itterer", "~itterēmur";
    subj_pass_impf_2: "~itterēris|~itterēre", "~itterēminī";
    subj_pass_impf_3: "~itterētur", "~itterentur";
    imperative_act_pres_2: "~itte", "~ittite";
    imperative_act_fut_2: "~ittitō", "~ittitōte";
    imperative_act_fut_3: _, "~ittuntō";
    imperative_pass_pres_2: "~ittere", _;
    imperative_pass_fut_2: "~ittitor", _;
    imperative_pass_fut_3: _, "~ittuntor";
    infinitive_act: _, "~īsisse", _;
    infinitive_pass: "~ittī", _, _;
    participle: "~ittēns", _, _, "~ittendus|~ittundus";
    gerund: _, "~ittendī", "~ittendō", "~ittendum", _, _;
    ),
    // Labōrō: 28 words (abambulō, labōrō, abortō, cursō, abequitō, lāpsō). Like Agitō, except in 25 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("labōrō", 28, "~ō", like Agitō;
    stem participle_perf_pass: "~āt- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~ātor", _;
    infinitive_act: "~āre", _, _;
    participle: _, _, _, "~andum";
    ),
    // Obeō: 27 words (adeō, dispereō, exeō, queō, ineō, intereō). Like Mūniō, except in 75 cells: ind_act_pres_1 sg, ind_act_pres_3 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ….
    conjugation!("obeō", 27, "~eō", like Mūniō;
    stem participle_perf_pass: "~it- like agitō";
    ind_act_pres_1: "~eō", _;
    ind_act_pres_3: _, "~eunt";
    ind_act_impf_1: "~ībam", "~ībāmus";
    ind_act_impf_2: "~ībās", "~ībātis";
    ind_act_impf_3: "~ībat", "~ībant";
    ind_act_fut_1: "~ībō", "~ībimus";
    ind_act_fut_2: "~ībis", "~ībitis";
    ind_act_fut_3: "~ībit", "~ībunt";
    ind_act_perf_1: "~iī|~īvī", "~iimus|~īvimus";
    ind_act_perf_2: "~īstī|~iistī|~īvistī", "~īstis|~iistis|~īvistis";
    ind_act_perf_3: "~iit|~īvit", "~iērunt|~iēre|~īvērunt|~īvēre";
    ind_act_plup_1: "~ieram|~īveram", "~ierāmus|~īverāmus";
    ind_act_plup_2: "~ierās|~īverās", "~ierātis|~īverātis";
    ind_act_plup_3: "~ierat|~īverat", "~ierant|~īverant";
    ind_act_futp_1: "~ierō|~īverō", "~ierimus|~īverimus";
    ind_act_futp_2: "~ieris|~īveris", "~ieritis|~īveritis";
    ind_act_futp_3: "~ierit|~īverit", "~ierint|~īverint";
    ind_pass_pres_1: "~eor", _;
    ind_pass_pres_3: _, "~euntur";
    ind_pass_impf_1: "~ībar", "~ībāmur";
    ind_pass_impf_2: "~ībāris|~ībāre", "~ībāminī";
    ind_pass_impf_3: "~ībātur", "~ībantur";
    ind_pass_fut_1: "~ībor", "~ībimur";
    ind_pass_fut_2: "~īberis|~ībere", "~ībiminī";
    ind_pass_fut_3: "~ībitur", "~ībuntur";
    subj_act_pres_1: "~eam", "~eāmus";
    subj_act_pres_2: "~eās", "~eātis";
    subj_act_pres_3: "~eat", "~eant";
    subj_act_perf_1: "~ierim|~īverim", "~ierīmus|~īverīmus";
    subj_act_perf_2: "~ierīs|~īverīs", "~ierītis|~īverītis";
    subj_act_plup_1: "~īssem|~iissem|~īvissem", "~īssēmus|~iissēmus|~īvissēmus";
    subj_act_plup_2: "~īssēs|~iissēs|~īvissēs", "~īssētis|~iissētis|~īvissētis";
    subj_act_plup_3: "~īsset|~iisset|~īvisset", "~īssent|~iissent|~īvissent";
    subj_pass_pres_1: "~ear", "~eāmur";
    subj_pass_pres_2: "~eāris|~eāre", "~eāminī";
    subj_pass_pres_3: "~eātur", "~eantur";
    imperative_act_fut_3: _, "~euntō";
    imperative_pass_fut_3: _, "~euntor";
    infinitive_act: _, "~īsse|~iisse|~īvisse", _;
    participle: _, _, _, "~eundus";
    gerund: _, "~eundī", "~eundō", "~eundum", _, _;
    ),
    // Perstringō: 27 words (cōnfingō, dēpingō, effingō, fingō, pingō, stringō).
    conjugation!("perstringō", 27, "~ingō";
    stem participle_perf_pass: "~ict- like agitō";
    ind_act_pres_1: "~ingō", "~ingimus";
    ind_act_pres_2: "~ingis", "~ingitis";
    ind_act_pres_3: "~ingit", "~ingunt";
    ind_act_impf_1: "~ingēbam", "~ingēbāmus";
    ind_act_impf_2: "~ingēbās", "~ingēbātis";
    ind_act_impf_3: "~ingēbat", "~ingēbant";
    ind_act_fut_1: "~ingam", "~ingēmus";
    ind_act_fut_2: "~ingēs", "~ingētis";
    ind_act_fut_3: "~inget", "~ingent";
    ind_act_perf_1: "~īnxī", "~īnximus";
    ind_act_perf_2: "~īnxistī", "~īnxistis";
    ind_act_perf_3: "~īnxit", "~īnxērunt|~īnxēre";
    ind_act_plup_1: "~īnxeram", "~īnxerāmus";
    ind_act_plup_2: "~īnxerās", "~īnxerātis";
    ind_act_plup_3: "~īnxerat", "~īnxerant";
    ind_act_futp_1: "~īnxerō", "~īnxerimus";
    ind_act_futp_2: "~īnxeris", "~īnxeritis";
    ind_act_futp_3: "~īnxerit", "~īnxerint";
    ind_pass_pres_1: "~ingor", "~ingimur";
    ind_pass_pres_2: "~ingeris|~ingere", "~ingiminī";
    ind_pass_pres_3: "~ingitur", "~inguntur";
    ind_pass_impf_1: "~ingēbar", "~ingēbāmur";
    ind_pass_impf_2: "~ingēbāris|~ingēbāre", "~ingēbāminī";
    ind_pass_impf_3: "~ingēbātur", "~ingēbantur";
    ind_pass_fut_1: "~ingar", "~ingēmur";
    ind_pass_fut_2: "~ingēris|~ingēre", "~ingēminī";
    ind_pass_fut_3: "~ingētur", "~ingentur";
    subj_act_pres_1: "~ingam", "~ingāmus";
    subj_act_pres_2: "~ingās", "~ingātis";
    subj_act_pres_3: "~ingat", "~ingant";
    subj_act_impf_1: "~ingerem", "~ingerēmus";
    subj_act_impf_2: "~ingerēs", "~ingerētis";
    subj_act_impf_3: "~ingeret", "~ingerent";
    subj_act_perf_1: "~īnxerim", "~īnxerīmus";
    subj_act_perf_2: "~īnxerīs", "~īnxerītis";
    subj_act_plup_1: "~īnxissem", "~īnxissēmus";
    subj_act_plup_2: "~īnxissēs", "~īnxissētis";
    subj_act_plup_3: "~īnxisset", "~īnxissent";
    subj_pass_pres_1: "~ingar", "~ingāmur";
    subj_pass_pres_2: "~ingāris|~ingāre", "~ingāminī";
    subj_pass_pres_3: "~ingātur", "~ingantur";
    subj_pass_impf_1: "~ingerer", "~ingerēmur";
    subj_pass_impf_2: "~ingerēris|~ingerēre", "~ingerēminī";
    subj_pass_impf_3: "~ingerētur", "~ingerentur";
    imperative_act_pres_2: "~inge", "~ingite";
    imperative_act_fut_2: "~ingitō", "~ingitōte";
    imperative_act_fut_3: _, "~inguntō";
    imperative_pass_pres_2: "~ingere", _;
    imperative_pass_fut_2: "~ingitor", _;
    imperative_pass_fut_3: _, "~inguntor";
    infinitive_act: _, "~īnxisse", _;
    infinitive_pass: "~ingī", _, _;
    participle: "~ingēns", _, _, "~ingendus|~ingundus";
    gerund: _, "~ingendī", "~ingendō", "~ingendum", _, _;
    ),
    // Coniungō: 27 words (ēmungō, pungō, compungō, interpungō, iungō, abiungō).
    conjugation!("coniungō", 27, "~u~gō";
    stem participle_perf_pass: "~ū~ct- like agitō";
    ind_act_pres_1: "~u~gō", "~u~gimus";
    ind_act_pres_2: "~u~gis", "~u~gitis";
    ind_act_pres_3: "~u~git", "~u~gunt";
    ind_act_impf_1: "~u~gēbam", "~u~gēbāmus";
    ind_act_impf_2: "~u~gēbās", "~u~gēbātis";
    ind_act_impf_3: "~u~gēbat", "~u~gēbant";
    ind_act_fut_1: "~u~gam", "~u~gēmus";
    ind_act_fut_2: "~u~gēs", "~u~gētis";
    ind_act_fut_3: "~u~get", "~u~gent";
    ind_act_perf_1: "~ū~xī", "~ū~ximus";
    ind_act_perf_2: "~ū~xistī", "~ū~xistis";
    ind_act_perf_3: "~ū~xit", "~ū~xērunt|~ū~xēre";
    ind_act_plup_1: "~ū~xeram", "~ū~xerāmus";
    ind_act_plup_2: "~ū~xerās", "~ū~xerātis";
    ind_act_plup_3: "~ū~xerat", "~ū~xerant";
    ind_act_futp_1: "~ū~xerō", "~ū~xerimus";
    ind_act_futp_2: "~ū~xeris", "~ū~xeritis";
    ind_act_futp_3: "~ū~xerit", "~ū~xerint";
    ind_pass_pres_1: "~u~gor", "~u~gimur";
    ind_pass_pres_2: "~u~geris|~u~gere", "~u~giminī";
    ind_pass_pres_3: "~u~gitur", "~u~guntur";
    ind_pass_impf_1: "~u~gēbar", "~u~gēbāmur";
    ind_pass_impf_2: "~u~gēbāris|~u~gēbāre", "~u~gēbāminī";
    ind_pass_impf_3: "~u~gēbātur", "~u~gēbantur";
    ind_pass_fut_1: "~u~gar", "~u~gēmur";
    ind_pass_fut_2: "~u~gēris|~u~gēre", "~u~gēminī";
    ind_pass_fut_3: "~u~gētur", "~u~gentur";
    subj_act_pres_1: "~u~gam", "~u~gāmus";
    subj_act_pres_2: "~u~gās", "~u~gātis";
    subj_act_pres_3: "~u~gat", "~u~gant";
    subj_act_impf_1: "~u~gerem", "~u~gerēmus";
    subj_act_impf_2: "~u~gerēs", "~u~gerētis";
    subj_act_impf_3: "~u~geret", "~u~gerent";
    subj_act_perf_1: "~ū~xerim", "~ū~xerīmus";
    subj_act_perf_2: "~ū~xerīs", "~ū~xerītis";
    subj_act_plup_1: "~ū~xissem", "~ū~xissēmus";
    subj_act_plup_2: "~ū~xissēs", "~ū~xissētis";
    subj_act_plup_3: "~ū~xisset", "~ū~xissent";
    subj_pass_pres_1: "~u~gar", "~u~gāmur";
    subj_pass_pres_2: "~u~gāris|~u~gāre", "~u~gāminī";
    subj_pass_pres_3: "~u~gātur", "~u~gantur";
    subj_pass_impf_1: "~u~gerer", "~u~gerēmur";
    subj_pass_impf_2: "~u~gerēris|~u~gerēre", "~u~gerēminī";
    subj_pass_impf_3: "~u~gerētur", "~u~gerentur";
    imperative_act_pres_2: "~u~ge", "~u~gite";
    imperative_act_fut_2: "~u~gitō", "~u~gitōte";
    imperative_act_fut_3: _, "~u~guntō";
    imperative_pass_pres_2: "~u~gere", _;
    imperative_pass_fut_2: "~u~gitor", _;
    imperative_pass_fut_3: _, "~u~guntor";
    infinitive_act: _, "~ū~xisse", _;
    infinitive_pass: "~u~gī", _, _;
    participle: "~u~gēns", _, _, "~u~gendus|~u~gundus";
    gerund: _, "~u~gendī", "~u~gendō", "~u~gendum", _, _;
    ),
    // Prōdō: 26 words (abdō, vēndō, dēdō, dīdō, condō, crēdō). Like Compescō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("prōdō", 26, "~ō", like Compescō;
    ind_act_perf_1: "~idī", "~idimus";
    ind_act_perf_2: "~idistī", "~idistis";
    ind_act_perf_3: "~idit", "~idērunt|~idēre";
    ind_act_plup_1: "~ideram", "~iderāmus";
    ind_act_plup_2: "~iderās", "~iderātis";
    ind_act_plup_3: "~iderat", "~iderant";
    ind_act_futp_1: "~iderō", "~iderimus";
    ind_act_futp_2: "~ideris", "~ideritis";
    ind_act_futp_3: "~iderit", "~iderint";
    subj_act_perf_1: "~iderim", "~iderīmus";
    subj_act_perf_2: "~iderīs", "~iderītis";
    subj_act_plup_1: "~idissem", "~idissēmus";
    subj_act_plup_2: "~idissēs", "~idissētis";
    subj_act_plup_3: "~idisset", "~idissent";
    infinitive_act: _, "~idisse", _;
    ),
    // Dēdūcō: 26 words (abdūcō, trādūcō, prōdūcō, condūcō, ēdūcō, dēdūcō).
    conjugation!("dēdūcō", 26, "~ūcō";
    stem participle_perf_pass: "~uct- like agitō";
    ind_act_pres_1: "~ūcō", "~ūcimus";
    ind_act_pres_2: "~ūcis", "~ūcitis";
    ind_act_pres_3: "~ūcit", "~ūcunt";
    ind_act_impf_1: "~ūcēbam", "~ūcēbāmus";
    ind_act_impf_2: "~ūcēbās", "~ūcēbātis";
    ind_act_impf_3: "~ūcēbat", "~ūcēbant";
    ind_act_fut_1: "~ūcam", "~ūcēmus";
    ind_act_fut_2: "~ūcēs", "~ūcētis";
    ind_act_fut_3: "~ūcet", "~ūcent";
    ind_act_perf_1: "~ūxī", "~ūximus";
    ind_act_perf_2: "~ūxistī", "~ūxistis";
    ind_act_perf_3: "~ūxit", "~ūxērunt|~ūxēre";
    ind_act_plup_1: "~ūxeram", "~ūxerāmus";
    ind_act_plup_2: "~ūxerās", "~ūxerātis";
    ind_act_plup_3: "~ūxerat", "~ūxerant";
    ind_act_futp_1: "~ūxerō", "~ūxerimus";
    ind_act_futp_2: "~ūxeris", "~ūxeritis";
    ind_act_futp_3: "~ūxerit", "~ūxerint";
    ind_pass_pres_1: "~ūcor", "~ūcimur";
    ind_pass_pres_2: "~ūceris|~ūcere", "~ūciminī";
    ind_pass_pres_3: "~ūcitur", "~ūcuntur";
    ind_pass_impf_1: "~ūcēbar", "~ūcēbāmur";
    ind_pass_impf_2: "~ūcēbāris|~ūcēbāre", "~ūcēbāminī";
    ind_pass_impf_3: "~ūcēbātur", "~ūcēbantur";
    ind_pass_fut_1: "~ūcar", "~ūcēmur";
    ind_pass_fut_2: "~ūcēris|~ūcēre", "~ūcēminī";
    ind_pass_fut_3: "~ūcētur", "~ūcentur";
    subj_act_pres_1: "~ūcam", "~ūcāmus";
    subj_act_pres_2: "~ūcās", "~ūcātis";
    subj_act_pres_3: "~ūcat", "~ūcant";
    subj_act_impf_1: "~ūcerem", "~ūcerēmus";
    subj_act_impf_2: "~ūcerēs", "~ūcerētis";
    subj_act_impf_3: "~ūceret", "~ūcerent";
    subj_act_perf_1: "~ūxerim", "~ūxerīmus";
    subj_act_perf_2: "~ūxerīs", "~ūxerītis";
    subj_act_plup_1: "~ūxissem", "~ūxissēmus";
    subj_act_plup_2: "~ūxissēs", "~ūxissētis";
    subj_act_plup_3: "~ūxisset", "~ūxissent";
    subj_pass_pres_1: "~ūcar", "~ūcāmur";
    subj_pass_pres_2: "~ūcāris|~ūcāre", "~ūcāminī";
    subj_pass_pres_3: "~ūcātur", "~ūcantur";
    subj_pass_impf_1: "~ūcerer", "~ūcerēmur";
    subj_pass_impf_2: "~ūcerēris|~ūcerēre", "~ūcerēminī";
    subj_pass_impf_3: "~ūcerētur", "~ūcerentur";
    imperative_act_pres_2: "~ūc|~ūce", "~ūcite";
    imperative_act_fut_2: "~ūcitō", "~ūcitōte";
    imperative_act_fut_3: _, "~ūcuntō";
    imperative_pass_pres_2: "~ūcere", _;
    imperative_pass_fut_2: "~ūcitor", _;
    imperative_pass_fut_3: _, "~ūcuntor";
    infinitive_act: _, "~ūxisse", _;
    infinitive_pass: "~ūcī", _, _;
    participle: "~ūcēns", _, _, "~ūcendus|~ūcundus";
    gerund: _, "~ūcendī", "~ūcendō", "~ūcendum", _, _;
    ),
    // Cōnsequor: 25 words (sequor, līquor, prōsequor, loquor, persequor, adsequor). Like Modulor, except in 19 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("cōnsequor", 25, "~quor", like Modulor as PERF_AS_FUT;
    stem verb_act_sg: "~qu- like cōnsequor";
    stem participle_perf_pass: "~cūt- like modulor";
    ind_act_pres_1: _, "~quimur";
    ind_act_impf_1: "~quēbar", "~quēbāmur";
    ind_act_impf_2: "~quēbāris|~quēbāre", "~quēbāminī";
    ind_act_impf_3: "~quēbātur", "~quēbantur";
    ind_act_fut_1: "~quar", "~quēmur";
    ind_act_fut_2: "~quēris|~quēre", "~quēminī";
    ind_act_fut_3: "~quētur", _;
    subj_act_pres_3: "~quātur", _;
    imperative_act_fut_2: "~quitor", _;
    participle: "~quēns", _, _, "~quendus";
    gerund: _, "~quendī", "~quendō", "~quendum", _, _;
    ),
    // Mergō: 25 words (mergō, dēmergō, immergō, ēmergō, spargō, cōnspargō). Like Ēvādō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("mergō", 25, "~gō", like Ēvādō;
    ind_act_pres_1: "~gō", "~gimus";
    ind_act_pres_2: "~gis", "~gitis";
    ind_act_pres_3: "~git", "~gunt";
    ind_act_impf_1: "~gēbam", "~gēbāmus";
    ind_act_impf_2: "~gēbās", "~gēbātis";
    ind_act_impf_3: "~gēbat", "~gēbant";
    ind_act_fut_1: "~gam", "~gēmus";
    ind_act_fut_2: "~gēs", "~gētis";
    ind_act_fut_3: "~get", "~gent";
    ind_pass_pres_1: "~gor", "~gimur";
    ind_pass_pres_2: "~geris|~gere", "~giminī";
    ind_pass_pres_3: "~gitur", "~guntur";
    ind_pass_impf_1: "~gēbar", "~gēbāmur";
    ind_pass_impf_2: "~gēbāris|~gēbāre", "~gēbāminī";
    ind_pass_impf_3: "~gēbātur", "~gēbantur";
    ind_pass_fut_1: "~gar", "~gēmur";
    ind_pass_fut_2: "~gēris|~gēre", "~gēminī";
    ind_pass_fut_3: "~gētur", "~gentur";
    subj_act_pres_1: "~gam", "~gāmus";
    subj_act_pres_2: "~gās", "~gātis";
    subj_act_pres_3: "~gat", "~gant";
    subj_act_impf_1: "~gerem", "~gerēmus";
    subj_act_impf_2: "~gerēs", "~gerētis";
    subj_act_impf_3: "~geret", "~gerent";
    subj_pass_pres_1: "~gar", "~gāmur";
    subj_pass_pres_2: "~gāris|~gāre", "~gāminī";
    subj_pass_pres_3: "~gātur", "~gantur";
    subj_pass_impf_1: "~gerer", "~gerēmur";
    subj_pass_impf_2: "~gerēris|~gerēre", "~gerēminī";
    subj_pass_impf_3: "~gerētur", "~gerentur";
    imperative_act_pres_2: "~ge", "~gite";
    imperative_act_fut_2: "~gitō", "~gitōte";
    imperative_act_fut_3: _, "~guntō";
    imperative_pass_pres_2: "~gere", _;
    imperative_pass_fut_2: "~gitor", _;
    imperative_pass_fut_3: _, "~guntor";
    infinitive_pass: "~gī", _, _;
    participle: "~gēns", _, _, "~gendus|~gundus";
    gerund: _, "~gendī", "~gendō", "~gendum", _, _;
    ),
    // Coniciō: 25 words (prōficiō, rabiō, iniciō, pariō, abiciō, subiciō). Like Excipiō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("coniciō", 25, "~~iō", like Excipiō;
    ind_act_pres_1: "~~iō", "~~imus";
    ind_act_pres_2: "~~is", "~~itis";
    ind_act_pres_3: "~~it", "~~iunt";
    ind_act_impf_1: "~~iēbam", "~~iēbāmus";
    ind_act_impf_2: "~~iēbās", "~~iēbātis";
    ind_act_impf_3: "~~iēbat", "~~iēbant";
    ind_act_fut_1: "~~iam", "~~iēmus";
    ind_act_fut_2: "~~iēs", "~~iētis";
    ind_act_fut_3: "~~iet", "~~ient";
    ind_pass_pres_1: "~~ior", "~~imur";
    ind_pass_pres_2: "~~eris|~~ere", "~~iminī";
    ind_pass_pres_3: "~~itur", "~~iuntur";
    ind_pass_impf_1: "~~iēbar", "~~iēbāmur";
    ind_pass_impf_2: "~~iēbāris|~~iēbāre", "~~iēbāminī";
    ind_pass_impf_3: "~~iēbātur", "~~iēbantur";
    ind_pass_fut_1: "~~iar", "~~iēmur";
    ind_pass_fut_2: "~~iēris|~~iēre", "~~iēminī";
    ind_pass_fut_3: "~~iētur", "~~ientur";
    subj_act_pres_1: "~~iam", "~~iāmus";
    subj_act_pres_2: "~~iās", "~~iātis";
    subj_act_pres_3: "~~iat", "~~iant";
    subj_act_impf_1: "~~erem", "~~erēmus";
    subj_act_impf_2: "~~erēs", "~~erētis";
    subj_act_impf_3: "~~eret", "~~erent";
    subj_pass_pres_1: "~~iar", "~~iāmur";
    subj_pass_pres_2: "~~iāris|~~iāre", "~~iāminī";
    subj_pass_pres_3: "~~iātur", "~~iantur";
    subj_pass_impf_1: "~~erer", "~~erēmur";
    subj_pass_impf_2: "~~erēris|~~erēre", "~~erēminī";
    subj_pass_impf_3: "~~erētur", "~~erentur";
    imperative_act_pres_2: "~~e", "~~ite";
    imperative_act_fut_2: "~~itō", "~~itōte";
    imperative_act_fut_3: _, "~~iuntō";
    imperative_pass_pres_2: "~~ere", _;
    imperative_pass_fut_2: "~~itor", _;
    imperative_pass_fut_3: _, "~~iuntor";
    infinitive_pass: "~~ī", _, _;
    participle: "~~iēns", _, _, "~~iendus|~~iundus";
    gerund: _, "~~iendī", "~~iendō", "~~iendum", _, _;
    ),
    // Gerō: 25 words (gerō, requīrō, inquīrō, ēgerō, conquīrō, aggerō).
    conjugation!("gerō", 25, "~rō";
    stem participle_perf_pass: "~st- like agitō";
    ind_act_pres_1: "~rō", "~rimus";
    ind_act_pres_2: "~ris", "~ritis";
    ind_act_pres_3: "~rit", "~runt";
    ind_act_impf_1: "~rēbam", "~rēbāmus";
    ind_act_impf_2: "~rēbās", "~rēbātis";
    ind_act_impf_3: "~rēbat", "~rēbant";
    ind_act_fut_1: "~ram", "~rēmus";
    ind_act_fut_2: "~rēs", "~rētis";
    ind_act_fut_3: "~ret", "~rent";
    ind_act_perf_1: "~ssī", "~ssimus";
    ind_act_perf_2: "~ssistī", "~ssistis";
    ind_act_perf_3: "~ssit", "~ssērunt|~ssēre";
    ind_act_plup_1: "~sseram", "~sserāmus";
    ind_act_plup_2: "~sserās", "~sserātis";
    ind_act_plup_3: "~sserat", "~sserant";
    ind_act_futp_1: "~sserō", "~sserimus";
    ind_act_futp_2: "~sseris", "~sseritis";
    ind_act_futp_3: "~sserit", "~sserint";
    ind_pass_pres_1: "~ror", "~rimur";
    ind_pass_pres_2: "~reris|~rere", "~riminī";
    ind_pass_pres_3: "~ritur", "~runtur";
    ind_pass_impf_1: "~rēbar", "~rēbāmur";
    ind_pass_impf_2: "~rēbāris|~rēbāre", "~rēbāminī";
    ind_pass_impf_3: "~rēbātur", "~rēbantur";
    ind_pass_fut_1: "~rar", "~rēmur";
    ind_pass_fut_2: "~rēris|~rēre", "~rēminī";
    ind_pass_fut_3: "~rētur", "~rentur";
    subj_act_pres_1: "~ram", "~rāmus";
    subj_act_pres_2: "~rās", "~rātis";
    subj_act_pres_3: "~rat", "~rant";
    subj_act_impf_1: "~rerem", "~rerēmus";
    subj_act_impf_2: "~rerēs", "~rerētis";
    subj_act_impf_3: "~reret", "~rerent";
    subj_act_perf_1: "~sserim", "~sserīmus";
    subj_act_perf_2: "~sserīs", "~sserītis";
    subj_act_plup_1: "~ssissem", "~ssissēmus";
    subj_act_plup_2: "~ssissēs", "~ssissētis";
    subj_act_plup_3: "~ssisset", "~ssissent";
    subj_pass_pres_1: "~rar", "~rāmur";
    subj_pass_pres_2: "~rāris|~rāre", "~rāminī";
    subj_pass_pres_3: "~rātur", "~rantur";
    subj_pass_impf_1: "~rerer", "~rerēmur";
    subj_pass_impf_2: "~rerēris|~rerēre", "~rerēminī";
    subj_pass_impf_3: "~rerētur", "~rerentur";
    imperative_act_pres_2: "~re", "~rite";
    imperative_act_fut_2: "~ritō", "~ritōte";
    imperative_act_fut_3: _, "~runtō";
    imperative_pass_pres_2: "~rere", _;
    imperative_pass_fut_2: "~ritor", _;
    imperative_pass_fut_3: _, "~runtor";
    infinitive_act: _, "~ssisse", _;
    infinitive_pass: "~rī", _, _;
    participle: "~rēns", _, _, "~rendus|~rundus";
    gerund: _, "~rendī", "~rendō", "~rendum", _, _;
    ),
    // Mōlior: 25 words (dēmōlior, mōlior, mentior, partior, blandior, potior). Like Mūniō, except in 36 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 sg, ind_pass_pres_3 pl, ….
    conjugation!("mōlior", 25, "~ior", like Mūniō as ACT_AS_PASS;
    stem verb_act_sg: "~- like mōlior";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_act_fut_2: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: _, derived;
    infinitive_pass: derived, _, _;
    participle: _, derived, _, _;
    ),
    // Pellō: 25 words (pellō, vellō, appellō, dēpellō, prōpellō, repellō).
    conjugation!("pellō", 25, "~e~lō";
    stem participle_perf_pass: "~u~s- like agitō";
    ind_act_pres_1: "~e~lō", "~e~limus";
    ind_act_pres_2: "~e~lis", "~e~litis";
    ind_act_pres_3: "~e~lit", "~e~lunt";
    ind_act_impf_1: "~e~lēbam", "~e~lēbāmus";
    ind_act_impf_2: "~e~lēbās", "~e~lēbātis";
    ind_act_impf_3: "~e~lēbat", "~e~lēbant";
    ind_act_fut_1: "~e~lam", "~e~lēmus";
    ind_act_fut_2: "~e~lēs", "~e~lētis";
    ind_act_fut_3: "~e~let", "~e~lent";
    ind_act_perf_1: "~u~ī", "~u~imus";
    ind_act_perf_2: "~u~istī", "~u~istis";
    ind_act_perf_3: "~u~it", "~u~ērunt|~u~ēre";
    ind_act_plup_1: "~u~eram", "~u~erāmus";
    ind_act_plup_2: "~u~erās", "~u~erātis";
    ind_act_plup_3: "~u~erat", "~u~erant";
    ind_act_futp_1: "~u~erō", "~u~erimus";
    ind_act_futp_2: "~u~eris", "~u~eritis";
    ind_act_futp_3: "~u~erit", "~u~erint";
    ind_pass_pres_1: "~e~lor", "~e~limur";
    ind_pass_pres_2: "~e~leris|~e~lere", "~e~liminī";
    ind_pass_pres_3: "~e~litur", "~e~luntur";
    ind_pass_impf_1: "~e~lēbar", "~e~lēbāmur";
    ind_pass_impf_2: "~e~lēbāris|~e~lēbāre", "~e~lēbāminī";
    ind_pass_impf_3: "~e~lēbātur", "~e~lēbantur";
    ind_pass_fut_1: "~e~lar", "~e~lēmur";
    ind_pass_fut_2: "~e~lēris|~e~lēre", "~e~lēminī";
    ind_pass_fut_3: "~e~lētur", "~e~lentur";
    subj_act_pres_1: "~e~lam", "~e~lāmus";
    subj_act_pres_2: "~e~lās", "~e~lātis";
    subj_act_pres_3: "~e~lat", "~e~lant";
    subj_act_impf_1: "~e~lerem", "~e~lerēmus";
    subj_act_impf_2: "~e~lerēs", "~e~lerētis";
    subj_act_impf_3: "~e~leret", "~e~lerent";
    subj_act_perf_1: "~u~erim", "~u~erīmus";
    subj_act_perf_2: "~u~erīs", "~u~erītis";
    subj_act_plup_1: "~u~issem", "~u~issēmus";
    subj_act_plup_2: "~u~issēs", "~u~issētis";
    subj_act_plup_3: "~u~isset", "~u~issent";
    subj_pass_pres_1: "~e~lar", "~e~lāmur";
    subj_pass_pres_2: "~e~lāris|~e~lāre", "~e~lāminī";
    subj_pass_pres_3: "~e~lātur", "~e~lantur";
    subj_pass_impf_1: "~e~lerer", "~e~lerēmur";
    subj_pass_impf_2: "~e~lerēris|~e~lerēre", "~e~lerēminī";
    subj_pass_impf_3: "~e~lerētur", "~e~lerentur";
    imperative_act_pres_2: "~e~le", "~e~lite";
    imperative_act_fut_2: "~e~litō", "~e~litōte";
    imperative_act_fut_3: _, "~e~luntō";
    imperative_pass_pres_2: "~e~lere", _;
    imperative_pass_fut_2: "~e~litor", _;
    imperative_pass_fut_3: _, "~e~luntor";
    infinitive_act: _, "~u~isse", _;
    infinitive_pass: "~e~lī", _, _;
    participle: "~e~lēns", _, _, "~e~lendus|~e~lundus";
    gerund: _, "~e~lendī", "~e~lendō", "~e~lendum", _, _;
    ),
    // Tueor: 23 words (polliceor, fateor, cōnfiteor, vereor, reor, intueor). Like Modulor, except in 36 cells: ind_act_pres_1 sg, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("tueor", 23, "~eor", like Modulor as IND_AS_SUBJ;
    stem participle_perf_pass: "~it- like modulor";
    ind_act_pres_1: "~eor", _;
    ind_act_impf_1: "~ēbar", "~ēbāmur";
    ind_act_impf_2: "~ēbāris|~ēbāre", "~ēbāminī";
    ind_act_impf_3: "~ēbātur", "~ēbantur";
    ind_act_fut_1: "~ēbor", "~ēbimur";
    ind_act_fut_2: "~ēberis|~ēbere", "~ēbiminī";
    ind_act_fut_3: "~ēbitur", "~ēbuntur";
    subj_act_pres_1: "~ear", "~eāmur";
    subj_act_pres_2: "~eāris|~eāre", "~eāminī";
    subj_act_pres_3: "~eātur", "~eantur";
    subj_act_impf_1: "~ērer", "~ērēmur";
    subj_act_impf_2: "~ērēris|~ērēre", "~ērēminī";
    subj_act_impf_3: "~ērētur", "~ērentur";
    imperative_act_pres_2: "~ēre", "~ēminī";
    imperative_act_fut_2: "~ētor", _;
    imperative_act_fut_3: _, "~entor";
    infinitive_act: "~ērī", derived, _;
    participle: "~ēns", _, _, "~endus";
    gerund: _, "~endī", "~endō", "~endum", _, _;
    ),
    // Ārdeō: 22 words (audeō, ārdeō, rīdeō, persuādeō, subrīdeō, suādeō). Like Ēvādō, except in 48 cells: ind_act_pres_1 sg, ind_act_fut_1 sg, ind_act_fut_1 pl, ind_act_fut_2 sg, ind_act_fut_2 pl, ind_act_fut_3 sg, ….
    conjugation!("ārdeō", 22, "~deō", like Ēvādō as PRES_AS_FUT;
    ind_act_pres_1: "~deō", _;
    ind_act_fut_1: "~dēbō", "~dēbimus";
    ind_act_fut_2: "~dēbis", "~dēbitis";
    ind_act_fut_3: "~dēbit", "~dēbunt";
    ind_pass_pres_1: "~deor", _;
    ind_pass_fut_1: "~dēbor", "~dēbimur";
    ind_pass_fut_2: "~dēberis|~dēbere", "~dēbiminī";
    ind_pass_fut_3: "~dēbitur", "~dēbuntur";
    subj_act_pres_1: "~deam", "~deāmus";
    subj_act_pres_2: "~deās", "~deātis";
    subj_act_pres_3: "~deat", "~deant";
    subj_act_impf_1: "~dērem", "~dērēmus";
    subj_act_impf_2: "~dērēs", "~dērētis";
    subj_act_impf_3: "~dēret", "~dērent";
    subj_pass_pres_1: "~dear", "~deāmur";
    subj_pass_pres_2: "~deāris|~deāre", "~deāminī";
    subj_pass_pres_3: "~deātur", "~deantur";
    subj_pass_impf_1: "~dērer", "~dērēmur";
    subj_pass_impf_2: "~dērēris|~dērēre", "~dērēminī";
    subj_pass_impf_3: "~dērētur", "~dērentur";
    imperative_act_pres_2: "~dē", "~dēte";
    imperative_act_fut_2: "~dētō", "~dētōte";
    imperative_act_fut_3: _, "~dentō";
    imperative_pass_pres_2: "~dēre", _;
    imperative_pass_fut_2: "~dētor", _;
    imperative_pass_fut_3: _, "~dentor";
    infinitive_pass: "~dērī", _, _;
    participle: _, _, _, "~dendus";
    ),
    // Praecīdō: 22 words (sīdō, comprēndō, prēndō, reprēndō, apprēndō, strīdō). Like Ēvādō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("praecīdō", 22, "~dō", like Ēvādō;
    ind_act_perf_1: "~dī", "~dimus";
    ind_act_perf_2: "~distī", "~distis";
    ind_act_perf_3: "~dit", "~dērunt|~dēre";
    ind_act_plup_1: "~deram", "~derāmus";
    ind_act_plup_2: "~derās", "~derātis";
    ind_act_plup_3: "~derat", "~derant";
    ind_act_futp_1: "~derō", "~derimus";
    ind_act_futp_2: "~deris", "~deritis";
    ind_act_futp_3: "~derit", "~derint";
    subj_act_perf_1: "~derim", "~derīmus";
    subj_act_perf_2: "~derīs", "~derītis";
    subj_act_plup_1: "~dissem", "~dissēmus";
    subj_act_plup_2: "~dissēs", "~dissētis";
    subj_act_plup_3: "~disset", "~dissent";
    infinitive_act: _, "~disse", _;
    ),
    // Exsarciō: 22 words (crociō, saciō, farciō, micciō, effarciō, sarciō). Like Agō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("exsarciō", 22, "~ciō", like Agō;
    ind_act_pres_1: "~ciō", "~cīmus";
    ind_act_pres_2: "~cīs", "~cītis";
    ind_act_pres_3: "~cit", "~ciunt";
    ind_act_impf_1: "~ciēbam", "~ciēbāmus";
    ind_act_impf_2: "~ciēbās", "~ciēbātis";
    ind_act_impf_3: "~ciēbat", "~ciēbant";
    ind_act_fut_1: "~ciam", "~ciēmus";
    ind_act_fut_2: "~ciēs", "~ciētis";
    ind_act_fut_3: "~ciet", "~cient";
    ind_pass_pres_1: "~cior", "~cīmur";
    ind_pass_pres_2: "~cīris|~cīre", "~cīminī";
    ind_pass_pres_3: "~cītur", "~ciuntur";
    ind_pass_impf_1: "~ciēbar", "~ciēbāmur";
    ind_pass_impf_2: "~ciēbāris|~ciēbāre", "~ciēbāminī";
    ind_pass_impf_3: "~ciēbātur", "~ciēbantur";
    ind_pass_fut_1: "~ciar", "~ciēmur";
    ind_pass_fut_2: "~ciēris|~ciēre", "~ciēminī";
    ind_pass_fut_3: "~ciētur", "~cientur";
    subj_act_pres_1: "~ciam", "~ciāmus";
    subj_act_pres_2: "~ciās", "~ciātis";
    subj_act_pres_3: "~ciat", "~ciant";
    subj_act_impf_1: "~cīrem", "~cīrēmus";
    subj_act_impf_2: "~cīrēs", "~cīrētis";
    subj_act_impf_3: "~cīret", "~cīrent";
    subj_pass_pres_1: "~ciar", "~ciāmur";
    subj_pass_pres_2: "~ciāris|~ciāre", "~ciāminī";
    subj_pass_pres_3: "~ciātur", "~ciantur";
    subj_pass_impf_1: "~cīrer", "~cīrēmur";
    subj_pass_impf_2: "~cīrēris|~cīrēre", "~cīrēminī";
    subj_pass_impf_3: "~cīrētur", "~cīrentur";
    imperative_act_pres_2: "~cī", "~cīte";
    imperative_act_fut_2: "~cītō", "~cītōte";
    imperative_act_fut_3: _, "~ciuntō";
    imperative_pass_pres_2: "~cīre", _;
    imperative_pass_fut_2: "~cītor", _;
    imperative_pass_fut_3: _, "~ciuntor";
    infinitive_pass: "~cīrī", _, _;
    participle: "~ciēns", _, _, "~ciendus|~ciundus";
    gerund: _, "~ciendī", "~ciendō", "~ciendum", _, _;
    ),
    // Solvō: 21 words (volvō, solvō, ēvolvō, convolvō, dissolvō, resolvō). Like Quiēscō, except in 73 cells: ind_act_pres_1 sg, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ….
    conjugation!("solvō", 21, "~vō", like Quiēscō as PRES_AS_PERF;
    stem participle_perf_pass: "~ūt- like agitō";
    ind_act_pres_1: "~vō", _;
    ind_act_pres_2: "~vis", "~vitis";
    ind_act_pres_3: _, "~vunt";
    ind_act_impf_1: "~vēbam", "~vēbāmus";
    ind_act_impf_2: "~vēbās", "~vēbātis";
    ind_act_impf_3: "~vēbat", "~vēbant";
    ind_act_fut_1: "~vam", "~vēmus";
    ind_act_fut_2: "~vēs", "~vētis";
    ind_act_fut_3: "~vet", "~vent";
    ind_act_perf_3: _, "~vērunt|~vēre";
    ind_pass_pres_1: "~vor", "~vimur";
    ind_pass_pres_2: "~veris|~vere", "~viminī";
    ind_pass_pres_3: "~vitur", "~vuntur";
    ind_pass_impf_1: "~vēbar", "~vēbāmur";
    ind_pass_impf_2: "~vēbāris|~vēbāre", "~vēbāminī";
    ind_pass_impf_3: "~vēbātur", "~vēbantur";
    ind_pass_fut_1: "~var", "~vēmur";
    ind_pass_fut_2: "~vēris|~vēre", "~vēminī";
    ind_pass_fut_3: "~vētur", "~ventur";
    subj_act_pres_1: "~vam", "~vāmus";
    subj_act_pres_2: "~vās", "~vātis";
    subj_act_pres_3: "~vat", "~vant";
    subj_act_impf_1: "~verem", "~verēmus";
    subj_act_impf_2: "~verēs", "~verētis";
    subj_act_impf_3: "~veret", "~verent";
    subj_pass_pres_1: "~var", "~vāmur";
    subj_pass_pres_2: "~vāris|~vāre", "~vāminī";
    subj_pass_pres_3: "~vātur", "~vantur";
    subj_pass_impf_1: "~verer", "~verēmur";
    subj_pass_impf_2: "~verēris|~verēre", "~verēminī";
    subj_pass_impf_3: "~verētur", "~verentur";
    imperative_act_pres_2: "~ve", "~vite";
    imperative_act_fut_2: "~vitō", "~vitōte";
    imperative_act_fut_3: _, "~vuntō";
    imperative_pass_pres_2: "~vere", _;
    imperative_pass_fut_2: "~vitor", _;
    imperative_pass_fut_3: _, "~vuntor";
    infinitive_pass: "~vī", _, _;
    participle: "~vēns", _, _, "~vendus";
    gerund: _, "~vendī", "~vendō", "~vendum", _, _;
    ),
    // Nāscor: 21 words (ulcīscor, īrāscor, nāscor, vēscor, aborīscor, oblīvīscor). Like Quiēscō, except in 39 cells: ind_act_impf_2 sg, ind_act_fut_2 sg, ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ….
    conjugation!("nāscor", 21, "~scor", like Quiēscō as ACT_AS_PASS;
    stem verb_act_sg: "~sc- like cōnsequor";
    ind_act_impf_2: "~scēbāris|~scēbāre", _;
    ind_act_fut_2: "~scēris|~scēre", _;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_act_fut_2: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: _, derived;
    infinitive_pass: derived, _, _;
    participle: _, derived, _, "~scendus|~scundus";
    ),
    // Abnuō: 20 words (ajō, bibō, uō, lambō, fervō, abscondō). Like Compescō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("abnuō", 20, "~ō", like Compescō;
    ind_act_perf_1: "~ī", "~imus";
    ind_act_perf_2: "~istī", "~istis";
    ind_act_perf_3: "~it", "~ērunt|~ēre";
    ind_act_plup_1: "~eram", "~erāmus";
    ind_act_plup_2: "~erās", "~erātis";
    ind_act_plup_3: "~erat", "~erant";
    ind_act_futp_1: "~erō", "~erimus";
    ind_act_futp_2: "~eris", "~eritis";
    ind_act_futp_3: "~erit", "~erint";
    subj_act_perf_1: "~erim", "~erīmus";
    subj_act_perf_2: "~erīs", "~erītis";
    subj_act_plup_1: "~issem", "~issēmus";
    subj_act_plup_2: "~issēs", "~issētis";
    subj_act_plup_3: "~isset", "~issent";
    infinitive_act: _, "~isse", _;
    ),
    // Cēdō: 19 words (cēdō, discēdō, concēdō, excēdō, prōcēdō, retrōcēdō).
    conjugation!("cēdō", 19, "~ēdō";
    stem participle_perf_pass: "~ess- like agitō";
    ind_act_pres_1: "~ēdō", "~ēdimus";
    ind_act_pres_2: "~ēdis", "~ēditis";
    ind_act_pres_3: "~ēdit", "~ēdunt";
    ind_act_impf_1: "~ēdēbam", "~ēdēbāmus";
    ind_act_impf_2: "~ēdēbās", "~ēdēbātis";
    ind_act_impf_3: "~ēdēbat", "~ēdēbant";
    ind_act_fut_1: "~ēdam", "~ēdēmus";
    ind_act_fut_2: "~ēdēs", "~ēdētis";
    ind_act_fut_3: "~ēdet", "~ēdent";
    ind_act_perf_1: "~essī", "~essimus";
    ind_act_perf_2: "~essistī", "~essistis";
    ind_act_perf_3: "~essit", "~essērunt|~essēre";
    ind_act_plup_1: "~esseram", "~esserāmus";
    ind_act_plup_2: "~esserās", "~esserātis";
    ind_act_plup_3: "~esserat", "~esserant";
    ind_act_futp_1: "~esserō", "~esserimus";
    ind_act_futp_2: "~esseris", "~esseritis";
    ind_act_futp_3: "~esserit", "~esserint";
    ind_pass_pres_1: "~ēdor", "~ēdimur";
    ind_pass_pres_2: "~ēderis|~ēdere", "~ēdiminī";
    ind_pass_pres_3: "~ēditur", "~ēduntur";
    ind_pass_impf_1: "~ēdēbar", "~ēdēbāmur";
    ind_pass_impf_2: "~ēdēbāris|~ēdēbāre", "~ēdēbāminī";
    ind_pass_impf_3: "~ēdēbātur", "~ēdēbantur";
    ind_pass_fut_1: "~ēdar", "~ēdēmur";
    ind_pass_fut_2: "~ēdēris|~ēdēre", "~ēdēminī";
    ind_pass_fut_3: "~ēdētur", "~ēdentur";
    subj_act_pres_1: "~ēdam", "~ēdāmus";
    subj_act_pres_2: "~ēdās", "~ēdātis";
    subj_act_pres_3: "~ēdat", "~ēdant";
    subj_act_impf_1: "~ēderem", "~ēderēmus";
    subj_act_impf_2: "~ēderēs", "~ēderētis";
    subj_act_impf_3: "~ēderet", "~ēderent";
    subj_act_perf_1: "~esserim", "~esserīmus";
    subj_act_perf_2: "~esserīs", "~esserītis";
    subj_act_plup_1: "~essissem", "~essissēmus";
    subj_act_plup_2: "~essissēs", "~essissētis";
    subj_act_plup_3: "~essisset", "~essissent";
    subj_pass_pres_1: "~ēdar", "~ēdāmur";
    subj_pass_pres_2: "~ēdāris|~ēdāre", "~ēdāminī";
    subj_pass_pres_3: "~ēdātur", "~ēdantur";
    subj_pass_impf_1: "~ēderer", "~ēderēmur";
    subj_pass_impf_2: "~ēderēris|~ēderēre", "~ēderēminī";
    subj_pass_impf_3: "~ēderētur", "~ēderentur";
    imperative_act_pres_2: "~ēde", "~ēdite";
    imperative_act_fut_2: "~ēditō", "~ēditōte";
    imperative_act_fut_3: _, "~ēduntō";
    imperative_pass_pres_2: "~ēdere", _;
    imperative_pass_fut_2: "~ēditor", _;
    imperative_pass_fut_3: _, "~ēduntor";
    infinitive_act: _, "~essisse", _;
    infinitive_pass: "~ēdī", _, _;
    participle: "~ēdēns", _, _, "~ēdendus|~ēdundus";
    gerund: _, "~ēdendī", "~ēdendō", "~ēdendum", _, _;
    ),
    // Possideō: 19 words (videō, resideō, assideō, invideō, revideō, īnsideō).
    conjugation!("possideō", 19, "~ideō";
    stem participle_perf_pass: "~ess- like agitō";
    ind_act_pres_1: "~ideō", "~idēmus";
    ind_act_pres_2: "~idēs", "~idētis";
    ind_act_pres_3: "~idet", "~ident";
    ind_act_impf_1: "~idēbam", "~idēbāmus";
    ind_act_impf_2: "~idēbās", "~idēbātis";
    ind_act_impf_3: "~idēbat", "~idēbant";
    ind_act_fut_1: "~idēbō", "~idēbimus";
    ind_act_fut_2: "~idēbis", "~idēbitis";
    ind_act_fut_3: "~idēbit", "~idēbunt";
    ind_act_perf_1: "~ēdī", "~ēdimus";
    ind_act_perf_2: "~ēdistī", "~ēdistis";
    ind_act_perf_3: "~ēdit", "~ēdērunt|~ēdēre";
    ind_act_plup_1: "~ēderam", "~ēderāmus";
    ind_act_plup_2: "~ēderās", "~ēderātis";
    ind_act_plup_3: "~ēderat", "~ēderant";
    ind_act_futp_1: "~ēderō", "~ēderimus";
    ind_act_futp_2: "~ēderis", "~ēderitis";
    ind_act_futp_3: "~ēderit", "~ēderint";
    ind_pass_pres_1: "~ideor", "~idēmur";
    ind_pass_pres_2: "~idēris", "~idēminī";
    ind_pass_pres_3: "~idētur", "~identur";
    ind_pass_impf_1: "~idēbar", "~idēbāmur";
    ind_pass_impf_2: "~idēbāris", "~idēbāminī";
    ind_pass_impf_3: "~idēbātur", "~idēbantur";
    ind_pass_fut_1: "~idēbor", "~idēbimur";
    ind_pass_fut_2: "~idēberis", "~idēbiminī";
    ind_pass_fut_3: "~idēbitur", "~idēbuntur";
    subj_act_pres_1: "~ideam", "~ideāmus";
    subj_act_pres_2: "~ideās", "~ideātis";
    subj_act_pres_3: "~ideat", "~ideant";
    subj_act_impf_1: "~idērem", "~idērēmus";
    subj_act_impf_2: "~idērēs", "~idērētis";
    subj_act_impf_3: "~idēret", "~idērent";
    subj_act_perf_1: "~ēderim", "~ēderīmus";
    subj_act_perf_2: "~ēderīs", "~ēderītis";
    subj_act_plup_1: "~ēdissem", "~ēdissēmus";
    subj_act_plup_2: "~ēdissēs", "~ēdissētis";
    subj_act_plup_3: "~ēdisset", "~ēdissent";
    subj_pass_pres_1: "~idear", "~ideāmur";
    subj_pass_pres_2: "~ideāris", "~ideāminī";
    subj_pass_pres_3: "~ideātur", "~ideantur";
    subj_pass_impf_1: "~idērer", "~idērēmur";
    subj_pass_impf_2: "~idērēris", "~idērēminī";
    subj_pass_impf_3: "~idērētur", "~idērentur";
    imperative_act_pres_2: "~idē", "~idēte";
    imperative_act_fut_2: "~idētō", "~idētōte";
    imperative_act_fut_3: _, "~identō";
    imperative_pass_pres_2: "~idēre", _;
    imperative_pass_fut_2: "~idētor", _;
    imperative_pass_fut_3: _, "~identor";
    infinitive_act: _, "~ēdisse", _;
    infinitive_pass: "~idērī", _, _;
    participle: "~idēns", _, _, "~idendus";
    gerund: _, "~idendī", "~idendō", "~idendum", _, _;
    ),
    // Ēgredior: 19 words (ēgredior, gradior, aggredior, trānsgredior, antegredior, congredior). Like Cōnsequor, except in 19 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("ēgredior", 19, "~dior", like Cōnsequor;
    stem verb_act_sg: "~d- like ēgredior";
    stem participle_perf_pass: "~ss- like modulor";
    ind_act_pres_1: _, "~dimur";
    ind_act_impf_1: "~diēbar", "~diēbāmur";
    ind_act_impf_2: "~diēbāris|~diēbāre", "~diēbāminī";
    ind_act_impf_3: "~diēbātur", "~diēbantur";
    ind_act_fut_1: "~diar", "~diēmur";
    ind_act_fut_2: "~diēris|~diēre", "~diēminī";
    ind_act_fut_3: "~diētur", _;
    subj_act_pres_3: "~diātur", _;
    imperative_act_fut_2: "~ditor", _;
    participle: "~diēns", _, _, "~diendus|~diundus";
    gerund: _, "~diendī", "~diendō", "~diendum", _, _;
    ),
    // Referō: 18 words (trānsferō, praeferō, īnferō, adferō, auferō, cōnferō).
    conjugation!("referō", 18, "~ferō";
    stem participle_perf_pass: "~lāt- like agitō";
    ind_act_pres_1: "~ferō", "~ferimus";
    ind_act_pres_2: "~fers", "~fertis";
    ind_act_pres_3: "~fert", "~ferunt";
    ind_act_impf_1: "~ferēbam", "~ferēbāmus";
    ind_act_impf_2: "~ferēbās", "~ferēbātis";
    ind_act_impf_3: "~ferēbat", "~ferēbant";
    ind_act_fut_1: "~feram", "~ferēmus";
    ind_act_fut_2: "~ferēs", "~ferētis";
    ind_act_fut_3: "~feret", "~ferent";
    ind_act_perf_1: "~tulī", "~tulimus";
    ind_act_perf_2: "~tulistī", "~tulistis";
    ind_act_perf_3: "~tulit", "~tulērunt|~tulēre";
    ind_act_plup_1: "~tuleram", "~tulerāmus";
    ind_act_plup_2: "~tulerās", "~tulerātis";
    ind_act_plup_3: "~tulerat", "~tulerant";
    ind_act_futp_1: "~tulerō", "~tulerimus";
    ind_act_futp_2: "~tuleris", "~tuleritis";
    ind_act_futp_3: "~tulerit", "~tulerint";
    ind_pass_pres_1: "~feror", "~ferimur";
    ind_pass_pres_2: "~ferris|~ferre", "~feriminī";
    ind_pass_pres_3: "~fertur", "~feruntur";
    ind_pass_impf_1: "~ferēbar", "~ferēbāmur";
    ind_pass_impf_2: "~ferēbāris|~ferēbāre", "~ferēbāminī";
    ind_pass_impf_3: "~ferēbātur", "~ferēbantur";
    ind_pass_fut_1: "~ferar", "~ferēmur";
    ind_pass_fut_2: "~ferēris|~ferēre", "~ferēminī";
    ind_pass_fut_3: "~ferētur", "~ferentur";
    subj_act_pres_1: "~feram", "~ferāmus";
    subj_act_pres_2: "~ferās", "~ferātis";
    subj_act_pres_3: "~ferat", "~ferant";
    subj_act_impf_1: "~ferrem", "~ferrēmus";
    subj_act_impf_2: "~ferrēs", "~ferrētis";
    subj_act_impf_3: "~ferret", "~ferrent";
    subj_act_perf_1: "~tulerim", "~tulerīmus";
    subj_act_perf_2: "~tulerīs", "~tulerītis";
    subj_act_plup_1: "~tulissem", "~tulissēmus";
    subj_act_plup_2: "~tulissēs", "~tulissētis";
    subj_act_plup_3: "~tulisset", "~tulissent";
    subj_pass_pres_1: "~ferar", "~ferāmur";
    subj_pass_pres_2: "~ferāris|~ferāre", "~ferāminī";
    subj_pass_pres_3: "~ferātur", "~ferantur";
    subj_pass_impf_1: "~ferrer", "~ferrēmur";
    subj_pass_impf_2: "~ferrēris|~ferrēre", "~ferrēminī";
    subj_pass_impf_3: "~ferrētur", "~ferrentur";
    imperative_act_pres_2: "~fer", "~ferte";
    imperative_act_fut_2: "~fertō", "~fertōte";
    imperative_act_fut_3: _, "~feruntō";
    imperative_pass_pres_2: "~ferre", _;
    imperative_pass_fut_2: "~fertor", _;
    imperative_pass_fut_3: _, "~feruntor";
    infinitive_act: _, "~tulisse", _;
    infinitive_pass: "~ferrī", _, _;
    participle: "~ferēns", _, _, "~ferendus|~ferundus";
    gerund: _, "~ferendī", "~ferendō", "~ferendum", _, _;
    ),
    // Dispiciō: 17 words (perspiciō, retrōspiciō, aspiciō, respiciō, cōnspiciō, suspiciō).
    conjugation!("dispiciō", 17, "~iciō";
    stem participle_perf_pass: "~ect- like agitō";
    ind_act_pres_1: "~iciō", "~icimus";
    ind_act_pres_2: "~icis", "~icitis";
    ind_act_pres_3: "~icit", "~iciunt";
    ind_act_impf_1: "~iciēbam", "~iciēbāmus";
    ind_act_impf_2: "~iciēbās", "~iciēbātis";
    ind_act_impf_3: "~iciēbat", "~iciēbant";
    ind_act_fut_1: "~iciam", "~iciēmus";
    ind_act_fut_2: "~iciēs", "~iciētis";
    ind_act_fut_3: "~iciet", "~icient";
    ind_act_perf_1: "~exī", "~eximus";
    ind_act_perf_2: "~existī", "~existis";
    ind_act_perf_3: "~exit", "~exērunt|~exēre";
    ind_act_plup_1: "~exeram", "~exerāmus";
    ind_act_plup_2: "~exerās", "~exerātis";
    ind_act_plup_3: "~exerat", "~exerant";
    ind_act_futp_1: "~exerō", "~exerimus";
    ind_act_futp_2: "~exeris", "~exeritis";
    ind_act_futp_3: "~exerit", "~exerint";
    ind_pass_pres_1: "~icior", "~icimur";
    ind_pass_pres_2: "~iceris|~icere", "~iciminī";
    ind_pass_pres_3: "~icitur", "~iciuntur";
    ind_pass_impf_1: "~iciēbar", "~iciēbāmur";
    ind_pass_impf_2: "~iciēbāris|~iciēbāre", "~iciēbāminī";
    ind_pass_impf_3: "~iciēbātur", "~iciēbantur";
    ind_pass_fut_1: "~iciar", "~iciēmur";
    ind_pass_fut_2: "~iciēris|~iciēre", "~iciēminī";
    ind_pass_fut_3: "~iciētur", "~icientur";
    subj_act_pres_1: "~iciam", "~iciāmus";
    subj_act_pres_2: "~iciās", "~iciātis";
    subj_act_pres_3: "~iciat", "~iciant";
    subj_act_impf_1: "~icerem", "~icerēmus";
    subj_act_impf_2: "~icerēs", "~icerētis";
    subj_act_impf_3: "~iceret", "~icerent";
    subj_act_perf_1: "~exerim", "~exerīmus";
    subj_act_perf_2: "~exerīs", "~exerītis";
    subj_act_plup_1: "~exissem", "~exissēmus";
    subj_act_plup_2: "~exissēs", "~exissētis";
    subj_act_plup_3: "~exisset", "~exissent";
    subj_pass_pres_1: "~iciar", "~iciāmur";
    subj_pass_pres_2: "~iciāris|~iciāre", "~iciāminī";
    subj_pass_pres_3: "~iciātur", "~iciantur";
    subj_pass_impf_1: "~icerer", "~icerēmur";
    subj_pass_impf_2: "~icerēris|~icerēre", "~icerēminī";
    subj_pass_impf_3: "~icerētur", "~icerentur";
    imperative_act_pres_2: "~ice", "~icite";
    imperative_act_fut_2: "~icitō", "~icitōte";
    imperative_act_fut_3: _, "~iciuntō";
    imperative_pass_pres_2: "~icere", _;
    imperative_pass_fut_2: "~icitor", _;
    imperative_pass_fut_3: _, "~iciuntor";
    infinitive_act: _, "~exisse", _;
    infinitive_pass: "~icī", _, _;
    participle: "~iciēns", _, _, "~iciendus|~iciundus";
    gerund: _, "~iciendī", "~iciendō", "~iciendum", _, _;
    ),
    // Trahō: 17 words (extrahō, trahō, abstrahō, subtrahō, attrahō, circumtrahō).
    conjugation!("trahō", 17, "~ahō";
    stem participle_perf_pass: "~act- like agitō";
    ind_act_pres_1: "~ahō", "~ahimus";
    ind_act_pres_2: "~ahis", "~ahitis";
    ind_act_pres_3: "~ahit", "~ahunt";
    ind_act_impf_1: "~ahēbam", "~ahēbāmus";
    ind_act_impf_2: "~ahēbās", "~ahēbātis";
    ind_act_impf_3: "~ahēbat", "~ahēbant";
    ind_act_fut_1: "~aham", "~ahēmus";
    ind_act_fut_2: "~ahēs", "~ahētis";
    ind_act_fut_3: "~ahet", "~ahent";
    ind_act_perf_1: "~āxī", "~āximus";
    ind_act_perf_2: "~āxistī", "~āxistis";
    ind_act_perf_3: "~āxit", "~āxērunt|~āxēre";
    ind_act_plup_1: "~āxeram", "~āxerāmus";
    ind_act_plup_2: "~āxerās", "~āxerātis";
    ind_act_plup_3: "~āxerat", "~āxerant";
    ind_act_futp_1: "~āxerō", "~āxerimus";
    ind_act_futp_2: "~āxeris", "~āxeritis";
    ind_act_futp_3: "~āxerit", "~āxerint";
    ind_pass_pres_1: "~ahor", "~ahimur";
    ind_pass_pres_2: "~aheris|~ahere", "~ahiminī";
    ind_pass_pres_3: "~ahitur", "~ahuntur";
    ind_pass_impf_1: "~ahēbar", "~ahēbāmur";
    ind_pass_impf_2: "~ahēbāris|~ahēbāre", "~ahēbāminī";
    ind_pass_impf_3: "~ahēbātur", "~ahēbantur";
    ind_pass_fut_1: "~ahar", "~ahēmur";
    ind_pass_fut_2: "~ahēris|~ahēre", "~ahēminī";
    ind_pass_fut_3: "~ahētur", "~ahentur";
    subj_act_pres_1: "~aham", "~ahāmus";
    subj_act_pres_2: "~ahās", "~ahātis";
    subj_act_pres_3: "~ahat", "~ahant";
    subj_act_impf_1: "~aherem", "~aherēmus";
    subj_act_impf_2: "~aherēs", "~aherētis";
    subj_act_impf_3: "~aheret", "~aherent";
    subj_act_perf_1: "~āxerim", "~āxerīmus";
    subj_act_perf_2: "~āxerīs", "~āxerītis";
    subj_act_plup_1: "~āxissem", "~āxissēmus";
    subj_act_plup_2: "~āxissēs", "~āxissētis";
    subj_act_plup_3: "~āxisset", "~āxissent";
    subj_pass_pres_1: "~ahar", "~ahāmur";
    subj_pass_pres_2: "~ahāris|~ahāre", "~ahāminī";
    subj_pass_pres_3: "~ahātur", "~ahantur";
    subj_pass_impf_1: "~aherer", "~aherēmur";
    subj_pass_impf_2: "~aherēris|~aherēre", "~aherēminī";
    subj_pass_impf_3: "~aherētur", "~aherentur";
    imperative_act_pres_2: "~ahe", "~ahite";
    imperative_act_fut_2: "~ahitō", "~ahitōte";
    imperative_act_fut_3: _, "~ahuntō";
    imperative_pass_pres_2: "~ahere", _;
    imperative_pass_fut_2: "~ahitor", _;
    imperative_pass_fut_3: _, "~ahuntor";
    infinitive_act: _, "~āxisse", _;
    infinitive_pass: "~ahī", _, _;
    participle: "~ahēns", _, _, "~ahendus|~ahundus";
    gerund: _, "~ahendī", "~ahendō", "~ahendum", _, _;
    ),
    // Compingō: 17 words (attingō, contingō, impingō, adtingō, compingō, cōnfringō). Like Perstringō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("compingō", 17, "~ingō", like Perstringō;
    stem participle_perf_pass: "~āct- like agitō";
    ind_act_perf_1: "~ēgī", "~ēgimus";
    ind_act_perf_2: "~ēgistī", "~ēgistis";
    ind_act_perf_3: "~ēgit", "~ēgērunt|~ēgēre";
    ind_act_plup_1: "~ēgeram", "~ēgerāmus";
    ind_act_plup_2: "~ēgerās", "~ēgerātis";
    ind_act_plup_3: "~ēgerat", "~ēgerant";
    ind_act_futp_1: "~ēgerō", "~ēgerimus";
    ind_act_futp_2: "~ēgeris", "~ēgeritis";
    ind_act_futp_3: "~ēgerit", "~ēgerint";
    subj_act_perf_1: "~ēgerim", "~ēgerīmus";
    subj_act_perf_2: "~ēgerīs", "~ēgerītis";
    subj_act_plup_1: "~ēgissem", "~ēgissēmus";
    subj_act_plup_2: "~ēgissēs", "~ēgissētis";
    subj_act_plup_3: "~ēgisset", "~ēgissent";
    infinitive_act: _, "~ēgisse", _;
    ),
    // Dēsinō: 17 words (sinō, linō, dēsinō, oblinō, collinō, perlinō). Like Impleō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("dēsinō", 17, "~inō", like Impleō;
    stem participle_perf_pass: "~it- like agitō";
    ind_act_pres_1: "~inō", "~inimus";
    ind_act_pres_2: "~inis", "~initis";
    ind_act_pres_3: "~init", "~inunt";
    ind_act_impf_1: "~inēbam", "~inēbāmus";
    ind_act_impf_2: "~inēbās", "~inēbātis";
    ind_act_impf_3: "~inēbat", "~inēbant";
    ind_act_fut_1: "~inam", "~inēmus";
    ind_act_fut_2: "~inēs", "~inētis";
    ind_act_fut_3: "~inet", "~inent";
    ind_pass_pres_1: "~inor", "~inimur";
    ind_pass_pres_2: "~ineris|~inere", "~iniminī";
    ind_pass_pres_3: "~initur", "~inuntur";
    ind_pass_impf_1: "~inēbar", "~inēbāmur";
    ind_pass_impf_2: "~inēbāris|~inēbāre", "~inēbāminī";
    ind_pass_impf_3: "~inēbātur", "~inēbantur";
    ind_pass_fut_1: "~inar", "~inēmur";
    ind_pass_fut_2: "~inēris|~inēre", "~inēminī";
    ind_pass_fut_3: "~inētur", "~inentur";
    subj_act_pres_1: "~inam", "~ināmus";
    subj_act_pres_2: "~inās", "~inātis";
    subj_act_pres_3: "~inat", "~inant";
    subj_act_impf_1: "~inerem", "~inerēmus";
    subj_act_impf_2: "~inerēs", "~inerētis";
    subj_act_impf_3: "~ineret", "~inerent";
    subj_pass_pres_1: "~inar", "~ināmur";
    subj_pass_pres_2: "~ināris|~ināre", "~ināminī";
    subj_pass_pres_3: "~inātur", "~inantur";
    subj_pass_impf_1: "~inerer", "~inerēmur";
    subj_pass_impf_2: "~inerēris|~inerēre", "~inerēminī";
    subj_pass_impf_3: "~inerētur", "~inerentur";
    imperative_act_pres_2: "~ine", "~inite";
    imperative_act_fut_2: "~initō", "~initōte";
    imperative_act_fut_3: _, "~inuntō";
    imperative_pass_pres_2: "~inere", _;
    imperative_pass_fut_2: "~initor", _;
    imperative_pass_fut_3: _, "~inuntor";
    infinitive_pass: "~inī", _, _;
    participle: "~inēns", _, _, "~inendus|~inundus";
    gerund: _, "~inendī", "~inendō", "~inendum", _, _;
    ),
    // Nectō: 16 words (flectō, īnflectō, genūflectō, reflectō, pectō, plectō).
    conjugation!("nectō", 16, "~ctō";
    stem participle_perf_pass: "~x- like agitō";
    ind_act_pres_1: "~ctō", "~ctimus";
    ind_act_pres_2: "~ctis", "~ctitis";
    ind_act_pres_3: "~ctit", "~ctunt";
    ind_act_impf_1: "~ctēbam", "~ctēbāmus";
    ind_act_impf_2: "~ctēbās", "~ctēbātis";
    ind_act_impf_3: "~ctēbat", "~ctēbant";
    ind_act_fut_1: "~ctam", "~ctēmus";
    ind_act_fut_2: "~ctēs", "~ctētis";
    ind_act_fut_3: "~ctet", "~ctent";
    ind_act_perf_1: "~xī", "~ximus";
    ind_act_perf_2: "~xistī", "~xistis";
    ind_act_perf_3: "~xit", "~xērunt|~xēre";
    ind_act_plup_1: "~xeram", "~xerāmus";
    ind_act_plup_2: "~xerās", "~xerātis";
    ind_act_plup_3: "~xerat", "~xerant";
    ind_act_futp_1: "~xerō", "~xerimus";
    ind_act_futp_2: "~xeris", "~xeritis";
    ind_act_futp_3: "~xerit", "~xerint";
    ind_pass_pres_1: "~ctor", "~ctimur";
    ind_pass_pres_2: "~cteris|~ctere", "~ctiminī";
    ind_pass_pres_3: "~ctitur", "~ctuntur";
    ind_pass_impf_1: "~ctēbar", "~ctēbāmur";
    ind_pass_impf_2: "~ctēbāris|~ctēbāre", "~ctēbāminī";
    ind_pass_impf_3: "~ctēbātur", "~ctēbantur";
    ind_pass_fut_1: "~ctar", "~ctēmur";
    ind_pass_fut_2: "~ctēris|~ctēre", "~ctēminī";
    ind_pass_fut_3: "~ctētur", "~ctentur";
    subj_act_pres_1: "~ctam", "~ctāmus";
    subj_act_pres_2: "~ctās", "~ctātis";
    subj_act_pres_3: "~ctat", "~ctant";
    subj_act_impf_1: "~cterem", "~cterēmus";
    subj_act_impf_2: "~cterēs", "~cterētis";
    subj_act_impf_3: "~cteret", "~cterent";
    subj_act_perf_1: "~xerim", "~xerīmus";
    subj_act_perf_2: "~xerīs", "~xerītis";
    subj_act_plup_1: "~xissem", "~xissēmus";
    subj_act_plup_2: "~xissēs", "~xissētis";
    subj_act_plup_3: "~xisset", "~xissent";
    subj_pass_pres_1: "~ctar", "~ctāmur";
    subj_pass_pres_2: "~ctāris|~ctāre", "~ctāminī";
    subj_pass_pres_3: "~ctātur", "~ctantur";
    subj_pass_impf_1: "~cterer", "~cterēmur";
    subj_pass_impf_2: "~cterēris|~cterēre", "~cterēminī";
    subj_pass_impf_3: "~cterētur", "~cterentur";
    imperative_act_pres_2: "~cte", "~ctite";
    imperative_act_fut_2: "~ctitō", "~ctitōte";
    imperative_act_fut_3: _, "~ctuntō";
    imperative_pass_pres_2: "~ctere", _;
    imperative_pass_fut_2: "~ctitor", _;
    imperative_pass_fut_3: _, "~ctuntor";
    infinitive_act: _, "~xisse", _;
    infinitive_pass: "~ctī", _, _;
    participle: "~ctēns", _, _, "~ctendus|~ctundus";
    gerund: _, "~ctendī", "~ctendō", "~ctendum", _, _;
    ),
    // Corripiō: 16 words (ēripiō, abripiō, dēripiō, alliciō, arripiō, dīripiō). Like Excipiō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("corripiō", 16, "~i~iō", like Excipiō;
    ind_act_perf_1: "~i~uī", "~i~uimus";
    ind_act_perf_2: "~i~uistī", "~i~uistis";
    ind_act_perf_3: "~i~uit", "~i~uērunt|~i~uēre";
    ind_act_plup_1: "~i~ueram", "~i~uerāmus";
    ind_act_plup_2: "~i~uerās", "~i~uerātis";
    ind_act_plup_3: "~i~uerat", "~i~uerant";
    ind_act_futp_1: "~i~uerō", "~i~uerimus";
    ind_act_futp_2: "~i~ueris", "~i~ueritis";
    ind_act_futp_3: "~i~uerit", "~i~uerint";
    subj_act_perf_1: "~i~uerim", "~i~uerīmus";
    subj_act_perf_2: "~i~uerīs", "~i~uerītis";
    subj_act_plup_1: "~i~uissem", "~i~uissēmus";
    subj_act_plup_2: "~i~uissēs", "~i~uissētis";
    subj_act_plup_3: "~i~uisset", "~i~uissent";
    infinitive_act: _, "~i~uisse", _;
    ),
    // Effundō: 16 words (profundō, cōnfundō, īnfundō, effundō, circumfundō, diffundō).
    conjugation!("effundō", 16, "~undō";
    stem participle_perf_pass: "~ūs- like agitō";
    ind_act_pres_1: "~undō", "~undimus";
    ind_act_pres_2: "~undis", "~unditis";
    ind_act_pres_3: "~undit", "~undunt";
    ind_act_impf_1: "~undēbam", "~undēbāmus";
    ind_act_impf_2: "~undēbās", "~undēbātis";
    ind_act_impf_3: "~undēbat", "~undēbant";
    ind_act_fut_1: "~undam", "~undēmus";
    ind_act_fut_2: "~undēs", "~undētis";
    ind_act_fut_3: "~undet", "~undent";
    ind_act_perf_1: "~ūdī", "~ūdimus";
    ind_act_perf_2: "~ūdistī", "~ūdistis";
    ind_act_perf_3: "~ūdit", "~ūdērunt|~ūdēre";
    ind_act_plup_1: "~ūderam", "~ūderāmus";
    ind_act_plup_2: "~ūderās", "~ūderātis";
    ind_act_plup_3: "~ūderat", "~ūderant";
    ind_act_futp_1: "~ūderō", "~ūderimus";
    ind_act_futp_2: "~ūderis", "~ūderitis";
    ind_act_futp_3: "~ūderit", "~ūderint";
    ind_pass_pres_1: "~undor", "~undimur";
    ind_pass_pres_2: "~underis|~undere", "~undiminī";
    ind_pass_pres_3: "~unditur", "~unduntur";
    ind_pass_impf_1: "~undēbar", "~undēbāmur";
    ind_pass_impf_2: "~undēbāris|~undēbāre", "~undēbāminī";
    ind_pass_impf_3: "~undēbātur", "~undēbantur";
    ind_pass_fut_1: "~undar", "~undēmur";
    ind_pass_fut_2: "~undēris|~undēre", "~undēminī";
    ind_pass_fut_3: "~undētur", "~undentur";
    subj_act_pres_1: "~undam", "~undāmus";
    subj_act_pres_2: "~undās", "~undātis";
    subj_act_pres_3: "~undat", "~undant";
    subj_act_impf_1: "~underem", "~underēmus";
    subj_act_impf_2: "~underēs", "~underētis";
    subj_act_impf_3: "~underet", "~underent";
    subj_act_perf_1: "~ūderim", "~ūderīmus";
    subj_act_perf_2: "~ūderīs", "~ūderītis";
    subj_act_plup_1: "~ūdissem", "~ūdissēmus";
    subj_act_plup_2: "~ūdissēs", "~ūdissētis";
    subj_act_plup_3: "~ūdisset", "~ūdissent";
    subj_pass_pres_1: "~undar", "~undāmur";
    subj_pass_pres_2: "~undāris|~undāre", "~undāminī";
    subj_pass_pres_3: "~undātur", "~undantur";
    subj_pass_impf_1: "~underer", "~underēmur";
    subj_pass_impf_2: "~underēris|~underēre", "~underēminī";
    subj_pass_impf_3: "~underētur", "~underentur";
    imperative_act_pres_2: "~unde", "~undite";
    imperative_act_fut_2: "~unditō", "~unditōte";
    imperative_act_fut_3: _, "~unduntō";
    imperative_pass_pres_2: "~undere", _;
    imperative_pass_fut_2: "~unditor", _;
    imperative_pass_fut_3: _, "~unduntor";
    infinitive_act: _, "~ūdisse", _;
    infinitive_pass: "~undī", _, _;
    participle: "~undēns", _, _, "~undendus|~undundus";
    gerund: _, "~undendī", "~undendō", "~undendum", _, _;
    ),
    // Exigō: 16 words (adigō, redigō, prōdigō, abigō, trānsigō, colligō). Like Compingō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("exigō", 16, "~igō", like Compingō;
    ind_act_pres_1: "~igō", "~igimus";
    ind_act_pres_2: "~igis", "~igitis";
    ind_act_pres_3: "~igit", "~igunt";
    ind_act_impf_1: "~igēbam", "~igēbāmus";
    ind_act_impf_2: "~igēbās", "~igēbātis";
    ind_act_impf_3: "~igēbat", "~igēbant";
    ind_act_fut_1: "~igam", "~igēmus";
    ind_act_fut_2: "~igēs", "~igētis";
    ind_act_fut_3: "~iget", "~igent";
    ind_pass_pres_1: "~igor", "~igimur";
    ind_pass_pres_2: "~igeris|~igere", "~igiminī";
    ind_pass_pres_3: "~igitur", "~iguntur";
    ind_pass_impf_1: "~igēbar", "~igēbāmur";
    ind_pass_impf_2: "~igēbāris|~igēbāre", "~igēbāminī";
    ind_pass_impf_3: "~igēbātur", "~igēbantur";
    ind_pass_fut_1: "~igar", "~igēmur";
    ind_pass_fut_2: "~igēris|~igēre", "~igēminī";
    ind_pass_fut_3: "~igētur", "~igentur";
    subj_act_pres_1: "~igam", "~igāmus";
    subj_act_pres_2: "~igās", "~igātis";
    subj_act_pres_3: "~igat", "~igant";
    subj_act_impf_1: "~igerem", "~igerēmus";
    subj_act_impf_2: "~igerēs", "~igerētis";
    subj_act_impf_3: "~igeret", "~igerent";
    subj_pass_pres_1: "~igar", "~igāmur";
    subj_pass_pres_2: "~igāris|~igāre", "~igāminī";
    subj_pass_pres_3: "~igātur", "~igantur";
    subj_pass_impf_1: "~igerer", "~igerēmur";
    subj_pass_impf_2: "~igerēris|~igerēre", "~igerēminī";
    subj_pass_impf_3: "~igerētur", "~igerentur";
    imperative_act_pres_2: "~ige", "~igite";
    imperative_act_fut_2: "~igitō", "~igitōte";
    imperative_act_fut_3: _, "~iguntō";
    imperative_pass_pres_2: "~igere", _;
    imperative_pass_fut_2: "~igitor", _;
    imperative_pass_fut_3: _, "~iguntor";
    infinitive_pass: "~igī", _, _;
    participle: "~igēns", _, _, "~igendus|~igundus";
    gerund: _, "~igendī", "~igendō", "~igendum", _, _;
    ),
    // Accumbō: 16 words (recumbō, dēlambō, accumbō, adcumbō, succumbō, prōcumbō).
    conjugation!("accumbō", 16, "~m~ō";
    stem participle_perf_pass: "~~it- like agitō";
    ind_act_pres_1: "~m~ō", "~m~imus";
    ind_act_pres_2: "~m~is", "~m~itis";
    ind_act_pres_3: "~m~it", "~m~unt";
    ind_act_impf_1: "~m~ēbam", "~m~ēbāmus";
    ind_act_impf_2: "~m~ēbās", "~m~ēbātis";
    ind_act_impf_3: "~m~ēbat", "~m~ēbant";
    ind_act_fut_1: "~m~am", "~m~ēmus";
    ind_act_fut_2: "~m~ēs", "~m~ētis";
    ind_act_fut_3: "~m~et", "~m~ent";
    ind_act_perf_1: "~~uī", "~~uimus";
    ind_act_perf_2: "~~uistī", "~~uistis";
    ind_act_perf_3: "~~uit", "~~uērunt|~~uēre";
    ind_act_plup_1: "~~ueram", "~~uerāmus";
    ind_act_plup_2: "~~uerās", "~~uerātis";
    ind_act_plup_3: "~~uerat", "~~uerant";
    ind_act_futp_1: "~~uerō", "~~uerimus";
    ind_act_futp_2: "~~ueris", "~~ueritis";
    ind_act_futp_3: "~~uerit", "~~uerint";
    ind_pass_pres_1: "~m~or", "~m~imur";
    ind_pass_pres_2: "~m~eris|~m~ere", "~m~iminī";
    ind_pass_pres_3: "~m~itur", "~m~untur";
    ind_pass_impf_1: "~m~ēbar", "~m~ēbāmur";
    ind_pass_impf_2: "~m~ēbāris|~m~ēbāre", "~m~ēbāminī";
    ind_pass_impf_3: "~m~ēbātur", "~m~ēbantur";
    ind_pass_fut_1: "~m~ar", "~m~ēmur";
    ind_pass_fut_2: "~m~ēris|~m~ēre", "~m~ēminī";
    ind_pass_fut_3: "~m~ētur", "~m~entur";
    subj_act_pres_1: "~m~am", "~m~āmus";
    subj_act_pres_2: "~m~ās", "~m~ātis";
    subj_act_pres_3: "~m~at", "~m~ant";
    subj_act_impf_1: "~m~erem", "~m~erēmus";
    subj_act_impf_2: "~m~erēs", "~m~erētis";
    subj_act_impf_3: "~m~eret", "~m~erent";
    subj_act_perf_1: "~~uerim", "~~uerīmus";
    subj_act_perf_2: "~~uerīs", "~~uerītis";
    subj_act_plup_1: "~~uissem", "~~uissēmus";
    subj_act_plup_2: "~~uissēs", "~~uissētis";
    subj_act_plup_3: "~~uisset", "~~uissent";
    subj_pass_pres_1: "~m~ar", "~m~āmur";
    subj_pass_pres_2: "~m~āris|~m~āre", "~m~āminī";
    subj_pass_pres_3: "~m~ātur", "~m~antur";
    subj_pass_impf_1: "~m~erer", "~m~erēmur";
    subj_pass_impf_2: "~m~erēris|~m~erēre", "~m~erēminī";
    subj_pass_impf_3: "~m~erētur", "~m~erentur";
    imperative_act_pres_2: "~m~e", "~m~ite";
    imperative_act_fut_2: "~m~itō", "~m~itōte";
    imperative_act_fut_3: _, "~m~untō";
    imperative_pass_pres_2: "~m~ere", _;
    imperative_pass_fut_2: "~m~itor", _;
    imperative_pass_fut_3: _, "~m~untor";
    infinitive_act: _, "~~uisse", _;
    infinitive_pass: "~m~ī", _, _;
    participle: "~m~ēns", _, _, "~m~endus|~m~undus";
    gerund: _, "~m~endī", "~m~endō", "~m~endum", _, _;
    ),
    // Nītor: 15 words (revertor, deūtor, ūtor, abūtor, nītor, amplector). Like Cōnsequor, except in 19 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("nītor", 15, "~tor", like Cōnsequor;
    stem verb_act_sg: "~t- like cōnsequor";
    stem participle_perf_pass: "~s- like modulor";
    ind_act_pres_1: _, "~timur";
    ind_act_impf_1: "~tēbar", "~tēbāmur";
    ind_act_impf_2: "~tēbāris|~tēbāre", "~tēbāminī";
    ind_act_impf_3: "~tēbātur", "~tēbantur";
    ind_act_fut_1: "~tar", "~tēmur";
    ind_act_fut_2: "~tēris|~tēre", "~tēminī";
    ind_act_fut_3: "~tētur", _;
    subj_act_pres_3: "~tātur", _;
    imperative_act_fut_2: "~titor", _;
    participle: "~tēns", _, _, "~tendus|~tundus";
    gerund: _, "~tendī", "~tendō", "~tendum", _, _;
    ),
    // Extruō: 15 words (tāgō, frīgō, sūgō, tragō, discingō, succingō). Like Petō, except in 30 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("extruō", 15, "~ō", like Petō;
    stem participle_perf_pass: "~ct- like agitō";
    ind_act_perf_1: "~xī", "~ximus";
    ind_act_perf_2: "~xistī", "~xistis";
    ind_act_perf_3: "~xit", "~xērunt|~xēre";
    ind_act_plup_1: "~xeram", "~xerāmus";
    ind_act_plup_2: "~xerās", "~xerātis";
    ind_act_plup_3: "~xerat", "~xerant";
    ind_act_futp_1: "~xerō", "~xerimus";
    ind_act_futp_2: "~xeris", "~xeritis";
    ind_act_futp_3: "~xerit", "~xerint";
    subj_act_perf_1: "~xerim", "~xerīmus";
    subj_act_perf_2: "~xerīs", "~xerītis";
    subj_act_plup_1: "~xissem", "~xissēmus";
    subj_act_plup_2: "~xissēs", "~xissētis";
    subj_act_plup_3: "~xisset", "~xissent";
    infinitive_act: _, "~xisse", _;
    participle: _, _, _, "~endus";
    ),
    // Dīcō: 14 words (abdīcō, dīcō, addīcō, interdīcō, maledīcō, benedīcō).
    conjugation!("dīcō", 14, "~īcō";
    stem participle_perf_pass: "~ict- like agitō";
    ind_act_pres_1: "~īcō", "~īcimus";
    ind_act_pres_2: "~īcis", "~īcitis";
    ind_act_pres_3: "~īcit", "~īcunt";
    ind_act_impf_1: "~īcēbam", "~īcēbāmus";
    ind_act_impf_2: "~īcēbās", "~īcēbātis";
    ind_act_impf_3: "~īcēbat", "~īcēbant";
    ind_act_fut_1: "~īcam", "~īcēmus";
    ind_act_fut_2: "~īcēs", "~īcētis";
    ind_act_fut_3: "~īcet", "~īcent";
    ind_act_perf_1: "~īxī", "~īximus";
    ind_act_perf_2: "~īxistī", "~īxistis";
    ind_act_perf_3: "~īxit", "~īxērunt|~īxēre";
    ind_act_plup_1: "~īxeram", "~īxerāmus";
    ind_act_plup_2: "~īxerās", "~īxerātis";
    ind_act_plup_3: "~īxerat", "~īxerant";
    ind_act_futp_1: "~īxerō", "~īxerimus";
    ind_act_futp_2: "~īxeris", "~īxeritis";
    ind_act_futp_3: "~īxerit", "~īxerint";
    ind_pass_pres_1: "~īcor", "~īcimur";
    ind_pass_pres_2: "~īceris|~īcere", "~īciminī";
    ind_pass_pres_3: "~īcitur", "~īcuntur";
    ind_pass_impf_1: "~īcēbar", "~īcēbāmur";
    ind_pass_impf_2: "~īcēbāris|~īcēbāre", "~īcēbāminī";
    ind_pass_impf_3: "~īcēbātur", "~īcēbantur";
    ind_pass_fut_1: "~īcar", "~īcēmur";
    ind_pass_fut_2: "~īcēris|~īcēre", "~īcēminī";
    ind_pass_fut_3: "~īcētur", "~īcentur";
    subj_act_pres_1: "~īcam", "~īcāmus";
    subj_act_pres_2: "~īcās", "~īcātis";
    subj_act_pres_3: "~īcat", "~īcant";
    subj_act_impf_1: "~īcerem", "~īcerēmus";
    subj_act_impf_2: "~īcerēs", "~īcerētis";
    subj_act_impf_3: "~īceret", "~īcerent";
    subj_act_perf_1: "~īxerim", "~īxerīmus";
    subj_act_perf_2: "~īxerīs", "~īxerītis";
    subj_act_plup_1: "~īxissem", "~īxissēmus";
    subj_act_plup_2: "~īxissēs", "~īxissētis";
    subj_act_plup_3: "~īxisset", "~īxissent";
    subj_pass_pres_1: "~īcar", "~īcāmur";
    subj_pass_pres_2: "~īcāris|~īcāre", "~īcāminī";
    subj_pass_pres_3: "~īcātur", "~īcantur";
    subj_pass_impf_1: "~īcerer", "~īcerēmur";
    subj_pass_impf_2: "~īcerēris|~īcerēre", "~īcerēminī";
    subj_pass_impf_3: "~īcerētur", "~īcerentur";
    imperative_act_pres_2: "~īc|~īce", "~īcite";
    imperative_act_fut_2: "~īcitō", "~īcitōte";
    imperative_act_fut_3: _, "~īcuntō";
    imperative_pass_pres_2: "~īcere", _;
    imperative_pass_fut_2: "~īcitor", _;
    imperative_pass_fut_3: _, "~īcuntor";
    infinitive_act: _, "~īxisse", _;
    infinitive_pass: "~īcī", _, _;
    participle: "~īcēns", _, _, "~īcendus|~īcundus";
    gerund: _, "~īcendī", "~īcendō", "~īcendum", _, _;
    ),
    // Regō: 14 words (regō, neglegō, tegō, dētegō, intellegō, prōtegō).
    conjugation!("regō", 14, "~egō";
    stem participle_perf_pass: "~ēct- like agitō";
    ind_act_pres_1: "~egō", "~egimus";
    ind_act_pres_2: "~egis", "~egitis";
    ind_act_pres_3: "~egit", "~egunt";
    ind_act_impf_1: "~egēbam", "~egēbāmus";
    ind_act_impf_2: "~egēbās", "~egēbātis";
    ind_act_impf_3: "~egēbat", "~egēbant";
    ind_act_fut_1: "~egam", "~egēmus";
    ind_act_fut_2: "~egēs", "~egētis";
    ind_act_fut_3: "~eget", "~egent";
    ind_act_perf_1: "~ēxī", "~ēximus";
    ind_act_perf_2: "~ēxistī", "~ēxistis";
    ind_act_perf_3: "~ēxit", "~ēxērunt|~ēxēre";
    ind_act_plup_1: "~ēxeram", "~ēxerāmus";
    ind_act_plup_2: "~ēxerās", "~ēxerātis";
    ind_act_plup_3: "~ēxerat", "~ēxerant";
    ind_act_futp_1: "~ēxerō", "~ēxerimus";
    ind_act_futp_2: "~ēxeris", "~ēxeritis";
    ind_act_futp_3: "~ēxerit", "~ēxerint";
    ind_pass_pres_1: "~egor", "~egimur";
    ind_pass_pres_2: "~egeris|~egere", "~egiminī";
    ind_pass_pres_3: "~egitur", "~eguntur";
    ind_pass_impf_1: "~egēbar", "~egēbāmur";
    ind_pass_impf_2: "~egēbāris|~egēbāre", "~egēbāminī";
    ind_pass_impf_3: "~egēbātur", "~egēbantur";
    ind_pass_fut_1: "~egar", "~egēmur";
    ind_pass_fut_2: "~egēris|~egēre", "~egēminī";
    ind_pass_fut_3: "~egētur", "~egentur";
    subj_act_pres_1: "~egam", "~egāmus";
    subj_act_pres_2: "~egās", "~egātis";
    subj_act_pres_3: "~egat", "~egant";
    subj_act_impf_1: "~egerem", "~egerēmus";
    subj_act_impf_2: "~egerēs", "~egerētis";
    subj_act_impf_3: "~egeret", "~egerent";
    subj_act_perf_1: "~ēxerim", "~ēxerīmus";
    subj_act_perf_2: "~ēxerīs", "~ēxerītis";
    subj_act_plup_1: "~ēxissem", "~ēxissēmus";
    subj_act_plup_2: "~ēxissēs", "~ēxissētis";
    subj_act_plup_3: "~ēxisset", "~ēxissent";
    subj_pass_pres_1: "~egar", "~egāmur";
    subj_pass_pres_2: "~egāris|~egāre", "~egāminī";
    subj_pass_pres_3: "~egātur", "~egantur";
    subj_pass_impf_1: "~egerer", "~egerēmur";
    subj_pass_impf_2: "~egerēris|~egerēre", "~egerēminī";
    subj_pass_impf_3: "~egerētur", "~egerentur";
    imperative_act_pres_2: "~ege", "~egite";
    imperative_act_fut_2: "~egitō", "~egitōte";
    imperative_act_fut_3: _, "~eguntō";
    imperative_pass_pres_2: "~egere", _;
    imperative_pass_fut_2: "~egitor", _;
    imperative_pass_fut_3: _, "~eguntor";
    infinitive_act: _, "~ēxisse", _;
    infinitive_pass: "~egī", _, _;
    participle: "~egēns", _, _, "~egendus|~egundus";
    gerund: _, "~egendī", "~egendō", "~egendum", _, _;
    ),
    // Struō: 14 words (dēstruō, struō, cōnstruō, īnstruō, exstruō, obstruō). Like Minuō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("struō", 14, "~uō", like Minuō;
    stem participle_perf_pass: "~ūct- like agitō";
    ind_act_perf_1: "~ūxī", "~ūximus";
    ind_act_perf_2: "~ūxistī", "~ūxistis";
    ind_act_perf_3: "~ūxit", "~ūxērunt|~ūxēre";
    ind_act_plup_1: "~ūxeram", "~ūxerāmus";
    ind_act_plup_2: "~ūxerās", "~ūxerātis";
    ind_act_plup_3: "~ūxerat", "~ūxerant";
    ind_act_futp_1: "~ūxerō", "~ūxerimus";
    ind_act_futp_2: "~ūxeris", "~ūxeritis";
    ind_act_futp_3: "~ūxerit", "~ūxerint";
    subj_act_perf_1: "~ūxerim", "~ūxerīmus";
    subj_act_perf_2: "~ūxerīs", "~ūxerītis";
    subj_act_plup_1: "~ūxissem", "~ūxissēmus";
    subj_act_plup_2: "~ūxissēs", "~ūxissētis";
    subj_act_plup_3: "~ūxisset", "~ūxissent";
    infinitive_act: _, "~ūxisse", _;
    ),
    // Rumpō: 14 words (irrumpō, interrumpō, abrumpō, rumpō, ērumpō, corrumpō).
    conjugation!("rumpō", 14, "~um~ō";
    stem participle_perf_pass: "~u~t- like agitō";
    ind_act_pres_1: "~um~ō", "~um~imus";
    ind_act_pres_2: "~um~is", "~um~itis";
    ind_act_pres_3: "~um~it", "~um~unt";
    ind_act_impf_1: "~um~ēbam", "~um~ēbāmus";
    ind_act_impf_2: "~um~ēbās", "~um~ēbātis";
    ind_act_impf_3: "~um~ēbat", "~um~ēbant";
    ind_act_fut_1: "~um~am", "~um~ēmus";
    ind_act_fut_2: "~um~ēs", "~um~ētis";
    ind_act_fut_3: "~um~et", "~um~ent";
    ind_act_perf_1: "~ū~ī", "~ū~imus";
    ind_act_perf_2: "~ū~istī", "~ū~istis";
    ind_act_perf_3: "~ū~it", "~ū~ērunt|~ū~ēre";
    ind_act_plup_1: "~ū~eram", "~ū~erāmus";
    ind_act_plup_2: "~ū~erās", "~ū~erātis";
    ind_act_plup_3: "~ū~erat", "~ū~erant";
    ind_act_futp_1: "~ū~erō", "~ū~erimus";
    ind_act_futp_2: "~ū~eris", "~ū~eritis";
    ind_act_futp_3: "~ū~erit", "~ū~erint";
    ind_pass_pres_1: "~um~or", "~um~imur";
    ind_pass_pres_2: "~um~eris|~um~ere", "~um~iminī";
    ind_pass_pres_3: "~um~itur", "~um~untur";
    ind_pass_impf_1: "~um~ēbar", "~um~ēbāmur";
    ind_pass_impf_2: "~um~ēbāris|~um~ēbāre", "~um~ēbāminī";
    ind_pass_impf_3: "~um~ēbātur", "~um~ēbantur";
    ind_pass_fut_1: "~um~ar", "~um~ēmur";
    ind_pass_fut_2: "~um~ēris|~um~ēre", "~um~ēminī";
    ind_pass_fut_3: "~um~ētur", "~um~entur";
    subj_act_pres_1: "~um~am", "~um~āmus";
    subj_act_pres_2: "~um~ās", "~um~ātis";
    subj_act_pres_3: "~um~at", "~um~ant";
    subj_act_impf_1: "~um~erem", "~um~erēmus";
    subj_act_impf_2: "~um~erēs", "~um~erētis";
    subj_act_impf_3: "~um~eret", "~um~erent";
    subj_act_perf_1: "~ū~erim", "~ū~erīmus";
    subj_act_perf_2: "~ū~erīs", "~ū~erītis";
    subj_act_plup_1: "~ū~issem", "~ū~issēmus";
    subj_act_plup_2: "~ū~issēs", "~ū~issētis";
    subj_act_plup_3: "~ū~isset", "~ū~issent";
    subj_pass_pres_1: "~um~ar", "~um~āmur";
    subj_pass_pres_2: "~um~āris|~um~āre", "~um~āminī";
    subj_pass_pres_3: "~um~ātur", "~um~antur";
    subj_pass_impf_1: "~um~erer", "~um~erēmur";
    subj_pass_impf_2: "~um~erēris|~um~erēre", "~um~erēminī";
    subj_pass_impf_3: "~um~erētur", "~um~erentur";
    imperative_act_pres_2: "~um~e", "~um~ite";
    imperative_act_fut_2: "~um~itō", "~um~itōte";
    imperative_act_fut_3: _, "~um~untō";
    imperative_pass_pres_2: "~um~ere", _;
    imperative_pass_fut_2: "~um~itor", _;
    imperative_pass_fut_3: _, "~um~untor";
    infinitive_act: _, "~ū~isse", _;
    infinitive_pass: "~um~ī", _, _;
    participle: "~um~ēns", _, _, "~um~endus|~um~undus";
    gerund: _, "~um~endī", "~um~endō", "~um~endum", _, _;
    ),
    // Fodiō: 14 words (fodiō, cōnfodiō, affodiō, dēfodiō, effodiō, īnfodiō).
    conjugation!("fodiō", 14, "~odiō";
    stem participle_perf_pass: "~oss- like agitō";
    ind_act_pres_1: "~odiō", "~odimus";
    ind_act_pres_2: "~odis", "~oditis";
    ind_act_pres_3: "~odit", "~odiunt";
    ind_act_impf_1: "~odiēbam", "~odiēbāmus";
    ind_act_impf_2: "~odiēbās", "~odiēbātis";
    ind_act_impf_3: "~odiēbat", "~odiēbant";
    ind_act_fut_1: "~odiam", "~odiēmus";
    ind_act_fut_2: "~odiēs", "~odiētis";
    ind_act_fut_3: "~odiet", "~odient";
    ind_act_perf_1: "~ōdī", "~ōdimus";
    ind_act_perf_2: "~ōdistī", "~ōdistis";
    ind_act_perf_3: "~ōdit", "~ōdērunt|~ōdēre";
    ind_act_plup_1: "~ōderam", "~ōderāmus";
    ind_act_plup_2: "~ōderās", "~ōderātis";
    ind_act_plup_3: "~ōderat", "~ōderant";
    ind_act_futp_1: "~ōderō", "~ōderimus";
    ind_act_futp_2: "~ōderis", "~ōderitis";
    ind_act_futp_3: "~ōderit", "~ōderint";
    ind_pass_pres_1: "~odior", "~odimur";
    ind_pass_pres_2: "~oderis|~odere", "~odiminī";
    ind_pass_pres_3: "~oditur", "~odiuntur";
    ind_pass_impf_1: "~odiēbar", "~odiēbāmur";
    ind_pass_impf_2: "~odiēbāris|~odiēbāre", "~odiēbāminī";
    ind_pass_impf_3: "~odiēbātur", "~odiēbantur";
    ind_pass_fut_1: "~odiar", "~odiēmur";
    ind_pass_fut_2: "~odiēris|~odiēre", "~odiēminī";
    ind_pass_fut_3: "~odiētur", "~odientur";
    subj_act_pres_1: "~odiam", "~odiāmus";
    subj_act_pres_2: "~odiās", "~odiātis";
    subj_act_pres_3: "~odiat", "~odiant";
    subj_act_impf_1: "~oderem", "~oderēmus";
    subj_act_impf_2: "~oderēs", "~oderētis";
    subj_act_impf_3: "~oderet", "~oderent";
    subj_act_perf_1: "~ōderim", "~ōderīmus";
    subj_act_perf_2: "~ōderīs", "~ōderītis";
    subj_act_plup_1: "~ōdissem", "~ōdissēmus";
    subj_act_plup_2: "~ōdissēs", "~ōdissētis";
    subj_act_plup_3: "~ōdisset", "~ōdissent";
    subj_pass_pres_1: "~odiar", "~odiāmur";
    subj_pass_pres_2: "~odiāris|~odiāre", "~odiāminī";
    subj_pass_pres_3: "~odiātur", "~odiantur";
    subj_pass_impf_1: "~oderer", "~oderēmur";
    subj_pass_impf_2: "~oderēris|~oderēre", "~oderēminī";
    subj_pass_impf_3: "~oderētur", "~oderentur";
    imperative_act_pres_2: "~ode", "~odite";
    imperative_act_fut_2: "~oditō", "~oditōte";
    imperative_act_fut_3: _, "~odiuntō";
    imperative_pass_pres_2: "~odere", _;
    imperative_pass_fut_2: "~oditor", _;
    imperative_pass_fut_3: _, "~odiuntor";
    infinitive_act: _, "~ōdisse", _;
    infinitive_pass: "~odī", _, _;
    participle: "~odiēns", _, _, "~odiendus|~odiundus";
    gerund: _, "~odiendī", "~odiendō", "~odiendum", _, _;
    ),
    // Ēveniō: 14 words (veniō, reveniō, interveniō, perveniō, adveniō, ēveniō). Like Excipiō, except in 42 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("ēveniō", 14, "~e~iō", like Excipiō as IMPF_AS_FUTP;
    stem verb_act_sg: "~e~- like ēveniō";
    stem participle_perf_pass: "~e~t- like labōrō";
    ind_act_pres_1: _, "~e~īmus";
    ind_act_impf_1: "~e~iēbam", "~e~iēbāmus";
    ind_act_impf_2: "~e~iēbās", "~e~iēbātis";
    ind_act_impf_3: "~e~iēbat", "~e~iēbant";
    ind_act_fut_1: "~e~iam", "~e~iēmus";
    ind_act_fut_2: "~e~iēs", "~e~iētis";
    ind_act_fut_3: "~e~iet", _;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, "~e~iuntur";
    ind_pass_impf_3: _, "~e~iēbantur";
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, "~e~ientur";
    subj_act_pres_3: "~e~iat", _;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: _, "~e~iantur";
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_act_fut_2: "~e~ītō", _;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: _, "~e~iuntor";
    participle: "~e~iēns", _, _, "~e~iendum|~e~iundum";
    gerund: _, "~e~iendī", "~e~iendō", "~e~iendum", _, _;
    ),
    // Nīvit: 14 words (vesperāscit, cōnfit, pluit, advesperāscit, ningit, compluit). Like Tueor, except in 19 cells: ind_act_pres_1 sg, ind_act_pres_3 sg, ind_act_impf_3 sg, ind_act_fut_3 sg, ind_pass_pres_3 sg, ind_pass_impf_3 sg, ….
    conjugation!("nīvit", 14, "~it", like Tueor as ACT_AS_PASS;
    ind_act_pres_1: "~it", _;
    ind_act_pres_3: "~it", _;
    ind_act_impf_3: "~ēbat", _;
    ind_act_fut_3: "~et", _;
    ind_pass_pres_3: "~itur", _;
    ind_pass_impf_3: "~ēbātur", _;
    ind_pass_fut_3: "~ētur", _;
    subj_act_pres_3: "~at", _;
    subj_act_impf_3: "~eret", _;
    subj_pass_pres_3: "~ātur", _;
    subj_pass_impf_3: "~erētur", _;
    imperative_act_fut_3: "~itō", _;
    imperative_pass_fut_3: "~itor", _;
    infinitive_act: "~ere", _, _;
    infinitive_pass: "~ī", _, _;
    participle: _, _, derived, "~endum";
    supine: _, _, _, derived, derived, _;
    ),
    // Tingō: 13 words (lingō, cingō, accingō, circumcingō, incingō, recingō).
    conjugation!("tingō", 13, "~i~gō";
    stem participle_perf_pass: "~ī~ct- like agitō";
    ind_act_pres_1: "~i~gō", "~i~gimus";
    ind_act_pres_2: "~i~gis", "~i~gitis";
    ind_act_pres_3: "~i~git", "~i~gunt";
    ind_act_impf_1: "~i~gēbam", "~i~gēbāmus";
    ind_act_impf_2: "~i~gēbās", "~i~gēbātis";
    ind_act_impf_3: "~i~gēbat", "~i~gēbant";
    ind_act_fut_1: "~i~gam", "~i~gēmus";
    ind_act_fut_2: "~i~gēs", "~i~gētis";
    ind_act_fut_3: "~i~get", "~i~gent";
    ind_act_perf_1: "~ī~xī", "~ī~ximus";
    ind_act_perf_2: "~ī~xistī", "~ī~xistis";
    ind_act_perf_3: "~ī~xit", "~ī~xērunt|~ī~xēre";
    ind_act_plup_1: "~ī~xeram", "~ī~xerāmus";
    ind_act_plup_2: "~ī~xerās", "~ī~xerātis";
    ind_act_plup_3: "~ī~xerat", "~ī~xerant";
    ind_act_futp_1: "~ī~xerō", "~ī~xerimus";
    ind_act_futp_2: "~ī~xeris", "~ī~xeritis";
    ind_act_futp_3: "~ī~xerit", "~ī~xerint";
    ind_pass_pres_1: "~i~gor", "~i~gimur";
    ind_pass_pres_2: "~i~geris|~i~gere", "~i~giminī";
    ind_pass_pres_3: "~i~gitur", "~i~guntur";
    ind_pass_impf_1: "~i~gēbar", "~i~gēbāmur";
    ind_pass_impf_2: "~i~gēbāris|~i~gēbāre", "~i~gēbāminī";
    ind_pass_impf_3: "~i~gēbātur", "~i~gēbantur";
    ind_pass_fut_1: "~i~gar", "~i~gēmur";
    ind_pass_fut_2: "~i~gēris|~i~gēre", "~i~gēminī";
    ind_pass_fut_3: "~i~gētur", "~i~gentur";
    subj_act_pres_1: "~i~gam", "~i~gāmus";
    subj_act_pres_2: "~i~gās", "~i~gātis";
    subj_act_pres_3: "~i~gat", "~i~gant";
    subj_act_impf_1: "~i~gerem", "~i~gerēmus";
    subj_act_impf_2: "~i~gerēs", "~i~gerētis";
    subj_act_impf_3: "~i~geret", "~i~gerent";
    subj_act_perf_1: "~ī~xerim", "~ī~xerīmus";
    subj_act_perf_2: "~ī~xerīs", "~ī~xerītis";
    subj_act_plup_1: "~ī~xissem", "~ī~xissēmus";
    subj_act_plup_2: "~ī~xissēs", "~ī~xissētis";
    subj_act_plup_3: "~ī~xisset", "~ī~xissent";
    subj_pass_pres_1: "~i~gar", "~i~gāmur";
    subj_pass_pres_2: "~i~gāris|~i~gāre", "~i~gāminī";
    subj_pass_pres_3: "~i~gātur", "~i~gantur";
    subj_pass_impf_1: "~i~gerer", "~i~gerēmur";
    subj_pass_impf_2: "~i~gerēris|~i~gerēre", "~i~gerēminī";
    subj_pass_impf_3: "~i~gerētur", "~i~gerentur";
    imperative_act_pres_2: "~i~ge", "~i~gite";
    imperative_act_fut_2: "~i~gitō", "~i~gitōte";
    imperative_act_fut_3: _, "~i~guntō";
    imperative_pass_pres_2: "~i~gere", _;
    imperative_pass_fut_2: "~i~gitor", _;
    imperative_pass_fut_3: _, "~i~guntor";
    infinitive_act: _, "~ī~xisse", _;
    infinitive_pass: "~i~gī", _, _;
    participle: "~i~gēns", _, _, "~i~gendus|~i~gundus";
    gerund: _, "~i~gendī", "~i~gendō", "~i~gendum", _, _;
    ),
    // Illābor: 13 words (lābor, collābor, dēlābor, conlābor, dīlābor, ēlābor). Like Cōnsequor, except in 19 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("illābor", 13, "~bor", like Cōnsequor;
    stem verb_act_sg: "~b- like cōnsequor";
    stem participle_perf_pass: "~ps- like modulor";
    ind_act_pres_1: _, "~bimur";
    ind_act_impf_1: "~bēbar", "~bēbāmur";
    ind_act_impf_2: "~bēbāris|~bēbāre", "~bēbāminī";
    ind_act_impf_3: "~bēbātur", "~bēbantur";
    ind_act_fut_1: "~bar", "~bēmur";
    ind_act_fut_2: "~bēris|~bēre", "~bēminī";
    ind_act_fut_3: "~bētur", _;
    subj_act_pres_3: "~bātur", _;
    imperative_act_fut_2: "~bitor", _;
    participle: "~bēns", _, _, "~bendus|~bundus";
    gerund: _, "~bendī", "~bendō", "~bendum", _, _;
    ),
    // Ēvehō: 13 words (vehō, advehō, āvehō, convehō, dēvehō, prōvehō).
    conjugation!("ēvehō", 13, "~ehō";
    stem participle_perf_pass: "~ect- like agitō";
    ind_act_pres_1: "~ehō", "~ehimus";
    ind_act_pres_2: "~ehis", "~ehitis";
    ind_act_pres_3: "~ehit", "~ehunt";
    ind_act_impf_1: "~ehēbam", "~ehēbāmus";
    ind_act_impf_2: "~ehēbās", "~ehēbātis";
    ind_act_impf_3: "~ehēbat", "~ehēbant";
    ind_act_fut_1: "~eham", "~ehēmus";
    ind_act_fut_2: "~ehēs", "~ehētis";
    ind_act_fut_3: "~ehet", "~ehent";
    ind_act_perf_1: "~ĕxī", "~ĕximus";
    ind_act_perf_2: "~ĕxistī", "~ĕxistis";
    ind_act_perf_3: "~ĕxit", "~ĕxērunt|~ĕxēre";
    ind_act_plup_1: "~ĕxeram", "~ĕxerāmus";
    ind_act_plup_2: "~ĕxerās", "~ĕxerātis";
    ind_act_plup_3: "~ĕxerat", "~ĕxerant";
    ind_act_futp_1: "~ĕxerō", "~ĕxerimus";
    ind_act_futp_2: "~ĕxeris", "~ĕxeritis";
    ind_act_futp_3: "~ĕxerit", "~ĕxerint";
    ind_pass_pres_1: "~ehor", "~ehimur";
    ind_pass_pres_2: "~eheris|~ehere", "~ehiminī";
    ind_pass_pres_3: "~ehitur", "~ehuntur";
    ind_pass_impf_1: "~ehēbar", "~ehēbāmur";
    ind_pass_impf_2: "~ehēbāris|~ehēbāre", "~ehēbāminī";
    ind_pass_impf_3: "~ehēbātur", "~ehēbantur";
    ind_pass_fut_1: "~ehar", "~ehēmur";
    ind_pass_fut_2: "~ehēris|~ehēre", "~ehēminī";
    ind_pass_fut_3: "~ehētur", "~ehentur";
    subj_act_pres_1: "~eham", "~ehāmus";
    subj_act_pres_2: "~ehās", "~ehātis";
    subj_act_pres_3: "~ehat", "~ehant";
    subj_act_impf_1: "~eherem", "~eherēmus";
    subj_act_impf_2: "~eherēs", "~eherētis";
    subj_act_impf_3: "~eheret", "~eherent";
    subj_act_perf_1: "~ĕxerim", "~ĕxerīmus";
    subj_act_perf_2: "~ĕxerīs", "~ĕxerītis";
    subj_act_plup_1: "~ĕxissem", "~ĕxissēmus";
    subj_act_plup_2: "~ĕxissēs", "~ĕxissētis";
    subj_act_plup_3: "~ĕxisset", "~ĕxissent";
    subj_pass_pres_1: "~ehar", "~ehāmur";
    subj_pass_pres_2: "~ehāris|~ehāre", "~ehāminī";
    subj_pass_pres_3: "~ehātur", "~ehantur";
    subj_pass_impf_1: "~eherer", "~eherēmur";
    subj_pass_impf_2: "~eherēris|~eherēre", "~eherēminī";
    subj_pass_impf_3: "~eherētur", "~eherentur";
    imperative_act_pres_2: "~ehe", "~ehite";
    imperative_act_fut_2: "~ehitō", "~ehitōte";
    imperative_act_fut_3: _, "~ehuntō";
    imperative_pass_pres_2: "~ehere", _;
    imperative_pass_fut_2: "~ehitor", _;
    imperative_pass_fut_3: _, "~ehuntor";
    infinitive_act: _, "~ĕxisse", _;
    infinitive_pass: "~ehī", _, _;
    participle: "~ehēns", _, _, "~ehendus|~ehundus";
    gerund: _, "~ehendī", "~ehendō", "~ehendum", _, _;
    ),
    // Madeō: 13 words (studeō, pāreō, horreō, vegeō, sileō, affleō). Like Habeō, except in 25 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("madeō", 13, "~eō", like Habeō;
    stem participle_perf_pass: "~it- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~ētor", _;
    infinitive_act: "~ēre", _, _;
    participle: _, _, _, "~endum";
    ),
    // Dēfīgō: 13 words (fīgō, praefīgō, īnfīgō, cōnfīgō, dēfīgō, trānsfīgō). Like Mergō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("dēfīgō", 13, "~gō", like Mergō;
    stem participle_perf_pass: "~x- like agitō";
    ind_act_perf_1: "~xī", "~ximus";
    ind_act_perf_2: "~xistī", "~xistis";
    ind_act_perf_3: "~xit", "~xērunt|~xēre";
    ind_act_plup_1: "~xeram", "~xerāmus";
    ind_act_plup_2: "~xerās", "~xerātis";
    ind_act_plup_3: "~xerat", "~xerant";
    ind_act_futp_1: "~xerō", "~xerimus";
    ind_act_futp_2: "~xeris", "~xeritis";
    ind_act_futp_3: "~xerit", "~xerint";
    subj_act_perf_1: "~xerim", "~xerīmus";
    subj_act_perf_2: "~xerīs", "~xerītis";
    subj_act_plup_1: "~xissem", "~xissēmus";
    subj_act_plup_2: "~xissēs", "~xissētis";
    subj_act_plup_3: "~xisset", "~xissent";
    infinitive_act: _, "~xisse", _;
    ),
    // Intersaepiō: 13 words (saepiō, sēpiō, reperiō, praesaepiō, cōnsaepiō, cōnsēpiō). Like Mūniō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("intersaepiō", 13, "~iō", like Mūniō;
    stem participle_perf_pass: "~t- like agitō";
    ind_act_perf_1: "~sī", "~simus";
    ind_act_perf_2: "~sistī", "~sistis";
    ind_act_perf_3: "~sit", "~sērunt|~sēre";
    ind_act_plup_1: "~seram", "~serāmus";
    ind_act_plup_2: "~serās", "~serātis";
    ind_act_plup_3: "~serat", "~serant";
    ind_act_futp_1: "~serō", "~serimus";
    ind_act_futp_2: "~seris", "~seritis";
    ind_act_futp_3: "~serit", "~serint";
    subj_act_perf_1: "~serim", "~serīmus";
    subj_act_perf_2: "~serīs", "~serītis";
    subj_act_plup_1: "~sissem", "~sissēmus";
    subj_act_plup_2: "~sissēs", "~sissētis";
    subj_act_plup_3: "~sisset", "~sissent";
    infinitive_act: _, "~sisse", _;
    ),
    // Contendō: 13 words (ostendō, contendō, tendō, attendō, intendō, portendō). Like Praecīdō, except in 0 cells: .
    conjugation!("contendō", 13, "~dō", like Praecīdō;
    stem participle_perf_pass: "~t- like agitō";
    ),
    // Cōnsulō: 12 words (alō, cōnsulō, incumbō, furō, occulō, dēserō). Like Compescō, except in 0 cells: .
    conjugation!("cōnsulō", 12, "~ō", like Compescō;
    stem participle_perf_pass: "~t- like agitō";
    ),
    // Feriō: 12 words (ūniō, feriō, absiliō, ambiō, salliō, lascīviō). Like Mūniō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("feriō", 12, "~iō", like Mūniō;
    ind_act_perf_1: "~iī", "~iimus";
    ind_act_perf_2: "~iistī", "~iistis";
    ind_act_perf_3: "~iit", "~iērunt|~iēre";
    ind_act_plup_1: "~ieram", "~ierāmus";
    ind_act_plup_2: "~ierās", "~ierātis";
    ind_act_plup_3: "~ierat", "~ierant";
    ind_act_futp_1: "~ierō", "~ierimus";
    ind_act_futp_2: "~ieris", "~ieritis";
    ind_act_futp_3: "~ierit", "~ierint";
    subj_act_perf_1: "~ierim", "~ierīmus";
    subj_act_perf_2: "~ierīs", "~ierītis";
    subj_act_plup_1: "~iissem", "~iissēmus";
    subj_act_plup_2: "~iissēs", "~iissētis";
    subj_act_plup_3: "~iisset", "~iissent";
    infinitive_act: _, "~iisse", _;
    ),
    // Fugiō: 12 words (fugiō, subterfugiō, effugiō, trānsfugiō, dēfugiō, supterfugiō). Like Rumpō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("fugiō", 12, "~u~iō", like Rumpō;
    stem participle_perf_pass: "~u~it- like agitō";
    ind_act_pres_1: "~u~iō", "~u~imus";
    ind_act_pres_2: "~u~is", "~u~itis";
    ind_act_pres_3: "~u~it", "~u~iunt";
    ind_act_impf_1: "~u~iēbam", "~u~iēbāmus";
    ind_act_impf_2: "~u~iēbās", "~u~iēbātis";
    ind_act_impf_3: "~u~iēbat", "~u~iēbant";
    ind_act_fut_1: "~u~iam", "~u~iēmus";
    ind_act_fut_2: "~u~iēs", "~u~iētis";
    ind_act_fut_3: "~u~iet", "~u~ient";
    ind_pass_pres_1: "~u~ior", "~u~imur";
    ind_pass_pres_2: "~u~eris", "~u~iminī";
    ind_pass_pres_3: "~u~itur", "~u~iuntur";
    ind_pass_impf_1: "~u~iēbar", "~u~iēbāmur";
    ind_pass_impf_2: "~u~iēbāris", "~u~iēbāminī";
    ind_pass_impf_3: "~u~iēbātur", "~u~iēbantur";
    ind_pass_fut_1: "~u~iar", "~u~iēmur";
    ind_pass_fut_2: "~u~iēris", "~u~iēminī";
    ind_pass_fut_3: "~u~iētur", "~u~ientur";
    subj_act_pres_1: "~u~iam", "~u~iāmus";
    subj_act_pres_2: "~u~iās", "~u~iātis";
    subj_act_pres_3: "~u~iat", "~u~iant";
    subj_act_impf_1: "~u~erem", "~u~erēmus";
    subj_act_impf_2: "~u~erēs", "~u~erētis";
    subj_act_impf_3: "~u~eret", "~u~erent";
    subj_pass_pres_1: "~u~iar", "~u~iāmur";
    subj_pass_pres_2: "~u~iāris", "~u~iāminī";
    subj_pass_pres_3: "~u~iātur", "~u~iantur";
    subj_pass_impf_1: "~u~erer", "~u~erēmur";
    subj_pass_impf_2: "~u~erēris", "~u~erēminī";
    subj_pass_impf_3: "~u~erētur", "~u~erentur";
    imperative_act_pres_2: "~u~e", "~u~ite";
    imperative_act_fut_2: "~u~itō", "~u~itōte";
    imperative_act_fut_3: _, "~u~iuntō";
    imperative_pass_pres_2: "~u~ere", _;
    imperative_pass_fut_2: "~u~itor", _;
    imperative_pass_fut_3: _, "~u~iuntor";
    infinitive_pass: "~u~ī", _, _;
    participle: "~u~iēns", _, _, "~u~iendus";
    gerund: _, "~u~iendī", "~u~iendō", "~u~iendum", _, _;
    ),
    // Scindō: 12 words (rescindō, scindō, abscindō, interscindō, findō, cōnscindō). Like Praecīdō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("scindō", 12, "~ndō", like Praecīdō;
    stem participle_perf_pass: "~ss- like agitō";
    ind_act_pres_1: "~ndō", "~ndimus";
    ind_act_pres_2: "~ndis", "~nditis";
    ind_act_pres_3: "~ndit", "~ndunt";
    ind_act_impf_1: "~ndēbam", "~ndēbāmus";
    ind_act_impf_2: "~ndēbās", "~ndēbātis";
    ind_act_impf_3: "~ndēbat", "~ndēbant";
    ind_act_fut_1: "~ndam", "~ndēmus";
    ind_act_fut_2: "~ndēs", "~ndētis";
    ind_act_fut_3: "~ndet", "~ndent";
    ind_pass_pres_1: "~ndor", "~ndimur";
    ind_pass_pres_2: "~nderis|~ndere", "~ndiminī";
    ind_pass_pres_3: "~nditur", "~nduntur";
    ind_pass_impf_1: "~ndēbar", "~ndēbāmur";
    ind_pass_impf_2: "~ndēbāris|~ndēbāre", "~ndēbāminī";
    ind_pass_impf_3: "~ndēbātur", "~ndēbantur";
    ind_pass_fut_1: "~ndar", "~ndēmur";
    ind_pass_fut_2: "~ndēris|~ndēre", "~ndēminī";
    ind_pass_fut_3: "~ndētur", "~ndentur";
    subj_act_pres_1: "~ndam", "~ndāmus";
    subj_act_pres_2: "~ndās", "~ndātis";
    subj_act_pres_3: "~ndat", "~ndant";
    subj_act_impf_1: "~nderem", "~nderēmus";
    subj_act_impf_2: "~nderēs", "~nderētis";
    subj_act_impf_3: "~nderet", "~nderent";
    subj_pass_pres_1: "~ndar", "~ndāmur";
    subj_pass_pres_2: "~ndāris|~ndāre", "~ndāminī";
    subj_pass_pres_3: "~ndātur", "~ndantur";
    subj_pass_impf_1: "~nderer", "~nderēmur";
    subj_pass_impf_2: "~nderēris|~nderēre", "~nderēminī";
    subj_pass_impf_3: "~nderētur", "~nderentur";
    imperative_act_pres_2: "~nde", "~ndite";
    imperative_act_fut_2: "~nditō", "~nditōte";
    imperative_act_fut_3: _, "~nduntō";
    imperative_pass_pres_2: "~ndere", _;
    imperative_pass_fut_2: "~nditor", _;
    imperative_pass_fut_3: _, "~nduntor";
    infinitive_pass: "~ndī", _, _;
    participle: "~ndēns", _, _, "~ndendus|~ndundus";
    gerund: _, "~ndendī", "~ndendō", "~ndendum", _, _;
    ),
    // Fallō: 12 words (fallō, excellō, extollō, attollō, adtollō, sustollō).
    conjugation!("fallō", 12, "~lō";
    stem participle_perf_pass: "~s- like agitō";
    ind_act_pres_1: "~lō", "~limus";
    ind_act_pres_2: "~lis", "~litis";
    ind_act_pres_3: "~lit", "~lunt";
    ind_act_impf_1: "~lēbam", "~lēbāmus";
    ind_act_impf_2: "~lēbās", "~lēbātis";
    ind_act_impf_3: "~lēbat", "~lēbant";
    ind_act_fut_1: "~lam", "~lēmus";
    ind_act_fut_2: "~lēs", "~lētis";
    ind_act_fut_3: "~let", "~lent";
    ind_act_perf_1: "~luī", "~luimus";
    ind_act_perf_2: "~luistī", "~luistis";
    ind_act_perf_3: "~luit", "~luērunt";
    ind_act_plup_1: "~lueram", "~luerāmus";
    ind_act_plup_2: "~luerās", "~luerātis";
    ind_act_plup_3: "~luerat", "~luerant";
    ind_act_futp_1: "~luerō", "~luerimus";
    ind_act_futp_2: "~lueris", "~lueritis";
    ind_act_futp_3: "~luerit", "~luerint";
    ind_pass_pres_1: "~lor", "~limur";
    ind_pass_pres_2: "~leris", "~liminī";
    ind_pass_pres_3: "~litur", "~luntur";
    ind_pass_impf_1: "~lēbar", "~lēbāmur";
    ind_pass_impf_2: "~lēbāris", "~lēbāminī";
    ind_pass_impf_3: "~lēbātur", "~lēbantur";
    ind_pass_fut_1: "~lar", "~lēmur";
    ind_pass_fut_2: "~lēris", "~lēminī";
    ind_pass_fut_3: "~lētur", "~lentur";
    subj_act_pres_1: "~lam", "~lāmus";
    subj_act_pres_2: "~lās", "~lātis";
    subj_act_pres_3: "~lat", "~lant";
    subj_act_impf_1: "~lerem", "~lerēmus";
    subj_act_impf_2: "~lerēs", "~lerētis";
    subj_act_impf_3: "~leret", "~lerent";
    subj_act_perf_1: "~luerim", "~luerīmus";
    subj_act_perf_2: "~luerīs", "~luerītis";
    subj_act_plup_1: "~luissem", "~luissēmus";
    subj_act_plup_2: "~luissēs", "~luissētis";
    subj_act_plup_3: "~luisset", "~luissent";
    subj_pass_pres_1: "~lar", "~lāmur";
    subj_pass_pres_2: "~lāris", "~lāminī";
    subj_pass_pres_3: "~lātur", "~lantur";
    subj_pass_impf_1: "~lerer", "~lerēmur";
    subj_pass_impf_2: "~lerēris", "~lerēminī";
    subj_pass_impf_3: "~lerētur", "~lerentur";
    imperative_act_pres_2: "~le", "~lite";
    imperative_act_fut_2: "~litō", "~litōte";
    imperative_act_fut_3: _, "~luntō";
    imperative_pass_pres_2: "~lere", _;
    imperative_pass_fut_2: "~litor", _;
    imperative_pass_fut_3: _, "~luntor";
    infinitive_act: _, "~luisse", _;
    infinitive_pass: "~lī", _, _;
    participle: "~lēns", _, _, "~lendus";
    gerund: _, "~lendī", "~lendō", "~lendum", _, _;
    ),
    // Adsum: 12 words (possum, absum, adsum, īnsum, dēsum, obsum). Like Modulor, except in 49 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("adsum", 12, "~sum", like Modulor as ACC_AS_VOC;
    stem verb_act_sg: "~- like adsum";
    ind_act_pres_1: _, "~sumus";
    ind_act_impf_1: "~eram", "~erāmus";
    ind_act_impf_2: "~erās", "~erātis";
    ind_act_impf_3: "~erat", "~erant";
    ind_act_fut_1: "~erō", "~erimus";
    ind_act_fut_2: "~eris|~ere", "~eritis";
    ind_act_fut_3: "~erit", _;
    ind_act_perf_1: "~fuī", "~fuimus";
    ind_act_perf_2: "~fuistī", "~fuistis";
    ind_act_perf_3: "~fuit", "~fuērunt|~fuēre";
    ind_act_plup_1: "~fueram", "~fuerāmus";
    ind_act_plup_2: "~fuerās", "~fuerātis";
    ind_act_plup_3: "~fuerat", "~fuerant";
    ind_act_futp_1: "~fuerō", "~fuerimus";
    ind_act_futp_2: "~fueris", "~fueritis";
    ind_act_futp_3: "~fuerit", "~fuerint";
    subj_act_pres_3: "~sit|~siet|~fuat", _;
    subj_act_perf_1: "~fuerim", "~fuerīmus";
    subj_act_perf_2: "~fuerīs", "~fuerītis";
    subj_act_plup_1: "~fuissem", "~fuissēmus";
    subj_act_plup_2: "~fuissēs", "~fuissētis";
    subj_act_plup_3: "~fuisset", "~fuissent";
    imperative_act_fut_2: "~estō", _;
    infinitive_act: _, "~fuisse", _;
    participle: "~sēns", _, "~futūrus", derived;
    gerund: _, derived, derived, _, _, _;
    supine: _, _, _, _, derived, _;
    ),
    // Misceō: 11 words (misceō, admisceō, commisceō, immisceō, intermisceō, ammisceō). Like Quiēscō, except in 78 cells: ind_act_pres_1 sg, ind_act_fut_1 sg, ind_act_fut_1 pl, ind_act_fut_2 sg, ind_act_fut_2 pl, ind_act_fut_3 sg, ….
    conjugation!("misceō", 11, "~sceō", like Quiēscō as PRES_AS_FUT;
    stem participle_perf_pass: "~xt- like agitō";
    ind_act_pres_1: "~sceō", _;
    ind_act_fut_1: "~scēbō", "~scēbimus";
    ind_act_fut_2: "~scēbis", "~scēbitis";
    ind_act_fut_3: "~scēbit", "~scēbunt";
    ind_act_perf_1: "~scuī", "~scuimus";
    ind_act_perf_2: "~scuistī", "~scuistis";
    ind_act_perf_3: "~scuit", "~scuērunt|~scuēre";
    ind_act_plup_1: "~scueram", "~scuerāmus";
    ind_act_plup_2: "~scuerās", "~scuerātis";
    ind_act_plup_3: "~scuerat", "~scuerant";
    ind_act_futp_1: "~scuerō", "~scuerimus";
    ind_act_futp_2: "~scueris", "~scueritis";
    ind_act_futp_3: "~scuerit", "~scuerint";
    ind_pass_pres_1: "~sceor", _;
    ind_pass_pres_2: "~scēris|~scēre", _;
    ind_pass_impf_2: "~scēbāris|~scēbāre", _;
    ind_pass_fut_1: "~scēbor", "~scēbimur";
    ind_pass_fut_2: "~scēberis|~scēbere", "~scēbiminī";
    ind_pass_fut_3: "~scēbitur", "~scēbuntur";
    subj_act_pres_1: "~sceam", "~sceāmus";
    subj_act_pres_2: "~sceās", "~sceātis";
    subj_act_pres_3: "~sceat", "~sceant";
    subj_act_impf_1: "~scērem", "~scērēmus";
    subj_act_impf_2: "~scērēs", "~scērētis";
    subj_act_impf_3: "~scēret", "~scērent";
    subj_act_perf_1: "~scuerim", "~scuerīmus";
    subj_act_perf_2: "~scuerīs", "~scuerītis";
    subj_act_plup_1: "~scuissem", "~scuissēmus";
    subj_act_plup_2: "~scuissēs", "~scuissētis";
    subj_act_plup_3: "~scuisset", "~scuissent";
    subj_pass_pres_1: "~scear", "~sceāmur";
    subj_pass_pres_2: "~sceāris|~sceāre", "~sceāminī";
    subj_pass_pres_3: "~sceātur", "~sceantur";
    subj_pass_impf_1: "~scērer", "~scērēmur";
    subj_pass_impf_2: "~scērēris|~scērēre", "~scērēminī";
    subj_pass_impf_3: "~scērētur", "~scērentur";
    imperative_act_pres_2: "~scē", "~scēte";
    imperative_act_fut_2: "~scētō", "~scētōte";
    imperative_act_fut_3: _, "~scentō";
    imperative_pass_pres_2: "~scēre", _;
    imperative_pass_fut_2: "~scētor", _;
    imperative_pass_fut_3: _, "~scentor";
    infinitive_act: _, "~scuisse", _;
    infinitive_pass: "~scērī", _, _;
    ),
    // Contineō: 11 words (contineō, abstineō, dētineō, attineō, distineō, obtineō). Like Corripiō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("contineō", 11, "~i~eō", like Corripiō;
    ind_act_pres_1: "~i~eō", "~i~ēmus";
    ind_act_pres_2: "~i~ēs", "~i~ētis";
    ind_act_pres_3: "~i~et", "~i~ent";
    ind_act_impf_1: "~i~ēbam", "~i~ēbāmus";
    ind_act_impf_2: "~i~ēbās", "~i~ēbātis";
    ind_act_impf_3: "~i~ēbat", "~i~ēbant";
    ind_act_fut_1: "~i~ēbō", "~i~ēbimus";
    ind_act_fut_2: "~i~ēbis", "~i~ēbitis";
    ind_act_fut_3: "~i~ēbit", "~i~ēbunt";
    ind_pass_pres_1: "~i~eor", "~i~ēmur";
    ind_pass_pres_2: "~i~ēris|~i~ēre", "~i~ēminī";
    ind_pass_pres_3: "~i~ētur", "~i~entur";
    ind_pass_impf_1: "~i~ēbar", "~i~ēbāmur";
    ind_pass_impf_2: "~i~ēbāris|~i~ēbāre", "~i~ēbāminī";
    ind_pass_impf_3: "~i~ēbātur", "~i~ēbantur";
    ind_pass_fut_1: "~i~ēbor", "~i~ēbimur";
    ind_pass_fut_2: "~i~ēberis|~i~ēbere", "~i~ēbiminī";
    ind_pass_fut_3: "~i~ēbitur", "~i~ēbuntur";
    subj_act_pres_1: "~i~eam", "~i~eāmus";
    subj_act_pres_2: "~i~eās", "~i~eātis";
    subj_act_pres_3: "~i~eat", "~i~eant";
    subj_act_impf_1: "~i~ērem", "~i~ērēmus";
    subj_act_impf_2: "~i~ērēs", "~i~ērētis";
    subj_act_impf_3: "~i~ēret", "~i~ērent";
    subj_pass_pres_1: "~i~ear", "~i~eāmur";
    subj_pass_pres_2: "~i~eāris|~i~eāre", "~i~eāminī";
    subj_pass_pres_3: "~i~eātur", "~i~eantur";
    subj_pass_impf_1: "~i~ērer", "~i~ērēmur";
    subj_pass_impf_2: "~i~ērēris|~i~ērēre", "~i~ērēminī";
    subj_pass_impf_3: "~i~ērētur", "~i~ērentur";
    imperative_act_pres_2: "~i~ē", "~i~ēte";
    imperative_act_fut_2: "~i~ētō", "~i~ētōte";
    imperative_act_fut_3: _, "~i~entō";
    imperative_pass_pres_2: "~i~ēre", _;
    imperative_pass_fut_2: "~i~ētor", _;
    imperative_pass_fut_3: _, "~i~entor";
    infinitive_pass: "~i~ērī", _, _;
    participle: "~i~ēns", _, _, "~i~endus";
    gerund: _, "~i~endī", "~i~endō", "~i~endum", _, _;
    ),
    // Opprimō: 11 words (dēprimō, exprimō, reprimō, comprimō, imprimō, opprimō). Like Cēdō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("opprimō", 11, "~imō", like Cēdō;
    ind_act_pres_1: "~imō", "~imimus";
    ind_act_pres_2: "~imis", "~imitis";
    ind_act_pres_3: "~imit", "~imunt";
    ind_act_impf_1: "~imēbam", "~imēbāmus";
    ind_act_impf_2: "~imēbās", "~imēbātis";
    ind_act_impf_3: "~imēbat", "~imēbant";
    ind_act_fut_1: "~imam", "~imēmus";
    ind_act_fut_2: "~imēs", "~imētis";
    ind_act_fut_3: "~imet", "~iment";
    ind_pass_pres_1: "~imor", "~imimur";
    ind_pass_pres_2: "~imeris|~imere", "~imiminī";
    ind_pass_pres_3: "~imitur", "~imuntur";
    ind_pass_impf_1: "~imēbar", "~imēbāmur";
    ind_pass_impf_2: "~imēbāris|~imēbāre", "~imēbāminī";
    ind_pass_impf_3: "~imēbātur", "~imēbantur";
    ind_pass_fut_1: "~imar", "~imēmur";
    ind_pass_fut_2: "~imēris|~imēre", "~imēminī";
    ind_pass_fut_3: "~imētur", "~imentur";
    subj_act_pres_1: "~imam", "~imāmus";
    subj_act_pres_2: "~imās", "~imātis";
    subj_act_pres_3: "~imat", "~imant";
    subj_act_impf_1: "~imerem", "~imerēmus";
    subj_act_impf_2: "~imerēs", "~imerētis";
    subj_act_impf_3: "~imeret", "~imerent";
    subj_pass_pres_1: "~imar", "~imāmur";
    subj_pass_pres_2: "~imāris|~imāre", "~imāminī";
    subj_pass_pres_3: "~imātur", "~imantur";
    subj_pass_impf_1: "~imerer", "~imerēmur";
    subj_pass_impf_2: "~imerēris|~imerēre", "~imerēminī";
    subj_pass_impf_3: "~imerētur", "~imerentur";
    imperative_act_pres_2: "~ime", "~imite";
    imperative_act_fut_2: "~imitō", "~imitōte";
    imperative_act_fut_3: _, "~imuntō";
    imperative_pass_pres_2: "~imere", _;
    imperative_pass_fut_2: "~imitor", _;
    imperative_pass_fut_3: _, "~imuntor";
    infinitive_pass: "~imī", _, _;
    participle: "~imēns", _, _, "~imendus|~imundus";
    gerund: _, "~imendī", "~imendō", "~imendum", _, _;
    ),
    // Incidō: 11 words (dēcidō, accidō, coincidō, incidō, intercidō, occidō). Like Prōdō, except in 73 cells: ind_act_pres_1 sg, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ….
    conjugation!("incidō", 11, "~idō", like Prōdō as PRES_AS_PERF;
    stem participle_perf_pass: "~ās- like agitō";
    ind_act_pres_1: "~idō", _;
    ind_act_pres_2: "~idis", "~iditis";
    ind_act_pres_3: _, "~idunt";
    ind_act_impf_1: "~idēbam", "~idēbāmus";
    ind_act_impf_2: "~idēbās", "~idēbātis";
    ind_act_impf_3: "~idēbat", "~idēbant";
    ind_act_fut_1: "~idam", "~idēmus";
    ind_act_fut_2: "~idēs", "~idētis";
    ind_act_fut_3: "~idet", "~ident";
    ind_act_perf_3: _, "~idērunt";
    ind_pass_pres_1: "~idor", "~idimur";
    ind_pass_pres_2: "~ideris", "~idiminī";
    ind_pass_pres_3: "~iditur", "~iduntur";
    ind_pass_impf_1: "~idēbar", "~idēbāmur";
    ind_pass_impf_2: "~idēbāris", "~idēbāminī";
    ind_pass_impf_3: "~idēbātur", "~idēbantur";
    ind_pass_fut_1: "~idar", "~idēmur";
    ind_pass_fut_2: "~idēris", "~idēminī";
    ind_pass_fut_3: "~idētur", "~identur";
    subj_act_pres_1: "~idam", "~idāmus";
    subj_act_pres_2: "~idās", "~idātis";
    subj_act_pres_3: "~idat", "~idant";
    subj_act_impf_1: "~iderem", "~iderēmus";
    subj_act_impf_2: "~iderēs", "~iderētis";
    subj_act_impf_3: "~ideret", "~iderent";
    subj_pass_pres_1: "~idar", "~idāmur";
    subj_pass_pres_2: "~idāris", "~idāminī";
    subj_pass_pres_3: "~idātur", "~idantur";
    subj_pass_impf_1: "~iderer", "~iderēmur";
    subj_pass_impf_2: "~iderēris", "~iderēminī";
    subj_pass_impf_3: "~iderētur", "~iderentur";
    imperative_act_pres_2: "~ide", "~idite";
    imperative_act_fut_2: "~iditō", "~iditōte";
    imperative_act_fut_3: _, "~iduntō";
    imperative_pass_pres_2: "~idere", _;
    imperative_pass_fut_2: "~iditor", _;
    imperative_pass_fut_3: _, "~iduntor";
    infinitive_pass: "~idī", _, _;
    participle: "~idēns", _, _, "~idendus";
    gerund: _, "~idendī", "~idendō", "~idendum", _, _;
    ),
    // Cernō: 11 words (cernō, concernō, sēcernō, discernō, dēcernō, condēcernō).
    conjugation!("cernō", 11, "~e~nō";
    stem participle_perf_pass: "~~ēt- like agitō";
    ind_act_pres_1: "~e~nō", "~e~nimus";
    ind_act_pres_2: "~e~nis", "~e~nitis";
    ind_act_pres_3: "~e~nit", "~e~nunt";
    ind_act_impf_1: "~e~nēbam", "~e~nēbāmus";
    ind_act_impf_2: "~e~nēbās", "~e~nēbātis";
    ind_act_impf_3: "~e~nēbat", "~e~nēbant";
    ind_act_fut_1: "~e~nam", "~e~nēmus";
    ind_act_fut_2: "~e~nēs", "~e~nētis";
    ind_act_fut_3: "~e~net", "~e~nent";
    ind_act_perf_1: "~~ēvī", "~~ēvimus";
    ind_act_perf_2: "~~ēvistī", "~~ēvistis";
    ind_act_perf_3: "~~ēvit", "~~ēvērunt|~~ēvēre";
    ind_act_plup_1: "~~ēveram", "~~ēverāmus";
    ind_act_plup_2: "~~ēverās", "~~ēverātis";
    ind_act_plup_3: "~~ēverat", "~~ēverant";
    ind_act_futp_1: "~~ēverō", "~~ēverimus";
    ind_act_futp_2: "~~ēveris", "~~ēveritis";
    ind_act_futp_3: "~~ēverit", "~~ēverint";
    ind_pass_pres_1: "~e~nor", "~e~nimur";
    ind_pass_pres_2: "~e~neris|~e~nere", "~e~niminī";
    ind_pass_pres_3: "~e~nitur", "~e~nuntur";
    ind_pass_impf_1: "~e~nēbar", "~e~nēbāmur";
    ind_pass_impf_2: "~e~nēbāris|~e~nēbāre", "~e~nēbāminī";
    ind_pass_impf_3: "~e~nēbātur", "~e~nēbantur";
    ind_pass_fut_1: "~e~nar", "~e~nēmur";
    ind_pass_fut_2: "~e~nēris|~e~nēre", "~e~nēminī";
    ind_pass_fut_3: "~e~nētur", "~e~nentur";
    subj_act_pres_1: "~e~nam", "~e~nāmus";
    subj_act_pres_2: "~e~nās", "~e~nātis";
    subj_act_pres_3: "~e~nat", "~e~nant";
    subj_act_impf_1: "~e~nerem", "~e~nerēmus";
    subj_act_impf_2: "~e~nerēs", "~e~nerētis";
    subj_act_impf_3: "~e~neret", "~e~nerent";
    subj_act_perf_1: "~~ēverim", "~~ēverīmus";
    subj_act_perf_2: "~~ēverīs", "~~ēverītis";
    subj_act_plup_1: "~~ēvissem", "~~ēvissēmus";
    subj_act_plup_2: "~~ēvissēs", "~~ēvissētis";
    subj_act_plup_3: "~~ēvisset", "~~ēvissent";
    subj_pass_pres_1: "~e~nar", "~e~nāmur";
    subj_pass_pres_2: "~e~nāris|~e~nāre", "~e~nāminī";
    subj_pass_pres_3: "~e~nātur", "~e~nantur";
    subj_pass_impf_1: "~e~nerer", "~e~nerēmur";
    subj_pass_impf_2: "~e~nerēris|~e~nerēre", "~e~nerēminī";
    subj_pass_impf_3: "~e~nerētur", "~e~nerentur";
    imperative_act_pres_2: "~e~ne", "~e~nite";
    imperative_act_fut_2: "~e~nitō", "~e~nitōte";
    imperative_act_fut_3: _, "~e~nuntō";
    imperative_pass_pres_2: "~e~nere", _;
    imperative_pass_fut_2: "~e~nitor", _;
    imperative_pass_fut_3: _, "~e~nuntor";
    infinitive_act: _, "~~ēvisse", _;
    infinitive_pass: "~e~nī", _, _;
    participle: "~e~nēns", _, _, "~e~nendus|~e~nundus";
    gerund: _, "~e~nendī", "~e~nendō", "~e~nendum", _, _;
    ),
    // Decet: 11 words (licet, libet, decet, addecet, miseret, oportet). Like Tueor, except in 24 cells: ind_act_pres_1 sg, ind_act_pres_3 sg, ind_act_pres_3 pl, ind_act_impf_3 sg, ind_act_impf_3 pl, ind_act_fut_3 sg, ….
    conjugation!("decet", 11, "~et", like Tueor as ACT_AS_PASS;
    ind_act_pres_1: "~et", _;
    ind_act_pres_3: "~et", "~ent";
    ind_act_impf_3: "~ēbat", "~ēbant";
    ind_act_fut_3: "~ēbit", "~ēbunt";
    ind_act_perf_3: "~uit", "~uērunt";
    ind_act_plup_3: "~uerat", "~uerant";
    ind_act_futp_3: "~uerit", "~uerint";
    subj_act_pres_3: "~eat", "~eant";
    subj_act_impf_3: "~ēret", "~ērent";
    subj_act_plup_3: "~uisset", "~uissent";
    imperative_act_fut_3: "~ētō", "~entō";
    infinitive_act: "~ēre", "~uisse", _;
    participle: _, _, _, derived;
    ),
    // Percutiō: 11 words (patiō, recutiō, excutiō, quatiō, discutiō, percutiō). Like Gerō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("percutiō", 11, "~tiō", like Gerō;
    stem participle_perf_pass: "~ss- like agitō";
    ind_act_pres_1: "~tiō", "~timus";
    ind_act_pres_2: "~tis", "~titis";
    ind_act_pres_3: "~tit", "~tiunt";
    ind_act_impf_1: "~tiēbam", "~tiēbāmus";
    ind_act_impf_2: "~tiēbās", "~tiēbātis";
    ind_act_impf_3: "~tiēbat", "~tiēbant";
    ind_act_fut_1: "~tiam", "~tiēmus";
    ind_act_fut_2: "~tiēs", "~tiētis";
    ind_act_fut_3: "~tiet", "~tient";
    ind_pass_pres_1: "~tior", "~timur";
    ind_pass_pres_2: "~teris|~tere", "~timinī";
    ind_pass_pres_3: "~titur", "~tiuntur";
    ind_pass_impf_1: "~tiēbar", "~tiēbāmur";
    ind_pass_impf_2: "~tiēbāris|~tiēbāre", "~tiēbāminī";
    ind_pass_impf_3: "~tiēbātur", "~tiēbantur";
    ind_pass_fut_1: "~tiar", "~tiēmur";
    ind_pass_fut_2: "~tiēris|~tiēre", "~tiēminī";
    ind_pass_fut_3: "~tiētur", "~tientur";
    subj_act_pres_1: "~tiam", "~tiāmus";
    subj_act_pres_2: "~tiās", "~tiātis";
    subj_act_pres_3: "~tiat", "~tiant";
    subj_act_impf_1: "~terem", "~terēmus";
    subj_act_impf_2: "~terēs", "~terētis";
    subj_act_impf_3: "~teret", "~terent";
    subj_pass_pres_1: "~tiar", "~tiāmur";
    subj_pass_pres_2: "~tiāris|~tiāre", "~tiāminī";
    subj_pass_pres_3: "~tiātur", "~tiantur";
    subj_pass_impf_1: "~terer", "~terēmur";
    subj_pass_impf_2: "~terēris|~terēre", "~terēminī";
    subj_pass_impf_3: "~terētur", "~terentur";
    imperative_act_pres_2: "~te", "~tite";
    imperative_act_fut_2: "~titō", "~titōte";
    imperative_act_fut_3: _, "~tiuntō";
    imperative_pass_pres_2: "~tere", _;
    imperative_pass_fut_2: "~titor", _;
    imperative_pass_fut_3: _, "~tiuntor";
    infinitive_pass: "~tī", _, _;
    participle: "~tiēns", _, _, "~tiendus|~tiundus";
    gerund: _, "~tiendī", "~tiendō", "~tiendum", _, _;
    ),
    // Aperiō: 11 words (aperiō, operiō, saliō, adaperiō, sarriō, cooperiō). Like Intersaepiō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("aperiō", 11, "~iō", like Intersaepiō;
    ind_act_perf_1: "~uī", "~uimus";
    ind_act_perf_2: "~uistī", "~uistis";
    ind_act_perf_3: "~uit", "~uērunt|~uēre";
    ind_act_plup_1: "~ueram", "~uerāmus";
    ind_act_plup_2: "~uerās", "~uerātis";
    ind_act_plup_3: "~uerat", "~uerant";
    ind_act_futp_1: "~uerō", "~uerimus";
    ind_act_futp_2: "~ueris", "~ueritis";
    ind_act_futp_3: "~uerit", "~uerint";
    subj_act_perf_1: "~uerim", "~uerīmus";
    subj_act_perf_2: "~uerīs", "~uerītis";
    subj_act_plup_1: "~uissem", "~uissēmus";
    subj_act_plup_2: "~uissēs", "~uissētis";
    subj_act_plup_3: "~uisset", "~uissent";
    infinitive_act: _, "~uisse", _;
    ),
    // Respondeō: 11 words (tondeō, respondeō, dēspondeō, spondeō, attondeō, dētondeō).
    conjugation!("respondeō", 11, "~o~deō";
    stem participle_perf_pass: "~ō~s- like agitō";
    ind_act_pres_1: "~o~deō", "~o~dēmus";
    ind_act_pres_2: "~o~dēs", "~o~dētis";
    ind_act_pres_3: "~o~det", "~o~dent";
    ind_act_impf_1: "~o~dēbam", "~o~dēbāmus";
    ind_act_impf_2: "~o~dēbās", "~o~dēbātis";
    ind_act_impf_3: "~o~dēbat", "~o~dēbant";
    ind_act_fut_1: "~o~dēbō", "~o~dēbimus";
    ind_act_fut_2: "~o~dēbis", "~o~dēbitis";
    ind_act_fut_3: "~o~dēbit", "~o~dēbunt";
    ind_act_perf_1: "~o~dī", "~o~dimus";
    ind_act_perf_2: "~o~distī", "~o~distis";
    ind_act_perf_3: "~o~dit", "~o~dērunt|~o~dēre";
    ind_act_plup_1: "~o~deram", "~o~derāmus";
    ind_act_plup_2: "~o~derās", "~o~derātis";
    ind_act_plup_3: "~o~derat", "~o~derant";
    ind_act_futp_1: "~o~derō", "~o~derimus";
    ind_act_futp_2: "~o~deris", "~o~deritis";
    ind_act_futp_3: "~o~derit", "~o~derint";
    ind_pass_pres_1: "~o~deor", "~o~dēmur";
    ind_pass_pres_2: "~o~dēris|~o~dēre", "~o~dēminī";
    ind_pass_pres_3: "~o~dētur", "~o~dentur";
    ind_pass_impf_1: "~o~dēbar", "~o~dēbāmur";
    ind_pass_impf_2: "~o~dēbāris|~o~dēbāre", "~o~dēbāminī";
    ind_pass_impf_3: "~o~dēbātur", "~o~dēbantur";
    ind_pass_fut_1: "~o~dēbor", "~o~dēbimur";
    ind_pass_fut_2: "~o~dēberis|~o~dēbere", "~o~dēbiminī";
    ind_pass_fut_3: "~o~dēbitur", "~o~dēbuntur";
    subj_act_pres_1: "~o~deam", "~o~deāmus";
    subj_act_pres_2: "~o~deās", "~o~deātis";
    subj_act_pres_3: "~o~deat", "~o~deant";
    subj_act_impf_1: "~o~dērem", "~o~dērēmus";
    subj_act_impf_2: "~o~dērēs", "~o~dērētis";
    subj_act_impf_3: "~o~dēret", "~o~dērent";
    subj_act_perf_1: "~o~derim", "~o~derīmus";
    subj_act_perf_2: "~o~derīs", "~o~derītis";
    subj_act_plup_1: "~o~dissem", "~o~dissēmus";
    subj_act_plup_2: "~o~dissēs", "~o~dissētis";
    subj_act_plup_3: "~o~disset", "~o~dissent";
    subj_pass_pres_1: "~o~dear", "~o~deāmur";
    subj_pass_pres_2: "~o~deāris|~o~deāre", "~o~deāminī";
    subj_pass_pres_3: "~o~deātur", "~o~deantur";
    subj_pass_impf_1: "~o~dērer", "~o~dērēmur";
    subj_pass_impf_2: "~o~dērēris|~o~dērēre", "~o~dērēminī";
    subj_pass_impf_3: "~o~dērētur", "~o~dērentur";
    imperative_act_pres_2: "~o~dē", "~o~dēte";
    imperative_act_fut_2: "~o~dētō", "~o~dētōte";
    imperative_act_fut_3: _, "~o~dentō";
    imperative_pass_pres_2: "~o~dēre", _;
    imperative_pass_fut_2: "~o~dētor", _;
    imperative_pass_fut_3: _, "~o~dentor";
    infinitive_act: _, "~o~disse", _;
    infinitive_pass: "~o~dērī", _, _;
    participle: "~o~dēns", _, _, "~o~dendus";
    gerund: _, "~o~dendī", "~o~dendō", "~o~dendum", _, _;
    ),
    // Tergeō: 10 words (abstergeō, mulgeō, urgeō, perurgeō, extergeō, tergeō). Like Mergō, except in 50 cells: ind_act_pres_1 sg, ind_act_fut_1 sg, ind_act_fut_1 pl, ind_act_fut_2 sg, ind_act_fut_2 pl, ind_act_fut_3 sg, ….
    conjugation!("tergeō", 10, "~geō", like Mergō as PRES_AS_FUT;
    ind_act_pres_1: "~geō", _;
    ind_act_fut_1: "~gēbō", "~gēbimus";
    ind_act_fut_2: "~gēbis", "~gēbitis";
    ind_act_fut_3: "~gēbit", "~gēbunt";
    ind_pass_pres_1: "~geor", _;
    ind_pass_pres_2: "~gēris", _;
    ind_pass_impf_2: "~gēbāris", _;
    ind_pass_fut_1: "~gēbor", "~gēbimur";
    ind_pass_fut_2: "~gēberis", "~gēbiminī";
    ind_pass_fut_3: "~gēbitur", "~gēbuntur";
    subj_act_pres_1: "~geam", "~geāmus";
    subj_act_pres_2: "~geās", "~geātis";
    subj_act_pres_3: "~geat", "~geant";
    subj_act_impf_1: "~gērem", "~gērēmus";
    subj_act_impf_2: "~gērēs", "~gērētis";
    subj_act_impf_3: "~gēret", "~gērent";
    subj_pass_pres_1: "~gear", "~geāmur";
    subj_pass_pres_2: "~geāris", "~geāminī";
    subj_pass_pres_3: "~geātur", "~geantur";
    subj_pass_impf_1: "~gērer", "~gērēmur";
    subj_pass_impf_2: "~gērēris", "~gērēminī";
    subj_pass_impf_3: "~gērētur", "~gērentur";
    imperative_act_pres_2: "~gē", "~gēte";
    imperative_act_fut_2: "~gētō", "~gētōte";
    imperative_act_fut_3: _, "~gentō";
    imperative_pass_pres_2: "~gēre", _;
    imperative_pass_fut_2: "~gētor", _;
    imperative_pass_fut_3: _, "~gentor";
    infinitive_pass: "~gērī", _, _;
    participle: _, _, _, "~gendus";
    ),
    // Legō: 10 words (legō, allegō, relegō, perlegō, praelegō, sublegō). Like Regō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("legō", 10, "~egō", like Regō;
    ind_act_perf_1: "~ēgī", "~ēgimus";
    ind_act_perf_2: "~ēgistī", "~ēgistis";
    ind_act_perf_3: "~ēgit", "~ēgērunt|~ēgēre";
    ind_act_plup_1: "~ēgeram", "~ēgerāmus";
    ind_act_plup_2: "~ēgerās", "~ēgerātis";
    ind_act_plup_3: "~ēgerat", "~ēgerant";
    ind_act_futp_1: "~ēgerō", "~ēgerimus";
    ind_act_futp_2: "~ēgeris", "~ēgeritis";
    ind_act_futp_3: "~ēgerit", "~ēgerint";
    subj_act_perf_1: "~ēgerim", "~ēgerīmus";
    subj_act_perf_2: "~ēgerīs", "~ēgerītis";
    subj_act_plup_1: "~ēgissem", "~ēgissēmus";
    subj_act_plup_2: "~ēgissēs", "~ēgissētis";
    subj_act_plup_3: "~ēgisset", "~ēgissent";
    infinitive_act: _, "~ēgisse", _;
    ),
    // Terō: 10 words (terō, interō, dēterō, atterō, exterō, prōterō).
    conjugation!("terō", 10, "~e~ō";
    stem participle_perf_pass: "~~īt- like agitō";
    ind_act_pres_1: "~e~ō", "~e~imus";
    ind_act_pres_2: "~e~is", "~e~itis";
    ind_act_pres_3: "~e~it", "~e~unt";
    ind_act_impf_1: "~e~ēbam", "~e~ēbāmus";
    ind_act_impf_2: "~e~ēbās", "~e~ēbātis";
    ind_act_impf_3: "~e~ēbat", "~e~ēbant";
    ind_act_fut_1: "~e~am", "~e~ēmus";
    ind_act_fut_2: "~e~ēs", "~e~ētis";
    ind_act_fut_3: "~e~et", "~e~ent";
    ind_act_perf_1: "~~īvī", "~~īvimus";
    ind_act_perf_2: "~~īvistī", "~~īvistis";
    ind_act_perf_3: "~~īvit", "~~īvērunt|~~īvēre";
    ind_act_plup_1: "~~īveram", "~~īverāmus";
    ind_act_plup_2: "~~īverās", "~~īverātis";
    ind_act_plup_3: "~~īverat", "~~īverant";
    ind_act_futp_1: "~~īverō", "~~īverimus";
    ind_act_futp_2: "~~īveris", "~~īveritis";
    ind_act_futp_3: "~~īverit", "~~īverint";
    ind_pass_pres_1: "~e~or", "~e~imur";
    ind_pass_pres_2: "~e~eris|~e~ere", "~e~iminī";
    ind_pass_pres_3: "~e~itur", "~e~untur";
    ind_pass_impf_1: "~e~ēbar", "~e~ēbāmur";
    ind_pass_impf_2: "~e~ēbāris|~e~ēbāre", "~e~ēbāminī";
    ind_pass_impf_3: "~e~ēbātur", "~e~ēbantur";
    ind_pass_fut_1: "~e~ar", "~e~ēmur";
    ind_pass_fut_2: "~e~ēris|~e~ēre", "~e~ēminī";
    ind_pass_fut_3: "~e~ētur", "~e~entur";
    subj_act_pres_1: "~e~am", "~e~āmus";
    subj_act_pres_2: "~e~ās", "~e~ātis";
    subj_act_pres_3: "~e~at", "~e~ant";
    subj_act_impf_1: "~e~erem", "~e~erēmus";
    subj_act_impf_2: "~e~erēs", "~e~erētis";
    subj_act_impf_3: "~e~eret", "~e~erent";
    subj_act_perf_1: "~~īverim", "~~īverīmus";
    subj_act_perf_2: "~~īverīs", "~~īverītis";
    subj_act_plup_1: "~~īvissem", "~~īvissēmus";
    subj_act_plup_2: "~~īvissēs", "~~īvissētis";
    subj_act_plup_3: "~~īvisset", "~~īvissent";
    subj_pass_pres_1: "~e~ar", "~e~āmur";
    subj_pass_pres_2: "~e~āris|~e~āre", "~e~āminī";
    subj_pass_pres_3: "~e~ātur", "~e~antur";
    subj_pass_impf_1: "~e~erer", "~e~erēmur";
    subj_pass_impf_2: "~e~erēris|~e~erēre", "~e~erēminī";
    subj_pass_impf_3: "~e~erētur", "~e~erentur";
    imperative_act_pres_2: "~e~e", "~e~ite";
    imperative_act_fut_2: "~e~itō", "~e~itōte";
    imperative_act_fut_3: _, "~e~untō";
    imperative_pass_pres_2: "~e~ere", _;
    imperative_pass_fut_2: "~e~itor", _;
    imperative_pass_fut_3: _, "~e~untor";
    infinitive_act: _, "~~īvisse", _;
    infinitive_pass: "~e~ī", _, _;
    participle: "~e~ēns", _, _, "~e~endus|~e~undus";
    gerund: _, "~e~endī", "~e~endō", "~e~endum", _, _;
    ),
    // Calvor: 10 words (queror, conqueror, locor, calvor, perdor, fraudor). Like Tueor, except in 8 cells: ind_act_pres_1 pl, ind_act_fut_1 sg, ind_act_fut_1 pl, ind_act_fut_2 sg, ind_act_fut_2 pl, ind_act_fut_3 sg, ….
    conjugation!("calvor", 10, "~or", like Tueor;
    stem verb_act_sg: "~- like cōnsequor";
    ind_act_pres_1: _, "~imur";
    ind_act_fut_1: "~ar", "~ēmur";
    ind_act_fut_2: "~ēris|~ēre", "~ēminī";
    ind_act_fut_3: "~ētur", _;
    subj_act_pres_3: "~ātur", _;
    imperative_act_fut_2: "~itor", _;
    ),
    // Vertō: 9 words (bētō, adbītō, complectō, interbītō, circumvertō, circumvortō). Like Nītor, except in 66 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("vertō", 9, "~tō", like Nītor as PASS_AS_ACT;
    stem participle_perf_pass: "~s- like agitō";
    ind_act_pres_1: "~tō", "~timus";
    ind_act_pres_2: "~tis", "~titis";
    ind_act_pres_3: "~tit", "~tunt";
    ind_act_impf_1: "~tēbam", "~tēbāmus";
    ind_act_impf_2: "~tēbās", "~tēbātis";
    ind_act_impf_3: "~tēbat", "~tēbant";
    ind_act_fut_1: "~tam", "~tēmus";
    ind_act_fut_2: "~tēs", "~tētis";
    ind_act_fut_3: "~tet", "~tent";
    ind_act_perf_1: "~tī", "~timus";
    ind_act_perf_2: "~tistī", "~tistis";
    ind_act_perf_3: "~tit", "~tērunt|~tēre";
    ind_act_plup_1: "~teram", "~terāmus";
    ind_act_plup_2: "~terās", "~terātis";
    ind_act_plup_3: "~terat", "~terant";
    ind_act_futp_1: "~terō", "~terimus";
    ind_act_futp_2: "~teris", "~teritis";
    ind_act_futp_3: "~terit", "~terint";
    subj_act_pres_1: "~tam", "~tāmus";
    subj_act_pres_2: "~tās", "~tātis";
    subj_act_pres_3: "~tat", "~tant";
    subj_act_impf_1: "~terem", "~terēmus";
    subj_act_impf_2: "~terēs", "~terētis";
    subj_act_impf_3: "~teret", "~terent";
    subj_act_perf_1: "~terim", "~terīmus";
    subj_act_perf_2: "~terīs", "~terītis";
    subj_act_plup_1: "~tissem", "~tissēmus";
    subj_act_plup_2: "~tissēs", "~tissētis";
    subj_act_plup_3: "~tisset", "~tissent";
    imperative_act_pres_2: "~te", "~tite";
    imperative_act_fut_2: "~titō", "~titōte";
    imperative_act_fut_3: _, "~tuntō";
    imperative_pass_pres_2: _, derived;
    infinitive_act: derived, "~tisse", _;
    ),
    // Contorqueō: 9 words (torqueō, extorqueō, dētorqueō, contorqueō, distorqueō, intorqueō). Like Agō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("contorqueō", 9, "~queō", like Agō;
    ind_act_pres_1: "~queō", "~quēmus";
    ind_act_pres_2: "~quēs", "~quētis";
    ind_act_pres_3: "~quet", "~quent";
    ind_act_impf_1: "~quēbam", "~quēbāmus";
    ind_act_impf_2: "~quēbās", "~quēbātis";
    ind_act_impf_3: "~quēbat", "~quēbant";
    ind_act_fut_1: "~quēbō", "~quēbimus";
    ind_act_fut_2: "~quēbis", "~quēbitis";
    ind_act_fut_3: "~quēbit", "~quēbunt";
    ind_pass_pres_1: "~queor", "~quēmur";
    ind_pass_pres_2: "~quēris|~quēre", "~quēminī";
    ind_pass_pres_3: "~quētur", "~quentur";
    ind_pass_impf_1: "~quēbar", "~quēbāmur";
    ind_pass_impf_2: "~quēbāris|~quēbāre", "~quēbāminī";
    ind_pass_impf_3: "~quēbātur", "~quēbantur";
    ind_pass_fut_1: "~quēbor", "~quēbimur";
    ind_pass_fut_2: "~quēberis|~quēbere", "~quēbiminī";
    ind_pass_fut_3: "~quēbitur", "~quēbuntur";
    subj_act_pres_1: "~queam", "~queāmus";
    subj_act_pres_2: "~queās", "~queātis";
    subj_act_pres_3: "~queat", "~queant";
    subj_act_impf_1: "~quērem", "~quērēmus";
    subj_act_impf_2: "~quērēs", "~quērētis";
    subj_act_impf_3: "~quēret", "~quērent";
    subj_pass_pres_1: "~quear", "~queāmur";
    subj_pass_pres_2: "~queāris|~queāre", "~queāminī";
    subj_pass_pres_3: "~queātur", "~queantur";
    subj_pass_impf_1: "~quērer", "~quērēmur";
    subj_pass_impf_2: "~quērēris|~quērēre", "~quērēminī";
    subj_pass_impf_3: "~quērētur", "~quērentur";
    imperative_act_pres_2: "~quē", "~quēte";
    imperative_act_fut_2: "~quētō", "~quētōte";
    imperative_act_fut_3: _, "~quentō";
    imperative_pass_pres_2: "~quēre", _;
    imperative_pass_fut_2: "~quētor", _;
    imperative_pass_fut_3: _, "~quentor";
    infinitive_pass: "~quērī", _, _;
    participle: "~quēns", _, _, "~quendus";
    gerund: _, "~quendī", "~quendō", "~quendum", _, _;
    ),
    // Micō: 9 words (personō, plicō, intonō, affricō, circumsonō, adfricō). Like Cubō, except in 0 cells: .
    conjugation!("micō", 9, "~ō", like Cubō;
    stem participle_perf_pass: "~āt- like agitō";
    ),
    // Inveniō: 9 words (inveniō, conveniō, circumveniō, contrāveniō, dēveniō, praeveniō). Like Ēveniō, except in 25 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("inveniō", 9, "~e~iō", like Ēveniō;
    stem participle_perf_pass: "~e~t- like agitō";
    ind_pass_pres_1: "~e~ior", "~e~īmur";
    ind_pass_pres_2: "~e~īris|~e~īre", "~e~īminī";
    ind_pass_impf_1: "~e~iēbar", "~e~iēbāmur";
    ind_pass_impf_2: "~e~iēbāris|~e~iēbāre", "~e~iēbāminī";
    ind_pass_fut_1: "~e~iar", "~e~iēmur";
    ind_pass_fut_2: "~e~iēris|~e~iēre", "~e~iēminī";
    subj_pass_pres_1: "~e~iar", "~e~iāmur";
    subj_pass_pres_2: "~e~iāris|~e~iāre", "~e~iāminī";
    subj_pass_impf_1: "~e~īrer", "~e~īrēmur";
    subj_pass_impf_2: "~e~īrēris|~e~īrēre", "~e~īrēminī";
    imperative_pass_pres_2: "~e~īre", _;
    imperative_pass_fut_2: "~e~ītor", _;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    participle: _, _, _, "~e~iendus|~e~iundus";
    ),
    // Sternō: 9 words (sternō, cōnsternō, asternō, prōsternō, substernō, dēsternō). Like Cernō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("sternō", 9, "~e~nō", like Cernō;
    stem participle_perf_pass: "~~āt- like agitō";
    ind_act_perf_1: "~~āvī", "~~āvimus";
    ind_act_perf_2: "~~āvistī", "~~āvistis";
    ind_act_perf_3: "~~āvit", "~~āvērunt|~~āvēre";
    ind_act_plup_1: "~~āveram", "~~āverāmus";
    ind_act_plup_2: "~~āverās", "~~āverātis";
    ind_act_plup_3: "~~āverat", "~~āverant";
    ind_act_futp_1: "~~āverō", "~~āverimus";
    ind_act_futp_2: "~~āveris", "~~āveritis";
    ind_act_futp_3: "~~āverit", "~~āverint";
    subj_act_perf_1: "~~āverim", "~~āverīmus";
    subj_act_perf_2: "~~āverīs", "~~āverītis";
    subj_act_plup_1: "~~āvissem", "~~āvissēmus";
    subj_act_plup_2: "~~āvissēs", "~~āvissētis";
    subj_act_plup_3: "~~āvisset", "~~āvissent";
    infinitive_act: _, "~~āvisse", _;
    ),
    // Vīsō: 9 words (revīsō, vīsō, invīsō, praevertō, intervīsō, convīsō). Like Abnuō, except in 0 cells: .
    conjugation!("vīsō", 9, "~ō", like Abnuō;
    stem participle_perf_pass: "~- like agitō";
    ),
    // Sentiō: 9 words (sentiō, cōnsentiō, dissentiō, praesentiō, assentiō, adsentiō).
    conjugation!("sentiō", 9, "~e~tiō";
    stem participle_perf_pass: "~ē~s- like agitō";
    ind_act_pres_1: "~e~tiō", "~e~tīmus";
    ind_act_pres_2: "~e~tīs", "~e~tītis";
    ind_act_pres_3: "~e~tit", "~e~tiunt";
    ind_act_impf_1: "~e~tiēbam", "~e~tiēbāmus";
    ind_act_impf_2: "~e~tiēbās", "~e~tiēbātis";
    ind_act_impf_3: "~e~tiēbat", "~e~tiēbant";
    ind_act_fut_1: "~e~tiam", "~e~tiēmus";
    ind_act_fut_2: "~e~tiēs", "~e~tiētis";
    ind_act_fut_3: "~e~tiet", "~e~tient";
    ind_act_perf_1: "~ē~sī", "~ē~simus";
    ind_act_perf_2: "~ē~sistī", "~ē~sistis";
    ind_act_perf_3: "~ē~sit", "~ē~sērunt|~ē~sēre";
    ind_act_plup_1: "~ē~seram", "~ē~serāmus";
    ind_act_plup_2: "~ē~serās", "~ē~serātis";
    ind_act_plup_3: "~ē~serat", "~ē~serant";
    ind_act_futp_1: "~ē~serō", "~ē~serimus";
    ind_act_futp_2: "~ē~seris", "~ē~seritis";
    ind_act_futp_3: "~ē~serit", "~ē~serint";
    ind_pass_pres_1: "~e~tior", "~e~tīmur";
    ind_pass_pres_2: "~e~tīris|~e~tīre", "~e~tīminī";
    ind_pass_pres_3: "~e~tītur", "~e~tiuntur";
    ind_pass_impf_1: "~e~tiēbar", "~e~tiēbāmur";
    ind_pass_impf_2: "~e~tiēbāris|~e~tiēbāre", "~e~tiēbāminī";
    ind_pass_impf_3: "~e~tiēbātur", "~e~tiēbantur";
    ind_pass_fut_1: "~e~tiar", "~e~tiēmur";
    ind_pass_fut_2: "~e~tiēris|~e~tiēre", "~e~tiēminī";
    ind_pass_fut_3: "~e~tiētur", "~e~tientur";
    subj_act_pres_1: "~e~tiam", "~e~tiāmus";
    subj_act_pres_2: "~e~tiās", "~e~tiātis";
    subj_act_pres_3: "~e~tiat", "~e~tiant";
    subj_act_impf_1: "~e~tīrem", "~e~tīrēmus";
    subj_act_impf_2: "~e~tīrēs", "~e~tīrētis";
    subj_act_impf_3: "~e~tīret", "~e~tīrent";
    subj_act_perf_1: "~ē~serim", "~ē~serīmus";
    subj_act_perf_2: "~ē~serīs", "~ē~serītis";
    subj_act_plup_1: "~ē~sissem", "~ē~sissēmus";
    subj_act_plup_2: "~ē~sissēs", "~ē~sissētis";
    subj_act_plup_3: "~ē~sisset", "~ē~sissent";
    subj_pass_pres_1: "~e~tiar", "~e~tiāmur";
    subj_pass_pres_2: "~e~tiāris|~e~tiāre", "~e~tiāminī";
    subj_pass_pres_3: "~e~tiātur", "~e~tiantur";
    subj_pass_impf_1: "~e~tīrer", "~e~tīrēmur";
    subj_pass_impf_2: "~e~tīrēris|~e~tīrēre", "~e~tīrēminī";
    subj_pass_impf_3: "~e~tīrētur", "~e~tīrentur";
    imperative_act_pres_2: "~e~tī", "~e~tīte";
    imperative_act_fut_2: "~e~tītō", "~e~tītōte";
    imperative_act_fut_3: _, "~e~tiuntō";
    imperative_pass_pres_2: "~e~tīre", _;
    imperative_pass_fut_2: "~e~tītor", _;
    imperative_pass_fut_3: _, "~e~tiuntor";
    infinitive_act: _, "~ē~sisse", _;
    infinitive_pass: "~e~tīrī", _, _;
    participle: "~e~tiēns", _, _, "~e~tiendus|~e~tiundus";
    gerund: _, "~e~tiendī", "~e~tiendō", "~e~tiendum", _, _;
    ),
    // Mētior: 9 words (admētior, mētior, dīmētior, remētior, commētior, conmētior). Like Cōnsequor, except in 19 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("mētior", 9, "~tior", like Cōnsequor;
    stem verb_act_sg: "~t- like mōlior";
    stem participle_perf_pass: "~ns- like modulor";
    ind_act_pres_1: _, "~tīmur";
    ind_act_impf_1: "~tiēbar", "~tiēbāmur";
    ind_act_impf_2: "~tiēbāris|~tiēbāre", "~tiēbāminī";
    ind_act_impf_3: "~tiēbātur", "~tiēbantur";
    ind_act_fut_1: "~tiar", "~tiēmur";
    ind_act_fut_2: "~tiēris|~tiēre", "~tiēminī";
    ind_act_fut_3: "~tiētur", _;
    subj_act_pres_3: "~tiātur", _;
    imperative_act_fut_2: "~tītor", _;
    participle: "~tiēns", _, _, "~tiendus|~tiundus";
    gerund: _, "~tiendī", "~tiendō", "~tiendum", _, _;
    ),
    // Īnsiliō: 9 words (exiliō, trānsiliō, dēsiliō, exsiliō, īnsiliō, subsiliō). Like Corripiō, except in 31 cells: ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_perf_3 pl, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ….
    conjugation!("īnsiliō", 9, "~i~iō", like Corripiō;
    stem participle_perf_pass: "~u~t- like agitō";
    ind_act_pres_1: _, "~i~īmus";
    ind_act_pres_2: "~i~īs", "~i~ītis";
    ind_act_perf_3: _, "~i~uērunt";
    ind_pass_pres_1: _, "~i~īmur";
    ind_pass_pres_2: "~i~īris", "~i~īminī";
    ind_pass_pres_3: "~i~ītur", _;
    ind_pass_impf_2: "~i~iēbāris", _;
    ind_pass_fut_2: "~i~iēris", _;
    subj_act_impf_1: "~i~īrem", "~i~īrēmus";
    subj_act_impf_2: "~i~īrēs", "~i~īrētis";
    subj_act_impf_3: "~i~īret", "~i~īrent";
    subj_pass_pres_2: "~i~iāris", _;
    subj_pass_impf_1: "~i~īrer", "~i~īrēmur";
    subj_pass_impf_2: "~i~īrēris", "~i~īrēminī";
    subj_pass_impf_3: "~i~īrētur", "~i~īrentur";
    imperative_act_pres_2: "~i~ī", "~i~īte";
    imperative_act_fut_2: "~i~ītō", "~i~ītōte";
    imperative_pass_pres_2: "~i~īre", _;
    imperative_pass_fut_2: "~i~ītor", _;
    infinitive_pass: "~i~īrī", _, _;
    participle: _, _, _, "~i~iendus";
    ),
    // Interstinguō: 9 words (distinguō, stinguō, extinguō, exstinguō, restinguō, intinguō). Like Tingō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("interstinguō", 9, "~i~guō", like Tingō;
    ind_act_pres_1: "~i~guō", "~i~guimus";
    ind_act_pres_2: "~i~guis", "~i~guitis";
    ind_act_pres_3: "~i~guit", "~i~guunt";
    ind_act_impf_1: "~i~guēbam", "~i~guēbāmus";
    ind_act_impf_2: "~i~guēbās", "~i~guēbātis";
    ind_act_impf_3: "~i~guēbat", "~i~guēbant";
    ind_act_fut_1: "~i~guam", "~i~guēmus";
    ind_act_fut_2: "~i~guēs", "~i~guētis";
    ind_act_fut_3: "~i~guet", "~i~guent";
    ind_pass_pres_1: "~i~guor", "~i~guimur";
    ind_pass_pres_2: "~i~gueris|~i~guere", "~i~guiminī";
    ind_pass_pres_3: "~i~guitur", "~i~guuntur";
    ind_pass_impf_1: "~i~guēbar", "~i~guēbāmur";
    ind_pass_impf_2: "~i~guēbāris|~i~guēbāre", "~i~guēbāminī";
    ind_pass_impf_3: "~i~guēbātur", "~i~guēbantur";
    ind_pass_fut_1: "~i~guar", "~i~guēmur";
    ind_pass_fut_2: "~i~guēris|~i~guēre", "~i~guēminī";
    ind_pass_fut_3: "~i~guētur", "~i~guentur";
    subj_act_pres_1: "~i~guam", "~i~guāmus";
    subj_act_pres_2: "~i~guās", "~i~guātis";
    subj_act_pres_3: "~i~guat", "~i~guant";
    subj_act_impf_1: "~i~guerem", "~i~guerēmus";
    subj_act_impf_2: "~i~guerēs", "~i~guerētis";
    subj_act_impf_3: "~i~gueret", "~i~guerent";
    subj_pass_pres_1: "~i~guar", "~i~guāmur";
    subj_pass_pres_2: "~i~guāris|~i~guāre", "~i~guāminī";
    subj_pass_pres_3: "~i~guātur", "~i~guantur";
    subj_pass_impf_1: "~i~guerer", "~i~guerēmur";
    subj_pass_impf_2: "~i~guerēris|~i~guerēre", "~i~guerēminī";
    subj_pass_impf_3: "~i~guerētur", "~i~guerentur";
    imperative_act_pres_2: "~i~gue", "~i~guite";
    imperative_act_fut_2: "~i~guitō", "~i~guitōte";
    imperative_act_fut_3: _, "~i~guuntō";
    imperative_pass_pres_2: "~i~guere", _;
    imperative_pass_fut_2: "~i~guitor", _;
    imperative_pass_fut_3: _, "~i~guuntor";
    infinitive_pass: "~i~guī", _, _;
    participle: "~i~guēns", _, _, "~i~guendus";
    gerund: _, "~i~guendī", "~i~guendō", "~i~guendum", _, _;
    ),
    // Pervincō: 8 words (vincō, convincō, ēvincō, dēvincō, prōvincō, revincō).
    conjugation!("pervincō", 8, "~in~ō";
    stem participle_perf_pass: "~i~t- like agitō";
    ind_act_pres_1: "~in~ō", "~in~imus";
    ind_act_pres_2: "~in~is", "~in~itis";
    ind_act_pres_3: "~in~it", "~in~unt";
    ind_act_impf_1: "~in~ēbam", "~in~ēbāmus";
    ind_act_impf_2: "~in~ēbās", "~in~ēbātis";
    ind_act_impf_3: "~in~ēbat", "~in~ēbant";
    ind_act_fut_1: "~in~am", "~in~ēmus";
    ind_act_fut_2: "~in~ēs", "~in~ētis";
    ind_act_fut_3: "~in~et", "~in~ent";
    ind_act_perf_1: "~ī~ī", "~ī~imus";
    ind_act_perf_2: "~ī~istī", "~ī~istis";
    ind_act_perf_3: "~ī~it", "~ī~ērunt|~ī~ēre";
    ind_act_plup_1: "~ī~eram", "~ī~erāmus";
    ind_act_plup_2: "~ī~erās", "~ī~erātis";
    ind_act_plup_3: "~ī~erat", "~ī~erant";
    ind_act_futp_1: "~ī~erō", "~ī~erimus";
    ind_act_futp_2: "~ī~eris", "~ī~eritis";
    ind_act_futp_3: "~ī~erit", "~ī~erint";
    ind_pass_pres_1: "~in~or", "~in~imur";
    ind_pass_pres_2: "~in~eris|~in~ere", "~in~iminī";
    ind_pass_pres_3: "~in~itur", "~in~untur";
    ind_pass_impf_1: "~in~ēbar", "~in~ēbāmur";
    ind_pass_impf_2: "~in~ēbāris|~in~ēbāre", "~in~ēbāminī";
    ind_pass_impf_3: "~in~ēbātur", "~in~ēbantur";
    ind_pass_fut_1: "~in~ar", "~in~ēmur";
    ind_pass_fut_2: "~in~ēris|~in~ēre", "~in~ēminī";
    ind_pass_fut_3: "~in~ētur", "~in~entur";
    subj_act_pres_1: "~in~am", "~in~āmus";
    subj_act_pres_2: "~in~ās", "~in~ātis";
    subj_act_pres_3: "~in~at", "~in~ant";
    subj_act_impf_1: "~in~erem", "~in~erēmus";
    subj_act_impf_2: "~in~erēs", "~in~erētis";
    subj_act_impf_3: "~in~eret", "~in~erent";
    subj_act_perf_1: "~ī~erim", "~ī~erīmus";
    subj_act_perf_2: "~ī~erīs", "~ī~erītis";
    subj_act_plup_1: "~ī~issem", "~ī~issēmus";
    subj_act_plup_2: "~ī~issēs", "~ī~issētis";
    subj_act_plup_3: "~ī~isset", "~ī~issent";
    subj_pass_pres_1: "~in~ar", "~in~āmur";
    subj_pass_pres_2: "~in~āris|~in~āre", "~in~āminī";
    subj_pass_pres_3: "~in~ātur", "~in~antur";
    subj_pass_impf_1: "~in~erer", "~in~erēmur";
    subj_pass_impf_2: "~in~erēris|~in~erēre", "~in~erēminī";
    subj_pass_impf_3: "~in~erētur", "~in~erentur";
    imperative_act_pres_2: "~in~e", "~in~ite";
    imperative_act_fut_2: "~in~itō", "~in~itōte";
    imperative_act_fut_3: _, "~in~untō";
    imperative_pass_pres_2: "~in~ere", _;
    imperative_pass_fut_2: "~in~itor", _;
    imperative_pass_fut_3: _, "~in~untor";
    infinitive_act: _, "~ī~isse", _;
    infinitive_pass: "~in~ī", _, _;
    participle: "~in~ēns", _, _, "~in~endus|~in~undus";
    gerund: _, "~in~endī", "~in~endō", "~in~endum", _, _;
    ),
    // Concoquō: 8 words (coquō, concoquō, discoquō, excoquō, dēcoquō, recoquō). Like Cōnsequor, except in 66 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("concoquō", 8, "~quō", like Cōnsequor as PASS_AS_ACT;
    stem participle_perf_pass: "~ct- like agitō";
    ind_act_pres_1: "~quō", "~quimus";
    ind_act_pres_2: "~quis", "~quitis";
    ind_act_pres_3: "~quit", "~quunt";
    ind_act_impf_1: "~quēbam", "~quēbāmus";
    ind_act_impf_2: "~quēbās", "~quēbātis";
    ind_act_impf_3: "~quēbat", "~quēbant";
    ind_act_fut_1: "~quam", "~quēmus";
    ind_act_fut_2: "~quēs", "~quētis";
    ind_act_fut_3: "~quet", "~quent";
    ind_act_perf_1: "~xī", "~ximus";
    ind_act_perf_2: "~xistī", "~xistis";
    ind_act_perf_3: "~xit", "~xērunt|~xēre";
    ind_act_plup_1: "~xeram", "~xerāmus";
    ind_act_plup_2: "~xerās", "~xerātis";
    ind_act_plup_3: "~xerat", "~xerant";
    ind_act_futp_1: "~xerō", "~xerimus";
    ind_act_futp_2: "~xeris", "~xeritis";
    ind_act_futp_3: "~xerit", "~xerint";
    subj_act_pres_1: "~quam", "~quāmus";
    subj_act_pres_2: "~quās", "~quātis";
    subj_act_pres_3: "~quat", "~quant";
    subj_act_impf_1: "~querem", "~querēmus";
    subj_act_impf_2: "~querēs", "~querētis";
    subj_act_impf_3: "~queret", "~querent";
    subj_act_perf_1: "~xerim", "~xerīmus";
    subj_act_perf_2: "~xerīs", "~xerītis";
    subj_act_plup_1: "~xissem", "~xissēmus";
    subj_act_plup_2: "~xissēs", "~xissētis";
    subj_act_plup_3: "~xisset", "~xissent";
    imperative_act_pres_2: "~que", "~quite";
    imperative_act_fut_2: "~quitō", "~quitōte";
    imperative_act_fut_3: _, "~quuntō";
    imperative_pass_pres_2: _, derived;
    infinitive_act: derived, "~xisse", _;
    ),
    // Nūbō: 8 words (nūbō, prōnūbō, glūbō, ēnūbō, dēglūbō, obnūbō).
    conjugation!("nūbō", 8, "~ūbō";
    stem participle_perf_pass: "~ŭpt- like agitō";
    ind_act_pres_1: "~ūbō", "~ūbimus";
    ind_act_pres_2: "~ūbis", "~ūbitis";
    ind_act_pres_3: "~ūbit", "~ūbunt";
    ind_act_impf_1: "~ūbēbam", "~ūbēbāmus";
    ind_act_impf_2: "~ūbēbās", "~ūbēbātis";
    ind_act_impf_3: "~ūbēbat", "~ūbēbant";
    ind_act_fut_1: "~ūbam", "~ūbēmus";
    ind_act_fut_2: "~ūbēs", "~ūbētis";
    ind_act_fut_3: "~ūbet", "~ūbent";
    ind_act_perf_1: "~ūpsī", "~ūpsimus";
    ind_act_perf_2: "~ūpsistī", "~ūpsistis";
    ind_act_perf_3: "~ūpsit", "~ūpsērunt|~ūpsēre";
    ind_act_plup_1: "~ūpseram", "~ūpserāmus";
    ind_act_plup_2: "~ūpserās", "~ūpserātis";
    ind_act_plup_3: "~ūpserat", "~ūpserant";
    ind_act_futp_1: "~ūpserō", "~ūpserimus";
    ind_act_futp_2: "~ūpseris", "~ūpseritis";
    ind_act_futp_3: "~ūpserit", "~ūpserint";
    ind_pass_pres_1: "~ūbor", "~ūbimur";
    ind_pass_pres_2: "~ūberis|~ūbere", "~ūbiminī";
    ind_pass_pres_3: "~ūbitur", "~ūbuntur";
    ind_pass_impf_1: "~ūbēbar", "~ūbēbāmur";
    ind_pass_impf_2: "~ūbēbāris|~ūbēbāre", "~ūbēbāminī";
    ind_pass_impf_3: "~ūbēbātur", "~ūbēbantur";
    ind_pass_fut_1: "~ūbar", "~ūbēmur";
    ind_pass_fut_2: "~ūbēris|~ūbēre", "~ūbēminī";
    ind_pass_fut_3: "~ūbētur", "~ūbentur";
    subj_act_pres_1: "~ūbam", "~ūbāmus";
    subj_act_pres_2: "~ūbās", "~ūbātis";
    subj_act_pres_3: "~ūbat", "~ūbant";
    subj_act_impf_1: "~ūberem", "~ūberēmus";
    subj_act_impf_2: "~ūberēs", "~ūberētis";
    subj_act_impf_3: "~ūberet", "~ūberent";
    subj_act_perf_1: "~ūpserim", "~ūpserīmus";
    subj_act_perf_2: "~ūpserīs", "~ūpserītis";
    subj_act_plup_1: "~ūpsissem", "~ūpsissēmus";
    subj_act_plup_2: "~ūpsissēs", "~ūpsissētis";
    subj_act_plup_3: "~ūpsisset", "~ūpsissent";
    subj_pass_pres_1: "~ūbar", "~ūbāmur";
    subj_pass_pres_2: "~ūbāris|~ūbāre", "~ūbāminī";
    subj_pass_pres_3: "~ūbātur", "~ūbantur";
    subj_pass_impf_1: "~ūberer", "~ūberēmur";
    subj_pass_impf_2: "~ūberēris|~ūberēre", "~ūberēminī";
    subj_pass_impf_3: "~ūberētur", "~ūberentur";
    imperative_act_pres_2: "~ūbe", "~ūbite";
    imperative_act_fut_2: "~ūbitō", "~ūbitōte";
    imperative_act_fut_3: _, "~ūbuntō";
    imperative_pass_pres_2: "~ūbere", _;
    imperative_pass_fut_2: "~ūbitor", _;
    imperative_pass_fut_3: _, "~ūbuntor";
    infinitive_act: _, "~ūpsisse", _;
    infinitive_pass: "~ūbī", _, _;
    participle: "~ūbēns", _, _, "~ūbendus|~ūbundus";
    gerund: _, "~ūbendī", "~ūbendō", "~ūbendum", _, _;
    ),
    // Prōfluō: 8 words (superfluō, circumfluō, cōnfluō, diffluō, interfluō, prōfluō). Like Struō, except in 5 cells: ind_pass_pres_2 sg, ind_pass_impf_2 sg, ind_pass_fut_2 sg, subj_pass_pres_2 sg, subj_pass_impf_2 sg.
    conjugation!("prōfluō", 8, "~uō", like Struō;
    stem participle_perf_pass: "~ūx- like agitō";
    ind_pass_pres_2: "~ueris", _;
    ind_pass_impf_2: "~uēbāris", _;
    ind_pass_fut_2: "~uēris", _;
    subj_pass_pres_2: "~uāris", _;
    subj_pass_impf_2: "~uerēris", _;
    ),
    // Exorior: 8 words (exorior, commorior, conmorior, intermorior, dēmorior, praemorior). Like Mōlior, except in 5 cells: ind_act_pres_1 pl, imperative_act_fut_2 sg, participle fut_act, supine acc, supine abl.
    conjugation!("exorior", 8, "~ior", like Mōlior;
    stem verb_act_sg: "~- like ēgredior";
    ind_act_pres_1: _, "~imur";
    imperative_act_fut_2: "~itor", _;
    participle: _, _, "~itūrus", _;
    supine: _, _, _, "~tuum", "~tuū", _;
    ),
    // Cohaereō: 8 words (adhaereō, haereō, adhēreō, cohaereō, inhaereō, cohēreō). Like Ēvādō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("cohaereō", 8, "~reō", like Ēvādō;
    ind_act_pres_1: "~reō", "~rēmus";
    ind_act_pres_2: "~rēs", "~rētis";
    ind_act_pres_3: "~ret", "~rent";
    ind_act_impf_1: "~rēbam", "~rēbāmus";
    ind_act_impf_2: "~rēbās", "~rēbātis";
    ind_act_impf_3: "~rēbat", "~rēbant";
    ind_act_fut_1: "~rēbō", "~rēbimus";
    ind_act_fut_2: "~rēbis", "~rēbitis";
    ind_act_fut_3: "~rēbit", "~rēbunt";
    ind_pass_pres_1: "~reor", "~rēmur";
    ind_pass_pres_2: "~rēris", "~rēminī";
    ind_pass_pres_3: "~rētur", "~rentur";
    ind_pass_impf_1: "~rēbar", "~rēbāmur";
    ind_pass_impf_2: "~rēbāris", "~rēbāminī";
    ind_pass_impf_3: "~rēbātur", "~rēbantur";
    ind_pass_fut_1: "~rēbor", "~rēbimur";
    ind_pass_fut_2: "~rēberis", "~rēbiminī";
    ind_pass_fut_3: "~rēbitur", "~rēbuntur";
    subj_act_pres_1: "~ream", "~reāmus";
    subj_act_pres_2: "~reās", "~reātis";
    subj_act_pres_3: "~reat", "~reant";
    subj_act_impf_1: "~rērem", "~rērēmus";
    subj_act_impf_2: "~rērēs", "~rērētis";
    subj_act_impf_3: "~rēret", "~rērent";
    subj_pass_pres_1: "~rear", "~reāmur";
    subj_pass_pres_2: "~reāris", "~reāminī";
    subj_pass_pres_3: "~reātur", "~reantur";
    subj_pass_impf_1: "~rērer", "~rērēmur";
    subj_pass_impf_2: "~rērēris", "~rērēminī";
    subj_pass_impf_3: "~rērētur", "~rērentur";
    imperative_act_pres_2: "~rē", "~rēte";
    imperative_act_fut_2: "~rētō", "~rētōte";
    imperative_act_fut_3: _, "~rentō";
    imperative_pass_pres_2: "~rēre", _;
    imperative_pass_fut_2: "~rētor", _;
    imperative_pass_fut_3: _, "~rentor";
    infinitive_pass: "~rērī", _, _;
    participle: "~rēns", _, _, "~rendus";
    gerund: _, "~rendī", "~rendō", "~rendum", _, _;
    ),
    // Recolō: 7 words (colō, incolō, recolō, excolō, circumcolō, accolō).
    conjugation!("recolō", 7, "~o~ō";
    stem participle_perf_pass: "~u~t- like agitō";
    ind_act_pres_1: "~o~ō", "~o~imus";
    ind_act_pres_2: "~o~is", "~o~itis";
    ind_act_pres_3: "~o~it", "~o~unt";
    ind_act_impf_1: "~o~ēbam", "~o~ēbāmus";
    ind_act_impf_2: "~o~ēbās", "~o~ēbātis";
    ind_act_impf_3: "~o~ēbat", "~o~ēbant";
    ind_act_fut_1: "~o~am", "~o~ēmus";
    ind_act_fut_2: "~o~ēs", "~o~ētis";
    ind_act_fut_3: "~o~et", "~o~ent";
    ind_act_perf_1: "~o~uī", "~o~uimus";
    ind_act_perf_2: "~o~uistī", "~o~uistis";
    ind_act_perf_3: "~o~uit", "~o~uērunt|~o~uēre";
    ind_act_plup_1: "~o~ueram", "~o~uerāmus";
    ind_act_plup_2: "~o~uerās", "~o~uerātis";
    ind_act_plup_3: "~o~uerat", "~o~uerant";
    ind_act_futp_1: "~o~uerō", "~o~uerimus";
    ind_act_futp_2: "~o~ueris", "~o~ueritis";
    ind_act_futp_3: "~o~uerit", "~o~uerint";
    ind_pass_pres_1: "~o~or", "~o~imur";
    ind_pass_pres_2: "~o~eris|~o~ere", "~o~iminī";
    ind_pass_pres_3: "~o~itur", "~o~untur";
    ind_pass_impf_1: "~o~ēbar", "~o~ēbāmur";
    ind_pass_impf_2: "~o~ēbāris|~o~ēbāre", "~o~ēbāminī";
    ind_pass_impf_3: "~o~ēbātur", "~o~ēbantur";
    ind_pass_fut_1: "~o~ar", "~o~ēmur";
    ind_pass_fut_2: "~o~ēris|~o~ēre", "~o~ēminī";
    ind_pass_fut_3: "~o~ētur", "~o~entur";
    subj_act_pres_1: "~o~am", "~o~āmus";
    subj_act_pres_2: "~o~ās", "~o~ātis";
    subj_act_pres_3: "~o~at", "~o~ant";
    subj_act_impf_1: "~o~erem", "~o~erēmus";
    subj_act_impf_2: "~o~erēs", "~o~erētis";
    subj_act_impf_3: "~o~eret", "~o~erent";
    subj_act_perf_1: "~o~uerim", "~o~uerīmus";
    subj_act_perf_2: "~o~uerīs", "~o~uerītis";
    subj_act_plup_1: "~o~uissem", "~o~uissēmus";
    subj_act_plup_2: "~o~uissēs", "~o~uissētis";
    subj_act_plup_3: "~o~uisset", "~o~uissent";
    subj_pass_pres_1: "~o~ar", "~o~āmur";
    subj_pass_pres_2: "~o~āris|~o~āre", "~o~āminī";
    subj_pass_pres_3: "~o~ātur", "~o~antur";
    subj_pass_impf_1: "~o~erer", "~o~erēmur";
    subj_pass_impf_2: "~o~erēris|~o~erēre", "~o~erēminī";
    subj_pass_impf_3: "~o~erētur", "~o~erentur";
    imperative_act_pres_2: "~o~e", "~o~ite";
    imperative_act_fut_2: "~o~itō", "~o~itōte";
    imperative_act_fut_3: _, "~o~untō";
    imperative_pass_pres_2: "~o~ere", _;
    imperative_pass_fut_2: "~o~itor", _;
    imperative_pass_fut_3: _, "~o~untor";
    infinitive_act: _, "~o~uisse", _;
    infinitive_pass: "~o~ī", _, _;
    participle: "~o~ēns", _, _, "~o~endus|~o~undus";
    gerund: _, "~o~endī", "~o~endō", "~o~endum", _, _;
    ),
    // Gaudeō: 7 words (gaudeō, aggaudeō, adgaudeō, congaudeō, pergaudeō, praegaudeō). Like Cōnsequor, except in 19 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("gaudeō", 7, "~audeō", like Cōnsequor;
    stem verb_act_sg: "~aud- like gaudeō";
    stem participle_perf_pass: "~āvīs- like modulor";
    ind_act_pres_1: _, "~audēmus";
    ind_act_impf_1: "~audēbam", "~audēbāmus";
    ind_act_impf_2: "~audēbās", "~audēbātis";
    ind_act_impf_3: "~audēbat", "~audēbant";
    ind_act_fut_1: "~audēbō", "~audēbimus";
    ind_act_fut_2: "~audēbis", "~audēbitis";
    ind_act_fut_3: "~audēbit", _;
    subj_act_pres_3: "~audeat", _;
    imperative_act_fut_2: "~audētō", _;
    participle: "~audēns", _, _, "~audendus";
    gerund: _, "~audendī", "~audendō", "~audendum", _, _;
    ),
    // Cognōscō: 7 words (conōscō, cognōscō, recognōscō, agnōscō, praecognōscō, adgnōscō).
    conjugation!("cognōscō", 7, "~ōscō";
    stem participle_perf_pass: "~it- like agitō";
    ind_act_pres_1: "~ōscō", "~ōscimus";
    ind_act_pres_2: "~ōscis", "~ōscitis";
    ind_act_pres_3: "~ōscit", "~ōscunt";
    ind_act_impf_1: "~ōscēbam", "~ōscēbāmus";
    ind_act_impf_2: "~ōscēbās", "~ōscēbātis";
    ind_act_impf_3: "~ōscēbat", "~ōscēbant";
    ind_act_fut_1: "~ōscam", "~ōscēmus";
    ind_act_fut_2: "~ōscēs", "~ōscētis";
    ind_act_fut_3: "~ōscet", "~ōscent";
    ind_act_perf_1: "~ōvī", "~ōvimus|~ōmus";
    ind_act_perf_2: "~ōvistī|~ōstī", "~ōvistis|~ōstis";
    ind_act_perf_3: "~ōvit|~ōt", "~ōvērunt|~ōrunt|~ōvēre";
    ind_act_plup_1: "~ōveram|~ōram", "~ōverāmus|~ōrāmus";
    ind_act_plup_2: "~ōverās|~ōrās", "~ōverātis|~ōrātis";
    ind_act_plup_3: "~ōverat|~ōrat", "~ōverant|~ōrant";
    ind_act_futp_1: "~ōverō|~ōrō", "~ōverimus|~ōrimus";
    ind_act_futp_2: "~ōveris|~ōris", "~ōveritis|~ōritis";
    ind_act_futp_3: "~ōverit|~ōrit", "~ōverint|~ōrint";
    ind_pass_pres_1: "~ōscor", "~ōscimur";
    ind_pass_pres_2: "~ōsceris|~ōscere", "~ōsciminī";
    ind_pass_pres_3: "~ōscitur", "~ōscuntur";
    ind_pass_impf_1: "~ōscēbar", "~ōscēbāmur";
    ind_pass_impf_2: "~ōscēbāris|~ōscēbāre", "~ōscēbāminī";
    ind_pass_impf_3: "~ōscēbātur", "~ōscēbantur";
    ind_pass_fut_1: "~ōscar", "~ōscēmur";
    ind_pass_fut_2: "~ōscēris|~ōscēre", "~ōscēminī";
    ind_pass_fut_3: "~ōscētur", "~ōscentur";
    subj_act_pres_1: "~ōscam", "~ōscāmus";
    subj_act_pres_2: "~ōscās", "~ōscātis";
    subj_act_pres_3: "~ōscat", "~ōscant";
    subj_act_impf_1: "~ōscerem", "~ōscerēmus";
    subj_act_impf_2: "~ōscerēs", "~ōscerētis";
    subj_act_impf_3: "~ōsceret", "~ōscerent";
    subj_act_perf_1: "~ōverim|~ōrim", "~ōverīmus|~ōrīmus";
    subj_act_perf_2: "~ōverīs|~ōrīs", "~ōverītis|~ōrītis";
    subj_act_plup_1: "~ōvissem|~ōssem", "~ōvissēmus|~ōssēmus";
    subj_act_plup_2: "~ōvissēs|~ōssēs", "~ōvissētis|~ōssētis";
    subj_act_plup_3: "~ōvisset|~ōsset", "~ōvissent|~ōssent";
    subj_pass_pres_1: "~ōscar", "~ōscāmur";
    subj_pass_pres_2: "~ōscāris|~ōscāre", "~ōscāminī";
    subj_pass_pres_3: "~ōscātur", "~ōscantur";
    subj_pass_impf_1: "~ōscerer", "~ōscerēmur";
    subj_pass_impf_2: "~ōscerēris|~ōscerēre", "~ōscerēminī";
    subj_pass_impf_3: "~ōscerētur", "~ōscerentur";
    imperative_act_pres_2: "~ōsce", "~ōscite";
    imperative_act_fut_2: "~ōscitō", "~ōscitōte";
    imperative_act_fut_3: _, "~ōscuntō";
    imperative_pass_pres_2: "~ōscere", _;
    imperative_pass_fut_2: "~ōscitor", _;
    imperative_pass_fut_3: _, "~ōscuntor";
    infinitive_act: _, "~ōvisse|~ōsse", _;
    infinitive_pass: "~ōscī", _, _;
    participle: "~ōscēns", _, _, "~ōscendus|~ōscundus";
    gerund: _, "~ōscendī", "~ōscendō", "~ōscendum", _, _;
    ),
    // Comedō: 7 words (comedō, ambedō, adedō, exedō, peredō, subedō). Like Possideō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("comedō", 7, "~edō", like Possideō;
    stem participle_perf_pass: "~ēs- like comedō";
    ind_act_pres_1: "~edō", "~edimus";
    ind_act_pres_2: "~edis|~ēs", "~editis|~ēstis";
    ind_act_pres_3: "~edit|~ēst", "~edunt";
    ind_act_impf_1: "~edēbam", "~edēbāmus";
    ind_act_impf_2: "~edēbās", "~edēbātis";
    ind_act_impf_3: "~edēbat", "~edēbant";
    ind_act_fut_1: "~edam", "~edēmus";
    ind_act_fut_2: "~edēs", "~edētis";
    ind_act_fut_3: "~edet", "~edent";
    ind_pass_pres_1: "~edor", "~edimur";
    ind_pass_pres_2: "~ederis|~edere|~ēssis|~ēsse", "~ediminī";
    ind_pass_pres_3: "~editur|~ēstur", "~eduntur";
    ind_pass_impf_1: "~edēbar", "~edēbāmur";
    ind_pass_impf_2: "~edēbāris|~edēbāre", "~edēbāminī";
    ind_pass_impf_3: "~edēbātur", "~edēbantur";
    ind_pass_fut_1: "~edar", "~edēmur";
    ind_pass_fut_2: "~edēris|~edēre", "~edēminī";
    ind_pass_fut_3: "~edētur", "~edentur";
    subj_act_pres_1: "~edam|~edim", "~edāmus|~edīmus";
    subj_act_pres_2: "~edās|~edīs", "~edātis|~edītis";
    subj_act_pres_3: "~edat|~edit|~ēst", "~edant|~edint";
    subj_act_impf_1: "~ederem|~ēssem", "~ederēmus|~ēssēmus";
    subj_act_impf_2: "~ederēs|~ēssēs", "~ederētis|~ēssētis";
    subj_act_impf_3: "~ederet|~ēsset", "~ederent|~ēssent";
    subj_pass_pres_1: "~edar", "~edāmur";
    subj_pass_pres_2: "~edāris|~edāre", "~edāminī";
    subj_pass_pres_3: "~edātur", "~edantur";
    subj_pass_impf_1: "~ederer|~ēsser", "~ederēmur|~ēssēmur";
    subj_pass_impf_2: "~ederēris|~ederēre|~ēssēris|~ēssēre", "~ederēminī|~ēssēminī";
    subj_pass_impf_3: "~ederētur|~ēssētur", "~ederentur|~ēssentur";
    imperative_act_pres_2: "~ede|~ēs", "~edite|~ēste";
    imperative_act_fut_2: "~editō|~ēstō", "~editōte|~ēstōte";
    imperative_act_fut_3: _, "~eduntō";
    imperative_pass_pres_2: "~edere|~ēsse", _;
    imperative_pass_fut_2: "~editor|~ēstor", _;
    imperative_pass_fut_3: _, "~eduntor";
    infinitive_pass: "~edī", _, _;
    participle: "~edēns", _, _, "~edendus|~edundus";
    gerund: _, "~edendī", "~edendō", "~edendum", _, _;
    ),
    // Maneō: 7 words (remaneō, maneō, permaneō, commaneō, ēmaneō, immaneō).
    conjugation!("maneō", 7, "~a~eō";
    stem participle_perf_pass: "~ā~s- like agitō";
    ind_act_pres_1: "~a~eō", "~a~ēmus";
    ind_act_pres_2: "~a~ēs", "~a~ētis";
    ind_act_pres_3: "~a~et", "~a~ent";
    ind_act_impf_1: "~a~ēbam", "~a~ēbāmus";
    ind_act_impf_2: "~a~ēbās", "~a~ēbātis";
    ind_act_impf_3: "~a~ēbat", "~a~ēbant";
    ind_act_fut_1: "~a~ēbō", "~a~ēbimus";
    ind_act_fut_2: "~a~ēbis", "~a~ēbitis";
    ind_act_fut_3: "~a~ēbit", "~a~ēbunt";
    ind_act_perf_1: "~ā~sī", "~ā~simus";
    ind_act_perf_2: "~ā~sistī", "~ā~sistis";
    ind_act_perf_3: "~ā~sit", "~ā~sērunt|~ā~sēre";
    ind_act_plup_1: "~ā~seram", "~ā~serāmus";
    ind_act_plup_2: "~ā~serās", "~ā~serātis";
    ind_act_plup_3: "~ā~serat", "~ā~serant";
    ind_act_futp_1: "~ā~serō", "~ā~serimus";
    ind_act_futp_2: "~ā~seris", "~ā~seritis";
    ind_act_futp_3: "~ā~serit", "~ā~serint";
    ind_pass_pres_1: "~a~eor", "~a~ēmur";
    ind_pass_pres_2: "~a~ēris", "~a~ēminī";
    ind_pass_pres_3: "~a~ētur", "~a~entur";
    ind_pass_impf_1: "~a~ēbar", "~a~ēbāmur";
    ind_pass_impf_2: "~a~ēbāris", "~a~ēbāminī";
    ind_pass_impf_3: "~a~ēbātur", "~a~ēbantur";
    ind_pass_fut_1: "~a~ēbor", "~a~ēbimur";
    ind_pass_fut_2: "~a~ēberis", "~a~ēbiminī";
    ind_pass_fut_3: "~a~ēbitur", "~a~ēbuntur";
    subj_act_pres_1: "~a~eam", "~a~eāmus";
    subj_act_pres_2: "~a~eās", "~a~eātis";
    subj_act_pres_3: "~a~eat", "~a~eant";
    subj_act_impf_1: "~a~ērem", "~a~ērēmus";
    subj_act_impf_2: "~a~ērēs", "~a~ērētis";
    subj_act_impf_3: "~a~ēret", "~a~ērent";
    subj_act_perf_1: "~ā~serim", "~ā~serīmus";
    subj_act_perf_2: "~ā~serīs", "~ā~serītis";
    subj_act_plup_1: "~ā~sissem", "~ā~sissēmus";
    subj_act_plup_2: "~ā~sissēs", "~ā~sissētis";
    subj_act_plup_3: "~ā~sisset", "~ā~sissent";
    subj_pass_pres_1: "~a~ear", "~a~eāmur";
    subj_pass_pres_2: "~a~eāris", "~a~eāminī";
    subj_pass_pres_3: "~a~eātur", "~a~eantur";
    subj_pass_impf_1: "~a~ērer", "~a~ērēmur";
    subj_pass_impf_2: "~a~ērēris", "~a~ērēminī";
    subj_pass_impf_3: "~a~ērētur", "~a~ērentur";
    imperative_act_pres_2: "~a~ē", "~a~ēte";
    imperative_act_fut_2: "~a~ētō", "~a~ētōte";
    imperative_act_fut_3: _, "~a~entō";
    imperative_pass_pres_2: "~a~ēre", _;
    imperative_pass_fut_2: "~a~ētor", _;
    imperative_pass_fut_3: _, "~a~entor";
    infinitive_act: _, "~ā~sisse", _;
    infinitive_pass: "~a~ērī", _, _;
    participle: "~a~ēns", _, _, "~a~endus";
    gerund: _, "~a~endī", "~a~endō", "~a~endum", _, _;
    ),
    // Īnsistō: 7 words (cōnsistō, dēsistō, resistō, īnsistō, exsistō, absistō).
    conjugation!("īnsistō", 7, "~~s~ō";
    stem verb_act_sg: "~~s~- like īnsistō";
    stem participle_perf_pass: "~t~~- like labōrō";
    ind_act_pres_1: _, "~~s~imus";
    ind_act_impf_1: "~~s~ēbam", "~~s~ēbāmus";
    ind_act_impf_2: "~~s~ēbās", "~~s~ēbātis";
    ind_act_impf_3: "~~s~ēbat", "~~s~ēbant";
    ind_act_fut_1: "~~s~am", "~~s~ēmus";
    ind_act_fut_2: "~~s~ēs", "~~s~ētis";
    ind_act_fut_3: "~~s~et", _;
    ind_act_perf_1: "~t~~ī", "~t~~imus";
    ind_act_perf_2: "~t~~istī", "~t~~istis";
    ind_act_perf_3: "~t~~it", "~t~~ērunt|~t~~ēre";
    ind_act_plup_1: "~t~~eram", "~t~~erāmus";
    ind_act_plup_2: "~t~~erās", "~t~~erātis";
    ind_act_plup_3: "~t~~erat", "~t~~erant";
    ind_act_futp_1: "~t~~erō", "~t~~erimus";
    ind_act_futp_2: "~t~~eris", "~t~~eritis";
    ind_act_futp_3: "~t~~erit", "~t~~erint";
    subj_act_pres_3: "~~s~at", _;
    subj_act_perf_1: "~t~~erim", "~t~~erīmus";
    subj_act_perf_2: "~t~~erīs", "~t~~erītis";
    subj_act_plup_1: "~t~~issem", "~t~~issēmus";
    subj_act_plup_2: "~t~~issēs", "~t~~issētis";
    subj_act_plup_3: "~t~~isset", "~t~~issent";
    imperative_act_fut_2: "~~s~itō", _;
    infinitive_act: _, "~t~~isse", _;
    participle: "~~s~ēns", _, _, "~~s~endum|~~s~undum";
    gerund: _, "~~s~endī", "~~s~endō", "~~s~endum", _, _;
    ),
    // Augeō: 7 words (augeō, lūgeō, adaugeō, ēlūgeō, frīgeō, exaugeō). Like Tergeō, except in 34 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("augeō", 7, "~geō", like Tergeō;
    stem participle_perf_pass: "~ct- like agitō";
    ind_act_perf_1: "~xī", "~ximus";
    ind_act_perf_2: "~xistī", "~xistis";
    ind_act_perf_3: "~xit", "~xērunt|~xēre";
    ind_act_plup_1: "~xeram", "~xerāmus";
    ind_act_plup_2: "~xerās", "~xerātis";
    ind_act_plup_3: "~xerat", "~xerant";
    ind_act_futp_1: "~xerō", "~xerimus";
    ind_act_futp_2: "~xeris", "~xeritis";
    ind_act_futp_3: "~xerit", "~xerint";
    ind_pass_pres_2: "~gēris|~gēre", _;
    ind_pass_impf_2: "~gēbāris|~gēbāre", _;
    ind_pass_fut_2: "~gēberis|~gēbere", _;
    subj_act_perf_1: "~xerim", "~xerīmus";
    subj_act_perf_2: "~xerīs", "~xerītis";
    subj_act_plup_1: "~xissem", "~xissēmus";
    subj_act_plup_2: "~xissēs", "~xissētis";
    subj_act_plup_3: "~xisset", "~xissent";
    subj_pass_pres_2: "~geāris|~geāre", _;
    subj_pass_impf_2: "~gērēris|~gērēre", _;
    infinitive_act: _, "~xisse", _;
    ),
    // Ferveō: 7 words (ferveō, cōnferveō, dēferveō, īnferveō, perferveō, referveō). Like Modulor, except in 48 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("ferveō", 7, "~veō", like Modulor;
    stem verb_act_sg: "~v- like ferveō";
    stem participle_perf_pass: "~vit- like labōrō";
    ind_act_pres_1: _, "~vēmus";
    ind_act_impf_1: "~vēbam", "~vēbāmus";
    ind_act_impf_2: "~vēbās", "~vēbātis";
    ind_act_impf_3: "~vēbat", "~vēbant";
    ind_act_fut_1: "~vēbō", "~vēbimus";
    ind_act_fut_2: "~vēbis", "~vēbitis";
    ind_act_fut_3: "~vēbit", _;
    ind_act_perf_1: "~buī", "~buimus";
    ind_act_perf_2: "~buistī", "~buistis";
    ind_act_perf_3: "~buit", "~buērunt|~buēre";
    ind_act_plup_1: "~bueram", "~buerāmus";
    ind_act_plup_2: "~buerās", "~buerātis";
    ind_act_plup_3: "~buerat", "~buerant";
    ind_act_futp_1: "~buerō", "~buerimus";
    ind_act_futp_2: "~bueris", "~bueritis";
    ind_act_futp_3: "~buerit", "~buerint";
    subj_act_pres_3: "~veat", _;
    subj_act_perf_1: "~buerim", "~buerīmus";
    subj_act_perf_2: "~buerīs", "~buerītis";
    subj_act_plup_1: "~buissem", "~buissēmus";
    subj_act_plup_2: "~buissēs", "~buissētis";
    subj_act_plup_3: "~buisset", "~buissent";
    imperative_act_fut_2: "~vētō", _;
    infinitive_act: _, "~buisse", _;
    participle: "~vēns", _, _, "~vendum";
    gerund: _, "~vendī", "~vendō", "~vendum", _, _;
    ),
    // Īnsīdō: 7 words (resīdō, assīdō, subsīdō, cōnsīdō, īnsīdō, dēsīdō). Like Possideō, except in 75 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("īnsīdō", 7, "~īdō", like Possideō;
    ind_act_pres_1: "~īdō", "~īdimus";
    ind_act_pres_2: "~īdis", "~īditis";
    ind_act_pres_3: "~īdit", "~īdunt";
    ind_act_impf_1: "~īdēbam", "~īdēbāmus";
    ind_act_impf_2: "~īdēbās", "~īdēbātis";
    ind_act_impf_3: "~īdēbat", "~īdēbant";
    ind_act_fut_1: "~īdam", "~īdēmus";
    ind_act_fut_2: "~īdēs", "~īdētis";
    ind_act_fut_3: "~īdet", "~īdent";
    ind_act_perf_3: _, "~ēdērunt";
    ind_pass_pres_1: "~īdor", "~īdimur";
    ind_pass_pres_2: "~īderis", "~īdiminī";
    ind_pass_pres_3: "~īditur", "~īduntur";
    ind_pass_impf_1: "~īdēbar", "~īdēbāmur";
    ind_pass_impf_2: "~īdēbāris", "~īdēbāminī";
    ind_pass_impf_3: "~īdēbātur", "~īdēbantur";
    ind_pass_fut_1: "~īdar", "~īdēmur";
    ind_pass_fut_2: "~īdēris", "~īdēminī";
    ind_pass_fut_3: "~īdētur", "~īdentur";
    subj_act_pres_1: "~īdam", "~īdāmus";
    subj_act_pres_2: "~īdās", "~īdātis";
    subj_act_pres_3: "~īdat", "~īdant";
    subj_act_impf_1: "~īderem", "~īderēmus";
    subj_act_impf_2: "~īderēs", "~īderētis";
    subj_act_impf_3: "~īderet", "~īderent";
    subj_pass_pres_1: "~īdar", "~īdāmur";
    subj_pass_pres_2: "~īdāris", "~īdāminī";
    subj_pass_pres_3: "~īdātur", "~īdantur";
    subj_pass_impf_1: "~īderer", "~īderēmur";
    subj_pass_impf_2: "~īderēris", "~īderēminī";
    subj_pass_impf_3: "~īderētur", "~īderentur";
    imperative_act_pres_2: "~īde", "~īdite";
    imperative_act_fut_2: "~īditō", "~īditōte";
    imperative_act_fut_3: _, "~īduntō";
    imperative_pass_pres_2: "~īdere", _;
    imperative_pass_fut_2: "~īditor", _;
    imperative_pass_fut_3: _, "~īduntor";
    infinitive_pass: "~īdī", _, _;
    participle: "~īdēns", _, _, "~īdendus";
    gerund: _, "~īdendī", "~īdendō", "~īdendum", _, _;
    ),
    // Dēpendeō: 7 words (strīdeō, commordeō, remordeō, ammordeō, dēpendeō, prōpendeō). Like Ārdeō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("dēpendeō", 7, "~deō", like Ārdeō;
    ind_act_perf_1: "~dī", "~dimus";
    ind_act_perf_2: "~distī", "~distis";
    ind_act_perf_3: "~dit", "~dērunt|~dēre";
    ind_act_plup_1: "~deram", "~derāmus";
    ind_act_plup_2: "~derās", "~derātis";
    ind_act_plup_3: "~derat", "~derant";
    ind_act_futp_1: "~derō", "~derimus";
    ind_act_futp_2: "~deris", "~deritis";
    ind_act_futp_3: "~derit", "~derint";
    subj_act_perf_1: "~derim", "~derīmus";
    subj_act_perf_2: "~derīs", "~derītis";
    subj_act_plup_1: "~dissem", "~dissēmus";
    subj_act_plup_2: "~dissēs", "~dissētis";
    subj_act_plup_3: "~disset", "~dissent";
    infinitive_act: _, "~disse", _;
    ),
    // Peragō: 7 words (peragō, satagō, reagō, circumagō, abagō, vervagō). Like Compingō, except in 75 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("peragō", 7, "~agō", like Compingō;
    ind_act_pres_1: "~agō", "~agimus";
    ind_act_pres_2: "~agis", "~agitis";
    ind_act_pres_3: "~agit", "~agunt";
    ind_act_impf_1: "~agēbam", "~agēbāmus";
    ind_act_impf_2: "~agēbās", "~agēbātis";
    ind_act_impf_3: "~agēbat", "~agēbant";
    ind_act_fut_1: "~agam", "~agēmus";
    ind_act_fut_2: "~agēs", "~agētis";
    ind_act_fut_3: "~aget", "~agent";
    ind_act_perf_3: _, "~ēgērunt";
    ind_pass_pres_1: "~agor", "~agimur";
    ind_pass_pres_2: "~ageris", "~agiminī";
    ind_pass_pres_3: "~agitur", "~aguntur";
    ind_pass_impf_1: "~agēbar", "~agēbāmur";
    ind_pass_impf_2: "~agēbāris", "~agēbāminī";
    ind_pass_impf_3: "~agēbātur", "~agēbantur";
    ind_pass_fut_1: "~agar", "~agēmur";
    ind_pass_fut_2: "~agēris", "~agēminī";
    ind_pass_fut_3: "~agētur", "~agentur";
    subj_act_pres_1: "~agam", "~agāmus";
    subj_act_pres_2: "~agās", "~agātis";
    subj_act_pres_3: "~agat", "~agant";
    subj_act_impf_1: "~agerem", "~agerēmus";
    subj_act_impf_2: "~agerēs", "~agerētis";
    subj_act_impf_3: "~ageret", "~agerent";
    subj_pass_pres_1: "~agar", "~agāmur";
    subj_pass_pres_2: "~agāris", "~agāminī";
    subj_pass_pres_3: "~agātur", "~agantur";
    subj_pass_impf_1: "~agerer", "~agerēmur";
    subj_pass_impf_2: "~agerēris", "~agerēminī";
    subj_pass_impf_3: "~agerētur", "~agerentur";
    imperative_act_pres_2: "~age", "~agite";
    imperative_act_fut_2: "~agitō", "~agitōte";
    imperative_act_fut_3: _, "~aguntō";
    imperative_pass_pres_2: "~agere", _;
    imperative_pass_fut_2: "~agitor", _;
    imperative_pass_fut_3: _, "~aguntor";
    infinitive_pass: "~agī", _, _;
    participle: "~agēns", _, _, "~agendus";
    gerund: _, "~agendī", "~agendō", "~agendum", _, _;
    ),
    // Īnserō: 7 words (disserō, serō, asserō, īnserō, cōnserō, adserō). Like Dēsinō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("īnserō", 7, "~erō", like Dēsinō;
    ind_act_pres_1: "~erō", "~erimus";
    ind_act_pres_2: "~eris", "~eritis";
    ind_act_pres_3: "~erit", "~erunt";
    ind_act_impf_1: "~erēbam", "~erēbāmus";
    ind_act_impf_2: "~erēbās", "~erēbātis";
    ind_act_impf_3: "~erēbat", "~erēbant";
    ind_act_fut_1: "~eram", "~erēmus";
    ind_act_fut_2: "~erēs", "~erētis";
    ind_act_fut_3: "~eret", "~erent";
    ind_pass_pres_1: "~eror", "~erimur";
    ind_pass_pres_2: "~ereris|~erere", "~eriminī";
    ind_pass_pres_3: "~eritur", "~eruntur";
    ind_pass_impf_1: "~erēbar", "~erēbāmur";
    ind_pass_impf_2: "~erēbāris|~erēbāre", "~erēbāminī";
    ind_pass_impf_3: "~erēbātur", "~erēbantur";
    ind_pass_fut_1: "~erar", "~erēmur";
    ind_pass_fut_2: "~erēris|~erēre", "~erēminī";
    ind_pass_fut_3: "~erētur", "~erentur";
    subj_act_pres_1: "~eram", "~erāmus";
    subj_act_pres_2: "~erās", "~erātis";
    subj_act_pres_3: "~erat", "~erant";
    subj_act_impf_1: "~ererem", "~ererēmus";
    subj_act_impf_2: "~ererēs", "~ererētis";
    subj_act_impf_3: "~ereret", "~ererent";
    subj_pass_pres_1: "~erar", "~erāmur";
    subj_pass_pres_2: "~erāris|~erāre", "~erāminī";
    subj_pass_pres_3: "~erātur", "~erantur";
    subj_pass_impf_1: "~ererer", "~ererēmur";
    subj_pass_impf_2: "~ererēris|~ererēre", "~ererēminī";
    subj_pass_impf_3: "~ererētur", "~ererentur";
    imperative_act_pres_2: "~ere", "~erite";
    imperative_act_fut_2: "~eritō", "~eritōte";
    imperative_act_fut_3: _, "~eruntō";
    imperative_pass_pres_2: "~erere", _;
    imperative_pass_fut_2: "~eritor", _;
    imperative_pass_fut_3: _, "~eruntor";
    infinitive_pass: "~erī", _, _;
    participle: "~erēns", _, _, "~erendus|~erundus";
    gerund: _, "~erendī", "~erendō", "~erendum", _, _;
    ),
    // Coorior: 7 words (aborior, ōrdior, experior, adorior, coorior, oborior). Like Mōlior, except in 0 cells: .
    conjugation!("coorior", 7, "~ior", like Mōlior;
    stem participle_perf_pass: "~t- like modulor";
    ),
    // Cēnseō: 6 words (recēnseō, cēnseō, paeniteō, accēnseō, percēnseō, poeniteō). Like Habeō, except in 5 cells: ind_pass_pres_2 sg, ind_pass_impf_2 sg, ind_pass_fut_2 sg, subj_pass_pres_2 sg, subj_pass_impf_2 sg.
    conjugation!("cēnseō", 6, "~eō", like Habeō;
    stem participle_perf_pass: "~- like agitō";
    ind_pass_pres_2: "~ēris|~ēre", _;
    ind_pass_impf_2: "~ēbāris|~ēbāre", _;
    ind_pass_fut_2: "~ēberis|~ēbere", _;
    subj_pass_pres_2: "~eāris|~eāre", _;
    subj_pass_impf_2: "~ērēris|~ērēre", _;
    ),
    // Pergō: 6 words (surgō, resurgō, pergō, cōnsurgō, exsurgō, exurgō). Like Mergō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("pergō", 6, "~gō", like Mergō;
    stem participle_perf_pass: "~rēct- like agitō";
    ind_act_perf_1: "~rēxī", "~rēximus";
    ind_act_perf_2: "~rēxistī", "~rēxistis";
    ind_act_perf_3: "~rēxit", "~rēxērunt|~rēxēre";
    ind_act_plup_1: "~rēxeram", "~rēxerāmus";
    ind_act_plup_2: "~rēxerās", "~rēxerātis";
    ind_act_plup_3: "~rēxerat", "~rēxerant";
    ind_act_futp_1: "~rēxerō", "~rēxerimus";
    ind_act_futp_2: "~rēxeris", "~rēxeritis";
    ind_act_futp_3: "~rēxerit", "~rēxerint";
    subj_act_perf_1: "~rēxerim", "~rēxerīmus";
    subj_act_perf_2: "~rēxerīs", "~rēxerītis";
    subj_act_plup_1: "~rēxissem", "~rēxissēmus";
    subj_act_plup_2: "~rēxissēs", "~rēxissētis";
    subj_act_plup_3: "~rēxisset", "~rēxissent";
    infinitive_act: _, "~rēxisse", _;
    ),
    // Vīvō: 6 words (vīvō, convīvō, revīvō, advīvō, supervīvō, fīvō). Like Solvō, except in 58 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("vīvō", 6, "~vō", like Solvō;
    stem verb_act_sg: "~v- like īnsistō";
    stem participle_perf_pass: "~ct- like labōrō";
    ind_act_perf_1: "~xī", "~ximus";
    ind_act_perf_2: "~xistī", "~xistis";
    ind_act_perf_3: "~xit", "~xērunt|~xēre";
    ind_act_plup_1: "~xeram", "~xerāmus";
    ind_act_plup_2: "~xerās", "~xerātis";
    ind_act_plup_3: "~xerat", "~xerant";
    ind_act_futp_1: "~xerō", "~xerimus";
    ind_act_futp_2: "~xeris", "~xeritis";
    ind_act_futp_3: "~xerit", "~xerint";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    subj_act_perf_1: "~xerim", "~xerīmus";
    subj_act_perf_2: "~xerīs", "~xerītis";
    subj_act_plup_1: "~xissem", "~xissēmus";
    subj_act_plup_2: "~xissēs", "~xissētis";
    subj_act_plup_3: "~xisset", "~xissent";
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: _, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: _, derived;
    infinitive_act: _, "~xisse", _;
    participle: _, _, _, "~vendum";
    ),
    // Cōnstō: 6 words (praestō, cōnstō, distō, īnstō, prōstō, antestō). Like Agitō, except in 34 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("cōnstō", 6, "~ō", like Agitō;
    ind_act_perf_1: "~itī", "~itimus";
    ind_act_perf_2: "~itistī", "~itistis";
    ind_act_perf_3: "~itit", "~itērunt|~itēre";
    ind_act_plup_1: "~iteram", "~iterāmus";
    ind_act_plup_2: "~iterās", "~iterātis";
    ind_act_plup_3: "~iterat", "~iterant";
    ind_act_futp_1: "~iterō", "~iterimus";
    ind_act_futp_2: "~iteris", "~iteritis";
    ind_act_futp_3: "~iterit", "~iterint";
    ind_pass_pres_2: "~āris", _;
    ind_pass_impf_2: "~ābāris", _;
    ind_pass_fut_2: "~āberis", _;
    subj_act_perf_1: "~iterim", "~iterīmus";
    subj_act_perf_2: "~iterīs", "~iterītis";
    subj_act_plup_1: "~itissem", "~itissēmus";
    subj_act_plup_2: "~itissēs", "~itissētis";
    subj_act_plup_3: "~itisset", "~itissent";
    subj_pass_pres_2: "~ēris", _;
    subj_pass_impf_2: "~ārēris", _;
    infinitive_act: _, "~itisse", _;
    ),
    // Capiō: 6 words (ūsūcapiō, iaciō, laciō, capiō, cōnsuēfaciō, disfaciō). Like Excipiō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("capiō", 6, "~a~iō", like Excipiō;
    stem participle_perf_pass: "~a~t- like agitō";
    ind_act_pres_1: "~a~iō", "~a~imus";
    ind_act_pres_2: "~a~is", "~a~itis";
    ind_act_pres_3: "~a~it", "~a~iunt";
    ind_act_impf_1: "~a~iēbam", "~a~iēbāmus";
    ind_act_impf_2: "~a~iēbās", "~a~iēbātis";
    ind_act_impf_3: "~a~iēbat", "~a~iēbant";
    ind_act_fut_1: "~a~iam", "~a~iēmus";
    ind_act_fut_2: "~a~iēs", "~a~iētis";
    ind_act_fut_3: "~a~iet", "~a~ient";
    ind_pass_pres_1: "~a~ior", "~a~imur";
    ind_pass_pres_2: "~a~eris|~a~ere", "~a~iminī";
    ind_pass_pres_3: "~a~itur", "~a~iuntur";
    ind_pass_impf_1: "~a~iēbar", "~a~iēbāmur";
    ind_pass_impf_2: "~a~iēbāris|~a~iēbāre", "~a~iēbāminī";
    ind_pass_impf_3: "~a~iēbātur", "~a~iēbantur";
    ind_pass_fut_1: "~a~iar", "~a~iēmur";
    ind_pass_fut_2: "~a~iēris|~a~iēre", "~a~iēminī";
    ind_pass_fut_3: "~a~iētur", "~a~ientur";
    subj_act_pres_1: "~a~iam", "~a~iāmus";
    subj_act_pres_2: "~a~iās", "~a~iātis";
    subj_act_pres_3: "~a~iat", "~a~iant";
    subj_act_impf_1: "~a~erem", "~a~erēmus";
    subj_act_impf_2: "~a~erēs", "~a~erētis";
    subj_act_impf_3: "~a~eret", "~a~erent";
    subj_pass_pres_1: "~a~iar", "~a~iāmur";
    subj_pass_pres_2: "~a~iāris|~a~iāre", "~a~iāminī";
    subj_pass_pres_3: "~a~iātur", "~a~iantur";
    subj_pass_impf_1: "~a~erer", "~a~erēmur";
    subj_pass_impf_2: "~a~erēris|~a~erēre", "~a~erēminī";
    subj_pass_impf_3: "~a~erētur", "~a~erentur";
    imperative_act_pres_2: "~a~e", "~a~ite";
    imperative_act_fut_2: "~a~itō", "~a~itōte";
    imperative_act_fut_3: _, "~a~iuntō";
    imperative_pass_pres_2: "~a~ere", _;
    imperative_pass_fut_2: "~a~itor", _;
    imperative_pass_fut_3: _, "~a~iuntor";
    infinitive_pass: "~a~ī", _, _;
    participle: "~a~iēns", _, _, "~a~iendus|~a~iundus";
    gerund: _, "~a~iendī", "~a~iendō", "~a~iendum", _, _;
    ),
    // Innectō: 6 words (cōnectō, connectō, subnectō, innectō, annectō, adnectō). Like Nectō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("innectō", 6, "~ctō", like Nectō;
    ind_act_perf_1: "~xuī", "~xuimus";
    ind_act_perf_2: "~xuistī", "~xuistis";
    ind_act_perf_3: "~xuit", "~xuērunt|~xuēre";
    ind_act_plup_1: "~xueram", "~xuerāmus";
    ind_act_plup_2: "~xuerās", "~xuerātis";
    ind_act_plup_3: "~xuerat", "~xuerant";
    ind_act_futp_1: "~xuerō", "~xuerimus";
    ind_act_futp_2: "~xueris", "~xueritis";
    ind_act_futp_3: "~xuerit", "~xuerint";
    subj_act_perf_1: "~xuerim", "~xuerīmus";
    subj_act_perf_2: "~xuerīs", "~xuerītis";
    subj_act_plup_1: "~xuissem", "~xuissēmus";
    subj_act_plup_2: "~xuissēs", "~xuissētis";
    subj_act_plup_3: "~xuisset", "~xuissent";
    infinitive_act: _, "~xuisse", _;
    ),
    // Eximō: 6 words (dirimō, eximō, redimō, adimō, interimō, perimō). Like Excipiō, except in 47 cells: ind_act_pres_1 sg, ind_act_pres_3 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ….
    conjugation!("eximō", 6, "~i~ō", like Excipiō;
    stem participle_perf_pass: "~ē~pt- like agitō";
    ind_act_pres_1: "~i~ō", _;
    ind_act_pres_3: _, "~i~unt";
    ind_act_impf_1: "~i~ēbam", "~i~ēbāmus";
    ind_act_impf_2: "~i~ēbās", "~i~ēbātis";
    ind_act_impf_3: "~i~ēbat", "~i~ēbant";
    ind_act_fut_1: "~i~am", "~i~ēmus";
    ind_act_fut_2: "~i~ēs", "~i~ētis";
    ind_act_fut_3: "~i~et", "~i~ent";
    ind_pass_pres_1: "~i~or", _;
    ind_pass_pres_3: _, "~i~untur";
    ind_pass_impf_1: "~i~ēbar", "~i~ēbāmur";
    ind_pass_impf_2: "~i~ēbāris|~i~ēbāre", "~i~ēbāminī";
    ind_pass_impf_3: "~i~ēbātur", "~i~ēbantur";
    ind_pass_fut_1: "~i~ar", "~i~ēmur";
    ind_pass_fut_2: "~i~ēris|~i~ēre", "~i~ēminī";
    ind_pass_fut_3: "~i~ētur", "~i~entur";
    subj_act_pres_1: "~i~am", "~i~āmus";
    subj_act_pres_2: "~i~ās", "~i~ātis";
    subj_act_pres_3: "~i~at", "~i~ant";
    subj_pass_pres_1: "~i~ar", "~i~āmur";
    subj_pass_pres_2: "~i~āris|~i~āre", "~i~āminī";
    subj_pass_pres_3: "~i~ātur", "~i~antur";
    imperative_act_fut_3: _, "~i~untō";
    imperative_pass_fut_3: _, "~i~untor";
    participle: "~i~ēns", _, _, "~i~endus|~i~undus";
    gerund: _, "~i~endī", "~i~endō", "~i~endum", _, _;
    ),
    // Vinciō: 6 words (vinciō, revinciō, dēvinciō, ēvinciō, praevinciō, circumvinciō). Like Tingō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("vinciō", 6, "~i~ciō", like Tingō;
    ind_act_pres_1: "~i~ciō", "~i~cīmus";
    ind_act_pres_2: "~i~cīs", "~i~cītis";
    ind_act_pres_3: "~i~cit", "~i~ciunt";
    ind_act_impf_1: "~i~ciēbam", "~i~ciēbāmus";
    ind_act_impf_2: "~i~ciēbās", "~i~ciēbātis";
    ind_act_impf_3: "~i~ciēbat", "~i~ciēbant";
    ind_act_fut_1: "~i~ciam", "~i~ciēmus";
    ind_act_fut_2: "~i~ciēs", "~i~ciētis";
    ind_act_fut_3: "~i~ciet", "~i~cient";
    ind_pass_pres_1: "~i~cior", "~i~cīmur";
    ind_pass_pres_2: "~i~cīris|~i~cīre", "~i~cīminī";
    ind_pass_pres_3: "~i~cītur", "~i~ciuntur";
    ind_pass_impf_1: "~i~ciēbar", "~i~ciēbāmur";
    ind_pass_impf_2: "~i~ciēbāris|~i~ciēbāre", "~i~ciēbāminī";
    ind_pass_impf_3: "~i~ciēbātur", "~i~ciēbantur";
    ind_pass_fut_1: "~i~ciar", "~i~ciēmur";
    ind_pass_fut_2: "~i~ciēris|~i~ciēre", "~i~ciēminī";
    ind_pass_fut_3: "~i~ciētur", "~i~cientur";
    subj_act_pres_1: "~i~ciam", "~i~ciāmus";
    subj_act_pres_2: "~i~ciās", "~i~ciātis";
    subj_act_pres_3: "~i~ciat", "~i~ciant";
    subj_act_impf_1: "~i~cīrem", "~i~cīrēmus";
    subj_act_impf_2: "~i~cīrēs", "~i~cīrētis";
    subj_act_impf_3: "~i~cīret", "~i~cīrent";
    subj_pass_pres_1: "~i~ciar", "~i~ciāmur";
    subj_pass_pres_2: "~i~ciāris|~i~ciāre", "~i~ciāminī";
    subj_pass_pres_3: "~i~ciātur", "~i~ciantur";
    subj_pass_impf_1: "~i~cīrer", "~i~cīrēmur";
    subj_pass_impf_2: "~i~cīrēris|~i~cīrēre", "~i~cīrēminī";
    subj_pass_impf_3: "~i~cīrētur", "~i~cīrentur";
    imperative_act_pres_2: "~i~cī", "~i~cīte";
    imperative_act_fut_2: "~i~cītō", "~i~cītōte";
    imperative_act_fut_3: _, "~i~ciuntō";
    imperative_pass_pres_2: "~i~cīre", _;
    imperative_pass_fut_2: "~i~cītor", _;
    imperative_pass_fut_3: _, "~i~ciuntor";
    infinitive_pass: "~i~cīrī", _, _;
    participle: "~i~ciēns", _, _, "~i~ciendus|~i~ciundus";
    gerund: _, "~i~ciendī", "~i~ciendō", "~i~ciendum", _, _;
    ),
    // Scandō: 6 words (expandō, scandō, oppandō, superscandō, suprascandō, dispandō).
    conjugation!("scandō", 6, "~a~dō";
    stem participle_perf_pass: "~ā~s- like agitō";
    ind_act_pres_1: "~a~dō", "~a~dimus";
    ind_act_pres_2: "~a~dis", "~a~ditis";
    ind_act_pres_3: "~a~dit", "~a~dunt";
    ind_act_impf_1: "~a~dēbam", "~a~dēbāmus";
    ind_act_impf_2: "~a~dēbās", "~a~dēbātis";
    ind_act_impf_3: "~a~dēbat", "~a~dēbant";
    ind_act_fut_1: "~a~dam", "~a~dēmus";
    ind_act_fut_2: "~a~dēs", "~a~dētis";
    ind_act_fut_3: "~a~det", "~a~dent";
    ind_act_perf_1: "~a~dī", "~a~dimus";
    ind_act_perf_2: "~a~distī", "~a~distis";
    ind_act_perf_3: "~a~dit", "~a~dērunt|~a~dēre";
    ind_act_plup_1: "~a~deram", "~a~derāmus";
    ind_act_plup_2: "~a~derās", "~a~derātis";
    ind_act_plup_3: "~a~derat", "~a~derant";
    ind_act_futp_1: "~a~derō", "~a~derimus";
    ind_act_futp_2: "~a~deris", "~a~deritis";
    ind_act_futp_3: "~a~derit", "~a~derint";
    ind_pass_pres_1: "~a~dor", "~a~dimur";
    ind_pass_pres_2: "~a~deris|~a~dere", "~a~diminī";
    ind_pass_pres_3: "~a~ditur", "~a~duntur";
    ind_pass_impf_1: "~a~dēbar", "~a~dēbāmur";
    ind_pass_impf_2: "~a~dēbāris|~a~dēbāre", "~a~dēbāminī";
    ind_pass_impf_3: "~a~dēbātur", "~a~dēbantur";
    ind_pass_fut_1: "~a~dar", "~a~dēmur";
    ind_pass_fut_2: "~a~dēris|~a~dēre", "~a~dēminī";
    ind_pass_fut_3: "~a~dētur", "~a~dentur";
    subj_act_pres_1: "~a~dam", "~a~dāmus";
    subj_act_pres_2: "~a~dās", "~a~dātis";
    subj_act_pres_3: "~a~dat", "~a~dant";
    subj_act_impf_1: "~a~derem", "~a~derēmus";
    subj_act_impf_2: "~a~derēs", "~a~derētis";
    subj_act_impf_3: "~a~deret", "~a~derent";
    subj_act_perf_1: "~a~derim", "~a~derīmus";
    subj_act_perf_2: "~a~derīs", "~a~derītis";
    subj_act_plup_1: "~a~dissem", "~a~dissēmus";
    subj_act_plup_2: "~a~dissēs", "~a~dissētis";
    subj_act_plup_3: "~a~disset", "~a~dissent";
    subj_pass_pres_1: "~a~dar", "~a~dāmur";
    subj_pass_pres_2: "~a~dāris|~a~dāre", "~a~dāminī";
    subj_pass_pres_3: "~a~dātur", "~a~dantur";
    subj_pass_impf_1: "~a~derer", "~a~derēmur";
    subj_pass_impf_2: "~a~derēris|~a~derēre", "~a~derēminī";
    subj_pass_impf_3: "~a~derētur", "~a~derentur";
    imperative_act_pres_2: "~a~de", "~a~dite";
    imperative_act_fut_2: "~a~ditō", "~a~ditōte";
    imperative_act_fut_3: _, "~a~duntō";
    imperative_pass_pres_2: "~a~dere", _;
    imperative_pass_fut_2: "~a~ditor", _;
    imperative_pass_fut_3: _, "~a~duntor";
    infinitive_act: _, "~a~disse", _;
    infinitive_pass: "~a~dī", _, _;
    participle: "~a~dēns", _, _, "~a~dendus|~a~dundus";
    gerund: _, "~a~dendī", "~a~dendō", "~a~dendum", _, _;
    ),
    // Dēfluō: 6 words (affluō, adfluō, īnfluō, dēfluō, effluō, perfluō). Like Prōfluō, except in 25 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("dēfluō", 6, "~uō", like Prōfluō;
    stem participle_perf_pass: "~ūx- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~uitor", _;
    infinitive_act: "~uere", _, _;
    participle: _, _, _, "~uendum";
    ),
    // Cōnfugiō: 6 words (refugiō, aufugiō, perfugiō, cōnfugiō, diffugiō, profugiō). Like Fugiō, except in 31 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ind_pass_impf_1 sg, ….
    conjugation!("cōnfugiō", 6, "~u~iō", like Fugiō;
    stem participle_perf_pass: "~u~it- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: _, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~u~itor", derived;
    infinitive_act: "~u~ere", _, _;
    participle: _, _, _, "~u~iendum|~u~iundum";
    ),
    // Apīscor: 6 words (apīscor, dēpecīscor, pacīscor, compacīscor, compecīscor, dēpacīscor). Like Nāscor, except in 19 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("apīscor", 6, "~īscor", like Nāscor;
    stem verb_act_sg: "~īsc- like cōnsequor";
    ind_act_pres_1: _, "~īscimur";
    ind_act_impf_1: "~īscēbar", "~īscēbāmur";
    ind_act_impf_2: "~īscēbāris|~īscēbāre", "~īscēbāminī";
    ind_act_impf_3: "~īscēbātur", "~īscēbantur";
    ind_act_fut_1: "~īscar", "~īscēmur";
    ind_act_fut_2: "~īscēris|~īscēre", "~īscēminī";
    ind_act_fut_3: "~īscētur", _;
    subj_act_pres_3: "~īscātur", _;
    imperative_act_fut_2: "~īscitor", _;
    participle: "~īscēns", _, _, "~īscendus|~īscundus";
    gerund: _, "~īscendī", "~īscendō", "~īscendum", _, _;
    ),
    // Sistō: 6 words (existō, persistō, subsistō, sistō, adsistō, coexsistō). Like Īnsistō, except in 34 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ind_pass_impf_1 sg, ….
    conjugation!("sistō", 6, "~~s~ō", like Īnsistō as ABL_AS_VOC;
    ind_pass_pres_1: "~~s~or", "~~s~imur";
    ind_pass_pres_2: "~~s~eris", "~~s~iminī";
    ind_pass_pres_3: _, "~~s~untur";
    ind_pass_impf_1: "~~s~ēbar", "~~s~ēbāmur";
    ind_pass_impf_2: "~~s~ēbāris", "~~s~ēbāminī";
    ind_pass_impf_3: _, "~~s~ēbantur";
    ind_pass_fut_1: "~~s~ar", "~~s~ēmur";
    ind_pass_fut_2: "~~s~ēris", "~~s~ēminī";
    ind_pass_fut_3: _, "~~s~entur";
    subj_pass_pres_1: "~~s~ar", "~~s~āmur";
    subj_pass_pres_2: "~~s~āris", "~~s~āminī";
    subj_pass_pres_3: _, "~~s~antur";
    subj_pass_impf_1: "~~s~erer", "~~s~erēmur";
    subj_pass_impf_2: "~~s~erēris", "~~s~erēminī";
    subj_pass_impf_3: _, "~~s~erentur";
    imperative_pass_pres_2: "~~s~ere", _;
    imperative_pass_fut_2: "~~s~itor", _;
    imperative_pass_fut_3: derived, "~~s~untor";
    infinitive_act: derived, _, _;
    participle: _, derived, derived, "~~s~endus";
    supine: _, _, _, derived, _, _;
    ),
    // Discō: 6 words (discō, condiscō, addiscō, ēdiscō, dēdiscō, perdiscō).
    conjugation!("discō", 6, "~s~ō";
    ind_act_pres_1: "~s~ō", "~s~imus";
    ind_act_pres_2: "~s~is", "~s~itis";
    ind_act_pres_3: "~s~it", "~s~unt";
    ind_act_impf_1: "~s~ēbam", "~s~ēbāmus";
    ind_act_impf_2: "~s~ēbās", "~s~ēbātis";
    ind_act_impf_3: "~s~ēbat", "~s~ēbant";
    ind_act_fut_1: "~s~am", "~s~ēmus";
    ind_act_fut_2: "~s~ēs", "~s~ētis";
    ind_act_fut_3: "~s~et", "~s~ent";
    ind_act_perf_1: "~di~ī", "~di~imus";
    ind_act_perf_2: "~di~istī", "~di~istis";
    ind_act_perf_3: "~di~it", "~di~ērunt|~di~ēre";
    ind_act_plup_1: "~di~eram", "~di~erāmus";
    ind_act_plup_2: "~di~erās", "~di~erātis";
    ind_act_plup_3: "~di~erat", "~di~erant";
    ind_act_futp_1: "~di~erō", "~di~erimus";
    ind_act_futp_2: "~di~eris", "~di~eritis";
    ind_act_futp_3: "~di~erit", "~di~erint";
    ind_pass_pres_1: "~s~or", "~s~imur";
    ind_pass_pres_2: "~s~eris", "~s~iminī";
    ind_pass_pres_3: "~s~itur", "~s~untur";
    ind_pass_impf_1: "~s~ēbar", "~s~ēbāmur";
    ind_pass_impf_2: "~s~ēbāris", "~s~ēbāminī";
    ind_pass_impf_3: "~s~ēbātur", "~s~ēbantur";
    ind_pass_fut_1: "~s~ar", "~s~ēmur";
    ind_pass_fut_2: "~s~ēris", "~s~ēminī";
    ind_pass_fut_3: "~s~ētur", "~s~entur";
    subj_act_pres_1: "~s~am", "~s~āmus";
    subj_act_pres_2: "~s~ās", "~s~ātis";
    subj_act_pres_3: "~s~at", "~s~ant";
    subj_act_impf_1: "~s~erem", "~s~erēmus";
    subj_act_impf_2: "~s~erēs", "~s~erētis";
    subj_act_impf_3: "~s~eret", "~s~erent";
    subj_act_perf_1: "~di~erim", "~di~erīmus";
    subj_act_perf_2: "~di~erīs", "~di~erītis";
    subj_act_plup_1: "~di~issem", "~di~issēmus";
    subj_act_plup_2: "~di~issēs", "~di~issētis";
    subj_act_plup_3: "~di~isset", "~di~issent";
    subj_pass_pres_1: "~s~ar", "~s~āmur";
    subj_pass_pres_2: "~s~āris", "~s~āminī";
    subj_pass_pres_3: "~s~ātur", "~s~antur";
    subj_pass_impf_1: "~s~erer", "~s~erēmur";
    subj_pass_impf_2: "~s~erēris", "~s~erēminī";
    subj_pass_impf_3: "~s~erētur", "~s~erentur";
    imperative_act_pres_2: "~s~e", "~s~ite";
    imperative_act_fut_2: "~s~itō", "~s~itōte";
    imperative_act_fut_3: _, "~s~untō";
    imperative_pass_pres_2: "~s~ere", _;
    imperative_pass_fut_2: "~s~itor", _;
    imperative_pass_fut_3: _, "~s~untor";
    infinitive_act: _, "~di~isse", _;
    infinitive_pass: "~s~ī", _, _;
    participle: "~s~ēns", _, _, "~s~endus";
    gerund: _, "~s~endī", "~s~endō", "~s~endum", _, _;
    ),
    // Deicō: 6 words (deicō, indeicō, addeicō, prodeicō, contrādeicō, interdeicō).
    conjugation!("deicō", 6, "~e~cō";
    stem participle_perf_pass: "~~ct- like agitō";
    ind_act_pres_1: "~e~cō", "~e~cimus";
    ind_act_pres_2: "~e~cis", "~e~citis";
    ind_act_pres_3: "~e~cit", "~e~cunt";
    ind_act_impf_1: "~e~cēbam", "~e~cēbāmus";
    ind_act_impf_2: "~e~cēbās", "~e~cēbātis";
    ind_act_impf_3: "~e~cēbat", "~e~cēbant";
    ind_act_fut_1: "~e~cam", "~e~cēmus";
    ind_act_fut_2: "~e~cēs", "~e~cētis";
    ind_act_fut_3: "~e~cet", "~e~cent";
    ind_act_perf_1: "~e~xī", "~e~ximus";
    ind_act_perf_2: "~e~xistī", "~e~xistis";
    ind_act_perf_3: "~e~xit", "~e~xērunt|~e~xēre";
    ind_act_plup_1: "~e~xeram", "~e~xerāmus";
    ind_act_plup_2: "~e~xerās", "~e~xerātis";
    ind_act_plup_3: "~e~xerat", "~e~xerant";
    ind_act_futp_1: "~e~xerō", "~e~xerimus";
    ind_act_futp_2: "~e~xeris", "~e~xeritis";
    ind_act_futp_3: "~e~xerit", "~e~xerint";
    ind_pass_pres_1: "~e~cor", "~e~cimur";
    ind_pass_pres_2: "~e~ceris|~e~cere", "~e~ciminī";
    ind_pass_pres_3: "~e~citur", "~e~cuntur";
    ind_pass_impf_1: "~e~cēbar", "~e~cēbāmur";
    ind_pass_impf_2: "~e~cēbāris|~e~cēbāre", "~e~cēbāminī";
    ind_pass_impf_3: "~e~cēbātur", "~e~cēbantur";
    ind_pass_fut_1: "~e~car", "~e~cēmur";
    ind_pass_fut_2: "~e~cēris|~e~cēre", "~e~cēminī";
    ind_pass_fut_3: "~e~cētur", "~e~centur";
    subj_act_pres_1: "~e~cam", "~e~cāmus";
    subj_act_pres_2: "~e~cās", "~e~cātis";
    subj_act_pres_3: "~e~cat", "~e~cant";
    subj_act_impf_1: "~e~cerem", "~e~cerēmus";
    subj_act_impf_2: "~e~cerēs", "~e~cerētis";
    subj_act_impf_3: "~e~ceret", "~e~cerent";
    subj_act_perf_1: "~e~xerim", "~e~xerīmus";
    subj_act_perf_2: "~e~xerīs", "~e~xerītis";
    subj_act_plup_1: "~e~xissem", "~e~xissēmus";
    subj_act_plup_2: "~e~xissēs", "~e~xissētis";
    subj_act_plup_3: "~e~xisset", "~e~xissent";
    subj_pass_pres_1: "~e~car", "~e~cāmur";
    subj_pass_pres_2: "~e~cāris|~e~cāre", "~e~cāminī";
    subj_pass_pres_3: "~e~cātur", "~e~cantur";
    subj_pass_impf_1: "~e~cerer", "~e~cerēmur";
    subj_pass_impf_2: "~e~cerēris|~e~cerēre", "~e~cerēminī";
    subj_pass_impf_3: "~e~cerētur", "~e~cerentur";
    imperative_act_pres_2: "~e~ce", "~e~cite";
    imperative_act_fut_2: "~e~citō", "~e~citōte";
    imperative_act_fut_3: _, "~e~cuntō";
    imperative_pass_pres_2: "~e~cere", _;
    imperative_pass_fut_2: "~e~citor", _;
    imperative_pass_fut_3: _, "~e~cuntor";
    infinitive_act: _, "~e~xisse", _;
    infinitive_pass: "~e~cī", _, _;
    participle: "~e~cēns", _, _, "~e~cendus|~e~cundus";
    gerund: _, "~e~cendī", "~e~cendō", "~e~cendum", _, _;
    ),
    // Occurrō: 5 words (carrō, antecurrō, circumcurrō, converrō, convorrō). Like Gerō, except in 35 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("occurrō", 5, "~rō", like Gerō;
    stem participle_perf_pass: "~s- like agitō";
    ind_act_perf_1: "~rī", "~rimus";
    ind_act_perf_2: "~ristī", "~ristis";
    ind_act_perf_3: "~rit", "~rērunt";
    ind_act_plup_1: "~reram", "~rerāmus";
    ind_act_plup_2: "~rerās", "~rerātis";
    ind_act_plup_3: "~rerat", "~rerant";
    ind_act_futp_1: "~rerō", "~rerimus";
    ind_act_futp_2: "~reris", "~reritis";
    ind_act_futp_3: "~rerit", "~rerint";
    ind_pass_pres_2: "~reris", _;
    ind_pass_impf_2: "~rēbāris", _;
    ind_pass_fut_2: "~rēris", _;
    subj_act_perf_1: "~rerim", "~rerīmus";
    subj_act_perf_2: "~rerīs", "~rerītis";
    subj_act_plup_1: "~rissem", "~rissēmus";
    subj_act_plup_2: "~rissēs", "~rissētis";
    subj_act_plup_3: "~risset", "~rissent";
    subj_pass_pres_2: "~rāris", _;
    subj_pass_impf_2: "~rerēris", _;
    infinitive_act: _, "~risse", _;
    participle: _, _, _, "~rendus";
    ),
    // Sūmō: 5 words (prōmō, dēmō, cōmō, exprōmō, dēprōmō). Like Petō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("sūmō", 5, "~ō", like Petō;
    stem participle_perf_pass: "~pt- like agitō";
    ind_act_perf_1: "~psī", "~psimus";
    ind_act_perf_2: "~psistī", "~psistis";
    ind_act_perf_3: "~psit", "~psērunt|~psēre";
    ind_act_plup_1: "~pseram", "~pserāmus";
    ind_act_plup_2: "~pserās", "~pserātis";
    ind_act_plup_3: "~pserat", "~pserant";
    ind_act_futp_1: "~pserō", "~pserimus";
    ind_act_futp_2: "~pseris", "~pseritis";
    ind_act_futp_3: "~pserit", "~pserint";
    subj_act_perf_1: "~pserim", "~pserīmus";
    subj_act_perf_2: "~pserīs", "~pserītis";
    subj_act_plup_1: "~psissem", "~psissēmus";
    subj_act_plup_2: "~psissēs", "~psissētis";
    subj_act_plup_3: "~psisset", "~psissent";
    infinitive_act: _, "~psisse", _;
    ),
    // Vēnumdō: 5 words (dō, circundō, circumdō, vēnundō, vēnumdō). Like Agitō, except in 77 cells: ind_act_pres_1 pl, ind_act_pres_2 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ….
    conjugation!("vēnumdō", 5, "~ō", like Agitō;
    stem participle_perf_pass: "~at- like agitō";
    ind_act_pres_1: _, "~amus";
    ind_act_pres_2: _, "~atis";
    ind_act_impf_1: "~abam", "~abāmus";
    ind_act_impf_2: "~abās", "~abātis";
    ind_act_impf_3: "~abat", "~abant";
    ind_act_fut_1: "~abō", "~abimus";
    ind_act_fut_2: "~abis", "~abitis";
    ind_act_fut_3: "~abit", "~abunt";
    ind_act_perf_1: "~edī", "~edimus";
    ind_act_perf_2: "~edistī", "~edistis";
    ind_act_perf_3: "~edit", "~edērunt|~edēre";
    ind_act_plup_1: "~ederam", "~ederāmus";
    ind_act_plup_2: "~ederās", "~ederātis";
    ind_act_plup_3: "~ederat", "~ederant";
    ind_act_futp_1: "~ederō", "~ederimus";
    ind_act_futp_2: "~ederis", "~ederitis";
    ind_act_futp_3: "~ederit", "~ederint";
    ind_pass_pres_1: _, "~amur";
    ind_pass_pres_2: "~aris|~are", "~aminī";
    ind_pass_pres_3: "~atur", _;
    ind_pass_impf_1: "~abar", "~abāmur";
    ind_pass_impf_2: "~abāris|~abāre", "~abāminī";
    ind_pass_impf_3: "~abātur", "~abantur";
    ind_pass_fut_1: "~abor", "~abimur";
    ind_pass_fut_2: "~aberis|~abere", "~abiminī";
    ind_pass_fut_3: "~abitur", "~abuntur";
    subj_act_impf_1: "~arem", "~arēmus";
    subj_act_impf_2: "~arēs", "~arētis";
    subj_act_impf_3: "~aret", "~arent";
    subj_act_perf_1: "~ederim", "~ederīmus";
    subj_act_perf_2: "~ederīs", "~ederītis";
    subj_act_plup_1: "~edissem", "~edissēmus";
    subj_act_plup_2: "~edissēs", "~edissētis";
    subj_act_plup_3: "~edisset", "~edissent";
    subj_pass_impf_1: "~arer", "~arēmur";
    subj_pass_impf_2: "~arēris|~arēre", "~arēminī";
    subj_pass_impf_3: "~arētur", "~arentur";
    imperative_act_pres_2: _, "~ate";
    imperative_act_fut_2: "~atō", "~atōte";
    imperative_pass_pres_2: "~are", _;
    imperative_pass_fut_2: "~ator", _;
    infinitive_act: _, "~edisse", _;
    infinitive_pass: "~arī", _, _;
    ),
    // Serviō: 5 words (serviō, dormiō, hymniō, saeviō, īnsāniō). Like Mūniō, except in 32 cells: ind_act_perf_1 sg, ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ….
    conjugation!("serviō", 5, "~iō", like Mūniō;
    stem participle_perf_pass: "~īt- like labōrō";
    ind_act_perf_1: "~īvī|~iī", _;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: _, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~ītor", derived;
    infinitive_act: "~īre", _, _;
    participle: _, _, _, "~iendum|~iundum";
    ),
    // Adipīscor: 5 words (proficīscor, adipīscor, comminīscor, indipīscor, minīscor). Like Cōnfugiō, except in 29 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("adipīscor", 5, "~i~īscor", like Cōnfugiō as ACT_AS_PASS;
    stem verb_act_sg: "~i~īsc- like cōnsequor";
    stem participle_perf_pass: "~e~t- like modulor";
    ind_act_pres_1: _, "~i~īscimur";
    ind_act_impf_1: "~i~īscēbar", "~i~īscēbāmur";
    ind_act_impf_2: "~i~īscēbāris|~i~īscēbāre", "~i~īscēbāminī";
    ind_act_impf_3: "~i~īscēbātur", "~i~īscēbantur";
    ind_act_fut_1: "~i~īscar", "~i~īscēmur";
    ind_act_fut_2: "~i~īscēris|~i~īscēre", "~i~īscēminī";
    ind_act_fut_3: "~i~īscētur", _;
    ind_pass_pres_3: derived, _;
    ind_pass_impf_3: derived, _;
    ind_pass_fut_3: derived, _;
    subj_act_pres_3: "~i~īscātur", _;
    subj_pass_pres_3: derived, _;
    subj_pass_impf_3: derived, _;
    imperative_act_fut_2: "~i~īscitor", derived;
    imperative_act_fut_3: derived, _;
    imperative_pass_fut_3: derived, _;
    infinitive_pass: derived, _, _;
    participle: "~i~īscēns", derived, _, "~i~īscendus|~i~īscundus";
    gerund: _, "~i~īscendī", "~i~īscendō", "~i~īscendum", _, _;
    ),
    // Astō: 5 words (restō, obstō, astō, perstō, adstō). Like Cōnstō, except in 33 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ind_pass_impf_1 sg, ….
    conjugation!("astō", 5, "~ō", like Cōnstō as ABL_AS_VOC;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: _, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~ātor", derived;
    infinitive_act: "~āre", _, _;
    participle: _, derived, _, "~andum";
    supine: _, _, _, derived, _, _;
    ),
    // Occinō: 5 words (concinō, accinō, praecinō, incinō, occinō). Like Eximō, except in 35 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("occinō", 5, "~i~ō", like Eximō;
    stem participle_perf_pass: "~e~t- like agitō";
    ind_act_perf_1: "~i~uī", "~i~uimus";
    ind_act_perf_2: "~i~uistī", "~i~uistis";
    ind_act_perf_3: "~i~uit", "~i~uērunt";
    ind_act_plup_1: "~i~ueram", "~i~uerāmus";
    ind_act_plup_2: "~i~uerās", "~i~uerātis";
    ind_act_plup_3: "~i~uerat", "~i~uerant";
    ind_act_futp_1: "~i~uerō", "~i~uerimus";
    ind_act_futp_2: "~i~ueris", "~i~ueritis";
    ind_act_futp_3: "~i~uerit", "~i~uerint";
    ind_pass_pres_2: "~i~eris", _;
    ind_pass_impf_2: "~i~ēbāris", _;
    ind_pass_fut_2: "~i~ēris", _;
    subj_act_perf_1: "~i~uerim", "~i~uerīmus";
    subj_act_perf_2: "~i~uerīs", "~i~uerītis";
    subj_act_plup_1: "~i~uissem", "~i~uissēmus";
    subj_act_plup_2: "~i~uissēs", "~i~uissētis";
    subj_act_plup_3: "~i~uisset", "~i~uissent";
    subj_pass_pres_2: "~i~āris", _;
    subj_pass_impf_2: "~i~erēris", _;
    infinitive_act: _, "~i~uisse", _;
    participle: _, _, _, "~i~endus";
    ),
    // Mulceō: 5 words (mulceō, commulceō, dēmulceō, permulceō, remulceō). Like Compercō, except in 48 cells: ind_act_pres_1 sg, ind_act_fut_1 sg, ind_act_fut_1 pl, ind_act_fut_2 sg, ind_act_fut_2 pl, ind_act_fut_3 sg, ….
    conjugation!("mulceō", 5, "~ceō", like Compercō as PRES_AS_FUT;
    stem participle_perf_pass: "~s- like agitō";
    ind_act_pres_1: "~ceō", _;
    ind_act_fut_1: "~cēbō", "~cēbimus";
    ind_act_fut_2: "~cēbis", "~cēbitis";
    ind_act_fut_3: "~cēbit", "~cēbunt";
    ind_pass_pres_1: "~ceor", _;
    ind_pass_fut_1: "~cēbor", "~cēbimur";
    ind_pass_fut_2: "~cēberis|~cēbere", "~cēbiminī";
    ind_pass_fut_3: "~cēbitur", "~cēbuntur";
    subj_act_pres_1: "~ceam", "~ceāmus";
    subj_act_pres_2: "~ceās", "~ceātis";
    subj_act_pres_3: "~ceat", "~ceant";
    subj_act_impf_1: "~cērem", "~cērēmus";
    subj_act_impf_2: "~cērēs", "~cērētis";
    subj_act_impf_3: "~cēret", "~cērent";
    subj_pass_pres_1: "~cear", "~ceāmur";
    subj_pass_pres_2: "~ceāris|~ceāre", "~ceāminī";
    subj_pass_pres_3: "~ceātur", "~ceantur";
    subj_pass_impf_1: "~cērer", "~cērēmur";
    subj_pass_impf_2: "~cērēris|~cērēre", "~cērēminī";
    subj_pass_impf_3: "~cērētur", "~cērentur";
    imperative_act_pres_2: "~cē", "~cēte";
    imperative_act_fut_2: "~cētō", "~cētōte";
    imperative_act_fut_3: _, "~centō";
    imperative_pass_pres_2: "~cēre", _;
    imperative_pass_fut_2: "~cētor", _;
    imperative_pass_fut_3: _, "~centor";
    infinitive_pass: "~cērī", _, _;
    participle: _, _, _, "~cendus";
    ),
    // Sedeō: 5 words (sedeō, circumsedeō, persedeō, supersedeō, suprāsedeō). Like Comedō, except in 50 cells: ind_act_pres_1 sg, ind_act_fut_1 sg, ind_act_fut_1 pl, ind_act_fut_2 sg, ind_act_fut_2 pl, ind_act_fut_3 sg, ….
    conjugation!("sedeō", 5, "~edeō", like Comedō as PRES_AS_FUT;
    stem participle_perf_pass: "~ess- like agitō";
    ind_act_pres_1: "~edeō", _;
    ind_act_fut_1: "~edēbō", "~edēbimus";
    ind_act_fut_2: "~edēbis", "~edēbitis";
    ind_act_fut_3: "~edēbit", "~edēbunt";
    ind_pass_pres_1: "~edeor", _;
    ind_pass_pres_2: "~edēris", _;
    ind_pass_impf_2: "~edēbāris", _;
    ind_pass_fut_1: "~edēbor", "~edēbimur";
    ind_pass_fut_2: "~edēberis", "~edēbiminī";
    ind_pass_fut_3: "~edēbitur", "~edēbuntur";
    subj_act_pres_1: "~edeam", "~edeāmus";
    subj_act_pres_2: "~edeās", "~edeātis";
    subj_act_pres_3: "~edeat", "~edeant";
    subj_act_impf_1: "~edērem", "~edērēmus";
    subj_act_impf_2: "~edērēs", "~edērētis";
    subj_act_impf_3: "~edēret", "~edērent";
    subj_pass_pres_1: "~edear", "~edeāmur";
    subj_pass_pres_2: "~edeāris", "~edeāminī";
    subj_pass_pres_3: "~edeātur", "~edeantur";
    subj_pass_impf_1: "~edērer", "~edērēmur";
    subj_pass_impf_2: "~edērēris", "~edērēminī";
    subj_pass_impf_3: "~edērētur", "~edērentur";
    imperative_act_pres_2: "~edē", "~edēte";
    imperative_act_fut_2: "~edētō", "~edētōte";
    imperative_act_fut_3: _, "~edentō";
    imperative_pass_pres_2: "~edēre", _;
    imperative_pass_fut_2: "~edētor", _;
    imperative_pass_fut_3: _, "~edentor";
    infinitive_pass: "~edērī", _, _;
    participle: _, _, _, "~edendus";
    ),
    // Clueō: 5 words (clueō, langueō, abnueō, cēveō, connīveō). Like Habeō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("clueō", 5, "~eō", like Habeō;
    ind_act_perf_1: "~ī", "~imus";
    ind_act_perf_2: "~istī", "~istis";
    ind_act_perf_3: "~it", "~ērunt";
    ind_act_plup_1: "~eram", "~erāmus";
    ind_act_plup_2: "~erās", "~erātis";
    ind_act_plup_3: "~erat", "~erant";
    ind_act_futp_1: "~erō", "~erimus";
    ind_act_futp_2: "~eris", "~eritis";
    ind_act_futp_3: "~erit", "~erint";
    subj_act_perf_1: "~erim", "~erīmus";
    subj_act_perf_2: "~erīs", "~erītis";
    subj_act_plup_1: "~issem", "~issēmus";
    subj_act_plup_2: "~issēs", "~issētis";
    subj_act_plup_3: "~isset", "~issent";
    infinitive_act: _, "~isse", _;
    ),
    // Ambigō: 5 words (convergō, assurgō, ambigō, īnsurgō, adsurgō). Like Pergō, except in 25 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("ambigō", 5, "~gō", like Pergō;
    stem participle_perf_pass: "~rēct- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~gitor", _;
    infinitive_act: "~gere", _, _;
    participle: _, _, _, "~gendum|~gundum";
    ),
    // Contremīscō: 5 words (resipīscō, ingemīscō, dēlitīscō, conticīscō, contremīscō). Like Apīscor, except in 69 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("contremīscō", 5, "~īscō", like Apīscor as PASS_AS_ACT;
    ind_act_pres_1: "~īscō", "~īscimus";
    ind_act_pres_2: "~īscis", "~īscitis";
    ind_act_pres_3: "~īscit", "~īscunt";
    ind_act_impf_1: "~īscēbam", "~īscēbāmus";
    ind_act_impf_2: "~īscēbās", "~īscēbātis";
    ind_act_impf_3: "~īscēbat", "~īscēbant";
    ind_act_fut_1: "~īscam", "~īscēmus";
    ind_act_fut_2: "~īscēs", "~īscētis";
    ind_act_fut_3: "~īscet", "~īscent";
    ind_act_perf_1: "~uī", "~uimus";
    ind_act_perf_2: "~uistī", "~uistis";
    ind_act_perf_3: "~uit", "~uērunt|~uēre";
    ind_act_plup_1: "~ueram", "~uerāmus";
    ind_act_plup_2: "~uerās", "~uerātis";
    ind_act_plup_3: "~uerat", "~uerant";
    ind_act_futp_1: "~uerō", "~uerimus";
    ind_act_futp_2: "~ueris", "~ueritis";
    ind_act_futp_3: "~uerit", "~uerint";
    subj_act_pres_1: "~īscam", "~īscāmus";
    subj_act_pres_2: "~īscās", "~īscātis";
    subj_act_pres_3: "~īscat", "~īscant";
    subj_act_impf_1: "~īscerem", "~īscerēmus";
    subj_act_impf_2: "~īscerēs", "~īscerētis";
    subj_act_impf_3: "~īsceret", "~īscerent";
    subj_act_perf_1: "~uerim", "~uerīmus";
    subj_act_perf_2: "~uerīs", "~uerītis";
    subj_act_plup_1: "~uissem", "~uissēmus";
    subj_act_plup_2: "~uissēs", "~uissētis";
    subj_act_plup_3: "~uisset", "~uissent";
    imperative_act_pres_2: "~īsce", "~īscite";
    imperative_act_fut_2: "~īscitō", "~īscitōte";
    imperative_act_fut_3: _, "~īscuntō";
    imperative_pass_pres_2: _, derived;
    infinitive_act: derived, "~uisse", _;
    participle: _, _, derived, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Clangō: 5 words (cadō, stertō, clangō, accanō, obtexō). Like Compescō, except in 28 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("clangō", 5, "~ō", like Compescō as ABL_AS_VOC;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~itor", _;
    infinitive_act: "~ere", _, _;
    participle: _, derived, derived, "~endum|~undum";
    supine: _, _, _, derived, _, _;
    ),
    // Dēpudet: 5 words (taedet, piget, pertaedet, dēpudet, dispudet). Like Decet, except in 17 cells: ind_act_impf_3 sg, ind_act_fut_3 sg, ind_act_perf_3 sg, ind_act_plup_3 sg, ind_act_futp_3 sg, ind_pass_pres_3 sg, ….
    conjugation!("dēpudet", 5, "~et", like Decet as C3_AS_1;
    stem participle_perf_pass: "~it- like labōrō";
    ind_act_impf_3: "~ēbat", _;
    ind_act_fut_3: "~ēbit", _;
    ind_act_perf_3: "~uit", _;
    ind_act_plup_3: "~uerat", _;
    ind_act_futp_3: "~uerit", _;
    ind_pass_pres_3: "~ētur", _;
    ind_pass_impf_3: "~ēbātur", _;
    ind_pass_fut_3: "~ēbitur", _;
    subj_act_pres_3: "~eat", _;
    subj_act_impf_3: "~ēret", _;
    subj_act_plup_3: "~uisset", _;
    subj_pass_pres_3: "~eātur", _;
    subj_pass_impf_3: "~ērētur", _;
    imperative_act_fut_3: "~ētō", _;
    imperative_pass_fut_3: "~ētor", _;
    infinitive_pass: "~ērī", _, _;
    participle: _, _, _, "~endum";
    ),
    // Caveō: 4 words (caveō, faveō, paveō, praecaveō).
    conjugation!("caveō", 4, "~aveō";
    stem participle_perf_pass: "~aut- like agitō";
    ind_act_pres_1: "~aveō", "~avēmus";
    ind_act_pres_2: "~avēs", "~avētis";
    ind_act_pres_3: "~avet", "~avent";
    ind_act_impf_1: "~avēbam", "~avēbāmus";
    ind_act_impf_2: "~avēbās", "~avēbātis";
    ind_act_impf_3: "~avēbat", "~avēbant";
    ind_act_fut_1: "~avēbō", "~avēbimus";
    ind_act_fut_2: "~avēbis", "~avēbitis";
    ind_act_fut_3: "~avēbit", "~avēbunt";
    ind_act_perf_1: "~āvī", "~āvimus|~āmus";
    ind_act_perf_2: "~āvistī|~āstī", "~āvistis|~āstis";
    ind_act_perf_3: "~āvit|~āt", "~āvērunt|~ārunt|~āvēre";
    ind_act_plup_1: "~āveram|~āram", "~āverāmus|~ārāmus";
    ind_act_plup_2: "~āverās|~ārās", "~āverātis|~ārātis";
    ind_act_plup_3: "~āverat|~ārat", "~āverant|~ārant";
    ind_act_futp_1: "~āverō|~ārō", "~āverimus|~ārimus";
    ind_act_futp_2: "~āveris|~āris", "~āveritis|~āritis";
    ind_act_futp_3: "~āverit|~ārit", "~āverint|~ārint";
    ind_pass_pres_1: "~aveor", "~avēmur";
    ind_pass_pres_2: "~avēris|~avēre", "~avēminī";
    ind_pass_pres_3: "~avētur", "~aventur";
    ind_pass_impf_1: "~avēbar", "~avēbāmur";
    ind_pass_impf_2: "~avēbāris|~avēbāre", "~avēbāminī";
    ind_pass_impf_3: "~avēbātur", "~avēbantur";
    ind_pass_fut_1: "~avēbor", "~avēbimur";
    ind_pass_fut_2: "~avēberis|~avēbere", "~avēbiminī";
    ind_pass_fut_3: "~avēbitur", "~avēbuntur";
    subj_act_pres_1: "~aveam", "~aveāmus";
    subj_act_pres_2: "~aveās", "~aveātis";
    subj_act_pres_3: "~aveat", "~aveant";
    subj_act_impf_1: "~avērem", "~avērēmus";
    subj_act_impf_2: "~avērēs", "~avērētis";
    subj_act_impf_3: "~avēret", "~avērent";
    subj_act_perf_1: "~āverim|~ārim", "~āverīmus|~ārīmus";
    subj_act_perf_2: "~āverīs|~ārīs", "~āverītis|~ārītis";
    subj_act_plup_1: "~āvissem|~āssem", "~āvissēmus|~āssēmus";
    subj_act_plup_2: "~āvissēs|~āssēs", "~āvissētis|~āssētis";
    subj_act_plup_3: "~āvisset|~āsset", "~āvissent|~āssent";
    subj_pass_pres_1: "~avear", "~aveāmur";
    subj_pass_pres_2: "~aveāris|~aveāre", "~aveāminī";
    subj_pass_pres_3: "~aveātur", "~aveantur";
    subj_pass_impf_1: "~avērer", "~avērēmur";
    subj_pass_impf_2: "~avērēris|~avērēre", "~avērēminī";
    subj_pass_impf_3: "~avērētur", "~avērentur";
    imperative_act_pres_2: "~avē", "~avēte";
    imperative_act_fut_2: "~avētō", "~avētōte";
    imperative_act_fut_3: _, "~aventō";
    imperative_pass_pres_2: "~avēre", _;
    imperative_pass_fut_2: "~avētor", _;
    imperative_pass_fut_3: _, "~aventor";
    infinitive_act: _, "~āvisse|~āsse", _;
    infinitive_pass: "~avērī", _, _;
    participle: "~avēns", _, _, "~avendus";
    gerund: _, "~avendī", "~avendō", "~avendum", _, _;
    ),
    // Cieō: 4 words (cieō, excieō, concieō, percieō). Like Mūniō, except in 48 cells: ind_act_pres_1 sg, ind_act_fut_1 sg, ind_act_fut_1 pl, ind_act_fut_2 sg, ind_act_fut_2 pl, ind_act_fut_3 sg, ….
    conjugation!("cieō", 4, "~ieō", like Mūniō as PRES_AS_FUT;
    stem participle_perf_pass: "~it- like agitō";
    ind_act_pres_1: "~ieō", _;
    ind_act_fut_1: "~iēbō", "~iēbimus";
    ind_act_fut_2: "~iēbis", "~iēbitis";
    ind_act_fut_3: "~iēbit", "~iēbunt";
    ind_pass_pres_1: "~ieor", _;
    ind_pass_fut_1: "~iēbor", "~iēbimur";
    ind_pass_fut_2: "~iēberis|~iēbere", "~iēbiminī";
    ind_pass_fut_3: "~iēbitur", "~iēbuntur";
    subj_act_pres_1: "~ieam", "~ieāmus";
    subj_act_pres_2: "~ieās", "~ieātis";
    subj_act_pres_3: "~ieat", "~ieant";
    subj_act_impf_1: "~iērem", "~iērēmus";
    subj_act_impf_2: "~iērēs", "~iērētis";
    subj_act_impf_3: "~iēret", "~iērent";
    subj_pass_pres_1: "~iear", "~ieāmur";
    subj_pass_pres_2: "~ieāris|~ieāre", "~ieāminī";
    subj_pass_pres_3: "~ieātur", "~ieantur";
    subj_pass_impf_1: "~iērer", "~iērēmur";
    subj_pass_impf_2: "~iērēris|~iērēre", "~iērēminī";
    subj_pass_impf_3: "~iērētur", "~iērentur";
    imperative_act_pres_2: "~iē", "~iēte";
    imperative_act_fut_2: "~iētō", "~iētōte";
    imperative_act_fut_3: _, "~ientō";
    imperative_pass_pres_2: "~iēre", _;
    imperative_pass_fut_2: "~iētor", _;
    imperative_pass_fut_3: _, "~ientor";
    infinitive_pass: "~iērī", _, _;
    participle: _, _, _, "~iendus";
    ),
    // Patior: 4 words (patior, compatior, perpetior, conpatior). Like Mētior, except in 2 cells: ind_act_pres_1 pl, imperative_act_fut_2 sg.
    conjugation!("patior", 4, "~tior", like Mētior;
    stem verb_act_sg: "~t- like ēgredior";
    stem participle_perf_pass: "~ss- like modulor";
    ind_act_pres_1: _, "~timur";
    imperative_act_fut_2: "~titor", _;
    ),
    // Fīō: 4 words (fīō, obsolefīō, dēfīō, vacēfīō). Like Faciō, except in 43 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 sg, ind_pass_pres_3 pl, ….
    conjugation!("fīō", 4, "~īō", like Faciō as ACT_AS_PASS;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_pass_pres_2: derived, derived;
    imperative_pass_fut_2: derived, derived;
    imperative_pass_fut_3: _, derived;
    infinitive_pass: derived, _, _;
    participle: "~īēns", derived, derived, "~īendus";
    gerund: _, "~īendī", "~īendō", "~īendum", _, _;
    ),
    // Relinquō: 4 words (dēlinquō, relinquō, linquō, dērelinquō).
    conjugation!("relinquō", 4, "~inquō";
    stem participle_perf_pass: "~ict- like agitō";
    ind_act_pres_1: "~inquō", "~inquimus";
    ind_act_pres_2: "~inquis", "~inquitis";
    ind_act_pres_3: "~inquit", "~inquunt";
    ind_act_impf_1: "~inquēbam", "~inquēbāmus";
    ind_act_impf_2: "~inquēbās", "~inquēbātis";
    ind_act_impf_3: "~inquēbat", "~inquēbant";
    ind_act_fut_1: "~inquam", "~inquēmus";
    ind_act_fut_2: "~inquēs", "~inquētis";
    ind_act_fut_3: "~inquet", "~inquent";
    ind_act_perf_1: "~īquī", "~īquimus";
    ind_act_perf_2: "~īquistī", "~īquistis";
    ind_act_perf_3: "~īquit", "~īquērunt|~īquēre";
    ind_act_plup_1: "~īqueram", "~īquerāmus";
    ind_act_plup_2: "~īquerās", "~īquerātis";
    ind_act_plup_3: "~īquerat", "~īquerant";
    ind_act_futp_1: "~īquerō", "~īquerimus";
    ind_act_futp_2: "~īqueris", "~īqueritis";
    ind_act_futp_3: "~īquerit", "~īquerint";
    ind_pass_pres_1: "~inquor", "~inquimur";
    ind_pass_pres_2: "~inqueris|~inquere", "~inquiminī";
    ind_pass_pres_3: "~inquitur", "~inquuntur";
    ind_pass_impf_1: "~inquēbar", "~inquēbāmur";
    ind_pass_impf_2: "~inquēbāris|~inquēbāre", "~inquēbāminī";
    ind_pass_impf_3: "~inquēbātur", "~inquēbantur";
    ind_pass_fut_1: "~inquar", "~inquēmur";
    ind_pass_fut_2: "~inquēris|~inquēre", "~inquēminī";
    ind_pass_fut_3: "~inquētur", "~inquentur";
    subj_act_pres_1: "~inquam", "~inquāmus";
    subj_act_pres_2: "~inquās", "~inquātis";
    subj_act_pres_3: "~inquat", "~inquant";
    subj_act_impf_1: "~inquerem", "~inquerēmus";
    subj_act_impf_2: "~inquerēs", "~inquerētis";
    subj_act_impf_3: "~inqueret", "~inquerent";
    subj_act_perf_1: "~īquerim", "~īquerīmus";
    subj_act_perf_2: "~īquerīs", "~īquerītis";
    subj_act_plup_1: "~īquissem", "~īquissēmus";
    subj_act_plup_2: "~īquissēs", "~īquissētis";
    subj_act_plup_3: "~īquisset", "~īquissent";
    subj_pass_pres_1: "~inquar", "~inquāmur";
    subj_pass_pres_2: "~inquāris|~inquāre", "~inquāminī";
    subj_pass_pres_3: "~inquātur", "~inquantur";
    subj_pass_impf_1: "~inquerer", "~inquerēmur";
    subj_pass_impf_2: "~inquerēris|~inquerēre", "~inquerēminī";
    subj_pass_impf_3: "~inquerētur", "~inquerentur";
    imperative_act_pres_2: "~inque", "~inquite";
    imperative_act_fut_2: "~inquitō", "~inquitōte";
    imperative_act_fut_3: _, "~inquuntō";
    imperative_pass_pres_2: "~inquere", _;
    imperative_pass_fut_2: "~inquitor", _;
    imperative_pass_fut_3: _, "~inquuntor";
    infinitive_act: _, "~īquisse", _;
    infinitive_pass: "~inquī", _, _;
    participle: "~inquēns", _, _, "~inquendus";
    gerund: _, "~inquendī", "~inquendō", "~inquendum", _, _;
    ),
    // Sapiō: 4 words (cupiō, sapiō, concupiō, supersapiō). Like Mūniō, except in 30 cells: ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ….
    conjugation!("sapiō", 4, "~iō", like Mūniō;
    ind_act_pres_1: _, "~imus";
    ind_act_pres_2: "~is", "~itis";
    ind_pass_pres_1: _, "~imur";
    ind_pass_pres_2: "~eris", "~iminī";
    ind_pass_pres_3: "~itur", _;
    ind_pass_impf_2: "~iēbāris", _;
    ind_pass_fut_2: "~iēris", _;
    subj_act_impf_1: "~erem", "~erēmus";
    subj_act_impf_2: "~erēs", "~erētis";
    subj_act_impf_3: "~eret", "~erent";
    subj_pass_pres_2: "~iāris", _;
    subj_pass_impf_1: "~erer", "~erēmur";
    subj_pass_impf_2: "~erēris", "~erēminī";
    subj_pass_impf_3: "~erētur", "~erentur";
    imperative_act_pres_2: "~e", "~ite";
    imperative_act_fut_2: "~itō", "~itōte";
    imperative_pass_pres_2: "~ere", _;
    imperative_pass_fut_2: "~itor", _;
    infinitive_pass: "~ī", _, _;
    participle: _, _, _, "~iendus";
    ),
    // Abeō: 4 words (abeō, prōdeō, pereō, redeō). Like Obeō, except in 31 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ind_pass_impf_1 sg, ….
    conjugation!("abeō", 4, "~eō", like Obeō;
    stem participle_perf_pass: "~it- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: _, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~ītor", derived;
    infinitive_act: "~īre", _, _;
    participle: _, _, _, "~eundum";
    ),
    // Ēvellō: 4 words (dīvellō, convellō, ēvellō, refellō). Like Pellō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("ēvellō", 4, "~e~lō", like Pellō;
    ind_act_perf_1: "~e~lī", "~e~limus";
    ind_act_perf_2: "~e~listī", "~e~listis";
    ind_act_perf_3: "~e~lit", "~e~lērunt|~e~lēre";
    ind_act_plup_1: "~e~leram", "~e~lerāmus";
    ind_act_plup_2: "~e~lerās", "~e~lerātis";
    ind_act_plup_3: "~e~lerat", "~e~lerant";
    ind_act_futp_1: "~e~lerō", "~e~lerimus";
    ind_act_futp_2: "~e~leris", "~e~leritis";
    ind_act_futp_3: "~e~lerit", "~e~lerint";
    subj_act_perf_1: "~e~lerim", "~e~lerīmus";
    subj_act_perf_2: "~e~lerīs", "~e~lerītis";
    subj_act_plup_1: "~e~lissem", "~e~lissēmus";
    subj_act_plup_2: "~e~lissēs", "~e~lissētis";
    subj_act_plup_3: "~e~lisset", "~e~lissent";
    infinitive_act: _, "~e~lisse", _;
    ),
    // Currō: 4 words (currō, percurrō, prōcurrō, superexcurrō).
    conjugation!("currō", 4, "~~rō";
    stem participle_perf_pass: "~~s- like agitō";
    ind_act_pres_1: "~~rō", "~~rimus";
    ind_act_pres_2: "~~ris", "~~ritis";
    ind_act_pres_3: "~~rit", "~~runt";
    ind_act_impf_1: "~~rēbam", "~~rēbāmus";
    ind_act_impf_2: "~~rēbās", "~~rēbātis";
    ind_act_impf_3: "~~rēbat", "~~rēbant";
    ind_act_fut_1: "~~ram", "~~rēmus";
    ind_act_fut_2: "~~rēs", "~~rētis";
    ind_act_fut_3: "~~ret", "~~rent";
    ind_act_perf_1: "~cu~rī", "~cu~rimus";
    ind_act_perf_2: "~cu~ristī", "~cu~ristis";
    ind_act_perf_3: "~cu~rit", "~cu~rērunt";
    ind_act_plup_1: "~cu~reram", "~cu~rerāmus";
    ind_act_plup_2: "~cu~rerās", "~cu~rerātis";
    ind_act_plup_3: "~cu~rerat", "~cu~rerant";
    ind_act_futp_1: "~cu~rerō", "~cu~rerimus";
    ind_act_futp_2: "~cu~reris", "~cu~reritis";
    ind_act_futp_3: "~cu~rerit", "~cu~rerint";
    ind_pass_pres_1: "~~ror", "~~rimur";
    ind_pass_pres_2: "~~reris|~~rere", "~~riminī";
    ind_pass_pres_3: "~~ritur", "~~runtur";
    ind_pass_impf_1: "~~rēbar", "~~rēbāmur";
    ind_pass_impf_2: "~~rēbāris|~~rēbāre", "~~rēbāminī";
    ind_pass_impf_3: "~~rēbātur", "~~rēbantur";
    ind_pass_fut_1: "~~rar", "~~rēmur";
    ind_pass_fut_2: "~~rēris|~~rēre", "~~rēminī";
    ind_pass_fut_3: "~~rētur", "~~rentur";
    subj_act_pres_1: "~~ram", "~~rāmus";
    subj_act_pres_2: "~~rās", "~~rātis";
    subj_act_pres_3: "~~rat", "~~rant";
    subj_act_impf_1: "~~rerem", "~~rerēmus";
    subj_act_impf_2: "~~rerēs", "~~rerētis";
    subj_act_impf_3: "~~reret", "~~rerent";
    subj_act_perf_1: "~cu~rerim", "~cu~rerīmus";
    subj_act_perf_2: "~cu~rerīs", "~cu~rerītis";
    subj_act_plup_1: "~cu~rissem", "~cu~rissēmus";
    subj_act_plup_2: "~cu~rissēs", "~cu~rissētis";
    subj_act_plup_3: "~cu~risset", "~cu~rissent";
    subj_pass_pres_1: "~~rar", "~~rāmur";
    subj_pass_pres_2: "~~rāris|~~rāre", "~~rāminī";
    subj_pass_pres_3: "~~rātur", "~~rantur";
    subj_pass_impf_1: "~~rerer", "~~rerēmur";
    subj_pass_impf_2: "~~rerēris|~~rerēre", "~~rerēminī";
    subj_pass_impf_3: "~~rerētur", "~~rerentur";
    imperative_act_pres_2: "~~re", "~~rite";
    imperative_act_fut_2: "~~ritō", "~~ritōte";
    imperative_act_fut_3: _, "~~runtō";
    imperative_pass_pres_2: "~~rere", _;
    imperative_pass_fut_2: "~~ritor", _;
    imperative_pass_fut_3: _, "~~runtor";
    infinitive_act: _, "~cu~risse", _;
    infinitive_pass: "~~rī", _, _;
    participle: "~~rēns", _, _, "~~rendus|~~rundus";
    gerund: _, "~~rendī", "~~rendō", "~~rendum", _, _;
    ),
    // Obtundō: 4 words (obtundō, pertundō, contundō, extundō). Like Effundō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("obtundō", 4, "~undō", like Effundō;
    ind_act_perf_1: "~udī", "~udimus";
    ind_act_perf_2: "~udistī", "~udistis";
    ind_act_perf_3: "~udit", "~udērunt|~udēre";
    ind_act_plup_1: "~uderam", "~uderāmus";
    ind_act_plup_2: "~uderās", "~uderātis";
    ind_act_plup_3: "~uderat", "~uderant";
    ind_act_futp_1: "~uderō", "~uderimus";
    ind_act_futp_2: "~uderis", "~uderitis";
    ind_act_futp_3: "~uderit", "~uderint";
    subj_act_perf_1: "~uderim", "~uderīmus";
    subj_act_perf_2: "~uderīs", "~uderītis";
    subj_act_plup_1: "~udissem", "~udissēmus";
    subj_act_plup_2: "~udissēs", "~udissētis";
    subj_act_plup_3: "~udisset", "~udissent";
    infinitive_act: _, "~udisse", _;
    ),
    // Hauriō: 4 words (exhauriō, hauriō, dehauriō, perhauriō). Like Agō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("hauriō", 4, "~riō", like Agō;
    stem participle_perf_pass: "~st- like agitō";
    ind_act_pres_1: "~riō", "~rīmus";
    ind_act_pres_2: "~rīs", "~rītis";
    ind_act_pres_3: "~rit", "~riunt";
    ind_act_impf_1: "~riēbam", "~riēbāmus";
    ind_act_impf_2: "~riēbās", "~riēbātis";
    ind_act_impf_3: "~riēbat", "~riēbant";
    ind_act_fut_1: "~riam", "~riēmus";
    ind_act_fut_2: "~riēs", "~riētis";
    ind_act_fut_3: "~riet", "~rient";
    ind_pass_pres_1: "~rior", "~rīmur";
    ind_pass_pres_2: "~rīris|~rīre", "~rīminī";
    ind_pass_pres_3: "~rītur", "~riuntur";
    ind_pass_impf_1: "~riēbar", "~riēbāmur";
    ind_pass_impf_2: "~riēbāris|~riēbāre", "~riēbāminī";
    ind_pass_impf_3: "~riēbātur", "~riēbantur";
    ind_pass_fut_1: "~riar", "~riēmur";
    ind_pass_fut_2: "~riēris|~riēre", "~riēminī";
    ind_pass_fut_3: "~riētur", "~rientur";
    subj_act_pres_1: "~riam", "~riāmus";
    subj_act_pres_2: "~riās", "~riātis";
    subj_act_pres_3: "~riat", "~riant";
    subj_act_impf_1: "~rīrem", "~rīrēmus";
    subj_act_impf_2: "~rīrēs", "~rīrētis";
    subj_act_impf_3: "~rīret", "~rīrent";
    subj_pass_pres_1: "~riar", "~riāmur";
    subj_pass_pres_2: "~riāris|~riāre", "~riāminī";
    subj_pass_pres_3: "~riātur", "~riantur";
    subj_pass_impf_1: "~rīrer", "~rīrēmur";
    subj_pass_impf_2: "~rīrēris|~rīrēre", "~rīrēminī";
    subj_pass_impf_3: "~rīrētur", "~rīrentur";
    imperative_act_pres_2: "~rī", "~rīte";
    imperative_act_fut_2: "~rītō", "~rītōte";
    imperative_act_fut_3: _, "~riuntō";
    imperative_pass_pres_2: "~rīre", _;
    imperative_pass_fut_2: "~rītor", _;
    imperative_pass_fut_3: _, "~riuntor";
    infinitive_pass: "~rīrī", _, _;
    participle: "~riēns", _, _, "~riendus|~riundus";
    gerund: _, "~riendī", "~riendō", "~riendum", _, _;
    ),
    // Comperiō: 4 words (comperiō, īciō, repperiō, conperiō). Like Intersaepiō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("comperiō", 4, "~iō", like Intersaepiō;
    ind_act_perf_1: "~ī", "~imus";
    ind_act_perf_2: "~istī", "~istis";
    ind_act_perf_3: "~it", "~ērunt|~ēre";
    ind_act_plup_1: "~eram", "~erāmus";
    ind_act_plup_2: "~erās", "~erātis";
    ind_act_plup_3: "~erat", "~erant";
    ind_act_futp_1: "~erō", "~erimus";
    ind_act_futp_2: "~eris", "~eritis";
    ind_act_futp_3: "~erit", "~erint";
    subj_act_perf_1: "~erim", "~erīmus";
    subj_act_perf_2: "~erīs", "~erītis";
    subj_act_plup_1: "~issem", "~issēmus";
    subj_act_plup_2: "~issēs", "~issētis";
    subj_act_plup_3: "~isset", "~issent";
    infinitive_act: _, "~isse", _;
    ),
    // Circumvehor: 4 words (circumvehor, praetervehor, praevehor, supervehor). Like Cōnsequor, except in 19 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("circumvehor", 4, "~hor", like Cōnsequor;
    stem verb_act_sg: "~h- like cōnsequor";
    stem participle_perf_pass: "~ct- like modulor";
    ind_act_pres_1: _, "~himur";
    ind_act_impf_1: "~hēbar", "~hēbāmur";
    ind_act_impf_2: "~hēbāris|~hēbāre", "~hēbāminī";
    ind_act_impf_3: "~hēbātur", "~hēbantur";
    ind_act_fut_1: "~har", "~hēmur";
    ind_act_fut_2: "~hēris|~hēre", "~hēminī";
    ind_act_fut_3: "~hētur", _;
    subj_act_pres_3: "~hātur", _;
    imperative_act_fut_2: "~hitor", _;
    participle: "~hēns", _, _, "~hendus|~hundus";
    gerund: _, "~hendī", "~hendō", "~hendum", _, _;
    ),
    // Adnītor: 4 words (innītor, obnītor, adnītor, annītor). Like Nītor, except in 3 cells: participle fut_act, supine acc, supine abl.
    conjugation!("adnītor", 4, "~tor", like Nītor;
    participle: _, _, "~xūrus|~sūrus", _;
    supine: _, _, _, "~xum|~sum", "~xū|~sū", _;
    ),
    // Fruor: 4 words (fruor, compluor, conpluor, tuor). Like Minuō, except in 36 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 sg, ind_pass_pres_3 pl, ….
    conjugation!("fruor", 4, "~uor", like Minuō as ACT_AS_PASS;
    stem verb_act_sg: "~u- like cōnsequor";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_act_fut_2: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: _, derived;
    infinitive_pass: derived, _, _;
    participle: _, derived, _, _;
    ),
    // Sum: 4 words (sum, inquam, apage, coest). Like Adipīscor, except in 11 cells: ind_act_pres_1 sg, imperative_act_pres_2 sg, imperative_act_pres_2 pl, participle pres_act, participle fut_act, participle fut_pass, ….
    conjugation!("sum", 4, "~~", like Adipīscor as ACT_AS_PASS;
    ind_act_pres_1: "~~", _;
    imperative_act_pres_2: "~~", "~it~";
    participle: derived, _, derived, derived;
    gerund: _, derived, derived, derived, _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Secō: 3 words (secō, fricō, dēfricō). Like Cubō, except in 0 cells: .
    conjugation!("secō", 3, "~ō", like Cubō;
    stem participle_perf_pass: "~t- like agitō";
    ),
    // Quaerō: 3 words (quaerō, conquaerō, disquīrō). Like Gerō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("quaerō", 3, "~rō", like Gerō;
    stem participle_perf_pass: "~sīt- like agitō";
    ind_act_perf_1: "~sīvī", "~sīvimus";
    ind_act_perf_2: "~sīvistī", "~sīvistis";
    ind_act_perf_3: "~sīvit", "~sīvērunt|~sīvēre";
    ind_act_plup_1: "~sīveram", "~sīverāmus";
    ind_act_plup_2: "~sīverās", "~sīverātis";
    ind_act_plup_3: "~sīverat", "~sīverant";
    ind_act_futp_1: "~sīverō", "~sīverimus";
    ind_act_futp_2: "~sīveris", "~sīveritis";
    ind_act_futp_3: "~sīverit", "~sīverint";
    subj_act_perf_1: "~sīverim", "~sīverīmus";
    subj_act_perf_2: "~sīverīs", "~sīverītis";
    subj_act_plup_1: "~sīvissem", "~sīvissēmus";
    subj_act_plup_2: "~sīvissēs", "~sīvissētis";
    subj_act_plup_3: "~sīvisset", "~sīvissent";
    infinitive_act: _, "~sīvisse", _;
    ),
    // Tangō: 3 words (frangō, tangō, dēpangō). Like Peragō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("tangō", 3, "~angō", like Peragō;
    ind_act_pres_1: "~angō", "~angimus";
    ind_act_pres_2: "~angis", "~angitis";
    ind_act_pres_3: "~angit", "~angunt";
    ind_act_impf_1: "~angēbam", "~angēbāmus";
    ind_act_impf_2: "~angēbās", "~angēbātis";
    ind_act_impf_3: "~angēbat", "~angēbant";
    ind_act_fut_1: "~angam", "~angēmus";
    ind_act_fut_2: "~angēs", "~angētis";
    ind_act_fut_3: "~anget", "~angent";
    ind_pass_pres_1: "~angor", "~angimur";
    ind_pass_pres_2: "~angeris|~angere", "~angiminī";
    ind_pass_pres_3: "~angitur", "~anguntur";
    ind_pass_impf_1: "~angēbar", "~angēbāmur";
    ind_pass_impf_2: "~angēbāris|~angēbāre", "~angēbāminī";
    ind_pass_impf_3: "~angēbātur", "~angēbantur";
    ind_pass_fut_1: "~angar", "~angēmur";
    ind_pass_fut_2: "~angēris|~angēre", "~angēminī";
    ind_pass_fut_3: "~angētur", "~angentur";
    subj_act_pres_1: "~angam", "~angāmus";
    subj_act_pres_2: "~angās", "~angātis";
    subj_act_pres_3: "~angat", "~angant";
    subj_act_impf_1: "~angerem", "~angerēmus";
    subj_act_impf_2: "~angerēs", "~angerētis";
    subj_act_impf_3: "~angeret", "~angerent";
    subj_pass_pres_1: "~angar", "~angāmur";
    subj_pass_pres_2: "~angāris|~angāre", "~angāminī";
    subj_pass_pres_3: "~angātur", "~angantur";
    subj_pass_impf_1: "~angerer", "~angerēmur";
    subj_pass_impf_2: "~angerēris|~angerēre", "~angerēminī";
    subj_pass_impf_3: "~angerētur", "~angerentur";
    imperative_act_pres_2: "~ange", "~angite";
    imperative_act_fut_2: "~angitō", "~angitōte";
    imperative_act_fut_3: _, "~anguntō";
    imperative_pass_pres_2: "~angere", _;
    imperative_pass_fut_2: "~angitor", _;
    imperative_pass_fut_3: _, "~anguntor";
    infinitive_pass: "~angī", _, _;
    participle: "~angēns", _, _, "~angendus|~angundus";
    gerund: _, "~angendī", "~angendō", "~angendum", _, _;
    ),
    // Adliciō: 3 words (illiciō, inliciō, adliciō). Like Dispiciō, except in 29 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("adliciō", 3, "~iciō", like Dispiciō;
    ind_act_perf_1: "~ĕxī", "~ĕximus";
    ind_act_perf_2: "~ĕxistī", "~ĕxistis";
    ind_act_perf_3: "~ĕxit", "~ĕxērunt|~ĕxēre";
    ind_act_plup_1: "~ĕxeram", "~ĕxerāmus";
    ind_act_plup_2: "~ĕxerās", "~ĕxerātis";
    ind_act_plup_3: "~ĕxerat", "~ĕxerant";
    ind_act_futp_1: "~ĕxerō", "~ĕxerimus";
    ind_act_futp_2: "~ĕxeris", "~ĕxeritis";
    ind_act_futp_3: "~ĕxerit", "~ĕxerint";
    subj_act_perf_1: "~ĕxerim", "~ĕxerīmus";
    subj_act_perf_2: "~ĕxerīs", "~ĕxerītis";
    subj_act_plup_1: "~ĕxissem", "~ĕxissēmus";
    subj_act_plup_2: "~ĕxissēs", "~ĕxissētis";
    subj_act_plup_3: "~ĕxisset", "~ĕxissent";
    infinitive_act: _, "~ĕxisse", _;
    ),
    // Stō: 3 words (stō, interstō, substō). Like Astō, except in 32 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("stō", 3, "~ō", like Astō;
    ind_act_perf_1: "~etī", "~etimus";
    ind_act_perf_2: "~etistī", "~etistis";
    ind_act_perf_3: "~etit", "~etērunt";
    ind_act_plup_1: "~eteram", "~eterāmus";
    ind_act_plup_2: "~eterās", "~eterātis";
    ind_act_plup_3: "~eterat", "~eterant";
    ind_act_futp_1: "~eterō", "~eterimus";
    ind_act_futp_2: "~eteris", "~eteritis";
    ind_act_futp_3: "~eterit", "~eterint";
    subj_act_perf_1: "~eterim", "~eterīmus";
    subj_act_perf_2: "~eterīs", "~eterītis";
    subj_act_plup_1: "~etissem", "~etissēmus";
    subj_act_plup_2: "~etissēs", "~etissētis";
    subj_act_plup_3: "~etisset", "~etissent";
    infinitive_act: _, "~etisse", _;
    participle: _, "~atum", _, _;
    supine: _, _, _, "~atum", "~atū", _;
    ),
    // Dīluō: 3 words (suō, tribuō, dīluō). Like Minuō, except in 25 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("dīluō", 3, "~uō", like Minuō;
    stem participle_perf_pass: "~ūt- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~uitor", _;
    infinitive_act: "~uere", _, _;
    participle: _, _, _, "~uendum";
    ),
    // Iubeō: 3 words (iubeō, fidēiubeō, ioubeō). Like Percutiō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("iubeō", 3, "~beō", like Percutiō;
    ind_act_pres_1: "~beō", "~bēmus";
    ind_act_pres_2: "~bēs", "~bētis";
    ind_act_pres_3: "~bet", "~bent";
    ind_act_impf_1: "~bēbam", "~bēbāmus";
    ind_act_impf_2: "~bēbās", "~bēbātis";
    ind_act_impf_3: "~bēbat", "~bēbant";
    ind_act_fut_1: "~bēbō", "~bēbimus";
    ind_act_fut_2: "~bēbis", "~bēbitis";
    ind_act_fut_3: "~bēbit", "~bēbunt";
    ind_pass_pres_1: "~beor", "~bēmur";
    ind_pass_pres_2: "~bēris|~bēre", "~bēminī";
    ind_pass_pres_3: "~bētur", "~bentur";
    ind_pass_impf_1: "~bēbar", "~bēbāmur";
    ind_pass_impf_2: "~bēbāris|~bēbāre", "~bēbāminī";
    ind_pass_impf_3: "~bēbātur", "~bēbantur";
    ind_pass_fut_1: "~bēbor", "~bēbimur";
    ind_pass_fut_2: "~bēberis|~bēbere", "~bēbiminī";
    ind_pass_fut_3: "~bēbitur", "~bēbuntur";
    subj_act_pres_1: "~beam", "~beāmus";
    subj_act_pres_2: "~beās", "~beātis";
    subj_act_pres_3: "~beat", "~beant";
    subj_act_impf_1: "~bērem", "~bērēmus";
    subj_act_impf_2: "~bērēs", "~bērētis";
    subj_act_impf_3: "~bēret", "~bērent";
    subj_pass_pres_1: "~bear", "~beāmur";
    subj_pass_pres_2: "~beāris|~beāre", "~beāminī";
    subj_pass_pres_3: "~beātur", "~beantur";
    subj_pass_impf_1: "~bērer", "~bērēmur";
    subj_pass_impf_2: "~bērēris|~bērēre", "~bērēminī";
    subj_pass_impf_3: "~bērētur", "~bērentur";
    imperative_act_pres_2: "~bē", "~bēte";
    imperative_act_fut_2: "~bētō", "~bētōte";
    imperative_act_fut_3: _, "~bentō";
    imperative_pass_pres_2: "~bēre", _;
    imperative_pass_fut_2: "~bētor", _;
    imperative_pass_fut_3: _, "~bentor";
    infinitive_pass: "~bērī", _, _;
    participle: "~bēns", _, _, "~bendus";
    gerund: _, "~bendī", "~bendō", "~bendum", _, _;
    ),
    // Apiō: 3 words (rapiō, apiō, speciō). Like Aperiō, except in 27 cells: ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_perf_3 pl, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ….
    conjugation!("apiō", 3, "~iō", like Aperiō;
    ind_act_pres_1: _, "~imus";
    ind_act_pres_2: "~is", "~itis";
    ind_act_perf_3: _, "~uērunt";
    ind_pass_pres_1: _, "~imur";
    ind_pass_pres_2: "~eris|~ere", "~iminī";
    ind_pass_pres_3: "~itur", _;
    subj_act_impf_1: "~erem", "~erēmus";
    subj_act_impf_2: "~erēs", "~erētis";
    subj_act_impf_3: "~eret", "~erent";
    subj_pass_impf_1: "~erer", "~erēmur";
    subj_pass_impf_2: "~erēris|~erēre", "~erēminī";
    subj_pass_impf_3: "~erētur", "~erentur";
    imperative_act_pres_2: "~e", "~ite";
    imperative_act_fut_2: "~itō", "~itōte";
    imperative_pass_pres_2: "~ere", _;
    imperative_pass_fut_2: "~itor", _;
    infinitive_pass: "~ī", _, _;
    ),
    // Fundō: 3 words (fundō, refundō, praefundō). Like Effundō, except in 25 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("fundō", 3, "~undō", like Effundō;
    stem participle_perf_pass: "~ūs- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~unditor", _;
    infinitive_act: "~undere", _, _;
    participle: _, _, _, "~undendum|~undundum";
    ),
    // Scatō: 3 words (scatō, battō, abbattō). Like Compescō, except in 0 cells: .
    conjugation!("scatō", 3, "~ō", like Compescō;
    stem participle_perf_pass: "~ūt- like agitō";
    ),
    // Perpōtō: 3 words (pōtō, ēpōtō, perpōtō). Like Agitō, except in 0 cells: .
    conjugation!("perpōtō", 3, "~ō", like Agitō;
    stem participle_perf_pass: "~- like agitō";
    ),
    // Dēfungor: 3 words (fungor, dēfungor, perfungor). Like Coniungō, except in 36 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 sg, ind_pass_pres_3 pl, ….
    conjugation!("dēfungor", 3, "~u~gor", like Coniungō as ACT_AS_PASS;
    stem verb_act_sg: "~u~g- like cōnsequor";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_act_fut_2: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: _, derived;
    infinitive_pass: derived, _, _;
    participle: _, derived, _, _;
    ),
    // Pāscō: 3 words (pāscō, dēpāscō, repāscō). Like Quiēscō, except in 34 cells: ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ind_act_plup_1 sg, ….
    conjugation!("pāscō", 3, "~scō", like Quiēscō;
    stem participle_perf_pass: "~st- like agitō";
    ind_act_perf_1: _, "~vimus|~mus";
    ind_act_perf_2: "~vistī|~stī", "~vistis|~stis";
    ind_act_perf_3: "~vit|~t", "~vērunt|~runt|~vēre";
    ind_act_plup_1: "~veram|~ram", "~verāmus|~rāmus";
    ind_act_plup_2: "~verās|~rās", "~verātis|~rātis";
    ind_act_plup_3: "~verat|~rat", "~verant|~rant";
    ind_act_futp_1: "~verō|~rō", "~verimus|~rimus";
    ind_act_futp_2: "~veris|~ris", "~veritis|~ritis";
    ind_act_futp_3: "~verit|~rit", "~verint|~rint";
    ind_pass_pres_2: "~sceris|~scere", _;
    ind_pass_impf_2: "~scēbāris|~scēbāre", _;
    ind_pass_fut_2: "~scēris|~scēre", _;
    subj_act_perf_1: "~verim|~rim", "~verīmus|~rīmus";
    subj_act_perf_2: "~verīs|~rīs", "~verītis|~rītis";
    subj_act_plup_1: "~vissem|~ssem", "~vissēmus|~ssēmus";
    subj_act_plup_2: "~vissēs|~ssēs", "~vissētis|~ssētis";
    subj_act_plup_3: "~visset|~sset", "~vissent|~ssent";
    subj_pass_pres_2: "~scāris|~scāre", _;
    subj_pass_impf_2: "~scerēris|~scerēre", _;
    infinitive_act: _, "~visse|~sse", _;
    participle: _, _, _, "~scendus|~scundus";
    ),
    // Ōdī: 3 words (ōdī, coepī, recoepī). Like Adsum, except in 31 cells: ind_act_pres_1 sg, ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ….
    conjugation!("ōdī", 3, "~ī", like Adsum as ACT_AS_PASS;
    stem participle_perf_pass: "~t- like agitō";
    ind_act_pres_1: "~ī", _;
    ind_act_perf_1: "~ī", "~imus";
    ind_act_perf_2: "~istī", "~istis";
    ind_act_perf_3: "~it", "~ērunt|~ēre";
    ind_act_plup_1: "~eram", "~erāmus";
    ind_act_plup_2: "~erās", "~erātis";
    ind_act_plup_3: "~erat", "~erant";
    ind_act_futp_1: "~erō", "~erimus";
    ind_act_futp_2: "~eris", "~eritis";
    ind_act_futp_3: "~erit", "~erint";
    subj_act_perf_1: "~erim", "~erīmus";
    subj_act_perf_2: "~erīs", "~erītis";
    subj_act_plup_1: "~issem", "~issēmus";
    subj_act_plup_2: "~issēs", "~issētis";
    subj_act_plup_3: "~isset", "~issent";
    infinitive_act: _, "~isse", _;
    participle: derived, _, _, _;
    ),
    // Dētergeō: 3 words (dētergeō, immulgeō, inmulgeō). Like Tergeō, except in 25 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("dētergeō", 3, "~geō", like Tergeō;
    stem participle_perf_pass: "~s- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~gētor", _;
    infinitive_act: "~gēre", _, _;
    participle: _, _, _, "~gendum";
    ),
    // Differō: 3 words (offerō, differō, sufferō).
    conjugation!("differō", 3, "~fferō";
    stem participle_perf_pass: "~blāt- like agitō";
    ind_act_pres_1: "~fferō", "~fferimus";
    ind_act_pres_2: "~ffers", "~ffertis";
    ind_act_pres_3: "~ffert", "~fferunt";
    ind_act_impf_1: "~fferēbam", "~fferēbāmus";
    ind_act_impf_2: "~fferēbās", "~fferēbātis";
    ind_act_impf_3: "~fferēbat", "~fferēbant";
    ind_act_fut_1: "~fferam", "~fferēmus";
    ind_act_fut_2: "~fferēs", "~fferētis";
    ind_act_fut_3: "~fferet", "~fferent";
    ind_act_perf_1: "~stulī", "~stulimus";
    ind_act_perf_2: "~stulistī", "~stulistis";
    ind_act_perf_3: "~stulit", "~stulērunt";
    ind_act_plup_1: "~stuleram", "~stulerāmus";
    ind_act_plup_2: "~stulerās", "~stulerātis";
    ind_act_plup_3: "~stulerat", "~stulerant";
    ind_act_futp_1: "~stulerō", "~stulerimus";
    ind_act_futp_2: "~stuleris", "~stuleritis";
    ind_act_futp_3: "~stulerit", "~stulerint";
    ind_pass_pres_1: "~fferor", "~fferimur";
    ind_pass_pres_2: "~fferris|~fferre", "~fferiminī";
    ind_pass_pres_3: "~ffertur", "~fferuntur";
    ind_pass_impf_1: "~fferēbar", "~fferēbāmur";
    ind_pass_impf_2: "~fferēbāris|~fferēbāre", "~fferēbāminī";
    ind_pass_impf_3: "~fferēbātur", "~fferēbantur";
    ind_pass_fut_1: "~fferar", "~fferēmur";
    ind_pass_fut_2: "~fferēris|~fferēre", "~fferēminī";
    ind_pass_fut_3: "~fferētur", "~fferentur";
    subj_act_pres_1: "~fferam", "~fferāmus";
    subj_act_pres_2: "~fferās", "~fferātis";
    subj_act_pres_3: "~fferat", "~fferant";
    subj_act_impf_1: "~fferrem", "~fferrēmus";
    subj_act_impf_2: "~fferrēs", "~fferrētis";
    subj_act_impf_3: "~fferret", "~fferrent";
    subj_act_perf_1: "~stulerim", "~stulerīmus";
    subj_act_perf_2: "~stulerīs", "~stulerītis";
    subj_act_plup_1: "~stulissem", "~stulissēmus";
    subj_act_plup_2: "~stulissēs", "~stulissētis";
    subj_act_plup_3: "~stulisset", "~stulissent";
    subj_pass_pres_1: "~fferar", "~fferāmur";
    subj_pass_pres_2: "~fferāris|~fferāre", "~fferāminī";
    subj_pass_pres_3: "~fferātur", "~fferantur";
    subj_pass_impf_1: "~fferrer", "~fferrēmur";
    subj_pass_impf_2: "~fferrēris|~fferrēre", "~fferrēminī";
    subj_pass_impf_3: "~fferrētur", "~fferrentur";
    imperative_act_pres_2: "~ffer", "~fferte";
    imperative_act_fut_2: "~ffertō", "~ffertōte";
    imperative_act_fut_3: _, "~fferuntō";
    imperative_pass_pres_2: "~fferre", _;
    imperative_pass_fut_2: "~ffertor", _;
    imperative_pass_fut_3: _, "~fferuntor";
    infinitive_act: _, "~stulisse", _;
    infinitive_pass: "~fferrī", _, _;
    participle: "~fferēns", _, _, "~fferendus|~fferundus";
    gerund: _, "~fferendī", "~fferendō", "~fferendum", _, _;
    ),
    // Antecapiō: 3 words (superiaciō, interiaciō, antecapiō). Like Capiō, except in 0 cells: .
    conjugation!("antecapiō", 3, "~a~iō", like Capiō;
    stem participle_perf_pass: "~e~t- like agitō";
    ),
    // Commeminī: 3 words (meminī, commeminī, rememinī). Like Sum, except in 34 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("commeminī", 3, "~i~ī", like Sum as PRES_AS_FUT;
    ind_act_pres_1: "~i~ī", "~i~imus";
    ind_act_pres_2: "~i~istī", "~i~istis";
    ind_act_pres_3: "~i~it", "~i~ērunt|~i~ēre";
    ind_act_impf_1: "~i~eram", "~i~erāmus";
    ind_act_impf_2: "~i~erās", "~i~erātis";
    ind_act_impf_3: "~i~erat", "~i~erant";
    ind_act_fut_1: "~i~erō", "~i~erimus";
    ind_act_fut_2: "~i~eris", "~i~eritis";
    ind_act_fut_3: "~i~erit", "~i~erint";
    subj_act_pres_1: "~i~erim", "~i~erīmus";
    subj_act_pres_2: "~i~erīs", "~i~erītis";
    subj_act_pres_3: "~i~erit", "~i~erint";
    subj_act_impf_1: "~i~issem", "~i~issēmus";
    subj_act_impf_2: "~i~issēs", "~i~issētis";
    subj_act_impf_3: "~i~isset", "~i~issent";
    imperative_act_fut_2: "~e~tō", "~e~tōte";
    infinitive_act: "~i~isse", _, _;
    participle: "~i~ēns", _, _, _;
    ),
    // Contemnō: 3 words (vannō, temnō, contemnō).
    conjugation!("contemnō", 3, "~nō";
    ind_act_pres_1: "~nō", "~nimus";
    ind_act_pres_2: "~nis", "~nitis";
    ind_act_pres_3: "~nit", "~nunt";
    ind_act_impf_1: "~nēbam", "~nēbāmus";
    ind_act_impf_2: "~nēbās", "~nēbātis";
    ind_act_impf_3: "~nēbat", "~nēbant";
    ind_act_fut_1: "~nam", "~nēmus";
    ind_act_fut_2: "~nēs", "~nētis";
    ind_act_fut_3: "~net", "~nent";
    ind_act_perf_1: "~psī|~sī", "~psimus|~simus";
    ind_act_perf_2: "~psistī|~sistī", "~psistis|~sistis";
    ind_act_perf_3: "~psit|~sit", "~psērunt|~psēre|~sērunt|~sēre";
    ind_act_plup_1: "~pseram|~seram", "~pserāmus|~serāmus";
    ind_act_plup_2: "~pserās|~serās", "~pserātis|~serātis";
    ind_act_plup_3: "~pserat|~serat", "~pserant|~serant";
    ind_act_futp_1: "~pserō|~serō", "~pserimus|~serimus";
    ind_act_futp_2: "~pseris|~seris", "~pseritis|~seritis";
    ind_act_futp_3: "~pserit|~serit", "~pserint|~serint";
    ind_pass_pres_1: "~nor", "~nimur";
    ind_pass_pres_2: "~neris|~nere", "~niminī";
    ind_pass_pres_3: "~nitur", "~nuntur";
    ind_pass_impf_1: "~nēbar", "~nēbāmur";
    ind_pass_impf_2: "~nēbāris|~nēbāre", "~nēbāminī";
    ind_pass_impf_3: "~nēbātur", "~nēbantur";
    ind_pass_fut_1: "~nar", "~nēmur";
    ind_pass_fut_2: "~nēris|~nēre", "~nēminī";
    ind_pass_fut_3: "~nētur", "~nentur";
    subj_act_pres_1: "~nam", "~nāmus";
    subj_act_pres_2: "~nās", "~nātis";
    subj_act_pres_3: "~nat", "~nant";
    subj_act_impf_1: "~nerem", "~nerēmus";
    subj_act_impf_2: "~nerēs", "~nerētis";
    subj_act_impf_3: "~neret", "~nerent";
    subj_act_perf_1: "~pserim|~serim", "~pserīmus|~serīmus";
    subj_act_perf_2: "~pserīs|~serīs", "~pserītis|~serītis";
    subj_act_plup_1: "~psissem|~sissem", "~psissēmus|~sissēmus";
    subj_act_plup_2: "~psissēs|~sissēs", "~psissētis|~sissētis";
    subj_act_plup_3: "~psisset|~sisset", "~psissent|~sissent";
    subj_pass_pres_1: "~nar", "~nāmur";
    subj_pass_pres_2: "~nāris|~nāre", "~nāminī";
    subj_pass_pres_3: "~nātur", "~nantur";
    subj_pass_impf_1: "~nerer", "~nerēmur";
    subj_pass_impf_2: "~nerēris|~nerēre", "~nerēminī";
    subj_pass_impf_3: "~nerētur", "~nerentur";
    imperative_act_pres_2: "~ne", "~nite";
    imperative_act_fut_2: "~nitō", "~nitōte";
    imperative_act_fut_3: _, "~nuntō";
    imperative_pass_pres_2: "~nere", _;
    imperative_pass_fut_2: "~nitor", _;
    imperative_pass_fut_3: _, "~nuntor";
    infinitive_act: _, "~psisse|~sisse", _;
    infinitive_pass: "~nī", _, _;
    participle: "~nēns", "~ptus|~tus", "~ptūrus|~tūrus", "~nendus|~nundus";
    gerund: _, "~nendī", "~nendō", "~nendum", _, _;
    supine: _, _, _, "~ptum|~tum", "~ptū|~tū", _;
    ),
    // Dētorreō: 3 words (contorreō, dētorreō, extorreō).
    conjugation!("dētorreō", 3, "~orreō";
    stem participle_perf_pass: "~ŏst- like agitō";
    ind_act_pres_1: "~orreō", "~orrēmus";
    ind_act_pres_2: "~orrēs", "~orrētis";
    ind_act_pres_3: "~orret", "~orrent";
    ind_act_impf_1: "~orrēbam", "~orrēbāmus";
    ind_act_impf_2: "~orrēbās", "~orrēbātis";
    ind_act_impf_3: "~orrēbat", "~orrēbant";
    ind_act_fut_1: "~orrēbō", "~orrēbimus";
    ind_act_fut_2: "~orrēbis", "~orrēbitis";
    ind_act_fut_3: "~orrēbit", "~orrēbunt";
    ind_act_perf_1: "~orruī", "~orruimus";
    ind_act_perf_2: "~orruistī", "~orruistis";
    ind_act_perf_3: "~orruit", "~orruērunt|~orruēre";
    ind_act_plup_1: "~orrueram", "~orruerāmus";
    ind_act_plup_2: "~orruerās", "~orruerātis";
    ind_act_plup_3: "~orruerat", "~orruerant";
    ind_act_futp_1: "~orruerō", "~orruerimus";
    ind_act_futp_2: "~orrueris", "~orrueritis";
    ind_act_futp_3: "~orruerit", "~orruerint";
    ind_pass_pres_1: "~orreor", "~orrēmur";
    ind_pass_pres_2: "~orrēris|~orrēre", "~orrēminī";
    ind_pass_pres_3: "~orrētur", "~orrentur";
    ind_pass_impf_1: "~orrēbar", "~orrēbāmur";
    ind_pass_impf_2: "~orrēbāris|~orrēbāre", "~orrēbāminī";
    ind_pass_impf_3: "~orrēbātur", "~orrēbantur";
    ind_pass_fut_1: "~orrēbor", "~orrēbimur";
    ind_pass_fut_2: "~orrēberis|~orrēbere", "~orrēbiminī";
    ind_pass_fut_3: "~orrēbitur", "~orrēbuntur";
    subj_act_pres_1: "~orream", "~orreāmus";
    subj_act_pres_2: "~orreās", "~orreātis";
    subj_act_pres_3: "~orreat", "~orreant";
    subj_act_impf_1: "~orrērem", "~orrērēmus";
    subj_act_impf_2: "~orrērēs", "~orrērētis";
    subj_act_impf_3: "~orrēret", "~orrērent";
    subj_act_perf_1: "~orruerim", "~orruerīmus";
    subj_act_perf_2: "~orruerīs", "~orruerītis";
    subj_act_plup_1: "~orruissem", "~orruissēmus";
    subj_act_plup_2: "~orruissēs", "~orruissētis";
    subj_act_plup_3: "~orruisset", "~orruissent";
    subj_pass_pres_1: "~orrear", "~orreāmur";
    subj_pass_pres_2: "~orreāris|~orreāre", "~orreāminī";
    subj_pass_pres_3: "~orreātur", "~orreantur";
    subj_pass_impf_1: "~orrērer", "~orrērēmur";
    subj_pass_impf_2: "~orrērēris|~orrērēre", "~orrērēminī";
    subj_pass_impf_3: "~orrērētur", "~orrērentur";
    imperative_act_pres_2: "~orrē", "~orrēte";
    imperative_act_fut_2: "~orrētō", "~orrētōte";
    imperative_act_fut_3: _, "~orrentō";
    imperative_pass_pres_2: "~orrēre", _;
    imperative_pass_fut_2: "~orrētor", _;
    imperative_pass_fut_3: _, "~orrentor";
    infinitive_act: _, "~orruisse", _;
    infinitive_pass: "~orrērī", _, _;
    participle: "~orrēns", _, _, "~orrendus";
    gerund: _, "~orrendī", "~orrendō", "~orrendum", _, _;
    ),
    // Orior: 3 words (morior, orior, ēmorior). Like Exorior, except in 15 cells: ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, subj_act_impf_1 sg, subj_act_impf_1 pl, ….
    conjugation!("orior", 3, "~ior", like Exorior;
    ind_act_pres_1: _, "~ĭmur";
    ind_act_pres_2: "~eris|~īris|~ere|~īre", "~ĭminī";
    ind_act_pres_3: "~ĭtur", _;
    subj_act_impf_1: "~erer|~īrer", "~erēmur|~īrēmur";
    subj_act_impf_2: "~erēris|~īrēris|~erēre|~īrēre", "~erēminī|~īrēminī";
    subj_act_impf_3: "~erētur|~īrētur", "~erentur|~īrentur";
    imperative_act_pres_2: "~ere|~īre", "~ĭminī";
    imperative_act_fut_2: "~ĭtor", _;
    infinitive_act: "~ī|~īrī", _, _;
    participle: _, _, _, "~iendus";
    ),
    // Superstō: 3 words (superstō, circumstō, antistō). Like Cōnstō, except in 32 cells: ind_act_perf_1 sg, ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ….
    conjugation!("superstō", 3, "~ō", like Cōnstō as ABL_AS_VOC;
    ind_act_perf_1: "~etī", "~etimus";
    ind_act_perf_2: "~etistī", "~etistis";
    ind_act_perf_3: "~etit", "~etērunt|~etēre";
    ind_act_plup_1: "~eteram", "~eterāmus";
    ind_act_plup_2: "~eterās", "~eterātis";
    ind_act_plup_3: "~eterat", "~eterant";
    ind_act_futp_1: "~eterō", "~eterimus";
    ind_act_futp_2: "~eteris", "~eteritis";
    ind_act_futp_3: "~eterit", "~eterint";
    subj_act_perf_1: "~eterim", "~eterīmus";
    subj_act_perf_2: "~eterīs", "~eterītis";
    subj_act_plup_1: "~etissem", "~etissēmus";
    subj_act_plup_2: "~etissēs", "~etissētis";
    subj_act_plup_3: "~etisset", "~etissent";
    infinitive_act: _, "~etisse", _;
    participle: _, derived, derived, _;
    supine: _, _, _, derived, _, _;
    ),
    // Gignō: 3 words (gignō, ingignō, prōgignō).
    conjugation!("gignō", 3, "~ig~ō";
    stem participle_perf_pass: "~e~it- like agitō";
    ind_act_pres_1: "~ig~ō", "~ig~imus";
    ind_act_pres_2: "~ig~is", "~ig~itis";
    ind_act_pres_3: "~ig~it", "~ig~unt";
    ind_act_impf_1: "~ig~ēbam", "~ig~ēbāmus";
    ind_act_impf_2: "~ig~ēbās", "~ig~ēbātis";
    ind_act_impf_3: "~ig~ēbat", "~ig~ēbant";
    ind_act_fut_1: "~ig~am", "~ig~ēmus";
    ind_act_fut_2: "~ig~ēs", "~ig~ētis";
    ind_act_fut_3: "~ig~et", "~ig~ent";
    ind_act_perf_1: "~e~uī", "~e~uimus";
    ind_act_perf_2: "~e~uistī", "~e~uistis";
    ind_act_perf_3: "~e~uit", "~e~uērunt|~e~uēre";
    ind_act_plup_1: "~e~ueram", "~e~uerāmus";
    ind_act_plup_2: "~e~uerās", "~e~uerātis";
    ind_act_plup_3: "~e~uerat", "~e~uerant";
    ind_act_futp_1: "~e~uerō", "~e~uerimus";
    ind_act_futp_2: "~e~ueris", "~e~ueritis";
    ind_act_futp_3: "~e~uerit", "~e~uerint";
    ind_pass_pres_1: "~ig~or", "~ig~imur";
    ind_pass_pres_2: "~ig~eris|~ig~ere", "~ig~iminī";
    ind_pass_pres_3: "~ig~itur", "~ig~untur";
    ind_pass_impf_1: "~ig~ēbar", "~ig~ēbāmur";
    ind_pass_impf_2: "~ig~ēbāris|~ig~ēbāre", "~ig~ēbāminī";
    ind_pass_impf_3: "~ig~ēbātur", "~ig~ēbantur";
    ind_pass_fut_1: "~ig~ar", "~ig~ēmur";
    ind_pass_fut_2: "~ig~ēris|~ig~ēre", "~ig~ēminī";
    ind_pass_fut_3: "~ig~ētur", "~ig~entur";
    subj_act_pres_1: "~ig~am", "~ig~āmus";
    subj_act_pres_2: "~ig~ās", "~ig~ātis";
    subj_act_pres_3: "~ig~at", "~ig~ant";
    subj_act_impf_1: "~ig~erem", "~ig~erēmus";
    subj_act_impf_2: "~ig~erēs", "~ig~erētis";
    subj_act_impf_3: "~ig~eret", "~ig~erent";
    subj_act_perf_1: "~e~uerim", "~e~uerīmus";
    subj_act_perf_2: "~e~uerīs", "~e~uerītis";
    subj_act_plup_1: "~e~uissem", "~e~uissēmus";
    subj_act_plup_2: "~e~uissēs", "~e~uissētis";
    subj_act_plup_3: "~e~uisset", "~e~uissent";
    subj_pass_pres_1: "~ig~ar", "~ig~āmur";
    subj_pass_pres_2: "~ig~āris|~ig~āre", "~ig~āminī";
    subj_pass_pres_3: "~ig~ātur", "~ig~antur";
    subj_pass_impf_1: "~ig~erer", "~ig~erēmur";
    subj_pass_impf_2: "~ig~erēris|~ig~erēre", "~ig~erēminī";
    subj_pass_impf_3: "~ig~erētur", "~ig~erentur";
    imperative_act_pres_2: "~ig~e", "~ig~ite";
    imperative_act_fut_2: "~ig~itō", "~ig~itōte";
    imperative_act_fut_3: _, "~ig~untō";
    imperative_pass_pres_2: "~ig~ere", _;
    imperative_pass_fut_2: "~ig~itor", _;
    imperative_pass_fut_3: _, "~ig~untor";
    infinitive_act: _, "~e~uisse", _;
    infinitive_pass: "~ig~ī", _, _;
    participle: "~ig~ēns", _, _, "~ig~endus|~ig~undus";
    gerund: _, "~ig~endī", "~ig~endō", "~ig~endum", _, _;
    ),
    // Assiliō: 3 words (assiliō, prōsiliō, resiliō). Like Īnsiliō, except in 23 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("assiliō", 3, "~i~iō", like Īnsiliō;
    stem verb_act_sg: "~i~- like ēveniō";
    stem participle_perf_pass: "~u~t- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    participle: _, _, _, "~i~iendum|~i~iundum";
    ),
    // Convalēscō: 3 words (convalēscō, coalēscō, tonēscō). Like Madeō, except in 32 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("convalēscō", 3, "~ēscō", like Madeō as IMPF_AS_FUTP;
    stem verb_act_sg: "~ēsc- like convalēscō";
    ind_act_pres_1: _, "~ēscimus";
    ind_act_impf_1: "~ēscēbam", "~ēscēbāmus";
    ind_act_impf_2: "~ēscēbās", "~ēscēbātis";
    ind_act_impf_3: "~ēscēbat", "~ēscēbant";
    ind_act_fut_1: "~ēscam", "~ēscēmus";
    ind_act_fut_2: "~ēscēs", "~ēscētis";
    ind_act_fut_3: "~ēscet", _;
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_3: derived, _;
    ind_pass_fut_3: derived, derived;
    subj_act_pres_3: "~ēscat", _;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_act_fut_2: "~ēscitō", _;
    imperative_pass_fut_3: derived, derived;
    infinitive_pass: derived, _, _;
    participle: "~ēscēns", derived, _, derived;
    gerund: _, "~ēscendī", "~ēscendō", "~ēscendum", _, _;
    ),
    // Adhērēscō: 3 words (adhērēscō, adhaerēscō, cohaerēscō). Like Sum, except in 47 cells: ind_act_pres_1 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ind_act_impf_3 sg, ….
    conjugation!("adhērēscō", 3, "~rē~cō", like Sum;
    stem verb_act_sg: "~rē~c- like convalēscō";
    stem participle_perf_pass: "~~- like modulor";
    ind_act_pres_1: _, "~rē~cimus";
    ind_act_impf_1: "~rē~cēbam", "~rē~cēbāmus";
    ind_act_impf_2: "~rē~cēbās", "~rē~cēbātis";
    ind_act_impf_3: "~rē~cēbat", "~rē~cēbant";
    ind_act_fut_1: "~rē~cam", "~rē~cēmus";
    ind_act_fut_2: "~rē~cēs", "~rē~cētis";
    ind_act_fut_3: "~rē~cet", _;
    ind_act_perf_1: "~~ī", "~~imus";
    ind_act_perf_2: "~~istī", "~~istis";
    ind_act_perf_3: "~~it", "~~ērunt|~~ēre";
    ind_act_plup_1: "~~eram", "~~erāmus";
    ind_act_plup_2: "~~erās", "~~erātis";
    ind_act_plup_3: "~~erat", "~~erant";
    ind_act_futp_1: "~~erō", "~~erimus";
    ind_act_futp_2: "~~eris", "~~eritis";
    ind_act_futp_3: "~~erit", "~~erint";
    subj_act_pres_3: "~rē~cat", _;
    subj_act_perf_1: "~~erim", "~~erīmus";
    subj_act_perf_2: "~~erīs", "~~erītis";
    subj_act_plup_1: "~~issem", "~~issēmus";
    subj_act_plup_2: "~~issēs", "~~issētis";
    subj_act_plup_3: "~~isset", "~~issent";
    imperative_act_fut_2: "~rē~citō", _;
    infinitive_act: _, "~~isse", _;
    participle: "~rē~cēns", _, _, _;
    gerund: _, "~rē~cendī", "~rē~cendō", "~rē~cendum", _, _;
    ),
    // Lavō: 3 words (lavō, perlavō, adlavō).
    conjugation!("lavō", 3, "~a~ō";
    stem participle_perf_pass: "~a~āt- like agitō";
    ind_act_pres_1: "~a~ō", "~a~āmus";
    ind_act_pres_2: "~a~ās", "~a~ātis";
    ind_act_pres_3: "~a~at", "~a~ant";
    ind_act_impf_1: "~a~ābam", "~a~ābāmus";
    ind_act_impf_2: "~a~ābās", "~a~ābātis";
    ind_act_impf_3: "~a~ābat", "~a~ābant";
    ind_act_fut_1: "~a~ābō", "~a~ābimus";
    ind_act_fut_2: "~a~ābis", "~a~ābitis";
    ind_act_fut_3: "~a~ābit", "~a~ābunt";
    ind_act_perf_1: "~ā~ī", "~ā~imus";
    ind_act_perf_2: "~ā~istī", "~ā~istis";
    ind_act_perf_3: "~ā~it", "~ā~ērunt|~ā~ēre";
    ind_act_plup_1: "~ā~eram", "~ā~erāmus";
    ind_act_plup_2: "~ā~erās", "~ā~erātis";
    ind_act_plup_3: "~ā~erat", "~ā~erant";
    ind_act_futp_1: "~ā~erō", "~ā~erimus";
    ind_act_futp_2: "~ā~eris", "~ā~eritis";
    ind_act_futp_3: "~ā~erit", "~ā~erint";
    ind_pass_pres_1: "~a~or", "~a~āmur";
    ind_pass_pres_2: "~a~āris|~a~āre", "~a~āminī";
    ind_pass_pres_3: "~a~ātur", "~a~antur";
    ind_pass_impf_1: "~a~ābar", "~a~ābāmur";
    ind_pass_impf_2: "~a~ābāris|~a~ābāre", "~a~ābāminī";
    ind_pass_impf_3: "~a~ābātur", "~a~ābantur";
    ind_pass_fut_1: "~a~ābor", "~a~ābimur";
    ind_pass_fut_2: "~a~āberis|~a~ābere", "~a~ābiminī";
    ind_pass_fut_3: "~a~ābitur", "~a~ābuntur";
    subj_act_pres_1: "~a~em", "~a~ēmus";
    subj_act_pres_2: "~a~ēs", "~a~ētis";
    subj_act_pres_3: "~a~et", "~a~ent";
    subj_act_impf_1: "~a~ārem", "~a~ārēmus";
    subj_act_impf_2: "~a~ārēs", "~a~ārētis";
    subj_act_impf_3: "~a~āret", "~a~ārent";
    subj_act_perf_1: "~ā~erim", "~ā~erīmus";
    subj_act_perf_2: "~ā~erīs", "~ā~erītis";
    subj_act_plup_1: "~ā~issem", "~ā~issēmus";
    subj_act_plup_2: "~ā~issēs", "~ā~issētis";
    subj_act_plup_3: "~ā~isset", "~ā~issent";
    subj_pass_pres_1: "~a~er", "~a~ēmur";
    subj_pass_pres_2: "~a~ēris|~a~ēre", "~a~ēminī";
    subj_pass_pres_3: "~a~ētur", "~a~entur";
    subj_pass_impf_1: "~a~ārer", "~a~ārēmur";
    subj_pass_impf_2: "~a~ārēris|~a~ārēre", "~a~ārēminī";
    subj_pass_impf_3: "~a~ārētur", "~a~ārentur";
    imperative_act_pres_2: "~a~ā", "~a~āte";
    imperative_act_fut_2: "~a~ātō", "~a~ātōte";
    imperative_act_fut_3: _, "~a~antō";
    imperative_pass_pres_2: "~a~āre", _;
    imperative_pass_fut_2: "~a~ātor", _;
    imperative_pass_fut_3: _, "~a~antor";
    infinitive_act: _, "~ā~isse", _;
    infinitive_pass: "~a~ārī", _, _;
    participle: "~a~āns", _, _, "~a~andus";
    gerund: _, "~a~andī", "~a~andō", "~a~andum", _, _;
    ),
    // Abblandior: 3 words (ēblandior, abblandior, adblandior). Like Mōlior, except in 0 cells: .
    conjugation!("abblandior", 3, "~ior", like Mōlior;
    stem participle_perf_pass: "~it- like modulor";
    ),
    // Effervēscō: 3 words (effervēscō, dēfervēscō, cōnfervēscō).
    conjugation!("effervēscō", 3, "~vēscō";
    ind_act_pres_1: "~vēscō", "~vēscimus";
    ind_act_pres_2: "~vēscis", "~vēscitis";
    ind_act_pres_3: "~vēscit", "~vēscunt";
    ind_act_impf_1: "~vēscēbam", "~vēscēbāmus";
    ind_act_impf_2: "~vēscēbās", "~vēscēbātis";
    ind_act_impf_3: "~vēscēbat", "~vēscēbant";
    ind_act_fut_1: "~vēscam", "~vēscēmus";
    ind_act_fut_2: "~vēscēs", "~vēscētis";
    ind_act_fut_3: "~vēscet", "~vēscent";
    ind_act_perf_1: "~buī|~vī", "~buimus|~vimus";
    ind_act_perf_2: "~buistī|~vistī", "~buistis|~vistis";
    ind_act_perf_3: "~buit|~vit", "~buērunt|~buēre|~vērunt|~vēre";
    ind_act_plup_1: "~bueram|~veram", "~buerāmus|~verāmus";
    ind_act_plup_2: "~buerās|~verās", "~buerātis|~verātis";
    ind_act_plup_3: "~buerat|~verat", "~buerant|~verant";
    ind_act_futp_1: "~buerō|~verō", "~buerimus|~verimus";
    ind_act_futp_2: "~bueris|~veris", "~bueritis|~veritis";
    ind_act_futp_3: "~buerit|~verit", "~buerint|~verint";
    ind_pass_pres_1: "~vēscor", "~vēscimur";
    ind_pass_pres_2: "~vēsceris", "~vēsciminī";
    ind_pass_pres_3: "~vēscitur", "~vēscuntur";
    ind_pass_impf_1: "~vēscēbar", "~vēscēbāmur";
    ind_pass_impf_2: "~vēscēbāris", "~vēscēbāminī";
    ind_pass_impf_3: "~vēscēbātur", "~vēscēbantur";
    ind_pass_fut_1: "~vēscar", "~vēscēmur";
    ind_pass_fut_2: "~vēscēris", "~vēscēminī";
    ind_pass_fut_3: "~vēscētur", "~vēscentur";
    subj_act_pres_1: "~vēscam", "~vēscāmus";
    subj_act_pres_2: "~vēscās", "~vēscātis";
    subj_act_pres_3: "~vēscat", "~vēscant";
    subj_act_impf_1: "~vēscerem", "~vēscerēmus";
    subj_act_impf_2: "~vēscerēs", "~vēscerētis";
    subj_act_impf_3: "~vēsceret", "~vēscerent";
    subj_act_perf_1: "~buerim|~verim", "~buerīmus|~verīmus";
    subj_act_perf_2: "~buerīs|~verīs", "~buerītis|~verītis";
    subj_act_plup_1: "~buissem|~vissem", "~buissēmus|~vissēmus";
    subj_act_plup_2: "~buissēs|~vissēs", "~buissētis|~vissētis";
    subj_act_plup_3: "~buisset|~visset", "~buissent|~vissent";
    subj_pass_pres_1: "~vēscar", "~vēscāmur";
    subj_pass_pres_2: "~vēscāris", "~vēscāminī";
    subj_pass_pres_3: "~vēscātur", "~vēscantur";
    subj_pass_impf_1: "~vēscerer", "~vēscerēmur";
    subj_pass_impf_2: "~vēscerēris", "~vēscerēminī";
    subj_pass_impf_3: "~vēscerētur", "~vēscerentur";
    imperative_act_pres_2: "~vēsce", "~vēscite";
    imperative_act_fut_2: "~vēscitō", "~vēscitōte";
    imperative_act_fut_3: _, "~vēscuntō";
    imperative_pass_pres_2: "~vēscere", _;
    imperative_pass_fut_2: "~vēscitor", _;
    imperative_pass_fut_3: _, "~vēscuntor";
    infinitive_act: _, "~buisse|~visse", _;
    infinitive_pass: "~vēscī", _, _;
    participle: "~vēscēns", _, _, "~vēscendus";
    gerund: _, "~vēscendī", "~vēscendō", "~vēscendum", _, _;
    ),
    // Dīlūculat: 2 words (grandinat, dīlūculat). Like Modulor, except in 17 cells: ind_act_pres_1 sg, ind_act_pres_3 sg, ind_act_impf_3 sg, ind_act_fut_3 sg, ind_act_perf_3 sg, ind_act_plup_3 sg, ….
    conjugation!("dīlūculat", 2, "~at", like Modulor as ACT_AS_PASS;
    ind_act_pres_1: "~at", _;
    ind_act_pres_3: "~at", _;
    ind_act_impf_3: "~ābat", _;
    ind_act_fut_3: "~ābit", _;
    ind_act_perf_3: "~āvit", _;
    ind_act_plup_3: "~āverat", _;
    ind_act_futp_3: "~āverit", _;
    subj_act_pres_3: "~et", _;
    subj_act_impf_3: "~āret", _;
    subj_act_plup_3: "~āvisset", _;
    imperative_act_fut_3: "~ātō", _;
    infinitive_act: "~āre", "~āvisse", _;
    participle: _, _, derived, derived;
    supine: _, _, _, derived, derived, _;
    ),
    // Sepultō: 1 words (sepultō).
    conjugation!("sepultō", 1, "~~ō";
    stem participle_perf_pass: "~t~āt- like agitō";
    ind_act_pres_1: "~t~ō", "~t~āmus";
    ind_act_pres_2: "~t~ās", "~t~ātis";
    ind_act_pres_3: "~t~at", "~t~ant";
    ind_act_impf_1: "~t~ābam", "~t~ābāmus";
    ind_act_impf_2: "~t~ābās", "~t~ābātis";
    ind_act_impf_3: "~t~ābat", "~t~ābant";
    ind_act_fut_1: "~t~ābō", "~t~ābimus";
    ind_act_fut_2: "~t~ābis", "~t~ābitis";
    ind_act_fut_3: "~t~ābit", "~t~ābunt";
    ind_act_perf_1: "~t~āvī", "~t~āvimus";
    ind_act_perf_2: "~t~āvistī", "~t~āvistis";
    ind_act_perf_3: "~t~āvit", "~t~āvērunt|~t~āvēre";
    ind_act_plup_1: "~t~āveram", "~t~āverāmus";
    ind_act_plup_2: "~t~āverās", "~t~āverātis";
    ind_act_plup_3: "~t~āverat", "~t~āverant";
    ind_act_futp_1: "~t~āverō", "~t~āverimus";
    ind_act_futp_2: "~t~āveris", "~t~āveritis";
    ind_act_futp_3: "~t~āverit", "~t~āverint";
    ind_pass_pres_1: "~t~or", "~t~āmur";
    ind_pass_pres_2: "~t~āris|~t~āre", "~t~āminī";
    ind_pass_pres_3: "~t~ātur", "~t~antur";
    ind_pass_impf_1: "~t~ābar", "~t~ābāmur";
    ind_pass_impf_2: "~t~ābāris|~t~ābāre", "~t~ābāminī";
    ind_pass_impf_3: "~t~ābātur", "~t~ābantur";
    ind_pass_fut_1: "~t~ābor", "~t~ābimur";
    ind_pass_fut_2: "~t~āberis|~t~ābere", "~t~ābiminī";
    ind_pass_fut_3: "~t~ābitur", "~t~ābuntur";
    subj_act_pres_1: "~t~em", "~t~ēmus";
    subj_act_pres_2: "~t~ēs", "~t~ētis";
    subj_act_pres_3: "~t~et", "~t~ent";
    subj_act_impf_1: "~t~ārem", "~t~ārēmus";
    subj_act_impf_2: "~t~ārēs", "~t~ārētis";
    subj_act_impf_3: "~t~āret", "~t~ārent";
    subj_act_perf_1: "~t~āverim", "~t~āverīmus";
    subj_act_perf_2: "~t~āverīs", "~t~āverītis";
    subj_act_plup_1: "~t~āvissem", "~t~āvissēmus";
    subj_act_plup_2: "~t~āvissēs", "~t~āvissētis";
    subj_act_plup_3: "~t~āvisset", "~t~āvissent";
    subj_pass_pres_1: "~t~er", "~t~ēmur";
    subj_pass_pres_2: "~t~ēris|~t~ēre", "~t~ēminī";
    subj_pass_pres_3: "~t~ētur", "~t~entur";
    subj_pass_impf_1: "~t~ārer", "~t~ārēmur";
    subj_pass_impf_2: "~t~ārēris|~t~ārēre", "~t~ārēminī";
    subj_pass_impf_3: "~t~ārētur", "~t~ārentur";
    imperative_act_pres_2: "~t~ā", "~t~āte";
    imperative_act_fut_2: "~t~ātō", "~t~ātōte";
    imperative_act_fut_3: _, "~t~antō";
    imperative_pass_pres_2: "~t~āre", _;
    imperative_pass_fut_2: "~t~ātor", _;
    imperative_pass_fut_3: _, "~t~antor";
    infinitive_act: _, "~t~āvisse", _;
    infinitive_pass: "~t~ārī", _, _;
    participle: "~t~āns", _, _, "~t~andus";
    gerund: _, "~t~andī", "~t~andō", "~t~andum", _, _;
    ),
    // Alāre: 1 words (alāre). Like Agitō, except in 28 cells: ind_act_perf_1 pl, ind_act_perf_2 sg, ind_act_perf_2 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ind_act_plup_1 sg, ….
    conjugation!("alāre", 1, "~āre", like Agitō;
    ind_act_perf_1: _, "~āvimus|~āmus";
    ind_act_perf_2: "~āvistī|~āstī", "~āvistis|~āstis";
    ind_act_perf_3: "~āvit|~āt", "~āvērunt|~ārunt|~āvēre";
    ind_act_plup_1: "~āveram|~āram", "~āverāmus|~ārāmus";
    ind_act_plup_2: "~āverās|~ārās", "~āverātis|~ārātis";
    ind_act_plup_3: "~āverat|~ārat", "~āverant|~ārant";
    ind_act_futp_1: "~āverō|~ārō", "~āverimus|~ārimus";
    ind_act_futp_2: "~āveris|~āris", "~āveritis|~āritis";
    ind_act_futp_3: "~āverit|~ārit", "~āverint|~ārint";
    subj_act_perf_1: "~āverim|~ārim", "~āverīmus|~ārīmus";
    subj_act_perf_2: "~āverīs|~ārīs", "~āverītis|~ārītis";
    subj_act_plup_1: "~āvissem|~āssem", "~āvissēmus|~āssēmus";
    subj_act_plup_2: "~āvissēs|~āssēs", "~āvissētis|~āssētis";
    subj_act_plup_3: "~āvisset|~āsset", "~āvissent|~āssent";
    infinitive_act: _, "~āvisse|~āsse", _;
    ),
    // Coerceo: 1 words (coerceo).
    conjugation!("coerceo", 1, "~e~eo";
    stem participle_perf_pass: "~ë~it- like agitō";
    ind_act_pres_1: "~ë~eō", "~ë~ēmus";
    ind_act_pres_2: "~ë~ēs", "~ë~ētis";
    ind_act_pres_3: "~ë~et", "~ë~ent";
    ind_act_impf_1: "~ë~ēbam", "~ë~ēbāmus";
    ind_act_impf_2: "~ë~ēbās", "~ë~ēbātis";
    ind_act_impf_3: "~ë~ēbat", "~ë~ēbant";
    ind_act_fut_1: "~ë~ēbō", "~ë~ēbimus";
    ind_act_fut_2: "~ë~ēbis", "~ë~ēbitis";
    ind_act_fut_3: "~ë~ēbit", "~ë~ēbunt";
    ind_act_perf_1: "~ë~uī", "~ë~uimus";
    ind_act_perf_2: "~ë~uistī", "~ë~uistis";
    ind_act_perf_3: "~ë~uit", "~ë~uērunt|~ë~uēre";
    ind_act_plup_1: "~ë~ueram", "~ë~uerāmus";
    ind_act_plup_2: "~ë~uerās", "~ë~uerātis";
    ind_act_plup_3: "~ë~uerat", "~ë~uerant";
    ind_act_futp_1: "~ë~uerō", "~ë~uerimus";
    ind_act_futp_2: "~ë~ueris", "~ë~ueritis";
    ind_act_futp_3: "~ë~uerit", "~ë~uerint";
    ind_pass_pres_1: "~ë~eor", "~ë~ēmur";
    ind_pass_pres_2: "~ë~ēris|~ë~ēre", "~ë~ēminī";
    ind_pass_pres_3: "~ë~ētur", "~ë~entur";
    ind_pass_impf_1: "~ë~ēbar", "~ë~ēbāmur";
    ind_pass_impf_2: "~ë~ēbāris|~ë~ēbāre", "~ë~ēbāminī";
    ind_pass_impf_3: "~ë~ēbātur", "~ë~ēbantur";
    ind_pass_fut_1: "~ë~ēbor", "~ë~ēbimur";
    ind_pass_fut_2: "~ë~ēberis|~ë~ēbere", "~ë~ēbiminī";
    ind_pass_fut_3: "~ë~ēbitur", "~ë~ēbuntur";
    subj_act_pres_1: "~ë~eam", "~ë~eāmus";
    subj_act_pres_2: "~ë~eās", "~ë~eātis";
    subj_act_pres_3: "~ë~eat", "~ë~eant";
    subj_act_impf_1: "~ë~ērem", "~ë~ērēmus";
    subj_act_impf_2: "~ë~ērēs", "~ë~ērētis";
    subj_act_impf_3: "~ë~ēret", "~ë~ērent";
    subj_act_perf_1: "~ë~uerim", "~ë~uerīmus";
    subj_act_perf_2: "~ë~uerīs", "~ë~uerītis";
    subj_act_plup_1: "~ë~uissem", "~ë~uissēmus";
    subj_act_plup_2: "~ë~uissēs", "~ë~uissētis";
    subj_act_plup_3: "~ë~uisset", "~ë~uissent";
    subj_pass_pres_1: "~ë~ear", "~ë~eāmur";
    subj_pass_pres_2: "~ë~eāris|~ë~eāre", "~ë~eāminī";
    subj_pass_pres_3: "~ë~eātur", "~ë~eantur";
    subj_pass_impf_1: "~ë~ērer", "~ë~ērēmur";
    subj_pass_impf_2: "~ë~ērēris|~ë~ērēre", "~ë~ērēminī";
    subj_pass_impf_3: "~ë~ērētur", "~ë~ērentur";
    imperative_act_pres_2: "~ë~ē", "~ë~ēte";
    imperative_act_fut_2: "~ë~ētō", "~ë~ētōte";
    imperative_act_fut_3: _, "~ë~entō";
    imperative_pass_pres_2: "~ë~ēre", _;
    imperative_pass_fut_2: "~ë~ētor", _;
    imperative_pass_fut_3: _, "~ë~entor";
    infinitive_act: _, "~ë~uisse", _;
    infinitive_pass: "~ë~ērī", _, _;
    participle: "~ë~ēns", _, _, "~ë~endus";
    gerund: _, "~ë~endī", "~ë~endō", "~ë~endum", _, _;
    ),
    // Rēfert: 1 words (rēfert). Like Dīlūculat, except in 17 cells: ind_act_pres_1 sg, ind_act_pres_3 sg, ind_act_impf_3 sg, ind_act_fut_3 sg, ind_act_perf_3 sg, ind_act_plup_3 sg, ….
    conjugation!("rēfert", 1, "~fert", like Dīlūculat;
    ind_act_pres_1: "~fert", _;
    ind_act_pres_3: "~fert", _;
    ind_act_impf_3: "~ferēbat", _;
    ind_act_fut_3: "~feret", _;
    ind_act_perf_3: "~tulit", _;
    ind_act_plup_3: "~tulerat", _;
    ind_act_futp_3: "~tulerit", _;
    subj_act_pres_3: "~ferat", _;
    subj_act_impf_3: "~ferret", _;
    subj_act_plup_3: "~tulisset", _;
    imperative_act_fut_3: "~fertō", _;
    infinitive_act: "~ferre", "~tulisse", _;
    participle: "~ferēns", _, _, _;
    gerund: _, "~ferendī", "~ferendō", "~ferendum", _, _;
    ),
    // Coemo: 1 words (coemo). Like Eximō, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("coemo", 1, "~e~o", like Eximō;
    ind_act_pres_1: "~ë~ō", "~ë~imus";
    ind_act_pres_2: "~ë~is", "~ë~itis";
    ind_act_pres_3: "~ë~it", "~ë~unt";
    ind_act_impf_1: "~ë~ēbam", "~ë~ēbāmus";
    ind_act_impf_2: "~ë~ēbās", "~ë~ēbātis";
    ind_act_impf_3: "~ë~ēbat", "~ë~ēbant";
    ind_act_fut_1: "~ë~am", "~ë~ēmus";
    ind_act_fut_2: "~ë~ēs", "~ë~ētis";
    ind_act_fut_3: "~ë~et", "~ë~ent";
    ind_pass_pres_1: "~ë~or", "~ë~imur";
    ind_pass_pres_2: "~ë~eris|~ë~ere", "~ë~iminī";
    ind_pass_pres_3: "~ë~itur", "~ë~untur";
    ind_pass_impf_1: "~ë~ēbar", "~ë~ēbāmur";
    ind_pass_impf_2: "~ë~ēbāris|~ë~ēbāre", "~ë~ēbāminī";
    ind_pass_impf_3: "~ë~ēbātur", "~ë~ēbantur";
    ind_pass_fut_1: "~ë~ar", "~ë~ēmur";
    ind_pass_fut_2: "~ë~ēris|~ë~ēre", "~ë~ēminī";
    ind_pass_fut_3: "~ë~ētur", "~ë~entur";
    subj_act_pres_1: "~ë~am", "~ë~āmus";
    subj_act_pres_2: "~ë~ās", "~ë~ātis";
    subj_act_pres_3: "~ë~at", "~ë~ant";
    subj_act_impf_1: "~ë~erem", "~ë~erēmus";
    subj_act_impf_2: "~ë~erēs", "~ë~erētis";
    subj_act_impf_3: "~ë~eret", "~ë~erent";
    subj_pass_pres_1: "~ë~ar", "~ë~āmur";
    subj_pass_pres_2: "~ë~āris|~ë~āre", "~ë~āminī";
    subj_pass_pres_3: "~ë~ātur", "~ë~antur";
    subj_pass_impf_1: "~ë~erer", "~ë~erēmur";
    subj_pass_impf_2: "~ë~erēris|~ë~erēre", "~ë~erēminī";
    subj_pass_impf_3: "~ë~erētur", "~ë~erentur";
    imperative_act_pres_2: "~ë~e", "~ë~ite";
    imperative_act_fut_2: "~ë~itō", "~ë~itōte";
    imperative_act_fut_3: _, "~ë~untō";
    imperative_pass_pres_2: "~ë~ere", _;
    imperative_pass_fut_2: "~ë~itor", _;
    imperative_pass_fut_3: _, "~ë~untor";
    infinitive_pass: "~ë~ī", _, _;
    participle: "~ë~ēns", _, _, "~ë~endus|~ë~undus";
    gerund: _, "~ë~endī", "~ë~endō", "~ë~endum", _, _;
    ),
    // Inclāmitō: 1 words (inclāmitō). Like Agitō, except in 0 cells: .
    conjugation!("inclāmitō", 1, "in~ō", like Agitō;
    ),
];
