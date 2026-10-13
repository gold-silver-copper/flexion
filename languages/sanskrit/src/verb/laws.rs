//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), on a letter a
//! `%` copies (`°`), or on a class letter an ending writes (harmony, by the
//! nearest letter of a tier before it as written): every form a conjugation builds
//! goes through them, in order, so one conjugation serves words whose stems change
//! its endings. Each with the words whose forms it gives, and how many of
//! the dictionary's forms it gets wrong (those are listed in `irregular.rs`).

use flexion::sound_law::compiled::*;

const RULES: &[CompiledRule<'static>] = &[
    // A → न after ख, घ, च, छ, ज, ट, ठ, ड, ण, त, द, ध, न, ब, ल, श, स, ी, none; ण after र, ष, ृ (the nearest of ख, घ, च, छ, ज, ट, ठ, ड, ण, त, द, ध, न, ब, र, ल, श, ष, स, ी, ृ before it, as written): 642 words (आद, आप, निन्दति, खादति), 0 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("ख"),
                        B("घ"),
                        B("च"),
                        B("छ"),
                        B("ज"),
                        B("ट"),
                        B("ठ"),
                        B("ड"),
                        B("ण"),
                        B("त"),
                        B("द"),
                        B("ध"),
                        B("न"),
                        B("ब"),
                        B("र"),
                        B("ल"),
                        B("श"),
                        B("ष"),
                        B("स"),
                        B("ी"),
                        B("\u{943}"),
                    ])),
                    quantifier: Quantifier::Optional,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("ख"),
                        B("घ"),
                        B("च"),
                        B("छ"),
                        B("ज"),
                        B("ट"),
                        B("ठ"),
                        B("ड"),
                        B("ण"),
                        B("त"),
                        B("द"),
                        B("ध"),
                        B("न"),
                        B("ब"),
                        B("र"),
                        B("ल"),
                        B("श"),
                        B("ष"),
                        B("स"),
                        B("ी"),
                        B("\u{943}"),
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
                (B("A"), B("ख"), B("न")),
                (B("A"), B("घ"), B("न")),
                (B("A"), B("च"), B("न")),
                (B("A"), B("छ"), B("न")),
                (B("A"), B("ज"), B("न")),
                (B("A"), B("ट"), B("न")),
                (B("A"), B("ठ"), B("न")),
                (B("A"), B("ड"), B("न")),
                (B("A"), B("ण"), B("न")),
                (B("A"), B("त"), B("न")),
                (B("A"), B("द"), B("न")),
                (B("A"), B("ध"), B("न")),
                (B("A"), B("न"), B("न")),
                (B("A"), B("ब"), B("न")),
                (B("A"), B("र"), B("ण")),
                (B("A"), B("ल"), B("न")),
                (B("A"), B("श"), B("न")),
                (B("A"), B("ष"), B("ण")),
                (B("A"), B("स"), B("न")),
                (B("A"), B("ी"), B("न")),
                (B("A"), B("\u{943}"), B("ण")),
                (B("A"), B(""), B("न")),
            ]),
        }]),
        capture_count: 3,
        direction: Direction::LeftToRight,
        application: Application::Iterative,
        required: B(&[]),
    },
    // B → ष after क, र, ि, ी, ु, े, ो, ौ; स after त, म, व, स, ा; ध after द; ज after none (the nearest of क, त, द, म, र, व, स, ा, ि, ी, ु, े, ो, ौ before it, as written): 140 words (भाति, द्राति, धृष्णोति, जानाति), 0 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("क"),
                        B("त"),
                        B("द"),
                        B("म"),
                        B("र"),
                        B("व"),
                        B("स"),
                        B("ा"),
                        B("ि"),
                        B("ी"),
                        B("\u{941}"),
                        B("\u{947}"),
                        B("ो"),
                        B("ौ"),
                    ])),
                    quantifier: Quantifier::Optional,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("क"),
                        B("त"),
                        B("द"),
                        B("म"),
                        B("र"),
                        B("व"),
                        B("स"),
                        B("ा"),
                        B("ि"),
                        B("ी"),
                        B("\u{941}"),
                        B("\u{947}"),
                        B("ो"),
                        B("ौ"),
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
                matcher: Matcher::Class(B(&[B("B")])),
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
                (B("B"), B("क"), B("ष")),
                (B("B"), B("त"), B("स")),
                (B("B"), B("द"), B("ध")),
                (B("B"), B("म"), B("स")),
                (B("B"), B("र"), B("ष")),
                (B("B"), B("व"), B("स")),
                (B("B"), B("स"), B("स")),
                (B("B"), B("ा"), B("स")),
                (B("B"), B("ि"), B("ष")),
                (B("B"), B("ी"), B("ष")),
                (B("B"), B("\u{941}"), B("ष")),
                (B("B"), B("\u{947}"), B("ष")),
                (B("B"), B("ो"), B("ष")),
                (B("B"), B("ौ"), B("ष")),
                (B("B"), B(""), B("ज")),
            ]),
        }]),
        capture_count: 3,
        direction: Direction::LeftToRight,
        application: Application::Iterative,
        required: B(&[]),
    },
    // [ा|े|ो|ू|ए]+ा → +: 6 words (एति, अश्रोत्, अभूत्, अगात्), 0 forms wrong.
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
                        B("ा"),
                        B("\u{947}"),
                        B("ो"),
                        B("\u{942}"),
                        B("ए"),
                    ])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+ा")),
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
        required: B(&[B("+ा")]),
    },
    // अ‹अ → आ: 13 words (अस्ति, अत्ति, अजति, अर्हति), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("अ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("अ")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
            ]),
            anchored_at_start: true,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::MappedCapture {
            capture: 0,
            entries: B(&[(B("अ"), B("आ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("अ‹")]),
    },
    // अ‹उ → औ: 8 words (उच्यति, उज्झति, उभ्नाति, उनत्ति), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("अ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("उ")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
            ]),
            anchored_at_start: true,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::MappedCapture {
            capture: 0,
            entries: B(&[(B("उ"), B("औ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("अ‹")]),
    },
    // [ष|स]+ीष → +: 5 words (अताप्सीत्, अभार्षीत्, अजैषीत्, अवाक्षीत्), 0 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("ष"), B("स")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+ीष")),
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
        required: B(&[B("+ीष")]),
    },
    // अ‹इ → ऐ: 6 words (इच्छति, इङ्गति, इरस्यति, इयर्ति), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("अ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("इ")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
            ]),
            anchored_at_start: true,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::MappedCapture {
            capture: 0,
            entries: B(&[(B("इ"), B("ऐ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("अ‹")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
