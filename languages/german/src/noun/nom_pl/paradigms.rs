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
/// `genitive` (pl)
/// `dative` (pl)
/// `accusative` (pl)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     nominative: $c0:tt;
     genitive: $c1:tt;
     dative: $c2:tt;
     accusative: $c3:tt;
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
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `platten`: 68,133 words.
    Platten,
    /// Like `füchse`: 51,083 words.
    Füchse,
    /// Like `alte`: 1,076 words.
    Alte,
    /// Like `billetts`: 486 words.
    Billetts,
    /// Like `stücke`: 264 words.
    Stücke,
    /// Like `mamsellen`: 197 words.
    Mamsellen,
    /// Like `lager`: 184 words.
    Lager,
    /// Like `haspeln`: 148 words.
    Haspeln,
    /// Like `gaden`: 144 words.
    Gaden,
    /// Like `scheichs`: 107 words.
    Scheichs,
    /// Like `vota`: 94 words.
    Vota,
    /// Like `obmänner`: 65 words.
    Obmänner,
    /// Like `deltas`: 63 words.
    Deltas,
    /// Like `fäden`: 59 words.
    Fäden,
    /// Like `kommandos`: 55 words.
    Kommandos,
    /// Like `omen`: 54 words.
    Omen,
    /// Like `pfosten`: 48 words.
    Pfosten,
    /// Like `pläne`: 46 words.
    Pläne,
    /// Like `polster`: 45 words.
    Polster,
    /// Like `verbunde`: 42 words.
    Verbunde,
    /// Like `grüfte`: 39 words.
    Grüfte,
    /// Like `primi`: 35 words.
    Primi,
    /// Like `loge`: 32 words.
    Loge,
    /// Like `wörter`: 26 words.
    Wörter,
    /// Like `couchs`: 22 words.
    Couchs,
    /// Like `blöcke`: 21 words.
    Blöcke,
    /// Like `seeleute`: 21 words.
    Seeleute,
    /// Like `mähler`: 18 words.
    Mähler,
    /// Like `männchen`: 18 words.
    Männchen,
    /// Like `lichter`: 17 words.
    Lichter,
    /// Like `haruspexe`: 16 words.
    Haruspexe,
    /// Like `fisken`: 14 words.
    Fisken,
    /// Like `hiatus`: 14 words.
    Hiatus,
    /// Like `adverbien`: 12 words.
    Adverbien,
    /// Like `stimulanzien`: 11 words.
    Stimulanzien,
    /// Like `hobbys`: 10 words.
    Hobbys,
    /// Like `eigentümer`: 10 words.
    Eigentümer,
    /// Like `mutä`: 9 words.
    Mutä,
    /// Like `bögen`: 8 words.
    Bögen,
    /// Like `stigmata`: 8 words.
    Stigmata,
    /// Like `ypsilons`: 8 words.
    Ypsilons,
    /// Like `trilemmata`: 8 words.
    Trilemmata,
    /// Like `nationalparks`: 8 words.
    Nationalparks,
    /// Like `nomina`: 8 words.
    Nomina,
    /// Like `risikos`: 8 words.
    Risikos,
    /// Like `crashs`: 8 words.
    Crashs,
    /// Like `thoraxe`: 7 words.
    Thoraxe,
    /// Like `amtfrauen`: 7 words.
    Amtfrauen,
    /// Like `häser`: 6 words.
    Häser,
    /// Like `tussis`: 6 words.
    Tussis,
    /// Like `flöße`: 6 words.
    Flöße,
    /// Like `bauten`: 6 words.
    Bauten,
    /// Like `krankheitsrisiken`: 6 words.
    Krankheitsrisiken,
    /// Like `präsentia`: 6 words.
    Präsentia,
    /// Like `faktotums`: 6 words.
    Faktotums,
    /// Like `romnija`: 6 words.
    Romnija,
    /// Like `windparks`: 6 words.
    Windparks,
    /// Like `gläser`: 5 words.
    Gläser,
    /// Like `lunches`: 5 words.
    Lunches,
    /// Like `konusse`: 5 words.
    Konusse,
    /// Like `kaffe`: 5 words.
    Kaffe,
    /// Like `autoatlasse`: 5 words.
    Autoatlasse,
    /// Like `cherubim`: 4 words.
    Cherubim,
    /// Like `abakusse`: 4 words.
    Abakusse,
    /// Like `embryonen`: 4 words.
    Embryonen,
    /// Like `indizes`: 4 words.
    Indizes,
    /// Like `burgen`: 4 words.
    Burgen,
    /// Like `moose`: 4 words.
    Moose,
    /// Like `aliasse`: 4 words.
    Aliasse,
    /// Like `bitoks`: 4 words.
    Bitoks,
    /// Like `pastoren`: 3 words.
    Pastoren,
    /// Like `penisse`: 3 words.
    Penisse,
    /// Like `auloi`: 3 words.
    Auloi,
    /// Like `ciabatte`: 3 words.
    Ciabatte,
    /// Like `forceps`: 3 words.
    Forceps,
    /// Like `pontifizes`: 3 words.
    Pontifizes,
    /// Like `chassidim`: 3 words.
    Chassidim,
    /// Like `suser`: 3 words.
    Suser,
    /// Like `narrenattribute`: 3 words.
    Narrenattribute,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 79] = [
        Declension::Platten,
        Declension::Füchse,
        Declension::Alte,
        Declension::Billetts,
        Declension::Stücke,
        Declension::Mamsellen,
        Declension::Lager,
        Declension::Haspeln,
        Declension::Gaden,
        Declension::Scheichs,
        Declension::Vota,
        Declension::Obmänner,
        Declension::Deltas,
        Declension::Fäden,
        Declension::Kommandos,
        Declension::Omen,
        Declension::Pfosten,
        Declension::Pläne,
        Declension::Polster,
        Declension::Verbunde,
        Declension::Grüfte,
        Declension::Primi,
        Declension::Loge,
        Declension::Wörter,
        Declension::Couchs,
        Declension::Blöcke,
        Declension::Seeleute,
        Declension::Mähler,
        Declension::Männchen,
        Declension::Lichter,
        Declension::Haruspexe,
        Declension::Fisken,
        Declension::Hiatus,
        Declension::Adverbien,
        Declension::Stimulanzien,
        Declension::Hobbys,
        Declension::Eigentümer,
        Declension::Mutä,
        Declension::Bögen,
        Declension::Stigmata,
        Declension::Ypsilons,
        Declension::Trilemmata,
        Declension::Nationalparks,
        Declension::Nomina,
        Declension::Risikos,
        Declension::Crashs,
        Declension::Thoraxe,
        Declension::Amtfrauen,
        Declension::Häser,
        Declension::Tussis,
        Declension::Flöße,
        Declension::Bauten,
        Declension::Krankheitsrisiken,
        Declension::Präsentia,
        Declension::Faktotums,
        Declension::Romnija,
        Declension::Windparks,
        Declension::Gläser,
        Declension::Lunches,
        Declension::Konusse,
        Declension::Kaffe,
        Declension::Autoatlasse,
        Declension::Cherubim,
        Declension::Abakusse,
        Declension::Embryonen,
        Declension::Indizes,
        Declension::Burgen,
        Declension::Moose,
        Declension::Aliasse,
        Declension::Bitoks,
        Declension::Pastoren,
        Declension::Penisse,
        Declension::Auloi,
        Declension::Ciabatte,
        Declension::Forceps,
        Declension::Pontifizes,
        Declension::Chassidim,
        Declension::Suser,
        Declension::Narrenattribute,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 79] = [
    // Platten: 68,133 words (a, f, y, o, leds, dvds).
    declension!("platten", 68133, "~";
        nominative: "~";
        genitive: "~";
        dative: "~";
        accusative: "~";
    ),
    // Füchse: 51,083 words (aprile, september, november, häuser, germane, dezember).
    declension!("füchse", 51083, "~";
        nominative: "~";
        genitive: "~";
        dative: "~n";
        accusative: "~";
    ),
    // Alte: 1,076 words (adlerjunge, schwanenjunge, homosexuelle, alte, deutsche, junge).
    declension!("alte", 1076, "~";
        nominative: "~";
        genitive: "~r";
        dative: "~n";
        accusative: "~";
    ),
    // Billetts: 486 words (ufos, mgs, moslems, pcs, wodkas, focs).
    declension!("billetts", 486, "~s";
        nominative: "~s";
        genitive: "~s";
        dative: "~s";
        accusative: "~s";
    ),
    // Stücke: 264 words (muslime, charaktere, handschuhe, schuhe, aperitife, biere).
    declension!("stücke", 264, "~e";
        nominative: "~e|~s";
        genitive: "~e|~s";
        dative: "~en|~s";
        accusative: "~e|~s";
    ),
    // Mamsellen: 197 words (gaeltachten, charismen, dogmen, foren, medien, pizzen).
    declension!("mamsellen", 197, "~en";
        nominative: "~en";
        genitive: "~en";
        dative: "~en";
        accusative: "~en";
    ),
    // Lager: 184 words (wasser, admirale, lager, sande, trame, zwiebacke).
    declension!("lager", 184, "~a~";
        nominative: "~a~|~ä~";
        genitive: "~a~|~ä~";
        dative: "~a~n|~ä~n";
        accusative: "~a~|~ä~";
    ),
    // Haspeln: 148 words (apachen, brezeln, leoparden, magneten, malaisen, masten).
    declension!("haspeln", 148, "~n";
        nominative: "~n|~";
        genitive: "~n|~";
        dative: "~n";
        accusative: "~n|~";
    ),
    // Gaden: 144 words (kragen, streitwagen, volkswagen, leichenwagen, lastkraftwagen, mietwagen).
    declension!("gaden", 144, "~a~";
        nominative: "~a~|~ä~";
        genitive: "~a~|~ä~";
        dative: "~a~|~ä~";
        accusative: "~a~|~ä~";
    ),
    // Scheichs: 107 words (starts, skats, haffs, blanketts, projektstarts, scheichs).
    declension!("scheichs", 107, "~s";
        nominative: "~s|~e";
        genitive: "~e|~s";
        dative: "~s|~en";
        accusative: "~s|~e";
    ),
    // Vota: 94 words (vakua, lexika, neutra, perinea, verba, visa).
    declension!("vota", 94, "~a";
        nominative: "~a|~en";
        genitive: "~a|~en";
        dative: "~a|~en";
        accusative: "~a|~en";
    ),
    // Obmänner: 65 words (ackermänner, gefolgsmänner, kaufmänner, fuhrmänner, schutzmänner, mittelsmänner).
    declension!("obmänner", 65, "~männ~r";
        nominative: "~männ~r|~l~ute";
        genitive: "~männ~r|~l~ute";
        dative: "~männ~rn|~l~uten";
        accusative: "~männ~r|~l~ute";
    ),
    // Deltas: 63 words (balalaikas, deltas, juntas, swastikas, mafias, plazentas).
    declension!("deltas", 63, "~as";
        nominative: "~as|~en";
        genitive: "~as|~en";
        dative: "~as|~en";
        accusative: "~as|~en";
    ),
    // Fäden: 59 words (fäden, sandkästen, rollläden, fensterläden, brutkästen, glühfäden).
    declension!("fäden", 59, "~ä~";
        nominative: "~ä~|~a~";
        genitive: "~ä~|~a~";
        dative: "~ä~|~a~";
        accusative: "~ä~|~a~";
    ),
    // Kommandos: 55 words (proseccos, cellos, cembalos, ghettos, torsos, kommandos).
    declension!("kommandos", 55, "~os";
        nominative: "~os|~i";
        genitive: "~os|~i";
        dative: "~os|~i";
        accusative: "~os|~i";
    ),
    // Omen: 54 words (possessivpronomen, abdomen, omen, pronomen, sporen, skioptiken).
    declension!("omen", 54, "~e~";
        nominative: "~e~|~i~a";
        genitive: "~e~|~i~a";
        dative: "~e~|~i~a";
        accusative: "~e~|~i~a";
    ),
    // Pfosten: 48 words (spannungsbogen, regenbogen, pfosten, fragebogen, triumphbogen, lichtbogen).
    declension!("pfosten", 48, "~o~";
        nominative: "~o~|~ö~";
        genitive: "~o~|~ö~";
        dative: "~o~|~ö~";
        accusative: "~o~|~ö~";
    ),
    // Pläne: 46 words (säue, generäle, hämmer, pläne, hähne, stähle).
    declension!("pläne", 46, "~ä~";
        nominative: "~ä~|~a~";
        genitive: "~ä~|~a~";
        dative: "~ä~n|~a~n";
        accusative: "~ä~|~a~";
    ),
    // Polster: 45 words (roste, rosse, polster, schlote, walrosse, herkunftsorte).
    declension!("polster", 45, "~o~";
        nominative: "~o~|~ö~r";
        genitive: "~o~|~ö~r";
        dative: "~o~n|~ö~rn";
        accusative: "~o~|~ö~r";
    ),
    // Verbunde: 42 words (buge, funde, hufe, schlucke, mundschutze, puffe).
    declension!("verbunde", 42, "~u~";
        nominative: "~u~|~ü~";
        genitive: "~u~|~ü~";
        dative: "~u~n|~ü~n";
        accusative: "~u~|~ü~";
    ),
    // Grüfte: 39 words (süchte, schlünde, stürze, armbrüste, unterschlüpfe, grüfte).
    declension!("grüfte", 39, "~ü~";
        nominative: "~ü~|~u~";
        genitive: "~ü~|~u~";
        dative: "~ü~n|~u~n";
        accusative: "~ü~|~u~";
    ),
    // Primi: 35 words (phalli, radii, tempi, thesauri, mafiosi, kubi).
    declension!("primi", 35, "~i";
        nominative: "~i";
        genitive: "~i";
        dative: "~i";
        accusative: "~i";
    ),
    // Loge: 32 words (genozide, ausbaue, monopole, kleinode, bacchanale, lithiumminerale).
    declension!("loge", 32, "~~";
        nominative: "~~|~i~n";
        genitive: "~~|~i~n";
        dative: "~~n|~i~n";
        accusative: "~~|~i~n";
    ),
    // Wörter: 26 words (wörter, wanderwörter, sprichwörter, schlagwörter, stichwörter, kosewörter).
    declension!("wörter", 26, "~ö~r";
        nominative: "~ö~r|~o~";
        genitive: "~ö~r|~o~";
        dative: "~ö~rn|~o~n";
        accusative: "~ö~r|~o~";
    ),
    // Couchs: 22 words (couchs, matchs, kautschs, bettcouchs, putensandwichs, schinkensandwichs).
    declension!("couchs", 22, "~s";
        nominative: "~s|~es|~e";
        genitive: "~es|~e|~s";
        dative: "~s|~es|~en";
        accusative: "~s|~es|~e";
    ),
    // Blöcke: 21 words (blöcke, häuserblöcke, schmöcke, fanblöcke, collegeblöcke, kollegblöcke).
    declension!("blöcke", 21, "~ö~e";
        nominative: "~ö~e|~o~s";
        genitive: "~ö~e|~o~s";
        dative: "~ö~en|~o~s";
        accusative: "~ö~e|~o~s";
    ),
    // Seeleute: 21 words (landsleute, feuerwehrleute, geschäftsleute, seeleute, fährleute, hauptleute).
    declension!("seeleute", 21, "~l~ute";
        nominative: "~l~ute|~männ~r";
        genitive: "~l~ute|~männ~r";
        dative: "~l~uten|~männ~rn";
        accusative: "~l~ute|~männ~r";
    ),
    // Mähler: 18 words (mähler, valenzbänder, haarbänder, muselmänner, henkersmähler, abendmähler).
    declension!("mähler", 18, "~ä~r";
        nominative: "~ä~r|~a~";
        genitive: "~ä~r|~a~";
        dative: "~ä~rn|~a~n";
        accusative: "~ä~r|~a~";
    ),
    // Männchen: 18 words (männchen, orthographien, liedchen, lämmchen, hühnchen, lichtchen).
    declension!("männchen", 18, "~~";
        nominative: "~~|~er~";
        genitive: "~~|~er~";
        dative: "~~|~er~";
        accusative: "~~|~er~";
    ),
    // Lichter: 17 words (lichter, regimenter, schilder, irrlichter, ultraviolettlichter, angesichter).
    declension!("lichter", 17, "~r";
        nominative: "~r|~";
        genitive: "~r|~";
        dative: "~rn|~n";
        accusative: "~r|~";
    ),
    // Haruspexe: 16 words (haruspexe, verhaltenskodexe, kodexe, brechungsindexe, aktienindexe, ehrenkodexe).
    declension!("haruspexe", 16, "~~xe";
        nominative: "~~xe";
        genitive: "~~xe";
        dative: "~~xen";
        accusative: "~~xe";
    ),
    // Fisken: 14 words (globen, hibisken, listerien, konklaven, fisken, strumen).
    declension!("fisken", 14, "~~n";
        nominative: "~~n";
        genitive: "~~n";
        dative: "~~n";
        accusative: "~~n";
    ),
    // Hiatus: 14 words (hiatus, palpus, maxillarpalpus, tribus, dekubitus, eukalyptus).
    declension!("hiatus", 14, "~us";
        nominative: "~us";
        genitive: "~us";
        dative: "~us";
        accusative: "~us";
    ),
    // Adverbien: 12 words (mineralien, adverbien, reptilien, privilegien, konzilien, sanktuarien).
    declension!("adverbien", 12, "~i~n";
        nominative: "~i~n|~~";
        genitive: "~i~n|~~";
        dative: "~i~n|~~n";
        accusative: "~i~n|~~";
    ),
    // Stimulanzien: 11 words (adstrigenzien, stimulanzien, formanzien, adjuvanzien, akzidenzien, semideponenzien).
    declension!("stimulanzien", 11, "~z~en";
        nominative: "~z~en|~t~a";
        genitive: "~z~en|~t~a";
        dative: "~z~en|~t~a";
        accusative: "~z~en|~t~a";
    ),
    // Hobbys: 10 words (tommys, hobbys, partys, rowdys, shantys, coronapartys).
    declension!("hobbys", 10, "~y~";
        nominative: "~y~|~ie~";
        genitive: "~y~|~ie~";
        dative: "~y~|~ie~";
        accusative: "~y~|~ie~";
    ),
    // Eigentümer: 10 words (tücher, eigentümer, vormünder, käsetücher, stichpünkter, eigenthümer).
    declension!("eigentümer", 10, "~ü~r";
        nominative: "~ü~r|~u~";
        genitive: "~ü~r|~u~";
        dative: "~ü~rn|~u~n";
        accusative: "~ü~r|~u~";
    ),
    // Mutä: 9 words (präantepänultimä, liquidä, mutä, pänultimä, antepänultimä, antepaenultimä).
    declension!("mutä", 9, "~ä";
        nominative: "~ä|~en";
        genitive: "~ä|~en";
        dative: "~ä|~en";
        accusative: "~ä|~en";
    ),
    // Bögen: 8 words (bögen, reflexbögen, papierbögen, geigenbögen, pergamentbögen, briefbögen).
    declension!("bögen", 8, "~ö~";
        nominative: "~ö~|~o~";
        genitive: "~ö~|~o~";
        dative: "~ö~|~o~";
        accusative: "~ö~|~o~";
    ),
    // Stigmata: 8 words (schemata, traumata, stigmata, chiasmata, xerodermata, leitthemata).
    declension!("stigmata", 8, "~ata";
        nominative: "~ata|~en";
        genitive: "~ata|~en";
        dative: "~ata|~en";
        accusative: "~ata|~en";
    ),
    // Ypsilons: 8 words (ypsilons, semikolons, kolons, telencephalons, diencephalons, mesencephalons).
    declension!("ypsilons", 8, "~ons";
        nominative: "~ons|~a";
        genitive: "~ons|~a";
        dative: "~ons|~a";
        accusative: "~ons|~a";
    ),
    // Trilemmata: 8 words (klimata, trilemmata, raumklimata, konsumklimata, wirtschaftsklimata, investitionsklimata).
    declension!("trilemmata", 8, "~ta";
        nominative: "~ta|~s|~te";
        genitive: "~ta|~s|~te";
        dative: "~ta|~s|~ten";
        accusative: "~ta|~s|~te";
    ),
    // Nationalparks: 8 words (nationalparks, kurparks, naturparks, walkmans, bärenparks, landschaftsparks).
    declension!("nationalparks", 8, "~a~s";
        nominative: "~a~s|~a~e|~ä~e";
        genitive: "~a~s|~a~e|~ä~e";
        dative: "~a~s|~a~en|~ä~en";
        accusative: "~a~s|~a~e|~ä~e";
    ),
    // Nomina: 8 words (nomina, omiramba, examina, wurzelnomina, bitumina, notexamina).
    declension!("nomina", 8, "~i~a";
        nominative: "~i~a|~e~";
        genitive: "~i~a|~e~";
        dative: "~i~a|~e~";
        accusative: "~i~a|~e~";
    ),
    // Risikos: 8 words (risikos, krebsrisikos, verletzungsrisikos, spielrisikos, gefährdungsrisikos, armutsrisikos).
    declension!("risikos", 8, "~i~os";
        nominative: "~i~os|~i~en|~~en";
        genitive: "~i~os|~i~en|~~en";
        dative: "~i~os|~i~en|~~en";
        accusative: "~i~os|~i~en|~~en";
    ),
    // Crashs: 8 words (crashs, slashs, ranchs, spielekanons, fächerkanons, wertekanons).
    declension!("crashs", 8, "~~";
        nominative: "~~|~e~";
        genitive: "~e~|~~";
        dative: "~~|~e~";
        accusative: "~~|~e~";
    ),
    // Thoraxe: 7 words (sphinxe, thoraxe, archaeopteryxe, cephalothoraxe, zephalothoraxe, hemithoraxe).
    declension!("thoraxe", 7, "~x~";
        nominative: "~x~|~c~s";
        genitive: "~x~|~c~s";
        dative: "~x~n|~c~s";
        accusative: "~x~|~c~s";
    ),
    // Amtfrauen: 7 words (kamerafrauen, amtfrauen, fuhrfrauen, instfrauen, kirchenfrauen, ss-frauen).
    declension!("amtfrauen", 7, "~fra~~n";
        nominative: "~fra~~n|~le~t~";
        genitive: "~fra~~n|~le~t~";
        dative: "~fra~~n|~le~t~n";
        accusative: "~fra~~n|~le~t~";
    ),
    // Häser: 6 words (skier, wasserskier, schier, häßer, häser, narrenhäser).
    declension!("häser", 6, "~er";
        nominative: "~er|~";
        genitive: "~er|~";
        dative: "~ern|~";
        accusative: "~er|~";
    ),
    // Tussis: 6 words (taxis, physalis, tussis, sozis, anrufsammeltaxis, anrufersammeltaxis).
    declension!("tussis", 6, "~is";
        nominative: "~is|~en";
        genitive: "~is|~en";
        dative: "~is|~en";
        accusative: "~is|~en";
    ),
    // Flöße: 6 words (tenöre, flöße, erzherzöge, heldentenöre, flösse, a-cappella-chöre).
    declension!("flöße", 6, "~ö~";
        nominative: "~ö~|~o~";
        genitive: "~ö~|~o~";
        dative: "~ö~n|~o~n";
        accusative: "~ö~|~o~";
    ),
    // Bauten: 6 words (bauten, reaktorbauten, neidbauten, überbauten, dachausbauten, ausbauten).
    declension!("bauten", 6, "~t~n";
        nominative: "~t~n|~~";
        genitive: "~t~n|~~";
        dative: "~t~n|~~n";
        accusative: "~t~n|~~";
    ),
    // Krankheitsrisiken: 6 words (risiken, gesundheitsrisiken, krankheitsrisiken, berufsrisiken, entdeckungsrisiken, hautkrebsrisiken).
    declension!("krankheitsrisiken", 6, "~i~en";
        nominative: "~i~en|~i~os|~~en";
        genitive: "~i~en|~i~os|~~en";
        dative: "~i~en|~i~os|~~en";
        accusative: "~i~en|~i~os|~~en";
    ),
    // Präsentia: 6 words (präsentia, deponentia, präteritopräsentia, präterito-präsentia, vesikantia, antikoagulantia).
    declension!("präsentia", 6, "~t~a";
        nominative: "~t~a|~z~en";
        genitive: "~t~a|~z~en";
        dative: "~t~a|~z~en";
        accusative: "~t~a|~z~en";
    ),
    // Faktotums: 6 words (magnums, cingulums, zingulums, faktotums, basilikums, ultimatums).
    declension!("faktotums", 6, "~ums";
        nominative: "~ums";
        genitive: "~ums";
        dative: "~ums";
        accusative: "~ums";
    ),
    // Romnija: 6 words (romnija, pronominaladverbien, präpositionaladverbien, präverbien, prä-verbien, brandrisiken).
    declension!("romnija", 6, "~i~";
        nominative: "~i~|~~";
        genitive: "~i~|~~";
        dative: "~i~|~~";
        accusative: "~i~|~~";
    ),
    // Windparks: 6 words (windparks, solarparks, offshorewindparks, privatparks, abenteuerparks, industrieparks).
    declension!("windparks", 6, "~a~s";
        nominative: "~a~s|~a~e|~ä~e";
        genitive: "~a~e|~ä~e|~a~s";
        dative: "~a~s|~a~en|~ä~en";
        accusative: "~a~s|~a~e|~ä~e";
    ),
    // Gläser: 5 words (gläser, männer, fässer, käffer, heiermänner).
    declension!("gläser", 5, "~ä~er";
        nominative: "~ä~er|~a~";
        genitive: "~ä~er|~a~";
        dative: "~ä~ern|~a~";
        accusative: "~ä~er|~a~";
    ),
    // Lunches: 5 words (lunches, cicerones, coaches, jarmulkes, kautsches).
    declension!("lunches", 5, "~es";
        nominative: "~es";
        genitive: "~es";
        dative: "~es|~s|~en";
        accusative: "~es";
    ),
    // Konusse: 5 words (diskusse, nimbusse, noniusse, konusse, werkzeugkonusse).
    declension!("konusse", 5, "~uss~";
        nominative: "~uss~|~~n";
        genitive: "~uss~|~~n";
        dative: "~uss~n|~~n";
        accusative: "~uss~|~~n";
    ),
    // Kaffe: 5 words (kaffe, kuhkaffe, provinzkaffe, dreckskaffe, scheißkaffe).
    declension!("kaffe", 5, "~a~e";
        nominative: "~a~e|~a~s|~ä~er";
        genitive: "~a~e|~a~s|~ä~er";
        dative: "~a~en|~a~s|~ä~ern";
        accusative: "~a~e|~a~s|~ä~er";
    ),
    // Autoatlasse: 5 words (sprachatlasse, autoatlasse, weltatlasse, stadtatlasse, reiseatlasse).
    declension!("autoatlasse", 5, "~ss~";
        nominative: "~ss~|~nt~n";
        genitive: "~ss~|~nt~n";
        dative: "~ss~n|~nt~n";
        accusative: "~ss~|~nt~n";
    ),
    // Cherubim: 4 words (sephardim, aschkenasim, cherubim, kerubim).
    declension!("cherubim", 4, "~m";
        nominative: "~m";
        genitive: "~m";
        dative: "~m";
        accusative: "~m";
    ),
    // Abakusse: 4 words (abakusse, bonusse, kallusse, branchenprimusse).
    declension!("abakusse", 4, "~usse";
        nominative: "~usse";
        genitive: "~usse";
        dative: "~ussen";
        accusative: "~usse";
    ),
    // Embryonen: 4 words (embryonen, rabbinen, geckonen, schafsembryonen).
    declension!("embryonen", 4, "~nen";
        nominative: "~nen|~s";
        genitive: "~nen|~s";
        dative: "~nen|~s";
        accusative: "~nen|~s";
    ),
    // Indizes: 4 words (indizes, kodizes, saprobienindizes, pressekodizes).
    declension!("indizes", 4, "~iz~s";
        nominative: "~iz~s|~~xe";
        genitive: "~iz~s|~~xe";
        dative: "~iz~s|~~xen";
        accusative: "~iz~s|~~xe";
    ),
    // Burgen: 4 words (schnuren, burgen, bruchen, bekandtnussen).
    declension!("burgen", 4, "~u~n";
        nominative: "~u~n|~ü~";
        genitive: "~u~n|~ü~";
        dative: "~u~n|~ü~n";
        accusative: "~u~n|~ü~";
    ),
    // Moose: 4 words (moose, lebermoose, schlafmoose, wikingerboote).
    declension!("moose", 4, "~oo~";
        nominative: "~oo~|~ö~r";
        genitive: "~oo~|~ö~r";
        dative: "~oo~n|~ö~rn";
        accusative: "~oo~|~ö~r";
    ),
    // Aliasse: 4 words (malusse, campusse, rizinusse, aliasse).
    declension!("aliasse", 4, "~se";
        nominative: "~se|~";
        genitive: "~se|~";
        dative: "~sen|~";
        accusative: "~se|~";
    ),
    // Bitoks: 4 words (superblocks, kintopps, orchestrions, bitoks).
    declension!("bitoks", 4, "~o~s";
        nominative: "~o~s";
        genitive: "~o~s";
        dative: "~o~s";
        accusative: "~o~s";
    ),
    // Pastoren: 3 words (pastoren, schlehdornen, schoßen).
    declension!("pastoren", 3, "~o~n";
        nominative: "~o~n|~ö~";
        genitive: "~o~n|~ö~";
        dative: "~o~n|~ö~n";
        accusative: "~o~n|~ö~";
    ),
    // Penisse: 3 words (penisse, saxophonpenisse, hemipenisse).
    declension!("penisse", 3, "~iss~";
        nominative: "~iss~|~~s";
        genitive: "~iss~|~~s";
        dative: "~iss~n|~~s";
        accusative: "~iss~|~~s";
    ),
    // Auloi: 3 words (topoi, auloi, tholoi).
    declension!("auloi", 3, "~oi";
        nominative: "~oi|~en";
        genitive: "~oi|~en";
        dative: "~oi|~en";
        accusative: "~oi|~en";
    ),
    // Ciabatte: 3 words (ciabatte, cupfinale, dynatrone).
    declension!("ciabatte", 3, "~e";
        nominative: "~e|~s";
        genitive: "~e|~s";
        dative: "~e|~s";
        accusative: "~e|~s";
    ),
    // Forceps: 3 words (forzeps, forceps, paterfamilias).
    declension!("forceps", 3, "~e~~";
        nominative: "~e~~|~i~e~";
        genitive: "~e~~|~i~e~";
        dative: "~e~~|~i~e~";
        accusative: "~e~~|~i~e~";
    ),
    // Pontifizes: 3 words (synthese-indizes, syntheseindizes, pontifizes).
    declension!("pontifizes", 3, "~z~";
        nominative: "~z~|~c~";
        genitive: "~z~|~c~";
        dative: "~z~|~c~";
        accusative: "~z~|~c~";
    ),
    // Chassidim: 3 words (kibbuzim, kubbuzim, chassidim).
    declension!("chassidim", 3, "~im";
        nominative: "~im|~e";
        genitive: "~im|~e";
        dative: "~im|~en";
        accusative: "~im|~e";
    ),
    // Suser: 3 words (fußgestelle, blutritte, suser).
    declension!("suser", 3, "~";
        nominative: "~";
        genitive: "~";
        dative: "~n";
        accusative: "~n";
    ),
    // Narrenattribute: 3 words (narrenattribute, südafrikaurlaube, fischerei-vereine).
    declension!("narrenattribute", 3, "~e";
        nominative: "~e";
        genitive: "~e";
        dative: "~en";
        accusative: "~";
    ),
];
