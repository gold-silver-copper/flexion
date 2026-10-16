Turkish declension for **any** Turkish noun, attested or not: six cases in
two numbers, and the possessed forms, each with how sure the grammar is and
why. Vowel harmony is a learned sound law: an ending is written once
(`-lAr`, `-DA`) and the law chooses its vowel from the vowel before it, as
written, so it runs through a chain of suffixes (`ev-ler-imiz-de`).

```rust
use turkish::*;

assert_eq!(noun("ev", Case::Locative, Number::Plural), "evlerde");
assert_eq!(noun("kitap", Case::Locative, Number::Plural), "kitaplarda");
```

Left out: comparison with `daha` and `en`, which are words of their own; and
the predicative forms the noun tables end with (`evim` "I am a house",
`evdir`). Those endings are the copula, which attaches to any predicate (an
adjective, a case form: `evdeyim`; Göksel & Kerslake ch. 12), not forms of
the noun; the verb part writes the copula's forms.
