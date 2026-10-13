//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), or on a letter a
//! `%` copies (`°`): every form a conjugation builds goes through them, in order, so
//! one conjugation serves words whose stems change its endings. Each with the
//! words whose forms it gives, and how many of the dictionary's forms it
//! gets wrong (those are listed in `irregular.rs`).

use flexion::sound_law::compiled::*;

const RULES: &[CompiledRule<'static>] = &[
    // ἐ‹ἀ → ἠ: 92 words (ἀγω, ἀκουω, ἀρχω, ἀγαλλιαω), 0 forms wrong.
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
    // %ε‹ἀ → ἠ: 19 words (ἀγω, ἀρχαϊζω, ἀνᾰγκᾰζω, ἀρνεομαι), 0 forms wrong.
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
    // copy of φ → π: 30 words (φοβεω, φῐλεω, φενᾱκῐζω, φωγω), 0 forms wrong.
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
    // copy of θ → τ: 26 words (θελω, θερᾰπευω, ἐπεντῐθημῐ, τῐθημῐ), 0 forms wrong.
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
    // copy of χ → κ: 23 words (χᾰρῐζομαι, χειρουργεω, χορευω, χρῑω), 0 forms wrong.
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
    // ἐ‹ὀ → ὠ: 30 words (ὀρχεομαι, ὀνομᾰζω, ὀζω, ὀρεγω), 0 forms wrong.
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
    // ἐ%ε‹ἀ → ἠ: 11 words (ἀγαπαω, ἀκουω, ἀρχαϊζω, ἀνᾰγκᾰζω), 0 forms wrong.
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
    // κ+σ → +ξ: 16 words (πεκω, πλεκω, τηκω, κᾰτᾰδῐωκω), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
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
    // ἐ‹ῥ → ἐρρ: 17 words (ῥεω, ῥωννῡμῐ, ῥοφεω, ῥεμβω), 0 forms wrong.
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
    // %ε‹ἐ → ἠ: 14 words (ἐρεφω, ἐθελω, ἐλεεω, ἐρωτᾰω), 0 forms wrong.
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
    // ιν+ηθ → +νθ: 15 words (σημαινω, ῥαινω, μιαινω, ποιμαινω), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ιν+ηθ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+νθ"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ιν+ηθ")]),
    },
    // χ+σ → +ξ: 14 words (ἀρχω, βρεχω, λειχω, ὀμειχω), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
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
    // ἐ‹ἐρ → ἠρ: 14 words (ἐρειπω, ἐρευγομαι, ἐρεφω, ἐργᾰζομαι), 0 forms wrong.
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
                    matcher: Matcher::Class(B(&[B("ἐρ")])),
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
            entries: B(&[(B("ἐρ"), B("ἠρ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // φ+σ → +ψ: 10 words (οἰφω, νῑφω, κᾰτᾰστρεφω, γλῠφω), 0 forms wrong.
    CompiledRule {
        name: B("law13"),
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
    // ἐ‹οἰ → ᾠ: 12 words (οἰφω, οἰκεω, οἰδεω, οἰκοδομεω), 0 forms wrong.
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
    // ἐ‹ὑ → ὑ: 11 words (ὑλακτεω, ὑφαινω, ὑμνεω, ὑστερεω), 0 forms wrong.
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
    // ν+σ → ν+: 11 words (εὐρῡνω, ἰθῡνω, δῐευθῡνω, εὐθῡνω), 0 forms wrong.
    CompiledRule {
        name: B("law16"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ν+σ")),
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
        replacement: B(&[ReplacementPart::Literal(B("ν+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ν+σ")]),
    },
    // %ε‹σκ → ἐσκ: 11 words (σκοπεω, σκηπτω, σκευᾰζω, σκεπᾰζω), 0 forms wrong.
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
    // ἐ‹ἁ → ἡ: 12 words (ἁρμοζω, ἁπτω, ἁρπᾰζω, ἁμᾰρτᾰνω), 0 forms wrong.
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
    // %ε‹ὀ → ὠ: 7 words (ὀρχεομαι, ὀζω, ὀρεγω, ὀφειλω), 0 forms wrong.
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
    // ἐ‹ε → ε: 7 words (εἰδομαι, εἰρω, εὐδοκεω, εἰσοραω), 0 forms wrong.
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
    // ἐ‹ἰ → ἰ: 9 words (ἰαομαι, ἰθῡνω, ἰαυω, ἰσχῡω), 0 forms wrong.
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
    // ἐ‹ἱ → ἱ: 7 words (ἱδροω, ἱζω, ἱκνεομαι, ἱστορεω), 0 forms wrong.
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
    // %ε‹ζ → ἐζ: 7 words (ζεω, ζω, ζητεω, ζηλοω), 0 forms wrong.
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
    // %ε‹ὑ → ὑ: 7 words (ὑμνεω, ὑστερεω, ὑβρῐζω, ὑᾰλῐζω), 0 forms wrong.
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
    // ἐ‹αἰ → ᾐ: 7 words (αἰθω, αἰρω, αἰτεω, αἰσχῡνω), 0 forms wrong.
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
    // ἐ‹ἐλ → ἠλ: 7 words (ἐλελῐζω, ἐλεεω, ἐλπῐζω, ἐλαυνω), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἐλ")])),
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
            entries: B(&[(B("ἐλ"), B("ἠλ"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // ἐ‹ἡ → ἡ: 6 words (ἡδομαι, ἡσῠχᾰζω, ἡγεμονευω, ἡβᾰω), 0 forms wrong.
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
                    matcher: Matcher::Literal(B("ἐ‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ἡ"), B("ῃ"), B("η")])),
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
            entries: B(&[(B("ἡ"), B("ἡ")), (B("η"), B("η")), (B("ῃ"), B("η"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // ἐ%ε‹ἐ → ἠ: 7 words (ἐθελω, ἐρωτᾰω, ἐγγῐζω, ἐλπῐζω), 0 forms wrong.
    CompiledRule {
        name: B("law29"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("ἐ°ἐε‹")),
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
        required: B(&[B("ἐ°ἐε‹")]),
    },
    // α+εῑ → +ᾰσε: 6 words (σῠνεργαζομαι, ἀσπαζομαι, κᾰτεργαζομαι, χαζομαι), 0 forms wrong.
    CompiledRule {
        name: B("law30"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("α+εῑ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ᾰσε"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("α+εῑ")]),
    },
    // α+οι → +ᾰσοι after a vowel earlier in the stem: 5 words (σῠνεργαζομαι, ἀσπαζομαι, κᾰτεργαζομαι, ἐξεργαζομαι), 0 forms wrong.
    CompiledRule {
        name: B("law31"),
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
                        B("ᾀ"),
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
                        B("ᾀ"),
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
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("α+οι")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ᾰσοι"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("α+οι")]),
    },
    // α+οῑ → +ᾰσοι: 6 words (σῠνεργαζομαι, ἀσπαζομαι, κᾰτεργαζομαι, χαζομαι), 0 forms wrong.
    CompiledRule {
        name: B("law32"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("α+οῑ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ᾰσοι"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("α+οῑ")]),
    },
    // α+οῡ → +ᾰσο: 6 words (σῠνεργαζομαι, ἀσπαζομαι, κᾰτεργαζομαι, χαζομαι), 0 forms wrong.
    CompiledRule {
        name: B("law33"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("α+οῡ")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ᾰσο"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("α+οῡ")]),
    },
    // ἐ‹ἐκ → ἐξε: 6 words (ἐκδῠω, ἐκχεω, ἐκφερω, ἐκλεγω), 0 forms wrong.
    CompiledRule {
        name: B("law34"),
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
                    matcher: Matcher::Class(B(&[B("ἐκ")])),
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
            entries: B(&[(B("ἐκ"), B("ἐξε"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // ἐ%ε‹ζ → ἐζ: 6 words (ζεω, ζητεω, ζηλοω, ζῠγοω), 0 forms wrong.
    CompiledRule {
        name: B("law35"),
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
    // ᾱ+ο → +ω: 5 words (δῐψᾱω, νᾱω, δρᾱω, ψᾱω), 0 forms wrong.
    CompiledRule {
        name: B("law36"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ᾱ+ο")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ω"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ᾱ+ο")]),
    },
    // %ε‹ὁ → ὡ: 5 words (ὁμοιοω, ὁρῐζω, ὁρμαω, ὁπλιζω), 0 forms wrong.
    CompiledRule {
        name: B("law37"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("°ὁε‹")),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::Class(B(&[B("ὁ"), B("ο")])),
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
            entries: B(&[(B("ὁ"), B("ὡ")), (B("ο"), B("ω"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("°ὁε‹")]),
    },
    // %ε‹ῥ → ἐρρ: 5 words (ῥοφεω, ῥᾰπτω, ῥῑπτω, ῥῑπτᾰζω), 0 forms wrong.
    CompiledRule {
        name: B("law38"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Literal(B("°ῥε‹")),
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
        required: B(&[B("°ῥε‹")]),
    },
    // ἐ‹α → η: 5 words (αὐξᾰνω, αὐλεω, αὐξω, αὐδᾰω), 0 forms wrong.
    CompiledRule {
        name: B("law39"),
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
                    matcher: Matcher::Class(B(&[B("α")])),
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
            entries: B(&[(B("α"), B("η"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ἐ‹")]),
    },
    // ἐ%ε‹σκ → ἐσκ: 5 words (σκοπεω, σκευᾰζω, σκεπᾰζω, σκορπῐζω), 0 forms wrong.
    CompiledRule {
        name: B("law40"),
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
        name: B("law41"),
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
