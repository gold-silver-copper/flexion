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
    /// Like `duro`: 7,674 words.
    Duro,
    /// Like `loco`: 2,377 words.
    Loco,
    /// Like `ste`: 1,876 words.
    Ste,
    /// Like `natural`: 1,722 words.
    Natural,
    /// Like `ola`: 1,538 words.
    Ola,
    /// Like `mple`: 899 words.
    Mple,
    /// Like `turbador`: 809 words.
    Turbador,
    /// Like `rbio`: 693 words.
    Rbio,
    /// Like `anis`: 472 words.
    Anis,
    /// Like `tiagués`: 227 words.
    Tiagués,
    /// Like `lón`: 200 words.
    Lón,
    /// Like `gaz`: 37 words.
    Gaz,
    /// Like `cán`: 29 words.
    Cán,
    /// Like `rín`: 27 words.
    Rín,
    /// Like `eno`: 21 words.
    Eno,
    /// Like `fácil`: 17 words.
    Fácil,
    /// Like `fiel`: 13 words.
    Fiel,
    /// Like `aweonao`: 13 words.
    Aweonao,
    /// Like `dote`: 10 words.
    Dote,
    /// Like `mún`: 7 words.
    Mún,
    /// Like `tontolaba`: 6 words.
    Tontolaba,
    /// Like `cro`: 6 words.
    Cro,
    /// Like `afín`: 5 words.
    Afín,
    /// Like `scaldún`: 5 words.
    Scaldún,
    /// Like `áspero`: 4 words.
    Áspero,
    /// Like `rón`: 3 words.
    Rón,
    /// Like `isién`: 3 words.
    Isién,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 27] = [
        Declension::Duro,
        Declension::Loco,
        Declension::Ste,
        Declension::Natural,
        Declension::Ola,
        Declension::Mple,
        Declension::Turbador,
        Declension::Rbio,
        Declension::Anis,
        Declension::Tiagués,
        Declension::Lón,
        Declension::Gaz,
        Declension::Cán,
        Declension::Rín,
        Declension::Eno,
        Declension::Fácil,
        Declension::Fiel,
        Declension::Aweonao,
        Declension::Dote,
        Declension::Mún,
        Declension::Tontolaba,
        Declension::Cro,
        Declension::Afín,
        Declension::Scaldún,
        Declension::Áspero,
        Declension::Rón,
        Declension::Isién,
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
    // Duro: 7,674 words (gringo, juramentado, moro, helado, vero, dino).
    declension!("duro", 7674, "~o";
        masculine: "~o", "~os";
        feminine: "~a", "~as";
        superlative: "~ísimo";
        before_noun: "~o";
    ),
    // Loco: 2,377 words (guanaco, loco, seco, hueco, blanco, naco).
    declension!("loco", 2377, "~co";
        masculine: "~co", "~cos";
        feminine: "~ca", "~cas";
        superlative: "~quísimo";
        before_noun: "~co";
    ),
    // Ste: 1,876 words (libre, acre, mate, breve, fome, grave).
    declension!("ste", 1876, "~e";
        masculine: "~e", "~es";
        feminine: "~e", "~es";
        superlative: "~ísimo";
        before_noun: "~e";
    ),
    // Natural: 1,722 words (deal, amoral, fiscal, global, general, dual).
    declension!("natural", 1722, "~";
        masculine: "~", "~es";
        feminine: "~", "~es";
        superlative: "~ísimo";
        before_noun: "~";
    ),
    // Ola: 1,538 words (yanqui, fit, heavy, teen, gourmet, nazi).
    declension!("ola", 1538, "~";
        masculine: "~", "~s";
        feminine: "~", "~s";
        superlative: "~lísimo";
        before_noun: "~";
    ),
    // Mple: 899 words (vegetable, habitable, mole, tile, ostensible, fungible).
    declension!("mple", 899, "~~e";
        masculine: "~~e", "~~es";
        feminine: "~~e", "~~es";
        superlative: "~i~ísimo";
        before_noun: "~~e";
    ),
    // Turbador: 809 words (bel, bon, hablador, quemador, luchador, fundador).
    declension!("turbador", 809, "~";
        masculine: "~", "~es";
        feminine: "~a", "~as";
        superlative: "~ísimo";
        before_noun: "~";
    ),
    // Rbio: 693 words (ordinario, oratorio, zafio, utilitario, urinario, sabio).
    declension!("rbio", 693, "~~o";
        masculine: "~~o", "~~os";
        feminine: "~~a", "~~as";
        superlative: "~ís~mo";
        before_noun: "~~o";
    ),
    // Anis: 472 words (acid, tao, bio, beige, hot, cool).
    declension!("anis", 472, "~";
        masculine: "~", "~";
        feminine: "~", "~";
        superlative: "~ísimo";
        before_noun: "~";
    ),
    // Tiagués: 227 words (japonés, luxemburgués, ugandés, javanés, finlandés, galés).
    declension!("tiagués", 227, "~é~";
        masculine: "~é~", "~e~es";
        feminine: "~e~a", "~e~as";
        superlative: "~i~ísimo";
        before_noun: "~é~";
    ),
    // Lón: 200 words (juguetón, llorón, zorrastrón, nalgón, glotón, bretón).
    declension!("lón", 200, "~ó~";
        masculine: "~ó~", "~o~es";
        feminine: "~o~a", "~o~as";
        superlative: "~i~ísimo";
        before_noun: "~ó~";
    ),
    // Gaz: 37 words (fugaz, locuaz, eficaz, veloz, audaz, agraz).
    declension!("gaz", 37, "~z";
        masculine: "~z", "~ces";
        feminine: "~z", "~ces";
        superlative: "~císimo";
        before_noun: "~z";
    ),
    // Cán: 29 words (haragán, holgazán, galán, yagán, jayán, protalibán).
    declension!("cán", 29, "~á~";
        masculine: "~á~", "~a~es";
        feminine: "~a~a", "~a~as";
        superlative: "~a~ísimo";
        before_noun: "~á~";
    ),
    // Rín: 27 words (kirguís, benjamín, guapín, danzarín, benimerín, luanquín).
    declension!("rín", 27, "~í~";
        masculine: "~í~", "~i~es";
        feminine: "~i~a", "~i~as";
        superlative: "~i~ísimo";
        before_noun: "~í~";
    ),
    // Eno: 21 words (malo, mero, galo, ralo, güeno, ítalo).
    declension!("eno", 21, "~o";
        masculine: "~o", "~os";
        feminine: "~a", "~as";
        superlative: "~ísimo";
        before_noun: "~";
    ),
    // Fácil: 17 words (grácil, hábil, fácil, volátil, táctil, pulsátil).
    declension!("fácil", 17, "~á~";
        masculine: "~á~", "~á~es";
        feminine: "~á~", "~á~es";
        superlative: "~a~ísimo";
        before_noun: "~á~";
    ),
    // Fiel: 13 words (cruel, fiel, kakchikel, kakchiquel, nóvel, kaqchikel).
    declension!("fiel", 13, "~~";
        masculine: "~~", "~~es";
        feminine: "~~", "~~es";
        superlative: "~d~ísimo";
        before_noun: "~~";
    ),
    // Aweonao: 13 words (aweonao, ahueonao, pelao, zumbao, azarao, liao).
    declension!("aweonao", 13, "~ao";
        masculine: "~ao", "~aos";
        feminine: "~á", "~ás";
        superlative: "~ísimo";
        before_noun: "~ao";
    ),
    // Dote: 10 words (regordete, hugonote, amarrete, puercote, dote, rote).
    declension!("dote", 10, "~e";
        masculine: "~e", "~es";
        feminine: "~a", "~as";
        superlative: "~ísimo";
        before_noun: "~e";
    ),
    // Mún: 7 words (putún, fiacún, mún, común, jún, dún).
    declension!("mún", 7, "~ú~";
        masculine: "~ú~", "~u~es";
        feminine: "~ú~", "~u~es";
        superlative: "~i~ísimo";
        before_noun: "~ú~";
    ),
    // Tontolaba: 6 words (tontolaba, tontolava, opecientos, chucientos, potocientos, orrocientos).
    declension!("tontolaba", 6, "~o~";
        masculine: "~o~", "~o~s";
        feminine: "~a~", "~a~s";
        superlative: "~i~ísimo";
        before_noun: "~o~";
    ),
    // Cro: 6 words (pulcro, lumbosacro, lúdicro, cro, ucro, sacro).
    declension!("cro", 6, "~o";
        masculine: "~o", "~os";
        feminine: "~a", "~as";
        superlative: "~atísimo";
        before_noun: "~o";
    ),
    // Afín: 5 words (afín, mesín, chín, gentafín, nzolín).
    declension!("afín", 5, "~í~";
        masculine: "~í~", "~i~es";
        feminine: "~í~", "~i~es";
        superlative: "~i~ísimo";
        before_noun: "~í~";
    ),
    // Scaldún: 5 words (zunzún, stún, scaldún, skaldún, shtún).
    declension!("scaldún", 5, "~ú~";
        masculine: "~ú~", "~u~es";
        feminine: "~u~a", "~u~as";
        superlative: "~i~ísimo";
        before_noun: "~ú~";
    ),
    // Áspero: 4 words (áspero, ácero, áquero, ámero).
    declension!("áspero", 4, "á~o";
        masculine: "á~o", "á~os";
        feminine: "á~a", "á~as";
        superlative: "a~ísimo";
        before_noun: "á~o";
    ),
    // Rón: 3 words (rón, cotiledón, egrón).
    declension!("rón", 3, "~ó~";
        masculine: "~ó~", "~o~es";
        feminine: "~ó~", "~o~es";
        superlative: "~i~ísimo";
        before_noun: "~ó~";
    ),
    // Isién: 3 words (cortés, isién, setén).
    declension!("isién", 3, "~é~";
        masculine: "~é~", "~e~es";
        feminine: "~é~", "~e~es";
        superlative: "~i~ísimo";
        before_noun: "~é~";
    ),
];
