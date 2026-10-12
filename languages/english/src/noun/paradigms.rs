//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `noun` (sg, pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     noun: $c0:tt, $c1:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0), flexion::form!($c1)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `head`: 69,119 words.
    Head,
    /// Like `body`: 4,461 words.
    Body,
    /// Like `box`: 2,810 words.
    Box,
    /// Like `fish`: 1,887 words.
    Fish,
    /// Like `synthesis`: 779 words.
    Synthesis,
    /// Like `culum`: 738 words.
    Culum,
    /// Like `ilicus`: 413 words.
    Ilicus,
    /// Like `aura`: 378 words.
    Aura,
    /// Like `man`: 353 words.
    Man,
    /// Like `lexicon`: 280 words.
    Lexicon,
    /// Like `abulum`: 176 words.
    Abulum,
    /// Like `phalos`: 147 words.
    Phalos,
    /// Like `foreperson`: 125 words.
    Foreperson,
    /// Like `glans`: 119 words.
    Glans,
    /// Like `mpo`: 119 words.
    Mpo,
    /// Like `dola`: 110 words.
    Dola,
    /// Like `eton`: 103 words.
    Eton,
    /// Like `apse`: 95 words.
    Apse,
    /// Like `virus`: 78 words.
    Virus,
    /// Like `stigma`: 73 words.
    Stigma,
    /// Like `orno`: 72 words.
    Orno,
    /// Like `rix`: 64 words.
    Rix,
    /// Like `anx`: 58 words.
    Anx,
    /// Like `goy`: 57 words.
    Goy,
    /// Like `dodman`: 35 words.
    Dodman,
    /// Like `dry`: 30 words.
    Dry,
    /// Like `atlas`: 30 words.
    Atlas,
    /// Like `turf`: 30 words.
    Turf,
    /// Like `apex`: 29 words.
    Apex,
    /// Like `simile`: 27 words.
    Simile,
    /// Like `aira`: 26 words.
    Aira,
    /// Like `leaf`: 25 words.
    Leaf,
    /// Like `kufah`: 25 words.
    Kufah,
    /// Like `mage`: 24 words.
    Mage,
    /// Like `zvah`: 24 words.
    Zvah,
    /// Like `endmusik`: 23 words.
    Endmusik,
    /// Like `whiz`: 20 words.
    Whiz,
    /// Like `gmen`: 20 words.
    Gmen,
    /// Like `chamois`: 19 words.
    Chamois,
    /// Like `eros`: 18 words.
    Eros,
    /// Like `lira`: 18 words.
    Lira,
    /// Like `apsis`: 18 words.
    Apsis,
    /// Like `tableau`: 17 words.
    Tableau,
    /// Like `exotic`: 17 words.
    Exotic,
    /// Like `jynx`: 16 words.
    Jynx,
    /// Like `utzstaffel`: 15 words.
    Utzstaffel,
    /// Like `alumna`: 14 words.
    Alumna,
    /// Like `dogtooth`: 14 words.
    Dogtooth,
    /// Like `pedale`: 14 words.
    Pedale,
    /// Like `foot`: 13 words.
    Foot,
    /// Like `mouse`: 12 words.
    Mouse,
    /// Like `bes`: 12 words.
    Bes,
    /// Like `zva`: 12 words.
    Zva,
    /// Like `stirps`: 11 words.
    Stirps,
    /// Like `boscis`: 10 words.
    Boscis,
    /// Like `ilex`: 10 words.
    Ilex,
    /// Like `umbo`: 9 words.
    Umbo,
    /// Like `ozoan`: 9 words.
    Ozoan,
    /// Like `tes`: 9 words.
    Tes,
    /// Like `life`: 8 words.
    Life,
    /// Like `ply`: 7 words.
    Ply,
    /// Like `child`: 6 words.
    Child,
    /// Like `fomes`: 6 words.
    Fomes,
    /// Like `kte`: 6 words.
    Kte,
    /// Like `anos`: 6 words.
    Anos,
    /// Like `money`: 5 words.
    Money,
    /// Like `ilak`: 5 words.
    Ilak,
    /// Like `rimen`: 5 words.
    Rimen,
    /// Like `godi`: 5 words.
    Godi,
    /// Like `lets`: 5 words.
    Lets,
    /// Like `anans`: 5 words.
    Anans,
    /// Like `melamed`: 5 words.
    Melamed,
    /// Like `bedroomful`: 5 words.
    Bedroomful,
    /// Like `smilo`: 4 words.
    Smilo,
    /// Like `ops`: 4 words.
    Ops,
    /// Like `firah`: 4 words.
    Firah,
    /// Like `aragus`: 4 words.
    Aragus,
    /// Like `aglio`: 4 words.
    Aglio,
    /// Like `khol`: 4 words.
    Khol,
    /// Like `mzee`: 4 words.
    Mzee,
    /// Like `tripus`: 4 words.
    Tripus,
    /// Like `nada`: 4 words.
    Nada,
    /// Like `nus`: 4 words.
    Nus,
    /// Like `zri`: 4 words.
    Zri,
    /// Like `filius`: 4 words.
    Filius,
    /// Like `nderkind`: 3 words.
    Nderkind,
    /// Like `arco`: 3 words.
    Arco,
    /// Like `ibis`: 3 words.
    Ibis,
    /// Like `dromos`: 3 words.
    Dromos,
    /// Like `ordo`: 3 words.
    Ordo,
    /// Like `lepiades`: 3 words.
    Lepiades,
    /// Like `lowlife`: 3 words.
    Lowlife,
    /// Like `mraad`: 3 words.
    Mraad,
    /// Like `dars`: 3 words.
    Dars,
    /// Like `larin`: 3 words.
    Larin,
    /// Like `mosotho`: 3 words.
    Mosotho,
    /// Like `colstaff`: 3 words.
    Colstaff,
    /// Like `silrada`: 3 words.
    Silrada,
    /// Like `pereq`: 3 words.
    Pereq,
    /// Like `forefemur`: 3 words.
    Forefemur,
    /// Like `iika`: 3 words.
    Iika,
    /// Like `luschick`: 3 words.
    Luschick,
    /// Like `chochem`: 3 words.
    Chochem,
    /// Like `ibhaca`: 3 words.
    Ibhaca,
    /// Like `gadol`: 3 words.
    Gadol,
    /// Like `roz`: 3 words.
    Roz,
    /// Like `ajir`: 3 words.
    Ajir,
    /// Like `qya`: 3 words.
    Qya,
    /// Like `nok`: 3 words.
    Nok,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 109] = [
        Declension::Head,
        Declension::Body,
        Declension::Box,
        Declension::Fish,
        Declension::Synthesis,
        Declension::Culum,
        Declension::Ilicus,
        Declension::Aura,
        Declension::Man,
        Declension::Lexicon,
        Declension::Abulum,
        Declension::Phalos,
        Declension::Foreperson,
        Declension::Glans,
        Declension::Mpo,
        Declension::Dola,
        Declension::Eton,
        Declension::Apse,
        Declension::Virus,
        Declension::Stigma,
        Declension::Orno,
        Declension::Rix,
        Declension::Anx,
        Declension::Goy,
        Declension::Dodman,
        Declension::Dry,
        Declension::Atlas,
        Declension::Turf,
        Declension::Apex,
        Declension::Simile,
        Declension::Aira,
        Declension::Leaf,
        Declension::Kufah,
        Declension::Mage,
        Declension::Zvah,
        Declension::Endmusik,
        Declension::Whiz,
        Declension::Gmen,
        Declension::Chamois,
        Declension::Eros,
        Declension::Lira,
        Declension::Apsis,
        Declension::Tableau,
        Declension::Exotic,
        Declension::Jynx,
        Declension::Utzstaffel,
        Declension::Alumna,
        Declension::Dogtooth,
        Declension::Pedale,
        Declension::Foot,
        Declension::Mouse,
        Declension::Bes,
        Declension::Zva,
        Declension::Stirps,
        Declension::Boscis,
        Declension::Ilex,
        Declension::Umbo,
        Declension::Ozoan,
        Declension::Tes,
        Declension::Life,
        Declension::Ply,
        Declension::Child,
        Declension::Fomes,
        Declension::Kte,
        Declension::Anos,
        Declension::Money,
        Declension::Ilak,
        Declension::Rimen,
        Declension::Godi,
        Declension::Lets,
        Declension::Anans,
        Declension::Melamed,
        Declension::Bedroomful,
        Declension::Smilo,
        Declension::Ops,
        Declension::Firah,
        Declension::Aragus,
        Declension::Aglio,
        Declension::Khol,
        Declension::Mzee,
        Declension::Tripus,
        Declension::Nada,
        Declension::Nus,
        Declension::Zri,
        Declension::Filius,
        Declension::Nderkind,
        Declension::Arco,
        Declension::Ibis,
        Declension::Dromos,
        Declension::Ordo,
        Declension::Lepiades,
        Declension::Lowlife,
        Declension::Mraad,
        Declension::Dars,
        Declension::Larin,
        Declension::Mosotho,
        Declension::Colstaff,
        Declension::Silrada,
        Declension::Pereq,
        Declension::Forefemur,
        Declension::Iika,
        Declension::Luschick,
        Declension::Chochem,
        Declension::Ibhaca,
        Declension::Gadol,
        Declension::Roz,
        Declension::Ajir,
        Declension::Qya,
        Declension::Nok,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 109] = [
    // Head: 69,119 words (free, cat, word, book, gdp, pond).
    declension!("head", 69119, "~";
        noun: "~", "~s";
    ),
    // Body: 4,461 words (sky, early, army, baby, laundry, weekly).
    declension!("body", 4461, "~y";
        noun: "~y", "~ies";
    ),
    // Box: 2,810 words (march, lens, o, os, i, six).
    declension!("box", 2810, "~";
        noun: "~", "~es";
    ),
    // Fish: 1,887 words (craft, english, bpd, bbl, irish, cod).
    declension!("fish", 1887, "~";
        noun: "~", "~";
    ),
    // Synthesis: 779 words (gnosis, acidosis, analysis, basis, crisis, osmosis).
    declension!("synthesis", 779, "~i~";
        noun: "~i~", "~e~";
    ),
    // Culum: 738 words (phylum, maximum, minimum, rosetum, stratum, septum).
    declension!("culum", 738, "~um";
        noun: "~um", "~a";
    ),
    // Ilicus: 413 words (acarus, hamulus, calculus, torus, incubus, oculus).
    declension!("ilicus", 413, "~us";
        noun: "~us", "~i";
    ),
    // Aura: 378 words (pupa, familia, larva, pyrena, agora, nebula).
    declension!("aura", 378, "~";
        noun: "~", "~e";
    ),
    // Man: 353 words (man, dangerman, mangas, fyrdman, doggerman, hordesman).
    declension!("man", 353, "~a~";
        noun: "~a~", "~e~";
    ),
    // Lexicon: 280 words (eidolon, lexicon, perihelion, lepton, taxon, ganglion).
    declension!("lexicon", 280, "~on";
        noun: "~on", "~a";
    ),
    // Abulum: 176 words (epithelium, sputum, epithalamium, subminimum, suboptimum, intermedium).
    declension!("abulum", 176, "~um";
        noun: "~um", "~ums|~a";
    ),
    // Phalos: 147 words (logos, barbitos, kouros, thymos, tholos, lats).
    declension!("phalos", 147, "~s";
        noun: "~s", "~i";
    ),
    // Foreperson: 125 words (chairperson, craftsperson, salesperson, spokesperson, tradesperson, anchorperson).
    declension!("foreperson", 125, "~rs~n";
        noun: "~rs~n", "~rs~ns|~~ple";
    ),
    // Glans: 119 words (absis, pes, enteritis, glans, encephalitis, pharyngitis).
    declension!("glans", 119, "~~";
        noun: "~~", "~de~";
    ),
    // Mpo: 119 words (alto, solo, selo, sforzando, mulo, sforzato).
    declension!("mpo", 119, "~o";
        noun: "~o", "~os|~i";
    ),
    // Dola: 110 words (aria, masala, kuna, pita, diva, vila).
    declension!("dola", 110, "~a";
        noun: "~a", "~as";
    ),
    // Eton: 103 words (cuboctahedron, icositetrahedron, lithopedion, hendecahedron, hydrozoon, tetrahexahedron).
    declension!("eton", 103, "~on";
        noun: "~on", "~ons|~a";
    ),
    // Apse: 95 words (acne, apse, lj, ewerful, bathtubful, hardi).
    declension!("apse", 95, "~~";
        noun: "~~", "~~s|~s~";
    ),
    // Virus: 78 words (citrus, octopus, phosphorus, onus, opus, anus).
    declension!("virus", 78, "~us";
        noun: "~us", "~uses|~i";
    ),
    // Stigma: 73 words (schema, stigma, lymphoma, stoma, tama, bregma).
    declension!("stigma", 73, "~";
        noun: "~", "~ta";
    ),
    // Orno: 72 words (gelato, spaghetto, fumetto, amaro, putto, albarello).
    declension!("orno", 72, "~o";
        noun: "~o", "~i";
    ),
    // Rix: 64 words (hallux, calx, faux, thorax, lanx, fornix).
    declension!("rix", 64, "~x";
        noun: "~x", "~ces";
    ),
    // Anx: 58 words (hyrax, unix, borax, crux, dux, vox).
    declension!("anx", 58, "~x";
        noun: "~x", "~xes|~ces";
    ),
    // Goy: 57 words (ars, hassid, zuz, gaon, goy, goi).
    declension!("goy", 57, "~";
        noun: "~", "~im";
    ),
    // Dodman: 35 words (dragoman, hetman, pitman, walkman, shaman, ksar).
    declension!("dodman", 35, "~a~";
        noun: "~a~", "~a~s|~e~";
    ),
    // Dry: 30 words (why, dry, only, andy, tony, roxy).
    declension!("dry", 30, "~y";
        noun: "~y", "~ys|~ies";
    ),
    // Atlas: 30 words (atlas, adonis, aegis, iris, chrysalis, endodermis).
    declension!("atlas", 30, "~~";
        noun: "~~", "~~es|~de~";
    ),
    // Turf: 30 words (turf, roof, oaf, flyleaf, tief, broadleaf).
    declension!("turf", 30, "~f";
        noun: "~f", "~fs|~ves";
    ),
    // Apex: 29 words (apex, pollex, vertex, vortex, codex, caudex).
    declension!("apex", 29, "~~x";
        noun: "~~x", "~ic~s";
    ),
    // Simile: 27 words (seme, doge, rete, olpe, simile, kiee).
    declension!("simile", 27, "~e";
        noun: "~e", "~es";
    ),
    // Aira: 26 words (sent, garda, kroon, cathar, blin, kulich).
    declension!("aira", 26, "~";
        noun: "~", "~i";
    ),
    // Leaf: 25 words (wolf, self, calf, elf, half, leaf).
    declension!("leaf", 25, "~f";
        noun: "~f", "~ves";
    ),
    // Kufah: 25 words (agorah, averah, sichah, mesorah, edah, haverah).
    declension!("kufah", 25, "~ah";
        noun: "~ah", "~ot";
    ),
    // Mage: 24 words (duce, mage, genie, zampone, principe, salume).
    declension!("mage", 24, "~e";
        noun: "~e", "~i";
    ),
    // Zvah: 24 words (zah, hillah, illah, uzah, zvah, kkah).
    declension!("zvah", 24, "~ah";
        noun: "~ah", "~ahs|~ot|~oth";
    ),
    // Endmusik: 23 words (ox, ubermensch, frau, herr, lspiess, ftmensch).
    declension!("endmusik", 23, "~";
        noun: "~", "~en";
    ),
    // Whiz: 20 words (quiz, lez, fez, spaz, whiz, prez).
    declension!("whiz", 20, "~";
        noun: "~", "~zes";
    ),
    // Gmen: 20 words (praenomen, prenomen, domen, hel, umen, zmer).
    declension!("gmen", 20, "~e~";
        noun: "~e~", "~e~s";
    ),
    // Chamois: 19 words (chamois, bris, cosmos, demos, elvis, ens).
    declension!("chamois", 19, "~s";
        noun: "~s", "~ses|~i";
    ),
    // Eros: 18 words (pons, dens, eros, gens, veritas, civitas).
    declension!("eros", 18, "~~";
        noun: "~~", "~te~";
    ),
    // Lira: 18 words (lira, libbra, fiamma, valuta, specchia, sfogliatella).
    declension!("lira", 18, "~a";
        noun: "~a", "~e";
    ),
    // Apsis: 18 words (apsis, osteomyelitis, urethritis, thyroiditis, spondylitis, apapsis).
    declension!("apsis", 18, "~i~";
        noun: "~i~", "~ide~|~e~";
    ),
    // Tableau: 17 words (beau, tableau, gateau, bateau, carreau, bandeau).
    declension!("tableau", 17, "~";
        noun: "~", "~x";
    ),
    // Exotic: 17 words (exotic, genu, vas, lev, geas, koulouri).
    declension!("exotic", 17, "~";
        noun: "~", "~a";
    ),
    // Jynx: 16 words (grex, sphex, iynx, jynx, ynx, cyx).
    declension!("jynx", 16, "~x";
        noun: "~x", "~ges";
    ),
    // Utzstaffel: 15 words (ee, kanone, grice, schnecke, palatschinke, kadelle).
    declension!("utzstaffel", 15, "~";
        noun: "~", "~n";
    ),
    // Alumna: 14 words (suka, alumna, capicola, sinta, truga, anka).
    declension!("alumna", 14, "~a";
        noun: "~a", "~i";
    ),
    // Dogtooth: 14 words (blackfoot, webfoot, crowfoot, flatfoot, spadefoot, lobefoot).
    declension!("dogtooth", 14, "~oo~";
        noun: "~oo~", "~oo~s|~ee~";
    ),
    // Pedale: 14 words (quale, centrale, radiale, pedale, carpale, tarsale).
    declension!("pedale", 14, "~e";
        noun: "~e", "~ia";
    ),
    // Foot: 13 words (tooth, foot, goose, cargoose, foote, toof).
    declension!("foot", 13, "~oo~";
        noun: "~oo~", "~ee~";
    ),
    // Mouse: 12 words (mouse, dormouse, louse, seamouse, sealouse, termouse).
    declension!("mouse", 12, "~ous~";
        noun: "~ous~", "~ic~";
    ),
    // Bes: 12 words (es, dis, bes, lis, tas, dorcas).
    declension!("bes", 12, "~";
        noun: "~", "~ses";
    ),
    // Zva: 12 words (halacha, bracha, chumra, semicha, natla, chavera).
    declension!("zva", 12, "~a";
        noun: "~a", "~ot";
    ),
    // Stirps: 11 words (urbs, polis, stirps, nys, rys, ytanis).
    declension!("stirps", 11, "~~";
        noun: "~~", "~e~";
    ),
    // Boscis: 10 words (penis, clavis, pelvis, geis, heliopsis, boscis).
    declension!("boscis", 10, "~i~";
        noun: "~i~", "~i~es";
    ),
    // Ilex: 10 words (ilex, fex, thex, narthex, duplex, index).
    declension!("ilex", 10, "~~x";
        noun: "~~x", "~~xes|~ic~s";
    ),
    // Umbo: 9 words (umbo, confessio, divisio, pernio, rusticatio, itio).
    declension!("umbo", 9, "~";
        noun: "~", "~nes";
    ),
    // Ozoan: 9 words (centroheliozoan, metazoan, ozoan, dysozoan, phozoan, bozoan).
    declension!("ozoan", 9, "~n";
        noun: "~n", "~|~ns";
    ),
    // Tes: 9 words (stes, tes, nothetes, states, eutes, oblytes).
    declension!("tes", 9, "~es";
        noun: "~es", "~ai";
    ),
    // Life: 8 words (wife, life, lyfe, wyfe, ife, knife).
    declension!("life", 8, "~f~";
        noun: "~f~", "~v~s";
    ),
    // Ply: 7 words (ply, zloty, goby, gasy, kontany, zhky).
    declension!("ply", 7, "~y";
        noun: "~y", "~y|~ies";
    ),
    // Child: 6 words (child, ey, shild, grandchild, hild, perchild).
    declension!("child", 6, "~";
        noun: "~", "~ren";
    ),
    // Fomes: 6 words (eques, stipes, limes, termes, fomes, comes).
    declension!("fomes", 6, "~~";
        noun: "~~", "~it~";
    ),
    // Kte: 6 words (kore, arche, spithame, kte, nokhoe, khme).
    declension!("kte", 6, "~e";
        noun: "~e", "~ai";
    ),
    // Anos: 6 words (kotlet, anos, enyk, lysnyk, lesnyk, insyn).
    declension!("anos", 6, "~";
        noun: "~", "~y";
    ),
    // Money: 5 words (money, curtsey, loosey, smokey, vey).
    declension!("money", 5, "~~y";
        noun: "~~y", "~i~s|~~ys";
    ),
    // Ilak: 5 words (inuk, iak, uksuk, ilak, owczyk).
    declension!("ilak", 5, "~k";
        noun: "~k", "~ks";
    ),
    // Rimen: 5 words (binomen, trinomen, sudamen, rimen, foramen).
    declension!("rimen", 5, "~e~";
        noun: "~e~", "~i~a";
    ),
    // Godi: 5 words (godi, hudi, tellini, hadi, ayi).
    declension!("godi", 5, "~i";
        noun: "~i", "~is";
    ),
    // Lets: 5 words (tchervonets, rvonets, lets, popovets, uzek).
    declension!("lets", 5, "~e~";
        noun: "~e~", "~~y";
    ),
    // Anans: 5 words (formans, niens, anans, idens, icans).
    declension!("anans", 5, "~s";
        noun: "~s", "~tia";
    ),
    // Melamed: 5 words (melamed, yored, shomer, poseq, naged).
    declension!("melamed", 5, "~e~";
        noun: "~e~", "~~im";
    ),
    // Bedroomful: 5 words (courtmartial, neighborhoodful, airplaneful, bedroomful, handbagful).
    declension!("bedroomful", 5, "~~";
        noun: "~~", "~s~";
    ),
    // Smilo: 4 words (stelo, speso, askulo, smilo).
    declension!("smilo", 4, "~";
        noun: "~", "~j";
    ),
    // Ops: 4 words (stapes, ops, iens, hases).
    declension!("ops", 4, "~~";
        noun: "~~", "~~";
    ),
    // Firah: 4 words (menorah, hirah, inah, firah).
    declension!("firah", 4, "~ah";
        noun: "~ah", "~oth";
    ),
    // Aragus: 4 words (quadriporticus, aragus, ctus, detritus).
    declension!("aragus", 4, "~us";
        noun: "~us", "~us";
    ),
    // Aglio: 4 words (finocchio, fenocchio, aglio, ochio).
    declension!("aglio", 4, "~o";
        noun: "~o", "~";
    ),
    // Khol: 4 words (vort, chok, vok, khol).
    declension!("khol", 4, "~o~";
        noun: "~o~", "~o~s";
    ),
    // Mzee: 4 words (mwalimu, mzee, mzungu, mganga).
    declension!("mzee", 4, "m~";
        noun: "m~", "m~s|wa~";
    ),
    // Tripus: 4 words (pleiopus, pleopus, tripus, lagopus).
    declension!("tripus", 4, "~u~";
        noun: "~u~", "~ode~";
    ),
    // Nada: 4 words (xueta, vraka, nada, ouna).
    declension!("nada", 4, "~a";
        noun: "~a", "~es";
    ),
    // Nus: 4 words (nus, genus, enus, latus).
    declension!("nus", 4, "~us";
        noun: "~us", "~era";
    ),
    // Zri: 4 words (kenazi, lui, kephardi, zri).
    declension!("zri", 4, "~";
        noun: "~", "~m";
    ),
    // Filius: 4 words (issimus, gasius, filius, tylius).
    declension!("filius", 4, "~us";
        noun: "~us", "~a";
    ),
    // Nderkind: 3 words (lied, nderkind, elied).
    declension!("nderkind", 3, "~";
        noun: "~", "~er";
    ),
    // Arco: 3 words (arco, attacco, intrico).
    declension!("arco", 3, "~o";
        noun: "~o", "~hi";
    ),
    // Ibis: 3 words (ibis, precis, psalis).
    declension!("ibis", 3, "~i~";
        noun: "~i~", "~i~";
    ),
    // Dromos: 3 words (dromos, enos, hoos).
    declension!("dromos", 3, "~os";
        noun: "~os", "~oi|~i";
    ),
    // Ordo: 3 words (ordo, cardo, imago).
    declension!("ordo", 3, "~o";
        noun: "~o", "~ines";
    ),
    // Lepiades: 3 words (hermes, lepiades, psaltis).
    declension!("lepiades", 3, "~~s";
        noun: "~~s", "~a~";
    ),
    // Lowlife: 3 words (lowlife, huswife, yfe).
    declension!("lowlife", 3, "~f~";
        noun: "~f~", "~f~s|~v~s";
    ),
    // Mraad: 3 words (raad, reaal, mraad).
    declension!("mraad", 3, "~a~";
        noun: "~a~", "~~en|~a~s";
    ),
    // Dars: 3 words (fals, hadd, dars).
    declension!("dars", 3, "~a~~";
        noun: "~a~~", "~u~u~";
    ),
    // Larin: 3 words (larin, poridian, aterian).
    declension!("larin", 3, "~n";
        noun: "~n", "~ns|~";
    ),
    // Mosotho: 3 words (mosotho, motswana, mosarwa).
    declension!("mosotho", 3, "mo~";
        noun: "mo~", "ba~";
    ),
    // Colstaff: 3 words (bookstaff, colstaff, colestaff).
    declension!("colstaff", 3, "~ff";
        noun: "~ff", "~ves";
    ),
    // Silrada: 3 words (barma, zmijovka, silrada).
    declension!("silrada", 3, "~a";
        noun: "~a", "~y";
    ),
    // Pereq: 3 words (pesher, pereq, shoresh).
    declension!("pereq", 3, "~e~";
        noun: "~e~", "~a~im";
    ),
    // Forefemur: 3 words (forefemur, mesofemur, metafemur).
    declension!("forefemur", 3, "~u~";
        noun: "~u~", "~o~a";
    ),
    // Iika: 3 words (dghajsa, iyka, iika).
    declension!("iika", 3, "~~a";
        noun: "~~a", "~~as";
    ),
    // Luschick: 3 words (seecatch, seacatch, luschick).
    declension!("luschick", 3, "~";
        noun: "~", "~ie";
    ),
    // Chochem: 3 words (chochem, bovanets, henets).
    declension!("chochem", 3, "~e~";
        noun: "~e~", "~e~";
    ),
    // Ibhaca: 3 words (igqirha, itongo, ibhaca).
    declension!("ibhaca", 3, "i~";
        noun: "i~", "ama~";
    ),
    // Gadol: 3 words (naphil, gadol, saraph).
    declension!("gadol", 3, "~a~";
        noun: "~a~", "~e~im";
    ),
    // Roz: 3 words (agaz, roz, imez).
    declension!("roz", 3, "~z";
        noun: "~z", "~zes|~ces";
    ),
    // Ajir: 3 words (ajir, jahid, jahed).
    declension!("ajir", 3, "~";
        noun: "~", "~een";
    ),
    // Qya: 3 words (ryful, raryful, qya).
    declension!("qya", 3, "~y~";
        noun: "~y~", "~y~s|~ies~";
    ),
    // Nok: 3 words (zhok, nok, shok).
    declension!("nok", 3, "~o~";
        noun: "~o~", "~~i";
    ),
];
