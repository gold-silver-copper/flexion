Turkish inflection for **any** Turkish word, attested or not: nouns, names,
adjectives, pronouns and numerals in six cases, two numbers and the
possessed forms; verbs in all 928 forms of the dictionary's tables (the
tenses with their copulas, the imperatives, participles and converbs, each
positive and negative, plain and abilitative), each with how sure the
grammar is and why. Vowel harmony is a learned sound law: an ending is
written once (`-lAr`, `-DA`) and the law chooses its vowel from the vowel
before it, as written, so it runs through a chain of suffixes
(`ev-ler-imiz-de`, `gel-me-yecek-ler`). A name keeps the apostrophe before
its endings (`Nepal'e`).

```rust
use turkish::*;

assert_eq!(noun("ev", Case::Locative, Number::Plural), "evlerde");
assert_eq!(noun("kitap", Case::Locative, Number::Plural), "kitaplarda");
let (yes, no, plain) = (Polarity::Positive, Polarity::Negative, Ability::Plain);
assert_eq!(finite("gelmek", yes, plain, Tense::Progressive, Copula::None, Subject::FirstSingular), "geliyorum");
assert_eq!(finite("gelmek", no, plain, Tense::Future, Copula::None, Subject::ThirdPlural), "gelmeyecekler");
assert_eq!(converb("gelmek", yes, plain, Converb::Doubled), "gele gele");
```

Left out: comparison with `daha` and `en`, which are words of their own; and
the predicative forms the noun tables end with (`evim` "I am a house",
`evdir`). Those endings are the copula, which attaches to any predicate (an
adjective, a case form: `evdeyim`; Göksel & Kerslake ch. 12), not forms of
the noun; the verb part writes the copula's forms.
