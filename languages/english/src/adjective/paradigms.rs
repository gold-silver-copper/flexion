//! The declensions, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one declension: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `adjective` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal;
     adjective: $c0:tt, $c1:tt, $c2:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[
                flexion::form!($c0),
                flexion::form!($c1),
                flexion::form!($c2),
            ]),
        }
    };
}

/// Every declension learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Declension {
    /// Like `good`: 40,940 words.
    Good,
    /// Like `solid`: 5,087 words.
    Solid,
    /// Like `trivial`: 2,559 words.
    Trivial,
    /// Like `major`: 2,117 words.
    Major,
    /// Like `external`: 2,023 words.
    External,
    /// Like `open`: 1,655 words.
    Open,
    /// Like `heavy`: 1,475 words.
    Heavy,
    /// Like `round`: 1,151 words.
    Round,
    /// Like `regular`: 927 words.
    Regular,
    /// Like `stacked`: 736 words.
    Stacked,
    /// Like `mental`: 541 words.
    Mental,
    /// Like `soft`: 370 words.
    Soft,
    /// Like `modal`: 330 words.
    Modal,
    /// Like `free`: 203 words.
    Free,
    /// Like `organic`: 174 words.
    Organic,
    /// Like `ayey`: 107 words.
    Ayey,
    /// Like `easy`: 62 words.
    Easy,
    /// Like `mortal`: 50 words.
    Mortal,
    /// Like `fiery`: 40 words.
    Fiery,
    /// Like `oscarworthy`: 35 words.
    Oscarworthy,
    /// Like `rich`: 34 words.
    Rich,
    /// Like `moot`: 32 words.
    Moot,
    /// Like `ostentatious`: 31 words.
    Ostentatious,
    /// Like `simple`: 28 words.
    Simple,
    /// Like `motile`: 25 words.
    Motile,
    /// Like `memorial`: 25 words.
    Memorial,
    /// Like `mosey`: 22 words.
    Mosey,
    /// Like `right`: 19 words.
    Right,
    /// Like `slim`: 13 words.
    Slim,
    /// Like `otiose`: 13 words.
    Otiose,
    /// Like `awful`: 13 words.
    Awful,
    /// Like `reeded`: 12 words.
    Reeded,
    /// Like `momental`: 12 words.
    Momental,
    /// Like `ooc`: 12 words.
    Ooc,
    /// Like `upstream`: 11 words.
    Upstream,
    /// Like `oedemic`: 11 words.
    Oedemic,
    /// Like `ottawian`: 10 words.
    Ottawian,
    /// Like `flat`: 9 words.
    Flat,
    /// Like `eevn`: 9 words.
    Eevn,
    /// Like `switchy`: 9 words.
    Switchy,
    /// Like `big`: 8 words.
    Big,
    /// Like `advanced`: 8 words.
    Advanced,
    /// Like `mellow`: 8 words.
    Mellow,
    /// Like `motoric`: 8 words.
    Motoric,
    /// Like `meetable`: 8 words.
    Meetable,
    /// Like `oodthirsty`: 8 words.
    Oodthirsty,
    /// Like `sad`: 7 words.
    Sad,
    /// Like `mid`: 7 words.
    Mid,
    /// Like `orotund`: 7 words.
    Orotund,
    /// Like `orectic`: 7 words.
    Orectic,
    /// Like `ossified`: 6 words.
    Ossified,
    /// Like `inner`: 6 words.
    Inner,
    /// Like `glib`: 5 words.
    Glib,
    /// Like `otomian`: 5 words.
    Otomian,
    /// Like `memetic`: 5 words.
    Memetic,
    /// Like `mossful`: 5 words.
    Mossful,
    /// Like `evil`: 4 words.
    Evil,
    /// Like `moreish`: 4 words.
    Moreish,
    /// Like `tonguey`: 4 words.
    Tonguey,
    /// Like `morrisian`: 4 words.
    Morrisian,
    /// Like `otesque`: 3 words.
    Otesque,
    /// Like `flip`: 3 words.
    Flip,
    /// Like `thin`: 3 words.
    Thin,
    /// Like `moronic`: 3 words.
    Moronic,
    /// Like `damned`: 3 words.
    Damned,
    /// Like `motty`: 3 words.
    Motty,
    /// Like `mommish`: 3 words.
    Mommish,
    /// Like `ossomy`: 3 words.
    Ossomy,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 68] = [
        Declension::Good,
        Declension::Solid,
        Declension::Trivial,
        Declension::Major,
        Declension::External,
        Declension::Open,
        Declension::Heavy,
        Declension::Round,
        Declension::Regular,
        Declension::Stacked,
        Declension::Mental,
        Declension::Soft,
        Declension::Modal,
        Declension::Free,
        Declension::Organic,
        Declension::Ayey,
        Declension::Easy,
        Declension::Mortal,
        Declension::Fiery,
        Declension::Oscarworthy,
        Declension::Rich,
        Declension::Moot,
        Declension::Ostentatious,
        Declension::Simple,
        Declension::Motile,
        Declension::Memorial,
        Declension::Mosey,
        Declension::Right,
        Declension::Slim,
        Declension::Otiose,
        Declension::Awful,
        Declension::Reeded,
        Declension::Momental,
        Declension::Ooc,
        Declension::Upstream,
        Declension::Oedemic,
        Declension::Ottawian,
        Declension::Flat,
        Declension::Eevn,
        Declension::Switchy,
        Declension::Big,
        Declension::Advanced,
        Declension::Mellow,
        Declension::Motoric,
        Declension::Meetable,
        Declension::Oodthirsty,
        Declension::Sad,
        Declension::Mid,
        Declension::Orotund,
        Declension::Orectic,
        Declension::Ossified,
        Declension::Inner,
        Declension::Glib,
        Declension::Otomian,
        Declension::Memetic,
        Declension::Mossful,
        Declension::Evil,
        Declension::Moreish,
        Declension::Tonguey,
        Declension::Morrisian,
        Declension::Otesque,
        Declension::Flip,
        Declension::Thin,
        Declension::Moronic,
        Declension::Damned,
        Declension::Motty,
        Declension::Mommish,
        Declension::Ossomy,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 68] = [
    // Good: 40,940 words (nonsense, polysemic, adjectival, fabaceous, fabian, abandoned).
    declension!("good", 40940, "~";
        adjective: "~", "more ~", "most ~";
    ),
    // Solid: 5,087 words (substantive, singular, secret, skillful, silver, saxon).
    declension!("solid", 5087, "~~";
        adjective: "~~", "more ~~", "mo~t s~";
    ),
    // Trivial: 2,559 words (tomato, thai, turquoise, teal, taupe, taiwanese).
    declension!("trivial", 2559, "~~";
        adjective: "~~", "more ~~", "mos~ t~";
    ),
    // Major: 2,117 words (music, minuscule, magenta, musical, masculine, maroon).
    declension!("major", 2117, "~~";
        adjective: "~~", "~ore m~", "~ost m~";
    ),
    // External: 2,023 words (english, epizootic, european, epidemic, ebony, emerald).
    declension!("external", 2023, "~~";
        adjective: "~~", "mor~ e~", "most ~~";
    ),
    // Open: 1,655 words (obfuscate, open, offset, olive, overweight, oa).
    declension!("open", 1655, "~~";
        adjective: "~~", "m~re o~", "m~st o~";
    ),
    // Heavy: 1,475 words (country, early, baby, yeasty, angry, pinky).
    declension!("heavy", 1475, "~y";
        adjective: "~y", "~ier", "~iest";
    ),
    // Round: 1,151 words (raven, russian, rational, room, russet, ruby).
    declension!("round", 1151, "~~";
        adjective: "~~", "mo~e r~", "most ~~";
    ),
    // Regular: 927 words (reminiscent, regular, remote, redhanded, relaxed, reprehensible).
    declension!("regular", 927, "~~";
        adjective: "~~", "mo~ re~", "most ~~";
    ),
    // Stacked: 736 words (stylized, street, stuck, stink, stank, strewn).
    declension!("stacked", 736, "~~";
        adjective: "~~", "more ~~", "mo~ st~";
    ),
    // Mental: 541 words (meticulous, mexican, mental, melancholy, melburnian, metal).
    declension!("mental", 541, "~~~";
        adjective: "~~~", "~or~ me~", "~ost m~~";
    ),
    // Soft: 370 words (august, abject, odd, absent, acid, slow).
    declension!("soft", 370, "~";
        adjective: "~", "~er", "~est";
    ),
    // Modal: 330 words (mobile, money, modest, mongolian, mozambican, monastic).
    declension!("modal", 330, "~~";
        adjective: "~~", "~re mo~", "~st mo~";
    ),
    // Free: 203 words (free, able, abstruse, false, welcome, beige).
    declension!("free", 203, "~";
        adjective: "~", "~r", "~st";
    ),
    // Organic: 174 words (ordinate, original, ordinary, orthodox, ornamental, orbed).
    declension!("organic", 174, "~~~";
        adjective: "~~~", "m~~e or~", "m~st o~~";
    ),
    // Ayey: 107 words (smiley, horsey, dykey, limey, cagey, dickey).
    declension!("ayey", 107, "~~y";
        adjective: "~~y", "~i~r", "~i~st";
    ),
    // Easy: 62 words (easy, rocky, shy, moldy, mushy, shapely).
    declension!("easy", 62, "~~y";
        adjective: "~~y", "~~ier", "~~iest";
    ),
    // Mortal: 50 words (moral, mortal, mordant, moriform, morbid, morto).
    declension!("mortal", 50, "~~~";
        adjective: "~~~", "~~e mor~", "~st mo~~";
    ),
    // Fiery: 40 words (fiery, brotherly, likely, cringeworthy, priestly, namely).
    declension!("fiery", 40, "~y";
        adjective: "~y", "more ~y|~ier", "most ~y|~iest";
    ),
    // Oscarworthy: 35 words (oscular, oscillatory, osm, oscillating, osculating, osculatory).
    declension!("oscarworthy", 35, "~~~";
        adjective: "~~~", "m~re o~~", "m~~t os~";
    ),
    // Rich: 34 words (skew, stay, swear, rotten, real, rigid).
    declension!("rich", 34, "~~";
        adjective: "~~", "~~er", "~~est";
    ),
    // Moot: 32 words (moot, moonbat, mooted, moonlit, moorish, moonish).
    declension!("moot", 32, "~~~";
        adjective: "~~~", "~re m~o~", "~st m~o~";
    ),
    // Ostentatious: 31 words (ostensible, ostentatious, ostracized, ostrobogulous, osteoarthritic, ostensive).
    declension!("ostentatious", 31, "~~~";
        adjective: "~~~", "m~re o~~", "m~~ ost~";
    ),
    // Simple: 28 words (minute, mauve, male, safe, simple, true).
    declension!("simple", 28, "~~";
        adjective: "~~", "~~r", "~~st";
    ),
    // Motile: 25 words (motherly, motile, motivating, motherish, motivational, mothered).
    declension!("motile", 25, "~~~";
        adjective: "~~~", "~re mo~~", "~s~ mot~";
    ),
    // Memorial: 25 words (memorial, membranous, memorable, memristive, memorious, memorizable).
    declension!("memorial", 25, "~~~~";
        adjective: "~~~~", "~or~ ~em~", "~ost m~~~";
    ),
    // Mosey: 22 words (moslem, mosey, mosaiclike, mosquelike, mosquitocidal, mosquitoed).
    declension!("mosey", 22, "~~~";
        adjective: "~~~", "~re mo~~", "~~t mos~";
    ),
    // Right: 19 words (left, right, gone, inset, downstairs, front).
    declension!("right", 19, "~";
        adjective: "~", "further ~|farther ~", "furthest ~|farthest ~";
    ),
    // Slim: 13 words (slim, bum, prim, clam, dim, rum).
    declension!("slim", 13, "~";
        adjective: "~", "~mer", "~mest";
    ),
    // Otiose: 13 words (otiose, otherworldly, otaheite, otherworldy, otherworldish, otherly).
    declension!("otiose", 13, "~~~";
        adjective: "~~~", "m~re o~~", "m~s~ ot~";
    ),
    // Awful: 13 words (painful, awful, wonderful, joyful, woeful, harmful).
    declension!("awful", 13, "~";
        adjective: "~", "~ler|more ~", "~lest|most ~";
    ),
    // Reeded: 12 words (reeded, reeling, reexamined, reefed, reedlike, reezed).
    declension!("reeded", 12, "~~~";
        adjective: "~~~", "mo~ r~e~", "most ~~~";
    ),
    // Momental: 12 words (momentary, momentous, momentany, momentane, momental, momly).
    declension!("momental", 12, "~~~";
        adjective: "~~~", "~re ~om~", "~st ~om~";
    ),
    // Ooc: 12 words (ooid, oolitic, oofy, oofless, oophytic, ooc).
    declension!("ooc", 12, "~~~";
        adjective: "~~~", "m~re ~o~", "m~st ~o~";
    ),
    // Upstream: 11 words (upstairs, upriver, uphill, upstream, uptown, upwind).
    declension!("upstream", 11, "~~";
        adjective: "~~", "f~rther u~|farther ~~", "f~rthest u~|farthest ~~";
    ),
    // Oedemic: 11 words (oedemic, oenological, oedemateous, oedematic, oedematous, oenophilic).
    declension!("oedemic", 11, "~~~";
        adjective: "~~~", "m~r~ oe~", "m~st o~~";
    ),
    // Ottawian: 10 words (ottawan, otterlike, ottery, otterish, ottomanic, ottawian).
    declension!("ottawian", 10, "~~~~";
        adjective: "~~~~", "m~re o~~~", "m~s~ o~t~";
    ),
    // Flat: 9 words (shit, hot, fit, wet, flat, jet).
    declension!("flat", 9, "~";
        adjective: "~", "~ter", "~test";
    ),
    // Eevn: 9 words (eeyorish, eensy, eesome, eellike, eevn, eentsy).
    declension!("eevn", 9, "~~~";
        adjective: "~~~", "mor~ ~e~", "most ~~~";
    ),
    // Switchy: 9 words (sisterly, spluttery, schizy, swimmy, schizzy, sweepy).
    declension!("switchy", 9, "~~y";
        adjective: "~~y", "more ~~y|~~ier", "mo~t s~y|~~iest";
    ),
    // Big: 8 words (big, bog, mug, snug, brag, smug).
    declension!("big", 8, "~";
        adjective: "~", "~ger", "~gest";
    ),
    // Advanced: 8 words (dumbass, advanced, aged, lightweight, hardheaded, hardhearted).
    declension!("advanced", 8, "~~";
        adjective: "~~", "more ~~", "most ~~";
    ),
    // Mellow: 8 words (mellow, reef, reem, oopid, eet, oro).
    declension!("mellow", 8, "~~~";
        adjective: "~~~", "~~~er", "~~~est";
    ),
    // Motoric: 8 words (motormouthed, motoric, motorable, motorik, motorlike, motogenic).
    declension!("motoric", 8, "~~~~";
        adjective: "~~~~", "~re mo~~~", "~s~ m~to~";
    ),
    // Meetable: 8 words (meetable, meetinglike, meedful, meekish, meedless, meedy).
    declension!("meetable", 8, "~~~~";
        adjective: "~~~~", "~or~ m~e~", "~ost m~~~";
    ),
    // Oodthirsty: 8 words (orly, oodthirsty, eppy, oopy, eechworthy, reetworthy).
    declension!("oodthirsty", 8, "~~~y";
        adjective: "~~~y", "~~~ier", "~~~iest";
    ),
    // Sad: 7 words (red, dud, glad, rad, sad, mad).
    declension!("sad", 7, "~";
        adjective: "~", "~der", "~dest";
    ),
    // Mid: 7 words (first, ben, middle, fore, mid, rightern).
    declension!("mid", 7, "~";
        adjective: "~", "~ner", "~most";
    ),
    // Orotund: 7 words (orotund, orodispersable, orodispersible, orographical, orophilous, orosian).
    declension!("orotund", 7, "~~~~";
        adjective: "~~~~", "m~~e ~ro~", "m~st o~~~";
    ),
    // Orectic: 7 words (orexigenic, orexinergic, orectic, orective, oresteian, orestean).
    declension!("orectic", 7, "~~~";
        adjective: "~~~", "m~~ ore~", "m~st o~~";
    ),
    // Ossified: 6 words (ossified, ossiferous, ossianic, ossiform, ossianesque, ossom).
    declension!("ossified", 6, "~~~~";
        adjective: "~~~~", "m~re o~~~", "m~~t o~s~";
    ),
    // Inner: 6 words (upper, outer, inner, nether, nither, ither).
    declension!("inner", 6, "~";
        adjective: "~", "~more", "~most";
    ),
    // Glib: 5 words (fab, sib, drab, glib, slab).
    declension!("glib", 5, "~";
        adjective: "~", "~ber", "~best";
    ),
    // Otomian: 5 words (otogenic, ototoxic, otoprotective, otoneuroprotective, otomian).
    declension!("otomian", 5, "~~~~";
        adjective: "~~~~", "m~re o~~~", "m~s~ ~to~";
    ),
    // Memetic: 5 words (memetic, memelike, memetical, memeable, memeworthy).
    declension!("memetic", 5, "~~~~";
        adjective: "~~~~", "~or~ ~me~", "~ost m~~~";
    ),
    // Mossful: 5 words (mosslike, mossgrown, mossbacked, mossened, mossful).
    declension!("mossful", 5, "~~~~";
        adjective: "~~~~", "~re mo~~~", "~~t mo~s~";
    ),
    // Evil: 4 words (evil, mournful, mirthful, entiful).
    declension!("evil", 4, "~~";
        adjective: "~~", "~~ler", "~~lest";
    ),
    // Moreish: 4 words (moreish, moresque, moreauvian, morescan).
    declension!("moreish", 4, "~~~";
        adjective: "~~~", "~~ more~", "~st mo~~";
    ),
    // Tonguey: 4 words (ropey, tomatoey, spikey, tonguey).
    declension!("tonguey", 4, "~~~y";
        adjective: "~~~y", "~~i~r|more ~~~y", "~~i~st";
    ),
    // Morrisian: 4 words (morrisian, morrisseyesque, morrisonian, morrisean).
    declension!("morrisian", 4, "~~~~";
        adjective: "~~~~", "~~e mo~r~", "~st mo~~~";
    ),
    // Otesque: 3 words (orange, otesque, ophane).
    declension!("otesque", 3, "~~~";
        adjective: "~~~", "~~~r", "~~~st";
    ),
    // Flip: 3 words (crap, hip, flip).
    declension!("flip", 3, "~";
        adjective: "~", "~per", "~pest";
    ),
    // Thin: 3 words (thin, tan, wan).
    declension!("thin", 3, "~";
        adjective: "~", "~ner", "~nest";
    ),
    // Moronic: 3 words (morose, moronic, moronical).
    declension!("moronic", 3, "~~~~";
        adjective: "~~~~", "~~e m~ro~", "~st mo~~~";
    ),
    // Damned: 3 words (damned, darned, doggoned).
    declension!("damned", 3, "~e~";
        adjective: "~e~", "~e~er", "~e~est";
    ),
    // Motty: 3 words (mottled, motty, mottolike).
    declension!("motty", 3, "~~~~";
        adjective: "~~~~", "~re mo~~~", "~s~ mo~t~";
    ),
    // Mommish: 3 words (mommyish, mommylike, mommish).
    declension!("mommish", 3, "~~~~";
        adjective: "~~~~", "~re ~o~m~", "~st ~o~m~";
    ),
    // Ossomy: 3 words (mormony, otchety, ossomy).
    declension!("ossomy", 3, "~~~~y";
        adjective: "~~~~y", "~~~~ier", "~~~~iest";
    ),
];
