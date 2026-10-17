//! The language description a config provides. Everything language-specific
//! lives here; the trainer reads only this.

use crate::kaikki::Entry;

/// A whole language: what to read from Wiktionary and what crate to write.
pub struct Language {
    /// Crate name of the generated crate (`english`).
    pub name: &'static str,
    /// Human name (`English`).
    pub display: &'static str,
    /// Kaikki `lang_code` (`en`).
    pub wiktionary: &'static str,
    /// Version of the generated crate.
    pub version: &'static str,
    /// One-line crate description.
    pub description: &'static str,
    pub features: Vec<Feature>,
    pub parts: Vec<PartOfSpeech>,
    pub spelling: Spelling,
    pub tags: TagPolicy,
    pub hooks: Box<dyn Hooks>,
    /// Hand-written Rust copied into the generated crate as `src/extra.rs`
    /// and re-exported (helpers the data can't provide).
    pub extra: Option<&'static str>,
    /// Markdown inserted at the top of the generated README, after the title.
    pub readme_intro: &'static str,
    /// Made-up words to show in the evaluation report: `(pos, cell group, word)`.
    pub nonce: Vec<(&'static str, &'static str, &'static str)>,
    /// Other Wiktionary editions to learn from, after the main dump (which is
    /// the English Wiktionary's). [`Hooks::adapt`] reads their entries.
    pub editions: Vec<Edition>,
}

/// Another Wiktionary edition's entries for the language (the Russian
/// Wiktionary's Russian entries), from its own Kaikki dump.
pub struct Edition {
    /// Kaikki's edition code: `ru` for ru.wiktionary.org.
    pub code: &'static str,
    /// For the report: `the Russian Wiktionary`.
    pub name: &'static str,
    /// The language's name in that edition, which names Kaikki's dump
    /// (`https://kaikki.org/ruwiktionary/Русский/kaikki.org-dictionary-Русский.jsonl`).
    pub dump: &'static str,
    pub merge: Merge,
}

/// What an edition adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Merge {
    /// Only words the main dump doesn't have.
    NewWords,
    /// Every word: a word the main dump has gets the edition's entry as one
    /// more homograph, after the main dump's.
    All,
}

impl Edition {
    /// The dump's file name: Kaikki's drops the spaces of the language's
    /// name (`Grec ancien` → `Grecancien`).
    pub fn file(&self) -> String {
        format!("kaikki.org-dictionary-{}.jsonl", self.dump.replace(' ', ""))
    }
}

/// A grammatical feature and its values.
pub struct Feature {
    pub name: &'static str,
    pub doc: &'static str,
    pub values: Vec<Value>,
    /// A form whose tags name no value of this feature applies to every value
    /// (Russian plural adjective forms carry no gender; accusative-only
    /// animacy).
    pub unspecified_means_all: bool,
}

/// One feature value and the Wiktionary tags that mean it.
pub struct Value {
    pub name: &'static str,
    /// Short name for cell names (`gen`, `pl`).
    pub short: &'static str,
    /// Tags that must all be present.
    pub all: Vec<&'static str>,
    /// Tags that must be absent.
    pub none: Vec<&'static str>,
}

/// Where an entry's forms come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormSource {
    /// Every form in the entry (English: head-template forms).
    All,
    /// Only inflection-table forms (`source` set), when the entry has a table.
    Tables,
}

/// A part of speech: the Wiktionary parts of speech it reads and the cell
/// groups it generates.
#[derive(Clone)]
pub struct PartOfSpeech {
    pub name: &'static str,
    pub doc: &'static str,
    /// Kaikki `pos` values read into this part of speech.
    pub wiktionary: Vec<&'static str>,
    /// When non-empty: only entries whose inflection template is one of these
    /// (a name ending in `*` matches by prefix: `pl-decl-noun*`).
    pub templates: Vec<&'static str>,
    pub forms: FormSource,
    pub groups: Vec<Group>,
    /// The cell the lemma itself fills: `(group, values)`. `None` when the
    /// lemma is no form of the word (Sanskrit nouns are cited by their stem).
    pub lemma: Option<(&'static str, Vec<&'static str>)>,
    /// Principal parts after the lemma, in dictionary order.
    pub citation: Vec<(&'static str, Vec<&'static str>)>,
    /// Features of the word itself (gender), read from the entry and predicted.
    pub inherent: Vec<&'static str>,
    /// Every word is written with a capital (German nouns): a lemma typed in
    /// lowercase still gets capitalized forms.
    pub capitalized: bool,
    /// Groups of other parts of speech whose forms are lemmas of this one,
    /// written in full: `(part of speech, group)`. Russian adjectives read the
    /// verb's `participle` group, so a typed `ахнувший` reads as the verb
    /// table writes it, `а́хнувший`.
    pub lemmas_from: Vec<(&'static str, &'static str)>,
    /// The most words the grammar's structure (its paradigms, relations and
    /// sound laws, learned form by form) is learned from, the commonest (most
    /// senses) first, where the dictionary has far more words than that
    /// needs (Durrett & DeNero 2013 fn. 6: the first 6,000 Finnish noun
    /// tables did as well as all of them). Every word still takes a paradigm,
    /// teaches the rules that choose one, and lists what the grammar gets
    /// wrong. `None`: every word.
    pub max_words: Option<usize>,
    /// A part of speech whose words it learns from too, where its own are
    /// few and inflect as those do (Turkish adjectives decline as nouns):
    /// the other's words teach it, its own are what it is scored on. The
    /// cells must be the other's.
    pub learns_from: Option<&'static str>,
}

impl PartOfSpeech {
    /// Whether it is a closed class: every Wiktionary part of speech it
    /// reads is one (pronouns, determiners, articles, numerals,
    /// adpositions, conjunctions, particles). Its words are few and mostly
    /// their own: it is judged on them (see [`crate::pipeline::small`]).
    pub fn closed(&self) -> bool {
        const CLOSED: [&str; 8] = [
            "pron", "det", "article", "num", "postp", "prep", "conj", "particle",
        ];
        !self.wiktionary.is_empty() && self.wiktionary.iter().all(|p| CLOSED.contains(p))
    }
}

/// A group of cells that is one generated function: the product of its
/// features' values.
#[derive(Clone)]
pub struct Group {
    /// Function name in the generated crate.
    pub name: &'static str,
    pub doc: &'static str,
    pub features: Vec<&'static str>,
    /// Tags every form of the group carries.
    pub all: Vec<&'static str>,
    /// At least one of these (when non-empty).
    pub any: Vec<&'static str>,
    /// None of these.
    pub none: Vec<&'static str>,
    /// The group's forms are on the entry's head line rather than in its
    /// inflection table (Latin comparatives).
    pub from_head: bool,
}

impl Group {
    /// Mark the group as read from the head line (see [`Group::from_head`]).
    pub fn from_head(mut self) -> Self {
        self.from_head = true;
        self
    }
}

/// Spelling conventions.
#[derive(Debug, Clone, Default)]
pub struct Spelling {
    pub fold_case: bool,
    /// The combining marks that write stress, one per kind (Russian the
    /// acute; Ancient Greek acute and circumflex). Precomposed letters that
    /// carry them are found from Unicode. Empty: the data writes no stress.
    pub stress_marks: &'static str,
    /// Marks read as a stress mark: `(mark, as)` (Greek grave as acute).
    pub stress_aliases: Vec<(char, char)>,
    pub mark_monosyllables: bool,
    /// Letters that carry a syllable. Letters made of one of these plus
    /// marks this spelling folds or drops are vowels too.
    pub vowels: &'static str,
    pub implicit: &'static str,
    pub fold: Vec<(char, char)>,
    /// Combining marks lookup ignores: every letter written with one folds
    /// to the letter without it (Greek breathings, iota subscript).
    pub fold_marks: &'static str,
    pub strip_marks: &'static str,
    /// Combining marks written only on a word's first letter or first vowels
    /// (Greek breathings): written after a consonant of the word (a
    /// dictionary's compound, `ὑπἔκρυπτον`), they are dropped.
    pub initial_marks: &'static str,
    /// Letters written plain when the caller turns marks off (Latin
    /// `ā` → `a`). Lookup ignores them too.
    pub unmark: Vec<(char, char)>,
    /// Combining marks the marks setting drops (Greek length marks): every
    /// letter written with one is written without it. Lookup ignores them.
    pub unmark_marks: &'static str,
    /// Letters that carry an inherent vowel (Devanagari consonants), and the
    /// marks that mute it (the virama): for syllable counts.
    pub syllabic: &'static str,
    pub mute: &'static str,
    /// Pairs of vowels that make one syllable (Greek `αι`, `ευ`), for stress
    /// counted in syllables from the word's end; lowercase, unmarked.
    pub diphthongs: Vec<&'static str>,
    /// The mark that keeps two vowels apart (the diaeresis): letters with it
    /// never close a diphthong.
    pub hiatus_mark: Option<char>,
    /// Vowels that are always long (Greek `η`, `ω`); set for a language with
    /// a law of limitation (see [`flexion::Prosody`]).
    pub long_vowels: &'static str,
    /// Marks that make a vowel long (the macron, the iota subscript).
    pub long_marks: &'static str,
    /// Marks that state a vowel's length both ways, long and short (Greek
    /// macron and breve). Where the dictionary writes short vowels marked
    /// too, a vowel it writes with neither has its length unstated: a
    /// second score, compatible with stated length, accepts an answer that
    /// marks it where the word's table doesn't state it otherwise (see
    /// [`crate::eval::TableLength`]). Empty: a bare vowel is short, or
    /// length isn't written.
    pub stated_length: &'static str,
    /// Diphthongs short at a word's end (`αι`, `οι`).
    pub short_finals: Vec<&'static str>,
    /// Endings whose last syllable counts short, long as it is (Greek
    /// `-εως`, `-εων`: `πόλεως`, Smyth §163a).
    pub short_endings: Vec<&'static str>,
    /// The stress mark that can only stand on a long vowel (the circumflex):
    /// a vowel read with it is read long (`ᾶ` as `ᾱ`).
    pub circumflex: Option<char>,
    /// Name of the generated setting that turns stress marks off (`stress`,
    /// `accents`); empty for no setting.
    pub stress_setting: &'static str,
    /// Name of the generated setting that writes marked letters plain
    /// (`macrons`); empty for no setting.
    pub marks_setting: &'static str,
}

/// How form tags rank forms.
pub struct TagPolicy {
    /// Forms carrying any of these are dropped.
    pub exclude: Vec<&'static str>,
    /// Real but nonstandard forms (dialectal, rare…): never trained on.
    pub soft: Vec<&'static str>,
    /// Tags of bookkeeping forms that are never cell forms.
    pub meta: Vec<&'static str>,
    /// Form strings that mean "no form" or "unknown".
    pub sentinels: Vec<&'static str>,
}

impl Default for TagPolicy {
    fn default() -> Self {
        TagPolicy {
            exclude: vec![
                "obsolete",
                "archaic",
                "error-unknown-tag",
                "error-unrecognized-form",
            ],
            soft: vec![],
            meta: vec![
                "table-tags",
                "inflection-template",
                "class",
                "romanization",
                "canonical",
            ],
            sentinels: vec!["-", "—", "–", "dubious", ""],
        }
    }
}

/// Editorial quirks the declarative part can't express.
pub trait Hooks: Sync + Send {
    /// Whether this lemma is trained on at all.
    fn lemma_ok(&self, _pos: &str, _lemma: &str) -> bool {
        true
    }
    /// Whether this form of `lemma` is admissible.
    fn form_ok(&self, _pos: &str, _lemma: &str, _form: &str) -> bool {
        true
    }
    /// Fix an entry before it is read: tags a dump gets wrong, forms in the
    /// wrong place. Runs once per entry and part of speech.
    fn repair(&self, _pos: &str, _entry: &mut Entry) {}
    /// Extra tags describing the word itself (e.g. gender parsed from the
    /// head template when the forms don't carry it).
    fn inherent_tags(&self, _entry: &Entry) -> Vec<String> {
        Vec::new()
    }
    /// Read an entry of another edition (`edition` is its
    /// [`Edition::code`]) for a part of speech: turn it into what
    /// [`Hooks::repair`] makes of a main-dump entry (table forms with
    /// `source` set and the main dump's tags), or return false to skip it.
    /// It replaces the template check and `repair` for that edition.
    fn adapt(&self, _edition: &str, _pos: &str, _entry: &mut Entry) -> bool {
        false
    }
}

impl Language {
    pub fn feature(&self, name: &str) -> Option<&Feature> {
        self.features.iter().find(|f| f.name == name)
    }

    /// Check internal consistency: every name refers to something that exists.
    pub fn validate(&self) -> Result<(), String> {
        for p in &self.parts {
            let unknown =
                |fs: &[&'static str]| fs.iter().find(|f| self.feature(f).is_none()).copied();
            for g in &p.groups {
                if let Some(f) = unknown(&g.features) {
                    return Err(format!(
                        "{}: group {} uses unknown feature {f}",
                        p.name, g.name
                    ));
                }
            }
            if let Some(f) = unknown(&p.inherent) {
                return Err(format!("{}: unknown inherent feature {f}", p.name));
            }
            let cells = crate::cells::Cells::new(self, p)?;
            if let Some((g, v)) = &p.lemma {
                cells
                    .find(g, v)
                    .ok_or(format!("{}: lemma cell not found", p.name))?;
            }
            for (g, v) in &p.citation {
                cells
                    .find(g, v)
                    .ok_or(format!("{}: citation cell {g} {v:?} not found", p.name))?;
            }
            if let Some(h) = p.learns_from {
                let host = self
                    .parts
                    .iter()
                    .find(|q| q.name == h)
                    .ok_or(format!("{}: learns from no part {h}", p.name))?;
                if crate::cells::Cells::new(self, host)?.total != cells.total {
                    return Err(format!("{}: learns from {h}, whose cells differ", p.name));
                }
            }
        }
        Ok(())
    }
}

/// Declare features compactly:
///
/// ```ignore
/// features! {
///     Case "Grammatical case." {
///         Nominative "nom" ["nominative"],
///         Genitive "gen" ["genitive"],
///     }
///     Animacy "Animacy." (all) {
///         Animate "anim" ["animate"],
///         Inanimate "inan" ["inanimate"],
///     }
///     Agreement "Gender or plural." {
///         Masculine "m" ["masculine"] !["plural"],
///         Plural "pl" ["plural"],
///     }
/// }
/// ```
///
/// `(all)` marks a feature whose absence from a form's tags means the form
/// fills every value. `!["tag"]` lists tags a value must not carry.
#[macro_export]
macro_rules! features {
    ($(
        $fname:ident $fdoc:literal $(($all:ident))? {
            $( $vname:ident $short:literal [$($tag:literal),*] $(! [$($not:literal),*])? ),* $(,)?
        }
    )*) => {
        vec![$(
            $crate::spec::Feature {
                name: stringify!($fname),
                doc: $fdoc,
                unspecified_means_all: $crate::features!(@all $($all)?),
                values: vec![$(
                    $crate::spec::Value {
                        name: stringify!($vname),
                        short: $short,
                        all: vec![$($tag),*],
                        none: vec![$($($not),*)?],
                    }
                ),*],
            }
        ),*]
    };
    (@all all) => { true };
    (@all) => { false };
}
