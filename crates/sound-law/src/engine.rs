use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::{Range, RangeInclusive};

use crate::compile::{Atom, CompiledRule, Matcher, Pattern, Quantifier, ReplacementPart};
use crate::{Application, Direction};

/// A validated, executable sequence of sound laws.
///
/// [`RuleSet::compile`](crate::RuleSet::compile) makes one at run time;
/// [`Program::from_rules`] makes one as static data, from rules
/// [`Program::to_rust`] wrote, with no compile step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'a> {
    rules: Cow<'a, [CompiledRule<'a>]>,
}

impl<'a> Program<'a> {
    /// A program of already compiled rules, usable in a `static`: the rules
    /// as [`Program::to_rust`] writes them.
    ///
    /// ```
    /// use sound_law::compiled::*;
    ///
    /// const RULES: &[CompiledRule<'static>] = &[
    ///     // drop_final[once]: "a" => "" / "" _ "$"
    ///     CompiledRule {
    ///         name: B("drop_final"),
    ///         left: Pattern { atoms: B(&[]), anchored_at_start: false, anchored_at_end: false },
    ///         target: Pattern {
    ///             atoms: B(&[Atom {
    ///                 matcher: Matcher::Literal(B("a")),
    ///                 quantifier: Quantifier::One,
    ///                 capture: None,
    ///             }]),
    ///             anchored_at_start: false,
    ///             anchored_at_end: false,
    ///         },
    ///         right: Pattern { atoms: B(&[]), anchored_at_start: false, anchored_at_end: true },
    ///         excluded_left: None,
    ///         excluded_right: None,
    ///         replacement: B(&[]),
    ///         capture_count: 0,
    ///         direction: Direction::LeftToRight,
    ///         application: Application::Once,
    ///         required: B(&[B("a")]),
    ///     },
    /// ];
    /// static PROGRAM: Program<'static> = Program::from_rules(RULES);
    ///
    /// assert_eq!(PROGRAM.apply("papa"), "pap");
    /// ```
    pub const fn from_rules(rules: &'a [CompiledRule<'a>]) -> Self {
        Self {
            rules: Cow::Borrowed(rules),
        }
    }

    pub(crate) fn new(rules: Vec<CompiledRule<'a>>) -> Self {
        Self {
            rules: Cow::Owned(rules),
        }
    }

    /// The compiled rules, in order.
    pub fn rules(&self) -> &[CompiledRule<'a>] {
        &self.rules
    }

    /// The number of ordered rules in this program.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Iterate over stable rule names in declaration order.
    pub fn rule_names(&self) -> impl ExactSizeIterator<Item = &str> {
        self.rules.iter().map(|rule| rule.name.as_ref())
    }

    /// Compose two compiled programs, applying `self` before `next`.
    ///
    /// Stable rule names must remain unique so traces and named ranges are
    /// unambiguous.
    pub fn then(&self, next: &Program<'a>) -> Result<Program<'a>, ProgramCompositionError> {
        let mut names: HashSet<&str> = self.rule_names().collect();
        if let Some(duplicate) = next.rule_names().find(|name| !names.insert(name)) {
            return Err(ProgramCompositionError {
                duplicate_rule: duplicate.to_owned(),
            });
        }
        let mut rules = Vec::with_capacity(self.rules.len() + next.rules.len());
        rules.extend(self.rules.iter().cloned());
        rules.extend(next.rules.iter().cloned());
        Ok(Program::new(rules))
    }

    /// Apply every rule once, in declaration order.
    pub fn apply(&self, input: &str) -> String {
        apply_rules(&self.rules, input)
    }

    /// Apply every rule with a deterministic limit on matcher search work.
    ///
    /// One work unit is charged for each recursive pattern state and repeated
    /// matcher state examined. This is an algorithmic guard, not wall-clock
    /// timing, so a given program and input fail reproducibly.
    pub fn apply_with_limit(
        &self,
        input: &str,
        max_steps: usize,
    ) -> Result<String, MatchLimitError> {
        let mut budget = WorkBudget::limited(max_steps);
        apply_rules_with_budget(&self.rules, input, &mut budget)
    }

    /// Apply an end-exclusive range of rules by declaration index.
    pub fn apply_range(&self, input: &str, range: Range<usize>) -> Result<String, RuleRangeError> {
        if range.start > range.end || range.end > self.rules.len() {
            return Err(RuleRangeError::OutOfBounds {
                start: range.start,
                end: range.end,
                rule_count: self.rules.len(),
            });
        }
        Ok(apply_rules(
            self.rules.get(range).unwrap_or_default(),
            input,
        ))
    }

    /// Apply the inclusive range from `first` through `last` by rule name.
    pub fn apply_named_range(
        &self,
        input: &str,
        first: &str,
        last: &str,
    ) -> Result<String, RuleRangeError> {
        let start = self.rule_index(first)?;
        let end = self.rule_index(last)?;
        if start > end {
            return Err(RuleRangeError::Reversed {
                first: first.to_owned(),
                last: last.to_owned(),
            });
        }
        Ok(apply_rules(
            self.rules.get(start..=end).unwrap_or_default(),
            input,
        ))
    }

    /// Apply rules from the beginning through `last`, inclusively.
    pub fn apply_through(&self, input: &str, last: &str) -> Result<String, RuleRangeError> {
        let end = self.rule_index(last)?;
        Ok(apply_rules(
            self.rules.get(..=end).unwrap_or_default(),
            input,
        ))
    }

    /// Apply rules from `first` through the end of the program.
    pub fn apply_from(&self, input: &str, first: &str) -> Result<String, RuleRangeError> {
        let start = self.rule_index(first)?;
        Ok(apply_rules(
            self.rules.get(start..).unwrap_or_default(),
            input,
        ))
    }

    /// Apply every rule while recording only the rules that changed the word.
    pub fn apply_with_trace(&self, input: &str) -> (String, Vec<TraceStep>) {
        let mut word = input.to_owned();
        let mut trace = Vec::new();
        let mut budget = WorkBudget::unlimited();
        let mut search = Search::new(&mut budget);

        for rule in self.rules.iter() {
            if let Some(next) = apply_rule(rule, &word, &mut search)
                && next != word
            {
                trace.push(TraceStep {
                    rule: rule.name.to_string(),
                    before: word,
                    after: next.clone(),
                });
                word = next;
            }
        }

        (word, trace)
    }

    /// Reapply the complete ordered program until it stops changing the word.
    ///
    /// Cycles are detected, and `max_passes` is a hard guard against programs
    /// that keep producing novel forms forever.
    pub fn apply_until_stable(
        &self,
        input: &str,
        max_passes: usize,
    ) -> Result<String, IterationError> {
        let mut seen = HashMap::new();
        let mut word = input.to_owned();
        seen.insert(word.clone(), 0usize);

        for pass in 1..=max_passes {
            let next = self.apply(&word);
            if next == word {
                return Ok(word);
            }
            if let Some(first_seen) = seen.insert(next.clone(), pass) {
                return Err(IterationError::Cycle {
                    form: next,
                    first_seen,
                    repeated_at: pass,
                });
            }
            word = next;
        }

        Err(IterationError::Limit {
            max_passes,
            last_form: word,
        })
    }

    fn rule_index(&self, name: &str) -> Result<usize, RuleRangeError> {
        self.rules
            .iter()
            .position(|rule| rule.name == name)
            .ok_or_else(|| RuleRangeError::UnknownRule(name.to_owned()))
    }
}

fn apply_rules(rules: &[CompiledRule], input: &str) -> String {
    let mut budget = WorkBudget::unlimited();
    // An unlimited budget is never exhausted; the input unchanged otherwise.
    apply_rules_with_budget(rules, input, &mut budget).unwrap_or_else(|_| input.to_owned())
}

fn apply_rules_with_budget(
    rules: &[CompiledRule],
    input: &str,
    budget: &mut WorkBudget,
) -> Result<String, MatchLimitError> {
    let mut word = input.to_owned();
    let mut search = Search::new(budget);
    for rule in rules {
        if let Some(next) = apply_rule(rule, &word, &mut search) {
            word = next;
        }
        if search.budget.exhausted {
            return Err(MatchLimitError {
                max_steps: search.budget.max_steps,
                rule: rule.name.to_string(),
            });
        }
    }
    Ok(word)
}

/// One changed intermediate form from [`Program::apply_with_trace`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceStep {
    /// The declarative rule name.
    pub rule: String,
    /// The word immediately before this rule.
    pub before: String,
    /// The word immediately after this rule.
    pub after: String,
}

/// Failure to reach a fixed point in [`Program::apply_until_stable`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IterationError {
    /// A form already seen during this run appeared again.
    Cycle {
        /// The repeated form.
        form: String,
        /// The zero-based pass where the form first appeared.
        first_seen: usize,
        /// The pass where it appeared again.
        repeated_at: usize,
    },
    /// The program was still changing after the requested number of passes.
    Limit {
        /// The configured pass limit.
        max_passes: usize,
        /// The last form produced before stopping.
        last_form: String,
    },
}

impl fmt::Display for IterationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cycle {
                form,
                first_seen,
                repeated_at,
            } => write!(
                f,
                "sound-law iteration cycled on `{form}` (passes {first_seen} and {repeated_at})"
            ),
            Self::Limit {
                max_passes,
                last_form,
            } => write!(
                f,
                "sound-law iteration did not stabilize in {max_passes} passes; last form was `{last_form}`"
            ),
        }
    }
}

impl std::error::Error for IterationError {}

/// An invalid index or name passed to a partial-execution API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleRangeError {
    /// No compiled rule has this name.
    UnknownRule(String),
    /// The named first rule occurs after the named last rule.
    Reversed {
        /// Requested first rule.
        first: String,
        /// Requested last rule.
        last: String,
    },
    /// An index range was reversed or extended beyond the program.
    OutOfBounds {
        /// Inclusive start index.
        start: usize,
        /// Exclusive end index.
        end: usize,
        /// Number of compiled rules.
        rule_count: usize,
    },
}

impl fmt::Display for RuleRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRule(rule) => write!(f, "unknown sound-law rule `{rule}`"),
            Self::Reversed { first, last } => write!(
                f,
                "sound-law rule range is reversed: `{first}` occurs after `{last}`"
            ),
            Self::OutOfBounds {
                start,
                end,
                rule_count,
            } => write!(
                f,
                "sound-law rule range {start}..{end} is invalid for {rule_count} rules"
            ),
        }
    }
}

impl std::error::Error for RuleRangeError {}

/// Two compiled programs cannot be composed without ambiguous rule names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramCompositionError {
    duplicate_rule: String,
}

impl ProgramCompositionError {
    /// The rule name present in both programs.
    pub fn duplicate_rule(&self) -> &str {
        &self.duplicate_rule
    }
}

impl fmt::Display for ProgramCompositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cannot compose sound-law programs with duplicate rule `{}`",
            self.duplicate_rule
        )
    }
}

impl std::error::Error for ProgramCompositionError {}

/// Matcher search exceeded the caller's deterministic work budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchLimitError {
    max_steps: usize,
    rule: String,
}

impl MatchLimitError {
    /// The configured maximum number of matcher work units.
    pub fn max_steps(&self) -> usize {
        self.max_steps
    }

    /// The rule being evaluated when the budget was exhausted.
    pub fn rule(&self) -> &str {
        &self.rule
    }
}

impl fmt::Display for MatchLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sound law `{}` exceeded its matcher work limit of {} steps",
            self.rule, self.max_steps
        )
    }
}

impl std::error::Error for MatchLimitError {}

#[derive(Debug)]
struct WorkBudget {
    remaining: Option<usize>,
    max_steps: usize,
    exhausted: bool,
}

impl WorkBudget {
    fn unlimited() -> Self {
        Self {
            remaining: None,
            max_steps: usize::MAX,
            exhausted: false,
        }
    }

    fn limited(max_steps: usize) -> Self {
        Self {
            remaining: Some(max_steps),
            max_steps,
            exhausted: false,
        }
    }

    fn spend(&mut self) -> bool {
        match self.remaining {
            None => true,
            Some(0) => {
                self.exhausted = true;
                false
            }
            Some(remaining) => {
                self.remaining = Some(remaining - 1);
                true
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Span {
    start: usize,
    end: usize,
}

struct FoundMatch {
    start: usize,
    end: usize,
    captures: Vec<Option<Span>>,
}

/// What a search changes as it goes: the captures so far, the ends the
/// atoms being matched have yet to try (a stack, one run per atom), and the
/// budget. One serves every rule of a program, so matching allocates nothing
/// once it has grown.
struct Search<'b> {
    captures: Vec<Option<Span>>,
    ends: Vec<usize>,
    budget: &'b mut WorkBudget,
}

impl<'b> Search<'b> {
    fn new(budget: &'b mut WorkBudget) -> Self {
        Self {
            captures: Vec::new(),
            ends: Vec::new(),
            budget,
        }
    }
}

/// What follows a match up to a position; `true` ends the search.
type Next<'k> = dyn FnMut(&mut Search<'_>, usize) -> bool + 'k;

/// The word after one rule, or `None` where the rule doesn't match: a rule
/// costs nothing on a word without its required literals (in flexion, a
/// word without its trigger).
fn apply_rule(rule: &CompiledRule, input: &str, s: &mut Search) -> Option<String> {
    if !rule.required.iter().all(|r| input.contains(r.as_ref())) {
        return None;
    }
    s.captures.clear();
    s.captures.resize(rule.capture_count, None);
    if rule.target.atoms.is_empty() {
        return apply_insertion_rule(rule, input, s);
    }

    let matches = select_matches(rule, input, s);
    if matches.is_empty() {
        return None;
    }

    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    for found in matches {
        output.push_str(&input[cursor..found.start]);
        render_replacement(rule, input, &found.captures, &mut output);
        cursor = found.end;
    }
    output.push_str(&input[cursor..]);
    Some(output)
}

fn apply_insertion_rule(rule: &CompiledRule, input: &str, s: &mut Search) -> Option<String> {
    let mut insertions = Vec::new();
    let mut boundaries: Vec<usize> = boundaries(input, 0..=input.len()).collect();
    if rule.direction == Direction::RightToLeft {
        boundaries.reverse();
    }
    for boundary in boundaries {
        if s.budget.exhausted {
            break;
        }
        if let Some(captures) = insertion_captures(rule, input, boundary, s) {
            insertions.push((boundary, captures));
            if rule.application == Application::Once {
                break;
            }
        }
    }
    if insertions.is_empty() {
        return None;
    }
    insertions.sort_by_key(|(boundary, _)| *boundary);

    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    let mut changed = false;

    // Unlike consuming matches, an insertion has no natural "next byte".
    // Enumerating the original boundaries exactly once gives it simultaneous
    // semantics and prevents emitted text from matching this rule recursively.
    for (boundary, captures) in insertions {
        output.push_str(&input[cursor..boundary]);
        let before_replacement = output.len();
        render_replacement(rule, input, &captures, &mut output);
        changed |= output.len() != before_replacement;
        cursor = boundary;
    }

    if !changed {
        return None;
    }
    output.push_str(&input[cursor..]);
    Some(output)
}

fn insertion_captures(
    rule: &CompiledRule,
    input: &str,
    boundary: usize,
    s: &mut Search,
) -> Option<Vec<Option<Span>>> {
    if (rule.target.anchored_at_start && boundary != 0)
        || (rule.target.anchored_at_end && boundary != input.len())
    {
        return None;
    }
    let mut found = None;
    left(&rule.left, input, boundary, s, &mut |s, _| {
        pattern(&rule.right, input, boundary, s, &mut |s, _| {
            let keep = !is_excluded(rule, input, boundary, boundary, s);
            if keep {
                found = Some(s.captures.clone());
            }
            keep
        })
    });
    found
}

fn select_matches(rule: &CompiledRule, input: &str, s: &mut Search) -> Vec<FoundMatch> {
    let mut selected = Vec::new();
    match rule.direction {
        Direction::LeftToRight => {
            let mut cursor = 0;
            while let Some(found) = find_next_match(rule, input, cursor, s) {
                cursor = found.end;
                selected.push(found);
                if rule.application == Application::Once {
                    break;
                }
            }
        }
        Direction::RightToLeft => {
            let mut edge = input.len();
            while let Some(found) = find_previous_match(rule, input, edge, s) {
                edge = found.start;
                selected.push(found);
                if rule.application == Application::Once {
                    break;
                }
            }
            selected.sort_by_key(|found| found.start);
        }
    }
    selected
}

fn find_next_match(
    rule: &CompiledRule,
    input: &str,
    search_from: usize,
    s: &mut Search,
) -> Option<FoundMatch> {
    for start in boundaries(input, search_from..=input.len()) {
        if s.budget.exhausted || (rule.target.anchored_at_start && start != 0) {
            break;
        }
        if let Some(found) = match_at(rule, input, start, None, s) {
            return Some(found);
        }
    }
    None
}

fn find_previous_match(
    rule: &CompiledRule,
    input: &str,
    through: usize,
    s: &mut Search,
) -> Option<FoundMatch> {
    for start in boundaries(input, 0..=through).rev() {
        if s.budget.exhausted {
            break;
        }
        if let Some(found) = match_at(rule, input, start, Some(through), s) {
            return Some(found);
        }
    }
    None
}

/// The first match whose target starts at `start`: its left context, the
/// longest target and its right context, in that order (a left context may
/// capture what the target refers back to). The target is tested first, on
/// its own, so most positions cost one comparison.
fn match_at(
    rule: &CompiledRule,
    input: &str,
    start: usize,
    must_end_by: Option<usize>,
    s: &mut Search,
) -> Option<FoundMatch> {
    if !may_start(&rule.target, input, start) {
        return None;
    }
    let mut found = None;
    left(&rule.left, input, start, s, &mut |s, _| {
        pattern(&rule.target, input, start, s, &mut |s, end| {
            // Nullable pieces are useful inside real patterns, but consuming
            // rules must make progress. Pure insertion has a separate path.
            end != start
                && must_end_by.is_none_or(|edge| end <= edge)
                && pattern(&rule.right, input, end, s, &mut |s, _| {
                    let keep = !is_excluded(rule, input, start, end, s);
                    if keep {
                        found = Some(FoundMatch {
                            start,
                            end,
                            captures: s.captures.clone(),
                        });
                    }
                    keep
                })
        })
    });
    found
}

/// Whether a pattern's first atom, if it must match, matches at `at`
/// (whatever has been captured): a match can't start where it doesn't.
fn may_start(pattern: &Pattern, input: &str, at: usize) -> bool {
    match pattern.atoms.first() {
        Some(atom)
            if matches!(atom.quantifier, Quantifier::One | Quantifier::OneOrMore)
                && !matches!(atom.matcher, Matcher::Backreference(_)) =>
        {
            unit_ends(&atom.matcher, input, at, &[]).next().is_some()
        }
        _ => !pattern.anchored_at_start || at == 0,
    }
}

fn is_excluded(
    rule: &CompiledRule,
    input: &str,
    target_start: usize,
    target_end: usize,
    s: &mut Search,
) -> bool {
    let (Some(left_pattern), Some(right)) = (&rule.excluded_left, &rule.excluded_right) else {
        return false;
    };
    left(left_pattern, input, target_start, s, &mut |s, _| {
        pattern(right, input, target_end, s, &mut |_, _| true)
    })
}

/// Matches of a left context ending at `end`, those starting leftmost
/// first.
fn left(context: &Pattern, input: &str, end: usize, s: &mut Search, next: &mut Next) -> bool {
    let first = if context.atoms.is_empty() { end } else { 0 };
    boundaries(input, first..=end).any(|start| {
        may_start(context, input, start)
            && pattern(context, input, start, s, &mut |s, at| {
                at == end && next(s, at)
            })
    })
}

/// Matches of a pattern from `start`, greediest first, each passed on to
/// `next` until it ends the search.
fn pattern(pattern: &Pattern, input: &str, start: usize, s: &mut Search, next: &mut Next) -> bool {
    (!pattern.anchored_at_start || start == 0) && atoms(pattern, 0, input, start, s, next)
}

fn atoms(
    pattern: &Pattern,
    index: usize,
    input: &str,
    at: usize,
    s: &mut Search,
    next: &mut Next,
) -> bool {
    if !s.budget.spend() {
        return false;
    }
    let Some(atom) = pattern.atoms.get(index) else {
        return (!pattern.anchored_at_end || at == input.len()) && next(s, at);
    };
    let base = s.ends.len();
    push_ends(atom, input, at, s);
    let top = s.ends.len();
    let mut found = false;
    for i in base..top {
        let Some(&end) = s.ends.get(i) else { break };
        let span = Some(Span { start: at, end });
        let previous = atom
            .capture
            .and_then(|c| s.captures.get_mut(c))
            .map(|slot| std::mem::replace(slot, span));
        found = atoms(pattern, index + 1, input, end, s, next);
        if let Some((slot, previous)) = atom
            .capture
            .and_then(|c| s.captures.get_mut(c))
            .zip(previous)
        {
            *slot = previous;
        }
        if found {
            break;
        }
    }
    s.ends.truncate(base);
    found
}

/// Push the ends `atom` reaches from `at`, each once, greatest first: greedy.
fn push_ends(atom: &Atom, input: &str, at: usize, s: &mut Search) {
    let base = s.ends.len();
    let (zero, many) = match atom.quantifier {
        Quantifier::One => (false, false),
        Quantifier::Optional => (true, false),
        Quantifier::ZeroOrMore => (true, true),
        Quantifier::OneOrMore => (false, true),
    };
    // A class can reach one end along two paths; anything else steps along
    // one, never back.
    let check = !many || matches!(atom.matcher, Matcher::Class(_));
    let (mut position, mut expanded) = (at, base);
    while s.budget.spend() {
        for end in unit_ends(&atom.matcher, input, position, &s.captures) {
            // Quantified matchers must make progress. A one-shot empty
            // backreference remains valid, but cannot make `*` loop forever.
            if !(many && end == position)
                && !(check && s.ends.get(base..).is_some_and(|e| e.contains(&end)))
            {
                s.ends.push(end);
            }
        }
        match s.ends.get(expanded) {
            Some(&end) if many => (position, expanded) = (end, expanded + 1),
            _ => break,
        }
    }
    if zero && !s.ends.get(base..).is_some_and(|e| e.contains(&at)) {
        s.ends.push(at);
    }
    if let Some(ends) = s.ends.get_mut(base..) {
        ends.sort_unstable_by(|a, b| b.cmp(a));
    }
}

/// Where one unit of `matcher` can end, matched at `at`.
fn unit_ends<'m, 'p>(
    matcher: &'m Matcher<'p>,
    input: &'m str,
    at: usize,
    captures: &[Option<Span>],
) -> impl Iterator<Item = usize> + use<'m, 'p> {
    let rest = input.get(at..).unwrap_or_default();
    let next_char = || rest.chars().next().map(|ch| at + ch.len_utf8());
    let (members, one): (&[Cow<str>], _) = match matcher {
        Matcher::Class(members) => (members, None),
        Matcher::Literal(literal) => (
            &[],
            rest.starts_with(literal.as_ref())
                .then(|| at + literal.len()),
        ),
        Matcher::NotClass(members) => (
            &[],
            next_char().filter(|_| !members.iter().any(|m| rest.starts_with(m.as_ref()))),
        ),
        Matcher::Any => (&[], next_char()),
        Matcher::WordBoundary => (&[], is_word_boundary(input, at).then_some(at)),
        Matcher::Backreference(capture) => (
            &[],
            captured(input, captures, *capture)
                .filter(|text| rest.starts_with(text))
                .map(|text| at + text.len()),
        ),
    };
    members
        .iter()
        .filter(move |member| rest.starts_with(member.as_ref()))
        .map(move |member| at + member.len())
        .chain(one)
}

fn is_word_boundary(input: &str, position: usize) -> bool {
    let previous = input[..position].chars().next_back();
    let next = input[position..].chars().next();
    match (previous, next) {
        (None, _) | (_, None) => true,
        (Some(previous), Some(next)) => previous.is_whitespace() != next.is_whitespace(),
    }
}

fn render_replacement(
    rule: &CompiledRule,
    input: &str,
    captures: &[Option<Span>],
    output: &mut String,
) {
    for part in rule.replacement.iter() {
        match part {
            ReplacementPart::Literal(literal) => output.push_str(literal),
            // Every successful match sets the captures a compiled
            // replacement names, and compiling checks that a map covers its
            // capture: nothing here is ever missing.
            ReplacementPart::Capture(capture) => {
                if let Some(captured) = captured(input, captures, *capture) {
                    output.push_str(captured);
                }
            }
            ReplacementPart::MappedCapture { capture, entries } => {
                if let Some((_, mapped)) = captured(input, captures, *capture)
                    .and_then(|c| entries.iter().find(|(source, _)| source == c))
                {
                    output.push_str(mapped);
                }
            }
        }
    }
}

/// The text capture `capture` matched.
fn captured<'i>(input: &'i str, captures: &[Option<Span>], capture: usize) -> Option<&'i str> {
    let span = captures.get(capture).copied().flatten()?;
    input.get(span.start..span.end)
}

/// The character boundaries of `input` in `range`.
fn boundaries(
    input: &str,
    range: RangeInclusive<usize>,
) -> impl DoubleEndedIterator<Item = usize> + '_ {
    range.filter(|&index| input.is_char_boundary(index))
}
