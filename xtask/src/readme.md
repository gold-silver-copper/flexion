<!-- Written by `cargo xtask readme` from `xtask/src/readme.md`: edit that, not this. -->
# flexion

Point a trainer at a language's [Kaikki](https://kaikki.org/) Wiktionary dump and get a
Rust crate that **is that language's inflectional grammar**: its declensions and
conjugations as named tables, the rules that choose them with their evidence, its sound
laws, its accent system, its irregular words' principal parts. The crate inflects **any** word, attested or
not, and every answer comes with a calibrated probability, how settled it is, and why.

```rust
use russian::*;

assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
// A word the dictionary doesn't have: every candidate, how likely, and why.
for c in Noun::new("бокр").candidates((Case::Accusative, Number::Singular)) {
    println!("{c}");
}
// бокр (61%, regular): like приём; 5031 of 5179 words in -р
// бокра (39%, regular): like приём; 5031 of 5179 words in -р
// (inanimate, or animate: say which with `.animacy(…)`, and the guess is gone)
```

{{count}} languages are built from the same trainer, with nothing language-specific in the
trainer or the runtime: {{languages}}. Each is a config in
`crates/flexion-configs`; `cargo xtask build german` rebuilds one.

## How well it does

Held-out words: a tenth of each part of speech (by a hash of the word) that the grammar
was not learned from, asked for every form the dictionary lists; the lemma's own form only
where it isn't what was typed (Latin `rex` asks for `rēx`). *Top-1 from the headword*: the
word typed as the dictionary's headword (Latin `rosa`, Greek `λύω`, no vowel length), with
its 95% interval (resampling words). *Lemma in full*: typed with every mark the tables
write. *Principal parts given*: the dictionary's principal parts given too. *Features
given*: the word's gender (Russian: and animacy; verbs: aspect) given, as `.gender(…)`
gives it. *Floor*: the exact non-neural baseline of the CoNLL–SIGMORPHON 2017 shared task
(letters only), learned from the same words as the grammar. *Against the floor*: the
grammar's letters right less the floor's, word by word on the same forms, with its 95%
interval. *On a training word / new root*: top-1 of the held-out words that are a training
word with a prefix or a compound's front, and of the rest. *Any listed form*: top-1 where
any form the dictionary lists in the cell counts. *Homograph ceiling*: the most any
grammar could get right (words typed alike get one answer per cell). *Lemma spelling*:
top-1 on the lemma's form where it isn't what was typed. *Own / form-of / alt-of*: top-1
of the words with an entry of their own, of those whose entry is a form of another word (a
participle, a comparative), and of spelling variants. *Phrases*: top-1 on the forms the
dictionary writes with a space (Latin `amātus sum`). *Irregular lines*: the share of the
training words the grammar lists a form or a stem of. This tenth was looked at while the
trainer was developed, up to PR #15; since PR #16 changes are judged on others (`cargo
xtask try` scores tenth {{develop}}, `--final` this one), and each PR's changes are confirmed once
on a tenth no decision looked at.

{{table results}}

*Where the errors are*: the wrong first answers by what went wrong. *Class*: the right
form is among the candidates, from another paradigm or reading (the grammar chose
wrong); *marks*: the same letters but for a mark the spelling folds (vowel length, a breathing; not a letter written with a combining mark, as a Devanagari vowel sign);
*accent*: the right letters, the accent elsewhere; *stem* and *ending*: where it goes
wrong; *variant*: another of the forms the dictionary lists in the cell; *homograph*:
another dictionary word spelled alike has this form; *gap*: no form at all (none, in
every part). *By block*: top-1 averaged over the part's groups of cells (a Turkish
verb's negative abilitative forms count as much as its plain ones). *Reproduced*: the
shipped grammar asked again for its own words (a sample of them), and *complete*: of their
cells, those with a form. Closed classes (pronouns, numerals) are scored both ways:
their held-out tenth is a handful of words.

{{table errors}}

*Compatible with stated length* (Ancient Greek): the dictionary marks a vowel's length both
ways, so a bare α, ι or υ whose table never marks it otherwise leaves its length unstated, and
an answer that is the dictionary's form but for a length mark there is compatible with it. A
second score, for the grammar and the floor alike; top-1 stays the score.

{{table compatible}}

*The shipped grammar and the scored one*: the grammar a crate ships is learned on every word,
the one scored here on all but this tenth. Their sound laws (with tiers) and irregular forms side
by side; a law in one and not the other is listed in the crate's README.

{{table shipped}}

Every language now has every part of speech its dictionary inflects: proper nouns
(English names choose `-ys`, `Kennedys`, where nouns choose `-ies`, so they have
grammars of their own), adverbs in their degrees, pronouns and numerals where the
tables give them, Italian articles as their spellings in order (`il|lo|l'`), Finnish
verbs and postpositions, Turkish verbs ({{cells turkish | verb}} forms each). Each crate's README says what
it leaves out and why (the Polish mobile person endings, which attach to any word
beginning a clause; English and French personal pronouns, a hand-written table, as the
dictionary gives only some of their forms).

Where the grammar is weak or below the floor, and why:

- **Latin, from the headword.** A headword doesn't write vowel length (`fox` for `fōx`);
  the grammar restores it by rules on the letters around each vowel (`-ātiō`, `dē-`, a
  diphthong), learned from every part of speech's words and reading which part a word is
  (`quīngēnī`, a numeral; `indigenus`, an adjective), and reads the three likeliest
  spellings. But a stem's own lengths are lexical, so a root its training words agree on
  (two or more, nine in ten) where the rules spell it otherwise is listed and read first
  (`ROOTS` in `marks.rs`). Every form of every training word votes on its root, one vote a
  word (a form-of page with the word whose table lists it), not only its lemma: on the
  development tenth nouns +0.61, on a tenth no decision looked at +0.89; adjectives +0.2
  (PR #21). A root no other word shares stays a guess. The vowel-length rules learn from
  every part's words, proper nouns and adverbs too, which took nouns from 70.1% to 71.1%,
  adjectives from 87.3% to 87.5% and verbs from 76.9% to 77.5% on this tenth (PR #19; on
  the development tenths verbs lost 0.41, within chance, and on tenth 9 0.61, within
  chance). Typed in full the grammars reach {{cell latin | noun | lemma in full}} (nouns), {{cell latin | adjective | lemma in full}} (adjectives)
  and {{cell latin | verb | lemma in full}} (verbs), and with the principal parts {{cell latin | noun | principal parts given}}, {{cell latin | adjective | principal parts given}} and
  {{cell latin | verb | principal parts given}}. The perfect passive is the phrase it is (`amātus sum`, `amātum esse`,
  `hortātus sum`): the verbs' *phrases*, {{cell latin | verb | phrases}} right as typed from the headword.
- **Ancient Greek.** A circumflex on a diphthong's second letter (`ἀριθμοῦ`) was read as
  vowel length, which split declensions in two; read as the diphthong's, nouns went from
  133 to 87 declensions and +3.8 points, adjectives +3.1 (development tenth, PR #16). The augment
  (a law where the form's first letters meet the stem: `ἐ‹ἀ → ἠ`), the perfect's
  reduplication (`%ε~κα`) and the augment behind a preverb (`ἀπο + ἐ → ἀπε`) are laws.
  The augment is one law for every vowel it meets, those few verbs show too (`ὑστέρουν`,
  `ηὔξανον`, `ἐνῴκουν`): verbs +1.3 points from the headword on the development tenth
  (PR #21).
  A dictionary leaves a short α, ι or υ bare as often as it writes its breve; a second
  score, *compatible with stated length* (its table above), counts an answer that differs
  from such a form only by a length mark the word's table never contradicts. Top-1 stays
  the score. The perfect middle-passive subjunctive and optative are phrases (`λελυμένος ὦ`), each
  word keeping its own accent. What's left in the verbs is mostly stems: a verb's aorist
  or perfect stem (`ἀγγέλλω` → `ἠγγέλθην`) follows from no ending, so a new verb's is a
  guess; known verbs list it with the ending table it takes, and a principal part given
  fixes its stem ({{cell ancient-greek | verb | principal parts given}}). A preverb kept in front moves the accent back only in a cell
  whose accent recedes, counting the optative's `-οι` long (`προφέροι`; Smyth §§ 169,
  425–426); a breathing stays at the word's start (`προοισθήσεσθαι`); `-εως`, `-εων`
  count short for the accent (`πόλεως`, § 163a); a form that needs an accent always has
  one. With the futures' and imperatives' invented cells gone (PR #19), nouns went from
  71.9% to 74.8%, adjectives from 77.3% to 78.0%, verbs from 65.5% to 68.0%, and the
  verbs' irregular lines from 52.4% to 46.7%.
- **Turkish verbs.** Their suffix vowels follow the vowel before them as written: one law
  on the vowels and the space (` a e i o u ö ü ı`). Learned on every word it used to take
  `b`, `d`, `g`, `v` for triggers (some paradigms write a `t` where the others write `a` or
  `e`, and a class with a consonant read every letter); 32,526 forms were listed to patch
  it, and the grammar shipped was not the one scored. Since PR #21 a class of vowels reads
  vowels: the verbs' irregular lines went from 59.04% to 6.33% (now {{cell turkish | verb | irregular lines}}, with
  {{cell turkish | verb | irregular forms}} listed forms).
- **Sanskrit.** The `n` → `ṇ` rule is a law (`तर्काणाम्`), and a noun's gender, given,
  takes it from {{cell sanskrit | noun | top-1}} to {{cell sanskrit | noun | features given}}. The verbs are few (a tenth is
  {{cell sanskrit | verb | held-out words}} words). Reading a root's tense pages (a perfect's, an aorist's) into its verb was tried in PR #19: fewer
  classes (59 → 34), but 1.9 points lost, so they stay apart.
- **Closed classes** (pronouns, numerals, determiners, postpositions) and parts with fewer
  than {{small}} held-out words are a handful of words each, mostly listed: their headline is what
  their grammar does with its own words (*reproduced N of N*), beside a new word's score,
  each word left out of training in turn (Turkish pronouns {{new word turkish | pronoun}}, Finnish pronouns
  {{new word finnish | pronoun}}).
- **Polish nouns** lost Middle Polish (`pl-mid-decl-noun`) tables, which aren't the modern
  language's (PR #18): on the development tenth −0.06 from the headword and −0.14 with the
  principal parts, the one loss this change was kept with.
- **Below the floor, beyond chance**, the floor learning from the same words as the
  grammar: {{below the floor}}. It isn't the declension chosen by the ending: in PR #18 a
  classifier reading endings up to eight letters chose no better by a point on German and
  Finnish nouns, English and French verbs or Spanish adjectives, and of 200 wrong German
  plurals, two in three were the plural class chosen wrong (a loanword's `-s`).
- **Russian** has no stress any more (stripped from input, never written).

## The generated code

A language crate has a module per part of speech. The excerpts below are read from the
crates by `cargo xtask readme`, which writes this README. From `russian`:

A paradigm: named after its commonest word, with the gender its words share, the stem
slots it builds through a shared ending table (`stems.rs`), then the cells that differ
from them, `~` for the stem, `derived` for a cell it takes from another. A paradigm
*like* another writes only what differs (`_` for the rest):

```rust
{{excerpt russian/src/noun/paradigms.rs | // Приём: | 7}}
{{excerpt russian/src/noun/paradigms.rs | // Полнота: | 7}}
```

An ending table: a slot's cells, in order, built on one stem. From `latin`, the third
declension's, which the consonant stems take (`stem gen_sg: "~- like ratiō"`: a slot is
named after the principal part it holds), and a word
that lists its stem with it (`irregular.rs`: `anas`, `anatis`… as the dictionary
writes it, `anit-`):

```rust
{{excerpt latin/src/noun/stems.rs | // gen_sg like ratiō: | 15}}
```

```text
{{excerpt latin/src/noun/irregular.rs | anas	gen_sg | 1}}
```

An analytic form is the phrase it is, learned like any other form: a Latin deponent's
perfect system (`modulātus sum` … `modulātum esse`), the participle agreeing in number:

```rust
{{excerpt latin/src/verb/stems.rs | // indicative_act_perf_1_sg like modulor: | 1}}
{{excerpt latin/src/verb/stems.rs | table: Cow::Borrowed(concat!( | 4}}
            ...
{{excerpt latin/src/verb/stems.rs | "~um esse\t", | 1}}
```

Where a `derived` cell comes from (`relations.rs`): from `russian`, the accusative is the
genitive or the nominative, by the word's animacy. From `latin`, a relation that changes
the form, with the rules on its ending that choose the change (`0`, `1`, `2`: the changes
in order):

```rust
{{excerpt russian/src/noun/relations.rs | relation!(ACC_SG, [GEN_SG, NOM_SG] | 1}}

{{excerpt latin/src/verb/relations.rs | relation!(ACT_PERF_2_SG, | 6}}
```

A cell no word (or too few) has is still built: by analogy with a pair of cells that
differ the same way. Few Greek verbs have a first person dual in the dictionary; it is
made from the first person plural as the second person's dual is from its plural:

```rust
{{excerpt ancient-greek/src/verb/relations.rs | // IND_ACT_1_DU, from IND_ACT_1_PL. | 4}}
```

Sound laws (`laws.rs`) are written in `sound-law`'s notation (`"target" => "replacement"
/ "before" _ "after"`) and compiled when the crate is built, each with the words whose
forms it gives and the forms it gets wrong (those are listed). Laws that are one law are
stated once: Russian `ы` → `и` after a velar or a hushing letter (seven laws, one per
letter, as learned), a captured letter (`{x}`) kept:

```rust
{{excerpt russian/src/noun/laws.rs | // [к|г|х|ш|ч|ж|щ]+ы → +и | 2}}
{{excerpt russian/src/noun/laws.rs | // [щ|ц|ч|ш|ж|к|г|х]+ям → +ам | 2}}
```

Sanskrit `n` → `ṇ`, with what may come between and what may not follow:

```rust
{{excerpt sanskrit/src/noun/laws.rs | // न → ण after | 2}}
```

The start of a word too. From `ancient-greek`: the perfect copies the stem's first letter
(`%`; a copy is written `°`, and a copied `φ` is `π`), and where the form's first letters
meet the stem (`‹`) a law lengthens the vowel, by a map; a preverb meets the augment as
the dictionary's compounds show:

```rust
{{excerpt ancient-greek/src/verb/laws.rs | law1 = [ | 1}}
    ...
{{excerpt ancient-greek/src/verb/laws.rs | // %ε‹ | 4}}

{{excerpt ancient-greek/src/verb/paradigms.rs | perfect_subj_pass_1: "%ε~γμένος ὦ" | 1}}
{{excerpt ancient-greek/src/verb/paradigms.rs | perfect_ind_act_1: "%ε~κᾰ" | 1}}

{{excerpt ancient-greek/src/verb/prefixes.rs | ), // ανα + ἐ → ἀνε | 1}}
```

The rules choosing a declension, each with its evidence (how many of the dictionary's
words with that ending take each declension), then the words no rule gets right, listed
by the declension they take (sorted into one lookup array when the crate is built):

```rust
{{excerpt russian/src/noun/rules.rs | "ба" => [ | 3}}
    ...
{{excerpt russian/src/noun/rules.rs | listed { | 3}}
```

Proper nouns have grammars of their own, as they choose otherwise: from `english`,
a name in `-y` takes `-s` (`Kennedys`), where a noun takes `-ies`:

```rust
{{excerpt english/src/proper_noun/rules.rs | "y" => [ | 2}}

{{excerpt english/src/proper_noun/paradigms.rs | // Hopewell: | 4}}
```

A part's cells are the language's. From `latin`, the subjunctive has no future:

```rust
{{excerpt latin/src/lib.rs | /// Tense of a subjunctive | 8}}
```

From `turkish`, a verb's {{cells turkish | verb}} forms; the doubled converb (`gele gele`) is the optative's
third person written twice, a change of its own (`~` → `~ ~`), beside the verbs of
words whose second word is another verb's (`~ gElE`):

```rust
{{excerpt turkish/src/verb/relations.rs | // CONVERB_POS_PLAIN_DOUBLED, from | 3 | 120}}
```

From `finnish`, a verb of two words inflects its first and keeps the other, after it
(`pyydän anteeksi`, `en pyydä anteeksi`) or before it in the participles (`anteeksi
pyytävä`, `kiinni otettu`), as the dictionary's verbs put it, cell by cell:

```rust
{{excerpt finnish/src/verb/compounds.rs | pub(super) static KEPT: | 2}}
    ...
{{excerpt finnish/src/verb/compounds.rs | flexion::Placement::Before, // PARTICIPLE_PRES_ACT | 1}}
```

From `latin`, how a vowel typed without its macron is written, by the letter before it
and the letters after it (`#` is the word's start or end), and, read first, a root whose
words all write a vowel one way the rules don't (`ROOTS`):

```rust
{{excerpt latin/src/marks.rs | "#a" => [ | 1}}
{{excerpt latin/src/marks.rs | "#act" => [ | 1}}
    ...
{{excerpt latin/src/marks.rs | "abbati" => | 1}}
```

From `ancient-greek`, the law of limitation's facts, and an accent pattern (where each
cell's accent goes before the law moves it):

```rust
{{excerpt ancient-greek/src/orthography.rs | prosody: Some( | 6}}
        ...
{{excerpt ancient-greek/src/orthography.rs | }), | 1}}

{{excerpt ancient-greek/src/noun/accent.rs | // 1, `νῠμφη` | 4}}
```

And what no rule gives, as the dictionary writes it (a table: word, then its listed
stems and forms by cell). Latin `adiuvō`'s perfect `adiūvī` follows from no rule on
`adiuvō`:

```text
{{excerpt latin/src/verb/irregular.rs | adiuvo	 | 1}}
```

## What it costs

The crates' source, as they are now, per crate (MB):

{{table size}}

They are {{mb}} MB in all (15.97 before PR #21), Turkish verbs {{mb turkish/src/verb}} MB (1.62 before
PR #21, when their vowel law came to read only vowels). Ancient Greek's verbs, {{cells ancient-greek | verb}} cells
each, are most of its size: their paradigms ({{mb ancient-greek/src/verb/paradigms.rs}} MB, {{cell ancient-greek | verb | paradigms}} conjugations) and
their irregular forms ({{mb ancient-greek/src/verb/irregular.rs}} MB: {{cell ancient-greek | verb | irregular forms}} forms no paradigm and listed stem
give, 27,750 before PR #21). Nothing is computed before the first answer, and the same entries
always give the same crate, byte for byte.

A clean debug build of each crate, one at a time (one machine, `-j 4`; measured on PR
#21's crates; CI's numbers are in its compile-cost job; every build's compiler peaked under
0.92 GB on PR #19's), and one call (release: a word's candidates for one cell, over 30
words and their first 40 cells; `cargo xtask bench <language> --pos <part>`, measured on
the grammars of PR #14), on words the grammar was learned from and on held-out words:

{{table costs}}

## How the grammar is learned

Everything below is learned from the dictionary's inflection tables, the same way for
every language, and kept only where it pays for itself: what an item costs to write
down (its letters and references, in bits) against what it saves in listing the words
it describes (`crates/flexion-train/src/bits.rs`).

- **Paradigms.** A word's forms are aligned on the letters they all keep (the stem
  pieces); what's left is a pattern per form: `~а`, `~о~` (a vowel the other forms drop),
  `ge~t`. Words with the same patterns share a paradigm, named after its commonest word
  that no sense marks vulgar or offensive (the dumps give no word frequencies: commonest
  means in the most dictionaries, with the most senses). A word missing some forms joins
  the fullest paradigm that builds the forms it has. A paradigm states the inherent
  features its words share (`[gender: Neuter]`), and is *like* the commoner paradigm it
  shares the most cells with: it writes only the cells that differ (`_` for the rest). It
  may be like it through a *view*: one feature's value read as another (a Latin deponent
  is like `agitō` with its active cells read as the passive ones: `like Agitō as
  ACT_AS_PASS`), or with something added after every pattern it reads there (an
  *affix*: a Russian reflexive is its plain verb with `~я`, `~ь` after a vowel, the stem's
  last piece their `с`: `like Играть with "~ь", "~я" after "йлмть"`). A paradigm of
  more than one word is kept where it costs fewer bits than
  listing the forms the next paradigm doesn't build; a word whose table only another
  paradigm nearly builds is listed there. A cell whose form is another cell's for nearly
  every word is that cell's, stated once (Greek middle = passive outside the aorist and
  future).
- **Stems and ending tables.** A part of speech's cells fall into stem slots, learned
  from where words' stems change together (Greek verbs: the twelve tense stems the
  dictionary's own conjugation template has). Per slot, the ending tables paradigms share
  are named after their commonest word; a word the paradigms don't build lists one stem
  per slot and the table that builds on it, not every form (`αγοραζω  perfect  ἠγορᾰσ-
  like ἐλπω`: ἀγοράζω builds its perfect on `ἠγορᾰσ-` with ἐλπω's endings). A form the
  caller gives fixes its slot's stem.
- **Sound laws.** A built form marks where its stem meets its ending (`+`). Where a word's
  paradigm, or a more general one, would build its forms but for a change there, the
  change is a law: Russian `[к|г|х|ш|ч|ж|щ]+ы → +и`, Greek `γ+σ → +ξ`, `κ+θ → +χθ`. A law
  can also change a letter of the ending by the nearest of some letters before it (a
  tier): Sanskrit `न → ण` after `र`, `ष`, `ृ` unless a dental, palatal or retroflex comes
  between. Which letters make the change, block it, or stop it after it, is learned. A
  law can also change a copied letter (reduplication, `%`): Greek copies `φ` as `π`
  (`πεφίληκα`); or join a form's first literal with the stem's first letters (`‹`): the
  Greek augment lengthens a vowel (`ἐ‹ἀ → ἠ`, `ἐ‹αἰ → ᾐ`: `ἦγον`, `ᾔτουν`), and the
  same law takes the letter written without its breathing (inside a compound). A law
  is kept when it gives forms of five or more words in two or more cells and breaks
  fewer than one in twenty of the forms it gives; the words it explains
  join the more general paradigm, and the forms it breaks are listed. A law may read its
  own earlier output and map a class of letters by the class letter that triggers it, so
  vowel harmony through stacked suffixes is one law: Turkish `I` → ı/i/u/ü and `A` →
  a/e by the word's last vowel, Finnish `A` → a/ä with `e`, `i` transparent; endings then
  print with class letters (`~lAr`). A class can copy the vowel before it: Finnish `V`,
  the illative `talo-Vn` → `taloon`, `maa-hVn` → `maahan`, and the possessive `-Vn`. A
  law can change a letter inside the stem, by what follows it, in the stems a paradigm
  marks as alternating (`ˇ`): Finnish consonant gradation (`katu`, `kadun`: `t → d`
  before a closed syllable; `kukka`, `kukan`) and its inverse (`hammas`, `hampaan`) are
  such laws, so `katu`, `itu`, `veto` and `otto` (`oton`) share a paradigm. Contexts grow
  one letter at a time while a law breaks too much, and every law is checked on held-out
  words. Laws run in [`sound-law`](crates/sound-law), printed in its notation and compiled
  when the crate is built (a law that doesn't compile is a compile error). A class letter
  goes on every spelling a cell lists (Finnish `~sellAAn|~sellAnsA`), not its first only.
- **Relations.** Each cell's form is told from another cell's (the one that predicts it
  best: a tree on the lemma's cell), its end changed by rules read off that form's ending
  (Latin `amāvī` → `amāvistī`). They are learned after the sound laws, and a change whose
  new ending is written in class letters goes through them: Turkish "my" is one change,
  `+Im` (`evim`, `kızım`, `gölüm`, `okulum`), where it was a change per vowel. A
  paradigm's cell that a relation with one change tells from a cell of the same stem
  slot is `derived` rather than written where that costs fewer bits (Finnish `talossani`
  is `talossa` and `-ni`, for every noun). Where a feature of the word decides which
  cell (Russian: the accusative is the genitive of animate nouns, the nominative of
  inanimate ones), the relation is by that feature, and the feature is a choice of its
  own; where one cell gives the form whatever the feature is, the feature has no say
  (Italian `parli`, the polite imperative, is the subjunctive's third person for every
  verb). A paradigm takes the cells none of its words has from its relations (`derived`),
  and a word whose forms are listed lists only its principal parts: its other forms are
  told from them, where that does as well as its paradigm. A cell too few words have is
  told by analogy: from the cell that differs from it as another pair of cells does,
  with that pair's change. Every word gets a form in every cell, attested or not.
- **Every word.** Where a part of speech has far more words than its structure needs
  (Finnish nouns: {{cell finnish | noun | words}}), its paradigms, relations and laws are learned from the
  commonest ({{max words finnish | noun}}), word by word; then every word takes the paradigm that builds the
  most of its forms, teaches the rules that choose one, and lists what the grammar still
  gets wrong. Every dictionary word gets its own forms.
- **Choices.** Which paradigm a word takes is a list of rules on its ending: the longest
  rule the word has decides, with how many of the dictionary's words with that ending
  take each paradigm. A rule is kept only when it describes its words in fewer bits
  than the broader rule does, by more than the rule costs (minimum description length). Words no rule gets
  right are listed. A rule's status follows Yang's Tolerance Principle, judged as a learner
  meets it: on the 250, 500, 1,000 and 2,000 commonest words (in the most dictionaries,
  with the most senses) that have its ending; a rule with few enough exceptions there is
  productive (`regular`, printed on the rule); otherwise speakers vary (`unsettled`).
- **Compounds and prefixes.** A word written in parts inflects its last part or each
  part (Russian `диван-кровать`). A word the grammar doesn't list that ends in a listed
  word takes that word's listing, its front kept (Finnish `aavameri` inflects as `meri`,
  and its forms aren't listed): every listed word from some length on, where nine in ten
  of the dictionary's words ending in an irregular word that long inflect as it (Finnish
  five letters; none in English, Russian or German), and, of the shorter ones, each word
  whose own compounds do (`heads.rs`: German `Schein` for `Fahrschein`, `Wein` for
  `Schaumwein`). German verbs inflect behind their prefixes: separable ones go where the
  form puts them (`fing an`), inseparable ones stay in front and take off the
  participle's `ge-` (`verhindert`). The prefixes, and what the participle drops, are
  learned.
- **Accent.** Where a language writes it (Ancient Greek), each cell has a rule placing the
  accent (where the lemma has it, on the ending, as far back as allowed, or on the word's
  last or next-to-last syllable) and the law
  of limitation moves it and picks acute or circumflex by the syllables' length. Which
  endings take the circumflex, and where a final `-οι` counts long (the optative), are
  learned.
- **Spelling.** Words are read in Unicode's composed form (NFC), so a letter typed with
  its marks in any order is the same letter. A word is found however it is typed:
  without vowel length (`rosa`, `λυω`), without `ё`. Rules on the letters around each
  vowel restore the marks (`-ātiō`, `dē-`), learned once for the language from every
  part of speech's words (`marks.rs`: a root's length shown in a verb tells it in its
  noun), and the three likeliest spellings are read, over every vowel at once; the
  dictionary's words that the rules don't spell are listed, a root whose words (two or
  more) all write a vowel one way the rules don't is listed and read first (Latin words
  beginning `amico` write `amīc-`), and the answer's probability
  says how much of the spelling was a guess. A circumflex is vowel length on a single
  vowel (`σῦς`), not on a diphthong's second letter (`ἀριθμοῦ`).
- **Phrases.** A form the dictionary writes as a phrase is learned like any other: Latin
  `amātus sum`, the participle agreeing in number (`amātī sumus`), Greek `λελυμένος ὦ`,
  each word keeping its own accent. Where the dictionary's table only gives the recipe
  (the Latin perfect passive: the participle and a form of `sum`), the forms are written
  out from it (Allen & Greenough §§184–196; Smyth §§599, 696).
- **Irregular forms.** Whatever no rule gives is listed, as the dictionary writes it.
- **Calibration.** Probabilities are sharpened by one power per part of speech, fitted on
  a tenth of the words the grammar didn't learn from, typed as their headwords; an answer
  of a status whose answers are right less often than they say keeps only the share it
  earns (`trust`), the rest is "none of these".

The trainer builds the same values the crates hold, and prints them; a test prints
grammars learned from real entries, compiles them, and checks that they answer exactly
as the grammars they were printed from.

## Layout

{{crates}}
- `languages/*`: the generated crates.

## Building a language

```sh
cd flexion
cargo xtask extract russian   # read ~/.cache/flexion/dumps/Russian.jsonl
cargo xtask build russian     # learn, measure, and write languages/russian
cargo xtask build all --lanes 2   # every language, two at a time, then this README
cargo xtask readme            # this README, from the saved builds and the crates
```

A build saves what it learned and measured in `target/build-cache/`: built again with only
the printer changed, it reprints from there without learning again.

## License

The code is MIT or Apache-2.0. The generated grammars are derived from Wiktionary and are
licensed under CC BY-SA 4.0 (see each crate's `DATA-LICENSE.md`).
