use flexion::{Edit, Stress};

pub(crate) static ITEMS: [Edit<'static>; 9] = [
    Edit::new("", "", "", "", Stress::Unmarked, 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "s", Stress::Unmarked, 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "", "e", Stress::Unmarked, 0, [0, 0, 0, 1, 0, 0]),
    Edit::new("", "", "", "es", Stress::Unmarked, 0, [0, 0, 0, 1, 0, 0]),
    Edit::new("", "", "s", "te", Stress::Unmarked, 0, [1, 0, 0, 1, 0, 0]),
    Edit::new("", "", "s", "tes", Stress::Unmarked, 0, [1, 0, 0, 1, 0, 0]),
    Edit::new("", "", "û", "us", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "û", "ue", Stress::Unmarked, 0, [1, 0, 1, 2, 0, 0]),
    Edit::new("", "", "û", "ues", Stress::Unmarked, 0, [1, 0, 1, 2, 0, 0]),
];
