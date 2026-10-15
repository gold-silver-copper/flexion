//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~nen`, `tasainen`: the stem and `nen`). A stem in two pieces (`~~n`, `laiton`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `~nnes%ä~~`, of
//! `yhdestoista`), as the copy laws in `laws.rs` change it. A capital (`A`, `I`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `loppu`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem rest: \"~s- like tasainen\"`: the slot's cells are `tasainen`'s table's patterns
/// with `~s` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem rest: $s1:tt;)?
     $(stem rest_: $s2:tt;)?
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
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// What a declension adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
macro_rules! affix {
    () => {
        None
    };
    ($affix:literal) => {
        affix!($affix, $affix, "")
    };
    ($affix:literal, $other:literal, $letters:literal) => {
        Some(flexion::Affix {
            pattern: ::std::borrow::Cow::Borrowed($affix),
            other: ::std::borrow::Cow::Borrowed($other),
            letters: ::std::borrow::Cow::Borrowed($letters),
        })
    };
}

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
    /// Like `terävä`: 428 words.
    Terävä,
    /// Like `rikas`: 276 words.
    Rikas,
    /// Like `loppu`: 273 words.
    Loppu,
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
    /// Like `huono`: 122 words.
    Huono,
    /// Like `violetti`: 97 words.
    Violetti,
    /// Like `kuiva`: 87 words.
    Kuiva,
    /// Like `selvä`: 87 words.
    Selvä,
    /// Like `räjähtänyt`: 85 words.
    Räjähtänyt,
    /// Like `hylätty`: 85 words.
    Hylätty,
    /// Like `villi`: 63 words.
    Villi,
    /// Like `alvari`: 63 words.
    Alvari,
    /// Like `vahva`: 56 words.
    Vahva,
    /// Like `paksu`: 55 words.
    Paksu,
    /// Like `vapaa`: 49 words.
    Vapaa,
    /// Like `jörö`: 41 words.
    Jörö,
    /// Like `tanakka`: 39 words.
    Tanakka,
    /// Like `arka`: 34 words.
    Arka,
    /// Like `valmis`: 29 words.
    Valmis,
    /// Like `vähäläntä`: 23 words.
    Vähäläntä,
    /// Like `mykkä`: 22 words.
    Mykkä,
    /// Like `ekstrovertti`: 21 words.
    Ekstrovertti,
    /// Like `kahes`: 19 words.
    Kahes,
    /// Like `sähäkkä`: 19 words.
    Sähäkkä,
    /// Like `suuri`: 17 words.
    Suuri,
    /// Like `hopeoitu`: 15 words.
    Hopeoitu,
    /// Like `hotto`: 14 words.
    Hotto,
    /// Like `altis`: 13 words.
    Altis,
    /// Like `kesy`: 13 words.
    Kesy,
    /// Like `hatara`: 12 words.
    Hatara,
    /// Like `tuore`: 11 words.
    Tuore,
    /// Like `märkä`: 10 words.
    Märkä,
    /// Like `vihanta`: 10 words.
    Vihanta,
    /// Like `tiukka`: 9 words.
    Tiukka,
    /// Like `kevyt`: 9 words.
    Kevyt,
    /// Like `puhdas`: 9 words.
    Puhdas,
    /// Like `aito`: 8 words.
    Aito,
    /// Like `tiivis`: 8 words.
    Tiivis,
    /// Like `tosi`: 8 words.
    Tosi,
    /// Like `lämmin`: 8 words.
    Lämmin,
    /// Like `epis`: 8 words.
    Epis,
    /// Like `haurea`: 8 words.
    Haurea,
    /// Like `autio`: 8 words.
    Autio,
    /// Like `diskreetti`: 7 words.
    Diskreetti,
    /// Like `minunlaiseni`: 7 words.
    Minunlaiseni,
    /// Like `lörppö`: 7 words.
    Lörppö,
    /// Like `yhdestoista`: 6 words.
    Yhdestoista,
    /// Like `veiterä`: 6 words.
    Veiterä,
    /// Like `terve`: 6 words.
    Terve,
    /// Like `joltinenkin`: 6 words.
    Joltinenkin,
    /// Like `voipa`: 6 words.
    Voipa,
    /// Like `kiva`: 5 words.
    Kiva,
    /// Like `hoikka`: 5 words.
    Hoikka,
    /// Like `kahdeskymmenes`: 5 words.
    Kahdeskymmenes,
    /// Like `taaja`: 5 words.
    Taaja,
    /// Like `väkevöity`: 4 words.
    Väkevöity,
    /// Like `ohut`: 4 words.
    Ohut,
    /// Like `kahdestoista`: 4 words.
    Kahdestoista,
    /// Like `avoin`: 4 words.
    Avoin,
    /// Like `neljäskymmenes`: 4 words.
    Neljäskymmenes,
    /// Like `neljässadas`: 4 words.
    Neljässadas,
    /// Like `kahdessadas`: 4 words.
    Kahdessadas,
    /// Like `söde`: 4 words.
    Söde,
    /// Like `akuutti`: 3 words.
    Akuutti,
    /// Like `heikko`: 3 words.
    Heikko,
    /// Like `tykky`: 3 words.
    Tykky,
    /// Like `orpo`: 3 words.
    Orpo,
    /// Like `liki-`: 3 words.
    Liki,
    /// Like `rento`: 3 words.
    Rento,
    /// Like `ulko-`: 3 words.
    Ulko,
    /// Like `alaston`: 3 words.
    Alaston,
    /// Like `reipas`: 3 words.
    Reipas,
    /// Like `vänkyrä`: 3 words.
    Vänkyrä,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 83] = [
        Declension::Tasainen,
        Declension::Kansainvälinen,
        Declension::Laiton,
        Declension::Matala,
        Declension::Kummempi,
        Declension::Arin,
        Declension::Terävä,
        Declension::Rikas,
        Declension::Loppu,
        Declension::Oikea,
        Declension::Pehmeä,
        Declension::Sammunut,
        Declension::Paljas,
        Declension::Abstrakti,
        Declension::Huono,
        Declension::Violetti,
        Declension::Kuiva,
        Declension::Selvä,
        Declension::Räjähtänyt,
        Declension::Hylätty,
        Declension::Villi,
        Declension::Alvari,
        Declension::Vahva,
        Declension::Paksu,
        Declension::Vapaa,
        Declension::Jörö,
        Declension::Tanakka,
        Declension::Arka,
        Declension::Valmis,
        Declension::Vähäläntä,
        Declension::Mykkä,
        Declension::Ekstrovertti,
        Declension::Kahes,
        Declension::Sähäkkä,
        Declension::Suuri,
        Declension::Hopeoitu,
        Declension::Hotto,
        Declension::Altis,
        Declension::Kesy,
        Declension::Hatara,
        Declension::Tuore,
        Declension::Märkä,
        Declension::Vihanta,
        Declension::Tiukka,
        Declension::Kevyt,
        Declension::Puhdas,
        Declension::Aito,
        Declension::Tiivis,
        Declension::Tosi,
        Declension::Lämmin,
        Declension::Epis,
        Declension::Haurea,
        Declension::Autio,
        Declension::Diskreetti,
        Declension::Minunlaiseni,
        Declension::Lörppö,
        Declension::Yhdestoista,
        Declension::Veiterä,
        Declension::Terve,
        Declension::Joltinenkin,
        Declension::Voipa,
        Declension::Kiva,
        Declension::Hoikka,
        Declension::Kahdeskymmenes,
        Declension::Taaja,
        Declension::Väkevöity,
        Declension::Ohut,
        Declension::Kahdestoista,
        Declension::Avoin,
        Declension::Neljäskymmenes,
        Declension::Neljässadas,
        Declension::Kahdessadas,
        Declension::Söde,
        Declension::Akuutti,
        Declension::Heikko,
        Declension::Tykky,
        Declension::Orpo,
        Declension::Liki,
        Declension::Rento,
        Declension::Ulko,
        Declension::Alaston,
        Declension::Reipas,
        Declension::Vänkyrä,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 83] = [
    // Tasainen: 8,426 words (suomalainen, valkoinen, punainen, keltainen, graafinen, iloinen).
    declension!("tasainen", 8426, "~nen";
    stem rest: "~s- like tasainen";
    stem rest_: "~s- like tasainen";
    nom: "~nen", _;
    superlative: derived;
    ),
    // Kansainvälinen: 2,994 words (sininen, ensimmäinen, yksinäinen, täydellinen, edellinen, öinen). Like Tasainen, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("kansainvälinen", 2994, "~nen", like Tasainen;
    par: "~stä", "~siä";
    ine: "~sessI", "~sissI";
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
    stem rest: "~- like matala";
    stem rest_: "~i- like matala";
    nom: "~a", _;
    superlative: derived;
    ),
    // Kummempi: 479 words (vanhempi, vasempi, rannempi, enempi, helpompi, alempi).
    declension!("kummempi", 479, "~pi";
    stem rest: "~- like kummempi";
    stem rest_: "~pi- like matala";
    nom: "~pi", _;
    ),
    // Arin: 459 words (pisin, vasen, herein, villein, arin, väärin).
    declension!("arin", 459, "~n";
    stem rest: "~- like arin";
    nom: "~n", _;
    gen_: _, "~mpIen|~nten";
    ill: _, "~mpIin";
    com: _, "~mpine";
    ),
    // Terävä: 428 words (pitkä, hyvä, terävä, säälittävä, vedenpitävä, läpinäkyvä). Like Matala, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("terävä", 428, "~ä", like Matala;
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
    stem rest_: "~k~i- like rikas";
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
    // Loppu: 273 words (halal, chic, loppu, ylä-, haram, lerppu). Like Paksu with `ˇ` after each pattern.
    declension!("loppu", 273, "~ˇ", like Paksu with "ˇ";
    ),
    // Oikea: 178 words (valkea, ruskea, vaalea, kapea, korkea, nopea). Like Matala, except in 2 cells: par sg, par pl.
    declension!("oikea", 178, "~a", like Matala;
    stem rest_: "~i- like oikea";
    par: "~aA|~atA", "~itA";
    ),
    // Pehmeä: 167 words (leveä, vihreä, pyöreä, pehmeä, kirpeä, tärkeä). Like Terävä, except in 8 cells: par sg, par pl, ine pl, ela pl, ade pl, abl pl, ….
    declension!("pehmeä", 167, "~ä", like Terävä;
    stem rest_: "~i- like oikea";
    par: "~äA|~ätA", "~itä";
    ine: _, "~issI";
    ela: _, "~istä";
    ade: _, "~illä";
    abl: _, "~iltä";
    ess: _, "~inä";
    abe: _, "~ittä";
    ),
    // Sammunut: 162 words (vittuuntunut, masentunut, kuollut, oppinut, kulunut, kadonnut).
    declension!("sammunut", 162, "~ut";
    stem rest: "~- like sammunut";
    stem rest_: "~ei- like oikea";
    nom: "~ut", _;
    superlative: derived;
    ),
    // Paljas: 147 words (raskas, sairas, viisas, vanhurskas, kuolemansairas, kohtelias). Like Oikea, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("paljas", 147, "~s", like Oikea;
    stem rest_: "~i- like rikas";
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
    stem rest: "~- like abstrakti";
    stem rest_: "~- like abstrakti";
    nom: "~i", _;
    superlative: derived;
    ),
    // Huono: 122 words (indigo, iso, huono, vino, ufo, mono).
    declension!("huono", 122, "~";
    stem rest: "~- like huono";
    stem rest_: "~- like huono";
    nom: "~", _;
    superlative: derived;
    ),
    // Violetti: 97 words (violetti, pinkki, mahti, akuutti, ultravioletti, elegantti). Like Abstrakti with `ˇ` after each pattern.
    declension!("violetti", 97, "~iˇ", like Abstrakti with "ˇ";
    ),
    // Kuiva: 87 words (musta, ruma, kuuma, kuiva, soma, kova). Like Matala, except in 1 cell: comparative.
    declension!("kuiva", 87, "~a", like Matala;
    comparative: "~empi";
    ),
    // Selvä: 87 words (köyhä, kylmä, väärä, tyhmä, pyhä, syvä). Like Terävä, except in 1 cell: comparative.
    declension!("selvä", 87, "~ä", like Terävä;
    comparative: "~empi";
    ),
    // Räjähtänyt: 85 words (eristäytynyt, kyrpiintynyt, kyllästynyt, alikehittynyt, hävinnyt, ärsyyntynyt). Like Sammunut, except in 2 cells: nom sg, par sg.
    declension!("räjähtänyt", 85, "~yt", like Sammunut;
    nom: "~yt", _;
    par: "~yttA", _;
    ),
    // Hylätty: 85 words (kristitty, petetty, hillitty, venytetty, väitetty, kätketty). Like Kesy with `ˇ` after each pattern.
    declension!("hylätty", 85, "~ˇ", like Kesy with "ˇ";
    ),
    // Villi: 63 words (vissi, eri, villi, riski, läski, rigidi). Like Abstrakti, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("villi", 63, "~i", like Abstrakti;
    par: "~iä", "~ejI";
    ine: "~issI", "~eissI";
    ela: "~istä", "~eistä";
    ade: "~illä", "~eillä";
    abl: "~iltä", "~eiltä";
    ess: "~inä", "~einä";
    abe: "~ittä", "~eittä";
    ),
    // Alvari: 63 words (versaali, abnormaali, normaali, humaani, globaali, neutraali). Like Abstrakti, except in 2 cells: gen_ pl, par pl.
    declension!("alvari", 63, "~i", like Abstrakti;
    gen_: _, "~ien|~eiden|~eitten";
    par: _, "~eitA|~ejA";
    ),
    // Vahva: 56 words (vanha, vahva, hauska, laiha, paha, ikivanha). Like Kuiva, except in 11 cells: par pl, ine pl, ela pl, ade pl, abl pl, all pl, ….
    declension!("vahva", 56, "~a", like Kuiva;
    stem rest_: "~o- like huono";
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
    // Paksu: 55 words (paksu, naku, hullu, tuju, kalju, karu). Like Huono, except in 1 cell: ill sg.
    declension!("paksu", 55, "~", like Huono;
    ill: "~un", _;
    ),
    // Vapaa: 49 words (vapaa, harmaa, vakaa, epävakaa, vajaa, tummanharmaa). Like Oikea, except in 2 cells: par sg, ill sg.
    declension!("vapaa", 49, "~a", like Oikea;
    stem rest_: "~i- like rikas";
    par: "~atA", _;
    ill: "~aseen", _;
    ),
    // Jörö: 41 words (pöllö, söpö, kömpelö, hölmö, kylmähkö, lyhyehkö). Like Huono, except in 1 cell: ill sg.
    declension!("jörö", 41, "~", like Huono;
    ill: "~ön", _;
    ),
    // Tanakka: 39 words (terhakka, hailakka, navakka, lupsakka, solakka, tanakka).
    declension!("tanakka", 39, "~ka";
    stem rest: "~- like tanakka";
    nom: "~ka", _;
    gen_: _, "~oiden|~oitten|~kojen";
    ill: _, "~oihin|~koihin";
    com: _, "~koine";
    ),
    // Arka: 34 words (matta, arka, ehta, sanatarkka, rankka, valonarka). Like Vahva with `ˇ` after each pattern.
    declension!("arka", 34, "~aˇ", like Vahva with "ˇ";
    ),
    // Valmis: 29 words (kaunis, kallis, aulis, valmis, patavalmis, kullankallis).
    declension!("valmis", 29, "~~s";
    stem rest_: "~~i- like valmis";
    nom: "~~s", "~~it";
    gen_: "~~in", _;
    par: "~~stA", "~~itA";
    ine: "~~issA", "~~issA";
    ela: "~~istA", "~~istA";
    ill: "~~iseen", _;
    ade: "~~illA", "~~illA";
    abl: "~~iltA", "~~iltA";
    all: "~~ille", "~~ille";
    ess: "~~inA", "~~inA";
    tra: "~~iksi", "~~iksi";
    abe: "~~ittA", "~~ittA";
    ins: _, "~~in";
    comparative: "~~impi";
    superlative: "~e~n";
    ),
    // Vähäläntä: 23 words (läpimärkä, tylppä, pelkkä, läppä, vähänläntä, järkeenkäypä). Like Terävä with `ˇ` after each pattern.
    declension!("vähäläntä", 23, "~äˇ", like Terävä with "ˇ";
    ),
    // Mykkä: 22 words (jyrkkä, herkkä, valoherkkä, äkkijyrkkä, yliherkkä, räjähdysherkkä).
    declension!("mykkä", 22, "~kä";
    stem rest: "~- like mykkä";
    stem rest_: "~ki- like matala";
    nom: "~kä", _;
    superlative: derived;
    ),
    // Ekstrovertti: 21 words (ekstrovertti, resistentti, introvertti, diskreetti, inertti, eksellentti). Like Villi with `ˇ` after each pattern.
    declension!("ekstrovertti", 21, "~iˇ", like Villi with "ˇ";
    ),
    // Kahes: 19 words (viides, yhdeksäs, kymmenes, kuudes, seitsemäs, kahdeksas).
    declension!("kahes", 19, "~s";
    stem rest: "~- like kahes";
    stem rest_: "~nsi- like matala";
    nom: "~s", _;
    ),
    // Sähäkkä: 19 words (ärhäkkä, tömäkkä, jämäkkä, räväkkä, sähäkkä, möräkkä). Like Vänkyrä with `ˇ` after each pattern.
    declension!("sähäkkä", 19, "~äˇ", like Vänkyrä with "ˇ";
    ),
    // Suuri: 17 words (nuori, pieni, suuri, puoli, tyyni, pienenpieni). Like Kuiva, except in 2 cells: nom sg, superlative.
    declension!("suuri", 17, "~i", like Kuiva;
    stem rest: "~- like tasainen";
    stem rest_: "~- like tasainen";
    nom: "~i", _;
    superlative: derived;
    ),
    // Hopeoitu: 15 words (panssaroitu, sertifioitu, denaturoitu, suoti, pastöroitu, patentoitu).
    declension!("hopeoitu", 15, "~t~";
    stem rest_: "~t~- like huono";
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
    // Hotto: 14 words (rutto, helppo, ontto, veltto, suippo, melto). Like Huono with `ˇ` after each pattern.
    declension!("hotto", 14, "~ˇ", like Huono with "ˇ";
    ),
    // Altis: 13 words (raitis, altis, rietas, riskialtis, raivoraitis, vaikutusaltis).
    declension!("altis", 13, "~~s";
    stem rest_: "~t~i- like rikas";
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
    par: _, "~iA|~oitA";
    ine: _, "~oissA|~issA";
    ela: _, "~oistA|~istA";
    ill: _, "~iin|~oihin";
    ade: _, "~oillA|~illA";
    abl: _, "~oiltA|~iltA";
    all: _, "~oille|~ille";
    ess: _, "~oinA|~inA";
    tra: _, "~oiksi|~iksi";
    abe: _, "~oittA|~ittA";
    ins: _, "~oin|~in";
    com: _, "~oine|~ine";
    superlative: "~in";
    ),
    // Tuore: 11 words (tuore, ahne, kade, tyven, rahanahne, paahtotuore). Like Suuri, except in 4 cells: nom sg, par sg, par pl, ill sg.
    declension!("tuore", 11, "~", like Suuri;
    stem rest_: "~i- like oikea";
    nom: "~", _;
    par: "~ttA", "~itA";
    ill: "~eseen", _;
    ),
    // Märkä: 10 words (märkä, mätä, jäykkä, synkkä, mykkä, lyhyenläntä). Like Selvä with `ˇ` after each pattern.
    declension!("märkä", 10, "~äˇ", like Selvä with "ˇ";
    ),
    // Vihanta: 10 words (poppa, kaikkivoipa, ikivihanta, tasasoukka, vihanta, ainavihanta). Like Matala with `ˇ` after each pattern.
    declension!("vihanta", 10, "~aˇ", like Matala with "ˇ";
    ),
    // Tiukka: 9 words (tarkka, vaka, raaka, liika, epätarkka, tunnontarkka). Like Vahva, except in 6 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl.
    declension!("tiukka", 9, "~ka", like Vahva;
    stem rest_: "~ko- like huono";
    nom: "~ka", _;
    par: "~kaA", "~kojA";
    ill: "~kaAn", _;
    ess: "~kanA", "~koinA";
    ),
    // Kevyt: 9 words (kevyt, lyhyt, ehyt, pehmyt, höyhenenkevyt, ultrakevyt). Like Tuore with `` after each pattern (`t` after ~), except in 1 cell: ill sg.
    declension!("kevyt", 9, "~t", like Tuore with "", "t" after "~";
    ill: "~een", _;
    ),
    // Puhdas: 9 words (hidas, puhdas, ahdas, epäpuhdas, putipuhdas, tyylipuhdas). Like Paljas with `ˇ` after each pattern.
    declension!("puhdas", 9, "~sˇ", like Paljas with "ˇ";
    ),
    // Aito: 8 words (mieto, outo, aito, epäaito, leuto, uppo-outo). Like Hopeoitu, except in 1 cell: ill sg.
    declension!("aito", 8, "~t~", like Hopeoitu;
    ill: "~t~on", _;
    ),
    // Tiivis: 8 words (tiivis, tyyris, ilmatiivis, vesitiivis, kaasutiivis, äänitiivis). Like Valmis, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("tiivis", 8, "~~s", like Valmis;
    par: "~~stä", "~~itä";
    ine: "~~issI", "~~issI";
    ela: "~~istä", "~~istä";
    ade: "~~illä", "~~illä";
    abl: "~~iltä", "~~iltä";
    ess: "~~inä", "~~inä";
    abe: "~~ittä", "~~ittä";
    ),
    // Tosi: 8 words (uusi, täysi, tosi, epätosi, upouusi, täpötäysi). Like Tasainen, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("tosi", 8, "~si", like Tasainen;
    stem rest_: "~si- like matala";
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
    // Lämmin: 8 words (lämmin, uunilämmin, käyttölämmin, lypsylämmin, talvilämmin, tasalämmin). Like Alaston with `ˇ` after each pattern.
    declension!("lämmin", 8, "~nˇ", like Alaston with "ˇ";
    ),
    // Epis: 8 words (epis, veres, mustis, symppis, liikkis, gimis).
    declension!("epis", 8, "~~";
    nom: "~~", "~k~et";
    gen_: "~k~en", "~~ten|~k~ien";
    par: "~~tä", "~k~iä";
    ine: "~k~essI", "~k~issI";
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
    stem rest_: "~oi- like pro";
    par: _, "~oitA";
    comparative: "~ampi";
    ),
    // Autio: 8 words (pro, tax-free, okei, autio, gurmee, mainio). Like Huono, except in 2 cells: par sg, par pl.
    declension!("autio", 8, "~", like Huono;
    stem rest_: "~i- like pro";
    par: "~tA", "~ItA";
    ),
    // Diskreetti: 7 words (nätti, kiltti, definiitti, indefiniitti, negatiividefiniitti, positiividefiniitti).
    declension!("diskreetti", 7, "~ti";
    stem rest: "~- like diskreetti";
    stem rest_: "~t- like abstrakti";
    nom: "~ti", _;
    superlative: derived;
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
    // Lörppö: 7 words (kökkö, höttö, törppö, lörppö, tönkkö, könkkö). Like Jörö with `ˇ` after each pattern.
    declension!("lörppö", 7, "~ˇ", like Jörö with "ˇ";
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
    // Veiterä: 6 words (säkkärä, lättänä, kiverä, veiterä, kähärä, niverä). Like Terävä, except in 14 cells: gen_ pl, par pl, ine pl, ela pl, ill pl, ade pl, ….
    declension!("veiterä", 6, "~ä", like Terävä;
    gen_: _, "~ien|~öiden|~öitten";
    par: _, "~iA|~öitA";
    ine: _, "~öissA|~issA";
    ela: _, "~öistA|~istA";
    ill: _, "~iin|~öihin";
    ade: _, "~öillA|~illA";
    abl: _, "~öiltA|~iltA";
    all: _, "~öille|~ille";
    ess: _, "~öinA|~inA";
    tra: _, "~öiksi|~iksi";
    abe: _, "~öittA|~ittA";
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
    // Voipa: 6 words (halpa, hyvänsuopa, hupa, aikaansaapa, pahansuopa, sikahalpa).
    declension!("voipa", 6, "~pa";
    stem rest: "~- like voipa";
    stem rest_: "~pi- like matala";
    nom: "~pa", _;
    superlative: derived;
    ),
    // Kiva: 5 words (kiva, lila, nasta, liila, vinha). Like Vahva, except in 3 cells: ill pl, comparative, superlative.
    declension!("kiva", 5, "~a", like Vahva;
    ill: _, "~oIhin";
    comparative: "~empi|~ampi";
    superlative: "~oin|~in";
    ),
    // Hoikka: 5 words (hoikka, hyvinvoipa, voipa, soukka, pahoinvoipa). Like Kuiva with `ˇ` after each pattern.
    declension!("hoikka", 5, "~aˇ", like Kuiva with "ˇ";
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
    // Taaja: 5 words (risa, sekunda, taaja, rela, moittiva). Like Vahva, except in 1 cell: comparative.
    declension!("taaja", 5, "~a", like Vahva;
    comparative: "~ampi";
    ),
    // Väkevöity: 4 words (laiminlyöty, loppuunmyyty, kitkeröity, punapilleröity). Like Hopeoitu, except in 1 cell: ill sg.
    declension!("väkevöity", 4, "~t~", like Hopeoitu;
    ill: "~t~yn", _;
    ),
    // Ohut: 4 words (ohut, ainut, seitinohut, paperinohut).
    declension!("ohut", 4, "~ut";
    stem rest: "~u- like kevyt";
    stem rest_: "~ui- like valmis";
    nom: "~ut", _;
    superlative: derived;
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
    // Avoin: 4 words (avoin, valtoin, irtain, puoliavoin).
    declension!("avoin", 4, "~n";
    stem rest: "~- like avoin";
    nom: "~n", _;
    gen_: _, "~mIen|~nten";
    ill: _, "~mIin";
    com: _, "~mine";
    superlative: derived;
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
    // Söde: 4 words (beige, rage, söde, skrode). Like Huono, except in 1 cell: ill sg.
    declension!("söde", 4, "~", like Huono;
    ill: "~en", _;
    ),
    // Akuutti: 3 words (kuitti, suupaltti, efferentti). Like Diskreetti, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("akuutti", 3, "~ti", like Diskreetti;
    par: "~tiA", "~tejA";
    ine: "~IssA", "~eissA";
    ela: "~IstA", "~eistA";
    ade: "~IllA", "~eillA";
    abl: "~IltA", "~eiltA";
    ess: "~tinA", "~teinA";
    abe: "~IttA", "~eittA";
    ),
    // Heikko: 3 words (heikko, kolkko, hermoheikko).
    declension!("heikko", 3, "~k~";
    stem rest_: "~k~- like huono";
    nom: "~k~", "~~t";
    gen_: "~~n", _;
    par: "~k~A", "~k~jA";
    ine: "~~ssA", "~~issA";
    ela: "~~stA", "~~istA";
    ill: "~k~on", _;
    ade: "~~llA", "~~illA";
    abl: "~~ltA", "~~iltA";
    all: "~~lle", "~~ille";
    ess: "~k~nA", "~k~inA";
    tra: "~~ksi", "~~iksi";
    abe: "~~ttA", "~~ittA";
    ins: _, "~~in";
    comparative: "~~mpi";
    ),
    // Tykky: 3 words (välkky, säikky, järkky). Like Heikko, except in 1 cell: ill sg.
    declension!("tykky", 3, "~k~", like Heikko;
    ill: "~k~yn", _;
    ),
    // Orpo: 3 words (orpo, epäkelpo, kelpo).
    declension!("orpo", 3, "~p~";
    stem rest_: "~p~- like huono";
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
    stem rest_: "~t~- like huono";
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
    // Reipas: 3 words (reipas, valpas, vilpas).
    declension!("reipas", 3, "~~s";
    stem rest_: "~p~i- like rikas";
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
    // Vänkyrä: 3 words (känttyrä, vänkyrä, viheriä). Like Veiterä, except in 10 cells: par pl, ine pl, ela pl, ade pl, abl pl, all pl, ….
    declension!("vänkyrä", 3, "~ä", like Veiterä;
    stem rest_: "~öi- like pro";
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
];
