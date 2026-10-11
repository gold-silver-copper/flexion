use super::super::ClassDef;
use flexion::{Class, Stress};

pub(crate) static ITEMS: [ClassDef; 16] = [
    // 0: like ἀλληλων
    ClassDef { class: Class { strip: "αλληλων", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [0] },
    // 1: like ἀμφοτερος
    ClassDef { class: Class { strip: "αμφοτεροσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [1] },
    // 2: like ἀμφω
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [2] },
    // 3: like ἁπας
    ClassDef { class: Class { strip: "απασ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [3] },
    // 4: like αὐτοτατος
    ClassDef { class: Class { strip: "ατοσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [4] },
    // 5: like ἑκαστος
    ClassDef { class: Class { strip: "αστοσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [5] },
    // 6: like ἑκατερος
    ClassDef { class: Class { strip: "ατεροσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [6] },
    // 7: like ἐνιοι
    ClassDef { class: Class { strip: "ιοι", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [7] },
    // 8: like ἱ
    ClassDef { class: Class { strip: "ι", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [8] },
    // 9: like ὁπηλικος
    ClassDef { class: Class { strip: "ικοσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [9] },
    // 10: like ὁσαπλασιος
    ClassDef { class: Class { strip: "απλασιοσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [10] },
    // 11: like ὁσαπλασιων
    ClassDef { class: Class { strip: "απλασιων", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [11] },
    // 12: like ὁστις
    ClassDef { class: Class { strip: "ισ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [12] },
    // 13: like παντοιος
    ClassDef { class: Class { strip: "αντοιοσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [13] },
    // 14: like συ
    ClassDef { class: Class { strip: "υ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [14] },
    // 15: like ὑμεις
    ClassDef { class: Class { strip: "υμεισ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [15] },
];
