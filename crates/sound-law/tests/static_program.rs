#![allow(clippy::expect_used)] // tests may panic

//! A program written as static data by `Program::to_rust` is the program
//! `compile` makes, and runs the same, with no compile step.

use sound_law::compiled::*;
use sound_law::{RuleSet, sound_laws};

const VOICING: RuleSet<'static> = sound_laws! {
    classes {
        V = ["a", "e"];
        STOP = ["p", "t", "k"];
    }
    maps {
        VOICE = ["p" => "b", "t" => "d", "k" => "g"];
    }
    rules {
        voice: "{stop:STOP}" => "{stop|VOICE}" / "{V}" _ "{V}";
        drop_final[once]: "a" => "" / "" _ "$";
    }
};

/// What `VOICING.compile()?.to_rust()` printed.
const VOICING_RULES: &[CompiledRule<'static>] = &[
    CompiledRule {
        name: B("voice"),
        left: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("a"), B("e")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("p"), B("t"), B("k")])),
                quantifier: Quantifier::One,
                capture: Some(0),
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Class(B(&[B("a"), B("e")])),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[ReplacementPart::MappedCapture {
            capture: 0,
            entries: B(&[(B("p"), B("b")), (B("t"), B("d")), (B("k"), B("g"))]),
        }]),
        capture_count: 1,
        direction: Direction::LeftToRight,
        application: Application::All,
        required: B(&[]),
    },
    CompiledRule {
        name: B("drop_final"),
        left: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        target: Pattern {
            atoms: B(&[Atom {
                matcher: Matcher::Literal(B("a")),
                quantifier: Quantifier::One,
                capture: None,
            }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern {
            atoms: B(&[]),
            anchored_at_start: false,
            anchored_at_end: true,
        },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::Once,
        required: B(&[B("a")]),
    },
];
static VOICING_STATIC: Program<'static> = Program::from_rules(VOICING_RULES);

#[test]
fn a_static_program_is_the_compiled_one() {
    let compiled = VOICING.compile().expect("compiles");
    assert_eq!(compiled, VOICING_STATIC);
    for word in ["apata", "takea", "pa", "ekepa", ""] {
        assert_eq!(VOICING_STATIC.apply(word), compiled.apply(word), "{word}");
    }
    assert_eq!(VOICING_STATIC.apply("apata"), "abad");
}

#[test]
fn to_rust_names_every_rule_in_order() {
    let rust = VOICING.compile().expect("compiles").to_rust();
    let voice = rust.find("B(\"voice\")").expect("voice");
    let drop = rust.find("B(\"drop_final\")").expect("drop_final");
    assert!(voice < drop);
    assert!(rust.starts_with("&[CompiledRule {"));
}
