//! Writing a language's grammar as a Rust crate: its `Cargo.toml`, README and
//! data license, `src/lib.rs` (the feature enums and a typed function per
//! group of cells), `src/orthography.rs`, and a module per part of speech
//! with its paradigms, the rules choosing them, and its irregular forms.

use crate::cells::Cells;
use crate::learn::Learned;
use crate::script::Script;
use crate::spec::{Feature, Language, PartOfSpeech};
use std::fmt::Write as _;
use std::path::Path;

/// One part of speech to print.
pub struct Part<'a, 's> {
    pub spec: &'a PartOfSpeech,
    pub learned: &'a Learned<'s>,
}

/// The files of the crate, `(path under the crate, text)`.
pub fn files(lang: &Language, script: &Script, parts: &[Part<'_, '_>], report: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = vec![
        ("Cargo.toml".to_string(), manifest(lang)),
        ("README.md".to_string(), readme(lang, report)),
        ("DATA-LICENSE.md".to_string(), data_license(lang)),
        ("src/orthography.rs".to_string(), orthography(script)),
        ("src/lib.rs".to_string(), library(lang, parts)?),
    ];
    if let Some(extra) = lang.extra {
        out.push(("src/extra.rs".to_string(), extra.to_string()));
    }
    for part in parts {
        let cells = Cells::new(lang, part.spec)?;
        let layout = Layout::new(&cells);
        out.extend(part_files(lang, part, &layout));
    }
    Ok(out)
}

/// Write the crate into `dir`, replacing its `src/`.
pub fn write(dir: &Path, files: &[(String, String)]) -> Result<(), String> {
    let src = dir.join("src");
    if src.exists() {
        std::fs::remove_dir_all(&src).map_err(|e| format!("{}: {e}", src.display()))?;
    }
    for (path, text) in files {
        let path = dir.join(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(())
}

/// How a part of speech's cells are laid out in its paradigm tables: rows
/// (the last feature of a group runs across), each cell's constant name.
struct Layout {
    rows: Vec<Row>,
    /// Per cell, its constant (`GEN_PL`).
    names: Vec<String>,
}

struct Row {
    label: String,
    /// The column headings (the last feature's values), empty for one cell.
    columns: Vec<String>,
    cells: Vec<usize>,
}

impl Layout {
    fn new(cells: &Cells<'_>) -> Layout {
        let mut rows: Vec<Row> = Vec::new();
        let mut names = vec![String::new(); cells.total];
        for (gi, g) in cells.groups.iter().enumerate() {
            let n = g.features.len();
            let Some((last, rest)) = g.features.split_last() else {
                rows.push(Row {
                    label: ident(g.name),
                    columns: Vec::new(),
                    cells: vec![g.offset],
                });
                continue;
            };
            let columns: Vec<String> = last.values.iter().map(|v| ident(v.short)).collect();
            let short: Vec<String> = last.values.iter().map(|v| v.short.replace(['.', '-', ' '], "_")).collect();
            // The rows: every combination of the other features' values.
            let mut combos: Vec<Vec<usize>> = vec![Vec::new()];
            for f in rest {
                combos = combos
                    .into_iter()
                    .flat_map(|c| {
                        (0..f.values.len()).map(move |v| {
                            let mut c = c.clone();
                            c.push(v);
                            c
                        })
                    })
                    .collect();
            }
            for combo in combos {
                // A group of one feature is one row named after the group; the
                // first group's rows are named by their values alone.
                let mut parts: Vec<String> = Vec::new();
                if n == 1 || gi > 0 {
                    parts.push(ident(g.name));
                }
                let mut const_parts: Vec<String> = if n == 1 || gi > 0 { vec![g.name.to_string()] } else { Vec::new() };
                for (f, &v) in rest.iter().zip(&combo) {
                    if let Some(val) = f.values.get(v) {
                        parts.push(snake(val.name));
                        const_parts.push(val.short.replace(['.', '-', ' '], "_"));
                    }
                }
                let label = ident(&parts.join("_"));
                let const_label = const_parts.join("_");
                let row_cells: Vec<usize> = (0..last.values.len())
                    .map(|v| {
                        let mut values = combo.clone();
                        values.push(v);
                        g.index(&values)
                    })
                    .collect();
                for (&cell, column) in row_cells.iter().zip(&short) {
                    if let Some(slot) = names.get_mut(cell) {
                        *slot = const_name(&format!("{const_label}_{column}"));
                    }
                }
                rows.push(Row {
                    label,
                    columns: columns.clone(),
                    cells: row_cells,
                });
            }
        }
        for row in &rows {
            if row.columns.is_empty()
                && let Some(&cell) = row.cells.first()
                && let Some(slot) = names.get_mut(cell)
            {
                *slot = const_name(&row.label);
            }
        }
        Layout { rows, names }
    }
}

/// A cell's constant: `gen_pl` → `GEN_PL`, `non_past_1_sg` → `NON_PAST_1_SG`.
fn const_name(s: &str) -> String {
    let mut out: String = s
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
        .collect();
    out = out.trim_start_matches('_').to_string();
    if out.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        out.insert(0, 'C');
    }
    out
}

/// `pres.act` → `pres_act`; a leading digit or a keyword gets an underscore.
fn ident(s: &str) -> String {
    let mut out: String = s
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    if out.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    if KEYWORDS.contains(&out.as_str()) {
        out.push('_');
    }
    out
}

const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl",
    "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
    "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual",
    "yield", "try", "gen",
];

/// A paradigm's enum variant: its name with a capital (`книга` → `Книга`),
/// letters and digits only.
fn variant(name: &str) -> String {
    let cleaned: String = name.chars().filter(|c| c.is_alphanumeric()).collect();
    let mut chars = cleaned.chars();
    let mut out: String = match chars.next() {
        Some(first) if first.is_alphabetic() => first.to_uppercase().chain(chars).collect(),
        Some(first) => std::iter::once('P').chain(std::iter::once(first)).chain(chars).collect(),
        None => "Paradigm".to_string(),
    };
    if KEYWORDS.contains(&out.as_str()) {
        out.push('_');
    }
    out
}

/// What a part of speech's paradigms are called: conjugations for verbs,
/// declensions otherwise.
fn kind(part: &PartOfSpeech) -> (&'static str, &'static str, &'static str) {
    if part.name == "verb" {
        ("Conjugation", "conjugation", "conjugations")
    } else {
        ("Declension", "declension", "declensions")
    }
}

fn part_files(lang: &Language, part: &Part<'_, '_>, layout: &Layout) -> Vec<(String, String)> {
    let dir = format!("src/{}", part.spec.name);
    let g = &part.learned.grammar;
    let (ty, one, many) = kind(part.spec);
    // Variants, unique.
    let mut variants: Vec<String> = Vec::new();
    for p in g.paradigms.iter() {
        let mut v = variant(&p.name);
        let base = v.clone();
        let mut k = 2;
        while variants.contains(&v) {
            v = format!("{base}{k}");
            k += 1;
        }
        variants.push(v);
    }
    let name_of = |p: u16| variants.get(usize::from(p)).cloned().unwrap_or_default();
    // The feature referrals depend on, as the crate's enum names it.
    let feature_name: Option<&str> = part.learned.feature.and_then(|f| part.spec.inherent.get(f).copied());
    let mut files = Vec::new();

    // mod.rs
    let mut m = String::new();
    let words: u32 = g.paradigms.iter().map(|p| p.words).sum();
    let _ = writeln!(
        m,
        "//! {} {}: {} {many} learned from {} dictionary words, the rules on a\n//! word's ending that choose one, and the forms no {one} gives.\n//!\n//! Generated by flexion-train from Wiktionary; don't edit by hand.\n",
        lang.display,
        part.spec.doc.trim_end_matches('.').to_lowercase(),
        g.paradigms.len(),
        thousands(u64::from(words)),
    );
    if let Some(f) = feature_name {
        let _ = writeln!(m, "mod {};", snake(f));
    }
    if g.compound.is_some() {
        m.push_str("mod compounds;\n");
    }
    m.push_str("mod irregular;\nmod paradigms;\nmod rules;\n\n");
    let _ = writeln!(m, "pub use paradigms::{ty};\n");
    m.push_str("use flexion::Grammar;\nuse std::borrow::Cow;\n\n");
    m.push_str("/// Each cell's place in a paradigm.\n#[allow(dead_code)]\npub mod cell {\n");
    for (i, name) in layout.names.iter().enumerate() {
        let _ = writeln!(m, "    pub const {name}: u16 = {i};");
    }
    m.push_str("}\n\n");
    let _ = writeln!(
        m,
        "/// The grammar: paradigms, the rules choosing one, irregular forms.\npub static GRAMMAR: Grammar<'static, {ty}{generics}> = Grammar {{\n    orthography: crate::orthography::ORTHOGRAPHY,\n    cells: {},\n    lemma_cell: {:?},\n    capitalized: {},\n    paradigms: Cow::Borrowed(&paradigms::{}),\n    paradigm: rules::{},\n    referrals: {referrals},\n    feature: {feature},\n    compound: {},\n    accent: None,\n    laws: None,\n    irregular: Cow::Borrowed(irregular::IRREGULAR),\n    smoothing: {:?},\n    sharpness: {:?},\n}};",
        g.cells,
        g.lemma_cell,
        g.capitalized,
        many.to_uppercase(),
        one.to_uppercase(),
        if g.compound.is_some() { "Some(compounds::COMPOUND)" } else { "None" },
        g.smoothing,
        g.sharpness,
        generics = feature_name.map_or(String::new(), |f| format!(", u8, crate::{f}")),
        referrals = feature_name.map_or("Cow::Borrowed(&[])".to_string(), |f| format!("Cow::Borrowed(&{}::REFERRALS)", snake(f))),
        feature = feature_name.map_or("None".to_string(), |f| format!("Some({}::{})", snake(f), f.to_uppercase())),
    );
    files.push((format!("{dir}/mod.rs"), m));

    // paradigms.rs
    let mut p = String::new();
    let _ = writeln!(
        p,
        "//! The {many}, commonest first, each named after a word that follows it.\n//!\n//! A pattern is the form with `~` where the stem goes: `~а` is the stem and\n//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other\n//! forms drop. Alternatives are separated by `|`, the usual one first.\n"
    );
    p.push_str("use flexion::Paradigm;\n\n");
    // The table macro.
    let _ = writeln!(p, "/// Writes one {one}: the word it is named after, how many words follow it,\n/// the lemma's pattern, then a row of patterns per line (`referred`: the form\n/// is another cell's, as the word's {feature} says):", feature = feature_name.map_or_else(|| "feature".to_string(), str::to_lowercase));
    for row in &layout.rows {
        let cols = if row.columns.is_empty() { String::new() } else { format!(" ({})", row.columns.join(", ")) };
        let _ = writeln!(p, "/// `{}`{cols}", row.label);
    }
    let _ = writeln!(p, "macro_rules! {one} {{");
    p.push_str("    ($name:literal, $words:literal, $lemma:literal;\n");
    let mut k = 0usize;
    let mut order: Vec<usize> = Vec::new();
    for row in &layout.rows {
        let args: Vec<String> = row
            .cells
            .iter()
            .map(|&c| {
                order.push(c);
                let a = format!("$c{k}:tt");
                k += 1;
                a
            })
            .collect();
        let _ = writeln!(p, "     {}: {};", row.label, args.join(", "));
    }
    p.push_str("    ) => {\n        Paradigm {\n            name: ::std::borrow::Cow::Borrowed($name),\n            words: $words,\n            lemma: ::std::borrow::Cow::Borrowed($lemma),\n            forms: ::std::borrow::Cow::Borrowed(&[\n");
    // Forms in cell order: the k-th argument is cell order[k].
    let mut by_cell: Vec<usize> = vec![0; order.len()];
    for (arg, &cell) in order.iter().enumerate() {
        if let Some(slot) = by_cell.get_mut(cell) {
            *slot = arg;
        }
    }
    for arg in &by_cell {
        let _ = writeln!(p, "                flexion::form!($c{arg}),");
    }
    p.push_str("            ]),\n        }\n    };\n}\n\n");
    let _ = writeln!(
        p,
        "/// Every {one} learned.\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum {ty} {{"
    );
    for (v, par) in variants.iter().zip(g.paradigms.iter()) {
        let _ = writeln!(p, "    /// Like `{}`: {} words.", par.name, thousands(u64::from(par.words)));
        let _ = writeln!(p, "    {v},");
    }
    p.push_str("}\n\n");
    let _ = writeln!(
        p,
        "impl {ty} {{\n    /// Every {one}, in order.\n    pub const ALL: [{ty}; {n}] = [",
        n = variants.len()
    );
    for v in &variants {
        let _ = writeln!(p, "        {ty}::{v},");
    }
    let _ = writeln!(
        p,
        "    ];\n\n    /// Its table.\n    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {{\n        {}.get(self as usize)\n    }}\n}}\n",
        many.to_uppercase()
    );
    let _ = writeln!(
        p,
        "impl flexion::Label for {ty} {{\n    fn index(self) -> usize {{\n        self as usize\n    }}\n    fn from_index(index: usize) -> Option<Self> {{\n        Self::ALL.get(index).copied()\n    }}\n}}\n"
    );
    let _ = writeln!(
        p,
        "pub(super) static {}: [Paradigm<'static>; {}] = [",
        many.to_uppercase(),
        variants.len()
    );
    for (i, par) in g.paradigms.iter().enumerate() {
        let examples: Vec<&str> = part
            .learned
            .members
            .get(i)
            .map(|m| m.iter().map(String::as_str).take(6).collect())
            .unwrap_or_default();
        let _ = writeln!(
            p,
            "    // {}: {} words ({}).",
            variants.get(i).map_or("", String::as_str),
            thousands(u64::from(par.words)),
            examples.join(", ")
        );
        let _ = writeln!(p, "    {one}!({:?}, {}, {:?};", par.name.as_ref(), par.words, par.lemma.as_ref());
        for row in &layout.rows {
            let pats: Vec<String> = row
                .cells
                .iter()
                .map(|&c| match par.forms.get(c) {
                    Some(flexion::Form::Pattern(p)) => format!("{:?}", p.as_ref()),
                    Some(flexion::Form::Referred) => "referred".to_string(),
                    None => "\"\"".to_string(),
                })
                .collect();
            let _ = writeln!(p, "        {}: {};", row.label, pats.join(", "));
        }
        p.push_str("    ),\n");
    }
    p.push_str("];\n");
    files.push((format!("{dir}/paradigms.rs"), p));

    // rules.rs
    let mut r = String::new();
    let _ = writeln!(
        r,
        "//! Which {one} a word takes. The rule with the longest ending the word has\n//! decides, giving each {one} with how many of the dictionary's words with\n//! that ending follow it; the words no rule gets right are listed. A rule is\n//! kept only when it pays for itself: it must describe the words it covers\n//! in fewer bits than the broader rule does, by more than the rule costs.\n"
    );
    let _ = writeln!(r, "use super::paradigms::{ty}::{{self, *}};\nuse flexion::Choice;\n");
    let _ = writeln!(
        r,
        "/// {} rules and {} listed words.\npub(super) const {}: Choice<'static, {ty}> = flexion::choice! {{\n    rules {{",
        thousands(g.paradigm.rules.len() as u64),
        thousands(g.paradigm.exceptions.len() as u64),
        one.to_uppercase()
    );
    for rule in g.paradigm.rules.iter() {
        let counts: Vec<String> = rule
            .counts
            .iter()
            .map(|&(l, n)| format!("{} {n}", name_of(l)))
            .collect();
        let _ = writeln!(r, "        {:?} => [{}],", rule.ending.as_ref(), counts.join(", "));
    }
    r.push_str("    }\n    listed {\n");
    for (w, l) in g.paradigm.exceptions.iter() {
        let _ = writeln!(r, "        {:?} => {},", w.as_ref(), name_of(*l));
    }
    r.push_str("    }\n};\n");
    files.push((format!("{dir}/rules.rs"), r));

    // The feature referrals depend on.
    if let (Some(f), Some(choice)) = (feature_name, &g.feature) {
        let values = lang.feature(f).map(|x| x.values.as_slice()).unwrap_or_default();
        let value = |v: u8| values.get(usize::from(v)).map_or("", |x| x.name);
        let cell_name = |c: u16| layout.names.get(usize::from(c)).cloned().unwrap_or_default();
        let lower = f.to_lowercase();
        let mut r = String::new();
        let _ = writeln!(
            r,
            "//! {f}, which decides the cells some {many} refer to others, and which\n//! words have which. The rule with the longest ending the word has decides;\n//! the words no rule gets right are listed.\n\nuse super::cell::*;\nuse crate::{f}::{{self, *}};\nuse flexion::{{Choice, Referral}};\nuse std::borrow::Cow;\n"
        );
        let _ = writeln!(r, "/// The cells whose form, in the {many} that say `referred`, is another\n/// cell's: per {lower}, in order ({}), the cell.", values.iter().map(|x| x.name).collect::<Vec<_>>().join(", "));
        let _ = writeln!(r, "pub(super) static REFERRALS: [Referral<'static>; {}] = [", g.referrals.len());
        for referral in g.referrals.iter() {
            let to: Vec<String> = referral.to.iter().map(|&t| cell_name(t)).collect();
            let _ = writeln!(
                r,
                "    Referral {{\n        cell: {},\n        to: Cow::Borrowed(&[{}]),\n    }},",
                cell_name(referral.cell),
                to.join(", ")
            );
        }
        r.push_str("];\n\n");
        let _ = writeln!(
            r,
            "/// Which {lower} a word has: {} rules and {} listed words.\npub(super) const {}: Choice<'static, {f}> = flexion::choice! {{\n    rules {{",
            thousands(choice.rules.len() as u64),
            thousands(choice.exceptions.len() as u64),
            f.to_uppercase()
        );
        for rule in choice.rules.iter() {
            let counts: Vec<String> = rule.counts.iter().map(|&(l, n)| format!("{} {n}", value(l))).collect();
            let _ = writeln!(r, "        {:?} => [{}],", rule.ending.as_ref(), counts.join(", "));
        }
        r.push_str("    }\n    listed {\n");
        for (w, l) in choice.exceptions.iter() {
            let _ = writeln!(r, "        {:?} => {},", w.as_ref(), value(*l));
        }
        r.push_str("    }\n};\n");
        files.push((format!("{dir}/{}.rs", snake(f)), r));
    }

    // compounds.rs
    if let Some(c) = &g.compound {
        let mut r = String::new();
        let _ = writeln!(
            r,
            "//! How a word written in parts inflects: only its last part (`Last`), or each\n//! part as the word it is (`Each`). The first part decides, by its ending\n//! (the longest rule it has), or as listed.\n\nuse flexion::Choice;\nuse flexion::Compound::{{self, *}};\n"
        );
        let _ = writeln!(
            r,
            "/// {} rules and {} listed first parts.\npub(super) const COMPOUND: Choice<'static, Compound> = flexion::choice! {{\n    rules {{",
            thousands(c.rules.len() as u64),
            thousands(c.exceptions.len() as u64),
        );
        for rule in c.rules.iter() {
            let counts: Vec<String> = rule.counts.iter().map(|&(l, n)| format!("{l:?} {n}")).collect();
            let _ = writeln!(r, "        {:?} => [{}],", rule.ending.as_ref(), counts.join(", "));
        }
        r.push_str("    }\n    listed {\n");
        for (w, l) in c.exceptions.iter() {
            let _ = writeln!(r, "        {:?} => {l:?},", w.as_ref());
        }
        r.push_str("    }\n};\n");
        files.push((format!("{dir}/compounds.rs"), r));
    }

    // irregular.rs
    let mut ir = String::new();
    let _ = writeln!(
        ir,
        "//! The dictionary's forms no {one} gives, as it writes them: `(word, cell,\n//! form)`, sorted. They are the answer for that word and cell.\n"
    );
    ir.push_str("use super::cell::*;\nuse std::borrow::Cow;\n\n");
    let _ = writeln!(
        ir,
        "pub(super) static IRREGULAR: &[(Cow<'static, str>, u16, Cow<'static, str>)] = flexion::irregular! {{"
    );
    for (w, c, f) in &part.learned.irregular {
        let name = layout.names.get(*c).map_or("", String::as_str);
        let _ = writeln!(ir, "    {w:?} {name} {f:?},");
    }
    ir.push_str("};\n");
    files.push((format!("{dir}/irregular.rs"), ir));
    files
}

fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

fn orthography(script: &Script) -> String {
    let stressed: Vec<String> = script
        .stressed
        .iter()
        .map(|(c, p, k)| format!("({c:?}, {p:?}, {k})"))
        .collect();
    format!(
        "//! How the language is written: its vowels, and the marks it reads and\n//! writes.\n\nuse flexion::Orthography;\n\npub(crate) const ORTHOGRAPHY: Orthography<'static> = Orthography {{\n    lowercase: {},\n    vowels: {:?},\n    stress_marks: {:?},\n    stressed: &[{}],\n    implicit: {:?},\n    dropped: {:?},\n    mark_monosyllables: {},\n}};\n",
        script.lowercase,
        script.vowels,
        script.stress_marks,
        stressed.join(", "),
        script.implicit,
        script.dropped,
        script.mark_monosyllables,
    )
}

fn library(lang: &Language, parts: &[Part<'_, '_>]) -> Result<String, String> {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "// Generated by flexion-train from the {} config; don't edit by hand.\n\n#![doc = include_str!(\"../README.md\")]\n#![forbid(unsafe_code)]\n",
        lang.display
    );
    s.push_str("pub use flexion::{Candidate, Status};\nuse flexion::Known;\n\n");
    if lang.extra.is_some() {
        s.push_str("mod extra;\npub use extra::*;\n");
    }
    s.push_str("mod orthography;\n");
    for part in parts {
        let _ = writeln!(s, "pub mod {};", part.spec.name);
    }
    s.push('\n');
    // The features the parts of speech use.
    let mut used: Vec<&str> = Vec::new();
    for part in parts {
        let features = part.spec.groups.iter().flat_map(|g| g.features.iter());
        for f in features.chain(&part.spec.inherent) {
            if !used.contains(f) {
                used.push(f);
            }
        }
    }
    for f in lang.features.iter().filter(|f| used.contains(&f.name)) {
        feature_enum(&mut s, f);
    }
    for part in parts {
        let cells = Cells::new(lang, part.spec)?;
        for g in &cells.groups {
            let args: Vec<String> = g
                .features
                .iter()
                .map(|f| format!("{}: {}", snake(f.name), f.name))
                .collect();
            let mut index = format!("{}", g.offset);
            let mut stride = 1usize;
            let mut terms: Vec<String> = Vec::new();
            for f in g.features.iter().rev() {
                terms.push(if stride == 1 {
                    format!("{} as usize", snake(f.name))
                } else {
                    format!("{} as usize * {stride}", snake(f.name))
                });
                stride *= f.values.len();
            }
            for t in terms.iter().rev() {
                index = format!("{index} + {t}");
            }
            let params = if args.is_empty() { String::new() } else { format!(", {}", args.join(", ")) };
            let _ = writeln!(
                s,
                "/// {}\npub fn {}(lemma: &str{params}) -> String {{\n    {}::GRAMMAR.best(lemma, {index}, &Known::default())\n}}\n",
                g.group.doc, g.name, part.spec.name
            );
            let _ = writeln!(
                s,
                "/// Every possible form of [`{}`], likeliest first, each with its\n/// probability and why.\npub fn {}_candidates(lemma: &str{params}) -> Vec<Candidate<'static>> {{\n    {}::GRAMMAR.candidates(lemma, {index}, &Known::default())\n}}\n",
                g.name, g.name, part.spec.name
            );
        }
    }
    Ok(s)
}

fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    if KEYWORDS.contains(&out.as_str()) {
        out.push('_');
    }
    out
}

fn feature_enum(s: &mut String, f: &Feature) {
    let _ = writeln!(
        s,
        "/// {}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]\npub enum {} {{",
        f.doc, f.name
    );
    for v in &f.values {
        let _ = writeln!(s, "    {},", v.name);
    }
    s.push_str("}\n\n");
    let _ = writeln!(
        s,
        "impl {} {{\n    /// Every value, in order.\n    pub const ALL: [{}; {}] = [",
        f.name,
        f.name,
        f.values.len()
    );
    for v in &f.values {
        let _ = writeln!(s, "        {}::{},", f.name, v.name);
    }
    s.push_str("    ];\n\n    /// Short name, as in cell names.\n    pub const fn short(self) -> &'static str {\n        match self {\n");
    for v in &f.values {
        let _ = writeln!(s, "            {}::{} => \"{}\",", f.name, v.name, v.short);
    }
    s.push_str("        }\n    }\n}\n\n");
    let _ = writeln!(
        s,
        "impl flexion::Label for {0} {{\n    fn index(self) -> usize {{\n        self as usize\n    }}\n    fn from_index(index: usize) -> Option<Self> {{\n        Self::ALL.get(index).copied()\n    }}\n}}\n",
        f.name
    );
}

fn manifest(lang: &Language) -> String {
    format!(
        r#"# Generated by flexion-train from the {display} config. Regenerate with
# `cargo xtask build {name}`; don't edit by hand.

[package]
name = "{name}"
version = "{version}"
description = "{description}"
readme = "README.md"
license = "(MIT OR Apache-2.0) AND CC-BY-SA-4.0"
keywords = ["{name}", "inflection", "morphology", "linguistics", "grammar"]
categories = ["text-processing"]
include = ["src/**", "README.md", "DATA-LICENSE.md", "LICENSE-MIT", "LICENSE-APACHE"]
authors.workspace = true
edition.workspace = true
repository.workspace = true
rust-version.workspace = true

# The crate's tests are its README's examples (doctests).
[lib]
test = false

[dependencies]
flexion = {{ workspace = true }}

[lints]
workspace = true
"#,
        name = lang.name,
        display = lang.display,
        version = lang.version,
        description = lang.description,
    )
}

fn readme(lang: &Language, report: &str) -> String {
    let sources: String = lang
        .editions
        .iter()
        .map(|e| format!(" and {}'s", e.name))
        .collect();
    format!(
        "# {name}\n\n{intro}\n## How it works\n\n\
         This crate is {display} inflection as a grammar, learned by\n\
         [flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's{sources}\n\
         {display} entries. Each part of speech is a module under `src/`:\n\n\
         - `paradigms.rs`: how words inflect, one table per paradigm, named after a word that\n  \
           follows it, with a pattern per form (`~а`: the stem, then `а`);\n\
         - `rules.rs`: which paradigm a word takes, as rules on its ending, each with how many of\n  \
           the dictionary's words with that ending follow each paradigm, and the words no rule\n  \
           gets right, listed;\n\
         - `irregular.rs`: the forms no paradigm gives.\n\n\
         A word the dictionary lists gets its own forms. Any other word gets the paradigm its\n\
         ending suggests, with a probability that says how often such a guess is right, and the\n\
         reason (the rule and its counts).\n\n\
         {report}\n\
         ## Data and license\n\n\
         The code is MIT or Apache-2.0. The grammar under `src/` is derived from Wiktionary via\n\
         [Kaikki](https://kaikki.org/) and [wiktextract](https://github.com/tatuylonen/wiktextract),\n\
         and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); see\n\
         `DATA-LICENSE.md`.\n\n\
         ## Regenerating\n\n\
         ```sh\ncd flexion\ncargo xtask extract {name}\ncargo xtask build {name}\n```\n",
        name = lang.name,
        display = lang.display,
        intro = lang.readme_intro,
    )
}

fn data_license(lang: &Language) -> String {
    let editions: String = lang
        .editions
        .iter()
        .map(|e| format!(" and [{}](https://{}.wiktionary.org/)", e.name, e.code))
        .collect();
    let attribution: String = lang
        .editions
        .iter()
        .map(|e| format!(", <https://{}.wiktionary.org/>", e.code))
        .collect();
    format!(
        "# Data license

The grammar under `src/` is derived from [the English Wiktionary](https://en.wiktionary.org/){editions},
the free dictionary, by way of the [Kaikki](https://kaikki.org/) extraction made with
[wiktextract](https://github.com/tatuylonen/wiktextract). Wiktionary's text is available
under the [Creative Commons Attribution-ShareAlike 4.0 International
License](https://creativecommons.org/licenses/by-sa/4.0/) (CC BY-SA 4.0). The grammar is a
derivative work and is distributed under the same license.

Attribution: Wiktionary contributors, <https://en.wiktionary.org/>{attribution}.

The rest of this crate's source code is licensed under MIT OR Apache-2.0.
"
    )
}
