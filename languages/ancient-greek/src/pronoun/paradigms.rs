//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative_masculine` (sg, du, pl)
/// `nominative_feminine` (sg, du, pl)
/// `nominative_neuter` (sg, du, pl)
/// `genitive_masculine` (sg, du, pl)
/// `genitive_feminine` (sg, du, pl)
/// `genitive_neuter` (sg, du, pl)
/// `dative_masculine` (sg, du, pl)
/// `dative_feminine` (sg, du, pl)
/// `dative_neuter` (sg, du, pl)
/// `accusative_masculine` (sg, du, pl)
/// `accusative_feminine` (sg, du, pl)
/// `accusative_neuter` (sg, du, pl)
/// `vocative_masculine` (sg, du, pl)
/// `vocative_feminine` (sg, du, pl)
/// `vocative_neuter` (sg, du, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative_masculine: $c0:tt, $c1:tt, $c2:tt;
     nominative_feminine: $c3:tt, $c4:tt, $c5:tt;
     nominative_neuter: $c6:tt, $c7:tt, $c8:tt;
     genitive_masculine: $c9:tt, $c10:tt, $c11:tt;
     genitive_feminine: $c12:tt, $c13:tt, $c14:tt;
     genitive_neuter: $c15:tt, $c16:tt, $c17:tt;
     dative_masculine: $c18:tt, $c19:tt, $c20:tt;
     dative_feminine: $c21:tt, $c22:tt, $c23:tt;
     dative_neuter: $c24:tt, $c25:tt, $c26:tt;
     accusative_masculine: $c27:tt, $c28:tt, $c29:tt;
     accusative_feminine: $c30:tt, $c31:tt, $c32:tt;
     accusative_neuter: $c33:tt, $c34:tt, $c35:tt;
     vocative_masculine: $c36:tt, $c37:tt, $c38:tt;
     vocative_feminine: $c39:tt, $c40:tt, $c41:tt;
     vocative_neuter: $c42:tt, $c43:tt, $c44:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
                flexion::form!($c3),
                flexion::form!($c4),
                flexion::form!($c5),
                flexion::form!($c6),
                flexion::form!($c7),
                flexion::form!($c8),
                flexion::form!($c9),
                flexion::form!($c10),
                flexion::form!($c11),
                flexion::form!($c12),
                flexion::form!($c13),
                flexion::form!($c14),
                flexion::form!($c15),
                flexion::form!($c16),
                flexion::form!($c17),
                flexion::form!($c18),
                flexion::form!($c19),
                flexion::form!($c20),
                flexion::form!($c21),
                flexion::form!($c22),
                flexion::form!($c23),
                flexion::form!($c24),
                flexion::form!($c25),
                flexion::form!($c26),
                flexion::form!($c27),
                flexion::form!($c28),
                flexion::form!($c29),
                flexion::form!($c30),
                flexion::form!($c31),
                flexion::form!($c32),
                flexion::form!($c33),
                flexion::form!($c34),
                flexion::form!($c35),
                flexion::form!($c36),
                flexion::form!($c37),
                flexion::form!($c38),
                flexion::form!($c39),
                flexion::form!($c40),
                flexion::form!($c41),
                flexion::form!($c42),
                flexion::form!($c43),
                flexion::form!($c44),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `ὁσος`: 16 words.
    Ὁσος,
    /// Like `οἱος`: 6 words.
    Οἱος,
    /// Like `ἑαυτοῡ`: 6 words.
    Ἑαυτοῡ,
    /// Like `τοῑος`: 4 words.
    Τοῑος,
    /// Like `ἐμαυτοῡ`: 3 words.
    Ἐμαυτοῡ,
    /// Like `σφω`: 2 words.
    Σφω,
    /// Like `ὁ`: 1 words.
    Ὁ,
    /// Like `ἐγω`: 1 words.
    Ἐγω,
    /// Like `σῠ`: 1 words.
    Σῠ,
    /// Like `τῐς`: 1 words.
    Τῐς,
    /// Like `ὁς`: 1 words.
    Ὁς,
    /// Like `ὁστῐς`: 1 words.
    Ὁστῐς,
    /// Like `οὐδεις`: 1 words.
    Οὐδεις,
    /// Like `ἱ`: 1 words.
    Ἱ,
    /// Like `ἡμεῑς`: 1 words.
    Ἡμεῑς,
    /// Like `ὑμεῑς`: 1 words.
    Ὑμεῑς,
    /// Like `πᾱς`: 1 words.
    Πᾱς,
    /// Like `οὑ`: 1 words.
    Οὑ,
    /// Like `ἁπας`: 1 words.
    Ἁπας,
    /// Like `ὁδε`: 1 words.
    Ὁδε,
    /// Like `μηδεις`: 1 words.
    Μηδεις,
    /// Like `ἀλληλων`: 1 words.
    Ἀλληλων,
    /// Like `ὁσπερ`: 1 words.
    Ὁσπερ,
    /// Like `σφεῑς`: 1 words.
    Σφεῑς,
    /// Like `ἀμφω`: 1 words.
    Ἀμφω,
    /// Like `ἐνῐοι`: 1 words.
    Ἐνῐοι,
    /// Like `ὁσᾰπλᾰσῐων`: 1 words.
    Ὁσᾰπλᾰσῐων,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 27] = [
        Declension::Ὁσος,
        Declension::Οἱος,
        Declension::Ἑαυτοῡ,
        Declension::Τοῑος,
        Declension::Ἐμαυτοῡ,
        Declension::Σφω,
        Declension::Ὁ,
        Declension::Ἐγω,
        Declension::Σῠ,
        Declension::Τῐς,
        Declension::Ὁς,
        Declension::Ὁστῐς,
        Declension::Οὐδεις,
        Declension::Ἱ,
        Declension::Ἡμεῑς,
        Declension::Ὑμεῑς,
        Declension::Πᾱς,
        Declension::Οὑ,
        Declension::Ἁπας,
        Declension::Ὁδε,
        Declension::Μηδεις,
        Declension::Ἀλληλων,
        Declension::Ὁσπερ,
        Declension::Σφεῑς,
        Declension::Ἀμφω,
        Declension::Ἐνῐοι,
        Declension::Ὁσᾰπλᾰσῐων,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        DECLENSIONS.get(self as usize)
    }
}

impl flexion::Label for Declension {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static DECLENSIONS: [Paradigm<'static>; 27] = [
    // Ὁσος: 16 words (αὐτος, οὑτος, ἑος, ἐκεῑνος, ἑκᾰστος, ταὐτος).
    declension!("ὁσος", 16, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ης", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ῃ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Οἱος: 6 words (οἱος, ἑκᾰτερος, οὐδετερος, ὁποτερος, μηδετερος, αὐτοτερος).
    declension!("οἱος", 6, "~ος";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~ᾱ", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ᾱς", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ᾳ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ᾱν", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~ᾱ", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Ἑαυτοῡ: 6 words (ἑαυτοῡ, ἑωυτοῡ, αὑτοῡ, σεαυτοῡ, σεωυτοῡ, σαυτοῡ).
    declension!("ἑαυτοῡ", 6, "~οῡ";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ης", "~αῑν", "~εων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ῃ", "~αῑν", "~ῃσῐ";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ο", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Τοῑος: 4 words (ποῑος, τοῑος, πᾰντοῑος, ὁποῑος).
    declension!("τοῑος", 4, "~ῑος";
        nominative_masculine: "~ῑος", "~ιω", "~ῑοι";
        nominative_feminine: "~ιᾱ", "~ιᾱ", "~ῑαι";
        nominative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        genitive_masculine: "~ιου", "~ιοιν", "~ιων";
        genitive_feminine: "~ιᾱς", "~ιαιν", "~ιων";
        genitive_neuter: "~ιου", "~ιοιν", "~ιων";
        dative_masculine: "~ιῳ", "~ιοιν", "~ιοις";
        dative_feminine: "~ιᾳ", "~ιαιν", "~ιαις";
        dative_neuter: "~ιῳ", "~ιοιν", "~ιοις";
        accusative_masculine: "~ῑον", "~ιω", "~ιους";
        accusative_feminine: "~ιᾱν", "~ιᾱ", "~ιᾱς";
        accusative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
        vocative_masculine: "~ῑε", "~ιω", "~ῑοι";
        vocative_feminine: "~ιᾱ", "~ιᾱ", "~ῑαι";
        vocative_neuter: "~ῑον", "~ιω", "~ῑᾰ";
    ),
    // Ἐμαυτοῡ: 3 words (ἐμαυτοῡ, ἐμεωυτοῡ, ἐμωυτοῡ).
    declension!("ἐμαυτοῡ", 3, "~οῡ";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~οῡ", "~οῑν", "~ων";
        genitive_feminine: "~ης", "~αῑν", "~εων";
        genitive_neuter: "~οῡ", "~οῑν", "~ων";
        dative_masculine: "~ῳ", "~οῑν", "~οῑς";
        dative_feminine: "~ῃ", "~αῑν", "~ῃσῐ";
        dative_neuter: "~ῳ", "~οῑν", "~οῑς";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Σφω: 2 words (νω, σφω).
    declension!("σφω", 2, "~ω";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ω", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~ῳν", "~ων";
        genitive_feminine: "~ης", "~ῳν", "~ων";
        genitive_neuter: "~ου", "~ῳν", "~ων";
        dative_masculine: "~ῳ", "~ῳν", "~οις";
        dative_feminine: "~ῃ", "~ῳν", "~αις";
        dative_neuter: "~ῳ", "~ῳν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ω", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ω", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Ὁ: 1 words (ὁ).
    declension!("ὁ", 1, "ὁ";
        nominative_masculine: "ὁ", "τω", "οἱ";
        nominative_feminine: "ἡ", "τω", "αἱ";
        nominative_neuter: "το", "τω", "τᾰ";
        genitive_masculine: "τοῡ", "τοῑν", "των";
        genitive_feminine: "της", "τοῑν", "των";
        genitive_neuter: "τοῡ", "τοῑν", "των";
        dative_masculine: "τῳ", "τοῑν", "τοῑς";
        dative_feminine: "τῃ", "τοῑν", "ταῑς";
        dative_neuter: "τῳ", "τοῑν", "τοῑς";
        accusative_masculine: "τον", "τω", "τους";
        accusative_feminine: "την", "τω", "τᾱς";
        accusative_neuter: "το", "τω", "τᾰ";
        vocative_masculine: "τᾱν", "ὁ", "σφεῑς";
        vocative_feminine: "τᾱν", "ὁ", "σφεῑς";
        vocative_neuter: "τᾱν", "ὁ", "σφεῑς";
    ),
    // Ἐγω: 1 words (ἐγω).
    declension!("ἐγω", 1, "ἐγω";
        nominative_masculine: "ἐγω", "ὡ", "σφεῑς";
        nominative_feminine: "ἐγω", "ᾱ\u{314}", "σφεῑς";
        nominative_neuter: "ἐγω", "ὡ", "σφεῑς";
        genitive_masculine: "ἐμοῡ|μου", "οἱν", "σφων";
        genitive_feminine: "ἐμοῡ|μου", "αἱν", "σφων";
        genitive_neuter: "ἐμοῡ|μου", "οἱν", "σφων";
        dative_masculine: "ἐμοι|μοι", "οἱν", "σφῐσῐ";
        dative_feminine: "ἐμοι|μοι", "αἱν", "σφῐσῐ";
        dative_neuter: "ἐμοι|μοι", "οἱν", "σφῐσῐ";
        accusative_masculine: "ἐμε|με", "ὡ", "σφᾱς";
        accusative_feminine: "ἐμε|με", "ᾱ\u{314}", "σφᾱς";
        accusative_neuter: "ἐμε|με", "ὡ", "σφᾱς";
        vocative_masculine: "τᾱν", "ἐγω", "σφεῑς";
        vocative_feminine: "τᾱν", "ἐγω", "σφεῑς";
        vocative_neuter: "τᾱν", "ἐγω", "σφεῑς";
    ),
    // Σῠ: 1 words (σῠ).
    declension!("σῠ", 1, "σῠ";
        nominative_masculine: "σῠ", "ὡ", "σφεῑς";
        nominative_feminine: "σῠ", "ᾱ\u{314}", "σφεῑς";
        nominative_neuter: "σῠ", "ὡ", "σφεῑς";
        genitive_masculine: "σοῡ|σου", "οἱν", "σφων";
        genitive_feminine: "σοῡ|σου", "αἱν", "σφων";
        genitive_neuter: "σοῡ|σου", "οἱν", "σφων";
        dative_masculine: "σοι|σοι", "οἱν", "σφῐσῐ";
        dative_feminine: "σοι|σοι", "αἱν", "σφῐσῐ";
        dative_neuter: "σοι|σοι", "οἱν", "σφῐσῐ";
        accusative_masculine: "σε|σε", "ὡ", "σφᾱς";
        accusative_feminine: "σε|σε", "ᾱ\u{314}", "σφᾱς";
        accusative_neuter: "σε|σε", "ὡ", "σφᾱς";
        vocative_masculine: "τᾱν", "σῠ", "σφεῑς";
        vocative_feminine: "τᾱν", "σῠ", "σφεῑς";
        vocative_neuter: "τᾱν", "σῠ", "σφεῑς";
    ),
    // Τῐς: 1 words (τῐς).
    declension!("τῐς", 1, "~ῐς";
        nominative_masculine: "~ῐς", "~ῐνε|~ῐνε", "~ῐνες|~ῐνες";
        nominative_feminine: "~ῐς", "~ῐνε|~ῐνε", "~ῐνες|~ῐνες";
        nominative_neuter: "~ῐ", "~ῐνε|~ῐνε", "~ῐνᾰ|~ῐνᾰ|ᾰ\u{313}~τᾰ";
        genitive_masculine: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        genitive_feminine: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        genitive_neuter: "~ῐνος|~ῐνος|~ου", "~ῐνοιν|~ῐνοῑν", "~ῐνων|~ῐνων";
        dative_masculine: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        dative_feminine: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        dative_neuter: "~ῐνῐ|~ῐνῐ|~ῳ", "~ῐνοιν|~ῐνοῑν", "~ῐσῐ|~ῐσῐν|~ῐσῐ|~ῐσῐν";
        accusative_masculine: "~ῐνᾰ|~ῐνᾰ", "~ῐνε|~ῐνε", "~ῐνᾰς|~ῐνᾰς";
        accusative_feminine: "~ῐνᾰ|~ῐνᾰ", "~ῐνε|~ῐνε", "~ῐνᾰς|~ῐνᾰς";
        accusative_neuter: "~ῐ", "~ῐνε|~ῐνε", "~ῐνᾰ|~ῐνᾰ|ᾰ\u{313}~τᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Ὁς: 1 words (ὁς).
    declension!("ὁς", 1, "ὁς";
        nominative_masculine: "ὁς", "ὡ", "οἱ";
        nominative_feminine: "ἡ", "ᾱ\u{314}", "αἱ";
        nominative_neuter: "ὁ", "ὡ", "ᾰ\u{314}";
        genitive_masculine: "οὑ", "οἱν", "ὡν";
        genitive_feminine: "ἡς", "αἱν", "ὡν";
        genitive_neuter: "οὑ", "οἱν", "ὡν";
        dative_masculine: "ᾡ", "οἱν", "οἱς";
        dative_feminine: "ᾑ", "αἱν", "αἱς";
        dative_neuter: "ᾡ", "οἱν", "οἱς";
        accusative_masculine: "ὁν", "ὡ", "οὑς";
        accusative_feminine: "ἡν", "ᾱ\u{314}", "ᾱ\u{314}ς";
        accusative_neuter: "ὁ", "ὡ", "ᾰ\u{314}";
        vocative_masculine: "τᾱν", "ὁς", "σφεῑς";
        vocative_feminine: "τᾱν", "ὁς", "σφεῑς";
        vocative_neuter: "τᾱν", "ὁς", "σφεῑς";
    ),
    // Ὁστῐς: 1 words (ὁστῐς).
    declension!("ὁστῐς", 1, "ὁσ~ῐς";
        nominative_masculine: "ὁσ~ῐς", "ὡ~ῐνε", "οἱ~ῐνες";
        nominative_feminine: "ἡ~ῐς", "ᾱ\u{314}~ῐνε", "αἱ~ῐνες";
        nominative_neuter: "~ον", "ὡ~ῐνε", "ᾰ\u{314}~ῐνᾰ|ᾰ\u{314}~τᾰ";
        genitive_masculine: "οὑ~ῐνος|ὁ~ου", "οἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
        genitive_feminine: "ἡσ~ῐνος", "αἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
        genitive_neuter: "οὑ~ῐνος|ὁ~ου", "οἱν~ῐνοιν", "ὡν~ῐνων|ὁ~ων";
        dative_masculine: "ᾡ~ῐνῐ|ὁ~ῳ", "οἱν~ῐνοιν", "οἱσ~ῐσῐ|οἱσ~ῐσῐν|ὁ~οις";
        dative_feminine: "ᾑ~ῐνῐ", "αἱν~ῐνοιν", "αἱσ~ῐσῐ|αἱσ~ῐσῐν";
        dative_neuter: "ᾡ~ῐνῐ|ὁ~ῳ", "οἱν~ῐνοιν", "οἱσ~ῐσῐ|οἱσ~ῐσῐν|ὁ~οις";
        accusative_masculine: "ὁν~ῐνᾰ", "ὡ~ῐνε", "οὑσ~ῐνᾰς";
        accusative_feminine: "ἡν~ῐνᾰ", "ᾱ\u{314}~ῐνε", "ᾱ\u{314}σ~ῐνᾰς";
        accusative_neuter: "~ον", "ὡ~ῐνε", "ᾰ\u{314}~ῐνᾰ|ᾰ\u{314}~τᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Οὐδεις: 1 words (οὐδεις).
    declension!("οὐδεις", 1, "~ις";
        nominative_masculine: "~ις", "~νε", "~νες";
        nominative_feminine: "~μιᾰ", "~μιᾱ", "~μιαι";
        nominative_neuter: "~ν", "~νε", "~νᾰ";
        genitive_masculine: "~νος", "~νοιν", "~νων";
        genitive_feminine: "~μιᾱς", "~μιαιν", "~μιων";
        genitive_neuter: "~νος", "~νοιν", "~νων";
        dative_masculine: "~νι", "~νοιν", "~σι|~σιν";
        dative_feminine: "~μιᾳ", "~μιαιν", "~μιαις";
        dative_neuter: "~νι", "~νοιν", "~σι|~σιν";
        accusative_masculine: "~νᾰ", "~νε", "~νᾰς";
        accusative_feminine: "~μιᾰν", "~μιᾱ", "~μῐᾱς";
        accusative_neuter: "~ν", "~νε", "~νᾰ";
        vocative_masculine: "~ις", "~νε", "~νες";
        vocative_feminine: "~μιᾰ", "~μιᾱ", "~μιαι";
        vocative_neuter: "~ν", "~νε", "~νᾰ";
    ),
    // Ἱ: 1 words (ἱ).
    declension!("ἱ", 1, "ἱ";
        nominative_masculine: "ῑ\u{314}|ῑ\u{314}", "ὡ", "σφεῑς";
        nominative_feminine: "ῑ\u{314}|ῑ\u{314}", "ᾱ\u{314}", "σφεῑς";
        nominative_neuter: "ῑ\u{314}|ῑ\u{314}", "ὡ", "σφεῑς";
        genitive_masculine: "οὑ|οῡ\u{314}", "οἱν", "σφων";
        genitive_feminine: "οὑ|οῡ\u{314}", "αἱν", "σφων";
        genitive_neuter: "οὑ|οῡ\u{314}", "οἱν", "σφων";
        dative_masculine: "ῐ\u{314}ν|ῐ\u{314}ν", "οἱν", "σφῐσῐ";
        dative_feminine: "ῐ\u{314}ν|ῐ\u{314}ν", "αἱν", "σφῐσῐ";
        dative_neuter: "ῐ\u{314}ν|ῐ\u{314}ν", "οἱν", "σφῐσῐ";
        accusative_masculine: "ἑ", "ὡ", "σφᾱς";
        accusative_feminine: "ἑ", "ᾱ\u{314}", "σφᾱς";
        accusative_neuter: "ἑ", "ὡ", "σφᾱς";
        vocative_masculine: "τᾱν", "ἱ", "σφεῑς";
        vocative_feminine: "τᾱν", "ἱ", "σφεῑς";
        vocative_neuter: "τᾱν", "ἱ", "σφεῑς";
    ),
    // Ἡμεῑς: 1 words (ἡμεῑς).
    declension!("ἡμεῑς", 1, "ἡ~εῑς";
        nominative_masculine: "~ος", "~ω", "ἡ~εῑς|ᾰ\u{313}~μες|ᾰ\u{313}~μες";
        nominative_feminine: "~η", "~ᾱ", "ἡ~εῑς|ᾰ\u{313}~μες|ᾰ\u{313}~μες";
        nominative_neuter: "~ον", "~ω", "ἡ~εῑς|ᾰ\u{313}~μες|ᾰ\u{313}~μες";
        genitive_masculine: "~ου", "~οιν", "ἡ~ων|ᾰ\u{313}~μεων|ᾰ\u{313}~μεουν";
        genitive_feminine: "~ης", "~αιν", "ἡ~ων|ᾰ\u{313}~μεων|ᾰ\u{313}~μεουν";
        genitive_neuter: "~ου", "~οιν", "ἡ~ων|ᾰ\u{313}~μεων|ᾰ\u{313}~μεουν";
        dative_masculine: "~ῳ", "~οιν", "ἡ~ῑν|ᾰ\u{313}~μῐ|ᾰ\u{313}~μῐν|ᾰ\u{313}~μῐν";
        dative_feminine: "~ῃ", "~αιν", "ἡ~ῑν|ᾰ\u{313}~μῐ|ᾰ\u{313}~μῐν|ᾰ\u{313}~μῐν";
        dative_neuter: "~ῳ", "~οιν", "ἡ~ῑν|ᾰ\u{313}~μῐ|ᾰ\u{313}~μῐν|ᾰ\u{313}~μῐν";
        accusative_masculine: "~ον", "~ω", "ἡ~ᾱς|ᾰ\u{313}~με|ᾰ\u{313}~με";
        accusative_feminine: "~ην", "~ᾱ", "ἡ~ᾱς|ᾰ\u{313}~με|ᾰ\u{313}~με";
        accusative_neuter: "~ον", "~ω", "ἡ~ᾱς|ᾰ\u{313}~με|ᾰ\u{313}~με";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Ὑμεῑς: 1 words (ὑμεῑς).
    declension!("ὑμεῑς", 1, "ὑ~εῑς";
        nominative_masculine: "~ος", "~ω", "ῡ\u{314}~εῑς";
        nominative_feminine: "~η", "~ᾱ", "ῡ\u{314}~εῑς";
        nominative_neuter: "~ον", "~ω", "ῡ\u{314}~εῑς";
        genitive_masculine: "~ου", "~οιν", "ῡ\u{314}~ων";
        genitive_feminine: "~ης", "~αιν", "ῡ\u{314}~ων";
        genitive_neuter: "~ου", "~οιν", "ῡ\u{314}~ων";
        dative_masculine: "~ῳ", "~οιν", "ῡ\u{314}~ῑν";
        dative_feminine: "~ῃ", "~αιν", "ῡ\u{314}~ῑν";
        dative_neuter: "~ῳ", "~οιν", "ῡ\u{314}~ῑν";
        accusative_masculine: "~ον", "~ω", "ῡ\u{314}~ᾱς";
        accusative_feminine: "~ην", "~ᾱ", "ῡ\u{314}~ᾱς";
        accusative_neuter: "~ον", "~ω", "ῡ\u{314}~ᾱς";
        vocative_masculine: "~ε", "~ω", "ῡ\u{314}~εῑς";
        vocative_feminine: "~η", "~ᾱ", "ῡ\u{314}~εῑς";
        vocative_neuter: "~ον", "~ω", "ῡ\u{314}~εῑς";
    ),
    // Πᾱς: 1 words (πᾱς).
    declension!("πᾱς", 1, "~ᾱς";
        nominative_masculine: "~ᾱς", "~ᾰντε", "~ᾰντες";
        nominative_feminine: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
        nominative_neuter: "~ᾱν", "~ᾰντε", "~ᾰντᾰ";
        genitive_masculine: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        genitive_feminine: "~ᾱσης", "~ᾱσαιν", "~ᾱσων";
        genitive_neuter: "~ᾰντος", "~ᾰντοιν", "~ᾰντων";
        dative_masculine: "~ᾰντῐ", "~ᾰντοιν", "~ᾱσῐ|~ᾱσῐν";
        dative_feminine: "~ᾱσῃ", "~ᾱσαιν", "~ᾱσαις";
        dative_neuter: "~ᾰντῐ", "~ᾰντοιν", "~ᾱσῐ|~ᾱσῐν";
        accusative_masculine: "~ᾰντᾰ", "~ᾰντε", "~ᾰντᾰς";
        accusative_feminine: "~ᾱσᾰν", "~ᾱσᾱ", "~ᾱσᾱς";
        accusative_neuter: "~ᾱν", "~ᾰντε", "~ᾰντᾰ";
        vocative_masculine: "~ᾱς", "~ᾰντε", "~ᾰντες";
        vocative_feminine: "~ᾱσᾰ", "~ᾱσᾱ", "~ᾱσαι";
        vocative_neuter: "~ᾱν", "~ᾰντε", "~ᾰντᾰ";
    ),
    // Οὑ: 1 words (οὑ).
    declension!("οὑ", 1, "οὑ";
        nominative_masculine: "ῑ\u{314}", "ὡ", "σφεῑς";
        nominative_feminine: "ῑ\u{314}", "ᾱ\u{314}", "σφεῑς";
        nominative_neuter: "ῑ\u{314}", "ὡ", "σφεῑς";
        genitive_masculine: "οὑ|οὑ", "οἱν", "σφων";
        genitive_feminine: "οὑ|οὑ", "αἱν", "σφων";
        genitive_neuter: "οὑ|οὑ", "οἱν", "σφων";
        dative_masculine: "οἱ|οἱ", "οἱν", "σφῐσῐ|σφῐσῐν";
        dative_feminine: "οἱ|οἱ", "αἱν", "σφῐσῐ|σφῐσῐν";
        dative_neuter: "οἱ|οἱ", "οἱν", "σφῐσῐ|σφῐσῐν";
        accusative_masculine: "ἑ|ἑ", "ὡ", "σφᾱς";
        accusative_feminine: "ἑ|ἑ", "ᾱ\u{314}", "σφᾱς";
        accusative_neuter: "ἑ|ἑ", "ὡ", "σφᾱς";
        vocative_masculine: "τᾱν", "οὑ", "σφεῑς";
        vocative_feminine: "τᾱν", "οὑ", "σφεῑς";
        vocative_neuter: "τᾱν", "οὑ", "σφεῑς";
    ),
    // Ἁπας: 1 words (ἁπας).
    declension!("ἁπας", 1, "ἁ~ας";
        nominative_masculine: "ᾰ\u{314}~ᾱς", "~ω", "ᾰ\u{314}~ᾰντες";
        nominative_feminine: "ᾰ\u{314}~ᾱσᾰ", "~ᾱ", "ᾰ\u{314}~ᾱσαι";
        nominative_neuter: "ᾰ\u{314}~ᾰν", "~ω", "ᾰ\u{314}~ᾰντᾰ";
        genitive_masculine: "ᾰ\u{314}~ᾰντος", "~οιν", "ᾰ\u{314}~ᾰντων";
        genitive_feminine: "ᾰ\u{314}~ᾱσης", "~αιν", "ᾰ\u{314}~ᾱσων";
        genitive_neuter: "ᾰ\u{314}~ᾰντος", "~οιν", "ᾰ\u{314}~ᾰντων";
        dative_masculine: "ᾰ\u{314}~ᾰντῐ", "~οιν", "ᾰ\u{314}~ᾱσῐ|ᾰ\u{314}~ᾱσῐν";
        dative_feminine: "ᾰ\u{314}~ᾱσῃ", "~αιν", "ᾰ\u{314}~ᾱσαις";
        dative_neuter: "ᾰ\u{314}~ᾰντῐ", "~οιν", "ᾰ\u{314}~ᾱσῐ|ᾰ\u{314}~ᾱσῐν";
        accusative_masculine: "ᾰ\u{314}~ᾰντᾰ", "~ω", "ᾰ\u{314}~ᾰντᾰς";
        accusative_feminine: "ᾰ\u{314}~ᾱσᾰν", "~ᾱ", "ᾰ\u{314}~ᾱσᾱς";
        accusative_neuter: "ᾰ\u{314}~ᾰν", "~ω", "ᾰ\u{314}~ᾰντᾰ";
        vocative_masculine: "ᾰ\u{314}~ᾱς", "~ω", "ᾰ\u{314}~ᾰντες";
        vocative_feminine: "ᾰ\u{314}~ᾱσᾰ", "~ᾱ", "ᾰ\u{314}~ᾱσαι";
        vocative_neuter: "ᾰ\u{314}~ᾰν", "~ω", "ᾰ\u{314}~ᾰντᾰ";
    ),
    // Ὁδε: 1 words (ὁδε).
    declension!("ὁδε", 1, "ὁ~";
        nominative_masculine: "ὁ~", "τω~", "οἱ~";
        nominative_feminine: "ἡ~", "τω~", "αἱ~";
        nominative_neuter: "το~", "τω~", "τᾰ~";
        genitive_masculine: "τοῡ~", "τοῑν~", "των~";
        genitive_feminine: "τησ~", "τοῑν~|ταῑν~", "των~";
        genitive_neuter: "τοῡ~", "τοῑν~", "των~";
        dative_masculine: "τῳ~", "τοῑν~", "τοῑσ~";
        dative_feminine: "τῃ~", "τοῑν~|ταῑν~", "ταῑσ~";
        dative_neuter: "τῳ~", "τοῑν~", "τοῑσ~";
        accusative_masculine: "τον~", "τω~", "τουσ~";
        accusative_feminine: "την~", "τω~", "τᾱσ~";
        accusative_neuter: "το~", "τω~", "τᾰ~";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Μηδεις: 1 words (μηδεις).
    declension!("μηδεις", 1, "~ις";
        nominative_masculine: "~ις", "~νε", "~νες";
        nominative_feminine: "~μῐᾰ", "~μιᾱ", "~μιαι";
        nominative_neuter: "~ν", "~νε", "~νᾰ";
        genitive_masculine: "~νος", "~νοιν", "~νων";
        genitive_feminine: "~μῐᾱς", "~μιαιν", "~μιων";
        genitive_neuter: "~νος", "~νοιν", "~νων";
        dative_masculine: "~νῐ", "~νοιν", "~σι";
        dative_feminine: "~μῐᾳ", "~μιαιν", "~μιαις";
        dative_neuter: "~νῐ", "~νοιν", "~σι";
        accusative_masculine: "~νᾰ", "~νε", "~νᾰς";
        accusative_feminine: "~μῐᾰν", "~μιᾱ", "~μῐᾱς";
        accusative_neuter: "~ν", "~νε", "~νᾰ";
        vocative_masculine: "~ις", "~νε", "~νες";
        vocative_feminine: "~μιᾰ", "~μιᾱ", "~μιαι";
        vocative_neuter: "~ν", "~νε", "~νᾰ";
    ),
    // Ἀλληλων: 1 words (ἀλληλων).
    declension!("ἀλληλων", 1, "ἀ~ων";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        genitive_feminine: "~ης", "ᾰ\u{313}~αιν", "ᾰ\u{313}~ων";
        genitive_neuter: "~ου", "ᾰ\u{313}~οιν", "ᾰ\u{313}~ων";
        dative_masculine: "~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        dative_feminine: "~ῃ", "ᾰ\u{313}~αιν", "ᾰ\u{313}~αις";
        dative_neuter: "~ῳ", "ᾰ\u{313}~οιν", "ᾰ\u{313}~οις";
        accusative_masculine: "~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ους";
        accusative_feminine: "~ην", "ᾰ\u{313}~ᾱ", "ᾰ\u{313}~ᾱς";
        accusative_neuter: "~ον", "ᾰ\u{313}~ω", "ᾰ\u{313}~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Ὁσπερ: 1 words (ὁσπερ).
    declension!("ὁσπερ", 1, "ὁσ~";
        nominative_masculine: "ὁσ~", "ὡ~", "οἱ~";
        nominative_feminine: "ἡ~", "ᾱ\u{314}~", "αἱ~";
        nominative_neuter: "ὁ~", "ὡ~", "ᾰ\u{314}~";
        genitive_masculine: "οὑ~", "οἱν~", "ὡν~";
        genitive_feminine: "ἡσ~", "αἱν~", "ὡν~";
        genitive_neuter: "οὑ~", "οἱν~", "ὡν~";
        dative_masculine: "ᾡ~", "οἱν~", "οἱσ~";
        dative_feminine: "ᾑ~", "αἱν~", "αἱσ~";
        dative_neuter: "ᾡ~", "οἱν~", "οἱσ~";
        accusative_masculine: "ὁν~", "ὡ~", "οὑσ~";
        accusative_feminine: "ἡν~", "ᾱ\u{314}~", "ᾱ\u{314}σ~";
        accusative_neuter: "ὁ~", "ὡ~", "ᾰ\u{314}~";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Σφεῑς: 1 words (σφεῑς).
    declension!("σφεῑς", 1, "~εῑς";
        nominative_masculine: "~ος", "~ω", "~εῑς";
        nominative_feminine: "~η", "~ᾱ", "~εῑς";
        nominative_neuter: "~ον", "~ω", "~εῑς";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ης", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~ῐσῐ|~ῐσῐν";
        dative_feminine: "~ῃ", "~αιν", "~ῐσῐ|~ῐσῐν";
        dative_neuter: "~ῳ", "~οιν", "~ῐσῐ|~ῐσῐν";
        accusative_masculine: "~ον", "~ω", "~ᾱς";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾱς";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Ἀμφω: 1 words (ἀμφω).
    declension!("ἀμφω", 1, "ἀ~";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ης", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ῃ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "ᾰ\u{313}~", "~οι";
        vocative_feminine: "~η", "ᾰ\u{313}~", "~αι";
        vocative_neuter: "~ον", "ᾰ\u{313}~", "~ᾰ";
    ),
    // Ἐνῐοι: 1 words (ἐνῐοι).
    declension!("ἐνῐοι", 1, "~οι";
        nominative_masculine: "~ος", "~ω", "~οι";
        nominative_feminine: "~η", "~ᾱ", "~αι";
        nominative_neuter: "~ον", "~ω", "~ᾰ";
        genitive_masculine: "~ου", "~οιν", "~ων";
        genitive_feminine: "~ης", "~αιν", "~ων";
        genitive_neuter: "~ου", "~οιν", "~ων";
        dative_masculine: "~ῳ", "~οιν", "~οις";
        dative_feminine: "~ῃ", "~αιν", "~αις";
        dative_neuter: "~ῳ", "~οιν", "~οις";
        accusative_masculine: "~ον", "~ω", "~ους";
        accusative_feminine: "~ην", "~ᾱ", "~ᾱς";
        accusative_neuter: "~ον", "~ω", "~ᾰ";
        vocative_masculine: "~ε", "~ω", "~οι";
        vocative_feminine: "~η", "~ᾱ", "~αι";
        vocative_neuter: "~ον", "~ω", "~ᾰ";
    ),
    // Ὁσᾰπλᾰσῐων: 1 words (ὁσᾰπλᾰσῐων).
    declension!("ὁσᾰπλᾰσῐων", 1, "~ων";
        nominative_masculine: "~ων", "~ονε", "~ονες";
        nominative_feminine: "~ων", "~ονε", "~ονες";
        nominative_neuter: "~ον", "~ονε", "~ονᾰ";
        genitive_masculine: "~ονος", "~ονοιν", "~ονων";
        genitive_feminine: "~ονος", "~ονοιν", "~ονων";
        genitive_neuter: "~ονος", "~ονοιν", "~ονων";
        dative_masculine: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        dative_feminine: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        dative_neuter: "~ονῐ", "~ονοιν", "~οσῐ|~οσῐν";
        accusative_masculine: "~ονᾰ", "~ονε", "~ονᾰς";
        accusative_feminine: "~ονᾰ", "~ονε", "~ονᾰς";
        accusative_neuter: "~ον", "~ονε", "~ονᾰ";
        vocative_masculine: "~ον", "~ονε", "~ονες";
        vocative_feminine: "~ον", "~ονε", "~ονες";
        vocative_neuter: "~ον", "~ονε", "~ονᾰ";
    ),
];
