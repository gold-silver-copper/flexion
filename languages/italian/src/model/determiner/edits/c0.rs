use flexion::{Edit, Stress};

pub(crate) static ITEMS: [Edit<'static>; 11] = [
    Edit::new("", "", "", "", Stress::Unmarked, 0, [0, 0, 0, 0, 0, 0]),
    Edit::new("", "", "o", "i", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "o", "a", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "o", "e", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "i", "e", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "o", "ei", Stress::Unmarked, 0, [1, 0, 1, 2, 0, 0]),
    Edit::new("", "", "", "a", Stress::Unmarked, 0, [0, 0, 0, 1, 0, 0]),
    Edit::new("", "", "o", "", Stress::Unmarked, 0, [1, 0, 1, 0, 0, 0]),
    Edit::new("", "", "o", "hi", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "o", "he", Stress::Unmarked, 0, [1, 0, 1, 1, 0, 0]),
    Edit::new("", "", "", "i", Stress::Unmarked, 0, [0, 0, 0, 1, 0, 0]),
];
