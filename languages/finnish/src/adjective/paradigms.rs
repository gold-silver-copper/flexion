//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~nen`, `tasainen`: the stem and `nen`). A stem in two pieces (`~~n`, `laiton`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `~nnes%ä~~`, of
//! `yhdestoista`), as the copy laws in `laws.rs` change it. A capital (`A`, `V`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `varattu`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem adjective_nom_pl: \"~s- like tasainen\"`: the slot's cells are `tasainen`'s table's patterns
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
     $(stem adjective_nom_sg: $s0:tt;)?
     $(stem adjective_nom_pl: $s1:tt;)?
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
    /// Like `tasainen`: 8,501 words.
    Tasainen,
    /// Like `kansainvälinen`: 2,919 words.
    Kansainvälinen,
    /// Like `laiton`: 976 words.
    Laiton,
    /// Like `matala`: 809 words.
    Matala,
    /// Like `kummempi`: 479 words.
    Kummempi,
    /// Like `arin`: 459 words.
    Arin,
    /// Like `terävä`: 414 words.
    Terävä,
    /// Like `varattu`: 340 words.
    Varattu,
    /// Like `rikas`: 276 words.
    Rikas,
    /// Like `huono`: 234 words.
    Huono,
    /// Like `oikea`: 178 words.
    Oikea,
    /// Like `pehmeä`: 167 words.
    Pehmeä,
    /// Like `sammunut`: 162 words.
    Sammunut,
    /// Like `paljas`: 147 words.
    Paljas,
    /// Like `abstrakti`: 144 words.
    Abstrakti,
    /// Like `kuiva`: 96 words.
    Kuiva,
    /// Like `selvä`: 96 words.
    Selvä,
    /// Like `violetti`: 90 words.
    Violetti,
    /// Like `räjähtänyt`: 85 words.
    Räjähtänyt,
    /// Like `alvari`: 63 words.
    Alvari,
    /// Like `villi`: 62 words.
    Villi,
    /// Like `hopeoitu`: 58 words.
    Hopeoitu,
    /// Like `vahva`: 56 words.
    Vahva,
    /// Like `vapaa`: 49 words.
    Vapaa,
    /// Like `tanakka`: 39 words.
    Tanakka,
    /// Like `mykkä`: 38 words.
    Mykkä,
    /// Like `tiukka`: 38 words.
    Tiukka,
    /// Like `valmis`: 29 words.
    Valmis,
    /// Like `nätti`: 26 words.
    Nätti,
    /// Like `sähäkkä`: 19 words.
    Sähäkkä,
    /// Like `kahes`: 19 words.
    Kahes,
    /// Like `suuri`: 17 words.
    Suuri,
    /// Like `tuore`: 17 words.
    Tuore,
    /// Like `heikko`: 16 words.
    Heikko,
    /// Like `altis`: 13 words.
    Altis,
    /// Like `hatara`: 12 words.
    Hatara,
    /// Like `voipa`: 11 words.
    Voipa,
    /// Like `kevyt`: 9 words.
    Kevyt,
    /// Like `kavokki`: 9 words.
    Kavokki,
    /// Like `puhdas`: 9 words.
    Puhdas,
    /// Like `autio`: 9 words.
    Autio,
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
    /// Like `käypä`: 7 words.
    Käypä,
    /// Like `kiva`: 7 words.
    Kiva,
    /// Like `minunlaiseni`: 7 words.
    Minunlaiseni,
    /// Like `yhdestoista`: 6 words.
    Yhdestoista,
    /// Like `veiterä`: 6 words.
    Veiterä,
    /// Like `joltinenkin`: 6 words.
    Joltinenkin,
    /// Like `mätä`: 5 words.
    Mätä,
    /// Like `hoikka`: 5 words.
    Hoikka,
    /// Like `kahdeskymmenes`: 5 words.
    Kahdeskymmenes,
    /// Like `ohut`: 4 words.
    Ohut,
    /// Like `tuhti`: 4 words.
    Tuhti,
    /// Like `kahdestoista`: 4 words.
    Kahdestoista,
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
    /// Like `orpo`: 3 words.
    Orpo,
    /// Like `taaja`: 3 words.
    Taaja,
    /// Like `rento`: 3 words.
    Rento,
    /// Like `alaston`: 3 words.
    Alaston,
    /// Like `ehta`: 3 words.
    Ehta,
    /// Like `reipas`: 3 words.
    Reipas,
    /// Like `vähäläntä`: 3 words.
    Vähäläntä,
    /// Like `vänkyrä`: 3 words.
    Vänkyrä,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 71] = [
        Declension::Tasainen,
        Declension::Kansainvälinen,
        Declension::Laiton,
        Declension::Matala,
        Declension::Kummempi,
        Declension::Arin,
        Declension::Terävä,
        Declension::Varattu,
        Declension::Rikas,
        Declension::Huono,
        Declension::Oikea,
        Declension::Pehmeä,
        Declension::Sammunut,
        Declension::Paljas,
        Declension::Abstrakti,
        Declension::Kuiva,
        Declension::Selvä,
        Declension::Violetti,
        Declension::Räjähtänyt,
        Declension::Alvari,
        Declension::Villi,
        Declension::Hopeoitu,
        Declension::Vahva,
        Declension::Vapaa,
        Declension::Tanakka,
        Declension::Mykkä,
        Declension::Tiukka,
        Declension::Valmis,
        Declension::Nätti,
        Declension::Sähäkkä,
        Declension::Kahes,
        Declension::Suuri,
        Declension::Tuore,
        Declension::Heikko,
        Declension::Altis,
        Declension::Hatara,
        Declension::Voipa,
        Declension::Kevyt,
        Declension::Kavokki,
        Declension::Puhdas,
        Declension::Autio,
        Declension::Tiivis,
        Declension::Tosi,
        Declension::Lämmin,
        Declension::Epis,
        Declension::Haurea,
        Declension::Käypä,
        Declension::Kiva,
        Declension::Minunlaiseni,
        Declension::Yhdestoista,
        Declension::Veiterä,
        Declension::Joltinenkin,
        Declension::Mätä,
        Declension::Hoikka,
        Declension::Kahdeskymmenes,
        Declension::Ohut,
        Declension::Tuhti,
        Declension::Kahdestoista,
        Declension::Avoin,
        Declension::Neljäskymmenes,
        Declension::Neljässadas,
        Declension::Vihanta,
        Declension::Kahdessadas,
        Declension::Orpo,
        Declension::Taaja,
        Declension::Rento,
        Declension::Alaston,
        Declension::Ehta,
        Declension::Reipas,
        Declension::Vähäläntä,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 71] = [
    // Tasainen: 8,501 words (suomalainen, valkoinen, punainen, keltainen, graafinen, iloinen).
    declension!("tasainen", 8501, "~nen";
    stem adjective_nom_pl: "~s- like tasainen";
    stem adjective_pl: "~s- like tasainen";
    nom: "~nen", _;
    superlative: derived;
    ),
    // Kansainvälinen: 2,919 words (sininen, ensimmäinen, yksinäinen, täydellinen, öinen, alkuperäinen). Like Tasainen, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("kansainvälinen", 2919, "~nen", like Tasainen;
    par: "~stä", "~siä";
    ine: "~sessä", "~sissä";
    ela: "~sestä", "~sistä";
    ade: "~sellä", "~sillä";
    abl: "~seltä", "~siltä";
    ess: "~senä", "~sinä";
    abe: "~settä", "~sittä";
    ),
    // Laiton: 976 words (peloton, kuolematon, ääretön, suunnaton, nimetön, uskomaton).
    declension!("laiton", 976, "~~n";
    stem adjective_pl: "~t~mi- like laiton";
    nom: "~~n", "~t~mAt";
    gen_: "~t~mAn", _;
    par: "~~ntA", "~t~miA";
    ine: "~t~mAssA", "~t~missA";
    ela: "~t~mAstA", "~t~mistA";
    ill: "~t~mAAn", _;
    ade: "~t~mAllA", "~t~millA";
    abl: "~t~mAltA", "~t~miltA";
    all: "~t~mAlle", "~t~mille";
    ess: "~t~mAnA", "~t~minA";
    tra: "~t~mAksi", "~t~miksi";
    abe: "~t~mAttA", "~t~mittA";
    ins: _, "~t~min";
    comparative: "~t~mAmpi";
    ),
    // Matala: 809 words (painava, lihava, matala, avara, kamala, ankara).
    declension!("matala", 809, "~a";
    stem adjective_nom_pl: "~- like matala";
    stem adjective_pl: "~i- like laiton";
    nom: "~a", _;
    superlative: derived;
    ),
    // Kummempi: 479 words (vanhempi, vasempi, rannempi, enempi, helpompi, alempi).
    declension!("kummempi", 479, "~pi";
    stem adjective_nom_pl: "~- like kummempi";
    stem adjective_pl: "~pi- like laiton";
    nom: "~pi", _;
    ),
    // Arin: 459 words (pisin, vasen, herein, villein, arin, väärin).
    declension!("arin", 459, "~n";
    stem adjective_nom_pl: "~- like arin";
    nom: "~n", _;
    gen_: _, "~mpVen|~nten";
    ill: _, "~mpVin";
    com: _, "~mpine";
    ),
    // Terävä: 414 words (pitkä, hyvä, terävä, säälittävä, vedenpitävä, läpinäkyvä). Like Matala, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("terävä", 414, "~ä", like Matala;
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
    // Varattu: 340 words (halal, rutto, matta, poppa, loppu, haram). Like Huono with `ˇ` after each pattern.
    declension!("varattu", 340, "~ˇ", like Huono with "ˇ";
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
    // Huono: 234 words (beige, indigo, iso, paksu, huono, vino).
    declension!("huono", 234, "~";
    stem adjective_nom_pl: "~- like huono";
    stem adjective_pl: "~- like huono";
    nom: "~", _;
    superlative: derived;
    ),
    // Oikea: 178 words (valkea, ruskea, vaalea, kapea, korkea, nopea). Like Matala, except in 2 cells: par sg, par pl.
    declension!("oikea", 178, "~a", like Matala;
    stem adjective_pl: "~i- like oikea";
    par: "~aA|~atA", "~itA";
    ),
    // Pehmeä: 167 words (leveä, vihreä, pyöreä, pehmeä, kirpeä, tärkeä). Like Terävä, except in 8 cells: par sg, par pl, ine pl, ela pl, ade pl, abl pl, ….
    declension!("pehmeä", 167, "~ä", like Terävä;
    stem adjective_pl: "~i- like oikea";
    par: "~äA|~ätA", "~itä";
    ine: _, "~issä";
    ela: _, "~istä";
    ade: _, "~illä";
    abl: _, "~iltä";
    ess: _, "~inä";
    abe: _, "~ittä";
    ),
    // Sammunut: 162 words (vittuuntunut, masentunut, kuollut, oppinut, kulunut, kadonnut).
    declension!("sammunut", 162, "~ut";
    stem adjective_nom_pl: "~- like sammunut";
    stem adjective_pl: "~ei- like oikea";
    nom: "~ut", _;
    superlative: derived;
    ),
    // Paljas: 147 words (raskas, sairas, viisas, vanhurskas, kuolemansairas, kohtelias). Like Oikea, except in 15 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("paljas", 147, "~s", like Oikea;
    nom: "~s", "~At";
    gen_: "~An", _;
    par: "~stA", _;
    ine: "~AssA", _;
    ela: "~AstA", _;
    ill: "~AseVn", "~isVin";
    ade: "~AllA", _;
    abl: "~AltA", _;
    all: "~Alle", _;
    ess: "~AnA", _;
    tra: "~Aksi", _;
    abe: "~AttA", _;
    comparative: "~Ampi";
    ),
    // Abstrakti: 144 words (khaki, syaani, radikaali, oranssi, turski, lokaali).
    declension!("abstrakti", 144, "~i";
    stem adjective_nom_pl: "~- like abstrakti";
    stem adjective_pl: "~- like abstrakti";
    nom: "~i", _;
    superlative: derived;
    ),
    // Kuiva: 96 words (musta, ruma, kuuma, kuiva, soma, kova). Like Matala, except in 1 cell: comparative.
    declension!("kuiva", 96, "~a", like Matala;
    comparative: "~empi";
    ),
    // Selvä: 96 words (köyhä, kylmä, väärä, tyhmä, pyhä, syvä). Like Terävä, except in 1 cell: comparative.
    declension!("selvä", 96, "~ä", like Terävä;
    comparative: "~empi";
    ),
    // Violetti: 90 words (violetti, akuutti, ultravioletti, elegantti, ekvivalentti, irrelevantti). Like Abstrakti with `ˇ` after each pattern.
    declension!("violetti", 90, "~iˇ", like Abstrakti with "ˇ";
    ),
    // Räjähtänyt: 85 words (eristäytynyt, kyrpiintynyt, kyllästynyt, alikehittynyt, hävinnyt, ärsyyntynyt). Like Sammunut, except in 2 cells: nom sg, par sg.
    declension!("räjähtänyt", 85, "~yt", like Sammunut;
    nom: "~yt", _;
    par: "~yttA", _;
    ),
    // Alvari: 63 words (versaali, abnormaali, normaali, humaani, globaali, neutraali). Like Abstrakti, except in 2 cells: gen_ pl, par pl.
    declension!("alvari", 63, "~i", like Abstrakti;
    gen_: _, "~ien|~eiden|~eitten";
    par: _, "~eitA|~ejA";
    ),
    // Villi: 62 words (vissi, eri, villi, riski, läski, rigidi). Like Abstrakti, except in 14 cells: par sg, par pl, ine sg, ine pl, ela sg, ela pl, ….
    declension!("villi", 62, "~i", like Abstrakti;
    par: "~iä", "~ejä";
    ine: "~issä", "~eissä";
    ela: "~istä", "~eistä";
    ade: "~illä", "~eillä";
    abl: "~iltä", "~eiltä";
    ess: "~inä", "~einä";
    abe: "~ittä", "~eittä";
    ),
    // Hopeoitu: 58 words (mieto, outo, aito, panssaroitu, sertifioitu, laiminlyöty).
    declension!("hopeoitu", 58, "~t~";
    stem adjective_pl: "~t~- like huono";
    nom: "~t~", "~d~t";
    gen_: "~d~n", _;
    par: "~t~A", "~t~jA";
    ine: "~d~ssA", "~d~issA";
    ela: "~d~stA", "~d~istA";
    ill: "~t~Vn", _;
    ade: "~d~llA", "~d~illA";
    abl: "~d~ltA", "~d~iltA";
    all: "~d~lle", "~d~ille";
    ess: "~t~nA", "~t~inA";
    tra: "~d~ksi", "~d~iksi";
    abe: "~d~ttA", "~d~ittA";
    ins: _, "~d~in";
    comparative: "~d~mpi";
    ),
    // Vahva: 56 words (vanha, vahva, hauska, laiha, paha, ikivanha). Like Kuiva, except in 11 cells: par pl, ine pl, ela pl, ade pl, abl pl, all pl, ….
    declension!("vahva", 56, "~a", like Kuiva;
    stem adjective_pl: "~o- like huono";
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
    // Vapaa: 49 words (vapaa, harmaa, vakaa, epävakaa, vajaa, tummanharmaa). Like Oikea, except in 3 cells: par sg, ill sg, ill pl.
    declension!("vapaa", 49, "~a", like Oikea;
    par: "~atA", _;
    ill: "~aseVn", "~isVin";
    ),
    // Tanakka: 39 words (terhakka, hailakka, navakka, lupsakka, solakka, tanakka).
    declension!("tanakka", 39, "~ka";
    stem adjective_nom_pl: "~- like tanakka";
    nom: "~ka", _;
    gen_: _, "~oiden|~oitten|~kojen";
    ill: _, "~oihin|~koihin";
    com: _, "~koine";
    ),
    // Mykkä: 38 words (märkä, läpimärkä, jyrkkä, jäykkä, synkkä, mykkä). Like Pehmeä, except in 7 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl, ….
    declension!("mykkä", 38, "~kä", like Pehmeä;
    stem adjective_pl: "~ki- like laiton";
    nom: "~kä", _;
    par: "~käA", "~kiä";
    ill: "~käAn", _;
    ess: "~känA", "~kinä";
    comparative: "~empi";
    ),
    // Tiukka: 38 words (tarkka, vaka, arka, raaka, liika, sanatarkka). Like Vahva, except in 6 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl.
    declension!("tiukka", 38, "~ka", like Vahva;
    stem adjective_pl: "~ko- like huono";
    nom: "~ka", _;
    par: "~kaA", "~kojA";
    ill: "~kaAn", _;
    ess: "~kanA", "~koinA";
    ),
    // Valmis: 29 words (kaunis, kallis, aulis, valmis, patavalmis, kullankallis).
    declension!("valmis", 29, "~~s";
    nom: "~~s", "~~it";
    gen_: "~~in", "~~iden";
    par: "~~stA", "~~itA";
    ine: "~~issA", "~~issA";
    ela: "~~istA", "~~istA";
    ill: "~~iseen", "~~isiin";
    ade: "~~illA", "~~illA";
    abl: "~~iltA", "~~iltA";
    all: "~~ille", "~~ille";
    ess: "~~inA", "~~inA";
    tra: "~~iksi", "~~iksi";
    abe: "~~ittA", "~~ittA";
    ins: _, "~~in";
    com: _, "~~ine";
    comparative: "~~impi";
    superlative: "~e~n";
    ),
    // Nätti: 26 words (nätti, ekstrovertti, resistentti, introvertti, definiitti, diskreetti). Like Villi with `ˇ` after each pattern.
    declension!("nätti", 26, "~iˇ", like Villi with "ˇ";
    ),
    // Sähäkkä: 19 words (ärhäkkä, tömäkkä, jämäkkä, räväkkä, sähäkkä, möräkkä).
    declension!("sähäkkä", 19, "~kä";
    stem adjective_nom_pl: "~- like sähäkkä";
    nom: "~kä", _;
    gen_: _, "~öiden|~öitten|~köjen";
    ill: _, "~öihin|~köihin";
    com: _, "~köine";
    ),
    // Kahes: 19 words (viides, yhdeksäs, kymmenes, kuudes, seitsemäs, kahdeksas).
    declension!("kahes", 19, "~s";
    stem adjective_nom_pl: "~- like kahes";
    stem adjective_pl: "~nsi- like laiton";
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
    ill: "~eVn", _;
    ade: "~ellA", _;
    abl: "~eltA", _;
    all: "~elle", _;
    ess: "~enA", _;
    tra: "~eksi", _;
    abe: "~ettA", _;
    ),
    // Tuore: 17 words (terve, chic, perkele, tuore, ahne, kade). Like Oikea, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("tuore", 17, "~", like Oikea;
    nom: "~", "~Vt";
    gen_: "~Vn", _;
    par: "~ttA", _;
    ine: "~VssA", _;
    ela: "~VstA", _;
    ill: "~eseen", _;
    ade: "~VllA", _;
    abl: "~VltA", _;
    all: "~Vlle", _;
    ess: "~VnA", _;
    tra: "~Vksi", _;
    abe: "~VttA", _;
    comparative: "~Vmpi";
    ),
    // Heikko: 16 words (heikko, välkky, kolkko, säikky, tykky, kökkö).
    declension!("heikko", 16, "~k~";
    stem adjective_pl: "~k~- like huono";
    nom: "~k~", "~~t";
    gen_: "~~n", _;
    par: "~k~A", "~k~jA";
    ine: "~~ssA", "~~issA";
    ela: "~~stA", "~~istA";
    ill: "~k~Vn", _;
    ade: "~~llA", "~~illA";
    abl: "~~ltA", "~~iltA";
    all: "~~lle", "~~ille";
    ess: "~k~nA", "~k~inA";
    tra: "~~ksi", "~~iksi";
    abe: "~~ttA", "~~ittA";
    ins: _, "~~in";
    comparative: "~~mpi";
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
    // Voipa: 11 words (halpa, kaikkivoipa, hyvinvoipa, hyvänsuopa, hupa, aikaansaapa).
    declension!("voipa", 11, "~pa";
    stem adjective_nom_pl: "~- like voipa";
    stem adjective_pl: "~pi- like laiton";
    nom: "~pa", _;
    superlative: derived;
    ),
    // Kevyt: 9 words (kevyt, lyhyt, ehyt, pehmyt, höyhenenkevyt, ultrakevyt). Like Suuri, except in 2 cells: nom sg, superlative.
    declension!("kevyt", 9, "~t", like Suuri;
    stem adjective_nom_pl: "~- like kevyt";
    stem adjective_pl: "~i- like oikea";
    nom: "~t", _;
    superlative: derived;
    ),
    // Kavokki: 9 words (pinkki, antiikki, uniikki, opaakki, ubiikki, feikki). Like Abstrakti, except in 6 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl.
    declension!("kavokki", 9, "~ki", like Abstrakti;
    stem adjective_pl: "~k- like abstrakti";
    nom: "~ki", _;
    par: "~kiA", "~kejA";
    ill: "~kiVn", _;
    ess: "~kinA", "~keinA";
    ),
    // Puhdas: 9 words (hidas, puhdas, ahdas, epäpuhdas, putipuhdas, tyylipuhdas). Like Paljas with `ˇ` after each pattern.
    declension!("puhdas", 9, "~sˇ", like Paljas with "ˇ";
    ),
    // Autio: 9 words (pro, tax-free, handsfree, okei, autio, gurmee). Like Huono, except in 2 cells: par sg, par pl.
    declension!("autio", 9, "~", like Huono;
    stem adjective_pl: "~i- like pro";
    par: "~tA", "~itA";
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
    // Tosi: 8 words (uusi, täysi, tosi, epätosi, upouusi, täpötäysi). Like Tasainen, except in 14 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("tosi", 8, "~si", like Tasainen;
    stem adjective_pl: "~si- like laiton";
    nom: "~si", "~det";
    gen_: "~den", _;
    par: "~ttA", _;
    ine: "~dessA", _;
    ela: "~destA", _;
    ill: "~teVn", _;
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
    ine: "~k~essä", "~k~issä";
    ela: "~k~estä", "~k~istä";
    ill: "~k~een", "~k~iin";
    ade: "~k~ellä", "~k~illä";
    abl: "~k~eltä", "~k~iltä";
    all: "~k~elle", "~k~ille";
    ess: "~k~enä", "~k~inä";
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
    // Käypä: 7 words (käypä, järkeenkäypä, päällekäypä, sydämeenkäypä, edelläkäypä, laatuunkäypä). Like Voipa, except in 4 cells: nom sg, par sg, ill sg, ess sg.
    declension!("käypä", 7, "~pä", like Voipa;
    nom: "~pä", _;
    par: "~päA", _;
    ill: "~päAn", _;
    ess: "~pänA", _;
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
    stem adjective_nom_pl: "~- like mätä";
    stem adjective_pl: "~ti- like laiton";
    nom: "~tä", _;
    superlative: derived;
    ),
    // Hoikka: 5 words (hoikka, tasasoukka, soukka, kuukka, erikoishoikka). Like Paljas, except in 7 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl, ….
    declension!("hoikka", 5, "~ka", like Paljas;
    stem adjective_pl: "~ki- like laiton";
    nom: "~ka", _;
    par: "~kaA", "~kiA";
    ill: "~kaAn", _;
    ess: "~kanA", "~kinA";
    comparative: "~empi";
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
    stem adjective_nom_pl: "~u- like kevyt";
    nom: "~ut", _;
    gen_: _, "~uiden";
    ill: _, "~uisVin";
    com: _, "~uine";
    superlative: derived;
    ),
    // Tuhti: 4 words (mahti, suoti, rehti, tuhti).
    declension!("tuhti", 4, "~ti";
    stem adjective_nom_pl: "~- like tuhti";
    stem adjective_pl: "~t- like abstrakti";
    nom: "~ti", _;
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
    stem adjective_nom_pl: "~- like avoin";
    nom: "~n", _;
    gen_: _, "~mVen|~nten";
    ill: _, "~mVin";
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
    // Vihanta: 4 words (ikivihanta, vihanta, ainavihanta, kesävihanta).
    declension!("vihanta", 4, "~ta";
    stem adjective_nom_pl: "~- like vihanta";
    stem adjective_pl: "~t- like vihanta";
    nom: "~ta", _;
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
    // Orpo: 3 words (orpo, epäkelpo, kelpo).
    declension!("orpo", 3, "~p~";
    stem adjective_pl: "~p~- like huono";
    nom: "~p~", "~v~t";
    gen_: "~v~n", _;
    par: "~p~A", "~p~jA";
    ine: "~v~ssa", "~v~issa";
    ela: "~v~sta", "~v~ista";
    ill: "~p~Vn", _;
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
    // Rento: 3 words (rento, hento, kokoonpantu).
    declension!("rento", 3, "~t~";
    stem adjective_pl: "~t~- like huono";
    nom: "~t~", "~n~t";
    gen_: "~n~n", _;
    par: "~t~A", "~t~jA";
    ine: "~n~ssa", "~n~issa";
    ela: "~n~sta", "~n~ista";
    ill: "~t~Vn", _;
    ade: "~n~lla", "~n~illa";
    abl: "~n~lta", "~n~ilta";
    all: "~n~lle", "~n~ille";
    ess: "~t~nA", "~t~inA";
    tra: "~n~ksi", "~n~iksi";
    abe: "~n~tta", "~n~itta";
    ins: _, "~n~in";
    comparative: "~n~mpi";
    ),
    // Alaston: 3 words (alaston, ilkialaston, puolialaston). Like Kummempi, except in 7 cells: nom sg, par sg, par pl, ill sg, ess sg, ess pl, ….
    declension!("alaston", 3, "~n", like Kummempi;
    stem adjective_pl: "~mi- like laiton";
    nom: "~n", _;
    par: "~ntA", "~miA";
    ill: "~mAAn", _;
    ess: "~mAnA", "~minA";
    comparative: "~mAmpi";
    ),
    // Ehta: 3 words (ehta, kaita, saita).
    declension!("ehta", 3, "~ta";
    stem adjective_nom_pl: "~- like ehta";
    stem adjective_pl: "~to- like huono";
    nom: "~ta", _;
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
    stem adjective_nom_pl: "~- like vähäläntä";
    stem adjective_pl: "~ti- like laiton";
    nom: "~tä", _;
    superlative: derived;
    ),
    // Vänkyrä: 3 words (känttyrä, vänkyrä, viheriä). Like Veiterä, except in 10 cells: par pl, ine pl, ela pl, ade pl, abl pl, all pl, ….
    declension!("vänkyrä", 3, "~ä", like Veiterä;
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
];
