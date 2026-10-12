Spanish inflection for **any** Spanish word, attested or not: noun plurals,
adjective agreement, and the whole conjugation (every simple tense, both
imperfect subjunctives, voseo, imperatives, participles), each with how sure
the grammar is and why.

```rust
use spanish::*;

assert_eq!(noun("canción", Number::Plural), "canciones");
assert_eq!(noun("joven", Number::Plural), "jóvenes");
assert_eq!(adjective("feliz", Gender::Feminine, Number::Plural), "felices");
assert_eq!(verb("contar", Tense::Present, Person::First, Number::Singular), "cuento");
assert_eq!(verb("hacer", Tense::Preterite, Person::First, Number::Singular), "hice");
assert_eq!(vos("tener", Mood::Indicative), "tenés");
assert_eq!(participle("escribir", Gender::Feminine, Number::Plural), "escritas");
```

Written accents are part of the word: `papá` and `papa` are two words.
