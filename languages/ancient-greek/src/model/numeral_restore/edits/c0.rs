use flexion::{Edit, Stress};

pub(crate) static ITEMS: [Edit<'static>; 9] = [
    Edit::new("", "", "ατεσσαρες", "ᾰτεσσᾰρες", Stress::Unmarked, 0, [9, 0, 4, 4, 0, 0]),
    Edit::new("", "", "", "", Stress::Unmarked, 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "ιακοσιοι", "ῐᾱκοσῐοι", Stress::Unmarked, 0, [8, 0, 6, 6, 0, 0]),
    Edit::new("", "", "ισχιλιοι", "ῐσχῑλῐοι", Stress::Unmarked, 0, [8, 0, 5, 5, 0, 0]),
    Edit::new("", "", "υο", "ῠο", Stress::Unmarked, 0, [2, 0, 2, 2, 0, 0]),
    Edit::new("", "", "ακοσιοι", "ᾰκοσῐοι", Stress::Unmarked, 0, [7, 0, 5, 5, 0, 0]),
    Edit::new("", "", "υρες", "ῠρες", Stress::Unmarked, 0, [4, 0, 2, 2, 0, 0]),
    Edit::new("", "", "αρες", "ᾰρες", Stress::Unmarked, 0, [4, 0, 2, 2, 0, 0]),
    Edit::new("", "", "ιλιοι", "ῑλῐοι", Stress::Unmarked, 0, [5, 0, 4, 4, 0, 0]),
];
