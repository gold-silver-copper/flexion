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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem rest__: $s1:tt;)?
     $(stem rest_: $s2:tt;)?
     $(stem rest: $s3:tt;)?
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
    stem rest__: "~- like agitō";
    stem rest_: "~āv- like agitō";
    stem rest: "~āt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Modulor: 357 words (for, cōnor, minor, māchinor, vōciferor, lupor).
    conjugation!("modulor", 357, "~or";
    stem rest__: "~- like modulor";
    stem rest_: "~āt- like modulor";
    stem rest: "~āt- like modulor";
    ind_act_pres_1: "~or", _;
    ind_act_pres_2: "~āris|~āre", _;
    ind_act_pres_3: "~ātur", "~antur";
    ind_act_fut_3: _, "~ābuntur";
    subj_act_pres_1: "~er", "~ēmur";
    subj_act_pres_2: "~ēris|~ēre", "~ēminī";
    subj_act_pres_3: _, "~entur";
    subj_act_impf_1: "~ārer", "~ārēmur";
    subj_act_impf_2: "~ārēris|~ārēre", "~ārēminī";
    subj_act_impf_3: "~ārētur", "~ārentur";
    imperative_act_pres_2: "~āre", _;
    imperative_act_fut_3: derived, "~antor";
    infinitive_act: "~ārī", _, _;
    participle: _, _, _, "~andus";
    gerund: _, _, _, _, derived, _;
    ),
    // Sūmō: 265 words (ferō, eō, sūmō, edō, agō, frīgēscō).
    conjugation!("sūmō", 265, "~ō";
    stem lemma: "~- like agō";
    stem rest__: "~- like agō";
    stem rest_: "~s- like agitō";
    stem rest: "~t- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Habeō: 256 words (abhorreō, moneō, aveō, oleō, stupeō, egeō). Like Sūmō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("habeō", 256, "~eō", like Sūmō;
    stem lemma: "~- like habeō";
    stem rest__: "~- like habeō";
    stem rest_: "~u- like agitō";
    stem rest: "~it- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Mūniō: 219 words (fīniō, hirriō, bombiō, poliō, audiō, odiō).
    conjugation!("mūniō", 219, "~iō";
    stem lemma: "~- like mūniō";
    stem rest__: "~- like mūniō";
    stem rest_: "~īv- like mūniō";
    stem rest: "~īt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Quiēscō: 152 words (albēscō, grandēscō, siccēscō, alēscō, mānēscō, cornēscō).
    conjugation!("quiēscō", 152, "~scō";
    stem lemma: "~sc- like agō";
    stem rest__: "~sc- like agō";
    stem rest_: "~v- like mūniō";
    stem rest: "~t- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Compercō: 116 words (crūdēscō, vigēscō, cānēscō, senēscō, dulcēscō, tumēscō). Like Sūmō, except in 9 cells: ind_pass_perf_1 sg, imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, infinitive_act fut, ….
    conjugation!("compercō", 116, "~cō", like Sūmō;
    stem lemma: "~c- like compercō";
    stem rest__: "~c- like agō";
    ind_pass_perf_1: derived, _;
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, derived;
    infinitive_pass: _, derived, _;
    participle: _, derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Minuō: 62 words (futuō, spuō, luō, cōnfutuō, acuō, exuō). Like Habeō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("minuō", 62, "~uō", like Habeō;
    stem lemma: "~u- like minuō";
    stem rest__: "~u- like agō";
    stem rest: "~ūt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Olfaciō: 53 words (faciō, patefaciō, satisfaciō, candefaciō, abolēfaciō, labefaciō).
    conjugation!("olfaciō", 53, "~aciō";
    stem lemma: "~- like faciō";
    stem rest__: "~aci- like faciō";
    stem rest_: "~ēc- like agitō";
    stem rest: "~act- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_fut_2: _, "~ītōte";
    imperative_pass_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Comprehendō: 49 words (dēfendō, comprehendō, fendō, dēpendō, pendō, suspendō).
    conjugation!("comprehendō", 49, "~e~dō";
    stem lemma: "~e~d- like compercō";
    stem rest__: "~e~d- like agō";
    stem rest_: "~e~d- like agitō";
    stem rest: "~ē~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Excipiō: 44 words (praeficiō, accipiō, excipiō, recipiō, suscipiō, prōficiō).
    conjugation!("excipiō", 44, "~i~iō";
    stem lemma: "~i~- like excipiō";
    stem rest__: "~i~i- like faciō";
    stem rest_: "~ē~- like agitō";
    stem rest: "~e~t- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Teneō: 41 words (abarceō, doceō, sorbeō, teneō, taedeō, absorbeō). Like Habeō.
    conjugation!("teneō", 41, "~eō", like Habeō;
    stem rest: "~t- like agitō";
    ),
    // Cubō: 39 words (vetō, domō, incubō, tonō, votō, cubō). Like Agitō, except in 5 cells: ind_pass_pres_2 sg, ind_pass_impf_2 sg, ind_pass_fut_2 sg, subj_pass_pres_2 sg, subj_pass_impf_2 sg.
    conjugation!("cubō", 39, "~ō", like Agitō;
    stem rest_: "~u- like agitō";
    stem rest: "~it- like agitō";
    ind_pass_pres_2: "~āris", _;
    ind_pass_impf_2: "~ābāris", _;
    ind_pass_fut_2: "~āberis", _;
    subj_pass_pres_2: "~ēris", _;
    subj_pass_impf_2: "~ārēris", _;
    ),
    // Compescō: 38 words (alō, volō, mālō, molō, parcō, tremō). Like Sūmō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("compescō", 38, "~ō", like Sūmō;
    stem lemma: "~- like compercō";
    stem rest_: "~u- like agitō";
    stem rest: "~it- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Prōdō: 34 words (abdō, vēndō, dēdō, bibō, dīdō, condō). Like Compescō.
    conjugation!("prōdō", 34, "~ō", like Compescō;
    stem rest_: "~id- like agitō";
    ),
    // Circumscrībō: 31 words (prōscrībō, bubō, nūbō, scrībō, dēscrībō, trānscrībō).
    conjugation!("circumscrībō", 31, "~bō";
    stem lemma: "~b- like compercō";
    stem rest__: "~b- like agō";
    stem rest_: "~ps- like agitō";
    stem rest: "~pt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Petō: 29 words (petō, impetō, competō, rūdō, ābaetō, appetō). Like Compescō.
    conjugation!("petō", 29, "~ō", like Compescō;
    stem rest_: "~īv- like mūniō";
    stem rest: "~īt- like agitō";
    ),
    // Vertō: 28 words (vortō, vertō, ābītō, bētō, invertō, convertō).
    conjugation!("vertō", 28, "~tō";
    stem lemma: "~t- like compercō";
    stem rest__: "~t- like agō";
    stem rest_: "~t- like agitō";
    stem rest: "~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Mittō: 28 words (mittō, prōmittō, āmittō, committō, admittō, remittō).
    conjugation!("mittō", 28, "~ittō";
    stem lemma: "~itt- like compercō";
    stem rest__: "~itt- like agō";
    stem rest_: "~īs- like agitō";
    stem rest: "~iss- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Impleō: 28 words (dēleō, expleō, neō, fleō, adimpleō, compleō). Like Habeō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("impleō", 28, "~eō", like Habeō;
    stem lemma: "~- like impleō";
    stem rest_: "~ēv- like mūniō";
    stem rest: "~ēt- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Labōrō: 28 words (abambulō, labōrō, abortō, cursō, abequitō, lāpsō). Like Agitō, except in 26 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("labōrō", 28, "~ō", like Agitō;
    stem rest: "~āt- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
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
    // Dēfīgō: 28 words (frīgō, fīgō, sūgō, discingō, succingō, praefīgō).
    conjugation!("dēfīgō", 28, "~gō";
    stem lemma: "~g- like compercō";
    stem rest__: "~g- like agō";
    stem rest_: "~x- like agitō";
    stem rest: "~ct- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Perstringō: 27 words (cōnfingō, dēpingō, effingō, fingō, pingō, stringō).
    conjugation!("perstringō", 27, "~ingō";
    stem lemma: "~ing- like compercō";
    stem rest__: "~ing- like agō";
    stem rest_: "~īnx- like agitō";
    stem rest: "~ict- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Obeō: 27 words (adeō, dispereō, exeō, queō, ineō, intereō). Like Mūniō, except in 46 cells: ind_act_pres_1 sg, ind_act_pres_3 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ….
    conjugation!("obeō", 27, "~eō", like Mūniō;
    stem rest_: "~- like obeō";
    stem rest: "~it- like agitō";
    ind_act_pres_1: "~eō", _;
    ind_act_pres_3: _, "~eunt";
    ind_act_impf_1: "~ībam", "~ībāmus";
    ind_act_impf_2: "~ībās", "~ībātis";
    ind_act_impf_3: "~ībat", "~ībant";
    ind_act_fut_1: "~ībō", "~ībimus";
    ind_act_fut_2: "~ībis", "~ībitis";
    ind_act_fut_3: "~ībit", "~ībunt";
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
    subj_pass_pres_1: "~ear", "~eāmur";
    subj_pass_pres_2: "~eāris|~eāre", "~eāminī";
    subj_pass_pres_3: "~eātur", "~eantur";
    imperative_act_fut_3: _, "~euntō";
    imperative_pass_fut_3: _, "~euntor";
    participle: _, _, _, "~eundus";
    gerund: _, "~eundī", "~eundō", "~eundum", _, _;
    ),
    // Mōlior: 27 words (dēmōlior, mōlior, mentior, partior, ōrdior, blandior). Like Modulor, except in 22 cells: ind_act_pres_1 sg, ind_act_pres_2 sg, ind_act_pres_3 sg, ind_act_pres_3 pl, ind_act_fut_3 pl, subj_act_pres_1 sg, ….
    conjugation!("mōlior", 27, "~ior", like Modulor;
    stem rest__: "~- like mōlior";
    stem rest_: "~īt- like modulor";
    stem rest: "~īt- like modulor";
    ind_act_pres_1: "~ior", _;
    ind_act_pres_2: "~īris|~īre", _;
    ind_act_pres_3: "~ītur", "~iuntur";
    ind_act_fut_3: _, "~ientur";
    subj_act_pres_1: "~iar", "~iāmur";
    subj_act_pres_2: "~iāris|~iāre", "~iāminī";
    subj_act_pres_3: _, "~iantur";
    subj_act_impf_1: "~īrer", "~īrēmur";
    subj_act_impf_2: "~īrēris|~īrēre", "~īrēminī";
    subj_act_impf_3: "~īrētur", "~īrentur";
    imperative_act_pres_2: "~īre", _;
    imperative_act_fut_3: derived, "~iuntor";
    infinitive_act: "~īrī", _, _;
    participle: _, _, _, "~iendus|~iundus";
    gerund: _, _, _, _, derived, _;
    ),
    // Dēdūcō: 26 words (abdūcō, trādūcō, prōdūcō, condūcō, ēdūcō, dēdūcō).
    conjugation!("dēdūcō", 26, "~ūcō";
    stem lemma: "~ūc- like dēdūcō";
    stem rest__: "~ūc- like agō";
    stem rest_: "~ūx- like agitō";
    stem rest: "~uct- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Moveō: 26 words (foveō, moveō, voveō, removeō, admoveō, āmoveō).
    conjugation!("moveō", 26, "~oveō";
    stem lemma: "~ov- like impleō";
    stem rest__: "~ov- like habeō";
    stem rest_: "~ōv- like agitō";
    stem rest: "~ōt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Repōnō: 26 words (pōnō, expōnō, oppōnō, antepōnō, prōpōnō, dēpōnō).
    conjugation!("repōnō", 26, "~ōnō";
    stem lemma: "~ōn- like compercō";
    stem rest__: "~ōn- like agō";
    stem rest_: "~os- like repōnō";
    stem rest: "~os- like repōnō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Mergō: 25 words (mergō, dēmergō, immergō, ēmergō, spargō, cōnspargō). Like Dēfīgō.
    conjugation!("mergō", 25, "~gō", like Dēfīgō;
    stem rest_: "~s- like agitō";
    stem rest: "~s- like agitō";
    ),
    // Compungō: 25 words (compungō, interpungō, iungō, abiungō, adiungō, sēiungō). Like Dēfīgō with `ˇ` after each pattern.
    conjugation!("compungō", 25, "~gōˇ", like Dēfīgō with "ˇ";
    ),
    // Dēpellō: 24 words (pellō, vellō, appellō, dēpellō, prōpellō, repellō).
    conjugation!("dēpellō", 24, "~e~lō";
    stem lemma: "~e~l- like compercō";
    stem rest__: "~e~l- like agō";
    stem rest_: "~u~- like agitō";
    stem rest: "~u~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Tueor: 23 words (polliceor, fateor, cōnfiteor, vereor, reor, intueor). Like Modulor, except in 22 cells: ind_act_pres_1 sg, ind_act_pres_2 sg, ind_act_pres_3 sg, ind_act_pres_3 pl, ind_act_fut_3 pl, subj_act_pres_1 sg, ….
    conjugation!("tueor", 23, "~eor", like Modulor;
    stem rest__: "~- like tueor";
    stem rest_: "~it- like modulor";
    stem rest: "~it- like modulor";
    ind_act_pres_1: "~eor", _;
    ind_act_pres_2: "~ēris|~ēre", _;
    ind_act_pres_3: "~ētur", "~entur";
    ind_act_fut_3: _, "~ēbuntur";
    subj_act_pres_1: "~ear", "~eāmur";
    subj_act_pres_2: "~eāris|~eāre", "~eāminī";
    subj_act_pres_3: _, "~eantur";
    subj_act_impf_1: "~ērer", "~ērēmur";
    subj_act_impf_2: "~ērēris|~ērēre", "~ērēminī";
    subj_act_impf_3: "~ērētur", "~ērentur";
    imperative_act_pres_2: "~ēre", _;
    imperative_act_fut_3: derived, "~entor";
    infinitive_act: "~ērī", _, _;
    participle: _, _, _, "~endus";
    gerund: _, _, _, _, derived, _;
    ),
    // Exsarciō: 22 words (crociō, saciō, farciō, micciō, effarciō, sarciō). Like Sūmō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("exsarciō", 22, "~ciō", like Sūmō;
    stem lemma: "~c- like mūniō";
    stem rest__: "~c- like mūniō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Praecīdō: 22 words (sīdō, comprēndō, expandō, prēndō, reprēndō, apprēndō).
    conjugation!("praecīdō", 22, "~dō";
    stem lemma: "~d- like compercō";
    stem rest__: "~d- like agō";
    stem rest_: "~d- like agitō";
    stem rest: "~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Solvō: 21 words (volvō, solvō, ēvolvō, convolvō, dissolvō, resolvō). Like Quiēscō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("solvō", 21, "~vō", like Quiēscō;
    stem lemma: "~v- like minuō";
    stem rest__: "~v- like agō";
    stem rest_: "~v- like agitō";
    stem rest: "~ūt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Pāscor: 21 words (ulcīscor, īrāscor, nāscor, vēscor, aborīscor, oblīvīscor). Like Modulor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("pāscor", 21, "~scor", like Modulor;
    stem lemma: "~sc- like nāscor";
    stem rest__: "~sc- like nāscor";
    stem rest_: "~t- like modulor";
    stem rest: "~t- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Coniciō: 21 words (rabiō, iniciō, abiciō, subiciō, adiciō, circumiciō). Like Excipiō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("coniciō", 21, "~~iō", like Excipiō;
    stem lemma: "~~- like excipiō";
    stem rest__: "~~i- like faciō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Nuō: 20 words (nuō, sinō, minuō, innuō, annuō, dēsinō). Like Vīsō with `ˇ` after each pattern, except in 5 cells: ind_pass_perf_1 sg, infinitive_act fut, infinitive_pass perf, participle perf_pass, participle fut_pass.
    conjugation!("nuō", 20, "~ōˇ", like Vīsō with "ˇ";
    ind_pass_perf_1: "~tus sumˇ", _;
    infinitive_act: _, _, "~tūrum esseˇ";
    infinitive_pass: _, "~tum esseˇ", _;
    participle: _, "~tusˇ", _, "~endusˇ";
    ),
    // Cēdō: 19 words (cēdō, discēdō, concēdō, excēdō, prōcēdō, retrōcēdō).
    conjugation!("cēdō", 19, "~ēdō";
    stem lemma: "~ēd- like compercō";
    stem rest__: "~ēd- like agō";
    stem rest_: "~ess- like agitō";
    stem rest: "~ess- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ēgredior: 19 words (ēgredior, gradior, aggredior, trānsgredior, antegredior, congredior). Like Modulor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("ēgredior", 19, "~dior", like Modulor;
    stem lemma: "~d- like ēgredior";
    stem rest__: "~di- like ēgredior";
    stem rest_: "~ss- like modulor";
    stem rest: "~ss- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Prōferō: 18 words (trānsferō, praeferō, īnferō, adferō, auferō, cōnferō).
    conjugation!("prōferō", 18, "~ferō";
    stem lemma: "~fer- like referō";
    stem rest__: "~fer- like referō";
    stem rest_: "~tul- like agitō";
    stem rest: "~lāt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Gerō: 17 words (gerō, ēgerō, aggerō, adgerō, regerō, suggerō).
    conjugation!("gerō", 17, "~rō";
    stem lemma: "~r- like compercō";
    stem rest__: "~r- like agō";
    stem rest_: "~ss- like agitō";
    stem rest: "~st- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Nectō: 17 words (flectō, īnflectō, genūflectō, reflectō, pectō, plectō). Like Dēfīgō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("nectō", 17, "~ctō", like Dēfīgō;
    stem lemma: "~ct- like compercō";
    stem rest__: "~ct- like agō";
    stem rest: "~x- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dispiciō: 17 words (perspiciō, retrōspiciō, aspiciō, respiciō, cōnspiciō, suspiciō).
    conjugation!("dispiciō", 17, "~iciō";
    stem lemma: "~ic- like excipiō";
    stem rest__: "~ici- like faciō";
    stem rest_: "~ex- like agitō";
    stem rest: "~ect- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Trahō: 17 words (extrahō, trahō, abstrahō, subtrahō, attrahō, circumtrahō).
    conjugation!("trahō", 17, "~ahō";
    stem lemma: "~ah- like compercō";
    stem rest__: "~ah- like agō";
    stem rest_: "~āx- like agitō";
    stem rest: "~act- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Compingō: 17 words (attingō, contingō, impingō, adtingō, compingō, cōnfringō). Like Perstringō.
    conjugation!("compingō", 17, "~ingō", like Perstringō;
    stem rest_: "~ēg- like agitō";
    stem rest: "~āct- like agitō";
    ),
    // Exigō: 17 words (adigō, redigō, prōdigō, abigō, trānsigō, colligō). Like Compingō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("exigō", 17, "~igō", like Compingō;
    stem lemma: "~ig- like compercō";
    stem rest__: "~ig- like agō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Effundō: 16 words (profundō, cōnfundō, īnfundō, tundō, effundō, circumfundō).
    conjugation!("effundō", 16, "~undō";
    stem lemma: "~und- like compercō";
    stem rest__: "~und- like agō";
    stem rest_: "~ūd- like agitō";
    stem rest: "~ūs- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Corripiō: 16 words (ēripiō, abripiō, dēripiō, alliciō, arripiō, dīripiō). Like Excipiō.
    conjugation!("corripiō", 16, "~i~iō", like Excipiō;
    stem rest_: "~i~u- like agitō";
    ),
    // Accumbō: 16 words (recumbō, dēlambō, accumbō, adcumbō, succumbō, prōcumbō).
    conjugation!("accumbō", 16, "~m~ō";
    stem lemma: "~m~- like compercō";
    stem rest__: "~m~- like agō";
    stem rest_: "~~u- like agitō";
    stem rest: "~~it- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Intersaepiō: 16 words (apiō, saepiō, speciō, sēpiō, reperiō, praesaepiō). Like Mūniō.
    conjugation!("intersaepiō", 16, "~iō", like Mūniō;
    stem rest_: "~s- like agitō";
    stem rest: "~t- like agitō";
    ),
    // Ārdeō: 15 words (audeō, ārdeō, rīdeō, persuādeō, subrīdeō, suādeō). Like Mergō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("ārdeō", 15, "~deō", like Mergō;
    stem lemma: "~d- like impleō";
    stem rest__: "~d- like habeō";
    stem rest_: "~s- like mūniō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ērigō: 15 words (dīrigō, ērigō, dīligō, intelligō, porrigō, negligō). Like Exigō.
    conjugation!("ērigō", 15, "~igō", like Exigō;
    stem rest_: "~ēx- like agitō";
    stem rest: "~ēct- like agitō";
    ),
    // Dēsinō: 14 words (linō, oblinō, collinō, perlinō, circumlinō, succinō). Like Impleō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("dēsinō", 14, "~inō", like Impleō;
    stem lemma: "~in- like compercō";
    stem rest__: "~in- like agō";
    stem rest_: "~ēv- like agitō";
    stem rest: "~it- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dīcō: 14 words (abdīcō, dīcō, addīcō, interdīcō, maledīcō, benedīcō).
    conjugation!("dīcō", 14, "~īcō";
    stem lemma: "~īc- like dēdūcō";
    stem rest__: "~īc- like agō";
    stem rest_: "~īx- like agitō";
    stem rest: "~ict- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Tingō: 14 words (lingō, cingō, accingō, circumcingō, incingō, recingō).
    conjugation!("tingō", 14, "~i~gō";
    stem lemma: "~i~g- like compercō";
    stem rest__: "~i~g- like agō";
    stem rest_: "~ī~x- like agitō";
    stem rest: "~ī~ct- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Regō: 14 words (regō, neglegō, tegō, dētegō, intellegō, prōtegō). Like Ērigō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("regō", 14, "~egō", like Ērigō;
    stem lemma: "~eg- like compercō";
    stem rest__: "~eg- like agō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Struō: 14 words (dēstruō, struō, cōnstruō, īnstruō, exstruō, obstruō). Like Minuō.
    conjugation!("struō", 14, "~uō", like Minuō;
    stem rest_: "~ūx- like agitō";
    stem rest: "~ūct- like agitō";
    ),
    // Rumpō: 14 words (irrumpō, interrumpō, abrumpō, rumpō, ērumpō, corrumpō).
    conjugation!("rumpō", 14, "~um~ō";
    stem lemma: "~um~- like compercō";
    stem rest__: "~um~- like agō";
    stem rest_: "~ū~- like agitō";
    stem rest: "~u~t- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fodiō: 14 words (fodiō, cōnfodiō, affodiō, dēfodiō, effodiō, īnfodiō).
    conjugation!("fodiō", 14, "~odiō";
    stem lemma: "~od- like excipiō";
    stem rest__: "~odi- like faciō";
    stem rest_: "~ōd- like agitō";
    stem rest: "~oss- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Asserviō: 14 words (veniō, reveniō, interveniō, perveniō, adveniō, ēveniō). Like Excipiō, except in 26 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("asserviō", 14, "~e~iō", like Excipiō;
    stem lemma: "~e~- like ēveniō";
    stem rest__: "~e~- like mūniō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    participle: _, "~e~tum", _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Autumnāscit: 14 words (vesperāscit, cōnfit, pluit, advesperāscit, ningit, compluit). Like Tueor, except in 78 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("autumnāscit", 14, "~it", like Tueor;
    ind_act_pres_1: "~it", derived;
    ind_act_pres_2: derived, derived;
    ind_act_pres_3: "~it", derived;
    ind_act_impf_1: derived, derived;
    ind_act_impf_2: derived, derived;
    ind_act_impf_3: "~ēbat", derived;
    ind_act_fut_1: derived, derived;
    ind_act_fut_2: derived, derived;
    ind_act_fut_3: "~et", derived;
    ind_act_perf_1: derived, derived;
    ind_act_perf_2: derived, derived;
    ind_act_perf_3: derived, derived;
    ind_act_plup_1: derived, derived;
    ind_act_plup_2: derived, derived;
    ind_act_plup_3: derived, derived;
    ind_act_futp_1: derived, derived;
    ind_act_futp_2: derived, derived;
    ind_act_futp_3: derived, derived;
    ind_pass_pres_3: "~itur", _;
    ind_pass_impf_3: "~ēbātur", _;
    ind_pass_fut_3: "~ētur", _;
    subj_act_pres_1: derived, derived;
    subj_act_pres_2: derived, derived;
    subj_act_pres_3: "~at", derived;
    subj_act_impf_1: derived, derived;
    subj_act_impf_2: derived, derived;
    subj_act_impf_3: "~eret", derived;
    subj_act_perf_1: derived, derived;
    subj_act_perf_2: derived, derived;
    subj_act_perf_3: derived, derived;
    subj_act_plup_1: derived, derived;
    subj_act_plup_2: derived, derived;
    subj_act_plup_3: derived, derived;
    subj_pass_pres_3: "~ātur", _;
    subj_pass_impf_3: "~erētur", _;
    imperative_act_pres_2: derived, derived;
    imperative_act_fut_2: derived, _;
    imperative_act_fut_3: "~itō", derived;
    imperative_pass_fut_3: "~itor", _;
    infinitive_act: "~ere", derived, derived;
    infinitive_pass: "~ī", _, _;
    participle: _, _, _, "~endum";
    supine: _, _, _, derived, derived, _;
    ),
    // Illābor: 13 words (lābor, collābor, dēlābor, conlābor, dīlābor, ēlābor). Like Modulor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("illābor", 13, "~bor", like Modulor;
    stem lemma: "~b- like nāscor";
    stem rest__: "~b- like nāscor";
    stem rest_: "~ps- like modulor";
    stem rest: "~ps- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ēvehō: 13 words (vehō, advehō, āvehō, convehō, dēvehō, prōvehō). Like Ērigō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("ēvehō", 13, "~ehō", like Ērigō;
    stem lemma: "~eh- like compercō";
    stem rest__: "~eh- like agō";
    stem rest: "~ect- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cōnsequor: 13 words (sequor, līquor, prōsequor, persequor, adsequor, assequor). Like Modulor, except in 6 cells: imperative_act_fut_3 sg, infinitive_act fut, participle fut_act, gerund abl, supine acc, supine abl.
    conjugation!("cōnsequor", 13, "~quor", like Modulor;
    stem lemma: "~qu- like cōnsequor";
    stem rest__: "~qu- like nāscor";
    stem rest_: "~- like cōnsequor";
    imperative_act_fut_3: derived, _;
    infinitive_act: _, _, "~cūtūrum esse|~quūtūrum esse";
    participle: _, _, "~cūtūrus|~quūtūrus", _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, "~cūtum|~quūtum", "~cūtū|~quūtū", _;
    ),
    // Madeō: 13 words (studeō, pāreō, horreō, vegeō, sileō, affleō). Like Habeō, except in 23 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("madeō", 13, "~eō", like Habeō;
    stem lemma: "~- like madeō";
    stem rest: "~it- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    ),
    // Excellō: 13 words (fallō, excellō, praecellō, extollō, attollō, adtollō).
    conjugation!("excellō", 13, "~lō";
    stem lemma: "~l- like agō";
    stem rest__: "~l- like agō";
    stem rest_: "~lu- like mūniō";
    stem rest: "~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Scindō: 13 words (rescindō, scindō, abscindō, pandō, interscindō, findō). Like Praecīdō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("scindō", 13, "~ndō", like Praecīdō;
    stem lemma: "~nd- like compercō";
    stem rest__: "~nd- like agō";
    stem rest: "~ss- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Īnsistō: 13 words (cōnsistō, dēsistō, existō, persistō, resistō, subsistō).
    conjugation!("īnsistō", 13, "~~s~ō";
    stem lemma: "~~s~- like īnsistō";
    stem rest__: "~~s~- like agō";
    stem rest_: "~t~~- like agitō";
    imperative_act_fut_3: derived, _;
    infinitive_act: _, _, "~t~~ūrum esse";
    infinitive_pass: _, "~t~~um esse", _;
    participle: _, "~t~~um", _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Feriō: 12 words (ūniō, feriō, absiliō, ambiō, salliō, lascīviō). Like Mūniō.
    conjugation!("feriō", 12, "~iō", like Mūniō;
    stem rest_: "~i- like agitō";
    ),
    // Fugiō: 12 words (fugiō, subterfugiō, effugiō, trānsfugiō, dēfugiō, supterfugiō). Like Rumpō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("fugiō", 12, "~u~iō", like Rumpō;
    stem lemma: "~u~- like fugiō";
    stem rest__: "~u~i- like faciō";
    stem rest: "~u~it- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Obloquor: 12 words (loquor, alloquor, circumloquor, colloquor, ēloquor, interloquor). Like Cōnsequor, except in 1 cell: participle fut_act.
    conjugation!("obloquor", 12, "~quor", like Cōnsequor;
    stem rest_: "~cūt- like modulor";
    stem rest: "~cūt- like modulor";
    participle: _, _, derived, _;
    ),
    // Possideō: 12 words (resideō, assideō, īnsideō, obsideō, adsideō, dēsideō).
    conjugation!("possideō", 12, "~ideō";
    stem lemma: "~id- like habeō";
    stem rest__: "~id- like habeō";
    stem rest_: "~ēd- like agitō";
    stem rest: "~ess- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Intersum: 12 words (possum, absum, adsum, īnsum, dēsum, obsum). Like Modulor, except in 28 cells: ind_act_pres_1 sg, ind_act_pres_2 sg, ind_act_pres_3 sg, ind_act_pres_3 pl, ind_act_fut_3 pl, subj_act_pres_1 sg, ….
    conjugation!("intersum", 12, "~sum", like Modulor;
    stem rest__: "~- like adsum";
    stem rest_: "~fu- like agitō";
    ind_act_pres_1: "~sum", _;
    ind_act_pres_2: "~es", _;
    ind_act_pres_3: "~est", "~sunt";
    ind_act_fut_3: _, "~erunt";
    subj_act_pres_1: "~sim|~siem|~fuam", "~sīmus|~siēmus|~fuāmus";
    subj_act_pres_2: "~sīs|~siēs|~fuās", "~sītis|~siētis|~fuātis";
    subj_act_pres_3: _, "~sint|~sient|~fuant";
    subj_act_impf_1: "~essem|~forem", "~essēmus|~forēmus";
    subj_act_impf_2: "~essēs|~forēs", "~essētis|~forētis";
    subj_act_impf_3: "~esset|~foret", "~essent|~forent";
    imperative_act_pres_2: "~es", _;
    imperative_act_fut_2: _, "~estōte";
    imperative_act_fut_3: derived, "~suntō";
    infinitive_act: "~esse", _, "~futūrum esse|~fore";
    participle: _, _, _, derived;
    gerund: _, derived, derived, derived, _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Cōnsulō: 11 words (cōnsulō, incumbō, furō, occulō, dēserō, exserō). Like Compescō.
    conjugation!("cōnsulō", 11, "~ō", like Compescō;
    stem rest: "~t- like agitō";
    ),
    // Contineō: 11 words (contineō, abstineō, dētineō, attineō, distineō, obtineō). Like Corripiō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("contineō", 11, "~i~eō", like Corripiō;
    stem lemma: "~i~- like impleō";
    stem rest__: "~i~- like habeō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Opprimō: 11 words (dēprimō, exprimō, reprimō, comprimō, imprimō, opprimō). Like Cēdō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("opprimō", 11, "~imō", like Cēdō;
    stem lemma: "~im- like compercō";
    stem rest__: "~im- like agō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Incidō: 11 words (dēcidō, accidō, coincidō, incidō, intercidō, occidō). Like Prōdō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("incidō", 11, "~idō", like Prōdō;
    stem lemma: "~id- like agō";
    stem rest__: "~id- like agō";
    stem rest_: "~id- like mūniō";
    stem rest: "~ās- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cernō: 11 words (cernō, concernō, sēcernō, discernō, dēcernō, condēcernō).
    conjugation!("cernō", 11, "~e~nō";
    stem lemma: "~e~n- like compercō";
    stem rest__: "~e~n- like agō";
    stem rest_: "~~ēv- like agitō";
    stem rest: "~~ēt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Percutiō: 11 words (patiō, recutiō, excutiō, quatiō, discutiō, percutiō). Like Gerō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("percutiō", 11, "~tiō", like Gerō;
    stem lemma: "~t- like excipiō";
    stem rest__: "~ti- like faciō";
    stem rest: "~ss- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ūtor: 11 words (revertor, deūtor, ūtor, abūtor, amplector, complector). Like Modulor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("ūtor", 11, "~tor", like Modulor;
    stem lemma: "~t- like nāscor";
    stem rest__: "~t- like nāscor";
    stem rest_: "~s- like modulor";
    stem rest: "~s- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Abblandior: 11 words (admētior, dīmētior, remētior, commētior, conmētior, ēmētior). Like Mōlior.
    conjugation!("abblandior", 11, "~ior", like Mōlior;
    stem rest_: "~it- like modulor";
    stem rest: "~it- like modulor";
    ),
    // Attondeō: 11 words (tondeō, respondeō, dēspondeō, spondeō, attondeō, dētondeō).
    conjugation!("attondeō", 11, "~o~deō";
    stem lemma: "~o~d- like impleō";
    stem rest__: "~o~d- like habeō";
    stem rest_: "~o~d- like agitō";
    stem rest: "~ō~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Micō: 11 words (sonō, personō, plicō, intonō, increpō, affricō). Like Cubō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("micō", 11, "~ō", like Cubō;
    stem lemma: "~- like agitō";
    stem rest: "~āt- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Calvor: 11 words (queror, fruor, conqueror, locor, calvor, perdor). Like Tueor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("calvor", 11, "~or", like Tueor;
    stem lemma: "~- like cōnsequor";
    stem rest__: "~- like nāscor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Tergeō: 10 words (abstergeō, mulgeō, urgeō, perurgeō, extergeō, tergeō). Like Mergō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("tergeō", 10, "~geō", like Mergō;
    stem lemma: "~g- like habeō";
    stem rest__: "~g- like habeō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Legō: 10 words (legō, allegō, relegō, perlegō, praelegō, sublegō). Like Regō.
    conjugation!("legō", 10, "~egō", like Regō;
    stem rest_: "~ēg- like agitō";
    ),
    // Terō: 10 words (terō, interō, dēterō, atterō, exterō, prōterō).
    conjugation!("terō", 10, "~e~ō";
    stem lemma: "~e~- like compercō";
    stem rest__: "~e~- like agō";
    stem rest_: "~~īv- like agitō";
    stem rest: "~~īt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Contendō: 10 words (ostendō, contendō, attendō, intendō, portendō, distendō). Like Praecīdō.
    conjugation!("contendō", 10, "~dō", like Praecīdō;
    stem rest: "~t- like agitō";
    ),
    // Vīsō: 10 words (revīsō, vīsō, invīsō, incessō, praevertō, intervīsō). Like Compescō.
    conjugation!("vīsō", 10, "~ō", like Compescō;
    stem rest_: "~- like agitō";
    stem rest: "~- like agitō";
    ),
    // Abnuō: 10 words (ajō, uō, īcō, fervō, abscondō, abnuō). Like Vīsō.
    conjugation!("abnuō", 10, "~ō", like Vīsō;
    stem rest: "~it- like agitō";
    ),
    // Aperiō: 10 words (aperiō, rapiō, operiō, saliō, adaperiō, cooperiō). Like Intersaepiō.
    conjugation!("aperiō", 10, "~iō", like Intersaepiō;
    stem rest_: "~u- like agitō";
    ),
    // Contorqueō: 9 words (torqueō, extorqueō, dētorqueō, contorqueō, distorqueō, intorqueō). Like Sūmō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("contorqueō", 9, "~queō", like Sūmō;
    stem lemma: "~qu- like impleō";
    stem rest__: "~qu- like habeō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Inveniō: 9 words (inveniō, conveniō, circumveniō, contrāveniō, dēveniō, praeveniō). Like Asserviō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("inveniō", 9, "~e~iō", like Asserviō;
    stem lemma: "~e~- like mūniō";
    stem rest: "~e~t- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Sternō: 9 words (sternō, cōnsternō, asternō, prōsternō, substernō, dēsternō). Like Cernō.
    conjugation!("sternō", 9, "~e~nō", like Cernō;
    stem rest_: "~~āv- like agitō";
    stem rest: "~~āt- like agitō";
    ),
    // Sentiō: 9 words (sentiō, cōnsentiō, dissentiō, praesentiō, assentiō, adsentiō).
    conjugation!("sentiō", 9, "~e~tiō";
    stem lemma: "~e~t- like mūniō";
    stem rest__: "~e~t- like mūniō";
    stem rest_: "~ē~s- like agitō";
    stem rest: "~ē~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fluō: 9 words (fluō, superfluō, circumfluō, cōnfluō, diffluō, interfluō). Like Struō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("fluō", 9, "~uō", like Struō;
    stem lemma: "~u- like agō";
    stem rest: "~ūx- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Īnsiliō: 9 words (exiliō, trānsiliō, dēsiliō, exsiliō, īnsiliō, subsiliō). Like Corripiō, except in 28 cells: ind_act_pres_2 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 sg, ind_pass_impf_2 sg, ….
    conjugation!("īnsiliō", 9, "~i~iō", like Corripiō;
    stem rest__: "~i~- like mūniō";
    stem rest_: "~i~u- like mūniō";
    stem rest: "~u~t- like agitō";
    ind_act_pres_2: "~i~īs", _;
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
    imperative_act_pres_2: "~i~ī", _;
    imperative_act_fut_2: _, "~i~ītōte";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: "~i~īre", _;
    imperative_pass_fut_2: "~i~ītor", _;
    infinitive_pass: "~i~īrī", _, _;
    participle: _, _, _, "~i~iendus";
    gerund: _, _, _, _, derived, _;
    ),
    // Interstinguō: 9 words (distinguō, stinguō, extinguō, exstinguō, restinguō, intinguō). Like Tingō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("interstinguō", 9, "~i~guō", like Tingō;
    stem lemma: "~i~gu- like minuō";
    stem rest__: "~i~gu- like agō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dēmorior: 9 words (morior, ēmorior, commorior, conmorior, intermorior, dēmorior). Like Abblandior, except in 4 cells: imperative_act_fut_3 sg, gerund abl, supine acc, supine abl.
    conjugation!("dēmorior", 9, "~ior", like Abblandior;
    stem lemma: "~- like ēgredior";
    stem rest__: "~i- like ēgredior";
    stem rest_: "~tu- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, "~tuum", "~tuū", _;
    ),
    // Pervincō: 8 words (vincō, convincō, ēvincō, dēvincō, prōvincō, revincō).
    conjugation!("pervincō", 8, "~in~ō";
    stem lemma: "~in~- like compercō";
    stem rest__: "~in~- like agō";
    stem rest_: "~ī~- like agitō";
    stem rest: "~i~t- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Coorior: 8 words (aborior, orior, experior, adorior, coorior, exorior). Like Mōlior.
    conjugation!("coorior", 8, "~ior", like Mōlior;
    stem rest_: "~t- like modulor";
    stem rest: "~t- like modulor";
    ),
    // Immisceō: 8 words (admisceō, commisceō, immisceō, intermisceō, ammisceō, ēmisceō). Like Quiēscō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("immisceō", 8, "~sceō", like Quiēscō;
    stem lemma: "~sc- like impleō";
    stem rest__: "~sc- like habeō";
    stem rest_: "~scu- like agitō";
    stem rest: "~xt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cohaereō: 8 words (adhaereō, haereō, adhēreō, cohaereō, inhaereō, cohēreō). Like Mergō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("cohaereō", 8, "~reō", like Mergō;
    stem lemma: "~r- like habeō";
    stem rest__: "~r- like habeō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dēpudet: 8 words (miseret, oportet, taedet, piget, pertaedet, dēpudet). Like Autumnāscit, except in 13 cells: ind_act_pres_1 sg, ind_act_pres_3 sg, ind_act_fut_3 sg, ind_pass_pres_3 sg, ind_pass_fut_3 sg, subj_act_pres_3 sg, ….
    conjugation!("dēpudet", 8, "~et", like Autumnāscit;
    stem rest_: "~u- like dēpudet";
    stem rest: "~it- like dēpudet";
    ind_act_pres_1: "~et", _;
    ind_act_pres_3: "~et", _;
    ind_act_fut_3: "~ēbit", _;
    ind_pass_pres_3: "~ētur", _;
    ind_pass_fut_3: "~ēbitur", _;
    subj_act_pres_3: "~eat", _;
    subj_act_impf_3: "~ēret", _;
    subj_pass_pres_3: "~eātur", _;
    subj_pass_impf_3: "~ērētur", _;
    imperative_act_fut_3: "~ētō", _;
    imperative_pass_fut_3: "~ētor", _;
    infinitive_act: "~ēre", _, _;
    infinitive_pass: "~ērī", _, _;
    ),
    // Gaudeō: 7 words (gaudeō, aggaudeō, adgaudeō, congaudeō, pergaudeō, praegaudeō). Like Modulor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("gaudeō", 7, "~audeō", like Modulor;
    stem lemma: "~aud- like gaudeō";
    stem rest__: "~aud- like habeō";
    stem rest_: "~āvīs- like modulor";
    stem rest: "~āvīs- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Comedō: 7 words (comedō, ambedō, adedō, exedō, peredō, subedō). Like Possideō, except in 23 cells: ind_act_pres_1 pl, ind_act_pres_2 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ….
    conjugation!("comedō", 7, "~edō", like Possideō;
    stem lemma: "~- like comedō";
    stem rest: "~ēs- like comedō";
    ind_act_pres_1: _, "~edimus";
    ind_act_pres_2: _, "~editis|~ēstis";
    ind_act_impf_1: "~edēbam", "~edēbāmus";
    ind_act_impf_2: "~edēbās", "~edēbātis";
    ind_act_impf_3: "~edēbat", "~edēbant";
    ind_act_fut_1: "~edam", "~edēmus";
    ind_act_fut_2: "~edēs", "~edētis";
    ind_act_fut_3: "~edet", _;
    subj_act_pres_3: "~edat|~edit|~ēst", _;
    imperative_act_pres_2: _, "~edite|~ēste";
    imperative_act_fut_2: "~editō|~ēstō", _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    participle: "~edēns", _, _, _;
    gerund: _, "~edendī", "~edendō", "~edendum", _, _;
    ),
    // Capiō: 7 words (ūsūcapiō, iaciō, laciō, pariō, capiō, cōnsuēfaciō). Like Excipiō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("capiō", 7, "~a~iō", like Excipiō;
    stem lemma: "~a~- like excipiō";
    stem rest__: "~a~i- like faciō";
    stem rest: "~a~t- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Maneō: 7 words (remaneō, maneō, permaneō, commaneō, ēmaneō, immaneō).
    conjugation!("maneō", 7, "~a~eō";
    stem lemma: "~a~- like habeō";
    stem rest__: "~a~- like habeō";
    stem rest_: "~ā~s- like agitō";
    stem rest: "~ā~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Augeō: 7 words (augeō, lūgeō, adaugeō, ēlūgeō, frīgeō, exaugeō). Like Tergeō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("augeō", 7, "~geō", like Tergeō;
    stem lemma: "~g- like impleō";
    stem rest_: "~x- like agitō";
    stem rest: "~ct- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Ferveō: 7 words (ferveō, cōnferveō, dēferveō, īnferveō, perferveō, referveō). Like Intersum, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("ferveō", 7, "~veō", like Intersum;
    stem lemma: "~v- like ferveō";
    stem rest__: "~v- like habeō";
    stem rest_: "~bu- like agitō";
    stem rest: "~vit- like dēpudet";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Īnsīdō: 7 words (resīdō, assīdō, subsīdō, cōnsīdō, īnsīdō, dēsīdō). Like Possideō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("īnsīdō", 7, "~īdō", like Possideō;
    stem lemma: "~īd- like agō";
    stem rest__: "~īd- like agō";
    stem rest_: "~ēd- like mūniō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dēpendeō: 7 words (strīdeō, commordeō, remordeō, ammordeō, dēpendeō, prōpendeō). Like Ārdeō.
    conjugation!("dēpendeō", 7, "~deō", like Ārdeō;
    stem rest_: "~d- like agitō";
    ),
    // Peragō: 7 words (peragō, satagō, reagō, circumagō, abagō, vervagō). Like Compingō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("peragō", 7, "~agō", like Compingō;
    stem lemma: "~ag- like agō";
    stem rest__: "~ag- like agō";
    stem rest_: "~ēg- like mūniō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cognōscō: 7 words (conōscō, cognōscō, recognōscō, agnōscō, praecognōscō, adgnōscō). Like Quiēscō with `ˇ` after each pattern.
    conjugation!("cognōscō", 7, "~scōˇ", like Quiēscō with "ˇ";
    ),
    // Coquō: 7 words (coquō, concoquō, discoquō, excoquō, dēcoquō, recoquō). Like Extruō with `ˇ` after each pattern.
    conjugation!("coquō", 7, "~ōˇ", like Extruō with "ˇ";
    ),
    // Īnserō: 7 words (disserō, serō, asserō, īnserō, cōnserō, adserō). Like Dēsinō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("īnserō", 7, "~erō", like Dēsinō;
    stem lemma: "~er- like compercō";
    stem rest__: "~er- like agō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cēnseō: 6 words (recēnseō, cēnseō, paeniteō, accēnseō, percēnseō, poeniteō). Like Habeō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("cēnseō", 6, "~eō", like Habeō;
    stem lemma: "~- like impleō";
    stem rest: "~- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Videō: 6 words (videō, invideō, revideō, prōvideō, pervideō, praevideō). Like Possideō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("videō", 6, "~ideō", like Possideō;
    stem lemma: "~id- like impleō";
    stem rest_: "~īd- like agitō";
    stem rest: "~īs- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Pergō: 6 words (surgō, resurgō, pergō, cōnsurgō, exsurgō, exurgō). Like Dēfīgō.
    conjugation!("pergō", 6, "~gō", like Dēfīgō;
    stem rest_: "~rēx- like agitō";
    stem rest: "~rēct- like agitō";
    ),
    // Advīvō: 6 words (vīvō, convīvō, revīvō, advīvō, supervīvō, fīvō). Like Solvō, except in 35 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ind_pass_impf_1 sg, ….
    conjugation!("advīvō", 6, "~vō", like Solvō;
    stem rest_: "~x- like agitō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    ind_pass_perf_1: derived, _;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_pres_3: _, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    subj_pass_impf_3: _, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~vitor", derived;
    infinitive_act: "~vere", _, "~ctūrum esse";
    infinitive_pass: _, "~ctum esse", _;
    participle: _, "~ctum", _, "~vendum";
    ),
    // Cōnstō: 6 words (praestō, cōnstō, distō, īnstō, prōstō, antestō). Like Cubō, except in 3 cells: infinitive_act fut, infinitive_pass perf, participle perf_pass.
    conjugation!("cōnstō", 6, "~ō", like Cubō;
    stem rest_: "~it- like agitō";
    infinitive_act: _, _, "~ātūrum esse";
    infinitive_pass: _, "~ātum esse", _;
    participle: _, "~ātus", _, _;
    ),
    // Innectō: 6 words (cōnectō, connectō, subnectō, innectō, annectō, adnectō). Like Nectō.
    conjugation!("innectō", 6, "~ctō", like Nectō;
    stem rest_: "~xu- like agitō";
    ),
    // Eximō: 6 words (dirimō, eximō, redimō, adimō, interimō, perimō). Like Excipiō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("eximō", 6, "~i~ō", like Excipiō;
    stem lemma: "~i~- like compercō";
    stem rest__: "~i~- like agō";
    stem rest: "~ē~pt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Vinciō: 6 words (vinciō, revinciō, dēvinciō, ēvinciō, praevinciō, circumvinciō). Like Tingō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("vinciō", 6, "~i~ciō", like Tingō;
    stem lemma: "~i~c- like mūniō";
    stem rest__: "~i~c- like mūniō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dēfluō: 6 words (affluō, adfluō, īnfluō, dēfluō, effluō, perfluō). Like Fluō, except in 26 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("dēfluō", 6, "~uō", like Fluō;
    stem rest: "~ūx- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
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
    // Exsorbeō: 6 words (exsorbeō, exorbeō, obsorbeō, persorbeō, resorbeō, apsorbeō). Like Ferveō, except in 10 cells: ind_pass_perf_3 sg, ind_pass_plup_3 sg, ind_pass_futp_3 sg, subj_pass_perf_3 sg, subj_pass_plup_3 sg, imperative_act_fut_3 sg, ….
    conjugation!("exsorbeō", 6, "~beō", like Ferveō;
    stem lemma: "~b- like impleō";
    stem rest__: "~b- like habeō";
    stem rest: "~pt- like agitō";
    ind_pass_perf_3: derived, _;
    ind_pass_plup_3: derived, _;
    ind_pass_futp_3: derived, _;
    subj_pass_perf_3: derived, _;
    subj_pass_plup_3: derived, _;
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cōnfugiō: 6 words (refugiō, aufugiō, perfugiō, cōnfugiō, diffugiō, profugiō). Like Fugiō, except in 32 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ind_pass_impf_1 sg, ….
    conjugation!("cōnfugiō", 6, "~u~iō", like Fugiō;
    stem rest: "~u~it- like dēpudet";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    ind_pass_perf_1: derived, _;
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
    // Apīscor: 6 words (apīscor, dēpecīscor, pacīscor, compacīscor, compecīscor, dēpacīscor). Like Pāscor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("apīscor", 6, "~īscor", like Pāscor;
    stem lemma: "~īsc- like nāscor";
    stem rest__: "~īsc- like nāscor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Ēdiscō: 6 words (discō, condiscō, addiscō, ēdiscō, dēdiscō, perdiscō).
    conjugation!("ēdiscō", 6, "~s~ō";
    stem lemma: "~s~- like agō";
    stem rest__: "~s~- like agō";
    stem rest_: "~di~- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Deicō: 6 words (deicō, indeicō, addeicō, prodeicō, contrādeicō, interdeicō).
    conjugation!("deicō", 6, "~e~cō";
    stem lemma: "~e~c- like compercō";
    stem rest__: "~e~c- like agō";
    stem rest_: "~e~x- like agitō";
    stem rest: "~~ct- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Scandō: 6 words (scandō, retundō, oppandō, superscandō, suprascandō, dispandō). Like Praecīdō with `ˇ` after each pattern.
    conjugation!("scandō", 6, "~dōˇ", like Praecīdō with "ˇ";
    ),
    // Occurrō: 5 words (carrō, antecurrō, circumcurrō, converrō, convorrō). Like Gerō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("occurrō", 5, "~rō", like Gerō;
    stem lemma: "~r- like agō";
    stem rest_: "~r- like mūniō";
    stem rest: "~s- like occurrō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Sūmō2: 5 words (prōmō, dēmō, cōmō, exprōmō, dēprōmō). Like Compescō.
    conjugation!("sūmō", 5, "~ō", like Compescō;
    stem rest_: "~ps- like agitō";
    stem rest: "~pt- like agitō";
    ),
    // Vēnumdō: 5 words (dō, circundō, circumdō, vēnundō, vēnumdō). Like Agitō, except in 48 cells: ind_act_pres_1 pl, ind_act_pres_2 pl, ind_act_impf_1 sg, ind_act_impf_1 pl, ind_act_impf_2 sg, ind_act_impf_2 pl, ….
    conjugation!("vēnumdō", 5, "~ō", like Agitō;
    stem rest_: "~ed- like agitō";
    stem rest: "~at- like agitō";
    ind_act_pres_1: _, "~amus";
    ind_act_pres_2: _, "~atis";
    ind_act_impf_1: "~abam", "~abāmus";
    ind_act_impf_2: "~abās", "~abātis";
    ind_act_impf_3: "~abat", "~abant";
    ind_act_fut_1: "~abō", "~abimus";
    ind_act_fut_2: "~abis", "~abitis";
    ind_act_fut_3: "~abit", "~abunt";
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
    subj_pass_impf_1: "~arer", "~arēmur";
    subj_pass_impf_2: "~arēris|~arēre", "~arēminī";
    subj_pass_impf_3: "~arētur", "~arentur";
    imperative_act_pres_2: _, "~ate";
    imperative_act_fut_2: "~atō", "~atōte";
    imperative_pass_pres_2: "~are", _;
    imperative_pass_fut_2: "~ator", _;
    infinitive_pass: "~arī", _, _;
    ),
    // Serviō: 5 words (serviō, dormiō, hymniō, saeviō, īnsāniō). Like Mūniō, except in 33 cells: ind_act_perf_1 sg, ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ….
    conjugation!("serviō", 5, "~iō", like Mūniō;
    stem rest: "~īt- like dēpudet";
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
    ind_pass_perf_1: derived, _;
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
    // Adipīscor: 5 words (proficīscor, adipīscor, comminīscor, indipīscor, minīscor). Like Asserviō, except in 18 cells: ind_pass_pres_3 sg, ind_pass_pres_3 pl, ind_pass_impf_3 sg, ind_pass_impf_3 pl, ind_pass_fut_3 sg, ind_pass_fut_3 pl, ….
    conjugation!("adipīscor", 5, "~i~īscor", like Asserviō;
    stem lemma: "~i~īsc- like nāscor";
    stem rest__: "~i~īsc- like nāscor";
    stem rest_: "~e~t- like modulor";
    stem rest: "~e~t- like modulor";
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_3: derived, derived;
    ind_pass_fut_3: derived, derived;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_act_fut_2: _, derived;
    imperative_act_fut_3: derived, _;
    imperative_pass_fut_3: derived, derived;
    infinitive_pass: derived, derived, _;
    participle: _, derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Astō: 5 words (restō, obstō, astō, perstō, adstō). Like Cōnstō, except in 34 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ind_pass_impf_1 sg, ….
    conjugation!("astō", 5, "~ō", like Cōnstō;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    ind_pass_perf_1: derived, _;
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
    infinitive_pass: _, derived, _;
    participle: _, derived, _, "~andum";
    ),
    // Occinō: 5 words (concinō, accinō, praecinō, incinō, occinō). Like Eximō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("occinō", 5, "~i~ō", like Eximō;
    stem lemma: "~i~- like agō";
    stem rest_: "~i~u- like mūniō";
    stem rest: "~e~t- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Decet: 5 words (licet, decet, addecet, dēdecet, condecet). Like Dēpudet, except in 26 cells: ind_act_pres_3 pl, ind_act_impf_3 pl, ind_act_fut_3 pl, ind_act_perf_3 pl, ind_act_plup_3 pl, ind_act_futp_3 pl, ….
    conjugation!("decet", 5, "~et", like Dēpudet;
    stem rest: "~it- like modulor";
    ind_act_pres_3: _, "~ent";
    ind_act_impf_3: _, "~ēbant";
    ind_act_fut_3: _, "~ēbunt";
    ind_act_perf_3: _, "~uērunt|~uēre";
    ind_act_plup_3: _, "~uerant";
    ind_act_futp_3: _, "~uerint";
    ind_pass_pres_3: derived, _;
    ind_pass_impf_3: derived, _;
    ind_pass_fut_3: derived, _;
    ind_pass_perf_3: derived, _;
    ind_pass_plup_3: derived, _;
    ind_pass_futp_3: derived, _;
    subj_act_pres_3: _, "~eant";
    subj_act_impf_3: _, "~ērent";
    subj_act_perf_3: _, "~uerint";
    subj_act_plup_3: _, "~uissent";
    subj_pass_pres_3: derived, _;
    subj_pass_impf_3: derived, _;
    subj_pass_perf_3: derived, _;
    subj_pass_plup_3: derived, _;
    imperative_act_fut_3: _, "~entō";
    imperative_pass_fut_3: derived, _;
    infinitive_pass: derived, derived, _;
    participle: _, derived, _, derived;
    ),
    // Supersedeō: 5 words (sedeō, circumsedeō, persedeō, supersedeō, suprāsedeō). Like Comedō, except in 9 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, infinitive_pass fut, participle fut_act, ….
    conjugation!("supersedeō", 5, "~edeō", like Comedō;
    stem lemma: "~ed- like habeō";
    stem rest__: "~ed- like habeō";
    stem rest: "~ess- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    infinitive_pass: _, _, derived;
    participle: _, _, derived, _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Clueō: 5 words (clueō, langueō, abnueō, cēveō, connīveō). Like Habeō.
    conjugation!("clueō", 5, "~eō", like Habeō;
    stem rest_: "~- like mūniō";
    ),
    // Ambigō: 5 words (convergō, assurgō, ambigō, īnsurgō, adsurgō). Like Pergō, except in 26 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("ambigō", 5, "~gō", like Pergō;
    stem rest: "~rēct- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
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
    // Contremīscō: 5 words (resipīscō, ingemīscō, dēlitīscō, conticīscō, contremīscō). Like Habeō, except in 9 cells: ind_pass_perf_1 sg, imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, infinitive_act fut, ….
    conjugation!("contremīscō", 5, "~īscō", like Habeō;
    stem lemma: "~īsc- like compercō";
    stem rest__: "~īsc- like agō";
    ind_pass_perf_1: derived, _;
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, derived;
    infinitive_pass: _, derived, _;
    participle: _, derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Colō: 5 words (colō, recolō, excolō, circumcolō, accolō). Like Cōnsulō with `ˇ` after each pattern.
    conjugation!("colō", 5, "~ōˇ", like Cōnsulō with "ˇ";
    ),
    // Clangō: 5 words (cadō, stertō, clangō, accanō, obtexō). Like Compescō, except in 29 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("clangō", 5, "~ō", like Compescō;
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    imperative_pass_fut_3: "~itor", _;
    infinitive_act: "~ere", _, derived;
    infinitive_pass: _, derived, _;
    participle: _, derived, _, "~endum|~undum";
    ),
    // Caveō: 4 words (caveō, faveō, paveō, praecaveō).
    conjugation!("caveō", 4, "~aveō";
    stem lemma: "~av- like impleō";
    stem rest__: "~av- like habeō";
    stem rest_: "~ā- like caveō";
    stem rest: "~aut- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Cieō: 4 words (cieō, excieō, concieō, percieō). Like Mūniō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("cieō", 4, "~ieō", like Mūniō;
    stem lemma: "~i- like impleō";
    stem rest__: "~i- like habeō";
    stem rest: "~it- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Patior: 4 words (patior, compatior, perpetior, conpatior). Like Ēgredior, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("patior", 4, "~tior", like Ēgredior;
    stem lemma: "~t- like ēgredior";
    stem rest__: "~ti- like ēgredior";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fīō: 4 words (fīō, obsolefīō, dēfīō, vacēfīō). Like Modulor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("fīō", 4, "~īō", like Modulor;
    stem lemma: "~- like fīō";
    stem rest__: "~ī- like faciō";
    stem rest_: "~act- like modulor";
    stem rest: "~act- like fīō";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Relinquō: 4 words (dēlinquō, relinquō, linquō, dērelinquō).
    conjugation!("relinquō", 4, "~inquō";
    stem lemma: "~inqu- like minuō";
    stem rest__: "~inqu- like agō";
    stem rest_: "~īqu- like agitō";
    stem rest: "~ict- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Sapiō: 4 words (cupiō, sapiō, concupiō, supersapiō). Like Mūniō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("sapiō", 4, "~iō", like Mūniō;
    stem lemma: "~- like fugiō";
    stem rest__: "~i- like faciō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Abeō: 4 words (abeō, prōdeō, pereō, redeō). Like Obeō, except in 32 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_pres_3 pl, ind_pass_impf_1 sg, ….
    conjugation!("abeō", 4, "~eō", like Obeō;
    stem rest: "~it- like dēpudet";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_pres_3: _, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_impf_3: _, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_fut_3: _, derived;
    ind_pass_perf_1: derived, _;
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
    // Ēvellō: 4 words (dīvellō, convellō, ēvellō, refellō). Like Dēpellō.
    conjugation!("ēvellō", 4, "~e~lō", like Dēpellō;
    stem rest_: "~e~l- like agitō";
    ),
    // Currō: 4 words (currō, percurrō, prōcurrō, superexcurrō).
    conjugation!("currō", 4, "~~rō";
    stem lemma: "~~r- like compercō";
    stem rest__: "~~r- like agō";
    stem rest_: "~cu~r- like mūniō";
    stem rest: "~~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Nītor: 4 words (nītor, ēnītor, cōnītor, connītor). Like Ūtor.
    conjugation!("nītor", 4, "~tor", like Ūtor;
    stem rest_: "~- like nītor";
    stem rest: "~- like nītor";
    ),
    // Obtundō: 4 words (obtundō, pertundō, contundō, extundō). Like Effundō.
    conjugation!("obtundō", 4, "~undō", like Effundō;
    stem rest_: "~ud- like agitō";
    ),
    // Hauriō: 4 words (exhauriō, hauriō, dehauriō, perhauriō). Like Sūmō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("hauriō", 4, "~riō", like Sūmō;
    stem lemma: "~r- like mūniō";
    stem rest__: "~r- like mūniō";
    stem rest: "~st- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Comperiō: 4 words (comperiō, īciō, repperiō, conperiō). Like Intersaepiō.
    conjugation!("comperiō", 4, "~iō", like Intersaepiō;
    stem rest_: "~- like agitō";
    ),
    // Circumvehor: 4 words (circumvehor, praetervehor, praevehor, supervehor). Like Modulor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("circumvehor", 4, "~hor", like Modulor;
    stem lemma: "~h- like nāscor";
    stem rest__: "~h- like nāscor";
    stem rest_: "~ct- like modulor";
    stem rest: "~ct- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Adnītor: 4 words (innītor, obnītor, adnītor, annītor). Like Ūtor, except in 5 cells: infinitive_act perf, infinitive_act fut, participle fut_act, supine acc, supine abl.
    conjugation!("adnītor", 4, "~tor", like Ūtor;
    infinitive_act: _, "~xum esse|~sum esse", "~xūrum esse|~sūrum esse";
    participle: _, _, "~xūrus|~sūrus", _;
    supine: _, _, _, "~xum|~sum", "~xū|~sū", _;
    ),
    // Extruō: 4 words (tāgō, tragō, extruō, superextruō). Like Compescō, except in 3 cells: imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres.
    conjugation!("extruō", 4, "~ō", like Compescō;
    stem lemma: "~- like minuō";
    stem rest_: "~x- like agitō";
    stem rest: "~ct- like agitō";
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    ),
    // Frangō: 4 words (frangō, tangō, pangō, dēpangō). Like Peragō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("frangō", 4, "~angō", like Peragō;
    stem lemma: "~ang- like compercō";
    stem rest__: "~ang- like agō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fricō: 4 words (secō, fricō, dēfricō, praesecō). Like Micō.
    conjugation!("fricō", 4, "~ō", like Micō;
    stem rest: "~- like fricō";
    ),
    // Mulceō: 4 words (mulceō, commulceō, permulceō, remulceō). Like Compercō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("mulceō", 4, "~ceō", like Compercō;
    stem lemma: "~c- like impleō";
    stem rest__: "~c- like habeō";
    stem rest: "~s- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Apage: 4 words (sum, inquam, apage, coest). Like Adipīscor, except in 74 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("apage", 4, "~~", like Adipīscor;
    ind_act_pres_1: "~~", derived;
    ind_act_pres_2: derived, derived;
    ind_act_pres_3: derived, derived;
    ind_act_impf_1: derived, derived;
    ind_act_impf_2: derived, derived;
    ind_act_impf_3: derived, derived;
    ind_act_fut_1: derived, derived;
    ind_act_fut_2: derived, derived;
    ind_act_fut_3: derived, derived;
    ind_act_perf_1: derived, derived;
    ind_act_perf_2: derived, derived;
    ind_act_perf_3: derived, derived;
    ind_act_plup_1: derived, derived;
    ind_act_plup_2: derived, derived;
    ind_act_plup_3: derived, derived;
    ind_act_futp_1: derived, derived;
    ind_act_futp_2: derived, derived;
    ind_act_futp_3: derived, derived;
    subj_act_pres_1: derived, derived;
    subj_act_pres_2: derived, derived;
    subj_act_pres_3: derived, derived;
    subj_act_impf_1: derived, derived;
    subj_act_impf_2: derived, derived;
    subj_act_impf_3: derived, derived;
    subj_act_perf_1: derived, derived;
    subj_act_perf_2: derived, derived;
    subj_act_perf_3: derived, derived;
    subj_act_plup_1: derived, derived;
    subj_act_plup_2: derived, derived;
    subj_act_plup_3: derived, derived;
    imperative_act_pres_2: "~~", "~it~";
    imperative_act_fut_2: derived, _;
    imperative_act_fut_3: _, derived;
    infinitive_act: derived, derived, derived;
    participle: derived, _, _, derived;
    gerund: _, derived, derived, derived, _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Ēvādō: 3 words (vilipendō, circumsīdō, obsīdō). Like Praecīdō.
    conjugation!("ēvādō", 3, "~dō", like Praecīdō;
    stem rest_: "~s- like agitō";
    ),
    // Coniungō: 3 words (ēmungō, pungō, suffringō).
    conjugation!("coniungō", 3, "~u~gō";
    stem lemma: "~u~g- like compercō";
    stem rest__: "~u~g- like agō";
    stem rest_: "~ū~x- like agitō";
    stem rest: "~ū~ct- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Quaerō: 3 words (quaerō, conquaerō, disquīrō). Like Gerō.
    conjugation!("quaerō", 3, "~rō", like Gerō;
    stem rest_: "~sīv- like agitō";
    stem rest: "~sīt- like agitō";
    ),
    // Adliciō: 3 words (illiciō, inliciō, adliciō). Like Dispiciō.
    conjugation!("adliciō", 3, "~iciō", like Dispiciō;
    stem rest_: "~ēx- like agitō";
    ),
    // Stō: 3 words (stō, interstō, substō). Like Astō, except in 7 cells: ind_pass_perf_3 sg, ind_pass_plup_3 sg, ind_pass_futp_3 sg, subj_pass_perf_3 sg, subj_pass_plup_3 sg, infinitive_pass perf, ….
    conjugation!("stō", 3, "~ō", like Astō;
    stem rest_: "~et- like mūniō";
    ind_pass_perf_3: "~atum est", _;
    ind_pass_plup_3: "~atum erat", _;
    ind_pass_futp_3: "~atum erit", _;
    subj_pass_perf_3: "~atum sit", _;
    subj_pass_plup_3: "~atum esset", _;
    infinitive_pass: _, "~atum esse", _;
    participle: _, "~atum", _, _;
    ),
    // Dīluō: 3 words (suō, tribuō, dīluō). Like Minuō, except in 26 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("dīluō", 3, "~uō", like Minuō;
    stem rest: "~ūt- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
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
    // Misceō: 3 words (misceō, permisceō, remisceō). Like Immisceō, except in 8 cells: ind_pass_perf_1 sg, infinitive_act fut, infinitive_pass perf, infinitive_pass fut, participle perf_pass, participle fut_act, ….
    conjugation!("misceō", 3, "~sceō", like Immisceō;
    ind_pass_perf_1: "~stus sum", _;
    infinitive_act: _, _, "~xtūrum esse|~stūrum esse";
    infinitive_pass: _, "~xtum esse|~stum esse", "~xtum īrī|~stum īrī";
    participle: _, "~xtus|~stus", "~xtūrus|~stūrus", _;
    supine: _, _, _, "~xtum|~stum", "~xtū|~stū", _;
    ),
    // Iubeō: 3 words (iubeō, fidēiubeō, ioubeō). Like Exsorbeō.
    conjugation!("iubeō", 3, "~beō", like Exsorbeō;
    stem rest_: "~ss- like agitō";
    stem rest: "~ss- like agitō";
    ),
    // Fundō: 3 words (fundō, refundō, praefundō). Like Effundō, except in 26 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("fundō", 3, "~undō", like Effundō;
    stem rest: "~ūs- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
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
    // Scatō: 3 words (scatō, battō, abbattō). Like Compescō.
    conjugation!("scatō", 3, "~ō", like Compescō;
    stem rest: "~ūt- like agitō";
    ),
    // Explicō: 3 words (explicō, applicō, adplicō). Like Agitō, except in 8 cells: ind_pass_perf_1 sg, infinitive_act fut, infinitive_pass perf, infinitive_pass fut, participle perf_pass, participle fut_act, ….
    conjugation!("explicō", 3, "~ō", like Agitō;
    stem rest_: "~- like explicō";
    ind_pass_perf_1: "~itus sum", _;
    infinitive_act: _, _, "~ātūrum esse|~itūrum esse";
    infinitive_pass: _, "~ātum esse|~itum esse", "~ātum īrī|~itum īrī";
    participle: _, "~ātus|~itus", "~ātūrus|~itūrus", _;
    supine: _, _, _, "~ātum|~itum", "~ātū|~itū", _;
    ),
    // Dēfungor: 3 words (fungor, dēfungor, perfungor). Like Adipīscor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("dēfungor", 3, "~u~gor", like Adipīscor;
    stem lemma: "~u~g- like nāscor";
    stem rest__: "~u~g- like nāscor";
    stem rest_: "~ū~ct- like modulor";
    stem rest: "~ū~ct- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Fīdō: 3 words (fīdō, cōnfīdō, diffīdō). Like Ūtor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("fīdō", 3, "~dō", like Ūtor;
    stem lemma: "~d- like fīdō";
    stem rest__: "~d- like agō";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Libet: 3 words (libet, lubet, perlibet). Like Decet, except in 21 cells: ind_act_pres_3 pl, ind_act_impf_3 pl, ind_act_fut_3 pl, ind_act_perf_3 sg, ind_act_perf_3 pl, ind_act_plup_3 sg, ….
    conjugation!("libet", 3, "~et", like Decet;
    ind_act_pres_3: _, derived;
    ind_act_impf_3: _, derived;
    ind_act_fut_3: _, derived;
    ind_act_perf_3: "~uit|~itus est", derived;
    ind_act_plup_3: "~uerat|~itus erat", derived;
    ind_act_futp_3: "~uerit|~itus erit", derived;
    subj_act_pres_3: _, derived;
    subj_act_impf_3: _, derived;
    subj_act_perf_3: "~uerit|~itus sit", derived;
    subj_act_plup_3: "~uisset|~itus esset", derived;
    imperative_act_fut_3: _, derived;
    infinitive_act: _, "~uisse|~itum esse", "~itum īre";
    gerund: _, derived, derived, derived, _, _;
    ),
    // Coepī: 3 words (ōdī, coepī, recoepī). Like Libet, except in 11 cells: ind_act_pres_1 sg, ind_act_pres_3 sg, ind_act_impf_3 sg, ind_act_fut_3 sg, subj_act_pres_3 sg, subj_act_impf_3 sg, ….
    conjugation!("coepī", 3, "~ī", like Libet;
    stem rest_: "~- like agitō";
    stem rest: "~t- like agitō";
    ind_act_pres_1: "~ī", _;
    ind_act_pres_3: derived, _;
    ind_act_impf_3: derived, _;
    ind_act_fut_3: derived, _;
    subj_act_pres_3: derived, _;
    subj_act_impf_3: derived, _;
    imperative_act_fut_3: derived, _;
    infinitive_act: derived, _, _;
    participle: derived, _, _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Dētergeō: 3 words (dētergeō, immulgeō, inmulgeō). Like Tergeō, except in 23 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("dētergeō", 3, "~geō", like Tergeō;
    stem lemma: "~g- like madeō";
    stem rest: "~s- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    ),
    // Sufferō: 3 words (offerō, differō, sufferō).
    conjugation!("sufferō", 3, "~fferō";
    stem lemma: "~ffer- like referō";
    stem rest__: "~ffer- like referō";
    stem rest_: "~stul- like mūniō";
    stem rest: "~blāt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Antecapiō: 3 words (superiaciō, interiaciō, antecapiō). Like Capiō.
    conjugation!("antecapiō", 3, "~a~iō", like Capiō;
    stem rest: "~e~t- like agitō";
    ),
    // Commeminī: 3 words (meminī, commeminī, rememinī). Like Apage, except in 36 cells: ind_act_pres_1 sg, ind_act_pres_1 pl, ind_act_pres_2 sg, ind_act_pres_2 pl, ind_act_pres_3 sg, ind_act_pres_3 pl, ….
    conjugation!("commeminī", 3, "~i~ī", like Apage;
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
    imperative_act_pres_2: derived, derived;
    imperative_act_fut_2: "~e~tō", "~e~tōte";
    infinitive_act: "~i~isse", _, _;
    participle: "~i~ēns", _, _, _;
    ),
    // Contemnō: 3 words (vannō, temnō, contemnō).
    conjugation!("contemnō", 3, "~nō";
    stem lemma: "~n- like compercō";
    stem rest__: "~n- like agō";
    stem rest_: "~- like contemnō";
    ind_pass_perf_1: "~tus sum", _;
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, "~ptūrum esse|~tūrum esse";
    infinitive_pass: _, "~ptum esse|~tum esse", "~ptum īrī|~tum īrī";
    participle: _, "~ptus|~tus", "~ptūrus|~tūrus", _;
    gerund: _, _, _, _, derived, _;
    supine: _, _, _, "~ptum|~tum", "~ptū|~tū", _;
    ),
    // Dētorreō: 3 words (contorreō, dētorreō, extorreō).
    conjugation!("dētorreō", 3, "~orreō";
    stem lemma: "~orr- like impleō";
    stem rest__: "~orr- like habeō";
    stem rest_: "~orru- like agitō";
    stem rest: "~ōst- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Superstō: 3 words (superstō, circumstō, antistō). Like Cubō, except in 4 cells: ind_pass_perf_1 sg, infinitive_act fut, infinitive_pass perf, participle perf_pass.
    conjugation!("superstō", 3, "~ō", like Cubō;
    stem rest_: "~et- like agitō";
    ind_pass_perf_1: derived, _;
    infinitive_act: _, _, derived;
    infinitive_pass: _, derived, _;
    participle: _, derived, _, _;
    ),
    // Gignō: 3 words (gignō, ingignō, prōgignō).
    conjugation!("gignō", 3, "~ig~ō";
    stem lemma: "~ig~- like compercō";
    stem rest__: "~ig~- like agō";
    stem rest_: "~e~u- like agitō";
    stem rest: "~e~it- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Assiliō: 3 words (assiliō, prōsiliō, resiliō). Like Īnsiliō, except in 23 cells: ind_pass_pres_1 sg, ind_pass_pres_1 pl, ind_pass_pres_2 sg, ind_pass_pres_2 pl, ind_pass_impf_1 sg, ind_pass_impf_1 pl, ….
    conjugation!("assiliō", 3, "~i~iō", like Īnsiliō;
    stem lemma: "~i~- like ēveniō";
    stem rest: "~u~t- like labōrō";
    ind_pass_pres_1: derived, derived;
    ind_pass_pres_2: derived, derived;
    ind_pass_impf_1: derived, derived;
    ind_pass_impf_2: derived, derived;
    ind_pass_fut_1: derived, derived;
    ind_pass_fut_2: derived, derived;
    ind_pass_perf_1: derived, _;
    subj_pass_pres_1: derived, derived;
    subj_pass_pres_2: derived, derived;
    subj_pass_impf_1: derived, derived;
    subj_pass_impf_2: derived, derived;
    imperative_pass_pres_2: derived, _;
    imperative_pass_fut_2: derived, _;
    ),
    // Convalēscō: 3 words (convalēscō, coalēscō, tonēscō). Like Clangō, except in 16 cells: ind_pass_pres_3 sg, ind_pass_pres_3 pl, ind_pass_impf_3 sg, ind_pass_impf_3 pl, ind_pass_fut_3 sg, ind_pass_fut_3 pl, ….
    conjugation!("convalēscō", 3, "~ēscō", like Clangō;
    stem lemma: "~ēsc- like convalēscō";
    stem rest__: "~ēsc- like agō";
    stem rest: "~it- like modulor";
    ind_pass_pres_3: derived, derived;
    ind_pass_impf_3: derived, derived;
    ind_pass_fut_3: derived, derived;
    subj_pass_pres_3: derived, derived;
    subj_pass_impf_3: derived, derived;
    imperative_act_fut_3: derived, _;
    imperative_pass_fut_3: derived, derived;
    infinitive_pass: derived, _, _;
    participle: _, _, _, derived;
    gerund: _, _, _, _, derived, _;
    ),
    // Tuor: 3 words (compluor, conpluor, tuor). Like Modulor, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("tuor", 3, "~uor", like Modulor;
    stem lemma: "~u- like cōnsequor";
    stem rest__: "~u- like nāscor";
    stem rest_: "~ūt- like modulor";
    stem rest: "~ūt- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Adhērēscō: 3 words (adhērēscō, adhaerēscō, cohaerēscō). Like Apage, except in 2 cells: imperative_act_fut_3 sg, gerund abl.
    conjugation!("adhērēscō", 3, "~rē~cō", like Apage;
    stem lemma: "~rē~c- like convalēscō";
    stem rest__: "~rē~c- like agō";
    stem rest_: "~~- like agitō";
    stem rest: "~~- like modulor";
    imperative_act_fut_3: derived, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Iūxtāpōnō: 3 words (praedispōnō, praesuppōnō, iūxtāpōnō). Like Repōnō, except in 4 cells: infinitive_pass fut, participle fut_act, supine acc, supine abl.
    conjugation!("iūxtāpōnō", 3, "~ōnō", like Repōnō;
    stem rest_: "~osu- like agitō";
    stem rest: "~osit- like agitō";
    infinitive_pass: _, _, derived;
    participle: _, _, derived, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Adlavō: 3 words (lavō, perlavō, adlavō).
    conjugation!("adlavō", 3, "~a~ō";
    stem lemma: "~a~- like agitō";
    stem rest__: "~a~- like agitō";
    stem rest_: "~ā~- like agitō";
    stem rest: "~a~āt- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Effervēscō: 3 words (effervēscō, dēfervēscō, cōnfervēscō).
    conjugation!("effervēscō", 3, "~vēscō";
    stem lemma: "~vēsc- like agō";
    stem rest__: "~vēsc- like agō";
    stem rest_: "~- like effervēscō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Dīlūculat: 2 words (grandinat, dīlūculat). Like Libet, except in 15 cells: ind_act_pres_1 sg, ind_act_pres_3 sg, ind_act_impf_3 sg, ind_act_fut_3 sg, subj_act_pres_3 sg, subj_act_impf_3 sg, ….
    conjugation!("dīlūculat", 2, "~at", like Libet;
    stem rest_: "~āv- like dēpudet";
    ind_act_pres_1: "~at", _;
    ind_act_pres_3: "~at", _;
    ind_act_impf_3: "~ābat", _;
    ind_act_fut_3: "~ābit", _;
    subj_act_pres_3: "~et", _;
    subj_act_impf_3: "~āret", _;
    imperative_act_fut_3: "~ātō", _;
    infinitive_act: "~āre", _, derived;
    participle: "~āns", _, _, _;
    gerund: _, "~andī", "~andō", "~andum", _, _;
    supine: _, _, _, derived, derived, _;
    ),
    // Coerceo: 1 words (coerceo).
    conjugation!("coerceo", 1, "~e~eo";
    stem lemma: "~ë~- like impleō";
    stem rest__: "~ë~- like habeō";
    stem rest_: "~ë~u- like agitō";
    stem rest: "~ë~it- like agitō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Rēfert: 1 words (rēfert). Like Dīlūculat, except in 5 cells: ind_act_pres_1 sg, ind_act_pres_3 sg, subj_act_impf_3 sg, infinitive_act pres, gerund abl.
    conjugation!("rēfert", 1, "~fert", like Dīlūculat;
    stem rest__: "~fer- like rēfert";
    stem rest_: "~tul- like dēpudet";
    ind_act_pres_1: "~fert", _;
    ind_act_pres_3: "~fert", _;
    subj_act_impf_3: "~ferret", _;
    infinitive_act: "~ferre", _, _;
    gerund: _, _, _, _, derived, _;
    ),
    // Coemo: 1 words (coemo). Like Eximō, except in 5 cells: imperative_act_fut_3 sg, imperative_pass_pres_2 pl, imperative_pass_fut_3 sg, infinitive_act pres, gerund abl.
    conjugation!("coemo", 1, "~e~o", like Eximō;
    stem lemma: "~ë~- like compercō";
    stem rest__: "~ë~- like agō";
    imperative_act_fut_3: derived, _;
    imperative_pass_pres_2: _, derived;
    imperative_pass_fut_3: derived, _;
    infinitive_act: derived, _, _;
    gerund: _, _, _, _, derived, _;
    ),
];
