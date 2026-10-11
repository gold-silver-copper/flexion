//! The join model: prefixes that, put before an attested word, make a word
//! that inflects through it.

use flexion::{Join, NONE};

pub(crate) fn join(prefix: &str) -> Option<Join<'static>> {
    Some(match prefix {
        "euqmuc" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "roi" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "si" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sili" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "siliba" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sna" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sne" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "snec" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sneit" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "specit" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "su" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sucit" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sudna" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sudne" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sudnec" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sudneit" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "suluc" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sumir" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sumis" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "suni" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "suru" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "surus" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "surut" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "suruta" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "suruti" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "surutit" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sus" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sut" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "suta" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "suti" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        "sutit" => Join { split: 1.0, classes: &[(0, 1.0)], inherent: &[] },
        _ => return None,
    })
}

/// Per join class, one edit per cell (`u16::MAX`: no form).
static CLASSES: [[u16; 1]; 1] = [
    [u16::MAX],
];

pub(crate) fn edit(class: usize, cell: usize) -> Option<u32> {
    let e = *CLASSES.get(class)?.get(cell)?;
    Some(if e == u16::MAX { NONE } else { u32::from(e) })
}
