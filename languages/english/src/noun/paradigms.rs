//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the declension it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`: the form is the declension's it is like):
/// `noun` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     noun: $c0:tt, $c1:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                flexion::cell!($c0), "\t",
                flexion::cell!($c1), "\t",
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

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `line`: 195,352 words.
    Line,
    /// Like `body`: 12,168 words.
    Body,
    /// Like `wash`: 7,184 words.
    Wash,
    /// Like `silverfish`: 2,576 words.
    Silverfish,
    /// Like `man`: 1,731 words.
    Man,
    /// Like `synthesis`: 1,682 words.
    Synthesis,
    /// Like `medium`: 1,457 words.
    Medium,
    /// Like `focus`: 797 words.
    Focus,
    /// Like `aura`: 533 words.
    Aura,
    /// Like `lexicon`: 438 words.
    Lexicon,
    /// Like `apse`: 412 words.
    Apse,
    /// Like `folium`: 375 words.
    Folium,
    /// Like `tempo`: 222 words.
    Tempo,
    /// Like `skeleton`: 188 words.
    Skeleton,
    /// Like `gondola`: 185 words.
    Gondola,
    /// Like `glans`: 178 words.
    Glans,
    /// Like `omphalos`: 172 words.
    Omphalos,
    /// Like `matrix`: 155 words.
    Matrix,
    /// Like `virus`: 144 words.
    Virus,
    /// Like `foreperson`: 144 words.
    Foreperson,
    /// Like `stigma`: 114 words.
    Stigma,
    /// Like `leaf`: 98 words.
    Leaf,
    /// Like `tasto`: 96 words.
    Tasto,
    /// Like `separatrix`: 95 words.
    Separatrix,
    /// Like `madrich`: 77 words.
    Madrich,
    /// Like `life`: 74 words.
    Life,
    /// Like `dodman`: 72 words.
    Dodman,
    /// Like `padrone`: 67 words.
    Padrone,
    /// Like `dry`: 54 words.
    Dry,
    /// Like `pecten`: 51 words.
    Pecten,
    /// Like `foot`: 48 words.
    Foot,
    /// Like `apex`: 41 words.
    Apex,
    /// Like `turf`: 41 words.
    Turf,
    /// Like `atlas`: 39 words.
    Atlas,
    /// Like `aliyah`: 38 words.
    Aliyah,
    /// Like `child`: 36 words.
    Child,
    /// Like `blin`: 35 words.
    Blin,
    /// Like `tekufah`: 35 words.
    Tekufah,
    /// Like `mouse`: 33 words.
    Mouse,
    /// Like `index`: 30 words.
    Index,
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
    /// Like `apsis`: 24 words.
    Apsis,
    /// Like `jynx`: 24 words.
    Jynx,
    /// Like `quiz`: 22 words.
    Quiz,
    /// Like `chamois`: 21 words.
    Chamois,
    /// Like `alumna`: 20 words.
    Alumna,
    /// Like `exotic`: 20 words.
    Exotic,
    /// Like `lira`: 20 words.
    Lira,
    /// Like `tableau`: 19 words.
    Tableau,
    /// Like `staff`: 18 words.
    Staff,
    /// Like `trolley`: 17 words.
    Trolley,
    /// Like `clavis`: 17 words.
    Clavis,
    /// Like `genus`: 17 words.
    Genus,
    /// Like `schutzstaffel`: 17 words.
    Schutzstaffel,
    /// Like `footful`: 17 words.
    Footful,
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
    /// Like `ply`: 11 words.
    Ply,
    /// Like `toxotes`: 11 words.
    Toxotes,
    /// Like `bellyful`: 11 words.
    Bellyful,
    /// Like `ordo`: 10 words.
    Ordo,
    /// Like `amban`: 9 words.
    Amban,
    /// Like `godi`: 8 words.
    Godi,
    /// Like `vort`: 8 words.
    Vort,
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
    /// Like `abies`: 5 words.
    Abies,
    /// Like `asparagus`: 5 words.
    Asparagus,
    /// Like `speso`: 5 words.
    Speso,
    /// Like `tupilak`: 5 words.
    Tupilak,
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
    /// Like `seder`: 4 words.
    Seder,
    /// Like `hakim`: 4 words.
    Hakim,
    /// Like `menorah`: 4 words.
    Menorah,
    /// Like `ceras`: 4 words.
    Ceras,
    /// Like `bocal`: 4 words.
    Bocal,
    /// Like `bolgia`: 4 words.
    Bolgia,
    /// Like `knyfe`: 4 words.
    Knyfe,
    /// Like `mzee`: 4 words.
    Mzee,
    /// Like `tripus`: 4 words.
    Tripus,
    /// Like `silrada`: 4 words.
    Silrada,
    /// Like `hawzah`: 4 words.
    Hawzah,
    /// Like `chous`: 4 words.
    Chous,
    /// Like `wunderkind`: 3 words.
    Wunderkind,
    /// Like `caput`: 3 words.
    Caput,
    /// Like `rebetiko`: 3 words.
    Rebetiko,
    /// Like `palea`: 3 words.
    Palea,
    /// Like `ibis`: 3 words.
    Ibis,
    /// Like `biceps`: 3 words.
    Biceps,
    /// Like `dromos`: 3 words.
    Dromos,
    /// Like `krone`: 3 words.
    Krone,
    /// Like `muhajir`: 3 words.
    Muhajir,
    /// Like `chochem`: 3 words.
    Chochem,
    /// Like `heemraad`: 3 words.
    Heemraad,
    /// Like `dghajsa`: 3 words.
    Dghajsa,
    /// Like `agaz`: 3 words.
    Agaz,
    /// Like `apikoros`: 3 words.
    Apikoros,
    /// Like `dars`: 3 words.
    Dars,
    /// Like `isicoco`: 3 words.
    Isicoco,
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
    /// Like `khariji`: 3 words.
    Khariji,
    /// Like `ibhaca`: 3 words.
    Ibhaca,
    /// Like `gadol`: 3 words.
    Gadol,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 130] = [
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
        Declension::Apse,
        Declension::Folium,
        Declension::Tempo,
        Declension::Skeleton,
        Declension::Gondola,
        Declension::Glans,
        Declension::Omphalos,
        Declension::Matrix,
        Declension::Virus,
        Declension::Foreperson,
        Declension::Stigma,
        Declension::Leaf,
        Declension::Tasto,
        Declension::Separatrix,
        Declension::Madrich,
        Declension::Life,
        Declension::Dodman,
        Declension::Padrone,
        Declension::Dry,
        Declension::Pecten,
        Declension::Foot,
        Declension::Apex,
        Declension::Turf,
        Declension::Atlas,
        Declension::Aliyah,
        Declension::Child,
        Declension::Blin,
        Declension::Tekufah,
        Declension::Mouse,
        Declension::Index,
        Declension::Metazoan,
        Declension::Mage,
        Declension::Person,
        Declension::Abendmusik,
        Declension::Eros,
        Declension::Apsis,
        Declension::Jynx,
        Declension::Quiz,
        Declension::Chamois,
        Declension::Alumna,
        Declension::Exotic,
        Declension::Lira,
        Declension::Tableau,
        Declension::Staff,
        Declension::Trolley,
        Declension::Clavis,
        Declension::Genus,
        Declension::Schutzstaffel,
        Declension::Footful,
        Declension::Bitumen,
        Declension::Pedale,
        Declension::Pocketful,
        Declension::Umbo,
        Declension::Mitzva,
        Declension::Bes,
        Declension::Stirps,
        Declension::Money,
        Declension::Ply,
        Declension::Toxotes,
        Declension::Bellyful,
        Declension::Ordo,
        Declension::Amban,
        Declension::Godi,
        Declension::Vort,
        Declension::Intaglio,
        Declension::Kotlet,
        Declension::Melamed,
        Declension::Fomes,
        Declension::Asclepiades,
        Declension::Kore,
        Declension::Kylix,
        Declension::Krouzek,
        Declension::Notzri,
        Declension::Abies,
        Declension::Asparagus,
        Declension::Speso,
        Declension::Tupilak,
        Declension::Vomitus,
        Declension::Formans,
        Declension::Perek,
        Declension::Vraka,
        Declension::Corpus,
        Declension::Fondaco,
        Declension::Seder,
        Declension::Hakim,
        Declension::Menorah,
        Declension::Ceras,
        Declension::Bocal,
        Declension::Bolgia,
        Declension::Knyfe,
        Declension::Mzee,
        Declension::Tripus,
        Declension::Silrada,
        Declension::Hawzah,
        Declension::Chous,
        Declension::Wunderkind,
        Declension::Caput,
        Declension::Rebetiko,
        Declension::Palea,
        Declension::Ibis,
        Declension::Biceps,
        Declension::Dromos,
        Declension::Krone,
        Declension::Muhajir,
        Declension::Chochem,
        Declension::Heemraad,
        Declension::Dghajsa,
        Declension::Agaz,
        Declension::Apikoros,
        Declension::Dars,
        Declension::Isicoco,
        Declension::Mosotho,
        Declension::Colstaff,
        Declension::Brocha,
        Declension::Seacatch,
        Declension::Valenok,
        Declension::Forefemur,
        Declension::Khariji,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 130] = [
    // Line: 195,352 words (free, encyclopedia, portmanteau, encyclopaedia, cat, word).
    declension!("line", 195352, "~";
        noun: "~", "~s";
    ),
    // Body: 12,168 words (dictionary, lexicography, etymology, abbacy, abditory, abecedary).
    declension!("body", 12168, "~y";
        noun: "~y", "~ies";
    ),
    // Wash: 7,184 words (march, abbess, abjectness, ableness, abortiveness, abraxas). Like Line, except in 1 cell: noun pl.
    declension!("wash", 7184, "~", like Line;
        noun: _, "~es";
    ),
    // Silverfish: 2,576 words (semantics, craft, abatis, abatvoix, english, bpd). Like Line, except in 1 cell: noun pl.
    declension!("silverfish", 2576, "~", like Line;
        noun: _, "~";
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
    // Aura: 533 words (fabella, pupa, vertebra, abolla, familia, larva). Like Line, except in 1 cell: noun pl.
    declension!("aura", 533, "~", like Line;
        noun: _, "~e";
    ),
    // Lexicon: 438 words (hexahedron, tetrahedron, pyramidion, eidolon, polyhedron, lexicon). Like Medium, except in 1 cell: noun sg.
    declension!("lexicon", 438, "~on", like Medium;
        noun: "~on", _;
    ),
    // Apse: 412 words (handful, culdesac, nymphe, acne, apse, rabbi).
    declension!("apse", 412, "~~";
        noun: "~~", "~~s|~s~";
    ),
    // Folium: 375 words (asylum, plenum, polygonum, fulcrum, petroleum, promontorium). Like Medium, except in 1 cell: noun pl.
    declension!("folium", 375, "~um", like Medium;
        noun: _, "~ums|~a";
    ),
    // Tempo: 222 words (tempo, secondo, cicisbeo, generalissimo, castrato, pizzicato).
    declension!("tempo", 222, "~o";
        noun: "~o", "~os|~i";
    ),
    // Skeleton: 188 words (oxymoron, colon, epsilon, semicolon, omicron, upsilon). Like Lexicon, except in 1 cell: noun pl.
    declension!("skeleton", 188, "~on", like Lexicon;
        noun: _, "~ons|~a";
    ),
    // Gondola: 185 words (acciaccatura, aria, balalaika, mozzarella, lasagna, influenza).
    declension!("gondola", 185, "~a";
        noun: "~a", "~as";
    ),
    // Glans: 178 words (absis, pyramis, pes, enteritis, glans, encephalitis). Like Apse, except in 1 cell: noun pl.
    declension!("glans", 178, "~~", like Apse;
        noun: _, "~de~";
    ),
    // Omphalos: 172 words (mythos, ouroboros, eromenos, logos, kourotrophos, psogos). Like Focus, except in 1 cell: noun sg.
    declension!("omphalos", 172, "~s", like Focus;
        noun: "~s", _;
    ),
    // Matrix: 155 words (hallux, generatrix, matrix, appendix, calx, faux).
    declension!("matrix", 155, "~x";
        noun: "~x", "~ces";
    ),
    // Virus: 144 words (acanthus, citrus, octopus, phosphorus, foetus, onus). Like Focus, except in 1 cell: noun pl.
    declension!("virus", 144, "~us", like Focus;
        noun: _, "~uses|~i";
    ),
    // Foreperson: 144 words (chairperson, craftsperson, salesperson, spokesperson, tradesperson, anchorperson).
    declension!("foreperson", 144, "~rs~n";
        noun: "~rs~n", "~rs~ns|~~ple";
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
    // Separatrix: 95 words (phalanx, hyrax, sphinx, unix, calix, borax). Like Matrix, except in 1 cell: noun pl.
    declension!("separatrix", 95, "~x", like Matrix;
        noun: _, "~xes|~ces";
    ),
    // Madrich: 77 words (ars, kibbutz, hassid, minyan, zuz, gaon). Like Line, except in 1 cell: noun pl.
    declension!("madrich", 77, "~", like Line;
        noun: _, "~im";
    ),
    // Life: 74 words (wife, life, knife, alewife, jackknife, housewife).
    declension!("life", 74, "~f~";
        noun: "~f~", "~v~s";
    ),
    // Dodman: 72 words (madam, madame, dragoman, hetman, pitman, walkman). Like Man, except in 1 cell: noun pl.
    declension!("dodman", 72, "~a~", like Man;
        noun: _, "~a~s|~e~";
    ),
    // Padrone: 67 words (seme, simile, padre, facsimile, doge, evzone). Like Wash, except in 1 cell: noun sg.
    declension!("padrone", 67, "~e", like Wash;
        noun: "~e", _;
    ),
    // Dry: 54 words (why, dry, only, sammy, jimmy, andy). Like Body, except in 1 cell: noun pl.
    declension!("dry", 54, "~y", like Body;
        noun: _, "~ys|~ies";
    ),
    // Pecten: 51 words (abdomen, shekel, lumen, stamen, mater, acumen).
    declension!("pecten", 51, "~e~";
        noun: "~e~", "~e~s";
    ),
    // Foot: 48 words (tooth, foot, goose, tenderfoot, bucktooth, leadfoot).
    declension!("foot", 48, "~oo~";
        noun: "~oo~", "~ee~";
    ),
    // Apex: 41 words (apex, pollex, vertex, pontifex, vortex, codex).
    declension!("apex", 41, "~~x";
        noun: "~~x", "~ic~s";
    ),
    // Turf: 41 words (handkerchief, turf, roof, dwarf, oaf, grief). Like Leaf, except in 1 cell: noun pl.
    declension!("turf", 41, "~f", like Leaf;
        noun: _, "~fs|~ves";
    ),
    // Atlas: 39 words (atlas, clitoris, adonis, adenitis, aegis, pancreatitis). Like Apse, except in 1 cell: noun pl.
    declension!("atlas", 39, "~~", like Apse;
        noun: _, "~~es|~de~";
    ),
    // Aliyah: 38 words (chuppah, genizah, torah, kehillah, megillah, mezuzah).
    declension!("aliyah", 38, "~ah";
        noun: "~ah", "~ahs|~ot|~oth";
    ),
    // Child: 36 words (child, grandchild, lovechild, godchild, stepchild, schoolchild). Like Line, except in 1 cell: noun pl.
    declension!("child", 36, "~", like Line;
        noun: _, "~ren";
    ),
    // Blin: 35 words (sent, triumvir, garda, kroon, monsignor, hetaira). Like Line, except in 1 cell: noun pl.
    declension!("blin", 35, "~", like Line;
        noun: _, "~i";
    ),
    // Tekufah: 35 words (tzedakah, matzah, massebah, sufganiyah, petuhah, petuchah). Like Aliyah, except in 1 cell: noun pl.
    declension!("tekufah", 35, "~ah", like Aliyah;
        noun: _, "~ot";
    ),
    // Mouse: 33 words (mouse, dormouse, louse, woodlouse, titmouse, flittermouse).
    declension!("mouse", 33, "~ous~";
        noun: "~ous~", "~ic~";
    ),
    // Index: 30 words (index, duplex, murex, carnifex, narthex, cortex). Like Apex, except in 1 cell: noun pl.
    declension!("index", 30, "~~x", like Apex;
        noun: _, "~~xes|~ic~s";
    ),
    // Metazoan: 30 words (bryozoan, metazoan, protozoan, micrometazoan, cubozoan, hydrozoan).
    declension!("metazoan", 30, "~n";
        noun: "~n", "~|~ns";
    ),
    // Mage: 28 words (dilettante, duce, mage, genie, zampone, cognoscente). Like Focus, except in 1 cell: noun sg.
    declension!("mage", 28, "~e", like Focus;
        noun: "~e", _;
    ),
    // Person: 25 words (person, conperson, layperson, floorperson, linesperson, catperson). Like Foreperson, except in 1 cell: noun pl.
    declension!("person", 25, "~rs~n", like Foreperson;
        noun: _, "~~ple";
    ),
    // Abendmusik: 25 words (ox, ahlspiess, ubermensch, luftmensch, frau, herr). Like Line, except in 1 cell: noun pl.
    declension!("abendmusik", 25, "~", like Line;
        noun: _, "~en";
    ),
    // Eros: 24 words (pons, dens, eros, gens, paries, veritas). Like Apse, except in 1 cell: noun pl.
    declension!("eros", 24, "~~", like Apse;
        noun: _, "~te~";
    ),
    // Apsis: 24 words (apoapsis, periapsis, conjunctivitis, mastoiditis, gingivitis, tonsillitis). Like Synthesis, except in 1 cell: noun pl.
    declension!("apsis", 24, "~i~", like Synthesis;
        noun: _, "~ide~|~e~";
    ),
    // Jynx: 24 words (larynx, pharynx, coccyx, meninx, salpinx, grex). Like Matrix, except in 1 cell: noun pl.
    declension!("jynx", 24, "~x", like Matrix;
        noun: _, "~ges";
    ),
    // Quiz: 22 words (quiz, lez, fez, spaz, schnoz, whiz). Like Line, except in 1 cell: noun pl.
    declension!("quiz", 22, "~", like Line;
        noun: _, "~zes";
    ),
    // Chamois: 21 words (pancreas, chamois, bris, cosmos, demos, elvis). Like Omphalos, except in 1 cell: noun pl.
    declension!("chamois", 21, "~s", like Omphalos;
        noun: _, "~ses|~i";
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
    // Staff: 18 words (staff, backstaff, flagstaff, distaff, quarterstaff, pikestaff).
    declension!("staff", 18, "~ff";
        noun: "~ff", "~ffs|~ves";
    ),
    // Trolley: 17 words (smiley, whiskey, coney, phoney, slavey, flunkey).
    declension!("trolley", 17, "~~y";
        noun: "~~y", "~~ys|~i~s";
    ),
    // Clavis: 17 words (penis, clavis, appendicitis, pelvis, dermatitis, proboscis). Like Synthesis, except in 1 cell: noun pl.
    declension!("clavis", 17, "~i~", like Synthesis;
        noun: _, "~i~es|~e~";
    ),
    // Genus: 17 words (genus, vulnus, latus, ulcus, viscus, nothogenus). Like Focus, except in 1 cell: noun pl.
    declension!("genus", 17, "~us", like Focus;
        noun: _, "~era";
    ),
    // Schutzstaffel: 17 words (ee, pease, frikadelle, kartoffel, ursprache, kanone). Like Line, except in 1 cell: noun pl.
    declension!("schutzstaffel", 17, "~", like Line;
        noun: _, "~n";
    ),
    // Footful: 17 words (blackfoot, webfoot, crowfoot, coltsfoot, flatfoot, spadefoot). Like Foot, except in 1 cell: noun pl.
    declension!("footful", 17, "~oo~", like Foot;
        noun: _, "~oo~s|~ee~";
    ),
    // Bitumen: 16 words (nomen, rumen, numen, bitumen, crimen, foramen). Like Pecten, except in 1 cell: noun pl.
    declension!("bitumen", 16, "~e~", like Pecten;
        noun: _, "~i~a";
    ),
    // Pedale: 14 words (quale, centrale, insigne, memorabile, marginale, radiale). Like Padrone, except in 1 cell: noun pl.
    declension!("pedale", 14, "~e", like Padrone;
        noun: _, "~ia";
    ),
    // Pocketful: 14 words (passerby, pocketful, tubful, courtmartial, swagful, courtful). Like Apse, except in 1 cell: noun pl.
    declension!("pocketful", 14, "~~", like Apse;
        noun: _, "~s~";
    ),
    // Umbo: 13 words (umbo, confessio, comedo, regio, divisio, pernio). Like Line, except in 1 cell: noun pl.
    declension!("umbo", 13, "~", like Line;
        noun: _, "~nes";
    ),
    // Mitzva: 13 words (mitzva, tzedaka, halacha, moshava, matza, sefira). Like Gondola, except in 1 cell: noun pl.
    declension!("mitzva", 13, "~a", like Gondola;
        noun: _, "~ot";
    ),
    // Bes: 12 words (es, dis, bes, lis, tas, dorcas). Like Line, except in 1 cell: noun pl.
    declension!("bes", 12, "~", like Line;
        noun: _, "~ses";
    ),
    // Stirps: 12 words (urbs, polis, erinys, labrys, stirps, prytanis). Like Man, except in 1 cell: noun sg.
    declension!("stirps", 12, "~~", like Man;
        noun: "~~", _;
    ),
    // Money: 11 words (money, covey, fogey, curtsey, jarvey, cartooney). Like Trolley, except in 1 cell: noun pl.
    declension!("money", 11, "~~y", like Trolley;
        noun: _, "~i~s|~~ys";
    ),
    // Ply: 11 words (ply, malagasy, zloty, romany, military, goby). Like Body, except in 1 cell: noun pl.
    declension!("ply", 11, "~y", like Body;
        noun: _, "~y|~ies";
    ),
    // Toxotes: 11 words (erastes, toxotes, agonothetes, metretes, protostates, bouleutes).
    declension!("toxotes", 11, "~es";
        noun: "~es", "~ai";
    ),
    // Bellyful: 11 words (bellyful, lorryful, skyful, styful, bodyful, galleryful).
    declension!("bellyful", 11, "~y~";
        noun: "~y~", "~y~s|~ies~";
    ),
    // Ordo: 10 words (ordo, imago, cardo, propago, suffrago, margo). Like Tempo, except in 1 cell: noun pl.
    declension!("ordo", 10, "~o", like Tempo;
        noun: _, "~ines";
    ),
    // Amban: 9 words (microsporidian, larin, bilaterian, moneran, amban, haemosporidian). Like Metazoan, except in 1 cell: noun pl.
    declension!("amban", 9, "~n", like Metazoan;
        noun: _, "~ns|~";
    ),
    // Godi: 8 words (yahudi, jinni, godi, tortellini, jihadi, jehadi).
    declension!("godi", 8, "~i";
        noun: "~i", "~is";
    ),
    // Vort: 8 words (morena, vort, mokoro, lubok, chok, vershok).
    declension!("vort", 8, "~o~";
        noun: "~o~", "~o~s";
    ),
    // Intaglio: 7 words (intaglio, finocchio, passaggio, libeccio, fenochio, fenocchio). Like Silverfish, except in 1 cell: noun sg.
    declension!("intaglio", 7, "~o", like Silverfish;
        noun: "~o", _;
    ),
    // Kotlet: 7 words (kotlet, kabanos, varenyk, nalysnyk, nalesnyk, syrnyk). Like Line, except in 1 cell: noun pl.
    declension!("kotlet", 7, "~", like Line;
        noun: _, "~y";
    ),
    // Melamed: 7 words (kofer, melamed, yored, shomer, poseq, misnaged). Like Pecten, except in 1 cell: noun pl.
    declension!("melamed", 7, "~e~", like Pecten;
        noun: _, "~~im";
    ),
    // Fomes: 6 words (eques, stipes, limes, termes, fomes, vicecomes). Like Apse, except in 1 cell: noun pl.
    declension!("fomes", 6, "~~", like Apse;
        noun: _, "~it~";
    ),
    // Asclepiades: 6 words (hermes, cerastes, psaltes, asclepiades, protopsaltis, mystes).
    declension!("asclepiades", 6, "~~s";
        noun: "~~s", "~a~";
    ),
    // Kore: 6 words (kore, arche, spithame, hekte, oinokhoe, drakhme). Like Padrone, except in 1 cell: noun pl.
    declension!("kore", 6, "~e", like Padrone;
        noun: _, "~ai";
    ),
    // Kylix: 6 words (phylax, kylix, somatophylax, nomophylax, larnax, khoinix). Like Matrix, except in 1 cell: noun pl.
    declension!("kylix", 6, "~x", like Matrix;
        noun: _, "~kes";
    ),
    // Krouzek: 6 words (chervonets, tchervonets, strelets, popovets, bezpopovets, krouzek). Like Pecten, except in 1 cell: noun pl.
    declension!("krouzek", 6, "~e~", like Pecten;
        noun: _, "~~y";
    ),
    // Notzri: 5 words (ashkenazi, illui, losse, ashkephardi, notzri). Like Line, except in 1 cell: noun pl.
    declension!("notzri", 5, "~", like Line;
        noun: _, "~m";
    ),
    // Abies: 5 words (cyclops, stapes, sapiens, abies, prophases). Like Apse, except in 1 cell: noun pl.
    declension!("abies", 5, "~~", like Apse;
        noun: _, "~~";
    ),
    // Asparagus: 5 words (asparagus, quadriporticus, actus, fluctus, zoodetritus). Like Focus, except in 1 cell: noun pl.
    declension!("asparagus", 5, "~us", like Focus;
        noun: _, "~us|~i";
    ),
    // Speso: 5 words (stelo, speso, denaskulo, spesmilo, esperantido). Like Line, except in 1 cell: noun pl.
    declension!("speso", 5, "~", like Line;
        noun: _, "~j";
    ),
    // Tupilak: 5 words (inuk, umiak, inuksuk, tupilak, lisowczyk).
    declension!("tupilak", 5, "~k";
        noun: "~k", "~ks";
    ),
    // Vomitus: 5 words (vomitus, generalissimus, pangasius, cytofilius, systylius). Like Medium, except in 1 cell: noun sg.
    declension!("vomitus", 5, "~us", like Medium;
        noun: "~us", _;
    ),
    // Formans: 5 words (definiens, explanans, accidens, explicans, formans). Like Omphalos, except in 1 cell: noun pl.
    declension!("formans", 5, "~s", like Omphalos;
        noun: _, "~tia";
    ),
    // Perek: 5 words (sefer, perek, pesher, pereq, shoresh). Like Pecten, except in 1 cell: noun pl.
    declension!("perek", 5, "~e~", like Pecten;
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
    // Fondaco: 4 words (arco, fondaco, attacco, intrico). Like Tempo, except in 1 cell: noun pl.
    declension!("fondaco", 4, "~o", like Tempo;
        noun: _, "~hi";
    ),
    // Seder: 4 words (seder, heder, cheder, metapelet).
    declension!("seder", 4, "~e~e~";
        noun: "~e~e~", "~e~e~s";
    ),
    // Hakim: 4 words (hanif, sharif, hakim, zahid).
    declension!("hakim", 4, "~a~i~";
        noun: "~a~i~", "~a~i~s";
    ),
    // Menorah: 4 words (menorah, sephirah, neginah, sefirah). Like Aliyah, except in 1 cell: noun pl.
    declension!("menorah", 4, "~ah", like Aliyah;
        noun: _, "~oth";
    ),
    // Ceras: 4 words (teras, ceras, hemiteras, orthoceras). Like Omphalos, except in 1 cell: noun pl.
    declension!("ceras", 4, "~s", like Omphalos;
        noun: _, "~ta";
    ),
    // Bocal: 4 words (metical, bocal, crenel, sanguicel).
    declension!("bocal", 4, "~l";
        noun: "~l", "~ls";
    ),
    // Bolgia: 4 words (loggia, bolgia, paracusia, coenesthesia).
    declension!("bolgia", 4, "~ia";
        noun: "~ia", "~ias";
    ),
    // Knyfe: 4 words (lowlife, huswife, knyfe, spife). Like Life, except in 1 cell: noun pl.
    declension!("knyfe", 4, "~f~", like Life;
        noun: _, "~f~s|~v~s";
    ),
    // Mzee: 4 words (mwalimu, mzee, mzungu, mganga).
    declension!("mzee", 4, "m~";
        noun: "m~", "m~s|wa~";
    ),
    // Tripus: 4 words (pleiopus, pleopus, tripus, lagopus).
    declension!("tripus", 4, "~u~";
        noun: "~u~", "~ode~";
    ),
    // Silrada: 4 words (barma, kotleta, zmijovka, silrada). Like Gondola, except in 1 cell: noun pl.
    declension!("silrada", 4, "~a", like Gondola;
        noun: _, "~y";
    ),
    // Hawzah: 4 words (muslimah, muhafazah, wilayah, hawzah).
    declension!("hawzah", 4, "~h";
        noun: "~h", "~hs|~t";
    ),
    // Chous: 4 words (medimnus, chous, gemmun, khous). Like Man, except in 1 cell: noun sg.
    declension!("chous", 4, "~u~", like Man;
        noun: "~u~", _;
    ),
    // Wunderkind: 3 words (lied, wunderkind, tagelied). Like Line, except in 1 cell: noun pl.
    declension!("wunderkind", 3, "~", like Line;
        noun: _, "~er";
    ),
    // Caput: 3 words (femur, caput, omuramba). Like Tripus, except in 1 cell: noun pl.
    declension!("caput", 3, "~u~", like Tripus;
        noun: _, "~u~s";
    ),
    // Rebetiko: 3 words (braccio, rebetiko, filakto). Like Medium, except in 1 cell: noun sg.
    declension!("rebetiko", 3, "~o", like Medium;
        noun: "~o", _;
    ),
    // Palea: 3 words (longa, palea, vulsella). Like Gondola, except in 1 cell: noun pl.
    declension!("palea", 3, "~a", like Gondola;
        noun: _, "~ae";
    ),
    // Ibis: 3 words (ibis, precis, rhipsalis). Like Synthesis, except in 1 cell: noun pl.
    declension!("ibis", 3, "~i~", like Synthesis;
        noun: _, "~i~";
    ),
    // Biceps: 3 words (biceps, forceps, gonoforceps).
    declension!("biceps", 3, "~e~~";
        noun: "~e~~", "~e~~";
    ),
    // Dromos: 3 words (temenos, dromos, prochoos).
    declension!("dromos", 3, "~os";
        noun: "~os", "~oi|~i";
    ),
    // Krone: 3 words (krone, tengwa, kransekage). Like Line, except in 1 cell: noun pl.
    declension!("krone", 3, "~", like Line;
        noun: _, "~r";
    ),
    // Muhajir: 3 words (muhajir, mujahid, mujahed). Like Line, except in 1 cell: noun pl.
    declension!("muhajir", 3, "~", like Line;
        noun: _, "~een";
    ),
    // Chochem: 3 words (karbovanets, lishenets, chochem). Like Man, except in 1 cell: noun sg.
    declension!("chochem", 3, "~e~", like Man;
        noun: "~e~", _;
    ),
    // Heemraad: 3 words (raad, heemraad, reaal). Like Man, except in 1 cell: noun pl.
    declension!("heemraad", 3, "~a~", like Man;
        noun: _, "~~en|~a~s";
    ),
    // Dghajsa: 3 words (kopiyka, dghajsa, kopiika).
    declension!("dghajsa", 3, "~~a";
        noun: "~~a", "~~as";
    ),
    // Agaz: 3 words (arroz, ajimez, agaz).
    declension!("agaz", 3, "~z";
        noun: "~z", "~zes|~ces";
    ),
    // Apikoros: 3 words (apikoros, poritz, epikoros). Like Vort, except in 1 cell: noun pl.
    declension!("apikoros", 3, "~o~", like Vort;
        noun: _, "~~im";
    ),
    // Dars: 3 words (fals, hadd, dars).
    declension!("dars", 3, "~a~~";
        noun: "~a~~", "~u~u~";
    ),
    // Isicoco: 3 words (isibongo, isangoma, isicoco).
    declension!("isicoco", 3, "~s~";
        noun: "~s~", "~s~s|~z~";
    ),
    // Mosotho: 3 words (mosotho, motswana, mosarwa).
    declension!("mosotho", 3, "mo~";
        noun: "mo~", "ba~";
    ),
    // Colstaff: 3 words (bookstaff, colstaff, colestaff). Like Leaf, except in 1 cell: noun sg.
    declension!("colstaff", 3, "~ff", like Leaf;
        noun: "~ff", _;
    ),
    // Brocha: 3 words (brocha, drasha, tefila). Like Gondola, except in 1 cell: noun pl.
    declension!("brocha", 3, "~a", like Gondola;
        noun: _, "~os";
    ),
    // Seacatch: 3 words (holluschick, seecatch, seacatch). Like Line, except in 1 cell: noun pl.
    declension!("seacatch", 3, "~", like Line;
        noun: _, "~ie";
    ),
    // Valenok: 3 words (pirozhok, valenok, piroshok). Like Vort, except in 1 cell: noun pl.
    declension!("valenok", 3, "~o~", like Vort;
        noun: _, "~~i";
    ),
    // Forefemur: 3 words (forefemur, mesofemur, metafemur). Like Tripus, except in 1 cell: noun pl.
    declension!("forefemur", 3, "~u~", like Tripus;
        noun: _, "~o~a";
    ),
    // Khariji: 3 words (nasibi, rafidi, khariji).
    declension!("khariji", 3, "~~i";
        noun: "~~i", "~~is|~wa~";
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
