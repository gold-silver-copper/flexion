use flexion::{Edit, Stress};

pub(crate) static ITEMS: [Edit<'static>; 16] = [
    Edit::new("", "", "ἀλληλων", "ᾰ\u{313}λληλων", Stress::Unmarked, 0, [7, 0, 3, 3, 0, 0]),
    Edit::new("", "", "ἀμφοτερος", "ᾰ\u{313}μφοτερος", Stress::Unmarked, 0, [9, 0, 4, 4, 0, 0]),
    Edit::new("", "", "", "", Stress::Unmarked, 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "ἁπας", "ᾰ\u{314}πᾱς", Stress::Unmarked, 0, [4, 0, 2, 2, 0, 0]),
    Edit::new("", "", "ατος", "ᾰτος", Stress::Unmarked, 0, [4, 0, 2, 2, 0, 0]),
    Edit::new("", "", "αστος", "ᾰστος", Stress::Unmarked, 0, [5, 0, 2, 2, 0, 0]),
    Edit::new("", "", "ατερος", "ᾰτερος", Stress::Unmarked, 0, [6, 0, 3, 3, 0, 0]),
    Edit::new("", "", "ιοι", "ῐοι", Stress::Unmarked, 0, [3, 0, 3, 3, 0, 0]),
    Edit::new("", "", "ἱ", "ῑ\u{314}", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "ικος", "ῐκος", Stress::Unmarked, 0, [4, 0, 2, 2, 0, 0]),
    Edit::new("", "", "απλασιος", "ᾰπλᾰσῐος", Stress::Unmarked, 0, [8, 0, 4, 4, 0, 0]),
    Edit::new("", "", "απλασιων", "ᾰπλᾰσῐων", Stress::Unmarked, 0, [8, 0, 4, 4, 0, 0]),
    Edit::new("", "", "ις", "ῐς", Stress::Unmarked, 0, [2, 0, 1, 1, 0, 0]),
    Edit::new("", "", "αντοιος", "ᾰντοιος", Stress::Unmarked, 0, [7, 0, 4, 4, 0, 0]),
    Edit::new("", "", "υ", "ῠ", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "ὑμεις", "ῡ\u{314}μεις", Stress::Unmarked, 0, [5, 0, 3, 3, 0, 0]),
];
