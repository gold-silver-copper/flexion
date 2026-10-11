use super::super::ClassDef;
use flexion::{Class, Stress};

pub(crate) static ITEMS: [ClassDef; 15] = [
    // 0: like а-мольный
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [0] },
    // 1: like абрикосово-жёлтый
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(1), 0), values: &[] }, subs: [1] },
    // 2: like авиасъёмочный
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(2), 0), values: &[] }, subs: [2] },
    // 3: like актёрско-режиссёрский
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(5), 0), values: &[] }, subs: [3] },
    // 4: like актёрствовавший
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(3), 0), values: &[] }, subs: [4] },
    // 5: like амёбообразный
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(4), 0), values: &[] }, subs: [5] },
    // 6: like жёлто-буро-голубой
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(6), 0), values: &[] }, subs: [6] },
    // 7: like казённо-административный
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(7), 0), values: &[] }, subs: [7] },
    // 8: like помехозащищённый
    ClassDef { class: Class { strip: "ехозащищенный", front: "", lemma: (Stress::Added(0), 0), values: &[] }, subs: [8] },
    // 9: like прищепленный
    ClassDef { class: Class { strip: "енный", front: "", lemma: (Stress::Added(0), 0), values: &[] }, subs: [9] },
    // 10: like трёхгодовой
    ClassDef { class: Class { strip: "ехгодовой", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [10] },
    // 11: like трёхоперационный
    ClassDef { class: Class { strip: "ехоперационный", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [11] },
    // 12: like трёхчасовой
    ClassDef { class: Class { strip: "ехчасовой", front: "", lemma: (Stress::Unmarked, 0), values: &[] }, subs: [12] },
    // 13: like царёв
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(0), 0), values: &[] }, subs: [13] },
    // 14: like шёлкообрабатывающий
    ClassDef { class: Class { strip: "", front: "", lemma: (Stress::KeptFromEnd(8), 0), values: &[] }, subs: [14] },
];
