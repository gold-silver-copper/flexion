//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), on a letter a
//! `%` copies (`°`), or on a class letter an ending writes (harmony, by the
//! nearest letter of a tier before it as written): every form a declension builds
//! goes through them, in order, so one declension serves words whose stems change
//! its endings. Each with the words whose forms it gives, and how many of
//! the dictionary's forms it gets wrong (those are listed in `irregular.rs`).

use flexion::sound_law::compiled::*;

const RULES: &[CompiledRule<'static>] = &[
    // A → a after a, o, u; ä after y, ä, ö, none (the nearest of a, o, u, y, ä, ö before it, as written): 5,952 words (swap, avatar, koira, luu), 988 forms wrong.
    CompiledRule {
        name: B("law0"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[B("ä"), B("ö"), B("a"), B("o"), B("u"), B("y")])),
                    quantifier: Quantifier::Optional,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("ä"),
                        B("ö"),
                        B("a"),
                        B("o"),
                        B("u"),
                        B("y"),
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
                (B("A"), B("a"), B("a")),
                (B("A"), B("o"), B("a")),
                (B("A"), B("u"), B("a")),
                (B("A"), B("y"), B("ä")),
                (B("A"), B("ä"), B("ä")),
                (B("A"), B("ö"), B("ä")),
                (B("A"), B(""), B("ä")),
            ]),
        }]),
        capture_count: 3,
        direction: Direction::LeftToRight,
        application: Application::Iterative,
        required: B(&[]),
    },
    // u+on → u+un: 296 words (itu, satu, menu, taulu), 16 forms wrong.
    CompiledRule {
        name: B("law1"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("u+on")),
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
        replacement: B(&[ReplacementPart::Literal(B("u+un"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("u+on")]),
    },
    // o+u → o+o: 10 words (happo, peippo, hepo, tappo), 0 forms wrong.
    CompiledRule {
        name: B("law2"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("o+u")),
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
        replacement: B(&[ReplacementPart::Literal(B("o+o"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("o+u")]),
    },
    // ku+o → ku+u: 72 words (kurkku, kinkku, laukku, polku), 0 forms wrong.
    CompiledRule {
        name: B("law3"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ku+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("ku+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ku+o")]),
    },
    // lu+o → lu+u: 62 words (taulu, laulu, zulu, sielu), 0 forms wrong.
    CompiledRule {
        name: B("law4"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("lu+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("lu+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("lu+o")]),
    },
    // [e|ä|a]+ii → +isii: 39 words (rike, liike, ärsyke, levyke), 0 forms wrong.
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
                    matcher: Matcher::Class(B(&[B("ä"), B("e"), B("a")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+ii")),
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
            ReplacementPart::Literal(B("+isii")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+ii")]),
    },
    // tu+o → tu+u: 51 words (itu, satu, kettu, hattu), 0 forms wrong.
    CompiledRule {
        name: B("law6"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("tu+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("tu+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("tu+o")]),
    },
    // e+ee → e+esee: 37 words (rike, liike, ärsyke, levyke), 0 forms wrong.
    CompiledRule {
        name: B("law7"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("e+ee")),
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
        replacement: B(&[ReplacementPart::Literal(B("e+esee"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("e+ee")]),
    },
    // n+d → n+n: 30 words (kynsi, punta, länsi, valtakunta), 0 forms wrong.
    CompiledRule {
        name: B("law8"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("n+d")),
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
        replacement: B(&[ReplacementPart::Literal(B("n+n"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("n+d")]),
    },
    // su+o → su+u: 41 words (sisu, asu, kirsu, maksu), 0 forms wrong.
    CompiledRule {
        name: B("law9"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("su+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("su+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("su+o")]),
    },
    // ke+te → k+keide: 37 words (rike, liike, ärsyke, levyke), 0 forms wrong.
    CompiledRule {
        name: B("law10"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ke+te")),
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
        replacement: B(&[ReplacementPart::Literal(B("k+keide"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ke+te")]),
    },
    // ö+y → ö+ö: 34 words (sähkö, pöllö, henkilö, pyrstö), 0 forms wrong.
    CompiledRule {
        name: B("law11"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ö+y")),
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
        replacement: B(&[ReplacementPart::Literal(B("ö+ö"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ö+y")]),
    },
    // [e|a]+ia → +ita: 22 words (ohdake, uloke, tarvike, vahvike), 0 forms wrong.
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
                    matcher: Matcher::Class(B(&[B("e"), B("a")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+ia")),
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
            ReplacementPart::Literal(B("+ita")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+ia")]),
    },
    // e+ta → e+tta: 21 words (ohdake, uloke, tarvike, vahvike), 6 forms wrong.
    CompiledRule {
        name: B("law13"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("e+ta")),
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
        replacement: B(&[ReplacementPart::Literal(B("e+tta"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("e+ta")]),
    },
    // t+d → t+: 14 words (mitta, kenttä, rotta, lentokenttä), 0 forms wrong.
    CompiledRule {
        name: B("law14"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("t+d")),
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
        replacement: B(&[ReplacementPart::Literal(B("t+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("t+d")]),
    },
    // t+n → t+: 12 words (kartta, riutta, latta, laatta), 0 forms wrong.
    CompiledRule {
        name: B("law15"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("t+n")),
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
        replacement: B(&[ReplacementPart::Literal(B("t+"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("t+n")]),
    },
    // [e|ä]+iä → +itä: 17 words (rike, liike, ärsyke, levyke), 0 forms wrong.
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
                    matcher: Matcher::Class(B(&[B("ä"), B("e")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+iä")),
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
            ReplacementPart::Literal(B("+itä")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+iä")]),
    },
    // r+d → r+r: 14 words (virta, hirsi, varsi, kerta), 0 forms wrong.
    CompiledRule {
        name: B("law17"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("r+d")),
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
        replacement: B(&[ReplacementPart::Literal(B("r+r"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("r+d")]),
    },
    // uu+i → u+i: 16 words (luu, kuu, suu, puu), 0 forms wrong.
    CompiledRule {
        name: B("law18"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("uu+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("u+i"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("uu+i")]),
    },
    // [u|i|a|o]+no → +do: 8 words (rauta, nauta, hauta, lauta), 0 forms wrong.
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
                    matcher: Matcher::Class(B(&[B("u"), B("i"), B("a"), B("o")])),
                    quantifier: Quantifier::One,
                    capture: Some(0),
                },
                Atom {
                    matcher: Matcher::Literal(B("+no")),
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
            ReplacementPart::Literal(B("+do")),
        ]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("+no")]),
    },
    // mu+o → mu+u: 18 words (imu, hirmu, luumu, leimu), 0 forms wrong.
    CompiledRule {
        name: B("law20"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("mu+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("mu+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("mu+o")]),
    },
    // ke+tä → ke+ttä: 16 words (rike, liike, ärsyke, levyke), 0 forms wrong.
    CompiledRule {
        name: B("law21"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ke+tä")),
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
        replacement: B(&[ReplacementPart::Literal(B("ke+ttä"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ke+tä")]),
    },
    // uu+o → uu+hu: 16 words (luu, kuu, suu, puu), 0 forms wrong.
    CompiledRule {
        name: B("law22"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("uu+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("uu+hu"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("uu+o")]),
    },
    // uu+u → uu+hu: 16 words (luu, kuu, suu, puu), 0 forms wrong.
    CompiledRule {
        name: B("law23"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("uu+u")),
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
        replacement: B(&[ReplacementPart::Literal(B("uu+hu"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("uu+u")]),
    },
    // ru+o → ru+u: 14 words (peru, piru, kuru, muru), 0 forms wrong.
    CompiledRule {
        name: B("law24"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ru+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("ru+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ru+o")]),
    },
    // a+aa → a+taa: 14 words (pata, rata, harmaa, maa), 0 forms wrong.
    CompiledRule {
        name: B("law25"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("a+aa")),
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
        replacement: B(&[ReplacementPart::Literal(B("a+taa"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("a+aa")]),
    },
    // a+je → +ide: 9 words (harmaa, maa, autiomaa, erämaa), 0 forms wrong.
    CompiledRule {
        name: B("law26"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("a+je")),
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
        replacement: B(&[ReplacementPart::Literal(B("+ide"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("a+je")]),
    },
    // ju+o → ju+u: 12 words (ketju, lieju, juju, paju), 0 forms wrong.
    CompiledRule {
        name: B("law27"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ju+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("ju+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ju+o")]),
    },
    // hu+o → hu+u: 11 words (karhu, kauhu, mehu, kihu), 0 forms wrong.
    CompiledRule {
        name: B("law28"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("hu+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("hu+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("hu+o")]),
    },
    // l+d → l+l: 5 words (kulta, multa, silta, punamulta), 0 forms wrong.
    CompiledRule {
        name: B("law29"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("l+d")),
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
        replacement: B(&[ReplacementPart::Literal(B("l+l"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("l+d")]),
    },
    // h+n → h+d: 10 words (vahti, mahti, maalivahti, ryhti), 0 forms wrong.
    CompiledRule {
        name: B("law30"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("h+n")),
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
        replacement: B(&[ReplacementPart::Literal(B("h+d"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("h+n")]),
    },
    // l+n → l+l: 6 words (valta, lainsäädäntövalta, pelti, keskusvalta), 0 forms wrong.
    CompiledRule {
        name: B("law31"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("l+n")),
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
        replacement: B(&[ReplacementPart::Literal(B("l+l"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("l+n")]),
    },
    // aa+a → aa+ta: 9 words (harmaa, maa, autiomaa, erämaa), 0 forms wrong.
    CompiledRule {
        name: B("law32"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("aa+a")),
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
        replacement: B(&[ReplacementPart::Literal(B("aa+ta"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("aa+a")]),
    },
    // aa+i → a+i: 9 words (harmaa, maa, autiomaa, erämaa), 0 forms wrong.
    CompiledRule {
        name: B("law33"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("aa+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("a+i"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("aa+i")]),
    },
    // aa+j → a+it: 9 words (harmaa, maa, autiomaa, erämaa), 0 forms wrong.
    CompiledRule {
        name: B("law34"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("aa+j")),
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
        replacement: B(&[ReplacementPart::Literal(B("a+it"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("aa+j")]),
    },
    // au+n → au+d: 9 words (rauta, nauta, hauta, lauta), 18 forms wrong.
    CompiledRule {
        name: B("law35"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("au+n")),
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
        replacement: B(&[ReplacementPart::Literal(B("au+d"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("au+n")]),
    },
    // p+v → p+: 9 words (kärppä, sappi, leppä, seppä), 0 forms wrong.
    CompiledRule {
        name: B("law36"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("p+v")),
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
        required: B(&[B("p+v")]),
    },
    // ul+ej → ul+eit after a vowel earlier in the stem: 9 words (valkosipuli, sopuli, konsuli, pumpuli), 0 forms wrong.
    CompiledRule {
        name: B("law37"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("ä"),
                        B("ö"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("o"),
                        B("u"),
                        B("y"),
                    ])),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("ä"),
                        B("ö"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("o"),
                        B("u"),
                        B("y"),
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
                matcher: Matcher::Literal(B("ul+ej")),
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
        replacement: B(&[ReplacementPart::Literal(B("ul+eit"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ul+ej")]),
    },
    // y+ö → y+y: 9 words (sänky, mänty, myrkky, näky), 0 forms wrong.
    CompiledRule {
        name: B("law38"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("y+ö")),
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
        replacement: B(&[ReplacementPart::Literal(B("y+y"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("y+ö")]),
    },
    // a+on → a+han: 7 words (autiomaa, erämaa, isänmaa, takamaa), 0 forms wrong.
    CompiledRule {
        name: B("law39"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("a+on")),
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
        replacement: B(&[ReplacementPart::Literal(B("a+han"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("a+on")]),
    },
    // aa+o → aa+ha after a vowel earlier in the stem: 7 words (autiomaa, erämaa, isänmaa, takamaa), 0 forms wrong.
    CompiledRule {
        name: B("law40"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("ä"),
                        B("ö"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("o"),
                        B("u"),
                        B("y"),
                    ])),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("ä"),
                        B("ö"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("o"),
                        B("u"),
                        B("y"),
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
                matcher: Matcher::Literal(B("aa+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("aa+ha"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("aa+o")]),
    },
    // nu+o → nu+u: 8 words (menu, vaunu, pannu, kruunu), 0 forms wrong.
    CompiledRule {
        name: B("law41"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("nu+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("nu+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("nu+o")]),
    },
    // vu+o → vu+u after a vowel earlier in the stem: 7 words (tavu, savu, kasvu, sivu), 0 forms wrong.
    CompiledRule {
        name: B("law42"),
        left: Pattern {
            atoms: B(&[
                Atom {
                    matcher: Matcher::Class(B(&[
                        B("ä"),
                        B("ö"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("o"),
                        B("u"),
                        B("y"),
                    ])),
                    quantifier: Quantifier::One,
                    capture: None,
                },
                Atom {
                    matcher: Matcher::NotClass(B(&[
                        B("ä"),
                        B("ö"),
                        B("a"),
                        B("e"),
                        B("i"),
                        B("o"),
                        B("u"),
                        B("y"),
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
                matcher: Matcher::Literal(B("vu+o")),
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
        replacement: B(&[ReplacementPart::Literal(B("vu+u"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("vu+o")]),
    },
    // r+n → r+r: 6 words (parta, potkurivirta, alakerta, vuosikerta), 0 forms wrong.
    CompiledRule {
        name: B("law43"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("r+n")),
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
        replacement: B(&[ReplacementPart::Literal(B("r+r"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("r+n")]),
    },
    // m+v → m+m: 5 words (sampi, kampa, kompa, lempi), 0 forms wrong.
    CompiledRule {
        name: B("law44"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("m+v")),
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
        replacement: B(&[ReplacementPart::Literal(B("m+m"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("m+v")]),
    },
    // us+ea → us+ta: 5 words (kuusi, jousi, joulukuusi, hevonkuusi), 0 forms wrong.
    CompiledRule {
        name: B("law45"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("us+ea")),
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
        replacement: B(&[ReplacementPart::Literal(B("us+ta"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("us+ea")]),
    },
    // ön+e → ön+ne: 5 words (välilyönti, ihmissyönti, lyönti, läpilyönti), 0 forms wrong.
    CompiledRule {
        name: B("law46"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ön+e")),
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
        replacement: B(&[ReplacementPart::Literal(B("ön+ne"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ön+e")]),
    },
    // ön+i → ön+ni: 5 words (välilyönti, ihmissyönti, lyönti, läpilyönti), 0 forms wrong.
    CompiledRule {
        name: B("law47"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("ön+i")),
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
        replacement: B(&[ReplacementPart::Literal(B("ön+ni"))]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[B("ön+i")]),
    },
];

/// The laws, in order.
pub(super) const LAWS: Program<'static> = Program::from_rules(RULES);
