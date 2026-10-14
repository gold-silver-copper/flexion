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
    // न → ण after र, ष, ृ unless च, ज, ट, ठ, ड, ढ, ण, त, थ, द, ध, न, ल, श, स comes between, not before ्: 1,181 words (वृक्ष, समुद्र, तरंग, तीर), 1 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("र"), B("ष"), B("\u{943}")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("र"),
                        B("ष"),
                        B("\u{943}"),
                        B("च"),
                        B("ज"),
                        B("ट"),
                        B("ठ"),
                        B("ड"),
                        B("ढ"),
                        B("ण"),
                        B("त"),
                        B("थ"),
                        B("द"),
                        B("ध"),
                        B("न"),
                        B("ल"),
                        B("श"),
                        B("स"),
                    ])),
                    quantifier: Quantifier::ZeroOrMore,
                    capture: Some(1),
                },
                Atom {
                    matcher: Matcher::Literal(B("+")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("र"),
                        B("ष"),
                        B("\u{943}"),
                        B("च"),
                        B("ज"),
                        B("ट"),
                        B("ठ"),
                        B("ड"),
                        B("ढ"),
                        B("ण"),
                        B("त"),
                        B("थ"),
                        B("द"),
                        B("ध"),
                        B("न"),
                        B("ल"),
                        B("श"),
                        B("स"),
                    ])),
                    quantifier: Quantifier::ZeroOrMore,
                    capture: Some(2),
                },
            ]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("न")),
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
        excluded_left: Some(Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        }),
        excluded_right: Some(Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("\u{94d}")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        }),
        replacement: B(&[ReplacementPart::Literal(B("ण"))]),
        capture_count: 3,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("न")]),
    },
    // ्म+् → ्म+ after a vowel earlier in the stem: 7 words (अश्मन्, अकर्मन्, आत्मन्, ऊष्मन्), 2 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("अ"),
                        B("आ"),
                        B("इ"),
                        B("ई"),
                        B("उ"),
                        B("ऊ"),
                        B("ऋ"),
                        B("ए"),
                        B("ऐ"),
                        B("ओ"),
                        B("औ"),
                    ])),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("अ"),
                        B("आ"),
                        B("इ"),
                        B("ई"),
                        B("उ"),
                        B("ऊ"),
                        B("ऋ"),
                        B("ए"),
                        B("ऐ"),
                        B("ओ"),
                        B("औ"),
                    ])),
                    quantifier: Quantifier::ZeroOrMore,
                    capture: None,
                },
            ]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("\u{94d}म+\u{94d}")),
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
        replacement: B(&[ReplacementPart::Literal(B("\u{94d}म+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("\u{94d}म+\u{94d}")]),
    },
    // ्म+्न → ्म+न: 13 words (तक्मन्, महात्मन्, वस्मन्, वर्त्मन्), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("\u{94d}म+\u{94d}न")),
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
        replacement: B(&[ReplacementPart::Literal(B("\u{94d}म+न"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("\u{94d}म+\u{94d}न")]),
    },
    // ्म+्ण → ्म+ण: 17 words (चर्मन्, ब्रह्मन्, कर्मन्, मर्मन्), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("\u{94d}म+\u{94d}ण")),
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
        replacement: B(&[ReplacementPart::Literal(B("\u{94d}म+ण"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("\u{94d}म+\u{94d}ण")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
