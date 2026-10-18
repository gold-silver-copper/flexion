//! How a letter typed without its mark is written (vowel length): the
//! language's marks rules, learned from its lemmas, and its roots, learned
//! from every form of its words (see [`flexion::Marks`]). Learned once for
//! a language, from every part of speech's words: a root's length shown in
//! one part of speech's word (`iūdicō`) tells it in another's (`iūdex`).

use crate::HashMap;
use crate::choose::{self, Settings};
use flexion::Orthography;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

/// A word as the roots read it: every spelling its forms write.
#[derive(Debug, Clone)]
pub struct Voter<'p> {
    /// The word whose vote it casts, numbered: its own, or that of the word
    /// a form-of page is a form of (Latin `abditus` votes with `abdō`), so
    /// that each word votes once.
    pub owner: usize,
    /// Its part of speech, as the marks read it.
    pub part: &'p str,
    /// Its lemma's key and spelling.
    pub key: String,
    pub lemma: String,
    /// Every spelling of its forms, once each.
    pub spellings: Vec<String>,
}

/// A language's marks model, and how its roots did per part of speech on
/// half the words, learned from the other half (see [`check`]).
#[derive(Debug, Clone)]
pub struct LanguageMarks {
    pub marks: flexion::Marks<'static>,
    /// Per part of speech, how its roots did.
    pub checked: BTreeMap<String, Checked>,
}

/// How a part of speech's roots did on lemmas they weren't learned from
/// (see [`check`]).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Checked {
    /// The letters the roots decide, as [`flexion::Marks::root_letters`].
    pub letters: (u32, u32),
    /// The lemmas the roots spelled right where the rules alone didn't.
    pub fixed: u32,
    /// The lemmas they spelled wrong where the rules alone didn't.
    pub broken: u32,
}

impl LanguageMarks {
    /// The parts of speech the roots spell more lemmas wrong than right:
    /// they take none.
    pub fn rootless(&self) -> impl Iterator<Item = &str> {
        (self.checked.iter())
            .filter(|(_, c)| c.broken > c.fixed)
            .map(|(part, _)| part.as_str())
    }

    /// The marks part of speech `part` reads.
    pub fn of(&self, part: &str) -> flexion::Marks<'static> {
        let rootless = self.rootless().any(|p| p == part);
        flexion::Marks {
            rules: self.marks.rules.clone(),
            roots: if rootless {
                Cow::Borrowed(&[])
            } else {
                self.marks.roots.clone()
            },
            part: Cow::Owned(part.to_string()),
            root_letters: self.checked.get(part).map_or((0, 0), |c| c.letters),
        }
    }
}

/// The marks model of `words` (key, lemma letters, part of speech) and
/// `voters`: the rules from every letter of the lemmas that a mark could
/// stand on, by the letter before it and the letters from it on; the roots
/// from every spelling of the voters. `None` where no lemma writes a mark.
pub fn learn(
    orthography: &Orthography<'_>,
    words: &[(&str, &str, &str)],
    voters: &[Voter<'_>],
    settings: Settings,
) -> Option<LanguageMarks> {
    let (examples, marked) = examples(orthography, words);
    if !marked {
        return None;
    }
    let rules = choose::learn_starts(&examples, settings);
    let roots = roots(orthography, voters.iter(), &rules, settings.smoothing);
    Some(LanguageMarks {
        marks: flexion::Marks {
            rules,
            roots: Cow::Owned(roots),
            part: Cow::Borrowed(""),
            root_letters: (0, 0),
        },
        checked: check(orthography, words, voters, settings),
    })
}

/// Every letter of `words` a mark could stand on, read as the rules read
/// it, with how the word writes it; and whether any writes a mark.
fn examples(
    orthography: &Orthography<'_>,
    words: &[(&str, &str, &str)],
) -> (Vec<(String, char)>, bool) {
    let markable = |c: char| orthography.folded.iter().any(|e| e.1 == c);
    let mut examples: Vec<(String, char)> = Vec::new();
    let mut marked = false;
    for &(key, letters, part) in words {
        let mut before = flexion::WORD_END;
        for ((i, k), l) in key.char_indices().zip(letters.chars()) {
            if markable(k)
                && let Some(rest) = key.get(i..)
            {
                marked |= l != k;
                examples.push((flexion::Marks::context(before, rest, part), l));
            }
            before = k;
        }
    }
    (examples, marked)
}

/// The fewest words that write a root's letter (see
/// [`flexion::Marks::roots`]) for it to be listed.
const ROOT_WORDS: u32 = 2;

/// The share of those words that must write it one way.
const ROOT_SHARE: f64 = 0.9;

/// Whether `c` written bare leaves its length unstated: the orthography's
/// prosody counts it long written with a mark (Greek `ᾱ`) but not bare
/// (`α`), so where the dictionary tells its length it marks the short one
/// too (`ᾰ`), and a bare one tells nothing.
fn unstated(orthography: &Orthography<'_>, c: char) -> bool {
    orthography.prosody.as_ref().is_some_and(|p| {
        !p.long.contains(c) && p.long.chars().any(|l| l != c && orthography.fold(l) == c)
    })
}

/// The roots the marks `rules` don't tell (see [`flexion::Marks::roots`]):
/// the beginnings of the voters' spellings through a letter that may take a
/// mark and two letters past it, whose words (each once, at least
/// [`ROOT_WORDS`]; a bare letter whose length is [`unstated`] none, nor a
/// word whose forms write the letter both ways) write that letter one way,
/// [`ROOT_SHARE`] of them; where they don't decide, the voters' lemmas
/// beginning so, each spelling once, all one way (at least [`ROOT_WORDS`]:
/// a verb's participles the dictionary lists as headwords of their own
/// tell its root, `perstīllō`, `perstīllāns`).
/// Listed where the rules write the letter otherwise in one of those
/// spellings. Sorted.
pub fn roots<'v, 'p: 'v>(
    orthography: &Orthography<'_>,
    voters: impl Iterator<Item = &'v Voter<'p>>,
    rules: &flexion::Choice<'_, char>,
    smoothing: f32,
) -> Vec<(Cow<'static, str>, char)> {
    let markable = |c: char| orthography.folded.iter().any(|e| e.1 == c);
    // Each place a spelling may hold a root's letter: where, the letters
    // through it, the root (if the spelling is long enough to hold it), and
    // how the spelling writes the letter (`None`: its length unstated). A
    // phrase's words are other spellings' (`amātus sum`).
    let places = |letters: &str| -> (String, Vec<Place>) {
        let key = orthography.key(letters);
        if letters.contains(char::is_whitespace) {
            return (key, Vec::new());
        }
        let places = (key.char_indices().zip(letters.chars()).enumerate())
            .filter(|&(_, ((_, c), _))| markable(c))
            .map(|(n, ((at, c), l))| Place {
                at: n,
                through: key.get(..at + c.len_utf8()).unwrap_or_default().to_string(),
                root: flexion::Marks::root_of(&key, n).map(str::to_string),
                letter: (l != c || !unstated(orthography, c)).then_some(l),
            })
            .collect();
        (key, places)
    };
    let mut voters: Vec<&Voter<'_>> = voters.collect();
    voters.sort_by_key(|v| v.owner);
    // How many words write each root's letter each way. A word whose forms
    // beginning with the same letters write the letter both ways casts no
    // vote on it: its length goes with the ending, not the root (`saliō`,
    // `salīmus`; `stat`, `stātō`).
    let mut votes: HashMap<String, BTreeMap<char, u32>> = HashMap::default();
    for word in voters.chunk_by(|a, b| a.owner == b.owner) {
        let mut ways: HashMap<String, BTreeSet<char>> = HashMap::default();
        let mut mine: BTreeSet<(String, String, char)> = BTreeSet::new();
        for place in word
            .iter()
            .flat_map(|v| &v.spellings)
            .flat_map(|s| places(s).1)
        {
            let Some(letter) = place.letter else { continue };
            ways.entry(place.through.clone())
                .or_default()
                .insert(letter);
            if let Some(root) = place.root {
                mine.insert((root, place.through, letter));
            }
        }
        for (root, through, l) in mine {
            if ways.get(&through).is_some_and(|w| w.len() == 1) {
                *votes.entry(root).or_default().entry(l).or_default() += 1;
            }
        }
    }
    let mut decided: HashMap<String, char> = votes
        .into_iter()
        .filter_map(|(root, ways)| {
            let total: u32 = ways.values().sum();
            let (&l, &n) = ways.iter().max_by_key(|w| *w.1)?;
            (total >= ROOT_WORDS && f64::from(n) >= ROOT_SHARE * f64::from(total))
                .then_some((root, l))
        })
        .collect();
    let spelled: BTreeSet<&str> = voters.iter().map(|v| v.lemma.as_str()).collect();
    let mut seen: HashMap<String, BTreeMap<char, u32>> = HashMap::default();
    for place in spelled.into_iter().flat_map(|s| places(s).1) {
        if let (Some(root), Some(l)) = (place.root, place.letter)
            && !decided.contains_key(&root)
        {
            *seen.entry(root).or_default().entry(l).or_default() += 1;
        }
    }
    decided.extend(seen.into_iter().filter_map(|(root, ways)| {
        match ways.len() {
            1 => ways
                .into_iter()
                .find(|w| w.1 >= ROOT_WORDS)
                .map(|w| (root, w.0)),
            _ => None,
        }
    }));
    // The spellings a decided root reads (with the part of speech they are
    // read as, and where), as the rules alone write them.
    let mut read: BTreeMap<(String, &str), Vec<(usize, &str)>> = BTreeMap::new();
    for v in &voters {
        for s in &v.spellings {
            let (key, places) = places(s);
            let mine: Vec<(usize, &str)> = (places.into_iter())
                .filter_map(|p| {
                    let (root, _) = decided.get_key_value(p.root.as_deref()?)?;
                    Some((p.at, root.as_str()))
                })
                .collect();
            if !mine.is_empty() {
                read.entry((key, v.part)).or_default().extend(mine);
            }
        }
    }
    for v in &voters {
        let (key, places) = places(&v.lemma);
        let mine: Vec<(usize, &str)> = (places.into_iter())
            .filter_map(|p| {
                let (root, _) = decided.get_key_value(p.root.as_deref()?)?;
                Some((p.at, root.as_str()))
            })
            .collect();
        if !mine.is_empty() {
            read.entry((key, v.part)).or_default().extend(mine);
        }
    }
    let read: Vec<Read<'_>> = read.into_iter().collect();
    let ruled = |((key, part), _): &Read<'_>| {
        let marks = marks(rules, &[], part, (0, 0));
        flexion::restore(orthography, &marks, smoothing, key, key).0
    };
    let told: Vec<String> = crate::par::map(&read, ruled);
    let mut out: BTreeSet<(&str, char)> = BTreeSet::new();
    for ((_, places), told) in read.iter().zip(&told) {
        let told: Vec<char> = told.chars().collect();
        for &(n, root) in places {
            let Some(&v) = decided.get(root) else {
                continue;
            };
            if told.get(n) != Some(&v) {
                out.insert((root, v));
            }
        }
    }
    out.into_iter()
        .map(|(r, v)| (Cow::Owned(r.to_string()), v))
        .collect()
}

/// A place in a spelling that may hold a root's letter (see [`roots`]).
struct Place {
    at: usize,
    through: String,
    root: Option<String>,
    letter: Option<char>,
}

/// A spelling's key and part of speech, and where it holds a decided
/// root's letter.
type Read<'r> = ((String, &'r str), Vec<(usize, &'r str)>);

/// The marks of `rules` and `roots`, read as part of speech `part`, its
/// roots deciding `root_letters` (see [`flexion::Marks::root_letters`]).
fn marks<'m>(
    rules: &'m flexion::Choice<'_, char>,
    roots: &'m [(Cow<'_, str>, char)],
    part: &'m str,
    root_letters: (u32, u32),
) -> flexion::Marks<'m> {
    flexion::Marks {
        rules: flexion::Choice {
            side: rules.side,
            rules: Cow::Borrowed(rules.rules.as_ref()),
            exceptions: Cow::Borrowed(rules.exceptions.as_ref()),
        },
        roots: Cow::Borrowed(roots),
        part: Cow::Borrowed(part),
        root_letters,
    }
}

/// How the roots do per part of speech, on lemmas they weren't learned
/// from: the words split in two halves by key, each half's lemmas spelled
/// by the marks learned from the other half's words. Per part of speech,
/// the letters the roots decide and those they write right; then, the
/// roots trusted that much, the lemmas they spell right where the rules
/// alone don't, and those they spell wrong where the rules alone don't.
fn check(
    orthography: &Orthography<'_>,
    words: &[(&str, &str, &str)],
    voters: &[Voter<'_>],
    settings: Settings,
) -> BTreeMap<String, Checked> {
    // Halves apart from the tenths a word is held out by (`hash % 10`).
    let half = |key: &str| crate::eval::hash(key) / 10 % 2;
    let halves = [0, 1].map(|h| {
        let taught: Vec<(&str, &str, &str)> =
            words.iter().filter(|w| half(w.0) != h).copied().collect();
        let (examples, _) = examples(orthography, &taught);
        let rules = choose::learn_starts(&examples, settings);
        let teachers = voters.iter().filter(|v| half(&v.key) != h);
        let roots = roots(orthography, teachers, &rules, settings.smoothing);
        let tested: Vec<&(&str, &str, &str)> = words.iter().filter(|w| half(w.0) == h).collect();
        (rules, roots, tested)
    });
    let mut checked: BTreeMap<String, Checked> = BTreeMap::new();
    for (rules, roots, tested) in &halves {
        let rooted = marks(rules, roots, "", (0, 0));
        for &&(key, letters, part) in tested {
            let c = checked.entry(part.to_string()).or_default();
            for (n, l) in letters.chars().enumerate() {
                if let Some(v) = rooted.root(key, n) {
                    c.letters.0 += u32::from(v == l);
                    c.letters.1 += 1;
                }
            }
        }
    }
    for (rules, roots, tested) in &halves {
        let spelled = crate::par::map(tested, |&&(key, letters, part)| {
            let trust = checked.get(part).map_or((0, 0), |c| c.letters);
            let right = |roots: &[(Cow<'_, str>, char)]| {
                let marks = marks(rules, roots, part, trust);
                flexion::restore(orthography, &marks, settings.smoothing, key, key).0 == letters
            };
            (right(roots), right(&[]))
        });
        for (w, (rooted, bare)) in tested.iter().zip(spelled) {
            if let Some(c) = checked.get_mut(w.2) {
                c.fixed += u32::from(rooted && !bare);
                c.broken += u32::from(bare && !rooted);
            }
        }
    }
    checked
}

#[cfg(test)]
mod tests {
    use super::*;

    const LATIN: Orthography<'static> = Orthography {
        lowercase: true,
        vowels: "aeiouēī",
        stress_marks: "",
        stressed: &[],
        implicit: "",
        dropped: "",
        mark_monosyllables: false,
        folded: &[('ē', 'e'), ('ī', 'i')],
        prosody: None,
        initial: "",
    };

    fn voter(owner: usize, part: &'static str, spellings: &[&str]) -> Voter<'static> {
        Voter {
            owner,
            part,
            key: LATIN.key(spellings.first().copied().unwrap_or_default()),
            lemma: spellings.first().copied().unwrap_or_default().to_string(),
            spellings: spellings.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    #[test]
    fn a_mark_the_letters_leave_open_is_told_by_the_part_of_speech() {
        // Latin adjectives in `-genus` ("born") have a short `e`, the
        // distributive numerals in `-gēnus` a long one: the same letters,
        // told apart by the part of speech alone.
        let mut words: Vec<(String, String, &str)> = Vec::new();
        for front in [
            "indi", "terri", "capri", "nubi", "ali", "primi", "uni", "multi",
        ] {
            let key = format!("{front}genus");
            words.push((key.clone(), key, "adjective"));
        }
        for front in [
            "quin", "septin", "non", "octin", "quadrin", "tre", "du", "ses",
        ] {
            words.push((format!("{front}genus"), format!("{front}gēnus"), "numeral"));
        }
        let words: Vec<(&str, &str, &str)> = words
            .iter()
            .map(|(k, l, p)| (k.as_str(), l.as_str(), *p))
            .collect();
        let marks = learn(&LATIN, &words, &[], Settings::default()).expect("marks");
        let spell =
            |word: &str, part: &str| flexion::restore(&LATIN, &marks.of(part), 1.0, word, word).0;
        assert_eq!(spell("sescentigenus", "numeral"), "sescentigēnus");
        assert_eq!(spell("sescentigenus", "adjective"), "sescentigenus");
    }

    #[test]
    fn a_root_is_told_by_every_form_of_its_words_once_each() {
        // The rules write a short `i` after `v` (`video`, `vinco`); `vīvō`'s
        // and `vīvus`'s forms write `vīv-` long. The root `vivi` (of the
        // forms `vīvis`, `vīvit`, `vīvī`) is listed, though no lemma begins
        // so; `fīgō`'s many forms are one word's, which alone decides
        // nothing; words whose forms write `sali-` both ways tell nothing of
        // `salīmu-`; where the words don't decide, the headwords do.
        let words: Vec<(&str, &str, &str)> = ["video", "vinco", "vigeo", "libet", "dico"]
            .into_iter()
            .map(|w| (w, w, "verb"))
            .collect();
        let rules = choose::learn_starts(&examples(&LATIN, &words).0, Settings::default());
        let voters = [
            voter(0, "verb", &["vīvo", "vīvis", "vīvit"]),
            voter(1, "adjective", &["vīvus", "vīvī"]),
            voter(2, "verb", &["fīgo", "fīgis", "fīgit", "fīgimus"]),
            voter(3, "verb", &["salio", "salīmus"]),
            voter(4, "verb", &["salit", "salīmur"]),
            // `perstīllō` and its participle, listed as a headword of its
            // own: one word's vote, but two headwords.
            voter(5, "verb", &["perstīllo", "perstīllat"]),
            voter(5, "adjective", &["perstīllans", "perstīllantis"]),
        ];
        let roots = roots(&LATIN, voters.iter(), &rules, 1.0);
        let has = |r: &str| roots.iter().any(|(root, v)| root == r && *v == 'ī');
        assert!(has("vivi"), "{roots:?}");
        assert!(!has("figi"), "{roots:?}");
        assert!(!has("salimu"), "{roots:?}");
        assert!(has("perstill"), "{roots:?}");
    }

    #[test]
    fn a_bare_letter_whose_length_is_unstated_casts_no_vote() {
        // Greek writes a short `α` `ᾰ` where it tells its length: the two
        // words writing `κᾰλο-` decide the root, `καλου` leaving it bare
        // says nothing against them.
        let greek = Orthography {
            vowels: "αᾰᾱ",
            folded: &[('ᾰ', 'α'), ('ᾱ', 'α')],
            prosody: Some(flexion::Prosody {
                diphthongs: &[],
                hiatus: "",
                long: "ᾱ",
                short_finals: &[],
                short_endings: &[],
                acute: 0,
                circumflex: 1,
            }),
            ..LATIN
        };
        let rules = choose::learn_starts(&[("#καλ".to_string(), 'α')], Settings::default());
        let voters: Vec<Voter<'_>> = ["κᾰλος", "κᾰλον", "καλη", "καλα", "καλου"]
            .into_iter()
            .enumerate()
            .map(|(owner, s)| Voter {
                owner,
                part: "noun",
                key: greek.key(s),
                lemma: s.to_string(),
                spellings: vec![s.to_string()],
            })
            .collect();
        let roots = roots(&greek, voters.iter(), &rules, 1.0);
        assert_eq!(roots, vec![(Cow::Borrowed("καλο"), 'ᾰ')]);
    }
}
