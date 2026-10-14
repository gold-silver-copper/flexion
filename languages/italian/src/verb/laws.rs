//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), on a letter a
//! `%` copies (`°`), or on a class letter an ending writes (harmony, by the
//! nearest letter of a tier before it as written): every form a conjugation builds
//! goes through them, in order, so one conjugation serves words whose stems change
//! its endings. Each with the words whose forms it gives, and how many of
//! the dictionary's forms it gets wrong (those are listed in `irregular.rs`).

use flexion::sound_law::compiled::*;

const RULES: &[CompiledRule<'static>] = &[
    // i+i → i+: 1,214 words (odiare, studiare, lasciare, invecchiare), 192 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("i+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("i+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("i+i")]),
    },
    // ci+e → c+e: 361 words (lasciare, abbracciare, lanciare, baciare), 12 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ci+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("c+e"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ci+e")]),
    },
    // [d|g|v|r|m|l|t|n]+gg → +: 11 words (valere, sfriggere, contravvalere, disvalere), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("d"),
                        B("g"),
                        B("v"),
                        B("r"),
                        B("m"),
                        B("l"),
                        B("t"),
                        B("n"),
                    ])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+gg")),
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
            ReplacementPart::Literal(B("+")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+gg")]),
    },
    // g+s → +ss: 8 words (affiggere, crocefiggere, crocifiggere, crucifiggere), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("g+s")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ss"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("g+s")]),
    },
    // ec+e → ec+he after a vowel earlier in the stem: 21 words (defecare, deprecare, arrecare, accecare), 12 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("a"), B("e"), B("i"), B("o"), B("u")])),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[B("a"), B("e"), B("i"), B("o"), B("u")])),
                    quantifier: Quantifier::ZeroOrMore,
                    capture: None,
                },
            ]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ec+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("ec+he"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ec+e")]),
    },
    // ec+i → ec+hi: 21 words (defecare, deprecare, arrecare, accecare), 1 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ec+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("ec+hi"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ec+i")]),
    },
    // [l|n]+ss → +s: 7 words (valere, contravvalere, disvalere, equivalere), 0 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("l"), B("n")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+ss")),
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
            ReplacementPart::Literal(B("+s")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+ss")]),
    },
    // on+s → +us: 20 words (fondere, confondere, circonfondere, diffondere), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("on+s")),
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
        replacement: B(&[ReplacementPart::Literal(B("+us"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("on+s")]),
    },
    // al+r → a+r: 6 words (valere, contravvalere, disvalere, equivalere), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("al+r")),
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
        replacement: B(&[ReplacementPart::Literal(B("a+r"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("al+r")]),
    },
    // gl+en → gl+ien: 5 words (inorgoglire, inorgoglirsi, incanaglirsi, orgoglire), 0 forms wrong.
    CompiledRule {
        name: B("law9"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("gl+en")),
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
        replacement: B(&[ReplacementPart::Literal(B("gl+ien"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("gl+en")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
