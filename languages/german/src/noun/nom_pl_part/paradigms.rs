//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `nominative` (pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `platte`: 23,309 words.
    Platte,
    /// Like `minderung`: 22,849 words.
    Minderung,
    /// Like `leben`: 22,089 words.
    Leben,
    /// Like `tag`: 21,167 words.
    Tag,
    /// Like `hündin`: 7,260 words.
    Hündin,
    /// Like `kanon`: 6,688 words.
    Kanon,
    /// Like `einsatz`: 5,734 words.
    Einsatz,
    /// Like `fuchs`: 2,295 words.
    Fuchs,
    /// Like `blatt`: 1,716 words.
    Blatt,
    /// Like `bild`: 1,156 words.
    Bild,
    /// Like `korb`: 1,093 words.
    Korb,
    /// Like `grüner`: 971 words.
    Grüner,
    /// Like `kasten`: 724 words.
    Kasten,
    /// Like `zentrum`: 624 words.
    Zentrum,
    /// Like `loch`: 614 words.
    Loch,
    /// Like `zerwürfnis`: 564 words.
    Zerwürfnis,
    /// Like `wurm`: 544 words.
    Wurm,
    /// Like `chauvinismus`: 439 words.
    Chauvinismus,
    /// Like `thema`: 437 words.
    Thema,
    /// Like `visum`: 342 words.
    Visum,
    /// Like `boden`: 277 words.
    Boden,
    /// Like `lager`: 201 words.
    Lager,
    /// Like `kapital`: 157 words.
    Kapital,
    /// Like `guß`: 155 words.
    Guß,
    /// Like `gauch`: 106 words.
    Gauch,
    /// Like `lexikon`: 105 words.
    Lexikon,
    /// Like `bau`: 103 words.
    Bau,
    /// Like `modus`: 100 words.
    Modus,
    /// Like `bruder`: 95 words.
    Bruder,
    /// Like `pyxis`: 94 words.
    Pyxis,
    /// Like `basis`: 86 words.
    Basis,
    /// Like `geschoß`: 78 words.
    Geschoß,
    /// Like `konto`: 77 words.
    Konto,
    /// Like `obmann`: 59 words.
    Obmann,
    /// Like `delta`: 57 words.
    Delta,
    /// Like `kommando`: 55 words.
    Kommando,
    /// Like `saal`: 52 words.
    Saal,
    /// Like `putto`: 47 words.
    Putto,
    /// Like `pfosten`: 46 words.
    Pfosten,
    /// Like `bestes`: 39 words.
    Bestes,
    /// Like `verbund`: 39 words.
    Verbund,
    /// Like `stadion`: 38 words.
    Stadion,
    /// Like `nomen`: 37 words.
    Nomen,
    /// Like `werkstatt`: 37 words.
    Werkstatt,
    /// Like `miss`: 36 words.
    Miss,
    /// Like `schlot`: 36 words.
    Schlot,
    /// Like `lemma`: 25 words.
    Lemma,
    /// Like `mythos`: 22 words.
    Mythos,
    /// Like `seemann`: 22 words.
    Seemann,
    /// Like `aetosaurus`: 21 words.
    Aetosaurus,
    /// Like `nationalpark`: 17 words.
    Nationalpark,
    /// Like `ablaß`: 16 words.
    Ablaß,
    /// Like `genitale`: 16 words.
    Genitale,
    /// Like `gravis`: 15 words.
    Gravis,
    /// Like `gondoliere`: 15 words.
    Gondoliere,
    /// Like `kodex`: 14 words.
    Kodex,
    /// Like `schloß`: 14 words.
    Schloß,
    /// Like `haruspex`: 14 words.
    Haruspex,
    /// Like `dressman`: 14 words.
    Dressman,
    /// Like `stimulans`: 14 words.
    Stimulans,
    /// Like `faß`: 13 words.
    Faß,
    /// Like `männchen`: 12 words.
    Männchen,
    /// Like `bocher`: 12 words.
    Bocher,
    /// Like `präsens`: 11 words.
    Präsens,
    /// Like `hiatus`: 11 words.
    Hiatus,
    /// Like `großbetrieb`: 11 words.
    Großbetrieb,
    /// Like `hobby`: 10 words.
    Hobby,
    /// Like `prostylos`: 10 words.
    Prostylos,
    /// Like `muta`: 10 words.
    Muta,
    /// Like `bonus`: 9 words.
    Bonus,
    /// Like `fellatio`: 9 words.
    Fellatio,
    /// Like `finale`: 9 words.
    Finale,
    /// Like `ciabatta`: 9 words.
    Ciabatta,
    /// Like `causa`: 9 words.
    Causa,
    /// Like `gravamen`: 9 words.
    Gravamen,
    /// Like `rom`: 8 words.
    Rom,
    /// Like `abt`: 8 words.
    Abt,
    /// Like `matrix`: 8 words.
    Matrix,
    /// Like `ypsilon`: 8 words.
    Ypsilon,
    /// Like `numerale`: 8 words.
    Numerale,
    /// Like `idiotikon`: 8 words.
    Idiotikon,
    /// Like `syndikus`: 8 words.
    Syndikus,
    /// Like `trauma`: 7 words.
    Trauma,
    /// Like `thorax`: 7 words.
    Thorax,
    /// Like `risiko`: 7 words.
    Risiko,
    /// Like `mikwe`: 7 words.
    Mikwe,
    /// Like `vitiligo`: 7 words.
    Vitiligo,
    /// Like `amtfrau`: 7 words.
    Amtfrau,
    /// Like `krebsrisiko`: 7 words.
    Krebsrisiko,
    /// Like `sephardi`: 6 words.
    Sephardi,
    /// Like `alija`: 6 words.
    Alija,
    /// Like `marend`: 6 words.
    Marend,
    /// Like `helix`: 6 words.
    Helix,
    /// Like `iris`: 5 words.
    Iris,
    /// Like `tussi`: 5 words.
    Tussi,
    /// Like `korpus`: 5 words.
    Korpus,
    /// Like `glottis`: 5 words.
    Glottis,
    /// Like `beisaß`: 5 words.
    Beisaß,
    /// Like `bütterken`: 5 words.
    Bütterken,
    /// Like `koog`: 5 words.
    Koog,
    /// Like `regens`: 5 words.
    Regens,
    /// Like `autoatlas`: 5 words.
    Autoatlas,
    /// Like `griwna`: 5 words.
    Griwna,
    /// Like `seeklima`: 5 words.
    Seeklima,
    /// Like `apfel`: 4 words.
    Apfel,
    /// Like `moos`: 4 words.
    Moos,
    /// Like `pater`: 4 words.
    Pater,
    /// Like `zingulum`: 4 words.
    Zingulum,
    /// Like `genus`: 4 words.
    Genus,
    /// Like `erhaltungszustand`: 4 words.
    Erhaltungszustand,
    /// Like `heros`: 4 words.
    Heros,
    /// Like `bitok`: 4 words.
    Bitok,
    /// Like `kursus`: 4 words.
    Kursus,
    /// Like `natanger`: 4 words.
    Natanger,
    /// Like `penis`: 3 words.
    Penis,
    /// Like `peschmerga`: 3 words.
    Peschmerga,
    /// Like `tupperparty`: 3 words.
    Tupperparty,
    /// Like `polis`: 3 words.
    Polis,
    /// Like `kaktus`: 3 words.
    Kaktus,
    /// Like `antioxidans`: 3 words.
    Antioxidans,
    /// Like `konklave`: 3 words.
    Konklave,
    /// Like `codex`: 3 words.
    Codex,
    /// Like `aulos`: 3 words.
    Aulos,
    /// Like `forzeps`: 3 words.
    Forzeps,
    /// Like `önophiler`: 3 words.
    Önophiler,
    /// Like `rindvieh`: 3 words.
    Rindvieh,
    /// Like `pharynx`: 3 words.
    Pharynx,
    /// Like `schängelche`: 3 words.
    Schängelche,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 128] = [
        Declension::Platte,
        Declension::Minderung,
        Declension::Leben,
        Declension::Tag,
        Declension::Hündin,
        Declension::Kanon,
        Declension::Einsatz,
        Declension::Fuchs,
        Declension::Blatt,
        Declension::Bild,
        Declension::Korb,
        Declension::Grüner,
        Declension::Kasten,
        Declension::Zentrum,
        Declension::Loch,
        Declension::Zerwürfnis,
        Declension::Wurm,
        Declension::Chauvinismus,
        Declension::Thema,
        Declension::Visum,
        Declension::Boden,
        Declension::Lager,
        Declension::Kapital,
        Declension::Guß,
        Declension::Gauch,
        Declension::Lexikon,
        Declension::Bau,
        Declension::Modus,
        Declension::Bruder,
        Declension::Pyxis,
        Declension::Basis,
        Declension::Geschoß,
        Declension::Konto,
        Declension::Obmann,
        Declension::Delta,
        Declension::Kommando,
        Declension::Saal,
        Declension::Putto,
        Declension::Pfosten,
        Declension::Bestes,
        Declension::Verbund,
        Declension::Stadion,
        Declension::Nomen,
        Declension::Werkstatt,
        Declension::Miss,
        Declension::Schlot,
        Declension::Lemma,
        Declension::Mythos,
        Declension::Seemann,
        Declension::Aetosaurus,
        Declension::Nationalpark,
        Declension::Ablaß,
        Declension::Genitale,
        Declension::Gravis,
        Declension::Gondoliere,
        Declension::Kodex,
        Declension::Schloß,
        Declension::Haruspex,
        Declension::Dressman,
        Declension::Stimulans,
        Declension::Faß,
        Declension::Männchen,
        Declension::Bocher,
        Declension::Präsens,
        Declension::Hiatus,
        Declension::Großbetrieb,
        Declension::Hobby,
        Declension::Prostylos,
        Declension::Muta,
        Declension::Bonus,
        Declension::Fellatio,
        Declension::Finale,
        Declension::Ciabatta,
        Declension::Causa,
        Declension::Gravamen,
        Declension::Rom,
        Declension::Abt,
        Declension::Matrix,
        Declension::Ypsilon,
        Declension::Numerale,
        Declension::Idiotikon,
        Declension::Syndikus,
        Declension::Trauma,
        Declension::Thorax,
        Declension::Risiko,
        Declension::Mikwe,
        Declension::Vitiligo,
        Declension::Amtfrau,
        Declension::Krebsrisiko,
        Declension::Sephardi,
        Declension::Alija,
        Declension::Marend,
        Declension::Helix,
        Declension::Iris,
        Declension::Tussi,
        Declension::Korpus,
        Declension::Glottis,
        Declension::Beisaß,
        Declension::Bütterken,
        Declension::Koog,
        Declension::Regens,
        Declension::Autoatlas,
        Declension::Griwna,
        Declension::Seeklima,
        Declension::Apfel,
        Declension::Moos,
        Declension::Pater,
        Declension::Zingulum,
        Declension::Genus,
        Declension::Erhaltungszustand,
        Declension::Heros,
        Declension::Bitok,
        Declension::Kursus,
        Declension::Natanger,
        Declension::Penis,
        Declension::Peschmerga,
        Declension::Tupperparty,
        Declension::Polis,
        Declension::Kaktus,
        Declension::Antioxidans,
        Declension::Konklave,
        Declension::Codex,
        Declension::Aulos,
        Declension::Forzeps,
        Declension::Önophiler,
        Declension::Rindvieh,
        Declension::Pharynx,
        Declension::Schängelche,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 128] = [
    // Platte: 23,309 words (japanese, chinese, vietnamese, kugelschnecke, kugel, feder).
    declension!("platte", 23309, "~";
        nominative: "~n";
    ),
    // Minderung: 22,849 words (uhr, wv, bestellung, herz, wissenschaft, u-bahn).
    declension!("minderung", 22849, "~";
        nominative: "~en";
    ),
    // Leben: 22,089 words (a, september, november, f, y, o).
    declension!("leben", 22089, "~";
        nominative: "~";
    ),
    // Tag: 21,167 words (april, german, samstag, muslim, sonnabend, parlament).
    declension!("tag", 21167, "~";
        nominative: "~e";
    ),
    // Hündin: 7,260 words (ärztin, freundin, ehegattin, gattin, gemahlin, sekretärin).
    declension!("hündin", 7260, "~";
        nominative: "~nen";
    ),
    // Kanon: 6,688 words (led, dvd, aborigine, ufo, mg, pb).
    declension!("kanon", 6688, "~";
        nominative: "~s";
    ),
    // Einsatz: 5,734 words (maus, gaul, schneeball, stadt, zahn, zahnarzt).
    declension!("einsatz", 5734, "~a~";
        nominative: "~ä~e";
    ),
    // Fuchs: 2,295 words (kuh, sehnsucht, ehebruch, fuchs, zug, aufzug).
    declension!("fuchs", 2295, "~u~";
        nominative: "~ü~e";
    ),
    // Blatt: 1,716 words (haus, schiebedach, fahrrad, kraut, glücksrad, wald).
    declension!("blatt", 1716, "~a~";
        nominative: "~ä~er";
    ),
    // Bild: 1,156 words (zeitgeist, geist, licht, schwert, bild, brett).
    declension!("bild", 1156, "~";
        nominative: "~er";
    ),
    // Korb: 1,093 words (warenkorb, kopf, dummkopf, block, bock, borg).
    declension!("korb", 1093, "~o~";
        nominative: "~ö~e";
    ),
    // Grüner: 971 words (homosexueller, deutscher, windischer, süßer, federweißer, lieber).
    declension!("grüner", 971, "~r";
        nominative: "~";
    ),
    // Kasten: 724 words (garten, vater, hammer, laden, nagel, erdapfel).
    declension!("kasten", 724, "~a~";
        nominative: "~ä~";
    ),
    // Zentrum: 624 words (impressum, elysium, album, aquarium, atrium, auditorium).
    declension!("zentrum", 624, "~um";
        nominative: "~en";
    ),
    // Loch: 614 words (vorhängeschloss, horn, loch, wort, herrenvolk, arschloch).
    declension!("loch", 614, "~o~";
        nominative: "~ö~er";
    ),
    // Zerwürfnis: 564 words (verhältnis, exodus, mercedes, albatros, as, autobus).
    declension!("zerwürfnis", 564, "~";
        nominative: "~se";
    ),
    // Wurm: 544 words (buch, übersetzungswörterbuch, wörterbuch, wurm, gut, ohrwurm).
    declension!("wurm", 544, "~u~";
        nominative: "~ü~er";
    ),
    // Chauvinismus: 439 words (kapitalismus, analphabetismus, chiasmus, virus, marxismus, kommunismus).
    declension!("chauvinismus", 439, "~us";
        nominative: "~en";
    ),
    // Thema: 437 words (tara, agenda, algebra, ambrosia, aroma, aura).
    declension!("thema", 437, "~a";
        nominative: "~en";
    ),
    // Visum: 342 words (vakuum, maximum, minimum, neutrum, optimum, perineum).
    declension!("visum", 342, "~um";
        nominative: "~a";
    ),
    // Boden: 277 words (boden, vogel, fußboden, bogen, backofen, tochter).
    declension!("boden", 277, "~o~";
        nominative: "~ö~";
    ),
    // Lager: 201 words (wasser, lager, wagen, hammel, kragen, wildwasser).
    declension!("lager", 201, "~a~";
        nominative: "~a~|~ä~";
    ),
    // Kapital: 157 words (mineral, adverb, fossil, material, partizip, filmmaterial).
    declension!("kapital", 157, "~";
        nominative: "~ien";
    ),
    // Guß: 155 words (erdnuß, muskatnuß, nuß, kuß, schluß, guß).
    declension!("guß", 155, "~uß";
        nominative: "~üsse";
    ),
    // Gauch: 106 words (admiral, sand, zwieback, pasch, verlag, bausch).
    declension!("gauch", 106, "~a~";
        nominative: "~a~e|~ä~e";
    ),
    // Lexikon: 105 words (lexikon, macron, mitochondrion, oxymoron, periastron, apastron).
    declension!("lexikon", 105, "~on";
        nominative: "~a";
    ),
    // Bau: 103 words (mastodon, bau, altbau, aufbau, vorbau, reaktorbau).
    declension!("bau", 103, "~";
        nominative: "~ten";
    ),
    // Modus: 100 words (anus, hippocampus, incubus, phallus, radius, stimulus).
    declension!("modus", 100, "~us";
        nominative: "~i";
    ),
    // Bruder: 95 words (puffmutter, mutter, großmutter, bruder, schwiegermutter, urgroßmutter).
    declension!("bruder", 95, "~u~";
        nominative: "~ü~";
    ),
    // Pyxis: 94 words (epiglottis, pyxis, apsis, tonsillitis, appendizitis, sinusitis).
    declension!("pyxis", 94, "~s";
        nominative: "~den";
    ),
    // Basis: 86 words (amaryllis, basis, genesis, praxis, synopsis, tmesis).
    declension!("basis", 86, "~is";
        nominative: "~en";
    ),
    // Geschoß: 78 words (biß, schiß, riß, imbiß, abszeß, bräunungsprozeß).
    declension!("geschoß", 78, "~ß";
        nominative: "~sse";
    ),
    // Konto: 77 words (konto, szenario, saldo, fresko, benutzerkonto, bankkonto).
    declension!("konto", 77, "~o";
        nominative: "~en";
    ),
    // Obmann: 59 words (ackermann, gefolgsmann, kaufmann, fuhrmann, schutzmann, mittelsmann).
    declension!("obmann", 59, "~mann";
        nominative: "~männer|~leute";
    ),
    // Delta: 57 words (balalaika, delta, junta, swastika, mafia, plazenta).
    declension!("delta", 57, "~a";
        nominative: "~as|~en";
    ),
    // Kommando: 55 words (prosecco, cello, cembalo, ghetto, torso, kommando).
    declension!("kommando", 55, "~o";
        nominative: "~os|~i";
    ),
    // Saal: 52 words (saal, prunksaal, operationssaal, hörsaal, ballsaal, wartesaal).
    declension!("saal", 52, "~aa~";
        nominative: "~ä~e";
    ),
    // Putto: 47 words (tempo, mafioso, limoncello, paparazzo, sinto, scherzo).
    declension!("putto", 47, "~o";
        nominative: "~i";
    ),
    // Pfosten: 46 words (spannungsbogen, regenbogen, pfosten, polster, fragebogen, triumphbogen).
    declension!("pfosten", 46, "~o~";
        nominative: "~o~|~ö~";
    ),
    // Bestes: 39 words (adlerjunges, schwanenjunges, junges, neugeborenes, vielfaches, immaterielles).
    declension!("bestes", 39, "~s";
        nominative: "~";
    ),
    // Verbund: 39 words (bug, fund, huf, schluck, mundschutz, puff).
    declension!("verbund", 39, "~u~";
        nominative: "~u~e|~ü~e";
    ),
    // Stadion: 38 words (ganglion, skioptikon, stadion, olympiastadion, distichon, scholion).
    declension!("stadion", 38, "~o~";
        nominative: "~e~";
    ),
    // Nomen: 37 words (possessivpronomen, abdomen, omen, pronomen, demonstrativpronomen, volumen).
    declension!("nomen", 37, "~e~";
        nominative: "~e~|~i~a";
    ),
    // Werkstatt: 37 words (werkstatt, autoreparaturwerkstatt, zinngießerwerkstatt, walstatt, autowerkstatt, servicewerkstatt).
    declension!("werkstatt", 37, "~a~";
        nominative: "~ä~en";
    ),
    // Miss: 36 words (lunch, miss, tremor, match, coach, kautsch).
    declension!("miss", 36, "~";
        nominative: "~es";
    ),
    // Schlot: 36 words (rost, ross, schlot, walross, herkunftsort, bloch).
    declension!("schlot", 36, "~o~";
        nominative: "~o~e|~ö~er";
    ),
    // Lemma: 25 words (klima, lemma, analemma, trilemma, anathema, raumklima).
    declension!("lemma", 25, "~";
        nominative: "~ta";
    ),
    // Mythos: 22 words (mythos, kosmos, epos, amphiprostylos, nationalepos, demos).
    declension!("mythos", 22, "~os";
        nominative: "~en";
    ),
    // Seemann: 22 words (landsmann, feuerwehrmann, geschäftsmann, seemann, fährmann, hauptmann).
    declension!("seemann", 22, "~mann";
        nominative: "~leute|~männer";
    ),
    // Aetosaurus: 21 words (dinosaurus, arizonasaurus, allosaurus, apatosaurus, brachiosaurus, stegosaurus).
    declension!("aetosaurus", 21, "~us";
        nominative: "~ier";
    ),
    // Nationalpark: 17 words (nationalpark, kurpark, naturpark, windpark, solarpark, walkman).
    declension!("nationalpark", 17, "~a~";
        nominative: "~a~s|~a~e|~ä~e";
    ),
    // Ablaß: 16 words (ablaß, paß, anlaß, nachlaß, baß, kontrabaß).
    declension!("ablaß", 16, "~aß";
        nominative: "~ässe";
    ),
    // Genitale: 16 words (generale, korporale, genitale, dimissoriale, adverbiale, insigne).
    declension!("genitale", 16, "~~";
        nominative: "~i~n";
    ),
    // Gravis: 15 words (gravis, tenuis, myiasis, acidosis, classis, amanuensis).
    declension!("gravis", 15, "~i~";
        nominative: "~e~";
    ),
    // Gondoliere: 15 words (gondoliere, monsignore, condottiere, kondottiere, panettone, minestrone).
    declension!("gondoliere", 15, "~e";
        nominative: "~i";
    ),
    // Kodex: 14 words (index, hitzeindex, apex, kodex, verbraucherpreisindex, synthese-index).
    declension!("kodex", 14, "~~x";
        nominative: "~iz~s";
    ),
    // Schloß: 14 words (schloß, vorhängeschloß, barockschloß, königsschloß, luftschloß, zahlenschloß).
    declension!("schloß", 14, "~oß";
        nominative: "~össer";
    ),
    // Haruspex: 14 words (haruspex, verhaltenskodex, brechungsindex, aktienindex, ehrenkodex, kortex).
    declension!("haruspex", 14, "~~x";
        nominative: "~~xe";
    ),
    // Dressman: 14 words (anchorman, gentleman, dressman, stuntman, ironman, ironwoman).
    declension!("dressman", 14, "~a~";
        nominative: "~e~";
    ),
    // Stimulans: 14 words (stimulans, ingrediens, formans, adjuvans, antezedens, agens).
    declension!("stimulans", 14, "~s";
        nominative: "~zien|~tia";
    ),
    // Faß: 13 words (faß, weinfaß, pulverfaß, tintenfaß, wasserfaß, holzfaß).
    declension!("faß", 13, "~aß";
        nominative: "~ässer";
    ),
    // Männchen: 12 words (männchen, eichen, liedchen, lämmchen, hühnchen, lichtchen).
    declension!("männchen", 12, "~~";
        nominative: "~~|~er~";
    ),
    // Bocher: 12 words (zaddik, cherub, bocher, kibbuz, moschaw, kubbuz).
    declension!("bocher", 12, "~";
        nominative: "~im";
    ),
    // Präsens: 11 words (präsens, deponens, definiens, präteritopräsens, präterito-präsens, rubefaziens).
    declension!("präsens", 11, "~s";
        nominative: "~tia";
    ),
    // Hiatus: 11 words (hiatus, palpus, maxillarpalpus, dekubitus, iktus, zirrus).
    declension!("hiatus", 11, "~us";
        nominative: "~us";
    ),
    // Großbetrieb: 11 words (portfolioumschlag, finanzierungsvorgang, anlagehorizont, anlageparameter, anlagezeitraum, portfolioumsatz).
    declension!("großbetrieb", 11, "~";
        nominative: "~-e";
    ),
    // Hobby: 10 words (tommy, hobby, party, rowdy, shanty, coronaparty).
    declension!("hobby", 10, "~y";
        nominative: "~ys|~ies";
    ),
    // Prostylos: 10 words (prostylos, uroboros, kantharos, oikos, hypostylos, pegasos).
    declension!("prostylos", 10, "~s";
        nominative: "~i";
    ),
    // Muta: 10 words (präantepänultima, liquida, muta, pänultima, antepänultima, antepaenultima).
    declension!("muta", 10, "~a";
        nominative: "~ä";
    ),
    // Bonus: 9 words (abakus, bonus, diskus, nimbus, nonius, konus).
    declension!("bonus", 9, "~us";
        nominative: "~usse|~en";
    ),
    // Fellatio: 9 words (fellatio, defensio, subdivisio, aberratio, divisio, debellatio).
    declension!("fellatio", 9, "~";
        nominative: "~nes";
    ),
    // Finale: 9 words (finale, achtelfinale, cupfinale, champions-league-finale, sechzehntelfinale, pokalfinale).
    declension!("finale", 9, "~e";
        nominative: "~e|~s";
    ),
    // Ciabatta: 9 words (pulcinella, stracciatella, ciabatta, banda, aranciata, viscontessa).
    declension!("ciabatta", 9, "~a";
        nominative: "~e";
    ),
    // Causa: 9 words (praeantepaenultima, causa, coxa, matta, vita, nympha).
    declension!("causa", 9, "~a";
        nominative: "~ae|~en";
    ),
    // Gravamen: 9 words (wasservolumen, wurzelnomen, notexamen, gravamen, lungenvolumen, juraexamen).
    declension!("gravamen", 9, "~e~";
        nominative: "~i~a";
    ),
    // Rom: 8 words (rom, pharmazeutik, artepithet, palliativ, kosmetik, tuber).
    declension!("rom", 8, "~";
        nominative: "~a";
    ),
    // Abt: 8 words (arzt, abt, angst, arsch, axt, ast).
    declension!("abt", 8, "a~";
        nominative: "ä~e";
    ),
    // Matrix: 8 words (matrix, varix, diagonalmatrix, fallmatrix, brandfallmatrix, steuermatrix).
    declension!("matrix", 8, "~x";
        nominative: "~zen";
    ),
    // Ypsilon: 8 words (ypsilon, semikolon, kolon, telencephalon, diencephalon, mesencephalon).
    declension!("ypsilon", 8, "~on";
        nominative: "~ons|~a";
    ),
    // Numerale: 8 words (kardinale, ordinale, numerale, pronominale, antifebrile, quale).
    declension!("numerale", 8, "~e";
        nominative: "~ia";
    ),
    // Idiotikon: 8 words (idiotikon, akrostichon, plektron, telestichon, pastophorion, pyramidion).
    declension!("idiotikon", 8, "~on";
        nominative: "~en|~a";
    ),
    // Syndikus: 8 words (syndikus, kretikus, prokeleusmatikus, spondiakus, askus, medikus).
    declension!("syndikus", 8, "~kus";
        nominative: "~zi";
    ),
    // Trauma: 7 words (schema, trauma, stigma, chiasma, xeroderma, leitthema).
    declension!("trauma", 7, "~a";
        nominative: "~ata|~en";
    ),
    // Thorax: 7 words (sphinx, thorax, archaeopteryx, cephalothorax, zephalothorax, hemithorax).
    declension!("thorax", 7, "~x";
        nominative: "~xe|~ces";
    ),
    // Risiko: 7 words (risiko, brandrisiko, gesundheitsrisiko, krankheitsrisiko, berufsrisiko, entdeckungsrisiko).
    declension!("risiko", 7, "~i~o";
        nominative: "~i~en|~i~os|~~en";
    ),
    // Mikwe: 7 words (marone, mikwe, straßenjunge, abbate, achtgroschenjunge, negerjunge).
    declension!("mikwe", 7, "~e";
        nominative: "~en";
    ),
    // Vitiligo: 7 words (vitiligo, subordo, infraordo, ordo, tendo, intertrigo).
    declension!("vitiligo", 7, "~o";
        nominative: "~ines";
    ),
    // Amtfrau: 7 words (kamerafrau, amtfrau, fuhrfrau, instfrau, kirchenfrau, ss-frau).
    declension!("amtfrau", 7, "~fra~";
        nominative: "~fra~en|~le~te";
    ),
    // Krebsrisiko: 7 words (krebsrisiko, verletzungsrisiko, spielrisiko, gefährdungsrisiko, armutsrisiko, unfallrisiko).
    declension!("krebsrisiko", 7, "~i~o";
        nominative: "~i~os|~i~en|~~en";
    ),
    // Sephardi: 6 words (sephardi, energieaufnahme, aschkenasi, charedi, mizrachi, misrachi).
    declension!("sephardi", 6, "~";
        nominative: "~m";
    ),
    // Alija: 6 words (aliya, alija, chanukkia, mesusa, semicha, halacha).
    declension!("alija", 6, "~a";
        nominative: "~ot";
    ),
    // Marend: 6 words (marend, pelmen, owrag, somal, signor, silowik).
    declension!("marend", 6, "~";
        nominative: "~i";
    ),
    // Helix: 6 words (vox, helix, unix, obstetrix, dux, doppelhelix).
    declension!("helix", 6, "~x";
        nominative: "~ces";
    ),
    // Iris: 5 words (iris, spezies, pflanzenspezies, tierspezies, patiens).
    declension!("iris", 5, "~s";
        nominative: "~s|~n";
    ),
    // Tussi: 5 words (taxi, tussi, sozi, anrufsammeltaxi, anrufersammeltaxi).
    declension!("tussi", 5, "~i";
        nominative: "~is|~en";
    ),
    // Korpus: 5 words (tempus, korpus, sprachkorpus, corpus, textkorpus).
    declension!("korpus", 5, "~us";
        nominative: "~ora";
    ),
    // Glottis: 5 words (glottis, klitoris, glans, capsulitis, conjunctivitis).
    declension!("glottis", 5, "~~";
        nominative: "~de~";
    ),
    // Beisaß: 5 words (prinzeß, bekandtnuß, stewardeß, beisaß, kothsaß).
    declension!("beisaß", 5, "~ß";
        nominative: "~ssen";
    ),
    // Bütterken: 5 words (bütterken, döneken, pädken, fietsken, dönneken).
    declension!("bütterken", 5, "~n";
        nominative: "~s|~ns|~n";
    ),
    // Koog: 5 words (koog, hook, schooß, schoos, schooss).
    declension!("koog", 5, "~oo~";
        nominative: "~ö~e";
    ),
    // Regens: 5 words (varietas, gens, regens, herpes, genitalherpes).
    declension!("regens", 5, "~~";
        nominative: "~te~";
    ),
    // Autoatlas: 5 words (sprachatlas, autoatlas, weltatlas, stadtatlas, reiseatlas).
    declension!("autoatlas", 5, "~s";
        nominative: "~sse|~nten";
    ),
    // Griwna: 5 words (isba, stotinka, papirossa, naliwka, griwna).
    declension!("griwna", 5, "~a";
        nominative: "~i";
    ),
    // Seeklima: 5 words (eisklima, trockenklima, kontinentalklima, seeklima, polarklima).
    declension!("seeklima", 5, "~";
        nominative: "~te";
    ),
    // Apfel: 4 words (apfel, appel, acker, aiw).
    declension!("apfel", 4, "a~";
        nominative: "ä~";
    ),
    // Moos: 4 words (moos, lebermoos, schlafmoos, wikingerboot).
    declension!("moos", 4, "~oo~";
        nominative: "~oo~e|~ö~er";
    ),
    // Pater: 4 words (pater, frater, jesuitenpater, konfrater).
    declension!("pater", 4, "~e~";
        nominative: "~~es";
    ),
    // Zingulum: 4 words (magnum, cingulum, zingulum, faktotum).
    declension!("zingulum", 4, "~um";
        nominative: "~ums|~a";
    ),
    // Genus: 4 words (genus, opus, onus, subgenus).
    declension!("genus", 4, "~us";
        nominative: "~era";
    ),
    // Erhaltungszustand: 4 words (spannungszustand, belagerungszustand, erhaltungszustand, ausgangszustand).
    declension!("erhaltungszustand", 4, "~s~a~";
        nominative: "~~ä~e";
    ),
    // Heros: 4 words (heros, taxis, topotaxis, kulturheros).
    declension!("heros", 4, "~s";
        nominative: "~en";
    ),
    // Bitok: 4 words (superblock, kintopp, orchestrion, bitok).
    declension!("bitok", 4, "~o~";
        nominative: "~o~s";
    ),
    // Kursus: 4 words (kursus, kultus, sprachkursus, abendkursus).
    declension!("kursus", 4, "~us";
        nominative: "~e";
    ),
    // Natanger: 4 words (natanger, schalauer, staufer, silurer).
    declension!("natanger", 4, "~r";
        nominative: "~r|~n";
    ),
    // Penis: 3 words (penis, saxophonpenis, hemipenis).
    declension!("penis", 3, "~i~";
        nominative: "~i~se|~e~";
    ),
    // Peschmerga: 3 words (visa, peschmerga, asta).
    declension!("peschmerga", 3, "~a";
        nominative: "~a|~as|~en";
    ),
    // Tupperparty: 3 words (tupperparty, tory, brony).
    declension!("tupperparty", 3, "~y";
        nominative: "~ies";
    ),
    // Polis: 3 words (sporn, polis, dexiosis).
    declension!("polis", 3, "~~";
        nominative: "~e~";
    ),
    // Kaktus: 3 words (kaktus, feigenkaktus, blattkaktus).
    declension!("kaktus", 3, "~us";
        nominative: "~een|~usse";
    ),
    // Antioxidans: 3 words (antioxidans, antikoagulans, determinans).
    declension!("antioxidans", 3, "~s";
        nominative: "~tien";
    ),
    // Konklave: 3 words (konklave, extemporale, missale).
    declension!("konklave", 3, "~~";
        nominative: "~~n|~i~n";
    ),
    // Codex: 3 words (codex, pollex, vertex).
    declension!("codex", 3, "~~x";
        nominative: "~ic~s";
    ),
    // Aulos: 3 words (topos, aulos, tholos).
    declension!("aulos", 3, "~os";
        nominative: "~oi|~en";
    ),
    // Forzeps: 3 words (forzeps, forceps, paterfamilias).
    declension!("forzeps", 3, "~e~~";
        nominative: "~e~~|~i~e~";
    ),
    // Önophiler: 3 words (önophiler, vinophiler, kaufsüchtiger).
    declension!("önophiler", 3, "~r";
        nominative: "~n";
    ),
    // Rindvieh: 3 words (urvieh, krabbelvieh, rindvieh).
    declension!("rindvieh", 3, "~~";
        nominative: "~c~er";
    ),
    // Pharynx: 3 words (pharynx, larynx, phorminx).
    declension!("pharynx", 3, "~x";
        nominative: "~gen";
    ),
    // Schängelche: 3 words (schängelche, krebbelche, fylke).
    declension!("schängelche", 3, "~";
        nominative: "~r";
    ),
];
