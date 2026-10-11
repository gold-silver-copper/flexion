use flexion::{Edit, Stress};

pub(crate) static ITEMS: [Edit<'static>; 10] = [
    Edit::new("", "", "", "", Stress::Unmarked, 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(0), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(1), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(2), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(4), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(3), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(5), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(7), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "", Stress::KeptFromEnd(6), 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "е", "ё", Stress::Added(0), 0, [1, 0, 1, 1, 0, 0]),
];
