//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~a`, `dąbrowa`: the stem and `a`). A stem in two pieces (`~ó~`, `aleksandrów`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `ˇ` ends
//! the patterns of a declension whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ˇ`: `melchizedek`).

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the declension it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~- like zachariasz\"`: the slot's cells are `zachariasz`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the declension's it is
/// like, or, in a declension like no other, derived):
/// `nom` (sg, pl)
/// `gen_` (sg, pl)
/// `dat` (sg, pl)
/// `acc` (sg, pl)
/// `ins` (sg, pl)
/// `loc` (sg, pl)
/// `voc` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, [$($f:ident: $v:ident),*])? $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem pl: $s1:tt;)?
     $(stem gen_sg: $s2:tt;)?
     $(nom: $c0:tt, $c1:tt;)?
     $(gen_: $c2:tt, $c3:tt;)?
     $(dat: $c4:tt, $c5:tt;)?
     $(acc: $c6:tt, $c7:tt;)?
     $(ins: $c8:tt, $c9:tt;)?
     $(loc: $c10:tt, $c11:tt;)?
     $(voc: $c12:tt, $c13:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[$($(feature!($f: $v)),*)?]),
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

/// An inherent feature's value, as `(feature, value)` by their places.
macro_rules! feature {
    (gender: $v:ident) => {
        (0, crate::Gender::$v as u8)
    };
}

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`acc_as_gen`: one value read as another).
const ACC_AS_GEN: [u16; 14] = [0, 1, 2, 3, 4, 5, 2, 3, 8, 9, 10, 11, 12, 13];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `zachariasz`: 6,621 words.
    Zachariasz,
    /// Like `dąbrowa`: 1,657 words.
    Dąbrowa,
    /// Like `zełenski`: 1,210 words.
    Zełenski,
    /// Like `melchizedek`: 508 words.
    Melchizedek,
    /// Like `grabowiecka`: 488 words.
    Grabowiecka,
    /// Like `pegaz`: 466 words.
    Pegaz,
    /// Like `goń`: 439 words.
    Goń,
    /// Like `wituś`: 339 words.
    Wituś,
    /// Like `filadelfia`: 321 words.
    Filadelfia,
    /// Like `żubr`: 296 words.
    Żubr,
    /// Like `birmingham`: 295 words.
    Birmingham,
    /// Like `wit`: 290 words.
    Wit,
    /// Like `lipowiec`: 203 words.
    Lipowiec,
    /// Like `polana`: 183 words.
    Polana,
    /// Like `boniec`: 153 words.
    Boniec,
    /// Like `juda`: 136 words.
    Juda,
    /// Like `grodzisk`: 124 words.
    Grodzisk,
    /// Like `góra`: 120 words.
    Góra,
    /// Like `aleksandrów`: 120 words.
    Aleksandrów,
    /// Like `natolin`: 92 words.
    Natolin,
    /// Like `bać`: 86 words.
    Bać,
    /// Like `bagno`: 68 words.
    Bagno,
    /// Like `rak`: 66 words.
    Rak,
    /// Like `lipiny`: 61 words.
    Lipiny,
    /// Like `racławice`: 60 words.
    Racławice,
    /// Like `buk`: 55 words.
    Buk,
    /// Like `liverpool`: 52 words.
    Liverpool,
    /// Like `jabłonka`: 51 words.
    Jabłonka,
    /// Like `horacy`: 44 words.
    Horacy,
    /// Like `idec`: 41 words.
    Idec,
    /// Like `głów`: 38 words.
    Głów,
    /// Like `depo`: 38 words.
    Depo,
    /// Like `róg`: 36 words.
    Róg,
    /// Like `guzy`: 36 words.
    Guzy,
    /// Like `łabędź`: 35 words.
    Łabędź,
    /// Like `września`: 33 words.
    Września,
    /// Like `mól`: 33 words.
    Mól,
    /// Like `tatary`: 32 words.
    Tatary,
    /// Like `oleśnica`: 32 words.
    Oleśnica,
    /// Like `dąbie`: 30 words.
    Dąbie,
    /// Like `kamień`: 30 words.
    Kamień,
    /// Like `piór`: 28 words.
    Piór,
    /// Like `kryszna`: 27 words.
    Kryszna,
    /// Like `drost`: 26 words.
    Drost,
    /// Like `magda`: 26 words.
    Magda,
    /// Like `kościuszko`: 23 words.
    Kościuszko,
    /// Like `bolko`: 23 words.
    Bolko,
    /// Like `biskaja`: 22 words.
    Biskaja,
    /// Like `tosia`: 22 words.
    Tosia,
    /// Like `grodzisko`: 19 words.
    Grodzisko,
    /// Like `leja`: 19 words.
    Leja,
    /// Like `stoczek`: 19 words.
    Stoczek,
    /// Like `annopol`: 18 words.
    Annopol,
    /// Like `świętokrzyskie`: 18 words.
    Świętokrzyskie,
    /// Like `wąż`: 17 words.
    Wąż,
    /// Like `pcim`: 16 words.
    Pcim,
    /// Like `anula`: 15 words.
    Anula,
    /// Like `wsół`: 15 words.
    Wsół,
    /// Like `wesoła`: 15 words.
    Wesoła,
    /// Like `malec`: 15 words.
    Malec,
    /// Like `wola`: 14 words.
    Wola,
    /// Like `ostrowiec`: 14 words.
    Ostrowiec,
    /// Like `naaman`: 13 words.
    Naaman,
    /// Like `obed`: 11 words.
    Obed,
    /// Like `powązki`: 11 words.
    Powązki,
    /// Like `głód`: 11 words.
    Głód,
    /// Like `noe`: 10 words.
    Noe,
    /// Like `apacz`: 10 words.
    Apacz,
    /// Like `wyszogród`: 10 words.
    Wyszogród,
    /// Like `tola`: 10 words.
    Tola,
    /// Like `knefel`: 10 words.
    Knefel,
    /// Like `piaseczno`: 10 words.
    Piaseczno,
    /// Like `lewi`: 10 words.
    Lewi,
    /// Like `liberec`: 9 words.
    Liberec,
    /// Like `ania`: 9 words.
    Ania,
    /// Like `bugiel`: 8 words.
    Bugiel,
    /// Like `pompeje`: 7 words.
    Pompeje,
    /// Like `stasiek`: 7 words.
    Stasiek,
    /// Like `cpn`: 7 words.
    Cpn,
    /// Like `koziec`: 7 words.
    Koziec,
    /// Like `zwierzyniec`: 7 words.
    Zwierzyniec,
    /// Like `aśka`: 7 words.
    Aśka,
    /// Like `błonie`: 7 words.
    Błonie,
    /// Like `japet`: 7 words.
    Japet,
    /// Like `szerlok`: 6 words.
    Szerlok,
    /// Like `luter`: 6 words.
    Luter,
    /// Like `triest`: 6 words.
    Triest,
    /// Like `grójec`: 6 words.
    Grójec,
    /// Like `bydgoszcz`: 6 words.
    Bydgoszcz,
    /// Like `śnieżka`: 5 words.
    Śnieżka,
    /// Like `rudy`: 5 words.
    Rudy,
    /// Like `myślibórz`: 5 words.
    Myślibórz,
    /// Like `ali`: 5 words.
    Ali,
    /// Like `indie`: 5 words.
    Indie,
    /// Like `tadeusz`: 5 words.
    Tadeusz,
    /// Like `stambuł`: 5 words.
    Stambuł,
    /// Like `woźniki`: 5 words.
    Woźniki,
    /// Like `biniek`: 5 words.
    Biniek,
    /// Like `stary`: 5 words.
    Stary,
    /// Like `wywrót`: 5 words.
    Wywrót,
    /// Like `jacuś`: 5 words.
    Jacuś,
    /// Like `kokot`: 5 words.
    Kokot,
    /// Like `gąsiorów`: 5 words.
    Gąsiorów,
    /// Like `fiszer`: 4 words.
    Fiszer,
    /// Like `prypeć`: 4 words.
    Prypeć,
    /// Like `józia`: 4 words.
    Józia,
    /// Like `witek`: 4 words.
    Witek,
    /// Like `nykiel`: 4 words.
    Nykiel,
    /// Like `kieś`: 4 words.
    Kieś,
    /// Like `borki`: 4 words.
    Borki,
    /// Like `szaweł`: 4 words.
    Szaweł,
    /// Like `drozd`: 4 words.
    Drozd,
    /// Like `borek`: 4 words.
    Borek,
    /// Like `becia`: 4 words.
    Becia,
    /// Like `estera`: 3 words.
    Estera,
    /// Like `maryja`: 3 words.
    Maryja,
    /// Like `landau`: 3 words.
    Landau,
    /// Like `iwo`: 3 words.
    Iwo,
    /// Like `węgry`: 3 words.
    Węgry,
    /// Like `ariel`: 3 words.
    Ariel,
    /// Like `idzi`: 3 words.
    Idzi,
    /// Like `puchacze`: 3 words.
    Puchacze,
    /// Like `bysiec`: 3 words.
    Bysiec,
    /// Like `jawor`: 3 words.
    Jawor,
    /// Like `lipnica`: 3 words.
    Lipnica,
    /// Like `azor`: 3 words.
    Azor,
    /// Like `wlazło`: 3 words.
    Wlazło,
    /// Like `jagiełło`: 3 words.
    Jagiełło,
    /// Like `kiec`: 3 words.
    Kiec,
    /// Like `prl`: 3 words.
    Prl,
    /// Like `natalli`: 3 words.
    Natalli,
    /// Like `dauha`: 3 words.
    Dauha,
    /// Like `cło`: 3 words.
    Cło,
    /// Like `iruś`: 3 words.
    Iruś,
    /// Like `bbwr`: 3 words.
    Bbwr,
    /// Like `guzda`: 3 words.
    Guzda,
    /// Like `świder`: 3 words.
    Świder,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 137] = [
        Declension::Zachariasz,
        Declension::Dąbrowa,
        Declension::Zełenski,
        Declension::Melchizedek,
        Declension::Grabowiecka,
        Declension::Pegaz,
        Declension::Goń,
        Declension::Wituś,
        Declension::Filadelfia,
        Declension::Żubr,
        Declension::Birmingham,
        Declension::Wit,
        Declension::Lipowiec,
        Declension::Polana,
        Declension::Boniec,
        Declension::Juda,
        Declension::Grodzisk,
        Declension::Góra,
        Declension::Aleksandrów,
        Declension::Natolin,
        Declension::Bać,
        Declension::Bagno,
        Declension::Rak,
        Declension::Lipiny,
        Declension::Racławice,
        Declension::Buk,
        Declension::Liverpool,
        Declension::Jabłonka,
        Declension::Horacy,
        Declension::Idec,
        Declension::Głów,
        Declension::Depo,
        Declension::Róg,
        Declension::Guzy,
        Declension::Łabędź,
        Declension::Września,
        Declension::Mól,
        Declension::Tatary,
        Declension::Oleśnica,
        Declension::Dąbie,
        Declension::Kamień,
        Declension::Piór,
        Declension::Kryszna,
        Declension::Drost,
        Declension::Magda,
        Declension::Kościuszko,
        Declension::Bolko,
        Declension::Biskaja,
        Declension::Tosia,
        Declension::Grodzisko,
        Declension::Leja,
        Declension::Stoczek,
        Declension::Annopol,
        Declension::Świętokrzyskie,
        Declension::Wąż,
        Declension::Pcim,
        Declension::Anula,
        Declension::Wsół,
        Declension::Wesoła,
        Declension::Malec,
        Declension::Wola,
        Declension::Ostrowiec,
        Declension::Naaman,
        Declension::Obed,
        Declension::Powązki,
        Declension::Głód,
        Declension::Noe,
        Declension::Apacz,
        Declension::Wyszogród,
        Declension::Tola,
        Declension::Knefel,
        Declension::Piaseczno,
        Declension::Lewi,
        Declension::Liberec,
        Declension::Ania,
        Declension::Bugiel,
        Declension::Pompeje,
        Declension::Stasiek,
        Declension::Cpn,
        Declension::Koziec,
        Declension::Zwierzyniec,
        Declension::Aśka,
        Declension::Błonie,
        Declension::Japet,
        Declension::Szerlok,
        Declension::Luter,
        Declension::Triest,
        Declension::Grójec,
        Declension::Bydgoszcz,
        Declension::Śnieżka,
        Declension::Rudy,
        Declension::Myślibórz,
        Declension::Ali,
        Declension::Indie,
        Declension::Tadeusz,
        Declension::Stambuł,
        Declension::Woźniki,
        Declension::Biniek,
        Declension::Stary,
        Declension::Wywrót,
        Declension::Jacuś,
        Declension::Kokot,
        Declension::Gąsiorów,
        Declension::Fiszer,
        Declension::Prypeć,
        Declension::Józia,
        Declension::Witek,
        Declension::Nykiel,
        Declension::Kieś,
        Declension::Borki,
        Declension::Szaweł,
        Declension::Drozd,
        Declension::Borek,
        Declension::Becia,
        Declension::Estera,
        Declension::Maryja,
        Declension::Landau,
        Declension::Iwo,
        Declension::Węgry,
        Declension::Ariel,
        Declension::Idzi,
        Declension::Puchacze,
        Declension::Bysiec,
        Declension::Jawor,
        Declension::Lipnica,
        Declension::Azor,
        Declension::Wlazło,
        Declension::Jagiełło,
        Declension::Kiec,
        Declension::Prl,
        Declension::Natalli,
        Declension::Dauha,
        Declension::Cło,
        Declension::Iruś,
        Declension::Bbwr,
        Declension::Guzda,
        Declension::Świder,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 137] = [
    // Zachariasz: 6,621 words (achilles, czech, daniel, allah, zeus, adonis).
    declension!("zachariasz", 6621, "~", [gender: MasculinePersonal];
    stem lemma: "~- like zachariasz";
    stem gen_sg: "~- like zachariasz";
    nom: _, "~owie";
    ),
    // Dąbrowa: 1,657 words (angola, botswana, ghana, malta, panama, rwanda). Like Zachariasz.
    declension!("dąbrowa", 1657, "~a", like Zachariasz;
    stem lemma: "~- like dąbrowa";
    stem gen_sg: "~- like dąbrowa";
    ),
    // Zełenski: 1,210 words (wierzbowski, kowalski, zamojski, owczarkowski, wiśniewski, dąbrowski).
    declension!("zełenski", 1210, "~ki", [gender: MasculinePersonal];
    stem gen_sg: "~ki- like zełenski";
    nom: "~ki", "~cy";
    acc: "~kiego", _;
    ),
    // Melchizedek: 508 words (melchizedek, marek, janek, radek, strzelec, jacek). Like Zachariasz with `ˇ` after each pattern.
    declension!("melchizedek", 508, "~ˇ", [gender: MasculinePersonal], like Zachariasz with "ˇ";
    ),
    // Grabowiecka: 488 words (kowalska, wiśniewska, racka, rzeczpospolita, wróblewska, szymańska).
    declension!("grabowiecka", 488, "~a", [gender: Feminine];
    nom: "~a", "~ie";
    gen_: "~iej", "~ich";
    dat: "~iej", "~im";
    acc: "~ą", "~ie";
    ins: "~ą", "~imi";
    loc: "~iej", "~ich";
    voc: "~a", _;
    ),
    // Pegaz: 466 words (fabian, edward, darwin, leon, milan, jan). Like Zachariasz.
    declension!("pegaz", 466, "~", [gender: MasculinePersonal], like Zachariasz;
    stem gen_sg: "~- like pegaz";
    ),
    // Goń: 439 words (gwiazdoń, dereń, jeleń, cichoń, szerszeń, korzeń).
    declension!("goń", 439, "~ń", [gender: MasculinePersonal];
    stem gen_sg: "~ni- like zachariasz";
    nom: "~ń", "~niowie";
    acc: "~nia", _;
    ),
    // Wituś: 339 words (łoś, adaś, kubuś, grześ, jaś, krzyś).
    declension!("wituś", 339, "~ś", [gender: MasculinePersonal];
    stem gen_sg: "~si- like zachariasz";
    nom: "~ś", "~siowie";
    acc: "~sia", _;
    ),
    // Filadelfia: 321 words (estonia, austria, albania, armenia, gambia, liberia). Like Dąbrowa, except in 6 cells: nom pl, gen_ sg, gen_ pl, dat sg, acc pl, loc sg.
    declension!("filadelfia", 321, "~a", [gender: Feminine], like Dąbrowa;
    nom: _, "~e";
    gen_: "~i", "~i";
    dat: "~i", _;
    acc: _, "~e";
    loc: "~i", _;
    ),
    // Żubr: 296 words (jupiter, hitler, bor, car, minotaur, singer). Like Zachariasz, except in 2 cells: loc sg, voc sg.
    declension!("żubr", 296, "~", [gender: MasculinePersonal], like Zachariasz;
    loc: "~ze", _;
    voc: "~ze", _;
    ),
    // Birmingham: 295 words (bhutan, gabon, honduras, iran, laos, liechtenstein). Like Pegaz, except in 2 cells: nom pl, acc sg.
    declension!("birmingham", 295, "~", [gender: MasculineInanimate], like Pegaz;
    stem gen_sg: "~- like birmingham";
    nom: _, "~y";
    acc: "~", _;
    ),
    // Wit: 290 words (albert, robert, herbert, gnat, gut, lot).
    declension!("wit", 290, "~t", [gender: MasculinePersonal];
    stem lemma: "~t- like zachariasz";
    stem gen_sg: "~- like wit";
    nom: _, "~towie";
    ),
    // Lipowiec: 203 words (niemiec, krawiec, kupiec, samiec, mrowiec, borowiec).
    declension!("lipowiec", 203, "~iec", [gender: MasculinePersonal];
    stem lemma: "~- like lipowiec";
    stem gen_sg: "~c- like lipowiec";
    nom: _, "~cowie";
    ),
    // Polana: 183 words (grenada, alberta, laura, valletta, teresa, liliana). Like Dąbrowa, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("polana", 183, "~a", [gender: Feminine], like Dąbrowa;
    nom: _, "~y";
    gen_: _, "~";
    acc: _, "~y";
    ),
    // Boniec: 153 words (chowaniec, marciniec, szymaniec, wawrzyniec, szafraniec, boniec).
    declension!("boniec", 153, "~niec", [gender: MasculinePersonal];
    stem gen_sg: "~ńc- like lipowiec";
    nom: "~niec", "~ńcowie";
    acc: "~ńca", _;
    ),
    // Juda: 136 words (lusaka, polska, alaska, nebraska, dhaka, osaka). Like Dąbrowa, except in 2 cells: dat sg, loc sg.
    declension!("juda", 136, "~a", like Dąbrowa;
    dat: "~zie", _;
    loc: "~zie", _;
    ),
    // Grodzisk: 124 words (johannesburg, hamburg, gdańsk, strasburg, wikisłownik, paryż). Like Zachariasz, except in 4 cells: nom pl, acc sg, acc pl, ins sg.
    declension!("grodzisk", 124, "~", [gender: MasculineInanimate], like Zachariasz;
    nom: _, "~i";
    acc: "~", "~i";
    ins: "~iem", _;
    ),
    // Góra: 120 words (spica, podgorica, kambodża, legnica, wołga, norymberga). Like Dąbrowa, except in 2 cells: dat sg, loc sg.
    declension!("góra", 120, "~a", like Dąbrowa;
    dat: "~ze", _;
    loc: "~ze", _;
    ),
    // Aleksandrów: 120 words (kraków, kijów, kościół, rzeszów, tarnów, chorzów).
    declension!("aleksandrów", 120, "~ó~", [gender: MasculineInanimate];
    stem lemma: "~ó~- like birmingham";
    stem gen_sg: "~o~- like aleksandrów";
    nom: _, "~o~y";
    ),
    // Natolin: 92 words (berlin, dublin, tallinn, ursus, będzin, szczecin). Like Birmingham.
    declension!("natolin", 92, "~", [gender: MasculineInanimate], like Birmingham;
    stem gen_sg: "~- like aleksandrów";
    ),
    // Bać: 86 words (zamość, kopeć, kuć, steć, pyć, nogieć).
    declension!("bać", 86, "~ć", [gender: MasculinePersonal];
    stem gen_sg: "~ci- like zachariasz";
    nom: "~ć", "~ciowie";
    acc: "~cia", _;
    ),
    // Bagno: 68 words (brno, gestapo, kosowo, wilno, berno, sarajewo). Like Annopol with `` after each pattern (`o` after ~), except in 3 cells: dat sg, loc sg, voc sg.
    declension!("bagno", 68, "~o", [gender: Neuter], like Annopol with "", "o" after "~";
    dat: "~u", _;
    loc: "~ie", _;
    voc: "~o", _;
    ),
    // Rak: 66 words (balzac, oleg, habsburg, delong, śnieg, maciąg). Like Zachariasz.
    declension!("rak", 66, "~", [gender: MasculinePersonal], like Zachariasz;
    stem gen_sg: "~- like rak";
    ),
    // Lipiny: 61 words (włochy, czechy, chiny, alpy, tatry, karpaty). Like Polana, except in 7 cells: nom sg, gen_ sg, dat sg, acc sg, ins sg, loc sg, ….
    declension!("lipiny", 61, "~y", like Polana;
    nom: "~y", _;
    gen_: derived, _;
    dat: derived, _;
    acc: derived, _;
    ins: derived, _;
    loc: derived, _;
    voc: derived, _;
    ),
    // Racławice: 60 words (katowice, kielce, gliwice, łużyce, police, pardubice). Like Lipiny, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("racławice", 60, "~e", like Lipiny;
    nom: "~e", "~e";
    acc: _, "~e";
    ),
    // Buk: 55 words (quebec, bangkok, salzburg, reykjavik, reykjavík, irak). Like Grodzisk, except in 6 cells: nom pl, gen_ pl, dat pl, acc pl, ins pl, loc pl.
    declension!("buk", 55, "~", [gender: MasculineInanimate], like Grodzisk;
    stem gen_sg: "~- like buk";
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: _, derived;
    ins: _, derived;
    loc: _, derived;
    ),
    // Liverpool: 52 words (nepal, senegal, montreal, kabul, liverpool, tanach). Like Zachariasz, except in 5 cells: nom pl, gen_ sg, gen_ pl, acc sg, acc pl.
    declension!("liverpool", 52, "~", [gender: MasculineInanimate], like Zachariasz;
    nom: _, "~e";
    gen_: "~u", "~i";
    acc: "~", "~e";
    ),
    // Jabłonka: 51 words (blanka, danka, bianka, anka, agnieszka, franciszka).
    declension!("jabłonka", 51, "~ka", [gender: Feminine];
    stem lemma: "~k- like dąbrowa";
    nom: _, "~ki";
    gen_: "~ki", "~ek";
    dat: "~ce", "~kom";
    acc: _, "~ki";
    ins: "~ką", "~kami";
    loc: "~ce", "~kach";
    voc: "~ko", _;
    ),
    // Horacy: 44 words (hilary, merkury, jerzy, grozny, ksawery, maurycy).
    declension!("horacy", 44, "~y", [gender: MasculinePersonal];
    nom: "~y", "~owie";
    gen_: "~ego", "~ych";
    dat: "~emu", "~ym";
    acc: "~ego", "~ych";
    ins: "~ym", "~ymi";
    loc: "~ym", "~ych";
    voc: "~y", _;
    ),
    // Idec: 41 words (vogel, nobel, riedel, hegel, fogel, wróbel).
    declension!("idec", 41, "~e~", [gender: MasculinePersonal];
    stem gen_sg: "~~- like zachariasz";
    nom: "~e~", "~~owie";
    acc: "~~a", _;
    ),
    // Głów: 38 words (mróz, mosór, szymków, zamróz, tomków, głów). Like Aleksandrów, except in 1 cell: nom pl.
    declension!("głów", 38, "~ó~", [gender: MasculinePersonal], like Aleksandrów as ACC_AS_GEN;
    nom: _, "~o~owie";
    ),
    // Depo: 38 words (ziobro, nabagło, namysło, czuryło, wojdyło, wojtyło). Like Dąbrowa.
    declension!("depo", 38, "~o", [gender: MasculinePersonal], like Dąbrowa;
    stem lemma: "~- like szczerbo";
    ),
    // Róg: 36 words (bóg, modlibóg, koziróg, jamróg, barłóg, zbróg). Like Głów.
    declension!("róg", 36, "~ó~", [gender: MasculinePersonal], like Głów;
    stem gen_sg: "~o~- like rak";
    ),
    // Guzy: 36 words (rudy, konieczny, chudy, zimny, tłusty, kościelny). Like Horacy, except in 1 cell: nom pl.
    declension!("guzy", 36, "~y", [gender: MasculinePersonal], like Horacy;
    nom: _, "~i";
    ),
    // Łabędź: 35 words (łabędź, niedźwiedź, żołądź, wlaź, śliź, paź).
    declension!("łabędź", 35, "~ź", [gender: MasculinePersonal];
    stem gen_sg: "~zi- like zachariasz";
    nom: "~ź", "~ziowie";
    acc: "~zia", _;
    ),
    // Września: 33 words (praia, ziemia, gdynia, bośnia, bogatynia, rędzia). Like Dąbrowa, except in 3 cells: gen_ sg, dat sg, loc sg.
    declension!("września", 33, "~a", like Dąbrowa;
    gen_: "~", _;
    dat: "~", _;
    loc: "~", _;
    ),
    // Mól: 33 words (soból, zagój, zagól, kozdrój, niestrój, rogóż). Like Róg.
    declension!("mól", 33, "~ó~", [gender: MasculinePersonal], like Róg;
    stem gen_sg: "~o~- like zachariasz";
    ),
    // Tatary: 32 words (andy, bahamy, falklandy, sudety, malediwy, tychy). Like Lipiny, except in 1 cell: gen_ pl.
    declension!("tatary", 32, "~y", [gender: MasculineInanimate], like Lipiny;
    gen_: _, "~ów";
    ),
    // Oleśnica: 32 words (kulesza, malesza, kieca, małysza, rząca, kościesza). Like Dąbrowa, except in 2 cells: dat sg, loc sg.
    declension!("oleśnica", 32, "~a", like Dąbrowa;
    dat: "~y", _;
    loc: "~y", _;
    ),
    // Dąbie: 30 words (słońce, naddniestrze, zabrze, opole, skopie, mazowsze). Like Annopol with `` after each pattern (`e` after ~), except in 2 cells: dat sg, voc sg.
    declension!("dąbie", 30, "~e", [gender: Neuter], like Annopol with "", "e" after "~";
    dat: "~u", _;
    voc: "~e", _;
    ),
    // Kamień: 30 words (poznań, erywań, toruń, kazań, kamień, zwoleń). Like Goń, except in 7 cells: nom pl, gen_ pl, dat pl, acc sg, acc pl, ins pl, ….
    declension!("kamień", 30, "~ń", [gender: MasculineInanimate], like Goń;
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: "~ń", derived;
    ins: _, derived;
    loc: _, derived;
    ),
    // Piór: 28 words (szczypiór, stompór, stąpór, topór, wiór, pagór). Like Głów, except in 2 cells: loc sg, voc sg.
    declension!("piór", 28, "~ó~", [gender: MasculinePersonal], like Głów;
    loc: "~o~ze", _;
    voc: "~o~ze", _;
    ),
    // Kryszna: 27 words (westa, basta, starosta, kapusta, chrapusta, bembnista). Like Dąbrowa with `ˇ` after each pattern.
    declension!("kryszna", 27, "~aˇ", like Dąbrowa with "ˇ";
    ),
    // Drost: 26 words (ernest, august, drost, chwast, świst, miensopust).
    declension!("drost", 26, "~st", [gender: MasculinePersonal];
    stem lemma: "~st- like zachariasz";
    nom: _, "~stowie";
    gen_: "~sta", "~stów";
    dat: "~stowi", "~stom";
    acc: _, "~stów";
    ins: "~stem", "~stami";
    loc: "~ście", "~stach";
    voc: "~ście", _;
    ),
    // Magda: 26 words (rebeka, monika, marika, amazonka, angelika, dominika). Like Polana, except in 2 cells: dat sg, loc sg.
    declension!("magda", 26, "~a", [gender: Feminine], like Polana;
    dat: "~zie", _;
    loc: "~zie", _;
    ),
    // Kościuszko: 23 words (legutko, adaśko, szyszko, żelazko, kołodko, pietraszko). Like Jabłonka, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("kościuszko", 23, "~ko", [gender: MasculinePersonal], like Jabłonka;
    stem lemma: "~k- like szczerbo";
    nom: _, "~kowie";
    gen_: _, "~ków";
    acc: _, "~ków";
    ),
    // Bolko: 23 words (romeo, apollo, janko, mieszko, łemko, stanko). Like Zachariasz with `` after each pattern (`o` after ~).
    declension!("bolko", 23, "~o", [gender: MasculinePersonal], like Zachariasz with "", "o" after "~";
    ),
    // Biskaja: 22 words (soja, troja, gaja, czaja, żmija, ziaja).
    declension!("biskaja", 22, "~ja";
    stem lemma: "~j- like dąbrowa";
    nom: _, "~jowie";
    gen_: "~i", "~jów";
    dat: "~i", "~jom";
    acc: _, "~jów";
    ins: "~ją", "~jami";
    loc: "~i", "~jach";
    voc: "~jo", _;
    ),
    // Tosia: 22 words (asia, basia, agusia, gosia, małgosia, joasia).
    declension!("tosia", 22, "~sia", [gender: Feminine];
    stem lemma: "~si- like dąbrowa";
    nom: _, "~sie";
    gen_: "~si", "~ś";
    dat: "~si", "~siom";
    acc: _, "~sie";
    ins: "~sią", "~siami";
    loc: "~si", "~siach";
    voc: "~siu", _;
    ),
    // Grodzisko: 19 words (kongo, maroko, portoryko, lipsko, bielsko, giżycko). Like Buk with `` after each pattern (`o` after ~), except in 3 cells: gen_ sg, dat sg, voc sg.
    declension!("grodzisko", 19, "~o", [gender: Neuter], like Buk with "", "o" after "~";
    gen_: "~a", _;
    dat: "~u", _;
    voc: "~o", _;
    ),
    // Leja: 19 words (leja, deja, śmieja, smieja, bareja, zachwieja). Like Września with `ˇ` after each pattern (`iˇ` after ~).
    declension!("leja", 19, "~aˇ", [gender: MasculinePersonal], like Września with "ˇ", "iˇ" after "~";
    ),
    // Stoczek: 19 words (turek, włocławek, targówek, zbyszek, lądek, ozimek). Like Grodzisk with `ˇ` after each pattern.
    declension!("stoczek", 19, "~ˇ", like Grodzisk with "ˇ";
    ),
    // Annopol: 18 words (izrael, kreml, konstantynopol, wałbrzych, czarnobyl, graal). Like Buk.
    declension!("annopol", 18, "~", [gender: MasculineInanimate], like Buk;
    stem gen_sg: "~- like kamień";
    ),
    // Świętokrzyskie: 18 words (stare, komory, zakopane, świętokrzyskie, równe, ateneum). Like Buk.
    declension!("świętokrzyskie", 18, "~", like Buk;
    stem gen_sg: "~- like świętokrzyskie";
    ),
    // Wąż: 17 words (gabriel, anatol, wikłacz, babicz, motyl, wąż). Like Zachariasz, except in 2 cells: gen_ pl, acc pl.
    declension!("wąż", 17, "~", [gender: MasculinePersonal], like Zachariasz;
    gen_: _, "~y";
    acc: _, "~y";
    ),
    // Pcim: 16 words (wrocław, zagrzeb, radom, bytom, oświęcim, jarząb). Like Buk.
    declension!("pcim", 16, "~", like Buk;
    stem gen_sg: "~i- like kamień";
    ),
    // Anula: 15 words (daniela, pamela, gabriela, nikola, kamila, stela). Like Filadelfia, except in 1 cell: gen_ pl.
    declension!("anula", 15, "~a", [gender: Feminine], like Filadelfia;
    gen_: _, "~";
    ),
    // Wsół: 15 words (sokół, popiół, bawół, suchodół, sudół, gocół).
    declension!("wsół", 15, "~ół";
    stem gen_sg: "~o- like wsół";
    nom: "~ół", "~ołowie";
    acc: "~oła", _;
    ),
    // Wesoła: 15 words (szczęsna, wolna, górna, biała, fiszerowa, dubna).
    declension!("wesoła", 15, "~a", [gender: Feminine];
    stem gen_sg: "~- like wesoła";
    nom: "~a", "~e";
    acc: "~ą", _;
    ),
    // Malec: 15 words (niger, malec, michalec, pawelec, kawalec, wujec). Like Idec.
    declension!("malec", 15, "~e~", like Idec;
    stem gen_sg: "~~- like lipowiec";
    ),
    // Wola: 14 words (zorza, kania, głowania, matynia, powierza, golenia). Like Dąbrowa, except in 3 cells: gen_ sg, dat sg, loc sg.
    declension!("wola", 14, "~a", like Dąbrowa;
    gen_: "~i", _;
    dat: "~i", _;
    loc: "~i", _;
    ),
    // Ostrowiec: 14 words (sosnowiec, królewiec, żywiec, śmigiel, drzewiec, bolesławiec). Like Idec, except in 1 cell: nom pl.
    declension!("ostrowiec", 14, "~ie~", [gender: MasculineInanimate], like Idec;
    stem lemma: "~ie~- like birmingham";
    stem gen_sg: "~~- like ostrowiec";
    nom: _, "~~e";
    ),
    // Naaman: 13 words (charon, alfred, bernard, enceladus, oberon, kain). Like Pegaz, except in 1 cell: nom pl.
    declension!("naaman", 13, "~", [gender: MasculinePersonal], like Pegaz;
    nom: _, "~i";
    ),
    // Obed: 11 words (freud, oswald, romuald, barud, marud, brud). Like Zachariasz, except in 2 cells: loc sg, voc sg.
    declension!("obed", 11, "~", [gender: MasculinePersonal], like Zachariasz;
    loc: "~zie", _;
    voc: "~zie", _;
    ),
    // Powązki: 11 words (helsinki, murcki, zaduszki, ząbki, błaszki, koluszki).
    declension!("powązki", 11, "~~i", [gender: Feminine];
    nom: "~~i", "~~i";
    gen_: _, "~e~";
    dat: _, "~~om";
    acc: _, "~~i";
    ins: _, "~~ami";
    loc: _, "~~ach";
    ),
    // Głód: 11 words (zaród, mrzygłód, pogłód, wrzód, ziemichód, głód). Like Głów, except in 2 cells: loc sg, voc sg.
    declension!("głód", 11, "~ó~", [gender: MasculinePersonal], like Głów;
    loc: "~o~zie", _;
    voc: "~o~zie", _;
    ),
    // Noe: 10 words (dante, goethe, noe, lange, nietzsche, jozue). Like Zachariasz, except in 4 cells: gen_ sg, dat sg, loc sg, voc sg.
    declension!("noe", 10, "~e", [gender: MasculinePersonal], like Zachariasz;
    stem lemma: "~e- like noe";
    gen_: "~ego", _;
    dat: "~emu", _;
    loc: "~em", _;
    voc: "~e", _;
    ),
    // Apacz: 10 words (mateusz, judasz, arkadiusz, remigiusz, klaudiusz, apacz). Like Wąż, except in 1 cell: nom pl.
    declension!("apacz", 10, "~", [gender: MasculinePersonal], like Wąż;
    nom: _, "~e";
    ),
    // Wyszogród: 10 words (siedmiogród, zachód, kaczogród, carogród, piotrogród, wschód).
    declension!("wyszogród", 10, "~ó~", [gender: MasculineInanimate];
    stem lemma: "~ó~- like birmingham";
    gen_: "~o~u", _;
    dat: "~o~owi", _;
    ins: "~o~em", _;
    loc: "~o~zie", _;
    voc: "~o~zie", _;
    ),
    // Tola: 10 words (ala, ela, ola, lola, madzia, pola). Like Anula, except in 1 cell: voc sg.
    declension!("tola", 10, "~a", [gender: Feminine], like Anula;
    voc: "~u", _;
    ),
    // Knefel: 10 words (skrzek, kurdziel, grądziel, grondziel, ćwiek, suchmiel). Like Idec, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("knefel", 10, "~e~", [gender: MasculinePersonal], like Idec;
    nom: _, "~e~owie";
    gen_: _, "~e~ów";
    acc: _, "~e~ów";
    ),
    // Piaseczno: 10 words (jaworzno, drezno, gniezno, leszno, debrzno, tuczno). Like Annopol with `ˇ` after each pattern (`oˇ` after ~), except in 3 cells: dat sg, loc sg, voc sg.
    declension!("piaseczno", 10, "~oˇ", [gender: Neuter], like Annopol with "ˇ", "oˇ" after "~";
    dat: "~uˇ", _;
    loc: "~ieˇ", _;
    voc: "~oˇ", _;
    ),
    // Lewi: 10 words (loki, bliźnięta, lewi, vivaldi, dostatni, wyszkoni).
    declension!("lewi", 10, "~";
    stem gen_sg: "~- like zełenski";
    nom: "~", "~|~owie";
    acc: "~ego", _;
    ),
    // Liberec: 9 words (liberec, dunajec, zgorzelec, mielec, popielec, grójec). Like Annopol with `ˇ` after each pattern.
    declension!("liberec", 9, "~ˇ", [gender: MasculineInanimate], like Annopol with "ˇ";
    ),
    // Ania: 9 words (hania, tania, ania, mania, monia, ksenia).
    declension!("ania", 9, "~nia", [gender: Feminine];
    stem lemma: "~ni- like dąbrowa";
    nom: _, "~nie";
    gen_: "~ni", "~ń";
    dat: "~ni", "~niom";
    acc: _, "~nie";
    ins: "~nią", "~niami";
    loc: "~ni", "~niach";
    voc: "~niu", _;
    ),
    // Bugiel: 8 words (kukiel, bugiel, smykiel, pęgiel, pengiel, fogiel).
    declension!("bugiel", 8, "~iel", [gender: MasculinePersonal];
    stem gen_sg: "~l- like zachariasz";
    nom: "~iel", "~lowie";
    acc: "~la", _;
    ),
    // Pompeje: 7 words (himalaje, pireneje, hawaje, międzyzdroje, czerniowce, pompeje). Like Racławice, except in 1 cell: gen_ pl.
    declension!("pompeje", 7, "~e", like Racławice;
    gen_: _, "~ów";
    ),
    // Stasiek: 7 words (adasiek, wiesiek, jasiek, grzesiek, mgłosiek, bosiek).
    declension!("stasiek", 7, "~siek", [gender: MasculinePersonal];
    stem gen_sg: "~śk- like rak";
    nom: "~siek", "~śkowie";
    acc: "~śka", _;
    ),
    // Cpn: 7 words (cpn, pis, mops, mon, pwn, zbowid).
    declension!("cpn", 7, "~";
    stem lemma: "~- like birmingham";
    stem gen_sg: "~-- like birmingham";
    nom: _, "~-y";
    ),
    // Koziec: 7 words (dudziec, chudziec, koziec, gryziec, maziec, hudziec).
    declension!("koziec", 7, "~ziec", [gender: MasculinePersonal];
    stem gen_sg: "~źc- like lipowiec";
    nom: "~ziec", "~źcowie";
    acc: "~źca", _;
    ),
    // Zwierzyniec: 7 words (ołomuniec, kozieniec, zwierzyniec, złocieniec, trzyniec, horyniec).
    declension!("zwierzyniec", 7, "~nie~", [gender: MasculineInanimate];
    stem lemma: "~nie~- like birmingham";
    stem gen_sg: "~ń~- like ostrowiec";
    nom: _, "~ń~e";
    ),
    // Aśka: 7 words (zośka, gośka, baśka, kaśka, joaśka, aśka).
    declension!("aśka", 7, "~śka", [gender: Feminine];
    stem lemma: "~śk- like dąbrowa";
    nom: _, "~śki";
    gen_: "~śki", "~siek";
    dat: "~śce", "~śkom";
    acc: _, "~śki";
    ins: "~śką", "~śkami";
    loc: "~śce", "~śkach";
    voc: "~śko", _;
    ),
    // Błonie: 7 words (odrodzenie, wniebowstąpienie, zmartwychwstanie, błonie, ustronie, zwiastowanie). Like Goń, except in 5 cells: nom pl, gen_ pl, dat sg, acc pl, voc sg.
    declension!("błonie", 7, "~nie", [gender: Neuter], like Goń;
    stem lemma: "~nie- like birmingham";
    nom: _, "~nia";
    gen_: _, "~ń";
    dat: "~niu", _;
    acc: _, "~nia";
    voc: "~nie", _;
    ),
    // Japet: 7 words (hamlet, hubert, lew, kohelet, paraklet, japet). Like Naaman with `ˇ` after each pattern.
    declension!("japet", 7, "~ˇ", like Naaman with "ˇ";
    ),
    // Szerlok: 6 words (ludwik, krzyżak, krawczuk, głombik, błaszak, szerlok).
    declension!("szerlok", 6, "~k", [gender: MasculinePersonal];
    stem lemma: "~k- like zachariasz";
    stem gen_sg: "~k- like rak";
    nom: _, "~cy";
    ),
    // Luter: 6 words (aleksander, zoroaster, sylwester, luter, kacper, chaber). Like Żubr with `ˇ` after each pattern.
    declension!("luter", 6, "~ˇ", [gender: MasculinePersonal], like Żubr with "ˇ";
    ),
    // Triest: 6 words (belfast, holocaust, triest, holokaust, babimost, zawichost).
    declension!("triest", 6, "~tˇ", [gender: MasculineInanimate];
    stem lemma: "~tˇ- like birmingham";
    nom: _, "~tyˇ";
    gen_: "~tuˇ", "~tówˇ";
    dat: "~towiˇ", "~tomˇ";
    acc: _, "~tyˇ";
    ins: "~temˇ", "~tamiˇ";
    loc: "~cieˇ", "~tachˇ";
    voc: "~cieˇ", _;
    ),
    // Grójec: 6 words (niemen, homel, kowel, rewel, chociwel, reszel). Like Idec, except in 7 cells: nom pl, gen_ pl, dat pl, acc sg, acc pl, ins pl, ….
    declension!("grójec", 6, "~e~", [gender: MasculineInanimate], like Idec;
    nom: _, derived;
    gen_: _, derived;
    dat: _, derived;
    acc: "~e~", derived;
    ins: _, derived;
    loc: _, derived;
    ),
    // Bydgoszcz: 6 words (bydgoszcz, choroszcz, mokosz, supraśl, gołańcz, chodzież). Like Buk.
    declension!("bydgoszcz", 6, "~", [gender: Feminine], like Buk;
    stem gen_sg: "~- like bydgoszcz";
    ),
    // Śnieżka: 5 words (mońka, miśka, pogródka, zawadka, bańka). Like Kościuszko.
    declension!("śnieżka", 5, "~ka", [gender: MasculinePersonal], like Kościuszko;
    stem lemma: "~k- like dąbrowa";
    ),
    // Rudy: 5 words (ciepły, cały, goły, rychły, obiegły). Like Horacy, except in 1 cell: nom pl.
    declension!("rudy", 5, "~y", [gender: MasculinePersonal], like Horacy;
    nom: _, "~zi";
    ),
    // Myślibórz: 5 words (racibórz, myślibórz, przedbórz, wolbórz, zdrój). Like Wyszogród.
    declension!("myślibórz", 5, "~ó~", [gender: MasculineInanimate], like Wyszogród;
    stem gen_sg: "~o~- like kamień";
    ),
    // Ali: 5 words (ali, aureli, marceli, bazyli, wasyli).
    declension!("ali", 5, "~i", [gender: MasculinePersonal];
    nom: "~i", "~owie";
    gen_: "~ego", "~ich";
    dat: "~emu", "~im";
    acc: "~ego", "~ich";
    ins: "~im", "~imi";
    loc: "~im", "~ich";
    voc: "~i", _;
    ),
    // Indie: 5 words (indie, seszele, antyle, dardanele, szawle). Like Racławice, except in 1 cell: gen_ pl.
    declension!("indie", 5, "~e", like Racławice;
    gen_: _, "~i";
    ),
    // Tadeusz: 5 words (tadeusz, andrzej, smalcjusz, zenobiusz, lukianusz). Like Zachariasz, except in 1 cell: nom pl.
    declension!("tadeusz", 5, "~", [gender: MasculinePersonal], like Zachariasz;
    nom: _, "~e|~owie";
    ),
    // Stambuł: 5 words (bajkał, komsomoł, stambuł, istambuł, kaguł). Like Bagno, except in 5 cells: gen_ sg, dat sg, ins sg, loc sg, voc sg.
    declension!("stambuł", 5, "~ł", [gender: MasculineInanimate], like Bagno;
    stem lemma: "~ł- like birmingham";
    gen_: "~łu", _;
    dat: "~łowi", _;
    ins: "~łem", _;
    loc: "~le", _;
    voc: "~le", _;
    ),
    // Woźniki: 5 words (saloniki, suwałki, oborniki, panewniki, woźniki). Like Lipiny, except in 3 cells: nom sg, nom pl, acc pl.
    declension!("woźniki", 5, "~i", [gender: Feminine], like Lipiny;
    nom: "~i", "~i";
    acc: _, "~i";
    ),
    // Biniek: 5 words (bieniek, staniek, heniek, biniek, maniek).
    declension!("biniek", 5, "~niek", [gender: MasculinePersonal];
    stem gen_sg: "~ńk- like rak";
    nom: "~niek", "~ńkowie";
    acc: "~ńka", _;
    ),
    // Stary: 5 words (stary, szczodry, szary, chytry, makary). Like Horacy, except in 1 cell: nom pl.
    declension!("stary", 5, "~y", [gender: MasculinePersonal], like Horacy;
    nom: _, "~zy";
    ),
    // Wywrót: 5 words (porzybót, parzybót, rzeszót, wywrót, skorzybót).
    declension!("wywrót", 5, "~ót", [gender: MasculinePersonal];
    stem gen_sg: "~o- like wit";
    nom: "~ót", "~otowie";
    acc: "~ota", _;
    ),
    // Jacuś: 5 words (jaruś, piotruś, jacuś, miruś, aguś). Like Wituś, except in 1 cell: nom pl.
    declension!("jacuś", 5, "~ś", [gender: MasculinePersonal], like Wituś;
    nom: _, "~sie|~siowie";
    ),
    // Kokot: 5 words (kot, tyber, kokot, łańcut, łobez). Like Natolin with `ˇ` after each pattern.
    declension!("kokot", 5, "~ˇ", [gender: MasculineInanimate], like Natolin with "ˇ";
    ),
    // Gąsiorów: 5 words (bóbr, przewodów, biskupów, krzyżtopór, gąsiorów). Like Wyszogród, except in 2 cells: loc sg, voc sg.
    declension!("gąsiorów", 5, "~ó~", [gender: MasculineInanimate], like Wyszogród;
    loc: "~o~ie", _;
    voc: "~o~ie", _;
    ),
    // Fiszer: 4 words (melchior, fiszer, krzysztofor, treder). Like Żubr, except in 1 cell: nom pl.
    declension!("fiszer", 4, "~", [gender: MasculinePersonal], like Żubr;
    nom: _, "~zy";
    ),
    // Prypeć: 4 words (noteć, prypeć, pakość, solidarność). Like Bagno.
    declension!("prypeć", 4, "~ć", [gender: Feminine], like Bagno;
    stem lemma: "~ć- like birmingham";
    stem gen_sg: "~ci- like prypeć";
    ),
    // Józia: 4 words (zuzia, jadzia, kazia, józia).
    declension!("józia", 4, "~zia", [gender: Feminine];
    stem lemma: "~zi- like dąbrowa";
    nom: _, "~zie";
    gen_: "~zi", "~ź";
    dat: "~zi", "~ziom";
    acc: _, "~zie";
    ins: "~zią", "~ziami";
    loc: "~zi", "~ziach";
    voc: "~ziu", _;
    ),
    // Witek: 4 words (witek, adamek, stefanek, antek). Like Idec, except in 1 cell: nom pl.
    declension!("witek", 4, "~e~", [gender: MasculinePersonal], like Idec;
    stem gen_sg: "~~- like rak";
    nom: _, "~~i|~~owie";
    ),
    // Nykiel: 4 words (nykiel, brongiel, brągiel, miszkiel). Like Idec, except in 4 cells: nom sg, nom pl, gen_ pl, acc pl.
    declension!("nykiel", 4, "~ie~", [gender: MasculinePersonal], like Idec;
    nom: "~ie~", "~ie~owie";
    gen_: _, "~ie~ów";
    acc: _, "~ie~ów";
    ),
    // Kieś: 4 words (białoruś, oś, kieś, wszechruś). Like Tosia, except in 2 cells: gen_ pl, voc sg.
    declension!("kieś", 4, "~ś", like Tosia;
    stem lemma: "~ś- like birmingham";
    gen_: _, "~si";
    voc: "~si", _;
    ),
    // Borki: 4 words (białobrzegi, bobrowniki, beatlesi, borki). Like Woźniki, except in 1 cell: gen_ pl.
    declension!("borki", 4, "~i", [gender: MasculineInanimate], like Woźniki;
    gen_: _, "~ów";
    ),
    // Szaweł: 4 words (paweł, gaweł, dybeł, szaweł).
    declension!("szaweł", 4, "~łˇ", [gender: MasculinePersonal];
    nom: "~łˇ", "~łowieˇ";
    gen_: "~łaˇ", "~łówˇ";
    dat: "~łowiˇ", "~łomˇ";
    acc: "~łaˇ", "~łówˇ";
    ins: "~łemˇ", "~łamiˇ";
    loc: "~leˇ", "~łachˇ";
    voc: "~leˇ", _;
    ),
    // Drozd: 4 words (drozd, gorazd, gwizd, druzd). Like Obed with `ˇ` after each pattern.
    declension!("drozd", 4, "~ˇ", [gender: MasculinePersonal], like Obed with "ˇ";
    ),
    // Borek: 4 words (damaszek, borek, szadek, łek). Like Buk with `ˇ` after each pattern.
    declension!("borek", 4, "~ˇ", [gender: MasculineInanimate], like Buk with "ˇ";
    ),
    // Becia: 4 words (tonia, ewunia, becia, rysia). Like Tola, except in 3 cells: gen_ sg, dat sg, loc sg.
    declension!("becia", 4, "~a", [gender: Feminine], like Tola;
    gen_: "~", _;
    dat: "~", _;
    loc: "~", _;
    ),
    // Estera: 3 words (kinga, olga, helga). Like Polana, except in 2 cells: dat sg, loc sg.
    declension!("estera", 3, "~a", [gender: Feminine], like Polana;
    dat: "~ze", _;
    loc: "~ze", _;
    ),
    // Maryja: 3 words (maja, kaja, maryja). Like Biskaja, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("maryja", 3, "~ja", [gender: Feminine], like Biskaja;
    nom: _, "~je";
    gen_: _, "~i";
    acc: _, "~je";
    ),
    // Landau: 3 words (landau, turnau, strehlau). Like Zachariasz, except in 2 cells: loc sg, voc sg.
    declension!("landau", 3, "~", [gender: MasculinePersonal], like Zachariasz;
    loc: "~", _;
    voc: "~", _;
    ),
    // Iwo: 3 words (otto, hugo, iwo).
    declension!("iwo", 3, "~", [gender: MasculinePersonal];
    stem gen_sg: "~n- like pegaz";
    nom: "~", "~nowie";
    acc: "~na", _;
    ),
    // Węgry: 3 words (niemcy, węgry, austro-węgry). Like Powązki, except in 5 cells: nom sg, nom pl, gen_ pl, acc pl, loc pl.
    declension!("węgry", 3, "~~y", like Powązki;
    nom: "~~y", "~~y";
    gen_: _, "~ie~";
    acc: _, "~~y";
    loc: _, "~~zech";
    ),
    // Ariel: 3 words (abel, ariel, umbriel). Like Idec, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("ariel", 3, "~e~", [gender: MasculinePersonal], like Idec;
    nom: _, "~e~e";
    gen_: _, "~~i";
    acc: _, "~~i";
    ),
    // Idzi: 3 words (antoni, bruzi, idzi). Like Lewi, except in 1 cell: nom pl.
    declension!("idzi", 3, "~", [gender: MasculinePersonal], like Lewi;
    nom: _, "~owie";
    ),
    // Puchacze: 3 words (karkonosze, puchacze, barszcze). Like Racławice, except in 1 cell: gen_ pl.
    declension!("puchacze", 3, "~e", [gender: MasculineInanimate], like Racławice;
    gen_: _, "~y";
    ),
    // Bysiec: 3 words (bysiec, kosiec, misiec).
    declension!("bysiec", 3, "~siec", [gender: MasculinePersonal];
    stem gen_sg: "~śc- like lipowiec";
    nom: "~siec", "~ścowie";
    acc: "~śca", _;
    ),
    // Jawor: 3 words (arktur, twitter, jawor). Like Annopol, except in 2 cells: loc sg, voc sg.
    declension!("jawor", 3, "~", like Annopol;
    loc: "~ze", _;
    voc: "~ze", _;
    ),
    // Lipnica: 3 words (róża, olsza, lipnica). Like Oleśnica, except in 3 cells: nom pl, gen_ pl, acc pl.
    declension!("lipnica", 3, "~a", like Oleśnica;
    nom: _, "~e";
    gen_: _, "~";
    acc: _, "~e";
    ),
    // Azor: 3 words (tytus, azor, justyn). Like Natolin.
    declension!("azor", 3, "~", like Natolin;
    stem lemma: "~- like zachariasz";
    ),
    // Wlazło: 3 words (wlazło, wlizło, oślizło).
    declension!("wlazło", 3, "~zło", [gender: MasculinePersonal];
    stem lemma: "~zł- like szczerbo";
    nom: _, "~złowie";
    gen_: "~zły", "~złów";
    dat: "~źle", "~złom";
    acc: _, "~złów";
    ins: "~złą", "~złami";
    loc: "~źle", "~złach";
    voc: "~zło", _;
    ),
    // Jagiełło: 3 words (jagiełło, skirgiełło, dowgiałło).
    declension!("jagiełło", 3, "~łło", [gender: MasculinePersonal];
    stem lemma: "~łł- like szczerbo";
    nom: _, "~łłowie";
    gen_: "~łły", "~łłów";
    dat: "~lle", "~łłom";
    acc: _, "~łłów";
    ins: "~łłą", "~łłami";
    loc: "~lle", "~łłach";
    voc: "~łło", _;
    ),
    // Kiec: 3 words (piec, kiec, świec). Like Nykiel, except in 1 cell: voc sg.
    declension!("kiec", 3, "~ie~", [gender: MasculinePersonal], like Nykiel;
    voc: "~~ze", _;
    ),
    // Prl: 3 words (sok, prl, zus). Like Buk, except in 5 cells: gen_ sg, dat sg, ins sg, loc sg, voc sg.
    declension!("prl", 3, "~", [gender: MasculineInanimate], like Buk;
    gen_: "~-u", _;
    dat: "~-owi", _;
    ins: "~-em", _;
    loc: "~-u", _;
    voc: "~-u", _;
    ),
    // Natalli: 3 words (natalli, bacciarelli, tekieli). Like Ali, except in 1 cell: nom pl.
    declension!("natalli", 3, "~i", [gender: MasculinePersonal], like Ali;
    nom: _, "~i";
    ),
    // Dauha: 3 words (ad-dauha, dauha, sapieha).
    declension!("dauha", 3, "~ha";
    stem lemma: "~h- like dąbrowa";
    nom: _, "~howie";
    gen_: "~hy", "~hów";
    dat: "~sze|~że", "~hom";
    acc: _, "~hów";
    ins: "~hą", "~hami";
    loc: "~sze|~że", "~hach";
    voc: "~ho", _;
    ),
    // Cło: 3 words (kadzidło, cło, gopło). Like Stambuł with `` after each pattern (`o` after ł), except in 3 cells: gen_ sg, dat sg, voc sg.
    declension!("cło", 3, "~ło", [gender: Neuter], like Stambuł with "", "o" after "ł";
    gen_: "~ła", _;
    dat: "~łu", _;
    voc: "~ło", _;
    ),
    // Iruś: 3 words (antoś, iruś, jorguś). Like Jacuś, except in 2 cells: gen_ pl, acc pl.
    declension!("iruś", 3, "~ś", [gender: MasculinePersonal], like Jacuś;
    gen_: _, "~si";
    acc: _, "~si";
    ),
    // Bbwr: 3 words (zbir, hur, bbwr). Like Prl, except in 2 cells: loc sg, voc sg.
    declension!("bbwr", 3, "~", [gender: MasculineInanimate], like Prl;
    loc: "~-ze", _;
    voc: "~-ze", _;
    ),
    // Guzda: 3 words (guzda, zagozda, gwiozda). Like Juda with `ˇ` after each pattern.
    declension!("guzda", 3, "~aˇ", [gender: MasculinePersonal], like Juda with "ˇ";
    ),
    // Świder: 3 words (świder, wawer, skamander). Like Jawor with `ˇ` after each pattern.
    declension!("świder", 3, "~ˇ", [gender: MasculineInanimate], like Jawor with "ˇ";
    ),
];
