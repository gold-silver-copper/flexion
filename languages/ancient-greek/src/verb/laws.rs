//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), on a letter a
//! `%` copies (`°`), or on a class letter an ending writes (harmony, by the
//! nearest letter of a tier before it as written): every form a conjugation builds
//! goes through them, in order, so one conjugation serves words whose stems change
//! its endings. Each with the words whose forms it gives, and how many of
//! the dictionary's forms it gets wrong (those are listed in `irregular.rs`).

use flexion::sound_law::compiled::*;

const RULES: &[CompiledRule<'static>] = &[
    // ἐ‹ἀ → ἠ: 84 words (ἀγω, ἀγᾰπᾰω, ἀγᾰθοποιεω, ἀγαλλιαω), 0 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἀ"), B("ᾰ"), B("ᾱ"), B("ᾳ")])),
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
            entries: B(&[
                (B("ἀ"), B("ἠ")),
                (B("ᾰ"), B("η")),
                (B("ᾱ"), B("η")),
                (B("ᾳ"), B("η")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // %ε‹ἀ → ἠ: 16 words (ἀγω, ἀγᾰπᾰω, ἀρκεω, ἀρνεομαι), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("°ἀε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἀ"), B("ᾰ"), B("ᾱ"), B("ᾳ"), B("α")])),
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
            entries: B(&[
                (B("ἀ"), B("ἠ")),
                (B("α"), B("η")),
                (B("ᾰ"), B("η")),
                (B("ᾱ"), B("η")),
                (B("ᾳ"), B("η")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°ἀε‹")]),
    },
    // copy of φ → π: 34 words (φοβεω, φῐλεω, φενᾱκῐζω, φαινω), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("°φ")),
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
        replacement: B(&[ReplacementPart::Literal(B("π"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°φ")]),
    },
    // copy of θ → τ: 31 words (θελω, θερᾰπευω, ἐπεντῐθημῐ, θεωρεω), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("°θ")),
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
        replacement: B(&[ReplacementPart::Literal(B("τ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°θ")]),
    },
    // copy of χ → κ: 31 words (χᾰρῐζομαι, χαιρω, χειρουργεω, χορευω), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("°χ")),
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
        replacement: B(&[ReplacementPart::Literal(B("κ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°χ")]),
    },
    // ἐ%ε‹ἀ → ἠ: 9 words (ἀγᾰπᾰω, ἀσπᾰζομαι, ἀμαυροω, ἀρτᾰω), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("ἐ°ἀε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἀ"), B("ᾰ"), B("ᾱ"), B("ᾳ"), B("α")])),
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
            entries: B(&[
                (B("ἀ"), B("ἠ")),
                (B("α"), B("η")),
                (B("ᾰ"), B("η")),
                (B("ᾱ"), B("η")),
                (B("ᾳ"), B("η")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ°ἀε‹")]),
    },
    // ἐ‹ὀ → ὠ: 23 words (ὀρχεομαι, ὀνομᾰζω, ὀζω, ὀρεγω), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ὀ")])),
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
            entries: B(&[(B("ὀ"), B("ὠ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // χ+σ → +ξ: 12 words (ψῡχω, βρεχω, ἀγχω, λειχω), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("χ+σ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ξ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("χ+σ")]),
    },
    // ἐ‹ῥ → ἐρρ: 15 words (ῥεω, ῥηγνῡμῐ, ῥωννῡμῐ, ῥοφεω), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ῥ")])),
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
            entries: B(&[(B("ῥ"), B("ἐρρ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // ἐ‹οἰ → ᾠ: 12 words (οἰφω, οἰκεω, οἰδεω, οἰκοδομεω), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("οἰ")])),
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
            entries: B(&[(B("οἰ"), B("ᾠ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // φ+σ → +ψ: 10 words (οἰφω, ἐρεφω, νῑφω, γλῠφω), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("φ+σ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ψ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("φ+σ")]),
    },
    // ἐ‹ἁ → ἡ: 10 words (ἁπτω, ἁρπᾰζω, ἁγῐζω, ἁλῐζω), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἁ"), B("ᾰ"), B("ᾱ"), B("ᾳ")])),
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
            entries: B(&[
                (B("ἁ"), B("ἡ")),
                (B("ᾰ"), B("η")),
                (B("ᾱ"), B("η")),
                (B("ᾳ"), B("η")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // ἐ‹ὁ → ὡ: 11 words (ὁμοιοω, ὁρῐζω, ὁμολογεω, ὁρμαινω), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ὁ")])),
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
            entries: B(&[(B("ὁ"), B("ὡ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // %ε‹σκ → ἐσκ: 10 words (σκοπεω, σκηπτω, σκευᾰζω, σκεπᾰζω), 0 forms wrong.
    CompiledRule {
        name: B("law13"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("°σε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("σκ")])),
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
            entries: B(&[(B("σκ"), B("ἐσκ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°σε‹")]),
    },
    // ἐ‹ὑ → ὑ: 9 words (ὑλακτεω, ὑφαινω, ὑμνεω, ὑστερεω), 0 forms wrong.
    CompiledRule {
        name: B("law14"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ὑ"), B("ῠ"), B("ῡ"), B("υ"), B("ϋ")])),
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
            entries: B(&[
                (B("ὑ"), B("ὑ")),
                (B("υ"), B("υ")),
                (B("ϋ"), B("υ")),
                (B("ῠ"), B("υ")),
                (B("ῡ"), B("υ")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // %ε‹ὀ → ὠ: 6 words (ὀρχεομαι, ὀζω, ὀρεγω, ὀφειλω), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("°ὀε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ὀ"), B("ο")])),
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
            entries: B(&[(B("ὀ"), B("ὠ")), (B("ο"), B("ω"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°ὀε‹")]),
    },
    // κ+σ → +ξ: 6 words (ἑλκω, ἱκω, εἰκω, ἐρῡκω), 0 forms wrong.
    CompiledRule {
        name: B("law16"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("κ+σ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ξ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("κ+σ")]),
    },
    // ἐ‹ε → ε: 5 words (εὐδοκεω, εἰσοραω, εἰκαζω, εἰλῠω), 0 forms wrong.
    CompiledRule {
        name: B("law17"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ε")])),
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
            entries: B(&[(B("ε"), B("ε"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // ἐ‹ἱ → ἱ: 9 words (ἱημῐ, ἱδροω, ἱζω, ἱκνεομαι), 0 forms wrong.
    CompiledRule {
        name: B("law18"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἱ"), B("ῐ"), B("ῑ"), B("ι"), B("ϊ")])),
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
            entries: B(&[
                (B("ἱ"), B("ἱ")),
                (B("ι"), B("ι")),
                (B("ϊ"), B("ι")),
                (B("ῐ"), B("ι")),
                (B("ῑ"), B("ι")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // %ε‹ὑ → ὑ: 7 words (ὑμνεω, ὑστερεω, ὑβρῐζω, ὑᾰλῐζω), 0 forms wrong.
    CompiledRule {
        name: B("law19"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("°ὑε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ὑ"), B("ῠ"), B("ῡ"), B("υ"), B("ϋ")])),
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
            entries: B(&[
                (B("ὑ"), B("ὑ")),
                (B("υ"), B("υ")),
                (B("ϋ"), B("υ")),
                (B("ῠ"), B("υ")),
                (B("ῡ"), B("υ")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°ὑε‹")]),
    },
    // ἐ‹ἰ → ἰ: 6 words (ἰσχῡω, ἰσχω, ἰχθῠᾰζομαι, ἰχθῠᾰω), 0 forms wrong.
    CompiledRule {
        name: B("law20"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἰ"), B("ῐ"), B("ῑ"), B("ι"), B("ϊ")])),
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
            entries: B(&[
                (B("ἰ"), B("ἰ")),
                (B("ι"), B("ι")),
                (B("ϊ"), B("ι")),
                (B("ῐ"), B("ι")),
                (B("ῑ"), B("ι")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // %ε‹ζ → ἐζ: 5 words (ζητεω, ζηλοω, ζῠγοω, ζωγρεω), 0 forms wrong.
    CompiledRule {
        name: B("law21"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("°ζε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ζ")])),
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
            entries: B(&[(B("ζ"), B("ἐζ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°ζε‹")]),
    },
    // ἐ‹αἰ → ᾐ: 6 words (αἰθω, αἰτεω, αἰσχῡνω, αἰδεομαι), 0 forms wrong.
    CompiledRule {
        name: B("law22"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("αἰ")])),
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
            entries: B(&[(B("αἰ"), B("ᾐ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // ἐ%ε‹ζ → ἐζ: 5 words (ζητεω, ζηλοω, ζῠγοω, ζωγρεω), 0 forms wrong.
    CompiledRule {
        name: B("law23"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ°ζε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ζ")])),
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
            entries: B(&[(B("ζ"), B("ἐζ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ°ζε‹")]),
    },
    // ἐ%ε‹σκ → ἐσκ: 5 words (σκοπεω, σκευᾰζω, σκεπᾰζω, σκορπῐζω), 0 forms wrong.
    CompiledRule {
        name: B("law24"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ°σε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("σκ")])),
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
            entries: B(&[(B("σκ"), B("ἐσκ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ°σε‹")]),
    },
    // ἐ%ε‹ὑ → ὑ: 5 words (ὑμνεω, ὑστερεω, ὑβρῐζω, ὑᾰλῐζω), 0 forms wrong.
    CompiledRule {
        name: B("law25"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ°ὑε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ὑ"), B("ῠ"), B("ῡ"), B("υ"), B("ϋ")])),
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
            entries: B(&[
                (B("ὑ"), B("ὑ")),
                (B("υ"), B("υ")),
                (B("ϋ"), B("υ")),
                (B("ῠ"), B("υ")),
                (B("ῡ"), B("υ")),
            ]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ°ὑε‹")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
