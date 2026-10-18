# flexion

The runtime of the grammar crates `flexion-train` writes. A language crate *is*
its grammar, written as Rust values of the types defined here; this crate
answers from them. Nothing is computed before the first answer: the grammar
is `static` data.

What a grammar is made of:

- **Paradigms** ([`Paradigm`]): how a group of words inflects, one pattern per
  cell: `~а` is the stem and `а`; `~о~` a stem with a vowel inside it that
  other forms drop; `ge~t` a prefix and an ending. A paradigm states the
  inherent features its words share (Sanskrit `साधन` is neuter), and can be
  *like* another, stating only the cells that differ. A cell may be
  [`Form::Derived`]: its form comes from another cell's.
- **Relations** ([`Relation`]): where a cell's form comes from: another cell's
  form of the word (which one, by a feature of the word where that decides:
  the Russian accusative is the genitive of an animate noun, the nominative
  of an inanimate one), its end changed by rules read off that form's ending
  (Latin `amāvī` → `amāvistī`). A word whose forms are listed lists its
  principal parts; its other forms are told from them.
- **Choices** ([`Choice`]): which paradigm (or accent class, animacy,
  change, spelling) a word takes, read off its ending: the longest rule
  decides, each with how many of the dictionary's words follow it, and the
  words no rule gets right are listed. A rule's status ([`Status`]) says whether
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
- **Sound laws** ([`sound_law::Program`]): every built form goes through
  them, marked `+` where its stem meets its ending (Russian `к+ы` → `к+и`,
  Greek `κ+σ` → `ξ`, Sanskrit `न` → `ण` after `र` along a tier).
- **Spelling**: the [`Orthography`] reads a typed word into letters and
  stress; marks a caller may leave off (vowel length, `ё`) are restored by
  rules on the letters around them, or as the dictionary lists them.
- **Irregular forms**: the forms no rule gives, listed.

Every answer is a list of [`Candidate`]s, likeliest first, each with a
calibrated probability, a status and its evidence, printed as `form
(probability, status): reason`: the paradigm the word is like, and how many of
the dictionary's words with its ending take it.

What the caller knows ([`Known`]: a principal part, the word's gender or
animacy, that the lemma is spelled in full) weighs the readings: forms that
disagree drop them, and a paradigm whose words have another gender is less
likely.
