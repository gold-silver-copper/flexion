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
    // ēn+t → +ent: 13 words (ēns, libēns, lubēns, ingēns), 0 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ēn+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ent"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ēn+t")]),
    },
    // x+em → +cem: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 0 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+em")),
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
        replacement: B(&[ReplacementPart::Literal(B("+cem"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+em")]),
    },
    // x+i → +ci: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 6 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ci"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+i")]),
    },
    // x+ē → +cē: 80 words (phoenīx, īnfēlīx, fēlīx, atrōx), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+ē")),
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
        replacement: B(&[ReplacementPart::Literal(B("+cē"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+ē")]),
    },
    // x → c after [ā|ī|ō]; ēs → ed after p; ex → ic after l; s → d after r in alternating stems (ˇ), before +.: 120 words (phoenīx, complex, simplex, duplex), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ā"), B("ī"), B("ō"), B("l"), B("p"), B("r")])),
                quantifier: Quantifier::One,
                capture: Some(0),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ēs"), B("ex"), B("s"), B("x")])),
                quantifier: Quantifier::One,
                capture: Some(1),
            }]),
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
        replacement: B(&[ReplacementPart::AgreeingCapture {
            capture: 1,
            key: 0,
            entries: B(&[
                (B("ex"), B("l"), B("ic¦")),
                (B("ex"), B("p"), B("ex")),
                (B("ex"), B("r"), B("ex")),
                (B("ex"), B("ā"), B("ex")),
                (B("ex"), B("ī"), B("ex")),
                (B("ex"), B("ō"), B("ex")),
                (B("s"), B("l"), B("s")),
                (B("s"), B("p"), B("s")),
                (B("s"), B("r"), B("d¦")),
                (B("s"), B("ā"), B("s")),
                (B("s"), B("ī"), B("s")),
                (B("s"), B("ō"), B("s")),
                (B("x"), B("l"), B("x")),
                (B("x"), B("p"), B("x")),
                (B("x"), B("r"), B("x")),
                (B("x"), B("ā"), B("c¦")),
                (B("x"), B("ī"), B("c¦")),
                (B("x"), B("ō"), B("c¦")),
                (B("ēs"), B("l"), B("ēs")),
                (B("ēs"), B("p"), B("ed¦")),
                (B("ēs"), B("r"), B("ēs")),
                (B("ēs"), B("ā"), B("ēs")),
                (B("ēs"), B("ī"), B("ēs")),
                (B("ēs"), B("ō"), B("ēs")),
            ]),
        }]),
        capture_count: 2,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // ān+t → +ant: 53 words (īnfāns, ignicomāns, ēlegāns, auricomāns), 0 forms wrong.
    CompiledRule {
        name: B("law5"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ān+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ant"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ān+t")]),
    },
    // e+c → +ic: 34 words (complex, simplex, duplex, multiplex), 29 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("e+c")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ic"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("e+c")]),
    },
    // ōn+t → +ont: 14 words (sōns, īnsōns, latifrōns, nigrifrōns), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ōn+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ont"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ōn+t")]),
    },
    // p+t → p+: 5 words (inops, cōps, mīcrōps, macrōps), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("p+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("p+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("p+t")]),
    },
    // x+ī → +cī before ˇ: 10 words (redux, dēdux, praecox, bilanx), 0 forms wrong.
    CompiledRule {
        name: B("law9"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+ī")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ˇ")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+cī"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+ī")]),
    },
    // x+īs → +cīs before ˇ: 10 words (redux, dēdux, praecox, bilanx), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("x+īs")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("ˇ")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::Literal(B("+cīs"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("x+īs")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
