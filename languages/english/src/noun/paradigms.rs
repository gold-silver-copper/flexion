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
/// `noun` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem noun_sg: $s0:tt;)?
     $(stem noun_pl: $s1:tt;)?
     $(noun: $c0:tt, $c1:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
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
/// like that it reads (`pl_as_sg`: one value read as another).
const PL_AS_SG: [u16; 2] = [0, 0];

/// A view: per cell (in the cells' order), the cell of the declension it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 2] = [1, 1];

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `line`: 197,330 words.
    Line,
    /// Like `body`: 12,168 words.
    Body,
    /// Like `wash`: 7,533 words.
    Wash,
    /// Like `silverfish`: 2,606 words.
    Silverfish,
    /// Like `man`: 1,731 words.
    Man,
    /// Like `synthesis`: 1,682 words.
    Synthesis,
    /// Like `medium`: 1,457 words.
    Medium,
    /// Like `focus`: 797 words.
    Focus,
    /// Like `aura`: 536 words.
    Aura,
    /// Like `lexicon`: 438 words.
    Lexicon,
    /// Like `glans`: 202 words.
    Glans,
    /// Like `omphalos`: 175 words.
    Omphalos,
    /// Like `matrix`: 155 words.
    Matrix,
    /// Like `stigma`: 114 words.
    Stigma,
    /// Like `leaf`: 98 words.
    Leaf,
    /// Like `tasto`: 96 words.
    Tasto,
    /// Like `madrich`: 77 words.
    Madrich,
    /// Like `life`: 74 words.
    Life,
    /// Like `foot`: 48 words.
    Foot,
    /// Like `apex`: 41 words.
    Apex,
    /// Like `child`: 36 words.
    Child,
    /// Like `blin`: 35 words.
    Blin,
    /// Like `tekufah`: 35 words.
    Tekufah,
    /// Like `mouse`: 33 words.
    Mouse,
    /// Like `metazoan`: 30 words.
    Metazoan,
    /// Like `mage`: 28 words.
    Mage,
    /// Like `person`: 25 words.
    Person,
    /// Like `abendmusik`: 25 words.
    Abendmusik,
    /// Like `eros`: 24 words.
    Eros,
    /// Like `jynx`: 24 words.
    Jynx,
    /// Like `quiz`: 22 words.
    Quiz,
    /// Like `alumna`: 20 words.
    Alumna,
    /// Like `exotic`: 20 words.
    Exotic,
    /// Like `lira`: 20 words.
    Lira,
    /// Like `tableau`: 19 words.
    Tableau,
    /// Like `genus`: 17 words.
    Genus,
    /// Like `schutzstaffel`: 17 words.
    Schutzstaffel,
    /// Like `bitumen`: 16 words.
    Bitumen,
    /// Like `pedale`: 14 words.
    Pedale,
    /// Like `pocketful`: 14 words.
    Pocketful,
    /// Like `umbo`: 13 words.
    Umbo,
    /// Like `mitzva`: 13 words.
    Mitzva,
    /// Like `bes`: 12 words.
    Bes,
    /// Like `stirps`: 12 words.
    Stirps,
    /// Like `money`: 11 words.
    Money,
    /// Like `toxotes`: 11 words.
    Toxotes,
    /// Like `ordo`: 10 words.
    Ordo,
    /// Like `intaglio`: 7 words.
    Intaglio,
    /// Like `kotlet`: 7 words.
    Kotlet,
    /// Like `melamed`: 7 words.
    Melamed,
    /// Like `fomes`: 6 words.
    Fomes,
    /// Like `asclepiades`: 6 words.
    Asclepiades,
    /// Like `kore`: 6 words.
    Kore,
    /// Like `kylix`: 6 words.
    Kylix,
    /// Like `krouzek`: 6 words.
    Krouzek,
    /// Like `notzri`: 5 words.
    Notzri,
    /// Like `speso`: 5 words.
    Speso,
    /// Like `vomitus`: 5 words.
    Vomitus,
    /// Like `formans`: 5 words.
    Formans,
    /// Like `perek`: 5 words.
    Perek,
    /// Like `vraka`: 5 words.
    Vraka,
    /// Like `corpus`: 4 words.
    Corpus,
    /// Like `fondaco`: 4 words.
    Fondaco,
    /// Like `menorah`: 4 words.
    Menorah,
    /// Like `ceras`: 4 words.
    Ceras,
    /// Like `raphe`: 4 words.
    Raphe,
    /// Like `tripus`: 4 words.
    Tripus,
    /// Like `silrada`: 4 words.
    Silrada,
    /// Like `chous`: 4 words.
    Chous,
    /// Like `wunderkind`: 3 words.
    Wunderkind,
    /// Like `rebetiko`: 3 words.
    Rebetiko,
    /// Like `krone`: 3 words.
    Krone,
    /// Like `muhajir`: 3 words.
    Muhajir,
    /// Like `heemraad`: 3 words.
    Heemraad,
    /// Like `apikoros`: 3 words.
    Apikoros,
    /// Like `dars`: 3 words.
    Dars,
    /// Like `mosotho`: 3 words.
    Mosotho,
    /// Like `colstaff`: 3 words.
    Colstaff,
    /// Like `brocha`: 3 words.
    Brocha,
    /// Like `seacatch`: 3 words.
    Seacatch,
    /// Like `valenok`: 3 words.
    Valenok,
    /// Like `forefemur`: 3 words.
    Forefemur,
    /// Like `ibhaca`: 3 words.
    Ibhaca,
    /// Like `gadol`: 3 words.
    Gadol,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 84] = [
        Declension::Line,
        Declension::Body,
        Declension::Wash,
        Declension::Silverfish,
        Declension::Man,
        Declension::Synthesis,
        Declension::Medium,
        Declension::Focus,
        Declension::Aura,
        Declension::Lexicon,
        Declension::Glans,
        Declension::Omphalos,
        Declension::Matrix,
        Declension::Stigma,
        Declension::Leaf,
        Declension::Tasto,
        Declension::Madrich,
        Declension::Life,
        Declension::Foot,
        Declension::Apex,
        Declension::Child,
        Declension::Blin,
        Declension::Tekufah,
        Declension::Mouse,
        Declension::Metazoan,
        Declension::Mage,
        Declension::Person,
        Declension::Abendmusik,
        Declension::Eros,
        Declension::Jynx,
        Declension::Quiz,
        Declension::Alumna,
        Declension::Exotic,
        Declension::Lira,
        Declension::Tableau,
        Declension::Genus,
        Declension::Schutzstaffel,
        Declension::Bitumen,
        Declension::Pedale,
        Declension::Pocketful,
        Declension::Umbo,
        Declension::Mitzva,
        Declension::Bes,
        Declension::Stirps,
        Declension::Money,
        Declension::Toxotes,
        Declension::Ordo,
        Declension::Intaglio,
        Declension::Kotlet,
        Declension::Melamed,
        Declension::Fomes,
        Declension::Asclepiades,
        Declension::Kore,
        Declension::Kylix,
        Declension::Krouzek,
        Declension::Notzri,
        Declension::Speso,
        Declension::Vomitus,
        Declension::Formans,
        Declension::Perek,
        Declension::Vraka,
        Declension::Corpus,
        Declension::Fondaco,
        Declension::Menorah,
        Declension::Ceras,
        Declension::Raphe,
        Declension::Tripus,
        Declension::Silrada,
        Declension::Chous,
        Declension::Wunderkind,
        Declension::Rebetiko,
        Declension::Krone,
        Declension::Muhajir,
        Declension::Heemraad,
        Declension::Apikoros,
        Declension::Dars,
        Declension::Mosotho,
        Declension::Colstaff,
        Declension::Brocha,
        Declension::Seacatch,
        Declension::Valenok,
        Declension::Forefemur,
        Declension::Ibhaca,
        Declension::Gadol,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 84] = [
    // Line: 197,330 words (free, encyclopedia, portmanteau, encyclopaedia, cat, word).
    declension!("line", 197330, "~";
    noun: "~", "~s";
    ),
    // Body: 12,168 words (dictionary, lexicography, etymology, abbacy, abditory, abecedary).
    declension!("body", 12168, "~y";
    noun: "~y", "~ies";
    ),
    // Wash: 7,533 words (march, abbess, abjectness, ableness, abortiveness, abraxas). Like Line, except in 1 cell: noun pl.
    declension!("wash", 7533, "~", like Line;
    noun: _, "~es";
    ),
    // Silverfish: 2,606 words (semantics, craft, abatis, abatvoix, english, bpd). Like Line, except in 0 cells: .
    declension!("silverfish", 2606, "~", like Line as PL_AS_SG;
    ),
    // Man: 1,731 words (chairman, man, hardwareman, woman, englishman, hammerman).
    declension!("man", 1731, "~a~";
    noun: "~a~", "~e~";
    ),
    // Synthesis: 1,682 words (abiogenesis, gnosis, acidosis, analysis, basis, crisis). Like Man, except in 1 cell: noun sg.
    declension!("synthesis", 1682, "~i~", like Man;
    noun: "~i~", _;
    ),
    // Medium: 1,457 words (abomasum, millennium, credendum, haustorium, corrigendum, phylum).
    declension!("medium", 1457, "~um";
    noun: "~um", "~a";
    ),
    // Focus: 797 words (thesaurus, abaciscus, abaculus, abacus, acarus, stylus).
    declension!("focus", 797, "~us";
    noun: "~us", "~i";
    ),
    // Aura: 536 words (fabella, pupa, vertebra, abolla, familia, larva). Like Line, except in 1 cell: noun pl.
    declension!("aura", 536, "~", like Line;
    noun: _, "~e";
    ),
    // Lexicon: 438 words (hexahedron, tetrahedron, pyramidion, eidolon, polyhedron, lexicon). Like Medium, except in 1 cell: noun sg.
    declension!("lexicon", 438, "~on", like Medium;
    noun: "~on", _;
    ),
    // Glans: 202 words (absis, pyramis, pes, apoapsis, periapsis, enteritis).
    declension!("glans", 202, "~~";
    noun: "~~", "~de~";
    ),
    // Omphalos: 175 words (mythos, ouroboros, eromenos, temenos, logos, dromos). Like Focus, except in 1 cell: noun sg.
    declension!("omphalos", 175, "~s", like Focus;
    noun: "~s", _;
    ),
    // Matrix: 155 words (hallux, generatrix, matrix, appendix, calx, faux).
    declension!("matrix", 155, "~x";
    noun: "~x", "~ces";
    ),
    // Stigma: 114 words (smegma, schema, stigma, lymphoma, stoma, problema). Like Line, except in 1 cell: noun pl.
    declension!("stigma", 114, "~", like Line;
    noun: _, "~ta";
    ),
    // Leaf: 98 words (aardwolf, wolf, self, calf, elf, half).
    declension!("leaf", 98, "~f";
    noun: "~f", "~ves";
    ),
    // Tasto: 96 words (glissando, zucchino, gelato, corno, spaghetto, graffito). Like Focus, except in 1 cell: noun sg.
    declension!("tasto", 96, "~o", like Focus;
    noun: "~o", _;
    ),
    // Madrich: 77 words (ars, kibbutz, hassid, minyan, zuz, gaon). Like Line, except in 1 cell: noun pl.
    declension!("madrich", 77, "~", like Line;
    noun: _, "~im";
    ),
    // Life: 74 words (wife, life, knife, alewife, jackknife, housewife).
    declension!("life", 74, "~f~";
    noun: "~f~", "~v~s";
    ),
    // Foot: 48 words (tooth, foot, goose, tenderfoot, bucktooth, leadfoot).
    declension!("foot", 48, "~oo~";
    noun: "~oo~", "~ee~";
    ),
    // Apex: 41 words (apex, pollex, vertex, pontifex, vortex, codex).
    declension!("apex", 41, "~~x";
    noun: "~~x", "~ic~s";
    ),
    // Child: 36 words (child, grandchild, lovechild, godchild, stepchild, schoolchild). Like Line, except in 1 cell: noun pl.
    declension!("child", 36, "~", like Line;
    noun: _, "~ren";
    ),
    // Blin: 35 words (sent, triumvir, garda, kroon, monsignor, hetaira). Like Line, except in 1 cell: noun pl.
    declension!("blin", 35, "~", like Line;
    noun: _, "~i";
    ),
    // Tekufah: 35 words (tzedakah, matzah, massebah, sufganiyah, petuhah, petuchah).
    declension!("tekufah", 35, "~ah";
    noun: "~ah", "~ot";
    ),
    // Mouse: 33 words (mouse, dormouse, louse, woodlouse, titmouse, flittermouse).
    declension!("mouse", 33, "~ous~";
    noun: "~ous~", "~ic~";
    ),
    // Metazoan: 30 words (bryozoan, metazoan, protozoan, micrometazoan, cubozoan, hydrozoan).
    declension!("metazoan", 30, "~n";
    noun: "~n", "~|~ns";
    ),
    // Mage: 28 words (dilettante, duce, mage, genie, zampone, cognoscente). Like Focus, except in 1 cell: noun sg.
    declension!("mage", 28, "~e", like Focus;
    noun: "~e", _;
    ),
    // Person: 25 words (person, conperson, layperson, floorperson, linesperson, catperson).
    declension!("person", 25, "~rs~n";
    noun: "~rs~n", "~~ple";
    ),
    // Abendmusik: 25 words (ox, ahlspiess, ubermensch, luftmensch, frau, herr). Like Line, except in 1 cell: noun pl.
    declension!("abendmusik", 25, "~", like Line;
    noun: _, "~en";
    ),
    // Eros: 24 words (pons, dens, eros, gens, paries, veritas). Like Glans, except in 1 cell: noun pl.
    declension!("eros", 24, "~~", like Glans;
    noun: _, "~te~";
    ),
    // Jynx: 24 words (larynx, pharynx, coccyx, meninx, salpinx, grex). Like Matrix, except in 1 cell: noun pl.
    declension!("jynx", 24, "~x", like Matrix;
    noun: _, "~ges";
    ),
    // Quiz: 22 words (quiz, lez, fez, spaz, schnoz, whiz). Like Line, except in 1 cell: noun pl.
    declension!("quiz", 22, "~", like Line;
    noun: _, "~zes";
    ),
    // Alumna: 20 words (suka, alumna, sastruga, squadrista, pisanka, capicola). Like Focus, except in 1 cell: noun sg.
    declension!("alumna", 20, "~a", like Focus;
    noun: "~a", _;
    ),
    // Exotic: 20 words (cornu, exotic, genu, vas, lev, geas). Like Line, except in 1 cell: noun pl.
    declension!("exotic", 20, "~", like Line;
    noun: _, "~a";
    ),
    // Lira: 20 words (lira, zucchina, libbra, frottola, farinata, braciola). Like Aura, except in 1 cell: noun sg.
    declension!("lira", 20, "~a", like Aura;
    noun: "~a", _;
    ),
    // Tableau: 19 words (beau, tableau, adieu, gateau, milieu, bateau). Like Line, except in 1 cell: noun pl.
    declension!("tableau", 19, "~", like Line;
    noun: _, "~x";
    ),
    // Genus: 17 words (genus, vulnus, latus, ulcus, viscus, nothogenus). Like Focus, except in 1 cell: noun pl.
    declension!("genus", 17, "~us", like Focus;
    noun: _, "~era";
    ),
    // Schutzstaffel: 17 words (ee, pease, frikadelle, kartoffel, ursprache, kanone). Like Line, except in 1 cell: noun pl.
    declension!("schutzstaffel", 17, "~", like Line;
    noun: _, "~n";
    ),
    // Bitumen: 16 words (nomen, rumen, numen, bitumen, crimen, foramen). Like Man, except in 1 cell: noun pl.
    declension!("bitumen", 16, "~e~", like Man as SG_AS_PL;
    noun: _, "~i~a";
    ),
    // Pedale: 14 words (quale, centrale, insigne, memorabile, marginale, radiale). Like Mage, except in 1 cell: noun pl.
    declension!("pedale", 14, "~e", like Mage;
    noun: _, "~ia";
    ),
    // Pocketful: 14 words (passerby, pocketful, tubful, courtmartial, swagful, courtful). Like Glans, except in 1 cell: noun pl.
    declension!("pocketful", 14, "~~", like Glans;
    noun: _, "~s~";
    ),
    // Umbo: 13 words (umbo, confessio, comedo, regio, divisio, pernio). Like Line, except in 1 cell: noun pl.
    declension!("umbo", 13, "~", like Line;
    noun: _, "~nes";
    ),
    // Mitzva: 13 words (mitzva, tzedaka, halacha, moshava, matza, sefira). Like Tekufah, except in 1 cell: noun sg.
    declension!("mitzva", 13, "~a", like Tekufah;
    noun: "~a", _;
    ),
    // Bes: 12 words (es, dis, bes, lis, tas, dorcas). Like Line, except in 1 cell: noun pl.
    declension!("bes", 12, "~", like Line;
    noun: _, "~ses";
    ),
    // Stirps: 12 words (urbs, polis, erinys, labrys, stirps, prytanis). Like Man, except in 1 cell: noun sg.
    declension!("stirps", 12, "~~", like Man;
    noun: "~~", _;
    ),
    // Money: 11 words (money, covey, fogey, curtsey, jarvey, cartooney).
    declension!("money", 11, "~~y";
    noun: "~~y", "~i~s|~~ys";
    ),
    // Toxotes: 11 words (erastes, toxotes, agonothetes, metretes, protostates, bouleutes). Like Wash, except in 1 cell: noun pl.
    declension!("toxotes", 11, "~es", like Wash as SG_AS_PL;
    noun: _, "~ai";
    ),
    // Ordo: 10 words (ordo, imago, cardo, propago, suffrago, margo). Like Tasto, except in 1 cell: noun pl.
    declension!("ordo", 10, "~o", like Tasto;
    noun: _, "~ines";
    ),
    // Intaglio: 7 words (intaglio, finocchio, passaggio, libeccio, fenochio, fenocchio). Like Silverfish, except in 1 cell: noun sg.
    declension!("intaglio", 7, "~o", like Silverfish;
    noun: "~o", _;
    ),
    // Kotlet: 7 words (kotlet, kabanos, varenyk, nalysnyk, nalesnyk, syrnyk). Like Line, except in 1 cell: noun pl.
    declension!("kotlet", 7, "~", like Line;
    noun: _, "~y";
    ),
    // Melamed: 7 words (kofer, melamed, yored, shomer, poseq, misnaged). Like Bitumen, except in 1 cell: noun pl.
    declension!("melamed", 7, "~e~", like Bitumen;
    noun: _, "~~im";
    ),
    // Fomes: 6 words (eques, stipes, limes, termes, fomes, vicecomes). Like Glans, except in 1 cell: noun pl.
    declension!("fomes", 6, "~~", like Glans;
    noun: _, "~it~";
    ),
    // Asclepiades: 6 words (hermes, cerastes, psaltes, asclepiades, protopsaltis, mystes). Like Man, except in 1 cell: noun sg.
    declension!("asclepiades", 6, "~~s", like Man as PL_AS_SG;
    noun: "~~s", _;
    ),
    // Kore: 6 words (kore, arche, spithame, hekte, oinokhoe, drakhme). Like Mage, except in 1 cell: noun pl.
    declension!("kore", 6, "~e", like Mage;
    noun: _, "~ai";
    ),
    // Kylix: 6 words (phylax, kylix, somatophylax, nomophylax, larnax, khoinix). Like Matrix, except in 1 cell: noun pl.
    declension!("kylix", 6, "~x", like Matrix;
    noun: _, "~kes";
    ),
    // Krouzek: 6 words (chervonets, tchervonets, strelets, popovets, bezpopovets, krouzek). Like Bitumen, except in 1 cell: noun pl.
    declension!("krouzek", 6, "~e~", like Bitumen;
    noun: _, "~~y";
    ),
    // Notzri: 5 words (ashkenazi, illui, losse, ashkephardi, notzri). Like Line, except in 1 cell: noun pl.
    declension!("notzri", 5, "~", like Line;
    noun: _, "~m";
    ),
    // Speso: 5 words (stelo, speso, denaskulo, spesmilo, esperantido). Like Line, except in 1 cell: noun pl.
    declension!("speso", 5, "~", like Line;
    noun: _, "~j";
    ),
    // Vomitus: 5 words (vomitus, generalissimus, pangasius, cytofilius, systylius). Like Medium, except in 1 cell: noun sg.
    declension!("vomitus", 5, "~us", like Medium;
    noun: "~us", _;
    ),
    // Formans: 5 words (definiens, explanans, accidens, explicans, formans). Like Omphalos, except in 1 cell: noun pl.
    declension!("formans", 5, "~s", like Omphalos;
    noun: _, "~tia";
    ),
    // Perek: 5 words (sefer, perek, pesher, pereq, shoresh). Like Bitumen, except in 1 cell: noun pl.
    declension!("perek", 5, "~e~", like Bitumen;
    noun: _, "~a~im";
    ),
    // Vraka: 5 words (mantinada, xueta, vraka, archontia, flaouna). Like Wash, except in 1 cell: noun sg.
    declension!("vraka", 5, "~a", like Wash;
    noun: "~a", _;
    ),
    // Corpus: 4 words (corpus, pectus, subcorpus, metacorpus). Like Focus, except in 1 cell: noun pl.
    declension!("corpus", 4, "~us", like Focus;
    noun: _, "~ora";
    ),
    // Fondaco: 4 words (arco, fondaco, attacco, intrico). Like Tasto, except in 1 cell: noun pl.
    declension!("fondaco", 4, "~o", like Tasto;
    noun: _, "~hi";
    ),
    // Menorah: 4 words (menorah, sephirah, neginah, sefirah). Like Tekufah, except in 1 cell: noun pl.
    declension!("menorah", 4, "~ah", like Tekufah;
    noun: _, "~oth";
    ),
    // Ceras: 4 words (teras, ceras, hemiteras, orthoceras). Like Omphalos, except in 1 cell: noun pl.
    declension!("ceras", 4, "~s", like Omphalos;
    noun: _, "~ta";
    ),
    // Raphe: 4 words (agape, raphe, parabole, scolopale). Like Glans, except in 1 cell: noun pl.
    declension!("raphe", 4, "~~", like Glans;
    noun: _, "~a~";
    ),
    // Tripus: 4 words (pleiopus, pleopus, tripus, lagopus).
    declension!("tripus", 4, "~u~";
    noun: "~u~", "~ode~";
    ),
    // Silrada: 4 words (barma, kotleta, zmijovka, silrada). Like Alumna, except in 1 cell: noun pl.
    declension!("silrada", 4, "~a", like Alumna;
    noun: _, "~y";
    ),
    // Chous: 4 words (medimnus, chous, gemmun, khous). Like Man, except in 1 cell: noun sg.
    declension!("chous", 4, "~u~", like Man;
    noun: "~u~", _;
    ),
    // Wunderkind: 3 words (lied, wunderkind, tagelied). Like Line, except in 1 cell: noun pl.
    declension!("wunderkind", 3, "~", like Line;
    noun: _, "~er";
    ),
    // Rebetiko: 3 words (braccio, rebetiko, filakto). Like Medium, except in 1 cell: noun sg.
    declension!("rebetiko", 3, "~o", like Medium;
    noun: "~o", _;
    ),
    // Krone: 3 words (krone, tengwa, kransekage). Like Line, except in 1 cell: noun pl.
    declension!("krone", 3, "~", like Line;
    noun: _, "~r";
    ),
    // Muhajir: 3 words (muhajir, mujahid, mujahed). Like Line, except in 1 cell: noun pl.
    declension!("muhajir", 3, "~", like Line;
    noun: _, "~een";
    ),
    // Heemraad: 3 words (raad, heemraad, reaal). Like Man, except in 1 cell: noun pl.
    declension!("heemraad", 3, "~a~", like Man;
    noun: _, "~~en|~a~s";
    ),
    // Apikoros: 3 words (apikoros, poritz, epikoros). Like Melamed, except in 1 cell: noun sg.
    declension!("apikoros", 3, "~o~", like Melamed;
    noun: "~o~", _;
    ),
    // Dars: 3 words (fals, hadd, dars).
    declension!("dars", 3, "~a~~";
    noun: "~a~~", "~u~u~";
    ),
    // Mosotho: 3 words (mosotho, motswana, mosarwa).
    declension!("mosotho", 3, "mo~";
    noun: "mo~", "ba~";
    ),
    // Colstaff: 3 words (bookstaff, colstaff, colestaff). Like Leaf, except in 1 cell: noun sg.
    declension!("colstaff", 3, "~ff", like Leaf;
    noun: "~ff", _;
    ),
    // Brocha: 3 words (brocha, drasha, tefila). Like Alumna, except in 1 cell: noun pl.
    declension!("brocha", 3, "~a", like Alumna;
    noun: _, "~os";
    ),
    // Seacatch: 3 words (holluschick, seecatch, seacatch). Like Line, except in 1 cell: noun pl.
    declension!("seacatch", 3, "~", like Line;
    noun: _, "~ie";
    ),
    // Valenok: 3 words (pirozhok, valenok, piroshok). Like Apikoros, except in 1 cell: noun pl.
    declension!("valenok", 3, "~o~", like Apikoros;
    noun: _, "~~i";
    ),
    // Forefemur: 3 words (forefemur, mesofemur, metafemur). Like Tripus, except in 1 cell: noun pl.
    declension!("forefemur", 3, "~u~", like Tripus;
    noun: _, "~o~a";
    ),
    // Ibhaca: 3 words (igqirha, itongo, ibhaca).
    declension!("ibhaca", 3, "i~";
    noun: "i~", "ama~";
    ),
    // Gadol: 3 words (naphil, gadol, saraph). Like Man, except in 1 cell: noun pl.
    declension!("gadol", 3, "~a~", like Man;
    noun: _, "~e~im";
    ),
];
