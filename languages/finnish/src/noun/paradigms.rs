//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`A`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `par` (sg, pl)
/// `ine` (sg, pl)
/// `ela` (sg, pl)
/// `ill` (sg, pl)
/// `ade` (sg, pl)
/// `abl` (sg, pl)
/// `all` (sg, pl)
/// `ess` (sg, pl)
/// `tra` (sg, pl)
/// `abe` (sg, pl)
/// `ins` (sg, pl)
/// `com` (sg, pl)
/// `possessive_nom_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_nom_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_gen_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_gen_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_par_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_par_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ine_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ine_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ela_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ela_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ill_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ill_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ade_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ade_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_abl_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_abl_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_all_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_all_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ess_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ess_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_tra_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_tra_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_abe_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_abe_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ins_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_ins_pl` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_com_sg` (_1sg, _2sg, _1pl, _2pl, _3)
/// `possessive_com_pl` (_1sg, _2sg, _1pl, _2pl, _3)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_nom_sg: $s0:tt;)?
     $(stem pl: $s1:tt;)?
     $(stem noun_gen_pl: $s2:tt;)?
     $(stem sg: $s3:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(par: $c4:tt, $c5:tt;)?
     $(ine: $c6:tt, $c7:tt;)?
     $(ela: $c8:tt, $c9:tt;)?
     $(ill: $c10:tt, $c11:tt;)?
     $(ade: $c12:tt, $c13:tt;)?
     $(abl: $c14:tt, $c15:tt;)?
     $(all: $c16:tt, $c17:tt;)?
     $(ess: $c18:tt, $c19:tt;)?
     $(tra: $c20:tt, $c21:tt;)?
     $(abe: $c22:tt, $c23:tt;)?
     $(ins: $c24:tt, $c25:tt;)?
     $(com: $c26:tt, $c27:tt;)?
     $(possessive_nom_sg: $c28:tt, $c29:tt, $c30:tt, $c31:tt, $c32:tt;)?
     $(possessive_nom_pl: $c33:tt, $c34:tt, $c35:tt, $c36:tt, $c37:tt;)?
     $(possessive_gen_sg: $c38:tt, $c39:tt, $c40:tt, $c41:tt, $c42:tt;)?
     $(possessive_gen_pl: $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(possessive_par_sg: $c48:tt, $c49:tt, $c50:tt, $c51:tt, $c52:tt;)?
     $(possessive_par_pl: $c53:tt, $c54:tt, $c55:tt, $c56:tt, $c57:tt;)?
     $(possessive_ine_sg: $c58:tt, $c59:tt, $c60:tt, $c61:tt, $c62:tt;)?
     $(possessive_ine_pl: $c63:tt, $c64:tt, $c65:tt, $c66:tt, $c67:tt;)?
     $(possessive_ela_sg: $c68:tt, $c69:tt, $c70:tt, $c71:tt, $c72:tt;)?
     $(possessive_ela_pl: $c73:tt, $c74:tt, $c75:tt, $c76:tt, $c77:tt;)?
     $(possessive_ill_sg: $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt;)?
     $(possessive_ill_pl: $c83:tt, $c84:tt, $c85:tt, $c86:tt, $c87:tt;)?
     $(possessive_ade_sg: $c88:tt, $c89:tt, $c90:tt, $c91:tt, $c92:tt;)?
     $(possessive_ade_pl: $c93:tt, $c94:tt, $c95:tt, $c96:tt, $c97:tt;)?
     $(possessive_abl_sg: $c98:tt, $c99:tt, $c100:tt, $c101:tt, $c102:tt;)?
     $(possessive_abl_pl: $c103:tt, $c104:tt, $c105:tt, $c106:tt, $c107:tt;)?
     $(possessive_all_sg: $c108:tt, $c109:tt, $c110:tt, $c111:tt, $c112:tt;)?
     $(possessive_all_pl: $c113:tt, $c114:tt, $c115:tt, $c116:tt, $c117:tt;)?
     $(possessive_ess_sg: $c118:tt, $c119:tt, $c120:tt, $c121:tt, $c122:tt;)?
     $(possessive_ess_pl: $c123:tt, $c124:tt, $c125:tt, $c126:tt, $c127:tt;)?
     $(possessive_tra_sg: $c128:tt, $c129:tt, $c130:tt, $c131:tt, $c132:tt;)?
     $(possessive_tra_pl: $c133:tt, $c134:tt, $c135:tt, $c136:tt, $c137:tt;)?
     $(possessive_abe_sg: $c138:tt, $c139:tt, $c140:tt, $c141:tt, $c142:tt;)?
     $(possessive_abe_pl: $c143:tt, $c144:tt, $c145:tt, $c146:tt, $c147:tt;)?
     $(possessive_ins_sg: $c148:tt, $c149:tt, $c150:tt, $c151:tt, $c152:tt;)?
     $(possessive_ins_pl: $c153:tt, $c154:tt, $c155:tt, $c156:tt, $c157:tt;)?
     $(possessive_com_sg: $c158:tt, $c159:tt, $c160:tt, $c161:tt, $c162:tt;)?
     $(possessive_com_pl: $c163:tt, $c164:tt, $c165:tt, $c166:tt, $c167:tt;)?
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

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`par_as_abe`: one value read as another).
const PAR_AS_ABE: [u16; 168] = [
    0, 1, 2, 3, 22, 23, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 138,
    139, 140, 141, 142, 143, 144, 145, 146, 147, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
    70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
    94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113,
    114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
    133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151,
    152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`nom_as_com`: one value read as another).
const NOM_AS_COM: [u16; 168] = [
    26, 27, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 38, 39, 40, 41, 42, 43, 44, 45, 46,
    47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
    71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94,
    95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114,
    115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133,
    134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 168] = [
    0, 0, 2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 12, 12, 14, 14, 16, 16, 18, 18, 20, 20, 22, 22, 24, 24,
    26, 26, 28, 29, 30, 31, 32, 28, 29, 30, 31, 32, 38, 39, 40, 41, 42, 38, 39, 40, 41, 42, 48, 49,
    50, 51, 52, 48, 49, 50, 51, 52, 58, 59, 60, 61, 62, 58, 59, 60, 61, 62, 68, 69, 70, 71, 72, 68,
    69, 70, 71, 72, 78, 79, 80, 81, 82, 78, 79, 80, 81, 82, 88, 89, 90, 91, 92, 88, 89, 90, 91, 92,
    98, 99, 100, 101, 102, 98, 99, 100, 101, 102, 108, 109, 110, 111, 112, 108, 109, 110, 111, 112,
    118, 119, 120, 121, 122, 118, 119, 120, 121, 122, 128, 129, 130, 131, 132, 128, 129, 130, 131,
    132, 138, 139, 140, 141, 142, 138, 139, 140, 141, 142, 148, 149, 150, 151, 152, 148, 149, 150,
    151, 152, 158, 159, 160, 161, 162, 158, 159, 160, 161, 162,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 168] = [
    1, 1, 3, 3, 5, 5, 7, 7, 9, 9, 11, 11, 13, 13, 15, 15, 17, 17, 19, 19, 21, 21, 23, 23, 25, 25,
    27, 27, 33, 34, 35, 36, 37, 33, 34, 35, 36, 37, 43, 44, 45, 46, 47, 43, 44, 45, 46, 47, 53, 54,
    55, 56, 57, 53, 54, 55, 56, 57, 63, 64, 65, 66, 67, 63, 64, 65, 66, 67, 73, 74, 75, 76, 77, 73,
    74, 75, 76, 77, 83, 84, 85, 86, 87, 83, 84, 85, 86, 87, 93, 94, 95, 96, 97, 93, 94, 95, 96, 97,
    103, 104, 105, 106, 107, 103, 104, 105, 106, 107, 113, 114, 115, 116, 117, 113, 114, 115, 116,
    117, 123, 124, 125, 126, 127, 123, 124, 125, 126, 127, 133, 134, 135, 136, 137, 133, 134, 135,
    136, 137, 143, 144, 145, 146, 147, 143, 144, 145, 146, 147, 153, 154, 155, 156, 157, 153, 154,
    155, 156, 157, 163, 164, 165, 166, 167, 163, 164, 165, 166, 167,
];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `purkaminen`: 519 words.
    Purkaminen,
    /// Like `taso`: 487 words.
    Taso,
    /// Like `tunnus`: 401 words.
    Tunnus,
    /// Like `pari`: 334 words.
    Pari,
    /// Like `osa`: 318 words.
    Osa,
    /// Like `sarja`: 311 words.
    Sarja,
    /// Like `ankkuri`: 255 words.
    Ankkuri,
    /// Like `ykkönen`: 207 words.
    Ykkönen,
    /// Like `erä`: 169 words.
    Erä,
    /// Like `nappula`: 156 words.
    Nappula,
    /// Like `verkko`: 144 words.
    Verkko,
    /// Like `oikeus`: 135 words.
    Oikeus,
    /// Like `keri`: 122 words.
    Keri,
    /// Like `tykitys`: 115 words.
    Tykitys,
    /// Like `aste`: 113 words.
    Aste,
    /// Like `konversio`: 105 words.
    Konversio,
    /// Like `kortti`: 104 words.
    Kortti,
    /// Like `levy`: 99 words.
    Levy,
    /// Like `paikka`: 71 words.
    Paikka,
    /// Like `kanta`: 71 words.
    Kanta,
    /// Like `veto`: 60 words.
    Veto,
    /// Like `piste`: 56 words.
    Piste,
    /// Like `yhteys`: 56 words.
    Yhteys,
    /// Like `heitto`: 52 words.
    Heitto,
    /// Like `silmukka`: 50 words.
    Silmukka,
    /// Like `pukki`: 50 words.
    Pukki,
    /// Like `kota`: 49 words.
    Kota,
    /// Like `avain`: 46 words.
    Avain,
    /// Like `kuori`: 43 words.
    Kuori,
    /// Like `ennakointi`: 43 words.
    Ennakointi,
    /// Like `piikki`: 42 words.
    Piikki,
    /// Like `luokka`: 40 words.
    Luokka,
    /// Like `asento`: 40 words.
    Asento,
    /// Like `lippu`: 33 words.
    Lippu,
    /// Like `tappi`: 31 words.
    Tappi,
    /// Like `tekijä`: 30 words.
    Tekijä,
    /// Like `kivi`: 29 words.
    Kivi,
    /// Like `lähetti`: 29 words.
    Lähetti,
    /// Like `parvi`: 29 words.
    Parvi,
    /// Like `tarvas`: 25 words.
    Tarvas,
    /// Like `kausi`: 25 words.
    Kausi,
    /// Like `osoitin`: 24 words.
    Osoitin,
    /// Like `kate`: 23 words.
    Kate,
    /// Like `rauta`: 21 words.
    Rauta,
    /// Like `hmong`: 20 words.
    Hmong,
    /// Like `suhde`: 19 words.
    Suhde,
    /// Like `hätä`: 19 words.
    Hätä,
    /// Like `peite`: 18 words.
    Peite,
    /// Like `kytkin`: 18 words.
    Kytkin,
    /// Like `yksikkö`: 18 words.
    Yksikkö,
    /// Like `etumies`: 17 words.
    Etumies,
    /// Like `neliö`: 17 words.
    Neliö,
    /// Like `käsi`: 17 words.
    Käsi,
    /// Like `luku`: 17 words.
    Luku,
    /// Like `vaippa`: 16 words.
    Vaippa,
    /// Like `lapa`: 16 words.
    Lapa,
    /// Like `keitin`: 15 words.
    Keitin,
    /// Like `ruis`: 15 words.
    Ruis,
    /// Like `lehti`: 14 words.
    Lehti,
    /// Like `leipä`: 14 words.
    Leipä,
    /// Like `kupu`: 14 words.
    Kupu,
    /// Like `aika`: 13 words.
    Aika,
    /// Like `tolppa`: 13 words.
    Tolppa,
    /// Like `kymppi`: 13 words.
    Kymppi,
    /// Like `kaanon`: 13 words.
    Kaanon,
    /// Like `olot`: 13 words.
    Olot,
    /// Like `lähde`: 12 words.
    Lähde,
    /// Like `syöttö`: 12 words.
    Syöttö,
    /// Like `selkä`: 12 words.
    Selkä,
    /// Like `siemen`: 12 words.
    Siemen,
    /// Like `pää`: 11 words.
    Pää,
    /// Like `siipi`: 11 words.
    Siipi,
    /// Like `runko`: 11 words.
    Runko,
    /// Like `hattara`: 10 words.
    Hattara,
    /// Like `tunne`: 10 words.
    Tunne,
    /// Like `lyhty`: 10 words.
    Lyhty,
    /// Like `makonde`: 9 words.
    Makonde,
    /// Like `työ`: 9 words.
    Työ,
    /// Like `lanka`: 9 words.
    Lanka,
    /// Like `perintä`: 9 words.
    Perintä,
    /// Like `sakset`: 9 words.
    Sakset,
    /// Like `kieli`: 8 words.
    Kieli,
    /// Like `jousi`: 8 words.
    Jousi,
    /// Like `hiili`: 8 words.
    Hiili,
    /// Like `työntö`: 8 words.
    Työntö,
    /// Like `ruoti`: 7 words.
    Ruoti,
    /// Like `lupa`: 7 words.
    Lupa,
    /// Like `toimi`: 7 words.
    Toimi,
    /// Like `kierto`: 7 words.
    Kierto,
    /// Like `rengas`: 7 words.
    Rengas,
    /// Like `väki`: 7 words.
    Väki,
    /// Like `koi`: 7 words.
    Koi,
    /// Like `keihäs`: 6 words.
    Keihäs,
    /// Like `kuningatar`: 6 words.
    Kuningatar,
    /// Like `ryyppy`: 6 words.
    Ryyppy,
    /// Like `kanapee`: 6 words.
    Kanapee,
    /// Like `aalto`: 6 words.
    Aalto,
    /// Like `värit`: 6 words.
    Värit,
    /// Like `nousu`: 5 words.
    Nousu,
    /// Like `koe`: 5 words.
    Koe,
    /// Like `liike`: 5 words.
    Liike,
    /// Like `äiti`: 5 words.
    Äiti,
    /// Like `jänne`: 5 words.
    Jänne,
    /// Like `ewe`: 5 words.
    Ewe,
    /// Like `khmer`: 5 words.
    Khmer,
    /// Like `tee`: 5 words.
    Tee,
    /// Like `taival`: 5 words.
    Taival,
    /// Like `syy`: 5 words.
    Syy,
    /// Like `varras`: 5 words.
    Varras,
    /// Like `kärki`: 5 words.
    Kärki,
    /// Like `askel`: 5 words.
    Askel,
    /// Like `alkeet`: 5 words.
    Alkeet,
    /// Like `kissanristiäiset`: 5 words.
    Kissanristiäiset,
    /// Like `muunnin`: 5 words.
    Muunnin,
    /// Like `poika`: 4 words.
    Poika,
    /// Like `hopea`: 4 words.
    Hopea,
    /// Like `uurre`: 4 words.
    Uurre,
    /// Like `ydin`: 4 words.
    Ydin,
    /// Like `kauris`: 4 words.
    Kauris,
    /// Like `kurki`: 4 words.
    Kurki,
    /// Like `rättänä`: 4 words.
    Rättänä,
    /// Like `takuu`: 4 words.
    Takuu,
    /// Like `vilunki`: 4 words.
    Vilunki,
    /// Like `käräjät`: 4 words.
    Käräjät,
    /// Like `vehje`: 4 words.
    Vehje,
    /// Like `onki`: 4 words.
    Onki,
    /// Like `tuki`: 3 words.
    Tuki,
    /// Like `body`: 3 words.
    Body,
    /// Like `hönkä`: 3 words.
    Hönkä,
    /// Like `hammas`: 3 words.
    Hammas,
    /// Like `piennar`: 3 words.
    Piennar,
    /// Like `komme`: 3 words.
    Komme,
    /// Like `leve`: 3 words.
    Leve,
    /// Like `korpi`: 3 words.
    Korpi,
    /// Like `renki`: 3 words.
    Renki,
    /// Like `bokserit`: 3 words.
    Bokserit,
    /// Like `opas`: 3 words.
    Opas,
    /// Like `lapsi`: 3 words.
    Lapsi,
    /// Like `markkinat`: 3 words.
    Markkinat,
    /// Like `kierre`: 3 words.
    Kierre,
    /// Like `ahdin`: 3 words.
    Ahdin,
    /// Like `valvojaiset`: 3 words.
    Valvojaiset,
    /// Like `hässäkkä`: 3 words.
    Hässäkkä,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 143] = [
        Declension::Purkaminen,
        Declension::Taso,
        Declension::Tunnus,
        Declension::Pari,
        Declension::Osa,
        Declension::Sarja,
        Declension::Ankkuri,
        Declension::Ykkönen,
        Declension::Erä,
        Declension::Nappula,
        Declension::Verkko,
        Declension::Oikeus,
        Declension::Keri,
        Declension::Tykitys,
        Declension::Aste,
        Declension::Konversio,
        Declension::Kortti,
        Declension::Levy,
        Declension::Paikka,
        Declension::Kanta,
        Declension::Veto,
        Declension::Piste,
        Declension::Yhteys,
        Declension::Heitto,
        Declension::Silmukka,
        Declension::Pukki,
        Declension::Kota,
        Declension::Avain,
        Declension::Kuori,
        Declension::Ennakointi,
        Declension::Piikki,
        Declension::Luokka,
        Declension::Asento,
        Declension::Lippu,
        Declension::Tappi,
        Declension::Tekijä,
        Declension::Kivi,
        Declension::Lähetti,
        Declension::Parvi,
        Declension::Tarvas,
        Declension::Kausi,
        Declension::Osoitin,
        Declension::Kate,
        Declension::Rauta,
        Declension::Hmong,
        Declension::Suhde,
        Declension::Hätä,
        Declension::Peite,
        Declension::Kytkin,
        Declension::Yksikkö,
        Declension::Etumies,
        Declension::Neliö,
        Declension::Käsi,
        Declension::Luku,
        Declension::Vaippa,
        Declension::Lapa,
        Declension::Keitin,
        Declension::Ruis,
        Declension::Lehti,
        Declension::Leipä,
        Declension::Kupu,
        Declension::Aika,
        Declension::Tolppa,
        Declension::Kymppi,
        Declension::Kaanon,
        Declension::Olot,
        Declension::Lähde,
        Declension::Syöttö,
        Declension::Selkä,
        Declension::Siemen,
        Declension::Pää,
        Declension::Siipi,
        Declension::Runko,
        Declension::Hattara,
        Declension::Tunne,
        Declension::Lyhty,
        Declension::Makonde,
        Declension::Työ,
        Declension::Lanka,
        Declension::Perintä,
        Declension::Sakset,
        Declension::Kieli,
        Declension::Jousi,
        Declension::Hiili,
        Declension::Työntö,
        Declension::Ruoti,
        Declension::Lupa,
        Declension::Toimi,
        Declension::Kierto,
        Declension::Rengas,
        Declension::Väki,
        Declension::Koi,
        Declension::Keihäs,
        Declension::Kuningatar,
        Declension::Ryyppy,
        Declension::Kanapee,
        Declension::Aalto,
        Declension::Värit,
        Declension::Nousu,
        Declension::Koe,
        Declension::Liike,
        Declension::Äiti,
        Declension::Jänne,
        Declension::Ewe,
        Declension::Khmer,
        Declension::Tee,
        Declension::Taival,
        Declension::Syy,
        Declension::Varras,
        Declension::Kärki,
        Declension::Askel,
        Declension::Alkeet,
        Declension::Kissanristiäiset,
        Declension::Muunnin,
        Declension::Poika,
        Declension::Hopea,
        Declension::Uurre,
        Declension::Ydin,
        Declension::Kauris,
        Declension::Kurki,
        Declension::Rättänä,
        Declension::Takuu,
        Declension::Vilunki,
        Declension::Käräjät,
        Declension::Vehje,
        Declension::Onki,
        Declension::Tuki,
        Declension::Body,
        Declension::Hönkä,
        Declension::Hammas,
        Declension::Piennar,
        Declension::Komme,
        Declension::Leve,
        Declension::Korpi,
        Declension::Renki,
        Declension::Bokserit,
        Declension::Opas,
        Declension::Lapsi,
        Declension::Markkinat,
        Declension::Kierre,
        Declension::Ahdin,
        Declension::Valvojaiset,
        Declension::Hässäkkä,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        DECLENSIONS.get(self as usize)
    }
}

impl flexion::Label for Declension {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static DECLENSIONS: [Paradigm<'static>; 143] = [
    // Purkaminen: 519 words (nainen, suomalainen, valkoinen, punainen, hevonen, lautanen).
    declension!("purkaminen", 519, "~nen";
    stem pl: "~s- like purkaminen";
    stem noun_gen_pl: "~s- like purkaminen";
    nom: "~nen", _;
    par: "~stA", _;
    ill: "~seen", _;
    ess: "~senA", _;
    possessive_nom_sg: "~seni", "~sesi", "~semme", "~senne", "~sensA";
    possessive_par_sg: "~stAni", "~stAsi", "~stAmme", "~stAnne", "~stAAn|~stansa";
    possessive_ill_sg: "~seeni", "~seesi", "~seemme", "~seenne", "~seensA";
    possessive_ess_sg: "~senAni", "~senAsi", "~senAmme", "~senAnne", "~senAAn|~senansa";
    ),
    // Taso: 487 words (tempo, indigo, sombrero, ajo, dodo, menu).
    declension!("taso", 487, "~";
    stem noun_nom_sg: "~- like taso";
    stem pl: "~- like taso";
    stem noun_gen_pl: "~- like taso";
    ill: "~on", _;
    ess: "~nA", _;
    possessive_nom_sg: "~ni", "~si", "~mme", "~nne", "~nsA";
    possessive_ill_sg: "~oni", "~osi", "~omme", "~onne", "~onsA";
    possessive_ess_sg: "~nAni", "~nAsi", "~nAmme", "~nAnne", "~nAAn|~nansa";
    ),
    // Tunnus: 401 words (genus, ape, atlas, opus, tytär, bonus).
    declension!("tunnus", 401, "~~";
    stem noun_nom_sg: "~~- like tunnus";
    stem pl: "~k~- like purkaminen";
    gen_: _, "~~ten|~k~ien";
    par: _, "~k~iA";
    ill: "~k~een", "~k~iin";
    ess: "~k~enA", "~k~inA";
    possessive_nom_sg: "~k~eni", "~k~esi", "~k~emme", "~k~enne", "~k~ensA";
    possessive_gen_pl: "~~teni|~k~ieni", "~~tesi|~k~iesi", "~~temme|~k~iemme", "~~tenne|~k~ienne", "~~tensA|~k~iensa";
    possessive_par_pl: "~k~iAni", "~k~iAsi", "~k~iAmme", "~k~iAnne", "~k~iAAn|~k~iansa";
    possessive_ill_sg: "~k~eeni", "~k~eesi", "~k~eemme", "~k~eenne", "~k~eensA";
    possessive_ill_pl: "~k~iini", "~k~iisi", "~k~iimme", "~k~iinne", "~k~iinsA";
    possessive_ess_sg: "~k~enAni", "~k~enAsi", "~k~enAmme", "~k~enAnne", "~k~enAAn|~k~enansa";
    possessive_ess_pl: "~k~inAni", "~k~inAsi", "~k~inAmme", "~k~inAnne", "~k~inAAn|~k~inansa";
    possessive_com_pl: "~k~ineni", "~k~inesi", "~k~inemme", "~k~inenne", "~k~ineen|~k~inensa";
    ),
    // Pari: 334 words (khaki, kali, wasabi, kaali, tali, uuni).
    declension!("pari", 334, "~i";
    stem noun_nom_sg: "~i- like taso";
    stem pl: "~- like pari";
    stem noun_gen_pl: "~- like pari";
    ill: "~iin", _;
    ess: "~inA", _;
    possessive_nom_sg: "~ini", "~isi", "~imme", "~inne", "~insA";
    possessive_ill_sg: "~iini", "~iisi", "~iimme", "~iinne", "~iinsA";
    possessive_ess_sg: "~inAni", "~inAsi", "~inAmme", "~inAnne", "~inAAn|~inansa";
    ),
    // Osa: 318 words (koira, pula, musta, korva, kuula, kura).
    declension!("osa", 318, "~a";
    stem noun_nom_sg: "~a- like osa";
    stem pl: "~- like osa";
    stem noun_gen_pl: "~i- like osa";
    ill: "~aAn", _;
    ess: "~anA", _;
    possessive_nom_sg: "~ani", "~asi", "~amme", "~anne", "~ansA";
    possessive_ill_sg: "~aAni", "~aAsi", "~aAmme", "~aAnne", "~aAnsA";
    possessive_ess_sg: "~anAni", "~anAsi", "~anAmme", "~anAnne", "~anAAn|~anansa";
    ),
    // Sarja: 311 words (lama, salsa, matka, kara, sana, gamma). Like Osa, except in 0 cells: .
    declension!("sarja", 311, "~a", like Osa;
    stem pl: "~- like sarja";
    stem noun_gen_pl: "~o- like taso";
    ),
    // Ankkuri: 255 words (ämpäri, banaani, mittari, versaali, paperi, moottori). Like Pari, except in 0 cells: .
    declension!("ankkuri", 255, "~i", like Pari;
    stem noun_gen_pl: "~- like ankkuri";
    ),
    // Ykkönen: 207 words (kääntäminen, etiäinen, mehiläinen, ihminen, herhiläinen, pistiäinen). Like Purkaminen, except in 5 cells: possessive_par_sg _3, possessive_par_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("ykkönen", 207, "~nen", like Purkaminen;
    stem pl: "~s- like ykkönen";
    possessive_par_sg: _, _, _, _, "~stAAn|~stänsä";
    possessive_par_pl: _, _, _, _, "~siAAn|~siänsä";
    possessive_ess_sg: _, _, _, _, "~senAAn|~senänsä";
    possessive_ess_pl: _, _, _, _, "~sinAAn|~sinänsä";
    possessive_com_pl: _, _, _, _, "~sineen|~sinensä";
    ),
    // Erä: 169 words (päivä, pitkä, isä, kynä, vihreä, kesä).
    declension!("erä", 169, "~ä";
    stem noun_nom_sg: "~ä- like osa";
    stem pl: "~- like erä";
    stem noun_gen_pl: "~i- like erä";
    ill: "~äAn", _;
    ess: "~änA", _;
    possessive_nom_sg: "~äni", "~äsi", "~ämme", "~änne", "~änsA";
    possessive_ill_sg: "~äAni", "~äAsi", "~äAmme", "~äAnne", "~äAnsA";
    possessive_ess_sg: "~änAni", "~änAsi", "~änAmme", "~änAnne", "~änAAn|~änänsä";
    ),
    // Nappula: 156 words (tortilla, harmonia, porkkana, ikkuna, gemena, vasara). Like Sarja, except in 0 cells: .
    declension!("nappula", 156, "~a", like Sarja;
    stem noun_gen_pl: "~oi- like nappula";
    ),
    // Verkko: 144 words (postiennakko, kurkku, kinkku, laukku, polku, haiku).
    declension!("verkko", 144, "~k~";
    stem noun_nom_sg: "~k~- like taso";
    stem pl: "~~- like taso";
    stem noun_gen_pl: "~k~- like taso";
    ill: "~k~on", _;
    ess: "~k~nA", _;
    possessive_nom_sg: "~k~ni", "~k~si", "~k~mme", "~k~nne", "~k~nsA";
    possessive_ill_sg: "~k~oni", "~k~osi", "~k~omme", "~k~onne", "~k~onsA";
    possessive_ess_sg: "~k~nAni", "~k~nAsi", "~k~nAmme", "~k~nAnne", "~k~nAAn|~k~nansa";
    ),
    // Oikeus: 135 words (nopeus, tilavuus, pituus, hiljaisuus, vinous, avaruus).
    declension!("oikeus", 135, "~s";
    stem pl: "~- like oikeus";
    stem noun_gen_pl: "~ksi- like osa";
    nom: "~s", _;
    par: "~ttA", _;
    ill: "~teen", _;
    ess: "~tenA", _;
    possessive_nom_sg: "~teni", "~tesi", "~temme", "~tenne", "~tensA";
    possessive_par_sg: "~ttAni", "~ttAsi", "~ttAmme", "~ttAnne", "~ttAAn|~ttansa";
    possessive_ill_sg: "~teeni", "~teesi", "~teemme", "~teenne", "~teensA";
    possessive_ess_sg: "~tenAni", "~tenAsi", "~tenAmme", "~tenAnne", "~tenAAn|~tenansa";
    ),
    // Keri: 122 words (keksi, peili, väri, seksi, peli, viini).
    declension!("keri", 122, "~i";
    stem noun_nom_sg: "~i- like keri";
    stem pl: "~- like keri";
    stem noun_gen_pl: "~- like keri";
    ill: "~iin", _;
    ess: "~inä", _;
    possessive_nom_sg: "~ini", "~isi", "~imme", "~inne", "~insä";
    possessive_ill_sg: "~iini", "~iisi", "~iimme", "~iinne", "~iinsä";
    possessive_ess_sg: "~inäni", "~inäsi", "~inämme", "~inänne", "~inäAn|~inänsä";
    ),
    // Tykitys: 115 words (kehys, kysymys, jänis, kives, käännös, pyhimys). Like Tunnus, except in 5 cells: possessive_gen_pl _3, possessive_par_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("tykitys", 115, "~~", like Tunnus;
    stem noun_nom_sg: "~~- like tykitys";
    stem pl: "~k~- like ykkönen";
    possessive_gen_pl: _, _, _, _, "~~tensA|~k~iensä";
    possessive_par_pl: _, _, _, _, "~k~iAAn|~k~iänsä";
    possessive_ess_sg: _, _, _, _, "~k~enAAn|~k~enänsä";
    possessive_ess_pl: _, _, _, _, "~k~inAAn|~k~inänsä";
    possessive_com_pl: _, _, _, _, "~k~ineen|~k~inensä";
    ),
    // Aste: 113 words (kale, tietokone, purse, sane, huone, aave). Like Taso, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("aste", 113, "~", like Taso as PAR_AS_ABE;
    stem pl: "~- like purkaminen";
    stem noun_gen_pl: "~i- like aste";
    ill: "~eseen", _;
    ess: "~enA", _;
    possessive_nom_sg: "~eni", "~esi", "~emme", "~enne", "~ensA";
    possessive_ill_sg: "~eseeni", "~eseesi", "~eseemme", "~eseenne", "~eseensA";
    possessive_ess_sg: "~enAni", "~enAsi", "~enAmme", "~enAnne", "~enAAn|~enansa";
    ),
    // Konversio: 105 words (luu, radio, valtio, tau, video, olio). Like Taso, except in 0 cells: .
    declension!("konversio", 105, "~", like Taso;
    stem noun_nom_sg: "~- like tunnus";
    stem noun_gen_pl: "~i- like nappula";
    ),
    // Kortti: 104 words (voltti, tutti, välilyönti, prosentti, gootti, timantti). Like Pari, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("kortti", 104, "~ti", like Pari;
    stem noun_nom_sg: "~ti- like taso";
    stem noun_gen_pl: "~t- like pari";
    ill: "~tiin", _;
    ess: "~tinA", _;
    possessive_nom_sg: "~tini", "~tisi", "~timme", "~tinne", "~tinsA";
    possessive_ill_sg: "~tiini", "~tiisi", "~tiimme", "~tiinne", "~tiinsA";
    possessive_ess_sg: "~tinAni", "~tinAsi", "~tinAmme", "~tinAnne", "~tinAAn|~tinansa";
    ),
    // Levy: 99 words (baby, öljy, whisky, käly, levy, jury). Like Taso, except in 7 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3, ….
    declension!("levy", 99, "~", like Taso;
    stem noun_nom_sg: "~- like levy";
    stem pl: "~- like levy";
    stem noun_gen_pl: "~- like levy";
    ill: "~yn", _;
    possessive_ill_sg: "~yni", "~ysi", "~ymme", "~ynne", "~ynsA";
    possessive_ess_sg: _, _, _, _, "~nAAn|~nänsä";
    ),
    // Paikka: 71 words (markka, jalka, parka, siika, sika, haukka). Like Sarja, except in 25 cells: nom sg, par sg, ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, ….
    declension!("paikka", 71, "~ka", like Sarja;
    stem noun_gen_pl: "~ko- like taso";
    nom: "~ka", _;
    par: "~kaA", _;
    ill: "~kaAn", _;
    ess: "~kanA", _;
    possessive_nom_sg: "~kani", "~kasi", "~kamme", "~kanne", "~kansA";
    possessive_par_sg: "~kaAni", "~kaAsi", "~kaAmme", "~kaAnne", "~kaAnsA";
    possessive_ill_sg: "~kaAni", "~kaAsi", "~kaAmme", "~kaAnne", "~kaAnsA";
    possessive_ess_sg: "~kanAni", "~kanAsi", "~kanAmme", "~kanAnne", "~kanAAn|~kanansa";
    possessive_ins_sg: _, _, _, _, "~kansa";
    ),
    // Kanta: 71 words (rauta, nauta, kartta, hauta, parta, panta).
    declension!("kanta", 71, "~ta";
    stem noun_nom_sg: "~ta- like osa";
    stem pl: "~n- like kanta";
    stem noun_gen_pl: "~to- like taso";
    ill: "~taAn", _;
    ess: "~tanA", _;
    possessive_nom_sg: "~tani", "~tasi", "~tamme", "~tanne", "~tansA";
    possessive_ill_sg: "~taAni", "~taAsi", "~taAmme", "~taAnne", "~taAnsA";
    possessive_ess_sg: "~tanAni", "~tanAsi", "~tanAmme", "~tanAnne", "~tanAAn|~tanansa";
    ),
    // Veto: 60 words (pato, itu, satu, veto, mato, pito).
    declension!("veto", 60, "~t~";
    stem noun_nom_sg: "~t~- like taso";
    stem pl: "~d~- like taso";
    stem noun_gen_pl: "~t~- like taso";
    ill: "~t~on", _;
    ess: "~t~nA", _;
    possessive_nom_sg: "~t~ni", "~t~si", "~t~mme", "~t~nne", "~t~nsA";
    possessive_ill_sg: "~t~oni", "~t~osi", "~t~omme", "~t~onne", "~t~onsA";
    possessive_ess_sg: "~t~nAni", "~t~nAsi", "~t~nAmme", "~t~nAnne", "~t~nAAn|~t~nansa";
    ),
    // Piste: 56 words (sine, piste, herne, hie, este, neste).
    declension!("piste", 56, "~";
    stem noun_nom_sg: "~- like piste";
    stem pl: "~- like piste";
    stem noun_gen_pl: "~i- like piste";
    ill: "~eseen", _;
    ess: "~enä", _;
    possessive_nom_sg: "~eni", "~esi", "~emme", "~enne", "~ensä";
    possessive_ill_sg: "~eseeni", "~eseesi", "~eseemme", "~eseenne", "~eseensä";
    possessive_ess_sg: "~enäni", "~enäsi", "~enämme", "~enänne", "~enäAn|~enänsä";
    ),
    // Yhteys: 56 words (tiheys, leveys, syyllisyys, typeryys, näkyvyys, kansainvälisyys). Like Oikeus, except in 16 cells: possessive_par_sg _3, possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, possessive_ade_sg _3, ….
    declension!("yhteys", 56, "~s", like Oikeus;
    stem noun_gen_pl: "~ksi- like yhteys";
    possessive_par_sg: _, _, _, _, "~ttAAn|~ttänsä";
    possessive_ine_sg: _, _, _, _, "~dessAAn|~dessänsä";
    possessive_ine_pl: _, _, _, _, "~ksissAAn|~ksissänsä";
    possessive_ela_sg: _, _, _, _, "~destAAn|~destänsä";
    possessive_ela_pl: _, _, _, _, "~ksistAAn|~ksistänsä";
    possessive_ade_sg: _, _, _, _, "~dellAAn|~dellänsä";
    possessive_ade_pl: _, _, _, _, "~ksillAAn|~ksillänsä";
    possessive_abl_sg: _, _, _, _, "~deltAAn|~deltänsä";
    possessive_abl_pl: _, _, _, _, "~ksiltAAn|~ksiltänsä";
    possessive_all_sg: _, _, _, _, "~delleen|~dellensä";
    possessive_all_pl: _, _, _, _, "~ksilleen|~ksillensä";
    possessive_ess_sg: _, _, _, _, "~tenAAn|~tenänsä";
    possessive_tra_sg: _, _, _, _, "~dekseen|~deksensä";
    possessive_tra_pl: _, _, _, _, "~ksikseen|~ksiksensä";
    possessive_abe_sg: _, _, _, _, "~dettAAn|~dettänsä";
    possessive_abe_pl: _, _, _, _, "~ksittAAn|~ksittänsä";
    ),
    // Heitto: 52 words (otto, katto, kettu, hattu, vittu, patto). Like Verkko, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("heitto", 52, "~t~", like Verkko;
    stem noun_nom_sg: "~t~- like taso";
    stem noun_gen_pl: "~t~- like taso";
    ill: "~t~on", _;
    ess: "~t~nA", _;
    possessive_nom_sg: "~t~ni", "~t~si", "~t~mme", "~t~nne", "~t~nsA";
    possessive_ill_sg: "~t~oni", "~t~osi", "~t~omme", "~t~onne", "~t~onsA";
    possessive_ess_sg: "~t~nAni", "~t~nAsi", "~t~nAmme", "~t~nAnne", "~t~nAAn|~t~nansa";
    ),
    // Silmukka: 50 words (lusikka, haarukka, harakka, mustikka, puolukka, tupakka). Like Paikka, except in 19 cells: gen_ pl, par pl, ill pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, ….
    declension!("silmukka", 50, "~ka", like Paikka;
    gen_: _, "~oiden|~oitten|~kojen";
    par: _, "~oitA|~kojA";
    ill: _, "~oihin|~koihin";
    possessive_gen_pl: "~oideni|~oitteni|~kojeni", "~oidesi|~oittesi|~kojesi", "~oidemme|~oittemme|~kojemme", "~oidenne|~oittenne|~kojenne", "~oidensA|~oittensa|~kojensA";
    possessive_par_pl: "~oitAni|~kojAni", "~oitAsi|~kojAsi", "~oitAmme|~kojAmme", "~oitAnne|~kojAnne", "~oitAAn|~kojAAn|~oitansa|~kojansa";
    possessive_ill_pl: "~oihini|~koihini", "~oihisi|~koihisi", "~oihimme|~koihimme", "~oihinne|~koihinne", "~oihinsA|~koihinsa";
    possessive_ins_sg: _, _, _, _, derived;
    ),
    // Pukki: 50 words (turkki, laki, šakki, shakki, lokki, punkki). Like Pari, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("pukki", 50, "~ki", like Pari;
    stem noun_nom_sg: "~ki- like taso";
    stem noun_gen_pl: "~k- like pari";
    ill: "~kiin", _;
    ess: "~kinA", _;
    possessive_nom_sg: "~kini", "~kisi", "~kimme", "~kinne", "~kinsA";
    possessive_ill_sg: "~kiini", "~kiisi", "~kiimme", "~kiinne", "~kiinsA";
    possessive_ess_sg: "~kinAni", "~kinAsi", "~kinAmme", "~kinAnne", "~kinAAn|~kinansa";
    ),
    // Kota: 49 words (kulta, punta, multa, kota, sota, rotta).
    declension!("kota", 49, "~ta";
    stem noun_nom_sg: "~ta- like osa";
    stem pl: "~d- like kota";
    stem noun_gen_pl: "~ti- like osa";
    ill: "~taAn", _;
    ess: "~tanA", _;
    possessive_nom_sg: "~tani", "~tasi", "~tamme", "~tanne", "~tansA";
    possessive_ill_sg: "~taAni", "~taAsi", "~taAmme", "~taAnne", "~taAnsA";
    possessive_ess_sg: "~tanAni", "~tanAsi", "~tanAmme", "~tanAnne", "~tanAAn|~tanansa";
    ),
    // Avain: 46 words (puhelin, kasvain, valaisin, morsian, avain, vahvistin).
    declension!("avain", 46, "~n";
    stem noun_nom_sg: "~n- like tunnus";
    stem pl: "~m- like purkaminen";
    stem noun_gen_pl: "~- like avain";
    ill: "~meen", _;
    ess: "~menA", _;
    possessive_nom_sg: "~meni", "~mesi", "~memme", "~menne", "~mensA";
    possessive_ill_sg: "~meeni", "~meesi", "~meemme", "~meenne", "~meensA";
    possessive_ess_sg: "~menAni", "~menAsi", "~menAmme", "~menAnne", "~menAAn|~menansa";
    ),
    // Kuori: 43 words (tuuli, nuori, vuori, kieli, ääni, juuri). Like Aste, except in 14 cells: nom sg, par sg, ill sg, possessive_par_sg _1sg, possessive_par_sg _2sg, possessive_par_sg _1pl, ….
    declension!("kuori", 43, "~i", like Aste;
    stem noun_gen_pl: "~- like purkaminen";
    nom: "~i", _;
    par: "~tA", _;
    ill: "~een", _;
    possessive_par_sg: "~tAni", "~tAsi", "~tAmme", "~tAnne", "~tAAn|~tansa";
    possessive_ill_sg: "~eeni", "~eesi", "~eemme", "~eenne", "~eensA";
    possessive_ins_sg: _, _, _, _, "~ensa";
    ),
    // Ennakointi: 43 words (sekunti, ruokatunti, anti, tunti, vahti, mahti).
    declension!("ennakointi", 43, "~ti";
    stem noun_nom_sg: "~ti- like taso";
    stem pl: "~n- like pari";
    stem noun_gen_pl: "~t- like pari";
    ill: "~tiin", _;
    ess: "~tinA", _;
    possessive_nom_sg: "~tini", "~tisi", "~timme", "~tinne", "~tinsA";
    possessive_ill_sg: "~tiini", "~tiisi", "~tiimme", "~tiinne", "~tiinsA";
    possessive_ess_sg: "~tinAni", "~tinAsi", "~tinAmme", "~tinAnne", "~tinAAn|~tinansa";
    ),
    // Piikki: 42 words (tšekki, pinkki, minkki, leikki, pyökki, nyrkki). Like Keri, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("piikki", 42, "~ki", like Keri;
    stem noun_nom_sg: "~ki- like keri";
    stem noun_gen_pl: "~k- like keri";
    ill: "~kiin", _;
    ess: "~kinä", _;
    possessive_nom_sg: "~kini", "~kisi", "~kimme", "~kinne", "~kinsä";
    possessive_ill_sg: "~kiini", "~kiisi", "~kiimme", "~kiinne", "~kiinsä";
    possessive_ess_sg: "~kinäni", "~kinäsi", "~kinämme", "~kinänne", "~kinäAn|~kinänsä";
    ),
    // Luokka: 40 words (suka, toukka, kukka, sukka, ruoka, kuikka).
    declension!("luokka", 40, "~ka";
    stem noun_nom_sg: "~ka- like osa";
    stem pl: "~- like kota";
    stem noun_gen_pl: "~ki- like osa";
    ill: "~kaAn", _;
    ess: "~kanA", _;
    possessive_nom_sg: "~kani", "~kasi", "~kamme", "~kanne", "~kansA";
    possessive_ill_sg: "~kaAni", "~kaAsi", "~kaAmme", "~kaAnne", "~kaAnsA";
    possessive_ess_sg: "~kanAni", "~kanAsi", "~kanAmme", "~kanAnne", "~kanAAn|~kanansa";
    ),
    // Asento: 40 words (lento, lintu, into, tähdenlento, luonto, asunto).
    declension!("asento", 40, "~t~";
    stem noun_nom_sg: "~t~- like taso";
    stem pl: "~n~- like taso";
    stem noun_gen_pl: "~t~- like taso";
    ill: "~t~on", _;
    ess: "~t~nA", _;
    possessive_nom_sg: "~t~ni", "~t~si", "~t~mme", "~t~nne", "~t~nsA";
    possessive_ill_sg: "~t~oni", "~t~osi", "~t~omme", "~t~onne", "~t~onsA";
    possessive_ess_sg: "~t~nAni", "~t~nAsi", "~t~nAmme", "~t~nAnne", "~t~nAAn|~t~nansa";
    ),
    // Lippu: 33 words (lippu, happo, kumpu, huppu, rumpu, peippo). Like Verkko, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("lippu", 33, "~p~", like Verkko;
    stem noun_nom_sg: "~p~- like taso";
    stem noun_gen_pl: "~p~- like taso";
    ill: "~p~un", _;
    ess: "~p~nA", _;
    possessive_nom_sg: "~p~ni", "~p~si", "~p~mme", "~p~nne", "~p~nsA";
    possessive_ill_sg: "~p~uni", "~p~usi", "~p~umme", "~p~unne", "~p~unsA";
    possessive_ess_sg: "~p~nAni", "~p~nAsi", "~p~nAmme", "~p~nAnne", "~p~nAAn|~p~nansa";
    ),
    // Tappi: 31 words (kaappi, jääkaappi, kielioppi, korppi, kuppi, nappi). Like Pari, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("tappi", 31, "~pi", like Pari;
    stem noun_nom_sg: "~pi- like taso";
    stem noun_gen_pl: "~p- like pari";
    ill: "~piin", _;
    ess: "~pinA", _;
    possessive_nom_sg: "~pini", "~pisi", "~pimme", "~pinne", "~pinsA";
    possessive_ill_sg: "~piini", "~piisi", "~piimme", "~piinne", "~piinsA";
    possessive_ess_sg: "~pinAni", "~pinAsi", "~pinAmme", "~pinAnne", "~pinAAn|~pinansa";
    ),
    // Tekijä: 30 words (päärynä, ympyrä, pähkinä, tekijä, kylpylä, näsiä). Like Erä, except in 43 cells: ine pl, ela pl, ade pl, abl pl, all pl, tra pl, ….
    declension!("tekijä", 30, "~ä", like Erä;
    stem noun_gen_pl: "~öi- like tekijä";
    ine: _, "~öissA";
    ela: _, "~öistA";
    ade: _, "~öillA";
    abl: _, "~öiltA";
    all: _, "~öille";
    tra: _, "~öiksi";
    abe: _, "~öittA";
    ins: _, "~öin";
    possessive_ine_pl: "~öissAni", "~öissAsi", "~öissAmme", "~öissAnne", "~öissAAn|~öissänsä";
    possessive_ela_pl: "~öistAni", "~öistAsi", "~öistAmme", "~öistAnne", "~öistAAn|~öistänsä";
    possessive_ade_pl: "~öillAni", "~öillAsi", "~öillAmme", "~öillAnne", "~öillAAn|~öillänsä";
    possessive_abl_pl: "~öiltAni", "~öiltAsi", "~öiltAmme", "~öiltAnne", "~öiltAAn|~öiltänsä";
    possessive_all_pl: "~öilleni", "~öillesi", "~öillemme", "~öillenne", "~öilleen|~öillensä";
    possessive_tra_pl: "~öikseni", "~öiksesi", "~öiksemme", "~öiksenne", "~öikseen|~öiksensä";
    possessive_abe_pl: "~öittAni", "~öittAsi", "~öittAmme", "~öittAnne", "~öittAAn|~öittänsä";
    ),
    // Kivi: 29 words (pilvi, hirvi, kivi, liemi, tyvi, helmi). Like Piste, except in 6 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3.
    declension!("kivi", 29, "~i", like Piste;
    stem noun_nom_sg: "~- like kivi";
    stem noun_gen_pl: "~i- like erä";
    ill: "~een", _;
    possessive_ill_sg: "~eeni", "~eesi", "~eemme", "~eenne", "~eensä";
    ),
    // Lähetti: 29 words (letti, hintti, helvetti, nötkötti, sentti, setti). Like Kortti, except in 4 cells: possessive_par_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("lähetti", 29, "~ti", like Kortti;
    stem noun_nom_sg: "~ti- like levy";
    stem pl: "~- like lähetti";
    possessive_par_pl: _, _, _, _, "~tejAAn|~tejänsä";
    possessive_ess_sg: _, _, _, _, "~tinAAn|~tinänsä";
    possessive_ess_pl: _, _, _, _, "~teinAAn|~teinänsä";
    possessive_com_pl: _, _, _, _, "~teineen|~teinensä";
    ),
    // Parvi: 29 words (kuusi, takatalvi, suvi, sormi, povi, tammi). Like Kuori, except in 1 cell: possessive_ins_sg _3.
    declension!("parvi", 29, "~i", like Kuori;
    stem noun_nom_sg: "~- like parvi";
    stem noun_gen_pl: "~i- like osa";
    possessive_ins_sg: _, _, _, _, derived;
    ),
    // Tarvas: 25 words (taivas, porsas, oinas, ankerias, lounas, patsas). Like Luokka, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("tarvas", 25, "~s", like Luokka;
    stem noun_nom_sg: "~s- like tunnus";
    stem noun_gen_pl: "~i- like tarvas";
    ill: "~Aseen", _;
    ess: "~AnA", _;
    possessive_nom_sg: "~Ani", "~Asi", "~Amme", "~Anne", "~AnsA";
    possessive_ill_sg: "~Aseeni", "~Aseesi", "~Aseemme", "~Aseenne", "~AseensA";
    possessive_ess_sg: "~AnAni", "~AnAsi", "~AnAmme", "~AnAnne", "~AnAAn|~anansa";
    ),
    // Kausi: 25 words (kynsi, susi, länsi, kausi, vuorokausi, varsi). Like Oikeus, except in 44 cells: nom sg, ine pl, ela pl, ade pl, abl pl, all pl, ….
    declension!("kausi", 25, "~si", like Oikeus;
    stem noun_gen_pl: "~si- like osa";
    nom: "~si", _;
    ine: _, "~sissA";
    ela: _, "~sistA";
    ade: _, "~sillA";
    abl: _, "~siltA";
    all: _, "~sille";
    tra: _, "~siksi";
    abe: _, "~sittA";
    ins: _, "~sin";
    possessive_ine_pl: "~sissAni", "~sissAsi", "~sissAmme", "~sissAnne", "~sissAAn|~sissansa";
    possessive_ela_pl: "~sistAni", "~sistAsi", "~sistAmme", "~sistAnne", "~sistAAn|~sistansa";
    possessive_ade_pl: "~sillAni", "~sillAsi", "~sillAmme", "~sillAnne", "~sillAAn|~sillansa";
    possessive_abl_pl: "~siltAni", "~siltAsi", "~siltAmme", "~siltAnne", "~siltAAn|~siltansa";
    possessive_all_pl: "~silleni", "~sillesi", "~sillemme", "~sillenne", "~silleen|~sillensa";
    possessive_tra_pl: "~sikseni", "~siksesi", "~siksemme", "~siksenne", "~sikseen|~siksensa";
    possessive_abe_pl: "~sittAni", "~sittAsi", "~sittAmme", "~sittAnne", "~sittAAn|~sittansa";
    ),
    // Osoitin: 24 words (ääretön, nimetön, tuntematon, viivoitin, osoitin, soitin).
    declension!("osoitin", 24, "~~n";
    stem noun_nom_sg: "~~n- like tunnus";
    stem pl: "~t~m- like purkaminen";
    gen_: _, "~t~mien|~~nten";
    par: _, "~t~miA";
    ill: "~t~meen", "~t~miin";
    ess: "~t~menA", "~t~minA";
    possessive_nom_sg: "~t~meni", "~t~mesi", "~t~memme", "~t~menne", "~t~mensA";
    possessive_gen_pl: "~t~mieni|~~nteni", "~t~miesi|~~ntesi", "~t~miemme|~~ntemme", "~t~mienne|~~ntenne", "~t~miensA|~~ntensa";
    possessive_par_pl: "~t~miAni", "~t~miAsi", "~t~miAmme", "~t~miAnne", "~t~miAAn|~t~miansa";
    possessive_ill_sg: "~t~meeni", "~t~meesi", "~t~meemme", "~t~meenne", "~t~meensA";
    possessive_ill_pl: "~t~miini", "~t~miisi", "~t~miimme", "~t~miinne", "~t~miinsA";
    possessive_ess_sg: "~t~menAni", "~t~menAsi", "~t~menAmme", "~t~menAnne", "~t~menAAn|~t~menansa";
    possessive_ess_pl: "~t~minAni", "~t~minAsi", "~t~minAmme", "~t~minAnne", "~t~minAAn|~t~minansa";
    possessive_com_pl: "~t~mineni", "~t~minesi", "~t~minemme", "~t~minenne", "~t~mineen|~t~minensa";
    ),
    // Kate: 23 words (ote, kate, aate, vaate, uute, toimintaperiaate).
    declension!("kate", 23, "~~";
    stem noun_nom_sg: "~~- like aste";
    stem pl: "~t~- like purkaminen";
    stem noun_gen_pl: "~t~i- like aste";
    ill: "~t~eseen", _;
    ess: "~t~enA", _;
    possessive_nom_sg: "~t~eni", "~t~esi", "~t~emme", "~t~enne", "~t~ensA";
    possessive_ill_sg: "~t~eseeni", "~t~eseesi", "~t~eseemme", "~t~eseenne", "~t~eseensA";
    possessive_ess_sg: "~t~enAni", "~t~enAsi", "~t~enAmme", "~t~enAnne", "~t~enAAn|~t~enansa";
    ),
    // Rauta: 21 words (pata, rata, kita, virta, linnunrata, mitta).
    declension!("rauta", 21, "~ta";
    stem noun_nom_sg: "~ta- like osa";
    stem pl: "~d- like sarja";
    stem noun_gen_pl: "~to- like taso";
    ill: "~taAn", _;
    ess: "~tanA", _;
    possessive_nom_sg: "~tani", "~tasi", "~tamme", "~tanne", "~tansA";
    possessive_ill_sg: "~taAni", "~taAsi", "~taAmme", "~taAnne", "~taAnsA";
    possessive_ess_sg: "~tanAni", "~tanAsi", "~tanAmme", "~tanAnne", "~tanAAn|~tanansa";
    ),
    // Hmong: 20 words (swap, avatar, output, wok, trap, atrium). Like Pari, except in 2 cells: nom sg, com pl.
    declension!("hmong", 20, "~", like Pari;
    nom: "~", _;
    com: _, "~peineen";
    ),
    // Suhde: 19 words (sade, luode, kohde, suhde, lude, laude). Like Kate, except in 0 cells: .
    declension!("suhde", 19, "~d~", like Kate;
    stem noun_nom_sg: "~d~- like aste";
    ),
    // Hätä: 19 words (setä, itä, kenttä, näätä, lentokenttä, hätä).
    declension!("hätä", 19, "~tä";
    stem noun_nom_sg: "~tä- like osa";
    stem pl: "~d- like erä";
    stem noun_gen_pl: "~ti- like erä";
    ill: "~täAn", _;
    ess: "~tänA", _;
    possessive_nom_sg: "~täni", "~täsi", "~tämme", "~tänne", "~tänsA";
    possessive_ill_sg: "~täAni", "~täAsi", "~täAmme", "~täAnne", "~täAnsA";
    possessive_ess_sg: "~tänAni", "~tänAsi", "~tänAmme", "~tänAnne", "~tänAAn|~tänänsä";
    ),
    // Peite: 18 words (jännite, hete, heikkojännite, viite, liite, väite).
    declension!("peite", 18, "~~";
    stem noun_nom_sg: "~~- like piste";
    stem pl: "~t~- like piste";
    stem noun_gen_pl: "~t~i- like piste";
    ill: "~t~eseen", _;
    ess: "~t~enä", _;
    possessive_nom_sg: "~t~eni", "~t~esi", "~t~emme", "~t~enne", "~t~ensä";
    possessive_ill_sg: "~t~eseeni", "~t~eseesi", "~t~eseemme", "~t~eseenne", "~t~eseensä";
    possessive_ess_sg: "~t~enäni", "~t~enäsi", "~t~enämme", "~t~enänne", "~t~enäAn|~t~enänsä";
    ),
    // Kytkin: 18 words (sydän, elin, eläin, kavioeläin, pistin, koiraeläin). Like Avain, except in 4 cells: possessive_par_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("kytkin", 18, "~n", like Avain;
    stem noun_nom_sg: "~n- like tykitys";
    stem pl: "~m- like ykkönen";
    possessive_par_pl: _, _, _, _, "~miAAn|~miänsä";
    possessive_ess_sg: _, _, _, _, "~menAAn|~menänsä";
    possessive_ess_pl: _, _, _, _, "~minAAn|~minänsä";
    possessive_com_pl: _, _, _, _, "~mineen|~minensä";
    ),
    // Yksikkö: 18 words (näkö, myrkky, näky, korvayökkö, hylky, yksikkö). Like Verkko, except in 27 cells: gen_ pl, par pl, ill sg, ill pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, ….
    declension!("yksikkö", 18, "~k~", like Verkko;
    stem noun_nom_sg: "~k~- like levy";
    stem pl: "~~- like levy";
    gen_: _, "~k~jen|~~iden|~~itten";
    par: _, "~k~jA|~~itä";
    ill: "~k~ön", "~k~ihin|~~ihin";
    possessive_gen_pl: "~k~jeni|~~ideni|~~itteni", "~k~jesi|~~idesi|~~ittesi", "~k~jemme|~~idemme|~~ittemme", "~k~jenne|~~idenne|~~ittenne", "~k~jensA|~~idensä|~~ittensä";
    possessive_par_pl: "~k~jAni|~~itäni", "~k~jAsi|~~itäsi", "~k~jAmme|~~itämme", "~k~jAnne|~~itänne", "~k~jAAn|~~itään|~k~jänsä|~~itänsä";
    possessive_ill_sg: "~k~öni", "~k~ösi", "~k~ömme", "~k~önne", "~k~önsA";
    possessive_ill_pl: "~k~ihini|~~ihini", "~k~ihisi|~~ihisi", "~k~ihimme|~~ihimme", "~k~ihinne|~~ihinne", "~k~ihinsA|~~ihinsä";
    possessive_ess_sg: _, _, _, _, "~k~nAAn|~k~nänsä";
    possessive_ess_pl: _, _, _, _, "~k~inAAn|~k~inänsä";
    possessive_com_pl: _, _, _, _, "~k~ineen|~k~inensä";
    ),
    // Etumies: 17 words (mies, luottamusmies, jousimies, sotamies, pursimies, perämies).
    declension!("etumies", 17, "~s";
    stem noun_nom_sg: "~s- like etumies";
    stem pl: "~h- like piste";
    stem noun_gen_pl: "~- like etumies";
    ill: "~heen", _;
    ess: "~henä", _;
    possessive_nom_sg: "~heni", "~hesi", "~hemme", "~henne", "~hensä";
    possessive_ill_sg: "~heeni", "~heesi", "~heemme", "~heenne", "~heensä";
    possessive_ess_sg: "~henäni", "~henäsi", "~henämme", "~henänne", "~henäAn|~henänsä";
    ),
    // Neliö: 17 words (sikiö, yksiö, neliö, häiriö, keskiö, kääpiö). Like Levy, except in 6 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3.
    declension!("neliö", 17, "~", like Levy;
    stem noun_nom_sg: "~- like tykitys";
    stem noun_gen_pl: "~i- like tekijä";
    ill: "~ön", _;
    possessive_ill_sg: "~öni", "~ösi", "~ömme", "~önne", "~önsA";
    ),
    // Käsi: 17 words (vesi, käsi, mesi, hiisi, hirsi, reisi).
    declension!("käsi", 17, "~si";
    stem pl: "~- like käsi";
    stem noun_gen_pl: "~si- like erä";
    nom: "~si", _;
    par: "~ttä", _;
    ill: "~teen", _;
    ess: "~tenä", _;
    possessive_nom_sg: "~teni", "~tesi", "~temme", "~tenne", "~tensä";
    possessive_par_sg: "~ttäni", "~ttäsi", "~ttämme", "~ttänne", "~ttäAn|~ttänsä";
    possessive_ill_sg: "~teeni", "~teesi", "~teemme", "~teenne", "~teensä";
    possessive_ess_sg: "~tenäni", "~tenäsi", "~tenämme", "~tenänne", "~tenäAn|~tenänsä";
    ),
    // Luku: 17 words (suku, puku, luku, alkuluku, kyky, oikoluku).
    declension!("luku", 17, "~k~";
    stem noun_nom_sg: "~k~- like taso";
    stem pl: "~v~- like taso";
    stem noun_gen_pl: "~k~- like taso";
    ill: "~k~un", _;
    ess: "~k~nA", _;
    possessive_nom_sg: "~k~ni", "~k~si", "~k~mme", "~k~nne", "~k~nsA";
    possessive_ill_sg: "~k~uni", "~k~usi", "~k~umme", "~k~unne", "~k~unsA";
    possessive_ess_sg: "~k~nAni", "~k~nAsi", "~k~nAmme", "~k~nAnne", "~k~nAAn|~k~nansa";
    ),
    // Vaippa: 16 words (kappa, kauppa, pappa, hippa, vaippa, verkkokauppa). Like Sarja, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("vaippa", 16, "~pa", like Sarja;
    stem noun_nom_sg: "~pa- like osa";
    stem noun_gen_pl: "~po- like taso";
    ill: "~paAn", _;
    ess: "~panA", _;
    possessive_nom_sg: "~pani", "~pasi", "~pamme", "~panne", "~pansA";
    possessive_ill_sg: "~paAni", "~paAsi", "~paAmme", "~paAnne", "~paAnsA";
    possessive_ess_sg: "~panAni", "~panAsi", "~panAmme", "~panAnne", "~panAAn|~panansa";
    ),
    // Lapa: 16 words (tapa, arpa, rapa, napa, kampa, kompa).
    declension!("lapa", 16, "~pa";
    stem noun_nom_sg: "~pa- like osa";
    stem pl: "~v- like kanta";
    stem noun_gen_pl: "~po- like taso";
    ill: "~paAn", _;
    ess: "~panA", _;
    possessive_nom_sg: "~pani", "~pasi", "~pamme", "~panne", "~pansA";
    possessive_ill_sg: "~paAni", "~paAsi", "~paAmme", "~paAnne", "~paAnsA";
    possessive_ess_sg: "~panAni", "~panAsi", "~panAmme", "~panAnne", "~panAAn|~panansa";
    ),
    // Keitin: 15 words (kiihdytin, jäähdytin, linnunpelätin, heitin, kehitin, keitin).
    declension!("keitin", 15, "~~n";
    stem noun_nom_sg: "~~n- like etumies";
    stem pl: "~t~m- like piste";
    gen_: _, "~t~mien|~~nten";
    par: _, "~t~miä";
    ill: "~t~meen", "~t~miin";
    ess: "~t~menä", "~t~minä";
    possessive_nom_sg: "~t~meni", "~t~mesi", "~t~memme", "~t~menne", "~t~mensä";
    possessive_gen_pl: "~t~mieni|~~nteni", "~t~miesi|~~ntesi", "~t~miemme|~~ntemme", "~t~mienne|~~ntenne", "~t~miensä|~~ntensä";
    possessive_par_pl: "~t~miäni", "~t~miäsi", "~t~miämme", "~t~miänne", "~t~miäAn|~t~miänsä";
    possessive_ill_sg: "~t~meeni", "~t~meesi", "~t~meemme", "~t~meenne", "~t~meensä";
    possessive_ill_pl: "~t~miini", "~t~miisi", "~t~miimme", "~t~miinne", "~t~miinsä";
    possessive_ess_sg: "~t~menäni", "~t~menäsi", "~t~menämme", "~t~menänne", "~t~menäAn|~t~menänsä";
    possessive_ess_pl: "~t~minäni", "~t~minäsi", "~t~minämme", "~t~minänne", "~t~minäAn|~t~minänsä";
    possessive_com_pl: "~t~mineni", "~t~minesi", "~t~minemme", "~t~minenne", "~t~mineen|~t~minensä";
    ),
    // Ruis: 15 words (ies, ruis, asiakas, osterivinokas, ehdokas, sokerijuurikas).
    declension!("ruis", 15, "~~s";
    stem noun_nom_sg: "~~s- like tunnus";
    stem pl: "~k~- like kota";
    stem noun_gen_pl: "~k~i- like tarvas";
    ill: "~k~Aseen", _;
    ess: "~k~AnA", _;
    possessive_nom_sg: "~k~Ani", "~k~Asi", "~k~Amme", "~k~Anne", "~k~AnsA";
    possessive_ill_sg: "~k~Aseeni", "~k~Aseesi", "~k~Aseemme", "~k~Aseenne", "~k~AseensA";
    possessive_ess_sg: "~k~AnAni", "~k~AnAsi", "~k~AnAmme", "~k~AnAnne", "~k~AnAAn|~k~anansa";
    ),
    // Lehti: 14 words (tähti, aamutähti, käärmetähti, lehti, vyyhti, iltatähti). Like Hätä, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("lehti", 14, "~ti", like Hätä;
    stem noun_nom_sg: "~t- like kivi";
    stem pl: "~d- like piste";
    ill: "~teen", _;
    ess: "~tenä", _;
    possessive_nom_sg: "~teni", "~tesi", "~temme", "~tenne", "~tensä";
    possessive_ill_sg: "~teeni", "~teesi", "~teemme", "~teenne", "~teensä";
    possessive_ess_sg: "~tenäni", "~tenäsi", "~tenämme", "~tenänne", "~tenäAn|~tenänsä";
    ),
    // Leipä: 14 words (leipä, kärppä, syöpä, kyrpä, kääpä, leppä).
    declension!("leipä", 14, "~pä";
    stem noun_nom_sg: "~pä- like osa";
    stem pl: "~v- like erä";
    stem noun_gen_pl: "~pi- like erä";
    ill: "~päAn", _;
    ess: "~pänA", _;
    possessive_nom_sg: "~päni", "~päsi", "~pämme", "~pänne", "~pänsA";
    possessive_ill_sg: "~päAni", "~päAsi", "~päAmme", "~päAnne", "~päAnsA";
    possessive_ess_sg: "~pänAni", "~pänAsi", "~pänAmme", "~pänAnne", "~pänAAn|~pänänsä";
    ),
    // Kupu: 14 words (rapu, apu, varpu, papu, riepu, hepo). Like Luku, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("kupu", 14, "~p~", like Luku;
    stem noun_nom_sg: "~p~- like taso";
    stem noun_gen_pl: "~p~- like taso";
    ill: "~p~un", _;
    ess: "~p~nA", _;
    possessive_nom_sg: "~p~ni", "~p~si", "~p~mme", "~p~nne", "~p~nsA";
    possessive_ill_sg: "~p~uni", "~p~usi", "~p~umme", "~p~unne", "~p~unsA";
    possessive_ess_sg: "~p~nAni", "~p~nAsi", "~p~nAmme", "~p~nAnne", "~p~nAAn|~p~nansa";
    ),
    // Aika: 13 words (aika, vuodenaika, kesäaika, talviaika, keskiaika, kiertoaika).
    declension!("aika", 13, "~ika";
    stem noun_nom_sg: "~ika- like aika";
    stem pl: "~j- like aika";
    stem noun_gen_pl: "~iko- like aika";
    ill: "~ikaAn", _;
    ess: "~ikana", _;
    possessive_nom_sg: "~ikani", "~ikasi", "~ikamme", "~ikanne", "~ikansa";
    possessive_ill_sg: "~ikaAni", "~ikaAsi", "~ikaAmme", "~ikaAnne", "~ikaAnsa";
    possessive_ess_sg: "~ikanani", "~ikanasi", "~ikanamme", "~ikananne", "~ikanaan|~ikanansa";
    ),
    // Tolppa: 13 words (koppa, kuoppa, noppa, hoppa, lehtokurppa, tolppa). Like Luokka, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("tolppa", 13, "~pa", like Luokka;
    stem noun_nom_sg: "~pa- like osa";
    stem noun_gen_pl: "~pi- like osa";
    ill: "~paAn", _;
    ess: "~panA", _;
    possessive_nom_sg: "~pani", "~pasi", "~pamme", "~panne", "~pansA";
    possessive_ill_sg: "~paAni", "~paAsi", "~paAmme", "~paAnne", "~paAnsA";
    possessive_ess_sg: "~panAni", "~panAsi", "~panAmme", "~panAnne", "~panAAn|~panansa";
    ),
    // Kymppi: 13 words (sirppi, sieppi, keppi, chippi, näppi, kymppi). Like Tappi, except in 4 cells: possessive_par_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("kymppi", 13, "~pi", like Tappi;
    stem noun_nom_sg: "~pi- like levy";
    stem pl: "~- like lähetti";
    possessive_par_pl: _, _, _, _, "~pejAAn|~pejänsä";
    possessive_ess_sg: _, _, _, _, "~pinAAn|~pinänsä";
    possessive_ess_pl: _, _, _, _, "~peinAAn|~peinänsä";
    possessive_com_pl: _, _, _, _, "~peineen|~peinensä";
    ),
    // Kaanon: 13 words (diesel, magnum, tandem, denier, afar, hangul). Like Ankkuri, except in 1 cell: nom sg.
    declension!("kaanon", 13, "~", like Ankkuri;
    nom: "~", _;
    ),
    // Olot: 13 words (aivot, olut, airut, pikkuaivot, palvelut, farkut). Like Taso, except in 68 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("olot", 13, "~t", like Taso as NOM_AS_COM;
    nom: "~t", "~t";
    gen_: derived, _;
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    all: derived, _;
    ess: derived, _;
    tra: derived, _;
    abe: derived, _;
    possessive_nom_pl: "~ni", "~si", "~mme", "~nne", "~nsa";
    possessive_par_sg: derived, derived, derived, derived, derived;
    possessive_ine_sg: derived, derived, derived, derived, derived;
    possessive_ela_sg: derived, derived, derived, derived, derived;
    possessive_ill_sg: derived, derived, derived, derived, derived;
    possessive_ade_sg: derived, derived, derived, derived, derived;
    possessive_abl_sg: derived, derived, derived, derived, derived;
    possessive_all_sg: derived, derived, derived, derived, derived;
    possessive_ess_sg: derived, derived, derived, derived, derived;
    possessive_tra_sg: derived, derived, derived, derived, derived;
    possessive_abe_sg: derived, derived, derived, derived, derived;
    ),
    // Lähde: 12 words (side, lähde, toimenpide, räjähde, säde, päivänsäde). Like Peite, except in 0 cells: .
    declension!("lähde", 12, "~d~", like Peite;
    stem noun_nom_sg: "~d~- like piste";
    ),
    // Syöttö: 12 words (tyttö, näyttö, syöttö, hyväksikäyttö, kyyttö, pytty). Like Heitto, except in 7 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3, ….
    declension!("syöttö", 12, "~t~", like Heitto;
    stem noun_nom_sg: "~t~- like levy";
    stem pl: "~~- like levy";
    stem noun_gen_pl: "~t~- like levy";
    ill: "~t~ön", _;
    possessive_ill_sg: "~t~öni", "~t~ösi", "~t~ömme", "~t~önne", "~t~önsA";
    possessive_ess_sg: _, _, _, _, "~t~nAAn|~t~nänsä";
    ),
    // Selkä: 12 words (ikä, nälkä, persereikä, reikä, härkä, telkkä). Like Luokka, except in 31 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("selkä", 12, "~kä", like Luokka;
    stem noun_nom_sg: "~kä- like osa";
    stem noun_gen_pl: "~ki- like yhteys";
    ill: "~käAn", _;
    ess: "~känA", _;
    possessive_nom_sg: "~käni", "~käsi", "~kämme", "~känne", "~känsA";
    possessive_ine_sg: _, _, _, _, "~AssAAn|~ässänsä";
    possessive_ine_pl: _, _, _, _, "~issAAn";
    possessive_ela_sg: _, _, _, _, "~AstAAn|~ästänsä";
    possessive_ela_pl: _, _, _, _, "~istAAn";
    possessive_ill_sg: "~käAni", "~käAsi", "~käAmme", "~käAnne", "~käAnsA";
    possessive_ade_sg: _, _, _, _, "~AllAAn|~ällänsä";
    possessive_ade_pl: _, _, _, _, "~illAAn";
    possessive_abl_sg: _, _, _, _, "~AltAAn|~ältänsä";
    possessive_abl_pl: _, _, _, _, "~iltAAn";
    possessive_all_sg: _, _, _, _, "~Alleen|~ällensä";
    possessive_all_pl: _, _, _, _, "~illeen";
    possessive_ess_sg: "~känAni", "~känAsi", "~känAmme", "~känAnne", "~känAAn|~känänsä";
    possessive_tra_sg: _, _, _, _, "~Akseen|~äksensä";
    possessive_tra_pl: _, _, _, _, "~ikseen";
    possessive_abe_sg: _, _, _, _, "~AttAAn|~ättänsä";
    possessive_abe_pl: _, _, _, _, "~ittAAn";
    ),
    // Siemen: 12 words (siemen, ahven, nivel, paimen, jäsen, höyhen). Like Kuori, except in 11 cells: gen_ pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_gen_pl _3, ….
    declension!("siemen", 12, "~", like Kuori;
    stem noun_nom_sg: "~- like tykitys";
    stem pl: "~- like ykkönen";
    gen_: _, "~ien|~ten";
    possessive_gen_pl: "~ieni|~teni", "~iesi|~tesi", "~iemme|~temme", "~ienne|~tenne", "~iensA|~tensA";
    possessive_par_pl: _, _, _, _, "~iAAn|~iänsä";
    possessive_ess_sg: _, _, _, _, "~enAAn|~enänsä";
    possessive_ess_pl: _, _, _, _, "~inAAn|~inänsä";
    possessive_ins_sg: _, _, _, _, derived;
    possessive_com_pl: _, _, _, _, "~ineen|~inensä";
    ),
    // Pää: 11 words (jää, pää, kyynärpää, luupää, akilleenkantapää, peräpää). Like Erä, except in 36 cells: ine pl, ela pl, ill sg, ade pl, abl pl, abe pl, ….
    declension!("pää", 11, "~ä", like Erä;
    stem noun_nom_sg: "~ä- like tykitys";
    stem noun_gen_pl: "~i- like tekijä";
    ine: _, "~issA";
    ela: _, "~istA";
    ill: "~ähAn", _;
    ade: _, "~illA";
    abl: _, "~iltA";
    abe: _, "~ittA";
    possessive_ine_pl: "~issAni", "~issAsi", "~issAmme", "~issAnne", "~issAAn|~issänsä";
    possessive_ela_pl: "~istAni", "~istAsi", "~istAmme", "~istAnne", "~istAAn|~istänsä";
    possessive_ill_sg: "~ähAni", "~ähAsi", "~ähAmme", "~ähAnne", "~ähAnsA";
    possessive_ade_pl: "~illAni", "~illAsi", "~illAmme", "~illAnne", "~illAAn|~illänsä";
    possessive_abl_pl: "~iltAni", "~iltAsi", "~iltAmme", "~iltAnne", "~iltAAn|~iltänsä";
    possessive_abe_pl: "~ittAni", "~ittAsi", "~ittAmme", "~ittAnne", "~ittAAn|~ittänsä";
    ),
    // Siipi: 11 words (sampi, siipi, sappi, kilpi, läpi, vanhempi).
    declension!("siipi", 11, "~pi";
    stem noun_nom_sg: "~p- like siipi";
    stem pl: "~v- like ykkönen";
    stem noun_gen_pl: "~pi- like yhteys";
    ill: "~peen", _;
    ess: "~penA", _;
    possessive_nom_sg: "~peni", "~pesi", "~pemme", "~penne", "~pensA";
    possessive_ill_sg: "~peeni", "~peesi", "~peemme", "~peenne", "~peensA";
    possessive_ess_sg: "~penAni", "~penAsi", "~penAmme", "~penAnne", "~penAAn|~penänsä";
    ),
    // Runko: 11 words (aurinko, sänky, luuranko, runko, tanko, linko).
    declension!("runko", 11, "~k~";
    stem noun_nom_sg: "~k~- like taso";
    stem pl: "~g~- like taso";
    stem noun_gen_pl: "~k~- like taso";
    ill: "~k~on", _;
    ess: "~k~nA", _;
    possessive_nom_sg: "~k~ni", "~k~si", "~k~mme", "~k~nne", "~k~nsA";
    possessive_ill_sg: "~k~oni", "~k~osi", "~k~omme", "~k~onne", "~k~onsA";
    possessive_ess_sg: "~k~nAni", "~k~nAsi", "~k~nAmme", "~k~nAnne", "~k~nAAn|~k~nansa";
    ),
    // Hattara: 10 words (granaattiomena, maruna, ohrana, harava, korona, hattara). Like Osa, except in 43 cells: ine pl, ela pl, ade pl, abl pl, all pl, tra pl, ….
    declension!("hattara", 10, "~a", like Osa;
    stem noun_gen_pl: "~- like hattara";
    ine: _, "~oissA|~issa";
    ela: _, "~oistA|~ista";
    ade: _, "~oillA|~illa";
    abl: _, "~oiltA|~ilta";
    all: _, "~oille|~ille";
    tra: _, "~oiksi|~iksi";
    abe: _, "~oittA|~itta";
    ins: _, "~oin|~in";
    possessive_ine_pl: "~oissAni|~issani", "~oissAsi|~issasi", "~oissAmme|~issamme", "~oissAnne|~issanne", "~oissAAn|~issaan|~oissansa|~issansa";
    possessive_ela_pl: "~oistAni|~istani", "~oistAsi|~istasi", "~oistAmme|~istamme", "~oistAnne|~istanne", "~oistAAn|~istaan|~oistansa|~istansa";
    possessive_ade_pl: "~oillAni|~illani", "~oillAsi|~illasi", "~oillAmme|~illamme", "~oillAnne|~illanne", "~oillAAn|~illaan|~oillansa|~illansa";
    possessive_abl_pl: "~oiltAni|~iltani", "~oiltAsi|~iltasi", "~oiltAmme|~iltamme", "~oiltAnne|~iltanne", "~oiltAAn|~iltaan|~oiltansa|~iltansa";
    possessive_all_pl: "~oilleni|~illeni", "~oillesi|~illesi", "~oillemme|~illemme", "~oillenne|~illenne", "~oilleen|~illeen|~oillensa|~illensa";
    possessive_tra_pl: "~oikseni|~ikseni", "~oiksesi|~iksesi", "~oiksemme|~iksemme", "~oiksenne|~iksenne", "~oikseen|~ikseen|~oiksensa|~iksensa";
    possessive_abe_pl: "~oittAni|~ittani", "~oittAsi|~ittasi", "~oittAmme|~ittamme", "~oittAnne|~ittanne", "~oittAAn|~ittaan|~oittansa|~ittansa";
    ),
    // Tunne: 10 words (anne, vanne, kinner, tunne, lanne, tilanne). Like Kate, except in 0 cells: .
    declension!("tunne", 10, "~n~", like Kate;
    stem noun_nom_sg: "~n~- like aste";
    ),
    // Lyhty: 10 words (vety, löytö, lyhty, hyöty, lähtö, yty). Like Veto, except in 7 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3, ….
    declension!("lyhty", 10, "~t~", like Veto;
    stem noun_nom_sg: "~t~- like levy";
    stem pl: "~d~- like levy";
    stem noun_gen_pl: "~t~- like levy";
    ill: "~t~yn", _;
    possessive_ill_sg: "~t~yni", "~t~ysi", "~t~ymme", "~t~ynne", "~t~ynsA";
    possessive_ess_sg: _, _, _, _, "~t~nAAn|~t~nänsä";
    ),
    // Makonde: 9 words (college, sale, freelance, mousse, nalle, polle). Like Taso, except in 6 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3.
    declension!("makonde", 9, "~", like Taso;
    ill: "~en", _;
    possessive_ill_sg: "~eni", "~esi", "~emme", "~enne", "~ensA";
    ),
    // Työ: 9 words (vyö, työ, keskiyö, lapsityö, käsityö, päivätyö).
    declension!("työ", 9, "~y~";
    stem noun_nom_sg: "~y~- like työ";
    stem noun_gen_pl: "~~i- like työ";
    nom: _, "~y~t";
    gen_: "~y~n", _;
    ine: "~y~ssä", "~~issA";
    ela: "~y~stä", "~~istA";
    ill: "~y~hön", _;
    ade: "~y~llä", "~~illA";
    abl: "~y~ltä", "~~iltA";
    all: "~y~lle", "~~ille";
    ess: "~y~nä", _;
    tra: "~y~ksi", "~~iksi";
    abe: "~y~ttä", "~~ittA";
    ins: _, "~~in";
    possessive_nom_sg: "~y~ni", "~y~si", "~y~mme", "~y~nne", "~y~nsä";
    possessive_ine_sg: "~y~ssäni", "~y~ssäsi", "~y~ssämme", "~y~ssänne", "~y~ssään|~y~ssänsä";
    possessive_ine_pl: "~~issAni", "~~issAsi", "~~issAmme", "~~issAnne", "~~issAAn|~~issänsä";
    possessive_ela_sg: "~y~stäni", "~y~stäsi", "~y~stämme", "~y~stänne", "~y~stään|~y~stänsä";
    possessive_ela_pl: "~~istAni", "~~istAsi", "~~istAmme", "~~istAnne", "~~istAAn|~~istänsä";
    possessive_ill_sg: "~y~höni", "~y~hösi", "~y~hömme", "~y~hönne", "~y~hönsä";
    possessive_ade_sg: "~y~lläni", "~y~lläsi", "~y~llämme", "~y~llänne", "~y~llään|~y~llänsä";
    possessive_ade_pl: "~~illAni", "~~illAsi", "~~illAmme", "~~illAnne", "~~illAAn|~~illänsä";
    possessive_abl_sg: "~y~ltäni", "~y~ltäsi", "~y~ltämme", "~y~ltänne", "~y~ltään|~y~ltänsä";
    possessive_abl_pl: "~~iltAni", "~~iltAsi", "~~iltAmme", "~~iltAnne", "~~iltAAn|~~iltänsä";
    possessive_all_sg: "~y~lleni", "~y~llesi", "~y~llemme", "~y~llenne", "~y~lleen|~y~llensä";
    possessive_all_pl: "~~illeni", "~~illesi", "~~illemme", "~~illenne", "~~illeen|~~illensä";
    possessive_ess_sg: "~y~näni", "~y~näsi", "~y~nämme", "~y~nänne", "~y~nään|~y~nänsä";
    possessive_tra_sg: "~y~kseni", "~y~ksesi", "~y~ksemme", "~y~ksenne", "~y~kseen|~y~ksensä";
    possessive_tra_pl: "~~ikseni", "~~iksesi", "~~iksemme", "~~iksenne", "~~ikseen|~~iksensä";
    possessive_abe_sg: "~y~ttäni", "~y~ttäsi", "~y~ttämme", "~y~ttänne", "~y~ttään|~y~ttänsä";
    possessive_abe_pl: "~~ittAni", "~~ittAsi", "~~ittAmme", "~~ittAnne", "~~ittAAn|~~ittänsä";
    ),
    // Lanka: 9 words (selkäranka, lanka, ranka, sanka, johtolanka, hanka).
    declension!("lanka", 9, "~ka";
    stem noun_nom_sg: "~ka- like osa";
    stem pl: "~g- like sarja";
    stem noun_gen_pl: "~ko- like taso";
    ill: "~kaAn", _;
    ess: "~kanA", _;
    possessive_nom_sg: "~kani", "~kasi", "~kamme", "~kanne", "~kansA";
    possessive_ill_sg: "~kaAni", "~kaAsi", "~kaAmme", "~kaAnne", "~kaAnsA";
    possessive_ess_sg: "~kanAni", "~kanAsi", "~kanAmme", "~kanAnne", "~kanAAn|~kanansa";
    ),
    // Perintä: 9 words (yhdyntä, kysyntä, itsehillintä, häirintä, liitäntä, merkintä).
    declension!("perintä", 9, "~tä";
    stem noun_nom_sg: "~tä- like osa";
    stem pl: "~n- like perintä";
    stem noun_gen_pl: "~tö- like levy";
    ill: "~täAn", _;
    ess: "~tänA", _;
    possessive_nom_sg: "~täni", "~täsi", "~tämme", "~tänne", "~tänsA";
    possessive_ill_sg: "~täAni", "~täAsi", "~täAmme", "~täAnne", "~täAnsA";
    possessive_ess_sg: "~tänAni", "~tänAsi", "~tänAmme", "~tänAnne", "~tänAAn|~tänänsä";
    ),
    // Sakset: 9 words (helmet, sakset, revontulet, nimikirjaimet, näätäeläimet, viikset). Like Olot, except in 2 cells: nom sg, nom pl.
    declension!("sakset", 9, "~et", like Olot;
    stem noun_gen_pl: "~i- like osa";
    stem sg: "~e- like sakset";
    nom: "~et", "~et";
    ),
    // Kieli: 8 words (veri, herkkusieni, korvasieni, puhekieli, huhtasieni, satakieli). Like Kivi, except in 12 cells: gen_ pl, par sg, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, ….
    declension!("kieli", 8, "~i", like Kivi;
    gen_: _, "~ten|~ien";
    par: "~tä", _;
    possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensä|~iensä";
    possessive_par_sg: "~täni", "~täsi", "~tämme", "~tänne", "~täAn|~tänsä";
    ),
    // Jousi: 8 words (uni, lohi, tuli, tuohi, vuohi, huuli). Like Kuori, except in 7 cells: gen_ pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_gen_pl _3, ….
    declension!("jousi", 8, "~i", like Kuori;
    gen_: _, "~ien|~ten";
    possessive_gen_pl: "~ieni|~teni", "~iesi|~tesi", "~iemme|~temme", "~ienne|~tenne", "~iensA|~tensA";
    possessive_ins_sg: _, _, _, _, derived;
    ),
    // Hiili: 8 words (hiiri, hiili, kotihiiri, metsähiiri, vaivaishiiri, koivuhiiri). Like Kieli, except in 6 cells: gen_ pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_gen_pl _3.
    declension!("hiili", 8, "~i", like Kieli;
    gen_: _, "~ien|~ten";
    possessive_gen_pl: "~ieni|~teni", "~iesi|~tesi", "~iemme|~temme", "~ienne|~tenne", "~iensä|~tensä";
    ),
    // Työntö: 8 words (mänty, sääntö, synty, perintö, takinkääntö, käytäntö). Like Asento, except in 7 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3, ….
    declension!("työntö", 8, "~t~", like Asento;
    stem noun_nom_sg: "~t~- like levy";
    stem pl: "~n~- like levy";
    stem noun_gen_pl: "~t~- like levy";
    ill: "~t~ön", _;
    possessive_ill_sg: "~t~öni", "~t~ösi", "~t~ömme", "~t~önne", "~t~önsA";
    possessive_ess_sg: _, _, _, _, "~t~nAAn|~t~nänsä";
    ),
    // Ruoti: 7 words (peti, luoti, muoti, vati, ruoti, kuti).
    declension!("ruoti", 7, "~ti";
    stem noun_nom_sg: "~ti- like taso";
    stem pl: "~d- like pari";
    stem noun_gen_pl: "~t- like pari";
    ill: "~tiin", _;
    ess: "~tinA", _;
    possessive_nom_sg: "~tini", "~tisi", "~timme", "~tinne", "~tinsA";
    possessive_ill_sg: "~tiini", "~tiisi", "~tiimme", "~tiinne", "~tiinsA";
    possessive_ess_sg: "~tinAni", "~tinAsi", "~tinAmme", "~tinAnne", "~tinAAn|~tinansa";
    ),
    // Lupa: 7 words (lupa, tupa, upa, sopa, turpa, soopa). Like Lapa, except in 0 cells: .
    declension!("lupa", 7, "~pa", like Lapa;
    stem pl: "~v- like kota";
    stem noun_gen_pl: "~pi- like osa";
    ),
    // Toimi: 7 words (lumi, luomi, tuomi, toimi, taimi, vastatoimi). Like Avain, except in 7 cells: nom sg, par sg, possessive_par_sg _3, possessive_par_pl _1sg, possessive_par_pl _2sg, possessive_par_pl _1pl, ….
    declension!("toimi", 7, "~mi", like Avain;
    nom: "~mi", _;
    par: "~ntA|~meA", _;
    possessive_par_sg: _, _, _, _, "~ntAAn|~meAAn|~ntansa|~meansa";
    possessive_par_pl: "~meAni|~miAni", "~meAsi|~miAsi", "~meAmme|~miAmme", "~meAnne|~miAnne", _;
    ),
    // Kierto: 7 words (sorto, murto, läpimurto, siirto, kierto, marto).
    declension!("kierto", 7, "~t~";
    stem noun_nom_sg: "~t~- like taso";
    stem pl: "~r~- like kierto";
    stem noun_gen_pl: "~t~- like taso";
    ill: "~t~on", _;
    ess: "~t~nA", _;
    possessive_nom_sg: "~t~ni", "~t~si", "~t~mme", "~t~nne", "~t~nsA";
    possessive_ill_sg: "~t~oni", "~t~osi", "~t~omme", "~t~onne", "~t~onsA";
    possessive_ess_sg: "~t~nAni", "~t~nAsi", "~t~nAmme", "~t~nAnne", "~t~nAAn|~t~nansa";
    ),
    // Rengas: 7 words (kuningas, kangas, rengas, valkokangas, uimarengas, jakorengas).
    declension!("rengas", 7, "~g~s";
    stem noun_nom_sg: "~g~s- like rengas";
    stem pl: "~k~- like rengas";
    stem noun_gen_pl: "~k~i- like rengas";
    ill: "~k~aseen", _;
    ess: "~k~ana", _;
    possessive_nom_sg: "~k~ani", "~k~asi", "~k~amme", "~k~anne", "~k~ansa";
    possessive_ill_sg: "~k~aseeni", "~k~aseesi", "~k~aseemme", "~k~aseenne", "~k~aseensa";
    possessive_ess_sg: "~k~anani", "~k~anasi", "~k~anamme", "~k~ananne", "~k~anaan|~k~anansa";
    ),
    // Väki: 7 words (hauki, mäki, käki, väki, jalkaväki, ylämäki). Like Siemen, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("väki", 7, "~ki", like Siemen;
    stem noun_nom_sg: "~k- like siipi";
    stem noun_gen_pl: "~ki- like yhteys";
    ill: "~keen", _;
    ess: "~kenA", _;
    possessive_nom_sg: "~keni", "~kesi", "~kemme", "~kenne", "~kensA";
    possessive_ill_sg: "~keeni", "~keesi", "~keemme", "~keenne", "~keensA";
    possessive_ess_sg: "~kenAni", "~kenAsi", "~kenAmme", "~kenAnne", "~kenAAn|~kenänsä";
    ),
    // Koi: 7 words (thai, pii, koi, täi, tiikerihai, rapuvoi). Like Konversio, except in 13 cells: nom pl, ill sg, ins pl, possessive_nom_pl _1sg, possessive_nom_pl _2sg, possessive_nom_pl _1pl, ….
    declension!("koi", 7, "~", like Konversio as PL_AS_SG;
    stem noun_gen_pl: "~- like nappula";
    nom: _, "~t";
    ill: "~hin", _;
    ins: _, "~n";
    possessive_nom_pl: derived, derived, derived, derived, derived;
    possessive_ill_sg: "~hini", "~hisi", "~himme", "~hinne", "~hinsA";
    ),
    // Keihäs: 6 words (kirves, pylväs, keihäs, räystäs, eväs, lähettiläs). Like Tarvas, except in 5 cells: possessive_gen_pl _3, possessive_par_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("keihäs", 6, "~s", like Tarvas;
    stem noun_nom_sg: "~s- like tykitys";
    stem pl: "~- like keihäs";
    possessive_gen_pl: _, _, _, _, "~idensA|~ittensä";
    possessive_par_pl: _, _, _, _, "~itAAn|~itänsä";
    possessive_ess_sg: _, _, _, _, "~AnAAn|~änänsä";
    possessive_ess_pl: _, _, _, _, "~inAAn|~inänsä";
    possessive_com_pl: _, _, _, _, "~ineen|~inensä";
    ),
    // Kuningatar: 6 words (kuningatar, ruhtinatar, haltijatar, onnetar, taiteilijatar, kiertotatar).
    declension!("kuningatar", 6, "~~";
    stem noun_nom_sg: "~~- like tunnus";
    stem pl: "~t~- like kuningatar";
    gen_: _, "~t~ien|~~ten";
    par: _, "~t~ia";
    ill: "~t~een", "~t~iin";
    ess: "~t~ena", "~t~ina";
    possessive_nom_sg: "~t~eni", "~t~esi", "~t~emme", "~t~enne", "~t~ensa";
    possessive_gen_pl: "~t~ieni|~~teni", "~t~iesi|~~tesi", "~t~iemme|~~temme", "~t~ienne|~~tenne", "~t~iensa|~~tensa";
    possessive_par_pl: "~t~iani", "~t~iasi", "~t~iamme", "~t~ianne", "~t~iaan|~t~iansa";
    possessive_ill_sg: "~t~eeni", "~t~eesi", "~t~eemme", "~t~eenne", "~t~eensa";
    possessive_ill_pl: "~t~iini", "~t~iisi", "~t~iimme", "~t~iinne", "~t~iinsa";
    possessive_ess_sg: "~t~enani", "~t~enasi", "~t~enamme", "~t~enanne", "~t~enaan|~t~enansa";
    possessive_ess_pl: "~t~inani", "~t~inasi", "~t~inamme", "~t~inanne", "~t~inaan|~t~inansa";
    possessive_com_pl: "~t~ineni", "~t~inesi", "~t~inemme", "~t~inenne", "~t~ineen|~t~inensa";
    ),
    // Ryyppy: 6 words (ryyppy, hyppy, ryöppy, syrjähyppy, törppö, nyppy). Like Lippu, except in 7 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3, ….
    declension!("ryyppy", 6, "~p~", like Lippu;
    stem noun_nom_sg: "~p~- like levy";
    stem pl: "~~- like levy";
    stem noun_gen_pl: "~p~- like levy";
    ill: "~p~yn", _;
    possessive_ill_sg: "~p~yni", "~p~ysi", "~p~ymme", "~p~ynne", "~p~ynsA";
    possessive_ess_sg: _, _, _, _, "~p~nAAn|~p~nänsä";
    ),
    // Kanapee: 6 words (glasee, filee, sisäfilee, essee, kanapee, shawnee). Like Aste, except in 12 cells: ill sg, ill pl, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, ….
    declension!("kanapee", 6, "~e", like Aste;
    stem noun_nom_sg: "~e- like tunnus";
    ill: "~ehen|~eseen", "~ihin|~isiin";
    possessive_ill_sg: "~eheni", "~ehesi", "~ehemme", "~ehenne", "~ehensA|~eseensa";
    possessive_ill_pl: "~eseeni|~ihini|~isiini", "~eseesi|~ihisi|~isiisi", "~eseemme|~ihimme|~isiimme", "~eseenne|~ihinne|~isiinne", "~ihinsA|~isiinsa";
    ),
    // Aalto: 6 words (aalto, hyökyaalto, jätehuolto, sisältö, kiilto, huolto).
    declension!("aalto", 6, "~t~";
    stem noun_nom_sg: "~t~- like taso";
    stem pl: "~l~- like taso";
    stem noun_gen_pl: "~t~- like taso";
    ill: "~t~on", _;
    ess: "~t~nA", _;
    possessive_nom_sg: "~t~ni", "~t~si", "~t~mme", "~t~nne", "~t~nsA";
    possessive_ill_sg: "~t~oni", "~t~osi", "~t~omme", "~t~onne", "~t~onsA";
    possessive_ess_sg: "~t~nAni", "~t~nAsi", "~t~nAmme", "~t~nAnne", "~t~nAAn|~t~nansa";
    ),
    // Värit: 6 words (lasit, rillit, tanssit, värit, näpit, pihdit). Like Sakset, except in 24 cells: nom sg, par pl, ill pl, ess pl, possessive_par_pl _1sg, possessive_par_pl _2sg, ….
    declension!("värit", 6, "~it", like Sakset;
    stem pl: "~- like värit";
    stem sg: "~i- like sakset";
    nom: "~it", _;
    par: _, "~ejA";
    ill: _, "~eihin";
    ess: _, "~einA";
    possessive_par_pl: "~ejAni", "~ejAsi", "~ejAmme", "~ejAnne", "~ejAAn";
    possessive_ill_pl: "~eihini", "~eihisi", "~eihimme", "~eihinne", "~eihinsA";
    possessive_ess_pl: "~einAni", "~einAsi", "~einAmme", "~einAnne", "~einAAn";
    possessive_com_pl: "~eineni", "~einesi", "~einemme", "~einenne", "~eineen";
    ),
    // Nousu: 5 words (nandu, degu, kakadu, tipu, skrubu). Like Taso, except in 6 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3.
    declension!("nousu", 5, "~", like Taso;
    ill: "~un", _;
    possessive_ill_sg: "~uni", "~usi", "~umme", "~unne", "~unsA";
    ),
    // Koe: 5 words (koe, rae, eläinkoe, jae, hajukoe). Like Tunnus, except in 6 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3.
    declension!("koe", 5, "~~", like Tunnus;
    stem noun_nom_sg: "~~- like aste";
    stem noun_gen_pl: "~k~i- like aste";
    ill: "~k~eseen", _;
    possessive_ill_sg: "~k~eseeni", "~k~eseesi", "~k~eseemme", "~k~eseenne", "~k~eseensA";
    ),
    // Liike: 5 words (säie, pyyhe, säe, muuttoliike, korjausliike).
    declension!("liike", 5, "~~";
    stem noun_nom_sg: "~~- like piste";
    stem pl: "~k~- like piste";
    stem noun_gen_pl: "~k~i- like piste";
    ill: "~k~eseen", _;
    ess: "~k~enä", _;
    possessive_nom_sg: "~k~eni", "~k~esi", "~k~emme", "~k~enne", "~k~ensä";
    possessive_ill_sg: "~k~eseeni", "~k~eseesi", "~k~eseemme", "~k~eseenne", "~k~eseensä";
    possessive_ess_sg: "~k~enäni", "~k~enäsi", "~k~enämme", "~k~enänne", "~k~enäAn|~k~enänsä";
    ),
    // Äiti: 5 words (äiti, täti, neiti, kyyti, kruununkyyti). Like Ruoti, except in 6 cells: com pl, possessive_par_sg _3, possessive_par_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("äiti", 5, "~ti", like Ruoti;
    stem pl: "~d- like lähetti";
    com: _, "~tineen";
    possessive_par_sg: _, _, _, _, "~tiAAn|~tiänsä";
    possessive_par_pl: _, _, _, _, "~tejAAn|~tejänsä";
    possessive_ess_sg: _, _, _, _, "~tinAAn|~tinänsä";
    possessive_ess_pl: _, _, _, _, "~teinAAn|~teinänsä";
    possessive_com_pl: _, _, _, _, "~teineen|~teinensä";
    ),
    // Jänne: 5 words (pinne, jänne, rinne, liikenne, käänne). Like Tunne, except in 6 cells: possessive_gen_pl _3, possessive_par_pl _3, possessive_ill_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("jänne", 5, "~n~", like Tunne;
    stem noun_nom_sg: "~n~- like jänne";
    stem pl: "~t~- like ykkönen";
    possessive_gen_pl: _, _, _, _, "~t~idensA|~t~ittensä";
    possessive_par_pl: _, _, _, _, "~t~itAAn|~t~itänsä";
    possessive_ill_pl: _, _, _, _, "~t~isiinsA|~t~ihinsä";
    possessive_ess_sg: _, _, _, _, "~t~enAAn|~t~enänsä";
    possessive_ess_pl: _, _, _, _, "~t~inAAn|~t~inänsä";
    possessive_com_pl: _, _, _, _, "~t~ineen|~t~inensä";
    ),
    // Ewe: 5 words (ewe, freestyle, skene, lime, bisse). Like Levy, except in 6 cells: ill sg, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, possessive_ill_sg _3.
    declension!("ewe", 5, "~", like Levy;
    ill: "~en", _;
    possessive_ill_sg: "~eni", "~esi", "~emme", "~enne", "~ensA";
    ),
    // Khmer: 5 words (sandwich, blackjack, khmer, hi-tech, preesens). Like Keri, except in 1 cell: nom sg.
    declension!("khmer", 5, "~", like Keri;
    nom: "~", _;
    ),
    // Tee: 5 words (tie, tee, gee, lee, kaksoisvee). Like Piste, except in 12 cells: ill sg, ill pl, possessive_ill_sg _1sg, possessive_ill_sg _2sg, possessive_ill_sg _1pl, possessive_ill_sg _2pl, ….
    declension!("tee", 5, "~e", like Piste;
    stem noun_nom_sg: "~e- like etumies";
    ill: "~ehen", "~ihin";
    possessive_ill_sg: "~eheni", "~ehesi", "~ehemme", "~ehenne", "~ehensä";
    possessive_ill_pl: "~ihini", "~ihisi", "~ihimme", "~ihinne", "~ihinsä";
    ),
    // Taival: 5 words (rove, tarve, taive, kuve, taival).
    declension!("taival", 5, "~v~";
    stem noun_nom_sg: "~v~- like aste";
    stem pl: "~p~- like purkaminen";
    stem noun_gen_pl: "~p~i- like aste";
    ill: "~p~eseen", _;
    ess: "~p~enA", _;
    possessive_nom_sg: "~p~eni", "~p~esi", "~p~emme", "~p~enne", "~p~ensA";
    possessive_ill_sg: "~p~eseeni", "~p~eseesi", "~p~eseemme", "~p~eseenne", "~p~eseensA";
    possessive_ess_sg: "~p~enAni", "~p~enAsi", "~p~enAmme", "~p~enAnne", "~p~enAAn|~p~enansa";
    ),
    // Syy: 5 words (syy, kyy, tekosyy, peltopyy, menyy). Like Neliö, except in 61 cells: nom pl, gen_ sg, ine sg, ela sg, ill sg, ade sg, ….
    declension!("syy", 5, "~y", like Neliö;
    stem noun_nom_sg: "~y- like tykitys";
    nom: _, "~yt";
    gen_: "~yn", _;
    ine: "~yssA", _;
    ela: "~ystA", _;
    ill: "~yhyn", _;
    ade: "~yllA", _;
    abl: "~yltA", _;
    all: "~ylle", _;
    ess: "~ynA", _;
    tra: "~yksi", _;
    abe: "~yttA", _;
    possessive_nom_sg: "~yni", "~ysi", "~ymme", "~ynne", "~ynsA";
    possessive_ine_sg: "~yssAni", "~yssAsi", "~yssAmme", "~yssAnne", "~yssAAn|~yssänsä";
    possessive_ela_sg: "~ystAni", "~ystAsi", "~ystAmme", "~ystAnne", "~ystAAn|~ystänsä";
    possessive_ill_sg: "~yhyni", "~yhysi", "~yhymme", "~yhynne", "~yhynsA";
    possessive_ade_sg: "~yllAni", "~yllAsi", "~yllAmme", "~yllAnne", "~yllAAn|~yllänsä";
    possessive_abl_sg: "~yltAni", "~yltAsi", "~yltAmme", "~yltAnne", "~yltAAn|~yltänsä";
    possessive_all_sg: "~ylleni", "~yllesi", "~yllemme", "~yllenne", "~ylleen|~yllensä";
    possessive_ess_sg: "~ynAni", "~ynAsi", "~ynAmme", "~ynAnne", "~ynAAn|~ynänsä";
    possessive_tra_sg: "~ykseni", "~yksesi", "~yksemme", "~yksenne", "~ykseen|~yksensä";
    possessive_abe_sg: "~yttAni", "~yttAsi", "~yttAmme", "~yttAnne", "~yttAAn|~yttänsä";
    ),
    // Varras: 5 words (parras, marras, porras, varras, hiirenporras). Like Kate, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("varras", 5, "~r~s", like Kate;
    stem noun_nom_sg: "~r~s- like rengas";
    stem pl: "~t~- like kota";
    stem noun_gen_pl: "~t~i- like tarvas";
    ill: "~t~Aseen", _;
    ess: "~t~AnA", _;
    possessive_nom_sg: "~t~Ani", "~t~Asi", "~t~Amme", "~t~Anne", "~t~AnsA";
    possessive_ill_sg: "~t~Aseeni", "~t~Aseesi", "~t~Aseemme", "~t~Aseenne", "~t~AseensA";
    possessive_ess_sg: "~t~AnAni", "~t~AnAsi", "~t~AnAmme", "~t~AnAnne", "~t~AnAAn|~t~anansa";
    ),
    // Kärki: 5 words (jälki, järki, kylki, kärki, palokärki).
    declension!("kärki", 5, "~ki";
    stem noun_nom_sg: "~k- like siipi";
    stem pl: "~j- like ykkönen";
    stem noun_gen_pl: "~ki- like yhteys";
    ill: "~keen", _;
    ess: "~kenA", _;
    possessive_nom_sg: "~keni", "~kesi", "~kemme", "~kenne", "~kensA";
    possessive_ill_sg: "~keeni", "~keesi", "~keemme", "~keenne", "~keensA";
    possessive_ess_sg: "~kenAni", "~kenAsi", "~kenAmme", "~kenAnne", "~kenAAn|~kenänsä";
    ),
    // Askel: 5 words (sammal, askel, lehtisammal, kivisammal, harmosammal).
    declension!("askel", 5, "~";
    stem pl: "~- like askel";
    stem noun_gen_pl: "~- like askel";
    nom: "~|~e", _;
    par: "~tA|~etta", _;
    ill: "~eeseen", _;
    ess: "~enA|~eena", _;
    possessive_nom_sg: "~eni", "~esi", "~emme", "~enne", "~ensA|~eensa";
    possessive_nom_pl: "~eeni", "~eesi", "~eemme", "~eenne", _;
    possessive_par_sg: "~tAni", "~tAsi", "~tAmme", "~tAnne", "~tAAn|~ettaan|~tansa|~ettansa";
    possessive_ill_sg: "~eeseeni", "~eeseesi", "~eeseemme", "~eeseenne", "~eeseensa";
    possessive_ess_sg: "~enAni", "~enAsi", "~enAmme", "~enAnne", "~enAAn|~eenaan|~enansa|~eenansa";
    ),
    // Alkeet: 5 words (alkeet, lauteet, työvaatteet, kalusteet, puitteet). Like Sakset, except in 0 cells: .
    declension!("alkeet", 5, "~et", like Sakset;
    stem noun_gen_pl: "~i- like aste";
    ),
    // Kissanristiäiset: 5 words (päättäjäiset, metsäpäästäiset, myötäjäiset, kädelliset, kissanristiäiset). Like Sakset, except in 16 cells: gen_ pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_gen_pl _3, ….
    declension!("kissanristiäiset", 5, "~et", like Sakset;
    gen_: _, "~ten|~ien";
    possessive_gen_pl: "~teni|~ieni", "~tesi|~iesi", "~temme|~iemme", "~tenne|~ienne", "~tensA|~iensA";
    possessive_par_pl: _, _, _, _, "~iAAn|~iänsä";
    possessive_ine_pl: _, _, _, _, "~issAAn|~issänsä";
    possessive_ela_pl: _, _, _, _, "~istAAn|~istänsä";
    possessive_ade_pl: _, _, _, _, "~illAAn|~illänsä";
    possessive_abl_pl: _, _, _, _, "~iltAAn|~iltänsä";
    possessive_all_pl: _, _, _, _, "~illeen|~illensä";
    possessive_ess_pl: _, _, _, _, "~inAAn|~inänsä";
    possessive_tra_pl: _, _, _, _, "~ikseen|~iksensä";
    possessive_abe_pl: _, _, _, _, "~ittAAn|~ittänsä";
    possessive_com_pl: _, _, _, _, "~ineen|~inensä";
    ),
    // Muunnin: 5 words (äänenvaimennin, vaimennin, muunnin, käännin, tarkennin). Like Osoitin, except in 6 cells: gen_ pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_gen_pl _3.
    declension!("muunnin", 5, "~n~n", like Osoitin;
    stem noun_nom_sg: "~n~n- like tunnus";
    gen_: _, "~t~mien|~n~nten";
    possessive_gen_pl: "~t~mieni|~n~nteni", "~t~miesi|~n~ntesi", "~t~miemme|~n~ntemme", "~t~mienne|~n~ntenne", "~t~miensA|~n~ntensa";
    ),
    // Poika: 4 words (poika, talonpoika, rattopoika, sulhaspoika). Like Aika, except in 0 cells: .
    declension!("poika", 4, "~ika", like Aika;
    stem pl: "~j- like kota";
    stem noun_gen_pl: "~iki- like poika";
    ),
    // Hopea: 4 words (valkea, elohopea, hopea, kulovalkea). Like Osa, except in 20 cells: gen_ pl, par sg, par pl, ill pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, ….
    declension!("hopea", 4, "~a", like Osa;
    gen_: _, "~iden|~itten";
    par: "~aA|~ata", "~itA";
    ill: _, "~isiin|~ihin";
    possessive_gen_pl: "~ideni|~itteni", "~idesi|~ittesi", "~idemme|~ittemme", "~idenne|~ittenne", "~idensA|~ittensa";
    possessive_par_sg: _, _, _, _, "~atAAn|~aansa|~atansa";
    possessive_par_pl: "~atAni|~itAni", "~atAsi|~itAsi", "~atAmme|~itAmme", "~atAnne|~itAnne", "~itAAn|~itansa";
    possessive_ill_pl: "~isiini|~ihini", "~isiisi|~ihisi", "~isiimme|~ihimme", "~isiinne|~ihinne", "~isiinsA|~ihinsa";
    ),
    // Uurre: 4 words (aarre, imarre, uurre, paarre). Like Kate, except in 0 cells: .
    declension!("uurre", 4, "~r~", like Kate;
    stem noun_nom_sg: "~r~- like aste";
    ),
    // Ydin: 4 words (ydin, säädin, vedin, ulosvedin). Like Keitin, except in 6 cells: gen_ pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_gen_pl _3.
    declension!("ydin", 4, "~d~n", like Keitin;
    stem noun_nom_sg: "~d~n- like etumies";
    gen_: _, "~t~mien|~d~nten";
    possessive_gen_pl: "~t~mieni|~d~nteni", "~t~miesi|~d~ntesi", "~t~miemme|~d~ntemme", "~t~mienne|~d~ntenne", "~t~miensä|~d~ntensä";
    ),
    // Kauris: 4 words (kauris, saalis, ruumis, metsäkauris). Like Tarvas, except in 31 cells: nom sg, nom pl, gen_ sg, par sg, ill sg, ins sg, ….
    declension!("kauris", 4, "~s", like Tarvas as SG_AS_PL;
    nom: "~s", "~it";
    gen_: "~in", _;
    par: "~stA", _;
    ill: "~iseen", _;
    ins: derived, _;
    possessive_nom_sg: "~ini", "~isi", "~imme", "~inne", "~insA";
    possessive_gen_sg: derived, derived, derived, derived, derived;
    possessive_par_sg: "~stAni", "~stAsi", "~stAmme", "~stAnne", "~stAAn|~stansa";
    possessive_ill_sg: "~iseeni", "~iseesi", "~iseemme", "~iseenne", "~iseensA";
    possessive_com_sg: derived, derived, derived, derived, derived;
    ),
    // Kurki: 4 words (kurki, olki, arki, solki). Like Kärki, except in 1 cell: possessive_ess_sg _3.
    declension!("kurki", 4, "~ki", like Kärki;
    stem noun_nom_sg: "~k- like parvi";
    stem pl: "~j- like purkaminen";
    stem noun_gen_pl: "~ki- like osa";
    possessive_ess_sg: _, _, _, _, "~kenAAn|~kenansa";
    ),
    // Rättänä: 4 words (äpärä, käpälä, kissankäpälä, rättänä). Like Erä, except in 43 cells: ine pl, ela pl, ade pl, abl pl, all pl, tra pl, ….
    declension!("rättänä", 4, "~ä", like Erä;
    stem noun_gen_pl: "~- like rättänä";
    ine: _, "~öissA|~issä";
    ela: _, "~öistA|~istä";
    ade: _, "~öillA|~illä";
    abl: _, "~öiltA|~iltä";
    all: _, "~öille|~ille";
    tra: _, "~öiksi|~iksi";
    abe: _, "~öittA|~ittä";
    ins: _, "~öin|~in";
    possessive_ine_pl: "~öissAni|~issäni", "~öissAsi|~issäsi", "~öissAmme|~issämme", "~öissAnne|~issänne", "~öissAAn|~issään|~öissänsä|~issänsä";
    possessive_ela_pl: "~öistAni|~istäni", "~öistAsi|~istäsi", "~öistAmme|~istämme", "~öistAnne|~istänne", "~öistAAn|~istään|~öistänsä|~istänsä";
    possessive_ade_pl: "~öillAni|~illäni", "~öillAsi|~illäsi", "~öillAmme|~illämme", "~öillAnne|~illänne", "~öillAAn|~illään|~öillänsä|~illänsä";
    possessive_abl_pl: "~öiltAni|~iltäni", "~öiltAsi|~iltäsi", "~öiltAmme|~iltämme", "~öiltAnne|~iltänne", "~öiltAAn|~iltään|~öiltänsä|~iltänsä";
    possessive_all_pl: "~öilleni|~illeni", "~öillesi|~illesi", "~öillemme|~illemme", "~öillenne|~illenne", "~öilleen|~illeen|~öillensä|~illensä";
    possessive_tra_pl: "~öikseni|~ikseni", "~öiksesi|~iksesi", "~öiksemme|~iksemme", "~öiksenne|~iksenne", "~öikseen|~ikseen|~öiksensä|~iksensä";
    possessive_abe_pl: "~öittAni|~ittäni", "~öittAsi|~ittäsi", "~öittAmme|~ittämme", "~öittAnne|~ittänne", "~öittAAn|~ittään|~öittänsä|~ittänsä";
    ),
    // Takuu: 4 words (takuu, paluu, vastuu, makuu). Like Tarvas, except in 61 cells: nom pl, gen_ sg, ine sg, ela sg, ill sg, ade sg, ….
    declension!("takuu", 4, "~u", like Tarvas;
    stem noun_nom_sg: "~u- like tunnus";
    nom: _, "~ut";
    gen_: "~un", _;
    ine: "~ussA", _;
    ela: "~ustA", _;
    ill: "~useen", _;
    ade: "~ullA", _;
    abl: "~ultA", _;
    all: "~ulle", _;
    ess: "~unA", _;
    tra: "~uksi", _;
    abe: "~uttA", _;
    possessive_nom_sg: "~uni", "~usi", "~umme", "~unne", "~unsA";
    possessive_ine_sg: "~ussAni", "~ussAsi", "~ussAmme", "~ussAnne", "~ussAAn|~ussansa";
    possessive_ela_sg: "~ustAni", "~ustAsi", "~ustAmme", "~ustAnne", "~ustAAn|~ustansa";
    possessive_ill_sg: "~useeni", "~useesi", "~useemme", "~useenne", "~useensA";
    possessive_ade_sg: "~ullAni", "~ullAsi", "~ullAmme", "~ullAnne", "~ullAAn|~ullansa";
    possessive_abl_sg: "~ultAni", "~ultAsi", "~ultAmme", "~ultAnne", "~ultAAn|~ultansa";
    possessive_all_sg: "~ulleni", "~ullesi", "~ullemme", "~ullenne", "~ulleen|~ullensa";
    possessive_ess_sg: "~unAni", "~unAsi", "~unAmme", "~unAnne", "~unAAn|~unansa";
    possessive_tra_sg: "~ukseni", "~uksesi", "~uksemme", "~uksenne", "~ukseen|~uksensa";
    possessive_abe_sg: "~uttAni", "~uttAsi", "~uttAmme", "~uttAnne", "~uttAAn|~uttansa";
    ),
    // Vilunki: 4 words (mielipidevanki, medaljonki, kartonki, vilunki).
    declension!("vilunki", 4, "~ki";
    stem noun_nom_sg: "~ki- like taso";
    stem pl: "~g- like pari";
    stem noun_gen_pl: "~k- like pari";
    ill: "~kiin", _;
    ess: "~kinA", _;
    possessive_nom_sg: "~kini", "~kisi", "~kimme", "~kinne", "~kinsA";
    possessive_ill_sg: "~kiini", "~kiisi", "~kiimme", "~kiinne", "~kiinsA";
    possessive_ess_sg: "~kinAni", "~kinAsi", "~kinAmme", "~kinAnne", "~kinAAn|~kinansa";
    ),
    // Käräjät: 4 words (myyrät, nisäkkäät, valtiopäivät, käräjät). Like Kissanristiäiset, except in 2 cells: nom sg, nom pl.
    declension!("käräjät", 4, "~ät", like Kissanristiäiset;
    stem noun_gen_pl: "~i- like yhteys";
    stem sg: "~A- like sakset";
    nom: "~ät", "~ät";
    ),
    // Vehje: 4 words (hylje, vehje, sulje, rahje). Like Koe, except in 15 cells: possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, possessive_ade_sg _3, possessive_ade_pl _3, ….
    declension!("vehje", 4, "~j~", like Koe;
    stem noun_nom_sg: "~j~- like jänne";
    stem noun_gen_pl: "~k~i- like vehje";
    possessive_ine_sg: _, _, _, _, "~k~essAAn";
    possessive_ine_pl: _, _, _, _, "~k~issAAn";
    possessive_ela_sg: _, _, _, _, "~k~estAAn";
    possessive_ela_pl: _, _, _, _, "~k~istAAn";
    possessive_ade_sg: _, _, _, _, "~k~ellAAn";
    possessive_ade_pl: _, _, _, _, "~k~illAAn";
    possessive_abl_sg: _, _, _, _, "~k~eltAAn";
    possessive_abl_pl: _, _, _, _, "~k~iltAAn";
    possessive_all_sg: _, _, _, _, "~k~elleen";
    possessive_all_pl: _, _, _, _, "~k~illeen";
    possessive_ess_sg: _, _, _, _, "~k~enAAn";
    possessive_tra_sg: _, _, _, _, "~k~ekseen";
    possessive_tra_pl: _, _, _, _, "~k~ikseen";
    possessive_abe_sg: _, _, _, _, "~k~ettAAn";
    possessive_abe_pl: _, _, _, _, "~k~ittAAn";
    ),
    // Onki: 4 words (henki, onki, kanki, kiertokanki). Like Vilunki, except in 22 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("onki", 4, "~ki", like Vilunki as PL_AS_SG;
    stem noun_nom_sg: "~k- like parvi";
    stem pl: "~g- like purkaminen";
    stem noun_gen_pl: "~ki- like osa";
    ill: "~keen", _;
    ess: "~kenA", _;
    possessive_nom_sg: "~keni", "~kesi", "~kemme", "~kenne", "~kensA";
    possessive_nom_pl: derived, derived, derived, derived, derived;
    possessive_ill_sg: "~keeni", "~keesi", "~keemme", "~keenne", "~keensA";
    possessive_ess_sg: "~kenAni", "~kenAsi", "~kenAmme", "~kenAnne", "~kenAAn|~kenansa";
    ),
    // Tuki: 3 words (noki, tuki, jalkatuki). Like Väki, except in 1 cell: possessive_ess_sg _3.
    declension!("tuki", 3, "~ki", like Väki;
    stem noun_nom_sg: "~k- like parvi";
    stem pl: "~- like purkaminen";
    stem noun_gen_pl: "~ki- like osa";
    possessive_ess_sg: _, _, _, _, "~kenAAn|~kenansa";
    ),
    // Body: 3 words (body, curry, copy).
    declension!("body", 3, "~";
    stem pl: "~- like body";
    stem noun_gen_pl: "~- like body";
    nom: "~", _;
    par: "~a", _;
    ill: "~yn", _;
    ess: "~na", _;
    possessive_nom_sg: "~ni", "~si", "~mme", "~nne", "~nsa";
    possessive_par_sg: "~ani", "~asi", "~amme", "~anne", "~aAn|~ansa";
    possessive_ill_sg: "~yni", "~ysi", "~ymme", "~ynne", "~ynsa";
    possessive_ess_sg: "~nani", "~nasi", "~namme", "~nanne", "~naAn|~nansa";
    ),
    // Hönkä: 3 words (kenkä, jänkä, hönkä). Like Onki, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("hönkä", 3, "~kä", like Onki;
    stem noun_nom_sg: "~kä- like osa";
    stem pl: "~g- like keihäs";
    stem noun_gen_pl: "~ki- like yhteys";
    ill: "~käAn", _;
    ess: "~känA", _;
    possessive_nom_sg: "~käni", "~käsi", "~kämme", "~känne", "~känsA";
    possessive_ill_sg: "~käAni", "~käAsi", "~käAmme", "~käAnne", "~käAnsA";
    possessive_ess_sg: "~känAni", "~känAsi", "~känAmme", "~känAnne", "~känAAn|~känänsä";
    ),
    // Hammas: 3 words (lammas, hammas, kulmahammas). Like Taival, except in 17 cells: ill sg, ess sg, possessive_nom_sg _1sg, possessive_nom_sg _2sg, possessive_nom_sg _1pl, possessive_nom_sg _2pl, ….
    declension!("hammas", 3, "~m~s", like Taival;
    stem noun_nom_sg: "~m~s- like rengas";
    stem pl: "~p~- like kota";
    stem noun_gen_pl: "~p~i- like tarvas";
    ill: "~p~Aseen", _;
    ess: "~p~AnA", _;
    possessive_nom_sg: "~p~Ani", "~p~Asi", "~p~Amme", "~p~Anne", "~p~AnsA";
    possessive_ill_sg: "~p~Aseeni", "~p~Aseesi", "~p~Aseemme", "~p~Aseenne", "~p~AseensA";
    possessive_ess_sg: "~p~AnAni", "~p~AnAsi", "~p~AnAmme", "~p~AnAnne", "~p~AnAAn|~p~anansa";
    ),
    // Piennar: 3 words (manner, tanner, piennar).
    declension!("piennar", 3, "~n~";
    stem pl: "~t~- like piennar";
    nom: "~n~|~t~e", _;
    gen_: _, "~t~ien|~n~ten|~t~eiden|~t~eitten";
    par: "~n~ta|~t~etta", "~t~iA|~t~eita";
    ill: "~t~eeseen", "~t~iin|~t~eisiin|~t~eihin";
    ess: "~t~enA|~t~eena", "~t~inA|~t~eina";
    possessive_nom_sg: "~t~eni", "~t~esi", "~t~emme", "~t~enne", "~t~ensA|~t~eensa";
    possessive_nom_pl: "~t~eeni", "~t~eesi", "~t~eemme", "~t~eenne", _;
    possessive_gen_pl: "~t~eeni|~t~ieni|~n~teni|~t~eideni|~t~eitteni", "~t~eesi|~t~iesi|~n~tesi|~t~eidesi|~t~eittesi", "~t~eemme|~t~iemme|~n~temme|~t~eidemme|~t~eittemme", "~t~eenne|~t~ienne|~n~tenne|~t~eidenne|~t~eittenne", "~t~iensA|~n~tensa|~t~eidensa|~t~eittensa";
    possessive_par_sg: "~n~tani", "~n~tasi", "~n~tamme", "~n~tanne", "~n~taan|~t~ettaan|~n~tansa|~t~ettansa";
    possessive_par_pl: "~t~ettani|~t~iani|~t~eitani", "~t~ettasi|~t~iasi|~t~eitasi", "~t~ettamme|~t~iamme|~t~eitamme", "~t~ettanne|~t~ianne|~t~eitanne", "~t~iAAn|~t~eitaan|~t~iansa|~t~eitansa";
    possessive_ill_sg: "~t~eeseeni", "~t~eeseesi", "~t~eeseemme", "~t~eeseenne", "~t~eeseensa";
    possessive_ill_pl: "~t~iini|~t~eisiini|~t~eihini", "~t~iisi|~t~eisiisi|~t~eihisi", "~t~iimme|~t~eisiimme|~t~eihimme", "~t~iinne|~t~eisiinne|~t~eihinne", "~t~iinsa|~t~eisiinsa|~t~eihinsa";
    possessive_ess_sg: "~t~enAni", "~t~enAsi", "~t~enAmme", "~t~enAnne", "~t~enAAn|~t~eenaan|~t~enansa|~t~eenansa";
    possessive_ess_pl: "~t~eenani|~t~inani|~t~einani", "~t~eenasi|~t~inasi|~t~einasi", "~t~eenamme|~t~inamme|~t~einamme", "~t~eenanne|~t~inanne|~t~einanne", "~t~inAAn|~t~einaan|~t~inansa|~t~einansa";
    possessive_com_pl: "~t~ineni|~t~eineni", "~t~inesi|~t~einesi", "~t~inemme|~t~einemme", "~t~inenne|~t~einenne", "~t~ineen|~t~eineen|~t~inensa|~t~einensa";
    ),
    // Komme: 3 words (komme, lumme, ommel). Like Taival, except in 15 cells: possessive_ine_sg _3, possessive_ine_pl _3, possessive_ela_sg _3, possessive_ela_pl _3, possessive_ade_sg _3, possessive_ade_pl _3, ….
    declension!("komme", 3, "~m~", like Taival;
    stem noun_nom_sg: "~m~- like komme";
    stem noun_gen_pl: "~p~i- like vehje";
    possessive_ine_sg: _, _, _, _, "~p~essAAn";
    possessive_ine_pl: _, _, _, _, "~p~issAAn";
    possessive_ela_sg: _, _, _, _, "~p~estAAn";
    possessive_ela_pl: _, _, _, _, "~p~istAAn";
    possessive_ade_sg: _, _, _, _, "~p~ellAAn";
    possessive_ade_pl: _, _, _, _, "~p~illAAn";
    possessive_abl_sg: _, _, _, _, "~p~eltAAn";
    possessive_abl_pl: _, _, _, _, "~p~iltAAn";
    possessive_all_sg: _, _, _, _, "~p~elleen";
    possessive_all_pl: _, _, _, _, "~p~illeen";
    possessive_ess_sg: _, _, _, _, "~p~enAAn";
    possessive_tra_sg: _, _, _, _, "~p~ekseen";
    possessive_tra_pl: _, _, _, _, "~p~ikseen";
    possessive_abe_sg: _, _, _, _, "~p~ettAAn";
    possessive_abe_pl: _, _, _, _, "~p~ittAAn";
    ),
    // Leve: 3 words (leve, helve, lieve). Like Taival, except in 6 cells: possessive_gen_pl _3, possessive_par_pl _3, possessive_ill_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("leve", 3, "~v~", like Taival;
    stem noun_nom_sg: "~v~- like jänne";
    stem pl: "~p~- like ykkönen";
    possessive_gen_pl: _, _, _, _, "~p~idensA|~p~ittensä";
    possessive_par_pl: _, _, _, _, "~p~itAAn|~p~itänsä";
    possessive_ill_pl: _, _, _, _, "~p~isiinsA|~p~ihinsä";
    possessive_ess_sg: _, _, _, _, "~p~enAAn|~p~enänsä";
    possessive_ess_pl: _, _, _, _, "~p~inAAn|~p~inänsä";
    possessive_com_pl: _, _, _, _, "~p~ineen|~p~inensä";
    ),
    // Korpi: 3 words (korpi, rupi, koipi). Like Siipi, except in 1 cell: possessive_ess_sg _3.
    declension!("korpi", 3, "~pi", like Siipi;
    stem noun_nom_sg: "~p- like parvi";
    stem pl: "~v- like purkaminen";
    stem noun_gen_pl: "~pi- like osa";
    possessive_ess_sg: _, _, _, _, "~penAAn|~penansa";
    ),
    // Renki: 3 words (renki, tinki, meininki). Like Vilunki, except in 4 cells: possessive_par_pl _3, possessive_ess_sg _3, possessive_ess_pl _3, possessive_com_pl _3.
    declension!("renki", 3, "~ki", like Vilunki;
    stem noun_nom_sg: "~ki- like levy";
    stem pl: "~g- like lähetti";
    possessive_par_pl: _, _, _, _, "~kejAAn|~kejänsä";
    possessive_ess_sg: _, _, _, _, "~kinAAn|~kinänsä";
    possessive_ess_pl: _, _, _, _, "~keinAAn|~keinänsä";
    possessive_com_pl: _, _, _, _, "~keineen|~keinensä";
    ),
    // Bokserit: 3 words (polttarit, kroaatit, bokserit). Like Värit, except in 7 cells: possessive_ine_pl _3, possessive_ela_pl _3, possessive_ade_pl _3, possessive_abl_pl _3, possessive_all_pl _3, possessive_tra_pl _3, ….
    declension!("bokserit", 3, "~it", like Värit;
    stem noun_gen_pl: "~- like ankkuri";
    possessive_ine_pl: _, _, _, _, "~eissAAn|~eissansa";
    possessive_ela_pl: _, _, _, _, "~eistAAn|~eistansa";
    possessive_ade_pl: _, _, _, _, "~eillAAn|~eillansa";
    possessive_abl_pl: _, _, _, _, "~eiltAAn|~eiltansa";
    possessive_all_pl: _, _, _, _, "~eilleen|~eillensa";
    possessive_tra_pl: _, _, _, _, "~eikseen|~eiksensa";
    possessive_abe_pl: _, _, _, _, "~eittAAn|~eittansa";
    ),
    // Opas: 3 words (opas, lipas, matkaopas).
    declension!("opas", 3, "~~s";
    stem noun_nom_sg: "~~s- like tunnus";
    stem pl: "~p~- like rengas";
    stem noun_gen_pl: "~p~i- like rengas";
    ill: "~p~aseen", _;
    ess: "~p~ana", _;
    possessive_nom_sg: "~p~ani", "~p~asi", "~p~amme", "~p~anne", "~p~ansa";
    possessive_ill_sg: "~p~aseeni", "~p~aseesi", "~p~aseemme", "~p~aseenne", "~p~aseensa";
    possessive_ess_sg: "~p~anani", "~p~anasi", "~p~anamme", "~p~ananne", "~p~anaan|~p~anansa";
    ),
    // Lapsi: 3 words (lapsi, opetuslapsi, sotalapsi). Like Taival, except in 31 cells: nom sg, gen_ pl, par sg, par pl, ill sg, ill pl, ….
    declension!("lapsi", 3, "~p~i", like Taival;
    nom: "~p~i", _;
    gen_: _, "~p~ien|~~ten";
    par: "~~ta", "~p~ia";
    ill: "~p~een", "~p~iin";
    possessive_gen_pl: "~p~ieni|~~teni", "~p~iesi|~~tesi", "~p~iemme|~~temme", "~p~ienne|~~tenne", "~p~iensa|~~tensa";
    possessive_par_sg: "~~tani", "~~tasi", "~~tamme", "~~tanne", "~~taan|~~tansa";
    possessive_par_pl: "~p~iani", "~p~iasi", "~p~iamme", "~p~ianne", "~p~iaan|~p~iansa";
    possessive_ill_sg: "~p~eeni", "~p~eesi", "~p~eemme", "~p~eenne", "~p~eensa";
    possessive_ill_pl: "~p~iini", "~p~iisi", "~p~iimme", "~p~iinne", "~p~iinsa";
    ),
    // Markkinat: 3 words (kakkulat, talvikisat, markkinat). Like Nappula, except in 68 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("markkinat", 3, "~at", like Nappula as NOM_AS_COM;
    stem sg: "~A- like sakset";
    nom: "~at", "~at";
    gen_: derived, _;
    par: derived, _;
    ine: derived, _;
    ela: derived, _;
    ill: derived, _;
    ade: derived, _;
    abl: derived, _;
    all: derived, _;
    ess: derived, _;
    tra: derived, _;
    abe: derived, _;
    possessive_nom_sg: derived, derived, derived, derived, derived;
    possessive_par_sg: derived, derived, derived, derived, derived;
    possessive_ine_sg: derived, derived, derived, derived, derived;
    possessive_ela_sg: derived, derived, derived, derived, derived;
    possessive_ill_sg: derived, derived, derived, derived, derived;
    possessive_ade_sg: derived, derived, derived, derived, derived;
    possessive_abl_sg: derived, derived, derived, derived, derived;
    possessive_all_sg: derived, derived, derived, derived, derived;
    possessive_ess_sg: derived, derived, derived, derived, derived;
    possessive_tra_sg: derived, derived, derived, derived, derived;
    possessive_abe_sg: derived, derived, derived, derived, derived;
    ),
    // Kierre: 3 words (vesipyörre, kierre, piirre). Like Jänne, except in 0 cells: .
    declension!("kierre", 3, "~r~", like Jänne;
    stem noun_nom_sg: "~r~- like jänne";
    ),
    // Ahdin: 3 words (kudin, johdin, ahdin). Like Osoitin, except in 6 cells: gen_ pl, possessive_gen_pl _1sg, possessive_gen_pl _2sg, possessive_gen_pl _1pl, possessive_gen_pl _2pl, possessive_gen_pl _3.
    declension!("ahdin", 3, "~d~n", like Osoitin;
    stem noun_nom_sg: "~d~n- like tunnus";
    gen_: _, "~t~mien|~d~nten";
    possessive_gen_pl: "~t~mieni|~d~nteni", "~t~miesi|~d~ntesi", "~t~miemme|~d~ntemme", "~t~mienne|~d~ntenne", "~t~miensA|~d~ntensa";
    ),
    // Valvojaiset: 3 words (karhunpeijaiset, arpajaiset, valvojaiset). Like Sakset, except in 0 cells: .
    declension!("valvojaiset", 3, "~et", like Sakset;
    stem noun_gen_pl: "~- like purkaminen";
    ),
    // Hässäkkä: 3 words (lehdykkä, hässäkkä, räpäkkä). Like Selkä, except in 0 cells: .
    declension!("hässäkkä", 3, "~kä", like Selkä;
    stem pl: "~- like perintä";
    stem noun_gen_pl: "~- like hässäkkä";
    ),
];
