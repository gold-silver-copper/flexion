use flexion::Path;

/// Words ending -vnoc.
pub(crate) fn n2043(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8367132), (574, 0.11554132), (184, 0.038513772), (46, 0.006820783)], 0.99758905);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.5673457), (184, 0.4077058), (574, 0.023102159)], 0.9981536);
            match s {
                [] => true,
                ['l', 'e', 's', 'c', 'e', 'n', 't', 'i', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(184, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(0, 0.99290454), (574, 0.0050202184)], 0.99792475);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(0, 0.6839399), (574, 0.3136187)], 0.9975586);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', s @ ..] => {
                            out.push(&[(574, 0.6162356), (0, 0.38341343)], 0.99964905);
                            match s {
                                [] => true,
                                ['u', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(0, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', s @ ..] => {
                            out.push(&[(0, 0.9548485), (574, 0.04480049)], 0.99964905);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['v', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(574, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word sutylesorp.
pub(crate) fn n6561(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1328, 1.0)], 1.0);
    s.is_empty()
}

/// The word murtemisorp.
pub(crate) fn n5271(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -osorp.
pub(crate) fn n5272(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', s @ ..] => {
            out.push(&[(2, 0.9139544), (0, 0.071946286), (1328, 0.010833906), (99, 0.0027008469)], 0.9994354);
            s.is_empty()
        }
        ['d', 'i', 'a', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['p', 'o', 'g', 'r', 'a', 'p', 'h', 'i', 'a', '\u{1}', s @ ..] => {
            out.push(&[(99, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word satirepsorp.
pub(crate) fn n5273(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -tsorp.
pub(crate) fn n5274(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 't', 'a', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(2, 0.9139544), (0, 0.071946286), (1328, 0.010833906), (99, 0.0027008469)], 0.9994354);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -torp.
pub(crate) fn n3318(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.7534142), (0, 0.13786526), (1311, 0.104448006)], 0.9957275);
    match s {
        [] => true,
        ['a', 's', 'i', 's', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(2, 0.54222935), (1311, 0.4340734), (0, 0.022980087)], 0.99928284);
                    match s {
                        [] => true,
                        ['t', s @ ..] => {
                            match s {
                                [] => true,
                                ['o', 'r', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(1311, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 'i', 'x', '\u{1}', s @ ..] => {
                                    out.push(&[(1311, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['r', 'v', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['h', 'e', 's', 'i', 's', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -vorp.
pub(crate) fn n3319(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.9785611), (0, 0.015838865)], 0.99439996);
    s.is_empty()
}

/// Words ending -xorp.
pub(crate) fn n3320(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.90785074), (2, 0.07757687), (912, 0.0021820401)], 0.9876096);
    s.is_empty()
}

