use flexion::Path;

/// Words ending -ola (3 syllables).
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9505455), (2, 0.023300527), (9, 0.009719997), (3, 0.0030518044), (139, 0.0022735943), (146, 0.0022735943), (154, 0.0022735943), (218, 0.0022735943), (26, 0.002044709)], 0.9977569);
    match s {
        [] => true,
        ['c', s @ ..] => {
            out.push(&[(6, 0.9555505), (9, 0.028961623), (3, 0.009643702), (4, 0.004776074)], 0.9989319);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'i', 'f', 'i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'z', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'r', 'g', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['a', 's', 'a', 'i', 'c', 'n', 'a', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'a', 'm', s @ ..] => {
            out.push(&[(6, 0.6618753), (9, 0.31367972), (2, 0.014343481)], 0.9898985);
            match s {
                [] => true,
                ['i', 'r', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            out.push(&[(6, 0.9626154), (3, 0.022888534), (4, 0.0113374535)], 0.9968414);
            match s {
                [] => true,
                ['o', 'i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            out.push(&[(6, 0.8574655), (2, 0.041443504), (154, 0.03978027), (218, 0.03978027), (9, 0.010666057), (15, 0.009903105)], 0.9990387);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', 'a', s @ ..] => {
                            out.push(&[(6, 0.4035096), (2, 0.25479513), (218, 0.25401694), (15, 0.06347753), (154, 0.01872282), (9, 0.0050202184)], 0.99954224);
                            match s {
                                [] => true,
                                ['l', 'a', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(218, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['s', 's', 'a', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', 'u', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'h', '\u{1}', s @ ..] => {
                    out.push(&[(154, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', 'n', 'e', s @ ..] => {
            match s {
                [] => true,
                ['p', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', 'o', 'b', 'r', 'u', 't', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'c', s @ ..] => {
            out.push(&[(6, 0.6501869), (2, 0.17444114), (139, 0.16742198), (9, 0.0032349126)], 0.9952849);
            match s {
                [] => true,
                ['s', 'o', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(139, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['v', s @ ..] => {
            out.push(&[(6, 0.84240484), (2, 0.10807965), (146, 0.04727245)], 0.99775696);
            match s {
                [] => true,
                ['a', 't', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['p', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(146, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['l', 'a', 'v', 'a', 'z', 'l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['z', 'n', 'o', 'g', 'r', 'o', 'g', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ula (3 syllables).
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9307698), (159, 0.026932174), (69, 0.013473717), (246, 0.013473717), (3, 0.0090638595), (4, 0.004562448)], 0.9982757);
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(6, 0.7146868), (159, 0.18670939), (3, 0.06225681), (4, 0.031128405), (69, 0.0024414435), (246, 0.0024414435)], 0.9996643);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', '\u{1}', s @ ..] => {
                            out.push(&[(159, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['t', '\u{1}', s @ ..] => {
                            out.push(&[(159, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['c', 'i', 'd', 'n', 'e', 'f', 'f', 'o', '\u{1}', s @ ..] => {
            out.push(&[(246, 1.0)], 1.0);
            s.is_empty()
        }
        ['l', 'l', 'e', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(69, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ma (3 syllables).
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(9, 0.70220494), (6, 0.24228275), (2, 0.033371482), (0, 0.0033722438)], 0.9812314);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(6, 0.3770962), (9, 0.3735256), (2, 0.19185168), (154, 0.04444953), (42, 0.011108568)], 0.99803156);
            match s {
                [] => true,
                ['d', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(42, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 't', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['g', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['l', 'o', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['l', 'a', 'm', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['r', 'e', 'n', 'a', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 't', 'i', 'r', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['i', 'g', 'i', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['k', 'o', 'k', '\u{1}', s @ ..] => {
                    out.push(&[(154, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'i', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', 'd', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['m', 's', 'o', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['n', 'a', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', 'o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'p', 'a', 'r', 'i', 'u', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['z', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(9, 0.84029907), (6, 0.10652323), (2, 0.02825971), (225, 0.012191959), (3, 0.008133058), (4, 0.0040589)], 0.99946594);
            match s {
                [] => true,
                ['d', 'a', 'n', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'a', 'l', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', s @ ..] => {
                            out.push(&[(2, 0.5427024), (9, 0.39543754), (6, 0.050125886), (225, 0.005737392), (3, 0.0038300145)], 0.9978332);
                            s.is_empty()
                        }
                        ['o', 'r', 'b', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'o', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'i', 'v', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'r', 'e', 'o', 'l', 'a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(225, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'o', s @ ..] => {
                            out.push(&[(9, 0.4201419), (6, 0.3032578), (3, 0.1707332), (4, 0.08535897), (2, 0.014129855), (225, 0.0061036088)], 0.99972534);
                            match s {
                                [] => true,
                                ['p', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
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
        ['g', s @ ..] => {
            out.push(&[(9, 0.95745784), (6, 0.034607463), (2, 0.004760815)], 0.9968261);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(6, 0.73023576), (9, 0.24200809), (2, 0.014023041), (0, 0.013244831)], 0.9995117);
            match s {
                [] => true,
                ['h', s @ ..] => {
                    out.push(&[(9, 0.7834287), (6, 0.20863661), (2, 0.004013123), (0, 0.0037842374)], 0.9998627);
                    s.is_empty()
                }
                ['l', 'c', s @ ..] => {
                    out.push(&[(9, 0.7473411), (6, 0.24341191), (2, 0.0046692607), (0, 0.0044098576)], 0.99983215);
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    out.push(&[(6, 0.88534373), (2, 0.046417944), (0, 0.04634165), (9, 0.02185092)], 0.9999542);
                    match s {
                        [] => true,
                        ['s', 'i', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', 'r', 'o', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(0, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 'n', 'a', 's', 'a', 'm', 'm', 'a', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['z', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', s @ ..] => {
                            out.push(&[(9, 0.68036926), (6, 0.31459525), (2, 0.0025482567), (0, 0.0024109255)], 0.9999237);
                            match s {
                                [] => true,
                                ['e', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
                                            out.push(&[(6, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['o', 'c', 'l', 'i', 't', 'e', 'c', 'a', '\u{1}', s @ ..] => {
                                            out.push(&[(6, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['o', 'b', 'i', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            out.push(&[(9, 0.8350805), (6, 0.10321202), (2, 0.04476997), (20, 0.0073853666), (175, 0.0073853666)], 0.99783325);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['g', 'i', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            out.push(&[(2, 0.4261387), (9, 0.36830702), (6, 0.20088503), (20, 0.00202945), (175, 0.00202945)], 0.99938965);
                            match s {
                                [] => true,
                                ['f', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['a', 'i', 'r', 'o', '\u{1}', s @ ..] => {
                                            out.push(&[(6, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['i', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['g', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
                                                    out.push(&[(9, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                ['r', 'o', '\u{1}', s @ ..] => {
                                                    out.push(&[(6, 1.0)], 1.0);
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
                        ['m', s @ ..] => super::t1::n2232(s, out),
                        ['r', s @ ..] => super::t1::n2233(s, out),
                        _ => false,
                    }
                }
                ['e', s @ ..] => super::t1::n1124(s, out),
                ['i', 'z', 'z', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 's', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => super::t1::n439(s, out),
        ['r', s @ ..] => super::t1::n440(s, out),
        ['s', s @ ..] => super::t1::n441(s, out),
        ['u', s @ ..] => super::t1::n442(s, out),
        ['è', 'n', 'i', 'c', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => super::t1::n443(s, out),
        ['ì', 'r', 'c', 's', 'i', 'a', '\u{1}', s @ ..] => super::t1::n444(s, out),
        _ => false,
    }
}

/// The word malaguegna (3 syllables).
pub(crate) fn n2283(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -ogna (3 syllables).
pub(crate) fn n1166(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['g', 'r', 'o', 'b', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', 'o', 'c', 'a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
            out.push(&[(226, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ina (3 syllables).
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9799039), (0, 0.007110704), (9, 0.0056000613), (2, 0.003524834)], 0.99613947);
    match s {
        [] => true,
        ['a', 'c', 'o', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['c', s @ ..] => {
            out.push(&[(6, 0.9576257), (0, 0.019897765), (89, 0.009872587), (99, 0.009872587), (120, 0.0024719615)], 0.9997406);
            match s {
                [] => true,
                ['a', 'r', 't', 'i', 'c', 'a', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['e', 'd', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(89, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'f', 'f', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(120, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(99, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'c', 'o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            out.push(&[(6, 0.9859922), (134, 0.010849165)], 0.9968414);
            match s {
                [] => true,
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(134, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['e', 't', 'o', 'r', 'p', 'o', 'c', 'u', 'm', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['f', 'r', 'o', 'm', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['g', s @ ..] => {
            out.push(&[(6, 0.87699705), (2, 0.07502861), (121, 0.037384603), (0, 0.009872587)], 0.99928284);
            match s {
                [] => true,
                ['a', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(121, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'r', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'a', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['h', 'c', s @ ..] => {
            match s {
                [] => true,
                ['c', 'a', 'm', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'r', 'r', 'e', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', 'm', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'a', 'i', 'a', 'm', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'r', 'i', 't', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
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
                ['a', 'm', s @ ..] => {
                    out.push(&[(6, 0.80401313), (2, 0.10171664), (0, 0.0923476)], 0.9980774);
                    match s {
                        [] => true,
                        ['a', 'i', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 'l', 'e', 'c', 'c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['r', 'a', 'l', 'i', 't', 'e', 'm', 'i', 'r', 't', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 'e', 'p', 'o', 'm', 'e', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(6, 0.9883879), (0, 0.0038757916), (9, 0.0038605325), (272, 0.0038147555)], 0.99993896);
            match s {
                [] => true,
                ['c', 'a', s @ ..] => {
                    out.push(&[(6, 0.7441978), (9, 0.25192645)], 0.99612427);
                    match s {
                        [] => true,
                        ['t', 'n', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'e', 'p', 'o', 'c', '-', 'a', 'z', 'z', 'a', 'g', 'a', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(272, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 'e', 'l', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['z', s @ ..] => {
            match s {
                [] => true,
                ['a', 'i', 't', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'r', 'o', 'l', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -nna (3 syllables).
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.7726253), (2, 0.16643015), (43, 0.030014496), (214, 0.024002442), (15, 0.005996796)], 0.9990692);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'a', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 'o', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['p', 'm', 'e', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 'o', 'g', 'n', 'u', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['d', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(43, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'i', 'r', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(43, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'd', 'n', 'e', 'p', 'p', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', '-', 'e', 'r', 'd', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(214, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ona (3 syllables).
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.98312354), (9, 0.011062791), (15, 0.005493248)], 0.99967957);
    match s {
        [] => true,
        ['r', 'd', 'n', 'a', 's', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', 'o', 'c', 'o', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['z', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(15, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word pincerna (3 syllables).
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(9, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -una (3 syllables).
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.84052795), (2, 0.10556191), (228, 0.05194171)], 0.99803156);
    match s {
        [] => true,
        ['l', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
            out.push(&[(228, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', 'r', 'o', 'f', s @ ..] => {
            match s {
                [] => true,
                ['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -tiera (3 syllables).
pub(crate) fn n2332(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', 'm', 'o', 'g', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['s', 'a', 't', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', 'e', 'h', 'c', 's', 'o', 'm', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -lera (3 syllables).
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['l', 'e', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'c', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nera (3 syllables).
pub(crate) fn n1222(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.655375), (0, 0.22333105), (2, 0.11755551)], 0.99626154);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['b', 'a', 'h', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -pera (3 syllables).
pub(crate) fn n1223(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.8712749), (47, 0.09565881), (94, 0.023804074), (2, 0.005523766)], 0.99626154);
    match s {
        [] => true,
        ['o', 'd', s @ ..] => {
            match s {
                [] => true,
                ['a', 'c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(47, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(94, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word portatessera (3 syllables).
pub(crate) fn n1224(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -ettera (3 syllables).
pub(crate) fn n1225(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.57967496), (2, 0.21161212), (110, 0.20088503), (9, 0.0028992142), (0, 0.002014191)], 0.9970855);
    match s {
        [] => true,
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['i', 'd', 'u', 'i', 'h', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(110, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ira (3 syllables).
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['p', 's', 'r', 'e', 'p', 'm', 'a', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['v', 'e', 'r', 't', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ora (3 syllables).
pub(crate) fn n485(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.8926833), (2, 0.08763256), (9, 0.019394217)], 0.9997101);
    match s {
        [] => true,
        ['n', 'e', 'm', 'u', 'l', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'e', s @ ..] => {
            out.push(&[(6, 0.44634166), (2, 0.29381245), (9, 0.2596933)], 0.9998474);
            match s {
                [] => true,
                ['p', 'm', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'l', 'o', 'v', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(6, 0.6252079), (2, 0.37279317)], 0.9980011);
            match s {
                [] => true,
                ['t', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['v', 'o', 'l', 'i', 'h', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['w', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['a', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['g', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['e', 'm', '\u{1}', s @ ..] => {
                                                    out.push(&[(2, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                ['i', 'g', '\u{1}', s @ ..] => {
                                                    out.push(&[(2, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        ['r', 'e', 't', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['o', 'l', 'i', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['h', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['k', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
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
        _ => false,
    }
}

/// Words ending -pra (3 syllables).
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.7841459), (175, 0.2), (2, 0.007553216), (9, 0.002105745)], 0.9938049);
    s.is_empty()
}

/// Words ending -rra (3 syllables).
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.72237736), (2, 0.27598992)], 0.9983673);
    match s {
        [] => true,
        ['e', s @ ..] => {
            out.push(&[(2, 0.6865492), (6, 0.3131609)], 0.9997101);
            match s {
                [] => true,
                ['c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', 'g', 'e', 't', 'n', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -tra (3 syllables).
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.6260624), (3, 0.16832227), (4, 0.0841535), (9, 0.07043564), (2, 0.015625238), (129, 0.015060655), (266, 0.015060655), (175, 0.0050202184)], 0.9997406);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(3, 0.43784237), (4, 0.21892118), (6, 0.21220721), (9, 0.11343557), (175, 0.013870451)], 0.9962768);
            match s {
                [] => true,
                ['b', 'l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'o', 'n', 'o', 'c', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['e', 'm', 'o', 'e', 'g', '\u{1}', s @ ..] => {
            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['s', s @ ..] => {
            out.push(&[(6, 0.82790875), (9, 0.06410315), (2, 0.030869002), (129, 0.030838484), (266, 0.030838484), (3, 0.010086213), (4, 0.005035477)], 0.99967957);
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', 'o', 'r', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['h', 'c', 'r', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(129, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', 'f', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(266, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'n', 'i', 's', 'o', 'r', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'm', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ura (3 syllables).
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9951171)], 0.9951171);
    match s {
        [] => true,
        ['d', s @ ..] => {
            out.push(&[(6, 0.9223316), (2, 0.07692073)], 0.9992523);
            match s {
                [] => true,
                ['r', 'e', 'v', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 'm', 'e', 't', '\u{1}', s @ ..] => {
            out.push(&[(15, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', s @ ..] => {
            out.push(&[(6, 0.99633783)], 0.99633783);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(6, 0.9988861)], 0.9988861);
                    match s {
                        [] => true,
                        ['i', s @ ..] => {
                            out.push(&[(6, 0.9951324), (9, 0.004867628)], 1.0);
                            match s {
                                [] => true,
                                ['b', 'b', 'e', 'd', 'd', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['z', 'z', 'a', 'p', 's', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
                    out.push(&[(6, 0.9914092), (0, 0.0085297935)], 0.99993896);
                    match s {
                        [] => true,
                        ['o', 'c', 'i', 'g', 'r', 'a', 'p', 's', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    out.push(&[(6, 0.94962996), (219, 0.050003815)], 0.9996338);
                    match s {
                        [] => true,
                        ['e', 'v', 'a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(219, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    out.push(&[(6, 0.97097737), (0, 0.014465553), (2, 0.014465553)], 0.99990845);
                    match s {
                        [] => true,
                        ['e', 'v', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'c', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
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

/// Words ending -vra (3 syllables).
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.7841459), (115, 0.2), (2, 0.007553216), (9, 0.002105745)], 0.9938049);
    match s {
        [] => true,
        ['o', 'n', 'a', 'm', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(115, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ta (3 syllables).
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.48752576), (3, 0.302327), (4, 0.15126269), (9, 0.03483635), (2, 0.012985428), (15, 0.0022278172)], 0.99116504);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(6, 0.9741512), (3, 0.007934691), (9, 0.007751583), (4, 0.0039673457)], 0.9938048);
            match s {
                [] => true,
                ['b', 'o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['d', s @ ..] => {
                    out.push(&[(6, 0.9620966), (80, 0.03604181)], 0.99813837);
                    match s {
                        [] => true,
                        ['r', 'o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(80, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['t', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['z', 'n', 'e', 'r', 'e', 'f', 'f', 'i', 'd', 'n', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'p', 'o', 'm', 'o', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['m', 'l', s @ ..] => {
                    out.push(&[(6, 0.60169375), (9, 0.20724803), (3, 0.12568857), (4, 0.062836654)], 0.997467);
                    match s {
                        [] => true,
                        ['a', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(9, 0.5263142), (3, 0.31578547), (4, 0.15790036)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    out.push(&[(6, 0.89567405), (3, 0.068925), (4, 0.03445487)], 0.9990539);
                    match s {
                        [] => true,
                        ['o', s @ ..] => {
                            out.push(&[(6, 0.46521705), (3, 0.3563134), (4, 0.17814907)], 0.99967957);
                            match s {
                                [] => true,
                                ['e', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['m', 'o', '\u{1}', s @ ..] => {
                                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                            s.is_empty()
                                        }
                                        ['t', 's', 'o', '\u{1}', s @ ..] => {
                                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['r', 'u', 't', 'a', 'n', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    out.push(&[(6, 0.9304341), (9, 0.02355993), (3, 0.01498436), (15, 0.011688411), (175, 0.011688411), (4, 0.00749218)], 0.9998474);
                    match s {
                        [] => true,
                        ['c', 'o', 't', 'n', 'o', 'r', 'e', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            out.push(&[(6, 0.6860914), (9, 0.20471504), (15, 0.10234226), (3, 0.0029907683), (175, 0.0023346303)], 0.9984741);
                            match s {
                                [] => true,
                                ['p', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(9, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['o', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
                                                    out.push(&[(9, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                ['r', 'g', 'a', '\u{1}', s @ ..] => {
                                                    out.push(&[(15, 1.0)], 1.0);
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
                ['s', s @ ..] => {
                    out.push(&[(6, 0.9715572), (2, 0.027038988)], 0.9985962);
                    match s {
                        [] => true,
                        ['o', 'p', 'i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', 's', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'r', 'o', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['z', s @ ..] => {
                    out.push(&[(6, 0.9421988), (9, 0.028610665), (0, 0.028168155)], 0.99897766);
                    match s {
                        [] => true,
                        ['n', 'a', 'z', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['z', 'a', 'p', 'm', 'i', '\u{1}', s @ ..] => {
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
            out.push(&[(6, 0.4732738), (3, 0.21828032), (9, 0.18207064), (4, 0.10914779), (164, 0.016449226)], 0.9992218);
            match s {
                [] => true,
                ['b', 'a', 'f', 'l', 'a', 'n', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['c', 's', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['d', 'o', 'e', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['f', 'o', 'r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['s', 'e', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', 'l', 'o', 'p', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
                    out.push(&[(3, 0.49295795), (4, 0.24647897), (6, 0.21628137), (9, 0.040466927), (164, 0.0036621653)], 0.9998474);
                    match s {
                        [] => true,
                        ['t', 'a', 't', 'p', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'i', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(9, 0.5263142), (6, 0.47368583)], 1.0);
                                    s.is_empty()
                                }
                                ['o', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['s', 'e', '\u{1}', s @ ..] => {
                                            out.push(&[(9, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                                            out.push(&[(9, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['e', 's', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', 'a', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(164, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 'a', 'r', 'a', 'm', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                            s.is_empty()
                                        }
                                        ['a', 'r', 't', 'l', 'u', '\u{1}', s @ ..] => {
                                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
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
                ['o', 'p', 'o', 's', 's', 'o', 'l', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['r', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'a', 'n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['o', 'n', '\u{1}', s @ ..] => {
                                            out.push(&[(9, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['s', 'e', 't', '\u{1}', s @ ..] => {
                                            out.push(&[(9, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', 't', 'o', 'l', 'g', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', 'e', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 't', 'a', 'n', 'a', 't', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['z', 'a', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => super::t1::n508(s, out),
        ['l', s @ ..] => {
            out.push(&[(6, 0.9254597), (3, 0.0439765), (4, 0.02200351), (9, 0.0050659953)], 0.9965057);
            s.is_empty()
        }
        ['n', s @ ..] => super::t1::n510(s, out),
        ['o', s @ ..] => {
            out.push(&[(3, 0.30501258), (6, 0.23933776), (9, 0.19519341), (4, 0.15251392), (2, 0.080689706), (268, 0.026672771)], 0.99942017);
            match s {
                [] => true,
                ['i', s @ ..] => {
                    out.push(&[(3, 0.43344778), (9, 0.30428016), (4, 0.21672389), (6, 0.03138781), (2, 0.010589762), (268, 0.003494316)], 0.9999237);
                    s.is_empty()
                }
                ['l', s @ ..] => super::t1::n1310(s, out),
                ['n', s @ ..] => super::t1::n1311(s, out),
                ['r', s @ ..] => super::t1::n1312(s, out),
                ['u', s @ ..] => super::t1::n1313(s, out),
                ['v', 'e', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', s @ ..] => super::t1::n512(s, out),
        ['s', s @ ..] => super::t1::n513(s, out),
        ['t', s @ ..] => super::t1::n514(s, out),
        ['u', s @ ..] => super::t1::n515(s, out),
        _ => false,
    }
}

/// Words ending -obile (3 syllables).
pub(crate) fn n2550(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.46797895), (7, 0.42987716), (170, 0.06236362), (11, 0.03962768)], 0.9998474);
    match s {
        [] => true,
        ['m', 'a', 's', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(170, 0.6666667), (7, 0.33333334)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word segnafile (3 syllables).
pub(crate) fn n1420(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -nile (3 syllables).
pub(crate) fn n1421(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.96095216), (11, 0.033142596), (7, 0.0042267493)], 0.99832153);
    s.is_empty()
}

/// Words ending -oile (3 syllables).
pub(crate) fn n1422(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.69018084), (15, 0.24174869), (11, 0.032043945), (7, 0.02839704), (2, 0.0049286643)], 0.9972992);
    match s {
        [] => true,
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['e', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['é', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word baciapile (3 syllables).
pub(crate) fn n1423(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(15, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -rile (3 syllables).
pub(crate) fn n1424(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.9381399), (2, 0.046112765), (11, 0.007553216), (7, 0.006698711)], 0.9985046);
    match s {
        [] => true,
        ['a', 'b', 'a', 'c', 'i', 'r', 'a', 'c', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word etilesile (3 syllables).
pub(crate) fn n1425(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -tile (3 syllables).
pub(crate) fn n1426(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.93786526), (7, 0.05618372), (11, 0.0043945983)], 0.9984436);
    s.is_empty()
}

/// Words ending -lle (3 syllables).
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.32046998), (0, 0.17721827), (15, 0.17279316), (8, 0.12677196), (7, 0.12150759), (11, 0.045181964), (198, 0.019302662), (252, 0.011032273), (253, 0.005508507)], 0.9997864);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(2, 0.452842), (15, 0.29088274), (0, 0.09373617), (8, 0.06251621), (7, 0.05168231), (198, 0.043183032), (11, 0.0037537194)], 0.9985962);
            match s {
                [] => true,
                ['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(8, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'a', 't', 'o', 'u', 'v', 's', '\u{1}', s @ ..] => {
                    out.push(&[(8, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'd', 'n', 'o', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(198, 1.0)], 1.0);
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
                ['b', 'i', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(11, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'l', 'l', 'i', 'v', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'u', s @ ..] => {
                            match s {
                                [] => true,
                                ['f', '\u{1}', s @ ..] => {
                                    out.push(&[(8, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', '\u{1}', s @ ..] => {
                                    out.push(&[(8, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    out.push(&[(7, 0.44731823), (2, 0.26082247), (15, 0.18559548), (0, 0.048889905), (8, 0.034973677), (11, 0.012466621), (198, 0.005325399), (252, 0.0030365454)], 0.99842834);
                    match s {
                        [] => true,
                        ['a', 't', 'u', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['l', 'i', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', 'i', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['n', 'i', 'v', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'e', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 'n', 'e', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
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
            out.push(&[(0, 0.47225147), (8, 0.282414), (11, 0.11183337), (2, 0.06573587), (15, 0.035446707), (7, 0.024917983), (198, 0.0039673457), (252, 0.0022583352)], 0.9988251);
            match s {
                [] => true,
                ['a', 'r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(8, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(8, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['c', 'e', 'b', 'm', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(11, 1.0)], 1.0);
                    s.is_empty()
                }
                ['h', 'c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(8, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'n', 'i', 'c', 's', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(8, 1.0)], 1.0);
                    s.is_empty()
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
                        ['a', 'p', 'p', 'a', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(252, 0.5714351), (253, 0.28570992), (11, 0.14285496)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(8, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ole (3 syllables).
pub(crate) fn n584(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.3639124), (7, 0.20547798), (2, 0.19819944), (15, 0.11548028), (11, 0.052384224), (0, 0.031616695), (23, 0.01637293), (206, 0.01637293)], 0.9998169);
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(7, 0.48206303), (8, 0.20523384), (15, 0.1345388), (11, 0.12074464), (2, 0.04332036), (0, 0.006912337), (23, 0.0035858701), (206, 0.0035858701)], 0.99998474);
            match s {
                [] => true,
                ['b', 'u', 'b', 'i', 'd', 'n', 'e', 'v', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            out.push(&[(8, 0.680766), (2, 0.23379873), (7, 0.04008545), (15, 0.022537576), (11, 0.010223545), (0, 0.006164645), (23, 0.0031891356), (206, 0.0031891356)], 0.9999542);
            match s {
                [] => true,
                ['c', 'a', 'i', 'f', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 'u', 'i', 'a', 'm', 'a', 's', 's', 'i', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', 'e', 'c', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['g', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
            out.push(&[(15, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['c', 'i', 'r', 'b', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'r', 'i', 'p', 's', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'l', 'g', 'o', 'c', 'c', 'a', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', 'g', 'e', 's', 'a', 'd', 'l', 'a', 'c', 's', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'l', 'i', 'p', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['m', '-', 'o', 'm', 'm', 'a', 'r', 'g', '\u{1}', s @ ..] => {
            out.push(&[(206, 1.0)], 1.0);
            s.is_empty()
        }
        ['p', 'a', 'u', 'q', 'a', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'e', 't', 'i', 'f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
            out.push(&[(23, 1.0)], 1.0);
            s.is_empty()
        }
        ['s', s @ ..] => {
            out.push(&[(8, 0.6278935), (2, 0.24434271), (0, 0.06929122), (7, 0.029571984), (15, 0.016617075), (11, 0.007537957), (23, 0.0023498894), (206, 0.0023498894)], 0.9999542);
            match s {
                [] => true,
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', 'n', 'e', 'r', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['g', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(7, 0.41431296), (15, 0.28719005), (2, 0.12411688), (0, 0.094514385), (8, 0.06469826), (11, 0.009308004), (23, 0.0029144732), (206, 0.0029144732)], 0.9999695);
            match s {
                [] => true,
                ['a', 'c', 's', 'i', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', 'm', 'o', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'p', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'e', 'p', 'a', 'c', 'c', 'e', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'i', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 's', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['a', 'r', 't', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['s', 'e', '\u{1}', s @ ..] => {
                                            out.push(&[(7, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['x', 'e', '\u{1}', s @ ..] => {
                                            out.push(&[(7, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['e', 'r', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['t', 'o', 'r', 'f', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'd', 'n', 'e', 'v', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
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
                ['a', 'f', 'a', 't', 'n', 'o', 'c', 'c', 'a', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['e', s @ ..] => {
                    out.push(&[(8, 0.52530706), (11, 0.23385976), (7, 0.1567712), (2, 0.044037536), (15, 0.025665674), (0, 0.00701915), (23, 0.0036469062), (206, 0.0036469062)], 0.99995416);
                    s.is_empty()
                }
                ['l', 'a', 'v', 'a', 'v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word infilaperle (3 syllables).
pub(crate) fn n585(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -ule (3 syllables).
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.6465553), (2, 0.23396659), (11, 0.090394445), (7, 0.022201877), (15, 0.0037994965), (0, 0.0022430762)], 0.99916077);
    match s {
        [] => true,
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['j', 'a', 'g', 'i', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', '-', 'e', 'd', '-', 'd', 'e', 'i', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', 'p', 'a', 'c', 'a', 'v', 'e', 'l', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ine (3 syllables).
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.7294423), (8, 0.16969559), (2, 0.060105287), (0, 0.022720683), (15, 0.009735256), (11, 0.005004959), (23, 0.0032501717)], 0.9999542);
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(7, 0.36472115), (8, 0.33484396), (0, 0.26135653), (2, 0.030060273), (15, 0.004867628), (11, 0.0025024796)], 0.99835205);
            match s {
                [] => true,
                ['m', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            out.push(&[(8, 0.7360952), (7, 0.2214847), (2, 0.03476005), (0, 0.004272526)], 0.9966125);
            match s {
                [] => true,
                ['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['r', s @ ..] => {
                    out.push(&[(8, 0.6949569), (2, 0.17668422), (7, 0.12156863), (0, 0.0037842374)], 0.99699396);
                    match s {
                        [] => true,
                        ['a', 'b', 'a', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'b', 'u', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', s @ ..] => {
                    out.push(&[(7, 0.9575036), (0, 0.02479591), (8, 0.013977264), (2, 0.002853437)], 0.99913025);
                    match s {
                        [] => true,
                        ['t', 'i', 'h', 'g', 'n', 'i', 'l', 'a', 's', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['f', s @ ..] => {
            out.push(&[(7, 0.291783), (8, 0.26787212), (0, 0.20909438), (11, 0.20199893), (2, 0.024048219), (15, 0.0038910506)], 0.9986877);
            match s {
                [] => true,
                ['r', 'e', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            out.push(&[(7, 0.92037845), (8, 0.06596475), (2, 0.00624094), (15, 0.005676356)], 0.9982605);
            match s {
                [] => true,
                ['a', 'p', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    out.push(&[(8, 0.41759366), (7, 0.4121004), (15, 0.13484398), (11, 0.033340964)], 0.997879);
                    match s {
                        [] => true,
                        ['a', 'm', 'a', 'r', 'e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['h', s @ ..] => {
            out.push(&[(2, 0.42404822), (7, 0.291783), (15, 0.20389105), (8, 0.06787213), (0, 0.009094377), (11, 0.0019989319)], 0.9986877);
            s.is_empty()
        }
        ['l', s @ ..] => {
            out.push(&[(8, 0.3342031), (0, 0.29221028), (7, 0.20840771), (2, 0.16003662), (15, 0.002777142)], 0.9976348);
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', 'i', 'u', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['p', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'a', 'g', 'a', 'b', 'u', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            out.push(&[(8, 0.49849698), (2, 0.39453727), (7, 0.061432824), (15, 0.04292363)], 0.9973907);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(2, 0.7378805), (8, 0.16302739), (15, 0.08863966), (7, 0.010025177)], 0.99957275);
                    match s {
                        [] => true,
                        ['l', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(8, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'e', 'f', 'a', 'p', 'u', 'i', 'c', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'a', 'm', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', s @ ..] => {
            out.push(&[(8, 0.37103838), (7, 0.2732128), (0, 0.22726788), (2, 0.12446784), (15, 0.002166781)], 0.9981537);
            match s {
                [] => true,
                ['t', 'a', 'l', 'a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', 'o', 'r', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['b', 'u', 's', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['u', s @ ..] => {
            out.push(&[(8, 0.46787214), (7, 0.291783), (2, 0.22404821), (0, 0.009094377), (15, 0.0038910506), (11, 0.0019989319)], 0.99868774);
            match s {
                [] => true,
                ['g', 'n', 'i', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['z', s @ ..] => {
            out.push(&[(7, 0.52581066), (23, 0.30969712), (8, 0.10443275), (2, 0.03698787), (0, 0.013977264), (15, 0.005996796), (11, 0.0030823224)], 0.9999848);
            match s {
                [] => true,
                ['a', 'g', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(23, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -penne (3 syllables).
pub(crate) fn n2620(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.68607616), (11, 0.29005876), (7, 0.016449226), (0, 0.00483711), (8, 0.0025329976)], 0.9999543);
    s.is_empty()
}

/// Words ending -one (3 syllables).
pub(crate) fn n609(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.7378195), (8, 0.25143817), (11, 0.006256199), (2, 0.002151522)], 0.9976654);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(8, 0.75048447), (7, 0.24594492), (11, 0.002090486)], 0.9985199);
            s.is_empty()
        }
        ['b', s @ ..] => {
            out.push(&[(8, 0.89411765), (7, 0.089433126), (162, 0.0151522085)], 0.998703);
            match s {
                [] => true,
                ['r', 'a', 'c', 'a', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(162, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            out.push(&[(8, 0.93794155), (7, 0.035629816), (11, 0.026306555)], 0.9998779);
            s.is_empty()
        }
        ['d', s @ ..] => {
            out.push(&[(8, 0.83418024), (7, 0.103273064), (11, 0.062394142)], 0.9998474);
            match s {
                [] => true,
                ['e', 'l', s @ ..] => {
                    out.push(&[(7, 0.534432), (8, 0.4447242), (11, 0.020798046)], 0.9999542);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(8, 0.8639048), (7, 0.13414206)], 0.9980468);
            s.is_empty()
        }
        ['f', s @ ..] => {
            out.push(&[(8, 0.9362936), (7, 0.06279088)], 0.9990845);
            s.is_empty()
        }
        ['g', s @ ..] => {
            out.push(&[(8, 0.8561227), (7, 0.10011444), (11, 0.043564506)], 0.99980164);
            s.is_empty()
        }
        ['h', s @ ..] => {
            out.push(&[(2, 0.50107574), (7, 0.36890212), (8, 0.12571909), (11, 0.0031280995)], 0.9988251);
            match s {
                [] => true,
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'r', 'a', 'm', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(7, 0.9196612), (8, 0.07692073)], 0.996582);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(8, 0.74535745), (7, 0.2536965)], 0.99905396);
                    s.is_empty()
                }
                ['b', s @ ..] => {
                    out.push(&[(8, 0.6442359), (7, 0.3549096)], 0.9991455);
                    s.is_empty()
                }
                ['c', s @ ..] => {
                    out.push(&[(8, 0.9459373), (7, 0.049820706), (0, 0.004150454)], 0.99990845);
                    s.is_empty()
                }
                ['d', s @ ..] => {
                    out.push(&[(8, 0.6003815), (7, 0.39896238)], 0.9993439);
                    s.is_empty()
                }
                ['f', s @ ..] => {
                    out.push(&[(8, 0.7264973), (7, 0.27249563)], 0.9989929);
                    s.is_empty()
                }
                ['g', s @ ..] => {
                    out.push(&[(7, 0.73638517), (8, 0.26216525)], 0.9985504);
                    match s {
                        [] => true,
                        ['g', s @ ..] => {
                            out.push(&[(8, 0.81554896), (7, 0.1841001)], 0.99964905);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['h', s @ ..] => {
                    out.push(&[(8, 0.94406044), (7, 0.055741206)], 0.99980164);
                    s.is_empty()
                }
                ['l', s @ ..] => {
                    out.push(&[(8, 0.87995726), (7, 0.10357824), (11, 0.016342413)], 0.9998779);
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    out.push(&[(8, 0.72237736), (7, 0.27658504)], 0.9989624);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    out.push(&[(8, 0.7049973), (7, 0.294728)], 0.99972534);
                    match s {
                        [] => true,
                        ['u', s @ ..] => {
                            out.push(&[(7, 0.76490426), (8, 0.2350042)], 0.99990845);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    out.push(&[(8, 0.6923018), (7, 0.30655375)], 0.9988556);
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    out.push(&[(8, 0.74610513), (7, 0.18820478), (2, 0.06535439)], 0.9996643);
                    match s {
                        [] => true,
                        ['r', 'a', 'i', 'c', 'n', 'a', 'l', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    out.push(&[(8, 0.8883802), (7, 0.11134508)], 0.99972534);
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    out.push(&[(7, 0.958648), (8, 0.026398107), (2, 0.0030518044), (93, 0.0030518044), (117, 0.0030518044), (182, 0.0030518044), (0, 0.002746624)], 1.0);
                    match s {
                        [] => true,
                        ['i', 'v', s @ ..] => {
                            out.push(&[(7, 0.9039902), (93, 0.04872206), (0, 0.04385443), (8, 0.0025635157)], 0.9991302);
                            match s {
                                [] => true,
                                ['i', 'd', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(93, 0.5263142), (0, 0.47368583)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['n', 'e', 't', 'a', s @ ..] => {
                            out.push(&[(7, 0.7293202), (2, 0.25152972), (8, 0.013199054)], 0.994049);
                            match s {
                                [] => true,
                                ['i', 'b', 'm', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', 'i', 'm', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(117, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['u', 'f', 'o', 'r', 't', s @ ..] => {
                            out.push(&[(7, 0.7293202), (182, 0.25152972), (8, 0.013199054)], 0.994049);
                            match s {
                                [] => true,
                                ['e', 'v', '\u{1}', s @ ..] => {
                                    out.push(&[(182, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    out.push(&[(7, 0.74392307), (8, 0.24908827), (11, 0.0068055238)], 0.99981683);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            out.push(&[(8, 0.68726635), (7, 0.27972838), (11, 0.03294423)], 0.99993896);
                            s.is_empty()
                        }
                        ['s', 'e', 'g', s @ ..] => {
                            out.push(&[(7, 0.9679866), (8, 0.031128405)], 0.999115);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['z', s @ ..] => {
                    out.push(&[(7, 0.99353015), (8, 0.004806592)], 0.99833673);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            out.push(&[(7, 0.9950561), (8, 0.003418021)], 0.9984741);
                            match s {
                                [] => true,
                                ['c', 'i', 'f', 'i', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['n', 'g', s @ ..] => {
                                            out.push(&[(7, 0.7623865), (8, 0.23689632)], 0.99928284);
                                            s.is_empty()
                                        }
                                        ['t', 's', 'i', s @ ..] => {
                                            out.push(&[(7, 0.7475242), (8, 0.25171283)], 0.99923706);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['g', s @ ..] => {
                                    out.push(&[(7, 0.97541773), (8, 0.012314031), (90, 0.012237736)], 0.9999695);
                                    match s {
                                        [] => true,
                                        ['e', 'l', 'e', 'd', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(90, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['l', s @ ..] => {
                                    out.push(&[(7, 0.994461), (182, 0.0036774243)], 0.9981384);
                                    match s {
                                        [] => true,
                                        ['l', s @ ..] => {
                                            out.push(&[(7, 0.9900206), (8, 0.009750515)], 0.9997711);
                                            s.is_empty()
                                        }
                                        ['u', 'p', 'a', 'n', 'a', 'c', 'e', 'd', '\u{1}', s @ ..] => {
                                            out.push(&[(182, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', s @ ..] => {
                                    out.push(&[(7, 0.9942016), (0, 0.0028839551), (145, 0.0028839551)], 0.9999695);
                                    match s {
                                        [] => true,
                                        ['i', 's', 'i', 'v', '\u{1}', s @ ..] => {
                                            out.push(&[(0, 0.5263142), (7, 0.47368583)], 1.0);
                                            s.is_empty()
                                        }
                                        ['s', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(145, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['z', 'z', 'i', 'l', 'a', 'c', 'i', 'd', 'e', s @ ..] => {
                                    out.push(&[(7, 0.7475242), (8, 0.25171283)], 0.99923706);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['e', s @ ..] => {
                            out.push(&[(7, 0.99145496), (141, 0.008438239)], 0.9998932);
                            match s {
                                [] => true,
                                ['s', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(141, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', s @ ..] => {
                            out.push(&[(7, 0.9775387), (8, 0.021240558)], 0.99877924);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            out.push(&[(8, 0.9540856), (7, 0.020141909), (11, 0.019928282), (2, 0.0058136876)], 0.9999695);
            match s {
                [] => true,
                ['a', 's', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            out.push(&[(8, 0.854078), (7, 0.07769894), (11, 0.062821396), (203, 0.0052185855)], 0.9998169);
            match s {
                [] => true,
                ['m', 'a', 'm', 'o', 't', 't', 'a', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(203, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            out.push(&[(8, 0.94047457), (7, 0.04214542), (11, 0.017227435)], 0.9998474);
            s.is_empty()
        }
        ['p', s @ ..] => {
            out.push(&[(8, 0.86590374), (7, 0.069932096), (11, 0.028595407), (2, 0.028366521), (15, 0.0070649274)], 0.9998627);
            match s {
                [] => true,
                ['a', 's', 'a', s @ ..] => {
                    out.push(&[(8, 0.5328603), (2, 0.32515448), (15, 0.08126955), (7, 0.04303044), (11, 0.017593652)], 0.9999084);
                    match s {
                        [] => true,
                        ['i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['r', s @ ..] => {
            out.push(&[(8, 0.9775692), (11, 0.012909133), (7, 0.009460594)], 0.99993896);
            s.is_empty()
        }
        ['s', s @ ..] => {
            out.push(&[(8, 0.77088577), (11, 0.08880751), (7, 0.077622645), (2, 0.06259251)], 0.99990845);
            match s {
                [] => true,
                ['r', s @ ..] => {
                    out.push(&[(8, 0.50464636), (2, 0.34724957), (11, 0.13084611), (7, 0.017242694)], 0.99998474);
                    match s {
                        [] => true,
                        ['e', 'p', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'r', 'e', 'c', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['i', 'r', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['s', 'e', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['s', 'a', 's', s @ ..] => {
                    out.push(&[(11, 0.5712062), (8, 0.362768), (7, 0.0365301), (2, 0.029449912)], 0.9999542);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(8, 0.95303273), (11, 0.027710384), (7, 0.008148318), (2, 0.005554284), (62, 0.005523766)], 0.9999695);
            match s {
                [] => true,
                ['s', 'a', 'b', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(62, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'e', 'z', 'z', 'a', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['v', s @ ..] => {
            out.push(&[(8, 0.8463264), (7, 0.09080644), (11, 0.062302586)], 0.9994354);
            s.is_empty()
        }
        ['z', s @ ..] => {
            out.push(&[(8, 0.8846723), (7, 0.10936141), (11, 0.0057679103)], 0.9998016);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rne (3 syllables).
pub(crate) fn n610(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.48108643), (2, 0.40341803), (8, 0.1051194), (11, 0.008361944)], 0.99798584);
    match s {
        [] => true,
        ['a', 'c', s @ ..] => {
            match s {
                [] => true,
                ['a', 't', 'i', 'r', 't', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 't', 't', 'a', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ere (3 syllables).
pub(crate) fn n625(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.88308537), (2, 0.053299762), (7, 0.020248722), (11, 0.013305867), (0, 0.009872587), (15, 0.007568475), (66, 0.0025177386), (103, 0.0025177386), (150, 0.0025177386), (154, 0.0025177386)], 0.9974517);
    match s {
        [] => true,
        ['b', s @ ..] => {
            out.push(&[(11, 0.53566796), (8, 0.4155642), (2, 0.025085833), (7, 0.00952163), (0, 0.0046387427), (15, 0.003555352)], 0.9940337);
            s.is_empty()
        }
        ['c', 'o', 'u', 's', 'a', 't', 'n', 'e', 'm', 'r', 'o', 'd', 'd', 'a', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['e', 'c', 'o', 'r', 'p', 'l', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'e', 'p', 's', '\u{1}', s @ ..] => {
                    out.push(&[(154, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', 'r', 'o', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(8, 0.9605249), (0, 0.010055696), (2, 0.008407721), (11, 0.003540093), (15, 0.003494316), (66, 0.003463798), (103, 0.003463798), (150, 0.003463798), (67, 0.0022888533)], 0.998703);
            match s {
                [] => true,
                ['b', 'b', s @ ..] => {
                    out.push(&[(8, 0.7302663), (103, 0.2517281), (0, 0.0050202184), (2, 0.0041962313)], 0.9912108);
                    match s {
                        [] => true,
                        ['a', 'g', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(103, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'g', 'i', 's', s @ ..] => {
                    out.push(&[(8, 0.7302663), (11, 0.25177386), (0, 0.0050202184), (2, 0.0041962313)], 0.9912566);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    out.push(&[(8, 0.9320974), (0, 0.020294499), (2, 0.020233463), (150, 0.020035096), (65, 0.0066376748)], 0.9992981);
                    match s {
                        [] => true,
                        ['i', 'd', 'e', 'p', 'r', 'o', 't', 'a', 'i', 'c', 'c', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'o', 'p', 'u', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'm', 'i', 't', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(150, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', 'e', 'u', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', s @ ..] => {
                    out.push(&[(8, 0.8631571), (2, 0.059998475), (15, 0.04475471), (67, 0.029831387)], 0.99774164);
                    match s {
                        [] => true,
                        ['e', 'c', 'r', 'a', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(67, 0.6666667), (2, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'a', 'b', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['u', 't', 'n', 'i', 't', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', 'n', s @ ..] => {
                    out.push(&[(8, 0.78371865), (66, 0.11004806), (0, 0.100556955)], 0.9943237);
                    match s {
                        [] => true,
                        ['a', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(66, 0.5263142), (0, 0.47368583)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            out.push(&[(8, 0.5680323), (7, 0.35486382), (2, 0.062012665), (11, 0.004623484), (0, 0.00343328), (15, 0.0026398108)], 0.9956054);
            match s {
                [] => true,
                ['o', 'v', 'n', 'o', 'u', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['a', 'c', 'a', 't', 't', 'i', 'f', 'f', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['h', 's', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            out.push(&[(8, 0.55484855), (2, 0.2527199), (7, 0.1255665), (11, 0.06317235)], 0.9963073);
            match s {
                [] => true,
                ['e', 'c', s @ ..] => {
                    out.push(&[(2, 0.64362556), (7, 0.17872892), (8, 0.15852597), (11, 0.018051423)], 0.9989319);
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
        ['p', s @ ..] => {
            out.push(&[(8, 0.52419317), (2, 0.4642405), (7, 0.0036774243), (11, 0.0024109255)], 0.99452204);
            match s {
                [] => true,
                ['a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['m', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['l', 'i', 'k', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
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
        ['r', 'e', s @ ..] => {
            out.push(&[(8, 0.44153506), (2, 0.27664608), (7, 0.26012054), (11, 0.0066529335), (0, 0.0049286643), (15, 0.0037842374)], 0.9936675);
            match s {
                [] => true,
                ['s', 'i', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', 's', 'e', s @ ..] => {
            match s {
                [] => true,
                ['n', 'e', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['s', 'i', 's', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    out.push(&[(8, 0.41602197), (2, 0.2905623), (7, 0.1435111), (15, 0.14001678), (11, 0.0036621653), (0, 0.002716106)], 0.9964904);
                    match s {
                        [] => true,
                        ['e', 'l', 'a', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['s', 'e', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
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
        ['v', 'l', 'o', s @ ..] => {
            match s {
                [] => true,
                ['i', 'c', 's', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'a', 'r', 'i', 'p', 's', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word ardire (3 syllables).
pub(crate) fn n1534(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -sire (3 syllables).
pub(crate) fn n1535(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'd', '\u{1}', s @ ..] => {
            out.push(&[(8, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['d', '\u{1}', s @ ..] => {
                    out.push(&[(8, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', '\u{1}', s @ ..] => {
                    out.push(&[(8, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -tire (3 syllables).
pub(crate) fn n1536(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(11, 0.6805676), (2, 0.19438468), (8, 0.09715419), (0, 0.02699321)], 0.99909973);
    s.is_empty()
}

/// Words ending -ore (3 syllables).
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.98442054), (7, 0.005355917), (11, 0.0044403756), (2, 0.003540093)], 0.9977569);
    match s {
        [] => true,
        ['c', 'e', 'p', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(8, 0.66904706), (11, 0.32945755)], 0.99850464);
            s.is_empty()
        }
        ['l', 'o', 'c', 'o', s @ ..] => {
            out.push(&[(8, 0.7422141), (2, 0.25177386), (7, 0.0026703288), (11, 0.0022278172)], 0.9988861);
            match s {
                [] => true,
                ['t', 'o', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 'a', 'v', 'o', 'l', 'l', 'a', 'v', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(173, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', s @ ..] => {
            out.push(&[(8, 0.92889297), (0, 0.06897078)], 0.99786377);
            match s {
                [] => true,
                ['t', 't', 'a', 'u', 'q', 'i', 't', 'n', 'e', 'v', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(8, 0.99056995), (7, 0.004562448)], 0.9951324);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(8, 0.9952087), (7, 0.0039673457)], 0.999176);
                    match s {
                        [] => true,
                        ['i', 'g', 'g', 'e', 's', 'r', s @ ..] => {
                            out.push(&[(8, 0.7476005), (7, 0.2519875)], 0.999588);
                            s.is_empty()
                        }
                        ['r', 'i', 'p', 's', 'e', s @ ..] => {
                            out.push(&[(8, 0.7476005), (7, 0.2519875)], 0.999588);
                            s.is_empty()
                        }
                        ['t', s @ ..] => {
                            out.push(&[(8, 0.9903258), (7, 0.004852369), (210, 0.004806592)], 0.99998474);
                            match s {
                                [] => true,
                                ['t', 'e', 'c', 'n', 'i', '\u{1}', s @ ..] => {
                                    out.push(&[(210, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    out.push(&[(8, 0.98556495), (11, 0.009567407), (55, 0.004776074)], 0.99990845);
                    match s {
                        [] => true,
                        ['l', 'g', 'o', 'c', 'c', 'a', 'r', '-', 'e', 'r', 'o', 't', 'a', 'i', 'c', 'c', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(55, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    out.push(&[(8, 0.94270235), (2, 0.044113833), (7, 0.012695506)], 0.9995117);
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    out.push(&[(8, 0.92004275), (2, 0.03959716), (199, 0.03959716)], 0.99923706);
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['-', 'o', 'n', 'e', 'r', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(199, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    out.push(&[(8, 0.9289082), (2, 0.070420384)], 0.9993286);
                    match s {
                        [] => true,
                        ['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'e', 'p', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    out.push(&[(8, 0.9774319), (7, 0.016906997), (0, 0.00561532)], 0.9999542);
                    match s {
                        [] => true,
                        ['o', 't', 'n', 'a', 'r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['u', s @ ..] => {
            out.push(&[(8, 0.81945527), (2, 0.17840849)], 0.99786377);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'd', 'l', 'a', 'c', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'g', 'g', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
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

/// The word succiacapre (3 syllables).
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -rre (3 syllables).
pub(crate) fn n630(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.59867245), (7, 0.19516289), (0, 0.19282825), (2, 0.007553216), (11, 0.004043641)], 0.99826044);
    match s {
        [] => true,
        ['a', 'g', 'a', 'b', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tre (3 syllables).
pub(crate) fn n631(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.60880446), (2, 0.18706036), (7, 0.078431375), (11, 0.062836654), (15, 0.061539635)], 0.9986725);
    match s {
        [] => true,
        ['e', 'i', 'p', 'a', s @ ..] => {
            match s {
                [] => true,
                ['c', 'c', 'a', 'p', 's', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'c', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['l', 'g', 'a', 't', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', 'l', 'o', 'v', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ure (3 syllables).
pub(crate) fn n632(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.50110626), (0, 0.30844587), (2, 0.12552072), (11, 0.062836654)], 0.9979095);
    match s {
        [] => true,
        ['c', 'i', s @ ..] => {
            match s {
                [] => true,
                ['d', 'e', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', 'r', 'e', 'v', 'a', 't', 't', 'e', 'f', 'f', 'a', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['h', 'c', 'o', 'r', 'b', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'a', 'p', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['r', 'e', 'v', 'u', 'o', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'a', 'f', 'a', 'i', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ère (3 syllables).
pub(crate) fn n633(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.40247196), (8, 0.37860686), (2, 0.20793469), (7, 0.004913405), (11, 0.004242008)], 0.99816895);
    s.is_empty()
}

/// Words ending -lite (3 syllables).
pub(crate) fn n1586(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.73145646), (8, 0.21945526), (0, 0.02932784), (11, 0.009903105), (15, 0.009658961)], 0.99980164);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', 'c', 'a', 't', 't', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'o', 'u', 'q', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'e', 'h', 'c', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(7, 0.89021134), (11, 0.07284657), (8, 0.03135729), (0, 0.0041962313)], 0.99861145);
                    s.is_empty()
                }
                ['d', 'o', s @ ..] => {
                    out.push(&[(8, 0.68778515), (7, 0.2925765), (0, 0.011734188), (11, 0.0039673457), (15, 0.0038605325)], 0.9999237);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['é', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -limite (3 syllables).
pub(crate) fn n1587(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.42626077), (8, 0.30331883), (2, 0.25424582), (0, 0.008712902), (11, 0.006149386)], 0.9986877);
    match s {
        [] => true,
        ['a', 'n', 'g', 'e', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nite (3 syllables).
pub(crate) fn n1588(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.92646676), (8, 0.050156403), (0, 0.022858014)], 0.9994812);
    match s {
        [] => true,
        ['a', 'i', 'v', 'i', 'l', 'o', 'b', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['n', 'a', 'm', 'k', 'c', 'o', 'l', 'k', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', s @ ..] => {
            out.push(&[(8, 0.6200656), (7, 0.3705806), (0, 0.009140154)], 0.9997864);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rite (3 syllables).
pub(crate) fn n1590(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['d', 'n', 'a', 's', 's', 'e', 'l', 'a', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['e', 't', 'n', 'a', 'c', 'o', 'r', 't', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'e', s @ ..] => {
            out.push(&[(7, 0.42626077), (8, 0.30331883), (11, 0.2561532), (0, 0.008712902), (2, 0.004242008)], 0.9986877);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word andalusite (3 syllables).
pub(crate) fn n1591(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -tite (3 syllables).
pub(crate) fn n1592(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.92126346), (2, 0.029999238), (8, 0.018036164), (0, 0.015381094), (11, 0.015228504)], 0.99990845);
    match s {
        [] => true,
        ['a', 'm', 'a', s @ ..] => {
            out.push(&[(2, 0.514992), (7, 0.46063936), (8, 0.009018082), (0, 0.007690547), (11, 0.007614252)], 0.9999542);
            s.is_empty()
        }
        ['r', 'o', 'a', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -avite (3 syllables).
pub(crate) fn n1593(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.47434196), (8, 0.3759823), (2, 0.07006943), (21, 0.06666667), (0, 0.0069733732), (11, 0.0049286643)], 0.9989624);
    s.is_empty()
}

/// Words ending -nte (3 syllables).
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.52895397), (11, 0.36118105), (7, 0.09816129), (2, 0.005737392)], 0.9940337);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', 'e', s @ ..] => {
                    out.push(&[(7, 0.5245441), (8, 0.3822385), (11, 0.09028763)], 0.99707025);
                    s.is_empty()
                }
                ['f', 'n', 'i', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', 'i', 'g', s @ ..] => {
                    out.push(&[(8, 0.588098), (7, 0.1588922), (2, 0.14659342), (11, 0.104692146)], 0.99827576);
                    match s {
                        [] => true,
                        ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 0.5263142), (7, 0.47368583)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'l', s @ ..] => {
                    out.push(&[(8, 0.8506142), (11, 0.13249409), (7, 0.015106432)], 0.9982147);
                    s.is_empty()
                }
                ['m', 'o', s @ ..] => {
                    out.push(&[(11, 0.8567483), (8, 0.12942703), (7, 0.0123445485)], 0.99851984);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'm', s @ ..] => {
                            out.push(&[(7, 0.5245441), (8, 0.3822385), (11, 0.09028763)], 0.99707025);
                            s.is_empty()
                        }
                        ['o', 's', s @ ..] => {
                            out.push(&[(7, 0.54908067), (8, 0.26446936), (11, 0.18059053), (2, 0.002868696)], 0.9970092);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', 'm', s @ ..] => {
                    out.push(&[(7, 0.6392615), (8, 0.2115816), (11, 0.14447242), (2, 0.0023041123)], 0.9976196);
                    s.is_empty()
                }
                ['r', 'u', s @ ..] => {
                    out.push(&[(8, 0.9057908), (11, 0.07223621), (7, 0.019638361)], 0.9976654);
                    s.is_empty()
                }
                ['t', 'i', s @ ..] => {
                    out.push(&[(11, 0.73296714), (8, 0.1729305), (7, 0.09213398)], 0.9980316);
                    s.is_empty()
                }
                ['z', s @ ..] => {
                    out.push(&[(8, 0.8677653), (11, 0.12234684), (7, 0.008819715)], 0.9989319);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(11, 0.4322423), (8, 0.42269015), (7, 0.1292134), (2, 0.004791333), (81, 0.0037994965), (205, 0.0037994965), (0, 0.003418021)], 0.9999542);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(11, 0.6949111), (8, 0.29173723), (7, 0.011886778)], 0.99853516);
                    s.is_empty()
                }
                ['d', 'n', 'e', 'p', s @ ..] => {
                    out.push(&[(11, 0.6608835), (7, 0.26541543), (8, 0.07104601)], 0.9973449);
                    s.is_empty()
                }
                ['g', s @ ..] => {
                    out.push(&[(8, 0.43706417), (11, 0.27486077), (7, 0.2560769), (2, 0.031311512)], 0.99931335);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            out.push(&[(8, 0.73426414), (2, 0.13282979), (11, 0.06871138), (7, 0.064011596)], 0.9998169);
                            match s {
                                [] => true,
                                ['l', 'l', 'o', 'f', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['g', s @ ..] => {
                            out.push(&[(11, 0.649958), (7, 0.21602197), (8, 0.12487984), (2, 0.008941787)], 0.99980164);
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            out.push(&[(8, 0.48950943), (7, 0.45934233), (11, 0.045807585), (2, 0.0052185855)], 0.9998779);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'r', s @ ..] => {
                    out.push(&[(8, 0.58731973), (7, 0.27374685), (11, 0.10478371), (2, 0.031464104)], 0.9973144);
                    match s {
                        [] => true,
                        ['o', 'i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
                    out.push(&[(8, 0.58454263), (11, 0.28644237), (205, 0.10075532), (7, 0.025848784)], 0.9975891);
                    match s {
                        [] => true,
                        ['a', 'v', 'i', 'u', 'q', 'e', '-', 'o', 'm', 'm', 'a', 'r', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(205, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', s @ ..] => {
                            out.push(&[(11, 0.8580606), (8, 0.10566872), (7, 0.03230335)], 0.99603266);
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            out.push(&[(8, 0.38820478), (7, 0.2877241), (11, 0.19354543), (81, 0.06808575), (0, 0.061280232)], 0.99884033);
                            match s {
                                [] => true,
                                ['o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(81, 0.5263142), (0, 0.47368583)], 1.0);
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
                        ['t', 'i', s @ ..] => {
                            out.push(&[(7, 0.5258412), (11, 0.38645), (8, 0.08453498)], 0.9968262);
                            s.is_empty()
                        }
                        ['u', s @ ..] => {
                            out.push(&[(11, 0.8580606), (8, 0.10566872), (7, 0.03230335)], 0.99603266);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['v', 'i', s @ ..] => {
                    out.push(&[(11, 0.48305485), (7, 0.4073091), (8, 0.10566872)], 0.99603266);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(8, 0.95251393), (161, 0.024444953), (11, 0.017654689), (7, 0.004806592)], 0.99942017);
            match s {
                [] => true,
                ['p', 'o', 'r', 'r', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(161, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'p', 'a', 'v', 'l', 'a', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ote (3 syllables).
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.5205005), (7, 0.13577478), (11, 0.12205692), (2, 0.11088731), (0, 0.07368582), (15, 0.036621653)], 0.999527);
    match s {
        [] => true,
        ['g', s @ ..] => {
            out.push(&[(8, 0.9064317), (7, 0.026489662), (11, 0.023819333), (2, 0.021637293), (0, 0.014373999), (15, 0.0071412222)], 0.9998932);
            s.is_empty()
        }
        ['n', 'e', 'c', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['p', s @ ..] => {
            out.push(&[(11, 0.5190509), (8, 0.20656139), (0, 0.1835813), (7, 0.043442436), (2, 0.035492484), (15, 0.011718929)], 0.9998474);
            match s {
                [] => true,
                ['a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'a', 'c', 'a', 't', 'n', 'a', 'i', 'p', '\u{1}', s @ ..] => {
            out.push(&[(15, 1.0)], 1.0);
            s.is_empty()
        }
        ['u', s @ ..] => {
            out.push(&[(2, 0.444358), (0, 0.22948043), (8, 0.20819409), (7, 0.054306857), (11, 0.04881361), (15, 0.014648661)], 0.99980164);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rte (3 syllables).
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.2778973), (8, 0.23492791), (2, 0.19319448), (18, 0.12231632), (15, 0.06919967), (11, 0.049988557), (0, 0.031906616), (275, 0.020355536)], 0.9997864);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(2, 0.4120241), (7, 0.35843444), (15, 0.1540856), (8, 0.030319676), (11, 0.022583352), (18, 0.015777828), (0, 0.004119936), (275, 0.0026245518)], 0.9999695);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(2, 0.64991224), (15, 0.2436408), (7, 0.068268865), (11, 0.028107118), (8, 0.0057679103), (18, 0.0030060273)], 0.99870294);
                    match s {
                        [] => true,
                        ['a', 'p', 'u', 'i', 'c', 's', '\u{1}', s @ ..] => {
                            out.push(&[(11, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'n', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 'e', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['p', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 't', 'o', 's', '\u{1}', s @ ..] => {
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
            out.push(&[(8, 0.38803694), (18, 0.22049287), (7, 0.20202945), (11, 0.057145037), (0, 0.055268176), (275, 0.036743727), (2, 0.0330663), (15, 0.0071869995)], 0.9999695);
            match s {
                [] => true,
                ['f', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'c', 'o', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(275, 0.6666667), (7, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(18, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['s', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(18, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['u', 'q', 'c', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(18, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['z', 'z', 'a', 'i', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(18, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'r', 'e', 'h', 'c', 'a', 's', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ste (3 syllables).
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(8, 0.46327916), (7, 0.18313879), (11, 0.16011292), (15, 0.115556575), (2, 0.07580682)], 0.9978943);
    match s {
        [] => true,
        ['a', 'r', s @ ..] => {
            out.push(&[(7, 0.39438468), (11, 0.3867094), (8, 0.1544213), (15, 0.038513772), (2, 0.02526894)], 0.9992981);
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['f', 'a', 't', 's', 'a', 'u', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['h', 'c', 'u', 'a', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', 'i', 'r', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'c', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
            out.push(&[(2, 0.5263142), (15, 0.47368583)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tte (3 syllables).
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 0.39676508), (2, 0.27765316), (7, 0.17224383), (8, 0.07695125), (11, 0.028900588), (15, 0.026382849), (230, 0.020828566)], 0.99972534);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(2, 0.7036698), (0, 0.21739529), (7, 0.041763943), (8, 0.018661784), (11, 0.0070038913), (15, 0.0063935304), (230, 0.0050507365)], 0.99993896);
            s.is_empty()
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['i', 'l', 'b', 'u', 'o', '\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'a', 'r', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(11, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['r', 't', 'i', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'o', 'n', '\u{1}', s @ ..] => {
                            out.push(&[(8, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', s @ ..] => {
                    out.push(&[(2, 0.7592126), (0, 0.13224994), (7, 0.0574197), (8, 0.025650416), (11, 0.009628443), (15, 0.008789197), (230, 0.006942855)], 0.9998932);
                    s.is_empty()
                }
                ['u', 'q', 'o', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(7, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', 'o', 'r', 'p', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(8, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(7, 0.3523308), (0, 0.27589837), (2, 0.11311513), (15, 0.09474327), (8, 0.08392462), (230, 0.07576104), (11, 0.0041962313)], 0.9999695);
            match s {
                [] => true,
                ['b', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', 'i', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['a', 'r', 'p', 'o', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 'p', 'o', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(7, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'c', 's', 'i', 'h', 'c', 'n', 'o', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(8, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'u', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', '\u{1}', s @ ..] => {
                            out.push(&[(7, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    out.push(&[(15, 0.27418935), (2, 0.23356985), (230, 0.21933319), (7, 0.13421835), (0, 0.10510414), (8, 0.03196765)], 0.9983825);
                    match s {
                        [] => true,
                        ['a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(230, 1.0)], 1.0);
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

/// Words ending -ute (3 syllables).
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(7, 0.34398413), (2, 0.32393378), (8, 0.2835584), (11, 0.04072633), (0, 0.004730297)], 0.9969329);
    match s {
        [] => true,
        ['d', s @ ..] => {
            match s {
                [] => true,
                ['a', 'c', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['e', 'v', 'a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'a', 'v', 'a', 'i', 'b', 'm', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -yte (3 syllables).
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.72532237), (7, 0.10969711), (8, 0.09910735), (11, 0.055283435), (0, 0.0064240484), (15, 0.0023498894)], 0.9981842);
    s.is_empty()
}

/// The word ombellico (3 syllables).
pub(crate) fn n2810(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 1.0)], 1.0);
    s.is_empty()
}

/// The word dolico (3 syllables).
pub(crate) fn n2811(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -nico (3 syllables).
pub(crate) fn n1664(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9763485), (147, 0.010498207), (5, 0.007553216), (148, 0.0052491035)], 0.99964905);
    match s {
        [] => true,
        ['c', 'e', 't', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(147, 0.6666667), (148, 0.33333334)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rico (3 syllables).
pub(crate) fn n1665(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.70110625), (5, 0.26965743), (54, 0.0230869), (137, 0.0057526515)], 0.9996032);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(5, 0.59328604), (1, 0.32741284), (54, 0.06340124), (137, 0.015854124)], 0.9999542);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(54, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(137, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['d', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['g', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'a', 'm', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['b', 'm', 'o', 'l', '\u{1}', s @ ..] => {
            out.push(&[(5, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', 'n', 'i', '\u{1}', s @ ..] => {
            out.push(&[(5, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tico (3 syllables).
pub(crate) fn n1666(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9844358), (5, 0.015457389)], 0.9998932);
    match s {
        [] => true,
        ['e', 'l', 'l', 'o', 's', '\u{1}', s @ ..] => {
            out.push(&[(5, 1.0)], 1.0);
            s.is_empty()
        }
        ['n', 'a', '\u{1}', s @ ..] => {
            out.push(&[(5, 1.0)], 1.0);
            s.is_empty()
        }
        ['s', 'a', 'm', 'i', 'r', '\u{1}', s @ ..] => {
            out.push(&[(5, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -vico (3 syllables).
pub(crate) fn n1667(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.53667504), (1, 0.45783168), (2, 0.0031433585)], 0.9976501);
    s.is_empty()
}

/// Words ending -zico (3 syllables).
pub(crate) fn n1668(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.5858549), (1, 0.40923172), (2, 0.00280766)], 0.9978943);
    s.is_empty()
}

/// Words ending -lco (3 syllables).
pub(crate) fn n707(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.7954376), (2, 0.098664835), (1, 0.055786986), (15, 0.04866102)], 0.9985504);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['f', 'e', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'i', 'r', 'p', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'i', 'g', 'r', 'a', 'p', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -nco (3 syllables).
pub(crate) fn n708(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.8373846), (1, 0.03588922), (2, 0.0324712), (63, 0.031006332), (154, 0.031006332), (263, 0.031006332)], 0.998764);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['i', 'b', 'o', 't', 't', 'e', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(263, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'b', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(63, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 'l', 'e', 'i', 't', 'n', 'a', 'v', 'a', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'r', 't', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(154, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -oco (3 syllables).
pub(crate) fn n709(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.5089494), (1, 0.18321508), (2, 0.17930877), (0, 0.041092545), (15, 0.03587396), (274, 0.035523005), (85, 0.010147249), (86, 0.0050812545)], 0.9991913);
    match s {
        [] => true,
        ['i', 'g', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['l', 'o', 'r', 'p', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', s @ ..] => {
            out.push(&[(1, 0.67328906), (5, 0.20358586), (2, 0.071717404), (0, 0.016433967), (15, 0.014343481), (274, 0.01420615), (85, 0.0040589), (86, 0.00202945)], 0.99966425);
            s.is_empty()
        }
        ['u', 'f', s @ ..] => {
            out.push(&[(5, 0.38898298), (2, 0.33788052), (15, 0.08308537), (274, 0.08302434), (0, 0.07614252), (1, 0.0284123)], 0.997528);
            match s {
                [] => true,
                ['-', 'a', 'c', 'c', 'o', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(274, 1.0)], 1.0);
                    s.is_empty()
                }
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['t', 't', 'u', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(5, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['z', 'z', 'i', 't', 't', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(5, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(5, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(5, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['v', 'i', 'u', 'q', 'e', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rco (3 syllables).
pub(crate) fn n710(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.8866102), (15, 0.05726711), (1, 0.05261311), (2, 0.002151522)], 0.99864197);
    match s {
        [] => true,
        ['a', 'p', 'a', s @ ..] => {
            match s {
                [] => true,
                ['d', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -sco (3 syllables).
pub(crate) fn n711(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.91345084), (1, 0.043762874), (2, 0.042236973)], 0.9994507);
    match s {
        [] => true,
        ['a', 'c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['d', 'i', 'g', 'n', 'i', 'p', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'a', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'b', 'a', 'i', 'c', 'c', 'a', 'r', 'b', 'b', 'a', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['u', 'r', 't', 'e', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -uco (3 syllables).
pub(crate) fn n712(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.7841764), (1, 0.111314565), (248, 0.0961471), (2, 0.004562448)], 0.9962005);
    match s {
        [] => true,
        ['b', 'o', 's', 's', 'o', '\u{1}', s @ ..] => {
            out.push(&[(248, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -logo (3 syllables).
pub(crate) fn n1728(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.949218), (5, 0.050766766)], 0.99998474);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(5, 0.51692986), (1, 0.4830701)], 1.0);
            match s {
                [] => true,
                ['m', 'm', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(5, 0.52538335), (1, 0.47461662)], 1.0);
            s.is_empty()
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['l', 'e', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(5, 1.0)], 1.0);
                    s.is_empty()
                }
                ['m', 'o', '\u{1}', s @ ..] => {
                    out.push(&[(5, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'o', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(5, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(5, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'p', s @ ..] => {
                    out.push(&[(5, 0.52538335), (1, 0.47461662)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -rgo (3 syllables).
pub(crate) fn n742(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.87881285), (1, 0.119935915)], 0.9987488);
    match s {
        [] => true,
        ['u', 't', 'a', 'm', 'u', 'a', 't', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ugo (3 syllables).
pub(crate) fn n743(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(5, 0.91410697), (1, 0.08381781)], 0.9979248);
    s.is_empty()
}

/// Words ending -agnio (2 syllables).
pub(crate) fn n1754(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['l', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -pio (2 syllables).
pub(crate) fn n765(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.8047303), (1, 0.13054094), (285, 0.060791943)], 0.9960632);
    match s {
        [] => true,
        ['m', 'e', 't', '\u{1}', s @ ..] => {
            out.push(&[(285, 1.0)], 1.0);
            s.is_empty()
        }
        ['s', 'a', 'r', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['u', 'i', 'c', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rio (2 syllables).
pub(crate) fn n766(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', 'g', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -sio (2 syllables).
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 'd', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', 'd', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['u', 'r', 'b', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tio (2 syllables).
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.50875103), (1, 0.4517891), (2, 0.023956664), (23, 0.011429007)], 0.99592584);
    match s {
        [] => true,
        ['i', 'a', 'r', 'b', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'c', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['s', 'e', 'p', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['e', 'a', 's', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['u', 'q', 's', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['z', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word sciacquio (2 syllables).
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -vio (2 syllables).
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.5250324), (1, 0.3823911), (2, 0.08815137)], 0.9955749);
    match s {
        [] => true,
        ['a', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['n', 'i', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['v', 'a', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -zio (2 syllables).
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['n', 'o', 'r', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'r', 'p', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['z', 'i', 'r', 'f', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -io (3 syllables).
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.9358206), (1, 0.048691537), (2, 0.008484016)], 0.99299616);
    match s {
        [] => true,
        ['-', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['a', s @ ..] => {
            out.push(&[(13, 0.99577326), (48, 0.0037384604)], 0.9995117);
            match s {
                [] => true,
                ['n', 'i', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(48, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['b', 'm', 'o', 'b', 'm', 'i', 'r', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['c', s @ ..] => {
            out.push(&[(13, 0.97615016), (2, 0.018814374), (156, 0.0025024796), (1, 0.0025024796)], 0.9999695);
            match s {
                [] => true,
                ['c', 'a', 'i', s @ ..] => {
                    out.push(&[(13, 0.8046998), (2, 0.19481193)], 0.9995117);
                    match s {
                        [] => true,
                        ['h', 'g', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['m', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['t', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['i', 'r', 't', '\u{1}', s @ ..] => {
                                                    out.push(&[(2, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                ['r', 'o', 'p', '\u{1}', s @ ..] => {
                                                    out.push(&[(2, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        _ => false,
                                    }
                                }
                                ['i', 't', 't', 'a', 'b', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
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
                        ['f', 'f', s @ ..] => {
                            out.push(&[(13, 0.8004425), (156, 0.19142443), (2, 0.0071717403)], 0.9990387);
                            match s {
                                [] => true,
                                ['u', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(156, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['l', 'i', 's', 'a', 'k', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'r', 'e', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'a', 'c', 'o', 't', 'o', 't', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 'u', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'm', 'r', 'e', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['g', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            out.push(&[(13, 0.92729074), (1, 0.038956285), (2, 0.019272145), (0, 0.007980469), (15, 0.0063935304)], 0.9998932);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['g', 'e', 'r', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['z', 'n', 'e', 's', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['d', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'r', 'u', 'i', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'd', 'a', 'r', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
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
                                ['a', 'r', 'g', '\u{1}', s @ ..] => {
                                    out.push(&[(1, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['i', 'r', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['o', s @ ..] => {
                                            out.push(&[(13, 0.57064164), (1, 0.40859082), (2, 0.01185626), (0, 0.004913405), (15, 0.0039368276)], 0.99993896);
                                            match s {
                                                [] => true,
                                                ['s', 's', 'u', '\u{1}', s @ ..] => {
                                                    out.push(&[(1, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                ['x', 'u', '\u{1}', s @ ..] => {
                                                    out.push(&[(1, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        ['t', 'a', 'p', '\u{1}', s @ ..] => {
                                            out.push(&[(1, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['d', 'n', 'e', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'e', 's', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'a', 'm', 'a', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['v', 'i', 'r', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'e', 'p', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'o', 'b', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['f', 'f', 'u', 'r', s @ ..] => {
            match s {
                [] => true,
                ['a', 'b', 'b', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            out.push(&[(13, 0.98326087), (2, 0.009231708), (1, 0.004592966), (40, 0.002166781)], 0.9992523);
            match s {
                [] => true,
                ['g', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            match s {
                                [] => true,
                                ['p', 'o', 'd', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 'o', 'f', 'a', 's', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['e', s @ ..] => {
                            match s {
                                [] => true,
                                ['l', 'a', 't', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['p', 'p', 'e', 'l', 'a', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['o', 'l', 'o', 'r', 'o', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['h', s @ ..] => {
            out.push(&[(13, 0.88667125), (1, 0.0964828), (176, 0.009918365), (48, 0.0066224155)], 0.9996948);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', s @ ..] => {
                            out.push(&[(13, 0.9659876), (176, 0.014648661), (48, 0.009765774), (1, 0.009597925)], 1.0);
                            match s {
                                [] => true,
                                ['e', 'r', 'o', '\u{1}', s @ ..] => {
                                    out.push(&[(176, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 'n', 'i', 'g', '\u{1}', s @ ..] => {
                                    out.push(&[(48, 0.6666667), (13, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['u', 'i', 'c', 'a', 'b', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(1, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', s @ ..] => {
                            out.push(&[(1, 0.64585334), (13, 0.3510033)], 0.99685663);
                            match s {
                                [] => true,
                                ['r', 'r', 'i', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 's', 'i', 'm', 'e', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'a', 'i', 'r', 't', 'a', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'u', 'r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', 'i', 't', 'i', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => super::t1::n780(s, out),
        ['n', s @ ..] => {
            out.push(&[(13, 0.9044633), (1, 0.08889906), (48, 0.0051728087)], 0.99853516);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'c', 'a', 'b', 's', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['p', s @ ..] => {
                            out.push(&[(1, 0.54445714), (13, 0.45223165), (48, 0.0025940337)], 0.99928284);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'a', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'i', s @ ..] => {
                            match s {
                                [] => true,
                                ['d', '\u{1}', s @ ..] => {
                                    out.push(&[(1, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', '\u{1}', s @ ..] => {
                                    out.push(&[(1, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', 'a', 'c', 's', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['g', 'e', 'p', 'o', 'n', 'c', 'e', 't', '\u{1}', s @ ..] => {
                    out.push(&[(48, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', s @ ..] => super::t1::n1799(s, out),
                ['n', s @ ..] => super::t1::n1800(s, out),
                ['o', s @ ..] => super::t1::n1801(s, out),
                _ => false,
            }
        }
        ['o', s @ ..] => super::t1::n782(s, out),
        ['p', s @ ..] => super::t1::n783(s, out),
        ['r', s @ ..] => super::t1::n784(s, out),
        ['s', 's', 'a', 't', 'a', 'p', '\u{1}', s @ ..] => super::t1::n785(s, out),
        ['t', s @ ..] => super::t1::n786(s, out),
        ['u', s @ ..] => super::t1::n787(s, out),
        ['z', s @ ..] => super::t1::n788(s, out),
        _ => false,
    }
}

/// The word torcicollo (3 syllables).
pub(crate) fn n4108(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// The word storcicollo (3 syllables).
pub(crate) fn n4109(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// The word originalismo (3 syllables).
pub(crate) fn n4110(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// The word antinazionalismo (3 syllables).
pub(crate) fn n4111(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -vanismo (3 syllables).
pub(crate) fn n3741(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.75558096), (39, 0.23680475), (2, 0.00608835)], 0.99847406);
    s.is_empty()
}

/// Words ending -inismo (3 syllables).
pub(crate) fn n3742(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['l', 'l', 'e', 'b', 'i', 'h', 'g', 'o', 'e', 'n', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['n', s @ ..] => {
            out.push(&[(1, 0.74030674), (222, 0.25160602), (2, 0.0064698253)], 0.99838257);
            match s {
                [] => true,
                ['e', 'l', '-', 'o', 'm', 's', 'i', 'x', 'r', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(222, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word clintonismo (3 syllables).
pub(crate) fn n3743(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -ocomunismo (3 syllables).
pub(crate) fn n3744(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.77346456), (2, 0.2251011)], 0.9985657);
    match s {
        [] => true,
        ['g', 'o', 'u', 'l', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'e', 't', 'e', 'v', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rismo (3 syllables).
pub(crate) fn n3010(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.98626685), (2, 0.009185932), (0, 0.0045319297)], 0.99998474);
    match s {
        [] => true,
        ['i', 'a', 'l', 'b', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'r', 'r', 'e', 't', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['u', 't', 'n', s @ ..] => {
            out.push(&[(1, 0.7431296), (0, 0.25226215), (2, 0.004592966)], 0.99998474);
            match s {
                [] => true,
                ['a', 'r', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word doppiopesismo (3 syllables).
pub(crate) fn n3011(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -tismo (3 syllables).
pub(crate) fn n3012(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.98861676), (2, 0.007614252), (39, 0.0037537194)], 0.99998474);
    match s {
        [] => true,
        ['i', 'c', 's', 'u', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', s @ ..] => {
            match s {
                [] => true,
                ['a', 'b', s @ ..] => {
                    out.push(&[(1, 0.7443046), (39, 0.25188068), (2, 0.0037994965)], 0.99998474);
                    s.is_empty()
                }
                ['e', 'r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word schiavismo (3 syllables).
pub(crate) fn n3013(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -umo (3 syllables).
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.64820325), (2, 0.3502861)], 0.9984894);
    match s {
        [] => true,
        ['f', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'c', 'a', 'c', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'a', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', 'r', 'e', 'n', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word portaspazzolino (3 syllables).
pub(crate) fn n1868(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -rino (3 syllables).
pub(crate) fn n1869(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9817502), (2, 0.0121767), (39, 0.0060425727)], 0.9999695);
    match s {
        [] => true,
        ['o', 't', 'o', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', 'e', 'm', 'a', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word scaricalasino (3 syllables).
pub(crate) fn n1870(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -istino (3 syllables).
pub(crate) fn n1871(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.7108873), (2, 0.28815138)], 0.9990387);
    match s {
        [] => true,
        ['l', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'p', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -avino (3 syllables).
pub(crate) fn n1872(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.8143282), (2, 0.18504615)], 0.99937433);
    match s {
        [] => true,
        ['i', 'g', 'g', 'a', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', 't', 'o', 'b', 'm', 'i', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nno (3 syllables).
pub(crate) fn n815(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9349813), (2, 0.037552454), (15, 0.014602884), (87, 0.008346685), (88, 0.004165713)], 0.99964905);
    match s {
        [] => true,
        ['a', 'm', s @ ..] => {
            match s {
                [] => true,
                ['a', 'l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['e', 'l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 's', 'a', s @ ..] => {
            match s {
                [] => true,
                ['c', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'u', 'p', 's', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -ono (3 syllables).
pub(crate) fn n816(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.99176013), (174, 0.0078583965)], 0.99961853);
    match s {
        [] => true,
        ['u', 'b', 'o', 'v', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(174, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rno (3 syllables).
pub(crate) fn n817(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9200733), (2, 0.02044709), (15, 0.019745175), (27, 0.019745175), (155, 0.019745175)], 0.99975586);
    match s {
        [] => true,
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['f', 'o', 't', 'l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(27, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'g', 'i', 'd', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 't', s @ ..] => {
            match s {
                [] => true,
                ['a', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(155, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word novantuno (3 syllables).
pub(crate) fn n818(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -nto (3 syllables).
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9884489), (2, 0.008514535)], 0.99696344);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(1, 0.9032578), (2, 0.07193103), (57, 0.02465858)], 0.9998474);
            match s {
                [] => true,
                ['c', 'l', 'e', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'p', 'm', 'i', 'e', 'r', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 'o', 'p', 'm', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(57, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(1, 0.99163806), (2, 0.006134127)], 0.9977722);
            match s {
                [] => true,
                ['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    out.push(&[(1, 0.9962005), (2, 0.0024719615)], 0.9986725);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            out.push(&[(1, 0.99769586), (2, 0.0019989319)], 0.9996948);
                            match s {
                                [] => true,
                                ['n', 'o', 'i', 'z', 'e', 'f', 'r', s @ ..] => {
                                    out.push(&[(1, 0.7488518), (2, 0.25099567)], 0.9998474);
                                    match s {
                                        [] => true,
                                        ['e', 'p', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['r', 'o', 'i', s @ ..] => {
                                    out.push(&[(1, 0.8776074), (2, 0.12234684)], 0.9999542);
                                    match s {
                                        [] => true,
                                        ['l', 'g', 'i', 'm', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['-', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                                                    out.push(&[(2, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                ['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                                                    out.push(&[(2, 1.0)], 1.0);
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
                                        ['e', 'l', s @ ..] => {
                                            out.push(&[(1, 0.7990845), (2, 0.20080873)], 0.9998932);
                                            match s {
                                                [] => true,
                                                ['p', 'm', 'o', 'c', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                                                    out.push(&[(2, 1.0)], 1.0);
                                                    s.is_empty()
                                                }
                                                _ => false,
                                            }
                                        }
                                        ['i', s @ ..] => {
                                            out.push(&[(1, 0.98080415), (39, 0.019012742)], 0.9998169);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['e', 'c', '-', 'o', 'i', 'a', 'i', 'c', 'c', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            out.push(&[(1, 0.99253833), (2, 0.0022430762), (91, 0.0022278172), (118, 0.0022278172)], 0.99923706);
                            match s {
                                [] => true,
                                ['r', 'a', 'i', s @ ..] => {
                                    out.push(&[(1, 0.7970092), (2, 0.20090029)], 0.99790955);
                                    match s {
                                        [] => true,
                                        ['h', 'c', 'n', 'i', 'm', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', 'r', 'a', 'p', 'i', s @ ..] => {
                                    out.push(&[(1, 0.7970092), (91, 0.20090029)], 0.99790955);
                                    match s {
                                        [] => true,
                                        ['d', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(91, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['v', 'o', 'm', 'o', s @ ..] => {
                                    out.push(&[(1, 0.746273), (118, 0.25111774)], 0.99739075);
                                    match s {
                                        [] => true,
                                        ['p', 'a', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(118, 1.0)], 1.0);
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
                ['v', s @ ..] => {
                    out.push(&[(1, 0.6158999), (2, 0.2987106), (0, 0.04260319), (15, 0.04260319)], 0.9998169);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['l', 'g', 'a', 't', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['n', 'g', 'e', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 't', 's', 'o', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['n', 'a', 'p', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['u', 'i', 'f', '\u{1}', s @ ..] => {
                                            out.push(&[(15, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['e', 'a', 'i', 'c', 'c', 'a', 'r', 't', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['g', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 't', 'o', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
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
        ['i', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'c', 'a', 'd', 'l', 'a', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['u', s @ ..] => {
            out.push(&[(1, 0.89352256), (2, 0.053528648), (238, 0.05262837)], 0.99967957);
            match s {
                [] => true,
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(238, 1.0)], 1.0);
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

/// Words ending -moto (3 syllables).
pub(crate) fn n1943(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', 'r', s @ ..] => {
            match s {
                [] => true,
                ['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rto (3 syllables).
pub(crate) fn n862(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9172656), (2, 0.027725643), (35, 0.013702602), (123, 0.013702602), (130, 0.013702602), (178, 0.013702602)], 0.99980164);
    match s {
        [] => true,
        ['a', 'p', s @ ..] => {
            match s {
                [] => true,
                ['e', 'r', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(130, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(123, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
                    out.push(&[(1, 0.58753335), (2, 0.20103762), (35, 0.19569695), (123, 0.0052185855), (130, 0.0052185855), (178, 0.0052185855)], 0.9999237);
                    match s {
                        [] => true,
                        ['a', 'c', 'c', 'e', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'b', 'b', 'a', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(35, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', 'a', 'd', 'r', 'a', 'u', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'o', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(178, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -sto (3 syllables).
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.97265583), (2, 0.0076752882), (15, 0.006027314), (48, 0.006027314), (128, 0.006027314)], 0.9984131);
    match s {
        [] => true,
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['c', 'l', 'a', 'l', 'l', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'n', 'i', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'f', 'e', 'm', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'p', 'o', s @ ..] => {
            out.push(&[(1, 0.657557), (15, 0.16867323), (128, 0.16867323), (2, 0.0025635157), (48, 0.002014191)], 0.9994812);
            match s {
                [] => true,
                ['n', 'o', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(128, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'i', 'g', 'n', 'i', '\u{1}', s @ ..] => {
            out.push(&[(48, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tto (3 syllables).
pub(crate) fn n864(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9660029), (2, 0.021606775), (15, 0.0040589), (0, 0.0036316472)], 0.9953002);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(1, 0.9491264), (2, 0.024063477), (0, 0.01873808), (105, 0.006195163)], 0.99812317);
            match s {
                [] => true,
                ['g', 'a', 'z', 'z', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'p', 'p', 'o', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'f', 's', 'a', 'l', 'l', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(1, 0.9760586), (2, 0.013595789)], 0.98965436);
            match s {
                [] => true,
                ['c', 'c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['g', 'i', 'p', 'm', 'o', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'a', 'r', 'b', 'b', 'e', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', s @ ..] => {
                    out.push(&[(1, 0.9726406), (2, 0.009216449), (39, 0.008972305), (184, 0.008972305)], 0.99980164);
                    match s {
                        [] => true,
                        ['-', 'o', 'n', 'a', 'v', 'i', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(184, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'd', 'n', 'e', 'c', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', s @ ..] => {
                    out.push(&[(1, 0.97965974), (104, 0.019424735)], 0.9990845);
                    match s {
                        [] => true,
                        ['i', 'b', 'a', 'g', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(104, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', 'o', s @ ..] => {
                    out.push(&[(1, 0.5181506), (2, 0.47698176)], 0.9951324);
                    match s {
                        [] => true,
                        ['i', 'p', 'p', 'o', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'o', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    out.push(&[(1, 0.9757687), (0, 0.017547876), (92, 0.0058442056)], 0.99916077);
                    match s {
                        [] => true,
                        ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 's', 'o', 'r', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    out.push(&[(1, 0.9144579), (15, 0.081635766), (2, 0.0022125582)], 0.9983062);
                    match s {
                        [] => true,
                        ['a', 'z', 'n', 'e', 's', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', 'r', s @ ..] => {
            match s {
                [] => true,
                ['f', 'a', 'l', 'o', 'c', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'v', 'i', 'r', 'c', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['l', 'a', 'n', 'e', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 0.5263142), (0, 0.47368583)], 1.0);
                    s.is_empty()
                }
                ['s', 'a', s @ ..] => {
                    out.push(&[(1, 0.68987566), (15, 0.29602504), (2, 0.010162509)], 0.99606323);
                    match s {
                        [] => true,
                        ['c', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'c', 's', 'i', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['u', s @ ..] => {
            out.push(&[(1, 0.68882275), (2, 0.25862515), (15, 0.051697567)], 0.99914545);
            match s {
                [] => true,
                ['t', s @ ..] => {
                    out.push(&[(2, 0.6130312), (1, 0.26417944), (15, 0.12260624)], 0.9998169);
                    match s {
                        [] => true,
                        ['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
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

/// Words ending -uto (3 syllables).
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.9073014), (2, 0.06175326), (0, 0.015411613), (15, 0.0153505765)], 0.9998169);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(2, 0.35391775), (1, 0.30243382), (0, 0.17180133), (15, 0.17178607)], 0.99993896);
            s.is_empty()
        }
        ['c', 'a', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['n', 'e', 'v', 'n', 'e', 'b', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word biocinetica (3 syllables).
pub(crate) fn n4141(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// The word termocinetica (3 syllables).
pub(crate) fn n4142(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// The word sottotraccia (3 syllables).
pub(crate) fn n4126(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// The word autotraccia (3 syllables).
pub(crate) fn n4127(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// The word camiscia (3 syllables).
pub(crate) fn n2098(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 1.0)], 1.0);
    s.is_empty()
}

/// The word dietrocoscia (3 syllables).
pub(crate) fn n2099(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(0, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -atiuscia (3 syllables).
pub(crate) fn n2100(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['c', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['k', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -fiducia (3 syllables).
pub(crate) fn n1003(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['s', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -dia (3 syllables).
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9554284), (2, 0.021881437), (107, 0.010833906), (195, 0.010833906)], 0.99897766);
    match s {
        [] => true,
        ['e', 'm', s @ ..] => {
            out.push(&[(6, 0.7118486), (2, 0.28189516), (107, 0.0029907683), (195, 0.0029907683)], 0.99972534);
            match s {
                [] => true,
                ['i', 't', 'l', 'u', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'e', 'p', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'i', 'd', 'o', 'c', 'i', 'f', '\u{1}', s @ ..] => {
            out.push(&[(195, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'a', 'u', 'g', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(107, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -fia (3 syllables).
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9871977), (0, 0.00624094), (9, 0.0031128405), (113, 0.0031128405)], 0.9996643);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
                    out.push(&[(6, 0.59487295), (0, 0.20250247), (113, 0.20125124)], 0.99862665);
                    match s {
                        [] => true,
                        ['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(113, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'g', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'r', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 's', 'o', s @ ..] => {
                            out.push(&[(6, 0.74360263), (9, 0.25156024), (0, 0.0031280995)], 0.99829096);
                            match s {
                                [] => true,
                                ['d', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
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

/// Words ending -gia (3 syllables).
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9381552), (16, 0.0533608), (2, 0.0032654307), (0, 0.0026092927)], 0.9973907);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['f', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
                                    out.push(&[(16, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['l', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(16, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', s @ ..] => {
                                    out.push(&[(6, 0.6529183), (16, 0.3451896)], 0.9981079);
                                    match s {
                                        [] => true,
                                        ['b', 'e', 'r', 'e', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(16, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['e', 't', 'e', '\u{1}', s @ ..] => {
                                            out.push(&[(16, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['u', '\u{1}', s @ ..] => {
                                            out.push(&[(16, 1.0)], 1.0);
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
                        ['a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['p', 'o', 'i', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'o', s @ ..] => {
                            match s {
                                [] => true,
                                ['l', 'a', 'f', 'n', 'o', '\u{1}', s @ ..] => {
                                    out.push(&[(16, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['t', 'a', 'm', 'o', 't', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(16, 1.0)], 1.0);
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
                ['i', 'l', 'i', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(16, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'p', 'o', 'r', 'f', 'e', 'n', '\u{1}', s @ ..] => {
                    out.push(&[(16, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            out.push(&[(16, 0.7322652), (6, 0.121248186), (2, 0.08119325), (180, 0.064621955)], 0.99932855);
            match s {
                [] => true,
                ['e', 'r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(180, 0.81632715), (16, 0.18367285)], 1.0);
                    s.is_empty()
                }
                ['o', 'i', 'p', 'a', 'r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['u', 's', 'i', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', 'p', s @ ..] => {
            out.push(&[(6, 0.7190814), (2, 0.25163653), (16, 0.02668803)], 0.99740595);
            match s {
                [] => true,
                ['a', 'i', 'g', 'i', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'a', 'n', s @ ..] => {
            match s {
                [] => true,
                ['i', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(16, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(16, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            out.push(&[(6, 0.5524071), (16, 0.44335088)], 0.99575794);
            match s {
                [] => true,
                ['a', 'r', 'f', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(16, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'r', 'u', 't', '\u{1}', s @ ..] => {
                    out.push(&[(16, 0.6666667), (6, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(6, 0.9943542), (0, 0.003524834)], 0.997879);
            match s {
                [] => true,
                ['l', s @ ..] => {
                    out.push(&[(6, 0.9946441), (0, 0.0035706111)], 0.9982147);
                    match s {
                        [] => true,
                        ['o', s @ ..] => {
                            out.push(&[(6, 0.9945068), (0, 0.0036621653)], 0.99816895);
                            match s {
                                [] => true,
                                ['c', s @ ..] => {
                                    out.push(&[(6, 0.9839475), (38, 0.015869383)], 0.9998169);
                                    match s {
                                        [] => true,
                                        ['a', 'r', 't', 'a', 'b', '\u{1}', s @ ..] => {
                                            out.push(&[(38, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', 'a', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['m', 'e', 'n', '\u{1}', s @ ..] => {
                                            out.push(&[(0, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['p', 'o', 'c', 'a', 'm', 'r', 'a', 'f', '\u{1}', s @ ..] => {
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
        ['r', s @ ..] => {
            out.push(&[(6, 0.7295949), (16, 0.27006942)], 0.9996643);
            match s {
                [] => true,
                ['e', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', s @ ..] => {
                            out.push(&[(16, 0.7438315), (6, 0.2560769)], 0.99990845);
                            s.is_empty()
                        }
                        ['l', 'l', 'a', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', 's', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['g', 'a', 'p', 'p', 'a', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 's', 'u', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'h', 'c', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['l', 'l', 'a', 't', 'e', 'm', 'o', 'm', 'r', 'e', 't', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'i', 'l', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            out.push(&[(6, 0.96620125), (16, 0.033752955)], 0.9999542);
                            s.is_empty()
                        }
                        ['t', 'i', 'l', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(16, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['u', 'l', 'f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
            out.push(&[(16, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -hia (3 syllables).
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.96017396), (2, 0.025833525), (15, 0.01280232)], 0.9988098);
    match s {
        [] => true,
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['c', 'a', 'm', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'i', s @ ..] => {
                    out.push(&[(6, 0.640116), (15, 0.34186313), (2, 0.017227435)], 0.99920654);
                    match s {
                        [] => true,
                        ['m', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['c', 'a', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(15, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['g', 'a', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(15, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['s', 's', 'a', 'c', 's', '\u{1}', s @ ..] => {
                                            out.push(&[(15, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['i', 'p', 'm', 'o', 'r', '\u{1}', s @ ..] => {
                                    out.push(&[(15, 1.0)], 1.0);
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
        ['g', 'n', 'i', 'c', 'i', 'd', 'n', 'e', 't', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -kia (3 syllables).
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.7551537), (45, 0.10000763), (46, 0.10000763), (16, 0.028503854), (2, 0.007583734), (0, 0.0028992142)], 0.99415576);
    match s {
        [] => true,
        ['n', 'i', 'm', s @ ..] => {
            match s {
                [] => true,
                ['a', 'b', 'm', 'i', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(45, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'b', 'm', 'i', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(46, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -lia (3 syllables).
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9610132), (2, 0.026321813), (98, 0.0029144732), (124, 0.0029144732), (143, 0.0029144732), (191, 0.0029144732)], 0.9989929);
    match s {
        [] => true,
        ['a', 'b', '-', 'o', 't', 'a', 't', 's', '\u{1}', s @ ..] => {
            out.push(&[(282, 1.0)], 1.0);
            s.is_empty()
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['p', s @ ..] => {
                            out.push(&[(6, 0.6116274), (2, 0.3859617)], 0.9975891);
                            match s {
                                [] => true,
                                ['a', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['i', 'c', 'n', 'i', 'r', 't', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['r', 'o', 'f', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['s', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['t', 'i', 'r', 't', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                _ => false,
                            }
                        }
                        ['v', 'o', 't', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 'v', 'i', 'm', 'r', 'o', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'a', 'g', 'e', 'i', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['h', 'c', 'a', 's', 'l', 'a', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(191, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'a', 'f', 'o', s @ ..] => {
                            out.push(&[(6, 0.7305104), (98, 0.25145343), (2, 0.013153277)], 0.9951171);
                            match s {
                                [] => true,
                                ['p', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(98, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'd', s @ ..] => {
                            out.push(&[(6, 0.7844053), (143, 0.20115969), (2, 0.010528726)], 0.9960937);
                            match s {
                                [] => true,
                                ['a', 'u', 'q', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(143, 1.0)], 1.0);
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
                        ['a', 'i', 'c', 'n', 'i', 'r', 't', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'a', 'i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 't', s @ ..] => {
                    out.push(&[(6, 0.7844053), (124, 0.20115969), (2, 0.010528726)], 0.9960937);
                    match s {
                        [] => true,
                        ['t', 'a', 'p', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(124, 1.0)], 1.0);
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

/// Words ending -mia (3 syllables).
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.99279773), (9, 0.0042877854), (10, 0.002136263)], 0.9992218);
    match s {
        [] => true,
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['r', 'c', s @ ..] => {
                    out.push(&[(6, 0.8992752), (9, 0.05043107), (10, 0.050217442)], 0.9999237);
                    match s {
                        [] => true,
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', 'r', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 't', 'e', 'v', '\u{1}', s @ ..] => {
                                    out.push(&[(10, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['t', 'o', 't', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['ì', 'h', 'c', 'l', 'a', '\u{1}', s @ ..] => {
            out.push(&[(25, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nia (3 syllables).
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9853208), (2, 0.01077287), (9, 0.0035706111)], 0.9996643);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['i', s @ ..] => {
                    out.push(&[(6, 0.5941253), (2, 0.4043183)], 0.9984436);
                    match s {
                        [] => true,
                        ['t', 's', 'i', 'r', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['k', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['m', 'o', 'f', 'u', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', 'g', 'r', 'i', 'v', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -pia (3 syllables).
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.99626154), (215, 0.0033569848)], 0.99961853);
    match s {
        [] => true,
        ['o', 'c', 'a', 'l', 'a', 'm', '\u{1}', s @ ..] => {
            out.push(&[(215, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ria (3 syllables).
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.98367286), (2, 0.007583734), (0, 0.0032501717), (15, 0.002166781)], 0.9966735);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['m', s @ ..] => {
                    out.push(&[(6, 0.70962083), (2, 0.14502175), (0, 0.14378576)], 0.99842834);
                    match s {
                        [] => true,
                        ['e', 'v', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'n', 'g', 'a', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'o', 'i', 's', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(6, 0.9883421), (2, 0.009292744), (180, 0.0023193713)], 0.9999542);
            match s {
                [] => true,
                ['h', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'a', 'b', 'a', 't', '-', 'r', 'a', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            out.push(&[(6, 0.7470817), (2, 0.25232318)], 0.9994049);
                            match s {
                                [] => true,
                                ['a', 'i', 'b', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['a', 'g', 'u', 'i', 'c', 's', 'a', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['i', 'd', 'n', 'e', 't', 's', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
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
                        ['r', 'o', s @ ..] => {
                            out.push(&[(6, 0.74416727), (180, 0.2511635), (2, 0.004654002)], 0.9999848);
                            match s {
                                [] => true,
                                ['p', '\u{1}', s @ ..] => {
                                    out.push(&[(180, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', 'a', 'b', 'a', 'c', 'i', 'r', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(6, 0.9803006), (2, 0.009842069), (163, 0.009689479)], 0.99983215);
            match s {
                [] => true,
                ['l', 'g', 'a', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(163, 1.0)], 1.0);
                    s.is_empty()
                }
                ['m', 'e', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            out.push(&[(6, 0.49184406), (2, 0.25378805), (15, 0.25108722)], 0.99671936);
            match s {
                [] => true,
                ['i', 'c', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 't', 't', 'o', 's', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(6, 0.99021894), (15, 0.00483711), (9, 0.004806592)], 0.9998626);
            match s {
                [] => true,
                ['a', 'p', 'a', 'z', 'n', 'e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['e', 'm', 'o', 'n', 'o', 'r', 'c', 'o', 'e', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', 'd', 'i', s @ ..] => {
            out.push(&[(6, 0.7418326), (0, 0.25162128), (2, 0.0037842374)], 0.9972381);
            match s {
                [] => true,
                ['c', 'a', 'o', 'n', 'i', 'm', 'm', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -sia (3 syllables).
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9805142), (2, 0.008773938), (0, 0.0057526515), (175, 0.0045777066)], 0.99961853);
    match s {
        [] => true,
        ['n', s @ ..] => {
            out.push(&[(6, 0.6601663), (2, 0.16958877), (0, 0.16858168)], 0.9983368);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'c', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['s', 'e', 'r', 'o', 'i', 'm', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tia (3 syllables).
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.987335), (0, 0.0060425727), (15, 0.005996796)], 0.9993744);
    match s {
        [] => true,
        ['i', 'l', 'o', 'p', 'a', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['s', 'e', 'b', s @ ..] => {
            out.push(&[(6, 0.6845197), (15, 0.3113756), (0, 0.0037232013)], 0.9996185);
            match s {
                [] => true,
                ['b', 'a', 'k', 'n', 'u', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The word alleluia (3 syllables).
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -via (3 syllables).
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.8102388), (2, 0.11740291), (4, 0.046784163), (0, 0.02355993)], 0.9979858);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['c', 'l', 'a', 'v', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'g', 'e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'p', 'a', 'c', 's', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', 's', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 'c', 'o', 'r', 'c', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'r', 'r', 'e', 'f', 'a', 'c', 'l', 'a', 'v', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -zia (3 syllables).
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9853513), (0, 0.014053559)], 0.9994049);
    match s {
        [] => true,
        ['a', 'b', 'b', 'a', 'i', 'c', 'r', 'a', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', 'l', 'l', 'o', 'm', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'e', 'b', 'n', 'a', 'p', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -mamma (3 syllables).
pub(crate) fn n2232(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(9, 0.51389337), (6, 0.37120622), (15, 0.07692073), (2, 0.027557794), (20, 0.0045471885), (175, 0.0045471885)], 0.9986725);
    match s {
        [] => true,
        ['i', 't', 't', 'o', 'f', '\u{1}', s @ ..] => {
            out.push(&[(15, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'e', 'n', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -ramma (3 syllables).
pub(crate) fn n2233(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(9, 0.9128252), (6, 0.060807202), (2, 0.00920119), (20, 0.008560311), (175, 0.008560311)], 0.9999542);
    match s {
        [] => true,
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['i', 'n', 'a', 'l', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'c', 'a', 'c', 's', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['l', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'f', 'e', 'c', 'n', 'e', 'o', 'e', 'r', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['u', 'c', 'o', 'o', 'r', 't', 't', 'e', 'l', 'e', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['m', 'o', 'n', '\u{1}', s @ ..] => {
                            out.push(&[(175, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['p', 'i', 'r', 'o', 'u', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['t', s @ ..] => {
                            match s {
                                [] => true,
                                ['e', 'i', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 'f', 'o', 'r', 'e', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
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

/// Words ending -emma (3 syllables).
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['g', 'l', 'a', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['l', 'i', 'r', 'v', 'e', 'n', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'a', 'm', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['t', 'f', 'o', 'p', 'a', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -oma (3 syllables).
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(9, 0.82975507), (6, 0.16961929)], 0.9993744);
    match s {
        [] => true,
        ['e', 'd', 'i', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['g', 'a', 's', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', 'g', 'n', 'a', s @ ..] => {
            match s {
                [] => true,
                ['f', 'n', 'i', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'f', 'n', 'i', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', s @ ..] => {
            match s {
                [] => true,
                ['o', 'c', 'r', 'e', 'b', 'u', 't', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', 's', 'n', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            match s {
                [] => true,
                ['e', 'd', 'a', 'o', 'f', 'n', 'i', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'n', 'e', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    out.push(&[(6, 0.792401), (9, 0.2074464)], 0.9998474);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 'i', 'v', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'c', s @ ..] => {
            out.push(&[(6, 0.6678416), (9, 0.331899)], 0.9997406);
            s.is_empty()
        }
        ['s', 'o', s @ ..] => {
            match s {
                [] => true,
                ['l', 'c', 'i', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'i', 'r', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'n', 'i', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['h', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['s', 'i', 'p', 'o', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
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
                ['a', 'r', 'e', s @ ..] => {
                    match s {
                        [] => true,
                        ['h', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'c', 'o', 'r', 't', 's', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'g', 'i', 'l', 'o', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'a', 'x', 'o', 'r', 'b', 'i', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'l', 'b', 'o', s @ ..] => {
                            match s {
                                [] => true,
                                ['l', 'u', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 'd', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', 't', 'i', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['z', 'a', 'i', 'd', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rma (3 syllables).
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.46433204), (9, 0.41220722), (2, 0.041748684), (59, 0.020202946), (71, 0.020202946), (171, 0.020202946), (286, 0.020202946)], 0.99909973);
    match s {
        [] => true,
        ['a', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(59, 1.0)], 1.0);
            s.is_empty()
        }
        ['e', s @ ..] => {
            out.push(&[(9, 0.60073245), (6, 0.33116657), (2, 0.032837417), (286, 0.03152514)], 0.9962616);
            match s {
                [] => true,
                ['d', s @ ..] => {
                    out.push(&[(9, 0.95564204), (6, 0.036789503), (2, 0.0036469062), (286, 0.003509575)], 0.999588);
                    s.is_empty()
                }
                ['f', s @ ..] => {
                    out.push(&[(6, 0.7077897), (9, 0.1501793), (286, 0.13287556), (2, 0.008209354)], 0.99905396);
                    match s {
                        [] => true,
                        ['a', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
                            out.push(&[(286, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['h', 'o', 'i', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 's', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'g', 'n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'm', 'i', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'o', s @ ..] => {
                    match s {
                        [] => true,
                        ['r', 'e', 'x', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(6, 0.7637446), (2, 0.08334478), (171, 0.08003357), (9, 0.063416496), (59, 0.0031128405), (71, 0.0031128405), (286, 0.0031128405)], 0.999878);
            match s {
                [] => true,
                ['f', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['s', 's', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(171, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'u', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['u', 'i', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(71, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -sma (3 syllables).
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(9, 0.9101091), (6, 0.08711375)], 0.99722284);
    match s {
        [] => true,
        ['a', 'l', 'p', 'o', 'l', 'a', 'i', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', 's', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['u', 'c', 'a', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -uma (3 syllables).
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.3737087), (2, 0.28334478), (9, 0.20036621), (0, 0.083894104), (36, 0.0555581)], 0.9968719);
    match s {
        [] => true,
        ['a', 'r', 't', 'o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['c', 'u', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'u', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['h', 'c', 's', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'i', 'c', 'c', 'o', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'n', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'n', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['g', 'a', 'b', '\u{1}', s @ ..] => {
                                    out.push(&[(36, 0.6666667), (2, 0.33333334)], 1.0);
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
                        ['a', 'm', 'm', 'o', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['s', 't', 'a', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word proscinèma (3 syllables).
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 1.0)], 1.0);
    s.is_empty()
}

/// The word aiscrìma (3 syllables).
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(24, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -ita (3 syllables).
pub(crate) fn n508(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.5027085), (9, 0.21066606), (3, 0.16531624), (4, 0.08265812), (2, 0.033844512), (20, 0.004745556)], 0.99993896);
    match s {
        [] => true,
        ['a', 'l', 'o', 'c', 'i', 'n', '\u{1}', s @ ..] => {
            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
            s.is_empty()
        }
        ['b', s @ ..] => {
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(3, 0.436408), (4, 0.218204), (9, 0.19604792), (6, 0.13867399), (2, 0.009338521)], 0.9986724);
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', 'a', 'i', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'e', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'e', 'v', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['c', s @ ..] => {
            out.push(&[(9, 0.5520256), (6, 0.41870755), (3, 0.01855497), (4, 0.009277485)], 0.9985656);
            match s {
                [] => true,
                ['c', 'o', 's', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['e', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'i', 'v', s @ ..] => {
                    match s {
                        [] => true,
                        ['\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['r', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['x', 'a', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    out.push(&[(9, 0.81487757), (6, 0.18355078)], 0.99842834);
                    match s {
                        [] => true,
                        ['i', 'v', 'o', 'n', 'i', 's', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'u', 'e', 'n', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'p', 'e', 'r', 'd', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['e', 'l', 'p', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'b', 'i', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'a', 'p', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    out.push(&[(6, 0.9050889), (9, 0.09011979), (3, 0.0030365454)], 0.99824524);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', s @ ..] => {
            out.push(&[(6, 0.6544137), (3, 0.1681239), (4, 0.08406195), (2, 0.0773022), (9, 0.01574731)], 0.99964905);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['i', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 'a', 'f', 'e', 's', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['u', 'q', 'c', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['v', 'a', 'l', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', 'a', 'z', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    out.push(&[(6, 0.95063704), (3, 0.0240177), (4, 0.01200885), (2, 0.011047532), (9, 0.0022430762)], 0.9999542);
                    s.is_empty()
                }
                ['o', 'l', 'g', 'o', 'r', 't', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['r', 'a', 'f', 'e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['u', 'a', 's', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['f', s @ ..] => {
            out.push(&[(6, 0.84727246), (9, 0.061692227), (3, 0.059372853), (4, 0.029694056)], 0.9980316);
            match s {
                [] => true,
                ['a', 'l', 'a', 's', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['e', 'n', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'p', 'a', 's', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'a', 'm', 'r', 'e', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 's', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['g', s @ ..] => {
            match s {
                [] => true,
                ['a', 'p', 'o', 'e', 'r', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', 'e', 'z', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
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
                        ['n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(20, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'f', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['b', 'o', 'r', 'i', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['e', 'a', s @ ..] => {
                    match s {
                        [] => true,
                        ['m', 's', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['r', 's', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['t', 's', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['z', 'a', 't', 'u', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', 'e', 'a', 'f', 'f', 'a', 'r', 'e', 'r', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['b', 'a', 't', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['e', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['p', 'o', s @ ..] => {
                            match s {
                                [] => true,
                                ['m', 's', 'o', 'c', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 't', 'e', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', 't', 't', 'e', 'l', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'a', 'm', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
                            out.push(&[(9, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', 'o', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['m', s @ ..] => {
            out.push(&[(3, 0.45531395), (6, 0.2464332), (4, 0.22764935), (9, 0.066971846), (2, 0.0031891356)], 0.9995575);
            match s {
                [] => true,
                ['a', 'l', 'a', s @ ..] => {
                    out.push(&[(6, 0.6985733), (3, 0.18211642), (4, 0.09106585), (9, 0.026779583)], 0.99853516);
                    s.is_empty()
                }
                ['r', 'o', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', s @ ..] => {
            out.push(&[(6, 0.54569316), (3, 0.28775463), (4, 0.14387731), (9, 0.019150073), (2, 0.0030823224)], 0.9995575);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'd', 'a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['m', 'o', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['e', 'm', 'e', 'y', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['i', 'e', 'r', 'h', 'a', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['n', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 's', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['u', 's', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['n', 'n', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            out.push(&[(6, 0.87795836), (9, 0.051697567), (3, 0.04057374), (4, 0.02027924), (2, 0.008300908)], 0.9988098);
            s.is_empty()
        }
        ['r', s @ ..] => {
            match s {
                [] => true,
                ['a', 'b', 'i', 's', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['c', 'o', 'p', 'i', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['d', s @ ..] => {
                    out.push(&[(9, 0.68427557), (6, 0.20108339), (3, 0.06611734), (4, 0.0330663), (2, 0.013534753)], 0.99807733);
                    s.is_empty()
                }
                ['e', 'd', 'b', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['i', 'g', 'a', 't', 's', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['o', 'i', 'g', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['t', 's', 'e', 'l', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            out.push(&[(9, 0.43654537), (3, 0.25474936), (6, 0.17679103), (4, 0.12736705), (2, 0.003982605)], 0.9994354);
            match s {
                [] => true,
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['u', 'q', 'n', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['v', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'a', 'r', 'a', 'p', 'o', s @ ..] => {
                    out.push(&[(9, 0.83901733), (3, 0.072785534), (6, 0.050507363), (4, 0.036392767)], 0.998703);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(6, 0.80363166), (2, 0.13559167), (9, 0.027634088), (3, 0.02168307), (4, 0.010833906)], 0.9993744);
            match s {
                [] => true,
                ['a', 'm', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'a', 'p', 'o', 'p', 'o', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', s @ ..] => {
            match s {
                [] => true,
                ['n', 's', 'i', 'v', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['s', 'e', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['v', s @ ..] => super::t1::n1302(s, out),
        ['z', s @ ..] => super::t1::n1303(s, out),
        _ => false,
    }
}

/// Words ending -nta (3 syllables).
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.66225684), (2, 0.12780957), (9, 0.073594265), (15, 0.042511635), (19, 0.03181506), (232, 0.02121004), (236, 0.02121004), (3, 0.012817578), (4, 0.006408789)], 0.9996338);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['f', 'o', 'c', 'i', 's', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', 'o', 'c', 'i', 's', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['r', 'e', 'z', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', s @ ..] => {
                    out.push(&[(19, 0.44675365), (6, 0.37842375), (2, 0.07302968), (9, 0.042053863), (15, 0.024292363), (232, 0.012115664), (236, 0.012115664), (3, 0.0073243305), (4, 0.0036621653)], 0.9997711);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            match s {
                [] => true,
                ['g', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'o', 'p', 'a', 'i', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['m', 'l', s @ ..] => {
                    out.push(&[(9, 0.5367971), (6, 0.3311208), (2, 0.063904785), (15, 0.021255817), (19, 0.0158999), (232, 0.01060502), (236, 0.01060502), (3, 0.006408789), (4, 0.0032043946)], 0.99980164);
                    s.is_empty()
                }
                ['p', 's', 'i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(236, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'd', 'n', 'o', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['o', 'r', 'p', 'm', 'i', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['u', 'p', 'a', s @ ..] => {
            match s {
                [] => true,
                ['t', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(232, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -lota (3 syllables).
pub(crate) fn n1310(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['i', 'p', 'o', s @ ..] => {
            out.push(&[(9, 0.56505686), (6, 0.24644846), (3, 0.10167086), (4, 0.05084306), (2, 0.026901655), (268, 0.00889601)], 0.9998169);
            match s {
                [] => true,
                ['c', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['l', 'a', 'b', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nota (3 syllables).
pub(crate) fn n1311(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'm', 'i', 'r', 'p', '\u{1}', s @ ..] => {
            out.push(&[(268, 1.0)], 1.0);
            s.is_empty()
        }
        ['o', 'c', 'n', 'a', 'b', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rota (3 syllables).
pub(crate) fn n1312(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', 'c', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'a', 'g', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -uota (3 syllables).
pub(crate) fn n1313(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.45162126), (6, 0.35410085), (3, 0.08714427), (9, 0.055771723), (4, 0.04357977), (268, 0.007614252)], 0.99983215);
    match s {
        [] => true,
        ['q', 'i', 'l', 'a', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        ['r', 'o', 't', 'o', 'm', '\u{1}', s @ ..] => {
            out.push(&[(6, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rta (3 syllables).
pub(crate) fn n512(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.69671166), (2, 0.1322652), (9, 0.034729533), (15, 0.026428627), (0, 0.026321813), (138, 0.026321813), (202, 0.026321813), (3, 0.01591516), (4, 0.00796521), (20, 0.0065766387)], 0.99955744);
    match s {
        [] => true,
        ['a', 'c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['e', s @ ..] => {
            out.push(&[(6, 0.9119707), (0, 0.0576791), (2, 0.014496071), (9, 0.0037994965), (15, 0.0028992142), (138, 0.0028839551), (202, 0.0028839551)], 0.9966125);
            match s {
                [] => true,
                ['l', 'l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(6, 0.47869077), (15, 0.21057451), (138, 0.21052873), (2, 0.05290303), (9, 0.01388571), (0, 0.010528726), (202, 0.010528726), (3, 0.0063630124), (4, 0.0031891356), (20, 0.0026245518)], 0.9998169);
                    match s {
                        [] => true,
                        ['a', 'b', 'm', 'a', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(138, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['m', 'a', 't', 't', 'a', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(202, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    out.push(&[(2, 0.4373846), (6, 0.37367818), (9, 0.13533227), (20, 0.02699321), (15, 0.005416953), (0, 0.0054016938), (138, 0.0054016938), (202, 0.0054016938), (3, 0.0032654307)], 0.99827576);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'c', 'o', 'b', '\u{1}', s @ ..] => {
                                    out.push(&[(20, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', 'p', 'o', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['p', 'a', 'r', s @ ..] => {
                            match s {
                                [] => true,
                                ['p', 'o', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(9, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['v', 'o', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
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

/// Words ending -sta (3 syllables).
pub(crate) fn n513(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6154574), (4, 0.3079576), (6, 0.040054932), (9, 0.018799115), (2, 0.010116732), (15, 0.0022888533)], 0.9946746);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(9, 0.28398564), (3, 0.2682536), (2, 0.20616464), (4, 0.13414206), (6, 0.05539025), (175, 0.051285572)], 0.99922174);
            match s {
                [] => true,
                ['p', s @ ..] => {
                    out.push(&[(2, 0.6303349), (175, 0.15750362), (9, 0.08113222), (3, 0.07664607), (4, 0.038330663), (6, 0.015823606)], 0.9997711);
                    match s {
                        [] => true,
                        ['a', 'p', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(175, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(6, 0.5665217), (2, 0.17868315), (15, 0.111314565), (3, 0.054703593), (149, 0.04444953), (4, 0.027374685), (165, 0.0148165105)], 0.9978637);
            match s {
                [] => true,
                ['c', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['d', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'a', 'b', 'm', 'a', 'g', '\u{1}', s @ ..] => {
                    out.push(&[(15, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            out.push(&[(2, 0.67147326), (6, 0.22661173), (15, 0.044525824), (3, 0.021881437), (149, 0.017776761), (4, 0.010955978), (165, 0.0059205005)], 0.9991455);
                            s.is_empty()
                        }
                        ['i', 'g', 'g', 'e', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(15, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(149, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', s @ ..] => {
            out.push(&[(3, 0.64008546), (4, 0.3202716), (6, 0.017914092), (9, 0.015045396), (2, 0.002121004)], 0.99543756);
            match s {
                [] => true,
                ['c', 'i', 's', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['d', 'o', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['g', 'g', 'e', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', s @ ..] => {
                    out.push(&[(3, 0.64846265), (4, 0.3242237), (6, 0.011505303), (9, 0.008545052), (112, 0.0037842374), (0, 0.003418021)], 0.99993896);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['n', 'a', 'g', 'o', 'd', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['e', s @ ..] => {
                            out.push(&[(3, 0.51802856), (4, 0.2590219), (9, 0.21927215), (6, 0.0022583352)], 0.99858093);
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'n', 'a', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(6, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['p', s @ ..] => {
                                    out.push(&[(3, 0.5017166), (4, 0.2508583), (112, 0.12753491), (0, 0.11479362), (6, 0.0029144732), (9, 0.002166781)], 0.9999847);
                                    match s {
                                        [] => true,
                                        ['a', 'c', '\u{1}', s @ ..] => {
                                            out.push(&[(112, 0.5263142), (0, 0.47368583)], 1.0);
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
                    out.push(&[(3, 0.64887464), (4, 0.32568857), (9, 0.015197986), (6, 0.0039063096), (84, 0.0025177386), (223, 0.0025177386)], 0.998703);
                    match s {
                        [] => true,
                        ['e', 'c', 's', 'o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            out.push(&[(3, 0.65148395), (4, 0.32579538), (223, 0.014526589), (224, 0.0072022583)], 0.9990082);
                            match s {
                                [] => true,
                                ['n', 'e', 'l', '-', 'a', 't', 's', 'i', 'x', 'r', 'a', 'm', '\u{1}', s @ ..] => {
                                    out.push(&[(223, 0.6666667), (224, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', 'r', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(84, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['p', s @ ..] => {
                    out.push(&[(3, 0.54695964), (4, 0.27348745), (6, 0.08235294), (2, 0.04913405), (9, 0.03320363), (15, 0.014724956)], 0.9998627);
                    match s {
                        [] => true,
                        ['a', 'r', 'e', 't', 'o', 't', 'r', 'o', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            out.push(&[(3, 0.4465553), (2, 0.240177), (4, 0.22328527), (15, 0.072053105), (6, 0.012771801), (9, 0.0051422906)], 0.99998474);
                            match s {
                                [] => true,
                                ['r', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['o', 'u', 'f', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['p', 'a', '\u{1}', s @ ..] => {
                                            out.push(&[(2, 0.5263142), (15, 0.47368583)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', 't', 'a', 'b', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['o', s @ ..] => {
                            out.push(&[(3, 0.40247196), (6, 0.37860686), (4, 0.20123598), (2, 0.008926528), (9, 0.0060425727), (15, 0.0026703288)], 0.9999542);
                            match s {
                                [] => true,
                                ['c', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                            s.is_empty()
                                        }
                                        ['s', s @ ..] => {
                                            match s {
                                                [] => true,
                                                ['\u{1}', s @ ..] => {
                                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                                    s.is_empty()
                                                }
                                                ['o', 'r', s @ ..] => {
                                                    match s {
                                                        [] => true,
                                                        ['c', 'i', 'm', '\u{1}', s @ ..] => super::t1::n4143(s, out),
                                                        ['t', 't', 'e', 'p', 's', '\u{1}', s @ ..] => super::t1::n4144(s, out),
                                                        _ => false,
                                                    }
                                                }
                                                _ => false,
                                            }
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', 'u', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    out.push(&[(3, 0.6344243), (4, 0.3172198), (9, 0.027435722), (6, 0.015548944), (53, 0.005294881)], 0.99992365);
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'g', 'n', 'i', 'l', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['à', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(53, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['t', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['e', 'm', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'q', s @ ..] => {
                    out.push(&[(3, 0.47840086), (6, 0.27707332), (4, 0.23926146), (9, 0.0036469062)], 0.99838257);
                    match s {
                        [] => true,
                        ['a', 'r', 't', 'u', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'b', 'u', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        ['n', s @ ..] => {
                            match s {
                                [] => true,
                                ['a', 'l', 'b', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['u', 'l', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['v', s @ ..] => {
                    out.push(&[(3, 0.53579), (4, 0.26790264), (6, 0.15213245), (9, 0.022278173), (44, 0.021622034)], 0.99972534);
                    match s {
                        [] => true,
                        ['a', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(44, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', s @ ..] => {
                            match s {
                                [] => true,
                                ['r', s @ ..] => {
                                    match s {
                                        [] => true,
                                        ['\u{1}', s @ ..] => {
                                            out.push(&[(6, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        ['o', 'n', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                                            out.push(&[(6, 1.0)], 1.0);
                                            s.is_empty()
                                        }
                                        _ => false,
                                    }
                                }
                                ['t', s @ ..] => {
                                    out.push(&[(3, 0.6542), (4, 0.32710764), (6, 0.014480812), (9, 0.002121004), (44, 0.002059968)], 0.9999694);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['r', s @ ..] => {
                            out.push(&[(6, 0.6148928), (3, 0.24832532), (4, 0.12416266), (9, 0.0063630124), (44, 0.006179904)], 0.99992365);
                            match s {
                                [] => true,
                                ['e', 's', 'i', 'r', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(6, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(6, 0.6272984), (2, 0.18918136), (3, 0.0892958), (9, 0.04882887), (4, 0.044663157)], 0.9992676);
            match s {
                [] => true,
                ['a', 'l', 'l', 'u', 'n', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                ['c', 'o', 'g', 'n', 'u', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(9, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'g', 'a', 'i', 'd', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'n', 'a', 'b', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['m', 'r', 'e', 'f', '\u{1}', s @ ..] => {
                                    out.push(&[(2, 1.0)], 1.0);
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
            out.push(&[(6, 0.76728463), (3, 0.14920272), (4, 0.074662395), (9, 0.004562448), (2, 0.0024567025)], 0.9981689);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -tta (3 syllables).
pub(crate) fn n514(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.9686122), (2, 0.008377203), (3, 0.005432212), (0, 0.0044098576), (17, 0.003921569), (4, 0.002716106)], 0.9934691);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(6, 0.8289311), (3, 0.056595713), (2, 0.042816814), (169, 0.04209964), (4, 0.028305486)], 0.9987487);
            match s {
                [] => true,
                ['d', s @ ..] => {
                    out.push(&[(6, 0.41446555), (3, 0.3616388), (4, 0.1808194), (2, 0.021408407), (169, 0.021057451)], 0.99938965);
                    match s {
                        [] => true,
                        ['i', 'd', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['m', 'a', 's', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(169, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', 'a', 'r', 'c', 'a', 'm', 'r', 'e', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(2, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(6, 0.9908751), (20, 0.0023956664), (175, 0.0023956664)], 0.99566644);
            match s {
                [] => true,
                ['i', 'h', 'c', 'c', 'o', s @ ..] => {
                    out.push(&[(6, 0.74543375), (20, 0.25119403)], 0.9966278);
                    match s {
                        [] => true,
                        ['n', 'a', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(20, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['l', s @ ..] => {
                    out.push(&[(6, 0.98385596), (3, 0.008484016), (4, 0.004242008), (2, 0.0031891356)], 0.9997711);
                    match s {
                        [] => true,
                        ['c', 'i', 'c', 'i', 'b', 'a', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'm', 'm', 'a', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['n', 'g', 'u', 'p', 'a', 'z', 'z', 'e', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(231, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', 'i', 'c', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(6, 0.9337453), (17, 0.040161747), (0, 0.020172426), (3, 0.003555352)], 0.9976348);
            match s {
                [] => true,
                ['c', s @ ..] => {
                    out.push(&[(6, 0.5734951), (17, 0.41606775), (0, 0.008072022)], 0.9976349);
                    match s {
                        [] => true,
                        ['a', s @ ..] => {
                            match s {
                                [] => true,
                                ['r', 'r', 'e', 't', '\u{1}', s @ ..] => {
                                    out.push(&[(17, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['u', 'q', 'c', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(17, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                ['i', 'r', 't', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
                    s.is_empty()
                }
                ['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
                    out.push(&[(0, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['u', s @ ..] => {
            out.push(&[(6, 0.63065535), (2, 0.2742504), (0, 0.09170672)], 0.9966124);
            match s {
                [] => true,
                ['r', s @ ..] => {
                    out.push(&[(2, 0.59140915), (6, 0.21022354), (0, 0.19723812)], 0.9988708);
                    s.is_empty()
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Words ending -uta (3 syllables).
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(6, 0.5734951), (3, 0.24226749), (4, 0.121141374), (9, 0.047959104), (0, 0.014450294)], 0.99931335);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(3, 0.5523003), (4, 0.2761425), (6, 0.08656443), (9, 0.082703896), (0, 0.0021820401)], 0.9998931);
            s.is_empty()
        }
        ['e', s @ ..] => {
            out.push(&[(3, 0.5167315), (4, 0.25836575), (6, 0.112993054), (9, 0.11030747)], 0.99839777);
            match s {
                [] => true,
                ['l', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'e', 't', 't', 'a', 'r', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['p', 'a', 's', 'n', 'i', '\u{1}', s @ ..] => {
            out.push(&[(0, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -lio (3 syllables).
pub(crate) fn n780(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.7454185), (1, 0.21055925), (2, 0.021316854), (40, 0.006118868), (48, 0.0059052417), (0, 0.003540093), (78, 0.003540093), (192, 0.003540093)], 0.99993896);
    match s {
        [] => true,
        ['a', 'c', s @ ..] => {
            match s {
                [] => true,
                ['a', 'r', 'b', 's', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(13, 0.97659266), (1, 0.0193637)], 0.99595636);
            s.is_empty()
        }
        ['g', s @ ..] => {
            out.push(&[(13, 0.91961545), (2, 0.021911956), (1, 0.019302662), (40, 0.012542916), (48, 0.012085145), (78, 0.0072480356), (192, 0.0072480356)], 0.9999542);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    match s {
                        [] => true,
                        ['g', 'a', 'b', 'i', 'r', 'p', 'o', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'm', 'e', 'r', 'p', 's', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'o', 's', 'l', 'a', 'f', '\u{1}', s @ ..] => {
                            out.push(&[(192, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['i', s @ ..] => {
                    match s {
                        [] => true,
                        ['c', s @ ..] => {
                            out.push(&[(13, 0.75634396), (48, 0.22545205), (2, 0.0058442056), (1, 0.0051422906), (40, 0.0033417258)], 0.9961242);
                            match s {
                                [] => true,
                                ['c', 'a', 'r', 'p', 'o', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(48, 0.6666667), (13, 0.33333334)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 'e', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(48, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                _ => false,
                            }
                        }
                        ['p', 's', 'i', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['f', 'i', 't', 't', 'e', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['v', 'n', 'o', 'c', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                            out.push(&[(78, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['i', 'b', 'i', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['l', s @ ..] => {
            out.push(&[(1, 0.58646524), (13, 0.4103914)], 0.99685663);
            match s {
                [] => true,
                ['a', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'e', s @ ..] => {
                            match s {
                                [] => true,
                                ['l', 'a', 'p', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['n', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
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
                        ['p', 'i', 'c', 'i', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', s @ ..] => {
                            match s {
                                [] => true,
                                ['b', 'u', 's', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['i', 'b', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
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
                        ['c', 'i', 'n', 'e', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['d', 'i', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['p', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'e', 'b', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(1, 0.60021365), (13, 0.33727017), (2, 0.04652476), (0, 0.015396353)], 0.9994049);
            match s {
                [] => true,
                ['a', s @ ..] => {
                    out.push(&[(2, 0.5155108), (13, 0.2790875), (1, 0.2000763), (0, 0.0051270314)], 0.99980164);
                    match s {
                        [] => true,
                        ['r', 'a', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['c', 'c', 'o', 'z', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'l', 'a', 't', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                ['m', 'o', 'r', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    out.push(&[(13, 0.6950942), (1, 0.21115434), (0, 0.085893035), (2, 0.007751583)], 0.9998932);
                    match s {
                        [] => true,
                        ['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(0, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['s', 'i', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', s @ ..] => {
                    match s {
                        [] => true,
                        ['a', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'e', 'p', s @ ..] => {
                            match s {
                                [] => true,
                                ['\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['o', 'u', 'l', 'f', 'e', 'r', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
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
                        ['a', 'g', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', s @ ..] => {
                            match s {
                                [] => true,
                                ['c', 'r', 'a', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
                                    s.is_empty()
                                }
                                ['r', '\u{1}', s @ ..] => {
                                    out.push(&[(13, 1.0)], 1.0);
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
                        ['s', 'o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['t', 'o', 's', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
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

/// Words ending -inio (3 syllables).
pub(crate) fn n1799(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', 'r', 'u', 't', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['c', s @ ..] => {
            match s {
                [] => true,
                ['a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'p', 's', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['f', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['g', 'u', 'l', 'a', 'b', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['m', s @ ..] => {
            match s {
                [] => true,
                ['e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['l', 'u', 'f', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['o', 'g', 's', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 'u', 'i', 'c', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['v', 'o', 'r', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -nnio (3 syllables).
pub(crate) fn n1800(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['e', 't', 'n', 'e', 't', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', 't', 'n', 'i', 't', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -onio (3 syllables).
pub(crate) fn n1801(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.99308765), (2, 0.003921569), (1, 0.002792401)], 0.99980164);
    match s {
        [] => true,
        ['r', 't', 'i', 'r', 'e', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -oio (3 syllables).
pub(crate) fn n782(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.9940337), (2, 0.005386435)], 0.9994201);
    match s {
        [] => true,
        ['u', 'c', 'l', 'i', 'm', 'i', 's', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -pio (3 syllables).
pub(crate) fn n783(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['o', s @ ..] => {
            out.push(&[(13, 0.99853516)], 0.99853516);
            s.is_empty()
        }
        ['r', 'a', 'i', 'c', 'c', 'a', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        ['s', 'a', 'n', 'n', 'a', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -rio (3 syllables).
pub(crate) fn n784(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.9698329), (1, 0.024628062), (2, 0.002136263)], 0.99659723);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(13, 0.9921721), (1, 0.0052338447), (40, 0.0025635157)], 0.9999695);
            match s {
                [] => true,
                ['f', 'f', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'e', 'g', 'a', 'u', 't', 'p', 'e', 's', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['z', s @ ..] => {
                    out.push(&[(13, 0.7968719), (40, 0.20102236), (1, 0.002090486)], 0.99998474);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', s @ ..] => {
            out.push(&[(13, 0.9013962), (1, 0.098420694)], 0.9998169);
            match s {
                [] => true,
                ['f', 'i', 'c', 'o', 'v', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['i', 'h', 'c', 'c', 'a', 'i', 'h', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 'i', 'c', 's', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['r', 'e', 'p', 's', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 's', 'a', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', s @ ..] => {
                    out.push(&[(13, 0.9954986), (1, 0.0045014117)], 1.0);
                    s.is_empty()
                }
                ['v', s @ ..] => {
                    out.push(&[(1, 0.549218), (13, 0.45070574)], 0.9999237);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', s @ ..] => {
            out.push(&[(13, 0.930663), (1, 0.05822843), (108, 0.010986496)], 0.9998779);
            match s {
                [] => true,
                ['g', s @ ..] => {
                    out.push(&[(1, 0.5291218), (13, 0.46532387), (108, 0.005493248)], 0.9999389);
                    s.is_empty()
                }
                ['m', s @ ..] => {
                    match s {
                        [] => true,
                        ['r', 'o', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['u', 'r', '\u{1}', s @ ..] => {
                            out.push(&[(1, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'i', 'b', 's', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                ['t', 'a', 'r', 'o', 'b', 'a', 'l', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(108, 1.0)], 1.0);
                    s.is_empty()
                }
                ['v', 'a', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(1, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', s @ ..] => {
            out.push(&[(13, 0.69169146), (2, 0.29512474), (1, 0.011596857)], 0.998413);
            match s {
                [] => true,
                ['o', 'r', 'p', s @ ..] => {
                    match s {
                        [] => true,
                        ['o', 't', 'o', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['u', 't', 'o', 'm', '\u{1}', s @ ..] => {
                            out.push(&[(2, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        ['r', 'u', 's', 's', 'u', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word patassio (3 syllables).
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 1.0)], 1.0);
    s.is_empty()
}

/// Words ending -tio (3 syllables).
pub(crate) fn n786(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(1, 0.63750666), (13, 0.2955062), (0, 0.044205386), (267, 0.022095064)], 0.9993133);
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(0, 0.5221027), (1, 0.31876096), (13, 0.1477531), (267, 0.011047532)], 0.9996643);
            s.is_empty()
        }
        ['c', 'o', s @ ..] => {
            match s {
                [] => true,
                ['l', 'i', 'n', 'n', 'u', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['d', 'a', 't', 's', 'm', 'r', 'a', 'd', '\u{1}', s @ ..] => {
            out.push(&[(13, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            match s {
                [] => true,
                ['l', s @ ..] => {
                    match s {
                        [] => true,
                        ['i', 'd', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['o', 'n', 'a', 'g', 'r', 'o', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['s', 'o', 'p', '\u{1}', s @ ..] => {
                    out.push(&[(267, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['n', 'e', s @ ..] => {
            match s {
                [] => true,
                ['p', s @ ..] => {
                    match s {
                        [] => true,
                        ['l', 'i', 'n', 'n', 'u', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['r', 'w', 'a', 'l', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['p', 'e', 's', s @ ..] => {
            match s {
                [] => true,
                ['l', 'i', 'n', 'n', 'u', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                ['n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['s', s @ ..] => {
            match s {
                [] => true,
                ['o', s @ ..] => {
                    match s {
                        [] => true,
                        ['d', 'n', 'e', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        ['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
                            out.push(&[(13, 1.0)], 1.0);
                            s.is_empty()
                        }
                        _ => false,
                    }
                }
                ['u', 'a', 'l', 'a', 'b', '\u{1}', s @ ..] => {
                    out.push(&[(13, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['t', s @ ..] => {
            out.push(&[(1, 0.95797664), (13, 0.034256503), (0, 0.0051270314), (267, 0.0025635157)], 0.9999237);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -uio (3 syllables).
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['b', 'o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['q', 'c', 'a', 'l', 'a', 'i', 'c', 's', '\u{1}', s @ ..] => {
            out.push(&[(1, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -zio (3 syllables).
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(13, 0.9089342), (1, 0.06381323), (2, 0.009140154), (40, 0.008987564), (140, 0.008987564)], 0.9998627);
    match s {
        [] => true,
        ['a', 'c', 'u', 'b', '\u{1}', s @ ..] => {
            out.push(&[(2, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', s @ ..] => {
            out.push(&[(13, 0.9744411), (140, 0.02200351), (1, 0.002761883)], 0.9992065);
            match s {
                [] => true,
                ['v', 'r', 'e', 's', 'o', 'p', 'a', 'c', '\u{1}', s @ ..] => {
                    out.push(&[(140, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['z', s @ ..] => {
            out.push(&[(1, 0.7919585), (13, 0.20198368), (2, 0.00202945)], 0.9959716);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -vita (3 syllables).
pub(crate) fn n1302(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(2, 0.40886548), (6, 0.32965592), (9, 0.116975665), (3, 0.08207828), (4, 0.04104677), (20, 0.02136263)], 0.99998474);
    match s {
        [] => true,
        ['a', s @ ..] => {
            match s {
                [] => true,
                ['l', 'a', 'm', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['u', 'q', 'c', 'a', '\u{1}', s @ ..] => {
                    out.push(&[(20, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['e', 'l', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        ['i', 'm', 'e', s @ ..] => {
            match s {
                [] => true,
                ['\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                ['s', '\u{1}', s @ ..] => {
                    out.push(&[(6, 1.0)], 1.0);
                    s.is_empty()
                }
                _ => false,
            }
        }
        ['o', 'c', 's', 'o', 'm', '\u{1}', s @ ..] => {
            out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// Words ending -zita (3 syllables).
pub(crate) fn n1303(s: &[char], out: &mut Path<'_>) -> bool {
    match s {
        [] => true,
        ['a', s @ ..] => {
            out.push(&[(3, 0.46611735), (4, 0.2330663), (6, 0.20108339), (9, 0.08427558), (2, 0.013534753)], 0.9980774);
            s.is_empty()
        }
        ['e', 'p', 'a', 'r', 't', '\u{1}', s @ ..] => {
            out.push(&[(9, 1.0)], 1.0);
            s.is_empty()
        }
        _ => false,
    }
}

/// The word microscopista (3 syllables).
pub(crate) fn n4143(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
    s.is_empty()
}

/// The word spettroscopista (3 syllables).
pub(crate) fn n4144(s: &[char], out: &mut Path<'_>) -> bool {
    out.push(&[(3, 0.6666667), (4, 0.33333334)], 1.0);
    s.is_empty()
}

