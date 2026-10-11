use flexion::Path;

/// Words ending -stans.
pub(crate) fn n789(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.6300145), (2, 0.33398947), (16, 0.018295567), (17, 0.017669948)], 0.9999695);
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
                ['a', s @ ..] => {
                    out.push(&[(16, 0.4505379), (17, 0.22810712), (1, 0.21000992), (2, 0.111329824)], 0.99998474);
                    match s {
                        [] => true,
                        ['r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
                ['r', '\u{1}', s @ ..] => {
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
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
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
            out.push(&[(1, 0.4503853), (2, 0.34097812), (17, 0.13836882), (16, 0.0702678)], 1.0);
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(1, 0.35088122), (17, 0.31578547), (2, 0.17543298), (16, 0.15790036)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                            s.is_empty()
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

/// Words ending -utans.
pub(crate) fn n790(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -docens.
pub(crate) fn n2162(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.66045624), (4, 0.33026627), (59, 0.004623484), (60, 0.004623484)], 0.9999695);
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

/// The word nocens.
pub(crate) fn n5131(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word innocens.
pub(crate) fn n5132(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(59, 0.6666667), (60, 0.33333334)], 1.0);
    s.is_empty()
}

/// Words ending -arcens.
pub(crate) fn n817(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', 'a', '\u{1}', s @ ..] => {
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
        ['p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -scens.
pub(crate) fn n818(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6523995), (4, 0.34073395), (33, 0.0022888533), (327, 0.0022735943)], 0.99769586);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 'i', 'v', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['u', 'c', 's', 'u', 'p', 'e', 'r', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                        ['g', 'a', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['r', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'i', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['p', 's', 'e', 'v', s @ ..] => {
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
                        ['t', 'e', 'v', s @ ..] => {
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
        ['e', s @ ..] => super::t4::n2169(s, out),
        ['i', s @ ..] => {
            out.push(&[(3, 0.6539559), (4, 0.32710764), (327, 0.012603953), (328, 0.006286717)], 0.99995416);
            match s {
                [] => true,
                ['c', s @ ..] => {
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
                                _ => false,
                            }
                        }
                        ['i', s @ ..] => {
                            out.push(&[(3, 0.39745173), (327, 0.2691844), (4, 0.19874877), (328, 0.13458458)], 0.9999695);
                            match s {
                                [] => true,
                                ['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(327, 0.6666667), (328, 0.33333334)], 1.0);
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
                                ['a', '\u{1}', s @ ..] => {
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
                                ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['g', 'n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['p', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                ['m', 'i', '\u{1}', s @ ..] => {
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
                                ['o', 'd', s @ ..] => {
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['o', s @ ..] => super::t4::n2171(s, out),
        _ => false,
    }
}

/// Words ending -ucens.
pub(crate) fn n819(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
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
                                ['i', 'l', 'l', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 'r', 't', 'i', 'n', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 'u', 'f', 'l', 'u', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                ['a', 'r', 't', '\u{1}', s @ ..] => {
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
                ['i', 'd', '\u{1}', s @ ..] => {
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
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', '\u{1}', s @ ..] => {
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

/// Words ending -idens.
pub(crate) fn n824(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.63382924), (4, 0.35336843), (67, 0.008453499), (66, 0.004242008)], 0.9998932);
    match s {
        [] => true,
        ['b', '\u{1}', s @ ..] => {
            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['n', 'i', s @ ..] => {
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
                ['u', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                ['c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['d', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'q', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', '\u{1}', s @ ..] => {
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
                        ['\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                ['p', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                                ['r', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', 'd', '\u{1}', s @ ..] => {
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
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['a', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['u', 'c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'q', 'e', 'a', 'n', 'i', '\u{1}', s @ ..] => {
            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['v', s @ ..] => {
            out.push(&[(3, 0.5953765), (4, 0.31100938), (67, 0.0624094), (66, 0.0312047)], 1.0);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(67, 0.6666667), (66, 0.33333334)], 1.0);
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
                ['i', 'd', s @ ..] => {
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
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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

/// Words ending -ndens.
pub(crate) fn n825(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6576333), (4, 0.3333486), (329, 0.005966278), (330, 0.0029755093)], 0.9999237);
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
                        ['s', s @ ..] => {
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
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['c', 's', 'e', s @ ..] => {
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
                                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['f', 'f', 'o', s @ ..] => {
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
                ['h', 'e', 'r', 'p', s @ ..] => {
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
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'p', 's', s @ ..] => {
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
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'c', 'o', 'l', 'f', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['m', 'i', s @ ..] => {
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
                        ['n', 'i', s @ ..] => {
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
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', '\u{1}', s @ ..] => {
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
                                ['e', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', s @ ..] => super::t4::n9536(s, out),
                        ['t', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', s @ ..] => super::t4::n9538(s, out),
                        _ => false,
                    }
                }
                ['v', s @ ..] => super::t4::n5263(s, out),
                _ => false,
            }
        }
        ['i', s @ ..] => super::t4::n2202(s, out),
        ['o', s @ ..] => super::t4::n2203(s, out),
        ['u', s @ ..] => super::t4::n2204(s, out),
        _ => false,
    }
}

/// Words ending -plodens.
pub(crate) fn n2205(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rodens.
pub(crate) fn n2206(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rdens.
pub(crate) fn n827(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6173037), (4, 0.30916303), (66, 0.048599984), (67, 0.0243534)], 0.99942017);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(3, 0.39012742), (66, 0.27638668), (4, 0.19516289), (67, 0.13820097)], 0.9998779);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(66, 0.6666667), (67, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
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
        ['o', 'm', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -udens.
pub(crate) fn n828(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.59546804), (4, 0.32014954), (59, 0.03636225), (60, 0.02809186), (338, 0.013214313), (339, 0.0066071567)], 0.9998932);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(60, 0.6666667), (59, 0.33333334)], 1.0);
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
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['g', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                ['l', s @ ..] => {
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
                                ['p', 'a', '\u{1}', s @ ..] => {
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
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            out.push(&[(3, 0.66488135), (4, 0.33299762)], 0.99787897);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', 'o', '\u{1}', s @ ..] => {
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
                        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
                ['l', s @ ..] => {
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
                _ => false,
            }
        }
        ['p', s @ ..] => {
            out.push(&[(59, 0.5520714), (60, 0.27783626), (3, 0.10826276), (4, 0.058213167), (338, 0.0024109255)], 0.9987945);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            out.push(&[(3, 0.5527581), (4, 0.36549935), (338, 0.052811474), (339, 0.026398107)], 0.99746704);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    out.push(&[(4, 0.44873732), (3, 0.27370107), (338, 0.18481727), (339, 0.092408635)], 0.9996643);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(338, 0.6666667), (339, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
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
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'b', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -rigens.
pub(crate) fn n2229(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['f', '\u{1}', s @ ..] => {
            s.is_empty()
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
                                ['e', 'r', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -lgens.
pub(crate) fn n832(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['f', s @ ..] => {
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
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
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
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -ngens.
pub(crate) fn n833(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['r', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(59, 0.6666667), (60, 0.33333334)], 1.0);
                    s.is_empty()
                }
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
                ['n', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                        ['p', 'u', 's', '\u{1}', s @ ..] => {
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
                                ['r', 'e', 'p', '\u{1}', s @ ..] => {
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
                                ['b', 'o', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => super::t4::n14509(s, out),
                                ['e', s @ ..] => super::t4::n14510(s, out),
                                ['i', 'd', '\u{1}', s @ ..] => super::t4::n14511(s, out),
                                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => super::t4::n14512(s, out),
                                ['n', s @ ..] => super::t4::n14513(s, out),
                                ['r', 'e', 'p', '\u{1}', s @ ..] => super::t4::n14514(s, out),
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => super::t4::n5362(s, out),
                _ => false,
            }
        }
        ['u', s @ ..] => super::t4::n2236(s, out),
        _ => false,
    }
}

/// The word cogens.
pub(crate) fn n834(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -rgens.
pub(crate) fn n835(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'p', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', '\u{1}', s @ ..] => {
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
        ['e', s @ ..] => {
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
                                ['u', 's', '\u{1}', s @ ..] => {
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
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
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
                        ['s', 'b', 'a', '\u{1}', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
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

/// Words ending -ugens.
pub(crate) fn n836(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['l', s @ ..] => {
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
        ['s', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -aciens.
pub(crate) fn n2270(s: &[char], out: &mut Path<'_>) -> bool {
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
                        ['b', s @ ..] => {
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
                        ['c', 'i', 'l', s @ ..] => {
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
                        ['d', s @ ..] => {
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
                        ['g', 'r', 'e', 'p', 'x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                                            s.is_empty()
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
                        ['m', s @ ..] => {
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
                        ['n', s @ ..] => {
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
                        ['p', s @ ..] => {
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
                        ['r', s @ ..] => {
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
                        ['t', 'a', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', s @ ..] => {
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
                        ['v', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'e', 'l', 'a', 'u', 'q', 's', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'c', 'u', 'l', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
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
                ['o', 'r', 'e', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
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
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
        ['l', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -iciens.
pub(crate) fn n2273(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.64927137), (4, 0.3383688), (67, 0.008209354), (66, 0.004089418)], 0.99993896);
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
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                    out.push(&[(3, 0.48763257), (4, 0.2458381), (67, 0.17767605), (66, 0.088838026)], 0.99998474);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.35088122), (67, 0.31578547), (4, 0.17543298), (66, 0.15790036)], 1.0);
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
                ['m', 'o', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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

/// Words ending -nciens.
pub(crate) fn n2275(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', 'v', s @ ..] => {
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
                _ => false,
            }
        }
        ['o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rciens.
pub(crate) fn n2277(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['e', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -sciens.
pub(crate) fn n2278(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.46015108), (4, 0.38504615), (60, 0.10289159), (59, 0.051453423)], 0.99954224);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(60, 0.6666667), (59, 0.33333334)], 1.0);
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
        ['i', 'n', 'm', 'o', '\u{1}', s @ ..] => {
            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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

/// Words ending -diens.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(3, 0.46912336), (4, 0.23590448), (10, 0.19516289), (11, 0.097581446)], 0.9977722);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(10, 0.6666667), (11, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['r', 'g', s @ ..] => {
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(10, 0.6666667), (11, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['g', s @ ..] => {
                            out.push(&[(3, 0.66517127), (4, 0.33287555)], 0.9980468);
                            match s {
                                [] => true,
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
                                ['g', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
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
                ['o', 'c', '\u{1}', s @ ..] => {
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
                ['r', s @ ..] => {
                    out.push(&[(10, 0.455375), (11, 0.22767986), (3, 0.20572214), (4, 0.10601968)], 0.9947967);
                    match s {
                        [] => true,
                        ['p', '\u{1}', s @ ..] => {
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

/// Words ending -eiens.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(10, 0.44808117), (11, 0.22404821), (3, 0.21672389), (4, 0.10940719)], 0.99826044);
    match s {
        [] => true,
        ['a', 'r', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', s @ ..] => {
            out.push(&[(3, 0.5166857), (4, 0.25868618), (10, 0.1493553), (11, 0.07467765)], 0.99940485);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', 'n', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -fiens.
pub(crate) fn n845(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -giens.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6655222), (4, 0.31747922), (178, 0.015503166)], 0.9985046);
    match s {
        [] => true,
        ['a', 's', s @ ..] => {
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
        ['u', s @ ..] => {
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
                        ['r', 'e', s @ ..] => {
                            match s {
                                [] => true,
                                ['p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['t', 'b', 'u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'a', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -liens.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6659342), (4, 0.33310446)], 0.9990387);
    match s {
        [] => true,
        ['a', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', 'e', 's', s @ ..] => {
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

/// Words ending -miens.
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['r', 'o', 'd', s @ ..] => {
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
                _ => false,
            }
        }
        ['u', s @ ..] => {
            out.push(&[(10, 0.455375), (11, 0.22767986), (3, 0.20572214), (4, 0.10601968)], 0.9947967);
            match s {
                [] => true,
                ['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -niens.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.640116), (4, 0.33969635), (10, 0.013351644), (11, 0.0066834516)], 0.9998474);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['n', 'i', s @ ..] => {
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
                ['s', 'e', 'v', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                                ['o', 'c', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['n', 'i', '\u{1}', s @ ..] => {
                                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
            out.push(&[(3, 0.5659419), (4, 0.28442818), (10, 0.099748224), (11, 0.04988174)], 1.0);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(10, 0.6666667), (11, 0.33333334)], 1.0);
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
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', 'i', 'h', s @ ..] => {
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

/// Words ending -oiens.
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(10, 0.38564125), (11, 0.192813), (117, 0.19047837), (118, 0.095231555), (3, 0.08816663), (4, 0.045441367)], 0.99777216);
    match s {
        [] => true,
        ['c', s @ ..] => {
            out.push(&[(117, 0.5079423), (118, 0.25397116), (10, 0.128542), (11, 0.064271), (3, 0.029388877), (4, 0.0151522085)], 0.9992676);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            out.push(&[(10, 0.6104677), (11, 0.3052262), (117, 0.038101777), (118, 0.01904326), (3, 0.01763943), (4, 0.009094377)], 0.9995727);
            match s {
                [] => true,
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
        _ => false,
    }
}

/// Words ending -piens.
pub(crate) fn n851(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6382238), (4, 0.3401541), (60, 0.014007783), (59, 0.0069733732)], 0.9993591);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(3, 0.5473411), (4, 0.33373007), (60, 0.07925536), (59, 0.03962768)], 0.9999542);
            match s {
                [] => true,
                ['c', s @ ..] => {
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
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(60, 0.6666667), (59, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                ['s', 'n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -riens.
pub(crate) fn n852(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.5946288), (4, 0.30565345), (10, 0.05537499), (11, 0.027679866), (66, 0.011017014), (67, 0.005508507)], 0.9998627);
    match s {
        [] => true,
        ['a', s @ ..] => {
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
        ['e', s @ ..] => {
            out.push(&[(3, 0.45140764), (4, 0.22592507), (10, 0.17924774), (11, 0.0896315), (66, 0.035843443), (67, 0.017929351)], 0.99998474);
            match s {
                [] => true,
                ['f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(10, 0.6666667), (11, 0.33333334)], 1.0);
                            s.is_empty()
                        }
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
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', s @ ..] => {
                                    out.push(&[(10, 0.5041886), (11, 0.2520943), (3, 0.15046921), (4, 0.07530327), (66, 0.011947814), (67, 0.0059815366)], 0.99998474);
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
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
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            out.push(&[(3, 0.6471046), (4, 0.32356757), (10, 0.016296636), (11, 0.008148318), (66, 0.0032654307)], 0.99838257);
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
                        ['p', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', s @ ..] => {
                            out.push(&[(10, 0.5041886), (11, 0.2520943), (3, 0.15046921), (4, 0.07530327), (66, 0.011947814), (67, 0.0059815366)], 0.99998474);
                            match s {
                                [] => true,
                                ['i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['x', s @ ..] => {
                            out.push(&[(66, 0.45639735), (67, 0.22819868), (3, 0.15046921), (4, 0.07530327), (10, 0.05975433), (11, 0.029877165)], 1.0);
                            match s {
                                [] => true,
                                ['e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    out.push(&[(10, 0.5691768), (11, 0.28459603), (3, 0.09028763), (4, 0.045181964), (66, 0.0071717403), (67, 0.0035858701)], 1.0);
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
        ['o', s @ ..] => {
            out.push(&[(3, 0.6640421), (4, 0.33232623), (10, 0.002014191)], 0.99838257);
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
            out.push(&[(3, 0.6416266), (4, 0.35500115)], 0.99662775);
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
                ['s', 'e', s @ ..] => {
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
                ['t', 'c', 'i', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -siens.
pub(crate) fn n853(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.38075837), (10, 0.2841535), (4, 0.19137865), (11, 0.14207675)], 0.99836725);
    match s {
        [] => true,
        ['n', s @ ..] => {
            out.push(&[(10, 0.5901579), (11, 0.29507896), (3, 0.07615778), (4, 0.038269628)], 0.9996643);
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
                _ => false,
            }
        }
        ['s', s @ ..] => {
            out.push(&[(3, 0.6094911), (4, 0.3049363), (10, 0.0568246), (11, 0.0284123)], 0.9996643);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tiens.
pub(crate) fn n854(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6252537), (4, 0.32259098), (10, 0.011749447), (60, 0.011490043), (33, 0.011459526), (11, 0.0058747237), (34, 0.005737392), (59, 0.005737392)], 0.9998932);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(3, 0.3791867), (4, 0.21559472), (60, 0.13447776), (33, 0.13447776), (34, 0.06724651), (59, 0.06724651)], 0.99823);
            match s {
                [] => true,
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(60, 0.6666667), (59, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', s @ ..] => {
                                    out.push(&[(33, 0.4892653), (34, 0.24463265), (3, 0.12639047), (4, 0.07186999), (60, 0.044831008), (59, 0.022415504)], 0.9994049);
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
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
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
        ['s', s @ ..] => {
            out.push(&[(3, 0.57502097), (4, 0.28816664), (10, 0.089677274), (11, 0.044831008)], 0.9976959);
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['g', s @ ..] => {
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
                ['o', s @ ..] => {
                    out.push(&[(10, 0.47434196), (11, 0.23717098), (3, 0.19166857), (4, 0.096055545)], 0.99923706);
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
        ['u', s @ ..] => {
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
                ['l', 'g', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -uiens.
pub(crate) fn n855(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(10, 0.44808117), (11, 0.22404821), (3, 0.21672389), (4, 0.10940719)], 0.99826044);
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
                ['n', s @ ..] => {
                    out.push(&[(3, 0.5166857), (4, 0.25868618), (10, 0.1493553), (11, 0.07467765)], 0.99940485);
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

/// Words ending -viens.
pub(crate) fn n856(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
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

/// Words ending -xiens.
pub(crate) fn n857(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(10, 0.455375), (11, 0.22767986), (3, 0.20572214), (4, 0.10601968)], 0.9947967);
    match s {
        [] => true,
        ['e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tollens.
pub(crate) fn n5573(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'u', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
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

/// Words ending -olens.
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.54316014), (4, 0.33649194), (33, 0.04303044), (34, 0.03994812), (59, 0.024750134), (60, 0.012466621)], 0.9998474);
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
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'a', '\u{1}', s @ ..] => {
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
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
        ['e', 'v', 'a', s @ ..] => {
            match s {
                [] => true,
                ['r', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['u', 's', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'v', '\u{1}', s @ ..] => {
            out.push(&[(33, 0.6666667), (34, 0.33333334)], 1.0);
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
        ['n', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', s @ ..] => {
            out.push(&[(33, 0.458793), (34, 0.23553826), (3, 0.18104829), (4, 0.11216907), (59, 0.008255131), (60, 0.004150454)], 0.9999542);
            match s {
                [] => true,
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['v', s @ ..] => {
            out.push(&[(4, 0.26709393), (3, 0.18353552), (34, 0.18310827), (59, 0.1810788), (33, 0.0946212), (60, 0.09054703)], 0.99998474);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(34, 0.6666667), (33, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            out.push(&[(59, 0.50479895), (60, 0.25239947), (4, 0.08903639), (3, 0.06117342), (34, 0.061036088), (33, 0.031540398)], 0.99998474);
                            match s {
                                [] => true,
                                ['e', 'b', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['i', 'l', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -plens.
pub(crate) fn n864(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word opulens.
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
    s.is_empty()
}

/// The word praemonens.
pub(crate) fn n10017(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word remonens.
pub(crate) fn n10018(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -mmonens.
pub(crate) fn n5656(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word conmonens.
pub(crate) fn n5657(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word promonens.
pub(crate) fn n5658(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word permonens.
pub(crate) fn n5659(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ponens.
pub(crate) fn n2398(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
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

/// Words ending -ernens.
pub(crate) fn n877(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word introferens.
pub(crate) fn n10080(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word perferens.
pub(crate) fn n5722(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word transferens.
pub(crate) fn n5723(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word auferens.
pub(crate) fn n5724(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -gerens.
pub(crate) fn n2425(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['g', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -herens.
pub(crate) fn n2426(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -merens.
pub(crate) fn n2427(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -serens.
pub(crate) fn n2428(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['i', 'm', '\u{1}', s @ ..] => {
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
        ['s', s @ ..] => {
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
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -terens.
pub(crate) fn n2429(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word conquerens.
pub(crate) fn n2430(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -verens.
pub(crate) fn n2431(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.48313114), (4, 0.355169), (67, 0.1067216), (66, 0.053650722)], 0.9986725);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', 'u', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(3, 0.35088122), (67, 0.31578547), (4, 0.17543298), (66, 0.15790036)], 1.0);
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['r', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word exerens.
pub(crate) fn n2432(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -frens.
pub(crate) fn n886(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(266, 0.4458686), (267, 0.2229343), (3, 0.2087129), (4, 0.115556575), (66, 0.002059968)], 0.9951324);
    s.is_empty()
}

/// The word aegrens.
pub(crate) fn n887(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -irens.
pub(crate) fn n888(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['u', 'q', s @ ..] => {
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
        ['v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 't', 'e', 'a', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['o', 'v', 'a', 'l', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['r', 'e', 'p', 'm', 'e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -orens.
pub(crate) fn n889(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.51401544), (4, 0.26074618), (66, 0.1488365), (67, 0.07473869)], 0.9983368);
    match s {
        [] => true,
        ['l', 'f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(66, 0.6666667), (67, 0.33333334)], 1.0);
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
                ['f', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rrens.
pub(crate) fn n890(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6661479), (4, 0.3335012)], 0.99964905);
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

/// Words ending -urens.
pub(crate) fn n891(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['f', s @ ..] => {
            out.push(&[(3, 0.6531624), (181, 0.2229343), (4, 0.115556575), (66, 0.002059968)], 0.99371326);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
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

/// Words ending -fitens.
pub(crate) fn n2465(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.46938276), (4, 0.2386511), (329, 0.19455253), (330, 0.09727626)], 0.9998627);
    match s {
        [] => true,
        ['o', s @ ..] => {
            out.push(&[(329, 0.50930035), (330, 0.25464255), (3, 0.156466), (4, 0.07954528)], 0.99995416);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nitens.
pub(crate) fn n2466(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'i', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'p', '\u{1}', s @ ..] => {
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
        ['o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -otens.
pub(crate) fn n902(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(4, 0.44492257), (3, 0.2699321), (67, 0.055817503), (60, 0.04481575), (227, 0.044586863), (275, 0.044586863), (66, 0.028015565), (59, 0.022507057), (228, 0.022293432), (276, 0.022293432)], 0.9997711);
    match s {
        [] => true,
        ['n', '\u{1}', s @ ..] => {
            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(60, 0.6666667), (59, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    out.push(&[(4, 0.5323262), (3, 0.26837566), (227, 0.06408789), (275, 0.06408789), (228, 0.032043945), (276, 0.032043945), (67, 0.0025940337), (60, 0.002090486)], 0.9976501);
                    match s {
                        [] => true,
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['g', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', s @ ..] => {
                                    out.push(&[(275, 0.46581215), (276, 0.23289844), (4, 0.17744717), (3, 0.089463644), (227, 0.02136263), (228, 0.010681315)], 0.99766535);
                                    match s {
                                        [] => true,
                                        ['o', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['r', s @ ..] => {
                            out.push(&[(4, 0.37312886), (227, 0.27948424), (3, 0.18701458), (228, 0.13974212), (275, 0.012817578), (276, 0.006408789)], 0.9985962);
                            match s {
                                [] => true,
                                ['u', 'a', 'l', '\u{1}', s @ ..] => {
                                    out.push(&[(227, 0.6666667), (228, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['m', s @ ..] => {
                    out.push(&[(67, 0.46305028), (66, 0.23155566), (4, 0.14830244), (3, 0.08998245), (60, 0.014938583), (227, 0.014862288), (275, 0.014862288), (59, 0.007507439), (228, 0.007431144), (276, 0.007431144)], 0.9999237);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(67, 0.6666667), (66, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rtens.
pub(crate) fn n903(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.66498816), (4, 0.3336843)], 0.9986725);
    match s {
        [] => true,
        ['e', 'v', s @ ..] => {
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
                ['d', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
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
                        ['t', 'n', 'a', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -sistens.
pub(crate) fn n904(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
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

/// Words ending -ttens.
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6529183), (4, 0.34605935)], 0.99897766);
    match s {
        [] => true,
        ['a', 'b', '\u{1}', s @ ..] => {
            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['i', 'm', s @ ..] => {
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

/// Words ending -utens.
pub(crate) fn n906(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -minuens.
pub(crate) fn n2501(s: &[char], _out: &mut Path<'_>) -> bool {
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
        _ => false,
    }
}

/// Words ending -spuens.
pub(crate) fn n913(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -quens.
pub(crate) fn n914(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6009308), (4, 0.3333486), (59, 0.020202946), (60, 0.020202946), (66, 0.013458458), (67, 0.0067139696), (49, 0.0033569848)], 0.9982147);
    match s {
        [] => true,
        ['e', s @ ..] => {
            out.push(&[(3, 0.53931487), (4, 0.3154803), (59, 0.07214466), (60, 0.07214466)], 0.9990845);
            match s {
                [] => true,
                ['r', 'f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(59, 0.6666667), (60, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                                ['o', s @ ..] => {
                                    out.push(&[(60, 0.46849775), (59, 0.24626535), (3, 0.17976654), (4, 0.105165176)], 0.9996948);
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
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
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'c', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['n', 'i', '\u{1}', s @ ..] => {
                                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                                            s.is_empty()
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
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['n', 'i', 'l', s @ ..] => {
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
        ['o', s @ ..] => {
            out.push(&[(3, 0.59453726), (4, 0.34509805), (66, 0.03169299), (67, 0.015838865), (49, 0.007919433), (50, 0.003921569)], 0.9990082);
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
                            out.push(&[(3, 0.43642327), (4, 0.3346456), (66, 0.15167467), (67, 0.07583734)], 0.9985809);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(66, 0.6666667), (67, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                                ['n', s @ ..] => {
                                    out.push(&[(3, 0.3963531), (4, 0.23006028), (49, 0.22749676), (50, 0.11372549), (66, 0.021133745), (67, 0.010559243)], 0.9993286);
                                    match s {
                                        [] => true,
                                        ['g', 'a', 'm', '\u{1}', s @ ..] => {
                                            out.push(&[(49, 0.6666667), (50, 0.33333334)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['v', 'e', 'r', 'b', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
        ['r', 'o', 't', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -ruens.
pub(crate) fn n915(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6276646), (4, 0.32054627), (34, 0.025711453), (33, 0.012817578), (178, 0.012817578)], 0.9995575);
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
        ['g', s @ ..] => {
            out.push(&[(3, 0.3753872), (4, 0.33150226), (34, 0.1941558), (33, 0.09706264)], 0.9981079);
            match s {
                [] => true,
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(34, 0.6666667), (33, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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

/// Words ending -suens.
pub(crate) fn n916(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -tuens.
pub(crate) fn n917(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['n', 'm', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 's', s @ ..] => {
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

/// Words ending -andus.
pub(crate) fn n995(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99905396)], 0.99905396);
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
                ['l', 'a', s @ ..] => {
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
                ['m', 'u', 'l', 'p', s @ ..] => {
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
                ['o', 'r', 'p', s @ ..] => {
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
                        ['p', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'c', 'a', s @ ..] => {
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
                        ['u', 't', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['b', 'o', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['c', s @ ..] => super::t4::n2670(s, out),
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'r', 'p', s @ ..] => {
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
                        ['o', 'f', '\u{1}', s @ ..] => {
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
                        ['r', 'e', 'h', 'x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => super::t4::n10428(s, out),
                        _ => false,
                    }
                }
                ['i', s @ ..] => super::t4::n6114(s, out),
                ['m', 'u', 'n', 'e', 'v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => super::t4::n6116(s, out),
                ['o', s @ ..] => super::t4::n6117(s, out),
                ['r', s @ ..] => super::t4::n6118(s, out),
                ['u', s @ ..] => super::t4::n6119(s, out),
                _ => false,
            }
        }
        ['e', s @ ..] => super::t4::n2672(s, out),
        ['f', s @ ..] => super::t4::n2673(s, out),
        ['g', s @ ..] => super::t5::n2674(s, out),
        ['h', 'c', 'c', 'a', 'b', s @ ..] => super::t5::n2675(s, out),
        ['i', s @ ..] => super::t5::n2676(s, out),
        ['l', s @ ..] => super::t5::n2677(s, out),
        ['m', s @ ..] => super::t5::n2678(s, out),
        ['n', s @ ..] => super::t5::n2679(s, out),
        ['o', 'b', s @ ..] => super::t5::n2680(s, out),
        ['p', s @ ..] => super::t5::n2681(s, out),
        ['r', s @ ..] => super::t5::n2682(s, out),
        ['s', s @ ..] => super::t5::n2683(s, out),
        ['t', s @ ..] => super::t5::n2684(s, out),
        ['u', s @ ..] => super::t5::n2685(s, out),
        ['v', s @ ..] => super::t5::n2686(s, out),
        ['x', s @ ..] => super::t5::n2687(s, out),
        _ => false,
    }
}

/// Words ending -endus.
pub(crate) fn n996(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9978332)], 0.9978332);
    match s {
        [] => true,
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['h', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['l', 'e', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', s @ ..] => {
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
                                _ => false,
                            }
                        }
                        ['r', 'c', 's', s @ ..] => {
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
                                        ['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
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
                                ['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
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
                ['m', 'a', 'l', s @ ..] => {
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
                ['r', 'o', 's', s @ ..] => {
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
                ['u', s @ ..] => {
                    match s {
                        [] => true,
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', s @ ..] => super::t5::n2689(s, out),
        ['d', s @ ..] => super::t5::n2690(s, out),
        ['g', s @ ..] => super::t5::n2691(s, out),
        ['h', s @ ..] => super::t5::n2692(s, out),
        ['i', s @ ..] => super::t5::n2693(s, out),
        ['l', s @ ..] => super::t5::n2694(s, out),
        ['m', s @ ..] => super::t5::n2695(s, out),
        ['n', s @ ..] => super::t5::n2696(s, out),
        ['p', s @ ..] => super::t5::n2697(s, out),
        ['r', s @ ..] => super::t5::n2698(s, out),
        ['s', s @ ..] => super::t5::n2699(s, out),
        ['t', s @ ..] => super::t5::n2700(s, out),
        ['u', s @ ..] => super::t5::n2701(s, out),
        ['v', s @ ..] => super::t5::n2702(s, out),
        ['x', 'e', 't', s @ ..] => super::t5::n2703(s, out),
        _ => false,
    }
}

/// Words ending -labundus.
pub(crate) fn n11154(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 't', 'n', 'e', 'v', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['v', 'a', 'c', '\u{1}', s @ ..] => {
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
        ['p', 'm', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'p', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'n', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', 'o', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', 'a', 'r', 'g', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word noscitabundus.
pub(crate) fn n20418(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ntabundus.
pub(crate) fn n16310(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -scribundus.
pub(crate) fn n6315(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'd', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -cundus.
pub(crate) fn n2705(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8392462), (7, 0.07614252), (15, 0.07598993), (12, 0.0085297935)], 0.99990845);
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
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['f', s @ ..] => {
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
                ['r', s @ ..] => {
                    out.push(&[(0, 0.5678492), (15, 0.415198), (7, 0.015228504)], 0.9982757);
                    match s {
                        [] => true,
                        ['e', 'v', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
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
                ['s', s @ ..] => {
                    out.push(&[(7, 0.6920424), (0, 0.27974364), (15, 0.025329977), (12, 0.002838178)], 0.9999542);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(15, 0.51898986), (0, 0.45981535), (7, 0.01904326), (12, 0.002136263)], 0.99998474);
            match s {
                [] => true,
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['b', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', 'i', 'v', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            out.push(&[(0, 0.9876402), (7, 0.0058594644), (15, 0.0058442056)], 0.9993439);
            match s {
                [] => true,
                ['o', 'n', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['g', '\u{1}', s @ ..] => {
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
                ['d', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    out.push(&[(0, 0.68734264), (7, 0.24864577), (12, 0.05491722), (15, 0.009094377)], 1.0);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 0.81632715), (12, 0.18367285)], 1.0);
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
        _ => false,
    }
}

/// Words ending -edundus.
pub(crate) fn n2706(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['c', 's', 'b', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'c', 'n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -eundus.
pub(crate) fn n2707(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9547112), (114, 0.044541083)], 0.99925226);
    match s {
        [] => true,
        ['b', 'a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(0, 0.70782024), (114, 0.29207295)], 0.9998932);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(114, 0.76401925), (0, 0.235935)], 0.9999542);
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
        ['r', 'e', 'p', s @ ..] => {
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
        ['s', 'n', 'a', 'r', 't', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -fundus.
pub(crate) fn n2708(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.67347217), (0, 0.32077515), (15, 0.004089418)], 0.99833673);
    s.is_empty()
}

/// Words ending -gundus.
pub(crate) fn n2709(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
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
        ['n', 'i', s @ ..] => {
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
                ['r', 't', 's', s @ ..] => {
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

/// Words ending -iundus.
pub(crate) fn n2710(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99931335)], 0.99931335);
    match s {
        [] => true,
        ['c', 'a', 'f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'o', 'b', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'a', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', 'e', 'v', s @ ..] => {
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

/// Words ending -mundus.
pub(crate) fn n2711(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.73949796), (7, 0.13034256), (15, 0.12982376)], 0.9996643);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(7, 1.0)], 1.0);
            s.is_empty()
        }
        ['e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['d', 'e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            out.push(&[(15, 0.70994127), (0, 0.24649423), (7, 0.043442436)], 0.9998779);
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ernundus.
pub(crate) fn n2712(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
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
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ferundus.
pub(crate) fn n2713(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['f', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tundus.
pub(crate) fn n2714(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8544442), (7, 0.14431983)], 0.99876404);
    match s {
        [] => true,
        ['c', 'e', 'n', s @ ..] => {
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
        ['o', 'r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -varius.
pub(crate) fn n2815(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', 'b', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -erius.
pub(crate) fn n1065(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.92484933), (371, 0.07470817)], 0.9995575);
    match s {
        [] => true,
        ['s', s @ ..] => {
            out.push(&[(371, 0.6915694), (0, 0.30827802)], 0.9998474);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -orius.
pub(crate) fn n1066(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.993225), (119, 0.006729229)], 0.9999542);
    match s {
        [] => true,
        ['s', s @ ..] => {
            out.push(&[(0, 0.94569314), (119, 0.054306857)], 1.0);
            match s {
                [] => true,
                ['i', 'r', s @ ..] => {
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
                ['n', 'e', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'i', s @ ..] => {
                    out.push(&[(119, 0.68476385), (0, 0.31523612)], 1.0);
                    s.is_empty()
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
                        ['c', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', 'r', 'p', s @ ..] => {
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
                                ['o', 'v', s @ ..] => {
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
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['g', 'o', 'r', 'e', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                        ['m', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['l', 'c', s @ ..] => {
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
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['g', 'u', 'p', s @ ..] => {
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
                                ['i', 'm', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['o', 'n', '\u{1}', s @ ..] => {
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
                                ['a', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['p', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
                                        ['m', '\u{1}', s @ ..] => {
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
                ['e', 'a', 'l', s @ ..] => {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'o', s @ ..] => {
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
                        ['r', 'e', 'm', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -proprius.
pub(crate) fn n1067(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['m', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -trius.
pub(crate) fn n1068(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.71227586), (203, 0.19047837), (15, 0.095231555)], 0.9979858);
    match s {
        [] => true,
        ['s', s @ ..] => {
            out.push(&[(203, 0.5079423), (15, 0.25397116), (0, 0.23743038)], 0.9993439);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -yrius.
pub(crate) fn n1069(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'p', '\u{1}', s @ ..] => {
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
                ['s', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -vinissimus.
pub(crate) fn n16388(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(58, 0.7632715), (0, 0.23662165)], 0.9998932);
    s.is_empty()
}

/// Words ending -rissimus.
pub(crate) fn n6492(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.82473487), (58, 0.1751583)], 0.9998932);
            match s {
                [] => true,
                ['l', 'c', s @ ..] => {
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
                ['m', s @ ..] => {
                    out.push(&[(58, 0.7250477), (0, 0.27490655)], 0.9999542);
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    out.push(&[(58, 0.7250477), (0, 0.27490655)], 0.9999542);
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
                ['e', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['m', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -sissimus.
pub(crate) fn n6493(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['o', s @ ..] => {
            out.push(&[(0, 0.9996643)], 0.9996643);
            s.is_empty()
        }
        ['s', s @ ..] => {
            out.push(&[(0, 0.7743343), (58, 0.22540627)], 0.9997406);
            match s {
                [] => true,
                ['e', s @ ..] => {
                    out.push(&[(58, 0.7418021), (0, 0.25810635)], 0.99990845);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -tissimus.
pub(crate) fn n6494(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.9996033)], 0.9996033);
            match s {
                [] => true,
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['p', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', 'i', 's', 'n', 'o', 'c', s @ ..] => {
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
                                ['l', 'e', 'c', 's', s @ ..] => {
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', 'e', 'l', s @ ..] => {
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
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(0, 0.88482493), (58, 0.115098804)], 0.9999237);
            match s {
                [] => true,
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(58, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['p', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['m', 'i', '\u{1}', s @ ..] => {
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
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'r', 'e', '\u{1}', s @ ..] => {
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
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['v', 'i', s @ ..] => {
                    out.push(&[(58, 0.7050278), (0, 0.29494163)], 0.9999695);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'u', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
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
                ['a', s @ ..] => {
                    out.push(&[(0, 0.87785155), (58, 0.122087434)], 0.99993896);
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
                            out.push(&[(58, 0.7073625), (0, 0.29262227)], 0.99998474);
                            s.is_empty()
                        }
                        ['r', 'e', 'p', 'm', 'e', 't', s @ ..] => {
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
                        ['t', 'c', s @ ..] => {
                            out.push(&[(58, 0.7073625), (0, 0.29262227)], 0.99998474);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', s @ ..] => {
                    out.push(&[(0, 0.9996948)], 0.9996948);
                    match s {
                        [] => true,
                        ['c', 'o', 'n', s @ ..] => {
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
                        ['d', 'u', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['g', 'e', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['l', 'g', 'e', 'n', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['a', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', 'o', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'n', 'm', 'o', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'i', '\u{1}', s @ ..] => {
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
        ['o', 'n', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'e', 'c', s @ ..] => {
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
        ['s', 'e', 'n', 'o', 'h', s @ ..] => {
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
        ['u', 'l', 'o', 's', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -uissimus.
pub(crate) fn n6495(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8646067), (401, 0.13348593)], 0.9980926);
    match s {
        [] => true,
        ['n', s @ ..] => {
            out.push(&[(401, 0.711162), (0, 0.28819716)], 0.99935913);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -vissimus.
pub(crate) fn n6496(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'n', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            out.push(&[(58, 0.67623407), (0, 0.32301822)], 0.9992523);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -surus.
pub(crate) fn n1195(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.995758), (324, 0.002121004), (331, 0.002121004)], 1.0);
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
                        ['c', 'o', '\u{1}', s @ ..] => {
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
                                ['d', '\u{1}', s @ ..] => {
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
                ['u', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
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
                        ['h', s @ ..] => {
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
                                _ => false,
                            }
                        }
                        ['l', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['b', 'm', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['h', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 's', 'i', 'd', '\u{1}', s @ ..] => {
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
                ['c', 'n', 'i', s @ ..] => {
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
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['f', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
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
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', '\u{1}', s @ ..] => {
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
                ['v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'g', s @ ..] => {
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
                                ['g', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                        ['i', 'd', s @ ..] => {
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['l', 'u', s @ ..] => super::t5::n3023(s, out),
        ['n', s @ ..] => super::t5::n3024(s, out),
        ['o', s @ ..] => super::t5::n3025(s, out),
        ['p', 'a', 'l', s @ ..] => super::t5::n3026(s, out),
        ['r', s @ ..] => super::t5::n3027(s, out),
        ['s', s @ ..] => super::t5::n3028(s, out),
        ['u', s @ ..] => super::t5::n3029(s, out),
        _ => false,
    }
}

/// Words ending -turus.
pub(crate) fn n1196(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99887085)], 0.99887085);
    match s {
        [] => true,
        ['a', s @ ..] => super::t5::n3030(s, out),
        ['c', s @ ..] => super::t5::n3031(s, out),
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['i', 'u', 'q', s @ ..] => {
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
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
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
                                ['f', 'e', '\u{1}', s @ ..] => {
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
                ['r', 'c', s @ ..] => {
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
                ['u', 's', s @ ..] => {
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
                        ['s', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => super::t5::n3033(s, out),
        ['l', s @ ..] => {
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
                        ['s', '\u{1}', s @ ..] => {
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
                        ['c', s @ ..] => super::t5::n11778(s, out),
                        ['d', s @ ..] => super::t5::n11779(s, out),
                        ['f', s @ ..] => super::t5::n11780(s, out),
                        ['p', 'e', 's', s @ ..] => super::t5::n11781(s, out),
                        ['s', s @ ..] => super::t5::n11782(s, out),
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', s @ ..] => super::t5::n3035(s, out),
        ['o', s @ ..] => super::t5::n3036(s, out),
        ['p', s @ ..] => super::t5::n3037(s, out),
        ['r', s @ ..] => super::t5::n3038(s, out),
        ['s', s @ ..] => super::t5::n3039(s, out),
        ['u', s @ ..] => super::t5::n3040(s, out),
        ['x', s @ ..] => super::t5::n3041(s, out),
        _ => false,
    }
}

/// Words ending -nixurus.
pub(crate) fn n6765(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['n', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -fluxurus.
pub(crate) fn n3044(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['f', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -mensus.
pub(crate) fn n3100(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -pensus.
pub(crate) fn n3101(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.81327534), (15, 0.18614481)], 0.99942017);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(15, 1.0)], 1.0);
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
                ['c', 'c', 'o', 'l', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', 'i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
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
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(15, 0.7287099), (0, 0.2710918)], 0.99980164);
            match s {
                [] => true,
                ['r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'u', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -prensus.
pub(crate) fn n3102(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['m', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['p', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -sensus.
pub(crate) fn n3103(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -tensus.
pub(crate) fn n3104(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['n', s @ ..] => {
            out.push(&[(15, 0.68685436), (0, 0.30844587), (7, 0.0037384604)], 0.9990387);
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
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
        ['s', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -onsus.
pub(crate) fn n1219(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['m', s @ ..] => {
            out.push(&[(0, 0.6168765), (12, 0.33525598), (15, 0.04036011), (7, 0.00749218)], 0.99998474);
            match s {
                [] => true,
                ['r', 'o', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(12, 1.0)], 1.0);
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

/// The word tunsus.
pub(crate) fn n3108(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word obtunsus.
pub(crate) fn n3109(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(15, 1.0)], 1.0);
    s.is_empty()
}

/// The word retunsus.
pub(crate) fn n3110(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word contunsus.
pub(crate) fn n3111(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word pertunsus.
pub(crate) fn n3112(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word inofficiosus.
pub(crate) fn n23754(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word inartificiosus.
pub(crate) fn n23755(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -eliosus.
pub(crate) fn n6911(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(12, 0.71950865), (0, 0.27263296), (7, 0.004745556), (15, 0.0031128405)], 1.0);
    s.is_empty()
}

/// Words ending -niosus.
pub(crate) fn n3133(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.75045395), (7, 0.2391394), (15, 0.006942855), (12, 0.003479057)], 1.0000153);
    match s {
        [] => true,
        ['e', s @ ..] => {
            out.push(&[(7, 0.7463798), (0, 0.2501564), (15, 0.0023193713)], 0.9988556);
            s.is_empty()
        }
        ['u', s @ ..] => {
            out.push(&[(7, 0.7463798), (0, 0.2501564), (15, 0.0023193713)], 0.9988556);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -piosus.
pub(crate) fn n3134(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.7282826), (0, 0.21094072), (15, 0.040497445), (12, 0.02027924)], 1.0);
    s.is_empty()
}

/// Words ending -riosus.
pub(crate) fn n3135(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(0, 0.52655834), (15, 0.4243076), (7, 0.03697261), (12, 0.01216144)], 1.0);
            match s {
                [] => true,
                ['e', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(0, 0.9591974), (7, 0.020538643), (15, 0.013504234), (12, 0.006759747)], 1.0);
            s.is_empty()
        }
        ['o', s @ ..] => {
            out.push(&[(0, 0.5402914), (7, 0.22998399), (12, 0.21693751), (15, 0.012787061)], 1.0);
            match s {
                [] => true,
                ['b', s @ ..] => {
                    out.push(&[(12, 0.73897916), (0, 0.18010223), (7, 0.076661326), (15, 0.0042572673)], 1.0);
                    s.is_empty()
                }
                ['l', 'g', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
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
        ['u', s @ ..] => {
            out.push(&[(15, 0.45513085), (0, 0.37790492), (7, 0.16244754), (12, 0.0045166705)], 1.0);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['f', s @ ..] => {
                    out.push(&[(7, 0.72082096), (15, 0.15170519), (0, 0.12596323)], 0.9984894);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -tiosus.
pub(crate) fn n3136(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.51265734), (15, 0.20242618), (7, 0.16369879), (12, 0.12121767)], 1.0);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(15, 0.4404822), (7, 0.43274587), (0, 0.10252537), (12, 0.024246586)], 1.0);
            match s {
                [] => true,
                ['r', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            out.push(&[(0, 0.9486992), (15, 0.021301595), (7, 0.017227435), (12, 0.012756542)], 0.99998474);
            s.is_empty()
        }
        ['e', s @ ..] => {
            out.push(&[(7, 0.72123295), (0, 0.17088579), (15, 0.06747539), (12, 0.04040589)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(15, 0.34457922), (0, 0.30626383), (12, 0.22896162), (7, 0.12021057)], 1.0000153);
            match s {
                [] => true,
                ['b', s @ ..] => {
                    out.push(&[(15, 0.7815213), (0, 0.102082856), (12, 0.076325625), (7, 0.04007019)], 1.0);
                    s.is_empty()
                }
                ['d', s @ ..] => {
                    out.push(&[(12, 0.7429923), (15, 0.114854656), (0, 0.102082856), (7, 0.04007019)], 1.0);
                    s.is_empty()
                }
                ['g', s @ ..] => {
                    out.push(&[(0, 0.4612497), (12, 0.44579232), (15, 0.06890974), (7, 0.024048219)], 1.0);
                    match s {
                        [] => true,
                        ['a', 'l', 'f', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
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
                ['l', s @ ..] => {
                    out.push(&[(15, 0.7815213), (0, 0.102082856), (12, 0.076325625), (7, 0.04007019)], 1.0);
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    out.push(&[(15, 0.7815213), (0, 0.102082856), (12, 0.076325625), (7, 0.04007019)], 1.0);
                    s.is_empty()
                }
                ['v', s @ ..] => {
                    out.push(&[(7, 0.70673686), (15, 0.114854656), (0, 0.102082856), (12, 0.076325625)], 1.0);
                    s.is_empty()
                }
                ['x', s @ ..] => {
                    out.push(&[(0, 0.76874954), (15, 0.114854656), (12, 0.076325625), (7, 0.04007019)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            out.push(&[(0, 0.77387655), (7, 0.19670404), (15, 0.01840238), (12, 0.011017014)], 1.0);
            match s {
                [] => true,
                ['e', 'c', s @ ..] => {
                    out.push(&[(7, 0.73223466), (0, 0.25795376), (15, 0.006134127), (12, 0.0036774243)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(0, 0.5025254), (12, 0.42424658), (15, 0.040482186), (7, 0.03274586)], 1.0);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(12, 1.0)], 1.0);
                    s.is_empty()
                }
                ['g', 'e', 'n', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            out.push(&[(15, 0.73414207), (0, 0.17088579), (7, 0.054566264), (12, 0.04040589)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -viosus.
pub(crate) fn n3137(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9591974), (7, 0.020538643), (15, 0.013504234), (12, 0.006759747)], 1.0);
    s.is_empty()
}

/// Words ending -losus.
pub(crate) fn n1228(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9547417), (12, 0.043884948)], 0.99862665);
    match s {
        [] => true,
        ['i', 'p', s @ ..] => {
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
        ['l', 'e', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'l', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'd', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'm', 'r', 'o', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(12, 1.0)], 1.0);
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
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'm', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'r', s @ ..] => {
                            out.push(&[(0, 0.5909514), (12, 0.40877393)], 0.99972534);
                            match s {
                                [] => true,
                                ['b', 'e', 'f', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['l', 'a', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'u', 'm', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['d', s @ ..] => {
                    out.push(&[(0, 0.62813765), (12, 0.3716182)], 0.99975586);
                    match s {
                        [] => true,
                        ['i', 'm', 'r', 'o', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
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
                        ['n', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'r', '\u{1}', s @ ..] => {
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

/// Words ending -mosus.
pub(crate) fn n1229(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.7909056), (7, 0.123277634), (12, 0.08465705)], 0.9988403);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.64924085), (12, 0.340383), (7, 0.0102693215)], 0.9998932);
            match s {
                [] => true,
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    out.push(&[(12, 0.7801328), (0, 0.21641871), (7, 0.003418021)], 0.9999695);
                    s.is_empty()
                }
                ['u', 'q', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 't', 'u', 'c', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(0, 0.6867933), (7, 0.30650797), (12, 0.0066071567)], 0.99990845);
            match s {
                [] => true,
                ['n', s @ ..] => {
                    out.push(&[(7, 0.8555276), (0, 0.14308386)], 0.99861145);
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
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
                        ['c', 'a', 'l', '\u{1}', s @ ..] => {
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
        ['r', s @ ..] => {
            out.push(&[(7, 0.7077592), (0, 0.26363012), (12, 0.028213931)], 0.9996033);
            s.is_empty()
        }
        ['u', s @ ..] => {
            out.push(&[(0, 0.98249793), (7, 0.010315099), (12, 0.007080186)], 0.9998932);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nosus.
pub(crate) fn n1230(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.90330356), (15, 0.0480354), (12, 0.032074463), (7, 0.01655604)], 0.9999695);
    match s {
        [] => true,
        ['a', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(0, 0.7004654), (15, 0.2925765), (12, 0.0045777066), (7, 0.0023651484)], 0.99998474);
                    match s {
                        [] => true,
                        ['s', s @ ..] => {
                            out.push(&[(15, 0.7641871), (0, 0.23349355)], 0.99768066);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['d', s @ ..] => {
                    out.push(&[(0, 0.7792935), (7, 0.21226825), (15, 0.0050507365), (12, 0.0033722438)], 0.9999847);
                    match s {
                        [] => true,
                        ['i', s @ ..] => {
                            out.push(&[(7, 0.73742276), (0, 0.2597696)], 0.9971924);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', s @ ..] => {
                    out.push(&[(0, 0.9964141)], 0.9964141);
                    match s {
                        [] => true,
                        ['i', 'l', 'u', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['m', s @ ..] => {
                    out.push(&[(0, 0.91776913), (12, 0.07968261)], 0.9974517);
                    match s {
                        [] => true,
                        ['i', s @ ..] => {
                            out.push(&[(0, 0.58355075), (12, 0.41593042)], 0.9994812);
                            match s {
                                [] => true,
                                ['r', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
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
                                ['b', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'u', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['t', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', 'l', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['u', 'm', '\u{1}', s @ ..] => {
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
                ['u', 'r', s @ ..] => {
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
                ['v', s @ ..] => {
                    out.push(&[(15, 0.6826734), (0, 0.30110627), (12, 0.010696574), (7, 0.005523766)], 1.0);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            out.push(&[(0, 0.61878383), (12, 0.3694667), (15, 0.0087281605), (7, 0.0030060273)], 0.99998474);
            match s {
                [] => true,
                ['u', 'r', 'e', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(12, 1.0)], 1.0);
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
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            out.push(&[(0, 0.7004654), (15, 0.2925765), (12, 0.0045777066), (7, 0.0023651484)], 0.99998474);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(15, 0.7641871), (0, 0.23349355)], 0.99768066);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rosus.
pub(crate) fn n1231(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8235447), (7, 0.13870451), (15, 0.033295184), (12, 0.0044556344)], 1.0);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', s @ ..] => {
                    out.push(&[(0, 0.44362554), (7, 0.3865568), (15, 0.16943619)], 0.99961853);
                    match s {
                        [] => true,
                        ['c', s @ ..] => {
                            out.push(&[(7, 0.82473487), (0, 0.1267567), (15, 0.048416875)], 0.99990845);
                            s.is_empty()
                        }
                        ['i', 'l', 'u', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['l', 'a', 's', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'e', 't', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'e', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', s @ ..] => {
                            out.push(&[(15, 0.7231403), (0, 0.14787517), (7, 0.12884718)], 0.9998627);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'r', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['c', 'u', 'l', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'u', '\u{1}', s @ ..] => {
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
                        ['i', 's', 'e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    out.push(&[(7, 0.71290153), (0, 0.27450982), (15, 0.011093309)], 0.99850464);
                    s.is_empty()
                }
                ['m', 'u', 'n', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    out.push(&[(0, 0.94338906), (12, 0.042206455), (7, 0.011612115), (15, 0.002792401)], 1.0);
                    match s {
                        [] => true,
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['g', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['v', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['a', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', '\u{1}', s @ ..] => {
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
                        ['o', s @ ..] => {
                            out.push(&[(7, 0.71290153), (0, 0.27450982), (15, 0.011093309)], 0.99850464);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                ['t', s @ ..] => {
                    out.push(&[(7, 0.71290153), (0, 0.27450982), (15, 0.011093309)], 0.99850464);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 'v', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['g', 'i', 'r', s @ ..] => {
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
                ['l', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    out.push(&[(0, 0.51865417), (7, 0.47728693), (15, 0.0035858701)], 0.999527);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'n', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 't', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    out.push(&[(7, 0.71290153), (0, 0.27450982), (15, 0.011093309)], 0.99850464);
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    out.push(&[(15, 0.5809415), (0, 0.3781491), (7, 0.03962768)], 0.99871826);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 's', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['e', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -tosus.
pub(crate) fn n1232(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.94737166), (7, 0.050278477)], 0.99765015);
    match s {
        [] => true,
        ['i', s @ ..] => {
            out.push(&[(0, 0.7067674), (7, 0.29289693)], 0.9996643);
            match s {
                [] => true,
                ['m', s @ ..] => {
                    out.push(&[(7, 0.7642939), (0, 0.23558404)], 0.9998779);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -uosus.
pub(crate) fn n1233(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.76359195), (7, 0.06811628), (15, 0.06755169), (12, 0.06736858), (215, 0.033371482)], 1.0);
    match s {
        [] => true,
        ['q', 'a', s @ ..] => {
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
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(0, 0.7175708), (7, 0.13788052), (12, 0.13781948), (15, 0.0045014117), (215, 0.0022278172)], 1.0);
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
                        ['u', s @ ..] => {
                            match s {
                                [] => true,
                                ['l', s @ ..] => {
                                    out.push(&[(7, 0.6108186), (0, 0.34787518), (12, 0.039383534)], 0.99807733);
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['f', '\u{1}', s @ ..] => {
                                            out.push(&[(0, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['r', 'f', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(12, 1.0)], 1.0);
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
                ['l', s @ ..] => {
                    out.push(&[(0, 0.55271226), (15, 0.41350424), (7, 0.013626306), (12, 0.013473717), (215, 0.006668193)], 0.99998474);
                    match s {
                        [] => true,
                        ['u', 'm', 'u', 't', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    out.push(&[(0, 0.43514153), (15, 0.2756695), (215, 0.2711223), (7, 0.009079118), (12, 0.008987564)], 1.0);
                    match s {
                        [] => true,
                        ['m', 'u', 's', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', s @ ..] => {
                                    out.push(&[(215, 0.75704587), (0, 0.14505227), (15, 0.09188983), (7, 0.0030212863), (12, 0.0029907683)], 1.0);
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
                ['r', 'o', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    out.push(&[(0, 0.6805066), (7, 0.29544517), (15, 0.009643702), (12, 0.009628443), (215, 0.004760815)], 0.99998474);
                    match s {
                        [] => true,
                        ['e', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', s @ ..] => {
                                    out.push(&[(7, 0.7651484), (0, 0.22684062), (15, 0.0032196536), (12, 0.0032043946)], 0.9984131);
                                    match s {
                                        [] => true,
                                        ['q', '\u{1}', s @ ..] => {
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
        ['x', s @ ..] => {
            out.push(&[(0, 0.6805066), (12, 0.29533836), (7, 0.009735256), (15, 0.009643702), (215, 0.004760815)], 0.99998474);
            match s {
                [] => true,
                ['e', 'l', 'f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
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

/// Words ending -vosus.
pub(crate) fn n1234(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.80143434), (15, 0.1866331), (7, 0.007888914), (12, 0.0038300145)], 0.9997864);
    match s {
        [] => true,
        ['r', s @ ..] => {
            out.push(&[(15, 0.7288777), (0, 0.2671397), (7, 0.0026245518)], 0.99864197);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word exosus.
pub(crate) fn n1235(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -cursus.
pub(crate) fn n1240(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ressus.
pub(crate) fn n3212(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.86257726), (15, 0.10247959), (124, 0.03416495)], 0.9992218);
    match s {
        [] => true,
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
                ['g', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                        ['p', s @ ..] => {
                            out.push(&[(15, 0.7008316), (0, 0.28752574), (124, 0.011383231)], 0.99974054);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
                ['r', 'e', s @ ..] => {
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
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', s @ ..] => {
                            out.push(&[(0, 0.5725185), (15, 0.42049286), (124, 0.006836042)], 0.9998474);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
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
                ['m', s @ ..] => {
                    out.push(&[(0, 0.5725185), (124, 0.40683603), (15, 0.020492867)], 0.9998474);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(124, 1.0)], 1.0);
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

/// Words ending -issus.
pub(crate) fn n1243(s: &[char], out: &mut Path<'_>) -> bool {
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
                    out.push(&[(0, 0.64005494), (7, 0.18832685), (15, 0.16974136)], 0.99812317);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', '\u{1}', s @ ..] => {
                            out.push(&[(0, 0.5263142), (15, 0.47368583)], 1.0);
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            out.push(&[(7, 0.7294423), (0, 0.21335164), (15, 0.056580454)], 0.9993744);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
        ['p', s @ ..] => {
            out.push(&[(7, 0.67795837), (0, 0.30421913), (15, 0.011047532), (124, 0.0022430762), (143, 0.0022430762), (325, 0.0022430762)], 0.9999543);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ossus.
pub(crate) fn n1244(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.94036776), (15, 0.058091097)], 0.99845886);
    match s {
        [] => true,
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
                        ['p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['m', 'i', '\u{1}', s @ ..] => {
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
        ['r', s @ ..] => {
            out.push(&[(15, 0.6860304), (0, 0.3134508)], 0.9994812);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ussus.
pub(crate) fn n1245(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99755853)], 0.99755853);
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['n', 'o', 'c', s @ ..] => {
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
                ['r', 'e', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
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
                ['e', 'd', 'i', 'f', '\u{1}', s @ ..] => {
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

/// Words ending -catus.
pub(crate) fn n1258(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', s @ ..] => {
            out.push(&[(0, 0.89338523), (12, 0.10556191)], 0.99894714);
            match s {
                [] => true,
                ['b', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
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
        ['b', 'o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['i', 's', s @ ..] => {
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
                ['o', '\u{1}', s @ ..] => {
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
                ['f', 'e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
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
            out.push(&[(0, 0.9813382), (7, 0.012420844), (15, 0.006210422)], 0.9999695);
            match s {
                [] => true,
                ['b', 'l', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', s @ ..] => super::t5::n7158(s, out),
                ['f', 'i', s @ ..] => {
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
                        ['n', 'g', 'i', 's', s @ ..] => super::t5::n12272(s, out),
                        ['r', 'o', 'l', 'g', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => super::t5::n17786(s, out),
                                ['n', 'o', 'c', '\u{1}', s @ ..] => super::t5::n17787(s, out),
                                _ => false,
                            }
                        }
                        ['v', 'i', 'v', s @ ..] => super::t5::n12274(s, out),
                        _ => false,
                    }
                }
                ['l', s @ ..] => super::t6::n7160(s, out),
                ['m', 'a', '\u{1}', s @ ..] => super::t6::n7161(s, out),
                ['n', 'r', 'o', 'f', s @ ..] => super::t6::n7162(s, out),
                ['p', s @ ..] => super::t6::n7163(s, out),
                ['r', s @ ..] => super::t6::n7164(s, out),
                _ => false,
            }
        }
        ['l', s @ ..] => super::t6::n3263(s, out),
        ['n', 'u', 'r', s @ ..] => super::t6::n3264(s, out),
        ['o', s @ ..] => super::t6::n3265(s, out),
        ['r', s @ ..] => super::t6::n3266(s, out),
        ['s', s @ ..] => super::t6::n3267(s, out),
        ['u', s @ ..] => super::t6::n3268(s, out),
        _ => false,
    }
}

