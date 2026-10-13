//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`A`, `B`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

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
/// `ind_act_1` (sg, du, pl)
/// `ind_act_2` (sg, du, pl)
/// `ind_act_3` (sg, du, pl)
/// `ind_mid_1` (sg, du, pl)
/// `ind_mid_2` (sg, du, pl)
/// `ind_mid_3` (sg, du, pl)
/// `imp_act_1` (sg, du, pl)
/// `imp_act_2` (sg, du, pl)
/// `imp_act_3` (sg, du, pl)
/// `imp_mid_1` (sg, du, pl)
/// `imp_mid_2` (sg, du, pl)
/// `imp_mid_3` (sg, du, pl)
/// `opt_act_1` (sg, du, pl)
/// `opt_act_2` (sg, du, pl)
/// `opt_act_3` (sg, du, pl)
/// `opt_mid_1` (sg, du, pl)
/// `opt_mid_2` (sg, du, pl)
/// `opt_mid_3` (sg, du, pl)
/// `subj_act_1` (sg, du, pl)
/// `subj_act_2` (sg, du, pl)
/// `subj_act_3` (sg, du, pl)
/// `subj_mid_1` (sg, du, pl)
/// `subj_mid_2` (sg, du, pl)
/// `subj_mid_3` (sg, du, pl)
/// `imperfect_act_1` (sg, du, pl)
/// `imperfect_act_2` (sg, du, pl)
/// `imperfect_act_3` (sg, du, pl)
/// `imperfect_mid_1` (sg, du, pl)
/// `imperfect_mid_2` (sg, du, pl)
/// `imperfect_mid_3` (sg, du, pl)
/// `future_act_1` (sg, du, pl)
/// `future_act_2` (sg, du, pl)
/// `future_act_3` (sg, du, pl)
/// `future_mid_1` (sg, du, pl)
/// `future_mid_2` (sg, du, pl)
/// `future_mid_3` (sg, du, pl)
/// `conditional_act_1` (sg, du, pl)
/// `conditional_act_2` (sg, du, pl)
/// `conditional_act_3` (sg, du, pl)
/// `conditional_mid_1` (sg, du, pl)
/// `conditional_mid_2` (sg, du, pl)
/// `conditional_mid_3` (sg, du, pl)
/// `perfect_act_1` (sg, du, pl)
/// `perfect_act_2` (sg, du, pl)
/// `perfect_act_3` (sg, du, pl)
/// `perfect_mid_1` (sg, du, pl)
/// `perfect_mid_2` (sg, du, pl)
/// `perfect_mid_3` (sg, du, pl)
/// `aorist_act_1` (sg, du, pl)
/// `aorist_act_2` (sg, du, pl)
/// `aorist_act_3` (sg, du, pl)
/// `aorist_mid_1` (sg, du, pl)
/// `aorist_mid_2` (sg, du, pl)
/// `aorist_mid_3` (sg, du, pl)
/// `infinitive`
/// `gerund`
/// `participle_pres` (act, mid)
/// `participle_fut` (act, mid)
/// `participle_perf` (act, mid)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem present_ind_act_1_sg: $s0:tt;)?
     $(stem present_act: $s1:tt;)?
     $(stem present_2: $s2:tt;)?
     $(stem present_ind_act_2_pl: $s3:tt;)?
     $(stem act_sg: $s4:tt;)?
     $(stem present_mid: $s5:tt;)?
     $(stem present: $s6:tt;)?
     $(stem imperfect_act_1_sg: $s7:tt;)?
     $(stem imperfect: $s8:tt;)?
     $(ind_act_1: $c0:tt, $c1:tt, $c2:tt;)?
     $(ind_act_2: $c3:tt, $c4:tt, $c5:tt;)?
     $(ind_act_3: $c6:tt, $c7:tt, $c8:tt;)?
     $(ind_mid_1: $c9:tt, $c10:tt, $c11:tt;)?
     $(ind_mid_2: $c12:tt, $c13:tt, $c14:tt;)?
     $(ind_mid_3: $c15:tt, $c16:tt, $c17:tt;)?
     $(imp_act_1: $c18:tt, $c19:tt, $c20:tt;)?
     $(imp_act_2: $c21:tt, $c22:tt, $c23:tt;)?
     $(imp_act_3: $c24:tt, $c25:tt, $c26:tt;)?
     $(imp_mid_1: $c27:tt, $c28:tt, $c29:tt;)?
     $(imp_mid_2: $c30:tt, $c31:tt, $c32:tt;)?
     $(imp_mid_3: $c33:tt, $c34:tt, $c35:tt;)?
     $(opt_act_1: $c36:tt, $c37:tt, $c38:tt;)?
     $(opt_act_2: $c39:tt, $c40:tt, $c41:tt;)?
     $(opt_act_3: $c42:tt, $c43:tt, $c44:tt;)?
     $(opt_mid_1: $c45:tt, $c46:tt, $c47:tt;)?
     $(opt_mid_2: $c48:tt, $c49:tt, $c50:tt;)?
     $(opt_mid_3: $c51:tt, $c52:tt, $c53:tt;)?
     $(subj_act_1: $c54:tt, $c55:tt, $c56:tt;)?
     $(subj_act_2: $c57:tt, $c58:tt, $c59:tt;)?
     $(subj_act_3: $c60:tt, $c61:tt, $c62:tt;)?
     $(subj_mid_1: $c63:tt, $c64:tt, $c65:tt;)?
     $(subj_mid_2: $c66:tt, $c67:tt, $c68:tt;)?
     $(subj_mid_3: $c69:tt, $c70:tt, $c71:tt;)?
     $(imperfect_act_1: $c72:tt, $c73:tt, $c74:tt;)?
     $(imperfect_act_2: $c75:tt, $c76:tt, $c77:tt;)?
     $(imperfect_act_3: $c78:tt, $c79:tt, $c80:tt;)?
     $(imperfect_mid_1: $c81:tt, $c82:tt, $c83:tt;)?
     $(imperfect_mid_2: $c84:tt, $c85:tt, $c86:tt;)?
     $(imperfect_mid_3: $c87:tt, $c88:tt, $c89:tt;)?
     $(future_act_1: $c90:tt, $c91:tt, $c92:tt;)?
     $(future_act_2: $c93:tt, $c94:tt, $c95:tt;)?
     $(future_act_3: $c96:tt, $c97:tt, $c98:tt;)?
     $(future_mid_1: $c99:tt, $c100:tt, $c101:tt;)?
     $(future_mid_2: $c102:tt, $c103:tt, $c104:tt;)?
     $(future_mid_3: $c105:tt, $c106:tt, $c107:tt;)?
     $(conditional_act_1: $c108:tt, $c109:tt, $c110:tt;)?
     $(conditional_act_2: $c111:tt, $c112:tt, $c113:tt;)?
     $(conditional_act_3: $c114:tt, $c115:tt, $c116:tt;)?
     $(conditional_mid_1: $c117:tt, $c118:tt, $c119:tt;)?
     $(conditional_mid_2: $c120:tt, $c121:tt, $c122:tt;)?
     $(conditional_mid_3: $c123:tt, $c124:tt, $c125:tt;)?
     $(perfect_act_1: $c126:tt, $c127:tt, $c128:tt;)?
     $(perfect_act_2: $c129:tt, $c130:tt, $c131:tt;)?
     $(perfect_act_3: $c132:tt, $c133:tt, $c134:tt;)?
     $(perfect_mid_1: $c135:tt, $c136:tt, $c137:tt;)?
     $(perfect_mid_2: $c138:tt, $c139:tt, $c140:tt;)?
     $(perfect_mid_3: $c141:tt, $c142:tt, $c143:tt;)?
     $(aorist_act_1: $c144:tt, $c145:tt, $c146:tt;)?
     $(aorist_act_2: $c147:tt, $c148:tt, $c149:tt;)?
     $(aorist_act_3: $c150:tt, $c151:tt, $c152:tt;)?
     $(aorist_mid_1: $c153:tt, $c154:tt, $c155:tt;)?
     $(aorist_mid_2: $c156:tt, $c157:tt, $c158:tt;)?
     $(aorist_mid_3: $c159:tt, $c160:tt, $c161:tt;)?
     $(infinitive: $c162:tt;)?
     $(gerund: $c163:tt;)?
     $(participle_pres: $c164:tt, $c165:tt;)?
     $(participle_fut: $c166:tt, $c167:tt;)?
     $(participle_perf: $c168:tt, $c169:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
                $(flexion::cell!($s6),)? "\t",
                $(flexion::cell!($s7),)? "\t",
                $(flexion::cell!($s8),)? "\t",
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
/// like that it reads (`aorist_as_perfect`: one value read as another).
const AORIST_AS_PERFECT: [u16; 170] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136,
    137, 138, 139, 140, 141, 142, 143, 162, 163, 164, 165, 166, 167, 168, 169,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`perfect_as_aorist`: one value read as another).
const PERFECT_AS_AORIST: [u16; 170] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153,
    154, 155, 156, 157, 158, 159, 160, 161, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`act_as_mid`: one value read as another).
const ACT_AS_MID: [u16; 170] = [
    9, 10, 11, 12, 13, 14, 15, 16, 17, 9, 10, 11, 12, 13, 14, 15, 16, 17, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 27, 28, 29, 30, 31, 32, 33, 34, 35, 45, 46, 47, 48, 49, 50, 51, 52, 53, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 63, 64, 65, 66, 67, 68, 69, 70, 71, 63, 64, 65, 66, 67, 68, 69, 70, 71,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 81, 82, 83, 84, 85, 86, 87, 88, 89, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 99, 100, 101, 102, 103, 104, 105, 106, 107, 117, 118, 119, 120, 121, 122,
    123, 124, 125, 117, 118, 119, 120, 121, 122, 123, 124, 125, 135, 136, 137, 138, 139, 140, 141,
    142, 143, 135, 136, 137, 138, 139, 140, 141, 142, 143, 153, 154, 155, 156, 157, 158, 159, 160,
    161, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 165, 165, 167, 167, 169, 169,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`future_as_aorist`: one value read as another).
const FUTURE_AS_AORIST: [u16; 170] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 108, 109, 110, 111, 112, 113, 114,
    115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133,
    134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 170] = [
    2, 1, 2, 5, 4, 5, 8, 7, 8, 11, 10, 11, 14, 13, 14, 17, 16, 17, 20, 19, 20, 23, 22, 23, 26, 25,
    26, 29, 28, 29, 32, 31, 32, 35, 34, 35, 38, 37, 38, 41, 40, 41, 44, 43, 44, 47, 46, 47, 50, 49,
    50, 53, 52, 53, 56, 55, 56, 59, 58, 59, 62, 61, 62, 65, 64, 65, 68, 67, 68, 71, 70, 71, 74, 73,
    74, 77, 76, 77, 80, 79, 80, 83, 82, 83, 86, 85, 86, 89, 88, 89, 92, 91, 92, 95, 94, 95, 98, 97,
    98, 101, 100, 101, 104, 103, 104, 107, 106, 107, 110, 109, 110, 113, 112, 113, 116, 115, 116,
    119, 118, 119, 122, 121, 122, 125, 124, 125, 128, 127, 128, 131, 130, 131, 134, 133, 134, 137,
    136, 137, 140, 139, 140, 143, 142, 143, 146, 145, 146, 149, 148, 149, 152, 151, 152, 155, 154,
    155, 158, 157, 158, 161, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `वहति`: 258 words.
    वहति,
    /// Like `लभते`: 108 words.
    लभते,
    /// Like `धरति`: 93 words.
    धरति,
    /// Like `साधयति`: 84 words.
    साधयति,
    /// Like `आनाश`: 43 words.
    आनाश,
    /// Like `रमते`: 41 words.
    रमते,
    /// Like `एष्यति`: 36 words.
    एषयति,
    /// Like `ईरयति`: 27 words.
    ईरयति,
    /// Like `आपत्`: 22 words.
    आपत,
    /// Like `इयाज`: 21 words.
    इयाज,
    /// Like `अग्रहीत्`: 21 words.
    अगरहीत,
    /// Like `जानाति`: 19 words.
    जानाति,
    /// Like `तनोति`: 16 words.
    तनोति,
    /// Like `आजीत्`: 10 words.
    आजीत,
    /// Like `आप्नोति`: 8 words.
    आपनोति,
    /// Like `ररंह`: 8 words.
    ररंह,
    /// Like `वेत्ति`: 8 words.
    वेतति,
    /// Like `नक्ति`: 8 words.
    नकति,
    /// Like `चकर्ष`: 8 words.
    चकरष,
    /// Like `भाति`: 7 words.
    भाति,
    /// Like `पिपर्ति`: 7 words.
    पिपरति,
    /// Like `बध्नाति`: 6 words.
    बधनाति,
    /// Like `ददौ`: 6 words.
    ददौ,
    /// Like `ईळे`: 5 words.
    ईळे,
    /// Like `भनक्ति`: 5 words.
    भनकति,
    /// Like `जुहाव`: 5 words.
    जुहाव,
    /// Like `इयाय`: 5 words.
    इयाय,
    /// Like `अगात्`: 5 words.
    अगात,
    /// Like `अनिति`: 5 words.
    अनिति,
    /// Like `अस्ति`: 5 words.
    असति,
    /// Like `अवति`: 4 words.
    अवति,
    /// Like `अनंसीत्`: 4 words.
    अनंसीत,
    /// Like `वमति`: 4 words.
    वमति,
    /// Like `वृणीते`: 4 words.
    वृणीते,
    /// Like `रवति`: 4 words.
    रवति,
    /// Like `जगार`: 4 words.
    जगार,
    /// Like `उवाच`: 4 words.
    उवाच,
    /// Like `आनयति`: 4 words.
    आनयति,
    /// Like `चकार`: 3 words.
    चकार,
    /// Like `आचरति`: 3 words.
    आचरति,
    /// Like `ईष्टे`: 3 words.
    ईषटे,
    /// Like `ऋघायति`: 3 words.
    ऋघायति,
    /// Like `बाधिष्यते`: 3 words.
    बाधिषयते,
    /// Like `अधुक्षत्`: 3 words.
    अधुकषत,
    /// Like `दुदोह`: 3 words.
    दुदोह,
    /// Like `वेद`: 3 words.
    वेद,
    /// Like `अचेत्`: 3 words.
    अचेत,
    /// Like `रौति`: 3 words.
    रौति,
    /// Like `अकर्`: 2 words.
    अकर,
    /// Like `अगन्`: 2 words.
    अगन,
    /// Like `आर्त`: 2 words.
    आरत,
    /// Like `अभोक्`: 2 words.
    अभोक,
    /// Like `आनट्`: 2 words.
    आनट,
    /// Like `अमृत`: 1 words.
    अमृत,
    /// Like `संतापयति`: 1 words.
    संतापयति,
    /// Like `अलेकिष्यत्`: 1 words.
    अलेकिषयत,
    /// Like `अप्राक्`: 1 words.
    अपराक,
    /// Like `अमौक्`: 1 words.
    अमौक,
    /// Like `अमोक्`: 1 words.
    अमोक,
    /// Like `अमत`: 1 words.
    अमत,
    /// Like `अवर्क्`: 1 words.
    अवरक,
    /// Like `अस्वार्`: 1 words.
    असवार,
    /// Like `अदर्क्`: 1 words.
    अदरक,
    /// Like `अजनिष्ट`: 1 words.
    अजनिषट,
    /// Like `अपत्त`: 1 words.
    अपतत,
    /// Like `अजुषत्`: 1 words.
    अजुषत,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 66] = [
        Conjugation::वहति,
        Conjugation::लभते,
        Conjugation::धरति,
        Conjugation::साधयति,
        Conjugation::आनाश,
        Conjugation::रमते,
        Conjugation::एषयति,
        Conjugation::ईरयति,
        Conjugation::आपत,
        Conjugation::इयाज,
        Conjugation::अगरहीत,
        Conjugation::जानाति,
        Conjugation::तनोति,
        Conjugation::आजीत,
        Conjugation::आपनोति,
        Conjugation::ररंह,
        Conjugation::वेतति,
        Conjugation::नकति,
        Conjugation::चकरष,
        Conjugation::भाति,
        Conjugation::पिपरति,
        Conjugation::बधनाति,
        Conjugation::ददौ,
        Conjugation::ईळे,
        Conjugation::भनकति,
        Conjugation::जुहाव,
        Conjugation::इयाय,
        Conjugation::अगात,
        Conjugation::अनिति,
        Conjugation::असति,
        Conjugation::अवति,
        Conjugation::अनंसीत,
        Conjugation::वमति,
        Conjugation::वृणीते,
        Conjugation::रवति,
        Conjugation::जगार,
        Conjugation::उवाच,
        Conjugation::आनयति,
        Conjugation::चकार,
        Conjugation::आचरति,
        Conjugation::ईषटे,
        Conjugation::ऋघायति,
        Conjugation::बाधिषयते,
        Conjugation::अधुकषत,
        Conjugation::दुदोह,
        Conjugation::वेद,
        Conjugation::अचेत,
        Conjugation::रौति,
        Conjugation::अकर,
        Conjugation::अगन,
        Conjugation::आरत,
        Conjugation::अभोक,
        Conjugation::आनट,
        Conjugation::अमृत,
        Conjugation::संतापयति,
        Conjugation::अलेकिषयत,
        Conjugation::अपराक,
        Conjugation::अमौक,
        Conjugation::अमोक,
        Conjugation::अमत,
        Conjugation::अवरक,
        Conjugation::असवार,
        Conjugation::अदरक,
        Conjugation::अजनिषट,
        Conjugation::अपतत,
        Conjugation::अजुषत,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 66] = [
    // वहति: 258 words (निन्दति, खादति, क्षालयति, वयति, बोधति, भवति).
    conjugation!("वहति", 258, "~ति";
    stem present_act: "~- like वहति";
    stem act_sg: "~- like वहति";
    stem present_mid: "~- like वहति";
    stem imperfect: "~- like वहति";
    ind_act_1: "~ामि", _, _;
    ind_act_2: _, "~थः", "~थ";
    ind_act_3: _, "~तः", _;
    ind_mid_2: "~से", _, "~ध्वे";
    ind_mid_3: "~ते", _, _;
    imp_act_1: "~ाAि", "~ाव", "~ाम";
    imp_act_2: _, "~तम्", "~त";
    imp_act_3: _, "~ताम्", _;
    imp_mid_1: "~ै", "~ावहै", "~ामहै";
    imp_mid_2: "~स्व", _, "~ध्वम्";
    subj_act_2: "~ाः|~ासि", "~ाथः", "~ाथ";
    subj_act_3: "~ात्|~ाति", "~ातः", "~ान्";
    subj_mid_2: "~ासे|~ासै", "~ैथे", "~ाध्वै";
    subj_mid_3: "~ाते|~ातै", "~ैते", "~न्त|~ान्तै";
    imperfect_act_1: "अ~म्", _, _;
    imperfect_act_2: _, "अ~तम्", "अ~त";
    imperfect_act_3: _, "अ~ताम्", _;
    imperfect_mid_2: "अ~थाः", _, "अ~ध्वम्";
    future_act_1: "~िष्यामि", "~िष्यावः", "~िष्यामः";
    future_act_2: "~िष्यसि", "~िष्यथः", "~िष्यथ";
    future_act_3: "~िष्यति", "~िष्यतः", "~िष्यन्ति";
    future_mid_1: "~िष्ये", "~िष्यावहे", "~िष्यामहे";
    future_mid_2: "~िष्यसे", "~िष्येथे", "~िष्यध्वे";
    future_mid_3: "~िष्यते", "~िष्येते", "~िष्यन्ते";
    conditional_act_1: "अ~िष्यम्", "अ~िष्याव", "अ~िष्याम";
    conditional_act_2: "अ~िष्यः", "अ~िष्यतम्", "अ~िष्यत";
    conditional_act_3: "अ~िष्यत्", "अ~िष्यताम्", "अ~िष्यन्";
    conditional_mid_1: "अ~िष्ये", "अ~िष्यावहि", "अ~िष्यामहि";
    conditional_mid_2: "अ~िष्यथाः", "अ~िष्येथाम्", "अ~िष्यध्वम्";
    conditional_mid_3: _, "अ~िष्येताम्", "अ~िष्यन्त";
    perfect_act_1: "नि~", "नि~िव", "नि~िम";
    perfect_act_2: "नि~िथ", "नि~थुः", "नि~";
    perfect_act_3: _, "नि~तुः", "नि~ुः";
    perfect_mid_1: "नि~े", "नि~िवहे", "नि~िमहे";
    perfect_mid_2: "नि~िषे", "नि~ाथे", "नि~िध्वे";
    perfect_mid_3: _, "नि~ाते", "नि~िरे";
    aorist_mid_1: "अ~िषि", "अ~िष्वहि", "अ~िष्महि";
    aorist_mid_2: "अ~िष्ठाः", "अ~िषाथाम्", "अ~िढ्वम्";
    aorist_mid_3: "अ~िष्ट", "अ~िषाताम्", "अ~िषत";
    participle_fut: "~िष्यत्", "~िष्यमाण";
    participle_perf: "नि~्वांस्", "नि~ान";
    ),
    // लभते: 108 words (वस्ते, सचते, रोचते, मन्यते, पर्दते, लभते). Like वहति, except in 39 cells: ind_act_1 pl, subj_act_2 sg, subj_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, ….
    conjugation!("लभते", 108, "~ते", like वहति as AORIST_AS_PERFECT;
    ind_act_1: _, _, "~ामः";
    subj_act_2: "~ाः", _, _;
    subj_act_3: "~ात्", _, _;
    perfect_act_1: "~ामास", "~ामासिव", "~ामासिम";
    perfect_act_2: "~ामासिथ", "~ामासथुः", "~ामास";
    perfect_act_3: "~ामास", "~ामासतुः", "~ामासुः";
    perfect_mid_1: "~ांचक्रे", "~ांचकृवहे", "~ांचकृमहे";
    perfect_mid_2: "~ांचकृषे", "~ांचक्राथे", "~ांचकृध्वे";
    perfect_mid_3: _, "~ांचक्राते", "~ांचक्रिरे";
    aorist_act_1: derived, derived, derived;
    aorist_act_2: derived, derived, derived;
    aorist_act_3: derived, derived, derived;
    aorist_mid_1: derived, derived, derived;
    aorist_mid_2: derived, derived, derived;
    aorist_mid_3: _, derived, derived;
    participle_perf: "~ामासिवांस्", "~ांचक्राण";
    ),
    // धरति: 93 words (भरति, भुरति, दुष्यति, द्रायति, ओषति, द्रुह्यति).
    conjugation!("धरति", 93, "~~ति";
    stem present_mid: "~~- like धरति";
    stem imperfect: "~~- like वहति";
    ind_act_1: "~~ामि", "~~ावः", "~~ामः";
    ind_act_2: "~~सि", "~~थः", "~~थ";
    ind_act_3: "~~ति", "~~तः", "~~न्ति";
    ind_mid_2: "~~से", _, "~~ध्वे";
    ind_mid_3: "~~ते", _, _;
    imp_act_1: "~~ाणि", "~~ाव", "~~ाम";
    imp_act_2: "~~", "~~तम्", "~~त";
    imp_act_3: "~~तु", "~~ताम्", "~~न्तु";
    imp_mid_1: "~~ै", "~~ावहै", "~~ामहै";
    imp_mid_2: "~~स्व", _, "~~ध्वम्";
    opt_act_1: "~~ेयम्", "~~ेव", "~~ेम";
    opt_act_2: "~~ेः", "~~ेतम्", "~~ेत";
    opt_act_3: "~~ेत्", "~~ेताम्", "~~ेयुः";
    subj_act_2: "~~ाः", "~~ाथः", "~~ाथ";
    subj_act_3: "~~ात्", "~~ातः", "~~ान्";
    subj_mid_2: "~~ासे", "~~ैथे", "~~ाध्वै";
    subj_mid_3: "~~ाते", "~~ैते", "~~न्त";
    imperfect_act_1: "अ~~म्", _, _;
    imperfect_act_2: _, "अ~~तम्", "अ~~त";
    imperfect_act_3: _, "अ~~ताम्", _;
    imperfect_mid_2: "अ~~थाः", _, "अ~~ध्वम्";
    future_act_1: "~~िष्यामि", "~~िष्यावः", "~~िष्यामः";
    future_act_2: "~~िष्यसि", "~~िष्यथः", "~~िष्यथ";
    future_act_3: "~~िष्यति", "~~िष्यतः", "~~िष्यन्ति";
    future_mid_1: "~~िष्ये", "~~िष्यावहे", "~~िष्यामहे";
    future_mid_2: "~~िष्यसे", "~~िष्येथे", "~~िष्यध्वे";
    future_mid_3: "~~िष्यते", "~~िष्येते", "~~िष्यन्ते";
    conditional_act_1: "अ~~िष्यम्", "अ~~िष्याव", "अ~~िष्याम";
    conditional_act_2: "अ~~िष्यः", "अ~~िष्यतम्", "अ~~िष्यत";
    conditional_act_3: "अ~~िष्यत्", "अ~~िष्यताम्", "अ~~िष्यन्";
    conditional_mid_1: "अ~~िष्ये", "अ~~िष्यावहि", "अ~~िष्यामहि";
    conditional_mid_2: "अ~~िष्यथाः", "अ~~िष्येथाम्", "अ~~िष्यध्वम्";
    conditional_mid_3: _, "अ~~िष्येताम्", "अ~~िष्यन्त";
    perfect_act_1: "च~~", "~े~िव", "~े~िम";
    perfect_act_2: "च~~्थ", "~े~थुः", "~े~";
    perfect_act_3: "~चा~", "~े~तुः", "~े~ुः";
    perfect_mid_1: "~े~े", "~े~िवहे", "~े~िमहे";
    perfect_mid_2: "~े~िषे", "~े~ाथे", "~े~िध्वे";
    perfect_mid_3: _, "~े~ाते", "~े~िरे";
    aorist_act_1: "अ~ा~िषम्", "अ~ा~िष्व", "अ~ा~िष्म";
    aorist_act_2: "अ~ा~ीः", "अ~ा~िष्टम्", "अ~ा~िष्ट";
    aorist_act_3: "अ~ा~ीत्", "अ~ा~िष्टाम्", "अ~ा~िषुः";
    aorist_mid_1: "अ~ा~िषि", "अ~ा~िष्वहि", "अ~ा~िष्महि";
    aorist_mid_2: "अ~ा~िष्ठाः", "अ~ा~िषाथाम्", "अ~ा~िढ्वम्";
    aorist_mid_3: "अ~ा~िष्ट", "अ~ा~िषाताम्", "अ~ा~िषत";
    participle_pres: "~~त्", _;
    participle_fut: "~~िष्यत्", "~~िष्यमाण";
    participle_perf: "~े~िवांस्", "~े~ाण";
    ),
    // साधयति: 84 words (इच्छति, दर्शयति, नाशयति, रन्धयति, जासयति, भाजयति). Like लभते, except in 13 cells: subj_act_2 sg, subj_act_3 sg, future_act_1 pl, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, ….
    conjugation!("साधयति", 84, "~ति", like लभते;
    stem present_act: "~- like साधयति";
    subj_act_2: "~ाः|~ासि", _, _;
    subj_act_3: "~ात्|~ाति", _, _;
    future_act_1: _, _, "~िष्यामः|~िष्यामसि";
    perfect_act_1: "~ामास|~ांचकर", "~ामासिव|~ांचकृव", "~ामासिम|~ांचकृम";
    perfect_act_2: "~ामासिथ|~ांचकर्थ", "~ामासथुः|~ांचक्रथुः", "~ामास|~ांचक्र";
    perfect_act_3: _, "~ामासतुः|~ांचक्रतुः", "~ामासुः|~ांचक्रुः";
    participle_pres: _, "~माA|~ान";
    participle_perf: "~ामासिवांस्|~ांचकृवांस्", _;
    ),
    // आनाश: 43 words (आद, आप, देग्धि, द्वेष्टि, आर, आह).
    conjugation!("आनाश", 43, "~";
    ind_act_3: "~", _, _;
    perfect_act_1: "~", "~िव", "~िम";
    perfect_act_2: "~िथ", "~थुः", "~";
    perfect_act_3: "~", "~तुः", "~ुः";
    perfect_mid_1: "~े", "~िवहे", "~िमहे";
    perfect_mid_2: "~िषे", "~ाथे", "~िध्वे";
    perfect_mid_3: _, "~ाते", "~िरे";
    participle_perf: "~िवांस्", "~ाA";
    ),
    // रमते: 41 words (म्रियते, भाषते, रमते, परीक्षते, त्रपते, ह्वरते). Like लभते, except in 0 cells: .
    conjugation!("रमते", 41, "~ते", like लभते;
    stem present_mid: "~- like धरति";
    ),
    // एषयति: 36 words (करिष्यति, गमिष्यति, पास्यति, भविष्यति, तप्स्यति, भरिष्यति). Like आनाश, except in 40 cells: ind_act_3 sg, future_act_1 sg, future_act_1 du, future_act_1 pl, future_act_2 sg, future_act_2 du, ….
    conjugation!("एष्यति", 36, "~ति", like आनाश as PERFECT_AS_AORIST;
    ind_act_3: "~ति", _, _;
    future_act_1: "~ामि", "~ावः", "~ामः|~ामसि";
    future_act_2: "~सि", "~थः", "~थ";
    future_act_3: "~ति", "~तः", "~न्ति";
    future_mid_1: "~े", "~ावहे", "~ामहे";
    future_mid_2: "~से", "~ेथे", "~ध्वे";
    future_mid_3: "~ते", "~ेते", "~न्ते";
    conditional_act_1: "अ~म्", "अ~ाव", "अ~ाम";
    conditional_act_2: "अ~ः", "अ~तम्", "अ~त";
    conditional_act_3: "अ~त्", "अ~ताम्", "अ~न्";
    conditional_mid_1: "अ~े", "अ~ावहि", "अ~ामहि";
    conditional_mid_2: "अ~थाः", "अ~ेथाम्", "अ~ध्वम्";
    conditional_mid_3: _, "अ~ेताम्", "अ~न्त";
    participle_fut: "~त्", "~माA";
    participle_perf: derived, derived;
    ),
    // ईरयति: 27 words (बर्हयति, धारयति, रोपयति, कारयति, शोषयति, चोरयति). Like साधयति, except in 1 cell: participle_pres mid.
    conjugation!("ईरयति", 27, "~ति", like साधयति;
    participle_pres: _, "~माA|~ाण";
    ),
    // आपत: 22 words (अगमत्, अश्रोत्, अभूत्, अवोचत्, अशुचत्, आरत्). Like आनाश, except in 15 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_2 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, ….
    conjugation!("आपत्", 22, "~त्", like आनाश as PERFECT_AS_AORIST;
    stem present_ind_act_1_sg: "~- like आपत्";
    ind_act_3: "~त्", _, _;
    perfect_act_1: derived, _, _;
    perfect_act_2: derived, _, _;
    aorist_act_1: "~म्", "~ाव", "~ाम";
    aorist_act_2: "~ः", "~तम्", "~त";
    aorist_act_3: "~त्", "~ताम्", "~न्";
    aorist_mid_2: _, _, "~ध्वम्";
    participle_perf: derived, derived;
    ),
    // इयाज: 21 words (जगाम, ससान, तताप, ततान, ददाह, जजान).
    conjugation!("इयाज", 21, "~ा~";
    ind_act_3: "~ा~", _, _;
    perfect_act_1: "~~", "~्~िव", "~्~िम";
    perfect_act_2: "~~्थ", "~्~थुः", "~्~";
    perfect_act_3: "~ा~", "~्~तुः", "~्~ुः";
    perfect_mid_1: "~्~े", "~्~िवहे", "~्~िमहे";
    perfect_mid_2: "~्~िषे", "~्~ाथे", "~्~िध्वे";
    perfect_mid_3: _, "~्~ाते", "~्~िरे";
    participle_perf: "~्~िवांस्", "~्~ान";
    ),
    // अगरहीत: 21 words (अद्राक्षीत्, अताप्सीत्, अहार्षीत्, अश्रौषीत्, अभार्षीत्, अनौत्सीत्). Like आपत, except in 11 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("अग्रहीत्", 21, "~ीत्", like आपत;
    stem present_ind_act_1_sg: "~ीB- like अग्रहीत्";
    ind_act_3: "~ीत्", _, _;
    aorist_act_1: "~ीBम्|~ैBम्", "~ीB्व", "~ीB्म";
    aorist_act_2: "~ीः", "~ीष्टम्", "~ीष्ट";
    aorist_act_3: "~ीत्", "~ीष्टाम्", "~ीBुः";
    aorist_mid_2: _, _, "~ीढ्वम्";
    ),
    // जानाति: 19 words (ददाति, दधाति, जानाति, श्रद्दधाति, गृणाति, जहाति). Like आपत, except in 39 cells: ind_act_1 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 pl, ….
    conjugation!("जानाति", 19, "~ाति", like आपत as AORIST_AS_PERFECT;
    stem present_act: "~- like जानाति";
    stem present_ind_act_2_pl: "~ी- like जानाति";
    stem present_mid: "~- like जानाति";
    stem imperfect: "~- like जानाति";
    ind_act_1: "~ामि", _, _;
    ind_act_2: _, "~ीथः", _;
    ind_act_3: "~ाति", "~ीतः", _;
    ind_mid_2: "~ीBे", _, "~ीध्वे";
    ind_mid_3: "~ीते", _, _;
    imp_act_1: "~ानि", "~ाव", "~ाम";
    imp_act_2: "~ीहि", "~ीतम्", _;
    imp_act_3: "~ातु", "~ीताम्", _;
    imp_mid_1: "~ै", "~ावहै", "~ामहै";
    imp_mid_2: "~ीB्व", _, "~ीध्वम्";
    subj_act_1: "~ानि|~ा", _, _;
    subj_act_2: "~ाः|~ासि", "~ाथः", "~ाथ";
    subj_act_3: "~ात्|~ाति", "~ातः", "~ान्";
    subj_mid_2: "~ासे|~ासै", "~ैथे", "~ाध्वे|~ाध्वै";
    subj_mid_3: "~ाते|~ातै", "~ैते", "~ान्त|~ान्तै";
    imperfect_act_1: "अ~ाम्", _, _;
    imperfect_act_2: _, "अ~ीतम्", "अ~ीत";
    imperfect_act_3: _, "अ~ीताम्", _;
    imperfect_mid_2: "अ~ीथाः", _, "अ~ीध्वम्";
    aorist_act_3: _, _, derived;
    ),
    // तनोति: 16 words (वृणोति, तनोति, करोति, जुहोति, शृणोति, घृणोति). Like जानाति, except in 38 cells: ind_act_1 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 pl, ….
    conjugation!("तनोति", 16, "~ोति", like जानाति;
    stem present_act: "~- like तनोति";
    stem present_ind_act_2_pl: "~ु- like जानाति";
    stem present_mid: "~्- like तनोति";
    stem imperfect: "~- like तनोति";
    ind_act_1: "~ोमि", _, _;
    ind_act_2: _, "~ुथः", _;
    ind_act_3: "~ोति", "~ुतः", _;
    ind_mid_2: "~ुBे", _, "~ुध्वे";
    ind_mid_3: "~ुते", _, _;
    imp_act_1: "~वाAि", "~वाव", "~वाम";
    imp_act_2: "~ु", "~ुतम्", _;
    imp_act_3: "~ोतु", "~ुताम्", _;
    imp_mid_1: "~वै", "~वावहै", "~वामहै";
    imp_mid_2: "~ुB्व", _, "~ुध्वम्";
    subj_act_1: "~वाAि|~वा", _, _;
    subj_act_2: "~वः|~वसि", "~वथः", "~वथ";
    subj_act_3: "~वत्|~वति", "~वतः", "~वन्";
    subj_mid_2: "~वसे", "~वैथे", "~वध्वे";
    subj_mid_3: "~वते", "~वैते", "~वन्त";
    imperfect_act_1: "अ~वम्", _, _;
    imperfect_act_2: _, "अ~ुतम्", "अ~ुत";
    imperfect_act_3: _, "अ~ुताम्", _;
    imperfect_mid_2: "अ~ुथाः", _, "अ~ुध्वम्";
    ),
    // आजीत: 10 words (अस्वारीत्, आजीत्, अकाषीत्, आञ्चीत्, अलेखीत्, अवेदीत्). Like अगरहीत, except in 8 cells: aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 du, aorist_act_2 pl, aorist_act_3 du, ….
    conjugation!("आजीत्", 10, "~ीत्", like अगरहीत;
    stem present_ind_act_1_sg: "~िB- like अग्रहीत्";
    aorist_act_1: "~िBम्", "~िB्व", "~िB्म";
    aorist_act_2: _, "~िष्टम्", "~िष्ट";
    aorist_act_3: _, "~िष्टाम्", "~िBुः";
    aorist_mid_2: _, _, "~िढ्वम्";
    ),
    // आपनोति: 8 words (धृष्णोति, शक्नोति, साध्नोति, आप्नोति, ऋध्नोति, दभ्नोति). Like तनोति, except in 15 cells: ind_act_1 du, ind_act_1 pl, ind_act_3 pl, imp_act_2 sg, imp_act_3 pl, imperfect_act_1 du, ….
    conjugation!("आप्नोति", 8, "~ोति", like तनोति;
    stem present_mid: "~ु- like तनोति";
    ind_act_1: _, "~ुवः", "~ुमः|~ुमसि";
    ind_act_3: _, _, "~ुवन्ति";
    imp_act_2: "~ुहि", _, _;
    imp_act_3: _, _, "~ुवन्तु";
    imperfect_act_1: _, "अ~ुव", "अ~ुम";
    imperfect_act_3: _, _, "अ~ुवन्";
    imperfect_mid_1: "अ~ुवि", "अ~ुवहि", "अ~ुमहि";
    imperfect_mid_2: _, "अ~ुवाथाम्", _;
    imperfect_mid_3: _, "अ~ुवाताम्", "अ~ुवत";
    participle_pres: "~ुवत्", _;
    ),
    // ररंह: 8 words (आस, ततक्ष, ररंह, चकाङ्क्ष, जिहीड, जिहिंस). Like आनाश, except in 1 cell: participle_perf act.
    conjugation!("ररंह", 8, "~", like आनाश;
    participle_perf: "~्वांस्", _;
    ),
    // वेतति: 8 words (वेत्ति, अत्ति, उनत्ति, भिनत्ति, छिनत्ति, वेवेत्ति). Like आपत, except in 38 cells: ind_act_1 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 pl, ….
    conjugation!("वेत्ति", 8, "~त्ति", like आपत as AORIST_AS_PERFECT;
    stem present_act: "~- like वेत्ति";
    stem present_ind_act_2_pl: "~्त्- like जानाति";
    stem present_mid: "~्द- like वेत्ति";
    stem imperfect: "~- like वेत्ति";
    ind_act_1: "~द्मि", _, _;
    ind_act_2: _, "~्त्थः", _;
    ind_act_3: "~त्ति", "~्त्तः", _;
    ind_mid_2: "~्त्से", _, "~्द्ध्वे";
    ind_mid_3: "~्त्ते", _, _;
    imp_act_1: "~दाAि", "~दाव", "~दाम";
    imp_act_2: "~्द्धि", "~्त्तम्", _;
    imp_act_3: "~त्तु", "~्त्ताम्", _;
    imp_mid_1: "~दै", "~दावहै", "~दामहै";
    imp_mid_2: "~्त्स्व", _, "~्द्ध्वम्";
    subj_act_2: "~दः", "~दथः", "~दथ";
    subj_act_3: "~दत्", "~दतः", "~दन्";
    subj_mid_2: "~दसे", "~दैथे", "~दध्वे";
    subj_mid_3: "~दते", "~दैते", "~दन्त";
    imperfect_act_1: "अ~दम्", _, _;
    imperfect_act_2: _, "अ~्त्तम्", "अ~्त्त";
    imperfect_act_3: _, "अ~्त्ताम्", _;
    imperfect_mid_2: "अ~्त्थाः", _, "अ~्द्ध्वम्";
    aorist_act_3: _, _, derived;
    ),
    // नकति: 8 words (रिणक्ति, वक्ति, नेनेक्ति, पृणक्ति, वृणक्ति, भिषक्ति). Like वेतति, except in 37 cells: ind_act_1 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 pl, ….
    conjugation!("नक्ति", 8, "~क्ति", like वेतति;
    stem present_act: "~- like नक्ति";
    stem present_ind_act_2_pl: "~क्- like जानाति";
    stem present_mid: "~ज- like नक्ति";
    stem imperfect: "~- like नक्ति";
    ind_act_1: "~ज्मि", _, _;
    ind_act_2: _, "~क्थः", _;
    ind_act_3: "~क्ति", "~क्तः", _;
    ind_mid_2: "~क्Bे", _, "~ग्ध्वे";
    ind_mid_3: "~क्ते", _, _;
    imp_act_1: "~जाAि", "~जाव", "~जाम";
    imp_act_2: "~ग्धि", "~क्तम्", _;
    imp_act_3: "~क्तु", "~क्ताम्", _;
    imp_mid_1: "~जै", "~जावहै", "~जामहै";
    imp_mid_2: "~क्B्व", _, "~ग्ध्वम्";
    subj_act_2: "~जः", "~जथः", "~जथ";
    subj_act_3: "~जत्", "~जतः", "~जन्";
    subj_mid_2: "~जसे", "~जैथे", "~जध्वे";
    subj_mid_3: "~जते", "~जैते", "~जन्त";
    imperfect_act_1: "अ~जम्", _, _;
    imperfect_act_2: _, "अ~क्तम्", "अ~क्त";
    imperfect_act_3: _, "अ~क्ताम्", _;
    imperfect_mid_2: "अ~क्थाः", _, "अ~ग्ध्वम्";
    ),
    // चकरष: 8 words (ददर्श, ववर्त, पपर्च, जगर्ध, बबर्ह, आनर्च). Like इयाज, except in 12 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("चकर्ष", 8, "~र्~", like इयाज as PERFECT_AS_AORIST;
    ind_act_3: "~र्~", _, _;
    perfect_act_1: "~र्~", "~ृ~िव", "~ृ~िम";
    perfect_act_2: "~र्~िथ", "~ृ~थुः", "~ृ~";
    perfect_act_3: "~र्~", "~ृ~तुः", "~ृ~ुः";
    participle_perf: "~ृ~्वांस्", derived;
    ),
    // भाति: 7 words (भाति, द्राति, वाति, पाति, याति, आयाति). Like जानाति, except in 36 cells: ind_act_2 du, ind_act_3 du, ind_mid_1 sg, ind_mid_1 du, ind_mid_1 pl, ind_mid_2 sg, ….
    conjugation!("भाति", 7, "~ाति", like जानाति;
    stem present_act: "~ा- like भाति";
    stem present_ind_act_2_pl: "~ा- like जानाति";
    ind_act_2: _, "~ाथः", _;
    ind_act_3: _, "~ातः", _;
    ind_mid_1: "~ै", "~ावहे", "~ामहे";
    ind_mid_2: "~ाBे", _, "~ाध्वे";
    ind_mid_3: "~ाते", _, "~ाते";
    imp_act_2: "~ाहि", "~ातम्", _;
    imp_act_3: _, "~ाताम्", _;
    imp_mid_2: "~ाB्व", _, "~ाध्वम्";
    imp_mid_3: _, _, "~ाताम्";
    opt_mid_1: "~ेय", "~ेवहि", "~ेमहि";
    opt_mid_2: "~ेथाः", "~ेयाथाम्", "~ेध्वम्";
    opt_mid_3: "~ेत", "~ेयाताम्", "~ेरन्";
    imperfect_act_1: _, "अ~ाव", "अ~ाम";
    imperfect_act_2: _, "अ~ातम्", "अ~ात";
    imperfect_act_3: _, "अ~ाताम्", "अ~ुः|अ~ान्";
    imperfect_mid_1: "अ~े", "अ~ावहि", "अ~ामहि";
    imperfect_mid_2: "अ~ाथाः", _, "अ~ाध्वम्";
    imperfect_mid_3: _, _, "अ~ात";
    ),
    // पिपरति: 7 words (पिपर्ति, जागर्ति, बिभर्ति, जिघर्ति, इयर्ति, सिसर्ति). Like जानाति, except in 38 cells: ind_act_1 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 pl, ….
    conjugation!("पिपर्ति", 7, "~र्ति", like जानाति;
    stem present_act: "~- like पिपर्ति";
    stem present_ind_act_2_pl: "~ृ- like जानाति";
    stem present_mid: "~- like पिपर्ति";
    stem imperfect: "~- like पिपर्ति";
    ind_act_1: "~र्मि", _, _;
    ind_act_2: _, "~ृथः", _;
    ind_act_3: "~र्ति", "~ृतः", _;
    ind_mid_2: "~ृषे", _, "~ृध्वे";
    ind_mid_3: "~ृते", _, _;
    imp_act_1: "~राAि", "~राव", "~राम";
    imp_act_2: "~ृहि", "~ृतम्", _;
    imp_act_3: "~र्तु", "~ृताम्", _;
    imp_mid_1: "~रै", "~रावहै", "~रामहै";
    imp_mid_2: "~ृष्व", _, "~ृध्वम्";
    subj_act_1: "~राAि|~रा", _, _;
    subj_act_2: "~रः|~रसि", "~रथः", "~रथ";
    subj_act_3: "~रत्|~रति", "~रतः", "~रन्";
    subj_mid_2: "~रसे", "~रैथे", "~रध्वे";
    subj_mid_3: "~रते", "~रैते", "~रन्त";
    imperfect_act_1: "अ~रम्", _, _;
    imperfect_act_2: _, "अ~ृतम्", "अ~ृत";
    imperfect_act_3: _, "अ~ृताम्", _;
    imperfect_mid_2: "अ~ृथाः", _, "अ~ृध्वम्";
    ),
    // बधनाति: 6 words (गृभ्णाति, उभ्नाति, गृह्णाति, इष्णाति, बध्नाति, तभ्नाति). Like चकरष, except in 46 cells: ind_act_1 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 pl, ….
    conjugation!("बध्नाति", 6, "~्~ाति", like चकरष as ACT_AS_MID;
    stem present_act: "~्~- like बध्नाति";
    stem present_ind_act_2_pl: "~्~ी- like जानाति";
    stem present_mid: "~्~- like जानाति";
    stem imperfect: "~्~- like बध्नाति";
    ind_act_1: "~्~ामि", _, _;
    ind_act_2: _, "~्~ीथः", _;
    ind_act_3: "~्~ाति", "~्~ीतः", _;
    ind_mid_2: "~्~ीषे", _, "~्~ीध्वे";
    ind_mid_3: "~्~ीते", _, _;
    imp_act_1: "~्~ानि", "~्~ाव", "~्~ाम";
    imp_act_2: "~ा~", "~्~ीतम्", _;
    imp_act_3: "~्~ातु", "~्~ीताम्", _;
    imp_mid_1: "~्~ै", "~्~ावहै", "~्~ामहै";
    imp_mid_2: "~्~ीष्व", _, "~्~ीध्वम्";
    subj_act_1: "~्~ानि|~्~ा", _, _;
    subj_act_2: "~्~ाः|~्~ासि", "~्~ाथः", "~्~ाथ";
    subj_act_3: "~्~ात्|~्~ाति", "~्~ातः", "~्~ान्";
    subj_mid_2: "~्~ासे", "~्~ैथे", "~्~ाध्वे";
    subj_mid_3: "~्~ाते", "~्~ैते", "~्~ान्त";
    imperfect_act_1: "अ~्~ाम्", _, _;
    imperfect_act_2: _, "अ~्~ीतम्", "अ~्~ीत";
    imperfect_act_3: _, "अ~्~ीताम्", _;
    imperfect_mid_2: "अ~्~ीथाः", _, "अ~्~ीध्वम्";
    perfect_act_1: _, derived, derived;
    perfect_act_2: _, derived, derived;
    perfect_act_3: derived, derived, derived;
    participle_perf: derived, _;
    ),
    // ददौ: 6 words (तस्थौ, ददौ, पपौ, दधौ, ररौ, पप्रौ). Like आनाश, except in 4 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_2 sg, perfect_act_3 sg.
    conjugation!("ददौ", 6, "~ौ", like आनाश;
    ind_act_3: "~ौ", _, _;
    perfect_act_1: "~ौ", _, _;
    perfect_act_2: "~ाथ|~िथ", _, _;
    perfect_act_3: "~ौ", _, _;
    ),
    // ईळे: 5 words (ईळे, जज्ञे, शश्ये, पस्पशे, चके). Like आनाश, except in 10 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("ईळे", 5, "~े", like आनाश as ACT_AS_MID;
    ind_act_3: "~े", _, _;
    perfect_act_1: derived, derived, derived;
    perfect_act_2: derived, derived, derived;
    perfect_act_3: _, derived, derived;
    participle_perf: derived, _;
    ),
    // भनकति: 5 words (भनक्ति, युनक्ति, तनक्ति, अनक्ति, भुनक्ति). Like जानाति, except in 38 cells: ind_act_1 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 pl, ….
    conjugation!("भनक्ति", 5, "~नक्ति", like जानाति;
    stem present_act: "~- like भनक्ति";
    stem present_ind_act_2_pl: "~ङ्क्- like जानाति";
    stem present_mid: "~ञ्ज- like नक्ति";
    stem imperfect: "~- like भनक्ति";
    ind_act_1: "~Aज्मि", _, _;
    ind_act_2: _, "~ङ्क्थः", _;
    ind_act_3: "~नक्ति", "~ङ्क्तः", _;
    ind_mid_2: "~ङ्क्Bे", _, "~ङ्ग्ध्वे";
    ind_mid_3: "~ङ्क्ते", _, _;
    imp_act_1: "~AजाAि", "~Aजाव", "~Aजाम";
    imp_act_2: "~ङ्ग्धि", "~ङ्क्तम्", _;
    imp_act_3: "~Aक्तु", "~ङ्क्ताम्", _;
    imp_mid_1: "~Aजै", "~Aजावहै", "~Aजामहै";
    imp_mid_2: "~ङ्क्B्व", _, "~ङ्ग्ध्वम्";
    subj_act_1: "~AजाAि|~नजा", _, _;
    subj_act_2: "~Aजः|~नजसि", "~Aजथः", "~Aजथ";
    subj_act_3: "~Aजत्|~Aजति", "~Aजतः", "~Aजन्";
    subj_mid_2: "~Aजसे", "~Aजैथे", "~Aजध्वे";
    subj_mid_3: "~Aजते", "~Aजैते", "~Aजन्त";
    imperfect_act_1: "अ~Aजम्", _, _;
    imperfect_act_2: _, "अ~ङ्क्तम्", "अ~ङ्क्त";
    imperfect_act_3: _, "अ~ङ्क्ताम्", _;
    imperfect_mid_2: "अ~ङ्क्थाः", _, "अ~ङ्ग्ध्वम्";
    ),
    // जुहाव: 5 words (दुद्राव, शुश्राव, तुष्टाव, सुस्राव, जुहाव). Like आनाश, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("जुहाव", 5, "~ाव", like आनाश;
    ind_act_3: "~ाव", _, _;
    perfect_act_1: "~व|~ाव", "~ुव", "~ुम";
    perfect_act_2: "~ोथ", "~ुवथुः", "~ुव";
    perfect_act_3: "~ाव", "~ुवतुः", "~ुवुः";
    perfect_mid_1: "~ुवे", "~ुवहे", "~ुमहे";
    perfect_mid_2: "~ुषे", "~ुवाथे", "~ुध्वे";
    perfect_mid_3: _, "~ुवाते", "~ुविरे";
    participle_perf: "~ुवांस्", "~ुवाA";
    ),
    // इयाय: 5 words (बिभाय, जिगाय, इयाय, जिघाय, निनाय). Like आनाश, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("इयाय", 5, "~ाय", like आनाश;
    ind_act_3: "~ाय", _, _;
    perfect_act_1: "~य|~ाय", "~्यिव", "~्यिम";
    perfect_act_2: "~ेथ|~यिथ", "~्यथुः", "~्य";
    perfect_act_3: "~ाय", "~्यतुः", "~्युः";
    perfect_mid_1: "~्ये", "~्यिवहे", "~्यिमहे";
    perfect_mid_2: "~्यिषे", "~्याथे", "~्यिध्वे";
    perfect_mid_3: _, "~्याते", "~्यिरे";
    participle_perf: "~ीवांस्", "~्याA";
    ),
    // अगात: 5 words (अधात्, अस्थात्, अदात्, अगात्, अपात्). Like आपत, except in 17 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_2 sg, aorist_act_2 du, aorist_act_2 pl, aorist_act_3 sg, ….
    conjugation!("अगात्", 5, "~ात्", like आपत;
    ind_act_3: "~ात्", _, _;
    aorist_act_1: "~ाम्", _, _;
    aorist_act_2: "~ाः", "~ातम्", "~ात";
    aorist_act_3: "~ात्", "~ाताम्", "~ुः";
    aorist_mid_1: "~ि", "~िवहि", "~िमहि";
    aorist_mid_2: "~िथाः", "~ाथाम्", "~िध्वम्";
    aorist_mid_3: "~ित", "~ाताम्", "~त";
    ),
    // अनिति: 5 words (रोदिति, अनिति, श्वसिति, उच्छ्वसिति, वपिति). Like जानाति, except in 51 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ….
    conjugation!("अनिति", 5, "~िति", like जानाति;
    stem present_ind_act_2_pl: "~ि- like जानाति";
    ind_act_1: "~िमि", "~िवः", "~िमः|~िमसि";
    ind_act_2: "~िBि", "~िथः", _;
    ind_act_3: "~िति", "~ितः", _;
    ind_mid_1: _, "~िवहे", "~िमहे";
    ind_mid_2: "~िBे", _, "~िध्वे";
    ind_mid_3: "~िते", _, _;
    imp_act_2: "~िहि", "~ितम्", _;
    imp_act_3: "~ितु", "~िताम्", _;
    imp_mid_2: "~िB्व", _, "~िध्वम्";
    opt_act_1: "~्याम्", "~्याव", "~्याम";
    opt_act_2: "~्याः", "~्यातम्", "~्यात";
    opt_act_3: "~्यात्", "~्याताम्", "~्युः";
    opt_mid_2: _, _, "~ीध्वम्";
    subj_act_1: derived, _, _;
    subj_act_2: "~ः", "~थः", "~थ";
    subj_act_3: "~त्", "~तः", "~न्";
    subj_mid_2: "~से", _, "~ध्वे";
    subj_mid_3: "~ते", _, "~न्त";
    imperfect_act_1: "अ~म्", "अ~िव", "अ~िम";
    imperfect_act_2: "अ~ीः", "अ~ितम्", "अ~ित";
    imperfect_act_3: "अ~ीत्", "अ~िताम्", _;
    imperfect_mid_1: _, "अ~िवहि", "अ~िमहि";
    imperfect_mid_2: "अ~िथाः", _, "अ~िध्वम्";
    ),
    // असति: 5 words (अस्ति, शास्ति, बभस्ति, हिनस्ति, सस्ति). Like आपत, except in 27 cells: ind_act_1 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, imp_act_1 sg, imp_act_1 du, ….
    conjugation!("अस्ति", 5, "~स्ति", like आपत as AORIST_AS_PERFECT;
    stem present_act: "~- like अस्ति";
    stem present_ind_act_2_pl: "~B्- like जानाति";
    ind_act_1: "~स्मि", _, _;
    ind_act_2: _, "~B्थः", _;
    ind_act_3: "~स्ति", "~B्तः", _;
    imp_act_1: "~साAि", "~साव", "~साम";
    imp_act_2: "~द्Bि", "~B्तम्", _;
    imp_act_3: "~स्तु", "~B्ताम्", _;
    subj_act_2: "~सः", "~सथः", "~सथ";
    subj_act_3: "~सत्", "~सतः", "~सन्";
    imperfect_act_1: "अ~सम्", "अ~B्व", "अ~B्म";
    imperfect_act_2: "अ~ः", "अ~B्तम्", "अ~B्त";
    imperfect_act_3: "अ~ः", "अ~B्ताम्", "अ~BA्";
    aorist_act_3: _, _, derived;
    ),
    // अवति: 4 words (अवति, अपनह्यति, अपिनह्यति, अर्चयति).
    conjugation!("अवति", 4, "अ~ति";
    stem present_act: "~- like अवति";
    stem act_sg: "~- like अवति";
    stem present_mid: "~- like अवति";
    stem imperfect: "~- like अवति";
    ind_act_1: "अ~ामि", _, _;
    ind_act_2: _, "अ~थः", "अ~थ";
    ind_act_3: _, "अ~तः", _;
    ind_mid_2: "अ~से", _, "अ~ध्वे";
    ind_mid_3: "अ~ते", _, _;
    imp_act_1: "अ~ाAि", "अ~ाव", "अ~ाम";
    imp_act_2: _, "अ~तम्", "अ~त";
    imp_act_3: _, "अ~ताम्", _;
    imp_mid_1: "अ~ै", "अ~ावहै", "अ~ामहै";
    imp_mid_2: "अ~स्व", _, "अ~ध्वम्";
    subj_act_2: "अ~ाः|अ~ासि", "अ~ाथः", "अ~ाथ";
    subj_act_3: "अ~ात्|अ~ाति", "अ~ातः", "अ~ान्";
    subj_mid_2: "अ~ासे|अ~ासै", "अ~ैथे", "अ~ाध्वै";
    subj_mid_3: "अ~ाते|अ~ातै", "अ~ैते", "अ~न्त|अ~ान्तै";
    imperfect_act_1: "आ~म्", _, _;
    imperfect_act_2: _, "आ~तम्", "आ~त";
    imperfect_act_3: _, "आ~ताम्", _;
    imperfect_mid_2: "आ~थाः", _, "आ~ध्वम्";
    future_act_1: "अ~िष्यामि", "अ~िष्यावः", "अ~िष्यामः";
    future_act_2: "अ~िष्यसि", "अ~िष्यथः", "अ~िष्यथ";
    future_act_3: "अ~िष्यति", "अ~िष्यतः", "अ~िष्यन्ति";
    future_mid_1: "अ~िष्ये", "अ~िष्यावहे", "अ~िष्यामहे";
    future_mid_2: "अ~िष्यसे", "अ~िष्येथे", "अ~िष्यध्वे";
    future_mid_3: "अ~िष्यते", "अ~िष्येते", "अ~िष्यन्ते";
    conditional_act_1: "आ~िष्यम्", "आ~िष्याव", "आ~िष्याम";
    conditional_act_2: "आ~िष्यः", "आ~िष्यतम्", "आ~िष्यत";
    conditional_act_3: "आ~िष्यत्", "आ~िष्यताम्", "आ~िष्यन्";
    conditional_mid_1: "आ~िष्ये", "आ~िष्यावहि", "आ~िष्यामहि";
    conditional_mid_2: "आ~िष्यथाः", "आ~िष्येथाम्", "आ~िष्यध्वम्";
    conditional_mid_3: _, "आ~िष्येताम्", "आ~िष्यन्त";
    perfect_act_1: "आ~", "आ~िव", "आ~िम";
    perfect_act_2: "आ~िथ", "आ~थुः", "आ~";
    perfect_act_3: _, "आ~तुः", "आ~ुः";
    perfect_mid_1: "आ~े", "आ~िवहे", "आ~िमहे";
    perfect_mid_2: "आ~िषे", "आ~ाथे", "आ~िध्वे";
    perfect_mid_3: _, "आ~ाते", "आ~िरे";
    aorist_mid_1: "आ~िषि", "आ~िष्वहि", "आ~िष्महि";
    aorist_mid_2: "आ~िष्ठाः", "आ~िषाथाम्", "आ~िढ्वम्";
    aorist_mid_3: "आ~िष्ट", "आ~िषाताम्", "आ~िषत";
    participle_fut: "अ~िष्यत्", "अ~िष्यमाण";
    participle_perf: "आ~िवांस्", "आ~ान";
    ),
    // अनंसीत: 4 words (अहासीत्, अतांसीत्, अयासीत्, अनंसीत्). Like आपत, except in 1 cell: aorist_act_3 pl.
    conjugation!("अनंसीत्", 4, "~सीत्", like आपत as AORIST_AS_PERFECT;
    stem act_sg: "~स- like अनंसीत्";
    aorist_act_3: _, _, "~सुः";
    ),
    // वमति: 4 words (वमति, चमति, नमति, गमति).
    conjugation!("वमति", 4, "~मति";
    stem present_act: "~- like वमति";
    stem act_sg: "~म- like वहति";
    stem present_mid: "~म- like वहति";
    stem imperfect: "~म- like वमति";
    ind_act_1: "~मामि", _, _;
    ind_act_2: _, "~मथः", "~मथ";
    ind_act_3: _, "~मतः", _;
    ind_mid_2: "~मBे", _, "~मध्वे";
    ind_mid_3: "~मते", _, _;
    imp_act_1: "~माAि", "~माव", "~माम";
    imp_act_2: _, "~मतम्", "~मत";
    imp_act_3: _, "~मताम्", _;
    imp_mid_1: "~मै", "~मावहै", "~मामहै";
    imp_mid_2: "~मB्व", _, "~मध्वम्";
    subj_act_2: "~माः", "~माथः", "~माथ";
    subj_act_3: "~मात्", "~मातः", "~मान्";
    subj_mid_2: "~मासे", "~मैथे", "~माध्वै";
    subj_mid_3: "~माते", "~मैते", "~मन्त";
    imperfect_act_1: "अ~मम्", _, _;
    imperfect_act_2: _, "अ~मतम्", "अ~मत";
    imperfect_act_3: _, "अ~मताम्", _;
    imperfect_mid_2: "अ~मथाः", _, "अ~मध्वम्";
    future_act_1: "~मिष्यामि", "~मिष्यावः", "~मिष्यामः";
    future_act_2: "~मिष्यसि", "~मिष्यथः", "~मिष्यथ";
    future_act_3: "~मिष्यति", "~मिष्यतः", "~मिष्यन्ति";
    future_mid_1: "~मिष्ये", "~मिष्यावहे", "~मिष्यामहे";
    future_mid_2: "~मिष्यसे", "~मिष्येथे", "~मिष्यध्वे";
    future_mid_3: "~मिष्यते", "~मिष्येते", "~मिष्यन्ते";
    conditional_act_1: "अ~मिष्यम्", "अ~मिष्याव", "अ~मिष्याम";
    conditional_act_2: "अ~मिष्यः", "अ~मिष्यतम्", "अ~मिष्यत";
    conditional_act_3: "अ~मिष्यत्", "अ~मिष्यताम्", "अ~मिष्यन्";
    conditional_mid_1: "अ~मिष्ये", "अ~मिष्यावहि", "अ~मिष्यामहि";
    conditional_mid_2: "अ~मिष्यथाः", "अ~मिष्येथाम्", "अ~मिष्यध्वम्";
    conditional_mid_3: _, "अ~मिष्येताम्", "अ~मिष्यन्त";
    perfect_act_1: "~चम", "~ेमिव", "~ेमिम";
    perfect_act_2: "~चA्थ", "~ेमथुः", "~ेम";
    perfect_act_3: _, "~ेमतुः", "~ेमुः";
    perfect_mid_1: "~ेमे", "~ेमिवहे", "~ेमिमहे";
    perfect_mid_2: "~ेमिषे", "~ेमाथे", "~ेमिध्वे";
    perfect_mid_3: _, "~ेमाते", "~ेमिरे";
    aorist_mid_1: "अ~मिषि", "अ~मिष्वहि", "अ~मिष्महि";
    aorist_mid_2: "अ~मिष्ठाः", "अ~मिषाथाम्", "अ~मिढ्वम्";
    aorist_mid_3: "अ~मिष्ट", "अ~मिषाताम्", "अ~मिषत";
    participle_fut: "~मिष्यत्", "~मिष्यमाण";
    participle_perf: "~ेमिवांस्", "~ेमाA";
    ),
    // वृणीते: 4 words (मिमीते, जानीते, जिहीते, वृणीते). Like जानाति, except in 41 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ….
    conjugation!("वृणीते", 4, "~ीते", like जानाति as ACT_AS_MID;
    ind_act_1: derived, derived, derived;
    ind_act_2: derived, derived, derived;
    ind_act_3: _, derived, derived;
    imp_act_1: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: _, derived, derived;
    imp_mid_3: "~ीताम्", _, _;
    opt_act_1: derived, derived, derived;
    opt_act_2: derived, derived, _;
    opt_act_3: derived, derived, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, derived, derived;
    imperfect_act_1: derived, derived, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: _, derived, derived;
    imperfect_mid_3: "अ~ीत", _, _;
    participle_pres: derived, _;
    ),
    // रवति: 4 words (द्रवति, चर्वति, तूर्वति, रवति).
    conjugation!("रवति", 4, "~वति";
    stem present_act: "~- like रवति";
    stem act_sg: "~- like रवति";
    stem present_mid: "~- like रवति";
    stem imperfect: "~व- like वहति";
    ind_act_1: "~वामि", _, _;
    ind_act_2: _, "~वथः", "~वथ";
    ind_act_3: _, "~वतः", _;
    ind_mid_2: "~वBे", _, "~वध्वे";
    ind_mid_3: "~वते", _, _;
    imp_act_1: "~वाAि", "~वाव", "~वाम";
    imp_act_2: _, "~वतम्", "~वत";
    imp_act_3: _, "~वताम्", _;
    imp_mid_1: "~वै", "~वावहै", "~वामहै";
    imp_mid_2: "~वB्व", _, "~वध्वम्";
    subj_act_2: "~वाः", "~वाथः", "~वाथ";
    subj_act_3: "~वात्", "~वातः", "~वान्";
    subj_mid_2: "~वासे", "~वैथे", "~वाध्वै";
    subj_mid_3: "~वाते", "~वैते", "~वन्त";
    imperfect_act_1: "अ~वम्", _, _;
    imperfect_act_2: _, "अ~वतम्", "अ~वत";
    imperfect_act_3: _, "अ~वताम्", _;
    imperfect_mid_2: "अ~वथाः", _, "अ~वध्वम्";
    future_act_1: "~ोष्यामि", "~ोष्यावः", "~ोष्यामः";
    future_act_2: "~ोष्यसि", "~ोष्यथः", "~ोष्यथ";
    future_act_3: "~ोष्यति", "~ोष्यतः", "~ोष्यन्ति";
    future_mid_1: "~ोष्ये", "~ोष्यावहे", "~ोष्यामहे";
    future_mid_2: "~ोष्यसे", "~ोष्येथे", "~ोष्यध्वे";
    future_mid_3: "~ोष्यते", "~ोष्येते", "~ोष्यन्ते";
    conditional_act_1: "अ~ोष्यम्", "अ~ोष्याव", "अ~ोष्याम";
    conditional_act_2: "अ~ोष्यः", "अ~ोष्यतम्", "अ~ोष्यत";
    conditional_act_3: "अ~ोष्यत्", "अ~ोष्यताम्", "अ~ोष्यन्";
    conditional_mid_1: "अ~ोष्ये", "अ~ोष्यावहि", "अ~ोष्यामहि";
    conditional_mid_2: "अ~ोष्यथाः", "अ~ोष्येथाम्", "अ~ोष्यध्वम्";
    conditional_mid_3: _, "अ~ोष्येताम्", "अ~ोष्यन्त";
    perfect_act_1: "दु~व", "दु~ुव", "दु~ुम";
    perfect_act_2: "दु~ोथ", "दु~ुवथुः", "दु~ुव";
    perfect_act_3: _, "दु~ुवतुः", "दु~ुवुः";
    perfect_mid_1: "दु~ुवे", "दु~ुवहे", "दु~ुमहे";
    perfect_mid_2: "दु~ुषे", "दु~ुवाथे", "दु~ुध्वे";
    perfect_mid_3: _, "दु~ुवाते", "दु~ुविरे";
    aorist_mid_1: "अ~ाविषि", "अ~ाविष्वहि", "अ~ाविष्महि";
    aorist_mid_2: "अ~ाविष्ठाः", "अ~ाविषाथाम्", "अ~ाविढ्वम्";
    aorist_mid_3: "अ~ाविष्ट", "अ~ाविषाताम्", "अ~ाविषत";
    participle_fut: "~ोष्यत्", "~ोष्यमाण";
    participle_perf: "दु~ुवांस्", "दु~ुवाण";
    ),
    // जगार: 4 words (जहार, दधार, जगार, ममार). Like आनाश, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("जगार", 4, "~ार", like आनाश;
    ind_act_3: "~ार", _, _;
    perfect_act_1: "~र|~ार", "~्रिव", "~्रिम";
    perfect_act_2: "~र्थ|~रिथ", "~्रथुः", "~्र";
    perfect_act_3: "~ार", "~्रतुः", "~्रुः";
    perfect_mid_1: "~्रे", "~्रिवहे", "~्रिमहे";
    perfect_mid_2: "~्रिषे", "~्राथे", "~्रिध्वे";
    perfect_mid_3: _, "~्राते", "~्रिरे";
    participle_perf: "~ृवांस्", "~्राA";
    ),
    // उवाच: 4 words (उवाच, उवाह, उवाप, उवाद). Like आनाश, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("उवाच", 4, "उवा~", like आनाश;
    ind_act_3: "उवा~", _, _;
    perfect_act_1: "उव~", "ऊ~िव", "ऊ~िम";
    perfect_act_2: "उव~्थ", "ऊ~थुः", "ऊ~";
    perfect_act_3: "उवा~", "ऊ~तुः", "ऊ~ुः";
    perfect_mid_1: "ऊ~े", "ऊ~िवहे", "ऊ~िमहे";
    perfect_mid_2: "ऊ~िषे", "ऊ~ाथे", "ऊ~िध्वे";
    perfect_mid_3: _, "ऊ~ाते", "ऊ~िरे";
    participle_perf: "ऊ~िवांस्", "ऊ~ान";
    ),
    // आनयति: 4 words (आनयति, आगच्छति, आक्रोशति, आमयति). Like साधयति, except in 26 cells: imperfect_act_1 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 du, imperfect_mid_2 sg, imperfect_mid_2 pl, ….
    conjugation!("आनयति", 4, "~ति", like साधयति as PERFECT_AS_AORIST;
    stem present_mid: "~- like वहति";
    stem imperfect: "~- like आनयति";
    imperfect_act_1: "~म्", _, _;
    imperfect_act_2: _, "~तम्", "~त";
    imperfect_act_3: _, "~ताम्", _;
    imperfect_mid_2: "~थाः", _, "~ध्वम्";
    future_act_1: _, _, "~िष्यामः";
    conditional_act_1: derived, derived, derived;
    conditional_act_2: derived, derived, derived;
    conditional_act_3: derived, derived, derived;
    conditional_mid_1: derived, derived, derived;
    conditional_mid_2: derived, derived, derived;
    conditional_mid_3: _, derived, derived;
    participle_perf: derived, derived;
    ),
    // चकार: 3 words (चकार, बभार, जभार). Like जगार, except in 7 cells: perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_mid_1 du, perfect_mid_1 pl, perfect_mid_2 sg, ….
    conjugation!("चकार", 3, "~ार", like जगार;
    perfect_act_1: _, "~ृव", "~ृम";
    perfect_act_2: "~र्थ", _, _;
    perfect_mid_1: _, "~ृवहे", "~ृमहे";
    perfect_mid_2: "~ृषे", _, "~ृध्वे";
    ),
    // आचरति: 3 words (आचरति, पक्षति, यक्षति). Like आनयति, except in 4 cells: infinitive, gerund, participle_fut act, participle_fut mid.
    conjugation!("आचरति", 3, "~ति", like आनयति as FUTURE_AS_AORIST;
    infinitive: derived;
    gerund: derived;
    participle_fut: derived, derived;
    ),
    // ईषटे: 3 words (ईष्टे, चष्टे, प्रचष्टे). Like चकरष, except in 19 cells: ind_act_3 sg, ind_mid_2 sg, ind_mid_2 pl, ind_mid_3 sg, imp_mid_1 sg, imp_mid_1 du, ….
    conjugation!("ईष्टे", 3, "~ष~टे", like चकरष as ACT_AS_MID;
    stem present_mid: "~क~ष- like ईष्टे";
    stem imperfect: "~क~ष- like ईष्टे";
    ind_act_3: "~ष~टे", _, _;
    ind_mid_2: "~क~षे", _, "~ड~ढ्वे";
    ind_mid_3: "~ष~टे", _, _;
    imp_mid_1: "~क~षै", "~क~षावहै", "~क~षामहै";
    imp_mid_2: "~क~ष्व", _, "~ड~ढ्वम्";
    imp_mid_3: "~ष~टाम्", _, _;
    subj_mid_2: "~क~षसे|~क~षासै", "~क~षैथे", "~क~षध्वे|~क~षाध्वै";
    subj_mid_3: "~क~षते|~क~षातै", "~क~षैते", "~क~षन्त|~क~षान्तै";
    imperfect_mid_2: "अ~ष~ठाः", _, "अ~ड~ढ्वम्";
    imperfect_mid_3: "अ~ष~ट", _, _;
    ),
    // ऋघायति: 3 words (ऋघायति, ऋच्छति, ऋध्यति). Like वेतति, except in 39 cells: ind_act_1 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ….
    conjugation!("ऋघायति", 3, "ऋ~ति", like वेतति;
    stem present_act: "~- like ऋघायति";
    stem present_mid: "~- like ऋघायति";
    stem imperfect: "~- like ऋघायति";
    ind_act_1: "ऋ~ामि", _, _;
    ind_act_2: _, "ऋ~थः", "ऋ~थ";
    ind_act_3: "ऋ~ति", "ऋ~तः", _;
    ind_mid_2: "ऋ~से", _, "ऋ~ध्वे";
    ind_mid_3: "ऋ~ते", _, _;
    imp_act_1: "ऋ~ानि", "ऋ~ाव", "ऋ~ाम";
    imp_act_2: "ऋ~", "ऋ~तम्", "ऋ~त";
    imp_act_3: "ऋ~तु", "ऋ~ताम्", _;
    imp_mid_1: "ऋ~ै", "ऋ~ावहै", "ऋ~ामहै";
    imp_mid_2: "ऋ~स्व", _, "ऋ~ध्वम्";
    subj_act_2: "ऋ~ाः|ऋ~ासि", "ऋ~ाथः", "ऋ~ाथ";
    subj_act_3: "ऋ~ात्|ऋ~ाति", "ऋ~ातः", "ऋ~ान्";
    subj_mid_2: "ऋ~ासे|ऋ~ासै", "ऋ~ैथे", "ऋ~ाध्वै";
    subj_mid_3: "ऋ~ाते|ऋ~ातै", "ऋ~ैते", "ऋ~न्त|ऋ~ान्तै";
    imperfect_act_1: "आर्~म्", _, _;
    imperfect_act_2: _, "आर्~तम्", "आर्~त";
    imperfect_act_3: _, "आर्~ताम्", _;
    imperfect_mid_2: "आर्~थाः", _, "आर्~ध्वम्";
    ),
    // बाधिषयते: 3 words (बाधिष्यते, श्यास्यते, रेक्ष्यते). Like एषयति, except in 20 cells: ind_act_3 sg, future_act_1 sg, future_act_1 du, future_act_1 pl, future_act_2 sg, future_act_2 du, ….
    conjugation!("बाधिष्यते", 3, "~ते", like एषयति as ACT_AS_MID;
    ind_act_3: "~ते", _, _;
    future_act_1: derived, derived, derived;
    future_act_2: derived, derived, derived;
    future_act_3: derived, derived, derived;
    conditional_act_1: derived, derived, derived;
    conditional_act_2: derived, derived, derived;
    conditional_act_3: _, derived, derived;
    conditional_mid_3: "अ~त", _, _;
    participle_fut: derived, _;
    ),
    // अधुकषत: 3 words (अधुक्षत्, अद्रुक्षत्, अरुक्षत्). Like आपत, except in 3 cells: aorist_mid_1 sg, aorist_mid_2 du, aorist_mid_3 du.
    conjugation!("अधुक्षत्", 3, "~त्", like आपत;
    aorist_mid_1: "~ि", _, _;
    aorist_mid_2: _, "~ाथाम्", _;
    aorist_mid_3: _, "~ाताम्", _;
    ),
    // दुदोह: 3 words (दुदोह, दुद्रोह, रुरोह). Like आनाश, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("दुदोह", 3, "~ोह", like आनाश;
    ind_act_3: "~ोह", _, _;
    perfect_act_1: "~ोह", "~ुहिव|~ुह्व", "~ुहिम|~ुह्म";
    perfect_act_2: "~ोहिथ", "~ुहथुः", "~ुह";
    perfect_act_3: "~ोह", "~ुहतुः", "~ुहुः";
    perfect_mid_1: "~ुहे", "~ुहिवहे|~ुह्वहे", "~ुहिमहे|~ुह्महे";
    perfect_mid_2: "~ुहिषे|~ुक्षे", "~ुहाथे", "~ुहिध्वे|~ूढ्वे";
    perfect_mid_3: _, "~ुहाते", "~ुहिरे|~ुह्रे";
    participle_perf: "~ुह्वांस्", "~ुहाA";
    ),
    // वेद: 3 words (वेद, विवेद, चिच्छेद). Like आनाश, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("वेद", 3, "~ेद", like आनाश;
    ind_act_3: "~ेद", _, _;
    perfect_act_1: "~ेद", "~िदिव|~िद्व", "~िदिम|~िद्म";
    perfect_act_2: "~ेदिथ", "~िदथुः", "~िद";
    perfect_act_3: "~ेद", "~िदतुः", "~िदुः";
    perfect_mid_1: "~िदे", "~िदिवहे|~िद्वहे", "~िदिमहे|~िद्महे";
    perfect_mid_2: "~िदिषे|~ित्से", "~िदाथे", "~िदिध्वे|~िद्ध्वे";
    perfect_mid_3: _, "~िदाते", "~िदिरे|~िद्रे";
    participle_perf: "~िद्वांस्", "~िदाA";
    ),
    // अचेत: 3 words (अश्रेत्, अचेत्, अहेत्). Like अनंसीत, except in 10 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("अचेत्", 3, "~ेत्", like अनंसीत;
    ind_act_3: "~ेत्", _, _;
    aorist_act_1: "~यम्", "~ेव", "~ेम";
    aorist_act_2: "~ेः", "~ेतम्", "~ेत";
    aorist_act_3: "~ेत्", "~ेताम्", "~्यA्";
    ),
    // रौति: 3 words (नौति, रौति, तौति). Like आपनोति, except in 6 cells: ind_act_1 sg, ind_act_2 sg, ind_act_3 sg, imp_act_3 sg, imperfect_act_2 sg, imperfect_act_3 sg.
    conjugation!("रौति", 3, "~ौति", like आपनोति;
    ind_act_1: "~ौमि", _, _;
    ind_act_2: "~ौBि", _, _;
    ind_act_3: "~ौति", _, _;
    imp_act_3: "~ौतु", _, _;
    imperfect_act_2: "अ~ौः", _, _;
    imperfect_act_3: "अ~ौत्", _, _;
    ),
    // अकर: 2 words (अकर्, अवर्). Like आपत, except in 10 cells: aorist_act_3 pl, aorist_mid_1 sg, aorist_mid_1 du, aorist_mid_1 pl, aorist_mid_2 sg, aorist_mid_2 du, ….
    conjugation!("अकर्", 2, "अ~र्", like आपत;
    stem act_sg: "~र- like अकर्";
    aorist_act_3: _, _, "अ~्रन्";
    aorist_mid_1: "अ~्रि", "अ~ृवहि", "अ~ृमहि";
    aorist_mid_2: "अ~ृथाः", "अ~्राथाम्", "अ~ृध्वम्";
    aorist_mid_3: "अ~ृत", "अ~्राताम्", "अ~्रत";
    ),
    // अगन: 2 words (अगन्, अतन्). Like आपत, except in 16 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("अगन्", 2, "~न्", like आपत;
    ind_act_3: "~न्", _, _;
    aorist_act_1: "~Aम्", "~न्व", "~न्म";
    aorist_act_2: "~न्", "~न्तम्", "~न्त";
    aorist_act_3: "~न्", "~न्ताम्", "~्नन्";
    aorist_mid_1: "~्Aि", "~न्वहि", "~न्महि";
    aorist_mid_2: _, "~्Aाथाम्", _;
    aorist_mid_3: _, "~्Aाताम्", "~्Aत";
    ),
    // आरत: 2 words (अजन्त, आर्त). Like आपत, except in 10 cells: ind_act_3 sg, aorist_mid_1 sg, aorist_mid_1 du, aorist_mid_1 pl, aorist_mid_2 sg, aorist_mid_2 du, ….
    conjugation!("आर्त", 2, "~्त", like आपत as AORIST_AS_PERFECT;
    ind_act_3: "~्त", _, _;
    aorist_mid_1: "~ि", "~्वहि", "~्महि";
    aorist_mid_2: "~्थाः", "~ाथाम्", "~्ध्वम्";
    aorist_mid_3: "~्त", "~ाताम्", "~त";
    ),
    // अभोक: 2 words (अयोक्, अभोक्). Like आपत, except in 2 cells: aorist_act_3 pl, aorist_mid_2 pl.
    conjugation!("अभोक्", 2, "~ोक्", like आपत;
    stem present_ind_act_1_sg: "~ु- like अभोक्";
    stem act_sg: "~ो- like अभोक्";
    aorist_act_3: _, _, "~ुजA्";
    aorist_mid_2: _, _, "~ुग्ध्वम्";
    ),
    // आनट: 2 words (आनट्, अक्रन्). Like चकरष, except in 1 cell: aorist_act_3 pl.
    conjugation!("आनट्", 2, "~~", like चकरष as ACT_AS_MID;
    stem act_sg: "~~- like आनट्";
    aorist_act_3: _, _, "~द~";
    ),
    // अमृत: 1 words (अमृत). Like आरत, except in 10 cells: ind_act_3 sg, aorist_mid_1 sg, aorist_mid_1 du, aorist_mid_1 pl, aorist_mid_2 sg, aorist_mid_2 du, ….
    conjugation!("अमृत", 1, "~ृत", like आरत;
    ind_act_3: "~ृत", _, _;
    aorist_mid_1: "~्रि", "~ृवहि", "~ृमहि";
    aorist_mid_2: "~ृथाः", "~्राथाम्", "~ृध्वम्";
    aorist_mid_3: "~ृत", "~्राताम्", "~्रत";
    ),
    // संतापयति: 1 words (संतापयति).
    conjugation!("संतापयति", 1, "~ं~ति";
    stem present_act: "~न्~- like साधयति";
    stem present_mid: "~न्~- like संतापयति";
    stem imperfect: "~न्~- like वहति";
    ind_act_1: "~न्~ामि", _, _;
    ind_act_2: _, "~न्~थः", "~न्~थ";
    ind_act_3: "~न्~ति", "~न्~तः", _;
    ind_mid_2: "~न्~से", _, "~न्~ध्वे";
    ind_mid_3: "~न्~ते", _, _;
    imp_act_1: "~न्~ानि", "~न्~ाव", "~न्~ाम";
    imp_act_2: "~न्~", "~न्~तम्", "~न्~त";
    imp_act_3: "~न्~तु", "~न्~ताम्", _;
    imp_mid_1: "~न्~ै", "~न्~ावहै", "~न्~ामहै";
    imp_mid_2: "~न्~स्व", _, "~न्~ध्वम्";
    subj_act_2: "~न्~ाः|~न्~ासि", "~न्~ाथः", "~न्~ाथ";
    subj_act_3: "~न्~ात्|~न्~ाति", "~न्~ातः", "~न्~ान्";
    subj_mid_2: "~न्~ासे|~न्~ासै", "~न्~ैथे", "~न्~ाध्वै";
    subj_mid_3: "~न्~ाते|~न्~ातै", "~न्~ैते", "~न्~न्त|~न्~ान्तै";
    imperfect_act_1: "अ~न्~म्", _, _;
    imperfect_act_2: _, "अ~न्~तम्", "अ~न्~त";
    imperfect_act_3: _, "अ~न्~ताम्", _;
    imperfect_mid_2: "अ~न्~थाः", _, "अ~न्~ध्वम्";
    future_act_1: "~न्~िष्यामि", "~न्~िष्यावः", "~न्~िष्यामः|~न्~िष्यामसि";
    future_act_2: "~न्~िष्यसि", "~न्~िष्यथः", "~न्~िष्यथ";
    future_act_3: "~न्~िष्यति", "~न्~िष्यतः", "~न्~िष्यन्ति";
    future_mid_1: "~न्~िष्ये", "~न्~िष्यावहे", "~न्~िष्यामहे";
    future_mid_2: "~न्~िष्यसे", "~न्~िष्येथे", "~न्~िष्यध्वे";
    future_mid_3: "~न्~िष्यते", "~न्~िष्येते", "~न्~िष्यन्ते";
    conditional_act_1: "अ~न्~िष्यम्", "अ~न्~िष्याव", "अ~न्~िष्याम";
    conditional_act_2: "अ~न्~िष्यः", "अ~न्~िष्यतम्", "अ~न्~िष्यत";
    conditional_act_3: "अ~न्~िष्यत्", "अ~न्~िष्यताम्", "अ~न्~िष्यन्";
    conditional_mid_1: "अ~न्~िष्ये", "अ~न्~िष्यावहि", "अ~न्~िष्यामहि";
    conditional_mid_2: "अ~न्~िष्यथाः", "अ~न्~िष्येथाम्", "अ~न्~िष्यध्वम्";
    conditional_mid_3: _, "अ~न्~िष्येताम्", "अ~न्~िष्यन्त";
    perfect_act_1: "~न्~ामास|~न्~ांचकर", "~न्~ामासिव|~न्~ांचकृव", "~न्~ामासिम|~न्~ांचकृम";
    perfect_act_2: "~न्~ामासिथ|~न्~ांचकर्थ", "~न्~ामासथुः|~न्~ांचक्रथुः", "~न्~ामास|~न्~ांचक्र";
    perfect_act_3: _, "~न्~ामासतुः|~न्~ांचक्रतुः", "~न्~ामासुः|~न्~ांचक्रुः";
    perfect_mid_1: "~न्~ांचक्रे", "~न्~ांचकृवहे", "~न्~ांचकृमहे";
    perfect_mid_2: "~न्~ांचकृषे", "~न्~ांचक्राथे", "~न्~ांचकृध्वे";
    perfect_mid_3: _, "~न्~ांचक्राते", "~न्~ांचक्रिरे";
    participle_fut: "~न्~िष्यत्", "~न्~िष्यमाण";
    participle_perf: "~न्~ामासिवांस्|~न्~ांचकृवांस्", "~न्~ांचक्राण";
    ),
    // अलेकिषयत: 1 words (अलेकिष्यत्). Like आपत, except in 9 cells: conditional_act_1 sg, conditional_act_1 du, conditional_act_1 pl, conditional_act_2 sg, conditional_act_2 du, conditional_act_2 pl, ….
    conjugation!("अलेकिष्यत्", 1, "~त्", like आपत as AORIST_AS_PERFECT;
    conditional_act_1: "~म्", "~ाव", "~ाम";
    conditional_act_2: "~ः", "~तम्", "~त";
    conditional_act_3: "~त्", "~ताम्", "~न्";
    ),
    // अपराक: 1 words (अप्राक्). Like आनट, except in 2 cells: aorist_act_3 pl, aorist_mid_2 pl.
    conjugation!("अप्राक्", 1, "~~राक्", like आनट;
    stem present_ind_act_1_sg: "~ृक~- like अमौक्";
    stem act_sg: "~~राक्- like अमौक्";
    aorist_act_3: _, _, "~~राक्षुः";
    aorist_mid_2: _, _, "~ृग~ध्वम्";
    ),
    // अमौक: 1 words (अमौक्). Like आनट, except in 2 cells: aorist_act_3 pl, aorist_mid_2 pl.
    conjugation!("अमौक्", 1, "~ौक~", like आनट;
    stem present_ind_act_1_sg: "~ुक~- like अमौक्";
    stem act_sg: "~ौक~- like अमौक्";
    aorist_act_3: _, _, "~ौक~षुः";
    aorist_mid_2: _, _, "~ुग~ध्वम्";
    ),
    // अमोक: 1 words (अमोक्). Like अभोक, except in 10 cells: aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_3 pl, aorist_mid_1 sg, aorist_mid_1 du, ….
    conjugation!("अमोक्", 1, "~ोक्", like अभोक;
    aorist_act_1: "~ोचम्", "~ोच्व", "~ोच्म";
    aorist_act_3: _, _, "~ुचA्";
    aorist_mid_1: "~ुचि", "~ुच्वहि", "~ुच्महि";
    aorist_mid_2: _, "~ुचाथाम्", _;
    aorist_mid_3: _, "~ुचाताम्", "~ुचत";
    ),
    // अमत: 1 words (अमत). Like आरत, except in 9 cells: ind_act_3 sg, aorist_mid_1 sg, aorist_mid_1 du, aorist_mid_1 pl, aorist_mid_2 sg, aorist_mid_2 du, ….
    conjugation!("अमत", 1, "~त", like आरत as SG_AS_PL;
    ind_act_3: "~त", _, _;
    aorist_mid_1: "~्नि", "~न्वहि", "~न्महि";
    aorist_mid_2: "~थाः", "~्नाथाम्", "~ध्वम्";
    aorist_mid_3: _, "~्नाताम्", "~्नत";
    ),
    // अवरक: 1 words (अवर्क्). Like आपत, except in 11 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("अवर्क्", 1, "~र्क्", like आपत;
    stem present_ind_act_1_sg: "~ृ- like अभोक्";
    ind_act_3: "~र्क्", _, _;
    aorist_act_1: "~र्जम्", "~र्ज्व", "~र्ज्म";
    aorist_act_2: "~र्क्", "~र्क्तम्", "~र्क्त";
    aorist_act_3: "~र्क्|~ृक्", "~र्क्ताम्", "~ृजन्";
    aorist_mid_2: _, _, "~ृग्ध्वम्";
    ),
    // असवार: 1 words (अस्वार्). Like अनंसीत, except in 10 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("अस्वार्", 1, "~र्", like अनंसीत;
    ind_act_3: "~र्", _, _;
    aorist_act_1: "~र्षम्", "~र्ष्व", "~र्ष्म";
    aorist_act_2: "~ः", "~र्ष्टम्", "~र्ष्ट";
    aorist_act_3: "~ः", "~र्ष्टाम्", "~र्षुः";
    ),
    // अदरक: 1 words (अदर्क्). Like आनट, except in 10 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("अदर्क्", 1, "~र~क्", like आनट;
    ind_act_3: "~र~क्", _, _;
    aorist_act_1: "~र~शम्", "~र~श्व", "~र~श्म|~ृश~म";
    aorist_act_2: "~र~क्", "~र~ष्टम्", "~र~ष्ट";
    aorist_act_3: "~र~क्", "~र~ष्टाम्", "~ृशन~|~र~शुः";
    ),
    // अजनिषट: 1 words (अजनिष्ट). Like आरत, except in 10 cells: ind_act_3 sg, aorist_mid_1 sg, aorist_mid_1 du, aorist_mid_1 pl, aorist_mid_2 sg, aorist_mid_2 du, ….
    conjugation!("अजनिष्ट", 1, "~ष्ट", like आरत;
    ind_act_3: "~ष्ट", _, _;
    aorist_mid_1: "~Bि", "~B्वहि", "~B्महि";
    aorist_mid_2: "~B्ठाः", "~Bाथाम्", "~ढ्वम्";
    aorist_mid_3: "~ष्ट", "~Bाताम्", "~Bत";
    ),
    // अपतत: 1 words (अपत्त). Like आरत, except in 10 cells: ind_act_3 sg, aorist_mid_1 sg, aorist_mid_1 du, aorist_mid_1 pl, aorist_mid_2 sg, aorist_mid_2 du, ….
    conjugation!("अपत्त", 1, "~त्त", like आरत;
    ind_act_3: "~त्त", _, _;
    aorist_mid_1: "~दि|~त्सि", "~द्वहि|~त्स्वहि", "~द्महि|~त्स्महि";
    aorist_mid_2: "~त्थाः", "~दाथाम्|~त्साथाम्", "~द्ध्वम्";
    aorist_mid_3: "~त्त", "~दाताम्|~त्साताम्", "~दत|~त्सत";
    ),
    // अजुषत: 1 words (अजुषत्). Like अनंसीत, except in 10 cells: ind_act_3 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, aorist_act_2 pl, ….
    conjugation!("अजुषत्", 1, "~षत्", like अनंसीत;
    stem present_ind_act_1_sg: "~स- like आपत्";
    ind_act_3: "~षत्", _, _;
    aorist_act_1: _, "~साव", "~साम";
    aorist_act_2: "~सः", "~सतम्", "~सत";
    aorist_act_3: "~सत्", "~सताम्", "~सन्";
    aorist_mid_2: _, _, "~सध्वम्";
    ),
];
