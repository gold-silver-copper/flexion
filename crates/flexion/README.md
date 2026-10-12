# flexion

The runtime of the grammar crates `flexion-train` writes. A language crate *is*
its grammar, written as Rust values of the types defined here; this crate
answers from them. Nothing is computed before the first answer: the grammar
is `static` data.

What a grammar is made of:

- **Paradigms** ([`Paradigm`]): how a group of words inflects, one pattern per
  cell. `~а` is the stem and `а`; `~о~` a stem with a vowel inside it that
  other forms drop; `ge~t` a prefix and an ending. A cell may be
  [`Form::Referred`]: its form is another cell's, which one the word's
  feature says ([`Referral`]: the Russian accusative is the genitive of an
  animate noun and the nominative of an inanimate one).
- **Choices** ([`Choice`]): which paradigm (or accent class, animacy,
  spelling) a word takes, read off its ending: the longest rule decides,
  each with how many of the dictionary's words follow it, and the words no
  rule gets right are listed. A rule's status ([`Status`]) says whether
  speakers extend it (Yang's Tolerance Principle) or vary.
- **Accent** ([`Accent`], [`Prosody`]): per cell, the rule that places the
  accent (on the lemma's syllable, on the ending, or as far back as the law
  allows), and the law of limitation that moves it and picks acute or
  circumflex by the syllables' length (Ancient Greek).
- **Prefixes** ([`Prefixes`]): separable ones go where the cell puts them
  (German `räuchre aus`, `ausgeräuchert`); inseparable ones stay in front and
  take off what the cell says (`verhindert`, not `vergehindert`).
- **Compounds** ([`Compound`]): a word written in parts inflects only its
  last part, or each part.
- **Principal parts** ([`Systems`]): each cell belongs to the system of one
  part (Latin `amō, amāre, amāvī, amātum`) and is inflected from it.
- **Spelling**: the [`Orthography`] reads a typed word into letters and
  stress; marks a caller may leave off (vowel length, `ё`) are restored by
  rules on the letters around them, or as the dictionary lists them.
- **Irregular forms**: the forms no rule gives, listed.

Every answer is a list of [`Candidate`]s, likeliest first, each with a
calibrated probability, a status and its evidence:

```text
книги (97%, regular): like книга; 1203 of 1210 words in -га
```

What the caller knows ([`Known`]: a principal part, the word's animacy, that
the lemma is spelled in full) drops the readings that disagree with it.
