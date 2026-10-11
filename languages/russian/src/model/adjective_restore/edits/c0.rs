use flexion::{Edit, Stress};

pub(crate) static ITEMS: [Edit<'static>; 15] = [
    Edit::new("", "", "", "", Stress::Unmarked, 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(1), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(2), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(5), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(3), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(4), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(6), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(7), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "ехозащищённый", "ёхозащищённый", Stress::Added(0), 0, [13, 0, 6, 6, 0, 0]),
    Edit::new("", "", "енный", "ённый", Stress::Added(0), 0, [5, 0, 2, 2, 0, 0]),
    Edit::new("", "", "ёхгодовой", "ехгодовой", Stress::Unmarked, 0, [9, 0, 4, 4, 0, 0]),
    Edit::new("", "", "ёхоперационный", "ехоперационный", Stress::Unmarked, 0, [14, 0, 7, 7, 0, 0]),
    Edit::new("", "", "ёхчасовой", "ехчасовой", Stress::Unmarked, 0, [9, 0, 4, 4, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(0), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(8), 0, [0, 0, 0, 0, 0, 0]),
];
