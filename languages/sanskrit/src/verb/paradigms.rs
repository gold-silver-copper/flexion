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
/// they share them), the conjugation it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the conjugation's it is like,
/// or, in a conjugation like no other, derived):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
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

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `वहति`: 265 words.
    वहति,
    /// Like `लभते`: 110 words.
    लभते,
    /// Like `अमत`: 99 words.
    अमत,
    /// Like `धरति`: 95 words.
    धरति,
    /// Like `साधयति`: 86 words.
    साधयति,
    /// Like `आपत्`: 46 words.
    आपत,
    /// Like `रमते`: 42 words.
    रमते,
    /// Like `एष्यति`: 36 words.
    एषयति,
    /// Like `ईरयति`: 27 words.
    ईरयति,
    /// Like `खादति`: 26 words.
    खादति,
    /// Like `ददाति`: 19 words.
    ददाति,
    /// Like `तनोति`: 16 words.
    तनोति,
    /// Like `ईळे`: 10 words.
    ईळे,
    /// Like `आजीत्`: 10 words.
    आजीत,
    /// Like `आप्नोति`: 8 words.
    आपनोति,
    /// Like `ररंह`: 8 words.
    ररंह,
    /// Like `भाति`: 7 words.
    भाति,
    /// Like `पिपर्ति`: 7 words.
    पिपरति,
    /// Like `बध्नाति`: 6 words.
    बधनाति,
    /// Like `ददौ`: 6 words.
    ददौ,
    /// Like `भनक्ति`: 5 words.
    भनकति,
    /// Like `अगात्`: 5 words.
    अगात,
    /// Like `जुहाव`: 5 words.
    जुहाव,
    /// Like `इयाय`: 5 words.
    इयाय,
    /// Like `चचार`: 5 words.
    चचार,
    /// Like `वमति`: 4 words.
    वमति,
    /// Like `वृणीते`: 4 words.
    वृणीते,
    /// Like `रवति`: 4 words.
    रवति,
    /// Like `आनयति`: 4 words.
    आनयति,
    /// Like `चकार`: 3 words.
    चकार,
    /// Like `आचरति`: 3 words.
    आचरति,
    /// Like `अकार्षीत्`: 3 words.
    अकारषीत,
    /// Like `अधुक्षत्`: 3 words.
    अधुकषत,
    /// Like `अचैषीत्`: 3 words.
    अचैषीत,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 34] = [
        Conjugation::वहति,
        Conjugation::लभते,
        Conjugation::अमत,
        Conjugation::धरति,
        Conjugation::साधयति,
        Conjugation::आपत,
        Conjugation::रमते,
        Conjugation::एषयति,
        Conjugation::ईरयति,
        Conjugation::खादति,
        Conjugation::ददाति,
        Conjugation::तनोति,
        Conjugation::ईळे,
        Conjugation::आजीत,
        Conjugation::आपनोति,
        Conjugation::ररंह,
        Conjugation::भाति,
        Conjugation::पिपरति,
        Conjugation::बधनाति,
        Conjugation::ददौ,
        Conjugation::भनकति,
        Conjugation::अगात,
        Conjugation::जुहाव,
        Conjugation::इयाय,
        Conjugation::चचार,
        Conjugation::वमति,
        Conjugation::वृणीते,
        Conjugation::रवति,
        Conjugation::आनयति,
        Conjugation::चकार,
        Conjugation::आचरति,
        Conjugation::अकारषीत,
        Conjugation::अधुकषत,
        Conjugation::अचैषीत,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 34] = [
    // वहति: 265 words (निन्दति, वयति, बोधति, भवति, यभति, दहति).
    conjugation!("वहति", 265, "~ति";
    ind_act_1: "~ामि", "~ावः", "~ामः|~ामसि";
    ind_act_2: "~सि", "~थः", "~थ";
    ind_act_3: "~ति", "~तः", "~न्ति";
    ind_mid_1: "~े", "~ावहे", "~ामहे";
    ind_mid_2: "~से", "~ेथे", "~ध्वे";
    ind_mid_3: "~ते", "~ेते", "~न्ते";
    imp_act_1: "~ानि", "~ाव", "~ाम";
    imp_act_2: "~", "~तम्", "~त";
    imp_act_3: "~तु", "~ताम्", "~न्तु";
    imp_mid_1: "~ै", "~ावहै", "~ामहै";
    imp_mid_2: "~स्व", "~ेथाम्", "~ध्वम्";
    imp_mid_3: "~ताम्", "~ेताम्", "~न्ताम्";
    opt_act_1: "~ेयम्", "~ेव", "~ेम";
    opt_act_2: "~ेः", "~ेतम्", "~ेत";
    opt_act_3: "~ेत्", "~ेताम्", "~ेयुः";
    opt_mid_1: "~ेय", "~ेवहि", "~ेमहि";
    opt_mid_2: "~ेथाः", "~ेयाथाम्", "~ेध्वम्";
    opt_mid_3: "~ेत", "~ेयाताम्", "~ेरन्";
    subj_act_2: "~ाः|~ासि", "~ाथः", "~ाथ";
    subj_act_3: "~ात्|~ाति", "~ातः", "~ान्";
    subj_mid_2: "~ासे|~ासै", "~ैथे", "~ाध्वै";
    subj_mid_3: "~ाते|~ातै", "~ैते", "~न्त|~ान्तै";
    imperfect_act_1: "अ~म्", "अ~ाव", "अ~ाम";
    imperfect_act_2: "अ~ः", "अ~तम्", _;
    imperfect_act_3: "अ~त्", "अ~ताम्", "अ~न्";
    imperfect_mid_1: "अ~े", "अ~ावहि", "अ~ामहि";
    imperfect_mid_2: "अ~थाः", "अ~ेथाम्", "अ~ध्वम्";
    imperfect_mid_3: "अ~त", "अ~ेताम्", "अ~न्त";
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
    perfect_act_3: "नि~", "नि~तुः", "नि~ुः";
    perfect_mid_1: "नि~े", "नि~िवहे", "नि~िमहे";
    perfect_mid_2: "नि~िषे", "नि~ाथे", "नि~िध्वे";
    perfect_mid_3: _, "नि~ाते", "नि~िरे";
    aorist_act_1: "अ~िषम्", "अ~िष्व", "अ~िष्म";
    aorist_act_2: "अ~ीः", "अ~िष्टम्", "अ~िष्ट";
    aorist_act_3: "अ~ीत्", "अ~िष्टाम्", "अ~िषुः";
    aorist_mid_1: "अ~िषि", "अ~िष्वहि", "अ~िष्महि";
    aorist_mid_2: "अ~िष्ठाः", "अ~िषाथाम्", "अ~िढ्वम्";
    aorist_mid_3: "अ~िष्ट", "अ~िषाताम्", "अ~िषत";
    infinitive: "~ितुम्";
    gerund: "~ित्वा";
    participle_pres: "~त्", "~मान";
    participle_fut: "~िष्यत्", "~िष्यमाण";
    participle_perf: "नि~्वांस्", "नि~ान";
    ),
    // लभते: 110 words (वस्ते, सचते, रोचते, मन्यते, पर्दते, लभते). Like वहति, except in 40 cells: ind_act_1 pl, subj_act_2 sg, subj_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, ….
    conjugation!("लभते", 110, "~ते", like वहति;
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
    aorist_mid_3: derived, derived, derived;
    participle_perf: "~ामासिवांस्", "~ांचक्राण";
    ),
    // अमत: 99 words (आद, आप, अमृत, वेद, देग्धि, द्वेष्टि).
    conjugation!("अमत", 99, "~";
    ind_act_3: "~", _, _;
    perfect_act_1: "~", "~िव", "~िम";
    perfect_act_2: "~िथ", "~थुः", "~";
    perfect_act_3: "~", "~तुः", "~ुः";
    perfect_mid_1: "~े", "~िवहे", "~िमहे";
    perfect_mid_2: "~िषे", "~ाथे", "~िध्वे";
    perfect_mid_3: _, "~ाते", "~िरे";
    participle_perf: "~िवांस्", "~ान";
    ),
    // धरति: 95 words (भरति, भुरति, दुष्यति, द्रायति, ओषति, द्रुह्यति).
    conjugation!("धरति", 95, "~~ति";
    ind_act_1: "~~ामि", "~~ावः", "~~ामः";
    ind_act_2: "~~सि", "~~थः", "~~थ";
    ind_act_3: "~~ति", "~~तः", "~~न्ति";
    ind_mid_1: "~~े", "~~ावहे", "~~ामहे";
    ind_mid_2: "~~से", "~~ेथे", "~~ध्वे";
    ind_mid_3: "~~ते", "~~ेते", "~~न्ते";
    imp_act_1: "~~ाणि", "~~ाव", "~~ाम";
    imp_act_2: "~~", "~~तम्", "~~त";
    imp_act_3: "~~तु", "~~ताम्", "~~न्तु";
    imp_mid_1: "~~ै", "~~ावहै", "~~ामहै";
    imp_mid_2: "~~स्व", "~~ेथाम्", "~~ध्वम्";
    imp_mid_3: "~~ताम्", "~~ेताम्", "~~न्ताम्";
    opt_act_1: "~~ेयम्", "~~ेव", "~~ेम";
    opt_act_2: "~~ेः", "~~ेतम्", "~~ेत";
    opt_act_3: "~~ेत्", "~~ेताम्", "~~ेयुः";
    opt_mid_1: "~~ेय", "~~ेवहि", "~~ेमहि";
    opt_mid_2: "~~ेथाः", "~~ेयाथाम्", "~~ेध्वम्";
    opt_mid_3: "~~ेत", "~~ेयाताम्", "~~ेरन्";
    subj_act_2: "~~ाः", "~~ाथः", "~~ाथ";
    subj_act_3: "~~ात्", "~~ातः", "~~ान्";
    subj_mid_2: "~~ासे", "~~ैथे", "~~ाध्वै";
    subj_mid_3: "~~ाते", "~~ैते", "~~न्त";
    imperfect_act_1: "अ~~म्", "अ~~ाव", "अ~~ाम";
    imperfect_act_2: "अ~~ः", "अ~~तम्", _;
    imperfect_act_3: "अ~~त्", "अ~~ताम्", "अ~~न्";
    imperfect_mid_1: "अ~~े", "अ~~ावहि", "अ~~ामहि";
    imperfect_mid_2: "अ~~थाः", "अ~~ेथाम्", "अ~~ध्वम्";
    imperfect_mid_3: "अ~~त", "अ~~ेताम्", "अ~~न्त";
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
    infinitive: "~~ितुम्";
    gerund: "~~ित्वा";
    participle_pres: "~~त्", "~~माण";
    participle_fut: "~~िष्यत्", "~~िष्यमाण";
    participle_perf: "~े~िवांस्", "~े~ाण";
    ),
    // साधयति: 86 words (इच्छति, दर्शयति, नाशयति, रन्धयति, जासयति, भाजयति). Like लभते, except in 15 cells: ind_act_1 pl, subj_act_2 sg, subj_act_3 sg, future_act_1 pl, perfect_act_1 sg, perfect_act_1 du, ….
    conjugation!("साधयति", 86, "~ति", like लभते;
    ind_act_1: _, _, "~ामः|~ामसि";
    subj_act_2: "~ाः|~ासि", _, _;
    subj_act_3: "~ात्|~ाति", _, _;
    future_act_1: _, _, "~िष्यामः|~िष्यामसि";
    perfect_act_1: "~ामास|~ांचकर", "~ामासिव|~ांचकृव", "~ामासिम|~ांचकृम";
    perfect_act_2: "~ामासिथ|~ांचकर्थ", "~ामासथुः|~ांचक्रथुः", "~ामास|~ांचक्र";
    perfect_act_3: "~ामास|~ांचकार", "~ामासतुः|~ांचक्रतुः", "~ामासुः|~ांचक्रुः";
    participle_pres: _, "~मान|~ान";
    participle_perf: "~ामासिवांस्|~ांचकृवांस्", _;
    ),
    // आपत: 46 words (अगमत्, अद्राक्षीत्, अश्रोत्, अभूत्, अताप्सीत्, अहासीत्). Like अमत, except in 38 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("आपत्", 46, "~त्", like अमत;
    ind_act_3: "~त्", _, _;
    perfect_act_1: derived, derived, derived;
    perfect_act_2: derived, derived, derived;
    perfect_act_3: derived, derived, derived;
    perfect_mid_1: derived, derived, derived;
    perfect_mid_2: derived, derived, derived;
    perfect_mid_3: _, derived, derived;
    aorist_act_1: "~म्", "~ाव", "~ाम";
    aorist_act_2: "~ः", "~तम्", "~त";
    aorist_act_3: "~त्", "~ताम्", "~न्";
    aorist_mid_1: "~े", "~ावहि", "~ामहि";
    aorist_mid_2: "~थाः", "~ेथाम्", "~ध्वम्";
    aorist_mid_3: "~त", "~ेताम्", "~न्त";
    participle_perf: derived, derived;
    ),
    // रमते: 42 words (म्रियते, भाषते, रमते, परीक्षते, त्रपते, ह्वरते). Like लभते, except in 2 cells: imp_act_1 sg, participle_pres mid.
    conjugation!("रमते", 42, "~ते", like लभते;
    imp_act_1: "~ाणि", _, _;
    participle_pres: _, "~माण";
    ),
    // एषयति: 36 words (करिष्यति, गमिष्यति, पास्यति, भविष्यति, तप्स्यति, भरिष्यति). Like आपत, except in 56 cells: ind_act_3 sg, future_act_1 sg, future_act_1 du, future_act_1 pl, future_act_2 sg, future_act_2 du, ….
    conjugation!("एष्यति", 36, "~ति", like आपत;
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
    aorist_act_1: derived, derived, derived;
    aorist_act_2: derived, derived, derived;
    aorist_act_3: derived, derived, derived;
    aorist_mid_1: derived, derived, derived;
    aorist_mid_2: derived, derived, derived;
    aorist_mid_3: derived, derived, derived;
    participle_fut: "~त्", "~माण";
    ),
    // ईरयति: 27 words (बर्हयति, धारयति, रोपयति, कारयति, शोषयति, चोरयति). Like साधयति, except in 2 cells: imp_act_1 sg, participle_pres mid.
    conjugation!("ईरयति", 27, "~ति", like साधयति;
    imp_act_1: "~ाणि", _, _;
    participle_pres: _, "~माण|~ाण";
    ),
    // खादति: 26 words (खादति, क्षालयति, दाम्यति, धावति, गायति, बाधति).
    conjugation!("खादति", 26, "~ा~ति";
    ind_act_1: "~ा~ामि", "~ा~ावः", "~ा~ामः";
    ind_act_2: "~ा~सि", "~ा~थः", "~ा~थ";
    ind_act_3: "~ा~ति", "~ा~तः", "~ा~न्ति";
    ind_mid_1: "~ा~े", "~ा~ावहे", "~ा~ामहे";
    ind_mid_2: "~ा~से", "~ा~ेथे", "~ा~ध्वे";
    ind_mid_3: "~ा~ते", "~ा~ेते", "~ा~न्ते";
    imp_act_1: "~ा~ानि", "~ा~ाव", "~ा~ाम";
    imp_act_2: "~ा~", "~ा~तम्", "~ा~त";
    imp_act_3: "~ा~तु", "~ा~ताम्", "~ा~न्तु";
    imp_mid_1: "~ा~ै", "~ा~ावहै", "~ा~ामहै";
    imp_mid_2: "~ा~स्व", "~ा~ेथाम्", "~ा~ध्वम्";
    imp_mid_3: "~ा~ताम्", "~ा~ेताम्", "~ा~न्ताम्";
    opt_act_1: "~ा~ेयम्", "~ा~ेव", "~ा~ेम";
    opt_act_2: "~ा~ेः", "~ा~ेतम्", "~ा~ेत";
    opt_act_3: "~ा~ेत्", "~ा~ेताम्", "~ा~ेयुः";
    opt_mid_1: "~ा~ेय", "~ा~ेवहि", "~ा~ेमहि";
    opt_mid_2: "~ा~ेथाः", "~ा~ेयाथाम्", "~ा~ेध्वम्";
    opt_mid_3: "~ा~ेत", "~ा~ेयाताम्", "~ा~ेरन्";
    subj_act_2: "~ा~ाः", "~ा~ाथः", "~ा~ाथ";
    subj_act_3: "~ा~ात्", "~ा~ातः", "~ा~ान्";
    subj_mid_2: "~ा~ासे", "~ा~ैथे", "~ा~ाध्वै";
    subj_mid_3: "~ा~ाते", "~ा~ैते", "~ा~न्त";
    imperfect_act_1: "अ~ा~म्", "अ~ा~ाव", "अ~ा~ाम";
    imperfect_act_2: "अ~ा~ः", "अ~ा~तम्", _;
    imperfect_act_3: "अ~ा~त्", "अ~ा~ताम्", "अ~ा~न्";
    imperfect_mid_1: "अ~ा~े", "अ~ा~ावहि", "अ~ा~ामहि";
    imperfect_mid_2: "अ~ा~थाः", "अ~ा~ेथाम्", "अ~ा~ध्वम्";
    imperfect_mid_3: "अ~ा~त", "अ~ा~ेताम्", "अ~ा~न्त";
    future_act_1: "~ा~िष्यामि", "~ा~िष्यावः", "~ा~िष्यामः";
    future_act_2: "~ा~िष्यसि", "~ा~िष्यथः", "~ा~िष्यथ";
    future_act_3: "~ा~िष्यति", "~ा~िष्यतः", "~ा~िष्यन्ति";
    future_mid_1: "~ा~िष्ये", "~ा~िष्यावहे", "~ा~िष्यामहे";
    future_mid_2: "~ा~िष्यसे", "~ा~िष्येथे", "~ा~िष्यध्वे";
    future_mid_3: "~ा~िष्यते", "~ा~िष्येते", "~ा~िष्यन्ते";
    conditional_act_1: "अ~ा~िष्यम्", "अ~ा~िष्याव", "अ~ा~िष्याम";
    conditional_act_2: "अ~ा~िष्यः", "अ~ा~िष्यतम्", "अ~ा~िष्यत";
    conditional_act_3: "अ~ा~िष्यत्", "अ~ा~िष्यताम्", "अ~ा~िष्यन्";
    conditional_mid_1: "अ~ा~िष्ये", "अ~ा~िष्यावहि", "अ~ा~िष्यामहि";
    conditional_mid_2: "अ~ा~िष्यथाः", "अ~ा~िष्येथाम्", "अ~ा~िष्यध्वम्";
    conditional_mid_3: _, "अ~ा~िष्येताम्", "अ~ा~िष्यन्त";
    perfect_act_1: "च~ा~", "च~ा~िव", "च~ा~िम";
    perfect_act_2: "च~ा~िथ", "च~ा~थुः", "च~ा~";
    perfect_act_3: "च~ा~", "च~ा~तुः", "च~ा~ुः";
    perfect_mid_1: "च~ा~े", "च~ा~िवहे", "च~ा~िमहे";
    perfect_mid_2: "च~ा~िषे", "च~ा~ाथे", "च~ा~िध्वे";
    perfect_mid_3: _, "च~ा~ाते", "च~ा~िरे";
    aorist_act_1: "अ~ा~िषम्", "अ~ा~िष्व", "अ~ा~िष्म";
    aorist_act_2: "अ~ा~ीः", "अ~ा~िष्टम्", "अ~ा~िष्ट";
    aorist_act_3: "अ~ा~ीत्", "अ~ा~िष्टाम्", "अ~ा~िषुः";
    aorist_mid_1: "अ~ा~िषि", "अ~ा~िष्वहि", "अ~ा~िष्महि";
    aorist_mid_2: "अ~ा~िष्ठाः", "अ~ा~िषाथाम्", "अ~ा~िढ्वम्";
    aorist_mid_3: "अ~ा~िष्ट", "अ~ा~िषाताम्", "अ~ा~िषत";
    infinitive: "~ा~ितुम्";
    gerund: "~~ित्वा";
    participle_pres: "~ा~त्", "~ा~मान";
    participle_fut: "~ा~िष्यत्", "~ा~िष्यमाण";
    participle_perf: "च~ा~्वांस्", "च~ा~ान";
    ),
    // ददाति: 19 words (ददाति, दधाति, जानाति, श्रद्दधाति, गृणाति, जहाति).
    conjugation!("ददाति", 19, "~ाति";
    ind_act_1: "~ामि", "~ीवः", "~ीमः|~ीमसि";
    ind_act_2: "~ासि", "~ीथः", "~ीथ";
    ind_act_3: "~ाति", "~ीतः", "~न्ति";
    ind_mid_1: "~े", "~ीवहे", "~ीमहे";
    ind_mid_2: "~ीषे", "~ाथे", "~ीध्वे";
    ind_mid_3: "~ीते", "~ाते", "~ते";
    imp_act_1: "~ानि", "~ाव", "~ाम";
    imp_act_2: "~ीहि", "~ीतम्", "~ीत";
    imp_act_3: "~ातु", "~ीताम्", "~न्तु";
    imp_mid_1: "~ै", "~ावहै", "~ामहै";
    imp_mid_2: "~ीष्व", "~ाथाम्", "~ीध्वम्";
    imp_mid_3: "~ीताम्", "~ाताम्", "~ताम्";
    opt_act_1: "~ीयाम्", "~ीयाव", "~ीयाम";
    opt_act_2: "~ीयाः", "~ीयातम्", "~ीयात";
    opt_act_3: "~ीयात्", "~ीयाताम्", "~ीयुः";
    opt_mid_1: "~ीय", "~ीवहि", "~ीमहि";
    opt_mid_2: "~ीथाः", "~ीयाथाम्", _;
    opt_mid_3: "~ीत", "~ीयाताम्", "~ीरन्";
    subj_act_1: "~ानि|~ा", _, _;
    subj_act_2: "~ाः|~ासि", "~ाथः", "~ाथ";
    subj_act_3: "~ात्|~ाति", "~ातः", "~ान्";
    subj_mid_2: "~ासे|~ासै", "~ैथे", "~ाध्वे|~ाध्वै";
    subj_mid_3: "~ाते|~ातै", "~ैते", "~ान्त|~ान्तै";
    imperfect_act_1: "अ~ाम्", "अ~ीव", "अ~ीम";
    imperfect_act_2: "अ~ाः", "अ~ीतम्", _;
    imperfect_act_3: "अ~ात्", "अ~ीताम्", "अ~न्";
    imperfect_mid_1: "अ~ि", "अ~ीवहि", "अ~ीमहि";
    imperfect_mid_2: "अ~ीथाः", "अ~ाथाम्", "अ~ीध्वम्";
    imperfect_mid_3: "अ~ीत", "अ~ाताम्", "अ~त";
    participle_pres: "~त्", "~ान";
    ),
    // तनोति: 16 words (वृणोति, तनोति, करोति, जुहोति, शृणोति, घृणोति).
    conjugation!("तनोति", 16, "~ोति";
    ind_act_1: "~ोमि", "~्वः", "~्मः";
    ind_act_2: "~ोषि", "~ुथः", "~ुथ";
    ind_act_3: "~ोति", "~ुतः", "~्वन्ति";
    ind_mid_1: "~्वे", "~्वहे", "~्महे";
    ind_mid_2: "~ुषे", "~्वाथे", "~ुध्वे";
    ind_mid_3: "~ुते", "~्वाते", "~्वते";
    imp_act_1: "~वानि", "~वाव", "~वाम";
    imp_act_2: "~ु", "~ुतम्", "~ुत";
    imp_act_3: "~ोतु", "~ुताम्", "~्वन्तु";
    imp_mid_1: "~वै", "~वावहै", "~वामहै";
    imp_mid_2: "~ुष्व", "~्वाथाम्", "~ुध्वम्";
    imp_mid_3: "~ुताम्", "~्वाताम्", "~्वताम्";
    opt_act_1: "~ुयाम्", "~ुयाव", "~ुयाम";
    opt_act_2: "~ुयाः", "~ुयातम्", "~ुयात";
    opt_act_3: "~ुयात्", "~ुयाताम्", "~ुयुः";
    opt_mid_1: "~्वीय", "~्वीवहि", "~्वीमहि";
    opt_mid_2: "~्वीथाः", "~्वीयाथाम्", "~्वीध्वम्";
    opt_mid_3: "~्वीत", "~्वीयाताम्", "~्वीरन्";
    subj_act_2: "~वः", "~वथः", "~वथ";
    subj_act_3: "~वत्", "~वतः", "~वन्";
    subj_mid_2: "~वसे", "~वैथे", "~वध्वे";
    subj_mid_3: "~वते", "~वैते", "~वन्त";
    imperfect_act_1: "अ~वम्", "अ~्व", "अ~्म";
    imperfect_act_2: "अ~ोः", "अ~ुतम्", _;
    imperfect_act_3: "अ~ोत्", "अ~ुताम्", "अ~्वन्";
    imperfect_mid_1: "अ~्वि", "अ~्वहि", "अ~्महि";
    imperfect_mid_2: "अ~ुथाः", "अ~्वाथाम्", "अ~ुध्वम्";
    imperfect_mid_3: "अ~ुत", "अ~्वाताम्", "अ~्वत";
    participle_pres: "~्वत्", "~्वान";
    ),
    // ईळे: 10 words (ईष्टे, चष्टे, ईळे, जज्ञे, ईट्टे, शश्ये). Like अमत, except in 11 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("ईळे", 10, "~े", like अमत;
    ind_act_3: "~े", _, _;
    perfect_act_1: derived, derived, derived;
    perfect_act_2: derived, derived, derived;
    perfect_act_3: derived, derived, derived;
    participle_perf: derived, _;
    ),
    // आजीत: 10 words (अस्वारीत्, आजीत्, अकाषीत्, आञ्चीत्, अलेखीत्, अवेदीत्). Like आपत, except in 19 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("आजीत्", 10, "~ीत्", like आपत;
    ind_act_3: "~ीत्", _, _;
    aorist_act_1: "~िषम्", "~िष्व", "~िष्म";
    aorist_act_2: "~ीः", "~िष्टम्", "~िष्ट";
    aorist_act_3: "~ीत्", "~िष्टाम्", "~िषुः";
    aorist_mid_1: "~िषि", "~िष्वहि", "~िष्महि";
    aorist_mid_2: "~िष्ठाः", "~िषाथाम्", "~िढ्वम्";
    aorist_mid_3: "~िष्ट", "~िषाताम्", "~िषत";
    ),
    // आपनोति: 8 words (धृष्णोति, शक्नोति, साध्नोति, आप्नोति, ऋध्नोति, दभ्नोति). Like तनोति, except in 37 cells: ind_act_1 du, ind_act_1 pl, ind_act_3 pl, ind_mid_1 sg, ind_mid_1 du, ind_mid_1 pl, ….
    conjugation!("आप्नोति", 8, "~ोति", like तनोति;
    ind_act_1: _, "~ुवः", "~ुमः|~ुमसि";
    ind_act_3: _, _, "~ुवन्ति";
    ind_mid_1: "~ुवे", "~ुवहे", "~ुमहे";
    ind_mid_2: _, "~ुवाथे", _;
    ind_mid_3: _, "~ुवाते", "~ुवते";
    imp_act_2: "~ुहि", _, _;
    imp_act_3: _, _, "~ुवन्तु";
    imp_mid_2: _, "~ुवाथाम्", _;
    imp_mid_3: _, "~ुवाताम्", "~ुवताम्";
    opt_mid_1: "~ुवीय", "~ुवीवहि", "~ुवीमहि";
    opt_mid_2: "~ुवीथाः", "~ुवीयाथाम्", "~ुवीध्वम्";
    opt_mid_3: "~ुवीत", "~ुवीयाताम्", "~ुवीरन्";
    subj_act_1: "~वानि|~वा", _, _;
    subj_act_2: "~वः|~वसि", _, _;
    subj_act_3: "~वत्|~वति", _, _;
    imperfect_act_1: _, "अ~ुव", "अ~ुम";
    imperfect_act_3: _, _, "अ~ुवन्";
    imperfect_mid_1: "अ~ुवि", "अ~ुवहि", "अ~ुमहि";
    imperfect_mid_2: _, "अ~ुवाथाम्", _;
    imperfect_mid_3: _, "अ~ुवाताम्", "अ~ुवत";
    participle_pres: "~ुवत्", "~ुवान";
    ),
    // ररंह: 8 words (आस, ततक्ष, ररंह, चकाङ्क्ष, जिहीड, जिहिंस). Like अमत, except in 1 cell: participle_perf act.
    conjugation!("ररंह", 8, "~", like अमत;
    participle_perf: "~्वांस्", _;
    ),
    // भाति: 7 words (भाति, द्राति, वाति, पाति, याति, आयाति). Like ददाति, except in 53 cells: ind_act_1 du, ind_act_1 pl, ind_act_2 du, ind_act_2 pl, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("भाति", 7, "~ाति", like ददाति;
    ind_act_1: _, "~ावः", "~ामः|~ामसि";
    ind_act_2: _, "~ाथः", "~ाथ";
    ind_act_3: _, "~ातः", "~ान्ति";
    ind_mid_1: "~ै", "~ावहे", "~ामहे";
    ind_mid_2: "~ासे", _, "~ाध्वे";
    ind_mid_3: "~ाते", _, "~ाते";
    imp_act_2: "~ाहि", "~ातम्", "~ात";
    imp_act_3: _, "~ाताम्", "~ान्तु";
    imp_mid_2: "~ास्व", _, "~ाध्वम्";
    imp_mid_3: "~ाताम्", _, "~ाताम्";
    opt_act_1: "~ायाम्", "~ायाव", "~ायाम";
    opt_act_2: "~ायाः", "~ायातम्", "~ायात";
    opt_act_3: "~ायात्", "~ायाताम्", "~ायुः";
    opt_mid_1: "~ेय", "~ेवहि", "~ेमहि";
    opt_mid_2: "~ेथाः", "~ेयाथाम्", "~ेध्वम्";
    opt_mid_3: "~ेत", "~ेयाताम्", "~ेरन्";
    imperfect_act_1: _, "अ~ाव", "अ~ाम";
    imperfect_act_2: _, "अ~ातम्", _;
    imperfect_act_3: _, "अ~ाताम्", "अ~ुः|अ~ान्";
    imperfect_mid_1: "अ~े", "अ~ावहि", "अ~ामहि";
    imperfect_mid_2: "अ~ाथाः", _, "अ~ाध्वम्";
    imperfect_mid_3: "अ~ात", _, "अ~ात";
    participle_pres: "~ात्", _;
    ),
    // पिपरति: 7 words (पिपर्ति, जागर्ति, बिभर्ति, जिघर्ति, इयर्ति, सिसर्ति).
    conjugation!("पिपर्ति", 7, "~र्ति";
    ind_act_1: "~र्मि", "~ृवः", "~ृमः|~ृमसि";
    ind_act_2: "~र्षि", "~ृथः", "~ृथ";
    ind_act_3: "~र्ति", "~ृतः", "~्रति";
    ind_mid_1: "~्रे", "~ृवहे", "~ृमहे";
    ind_mid_2: "~ृषे", "~्राथे", "~ृध्वे";
    ind_mid_3: "~ृते", "~्राते", "~्रते";
    imp_act_1: "~राणि", "~राव", "~राम";
    imp_act_2: "~ृहि", "~ृतम्", "~ृत";
    imp_act_3: "~र्तु", "~ृताम्", "~्रतु";
    imp_mid_1: "~रै", "~रावहै", "~रामहै";
    imp_mid_2: "~ृष्व", "~्राथाम्", "~ृध्वम्";
    imp_mid_3: "~ृताम्", "~्राताम्", "~्रताम्";
    opt_act_1: "~ृयाम्", "~ृयाव", "~ृयाम";
    opt_act_2: "~ृयाः", "~ृयातम्", "~ृयात";
    opt_act_3: "~ृयात्", "~ृयाताम्", "~ृयुः";
    opt_mid_1: "~्रीय", "~्रीवहि", "~्रीमहि";
    opt_mid_2: "~्रीथाः", "~्रीयाथाम्", "~्रीध्वम्";
    opt_mid_3: "~्रीत", "~्रीयाताम्", "~्रीरन्";
    subj_act_1: "~राणि|~रा", _, _;
    subj_act_2: "~रः|~रसि", "~रथः", "~रथ";
    subj_act_3: "~रत्|~रति", "~रतः", "~रन्";
    subj_mid_2: "~रसे", "~रैथे", "~रध्वे";
    subj_mid_3: "~रते", "~रैते", "~रन्त";
    imperfect_act_1: "अ~रम्", "अ~ृव", "अ~ृम";
    imperfect_act_2: "अ~ः", "अ~ृतम्", _;
    imperfect_act_3: "अ~ः", "अ~ृताम्", "अ~रुः";
    imperfect_mid_1: "अ~्रि", "अ~ृवहि", "अ~ृमहि";
    imperfect_mid_2: "अ~ृथाः", "अ~्राथाम्", "अ~ृध्वम्";
    imperfect_mid_3: "अ~ृत", "अ~्राताम्", "अ~्रत";
    participle_pres: "~्रत्", "~्राण";
    ),
    // बधनाति: 6 words (गृभ्णाति, उभ्नाति, गृह्णाति, इष्णाति, बध्नाति, तभ्नाति).
    conjugation!("बध्नाति", 6, "~्~ाति";
    ind_act_1: "~्~ामि", "~्~ीवः", "~्~ीमः|~्~ीमसि";
    ind_act_2: "~्~ासि", "~्~ीथः", "~्~ीथ";
    ind_act_3: "~्~ाति", "~्~ीतः", "~्~न्ति";
    ind_mid_1: "~्~े", "~्~ीवहे", "~्~ीमहे";
    ind_mid_2: "~्~ीषे", "~्~ाथे", "~्~ीध्वे";
    ind_mid_3: "~्~ीते", "~्~ाते", "~्~ते";
    imp_act_1: "~्~ानि", "~्~ाव", "~्~ाम";
    imp_act_2: "~ा~", "~्~ीतम्", "~्~ीत";
    imp_act_3: "~्~ातु", "~्~ीताम्", "~्~न्तु";
    imp_mid_1: "~्~ै", "~्~ावहै", "~्~ामहै";
    imp_mid_2: "~्~ीष्व", "~्~ाथाम्", "~्~ीध्वम्";
    imp_mid_3: "~्~ीताम्", "~्~ाताम्", "~्~ताम्";
    opt_act_1: "~्~ीयाम्", "~्~ीयाव", "~्~ीयाम";
    opt_act_2: "~्~ीयाः", "~्~ीयातम्", "~्~ीयात";
    opt_act_3: "~्~ीयात्", "~्~ीयाताम्", "~्~ीयुः";
    opt_mid_1: "~्~ीय", "~्~ीवहि", "~्~ीमहि";
    opt_mid_2: "~्~ीथाः", "~्~ीयाथाम्", _;
    opt_mid_3: "~्~ीत", "~्~ीयाताम्", "~्~ीरन्";
    subj_act_1: "~्~ानि|~्~ा", _, _;
    subj_act_2: "~्~ाः|~्~ासि", "~्~ाथः", "~्~ाथ";
    subj_act_3: "~्~ात्|~्~ाति", "~्~ातः", "~्~ान्";
    subj_mid_2: "~्~ासे", "~्~ैथे", "~्~ाध्वे";
    subj_mid_3: "~्~ाते", "~्~ैते", "~्~ान्त";
    imperfect_act_1: "अ~्~ाम्", "अ~्~ीव", "अ~्~ीम";
    imperfect_act_2: "अ~्~ाः", "अ~्~ीतम्", _;
    imperfect_act_3: "अ~्~ात्", "अ~्~ीताम्", "अ~्~न्";
    imperfect_mid_1: "अ~्~ि", "अ~्~ीवहि", "अ~्~ीमहि";
    imperfect_mid_2: "अ~्~ीथाः", "अ~्~ाथाम्", "अ~्~ीध्वम्";
    imperfect_mid_3: "अ~्~ीत", "अ~्~ाताम्", "अ~्~त";
    participle_pres: "~्~त्", "~्~ान";
    ),
    // ददौ: 6 words (तस्थौ, ददौ, पपौ, दधौ, ररौ, पप्रौ). Like अमत, except in 4 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_2 sg, perfect_act_3 sg.
    conjugation!("ददौ", 6, "~ौ", like अमत;
    ind_act_3: "~ौ", _, _;
    perfect_act_1: "~ौ", _, _;
    perfect_act_2: "~ाथ|~िथ", _, _;
    perfect_act_3: "~ौ", _, _;
    ),
    // भनकति: 5 words (भनक्ति, युनक्ति, तनक्ति, अनक्ति, भुनक्ति).
    conjugation!("भनक्ति", 5, "~नक्ति";
    ind_act_1: "~नज्मि", "~ञ्ज्वः", "~ञ्ज्मः|~ञ्ज्मसि";
    ind_act_2: "~नक्षि", "~ङ्क्थः", "~ङ्क्थ";
    ind_act_3: "~नक्ति", "~ङ्क्तः", "~ञ्जन्ति";
    ind_mid_1: "~ञ्जे", "~ञ्ज्वहे", "~ञ्ज्महे";
    ind_mid_2: "~ङ्क्षे", "~ञ्जाथे", "~ङ्ग्ध्वे";
    ind_mid_3: "~ङ्क्ते", "~ञ्जाते", "~ञ्जते";
    imp_act_1: "~नजानि", "~नजाव", "~नजाम";
    imp_act_2: "~ङ्ग्धि", "~ङ्क्तम्", "~ङ्क्त";
    imp_act_3: "~नक्तु", "~ङ्क्ताम्", "~ञ्जन्तु";
    imp_mid_1: "~नजै", "~नजावहै", "~नजामहै";
    imp_mid_2: "~ङ्क्ष्व", "~ञ्जाथाम्", "~ङ्ग्ध्वम्";
    imp_mid_3: "~ङ्क्ताम्", "~ञ्जाताम्", "~ञ्जताम्";
    opt_act_1: "~ञ्ज्याम्", "~ञ्ज्याव", "~ञ्ज्याम";
    opt_act_2: "~ञ्ज्याः", "~ञ्ज्यातम्", "~ञ्ज्यात";
    opt_act_3: "~ञ्ज्यात्", "~ञ्ज्याताम्", "~ञ्ज्युः";
    opt_mid_1: "~ञ्जीय", "~ञ्जीवहि", "~ञ्जीमहि";
    opt_mid_2: "~ञ्जीथाः", "~ञ्जीयाथाम्", "~ञ्जीध्वम्";
    opt_mid_3: "~ञ्जीत", "~ञ्जीयाताम्", "~ञ्जीरन्";
    subj_act_1: "~नजानि|~नजा", _, _;
    subj_act_2: "~नजः|~नजसि", "~नजथः", "~नजथ";
    subj_act_3: "~नजत्|~नजति", "~नजतः", "~नजन्";
    subj_mid_2: "~नजसे", "~नजैथे", "~नजध्वे";
    subj_mid_3: "~नजते", "~नजैते", "~नजन्त";
    imperfect_act_1: "अ~नजम्", "अ~ञ्ज्व", "अ~ञ्ज्म";
    imperfect_act_2: "अ~नक्", "अ~ङ्क्तम्", _;
    imperfect_act_3: "अ~नक्", "अ~ङ्क्ताम्", "अ~ञ्जन्";
    imperfect_mid_1: "अ~ञ्जि", "अ~ञ्ज्वहि", "अ~ञ्ज्महि";
    imperfect_mid_2: "अ~ङ्क्थाः", "अ~ञ्जाथाम्", "अ~ङ्ग्ध्वम्";
    imperfect_mid_3: "अ~ङ्क्त", "अ~ञ्जाताम्", "अ~ञ्जत";
    participle_pres: "~ञ्जत्", "~ञ्जान";
    ),
    // अगात: 5 words (अधात्, अस्थात्, अदात्, अगात्, अपात्). Like आपत, except in 17 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_2 sg, aorist_act_2 du, aorist_act_2 pl, aorist_act_3 sg, ….
    conjugation!("अगात्", 5, "~ात्", like आपत;
    ind_act_3: "~ात्", _, _;
    aorist_act_1: "~ाम्", _, _;
    aorist_act_2: "~ाः", "~ातम्", "~ात";
    aorist_act_3: "~ात्", "~ाताम्", "~ुः";
    aorist_mid_1: "~ि", "~िवहि", "~िमहि";
    aorist_mid_2: "~िथाः", "~ाथाम्", "~िध्वम्";
    aorist_mid_3: "~ित", "~ाताम्", "~िरन्";
    ),
    // जुहाव: 5 words (दुद्राव, शुश्राव, तुष्टाव, सुस्राव, जुहाव). Like अमत, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("जुहाव", 5, "~ाव", like अमत;
    ind_act_3: "~ाव", _, _;
    perfect_act_1: "~व|~ाव", "~ुव", "~ुम";
    perfect_act_2: "~ोथ", "~ुवथुः", "~ुव";
    perfect_act_3: "~ाव", "~ुवतुः", "~ुवुः";
    perfect_mid_1: "~ुवे", "~ुवहे", "~ुमहे";
    perfect_mid_2: "~ुषे", "~ुवाथे", "~ुध्वे";
    perfect_mid_3: _, "~ुवाते", "~ुविरे";
    participle_perf: "~ुवांस्", "~ुवाण";
    ),
    // इयाय: 5 words (बिभाय, जिगाय, इयाय, जिघाय, निनाय). Like अमत, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("इयाय", 5, "~ाय", like अमत;
    ind_act_3: "~ाय", _, _;
    perfect_act_1: "~य|~ाय", "~्यिव", "~्यिम";
    perfect_act_2: "~ेथ|~यिथ", "~्यथुः", "~्य";
    perfect_act_3: "~ाय", "~्यतुः", "~्युः";
    perfect_mid_1: "~्ये", "~्यिवहे", "~्यिमहे";
    perfect_mid_2: "~्यिषे", "~्याथे", "~्यिध्वे";
    perfect_mid_3: _, "~्याते", "~्यिरे";
    participle_perf: "~ीवांस्", "~्यान";
    ),
    // चचार: 5 words (जहार, दधार, जगार, चचार, ममार). Like अमत, except in 20 cells: ind_act_3 sg, perfect_act_1 sg, perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_act_2 du, ….
    conjugation!("चचार", 5, "~ार", like अमत;
    ind_act_3: "~ार", _, _;
    perfect_act_1: "~र|~ार", "~्रिव", "~्रिम";
    perfect_act_2: "~र्थ|~रिथ", "~्रथुः", "~्र";
    perfect_act_3: "~ार", "~्रतुः", "~्रुः";
    perfect_mid_1: "~्रे", "~्रिवहे", "~्रिमहे";
    perfect_mid_2: "~्रिषे", "~्राथे", "~्रिध्वे";
    perfect_mid_3: _, "~्राते", "~्रिरे";
    participle_perf: "~ृवांस्", "~्राण";
    ),
    // वमति: 4 words (वमति, चमति, नमति, गमति).
    conjugation!("वमति", 4, "~मति";
    ind_act_1: "~मामि", "~मावः", "~मामः";
    ind_act_2: "~मसि", "~मथः", "~मथ";
    ind_act_3: "~मति", "~मतः", "~मन्ति";
    ind_mid_1: "~मे", "~मावहे", "~मामहे";
    ind_mid_2: "~मसे", "~मेथे", "~मध्वे";
    ind_mid_3: "~मते", "~मेते", "~मन्ते";
    imp_act_1: "~मानि", "~माव", "~माम";
    imp_act_2: "~म", "~मतम्", "~मत";
    imp_act_3: "~मतु", "~मताम्", "~मन्तु";
    imp_mid_1: "~मै", "~मावहै", "~मामहै";
    imp_mid_2: "~मस्व", "~मेथाम्", "~मध्वम्";
    imp_mid_3: "~मताम्", "~मेताम्", "~मन्ताम्";
    opt_act_1: "~मेयम्", "~मेव", "~मेम";
    opt_act_2: "~मेः", "~मेतम्", "~मेत";
    opt_act_3: "~मेत्", "~मेताम्", "~मेयुः";
    opt_mid_1: "~मेय", "~मेवहि", "~मेमहि";
    opt_mid_2: "~मेथाः", "~मेयाथाम्", "~मेध्वम्";
    opt_mid_3: "~मेत", "~मेयाताम्", "~मेरन्";
    subj_act_2: "~माः", "~माथः", "~माथ";
    subj_act_3: "~मात्", "~मातः", "~मान्";
    subj_mid_2: "~मासे", "~मैथे", "~माध्वै";
    subj_mid_3: "~माते", "~मैते", "~मन्त";
    imperfect_act_1: "अ~मम्", "अ~माव", "अ~माम";
    imperfect_act_2: "अ~मः", "अ~मतम्", _;
    imperfect_act_3: "अ~मत्", "अ~मताम्", "अ~मन्";
    imperfect_mid_1: "अ~मे", "अ~मावहि", "अ~मामहि";
    imperfect_mid_2: "अ~मथाः", "अ~मेथाम्", "अ~मध्वम्";
    imperfect_mid_3: "अ~मत", "अ~मेताम्", "अ~मन्त";
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
    perfect_act_2: "~चन्थ", "~ेमथुः", "~ेम";
    perfect_act_3: "~चाम", "~ेमतुः", "~ेमुः";
    perfect_mid_1: "~ेमे", "~ेमिवहे", "~ेमिमहे";
    perfect_mid_2: "~ेमिषे", "~ेमाथे", "~ेमिध्वे";
    perfect_mid_3: _, "~ेमाते", "~ेमिरे";
    aorist_act_1: "अ~मिषम्", "अ~मिष्व", "अ~मिष्म";
    aorist_act_2: "अ~मीः", "अ~मिष्टम्", "अ~मिष्ट";
    aorist_act_3: "अ~मीत्", "अ~मिष्टाम्", "अ~मिषुः";
    aorist_mid_1: "अ~मिषि", "अ~मिष्वहि", "अ~मिष्महि";
    aorist_mid_2: "अ~मिष्ठाः", "अ~मिषाथाम्", "अ~मिढ्वम्";
    aorist_mid_3: "अ~मिष्ट", "अ~मिषाताम्", "अ~मिषत";
    infinitive: "~मितुम्";
    gerund: "~मित्वा";
    participle_pres: "~मत्", "~ममान";
    participle_fut: "~मिष्यत्", "~मिष्यमाण";
    participle_perf: "~ेमिवांस्", "~ेमान";
    ),
    // वृणीते: 4 words (मिमीते, जानीते, जिहीते, वृणीते). Like ददाति, except in 43 cells: ind_act_1 sg, ind_act_1 du, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ….
    conjugation!("वृणीते", 4, "~ीते", like ददाति;
    ind_act_1: derived, derived, derived;
    ind_act_2: derived, derived, derived;
    ind_act_3: "~ीते", derived, derived;
    imp_act_1: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    opt_act_1: derived, derived, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    subj_act_1: derived, _, _;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, derived, derived;
    imperfect_act_1: derived, derived, derived;
    imperfect_act_2: derived, derived, _;
    imperfect_act_3: derived, derived, derived;
    participle_pres: derived, _;
    ),
    // रवति: 4 words (द्रवति, चर्वति, तूर्वति, रवति).
    conjugation!("रवति", 4, "~वति";
    ind_act_1: "~वामि", "~वावः", "~वामः";
    ind_act_2: "~वसि", "~वथः", "~वथ";
    ind_act_3: "~वति", "~वतः", "~वन्ति";
    ind_mid_1: "~वे", "~वावहे", "~वामहे";
    ind_mid_2: "~वसे", "~वेथे", "~वध्वे";
    ind_mid_3: "~वते", "~वेते", "~वन्ते";
    imp_act_1: "~वाणि", "~वाव", "~वाम";
    imp_act_2: "~व", "~वतम्", "~वत";
    imp_act_3: "~वतु", "~वताम्", "~वन्तु";
    imp_mid_1: "~वै", "~वावहै", "~वामहै";
    imp_mid_2: "~वस्व", "~वेथाम्", "~वध्वम्";
    imp_mid_3: "~वताम्", "~वेताम्", "~वन्ताम्";
    opt_act_1: "~वेयम्", "~वेव", "~वेम";
    opt_act_2: "~वेः", "~वेतम्", "~वेत";
    opt_act_3: "~वेत्", "~वेताम्", "~वेयुः";
    opt_mid_1: "~वेय", "~वेवहि", "~वेमहि";
    opt_mid_2: "~वेथाः", "~वेयाथाम्", "~वेध्वम्";
    opt_mid_3: "~वेत", "~वेयाताम्", "~वेरन्";
    subj_act_2: "~वाः", "~वाथः", "~वाथ";
    subj_act_3: "~वात्", "~वातः", "~वान्";
    subj_mid_2: "~वासे", "~वैथे", "~वाध्वै";
    subj_mid_3: "~वाते", "~वैते", "~वन्त";
    imperfect_act_1: "अ~वम्", "अ~वाव", "अ~वाम";
    imperfect_act_2: "अ~वः", "अ~वतम्", _;
    imperfect_act_3: "अ~वत्", "अ~वताम्", "अ~वन्";
    imperfect_mid_1: "अ~वे", "अ~वावहि", "अ~वामहि";
    imperfect_mid_2: "अ~वथाः", "अ~वेथाम्", "अ~वध्वम्";
    imperfect_mid_3: "अ~वत", "अ~वेताम्", "अ~वन्त";
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
    perfect_act_3: "दु~ाव", "दु~ुवतुः", "दु~ुवुः";
    perfect_mid_1: "दु~ुवे", "दु~ुवहे", "दु~ुमहे";
    perfect_mid_2: "दु~ुषे", "दु~ुवाथे", "दु~ुध्वे";
    perfect_mid_3: _, "दु~ुवाते", "दु~ुविरे";
    aorist_act_1: "अ~ुद्रुवम्", "अ~ुद्रुवाव", "अ~ुद्रुवाम";
    aorist_act_2: "अ~ुद्रुवः", "अ~ुद्रुवतम्", "अ~ुद्रुवत";
    aorist_act_3: "अ~ुद्रुवत्", "अ~ुद्रुवताम्", "अ~ुद्रुवन्";
    aorist_mid_1: "अ~ाविषि", "अ~ाविष्वहि", "अ~ाविष्महि";
    aorist_mid_2: "अ~ाविष्ठाः", "अ~ाविषाथाम्", "अ~ाविढ्वम्";
    aorist_mid_3: "अ~ाविष्ट", "अ~ाविषाताम्", "अ~ाविषत";
    infinitive: "~ोतुम्";
    gerund: "~ुत्वा";
    participle_pres: "~वत्", "~वमाण";
    participle_fut: "~ोष्यत्", "~ोष्यमाण";
    participle_perf: "दु~ुवांस्", "दु~ुवाण";
    ),
    // आनयति: 4 words (आनयति, आगच्छति, आक्रोशति, आमयति). Like साधयति, except in 55 cells: imperfect_act_1 sg, imperfect_act_1 du, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_3 sg, ….
    conjugation!("आनयति", 4, "~ति", like साधयति;
    imperfect_act_1: "~म्", "~ाव", "~ाम";
    imperfect_act_2: "~ः", "~तम्", _;
    imperfect_act_3: "~त्", "~ताम्", "~न्";
    imperfect_mid_1: "~े", "~ावहि", "~ामहि";
    imperfect_mid_2: "~थाः", "~ेथाम्", "~ध्वम्";
    imperfect_mid_3: "~त", "~ेताम्", "~न्त";
    future_act_1: _, _, "~िष्यामः";
    conditional_act_1: derived, derived, derived;
    conditional_act_2: derived, derived, derived;
    conditional_act_3: derived, derived, derived;
    conditional_mid_1: derived, derived, derived;
    conditional_mid_2: derived, derived, derived;
    conditional_mid_3: _, derived, derived;
    perfect_act_1: derived, derived, derived;
    perfect_act_2: derived, derived, derived;
    perfect_act_3: derived, derived, derived;
    perfect_mid_1: derived, derived, derived;
    perfect_mid_2: derived, derived, derived;
    perfect_mid_3: _, derived, derived;
    participle_pres: _, "~मान";
    participle_perf: derived, derived;
    ),
    // चकार: 3 words (चकार, बभार, जभार). Like चचार, except in 7 cells: perfect_act_1 du, perfect_act_1 pl, perfect_act_2 sg, perfect_mid_1 du, perfect_mid_1 pl, perfect_mid_2 sg, ….
    conjugation!("चकार", 3, "~ार", like चचार;
    perfect_act_1: _, "~ृव", "~ृम";
    perfect_act_2: "~र्थ", _, _;
    perfect_mid_1: _, "~ृवहे", "~ृमहे";
    perfect_mid_2: "~ृषे", _, "~ृध्वे";
    ),
    // आचरति: 3 words (आचरति, पक्षति, यक्षति). Like आनयति, except in 24 cells: imp_act_1 sg, future_act_1 sg, future_act_1 du, future_act_1 pl, future_act_2 sg, future_act_2 du, ….
    conjugation!("आचरति", 3, "~ति", like आनयति;
    imp_act_1: "~ाणि", _, _;
    future_act_1: derived, derived, derived;
    future_act_2: derived, derived, derived;
    future_act_3: derived, derived, derived;
    future_mid_1: derived, derived, derived;
    future_mid_2: derived, derived, derived;
    future_mid_3: derived, derived, derived;
    infinitive: derived;
    gerund: derived;
    participle_pres: _, "~माण";
    participle_fut: derived, derived;
    ),
    // अकारषीत: 3 words (अहार्षीत्, अभार्षीत्, अकार्षीत्). Like आपत, except in 19 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("अकार्षीत्", 3, "~ार्षीत्", like आपत;
    ind_act_3: "~ार्षीत्", _, _;
    aorist_act_1: "~ार्षम्", "~ार्ष्व", "~ार्ष्म";
    aorist_act_2: "~ार्षीः|~ाः", "~ार्ष्टम्", "~ार्ष्ट";
    aorist_act_3: "~ार्षीत्|~ाः", "~ार्ष्टाम्", "~ार्षुः";
    aorist_mid_1: "~ृषि", "~ृष्वहि", "~ृष्महि";
    aorist_mid_2: "~ृष्ठाः", "~ृषाथाम्", "~ृढ्वम्";
    aorist_mid_3: "~ृष्ट", "~ृषाताम्", "~ृषत";
    ),
    // अधुकषत: 3 words (अधुक्षत्, अद्रुक्षत्, अरुक्षत्). Like आपत, except in 3 cells: aorist_mid_1 sg, aorist_mid_2 du, aorist_mid_3 du.
    conjugation!("अधुक्षत्", 3, "~त्", like आपत;
    aorist_mid_1: "~ि", _, _;
    aorist_mid_2: _, "~ाथाम्", _;
    aorist_mid_3: _, "~ाताम्", _;
    ),
    // अचैषीत: 3 words (अनैषीत्, अजैषीत्, अचैषीत्). Like आपत, except in 19 cells: ind_act_3 sg, aorist_act_1 sg, aorist_act_1 du, aorist_act_1 pl, aorist_act_2 sg, aorist_act_2 du, ….
    conjugation!("अचैषीत्", 3, "~ैषीत्", like आपत;
    ind_act_3: "~ैषीत्", _, _;
    aorist_act_1: "~ैषम्", "~ैष्व", "~ैष्म";
    aorist_act_2: "~ैषीः|~ैः", "~ैष्टम्", "~ैष्ट";
    aorist_act_3: "~ैषीत्|~ैः", "~ैष्टाम्", "~ैषुः";
    aorist_mid_1: "~ेषि", "~ेष्वहि", "~ेष्महि";
    aorist_mid_2: "~ेष्ठाः", "~ेषाथाम्", "~ेढ्वम्";
    aorist_mid_3: "~ेष्ट", "~ेषाताम्", "~ेषत";
    ),
];
