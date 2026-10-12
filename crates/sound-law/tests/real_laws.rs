//! The programs flexion learned, timed on forms built at a stem's edge (the
//! dictionary's irregular forms, split where they leave their lemma), with a
//! digest of what they make of those and of generated strings. An engine
//! change must keep every digest:
//! `cargo test -p sound-law --release --test real_laws -- --ignored --nocapture`.

extern crate self as flexion;
pub use sound_law;

use sound_law::{Matcher, Program, ReplacementPart};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;

macro_rules! programs {
    ($($name:ident: $laws:literal, $dir:literal;)*) => {
        $(#[path = $laws] mod $name;)*
        const PROGRAMS: &[(&str, &Program<'static>, &str)] = &[$((
            stringify!($name),
            &$name::LAWS,
            include_str!(concat!("../../../languages/", $dir, "/irregular.rs")),
        )),*];
    };
}

programs! {
    sa_noun: "../../../languages/sanskrit/src/noun/laws.rs", "sanskrit/src/noun";
    sa_adjective: "../../../languages/sanskrit/src/adjective/laws.rs", "sanskrit/src/adjective";
    sa_verb: "../../../languages/sanskrit/src/verb/laws.rs", "sanskrit/src/verb";
    grc_noun: "../../../languages/ancient-greek/src/noun/laws.rs", "ancient-greek/src/noun";
    grc_adjective: "../../../languages/ancient-greek/src/adjective/laws.rs", "ancient-greek/src/adjective";
    grc_verb: "../../../languages/ancient-greek/src/verb/laws.rs", "ancient-greek/src/verb";
    ru_noun: "../../../languages/russian/src/noun/laws.rs", "russian/src/noun";
    ru_adjective: "../../../languages/russian/src/adjective/laws.rs", "russian/src/adjective";
    ru_numeral: "../../../languages/russian/src/numeral/laws.rs", "russian/src/numeral";
    ru_verb: "../../../languages/russian/src/verb/laws.rs", "russian/src/verb";
    la_noun: "../../../languages/latin/src/noun/laws.rs", "latin/src/noun";
    la_verb: "../../../languages/latin/src/verb/laws.rs", "latin/src/verb";
    en_verb: "../../../languages/english/src/verb/laws.rs", "english/src/verb";
}

/// Forms marked at the stem's edge: each irregular form split where it
/// leaves its lemma, and as many again with another form's ending.
fn built(table: &str) -> Vec<String> {
    let pairs: Vec<(String, String)> = table
        .lines()
        .filter_map(|line| {
            let mut fields = line.trim_start_matches("r#\"").split('\t');
            let (lemma, form) = (fields.next()?, fields.nth(1)?);
            let stem: String = lemma
                .chars()
                .zip(form.chars())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect();
            Some((form[..stem.len()].to_owned(), form[stem.len()..].to_owned()))
        })
        .collect();
    let n = pairs.len().min(2000);
    (0..2 * n)
        .filter_map(|i| {
            let (stem, _) = pairs.get(i * 7919 % pairs.len())?;
            let (_, ending) =
                pairs.get(if i < n { i * 7919 } else { i * 104_729 } % pairs.len())?;
            Some(format!("{stem}+{ending}"))
        })
        .collect()
}

/// Strings of the text a program's rules name, its letters, `+` and `x`.
fn generated(program: &Program<'_>, count: usize) -> Vec<String> {
    let mut texts: Vec<String> = vec!["+".into(), "x".into()];
    for rule in program.rules() {
        let patterns = [&rule.left, &rule.target, &rule.right];
        let excluded = [&rule.excluded_left, &rule.excluded_right];
        for atom in patterns
            .into_iter()
            .chain(excluded.into_iter().flatten())
            .flat_map(|p| p.atoms.iter())
        {
            match &atom.matcher {
                Matcher::Literal(s) => texts.push(s.to_string()),
                Matcher::Class(m) | Matcher::NotClass(m) => {
                    texts.extend(m.iter().map(|s| s.to_string()))
                }
                _ => {}
            }
        }
        for part in rule.replacement.iter() {
            if let ReplacementPart::Literal(s) = part {
                texts.push(s.to_string());
            }
        }
    }
    let letters = texts
        .iter()
        .flat_map(|t| t.chars())
        .map(String::from)
        .collect::<Vec<_>>();
    texts.extend(letters);
    texts.sort_unstable();
    texts.dedup();
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state as usize
    };
    (0..count)
        .map(|_| {
            let len = next() % 8;
            (0..len)
                .filter_map(|_| texts.get(next() % texts.len()).map(String::as_str))
                .collect()
        })
        .collect()
}

#[test]
#[ignore = "a measurement: run in release with --nocapture"]
fn real_laws() {
    let dump = std::env::var_os("SOUND_LAW_DUMP");
    let mut out = String::new();
    for (name, program, table) in PROGRAMS {
        let forms = built(table);
        // The fastest of some rounds: the least disturbed.
        let round = || {
            let start = Instant::now();
            for form in &forms {
                std::hint::black_box(program.apply(form));
            }
            start.elapsed().as_nanos()
        };
        let ns = (0..30).map(|_| round()).min().unwrap() as f64 / forms.len() as f64;
        let mut digest = DefaultHasher::new();
        for input in forms.iter().cloned().chain(generated(program, 50_000)) {
            let output = program.apply(&input);
            output.hash(&mut digest);
            if dump.is_some() {
                out.push_str(&format!("{name}\t{input}\t{output}\n"));
            }
        }
        println!(
            "{name}: {} rules, {} forms, {ns:.0} ns a form, digest {:016x}",
            program.rule_count(),
            forms.len(),
            digest.finish()
        );
    }
    if let Some(path) = dump {
        std::fs::write(path, out).unwrap();
    }
}
