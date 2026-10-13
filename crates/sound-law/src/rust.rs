//! Compiled programs written as Rust: [`Program::to_rust`] prints a constant
//! expression, and [`Program::from_rules`] runs it with no compile step.

use std::borrow::Cow;
use std::fmt::Write as _;

use crate::Program;
use crate::compile::{Atom, CompiledRule, Matcher, Pattern, ReplacementPart};

impl Program<'_> {
    /// The program's rules as a Rust expression of type
    /// `&[CompiledRule<'static>]`, for a `const` that
    /// [`Program::from_rules`] runs. It names the types as
    /// [`compiled`](crate::compiled) exports them: `use
    /// sound_law::compiled::*;` brings them into scope.
    pub fn to_rust(&self) -> String {
        let rules: Vec<String> = self.rules().iter().map(CompiledRule::to_rust).collect();
        format!("&[{}]", rules.join(", "))
    }
}

impl CompiledRule<'_> {
    /// The rule as a Rust expression of type `CompiledRule<'static>`; see
    /// [`Program::to_rust`].
    pub fn to_rust(&self) -> String {
        let excluded = |p: &Option<Pattern<'_>>| {
            p.as_ref()
                .map_or("None".to_string(), |p| format!("Some({})", pattern(p)))
        };
        let replacement: Vec<String> = self.replacement.iter().map(part).collect();
        format!(
            "CompiledRule {{ name: B({:?}), left: {}, target: {}, right: {}, \
             excluded_left: {}, excluded_right: {}, replacement: B(&[{}]), \
             capture_count: {}, direction: Direction::{:?}, \
             application: Application::{:?}, required: B({}) }}",
            self.name,
            pattern(&self.left),
            pattern(&self.target),
            pattern(&self.right),
            excluded(&self.excluded_left),
            excluded(&self.excluded_right),
            replacement.join(", "),
            self.capture_count,
            self.direction,
            self.application,
            strings(&self.required),
        )
    }
}

fn pattern(p: &Pattern<'_>) -> String {
    let atoms: Vec<String> = p.atoms.iter().map(atom).collect();
    format!(
        "Pattern {{ atoms: B(&[{}]), anchored_at_start: {}, anchored_at_end: {} }}",
        atoms.join(", "),
        p.anchored_at_start,
        p.anchored_at_end
    )
}

fn atom(a: &Atom<'_>) -> String {
    let matcher = match &a.matcher {
        Matcher::Literal(s) => format!("Matcher::Literal(B({s:?}))"),
        Matcher::Class(m) => format!("Matcher::Class(B({}))", strings(m)),
        Matcher::NotClass(m) => format!("Matcher::NotClass(B({}))", strings(m)),
        Matcher::Any => "Matcher::Any".to_string(),
        Matcher::WordBoundary => "Matcher::WordBoundary".to_string(),
        Matcher::Backreference(i) => format!("Matcher::Backreference({i})"),
        Matcher::NotAhead(m) => format!("Matcher::NotAhead(B({}))", strings(m)),
    };
    format!(
        "Atom {{ matcher: {matcher}, quantifier: Quantifier::{:?}, capture: {:?} }}",
        a.quantifier, a.capture
    )
}

fn part(p: &ReplacementPart<'_>) -> String {
    match p {
        ReplacementPart::Literal(s) => format!("ReplacementPart::Literal(B({s:?}))"),
        ReplacementPart::Capture(i) => format!("ReplacementPart::Capture({i})"),
        ReplacementPart::MappedCapture { capture, entries } => {
            let mut e = String::new();
            for (a, b) in entries.iter() {
                let _ = write!(e, "(B({a:?}), B({b:?})), ");
            }
            format!("ReplacementPart::MappedCapture {{ capture: {capture}, entries: B(&[{e}]) }}")
        }
        ReplacementPart::AgreeingCapture {
            capture,
            key,
            entries,
        } => {
            let mut e = String::new();
            for (a, k, b) in entries.iter() {
                let _ = write!(e, "(B({a:?}), B({k:?}), B({b:?})), ");
            }
            format!(
                "ReplacementPart::AgreeingCapture {{ capture: {capture}, key: {key}, entries: B(&[{e}]) }}"
            )
        }
    }
}

/// `&["a", "b"]` as borrowed strings.
fn strings(list: &[Cow<'_, str>]) -> String {
    let items: Vec<String> = list.iter().map(|s| format!("B({s:?})")).collect();
    format!("&[{}]", items.join(", "))
}
