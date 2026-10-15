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
    // η+δ → η+τ: 68 words (μῠκης, κρης, σκληροτης, θης), 0 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("η+δ")),
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
        replacement: B(&[ReplacementPart::Literal(B("η+τ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("η+δ")]),
    },
    // γ+κ → γ+γ: 30 words (μηνῐγξ, φᾰλᾰγξ, σπηλῠγξ, λᾰρῠγξ), 12 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("γ+κ")),
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
        replacement: B(&[ReplacementPart::Literal(B("γ+γ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("γ+κ")]),
    },
    // [ῑ|ῡ|ἰ]+τ → +ν: 10 words (ῥῑς, ἰς, ἀκτῑς, θῑς), 0 forms wrong.
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
                    matcher: Matcher::Class(B(&[B("ῑ"), B("ῡ"), B("ἰ")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+τ")),
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
            ReplacementPart::Literal(B("+ν")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+τ")]),
    },
    // ω+δ → ω+τ: 8 words (εἱλως, ἱδρως, ἐρως, γελως), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ω+δ")),
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
        replacement: B(&[ReplacementPart::Literal(B("ω+τ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ω+δ")]),
    },
    // ρῠ+κ → ρῠ+γ: 5 words (πτερῠξ, φρῠξ, ὀρῠξ, τρῠξ), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ρῠ+κ")),
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
        replacement: B(&[ReplacementPart::Literal(B("ρῠ+γ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ρῠ+κ")]),
    },
    // [ε|ᾰ]+ν → +ς after a vowel earlier in the stem: 7 words (τερᾰς, πᾰγκρεᾰς, ἐρῠσῐπελᾰς, περᾰς), 0 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("ἀ"),
                        B("ἁ"),
                        B("ἐ"),
                        B("ἑ"),
                        B("ἠ"),
                        B("ἡ"),
                        B("ἰ"),
                        B("ἱ"),
                        B("ὀ"),
                        B("ὁ"),
                        B("ὐ"),
                        B("ὑ"),
                        B("ὠ"),
                        B("ὡ"),
                        B("ᾀ"),
                        B("ᾠ"),
                        B("ᾰ"),
                        B("ᾱ"),
                        B("ᾳ"),
                        B("ῃ"),
                        B("ῐ"),
                        B("ῑ"),
                        B("ῠ"),
                        B("ῡ"),
                        B("ῳ"),
                        B("α"),
                        B("ε"),
                        B("η"),
                        B("ι"),
                        B("ο"),
                        B("υ"),
                        B("ω"),
                        B("ϊ"),
                        B("ϋ"),
                    ])),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("ἀ"),
                        B("ἁ"),
                        B("ἐ"),
                        B("ἑ"),
                        B("ἠ"),
                        B("ἡ"),
                        B("ἰ"),
                        B("ἱ"),
                        B("ὀ"),
                        B("ὁ"),
                        B("ὐ"),
                        B("ὑ"),
                        B("ὠ"),
                        B("ὡ"),
                        B("ᾀ"),
                        B("ᾠ"),
                        B("ᾰ"),
                        B("ᾱ"),
                        B("ᾳ"),
                        B("ῃ"),
                        B("ῐ"),
                        B("ῑ"),
                        B("ῠ"),
                        B("ῡ"),
                        B("ῳ"),
                        B("α"),
                        B("ε"),
                        B("η"),
                        B("ι"),
                        B("ο"),
                        B("υ"),
                        B("ω"),
                        B("ϊ"),
                        B("ϋ"),
                    ])),
                    quantifier: Quantifier::ZeroOrMore,
                    capture: None,
                },
            ]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("ᾰ"), B("ε")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+ν")),
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
            ReplacementPart::Literal(B("+ς")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+ν")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
