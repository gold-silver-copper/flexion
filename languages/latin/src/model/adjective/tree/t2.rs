use flexion::Path;

/// Words ending -simus.
pub(crate) fn n1109(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.96906996), (58, 0.028717479), (401, 0.0022125582)], 1.0);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 'r', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'i', s @ ..] => {
            match s {
                [] => true,
                ['b', 'o', 'r', 'p', s @ ..] => {
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
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'p', s @ ..] => {
                            out.push(&[(58, 0.67623407), (0, 0.32301822)], 0.9992523);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', 'u', 'p', s @ ..] => {
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
                                ['l', 'e', 'f', s @ ..] => {
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
                            out.push(&[(58, 0.67623407), (0, 0.32301822)], 0.9992523);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'u', s @ ..] => {
                            out.push(&[(0, 0.5938201), (58, 0.4057374)], 0.9995575);
                            match s {
                                [] => true,
                                ['q', 'i', 'l', '\u{1}', s @ ..] => {
                                    out.push(&[(58, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', 'm', 'm', 'o', 'c', s @ ..] => {
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
                ['i', 'p', s @ ..] => {
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
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'u', 'q', 'e', 'a', s @ ..] => {
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
                        ['e', 'd', 'i', 'f', s @ ..] => {
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
                ['m', 'r', 'i', 'f', s @ ..] => {
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
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'm', 'u', 'h', s @ ..] => {
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
                        ['e', 'l', s @ ..] => {
                            out.push(&[(0, 0.5938201), (58, 0.4057374)], 0.9995575);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['p', '\u{1}', s @ ..] => {
                                    out.push(&[(58, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['g', 'i', 'd', s @ ..] => {
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
                            out.push(&[(0, 0.709865), (58, 0.2898146)], 0.99967957);
                            match s {
                                [] => true,
                                ['v', s @ ..] => super::t3::n16388(s, out),
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => super::t3::n6492(s, out),
                ['s', s @ ..] => super::t3::n6493(s, out),
                ['t', s @ ..] => super::t3::n6494(s, out),
                ['u', s @ ..] => super::t3::n6495(s, out),
                ['v', s @ ..] => super::t3::n6496(s, out),
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -timus.
pub(crate) fn n1110(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.91522086), (219, 0.08332952)], 0.99855036);
    match s {
        [] => true,
        ['i', 'g', 'e', 'l', s @ ..] => {
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
        ['l', 'u', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
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
                                ['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
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
            out.push(&[(219, 0.6944381), (0, 0.30507362)], 0.9995117);
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
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

/// Words ending -ximus.
pub(crate) fn n1111(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.5930724), (249, 0.4), (58, 0.005004959)], 0.9980774);
    match s {
        [] => true,
        ['a', 'm', '\u{1}', s @ ..] => {
            out.push(&[(249, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rmus.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.50045013), (0, 0.4952621), (58, 0.0028992142)], 0.99861145);
    match s {
        [] => true,
        ['e', 'n', 'i', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(7, 0.9000839), (0, 0.09904631)], 0.9991302);
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
                _ => false,
            }
        }
        ['o', 'f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'c', 's', 'i', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -umus.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.85900664), (281, 0.1111162), (220, 0.02777142)], 0.9978943);
    match s {
        [] => true,
        ['s', s @ ..] => {
            out.push(&[(281, 0.7037003), (0, 0.28633556), (220, 0.0092622265)], 0.9992981);
            s.is_empty()
        }
        ['t', 'n', 'i', '\u{1}', s @ ..] => {
            out.push(&[(220, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -imanus.
pub(crate) fn n2913(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8780194), (63, 0.11047532), (7, 0.011444267)], 0.99993896);
    match s {
        [] => true,
        ['b', 'l', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['c', 'e', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['d', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', 'u', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['u', 'g', 'n', 'a', '\u{1}', s @ ..] => {
            out.push(&[(63, 0.6666667), (0, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['v', 'a', 'l', 'f', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -umanus.
pub(crate) fn n2916(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.59186697), (0, 0.4012207), (63, 0.006790265)], 0.9998779);
    match s {
        [] => true,
        ['c', 'e', 'd', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['h', s @ ..] => {
            out.push(&[(7, 0.9183795), (0, 0.08024719)], 0.9986267);
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

/// Words ending -rhenanus.
pub(crate) fn n1125(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -sanus.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.632105), (7, 0.18371862), (15, 0.18219273)], 0.99801636);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(15, 1.0)], 1.0);
            s.is_empty()
        }
        ['e', s @ ..] => {
            out.push(&[(0, 0.93868923), (7, 0.030624857), (15, 0.030365454)], 0.99967957);
            match s {
                [] => true,
                ['l', 'a', 'm', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['v', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            out.push(&[(7, 0.7279011), (0, 0.21069658), (15, 0.060730908)], 0.9993286);
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

/// Words ending -tanus.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9912871), (271, 0.00796521)], 0.9992523);
    match s {
        [] => true,
        ['i', 'c', s @ ..] => {
            out.push(&[(0, 0.66085297), (271, 0.33864346)], 0.99949646);
            s.is_empty()
        }
        ['n', 'o', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'u', 's', '\u{1}', s @ ..] => {
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

/// Words ending -vanus.
pub(crate) fn n1128(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.70759135), (12, 0.2863203), (7, 0.0029907683)], 0.9969024);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(12, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -malayanus.
pub(crate) fn n1129(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', 'h', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -enus.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.89271384), (7, 0.05146868), (166, 0.034210727), (12, 0.017120622), (271, 0.004272526)], 0.9997864);
    match s {
        [] => true,
        ['c', s @ ..] => {
            out.push(&[(0, 0.73902494), (12, 0.13357748), (166, 0.09056229), (271, 0.033402), (7, 0.0034027619)], 0.9999695);
            match s {
                [] => true,
                ['i', 'v', s @ ..] => {
                    out.push(&[(166, 0.47463188), (0, 0.4685588), (12, 0.044525824), (271, 0.011139086)], 0.9988556);
                    match s {
                        [] => true,
                        ['e', 'd', 'o', 'u', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(166, 0.6666667), (0, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'b', s @ ..] => {
                    out.push(&[(12, 0.7111925), (0, 0.24634165), (166, 0.030182345), (271, 0.011139086)], 0.9988556);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            out.push(&[(0, 0.91841), (166, 0.07943847)], 0.99784845);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(0, 0.5283589), (166, 0.47092393)], 0.99928284);
                    match s {
                        [] => true,
                        ['r', 'd', 'a', 'u', 'q', 'e', 'd', 'o', 'u', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(166, 0.6666667), (0, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'v', s @ ..] => {
                    out.push(&[(0, 0.5283589), (166, 0.47092393)], 0.99928284);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(166, 0.6666667), (0, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(0, 0.5785458), (7, 0.41029984), (166, 0.006836042), (12, 0.003418021)], 0.99909973);
            match s {
                [] => true,
                ['l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['a', 'c', s @ ..] => {
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
                    out.push(&[(0, 0.5785458), (7, 0.41029984), (166, 0.006836042), (12, 0.003418021)], 0.99909973);
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
        ['o', s @ ..] => {
            out.push(&[(0, 0.7658503), (7, 0.22793926), (166, 0.0037994965)], 0.99758905);
            match s {
                [] => true,
                ['m', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
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

/// Words ending -gnus.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.7823453), (7, 0.10872053), (81, 0.054047454), (237, 0.03604181), (238, 0.018020906)], 0.999176);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.7983978), (237, 0.1244831), (238, 0.06224155), (7, 0.009887846), (81, 0.004913405)], 0.9999237);
            match s {
                [] => true,
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(237, 0.6666667), (238, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'a', 'u', 'q', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'l', 'a', '\u{1}', s @ ..] => {
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
            out.push(&[(0, 0.7039139), (7, 0.19565117), (81, 0.09781033)], 0.9973754);
            match s {
                [] => true,
                ['d', s @ ..] => {
                    out.push(&[(0, 0.52265203), (7, 0.4661784), (81, 0.010864424)], 0.9996948);
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
                                    out.push(&[(7, 1.0)], 1.0);
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
                ['l', s @ ..] => {
                    out.push(&[(0, 0.9671016), (7, 0.021744106), (81, 0.010864424)], 0.9997101);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    out.push(&[(81, 0.6992752), (0, 0.23463798), (7, 0.06521706)], 0.99913025);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -inus.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.97752345), (7, 0.007507439), (15, 0.0037384604), (177, 0.0037232013), (193, 0.0037232013), (392, 0.0037232013)], 0.99993896);
    match s {
        [] => true,
        ['c', s @ ..] => {
            out.push(&[(0, 0.9361563), (15, 0.0317998), (193, 0.0317998)], 0.99975586);
            match s {
                [] => true,
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', s @ ..] => {
                            out.push(&[(193, 0.67727166), (0, 0.312047), (15, 0.01060502)], 0.9999237);
                            s.is_empty()
                        }
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['a', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'r', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['i', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['v', s @ ..] => {
                            out.push(&[(15, 0.67727166), (0, 0.312047), (193, 0.01060502)], 0.9999237);
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
                        ['p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            out.push(&[(0, 0.96810865), (7, 0.031647213)], 0.99975586);
            match s {
                [] => true,
                ['u', 'c', s @ ..] => {
                    out.push(&[(0, 0.59362173), (7, 0.4063325)], 0.9999542);
                    match s {
                        [] => true,
                        ['s', 'a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['m', 'e', 'g', s @ ..] => {
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
                ['r', 'e', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['e', 'b', 'e', s @ ..] => {
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
                _ => false,
            }
        }
        ['p', s @ ..] => {
            out.push(&[(0, 0.9222095), (392, 0.07707332)], 0.99928284);
            match s {
                [] => true,
                ['l', 'a', s @ ..] => {
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
                ['u', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(392, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => {
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
        ['r', s @ ..] => {
            out.push(&[(0, 0.981033), (177, 0.018783856)], 0.9998169);
            match s {
                [] => true,
                ['a', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 'f', s @ ..] => {
                    out.push(&[(177, 0.67292285), (0, 0.3270161)], 0.99993896);
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'q', '\u{1}', s @ ..] => {
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
                        ['i', 'v', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
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
                ['c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(0, 0.99980164)], 0.99980164);
            match s {
                [] => true,
                ['a', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['l', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'e', 'n', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['v', s @ ..] => {
            out.push(&[(0, 0.8952163), (7, 0.10402075)], 0.99923706);
            match s {
                [] => true,
                ['i', s @ ..] => {
                    out.push(&[(7, 0.60114443), (0, 0.3986267)], 0.9997711);
                    match s {
                        [] => true,
                        ['l', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
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
                        ['b', '\u{1}', s @ ..] => {
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

/// Words ending -nnus.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8214694), (15, 0.17444114)], 0.9959106);
    match s {
        [] => true,
        ['i', 'c', 'n', 'o', 'c', s @ ..] => {
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

/// Words ending -onus.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.94125277), (7, 0.02931258), (89, 0.019317921), (90, 0.009658961)], 0.99954224);
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(0, 0.5882506), (89, 0.2705272), (90, 0.13527124), (7, 0.0058594644)], 0.9999085);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(89, 0.6666667), (90, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            out.push(&[(0, 0.5882506), (7, 0.40585947), (89, 0.0038605325)], 0.9979706);
            match s {
                [] => true,
                ['p', '\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            out.push(&[(0, 0.9985962)], 0.9985962);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rnus.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9636683), (15, 0.035507742)], 0.999176);
    match s {
        [] => true,
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['t', 'e', 'a', s @ ..] => {
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
                ['v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'a', '\u{1}', s @ ..] => {
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
                ['i', 'd', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'u', 'q', 's', 'e', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    out.push(&[(0, 0.78564125), (15, 0.21426718)], 0.99990845);
                    match s {
                        [] => true,
                        ['u', s @ ..] => {
                            out.push(&[(15, 0.73809415), (0, 0.26187533)], 0.9999695);
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

/// Words ending -unus.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.5111162), (7, 0.26036468), (270, 0.107881285), (408, 0.043152515), (12, 0.03257801), (277, 0.032364387), (405, 0.010788129)], 0.99824524);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(270, 0.6666667), (408, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['i', 'r', 't', '\u{1}', s @ ..] => {
            out.push(&[(270, 0.6666667), (405, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['t', s @ ..] => {
            out.push(&[(0, 0.45111772), (7, 0.4260319), (12, 0.053253986), (277, 0.053238727), (270, 0.010788129), (408, 0.0043183034)], 0.9987488);
            match s {
                [] => true,
                ['r', 'o', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', '\u{1}', s @ ..] => {
                            out.push(&[(277, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['p', 'o', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -orus.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.98449683), (7, 0.007751583), (85, 0.007690547)], 0.99993896);
    match s {
        [] => true,
        ['c', 'e', 'd', s @ ..] => {
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', 'o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
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
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
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
                                        ['l', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['c', 'u', 'a', 'p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['i', 'r', 'i', 'v', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['n', 'u', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['t', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['l', 'u', 'm', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['u', 'c', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['u', 'n', 'e', 't', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['v', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['e', 'r', 'b', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['r', 'a', 'p', '\u{1}', s @ ..] => {
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
                ['h', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'o', 'h', s @ ..] => {
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
        ['p', 'i', s @ ..] => {
            out.push(&[(0, 0.65632105), (85, 0.33846036), (7, 0.0051728087)], 0.9999542);
            match s {
                [] => true,
                ['n', 'i', 'm', 'u', 'l', 'i', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(85, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -urus.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.99691767)], 0.99691767);
    match s {
        [] => true,
        ['c', s @ ..] => {
            out.push(&[(0, 0.6519112), (7, 0.17390707), (15, 0.17390707)], 0.99972534);
            match s {
                [] => true,
                ['e', s @ ..] => {
                    out.push(&[(15, 0.7246357), (0, 0.21730374), (7, 0.057969023)], 0.9999085);
                    s.is_empty()
                }
                ['s', 'b', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
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
        ['d', s @ ..] => {
            out.push(&[(7, 0.6666667), (0, 0.33231097)], 0.99897766);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(0, 0.61670864), (12, 0.38281834)], 0.999527);
            match s {
                [] => true,
                ['r', s @ ..] => {
                    out.push(&[(12, 0.8211032), (0, 0.17875944)], 0.9998627);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            out.push(&[(7, 0.7272755), (0, 0.18126193), (15, 0.09091325)], 0.9994507);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => super::t3::n1195(s, out),
        ['t', s @ ..] => super::t3::n1196(s, out),
        ['x', s @ ..] => {
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', s @ ..] => {
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
                                        ['f', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['n', 'i', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['u', 'n', 'e', 'g', '\u{1}', s @ ..] => {
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
                                        ['m', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['a', '\u{1}', s @ ..] => {
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
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['b', 'u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['d', 'a', '\u{1}', s @ ..] => {
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
                                        ['o', 'c', '\u{1}', s @ ..] => {
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
                                        ['u', 's', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['i', 'c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
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
                                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', s @ ..] => super::t3::n6765(s, out),
                        _ => false,
                    }
                }
                ['u', 'l', 'f', s @ ..] => super::t3::n3044(s, out),
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -visus.
pub(crate) fn n1213(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['n', s @ ..] => {
            out.push(&[(0, 0.61748683), (7, 0.25238422), (221, 0.12614633), (15, 0.0039063096)], 0.9999237);
            match s {
                [] => true,
                ['i', '\u{1}', s @ ..] => {
                    out.push(&[(0, 0.5263142), (7, 0.31578547), (221, 0.15790036)], 1.0);
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
                ['n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -lsus.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.88238347), (15, 0.034683757), (7, 0.02793927), (12, 0.02761883), (215, 0.02731365)], 0.99993896);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.5240711), (12, 0.36865798), (15, 0.09721523), (7, 0.0050812545), (215, 0.0049591823)], 0.99998474);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['f', '\u{1}', s @ ..] => {
                    out.push(&[(12, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(0, 0.54248875), (15, 0.22607768), (7, 0.22532998), (12, 0.0030670634), (215, 0.0030365454)], 1.0);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['x', 'e', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
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
        ['u', s @ ..] => {
            out.push(&[(0, 0.96368355), (215, 0.03477531)], 0.99845886);
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
                        ['p', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                ['s', s @ ..] => {
                    out.push(&[(215, 0.67826355), (0, 0.32123291)], 0.99949646);
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

/// Words ending -nsus.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.92532235), (15, 0.0605478), (7, 0.01123064), (12, 0.0028839551)], 0.99998474);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', 's', s @ ..] => {
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
                ['r', 'p', s @ ..] => {
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
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(0, 0.9976043)], 0.9976043);
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
                        ['e', 'r', '\u{1}', s @ ..] => {
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
                                ['n', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['a', 'r', 't', '\u{1}', s @ ..] => {
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
                ['d', s @ ..] => {
                    out.push(&[(0, 0.58506143), (7, 0.40224308), (15, 0.012115664)], 0.99942017);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['f', s @ ..] => {
                    out.push(&[(0, 0.68656445), (15, 0.31235218)], 0.9989166);
                    match s {
                        [] => true,
                        ['e', 'd', s @ ..] => {
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
                        ['f', 'o', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
                                    s.is_empty()
                                }
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
                ['m', s @ ..] => super::t3::n3100(s, out),
                ['p', s @ ..] => super::t3::n3101(s, out),
                ['r', 'p', s @ ..] => super::t3::n3102(s, out),
                ['s', s @ ..] => super::t3::n3103(s, out),
                ['t', s @ ..] => super::t3::n3104(s, out),
                _ => false,
            }
        }
        ['o', s @ ..] => super::t3::n1219(s, out),
        ['u', 't', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => super::t3::n3108(s, out),
                ['b', 'o', '\u{1}', s @ ..] => super::t3::n3109(s, out),
                ['e', 'r', '\u{1}', s @ ..] => super::t3::n3110(s, out),
                ['n', 'o', 'c', '\u{1}', s @ ..] => super::t3::n3111(s, out),
                ['r', 'e', 'p', '\u{1}', s @ ..] => super::t3::n3112(s, out),
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -osus.
pub(crate) fn n417(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8157626), (7, 0.086717024), (15, 0.052979324), (12, 0.04213016), (215, 0.0024109255)], 1.0);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', 'l', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'b', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', s @ ..] => {
                    out.push(&[(7, 0.69556725), (0, 0.27191576), (15, 0.017654689), (12, 0.0140383)], 0.999176);
                    s.is_empty()
                }
                ['o', 'l', 'g', s @ ..] => {
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
                ['r', s @ ..] => {
                    out.push(&[(0, 0.68796825), (15, 0.2932784), (7, 0.012390326), (12, 0.0060120546)], 0.99964905);
                    match s {
                        [] => true,
                        ['e', 'v', s @ ..] => {
                            out.push(&[(15, 0.76443124), (0, 0.22932784), (7, 0.004135195), (12, 0.0019989319)], 0.9998932);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            out.push(&[(0, 0.85766387), (12, 0.09303426), (15, 0.047241934)], 0.99794006);
            match s {
                [] => true,
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', s @ ..] => {
                            out.push(&[(12, 0.697673), (0, 0.28589302), (15, 0.01574731)], 0.99931335);
                            s.is_empty()
                        }
                        ['r', 'b', s @ ..] => {
                            out.push(&[(0, 0.57152665), (12, 0.41860074), (15, 0.009445335)], 0.9995727);
                            match s {
                                [] => true,
                                ['e', 'n', 'e', 't', '\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    out.push(&[(0, 0.61048293), (15, 0.3722286), (12, 0.016906997)], 0.99961853);
                    match s {
                        [] => true,
                        ['u', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
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
        ['d', 'n', s @ ..] => {
            out.push(&[(0, 0.40225834), (7, 0.29810026), (12, 0.29173723), (15, 0.007568475)], 0.9996643);
            match s {
                [] => true,
                ['e', s @ ..] => {
                    out.push(&[(12, 0.76391244), (0, 0.13408102), (7, 0.099366754), (15, 0.0025177386)], 0.9998779);
                    s.is_empty()
                }
                ['o', 'r', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', s @ ..] => {
                    out.push(&[(7, 0.7660334), (0, 0.13408102), (12, 0.097245745), (15, 0.0025177386)], 0.9998779);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(0, 0.98582435), (7, 0.006668193), (15, 0.004074159), (12, 0.0032349126)], 0.99980164);
            match s {
                [] => true,
                ['b', 'a', 'l', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['a', 'r', 'f', s @ ..] => {
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
                ['l', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'i', s @ ..] => {
                    out.push(&[(7, 0.53165483), (0, 0.41834134), (15, 0.02716106), (12, 0.021606775)], 0.99876404);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(0, 0.63282216), (7, 0.18483253), (15, 0.12150759), (12, 0.06083772)], 1.0);
            match s {
                [] => true,
                ['b', s @ ..] => {
                    out.push(&[(0, 0.95410085), (7, 0.023102159), (15, 0.015182727), (12, 0.007598993)], 0.99998474);
                    s.is_empty()
                }
                ['c', s @ ..] => {
                    out.push(&[(7, 0.5115587), (0, 0.35205615), (12, 0.1288014), (15, 0.007598993)], 1.0000153);
                    match s {
                        [] => true,
                        ['a', 'd', 'n', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['f', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['f', 'o', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                ['n', s @ ..] => {
                                                    out.push(&[(0, 0.7840238), (7, 0.17051958), (12, 0.042938888), (15, 0.0025329976)], 1.0000153);
                                                    match s {
                                                        [] => true,
                                                        ['i', '\u{1}', s @ ..] => super::t3::n23754(s, out),
                                                        _ => false,
                                                    }
                                                }
                                                _ => false,
                                            }
                                        }
                                        ['i', 't', 'r', 'a', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                ['n', s @ ..] => {
                                                    out.push(&[(0, 0.7840238), (7, 0.17051958), (12, 0.042938888), (15, 0.0025329976)], 1.0000153);
                                                    match s {
                                                        [] => true,
                                                        ['i', '\u{1}', s @ ..] => super::t3::n23755(s, out),
                                                        _ => false,
                                                    }
                                                }
                                                _ => false,
                                            }
                                        }
                                        _ => false,
                                    }
                                }
                                ['l', 'e', 'd', '\u{1}', s @ ..] => {
                                    out.push(&[(0, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['p', s @ ..] => {
                                    out.push(&[(12, 0.7096056), (7, 0.17051958), (0, 0.117357135), (15, 0.0025329976)], 1.0000153);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['d', s @ ..] => {
                    out.push(&[(0, 0.42531472), (7, 0.32739758), (15, 0.24486153), (12, 0.0024261845)], 1.0);
                    match s {
                        [] => true,
                        ['a', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'a', 't', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['f', s @ ..] => {
                                    out.push(&[(15, 0.7482872), (0, 0.14177157), (7, 0.10913253)], 0.9991913);
                                    s.is_empty()
                                }
                                ['s', s @ ..] => {
                                    out.push(&[(7, 0.8654765), (0, 0.085069045), (15, 0.048966203)], 0.9995117);
                                    s.is_empty()
                                }
                                ['t', 's', 'a', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(0, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['v', s @ ..] => {
                                    out.push(&[(7, 0.7757992), (0, 0.14177157), (15, 0.08162051)], 0.9991913);
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
                                    out.push(&[(0, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'o', 'c', 's', 'i', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['u', s @ ..] => {
                            out.push(&[(7, 0.46547645), (15, 0.4489662), (0, 0.085069045)], 0.9995117);
                            match s {
                                [] => true,
                                ['t', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['g', s @ ..] => {
                    out.push(&[(0, 0.8601358), (7, 0.13096818), (15, 0.0059205005), (12, 0.0029602502)], 0.99998474);
                    match s {
                        [] => true,
                        ['i', 'l', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', 'r', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(7, 1.0)], 1.0);
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
                                ['l', 'e', 'r', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
                    out.push(&[(0, 0.81791407), (12, 0.15852597), (7, 0.014221408), (15, 0.00935378)], 1.0000153);
                    match s {
                        [] => true,
                        ['e', s @ ..] => super::t3::n6911(s, out),
                        _ => false,
                    }
                }
                ['n', s @ ..] => super::t3::n3133(s, out),
                ['p', s @ ..] => super::t3::n3134(s, out),
                ['r', s @ ..] => super::t3::n3135(s, out),
                ['t', s @ ..] => super::t3::n3136(s, out),
                ['v', s @ ..] => super::t3::n3137(s, out),
                _ => false,
            }
        }
        ['l', s @ ..] => super::t3::n1228(s, out),
        ['m', s @ ..] => super::t3::n1229(s, out),
        ['n', s @ ..] => super::t3::n1230(s, out),
        ['r', s @ ..] => super::t3::n1231(s, out),
        ['t', s @ ..] => super::t3::n1232(s, out),
        ['u', s @ ..] => super::t3::n1233(s, out),
        ['v', s @ ..] => super::t3::n1234(s, out),
        ['x', 'e', '\u{1}', s @ ..] => super::t3::n1235(s, out),
        _ => false,
    }
}

/// Words ending -lapsus.
pub(crate) fn n1236(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
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
                ['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rsus.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9541466), (12, 0.045181964)], 0.9993286);
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
                ['p', s @ ..] => {
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
                ['x', 'e', '\u{1}', s @ ..] => {
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
                    out.push(&[(0, 0.8659495), (12, 0.13403524)], 0.99998474);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', s @ ..] => {
                            out.push(&[(12, 0.60972), (0, 0.39026475)], 0.99998474);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', 't', '\u{1}', s @ ..] => {
                                    out.push(&[(0, 1.0)], 1.0);
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
                        ['d', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(12, 1.0)], 1.0);
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
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', s @ ..] => {
                                    out.push(&[(12, 0.7113451), (0, 0.28865492)], 1.0);
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
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
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 's', '\u{1}', s @ ..] => {
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
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['r', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(12, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
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
        ['u', 'c', s @ ..] => super::t3::n1240(s, out),
        _ => false,
    }
}

/// Words ending -ssus.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9126574), (7, 0.03385977), (15, 0.033157855), (124, 0.0067444877), (143, 0.0067444877), (325, 0.0067444877)], 0.99990845);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.7986572), (7, 0.19871823)], 0.9973754);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', s @ ..] => {
                    out.push(&[(7, 0.7329061), (0, 0.26622415)], 0.99913025);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    out.push(&[(7, 0.6281987), (0, 0.37103838)], 0.99923706);
                    match s {
                        [] => true,
                        ['c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['g', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(0, 0.89246964), (15, 0.04605173), (7, 0.015518425), (124, 0.015320058), (143, 0.015320058), (325, 0.015320058)], 1.0);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', s @ ..] => {
                    out.push(&[(0, 0.9969329)], 0.9969329);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
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
                                ['s', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['t', 'n', 'a', '\u{1}', s @ ..] => {
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
                        ['o', 'r', s @ ..] => {
                            match s {
                                [] => true,
                                ['p', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['t', 'e', 'r', '\u{1}', s @ ..] => {
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
                                ['b', 'a', '\u{1}', s @ ..] => {
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
                ['f', s @ ..] => {
                    out.push(&[(0, 0.53019), (7, 0.15504692), (143, 0.15503167), (325, 0.15503167), (15, 0.003540093)], 0.99884033);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            out.push(&[(0, 0.506035), (143, 0.43100634), (7, 0.031006332), (325, 0.031006332)], 0.99905396);
                            match s {
                                [] => true,
                                ['d', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(143, 1.0)], 1.0);
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
                        ['f', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            out.push(&[(325, 0.718349), (0, 0.17672999), (7, 0.05168231), (143, 0.05168231)], 0.9984436);
                            match s {
                                [] => true,
                                ['r', 'p', '\u{1}', s @ ..] => {
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
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', s @ ..] => super::t3::n3212(s, out),
                ['s', s @ ..] => {
                    out.push(&[(0, 0.9943389), (15, 0.0024261845)], 0.99676514);
                    match s {
                        [] => true,
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
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
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
        ['i', s @ ..] => super::t3::n1243(s, out),
        ['o', s @ ..] => super::t3::n1244(s, out),
        ['u', s @ ..] => super::t3::n1245(s, out),
        _ => false,
    }
}

/// Words ending -usus.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8967575), (15, 0.07255665), (7, 0.015411613), (12, 0.015228504)], 0.9999542);
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
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', s @ ..] => {
                            out.push(&[(15, 0.6908522), (0, 0.29892424), (7, 0.0051422906), (12, 0.0050812545)], 1.0);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                                ['p', 'a', '\u{1}', s @ ..] => {
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
                ['r', 'f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
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
                ['o', 'r', 'p', '\u{1}', s @ ..] => {
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
        ['e', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['f', s @ ..] => {
            out.push(&[(0, 0.78811324), (15, 0.10382239), (7, 0.054032195), (12, 0.054032195)], 1.0);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
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
                            out.push(&[(0, 0.5263142), (15, 0.47368583)], 1.0);
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
                    out.push(&[(0, 0.68401617), (7, 0.29343098), (15, 0.014831769), (12, 0.007721065)], 1.0);
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'c', s @ ..] => {
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
                ['o', s @ ..] => {
                    out.push(&[(12, 0.6846723), (0, 0.26269934), (15, 0.034607463), (7, 0.018005647)], 0.99998474);
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
        ['l', s @ ..] => {
            out.push(&[(0, 0.9971618)], 0.9971618);
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
                                ['s', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
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
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['e', 'p', '\u{1}', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            out.push(&[(0, 0.6034943), (15, 0.39011216), (7, 0.0032043946), (12, 0.0031738766)], 0.99998474);
                            match s {
                                [] => true,
                                ['b', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(0, 0.5263142), (15, 0.47368583)], 1.0);
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
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['b', s @ ..] => {
                    out.push(&[(0, 0.5793545), (15, 0.41451132), (7, 0.0030823224), (12, 0.0030518044)], 1.0);
                    match s {
                        [] => true,
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
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

/// Words ending -atus.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.97445637), (7, 0.011657893), (15, 0.007431144), (12, 0.005676356)], 0.99922174);
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(0, 0.97862214), (12, 0.021164263)], 0.9997864);
            match s {
                [] => true,
                ['i', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'd', s @ ..] => {
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
                        ['l', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
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
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['b', 'u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['g', 'n', 'o', 'c', '\u{1}', s @ ..] => {
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
                                _ => false,
                            }
                        }
                        ['r', s @ ..] => {
                            out.push(&[(0, 0.8382849), (12, 0.16169986)], 0.99998474);
                            match s {
                                [] => true,
                                ['p', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(12, 1.0)], 1.0);
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
                                _ => false,
                            }
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
                                ['s', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['a', 'm', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', s @ ..] => super::t3::n1258(s, out),
        ['d', s @ ..] => super::t4::n1259(s, out),
        ['e', s @ ..] => super::t4::n1260(s, out),
        ['f', s @ ..] => {
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
        ['g', s @ ..] => super::t4::n1262(s, out),
        ['h', 'c', 'c', 'a', 'b', s @ ..] => {
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
        ['i', s @ ..] => super::t4::n1264(s, out),
        ['l', s @ ..] => super::t4::n1265(s, out),
        ['m', s @ ..] => super::t4::n1266(s, out),
        ['n', s @ ..] => super::t4::n1267(s, out),
        ['p', s @ ..] => super::t4::n1268(s, out),
        ['r', s @ ..] => super::t4::n1269(s, out),
        ['s', s @ ..] => super::t4::n1270(s, out),
        ['t', s @ ..] => super::t4::n1271(s, out),
        ['u', s @ ..] => super::t4::n1272(s, out),
        ['v', s @ ..] => super::t4::n1273(s, out),
        ['x', s @ ..] => {
            match s {
                [] => true,
                ['a', 'l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', 'r', '\u{1}', s @ ..] => super::t4::n7603(s, out),
                        _ => false,
                    }
                }
                ['e', s @ ..] => super::t4::n3393(s, out),
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ctus.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.9326162), (7, 0.033188373), (15, 0.020569162), (12, 0.011367971), (396, 0.0022735943)], 1.0000153);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.962356), (15, 0.018753339), (12, 0.009872587), (7, 0.009002823)], 0.99998474);
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
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
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
                ['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['f', s @ ..] => {
                    out.push(&[(0, 0.9996033)], 0.9996033);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => super::t4::n7612(s, out),
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', 'e', 's', '\u{1}', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => super::t4::n3403(s, out),
                ['r', s @ ..] => super::t4::n3404(s, out),
                ['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => super::t4::n7627(s, out),
                        ['d', 'a', '\u{1}', s @ ..] => super::t4::n7628(s, out),
                        ['n', s @ ..] => super::t4::n7629(s, out),
                        ['t', 'a', '\u{1}', s @ ..] => super::t4::n7630(s, out),
                        _ => false,
                    }
                }
                ['x', s @ ..] => super::t4::n3407(s, out),
                _ => false,
            }
        }
        ['e', s @ ..] => super::t4::n1276(s, out),
        ['i', s @ ..] => super::t4::n1277(s, out),
        ['n', s @ ..] => super::t4::n1278(s, out),
        ['o', s @ ..] => super::t4::n1279(s, out),
        ['u', s @ ..] => super::t4::n1280(s, out),
        _ => false,
    }
}

/// Words ending -ecretus.
pub(crate) fn n7784(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['s', s @ ..] => {
            out.push(&[(15, 0.68755627), (0, 0.31149766)], 0.99905396);
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

/// The word concretus.
pub(crate) fn n13422(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -discretus.
pub(crate) fn n7786(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word excretus.
pub(crate) fn n7787(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -suetus.
pub(crate) fn n1288(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['a', 'm', s @ ..] => {
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

/// Words ending -vetus.
pub(crate) fn n1289(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(301, 0.26869613), (422, 0.26869613), (0, 0.17563134), (302, 0.13434044), (423, 0.13434044), (7, 0.012191959), (15, 0.00608835)], 0.9999848);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(422, 0.6666667), (423, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -itus.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.95165944), (7, 0.027893493), (15, 0.011612115), (12, 0.0066376748), (264, 0.0022125582)], 1.0000153);
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(0, 0.99935913)], 0.99935913);
            match s {
                [] => true,
                ['a', 'h', s @ ..] => {
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
                ['e', 'd', s @ ..] => {
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
                ['i', 'b', s @ ..] => {
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
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
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
                        ['l', '\u{1}', s @ ..] => {
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
                        ['s', s @ ..] => {
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
                ['u', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'r', 'p', '\u{1}', s @ ..] => {
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
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
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
                        ['t', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['c', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    out.push(&[(0, 0.88160527), (15, 0.116273746)], 0.997879);
                    match s {
                        [] => true,
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['e', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['l', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['i', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['o', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['p', '\u{1}', s @ ..] => {
                                                    s.is_empty()
                                                }
                                                ['s', s @ ..] => {
                                                    out.push(&[(15, 0.7054246), (0, 0.2938735)], 0.9992981);
                                                    match s {
                                                        [] => true,
                                                        ['\u{1}', s @ ..] => super::t4::n23836(s, out),
                                                        _ => false,
                                                    }
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
                                ['o', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['p', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['d', 'a', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['m', 'i', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['n', 'o', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'n', '\u{1}', s @ ..] => {
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
                                ['h', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['p', '\u{1}', s @ ..] => {
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
                            out.push(&[(0, 0.70110625), (7, 0.29889372)], 1.0);
                            s.is_empty()
                        }
                        ['a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => super::t4::n7821(s, out),
                        ['i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => super::t4::n7823(s, out),
                        _ => false,
                    }
                }
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => super::t4::n1292(s, out),
        ['g', s @ ..] => super::t4::n1293(s, out),
        ['l', s @ ..] => super::t4::n1294(s, out),
        ['m', 'o', s @ ..] => super::t4::n1295(s, out),
        ['n', s @ ..] => super::t4::n1296(s, out),
        ['p', s @ ..] => super::t4::n1297(s, out),
        ['r', s @ ..] => super::t4::n1298(s, out),
        ['s', s @ ..] => super::t4::n1299(s, out),
        ['t', s @ ..] => super::t4::n1300(s, out),
        ['u', s @ ..] => super::t4::n1301(s, out),
        ['v', s @ ..] => super::t4::n1302(s, out),
        _ => false,
    }
}

/// Words ending -ltus.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.81367207), (7, 0.106462196), (15, 0.026672771), (12, 0.026626993), (262, 0.017685207), (263, 0.008834974)], 0.9999542);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 0.5263142), (0, 0.47368583)], 1.0);
                    s.is_empty()
                }
                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', s @ ..] => {
                    out.push(&[(0, 0.45424584), (7, 0.2737621), (15, 0.13511863), (12, 0.13510337)], 0.99823);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['c', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', s @ ..] => {
                                    out.push(&[(7, 0.7579156), (0, 0.15141527), (15, 0.045044634), (12, 0.045029372)], 0.9994049);
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
                        ['e', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            out.push(&[(15, 0.7117113), (0, 0.15141527), (7, 0.09124895), (12, 0.045029372)], 0.9994049);
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['x', s @ ..] => {
                            out.push(&[(12, 0.711696), (0, 0.15141527), (7, 0.09124895), (15, 0.045044634)], 0.9994049);
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
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
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
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['f', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', '\u{1}', s @ ..] => {
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
                ['m', s @ ..] => {
                    out.push(&[(0, 0.56273746), (262, 0.27020675), (263, 0.13510337), (7, 0.021286335), (15, 0.0053406577), (12, 0.005325399)], 1.0);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(262, 0.6666667), (263, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', 'e', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
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
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    out.push(&[(0, 0.9874113), (7, 0.0071869995)], 0.9945983);
                    match s {
                        [] => true,
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
                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    out.push(&[(0, 0.56273746), (7, 0.42128634), (15, 0.0053406577), (12, 0.005325399), (262, 0.003540093)], 0.99823);
                    match s {
                        [] => true,
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
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
                ['x', 'e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ntus.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.83378345), (7, 0.071473256), (12, 0.04257267), (15, 0.0415198), (171, 0.010635538)], 0.9999847);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(0, 0.69054705), (7, 0.29591823), (12, 0.00608835), (15, 0.0059357597)], 0.9984894);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['c', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            out.push(&[(7, 0.765301), (0, 0.23018235), (12, 0.00202945)], 0.9975128);
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
                        ['i', 'l', 'a', '\u{1}', s @ ..] => {
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
                    out.push(&[(0, 0.75370413), (12, 0.08935683), (7, 0.067460135), (15, 0.067124434), (171, 0.022339208)], 0.99998474);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            out.push(&[(0, 0.52819103), (7, 0.22972457), (15, 0.2296788), (12, 0.0099336235), (171, 0.0024872206)], 1.0000153);
                            match s {
                                [] => true,
                                ['c', s @ ..] => {
                                    out.push(&[(7, 0.4459449), (15, 0.44592965), (0, 0.105638206)], 0.99751276);
                                    match s {
                                        [] => true,
                                        ['a', 'r', 'g', '\u{1}', s @ ..] => {
                                            out.push(&[(7, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['o', s @ ..] => {
                            out.push(&[(0, 0.7504082), (12, 0.23215076), (7, 0.00749218), (15, 0.007461662), (171, 0.0024872206)], 1.0);
                            match s {
                                [] => true,
                                ['i', s @ ..] => {
                                    out.push(&[(12, 0.74404514), (0, 0.25014114), (7, 0.0025024796), (15, 0.0024872206)], 0.99917597);
                                    match s {
                                        [] => true,
                                        ['v', '\u{1}', s @ ..] => {
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
                                ['b', s @ ..] => {
                                    out.push(&[(0, 0.5507439), (12, 0.41786832), (7, 0.013488975), (15, 0.013427939), (171, 0.0044708936)], 1.0);
                                    match s {
                                        [] => true,
                                        ['r', 'u', 't', '\u{1}', s @ ..] => {
                                            out.push(&[(12, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['c', s @ ..] => {
                                    out.push(&[(0, 0.7223926), (12, 0.092866406), (15, 0.09187457), (171, 0.08987564), (7, 0.0029907683)], 1.0);
                                    match s {
                                        [] => true,
                                        ['a', 'm', '\u{1}', s @ ..] => {
                                            s.is_empty()
                                        }
                                        ['s', s @ ..] => {
                                            out.push(&[(0, 0.5444724), (171, 0.41797513), (12, 0.01857023), (15, 0.018371863)], 0.9993896);
                                            match s {
                                                [] => true,
                                                ['e', '\u{1}', s @ ..] => {
                                                    out.push(&[(171, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        ['u', s @ ..] => {
                                            out.push(&[(0, 0.42964828), (12, 0.27904174), (15, 0.27891967), (171, 0.011978332)], 0.999588);
                                            match s {
                                                [] => true,
                                                ['l', s @ ..] => {
                                                    out.push(&[(12, 0.75968564), (0, 0.14322118), (15, 0.09297322), (171, 0.0039978637)], 0.9998779);
                                                    s.is_empty()
                                                }
                                                ['r', s @ ..] => {
                                                    out.push(&[(15, 0.75963986), (0, 0.14322118), (12, 0.093019), (171, 0.0039978637)], 0.9998779);
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        _ => false,
                                    }
                                }
                                ['n', s @ ..] => {
                                    out.push(&[(0, 0.97102314), (12, 0.010513466), (7, 0.007934691), (15, 0.007904174), (171, 0.0026245518)], 1.0);
                                    s.is_empty()
                                }
                                ['p', s @ ..] => {
                                    out.push(&[(0, 0.39339283), (7, 0.29535362), (15, 0.29530784), (12, 0.012771801), (171, 0.0031891356)], 1.0000153);
                                    match s {
                                        [] => true,
                                        ['a', 'r', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(0, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['o', s @ ..] => {
                                            out.push(&[(7, 0.7651179), (0, 0.13113603), (15, 0.09843595), (12, 0.0042572673)], 0.99894714);
                                            s.is_empty()
                                        }
                                        ['r', s @ ..] => {
                                            out.push(&[(15, 0.7651026), (0, 0.13113603), (7, 0.09845121), (12, 0.0042572673)], 0.99894714);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['r', s @ ..] => {
                                    out.push(&[(0, 0.8716411), (12, 0.11938659), (7, 0.0038605325), (15, 0.0038300145)], 0.99871826);
                                    match s {
                                        [] => true,
                                        ['i', s @ ..] => {
                                            out.push(&[(12, 0.7064622), (0, 0.29054704)], 0.9970093);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', s @ ..] => {
                                    out.push(&[(0, 0.97102314), (12, 0.010513466), (7, 0.007934691), (15, 0.007904174), (171, 0.0026245518)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    out.push(&[(0, 0.8432441), (7, 0.11654841), (15, 0.039078355)], 0.99887085);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['b', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', s @ ..] => {
                            out.push(&[(15, 0.55729), (0, 0.40704966), (7, 0.035309378)], 0.99964905);
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
                            out.push(&[(0, 0.55146104), (7, 0.44376287), (15, 0.0046387427)], 0.9998627);
                            match s {
                                [] => true,
                                ['i', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(0, 0.70110625), (7, 0.29889372)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['p', 'o', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'o', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                                ['o', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['p', 'a', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['u', 's', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', 'a', s @ ..] => {
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
                ['u', 'r', 'c', s @ ..] => {
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
                ['v', s @ ..] => {
                    out.push(&[(0, 0.9922637), (7, 0.0033264668)], 0.99559015);
                    match s {
                        [] => true,
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
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -otus.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8936599), (7, 0.08081178), (270, 0.020645456), (404, 0.0046387427)], 0.99975586);
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
                ['l', s @ ..] => {
                    out.push(&[(7, 0.69359887), (0, 0.29788664), (270, 0.006881819)], 0.9983673);
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
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            out.push(&[(7, 0.69359887), (0, 0.29788664), (270, 0.006881819)], 0.9983673);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'm', '\u{1}', s @ ..] => {
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
                    out.push(&[(7, 0.5263142), (0, 0.47368583)], 1.0);
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
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
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
                ['e', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(270, 0.5857023), (0, 0.25903717), (404, 0.13177691), (7, 0.023422599)], 0.99993896);
            s.is_empty()
        }
        ['u', 'q', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['i', 'l', 'a', '\u{1}', s @ ..] => {
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
                ['e', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ptus.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.93093765), (15, 0.028152896), (7, 0.027512016), (12, 0.013382162)], 0.99998474);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.8408484), (7, 0.15596247), (15, 0.002166781)], 0.99897766);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
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
                ['r', '\u{1}', s @ ..] => {
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
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', s @ ..] => {
                            out.push(&[(0, 0.6238041), (7, 0.36864272), (15, 0.0051270314), (12, 0.0024261845)], 1.0);
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
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
                                ['r', 'e', 'p', '\u{1}', s @ ..] => {
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
                                ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
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
                ['n', s @ ..] => {
                    out.push(&[(15, 0.67605096), (0, 0.31030747), (7, 0.009170672), (12, 0.0044556344)], 0.99998474);
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
                ['r', s @ ..] => {
                    out.push(&[(0, 0.99787897)], 0.99787897);
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
                        ['i', 'd', s @ ..] => {
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
            out.push(&[(0, 0.9984894)], 0.9984894);
            match s {
                [] => true,
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
                                ['i', s @ ..] => {
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
                        ['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
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
        ['l', 'u', 'c', 's', s @ ..] => {
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
        ['m', s @ ..] => super::t4::n1319(s, out),
        ['r', 'o', 's', s @ ..] => super::t4::n1320(s, out),
        ['u', s @ ..] => super::t4::n1321(s, out),
        _ => false,
    }
}

/// Words ending -rtus.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.86767375), (7, 0.07350271), (12, 0.044052795), (128, 0.01466392)], 0.9998932);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
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
                ['c', s @ ..] => {
                    out.push(&[(7, 0.81469446), (0, 0.17354086), (12, 0.008804455), (128, 0.0029297322)], 0.9999695);
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
                    out.push(&[(0, 0.6821241), (12, 0.15722896), (128, 0.15497063), (7, 0.0056610974)], 0.99998474);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            out.push(&[(12, 0.7190814), (0, 0.22737469), (128, 0.05165179)], 0.9981079);
                            match s {
                                [] => true,
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
                                ['o', s @ ..] => {
                                    out.push(&[(128, 0.71831846), (0, 0.22737469), (12, 0.05241474)], 0.9981079);
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
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['d', 'a', s @ ..] => {
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
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['d', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['e', s @ ..] => {
                            out.push(&[(0, 0.57354087), (7, 0.41469443), (12, 0.008804455), (128, 0.0029297322)], 0.9999695);
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
                        ['i', s @ ..] => {
                            out.push(&[(0, 0.69538414), (12, 0.29201192), (7, 0.010498207), (128, 0.002090486)], 0.99998474);
                            match s {
                                [] => true,
                                ['d', s @ ..] => {
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
                                ['m', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        ['o', s @ ..] => super::t4::n1324(s, out),
        _ => false,
    }
}

/// Words ending -stus.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.79771113), (7, 0.18994431), (15, 0.012298772)], 0.9999542);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(7, 0.72997636), (0, 0.2659037), (15, 0.004104677)], 0.99998474);
                    s.is_empty()
                }
                ['f', s @ ..] => {
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
                ['p', s @ ..] => {
                    out.push(&[(0, 0.9812619), (7, 0.017593652)], 0.9988556);
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
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['v', s @ ..] => {
                    out.push(&[(7, 0.72997636), (0, 0.2659037), (15, 0.004104677)], 0.99998474);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(0, 0.721355), (7, 0.27847716)], 0.99983215);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
                            out.push(&[(7, 0.7594873), (0, 0.24045166)], 0.99993896);
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
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
                ['c', 'n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['d', 'o', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
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
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            out.push(&[(7, 0.7594873), (0, 0.24045166)], 0.99993896);
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            out.push(&[(7, 0.7594873), (0, 0.24045166)], 0.99993896);
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
                        _ => false,
                    }
                }
                ['g', s @ ..] => {
                    out.push(&[(0, 0.9870451), (7, 0.012954909)], 1.0);
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
                    out.push(&[(7, 0.697612), (0, 0.30237278)], 0.99998474);
                    match s {
                        [] => true,
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['o', 'm', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['r', s @ ..] => {
                                    out.push(&[(0, 0.7674525), (7, 0.23253223)], 0.99998474);
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
                ['m', s @ ..] => {
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
                    out.push(&[(7, 0.89692533), (0, 0.103044175)], 0.9999695);
                    match s {
                        [] => true,
                        ['o', 'h', s @ ..] => {
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
                        ['u', 'f', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'm', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'q', s @ ..] => {
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
        ['i', 'm', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
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
        ['o', s @ ..] => {
            out.push(&[(0, 0.9844358), (7, 0.014618143)], 0.99905396);
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
        ['p', 'e', 'd', s @ ..] => {
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
        ['u', s @ ..] => super::t4::n1330(s, out),
        _ => false,
    }
}

/// Words ending -utus.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.90020597), (7, 0.047211412), (15, 0.020233463), (12, 0.012817578), (73, 0.006729229), (378, 0.006729229), (259, 0.006057832)], 0.99998474);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.6064698), (7, 0.2611124), (15, 0.13034256)], 0.99792475);
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            out.push(&[(15, 0.7101091), (0, 0.20215152), (7, 0.08703746)], 0.9992981);
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
                ['f', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
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
        ['b', s @ ..] => {
            out.push(&[(0, 0.9952697), (7, 0.0022430762)], 0.9975128);
            match s {
                [] => true,
                ['i', 'r', 't', s @ ..] => {
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
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'i', 'd', '\u{1}', s @ ..] => {
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
        ['c', s @ ..] => {
            out.push(&[(0, 0.96217287), (7, 0.036881056)], 0.9990539);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(0, 0.7735714), (7, 0.22632182)], 0.9998932);
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
                ['e', 's', s @ ..] => {
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
                ['o', 'l', s @ ..] => {
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
        ['d', 'n', 'i', s @ ..] => {
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
        ['g', s @ ..] => {
            out.push(&[(0, 0.76668954), (7, 0.22746624), (15, 0.0022430762)], 0.99639887);
            match s {
                [] => true,
                ['r', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['d', 'e', 'r', '\u{1}', s @ ..] => {
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
        ['l', s @ ..] => {
            out.push(&[(0, 0.88084227), (12, 0.057511255), (7, 0.03088426), (378, 0.0302739)], 0.99951166);
            match s {
                [] => true,
                ['i', s @ ..] => {
                    out.push(&[(7, 0.6769665), (0, 0.2936141), (12, 0.019165331), (378, 0.010086213)], 0.9998321);
                    s.is_empty()
                }
                ['l', 'o', 'p', s @ ..] => {
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
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['s', s @ ..] => super::t4::n8224(s, out),
                        ['v', s @ ..] => super::t4::n8225(s, out),
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['n', s @ ..] => super::t4::n1338(s, out),
        ['p', s @ ..] => super::t4::n1339(s, out),
        ['r', s @ ..] => super::t4::n1340(s, out),
        ['s', s @ ..] => super::t4::n1341(s, out),
        ['t', s @ ..] => super::t4::n1342(s, out),
        ['u', 'q', 'e', 's', s @ ..] => {
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

/// Words ending -xtus.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 't', s @ ..] => {
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
                ['t', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'm', s @ ..] => {
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
                ['n', 'i', '\u{1}', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -mortuus.
pub(crate) fn n1364(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
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
        _ => false,
    }
}

/// The word mutuus.
pub(crate) fn n3746(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word promutuus.
pub(crate) fn n3747(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ativus.
pub(crate) fn n3762(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', 'u', 'i', s @ ..] => {
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
                        ['n', 'u', 'm', 'm', 'o', 'c', s @ ..] => {
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
                ['o', 'v', s @ ..] => {
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
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['e', 's', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['r', 'o', 'c', 'e', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', 'e', 'n', s @ ..] => {
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
        ['l', s @ ..] => {
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
                ['l', 'i', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 's', '\u{1}', s @ ..] => {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'o', 'i', 't', 'a', 'r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['g', 'a', 'm', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', 'o', 'c', '\u{1}', s @ ..] => {
                                    s.is_empty()
                                }
                                ['o', 'n', s @ ..] => {
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
                                ['r', 'e', 'g', '\u{1}', s @ ..] => {
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
                        ['b', 'u', 'g', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'l', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['r', 'e', 'p', 'o', s @ ..] => {
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
                ['u', 'c', 'c', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -finitivus.
pub(crate) fn n8306(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -sitivus.
pub(crate) fn n8307(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['n', 'a', 'r', 't', s @ ..] => {
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
                ['s', 'i', 'd', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word optivus.
pub(crate) fn n8308(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word adoptivus.
pub(crate) fn n8309(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -tempestivus.
pub(crate) fn n3766(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -vivus.
pub(crate) fn n1373(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['d', 'e', 'r', '\u{1}', s @ ..] => {
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

/// Words ending -lvus.
pub(crate) fn n445(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', s @ ..] => {
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
        ['i', 'g', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'b', 'l', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ovus.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.6776989), (0, 0.3168078), (15, 0.0029449912)], 0.9974517);
    s.is_empty()
}

/// Words ending -rvus.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.8071565), (15, 0.09565881), (283, 0.0509041), (7, 0.02697795), (284, 0.012695506), (285, 0.006347753)], 0.9997406);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(283, 0.53333336), (7, 0.26666668), (284, 0.13333334), (285, 0.06666667)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
        ['e', s @ ..] => {
            out.push(&[(0, 0.5614252), (15, 0.41913483), (283, 0.010177768), (7, 0.0054016938), (284, 0.0025329976)], 0.9986725);
            match s {
                [] => true,
                ['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -luxus.
pub(crate) fn n1387(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
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
        _ => false,
    }
}

/// Words ending -bilis.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.61268026), (9, 0.30634013), (31, 0.036896314), (32, 0.018448157), (29, 0.009704738), (30, 0.004852369), (160, 0.0036926833)], 0.9926146);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(8, 0.6100099), (9, 0.30499732), (31, 0.043152515), (32, 0.021576257), (29, 0.0054016938), (30, 0.0027008469), (160, 0.0027008469), (137, 0.0026855879), (168, 0.0026855879)], 0.9959106);
            match s {
                [] => true,
                ['b', s @ ..] => {
                    out.push(&[(31, 0.45882353), (32, 0.2294194), (8, 0.20334172), (9, 0.10167086)], 0.9932555);
                    s.is_empty()
                }
                ['c', s @ ..] => super::t4::n4007(s, out),
                ['d', s @ ..] => super::t4::n4008(s, out),
                ['e', 'm', s @ ..] => super::t4::n4009(s, out),
                ['f', 'f', 'e', s @ ..] => super::t4::n4010(s, out),
                ['g', 'i', s @ ..] => super::t4::n4011(s, out),
                ['h', s @ ..] => super::t4::n4012(s, out),
                ['i', s @ ..] => super::t4::n4013(s, out),
                ['l', s @ ..] => super::t4::n4014(s, out),
                ['m', s @ ..] => super::t4::n4015(s, out),
                ['n', s @ ..] => super::t4::n4016(s, out),
                ['p', s @ ..] => super::t4::n4017(s, out),
                ['r', s @ ..] => super::t4::n4018(s, out),
                ['s', 'u', s @ ..] => super::t4::n4019(s, out),
                ['t', s @ ..] => super::t4::n4020(s, out),
                ['u', 'q', 'e', 'a', s @ ..] => super::t4::n4021(s, out),
                ['v', 'r', 'e', 's', s @ ..] => super::t4::n4022(s, out),
                _ => false,
            }
        }
        ['e', s @ ..] => super::t4::n1596(s, out),
        ['i', s @ ..] => super::t4::n1597(s, out),
        ['o', s @ ..] => super::t4::n1598(s, out),
        ['u', s @ ..] => super::t4::n1599(s, out),
        _ => false,
    }
}

/// Words ending -vilis.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.41808194), (29, 0.24411383), (9, 0.20903334), (30, 0.12199588), (31, 0.00280766)], 0.99603266);
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(29, 0.6666667), (30, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(29, 0.52581066), (30, 0.26288244), (8, 0.13936065), (9, 0.0696727)], 0.99772644);
            match s {
                [] => true,
                ['c', '\u{1}', s @ ..] => {
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
                ['b', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'e', 's', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -exilis.
pub(crate) fn n627(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(31, 0.6666667), (32, 0.33333334)], 1.0);
            s.is_empty()
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
                        ['n', 'i', '\u{1}', s @ ..] => {
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
        ['n', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -dicans.
pub(crate) fn n1715(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['u', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -ificans.
pub(crate) fn n1716(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['o', 'l', 'g', s @ ..] => {
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
                ['t', 'i', 'n', 'e', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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

/// Words ending -plicans.
pub(crate) fn n1718(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word dimicans.
pub(crate) fn n8692(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word stellimicans.
pub(crate) fn n8693(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
    s.is_empty()
}

/// The word inimicans.
pub(crate) fn n8694(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word formicans.
pub(crate) fn n4221(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -ricans.
pub(crate) fn n1720(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['g', 'i', 'n', '\u{1}', s @ ..] => {
            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
            s.is_empty()
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

/// The word flavicans.
pub(crate) fn n1721(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
    s.is_empty()
}

/// Words ending -lcans.
pub(crate) fn n683(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -runcans.
pub(crate) fn n684(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ocans.
pub(crate) fn n685(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -rcans.
pub(crate) fn n686(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -scans.
pub(crate) fn n687(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -manducans.
pub(crate) fn n688(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -odans.
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['i', 'l', 'u', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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

/// Words ending -tardans.
pub(crate) fn n1758(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -cordans.
pub(crate) fn n1759(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'r', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['n', s @ ..] => {
            out.push(&[(49, 0.48509955), (50, 0.2425574), (1, 0.18039216), (2, 0.091569394)], 0.99961853);
            s.is_empty()
        }
        ['s', 'i', 'd', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -udans.
pub(crate) fn n697(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word transnavigans.
pub(crate) fn n4343(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -lgans.
pub(crate) fn n714(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -rogans.
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['d', s @ ..] => {
            out.push(&[(1, 0.43103686), (16, 0.22653544), (2, 0.22403295), (17, 0.11839475)], 1.0);
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    out.push(&[(16, 0.6666667), (17, 0.33333334)], 1.0);
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
            out.push(&[(1, 0.5749294), (2, 0.28842604), (17, 0.09105058), (16, 0.04559396)], 1.0);
            match s {
                [] => true,
                ['a', '\u{1}', s @ ..] => {
                    out.push(&[(1, 0.35088122), (17, 0.31578547), (2, 0.17543298), (16, 0.15790036)], 1.0);
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

/// Words ending -urgans.
pub(crate) fn n716(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ugans.
pub(crate) fn n717(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['e', 'd', '\u{1}', s @ ..] => {
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

/// Words ending -stillans.
pub(crate) fn n1859(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -olans.
pub(crate) fn n734(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['i', 'r', 'a', s @ ..] => {
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
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['t', 'n', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 't', 'l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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

/// Words ending -ulans.
pub(crate) fn n735(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', 'f', s @ ..] => {
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
                        ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', 'i', '\u{1}', s @ ..] => {
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
                ['a', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['a', 'e', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'o', 'c', '\u{1}', s @ ..] => {
                            s.is_empty()
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
                        ['e', 'd', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', 'e', 'p', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'n', 'r', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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
        ['d', 'o', 'm', s @ ..] => {
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
        ['r', 'e', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['t', s @ ..] => {
            out.push(&[(1, 0.6055543), (2, 0.3031815), (17, 0.060746167), (16, 0.03041123)], 0.9998932);
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
                ['e', s @ ..] => {
                    out.push(&[(17, 0.46469826), (16, 0.23236439), (1, 0.20184635), (2, 0.1010605)], 0.9999695);
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

/// The word conformans.
pub(crate) fn n8867(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word transformans.
pub(crate) fn n4552(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -plasmans.
pub(crate) fn n742(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -spumans.
pub(crate) fn n743(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -inans.
pub(crate) fn n749(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.65145344), (2, 0.3394217), (333, 0.0060730907), (334, 0.0030365454)], 0.99998474);
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['o', 'i', 't', 'a', 'r', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['u', 'l', 'a', s @ ..] => {
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
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', 'i', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'r', 'e', s @ ..] => {
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
                ['u', s @ ..] => super::t4::n4607(s, out),
                _ => false,
            }
        }
        ['p', s @ ..] => super::t4::n1919(s, out),
        ['t', s @ ..] => super::t4::n1920(s, out),
        ['u', 'q', 'n', 'i', s @ ..] => super::t4::n1921(s, out),
        _ => false,
    }
}

/// The word damnans.
pub(crate) fn n1922(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word praedamnans.
pub(crate) fn n1923(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -nnans.
pub(crate) fn n751(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
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
        ['i', 'c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -onans.
pub(crate) fn n752(s: &[char], out: &mut Path<'_>) -> bool {
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
        ['n', 'n', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['r', 'p', s @ ..] => {
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
        ['s', s @ ..] => {
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
                ['i', 'n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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
                        ['d', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 't', 'l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                    s.is_empty()
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

/// Words ending -rnans.
pub(crate) fn n753(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', 'n', 'i', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['b', 'u', 'g', s @ ..] => {
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
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'a', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -unans.
pub(crate) fn n754(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['d', 'a', s @ ..] => {
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
        ['l', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -erans.
pub(crate) fn n765(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.64351875), (2, 0.32657358), (17, 0.0148165105), (16, 0.0074158846), (148, 0.0051117726), (149, 0.0025482567)], 0.99998474);
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
            out.push(&[(1, 0.5868772), (2, 0.2937209), (148, 0.07873655), (149, 0.039368276)], 0.99870294);
            match s {
                [] => true,
                ['i', 's', 'e', s @ ..] => {
                    out.push(&[(148, 0.47069505), (149, 0.2353399), (1, 0.19562066), (2, 0.09790189)], 0.9995575);
                    s.is_empty()
                }
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
        ['g', s @ ..] => {
            match s {
                [] => true,
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
        ['l', s @ ..] => {
            out.push(&[(1, 0.566186), (2, 0.2834516), (17, 0.0998703), (16, 0.04992752)], 0.9994354);
            match s {
                [] => true,
                ['e', 'c', s @ ..] => {
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
                ['o', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            out.push(&[(17, 0.47772947), (16, 0.23886473), (1, 0.18872358), (2, 0.09448387)], 0.99980164);
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
                        ['b', 'a', '\u{1}', s @ ..] => {
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
                ['y', 'l', 'o', 'p', 'e', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                    s.is_empty()
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
                ['g', 'i', 'p', s @ ..] => super::t4::n4703(s, out),
                ['l', 'u', 'v', s @ ..] => super::t4::n4704(s, out),
                ['o', s @ ..] => super::t4::n4705(s, out),
                ['u', 'm', s @ ..] => super::t4::n4706(s, out),
                _ => false,
            }
        }
        ['p', s @ ..] => super::t4::n1973(s, out),
        ['s', s @ ..] => super::t4::n1974(s, out),
        ['t', s @ ..] => super::t4::n1975(s, out),
        ['v', s @ ..] => super::t4::n1976(s, out),
        _ => false,
    }
}

/// The word suspirans.
pub(crate) fn n4739(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word exspirans.
pub(crate) fn n4740(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -orans.
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
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
                        ['d', 'a', '\u{1}', s @ ..] => {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'e', 'l', 'a', 'h', 't', 'h', 'p', 'a', 'n', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'r', 'e', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'd', 'i', 'c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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

/// Words ending -prans.
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.56769663), (2, 0.28625926), (144, 0.095231555), (145, 0.047623407)], 0.99681085);
    match s {
        [] => true,
        ['e', 'd', '\u{1}', s @ ..] => {
            out.push(&[(144, 0.6666667), (145, 0.33333334)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rrans.
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['d', 'a', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.5087663), (1, 0.4912337)], 1.0);
                    s.is_empty()
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

/// Words ending -trans.
pub(crate) fn n771(s: &[char], _out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'l', s @ ..] => {
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
                ['n', 'o', 'm', s @ ..] => {
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
                ['u', 'l', s @ ..] => {
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
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -urans.
pub(crate) fn n772(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'l', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'r', 'e', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'i', '\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['o', 'i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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
        ['t', 'a', s @ ..] => {
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
        _ => false,
    }
}

/// Words ending -gyrans.
pub(crate) fn n773(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word inspectans.
pub(crate) fn n18986(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word adinspectans.
pub(crate) fn n18987(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word prospectans.
pub(crate) fn n14200(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word perspectans.
pub(crate) fn n14201(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// The word exspectans.
pub(crate) fn n14202(s: &[char], _out: &mut Path<'_>) -> bool {
    s.is_empty()
}

/// Words ending -expectans.
pub(crate) fn n9132(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -sectans.
pub(crate) fn n4856(s: &[char], _out: &mut Path<'_>) -> bool {
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
        ['r', 'e', 'p', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        ['s', 'a', '\u{1}', s @ ..] => {
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -vectans.
pub(crate) fn n4857(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -dictans.
pub(crate) fn n2039(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -nctans.
pub(crate) fn n2040(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(16, 0.44528878), (17, 0.22328527), (1, 0.2185092), (2, 0.112245366)], 0.9993286);
    s.is_empty()
}

/// Words ending -arctans.
pub(crate) fn n2041(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -uctans.
pub(crate) fn n2042(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -etans.
pub(crate) fn n782(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -itans.
pub(crate) fn n783(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'x', 'e', '\u{1}', s @ ..] => {
                            s.is_empty()
                        }
                        ['i', 'c', '\u{1}', s @ ..] => {
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
                        ['n', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    s.is_empty()
                }
                ['a', 'b', '\u{1}', s @ ..] => {
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
                                ['o', 'n', '\u{1}', s @ ..] => super::t4::n14233(s, out),
                                _ => false,
                            }
                        }
                        ['m', '\u{1}', s @ ..] => super::t4::n9181(s, out),
                        _ => false,
                    }
                }
                ['l', 'e', 'p', 'p', 'a', '\u{1}', s @ ..] => super::t4::n4896(s, out),
                ['o', 'v', '\u{1}', s @ ..] => super::t4::n4897(s, out),
                _ => false,
            }
        }
        ['m', s @ ..] => super::t4::n2052(s, out),
        ['p', s @ ..] => super::t4::n2053(s, out),
        ['s', 'u', s @ ..] => super::t4::n2054(s, out),
        ['t', 'c', 'a', s @ ..] => super::t4::n2055(s, out),
        ['u', 'q', 'e', s @ ..] => super::t4::n2056(s, out),
        ['v', s @ ..] => super::t4::n2057(s, out),
        _ => false,
    }
}

/// Words ending -exaltans.
pub(crate) fn n784(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ntans.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
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
                ['m', s @ ..] => {
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
                                _ => false,
                            }
                        }
                        ['r', 'e', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.6666667), (1, 0.33333334)], 1.0);
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

/// Words ending -otans.
pub(crate) fn n786(s: &[char], _out: &mut Path<'_>) -> bool {
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
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
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

/// Words ending -ptans.
pub(crate) fn n787(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -rtans.
pub(crate) fn n788(s: &[char], _out: &mut Path<'_>) -> bool {
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

