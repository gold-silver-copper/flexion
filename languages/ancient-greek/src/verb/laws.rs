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
    // ἐ‹ἀ → ἠ: 89 words (ἀγω, ἀγᾰπᾰω, ἀκουω, ἀγᾰθοποιεω), 0 forms wrong.
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
    // %ε‹ἀ → ἠ: 17 words (ἀγω, ἀγᾰπᾰω, ἀκουω, ἀρκεω), 0 forms wrong.
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
    // copy of φ → π: 33 words (φοβεω, φῐλεω, φενᾱκῐζω, φῠω), 0 forms wrong.
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
    // copy of θ → τ: 32 words (θελω, θερᾰπευω, ἐπεντῐθημῐ, θῡω), 0 forms wrong.
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
    // copy of χ → κ: 30 words (χᾰρῐζομαι, χειρουργεω, χορευω, χρῑω), 0 forms wrong.
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
    // ἐ‹ὀ → ὠ: 28 words (ὀρχεομαι, ὀνομᾰζω, ὀζω, ὀρεγω), 0 forms wrong.
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
    // ἐ%ε‹ἀ → ἠ: 9 words (ἀγᾰπᾰω, ἀσπᾰζομαι, ἀμαυροω, ἀρτᾰω), 0 forms wrong.
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
    // ζ+σ → +σ: 14 words (σκᾰνδᾰλῐζω, δειγμᾰτῐζω, γογγῠζω, βλυζω), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ζ+σ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+σ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ζ+σ")]),
    },
    // χ+σ → +ξ: 14 words (ψῡχω, δεχομαι, βρεχω, εὐχομαι), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
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
    // ἐ‹οἰ → ᾠ: 13 words (οἰφω, οἰκεω, οἰδεω, οἰκοδομεω), 0 forms wrong.
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
    // φ+σ → +ψ: 11 words (μεμφομαι, οἰφω, ἐρεφω, νῑφω), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
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
    // ἐ‹ἁ → ἡ: 9 words (ἁπτω, ἁρπᾰζω, ἁγῐζω, ἁλῐζω), 0 forms wrong.
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
        name: B("law13"),
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
        name: B("law14"),
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
    // %ε‹ὀ → ὠ: 7 words (ὀρχεομαι, ὀζω, ὀρεγω, ὀφειλω), 0 forms wrong.
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
    // ἐ‹ἱ → ἱ: 8 words (ἱημῐ, ἱδροω, ἱζω, ἱκνεομαι), 0 forms wrong.
    CompiledRule {
        name: B("law16"),
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
    // ἐ‹ὑ → ὑ: 9 words (ὑλακτεω, ὑφαινω, ὑμνεω, ὑστερεω), 0 forms wrong.
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
    // κ+σ → +ξ: 7 words (δερκομαι, ἑλκω, ἱκω, εἰκω), 0 forms wrong.
    CompiledRule {
        name: B("law18"),
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
    // ἐ‹ε → ε: 6 words (εὑω, εὐδοκεω, εἰσοραω, εἰκαζω), 0 forms wrong.
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
    // ἐ‹ἰ → ἰ: 8 words (ἰδῑω, ἰθῡνω, ἰαυω, ἰσχω), 0 forms wrong.
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
    // %ε‹ὑ → ὑ: 7 words (ὑμνεω, ὑστερεω, ὑβρῐζω, ὑᾰλῐζω), 0 forms wrong.
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
    // ζ+θ → +σθ: 6 words (σκᾰνδᾰλῐζω, βλυζω, λᾰλᾰζω, μηδῐζω), 0 forms wrong.
    CompiledRule {
        name: B("law22"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ζ+θ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+σθ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ζ+θ")]),
    },
    // %ε‹ζ → ἐζ: 7 words (ζεω, ζητεω, ζηλοω, ζευγνῡμῐ), 0 forms wrong.
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
    // %ε‹ἐ → ἠ: 5 words (ἐρεφω, ἐλεεω, ἐπειγω, ἐρημοω), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("°ἐε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἐ"), B("ε")])),
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
            entries: B(&[(B("ἐ"), B("ἠ")), (B("ε"), B("η"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°ἐε‹")]),
    },
    // ἐ‹αἰ → ᾐ: 6 words (αἰθω, αἰτεω, αἰσχῡνω, αἰδεομαι), 0 forms wrong.
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
    // ἐ%ε‹ζ → ἐζ: 6 words (ζεω, ζητεω, ζηλοω, ζῠγοω), 0 forms wrong.
    CompiledRule {
        name: B("law26"),
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
        name: B("law27"),
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
        name: B("law28"),
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
    // [υ|ι|ε|ῡ|ῠ|ῑ|ᾱ|ὑ|ω]+θ → +σθ after [α|ζ|ξ]: 7 words (παυω, ξῡω, ζωννῡμῐ, πταιω), 0 forms wrong.
    CompiledRule {
        name: B("law29"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("α"), B("ζ"), B("ξ")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("ῡ"),
                        B("ῠ"),
                        B("ῑ"),
                        B("ᾱ"),
                        B("ὑ"),
                        B("υ"),
                        B("ι"),
                        B("ε"),
                        B("ω"),
                    ])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+θ")),
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
            ReplacementPart::Literal(B("+σθ")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+θ")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
