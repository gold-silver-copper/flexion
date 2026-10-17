//! Learn each language's grammar from a few dozen real Wiktionary entries
//! and check what a grammar must do: give every attested form, give every
//! word every form with a probability and a reason, and, printed as a crate,
//! answer exactly as the grammar it was printed from. CI has no dumps, so
//! this is the test that the pieces fit together.

#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic)] // tests may panic

use flexion::{Known, Status};
use flexion_train::choose::Settings;
use flexion_train::eval::same_form;
use flexion_train::extract::{self, Lexeme, PosData};
use flexion_train::learn::{self, Learned};
use flexion_train::script::Script;
use flexion_train::spec::Language;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(format!("{name}.jsonl"))
}

/// The fixture's lexemes, per part of speech (the Russian edition's too).
fn data(lang: &Language, script: &Script) -> Vec<PosData> {
    let mut data = extract::extract(lang, &script.orthography(), &fixture(lang.name))
        .expect("fixture extracts");
    if lang.name == "russian" {
        extract::add_edition(
            lang,
            &script.orthography(),
            &mut data,
            0,
            &fixture("russian-ru"),
        )
        .expect("edition fixture extracts");
    }
    data
}

/// Every part of speech's grammar, learned from all its words (with the
/// language's marks model, learned once).
fn grammars<'s>(lang: &Language, script: &'s Script, data: &[PosData]) -> Vec<Learned<'s>> {
    let marks = flexion_train::marks(
        lang,
        &script.orthography(),
        data,
        |_| true,
        Settings::default(),
    )
    .expect("marks");
    lang.parts
        .iter()
        .zip(data)
        .map(|(part, d)| {
            let mut shape = flexion_train::shape(lang, part).expect("shape");
            shape.marks = marks.as_ref().map(|m| m.of(part.name));
            learn::learn(
                script.orthography(),
                &shape,
                &d.lexemes,
                Settings::default(),
            )
        })
        .collect()
}

/// The words the grammar answers for: of homographs, the first.
fn firsts<'d>(script: &Script, lexemes: &'d [Lexeme]) -> Vec<&'d Lexeme> {
    let mut sorted: Vec<&Lexeme> = lexemes.iter().collect();
    sorted.sort_by_key(|l| (l.source, l.order));
    let mut seen = HashSet::new();
    sorted
        .into_iter()
        .filter(|l| seen.insert(script.orthography().read(&l.full).letters))
        .collect()
}

#[test]
fn every_language_gives_its_dictionary_forms_with_its_principal_parts_given_or_not() {
    for lang in flexion_configs::all() {
        lang.validate().expect("valid config");
        let script = Script::new(&lang.spelling);
        let data = data(&lang, &script);
        let grammars = grammars(&lang, &script, &data);
        for ((part, d), learned) in lang.parts.iter().zip(&data).zip(&grammars) {
            let g = &learned.grammar;
            let parts = flexion_train::parts(&lang, part).expect("principal parts");
            for l in firsts(&script, &d.lexemes) {
                // The dictionary's own principal parts give nothing the
                // grammar doesn't know, and take nothing from it.
                let given: Vec<(usize, &str)> = parts
                    .iter()
                    .filter_map(|&c| Some((c, l.cells.get(c)?.first()?.as_str())))
                    .collect();
                let with_parts = Known {
                    forms: &given,
                    full: true,
                    ..Known::default()
                };
                // A word the grammar lists gives each cell as the dictionary
                // spells it, its other spellings too (English `sang|sung`).
                let listed = g.irregular.has(&l.key);
                let table = g.table(&l.full, &Known::default());
                for (cell, forms) in l.cells.iter().enumerate() {
                    let Some(want) = forms.first() else { continue };
                    for (known, how) in [
                        (Known::default(), ""),
                        (with_parts, ", given its principal parts"),
                    ] {
                        let got = g.best(&l.full, cell, &known);
                        assert!(
                            same_form(&g.orthography, &got, want),
                            "{} {}: {} cell {cell} is {want}, the grammar says {got}{how}",
                            lang.name,
                            part.name,
                            l.full
                        );
                    }
                    let top = table.get(cell).and_then(|c| c.first());
                    if listed
                        && forms.len() > 1
                        && let Some(top) = top
                    {
                        let also: Vec<&str> = top.also.iter().map(String::as_str).collect();
                        let others: Vec<&str> = forms.iter().skip(1).map(String::as_str).collect();
                        assert!(
                            also.len() == others.len()
                                && also.iter().zip(&others).all(|(a, o)| same_form(
                                    &g.orthography,
                                    a,
                                    o
                                )),
                            "{} {}: {} cell {cell} is also {others:?}, the grammar says {also:?}",
                            lang.name,
                            part.name,
                            l.full
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn a_word_beyond_the_words_learned_from_still_gives_its_dictionary_forms() {
    // Learned from the four commonest Finnish nouns only, the grammar still
    // gives every noun the dictionary has its own forms.
    let lang = flexion_configs::finnish::language();
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let (part, d) = lang.parts.iter().zip(&data).next().expect("nouns");
    let mut shape = flexion_train::shape(&lang, part).expect("shape");
    shape.max_words = Some(4);
    let learned = learn::learn(
        script.orthography(),
        &shape,
        &d.lexemes,
        Settings::default(),
    );
    let g = &learned.grammar;
    for l in firsts(&script, &d.lexemes) {
        for (cell, forms) in l.cells.iter().enumerate() {
            let Some(want) = forms.first() else { continue };
            let got = g.best(&l.full, cell, &Known::default());
            assert!(
                same_form(&g.orthography, &got, want),
                "{} cell {cell} is {want}, the grammar says {got}",
                l.full
            );
        }
    }
}

#[test]
fn every_word_has_every_form_with_a_probability_and_a_reason() {
    // Cells the dictionary leaves empty: the grammar must fill them too.
    let mut unattested = 0;
    for lang in flexion_configs::all() {
        let script = Script::new(&lang.spelling);
        let data = data(&lang, &script);
        let grammars = grammars(&lang, &script, &data);
        for ((part, d), learned) in lang.parts.iter().zip(&data).zip(&grammars) {
            let g = &learned.grammar;
            for l in &d.lexemes {
                for cell in 0..g.cells {
                    if l.cells.get(cell).is_none_or(Vec::is_empty) {
                        unattested += 1;
                    }
                    let candidates = g.candidates(&l.full, cell, &Known::default());
                    let what = format!("{} {}: {} cell {cell}", lang.name, part.name, l.full);
                    assert!(!candidates.is_empty(), "{what}: no form");
                    let total: f32 = candidates.iter().map(|c| c.probability).sum();
                    assert!(total <= 1.001, "{what}: probabilities sum to {total}");
                    for pair in candidates.windows(2) {
                        assert!(
                            pair[0].probability >= pair[1].probability,
                            "{what}: not likeliest first"
                        );
                    }
                    for c in &candidates {
                        assert!(c.probability > 0.0 && c.probability <= 1.001, "{what}: {c}");
                        // A reason: the word is listed, or a paradigm (and,
                        // when a rule chose it, the rule's evidence).
                        assert!(
                            c.status == Status::Listed || !c.paradigm.is_empty(),
                            "{what}: {c} gives no reason"
                        );
                    }
                }
            }
        }
    }
    assert!(
        unattested > 0,
        "the fixtures fill every cell, so nothing is tested"
    );
}

#[test]
fn a_word_the_dictionary_lacks_takes_its_paradigm_with_evidence() {
    let lang = flexion_configs::russian::language();
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let grammars = grammars(&lang, &script, &data);
    let noun = &grammars[0].grammar;
    // An invented noun in -а: its readings say how sure and why, and say it
    // by a rule's evidence, not a listing.
    let readings = noun.readings("бокра", &Known::default());
    let first = readings.first().expect("a reading");
    assert_ne!(first.status, Status::Listed);
    assert!(first.rule.is_some(), "{first}");
    let total: f32 = readings.iter().map(|r| r.probability).sum();
    assert!((total - 1.0).abs() < 0.01, "readings sum to {total}");
}

#[test]
fn an_inflected_form_with_its_own_table_is_no_word() {
    // `γέγονε` (a form of `γίγνομαι`) has an entry with its lemma's table;
    // the comparative `βελτίων` (of `ἀγαθός`) has one with its own.
    let lang = flexion_configs::ancient_greek::language();
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let words: Vec<&str> = data
        .iter()
        .flat_map(|d| &d.lexemes)
        .map(|l| l.lemma.as_str())
        .collect();
    assert!(!words.contains(&"γέγονε"), "{words:?}");
    assert!(words.contains(&"βελτίων"), "{words:?}");
}

#[test]
fn harmony_is_one_law_and_endings_name_its_class_letters() {
    // Turkish suffix vowels agree with the vowel before them as written,
    // through a chain of suffixes (`ev-ler-imiz-de`): laws on a tier, the
    // endings written once (`~lAr`). (On the whole dictionary it is one law
    // on the vowels; a hundred words show less.)
    let lang = flexion_configs::turkish::language();
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let grammars = grammars(&lang, &script, &data);
    let noun = &grammars[0];
    let harmony: Vec<&flexion_train::laws::Law> = noun
        .laws
        .iter()
        .filter(|l| matches!(l.change, flexion_train::laws::Change::Harmony { .. }))
        .collect();
    assert!(!harmony.is_empty(), "{:?}", noun.laws);
    let tables: Vec<&str> = noun
        .grammar
        .paradigms
        .iter()
        .map(|p| p.table.as_ref())
        .collect();
    assert!(tables.iter().any(|t| t.contains("~lAr")), "{tables:?}");
    // Words the dictionary lacks: locative plural with `our` (cell 57).
    let g = &noun.grammar;
    assert_eq!(g.best("gölüm", 57, &Known::default()), "gölümlerimizde");
    assert_eq!(g.best("tarum", 57, &Known::default()), "tarumlarımızda");
}

#[test]
fn a_printed_grammar_answers_as_the_grammar_it_was_printed_from() {
    // Russian has referrals, compounds and sound laws; Ancient Greek, accent;
    // Latin, restored vowel length; Turkish, harmony; English, verbs that
    // list a stem.
    print_and_compare(flexion_configs::russian::language());
    print_and_compare(flexion_configs::ancient_greek::language());
    print_and_compare(flexion_configs::latin::language());
    print_and_compare(flexion_configs::turkish::language());
    print_and_compare(flexion_configs::english::language());
}

/// Print `lang`'s fixture grammar as a crate and check, by compiling and
/// running it, that it answers every word's every cell as the grammar does.
fn print_and_compare(lang: Language) {
    let crate_name = lang.name.replace('-', "_");
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let learned = grammars(&lang, &script, &data);
    let parts: Vec<flexion_train::print::Part<'_, '_>> = lang
        .parts
        .iter()
        .zip(&learned)
        .map(|(spec, learned)| flexion_train::print::Part { spec, learned })
        .collect();
    let files = flexion_train::print::files(&lang, &script, &parts, "").expect("printed");
    // Russian nouns' spelling rule (`к+ы → к+и`) is a sound law, and their
    // accusative is the genitive or nominative by animacy: printed too.
    let printed = |file: &str| files.iter().any(|(path, _)| path.ends_with(file));
    assert!(
        lang.name != "russian" || (printed("noun/laws.rs") && printed("noun/relations.rs")),
        "the Russian nouns have their sound laws and relations"
    );
    // English verbs have stem slots (`sing` and `sang` are built on two
    // stems).
    assert!(
        lang.name != "english" || printed("verb/stems.rs"),
        "the English verbs have their stem slots"
    );
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("printed-{}", lang.name));
    flexion_train::print::write(&dir, &files).expect("written");
    // A crate of its own, depending on this workspace's runtime.
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("../flexion");
    std::fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{}\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n\
             [dependencies]\nflexion = {{ path = {:?} }}\n\n[workspace]\n",
            lang.name,
            runtime.display().to_string()
        ),
    )
    .expect("manifest");
    // Its test: every word's every cell, and every candidate's probability,
    // as the grammar in memory answers them: a table of answers per part of
    // speech, and a loop over it (a crate of asserts compiles slowly).
    let mut test = String::from(
        "use flexion::Known;\n\ntype Answer = (&'static str, usize, &'static [(u8, u8)], &'static [(usize, &'static str)], &'static [&'static str], &'static [f32]);\n\n",
    );
    let mut checks = String::new();
    for (part, (d, learned)) in lang.parts.iter().zip(data.iter().zip(&learned)) {
        let g = &learned.grammar;
        let table = part.name.to_uppercase();
        let _ = writeln!(test, "const {table}: &[Answer] = &[");
        for l in &d.lexemes {
            // Asked without its gender, and (in its first cell: the gender
            // picks the paradigm) with it where the dictionary says.
            let inherent: Vec<(u8, u8)> = l
                .inherent
                .iter()
                .enumerate()
                .filter(|&(_, &v)| v != u8::MAX)
                .filter_map(|(f, &v)| Some((u8::try_from(f).ok()?, v)))
                .collect();
            // A form of its last stem slot given (a principal part), where
            // its part of speech has more than one slot.
            let given: Vec<(usize, &str)> = g
                .slots
                .last()
                .filter(|_| g.slots.len() > 1)
                .and_then(|slot| slot.cells.first())
                .and_then(|&c| {
                    Some((
                        usize::from(c),
                        l.cells.get(usize::from(c))?.first()?.as_str(),
                    ))
                })
                .into_iter()
                .collect();
            for cell in 0..g.cells {
                let asks: &[&[(u8, u8)]] = if cell > 0 || inherent.is_empty() {
                    &[&[]]
                } else {
                    &[&[], &inherent]
                };
                let gives: &[&[(usize, &str)]] = if given.is_empty() {
                    &[&[]]
                } else {
                    &[&[], &given]
                };
                for (&ask, &give) in asks.iter().flat_map(|a| gives.iter().map(move |f| (a, f))) {
                    let known = Known {
                        inherent: ask,
                        forms: give,
                        ..Known::default()
                    };
                    let candidates: Vec<(String, f32)> = g
                        .candidates(&l.full, cell, &known)
                        .into_iter()
                        .map(|c| (c.form, c.probability))
                        .collect();
                    let forms: Vec<&str> = candidates.iter().map(|c| c.0.as_str()).collect();
                    let probabilities: Vec<String> =
                        candidates.iter().map(|c| format!("{:?}f32", c.1)).collect();
                    let _ = writeln!(
                        test,
                        "    ({:?}, {cell}, &{ask:?}, &{give:?}, &{forms:?}, &[{}]),",
                        l.full,
                        probabilities.join(", ")
                    );
                }
            }
        }
        test.push_str("];\n\n");
        let _ = writeln!(
            checks,
            "    for (word, cell, ask, give, forms, probabilities) in {table} {{\n        \
             let c = {crate_name}::{}::GRAMMAR.candidates(word, *cell, &Known {{ inherent: ask, forms: give, ..Known::default() }});\n        \
             assert_eq!(c.iter().map(|c| c.form.as_str()).collect::<Vec<&str>>(), *forms, \"{{word}} {{cell}}\");\n        \
             assert_eq!(c.iter().map(|c| c.probability).collect::<Vec<f32>>(), *probabilities, \"{{word}} {{cell}}\");\n    }}",
            part.name
        );
        // Each typed cell's place, as the grammar numbers its cells: every
        // group's values in order, the last fastest.
        let cells = flexion_train::cells::Cells::new(&lang, part).expect("cells");
        for g in &cells.groups {
            let cell = format!(
                "{crate_name}::{}::Cell::{}",
                part.name,
                flexion_train::print::cell_variant(part.name, g.name)
            );
            let vars: Vec<String> = (0..g.features.len()).map(|i| format!("v{i}")).collect();
            if vars.is_empty() {
                let _ = writeln!(checks, "    assert_eq!({cell}.index(), {});", g.offset);
                continue;
            }
            let _ = writeln!(checks, "    let mut place = {};", g.offset);
            for (f, v) in g.features.iter().zip(&vars) {
                let _ = writeln!(checks, "    for {v} in {crate_name}::{}::ALL {{", f.name);
            }
            let _ = writeln!(
                checks,
                "    assert_eq!({cell}({}).index(), place);\n    place += 1;\n    {}",
                vars.join(", "),
                "}".repeat(vars.len())
            );
            let _ = writeln!(checks, "    assert_eq!(place, {});", g.offset + g.len());
        }
    }
    let _ = write!(test, "#[test]\nfn answers() {{\n{checks}}}\n");
    std::fs::create_dir_all(dir.join("tests")).expect("tests dir");
    std::fs::write(dir.join("tests/answers.rs"), test).expect("test written");
    let status = Command::new(env!("CARGO"))
        .args([
            "test",
            "--quiet",
            "--offline",
            "--test",
            "answers",
            "-j",
            "4",
        ])
        .current_dir(&dir)
        .env("CARGO_TARGET_DIR", dir.join("target"))
        // The printed code is as clean as code written by hand.
        .env("RUSTFLAGS", "-D warnings")
        .status()
        .expect("cargo runs");
    assert!(
        status.success(),
        "the printed crate answers differently from its grammar"
    );
}

#[test]
fn the_same_entries_print_the_same_crate_whatever_order_the_hash_maps_take() {
    // The trainer's hash maps are filled in another order under another
    // seed: nothing it writes may depend on that order.
    let print = |lang: &Language, seed| {
        flexion_train::reseed(seed);
        let script = Script::new(&lang.spelling);
        let data = data(lang, &script);
        let learned = grammars(lang, &script, &data);
        let parts: Vec<flexion_train::print::Part<'_, '_>> = lang
            .parts
            .iter()
            .zip(&learned)
            .map(|(spec, learned)| flexion_train::print::Part { spec, learned })
            .collect();
        flexion_train::print::files(lang, &script, &parts, "").expect("printed")
    };
    for lang in [
        flexion_configs::russian::language(),
        flexion_configs::finnish::language(),
    ] {
        let a = print(&lang, 0xcbf2_9ce4_8422_2325);
        let b = print(&lang, 0x9e37_79b9_7f4a_7c15);
        for ((path, x), (_, y)) in a.iter().zip(&b) {
            assert!(
                x == y,
                "{}: {path} differs under another hash seed",
                lang.name
            );
        }
    }
}

#[test]
fn the_lemma_s_own_form_is_scored_only_where_it_is_not_what_was_typed() {
    // Typed as its headword, `rex`, the lemma's cell asks for `rēx`: vowel
    // length the headword doesn't write, a real answer. Typed `rēx`, the
    // cell's answer is the input itself. Other cells always count.
    let lang = flexion_configs::latin::language();
    let script = Script::new(&lang.spelling);
    let o = script.orthography();
    let typed_back =
        |cell, want, typed| flexion_train::eval::typed_back(&o, Some(0), cell, want, typed);
    assert!(!typed_back(0, "rēx", "rex"));
    assert!(typed_back(0, "rēx", "rēx"));
    assert!(!typed_back(1, "rēx", "rēx"));
}

#[test]
fn a_circumflex_closing_a_diphthong_is_not_vowel_length() {
    // `ῦ` alone is a long `ῡ` (`σῦς`), but in `ἀριθμοῦ` it is the
    // diphthong `ου`'s: the same letters as `ἀριθμός`'s stem and ending.
    // A diaeresis parts the two vowels: `ῧ` closes no diphthong.
    let lang = flexion_configs::ancient_greek::language();
    let script = Script::new(&lang.spelling);
    let o = script.orthography();
    assert_eq!(o.read("ἀριθμοῦ").letters, "ἀριθμου");
    assert_eq!(o.read("σῦς").letters, "σῡς");
    assert_eq!(o.read("προῧπτος").letters, "προϋπτος");
    let w = o.read("ἀριθμοῦ");
    assert_eq!(o.write(&w.letters, w.stress, w.casing), "ἀριθμοῦ");
}

#[test]
fn every_relation_s_changes_have_a_reader() {
    // A relation's changes are read where a paradigm derives its cell (or
    // derives a cell by analogy with it), or where it tells a listed word's
    // form: anything else would be printed for nothing.
    for lang in flexion_configs::all() {
        let script = Script::new(&lang.spelling);
        let data = data(&lang, &script);
        for (part, learned) in lang.parts.iter().zip(grammars(&lang, &script, &data)) {
            let g = &learned.grammar;
            let derives = |cell: usize| {
                g.paradigms
                    .iter()
                    .any(|p| p.pattern_in(&g.paradigms, g.space(), cell).is_none())
            };
            let listed: HashSet<&str> = learned
                .irregular
                .iter()
                .map(|(k, _, _)| k.as_str())
                .collect();
            let told: HashSet<usize> = listed
                .iter()
                .flat_map(|k| g.listed_sources(k))
                .map(|(cell, _)| cell)
                .collect();
            for r in g.relations.iter().filter(|r| !r.changes.is_empty()) {
                let cell = usize::from(r.cell);
                let read = g
                    .relations
                    .iter()
                    .any(|q| g.changing(q).cell == r.cell && derives(usize::from(q.cell)))
                    || told.contains(&cell);
                assert!(
                    read,
                    "{} {}: the relation of cell {cell} has changes nothing reads",
                    lang.name, part.name
                );
            }
        }
    }
}

#[test]
fn the_perfect_passive_is_the_participle_and_sum() {
    // The tables write it as a recipe (`fātus + present active indicative of
    // sum`): it is a phrase per person and number, the participle agreeing
    // in number, and the grammar gives it back.
    let lang = flexion_configs::latin::language();
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let (_, d) = lang
        .parts
        .iter()
        .zip(&data)
        .find(|(p, _)| p.name == "verb")
        .expect("verbs");
    let forms: Vec<&str> = d
        .lexemes
        .iter()
        .flat_map(|l| l.cells.iter().flatten())
        .map(String::as_str)
        .collect();
    assert!(forms.contains(&"fātus sum") && forms.contains(&"fātī sunt"));
    assert!(!forms.iter().any(|f| f.contains(" + ")));
    let grammars = grammars(&lang, &script, &data);
    let g = &grammars[lang
        .parts
        .iter()
        .position(|p| p.name == "verb")
        .expect("verb")]
    .grammar;
    let l = d
        .lexemes
        .iter()
        .find(|l| l.cells.iter().flatten().any(|f| f == "fātus sum"))
        .expect("the verb");
    for (cell, fs) in l.cells.iter().enumerate() {
        if let Some(want) = fs.first().filter(|f| f.contains(' ')) {
            assert_eq!(&g.best(&l.full, cell, &Known::default()), want);
        }
    }
}

/// The crate's files printed from `learned` and `measured`, with their
/// report.
fn printed(
    lang: &Language,
    script: &Script,
    learned: &[Learned<'_>],
    measured: &[flexion_train::pipeline::Measured],
) -> Vec<(String, String)> {
    let report = flexion_train::report::report(lang, learned, measured);
    let parts: Vec<flexion_train::print::Part<'_, '_>> = lang
        .parts
        .iter()
        .zip(learned)
        .map(|(spec, learned)| flexion_train::print::Part { spec, learned })
        .collect();
    flexion_train::print::files(lang, script, &parts, &report).expect("printed")
}

#[test]
fn a_saved_build_prints_the_crate_its_learning_printed_and_rewrites_nothing() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("saved");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("dir");
    // Other data, a letter apart, is another key.
    let english = flexion_configs::english::language();
    let key = |path: &Path| {
        flexion_train::saved::key(&english, Settings::default(), &[path]).expect("key")
    };
    let text = std::fs::read_to_string(fixture("english")).expect("fixture");
    let other = dir.join("english.jsonl");
    let changed = text.replacen("walk", "wolk", 1);
    assert_ne!(changed, text, "the fixture has `walk`");
    std::fs::write(&other, changed).expect("written");
    assert_ne!(key(&fixture("english")), key(&other));
    for lang in flexion_configs::all() {
        let script = Script::new(&lang.spelling);
        let data = data(&lang, &script);
        // Turkish, whose harmony laws no other fixture has, learns slowly:
        // its shipped grammars alone.
        let (learned, measured) = if lang.name == "turkish" {
            (grammars(&lang, &script, &data), Vec::new())
        } else {
            flexion_train::pipeline::build(&lang, &script, &data, Settings::default())
                .expect("built")
        };
        saved_and_printed(&lang, &script, &learned, &measured, &dir);
    }
}

/// Save `learned` and `measured`, load them, and check that they print the
/// same crate; that writing it again over the first touches no file; and
/// that a file it no longer has goes.
fn saved_and_printed(
    lang: &Language,
    script: &Script,
    learned: &[Learned<'_>],
    measured: &[flexion_train::pipeline::Measured],
    dir: &Path,
) {
    use flexion_train::saved;
    let key = saved::key(lang, Settings::default(), &[&fixture(lang.name)]).expect("key");
    saved::save(dir, lang.name, key, script, learned, measured).expect("saved");
    let (kept, kept_measured) = saved::load(dir, lang.name, key, script)
        .expect("readable")
        .expect("kept");
    let fresh = printed(lang, script, learned, measured);
    let again = printed(lang, script, &kept, &kept_measured);
    assert_eq!(fresh.len(), again.len(), "{}: the files", lang.name);
    for ((path, a), (_, b)) in fresh.iter().zip(&again) {
        assert!(
            a == b,
            "{}: {path} differs printed from the saved build",
            lang.name
        );
    }
    let crate_dir = dir.join(lang.name);
    flexion_train::print::write(&crate_dir, &fresh).expect("written");
    let stale = crate_dir.join("src/stale.rs");
    std::fs::write(&stale, "").expect("stale file");
    let then = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(86_400);
    for (path, _) in &fresh {
        let file = std::fs::File::options()
            .write(true)
            .open(crate_dir.join(path))
            .expect("printed file");
        file.set_modified(then).expect("set its time");
    }
    let written = flexion_train::print::write(&crate_dir, &again).expect("written again");
    assert_eq!(written, 0, "{}: files written again", lang.name);
    for (path, _) in &fresh {
        let modified = std::fs::metadata(crate_dir.join(path))
            .and_then(|m| m.modified())
            .expect("its time");
        assert_eq!(modified, then, "{}: {path} touched", lang.name);
    }
    assert!(!stale.exists(), "{}: a stale file kept", lang.name);
}

#[test]
fn the_augment_is_one_law_whatever_vowel_it_meets() {
    // Of the verbs, five begin with a vowel their class's augment lengthens
    // (ἀκούω, αἰτέω, οἰκέω, ὀνομάζω, αὐλέω): no vowel has the five words a
    // law needs, the five together do, and the augment is one law (Smyth
    // §§ 428–438). It lengthens a vowel no verb begins with too: αἰνέω, and
    // through `α` the `αὐ` of αὐδάω.
    let lang = flexion_configs::ancient_greek::language();
    let script = Script::new(&lang.spelling);
    let data = data(&lang, &script);
    let grammars = grammars(&lang, &script, &data);
    let (at, _) = lang
        .parts
        .iter()
        .enumerate()
        .find(|(_, p)| p.name == "verb")
        .expect("verbs");
    let verb = &grammars[at];
    let augments: Vec<&flexion_train::laws::Law> = verb
        .laws
        .iter()
        .filter(|l| {
            matches!(&l.change, flexion_train::laws::Change::Front { literal, .. } if literal == "ἐ")
        })
        .collect();
    assert_eq!(augments.len(), 1, "{:?}", verb.laws);
    let flexion_train::laws::Change::Front { map, .. } = &augments[0].change else {
        unreachable!()
    };
    assert!(map.len() >= 4, "{map:?}");
    // The imperfect active, 1st singular.
    let g = &verb.grammar;
    assert_eq!(g.best("αἰνέω", 99, &Known::default()), "ᾔνουν");
    assert_eq!(g.best("αὐδάω", 99, &Known::default()), "ηὔδαον");
}
