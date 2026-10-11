use flexion::Path;

/// Words ending -datus.
pub(crate) fn n1259(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.95564204), (15, 0.022720683), (7, 0.021606775)], 0.9999695);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['r', 'g', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
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
                ['s', s @ ..] => {
                    out.push(&[(0, 0.7287709), (7, 0.26788738), (15, 0.0033417258)], 1.0);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
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
                        ['a', 'l', '\u{1}', s @ ..] => {
                            s.is_empty()
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
        ['m', 'u', s @ ..] => {
            match s {
                [] => true,
                ['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'e', 'v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            out.push(&[(0, 0.91012436), (15, 0.08938735)], 0.9995117);
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
                ['e', s @ ..] => {
                    out.push(&[(15, 0.81788355), (0, 0.18202487)], 0.99990845);
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['u', s @ ..] => {
                    out.push(&[(0, 0.9971008), (15, 0.0028839551)], 0.99998474);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'a', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
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
        ['o', s @ ..] => {
            out.push(&[(0, 0.87222093), (7, 0.1263447)], 0.9985656);
            match s {
                [] => true,
                ['m', 'm', 'o', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', s @ ..] => {
                            out.push(&[(7, 0.70878154), (0, 0.2907454)], 0.999527);
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
        ['r', s @ ..] => super::t6::n3276(s, out),
        ['u', s @ ..] => super::t6::n3277(s, out),
        _ => false,
    }
}

/// Words ending -eatus.
pub(crate) fn n1260(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.96342415), (7, 0.018280309), (12, 0.018219272)], 0.9999237);
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(0, 0.7737087), (7, 0.22424659), (12, 0.00202945)], 0.99998474);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
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
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['c', 'u', 'n', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
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
                        ['c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', s @ ..] => {
                    out.push(&[(0, 0.59269094), (12, 0.40364692), (7, 0.0036621653)], 1.0);
                    match s {
                        [] => true,
                        ['c', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
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
                ['e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'm', '\u{1}', s @ ..] => {
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
                ['b', 'u', 's', '\u{1}', s @ ..] => {
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
                        ['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', 'l', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', 'u', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['g', 'i', 'n', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['v', 'a', 'l', 'f', '\u{1}', s @ ..] => {
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
                ['c', s @ ..] => {
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
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 'p', '\u{1}', s @ ..] => {
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
                ['u', 'a', s @ ..] => {
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

/// Words ending -gatus.
pub(crate) fn n1262(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9773556), (7, 0.015075914), (12, 0.007537957)], 0.9999695);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.9120622), (7, 0.0876173)], 0.9996795);
            match s {
                [] => true,
                ['v', s @ ..] => {
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
                        ['r', s @ ..] => {
                            out.push(&[(7, 0.6958724), (0, 0.30402076)], 0.9998932);
                            match s {
                                [] => true,
                                ['e', 'p', '\u{1}', s @ ..] => {
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
            out.push(&[(0, 0.9782711), (12, 0.021560999)], 0.99983215);
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
                        ['f', s @ ..] => {
                            out.push(&[(0, 0.59565115), (12, 0.4043183)], 0.9999695);
                            match s {
                                [] => true,
                                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
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
                        ['t', 's', '\u{1}', s @ ..] => {
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
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'c', s @ ..] => {
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
                                ['e', 'v', s @ ..] => {
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
                ['v', 'a', 'n', s @ ..] => super::t6::n7241(s, out),
                _ => false,
            }
        }
        ['l', s @ ..] => super::t6::n3294(s, out),
        ['o', 'r', s @ ..] => super::t6::n3295(s, out),
        ['r', s @ ..] => super::t6::n3296(s, out),
        ['u', s @ ..] => super::t6::n3297(s, out),
        _ => false,
    }
}

/// Words ending -iatus.
pub(crate) fn n1264(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99990845)], 0.99990845);
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
                ['n', 'u', 'n', 'e', s @ ..] => {
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
                ['s', 'a', s @ ..] => {
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
                                ['e', 'u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
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
                                        ['r', 't', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['o', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['b', 'l', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['r', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['b', 'u', 'r', '\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                ['g', 'i', 'n', '\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        _ => false,
                                    }
                                }
                                ['x', 'e', 's', '\u{1}', s @ ..] => {
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
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['e', 'm', s @ ..] => {
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
                ['o', s @ ..] => {
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
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
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
                ['e', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                                ['n', 'o', 'c', s @ ..] => {
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
                        ['m', 'u', 'h', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'u', 'a', s @ ..] => {
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
                ['l', 'a', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'r', 't', '\u{1}', s @ ..] => {
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
        ['n', 'a', 'l', s @ ..] => super::t6::n3304(s, out),
        ['p', s @ ..] => super::t6::n3305(s, out),
        ['r', s @ ..] => super::t6::n3306(s, out),
        ['t', s @ ..] => super::t6::n3307(s, out),
        ['v', s @ ..] => super::t6::n3308(s, out),
        _ => false,
    }
}

/// Words ending -latus.
pub(crate) fn n1265(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9850004), (15, 0.011505303), (7, 0.003479057)], 0.99998474);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
            s.is_empty()
        }
        ['a', s @ ..] => {
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
                ['p', s @ ..] => {
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
        ['b', s @ ..] => {
            out.push(&[(0, 0.8430915), (15, 0.15657282)], 0.9996643);
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', '\u{1}', s @ ..] => {
                    out.push(&[(0, 0.70110625), (15, 0.29889372)], 1.0);
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
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['a', s @ ..] => {
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
                        ['u', 's', s @ ..] => {
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
        ['l', s @ ..] => super::t6::n3317(s, out),
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
        ['o', s @ ..] => super::t6::n3320(s, out),
        ['p', 'm', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => super::t6::n3321(s, out),
        ['r', 'e', s @ ..] => super::t6::n3322(s, out),
        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => super::t6::n3323(s, out),
        ['u', s @ ..] => super::t6::n3324(s, out),
        _ => false,
    }
}

/// Words ending -matus.
pub(crate) fn n1266(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9783932), (15, 0.01077287), (12, 0.01077287)], 0.99993896);
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
                ['h', s @ ..] => {
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
        ['e', 'r', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
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
                                ['n', 'i', '\u{1}', s @ ..] => {
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
                ['e', 'g', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 's', 'n', 'o', 'c', s @ ..] => {
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
        ['r', s @ ..] => {
            out.push(&[(0, 0.8817121), (15, 0.059143968), (12, 0.059143968)], 1.0);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(0, 0.6973831), (12, 0.29416344), (15, 0.008453499)], 1.0);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', '\u{1}', s @ ..] => {
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
                        ['f', s @ ..] => super::t6::n12680(s, out),
                        ['n', s @ ..] => super::t6::n12681(s, out),
                        _ => false,
                    }
                }
                ['o', s @ ..] => super::t6::n7367(s, out),
                _ => false,
            }
        }
        ['s', 'a', 'l', 'p', s @ ..] => super::t6::n3333(s, out),
        ['u', s @ ..] => super::t6::n3334(s, out),
        _ => false,
    }
}

/// Words ending -natus.
pub(crate) fn n1267(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.95996034), (7, 0.014373999), (12, 0.0116731515), (15, 0.006836042), (136, 0.0035858701), (336, 0.0035858701)], 1.0000153);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', s @ ..] => {
                            match s {
                                [] => true,
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
                        _ => false,
                    }
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
                                ['r', '\u{1}', s @ ..] => {
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['g', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', '\u{1}', s @ ..] => {
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
            out.push(&[(0, 0.95342946), (12, 0.045929655)], 0.99935913);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'r', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
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
                        ['a', s @ ..] => {
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
                        ['c', s @ ..] => {
                            out.push(&[(0, 0.590692), (12, 0.40918595)], 0.9998779);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['b', 'u', 's', '\u{1}', s @ ..] => {
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
            out.push(&[(0, 0.9994049)], 0.9994049);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', '\u{1}', s @ ..] => {
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
                                ['\u{1}', s @ ..] => super::t6::n18084(s, out),
                                ['a', 'm', '\u{1}', s @ ..] => super::t6::n18085(s, out),
                                _ => false,
                            }
                        }
                        ['s', s @ ..] => super::t6::n12711(s, out),
                        ['t', 'n', 'o', 'c', '\u{1}', s @ ..] => super::t6::n12712(s, out),
                        _ => false,
                    }
                }
                ['o', s @ ..] => super::t6::n7393(s, out),
                ['u', 'p', s @ ..] => super::t6::n7394(s, out),
                _ => false,
            }
        }
        ['i', s @ ..] => super::t6::n3339(s, out),
        ['m', s @ ..] => super::t6::n3340(s, out),
        ['n', s @ ..] => super::t6::n3341(s, out),
        ['o', s @ ..] => super::t6::n3342(s, out),
        ['r', s @ ..] => super::t6::n3343(s, out),
        ['u', s @ ..] => super::t6::n3344(s, out),
        _ => false,
    }
}

/// Words ending -patus.
pub(crate) fn n1268(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.96583503), (12, 0.033829253)], 0.9996643);
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
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['a', 'p', s @ ..] => {
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
                ['u', 'c', s @ ..] => {
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
        ['u', 'c', s @ ..] => {
            out.push(&[(0, 0.81507593), (12, 0.18489356)], 0.9999695);
            match s {
                [] => true,
                ['c', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
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
        _ => false,
    }
}

/// Words ending -ratus.
pub(crate) fn n1269(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9536431), (7, 0.029129473), (15, 0.009750515), (12, 0.007461662)], 0.99998474);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', s @ ..] => {
            out.push(&[(0, 0.8825971), (7, 0.078233), (15, 0.03901732)], 0.9998474);
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
                        ['a', 'c', s @ ..] => {
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
                    out.push(&[(0, 0.70869), (7, 0.19420157), (15, 0.09709316)], 0.99998474);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            out.push(&[(0, 0.74540323), (15, 0.23300527), (7, 0.021576257)], 0.99998474);
                            match s {
                                [] => true,
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', s @ ..] => {
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
                                ['s', s @ ..] => {
                                    out.push(&[(15, 0.7443351), (0, 0.24846265), (7, 0.0071869995)], 0.99998474);
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
                        ['i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
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
                        ['p', s @ ..] => {
                            out.push(&[(7, 0.7313954), (0, 0.23622492), (15, 0.032364387)], 0.99998474);
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
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
                ['e', 'l', s @ ..] => {
                    out.push(&[(0, 0.7726253), (7, 0.22545205)], 0.9980774);
                    match s {
                        [] => true,
                        ['e', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
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
        ['e', s @ ..] => super::t6::n3354(s, out),
        ['g', s @ ..] => {
            out.push(&[(0, 0.8975662), (7, 0.051377125), (12, 0.0508278)], 0.9997711);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'f', s @ ..] => super::t6::n12866(s, out),
                        ['r', 'f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 't', 'n', 'i', s @ ..] => super::t6::n7458(s, out),
                ['i', s @ ..] => super::t6::n7459(s, out),
                ['n', s @ ..] => {
                    out.push(&[(12, 0.6836042), (0, 0.29918364), (7, 0.017120622)], 0.99990845);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => super::t6::n12873(s, out),
                        _ => false,
                    }
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => super::t6::n7461(s, out),
                _ => false,
            }
        }
        ['i', s @ ..] => super::t6::n3356(s, out),
        ['o', s @ ..] => super::t6::n3357(s, out),
        ['r', s @ ..] => super::t6::n3358(s, out),
        ['t', s @ ..] => super::t6::n3359(s, out),
        ['u', s @ ..] => super::t6::n3360(s, out),
        ['y', 'g', s @ ..] => super::t6::n3361(s, out),
        _ => false,
    }
}

/// Words ending -satus.
pub(crate) fn n1270(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99980164)], 0.99980164);
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
        ['o', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
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
                ['e', 'c', 'a', 'l', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'c', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['a', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['t', 'b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -tatus.
pub(crate) fn n1271(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9772335), (7, 0.011383231), (15, 0.009094377), (12, 0.0022735943)], 0.99998474);
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
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
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
            out.push(&[(0, 0.9743038), (7, 0.012848097), (12, 0.012787061)], 0.99993896);
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
                                ['n', 'i', '\u{1}', s @ ..] => {
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
                            out.push(&[(0, 0.9156023), (12, 0.083863586)], 0.99946594);
                            match s {
                                [] => true,
                                ['s', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(12, 1.0)], 1.0);
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
                                                ['d', 'a', '\u{1}', s @ ..] => super::t6::n23815(s, out),
                                                _ => false,
                                            }
                                        }
                                        ['o', 'r', 'p', '\u{1}', s @ ..] => super::t6::n22200(s, out),
                                        ['r', 'e', 'p', '\u{1}', s @ ..] => super::t6::n22201(s, out),
                                        ['x', 'e', s @ ..] => super::t6::n22202(s, out),
                                        _ => false,
                                    }
                                }
                                ['x', 'e', s @ ..] => super::t6::n18401(s, out),
                                _ => false,
                            }
                        }
                        ['s', s @ ..] => super::t6::n13001(s, out),
                        ['v', s @ ..] => super::t6::n13002(s, out),
                        _ => false,
                    }
                }
                ['i', 'd', s @ ..] => super::t6::n7524(s, out),
                ['n', s @ ..] => super::t6::n7525(s, out),
                ['r', 'a', s @ ..] => super::t6::n7526(s, out),
                ['u', s @ ..] => super::t6::n7527(s, out),
                _ => false,
            }
        }
        ['e', s @ ..] => super::t6::n3372(s, out),
        ['i', s @ ..] => super::t6::n3373(s, out),
        ['l', 'a', 'x', 'e', s @ ..] => super::t6::n3374(s, out),
        ['n', s @ ..] => super::t6::n3375(s, out),
        ['o', s @ ..] => super::t6::n3376(s, out),
        ['p', s @ ..] => super::t6::n3377(s, out),
        ['r', s @ ..] => super::t6::n3378(s, out),
        ['s', s @ ..] => super::t6::n3379(s, out),
        ['t', s @ ..] => super::t6::n3380(s, out),
        ['u', s @ ..] => super::t6::n3381(s, out),
        _ => false,
    }
}

/// Words ending -uatus.
pub(crate) fn n1272(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9997711)], 0.9997711);
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
                ['r', 'o', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
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

/// Words ending -vatus.
pub(crate) fn n1273(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.98088044), (15, 0.018951705)], 0.99983215);
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
                ['l', s @ ..] => {
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
                                ['b', 'u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['r', s @ ..] => {
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
                ['v', s @ ..] => {
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
        ['o', s @ ..] => {
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
                        ['n', s @ ..] => {
                            out.push(&[(0, 0.59617), (15, 0.40378425)], 0.9999542);
                            match s {
                                [] => true,
                                ['e', '\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word relaxatus.
pub(crate) fn n7603(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -exatus.
pub(crate) fn n3393(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -efactus.
pub(crate) fn n7612(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
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
                ['u', 'p', '\u{1}', s @ ..] => {
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
                ['i', 'c', '\u{1}', s @ ..] => {
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
                ['q', 'i', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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

/// Words ending -pactus.
pub(crate) fn n3403(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8650492), (12, 0.13310444)], 0.9981536);
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
                    out.push(&[(12, 0.5263142), (0, 0.47368583)], 1.0);
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

/// Words ending -ractus.
pub(crate) fn n3404(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['f', s @ ..] => {
            out.push(&[(0, 0.9137255), (7, 0.08493172)], 0.9986572);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
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
                    out.push(&[(0, 0.72976273), (15, 0.26746014)], 0.9972229);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(0, 0.5263142), (15, 0.47368583)], 1.0);
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

/// The word tactus.
pub(crate) fn n7627(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word adtactus.
pub(crate) fn n7628(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ntactus.
pub(crate) fn n7629(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word attactus.
pub(crate) fn n7630(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -xactus.
pub(crate) fn n3407(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(15, 0.67292285), (0, 0.3207904), (12, 0.0032959487), (7, 0.0030060273)], 1.0000153);
    match s {
        [] => true,
        ['e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ectus.
pub(crate) fn n1276(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', s @ ..] => {
            out.push(&[(0, 0.8640879), (7, 0.08712901), (15, 0.048447393)], 0.9996643);
            match s {
                [] => true,
                ['e', 'd', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
                            s.is_empty()
                        }
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'e', 'p', s @ ..] => {
                    out.push(&[(0, 0.57679105), (15, 0.22026399), (7, 0.20291448)], 0.9999695);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'i', s @ ..] => {
                            out.push(&[(15, 0.74009305), (0, 0.19226368), (7, 0.06764325)], 1.0);
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
                    out.push(&[(0, 0.8085756), (7, 0.18825056)], 0.9968261);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            out.push(&[(7, 0.63427174), (0, 0.36479744)], 0.9990692);
                            s.is_empty()
                        }
                        ['u', 's', s @ ..] => {
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
                _ => false,
            }
        }
        ['l', s @ ..] => {
            out.push(&[(0, 0.91993594), (7, 0.040024415), (12, 0.03959716)], 0.9995575);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
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
                ['i', s @ ..] => {
                    out.push(&[(0, 0.5839933), (12, 0.40791944), (7, 0.008010986)], 0.99992377);
                    match s {
                        [] => true,
                        ['d', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
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
        ['n', 'e', s @ ..] => {
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
        ['p', s @ ..] => {
            out.push(&[(0, 0.88542), (15, 0.05691615), (12, 0.056656748)], 0.9989929);
            match s {
                [] => true,
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
                            out.push(&[(0, 0.69791716), (12, 0.29381245), (15, 0.008133058)], 0.9998627);
                            match s {
                                [] => true,
                                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', s @ ..] => {
                                    out.push(&[(12, 0.7645991), (0, 0.23263904), (15, 0.002716106)], 0.9999542);
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
                        ['u', s @ ..] => super::t6::n13276(s, out),
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['r', s @ ..] => super::t6::n3414(s, out),
        ['s', s @ ..] => super::t6::n3415(s, out),
        ['t', s @ ..] => super::t6::n3416(s, out),
        ['v', s @ ..] => super::t6::n3417(s, out),
        _ => false,
    }
}

/// Words ending -ictus.
pub(crate) fn n1277(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.957763), (12, 0.02888533), (7, 0.013199054)], 0.9998474);
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
                ['d', s @ ..] => {
                    out.push(&[(12, 0.6762951), (0, 0.31924924), (7, 0.0043945983)], 0.99993896);
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
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
                ['o', 'r', 'p', s @ ..] => {
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
                                ['b', 'a', '\u{1}', s @ ..] => {
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
                ['b', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'b', 'l', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            out.push(&[(0, 0.94517434), (7, 0.05397116)], 0.9991455);
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
                        ['f', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'v', 'e', 'r', 'b', '\u{1}', s @ ..] => {
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
                                ['r', s @ ..] => {
                                    out.push(&[(0, 0.6135805), (7, 0.38623637)], 0.9998169);
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
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
                ['n', s @ ..] => {
                    out.push(&[(0, 0.5915465), (12, 0.40578318), (7, 0.0026398108)], 0.9999695);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => super::t6::n13342(s, out),
                        ['o', 'c', '\u{1}', s @ ..] => super::t6::n13343(s, out),
                        _ => false,
                    }
                }
                ['o', 'r', 'p', '\u{1}', s @ ..] => super::t6::n7714(s, out),
                ['r', 'e', 'p', s @ ..] => super::t6::n7715(s, out),
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -nctus.
pub(crate) fn n1278(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9030594), (7, 0.078645), (15, 0.018188754)], 0.9998932);
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
                ['n', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    out.push(&[(0, 0.5806058), (7, 0.41573206), (15, 0.0036316472)], 0.9999695);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(0, 0.96534675), (7, 0.03436332)], 0.9997101);
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
                        ['i', 'b', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
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
                                ['i', s @ ..] => {
                                    out.push(&[(0, 0.5930724), (7, 0.40686655)], 0.99993896);
                                    match s {
                                        [] => true,
                                        ['d', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['\u{1}', s @ ..] => {
                                                    out.push(&[(7, 1.0)], 1.0);
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
                                ['n', 'i', '\u{1}', s @ ..] => {
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
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['c', '\u{1}', s @ ..] => {
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
                    out.push(&[(0, 0.80360115), (7, 0.12523079), (15, 0.07116808)], 1.0);
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
                        ['d', s @ ..] => {
                            out.push(&[(0, 0.68903637), (15, 0.29146257), (7, 0.01950103)], 1.0);
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    out.push(&[(0, 0.66790265), (15, 0.33209735)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', s @ ..] => {
                                    out.push(&[(7, 0.7084153), (0, 0.26787212), (15, 0.023727778)], 1.0000153);
                                    match s {
                                        [] => true,
                                        ['c', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(7, 0.5263142), (0, 0.47368583)], 1.0);
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
                        ['i', 'b', 'l', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -octus.
pub(crate) fn n1279(s: &[char], out: &mut Path<'_>) -> bool {
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
                    out.push(&[(7, 1.0)], 1.0);
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
                ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -uctus.
pub(crate) fn n1280(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9310597), (15, 0.04568551), (7, 0.023102159)], 0.9998474);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.58620584), (15, 0.40914014), (7, 0.004623484)], 0.9999695);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
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
                        ['d', s @ ..] => {
                            out.push(&[(15, 0.6818952), (0, 0.31035325), (7, 0.0077058063)], 0.9999542);
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
                ['l', 'o', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            out.push(&[(0, 0.92477304), (7, 0.07356375)], 0.9983368);
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
                                    out.push(&[(0, 0.5849546), (7, 0.4147097)], 0.9996643);
                                    match s {
                                        [] => true,
                                        ['i', '\u{1}', s @ ..] => {
                                            out.push(&[(7, 1.0)], 1.0);
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

/// The word sollicitus.
pub(crate) fn n23836(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -escitus.
pub(crate) fn n7821(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'r', 'p', '\u{1}', s @ ..] => {
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

/// Words ending -nscitus.
pub(crate) fn n7823(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ditus.
pub(crate) fn n1292(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.87922484), (7, 0.096513316), (15, 0.02417029)], 0.99990845);
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
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    out.push(&[(0, 0.5379263), (7, 0.40965897), (15, 0.05241474)], 1.0);
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
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
                                    out.push(&[(15, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['x', 'e', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
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
                ['r', 'c', s @ ..] => {
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
        ['n', s @ ..] => {
            out.push(&[(0, 0.90818644), (15, 0.08763256), (7, 0.004180972)], 0.99999994);
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
                ['o', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            out.push(&[(15, 0.6958724), (0, 0.30272374)], 0.9985962);
                            match s {
                                [] => true,
                                ['r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'b', 'a', '\u{1}', s @ ..] => {
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
                ['t', 's', 'u', 'c', s @ ..] => {
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
        ['r', 'e', 'p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            out.push(&[(0, 0.57584494), (7, 0.41930267), (15, 0.00483711)], 0.99998474);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
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
        _ => false,
    }
}

/// Words ending -gitus.
pub(crate) fn n1293(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -litus.
pub(crate) fn n1294(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
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
        ['c', 'n', 'i', '\u{1}', s @ ..] => {
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
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', '\u{1}', s @ ..] => {
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
            out.push(&[(0, 0.8517128), (7, 0.07428092), (15, 0.07368582)], 0.99967957);
            match s {
                [] => true,
                ['b', 'a', '\u{1}', s @ ..] => {
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
                    out.push(&[(0, 0.56333256), (7, 0.21834135), (15, 0.21828032)], 0.9999542);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', s @ ..] => {
                            out.push(&[(15, 0.73942167), (0, 0.18777752), (7, 0.072785534)], 0.99998474);
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

/// Words ending -omitus.
pub(crate) fn n1295(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -nitus.
pub(crate) fn n1296(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9488365), (7, 0.017151142), (12, 0.016983291), (264, 0.016937515)], 0.99990845);
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
            out.push(&[(0, 0.92185855), (7, 0.07684443)], 0.998703);
            match s {
                [] => true,
                ['g', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['l', 'o', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'u', '\u{1}', s @ ..] => {
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
                ['p', s @ ..] => {
                    out.push(&[(7, 0.60198367), (0, 0.39763486)], 0.99961853);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            out.push(&[(0, 0.87180895), (12, 0.12605478)], 0.9978637);
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
                            out.push(&[(12, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['c', 'a', '\u{1}', s @ ..] => {
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
                                _ => false,
                            }
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
                ['r', 'c', '\u{1}', s @ ..] => {
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
        ['u', s @ ..] => {
            out.push(&[(0, 0.92245364), (264, 0.07624933)], 0.998703);
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
                            out.push(&[(264, 1.0)], 1.0);
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
                        ['n', 'i', '\u{1}', s @ ..] => {
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
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -pitus.
pub(crate) fn n1297(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ritus.
pub(crate) fn n1298(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9320211), (7, 0.0509041), (15, 0.016983291)], 0.99990845);
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
                                _ => false,
                            }
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    out.push(&[(0, 0.59320974), (7, 0.4050965)], 0.9983063);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'o', '\u{1}', s @ ..] => {
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
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['e', 't', s @ ..] => {
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
                ['i', s @ ..] => {
                    out.push(&[(7, 0.6836347), (0, 0.31067368), (15, 0.0056610974)], 0.9999695);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(0, 0.93012893), (15, 0.068146795)], 0.99827576);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'p', '\u{1}', s @ ..] => {
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
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    out.push(&[(15, 0.6893873), (0, 0.31004807)], 0.9994354);
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
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

/// Words ending -situs.
pub(crate) fn n1299(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['i', s @ ..] => {
            out.push(&[(0, 0.8678416), (12, 0.12945755)], 0.99729913);
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
                        ['x', s @ ..] => {
                            out.push(&[(12, 0.7098192), (0, 0.28928053)], 0.99909973);
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
                    out.push(&[(0, 0.8780804), (15, 0.11973754)], 0.997818);
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
                        ['o', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
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
                                _ => false,
                            }
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
                        ['a', s @ ..] => {
                            out.push(&[(0, 0.728191), (7, 0.26880294)], 0.99699396);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
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
                                ['n', 'i', '\u{1}', s @ ..] => {
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
                ['e', 'c', 'a', 'l', s @ ..] => {
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
                ['i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -titus.
pub(crate) fn n1300(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9994812)], 0.9994812);
    match s {
        [] => true,
        ['e', 'p', s @ ..] => {
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
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', '\u{1}', s @ ..] => {
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
                ['e', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', '\u{1}', s @ ..] => {
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
                                _ => false,
                            }
                        }
                        ['m', 'i', '\u{1}', s @ ..] => {
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
                ['e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
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

/// Words ending -uitus.
pub(crate) fn n1301(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['r', 'f', s @ ..] => {
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
                ['a', 'r', 'g', '\u{1}', s @ ..] => {
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
                ['r', 'o', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -vitus.
pub(crate) fn n1302(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.863447), (12, 0.13377585)], 0.99722284);
    match s {
        [] => true,
        ['a', s @ ..] => {
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
        ['n', s @ ..] => {
            out.push(&[(12, 0.7112535), (0, 0.28781566)], 0.9990692);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -mptus.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.89575034), (7, 0.06797894), (15, 0.036026552)], 0.99975586);
    match s {
        [] => true,
        ['e', s @ ..] => {
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
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['t', s @ ..] => {
                    out.push(&[(7, 0.6893263), (0, 0.29858854), (15, 0.01200885)], 0.9999237);
                    match s {
                        [] => true,
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
        ['o', s @ ..] => {
            out.push(&[(0, 0.7170825), (15, 0.14753948), (7, 0.13536279)], 0.99998474);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            out.push(&[(15, 0.7158465), (0, 0.23903258), (7, 0.04512093)], 1.0);
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
                ['r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
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
            out.push(&[(0, 0.99514765), (7, 0.0031586175)], 0.9983063);
            match s {
                [] => true,
                ['s', s @ ..] => {
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
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'r', 't', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
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
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
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
        _ => false,
    }
}

/// Words ending -sorptus.
pub(crate) fn n1320(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -uptus.
pub(crate) fn n1321(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.83292896), (15, 0.08552682), (12, 0.08096437)], 0.99942017);
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
                ['b', s @ ..] => {
                    out.push(&[(0, 0.56658274), (12, 0.41618982), (15, 0.017105363)], 0.9998779);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
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
                ['e', s @ ..] => {
                    out.push(&[(0, 0.54920274), (15, 0.23698787), (12, 0.21374838)], 0.99993896);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'r', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(0, 0.5263142), (12, 0.47368583)], 1.0);
                            s.is_empty()
                        }
                        ['d', s @ ..] => {
                            out.push(&[(15, 0.7456626), (0, 0.18306248), (12, 0.071244374)], 0.9999695);
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
                        ['o', s @ ..] => {
                            out.push(&[(0, 0.56658274), (15, 0.41710538), (12, 0.016189823)], 0.9998779);
                            match s {
                                [] => true,
                                ['c', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['n', 'i', '\u{1}', s @ ..] => {
                                            out.push(&[(15, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
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
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ortus.
pub(crate) fn n1324(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.93499655), (12, 0.062317844), (7, 0.0022430762)], 0.9995575);
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
        ['o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'b', 'a', '\u{1}', s @ ..] => {
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
                ['s', s @ ..] => {
                    out.push(&[(12, 0.6874342), (0, 0.31166553)], 0.99909973);
                    match s {
                        [] => true,
                        ['i', 'd', '\u{1}', s @ ..] => {
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
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ustus.
pub(crate) fn n1330(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', s @ ..] => {
            out.push(&[(0, 0.9912108), (7, 0.008255131)], 0.99946594);
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
                ['h', s @ ..] => {
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
                _ => false,
            }
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
                        ['o', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            out.push(&[(7, 0.72997636), (0, 0.2659037), (15, 0.004104677)], 0.99998474);
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
                ['n', s @ ..] => {
                    out.push(&[(7, 0.72997636), (0, 0.2659037), (15, 0.004104677)], 0.99998474);
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
        ['g', s @ ..] => {
            out.push(&[(7, 0.5586633), (0, 0.4396887)], 0.99835205);
            match s {
                [] => true,
                ['n', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(15, 0.38474098), (7, 0.34218356), (0, 0.27307546)], 1.0);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 0.81632715), (15, 0.18367285)], 1.0);
                    s.is_empty()
                }
                ['m', 'e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
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
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['e', 'v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -solutus.
pub(crate) fn n8224(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.6476692), (12, 0.22961776), (378, 0.12085146)], 0.9981384);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(378, 1.0)], 1.0);
            s.is_empty()
        }
        ['b', 'a', s @ ..] => {
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
        ['e', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'i', 'd', '\u{1}', s @ ..] => {
            out.push(&[(0, 0.5263142), (12, 0.47368583)], 1.0);
            s.is_empty()
        }
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -volutus.
pub(crate) fn n8225(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99481195), (12, 0.0025024796)], 0.99731445);
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nutus.
pub(crate) fn n1338(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.91480887), (259, 0.08099489), (7, 0.002121004)], 0.99792475);
    match s {
        [] => true,
        ['i', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(0, 0.5263142), (259, 0.47368583)], 1.0);
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
        ['r', 'o', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -putus.
pub(crate) fn n1339(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['x', 'e', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rutus.
pub(crate) fn n1340(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99636835)], 0.99636835);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
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
                        _ => false,
                    }
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
                ['\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['d', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'e', 's', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -sutus.
pub(crate) fn n1341(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.809308), (7, 0.0952163), (15, 0.09396505)], 0.9984893);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['a', 'n', '\u{1}', s @ ..] => {
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
        ['r', s @ ..] => {
            out.push(&[(7, 0.41904327), (15, 0.4187991), (0, 0.16186771)], 0.9997101);
            match s {
                [] => true,
                ['e', 'v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'h', '\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
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

/// Words ending -tutus.
pub(crate) fn n1342(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.82816815), (7, 0.057663843), (15, 0.056900892), (73, 0.056534678)], 0.9992676);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(7, 1.0)], 1.0);
            s.is_empty()
        }
        ['a', 't', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'm', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(0, 0.9885405), (7, 0.0038452735), (15, 0.0037994965), (73, 0.0037689784)], 0.9999542);
            match s {
                [] => true,
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
        ['n', s @ ..] => {
            out.push(&[(15, 0.68563366), (0, 0.27605096), (7, 0.019226369), (73, 0.018844891)], 0.99975586);
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            out.push(&[(73, 0.6855115), (0, 0.27605096), (7, 0.019226369), (15, 0.018966964)], 0.9997558);
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -cabilis.
pub(crate) fn n4007(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.6127413), (9, 0.30637065), (31, 0.027252613), (29, 0.02650492), (32, 0.013626306), (30, 0.01326009)], 0.99975586);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(8, 0.38921186), (31, 0.27211413), (9, 0.19461356), (32, 0.1360647), (29, 0.005294881), (30, 0.0026550698)], 0.9999542);
            match s {
                [] => true,
                ['l', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
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
        ['i', s @ ..] => {
            out.push(&[(8, 0.66514075), (9, 0.33257037)], 0.9977111);
            match s {
                [] => true,
                ['d', 'e', 'm', s @ ..] => {
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
                ['l', 'p', 'x', 'e', s @ ..] => {
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
                ['n', 'u', 'm', 'm', 'o', 'c', s @ ..] => {
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
        ['o', s @ ..] => {
            out.push(&[(8, 0.4816968), (9, 0.2408484), (29, 0.1813077), (30, 0.09065385), (31, 0.0036316472)], 0.9981384);
            match s {
                [] => true,
                ['v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(29, 0.6666667), (30, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'r', s @ ..] => {
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

/// Words ending -dabilis.
pub(crate) fn n4008(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['o', 'n', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['u', 'a', 'l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', 'o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -meabilis.
pub(crate) fn n4009(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'r', 'r', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -effabilis.
pub(crate) fn n4010(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -igabilis.
pub(crate) fn n4011(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['t', 's', 'e', 'v', 'n', 'i', s @ ..] => {
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
        ['v', 'a', 'n', s @ ..] => {
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

/// Words ending -habilis.
pub(crate) fn n4012(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.38866255), (31, 0.27530327), (9, 0.1943389), (32, 0.13765164)], 0.99595636);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -iabilis.
pub(crate) fn n4013(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.6649424), (9, 0.33247882)], 0.9974212);
    match s {
        [] => true,
        ['c', 'o', 's', s @ ..] => {
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
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', 'a', 's', s @ ..] => {
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

/// Words ending -labilis.
pub(crate) fn n4014(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 'g', 'n', 'o', 'c', s @ ..] => {
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
        ['o', 's', 'n', 'o', 'c', s @ ..] => {
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
        ['p', 'm', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', 'c', 'a', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -mabilis.
pub(crate) fn n4015(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(8, 0.38866255), (31, 0.27530327), (9, 0.1943389), (32, 0.13765164)], 0.99595636);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 't', 's', 'e', 'a', s @ ..] => {
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

/// Words ending -nabilis.
pub(crate) fn n4016(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.6653544), (9, 0.3326772)], 0.99803156);
    match s {
        [] => true,
        ['a', 's', s @ ..] => {
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
        ['g', 'u', 'p', 'x', 'e', s @ ..] => {
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
        ['i', 'm', 'o', 'n', s @ ..] => {
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
        ['o', 'i', 't', 'a', 'r', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -pabilis.
pub(crate) fn n4017(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.45282674), (9, 0.22641337), (137, 0.20553902), (138, 0.102571145), (31, 0.0066376748), (32, 0.0033264668)], 0.9973144);
    match s {
        [] => true,
        ['l', 'u', 'c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(137, 0.6666667), (138, 0.33333334)], 1.0);
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

/// Words ending -rabilis.
pub(crate) fn n4018(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.57912564), (9, 0.28957045), (31, 0.076539256), (32, 0.038269628), (160, 0.010910201), (161, 0.005447471)], 0.9998627);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(8, 0.66193634), (9, 0.33096817), (31, 0.004135195), (32, 0.002075227)], 0.99911493);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
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
                        ['m', 'o', 'c', s @ ..] => {
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
                    out.push(&[(31, 0.5486381), (32, 0.27432668), (8, 0.11583123), (9, 0.057907987), (160, 0.0021820401)], 0.99888605);
                    match s {
                        [] => true,
                        ['o', 't', s @ ..] => {
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
                ['m', 'u', 'n', s @ ..] => {
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
                ['n', 'l', 'u', 'v', s @ ..] => {
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
                ['p', 'u', 's', s @ ..] => {
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
                ['s', 'i', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
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
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', s @ ..] => {
                            out.push(&[(31, 0.46996263), (32, 0.23497368), (8, 0.19304189), (9, 0.096528575), (160, 0.0036316472)], 0.9981384);
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(8, 0.45705348), (9, 0.22851911), (31, 0.14067292), (32, 0.07034409), (160, 0.06894026), (161, 0.03447013)], 1.0);
            match s {
                [] => true,
                ['m', 'e', 'm', s @ ..] => {
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
                ['n', 'o', s @ ..] => {
                    out.push(&[(31, 0.49134052), (32, 0.24567026), (8, 0.15234607), (9, 0.07617304), (160, 0.022980087), (161, 0.011490043)], 1.0);
                    s.is_empty()
                }
                ['v', s @ ..] => {
                    out.push(&[(160, 0.4674296), (161, 0.23370717), (8, 0.15234607), (9, 0.07617304), (31, 0.046890974), (32, 0.023453116)], 1.0);
                    s.is_empty()
                }
                ['x', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
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
        ['r', 'a', 'n', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['t', 'e', 'n', 'e', 'p', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['m', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', s @ ..] => {
            out.push(&[(8, 0.6616617), (9, 0.33083084), (31, 0.0043793395), (32, 0.0021820401)], 0.9990539);
            match s {
                [] => true,
                ['s', 'n', 'e', 'm', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['t', 'a', 's', s @ ..] => {
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

/// Words ending -usabilis.
pub(crate) fn n4019(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', s @ ..] => {
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
        ['c', s @ ..] => {
            out.push(&[(8, 0.46810102), (9, 0.23404288), (31, 0.19664301), (32, 0.098313875)], 0.9971008);
            match s {
                [] => true,
                ['x', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
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

/// Words ending -tabilis.
pub(crate) fn n4020(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.5973297), (9, 0.29866484), (31, 0.046265356), (32, 0.023132678), (29, 0.011520562), (168, 0.011490043), (30, 0.0057679103), (169, 0.005737392)], 0.99990845);
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['a', 'r', s @ ..] => {
                    out.push(&[(8, 0.38612956), (31, 0.27591363), (9, 0.19307241), (32, 0.13795681), (29, 0.0023041123), (168, 0.0023041123)], 0.9976806);
                    match s {
                        [] => true,
                        ['t', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
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
                ['e', 'p', 's', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['u', 'l', 'e', s @ ..] => {
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
        ['e', 'r', 'p', 'r', 'e', 't', 'n', 'i', s @ ..] => {
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
            out.push(&[(8, 0.6644236), (9, 0.33221942)], 0.996643);
            match s {
                [] => true,
                ['b', 'a', 'h', s @ ..] => {
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
                ['g', 'o', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', 'n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'q', 'e', s @ ..] => {
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
                ['v', 'e', s @ ..] => {
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
        ['n', s @ ..] => {
            out.push(&[(8, 0.51081103), (9, 0.25540552), (168, 0.1494316), (169, 0.07470817), (31, 0.0051422906), (32, 0.0025635157)], 0.99806213);
            match s {
                [] => true,
                ['e', 'm', 'a', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            out.push(&[(168, 0.49425498), (169, 0.24711986), (8, 0.17027542), (9, 0.08513008)], 0.99678034);
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
                _ => false,
            }
        }
        ['p', 'o', s @ ..] => {
            out.push(&[(8, 0.38612956), (31, 0.27591363), (9, 0.19307241), (32, 0.13795681), (29, 0.0023041123), (168, 0.0023041123)], 0.9976806);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
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
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            out.push(&[(8, 0.4272984), (9, 0.21364157), (31, 0.15957884), (32, 0.07978942), (29, 0.07910277), (30, 0.039551385)], 0.9989624);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(29, 0.6666667), (30, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            out.push(&[(31, 0.4976425), (32, 0.24881361), (8, 0.14242771), (9, 0.07121386), (29, 0.02636759), (30, 0.013183795)], 0.99964905);
                            match s {
                                [] => true,
                                ['r', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['g', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'e', s @ ..] => {
                            out.push(&[(31, 0.4976425), (32, 0.24881361), (8, 0.14242771), (9, 0.07121386), (29, 0.02636759), (30, 0.013183795)], 0.99964905);
                            match s {
                                [] => true,
                                ['d', '\u{1}', s @ ..] => {
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
                ['u', 'g', s @ ..] => {
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
        ['u', 'm', s @ ..] => {
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
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -aequabilis.
pub(crate) fn n4021(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -servabilis.
pub(crate) fn n4022(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
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
        ['n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ebilis.
pub(crate) fn n1596(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.55994505), (9, 0.27997252), (29, 0.10330358), (30, 0.05165179), (31, 0.002838178)], 0.9977111);
    match s {
        [] => true,
        ['d', s @ ..] => {
            out.push(&[(29, 0.47887388), (30, 0.23944457), (8, 0.18664835), (9, 0.09332418)], 0.99829096);
            s.is_empty()
        }
        ['l', 'e', 'd', s @ ..] => {
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

/// Words ending -ibilis.
pub(crate) fn n1597(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.6431983), (9, 0.32159916), (31, 0.014465553), (29, 0.008972305), (32, 0.0072327764), (30, 0.0044861524)], 0.9999542);
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['e', s @ ..] => {
                    out.push(&[(8, 0.4287938), (29, 0.22819868), (9, 0.21440452), (30, 0.11410697), (31, 0.009643702), (32, 0.004821851)], 0.9999695);
                    match s {
                        [] => true,
                        ['d', '\u{1}', s @ ..] => {
                            out.push(&[(29, 0.6666667), (30, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'd', 'a', 'r', 't', 'n', 'o', 'c', s @ ..] => {
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
                ['n', 'i', 'v', s @ ..] => {
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
                        ['i', 'm', 'm', 'o', 'c', s @ ..] => {
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
                        ['o', 'n', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                ['u', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'o', 'p', '\u{1}', s @ ..] => {
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
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
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
                                _ => false,
                            }
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
                        ['x', 'e', '\u{1}', s @ ..] => {
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
                ['e', 'r', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['i', 'l', 'l', 'e', 't', 'n', 'i', s @ ..] => {
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
        ['r', s @ ..] => {
            out.push(&[(8, 0.39765012), (9, 0.19882506), (31, 0.13478294), (29, 0.13423361), (32, 0.06738384), (30, 0.067109175)], 0.99998474);
            match s {
                [] => true,
                ['r', s @ ..] => {
                    out.push(&[(31, 0.29362935), (29, 0.29350728), (32, 0.14680704), (30, 0.14676127), (8, 0.07953002), (9, 0.03976501)], 1.0);
                    match s {
                        [] => true,
                        ['o', 'h', '\u{1}', s @ ..] => {
                            out.push(&[(29, 0.6666667), (30, 0.33333334)], 1.0);
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
                ['i', s @ ..] => {
                    out.push(&[(8, 0.49706262), (9, 0.24853896), (31, 0.16847487), (32, 0.084229805)], 0.9983063);
                    match s {
                        [] => true,
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', s @ ..] => {
                                    out.push(&[(31, 0.5006027), (32, 0.25029373), (8, 0.16568246), (9, 0.08284123)], 0.99942017);
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
                        ['v', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'd', '\u{1}', s @ ..] => {
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
                ['n', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['h', 'e', 'r', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', 'r', s @ ..] => {
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
                                        _ => false,
                                    }
                                }
                                ['m', 'o', 'c', s @ ..] => {
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
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'p', s @ ..] => {
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
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['e', 'c', 'c', 'a', s @ ..] => {
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
                        ['o', 'p', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['u', 'f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => super::t6::n18954(s, out),
                                ['n', 'i', '\u{1}', s @ ..] => super::t6::n18955(s, out),
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['t', s @ ..] => super::t6::n4030(s, out),
        _ => false,
    }
}

/// Words ending -obilis.
pub(crate) fn n1598(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.36636913), (31, 0.2002594), (9, 0.18318456), (32, 0.1001297), (29, 0.099488825), (30, 0.049744412)], 0.999176);
    match s {
        [] => true,
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['i', 'c', 'u', 'a', 'p', '\u{1}', s @ ..] => {
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
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(29, 0.6666667), (30, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['g', s @ ..] => {
                    out.push(&[(31, 0.5111925), (32, 0.25560388), (8, 0.12211795), (9, 0.061066605), (29, 0.033157855), (30, 0.016586557)], 0.99972534);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
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

/// Words ending -ubilis.
pub(crate) fn n1599(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.58506143), (9, 0.29253072), (160, 0.078645), (161, 0.039215688), (31, 0.002166781)], 0.9976196);
    match s {
        [] => true,
        ['l', 'o', 's', s @ ..] => {
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
                        ['r', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['s', 'i', 'd', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(160, 0.6666667), (161, 0.33333334)], 1.0);
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

/// Words ending -uminans.
pub(crate) fn n4607(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -pinans.
pub(crate) fn n1919(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.49042496), (2, 0.40383002), (333, 0.07049668), (334, 0.03524834)], 1.0);
    match s {
        [] => true,
        ['a', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['l', 'u', 'v', '\u{1}', s @ ..] => {
            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', 'e', 'n', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    out.push(&[(333, 0.46794844), (334, 0.23396659), (1, 0.1634699), (2, 0.1346151)], 1.0);
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

/// Words ending -tinans.
pub(crate) fn n1920(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -inquinans.
pub(crate) fn n1921(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -pignerans.
pub(crate) fn n4703(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -vulnerans.
pub(crate) fn n4704(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -onerans.
pub(crate) fn n4705(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -munerans.
pub(crate) fn n4706(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -perans.
pub(crate) fn n1973(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.6104219), (2, 0.32274356), (17, 0.04446479), (16, 0.022232395)], 0.9998627);
    match s {
        [] => true,
        ['m', s @ ..] => {
            out.push(&[(1, 0.50276953), (2, 0.25249103), (17, 0.16314946), (16, 0.08157473)], 0.99998474);
            match s {
                [] => true,
                ['e', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(1, 0.35088122), (17, 0.31578547), (2, 0.17543298), (16, 0.15790036)], 1.0);
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
                                ['i', s @ ..] => {
                                    out.push(&[(17, 0.49883267), (16, 0.2494087), (1, 0.16758984), (2, 0.08416876)], 1.0);
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
                        ['p', 'o', '\u{1}', s @ ..] => {
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
                ['n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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

/// Words ending -serans.
pub(crate) fn n1974(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -terans.
pub(crate) fn n1975(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -verans.
pub(crate) fn n1976(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', 's', 'd', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// The word nobilitans.
pub(crate) fn n14233(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word militans.
pub(crate) fn n9181(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word appellitans.
pub(crate) fn n4896(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word volitans.
pub(crate) fn n4897(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -mitans.
pub(crate) fn n2052(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -pitans.
pub(crate) fn n2053(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -usitans.
pub(crate) fn n2054(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -actitans.
pub(crate) fn n2055(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -equitans.
pub(crate) fn n2056(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -vitans.
pub(crate) fn n2057(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -escens.
pub(crate) fn n2169(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6533913), (4, 0.34177157), (33, 0.0032196536)], 0.9983825);
    match s {
        [] => true,
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', 't', s @ ..] => {
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
                ['e', 'r', 'c', s @ ..] => {
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
                ['l', 'a', s @ ..] => {
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
                ['u', 'r', s @ ..] => {
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
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'u', 'd', s @ ..] => {
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
                ['r', 'a', 'm', s @ ..] => {
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
                ['s', 'u', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'l', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                            s.is_empty()
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
                                ['i', 'd', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['i', 'r', 'i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['v', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'a', 'c', s @ ..] => {
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
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
                        ['o', 's', s @ ..] => {
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
                _ => false,
            }
        }
        ['g', 'i', 'r', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'o', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', s @ ..] => {
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
            out.push(&[(3, 0.61286336), (4, 0.35133898), (33, 0.023865111), (34, 0.011902037)], 0.9999695);
            match s {
                [] => true,
                ['a', s @ ..] => {
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
                                ['l', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['n', 'i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['p', 'o', '\u{1}', s @ ..] => {
                            out.push(&[(4, 0.6666667), (3, 0.33333334)], 1.0);
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
                                _ => false,
                            }
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => super::t6::n9371(s, out),
                ['o', s @ ..] => super::t6::n9372(s, out),
                ['u', s @ ..] => {
                    out.push(&[(4, 0.39427787), (3, 0.36621654), (33, 0.15967041), (34, 0.079835206)], 1.0);
                    match s {
                        [] => true,
                        ['d', 'a', s @ ..] => super::t6::n14321(s, out),
                        ['t', 's', 'i', 'f', '\u{1}', s @ ..] => super::t6::n14322(s, out),
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['m', 'u', 't', s @ ..] => super::t6::n5145(s, out),
        ['n', s @ ..] => super::t6::n5146(s, out),
        ['p', 'u', 't', 's', s @ ..] => super::t6::n5147(s, out),
        ['r', s @ ..] => super::t6::n5148(s, out),
        ['s', 'i', 'r', 'g', '\u{1}', s @ ..] => super::t6::n5149(s, out),
        ['t', 'o', 'n', s @ ..] => super::t6::n5150(s, out),
        ['u', s @ ..] => super::t6::n5151(s, out),
        ['v', s @ ..] => super::t6::n5152(s, out),
        _ => false,
    }
}

/// Words ending -oscens.
pub(crate) fn n2171(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['g', s @ ..] => {
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
                        ['i', s @ ..] => {
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
                ['i', 'd', '\u{1}', s @ ..] => {
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
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -stendens.
pub(crate) fn n9536(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['o', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -extendens.
pub(crate) fn n9538(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -vendens.
pub(crate) fn n5263(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -indens.
pub(crate) fn n2202(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ondens.
pub(crate) fn n2203(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'f', '\u{1}', s @ ..] => {
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

/// Words ending -undens.
pub(crate) fn n2204(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['f', s @ ..] => {
            out.push(&[(3, 0.62659645), (4, 0.31343555), (329, 0.03997864), (330, 0.01998932)], 0.99999994);
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
                    out.push(&[(329, 0.45777065), (330, 0.22888532), (3, 0.2088655), (4, 0.10447852)], 1.0);
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

/// The word adstringens.
pub(crate) fn n14509(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -estringens.
pub(crate) fn n14510(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word distringens.
pub(crate) fn n14511(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word circumstringens.
pub(crate) fn n14512(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -nstringens.
pub(crate) fn n14513(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word perstringens.
pub(crate) fn n14514(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -tingens.
pub(crate) fn n5362(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['t', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ungens.
pub(crate) fn n2236(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -candus.
pub(crate) fn n2670(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'p', s @ ..] => {
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
                        ['e', '\u{1}', s @ ..] => {
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
                ['s', s @ ..] => {
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
                                ['t', 'b', 'u', 's', '\u{1}', s @ ..] => {
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
                        ['e', s @ ..] => super::t6::n14838(s, out),
                        ['n', s @ ..] => super::t6::n14839(s, out),
                        ['o', s @ ..] => super::t6::n14840(s, out),
                        ['u', s @ ..] => super::t6::n14841(s, out),
                        _ => false,
                    }
                }
                ['f', 'i', s @ ..] => super::t6::n10406(s, out),
                ['l', 'p', s @ ..] => super::t6::n10407(s, out),
                ['r', s @ ..] => super::t6::n10408(s, out),
                _ => false,
            }
        }
        ['l', s @ ..] => super::t6::n6105(s, out),
        ['n', 'u', 'r', s @ ..] => super::t6::n6106(s, out),
        ['o', s @ ..] => super::t6::n6107(s, out),
        ['r', s @ ..] => super::t6::n6108(s, out),
        ['s', s @ ..] => super::t6::n6109(s, out),
        ['u', 'd', 'n', 'a', 'm', s @ ..] => super::t6::n6110(s, out),
        _ => false,
    }
}

/// Words ending -sedandus.
pub(crate) fn n10428(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -idandus.
pub(crate) fn n6114(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['f', 'f', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
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
                ['a', 'l', '\u{1}', s @ ..] => {
                    s.is_empty()
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

/// Words ending -ndandus.
pub(crate) fn n6116(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['b', 'a', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
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

/// Words ending -odandus.
pub(crate) fn n6117(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -rdandus.
pub(crate) fn n6118(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -udandus.
pub(crate) fn n6119(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -eandus.
pub(crate) fn n2672(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['l', 'o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -fandus.
pub(crate) fn n2673(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['n', '\u{1}', s @ ..] => {
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
        ['n', 'i', '\u{1}', s @ ..] => {
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

