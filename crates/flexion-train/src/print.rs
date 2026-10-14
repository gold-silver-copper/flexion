//! Writing a language's grammar as a Rust crate: its `Cargo.toml`, README and
//! data license, `src/lib.rs` (the feature enums and a typed function per
//! group of cells), `src/orthography.rs`, and a module per part of speech
//! with its paradigms, the rules choosing them, and its irregular forms.

use crate::cells::{Cells, GroupCells, combinations};
use crate::learn::Learned;
use crate::script::Script;
use crate::spec::{Feature, Language, PartOfSpeech, Value};
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
        ("DATA-LICENSE.md".to_string(), data_license(lang)),
        ("src/orthography.rs".to_string(), orthography(script)),
        ("src/lib.rs".to_string(), library(lang, parts)?),
    ];
    if let Some(extra) = lang.extra {
        out.push(("src/extra.rs".to_string(), extra.to_string()));
    }
    for part in parts {
        let cells = Cells::new(lang, part.spec)?;
        out.extend(grammar_files(lang, part.spec, part.learned, &cells));
    }
    let mut grammar: Vec<String> = out
        .iter()
        .filter_map(|(path, _)| Some(path.strip_prefix("src/")?.split_once('/')?.1.to_string()))
        .collect();
    grammar.sort();
    grammar.dedup();
    out.push(("README.md".to_string(), readme(lang, report, &grammar)));
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
        let short = |v: &Value| v.short.replace(['.', '-', ' '], "_");
        for (gi, g) in cells.groups.iter().enumerate() {
            let Some((last, rest)) = g.features.split_last() else {
                let label = ident(g.name);
                if let Some(slot) = names.get_mut(g.offset) {
                    *slot = const_name(&label);
                }
                rows.push(Row {
                    label,
                    columns: Vec::new(),
                    cells: vec![g.offset],
                });
                continue;
            };
            let columns: Vec<String> = last.values.iter().map(|v| ident(v.short)).collect();
            // The rows: every combination of the other features' values. A
            // group of one feature is one row named after the group; the
            // first group's rows are named by their values alone.
            for combo in combinations(rest.iter().map(|f| (0..f.values.len()).collect())) {
                let values = rest
                    .iter()
                    .zip(&combo)
                    .filter_map(|(f, &v)| f.values.get(v));
                let (mut parts, mut consts): (Vec<String>, Vec<String>) =
                    values.map(|v| (short(v).to_lowercase(), short(v))).unzip();
                if rest.is_empty() || gi > 0 {
                    parts.insert(0, ident(g.name));
                    consts.insert(0, g.name.to_string());
                }
                let row_cells: Vec<usize> = (0..last.values.len())
                    .map(|v| g.index(&[combo.as_slice(), &[v]].concat()))
                    .collect();
                for (&cell, v) in row_cells.iter().zip(&last.values) {
                    if let Some(slot) = names.get_mut(cell) {
                        *slot = const_name(&format!("{}_{}", consts.join("_"), short(v)));
                    }
                }
                rows.push(Row {
                    label: ident(&parts.join("_")),
                    columns: columns.clone(),
                    cells: row_cells,
                });
            }
        }
        Layout { rows, names }
    }

    /// A cell's constant.
    fn name(&self, cell: impl Into<usize>) -> &str {
        self.names.get(cell.into()).map_or("", String::as_str)
    }

    /// A table's rows, each cell written by `cell` (`_`: written elsewhere),
    /// and its closing. A row of `_` only isn't written.
    fn table(&self, cell: impl Fn(usize) -> String) -> String {
        let mut s = String::new();
        for row in &self.rows {
            let cells: Vec<String> = row.cells.iter().map(|&c| cell(c)).collect();
            if cells.iter().all(|c| c == "_") {
                continue;
            }
            let _ = writeln!(s, "    {}: {};", row.label, cells.join(", "));
        }
        s + "    ),\n"
    }

    /// A table macro's rows of arguments (`$c0:tt, $c1:tt`), a line each,
    /// each row optional; and its table: per cell, its argument (as
    /// `flexion::cell!` reads it), a tab after each, nothing for a row left
    /// out.
    fn macro_rows(&self) -> (String, String) {
        let mut rows = String::new();
        let mut by_cell = vec![0; self.names.len()];
        let mut k = 0;
        for row in &self.rows {
            let args: Vec<String> = row
                .cells
                .iter()
                .map(|&c| {
                    if let Some(slot) = by_cell.get_mut(c) {
                        *slot = k;
                    }
                    k += 1;
                    format!("$c{}:tt", k - 1)
                })
                .collect();
            let _ = writeln!(rows, "     $({}: {};)?", row.label, args.join(", "));
        }
        let table = by_cell
            .iter()
            .map(|arg| format!("                $(flexion::cell!($c{arg}),)? \"\\t\",\n"))
            .collect();
        (rows, table)
    }
}

/// Each cell's name: its constant in lowercase, after its group's name
/// (`present_ind_act_1_sg`, `noun_gen_pl`).
pub fn cell_names(cells: &Cells<'_>) -> Vec<String> {
    let mut names: Vec<String> = Layout::new(cells)
        .names
        .into_iter()
        .map(|n| n.to_lowercase())
        .collect();
    // The first group's cells are named without it: named with it here.
    for g in &cells.groups {
        let group = underscored(g.name).to_lowercase();
        for name in names.iter_mut().skip(g.offset).take(g.len()) {
            if *name != group && !name.starts_with(&format!("{group}_")) {
                *name = format!("{group}_{name}");
            }
        }
    }
    names
}

/// A cell's constant: `gen_pl` → `GEN_PL`, `non_past_1_sg` → `NON_PAST_1_SG`.
fn const_name(s: &str) -> String {
    let mut out = underscored(s).trim_start_matches('_').to_ascii_uppercase();
    if out.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        out.insert(0, 'C');
    }
    out
}

/// `pres.act` → `pres_act`; a leading digit or a keyword gets an underscore.
fn ident(s: &str) -> String {
    let mut out = underscored(s);
    if out.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    if KEYWORDS.contains(&out.as_str()) {
        out.push('_');
    }
    out
}

/// `s` with every character but letters and digits an underscore.
fn underscored(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
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
        // A name must start with a letter, not a mark (`ेष`).
        Some(first)
            if first.is_alphabetic() && !unicode_normalization::char::is_combining_mark(first) =>
        {
            first.to_uppercase().chain(chars).collect()
        }
        Some(first) => format!("P{first}{}", chars.as_str()),
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

/// One part of speech's files under `src/<part>/`, and its typed API on
/// `cells`.
fn grammar_files(
    lang: &Language,
    spec: &PartOfSpeech,
    learned: &Learned<'_>,
    cells: &Cells<'_>,
) -> Vec<(String, String)> {
    let layout = &Layout::new(cells);
    let dir = format!("src/{}", spec.name);
    let doc = format!(
        "{} {}",
        lang.display,
        spec.doc.trim_end_matches('.').to_lowercase()
    );
    let g = &learned.grammar;
    let (ty, one, many) = kind(spec);
    let variants = unique(g.paradigms.iter().map(|p| variant(&p.name)));
    let name_of = |p: u16| variants.get(usize::from(p)).cloned().unwrap_or_default();
    // The feature relations depend on, as the crate's enum names it.
    let feature_name: Option<&str> = learned.feature.and_then(|f| spec.inherent.get(f).copied());
    // Its inherent features (gender), with their values.
    let inherent: Vec<&Feature> = spec
        .inherent
        .iter()
        .filter_map(|n| lang.feature(n))
        .collect();
    let mut files = Vec::new();

    // mod.rs
    let words: u32 = g.paradigms.iter().map(|p| p.words).sum();
    let mut m = format!(
        "//! {doc}: {} {many} learned from {} dictionary words, the rules on a\n//! word's ending that choose one, and the forms no {one} gives.\n//!\n//! Generated by flexion-train from Wiktionary; don't edit by hand.\n\n",
        g.paradigms.len(),
        thousands(words),
    );
    if let Some(f) = feature_name {
        let _ = writeln!(m, "mod {};", snake(f));
    }
    let modules = [
        (g.accent.is_some(), "pub mod accent"),
        (!g.relations.is_empty(), "mod relations"),
        (g.compound.is_some(), "mod compounds"),
        (g.prefixes.is_some(), "mod prefixes"),
        (g.marks.is_some(), "mod marks"),
        (true, "mod irregular"),
        (!g.slots.is_empty(), "mod stems"),
        (g.laws.is_some(), "mod laws"),
        (true, "mod paradigms;\nmod rules;\nmod spellings"),
    ];
    for (_, module) in modules.iter().filter(|(on, _)| *on) {
        let _ = writeln!(m, "{module};");
    }
    let _ = writeln!(m, "\npub use paradigms::{ty};\n");
    m.push_str("use flexion::{Candidate, Grammar, Known, Reading};\nuse std::borrow::Cow;\n\n");
    m.push_str("/// Each cell's place in a paradigm.\n#[allow(dead_code)]\npub mod cell {\n");
    for (i, name) in layout.names.iter().enumerate() {
        let _ = writeln!(m, "    pub const {name}: u16 = {i};");
    }
    m.push_str("}\n\n");
    let some = |on: bool, value: &str| {
        if on {
            format!("Some({value})")
        } else {
            "None".into()
        }
    };
    let _ = writeln!(
        m,
        "/// The grammar: paradigms, the rules choosing one, irregular forms.\npub static GRAMMAR: Grammar<'static, {ty}{generics}> = Grammar {{\n    orthography: crate::orthography::ORTHOGRAPHY,\n    cells: {},\n    lemma_cell: {:?},\n    capitalized: {},\n    paradigms: Cow::Borrowed(&paradigms::{}),\n    paradigm: rules::{},\n    relations: Cow::Borrowed(&{relations}),\n    feature: {feature},\n    compound: {compound},\n    prefixes: {prefixes},\n    accent: {accent},\n    laws: {laws},\n    spellings: Cow::Borrowed(spellings::SPELLINGS),\n    marks: {marks},\n    slots: {slots},\n    tables: {tables},\n    irregular: irregular::IRREGULAR,\n    smoothing: {:?},\n    sharpness: {:?},\n    trust: flexion::Trust {{ regular: {:?}, unsettled: {:?}, unattested: {:?} }},\n}};\n",
        g.cells,
        g.lemma_cell,
        g.capitalized,
        many.to_uppercase(),
        one.to_uppercase(),
        g.smoothing,
        g.sharpness,
        g.trust.regular,
        g.trust.unsettled,
        g.trust.unattested,
        slots = if g.slots.is_empty() {
            "Cow::Borrowed(&[])"
        } else {
            "Cow::Borrowed(&stems::SLOTS)"
        },
        tables = if g.slots.is_empty() {
            "Cow::Borrowed(&[])"
        } else {
            "Cow::Borrowed(&stems::TABLES)"
        },
        generics = match (g.accent.is_some(), feature_name) {
            (false, None) => String::new(),
            (true, None) => ", accent::AccentClass".to_string(),
            (false, Some(f)) => format!(", u8, crate::{f}"),
            (true, Some(f)) => format!(", accent::AccentClass, crate::{f}"),
        },
        relations = if g.relations.is_empty() {
            "[]"
        } else {
            "relations::RELATIONS"
        },
        feature = feature_name
            .zip(learned.feature)
            .map_or("None".to_string(), |(f, i)| format!(
                "Some(({i}, {}::{}))",
                snake(f),
                f.to_uppercase()
            )),
        compound = some(g.compound.is_some(), "compounds::COMPOUND"),
        prefixes = some(g.prefixes.is_some(), "prefixes::PREFIXES"),
        accent = some(
            g.accent.is_some(),
            "flexion::Accent {\n        patterns: Cow::Borrowed(&accent::PATTERNS),\n        classes: Cow::Borrowed(&accent::CLASSES),\n        class: accent::CLASS,\n        circumflex: Cow::Borrowed(&accent::CIRCUMFLEX),\n        long_finals: Cow::Borrowed(&accent::LONG_FINALS),\n    }"
        ),
        laws = some(g.laws.is_some(), "laws::LAWS"),
        marks = some(g.marks.is_some(), "marks::MARKS"),
    );
    m.push_str(&api_code(
        spec,
        learned,
        layout,
        cells,
        feature_name,
        &inherent,
    ));
    files.push((format!("{dir}/mod.rs"), m));

    // paradigms.rs
    // The class letters a harmony law writes (`A` of `~lAr`).
    let classes: Vec<String> = learned
        .laws
        .iter()
        .filter_map(|l| match &l.change {
            crate::laws::Change::Harmony { map, .. } => Some(map),
            _ => None,
        })
        .flat_map(|map| {
            let mut letters: Vec<char> = map.iter().map(|e| e.0).collect();
            letters.dedup();
            letters
        })
        .map(|c| format!("`{c}`"))
        .collect();
    let harmony = match classes.as_slice() {
        [] => String::new(),
        _ => format!(
            " A capital ({})\n//! is a class letter: a harmony law in `laws.rs` writes it, by the\n//! letter before it.",
            classes.join(", ")
        ),
    };
    let alternating = g
        .paradigms
        .iter()
        .any(|p| p.lemma.ends_with(flexion::pattern::ALTERNATES));
    let alternating = if alternating {
        format!(
            " A `{}` ends\n//! the patterns of a {one} whose stems alternate: the laws in `laws.rs`\n//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).",
            flexion::pattern::ALTERNATES
        )
    } else {
        String::new()
    };
    let mut p = format!(
        "//! The {many}, commonest first, each named after a word that follows it.\n//!\n//! A pattern is the form with `~` where the stem goes: `~а` is the stem and\n//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other\n//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`\n//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),\n//! as the copy laws in `laws.rs` change it.{harmony}{alternating}\n\nuse flexion::Paradigm;\n\n"
    );
    // The table macro (a table with no rows needs none).
    if !g.paradigms.is_empty() {
        let _ = writeln!(
            p,
            "/// Writes one {one}: the word it is named after, how many words follow it,\n/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if\n/// they share them), the {one} it is like (if any), the stem slots\n/// it states with a stem and an ending table (`stem aorist: \"~σ- like λυω\"`:\n/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see\n/// `stems.rs`), then a row of patterns per line (`derived`: the form comes\n/// from another cell's, see `relations.rs`; `_`, or a row left out: the\n/// form is its stem slot's, or the {one}'s it is like, or, in a {one} like\n/// no other, derived):"
        );
        for row in &layout.rows {
            let cols = if row.columns.is_empty() {
                String::new()
            } else {
                format!(" ({})", row.columns.join(", "))
            };
            let _ = writeln!(p, "/// `{}`{cols}", row.label);
        }
        let (features, values) = if inherent.is_empty() {
            ("", "")
        } else {
            (
                " $(, [$($f:ident: $v:ident),*])?",
                "$($(feature!($f: $v)),*)?",
            )
        };
        let (rows, table) = layout.macro_rows();
        // A row per stem slot, before the cells' rows.
        let stem_rows: String = g
            .slots
            .iter()
            .enumerate()
            .map(|(i, slot)| format!("     $(stem {}: $s{i}:tt;)?\n", slot.name))
            .collect();
        let stems: String = (0..g.slots.len())
            .map(|i| format!("                $(flexion::cell!($s{i}),)? \"\\t\",\n"))
            .collect();
        let _ = writeln!(
            p,
            "macro_rules! {one} {{\n    ($name:literal, $words:literal, $lemma:literal{features} $(, like $parent:ident $(as $view:ident)?)?;\n{stem_rows}{rows}    ) => {{\n        Paradigm {{\n            name: ::std::borrow::Cow::Borrowed($name),\n            words: $words,\n            lemma: ::std::borrow::Cow::Borrowed($lemma),\n            parent: like!($($parent)?),\n            reads: view!($($($view)?)?),\n            inherent: ::std::borrow::Cow::Borrowed(&[{values}]),\n            table: ::std::borrow::Cow::Borrowed(concat!(\n{table}            )),\n            stems: ::std::borrow::Cow::Borrowed(concat!(\n{stems}            )),\n        }}\n    }};\n}}\n\n/// The {one} a {one} is like, by its name.\nmacro_rules! like {{\n    () => {{\n        None\n    }};\n    ($parent:ident) => {{\n        Some({ty}::$parent as u16)\n    }};\n}}\n\n/// The cells a {one} reads in the one it is like, by the view's name (see\n/// the views below); none: the same cells.\nmacro_rules! view {{\n    () => {{\n        None\n    }};\n    ($view:ident) => {{\n        Some(::std::borrow::Cow::Borrowed(&$view))\n    }};\n}}\n"
        );
        if g.paradigms.iter().any(|p| !p.inherent.is_empty()) {
            let _ = writeln!(
                p,
                "/// An inherent feature's value, as `(feature, value)` by their places.\nmacro_rules! feature {{"
            );
            for (i, f) in inherent.iter().enumerate() {
                let _ = writeln!(
                    p,
                    "    ({}: $v:ident) => {{\n        ({i}, crate::{}::$v as u8)\n    }};",
                    snake(f.name),
                    f.name
                );
            }
            p.push_str("}\n\n");
        }
    }
    let enumerated: String = variants
        .iter()
        .zip(g.paradigms.iter())
        .map(|(v, par)| {
            format!(
                "    /// Like `{}`: {} words.\n    {v},\n",
                par.name,
                thousands(par.words)
            )
        })
        .collect();
    let all: String = variants
        .iter()
        .map(|v| format!("        {ty}::{v},\n"))
        .collect();
    // The views paradigms read their parents through, each once.
    let mut used: Vec<&crate::learn::View> = Vec::new();
    for par in g.paradigms.iter() {
        if let Some(v) = par
            .reads
            .as_ref()
            .and_then(|r| learned.views.iter().find(|v| v.1.as_slice() == r.as_ref()))
            && !used.iter().any(|u| u.0 == v.0)
        {
            used.push(v);
        }
    }
    for (name, map) in used {
        let cells: Vec<String> = map.iter().map(u16::to_string).collect();
        let _ = writeln!(
            p,
            "/// A view: per cell (in the cells' order), the cell of the {one} it is\n/// like that it reads (`{name}`: one value read as another).\nconst {}: [u16; {}] = [{}];\n",
            const_name(name),
            map.len(),
            cells.join(", ")
        );
    }
    let _ = write!(
        p,
        "/// Every {one} learned.\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum {ty} {{\n{enumerated}}}\n\nimpl {ty} {{\n    /// Every {one}, in order.\n    pub const ALL: [{ty}; {n}] = [\n{all}    ];\n\n    /// Its table.\n    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {{\n        {many_}.get(self as usize)\n    }}\n}}\n\n{}pub(super) static {many_}: [Paradigm<'static>; {n}] = [\n",
        label_impl(ty),
        n = variants.len(),
        many_ = many.to_uppercase(),
    );
    for (i, par) in g.paradigms.iter().enumerate() {
        let examples: Vec<&str> = learned
            .members
            .get(i)
            .map(|m| m.iter().map(String::as_str).take(6).collect())
            .unwrap_or_default();
        let parent = par.parent.and_then(|q| variants.get(usize::from(q)));
        // A child: like its parent, except in the cells it writes.
        let like = parent.map_or(String::new(), |q| {
            let own: Vec<String> = (0..g.cells)
                .filter(|&c| par.own(c).is_some())
                .map(|c| {
                    layout
                        .rows
                        .iter()
                        .find_map(|row| {
                            let at = row.cells.iter().position(|&x| x == c)?;
                            Some(match row.columns.get(at) {
                                Some(column) => format!("{} {column}", row.label),
                                None => row.label.clone(),
                            })
                        })
                        .unwrap_or_default()
                })
                .collect();
            let more = if own.len() > 6 { ", …" } else { "" };
            let named: Vec<&str> = own.iter().take(6).map(String::as_str).collect();
            let cells = if own.len() == 1 { "cell" } else { "cells" };
            format!(
                " Like {q}, except in {} {cells}: {}{more}.",
                own.len(),
                named.join(", ")
            )
        });
        let _ = writeln!(
            p,
            "    // {}: {} words ({}).{like}",
            variants.get(i).map_or("", String::as_str),
            thousands(par.words),
            examples.join(", "),
        );
        let values: Vec<String> = par
            .inherent
            .iter()
            .filter_map(|&(f, v)| {
                let f = inherent.get(usize::from(f))?;
                Some(format!(
                    "{}: {}",
                    snake(f.name),
                    f.values.get(usize::from(v))?.name
                ))
            })
            .collect();
        let _ = writeln!(
            p,
            "    {one}!({}, {}, {}{}{};",
            Lit(&par.name),
            par.words,
            Lit(&par.lemma),
            if values.is_empty() {
                String::new()
            } else {
                format!(", [{}]", values.join(", "))
            },
            parent.map_or(String::new(), |q| {
                let view = par
                    .reads
                    .as_ref()
                    .and_then(|r| learned.views.iter().find(|v| v.1.as_slice() == r.as_ref()))
                    .map_or(String::new(), |v| format!(" as {}", const_name(&v.0)));
                format!(", like {q}{view}")
            })
        );
        // The slots it states with a stem and an ending table.
        for (slot, entry) in g.slots.iter().zip(par.stems.split('\t')) {
            if !entry.is_empty() {
                let _ = writeln!(p, "    stem {}: {};", slot.name, Lit(entry));
            }
        }
        p.push_str(&layout.table(|c| match par.own(c) {
            Some(flexion::Form::Pattern(p)) => Lit(p).to_string(),
            Some(flexion::Form::Derived) => "derived".to_string(),
            None => "_".to_string(),
        }));
    }
    p.push_str("];\n");
    files.push((format!("{dir}/paradigms.rs"), p));

    // rules.rs
    let uses = if g.paradigm.rules.is_empty() && g.paradigm.exceptions.is_empty() {
        ""
    } else {
        "::{self, *}"
    };
    let r = format!(
        "//! Which {one} a word takes. The rule with the longest ending the word has\n//! decides (`#` is the word's start: `#lux` is the word `lux`), giving each\n//! {one} with how many of the dictionary's words with that ending follow it; the words no rule gets right are listed. A rule is\n//! kept only when it pays for itself: it must describe the words it covers\n//! in fewer bits than the broader rule does, by more than the rule costs.\n\nuse super::paradigms::{ty}{uses};\nuse flexion::Choice;\n\n{}",
        choice_const(&one.to_uppercase(), ty, "words", &g.paradigm, name_of)
    );
    files.push((format!("{dir}/rules.rs"), r));

    // The feature relations depend on.
    if let (Some(f), Some((_, choice))) = (feature_name, &g.feature) {
        let values = lang
            .feature(f)
            .map(|x| x.values.as_slice())
            .unwrap_or_default();
        let value = |v: u8| {
            values
                .get(usize::from(v))
                .map_or("", |x| x.name)
                .to_string()
        };
        let uses = if choice.rules.is_empty() && choice.exceptions.is_empty() {
            ""
        } else {
            "::{self, *}"
        };
        let r = format!(
            "//! A word's {lower}, which decides which cell some cells' forms come from\n//! (see `relations.rs`). The rule with the longest ending the word has\n//! decides; the words no rule gets right are listed.\n\nuse crate::{f}{uses};\nuse flexion::Choice;\n\n{}",
            choice_const(&f.to_uppercase(), f, "words", choice, value),
            lower = f.to_lowercase()
        );
        files.push((format!("{dir}/{}.rs", snake(f)), r));
    }
    // relations.rs
    if !g.relations.is_empty() {
        let mut r = format!(
            "//! Where a cell's form comes from: another cell's form of the word, changed.\n//! A {one} that writes `derived` in a cell takes the form this way, and so\n//! does a word whose form of the other cell is listed but not this one's.\n//! Each relation lists its changes, as patterns around the letters they keep\n//! (`(\"~ī\", \"~istī\")`, numbered in order), and the rules on that form's\n//! ending choosing one.\n\nuse super::cell::*;\nuse flexion::Relation;\nuse std::borrow::Cow;\n\n/// Writes one relation: its cell, the cells it comes from (by the word's\n/// feature, in order, if several), its changes, then the rules choosing one\n/// and the words they get wrong (`none`: the form itself), whether it tells\n/// listed words' forms, and the pair of cells it is by analogy with.\nmacro_rules! relation {{\n    ($cell:ident, [$($from:ident),*], [$(($old:literal, $new:literal)),*], none, $listed:literal, $analogy:expr) => {{\n        Relation {{\n            cell: $cell,\n            from: Cow::Borrowed(&[$($from),*]),\n            changes: flexion::changes![$(($old, $new)),*],\n            change: None,\n            listed: $listed,\n            analogy: $analogy,\n        }}\n    }};\n    ($cell:ident, [$($from:ident),*], [$(($old:literal, $new:literal)),*], rules {{ $($rules:tt)* }} listed {{ $($words:tt)* }}, $listed:literal, $analogy:expr) => {{\n        Relation {{\n            cell: $cell,\n            from: Cow::Borrowed(&[$($from),*]),\n            changes: flexion::changes![$(($old, $new)),*],\n            change: Some(flexion::choice! {{ rules {{ $($rules)* }} listed {{ $($words)* }} }}),\n            listed: $listed,\n            analogy: $analogy,\n        }}\n    }};\n}}\n\npub(super) static RELATIONS: [Relation<'static>; {}] = [\n",
            g.relations.len()
        );
        for rel in g.relations.iter() {
            let from: Vec<&str> = rel.from.iter().map(|&c| layout.name(c)).collect();
            let feature = match (rel.from.len(), feature_name) {
                (n, Some(f)) if n > 1 => format!(" (by {}, in order)", f.to_lowercase()),
                _ => String::new(),
            };
            let changes: Vec<String> = rel
                .changes
                .iter()
                .map(|(old, new)| format!("({}, {})", Lit(old), Lit(new)))
                .collect();
            let change = rel.change.as_ref().map_or("none".to_string(), |choice| {
                let mut listed = listed(choice, |l| l.to_string());
                if !listed.is_empty() {
                    listed = format!("\n{listed}    ");
                }
                format!(
                    "rules {{\n{}    }} listed {{{listed}}}",
                    rules(choice, |l| l.to_string())
                )
            });
            let (analogy, why) = rel
                .analogy
                .map_or(("None".into(), String::new()), |(a, b)| {
                    let (a, b) = (layout.name(a), layout.name(b));
                    let how = if a == b {
                        "as it is".to_string()
                    } else {
                        format!("changed as {a} is into {b}")
                    };
                    (
                        format!("Some(({a}, {b}))"),
                        format!(" Few or no words have it: {how}."),
                    )
                });
            let _ = writeln!(
                r,
                "    // {cell}, from {}{feature}.{why}\n    relation!({cell}, [{}], [{}], {change}, {}, {analogy}),",
                from.join(" or "),
                from.join(", "),
                changes.join(", "),
                rel.listed,
                cell = layout.name(rel.cell),
            );
        }
        r.push_str("];\n");
        files.push((format!("{dir}/relations.rs"), r));
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
        let r = format!(
            "//! How a word written in parts inflects: only its last part (`Last`), or each\n//! part as the word it is (`Each`). The first part decides, by its ending\n//! (the longest rule it has), or as listed.\n\nuse flexion::Choice;\nuse flexion::Compound::{{self, *}};\n\n{}",
            choice_const("COMPOUND", "Compound", "first parts", c, |l| format!(
                "{l:?}"
            ))
        );
        files.push((format!("{dir}/compounds.rs"), r));
    }

    // marks.rs
    if let Some(marks) = &g.marks {
        let r = format!(
            "//! How a letter typed without its mark is written. Each rule reads the letter\n//! before it, the letter and the letters after it (`#` is the word's start or\n//! end): the longest one that matches decides (`tatio#` → `ā`, as in `-ātiō`;\n//! `#de` → `ē`, as in `dē-`; `ae` → `e`, a diphthong), giving each way of writing\n//! the letter with how many of the dictionary's letters in that place are\n//! written so.\n\nuse flexion::Choice;\n\n/// {} rules.\npub(super) const MARKS: Choice<'static, char> = {}}};\n",
            thousands(marks.rules.len()),
            choice_macro("starts", marks, |c| format!("{c:?}"))
        );
        files.push((format!("{dir}/marks.rs"), r));
    }

    // spellings.rs
    let mut sp = format!(
        "//! Words whose spelling their key doesn't show, as the dictionary writes\n//! them: a caller who types a word without its marks means this. Marks a\n//! {one} writes itself aren't listed.\n\nuse std::borrow::Cow;\n\n/// {} words.\npub(super) static SPELLINGS: &[(Cow<'static, str>, Cow<'static, str>)] = flexion::spellings! {{\n",
        thousands(g.spellings.len())
    );
    for (k, w) in g.spellings.iter() {
        let _ = writeln!(sp, "    {} => {},", Lit(k), Lit(w));
    }
    sp.push_str("};\n");
    files.push((format!("{dir}/spellings.rs"), sp));

    // laws.rs
    if let Some(program) = &g.laws {
        let mut l = format!(
            "//! Sound laws where a stem meets its ending (`+`), where a form's first\n//! literal meets the stem (`‹`: the augment `ἐ‹ἀ → ἠ`), on a letter a\n//! `%` copies (`°`), on a class letter an ending writes (harmony, by the\n//! nearest letter of a tier before it as written), or inside a stem that\n//! alternates (`ˇ`, by what follows it): every form a {one} builds\n//! goes through them, in order, so one {one} serves words whose stems change\n//! its endings. Each with the words whose forms it gives, and how many of\n//! the dictionary's forms it gets wrong (those are listed in `irregular.rs`).\n\nuse flexion::sound_law::compiled::*;\n\nconst RULES: &[CompiledRule<'static>] = &[\n"
        );
        for (law, rule) in learned.laws.iter().zip(program.rules()) {
            let _ = writeln!(
                l,
                "    // {law}: {} words ({}), {} forms wrong.\n    {},",
                thousands(law.words),
                law.examples.join(", "),
                thousands(law.breaks),
                rule.to_rust()
            );
        }
        l.push_str("];\n\n/// The laws, in order.\npub(super) const LAWS: Program<'static> = Program::from_rules(RULES);\n");
        files.push((format!("{dir}/laws.rs"), l));
    }

    // irregular.rs
    let table = g.irregular.table.as_ref();
    let hashes = "#".repeat(
        (1..)
            .find(|&n| !table.contains(&format!("\"{}", "#".repeat(n))))
            .unwrap_or(1),
    );
    let ir = format!(
        "//! What no {one} gives a word, as the dictionary writes it: a line per word,\n//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):\n//! the slot's name and the stem with the ending table its slot's cells are\n//! built with (`aorist`, `ἀγγειλ- like λυω`), then each listed form, by its\n//! cell's number (see `super::cell`), all tab-separated. They are the answer\n//! for that word.\n\nuse flexion::Irregular;\nuse std::borrow::Cow;\n\n/// {} stems, {} forms.\npub(super) const IRREGULAR: Irregular<'static> = Irregular {{\n    table: Cow::Borrowed(\n        r{hashes}\"{table}\"{hashes},\n    ),\n}};\n",
        thousands(g.irregular.stems()),
        thousands(g.irregular.len())
    );
    files.push((format!("{dir}/irregular.rs"), ir));
    if !g.slots.is_empty() {
        files.push((format!("{dir}/stems.rs"), stems_file(g, layout, one)));
    }
    files
}

/// A part of speech's `stems.rs`: its stem slots, each with its cells, and
/// the ending tables listed stems are built with, each a line per row of
/// cells.
fn stems_file<P: flexion::Label, A: flexion::Label, F: flexion::Label>(
    g: &flexion::Grammar<'_, P, A, F>,
    layout: &Layout,
    one: &str,
) -> String {
    let mut s = format!(
        "//! The stem slots: the cells a word builds on one stem, learned from the\n//! words that put different stems in different cells; and the ending tables a\n//! word that lists its stem for a slot (see `irregular.rs`) builds the slot's\n//! cells with: per cell of the slot, in the slot's order, a tab after each, a\n//! {one}'s pattern with the slot's stem as its one piece (`~`), the stem being\n//! what every cell's form keeps of the word and what all its endings begin\n//! with. Each table is named after the commonest word built with it.\n\nuse super::cell::*;\nuse flexion::{{EndingTable, Slot}};\nuse std::borrow::Cow;\n\n"
    );
    let _ = writeln!(
        s,
        "/// The slots, in the order of their first cells.\npub(super) static SLOTS: [Slot<'static>; {}] = [",
        g.slots.len()
    );
    for slot in g.slots.iter() {
        // A line per row of cells.
        let mut lines: Vec<String> = Vec::new();
        let mut row: Option<usize> = None;
        for &c in slot.cells.iter() {
            let here = layout
                .rows
                .iter()
                .position(|r| r.cells.contains(&usize::from(c)));
            let name = layout.name(c);
            match lines.last_mut() {
                Some(line) if here == row => line.push_str(&format!(" {name},")),
                _ => lines.push(format!("{name},")),
            }
            row = here;
        }
        let _ = writeln!(
            s,
            "    Slot {{\n        name: Cow::Borrowed({:?}),\n        cells: Cow::Borrowed(&[\n            {}\n        ]),\n    }},",
            slot.name,
            lines.join("\n            ")
        );
    }
    s.push_str("];\n\n");
    let _ = writeln!(
        s,
        "/// The ending tables, by slot, most words first.\npub(super) static TABLES: [EndingTable<'static>; {}] = [",
        g.tables.len()
    );
    for t in g.tables.iter() {
        let Some(slot) = g.slots.get(usize::from(t.slot)) else {
            continue;
        };
        let _ = writeln!(
            s,
            "    // {} like {}: {} words.\n    EndingTable {{\n        name: Cow::Borrowed({:?}),\n        words: {},\n        slot: {},",
            slot.name,
            t.name,
            thousands(t.words),
            t.name,
            t.words,
            t.slot
        );
        // A line per row of cells, in the slot's order.
        let patterns: Vec<&str> = t.table.split('\t').collect();
        let mut row: Option<usize> = None;
        let mut lines: Vec<String> = Vec::new();
        for (k, &c) in slot.cells.iter().enumerate() {
            let here = layout
                .rows
                .iter()
                .position(|r| r.cells.contains(&usize::from(c)));
            if here != row || lines.is_empty() {
                lines.push(String::new());
            }
            row = here;
            if let Some(line) = lines.last_mut() {
                line.push_str(patterns.get(k).copied().unwrap_or(""));
                line.push('\t');
            }
        }
        match lines.as_slice() {
            [line] => {
                let _ = writeln!(s, "        table: Cow::Borrowed({line:?}),");
            }
            _ => {
                s.push_str("        table: Cow::Borrowed(concat!(\n");
                for line in &lines {
                    let _ = writeln!(s, "            {line:?},");
                }
                s.push_str("        )),\n");
            }
        }
        s.push_str("    },\n");
    }
    s.push_str("];\n");
    s
}

fn thousands(n: impl std::fmt::Display) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
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
    // Each part of speech's word type, unless a feature has its name
    // (French `Participle`).
    for part in parts {
        let word = variant(part.spec.name);
        if !lang.features.iter().any(|f| f.name == word) {
            let _ = writeln!(s, "pub use {}::{word};", part.spec.name);
        }
    }
    s.push('\n');
    // The features the parts of speech use.
    let used = |f: &&Feature| {
        parts.iter().any(|p| {
            let mut groups = p.spec.groups.iter();
            p.spec.inherent.contains(&f.name) || groups.any(|g| g.features.contains(&f.name))
        })
    };
    for f in lang.features.iter().filter(used) {
        feature_enum(&mut s, f);
    }
    for part in parts {
        let cells = Cells::new(lang, part.spec)?;
        let name = part.spec.name;
        let word = variant(name);
        for g in &cells.groups {
            let params: String = g
                .features
                .iter()
                .map(|f| format!(", {}: {}", snake(f.name), f.name))
                .collect();
            let values: Vec<String> = g.features.iter().map(|f| snake(f.name)).collect();
            let _ = writeln!(
                s,
                "/// {}\n#[must_use]\npub fn {}(lemma: &str{params}) -> String {{\n    {name}::{word}::new(lemma).form({name}::Cell::{})\n}}\n",
                g.group.doc,
                g.name,
                call(&cell_variant(name, g.name), &values),
                name = part.spec.name,
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
         //! as the law of limitation allows; `Last`, `NextToLast`: on that syllable of\n\
         //! the word. The law then moves it where it must, and\n\
         //! chooses acute or circumflex by the syllables' length.\n\n\
         use flexion::{AccentPattern, Choice, StressClass};\n\
         use std::borrow::Cow;\n",
    );
    if a.circumflex.iter().any(|c| !c.is_empty()) || !a.long_finals.is_empty() {
        s.push_str("use super::cell::*;\n");
    }
    s.push('\n');
    // The pattern table macro (no patterns need none).
    if !a.patterns.is_empty() {
        let (rows, table) = layout.macro_rows();
        let _ = write!(
            s,
            "/// Writes one accent pattern: the word it is named after, how many words\n/// follow it, the pattern it is like (by its place, if any), then a row of\n/// places per line, as the paradigm tables are laid out (`_`, or a row left\n/// out: the place of the pattern it is like).\nmacro_rules! pattern {{\n    ($name:literal, $words:literal $(, like $parent:literal)?;\n{rows}    ) => {{\n        AccentPattern {{\n            name: Cow::Borrowed($name),\n            words: $words,\n            parent: like!($($parent)?),\n            places: Cow::Borrowed(concat!(\n{table}            )),\n        }}\n    }};\n}}\n\n/// The pattern an accent pattern is like, by its place.\nmacro_rules! like {{\n    () => {{\n        None\n    }};\n    ($parent:literal) => {{\n        Some($parent)\n    }};\n}}\n\n"
        );
    }
    let _ = writeln!(
        s,
        "/// The accent patterns, commonest first.\npub static PATTERNS: [AccentPattern<'static>; {}] = [",
        a.patterns.len()
    );
    for (i, p) in a.patterns.iter().enumerate() {
        let parent = p
            .parent
            .and_then(|q| a.patterns.get(usize::from(q)).map(|x| (q, x)));
        let like = parent.map_or(String::new(), |(q, x)| {
            let differ = (0..layout.names.len())
                .filter(|&c| p.own(c).is_some())
                .count();
            format!(
                " Like {q} (`{}`), except in {differ} {}.",
                x.name,
                if differ == 1 { "cell" } else { "cells" }
            )
        });
        let _ = writeln!(
            s,
            "    // {i}, `{}`: {} words.{like}\n    pattern!({}, {}{};",
            p.name,
            thousands(p.words),
            Lit(&p.name),
            p.words,
            parent.map_or(String::new(), |(q, _)| format!(", like {q}"))
        );
        s.push_str(&layout.table(|c| p.own(c).map_or("_".to_string(), |pl| format!("{pl:?}"))));
    }
    // Classes.
    let names = unique(a.classes.iter().map(|c| variant(&c.name)));
    let all: Vec<String> = names.iter().map(|v| format!("AccentClass::{v}")).collect();
    s.push_str(
        "];\n\n/// Every accent class: a pattern and the stem's accented syllable.\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum AccentClass {\n",
    );
    let pattern = |c: &flexion::StressClass<'_>| {
        a.patterns
            .get(usize::from(c.pattern))
            .map_or("", |p| p.name.as_ref())
    };
    for (v, c) in names.iter().zip(a.classes.iter()) {
        let syllable = c
            .vowel
            .map_or("never on the stem".to_string(), |v| match v {
                0 => "the stem accented on the lemma's last syllable".to_string(),
                1 => "the stem accented on its next-to-last".to_string(),
                n => format!("the stem accented on its syllable {} from the end", n + 1),
            });
        let _ = writeln!(
            s,
            "    /// Pattern `{}`, {syllable}: {} words.\n    {v},",
            pattern(c),
            thousands(c.words)
        );
    }
    let _ = writeln!(
        s,
        "}}\n\nimpl AccentClass {{\n    /// Every class, in order.\n    pub const ALL: [AccentClass; {}] = [{}];\n}}\n\n{}/// The classes' patterns and stem syllables.\npub static CLASSES: [StressClass<'static>; {}] = [",
        names.len(),
        all.join(", "),
        label_impl("AccentClass"),
        a.classes.len()
    );
    for c in a.classes.iter() {
        let _ = writeln!(
            s,
            "    StressClass {{\n        name: Cow::Borrowed({}),\n        pattern: {}, // {}\n        vowel: {:?},\n        words: {},\n    }},",
            Lit(&c.name),
            c.pattern,
            pattern(c),
            c.vowel,
            c.words
        );
    }
    let value = |l: u16| names.get(usize::from(l)).cloned().unwrap_or_default();
    let _ = writeln!(
        s,
        "];\n\n{}/// Which class a word is of: {} rules and {} listed words.\npub const CLASS: Choice<'static, AccentClass> = {}}};\n\n/// Per {}, the cells whose ending takes the circumflex when accented on\n/// its last syllable (`θεοῦ`).\npub static CIRCUMFLEX: [Cow<'static, [u16]>; {}] = [",
        if a.class.rules.is_empty() && a.class.exceptions.is_empty() {
            ""
        } else {
            "use AccentClass::*;\n\n"
        },
        thousands(a.class.rules.len()),
        thousands(a.class.exceptions.len()),
        choice_macro("rules", &a.class, value),
        many.trim_end_matches('s'),
        a.circumflex.len()
    );
    for (cells, p) in a.circumflex.iter().zip(paradigms) {
        let names: Vec<&str> = cells.iter().map(|&c| layout.name(c)).collect();
        let _ = writeln!(s, "    Cow::Borrowed(&[{}]), // {p}", names.join(", "));
    }
    s.push_str("];\n\n");
    let finals: Vec<&str> = a.long_finals.iter().map(|&c| layout.name(c)).collect();
    let _ = writeln!(
        s,
        "/// The cells whose final `-αι`, `-οι` counts as long.\npub static LONG_FINALS: [u16; {}] = [{}];",
        finals.len(),
        finals.join(", ")
    );
    s
}

/// `choice` as `flexion::choice!` reads it, opening with `block` (`rules` or
/// `starts`): its rules and its listed words, each value written by `name`,
/// and the macro left open for the caller to close.
fn choice_macro<L: flexion::Label>(
    block: &str,
    choice: &flexion::Choice<'_, L>,
    name: impl Fn(L) -> String,
) -> String {
    format!(
        "flexion::choice! {{\n    {block} {{\n{}    }}\n    listed {{\n{}    }}\n",
        rules(choice, &name),
        listed(choice, &name)
    )
}

/// A choice's rules as `flexion::choice!` reads them, a line each, each
/// value written by `name`.
fn rules<L: flexion::Label>(choice: &flexion::Choice<'_, L>, name: impl Fn(L) -> String) -> String {
    let rule = |r: &flexion::Rule<'_, L>| {
        let counts: Vec<String> = r
            .counts
            .iter()
            .map(|&(l, n)| format!("{} {n}", name(l)))
            .collect();
        let regular = if r.regular { " regular" } else { "" };
        format!(
            "        {} => [{}]{regular},\n",
            Lit(&r.ending),
            counts.join(", ")
        )
    };
    choice.rules.iter().map(rule).collect()
}

/// A choice's listed words, a line each.
fn listed<L: flexion::Label>(
    choice: &flexion::Choice<'_, L>,
    name: impl Fn(L) -> String,
) -> String {
    let word =
        |(w, l): &(std::borrow::Cow<'_, str>, L)| format!("        {} => {},\n", Lit(w), name(*l));
    choice.exceptions.iter().map(word).collect()
}

/// A choice as a constant `name` of `ty`s, saying how many rules and listed
/// `words` it has.
fn choice_const<L: flexion::Label>(
    name: &str,
    ty: &str,
    words: &str,
    choice: &flexion::Choice<'_, L>,
    value: impl Fn(L) -> String,
) -> String {
    format!(
        "/// {} rules and {} listed {words}.\npub(super) const {name}: Choice<'static, {ty}> = {}}};\n",
        thousands(choice.rules.len()),
        thousands(choice.exceptions.len()),
        choice_macro("rules", choice, value)
    )
}

/// `impl flexion::Label` for the enum `ty`, by its places in `ALL`.
fn label_impl(ty: &str) -> String {
    format!(
        "impl flexion::Label for {ty} {{\n    fn index(self) -> usize {{\n        self as usize\n    }}\n    fn from_index(index: usize) -> Option<Self> {{\n        Self::ALL.get(index).copied()\n    }}\n}}\n\n"
    )
}

/// `names`, made unique by numbering the repeats (`Книга`, `Книга2`).
fn unique(names: impl Iterator<Item = String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for base in names {
        let mut v = base.clone();
        let mut k = 2;
        while out.contains(&v) {
            v = format!("{base}{k}");
            k += 1;
        }
        out.push(v);
    }
    out
}

/// The prefixes words inflect behind, written out: each prefix with how many
/// words separate it or keep it, the words that go against most, and per
/// cell where a separable prefix goes and what an inseparable one takes off.
fn prefixes_file(p: &flexion::Prefixes<'_>, layout: &Layout) -> String {
    let mut s = String::from(
        "//! Prefixes words inflect behind. A word that begins with one inflects as the\n\
         //! rest of it does: a separable prefix goes where the cell puts it (`Before`\n\
         //! the rest, written together, or `After` it, as a word of its own); an\n\
         //! inseparable one stays in front, takes off the front of the rest's form\n\
         //! what the cell says, and meets it as `junctions` says. How likely a word is to do either is how many of the\n\
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
            "        Prefix {{ letters: Cow::Borrowed({}), separable: {}, inseparable: {}, words: {} }},",
            Lit(&x.letters),
            x.separable,
            x.inseparable,
            x.words
        );
    }
    s.push_str("    ]),\n    listed: Cow::Borrowed(&[\n");
    for (w, sep) in p.listed.iter() {
        let _ = writeln!(
            s,
            "        (Cow::Borrowed({}), Separation::{sep:?}),",
            Lit(w)
        );
    }
    s.push_str("    ]),\n    placement: Cow::Borrowed(&[\n");
    for (c, place) in p.placement.iter().enumerate() {
        let _ = writeln!(s, "        {place:?}, // {}", layout.name(c));
    }
    s.push_str("    ]),\n    drops: Cow::Borrowed(&[\n");
    for (c, d) in p.drops.iter().enumerate() {
        let name = layout.name(c);
        let _ = writeln!(s, "        Cow::Borrowed({}), // {name}", Lit(d));
    }
    s.push_str(
        "    ]),\n    // How an inseparable prefix meets the rest's form, by the form's first\n    // letter: what the two become.\n    junctions: Cow::Borrowed(&[\n",
    );
    for (prefix, head, joined) in p.junctions.iter() {
        let _ = writeln!(
            s,
            "        (Cow::Borrowed({}), Cow::Borrowed({}), Cow::Borrowed({})), // {prefix} + {head} → {joined}",
            Lit(prefix),
            Lit(head),
            Lit(joined)
        );
    }
    s.push_str("    ]),\n};\n");
    s
}

/// A Rust string literal of its text, written as it is: only `"`, `\` and
/// control characters are escaped (combining accents stay readable).
struct Lit<'a>(&'a str);

impl std::fmt::Display for Lit<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        for c in self.0.chars() {
            match c {
                '"' | '\\' => write!(f, "\\{c}")?,
                c if c.is_control() => write!(f, "{}", c.escape_default())?,
                c => f.write_char(c)?,
            }
        }
        f.write_char('"')
    }
}

/// `name` applied to `args`, if it has any: `Case(case)`.
fn call(name: &str, args: &[String]) -> String {
    if args.is_empty() {
        name.to_string()
    } else {
        format!("{name}({})", args.join(", "))
    }
}

/// `items` as a tuple, or the one item.
fn tuple(items: &[String]) -> String {
    match items {
        [one] => one.clone(),
        _ => format!("({})", items.join(", ")),
    }
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
fn api_code(
    spec: &PartOfSpeech,
    learned: &Learned<'_>,
    layout: &Layout,
    cells: &Cells<'_>,
    feature: Option<&str>,
    inherent: &[&Feature],
) -> String {
    let (ty, one, many) = kind(spec);
    let name = spec.name;
    let word = variant(name);
    // The cell type.
    let mut s = format!(
        "/// A {name} form: which cell of its {one}.\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum Cell {{\n"
    );
    // A group's features, and their types.
    let fields = |g: &GroupCells<'_>| g.features.iter().map(|f| f.name).collect::<Vec<_>>();
    let paths = |g: &GroupCells<'_>| -> Vec<String> {
        fields(g).iter().map(|f| format!("crate::{f}")).collect()
    };
    for g in &cells.groups {
        let v = call(&cell_variant(name, g.name), &paths(g));
        let _ = writeln!(s, "    /// {}\n    {v},", g.group.doc);
    }
    // A feature's values are written bare where no other feature has a value
    // of the same name.
    let mut features: Vec<&Feature> = cells
        .groups
        .iter()
        .flat_map(|g| g.features.iter().copied())
        .collect();
    features.sort_by_key(|f| f.name);
    features.dedup_by_key(|f| f.name);
    let bare: Vec<&str> = features
        .iter()
        .filter(|f| {
            f.values.iter().all(|v| {
                features
                    .iter()
                    .filter(|o| o.name != f.name)
                    .all(|o| o.values.iter().all(|w| w.name != v.name))
            })
        })
        .map(|f| f.name)
        .collect();
    let uses: String = bare
        .iter()
        .map(|f| format!("        use crate::{f}::*;\n"))
        .collect();
    let _ = write!(
        s,
        "}}\n\nimpl Cell {{\n    /// Its place in a paradigm.\n    pub const fn index(self) -> usize {{\n{uses}        let cell = match self {{\n"
    );
    for g in &cells.groups {
        let v = cell_variant(name, g.name);
        for cell in g.offset..g.offset + g.len() {
            let args: Vec<String> = g
                .features
                .iter()
                .zip(g.values(cell))
                .map(|(f, x)| {
                    let value = f.values.get(x).map_or("", |v| v.name);
                    if bare.contains(&f.name) {
                        value.to_string()
                    } else {
                        format!("crate::{}::{value}", f.name)
                    }
                })
                .collect();
            let constant = layout.name(cell);
            let _ = writeln!(
                s,
                "            Cell::{} => cell::{constant},",
                call(&v, &args)
            );
        }
    }
    s.push_str("        };\n        cell as usize\n    }\n}\n\n");
    // Conversions from a group's values, where no other group has the same
    // features.
    for g in &cells.groups {
        let shared = cells
            .groups
            .iter()
            .filter(|o| fields(o) == fields(g))
            .count();
        if g.features.is_empty() || shared > 1 {
            continue;
        }
        let vars: Vec<String> = g.features.iter().map(|f| snake(f.name)).collect();
        let _ = writeln!(
            s,
            "impl From<{ty_}> for Cell {{\n    fn from({}: {ty_}) -> Self {{\n        {}\n    }}\n}}\n",
            tuple(&vars),
            call(&format!("Cell::{}", cell_variant(name, g.name)), &vars),
            ty_ = tuple(&paths(g)),
        );
    }
    // The word.
    let (inherent_field, inherent_new, inherent_value) = if inherent.is_empty() {
        ("", "", "&[]")
    } else {
        (
            "    inherent: Vec<(u8, u8)>,\n",
            "            inherent: Vec::new(),\n",
            "&self.inherent",
        )
    };
    let folds = !learned.grammar.orthography.folded.is_empty();
    let (full_field, full_new, full, spelled_in_full) = if folds {
        (
            "    full: bool,\n",
            "            full: false,\n",
            "self.full",
            "    /// Its lemma is spelled with every mark the dictionary writes, so\n    /// none is hidden (without this, a lemma typed without marks may be\n    /// missing some, and its forms are less sure).\n    #[must_use]\n    pub fn spelled_in_full(mut self) -> Self {\n        self.full = true;\n        self\n    }\n\n",
        )
    } else {
        ("", "", "false", "")
    };
    let _ = writeln!(
        s,
        "/// A {name} to inflect: its lemma, and what you know of it. The more you\n/// tell, the surer its forms.\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct {word}<'w> {{\n    lemma: &'w str,\n    given: Vec<(usize, &'w str)>,\n{inherent_field}{full_field}}}\n\nimpl<'w> {word}<'w> {{\n    /// The {name} `lemma`, as the dictionary writes it.\n    #[must_use]\n    pub fn new(lemma: &'w str) -> Self {{\n        {word} {{\n            lemma,\n            given: Vec::new(),\n{inherent_new}{full_new}        }}\n    }}\n\n    /// One of its forms you know (a principal part): readings that disagree\n    /// with it are dropped.\n    #[must_use]\n    pub fn given(mut self, cell: impl Into<Cell>, form: &'w str) -> Self {{\n        self.given.push((cell.into().index(), form));\n        self\n    }}\n"
    );
    for (i, f) in inherent.iter().enumerate() {
        let field = snake(f.name);
        let lower = f.name.to_lowercase();
        let doc = if feature == Some(f.name) {
            format!(
                "Its {lower}, which says which cell some `derived` forms (see its\n    /// {many}) come from."
            )
        } else {
            format!("Its {lower}: only {many} of words of that {lower} are read.")
        };
        let _ = writeln!(
            s,
            "    /// {doc}\n    #[must_use]\n    pub fn {field}(mut self, {field}: crate::{}) -> Self {{\n        self.inherent.push(({i}, {field} as u8));\n        self\n    }}\n",
            f.name
        );
    }
    let _ = writeln!(
        s,
        "{spelled_in_full}    fn known(&self) -> Known<'_> {{\n        Known {{\n            forms: &self.given,\n            inherent: {inherent_value},\n            full: {full},\n        }}\n    }}\n\n    /// Its likeliest form in `cell`.\n    #[must_use]\n    pub fn form(&self, cell: impl Into<Cell>) -> String {{\n        GRAMMAR.best(self.lemma, cell.into().index(), &self.known())\n    }}\n\n    /// Every form it may have in `cell`, likeliest first, each with its\n    /// probability and why.\n    #[must_use]\n    pub fn candidates(&self, cell: impl Into<Cell>) -> Vec<Candidate<'static>> {{\n        GRAMMAR.candidates(self.lemma, cell.into().index(), &self.known())\n    }}\n\n    /// The {many} it may follow, likeliest first, each with its probability\n    /// and why.\n    #[must_use]\n    pub fn {one}(&self) -> Vec<Reading<'static, 'static, {ty}>> {{\n        GRAMMAR.readings(self.lemma, &self.known())\n    }}\n}}",
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
    let n = f.name;
    let each = |line: &dyn Fn(&Value) -> String| f.values.iter().map(line).collect::<String>();
    let _ = writeln!(
        s,
        "/// {}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]\npub enum {n} {{\n{}}}\n\nimpl {n} {{\n    /// Every value, in order.\n    pub const ALL: [{n}; {}] = [\n{}    ];\n\n    /// Short name, as in cell names.\n    pub const fn short(self) -> &'static str {{\n        match self {{\n{}        }}\n    }}\n}}\n\nimpl std::fmt::Display for {n} {{\n    /// Its name in lowercase: `genitive`.\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n        f.write_str(match self {{\n{}        }})\n    }}\n}}\n\nimpl std::str::FromStr for {n} {{\n    type Err = flexion::UnknownValue;\n\n    /// By its name (`genitive`) or its short name (`gen`).\n    fn from_str(name: &str) -> Result<Self, Self::Err> {{\n        Self::ALL\n            .into_iter()\n            .find(|v| v.short() == name || v.to_string() == name)\n            .ok_or_else(|| flexion::UnknownValue {{\n                feature: {n:?},\n                name: name.to_string(),\n            }})\n    }}\n}}\n",
        f.doc,
        each(&|v| format!("    {},\n", v.name)),
        f.values.len(),
        each(&|v| format!("        {n}::{},\n", v.name)),
        each(&|v| format!("            {n}::{} => \"{}\",\n", v.name, v.short)),
        each(&|v| format!(
            "            {n}::{} => {:?},\n",
            v.name,
            v.name.to_lowercase()
        )),
    );
    s.push_str(&label_impl(f.name));
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

/// The crate's README: the language's intro, how the grammar is laid out
/// (the files its parts of speech have, by name: `files`), and `report`
/// (what it is and how well it does).
pub fn readme(lang: &Language, report: &str, files: &[String]) -> String {
    let sources: String = lang
        .editions
        .iter()
        .map(|e| format!(" and {}'s", e.name))
        .collect();
    let has = |f: &str| files.iter().any(|g| g == f);
    let known = [
        (
            "paradigms.rs",
            "how words inflect, one table per paradigm, named after a word that\n  \
             follows it, with a pattern per form (`~`: the stem, then what follows it;\n  \
             `derived`: another cell's form, changed)",
        ),
        (
            "relations.rs",
            "which cell each cell's form is told from, and the rules on that\n  \
             form's ending that change it",
        ),
        (
            "laws.rs",
            "sound laws where a stem meets its ending or a literal before it (along a\n  \
             tier, on a copied letter, or on a class letter an ending writes: harmony), each\n  \
             with the words whose forms it gives and the forms it gets wrong",
        ),
        (
            "rules.rs",
            "which paradigm a word takes, as rules on its ending, each with how many\n  \
             of the dictionary's words with that ending follow each paradigm, and the words no\n  \
             rule gets right, listed",
        ),
        (
            "accent.rs",
            "per cell, which rule places the accent, and which words have which\n  \
             pattern",
        ),
        (
            "prefixes.rs",
            "prefixes a word inflects behind, separable or not",
        ),
        (
            "compounds.rs",
            "whether a word written in parts inflects each part",
        ),
        (
            "marks.rs",
            "how a letter typed without its mark (vowel length) is written",
        ),
        (
            "spellings.rs",
            "the words whose spelling their typed form doesn't show",
        ),
        (
            "stems.rs",
            "the stem slots (the cells a word builds on one stem, learned from the\n  \
             words that put different stems in different cells) and the ending tables paradigms\n  \
             share over a slot, each named after its commonest word",
        ),
        (
            "irregular.rs",
            "what no paradigm gives a word: the stem it lists for a slot, built\n  \
             with an ending table, and the forms no stem gives",
        ),
    ];
    let mut list: String = known
        .iter()
        .filter(|(f, _)| has(f))
        .map(|(f, what)| format!("- `{f}`: {what};\n"))
        .collect();
    // A file per inherent feature relations depend on (`gender.rs`).
    for f in files
        .iter()
        .filter(|f| f.as_str() != "mod.rs" && known.iter().all(|(k, _)| k != f))
    {
        let feature = f.trim_end_matches(".rs").replace('_', " ");
        let _ = writeln!(
            list,
            "- `{f}`: a word's {feature}, which decides which cell some cells' forms\n  come from;"
        );
    }
    let list = list.trim_end().trim_end_matches(';');
    format!(
        "# {name}\n\n{intro}\n## How it works\n\n\
         This crate is {display} inflection as a grammar, learned by\n\
         [flexion](https://github.com/gold-silver-copper/flexion) from the English Wiktionary's{sources}\n\
         {display} entries. Each part of speech is a module under `src/`, and has what its\n\
         grammar needs of:\n\n\
         {list}.\n\n\
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
