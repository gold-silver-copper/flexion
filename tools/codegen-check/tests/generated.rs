//! The generated code answers exactly as the in-memory model it was
//! written from, on every fixture word, cell and input mode.

#![allow(clippy::expect_used)] // tests may panic

include!("../src/cases.rs");

#[test]
fn generated_code_answers_as_the_in_memory_model() {
    for &(module, language, joins, front) in CASES {
        let (lang, data, built) = train_fixture(language, joins, front).expect("trains");
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
