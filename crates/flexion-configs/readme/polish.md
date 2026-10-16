Polish inflection for **any** Polish word, attested or not: noun declension,
adjectives, and verbs (non-past, past and conditional by person and gender,
imperative, participles), each with how sure the grammar is and why.

```rust
use polish::*;

assert_eq!(noun("kot", Case::Genitive, Number::Singular), "kota");
assert_eq!(noun("ręka", Case::Locative, Number::Singular), "ręce");
assert_eq!(non_past("czytać", Person::First, Number::Singular), "czytam");
assert_eq!(past("czytać", Person::First, Agreement::Feminine), "czytałam");
```

Whether a masculine noun's genitive takes `-a` or `-u` can't be told from its
spelling; the grammar says how sure it is, and a form you know settles it:

```rust
use polish::*;

for c in Noun::new("blorp").candidates((Case::Genitive, Number::Singular)) {
    println!("{c}");
}
let blorp = Noun::new("blorp").given((Case::Genitive, Number::Singular), "blorpu");
assert_eq!(blorp.form((Case::Locative, Number::Singular)), "blorpie");
```

Left out: the mobile person endings (`żebym`, `kiedyś`, `gdzieśmy`). The
dictionary gives the same six forms on conjunctions, adverbs and particles,
but they attach to whatever word begins the clause: they are no inflection of
those words.
