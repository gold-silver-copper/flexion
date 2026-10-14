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
    // or+e → +ōre: 885 words (abāctor, abbreviātor, abductor, abnegātor), 40 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("or+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ōre"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("or+e")]),
    },
    // or+i → +ōri: 885 words (abāctor, abbreviātor, abductor, abnegātor), 47 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("or+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ōri"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("or+i")]),
    },
    // or+ē → +ōrē: 882 words (abāctor, abbreviātor, abductor, abnegātor), 18 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("or+ē")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ōrē"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("or+ē")]),
    },
    // s → ∅ after [e|p|ā|ē|ō|ū]; x → c after [a|e|i|n|u|y|ā|ī|ō]; ō → i after [d|g]; us → e after [d|l|m|n] in alternating stems (ˇ), before +[e|i|n|r|t|u|ē]: 876 words (ōs, genus, onus, cōs), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[
                    B("ā"),
                    B("ē"),
                    B("ī"),
                    B("ō"),
                    B("ū"),
                    B("a"),
                    B("d"),
                    B("e"),
                    B("g"),
                    B("i"),
                    B("l"),
                    B("m"),
                    B("n"),
                    B("p"),
                    B("u"),
                    B("y"),
                ])),
                quantifier: Quantifier::One,
                capture: Some(0),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("us"), B("ō"), B("s"), B("x")])),
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
                    matcher: Matcher::Class(B(&[
                        B("ē"),
                        B("e"),
                        B("i"),
                        B("n"),
                        B("r"),
                        B("t"),
                        B("u"),
                    ])),
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
                (B("s"), B("a"), B("s")),
                (B("s"), B("d"), B("s")),
                (B("s"), B("e"), B("¦")),
                (B("s"), B("g"), B("s")),
                (B("s"), B("i"), B("s")),
                (B("s"), B("l"), B("s")),
                (B("s"), B("m"), B("s")),
                (B("s"), B("n"), B("s")),
                (B("s"), B("p"), B("¦")),
                (B("s"), B("u"), B("s")),
                (B("s"), B("y"), B("s")),
                (B("s"), B("ā"), B("¦")),
                (B("s"), B("ē"), B("¦")),
                (B("s"), B("ī"), B("s")),
                (B("s"), B("ō"), B("¦")),
                (B("s"), B("ū"), B("¦")),
                (B("us"), B("a"), B("us")),
                (B("us"), B("d"), B("e¦")),
                (B("us"), B("e"), B("us")),
                (B("us"), B("g"), B("us")),
                (B("us"), B("i"), B("us")),
                (B("us"), B("l"), B("e¦")),
                (B("us"), B("m"), B("e¦")),
                (B("us"), B("n"), B("e¦")),
                (B("us"), B("p"), B("us")),
                (B("us"), B("u"), B("us")),
                (B("us"), B("y"), B("us")),
                (B("us"), B("ā"), B("us")),
                (B("us"), B("ē"), B("us")),
                (B("us"), B("ī"), B("us")),
                (B("us"), B("ō"), B("us")),
                (B("us"), B("ū"), B("us")),
                (B("x"), B("a"), B("c¦")),
                (B("x"), B("d"), B("x")),
                (B("x"), B("e"), B("c¦")),
                (B("x"), B("g"), B("x")),
                (B("x"), B("i"), B("c¦")),
                (B("x"), B("l"), B("x")),
                (B("x"), B("m"), B("x")),
                (B("x"), B("n"), B("c¦")),
                (B("x"), B("p"), B("x")),
                (B("x"), B("u"), B("c¦")),
                (B("x"), B("y"), B("c¦")),
                (B("x"), B("ā"), B("c¦")),
                (B("x"), B("ē"), B("x")),
                (B("x"), B("ī"), B("c¦")),
                (B("x"), B("ō"), B("c¦")),
                (B("x"), B("ū"), B("x")),
                (B("ō"), B("a"), B("ō")),
                (B("ō"), B("d"), B("i¦")),
                (B("ō"), B("e"), B("ō")),
                (B("ō"), B("g"), B("i¦")),
                (B("ō"), B("i"), B("ō")),
                (B("ō"), B("l"), B("ō")),
                (B("ō"), B("m"), B("ō")),
                (B("ō"), B("n"), B("ō")),
                (B("ō"), B("p"), B("ō")),
                (B("ō"), B("u"), B("ō")),
                (B("ō"), B("y"), B("ō")),
                (B("ō"), B("ā"), B("ō")),
                (B("ō"), B("ē"), B("ō")),
                (B("ō"), B("ī"), B("ō")),
                (B("ō"), B("ō"), B("ō")),
                (B("ō"), B("ū"), B("ō")),
            ]),
        }]),
        capture_count: 2,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+"), B("ˇ")]),
    },
    // s+t → +t: 8 words (pancreas, erysipelas, mārs, rinoceros), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("s+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("+t"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("s+t")]),
    },
    // i+i → i+: 15 words (axis, anguis, orexis, unguis), 0 forms wrong.
    CompiledRule {
        name: B("law5"),
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
    // i+t → i+d: 147 words (pȳramis, clītoris, syphilis, achaemenis), 9 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("i+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("i+d"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("i+t")]),
    },
    // [r|n|l|x]+n → +: 123 words (sōl, martyr, ēr, massētēr), 10 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("r"), B("n"), B("l"), B("x")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+n")),
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
        required: B(&[B("+n")]),
    },
    // a+e → a+: 24 words (anthrax, manna, victuma, lacruma), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("a+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("a+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("a+e")]),
    },
    // [p|b|ō|m|y]+tē → +ē: 23 words (urbs, cyclōps, thrīps, ops), 2 forms wrong.
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
                    matcher: Matcher::Class(B(&[B("ō"), B("p"), B("b"), B("m"), B("y")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+tē")),
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
            ReplacementPart::Literal(B("+ē")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+tē")]),
    },
    // ēn+t → +ent: 29 words (lēns, dēns, mēns, gēns), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
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
    // us+r → +er: 5 words (opus, latus, ulcus, vīscus), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("us+r")),
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
        replacement: B(&[ReplacementPart::Literal(B("+er"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("us+r")]),
    },
    // p+t → p+: 18 words (cyclōps, thrīps, ops, daps), 0 forms wrong.
    CompiledRule {
        name: B("law12"),
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
    // ān+t → +ant: 16 words (accūsāns, īnfāns, elephāns, quadrāns), 0 forms wrong.
    CompiledRule {
        name: B("law13"),
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
    // iī+ → i+: 11 words (aquāriī, eximiī, stadiī, rōrāriī), 0 forms wrong.
    CompiledRule {
        name: B("law14"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("iī+")),
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
        required: B(&[B("iī+")]),
    },
    // xi+ē → x+ē: 9 words (axis, orexis, syntaxis, rīxis), 0 forms wrong.
    CompiledRule {
        name: B("law15"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("xi+ē")),
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
        replacement: B(&[ReplacementPart::Literal(B("x+ē"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("xi+ē")]),
    },
    // ca+t → ca+d: 7 words (dorcas, īcas, dyōdecas, duōdecas), 0 forms wrong.
    CompiledRule {
        name: B("law16"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ca+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("ca+d"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ca+t")]),
    },
    // [n|r|t]+b → +ib: 7 words (vetera, tormina, arōmata, posteriōra), 0 forms wrong.
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
                    matcher: Matcher::Class(B(&[B("n"), B("r"), B("t")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+b")),
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
            ReplacementPart::Literal(B("+ib")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+b")]),
    },
    // ui+e → u+e: 6 words (anguis, unguis, ninguis, torquis), 0 forms wrong.
    CompiledRule {
        name: B("law18"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ui+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("u+e"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ui+e")]),
    },
    // ui+ē → u+ē: 6 words (anguis, unguis, ninguis, torquis), 0 forms wrong.
    CompiledRule {
        name: B("law19"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ui+ē")),
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
        replacement: B(&[ReplacementPart::Literal(B("u+ē"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ui+ē")]),
    },
    // b+t → b+: 5 words (urbs, plēbs, arabs, chalybs), 0 forms wrong.
    CompiledRule {
        name: B("law20"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("b+t")),
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
        replacement: B(&[ReplacementPart::Literal(B("b+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("b+t")]),
    },
    // in+c → in+g: 5 words (sphinx, mēninx, sȳrinx, salpinx), 0 forms wrong.
    CompiledRule {
        name: B("law21"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("in+c")),
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
        replacement: B(&[ReplacementPart::Literal(B("in+g"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("in+c")]),
    },
    // xi+e → x+e: 5 words (axis, orexis, rīxis, rhēxis), 0 forms wrong.
    CompiledRule {
        name: B("law22"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("xi+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("x+e"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("xi+e")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
