French inflection for **any** French word, attested or not: noun plurals,
adjective and participle agreement, and the whole simple conjugation
(indicative, subjunctive, imperative, participles), each with how sure the
grammar is and why.

```rust
use french::*;

assert_eq!(noun("cheval", Number::Plural), "chevaux");
assert_eq!(adjective("beau", Gender::Feminine, Number::Singular), "belle");
assert_eq!(before_vowel("nouveau"), "nouvel");
assert_eq!(verb("appeler", Tense::Present, Person::First, Number::Singular), "appelle");
assert_eq!(verb("manger", Tense::Present, Person::First, Number::Plural), "mangeons");
assert_eq!(verb("prendre", Tense::PastHistoric, Person::Third, Number::Singular), "prit");
assert_eq!(agreeing_participle("prendre", Gender::Feminine, Number::Plural), "prises");
```

Accents and `ç` are letters: `pêcher` and `pécher` are two words.

Left out: proper nouns, whose dictionary entries give no forms (a French
name doesn't inflect).
