//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), on a letter a
//! `%` copies (`°`), or on a class letter an ending writes (harmony, by the
//! nearest letter of a tier before it as written): every form a declension builds
//! goes through them, in order, so one declension serves words whose stems change
//! its endings. Each with the words whose forms it gives, and how many of
//! the dictionary's forms it gets wrong (those are listed in `irregular.rs`).

use flexion::sound_law::compiled::*;

const RULES: &[CompiledRule<'static>] = &[
    // I → ı after a, â, ı, none; i after e, i, î; u after o, u, û; ü after ö, ü; A → a after a, o, u, â, û, ı, none; e after e, i, î, ö, ü (the nearest of a, e, i, o, u, â, î, ö, û, ü, ı before it, as written): 10,313 words (pound, ab, abdal, abdest), 1,374 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("â"),
                        B("î"),
                        B("ö"),
                        B("û"),
                        B("ü"),
                        B("ı"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("o"),
                        B("u"),
                    ])),
                    quantifier: Quantifier::Optional,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("â"),
                        B("î"),
                        B("ö"),
                        B("û"),
                        B("ü"),
                        B("ı"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("o"),
                        B("u"),
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
                matcher: Matcher::Class(B(&[B("I"), B("A")])),
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
                (B("I"), B("a"), B("ı")),
                (B("I"), B("e"), B("i")),
                (B("I"), B("i"), B("i")),
                (B("I"), B("o"), B("u")),
                (B("I"), B("u"), B("u")),
                (B("I"), B("â"), B("ı")),
                (B("I"), B("î"), B("i")),
                (B("I"), B("ö"), B("ü")),
                (B("I"), B("û"), B("u")),
                (B("I"), B("ü"), B("ü")),
                (B("I"), B("ı"), B("ı")),
                (B("I"), B(""), B("ı")),
                (B("A"), B("a"), B("a")),
                (B("A"), B("e"), B("e")),
                (B("A"), B("i"), B("e")),
                (B("A"), B("o"), B("a")),
                (B("A"), B("u"), B("a")),
                (B("A"), B("â"), B("a")),
                (B("A"), B("î"), B("e")),
                (B("A"), B("ö"), B("e")),
                (B("A"), B("û"), B("a")),
                (B("A"), B("ü"), B("e")),
                (B("A"), B("ı"), B("a")),
                (B("A"), B(""), B("a")),
            ]),
        }]),
        capture_count: 3,
        direction: Direction::LeftToRight,
        application: Application::Iterative,
        required: B(&[]),
    },
    // n+ğ → n+g: 13 words (renk, cenk, pelenk, denk), 0 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("n+ğ")),
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
        replacement: B(&[ReplacementPart::Literal(B("n+g"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("n+ğ")]),
    },
    // ol+i → ol+ü: 5 words (alkol, metropol, alveol, megapol), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ol+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("ol+ü"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ol+i")]),
    },
    // ul+i → ul+ü: 5 words (kabul, usul, maktul, meful), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ul+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("ul+ü"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ul+i")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
