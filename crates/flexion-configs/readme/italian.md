Italian inflection for **any** Italian word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, gerund, participles), each with how sure
the grammar is and why.

```rust
use italian::*;

assert_eq!(noun("uomo", Number::Plural), "uomini");
assert_eq!(noun("città", Number::Plural), "città");
assert_eq!(adjective("bello", Gender::Feminine, Number::Plural), "belle");
assert_eq!(verb("finire", Tense::Present, Person::First, Number::Singular), "finisco");
assert_eq!(verb("prendere", Tense::PastHistoric, Person::First, Number::Singular), "presi");
assert_eq!(agreeing_participle("scrivere", Gender::Feminine, Number::Singular), "scritta");
assert_eq!(before_noun("bello", Gender::Masculine, Number::Plural, "occhi").as_deref(), Some("begli"));
```

Forms are written in standard spelling, with an accent only on a final
stressed vowel (`città`, `prenderò`) and on the monosyllables that take one
(`dà`, `può`).

Left out: pronouns. Their dictionary entries give no forms but the
possessive (`io`, `mio`), which the determiners already decline.
