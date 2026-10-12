//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `indicative_active_first` (sg, du, pl)
/// `indicative_active_second` (sg, du, pl)
/// `indicative_active_third` (sg, du, pl)
/// `indicative_middle_first` (sg, du, pl)
/// `indicative_middle_second` (sg, du, pl)
/// `indicative_middle_third` (sg, du, pl)
/// `imperative_active_first` (sg, du, pl)
/// `imperative_active_second` (sg, du, pl)
/// `imperative_active_third` (sg, du, pl)
/// `imperative_middle_first` (sg, du, pl)
/// `imperative_middle_second` (sg, du, pl)
/// `imperative_middle_third` (sg, du, pl)
/// `optative_active_first` (sg, du, pl)
/// `optative_active_second` (sg, du, pl)
/// `optative_active_third` (sg, du, pl)
/// `optative_middle_first` (sg, du, pl)
/// `optative_middle_second` (sg, du, pl)
/// `optative_middle_third` (sg, du, pl)
/// `subjunctive_active_first` (sg, du, pl)
/// `subjunctive_active_second` (sg, du, pl)
/// `subjunctive_active_third` (sg, du, pl)
/// `subjunctive_middle_first` (sg, du, pl)
/// `subjunctive_middle_second` (sg, du, pl)
/// `subjunctive_middle_third` (sg, du, pl)
/// `imperfect_active_first` (sg, du, pl)
/// `imperfect_active_second` (sg, du, pl)
/// `imperfect_active_third` (sg, du, pl)
/// `imperfect_middle_first` (sg, du, pl)
/// `imperfect_middle_second` (sg, du, pl)
/// `imperfect_middle_third` (sg, du, pl)
/// `future_active_first` (sg, du, pl)
/// `future_active_second` (sg, du, pl)
/// `future_active_third` (sg, du, pl)
/// `future_middle_first` (sg, du, pl)
/// `future_middle_second` (sg, du, pl)
/// `future_middle_third` (sg, du, pl)
/// `conditional_active_first` (sg, du, pl)
/// `conditional_active_second` (sg, du, pl)
/// `conditional_active_third` (sg, du, pl)
/// `conditional_middle_first` (sg, du, pl)
/// `conditional_middle_second` (sg, du, pl)
/// `conditional_middle_third` (sg, du, pl)
/// `perfect_active_first` (sg, du, pl)
/// `perfect_active_second` (sg, du, pl)
/// `perfect_active_third` (sg, du, pl)
/// `perfect_middle_first` (sg, du, pl)
/// `perfect_middle_second` (sg, du, pl)
/// `perfect_middle_third` (sg, du, pl)
/// `aorist_active_first` (sg, du, pl)
/// `aorist_active_second` (sg, du, pl)
/// `aorist_active_third` (sg, du, pl)
/// `aorist_middle_first` (sg, du, pl)
/// `aorist_middle_second` (sg, du, pl)
/// `aorist_middle_third` (sg, du, pl)
/// `infinitive`
/// `gerund`
/// `participle_present` (act, mid)
/// `participle_future` (act, mid)
/// `participle_perfect` (act, mid)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     indicative_active_first: $c0:tt, $c1:tt, $c2:tt;
     indicative_active_second: $c3:tt, $c4:tt, $c5:tt;
     indicative_active_third: $c6:tt, $c7:tt, $c8:tt;
     indicative_middle_first: $c9:tt, $c10:tt, $c11:tt;
     indicative_middle_second: $c12:tt, $c13:tt, $c14:tt;
     indicative_middle_third: $c15:tt, $c16:tt, $c17:tt;
     imperative_active_first: $c18:tt, $c19:tt, $c20:tt;
     imperative_active_second: $c21:tt, $c22:tt, $c23:tt;
     imperative_active_third: $c24:tt, $c25:tt, $c26:tt;
     imperative_middle_first: $c27:tt, $c28:tt, $c29:tt;
     imperative_middle_second: $c30:tt, $c31:tt, $c32:tt;
     imperative_middle_third: $c33:tt, $c34:tt, $c35:tt;
     optative_active_first: $c36:tt, $c37:tt, $c38:tt;
     optative_active_second: $c39:tt, $c40:tt, $c41:tt;
     optative_active_third: $c42:tt, $c43:tt, $c44:tt;
     optative_middle_first: $c45:tt, $c46:tt, $c47:tt;
     optative_middle_second: $c48:tt, $c49:tt, $c50:tt;
     optative_middle_third: $c51:tt, $c52:tt, $c53:tt;
     subjunctive_active_first: $c54:tt, $c55:tt, $c56:tt;
     subjunctive_active_second: $c57:tt, $c58:tt, $c59:tt;
     subjunctive_active_third: $c60:tt, $c61:tt, $c62:tt;
     subjunctive_middle_first: $c63:tt, $c64:tt, $c65:tt;
     subjunctive_middle_second: $c66:tt, $c67:tt, $c68:tt;
     subjunctive_middle_third: $c69:tt, $c70:tt, $c71:tt;
     imperfect_active_first: $c72:tt, $c73:tt, $c74:tt;
     imperfect_active_second: $c75:tt, $c76:tt, $c77:tt;
     imperfect_active_third: $c78:tt, $c79:tt, $c80:tt;
     imperfect_middle_first: $c81:tt, $c82:tt, $c83:tt;
     imperfect_middle_second: $c84:tt, $c85:tt, $c86:tt;
     imperfect_middle_third: $c87:tt, $c88:tt, $c89:tt;
     future_active_first: $c90:tt, $c91:tt, $c92:tt;
     future_active_second: $c93:tt, $c94:tt, $c95:tt;
     future_active_third: $c96:tt, $c97:tt, $c98:tt;
     future_middle_first: $c99:tt, $c100:tt, $c101:tt;
     future_middle_second: $c102:tt, $c103:tt, $c104:tt;
     future_middle_third: $c105:tt, $c106:tt, $c107:tt;
     conditional_active_first: $c108:tt, $c109:tt, $c110:tt;
     conditional_active_second: $c111:tt, $c112:tt, $c113:tt;
     conditional_active_third: $c114:tt, $c115:tt, $c116:tt;
     conditional_middle_first: $c117:tt, $c118:tt, $c119:tt;
     conditional_middle_second: $c120:tt, $c121:tt, $c122:tt;
     conditional_middle_third: $c123:tt, $c124:tt, $c125:tt;
     perfect_active_first: $c126:tt, $c127:tt, $c128:tt;
     perfect_active_second: $c129:tt, $c130:tt, $c131:tt;
     perfect_active_third: $c132:tt, $c133:tt, $c134:tt;
     perfect_middle_first: $c135:tt, $c136:tt, $c137:tt;
     perfect_middle_second: $c138:tt, $c139:tt, $c140:tt;
     perfect_middle_third: $c141:tt, $c142:tt, $c143:tt;
     aorist_active_first: $c144:tt, $c145:tt, $c146:tt;
     aorist_active_second: $c147:tt, $c148:tt, $c149:tt;
     aorist_active_third: $c150:tt, $c151:tt, $c152:tt;
     aorist_middle_first: $c153:tt, $c154:tt, $c155:tt;
     aorist_middle_second: $c156:tt, $c157:tt, $c158:tt;
     aorist_middle_third: $c159:tt, $c160:tt, $c161:tt;
     infinitive: $c162:tt;
     gerund: $c163:tt;
     participle_present: $c164:tt, $c165:tt;
     participle_future: $c166:tt, $c167:tt;
     participle_perfect: $c168:tt, $c169:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
                flexion::form!($c45),
                flexion::form!($c46),
                flexion::form!($c47),
                flexion::form!($c48),
                flexion::form!($c49),
                flexion::form!($c50),
                flexion::form!($c51),
                flexion::form!($c52),
                flexion::form!($c53),
                flexion::form!($c54),
                flexion::form!($c55),
                flexion::form!($c56),
                flexion::form!($c57),
                flexion::form!($c58),
                flexion::form!($c59),
                flexion::form!($c60),
                flexion::form!($c61),
                flexion::form!($c62),
                flexion::form!($c63),
                flexion::form!($c64),
                flexion::form!($c65),
                flexion::form!($c66),
                flexion::form!($c67),
                flexion::form!($c68),
                flexion::form!($c69),
                flexion::form!($c70),
                flexion::form!($c71),
                flexion::form!($c72),
                flexion::form!($c73),
                flexion::form!($c74),
                flexion::form!($c75),
                flexion::form!($c76),
                flexion::form!($c77),
                flexion::form!($c78),
                flexion::form!($c79),
                flexion::form!($c80),
                flexion::form!($c81),
                flexion::form!($c82),
                flexion::form!($c83),
                flexion::form!($c84),
                flexion::form!($c85),
                flexion::form!($c86),
                flexion::form!($c87),
                flexion::form!($c88),
                flexion::form!($c89),
                flexion::form!($c90),
                flexion::form!($c91),
                flexion::form!($c92),
                flexion::form!($c93),
                flexion::form!($c94),
                flexion::form!($c95),
                flexion::form!($c96),
                flexion::form!($c97),
                flexion::form!($c98),
                flexion::form!($c99),
                flexion::form!($c100),
                flexion::form!($c101),
                flexion::form!($c102),
                flexion::form!($c103),
                flexion::form!($c104),
                flexion::form!($c105),
                flexion::form!($c106),
                flexion::form!($c107),
                flexion::form!($c108),
                flexion::form!($c109),
                flexion::form!($c110),
                flexion::form!($c111),
                flexion::form!($c112),
                flexion::form!($c113),
                flexion::form!($c114),
                flexion::form!($c115),
                flexion::form!($c116),
                flexion::form!($c117),
                flexion::form!($c118),
                flexion::form!($c119),
                flexion::form!($c120),
                flexion::form!($c121),
                flexion::form!($c122),
                flexion::form!($c123),
                flexion::form!($c124),
                flexion::form!($c125),
                flexion::form!($c126),
                flexion::form!($c127),
                flexion::form!($c128),
                flexion::form!($c129),
                flexion::form!($c130),
                flexion::form!($c131),
                flexion::form!($c132),
                flexion::form!($c133),
                flexion::form!($c134),
                flexion::form!($c135),
                flexion::form!($c136),
                flexion::form!($c137),
                flexion::form!($c138),
                flexion::form!($c139),
                flexion::form!($c140),
                flexion::form!($c141),
                flexion::form!($c142),
                flexion::form!($c143),
                flexion::form!($c144),
                flexion::form!($c145),
                flexion::form!($c146),
                flexion::form!($c147),
                flexion::form!($c148),
                flexion::form!($c149),
                flexion::form!($c150),
                flexion::form!($c151),
                flexion::form!($c152),
                flexion::form!($c153),
                flexion::form!($c154),
                flexion::form!($c155),
                flexion::form!($c156),
                flexion::form!($c157),
                flexion::form!($c158),
                flexion::form!($c159),
                flexion::form!($c160),
                flexion::form!($c161),
                flexion::form!($c162),
                flexion::form!($c163),
                flexion::form!($c164),
                flexion::form!($c165),
                flexion::form!($c166),
                flexion::form!($c167),
                flexion::form!($c168),
                flexion::form!($c169),
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `वहति`: 247 words.
    वहति,
    /// Like `लभते`: 112 words.
    लभते,
    /// Like `अमत`: 107 words.
    अमत,
    /// Like `धरति`: 99 words.
    धरति,
    /// Like `एति`: 81 words.
    एति,
    /// Like `रमते`: 43 words.
    रमते,
    /// Like `आपत्`: 41 words.
    आपत,
    /// Like `खादति`: 28 words.
    खादति,
    /// Like `ईरयति`: 28 words.
    ईरयति,
    /// Like `एष्यति`: 28 words.
    एषयति,
    /// Like `ददाति`: 19 words.
    ददाति,
    /// Like `तनोति`: 16 words.
    तनोति,
    /// Like `अवति`: 14 words.
    अवति,
    /// Like `आजीत्`: 11 words.
    आजीत,
    /// Like `भाति`: 10 words.
    भाति,
    /// Like `धास्यति`: 10 words.
    धासयति,
    /// Like `अनंसीत्`: 9 words.
    अनंसीत,
    /// Like `आप्नोति`: 8 words.
    आपनोति,
    /// Like `बध्नाति`: 7 words.
    बधनाति,
    /// Like `ददौ`: 7 words.
    ददौ,
    /// Like `पिपर्ति`: 7 words.
    पिपरति,
    /// Like `भनक्ति`: 6 words.
    भनकति,
    /// Like `उपविशति`: 6 words.
    उपविशति,
    /// Like `ईळे`: 5 words.
    ईळे,
    /// Like `अगात्`: 5 words.
    अगात,
    /// Like `जुहाव`: 5 words.
    जुहाव,
    /// Like `इयाय`: 5 words.
    इयाय,
    /// Like `चचार`: 5 words.
    चचार,
    /// Like `आद`: 4 words.
    आद,
    /// Like `वमति`: 4 words.
    वमति,
    /// Like `वृणीते`: 4 words.
    वृणीते,
    /// Like `स्रवति`: 4 words.
    सरवति,
    /// Like `आनयति`: 4 words.
    आनयति,
    /// Like `इरस्यति`: 4 words.
    इरसयति,
    /// Like `चकार`: 3 words.
    चकार,
    /// Like `आचरति`: 3 words.
    आचरति,
    /// Like `प्रवदति`: 3 words.
    परवदति,
    /// Like `मुमोच`: 3 words.
    मुमोच,
    /// Like `अकार्षीत्`: 3 words.
    अकारषीत,
    /// Like `अधुक्षत्`: 3 words.
    अधुकषत,
    /// Like `अचैषीत्`: 3 words.
    अचैषीत,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 41] = [
        Conjugation::वहति,
        Conjugation::लभते,
        Conjugation::अमत,
        Conjugation::धरति,
        Conjugation::एति,
        Conjugation::रमते,
        Conjugation::आपत,
        Conjugation::खादति,
        Conjugation::ईरयति,
        Conjugation::एषयति,
        Conjugation::ददाति,
        Conjugation::तनोति,
        Conjugation::अवति,
        Conjugation::आजीत,
        Conjugation::भाति,
        Conjugation::धासयति,
        Conjugation::अनंसीत,
        Conjugation::आपनोति,
        Conjugation::बधनाति,
        Conjugation::ददौ,
        Conjugation::पिपरति,
        Conjugation::भनकति,
        Conjugation::उपविशति,
        Conjugation::ईळे,
        Conjugation::अगात,
        Conjugation::जुहाव,
        Conjugation::इयाय,
        Conjugation::चचार,
        Conjugation::आद,
        Conjugation::वमति,
        Conjugation::वृणीते,
        Conjugation::सरवति,
        Conjugation::आनयति,
        Conjugation::इरसयति,
        Conjugation::चकार,
        Conjugation::आचरति,
        Conjugation::परवदति,
        Conjugation::मुमोच,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 41] = [
    // वहति: 247 words (निन्दति, वयति, बोधति, भवति, यभति, दहति).
    conjugation!("वहति", 247, "~~ति";
        indicative_active_first: "~~ामि", "~~ावः", "~~ामः";
        indicative_active_second: "~~सि", "~~थः", "~~थ";
        indicative_active_third: "~~ति", "~~तः", "~~न\u{94d}ति";
        indicative_middle_first: "~~\u{947}", "~~ावह\u{947}", "~~ामह\u{947}";
        indicative_middle_second: "~~स\u{947}", "~~\u{947}थ\u{947}", "~~ध\u{94d}व\u{947}";
        indicative_middle_third: "~~त\u{947}", "~~\u{947}त\u{947}", "~~न\u{94d}त\u{947}";
        imperative_active_first: "~~ानि", "~~ाव", "~~ाम";
        imperative_active_second: "~~", "~~तम\u{94d}", "~~त";
        imperative_active_third: "~~त\u{941}", "~~ताम\u{94d}", "~~न\u{94d}त\u{941}";
        imperative_middle_first: "~~\u{948}", "~~ावह\u{948}", "~~ामह\u{948}";
        imperative_middle_second: "~~स\u{94d}व", "~~\u{947}थाम\u{94d}", "~~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~~ताम\u{94d}", "~~\u{947}ताम\u{94d}", "~~न\u{94d}ताम\u{94d}";
        optative_active_first: "~~\u{947}यम\u{94d}", "~~\u{947}व", "~~\u{947}म";
        optative_active_second: "~~\u{947}ः", "~~\u{947}तम\u{94d}", "~~\u{947}त";
        optative_active_third: "~~\u{947}त\u{94d}", "~~\u{947}ताम\u{94d}", "~~\u{947}य\u{941}ः";
        optative_middle_first: "~~\u{947}य", "~~\u{947}वहि", "~~\u{947}महि";
        optative_middle_second: "~~\u{947}थाः", "~~\u{947}याथाम\u{94d}", "~~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~~\u{947}त", "~~\u{947}याताम\u{94d}", "~~\u{947}रन\u{94d}";
        subjunctive_active_first: "~~ानि", "~~ाव", "~~ाम";
        subjunctive_active_second: "~~ाः", "~~ाथः", "~~ाथ";
        subjunctive_active_third: "~~ात\u{94d}", "~~ातः", "~~ान\u{94d}";
        subjunctive_middle_first: "~~\u{948}", "~~ावह\u{948}", "~~ामह\u{948}";
        subjunctive_middle_second: "~~ास\u{947}", "~~\u{948}थ\u{947}", "~~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~~ात\u{947}", "~~\u{948}त\u{947}", "~~न\u{94d}त";
        imperfect_active_first: "अ~~म\u{94d}", "अ~~ाव", "अ~~ाम";
        imperfect_active_second: "अ~~ः", "अ~~तम\u{94d}", "अ~~त";
        imperfect_active_third: "अ~~त\u{94d}", "अ~~ताम\u{94d}", "अ~~न\u{94d}";
        imperfect_middle_first: "अ~~\u{947}", "अ~~ावहि", "अ~~ामहि";
        imperfect_middle_second: "अ~~थाः", "अ~~\u{947}थाम\u{94d}", "अ~~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~~त", "अ~~\u{947}ताम\u{94d}", "अ~~न\u{94d}त";
        future_active_first: "~~िष\u{94d}यामि", "~~िष\u{94d}यावः", "~~िष\u{94d}यामः";
        future_active_second: "~~िष\u{94d}यसि", "~~िष\u{94d}यथः", "~~िष\u{94d}यथ";
        future_active_third: "~~िष\u{94d}यति", "~~िष\u{94d}यतः", "~~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~~िष\u{94d}य\u{947}", "~~िष\u{94d}यावह\u{947}", "~~िष\u{94d}यामह\u{947}";
        future_middle_second: "~~िष\u{94d}यस\u{947}", "~~िष\u{94d}य\u{947}थ\u{947}", "~~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~~िष\u{94d}यत\u{947}", "~~िष\u{94d}य\u{947}त\u{947}", "~~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~~िष\u{94d}यम\u{94d}", "अ~~िष\u{94d}याव", "अ~~िष\u{94d}याम";
        conditional_active_second: "अ~~िष\u{94d}यः", "अ~~िष\u{94d}यतम\u{94d}", "अ~~िष\u{94d}यत";
        conditional_active_third: "अ~~िष\u{94d}यत\u{94d}", "अ~~िष\u{94d}यताम\u{94d}", "अ~~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~~िष\u{94d}य\u{947}", "अ~~िष\u{94d}यावहि", "अ~~िष\u{94d}यामहि";
        conditional_middle_second: "अ~~िष\u{94d}यथाः", "अ~~िष\u{94d}य\u{947}थाम\u{94d}", "अ~~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~~िष\u{94d}यत", "अ~~िष\u{94d}य\u{947}ताम\u{94d}", "अ~~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~िन~", "~िन~िव", "~िन~िम";
        perfect_active_second: "~िन~िथ", "~िन~थ\u{941}ः", "~िन~";
        perfect_active_third: "~िन~", "~िन~त\u{941}ः", "~िन~\u{941}ः";
        perfect_middle_first: "~िन~\u{947}", "~िन~िवह\u{947}", "~िन~िमह\u{947}";
        perfect_middle_second: "~िन~िष\u{947}", "~िन~ाथ\u{947}", "~िन~िध\u{94d}व\u{947}";
        perfect_middle_third: "~िन~\u{947}", "~िन~ात\u{947}", "~िन~िर\u{947}";
        aorist_active_first: "अ~~िषम\u{94d}", "अ~~िष\u{94d}व", "अ~~िष\u{94d}म";
        aorist_active_second: "अ~~ीः", "अ~~िष\u{94d}टम\u{94d}", "अ~~िष\u{94d}ट";
        aorist_active_third: "अ~~ीत\u{94d}", "अ~~िष\u{94d}टाम\u{94d}", "अ~~िष\u{941}ः";
        aorist_middle_first: "अ~~िषि", "अ~~िष\u{94d}वहि", "अ~~िष\u{94d}महि";
        aorist_middle_second: "अ~~िष\u{94d}ठाः", "अ~~िषाथाम\u{94d}", "अ~~िढ\u{94d}वम\u{94d}";
        aorist_middle_third: "अ~~िष\u{94d}ट", "अ~~िषाताम\u{94d}", "अ~~िषत";
        infinitive: "~~ित\u{941}म\u{94d}";
        gerund: "~~ित\u{94d}वा";
        participle_present: "~~त\u{94d}", "~~मान";
        participle_future: "~~िष\u{94d}यत\u{94d}", "~~िष\u{94d}यमाण";
        participle_perfect: "~िन~\u{94d}वा\u{902}स\u{94d}", "~िन~ान";
    ),
    // लभते: 112 words (वस्ते, सचते, रोचते, मन्यते, पर्दते, स्वेदते).
    conjugation!("लभत\u{947}", 112, "~त\u{947}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~न\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // अमत: 107 words (अमृत, वेद, देग्धि, द्वेष्टि, आस, ईष्टे).
    conjugation!("अमत", 107, "~";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~", "~िव", "~िम";
        perfect_active_second: "~िथ", "~थ\u{941}ः", "~";
        perfect_active_third: "~", "~त\u{941}ः", "~\u{941}ः";
        perfect_middle_first: "~\u{947}", "~िवह\u{947}", "~िमह\u{947}";
        perfect_middle_second: "~िष\u{947}", "~ाथ\u{947}", "~िध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{947}", "~ात\u{947}", "~िर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~\u{94d}वा\u{902}स\u{94d}", "~ान";
    ),
    // धरति: 99 words (भरति, भुरति, दुष्यति, द्रायति, ओषति, द्रुह्यति).
    conjugation!("धरति", 99, "~~ति";
        indicative_active_first: "~~ामि", "~~ावः", "~~ामः";
        indicative_active_second: "~~सि", "~~थः", "~~थ";
        indicative_active_third: "~~ति", "~~तः", "~~न\u{94d}ति";
        indicative_middle_first: "~~\u{947}", "~~ावह\u{947}", "~~ामह\u{947}";
        indicative_middle_second: "~~स\u{947}", "~~\u{947}थ\u{947}", "~~ध\u{94d}व\u{947}";
        indicative_middle_third: "~~त\u{947}", "~~\u{947}त\u{947}", "~~न\u{94d}त\u{947}";
        imperative_active_first: "~~ाणि", "~~ाव", "~~ाम";
        imperative_active_second: "~~", "~~तम\u{94d}", "~~त";
        imperative_active_third: "~~त\u{941}", "~~ताम\u{94d}", "~~न\u{94d}त\u{941}";
        imperative_middle_first: "~~\u{948}", "~~ावह\u{948}", "~~ामह\u{948}";
        imperative_middle_second: "~~स\u{94d}व", "~~\u{947}थाम\u{94d}", "~~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~~ताम\u{94d}", "~~\u{947}ताम\u{94d}", "~~न\u{94d}ताम\u{94d}";
        optative_active_first: "~~\u{947}यम\u{94d}", "~~\u{947}व", "~~\u{947}म";
        optative_active_second: "~~\u{947}ः", "~~\u{947}तम\u{94d}", "~~\u{947}त";
        optative_active_third: "~~\u{947}त\u{94d}", "~~\u{947}ताम\u{94d}", "~~\u{947}य\u{941}ः";
        optative_middle_first: "~~\u{947}य", "~~\u{947}वहि", "~~\u{947}महि";
        optative_middle_second: "~~\u{947}थाः", "~~\u{947}याथाम\u{94d}", "~~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~~\u{947}त", "~~\u{947}याताम\u{94d}", "~~\u{947}रन\u{94d}";
        subjunctive_active_first: "~~ाणि", "~~ाव", "~~ाम";
        subjunctive_active_second: "~~ाः", "~~ाथः", "~~ाथ";
        subjunctive_active_third: "~~ात\u{94d}", "~~ातः", "~~ान\u{94d}";
        subjunctive_middle_first: "~~\u{948}", "~~ावह\u{948}", "~~ामह\u{948}";
        subjunctive_middle_second: "~~ास\u{947}", "~~\u{948}थ\u{947}", "~~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~~ात\u{947}", "~~\u{948}त\u{947}", "~~न\u{94d}त";
        imperfect_active_first: "अ~~म\u{94d}", "अ~~ाव", "अ~~ाम";
        imperfect_active_second: "अ~~ः", "अ~~तम\u{94d}", "अ~~त";
        imperfect_active_third: "अ~~त\u{94d}", "अ~~ताम\u{94d}", "अ~~न\u{94d}";
        imperfect_middle_first: "अ~~\u{947}", "अ~~ावहि", "अ~~ामहि";
        imperfect_middle_second: "अ~~थाः", "अ~~\u{947}थाम\u{94d}", "अ~~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~~त", "अ~~\u{947}ताम\u{94d}", "अ~~न\u{94d}त";
        future_active_first: "~~िष\u{94d}यामि", "~~िष\u{94d}यावः", "~~िष\u{94d}यामः";
        future_active_second: "~~िष\u{94d}यसि", "~~िष\u{94d}यथः", "~~िष\u{94d}यथ";
        future_active_third: "~~िष\u{94d}यति", "~~िष\u{94d}यतः", "~~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~~िष\u{94d}य\u{947}", "~~िष\u{94d}यावह\u{947}", "~~िष\u{94d}यामह\u{947}";
        future_middle_second: "~~िष\u{94d}यस\u{947}", "~~िष\u{94d}य\u{947}थ\u{947}", "~~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~~िष\u{94d}यत\u{947}", "~~िष\u{94d}य\u{947}त\u{947}", "~~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~~िष\u{94d}यम\u{94d}", "अ~~िष\u{94d}याव", "अ~~िष\u{94d}याम";
        conditional_active_second: "अ~~िष\u{94d}यः", "अ~~िष\u{94d}यतम\u{94d}", "अ~~िष\u{94d}यत";
        conditional_active_third: "अ~~िष\u{94d}यत\u{94d}", "अ~~िष\u{94d}यताम\u{94d}", "अ~~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~~िष\u{94d}य\u{947}", "अ~~िष\u{94d}यावहि", "अ~~िष\u{94d}यामहि";
        conditional_middle_second: "अ~~िष\u{94d}यथाः", "अ~~िष\u{94d}य\u{947}थाम\u{94d}", "अ~~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~~िष\u{94d}यत", "अ~~िष\u{94d}य\u{947}ताम\u{94d}", "अ~~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~च~", "~\u{947}~िव", "~\u{947}~िम";
        perfect_active_second: "~च~\u{94d}थ", "~\u{947}~थ\u{941}ः", "~\u{947}~";
        perfect_active_third: "~चा~", "~\u{947}~त\u{941}ः", "~\u{947}~\u{941}ः";
        perfect_middle_first: "~\u{947}~\u{947}", "~\u{947}~िवह\u{947}", "~\u{947}~िमह\u{947}";
        perfect_middle_second: "~\u{947}~िष\u{947}", "~\u{947}~ाथ\u{947}", "~\u{947}~िध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{947}~\u{947}", "~\u{947}~ात\u{947}", "~\u{947}~िर\u{947}";
        aorist_active_first: "अ~ा~िषम\u{94d}", "अ~ा~िष\u{94d}व", "अ~ा~िष\u{94d}म";
        aorist_active_second: "अ~ा~ीः", "अ~ा~िष\u{94d}टम\u{94d}", "अ~ा~िष\u{94d}ट";
        aorist_active_third: "अ~ा~ीत\u{94d}", "अ~ा~िष\u{94d}टाम\u{94d}", "अ~ा~िष\u{941}ः";
        aorist_middle_first: "अ~ा~िषि", "अ~ा~िष\u{94d}वहि", "अ~ा~िष\u{94d}महि";
        aorist_middle_second: "अ~ा~िष\u{94d}ठाः", "अ~ा~िषाथाम\u{94d}", "अ~ा~िढ\u{94d}वम\u{94d}";
        aorist_middle_third: "अ~ा~िष\u{94d}ट", "अ~ा~िषाताम\u{94d}", "अ~ा~िषत";
        infinitive: "~~ित\u{941}म\u{94d}";
        gerund: "~~ित\u{94d}वा";
        participle_present: "~~त\u{94d}", "~~माण";
        participle_future: "~~िष\u{94d}यत\u{94d}", "~~िष\u{94d}यमाण";
        participle_perfect: "~\u{947}~िवा\u{902}स\u{94d}", "~\u{947}~ाण";
    ),
    // एति: 81 words (एति, दर्शयति, नाशयति, रन्धयति, जासयति, भाजयति).
    conjugation!("एति", 81, "~ति";
        indicative_active_first: "~ामि", "~ावः", "~ामः|~ामसि";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः|~ासि", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}|~ाति", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~न\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः|~िष\u{94d}यामसि";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास|~ा\u{902}चकर", "~ामासिव|~ा\u{902}चक\u{943}व", "~ामासिम|~ा\u{902}चक\u{943}म";
        perfect_active_second: "~ामासिथ|~ा\u{902}चकर\u{94d}थ", "~ामासथ\u{941}ः|~ा\u{902}चक\u{94d}रथ\u{941}ः", "~ामास|~ा\u{902}चक\u{94d}र";
        perfect_active_third: "~ामास|~ा\u{902}चकार", "~ामासत\u{941}ः|~ा\u{902}चक\u{94d}रत\u{941}ः", "~ामास\u{941}ः|~ा\u{902}चक\u{94d}र\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान|~ान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}|~ा\u{902}चक\u{943}वा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // रमते: 43 words (प्रवते, म्रियते, भाषते, रमते, परीक्षते, त्रपते).
    conjugation!("रमत\u{947}", 43, "~त\u{947}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ाणि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ाणि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~न\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~माण";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // आपत: 41 words (अगमत्, अद्राक्षीत्, अश्रोत्, अभूत्, अवोचत्, अलेकिष्यत्).
    conjugation!("आपत\u{94d}", 41, "~त\u{94d}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // खादति: 28 words (खादति, क्षालयति, दाम्यति, धावति, गायति, बाधति).
    conjugation!("खादति", 28, "~ा~ति";
        indicative_active_first: "~ा~ामि", "~ा~ावः", "~ा~ामः";
        indicative_active_second: "~ा~सि", "~ा~थः", "~ा~थ";
        indicative_active_third: "~ा~ति", "~ा~तः", "~ा~न\u{94d}ति";
        indicative_middle_first: "~ा~\u{947}", "~ा~ावह\u{947}", "~ा~ामह\u{947}";
        indicative_middle_second: "~ा~स\u{947}", "~ा~\u{947}थ\u{947}", "~ा~ध\u{94d}व\u{947}";
        indicative_middle_third: "~ा~त\u{947}", "~ा~\u{947}त\u{947}", "~ा~न\u{94d}त\u{947}";
        imperative_active_first: "~ा~ानि", "~ा~ाव", "~ा~ाम";
        imperative_active_second: "~ा~", "~ा~तम\u{94d}", "~ा~त";
        imperative_active_third: "~ा~त\u{941}", "~ा~ताम\u{94d}", "~ा~न\u{94d}त\u{941}";
        imperative_middle_first: "~ा~\u{948}", "~ा~ावह\u{948}", "~ा~ामह\u{948}";
        imperative_middle_second: "~ा~स\u{94d}व", "~ा~\u{947}थाम\u{94d}", "~ा~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ा~ताम\u{94d}", "~ा~\u{947}ताम\u{94d}", "~ा~न\u{94d}ताम\u{94d}";
        optative_active_first: "~ा~\u{947}यम\u{94d}", "~ा~\u{947}व", "~ा~\u{947}म";
        optative_active_second: "~ा~\u{947}ः", "~ा~\u{947}तम\u{94d}", "~ा~\u{947}त";
        optative_active_third: "~ा~\u{947}त\u{94d}", "~ा~\u{947}ताम\u{94d}", "~ा~\u{947}य\u{941}ः";
        optative_middle_first: "~ा~\u{947}य", "~ा~\u{947}वहि", "~ा~\u{947}महि";
        optative_middle_second: "~ा~\u{947}थाः", "~ा~\u{947}याथाम\u{94d}", "~ा~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~ा~\u{947}त", "~ा~\u{947}याताम\u{94d}", "~ा~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ा~ानि", "~ा~ाव", "~ा~ाम";
        subjunctive_active_second: "~ा~ाः", "~ा~ाथः", "~ा~ाथ";
        subjunctive_active_third: "~ा~ात\u{94d}", "~ा~ातः", "~ा~ान\u{94d}";
        subjunctive_middle_first: "~ा~\u{948}", "~ा~ावह\u{948}", "~ा~ामह\u{948}";
        subjunctive_middle_second: "~ा~ास\u{947}", "~ा~\u{948}थ\u{947}", "~ा~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ा~ात\u{947}", "~ा~\u{948}त\u{947}", "~ा~न\u{94d}त";
        imperfect_active_first: "अ~ा~म\u{94d}", "अ~ा~ाव", "अ~ा~ाम";
        imperfect_active_second: "अ~ा~ः", "अ~ा~तम\u{94d}", "अ~ा~त";
        imperfect_active_third: "अ~ा~त\u{94d}", "अ~ा~ताम\u{94d}", "अ~ा~न\u{94d}";
        imperfect_middle_first: "अ~ा~\u{947}", "अ~ा~ावहि", "अ~ा~ामहि";
        imperfect_middle_second: "अ~ा~थाः", "अ~ा~\u{947}थाम\u{94d}", "अ~ा~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~ा~त", "अ~ा~\u{947}ताम\u{94d}", "अ~ा~न\u{94d}त";
        future_active_first: "~ा~िष\u{94d}यामि", "~ा~िष\u{94d}यावः", "~ा~िष\u{94d}यामः";
        future_active_second: "~ा~िष\u{94d}यसि", "~ा~िष\u{94d}यथः", "~ा~िष\u{94d}यथ";
        future_active_third: "~ा~िष\u{94d}यति", "~ा~िष\u{94d}यतः", "~ा~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~ा~िष\u{94d}य\u{947}", "~ा~िष\u{94d}यावह\u{947}", "~ा~िष\u{94d}यामह\u{947}";
        future_middle_second: "~ा~िष\u{94d}यस\u{947}", "~ा~िष\u{94d}य\u{947}थ\u{947}", "~ा~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~ा~िष\u{94d}यत\u{947}", "~ा~िष\u{94d}य\u{947}त\u{947}", "~ा~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~ा~िष\u{94d}यम\u{94d}", "अ~ा~िष\u{94d}याव", "अ~ा~िष\u{94d}याम";
        conditional_active_second: "अ~ा~िष\u{94d}यः", "अ~ा~िष\u{94d}यतम\u{94d}", "अ~ा~िष\u{94d}यत";
        conditional_active_third: "अ~ा~िष\u{94d}यत\u{94d}", "अ~ा~िष\u{94d}यताम\u{94d}", "अ~ा~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~ा~िष\u{94d}य\u{947}", "अ~ा~िष\u{94d}यावहि", "अ~ा~िष\u{94d}यामहि";
        conditional_middle_second: "अ~ा~िष\u{94d}यथाः", "अ~ा~िष\u{94d}य\u{947}थाम\u{94d}", "अ~ा~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~ा~िष\u{94d}यत", "अ~ा~िष\u{94d}य\u{947}ताम\u{94d}", "अ~ा~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "च~ा~", "च~ा~िव", "च~ा~िम";
        perfect_active_second: "च~ा~िथ", "च~ा~थ\u{941}ः", "च~ा~";
        perfect_active_third: "च~ा~", "च~ा~त\u{941}ः", "च~ा~\u{941}ः";
        perfect_middle_first: "च~ा~\u{947}", "च~ा~िवह\u{947}", "च~ा~िमह\u{947}";
        perfect_middle_second: "च~ा~िष\u{947}", "च~ा~ाथ\u{947}", "च~ा~िध\u{94d}व\u{947}";
        perfect_middle_third: "च~ा~\u{947}", "च~ा~ात\u{947}", "च~ा~िर\u{947}";
        aorist_active_first: "अ~ा~िषम\u{94d}", "अ~ा~िष\u{94d}व", "अ~ा~िष\u{94d}म";
        aorist_active_second: "अ~ा~ीः", "अ~ा~िष\u{94d}टम\u{94d}", "अ~ा~िष\u{94d}ट";
        aorist_active_third: "अ~ा~ीत\u{94d}", "अ~ा~िष\u{94d}टाम\u{94d}", "अ~ा~िष\u{941}ः";
        aorist_middle_first: "अ~ा~िषि", "अ~ा~िष\u{94d}वहि", "अ~ा~िष\u{94d}महि";
        aorist_middle_second: "अ~ा~िष\u{94d}ठाः", "अ~ा~िषाथाम\u{94d}", "अ~ा~िढ\u{94d}वम\u{94d}";
        aorist_middle_third: "अ~ा~िष\u{94d}ट", "अ~ा~िषाताम\u{94d}", "अ~ा~िषत";
        infinitive: "~ा~ित\u{941}म\u{94d}";
        gerund: "~~ित\u{94d}वा";
        participle_present: "~ा~त\u{94d}", "~ा~मान";
        participle_future: "~ा~िष\u{94d}यत\u{94d}", "~ा~िष\u{94d}यमाण";
        participle_perfect: "च~ा~\u{94d}वा\u{902}स\u{94d}", "च~ा~ान";
    ),
    // ईरयति: 28 words (बर्हयति, धारयति, रोपयति, कारयति, शोषयति, चोरयति).
    conjugation!("ईरयति", 28, "~ति";
        indicative_active_first: "~ामि", "~ावः", "~ामः|~ामसि";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ाणि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ाणि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः|~ासि", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}|~ाति", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~न\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः|~िष\u{94d}यामसि";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास|~ा\u{902}चकर", "~ामासिव|~ा\u{902}चक\u{943}व", "~ामासिम|~ा\u{902}चक\u{943}म";
        perfect_active_second: "~ामासिथ|~ा\u{902}चकर\u{94d}थ", "~ामासथ\u{941}ः|~ा\u{902}चक\u{94d}रथ\u{941}ः", "~ामास|~ा\u{902}चक\u{94d}र";
        perfect_active_third: "~ामास|~ा\u{902}चकार", "~ामासत\u{941}ः|~ा\u{902}चक\u{94d}रत\u{941}ः", "~ामास\u{941}ः|~ा\u{902}चक\u{94d}र\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~माण|~ाण";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}|~ा\u{902}चक\u{943}वा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // एषयति: 28 words (करिष्यति, गमिष्यति, भविष्यति, भरिष्यति, अञ्चिष्यति, वक्ष्यति).
    conjugation!("एष\u{94d}यति", 28, "~ति";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~ामि", "~ावः", "~ामः|~ामसि";
        future_active_second: "~सि", "~थः", "~थ";
        future_active_third: "~ति", "~तः", "~न\u{94d}ति";
        future_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        future_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        future_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        conditional_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        conditional_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        conditional_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        conditional_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        conditional_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~त\u{94d}", "~माण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // ददाति: 19 words (ददाति, दधाति, जानाति, प्रीणाति, श्रद्दधाति, गृणाति).
    conjugation!("ददाति", 19, "~ाति";
        indicative_active_first: "~ामि", "~ीवः", "~ीमः|~ीमसि";
        indicative_active_second: "~ासि", "~ीथः", "~ीथ";
        indicative_active_third: "~ाति", "~ीतः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ीवह\u{947}", "~ीमह\u{947}";
        indicative_middle_second: "~ीष\u{947}", "~ाथ\u{947}", "~ीध\u{94d}व\u{947}";
        indicative_middle_third: "~ीत\u{947}", "~ात\u{947}", "~त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~ीहि", "~ीतम\u{94d}", "~ीत";
        imperative_active_third: "~ात\u{941}", "~ीताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~ीष\u{94d}व", "~ाथाम\u{94d}", "~ीध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ीताम\u{94d}", "~ाताम\u{94d}", "~ताम\u{94d}";
        optative_active_first: "~ीयाम\u{94d}", "~ीयाव", "~ीयाम";
        optative_active_second: "~ीयाः", "~ीयातम\u{94d}", "~ीयात";
        optative_active_third: "~ीयात\u{94d}", "~ीयाताम\u{94d}", "~ीय\u{941}ः";
        optative_middle_first: "~ीय", "~ीवहि", "~ीमहि";
        optative_middle_second: "~ीथाः", "~ीयाथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~ीत", "~ीयाताम\u{94d}", "~ीरन\u{94d}";
        subjunctive_active_first: "~ानि|~ा", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः|~ासि", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}|~ाति", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{947}|~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~ान\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "अ~ाम\u{94d}", "अ~ीव", "अ~ीम";
        imperfect_active_second: "अ~ाः", "अ~ीतम\u{94d}", "अ~ीत";
        imperfect_active_third: "अ~ात\u{94d}", "अ~ीताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~ि", "अ~ीवहि", "अ~ीमहि";
        imperfect_middle_second: "अ~ीथाः", "अ~ाथाम\u{94d}", "अ~ीध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~ीत", "अ~ाताम\u{94d}", "अ~त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~ान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // तनोति: 16 words (वृणोति, तनोति, करोति, जुहोति, शृणोति, घृणोति).
    conjugation!("तनोति", 16, "~ोति";
        indicative_active_first: "~ोमि", "~\u{94d}वः", "~\u{94d}मः";
        indicative_active_second: "~ोषि", "~\u{941}थः", "~\u{941}थ";
        indicative_active_third: "~ोति", "~\u{941}तः", "~\u{94d}वन\u{94d}ति";
        indicative_middle_first: "~\u{94d}व\u{947}", "~\u{94d}वह\u{947}", "~\u{94d}मह\u{947}";
        indicative_middle_second: "~\u{941}ष\u{947}", "~\u{94d}वाथ\u{947}", "~\u{941}ध\u{94d}व\u{947}";
        indicative_middle_third: "~\u{941}त\u{947}", "~\u{94d}वात\u{947}", "~\u{94d}वत\u{947}";
        imperative_active_first: "~वानि", "~वाव", "~वाम";
        imperative_active_second: "~\u{941}", "~\u{941}तम\u{94d}", "~\u{941}त";
        imperative_active_third: "~ोत\u{941}", "~\u{941}ताम\u{94d}", "~\u{94d}वन\u{94d}त\u{941}";
        imperative_middle_first: "~व\u{948}", "~वावह\u{948}", "~वामह\u{948}";
        imperative_middle_second: "~\u{941}ष\u{94d}व", "~\u{94d}वाथाम\u{94d}", "~\u{941}ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~\u{941}ताम\u{94d}", "~\u{94d}वाताम\u{94d}", "~\u{94d}वताम\u{94d}";
        optative_active_first: "~\u{941}याम\u{94d}", "~\u{941}याव", "~\u{941}याम";
        optative_active_second: "~\u{941}याः", "~\u{941}यातम\u{94d}", "~\u{941}यात";
        optative_active_third: "~\u{941}यात\u{94d}", "~\u{941}याताम\u{94d}", "~\u{941}य\u{941}ः";
        optative_middle_first: "~\u{94d}वीय", "~\u{94d}वीवहि", "~\u{94d}वीमहि";
        optative_middle_second: "~\u{94d}वीथाः", "~\u{94d}वीयाथाम\u{94d}", "~\u{94d}वीध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{94d}वीत", "~\u{94d}वीयाताम\u{94d}", "~\u{94d}वीरन\u{94d}";
        subjunctive_active_first: "~वानि", "~वाव", "~वाम";
        subjunctive_active_second: "~वः", "~वथः", "~वथ";
        subjunctive_active_third: "~वत\u{94d}", "~वतः", "~वन\u{94d}";
        subjunctive_middle_first: "~व\u{948}", "~वावह\u{948}", "~वामह\u{948}";
        subjunctive_middle_second: "~वस\u{947}", "~व\u{948}थ\u{947}", "~वध\u{94d}व\u{947}";
        subjunctive_middle_third: "~वत\u{947}", "~व\u{948}त\u{947}", "~वन\u{94d}त";
        imperfect_active_first: "अ~वम\u{94d}", "अ~\u{94d}व", "अ~\u{94d}म";
        imperfect_active_second: "अ~ोः", "अ~\u{941}तम\u{94d}", "अ~\u{941}त";
        imperfect_active_third: "अ~ोत\u{94d}", "अ~\u{941}ताम\u{94d}", "अ~\u{94d}वन\u{94d}";
        imperfect_middle_first: "अ~\u{94d}वि", "अ~\u{94d}वहि", "अ~\u{94d}महि";
        imperfect_middle_second: "अ~\u{941}थाः", "अ~\u{94d}वाथाम\u{94d}", "अ~\u{941}ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~\u{941}त", "अ~\u{94d}वाताम\u{94d}", "अ~\u{94d}वत";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~\u{94d}वत\u{94d}", "~\u{94d}वान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // अवति: 14 words (अस्ति, अत्ति, अजति, अवति, अर्हति, अर्चति).
    conjugation!("अवति", 14, "अ~ति";
        indicative_active_first: "अ~ामि", "अ~ावः", "अ~ामः|अ~ामसि";
        indicative_active_second: "अ~सि", "अ~थः", "अ~थ";
        indicative_active_third: "अ~ति", "अ~तः", "अ~न\u{94d}ति";
        indicative_middle_first: "अ~\u{947}", "अ~ावह\u{947}", "अ~ामह\u{947}";
        indicative_middle_second: "अ~स\u{947}", "अ~\u{947}थ\u{947}", "अ~ध\u{94d}व\u{947}";
        indicative_middle_third: "अ~त\u{947}", "अ~\u{947}त\u{947}", "अ~न\u{94d}त\u{947}";
        imperative_active_first: "अ~ानि", "अ~ाव", "अ~ाम";
        imperative_active_second: "अ~", "अ~तम\u{94d}", "अ~त";
        imperative_active_third: "अ~त\u{941}", "अ~ताम\u{94d}", "अ~न\u{94d}त\u{941}";
        imperative_middle_first: "अ~\u{948}", "अ~ावह\u{948}", "अ~ामह\u{948}";
        imperative_middle_second: "अ~स\u{94d}व", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "अ~ताम\u{94d}", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}ताम\u{94d}";
        optative_active_first: "अ~\u{947}यम\u{94d}", "अ~\u{947}व", "अ~\u{947}म";
        optative_active_second: "अ~\u{947}ः", "अ~\u{947}तम\u{94d}", "अ~\u{947}त";
        optative_active_third: "अ~\u{947}त\u{94d}", "अ~\u{947}ताम\u{94d}", "अ~\u{947}य\u{941}ः";
        optative_middle_first: "अ~\u{947}य", "अ~\u{947}वहि", "अ~\u{947}महि";
        optative_middle_second: "अ~\u{947}थाः", "अ~\u{947}याथाम\u{94d}", "अ~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "अ~\u{947}त", "अ~\u{947}याताम\u{94d}", "अ~\u{947}रन\u{94d}";
        subjunctive_active_first: "अ~ानि", "अ~ाव", "अ~ाम";
        subjunctive_active_second: "अ~ाः|अ~ासि", "अ~ाथः", "अ~ाथ";
        subjunctive_active_third: "अ~ात\u{94d}|अ~ाति", "अ~ातः", "अ~ान\u{94d}";
        subjunctive_middle_first: "अ~\u{948}", "अ~ावह\u{948}", "अ~ामह\u{948}";
        subjunctive_middle_second: "अ~ास\u{947}|अ~ास\u{948}", "अ~\u{948}थ\u{947}", "अ~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "अ~ात\u{947}|अ~ात\u{948}", "अ~\u{948}त\u{947}", "अ~न\u{94d}त|अ~ान\u{94d}त\u{948}";
        imperfect_active_first: "आ~म\u{94d}", "आ~ाव", "आ~ाम";
        imperfect_active_second: "आ~ः", "आ~तम\u{94d}", "आ~त";
        imperfect_active_third: "आ~त\u{94d}", "आ~ताम\u{94d}", "आ~न\u{94d}";
        imperfect_middle_first: "आ~\u{947}", "आ~ावहि", "आ~ामहि";
        imperfect_middle_second: "आ~थाः", "आ~\u{947}थाम\u{94d}", "आ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "आ~त", "आ~\u{947}ताम\u{94d}", "आ~न\u{94d}त";
        future_active_first: "अ~िष\u{94d}यामि", "अ~िष\u{94d}यावः", "अ~िष\u{94d}यामः";
        future_active_second: "अ~िष\u{94d}यसि", "अ~िष\u{94d}यथः", "अ~िष\u{94d}यथ";
        future_active_third: "अ~िष\u{94d}यति", "अ~िष\u{94d}यतः", "अ~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावह\u{947}", "अ~िष\u{94d}यामह\u{947}";
        future_middle_second: "अ~िष\u{94d}यस\u{947}", "अ~िष\u{94d}य\u{947}थ\u{947}", "अ~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "अ~िष\u{94d}यत\u{947}", "अ~िष\u{94d}य\u{947}त\u{947}", "अ~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "आ~िष\u{94d}यम\u{94d}", "आ~िष\u{94d}याव", "आ~िष\u{94d}याम";
        conditional_active_second: "आ~िष\u{94d}यः", "आ~िष\u{94d}यतम\u{94d}", "आ~िष\u{94d}यत";
        conditional_active_third: "आ~िष\u{94d}यत\u{94d}", "आ~िष\u{94d}यताम\u{94d}", "आ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "आ~िष\u{94d}य\u{947}", "आ~िष\u{94d}यावहि", "आ~िष\u{94d}यामहि";
        conditional_middle_second: "आ~िष\u{94d}यथाः", "आ~िष\u{94d}य\u{947}थाम\u{94d}", "आ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "आ~िष\u{94d}यत", "आ~िष\u{94d}य\u{947}ताम\u{94d}", "आ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "आ~", "आ~िव", "आ~िम";
        perfect_active_second: "आ~िथ", "आ~थ\u{941}ः", "आ~";
        perfect_active_third: "आ~", "आ~त\u{941}ः", "आ~\u{941}ः";
        perfect_middle_first: "आ~\u{947}", "आ~िवह\u{947}", "आ~िमह\u{947}";
        perfect_middle_second: "आ~िष\u{947}", "आ~ाथ\u{947}", "आ~िध\u{94d}व\u{947}";
        perfect_middle_third: "आ~\u{947}", "आ~ात\u{947}", "आ~िर\u{947}";
        aorist_active_first: "आ~िषम\u{94d}", "आ~िष\u{94d}व", "आ~िष\u{94d}म";
        aorist_active_second: "आ~ीः", "आ~िष\u{94d}टम\u{94d}", "आ~िष\u{94d}ट";
        aorist_active_third: "आ~ीत\u{94d}", "आ~िष\u{94d}टाम\u{94d}", "आ~िष\u{941}ः";
        aorist_middle_first: "आ~िषि", "आ~िष\u{94d}वहि", "आ~िष\u{94d}महि";
        aorist_middle_second: "आ~िष\u{94d}ठाः", "आ~िषाथाम\u{94d}", "आ~िढ\u{94d}वम\u{94d}";
        aorist_middle_third: "आ~िष\u{94d}ट", "आ~िषाताम\u{94d}", "आ~िषत";
        infinitive: "अ~ित\u{941}म\u{94d}";
        gerund: "अ~ित\u{94d}वा";
        participle_present: "अ~त\u{94d}", "अ~मान";
        participle_future: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यमाण";
        participle_perfect: "आ~िवा\u{902}स\u{94d}", "आ~ान";
    ),
    // आजीत: 11 words (अस्वारीत्, आजीत्, अकाषीत्, आञ्चीत्, अलेखीत्, अजिवीत्).
    conjugation!("आजीत\u{94d}", 11, "~ीत\u{94d}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~िषम\u{94d}", "~िष\u{94d}व", "~िष\u{94d}म";
        aorist_active_second: "~ीः", "~िष\u{94d}टम\u{94d}", "~िष\u{94d}ट";
        aorist_active_third: "~ीत\u{94d}", "~िष\u{94d}टाम\u{94d}", "~िष\u{941}ः";
        aorist_middle_first: "~िषि", "~िष\u{94d}वहि", "~िष\u{94d}महि";
        aorist_middle_second: "~िष\u{94d}ठाः", "~िषाथाम\u{94d}", "~िढ\u{94d}वम\u{94d}";
        aorist_middle_third: "~िष\u{94d}ट", "~िषाताम\u{94d}", "~िषत";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // भाति: 10 words (भाति, द्राति, वाति, पाति, स्नाति, याति).
    conjugation!("भाति", 10, "~ाति";
        indicative_active_first: "~ामि", "~ावः", "~ामः|~ामसि";
        indicative_active_second: "~ासि", "~ाथः", "~ाथ";
        indicative_active_third: "~ाति", "~ातः", "~ान\u{94d}ति";
        indicative_middle_first: "~\u{948}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~ास\u{947}", "~ाथ\u{947}", "~ाध\u{94d}व\u{947}";
        indicative_middle_third: "~ात\u{947}", "~ात\u{947}", "~ात\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~ाहि", "~ातम\u{94d}", "~ात";
        imperative_active_third: "~ात\u{941}", "~ाताम\u{94d}", "~ान\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~ास\u{94d}व", "~ाथाम\u{94d}", "~ाध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ाताम\u{94d}", "~ाताम\u{94d}", "~ाताम\u{94d}";
        optative_active_first: "~ायाम\u{94d}", "~ायाव", "~ायाम";
        optative_active_second: "~ायाः", "~ायातम\u{94d}", "~ायात";
        optative_active_third: "~ायात\u{94d}", "~ायाताम\u{94d}", "~ाय\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि|~ा", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः|~ासि", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}|~ाति", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{947}|~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~ान\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "अ~ाम\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ाः", "अ~ातम\u{94d}", "अ~ात";
        imperfect_active_third: "अ~ात\u{94d}", "अ~ाताम\u{94d}", "अ~\u{941}ः|अ~ान\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~ाथाः", "अ~ाथाम\u{94d}", "अ~ाध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~ात", "अ~ाताम\u{94d}", "अ~ात";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~ात\u{94d}", "~ान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // धासयति: 10 words (पास्यति, तप्स्यति, ज्ञास्यति, नोत्स्यति, सत्स्यति, वत्स्यति).
    conjugation!("धास\u{94d}यति", 10, "~ति";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~ामि", "~ावः", "~ामः|~ामसि";
        future_active_second: "~सि", "~थः", "~थ";
        future_active_third: "~ति", "~तः", "~न\u{94d}ति";
        future_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        future_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        future_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        conditional_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        conditional_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        conditional_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        conditional_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        conditional_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~त\u{94d}", "~मान";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // अनंसीत: 9 words (अताप्सीत्, अहासीत्, अज्ञासीत्, अनौत्सीत्, अरौत्सीत्, अवात्सीत्).
    conjugation!("अन\u{902}सीत\u{94d}", 9, "~सीत\u{94d}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~सम\u{94d}", "~स\u{94d}व", "~स\u{94d}म";
        aorist_active_second: "~सीः", "~स\u{94d}तम\u{94d}", "~स\u{94d}त";
        aorist_active_third: "~सीत\u{94d}", "~स\u{94d}ताम\u{94d}", "~स\u{941}ः";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // आपनोति: 8 words (धृष्णोति, शक्नोति, साध्नोति, स्तिघ्नोति, प्रुष्णोति, आप्नोति).
    conjugation!("आप\u{94d}नोति", 8, "~ोति";
        indicative_active_first: "~ोमि", "~\u{941}वः", "~\u{941}मः";
        indicative_active_second: "~ोषि", "~\u{941}थः", "~\u{941}थ";
        indicative_active_third: "~ोति", "~\u{941}तः", "~\u{941}वन\u{94d}ति";
        indicative_middle_first: "~\u{941}व\u{947}", "~\u{941}वह\u{947}", "~\u{941}मह\u{947}";
        indicative_middle_second: "~\u{941}ष\u{947}", "~\u{941}वाथ\u{947}", "~\u{941}ध\u{94d}व\u{947}";
        indicative_middle_third: "~\u{941}त\u{947}", "~\u{941}वात\u{947}", "~\u{941}वत\u{947}";
        imperative_active_first: "~वानि", "~वाव", "~वाम";
        imperative_active_second: "~\u{941}हि", "~\u{941}तम\u{94d}", "~\u{941}त";
        imperative_active_third: "~ोत\u{941}", "~\u{941}ताम\u{94d}", "~\u{941}वन\u{94d}त\u{941}";
        imperative_middle_first: "~व\u{948}", "~वावह\u{948}", "~वामह\u{948}";
        imperative_middle_second: "~\u{941}ष\u{94d}व", "~\u{941}वाथाम\u{94d}", "~\u{941}ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~\u{941}ताम\u{94d}", "~\u{941}वाताम\u{94d}", "~\u{941}वताम\u{94d}";
        optative_active_first: "~\u{941}याम\u{94d}", "~\u{941}याव", "~\u{941}याम";
        optative_active_second: "~\u{941}याः", "~\u{941}यातम\u{94d}", "~\u{941}यात";
        optative_active_third: "~\u{941}यात\u{94d}", "~\u{941}याताम\u{94d}", "~\u{941}य\u{941}ः";
        optative_middle_first: "~\u{941}वीय", "~\u{941}वीवहि", "~\u{941}वीमहि";
        optative_middle_second: "~\u{941}वीथाः", "~\u{941}वीयाथाम\u{94d}", "~\u{941}वीध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{941}वीत", "~\u{941}वीयाताम\u{94d}", "~\u{941}वीरन\u{94d}";
        subjunctive_active_first: "~वानि", "~वाव", "~वाम";
        subjunctive_active_second: "~वः", "~वथः", "~वथ";
        subjunctive_active_third: "~वत\u{94d}", "~वतः", "~वन\u{94d}";
        subjunctive_middle_first: "~व\u{948}", "~वावह\u{948}", "~वामह\u{948}";
        subjunctive_middle_second: "~वस\u{947}", "~व\u{948}थ\u{947}", "~वध\u{94d}व\u{947}";
        subjunctive_middle_third: "~वत\u{947}", "~व\u{948}त\u{947}", "~वन\u{94d}त";
        imperfect_active_first: "अ~वम\u{94d}", "अ~\u{941}व", "अ~\u{941}म";
        imperfect_active_second: "अ~ोः", "अ~\u{941}तम\u{94d}", "अ~\u{941}त";
        imperfect_active_third: "अ~ोत\u{94d}", "अ~\u{941}ताम\u{94d}", "अ~\u{941}वन\u{94d}";
        imperfect_middle_first: "अ~\u{941}वि", "अ~\u{941}वहि", "अ~\u{941}महि";
        imperfect_middle_second: "अ~\u{941}थाः", "अ~\u{941}वाथाम\u{94d}", "अ~\u{941}ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~\u{941}त", "अ~\u{941}वाताम\u{94d}", "अ~\u{941}वत";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~\u{941}वत\u{94d}", "~\u{941}वान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // बधनाति: 7 words (गृभ्णाति, उभ्नाति, गृह्णाति, इष्णाति, बध्नाति, स्तभ्नाति).
    conjugation!("बध\u{94d}नाति", 7, "~\u{94d}~ाति";
        indicative_active_first: "~\u{94d}~ामि", "~\u{94d}~ीवः", "~\u{94d}~ीमः|~\u{94d}~ीमसि";
        indicative_active_second: "~\u{94d}~ासि", "~\u{94d}~ीथः", "~\u{94d}~ीथ";
        indicative_active_third: "~\u{94d}~ाति", "~\u{94d}~ीतः", "~\u{94d}~न\u{94d}ति";
        indicative_middle_first: "~\u{94d}~\u{947}", "~\u{94d}~ीवह\u{947}", "~\u{94d}~ीमह\u{947}";
        indicative_middle_second: "~\u{94d}~ीष\u{947}", "~\u{94d}~ाथ\u{947}", "~\u{94d}~ीध\u{94d}व\u{947}";
        indicative_middle_third: "~\u{94d}~ीत\u{947}", "~\u{94d}~ात\u{947}", "~\u{94d}~त\u{947}";
        imperative_active_first: "~\u{94d}~ानि", "~\u{94d}~ाव", "~\u{94d}~ाम";
        imperative_active_second: "~ा~", "~\u{94d}~ीतम\u{94d}", "~\u{94d}~ीत";
        imperative_active_third: "~\u{94d}~ात\u{941}", "~\u{94d}~ीताम\u{94d}", "~\u{94d}~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{94d}~\u{948}", "~\u{94d}~ावह\u{948}", "~\u{94d}~ामह\u{948}";
        imperative_middle_second: "~\u{94d}~ीष\u{94d}व", "~\u{94d}~ाथाम\u{94d}", "~\u{94d}~ीध\u{94d}वम\u{94d}";
        imperative_middle_third: "~\u{94d}~ीताम\u{94d}", "~\u{94d}~ाताम\u{94d}", "~\u{94d}~ताम\u{94d}";
        optative_active_first: "~\u{94d}~ीयाम\u{94d}", "~\u{94d}~ीयाव", "~\u{94d}~ीयाम";
        optative_active_second: "~\u{94d}~ीयाः", "~\u{94d}~ीयातम\u{94d}", "~\u{94d}~ीयात";
        optative_active_third: "~\u{94d}~ीयात\u{94d}", "~\u{94d}~ीयाताम\u{94d}", "~\u{94d}~ीय\u{941}ः";
        optative_middle_first: "~\u{94d}~ीय", "~\u{94d}~ीवहि", "~\u{94d}~ीमहि";
        optative_middle_second: "~\u{94d}~ीथाः", "~\u{94d}~ीयाथाम\u{94d}", "~ा~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{94d}~ीत", "~\u{94d}~ीयाताम\u{94d}", "~\u{94d}~ीरन\u{94d}";
        subjunctive_active_first: "~\u{94d}~ानि|~\u{94d}~ा", "~\u{94d}~ाव", "~\u{94d}~ाम";
        subjunctive_active_second: "~\u{94d}~ाः|~\u{94d}~ासि", "~\u{94d}~ाथः", "~\u{94d}~ाथ";
        subjunctive_active_third: "~\u{94d}~ात\u{94d}|~\u{94d}~ाति", "~\u{94d}~ातः", "~\u{94d}~ान\u{94d}";
        subjunctive_middle_first: "~\u{94d}~\u{948}", "~\u{94d}~ावह\u{948}", "~\u{94d}~ामह\u{948}";
        subjunctive_middle_second: "~\u{94d}~ास\u{947}|~\u{94d}~ास\u{948}", "~\u{94d}~\u{948}थ\u{947}", "~\u{94d}~ाध\u{94d}व\u{947}|~\u{94d}~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~\u{94d}~ात\u{947}|~\u{94d}~ात\u{948}", "~\u{94d}~\u{948}त\u{947}", "~\u{94d}~ान\u{94d}त|~\u{94d}~ान\u{94d}त\u{948}";
        imperfect_active_first: "अ~\u{94d}~ाम\u{94d}", "अ~\u{94d}~ीव", "अ~\u{94d}~ीम";
        imperfect_active_second: "अ~\u{94d}~ाः", "अ~\u{94d}~ीतम\u{94d}", "अ~\u{94d}~ीत";
        imperfect_active_third: "अ~\u{94d}~ात\u{94d}", "अ~\u{94d}~ीताम\u{94d}", "अ~\u{94d}~न\u{94d}";
        imperfect_middle_first: "अ~\u{94d}~ि", "अ~\u{94d}~ीवहि", "अ~\u{94d}~ीमहि";
        imperfect_middle_second: "अ~\u{94d}~ीथाः", "अ~\u{94d}~ाथाम\u{94d}", "अ~\u{94d}~ीध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~\u{94d}~ीत", "अ~\u{94d}~ाताम\u{94d}", "अ~\u{94d}~त";
        future_active_first: "~ा~िष\u{94d}यामि", "~ा~िष\u{94d}यावः", "~ा~िष\u{94d}यामः";
        future_active_second: "~ा~िष\u{94d}यसि", "~ा~िष\u{94d}यथः", "~ा~िष\u{94d}यथ";
        future_active_third: "~ा~िष\u{94d}यति", "~ा~िष\u{94d}यतः", "~ा~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~ा~िष\u{94d}य\u{947}", "~ा~िष\u{94d}यावह\u{947}", "~ा~िष\u{94d}यामह\u{947}";
        future_middle_second: "~ा~िष\u{94d}यस\u{947}", "~ा~िष\u{94d}य\u{947}थ\u{947}", "~ा~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~ा~िष\u{94d}यत\u{947}", "~ा~िष\u{94d}य\u{947}त\u{947}", "~ा~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~ा~िष\u{94d}यम\u{94d}", "अ~ा~िष\u{94d}याव", "अ~ा~िष\u{94d}याम";
        conditional_active_second: "अ~ा~िष\u{94d}यः", "अ~ा~िष\u{94d}यतम\u{94d}", "अ~ा~िष\u{94d}यत";
        conditional_active_third: "अ~ा~िष\u{94d}यत\u{94d}", "अ~ा~िष\u{94d}यताम\u{94d}", "अ~ा~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~ा~िष\u{94d}य\u{947}", "अ~ा~िष\u{94d}यावहि", "अ~ा~िष\u{94d}यामहि";
        conditional_middle_second: "अ~ा~िष\u{94d}यथाः", "अ~ा~िष\u{94d}य\u{947}थाम\u{94d}", "अ~ा~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~ा~िष\u{94d}यत", "अ~ा~िष\u{94d}य\u{947}ताम\u{94d}", "अ~ा~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "च~ा~", "च~ा~िव", "च~ा~िम";
        perfect_active_second: "च~ा~िथ", "च~ा~थ\u{941}ः", "च~ा~";
        perfect_active_third: "च~ा~", "च~ा~त\u{941}ः", "च~ा~\u{941}ः";
        perfect_middle_first: "च~ा~\u{947}", "च~ा~िवह\u{947}", "च~ा~िमह\u{947}";
        perfect_middle_second: "च~ा~िष\u{947}", "च~ा~ाथ\u{947}", "च~ा~िध\u{94d}व\u{947}";
        perfect_middle_third: "च~ा~\u{947}", "च~ा~ात\u{947}", "च~ा~िर\u{947}";
        aorist_active_first: "अ~ा~िषम\u{94d}", "अ~ा~िष\u{94d}व", "अ~ा~िष\u{94d}म";
        aorist_active_second: "अ~ा~ीः", "अ~ा~िष\u{94d}टम\u{94d}", "अ~ा~िष\u{94d}ट";
        aorist_active_third: "अ~ा~ीत\u{94d}", "अ~ा~िष\u{94d}टाम\u{94d}", "अ~ा~िष\u{941}ः";
        aorist_middle_first: "अ~ा~िषि", "अ~ा~िष\u{94d}वहि", "अ~ा~िष\u{94d}महि";
        aorist_middle_second: "अ~ा~िष\u{94d}ठाः", "अ~ा~िषाथाम\u{94d}", "अ~ा~िढ\u{94d}वम\u{94d}";
        aorist_middle_third: "अ~ा~िष\u{94d}ट", "अ~ा~िषाताम\u{94d}", "अ~ा~िषत";
        infinitive: "~ा~ित\u{941}म\u{94d}";
        gerund: "~~ित\u{94d}वा";
        participle_present: "~\u{94d}~त\u{94d}", "~\u{94d}~ान";
        participle_future: "~ा~िष\u{94d}यत\u{94d}", "~ा~िष\u{94d}यमाण";
        participle_perfect: "च~ा~\u{94d}वा\u{902}स\u{94d}", "च~ा~ान";
    ),
    // ददौ: 7 words (तस्थौ, जज्ञौ, ददौ, पपौ, दधौ, ररौ).
    conjugation!("ददौ", 7, "~ौ";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ौ", "~िव", "~िम";
        perfect_active_second: "~ाथ|~िथ", "~थ\u{941}ः", "~";
        perfect_active_third: "~ौ", "~त\u{941}ः", "~\u{941}ः";
        perfect_middle_first: "~\u{947}", "~िवह\u{947}", "~िमह\u{947}";
        perfect_middle_second: "~िष\u{947}", "~ाथ\u{947}", "~िध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{947}", "~ात\u{947}", "~िर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~िवा\u{902}स\u{94d}", "~ान";
    ),
    // पिपरति: 7 words (पिपर्ति, जागर्ति, बिभर्ति, जिघर्ति, इयर्ति, सिसर्ति).
    conjugation!("पिपर\u{94d}ति", 7, "~र\u{94d}ति";
        indicative_active_first: "~र\u{94d}मि", "~\u{943}वः", "~\u{943}मः|~\u{943}मसि";
        indicative_active_second: "~र\u{94d}षि", "~\u{943}थः", "~\u{943}थ";
        indicative_active_third: "~र\u{94d}ति", "~\u{943}तः", "~\u{94d}रति";
        indicative_middle_first: "~\u{94d}र\u{947}", "~\u{943}वह\u{947}", "~\u{943}मह\u{947}";
        indicative_middle_second: "~\u{943}ष\u{947}", "~\u{94d}राथ\u{947}", "~\u{943}ध\u{94d}व\u{947}";
        indicative_middle_third: "~\u{943}त\u{947}", "~\u{94d}रात\u{947}", "~\u{94d}रत\u{947}";
        imperative_active_first: "~राणि", "~राव", "~राम";
        imperative_active_second: "~\u{943}हि", "~\u{943}तम\u{94d}", "~\u{943}त";
        imperative_active_third: "~र\u{94d}त\u{941}", "~\u{943}ताम\u{94d}", "~\u{94d}रत\u{941}";
        imperative_middle_first: "~र\u{948}", "~रावह\u{948}", "~रामह\u{948}";
        imperative_middle_second: "~\u{943}ष\u{94d}व", "~\u{94d}राथाम\u{94d}", "~\u{943}ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~\u{943}ताम\u{94d}", "~\u{94d}राताम\u{94d}", "~\u{94d}रताम\u{94d}";
        optative_active_first: "~\u{943}याम\u{94d}", "~\u{943}याव", "~\u{943}याम";
        optative_active_second: "~\u{943}याः", "~\u{943}यातम\u{94d}", "~\u{943}यात";
        optative_active_third: "~\u{943}यात\u{94d}", "~\u{943}याताम\u{94d}", "~\u{943}य\u{941}ः";
        optative_middle_first: "~\u{94d}रीय", "~\u{94d}रीवहि", "~\u{94d}रीमहि";
        optative_middle_second: "~\u{94d}रीथाः", "~\u{94d}रीयाथाम\u{94d}", "~\u{94d}रीध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{94d}रीत", "~\u{94d}रीयाताम\u{94d}", "~\u{94d}रीरन\u{94d}";
        subjunctive_active_first: "~राणि|~रा", "~राव", "~राम";
        subjunctive_active_second: "~रः|~रसि", "~रथः", "~रथ";
        subjunctive_active_third: "~रत\u{94d}|~रति", "~रतः", "~रन\u{94d}";
        subjunctive_middle_first: "~र\u{948}", "~रावह\u{948}", "~रामह\u{948}";
        subjunctive_middle_second: "~रस\u{947}", "~र\u{948}थ\u{947}", "~रध\u{94d}व\u{947}";
        subjunctive_middle_third: "~रत\u{947}", "~र\u{948}त\u{947}", "~रन\u{94d}त";
        imperfect_active_first: "अ~रम\u{94d}", "अ~\u{943}व", "अ~\u{943}म";
        imperfect_active_second: "अ~ः", "अ~\u{943}तम\u{94d}", "अ~\u{943}त";
        imperfect_active_third: "अ~ः", "अ~\u{943}ताम\u{94d}", "अ~र\u{941}ः";
        imperfect_middle_first: "अ~\u{94d}रि", "अ~\u{943}वहि", "अ~\u{943}महि";
        imperfect_middle_second: "अ~\u{943}थाः", "अ~\u{94d}राथाम\u{94d}", "अ~\u{943}ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~\u{943}त", "अ~\u{94d}राताम\u{94d}", "अ~\u{94d}रत";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~\u{94d}रत\u{94d}", "~\u{94d}राण";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // भनकति: 6 words (भनक्ति, युनक्ति, विनक्ति, तनक्ति, अनक्ति, भुनक्ति).
    conjugation!("भनक\u{94d}ति", 6, "~नक\u{94d}ति";
        indicative_active_first: "~नज\u{94d}मि", "~ञ\u{94d}ज\u{94d}वः", "~ञ\u{94d}ज\u{94d}मः";
        indicative_active_second: "~नक\u{94d}षि", "~ङ\u{94d}क\u{94d}थः", "~ङ\u{94d}क\u{94d}थ";
        indicative_active_third: "~नक\u{94d}ति", "~ङ\u{94d}क\u{94d}तः", "~ञ\u{94d}जन\u{94d}ति";
        indicative_middle_first: "~ञ\u{94d}ज\u{947}", "~ञ\u{94d}ज\u{94d}वह\u{947}", "~ञ\u{94d}ज\u{94d}मह\u{947}";
        indicative_middle_second: "~ङ\u{94d}क\u{94d}ष\u{947}", "~ञ\u{94d}जाथ\u{947}", "~ङ\u{94d}ग\u{94d}ध\u{94d}व\u{947}";
        indicative_middle_third: "~ङ\u{94d}क\u{94d}त\u{947}", "~ञ\u{94d}जात\u{947}", "~ञ\u{94d}जत\u{947}";
        imperative_active_first: "~नजानि", "~नजाव", "~नजाम";
        imperative_active_second: "~ङ\u{94d}ग\u{94d}धि", "~ङ\u{94d}क\u{94d}तम\u{94d}", "~ङ\u{94d}क\u{94d}त";
        imperative_active_third: "~नक\u{94d}त\u{941}", "~ङ\u{94d}क\u{94d}ताम\u{94d}", "~ञ\u{94d}जन\u{94d}त\u{941}";
        imperative_middle_first: "~नज\u{948}", "~नजावह\u{948}", "~नजामह\u{948}";
        imperative_middle_second: "~ङ\u{94d}क\u{94d}ष\u{94d}व", "~ञ\u{94d}जाथाम\u{94d}", "~ङ\u{94d}ग\u{94d}ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ङ\u{94d}क\u{94d}ताम\u{94d}", "~ञ\u{94d}जाताम\u{94d}", "~ञ\u{94d}जताम\u{94d}";
        optative_active_first: "~ञ\u{94d}ज\u{94d}याम\u{94d}", "~ञ\u{94d}ज\u{94d}याव", "~ञ\u{94d}ज\u{94d}याम";
        optative_active_second: "~ञ\u{94d}ज\u{94d}याः", "~ञ\u{94d}ज\u{94d}यातम\u{94d}", "~ञ\u{94d}ज\u{94d}यात";
        optative_active_third: "~ञ\u{94d}ज\u{94d}यात\u{94d}", "~ञ\u{94d}ज\u{94d}याताम\u{94d}", "~ञ\u{94d}ज\u{94d}य\u{941}ः";
        optative_middle_first: "~ञ\u{94d}जीय", "~ञ\u{94d}जीवहि", "~ञ\u{94d}जीमहि";
        optative_middle_second: "~ञ\u{94d}जीथाः", "~ञ\u{94d}जीयाथाम\u{94d}", "~ञ\u{94d}जीध\u{94d}वम\u{94d}";
        optative_middle_third: "~ञ\u{94d}जीत", "~ञ\u{94d}जीयाताम\u{94d}", "~ञ\u{94d}जीरन\u{94d}";
        subjunctive_active_first: "~नजानि", "~नजाव", "~नजाम";
        subjunctive_active_second: "~नजः", "~नजथः", "~नजथ";
        subjunctive_active_third: "~नजत\u{94d}", "~नजतः", "~नजन\u{94d}";
        subjunctive_middle_first: "~नज\u{948}", "~नजावह\u{948}", "~नजामह\u{948}";
        subjunctive_middle_second: "~नजस\u{947}", "~नज\u{948}थ\u{947}", "~नजध\u{94d}व\u{947}";
        subjunctive_middle_third: "~नजत\u{947}", "~नज\u{948}त\u{947}", "~नजन\u{94d}त";
        imperfect_active_first: "अ~नजम\u{94d}", "अ~ञ\u{94d}ज\u{94d}व", "अ~ञ\u{94d}ज\u{94d}म";
        imperfect_active_second: "अ~नक\u{94d}", "अ~ङ\u{94d}क\u{94d}तम\u{94d}", "अ~ङ\u{94d}क\u{94d}त";
        imperfect_active_third: "अ~नक\u{94d}", "अ~ङ\u{94d}क\u{94d}ताम\u{94d}", "अ~ञ\u{94d}जन\u{94d}";
        imperfect_middle_first: "अ~ञ\u{94d}जि", "अ~ञ\u{94d}ज\u{94d}वहि", "अ~ञ\u{94d}ज\u{94d}महि";
        imperfect_middle_second: "अ~ङ\u{94d}क\u{94d}थाः", "अ~ञ\u{94d}जाथाम\u{94d}", "अ~ङ\u{94d}ग\u{94d}ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~ङ\u{94d}क\u{94d}त", "अ~ञ\u{94d}जाताम\u{94d}", "अ~ञ\u{94d}जत";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~ञ\u{94d}जत\u{94d}", "~ञ\u{94d}जान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // उपविशति: 6 words (उच्यति, उज्झति, उच्छति, उपविशति, उक्षति, उड्डापयति).
    conjugation!("उपविशति", 6, "उ~ति";
        indicative_active_first: "उ~ामि", "उ~ावः", "उ~ामः|उ~ामसि";
        indicative_active_second: "उ~सि", "उ~थः", "उ~थ";
        indicative_active_third: "उ~ति", "उ~तः", "उ~न\u{94d}ति";
        indicative_middle_first: "उ~\u{947}", "उ~ावह\u{947}", "उ~ामह\u{947}";
        indicative_middle_second: "उ~स\u{947}", "उ~\u{947}थ\u{947}", "उ~ध\u{94d}व\u{947}";
        indicative_middle_third: "उ~त\u{947}", "उ~\u{947}त\u{947}", "उ~न\u{94d}त\u{947}";
        imperative_active_first: "उ~ानि", "उ~ाव", "उ~ाम";
        imperative_active_second: "उ~", "उ~तम\u{94d}", "उ~त";
        imperative_active_third: "उ~त\u{941}", "उ~ताम\u{94d}", "उ~न\u{94d}त\u{941}";
        imperative_middle_first: "उ~\u{948}", "उ~ावह\u{948}", "उ~ामह\u{948}";
        imperative_middle_second: "उ~स\u{94d}व", "उ~\u{947}थाम\u{94d}", "उ~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "उ~ताम\u{94d}", "उ~\u{947}ताम\u{94d}", "उ~न\u{94d}ताम\u{94d}";
        optative_active_first: "उ~\u{947}यम\u{94d}", "उ~\u{947}व", "उ~\u{947}म";
        optative_active_second: "उ~\u{947}ः", "उ~\u{947}तम\u{94d}", "उ~\u{947}त";
        optative_active_third: "उ~\u{947}त\u{94d}", "उ~\u{947}ताम\u{94d}", "उ~\u{947}य\u{941}ः";
        optative_middle_first: "उ~\u{947}य", "उ~\u{947}वहि", "उ~\u{947}महि";
        optative_middle_second: "उ~\u{947}थाः", "उ~\u{947}याथाम\u{94d}", "उ~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "उ~\u{947}त", "उ~\u{947}याताम\u{94d}", "उ~\u{947}रन\u{94d}";
        subjunctive_active_first: "उ~ानि", "उ~ाव", "उ~ाम";
        subjunctive_active_second: "उ~ाः|उ~ासि", "उ~ाथः", "उ~ाथ";
        subjunctive_active_third: "उ~ात\u{94d}|उ~ाति", "उ~ातः", "उ~ान\u{94d}";
        subjunctive_middle_first: "उ~\u{948}", "उ~ावह\u{948}", "उ~ामह\u{948}";
        subjunctive_middle_second: "उ~ास\u{947}|उ~ास\u{948}", "उ~\u{948}थ\u{947}", "उ~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "उ~ात\u{947}|उ~ात\u{948}", "उ~\u{948}त\u{947}", "उ~न\u{94d}त|उ~ान\u{94d}त\u{948}";
        imperfect_active_first: "औ~म\u{94d}", "औ~ाव", "औ~ाम";
        imperfect_active_second: "औ~ः", "औ~तम\u{94d}", "औ~त";
        imperfect_active_third: "औ~त\u{94d}", "औ~ताम\u{94d}", "औ~न\u{94d}";
        imperfect_middle_first: "औ~\u{947}", "औ~ावहि", "औ~ामहि";
        imperfect_middle_second: "औ~थाः", "औ~\u{947}थाम\u{94d}", "औ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "औ~त", "औ~\u{947}ताम\u{94d}", "औ~न\u{94d}त";
        future_active_first: "उ~िष\u{94d}यामि", "उ~िष\u{94d}यावः", "उ~िष\u{94d}यामः";
        future_active_second: "उ~िष\u{94d}यसि", "उ~िष\u{94d}यथः", "उ~िष\u{94d}यथ";
        future_active_third: "उ~िष\u{94d}यति", "उ~िष\u{94d}यतः", "उ~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "उ~िष\u{94d}य\u{947}", "उ~िष\u{94d}यावह\u{947}", "उ~िष\u{94d}यामह\u{947}";
        future_middle_second: "उ~िष\u{94d}यस\u{947}", "उ~िष\u{94d}य\u{947}थ\u{947}", "उ~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "उ~िष\u{94d}यत\u{947}", "उ~िष\u{94d}य\u{947}त\u{947}", "उ~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "औ~िष\u{94d}यम\u{94d}", "औ~िष\u{94d}याव", "औ~िष\u{94d}याम";
        conditional_active_second: "औ~िष\u{94d}यः", "औ~िष\u{94d}यतम\u{94d}", "औ~िष\u{94d}यत";
        conditional_active_third: "औ~िष\u{94d}यत\u{94d}", "औ~िष\u{94d}यताम\u{94d}", "औ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "औ~िष\u{94d}य\u{947}", "औ~िष\u{94d}यावहि", "औ~िष\u{94d}यामहि";
        conditional_middle_second: "औ~िष\u{94d}यथाः", "औ~िष\u{94d}य\u{947}थाम\u{94d}", "औ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "औ~िष\u{94d}यत", "औ~िष\u{94d}य\u{947}ताम\u{94d}", "औ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "उ~ामास", "उ~ामासिव", "उ~ामासिम";
        perfect_active_second: "उ~ामासिथ", "उ~ामासथ\u{941}ः", "उ~ामास";
        perfect_active_third: "उ~ामास", "उ~ामासत\u{941}ः", "उ~ामास\u{941}ः";
        perfect_middle_first: "उ~ा\u{902}चक\u{94d}र\u{947}", "उ~ा\u{902}चक\u{943}वह\u{947}", "उ~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "उ~ा\u{902}चक\u{943}ष\u{947}", "उ~ा\u{902}चक\u{94d}राथ\u{947}", "उ~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "उ~ा\u{902}चक\u{94d}र\u{947}", "उ~ा\u{902}चक\u{94d}रात\u{947}", "उ~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "उ~ित\u{941}म\u{94d}";
        gerund: "उ~ित\u{94d}वा";
        participle_present: "उ~त\u{94d}", "उ~मान";
        participle_future: "उ~िष\u{94d}यत\u{94d}", "उ~िष\u{94d}यमाण";
        participle_perfect: "उ~ामासिवा\u{902}स\u{94d}", "उ~ा\u{902}चक\u{94d}राण";
    ),
    // ईळे: 5 words (ईळे, जज्ञे, शश्ये, पस्पशे, चके).
    conjugation!("ईळ\u{947}", 5, "~\u{947}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~", "~िव", "~िम";
        perfect_active_second: "~िथ", "~थ\u{941}ः", "~";
        perfect_active_third: "~", "~त\u{941}ः", "~\u{941}ः";
        perfect_middle_first: "~\u{947}", "~िवह\u{947}", "~िमह\u{947}";
        perfect_middle_second: "~िष\u{947}", "~ाथ\u{947}", "~िध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{947}", "~ात\u{947}", "~िर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~\u{94d}वा\u{902}स\u{94d}", "~ान";
    ),
    // अगात: 5 words (अधात्, अस्थात्, अदात्, अगात्, अपात्).
    conjugation!("अगात\u{94d}", 5, "~ात\u{94d}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~ाम\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ाः", "~ातम\u{94d}", "~ात";
        aorist_active_third: "~ात\u{94d}", "~ाताम\u{94d}", "~\u{941}ः";
        aorist_middle_first: "~ि", "~िवहि", "~िमहि";
        aorist_middle_second: "~िथाः", "~ाथाम\u{94d}", "~िध\u{94d}वम\u{94d}";
        aorist_middle_third: "~ित", "~ाताम\u{94d}", "~िरन\u{94d}";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // जुहाव: 5 words (दुद्राव, शुश्राव, तुष्टाव, सुस्राव, जुहाव).
    conjugation!("ज\u{941}हाव", 5, "~ाव";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~व|~ाव", "~\u{941}व", "~\u{941}म";
        perfect_active_second: "~ोथ", "~\u{941}वथ\u{941}ः", "~\u{941}व";
        perfect_active_third: "~ाव", "~\u{941}वत\u{941}ः", "~\u{941}व\u{941}ः";
        perfect_middle_first: "~\u{941}व\u{947}", "~\u{941}वह\u{947}", "~\u{941}मह\u{947}";
        perfect_middle_second: "~\u{941}ष\u{947}", "~\u{941}वाथ\u{947}", "~\u{941}ध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{941}व\u{947}", "~\u{941}वात\u{947}", "~\u{941}विर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~\u{941}वा\u{902}स\u{94d}", "~\u{941}वाण";
    ),
    // इयाय: 5 words (बिभाय, जिगाय, इयाय, जिघाय, निनाय).
    conjugation!("इयाय", 5, "~ाय";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~य|~ाय", "~\u{94d}यिव", "~\u{94d}यिम";
        perfect_active_second: "~\u{947}थ|~यिथ", "~\u{94d}यथ\u{941}ः", "~\u{94d}य";
        perfect_active_third: "~ाय", "~\u{94d}यत\u{941}ः", "~\u{94d}य\u{941}ः";
        perfect_middle_first: "~\u{94d}य\u{947}", "~\u{94d}यिवह\u{947}", "~\u{94d}यिमह\u{947}";
        perfect_middle_second: "~\u{94d}यिष\u{947}", "~\u{94d}याथ\u{947}", "~\u{94d}यिध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{94d}य\u{947}", "~\u{94d}यात\u{947}", "~\u{94d}यिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ीवा\u{902}स\u{94d}", "~\u{94d}यान";
    ),
    // चचार: 5 words (जहार, दधार, जगार, चचार, ममार).
    conjugation!("चचार", 5, "~ार";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~र|~ार", "~\u{94d}रिव", "~\u{94d}रिम";
        perfect_active_second: "~र\u{94d}थ|~रिथ", "~\u{94d}रथ\u{941}ः", "~\u{94d}र";
        perfect_active_third: "~ार", "~\u{94d}रत\u{941}ः", "~\u{94d}र\u{941}ः";
        perfect_middle_first: "~\u{94d}र\u{947}", "~\u{94d}रिवह\u{947}", "~\u{94d}रिमह\u{947}";
        perfect_middle_second: "~\u{94d}रिष\u{947}", "~\u{94d}राथ\u{947}", "~\u{94d}रिध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{94d}र\u{947}", "~\u{94d}रात\u{947}", "~\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~\u{943}वा\u{902}स\u{94d}", "~\u{94d}राण";
    ),
    // आद: 4 words (आद, आप, आर, आव).
    conjugation!("आद", 4, "~";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~", "~िव", "~िम";
        perfect_active_second: "~िथ", "~थ\u{941}ः", "~";
        perfect_active_third: "~", "~त\u{941}ः", "~\u{941}ः";
        perfect_middle_first: "~\u{947}", "~िवह\u{947}", "~िमह\u{947}";
        perfect_middle_second: "~िष\u{947}", "~ाथ\u{947}", "~िध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{947}", "~ात\u{947}", "~िर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~िवा\u{902}स\u{94d}", "~ान";
    ),
    // वमति: 4 words (वमति, चमति, नमति, गमति).
    conjugation!("वमति", 4, "~मति";
        indicative_active_first: "~मामि", "~मावः", "~मामः";
        indicative_active_second: "~मसि", "~मथः", "~मथ";
        indicative_active_third: "~मति", "~मतः", "~मन\u{94d}ति";
        indicative_middle_first: "~म\u{947}", "~मावह\u{947}", "~मामह\u{947}";
        indicative_middle_second: "~मस\u{947}", "~म\u{947}थ\u{947}", "~मध\u{94d}व\u{947}";
        indicative_middle_third: "~मत\u{947}", "~म\u{947}त\u{947}", "~मन\u{94d}त\u{947}";
        imperative_active_first: "~मानि", "~माव", "~माम";
        imperative_active_second: "~म", "~मतम\u{94d}", "~मत";
        imperative_active_third: "~मत\u{941}", "~मताम\u{94d}", "~मन\u{94d}त\u{941}";
        imperative_middle_first: "~म\u{948}", "~मावह\u{948}", "~मामह\u{948}";
        imperative_middle_second: "~मस\u{94d}व", "~म\u{947}थाम\u{94d}", "~मध\u{94d}वम\u{94d}";
        imperative_middle_third: "~मताम\u{94d}", "~म\u{947}ताम\u{94d}", "~मन\u{94d}ताम\u{94d}";
        optative_active_first: "~म\u{947}यम\u{94d}", "~म\u{947}व", "~म\u{947}म";
        optative_active_second: "~म\u{947}ः", "~म\u{947}तम\u{94d}", "~म\u{947}त";
        optative_active_third: "~म\u{947}त\u{94d}", "~म\u{947}ताम\u{94d}", "~म\u{947}य\u{941}ः";
        optative_middle_first: "~म\u{947}य", "~म\u{947}वहि", "~म\u{947}महि";
        optative_middle_second: "~म\u{947}थाः", "~म\u{947}याथाम\u{94d}", "~म\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~म\u{947}त", "~म\u{947}याताम\u{94d}", "~म\u{947}रन\u{94d}";
        subjunctive_active_first: "~मानि", "~माव", "~माम";
        subjunctive_active_second: "~माः", "~माथः", "~माथ";
        subjunctive_active_third: "~मात\u{94d}", "~मातः", "~मान\u{94d}";
        subjunctive_middle_first: "~म\u{948}", "~मावह\u{948}", "~मामह\u{948}";
        subjunctive_middle_second: "~मास\u{947}", "~म\u{948}थ\u{947}", "~माध\u{94d}व\u{948}";
        subjunctive_middle_third: "~मात\u{947}", "~म\u{948}त\u{947}", "~मन\u{94d}त";
        imperfect_active_first: "अ~मम\u{94d}", "अ~माव", "अ~माम";
        imperfect_active_second: "अ~मः", "अ~मतम\u{94d}", "अ~मत";
        imperfect_active_third: "अ~मत\u{94d}", "अ~मताम\u{94d}", "अ~मन\u{94d}";
        imperfect_middle_first: "अ~म\u{947}", "अ~मावहि", "अ~मामहि";
        imperfect_middle_second: "अ~मथाः", "अ~म\u{947}थाम\u{94d}", "अ~मध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~मत", "अ~म\u{947}ताम\u{94d}", "अ~मन\u{94d}त";
        future_active_first: "~मिष\u{94d}यामि", "~मिष\u{94d}यावः", "~मिष\u{94d}यामः";
        future_active_second: "~मिष\u{94d}यसि", "~मिष\u{94d}यथः", "~मिष\u{94d}यथ";
        future_active_third: "~मिष\u{94d}यति", "~मिष\u{94d}यतः", "~मिष\u{94d}यन\u{94d}ति";
        future_middle_first: "~मिष\u{94d}य\u{947}", "~मिष\u{94d}यावह\u{947}", "~मिष\u{94d}यामह\u{947}";
        future_middle_second: "~मिष\u{94d}यस\u{947}", "~मिष\u{94d}य\u{947}थ\u{947}", "~मिष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~मिष\u{94d}यत\u{947}", "~मिष\u{94d}य\u{947}त\u{947}", "~मिष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~मिष\u{94d}यम\u{94d}", "अ~मिष\u{94d}याव", "अ~मिष\u{94d}याम";
        conditional_active_second: "अ~मिष\u{94d}यः", "अ~मिष\u{94d}यतम\u{94d}", "अ~मिष\u{94d}यत";
        conditional_active_third: "अ~मिष\u{94d}यत\u{94d}", "अ~मिष\u{94d}यताम\u{94d}", "अ~मिष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~मिष\u{94d}य\u{947}", "अ~मिष\u{94d}यावहि", "अ~मिष\u{94d}यामहि";
        conditional_middle_second: "अ~मिष\u{94d}यथाः", "अ~मिष\u{94d}य\u{947}थाम\u{94d}", "अ~मिष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~मिष\u{94d}यत", "अ~मिष\u{94d}य\u{947}ताम\u{94d}", "अ~मिष\u{94d}यन\u{94d}त";
        perfect_active_first: "~चम", "~\u{947}मिव", "~\u{947}मिम";
        perfect_active_second: "~चन\u{94d}थ", "~\u{947}मथ\u{941}ः", "~\u{947}म";
        perfect_active_third: "~चाम", "~\u{947}मत\u{941}ः", "~\u{947}म\u{941}ः";
        perfect_middle_first: "~\u{947}म\u{947}", "~\u{947}मिवह\u{947}", "~\u{947}मिमह\u{947}";
        perfect_middle_second: "~\u{947}मिष\u{947}", "~\u{947}माथ\u{947}", "~\u{947}मिध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{947}म\u{947}", "~\u{947}मात\u{947}", "~\u{947}मिर\u{947}";
        aorist_active_first: "अ~मिषम\u{94d}", "अ~मिष\u{94d}व", "अ~मिष\u{94d}म";
        aorist_active_second: "अ~मीः", "अ~मिष\u{94d}टम\u{94d}", "अ~मिष\u{94d}ट";
        aorist_active_third: "अ~मीत\u{94d}", "अ~मिष\u{94d}टाम\u{94d}", "अ~मिष\u{941}ः";
        aorist_middle_first: "अ~मिषि", "अ~मिष\u{94d}वहि", "अ~मिष\u{94d}महि";
        aorist_middle_second: "अ~मिष\u{94d}ठाः", "अ~मिषाथाम\u{94d}", "अ~मिढ\u{94d}वम\u{94d}";
        aorist_middle_third: "अ~मिष\u{94d}ट", "अ~मिषाताम\u{94d}", "अ~मिषत";
        infinitive: "~मित\u{941}म\u{94d}";
        gerund: "~मित\u{94d}वा";
        participle_present: "~मत\u{94d}", "~ममान";
        participle_future: "~मिष\u{94d}यत\u{94d}", "~मिष\u{94d}यमाण";
        participle_perfect: "~\u{947}मिवा\u{902}स\u{94d}", "~\u{947}मान";
    ),
    // वृणीते: 4 words (मिमीते, जानीते, जिहीते, वृणीते).
    conjugation!("व\u{943}णीत\u{947}", 4, "~ीत\u{947}";
        indicative_active_first: "~ामि", "~ीवः", "~ीमः";
        indicative_active_second: "~ासि", "~ीथः", "~ीथ";
        indicative_active_third: "~ाति", "~ीतः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ीवह\u{947}", "~ीमह\u{947}";
        indicative_middle_second: "~ीष\u{947}", "~ाथ\u{947}", "~ीध\u{94d}व\u{947}";
        indicative_middle_third: "~ीत\u{947}", "~ात\u{947}", "~त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~ीहि", "~ीतम\u{94d}", "~ीत";
        imperative_active_third: "~ात\u{941}", "~ीताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~ीष\u{94d}व", "~ाथाम\u{94d}", "~ीध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ीताम\u{94d}", "~ाताम\u{94d}", "~ताम\u{94d}";
        optative_active_first: "~ीयाम\u{94d}", "~ीयाव", "~ीयाम";
        optative_active_second: "~ीयाः", "~ीयातम\u{94d}", "~ीयात";
        optative_active_third: "~ीयात\u{94d}", "~ीयाताम\u{94d}", "~ीय\u{941}ः";
        optative_middle_first: "~ीय", "~ीवहि", "~ीमहि";
        optative_middle_second: "~ीथाः", "~ीयाथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~ीत", "~ीयाताम\u{94d}", "~ीरन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{947}|~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~ान\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "अ~ाम\u{94d}", "अ~ीव", "अ~ीम";
        imperfect_active_second: "अ~ाः", "अ~ीतम\u{94d}", "अ~ीत";
        imperfect_active_third: "अ~ात\u{94d}", "अ~ीताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~ि", "अ~ीवहि", "अ~ीमहि";
        imperfect_middle_second: "अ~ीथाः", "अ~ाथाम\u{94d}", "अ~ीध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~ीत", "अ~ाताम\u{94d}", "अ~त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~ान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // सरवति: 4 words (स्रवति, द्रवति, चर्वति, तूर्वति).
    conjugation!("स\u{94d}रवति", 4, "~~वति";
        indicative_active_first: "~~वामि", "~~वावः", "~~वामः";
        indicative_active_second: "~~वसि", "~~वथः", "~~वथ";
        indicative_active_third: "~~वति", "~~वतः", "~~वन\u{94d}ति";
        indicative_middle_first: "~~व\u{947}", "~~वावह\u{947}", "~~वामह\u{947}";
        indicative_middle_second: "~~वस\u{947}", "~~व\u{947}थ\u{947}", "~~वध\u{94d}व\u{947}";
        indicative_middle_third: "~~वत\u{947}", "~~व\u{947}त\u{947}", "~~वन\u{94d}त\u{947}";
        imperative_active_first: "~~वाणि", "~~वाव", "~~वाम";
        imperative_active_second: "~~व", "~~वतम\u{94d}", "~~वत";
        imperative_active_third: "~~वत\u{941}", "~~वताम\u{94d}", "~~वन\u{94d}त\u{941}";
        imperative_middle_first: "~~व\u{948}", "~~वावह\u{948}", "~~वामह\u{948}";
        imperative_middle_second: "~~वस\u{94d}व", "~~व\u{947}थाम\u{94d}", "~~वध\u{94d}वम\u{94d}";
        imperative_middle_third: "~~वताम\u{94d}", "~~व\u{947}ताम\u{94d}", "~~वन\u{94d}ताम\u{94d}";
        optative_active_first: "~~व\u{947}यम\u{94d}", "~~व\u{947}व", "~~व\u{947}म";
        optative_active_second: "~~व\u{947}ः", "~~व\u{947}तम\u{94d}", "~~व\u{947}त";
        optative_active_third: "~~व\u{947}त\u{94d}", "~~व\u{947}ताम\u{94d}", "~~व\u{947}य\u{941}ः";
        optative_middle_first: "~~व\u{947}य", "~~व\u{947}वहि", "~~व\u{947}महि";
        optative_middle_second: "~~व\u{947}थाः", "~~व\u{947}याथाम\u{94d}", "~~व\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~~व\u{947}त", "~~व\u{947}याताम\u{94d}", "~~व\u{947}रन\u{94d}";
        subjunctive_active_first: "~~वाणि", "~~वाव", "~~वाम";
        subjunctive_active_second: "~~वाः", "~~वाथः", "~~वाथ";
        subjunctive_active_third: "~~वात\u{94d}", "~~वातः", "~~वान\u{94d}";
        subjunctive_middle_first: "~~व\u{948}", "~~वावह\u{948}", "~~वामह\u{948}";
        subjunctive_middle_second: "~~वास\u{947}", "~~व\u{948}थ\u{947}", "~~वाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~~वात\u{947}", "~~व\u{948}त\u{947}", "~~वन\u{94d}त";
        imperfect_active_first: "अ~~वम\u{94d}", "अ~~वाव", "अ~~वाम";
        imperfect_active_second: "अ~~वः", "अ~~वतम\u{94d}", "अ~~वत";
        imperfect_active_third: "अ~~वत\u{94d}", "अ~~वताम\u{94d}", "अ~~वन\u{94d}";
        imperfect_middle_first: "अ~~व\u{947}", "अ~~वावहि", "अ~~वामहि";
        imperfect_middle_second: "अ~~वथाः", "अ~~व\u{947}थाम\u{94d}", "अ~~वध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~~वत", "अ~~व\u{947}ताम\u{94d}", "अ~~वन\u{94d}त";
        future_active_first: "~~ोष\u{94d}यामि", "~~ोष\u{94d}यावः", "~~ोष\u{94d}यामः";
        future_active_second: "~~ोष\u{94d}यसि", "~~ोष\u{94d}यथः", "~~ोष\u{94d}यथ";
        future_active_third: "~~ोष\u{94d}यति", "~~ोष\u{94d}यतः", "~~ोष\u{94d}यन\u{94d}ति";
        future_middle_first: "~~ोष\u{94d}य\u{947}", "~~ोष\u{94d}यावह\u{947}", "~~ोष\u{94d}यामह\u{947}";
        future_middle_second: "~~ोष\u{94d}यस\u{947}", "~~ोष\u{94d}य\u{947}थ\u{947}", "~~ोष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~~ोष\u{94d}यत\u{947}", "~~ोष\u{94d}य\u{947}त\u{947}", "~~ोष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~~ोष\u{94d}यम\u{94d}", "अ~~ोष\u{94d}याव", "अ~~ोष\u{94d}याम";
        conditional_active_second: "अ~~ोष\u{94d}यः", "अ~~ोष\u{94d}यतम\u{94d}", "अ~~ोष\u{94d}यत";
        conditional_active_third: "अ~~ोष\u{94d}यत\u{94d}", "अ~~ोष\u{94d}यताम\u{94d}", "अ~~ोष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~~ोष\u{94d}य\u{947}", "अ~~ोष\u{94d}यावहि", "अ~~ोष\u{94d}यामहि";
        conditional_middle_second: "अ~~ोष\u{94d}यथाः", "अ~~ोष\u{94d}य\u{947}थाम\u{94d}", "अ~~ोष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~~ोष\u{94d}यत", "अ~~ोष\u{94d}य\u{947}ताम\u{94d}", "अ~~ोष\u{94d}यन\u{94d}त";
        perfect_active_first: "~\u{941}द~व", "~\u{941}द~\u{941}व", "~\u{941}द~\u{941}म";
        perfect_active_second: "~\u{941}द~ोथ", "~\u{941}द~\u{941}वथ\u{941}ः", "~\u{941}द~\u{941}व";
        perfect_active_third: "~\u{941}द~ाव", "~\u{941}द~\u{941}वत\u{941}ः", "~\u{941}द~\u{941}व\u{941}ः";
        perfect_middle_first: "~\u{941}द~\u{941}व\u{947}", "~\u{941}द~\u{941}वह\u{947}", "~\u{941}द~\u{941}मह\u{947}";
        perfect_middle_second: "~\u{941}द~\u{941}ष\u{947}", "~\u{941}द~\u{941}वाथ\u{947}", "~\u{941}द~\u{941}ध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{941}द~\u{941}व\u{947}", "~\u{941}द~\u{941}वात\u{947}", "~\u{941}द~\u{941}विर\u{947}";
        aorist_active_first: "अ~~\u{941}द\u{94d}र\u{941}वम\u{94d}", "अ~~\u{941}द\u{94d}र\u{941}वाव", "अ~~\u{941}द\u{94d}र\u{941}वाम";
        aorist_active_second: "अ~~\u{941}द\u{94d}र\u{941}वः", "अ~~\u{941}द\u{94d}र\u{941}वतम\u{94d}", "अ~~\u{941}द\u{94d}र\u{941}वत";
        aorist_active_third: "अ~~\u{941}द\u{94d}र\u{941}वत\u{94d}", "अ~~\u{941}द\u{94d}र\u{941}वताम\u{94d}", "अ~~\u{941}द\u{94d}र\u{941}वन\u{94d}";
        aorist_middle_first: "अ~~ाविषि", "अ~~ाविष\u{94d}वहि", "अ~~ाविष\u{94d}महि";
        aorist_middle_second: "अ~~ाविष\u{94d}ठाः", "अ~~ाविषाथाम\u{94d}", "अ~~ाविढ\u{94d}वम\u{94d}";
        aorist_middle_third: "अ~~ाविष\u{94d}ट", "अ~~ाविषाताम\u{94d}", "अ~~ाविषत";
        infinitive: "~~ोत\u{941}म\u{94d}";
        gerund: "~~\u{941}त\u{94d}वा";
        participle_present: "~~वत\u{94d}", "~~वमाण";
        participle_future: "~~ोष\u{94d}यत\u{94d}", "~~ोष\u{94d}यमाण";
        participle_perfect: "~\u{941}द~\u{941}वा\u{902}स\u{94d}", "~\u{941}द~\u{941}वाण";
    ),
    // आनयति: 4 words (आनयति, आगच्छति, आक्रोशति, आमयति).
    conjugation!("आनयति", 4, "~ति";
        indicative_active_first: "~ामि", "~ावः", "~ामः|~ामसि";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः|~ासि", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}|~ाति", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~न\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "~म\u{94d}", "~ाव", "~ाम";
        imperfect_active_second: "~ः", "~तम\u{94d}", "~त";
        imperfect_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        imperfect_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        imperfect_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // इरसयति: 4 words (इच्छति, इङ्गति, इरस्यति, इङ्गयति).
    conjugation!("इरस\u{94d}यति", 4, "इ~ति";
        indicative_active_first: "इ~ामि", "इ~ावः", "इ~ामः|इ~ामसि";
        indicative_active_second: "इ~सि", "इ~थः", "इ~थ";
        indicative_active_third: "इ~ति", "इ~तः", "इ~न\u{94d}ति";
        indicative_middle_first: "इ~\u{947}", "इ~ावह\u{947}", "इ~ामह\u{947}";
        indicative_middle_second: "इ~स\u{947}", "इ~\u{947}थ\u{947}", "इ~ध\u{94d}व\u{947}";
        indicative_middle_third: "इ~त\u{947}", "इ~\u{947}त\u{947}", "इ~न\u{94d}त\u{947}";
        imperative_active_first: "इ~ानि", "इ~ाव", "इ~ाम";
        imperative_active_second: "इ~", "इ~तम\u{94d}", "इ~त";
        imperative_active_third: "इ~त\u{941}", "इ~ताम\u{94d}", "इ~न\u{94d}त\u{941}";
        imperative_middle_first: "इ~\u{948}", "इ~ावह\u{948}", "इ~ामह\u{948}";
        imperative_middle_second: "इ~स\u{94d}व", "इ~\u{947}थाम\u{94d}", "इ~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "इ~ताम\u{94d}", "इ~\u{947}ताम\u{94d}", "इ~न\u{94d}ताम\u{94d}";
        optative_active_first: "इ~\u{947}यम\u{94d}", "इ~\u{947}व", "इ~\u{947}म";
        optative_active_second: "इ~\u{947}ः", "इ~\u{947}तम\u{94d}", "इ~\u{947}त";
        optative_active_third: "इ~\u{947}त\u{94d}", "इ~\u{947}ताम\u{94d}", "इ~\u{947}य\u{941}ः";
        optative_middle_first: "इ~\u{947}य", "इ~\u{947}वहि", "इ~\u{947}महि";
        optative_middle_second: "इ~\u{947}थाः", "इ~\u{947}याथाम\u{94d}", "इ~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "इ~\u{947}त", "इ~\u{947}याताम\u{94d}", "इ~\u{947}रन\u{94d}";
        subjunctive_active_first: "इ~ानि", "इ~ाव", "इ~ाम";
        subjunctive_active_second: "इ~ाः|इ~ासि", "इ~ाथः", "इ~ाथ";
        subjunctive_active_third: "इ~ात\u{94d}|इ~ाति", "इ~ातः", "इ~ान\u{94d}";
        subjunctive_middle_first: "इ~\u{948}", "इ~ावह\u{948}", "इ~ामह\u{948}";
        subjunctive_middle_second: "इ~ास\u{947}|इ~ास\u{948}", "इ~\u{948}थ\u{947}", "इ~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "इ~ात\u{947}|इ~ात\u{948}", "इ~\u{948}त\u{947}", "इ~न\u{94d}त|इ~ान\u{94d}त\u{948}";
        imperfect_active_first: "ऐ~म\u{94d}", "ऐ~ाव", "ऐ~ाम";
        imperfect_active_second: "ऐ~ः", "ऐ~तम\u{94d}", "ऐ~त";
        imperfect_active_third: "ऐ~त\u{94d}", "ऐ~ताम\u{94d}", "ऐ~न\u{94d}";
        imperfect_middle_first: "ऐ~\u{947}", "ऐ~ावहि", "ऐ~ामहि";
        imperfect_middle_second: "ऐ~थाः", "ऐ~\u{947}थाम\u{94d}", "ऐ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "ऐ~त", "ऐ~\u{947}ताम\u{94d}", "ऐ~न\u{94d}त";
        future_active_first: "इ~िष\u{94d}यामि", "इ~िष\u{94d}यावः", "इ~िष\u{94d}यामः";
        future_active_second: "इ~िष\u{94d}यसि", "इ~िष\u{94d}यथः", "इ~िष\u{94d}यथ";
        future_active_third: "इ~िष\u{94d}यति", "इ~िष\u{94d}यतः", "इ~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "इ~िष\u{94d}य\u{947}", "इ~िष\u{94d}यावह\u{947}", "इ~िष\u{94d}यामह\u{947}";
        future_middle_second: "इ~िष\u{94d}यस\u{947}", "इ~िष\u{94d}य\u{947}थ\u{947}", "इ~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "इ~िष\u{94d}यत\u{947}", "इ~िष\u{94d}य\u{947}त\u{947}", "इ~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "ऐ~िष\u{94d}यम\u{94d}", "ऐ~िष\u{94d}याव", "ऐ~िष\u{94d}याम";
        conditional_active_second: "ऐ~िष\u{94d}यः", "ऐ~िष\u{94d}यतम\u{94d}", "ऐ~िष\u{94d}यत";
        conditional_active_third: "ऐ~िष\u{94d}यत\u{94d}", "ऐ~िष\u{94d}यताम\u{94d}", "ऐ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "ऐ~िष\u{94d}य\u{947}", "ऐ~िष\u{94d}यावहि", "ऐ~िष\u{94d}यामहि";
        conditional_middle_second: "ऐ~िष\u{94d}यथाः", "ऐ~िष\u{94d}य\u{947}थाम\u{94d}", "ऐ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "ऐ~िष\u{94d}यत", "ऐ~िष\u{94d}य\u{947}ताम\u{94d}", "ऐ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "इ~ामास", "इ~ामासिव", "इ~ामासिम";
        perfect_active_second: "इ~ामासिथ", "इ~ामासथ\u{941}ः", "इ~ामास";
        perfect_active_third: "इ~ामास", "इ~ामासत\u{941}ः", "इ~ामास\u{941}ः";
        perfect_middle_first: "इ~ा\u{902}चक\u{94d}र\u{947}", "इ~ा\u{902}चक\u{943}वह\u{947}", "इ~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "इ~ा\u{902}चक\u{943}ष\u{947}", "इ~ा\u{902}चक\u{94d}राथ\u{947}", "इ~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "इ~ा\u{902}चक\u{94d}र\u{947}", "इ~ा\u{902}चक\u{94d}रात\u{947}", "इ~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "इ~ित\u{941}म\u{94d}";
        gerund: "इ~ित\u{94d}वा";
        participle_present: "इ~त\u{94d}", "इ~मान";
        participle_future: "इ~िष\u{94d}यत\u{94d}", "इ~िष\u{94d}यमाण";
        participle_perfect: "इ~ामासिवा\u{902}स\u{94d}", "इ~ा\u{902}चक\u{94d}राण";
    ),
    // चकार: 3 words (चकार, बभार, जभार).
    conjugation!("चकार", 3, "~ार";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~र|~ार", "~\u{943}व", "~\u{943}म";
        perfect_active_second: "~र\u{94d}थ", "~\u{94d}रथ\u{941}ः", "~\u{94d}र";
        perfect_active_third: "~ार", "~\u{94d}रत\u{941}ः", "~\u{94d}र\u{941}ः";
        perfect_middle_first: "~\u{94d}र\u{947}", "~\u{943}वह\u{947}", "~\u{943}मह\u{947}";
        perfect_middle_second: "~\u{943}ष\u{947}", "~\u{94d}राथ\u{947}", "~\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{94d}र\u{947}", "~\u{94d}रात\u{947}", "~\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~\u{943}वा\u{902}स\u{94d}", "~\u{94d}राण";
    ),
    // आचरति: 3 words (आचरति, पक्षति, यक्षति).
    conjugation!("आचरति", 3, "~ति";
        indicative_active_first: "~ामि", "~ावः", "~ामः|~ामसि";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ाणि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ाणि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः|~ासि", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}|~ाति", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}|~ास\u{948}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}|~ात\u{948}", "~\u{948}त\u{947}", "~न\u{94d}त|~ान\u{94d}त\u{948}";
        imperfect_active_first: "~म\u{94d}", "~ाव", "~ाम";
        imperfect_active_second: "~ः", "~तम\u{94d}", "~त";
        imperfect_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        imperfect_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        imperfect_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~माण";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // परवदति: 3 words (उपन्यस्यति, प्रवदति, प्रक्षालयति).
    conjugation!("प\u{94d}रवदति", 3, "~~ति";
        indicative_active_first: "~~ामि", "~~ावः", "~~ामः|~~ामसि";
        indicative_active_second: "~~सि", "~~थः", "~~थ";
        indicative_active_third: "~~ति", "~~तः", "~~न\u{94d}ति";
        indicative_middle_first: "~~\u{947}", "~~ावह\u{947}", "~~ामह\u{947}";
        indicative_middle_second: "~~स\u{947}", "~~\u{947}थ\u{947}", "~~ध\u{94d}व\u{947}";
        indicative_middle_third: "~~त\u{947}", "~~\u{947}त\u{947}", "~~न\u{94d}त\u{947}";
        imperative_active_first: "~~ानि", "~~ाव", "~~ाम";
        imperative_active_second: "~~", "~~तम\u{94d}", "~~त";
        imperative_active_third: "~~त\u{941}", "~~ताम\u{94d}", "~~न\u{94d}त\u{941}";
        imperative_middle_first: "~~\u{948}", "~~ावह\u{948}", "~~ामह\u{948}";
        imperative_middle_second: "~~स\u{94d}व", "~~\u{947}थाम\u{94d}", "~~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~~ताम\u{94d}", "~~\u{947}ताम\u{94d}", "~~न\u{94d}ताम\u{94d}";
        optative_active_first: "~~\u{947}यम\u{94d}", "~~\u{947}व", "~~\u{947}म";
        optative_active_second: "~~\u{947}ः", "~~\u{947}तम\u{94d}", "~~\u{947}त";
        optative_active_third: "~~\u{947}त\u{94d}", "~~\u{947}ताम\u{94d}", "~~\u{947}य\u{941}ः";
        optative_middle_first: "~~\u{947}य", "~~\u{947}वहि", "~~\u{947}महि";
        optative_middle_second: "~~\u{947}थाः", "~~\u{947}याथाम\u{94d}", "~~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~~\u{947}त", "~~\u{947}याताम\u{94d}", "~~\u{947}रन\u{94d}";
        subjunctive_active_first: "~~ानि", "~~ाव", "~~ाम";
        subjunctive_active_second: "~~ाः|~~ासि", "~~ाथः", "~~ाथ";
        subjunctive_active_third: "~~ात\u{94d}|~~ाति", "~~ातः", "~~ान\u{94d}";
        subjunctive_middle_first: "~~\u{948}", "~~ावह\u{948}", "~~ामह\u{948}";
        subjunctive_middle_second: "~~ास\u{947}|~~ास\u{948}", "~~\u{948}थ\u{947}", "~~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~~ात\u{947}|~~ात\u{948}", "~~\u{948}त\u{947}", "~~न\u{94d}त|~~ान\u{94d}त\u{948}";
        imperfect_active_first: "~ा~म\u{94d}", "~ा~ाव", "~ा~ाम";
        imperfect_active_second: "~ा~ः", "~ा~तम\u{94d}", "~ा~त";
        imperfect_active_third: "~ा~त\u{94d}", "~ा~ताम\u{94d}", "~ा~न\u{94d}";
        imperfect_middle_first: "~ा~\u{947}", "~ा~ावहि", "~ा~ामहि";
        imperfect_middle_second: "~ा~थाः", "~ा~\u{947}थाम\u{94d}", "~ा~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "~ा~त", "~ा~\u{947}ताम\u{94d}", "~ा~न\u{94d}त";
        future_active_first: "~~िष\u{94d}यामि", "~~िष\u{94d}यावः", "~~िष\u{94d}यामः";
        future_active_second: "~~िष\u{94d}यसि", "~~िष\u{94d}यथः", "~~िष\u{94d}यथ";
        future_active_third: "~~िष\u{94d}यति", "~~िष\u{94d}यतः", "~~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~~िष\u{94d}य\u{947}", "~~िष\u{94d}यावह\u{947}", "~~िष\u{94d}यामह\u{947}";
        future_middle_second: "~~िष\u{94d}यस\u{947}", "~~िष\u{94d}य\u{947}थ\u{947}", "~~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~~िष\u{94d}यत\u{947}", "~~िष\u{94d}य\u{947}त\u{947}", "~~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~~िष\u{94d}यम\u{94d}", "अ~~िष\u{94d}याव", "अ~~िष\u{94d}याम";
        conditional_active_second: "अ~~िष\u{94d}यः", "अ~~िष\u{94d}यतम\u{94d}", "अ~~िष\u{94d}यत";
        conditional_active_third: "अ~~िष\u{94d}यत\u{94d}", "अ~~िष\u{94d}यताम\u{94d}", "अ~~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~~िष\u{94d}य\u{947}", "अ~~िष\u{94d}यावहि", "अ~~िष\u{94d}यामहि";
        conditional_middle_second: "अ~~िष\u{94d}यथाः", "अ~~िष\u{94d}य\u{947}थाम\u{94d}", "अ~~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~~िष\u{94d}यत", "अ~~िष\u{94d}य\u{947}ताम\u{94d}", "अ~~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~िन~", "~िन~िव", "~िन~िम";
        perfect_active_second: "~िन~िथ", "~िन~थ\u{941}ः", "~िन~";
        perfect_active_third: "~िन~", "~िन~त\u{941}ः", "~िन~\u{941}ः";
        perfect_middle_first: "~िन~\u{947}", "~िन~िवह\u{947}", "~िन~िमह\u{947}";
        perfect_middle_second: "~िन~िष\u{947}", "~िन~ाथ\u{947}", "~िन~िध\u{94d}व\u{947}";
        perfect_middle_third: "~िन~\u{947}", "~िन~ात\u{947}", "~िन~िर\u{947}";
        aorist_active_first: "अ~~िषम\u{94d}", "अ~~िष\u{94d}व", "अ~~िष\u{94d}म";
        aorist_active_second: "अ~~ीः", "अ~~िष\u{94d}टम\u{94d}", "अ~~िष\u{94d}ट";
        aorist_active_third: "अ~~ीत\u{94d}", "अ~~िष\u{94d}टाम\u{94d}", "अ~~िष\u{941}ः";
        aorist_middle_first: "अ~~िषि", "अ~~िष\u{94d}वहि", "अ~~िष\u{94d}महि";
        aorist_middle_second: "अ~~िष\u{94d}ठाः", "अ~~िषाथाम\u{94d}", "अ~~िढ\u{94d}वम\u{94d}";
        aorist_middle_third: "अ~~िष\u{94d}ट", "अ~~िषाताम\u{94d}", "अ~~िषत";
        infinitive: "~~ित\u{941}म\u{94d}";
        gerund: "~~ित\u{94d}वा";
        participle_present: "~~त\u{94d}", "~~मान";
        participle_future: "~~िष\u{94d}यत\u{94d}", "~~िष\u{94d}यमाण";
        participle_perfect: "~िन~\u{94d}वा\u{902}स\u{94d}", "~िन~ान";
    ),
    // मुमोच: 3 words (मुमोच, शुशोच, रुरोच).
    conjugation!("म\u{941}मोच", 3, "~ोच";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ोच", "~\u{941}चिव|~\u{941}च\u{94d}व", "~\u{941}चिम|~\u{941}च\u{94d}म";
        perfect_active_second: "~ोचिथ", "~\u{941}चथ\u{941}ः", "~\u{941}च";
        perfect_active_third: "~ोच", "~\u{941}चत\u{941}ः", "~\u{941}च\u{941}ः";
        perfect_middle_first: "~\u{941}च\u{947}", "~\u{941}चिवह\u{947}|~\u{941}च\u{94d}वह\u{947}", "~\u{941}चिमह\u{947}|~\u{941}च\u{94d}मह\u{947}";
        perfect_middle_second: "~\u{941}चिष\u{947}|~\u{941}क\u{94d}ष\u{947}", "~\u{941}चाथ\u{947}", "~\u{941}चिध\u{94d}व\u{947}|~\u{941}ग\u{94d}ध\u{94d}व\u{947}";
        perfect_middle_third: "~\u{941}च\u{947}", "~\u{941}चात\u{947}", "~\u{941}चिर\u{947}|~\u{941}च\u{94d}र\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~\u{947}", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~\u{947}ताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~\u{941}क\u{94d}वा\u{902}स\u{94d}", "~\u{941}चान";
    ),
    // अकारषीत: 3 words (अहार्षीत्, अभार्षीत्, अकार्षीत्).
    conjugation!("अकार\u{94d}षीत\u{94d}", 3, "~ार\u{94d}षीत\u{94d}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~ार\u{94d}षम\u{94d}", "~ार\u{94d}ष\u{94d}व", "~ार\u{94d}ष\u{94d}म";
        aorist_active_second: "~ार\u{94d}षीः|~ाः", "~ार\u{94d}ष\u{94d}टम\u{94d}", "~ार\u{94d}ष\u{94d}ट";
        aorist_active_third: "~ार\u{94d}षीत\u{94d}|~ाः", "~ार\u{94d}ष\u{94d}टाम\u{94d}", "~ार\u{94d}ष\u{941}ः";
        aorist_middle_first: "~\u{943}षि", "~\u{943}ष\u{94d}वहि", "~\u{943}ष\u{94d}महि";
        aorist_middle_second: "~\u{943}ष\u{94d}ठाः", "~\u{943}षाथाम\u{94d}", "~\u{943}ढ\u{94d}वम\u{94d}";
        aorist_middle_third: "~\u{943}ष\u{94d}ट", "~\u{943}षाताम\u{94d}", "~\u{943}षत";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // अधुकषत: 3 words (अधुक्षत्, अद्रुक्षत्, अरुक्षत्).
    conjugation!("अध\u{941}क\u{94d}षत\u{94d}", 3, "~त\u{94d}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~म\u{94d}", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम\u{94d}", "~त";
        aorist_active_third: "~त\u{94d}", "~ताम\u{94d}", "~न\u{94d}";
        aorist_middle_first: "~ि", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~ाथाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        aorist_middle_third: "~त", "~ाताम\u{94d}", "~न\u{94d}त";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
    // अचैषीत: 3 words (अनैषीत्, अजैषीत्, अचैषीत्).
    conjugation!("अच\u{948}षीत\u{94d}", 3, "~\u{948}षीत\u{94d}";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न\u{94d}ति";
        indicative_middle_first: "~\u{947}", "~ावह\u{947}", "~ामह\u{947}";
        indicative_middle_second: "~स\u{947}", "~\u{947}थ\u{947}", "~ध\u{94d}व\u{947}";
        indicative_middle_third: "~त\u{947}", "~\u{947}त\u{947}", "~न\u{94d}त\u{947}";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम\u{94d}", "~त";
        imperative_active_third: "~त\u{941}", "~ताम\u{94d}", "~न\u{94d}त\u{941}";
        imperative_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        imperative_middle_second: "~स\u{94d}व", "~\u{947}थाम\u{94d}", "~ध\u{94d}वम\u{94d}";
        imperative_middle_third: "~ताम\u{94d}", "~\u{947}ताम\u{94d}", "~न\u{94d}ताम\u{94d}";
        optative_active_first: "~\u{947}यम\u{94d}", "~\u{947}व", "~\u{947}म";
        optative_active_second: "~\u{947}ः", "~\u{947}तम\u{94d}", "~\u{947}त";
        optative_active_third: "~\u{947}त\u{94d}", "~\u{947}ताम\u{94d}", "~\u{947}य\u{941}ः";
        optative_middle_first: "~\u{947}य", "~\u{947}वहि", "~\u{947}महि";
        optative_middle_second: "~\u{947}थाः", "~\u{947}याथाम\u{94d}", "~\u{947}ध\u{94d}वम\u{94d}";
        optative_middle_third: "~\u{947}त", "~\u{947}याताम\u{94d}", "~\u{947}रन\u{94d}";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात\u{94d}", "~ातः", "~ान\u{94d}";
        subjunctive_middle_first: "~\u{948}", "~ावह\u{948}", "~ामह\u{948}";
        subjunctive_middle_second: "~ास\u{947}", "~\u{948}थ\u{947}", "~ाध\u{94d}व\u{948}";
        subjunctive_middle_third: "~ात\u{947}", "~\u{948}त\u{947}", "~न\u{94d}त";
        imperfect_active_first: "अ~म\u{94d}", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम\u{94d}", "अ~त";
        imperfect_active_third: "अ~त\u{94d}", "अ~ताम\u{94d}", "अ~न\u{94d}";
        imperfect_middle_first: "अ~\u{947}", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~\u{947}थाम\u{94d}", "अ~ध\u{94d}वम\u{94d}";
        imperfect_middle_third: "अ~त", "अ~\u{947}ताम\u{94d}", "अ~न\u{94d}त";
        future_active_first: "~िष\u{94d}यामि", "~िष\u{94d}यावः", "~िष\u{94d}यामः";
        future_active_second: "~िष\u{94d}यसि", "~िष\u{94d}यथः", "~िष\u{94d}यथ";
        future_active_third: "~िष\u{94d}यति", "~िष\u{94d}यतः", "~िष\u{94d}यन\u{94d}ति";
        future_middle_first: "~िष\u{94d}य\u{947}", "~िष\u{94d}यावह\u{947}", "~िष\u{94d}यामह\u{947}";
        future_middle_second: "~िष\u{94d}यस\u{947}", "~िष\u{94d}य\u{947}थ\u{947}", "~िष\u{94d}यध\u{94d}व\u{947}";
        future_middle_third: "~िष\u{94d}यत\u{947}", "~िष\u{94d}य\u{947}त\u{947}", "~िष\u{94d}यन\u{94d}त\u{947}";
        conditional_active_first: "अ~िष\u{94d}यम\u{94d}", "अ~िष\u{94d}याव", "अ~िष\u{94d}याम";
        conditional_active_second: "अ~िष\u{94d}यः", "अ~िष\u{94d}यतम\u{94d}", "अ~िष\u{94d}यत";
        conditional_active_third: "अ~िष\u{94d}यत\u{94d}", "अ~िष\u{94d}यताम\u{94d}", "अ~िष\u{94d}यन\u{94d}";
        conditional_middle_first: "अ~िष\u{94d}य\u{947}", "अ~िष\u{94d}यावहि", "अ~िष\u{94d}यामहि";
        conditional_middle_second: "अ~िष\u{94d}यथाः", "अ~िष\u{94d}य\u{947}थाम\u{94d}", "अ~िष\u{94d}यध\u{94d}वम\u{94d}";
        conditional_middle_third: "अ~िष\u{94d}यत", "अ~िष\u{94d}य\u{947}ताम\u{94d}", "अ~िष\u{94d}यन\u{94d}त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथ\u{941}ः", "~ामास";
        perfect_active_third: "~ामास", "~ामासत\u{941}ः", "~ामास\u{941}ः";
        perfect_middle_first: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{943}वह\u{947}", "~ा\u{902}चक\u{943}मह\u{947}";
        perfect_middle_second: "~ा\u{902}चक\u{943}ष\u{947}", "~ा\u{902}चक\u{94d}राथ\u{947}", "~ा\u{902}चक\u{943}ध\u{94d}व\u{947}";
        perfect_middle_third: "~ा\u{902}चक\u{94d}र\u{947}", "~ा\u{902}चक\u{94d}रात\u{947}", "~ा\u{902}चक\u{94d}रिर\u{947}";
        aorist_active_first: "~\u{948}षम\u{94d}", "~\u{948}ष\u{94d}व", "~\u{948}ष\u{94d}म";
        aorist_active_second: "~\u{948}षीः|~\u{948}ः", "~\u{948}ष\u{94d}टम\u{94d}", "~\u{948}ष\u{94d}ट";
        aorist_active_third: "~\u{948}षीत\u{94d}|~\u{948}ः", "~\u{948}ष\u{94d}टाम\u{94d}", "~\u{948}ष\u{941}ः";
        aorist_middle_first: "~\u{947}षि", "~\u{947}ष\u{94d}वहि", "~\u{947}ष\u{94d}महि";
        aorist_middle_second: "~\u{947}ष\u{94d}ठाः", "~\u{947}षाथाम\u{94d}", "~\u{947}ढ\u{94d}वम\u{94d}";
        aorist_middle_third: "~\u{947}ष\u{94d}ट", "~\u{947}षाताम\u{94d}", "~\u{947}षत";
        infinitive: "~ित\u{941}म\u{94d}";
        gerund: "~ित\u{94d}वा";
        participle_present: "~त\u{94d}", "~मान";
        participle_future: "~िष\u{94d}यत\u{94d}", "~िष\u{94d}यमाण";
        participle_perfect: "~ामासिवा\u{902}स\u{94d}", "~ा\u{902}चक\u{94d}राण";
    ),
];
