//! A small, dependency-free engine for writing and applying sound laws.
//!
//! [`sound_laws!`] declares both the segment classes and an ordered rule list.
//! The same notation handles simple letter maps, contextual rewrites, deletion,
//! insertion, assimilation, capture, and metathesis:
//!
//! ```
//! use sound_law::{RuleSet, sound_laws};
//!
//! const LAWS: RuleSet<'static> = sound_laws! {
//!     classes {
//!         V = ["a", "e", "i", "o", "u"];
//!         C = ["p", "t", "k", "s"];
//!         K = ["k"];
//!     }
//!     maps {
//!         PALATAL = ["k" => "tʲ"];
//!     }
//!     rules {
//!         // A plain letter map and a contextual rewrite use the same syntax.
//!         palatalize_k: "{c:K}" => "{c|PALATAL}" / "" _ "{V}";
//!         // Capture a consonant and vowel, then emit them in reverse order.
//!         metathesis: "{c:C}{v:V}" => "{v}{c}";
//!     }
//! };
//!
//! let program = LAWS.compile()?;
//! assert_eq!(program.apply("pa"), "ap");
//! # Ok::<(), sound_law::CompileError>(())
//! ```
//!
//! # Pattern notation
//!
//! Patterns are strings, keeping the macro grammar compact and allowing rules
//! to be assembled as ordinary const data.
//!
//! - literal text matches itself;
//! - `{V}` matches one member of class `V`;
//! - `{v:V}` captures one member of `V` as `v`;
//! - `{[a|e|ai]}` matches one inline literal alternative, longest first;
//! - `{v:[a|e|ai]}` captures an inline alternative;
//! - `{C|V}`, `{C&SONORANT}`, and `{C-[p|t]}` form finite class unions,
//!   intersections, and differences;
//! - `{x:.}` captures any one Unicode scalar value;
//! - `{cluster:C+}`, `{maybe:C?}`, and `{run:C*}` use greedy quantifiers with
//!   backtracking and capture the complete matched span;
//! - `{x:!V}` matches and captures one scalar value that does not begin a member
//!   of `V`;
//! - `{?!V}` matches nowhere but where no member of `V` begins (it reads
//!   ahead, consuming nothing: `"о" => "е" / "{[ш|ж]}" _ "{?![́]}"` changes an
//!   unstressed vowel, one no stress mark follows, at the word's end too);
//! - `{=v}` matches the exact text captured earlier as `v`;
//! - `^` and `$` anchor a pattern to the start and end of the complete form;
//! - `#` matches a zero-width edge of a whitespace-delimited word.
//!
//! Replacements contain literal text and capture references such as `{v}`.
//! `{v|MAP}` emits a capture through a declared correspondence map. Captures
//! may be omitted, duplicated, transformed, or emitted in any order. A
//! backslash escapes a metacharacter in either a pattern or replacement. An
//! empty target inserts at every boundary satisfying its context:
//! `"" => "ə" / "{C}" _ "{C}"`.
//!
//! `{v|MAP[k]}` emits it through an agreement map, keyed by capture `k` too.
//!
//! Rules apply globally and left-to-right by default, to the result of the
//! preceding rule. `[rtl]` changes overlap selection and `[once]` selects only
//! the first directional match. Matches made by one rule are simultaneous: its
//! own output is not reconsidered until another pass is explicitly requested
//! with [`Program::apply_until_stable`], unless the rule is `[iterative]`:
//! then each match is written before the next is sought, and the context
//! behind it reads what the rule wrote (harmony through stacked suffixes):
//!
//! ```
//! use sound_law::{RuleSet, sound_laws};
//!
//! // `A` is `a` after a back vowel, `e` after a front one or none.
//! const HARMONY: RuleSet<'static> = sound_laws! {
//!     classes {
//!         V = ["a", "e", "o", "i"];
//!     }
//!     agreements {
//!         AGREE = [
//!             ("A", "a") => "a", ("A", "o") => "a",
//!             ("A", "e") => "e", ("A", "i") => "e", ("A", "") => "e",
//!         ];
//!     }
//!     rules {
//!         harmony[iterative]: "{x:[A]}" => "{x|AGREE[t]}" / "{t:V?}{gap:!V*}" _ "";
//!     }
//! };
//! assert_eq!(HARMONY.apply("kotlAndA")?, "kotlanda");
//! assert_eq!(HARMONY.apply("ellAndA")?, "ellende");
//! assert_eq!(HARMONY.apply("srbA")?, "srbe");
//! # Ok::<(), sound_law::CompileError>(())
//! ```

#![forbid(unsafe_code)]

mod compile;
mod engine;
mod rust;

use std::fmt;

pub use compile::{Agreement, Atom, CompiledRule, Matcher, Pattern, Quantifier, ReplacementPart};
pub use engine::{
    IterationError, MatchLimitError, Program, ProgramCompositionError, RuleRangeError, TraceStep,
};

/// What a program written by [`Program::to_rust`] names: `use
/// sound_law::compiled::*;` before it. `B` is [`Cow::Borrowed`](std::borrow::Cow).
pub mod compiled {
    pub use crate::{
        Application, Atom, CompiledRule, Direction, Matcher, Pattern, Program, Quantifier,
        ReplacementPart,
    };
    pub use std::borrow::Cow::Borrowed as B;
}

/// The direction in which a rule selects non-overlapping matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    /// Select the leftmost remaining match first.
    #[default]
    LeftToRight,
    /// Select the rightmost remaining match first.
    RightToLeft,
}

/// How many matches one rule applies to in a word, and what their contexts
/// read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Application {
    /// Apply to every non-overlapping match selected in the rule's direction,
    /// all of them found in the rule's input.
    #[default]
    All,
    /// Apply only to the first match selected in the rule's direction.
    Once,
    /// Apply to every match in the rule's direction, one after another: each
    /// is written before the next is sought, so the context on the side
    /// already passed reads what the rule wrote there (its output), and the
    /// other side its input. Harmony runs through a chain of suffixes this
    /// way, each vowel agreeing with the one before it as written.
    Iterative,
}

/// A named set of possible segments.
///
/// Members may contain more than one Unicode scalar value. When multiple
/// members match at one position, the engine tries the longest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Class<'a> {
    name: &'a str,
    members: &'a [&'a str],
}

impl<'a> Class<'a> {
    /// Construct a class declaration.
    pub const fn new(name: &'a str, members: &'a [&'a str]) -> Self {
        Self { name, members }
    }

    /// The name used to reference this class from patterns.
    pub const fn name(&self) -> &'a str {
        self.name
    }

    /// The class's possible segments.
    pub const fn members(&self) -> &'a [&'a str] {
        self.members
    }
}

/// A named, source-keyed correspondence used by mapped capture replacements.
///
/// A pattern captures a source member normally, then `{capture|MAP}` emits the
/// corresponding output. Compilation rejects maps that do not cover every
/// possible value of that capture.
///
/// An agreement map ([`Map::keyed`]) is keyed by a second capture too:
/// `{capture|MAP[key]}` emits the output for both values (a suffix vowel
/// by the vowel before it). A key may be empty: what an optional capture
/// that matched nothing gets (a word with no vowel before the suffix).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Map<'a> {
    name: &'a str,
    entries: &'a [(&'a str, &'a str)],
    keyed: &'a [(&'a str, &'a str, &'a str)],
}

impl<'a> Map<'a> {
    /// Construct a correspondence map.
    pub const fn new(name: &'a str, entries: &'a [(&'a str, &'a str)]) -> Self {
        Self {
            name,
            entries,
            keyed: &[],
        }
    }

    /// Construct an agreement map: `(source, key, output)` entries.
    pub const fn keyed(name: &'a str, keyed: &'a [(&'a str, &'a str, &'a str)]) -> Self {
        Self {
            name,
            entries: &[],
            keyed,
        }
    }

    /// The name used after `|` in replacement capture references.
    pub const fn name(&self) -> &'a str {
        self.name
    }

    /// The source-to-output correspondence entries.
    pub const fn entries(&self) -> &'a [(&'a str, &'a str)] {
        self.entries
    }

    /// An agreement map's `(source, key, output)` entries.
    pub const fn keyed_entries(&self) -> &'a [(&'a str, &'a str, &'a str)] {
        self.keyed
    }
}

/// One declarative sound law.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule<'a> {
    name: &'a str,
    target: &'a str,
    replacement: &'a str,
    left: &'a str,
    right: &'a str,
    or_right: &'a [&'a str],
    excluded_left: Option<&'a str>,
    excluded_right: Option<&'a str>,
    direction: Direction,
    application: Application,
}

impl<'a> Rule<'a> {
    /// Construct an unconditional `target → replacement` rule.
    pub const fn new(name: &'a str, target: &'a str, replacement: &'a str) -> Self {
        Self {
            name,
            target,
            replacement,
            left: "",
            right: "",
            or_right: &[],
            excluded_left: None,
            excluded_right: None,
            direction: Direction::LeftToRight,
            application: Application::All,
        }
    }

    /// Restrict this rule to `left _ right`.
    pub const fn in_context(mut self, left: &'a str, right: &'a str) -> Self {
        self.left = left;
        self.right = right;
        self
    }

    /// Let this rule apply before any of `rights` too, where its right
    /// context isn't there: one rule in several environments, its target
    /// sought once. They declare no captures.
    ///
    /// ```
    /// use sound_law::{RuleSet, sound_laws};
    ///
    /// // A consonant weakens before a closed syllable's ends.
    /// const LAWS: RuleSet<'static> = sound_laws! {
    ///     classes {
    ///         V = ["a", "o", "u"];
    ///     }
    ///     rules {
    ///         weak: "t" => "d" / "{V}" _ "{V}n$" | "{V}ss";
    ///     }
    /// };
    /// assert_eq!(LAWS.apply("katun")?, "kadun");
    /// assert_eq!(LAWS.apply("katussa")?, "kadussa");
    /// assert_eq!(LAWS.apply("katua")?, "katua");
    /// # Ok::<(), sound_law::CompileError>(())
    /// ```
    pub const fn or_before(mut self, rights: &'a [&'a str]) -> Self {
        self.or_right = rights;
        self
    }

    /// Prevent this rule in the `left _ right` environment.
    ///
    /// The exclusion is checked after the positive environment. Captures from
    /// the positive environment and target may be used as backreferences, but
    /// exclusions cannot declare new captures.
    pub const fn unless_context(mut self, left: &'a str, right: &'a str) -> Self {
        self.excluded_left = Some(left);
        self.excluded_right = Some(right);
        self
    }

    /// Select non-overlapping matches from the right edge toward the left.
    pub const fn right_to_left(mut self) -> Self {
        self.direction = Direction::RightToLeft;
        self
    }

    /// Apply only to the first match selected in this rule's direction.
    pub const fn once(mut self) -> Self {
        self.application = Application::Once;
        self
    }

    /// Apply to each match in turn, its context on the side already passed
    /// read on the rule's output (see [`Application::Iterative`]).
    pub const fn iterative(mut self) -> Self {
        self.application = Application::Iterative;
        self
    }

    /// A stable name used by traces and diagnostics.
    pub const fn name(&self) -> &'a str {
        self.name
    }

    /// The pattern replaced by this rule.
    pub const fn target(&self) -> &'a str {
        self.target
    }

    /// The replacement template emitted by this rule.
    pub const fn replacement(&self) -> &'a str {
        self.replacement
    }

    /// The immediately preceding context pattern, or `""` when unrestricted.
    pub const fn left_context(&self) -> &'a str {
        self.left
    }

    /// The immediately following context pattern, or `""` when unrestricted.
    pub const fn right_context(&self) -> &'a str {
        self.right
    }

    /// The right contexts it may have instead (see [`Rule::or_before`]).
    pub const fn or_right_contexts(&self) -> &'a [&'a str] {
        self.or_right
    }

    /// The excluded preceding context, if one was declared.
    pub const fn excluded_left_context(&self) -> Option<&'a str> {
        self.excluded_left
    }

    /// The excluded following context, if one was declared.
    pub const fn excluded_right_context(&self) -> Option<&'a str> {
        self.excluded_right
    }

    /// The match-selection direction.
    pub const fn direction(&self) -> Direction {
        self.direction
    }

    /// Whether this rule applies globally or once.
    pub const fn application(&self) -> Application {
        self.application
    }
}

/// An uncompiled collection of classes and ordered rules.
///
/// This type is const-friendly, so a [`sound_laws!`] declaration can live in a
/// `const` or `static`. Call [`RuleSet::compile`] once before applying it many
/// times.
#[derive(Debug, Clone, Copy)]
pub struct RuleSet<'a> {
    classes: &'a [Class<'a>],
    maps: &'a [Map<'a>],
    rules: &'a [Rule<'a>],
}

impl<'a> RuleSet<'a> {
    /// Construct a rule set from declaration slices.
    pub const fn new(classes: &'a [Class<'a>], rules: &'a [Rule<'a>]) -> Self {
        Self {
            classes,
            maps: &[],
            rules,
        }
    }

    /// Construct a rule set with correspondence maps.
    pub const fn new_with_maps(
        classes: &'a [Class<'a>],
        maps: &'a [Map<'a>],
        rules: &'a [Rule<'a>],
    ) -> Self {
        Self {
            classes,
            maps,
            rules,
        }
    }

    /// Access the class declarations.
    pub const fn classes(&self) -> &'a [Class<'a>] {
        self.classes
    }

    /// Access the correspondence-map declarations.
    pub const fn maps(&self) -> &'a [Map<'a>] {
        self.maps
    }

    /// Access the ordered rule declarations.
    pub const fn rules(&self) -> &'a [Rule<'a>] {
        self.rules
    }

    /// Validate and compile the string notation into an executable program.
    pub fn compile(&self) -> Result<Program<'static>, CompileError> {
        compile::compile(self)
    }

    /// Compile and apply the complete rule set once.
    ///
    /// Prefer retaining a [`Program`] when applying the same laws repeatedly.
    pub fn apply(&self, input: &str) -> Result<String, CompileError> {
        Ok(self.compile()?.apply(input))
    }
}

/// A problem in a class, pattern, backreference, or replacement template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileError {
    rule: Option<String>,
    message: String,
}

impl CompileError {
    pub(crate) fn global(message: impl Into<String>) -> Self {
        Self {
            rule: None,
            message: message.into(),
        }
    }

    pub(crate) fn in_rule(rule: &str, message: impl Into<String>) -> Self {
        Self {
            rule: Some(rule.to_owned()),
            message: message.into(),
        }
    }

    /// The rule that failed to compile, when the error belongs to one rule.
    pub fn rule(&self) -> Option<&str> {
        self.rule.as_deref()
    }

    /// The diagnostic without its optional rule-name prefix.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.rule {
            Some(rule) => write!(f, "sound law `{rule}`: {}", self.message),
            None => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for CompileError {}

/// Declare segment classes and ordered sound laws in one expression.
///
/// Every rule has the shape `name: "target" => "replacement";`. Add
/// `/ "left" _ "right"` before the semicolon to constrain its environment
/// (`_ "right" | "other"`: before either). An
/// empty target denotes insertion at a boundary rather than replacement. Add
/// `unless "left" _ "right"` to exclude an environment. Rule options `[rtl]`,
/// `[once]`, or `[rtl, once]` control match selection; `[iterative]` makes a
/// rule read its own output on the side it has passed. An `agreements`
/// block after `maps` declares maps keyed by a second capture
/// (`V = [("A", "a") => "a", ("A", "") => "e"];`, used as `{x|V[t]}`).
#[macro_export]
macro_rules! sound_laws {
    (
        classes {
            $( $class:ident = [ $( $member:literal ),+ $(,)? ]; )*
        }
        $(
            maps {
                $( $map:ident = [ $( $map_from:literal => $map_to:literal ),+ $(,)? ]; )*
            }
        )?
        $(
            agreements {
                $( $keyed:ident = [
                    $( ( $keyed_from:literal, $keyed_key:literal ) => $keyed_to:literal ),+ $(,)?
                ]; )*
            }
        )?
        rules {
            $(
                $rule:ident $( [ $( $option:ident ),+ $(,)? ] )?
                : $target:literal => $replacement:literal
                $( / $left:literal _ $right:literal $( | $or:literal )* )?
                $( unless $excluded_left:literal _ $excluded_right:literal )? ;
            )*
        }
    ) => {
        $crate::RuleSet::new_with_maps(
            &[
                $(
                    $crate::Class::new(stringify!($class), &[ $( $member ),+ ])
                ),*
            ],
            &[
                $($(
                    $crate::Map::new(
                        stringify!($map),
                        &[ $( ($map_from, $map_to) ),+ ],
                    ),
                )*)?
                $($(
                    $crate::Map::keyed(
                        stringify!($keyed),
                        &[ $( ($keyed_from, $keyed_key, $keyed_to) ),+ ],
                    ),
                )*)?
            ],
            &[
                $(
                    $crate::sound_laws!(
                        @options
                        $crate::Rule::new(stringify!($rule), $target, $replacement)
                            $( .in_context($left, $right).or_before(&[ $( $or ),* ]) )?
                            $( .unless_context($excluded_left, $excluded_right) )?
                        ; $( $( $option ),+ )?
                    )
                ),*
            ],
        )
    };
    (@options $rule:expr; ) => { $rule };
    (@options $rule:expr; rtl $(, $rest:ident)*) => {
        $crate::sound_laws!(@options $rule.right_to_left(); $( $rest ),*)
    };
    (@options $rule:expr; once $(, $rest:ident)*) => {
        $crate::sound_laws!(@options $rule.once(); $( $rest ),*)
    };
    (@options $rule:expr; iterative $(, $rest:ident)*) => {
        $crate::sound_laws!(@options $rule.iterative(); $( $rest ),*)
    };
    (@options $rule:expr; $unknown:ident $(, $rest:ident)*) => {
        compile_error!(concat!("unknown sound-law rule option `", stringify!($unknown), "`"))
    };
}
