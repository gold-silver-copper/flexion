use flexion::Path;

/// Words ending -ricaturus.
pub(crate) fn n16683(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'a', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -lcaturus.
pub(crate) fn n11508(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -runcaturus.
pub(crate) fn n11509(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'v', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ocaturus.
pub(crate) fn n11510(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rcaturus.
pub(crate) fn n11511(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'p', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -scaturus.
pub(crate) fn n11512(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', 'p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word conmanducaturus.
pub(crate) fn n16698(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -rogaturus.
pub(crate) fn n11538(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -urgaturus.
pub(crate) fn n11539(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ugaturus.
pub(crate) fn n11540(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -llaturus.
pub(crate) fn n11559(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['b', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['c', 'n', 'a', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', 'p', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'r', 'c', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'o', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', s @ ..] => {
                    match s {
                        [] => true,
                        ['g', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'a', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['x', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['v', 'a', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'b', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -olaturus.
pub(crate) fn n11562(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'r', 'g', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['r', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['h', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', 'm', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['p', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word contemplaturus.
pub(crate) fn n11563(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -erlaturus.
pub(crate) fn n11564(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word translaturus.
pub(crate) fn n11565(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ulaturus.
pub(crate) fn n11566(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'r', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['e', 'p', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'n', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', 'a', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['g', 'n', 'a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 't', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', 'e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['h', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'u', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['e', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['u', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['i', 't', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['a', 'r', 'g', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['x', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['u', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'e', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nformaturus.
pub(crate) fn n20970(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word transformaturus.
pub(crate) fn n20971(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -plasmaturus.
pub(crate) fn n11575(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 't', 'a', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -umaturus.
pub(crate) fn n11576(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'e', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['f', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['h', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['p', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'r', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -inaturus.
pub(crate) fn n11580(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'u', 'l', 'a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['h', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', 'r', 'o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', 'u', 'r', 'e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['t', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['f', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['g', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['l', 'u', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['g', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['g', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['t', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['d', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['e', 'd', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        ['n', 'i', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['t', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['x', 'e', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['l', s @ ..] => super::t8::n23217(s, out),
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['p', s @ ..] => super::t8::n16897(s, out),
        ['t', s @ ..] => super::t8::n16898(s, out),
        ['u', 'q', 'n', 'i', s @ ..] => super::t8::n16899(s, out),
        _ => false,
    }
}

/// Words ending -onaturus.
pub(crate) fn n11583(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rnaturus.
pub(crate) fn n11584(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'b', 'u', 'g', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -eraturus.
pub(crate) fn n11597(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['i', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'e', 'v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['m', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'u', 'x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['n', 'o', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['c', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'r', 'f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', 'e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'e', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['e', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'l', 'g', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['g', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'n', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['g', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['d', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['r', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['g', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['v', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['g', 'i', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'u', 'v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => super::t8::n23283(s, out),
                        ['x', 'e', '\u{1}', s @ ..] => super::t8::n23284(s, out),
                        _ => false,
                    }
                }
                ['u', s @ ..] => super::t8::n21100(s, out),
                _ => false,
            }
        }
        ['p', s @ ..] => super::t8::n16944(s, out),
        ['s', s @ ..] => super::t8::n16945(s, out),
        ['t', s @ ..] => super::t8::n16946(s, out),
        _ => false,
    }
}

/// The word denigraturus.
pub(crate) fn n23316(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word pigraturus.
pub(crate) fn n21120(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -iraturus.
pub(crate) fn n11599(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', 'e', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['v', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -oraturus.
pub(crate) fn n11600(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'r', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['e', 'd', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'u', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 't', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', 'i', 'v', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', 'e', 'm', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['o', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['g', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'm', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'h', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['a', 'v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'o', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', 'c', s @ ..] => {
            match s {
                [] => true,
                ['o', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rraturus.
pub(crate) fn n11601(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'n', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 's', 'u', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -traturus.
pub(crate) fn n11602(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'c', 'l', 'a', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'n', 'i', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['e', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                ['d', '\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['l', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['i', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['o', 'c', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -uraturus.
pub(crate) fn n11603(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['i', 'f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['f', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['x', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', 'r', 'u', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['x', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -gyraturus.
pub(crate) fn n11604(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word insectaturus.
pub(crate) fn n24102(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word consectaturus.
pub(crate) fn n24103(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word persectaturus.
pub(crate) fn n23433(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word assectaturus.
pub(crate) fn n23434(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -vectaturus.
pub(crate) fn n21238(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -dictaturus.
pub(crate) fn n17008(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -arctaturus.
pub(crate) fn n17009(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -uctaturus.
pub(crate) fn n17010(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -etaturus.
pub(crate) fn n11614(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'u', 'q', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', 'u', 'c', 'o', 'l', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'r', 't', 's', 'a', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -itaturus.
pub(crate) fn n11615(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', 'h', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['c', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['d', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'l', 'o', s @ ..] => {
                            match s {
                                [] => true,
                                ['p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['s', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'b', 'e', 'f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['v', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'e', 'x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'c', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['u', 's', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'r', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['r', 'o', 'c', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['x', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'e', 'h', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['n', 'e', 'v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'f', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['f', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'd', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'h', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'n', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['m', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => super::t8::n17020(s, out),
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['e', 'r', 'c', s @ ..] => super::t8::n21280(s, out),
                ['s', 'o', 'h', s @ ..] => super::t8::n21281(s, out),
                _ => false,
            }
        }
        ['s', 'u', s @ ..] => super::t8::n17022(s, out),
        ['t', 'c', 'a', s @ ..] => super::t8::n17023(s, out),
        ['u', 'q', 'e', s @ ..] => super::t8::n17024(s, out),
        ['v', s @ ..] => super::t8::n17025(s, out),
        _ => false,
    }
}

/// Words ending -exaltaturus.
pub(crate) fn n11616(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ntaturus.
pub(crate) fn n11617(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['m', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'e', 'a', 'r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['o', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -otaturus.
pub(crate) fn n11618(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', 'e', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ptaturus.
pub(crate) fn n11619(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 'r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', 'e', 't', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rtaturus.
pub(crate) fn n11620(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['h', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['p', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['b', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -staturus.
pub(crate) fn n11621(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 'v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 'h', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['u', 'f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'q', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['b', 'r', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['u', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'e', 'v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -utaturus.
pub(crate) fn n11622(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['l', 'a', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 't', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['r', 'c', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -secturus.
pub(crate) fn n11653(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', s @ ..] => {
            match s {
                [] => true,
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['s', 'i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tecturus.
pub(crate) fn n11654(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -vecturus.
pub(crate) fn n11655(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['b', 'u', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', s @ ..] => {
            match s {
                [] => true,
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -luminaturus.
pub(crate) fn n23217(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -pinaturus.
pub(crate) fn n16897(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8663615), (335, 0.13363852)], 1.0);
    match s {
        [] => true,
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    out.push(&[(335, 0.71120775), (0, 0.28879225)], 1.0);
                    match s {
                        [] => true,
                        ['p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -tinaturus.
pub(crate) fn n16898(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['s', 'e', s @ ..] => {
            match s {
                [] => true,
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['u', 'l', 'g', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -inquinaturus.
pub(crate) fn n16899(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word deoneraturus.
pub(crate) fn n23283(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word exoneraturus.
pub(crate) fn n23284(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -uneraturus.
pub(crate) fn n21100(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -peraturus.
pub(crate) fn n16944(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['e', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -seraturus.
pub(crate) fn n16945(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -teraturus.
pub(crate) fn n16946(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'l', 'b', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 'v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', 'b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -mitaturus.
pub(crate) fn n17020(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'l', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -crepitaturus.
pub(crate) fn n21280(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -hospitaturus.
pub(crate) fn n21281(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -usitaturus.
pub(crate) fn n17022(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['c', 'c', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -actitaturus.
pub(crate) fn n17023(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['f', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -equitaturus.
pub(crate) fn n17024(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -vitaturus.
pub(crate) fn n17025(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 'r', 'g', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

