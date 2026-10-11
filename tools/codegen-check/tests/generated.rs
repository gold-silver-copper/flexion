//! The generated code answers exactly as the in-memory model it was
//! written from, on every fixture word, cell and input mode.

#![allow(clippy::expect_used)] // tests may panic

include!("../src/cases.rs");

#[test]
fn generated_code_answers_as_the_in_memory_model() {
    for &(module, language, joins, front, laws, derive) in CASES {
        let (lang, data, built) =
            train_fixture(language, joins, front, laws, derive).expect("trains");
        if derive {
            assert!(
                built.tables().iter().any(|t| !t.lexicon_from.is_empty()),
                "{module}: no derived lexicon"
            );
        }
        let code = codegen_check::model(module).expect("generated");
        let report = flexion_train::audit::compare(&lang, &data, &built.model(), code, usize::MAX)
            .expect("compares");
        assert!(report.answers > 0, "{module}: nothing compared");
        assert!(
            report.mismatches.is_empty(),
            "{module}: {}",
            report
                .mismatches
                .iter()
                .take(5)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

#[test]
fn generated_sound_laws_answer_as_in_memory() {
    let built = handmade().expect("builds");
    let memory = built.model();
    let code = codegen_check::model("handmade").expect("generated");
    let (a, b) = (memory.pos(0).expect("pos"), code.pos(0).expect("pos"));
    for word in ["ręka", "noga", "lipa", "mąka", "kot", "Saga"] {
        for cell in 0..2 {
            let ca = a.candidates(&a.citation(word, &[]), cell, flexion::Options::default());
            let cb = b.candidates(&b.citation(word, &[]), cell, flexion::Options::default());
            assert_eq!(ca, cb, "{word} cell {cell}");
        }
    }
    let loc = |w: &str| b.best(&b.citation(w, &[]), 1, flexion::Options::default());
    assert_eq!(loc("ręka"), "ręce");
    assert_eq!(loc("noga"), "nodze");
    assert_eq!(loc("lipa"), "lipe");
    assert_eq!(loc("Saga"), "Sadze");
}
