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
    // sz+i → s+i: 798 words (pierwszy, pieszy, lepszy, teraźniejszy), 0 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("sz+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("s+i"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("sz+i")]),
    },
    // c+i → c+y: 620 words (mulczujący, lśniący, zwierzęcy, obcy), 0 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("c+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("c+y"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("c+i")]),
    },
    // t+i → +ci: 424 words (party, wart, złoty, pity), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("t+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ci"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("t+i")]),
    },
    // on+i → +eni: 440 words (spragniony, zmęczony, ograniczony, uzależniony), 30 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("on+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("+eni"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("on+i")]),
    },
    // z+i → z+y: 331 words (uroczy, tajemniczy, rolniczy, oszukańczy), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("z+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("z+y"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("z+i")]),
    },
    // s → ś after [a|e|i|o|u|y|ę]; z → ź after a in alternating stems (ˇ), before +[c|n]i, +l, .+i: 251 words (prosty, pusty, ciasny, jedenasty), 0 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[
                    B("ę"),
                    B("a"),
                    B("e"),
                    B("i"),
                    B("o"),
                    B("u"),
                    B("y"),
                ])),
                quantifier: Quantifier::One,
                capture: Some(0),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("s"), B("z")])),
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
                                matcher: Matcher::Class(B(&[B("c"), B("n")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("i")])),
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
                                matcher: Matcher::Class(B(&[B("l")])),
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
        replacement: B(&[ReplacementPart::AgreeingCapture {
            capture: 1,
            key: 0,
            entries: B(&[
                (B("s"), B("a"), B("ś¦")),
                (B("s"), B("e"), B("ś¦")),
                (B("s"), B("i"), B("ś¦")),
                (B("s"), B("o"), B("ś¦")),
                (B("s"), B("u"), B("ś¦")),
                (B("s"), B("y"), B("ś¦")),
                (B("s"), B("ę"), B("ś¦")),
                (B("z"), B("a"), B("ź¦")),
                (B("z"), B("e"), B("z")),
                (B("z"), B("i"), B("z")),
                (B("z"), B("o"), B("z")),
                (B("z"), B("u"), B("z")),
                (B("z"), B("y"), B("z")),
                (B("z"), B("ę"), B("z")),
            ]),
        }]),
        capture_count: 2,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // ł+i → +li: 248 words (ciepły, miły, oziębły, prostopadły), 0 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ł+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("+li"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ł+i")]),
    },
    // s+ci → +ści: 9 words (przezroczysty, szósty, czysty, przejrzysty), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("s+ci")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ści"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("s+ci")]),
    },
    // ∅ → z after [d|r] in alternating stems (ˇ), before +e..., +i: 17 words (hardy, młody, twardy, chudy), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("d"), B("r")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[]),
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
                                matcher: Matcher::Class(B(&[B("i")])),
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
        replacement: B(&[ReplacementPart::Literal(B("z¦"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // r+i → r+zy: 30 words (bury, dobry, chory, stary), 0 forms wrong.
    CompiledRule {
        name: B("law9"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("r+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("r+zy"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("r+i")]),
    },
    // s+l → +śl: 6 words (dorosły, wzniosły, rosły, wyniosły), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("s+l")),
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
        replacement: B(&[ReplacementPart::Literal(B("+śl"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("s+l")]),
    },
    // ł+s → +lejs: 16 words (ciepły, okrągły, szczupły, rychły), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ł+s")),
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
        replacement: B(&[ReplacementPart::Literal(B("+lejs"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ł+s")]),
    },
    // ch+i → +si: 16 words (cichy, suchy, głuchy, kruchy), 0 forms wrong.
    CompiledRule {
        name: B("law12"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ch+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("+si"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ch+i")]),
    },
    // os+l → o+śl: 6 words (dorosły, wzniosły, rosły, wyniosły), 0 forms wrong.
    CompiledRule {
        name: B("law13"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("os+l")),
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
        replacement: B(&[ReplacementPart::Literal(B("o+śl"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("os+l")]),
    },
    // ż+i → ż+y: 13 words (świeży, boży, hoży, chyży), 0 forms wrong.
    CompiledRule {
        name: B("law14"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ż+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("ż+y"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ż+i")]),
    },
    // [t|w|n|d]+m → +ym: 31 words (syt, odkryty, szyty, uszyty), 0 forms wrong.
    CompiledRule {
        name: B("law15"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("t"), B("w"), B("n"), B("d")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+m")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
            ]),
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
        replacement: B(&[
            ReplacementPart::Capture(0),
            ReplacementPart::Literal(B("+ym")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+m")]),
    },
    // n+s → +ńs: 6 words (tani, szczwany, cwany, wyrafinowany), 0 forms wrong.
    CompiledRule {
        name: B("law16"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("n+s")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ńs"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("n+s")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
