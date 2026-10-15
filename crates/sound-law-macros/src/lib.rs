//! Sound laws written in [`sound_law`]'s notation and compiled when the crate
//! that writes them is built: [`compiled!`] reads a path to `sound_law` and
//! the body [`sound_law::sound_laws!`] reads, compiles it as
//! [`sound_law::RuleSet::compile`] does, and writes the compiled program, a
//! `Program` constant expression (see `Program::from_rules`). A law that
//! doesn't compile is a compile error, naming it.
//!
//! ```ignore
//! const LAWS: sound_law::Program<'static> = sound_law_macros::compiled! {
//!     sound_law;
//!     classes {}
//!     rules {
//!         law0: "к+ы" => "к+и";
//!     }
//! };
//! ```

use proc_macro::{Delimiter, Group, Ident, Spacing, Span, TokenStream, TokenTree};
use sound_law::{Class, Map, Rule, RuleSet};

/// See the crate's documentation.
#[proc_macro]
pub fn compiled(input: TokenStream) -> TokenStream {
    match expand(input) {
        Ok(out) => out,
        Err(message) => error(&message),
    }
}

/// A `compile_error!` with `message`.
fn error(message: &str) -> TokenStream {
    format!("compile_error!({message:?})")
        .parse()
        .unwrap_or_default()
}

/// An agreement map's entry: the letter, the letter captured before it,
/// and what they become.
type Agreement = (String, String, String);

/// The laws' notation, owned.
#[derive(Default)]
struct Laws {
    classes: Vec<(String, Vec<String>)>,
    maps: Vec<(String, Vec<(String, String)>)>,
    agreements: Vec<(String, Vec<Agreement>)>,
    rules: Vec<Law>,
}

#[derive(Default)]
struct Law {
    name: String,
    options: Vec<String>,
    target: String,
    replacement: String,
    context: Option<(String, String, Vec<String>)>,
    unless: Option<(String, String)>,
}

fn expand(input: TokenStream) -> Result<TokenStream, String> {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let semicolon = tokens
        .iter()
        .position(|t| punct(t, ';'))
        .ok_or("expected the path to `sound_law`, then `;`")?;
    let (path, rest) = tokens.split_at(semicolon);
    let laws = parse(rest.get(1..).unwrap_or(&[]))?;
    let rules = compile(&laws)?;
    let path: TokenStream = path.iter().cloned().collect();
    let mut body = TokenStream::new();
    body.extend([TokenTree::Ident(Ident::new("use", Span::call_site()))]);
    body.extend(path.clone());
    body.extend(parsed("::compiled::*;")?);
    body.extend(path);
    body.extend(parsed("::Program::from_rules")?);
    body.extend([TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        parsed(&rules)?,
    ))]);
    Ok(TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Brace,
        body,
    ))))
}

fn parsed(text: &str) -> Result<TokenStream, String> {
    text.parse().map_err(|e| format!("{e:?}"))
}

/// The laws compiled, as the Rust expression `Program::to_rust` writes.
fn compile(laws: &Laws) -> Result<String, String> {
    let members: Vec<Vec<&str>> = laws
        .classes
        .iter()
        .map(|(_, m)| m.iter().map(String::as_str).collect())
        .collect();
    let classes: Vec<Class<'_>> = laws
        .classes
        .iter()
        .zip(&members)
        .map(|((name, _), m)| Class::new(name, m))
        .collect();
    let pairs: Vec<Vec<(&str, &str)>> = laws
        .maps
        .iter()
        .map(|(_, e)| e.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect())
        .collect();
    let triples: Vec<Vec<(&str, &str, &str)>> = laws
        .agreements
        .iter()
        .map(|(_, e)| {
            e.iter()
                .map(|(a, k, b)| (a.as_str(), k.as_str(), b.as_str()))
                .collect()
        })
        .collect();
    let maps: Vec<Map<'_>> = laws
        .maps
        .iter()
        .zip(&pairs)
        .map(|((name, _), p)| Map::new(name, p))
        .chain(
            laws.agreements
                .iter()
                .zip(&triples)
                .map(|((name, _), t)| Map::keyed(name, t)),
        )
        .collect();
    let ors: Vec<Vec<&str>> = laws
        .rules
        .iter()
        .map(|r| {
            r.context.as_ref().map_or(Vec::new(), |(_, _, or)| {
                or.iter().map(String::as_str).collect()
            })
        })
        .collect();
    let mut rules: Vec<Rule<'_>> = Vec::with_capacity(laws.rules.len());
    for (law, or) in laws.rules.iter().zip(&ors) {
        let mut rule = Rule::new(&law.name, &law.target, &law.replacement);
        if let Some((left, right, _)) = &law.context {
            rule = rule.in_context(left, right).or_before(or);
        }
        if let Some((left, right)) = &law.unless {
            rule = rule.unless_context(left, right);
        }
        for option in &law.options {
            rule = match option.as_str() {
                "rtl" => rule.right_to_left(),
                "once" => rule.once(),
                "iterative" => rule.iterative(),
                other => return Err(format!("{}: unknown option `{other}`", law.name)),
            };
        }
        rules.push(rule);
    }
    let program = RuleSet::new_with_maps(&classes, &maps, &rules)
        .compile()
        .map_err(|e| e.to_string())?;
    Ok(program.to_rust())
}

fn punct(t: &TokenTree, c: char) -> bool {
    matches!(t, TokenTree::Punct(p) if p.as_char() == c)
}

/// `=>` at `at`.
fn arrow(tokens: &[TokenTree], at: usize) -> bool {
    matches!(
        (tokens.get(at), tokens.get(at + 1)),
        (Some(TokenTree::Punct(a)), Some(TokenTree::Punct(b)))
            if a.as_char() == '=' && a.spacing() == Spacing::Joint && b.as_char() == '>'
    )
}

fn word(t: Option<&TokenTree>) -> Option<String> {
    match t? {
        TokenTree::Ident(i) => Some(i.to_string()),
        _ => None,
    }
}

/// The text of a string literal token.
fn text(t: Option<&TokenTree>) -> Result<String, String> {
    let Some(TokenTree::Literal(l)) = t else {
        return Err(format!("expected a string, found {t:?}"));
    };
    unescape(&l.to_string())
}

/// A Rust string literal's text: `"…"` with its escapes, or `r#"…"#`.
fn unescape(literal: &str) -> Result<String, String> {
    if let Some(raw) = literal.strip_prefix('r') {
        let hashes = raw.chars().take_while(|&c| c == '#').count();
        let fence = "#".repeat(hashes);
        return raw
            .strip_prefix(&format!("{fence}\""))
            .and_then(|r| r.strip_suffix(&format!("\"{fence}")))
            .map(str::to_string)
            .ok_or(format!("not a string: {literal}"));
    }
    let inner = literal
        .strip_prefix('"')
        .and_then(|l| l.strip_suffix('"'))
        .ok_or(format!("not a string: {literal}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('0') => out.push('\0'),
            Some('u') => {
                let hex: String = chars
                    .by_ref()
                    .skip_while(|&c| c == '{')
                    .take_while(|&c| c != '}')
                    .collect();
                let c = u32::from_str_radix(&hex, 16)
                    .ok()
                    .and_then(char::from_u32)
                    .ok_or(format!("bad escape in {literal}"))?;
                out.push(c);
            }
            Some('x') => {
                let hex: String = chars.by_ref().take(2).collect();
                let c =
                    u8::from_str_radix(&hex, 16).map_err(|_| format!("bad escape in {literal}"))?;
                out.push(char::from(c));
            }
            Some(c) => out.push(c),
            None => return Err(format!("bad escape in {literal}")),
        }
    }
    Ok(out)
}

/// The sections: `classes { … }`, then optionally `maps { … }` and
/// `agreements { … }`, then `rules { … }`.
fn parse(tokens: &[TokenTree]) -> Result<Laws, String> {
    let mut laws = Laws::default();
    let mut at = 0;
    while let Some(section) = word(tokens.get(at)) {
        let Some(TokenTree::Group(body)) = tokens.get(at + 1) else {
            return Err(format!("expected `{section} {{ … }}`"));
        };
        let body: Vec<TokenTree> = body.stream().into_iter().collect();
        match section.as_str() {
            "classes" => laws.classes = entries(&body, class)?,
            "maps" => laws.maps = entries(&body, map)?,
            "agreements" => laws.agreements = entries(&body, agreement)?,
            "rules" => laws.rules = rules(&body)?,
            other => return Err(format!("unknown section `{other}`")),
        }
        at += 2;
    }
    if let Some(t) = tokens.get(at) {
        return Err(format!("unexpected {t}"));
    }
    Ok(laws)
}

/// `NAME = [ … ];` entries, each list read by `item` from its bracket.
fn entries<T>(
    body: &[TokenTree],
    item: fn(&[TokenTree]) -> Result<T, String>,
) -> Result<Vec<(String, T)>, String> {
    let mut out = Vec::new();
    for entry in body.split(|t| punct(t, ';')).filter(|e| !e.is_empty()) {
        let name = word(entry.first()).ok_or("expected a name")?;
        let (Some(eq), Some(TokenTree::Group(list))) = (entry.get(1), entry.get(2)) else {
            return Err(format!("{name}: expected `= [ … ]`"));
        };
        if !punct(eq, '=') {
            return Err(format!("{name}: expected `=`"));
        }
        let list: Vec<TokenTree> = list.stream().into_iter().collect();
        out.push((name, item(&list)?));
    }
    Ok(out)
}

fn class(list: &[TokenTree]) -> Result<Vec<String>, String> {
    list.split(|t| punct(t, ','))
        .filter(|m| !m.is_empty())
        .map(|m| text(m.first()))
        .collect()
}

fn map(list: &[TokenTree]) -> Result<Vec<(String, String)>, String> {
    list.split(|t| punct(t, ','))
        .filter(|e| !e.is_empty())
        .map(|e| {
            if !arrow(e, 1) {
                return Err("expected `\"from\" => \"to\"`".to_string());
            }
            Ok((text(e.first())?, text(e.get(3))?))
        })
        .collect()
}

fn agreement(list: &[TokenTree]) -> Result<Vec<Agreement>, String> {
    // Entries are `("from", "key") => "to"`: commas inside the parentheses
    // are a group's own.
    list.split(|t| punct(t, ','))
        .filter(|e| !e.is_empty())
        .map(|e| {
            let Some(TokenTree::Group(pair)) = e.first() else {
                return Err("expected `(\"from\", \"key\") => \"to\"`".to_string());
            };
            let pair: Vec<TokenTree> = pair.stream().into_iter().collect();
            if !arrow(e, 1) {
                return Err("expected `=>`".to_string());
            }
            Ok((text(pair.first())?, text(pair.get(2))?, text(e.get(3))?))
        })
        .collect()
}

/// `name[options]: "target" => "replacement" / "left" _ "right" | "or" …
/// unless "left" _ "right";`, each.
fn rules(body: &[TokenTree]) -> Result<Vec<Law>, String> {
    let mut out = Vec::new();
    for r in body.split(|t| punct(t, ';')).filter(|r| !r.is_empty()) {
        let mut law = Law {
            name: word(r.first()).ok_or("expected a rule's name")?,
            ..Law::default()
        };
        let mut at = 1;
        if let Some(TokenTree::Group(g)) = r.get(at)
            && g.delimiter() == Delimiter::Bracket
        {
            law.options = g
                .stream()
                .into_iter()
                .filter_map(|t| word(Some(&t)))
                .collect();
            at += 1;
        }
        if !r.get(at).is_some_and(|t| punct(t, ':')) {
            return Err(format!("{}: expected `:`", law.name));
        }
        law.target = text(r.get(at + 1))?;
        if !arrow(r, at + 2) {
            return Err(format!("{}: expected `=>`", law.name));
        }
        law.replacement = text(r.get(at + 4))?;
        at += 5;
        if r.get(at).is_some_and(|t| punct(t, '/')) {
            let left = text(r.get(at + 1))?;
            let right = text(r.get(at + 3))?;
            at += 4;
            let mut or = Vec::new();
            while r.get(at).is_some_and(|t| punct(t, '|')) {
                or.push(text(r.get(at + 1))?);
                at += 2;
            }
            law.context = Some((left, right, or));
        }
        if word(r.get(at)).as_deref() == Some("unless") {
            law.unless = Some((text(r.get(at + 1))?, text(r.get(at + 3))?));
            at += 4;
        }
        if let Some(t) = r.get(at) {
            return Err(format!("{}: unexpected {t}", law.name));
        }
        out.push(law);
    }
    Ok(out)
}
