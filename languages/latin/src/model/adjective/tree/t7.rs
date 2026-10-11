use flexion::Path;

/// Words ending -iciendus.
pub(crate) fn n10897(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 'r', 't', '\u{1}', s @ ..] => {
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
                ['r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', s @ ..] => {
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
                        ['o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
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
                ['r', 'e', s @ ..] => {
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
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['a', 'r', 't', '\u{1}', s @ ..] => {
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
                        ['r', '\u{1}', s @ ..] => {
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
                ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
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
                        ['e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
                ['c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 's', s @ ..] => {
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
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'r', 't', '\u{1}', s @ ..] => {
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
                ['o', 'r', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', s @ ..] => {
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -arciendus.
pub(crate) fn n15891(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', s @ ..] => {
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
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -erciendus.
pub(crate) fn n15892(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', 'e', '\u{1}', s @ ..] => {
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
        ['p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -sciendus.
pub(crate) fn n10901(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['n', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -diendus.
pub(crate) fn n6250(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'r', 'g', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['o', 'b', 'o', s @ ..] => {
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
                ['r', 'g', 'e', s @ ..] => {
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
                        ['t', 'n', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', 'a', 'l', 'b', s @ ..] => {
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
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', s @ ..] => {
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
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', '\u{1}', s @ ..] => {
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
                                ['u', 's', '\u{1}', s @ ..] => {
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
                        ['r', 'e', s @ ..] => {
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
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', 's', 'u', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'o', s @ ..] => {
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
        ['u', 'a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
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

/// The word fiendus.
pub(crate) fn n10909(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word suffiendus.
pub(crate) fn n10910(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -giendus.
pub(crate) fn n6252(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['r', 'a', 'l', s @ ..] => {
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
        ['u', 'f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -liendus.
pub(crate) fn n6253(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'p', 'e', 's', s @ ..] => {
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
        ['i', 'b', 'a', 't', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
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
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['o', 'm', s @ ..] => {
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
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['u', 'b', s @ ..] => {
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
                                ['r', '\u{1}', s @ ..] => {
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
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -dormiendus.
pub(crate) fn n6254(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -niendus.
pub(crate) fn n6255(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'n', 'i', s @ ..] => {
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
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['l', s @ ..] => {
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
                ['v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                                ['t', 'n', 'a', '\u{1}', s @ ..] => {
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
                                ['i', s @ ..] => {
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
                                ['t', 'n', 'i', '\u{1}', s @ ..] => {
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
        ['g', 'i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['f', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
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
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
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
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
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

/// Words ending -piendus.
pub(crate) fn n6256(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', 'u', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['a', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['c', 'e', 'd', s @ ..] => {
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
                ['r', 'e', s @ ..] => {
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['o', 's', s @ ..] => {
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
        ['u', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -riendus.
pub(crate) fn n6257(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'p', s @ ..] => {
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
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', 's', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', s @ ..] => {
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
                _ => false,
            }
        }
        ['o', s @ ..] => {
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
                ['h', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
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
                                _ => false,
                            }
                        }
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
        ['r', 'a', 's', s @ ..] => {
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
        ['t', 'u', 'n', s @ ..] => {
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
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                ['a', 'h', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
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
                ['g', 'i', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -tiendus.
pub(crate) fn n6258(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 'm', s @ ..] => {
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
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'o', 'c', '\u{1}', s @ ..] => {
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
        ['n', 'e', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
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
                ['s', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['a', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 's', s @ ..] => {
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
                _ => false,
            }
        }
        ['s', 'e', 'v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
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
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', 'u', 'l', 'g', s @ ..] => {
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
        ['u', 'c', 'r', 'e', 'p', s @ ..] => {
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

/// Words ending -viendus.
pub(crate) fn n6259(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 's', s @ ..] => {
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
                ['s', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word superponendus.
pub(crate) fn n20243(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word interponendus.
pub(crate) fn n20244(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -sponendus.
pub(crate) fn n16069(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', 'd', s @ ..] => {
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
        ['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word postponendus.
pub(crate) fn n16070(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word exponendus.
pub(crate) fn n16071(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ernendus.
pub(crate) fn n6274(s: &[char], _out: &mut Path<'_>) -> bool {
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
                        ['d', s @ ..] => {
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
                        ['s', '\u{1}', s @ ..] => {
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
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
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
                ['s', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
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

/// The word inserendus.
pub(crate) fn n20276(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word conserendus.
pub(crate) fn n20277(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word interserendus.
pub(crate) fn n16128(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -sserendus.
pub(crate) fn n16129(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 'd', s @ ..] => {
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

/// The word exserendus.
pub(crate) fn n16130(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -terendus.
pub(crate) fn n11013(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
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
        ['d', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'd', '\u{1}', s @ ..] => {
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
        ['t', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word conquerendus.
pub(crate) fn n11014(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -verendus.
pub(crate) fn n11015(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', 'u', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word aegrendus.
pub(crate) fn n6280(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -quirendus.
pub(crate) fn n6281(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'a', '\u{1}', s @ ..] => {
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rrendus.
pub(crate) fn n6282(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['t', s @ ..] => {
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
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['p', 'a', '\u{1}', s @ ..] => {
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
                ['v', s @ ..] => {
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
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', '\u{1}', s @ ..] => {
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
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['h', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'a', '\u{1}', s @ ..] => {
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
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
        ['u', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', s @ ..] => {
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
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'n', 'a', '\u{1}', s @ ..] => {
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
                        ['t', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
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
        _ => false,
    }
}

/// Words ending -urendus.
pub(crate) fn n6283(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word animadvertendus.
pub(crate) fn n16188(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -evertendus.
pub(crate) fn n11052(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['t', 'n', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word divertendus.
pub(crate) fn n11053(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -nvertendus.
pub(crate) fn n11054(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ervertendus.
pub(crate) fn n11055(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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

/// The word transvertendus.
pub(crate) fn n11056(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -sistendus.
pub(crate) fn n6291(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -mittendus.
pub(crate) fn n6292(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['i', 'd', '\u{1}', s @ ..] => {
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
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', s @ ..] => {
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
                                ['p', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
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
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -utendus.
pub(crate) fn n6293(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['e', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word insequendus.
pub(crate) fn n20358(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word consequendus.
pub(crate) fn n20359(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word prosequendus.
pub(crate) fn n16229(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word persequendus.
pub(crate) fn n16230(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word assequendus.
pub(crate) fn n16231(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word exsequendus.
pub(crate) fn n16232(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -linquendus.
pub(crate) fn n11111(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['r', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -oquendus.
pub(crate) fn n11112(s: &[char], _out: &mut Path<'_>) -> bool {
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
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
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
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'o', '\u{1}', s @ ..] => {
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
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -torquendus.
pub(crate) fn n11113(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['s', 'i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ruendus.
pub(crate) fn n6302(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
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
        ['e', '\u{1}', s @ ..] => {
            s.is_empty()
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
        ['i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', s @ ..] => {
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
        ['t', s @ ..] => {
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
                                ['d', '\u{1}', s @ ..] => {
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
                        ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['x', 'e', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -suendus.
pub(crate) fn n6303(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -tuendus.
pub(crate) fn n6304(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 't', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'm', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 't', 's', s @ ..] => {
            match s {
                [] => true,
                ['b', 'u', 's', '\u{1}', s @ ..] => {
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
                        ['r', '\u{1}', s @ ..] => {
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
        ['u', 'f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', 'e', '\u{1}', s @ ..] => {
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

/// The word propensurus.
pub(crate) fn n16506(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word appensurus.
pub(crate) fn n16507(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word perpensurus.
pub(crate) fn n16508(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word suspensurus.
pub(crate) fn n16509(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word expensurus.
pub(crate) fn n16510(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -prensurus.
pub(crate) fn n11408(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['m', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['p', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -sensurus.
pub(crate) fn n11409(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -onsurus.
pub(crate) fn n6642(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['p', 's', s @ ..] => {
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
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                ['t', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -tunsurus.
pub(crate) fn n6643(s: &[char], _out: &mut Path<'_>) -> bool {
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
        _ => false,
    }
}

/// The word concursurus.
pub(crate) fn n16558(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word procursurus.
pub(crate) fn n11446(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ercursurus.
pub(crate) fn n11447(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word discursurus.
pub(crate) fn n11448(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word excursurus.
pub(crate) fn n11449(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word circumgressurus.
pub(crate) fn n20542(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ngressurus.
pub(crate) fn n20543(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -rogressurus.
pub(crate) fn n20544(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', s @ ..] => {
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

/// Words ending -ergressurus.
pub(crate) fn n20545(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['p', 'u', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word transgressurus.
pub(crate) fn n20546(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -pressurus.
pub(crate) fn n16576(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['p', s @ ..] => {
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
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -sessurus.
pub(crate) fn n11459(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
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
        ['d', 'a', '\u{1}', s @ ..] => {
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
        ['p', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -issurus.
pub(crate) fn n6660(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
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
                ['f', 'i', 'd', '\u{1}', s @ ..] => {
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
                ['i', 'd', '\u{1}', s @ ..] => {
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
                        ['u', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['s', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['p', s @ ..] => {
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
                                        ['p', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', 'n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
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
                                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -fossurus.
pub(crate) fn n6661(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', '\u{1}', s @ ..] => {
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
                ['u', 's', '\u{1}', s @ ..] => {
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
        ['r', 'e', s @ ..] => {
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
        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ussurus.
pub(crate) fn n6662(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'r', 'e', 'p', s @ ..] => {
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', 'i', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -caturus.
pub(crate) fn n6675(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['p', s @ ..] => {
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
                ['v', s @ ..] => {
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
                _ => false,
            }
        }
        ['c', 'i', 's', s @ ..] => {
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
                ['s', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['a', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
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
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
                        ['e', 'd', '\u{1}', s @ ..] => {
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'r', s @ ..] => {
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
                        ['b', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['p', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['r', 'p', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['d', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', s @ ..] => {
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
                                ['i', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['v', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['f', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['l', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['u', s @ ..] => super::t7::n20620(s, out),
                        _ => false,
                    }
                }
                ['f', 'i', s @ ..] => super::t7::n16681(s, out),
                ['l', 'p', s @ ..] => super::t7::n16682(s, out),
                ['r', s @ ..] => super::t8::n16683(s, out),
                _ => false,
            }
        }
        ['l', s @ ..] => super::t8::n11508(s, out),
        ['n', 'u', 'r', s @ ..] => super::t8::n11509(s, out),
        ['o', s @ ..] => super::t8::n11510(s, out),
        ['r', s @ ..] => super::t8::n11511(s, out),
        ['s', s @ ..] => super::t8::n11512(s, out),
        ['u', 'd', 'n', 'a', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => super::t8::n16698(s, out),
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -praedaturus.
pub(crate) fn n16699(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word foedaturus.
pub(crate) fn n16700(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -pedaturus.
pub(crate) fn n16701(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word exheredaturus.
pub(crate) fn n16702(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -sedaturus.
pub(crate) fn n16703(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -idaturus.
pub(crate) fn n11517(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'u', s @ ..] => {
            match s {
                [] => true,
                ['l', 'e', '\u{1}', s @ ..] => {
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
        ['g', 'i', 'r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', 'o', 's', s @ ..] => {
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
        ['m', 'r', 'o', 'f', s @ ..] => {
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
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['a', 'l', s @ ..] => {
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
                ['e', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'u', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'i', 'v', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', 'i', 'n', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word venumdaturus.
pub(crate) fn n11518(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ndaturus.
pub(crate) fn n11519(s: &[char], _out: &mut Path<'_>) -> bool {
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
                        ['a', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 'm', s @ ..] => {
            match s {
                [] => true,
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'o', 'c', '\u{1}', s @ ..] => {
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
                ['b', 'a', s @ ..] => {
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
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 's', s @ ..] => {
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
                        ['u', 'i', s @ ..] => {
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
                ['d', 'e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'v', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', 'o', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -odaturus.
pub(crate) fn n11520(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['m', 'm', 'o', 'c', s @ ..] => {
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
        ['n', s @ ..] => {
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
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rdaturus.
pub(crate) fn n11521(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 't', s @ ..] => {
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
        ['o', 'c', s @ ..] => {
            match s {
                [] => true,
                ['e', 'r', '\u{1}', s @ ..] => {
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

/// Words ending -udaturus.
pub(crate) fn n11522(s: &[char], _out: &mut Path<'_>) -> bool {
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
                        ['d', 'a', '\u{1}', s @ ..] => {
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
                ['r', 'f', s @ ..] => {
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
        ['e', 'f', '\u{1}', s @ ..] => {
            s.is_empty()
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
                _ => false,
            }
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -eaturus.
pub(crate) fn n6677(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'l', 'a', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'i', 'l', s @ ..] => {
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
        ['r', 'c', s @ ..] => {
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
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
        ['u', 'q', 'a', 'l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -faturus.
pub(crate) fn n6678(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['o', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -gaturus.
pub(crate) fn n6679(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
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
                        ['d', 'a', '\u{1}', s @ ..] => {
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
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['l', 'a', '\u{1}', s @ ..] => {
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
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'g', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['g', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                        ['d', 'a', '\u{1}', s @ ..] => {
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
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', s @ ..] => {
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
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'f', s @ ..] => {
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
                        ['s', 'e', 'v', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['v', 'a', 'n', s @ ..] => {
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
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', '\u{1}', s @ ..] => {
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
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['o', 'v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
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
        ['o', 'r', s @ ..] => super::t8::n11538(s, out),
        ['r', 'u', s @ ..] => super::t8::n11539(s, out),
        ['u', s @ ..] => super::t8::n11540(s, out),
        _ => false,
    }
}

/// Words ending -bacchaturus.
pub(crate) fn n6680(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -iaturus.
pub(crate) fn n6681(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['a', 'l', 'g', s @ ..] => {
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
                ['o', 's', s @ ..] => {
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
                        ['s', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 's', s @ ..] => {
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
                        ['r', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['h', s @ ..] => {
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
                ['a', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'n', 'o', 'c', 'e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'h', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'u', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', 'x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                                        ['e', 'd', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['p', 'm', 'a', s @ ..] => {
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
        ['n', 'a', 'l', s @ ..] => {
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
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'a', '\u{1}', s @ ..] => {
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
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['e', 'r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'p', 'o', 'r', 'p', s @ ..] => {
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
                ['n', 'u', 'n', s @ ..] => {
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
                                ['r', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['b', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['n', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
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
        ['v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['l', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'b', s @ ..] => {
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
                                _ => false,
                            }
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

/// Words ending -laturus.
pub(crate) fn n6682(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['b', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['h', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
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
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
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
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['z', s @ ..] => {
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
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                        ['u', s @ ..] => {
                            match s {
                                [] => true,
                                ['s', s @ ..] => {
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
                                ['x', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', s @ ..] => {
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
                _ => false,
            }
        }
        ['g', 'a', 'r', 'f', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['b', 'i', 's', s @ ..] => {
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
                ['c', 'o', 'f', s @ ..] => {
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
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', 'i', 'v', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['h', 'i', 'n', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'i', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'a', '\u{1}', s @ ..] => {
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
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'o', '\u{1}', s @ ..] => {
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
                        ['n', 'e', 'v', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'm', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'a', '\u{1}', s @ ..] => {
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
        ['l', s @ ..] => super::t8::n11559(s, out),
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
        ['o', s @ ..] => super::t8::n11562(s, out),
        ['p', 'm', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => super::t8::n11563(s, out),
        ['r', 'e', s @ ..] => super::t8::n11564(s, out),
        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => super::t8::n11565(s, out),
        ['u', s @ ..] => super::t8::n11566(s, out),
        _ => false,
    }
}

/// Words ending -maturus.
pub(crate) fn n6683(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.97552454), (7, 0.012237736), (196, 0.012237736)], 1.0);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(7, 1.0)], 1.0);
            s.is_empty()
        }
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['h', 'd', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', s @ ..] => {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
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
                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['c', 'e', 'd', s @ ..] => {
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
                        ['e', '\u{1}', s @ ..] => {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
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
                        ['c', 'a', 'l', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['l', 'a', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['x', 'o', 'r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
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
        ['l', 'a', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', s @ ..] => {
            out.push(&[(0, 0.81596094), (196, 0.18292515)], 0.9988861);
            match s {
                [] => true,
                ['a', 'l', 'f', s @ ..] => {
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
                ['e', 'g', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    out.push(&[(196, 0.7276417), (0, 0.27199206)], 0.9996338);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
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
                ['i', 'f', s @ ..] => {
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
                        ['f', s @ ..] => {
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
                ['o', 'f', s @ ..] => {
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
                        ['f', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => super::t8::n20970(s, out),
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => super::t8::n20971(s, out),
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['s', 'a', 'l', 'p', s @ ..] => super::t8::n11575(s, out),
        ['u', s @ ..] => super::t8::n11576(s, out),
        _ => false,
    }
}

/// Words ending -naturus.
pub(crate) fn n6684(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9953155), (335, 0.00468452)], 1.0);
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
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'l', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'a', '\u{1}', s @ ..] => {
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
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['f', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['c', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['f', 'e', '\u{1}', s @ ..] => {
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
                ['t', 'a', 'c', s @ ..] => {
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
                ['v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['e', 'r', s @ ..] => {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', s @ ..] => {
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
                                ['a', 'm', '\u{1}', s @ ..] => {
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
                                ['s', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['u', 'p', s @ ..] => {
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
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', s @ ..] => {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => super::t8::n11580(s, out),
        ['m', 'a', 'd', s @ ..] => {
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
        ['n', 'a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => super::t8::n11583(s, out),
        ['r', s @ ..] => super::t8::n11584(s, out),
        ['u', 'd', 'a', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -boaturus.
pub(crate) fn n6685(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -paturus.
pub(crate) fn n6686(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['c', 'n', 'a', 'm', s @ ..] => {
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
                ['t', 's', s @ ..] => {
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
        ['l', 'a', 'p', s @ ..] => {
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
        ['r', 'u', 't', s @ ..] => {
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
        ['u', 'c', 'c', 'o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -raturus.
pub(crate) fn n6687(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', s @ ..] => {
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
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', 'h', s @ ..] => {
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
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => {
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
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
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
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['e', 'l', 'e', 'c', s @ ..] => {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', s @ ..] => {
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
                        ['r', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'u', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['u', 'c', 'u', 'l', s @ ..] => {
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
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['a', 's', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['u', 'l', s @ ..] => {
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
                _ => false,
            }
        }
        ['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => super::t8::n11597(s, out),
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'f', s @ ..] => {
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
                                _ => false,
                            }
                        }
                        ['r', 'f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 't', 'n', 'i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
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
                                ['e', 'd', '\u{1}', s @ ..] => super::t8::n23316(s, out),
                                _ => false,
                            }
                        }
                        ['p', '\u{1}', s @ ..] => super::t8::n21120(s, out),
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => super::t8::n11599(s, out),
        ['o', s @ ..] => super::t8::n11600(s, out),
        ['r', s @ ..] => super::t8::n11601(s, out),
        ['t', s @ ..] => super::t8::n11602(s, out),
        ['u', s @ ..] => super::t8::n11603(s, out),
        ['y', 'g', s @ ..] => super::t8::n11604(s, out),
        _ => false,
    }
}

/// Words ending -saturus.
pub(crate) fn n6688(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 'r', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['a', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
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
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', s @ ..] => {
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
                                _ => false,
                            }
                        }
                        ['h', 'e', 'r', 'p', s @ ..] => {
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
                        ['p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'o', 'c', '\u{1}', s @ ..] => {
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
                ['i', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 's', s @ ..] => {
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                                ['i', 'g', 'r', 'e', 't', '\u{1}', s @ ..] => {
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
                ['u', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'o', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['g', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['u', 'q', s @ ..] => {
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
                ['u', 'm', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -taturus.
pub(crate) fn n6689(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'n', s @ ..] => {
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
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                        ['f', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'b', 'a', 'l', '\u{1}', s @ ..] => {
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
                                ['b', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 't', s @ ..] => {
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
                                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
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
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', 'o', s @ ..] => {
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
                                ['e', 'd', s @ ..] => {
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
                        ['m', 'u', s @ ..] => {
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
                                        ['e', 'r', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['n', 'i', s @ ..] => {
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
                                ['x', 'e', s @ ..] => {
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
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['i', '\u{1}', s @ ..] => super::t8::n24102(s, out),
                                        ['o', 'c', '\u{1}', s @ ..] => super::t8::n24103(s, out),
                                        _ => false,
                                    }
                                }
                                ['r', 'e', 'p', '\u{1}', s @ ..] => super::t8::n23433(s, out),
                                ['s', 'a', '\u{1}', s @ ..] => super::t8::n23434(s, out),
                                _ => false,
                            }
                        }
                        ['v', s @ ..] => super::t8::n21238(s, out),
                        _ => false,
                    }
                }
                ['i', 'd', s @ ..] => super::t8::n17008(s, out),
                ['r', 'a', s @ ..] => super::t8::n17009(s, out),
                ['u', s @ ..] => super::t8::n17010(s, out),
                _ => false,
            }
        }
        ['e', s @ ..] => super::t8::n11614(s, out),
        ['i', s @ ..] => super::t8::n11615(s, out),
        ['l', 'a', 'x', 'e', s @ ..] => super::t8::n11616(s, out),
        ['n', s @ ..] => super::t8::n11617(s, out),
        ['o', s @ ..] => super::t8::n11618(s, out),
        ['p', s @ ..] => super::t8::n11619(s, out),
        ['r', s @ ..] => super::t8::n11620(s, out),
        ['s', s @ ..] => super::t8::n11621(s, out),
        ['u', s @ ..] => super::t8::n11622(s, out),
        _ => false,
    }
}

/// Words ending -uaturus.
pub(crate) fn n6690(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'a', 'v', 'e', s @ ..] => {
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
        ['l', 'l', 'e', s @ ..] => {
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
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['e', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
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
                        ['t', 'n', 'o', 'c', s @ ..] => {
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
                _ => false,
            }
        }
        ['q', s @ ..] => {
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
                ['e', 'a', s @ ..] => {
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
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'l', s @ ..] => {
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
                        ['l', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', 'p', 'o', 'r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
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
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['a', 'f', s @ ..] => {
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
                ['s', 'e', 'a', s @ ..] => {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
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
        _ => false,
    }
}

/// Words ending -vaturus.
pub(crate) fn n6691(s: &[char], _out: &mut Path<'_>) -> bool {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'g', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['e', 'l', s @ ..] => {
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
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', '\u{1}', s @ ..] => {
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
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
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
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'a', s @ ..] => {
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
                        ['s', s @ ..] => {
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
                                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['s', 'a', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -xaturus.
pub(crate) fn n6692(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'l', s @ ..] => {
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
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['l', 'p', 'm', 'a', s @ ..] => {
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
                ['v', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -befacturus.
pub(crate) fn n21426(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['l', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -licefacturus.
pub(crate) fn n21427(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['d', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -defacturus.
pub(crate) fn n21428(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'm', s @ ..] => {
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
        ['n', 'a', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -lefacturus.
pub(crate) fn n21430(s: &[char], _out: &mut Path<'_>) -> bool {
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
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'b', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -mefacturus.
pub(crate) fn n21431(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'r', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nefacturus.
pub(crate) fn n21432(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'b', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -pefacturus.
pub(crate) fn n21433(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', 't', 's', s @ ..] => {
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

/// Words ending -refacturus.
pub(crate) fn n21434(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
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
        ['b', 'a', 'f', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'o', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', 'u', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', 'd', 'b', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -uefacturus.
pub(crate) fn n21436(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'a', 'v', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['q', 'i', 'l', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -vefacturus.
pub(crate) fn n21437(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'f', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word lucrifacturus.
pub(crate) fn n17093(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -lfacturus.
pub(crate) fn n17094(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word satisfacturus.
pub(crate) fn n17095(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word iacturus.
pub(crate) fn n11639(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word circumacturus.
pub(crate) fn n11640(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word nacturus.
pub(crate) fn n11641(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word coacturus.
pub(crate) fn n11642(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -pacturus.
pub(crate) fn n11643(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -racturus.
pub(crate) fn n11644(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'e', '\u{1}', s @ ..] => {
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
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
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
                        ['t', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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

/// The word transacturus.
pub(crate) fn n11645(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -tacturus.
pub(crate) fn n11646(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word exacturus.
pub(crate) fn n11647(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ecturus.
pub(crate) fn n6694(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', s @ ..] => {
            out.push(&[(0, 0.9395285), (324, 0.060471505)], 1.0);
            match s {
                [] => true,
                ['e', 'd', s @ ..] => {
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
                ['o', s @ ..] => {
                    out.push(&[(0, 0.6124056), (324, 0.3875944)], 1.0);
                    s.is_empty()
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
        ['l', s @ ..] => {
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
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', s @ ..] => {
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
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', 'e', 'n', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', s @ ..] => {
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
                                ['t', 'n', 'i', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['b', 'u', 's', '\u{1}', s @ ..] => {
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
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 's', s @ ..] => {
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
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['o', 'r', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', s @ ..] => {
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', '\u{1}', s @ ..] => {
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
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'p', s @ ..] => {
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
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', '\u{1}', s @ ..] => {
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
                        ['u', 's', s @ ..] => {
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
                                ['s', 'a', '\u{1}', s @ ..] => {
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
        ['s', s @ ..] => super::t8::n11653(s, out),
        ['t', s @ ..] => super::t8::n11654(s, out),
        ['v', s @ ..] => super::t8::n11655(s, out),
        _ => false,
    }
}

/// Words ending -icturus.
pub(crate) fn n6695(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'r', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['v', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', 'e', 'b', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'n', 'a', '\u{1}', s @ ..] => {
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
                ['r', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'n', 'i', '\u{1}', s @ ..] => {
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
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                _ => false,
            }
        }
        ['l', s @ ..] => {
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
                        ['r', s @ ..] => {
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
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['f', 'a', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['m', s @ ..] => {
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
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
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
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ncturus.
pub(crate) fn n6696(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', s @ ..] => {
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
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
                ['l', s @ ..] => {
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
                                ['d', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['l', 'o', 'p', '\u{1}', s @ ..] => {
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
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
                ['v', s @ ..] => {
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
                        _ => false,
                    }
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
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
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
                        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
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

/// Words ending -octurus.
pub(crate) fn n6697(s: &[char], _out: &mut Path<'_>) -> bool {
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
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
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
        ['d', s @ ..] => {
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
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
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

/// Words ending -ucturus.
pub(crate) fn n6698(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['a', 'r', 't', '\u{1}', s @ ..] => {
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
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
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
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
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
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', s @ ..] => {
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
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'r', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'n', 'i', s @ ..] => {
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
                        _ => false,
                    }
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
                        ['t', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', 'u', 's', '\u{1}', s @ ..] => {
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
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
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
                ['f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', 'o', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
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
                ['t', s @ ..] => {
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
                                        ['d', '\u{1}', s @ ..] => {
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
                                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['x', 'e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['x', 'e', s @ ..] => {
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -esciturus.
pub(crate) fn n17277(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word exciturus.
pub(crate) fn n11708(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -diturus.
pub(crate) fn n6707(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
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
        ['d', 'a', s @ ..] => {
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
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'o', '\u{1}', s @ ..] => {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
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
                        ['c', s @ ..] => {
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
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['a', 'l', 'b', s @ ..] => {
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
                ['e', 'v', s @ ..] => {
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
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['s', 'b', 'a', '\u{1}', s @ ..] => {
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
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 's', 'u', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['e', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                ['a', s @ ..] => {
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
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
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
        _ => false,
    }
}

/// Words ending -eiturus.
pub(crate) fn n6708(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', 'n', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word suffiturus.
pub(crate) fn n6709(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -giturus.
pub(crate) fn n6710(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'a', 'l', s @ ..] => {
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
        ['u', 'f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
        _ => false,
    }
}

/// Words ending -liturus.
pub(crate) fn n6711(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                        _ => false,
                    }
                }
                _ => false,
            }
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['b', 'a', 't', 's', s @ ..] => {
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
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
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
                                        ['\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['u', 'b', s @ ..] => {
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
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
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
                ['v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'e', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -miturus.
pub(crate) fn n6712(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'g', s @ ..] => {
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
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['d', s @ ..] => {
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
                ['v', s @ ..] => {
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
        ['r', 'o', 'd', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -niturus.
pub(crate) fn n6713(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'n', 'i', s @ ..] => {
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
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['a', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['l', s @ ..] => {
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
                ['v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'n', 'i', '\u{1}', s @ ..] => {
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
                ['f', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
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
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
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
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
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
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
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
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
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
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
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
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
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

/// Words ending -oiturus.
pub(crate) fn n6714(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 't', 'n', 'i', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -piturus.
pub(crate) fn n6715(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'r', 't', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
        ['o', 's', s @ ..] => {
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
        ['u', 'c', s @ ..] => {
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

/// Words ending -riturus.
pub(crate) fn n6716(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['s', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
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
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
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
                        ['p', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
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
        ['o', s @ ..] => {
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
                ['h', '\u{1}', s @ ..] => {
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
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
        ['r', 'e', 't', s @ ..] => {
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
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'a', '\u{1}', s @ ..] => {
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
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
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
                ['t', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'n', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', '\u{1}', s @ ..] => {
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
        ['u', 'g', 'i', 'l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -siturus.
pub(crate) fn n6717(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['a', 'u', 'q', s @ ..] => {
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
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'u', 'q', s @ ..] => {
            match s {
                [] => true,
                ['c', 'a', '\u{1}', s @ ..] => {
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['a', 'r', 't', s @ ..] => {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'c', '\u{1}', s @ ..] => {
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
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
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
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'n', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', s @ ..] => {
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
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 's', s @ ..] => {
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
                ['r', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', 'u', 's', '\u{1}', s @ ..] => {
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
                        ['i', 'd', s @ ..] => {
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
                        ['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
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
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -titurus.
pub(crate) fn n6718(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
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
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
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
                ['p', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', s @ ..] => {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', 'e', 'm', s @ ..] => {
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
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['a', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 's', s @ ..] => {
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
                _ => false,
            }
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['b', 'o', '\u{1}', s @ ..] => {
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
                        ['v', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
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
                ['o', 'p', '\u{1}', s @ ..] => {
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

/// Words ending -uiturus.
pub(crate) fn n6719(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['q', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'n', '\u{1}', s @ ..] => {
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
                ['f', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -viturus.
pub(crate) fn n6720(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'a', 's', s @ ..] => {
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
        ['r', 'e', s @ ..] => {
            match s {
                [] => true,
                ['f', s @ ..] => {
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
                        _ => false,
                    }
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

/// The word exiturus.
pub(crate) fn n6721(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -sorpturus.
pub(crate) fn n6736(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['e', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['p', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -upturus.
pub(crate) fn n6737(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['l', 'g', s @ ..] => {
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
        ['n', s @ ..] => {
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
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', s @ ..] => {
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
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
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word diminuturus.
pub(crate) fn n11913(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -mminuturus.
pub(crate) fn n11914(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word inminuturus.
pub(crate) fn n11915(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -sputurus.
pub(crate) fn n6756(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -suturus.
pub(crate) fn n6757(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -fututurus.
pub(crate) fn n6758(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['c', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['f', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -sequuturus.
pub(crate) fn n6759(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -tuberculatus.
pub(crate) fn n18018(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 'b', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -osculatus.
pub(crate) fn n12618(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -dulatus.
pub(crate) fn n7335(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
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

/// Words ending -angulatus.
pub(crate) fn n7336(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['b', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 's', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -eiulatus.
pub(crate) fn n7337(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ululatus.
pub(crate) fn n7338(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -mulatus.
pub(crate) fn n7339(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['s', s @ ..] => {
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
                ['t', 's', s @ ..] => {
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
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -anulatus.
pub(crate) fn n7340(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'g', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -pulatus.
pub(crate) fn n7341(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -serrulatus.
pub(crate) fn n7342(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -sulatus.
pub(crate) fn n7343(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -tulatus.
pub(crate) fn n7344(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['n', s @ ..] => {
            out.push(&[(15, 0.6704967), (0, 0.32832837)], 0.9988251);
            s.is_empty()
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

/// The word exulatus.
pub(crate) fn n7345(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ominatus.
pub(crate) fn n12745(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rminatus.
pub(crate) fn n12746(s: &[char], _out: &mut Path<'_>) -> bool {
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
                                ['n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -uminatus.
pub(crate) fn n12747(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', s @ ..] => {
            out.push(&[(0, 0.5917143), (12, 0.4082246)], 0.99993896);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['c', '\u{1}', s @ ..] => {
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
                ['l', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -pinatus.
pub(crate) fn n7401(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9105211), (336, 0.08787671)], 0.9983978);
    match s {
        [] => true,
        ['a', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
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
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    out.push(&[(336, 0.695964), (0, 0.30350193)], 0.99946594);
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
        ['u', 's', s @ ..] => {
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

/// The word carinatus.
pub(crate) fn n12751(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -icarinatus.
pub(crate) fn n12752(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tinatus.
pub(crate) fn n7403(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.92581064), (7, 0.073243305)], 0.99905396);
    match s {
        [] => true,
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['b', s @ ..] => {
                    out.push(&[(7, 0.6910811), (0, 0.30859846)], 0.99967957);
                    s.is_empty()
                }
                ['e', s @ ..] => {
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

/// Words ending -uinatus.
pub(crate) fn n7404(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.74499124), (7, 0.25178912)], 0.9967804);
    match s {
        [] => true,
        ['q', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'c', s @ ..] => {
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
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -neratus.
pub(crate) fn n7452(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['e', 'd', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -peratus.
pub(crate) fn n7453(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['e', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
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
                        ['x', 'e', '\u{1}', s @ ..] => {
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
                ['e', s @ ..] => {
                    out.push(&[(7, 0.6763714), (0, 0.31787595), (15, 0.0032501717), (12, 0.0024872206)], 0.99998474);
                    match s {
                        [] => true,
                        ['d', '\u{1}', s @ ..] => {
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

/// The word immiseratus.
pub(crate) fn n22147(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word commiseratus.
pub(crate) fn n22148(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -teratus.
pub(crate) fn n7455(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['b', 'o', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(0, 0.70766765), (7, 0.28987563)], 0.9975433);
            match s {
                [] => true,
                ['i', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -ilitatus.
pub(crate) fn n13039(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -mitatus.
pub(crate) fn n7537(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
        _ => false,
    }
}

/// Words ending -pitatus.
pub(crate) fn n7538(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', s @ ..] => {
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
        ['e', 'r', 'c', s @ ..] => {
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
        ['i', 't', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'u', 'n', 'e', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', 'o', 'h', s @ ..] => {
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

/// Words ending -sitatus.
pub(crate) fn n7539(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.787274), (15, 0.106584266), (7, 0.10614175)], 1.0);
    match s {
        [] => true,
        ['i', 'v', s @ ..] => {
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
        ['u', s @ ..] => {
            out.push(&[(0, 0.39818418), (15, 0.30093843), (7, 0.3008774)], 1.0);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['c', 'c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    out.push(&[(15, 0.76697946), (0, 0.13272297), (7, 0.10029755)], 1.0);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
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

/// Words ending -actitatus.
pub(crate) fn n7540(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -vitatus.
pub(crate) fn n7541(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -pinandus.
pub(crate) fn n10595(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.86634624), (332, 0.13365377)], 1.0);
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
                    out.push(&[(332, 0.711223), (0, 0.288777)], 1.0);
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

/// Words ending -tinandus.
pub(crate) fn n10596(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word inquinandus.
pub(crate) fn n15228(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word coinquinandus.
pub(crate) fn n15229(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word pignerandus.
pub(crate) fn n19448(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word oppignerandus.
pub(crate) fn n19449(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -vulnerandus.
pub(crate) fn n15303(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -onerandus.
pub(crate) fn n15304(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -munerandus.
pub(crate) fn n15305(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -perandus.
pub(crate) fn n10640(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['m', 'e', 't', s @ ..] => {
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

/// Words ending -serandus.
pub(crate) fn n10641(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -terandus.
pub(crate) fn n10642(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
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

/// Words ending -clamitandus.
pub(crate) fn n15476(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word limitandus.
pub(crate) fn n19625(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -comitandus.
pub(crate) fn n15478(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -pitandus.
pub(crate) fn n10714(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'r', 'c', s @ ..] => {
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
        ['s', 'o', 'h', s @ ..] => {
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

/// Words ending -usitandus.
pub(crate) fn n10715(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -actitandus.
pub(crate) fn n10716(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -equitandus.
pub(crate) fn n10717(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -vitandus.
pub(crate) fn n10718(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word gnoscendus.
pub(crate) fn n22628(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word agnoscendus.
pub(crate) fn n22629(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word adgnoscendus.
pub(crate) fn n22630(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ignoscendus.
pub(crate) fn n22631(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -cognoscendus.
pub(crate) fn n22632(s: &[char], _out: &mut Path<'_>) -> bool {
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
        _ => false,
    }
}

/// The word dinoscendus.
pub(crate) fn n19786(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ernoscendus.
pub(crate) fn n19787(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -poscendus.
pub(crate) fn n15683(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -indendus.
pub(crate) fn n10840(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['c', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
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
                ['f', 'i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ondendus.
pub(crate) fn n10841(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 's', s @ ..] => {
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
        ['t', s @ ..] => {
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
                ['t', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -undendus.
pub(crate) fn n10842(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.953933), (325, 0.04606699)], 1.0);
    match s {
        [] => true,
        ['f', s @ ..] => {
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
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
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
                ['o', s @ ..] => {
                    out.push(&[(325, 0.68201727), (0, 0.31798276)], 1.0);
                    match s {
                        [] => true,
                        ['r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'e', 'p', s @ ..] => {
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
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -iungendus.
pub(crate) fn n15833(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word pungendus.
pub(crate) fn n20012(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word compungendus.
pub(crate) fn n20013(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word conpungendus.
pub(crate) fn n20014(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word interpungendus.
pub(crate) fn n20015(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word expungendus.
pub(crate) fn n20016(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word perungendus.
pub(crate) fn n15836(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -udicaturus.
pub(crate) fn n20620(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'l', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
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
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 'e', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ificaturus.
pub(crate) fn n16681(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['d', 'e', 'a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'g', 'i', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'o', 'l', 'g', s @ ..] => {
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
        ['v', 'i', 'v', s @ ..] => {
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

/// Words ending -plicaturus.
pub(crate) fn n16682(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 'n', 'e', 'c', '\u{1}', s @ ..] => {
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

