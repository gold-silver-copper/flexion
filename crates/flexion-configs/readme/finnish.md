Finnish inflection for **any** Finnish word, attested or not: nouns, names,
adjectives, pronouns and numerals in fourteen cases, two numbers and the
possessive forms; verbs in every mood, person, tense and polarity, the
negatives and perfects the phrases they are (`en tule`, `olen tullut`), and
their infinitives and participles; postpositions with their possessive
suffixes; adverbs in their degrees. Each answer says how sure the grammar is
and why. Vowel harmony is a learned sound law: an ending is written once
(`-ssA`) and the law chooses `a` or `ä` from the word's vowels. A verb of two
words inflects its first (`pyydän anteeksi`, `kiinni otettu`).

```rust
use finnish::*;

assert_eq!(noun("talo", Case::Inessive, Number::Singular), "talossa");
assert_eq!(noun("kylä", Case::Inessive, Number::Singular), "kylässä");
let me = Person::FirstSingular;
assert_eq!(indicative("tulla", Tense::Present, Polarity::Affirmative, me), "tulen");
assert_eq!(indicative("tulla", Tense::Perfect, Polarity::Negative, Person::ThirdSingular), "ei ole tullut");
assert_eq!(indicative("pyytää anteeksi", Tense::Present, Polarity::Affirmative, me), "pyydän anteeksi");
assert_eq!(postposition("alla", Suffix::FirstSingular), "allani");
```
