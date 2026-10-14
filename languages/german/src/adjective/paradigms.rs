//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the declension's it is like, or, in a declension like
/// no other, derived):
/// `strong_nom` (m, f, n, pl)
/// `strong_gen` (m, f, n, pl)
/// `strong_dat` (m, f, n, pl)
/// `strong_acc` (m, f, n, pl)
/// `weak_nom` (m, f, n, pl)
/// `weak_gen` (m, f, n, pl)
/// `weak_dat` (m, f, n, pl)
/// `weak_acc` (m, f, n, pl)
/// `mixed_nom` (m, f, n, pl)
/// `mixed_gen` (m, f, n, pl)
/// `mixed_dat` (m, f, n, pl)
/// `mixed_acc` (m, f, n, pl)
/// `comparative_strong_nom` (m, f, n, pl)
/// `comparative_strong_gen` (m, f, n, pl)
/// `comparative_strong_dat` (m, f, n, pl)
/// `comparative_strong_acc` (m, f, n, pl)
/// `comparative_weak_nom` (m, f, n, pl)
/// `comparative_weak_gen` (m, f, n, pl)
/// `comparative_weak_dat` (m, f, n, pl)
/// `comparative_weak_acc` (m, f, n, pl)
/// `comparative_mixed_nom` (m, f, n, pl)
/// `comparative_mixed_gen` (m, f, n, pl)
/// `comparative_mixed_dat` (m, f, n, pl)
/// `comparative_mixed_acc` (m, f, n, pl)
/// `superlative_strong_nom` (m, f, n, pl)
/// `superlative_strong_gen` (m, f, n, pl)
/// `superlative_strong_dat` (m, f, n, pl)
/// `superlative_strong_acc` (m, f, n, pl)
/// `superlative_weak_nom` (m, f, n, pl)
/// `superlative_weak_gen` (m, f, n, pl)
/// `superlative_weak_dat` (m, f, n, pl)
/// `superlative_weak_acc` (m, f, n, pl)
/// `superlative_mixed_nom` (m, f, n, pl)
/// `superlative_mixed_gen` (m, f, n, pl)
/// `superlative_mixed_dat` (m, f, n, pl)
/// `superlative_mixed_acc` (m, f, n, pl)
/// `predicative` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem adjective: $s0:tt;)?
     $(stem comparative: $s1:tt;)?
     $(stem superlative: $s2:tt;)?
     $(strong_nom: $c0:tt, $c1:tt, $c2:tt, $c3:tt;)?
     $(strong_gen: $c4:tt, $c5:tt, $c6:tt, $c7:tt;)?
     $(strong_dat: $c8:tt, $c9:tt, $c10:tt, $c11:tt;)?
     $(strong_acc: $c12:tt, $c13:tt, $c14:tt, $c15:tt;)?
     $(weak_nom: $c16:tt, $c17:tt, $c18:tt, $c19:tt;)?
     $(weak_gen: $c20:tt, $c21:tt, $c22:tt, $c23:tt;)?
     $(weak_dat: $c24:tt, $c25:tt, $c26:tt, $c27:tt;)?
     $(weak_acc: $c28:tt, $c29:tt, $c30:tt, $c31:tt;)?
     $(mixed_nom: $c32:tt, $c33:tt, $c34:tt, $c35:tt;)?
     $(mixed_gen: $c36:tt, $c37:tt, $c38:tt, $c39:tt;)?
     $(mixed_dat: $c40:tt, $c41:tt, $c42:tt, $c43:tt;)?
     $(mixed_acc: $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(comparative_strong_nom: $c48:tt, $c49:tt, $c50:tt, $c51:tt;)?
     $(comparative_strong_gen: $c52:tt, $c53:tt, $c54:tt, $c55:tt;)?
     $(comparative_strong_dat: $c56:tt, $c57:tt, $c58:tt, $c59:tt;)?
     $(comparative_strong_acc: $c60:tt, $c61:tt, $c62:tt, $c63:tt;)?
     $(comparative_weak_nom: $c64:tt, $c65:tt, $c66:tt, $c67:tt;)?
     $(comparative_weak_gen: $c68:tt, $c69:tt, $c70:tt, $c71:tt;)?
     $(comparative_weak_dat: $c72:tt, $c73:tt, $c74:tt, $c75:tt;)?
     $(comparative_weak_acc: $c76:tt, $c77:tt, $c78:tt, $c79:tt;)?
     $(comparative_mixed_nom: $c80:tt, $c81:tt, $c82:tt, $c83:tt;)?
     $(comparative_mixed_gen: $c84:tt, $c85:tt, $c86:tt, $c87:tt;)?
     $(comparative_mixed_dat: $c88:tt, $c89:tt, $c90:tt, $c91:tt;)?
     $(comparative_mixed_acc: $c92:tt, $c93:tt, $c94:tt, $c95:tt;)?
     $(superlative_strong_nom: $c96:tt, $c97:tt, $c98:tt, $c99:tt;)?
     $(superlative_strong_gen: $c100:tt, $c101:tt, $c102:tt, $c103:tt;)?
     $(superlative_strong_dat: $c104:tt, $c105:tt, $c106:tt, $c107:tt;)?
     $(superlative_strong_acc: $c108:tt, $c109:tt, $c110:tt, $c111:tt;)?
     $(superlative_weak_nom: $c112:tt, $c113:tt, $c114:tt, $c115:tt;)?
     $(superlative_weak_gen: $c116:tt, $c117:tt, $c118:tt, $c119:tt;)?
     $(superlative_weak_dat: $c120:tt, $c121:tt, $c122:tt, $c123:tt;)?
     $(superlative_weak_acc: $c124:tt, $c125:tt, $c126:tt, $c127:tt;)?
     $(superlative_mixed_nom: $c128:tt, $c129:tt, $c130:tt, $c131:tt;)?
     $(superlative_mixed_gen: $c132:tt, $c133:tt, $c134:tt, $c135:tt;)?
     $(superlative_mixed_dat: $c136:tt, $c137:tt, $c138:tt, $c139:tt;)?
     $(superlative_mixed_acc: $c140:tt, $c141:tt, $c142:tt, $c143:tt;)?
     $(predicative: $c144:tt, $c145:tt, $c146:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
                $(flexion::cell!($c48),)? "\t",
                $(flexion::cell!($c49),)? "\t",
                $(flexion::cell!($c50),)? "\t",
                $(flexion::cell!($c51),)? "\t",
                $(flexion::cell!($c52),)? "\t",
                $(flexion::cell!($c53),)? "\t",
                $(flexion::cell!($c54),)? "\t",
                $(flexion::cell!($c55),)? "\t",
                $(flexion::cell!($c56),)? "\t",
                $(flexion::cell!($c57),)? "\t",
                $(flexion::cell!($c58),)? "\t",
                $(flexion::cell!($c59),)? "\t",
                $(flexion::cell!($c60),)? "\t",
                $(flexion::cell!($c61),)? "\t",
                $(flexion::cell!($c62),)? "\t",
                $(flexion::cell!($c63),)? "\t",
                $(flexion::cell!($c64),)? "\t",
                $(flexion::cell!($c65),)? "\t",
                $(flexion::cell!($c66),)? "\t",
                $(flexion::cell!($c67),)? "\t",
                $(flexion::cell!($c68),)? "\t",
                $(flexion::cell!($c69),)? "\t",
                $(flexion::cell!($c70),)? "\t",
                $(flexion::cell!($c71),)? "\t",
                $(flexion::cell!($c72),)? "\t",
                $(flexion::cell!($c73),)? "\t",
                $(flexion::cell!($c74),)? "\t",
                $(flexion::cell!($c75),)? "\t",
                $(flexion::cell!($c76),)? "\t",
                $(flexion::cell!($c77),)? "\t",
                $(flexion::cell!($c78),)? "\t",
                $(flexion::cell!($c79),)? "\t",
                $(flexion::cell!($c80),)? "\t",
                $(flexion::cell!($c81),)? "\t",
                $(flexion::cell!($c82),)? "\t",
                $(flexion::cell!($c83),)? "\t",
                $(flexion::cell!($c84),)? "\t",
                $(flexion::cell!($c85),)? "\t",
                $(flexion::cell!($c86),)? "\t",
                $(flexion::cell!($c87),)? "\t",
                $(flexion::cell!($c88),)? "\t",
                $(flexion::cell!($c89),)? "\t",
                $(flexion::cell!($c90),)? "\t",
                $(flexion::cell!($c91),)? "\t",
                $(flexion::cell!($c92),)? "\t",
                $(flexion::cell!($c93),)? "\t",
                $(flexion::cell!($c94),)? "\t",
                $(flexion::cell!($c95),)? "\t",
                $(flexion::cell!($c96),)? "\t",
                $(flexion::cell!($c97),)? "\t",
                $(flexion::cell!($c98),)? "\t",
                $(flexion::cell!($c99),)? "\t",
                $(flexion::cell!($c100),)? "\t",
                $(flexion::cell!($c101),)? "\t",
                $(flexion::cell!($c102),)? "\t",
                $(flexion::cell!($c103),)? "\t",
                $(flexion::cell!($c104),)? "\t",
                $(flexion::cell!($c105),)? "\t",
                $(flexion::cell!($c106),)? "\t",
                $(flexion::cell!($c107),)? "\t",
                $(flexion::cell!($c108),)? "\t",
                $(flexion::cell!($c109),)? "\t",
                $(flexion::cell!($c110),)? "\t",
                $(flexion::cell!($c111),)? "\t",
                $(flexion::cell!($c112),)? "\t",
                $(flexion::cell!($c113),)? "\t",
                $(flexion::cell!($c114),)? "\t",
                $(flexion::cell!($c115),)? "\t",
                $(flexion::cell!($c116),)? "\t",
                $(flexion::cell!($c117),)? "\t",
                $(flexion::cell!($c118),)? "\t",
                $(flexion::cell!($c119),)? "\t",
                $(flexion::cell!($c120),)? "\t",
                $(flexion::cell!($c121),)? "\t",
                $(flexion::cell!($c122),)? "\t",
                $(flexion::cell!($c123),)? "\t",
                $(flexion::cell!($c124),)? "\t",
                $(flexion::cell!($c125),)? "\t",
                $(flexion::cell!($c126),)? "\t",
                $(flexion::cell!($c127),)? "\t",
                $(flexion::cell!($c128),)? "\t",
                $(flexion::cell!($c129),)? "\t",
                $(flexion::cell!($c130),)? "\t",
                $(flexion::cell!($c131),)? "\t",
                $(flexion::cell!($c132),)? "\t",
                $(flexion::cell!($c133),)? "\t",
                $(flexion::cell!($c134),)? "\t",
                $(flexion::cell!($c135),)? "\t",
                $(flexion::cell!($c136),)? "\t",
                $(flexion::cell!($c137),)? "\t",
                $(flexion::cell!($c138),)? "\t",
                $(flexion::cell!($c139),)? "\t",
                $(flexion::cell!($c140),)? "\t",
                $(flexion::cell!($c141),)? "\t",
                $(flexion::cell!($c142),)? "\t",
                $(flexion::cell!($c143),)? "\t",
                $(flexion::cell!($c144),)? "\t",
                $(flexion::cell!($c145),)? "\t",
                $(flexion::cell!($c146),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
            )),
        }
    };
}

/// The declension a declension is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Declension::$parent as u16)
    };
}

/// The cells a declension reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `veraltet`: 12,225 words.
    Veraltet,
    /// Like `glatt`: 1,862 words.
    Glatt,
    /// Like `überfinanziert`: 439 words.
    Überfinanziert,
    /// Like `edel`: 102 words.
    Edel,
    /// Like `scharf`: 50 words.
    Scharf,
    /// Like `schwarz`: 33 words.
    Schwarz,
    /// Like `träge`: 24 words.
    Träge,
    /// Like `zeitnah`: 11 words.
    Zeitnah,
    /// Like `groß`: 11 words.
    Groß,
    /// Like `kurz`: 9 words.
    Kurz,
    /// Like `dumm`: 9 words.
    Dumm,
    /// Like `unedel`: 9 words.
    Unedel,
    /// Like `oberer`: 9 words.
    Oberer,
    /// Like `rot`: 7 words.
    Rot,
    /// Like `grob`: 7 words.
    Grob,
    /// Like `lila`: 6 words.
    Lila,
    /// Like `hoch`: 6 words.
    Hoch,
    /// Like `naß`: 6 words.
    Naß,
    /// Like `arg`: 3 words.
    Arg,
    /// Like `gutbesucht`: 3 words.
    Gutbesucht,
    /// Like `klageweise`: 1 words.
    Klageweise,
    /// Like `kaufweise`: 1 words.
    Kaufweise,
    /// Like `pachtweise`: 1 words.
    Pachtweise,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 23] = [
        Declension::Veraltet,
        Declension::Glatt,
        Declension::Überfinanziert,
        Declension::Edel,
        Declension::Scharf,
        Declension::Schwarz,
        Declension::Träge,
        Declension::Zeitnah,
        Declension::Groß,
        Declension::Kurz,
        Declension::Dumm,
        Declension::Unedel,
        Declension::Oberer,
        Declension::Rot,
        Declension::Grob,
        Declension::Lila,
        Declension::Hoch,
        Declension::Naß,
        Declension::Arg,
        Declension::Gutbesucht,
        Declension::Klageweise,
        Declension::Kaufweise,
        Declension::Pachtweise,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 23] = [
    // Veraltet: 12,225 words (frei, synonym, abaxial, abdominal, aberrant, abnormal).
    declension!("veraltet", 12225, "~";
    stem superlative: "~ste- like veraltet";
    strong_nom: _, "~e", _, _;
    strong_gen: _, "~er", "~en", "~er";
    strong_dat: _, "~er", "~em", "~en";
    strong_acc: "~en", _, "~es", _;
    weak_nom: _, _, _, "~en";
    weak_gen: "~en", "~en", "~en", "~en";
    weak_dat: "~en", "~en", "~en", "~en";
    weak_acc: "~en", _, _, "~en";
    mixed_nom: "~er", _, "~es", "~en";
    mixed_gen: "~en", "~en", "~en", "~en";
    mixed_dat: "~en", "~en", "~en", "~en";
    mixed_acc: "~en", _, "~es", "~en";
    comparative_strong_nom: "~erer", _, _, "~ere";
    comparative_strong_gen: _, _, "~eren", _;
    comparative_strong_dat: _, _, "~erem", "~eren";
    comparative_strong_acc: "~eren", "~ere", "~eres", "~ere";
    comparative_weak_nom: "~ere", "~ere", "~ere", "~eren";
    comparative_weak_gen: "~eren", "~eren", "~eren", "~eren";
    comparative_weak_dat: "~eren", "~eren", "~eren", "~eren";
    comparative_weak_acc: "~eren", "~ere", "~ere", "~eren";
    comparative_mixed_nom: _, "~ere", "~eres", "~eren";
    comparative_mixed_gen: "~eren", "~eren", "~eren", "~eren";
    comparative_mixed_dat: "~eren", "~eren", "~eren", "~eren";
    comparative_mixed_acc: "~eren", "~ere", "~eres", "~eren";
    predicative: "~", "~er", _;
    ),
    // Glatt: 1,862 words (absent, abstinent, absurd, blond, hold, fluid). Like Veraltet, except in 0 cells: .
    declension!("glatt", 1862, "~", like Veraltet;
    stem superlative: "~este- like veraltet";
    ),
    // Überfinanziert: 439 words (gelaufen, gegessen, getrunken, gesprochen, gekommen, geboren). Like Veraltet, except in 51 cells: superlative_strong_nom m, superlative_strong_nom f, superlative_strong_nom n, superlative_strong_nom pl, superlative_strong_gen m, superlative_strong_gen f, ….
    declension!("überfinanziert", 439, "~", like Veraltet;
    superlative_strong_nom: "~ster", "~ste", "~stes", derived;
    superlative_strong_gen: "~sten", derived, derived, derived;
    superlative_strong_dat: "~stem", derived, derived, derived;
    superlative_strong_acc: derived, derived, derived, derived;
    superlative_weak_nom: derived, derived, derived, derived;
    superlative_weak_gen: derived, derived, derived, derived;
    superlative_weak_dat: derived, derived, derived, derived;
    superlative_weak_acc: derived, derived, derived, derived;
    superlative_mixed_nom: derived, derived, derived, derived;
    superlative_mixed_gen: derived, derived, derived, derived;
    superlative_mixed_dat: derived, derived, derived, derived;
    superlative_mixed_acc: derived, derived, derived, derived;
    predicative: "ist ~|es ist ~|sind ~", "ist ~er", "ist am ~sten";
    ),
    // Edel: 102 words (übel, nobel, dunkel, simpel, sauer, teuer).
    declension!("edel", 102, "~e~";
    stem superlative: "~e~ste- like veraltet";
    strong_nom: _, "~~e", _, _;
    strong_gen: _, "~~er", "~~en", "~~er";
    strong_dat: _, "~~er", "~~em", "~~en";
    strong_acc: "~~en", _, "~~es", _;
    weak_nom: _, _, _, "~~en";
    weak_gen: "~~en", "~~en", "~~en", "~~en";
    weak_dat: "~~en", "~~en", "~~en", "~~en";
    weak_acc: "~~en", _, _, "~~en";
    mixed_nom: "~~er", _, "~~es", "~~en";
    mixed_gen: "~~en", "~~en", "~~en", "~~en";
    mixed_dat: "~~en", "~~en", "~~en", "~~en";
    mixed_acc: "~~en", _, "~~es", "~~en";
    comparative_strong_nom: "~~erer", _, _, "~~ere";
    comparative_strong_gen: _, _, "~~eren", _;
    comparative_strong_dat: _, _, "~~erem", "~~eren";
    comparative_strong_acc: "~~eren", "~~ere", "~~eres", "~~ere";
    comparative_weak_nom: "~~ere", "~~ere", "~~ere", "~~eren";
    comparative_weak_gen: "~~eren", "~~eren", "~~eren", "~~eren";
    comparative_weak_dat: "~~eren", "~~eren", "~~eren", "~~eren";
    comparative_weak_acc: "~~eren", "~~ere", "~~ere", "~~eren";
    comparative_mixed_nom: _, "~~ere", "~~eres", "~~eren";
    comparative_mixed_gen: "~~eren", "~~eren", "~~eren", "~~eren";
    comparative_mixed_dat: "~~eren", "~~eren", "~~eren", "~~eren";
    comparative_mixed_acc: "~~eren", "~~ere", "~~eres", "~~eren";
    predicative: "~e~", "~~er", _;
    ),
    // Scharf: 50 words (warm, schwarz, stark, lang, scharf, schwach).
    declension!("scharf", 50, "~a~";
    stem superlative: "~ä~ste- like veraltet";
    strong_nom: _, "~a~e", _, _;
    strong_gen: _, "~a~er", "~a~en", "~a~er";
    strong_dat: _, "~a~er", "~a~em", "~a~en";
    strong_acc: "~a~en", _, "~a~es", _;
    weak_nom: _, _, _, "~a~en";
    weak_gen: "~a~en", "~a~en", "~a~en", "~a~en";
    weak_dat: "~a~en", "~a~en", "~a~en", "~a~en";
    weak_acc: "~a~en", _, _, "~a~en";
    mixed_nom: "~a~er", _, "~a~es", "~a~en";
    mixed_gen: "~a~en", "~a~en", "~a~en", "~a~en";
    mixed_dat: "~a~en", "~a~en", "~a~en", "~a~en";
    mixed_acc: "~a~en", _, "~a~es", "~a~en";
    comparative_strong_nom: "~ä~erer", _, _, "~ä~ere";
    comparative_strong_gen: _, _, "~ä~eren", _;
    comparative_strong_dat: _, _, "~ä~erem", "~ä~eren";
    comparative_strong_acc: "~ä~eren", "~ä~ere", "~ä~eres", "~ä~ere";
    comparative_weak_nom: "~ä~ere", "~ä~ere", "~ä~ere", "~ä~eren";
    comparative_weak_gen: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
    comparative_weak_dat: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
    comparative_weak_acc: "~ä~eren", "~ä~ere", "~ä~ere", "~ä~eren";
    comparative_mixed_nom: _, "~ä~ere", "~ä~eres", "~ä~eren";
    comparative_mixed_gen: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
    comparative_mixed_dat: "~ä~eren", "~ä~eren", "~ä~eren", "~ä~eren";
    comparative_mixed_acc: "~ä~eren", "~ä~ere", "~ä~eres", "~ä~eren";
    predicative: "~a~", "~ä~er", _;
    ),
    // Schwarz: 33 words (hart, kalt, galt, uralt, pechschwarz, steinalt). Like Scharf, except in 0 cells: .
    declension!("schwarz", 33, "~a~", like Scharf;
    stem superlative: "~ä~este- like veraltet";
    ),
    // Träge: 24 words (beige, broke, malle, fake, knorke, äußere).
    declension!("träge", 24, "~~";
    stem superlative: "~st~- like veraltet";
    strong_nom: _, "~~", _, _;
    strong_gen: _, "~~r", "~~n", "~~r";
    strong_dat: _, "~~r", "~~m", "~~n";
    strong_acc: "~~n", _, "~~s", _;
    weak_nom: _, _, _, "~~n";
    weak_gen: "~~n", "~~n", "~~n", "~~n";
    weak_dat: "~~n", "~~n", "~~n", "~~n";
    weak_acc: "~~n", _, _, "~~n";
    mixed_nom: "~~r", _, "~~s", "~~n";
    mixed_gen: "~~n", "~~n", "~~n", "~~n";
    mixed_dat: "~~n", "~~n", "~~n", "~~n";
    mixed_acc: "~~n", _, "~~s", "~~n";
    comparative_strong_nom: "~~rer", _, _, "~~re";
    comparative_strong_gen: _, _, "~~ren", _;
    comparative_strong_dat: _, _, "~~rem", "~~ren";
    comparative_strong_acc: "~~ren", "~~re", "~~res", "~~re";
    comparative_weak_nom: "~~re", "~~re", "~~re", "~~ren";
    comparative_weak_gen: "~~ren", "~~ren", "~~ren", "~~ren";
    comparative_weak_dat: "~~ren", "~~ren", "~~ren", "~~ren";
    comparative_weak_acc: "~~ren", "~~re", "~~re", "~~ren";
    comparative_mixed_nom: _, "~~re", "~~res", "~~ren";
    comparative_mixed_gen: "~~ren", "~~ren", "~~ren", "~~ren";
    comparative_mixed_dat: "~~ren", "~~ren", "~~ren", "~~ren";
    comparative_mixed_acc: "~~ren", "~~re", "~~res", "~~ren";
    predicative: "~~", "~~r", _;
    ),
    // Zeitnah: 11 words (nah, bodennah, oberflächennah, verbrauchernah, regierungsnah, berlinnah). Like Scharf, except in 0 cells: .
    declension!("zeitnah", 11, "~a~", like Scharf;
    stem superlative: "~äc~ste- like veraltet";
    ),
    // Groß: 11 words (groß, gross, riesengroß, mittelgroß, flächengroß, übergroß).
    declension!("groß", 11, "~o~";
    stem superlative: "~ö~te- like veraltet";
    strong_nom: _, "~o~e", _, _;
    strong_gen: _, "~o~er", "~o~en", "~o~er";
    strong_dat: _, "~o~er", "~o~em", "~o~en";
    strong_acc: "~o~en", _, "~o~es", _;
    weak_nom: _, _, _, "~o~en";
    weak_gen: "~o~en", "~o~en", "~o~en", "~o~en";
    weak_dat: "~o~en", "~o~en", "~o~en", "~o~en";
    weak_acc: "~o~en", _, _, "~o~en";
    mixed_nom: "~o~er", _, "~o~es", "~o~en";
    mixed_gen: "~o~en", "~o~en", "~o~en", "~o~en";
    mixed_dat: "~o~en", "~o~en", "~o~en", "~o~en";
    mixed_acc: "~o~en", _, "~o~es", "~o~en";
    comparative_strong_nom: "~ö~erer", _, _, "~ö~ere";
    comparative_strong_gen: _, _, "~ö~eren", _;
    comparative_strong_dat: _, _, "~ö~erem", "~ö~eren";
    comparative_strong_acc: "~ö~eren", "~ö~ere", "~ö~eres", "~ö~ere";
    comparative_weak_nom: "~ö~ere", "~ö~ere", "~ö~ere", "~ö~eren";
    comparative_weak_gen: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
    comparative_weak_dat: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
    comparative_weak_acc: "~ö~eren", "~ö~ere", "~ö~ere", "~ö~eren";
    comparative_mixed_nom: _, "~ö~ere", "~ö~eres", "~ö~eren";
    comparative_mixed_gen: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
    comparative_mixed_dat: "~ö~eren", "~ö~eren", "~ö~eren", "~ö~eren";
    comparative_mixed_acc: "~ö~eren", "~ö~ere", "~ö~eres", "~ö~eren";
    predicative: "~o~", "~ö~er", _;
    ),
    // Kurz: 9 words (kurz, gesund, schnurz, ungesund, superkurz, ultrakurz).
    declension!("kurz", 9, "~u~";
    stem superlative: "~ü~este- like veraltet";
    strong_nom: _, "~u~e", _, _;
    strong_gen: _, "~u~er", "~u~en", "~u~er";
    strong_dat: _, "~u~er", "~u~em", "~u~en";
    strong_acc: "~u~en", _, "~u~es", _;
    weak_nom: _, _, _, "~u~en";
    weak_gen: "~u~en", "~u~en", "~u~en", "~u~en";
    weak_dat: "~u~en", "~u~en", "~u~en", "~u~en";
    weak_acc: "~u~en", _, _, "~u~en";
    mixed_nom: "~u~er", _, "~u~es", "~u~en";
    mixed_gen: "~u~en", "~u~en", "~u~en", "~u~en";
    mixed_dat: "~u~en", "~u~en", "~u~en", "~u~en";
    mixed_acc: "~u~en", _, "~u~es", "~u~en";
    comparative_strong_nom: "~ü~erer", _, _, "~ü~ere";
    comparative_strong_gen: _, _, "~ü~eren", _;
    comparative_strong_dat: _, _, "~ü~erem", "~ü~eren";
    comparative_strong_acc: "~ü~eren", "~ü~ere", "~ü~eres", "~ü~ere";
    comparative_weak_nom: "~ü~ere", "~ü~ere", "~ü~ere", "~ü~eren";
    comparative_weak_gen: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
    comparative_weak_dat: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
    comparative_weak_acc: "~ü~eren", "~ü~ere", "~ü~ere", "~ü~eren";
    comparative_mixed_nom: _, "~ü~ere", "~ü~eres", "~ü~eren";
    comparative_mixed_gen: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
    comparative_mixed_dat: "~ü~eren", "~ü~eren", "~ü~eren", "~ü~eren";
    comparative_mixed_acc: "~ü~eren", "~ü~ere", "~ü~eres", "~ü~eren";
    predicative: "~u~", "~ü~er", _;
    ),
    // Dumm: 9 words (dumm, jung, klug, genug, unklug, altklug). Like Kurz, except in 0 cells: .
    declension!("dumm", 9, "~u~", like Kurz;
    stem superlative: "~ü~ste- like veraltet";
    ),
    // Unedel: 9 words (integer, schofel, illuster, makaber, speiübel, unedel). Like Edel, except in 34 cells: strong_nom f, strong_gen f, strong_gen n, strong_gen pl, strong_dat f, strong_dat n, ….
    declension!("unedel", 9, "~e~", like Edel;
    strong_nom: _, "~e~e", _, _;
    strong_gen: _, "~e~er", "~e~en", "~e~er";
    strong_dat: _, "~e~er", "~e~em", "~e~en";
    strong_acc: "~e~en", _, "~e~es", _;
    weak_nom: _, _, _, "~e~en";
    weak_gen: "~e~en", "~e~en", "~e~en", "~e~en";
    weak_dat: "~e~en", "~e~en", "~e~en", "~e~en";
    weak_acc: "~e~en", _, _, "~e~en";
    mixed_nom: "~e~er", _, "~e~es", "~e~en";
    mixed_gen: "~e~en", "~e~en", "~e~en", "~e~en";
    mixed_dat: "~e~en", "~e~en", "~e~en", "~e~en";
    mixed_acc: "~e~en", _, "~e~es", "~e~en";
    ),
    // Oberer: 9 words (nächster, vorkantscher, mittlerer, unterer, oberer, überdünischer).
    declension!("oberer", 9, "~~r";
    strong_nom: _, "~~", _, _;
    strong_gen: _, "~~r", "~~n", "~~r";
    strong_dat: _, "~~r", "~~m", "~~n";
    strong_acc: "~~n", _, "~~s", _;
    weak_nom: _, _, _, "~~n";
    weak_gen: "~~n", "~~n", "~~n", "~~n";
    weak_dat: "~~n", "~~n", "~~n", "~~n";
    weak_acc: "~~n", _, _, "~~n";
    mixed_nom: "~~r", _, "~~s", "~~n";
    mixed_gen: "~~n", "~~n", "~~n", "~~n";
    mixed_dat: "~~n", "~~n", "~~n", "~~n";
    mixed_acc: "~~n", _, "~~s", "~~n";
    superlative_strong_nom: "~st~r", "~st~", "~st~s", _;
    superlative_strong_gen: "~st~n", _, _, _;
    superlative_strong_dat: "~st~m", _, _, _;
    predicative: "~~r", _, _;
    ),
    // Rot: 7 words (rot, roth, orthognath, prognath, dunckelroth, dunkelroth). Like Groß, except in 0 cells: .
    declension!("rot", 7, "~o~", like Groß;
    stem superlative: "~ö~este- like veraltet";
    ),
    // Grob: 7 words (grob, fromm, aerob, anaerob, lammfromm, storr). Like Groß, except in 0 cells: .
    declension!("grob", 7, "~o~", like Groß;
    stem superlative: "~ö~ste- like veraltet";
    ),
    // Lila: 6 words (rosa, lila, altrosa, hellrosa, schweinchenrosa, blasslila).
    declension!("lila", 6, "~";
    stem superlative: "~nste- like veraltet";
    strong_nom: _, "~ne", _, _;
    strong_gen: _, "~ner", "~nen", "~ner";
    strong_dat: _, "~ner", "~nem", "~nen";
    strong_acc: "~nen", _, "~nes", _;
    weak_nom: _, _, _, "~nen";
    weak_gen: "~nen", "~nen", "~nen", "~nen";
    weak_dat: "~nen", "~nen", "~nen", "~nen";
    weak_acc: "~nen", _, _, "~nen";
    mixed_nom: "~ner", _, "~nes", "~nen";
    mixed_gen: "~nen", "~nen", "~nen", "~nen";
    mixed_dat: "~nen", "~nen", "~nen", "~nen";
    mixed_acc: "~nen", _, "~nes", "~nen";
    comparative_strong_nom: "~nerer", _, _, "~nere";
    comparative_strong_gen: _, _, "~neren", _;
    comparative_strong_dat: _, _, "~nerem", "~neren";
    comparative_strong_acc: "~neren", "~nere", "~neres", "~nere";
    comparative_weak_nom: "~nere", "~nere", "~nere", "~neren";
    comparative_weak_gen: "~neren", "~neren", "~neren", "~neren";
    comparative_weak_dat: "~neren", "~neren", "~neren", "~neren";
    comparative_weak_acc: "~neren", "~nere", "~nere", "~neren";
    comparative_mixed_nom: _, "~nere", "~neres", "~neren";
    comparative_mixed_gen: "~neren", "~neren", "~neren", "~neren";
    comparative_mixed_dat: "~neren", "~neren", "~neren", "~neren";
    comparative_mixed_acc: "~neren", "~nere", "~neres", "~neren";
    predicative: "~", "~ner", _;
    ),
    // Hoch: 6 words (hoch, himmelhoch, ranghoch, wolkenhoch, hüfthoch, halbhoch). Like Groß, except in 1 cell: predicative pos.
    declension!("hoch", 6, "~oc~", like Groß;
    stem superlative: "~öc~ste- like veraltet";
    predicative: "~oc~", _, _;
    ),
    // Naß: 6 words (naß, gewiß, ungewiß, kraß, blaß, totenblaß).
    declension!("naß", 6, "~ß";
    stem superlative: "~sseste- like veraltet";
    strong_nom: _, "~sse", _, _;
    strong_gen: _, "~sser", "~ssen", "~sser";
    strong_dat: _, "~sser", "~ssem", "~ssen";
    strong_acc: "~ssen", _, "~sses", _;
    weak_nom: _, _, _, "~ssen";
    weak_gen: "~ssen", "~ssen", "~ssen", "~ssen";
    weak_dat: "~ssen", "~ssen", "~ssen", "~ssen";
    weak_acc: "~ssen", _, _, "~ssen";
    mixed_nom: "~sser", _, "~sses", "~ssen";
    mixed_gen: "~ssen", "~ssen", "~ssen", "~ssen";
    mixed_dat: "~ssen", "~ssen", "~ssen", "~ssen";
    mixed_acc: "~ssen", _, "~sses", "~ssen";
    comparative_strong_nom: "~sserer", _, _, "~ssere";
    comparative_strong_gen: _, _, "~sseren", _;
    comparative_strong_dat: _, _, "~sserem", "~sseren";
    comparative_strong_acc: "~sseren", "~ssere", "~sseres", "~ssere";
    comparative_weak_nom: "~ssere", "~ssere", "~ssere", "~sseren";
    comparative_weak_gen: "~sseren", "~sseren", "~sseren", "~sseren";
    comparative_weak_dat: "~sseren", "~sseren", "~sseren", "~sseren";
    comparative_weak_acc: "~sseren", "~ssere", "~ssere", "~sseren";
    comparative_mixed_nom: _, "~ssere", "~sseres", "~sseren";
    comparative_mixed_gen: "~sseren", "~sseren", "~sseren", "~sseren";
    comparative_mixed_dat: "~sseren", "~sseren", "~sseren", "~sseren";
    comparative_mixed_acc: "~sseren", "~ssere", "~sseres", "~sseren";
    predicative: "~ß", "~sser", _;
    ),
    // Arg: 3 words (arm, alt, arg).
    declension!("arg", 3, "a~";
    stem superlative: "~ste- like arg";
    strong_nom: _, "a~e", _, _;
    strong_gen: _, "a~er", "a~en", "a~er";
    strong_dat: _, "a~er", "a~em", "a~en";
    strong_acc: "a~en", _, "a~es", _;
    weak_nom: _, _, _, "a~en";
    weak_gen: "a~en", "a~en", "a~en", "a~en";
    weak_dat: "a~en", "a~en", "a~en", "a~en";
    weak_acc: "a~en", _, _, "a~en";
    mixed_nom: "a~er", _, "a~es", "a~en";
    mixed_gen: "a~en", "a~en", "a~en", "a~en";
    mixed_dat: "a~en", "a~en", "a~en", "a~en";
    mixed_acc: "a~en", _, "a~es", "a~en";
    comparative_strong_nom: "ä~erer", _, _, "ä~ere";
    comparative_strong_gen: _, _, "ä~eren", _;
    comparative_strong_dat: _, _, "ä~erem", "ä~eren";
    comparative_strong_acc: "ä~eren", "ä~ere", "ä~eres", "ä~ere";
    comparative_weak_nom: "ä~ere", "ä~ere", "ä~ere", "ä~eren";
    comparative_weak_gen: "ä~eren", "ä~eren", "ä~eren", "ä~eren";
    comparative_weak_dat: "ä~eren", "ä~eren", "ä~eren", "ä~eren";
    comparative_weak_acc: "ä~eren", "ä~ere", "ä~ere", "ä~eren";
    comparative_mixed_nom: _, "ä~ere", "ä~eres", "ä~eren";
    comparative_mixed_gen: "ä~eren", "ä~eren", "ä~eren", "ä~eren";
    comparative_mixed_dat: "ä~eren", "ä~eren", "ä~eren", "ä~eren";
    comparative_mixed_acc: "ä~eren", "ä~ere", "ä~eres", "ä~eren";
    predicative: "a~", "ä~er", _;
    ),
    // Gutbesucht: 3 words (gutbesucht, guterhalten, gutgelaunt).
    declension!("gutbesucht", 3, "gut~";
    strong_nom: _, "gut~e", _, _;
    strong_gen: _, "gut~er", "gut~en", "gut~er";
    strong_dat: _, "gut~er", "gut~em", "gut~en";
    strong_acc: "gut~en", _, "gut~es", _;
    weak_nom: _, _, _, "gut~en";
    weak_gen: "gut~en", "gut~en", "gut~en", "gut~en";
    weak_dat: "gut~en", "gut~en", "gut~en", "gut~en";
    weak_acc: "gut~en", _, _, "gut~en";
    mixed_nom: "gut~er", _, "gut~es", "gut~en";
    mixed_gen: "gut~en", "gut~en", "gut~en", "gut~en";
    mixed_dat: "gut~en", "gut~en", "gut~en", "gut~en";
    mixed_acc: "gut~en", _, "gut~es", "gut~en";
    comparative_strong_nom: "besser ~er", _, _, "besser ~e";
    comparative_strong_gen: _, _, "besser ~en", _;
    comparative_strong_dat: _, _, "besser ~em", "besser ~en";
    comparative_strong_acc: "besser ~en", "besser ~e", "besser ~es", "besser ~e";
    comparative_weak_nom: "besser ~e", "besser ~e", "besser ~e", "besser ~en";
    comparative_weak_gen: "besser ~en", "besser ~en", "besser ~en", "besser ~en";
    comparative_weak_dat: "besser ~en", "besser ~en", "besser ~en", "besser ~en";
    comparative_weak_acc: "besser ~en", "besser ~e", "besser ~e", "besser ~en";
    comparative_mixed_nom: _, "besser ~e", "besser ~es", "~en|besser ~en";
    comparative_mixed_gen: "besser ~en", "besser ~en", "besser ~en", "~en|besser ~en";
    comparative_mixed_dat: "besser ~en", "besser ~en", "besser ~en", "~en|besser ~en";
    comparative_mixed_acc: "besser ~en", "besser ~e", "besser ~es", "~en|besser ~en";
    superlative_strong_nom: "best~er", "best~e", "best~es", _;
    superlative_strong_gen: "best~en", _, _, _;
    superlative_strong_dat: "best~em", _, _, _;
    predicative: "gut~", "besser ~", "am besten ~";
    ),
    // Klageweise: 1 words (klageweise). Like Oberer, except in 40 cells: strong_nom f, strong_gen f, strong_gen n, strong_gen pl, strong_dat f, strong_dat n, ….
    declension!("klageweise", 1, "klag~~", like Oberer;
    strong_nom: _, "mi~t~", _, _;
    strong_gen: _, "mi~t~r", "mi~t~n", "mi~t~r";
    strong_dat: _, "mi~t~r", "mi~t~m", "mi~t~n";
    strong_acc: "mi~t~n", _, "mi~t~s", _;
    weak_nom: _, _, _, "mi~t~n";
    weak_gen: "mi~t~n", "mi~t~n", "mi~t~n", "mi~t~n";
    weak_dat: "mi~t~n", "mi~t~n", "mi~t~n", "mi~t~n";
    weak_acc: "mi~t~n", _, _, "mi~t~n";
    mixed_nom: "mi~t~r", _, "mi~t~s", "mi~t~n";
    mixed_gen: "mi~t~n", "mi~t~n", "mi~t~n", "mi~t~n";
    mixed_dat: "mi~t~n", "mi~t~n", "mi~t~n", "mi~t~n";
    mixed_acc: "mi~t~n", _, "mi~t~s", "mi~t~n";
    superlative_strong_nom: derived, derived, derived, _;
    superlative_strong_gen: derived, _, _, _;
    superlative_strong_dat: derived, _, _, _;
    predicative: "klag~~", _, _;
    ),
    // Kaufweise: 1 words (kaufweise).
    declension!("kaufweise", 1, "kauf~";
    strong_nom: _, "miet~", _, _;
    strong_gen: _, "miet~r", "miet~n", "miet~r";
    strong_dat: _, "miet~r", "miet~m", "miet~n";
    strong_acc: "miet~n", _, "miet~s", _;
    weak_nom: _, _, _, "miet~n";
    weak_gen: "miet~n", "miet~n", "miet~n", "miet~n";
    weak_dat: "miet~n", "miet~n", "miet~n", "miet~n";
    weak_acc: "miet~n", _, _, "miet~n";
    mixed_nom: "miet~r", _, "miet~s", "miet~n";
    mixed_gen: "miet~n", "miet~n", "miet~n", "miet~n";
    mixed_dat: "miet~n", "miet~n", "miet~n", "miet~n";
    mixed_acc: "miet~n", _, "miet~s", "miet~n";
    predicative: "kauf~", _, _;
    ),
    // Pachtweise: 1 words (pachtweise). Like Kaufweise, except in 35 cells: strong_nom f, strong_gen f, strong_gen n, strong_gen pl, strong_dat f, strong_dat n, ….
    declension!("pachtweise", 1, "pach~", like Kaufweise;
    strong_nom: _, "mie~", _, _;
    strong_gen: _, "mie~r", "mie~n", "mie~r";
    strong_dat: _, "mie~r", "mie~m", "mie~n";
    strong_acc: "mie~n", _, "mie~s", _;
    weak_nom: _, _, _, "mie~n";
    weak_gen: "mie~n", "mie~n", "mie~n", "mie~n";
    weak_dat: "mie~n", "mie~n", "mie~n", "mie~n";
    weak_acc: "mie~n", _, _, "mie~n";
    mixed_nom: "mie~r", _, "mie~s", "mie~n";
    mixed_gen: "mie~n", "mie~n", "mie~n", "mie~n";
    mixed_dat: "mie~n", "mie~n", "mie~n", "mie~n";
    mixed_acc: "mie~n", _, "mie~s", "mie~n";
    predicative: "pach~", _, _;
    ),
];
