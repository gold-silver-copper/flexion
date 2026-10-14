use std::borrow::Cow;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use crate::engine::Program;
use crate::{Application, CompileError, Direction, RuleSet};

const MAX_PATTERN_ATOMS: usize = 256;

/// One compiled rule: its patterns parsed, its classes and maps resolved.
///
/// [`RuleSet::compile`] makes these; [`CompiledRule::to_rust`] writes one as
/// a constant expression, so a build step can emit a ready-to-run program as
/// static data. Rules written by hand need not keep the compiler's
/// invariants; the engine never panics on them, but may not match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledRule<'a> {
    pub name: Cow<'a, str>,
    pub left: Pattern<'a>,
    pub target: Pattern<'a>,
    pub right: Pattern<'a>,
    pub excluded_left: Option<Pattern<'a>>,
    pub excluded_right: Option<Pattern<'a>>,
    pub replacement: Cow<'a, [ReplacementPart<'a>]>,
    pub capture_count: usize,
    pub direction: Direction,
    pub application: Application,
    /// Literals every match needs, in its environment or target: a word
    /// without one of them is left as it is, unscanned.
    pub required: Cow<'a, [Cow<'a, str>]>,
}

/// A compiled pattern: atoms matched in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern<'a> {
    pub atoms: Cow<'a, [Atom<'a>]>,
    pub anchored_at_start: bool,
    pub anchored_at_end: bool,
}

/// One matcher, how often it repeats, and the capture it fills.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom<'a> {
    pub matcher: Matcher<'a>,
    pub quantifier: Quantifier,
    pub capture: Option<usize>,
}

/// What one atom matches. Class members are tried longest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Matcher<'a> {
    Literal(Cow<'a, str>),
    Class(Cow<'a, [Cow<'a, str>]>),
    NotClass(Cow<'a, [Cow<'a, str>]>),
    Any,
    WordBoundary,
    Backreference(usize),
    /// Nothing, where no member begins: `{?!V}`, read ahead.
    NotAhead(Cow<'a, [Cow<'a, str>]>),
    /// What any of the patterns matches, captures in none: a right context
    /// with others it may be instead ([`Rule::or_before`](crate::Rule::or_before)).
    Either(Cow<'a, [Pattern<'a>]>),
}

/// How many times an atom matches: once, `?`, `*` or `+`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    One,
    Optional,
    ZeroOrMore,
    OneOrMore,
}

/// One piece of a replacement: literal text, a capture, or a capture put
/// through a map, or through an agreement map by another capture (`key`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplacementPart<'a> {
    Literal(Cow<'a, str>),
    Capture(usize),
    MappedCapture {
        capture: usize,
        entries: Cow<'a, [(Cow<'a, str>, Cow<'a, str>)]>,
    },
    AgreeingCapture {
        capture: usize,
        key: usize,
        entries: Cow<'a, [Agreement<'a>]>,
    },
}

/// One entry of an agreement map: a source, the key it is keyed by, and
/// its output.
pub type Agreement<'a> = (Cow<'a, str>, Cow<'a, str>, Cow<'a, str>);

#[derive(Debug, Clone)]
struct ResolvedClass {
    members: Vec<String>,
}

#[derive(Debug, Clone)]
struct ResolvedMap {
    entries: Vec<(String, String)>,
    keyed: Vec<(String, String, String)>,
}

#[derive(Debug, Clone)]
struct CaptureDef {
    index: usize,
    /// The exact possible values, when statically knowable.
    values: Option<Vec<String>>,
}

pub(crate) fn compile(spec: &RuleSet<'_>) -> Result<Program<'static>, CompileError> {
    let classes = resolve_classes(spec)?;
    let maps = resolve_maps(spec)?;
    let mut seen_rules = HashSet::new();
    let mut compiled = Vec::with_capacity(spec.rules().len());

    for rule in spec.rules() {
        if !is_identifier(rule.name()) {
            return Err(CompileError::in_rule(
                rule.name(),
                "the rule name must be an ASCII identifier",
            ));
        }
        if !seen_rules.insert(rule.name()) {
            return Err(CompileError::in_rule(rule.name(), "duplicate rule name"));
        }

        // Context is matched left-to-right. Captures made on the left are
        // therefore available as backreferences in the target, and target
        // captures are available on the right.
        let mut captures = HashMap::new();
        let left = parse_pattern(
            rule.left_context(),
            &classes,
            &mut captures,
            rule.name(),
            "left context",
        )?;
        let target = parse_pattern(
            rule.target(),
            &classes,
            &mut captures,
            rule.name(),
            "target",
        )?;
        let right = parse_pattern(
            rule.right_context(),
            &classes,
            &mut captures,
            rule.name(),
            "right context",
        )?;
        let mut or_right = Vec::with_capacity(rule.or_right_contexts().len());
        for source in rule.or_right_contexts() {
            let mut theirs = captures.clone();
            or_right.push(parse_pattern(
                source,
                &classes,
                &mut theirs,
                rule.name(),
                "right context",
            )?);
            if theirs.len() != captures.len() {
                return Err(CompileError::in_rule(
                    rule.name(),
                    "a right context after the first cannot declare captures",
                ));
            }
        }
        let replacement = parse_replacement(rule.replacement(), &captures, &maps, rule.name())?;

        let (excluded_left, excluded_right) = match (
            rule.excluded_left_context(),
            rule.excluded_right_context(),
        ) {
            (Some(left_source), Some(right_source)) => {
                let mut exclusion_captures = captures.clone();
                let left = parse_pattern(
                    left_source,
                    &classes,
                    &mut exclusion_captures,
                    rule.name(),
                    "excluded left context",
                )?;
                let right = parse_pattern(
                    right_source,
                    &classes,
                    &mut exclusion_captures,
                    rule.name(),
                    "excluded right context",
                )?;
                if exclusion_captures.len() != captures.len() {
                    return Err(CompileError::in_rule(
                        rule.name(),
                        "excluded environments cannot declare captures; capture in the positive environment and use a backreference instead",
                    ));
                }
                (Some(left), Some(right))
            }
            (None, None) => (None, None),
            _ => unreachable!("Rule::unless_context always sets both context halves"),
        };

        // A right context's literals only where every one has them.
        let literals = |p: &Pattern<'static>| -> Vec<Cow<'static, str>> {
            p.atoms
                .iter()
                .filter(|a| matches!(a.quantifier, Quantifier::One | Quantifier::OneOrMore))
                .filter_map(|a| match &a.matcher {
                    Matcher::Literal(s) if !s.is_empty() => Some(s.clone()),
                    _ => None,
                })
                .collect()
        };
        let mut required = literals(&left);
        required.extend(literals(&target));
        required.extend(
            literals(&right)
                .into_iter()
                .filter(|l| or_right.iter().all(|r| literals(r).contains(l))),
        );
        let required = required.into();
        // Several right contexts: one, matching any of them.
        let right = if or_right.is_empty() {
            right
        } else {
            let either = std::iter::once(right).chain(or_right).collect::<Vec<_>>();
            Pattern {
                atoms: vec![Atom {
                    matcher: Matcher::Either(either.into()),
                    quantifier: Quantifier::One,
                    capture: None,
                }]
                .into(),
                anchored_at_start: false,
                anchored_at_end: false,
            }
        };
        compiled.push(CompiledRule {
            name: Cow::Owned(rule.name().to_owned()),
            required,
            left,
            target,
            right,
            excluded_left,
            excluded_right,
            replacement: replacement.into(),
            capture_count: captures.len(),
            direction: rule.direction(),
            application: rule.application(),
        });
    }

    Ok(Program::new(compiled))
}

fn resolve_maps(spec: &RuleSet<'_>) -> Result<HashMap<String, ResolvedMap>, CompileError> {
    let mut maps = HashMap::with_capacity(spec.maps().len());

    for map in spec.maps() {
        if !is_identifier(map.name()) {
            return Err(CompileError::global(format!(
                "sound-law map `{}` must have an ASCII identifier name",
                map.name()
            )));
        }
        if maps.contains_key(map.name()) {
            return Err(CompileError::global(format!(
                "duplicate sound-law map `{}`",
                map.name()
            )));
        }

        let plain = map
            .entries()
            .iter()
            .map(|&(source, output)| (source, "", output));
        let keyed = map.keyed_entries().iter().copied();
        let mut seen = HashSet::new();
        let mut entries = Vec::with_capacity(map.entries().len());
        let mut agreeing = Vec::with_capacity(map.keyed_entries().len());
        for (source, key, output) in plain.chain(keyed) {
            if source.is_empty() {
                return Err(CompileError::global(format!(
                    "sound-law map `{}` contains an empty source",
                    map.name()
                )));
            }
            if !seen.insert((source, key)) {
                return Err(CompileError::global(format!(
                    "sound-law map `{}` contains duplicate source `{source}`{}",
                    map.name(),
                    if map.keyed_entries().is_empty() {
                        String::new()
                    } else {
                        format!(" for key `{key}`")
                    }
                )));
            }
            if map.keyed_entries().is_empty() {
                entries.push((source.to_owned(), output.to_owned()));
            } else {
                agreeing.push((source.to_owned(), key.to_owned(), output.to_owned()));
            }
        }
        if !map.entries().is_empty() && !map.keyed_entries().is_empty() {
            return Err(CompileError::global(format!(
                "sound-law map `{}` is both plain and keyed",
                map.name()
            )));
        }
        if entries.is_empty() && agreeing.is_empty() {
            return Err(CompileError::global(format!(
                "sound-law map `{}` has no entries",
                map.name()
            )));
        }
        maps.insert(
            map.name().to_owned(),
            ResolvedMap {
                entries,
                keyed: agreeing,
            },
        );
    }

    Ok(maps)
}

fn resolve_classes(spec: &RuleSet<'_>) -> Result<HashMap<String, ResolvedClass>, CompileError> {
    let mut classes = HashMap::with_capacity(spec.classes().len());

    for class in spec.classes() {
        if !is_identifier(class.name()) {
            return Err(CompileError::global(format!(
                "sound-law class `{}` must have an ASCII identifier name",
                class.name()
            )));
        }
        if classes.contains_key(class.name()) {
            return Err(CompileError::global(format!(
                "duplicate sound-law class `{}`",
                class.name()
            )));
        }

        let mut seen = HashSet::new();
        let mut members = Vec::with_capacity(class.members().len());
        for member in class.members() {
            if member.is_empty() {
                return Err(CompileError::global(format!(
                    "sound-law class `{}` contains an empty member",
                    class.name()
                )));
            }
            if !seen.insert(*member) {
                return Err(CompileError::global(format!(
                    "sound-law class `{}` contains duplicate member `{member}`",
                    class.name()
                )));
            }
            members.push((*member).to_owned());
        }
        if members.is_empty() {
            return Err(CompileError::global(format!(
                "sound-law class `{}` has no members",
                class.name()
            )));
        }

        // `sort_by_key` is stable, retaining declaration order for equal-sized
        // alternatives while ensuring a digraph wins over its prefix.
        members.sort_by_key(|member| Reverse(member.len()));
        classes.insert(class.name().to_owned(), ResolvedClass { members });
    }

    Ok(classes)
}

fn parse_pattern(
    source: &str,
    classes: &HashMap<String, ResolvedClass>,
    captures: &mut HashMap<String, CaptureDef>,
    rule: &str,
    location: &str,
) -> Result<Pattern<'static>, CompileError> {
    let (body, anchored_at_start, anchored_at_end) = strip_anchors(source);
    let mut atoms = Vec::new();
    let mut literal = String::new();
    let mut chars = body.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                let escaped = chars.next().ok_or_else(|| {
                    pattern_error(rule, location, "a trailing backslash escapes nothing")
                })?;
                literal.push(escaped);
            }
            '{' => {
                push_literal(&mut atoms, &mut literal);
                let mut placeholder = String::new();
                loop {
                    match chars.next() {
                        Some('}') => break,
                        Some('{') => {
                            return Err(pattern_error(
                                rule,
                                location,
                                "nested `{` inside a placeholder",
                            ));
                        }
                        Some(ch) => placeholder.push(ch),
                        None => {
                            return Err(pattern_error(
                                rule,
                                location,
                                "an opening `{` has no closing `}`",
                            ));
                        }
                    }
                }
                atoms.push(parse_placeholder(
                    placeholder.trim(),
                    classes,
                    captures,
                    rule,
                    location,
                )?);
            }
            '}' => {
                return Err(pattern_error(
                    rule,
                    location,
                    "an unescaped `}` has no opening `{`",
                ));
            }
            '^' | '$' => {
                return Err(pattern_error(
                    rule,
                    location,
                    format!(
                        "anchor `{ch}` is only valid at its pattern boundary; escape it for a literal"
                    ),
                ));
            }
            '#' => {
                push_literal(&mut atoms, &mut literal);
                atoms.push(Atom {
                    matcher: Matcher::WordBoundary,
                    quantifier: Quantifier::One,
                    capture: None,
                });
            }
            _ => literal.push(ch),
        }
    }
    push_literal(&mut atoms, &mut literal);

    if atoms.len() > MAX_PATTERN_ATOMS {
        return Err(pattern_error(
            rule,
            location,
            format!(
                "pattern has {} atoms; the maximum is {MAX_PATTERN_ATOMS}",
                atoms.len()
            ),
        ));
    }

    Ok(Pattern {
        atoms: atoms.into(),
        anchored_at_start,
        anchored_at_end,
    })
}

fn parse_placeholder(
    placeholder: &str,
    classes: &HashMap<String, ResolvedClass>,
    captures: &mut HashMap<String, CaptureDef>,
    rule: &str,
    location: &str,
) -> Result<Atom<'static>, CompileError> {
    if placeholder.is_empty() {
        return Err(pattern_error(rule, location, "empty `{}` placeholder"));
    }

    let (core, quantifier) = match placeholder.chars().last() {
        Some('?') => (&placeholder[..placeholder.len() - 1], Quantifier::Optional),
        Some('*') => (
            &placeholder[..placeholder.len() - 1],
            Quantifier::ZeroOrMore,
        ),
        Some('+') => (&placeholder[..placeholder.len() - 1], Quantifier::OneOrMore),
        _ => (placeholder, Quantifier::One),
    };
    if core.is_empty() {
        return Err(pattern_error(
            rule,
            location,
            "a quantifier must follow a class or wildcard",
        ));
    }

    if let Some(expression) = core.strip_prefix("?!") {
        if quantifier != Quantifier::One {
            return Err(pattern_error(
                rule,
                location,
                "a look-ahead cannot be quantified",
            ));
        }
        return Ok(Atom {
            matcher: Matcher::NotAhead(owned(resolve_class_expression(
                classes, expression, rule, location,
            )?)),
            quantifier,
            capture: None,
        });
    }

    if let Some(name) = core.strip_prefix('=') {
        if quantifier != Quantifier::One {
            return Err(pattern_error(
                rule,
                location,
                "backreferences cannot be quantified",
            ));
        }
        if !is_identifier(name) {
            return Err(pattern_error(
                rule,
                location,
                format!("invalid backreference name `{name}`"),
            ));
        }
        let capture = captures
            .get(name)
            .map(|capture| capture.index)
            .ok_or_else(|| {
                pattern_error(
                    rule,
                    location,
                    format!("backreference `{name}` appears before its capture"),
                )
            })?;
        return Ok(Atom {
            matcher: Matcher::Backreference(capture),
            quantifier,
            capture: None,
        });
    }

    let (capture_name, matcher_name) = match core.split_once(':') {
        Some((capture, matcher)) => {
            if !is_identifier(capture) {
                return Err(pattern_error(
                    rule,
                    location,
                    format!("invalid capture name `{capture}`"),
                ));
            }
            if matcher.is_empty() {
                return Err(pattern_error(
                    rule,
                    location,
                    format!("capture `{capture}` has no matcher"),
                ));
            }
            (Some(capture), matcher)
        }
        None => (None, core),
    };

    let matcher = if matcher_name == "." {
        Matcher::Any
    } else if let Some(expression) = matcher_name.strip_prefix('!') {
        Matcher::NotClass(owned(resolve_class_expression(
            classes, expression, rule, location,
        )?))
    } else {
        Matcher::Class(owned(resolve_class_expression(
            classes,
            matcher_name,
            rule,
            location,
        )?))
    };

    let capture = match capture_name {
        Some(name) => {
            if captures.contains_key(name) {
                return Err(pattern_error(
                    rule,
                    location,
                    format!("capture `{name}` is declared more than once"),
                ));
            }
            let index = captures.len();
            // An optional capture that matched nothing has the empty text.
            let values = match (&matcher, quantifier) {
                (Matcher::Class(members), Quantifier::One) => {
                    Some(members.iter().map(|m| m.to_string()).collect())
                }
                (Matcher::Class(members), Quantifier::Optional) => Some(
                    members
                        .iter()
                        .map(|m| m.to_string())
                        .chain(std::iter::once(String::new()))
                        .collect(),
                ),
                _ => None,
            };
            captures.insert(name.to_owned(), CaptureDef { index, values });
            Some(index)
        }
        None => None,
    };

    Ok(Atom {
        matcher,
        quantifier,
        capture,
    })
}

fn resolve_class_expression(
    classes: &HashMap<String, ResolvedClass>,
    source: &str,
    rule: &str,
    location: &str,
) -> Result<Vec<String>, CompileError> {
    let union_terms = split_top_level(source, '|', rule, location)?;
    let mut result = Vec::new();
    for term in union_terms {
        let (operands, operators) = split_class_term(term, rule, location)?;
        let Some((&first, rest)) = operands.split_first() else {
            return Err(pattern_error(rule, location, "empty class term"));
        };
        let mut term_values = resolve_class_operand(classes, first, rule, location)?;
        for (operator, operand) in operators.into_iter().zip(rest) {
            let right = resolve_class_operand(classes, operand, rule, location)?;
            match operator {
                '&' => term_values.retain(|value| right.contains(value)),
                '-' => term_values.retain(|value| !right.contains(value)),
                _ => unreachable!("split_class_term only returns known operators"),
            }
        }
        for value in term_values {
            if !result.contains(&value) {
                result.push(value);
            }
        }
    }
    if result.is_empty() {
        return Err(pattern_error(
            rule,
            location,
            format!("class expression `{source}` has no members"),
        ));
    }
    result.sort_by_key(|member| Reverse(member.len()));
    Ok(result)
}

fn resolve_class_operand(
    classes: &HashMap<String, ResolvedClass>,
    source: &str,
    rule: &str,
    location: &str,
) -> Result<Vec<String>, CompileError> {
    let source = source.trim();
    if source.starts_with('[') {
        parse_inline_alternatives(source, rule, location)
    } else {
        resolve_class(classes, source, rule, location)
    }
}

fn split_class_term<'a>(
    source: &'a str,
    rule: &str,
    location: &str,
) -> Result<(Vec<&'a str>, Vec<char>), CompileError> {
    let mut operands = Vec::new();
    let mut operators = Vec::new();
    let mut start = 0;
    let mut in_inline = false;
    for (index, ch) in source.char_indices() {
        match ch {
            '[' => in_inline = true,
            ']' => in_inline = false,
            '&' | '-' if !in_inline => {
                operands.push(&source[start..index]);
                operators.push(ch);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    operands.push(&source[start..]);
    if operands.iter().any(|operand| operand.trim().is_empty()) {
        return Err(pattern_error(
            rule,
            location,
            format!("class expression `{source}` has a missing operand"),
        ));
    }
    Ok((operands, operators))
}

fn split_top_level<'a>(
    source: &'a str,
    separator: char,
    rule: &str,
    location: &str,
) -> Result<Vec<&'a str>, CompileError> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_inline = false;
    for (index, ch) in source.char_indices() {
        match ch {
            '[' if in_inline => {
                return Err(pattern_error(
                    rule,
                    location,
                    "class expressions cannot contain nested `[`",
                ));
            }
            '[' => in_inline = true,
            ']' if !in_inline => {
                return Err(pattern_error(
                    rule,
                    location,
                    "class expression has an unmatched `]`",
                ));
            }
            ']' => in_inline = false,
            _ if ch == separator && !in_inline => {
                parts.push(&source[start..index]);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    if in_inline {
        return Err(pattern_error(
            rule,
            location,
            "class expression has an unmatched `[`",
        ));
    }
    parts.push(&source[start..]);
    if parts.iter().any(|part| part.trim().is_empty()) {
        return Err(pattern_error(
            rule,
            location,
            format!("class expression `{source}` has a missing operand"),
        ));
    }
    Ok(parts)
}

fn parse_inline_alternatives(
    source: &str,
    rule: &str,
    location: &str,
) -> Result<Vec<String>, CompileError> {
    let Some(body) = source
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
    else {
        return Err(pattern_error(
            rule,
            location,
            "an inline alternative must end with `]`",
        ));
    };

    let mut alternatives = Vec::new();
    let mut current = String::new();
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                let escaped = chars.next().ok_or_else(|| {
                    pattern_error(
                        rule,
                        location,
                        "an inline alternative has a trailing backslash",
                    )
                })?;
                current.push(escaped);
            }
            '|' => alternatives.push(std::mem::take(&mut current)),
            '[' | ']' => {
                return Err(pattern_error(
                    rule,
                    location,
                    "inline alternatives cannot be nested",
                ));
            }
            _ => current.push(ch),
        }
    }
    alternatives.push(current);

    if alternatives.iter().any(String::is_empty) {
        return Err(pattern_error(
            rule,
            location,
            "an inline alternative cannot be empty",
        ));
    }
    let mut seen = HashSet::new();
    if let Some(duplicate) = alternatives
        .iter()
        .find(|alternative| !seen.insert(alternative.as_str()))
    {
        return Err(pattern_error(
            rule,
            location,
            format!("duplicate inline alternative `{duplicate}`"),
        ));
    }
    alternatives.sort_by_key(|alternative| Reverse(alternative.len()));
    Ok(alternatives)
}

fn resolve_class(
    classes: &HashMap<String, ResolvedClass>,
    name: &str,
    rule: &str,
    location: &str,
) -> Result<Vec<String>, CompileError> {
    if !is_identifier(name) {
        return Err(pattern_error(
            rule,
            location,
            format!("invalid class name `{name}`"),
        ));
    }
    classes
        .get(name)
        .map(|class| class.members.clone())
        .ok_or_else(|| pattern_error(rule, location, format!("unknown class `{name}`")))
}

fn parse_replacement(
    source: &str,
    captures: &HashMap<String, CaptureDef>,
    maps: &HashMap<String, ResolvedMap>,
    rule: &str,
) -> Result<Vec<ReplacementPart<'static>>, CompileError> {
    let mut parts = Vec::new();
    let mut literal = String::new();
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                let escaped = chars.next().ok_or_else(|| {
                    CompileError::in_rule(rule, "replacement has a trailing backslash")
                })?;
                literal.push(escaped);
            }
            '{' => {
                push_replacement_literal(&mut parts, &mut literal);
                let mut name = String::new();
                loop {
                    match chars.next() {
                        Some('}') => break,
                        Some('{') => {
                            return Err(CompileError::in_rule(
                                rule,
                                "replacement contains a nested `{`",
                            ));
                        }
                        Some(ch) => name.push(ch),
                        None => {
                            return Err(CompileError::in_rule(
                                rule,
                                "replacement has an opening `{` without a closing `}`",
                            ));
                        }
                    }
                }
                let reference = name.trim();
                // `{x|MAP[k]}`: an agreement map keyed by capture `k`.
                let (reference, key_name) =
                    match reference.strip_suffix(']').and_then(|r| r.split_once('[')) {
                        Some((r, k)) => (r.trim(), Some(k.trim())),
                        None => (reference, None),
                    };
                let (capture_name, map_name) = match reference.split_once('|') {
                    Some((capture, map)) => {
                        if map.contains('|') {
                            return Err(CompileError::in_rule(
                                rule,
                                format!(
                                    "replacement has invalid mapped capture reference `{reference}`"
                                ),
                            ));
                        }
                        (capture.trim(), Some(map.trim()))
                    }
                    None => (reference, None),
                };
                if !is_identifier(capture_name) {
                    return Err(CompileError::in_rule(
                        rule,
                        format!("replacement has invalid capture reference `{capture_name}`"),
                    ));
                }
                let capture = captures.get(capture_name).ok_or_else(|| {
                    CompileError::in_rule(
                        rule,
                        format!("replacement refers to unknown capture `{capture_name}`"),
                    )
                })?;
                if let (Some(map_name), Some(key_name)) = (map_name, key_name) {
                    parts.push(agreeing(
                        rule,
                        maps,
                        captures,
                        (capture_name, capture),
                        map_name,
                        key_name,
                    )?);
                } else if key_name.is_some() {
                    return Err(CompileError::in_rule(
                        rule,
                        format!("replacement `{reference}` has a key but no map"),
                    ));
                } else if let Some(map_name) = map_name {
                    if !is_identifier(map_name) {
                        return Err(CompileError::in_rule(
                            rule,
                            format!("replacement has invalid map reference `{map_name}`"),
                        ));
                    }
                    let map = maps.get(map_name).ok_or_else(|| {
                        CompileError::in_rule(
                            rule,
                            format!("replacement refers to unknown map `{map_name}`"),
                        )
                    })?;
                    let possible_values = capture.values.as_ref().ok_or_else(|| {
                        CompileError::in_rule(
                            rule,
                            format!(
                                "capture `{capture_name}` cannot use map `{map_name}` because its possible values are not statically known"
                            ),
                        )
                    })?;
                    let missing: Vec<&str> = possible_values
                        .iter()
                        .filter(|value| !map.entries.iter().any(|(source, _)| source == *value))
                        .map(String::as_str)
                        .collect();
                    if !missing.is_empty() {
                        return Err(CompileError::in_rule(
                            rule,
                            format!(
                                "map `{map_name}` does not cover capture `{capture_name}` values: {}",
                                missing.join(", ")
                            ),
                        ));
                    }
                    parts.push(ReplacementPart::MappedCapture {
                        capture: capture.index,
                        entries: map
                            .entries
                            .iter()
                            .map(|(a, b)| (Cow::Owned(a.clone()), Cow::Owned(b.clone())))
                            .collect(),
                    });
                } else {
                    parts.push(ReplacementPart::Capture(capture.index));
                }
            }
            '}' => {
                return Err(CompileError::in_rule(
                    rule,
                    "replacement has an unescaped `}`",
                ));
            }
            _ => literal.push(ch),
        }
    }
    push_replacement_literal(&mut parts, &mut literal);
    Ok(parts)
}

/// `{capture|map[key]}`: the capture through agreement map `map`, keyed by
/// capture `key`, which must cover every pair of their possible values.
fn agreeing(
    rule: &str,
    maps: &HashMap<String, ResolvedMap>,
    captures: &HashMap<String, CaptureDef>,
    (capture_name, capture): (&str, &CaptureDef),
    map_name: &str,
    key_name: &str,
) -> Result<ReplacementPart<'static>, CompileError> {
    let error = |message: String| CompileError::in_rule(rule, message);
    if !is_identifier(map_name) || !is_identifier(key_name) {
        return Err(error(format!(
            "replacement has invalid agreement `{map_name}[{key_name}]`"
        )));
    }
    let map = maps
        .get(map_name)
        .filter(|m| !m.keyed.is_empty())
        .ok_or_else(|| {
            error(format!(
                "replacement refers to unknown agreement map `{map_name}`"
            ))
        })?;
    let key = captures.get(key_name).ok_or_else(|| {
        error(format!(
            "replacement refers to unknown capture `{key_name}`"
        ))
    })?;
    let (Some(values), Some(keys)) = (&capture.values, &key.values) else {
        return Err(error(format!(
            "captures `{capture_name}` and `{key_name}` cannot use agreement map `{map_name}` because their possible values are not statically known"
        )));
    };
    let missing: Vec<String> = values
        .iter()
        .flat_map(|v| keys.iter().map(move |k| (v, k)))
        .filter(|(v, k)| !map.keyed.iter().any(|(s, x, _)| s == *v && x == *k))
        .map(|(v, k)| format!("{v} by `{k}`"))
        .collect();
    if !missing.is_empty() {
        return Err(error(format!(
            "agreement map `{map_name}` does not cover capture `{capture_name}` keyed by `{key_name}`: {}",
            missing.join(", ")
        )));
    }
    Ok(ReplacementPart::AgreeingCapture {
        capture: capture.index,
        key: key.index,
        entries: map
            .keyed
            .iter()
            .map(|(a, b, c)| {
                (
                    Cow::Owned(a.clone()),
                    Cow::Owned(b.clone()),
                    Cow::Owned(c.clone()),
                )
            })
            .collect(),
    })
}

/// Members as a compiled class.
fn owned(members: Vec<String>) -> Cow<'static, [Cow<'static, str>]> {
    members.into_iter().map(Cow::Owned).collect()
}

fn strip_anchors(source: &str) -> (&str, bool, bool) {
    let anchored_at_start = source.starts_with('^');
    let start = usize::from(anchored_at_start);
    let anchored_at_end = source.ends_with('$') && !is_escaped(source, source.len() - 1);
    let end = source.len() - usize::from(anchored_at_end);
    (&source[start..end], anchored_at_start, anchored_at_end)
}

fn is_escaped(source: &str, byte_index: usize) -> bool {
    let slash_count = source[..byte_index]
        .chars()
        .rev()
        .take_while(|ch| *ch == '\\')
        .count();
    slash_count % 2 == 1
}

fn push_literal(atoms: &mut Vec<Atom<'static>>, literal: &mut String) {
    if !literal.is_empty() {
        atoms.push(Atom {
            matcher: Matcher::Literal(Cow::Owned(std::mem::take(literal))),
            quantifier: Quantifier::One,
            capture: None,
        });
    }
}

fn push_replacement_literal(parts: &mut Vec<ReplacementPart<'static>>, literal: &mut String) {
    if !literal.is_empty() {
        parts.push(ReplacementPart::Literal(Cow::Owned(std::mem::take(
            literal,
        ))));
    }
}

fn pattern_error(rule: &str, location: &str, message: impl Into<String>) -> CompileError {
    CompileError::in_rule(rule, format!("{location}: {}", message.into()))
}

fn is_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('a'..='z' | 'A'..='Z' | '_'))
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Class, Rule};

    #[test]
    fn escaped_final_dollar_is_not_an_anchor() {
        let classes = HashMap::new();
        let mut captures = HashMap::new();
        let pattern = parse_pattern(r"\$", &classes, &mut captures, "cash", "target").unwrap();
        assert!(!pattern.anchored_at_end);
        assert!(matches!(&pattern.atoms[0].matcher, Matcher::Literal(s) if s == "$"));
    }

    #[test]
    fn manual_declarations_reject_invalid_names_and_empty_members() {
        let classes = [Class::new("not-a-name", &["x"])];
        let rules = [Rule::new("ok", "x", "y")];
        assert!(RuleSet::new(&classes, &rules).compile().is_err());

        let classes = [Class::new("X", &[""])];
        assert!(RuleSet::new(&classes, &rules).compile().is_err());
    }
}
