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
    // é → è after [b|c|d|f|g|h|i|l|m|n|p|r|s|t] in alternating stems (ˇ), before [c|q].+e$, [c|q].+e[n|s], [c|q].+er., [d|l|n|r|s]+e$, [d|l|n|r|s]+e[n|s], [d|l|n|r|s]+er., g+e$, g+e[n|s], g.+e$, g.+e[n|s], g.+er., m+e$, m+e.., t+e$, t+e[n|s], t+er., t.+e$, t.+e[n|s], t.+er., v.+e$, v.+e..: 227 words (pécher, accélérer, espérer, péter), 0 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[
                    B("b"),
                    B("c"),
                    B("d"),
                    B("f"),
                    B("g"),
                    B("h"),
                    B("i"),
                    B("l"),
                    B("m"),
                    B("n"),
                    B("p"),
                    B("r"),
                    B("s"),
                    B("t"),
                ])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("é")),
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
                                matcher: Matcher::Class(B(&[B("c"), B("q")])),
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
                                matcher: Matcher::Class(B(&[B("c"), B("q")])),
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
                                matcher: Matcher::Class(B(&[B("c"), B("q")])),
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
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                matcher: Matcher::Class(B(&[
                                    B("d"),
                                    B("l"),
                                    B("n"),
                                    B("r"),
                                    B("s"),
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
                                matcher: Matcher::Class(B(&[
                                    B("d"),
                                    B("l"),
                                    B("n"),
                                    B("r"),
                                    B("s"),
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
                                matcher: Matcher::Class(B(&[
                                    B("d"),
                                    B("l"),
                                    B("n"),
                                    B("r"),
                                    B("s"),
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
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                matcher: Matcher::Class(B(&[B("g")])),
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
                                matcher: Matcher::Class(B(&[B("g")])),
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
                                matcher: Matcher::Class(B(&[B("g")])),
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
                                matcher: Matcher::Class(B(&[B("g")])),
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
                                matcher: Matcher::Class(B(&[B("g")])),
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
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                matcher: Matcher::Class(B(&[B("t")])),
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
                                matcher: Matcher::Class(B(&[B("t")])),
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
                                matcher: Matcher::Class(B(&[B("t")])),
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
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("r")])),
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
                                matcher: Matcher::Class(B(&[B("v")])),
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
                                matcher: Matcher::Class(B(&[B("v")])),
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
        replacement: B(&[ReplacementPart::Literal(B("è¦"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("é"), B("+"), B("ˇ")]),
    },
    // g+a → g+ea: 194 words (badger, voyager, manager, singer), 39 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("g+a")),
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
        replacement: B(&[ReplacementPart::Literal(B("g+ea"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("g+a")]),
    },
    // g+â → g+eâ: 194 words (badger, voyager, manager, singer), 9 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("g+â")),
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
        replacement: B(&[ReplacementPart::Literal(B("g+eâ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("g+â")]),
    },
    // ∅ → e after g in alternating stems (ˇ), before +.[^a|e|i|o|u|y|â|æ|è|é|ê|ë|î|ï|ô|û|ü|œ]s: 193 words (badger, voyager, manager, singer), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("g")])),
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
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("+")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[B("‹"), B("ˇ"), B("¦"), B("°"), B("+")])),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("‹"),
                        B("ˇ"),
                        B("¦"),
                        B("°"),
                        B("â"),
                        B("æ"),
                        B("è"),
                        B("é"),
                        B("ê"),
                        B("ë"),
                        B("î"),
                        B("ï"),
                        B("ô"),
                        B("û"),
                        B("ü"),
                        B("œ"),
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
                    matcher: Matcher::Class(B(&[B("s")])),
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
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("e¦"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // c+o → +ço: 131 words (tracer, placer, agacer, forcer), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("c+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ço"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("c+o")]),
    },
    // c+a → +ça: 124 words (tracer, placer, agacer, forcer), 0 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("c+a")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ça"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("c+a")]),
    },
    // c+â → +çâ: 124 words (tracer, placer, agacer, forcer), 0 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("c+â")),
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
        replacement: B(&[ReplacementPart::Literal(B("+çâ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("c+â")]),
    },
    // e → è after [c|g|h|l|m|p|r|s|t|u|v] in alternating stems (ˇ), before +.e$, +.e.., .+e$, .+e[n|s], .+er.: 84 words (lever, élever, peser, acheter), 21 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[
                    B("c"),
                    B("g"),
                    B("h"),
                    B("l"),
                    B("m"),
                    B("p"),
                    B("r"),
                    B("s"),
                    B("t"),
                    B("u"),
                    B("v"),
                ])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("e")),
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
                                matcher: Matcher::Class(B(&[B("e")])),
                                quantifier: Quantifier::One,
                                capture: None,
                            },
                            Atom {
                                matcher: Matcher::Class(B(&[B("r")])),
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
        replacement: B(&[ReplacementPart::Literal(B("è¦"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("e"), B("+"), B("ˇ")]),
    },
    // ri+sa → ri+va: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ri+sa")),
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
        replacement: B(&[ReplacementPart::Literal(B("ri+va"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ri+sa")]),
    },
    // ri+se → ri+ve: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
    CompiledRule {
        name: B("law9"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ri+se")),
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
        replacement: B(&[ReplacementPart::Literal(B("ri+ve"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ri+se")]),
    },
    // ri+si → ri+vi: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ri+si")),
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
        replacement: B(&[ReplacementPart::Literal(B("ri+vi"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ri+si")]),
    },
    // ri+so → ri+vo: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ri+so")),
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
        replacement: B(&[ReplacementPart::Literal(B("ri+vo"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ri+so")]),
    },
    // ri+sî → ri+vî: 15 words (écrire, décrire, prescrire, récrire), 0 forms wrong.
    CompiledRule {
        name: B("law12"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ri+sî")),
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
        replacement: B(&[ReplacementPart::Literal(B("ri+vî"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ri+sî")]),
    },
    // rt+s → r+s: 5 words (partir, sortir, repartir, ressortir), 0 forms wrong.
    CompiledRule {
        name: B("law13"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("rt+s")),
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
        replacement: B(&[ReplacementPart::Literal(B("r+s"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("rt+s")]),
    },
    // rt+u → rt+i: 5 words (partir, sortir, repartir, ressortir), 0 forms wrong.
    CompiledRule {
        name: B("law14"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("rt+u")),
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
        replacement: B(&[ReplacementPart::Literal(B("rt+i"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("rt+u")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
