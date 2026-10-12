//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `masculine` (sg, pl)
/// `feminine` (sg, pl)
/// `superlative`
/// `before_noun`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     masculine: $c0:tt, $c1:tt;
     feminine: $c2:tt, $c3:tt;
     superlative: $c4:tt;
     before_noun: $c5:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `duro`: 9,762 words.
    Duro,
    /// Like `loco`: 3,762 words.
    Loco,
    /// Like `natural`: 2,687 words.
    Natural,
    /// Like `verde`: 2,318 words.
    Verde,
    /// Like `piola`: 1,736 words.
    Piola,
    /// Like `simple`: 1,138 words.
    Simple,
    /// Like `entrador`: 979 words.
    Entrador,
    /// Like `medio`: 872 words.
    Medio,
    /// Like `tuanis`: 509 words.
    Tuanis,
    /// Like `santiagués`: 262 words.
    Santiagués,
    /// Like `pelón`: 248 words.
    Pelón,
    /// Like `sagaz`: 49 words.
    Sagaz,
    /// Like `bacán`: 38 words.
    Bacán,
    /// Like `catrín`: 31 words.
    Catrín,
    /// Like `malo`: 30 words.
    Malo,
    /// Like `fácil`: 28 words.
    Fácil,
    /// Like `fiel`: 14 words.
    Fiel,
    /// Like `aweonao`: 14 words.
    Aweonao,
    /// Like `grandote`: 11 words.
    Grandote,
    /// Like `antiguo`: 8 words.
    Antiguo,
    /// Like `común`: 8 words.
    Común,
    /// Like `siniestro`: 7 words.
    Siniestro,
    /// Like `afín`: 7 words.
    Afín,
    /// Like `sacro`: 6 words.
    Sacro,
    /// Like `tontolaba`: 6 words.
    Tontolaba,
    /// Like `euscaldún`: 5 words.
    Euscaldún,
    /// Like `cortés`: 4 words.
    Cortés,
    /// Like `noruego`: 4 words.
    Noruego,
    /// Like `automotor`: 3 words.
    Automotor,
    /// Like `ambos`: 3 words.
    Ambos,
    /// Like `marrón`: 3 words.
    Marrón,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 31] = [
        Declension::Duro,
        Declension::Loco,
        Declension::Natural,
        Declension::Verde,
        Declension::Piola,
        Declension::Simple,
        Declension::Entrador,
        Declension::Medio,
        Declension::Tuanis,
        Declension::Santiagués,
        Declension::Pelón,
        Declension::Sagaz,
        Declension::Bacán,
        Declension::Catrín,
        Declension::Malo,
        Declension::Fácil,
        Declension::Fiel,
        Declension::Aweonao,
        Declension::Grandote,
        Declension::Antiguo,
        Declension::Común,
        Declension::Siniestro,
        Declension::Afín,
        Declension::Sacro,
        Declension::Tontolaba,
        Declension::Euscaldún,
        Declension::Cortés,
        Declension::Noruego,
        Declension::Automotor,
        Declension::Ambos,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 31] = [
    // Duro: 9,762 words (perro, centavo, derecho, paceño, gringo, barato).
    declension!("duro", 9762, "~o";
        masculine: "~o", "~os";
        feminine: "~a", "~as";
        superlative: "~ísimo";
        before_noun: "~o";
    ),
    // Loco: 3,762 words (chiapaneco, guanaco, loco, rauco, flamenco, polaco).
    declension!("loco", 3762, "~co";
        masculine: "~co", "~cos";
        feminine: "~ca", "~cas";
        superlative: "~quísimo";
        before_noun: "~co";
    ),
    // Natural: 2,687 words (deal, abactinal, abaxial, abdominal, aboral, accidental).
    declension!("natural", 2687, "~";
        masculine: "~", "~es";
        feminine: "~", "~es";
        superlative: "~ísimo";
        before_noun: "~";
    ),
    // Verde: 2,318 words (libre, andante, chihuahuense, estadounidense, acre, mate).
    declension!("verde", 2318, "~e";
        masculine: "~e", "~es";
        feminine: "~e", "~es";
        superlative: "~ísimo";
        before_noun: "~e";
    ),
    // Piola: 1,736 words (panista, foxista, yanqui, plata, simba, larva).
    declension!("piola", 1736, "~";
        masculine: "~", "~s";
        feminine: "~", "~s";
        superlative: "~lísimo";
        before_noun: "~";
    ),
    // Simple: 1,138 words (abominable, abrogable, acceptable, viable, vegetable, habitable).
    declension!("simple", 1138, "~~e";
        masculine: "~~e", "~~es";
        feminine: "~~e", "~~es";
        superlative: "~i~ísimo";
        before_noun: "~~e";
    ),
    // Entrador: 979 words (accelerator, bel, español, tractor, semiconductor, computador).
    declension!("entrador", 979, "~";
        masculine: "~", "~es";
        feminine: "~a", "~as";
        superlative: "~ísimo";
        before_noun: "~";
    ),
    // Medio: 872 words (monetario, bancario, ordinario, oratorio, zafio, utilitario).
    declension!("medio", 872, "~~o";
        masculine: "~~o", "~~os";
        feminine: "~~a", "~~as";
        superlative: "~ís~mo";
        before_noun: "~~o";
    ),
    // Tuanis: 509 words (gratis, acid, tao, bio, beige, magenta).
    declension!("tuanis", 509, "~";
        masculine: "~", "~";
        feminine: "~", "~";
        superlative: "~ísimo";
        before_noun: "~";
    ),
    // Santiagués: 262 words (japonés, luxemburgués, inglés, francés, ugandés, javanés).
    declension!("santiagués", 262, "~é~";
        masculine: "~é~", "~e~es";
        feminine: "~e~a", "~e~as";
        superlative: "~i~ísimo";
        before_noun: "~é~";
    ),
    // Pelón: 248 words (chingón, juguetón, cabrón, llorón, zorrastrón, cabezón).
    declension!("pelón", 248, "~ó~";
        masculine: "~ó~", "~o~es";
        feminine: "~o~a", "~o~as";
        superlative: "~i~ísimo";
        before_noun: "~ó~";
    ),
    // Sagaz: 49 words (rapaz, fugaz, feliz, locuaz, capaz, incapaz).
    declension!("sagaz", 49, "~z";
        masculine: "~z", "~ces";
        feminine: "~z", "~ces";
        superlative: "~císimo";
        before_noun: "~z";
    ),
    // Bacán: 38 words (catalán, musulmán, alemán, charlatán, chuán, haragán).
    declension!("bacán", 38, "~á~";
        masculine: "~á~", "~a~es";
        feminine: "~a~a", "~a~as";
        superlative: "~a~ísimo";
        before_noun: "~á~";
    ),
    // Catrín: 31 words (parlanchín, chiquitín, comodín, bailarín, borrachín, pequeñín).
    declension!("catrín", 31, "~í~";
        masculine: "~í~", "~i~es";
        feminine: "~i~a", "~i~as";
        superlative: "~i~ísimo";
        before_noun: "~í~";
    ),
    // Malo: 30 words (bueno, malo, primero, tercero, galo, decimotercero).
    declension!("malo", 30, "~o";
        masculine: "~o", "~os";
        feminine: "~a", "~as";
        superlative: "~ísimo";
        before_noun: "~";
    ),
    // Fácil: 28 words (grácil, versátil, hábil, fácil, estándar, frágil).
    declension!("fácil", 28, "~á~";
        masculine: "~á~", "~á~es";
        feminine: "~á~", "~á~es";
        superlative: "~a~ísimo";
        before_noun: "~á~";
    ),
    // Fiel: 14 words (cruel, novel, burel, fiel, infiel, kakchikel).
    declension!("fiel", 14, "~~";
        masculine: "~~", "~~es";
        feminine: "~~", "~~es";
        superlative: "~d~ísimo";
        before_noun: "~~";
    ),
    // Aweonao: 14 words (culiao, pringao, aweonao, ahueonao, deguabinao, pelao).
    declension!("aweonao", 14, "~ao";
        masculine: "~ao", "~aos";
        feminine: "~á", "~ás";
        superlative: "~ísimo";
        before_noun: "~ao";
    ),
    // Grandote: 11 words (grandote, cerote, chilote, regordete, franchute, hugonote).
    declension!("grandote", 11, "~e";
        masculine: "~e", "~es";
        feminine: "~a", "~as";
        superlative: "~ísimo";
        before_noun: "~e";
    ),
    // Antiguo: 8 words (antiguo, ingenuo, contiguo, ambiguo, exiguo, ignífluo).
    declension!("antiguo", 8, "~g~o";
        masculine: "~g~o", "~g~os";
        feminine: "~g~a", "~g~as";
        superlative: "~q~ísimo";
        before_noun: "~g~o";
    ),
    // Común: 8 words (común, supercomún, incomún, cajún, bidún, putún).
    declension!("común", 8, "~ú~";
        masculine: "~ú~", "~u~es";
        feminine: "~ú~", "~u~es";
        superlative: "~i~ísimo";
        before_noun: "~ú~";
    ),
    // Siniestro: 7 words (diestro, siniestro, ambidextro, triquetro, ambidiestro, conirrostro).
    declension!("siniestro", 7, "~i~o";
        masculine: "~i~o", "~i~os";
        feminine: "~i~a", "~i~as";
        superlative: "~i~ísimo";
        before_noun: "~i~o";
    ),
    // Afín: 7 words (afín, carmesín, machín, argentafín, cinzolín, coronigrís).
    declension!("afín", 7, "~í~";
        masculine: "~í~", "~i~es";
        feminine: "~í~", "~i~es";
        superlative: "~i~ísimo";
        before_noun: "~í~";
    ),
    // Sacro: 6 words (sacro, teucro, pulcro, lumbosacro, presacro, lúdicro).
    declension!("sacro", 6, "~o";
        masculine: "~o", "~os";
        feminine: "~a", "~as";
        superlative: "~atísimo";
        before_noun: "~o";
    ),
    // Tontolaba: 6 words (tropecientos, tontolaba, cuchucientos, sopotocientos, chorrocientos, tontolava).
    declension!("tontolaba", 6, "~o~";
        masculine: "~o~", "~o~s";
        feminine: "~a~", "~a~s";
        superlative: "~i~ísimo";
        before_noun: "~o~";
    ),
    // Euscaldún: 5 words (pastún, euscaldún, euskaldún, pashtún, zunzún).
    declension!("euscaldún", 5, "~ú~";
        masculine: "~ú~", "~u~es";
        feminine: "~u~a", "~u~as";
        superlative: "~i~ísimo";
        before_noun: "~ú~";
    ),
    // Cortés: 4 words (cortés, descortés, parisién, mosetén).
    declension!("cortés", 4, "~é~";
        masculine: "~é~", "~e~es";
        feminine: "~é~", "~e~es";
        superlative: "~i~ísimo";
        before_noun: "~é~";
    ),
    // Noruego: 4 words (noruego, cuellilargo, luengo, nuevingo).
    declension!("noruego", 4, "~ue~o";
        masculine: "~ue~o", "~ue~os";
        feminine: "~ue~a", "~ue~as";
        superlative: "~o~uísimo";
        before_noun: "~ue~o";
    ),
    // Automotor: 3 words (protector, automotor, audiomotor).
    declension!("automotor", 3, "~o~";
        masculine: "~o~", "~o~es";
        feminine: "~o~a|~~iz", "~o~as|~~ices";
        superlative: "~i~ísimo";
        before_noun: "~o~";
    ),
    // Ambos: 3 words (ambos, entrambos, entreambos).
    declension!("ambos", 3, "~o~";
        masculine: "~o~", "~o~s";
        feminine: "~a~", "~a~";
        superlative: "~i~ísimo";
        before_noun: "~o~";
    ),
    // Marrón: 3 words (marrón, dicotiledón, alegrón).
    declension!("marrón", 3, "~ó~";
        masculine: "~ó~", "~o~es";
        feminine: "~ó~", "~o~es";
        superlative: "~i~ísimo";
        before_noun: "~ó~";
    ),
];
