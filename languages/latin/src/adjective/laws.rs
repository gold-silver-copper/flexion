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
    // ē → e after [c|d|i|l|m|p|r|t|u] in alternating stems (ˇ), before +.., .+..: 186 words (quadrupēs, volucripēs, recēns, clēmēns), 2 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[
                    B("c"),
                    B("d"),
                    B("i"),
                    B("l"),
                    B("m"),
                    B("p"),
                    B("r"),
                    B("t"),
                    B("u"),
                ])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ē")),
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
        replacement: B(&[ReplacementPart::Literal(B("e¦"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ē"), B("+"), B("ˇ")]),
    },
    // ēn+t → +ent: 13 words (ēns, libēns, lubēns, ingēns), 0 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ēn+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ent"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ēn+t")]),
    },
    // x+em → +cem: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+em")),
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
        replacement: B(&[ReplacementPart::Literal(B("+cem"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+em")]),
    },
    // x+i → +ci: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 6 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+i")),
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
        required: B(&[B("x+i")]),
    },
    // x+ē → +cē: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+ē")),
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
        replacement: B(&[ReplacementPart::Literal(B("+cē"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+ē")]),
    },
    // ān+t → +ant: 53 words (īnfāns, ignicomāns, ēlegāns, auricomāns), 0 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ān+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ant"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ān+t")]),
    },
    // ā → a after [c|d|l|n|p|r|t]; e → ∅ after [b|c|g|t]; o → ō after l in alternating stems (ˇ), before .+[a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ], .+[^a|e|i|o|u|y|ā|ē|ī|ō|ū|ȳ].: 57 words (multicolor, subniger, pār, umber), 42 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[
                    B("b"),
                    B("c"),
                    B("d"),
                    B("g"),
                    B("l"),
                    B("n"),
                    B("p"),
                    B("r"),
                    B("t"),
                ])),
                quantifier: Quantifier::One,
                capture: Some(0),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ā"), B("e"), B("o")])),
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
                                    B("ā"),
                                    B("ē"),
                                    B("ī"),
                                    B("ō"),
                                    B("ū"),
                                    B("ȳ"),
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
                                matcher: Matcher::NotClass(B(&[
                                    B("‹"),
                                    B("ˇ"),
                                    B("¦"),
                                    B("°"),
                                    B("ā"),
                                    B("ē"),
                                    B("ī"),
                                    B("ō"),
                                    B("ū"),
                                    B("ȳ"),
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
                (B("e"), B("b"), B("¦")),
                (B("e"), B("c"), B("¦")),
                (B("e"), B("d"), B("e")),
                (B("e"), B("g"), B("¦")),
                (B("e"), B("l"), B("e")),
                (B("e"), B("n"), B("e")),
                (B("e"), B("p"), B("e")),
                (B("e"), B("r"), B("e")),
                (B("e"), B("t"), B("¦")),
                (B("o"), B("b"), B("o")),
                (B("o"), B("c"), B("o")),
                (B("o"), B("d"), B("o")),
                (B("o"), B("g"), B("o")),
                (B("o"), B("l"), B("ō¦")),
                (B("o"), B("n"), B("o")),
                (B("o"), B("p"), B("o")),
                (B("o"), B("r"), B("o")),
                (B("o"), B("t"), B("o")),
                (B("ā"), B("b"), B("ā")),
                (B("ā"), B("c"), B("a¦")),
                (B("ā"), B("d"), B("a¦")),
                (B("ā"), B("g"), B("ā")),
                (B("ā"), B("l"), B("a¦")),
                (B("ā"), B("n"), B("a¦")),
                (B("ā"), B("p"), B("a¦")),
                (B("ā"), B("r"), B("a¦")),
                (B("ā"), B("t"), B("a¦")),
            ]),
        }]),
        capture_count: 2,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // e+c → +ic: 34 words (complex, simplex, duplex, multiplex), 29 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("e+c")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ic"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("e+c")]),
    },
    // ōn+t → +ont: 14 words (sōns, īnsōns, latifrōns, nigrifrōns), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ōn+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ont"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ōn+t")]),
    },
    // p+t → p+: 5 words (inops, cōps, mīcrōps, macrōps), 0 forms wrong.
    CompiledRule {
        name: B("law9"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("p+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("p+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("p+t")]),
    },
    // x+ī → +cī before ˇ: 5 words (praecox, bilanx, sēminex, hystrix), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+ī")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ˇ")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+cī"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+ī")]),
    },
    // x+īs → +cīs before ˇ: 5 words (praecox, bilanx, sēminex, hystrix), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+īs")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ˇ")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+cīs"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+īs")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
