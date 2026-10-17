//! A generated grammar's listed words, written grouped by what they are
//! (`Tag => ["#aak", "#aal"]`), and turned when the crate is built into the
//! one array, sorted by word, the grammar looks words up in:
//! `&[(Cow::Borrowed("#aak"), Tag), (Cow::Borrowed("#aal"), Tag), …]`.
//!
//! ```ignore
//! const LISTED: &[(Cow<'static, str>, u8)] = flexion_macros::listed! {
//!     1 => ["#мама"],
//!     0 => ["#папа", "#дядя"],
//! };
//! ```

use proc_macro::{Delimiter, Literal, TokenStream, TokenTree};

/// See the crate's documentation.
#[proc_macro]
pub fn listed(input: TokenStream) -> TokenStream {
    match expand(input) {
        Ok(out) => out,
        Err(message) => format!("compile_error!({message:?})")
            .parse()
            .unwrap_or_default(),
    }
}

fn expand(input: TokenStream) -> Result<TokenStream, String> {
    let mut entries: Vec<(String, Literal, TokenTree)> = Vec::new();
    let mut tokens = input.into_iter().peekable();
    while let Some(value) = tokens.next() {
        for want in ['=', '>'] {
            match tokens.next() {
                Some(TokenTree::Punct(p)) if p.as_char() == want => {}
                _ => return Err(format!("listed!: `=>` after {value}")),
            }
        }
        let Some(TokenTree::Group(words)) = tokens.next() else {
            return Err(format!("listed!: `[words]` after {value} =>"));
        };
        if words.delimiter() != Delimiter::Bracket {
            return Err(format!("listed!: `[words]` after {value} =>"));
        }
        for word in words.stream() {
            // A word passed on by another macro comes in a group of its own.
            let word = match word {
                TokenTree::Group(g) if g.delimiter() == Delimiter::None => {
                    let mut inner = g.stream().into_iter();
                    match (inner.next(), inner.next()) {
                        (Some(only), None) => only,
                        _ => return Err(format!("listed!: a word, not {g}")),
                    }
                }
                other => other,
            };
            match word {
                TokenTree::Literal(l) => {
                    let text = unquote(&l.to_string())?;
                    entries.push((text, l, value.clone()));
                }
                TokenTree::Punct(p) if p.as_char() == ',' => {}
                other => return Err(format!("listed!: a word, not {other}")),
            }
        }
        if let Some(TokenTree::Punct(p)) = tokens.peek()
            && p.as_char() == ','
        {
            tokens.next();
        }
    }
    // As the grammar looks them up: by the word's bytes.
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::from("&[");
    for (_, literal, value) in &entries {
        out.push_str(&format!(
            "(::std::borrow::Cow::Borrowed({literal}), {value}),"
        ));
    }
    out.push(']');
    out.parse().map_err(|e| format!("listed!: {e:?}"))
}

/// The text of a string literal as written (`"a\u{301}"`): its quotes taken
/// off and its escapes read.
fn unquote(literal: &str) -> Result<String, String> {
    let inner = literal
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .ok_or(format!("listed!: a string, not {literal}"))?;
    let mut out = String::new();
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
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('\'') => out.push('\''),
            Some('u') => {
                let code: String = chars.by_ref().skip(1).take_while(|&c| c != '}').collect();
                let c = u32::from_str_radix(&code, 16)
                    .ok()
                    .and_then(char::from_u32)
                    .ok_or(format!("listed!: bad escape in {literal}"))?;
                out.push(c);
            }
            _ => return Err(format!("listed!: an escape it can't read in {literal}")),
        }
    }
    Ok(out)
}
