//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ति`, `वहति`: the stem and `ति`). A stem in two pieces (`~~ति`, `धरति`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A capital (`A`, `B`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~- like वहति\"`: the slot's cells are `वहति`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
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
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem future_act_3_sg: $s0:tt;)?
     $(stem perfect_act_3_sg: $s1:tt;)?
     $(stem present_: $s2:tt;)?
     $(stem present_ind_act_2_pl: $s3:tt;)?
     $(stem lemma: $s4:tt;)?
     $(stem present_ind_mid_1_sg: $s5:tt;)?
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
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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

/// What a conjugation adds after each pattern of the one it is like (`with "ि"`: `ि` after every pattern).
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (present_class: $v:ident) => {
        (0, crate::PresentClass::$v as u8)
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `वहति`: 261 words.
    वहति,
    /// Like `साधयति`: 108 words.
    साधयति,
    /// Like `लभते`: 108 words.
    लभते,
    /// Like `धरति`: 93 words.
    धरति,
    /// Like `वेश`: 43 words.
    वेश,
    /// Like `रमते`: 41 words.
    रमते,
    /// Like `जेष्यति`: 36 words.
    जेषयति,
    /// Like `आपत्`: 22 words.
    आपत,
    /// Like `चखान`: 21 words.
    चखान,
    /// Like `अग्रहीत्`: 21 words.
    अगरहीत,
    /// Like `दृणाति`: 19 words.
    दृणाति,
    /// Like `तनोति`: 16 words.
    तनोति,
    /// Like `आजीत्`: 10 words.
    आजीत,
    /// Like `शक्नोति`: 8 words.
    शकनोति,
    /// Like `ररंह`: 8 words.
    ररंह,
    /// Like `भिनत्ति`: 8 words.
    भिनतति,
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
    /// Like `तुष्टाव`: 5 words.
    तुषटाव,
    /// Like `जिगाय`: 5 words.
    जिगाय,
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
    /// Like `उवाप`: 4 words.
    उवाप,
    /// Like `आनयति`: 4 words.
    आनयति,
    /// Like `चकार`: 3 words.
    चकार,
    /// Like `आचरति`: 3 words.
    आचरति,
    /// Like `चष्टे`: 3 words.
    चषटे,
    /// Like `ऋच्छति`: 3 words.
    ऋचछति,
    /// Like `बाधिष्यते`: 3 words.
    बाधिषयते,
    /// Like `अधुक्षत्`: 3 words.
    अधुकषत,
    /// Like `रुरोह`: 3 words.
    रुरोह,
    /// Like `विवेद`: 3 words.
    विवेद,
    /// Like `अचेत्`: 3 words.
    अचेत,
    /// Like `रौति`: 3 words.
    रौति,
    /// Like `अवर्`: 2 words.
    अवर,
    /// Like `अतन्`: 2 words.
    अतन,
    /// Like `आर्त`: 2 words.
    आरत,
    /// Like `अभोक्`: 2 words.
    अभोक,
    /// Like `आनट्`: 2 words.
    आनट,
    /// Like `अमृत`: 1 words.
    अमृत,
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
    pub const ALL: [Conjugation; 64] = [
        Conjugation::वहति,
        Conjugation::साधयति,
        Conjugation::लभते,
        Conjugation::धरति,
        Conjugation::वेश,
        Conjugation::रमते,
        Conjugation::जेषयति,
        Conjugation::आपत,
        Conjugation::चखान,
        Conjugation::अगरहीत,
        Conjugation::दृणाति,
        Conjugation::तनोति,
        Conjugation::आजीत,
        Conjugation::शकनोति,
        Conjugation::ररंह,
        Conjugation::भिनतति,
        Conjugation::नकति,
        Conjugation::चकरष,
        Conjugation::भाति,
        Conjugation::पिपरति,
        Conjugation::बधनाति,
        Conjugation::ददौ,
        Conjugation::ईळे,
        Conjugation::भनकति,
        Conjugation::तुषटाव,
        Conjugation::जिगाय,
        Conjugation::अगात,
        Conjugation::अनिति,
        Conjugation::असति,
        Conjugation::अवति,
        Conjugation::अनंसीत,
        Conjugation::वमति,
        Conjugation::वृणीते,
        Conjugation::रवति,
        Conjugation::जगार,
        Conjugation::उवाप,
        Conjugation::आनयति,
        Conjugation::चकार,
        Conjugation::आचरति,
        Conjugation::चषटे,
        Conjugation::ऋचछति,
        Conjugation::बाधिषयते,
        Conjugation::अधुकषत,
        Conjugation::रुरोह,
        Conjugation::विवेद,
        Conjugation::अचेत,
        Conjugation::रौति,
        Conjugation::अवर,
        Conjugation::अतन,
        Conjugation::आरत,
        Conjugation::अभोक,
        Conjugation::आनट,
        Conjugation::अमृत,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 64] = [
    // वहति: 261 words (निन्दति, खादति, इच्छति, क्षालयति, वयति, बोधति).
    conjugation!("वहति", 261, "~ति";
    stem lemma: "~- like वहति";
    stem present_ind_mid_1_sg: "~- like वहति";
    stem present: "~- like वहति";
    stem imperfect_act_1_sg: "~- like वहति";
    stem imperfect: "~- like वहति";
    ind_act_1: "~ामि", "~ावः", "~ामः|~ामसि";
    ind_act_2: "~सि", "~थः", "~थ";
    ind_act_3: _, _, "~न्ति";
    ind_mid_2: "~से", _, "~ध्वे";
    ind_mid_3: "~ते", _, _;
    imp_act_2: _, _, "~त";
    imp_mid_3: "~ताम्", _, _;
    opt_act_1: "~ेयम्", "~ेव", _;
    opt_act_3: _, _, "~ेयुः";
    subj_act_1: derived, derived, _;
    future_act_1: "~िष्यामि", _, _;
    conditional_mid_1: "अ~िष्ये", _, _;
    conditional_mid_3: "अ~िष्यत", _, _;
    perfect_act_1: "नि~", "नि~िव", _;
    perfect_act_2: "नि~िथ", _, "नि~";
    perfect_act_3: "नि~", _, _;
    perfect_mid_1: "नि~े", "नि~िवहे", _;
    aorist_act_3: _, _, "अ~िषुः";
    aorist_mid_1: _, "अ~िष्वहि", _;
    aorist_mid_2: "अ~िष्ठाः", "अ~िषाथाम्", _;
    aorist_mid_3: "अ~िष्ट", _, "अ~िषत";
    participle_pres: "~त्", _;
    participle_perf: "नि~्वांस्", _;
    ),
    // साधयति: 108 words (बर्हयति, दर्शयति, नाशयति, रन्धयति, धारयति, जासयति). Like वहति, except in 26 cells: future_act_1 pl, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("साधयति", 108, "~ति", [present_class: Class10], like वहति;
    future_act_1: _, _, "~िष्यामः|~िष्यामसि";
    perfect_act_1: "~ामास|~ांचकर", "~ामासिव|~ांचकृव", "~ामासिम|~ांचकृम";
    perfect_act_2: "~ामासिथ|~ांचकर्थ", "~ामासथुः|~ांचक्रथुः", "~ामास|~ांचक्र";
    perfect_act_3: "~ामास|~ांचकार", "~ामासतुः|~ांचक्रतुः", "~ामासुः|~ांचक्रुः";
    perfect_mid_1: "~ांचक्रे", "~ांचकृवहे", _;
    aorist_act_1: derived, derived, _;
    aorist_act_2: derived, derived, _;
    aorist_act_3: derived, _, derived;
    aorist_mid_1: _, derived, _;
    aorist_mid_2: derived, derived, derived;
    aorist_mid_3: derived, _, derived;
    participle_pres: _, "~माA|~ाA";
    participle_perf: "~ामासिवांस्|~ांचकृवांस्", _;
    ),
    // लभते: 108 words (वस्ते, सचते, रोचते, मन्यते, पर्दते, लभते). Like साधयति, except in 14 cells: ind_act_1 pl, subj_act_2 sg, subj_act_3 sg, future_act_1 pl, perfect_act_1 sg, perfect_act_1 du, ….
    conjugation!("लभते", 108, "~ते", like साधयति;
    stem present_ind_mid_1_sg: "~- like वहति";
    ind_act_1: _, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, _, _;
    future_act_1: _, _, derived;
    perfect_act_1: "~ामास", "~ामासिव", derived;
    perfect_act_2: "~ामासिथ", derived, "~ामास";
    perfect_act_3: "~ामास", derived, derived;
    participle_perf: "~ामासिवांस्", _;
    ),
    // धरति: 93 words (भरति, भुरति, दुष्यति, द्रायति, ओषति, द्रुह्यति).
    conjugation!("धरति", 93, "~~ति";
    stem imperfect: "~~- like वहति";
    ind_act_1: "~~ामि", "~~ावः", _;
    ind_act_2: "~~सि", "~~थः", "~~थ";
    ind_act_3: "~~ति", "~~तः", "~~न्ति";
    ind_mid_1: "~~े", "~~ावहे", _;
    ind_mid_2: "~~से", "~~ेथे", "~~ध्वे";
    ind_mid_3: "~~ते", _, "~~न्ते";
    imp_act_1: "~~ाणि", "~~ाव", _;
    imp_act_2: "~~", _, "~~त";
    imp_mid_3: "~~ताम्", _, _;
    opt_act_1: "~~ेयम्", "~~ेव", _;
    opt_act_3: _, _, "~~ेयुः";
    opt_mid_2: _, _, "~~ेध्वम्";
    subj_act_1: _, _, "~~ाम";
    subj_act_2: "~~ाः", _, _;
    subj_mid_1: "~~ै", "~~ावहै", "~~ामहै";
    subj_mid_2: "~~ासे", _, "~~ाध्वै";
    subj_mid_3: _, _, "~~न्त";
    imperfect_act_1: "अ~~म्", _, _;
    imperfect_act_2: _, "अ~~तम्", _;
    imperfect_mid_2: "अ~~थाः", _, "अ~~ध्वम्";
    imperfect_mid_3: "अ~~त", _, _;
    future_act_1: "~~िष्यामि", _, _;
    conditional_mid_1: "अ~~िष्ये", _, _;
    conditional_mid_3: "अ~~िष्यत", _, _;
    perfect_act_1: "च~~", "~े~िव", _;
    perfect_act_2: "च~~्थ", _, "~े~";
    perfect_act_3: "~चा~", _, _;
    perfect_mid_1: "~े~े", "~े~िवहे", _;
    aorist_act_1: "अ~ा~िषम्", "अ~ा~िष्व", _;
    aorist_act_2: "अ~ा~ीः", "अ~ा~िष्टम्", _;
    aorist_act_3: "अ~ा~ीत्", _, "अ~ा~िषुः";
    aorist_mid_1: "अ~ा~िषि", "अ~ा~िष्वहि", _;
    aorist_mid_2: "अ~ा~िष्ठाः", "अ~ा~िषाथाम्", "अ~ा~िढ्वम्";
    aorist_mid_3: "अ~ा~िष्ट", _, "अ~ा~िषत";
    infinitive: "~~ितुम्";
    gerund: "~~ित्वा";
    participle_pres: "~~त्", "~~माण";
    participle_perf: "~े~िवांस्", _;
    ),
    // वेश: 43 words (आद, आप, देग्धि, द्वेष्टि, आर, आह). Like लभते, except in 54 cells: ind_act_1 sg, ind_act_1 du, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("वेश", 43, "~", like लभते;
    stem present_ind_act_2_pl: "~- like आनाश";
    ind_act_1: derived, derived, _;
    ind_act_2: derived, derived, derived;
    ind_act_3: "~", _, derived;
    ind_mid_1: derived, derived, _;
    ind_mid_2: derived, derived, derived;
    ind_mid_3: derived, _, derived;
    imp_act_1: derived, derived, _;
    imp_act_2: derived, _, derived;
    imp_mid_3: derived, _, _;
    opt_act_1: derived, derived, _;
    opt_act_3: _, _, derived;
    opt_mid_2: _, _, derived;
    subj_act_1: _, _, derived;
    subj_mid_1: derived, derived, derived;
    subj_mid_2: derived, _, derived;
    subj_mid_3: derived, _, derived;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, _;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_1: derived, derived, _;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, _, derived;
    future_act_1: derived, _, _;
    conditional_mid_1: derived, _, _;
    conditional_mid_3: derived, _, _;
    perfect_act_1: "~", _, _;
    perfect_act_2: "~िथ", _, _;
    perfect_act_3: "~", _, _;
    perfect_mid_3: derived, _, _;
    infinitive: derived;
    gerund: derived;
    participle_pres: derived, derived;
    ),
    // रमते: 41 words (म्रियते, भाषते, रमते, परीक्षते, त्रपते, ह्वरते). Like लभते, except in 1 cell: participle_pres mid.
    conjugation!("रमते", 41, "~ते", like लभते;
    participle_pres: _, "~माण";
    ),
    // जेषयति: 36 words (करिष्यति, गमिष्यति, पास्यति, भविष्यति, तप्स्यति, भरिष्यति). Like अमत with `ि` after each pattern, except in 11 cells: future_act_1 sg, future_act_1 pl, conditional_mid_1 sg, conditional_mid_3 sg, aorist_mid_1 sg, aorist_mid_1 du, ….
    conjugation!("जेष्यति", 36, "~ति", like अमत with "ि";
    future_act_1: "~ामि", _, "~ामः|~ामसि";
    conditional_mid_1: "अ~े", _, _;
    conditional_mid_3: "अ~त", _, _;
    aorist_mid_1: derived, derived, _;
    aorist_mid_2: derived, derived, derived;
    aorist_mid_3: derived, _, derived;
    ),
    // आपत: 22 words (अगमत्, अश्रोत्, अभूत्, अवोचत्, अशुचत्, आरत्). Like अमत with `` after each pattern (`्` after त), except in 11 cells: aorist_act_1 sg, aorist_act_1 du, aorist_act_2 sg, aorist_act_2 du, aorist_act_3 sg, aorist_act_3 pl, ….
    conjugation!("आपत्", 22, "~त्", like अमत with "", "्" after "त";
    aorist_act_1: "~म्", "~ाव", _;
    aorist_act_2: "~ः", "~तम्", _;
    aorist_act_3: "~त्", _, "~न्";
    aorist_mid_1: "~े", "~ावहि", _;
    aorist_mid_2: _, "~ेथाम्", _;
    aorist_mid_3: "~त", _, "~न्त";
    ),
    // चखान: 21 words (जगाम, ससान, तताप, ततान, ददाह, जजान). Like धरति, except in 68 cells: ind_act_1 sg, ind_act_1 du, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("चखान", 21, "~ा~", like धरति;
    stem present_ind_act_2_pl: "~्~- like आनाश";
    ind_act_1: derived, derived, _;
    ind_act_2: derived, derived, derived;
    ind_act_3: "~ा~", derived, derived;
    ind_mid_1: derived, derived, _;
    ind_mid_2: derived, derived, derived;
    ind_mid_3: derived, _, derived;
    imp_act_1: derived, derived, _;
    imp_act_2: derived, _, derived;
    imp_mid_3: derived, _, _;
    opt_act_1: derived, derived, _;
    opt_act_3: _, _, derived;
    opt_mid_2: _, _, derived;
    subj_act_1: _, _, derived;
    subj_act_2: derived, _, _;
    subj_mid_1: derived, derived, derived;
    subj_mid_2: derived, _, derived;
    subj_mid_3: _, _, derived;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, _;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_1: derived, derived, _;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, _, derived;
    future_act_1: derived, _, _;
    conditional_mid_1: derived, _, _;
    conditional_mid_3: derived, _, _;
    perfect_act_1: "~~", _, _;
    perfect_act_2: "~~्थ", _, _;
    perfect_act_3: "~ा~", _, _;
    perfect_mid_3: derived, _, _;
    aorist_act_1: derived, derived, _;
    aorist_act_2: derived, derived, _;
    aorist_act_3: derived, _, derived;
    aorist_mid_1: derived, derived, _;
    aorist_mid_2: derived, derived, derived;
    aorist_mid_3: derived, _, derived;
    infinitive: derived;
    gerund: derived;
    participle_pres: derived, derived;
    ),
    // अगरहीत: 21 words (अद्राक्षीत्, अताप्सीत्, अहार्षीत्, अश्रौषीत्, अभार्षीत्, अनौत्सीत्). Like वेश with `ीत्` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अग्रहीत्", 21, "~ीत्", like वेश with "ीत्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~ीBम्|~ैBम्", "~ीB्व", _;
    aorist_act_2: "~ीः", "~ीष्टम्", _;
    aorist_act_3: "~ीत्", _, "~ीBुः";
    aorist_mid_1: "~ीBि", "~ीB्वहि", _;
    aorist_mid_2: "~ीB्ठाः", "~ीBाथाम्", "~ीढ्वम्";
    aorist_mid_3: "~ीB्ट", _, "~ीBत";
    participle_perf: derived, _;
    ),
    // दृणाति: 19 words (ददाति, दधाति, जानाति, श्रद्दधाति, गृणाति, जहाति). Like लभते, except in 40 cells: ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 du, ….
    conjugation!("दृणाति", 19, "~ाति", like लभते;
    stem present_: "~ी- like जानाति";
    stem present_ind_act_2_pl: "~ी- like जानाति";
    stem imperfect_act_1_sg: "~- like जानाति";
    ind_act_1: _, "~ीवः", "~ीमः|~ीमसि";
    ind_act_2: "~ाBि", _, _;
    ind_act_3: "~ाति", _, "~A्ति";
    ind_mid_1: _, "~ीवहे", _;
    ind_mid_2: _, "~ाथे", _;
    ind_mid_3: _, _, "~ते";
    imp_act_1: "~ानि", _, _;
    imp_act_2: "~ीहि", _, _;
    opt_act_1: "~ीयाम्", "~ीयाव", _;
    opt_act_3: _, _, "~ीयुः";
    opt_mid_2: _, _, derived;
    subj_act_1: "~ानि|~ा", _, _;
    subj_act_2: "~ाः|~ासि", _, _;
    subj_act_3: "~ात्|~ाति", _, _;
    subj_mid_2: _, _, "~ाध्वे|~ाध्वै";
    subj_mid_3: _, _, "~ान्त|~ान्तै";
    imperfect_act_2: "अ~ाः", _, _;
    imperfect_act_3: "अ~ात्", _, "अ~A्";
    imperfect_mid_1: "अ~ि", "अ~ीवहि", _;
    imperfect_mid_2: _, "अ~ाथाम्", _;
    imperfect_mid_3: _, _, "अ~त";
    future_act_1: derived, _, _;
    conditional_mid_1: derived, _, _;
    conditional_mid_3: derived, _, _;
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    infinitive: derived;
    gerund: derived;
    participle_pres: _, "~ान";
    participle_perf: derived, _;
    ),
    // तनोति: 16 words (वृणोति, तनोति, करोति, जुहोति, शृणोति, घृणोति). Like दृणाति, except in 25 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("तनोति", 16, "~ोति", like दृणाति;
    stem present_: "~ु- like जानाति";
    stem present_ind_act_2_pl: "~ु- like जानाति";
    stem present_ind_mid_1_sg: "~्व- like तनोति";
    stem present: "~व- like तनोति";
    ind_act_1: "~ोमि", "~्वः", derived;
    ind_act_2: "~ोBि", _, _;
    ind_act_3: "~ोति", _, "~्वन्ति";
    imp_act_2: "~ु", _, _;
    opt_act_1: "~ुयाम्", "~ुयाव", _;
    opt_act_3: _, _, "~ुयुः";
    subj_act_1: _, derived, _;
    subj_mid_3: derived, _, _;
    imperfect_act_1: "अ~वम्", _, _;
    imperfect_act_2: "अ~ोः", "अ~ुतम्", _;
    imperfect_act_3: "अ~ोत्", _, "अ~्वन्";
    imperfect_mid_1: "अ~्वि", "अ~्वहि", _;
    imperfect_mid_2: "अ~ुथाः", "अ~्वाथाम्", "अ~ुध्वम्";
    imperfect_mid_3: "अ~ुत", _, "अ~्वत";
    participle_pres: "~्वत्", _;
    ),
    // आजीत: 10 words (अस्वारीत्, आजीत्, अकाषीत्, आञ्चीत्, अलेखीत्, अवेदीत्). Like वेश with `ीत्` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("आजीत्", 10, "~ीत्", like वेश with "ीत्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~िBम्", "~िB्व", _;
    aorist_act_2: "~ीः", "~िष्टम्", _;
    aorist_act_3: "~ीत्", _, "~िBुः";
    aorist_mid_1: "~िBि", "~िB्वहि", _;
    aorist_mid_2: "~िB्ठाः", "~िBाथाम्", "~िढ्वम्";
    aorist_mid_3: "~िB्ट", _, "~िBत";
    participle_perf: derived, _;
    ),
    // शकनोति: 8 words (धृष्णोति, शक्नोति, साध्नोति, आप्नोति, ऋध्नोति, दभ्नोति). Like तनोति, except in 10 cells: ind_act_1 du, ind_act_1 pl, ind_act_3 pl, imp_act_2 sg, imperfect_act_3 pl, imperfect_mid_1 sg, ….
    conjugation!("शक्नोति", 8, "~ोति", [present_class: Class5], like तनोति;
    stem present_ind_mid_1_sg: "~ुव- like तनोति";
    ind_act_1: _, "~ुवः", "~ुमः|~ुमसि";
    ind_act_3: _, _, "~ुवन्ति";
    imp_act_2: "~ुहि", _, _;
    imperfect_act_3: _, _, "अ~ुवन्";
    imperfect_mid_1: "अ~ुवि", "अ~ुवहि", _;
    imperfect_mid_2: _, "अ~ुवाथाम्", _;
    imperfect_mid_3: _, _, "अ~ुवत";
    participle_pres: "~ुवत्", _;
    ),
    // ररंह: 8 words (आस, ततक्ष, ररंह, चकाङ्क्ष, जिहीड, जिहिंस). Like वेश, except in 1 cell: participle_perf act.
    conjugation!("ररंह", 8, "~", like वेश;
    participle_perf: "~्वांस्", _;
    ),
    // भिनतति: 8 words (वेत्ति, अत्ति, उनत्ति, भिनत्ति, छिनत्ति, वेवेत्ति). Like तनोति, except in 37 cells: ind_act_1 sg, ind_act_1 du, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("भिनत्ति", 8, "~त्ति", like तनोति;
    stem present_ind_act_2_pl: "~्त्- like जानाति";
    stem present: "~द- like वेत्ति";
    ind_act_1: "~द्मि", "~्द्वः", _;
    ind_act_2: "~त्Bि", "~्त्थः", _;
    ind_act_3: "~त्ति", _, "~्दन्ति";
    ind_mid_1: "~्दे", "~्द्वहे", _;
    ind_mid_2: "~्त्से", "~्दाथे", "~्द्ध्वे";
    ind_mid_3: "~्त्ते", _, "~्दते";
    imp_act_2: "~्द्धि", _, _;
    imp_mid_3: "~्त्ताम्", _, _;
    opt_act_1: "~्द्याम्", "~्द्याव", _;
    opt_act_3: _, _, "~्द्युः";
    opt_mid_2: _, _, "~्दीध्वम्";
    subj_act_1: derived, derived, _;
    subj_act_2: derived, _, _;
    subj_act_3: derived, _, _;
    imperfect_act_1: "अ~दम्", _, _;
    imperfect_act_2: "अ~त्", "अ~्त्तम्", _;
    imperfect_act_3: "अ~त्", _, "अ~्दन्";
    imperfect_mid_1: "अ~्दि", "अ~्द्वहि", _;
    imperfect_mid_2: "अ~्त्थाः", "अ~्दाथाम्", "अ~्द्ध्वम्";
    imperfect_mid_3: "अ~्त्त", _, "अ~्दत";
    participle_pres: "~्दत्", "~्दाA";
    ),
    // नकति: 8 words (रिणक्ति, वक्ति, नेनेक्ति, पृणक्ति, वृणक्ति, भिषक्ति). Like भिनतति, except in 19 cells: ind_act_1 sg, ind_act_1 du, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, imp_act_2 sg, ….
    conjugation!("नक्ति", 8, "~क्ति", like भिनतति;
    stem present_: "~- like तक्ति";
    stem present_ind_act_2_pl: "~क्- like जानाति";
    stem present_ind_mid_1_sg: "~B- like तक्ति";
    stem present: "~B- like वेत्ति";
    stem imperfect_act_1_sg: "~- like तक्ति";
    ind_act_1: "~B्मि", "~B्वः", _;
    ind_act_2: "~क्Bि", _, _;
    ind_act_3: "~क्ति", _, "~Bन्ति";
    imp_act_2: "~ग्धि", _, _;
    opt_act_1: "~B्याम्", "~B्याव", _;
    opt_act_3: _, _, "~B्युः";
    subj_act_1: derived, derived, _;
    imperfect_act_2: "अ~क्", _, _;
    imperfect_act_3: "अ~क्", _, "अ~BA्";
    imperfect_mid_1: "अ~जि", "अ~B्वहि", _;
    imperfect_mid_2: _, "अ~जाथाम्", _;
    imperfect_mid_3: _, _, "अ~जत";
    participle_pres: "~Bत्", _;
    ),
    // चकरष: 8 words (ददर्श, ववर्त, पपर्च, जगर्ध, बबर्ह, आनर्च). Like असवार with `~` after each pattern, except in 12 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, aorist_act_1 sg, ….
    conjugation!("चकर्ष", 8, "~र्~", like असवार with "~";
    perfect_act_1: "~र्~", "~ृ~िव", _;
    perfect_act_2: "~र्~िथ", _, "~ृ~";
    perfect_act_3: "~र्~", _, _;
    aorist_act_1: derived, derived, _;
    aorist_act_2: derived, derived, _;
    aorist_act_3: derived, _, derived;
    participle_perf: "~ृ~्वांस्", _;
    ),
    // भाति: 7 words (भाति, द्राति, वाति, पाति, याति, आयाति). Like दृणाति, except in 20 cells: ind_act_1 du, ind_act_1 pl, ind_act_3 pl, ind_mid_1 sg, ind_mid_1 du, ind_mid_3 pl, ….
    conjugation!("भाति", 7, "~ाति", [present_class: Class2], like दृणाति;
    stem present_: "~ा- like जानाति";
    stem present_ind_act_2_pl: "~ा- like जानाति";
    ind_act_1: _, "~ावः", "~ामः|~ामसि";
    ind_act_3: _, _, "~ान्ति";
    ind_mid_1: "~ै", "~ावहे", _;
    ind_mid_3: _, _, "~ाते";
    imp_act_2: "~ाहि", _, _;
    opt_act_1: "~ायाम्", "~ायाव", _;
    opt_act_3: _, _, "~ायुः";
    opt_mid_2: _, _, "~ेध्वम्";
    imperfect_act_2: _, "अ~ातम्", _;
    imperfect_act_3: _, _, "अ~ुः|अ~ान्";
    imperfect_mid_1: "अ~े", "अ~ावहि", _;
    imperfect_mid_2: "अ~ाथाः", _, "अ~ाध्वम्";
    imperfect_mid_3: "अ~ात", _, "अ~ात";
    participle_pres: "~ात्", _;
    ),
    // पिपरति: 7 words (पिपर्ति, जागर्ति, बिभर्ति, जिघर्ति, इयर्ति, सिसर्ति). Like तनोति, except in 30 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("पिपर्ति", 7, "~र्ति", like तनोति;
    stem present_: "~ृ- like पिपर्ति";
    stem present_ind_act_2_pl: "~ृ- like जानाति";
    stem present: "~र- like तनोति";
    ind_act_1: "~र्मि", "~ृवः", "~ृमः|~ृमसि";
    ind_act_2: "~र्Bि", _, _;
    ind_act_3: "~र्ति", _, "~्रति";
    ind_mid_1: "~्रे", "~ृवहे", _;
    ind_mid_2: _, "~्राथे", _;
    ind_mid_3: _, _, "~्रते";
    imp_act_2: "~ृहि", _, _;
    opt_act_1: "~ृयाम्", "~ृयाव", _;
    opt_act_3: _, _, "~ृयुः";
    opt_mid_2: _, _, "~्रीध्वम्";
    subj_act_1: _, derived, _;
    imperfect_act_1: "अ~रम्", _, _;
    imperfect_act_2: "अ~ः", "अ~ृतम्", _;
    imperfect_act_3: "अ~ः", _, "अ~रुः";
    imperfect_mid_1: "अ~्रि", "अ~ृवहि", _;
    imperfect_mid_2: "अ~ृथाः", "अ~्राथाम्", "अ~ृध्वम्";
    imperfect_mid_3: "अ~ृत", _, "अ~्रत";
    participle_pres: "~्रत्", "~्राA";
    ),
    // बधनाति: 6 words (गृभ्णाति, उभ्नाति, गृह्णाति, इष्णाति, बध्नाति, तभ्नाति). Like चकरष, except in 10 cells: ind_act_1 sg, ind_act_3 sg, imp_act_2 sg, subj_act_1 du, perfect_act_1 sg, perfect_act_1 du, ….
    conjugation!("बध्नाति", 6, "~्~ाति", [present_class: Class9], like चकरष;
    stem perfect_act_3_sg: "~्~- like बध्नाति";
    stem present_: "~्~ी- like पिपर्ति";
    stem present_ind_act_2_pl: "~्~ी- like जानाति";
    stem present_ind_mid_1_sg: "~्~- like बध्नाति";
    stem present: "~्~- like बध्नाति";
    stem imperfect_act_1_sg: "~्~- like जानाति";
    stem imperfect: "~्~- like बध्नाति";
    ind_act_1: "~्~ामि", _, _;
    ind_act_3: "~्~ाति", _, _;
    imp_act_2: "~ा~", _, _;
    subj_act_1: _, derived, _;
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    participle_perf: derived, _;
    ),
    // ददौ: 6 words (तस्थौ, ददौ, पपौ, दधौ, ररौ, पप्रौ). Like वेश with `` after each pattern (`ौ` after ~), except in 2 cells: perfect_act_2 sg, perfect_act_2 pl.
    conjugation!("ददौ", 6, "~ौ", like वेश with "", "ौ" after "~";
    perfect_act_2: "~ाथ|~िथ", _, "~";
    ),
    // ईळे: 5 words (ईळे, जज्ञे, शश्ये, पस्पशे, चके). Like वेश with `` after each pattern (`े` after ~), except in 6 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, participle_perf act.
    conjugation!("ईळे", 5, "~े", like वेश with "", "े" after "~";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    participle_perf: derived, _;
    ),
    // भनकति: 5 words (भनक्ति, युनक्ति, तनक्ति, अनक्ति, भुनक्ति). Like तनोति, except in 24 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("भनक्ति", 5, "~नक्ति", [present_class: Class7], like तनोति;
    stem present_: "~ङ्- like तक्ति";
    stem present_ind_act_2_pl: "~ङ्क्- like जानाति";
    stem present_ind_mid_1_sg: "~ञ्ज- like तक्ति";
    stem present: "~Aज- like तनोति";
    ind_act_1: "~Aज्मि", "~ञ्ज्वः", "~ञ्ज्मः|~ञ्ज्मसि";
    ind_act_2: "~Aक्Bि", _, _;
    ind_act_3: "~नक्ति", _, "~ञ्जन्ति";
    imp_act_2: "~ङ्ग्धि", _, _;
    opt_act_1: "~ञ्ज्याम्", "~ञ्ज्याव", _;
    opt_act_3: _, _, "~ञ्ज्युः";
    subj_act_1: _, derived, _;
    imperfect_act_1: "अ~Aजम्", _, _;
    imperfect_act_2: "अ~Aक्", "अ~ङ्क्तम्", _;
    imperfect_act_3: "अ~Aक्", _, "अ~ञ्जन्";
    imperfect_mid_1: "अ~ञ्जि", "अ~ञ्ज्वहि", _;
    imperfect_mid_2: "अ~ङ्क्थाः", "अ~ञ्जाथाम्", "अ~ङ्ग्ध्वम्";
    imperfect_mid_3: "अ~ङ्क्त", _, "अ~ञ्जत";
    participle_pres: "~ञ्जत्", _;
    ),
    // तुषटाव: 5 words (दुद्राव, शुश्राव, तुष्टाव, सुस्राव, जुहाव). Like वेश, except in 9 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, ….
    conjugation!("तुष्टाव", 5, "~ाव", like वेश;
    ind_act_3: "~ाव", _, _;
    perfect_act_1: "~व|~ाव", "~ुव", _;
    perfect_act_2: "~ोथ", _, "~ुव";
    perfect_act_3: "~ाव", _, _;
    perfect_mid_1: "~ुवे", "~ुवहे", _;
    participle_perf: "~ुवांस्", _;
    ),
    // जिगाय: 5 words (बिभाय, जिगाय, इयाय, जिघाय, निनाय). Like वेश, except in 9 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, ….
    conjugation!("जिगाय", 5, "~ाय", like वेश;
    ind_act_3: "~ाय", _, _;
    perfect_act_1: "~य|~ाय", "~्यिव", _;
    perfect_act_2: "~ेथ|~यिथ", _, "~्य";
    perfect_act_3: "~ाय", _, _;
    perfect_mid_1: "~्ये", "~्यिवहे", _;
    participle_perf: "~ीवांस्", _;
    ),
    // अगात: 5 words (अधात्, अस्थात्, अदात्, अगात्, अपात्). Like वेश with `ात्` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अगात्", 5, "~ात्", like वेश with "ात्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~ाम्", "~ाव", _;
    aorist_act_2: "~ाः", "~ातम्", _;
    aorist_act_3: "~ात्", _, "~ुः";
    aorist_mid_1: "~ि", "~िवहि", _;
    aorist_mid_2: "~िथाः", "~ाथाम्", "~िध्वम्";
    aorist_mid_3: "~ित", _, "~त";
    participle_perf: derived, _;
    ),
    // अनिति: 5 words (रोदिति, अनिति, श्वसिति, उच्छ्वसिति, वपिति). Like दृणाति, except in 26 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_mid_1 du, ….
    conjugation!("अनिति", 5, "~िति", [present_class: Class2], like दृणाति;
    stem present_: "~ि- like जानाति";
    stem present_ind_act_2_pl: "~ि- like जानाति";
    ind_act_1: "~िमि", "~िवः", "~िमः|~िमसि";
    ind_act_2: "~िBि", _, _;
    ind_act_3: "~िति", _, _;
    ind_mid_1: _, "~िवहे", _;
    imp_act_2: "~िहि", _, _;
    opt_act_1: "~्याम्", "~्याव", _;
    opt_act_3: _, _, "~्युः";
    opt_mid_2: _, _, "~ीध्वम्";
    subj_act_1: derived, _, _;
    subj_act_2: derived, _, _;
    subj_act_3: derived, _, _;
    subj_mid_2: "~से", _, "~ध्वे";
    subj_mid_3: derived, _, "~न्त";
    imperfect_act_1: "अ~म्", _, _;
    imperfect_act_2: "अ~ीः", "अ~ितम्", _;
    imperfect_act_3: "अ~ीत्", _, _;
    imperfect_mid_1: _, "अ~िवहि", _;
    imperfect_mid_2: "अ~िथाः", _, "अ~िध्वम्";
    imperfect_mid_3: "अ~ित", _, _;
    ),
    // असति: 5 words (अस्ति, शास्ति, बभस्ति, हिनस्ति, सस्ति). Like वेश with `स्ति` after each pattern, except in 28 cells: ind_act_1 sg, ind_act_1 du, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 pl, ….
    conjugation!("अस्ति", 5, "~स्ति", like वेश with "स्ति";
    ind_act_1: "~स्मि", "~B्वः", _;
    ind_act_2: "~स्Bि", "~B्थः", "~B्थ";
    ind_act_3: _, _, "~Bन्ति";
    imp_act_1: "~साAि", "~साव", _;
    imp_act_2: "~द्Bि", _, "~B्त";
    opt_act_1: "~B्याम्", "~B्याव", _;
    opt_act_3: _, _, "~B्युः";
    subj_act_1: _, _, "~साम";
    imperfect_act_1: "अ~सम्", _, _;
    imperfect_act_2: "अ~ः", "अ~B्तम्", _;
    imperfect_act_3: "अ~ः", _, "अ~BA्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    participle_pres: "~Bत्", _;
    participle_perf: derived, _;
    ),
    // अवति: 4 words (अवति, अपनह्यति, अपिनह्यति, अर्चयति). Like वहति, except in 73 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ….
    conjugation!("अवति", 4, "अ~ति", like वहति;
    ind_act_1: "अ~ामि", "अ~ावः", "अ~ामः|अ~ामसि";
    ind_act_2: "अ~सि", "अ~थः", "अ~थ";
    ind_act_3: "अ~ति", _, "अ~न्ति";
    ind_mid_1: "अ~े", "अ~ावहे", _;
    ind_mid_2: "अ~से", "अ~ेथे", "अ~ध्वे";
    ind_mid_3: "अ~ते", _, "अ~न्ते";
    imp_act_1: "अ~ाAि", "अ~ाव", _;
    imp_act_2: "अ~", _, "अ~त";
    imp_mid_3: "अ~ताम्", _, _;
    opt_act_1: "अ~ेयम्", "अ~ेव", _;
    opt_act_3: _, _, "अ~ेयुः";
    opt_mid_2: _, _, "अ~ेध्वम्";
    subj_act_1: _, _, "अ~ाम";
    subj_act_2: "अ~ाः|अ~ासि", _, _;
    subj_act_3: "अ~ात्|अ~ाति", _, _;
    subj_mid_1: "अ~ै", "अ~ावहै", "अ~ामहै";
    subj_mid_2: "अ~ासे|अ~ासै", _, "अ~ाध्वै";
    subj_mid_3: "अ~ाते|अ~ातै", _, "अ~न्त|अ~ान्तै";
    imperfect_act_1: "आ~म्", _, _;
    imperfect_act_2: "आ~ः", "आ~तम्", _;
    imperfect_act_3: "आ~त्", _, "आ~न्";
    imperfect_mid_1: "आ~े", "आ~ावहि", _;
    imperfect_mid_2: "आ~थाः", "आ~ेथाम्", "आ~ध्वम्";
    imperfect_mid_3: "आ~त", _, "आ~न्त";
    future_act_1: "अ~िष्यामि", _, _;
    conditional_mid_1: "आ~िष्ये", _, _;
    conditional_mid_3: "आ~िष्यत", _, _;
    perfect_act_1: "आ~", "आ~िव", _;
    perfect_act_2: "आ~िथ", _, "आ~";
    perfect_act_3: "आ~", _, _;
    perfect_mid_1: "आ~े", "आ~िवहे", _;
    aorist_act_1: "आ~िषम्", "आ~िष्व", _;
    aorist_act_2: "आ~ीः", "आ~िष्टम्", _;
    aorist_act_3: "आ~ीत्", _, "आ~िषुः";
    aorist_mid_1: _, "आ~िष्वहि", _;
    aorist_mid_2: "आ~िष्ठाः", "आ~िषाथाम्", "आ~िढ्वम्";
    aorist_mid_3: "आ~िष्ट", _, "आ~िषत";
    infinitive: "अ~ितुम्";
    gerund: "अ~ित्वा";
    participle_pres: "अ~त्", "अ~माA";
    participle_perf: "आ~िवांस्", _;
    ),
    // अनंसीत: 4 words (अहासीत्, अतांसीत्, अयासीत्, अनंसीत्). Like वेश with `सीत्` after each pattern, except in 14 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अनंसीत्", 4, "~सीत्", like वेश with "सीत्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~सम्", "~स्व", _;
    aorist_act_2: "~सीः", "~स्तम्", _;
    aorist_act_3: "~सीत्", _, "~सुः";
    participle_perf: derived, _;
    ),
    // वमति: 4 words (वमति, चमति, नमति, गमति). Like वेश, except in 36 cells: ind_act_1 sg, ind_act_1 du, ind_act_2 sg, ind_act_2 pl, ind_act_3 pl, imp_act_1 sg, ….
    conjugation!("वमति", 4, "~मति", [present_class: Class1], like वेश;
    stem present_: "~म- like जानाति";
    stem lemma: "~म- like वहति";
    stem present_ind_mid_1_sg: "~म- like वहति";
    stem imperfect_act_1_sg: "~म- like वहति";
    stem imperfect: "~म- like वमति";
    ind_act_1: "~मामि", "~मावः", _;
    ind_act_2: "~मBि", _, "~मथ";
    ind_act_3: _, _, "~मन्ति";
    imp_act_1: "~माAि", "~माव", _;
    imp_act_2: _, _, "~मत";
    opt_act_1: "~मेयम्", "~मेव", _;
    opt_act_3: _, _, "~मेयुः";
    subj_act_1: _, _, "~माम";
    subj_mid_1: "~मै", "~मावहै", "~मामहै";
    subj_mid_2: "~मासे", _, "~माध्वै";
    subj_mid_3: _, _, "~मन्त";
    future_act_1: "~मिष्यामि", _, _;
    conditional_mid_1: "अ~मिष्ये", _, _;
    conditional_mid_3: "अ~मिष्यत", _, _;
    perfect_act_1: "~चम", "~ेमिव", _;
    perfect_act_2: "~चA्थ", _, "~ेम";
    perfect_act_3: "~चाम", _, _;
    perfect_mid_1: "~ेमे", "~ेमिवहे", _;
    aorist_act_3: _, _, "अ~मिषुः";
    aorist_mid_1: _, "अ~मिष्वहि", _;
    aorist_mid_2: "अ~मिष्ठाः", "अ~मिषाथाम्", _;
    aorist_mid_3: "अ~मिष्ट", _, "अ~मिषत";
    participle_pres: "~मत्", _;
    participle_perf: "~ेमिवांस्", _;
    ),
    // वृणीते: 4 words (मिमीते, जानीते, जिहीते, वृणीते). Like वेश with `ीते` after each pattern, except in 32 cells: ind_mid_1 sg, ind_mid_1 du, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ind_mid_3 sg, ….
    conjugation!("वृणीते", 4, "~ीते", like वेश with "ीते";
    ind_mid_1: "~े", "~ीवहे", _;
    ind_mid_2: "~ीBे", "~ाथे", "~ीध्वे";
    ind_mid_3: "~ीते", _, "~ते";
    imp_mid_1: "~ै", _, _;
    imp_mid_3: "~ीताम्", _, _;
    opt_mid_1: "~ीय", _, _;
    subj_mid_1: _, "~ावहै", "~ामहै";
    subj_mid_2: "~ासे|~ासै", _, "~ाध्वे|~ाध्वै";
    subj_mid_3: "~ाते|~ातै", _, "~ान्त|~ान्तै";
    imperfect_mid_1: "अ~ि", "अ~ीवहि", _;
    imperfect_mid_2: "अ~ीथाः", "अ~ाथाम्", "अ~ीध्वम्";
    imperfect_mid_3: "अ~ीत", _, "अ~त";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    participle_pres: _, "~ान";
    participle_perf: derived, _;
    ),
    // रवति: 4 words (द्रवति, चर्वति, तूर्वति, रवति). Like तनोति, except in 53 cells: ind_act_1 sg, ind_act_1 du, ind_act_2 sg, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("रवति", 4, "~वति", [present_class: Class1], like तनोति;
    stem present_: "~व- like जानाति";
    stem imperfect: "~व- like वहति";
    ind_act_1: "~वामि", "~वावः", _;
    ind_act_2: "~वBि", _, "~वथ";
    ind_act_3: "~वति", _, "~वन्ति";
    ind_mid_1: "~वे", "~वावहे", _;
    ind_mid_2: _, "~वेथे", _;
    ind_mid_3: _, _, "~वन्ते";
    imp_act_2: "~व", _, "~वत";
    opt_act_1: "~वेयम्", "~वेव", _;
    opt_act_3: _, _, "~वेयुः";
    opt_mid_2: _, _, "~वेध्वम्";
    subj_act_1: derived, _, _;
    subj_act_2: derived, _, _;
    subj_act_3: derived, _, _;
    subj_mid_2: "~वासे", _, "~वाध्वै";
    imperfect_act_2: _, "अ~वतम्", _;
    imperfect_mid_2: "अ~वथाः", _, "अ~वध्वम्";
    imperfect_mid_3: "अ~वत", _, _;
    future_act_1: "~ोष्यामि", _, _;
    conditional_mid_1: "अ~ोष्ये", _, _;
    conditional_mid_3: "अ~ोष्यत", _, _;
    perfect_act_1: "दु~व", "दु~ुव", _;
    perfect_act_2: "दु~ोथ", _, "दु~ुव";
    perfect_act_3: "दु~ाव", _, _;
    perfect_mid_1: "दु~ुवे", "दु~ुवहे", _;
    aorist_act_1: "अ~ुद्रुवम्", "अ~ुद्रुवाव", _;
    aorist_act_2: "अ~ुद्रुवः", "अ~ुद्रुवतम्", _;
    aorist_act_3: "अ~ुद्रुवत्", _, "अ~ुद्रुवन्";
    aorist_mid_1: "अ~ाविषि", "अ~ाविष्वहि", _;
    aorist_mid_2: "अ~ाविष्ठाः", "अ~ाविषाथाम्", "अ~ाविढ्वम्";
    aorist_mid_3: "अ~ाविष्ट", _, "अ~ाविषत";
    infinitive: "~ोतुम्";
    gerund: "~ुत्वा";
    participle_pres: "~वत्", "~वमाA";
    participle_perf: "दु~ुवांस्", _;
    ),
    // जगार: 4 words (जहार, दधार, जगार, ममार). Like वेश, except in 9 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, ….
    conjugation!("जगार", 4, "~ार", like वेश;
    ind_act_3: "~ार", _, _;
    perfect_act_1: "~र|~ार", "~्रिव", _;
    perfect_act_2: "~र्थ|~रिथ", _, "~्र";
    perfect_act_3: "~ार", _, _;
    perfect_mid_1: "~्रे", "~्रिवहे", _;
    participle_perf: "~ृवांस्", _;
    ),
    // उवाप: 4 words (उवाच, उवाह, उवाप, उवाद). Like वेश, except in 9 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, ….
    conjugation!("उवाप", 4, "उवा~", like वेश;
    ind_act_3: "उवा~", _, _;
    perfect_act_1: "उव~", "ऊ~िव", _;
    perfect_act_2: "उव~्थ", _, "ऊ~";
    perfect_act_3: "उवा~", _, _;
    perfect_mid_1: "ऊ~े", "ऊ~िवहे", _;
    participle_perf: "ऊ~िवांस्", _;
    ),
    // आनयति: 4 words (आनयति, आगच्छति, आक्रोशति, आमयति). Like लभते, except in 13 cells: ind_act_1 pl, subj_act_1 sg, subj_act_1 du, conditional_mid_1 sg, conditional_mid_3 sg, perfect_act_1 sg, ….
    conjugation!("आनयति", 4, "~ति", like लभते;
    stem present: "~- like वहति";
    stem imperfect_act_1_sg: "~- like आनयति";
    stem imperfect: "~- like आनयति";
    ind_act_1: _, _, "~ामः|~ामसि";
    subj_act_1: derived, derived, _;
    conditional_mid_1: derived, _, _;
    conditional_mid_3: derived, _, _;
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    participle_perf: derived, _;
    ),
    // चकार: 3 words (चकार, बभार, जभार). Like जगार, except in 3 cells: perfect_act_1 du, perfect_act_2 sg, perfect_mid_1 du.
    conjugation!("चकार", 3, "~ार", like जगार;
    perfect_act_1: _, "~ृव", _;
    perfect_act_2: "~र्थ", _, _;
    perfect_mid_1: _, "~ृवहे", _;
    ),
    // आचरति: 3 words (आचरति, पक्षति, यक्षति). Like आनयति, except in 3 cells: future_act_1 sg, infinitive, gerund.
    conjugation!("आचरति", 3, "~ति", like आनयति;
    future_act_1: derived, _, _;
    infinitive: derived;
    gerund: derived;
    ),
    // चषटे: 3 words (ईष्टे, चष्टे, प्रचष्टे). Like वेश with `ष~टे` after each pattern, except in 32 cells: ind_mid_1 sg, ind_mid_1 du, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ind_mid_3 sg, ….
    conjugation!("चष्टे", 3, "~ष~टे", [present_class: Class2], like वेश with "ष~टे";
    ind_mid_1: "~क~षे", "~क~ष्वहे", _;
    ind_mid_2: "~क~षे", "~क~षाथे", "~ड~ढ्वे";
    ind_mid_3: "~ष~टे", _, "~क~षते";
    imp_mid_1: "~क~षै", _, _;
    imp_mid_3: "~ष~टाम्", _, _;
    opt_mid_2: _, _, "~क~षीध्वम्";
    subj_mid_1: _, "~क~षावहै", "~क~षामहै";
    subj_mid_2: "~क~षसे|~क~षासै", _, "~क~षध्वे|~क~षाध्वै";
    subj_mid_3: "~क~षते|~क~षातै", _, "~क~षन्त|~क~षान्तै";
    imperfect_mid_1: "अ~क~षि", "अ~क~ष्वहि", _;
    imperfect_mid_2: "अ~ष~ठाः", "अ~क~षाथाम्", "अ~ड~ढ्वम्";
    imperfect_mid_3: "अ~ष~ट", _, "अ~क~षत";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    participle_pres: _, "~क~षाण";
    participle_perf: derived, _;
    ),
    // ऋचछति: 3 words (ऋघायति, ऋच्छति, ऋध्यति). Like भिनतति, except in 48 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ….
    conjugation!("ऋच्छति", 3, "ऋ~ति", like भिनतति;
    ind_act_1: "ऋ~ामि", "ऋ~ावः", "ऋ~ामः|ऋ~ामसि";
    ind_act_2: "ऋ~सि", "ऋ~थः", "ऋ~थ";
    ind_act_3: "ऋ~ति", _, "ऋ~न्ति";
    ind_mid_1: "ऋ~े", "ऋ~ावहे", _;
    ind_mid_2: "ऋ~से", "ऋ~ेथे", "ऋ~ध्वे";
    ind_mid_3: "ऋ~ते", _, "ऋ~न्ते";
    imp_act_1: "ऋ~ानि", "ऋ~ाव", _;
    imp_act_2: "ऋ~", _, "ऋ~त";
    imp_mid_3: "ऋ~ताम्", _, _;
    opt_act_1: "ऋ~ेयम्", "ऋ~ेव", _;
    opt_act_3: _, _, "ऋ~ेयुः";
    opt_mid_2: _, _, "ऋ~ेध्वम्";
    subj_act_1: _, _, "ऋ~ाम";
    subj_act_2: "ऋ~ाः|ऋ~ासि", _, _;
    subj_act_3: "ऋ~ात्|ऋ~ाति", _, _;
    subj_mid_1: "ऋ~ै", "ऋ~ावहै", "ऋ~ामहै";
    subj_mid_2: "ऋ~ासे|ऋ~ासै", _, "ऋ~ाध्वै";
    subj_mid_3: "ऋ~ाते|ऋ~ातै", _, "ऋ~न्त|ऋ~ान्तै";
    imperfect_act_1: "आर्~म्", _, _;
    imperfect_act_2: "आर्~ः", "आर्~तम्", _;
    imperfect_act_3: "आर्~त्", _, "आर्~न्";
    imperfect_mid_1: "आर्~े", "आर्~ावहि", _;
    imperfect_mid_2: "आर्~थाः", "आर्~ेथाम्", "आर्~ध्वम्";
    imperfect_mid_3: "आर्~त", _, "आर्~न्त";
    participle_pres: "ऋ~त्", "ऋ~मान";
    ),
    // बाधिषयते: 3 words (बाधिष्यते, श्यास्यते, रेक्ष्यते). Like अमत with `े` after each pattern, except in 10 cells: future_mid_1 sg, conditional_mid_1 sg, conditional_mid_3 sg, aorist_mid_1 sg, aorist_mid_1 du, aorist_mid_2 sg, ….
    conjugation!("बाधिष्यते", 3, "~ते", like अमत with "े";
    future_mid_1: "~े", _, _;
    conditional_mid_1: "अ~े", _, _;
    conditional_mid_3: "अ~त", _, _;
    aorist_mid_1: derived, derived, _;
    aorist_mid_2: derived, derived, derived;
    aorist_mid_3: derived, _, derived;
    ),
    // अधुकषत: 3 words (अधुक्षत्, अद्रुक्षत्, अरुक्षत्). Like अमत with `` after each pattern (`्` after त), except in 11 cells: aorist_act_1 sg, aorist_act_1 du, aorist_act_2 sg, aorist_act_2 du, aorist_act_3 sg, aorist_act_3 pl, ….
    conjugation!("अधुक्षत्", 3, "~त्", like अमत with "", "्" after "त";
    aorist_act_1: "~म्", "~ाव", _;
    aorist_act_2: "~ः", "~तम्", _;
    aorist_act_3: "~त्", _, "~न्";
    aorist_mid_1: "~ि", "~ावहि", _;
    aorist_mid_2: _, "~ाथाम्", _;
    aorist_mid_3: "~त", _, "~न्त";
    ),
    // रुरोह: 3 words (दुदोह, दुद्रोह, रुरोह). Like वेश, except in 14 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 pl, ….
    conjugation!("रुरोह", 3, "~ोह", like वेश;
    ind_act_3: "~ोह", _, _;
    perfect_act_1: "~ोह", "~ुहिव|~ुह्व", "~ुहिम|~ुह्म";
    perfect_act_2: "~ोहिथ", _, "~ुह";
    perfect_act_3: "~ोह", _, _;
    perfect_mid_1: "~ुहे", "~ुहिवहे|~ुह्वहे", "~ुहिमहे|~ुह्महे";
    perfect_mid_2: "~ुहिषे|~ुक्षे", _, "~ुहिध्वे|~ूढ्वे";
    perfect_mid_3: _, _, "~ुहिरे|~ुह्रे";
    participle_perf: "~ुह्वांस्", _;
    ),
    // विवेद: 3 words (वेद, विवेद, चिच्छेद). Like वेश, except in 5 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_2 sg, perfect_act_3 sg, perfect_mid_3 sg.
    conjugation!("विवेद", 3, "~ेद", like वेश;
    stem present_ind_act_2_pl: "~ि- like वेद";
    ind_act_3: "~ेद", _, _;
    perfect_act_1: "~ेद", _, _;
    perfect_act_2: "~ेदिथ", _, _;
    perfect_act_3: "~ेद", _, _;
    perfect_mid_3: derived, _, _;
    ),
    // अचेत: 3 words (अश्रेत्, अचेत्, अहेत्). Like वेश with `ेत्` after each pattern, except in 14 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अचेत्", 3, "~ेत्", like वेश with "ेत्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~यम्", "~ेव", _;
    aorist_act_2: "~ेः", "~ेतम्", _;
    aorist_act_3: "~ेत्", _, "~्यA्";
    participle_perf: derived, _;
    ),
    // रौति: 3 words (नौति, रौति, तौति). Like शकनोति, except in 5 cells: ind_act_1 sg, ind_act_2 sg, ind_act_3 sg, imperfect_act_2 sg, imperfect_act_3 sg.
    conjugation!("रौति", 3, "~ौति", [present_class: Class2], like शकनोति;
    ind_act_1: "~ौमि", _, _;
    ind_act_2: "~ौषि", _, _;
    ind_act_3: "~ौति", _, _;
    imperfect_act_2: "अ~ौः", _, _;
    imperfect_act_3: "अ~ौत्", _, _;
    ),
    // अवर: 2 words (अकर्, अवर्). Like आपत, except in 12 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_2 sg, aorist_act_2 du, aorist_act_3 sg, ….
    conjugation!("अवर्", 2, "अ~र्", like आपत;
    stem perfect_act_3_sg: "~्रन्- like अकर्";
    stem imperfect_act_1_sg: "~ृध्वम्- like अकर्";
    ind_act_3: "अ~र्", _, _;
    aorist_act_1: "अ~रम्", "अ~र्व", _;
    aorist_act_2: "अ~र्", "अ~र्तम्", _;
    aorist_act_3: "अ~र्", _, _;
    aorist_mid_1: "अ~्रि", "अ~ृवहि", _;
    aorist_mid_2: "अ~ृथाः", "अ~्राथाम्", _;
    aorist_mid_3: "अ~ृत", _, "अ~्रत";
    ),
    // अतन: 2 words (अगन्, अतन्). Like वेश with `न्` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अतन्", 2, "~न्", like वेश with "न्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~Aम्", "~न्व", _;
    aorist_act_2: "~न्", "~न्तम्", _;
    aorist_act_3: "~न्", _, "~्नन्";
    aorist_mid_1: "~्Aि", "~न्वहि", _;
    aorist_mid_2: "~थाः", "~्Aाथाम्", "~ध्वम्";
    aorist_mid_3: "~त", _, "~्Aत";
    participle_perf: derived, _;
    ),
    // आरत: 2 words (अजन्त, आर्त). Like वेश with `्त` after each pattern, except in 15 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("आर्त", 2, "~्त", like वेश with "्त";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_mid_1: "~ि", "~्वहि", _;
    aorist_mid_2: "~्थाः", "~ाथाम्", "~्ध्वम्";
    aorist_mid_3: "~्त", _, "~त";
    participle_perf: derived, _;
    ),
    // अभोक: 2 words (अयोक्, अभोक्). Like वेश with `ोक्` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अभोक्", 2, "~ोक्", like वेश with "ोक्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~ोजम्", "~ोज्व", _;
    aorist_act_2: "~ोक्", "~ोक्तम्", _;
    aorist_act_3: "~ोक्", _, "~ुजA्";
    aorist_mid_1: "~ुजि", "~ुज्वहि", _;
    aorist_mid_2: "~ुक्थाः", "~ुजाथाम्", "~ुग्ध्वम्";
    aorist_mid_3: "~ुक्त", _, "~ुजत";
    participle_perf: derived, _;
    ),
    // आनट: 2 words (आनट्, अक्रन्). Like वेश with `~` after each pattern, except in 14 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("आनट्", 2, "~~", like वेश with "~";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~~दम्", "~~द्व", _;
    aorist_act_2: "~~", "~~त्तम्", _;
    aorist_act_3: "~~", _, "~द~";
    participle_perf: derived, _;
    ),
    // अमृत: 1 words (अमृत). Like वेश with `ृत` after each pattern, except in 15 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अमृत", 1, "~ृत", like वेश with "ृत";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_mid_1: "~्रि", "~ृवहि", _;
    aorist_mid_2: "~ृथाः", "~्राथाम्", "~ृध्वम्";
    aorist_mid_3: "~ृत", _, "~्रत";
    participle_perf: derived, _;
    ),
    // अलेकिषयत: 1 words (अलेकिष्यत्). Like अमत with `्` after each pattern, except in 8 cells: conditional_act_1 sg, aorist_mid_1 sg, aorist_mid_1 du, aorist_mid_2 sg, aorist_mid_2 du, aorist_mid_2 pl, ….
    conjugation!("अलेकिष्यत्", 1, "~त्", like अमत with "्";
    conditional_act_1: "~म्", _, _;
    aorist_mid_1: derived, derived, _;
    aorist_mid_2: derived, derived, derived;
    aorist_mid_3: derived, _, derived;
    ),
    // अपराक: 1 words (अप्राक्). Like वेश with `~राक्` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अप्राक्", 1, "~~राक्", like वेश with "~राक्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~~राक्षम्", "~~राक्ष्व", _;
    aorist_act_2: "~~राक्", "~~राक्तम्", _;
    aorist_act_3: "~~राक्", _, "~~राक्षुः";
    aorist_mid_1: "~ृक~षि", "~ृक~ष्वहि", _;
    aorist_mid_2: "~ृक~थाः", "~ृक~षाथाम्", "~ृग~ध्वम्";
    aorist_mid_3: "~ृक~त", _, "~ृक~षत";
    participle_perf: derived, _;
    ),
    // अमौक: 1 words (अमौक्). Like वेश with `ौक~` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अमौक्", 1, "~ौक~", like वेश with "ौक~";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~ौक~षम्", "~ौक~ष्व", _;
    aorist_act_2: "~ौक~", "~ौक~तम्", _;
    aorist_act_3: "~ौक~", _, "~ौक~षुः";
    aorist_mid_1: "~ुक~षि", "~ुक~ष्वहि", _;
    aorist_mid_2: "~ुक~थाः", "~ुक~षाथाम्", "~ुग~ध्वम्";
    aorist_mid_3: "~ुक~त", _, "~ुक~षत";
    participle_perf: derived, _;
    ),
    // अमोक: 1 words (अमोक्). Like वेश with `ोक्` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अमोक्", 1, "~ोक्", like वेश with "ोक्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~ोचम्", "~ोच्व", _;
    aorist_act_2: "~ोक्", "~ोक्तम्", _;
    aorist_act_3: "~ोक्", _, "~ुचA्";
    aorist_mid_1: "~ुचि", "~ुच्वहि", _;
    aorist_mid_2: "~ुक्थाः", "~ुचाथाम्", "~ुग्ध्वम्";
    aorist_mid_3: "~ुक्त", _, "~ुचत";
    participle_perf: derived, _;
    ),
    // अमत: 1 words (अमत). Like वेश with `त` after each pattern, except in 15 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अमत", 1, "~त", like वेश with "त";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_mid_1: "~्नि", "~न्वहि", _;
    aorist_mid_2: "~थाः", "~्नाथाम्", "~ध्वम्";
    aorist_mid_3: "~त", _, "~्नत";
    participle_perf: derived, _;
    ),
    // अवरक: 1 words (अवर्क्). Like असवार with `क्` after each pattern, except in 13 cells: aorist_act_1 sg, aorist_act_1 du, aorist_act_2 sg, aorist_act_2 du, aorist_act_3 sg, aorist_act_3 pl, ….
    conjugation!("अवर्क्", 1, "~र्क्", like असवार with "क्";
    aorist_act_1: "~र्जम्", "~र्ज्व", _;
    aorist_act_2: "~र्क्", "~र्क्तम्", _;
    aorist_act_3: "~र्क्|~ृक्", _, "~ृजन्";
    aorist_mid_1: "~ृजि", "~ृज्वहि", _;
    aorist_mid_2: "~ृक्थाः", "~ृजाथाम्", "~ृग्ध्वम्";
    aorist_mid_3: "~ृक्त", _, "~ृजत";
    ),
    // असवार: 1 words (अस्वार्). Like वेश with `र्` after each pattern, except in 14 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अस्वार्", 1, "~र्", like वेश with "र्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~र्षम्", "~र्ष्व", _;
    aorist_act_2: "~ः", "~र्ष्टम्", _;
    aorist_act_3: "~ः", _, "~र्षुः";
    participle_perf: derived, _;
    ),
    // अदरक: 1 words (अदर्क्). Like वेश with `र~क्` after each pattern, except in 15 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अदर्क्", 1, "~र~क्", like वेश with "र~क्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~र~शम्", "~र~श्व", "~र~श्म|~ृश~म";
    aorist_act_2: "~र~क्", "~र~ष्टम्", _;
    aorist_act_3: "~र~क्", _, "~ृशन~|~र~शुः";
    participle_perf: derived, _;
    ),
    // अजनिषट: 1 words (अजनिष्ट). Like वेश with `ष्ट` after each pattern, except in 15 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अजनिष्ट", 1, "~ष्ट", like वेश with "ष्ट";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_mid_1: "~Bि", "~B्वहि", _;
    aorist_mid_2: "~B्ठाः", "~Bाथाम्", "~ढ्वम्";
    aorist_mid_3: "~ष्ट", _, "~Bत";
    participle_perf: derived, _;
    ),
    // अपतत: 1 words (अपत्त). Like वेश with `त्त` after each pattern, except in 17 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अपत्त", 1, "~त्त", like वेश with "त्त";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_mid_1: "~दि|~त्सि", "~द्वहि|~त्स्वहि", "~द्महि|~त्स्महि";
    aorist_mid_2: "~त्थाः", "~दाथाम्|~त्साथाम्", "~द्ध्वम्";
    aorist_mid_3: "~त्त", "~दाताम्|~त्साताम्", "~दत|~त्सत";
    participle_perf: derived, _;
    ),
    // अजुषत: 1 words (अजुषत्). Like वेश with `षत्` after each pattern, except in 21 cells: perfect_act_1 sg, perfect_act_1 du, perfect_act_2 sg, perfect_act_2 pl, perfect_act_3 sg, perfect_mid_1 sg, ….
    conjugation!("अजुषत्", 1, "~षत्", like वेश with "षत्";
    perfect_act_1: derived, derived, _;
    perfect_act_2: derived, _, derived;
    perfect_act_3: derived, _, _;
    perfect_mid_1: derived, derived, _;
    aorist_act_1: "~सम्", "~साव", _;
    aorist_act_2: "~सः", "~सतम्", _;
    aorist_act_3: "~सत्", _, "~सन्";
    aorist_mid_1: "~से", "~सावहि", _;
    aorist_mid_2: "~सथाः", "~सेथाम्", "~सध्वम्";
    aorist_mid_3: "~सत", _, "~सन्त";
    participle_perf: derived, _;
    ),
];
