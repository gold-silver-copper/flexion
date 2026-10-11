use super::super::ClassDef;
use flexion::{Class, Stress};

pub(crate) static ITEMS: [ClassDef; 9] = [
    // 0: like δεκατεσσαρες
    ClassDef { class: Class { strip: "ατεσσαρεσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [0] },
    // 1: like εἱς
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [1] },
    // 2: like διακοσιοι
    ClassDef { class: Class { strip: "ιακοσιοι", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [2] },
    // 3: like δισχιλιοι
    ClassDef { class: Class { strip: "ισχιλιοι", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [3] },
    // 4: like δυο
    ClassDef { class: Class { strip: "υο", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [4] },
    // 5: like ἐνακοσιοι
    ClassDef { class: Class { strip: "ακοσιοι", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [5] },
    // 6: like πισυρες
    ClassDef { class: Class { strip: "υρεσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [6] },
    // 7: like τεσσαρες
    ClassDef { class: Class { strip: "αρεσ", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [7] },
    // 8: like χιλιοι
    ClassDef { class: Class { strip: "ιλιοι", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [8] },
];
