//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the conjugation's it is like):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
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
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
                flexion::cell!($c2), "\t",
                flexion::cell!($c3), "\t",
                flexion::cell!($c4), "\t",
                flexion::cell!($c5), "\t",
                flexion::cell!($c6), "\t",
                flexion::cell!($c7), "\t",
                flexion::cell!($c8), "\t",
                flexion::cell!($c9), "\t",
                flexion::cell!($c10), "\t",
                flexion::cell!($c11), "\t",
                flexion::cell!($c12), "\t",
                flexion::cell!($c13), "\t",
                flexion::cell!($c14), "\t",
                flexion::cell!($c15), "\t",
                flexion::cell!($c16), "\t",
                flexion::cell!($c17), "\t",
                flexion::cell!($c18), "\t",
                flexion::cell!($c19), "\t",
                flexion::cell!($c20), "\t",
                flexion::cell!($c21), "\t",
                flexion::cell!($c22), "\t",
                flexion::cell!($c23), "\t",
                flexion::cell!($c24), "\t",
                flexion::cell!($c25), "\t",
                flexion::cell!($c26), "\t",
                flexion::cell!($c27), "\t",
                flexion::cell!($c28), "\t",
                flexion::cell!($c29), "\t",
                flexion::cell!($c30), "\t",
                flexion::cell!($c31), "\t",
                flexion::cell!($c32), "\t",
                flexion::cell!($c33), "\t",
                flexion::cell!($c34), "\t",
                flexion::cell!($c35), "\t",
                flexion::cell!($c36), "\t",
                flexion::cell!($c37), "\t",
                flexion::cell!($c38), "\t",
                flexion::cell!($c39), "\t",
                flexion::cell!($c40), "\t",
                flexion::cell!($c41), "\t",
                flexion::cell!($c42), "\t",
                flexion::cell!($c43), "\t",
                flexion::cell!($c44), "\t",
                flexion::cell!($c45), "\t",
                flexion::cell!($c46), "\t",
                flexion::cell!($c47), "\t",
                flexion::cell!($c48), "\t",
                flexion::cell!($c49), "\t",
                flexion::cell!($c50), "\t",
                flexion::cell!($c51), "\t",
                flexion::cell!($c52), "\t",
                flexion::cell!($c53), "\t",
                flexion::cell!($c54), "\t",
                flexion::cell!($c55), "\t",
                flexion::cell!($c56), "\t",
                flexion::cell!($c57), "\t",
                flexion::cell!($c58), "\t",
                flexion::cell!($c59), "\t",
                flexion::cell!($c60), "\t",
                flexion::cell!($c61), "\t",
                flexion::cell!($c62), "\t",
                flexion::cell!($c63), "\t",
                flexion::cell!($c64), "\t",
                flexion::cell!($c65), "\t",
                flexion::cell!($c66), "\t",
                flexion::cell!($c67), "\t",
                flexion::cell!($c68), "\t",
                flexion::cell!($c69), "\t",
                flexion::cell!($c70), "\t",
                flexion::cell!($c71), "\t",
                flexion::cell!($c72), "\t",
                flexion::cell!($c73), "\t",
                flexion::cell!($c74), "\t",
                flexion::cell!($c75), "\t",
                flexion::cell!($c76), "\t",
                flexion::cell!($c77), "\t",
                flexion::cell!($c78), "\t",
                flexion::cell!($c79), "\t",
                flexion::cell!($c80), "\t",
                flexion::cell!($c81), "\t",
                flexion::cell!($c82), "\t",
                flexion::cell!($c83), "\t",
                flexion::cell!($c84), "\t",
                flexion::cell!($c85), "\t",
                flexion::cell!($c86), "\t",
                flexion::cell!($c87), "\t",
                flexion::cell!($c88), "\t",
                flexion::cell!($c89), "\t",
                flexion::cell!($c90), "\t",
                flexion::cell!($c91), "\t",
                flexion::cell!($c92), "\t",
                flexion::cell!($c93), "\t",
                flexion::cell!($c94), "\t",
                flexion::cell!($c95), "\t",
                flexion::cell!($c96), "\t",
                flexion::cell!($c97), "\t",
                flexion::cell!($c98), "\t",
                flexion::cell!($c99), "\t",
                flexion::cell!($c100), "\t",
                flexion::cell!($c101), "\t",
                flexion::cell!($c102), "\t",
                flexion::cell!($c103), "\t",
                flexion::cell!($c104), "\t",
                flexion::cell!($c105), "\t",
                flexion::cell!($c106), "\t",
                flexion::cell!($c107), "\t",
                flexion::cell!($c108), "\t",
                flexion::cell!($c109), "\t",
                flexion::cell!($c110), "\t",
                flexion::cell!($c111), "\t",
                flexion::cell!($c112), "\t",
                flexion::cell!($c113), "\t",
                flexion::cell!($c114), "\t",
                flexion::cell!($c115), "\t",
                flexion::cell!($c116), "\t",
                flexion::cell!($c117), "\t",
                flexion::cell!($c118), "\t",
                flexion::cell!($c119), "\t",
                flexion::cell!($c120), "\t",
                flexion::cell!($c121), "\t",
                flexion::cell!($c122), "\t",
                flexion::cell!($c123), "\t",
                flexion::cell!($c124), "\t",
                flexion::cell!($c125), "\t",
                flexion::cell!($c126), "\t",
                flexion::cell!($c127), "\t",
                flexion::cell!($c128), "\t",
                flexion::cell!($c129), "\t",
                flexion::cell!($c130), "\t",
                flexion::cell!($c131), "\t",
                flexion::cell!($c132), "\t",
                flexion::cell!($c133), "\t",
                flexion::cell!($c134), "\t",
                flexion::cell!($c135), "\t",
                flexion::cell!($c136), "\t",
                flexion::cell!($c137), "\t",
                flexion::cell!($c138), "\t",
                flexion::cell!($c139), "\t",
                flexion::cell!($c140), "\t",
                flexion::cell!($c141), "\t",
                flexion::cell!($c142), "\t",
                flexion::cell!($c143), "\t",
                flexion::cell!($c144), "\t",
                flexion::cell!($c145), "\t",
                flexion::cell!($c146), "\t",
                flexion::cell!($c147), "\t",
                flexion::cell!($c148), "\t",
                flexion::cell!($c149), "\t",
                flexion::cell!($c150), "\t",
                flexion::cell!($c151), "\t",
                flexion::cell!($c152), "\t",
                flexion::cell!($c153), "\t",
                flexion::cell!($c154), "\t",
                flexion::cell!($c155), "\t",
                flexion::cell!($c156), "\t",
                flexion::cell!($c157), "\t",
                flexion::cell!($c158), "\t",
                flexion::cell!($c159), "\t",
                flexion::cell!($c160), "\t",
                flexion::cell!($c161), "\t",
                flexion::cell!($c162), "\t",
                flexion::cell!($c163), "\t",
                flexion::cell!($c164), "\t",
                flexion::cell!($c165), "\t",
                flexion::cell!($c166), "\t",
                flexion::cell!($c167), "\t",
                flexion::cell!($c168), "\t",
                flexion::cell!($c169), "\t",
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
    /// Like `वहति`: 272 words.
    वहति,
    /// Like `लभते`: 112 words.
    लभते,
    /// Like `अमत`: 107 words.
    अमत,
    /// Like `धरति`: 99 words.
    धरति,
    /// Like `एति`: 83 words.
    एति,
    /// Like `रमते`: 43 words.
    रमते,
    /// Like `आपत्`: 41 words.
    आपत,
    /// Like `ईरयति`: 28 words.
    ईरयति,
    /// Like `एष्यति`: 28 words.
    एषयति,
    /// Like `ददाति`: 19 words.
    ददाति,
    /// Like `अवति`: 17 words.
    अवति,
    /// Like `तनोति`: 16 words.
    तनोति,
    /// Like `आजीत्`: 11 words.
    आजीत,
    /// Like `धास्यति`: 10 words.
    धासयति,
    /// Like `अनंसीत्`: 9 words.
    अनंसीत,
    /// Like `भाति`: 8 words.
    भाति,
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
    pub const ALL: [Conjugation; 40] = [
        Conjugation::वहति,
        Conjugation::लभते,
        Conjugation::अमत,
        Conjugation::धरति,
        Conjugation::एति,
        Conjugation::रमते,
        Conjugation::आपत,
        Conjugation::ईरयति,
        Conjugation::एषयति,
        Conjugation::ददाति,
        Conjugation::अवति,
        Conjugation::तनोति,
        Conjugation::आजीत,
        Conjugation::धासयति,
        Conjugation::अनंसीत,
        Conjugation::भाति,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 40] = [
    // वहति: 272 words (निन्दति, खादति, क्षालयति, वयति, बोधति, भवति).
    conjugation!("वहति", 272, "~~ति";
        indicative_active_first: "~~ामि", "~~ावः", "~~ामः";
        indicative_active_second: "~~सि", "~~थः", "~~थ";
        indicative_active_third: "~~ति", "~~तः", "~~न्ति";
        indicative_middle_first: "~~े", "~~ावहे", "~~ामहे";
        indicative_middle_second: "~~से", "~~ेथे", "~~ध्वे";
        indicative_middle_third: "~~ते", "~~ेते", "~~न्ते";
        imperative_active_first: "~~ानि", "~~ाव", "~~ाम";
        imperative_active_second: "~~", "~~तम्", "~~त";
        imperative_active_third: "~~तु", "~~ताम्", "~~न्तु";
        imperative_middle_first: "~~ै", "~~ावहै", "~~ामहै";
        imperative_middle_second: "~~स्व", "~~ेथाम्", "~~ध्वम्";
        imperative_middle_third: "~~ताम्", "~~ेताम्", "~~न्ताम्";
        optative_active_first: "~~ेयम्", "~~ेव", "~~ेम";
        optative_active_second: "~~ेः", "~~ेतम्", "~~ेत";
        optative_active_third: "~~ेत्", "~~ेताम्", "~~ेयुः";
        optative_middle_first: "~~ेय", "~~ेवहि", "~~ेमहि";
        optative_middle_second: "~~ेथाः", "~~ेयाथाम्", "~~ेध्वम्";
        optative_middle_third: "~~ेत", "~~ेयाताम्", "~~ेरन्";
        subjunctive_active_first: "~~ानि", "~~ाव", "~~ाम";
        subjunctive_active_second: "~~ाः", "~~ाथः", "~~ाथ";
        subjunctive_active_third: "~~ात्", "~~ातः", "~~ान्";
        subjunctive_middle_first: "~~ै", "~~ावहै", "~~ामहै";
        subjunctive_middle_second: "~~ासे", "~~ैथे", "~~ाध्वै";
        subjunctive_middle_third: "~~ाते", "~~ैते", "~~न्त";
        imperfect_active_first: "अ~~म्", "अ~~ाव", "अ~~ाम";
        imperfect_active_second: "अ~~ः", "अ~~तम्", "अ~~त";
        imperfect_active_third: "अ~~त्", "अ~~ताम्", "अ~~न्";
        imperfect_middle_first: "अ~~े", "अ~~ावहि", "अ~~ामहि";
        imperfect_middle_second: "अ~~थाः", "अ~~ेथाम्", "अ~~ध्वम्";
        imperfect_middle_third: "अ~~त", "अ~~ेताम्", "अ~~न्त";
        future_active_first: "~~िष्यामि", "~~िष्यावः", "~~िष्यामः";
        future_active_second: "~~िष्यसि", "~~िष्यथः", "~~िष्यथ";
        future_active_third: "~~िष्यति", "~~िष्यतः", "~~िष्यन्ति";
        future_middle_first: "~~िष्ये", "~~िष्यावहे", "~~िष्यामहे";
        future_middle_second: "~~िष्यसे", "~~िष्येथे", "~~िष्यध्वे";
        future_middle_third: "~~िष्यते", "~~िष्येते", "~~िष्यन्ते";
        conditional_active_first: "अ~~िष्यम्", "अ~~िष्याव", "अ~~िष्याम";
        conditional_active_second: "अ~~िष्यः", "अ~~िष्यतम्", "अ~~िष्यत";
        conditional_active_third: "अ~~िष्यत्", "अ~~िष्यताम्", "अ~~िष्यन्";
        conditional_middle_first: "अ~~िष्ये", "अ~~िष्यावहि", "अ~~िष्यामहि";
        conditional_middle_second: "अ~~िष्यथाः", "अ~~िष्येथाम्", "अ~~िष्यध्वम्";
        conditional_middle_third: "अ~~िष्यत", "अ~~िष्येताम्", "अ~~िष्यन्त";
        perfect_active_first: "~िन~", "~िन~िव", "~िन~िम";
        perfect_active_second: "~िन~िथ", "~िन~थुः", "~िन~";
        perfect_active_third: "~िन~", "~िन~तुः", "~िन~ुः";
        perfect_middle_first: "~िन~े", "~िन~िवहे", "~िन~िमहे";
        perfect_middle_second: "~िन~िषे", "~िन~ाथे", "~िन~िध्वे";
        perfect_middle_third: "~िन~े", "~िन~ाते", "~िन~िरे";
        aorist_active_first: "अ~~िषम्", "अ~~िष्व", "अ~~िष्म";
        aorist_active_second: "अ~~ीः", "अ~~िष्टम्", "अ~~िष्ट";
        aorist_active_third: "अ~~ीत्", "अ~~िष्टाम्", "अ~~िषुः";
        aorist_middle_first: "अ~~िषि", "अ~~िष्वहि", "अ~~िष्महि";
        aorist_middle_second: "अ~~िष्ठाः", "अ~~िषाथाम्", "अ~~िढ्वम्";
        aorist_middle_third: "अ~~िष्ट", "अ~~िषाताम्", "अ~~िषत";
        infinitive: "~~ितुम्";
        gerund: "~~ित्वा";
        participle_present: "~~त्", "~~मान";
        participle_future: "~~िष्यत्", "~~िष्यमाण";
        participle_perfect: "~िन~्वांस्", "~िन~ान";
    ),
    // लभते: 112 words (वस्ते, सचते, रोचते, मन्यते, पर्दते, स्वेदते).
    conjugation!("लभते", 112, "~ते";
        indicative_active_first: "~ामि", "~ावः", "~ामः";
        indicative_active_second: "~सि", "~थः", "~थ";
        indicative_active_third: "~ति", "~तः", "~न्ति";
        indicative_middle_first: "~े", "~ावहे", "~ामहे";
        indicative_middle_second: "~से", "~ेथे", "~ध्वे";
        indicative_middle_third: "~ते", "~ेते", "~न्ते";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~", "~तम्", "~त";
        imperative_active_third: "~तु", "~ताम्", "~न्तु";
        imperative_middle_first: "~ै", "~ावहै", "~ामहै";
        imperative_middle_second: "~स्व", "~ेथाम्", "~ध्वम्";
        imperative_middle_third: "~ताम्", "~ेताम्", "~न्ताम्";
        optative_active_first: "~ेयम्", "~ेव", "~ेम";
        optative_active_second: "~ेः", "~ेतम्", "~ेत";
        optative_active_third: "~ेत्", "~ेताम्", "~ेयुः";
        optative_middle_first: "~ेय", "~ेवहि", "~ेमहि";
        optative_middle_second: "~ेथाः", "~ेयाथाम्", "~ेध्वम्";
        optative_middle_third: "~ेत", "~ेयाताम्", "~ेरन्";
        subjunctive_active_first: "~ानि", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात्", "~ातः", "~ान्";
        subjunctive_middle_first: "~ै", "~ावहै", "~ामहै";
        subjunctive_middle_second: "~ासे|~ासै", "~ैथे", "~ाध्वै";
        subjunctive_middle_third: "~ाते|~ातै", "~ैते", "~न्त|~ान्तै";
        imperfect_active_first: "अ~म्", "अ~ाव", "अ~ाम";
        imperfect_active_second: "अ~ः", "अ~तम्", "अ~त";
        imperfect_active_third: "अ~त्", "अ~ताम्", "अ~न्";
        imperfect_middle_first: "अ~े", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~थाः", "अ~ेथाम्", "अ~ध्वम्";
        imperfect_middle_third: "अ~त", "अ~ेताम्", "अ~न्त";
        future_active_first: "~िष्यामि", "~िष्यावः", "~िष्यामः";
        future_active_second: "~िष्यसि", "~िष्यथः", "~िष्यथ";
        future_active_third: "~िष्यति", "~िष्यतः", "~िष्यन्ति";
        future_middle_first: "~िष्ये", "~िष्यावहे", "~िष्यामहे";
        future_middle_second: "~िष्यसे", "~िष्येथे", "~िष्यध्वे";
        future_middle_third: "~िष्यते", "~िष्येते", "~िष्यन्ते";
        conditional_active_first: "अ~िष्यम्", "अ~िष्याव", "अ~िष्याम";
        conditional_active_second: "अ~िष्यः", "अ~िष्यतम्", "अ~िष्यत";
        conditional_active_third: "अ~िष्यत्", "अ~िष्यताम्", "अ~िष्यन्";
        conditional_middle_first: "अ~िष्ये", "अ~िष्यावहि", "अ~िष्यामहि";
        conditional_middle_second: "अ~िष्यथाः", "अ~िष्येथाम्", "अ~िष्यध्वम्";
        conditional_middle_third: "अ~िष्यत", "अ~िष्येताम्", "अ~िष्यन्त";
        perfect_active_first: "~ामास", "~ामासिव", "~ामासिम";
        perfect_active_second: "~ामासिथ", "~ामासथुः", "~ामास";
        perfect_active_third: "~ामास", "~ामासतुः", "~ामासुः";
        perfect_middle_first: "~ांचक्रे", "~ांचकृवहे", "~ांचकृमहे";
        perfect_middle_second: "~ांचकृषे", "~ांचक्राथे", "~ांचकृध्वे";
        perfect_middle_third: "~ांचक्रे", "~ांचक्राते", "~ांचक्रिरे";
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: "~ितुम्";
        gerund: "~ित्वा";
        participle_present: "~त्", "~मान";
        participle_future: "~िष्यत्", "~िष्यमाण";
        participle_perfect: "~ामासिवांस्", "~ांचक्राण";
    ),
    // अमत: 107 words (अमृत, वेद, देग्धि, द्वेष्टि, आस, ईष्टे).
    conjugation!("अमत", 107, "~";
        indicative_active_first: derived, derived, derived;
        indicative_active_second: derived, derived, derived;
        indicative_active_third: "~", derived, derived;
        indicative_middle_first: derived, derived, derived;
        indicative_middle_second: derived, derived, derived;
        indicative_middle_third: derived, derived, derived;
        imperative_active_first: derived, derived, derived;
        imperative_active_second: derived, derived, derived;
        imperative_active_third: derived, derived, derived;
        imperative_middle_first: derived, derived, derived;
        imperative_middle_second: derived, derived, derived;
        imperative_middle_third: derived, derived, derived;
        optative_active_first: derived, derived, derived;
        optative_active_second: derived, derived, derived;
        optative_active_third: derived, derived, derived;
        optative_middle_first: derived, derived, derived;
        optative_middle_second: derived, derived, derived;
        optative_middle_third: derived, derived, derived;
        subjunctive_active_first: derived, derived, derived;
        subjunctive_active_second: derived, derived, derived;
        subjunctive_active_third: derived, derived, derived;
        subjunctive_middle_first: derived, derived, derived;
        subjunctive_middle_second: derived, derived, derived;
        subjunctive_middle_third: derived, derived, derived;
        imperfect_active_first: derived, derived, derived;
        imperfect_active_second: derived, derived, derived;
        imperfect_active_third: derived, derived, derived;
        imperfect_middle_first: derived, derived, derived;
        imperfect_middle_second: derived, derived, derived;
        imperfect_middle_third: derived, derived, derived;
        future_active_first: derived, derived, derived;
        future_active_second: derived, derived, derived;
        future_active_third: derived, derived, derived;
        future_middle_first: derived, derived, derived;
        future_middle_second: derived, derived, derived;
        future_middle_third: derived, derived, derived;
        conditional_active_first: derived, derived, derived;
        conditional_active_second: derived, derived, derived;
        conditional_active_third: derived, derived, derived;
        conditional_middle_first: derived, derived, derived;
        conditional_middle_second: derived, derived, derived;
        conditional_middle_third: derived, derived, derived;
        perfect_active_first: "~", "~िव", "~िम";
        perfect_active_second: "~िथ", "~थुः", "~";
        perfect_active_third: "~", "~तुः", "~ुः";
        perfect_middle_first: "~े", "~िवहे", "~िमहे";
        perfect_middle_second: "~िषे", "~ाथे", "~िध्वे";
        perfect_middle_third: "~े", "~ाते", "~िरे";
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: derived;
        gerund: derived;
        participle_present: derived, derived;
        participle_future: derived, derived;
        participle_perfect: "~्वांस्", "~ान";
    ),
    // धरति: 99 words (भरति, भुरति, दुष्यति, द्रायति, ओषति, द्रुह्यति). Like वहति, except in 41 cells: imperative_active_first sg, subjunctive_active_first sg, perfect_active_first sg, perfect_active_first du, perfect_active_first pl, perfect_active_second sg, ….
    conjugation!("धरति", 99, "~~ति", like वहति;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: "~~ाणि", _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: "~~ाणि", _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: "~च~", "~े~िव", "~े~िम";
        perfect_active_second: "~च~्थ", "~े~थुः", "~े~";
        perfect_active_third: "~चा~", "~े~तुः", "~े~ुः";
        perfect_middle_first: "~े~े", "~े~िवहे", "~े~िमहे";
        perfect_middle_second: "~े~िषे", "~े~ाथे", "~े~िध्वे";
        perfect_middle_third: "~े~े", "~े~ाते", "~े~िरे";
        aorist_active_first: "अ~ा~िषम्", "अ~ा~िष्व", "अ~ा~िष्म";
        aorist_active_second: "अ~ा~ीः", "अ~ा~िष्टम्", "अ~ा~िष्ट";
        aorist_active_third: "अ~ा~ीत्", "अ~ा~िष्टाम्", "अ~ा~िषुः";
        aorist_middle_first: "अ~ा~िषि", "अ~ा~िष्वहि", "अ~ा~िष्महि";
        aorist_middle_second: "अ~ा~िष्ठाः", "अ~ा~िषाथाम्", "अ~ा~िढ्वम्";
        aorist_middle_third: "अ~ा~िष्ट", "अ~ा~िषाताम्", "अ~ा~िषत";
        infinitive: _;
        gerund: _;
        participle_present: _, "~~माण";
        participle_future: _, _;
        participle_perfect: "~े~िवांस्", "~े~ाण";
    ),
    // एति: 83 words (एति, दर्शयति, नाशयति, रन्धयति, जासयति, भाजयति). Like लभते, except in 15 cells: indicative_active_first pl, subjunctive_active_second sg, subjunctive_active_third sg, future_active_first pl, perfect_active_first sg, perfect_active_first du, ….
    conjugation!("एति", 83, "~ति", like लभते;
        indicative_active_first: _, _, "~ामः|~ामसि";
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: "~ाः|~ासि", _, _;
        subjunctive_active_third: "~ात्|~ाति", _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, "~िष्यामः|~िष्यामसि";
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: "~ामास|~ांचकर", "~ामासिव|~ांचकृव", "~ामासिम|~ांचकृम";
        perfect_active_second: "~ामासिथ|~ांचकर्थ", "~ामासथुः|~ांचक्रथुः", "~ामास|~ांचक्र";
        perfect_active_third: "~ामास|~ांचकार", "~ामासतुः|~ांचक्रतुः", "~ामासुः|~ांचक्रुः";
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, "~मान|~ान";
        participle_future: _, _;
        participle_perfect: "~ामासिवांस्|~ांचकृवांस्", _;
    ),
    // रमते: 43 words (प्रवते, म्रियते, भाषते, रमते, परीक्षते, त्रपते). Like लभते, except in 3 cells: imperative_active_first sg, subjunctive_active_first sg, participle_present mid.
    conjugation!("रमते", 43, "~ते", like लभते;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: "~ाणि", _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: "~ाणि", _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, "~माण";
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // आपत: 41 words (अगमत्, अद्राक्षीत्, अश्रोत्, अभूत्, अवोचत्, अलेकिष्यत्). Like अमत, except in 39 cells: indicative_active_third sg, perfect_active_first sg, perfect_active_first du, perfect_active_first pl, perfect_active_second sg, perfect_active_second du, ….
    conjugation!("आपत्", 41, "~त्", like अमत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~त्", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: derived, derived, derived;
        perfect_middle_second: derived, derived, derived;
        perfect_middle_third: derived, derived, derived;
        aorist_active_first: "~म्", "~ाव", "~ाम";
        aorist_active_second: "~ः", "~तम्", "~त";
        aorist_active_third: "~त्", "~ताम्", "~न्";
        aorist_middle_first: "~े", "~ावहि", "~ामहि";
        aorist_middle_second: "~थाः", "~ेथाम्", "~ध्वम्";
        aorist_middle_third: "~त", "~ेताम्", "~न्त";
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: derived, derived;
    ),
    // ईरयति: 28 words (बर्हयति, धारयति, रोपयति, कारयति, शोषयति, चोरयति). Like एति, except in 3 cells: imperative_active_first sg, subjunctive_active_first sg, participle_present mid.
    conjugation!("ईरयति", 28, "~ति", like एति;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: "~ाणि", _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: "~ाणि", _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, "~माण|~ाण";
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // एषयति: 28 words (करिष्यति, गमिष्यति, भविष्यति, भरिष्यति, अञ्चिष्यति, वक्ष्यति). Like आपत, except in 57 cells: indicative_active_third sg, future_active_first sg, future_active_first du, future_active_first pl, future_active_second sg, future_active_second du, ….
    conjugation!("एष्यति", 28, "~ति", like आपत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ति", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: "~ामि", "~ावः", "~ामः|~ामसि";
        future_active_second: "~सि", "~थः", "~थ";
        future_active_third: "~ति", "~तः", "~न्ति";
        future_middle_first: "~े", "~ावहे", "~ामहे";
        future_middle_second: "~से", "~ेथे", "~ध्वे";
        future_middle_third: "~ते", "~ेते", "~न्ते";
        conditional_active_first: "अ~म्", "अ~ाव", "अ~ाम";
        conditional_active_second: "अ~ः", "अ~तम्", "अ~त";
        conditional_active_third: "अ~त्", "अ~ताम्", "अ~न्";
        conditional_middle_first: "अ~े", "अ~ावहि", "अ~ामहि";
        conditional_middle_second: "अ~थाः", "अ~ेथाम्", "अ~ध्वम्";
        conditional_middle_third: "अ~त", "अ~ेताम्", "अ~न्त";
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: "~त्", "~माण";
        participle_perfect: _, _;
    ),
    // ददाति: 19 words (ददाति, दधाति, जानाति, प्रीणाति, श्रद्दधाति, गृणाति).
    conjugation!("ददाति", 19, "~ाति";
        indicative_active_first: "~ामि", "~ीवः", "~ीमः|~ीमसि";
        indicative_active_second: "~ासि", "~ीथः", "~ीथ";
        indicative_active_third: "~ाति", "~ीतः", "~न्ति";
        indicative_middle_first: "~े", "~ीवहे", "~ीमहे";
        indicative_middle_second: "~ीषे", "~ाथे", "~ीध्वे";
        indicative_middle_third: "~ीते", "~ाते", "~ते";
        imperative_active_first: "~ानि", "~ाव", "~ाम";
        imperative_active_second: "~ीहि", "~ीतम्", "~ीत";
        imperative_active_third: "~ातु", "~ीताम्", "~न्तु";
        imperative_middle_first: "~ै", "~ावहै", "~ामहै";
        imperative_middle_second: "~ीष्व", "~ाथाम्", "~ीध्वम्";
        imperative_middle_third: "~ीताम्", "~ाताम्", "~ताम्";
        optative_active_first: "~ीयाम्", "~ीयाव", "~ीयाम";
        optative_active_second: "~ीयाः", "~ीयातम्", "~ीयात";
        optative_active_third: "~ीयात्", "~ीयाताम्", "~ीयुः";
        optative_middle_first: "~ीय", "~ीवहि", "~ीमहि";
        optative_middle_second: "~ीथाः", "~ीयाथाम्", derived;
        optative_middle_third: "~ीत", "~ीयाताम्", "~ीरन्";
        subjunctive_active_first: "~ानि|~ा", "~ाव", "~ाम";
        subjunctive_active_second: "~ाः|~ासि", "~ाथः", "~ाथ";
        subjunctive_active_third: "~ात्|~ाति", "~ातः", "~ान्";
        subjunctive_middle_first: "~ै", "~ावहै", "~ामहै";
        subjunctive_middle_second: "~ासे|~ासै", "~ैथे", "~ाध्वे|~ाध्वै";
        subjunctive_middle_third: "~ाते|~ातै", "~ैते", "~ान्त|~ान्तै";
        imperfect_active_first: "अ~ाम्", "अ~ीव", "अ~ीम";
        imperfect_active_second: "अ~ाः", "अ~ीतम्", "अ~ीत";
        imperfect_active_third: "अ~ात्", "अ~ीताम्", "अ~न्";
        imperfect_middle_first: "अ~ि", "अ~ीवहि", "अ~ीमहि";
        imperfect_middle_second: "अ~ीथाः", "अ~ाथाम्", "अ~ीध्वम्";
        imperfect_middle_third: "अ~ीत", "अ~ाताम्", "अ~त";
        future_active_first: derived, derived, derived;
        future_active_second: derived, derived, derived;
        future_active_third: derived, derived, derived;
        future_middle_first: derived, derived, derived;
        future_middle_second: derived, derived, derived;
        future_middle_third: derived, derived, derived;
        conditional_active_first: derived, derived, derived;
        conditional_active_second: derived, derived, derived;
        conditional_active_third: derived, derived, derived;
        conditional_middle_first: derived, derived, derived;
        conditional_middle_second: derived, derived, derived;
        conditional_middle_third: derived, derived, derived;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: derived, derived, derived;
        perfect_middle_second: derived, derived, derived;
        perfect_middle_third: derived, derived, derived;
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: derived;
        gerund: derived;
        participle_present: "~त्", "~ान";
        participle_future: derived, derived;
        participle_perfect: derived, derived;
    ),
    // अवति: 17 words (अस्ति, अत्ति, अजति, अवति, अर्हति, अर्चति).
    conjugation!("अवति", 17, "अ~ति";
        indicative_active_first: "अ~ामि", "अ~ावः", "अ~ामः|अ~ामसि";
        indicative_active_second: "अ~सि", "अ~थः", "अ~थ";
        indicative_active_third: "अ~ति", "अ~तः", "अ~न्ति";
        indicative_middle_first: "अ~े", "अ~ावहे", "अ~ामहे";
        indicative_middle_second: "अ~से", "अ~ेथे", "अ~ध्वे";
        indicative_middle_third: "अ~ते", "अ~ेते", "अ~न्ते";
        imperative_active_first: "अ~ानि", "अ~ाव", "अ~ाम";
        imperative_active_second: "अ~", "अ~तम्", "अ~त";
        imperative_active_third: "अ~तु", "अ~ताम्", "अ~न्तु";
        imperative_middle_first: "अ~ै", "अ~ावहै", "अ~ामहै";
        imperative_middle_second: "अ~स्व", "अ~ेथाम्", "अ~ध्वम्";
        imperative_middle_third: "अ~ताम्", "अ~ेताम्", "अ~न्ताम्";
        optative_active_first: "अ~ेयम्", "अ~ेव", "अ~ेम";
        optative_active_second: "अ~ेः", "अ~ेतम्", "अ~ेत";
        optative_active_third: "अ~ेत्", "अ~ेताम्", "अ~ेयुः";
        optative_middle_first: "अ~ेय", "अ~ेवहि", "अ~ेमहि";
        optative_middle_second: "अ~ेथाः", "अ~ेयाथाम्", "अ~ेध्वम्";
        optative_middle_third: "अ~ेत", "अ~ेयाताम्", "अ~ेरन्";
        subjunctive_active_first: "अ~ानि", "अ~ाव", "अ~ाम";
        subjunctive_active_second: "अ~ाः|अ~ासि", "अ~ाथः", "अ~ाथ";
        subjunctive_active_third: "अ~ात्|अ~ाति", "अ~ातः", "अ~ान्";
        subjunctive_middle_first: "अ~ै", "अ~ावहै", "अ~ामहै";
        subjunctive_middle_second: "अ~ासे|अ~ासै", "अ~ैथे", "अ~ाध्वै";
        subjunctive_middle_third: "अ~ाते|अ~ातै", "अ~ैते", "अ~न्त|अ~ान्तै";
        imperfect_active_first: "आ~म्", "आ~ाव", "आ~ाम";
        imperfect_active_second: "आ~ः", "आ~तम्", "आ~त";
        imperfect_active_third: "आ~त्", "आ~ताम्", "आ~न्";
        imperfect_middle_first: "आ~े", "आ~ावहि", "आ~ामहि";
        imperfect_middle_second: "आ~थाः", "आ~ेथाम्", "आ~ध्वम्";
        imperfect_middle_third: "आ~त", "आ~ेताम्", "आ~न्त";
        future_active_first: "अ~िष्यामि", "अ~िष्यावः", "अ~िष्यामः";
        future_active_second: "अ~िष्यसि", "अ~िष्यथः", "अ~िष्यथ";
        future_active_third: "अ~िष्यति", "अ~िष्यतः", "अ~िष्यन्ति";
        future_middle_first: "अ~िष्ये", "अ~िष्यावहे", "अ~िष्यामहे";
        future_middle_second: "अ~िष्यसे", "अ~िष्येथे", "अ~िष्यध्वे";
        future_middle_third: "अ~िष्यते", "अ~िष्येते", "अ~िष्यन्ते";
        conditional_active_first: "आ~िष्यम्", "आ~िष्याव", "आ~िष्याम";
        conditional_active_second: "आ~िष्यः", "आ~िष्यतम्", "आ~िष्यत";
        conditional_active_third: "आ~िष्यत्", "आ~िष्यताम्", "आ~िष्यन्";
        conditional_middle_first: "आ~िष्ये", "आ~िष्यावहि", "आ~िष्यामहि";
        conditional_middle_second: "आ~िष्यथाः", "आ~िष्येथाम्", "आ~िष्यध्वम्";
        conditional_middle_third: "आ~िष्यत", "आ~िष्येताम्", "आ~िष्यन्त";
        perfect_active_first: "आ~", "आ~िव", "आ~िम";
        perfect_active_second: "आ~िथ", "आ~थुः", "आ~";
        perfect_active_third: "आ~", "आ~तुः", "आ~ुः";
        perfect_middle_first: "आ~े", "आ~िवहे", "आ~िमहे";
        perfect_middle_second: "आ~िषे", "आ~ाथे", "आ~िध्वे";
        perfect_middle_third: "आ~े", "आ~ाते", "आ~िरे";
        aorist_active_first: "आ~िषम्", "आ~िष्व", "आ~िष्म";
        aorist_active_second: "आ~ीः", "आ~िष्टम्", "आ~िष्ट";
        aorist_active_third: "आ~ीत्", "आ~िष्टाम्", "आ~िषुः";
        aorist_middle_first: "आ~िषि", "आ~िष्वहि", "आ~िष्महि";
        aorist_middle_second: "आ~िष्ठाः", "आ~िषाथाम्", "आ~िढ्वम्";
        aorist_middle_third: "आ~िष्ट", "आ~िषाताम्", "आ~िषत";
        infinitive: "अ~ितुम्";
        gerund: "अ~ित्वा";
        participle_present: "अ~त्", "अ~मान";
        participle_future: "अ~िष्यत्", "अ~िष्यमाण";
        participle_perfect: "आ~िवांस्", "आ~ान";
    ),
    // तनोति: 16 words (वृणोति, तनोति, करोति, जुहोति, शृणोति, घृणोति).
    conjugation!("तनोति", 16, "~ोति";
        indicative_active_first: "~ोमि", "~्वः", "~्मः";
        indicative_active_second: "~ोषि", "~ुथः", "~ुथ";
        indicative_active_third: "~ोति", "~ुतः", "~्वन्ति";
        indicative_middle_first: "~्वे", "~्वहे", "~्महे";
        indicative_middle_second: "~ुषे", "~्वाथे", "~ुध्वे";
        indicative_middle_third: "~ुते", "~्वाते", "~्वते";
        imperative_active_first: "~वानि", "~वाव", "~वाम";
        imperative_active_second: "~ु", "~ुतम्", "~ुत";
        imperative_active_third: "~ोतु", "~ुताम्", "~्वन्तु";
        imperative_middle_first: "~वै", "~वावहै", "~वामहै";
        imperative_middle_second: "~ुष्व", "~्वाथाम्", "~ुध्वम्";
        imperative_middle_third: "~ुताम्", "~्वाताम्", "~्वताम्";
        optative_active_first: "~ुयाम्", "~ुयाव", "~ुयाम";
        optative_active_second: "~ुयाः", "~ुयातम्", "~ुयात";
        optative_active_third: "~ुयात्", "~ुयाताम्", "~ुयुः";
        optative_middle_first: "~्वीय", "~्वीवहि", "~्वीमहि";
        optative_middle_second: "~्वीथाः", "~्वीयाथाम्", "~्वीध्वम्";
        optative_middle_third: "~्वीत", "~्वीयाताम्", "~्वीरन्";
        subjunctive_active_first: "~वानि", "~वाव", "~वाम";
        subjunctive_active_second: "~वः", "~वथः", "~वथ";
        subjunctive_active_third: "~वत्", "~वतः", "~वन्";
        subjunctive_middle_first: "~वै", "~वावहै", "~वामहै";
        subjunctive_middle_second: "~वसे", "~वैथे", "~वध्वे";
        subjunctive_middle_third: "~वते", "~वैते", "~वन्त";
        imperfect_active_first: "अ~वम्", "अ~्व", "अ~्म";
        imperfect_active_second: "अ~ोः", "अ~ुतम्", "अ~ुत";
        imperfect_active_third: "अ~ोत्", "अ~ुताम्", "अ~्वन्";
        imperfect_middle_first: "अ~्वि", "अ~्वहि", "अ~्महि";
        imperfect_middle_second: "अ~ुथाः", "अ~्वाथाम्", "अ~ुध्वम्";
        imperfect_middle_third: "अ~ुत", "अ~्वाताम्", "अ~्वत";
        future_active_first: derived, derived, derived;
        future_active_second: derived, derived, derived;
        future_active_third: derived, derived, derived;
        future_middle_first: derived, derived, derived;
        future_middle_second: derived, derived, derived;
        future_middle_third: derived, derived, derived;
        conditional_active_first: derived, derived, derived;
        conditional_active_second: derived, derived, derived;
        conditional_active_third: derived, derived, derived;
        conditional_middle_first: derived, derived, derived;
        conditional_middle_second: derived, derived, derived;
        conditional_middle_third: derived, derived, derived;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: derived, derived, derived;
        perfect_middle_second: derived, derived, derived;
        perfect_middle_third: derived, derived, derived;
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: derived;
        gerund: derived;
        participle_present: "~्वत्", "~्वान";
        participle_future: derived, derived;
        participle_perfect: derived, derived;
    ),
    // आजीत: 11 words (अस्वारीत्, आजीत्, अकाषीत्, आञ्चीत्, अलेखीत्, अजिवीत्). Like आपत, except in 19 cells: indicative_active_third sg, aorist_active_first sg, aorist_active_first du, aorist_active_first pl, aorist_active_second sg, aorist_active_second du, ….
    conjugation!("आजीत्", 11, "~ीत्", like आपत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ीत्", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: "~िषम्", "~िष्व", "~िष्म";
        aorist_active_second: "~ीः", "~िष्टम्", "~िष्ट";
        aorist_active_third: "~ीत्", "~िष्टाम्", "~िषुः";
        aorist_middle_first: "~िषि", "~िष्वहि", "~िष्महि";
        aorist_middle_second: "~िष्ठाः", "~िषाथाम्", "~िढ्वम्";
        aorist_middle_third: "~िष्ट", "~िषाताम्", "~िषत";
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // धासयति: 10 words (पास्यति, तप्स्यति, ज्ञास्यति, नोत्स्यति, सत्स्यति, वत्स्यति). Like एषयति, except in 1 cell: participle_future mid.
    conjugation!("धास्यति", 10, "~ति", like एषयति;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, "~मान";
        participle_perfect: _, _;
    ),
    // अनंसीत: 9 words (अताप्सीत्, अहासीत्, अज्ञासीत्, अनौत्सीत्, अरौत्सीत्, अवात्सीत्). Like आपत, except in 19 cells: indicative_active_third sg, aorist_active_first sg, aorist_active_first du, aorist_active_first pl, aorist_active_second sg, aorist_active_second du, ….
    conjugation!("अनंसीत्", 9, "~सीत्", like आपत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~सीत्", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: "~सम्", "~स्व", "~स्म";
        aorist_active_second: "~सीः", "~स्तम्", "~स्त";
        aorist_active_third: "~सीत्", "~स्ताम्", "~सुः";
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // भाति: 8 words (भाति, द्राति, वाति, पाति, स्नाति, याति). Like ददाति, except in 54 cells: indicative_active_first du, indicative_active_first pl, indicative_active_second du, indicative_active_second pl, indicative_active_third du, indicative_active_third pl, ….
    conjugation!("भाति", 8, "~ाति", like ददाति;
        indicative_active_first: _, "~ावः", "~ामः|~ामसि";
        indicative_active_second: _, "~ाथः", "~ाथ";
        indicative_active_third: _, "~ातः", "~ान्ति";
        indicative_middle_first: "~ै", "~ावहे", "~ामहे";
        indicative_middle_second: "~ासे", _, "~ाध्वे";
        indicative_middle_third: "~ाते", _, "~ाते";
        imperative_active_first: _, _, _;
        imperative_active_second: "~ाहि", "~ातम्", "~ात";
        imperative_active_third: _, "~ाताम्", "~ान्तु";
        imperative_middle_first: _, _, _;
        imperative_middle_second: "~ास्व", _, "~ाध्वम्";
        imperative_middle_third: "~ाताम्", _, "~ाताम्";
        optative_active_first: "~ायाम्", "~ायाव", "~ायाम";
        optative_active_second: "~ायाः", "~ायातम्", "~ायात";
        optative_active_third: "~ायात्", "~ायाताम्", "~ायुः";
        optative_middle_first: "~ेय", "~ेवहि", "~ेमहि";
        optative_middle_second: "~ेथाः", "~ेयाथाम्", "~ेध्वम्";
        optative_middle_third: "~ेत", "~ेयाताम्", "~ेरन्";
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, "अ~ाव", "अ~ाम";
        imperfect_active_second: _, "अ~ातम्", "अ~ात";
        imperfect_active_third: _, "अ~ाताम्", "अ~ुः|अ~ान्";
        imperfect_middle_first: "अ~े", "अ~ावहि", "अ~ामहि";
        imperfect_middle_second: "अ~ाथाः", _, "अ~ाध्वम्";
        imperfect_middle_third: "अ~ात", _, "अ~ात";
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: "~ात्", _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // आपनोति: 8 words (धृष्णोति, शक्नोति, साध्नोति, स्तिघ्नोति, प्रुष्णोति, आप्नोति). Like तनोति, except in 34 cells: indicative_active_first du, indicative_active_first pl, indicative_active_third pl, indicative_middle_first sg, indicative_middle_first du, indicative_middle_first pl, ….
    conjugation!("आप्नोति", 8, "~ोति", like तनोति;
        indicative_active_first: _, "~ुवः", "~ुमः";
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, "~ुवन्ति";
        indicative_middle_first: "~ुवे", "~ुवहे", "~ुमहे";
        indicative_middle_second: _, "~ुवाथे", _;
        indicative_middle_third: _, "~ुवाते", "~ुवते";
        imperative_active_first: _, _, _;
        imperative_active_second: "~ुहि", _, _;
        imperative_active_third: _, _, "~ुवन्तु";
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, "~ुवाथाम्", _;
        imperative_middle_third: _, "~ुवाताम्", "~ुवताम्";
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: "~ुवीय", "~ुवीवहि", "~ुवीमहि";
        optative_middle_second: "~ुवीथाः", "~ुवीयाथाम्", "~ुवीध्वम्";
        optative_middle_third: "~ुवीत", "~ुवीयाताम्", "~ुवीरन्";
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, "अ~ुव", "अ~ुम";
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, "अ~ुवन्";
        imperfect_middle_first: "अ~ुवि", "अ~ुवहि", "अ~ुमहि";
        imperfect_middle_second: _, "अ~ुवाथाम्", _;
        imperfect_middle_third: _, "अ~ुवाताम्", "अ~ुवत";
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: "~ुवत्", "~ुवान";
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // बधनाति: 7 words (गृभ्णाति, उभ्नाति, गृह्णाति, इष्णाति, बध्नाति, स्तभ्नाति).
    conjugation!("बध्नाति", 7, "~्~ाति";
        indicative_active_first: "~्~ामि", "~्~ीवः", "~्~ीमः|~्~ीमसि";
        indicative_active_second: "~्~ासि", "~्~ीथः", "~्~ीथ";
        indicative_active_third: "~्~ाति", "~्~ीतः", "~्~न्ति";
        indicative_middle_first: "~्~े", "~्~ीवहे", "~्~ीमहे";
        indicative_middle_second: "~्~ीषे", "~्~ाथे", "~्~ीध्वे";
        indicative_middle_third: "~्~ीते", "~्~ाते", "~्~ते";
        imperative_active_first: "~्~ानि", "~्~ाव", "~्~ाम";
        imperative_active_second: "~ा~", "~्~ीतम्", "~्~ीत";
        imperative_active_third: "~्~ातु", "~्~ीताम्", "~्~न्तु";
        imperative_middle_first: "~्~ै", "~्~ावहै", "~्~ामहै";
        imperative_middle_second: "~्~ीष्व", "~्~ाथाम्", "~्~ीध्वम्";
        imperative_middle_third: "~्~ीताम्", "~्~ाताम्", "~्~ताम्";
        optative_active_first: "~्~ीयाम्", "~्~ीयाव", "~्~ीयाम";
        optative_active_second: "~्~ीयाः", "~्~ीयातम्", "~्~ीयात";
        optative_active_third: "~्~ीयात्", "~्~ीयाताम्", "~्~ीयुः";
        optative_middle_first: "~्~ीय", "~्~ीवहि", "~्~ीमहि";
        optative_middle_second: "~्~ीथाः", "~्~ीयाथाम्", derived;
        optative_middle_third: "~्~ीत", "~्~ीयाताम्", "~्~ीरन्";
        subjunctive_active_first: "~्~ानि|~्~ा", "~्~ाव", "~्~ाम";
        subjunctive_active_second: "~्~ाः|~्~ासि", "~्~ाथः", "~्~ाथ";
        subjunctive_active_third: "~्~ात्|~्~ाति", "~्~ातः", "~्~ान्";
        subjunctive_middle_first: "~्~ै", "~्~ावहै", "~्~ामहै";
        subjunctive_middle_second: "~्~ासे|~्~ासै", "~्~ैथे", "~्~ाध्वे|~्~ाध्वै";
        subjunctive_middle_third: "~्~ाते|~्~ातै", "~्~ैते", "~्~ान्त|~्~ान्तै";
        imperfect_active_first: "अ~्~ाम्", "अ~्~ीव", "अ~्~ीम";
        imperfect_active_second: "अ~्~ाः", "अ~्~ीतम्", "अ~्~ीत";
        imperfect_active_third: "अ~्~ात्", "अ~्~ीताम्", "अ~्~न्";
        imperfect_middle_first: "अ~्~ि", "अ~्~ीवहि", "अ~्~ीमहि";
        imperfect_middle_second: "अ~्~ीथाः", "अ~्~ाथाम्", "अ~्~ीध्वम्";
        imperfect_middle_third: "अ~्~ीत", "अ~्~ाताम्", "अ~्~त";
        future_active_first: derived, derived, derived;
        future_active_second: derived, derived, derived;
        future_active_third: derived, derived, derived;
        future_middle_first: derived, derived, derived;
        future_middle_second: derived, derived, derived;
        future_middle_third: derived, derived, derived;
        conditional_active_first: derived, derived, derived;
        conditional_active_second: derived, derived, derived;
        conditional_active_third: derived, derived, derived;
        conditional_middle_first: derived, derived, derived;
        conditional_middle_second: derived, derived, derived;
        conditional_middle_third: derived, derived, derived;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: derived, derived, derived;
        perfect_middle_second: derived, derived, derived;
        perfect_middle_third: derived, derived, derived;
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: derived;
        gerund: derived;
        participle_present: "~्~त्", "~्~ान";
        participle_future: derived, derived;
        participle_perfect: derived, derived;
    ),
    // ददौ: 7 words (तस्थौ, जज्ञौ, ददौ, पपौ, दधौ, ररौ). Like अमत, except in 5 cells: indicative_active_third sg, perfect_active_first sg, perfect_active_second sg, perfect_active_third sg, participle_perfect act.
    conjugation!("ददौ", 7, "~ौ", like अमत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ौ", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: "~ौ", _, _;
        perfect_active_second: "~ाथ|~िथ", _, _;
        perfect_active_third: "~ौ", _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: "~िवांस्", _;
    ),
    // पिपरति: 7 words (पिपर्ति, जागर्ति, बिभर्ति, जिघर्ति, इयर्ति, सिसर्ति).
    conjugation!("पिपर्ति", 7, "~र्ति";
        indicative_active_first: "~र्मि", "~ृवः", "~ृमः|~ृमसि";
        indicative_active_second: "~र्षि", "~ृथः", "~ृथ";
        indicative_active_third: "~र्ति", "~ृतः", "~्रति";
        indicative_middle_first: "~्रे", "~ृवहे", "~ृमहे";
        indicative_middle_second: "~ृषे", "~्राथे", "~ृध्वे";
        indicative_middle_third: "~ृते", "~्राते", "~्रते";
        imperative_active_first: "~राणि", "~राव", "~राम";
        imperative_active_second: "~ृहि", "~ृतम्", "~ृत";
        imperative_active_third: "~र्तु", "~ृताम्", "~्रतु";
        imperative_middle_first: "~रै", "~रावहै", "~रामहै";
        imperative_middle_second: "~ृष्व", "~्राथाम्", "~ृध्वम्";
        imperative_middle_third: "~ृताम्", "~्राताम्", "~्रताम्";
        optative_active_first: "~ृयाम्", "~ृयाव", "~ृयाम";
        optative_active_second: "~ृयाः", "~ृयातम्", "~ृयात";
        optative_active_third: "~ृयात्", "~ृयाताम्", "~ृयुः";
        optative_middle_first: "~्रीय", "~्रीवहि", "~्रीमहि";
        optative_middle_second: "~्रीथाः", "~्रीयाथाम्", "~्रीध्वम्";
        optative_middle_third: "~्रीत", "~्रीयाताम्", "~्रीरन्";
        subjunctive_active_first: "~राणि|~रा", "~राव", "~राम";
        subjunctive_active_second: "~रः|~रसि", "~रथः", "~रथ";
        subjunctive_active_third: "~रत्|~रति", "~रतः", "~रन्";
        subjunctive_middle_first: "~रै", "~रावहै", "~रामहै";
        subjunctive_middle_second: "~रसे", "~रैथे", "~रध्वे";
        subjunctive_middle_third: "~रते", "~रैते", "~रन्त";
        imperfect_active_first: "अ~रम्", "अ~ृव", "अ~ृम";
        imperfect_active_second: "अ~ः", "अ~ृतम्", "अ~ृत";
        imperfect_active_third: "अ~ः", "अ~ृताम्", "अ~रुः";
        imperfect_middle_first: "अ~्रि", "अ~ृवहि", "अ~ृमहि";
        imperfect_middle_second: "अ~ृथाः", "अ~्राथाम्", "अ~ृध्वम्";
        imperfect_middle_third: "अ~ृत", "अ~्राताम्", "अ~्रत";
        future_active_first: derived, derived, derived;
        future_active_second: derived, derived, derived;
        future_active_third: derived, derived, derived;
        future_middle_first: derived, derived, derived;
        future_middle_second: derived, derived, derived;
        future_middle_third: derived, derived, derived;
        conditional_active_first: derived, derived, derived;
        conditional_active_second: derived, derived, derived;
        conditional_active_third: derived, derived, derived;
        conditional_middle_first: derived, derived, derived;
        conditional_middle_second: derived, derived, derived;
        conditional_middle_third: derived, derived, derived;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: derived, derived, derived;
        perfect_middle_second: derived, derived, derived;
        perfect_middle_third: derived, derived, derived;
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: derived;
        gerund: derived;
        participle_present: "~्रत्", "~्राण";
        participle_future: derived, derived;
        participle_perfect: derived, derived;
    ),
    // भनकति: 6 words (भनक्ति, युनक्ति, विनक्ति, तनक्ति, अनक्ति, भुनक्ति).
    conjugation!("भनक्ति", 6, "~नक्ति";
        indicative_active_first: "~नज्मि", "~ञ्ज्वः", "~ञ्ज्मः";
        indicative_active_second: "~नक्षि", "~ङ्क्थः", "~ङ्क्थ";
        indicative_active_third: "~नक्ति", "~ङ्क्तः", "~ञ्जन्ति";
        indicative_middle_first: "~ञ्जे", "~ञ्ज्वहे", "~ञ्ज्महे";
        indicative_middle_second: "~ङ्क्षे", "~ञ्जाथे", "~ङ्ग्ध्वे";
        indicative_middle_third: "~ङ्क्ते", "~ञ्जाते", "~ञ्जते";
        imperative_active_first: "~नजानि", "~नजाव", "~नजाम";
        imperative_active_second: "~ङ्ग्धि", "~ङ्क्तम्", "~ङ्क्त";
        imperative_active_third: "~नक्तु", "~ङ्क्ताम्", "~ञ्जन्तु";
        imperative_middle_first: "~नजै", "~नजावहै", "~नजामहै";
        imperative_middle_second: "~ङ्क्ष्व", "~ञ्जाथाम्", "~ङ्ग्ध्वम्";
        imperative_middle_third: "~ङ्क्ताम्", "~ञ्जाताम्", "~ञ्जताम्";
        optative_active_first: "~ञ्ज्याम्", "~ञ्ज्याव", "~ञ्ज्याम";
        optative_active_second: "~ञ्ज्याः", "~ञ्ज्यातम्", "~ञ्ज्यात";
        optative_active_third: "~ञ्ज्यात्", "~ञ्ज्याताम्", "~ञ्ज्युः";
        optative_middle_first: "~ञ्जीय", "~ञ्जीवहि", "~ञ्जीमहि";
        optative_middle_second: "~ञ्जीथाः", "~ञ्जीयाथाम्", "~ञ्जीध्वम्";
        optative_middle_third: "~ञ्जीत", "~ञ्जीयाताम्", "~ञ्जीरन्";
        subjunctive_active_first: "~नजानि", "~नजाव", "~नजाम";
        subjunctive_active_second: "~नजः", "~नजथः", "~नजथ";
        subjunctive_active_third: "~नजत्", "~नजतः", "~नजन्";
        subjunctive_middle_first: "~नजै", "~नजावहै", "~नजामहै";
        subjunctive_middle_second: "~नजसे", "~नजैथे", "~नजध्वे";
        subjunctive_middle_third: "~नजते", "~नजैते", "~नजन्त";
        imperfect_active_first: "अ~नजम्", "अ~ञ्ज्व", "अ~ञ्ज्म";
        imperfect_active_second: "अ~नक्", "अ~ङ्क्तम्", "अ~ङ्क्त";
        imperfect_active_third: "अ~नक्", "अ~ङ्क्ताम्", "अ~ञ्जन्";
        imperfect_middle_first: "अ~ञ्जि", "अ~ञ्ज्वहि", "अ~ञ्ज्महि";
        imperfect_middle_second: "अ~ङ्क्थाः", "अ~ञ्जाथाम्", "अ~ङ्ग्ध्वम्";
        imperfect_middle_third: "अ~ङ्क्त", "अ~ञ्जाताम्", "अ~ञ्जत";
        future_active_first: derived, derived, derived;
        future_active_second: derived, derived, derived;
        future_active_third: derived, derived, derived;
        future_middle_first: derived, derived, derived;
        future_middle_second: derived, derived, derived;
        future_middle_third: derived, derived, derived;
        conditional_active_first: derived, derived, derived;
        conditional_active_second: derived, derived, derived;
        conditional_active_third: derived, derived, derived;
        conditional_middle_first: derived, derived, derived;
        conditional_middle_second: derived, derived, derived;
        conditional_middle_third: derived, derived, derived;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: derived, derived, derived;
        perfect_middle_second: derived, derived, derived;
        perfect_middle_third: derived, derived, derived;
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: derived;
        gerund: derived;
        participle_present: "~ञ्जत्", "~ञ्जान";
        participle_future: derived, derived;
        participle_perfect: derived, derived;
    ),
    // उपविशति: 6 words (उच्यति, उज्झति, उच्छति, उपविशति, उक्षति, उड्डापयति).
    conjugation!("उपविशति", 6, "उ~ति";
        indicative_active_first: "उ~ामि", "उ~ावः", "उ~ामः|उ~ामसि";
        indicative_active_second: "उ~सि", "उ~थः", "उ~थ";
        indicative_active_third: "उ~ति", "उ~तः", "उ~न्ति";
        indicative_middle_first: "उ~े", "उ~ावहे", "उ~ामहे";
        indicative_middle_second: "उ~से", "उ~ेथे", "उ~ध्वे";
        indicative_middle_third: "उ~ते", "उ~ेते", "उ~न्ते";
        imperative_active_first: "उ~ानि", "उ~ाव", "उ~ाम";
        imperative_active_second: "उ~", "उ~तम्", "उ~त";
        imperative_active_third: "उ~तु", "उ~ताम्", "उ~न्तु";
        imperative_middle_first: "उ~ै", "उ~ावहै", "उ~ामहै";
        imperative_middle_second: "उ~स्व", "उ~ेथाम्", "उ~ध्वम्";
        imperative_middle_third: "उ~ताम्", "उ~ेताम्", "उ~न्ताम्";
        optative_active_first: "उ~ेयम्", "उ~ेव", "उ~ेम";
        optative_active_second: "उ~ेः", "उ~ेतम्", "उ~ेत";
        optative_active_third: "उ~ेत्", "उ~ेताम्", "उ~ेयुः";
        optative_middle_first: "उ~ेय", "उ~ेवहि", "उ~ेमहि";
        optative_middle_second: "उ~ेथाः", "उ~ेयाथाम्", "उ~ेध्वम्";
        optative_middle_third: "उ~ेत", "उ~ेयाताम्", "उ~ेरन्";
        subjunctive_active_first: "उ~ानि", "उ~ाव", "उ~ाम";
        subjunctive_active_second: "उ~ाः|उ~ासि", "उ~ाथः", "उ~ाथ";
        subjunctive_active_third: "उ~ात्|उ~ाति", "उ~ातः", "उ~ान्";
        subjunctive_middle_first: "उ~ै", "उ~ावहै", "उ~ामहै";
        subjunctive_middle_second: "उ~ासे|उ~ासै", "उ~ैथे", "उ~ाध्वै";
        subjunctive_middle_third: "उ~ाते|उ~ातै", "उ~ैते", "उ~न्त|उ~ान्तै";
        imperfect_active_first: "औ~म्", "औ~ाव", "औ~ाम";
        imperfect_active_second: "औ~ः", "औ~तम्", "औ~त";
        imperfect_active_third: "औ~त्", "औ~ताम्", "औ~न्";
        imperfect_middle_first: "औ~े", "औ~ावहि", "औ~ामहि";
        imperfect_middle_second: "औ~थाः", "औ~ेथाम्", "औ~ध्वम्";
        imperfect_middle_third: "औ~त", "औ~ेताम्", "औ~न्त";
        future_active_first: "उ~िष्यामि", "उ~िष्यावः", "उ~िष्यामः";
        future_active_second: "उ~िष्यसि", "उ~िष्यथः", "उ~िष्यथ";
        future_active_third: "उ~िष्यति", "उ~िष्यतः", "उ~िष्यन्ति";
        future_middle_first: "उ~िष्ये", "उ~िष्यावहे", "उ~िष्यामहे";
        future_middle_second: "उ~िष्यसे", "उ~िष्येथे", "उ~िष्यध्वे";
        future_middle_third: "उ~िष्यते", "उ~िष्येते", "उ~िष्यन्ते";
        conditional_active_first: "औ~िष्यम्", "औ~िष्याव", "औ~िष्याम";
        conditional_active_second: "औ~िष्यः", "औ~िष्यतम्", "औ~िष्यत";
        conditional_active_third: "औ~िष्यत्", "औ~िष्यताम्", "औ~िष्यन्";
        conditional_middle_first: "औ~िष्ये", "औ~िष्यावहि", "औ~िष्यामहि";
        conditional_middle_second: "औ~िष्यथाः", "औ~िष्येथाम्", "औ~िष्यध्वम्";
        conditional_middle_third: "औ~िष्यत", "औ~िष्येताम्", "औ~िष्यन्त";
        perfect_active_first: "उ~ामास", "उ~ामासिव", "उ~ामासिम";
        perfect_active_second: "उ~ामासिथ", "उ~ामासथुः", "उ~ामास";
        perfect_active_third: "उ~ामास", "उ~ामासतुः", "उ~ामासुः";
        perfect_middle_first: "उ~ांचक्रे", "उ~ांचकृवहे", "उ~ांचकृमहे";
        perfect_middle_second: "उ~ांचकृषे", "उ~ांचक्राथे", "उ~ांचकृध्वे";
        perfect_middle_third: "उ~ांचक्रे", "उ~ांचक्राते", "उ~ांचक्रिरे";
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: "उ~ितुम्";
        gerund: "उ~ित्वा";
        participle_present: "उ~त्", "उ~मान";
        participle_future: "उ~िष्यत्", "उ~िष्यमाण";
        participle_perfect: "उ~ामासिवांस्", "उ~ांचक्राण";
    ),
    // ईळे: 5 words (ईळे, जज्ञे, शश्ये, पस्पशे, चके). Like अमत, except in 11 cells: indicative_active_third sg, perfect_active_first sg, perfect_active_first du, perfect_active_first pl, perfect_active_second sg, perfect_active_second du, ….
    conjugation!("ईळे", 5, "~े", like अमत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~े", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: derived, _;
    ),
    // अगात: 5 words (अधात्, अस्थात्, अदात्, अगात्, अपात्). Like आपत, except in 17 cells: indicative_active_third sg, aorist_active_first sg, aorist_active_second sg, aorist_active_second du, aorist_active_second pl, aorist_active_third sg, ….
    conjugation!("अगात्", 5, "~ात्", like आपत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ात्", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: "~ाम्", _, _;
        aorist_active_second: "~ाः", "~ातम्", "~ात";
        aorist_active_third: "~ात्", "~ाताम्", "~ुः";
        aorist_middle_first: "~ि", "~िवहि", "~िमहि";
        aorist_middle_second: "~िथाः", "~ाथाम्", "~िध्वम्";
        aorist_middle_third: "~ित", "~ाताम्", "~िरन्";
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // जुहाव: 5 words (दुद्राव, शुश्राव, तुष्टाव, सुस्राव, जुहाव). Like अमत, except in 21 cells: indicative_active_third sg, perfect_active_first sg, perfect_active_first du, perfect_active_first pl, perfect_active_second sg, perfect_active_second du, ….
    conjugation!("जुहाव", 5, "~ाव", like अमत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ाव", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: "~व|~ाव", "~ुव", "~ुम";
        perfect_active_second: "~ोथ", "~ुवथुः", "~ुव";
        perfect_active_third: "~ाव", "~ुवतुः", "~ुवुः";
        perfect_middle_first: "~ुवे", "~ुवहे", "~ुमहे";
        perfect_middle_second: "~ुषे", "~ुवाथे", "~ुध्वे";
        perfect_middle_third: "~ुवे", "~ुवाते", "~ुविरे";
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: "~ुवांस्", "~ुवाण";
    ),
    // इयाय: 5 words (बिभाय, जिगाय, इयाय, जिघाय, निनाय). Like अमत, except in 21 cells: indicative_active_third sg, perfect_active_first sg, perfect_active_first du, perfect_active_first pl, perfect_active_second sg, perfect_active_second du, ….
    conjugation!("इयाय", 5, "~ाय", like अमत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ाय", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: "~य|~ाय", "~्यिव", "~्यिम";
        perfect_active_second: "~ेथ|~यिथ", "~्यथुः", "~्य";
        perfect_active_third: "~ाय", "~्यतुः", "~्युः";
        perfect_middle_first: "~्ये", "~्यिवहे", "~्यिमहे";
        perfect_middle_second: "~्यिषे", "~्याथे", "~्यिध्वे";
        perfect_middle_third: "~्ये", "~्याते", "~्यिरे";
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: "~ीवांस्", "~्यान";
    ),
    // चचार: 5 words (जहार, दधार, जगार, चचार, ममार). Like अमत, except in 21 cells: indicative_active_third sg, perfect_active_first sg, perfect_active_first du, perfect_active_first pl, perfect_active_second sg, perfect_active_second du, ….
    conjugation!("चचार", 5, "~ार", like अमत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ार", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: "~र|~ार", "~्रिव", "~्रिम";
        perfect_active_second: "~र्थ|~रिथ", "~्रथुः", "~्र";
        perfect_active_third: "~ार", "~्रतुः", "~्रुः";
        perfect_middle_first: "~्रे", "~्रिवहे", "~्रिमहे";
        perfect_middle_second: "~्रिषे", "~्राथे", "~्रिध्वे";
        perfect_middle_third: "~्रे", "~्राते", "~्रिरे";
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: "~ृवांस्", "~्राण";
    ),
    // आद: 4 words (आद, आप, आर, आव). Like अमत, except in 1 cell: participle_perfect act.
    conjugation!("आद", 4, "~", like अमत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: "~िवांस्", _;
    ),
    // वमति: 4 words (वमति, चमति, नमति, गमति).
    conjugation!("वमति", 4, "~मति";
        indicative_active_first: "~मामि", "~मावः", "~मामः";
        indicative_active_second: "~मसि", "~मथः", "~मथ";
        indicative_active_third: "~मति", "~मतः", "~मन्ति";
        indicative_middle_first: "~मे", "~मावहे", "~मामहे";
        indicative_middle_second: "~मसे", "~मेथे", "~मध्वे";
        indicative_middle_third: "~मते", "~मेते", "~मन्ते";
        imperative_active_first: "~मानि", "~माव", "~माम";
        imperative_active_second: "~म", "~मतम्", "~मत";
        imperative_active_third: "~मतु", "~मताम्", "~मन्तु";
        imperative_middle_first: "~मै", "~मावहै", "~मामहै";
        imperative_middle_second: "~मस्व", "~मेथाम्", "~मध्वम्";
        imperative_middle_third: "~मताम्", "~मेताम्", "~मन्ताम्";
        optative_active_first: "~मेयम्", "~मेव", "~मेम";
        optative_active_second: "~मेः", "~मेतम्", "~मेत";
        optative_active_third: "~मेत्", "~मेताम्", "~मेयुः";
        optative_middle_first: "~मेय", "~मेवहि", "~मेमहि";
        optative_middle_second: "~मेथाः", "~मेयाथाम्", "~मेध्वम्";
        optative_middle_third: "~मेत", "~मेयाताम्", "~मेरन्";
        subjunctive_active_first: "~मानि", "~माव", "~माम";
        subjunctive_active_second: "~माः", "~माथः", "~माथ";
        subjunctive_active_third: "~मात्", "~मातः", "~मान्";
        subjunctive_middle_first: "~मै", "~मावहै", "~मामहै";
        subjunctive_middle_second: "~मासे", "~मैथे", "~माध्वै";
        subjunctive_middle_third: "~माते", "~मैते", "~मन्त";
        imperfect_active_first: "अ~मम्", "अ~माव", "अ~माम";
        imperfect_active_second: "अ~मः", "अ~मतम्", "अ~मत";
        imperfect_active_third: "अ~मत्", "अ~मताम्", "अ~मन्";
        imperfect_middle_first: "अ~मे", "अ~मावहि", "अ~मामहि";
        imperfect_middle_second: "अ~मथाः", "अ~मेथाम्", "अ~मध्वम्";
        imperfect_middle_third: "अ~मत", "अ~मेताम्", "अ~मन्त";
        future_active_first: "~मिष्यामि", "~मिष्यावः", "~मिष्यामः";
        future_active_second: "~मिष्यसि", "~मिष्यथः", "~मिष्यथ";
        future_active_third: "~मिष्यति", "~मिष्यतः", "~मिष्यन्ति";
        future_middle_first: "~मिष्ये", "~मिष्यावहे", "~मिष्यामहे";
        future_middle_second: "~मिष्यसे", "~मिष्येथे", "~मिष्यध्वे";
        future_middle_third: "~मिष्यते", "~मिष्येते", "~मिष्यन्ते";
        conditional_active_first: "अ~मिष्यम्", "अ~मिष्याव", "अ~मिष्याम";
        conditional_active_second: "अ~मिष्यः", "अ~मिष्यतम्", "अ~मिष्यत";
        conditional_active_third: "अ~मिष्यत्", "अ~मिष्यताम्", "अ~मिष्यन्";
        conditional_middle_first: "अ~मिष्ये", "अ~मिष्यावहि", "अ~मिष्यामहि";
        conditional_middle_second: "अ~मिष्यथाः", "अ~मिष्येथाम्", "अ~मिष्यध्वम्";
        conditional_middle_third: "अ~मिष्यत", "अ~मिष्येताम्", "अ~मिष्यन्त";
        perfect_active_first: "~चम", "~ेमिव", "~ेमिम";
        perfect_active_second: "~चन्थ", "~ेमथुः", "~ेम";
        perfect_active_third: "~चाम", "~ेमतुः", "~ेमुः";
        perfect_middle_first: "~ेमे", "~ेमिवहे", "~ेमिमहे";
        perfect_middle_second: "~ेमिषे", "~ेमाथे", "~ेमिध्वे";
        perfect_middle_third: "~ेमे", "~ेमाते", "~ेमिरे";
        aorist_active_first: "अ~मिषम्", "अ~मिष्व", "अ~मिष्म";
        aorist_active_second: "अ~मीः", "अ~मिष्टम्", "अ~मिष्ट";
        aorist_active_third: "अ~मीत्", "अ~मिष्टाम्", "अ~मिषुः";
        aorist_middle_first: "अ~मिषि", "अ~मिष्वहि", "अ~मिष्महि";
        aorist_middle_second: "अ~मिष्ठाः", "अ~मिषाथाम्", "अ~मिढ्वम्";
        aorist_middle_third: "अ~मिष्ट", "अ~मिषाताम्", "अ~मिषत";
        infinitive: "~मितुम्";
        gerund: "~मित्वा";
        participle_present: "~मत्", "~ममान";
        participle_future: "~मिष्यत्", "~मिष्यमाण";
        participle_perfect: "~ेमिवांस्", "~ेमान";
    ),
    // वृणीते: 4 words (मिमीते, जानीते, जिहीते, वृणीते). Like ददाति, except in 46 cells: indicative_active_first sg, indicative_active_first du, indicative_active_first pl, indicative_active_second sg, indicative_active_second du, indicative_active_second pl, ….
    conjugation!("वृणीते", 4, "~ीते", like ददाति;
        indicative_active_first: derived, derived, derived;
        indicative_active_second: derived, derived, derived;
        indicative_active_third: "~ीते", derived, derived;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: derived, derived, derived;
        imperative_active_second: derived, derived, derived;
        imperative_active_third: derived, derived, derived;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: derived, derived, derived;
        optative_active_second: derived, derived, derived;
        optative_active_third: derived, derived, derived;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: derived, derived, derived;
        subjunctive_active_second: derived, derived, derived;
        subjunctive_active_third: derived, derived, derived;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: derived, derived, derived;
        imperfect_active_second: derived, derived, derived;
        imperfect_active_third: derived, derived, derived;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: derived, _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // सरवति: 4 words (स्रवति, द्रवति, चर्वति, तूर्वति).
    conjugation!("स्रवति", 4, "~~वति";
        indicative_active_first: "~~वामि", "~~वावः", "~~वामः";
        indicative_active_second: "~~वसि", "~~वथः", "~~वथ";
        indicative_active_third: "~~वति", "~~वतः", "~~वन्ति";
        indicative_middle_first: "~~वे", "~~वावहे", "~~वामहे";
        indicative_middle_second: "~~वसे", "~~वेथे", "~~वध्वे";
        indicative_middle_third: "~~वते", "~~वेते", "~~वन्ते";
        imperative_active_first: "~~वाणि", "~~वाव", "~~वाम";
        imperative_active_second: "~~व", "~~वतम्", "~~वत";
        imperative_active_third: "~~वतु", "~~वताम्", "~~वन्तु";
        imperative_middle_first: "~~वै", "~~वावहै", "~~वामहै";
        imperative_middle_second: "~~वस्व", "~~वेथाम्", "~~वध्वम्";
        imperative_middle_third: "~~वताम्", "~~वेताम्", "~~वन्ताम्";
        optative_active_first: "~~वेयम्", "~~वेव", "~~वेम";
        optative_active_second: "~~वेः", "~~वेतम्", "~~वेत";
        optative_active_third: "~~वेत्", "~~वेताम्", "~~वेयुः";
        optative_middle_first: "~~वेय", "~~वेवहि", "~~वेमहि";
        optative_middle_second: "~~वेथाः", "~~वेयाथाम्", "~~वेध्वम्";
        optative_middle_third: "~~वेत", "~~वेयाताम्", "~~वेरन्";
        subjunctive_active_first: "~~वाणि", "~~वाव", "~~वाम";
        subjunctive_active_second: "~~वाः", "~~वाथः", "~~वाथ";
        subjunctive_active_third: "~~वात्", "~~वातः", "~~वान्";
        subjunctive_middle_first: "~~वै", "~~वावहै", "~~वामहै";
        subjunctive_middle_second: "~~वासे", "~~वैथे", "~~वाध्वै";
        subjunctive_middle_third: "~~वाते", "~~वैते", "~~वन्त";
        imperfect_active_first: "अ~~वम्", "अ~~वाव", "अ~~वाम";
        imperfect_active_second: "अ~~वः", "अ~~वतम्", "अ~~वत";
        imperfect_active_third: "अ~~वत्", "अ~~वताम्", "अ~~वन्";
        imperfect_middle_first: "अ~~वे", "अ~~वावहि", "अ~~वामहि";
        imperfect_middle_second: "अ~~वथाः", "अ~~वेथाम्", "अ~~वध्वम्";
        imperfect_middle_third: "अ~~वत", "अ~~वेताम्", "अ~~वन्त";
        future_active_first: "~~ोष्यामि", "~~ोष्यावः", "~~ोष्यामः";
        future_active_second: "~~ोष्यसि", "~~ोष्यथः", "~~ोष्यथ";
        future_active_third: "~~ोष्यति", "~~ोष्यतः", "~~ोष्यन्ति";
        future_middle_first: "~~ोष्ये", "~~ोष्यावहे", "~~ोष्यामहे";
        future_middle_second: "~~ोष्यसे", "~~ोष्येथे", "~~ोष्यध्वे";
        future_middle_third: "~~ोष्यते", "~~ोष्येते", "~~ोष्यन्ते";
        conditional_active_first: "अ~~ोष्यम्", "अ~~ोष्याव", "अ~~ोष्याम";
        conditional_active_second: "अ~~ोष्यः", "अ~~ोष्यतम्", "अ~~ोष्यत";
        conditional_active_third: "अ~~ोष्यत्", "अ~~ोष्यताम्", "अ~~ोष्यन्";
        conditional_middle_first: "अ~~ोष्ये", "अ~~ोष्यावहि", "अ~~ोष्यामहि";
        conditional_middle_second: "अ~~ोष्यथाः", "अ~~ोष्येथाम्", "अ~~ोष्यध्वम्";
        conditional_middle_third: "अ~~ोष्यत", "अ~~ोष्येताम्", "अ~~ोष्यन्त";
        perfect_active_first: "~ुद~व", "~ुद~ुव", "~ुद~ुम";
        perfect_active_second: "~ुद~ोथ", "~ुद~ुवथुः", "~ुद~ुव";
        perfect_active_third: "~ुद~ाव", "~ुद~ुवतुः", "~ुद~ुवुः";
        perfect_middle_first: "~ुद~ुवे", "~ुद~ुवहे", "~ुद~ुमहे";
        perfect_middle_second: "~ुद~ुषे", "~ुद~ुवाथे", "~ुद~ुध्वे";
        perfect_middle_third: "~ुद~ुवे", "~ुद~ुवाते", "~ुद~ुविरे";
        aorist_active_first: "अ~~ुद्रुवम्", "अ~~ुद्रुवाव", "अ~~ुद्रुवाम";
        aorist_active_second: "अ~~ुद्रुवः", "अ~~ुद्रुवतम्", "अ~~ुद्रुवत";
        aorist_active_third: "अ~~ुद्रुवत्", "अ~~ुद्रुवताम्", "अ~~ुद्रुवन्";
        aorist_middle_first: "अ~~ाविषि", "अ~~ाविष्वहि", "अ~~ाविष्महि";
        aorist_middle_second: "अ~~ाविष्ठाः", "अ~~ाविषाथाम्", "अ~~ाविढ्वम्";
        aorist_middle_third: "अ~~ाविष्ट", "अ~~ाविषाताम्", "अ~~ाविषत";
        infinitive: "~~ोतुम्";
        gerund: "~~ुत्वा";
        participle_present: "~~वत्", "~~वमाण";
        participle_future: "~~ोष्यत्", "~~ोष्यमाण";
        participle_perfect: "~ुद~ुवांस्", "~ुद~ुवाण";
    ),
    // आनयति: 4 words (आनयति, आगच्छति, आक्रोशति, आमयति). Like एति, except in 58 cells: imperfect_active_first sg, imperfect_active_first du, imperfect_active_first pl, imperfect_active_second sg, imperfect_active_second du, imperfect_active_second pl, ….
    conjugation!("आनयति", 4, "~ति", like एति;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: "~म्", "~ाव", "~ाम";
        imperfect_active_second: "~ः", "~तम्", "~त";
        imperfect_active_third: "~त्", "~ताम्", "~न्";
        imperfect_middle_first: "~े", "~ावहि", "~ामहि";
        imperfect_middle_second: "~थाः", "~ेथाम्", "~ध्वम्";
        imperfect_middle_third: "~त", "~ेताम्", "~न्त";
        future_active_first: _, _, "~िष्यामः";
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: derived, derived, derived;
        conditional_active_second: derived, derived, derived;
        conditional_active_third: derived, derived, derived;
        conditional_middle_first: derived, derived, derived;
        conditional_middle_second: derived, derived, derived;
        conditional_middle_third: derived, derived, derived;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: derived, derived, derived;
        perfect_middle_second: derived, derived, derived;
        perfect_middle_third: derived, derived, derived;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, "~मान";
        participle_future: _, _;
        participle_perfect: derived, derived;
    ),
    // इरसयति: 4 words (इच्छति, इङ्गति, इरस्यति, इङ्गयति).
    conjugation!("इरस्यति", 4, "इ~ति";
        indicative_active_first: "इ~ामि", "इ~ावः", "इ~ामः|इ~ामसि";
        indicative_active_second: "इ~सि", "इ~थः", "इ~थ";
        indicative_active_third: "इ~ति", "इ~तः", "इ~न्ति";
        indicative_middle_first: "इ~े", "इ~ावहे", "इ~ामहे";
        indicative_middle_second: "इ~से", "इ~ेथे", "इ~ध्वे";
        indicative_middle_third: "इ~ते", "इ~ेते", "इ~न्ते";
        imperative_active_first: "इ~ानि", "इ~ाव", "इ~ाम";
        imperative_active_second: "इ~", "इ~तम्", "इ~त";
        imperative_active_third: "इ~तु", "इ~ताम्", "इ~न्तु";
        imperative_middle_first: "इ~ै", "इ~ावहै", "इ~ामहै";
        imperative_middle_second: "इ~स्व", "इ~ेथाम्", "इ~ध्वम्";
        imperative_middle_third: "इ~ताम्", "इ~ेताम्", "इ~न्ताम्";
        optative_active_first: "इ~ेयम्", "इ~ेव", "इ~ेम";
        optative_active_second: "इ~ेः", "इ~ेतम्", "इ~ेत";
        optative_active_third: "इ~ेत्", "इ~ेताम्", "इ~ेयुः";
        optative_middle_first: "इ~ेय", "इ~ेवहि", "इ~ेमहि";
        optative_middle_second: "इ~ेथाः", "इ~ेयाथाम्", "इ~ेध्वम्";
        optative_middle_third: "इ~ेत", "इ~ेयाताम्", "इ~ेरन्";
        subjunctive_active_first: "इ~ानि", "इ~ाव", "इ~ाम";
        subjunctive_active_second: "इ~ाः|इ~ासि", "इ~ाथः", "इ~ाथ";
        subjunctive_active_third: "इ~ात्|इ~ाति", "इ~ातः", "इ~ान्";
        subjunctive_middle_first: "इ~ै", "इ~ावहै", "इ~ामहै";
        subjunctive_middle_second: "इ~ासे|इ~ासै", "इ~ैथे", "इ~ाध्वै";
        subjunctive_middle_third: "इ~ाते|इ~ातै", "इ~ैते", "इ~न्त|इ~ान्तै";
        imperfect_active_first: "ऐ~म्", "ऐ~ाव", "ऐ~ाम";
        imperfect_active_second: "ऐ~ः", "ऐ~तम्", "ऐ~त";
        imperfect_active_third: "ऐ~त्", "ऐ~ताम्", "ऐ~न्";
        imperfect_middle_first: "ऐ~े", "ऐ~ावहि", "ऐ~ामहि";
        imperfect_middle_second: "ऐ~थाः", "ऐ~ेथाम्", "ऐ~ध्वम्";
        imperfect_middle_third: "ऐ~त", "ऐ~ेताम्", "ऐ~न्त";
        future_active_first: "इ~िष्यामि", "इ~िष्यावः", "इ~िष्यामः";
        future_active_second: "इ~िष्यसि", "इ~िष्यथः", "इ~िष्यथ";
        future_active_third: "इ~िष्यति", "इ~िष्यतः", "इ~िष्यन्ति";
        future_middle_first: "इ~िष्ये", "इ~िष्यावहे", "इ~िष्यामहे";
        future_middle_second: "इ~िष्यसे", "इ~िष्येथे", "इ~िष्यध्वे";
        future_middle_third: "इ~िष्यते", "इ~िष्येते", "इ~िष्यन्ते";
        conditional_active_first: "ऐ~िष्यम्", "ऐ~िष्याव", "ऐ~िष्याम";
        conditional_active_second: "ऐ~िष्यः", "ऐ~िष्यतम्", "ऐ~िष्यत";
        conditional_active_third: "ऐ~िष्यत्", "ऐ~िष्यताम्", "ऐ~िष्यन्";
        conditional_middle_first: "ऐ~िष्ये", "ऐ~िष्यावहि", "ऐ~िष्यामहि";
        conditional_middle_second: "ऐ~िष्यथाः", "ऐ~िष्येथाम्", "ऐ~िष्यध्वम्";
        conditional_middle_third: "ऐ~िष्यत", "ऐ~िष्येताम्", "ऐ~िष्यन्त";
        perfect_active_first: "इ~ामास", "इ~ामासिव", "इ~ामासिम";
        perfect_active_second: "इ~ामासिथ", "इ~ामासथुः", "इ~ामास";
        perfect_active_third: "इ~ामास", "इ~ामासतुः", "इ~ामासुः";
        perfect_middle_first: "इ~ांचक्रे", "इ~ांचकृवहे", "इ~ांचकृमहे";
        perfect_middle_second: "इ~ांचकृषे", "इ~ांचक्राथे", "इ~ांचकृध्वे";
        perfect_middle_third: "इ~ांचक्रे", "इ~ांचक्राते", "इ~ांचक्रिरे";
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: "इ~ितुम्";
        gerund: "इ~ित्वा";
        participle_present: "इ~त्", "इ~मान";
        participle_future: "इ~िष्यत्", "इ~िष्यमाण";
        participle_perfect: "इ~ामासिवांस्", "इ~ांचक्राण";
    ),
    // चकार: 3 words (चकार, बभार, जभार). Like चचार, except in 7 cells: perfect_active_first du, perfect_active_first pl, perfect_active_second sg, perfect_middle_first du, perfect_middle_first pl, perfect_middle_second sg, ….
    conjugation!("चकार", 3, "~ार", like चचार;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, "~ृव", "~ृम";
        perfect_active_second: "~र्थ", _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, "~ृवहे", "~ृमहे";
        perfect_middle_second: "~ृषे", _, "~ृध्वे";
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // आचरति: 3 words (आचरति, पक्षति, यक्षति). Like आनयति, except in 25 cells: imperative_active_first sg, subjunctive_active_first sg, future_active_first sg, future_active_first du, future_active_first pl, future_active_second sg, ….
    conjugation!("आचरति", 3, "~ति", like आनयति;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: "~ाणि", _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: "~ाणि", _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: derived, derived, derived;
        future_active_second: derived, derived, derived;
        future_active_third: derived, derived, derived;
        future_middle_first: derived, derived, derived;
        future_middle_second: derived, derived, derived;
        future_middle_third: derived, derived, derived;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: derived;
        gerund: derived;
        participle_present: _, "~माण";
        participle_future: derived, derived;
        participle_perfect: _, _;
    ),
    // परवदति: 3 words (उपन्यस्यति, प्रवदति, प्रक्षालयति).
    conjugation!("प्रवदति", 3, "~~ति";
        indicative_active_first: "~~ामि", "~~ावः", "~~ामः|~~ामसि";
        indicative_active_second: "~~सि", "~~थः", "~~थ";
        indicative_active_third: "~~ति", "~~तः", "~~न्ति";
        indicative_middle_first: "~~े", "~~ावहे", "~~ामहे";
        indicative_middle_second: "~~से", "~~ेथे", "~~ध्वे";
        indicative_middle_third: "~~ते", "~~ेते", "~~न्ते";
        imperative_active_first: "~~ानि", "~~ाव", "~~ाम";
        imperative_active_second: "~~", "~~तम्", "~~त";
        imperative_active_third: "~~तु", "~~ताम्", "~~न्तु";
        imperative_middle_first: "~~ै", "~~ावहै", "~~ामहै";
        imperative_middle_second: "~~स्व", "~~ेथाम्", "~~ध्वम्";
        imperative_middle_third: "~~ताम्", "~~ेताम्", "~~न्ताम्";
        optative_active_first: "~~ेयम्", "~~ेव", "~~ेम";
        optative_active_second: "~~ेः", "~~ेतम्", "~~ेत";
        optative_active_third: "~~ेत्", "~~ेताम्", "~~ेयुः";
        optative_middle_first: "~~ेय", "~~ेवहि", "~~ेमहि";
        optative_middle_second: "~~ेथाः", "~~ेयाथाम्", "~~ेध्वम्";
        optative_middle_third: "~~ेत", "~~ेयाताम्", "~~ेरन्";
        subjunctive_active_first: "~~ानि", "~~ाव", "~~ाम";
        subjunctive_active_second: "~~ाः|~~ासि", "~~ाथः", "~~ाथ";
        subjunctive_active_third: "~~ात्|~~ाति", "~~ातः", "~~ान्";
        subjunctive_middle_first: "~~ै", "~~ावहै", "~~ामहै";
        subjunctive_middle_second: "~~ासे|~~ासै", "~~ैथे", "~~ाध्वै";
        subjunctive_middle_third: "~~ाते|~~ातै", "~~ैते", "~~न्त|~~ान्तै";
        imperfect_active_first: "~ा~म्", "~ा~ाव", "~ा~ाम";
        imperfect_active_second: "~ा~ः", "~ा~तम्", "~ा~त";
        imperfect_active_third: "~ा~त्", "~ा~ताम्", "~ा~न्";
        imperfect_middle_first: "~ा~े", "~ा~ावहि", "~ा~ामहि";
        imperfect_middle_second: "~ा~थाः", "~ा~ेथाम्", "~ा~ध्वम्";
        imperfect_middle_third: "~ा~त", "~ा~ेताम्", "~ा~न्त";
        future_active_first: derived, derived, derived;
        future_active_second: derived, derived, derived;
        future_active_third: derived, derived, derived;
        future_middle_first: derived, derived, derived;
        future_middle_second: derived, derived, derived;
        future_middle_third: derived, derived, derived;
        conditional_active_first: derived, derived, derived;
        conditional_active_second: derived, derived, derived;
        conditional_active_third: derived, derived, derived;
        conditional_middle_first: derived, derived, derived;
        conditional_middle_second: derived, derived, derived;
        conditional_middle_third: derived, derived, derived;
        perfect_active_first: derived, derived, derived;
        perfect_active_second: derived, derived, derived;
        perfect_active_third: derived, derived, derived;
        perfect_middle_first: derived, derived, derived;
        perfect_middle_second: derived, derived, derived;
        perfect_middle_third: derived, derived, derived;
        aorist_active_first: derived, derived, derived;
        aorist_active_second: derived, derived, derived;
        aorist_active_third: derived, derived, derived;
        aorist_middle_first: derived, derived, derived;
        aorist_middle_second: derived, derived, derived;
        aorist_middle_third: derived, derived, derived;
        infinitive: derived;
        gerund: derived;
        participle_present: "~~त्", "~~मान";
        participle_future: derived, derived;
        participle_perfect: derived, derived;
    ),
    // मुमोच: 3 words (मुमोच, शुशोच, रुरोच). Like अमत, except in 21 cells: indicative_active_third sg, perfect_active_first sg, perfect_active_first du, perfect_active_first pl, perfect_active_second sg, perfect_active_second du, ….
    conjugation!("मुमोच", 3, "~ोच", like अमत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ोच", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: "~ोच", "~ुचिव|~ुच्व", "~ुचिम|~ुच्म";
        perfect_active_second: "~ोचिथ", "~ुचथुः", "~ुच";
        perfect_active_third: "~ोच", "~ुचतुः", "~ुचुः";
        perfect_middle_first: "~ुचे", "~ुचिवहे|~ुच्वहे", "~ुचिमहे|~ुच्महे";
        perfect_middle_second: "~ुचिषे|~ुक्षे", "~ुचाथे", "~ुचिध्वे|~ुग्ध्वे";
        perfect_middle_third: "~ुचे", "~ुचाते", "~ुचिरे|~ुच्रे";
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: _, _, _;
        aorist_middle_second: _, _, _;
        aorist_middle_third: _, _, _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: "~ुक्वांस्", "~ुचान";
    ),
    // अकारषीत: 3 words (अहार्षीत्, अभार्षीत्, अकार्षीत्). Like आपत, except in 19 cells: indicative_active_third sg, aorist_active_first sg, aorist_active_first du, aorist_active_first pl, aorist_active_second sg, aorist_active_second du, ….
    conjugation!("अकार्षीत्", 3, "~ार्षीत्", like आपत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ार्षीत्", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: "~ार्षम्", "~ार्ष्व", "~ार्ष्म";
        aorist_active_second: "~ार्षीः|~ाः", "~ार्ष्टम्", "~ार्ष्ट";
        aorist_active_third: "~ार्षीत्|~ाः", "~ार्ष्टाम्", "~ार्षुः";
        aorist_middle_first: "~ृषि", "~ृष्वहि", "~ृष्महि";
        aorist_middle_second: "~ृष्ठाः", "~ृषाथाम्", "~ृढ्वम्";
        aorist_middle_third: "~ृष्ट", "~ृषाताम्", "~ृषत";
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // अधुकषत: 3 words (अधुक्षत्, अद्रुक्षत्, अरुक्षत्). Like आपत, except in 3 cells: aorist_middle_first sg, aorist_middle_second du, aorist_middle_third du.
    conjugation!("अधुक्षत्", 3, "~त्", like आपत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: _, _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: _, _, _;
        aorist_active_second: _, _, _;
        aorist_active_third: _, _, _;
        aorist_middle_first: "~ि", _, _;
        aorist_middle_second: _, "~ाथाम्", _;
        aorist_middle_third: _, "~ाताम्", _;
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
    // अचैषीत: 3 words (अनैषीत्, अजैषीत्, अचैषीत्). Like आपत, except in 19 cells: indicative_active_third sg, aorist_active_first sg, aorist_active_first du, aorist_active_first pl, aorist_active_second sg, aorist_active_second du, ….
    conjugation!("अचैषीत्", 3, "~ैषीत्", like आपत;
        indicative_active_first: _, _, _;
        indicative_active_second: _, _, _;
        indicative_active_third: "~ैषीत्", _, _;
        indicative_middle_first: _, _, _;
        indicative_middle_second: _, _, _;
        indicative_middle_third: _, _, _;
        imperative_active_first: _, _, _;
        imperative_active_second: _, _, _;
        imperative_active_third: _, _, _;
        imperative_middle_first: _, _, _;
        imperative_middle_second: _, _, _;
        imperative_middle_third: _, _, _;
        optative_active_first: _, _, _;
        optative_active_second: _, _, _;
        optative_active_third: _, _, _;
        optative_middle_first: _, _, _;
        optative_middle_second: _, _, _;
        optative_middle_third: _, _, _;
        subjunctive_active_first: _, _, _;
        subjunctive_active_second: _, _, _;
        subjunctive_active_third: _, _, _;
        subjunctive_middle_first: _, _, _;
        subjunctive_middle_second: _, _, _;
        subjunctive_middle_third: _, _, _;
        imperfect_active_first: _, _, _;
        imperfect_active_second: _, _, _;
        imperfect_active_third: _, _, _;
        imperfect_middle_first: _, _, _;
        imperfect_middle_second: _, _, _;
        imperfect_middle_third: _, _, _;
        future_active_first: _, _, _;
        future_active_second: _, _, _;
        future_active_third: _, _, _;
        future_middle_first: _, _, _;
        future_middle_second: _, _, _;
        future_middle_third: _, _, _;
        conditional_active_first: _, _, _;
        conditional_active_second: _, _, _;
        conditional_active_third: _, _, _;
        conditional_middle_first: _, _, _;
        conditional_middle_second: _, _, _;
        conditional_middle_third: _, _, _;
        perfect_active_first: _, _, _;
        perfect_active_second: _, _, _;
        perfect_active_third: _, _, _;
        perfect_middle_first: _, _, _;
        perfect_middle_second: _, _, _;
        perfect_middle_third: _, _, _;
        aorist_active_first: "~ैषम्", "~ैष्व", "~ैष्म";
        aorist_active_second: "~ैषीः|~ैः", "~ैष्टम्", "~ैष्ट";
        aorist_active_third: "~ैषीत्|~ैः", "~ैष्टाम्", "~ैषुः";
        aorist_middle_first: "~ेषि", "~ेष्वहि", "~ेष्महि";
        aorist_middle_second: "~ेष्ठाः", "~ेषाथाम्", "~ेढ्वम्";
        aorist_middle_third: "~ेष्ट", "~ेषाताम्", "~ेषत";
        infinitive: _;
        gerund: _;
        participle_present: _, _;
        participle_future: _, _;
        participle_perfect: _, _;
    ),
];
