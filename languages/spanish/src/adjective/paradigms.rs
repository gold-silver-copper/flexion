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
/// `m` (sg, pl)
/// `f` (sg, pl)
/// `superlative`
/// `before_noun`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem adjective_m: $s0:tt;)?
     $(stem adjective_f: $s1:tt;)?
     $(stem superlative: $s2:tt;)?
     $(m: $c0:tt, $c1:tt;)?
     $(f: $c2:tt, $c3:tt;)?
     $(superlative: $c4:tt;)?
     $(before_noun: $c5:tt;)?
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
/// like that it reads (`superlative_as_before`: one value read as another).
const SUPERLATIVE_AS_BEFORE: [u16; 6] = [0, 1, 2, 3, 5, 5];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 6] = [0, 0, 2, 2, 4, 5];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`f_as_m`: one value read as another).
const F_AS_M: [u16; 6] = [0, 1, 0, 1, 4, 5];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `duro`: 14,448 words.
    Duro,
    /// Like `natural`: 6,818 words.
    Natural,
    /// Like `fiel`: 1,150 words.
    Fiel,
    /// Like `entrador`: 982 words.
    Entrador,
    /// Like `santiagués`: 262 words.
    Santiagués,
    /// Like `pelón`: 248 words.
    Pelón,
    /// Like `tuanis`: 232 words.
    Tuanis,
    /// Like `piola`: 188 words.
    Piola,
    /// Like `sagaz`: 49 words.
    Sagaz,
    /// Like `bacán`: 38 words.
    Bacán,
    /// Like `catrín`: 31 words.
    Catrín,
    /// Like `fácil`: 28 words.
    Fácil,
    /// Like `verde`: 14 words.
    Verde,
    /// Like `aweonao`: 14 words.
    Aweonao,
    /// Like `grandote`: 11 words.
    Grandote,
    /// Like `tontolaba`: 9 words.
    Tontolaba,
    /// Like `común`: 8 words.
    Común,
    /// Like `afín`: 7 words.
    Afín,
    /// Like `euscaldún`: 5 words.
    Euscaldún,
    /// Like `cortés`: 4 words.
    Cortés,
    /// Like `loco`: 3 words.
    Loco,
    /// Like `marrón`: 3 words.
    Marrón,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 22] = [
        Declension::Duro,
        Declension::Natural,
        Declension::Fiel,
        Declension::Entrador,
        Declension::Santiagués,
        Declension::Pelón,
        Declension::Tuanis,
        Declension::Piola,
        Declension::Sagaz,
        Declension::Bacán,
        Declension::Catrín,
        Declension::Fácil,
        Declension::Verde,
        Declension::Aweonao,
        Declension::Grandote,
        Declension::Tontolaba,
        Declension::Común,
        Declension::Afín,
        Declension::Euscaldún,
        Declension::Cortés,
        Declension::Loco,
        Declension::Marrón,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 22] = [
    // Duro: 14,448 words (chiapaneco, perro, centavo, derecho, monetario, paceño).
    declension!("duro", 14448, "~o";
    stem adjective_f: "~a- like duro";
    m: "~o", "~os";
    superlative: "~ísimo";
    before_noun: "~";
    ),
    // Natural: 6,818 words (deal, abactinal, abaxial, abdominal, aboral, accidental).
    declension!("natural", 6818, "~";
    m: "~", "~es";
    f: "~", "~es";
    superlative: "~ísimo";
    ),
    // Fiel: 1,150 words (abominable, abrogable, acceptable, viable, vegetable, habitable).
    declension!("fiel", 1150, "~~";
    m: "~~", "~~es";
    f: "~~", "~~es";
    superlative: "~d~ísimo";
    ),
    // Entrador: 982 words (accelerator, bel, español, tractor, semiconductor, computador). Like Natural, except in 0 cells: .
    declension!("entrador", 982, "~", like Natural as SUPERLATIVE_AS_BEFORE;
    stem adjective_f: "~a- like duro";
    ),
    // Santiagués: 262 words (japonés, luxemburgués, inglés, francés, ugandés, javanés).
    declension!("santiagués", 262, "~é~";
    stem adjective_f: "~e~a- like duro";
    m: "~é~", "~e~es";
    ),
    // Pelón: 248 words (chingón, juguetón, cabrón, llorón, zorrastrón, cabezón).
    declension!("pelón", 248, "~ó~";
    stem adjective_f: "~o~a- like duro";
    m: "~ó~", "~o~es";
    ),
    // Tuanis: 232 words (beige, magenta, hot, touch, ok, perfect). Like Natural, except in 0 cells: .
    declension!("tuanis", 232, "~", like Natural as PL_AS_SG;
    ),
    // Piola: 188 words (gratis, acid, cool, god, hard, fit). Like Natural, except in 1 cell: superlative.
    declension!("piola", 188, "~", like Natural;
    stem adjective_m: "~- like piola";
    stem adjective_f: "~- like duro";
    superlative: "~lísimo";
    ),
    // Sagaz: 49 words (rapaz, fugaz, feliz, locuaz, capaz, incapaz).
    declension!("sagaz", 49, "~z";
    m: "~z", "~ces";
    f: "~z", "~ces";
    superlative: "~císimo";
    ),
    // Bacán: 38 words (catalán, musulmán, alemán, charlatán, chuán, haragán).
    declension!("bacán", 38, "~á~";
    stem adjective_f: "~a~a- like duro";
    m: "~á~", "~a~es";
    ),
    // Catrín: 31 words (parlanchín, chiquitín, comodín, bailarín, borrachín, pequeñín).
    declension!("catrín", 31, "~í~";
    stem adjective_f: "~i~a- like duro";
    m: "~í~", "~i~es";
    ),
    // Fácil: 28 words (grácil, versátil, hábil, fácil, estándar, frágil). Like Bacán, except in 3 cells: m pl, f pl, superlative.
    declension!("fácil", 28, "~á~", like Bacán as F_AS_M;
    m: _, "~á~es";
    f: _, "~á~es";
    superlative: "~a~ísimo";
    ),
    // Verde: 14 words (padre, breve, triste, grave, grande, dulce). Like Natural, except in 2 cells: m sg, f sg.
    declension!("verde", 14, "~e", like Natural;
    m: "~e", _;
    f: "~e", _;
    ),
    // Aweonao: 14 words (culiao, pringao, aweonao, ahueonao, deguabinao, pelao).
    declension!("aweonao", 14, "~ao";
    stem adjective_m: "~ao- like piola";
    stem adjective_f: "~á- like duro";
    ),
    // Grandote: 11 words (grandote, cerote, chilote, regordete, franchute, hugonote). Like Entrador, except in 1 cell: m sg.
    declension!("grandote", 11, "~e", like Entrador;
    m: "~e", _;
    ),
    // Tontolaba: 9 words (ambos, tropecientos, entrambos, tontolaba, cuchucientos, sopotocientos).
    declension!("tontolaba", 9, "~o~";
    stem adjective_m: "~o~- like piola";
    stem adjective_f: "~a~- like duro";
    ),
    // Común: 8 words (común, supercomún, incomún, cajún, bidún, putún).
    declension!("común", 8, "~ú~";
    m: "~ú~", "~u~es";
    f: "~ú~", "~u~es";
    ),
    // Afín: 7 words (afín, carmesín, machín, argentafín, cinzolín, coronigrís). Like Catrín, except in 0 cells: .
    declension!("afín", 7, "~í~", like Catrín as F_AS_M;
    ),
    // Euscaldún: 5 words (pastún, euscaldún, euskaldún, pashtún, zunzún). Like Común, except in 0 cells: .
    declension!("euscaldún", 5, "~ú~", like Común;
    stem adjective_f: "~u~a- like duro";
    ),
    // Cortés: 4 words (cortés, descortés, parisién, mosetén). Like Santiagués, except in 0 cells: .
    declension!("cortés", 4, "~é~", like Santiagués as F_AS_M;
    ),
    // Loco: 3 words (loco, seco, flaco).
    declension!("loco", 3, "~co";
    stem adjective_m: "~co- like piola";
    stem adjective_f: "~ca- like duro";
    superlative: "~quísimo";
    ),
    // Marrón: 3 words (marrón, dicotiledón, alegrón). Like Pelón, except in 0 cells: .
    declension!("marrón", 3, "~ó~", like Pelón as F_AS_M;
    ),
];
