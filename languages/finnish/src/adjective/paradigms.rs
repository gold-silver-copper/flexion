//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A capital (`A`, `I`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

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
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `par` (sg, pl)
/// `ine` (sg, pl)
/// `ela` (sg, pl)
/// `ill` (sg, pl)
/// `ade` (sg, pl)
/// `abl` (sg, pl)
/// `all` (sg, pl)
/// `ess` (sg, pl)
/// `tra` (sg, pl)
/// `abe` (sg, pl)
/// `ins` (sg, pl)
/// `com` (sg, pl)
/// `comparative`
/// `superlative`
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem adjective_sg: $s0:tt;)?
     $(stem adjective: $s1:tt;)?
     $(stem adjective_pl: $s2:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(par: $c4:tt, $c5:tt;)?
     $(ine: $c6:tt, $c7:tt;)?
     $(ela: $c8:tt, $c9:tt;)?
     $(ill: $c10:tt, $c11:tt;)?
     $(ade: $c12:tt, $c13:tt;)?
     $(abl: $c14:tt, $c15:tt;)?
     $(all: $c16:tt, $c17:tt;)?
     $(ess: $c18:tt, $c19:tt;)?
     $(tra: $c20:tt, $c21:tt;)?
     $(abe: $c22:tt, $c23:tt;)?
     $(ins: $c24:tt, $c25:tt;)?
     $(com: $c26:tt, $c27:tt;)?
     $(comparative: $c28:tt;)?
     $(superlative: $c29:tt;)?
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
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 30] = [
    1, 1, 3, 3, 5, 5, 7, 7, 9, 9, 11, 11, 13, 13, 15, 15, 17, 17, 19, 19, 21, 21, 23, 23, 25, 25,
    27, 27, 28, 29,
];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `tasainen`: 8,426 words.
    Tasainen,
    /// Like `kansainvälinen`: 2,994 words.
    Kansainvälinen,
    /// Like `laiton`: 976 words.
    Laiton,
    /// Like `matala`: 818 words.
    Matala,
    /// Like `kummempi`: 479 words.
    Kummempi,
    /// Like `arin`: 459 words.
    Arin,
    /// Like `hyvä`: 428 words.
    Hyvä,
    /// Like `rikas`: 276 words.
    Rikas,
    /// Like `varattu`: 226 words.
    Varattu,
    /// Like `oikea`: 178 words.
    Oikea,
    /// Like `pehmeä`: 167 words.
    Pehmeä,
    /// Like `sammunut`: 162 words.
    Sammunut,
    /// Like `paljas`: 147 words.
    Paljas,
    /// Like `abstrakti`: 143 words.
    Abstrakti,
    /// Like `huono`: 128 words.
    Huono,
    /// Like `kuiva`: 87 words.
    Kuiva,
    /// Like `selvä`: 87 words.
    Selvä,
    /// Like `räjähtänyt`: 85 words.
    Räjähtänyt,
    /// Like `akuutti`: 85 words.
    Akuutti,
    /// Like `hylätty`: 73 words.
    Hylätty,
    /// Like `villi`: 63 words.
    Villi,
    /// Like `alvari`: 63 words.
    Alvari,
    /// Like `paksu`: 56 words.
    Paksu,
    /// Like `vahva`: 56 words.
    Vahva,
    /// Like `vapaa`: 49 words.
    Vapaa,
    /// Like `jörö`: 41 words.
    Jörö,
    /// Like `hopeoitu`: 40 words.
    Hopeoitu,
    /// Like `tanakka`: 39 words.
    Tanakka,
    /// Like `mykkä`: 38 words.
    Mykkä,
    /// Like `tiukka`: 38 words.
    Tiukka,
    /// Like `valmis`: 29 words.
    Valmis,
    /// Like `diskreetti`: 28 words.
    Diskreetti,
    /// Like `sähäkkä`: 19 words.
    Sähäkkä,
    /// Like `kahes`: 19 words.
    Kahes,
    /// Like `suuri`: 17 words.
    Suuri,
    /// Like `altis`: 13 words.
    Altis,
    /// Like `kesy`: 13 words.
    Kesy,
    /// Like `hatara`: 12 words.
    Hatara,
    /// Like `voipa`: 11 words.
    Voipa,
    /// Like `tuore`: 11 words.
    Tuore,
    /// Like `kevyt`: 9 words.
    Kevyt,
    /// Like `puhdas`: 9 words.
    Puhdas,
    /// Like `aito`: 9 words.
    Aito,
    /// Like `huoliteltu`: 9 words.
    Huoliteltu,
    /// Like `väkevöity`: 9 words.
    Väkevöity,
    /// Like `kavokki`: 9 words.
    Kavokki,
    /// Like `loppu`: 9 words.
    Loppu,
    /// Like `lämmin`: 8 words.
    Lämmin,
    /// Like `tiivis`: 8 words.
    Tiivis,
    /// Like `hotto`: 8 words.
    Hotto,
    /// Like `tosi`: 8 words.
    Tosi,
    /// Like `epis`: 8 words.
    Epis,
    /// Like `haurea`: 8 words.
    Haurea,
    /// Like `pro`: 8 words.
    Pro,
    /// Like `käypä`: 7 words.
    Käypä,
    /// Like `epäilty`: 7 words.
    Epäilty,
    /// Like `kiva`: 7 words.
    Kiva,
    /// Like `minunlaiseni`: 7 words.
    Minunlaiseni,
    /// Like `yhdestoista`: 6 words.
    Yhdestoista,
    /// Like `veiterä`: 6 words.
    Veiterä,
    /// Like `terve`: 6 words.
    Terve,
    /// Like `joltinenkin`: 6 words.
    Joltinenkin,
    /// Like `mätä`: 5 words.
    Mätä,
    /// Like `hoikka`: 5 words.
    Hoikka,
    /// Like `heikko`: 5 words.
    Heikko,
    /// Like `kahdeskymmenes`: 5 words.
    Kahdeskymmenes,
    /// Like `ohut`: 4 words.
    Ohut,
    /// Like `tuhti`: 4 words.
    Tuhti,
    /// Like `kahdestoista`: 4 words.
    Kahdestoista,
    /// Like `tykky`: 4 words.
    Tykky,
    /// Like `avoin`: 4 words.
    Avoin,
    /// Like `neljäskymmenes`: 4 words.
    Neljäskymmenes,
    /// Like `neljässadas`: 4 words.
    Neljässadas,
    /// Like `vihanta`: 4 words.
    Vihanta,
    /// Like `kahdessadas`: 4 words.
    Kahdessadas,
    /// Like `kenkku`: 4 words.
    Kenkku,
    /// Like `söde`: 4 words.
    Söde,
    /// Like `orpo`: 3 words.
    Orpo,
    /// Like `taaja`: 3 words.
    Taaja,
    /// Like `liki-`: 3 words.
    Liki,
    /// Like `rento`: 3 words.
    Rento,
    /// Like `ulko-`: 3 words.
    Ulko,
    /// Like `alaston`: 3 words.
    Alaston,
    /// Like `ehta`: 3 words.
    Ehta,
    /// Like `reipas`: 3 words.
    Reipas,
    /// Like `vähäläntä`: 3 words.
    Vähäläntä,
    /// Like `kökkö`: 3 words.
    Kökkö,
    /// Like `viheriä`: 3 words.
    Viheriä,
    /// Like `lörppö`: 3 words.
    Lörppö,
    /// Like `sippi`: 3 words.
    Sippi,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 90] = [
        Declension::Tasainen,
        Declension::Kansainvälinen,
        Declension::Laiton,
        Declension::Matala,
        Declension::Kummempi,
        Declension::Arin,
        Declension::Hyvä,
        Declension::Rikas,
        Declension::Varattu,
        Declension::Oikea,
        Declension::Pehmeä,
        Declension::Sammunut,
        Declension::Paljas,
        Declension::Abstrakti,
        Declension::Huono,
        Declension::Kuiva,
        Declension::Selvä,
        Declension::Räjähtänyt,
        Declension::Akuutti,
        Declension::Hylätty,
        Declension::Villi,
        Declension::Alvari,
        Declension::Paksu,
        Declension::Vahva,
        Declension::Vapaa,
        Declension::Jörö,
        Declension::Hopeoitu,
        Declension::Tanakka,
        Declension::Mykkä,
        Declension::Tiukka,
        Declension::Valmis,
        Declension::Diskreetti,
        Declension::Sähäkkä,
        Declension::Kahes,
        Declension::Suuri,
        Declension::Altis,
        Declension::Kesy,
        Declension::Hatara,
        Declension::Voipa,
        Declension::Tuore,
        Declension::Kevyt,
        Declension::Puhdas,
        Declension::Aito,
        Declension::Huoliteltu,
        Declension::Väkevöity,
        Declension::Kavokki,
        Declension::Loppu,
        Declension::Lämmin,
        Declension::Tiivis,
        Declension::Hotto,
        Declension::Tosi,
        Declension::Epis,
        Declension::Haurea,
        Declension::Pro,
        Declension::Käypä,
        Declension::Epäilty,
        Declension::Kiva,
        Declension::Minunlaiseni,
        Declension::Yhdestoista,
        Declension::Veiterä,
        Declension::Terve,
        Declension::Joltinenkin,
        Declension::Mätä,
        Declension::Hoikka,
        Declension::Heikko,
        Declension::Kahdeskymmenes,
        Declension::Ohut,
        Declension::Tuhti,
        Declension::Kahdestoista,
        Declension::Tykky,
        Declension::Avoin,
        Declension::Neljäskymmenes,
        Declension::Neljässadas,
        Declension::Vihanta,
        Declension::Kahdessadas,
        Declension::Kenkku,
        Declension::Söde,
        Declension::Orpo,
        Declension::Taaja,
        Declension::Liki,
        Declension::Rento,
        Declension::Ulko,
        Declension::Alaston,
        Declension::Ehta,
        Declension::Reipas,
        Declension::Vähäläntä,
        Declension::Kökkö,
        Declension::Viheriä,
        Declension::Lörppö,
        Declension::Sippi,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 90] = [
    // Tasainen: 8,426 words (suomalainen, valkoinen, punainen, keltainen, graafinen, iloinen).
    declension!("tasainen", 8426, "~nen";
    stem adjective_pl: "~s- like tasainen";
    nom: "~nen", "~set";
    gen_: "~sen", _;
    par: "~stA", "~siA";
    ine: "~sessA", "~sissA";
    ela: "~sestA", "~sistA";
    ill: "~seen", _;
    ade: "~sellA", "~sillA";
    abl: "~seltA", "~siltA";
    all: "~selle", "~sille";
    ess: "~senA", "~sinA";
    tra: "~seksi", "~siksi";
    abe: "~settA", "~sittA";
    ins: _, "~sin";
    comparative: "~sempi";
    ),
    // Kansainvälinen: 2,994 words (sininen, ensimmäinen, yksinäinen, täydellinen, edellinen, öinen). Like Tasainen, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("kansainvälinen", 2994, "~nen", like Tasainen;
    par: "~stä", "~siä";
    ine: "~sessI", "~sissä";
    ela: "~sestä", "~sistä";
    ade: "~sellI", "~sillä";
    abl: "~seltä", "~siltä";
    ess: "~senI", "~sinä";
    abe: "~settä", "~sittä";
    ),
    // Laiton: 976 words (peloton, kuolematon, ääretön, suunnaton, nimetön, uskomaton).
    declension!("laiton", 976, "~~n";
    nom: "~~n", "~t~mAt";
    gen_: "~t~mAn", "~t~mIen";
    par: "~~ntA", "~t~mIA";
    ine: "~t~mAssA", "~t~missA";
    ela: "~t~mAstA", "~t~mistA";
    ill: "~t~mAAn", "~t~miin";
    ade: "~t~mAllA", "~t~millA";
    abl: "~t~mAltA", "~t~miltA";
    all: "~t~mAlle", "~t~mille";
    ess: "~t~mAnA", "~t~minA";
    tra: "~t~mAksi", "~t~miksi";
    abe: "~t~mAttA", "~t~mittA";
    ins: _, "~t~min";
    com: _, "~t~mine";
    comparative: "~t~mAmpi";
    ),
    // Matala: 818 words (painava, lihava, matala, avara, kamala, ankara).
    declension!("matala", 818, "~a";
    stem adjective_pl: "~i- like matala";
    nom: "~a", "~at";
    gen_: "~an", _;
    par: "~aA", "~iA";
    ine: "~assA", "~issA";
    ela: "~astA", "~istA";
    ill: "~aAn", _;
    ade: "~allA", "~illA";
    abl: "~altA", "~iltA";
    all: "~alle", "~ille";
    ess: "~anA", "~inA";
    tra: "~aksi", "~iksi";
    abe: "~attA", "~ittA";
    ins: _, "~in";
    comparative: "~ampi";
    ),
    // Kummempi: 479 words (vanhempi, vasempi, rannempi, enempi, helpompi, alempi).
    declension!("kummempi", 479, "~pi";
    stem adjective: "~- like kummempi";
    stem adjective_pl: "~pi- like matala";
    nom: "~pi", _;
    ),
    // Arin: 459 words (pisin, vasen, herein, villein, arin, väärin).
    declension!("arin", 459, "~n";
    stem adjective: "~- like arin";
    nom: "~n", _;
    gen_: _, "~mpien|~nten";
    ill: _, "~mpiin";
    com: _, "~mpine";
    ),
    // Hyvä: 428 words (pitkä, hyvä, terävä, säälittävä, vedenpitävä, läpinäkyvä). Like Matala, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("hyvä", 428, "~ä", like Matala;
    nom: "~ä", "~ät";
    gen_: "~än", _;
    par: "~äA", _;
    ine: "~ässA", _;
    ela: "~ästA", _;
    ill: "~äAn", _;
    ade: "~ällA", _;
    abl: "~ältA", _;
    all: "~älle", _;
    ess: "~änA", _;
    tra: "~äksi", _;
    abe: "~ättA", _;
    comparative: "~ämpi";
    ),
    // Rikas: 276 words (sees, voimakas, rikas, tehokas, kirkas, mehukas).
    declension!("rikas", 276, "~~s";
    stem adjective_pl: "~k~i- like rikas";
    nom: "~~s", "~k~At";
    gen_: "~k~An", _;
    par: "~~stA", "~k~itA";
    ine: "~k~AssA", "~k~issA";
    ela: "~k~AstA", "~k~istA";
    ill: "~k~Aseen", _;
    ade: "~k~AllA", "~k~illA";
    abl: "~k~AltA", "~k~iltA";
    all: "~k~Alle", "~k~ille";
    ess: "~k~AnA", "~k~inA";
    tra: "~k~Aksi", "~k~iksi";
    abe: "~k~AttA", "~k~ittA";
    ins: _, "~k~in";
    comparative: "~k~Ampi";
    ),
    // Varattu: 226 words (matta, odotettu, varustettu, hyvinvarustettu, etä-, ympärileikattu).
    declension!("varattu", 226, "~t~";
    stem adjective_pl: "~t~- like varattu";
    nom: "~t~", "~~t";
    gen_: "~~n", _;
    par: "~t~A", "~t~jA";
    ine: "~~ssA", "~~issA";
    ela: "~~stA", "~~istA";
    ill: "~t~un", _;
    ade: "~~llA", "~~illA";
    abl: "~~ltA", "~~iltA";
    all: "~~lle", "~~ille";
    ess: "~t~nA", "~t~inA";
    tra: "~~ksi", "~~iksi";
    abe: "~~ttA", "~~ittA";
    ins: _, "~~in";
    comparative: "~~mpi";
    ),
    // Oikea: 178 words (valkea, ruskea, vaalea, kapea, korkea, nopea). Like Matala, except in 2 cells: par sg, par pl.
    declension!("oikea", 178, "~a", like Matala;
    stem adjective_pl: "~i- like oikea";
    par: "~aA|~ata", "~itA";
    ),
    // Pehmeä: 167 words (leveä, vihreä, pyöreä, pehmeä, kirpeä, tärkeä). Like Hyvä, except in 8 cells: par sg, par pl, ine pl, ela pl, ade pl, abl pl, ….
    declension!("pehmeä", 167, "~ä", like Hyvä;
    stem adjective_pl: "~i- like oikea";
    par: "~äA|~ätä", "~itä";
    ine: _, "~issä";
    ela: _, "~istä";
    ade: _, "~illä";
    abl: _, "~iltä";
    ess: _, "~inä";
    abe: _, "~ittä";
    ),
    // Sammunut: 162 words (vittuuntunut, masentunut, kuollut, oppinut, kulunut, kadonnut).
    declension!("sammunut", 162, "~ut";
    stem adjective_pl: "~ei- like oikea";
    nom: "~ut", "~eet";
    gen_: "~een", _;
    par: "~uttA", "~eitA";
    ine: "~eessA", "~eissA";
    ela: "~eestA", "~eistA";
    ill: "~eeseen", _;
    ade: "~eellA", "~eillA";
    abl: "~eeltA", "~eiltA";
    all: "~eelle", "~eille";
    ess: "~eenA", "~einA";
    tra: "~eeksi", "~eiksi";
    abe: "~eettA", "~eittA";
    ins: _, "~ein";
    comparative: "~eempi";
    ),
    // Paljas: 147 words (raskas, sairas, viisas, vanhurskas, kuolemansairas, kohtelias). Like Oikea, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("paljas", 147, "~s", like Oikea;
    stem adjective_pl: "~i- like rikas";
    nom: "~s", "~At";
    gen_: "~An", _;
    par: "~stA", _;
    ine: "~AssA", _;
    ela: "~AstA", _;
    ill: "~Aseen", _;
    ade: "~AllA", _;
    abl: "~AltA", _;
    all: "~Alle", _;
    ess: "~AnA", _;
    tra: "~Aksi", _;
    abe: "~AttA", _;
    comparative: "~Ampi";
    ),
    // Abstrakti: 143 words (khaki, syaani, radikaali, oranssi, turski, lokaali).
    declension!("abstrakti", 143, "~i";
    stem adjective_pl: "~- like abstrakti";
    nom: "~i", "~it";
    gen_: "~in", _;
    par: "~iA", "~ejA";
    ine: "~issA", "~eissA";
    ela: "~istA", "~eistA";
    ill: "~iin", _;
    ade: "~illA", "~eillA";
    abl: "~iltA", "~eiltA";
    all: "~ille", "~eille";
    ess: "~inA", "~einA";
    tra: "~iksi", "~eiksi";
    abe: "~ittA", "~eittA";
    ins: _, "~ein";
    comparative: "~impi";
    ),
    // Huono: 128 words (indigo, halal, iso, huono, vino, ufo).
    declension!("huono", 128, "~";
    stem adjective_pl: "~- like varattu";
    nom: "~", "~t";
    gen_: "~n", _;
    par: "~A", "~jA";
    ine: "~ssA", "~issA";
    ela: "~stA", "~istA";
    ill: "~on", _;
    ade: "~llA", "~illA";
    abl: "~ltA", "~iltA";
    all: "~lle", "~ille";
    ess: "~nA", "~inA";
    tra: "~ksi", "~iksi";
    abe: "~ttA", "~ittA";
    ins: _, "~in";
    comparative: "~mpi";
    ),
    // Kuiva: 87 words (musta, ruma, kuuma, kuiva, soma, kova). Like Matala, except in 1 cell: comparative.
    declension!("kuiva", 87, "~a", like Matala;
    comparative: "~empi";
    ),
    // Selvä: 87 words (köyhä, kylmä, väärä, tyhmä, pyhä, syvä). Like Hyvä, except in 1 cell: comparative.
    declension!("selvä", 87, "~ä", like Hyvä;
    comparative: "~empi";
    ),
    // Räjähtänyt: 85 words (eristäytynyt, kyrpiintynyt, kyllästynyt, alikehittynyt, hävinnyt, ärsyyntynyt). Like Sammunut, except in 2 cells: nom sg, par sg.
    declension!("räjähtänyt", 85, "~yt", like Sammunut;
    nom: "~yt", _;
    par: "~yttA", _;
    ),
    // Akuutti: 85 words (violetti, akuutti, ultravioletti, elegantti, ekvivalentti, irrelevantti).
    declension!("akuutti", 85, "~ti";
    stem adjective_pl: "~t- like abstrakti";
    nom: "~ti", "~It";
    gen_: "~In", _;
    par: "~tiA", "~tejA";
    ine: "~IssA", "~eissA";
    ela: "~IstA", "~eistA";
    ill: "~tiin", _;
    ade: "~IllA", "~eillA";
    abl: "~IltA", "~eiltA";
    all: "~Ille", "~eille";
    ess: "~tinA", "~teinA";
    tra: "~Iksi", "~eiksi";
    abe: "~IttA", "~eittA";
    ins: _, "~ein";
    comparative: "~Impi";
    ),
    // Hylätty: 73 words (kristitty, petetty, hillitty, venytetty, väitetty, kätketty). Like Varattu, except in 1 cell: ill sg.
    declension!("hylätty", 73, "~t~", like Varattu;
    ill: "~t~yn", _;
    ),
    // Villi: 63 words (vissi, eri, villi, riski, läski, rigidi). Like Abstrakti, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("villi", 63, "~i", like Abstrakti;
    par: "~iä", "~ejI";
    ine: "~issä", "~eissä";
    ela: "~istä", "~eistä";
    ade: "~illä", "~eillä";
    abl: "~iltä", "~eiltä";
    ess: "~inä", "~einä";
    abe: "~ittä", "~eittä";
    ),
    // Alvari: 63 words (versaali, abnormaali, normaali, humaani, globaali, neutraali). Like Abstrakti, except in 2 cells: gen_ pl, par pl.
    declension!("alvari", 63, "~i", like Abstrakti;
    gen_: _, "~ien|~eiden|~eitten";
    par: _, "~eitA|~eja";
    ),
    // Paksu: 56 words (paksu, naku, hullu, tuju, kalju, karu). Like Huono, except in 1 cell: ill sg.
    declension!("paksu", 56, "~", like Huono;
    ill: "~un", _;
    ),
    // Vahva: 56 words (vanha, vahva, hauska, laiha, paha, ikivanha). Like Kuiva, except in 11 cells: par pl, ine pl, ela pl, ade pl, abl pl, all pl, ….
    declension!("vahva", 56, "~a", like Kuiva;
    stem adjective_pl: "~o- like varattu";
    par: _, "~ojA";
    ine: _, "~oissA";
    ela: _, "~oistA";
    ade: _, "~oillA";
    abl: _, "~oiltA";
    all: _, "~oille";
    ess: _, "~oinA";
    tra: _, "~oiksi";
    abe: _, "~oittA";
    ins: _, "~oin";
    superlative: "~in";
    ),
    // Vapaa: 49 words (vapaa, harmaa, vakaa, epävakaa, vajaa, tummanharmaa). Like Oikea, except in 2 cells: par sg, ill sg.
    declension!("vapaa", 49, "~a", like Oikea;
    stem adjective_pl: "~i- like rikas";
    par: "~atA", _;
    ill: "~aseen", _;
    ),
    // Jörö: 41 words (pöllö, söpö, kömpelö, hölmö, kylmähkö, lyhyehkö). Like Huono, except in 1 cell: ill sg.
    declension!("jörö", 41, "~", like Huono;
    ill: "~ön", _;
    ),
    // Hopeoitu: 40 words (panssaroitu, sertifioitu, denaturoitu, kursivoitu, pastöroitu, patentoitu).
    declension!("hopeoitu", 40, "~t~";
    stem adjective_pl: "~t~- like varattu";
    nom: "~t~", "~d~t";
    gen_: "~d~n", _;
    par: "~t~A", "~t~jA";
    ine: "~d~ssA", "~d~issA";
    ela: "~d~stA", "~d~istA";
    ill: "~t~un", _;
    ade: "~d~llA", "~d~illA";
    abl: "~d~ltA", "~d~iltA";
    all: "~d~lle", "~d~ille";
    ess: "~t~nA", "~t~inA";
    tra: "~d~ksi", "~d~iksi";
    abe: "~d~ttA", "~d~ittA";
    ins: _, "~d~in";
    comparative: "~d~mpi";
    ),
    // Tanakka: 39 words (terhakka, hailakka, navakka, lupsakka, solakka, tanakka).
    declension!("tanakka", 39, "~ka";
    stem adjective: "~- like tanakka";
    nom: "~ka", _;
    gen_: _, "~oiden|~oitten|~kojen";
    ill: _, "~oihin|~koihin";
    com: _, "~koine";
    ),
    // Mykkä: 38 words (märkä, läpimärkä, jyrkkä, jäykkä, synkkä, mykkä).
    declension!("mykkä", 38, "~kä";
    stem adjective_pl: "~ki- like matala";
    nom: "~kä", "~It";
    gen_: "~In", _;
    par: "~käA", "~kiä";
    ine: "~IssA", "~issä";
    ela: "~IstA", "~istä";
    ill: "~käAn", _;
    ade: "~IllA", "~illä";
    abl: "~IltA", "~iltä";
    all: "~Ille", "~ille";
    ess: "~känA", "~kinä";
    tra: "~Iksi", "~iksi";
    abe: "~IttA", "~ittä";
    ins: _, "~in";
    comparative: "~empi";
    ),
    // Tiukka: 38 words (tarkka, vaka, arka, raaka, liika, sanatarkka). Like Tanakka, except in 12 cells: nom pl, gen_ sg, par pl, ine sg, ela sg, ade sg, ….
    declension!("tiukka", 38, "~ka", like Tanakka;
    stem adjective_pl: "~ko- like varattu";
    nom: _, "~It";
    gen_: "~In", _;
    par: _, "~kojA";
    ine: "~IssA", _;
    ela: "~IstA", _;
    ade: "~IllA", _;
    abl: "~IltA", _;
    all: "~Ille", _;
    tra: "~Iksi", _;
    abe: "~IttA", _;
    comparative: "~empi";
    superlative: "~in";
    ),
    // Valmis: 29 words (kaunis, kallis, aulis, valmis, patavalmis, kullankallis). Like Varattu, except in 12 cells: nom sg, nom pl, gen_ sg, par sg, par pl, ill sg, ….
    declension!("valmis", 29, "~~s", like Varattu as SG_AS_PL;
    stem adjective_pl: "~~i- like valmis";
    nom: "~~s", "~~it";
    gen_: "~~in", _;
    par: "~~stA", "~~itA";
    ill: "~~iseen", _;
    ess: "~~inA", "~~inA";
    ins: derived, _;
    com: derived, _;
    comparative: "~~impi";
    superlative: "~e~n";
    ),
    // Diskreetti: 28 words (nätti, ekstrovertti, resistentti, introvertti, kiltti, definiitti). Like Akuutti, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("diskreetti", 28, "~ti", like Akuutti;
    par: "~tiä", "~tejI";
    ine: "~Issä", "~eissä";
    ela: "~Istä", "~eistä";
    ade: "~Illä", "~eillä";
    abl: "~Iltä", "~eiltä";
    ess: "~tinä", "~teinä";
    abe: "~Ittä", "~eittä";
    ),
    // Sähäkkä: 19 words (ärhäkkä, tömäkkä, jämäkkä, räväkkä, sähäkkä, möräkkä).
    declension!("sähäkkä", 19, "~kä";
    stem adjective: "~- like sähäkkä";
    nom: "~kä", _;
    gen_: _, "~öiden|~öitten|~köjen";
    ill: _, "~öihin|~köihin";
    com: _, "~köine";
    ),
    // Kahes: 19 words (viides, yhdeksäs, kymmenes, kuudes, seitsemäs, kahdeksas).
    declension!("kahes", 19, "~s";
    stem adjective: "~- like kahes";
    stem adjective_pl: "~nsi- like matala";
    nom: "~s", _;
    ),
    // Suuri: 17 words (nuori, pieni, suuri, puoli, tyyni, pienenpieni). Like Kuiva, except in 13 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("suuri", 17, "~i", like Kuiva;
    stem adjective_pl: "~- like tasainen";
    nom: "~i", "~et";
    gen_: "~en", _;
    par: "~tA", _;
    ine: "~essA", _;
    ela: "~estA", _;
    ill: "~een", _;
    ade: "~ellA", _;
    abl: "~eltA", _;
    all: "~elle", _;
    ess: "~enA", _;
    tra: "~eksi", _;
    abe: "~ettA", _;
    ),
    // Altis: 13 words (raitis, altis, rietas, riskialtis, raivoraitis, vaikutusaltis).
    declension!("altis", 13, "~~s";
    stem adjective_pl: "~t~i- like rikas";
    nom: "~~s", "~t~it";
    gen_: "~t~in", _;
    par: "~~stA", "~t~itA";
    ine: "~t~issa", "~t~issa";
    ela: "~t~ista", "~t~ista";
    ill: "~t~iseen", _;
    ade: "~t~illa", "~t~illa";
    abl: "~t~ilta", "~t~ilta";
    all: "~t~ille", "~t~ille";
    ess: "~t~ina", "~t~ina";
    tra: "~t~iksi", "~t~iksi";
    abe: "~t~itta", "~t~itta";
    ins: _, "~t~in";
    comparative: "~t~impi";
    superlative: "~te~n";
    ),
    // Kesy: 13 words (kesy, pysty, väsy, kätsy, tyly, äksy). Like Huono, except in 1 cell: ill sg.
    declension!("kesy", 13, "~", like Huono;
    ill: "~yn", _;
    ),
    // Hatara: 12 words (hatara, kihara, itara, hutera, sikkara, tomera). Like Matala, except in 14 cells: gen_ pl, par pl, ine pl, ela pl, ill pl, ade pl, ….
    declension!("hatara", 12, "~a", like Matala;
    gen_: _, "~ien|~oiden|~oitten";
    par: _, "~iA|~oita";
    ine: _, "~oissA|~issa";
    ela: _, "~oistA|~ista";
    ill: _, "~iin|~oihin";
    ade: _, "~oillA|~illa";
    abl: _, "~oiltA|~ilta";
    all: _, "~oille|~ille";
    ess: _, "~oinA|~ina";
    tra: _, "~oiksi|~iksi";
    abe: _, "~oittA|~itta";
    ins: _, "~oin|~in";
    com: _, "~oine|~ine";
    superlative: "~in";
    ),
    // Voipa: 11 words (halpa, kaikkivoipa, hyvinvoipa, hyvänsuopa, hupa, aikaansaapa).
    declension!("voipa", 11, "~pa";
    stem adjective_pl: "~pi- like matala";
    nom: "~pa", "~vAt";
    gen_: "~vAn", _;
    par: "~paA", "~piA";
    ine: "~vAssA", "~vissA";
    ela: "~vAstA", "~vistA";
    ill: "~paAn", _;
    ade: "~vAllA", "~villA";
    abl: "~vAltA", "~viltA";
    all: "~vAlle", "~ville";
    ess: "~panA", "~pinA";
    tra: "~vAksi", "~viksi";
    abe: "~vAttA", "~vittA";
    ins: _, "~vin";
    comparative: "~vempi";
    ),
    // Tuore: 11 words (tuore, ahne, kade, tyven, rahanahne, paahtotuore). Like Suuri, except in 4 cells: nom sg, par sg, par pl, ill sg.
    declension!("tuore", 11, "~", like Suuri;
    stem adjective_pl: "~i- like oikea";
    nom: "~", _;
    par: "~ttA", "~itA";
    ill: "~eseen", _;
    ),
    // Kevyt: 9 words (kevyt, lyhyt, ehyt, pehmyt, höyhenenkevyt, ultrakevyt). Like Tuore, except in 2 cells: nom sg, ill sg.
    declension!("kevyt", 9, "~t", like Tuore;
    nom: "~t", _;
    ill: "~een", _;
    ),
    // Puhdas: 9 words (hidas, puhdas, ahdas, epäpuhdas, putipuhdas, tyylipuhdas).
    declension!("puhdas", 9, "~d~s";
    stem adjective_pl: "~t~i- like rikas";
    nom: "~d~s", "~t~at";
    gen_: "~t~an", _;
    par: "~d~sta", "~t~ita";
    ine: "~t~assa", "~t~issa";
    ela: "~t~asta", "~t~ista";
    ill: "~t~aseen", _;
    ade: "~t~alla", "~t~illa";
    abl: "~t~alta", "~t~ilta";
    all: "~t~alle", "~t~ille";
    ess: "~t~ana", "~t~ina";
    tra: "~t~aksi", "~t~iksi";
    abe: "~t~atta", "~t~itta";
    ins: _, "~t~in";
    comparative: "~t~ampi";
    ),
    // Aito: 9 words (mieto, outo, aito, epäaito, leuto, uppo-outo). Like Hopeoitu, except in 1 cell: ill sg.
    declension!("aito", 9, "~t~", like Hopeoitu;
    ill: "~t~on", _;
    ),
    // Huoliteltu: 9 words (huoliteltu, liioiteltu, melto, perusteltu, koeteltu, varjeltu).
    declension!("huoliteltu", 9, "~t~";
    stem adjective_pl: "~t~- like varattu";
    nom: "~t~", "~l~t";
    gen_: "~l~n", _;
    par: "~t~A", "~t~jA";
    ine: "~l~ssA", "~l~issA";
    ela: "~l~stA", "~l~istA";
    ill: "~t~un", _;
    ade: "~l~llA", "~l~illA";
    abl: "~l~ltA", "~l~iltA";
    all: "~l~lle", "~l~ille";
    ess: "~t~nA", "~t~inA";
    tra: "~l~ksi", "~l~iksi";
    abe: "~l~ttA", "~l~ittA";
    ins: _, "~l~in";
    comparative: "~l~mpi";
    ),
    // Väkevöity: 9 words (laiminlyöty, rekisteröity, käsintehty, väkevöity, itsetehty, räätälöity). Like Hopeoitu, except in 1 cell: ill sg.
    declension!("väkevöity", 9, "~t~", like Hopeoitu;
    ill: "~t~yn", _;
    ),
    // Kavokki: 9 words (pinkki, antiikki, uniikki, opaakki, ubiikki, feikki). Like Abstrakti, except in 6 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl.
    declension!("kavokki", 9, "~ki", like Abstrakti;
    stem adjective_pl: "~k- like abstrakti";
    nom: "~ki", _;
    par: "~kiA", "~kejA";
    ill: "~kiin", _;
    ess: "~kinA", "~keinA";
    ),
    // Loppu: 9 words (poppa, loppu, helppo, lerppu, tylppä, suippo). Like Varattu, except in 6 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl.
    declension!("loppu", 9, "~p~", like Varattu;
    stem adjective_pl: "~p~- like varattu";
    nom: "~p~", _;
    par: "~p~A", "~p~jA";
    ill: "~p~un", _;
    ess: "~p~nA", "~p~inA";
    ),
    // Lämmin: 8 words (lämmin, uunilämmin, käyttölämmin, lypsylämmin, talvilämmin, tasalämmin).
    declension!("lämmin", 8, "~m~n";
    stem adjective_pl: "~p~mi- like matala";
    nom: "~m~n", "~p~mät";
    gen_: "~p~män", _;
    par: "~m~ntä", "~p~miä";
    ine: "~p~mässä", "~p~missä";
    ela: "~p~mästä", "~p~mistä";
    ill: "~p~mään", _;
    ade: "~p~mällä", "~p~millä";
    abl: "~p~mältä", "~p~miltä";
    all: "~p~mälle", "~p~mille";
    ess: "~p~mänä", "~p~minä";
    tra: "~p~mäksi", "~p~miksi";
    abe: "~p~mättä", "~p~mittä";
    ins: _, "~p~min";
    comparative: "~p~mämpi";
    ),
    // Tiivis: 8 words (tiivis, tyyris, ilmatiivis, vesitiivis, kaasutiivis, äänitiivis). Like Valmis, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("tiivis", 8, "~~s", like Valmis;
    par: "~~stä", "~~itä";
    ine: "~~issä", "~~issä";
    ela: "~~istä", "~~istä";
    ade: "~~illä", "~~illä";
    abl: "~~iltä", "~~iltä";
    ess: "~~inä", "~~inä";
    abe: "~~ittä", "~~ittä";
    ),
    // Hotto: 8 words (rutto, ontto, veltto, loitto, lantto, kaltto). Like Varattu, except in 1 cell: ill sg.
    declension!("hotto", 8, "~t~", like Varattu;
    ill: "~t~on", _;
    ),
    // Tosi: 8 words (uusi, täysi, tosi, epätosi, upouusi, täpötäysi). Like Tasainen, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("tosi", 8, "~si", like Tasainen;
    stem adjective_pl: "~si- like matala";
    nom: "~si", "~det";
    gen_: "~den", _;
    par: "~ttA", _;
    ine: "~dessA", _;
    ela: "~destA", _;
    ill: "~teen", _;
    ade: "~dellA", _;
    abl: "~deltA", _;
    all: "~delle", _;
    ess: "~tenA", _;
    tra: "~deksi", _;
    abe: "~dettA", _;
    comparative: "~dempi";
    ),
    // Epis: 8 words (epis, veres, mustis, symppis, liikkis, gimis).
    declension!("epis", 8, "~~";
    nom: "~~", "~k~et";
    gen_: "~k~en", "~~ten|~k~ien";
    par: "~~tä", "~k~iä";
    ine: "~k~essI", "~k~issä";
    ela: "~k~estä", "~k~istä";
    ill: "~k~een", "~k~iin";
    ade: "~k~ellI", "~k~illä";
    abl: "~k~eltä", "~k~iltä";
    all: "~k~elle", "~k~ille";
    ess: "~k~enI", "~k~inä";
    tra: "~k~eksi", "~k~iksi";
    abe: "~k~että", "~k~ittä";
    ins: _, "~k~in";
    com: _, "~k~ine";
    comparative: "~k~empi";
    ),
    // Haurea: 8 words (purppura, sikkura, kikkara, kompura, kampura, kintsana). Like Vahva, except in 2 cells: par pl, comparative.
    declension!("haurea", 8, "~a", like Vahva;
    stem adjective_pl: "~oi- like pro";
    par: _, "~oitA";
    comparative: "~ampi";
    ),
    // Pro: 8 words (pro, tax-free, okei, autio, gurmee, mainio). Like Huono, except in 2 cells: par sg, par pl.
    declension!("pro", 8, "~", like Huono;
    stem adjective_pl: "~i- like pro";
    par: "~tA", "~itA";
    ),
    // Käypä: 7 words (käypä, järkeenkäypä, päällekäypä, sydämeenkäypä, edelläkäypä, laatuunkäypä). Like Voipa, except in 4 cells: nom sg, par sg, ill sg, ess sg.
    declension!("käypä", 7, "~pä", like Voipa;
    nom: "~pä", _;
    par: "~päA", _;
    ill: "~päAn", _;
    ess: "~pänA", _;
    ),
    // Epäilty: 7 words (epäilty, viljelty, kiistelty, loppuunkäsitelty, vuokrasäännelty, peitelty). Like Huoliteltu, except in 1 cell: ill sg.
    declension!("epäilty", 7, "~t~", like Huoliteltu;
    ill: "~t~yn", _;
    ),
    // Kiva: 7 words (kiva, risa, lila, nasta, liila, vinha). Like Vahva, except in 2 cells: comparative, superlative.
    declension!("kiva", 7, "~a", like Vahva;
    comparative: "~empi|~ampi";
    superlative: "~oin|~in";
    ),
    // Minunlaiseni: 7 words (minunlaiseni, sinunlaisesi, teidänlaisenne, meidänlaisemme, hänenlaisensa, heidänlaisensa).
    declension!("minunlaiseni", 7, "~e~";
    nom: "~e~", "~e~";
    gen_: "~e~", "~te~|~ie~";
    par: "~ta~", "~ia~";
    ine: "~essa~", "~issa~";
    ela: "~esta~", "~ista~";
    ill: "~ee~", "~ii~";
    ade: "~ella~", "~illa~";
    abl: "~elta~", "~ilta~";
    all: "~elle~", "~ille~";
    ess: "~ena~", "~ina~";
    tra: "~ekse~", "~ikse~";
    abe: "~etta~", "~itta~";
    com: _, "~ine~";
    ),
    // Yhdestoista: 6 words (yhdestoista, neljästoista, viidestoista, seitsemästoista, yhdeksästoista, viidesneljättä).
    declension!("yhdestoista", 6, "~s~~";
    nom: "~s~~", "~nnet~~";
    gen_: "~nnen~~", "~nsien~~";
    par: "~~tät~", "~nsiä~~";
    ine: "~nnessä~~", "~nsissä~~";
    ela: "~nnes%ä~~", "~nsis%ä~~";
    ill: "~nteen~~", "~nsiin~~";
    ade: "~nnellä~~", "~nsillä~~";
    abl: "~nnel%ä~~", "~nsil%ä~~";
    all: "~nnelle~~", "~nsille~~";
    ess: "~ntenä~~", "~nsinä~~";
    tra: "~nneksi~~", "~nsiksi~~";
    abe: "~nnet%ä~~", "~nsit%ä~~";
    ins: _, "~nsin~~";
    com: _, "~nsine~~";
    ),
    // Veiterä: 6 words (säkkärä, lättänä, kiverä, veiterä, kähärä, niverä). Like Hyvä, except in 14 cells: gen_ pl, par pl, ine pl, ela pl, ill pl, ade pl, ….
    declension!("veiterä", 6, "~ä", like Hyvä;
    gen_: _, "~ien|~öiden|~öitten";
    par: _, "~iA|~öitä";
    ine: _, "~öissA|~issä";
    ela: _, "~öistA|~istä";
    ill: _, "~iin|~öihin";
    ade: _, "~öillA|~illä";
    abl: _, "~öiltA|~iltä";
    all: _, "~öille|~ille";
    ess: _, "~öinA|~inä";
    tra: _, "~öiksi|~iksi";
    abe: _, "~öittA|~ittä";
    ins: _, "~öin|~in";
    com: _, "~öine|~ine";
    superlative: "~in";
    ),
    // Terve: 6 words (terve, perkele, handsfree, epäterve, tiine, perusterve). Like Pehmeä, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("terve", 6, "~", like Pehmeä;
    nom: "~", "~et";
    gen_: "~en", _;
    par: "~ttä", _;
    ine: "~essI", _;
    ela: "~estä", _;
    ill: "~eseen", _;
    ade: "~ellI", _;
    abl: "~eltä", _;
    all: "~elle", _;
    ess: "~enI", _;
    tra: "~eksi", _;
    abe: "~että", _;
    comparative: "~empi";
    ),
    // Joltinenkin: 6 words (ensimmäinenkolmatta, toinenkolmatta, kulloinenkin, joltinenkin, jommoinenkin, kumpainenkin).
    declension!("joltinenkin", 6, "~nen~";
    nom: "~nen~", "~set~";
    gen_: "~sen~", "~sten~|~sien~";
    par: "~sta~", "~sia~";
    ine: "~ses%a~", "~sis%a~";
    ela: "~ses%a~", "~sis%a~";
    ill: "~seen~", "~siin~";
    ade: "~sella~", "~silla~";
    abl: "~sel%a~", "~sil%a~";
    all: "~selle~", "~sille~";
    ess: "~sena~", "~sina~";
    tra: "~sek%i~", "~sik%i~";
    abe: "~set%a~", "~sit%a~";
    ins: _, "~sin~";
    com: _, "~sine~";
    ),
    // Mätä: 5 words (mätä, märkämätä, puolimätä, tyvimätä, läpimätä).
    declension!("mätä", 5, "~tä";
    stem adjective_pl: "~ti- like matala";
    nom: "~tä", "~dAt";
    gen_: "~dAn", _;
    par: "~täA", "~tiA";
    ine: "~dAssA", "~dissA";
    ela: "~dAstA", "~distA";
    ill: "~täAn", _;
    ade: "~dAllA", "~dillA";
    abl: "~dAltA", "~diltA";
    all: "~dAlle", "~dille";
    ess: "~tänA", "~tinA";
    tra: "~dAksi", "~diksi";
    abe: "~dAttA", "~dittA";
    ins: _, "~din";
    comparative: "~dempi";
    ),
    // Hoikka: 5 words (hoikka, tasasoukka, soukka, kuukka, erikoishoikka). Like Paljas, except in 7 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl, ….
    declension!("hoikka", 5, "~ka", like Paljas;
    stem adjective_pl: "~ki- like matala";
    nom: "~ka", _;
    par: "~kaA", "~kiA";
    ill: "~kaAn", _;
    ess: "~kanA", "~kinA";
    comparative: "~empi";
    ),
    // Heikko: 5 words (heikko, kolkko, sähköheikko, hermoheikko, vanhuudenheikko). Like Varattu, except in 6 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl.
    declension!("heikko", 5, "~k~", like Varattu;
    stem adjective_pl: "~k~- like varattu";
    nom: "~k~", _;
    par: "~k~A", "~k~jA";
    ill: "~k~on", _;
    ess: "~k~nA", "~k~inA";
    ),
    // Kahdeskymmenes: 5 words (kahdeskymmenes, kolmaskymmenes, kahdeksaskymmenes, kuudeskymmenes, viidestuhannes).
    declension!("kahdeskymmenes", 5, "~s~s";
    nom: "~s~s", "~nnet~nnet";
    gen_: "~nnen~nnen", "~nsien~nsien";
    par: "~tta~ttä", "~nsia~nsiä";
    ine: "~nnes%a~nnessä", "~nsis%a~nsissä";
    ela: "~nnes%a~nnestä", "~nsis%a~nsistä";
    ill: "~nteen~nteen", "~nsiin~nsiin";
    ade: "~nnella~nnellä", "~nsilla~nsillä";
    abl: "~nnel%a~nneltä", "~nsil%a~nsiltä";
    all: "~nnelle~nnelle", "~nsille~nsille";
    ess: "~ntena~ntenä", "~nsina~nsinä";
    tra: "~nnek%i~nneksi", "~nsik%i~nsiksi";
    abe: "~nnet%a~nnettä", "~nsit%a~nsittä";
    ins: _, "~nsin~nsin";
    com: _, "~nsine~nsine";
    ),
    // Ohut: 4 words (ohut, ainut, seitinohut, paperinohut).
    declension!("ohut", 4, "~ut";
    stem adjective_pl: "~ui- like valmis";
    nom: "~ut", "~uet";
    gen_: "~uen", _;
    par: "~uttA", "~uitA";
    ine: "~uessA", "~uissA";
    ela: "~uestA", "~uistA";
    ill: "~ueen", _;
    ade: "~uellA", "~uillA";
    abl: "~ueltA", "~uiltA";
    all: "~uelle", "~uille";
    ess: "~uenA", "~uinA";
    tra: "~ueksi", "~uiksi";
    abe: "~uettA", "~uittA";
    ins: _, "~uin";
    comparative: "~uempi";
    ),
    // Tuhti: 4 words (mahti, suoti, rehti, tuhti).
    declension!("tuhti", 4, "~ti";
    stem adjective_pl: "~t- like abstrakti";
    nom: "~ti", "~dit";
    gen_: "~din", _;
    par: "~tiA", "~tejA";
    ine: "~dissA", "~deissA";
    ela: "~distA", "~deistA";
    ill: "~tiin", _;
    ade: "~dillA", "~deillA";
    abl: "~diltA", "~deiltA";
    all: "~dille", "~deille";
    ess: "~tinA", "~teinA";
    tra: "~diksi", "~deiksi";
    abe: "~dittA", "~deittA";
    ins: _, "~dein";
    comparative: "~dimpi";
    ),
    // Kahdestoista: 4 words (kahdestoista, kolmastoista, kuudestoista, kahdeksastoista). Like Yhdestoista, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("kahdestoista", 4, "~s~~", like Yhdestoista;
    par: "~~tat~", "~nsia~~";
    ine: "~nnessa~~", "~nsissa~~";
    ela: "~nnes%a~~", "~nsis%a~~";
    ade: "~nnella~~", "~nsilla~~";
    abl: "~nnel%a~~", "~nsil%a~~";
    ess: "~ntena~~", "~nsina~~";
    abe: "~nnet%a~~", "~nsit%a~~";
    ),
    // Tykky: 4 words (välkky, säikky, tykky, järkky). Like Heikko, except in 1 cell: ill sg.
    declension!("tykky", 4, "~k~", like Heikko;
    ill: "~k~yn", _;
    ),
    // Avoin: 4 words (avoin, valtoin, irtain, puoliavoin).
    declension!("avoin", 4, "~n";
    nom: "~n", "~met";
    gen_: "~men", "~mien|~nten";
    par: "~ntA", "~miA";
    ine: "~messA", "~missA";
    ela: "~mestA", "~mistA";
    ill: "~meen", "~miin";
    ade: "~mellA", "~millA";
    abl: "~meltA", "~miltA";
    all: "~melle", "~mille";
    ess: "~menA", "~minA";
    tra: "~meksi", "~miksi";
    abe: "~mettA", "~mittA";
    ins: _, "~min";
    com: _, "~mine";
    comparative: "~mempi";
    ),
    // Neljäskymmenes: 4 words (yhdeksäskymmenes, neljäskymmenes, seitsemäskymmenes, viideskymmenes). Like Kahdeskymmenes, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("neljäskymmenes", 4, "~s~s", like Kahdeskymmenes;
    par: "~ttä~ttä", "~nsiä~nsiä";
    ine: "~nnes%ä~nnessä", "~nsis%ä~nsissä";
    ela: "~nnes%ä~nnestä", "~nsis%ä~nsistä";
    ade: "~nnellä~nnellä", "~nsillä~nsillä";
    abl: "~nnel%ä~nneltä", "~nsil%ä~nsiltä";
    ess: "~ntenä~ntenä", "~nsinä~nsinä";
    abe: "~nnet%ä~nnettä", "~nsit%ä~nsittä";
    ),
    // Neljässadas: 4 words (yhdeksässadas, seitsemässadas, neljässadas, viidessadas).
    declension!("neljässadas", 4, "~~s~s";
    nom: "~~s~s", "~nnet~~nnet";
    gen_: "~nnen~~nnen", "~nsien~~nsien";
    par: "~ttä~~tta", "~nsiä~~nsia";
    ine: "~nnes%ä~~nnessa", "~nsis%ä~~nsissa";
    ela: "~nnestä~~nnesta", "~nsistä~~nsista";
    ill: "~nteen~~nteen", "~nsiin~~nsiin";
    ade: "~nnellä~~nnella", "~nsillä~~nsilla";
    abl: "~nneltä~~nnelta", "~nsiltä~~nsilta";
    all: "~nnelle~~nnelle", "~nsille~~nsille";
    ess: "~ntenä~~ntena", "~nsinä~~nsina";
    tra: "~nnek%i~~nneksi", "~nsik%i~~nsiksi";
    abe: "~nnettä~~nnetta", "~nsittä~~nsitta";
    ins: _, "~nsin~~nsin";
    com: _, "~nsine~~nsine";
    ),
    // Vihanta: 4 words (ikivihanta, vihanta, ainavihanta, kesävihanta).
    declension!("vihanta", 4, "~ta";
    stem adjective: "~- like vihanta";
    nom: "~ta", _;
    gen_: _, "~tien|~tojen";
    ill: _, "~tiin|~toihin";
    com: _, "~tine|~toine";
    ),
    // Kahdessadas: 4 words (kolmassadas, kahdessadas, kuudessadas, kahdeksassadas). Like Neljässadas, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("kahdessadas", 4, "~~s~s", like Neljässadas;
    par: "~tta~~tta", "~nsia~~nsia";
    ine: "~nnes%a~~nnessa", "~nsis%a~~nsissa";
    ela: "~nnesta~~nnesta", "~nsista~~nsista";
    ade: "~nnella~~nnella", "~nsilla~~nsilla";
    abl: "~nnelta~~nnelta", "~nsilta~~nsilta";
    ess: "~ntena~~ntena", "~nsina~~nsina";
    abe: "~nnetta~~nnetta", "~nsitta~~nsitta";
    ),
    // Kenkku: 4 words (virkku, nilkku, kenkku, torkku). Like Heikko, except in 1 cell: ill sg.
    declension!("kenkku", 4, "~k~", like Heikko;
    ill: "~k~un", _;
    ),
    // Söde: 4 words (beige, rage, söde, skrode). Like Huono, except in 1 cell: ill sg.
    declension!("söde", 4, "~", like Huono;
    ill: "~en", _;
    ),
    // Orpo: 3 words (orpo, epäkelpo, kelpo).
    declension!("orpo", 3, "~p~";
    stem adjective_pl: "~p~- like varattu";
    nom: "~p~", "~v~t";
    gen_: "~v~n", _;
    par: "~p~A", "~p~jA";
    ine: "~v~ssa", "~v~issa";
    ela: "~v~sta", "~v~ista";
    ill: "~p~on", _;
    ade: "~v~lla", "~v~illa";
    abl: "~v~lta", "~v~ilta";
    all: "~v~lle", "~v~ille";
    ess: "~p~nA", "~p~inA";
    tra: "~v~ksi", "~v~iksi";
    abe: "~v~tta", "~v~itta";
    ins: _, "~v~in";
    comparative: "~v~mpi";
    ),
    // Taaja: 3 words (sekunda, taaja, moittiva). Like Vahva, except in 1 cell: comparative.
    declension!("taaja", 3, "~a", like Vahva;
    comparative: "~ampi";
    ),
    // Liki: 3 words (keski-, lähi-, liki-).
    declension!("liki-", 3, "~i-";
    nom: "~i-", _;
    par: "~eä", _;
    ade: "~ellA", _;
    abl: "~eltA", _;
    all: "~elle", _;
    ess: "~enään", _;
    ins: "~en", _;
    ),
    // Rento: 3 words (rento, hento, kokoonpantu).
    declension!("rento", 3, "~t~";
    stem adjective_pl: "~t~- like varattu";
    nom: "~t~", "~n~t";
    gen_: "~n~n", _;
    par: "~t~A", "~t~jA";
    ine: "~n~ssa", "~n~issa";
    ela: "~n~sta", "~n~ista";
    ill: "~t~on", _;
    ade: "~n~lla", "~n~illa";
    abl: "~n~lta", "~n~ilta";
    all: "~n~lle", "~n~ille";
    ess: "~t~nA", "~t~inA";
    tra: "~n~ksi", "~n~iksi";
    abe: "~n~tta", "~n~itta";
    ins: _, "~n~in";
    comparative: "~n~mpi";
    ),
    // Ulko: 3 words (taka-, ulko-, pika-).
    declension!("ulko-", 3, "~k~-";
    nom: "~k~-", _;
    par: "~k~a", _;
    ade: "~~lla", _;
    abl: "~~lta", _;
    all: "~~lle", _;
    ess: "~k~na", _;
    comparative: "~~mpi";
    superlative: "~~in";
    ),
    // Alaston: 3 words (alaston, ilkialaston, puolialaston). Like Kummempi, except in 10 cells: nom sg, gen_ pl, par sg, par pl, ill sg, ill pl, ….
    declension!("alaston", 3, "~n", like Kummempi;
    nom: "~n", _;
    gen_: _, "~mIen";
    par: "~ntA", "~mIA";
    ill: "~mAAn", "~mIin";
    ess: "~mAnA", "~minA";
    com: _, "~mine";
    comparative: "~mAmpi";
    ),
    // Ehta: 3 words (ehta, kaita, saita).
    declension!("ehta", 3, "~ta";
    stem adjective: "~- like ehta";
    nom: "~ta", _;
    gen_: _, "~tojen";
    ill: _, "~toIhin";
    com: _, "~toine";
    ),
    // Reipas: 3 words (reipas, valpas, vilpas).
    declension!("reipas", 3, "~~s";
    stem adjective_pl: "~p~i- like rikas";
    nom: "~~s", "~p~at";
    gen_: "~p~an", _;
    par: "~~stA", "~p~ita";
    ine: "~p~assa", "~p~issa";
    ela: "~p~asta", "~p~ista";
    ill: "~p~aseen", _;
    ade: "~p~alla", "~p~illa";
    abl: "~p~alta", "~p~ilta";
    all: "~p~alle", "~p~ille";
    ess: "~p~ana", "~p~ina";
    tra: "~p~aksi", "~p~iksi";
    abe: "~p~atta", "~p~itta";
    ins: _, "~p~in";
    comparative: "~p~ampi";
    ),
    // Vähäläntä: 3 words (lyhyenläntä, vähänläntä, vähäläntä).
    declension!("vähäläntä", 3, "~tä";
    stem adjective_pl: "~ti- like matala";
    nom: "~tä", "~nAt";
    gen_: "~nAn", _;
    par: "~täA", "~tiA";
    ine: "~nAssA", "~nissA";
    ela: "~nAstA", "~nistA";
    ill: "~täAn", _;
    ade: "~nAllA", "~nillA";
    abl: "~nAltA", "~niltA";
    all: "~nAlle", "~nille";
    ess: "~tänA", "~tinA";
    tra: "~nAksi", "~niksi";
    abe: "~nAttA", "~nittA";
    ins: _, "~nin";
    comparative: "~nempi";
    ),
    // Kökkö: 3 words (kökkö, tönkkö, könkkö). Like Heikko, except in 1 cell: ill sg.
    declension!("kökkö", 3, "~k~", like Heikko;
    ill: "~k~ön", _;
    ),
    // Viheriä: 3 words (känttyrä, vänkyrä, viheriä). Like Veiterä, except in 10 cells: par pl, ine pl, ela pl, ade pl, abl pl, all pl, ….
    declension!("viheriä", 3, "~ä", like Veiterä;
    stem adjective_pl: "~öi- like pro";
    par: _, "~öitA";
    ine: _, "~öissA";
    ela: _, "~öistA";
    ade: _, "~öillA";
    abl: _, "~öiltA";
    all: _, "~öille";
    ess: _, "~öinA";
    tra: _, "~öiksi";
    abe: _, "~öittA";
    ins: _, "~öin";
    ),
    // Lörppö: 3 words (törppö, lörppö, tylppö). Like Loppu, except in 1 cell: ill sg.
    declension!("lörppö", 3, "~p~", like Loppu;
    ill: "~p~ön", _;
    ),
    // Sippi: 3 words (skarppi, kemppi, sippi). Like Abstrakti, except in 6 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl.
    declension!("sippi", 3, "~pi", like Abstrakti;
    stem adjective_pl: "~p- like abstrakti";
    nom: "~pi", _;
    par: "~piA", "~pejA";
    ill: "~piin", _;
    ess: "~pinA", "~peinA";
    ),
];
