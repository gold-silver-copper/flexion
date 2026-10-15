//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), on a letter a
//! `%` copies (`°`), on a class letter an ending writes (harmony, by the
//! nearest letter of a tier before it as written), or inside a stem that
//! alternates (`ˇ`, by what follows it): every form a conjugation builds
//! goes through them, in order, so one conjugation serves words whose stems change
//! its endings. Each with the words whose forms it gives, and how many of
//! the dictionary's forms it gets wrong (those are listed in `irregular.rs`).

use flexion::sound_law::compiled::*;

const RULES: &[CompiledRule<'static>] = &[
    // g+e → g+ue: 586 words (llegar, delegar, fatigar, cabalgar), 8 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("g+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("g+ue"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("g+e")]),
    },
    // g+é → g+ué: 585 words (llegar, delegar, fatigar, cabalgar), 0 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("g+é")),
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
        replacement: B(&[ReplacementPart::Literal(B("g+ué"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("g+é")]),
    },
    // ic+e → i+que: 348 words (ubicar, justificar, indicar, identificar), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ic+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("i+que"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ic+e")]),
    },
    // ic+é → i+qué: 348 words (ubicar, justificar, indicar, identificar), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ic+é")),
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
        replacement: B(&[ReplacementPart::Literal(B("i+qué"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ic+é")]),
    },
    // o → ue after [c|d|f|l|m|p|r|s|t|v]; o → üe after g in alternating stems (ˇ), before +.a$, +.a[n|s]$, +.o, b+[a|e|i|o|u|í|ü]$, b+[a|e|i|o|u|í|ü][n|s]$, b.+[a|e|i|o|u|í|ü]$, b.+[a|e|i|o|u|í|ü][n|s]$, [d|ñ]+[a|e|i|o|u|í|ü]$, l+a$, l+a[n|s]$, l+e$, l+e[n|s], l+o, l[d|l]+[a|e|i|o|u|í|ü]$, l[d|l]+[a|e|i|o|u|í|ü][n|s]$, lt+[a|e|i|o|u|í|ü]$, [n|s]+[a|e|i|o|u|í|ü]$, [n|s]+[a|e|i|o|u|í|ü][n|s]$, [n|s]t+[a|e|i|o|u|í|ü]$, [n|s]t+[a|e|i|o|u|í|ü][n|s]$, r+[a|e|i|o|u|í|ü]$, r+[^a|e|i|o|u|í|ü]a$, r+[^a|e|i|o|u|í|ü]a[n|s]$, r+[^a|e|i|o|u|í|ü]o, rd+a$, rd+a[n|s]$, rd+e$, rd+e[n|s], rd+o, rm+[a|e], rn+[a|e|i|o|u|í|ü]$, rn+[a|e|i|o|u|í|ü][n|s]$, rt+[a|e|i|o|u|í|ü]$, v+a$, v+a.$, v+e$: 88 words (sonar, volar, llover, doler), 10 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[
                    B("c"),
                    B("d"),
                    B("f"),
                    B("g"),
                    B("l"),
                    B("m"),
                    B("p"),
                    B("r"),
                    B("s"),
                    B("t"),
                    B("v"),
                ])),
                quantifier: Quantifier::One,
                capture: Some(0),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("o")])),
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
                                matcher: Matcher::Class(B(&[B("a")])),
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
                                matcher: Matcher::Class(B(&[B("a")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("o")])),
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
                                matcher: Matcher::Class(B(&[B("b")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("b")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("b")])),
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
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("b")])),
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
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("ñ"), B("d")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("l")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a")])),
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
                                matcher: Matcher::Class(B(&[B("l")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("l")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
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
                                matcher: Matcher::Class(B(&[B("l")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
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
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("l")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
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
                                matcher: Matcher::Class(B(&[B("l")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("d"), B("l")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("l")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("d"), B("l")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("l")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("t")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("t")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("t")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("+"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("o")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("d")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("d")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("d")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("d")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
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
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("d")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("m")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a"), B("e")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
                                ])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("n"), B("s")])),
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
                                matcher: Matcher::Class(B(&[B("r")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("t")])),
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
                                    B("í"),
                                    B("ü"),
                                    B("a"),
                                    B("e"),
                                    B("i"),
                                    B("o"),
                                    B("u"),
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
                                matcher: Matcher::Class(B(&[B("v")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a")])),
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
                                matcher: Matcher::Class(B(&[B("v")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Literal(B("+")),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("a")])),
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
                                matcher: Matcher::Class(B(&[B("v")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
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
                (B("o"), B("c"), B("ue¦")),
                (B("o"), B("d"), B("ue¦")),
                (B("o"), B("f"), B("ue¦")),
                (B("o"), B("g"), B("üe¦")),
                (B("o"), B("l"), B("ue¦")),
                (B("o"), B("m"), B("ue¦")),
                (B("o"), B("p"), B("ue¦")),
                (B("o"), B("r"), B("ue¦")),
                (B("o"), B("s"), B("ue¦")),
                (B("o"), B("t"), B("ue¦")),
                (B("o"), B("v"), B("ue¦")),
            ]),
        }]),
        capture_count: 2,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // nc+e → n+que: 104 words (roncar, ahincar, hincar, brincar), 4 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("nc+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("n+que"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("nc+e")]),
    },
    // nc+é → n+qué: 104 words (roncar, ahincar, hincar, brincar), 0 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("nc+é")),
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
        replacement: B(&[ReplacementPart::Literal(B("n+qué"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("nc+é")]),
    },
    // rc+e → r+que: 70 words (embarcar, marcar, acercar, recercar), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("rc+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("r+que"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("rc+e")]),
    },
    // rc+é → r+qué: 70 words (embarcar, marcar, acercar, recercar), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("rc+é")),
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
        replacement: B(&[ReplacementPart::Literal(B("r+qué"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("rc+é")]),
    },
    // [ñ|b|l]+ya → +a: 43 words (bullir, plañir, mullir, muñir), 0 forms wrong.
    CompiledRule {
        name: B("law9"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("ñ"), B("b"), B("l")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+ya")),
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
            ReplacementPart::Literal(B("+a")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+ya")]),
    },
    // [ñ|b|l]+yá → +á: 43 words (bullir, plañir, mullir, muñir), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("ñ"), B("b"), B("l")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+yá")),
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
            ReplacementPart::Literal(B("+á")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+yá")]),
    },
    // uc+é → u+qué: 38 words (educar, caducar, machucar, acurrucar), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("uc+é")),
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
        replacement: B(&[ReplacementPart::Literal(B("u+qué"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("uc+é")]),
    },
    // [ñ|l|y|i]+yé → +é: 43 words (bullir, plañir, mullir, muñir), 0 forms wrong.
    CompiledRule {
        name: B("law12"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("ñ"), B("l"), B("y"), B("i")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+yé")),
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
            ReplacementPart::Literal(B("+é")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+yé")]),
    },
    // gu+é → g+üé: 47 words (averiguar, atestiguar, menguar, aguar), 0 forms wrong.
    CompiledRule {
        name: B("law13"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("gu+é")),
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
        replacement: B(&[ReplacementPart::Literal(B("g+üé"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("gu+é")]),
    },
    // zd+ → +d: 37 words (tender, perder, encender, defender), 0 forms wrong.
    CompiledRule {
        name: B("law14"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("zd+")),
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
        replacement: B(&[ReplacementPart::Literal(B("+d"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("zd+")]),
    },
    // ñ+y → ñ+: 26 words (atañer, plañir, muñir, gruñir), 0 forms wrong.
    CompiledRule {
        name: B("law15"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ñ+y")),
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
        replacement: B(&[ReplacementPart::Literal(B("ñ+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ñ+y")]),
    },
    // l+y → l+: 18 words (bullir, mullir, zabullir, çabullir), 0 forms wrong.
    CompiledRule {
        name: B("law16"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("l+y")),
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
        replacement: B(&[ReplacementPart::Literal(B("l+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("l+y")]),
    },
    // lc+e → l+que: 23 words (calcar, revolcar, volcar, remolcar), 0 forms wrong.
    CompiledRule {
        name: B("law17"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("lc+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("l+que"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("lc+e")]),
    },
    // lc+é → l+qué: 23 words (calcar, revolcar, volcar, remolcar), 0 forms wrong.
    CompiledRule {
        name: B("law18"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("lc+é")),
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
        replacement: B(&[ReplacementPart::Literal(B("l+qué"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("lc+é")]),
    },
    // zc+e → z+que: 12 words (pellizcar, pizcar, bizcar, peñizcar), 0 forms wrong.
    CompiledRule {
        name: B("law19"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("zc+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("z+que"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("zc+e")]),
    },
    // zc+é → z+qué: 12 words (pellizcar, pizcar, bizcar, peñizcar), 0 forms wrong.
    CompiledRule {
        name: B("law20"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("zc+é")),
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
        replacement: B(&[ReplacementPart::Literal(B("z+qué"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("zc+é")]),
    },
    // zt+ → +t: 11 words (verter, entrometerse, entremeterse, cometerse), 0 forms wrong.
    CompiledRule {
        name: B("law21"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("zt+")),
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
        replacement: B(&[ReplacementPart::Literal(B("+t"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("zt+")]),
    },
    // c+e → +que before [p|ˇ]: 187 words (caber, buscar, rascar, tocar), 0 forms wrong.
    CompiledRule {
        name: B("law22"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("c+e")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ˇ"), B("p")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+que"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("c+e")]),
    },
    // eg+ue → +iegue before ˇ: 25 words (negar, renegar, fregar, regar), 0 forms wrong.
    CompiledRule {
        name: B("law23"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("eg+ue")),
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
        replacement: B(&[ReplacementPart::Literal(B("+iegue"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("eg+ue")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
