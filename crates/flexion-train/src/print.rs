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
pub fn files(
    lang: &Language,
    script: &Script,
    parts: &[Part<'_, '_>],
    report: &str,
) -> Result<Vec<(String, String)>, String> {
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
        out.extend(part_files(lang, part, &cells, &layout));
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
            let short: Vec<String> = last
                .values
                .iter()
                .map(|v| v.short.replace(['.', '-', ' '], "_"))
                .collect();
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
                let mut const_parts: Vec<String> = if n == 1 || gi > 0 {
                    vec![g.name.to_string()]
                } else {
                    Vec::new()
                };
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
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
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
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "gen",
];

/// A paradigm's enum variant: its name with a capital (`книга` → `Книга`),
/// letters and digits only.
fn variant(name: &str) -> String {
    let cleaned: String = name.chars().filter(|c| c.is_alphanumeric()).collect();
    let mut chars = cleaned.chars();
    let mut out: String = match chars.next() {
        Some(first) if first.is_alphabetic() => first.to_uppercase().chain(chars).collect(),
        Some(first) => std::iter::once('P')
            .chain(std::iter::once(first))
            .chain(chars)
            .collect(),
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

fn part_files(
    lang: &Language,
    part: &Part<'_, '_>,
    cells: &Cells<'_>,
    layout: &Layout,
) -> Vec<(String, String)> {
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
    let feature_name: Option<&str> = part
        .learned
        .feature
        .and_then(|f| part.spec.inherent.get(f).copied());
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
    if g.accent.is_some() {
        m.push_str("pub mod accent;\n");
    }
    if g.compound.is_some() {
        m.push_str("mod compounds;\n");
    }
    if g.prefixes.is_some() {
        m.push_str("mod prefixes;\n");
    }
    m.push_str("mod irregular;\nmod paradigms;\nmod rules;\nmod spellings;\n\n");
    let _ = writeln!(m, "pub use paradigms::{ty};\n");
    m.push_str("use crate::*;\nuse flexion::{{Candidate, Grammar, Known, Reading}};\nuse std::borrow::Cow;\n\n");
    m.push_str("/// Each cell's place in a paradigm.\n#[allow(dead_code)]\npub mod cell {\n");
    for (i, name) in layout.names.iter().enumerate() {
        let _ = writeln!(m, "    pub const {name}: u16 = {i};");
    }
    m.push_str("}\n\n");
    let _ = writeln!(
        m,
        "/// The grammar: paradigms, the rules choosing one, irregular forms.\npub static GRAMMAR: Grammar<'static, {ty}{generics}> = Grammar {{\n    orthography: crate::orthography::ORTHOGRAPHY,\n    cells: {},\n    lemma_cell: {:?},\n    capitalized: {},\n    paradigms: Cow::Borrowed(&paradigms::{}),\n    paradigm: rules::{},\n    referrals: {referrals},\n    feature: {feature},\n    compound: {},\n    prefixes: {prefixes},\n    accent: {accent},\n    laws: None,\n    spellings: Cow::Borrowed(spellings::SPELLINGS),\n    hidden: {hidden:?},\n    irregular: Cow::Borrowed(irregular::IRREGULAR),\n    smoothing: {:?},\n    sharpness: {:?},\n}};",
        g.cells,
        g.lemma_cell,
        g.capitalized,
        many.to_uppercase(),
        one.to_uppercase(),
        if g.compound.is_some() {
            "Some(compounds::COMPOUND)"
        } else {
            "None"
        },
        g.smoothing,
        g.sharpness,
        hidden = g.hidden,
        generics = match (g.accent.is_some(), feature_name) {
            (false, None) => String::new(),
            (true, None) => ", accent::AccentClass".to_string(),
            (false, Some(f)) => format!(", u8, crate::{f}"),
            (true, Some(f)) => format!(", accent::AccentClass, crate::{f}"),
        },
        prefixes = if g.prefixes.is_some() {
            "Some(prefixes::PREFIXES)"
        } else {
            "None"
        },
        accent = if g.accent.is_some() {
            "Some(flexion::Accent {\n        patterns: Cow::Borrowed(&accent::PATTERNS),\n        classes: Cow::Borrowed(&accent::CLASSES),\n        class: accent::CLASS,\n        circumflex: Cow::Borrowed(&accent::CIRCUMFLEX),\n        long_finals: Cow::Borrowed(&accent::LONG_FINALS),\n    })"
        } else {
            "None"
        },
        referrals = feature_name.map_or("Cow::Borrowed(&[])".to_string(), |f| format!(
            "Cow::Borrowed(&{}::REFERRALS)",
            snake(f)
        )),
        feature = feature_name.map_or("None".to_string(), |f| format!(
            "Some({}::{})",
            snake(f),
            f.to_uppercase()
        )),
    );
    m.push('\n');
    m.push_str(&api(part, cells, layout, ty, feature_name));
    files.push((format!("{dir}/mod.rs"), m));

    // paradigms.rs
    let mut p = String::new();
    let _ = writeln!(
        p,
        "//! The {many}, commonest first, each named after a word that follows it.\n//!\n//! A pattern is the form with `~` where the stem goes: `~а` is the stem and\n//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other\n//! forms drop. Alternatives are separated by `|`, the usual one first.\n"
    );
    p.push_str("use flexion::Paradigm;\n\n");
    // The table macro.
    let _ = writeln!(
        p,
        "/// Writes one {one}: the word it is named after, how many words follow it,\n/// the lemma's pattern, then a row of patterns per line (`referred`: the form\n/// is another cell's, as the word's {feature} says):",
        feature = feature_name.map_or_else(|| "feature".to_string(), str::to_lowercase)
    );
    for row in &layout.rows {
        let cols = if row.columns.is_empty() {
            String::new()
        } else {
            format!(" ({})", row.columns.join(", "))
        };
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
        let _ = writeln!(
            p,
            "    /// Like `{}`: {} words.",
            par.name,
            thousands(u64::from(par.words))
        );
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
        let _ = writeln!(
            p,
            "    {one}!({:?}, {}, {:?};",
            par.name.as_ref(),
            par.words,
            par.lemma.as_ref()
        );
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
    if g.paradigms.is_empty() {
        p = without_macro(&p, "/// Writes one", "/// Every ");
    }
    files.push((format!("{dir}/paradigms.rs"), p));

    // rules.rs
    let mut r = String::new();
    let _ = writeln!(
        r,
        "//! Which {one} a word takes. The rule with the longest ending the word has\n//! decides, giving each {one} with how many of the dictionary's words with\n//! that ending follow it; the words no rule gets right are listed. A rule is\n//! kept only when it pays for itself: it must describe the words it covers\n//! in fewer bits than the broader rule does, by more than the rule costs.\n"
    );
    let _ = writeln!(
        r,
        "use super::paradigms::{ty}{};\nuse flexion::Choice;\n",
        if g.paradigm.rules.is_empty() && g.paradigm.exceptions.is_empty() {
            ""
        } else {
            "::{self, *}"
        }
    );
    let _ = writeln!(
        r,
        "/// {} rules and {} listed words.\npub(super) const {}: Choice<'static, {ty}> = flexion::choice! {{\n    rules {{",
        thousands(g.paradigm.rules.len() as u64),
        thousands(g.paradigm.exceptions.len() as u64),
        one.to_uppercase()
    );
    r.push_str(&choice_body(&g.paradigm, &name_of));
    r.push_str("    }\n};\n");
    files.push((format!("{dir}/rules.rs"), r));

    // The feature referrals depend on.
    if let (Some(f), Some(choice)) = (feature_name, &g.feature) {
        let values = lang
            .feature(f)
            .map(|x| x.values.as_slice())
            .unwrap_or_default();
        let value = |v: u8| values.get(usize::from(v)).map_or("", |x| x.name);
        let cell_name = |c: u16| {
            layout
                .names
                .get(usize::from(c))
                .cloned()
                .unwrap_or_default()
        };
        let lower = f.to_lowercase();
        let mut r = String::new();
        let _ = writeln!(
            r,
            "//! {f}, which decides the cells some {many} refer to others, and which\n//! words have which. The rule with the longest ending the word has decides;\n//! the words no rule gets right are listed.\n\nuse super::cell::*;\nuse crate::{f}::{{self, *}};\nuse flexion::{{Choice, Referral}};\nuse std::borrow::Cow;\n"
        );
        let _ = writeln!(
            r,
            "/// The cells whose form, in the {many} that say `referred`, is another\n/// cell's: per {lower}, in order ({}), the cell.",
            values.iter().map(|x| x.name).collect::<Vec<_>>().join(", ")
        );
        let _ = writeln!(
            r,
            "pub(super) static REFERRALS: [Referral<'static>; {}] = [",
            g.referrals.len()
        );
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
        r.push_str(&choice_body(choice, |l| value(l).to_string()));
        r.push_str("    }\n};\n");
        files.push((format!("{dir}/{}.rs", snake(f)), r));
    }

    if let Some(a) = &g.accent {
        files.push((
            format!("{dir}/accent.rs"),
            accent_file(a, layout, &variants, many),
        ));
    }

    if let Some(p) = &g.prefixes {
        files.push((format!("{dir}/prefixes.rs"), prefixes_file(p, layout)));
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
        r.push_str(&choice_body(c, |l| format!("{l:?}")));
        r.push_str("    }\n};\n");
        files.push((format!("{dir}/compounds.rs"), r));
    }

    // spellings.rs
    let mut sp = String::new();
    let _ = writeln!(
        sp,
        "//! Words whose spelling their key doesn't show, as the dictionary writes\n//! them: a caller who types a word without its marks means this. Marks a\n//! {one} writes itself aren't listed.\n\nuse std::borrow::Cow;\n"
    );
    let _ = writeln!(
        sp,
        "/// {} words.\npub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {{",
        thousands(g.spellings.len() as u64)
    );
    for (k, w) in g.spellings.iter() {
        let _ = writeln!(sp, "    {:?} => {:?},", k.as_ref(), w.as_ref());
    }
    sp.push_str("};\n");
    files.push((format!("{dir}/spellings.rs"), sp));

    // irregular.rs
    let mut ir = String::new();
    let _ = writeln!(
        ir,
        "//! The dictionary's forms no {one} gives, as it writes them: `(word, cell,\n//! form)`, sorted. They are the answer for that word and cell.\n"
    );
    if !part.learned.irregular.is_empty() {
        ir.push_str("use super::cell::*;\n");
    }
    ir.push_str("use std::borrow::Cow;\n\n");
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
        "//! How the language is written: its vowels, and the marks it reads and\n//! writes.\n\nuse flexion::Orthography;\n\npub(crate) const ORTHOGRAPHY: Orthography<'static> = Orthography {{\n    lowercase: {},\n    vowels: {:?},\n    stress_marks: {:?},\n    stressed: &[{}],\n    implicit: {:?},\n    dropped: {:?},\n    mark_monosyllables: {},\n    folded: &[{}],\n    prosody: {},\n}};\n",
        script.lowercase,
        script.vowels,
        script.stress_marks,
        stressed.join(", "),
        script.implicit,
        script.dropped,
        script.mark_monosyllables,
        script
            .folded
            .iter()
            .map(|(a, b)| format!("({a:?}, {b:?})"))
            .collect::<Vec<_>>()
            .join(", "),
        script.prosody.as_ref().map_or("None".to_string(), |p| format!(
            "Some(flexion::Prosody {{\n        diphthongs: &{:?},\n        hiatus: {:?},\n        long: {:?},\n        short_finals: &{:?},\n        acute: {},\n        circumflex: {},\n    }})",
            p.diphthongs, p.hiatus, p.long, p.short_finals, p.acute, p.circumflex
        )),
    )
}

fn library(lang: &Language, parts: &[Part<'_, '_>]) -> Result<String, String> {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "// Generated by flexion-train from the {} config; don't edit by hand.\n\n#![doc = include_str!(\"../README.md\")]\n#![forbid(unsafe_code)]\n",
        lang.display
    );
    s.push_str("pub use flexion::{Candidate, Reading, Status, UnknownValue};\n\n");
    if lang.extra.is_some() {
        s.push_str("mod extra;\npub use extra::*;\n");
    }
    s.push_str("mod orthography;\n");
    for part in parts {
        let _ = writeln!(s, "pub mod {};", part.spec.name);
    }
    for part in parts {
        let _ = writeln!(
            s,
            "pub use {}::{};",
            part.spec.name,
            variant(part.spec.name)
        );
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
        let word = variant(part.spec.name);
        for g in &cells.groups {
            let args: Vec<String> = g
                .features
                .iter()
                .map(|f| format!("{}: {}", snake(f.name), f.name))
                .collect();
            let values: Vec<String> = g.features.iter().map(|f| snake(f.name)).collect();
            let params = if args.is_empty() {
                String::new()
            } else {
                format!(", {}", args.join(", "))
            };
            let cell = if values.is_empty() {
                format!(
                    "{}::Cell::{}",
                    part.spec.name,
                    cell_variant(part.spec.name, g.name)
                )
            } else {
                format!(
                    "{}::Cell::{}({})",
                    part.spec.name,
                    cell_variant(part.spec.name, g.name),
                    values.join(", ")
                )
            };
            let _ = writeln!(
                s,
                "/// {}\n#[must_use]\npub fn {}(lemma: &str{params}) -> String {{\n    {word}::new(lemma).form({cell})\n}}\n",
                g.group.doc, g.name
            );
        }
    }
    Ok(s)
}

/// A part of speech's accent, written out: its patterns as tables of places
/// (in the paradigms' rows), its classes, the rules choosing a class, and
/// what endings say about the last syllable.
fn accent_file(
    a: &flexion::Accent<'_, u16>,
    layout: &Layout,
    paradigms: &[String],
    many: &str,
) -> String {
    let mut s = String::from(
        "//! Accent. Each word has a class: an accent pattern, which says cell by cell\n\
         //! which rule places the accent, and the syllable its stem is accented on,\n\
         //! counted from the lemma's end. `Stem`: where the lemma has it (persistent);\n\
         //! `Ending`: on the ending's first syllable; `Recessive`: as far from the end\n\
         //! as the law of limitation allows. The law then moves it where it must, and\n\
         //! chooses acute or circumflex by the syllables' length.\n\n\
         use flexion::{AccentPattern, Choice, StressClass};\n\
         use std::borrow::Cow;\n",
    );
    if a.circumflex.iter().any(|c| !c.is_empty()) || !a.long_finals.is_empty() {
        s.push_str("use super::cell::*;\n");
    }
    if !a.patterns.is_empty() {
        s.push_str("use flexion::Place::*;\n");
    }
    s.push('\n');
    let cell_name = |c: u16| {
        layout
            .names
            .get(usize::from(c))
            .cloned()
            .unwrap_or_default()
    };
    // The pattern table macro.
    s.push_str(
        "/// Writes one accent pattern: the word it is named after, how many words\n/// follow it, then a row of places per line, as the paradigm tables are laid out.\nmacro_rules! pattern {\n    ($name:literal, $words:literal;\n",
    );
    let mut k = 0usize;
    let mut order: Vec<usize> = Vec::new();
    for row in &layout.rows {
        let args: Vec<String> = row
            .cells
            .iter()
            .map(|&c| {
                order.push(c);
                k += 1;
                format!("$c{}:ident", k - 1)
            })
            .collect();
        let _ = writeln!(s, "     {}: {};", row.label, args.join(", "));
    }
    let mut by_cell: Vec<usize> = vec![0; order.len()];
    for (arg, &cell) in order.iter().enumerate() {
        if let Some(slot) = by_cell.get_mut(cell) {
            *slot = arg;
        }
    }
    s.push_str("    ) => {\n        AccentPattern {\n            name: Cow::Borrowed($name),\n            words: $words,\n            places: Cow::Borrowed(&[");
    let places: Vec<String> = by_cell.iter().map(|a| format!("$c{a}")).collect();
    s.push_str(&places.join(", "));
    s.push_str("]),\n        }\n    };\n}\n\n");
    let _ = writeln!(
        s,
        "/// The accent patterns, commonest first.\npub static PATTERNS: [AccentPattern<'static>; {}] = [",
        a.patterns.len()
    );
    for p in a.patterns.iter() {
        let _ = writeln!(
            s,
            "    // Like `{}`: {} words.\n    pattern!({:?}, {};",
            p.name,
            thousands(u64::from(p.words)),
            p.name.as_ref(),
            p.words
        );
        for row in &layout.rows {
            let cells: Vec<String> = row
                .cells
                .iter()
                .map(|&c| {
                    format!(
                        "{:?}",
                        p.places.get(c).copied().unwrap_or(flexion::Place::Stem)
                    )
                })
                .collect();
            let _ = writeln!(s, "        {}: {};", row.label, cells.join(", "));
        }
        s.push_str("    ),\n");
    }
    s.push_str("];\n\n");
    // Classes.
    let mut names: Vec<String> = Vec::new();
    for c in a.classes.iter() {
        let base = variant(&c.name);
        let mut v = base.clone();
        let mut n = 2;
        while names.contains(&v) {
            v = format!("{base}{n}");
            n += 1;
        }
        names.push(v);
    }
    s.push_str(
        "/// Every accent class: a pattern and the stem's accented syllable.\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum AccentClass {\n",
    );
    for (v, c) in names.iter().zip(a.classes.iter()) {
        let pattern = a
            .patterns
            .get(usize::from(c.pattern))
            .map_or("", |p| p.name.as_ref());
        let syllable = c
            .vowel
            .map_or("never on the stem".to_string(), |v| match v {
                0 => "the stem accented on the lemma's last syllable".to_string(),
                1 => "the stem accented on its next-to-last".to_string(),
                n => format!("the stem accented on its syllable {} from the end", n + 1),
            });
        let _ = writeln!(
            s,
            "    /// Pattern `{pattern}`, {syllable}: {} words.\n    {v},",
            thousands(u64::from(c.words))
        );
    }
    let _ = writeln!(
        s,
        "}}\n\nimpl AccentClass {{\n    /// Every class, in order.\n    pub const ALL: [AccentClass; {n}] = [{}];\n}}\n\nimpl flexion::Label for AccentClass {{\n    fn index(self) -> usize {{\n        self as usize\n    }}\n    fn from_index(index: usize) -> Option<Self> {{\n        Self::ALL.get(index).copied()\n    }}\n}}\n",
        names
            .iter()
            .map(|v| format!("AccentClass::{v}"))
            .collect::<Vec<_>>()
            .join(", "),
        n = names.len()
    );
    let _ = writeln!(
        s,
        "/// The classes' patterns and stem syllables.\npub static CLASSES: [StressClass<'static>; {}] = [",
        a.classes.len()
    );
    for c in a.classes.iter() {
        let pattern = a
            .patterns
            .get(usize::from(c.pattern))
            .map_or("", |p| p.name.as_ref());
        let _ = writeln!(
            s,
            "    StressClass {{\n        name: Cow::Borrowed({:?}),\n        pattern: {}, // {pattern}\n        vowel: {:?},\n        words: {},\n    }},",
            c.name.as_ref(),
            c.pattern,
            c.vowel,
            c.words
        );
    }
    s.push_str("];\n\n");
    let value = |l: u16| names.get(usize::from(l)).cloned().unwrap_or_default();
    let _ = writeln!(
        s,
        "{}/// Which class a word is of: {} rules and {} listed words.\npub const CLASS: Choice<'static, AccentClass> = flexion::choice! {{\n    rules {{",
        if a.class.rules.is_empty() && a.class.exceptions.is_empty() {
            ""
        } else {
            "use AccentClass::*;\n\n"
        },
        thousands(a.class.rules.len() as u64),
        thousands(a.class.exceptions.len() as u64)
    );
    s.push_str(&choice_body(&a.class, &value));
    s.push_str("    }\n};\n\n");
    let _ = writeln!(
        s,
        "/// Per {}, the cells whose ending takes the circumflex when accented on\n/// its last syllable (`θεοῦ`).\npub static CIRCUMFLEX: [Cow<'static, [u16]>; {}] = [",
        many.trim_end_matches('s'),
        a.circumflex.len()
    );
    for (cells, p) in a.circumflex.iter().zip(paradigms) {
        let names: Vec<String> = cells.iter().map(|&c| cell_name(c)).collect();
        let _ = writeln!(s, "    Cow::Borrowed(&[{}]), // {p}", names.join(", "));
    }
    s.push_str("];\n\n");
    if a.patterns.is_empty() {
        s = without_macro(
            &s,
            "/// Writes one accent pattern",
            "/// The accent patterns",
        );
    }
    let finals: Vec<String> = a.long_finals.iter().map(|&c| cell_name(c)).collect();
    let _ = writeln!(
        s,
        "/// The cells whose final `-αι`, `-οι` counts as long.\npub static LONG_FINALS: [u16; {}] = [{}];",
        finals.len(),
        finals.join(", ")
    );
    s
}

/// A choice's insides as `flexion::choice!` reads them, after its opening
/// `rules {`: its rules and its listed words (left open for the caller to
/// close), each value written by `name`.
fn choice_body<L: flexion::Label>(
    choice: &flexion::Choice<'_, L>,
    name: impl Fn(L) -> String,
) -> String {
    let mut s = String::new();
    for rule in choice.rules.iter() {
        let counts: Vec<String> = rule
            .counts
            .iter()
            .map(|&(l, n)| format!("{} {n}", name(l)))
            .collect();
        let _ = writeln!(
            s,
            "        {:?} => [{}],",
            rule.ending.as_ref(),
            counts.join(", ")
        );
    }
    s.push_str("    }\n    listed {\n");
    for (w, l) in choice.exceptions.iter() {
        let _ = writeln!(s, "        {:?} => {},", w.as_ref(), name(*l));
    }
    s
}

/// `text` without the macro that starts at `from`, up to `to` (a table
/// with no rows doesn't need its macro).
fn without_macro(text: &str, from: &str, to: &str) -> String {
    match (text.find(from), text.find(to)) {
        (Some(a), Some(b)) if a < b => format!(
            "{}{}",
            text.get(..a).unwrap_or(""),
            text.get(b..).unwrap_or("")
        ),
        _ => text.to_string(),
    }
}

/// The prefixes words inflect behind, written out: each prefix with how many
/// words separate it or keep it, the words that go against most, and per
/// cell where a separable prefix goes and what an inseparable one takes off.
fn prefixes_file(p: &flexion::Prefixes<'_>, layout: &Layout) -> String {
    let mut s = String::from(
        "//! Prefixes words inflect behind. A word that begins with one inflects as the\n\
         //! rest of it does: a separable prefix goes where the cell puts it (`Before`\n\
         //! the rest, written together, or `After` it, as a word of its own); an\n\
         //! inseparable one stays in front and takes off the front of the rest's form\n\
         //! what the cell says. How likely a word is to do either is how many of the\n\
         //! dictionary's words with its prefix do.\n\n\
         use flexion::Placement::*;\n\
         use flexion::{Prefix, Prefixes, Separation};\n\
         use std::borrow::Cow;\n\n",
    );
    let _ = writeln!(
        s,
        "/// {} prefixes and {} words that go against most with theirs.\npub(super) const PREFIXES: Prefixes<'static> = Prefixes {{\n    prefixes: Cow::Borrowed(&[",
        p.prefixes.len(),
        thousands(p.listed.len() as u64)
    );
    for x in p.prefixes.iter() {
        let _ = writeln!(
            s,
            "        Prefix {{ letters: Cow::Borrowed({:?}), separable: {}, inseparable: {}, words: {} }},",
            x.letters.as_ref(),
            x.separable,
            x.inseparable,
            x.words
        );
    }
    s.push_str("    ]),\n    listed: Cow::Borrowed(&[\n");
    for (w, sep) in p.listed.iter() {
        let _ = writeln!(
            s,
            "        (Cow::Borrowed({:?}), Separation::{sep:?}),",
            w.as_ref()
        );
    }
    s.push_str("    ]),\n    placement: Cow::Borrowed(&[\n");
    for (c, place) in p.placement.iter().enumerate() {
        let name = layout.names.get(c).map_or("", String::as_str);
        let _ = writeln!(s, "        {place:?}, // {name}");
    }
    s.push_str("    ]),\n    drops: Cow::Borrowed(&[\n");
    for (c, d) in p.drops.iter().enumerate() {
        let name = layout.names.get(c).map_or("", String::as_str);
        let _ = writeln!(s, "        Cow::Borrowed({:?}), // {name}", d.as_ref());
    }
    s.push_str("    ]),\n};\n");
    s
}

/// A cell group's variant of a part of speech's `Cell`: `noun_second_case`
/// → `SecondCase`; the part's own group is `Form`.
fn cell_variant(part: &str, group: &str) -> String {
    let name = group
        .strip_prefix(part)
        .map_or(group, |g| g.trim_start_matches('_'));
    if name.is_empty() {
        return "Form".to_string();
    }
    name.split('_').map(variant).collect()
}

/// A part of speech's typed API: its `Cell` (a typed combination of
/// feature values, one variant per group), and the word type that inflects.
fn api(
    part: &Part<'_, '_>,
    cells: &Cells<'_>,
    layout: &Layout,
    ty: &str,
    feature: Option<&str>,
) -> String {
    let name = part.spec.name;
    let word = variant(name);
    let mut s = String::new();
    // The cell type.
    let _ = writeln!(
        s,
        "/// A {name} form: which cell of its {}.\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum Cell {{",
        ty.to_lowercase()
    );
    for g in &cells.groups {
        let fields: Vec<&str> = g.features.iter().map(|f| f.name).collect();
        let v = cell_variant(name, g.name);
        let _ = writeln!(s, "    /// {}", g.group.doc);
        if fields.is_empty() {
            let _ = writeln!(s, "    {v},");
        } else {
            let _ = writeln!(s, "    {v}({}),", fields.join(", "));
        }
    }
    s.push_str("}\n\nimpl Cell {\n    /// Its place in a paradigm.\n    pub const fn index(self) -> usize {\n        let cell = match self {\n");
    for g in &cells.groups {
        let v = cell_variant(name, g.name);
        for i in 0..g.len() {
            let values = g.values(g.offset + i);
            let args: Vec<String> = g
                .features
                .iter()
                .zip(&values)
                .map(|(f, &x)| format!("{}::{}", f.name, f.values.get(x).map_or("", |v| v.name)))
                .collect();
            let constant = layout.names.get(g.offset + i).map_or("", String::as_str);
            if args.is_empty() {
                let _ = writeln!(s, "            Cell::{v} => cell::{constant},");
            } else {
                let _ = writeln!(
                    s,
                    "            Cell::{v}({}) => cell::{constant},",
                    args.join(", ")
                );
            }
        }
    }
    s.push_str("        };\n        cell as usize\n    }\n}\n\n");
    // Conversions from a group's values, where no other group has the same
    // features.
    for g in &cells.groups {
        let fields: Vec<&str> = g.features.iter().map(|f| f.name).collect();
        let shared = cells
            .groups
            .iter()
            .filter(|o| o.features.iter().map(|f| f.name).eq(fields.iter().copied()))
            .count();
        if fields.is_empty() || shared > 1 {
            continue;
        }
        let v = cell_variant(name, g.name);
        let vars: Vec<String> = g.features.iter().map(|f| snake(f.name)).collect();
        let (ty_, pat) = if fields.len() == 1 {
            (fields.join(""), vars.join(""))
        } else {
            (
                format!("({})", fields.join(", ")),
                format!("({})", vars.join(", ")),
            )
        };
        let _ = writeln!(
            s,
            "impl From<{ty_}> for Cell {{\n    fn from({pat}: {ty_}) -> Self {{\n        Cell::{v}({})\n    }}\n}}\n",
            vars.join(", ")
        );
    }
    // The word.
    let (feature_field, feature_type) = match feature {
        Some(f) => (snake(f), format!("crate::{f}")),
        None => (String::new(), "u8".to_string()),
    };
    let _ = writeln!(
        s,
        "/// A {name} to inflect: its lemma, and what you know of it. The more you\n/// tell, the surer its forms.\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct {word}<'w> {{\n    lemma: &'w str,\n    given: Vec<(usize, &'w str)>,"
    );
    if feature.is_some() {
        let _ = writeln!(s, "    {feature_field}: Option<{feature_type}>,");
    }
    let folds = !part.learned.grammar.orthography.folded.is_empty();
    if folds {
        s.push_str("    full: bool,\n");
    }
    s.push_str("}\n\n");
    let _ = writeln!(
        s,
        "impl<'w> {word}<'w> {{\n    /// The {name} `lemma`, as the dictionary writes it.\n    #[must_use]\n    pub fn new(lemma: &'w str) -> Self {{\n        {word} {{\n            lemma,\n            given: Vec::new(),"
    );
    if feature.is_some() {
        let _ = writeln!(s, "            {feature_field}: None,");
    }
    if folds {
        s.push_str("            full: false,\n");
    }
    s.push_str(
        "        }
    }

",
    );
    let _ = writeln!(
        s,
        "    /// One of its forms you know (a principal part): readings that disagree\n    /// with it are dropped.\n    #[must_use]\n    pub fn given(mut self, cell: impl Into<Cell>, form: &'w str) -> Self {{\n        self.given.push((cell.into().index(), form));\n        self\n    }}\n"
    );
    if let Some(f) = feature {
        let _ = writeln!(
            s,
            "    /// Its {lower}, which says which cell its `referred` forms (see its\n    /// {many}) are taken from.\n    #[must_use]\n    pub fn {feature_field}(mut self, {feature_field}: {f}) -> Self {{\n        self.{feature_field} = Some({feature_field});\n        self\n    }}\n",
            lower = f.to_lowercase(),
            many = ty.to_lowercase(),
        );
    }
    if folds {
        let _ = writeln!(
            s,
            "    /// Its lemma is spelled with every mark the dictionary writes, so\n    /// none is hidden (without this, a lemma typed without marks may be\n    /// missing some, and its forms are less sure).\n    #[must_use]\n    pub fn spelled_in_full(mut self) -> Self {{\n        self.full = true;\n        self\n    }}\n"
        );
    }
    let full = if folds { "self.full" } else { "false" };
    let feature_value = if feature.is_some() {
        format!("self.{feature_field}")
    } else {
        "None".to_string()
    };
    let _ = writeln!(
        s,
        "    fn known(&self) -> Known<'_, {feature_type}> {{\n        Known {{\n            forms: &self.given,\n            feature: {feature_value},\n            full: {full},\n        }}\n    }}\n"
    );
    let _ = writeln!(
        s,
        "    /// Its likeliest form in `cell`.\n    #[must_use]\n    pub fn form(&self, cell: impl Into<Cell>) -> String {{\n        GRAMMAR.best(self.lemma, cell.into().index(), &self.known())\n    }}\n\n    /// Every form it may have in `cell`, likeliest first, each with its\n    /// probability and why.\n    #[must_use]\n    pub fn candidates(&self, cell: impl Into<Cell>) -> Vec<Candidate<'static>> {{\n        GRAMMAR.candidates(self.lemma, cell.into().index(), &self.known())\n    }}\n\n    /// The {many} it may follow, likeliest first, each with its probability\n    /// and why.\n    #[must_use]\n    pub fn {one}(&self) -> Vec<Reading<'static, 'static, {ty}>> {{\n        GRAMMAR.readings(self.lemma, &self.known())\n    }}\n}}",
        many = format!("{}s", ty.to_lowercase()),
        one = ty.to_lowercase(),
    );
    s
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
        "impl std::fmt::Display for {} {{\n    /// Its name in lowercase: `genitive`.\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n        f.write_str(match self {{",
        f.name
    );
    for v in &f.values {
        let _ = writeln!(
            s,
            "            {}::{} => {:?},",
            f.name,
            v.name,
            v.name.to_lowercase()
        );
    }
    s.push_str("        })\n    }\n}\n\n");
    let _ = writeln!(
        s,
        "impl std::str::FromStr for {0} {{\n    type Err = flexion::UnknownValue;\n\n    /// By its name (`genitive`) or its short name (`gen`).\n    fn from_str(name: &str) -> Result<Self, Self::Err> {{\n        Self::ALL\n            .into_iter()\n            .find(|v| v.short() == name || v.to_string() == name)\n            .ok_or_else(|| flexion::UnknownValue {{\n                feature: {0:?},\n                name: name.to_string(),\n            }})\n    }}\n}}\n",
        f.name
    );
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
