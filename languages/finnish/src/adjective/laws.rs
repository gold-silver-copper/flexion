//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), on a letter a
//! `%` copies (`°`), on a class letter an ending writes (harmony, by the
//! nearest letter of a tier before it as written), or inside a stem that
//! alternates (`ˇ`, by what follows it): every form a declension builds
//! goes through them, in order, so one declension serves words whose stems change
//! its endings. Each with the words whose forms it gives, and how many of
//! the dictionary's forms it gets wrong (those are listed in `irregular.rs`).

use flexion::sound_law::compiled::*;

const RULES: &[CompiledRule<'static>] = &[
    // A → ä after -, y, z, ä, ö, none; a after a, o, u (the nearest of -, a, o, u, y, z, ä, ö before it, as written): 14,176 words (beige, indigo, khaki, vapaa), 687 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("ä"),
                        B("ö"),
                        B("-"),
                        B("a"),
                        B("o"),
                        B("u"),
                        B("y"),
                        B("z"),
                    ])),
                    quantifier: Quantifier::Optional,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("ä"),
                        B("ö"),
                        B("-"),
                        B("a"),
                        B("o"),
                        B("u"),
                        B("y"),
                        B("z"),
                    ])),
                    quantifier: Quantifier::ZeroOrMore,
                    capture: Some(1),
                },
            ]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("A")])),
                quantifier: Quantifier::One,
                capture: Some(2),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::AgreeingCapture {
            capture: 2,
            key: 0,
            entries: B(&[
                (B("A"), B("-"), B("ä")),
                (B("A"), B("a"), B("a")),
                (B("A"), B("o"), B("a")),
                (B("A"), B("u"), B("a")),
                (B("A"), B("y"), B("ä")),
                (B("A"), B("z"), B("ä")),
                (B("A"), B("ä"), B("ä")),
                (B("A"), B("ö"), B("ä")),
                (B("A"), B(""), B("ä")),
            ]),
        }]),
        capture_count: 3,
        direction: Direction::LeftToRight,
        application: Application::Iterative,
        required: B(&[]),
    },
    // I → a after a, i, none; ä after e, y, ä; i after t (the nearest of a, e, i, t, y, ä before it, as written): 4,240 words (sininen, violetti, ensimmäinen, yksinäinen), 9 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("ä"), B("a"), B("e"), B("i"), B("t"), B("y")])),
                    quantifier: Quantifier::Optional,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("ä"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("t"),
                        B("y"),
                    ])),
                    quantifier: Quantifier::ZeroOrMore,
                    capture: Some(1),
                },
            ]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("I")])),
                quantifier: Quantifier::One,
                capture: Some(2),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::AgreeingCapture {
            capture: 2,
            key: 0,
            entries: B(&[
                (B("I"), B("a"), B("a")),
                (B("I"), B("e"), B("ä")),
                (B("I"), B("i"), B("a")),
                (B("I"), B("t"), B("i")),
                (B("I"), B("y"), B("ä")),
                (B("I"), B("ä"), B("ä")),
                (B("I"), B(""), B("a")),
            ]),
        }]),
        capture_count: 3,
        direction: Direction::LeftToRight,
        application: Application::Iterative,
        required: B(&[]),
    },
    // ä+j → +i: 16 words (tuliterä, aikaavievä, kaikkinielevä, hapanimelä), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ä+j")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+i"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ä+j")]),
    },
    // t → ∅ after t; t → l after l; p → ∅ after p in alternating stems (ˇ), before +e.[^a|e|i|o|u|y|ä|ö]$, +e.[^a|e|i|o|u|y|ä|ö].[a|e|i|o|u|y|ä|ö], +i[^a|e|i|o|u|y|ä|ö]$, +i[^a|e|i|o|u|y|ä|ö].., .+i.$, .+i..[a|e|i|o|u|y|ä|ö], .+[k|l|m|s|t], .+n$: 444 words (violetti, rutto, loppu, helppo), 2 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("l"), B("p"), B("t")])),
                quantifier: Quantifier::One,
                capture: Some(0),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("p"), B("t")])),
                quantifier: Quantifier::One,
                capture: Some(1),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Either(B(&[
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("ä"),
                                    B("ö"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("i")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("i")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("i")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("i")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("ä"),
                                    B("ö"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("k"),
                                    B("l"),
                                    B("m"),
                                    B("s"),
                                    B("t"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                ])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::AgreeingCapture {
            capture: 1,
            key: 0,
            entries: B(&[
                (B("p"), B("l"), B("p")),
                (B("p"), B("p"), B("¦")),
                (B("p"), B("t"), B("p")),
                (B("t"), B("l"), B("l¦")),
                (B("t"), B("p"), B("t")),
                (B("t"), B("t"), B("¦")),
            ]),
        }]),
        capture_count: 2,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // i+e → +e: 9 words (makaaberi, obskuuri, populaari, primaari), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("i+e")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+e"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("i+e")]),
    },
    // k → ∅ after [k|r]; t → d after [h|i|ä]; p → v after [i|y] in alternating stems (ˇ), before +[a|i|ä][^a|e|i|o|u|y|ä|ö]$, +[a|i|ä][^a|e|i|o|u|y|ä|ö].., +e[a|e|i|o|u|y|ä|ö]..[a|e|i|o|u|y|ä|ö], +e[^a|e|i|o|u|y|ä|ö].., +o.[d|k|l|s|t], +o.n$, +ö.[d|h|k|l|s|t], +ö.n$, .+i.$, .+i..[a|e|i|o|u|y|ä|ö], .+[k|l|m|s|t], .+n$: 156 words (märkä, mätä, hoikka, heikko), 27 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ä"), B("h"), B("i"), B("k"), B("r"), B("y")])),
                quantifier: Quantifier::One,
                capture: Some(0),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("k"), B("p"), B("t")])),
                quantifier: Quantifier::One,
                capture: Some(1),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Either(B(&[
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("ä"), B("a"), B("i")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("ä"), B("a"), B("i")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("ä"),
                                    B("ö"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("ä"),
                                    B("ö"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("o")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("d"),
                                    B("k"),
                                    B("l"),
                                    B("s"),
                                    B("t"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("o")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("ö")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("d"),
                                    B("h"),
                                    B("k"),
                                    B("l"),
                                    B("s"),
                                    B("t"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("ö")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("i")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("i")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("ä"),
                                    B("ö"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("k"),
                                    B("l"),
                                    B("m"),
                                    B("s"),
                                    B("t"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                ])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::AgreeingCapture {
            capture: 1,
            key: 0,
            entries: B(&[
                (B("k"), B("h"), B("k")),
                (B("k"), B("i"), B("k")),
                (B("k"), B("k"), B("¦")),
                (B("k"), B("r"), B("¦")),
                (B("k"), B("y"), B("k")),
                (B("k"), B("ä"), B("k")),
                (B("p"), B("h"), B("p")),
                (B("p"), B("i"), B("v¦")),
                (B("p"), B("k"), B("p")),
                (B("p"), B("r"), B("p")),
                (B("p"), B("y"), B("v¦")),
                (B("p"), B("ä"), B("p")),
                (B("t"), B("h"), B("d¦")),
                (B("t"), B("i"), B("d¦")),
                (B("t"), B("k"), B("t")),
                (B("t"), B("r"), B("t")),
                (B("t"), B("y"), B("t")),
                (B("t"), B("ä"), B("d¦")),
            ]),
        }]),
        capture_count: 2,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // copy of k → s: 14 words (kahdeskymmenes, yhdeksäskymmenes, neljäskymmenes, kolmaskymmenes), 0 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("°k")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("s"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°k")]),
    },
    // t → n after n in alternating stems (ˇ), before +.[^a|e|i|o|u|y|ä|ö]$, +.[^a|e|i|o|u|y|ä|ö]..: 7 words (ikivihanta, lyhyenläntä, vihanta, vähänläntä), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("n")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("t")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Either(B(&[
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                    Pattern {
                        atoms: B(&[
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ä"),
                                    B("ö"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                    B("y"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("+"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::NotClass(B(&[B("ˇ")])),
                                quantifier: Quantifier::ZeroOrMore,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("ˇ")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                        ]),
                        anchored_at_start: false,
                        anchored_at_end: true,
                    },
                ])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("n¦"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("t"), B("+"), B("ˇ")]),
    },
    // da+a → +taa: 9 words (hidas, puhdas, ahdas, epäpuhdas), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("da+a")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+taa"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("da+a")]),
    },
    // da+i → +tai: 9 words (hidas, puhdas, ahdas, epäpuhdas), 0 forms wrong.
    CompiledRule {
        name: B("law9"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("da+i")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+tai"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("da+i")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
