//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~a`, `sentiljoona`: the stem and `a`). A stem in two pieces (`~si~`, `viisitoista`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem nom_pl: \"~- like sentiljoona\"`: the slot's cells are `sentiljoona`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
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
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem nom_pl: $s1:tt;)?
     $(stem pl: $s2:tt;)?
     $(stem ill_pl: $s3:tt;)?
     $(stem pl_: $s4:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
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

/// What a declension adds after each pattern of the one it is like (`with "~"`: `~` after every pattern).
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
    /// Like `sentiljoona`: 35 words.
    Sentiljoona,
    /// Like `viisitoista`: 13 words.
    Viisitoista,
    /// Like `yhdeksän`: 3 words.
    Yhdeksän,
    /// Like `kuusi`: 2 words.
    Kuusi,
    /// Like `kybä`: 2 words.
    Kybä,
    /// Like `yhdeksänkymmentä`: 2 words.
    Yhdeksänkymmentä,
    /// Like `miljardi`: 2 words.
    Miljardi,
    /// Like `googol`: 1 words.
    Googol,
    /// Like `kolme`: 1 words.
    Kolme,
    /// Like `viisi`: 1 words.
    Viisi,
    /// Like `kahdeksan`: 1 words.
    Kahdeksan,
    /// Like `kymmenen`: 1 words.
    Kymmenen,
    /// Like `tuhat`: 1 words.
    Tuhat,
    /// Like `kaks`: 1 words.
    Kaks,
    /// Like `kolmekymmentä`: 1 words.
    Kolmekymmentä,
    /// Like `neljäkymmentä`: 1 words.
    Neljäkymmentä,
    /// Like `kahdeksankymmentä`: 1 words.
    Kahdeksankymmentä,
    /// Like `yhdes`: 1 words.
    Yhdes,
    /// Like `parikymmentä`: 1 words.
    Parikymmentä,
    /// Like `puolikymmentä`: 1 words.
    Puolikymmentä,
    /// Like `seitsen`: 1 words.
    Seitsen,
    /// Like `satakymmenen`: 1 words.
    Satakymmenen,
    /// Like `tyhjykkä`: 1 words.
    Tyhjykkä,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 23] = [
        Declension::Sentiljoona,
        Declension::Viisitoista,
        Declension::Yhdeksän,
        Declension::Kuusi,
        Declension::Kybä,
        Declension::Yhdeksänkymmentä,
        Declension::Miljardi,
        Declension::Googol,
        Declension::Kolme,
        Declension::Viisi,
        Declension::Kahdeksan,
        Declension::Kymmenen,
        Declension::Tuhat,
        Declension::Kaks,
        Declension::Kolmekymmentä,
        Declension::Neljäkymmentä,
        Declension::Kahdeksankymmentä,
        Declension::Yhdes,
        Declension::Parikymmentä,
        Declension::Puolikymmentä,
        Declension::Seitsen,
        Declension::Satakymmenen,
        Declension::Tyhjykkä,
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
    // Sentiljoona: 35 words (sata, nolla, kolmetoista, neljätoista, seitsemäntoista, kahdeksantoista).
    declension!("sentiljoona", 35, "~a";
    stem nom_pl: "~- like sentiljoona";
    stem pl: "~i- like sentiljoona";
    stem pl_: "~in- like sentiljoona";
    nom: "~a", _;
    ill: _, "~iin";
    ),
    // Viisitoista: 13 words (yksitoista, kaksitoista, viisitoista, kaksikymmentä, kuusitoista, viisikymmentä). Like Viisi with `~` after each pattern.
    declension!("viisitoista", 13, "~si~", like Viisi with "~";
    ),
    // Yhdeksän: 3 words (seitsemän, yhdeksän, seitsämän).
    declension!("yhdeksän", 3, "~än";
    stem nom_pl: "~- like yhdeksän";
    stem pl: "~i- like yhdeksän";
    stem pl_: "~in- like yhdeksän";
    nom: "~än", _;
    ill: _, "~iin";
    ),
    // Kuusi: 2 words (kuusi, kaksi).
    declension!("kuusi", 2, "~si";
    stem nom_pl: "~- like kaksi";
    stem pl: "~si- like sentiljoona";
    stem pl_: "~sin- like sentiljoona";
    nom: "~si", _;
    ill: _, "~siin";
    ),
    // Kybä: 2 words (neljä, kybä). Like Yhdeksän, except in 1 cell: nom sg.
    declension!("kybä", 2, "~ä", like Yhdeksän;
    nom: "~ä", _;
    ),
    // Yhdeksänkymmentä: 2 words (seitsemänkymmentä, yhdeksänkymmentä).
    declension!("yhdeksänkymmentä", 2, "~än~tä";
    nom: "~än~tä", "~ät~et";
    gen_: "~än~en", "~ien~ien|~ien~ten";
    par: "~ää~tä", "~iä~iä";
    ine: "~ässä~essä", "~issä~issä";
    ela: "~ästä~estä", "~istä~istä";
    ill: "~ään~een", "~iin~iin";
    ade: "~ällä~ellä", "~illä~illä";
    abl: "~ältä~eltä", "~iltä~iltä";
    all: "~älle~elle", "~ille~ille";
    ess: "~änä~enä", "~inä~inä";
    tra: "~äksi~eksi", "~iksi~iksi";
    abe: "~ättä~että", "~ittä~ittä";
    ins: _, "~in~in";
    com: _, "~ine~ine";
    ),
    // Miljardi: 2 words (miljardi, triljardi).
    declension!("miljardi", 2, "~i";
    stem nom_pl: "~- like miljardi";
    stem pl_: "~ein- like sentiljoona";
    nom: "~i", _;
    gen_: _, "~ien";
    par: _, "~eja";
    ill: _, "~eihin";
    ),
    // Googol: 1 words (googol). Like Miljardi, except in 1 cell: nom sg.
    declension!("googol", 1, "~", like Miljardi;
    nom: "~", _;
    ),
    // Kolme: 1 words (kolme). Like Sentiljoona, except in 13 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("kolme", 1, "~e", like Sentiljoona;
    nom: "~e", "~et";
    gen_: "~en", _;
    par: "~ea", _;
    ine: "~essa", _;
    ela: "~esta", _;
    ill: "~een", _;
    ade: "~ella", _;
    abl: "~elta", _;
    all: "~elle", _;
    ess: "~ena", _;
    tra: "~eksi", _;
    abe: "~etta", _;
    ),
    // Viisi: 1 words (viisi). Like Kuusi, except in 12 cells: par sg, ine sg, ine pl, ela sg, ela pl, ade sg, ….
    declension!("viisi", 1, "~si", like Kuusi;
    stem pl: "~si- like yhdeksän";
    stem pl_: "~sin- like yhdeksän";
    par: "~ttä", _;
    ine: "~dessä", "~sissä";
    ela: "~destä", "~sistä";
    ade: "~dellä", "~sillä";
    abl: "~deltä", "~siltä";
    ess: "~tenä", _;
    abe: "~dettä", "~sittä";
    ),
    // Kahdeksan: 1 words (kahdeksan). Like Sentiljoona, except in 1 cell: nom sg.
    declension!("kahdeksan", 1, "~an", like Sentiljoona;
    nom: "~an", _;
    ),
    // Kymmenen: 1 words (kymmenen). Like Yhdeksän, except in 14 cells: nom sg, nom pl, gen_ sg, gen_ pl, par sg, ine sg, ….
    declension!("kymmenen", 1, "~en", like Yhdeksän;
    nom: "~en", "~et";
    gen_: "~en", "~ien|~ten";
    par: "~tä", _;
    ine: "~essä", _;
    ela: "~estä", _;
    ill: "~een", _;
    ade: "~ellä", _;
    abl: "~eltä", _;
    all: "~elle", _;
    ess: "~enä", _;
    tra: "~eksi", _;
    abe: "~että", _;
    ),
    // Tuhat: 1 words (tuhat).
    declension!("tuhat", 1, "~t";
    stem nom_pl: "~- like tuhat";
    stem pl: "~nsi- like sentiljoona";
    stem pl_: "~nsin- like sentiljoona";
    nom: "~t", _;
    ill: _, "~nsiin";
    ),
    // Kaks: 1 words (kaks).
    declension!("kaks", 1, "~ks";
    stem nom_pl: "~- like kaks";
    stem pl: "~ksi- like sentiljoona";
    nom: "~ks", _;
    ill: _, "~ksiin|~ksii";
    ess: _, "~ksina";
    ),
    // Kolmekymmentä: 1 words (kolmekymmentä).
    declension!("kolmekymmentä", 1, "~e~tä";
    nom: "~e~tä", "~et~et";
    gen_: "~en~en", "~ien~ien|~ien~ten";
    par: "~ea~tä", "~ia~iä";
    ine: "~essa~essä", "~issa~issä";
    ela: "~esta~estä", "~ista~istä";
    ill: "~een~een", "~iin~iin";
    ade: "~ella~ellä", "~illa~illä";
    abl: "~elta~eltä", "~ilta~iltä";
    all: "~elle~elle", "~ille~ille";
    ess: "~ena~enä", "~ina~inä";
    tra: "~eksi~eksi", "~iksi~iksi";
    abe: "~etta~että", "~itta~ittä";
    ins: _, "~in~in";
    com: _, "~ine~ine";
    ),
    // Neljäkymmentä: 1 words (neljäkymmentä). Like Yhdeksänkymmentä, except in 1 cell: nom sg.
    declension!("neljäkymmentä", 1, "~ä~tä", like Yhdeksänkymmentä;
    nom: "~ä~tä", _;
    ),
    // Kahdeksankymmentä: 1 words (kahdeksankymmentä). Like Kolmekymmentä, except in 13 cells: nom sg, nom pl, gen_ sg, par sg, ine sg, ela sg, ….
    declension!("kahdeksankymmentä", 1, "~an~tä", like Kolmekymmentä;
    nom: "~an~tä", "~at~et";
    gen_: "~an~en", _;
    par: "~aa~tä", _;
    ine: "~assa~essä", _;
    ela: "~asta~estä", _;
    ill: "~aan~een", _;
    ade: "~alla~ellä", _;
    abl: "~alta~eltä", _;
    all: "~alle~elle", _;
    ess: "~ana~enä", _;
    tra: "~aksi~eksi", _;
    abe: "~atta~että", _;
    ),
    // Yhdes: 1 words (yhdes).
    declension!("yhdes", 1, "~s";
    stem nom_pl: "~- like yhdes";
    stem pl: "~nsi- like yhdeksän";
    stem pl_: "~nsin- like yhdeksän";
    nom: "~s", _;
    ill: _, "~nsiin";
    ),
    // Parikymmentä: 1 words (parikymmentä).
    declension!("parikymmentä", 1, "~i~tä";
    nom: "~i~tä", "~it~et";
    gen_: "~in~en", "~ien~ien|~ien~ten";
    par: "~ia~tä", "~eja~iä";
    ine: "~issa~essä", "~eissa~issä";
    ela: "~ista~estä", "~eista~istä";
    ill: "~iin~een", "~eihin~iin";
    ade: "~illa~ellä", "~eilla~illä";
    abl: "~ilta~eltä", "~eilta~iltä";
    all: "~ille~elle", "~eille~ille";
    ess: "~ina~enä", "~eina~inä";
    tra: "~iksi~eksi", "~eiksi~iksi";
    abe: "~itta~että", "~eitta~ittä";
    ins: _, "~ein~in";
    com: _, "~eine~ine";
    ),
    // Puolikymmentä: 1 words (puolikymmentä). Like Kolmekymmentä, except in 3 cells: nom sg, gen_ pl, par sg.
    declension!("puolikymmentä", 1, "~i~tä", like Kolmekymmentä;
    nom: "~i~tä", _;
    gen_: _, "~ten~ien|~ten~ten|~ien~ien|~ien~ten";
    par: "~ta~tä", _;
    ),
    // Seitsen: 1 words (seitsen).
    declension!("seitsen", 1, "~n";
    stem nom_pl: "~- like seitsen";
    stem pl_: "~min- like yhdeksän";
    nom: "~n", _;
    gen_: _, "~mien|~mäin";
    par: _, "~miä";
    ill: _, "~miin";
    ),
    // Satakymmenen: 1 words (satakymmenen).
    declension!("satakymmenen", 1, "~ta~en";
    nom: "~ta~en", "~dat~et";
    gen_: "~dan~en", "~tojen~ien|~tojen~ten";
    par: "~taa~tä", "~toja~iä";
    ine: "sadas~~essä", "sadois~~issä";
    ela: "~dasta~estä", "~doista~istä";
    ill: "~taan~een", "~toihin~iin";
    ade: "~dalla~ellä", "~doilla~illä";
    abl: "~dalta~eltä", "~doilta~iltä";
    all: "~dalle~elle", "~doille~ille";
    ess: "~tana~enä", "~toina~inä";
    tra: "~daksi~eksi", "~doiksi~iksi";
    abe: "~datta~että", "~doitta~ittä";
    ins: _, "~doin~in";
    com: _, "~toine~ine";
    ),
    // Tyhjykkä: 1 words (tyhjykkä).
    declension!("tyhjykkä", 1, "~kä";
    stem nom_pl: "~- like tyhjykkä";
    stem pl_: "~köin- like yhdeksän";
    nom: "~kä", _;
    gen_: _, "~öiden|~öitten|~köjen";
    par: _, "~öitä|~köjä";
    ill: _, "~öihin|~köihin";
    ),
];
