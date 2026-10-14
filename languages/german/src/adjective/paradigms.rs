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

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sup_as_comp`: one value read as another).
const SUP_AS_COMP: [u16; 147] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 145,
];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`strong_as_mixed`: one value read as another).
const STRONG_AS_MIXED: [u16; 147] = [
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 64, 65, 66, 67, 68, 69, 70, 71,
    72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 112, 113, 114,
    115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133,
    134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146,
];

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
    strong_nom: "~er", "~e", "~es", _;
    strong_gen: "~en", _, _, _;
    strong_dat: "~em", _, _, _;
    comparative_strong_nom: "~erer", "~ere", "~eres", _;
    comparative_strong_gen: "~eren", _, _, _;
    comparative_strong_dat: "~erem", _, _, _;
    superlative_strong_nom: "~ster", "~ste", "~stes", _;
    superlative_strong_gen: "~sten", _, _, _;
    superlative_strong_dat: "~stem", _, _, _;
    predicative: "~", _, "am ~sten";
    ),
    // Glatt: 1,862 words (absent, abstinent, absurd, blond, hold, fluid). Like Veraltet, except in 6 cells: superlative_strong_nom m, superlative_strong_nom f, superlative_strong_nom n, superlative_strong_gen m, superlative_strong_dat m, predicative sup.
    declension!("glatt", 1862, "~", like Veraltet;
    superlative_strong_nom: "~ester", "~este", "~estes", _;
    superlative_strong_gen: "~esten", _, _, _;
    superlative_strong_dat: "~estem", _, _, _;
    predicative: _, _, "am ~esten";
    ),
    // Überfinanziert: 439 words (gelaufen, gegessen, getrunken, gesprochen, gekommen, geboren). Like Veraltet, except in 3 cells: predicative pos, predicative comp, predicative sup.
    declension!("überfinanziert", 439, "~", like Veraltet;
    predicative: "ist ~|es ist ~|sind ~", "ist ~er", "ist am ~sten";
    ),
    // Edel: 102 words (übel, nobel, dunkel, simpel, sauer, teuer).
    declension!("edel", 102, "~e~";
    strong_nom: "~~er", "~~e", "~~es", _;
    strong_gen: "~~en", _, _, _;
    strong_dat: "~~em", _, _, _;
    comparative_strong_nom: "~~erer", "~~ere", "~~eres", _;
    comparative_strong_gen: "~~eren", _, _, _;
    comparative_strong_dat: "~~erem", _, _, _;
    superlative_strong_nom: "~e~ster", "~e~ste", "~e~stes", _;
    superlative_strong_gen: "~e~sten", _, _, _;
    superlative_strong_dat: "~e~stem", _, _, _;
    predicative: "~e~", _, "am ~e~sten";
    ),
    // Scharf: 50 words (warm, schwarz, stark, lang, scharf, schwach). Like Edel, except in 18 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, comparative_strong_nom m, ….
    declension!("scharf", 50, "~a~", like Edel;
    strong_nom: "~a~er", "~a~e", "~a~es", _;
    strong_gen: "~a~en", _, _, _;
    strong_dat: "~a~em", _, _, _;
    comparative_strong_nom: "~ä~erer", "~ä~ere", "~ä~eres", _;
    comparative_strong_gen: "~ä~eren", _, _, _;
    comparative_strong_dat: "~ä~erem", _, _, _;
    superlative_strong_nom: "~ä~ster", "~ä~ste", "~ä~stes", _;
    superlative_strong_gen: "~ä~sten", _, _, _;
    superlative_strong_dat: "~ä~stem", _, _, _;
    predicative: "~a~", "~ä~er", "am ~ä~sten";
    ),
    // Schwarz: 33 words (hart, kalt, galt, uralt, pechschwarz, steinalt). Like Scharf, except in 6 cells: superlative_strong_nom m, superlative_strong_nom f, superlative_strong_nom n, superlative_strong_gen m, superlative_strong_dat m, predicative sup.
    declension!("schwarz", 33, "~a~", like Scharf;
    superlative_strong_nom: "~ä~ester", "~ä~este", "~ä~estes", _;
    superlative_strong_gen: "~ä~esten", _, _, _;
    superlative_strong_dat: "~ä~estem", _, _, _;
    predicative: _, _, "am ~ä~esten";
    ),
    // Träge: 24 words (beige, broke, malle, fake, knorke, äußere). Like Edel, except in 17 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, comparative_strong_nom m, ….
    declension!("träge", 24, "~~", like Edel;
    strong_nom: "~~r", "~~", "~~s", _;
    strong_gen: "~~n", _, _, _;
    strong_dat: "~~m", _, _, _;
    comparative_strong_nom: "~~rer", "~~re", "~~res", _;
    comparative_strong_gen: "~~ren", _, _, _;
    comparative_strong_dat: "~~rem", _, _, _;
    superlative_strong_nom: "~st~r", "~st~", "~st~s", _;
    superlative_strong_gen: "~st~n", _, _, _;
    superlative_strong_dat: "~st~m", _, _, _;
    predicative: "~~", _, "am ~st~n";
    ),
    // Zeitnah: 11 words (nah, bodennah, oberflächennah, verbrauchernah, regierungsnah, berlinnah). Like Scharf, except in 6 cells: superlative_strong_nom m, superlative_strong_nom f, superlative_strong_nom n, superlative_strong_gen m, superlative_strong_dat m, predicative sup.
    declension!("zeitnah", 11, "~a~", like Scharf;
    superlative_strong_nom: "~äc~ster", "~äc~ste", "~äc~stes", _;
    superlative_strong_gen: "~äc~sten", _, _, _;
    superlative_strong_dat: "~äc~stem", _, _, _;
    predicative: _, _, "am ~äc~sten";
    ),
    // Groß: 11 words (groß, gross, riesengroß, mittelgroß, flächengroß, übergroß). Like Edel, except in 18 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, comparative_strong_nom m, ….
    declension!("groß", 11, "~o~", like Edel;
    strong_nom: "~o~er", "~o~e", "~o~es", _;
    strong_gen: "~o~en", _, _, _;
    strong_dat: "~o~em", _, _, _;
    comparative_strong_nom: "~ö~erer", "~ö~ere", "~ö~eres", _;
    comparative_strong_gen: "~ö~eren", _, _, _;
    comparative_strong_dat: "~ö~erem", _, _, _;
    superlative_strong_nom: "~ö~ter", "~ö~te", "~ö~tes", _;
    superlative_strong_gen: "~ö~ten", _, _, _;
    superlative_strong_dat: "~ö~tem", _, _, _;
    predicative: "~o~", "~ö~er", "am ~ö~ten";
    ),
    // Kurz: 9 words (kurz, gesund, schnurz, ungesund, superkurz, ultrakurz). Like Edel, except in 18 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, comparative_strong_nom m, ….
    declension!("kurz", 9, "~u~", like Edel;
    strong_nom: "~u~er", "~u~e", "~u~es", _;
    strong_gen: "~u~en", _, _, _;
    strong_dat: "~u~em", _, _, _;
    comparative_strong_nom: "~ü~erer", "~ü~ere", "~ü~eres", _;
    comparative_strong_gen: "~ü~eren", _, _, _;
    comparative_strong_dat: "~ü~erem", _, _, _;
    superlative_strong_nom: "~ü~ester", "~ü~este", "~ü~estes", _;
    superlative_strong_gen: "~ü~esten", _, _, _;
    superlative_strong_dat: "~ü~estem", _, _, _;
    predicative: "~u~", "~ü~er", "am ~ü~esten";
    ),
    // Dumm: 9 words (dumm, jung, klug, genug, unklug, altklug). Like Kurz, except in 6 cells: superlative_strong_nom m, superlative_strong_nom f, superlative_strong_nom n, superlative_strong_gen m, superlative_strong_dat m, predicative sup.
    declension!("dumm", 9, "~u~", like Kurz;
    superlative_strong_nom: "~ü~ster", "~ü~ste", "~ü~stes", _;
    superlative_strong_gen: "~ü~sten", _, _, _;
    superlative_strong_dat: "~ü~stem", _, _, _;
    predicative: _, _, "am ~ü~sten";
    ),
    // Unedel: 9 words (integer, schofel, illuster, makaber, speiübel, unedel). Like Edel, except in 6 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, predicative comp.
    declension!("unedel", 9, "~e~", like Edel;
    strong_nom: "~e~er", "~e~e", "~e~es", _;
    strong_gen: "~e~en", _, _, _;
    strong_dat: "~e~em", _, _, _;
    predicative: _, "~~er", _;
    ),
    // Oberer: 9 words (nächster, vorkantscher, mittlerer, unterer, oberer, überdünischer). Like Träge, except in 6 cells: comparative_strong_nom m, comparative_strong_nom f, comparative_strong_nom n, comparative_strong_gen m, comparative_strong_dat m, predicative pos.
    declension!("oberer", 9, "~~r", like Träge as SUP_AS_COMP;
    comparative_strong_nom: derived, derived, derived, _;
    comparative_strong_gen: derived, _, _, _;
    comparative_strong_dat: derived, _, _, _;
    predicative: "~~r", _, _;
    ),
    // Rot: 7 words (rot, roth, orthognath, prognath, dunckelroth, dunkelroth). Like Groß, except in 6 cells: superlative_strong_nom m, superlative_strong_nom f, superlative_strong_nom n, superlative_strong_gen m, superlative_strong_dat m, predicative sup.
    declension!("rot", 7, "~o~", like Groß;
    superlative_strong_nom: "~ö~ester", "~ö~este", "~ö~estes", _;
    superlative_strong_gen: "~ö~esten", _, _, _;
    superlative_strong_dat: "~ö~estem", _, _, _;
    predicative: _, _, "am ~ö~esten";
    ),
    // Grob: 7 words (grob, fromm, aerob, anaerob, lammfromm, storr). Like Groß, except in 6 cells: superlative_strong_nom m, superlative_strong_nom f, superlative_strong_nom n, superlative_strong_gen m, superlative_strong_dat m, predicative sup.
    declension!("grob", 7, "~o~", like Groß;
    superlative_strong_nom: "~ö~ster", "~ö~ste", "~ö~stes", _;
    superlative_strong_gen: "~ö~sten", _, _, _;
    superlative_strong_dat: "~ö~stem", _, _, _;
    predicative: _, _, "am ~ö~sten";
    ),
    // Lila: 6 words (rosa, lila, altrosa, hellrosa, schweinchenrosa, blasslila). Like Veraltet, except in 16 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, comparative_strong_nom m, ….
    declension!("lila", 6, "~", like Veraltet;
    strong_nom: "~ner", "~ne", "~nes", _;
    strong_gen: "~nen", _, _, _;
    strong_dat: "~nem", _, _, _;
    comparative_strong_nom: "~nerer", "~nere", "~neres", _;
    comparative_strong_gen: "~neren", _, _, _;
    comparative_strong_dat: "~nerem", _, _, _;
    superlative_strong_nom: "~nster", "~nste", "~nstes", _;
    superlative_strong_gen: "~nsten", _, _, _;
    superlative_strong_dat: "~nstem", _, _, _;
    predicative: _, _, "am ~nsten";
    ),
    // Hoch: 6 words (hoch, himmelhoch, ranghoch, wolkenhoch, hüfthoch, halbhoch). Like Groß, except in 7 cells: superlative_strong_nom m, superlative_strong_nom f, superlative_strong_nom n, superlative_strong_gen m, superlative_strong_dat m, predicative pos, ….
    declension!("hoch", 6, "~oc~", like Groß;
    superlative_strong_nom: "~öc~ster", "~öc~ste", "~öc~stes", _;
    superlative_strong_gen: "~öc~sten", _, _, _;
    superlative_strong_dat: "~öc~stem", _, _, _;
    predicative: "~oc~", _, "am ~öc~sten";
    ),
    // Naß: 6 words (naß, gewiß, ungewiß, kraß, blaß, totenblaß). Like Veraltet, except in 17 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, comparative_strong_nom m, ….
    declension!("naß", 6, "~ß", like Veraltet;
    strong_nom: "~sser", "~sse", "~sses", _;
    strong_gen: "~ssen", _, _, _;
    strong_dat: "~ssem", _, _, _;
    comparative_strong_nom: "~sserer", "~ssere", "~sseres", _;
    comparative_strong_gen: "~sseren", _, _, _;
    comparative_strong_dat: "~sserem", _, _, _;
    superlative_strong_nom: "~ssester", "~sseste", "~ssestes", _;
    superlative_strong_gen: "~ssesten", _, _, _;
    superlative_strong_dat: "~ssestem", _, _, _;
    predicative: "~ß", _, "am ~ssesten";
    ),
    // Arg: 3 words (arm, alt, arg). Like Veraltet, except in 18 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, comparative_strong_nom m, ….
    declension!("arg", 3, "a~", like Veraltet;
    strong_nom: "a~er", "a~e", "a~es", _;
    strong_gen: "a~en", _, _, _;
    strong_dat: "a~em", _, _, _;
    comparative_strong_nom: "ä~erer", "ä~ere", "ä~eres", _;
    comparative_strong_gen: "ä~eren", _, _, _;
    comparative_strong_dat: "ä~erem", _, _, _;
    superlative_strong_nom: "ä~ster", "ä~ste", "ä~stes", _;
    superlative_strong_gen: "ä~sten", _, _, _;
    superlative_strong_dat: "ä~stem", _, _, _;
    predicative: "a~", "ä~er", "am ä~sten";
    ),
    // Gutbesucht: 3 words (gutbesucht, guterhalten, gutgelaunt). Like Veraltet, except in 22 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, comparative_strong_nom m, ….
    declension!("gutbesucht", 3, "gut~", like Veraltet;
    strong_nom: "gut~er", "gut~e", "gut~es", _;
    strong_gen: "gut~en", _, _, _;
    strong_dat: "gut~em", _, _, _;
    comparative_strong_nom: "besser ~er", "besser ~e", "besser ~es", _;
    comparative_strong_gen: "besser ~en", _, _, _;
    comparative_strong_dat: "besser ~em", _, _, _;
    comparative_mixed_nom: _, _, _, "~en|besser ~en";
    comparative_mixed_gen: _, _, _, "~en|besser ~en";
    comparative_mixed_dat: _, _, _, "~en|besser ~en";
    comparative_mixed_acc: _, _, _, "~en|besser ~en";
    superlative_strong_nom: "best~er", "best~e", "best~es", _;
    superlative_strong_gen: "best~en", _, _, _;
    superlative_strong_dat: "best~em", _, _, _;
    predicative: "gut~", "besser ~", "am besten ~";
    ),
    // Klageweise: 1 words (klageweise). Like Oberer, except in 6 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, predicative pos.
    declension!("klageweise", 1, "klag~~", like Oberer as STRONG_AS_MIXED;
    strong_nom: "mi~t~r", "mi~t~", "mi~t~s", _;
    strong_gen: "mi~t~n", _, _, _;
    strong_dat: "mi~t~m", _, _, _;
    predicative: "klag~~", _, _;
    ),
    // Kaufweise: 1 words (kaufweise). Like Veraltet, except in 7 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, predicative pos, ….
    declension!("kaufweise", 1, "kauf~", like Veraltet as STRONG_AS_MIXED;
    strong_nom: "miet~r", "miet~", "miet~s", _;
    strong_gen: "miet~n", _, _, _;
    strong_dat: "miet~m", _, _, _;
    predicative: "kauf~", _, derived;
    ),
    // Pachtweise: 1 words (pachtweise). Like Kaufweise, except in 6 cells: strong_nom m, strong_nom f, strong_nom n, strong_gen m, strong_dat m, predicative pos.
    declension!("pachtweise", 1, "pach~", like Kaufweise;
    strong_nom: "mie~r", "mie~", "mie~s", _;
    strong_gen: "mie~n", _, _, _;
    strong_dat: "mie~m", _, _, _;
    predicative: "pach~", _, _;
    ),
];
