use super::super::ClassDef;
use flexion::{Class, Stress};

pub(crate) static ITEMS: [ClassDef; 10] = [
    // 0: like а-дато
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [0] },
    // 1: like авианалёт
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(0), 0), values: &[] }, subs: [1] },
    // 2: like авиаприёмник
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(1), 0), values: &[] }, subs: [2] },
    // 3: like аксёнович
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(2), 0), values: &[] }, subs: [3] },
    // 4: like актёрствование
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(4), 0), values: &[] }, subs: [4] },
    // 5: like альстрёмерия
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(3), 0), values: &[] }, subs: [5] },
    // 6: like вертолётостроение
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(5), 0), values: &[] }, subs: [6] },
    // 7: like гололёдообразование
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(7), 0), values: &[] }, subs: [7] },
    // 8: like плёнкообразователь
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(6), 0), values: &[] }, subs: [8] },
    // 9: like средостение
    ClassDef { class: Class { strip: "е", front: "", lemma: (Stress::Added(0), 0), values: &[] }, subs: [9] },
];
