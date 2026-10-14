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
    // I → ı after a; i after e, i; u after o, u; ü after none; E → a after a, o, u; e after e, i, none (the nearest of a, e, i, o, u before it, as written): 55 words (sik, bay, alp, fino), 0 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("a"), B("e"), B("i"), B("o"), B("u")])),
                    quantifier: Quantifier::Optional,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[B("a"), B("e"), B("i"), B("o"), B("u")])),
                    quantifier: Quantifier::ZeroOrMore,
                    capture: Some(1),
                },
            ]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("I"), B("E")])),
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
                (B("I"), B(""), B("ü")),
                (B("E"), B("a"), B("a")),
                (B("E"), B("e"), B("e")),
                (B("E"), B("i"), B("e")),
                (B("E"), B("o"), B("a")),
                (B("E"), B("u"), B("a")),
                (B("E"), B(""), B("e")),
            ]),
        }]),
        capture_count: 3,
        direction: Direction::LeftToRight,
        application: Application::Iterative,
        required: B(&[]),
    },
    // [t|p|k|ç|s]+d → +t: 8 words (sik, alp, iç, çift), 0 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("ç"), B("t"), B("p"), B("k"), B("s")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+d")),
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
            ReplacementPart::Literal(B("+t")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+d")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
