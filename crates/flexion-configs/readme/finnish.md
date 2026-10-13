Finnish declension for **any** Finnish noun or adjective, attested or not:
fourteen cases in two numbers, and a noun's possessive forms, each with how
sure the grammar is and why. Vowel harmony is a learned sound law: an ending
is written once (`-ssA`) and the law chooses `a` or `ä` from the word's vowels.

```rust
use finnish::*;

assert_eq!(noun("talo", Case::Inessive, Number::Singular), "talossa");
assert_eq!(noun("kylä", Case::Inessive, Number::Singular), "kylässä");
```
