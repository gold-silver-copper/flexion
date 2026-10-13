# sound-law

`sound-law` is a dependency-free, generic sound-change engine. One macro owns
both the alphabet classes and the ordered rules, so a direct letter map and a
context-sensitive capture rule use the same notation.

```rust
use sound_law::{RuleSet, sound_laws};

const LAWS: RuleSet<'static> = sound_laws! {
    classes {
        V = ["a", "e", "i", "o", "u"];
        C = ["p", "t", "k", "s", "r"];
        HUSHING = ["sh", "ch"];
    }
    rules {
        // The old `letters!` shape: an unconditional map.
        palatalize: "k" => "ch" / "" _ "{V}";

        // The old `rewrites!` shape: A → B / C _ D.
        drop_glide: "j" => "" / "{HUSHING}" _ "{V}";

        // Capture a whole onset and move it after the vowel.
        metathesis: "{onset:C+}{v:V}" => "{v}{onset}";
    }
};

let program = LAWS.compile()?;
assert_eq!(program.apply("stra"), "astr");
# Ok::<(), sound_law::CompileError>(())
```

Rules are global and ordered. Every match of one rule is selected from the same
input form, so newly emitted text is not fed back into that rule, unless the
rule is `[iterative]` (see below). The next rule sees the complete result. A rule that needs a literal the word doesn't contain
(anywhere outside its optional and `*` parts) leaves the word as it is without
searching it.

## In flexion

[flexion](../../README.md) learns each part of speech's sound laws from its inflection
tables and runs them with this crate. A built form marks where its stem meets its ending
with `+`, and a law rewrites the letters around it (Greek `"ε+ο" => "+ου"`, Russian
`"{x:[к|г|х]}+ы" => "{x}+и"`), or changes a letter of the ending by the nearest letter of a
tier before it (Sanskrit `"न" => "ण" / "{t:[र|ष|ृ]}{a:![…]*}+{b:![…]*}" _ ""`, unless `्`
follows). Harmony is an `[iterative]` rule with an agreement map: an ending writes a class
letter once (Turkish `-lAr`, `-ImIz`), and the rule fills it by the nearest vowel before it as
written (`"{x:[A|I]}" => "{x|law0[t]}" / "{t:[a|e|ı|i|o|ö|u|ü]?}{g:![…]*}" _ ""`), so
`ev+lArImIzdA` is `evlerimizde`. The trainer builds rules at runtime and compiles them; a
generated crate holds the compiled program as static data (see below), so it never compiles
anything.

## Programs as static data

A compiled program is plain data: `CompiledRule`, `Pattern`, `Atom`, `Matcher`,
`ReplacementPart`, holding their strings and lists as `Cow`s. `Program::to_rust` writes a
program's rules as a Rust expression, and `Program::from_rules` runs them from a `static`,
with no compile step and no `OnceLock`:

```rust
use sound_law::compiled::*; // the compiled types, and `B` for `Cow::Borrowed`

const RULES: &[CompiledRule<'static>] = &[
    // drop_final[once]: "a" => "" / "" _ "$"
    CompiledRule {
        name: B("drop_final"),
        left: Pattern { atoms: B(&[]), anchored_at_start: false, anchored_at_end: false },
        target: Pattern {
            atoms: B(&[Atom { matcher: Matcher::Literal(B("a")), quantifier: Quantifier::One, capture: None }]),
            anchored_at_start: false,
            anchored_at_end: false,
        },
        right: Pattern { atoms: B(&[]), anchored_at_start: false, anchored_at_end: true },
        excluded_left: None,
        excluded_right: None,
        replacement: B(&[]),
        capture_count: 0,
        direction: Direction::LeftToRight,
        application: Application::Once,
        required: B(&[B("a")]),
    },
];
static PROGRAM: Program<'static> = Program::from_rules(RULES);

assert_eq!(PROGRAM.apply("papa"), "pap");
```

The output is meant to be written by `to_rust`, from a program `compile` checked. A
program written by hand that breaks the compiler's invariants never makes the engine
panic, but may not match what its notation would. `tests/static_program.rs` checks that a
written program equals the compiled one and runs the same.

A rule needs the literals its match can't do without (`required`): a word without one of
them costs the rule one substring search and no copy. In flexion every rule needs its
trigger character, so a form runs only the rules its trigger sets off.

## Correspondence maps

Named maps preserve a finite source/output pairing, such as voicing or vowel
height. Compilation proves that the map covers every possible value of its
capture:

```rust
# use sound_law::{RuleSet, sound_laws};
const VOICING: RuleSet<'static> = sound_laws! {
    classes {
        V = ["a", "e"];
        STOP = ["p", "t", "k"];
    }
    maps {
        VOICE = ["p" => "b", "t" => "d", "k" => "g"];
    }
    rules {
        voice: "{stop:STOP}" => "{stop|VOICE}" / "{V}" _ "{V}";
    }
};
# assert_eq!(VOICING.apply("apata").unwrap(), "abada");
```

Map outputs may be empty and several sources may share an output. Sources must
be nonempty and unique.

An agreement map is keyed by a second capture as well: `{x|MAP[t]}` emits the
output for `x`'s value and `t`'s. Compilation proves it covers every pair. An
optional capture that matched nothing has the empty value, so `("A", "")`
says what a word with no trigger gets. One harmony law maps target × trigger:

```rust
# use sound_law::{RuleSet, sound_laws};
const HARMONY: RuleSet<'static> = sound_laws! {
    classes {
        V = ["a", "e", "o", "i"];
    }
    agreements {
        AGREE = [
            ("A", "a") => "a", ("A", "o") => "a",
            ("A", "e") => "e", ("A", "i") => "e", ("A", "") => "e",
        ];
    }
    rules {
        // The nearest vowel before `A`, as written, or none.
        harmony[iterative]: "{x:[A]}" => "{x|AGREE[t]}" / "{t:V?}{gap:!V*}" _ "";
    }
};
# assert_eq!(HARMONY.apply("kotlAndA").unwrap(), "kotlanda");
# assert_eq!(HARMONY.apply("srbA").unwrap(), "srbe");
```

## Pattern language

| Form | Meaning |
|---|---|
| `text` | literal text |
| `{V}` | one member of class `V` |
| `{v:V}` | a member of `V`, captured as `v` |
| `{[a\|e\|ai]}` | one inline literal alternative, longest first |
| `{v:[a\|e\|ai]}` | an inline literal alternative captured as `v` |
| `{C\|V}` | union of two declared classes |
| `{C&SONORANT}` | intersection of two classes |
| `{C-[p\|t]}` | class difference using an inline set |
| `{x:.}` | any one Unicode scalar value, captured as `x` |
| `{x:!V}` | one scalar value that does not begin a member of `V` |
| `{?!V}` | nothing, where no member of `V` begins (reads ahead) |
| `{cluster:C+}` | one or more `C` segments, captured together |
| `{maybe:C?}` | zero or one `C` segment |
| `{run:C*}` | zero or more `C` segments |
| `{=x}` | the exact text captured earlier as `x` |
| `^`, `$` | start and end of the complete input form |
| `#` | edge of a whitespace-delimited word |
| empty target `""` | a boundary at which to insert |

Quantifiers are greedy and backtrack when a following atom requires it. Class
members can be multi-scalar strings; longer matching members win first. The
wildcard and a negated class consume one Unicode scalar because the engine
cannot infer a language's segment inventory outside declared classes.
Class intersection and difference bind before union and otherwise evaluate
left-to-right. Parenthesized or nested matcher expressions are not supported.

Replacement strings use `{capture}` references, `{capture|MAP}` and
`{capture|MAP[key]}`. Referencing captures in any
order makes metathesis and arbitrary reordering straightforward:

```text
{c:C}{v:V}             => {v}{c}       # CV → VC
{a:C}{b:C}             => {b}{b}       # regressive assimilation
{cluster:C+}{v:V}      => {v}{cluster} # move an entire cluster
{v:V}{=v}              => {v}          # collapse identical vowels
{left:C}{right:C}      => {left}e{right} # epenthesis
"" => e / "{C}" _ "{C}"               # pure boundary insertion
```

Captures declared in the left environment are available to the target and
replacement. Target captures are available in the right environment. This
allows agreement constraints without lookbehind-specific syntax:

```text
"{v:V}" _ "{=v}"       # between two identical vowels
```

Add `unless "left" _ "right"` after the positive context to suppress one
environment. The exclusion may use existing backreferences but cannot declare
captures:

```text
"s" => "" / "{v:V}" _ "{V}" unless "{=v}" _ "{=v}"
```

Use a backslash for literal metacharacters: `\{`, `\}`, `\^`, `\$`, and `\\`.
Use `\#` for a literal number sign. Word boundaries distinguish whitespace from
non-whitespace; punctuation has no implicit structural meaning.

## Application modes

Rules select simultaneous, non-overlapping target spans. The default selection
starts at the left; `[rtl]` starts at the right, changing which overlapping
spans win. `[once]` keeps only the first selected span. Options combine as
`[rtl, once]`, which applies at the last eligible match. Empty-target insertion
uses the same direction and count policy over original Unicode-scalar
boundaries.

`[iterative]` writes each match before seeking the next, past it in the rule's
direction: the context the rule has passed reads its output, the context ahead
its input (an output strictly local map; with a starred gap, one on a tier).
Harmony then runs through stacked suffixes, each vowel agreeing with the one
before it as written: `buzlArIn` → `buzların`, where a simultaneous rule reads
the stem's `u` through the unwritten `A` and gives `buzlarun`.
`[rtl, iterative]` reads its output on the right. An iterative insertion tries
each original boundary once.

A rule can read stress written as a mark: `{?!STRESS}` after a vowel is "this
vowel is unstressed", at the word's end too.

## Execution APIs

- `RuleSet::compile` validates names, classes, patterns, captures, and
  replacements once and returns a reusable `Program`.
- `Program::to_rust` and `Program::from_rules` write a compiled program as Rust
  and run it from a `static`; `Program::rules` lists its compiled rules.
- `Program::apply` runs the ordered program once.
- `Program::then` composes validated programs while rejecting duplicate rule
  identities.
- `Program::apply_range`, `apply_named_range`, `apply_through`, and `apply_from`
  execute a checked rule subset for staged debugging or partial derivations.
- `Program::apply_with_limit` applies a deterministic budget to matcher search
  states, returning the responsible rule and limit if it is exhausted.
- `Program::apply_with_trace` returns changed intermediate forms and rule names.
- `Program::apply_until_stable` repeats the whole program with a pass limit and
  explicit cycle detection.

Each left/target/right pattern is limited to 256 atoms at compilation, bounding
recursive matcher depth. `apply_with_limit` is recommended for declarations or
inputs received from an untrusted source; ordinary `apply` has no search-work
limit.

For an empty target, the engine tests each Unicode-scalar boundary in the
original form once. Contexts and captures see that unchanged form, and newly
inserted text is not reconsidered by the same rule. Thus an unconditional
`"" => "."` maps `ab` to `.a.b.`, while contexts normally restrict insertion to
linguistically relevant boundaries. A later rule still sees the complete
inserted result.

## Current limitations

- One run produces one deterministic result. Sporadic/probabilistic rules and
  branching result sets are not modeled.
- Classes and maps are finite inventories. There is no phonological feature
  matrix, feature mutation, normalization policy, syllabifier, stress model or
  tone-bearing-unit model; propagation (harmony) is an `[iterative]` rule on a
  tier written as a starred gap.
- Inline alternatives contain literal sequences; class expressions provide
  finite union/intersection/difference. Nested matcher groups, arbitrary
  lookaround, and quantified groups containing several matcher atoms are not
  available.
- `^`/`$` are complete-form edges and `#` is a whitespace-delimited word edge.
  Morpheme seams can be explicit literal symbols, but syllable and custom
  structural boundaries have no built-in semantics.
- A rule has one positive and at most one excluded environment. Multiple
  environments can often be factored into class/inline alternatives, but there
  is no environment-list syntax.
- `apply_with_limit` is opt-in; ordinary `apply`, traces, partial runs, and
  fixed-point execution do not impose a matcher work budget.
- The crate compiles Rust declarations and pattern strings. It does not parse
  Brassica, Lexurgy, or corpus-specific program formats.

See `UPSTREAM_CONFORMANCE.md` for exact tested subsets and `CORPUS_TOOLS.md` for
optional pinned-checkout metrics.
