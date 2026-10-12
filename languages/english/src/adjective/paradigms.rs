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
/// `adjective` (pos, comp, sup)
macro_rules! declension {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     adjective: $c0:tt, $c1:tt, $c2:tt;
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
                flexion::cell!($c2), "\t",
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
    /// Like `heavy`: 1,776 words.
    Heavy,
    /// Like `open`: 1,655 words.
    Open,
    /// Like `round`: 1,151 words.
    Round,
    /// Like `regular`: 927 words.
    Regular,
    /// Like `stacked`: 736 words.
    Stacked,
    /// Like `soft`: 635 words.
    Soft,
    /// Like `mental`: 541 words.
    Mental,
    /// Like `modal`: 330 words.
    Modal,
    /// Like `organic`: 174 words.
    Organic,
    /// Like `clayey`: 116 words.
    Clayey,
    /// Like `shy`: 87 words.
    Shy,
    /// Like `rich`: 66 words.
    Rich,
    /// Like `mortal`: 50 words.
    Mortal,
    /// Like `awful`: 46 words.
    Awful,
    /// Like `fiery`: 40 words.
    Fiery,
    /// Like `oscarworthy`: 35 words.
    Oscarworthy,
    /// Like `moot`: 32 words.
    Moot,
    /// Like `ostentatious`: 31 words.
    Ostentatious,
    /// Like `motile`: 25 words.
    Motile,
    /// Like `memorial`: 25 words.
    Memorial,
    /// Like `mosey`: 22 words.
    Mosey,
    /// Like `right`: 19 words.
    Right,
    /// Like `otiose`: 13 words.
    Otiose,
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
    /// Like `eevn`: 9 words.
    Eevn,
    /// Like `switchy`: 9 words.
    Switchy,
    /// Like `advanced`: 8 words.
    Advanced,
    /// Like `motoric`: 8 words.
    Motoric,
    /// Like `meetable`: 8 words.
    Meetable,
    /// Like `mid`: 7 words.
    Mid,
    /// Like `orosian`: 7 words.
    Orosian,
    /// Like `orectic`: 7 words.
    Orectic,
    /// Like `ossified`: 6 words.
    Ossified,
    /// Like `inner`: 6 words.
    Inner,
    /// Like `otomian`: 5 words.
    Otomian,
    /// Like `memetic`: 5 words.
    Memetic,
    /// Like `mossful`: 5 words.
    Mossful,
    /// Like `moreish`: 4 words.
    Moreish,
    /// Like `tonguey`: 4 words.
    Tonguey,
    /// Like `morrisian`: 4 words.
    Morrisian,
    /// Like `flip`: 3 words.
    Flip,
    /// Like `thin`: 3 words.
    Thin,
    /// Like `evil`: 3 words.
    Evil,
    /// Like `moronic`: 3 words.
    Moronic,
    /// Like `mellow`: 3 words.
    Mellow,
    /// Like `damned`: 3 words.
    Damned,
    /// Like `motty`: 3 words.
    Motty,
    /// Like `mommish`: 3 words.
    Mommish,
    /// Like `slim`: 2 words.
    Slim,
}

impl Declension {
    /// Every declension, in order.
    pub const ALL: [Declension; 59] = [
        Declension::Good,
        Declension::Solid,
        Declension::Trivial,
        Declension::Major,
        Declension::External,
        Declension::Heavy,
        Declension::Open,
        Declension::Round,
        Declension::Regular,
        Declension::Stacked,
        Declension::Soft,
        Declension::Mental,
        Declension::Modal,
        Declension::Organic,
        Declension::Clayey,
        Declension::Shy,
        Declension::Rich,
        Declension::Mortal,
        Declension::Awful,
        Declension::Fiery,
        Declension::Oscarworthy,
        Declension::Moot,
        Declension::Ostentatious,
        Declension::Motile,
        Declension::Memorial,
        Declension::Mosey,
        Declension::Right,
        Declension::Otiose,
        Declension::Reeded,
        Declension::Momental,
        Declension::Ooc,
        Declension::Upstream,
        Declension::Oedemic,
        Declension::Ottawian,
        Declension::Eevn,
        Declension::Switchy,
        Declension::Advanced,
        Declension::Motoric,
        Declension::Meetable,
        Declension::Mid,
        Declension::Orosian,
        Declension::Orectic,
        Declension::Ossified,
        Declension::Inner,
        Declension::Otomian,
        Declension::Memetic,
        Declension::Mossful,
        Declension::Moreish,
        Declension::Tonguey,
        Declension::Morrisian,
        Declension::Flip,
        Declension::Thin,
        Declension::Evil,
        Declension::Moronic,
        Declension::Mellow,
        Declension::Damned,
        Declension::Motty,
        Declension::Mommish,
        Declension::Slim,
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

pub(super) static DECLENSIONS: [Paradigm<'static>; 59] = [
    // Good: 40,940 words (nonsense, polysemic, adjectival, fabaceous, fabian, abandoned).
    declension!("good", 40940, "~";
        adjective: "~", "more ~", "most ~";
    ),
    // Solid: 5,087 words (substantive, singular, secret, skillful, silver, saxon).
    declension!("solid", 5087, "~~";
        adjective: "~~", "more ~~", "mo~t s~";
    ),
    // Trivial: 2,559 words (tomato, thai, turquoise, teal, taupe, taiwanese). Like Solid, except in 1 cell: adjective sup.
    declension!("trivial", 2559, "~~", like Solid;
        adjective: _, _, "mos~ t~";
    ),
    // Major: 2,117 words (music, minuscule, magenta, musical, masculine, maroon).
    declension!("major", 2117, "~~";
        adjective: "~~", "~ore m~", "~ost m~";
    ),
    // External: 2,023 words (english, epizootic, european, epidemic, ebony, emerald).
    declension!("external", 2023, "~~";
        adjective: "~~", "mor~ e~", "most ~~";
    ),
    // Heavy: 1,776 words (craggy, country, early, baby, cherry, yeasty).
    declension!("heavy", 1776, "~y";
        adjective: "~y", "~ier", "~iest";
    ),
    // Open: 1,655 words (obfuscate, open, offset, olive, overweight, oa).
    declension!("open", 1655, "~~";
        adjective: "~~", "m~re o~", "m~st o~";
    ),
    // Round: 1,151 words (raven, russian, rational, room, russet, ruby). Like External, except in 1 cell: adjective comp.
    declension!("round", 1151, "~~", like External;
        adjective: _, "mo~e r~", _;
    ),
    // Regular: 927 words (reminiscent, regular, remote, redhanded, relaxed, reprehensible). Like External, except in 1 cell: adjective comp.
    declension!("regular", 927, "~~", like External;
        adjective: _, "mo~ re~", _;
    ),
    // Stacked: 736 words (stylized, street, stuck, stink, stank, strewn). Like Solid, except in 1 cell: adjective sup.
    declension!("stacked", 736, "~~", like Solid;
        adjective: _, _, "mo~ st~";
    ),
    // Soft: 635 words (free, brown, august, abject, able, odd).
    declension!("soft", 635, "~";
        adjective: "~", "~er", "~est";
    ),
    // Mental: 541 words (meticulous, mexican, mental, melancholy, melburnian, metal).
    declension!("mental", 541, "~~~";
        adjective: "~~~", "~or~ me~", "~ost m~~";
    ),
    // Modal: 330 words (mobile, money, modest, mongolian, mozambican, monastic).
    declension!("modal", 330, "~~";
        adjective: "~~", "~re mo~", "~st mo~";
    ),
    // Organic: 174 words (ordinate, original, ordinary, orthodox, ornamental, orbed).
    declension!("organic", 174, "~~~";
        adjective: "~~~", "m~~e or~", "m~st o~~";
    ),
    // Clayey: 116 words (smiley, horsey, dykey, limey, cagey, dickey).
    declension!("clayey", 116, "~~y";
        adjective: "~~y", "~i~r", "~i~st";
    ),
    // Shy: 87 words (shiny, stodgy, easy, rocky, shy, moldy).
    declension!("shy", 87, "~~y";
        adjective: "~~y", "~~ier|more ~~y", "~~iest";
    ),
    // Rich: 66 words (minute, mauve, skew, male, safe, simple).
    declension!("rich", 66, "~~";
        adjective: "~~", "~~er|more ~~", "~~est";
    ),
    // Mortal: 50 words (moral, mortal, mordant, moriform, morbid, morto).
    declension!("mortal", 50, "~~~";
        adjective: "~~~", "~~e mor~", "~st mo~~";
    ),
    // Awful: 46 words (big, red, shit, hot, fit, bog).
    declension!("awful", 46, "~";
        adjective: "~", "~ler|more ~", "~lest|most ~";
    ),
    // Fiery: 40 words (fiery, brotherly, likely, cringeworthy, priestly, namely).
    declension!("fiery", 40, "~y";
        adjective: "~y", "more ~y|~ier", "most ~y|~iest";
    ),
    // Oscarworthy: 35 words (oscular, oscillatory, osm, oscillating, osculating, osculatory).
    declension!("oscarworthy", 35, "~~~";
        adjective: "~~~", "m~re o~~", "m~~t os~";
    ),
    // Moot: 32 words (moot, moonbat, mooted, moonlit, moorish, moonish).
    declension!("moot", 32, "~~~";
        adjective: "~~~", "~re m~o~", "~st m~o~";
    ),
    // Ostentatious: 31 words (ostensible, ostentatious, ostracized, ostrobogulous, osteoarthritic, ostensive). Like Oscarworthy, except in 1 cell: adjective sup.
    declension!("ostentatious", 31, "~~~", like Oscarworthy;
        adjective: _, _, "m~~ ost~";
    ),
    // Motile: 25 words (motherly, motile, motivating, motherish, motivational, mothered).
    declension!("motile", 25, "~~~";
        adjective: "~~~", "~re mo~~", "~s~ mot~";
    ),
    // Memorial: 25 words (memorial, membranous, memorable, memristive, memorious, memorizable).
    declension!("memorial", 25, "~~~~";
        adjective: "~~~~", "~or~ ~em~", "~ost m~~~";
    ),
    // Mosey: 22 words (moslem, mosey, mosaiclike, mosquelike, mosquitocidal, mosquitoed). Like Motile, except in 1 cell: adjective sup.
    declension!("mosey", 22, "~~~", like Motile;
        adjective: _, _, "~~t mos~";
    ),
    // Right: 19 words (left, right, gone, inset, downstairs, front).
    declension!("right", 19, "~";
        adjective: "~", "further ~|farther ~", "furthest ~|farthest ~";
    ),
    // Otiose: 13 words (otiose, otherworldly, otaheite, otherworldy, otherworldish, otherly). Like Oscarworthy, except in 1 cell: adjective sup.
    declension!("otiose", 13, "~~~", like Oscarworthy;
        adjective: _, _, "m~s~ ot~";
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
    // Oedemic: 11 words (oedemic, oenological, oedemateous, oedematic, oedematous, oenophilic). Like Organic, except in 1 cell: adjective comp.
    declension!("oedemic", 11, "~~~", like Organic;
        adjective: _, "m~r~ oe~", _;
    ),
    // Ottawian: 10 words (ottawan, otterlike, ottery, otterish, ottomanic, ottawian).
    declension!("ottawian", 10, "~~~~";
        adjective: "~~~~", "m~re o~~~", "m~s~ o~t~";
    ),
    // Eevn: 9 words (eeyorish, eensy, eesome, eellike, eevn, eentsy). Like Reeded, except in 1 cell: adjective comp.
    declension!("eevn", 9, "~~~", like Reeded;
        adjective: _, "mor~ ~e~", _;
    ),
    // Switchy: 9 words (sisterly, spluttery, schizy, swimmy, schizzy, sweepy).
    declension!("switchy", 9, "~~y";
        adjective: "~~y", "more ~~y|~~ier", "mo~t s~y|~~iest";
    ),
    // Advanced: 8 words (dumbass, advanced, aged, lightweight, hardheaded, hardhearted). Like Solid, except in 1 cell: adjective sup.
    declension!("advanced", 8, "~~", like Solid;
        adjective: _, _, "most ~~";
    ),
    // Motoric: 8 words (motormouthed, motoric, motorable, motorik, motorlike, motogenic).
    declension!("motoric", 8, "~~~~";
        adjective: "~~~~", "~re mo~~~", "~s~ m~to~";
    ),
    // Meetable: 8 words (meetable, meetinglike, meedful, meekish, meedless, meedy). Like Memorial, except in 1 cell: adjective comp.
    declension!("meetable", 8, "~~~~", like Memorial;
        adjective: _, "~or~ m~e~", _;
    ),
    // Mid: 7 words (first, ben, middle, fore, mid, rightern).
    declension!("mid", 7, "~";
        adjective: "~", "~ner", "~most";
    ),
    // Orosian: 7 words (orotund, orodispersable, orodispersible, orographical, orophilous, orosian).
    declension!("orosian", 7, "~~~~";
        adjective: "~~~~", "m~~e ~ro~", "m~st o~~~";
    ),
    // Orectic: 7 words (orexigenic, orexinergic, orectic, orective, oresteian, orestean). Like Organic, except in 1 cell: adjective comp.
    declension!("orectic", 7, "~~~", like Organic;
        adjective: _, "m~~ ore~", _;
    ),
    // Ossified: 6 words (ossified, ossiferous, ossianic, ossiform, ossianesque, ossom). Like Ottawian, except in 1 cell: adjective sup.
    declension!("ossified", 6, "~~~~", like Ottawian;
        adjective: _, _, "m~~t o~s~";
    ),
    // Inner: 6 words (thither, upper, outer, inner, nether, nither). Like Mid, except in 1 cell: adjective comp.
    declension!("inner", 6, "~", like Mid;
        adjective: _, "~more", _;
    ),
    // Otomian: 5 words (otogenic, ototoxic, otoprotective, otoneuroprotective, otomian). Like Ottawian, except in 1 cell: adjective sup.
    declension!("otomian", 5, "~~~~", like Ottawian;
        adjective: _, _, "m~s~ ~to~";
    ),
    // Memetic: 5 words (memetic, memelike, memetical, memeable, memeworthy). Like Memorial, except in 1 cell: adjective comp.
    declension!("memetic", 5, "~~~~", like Memorial;
        adjective: _, "~or~ ~me~", _;
    ),
    // Mossful: 5 words (mosslike, mossgrown, mossbacked, mossened, mossful). Like Motoric, except in 1 cell: adjective sup.
    declension!("mossful", 5, "~~~~", like Motoric;
        adjective: _, _, "~~t mo~s~";
    ),
    // Moreish: 4 words (moreish, moresque, moreauvian, morescan). Like Mortal, except in 1 cell: adjective comp.
    declension!("moreish", 4, "~~~", like Mortal;
        adjective: _, "~~ more~", _;
    ),
    // Tonguey: 4 words (ropey, tomatoey, spikey, tonguey).
    declension!("tonguey", 4, "~~~y";
        adjective: "~~~y", "~~i~r|more ~~~y", "~~i~st";
    ),
    // Morrisian: 4 words (morrisian, morrisseyesque, morrisonian, morrisean).
    declension!("morrisian", 4, "~~~~";
        adjective: "~~~~", "~~e mo~r~", "~st mo~~~";
    ),
    // Flip: 3 words (crap, hip, flip).
    declension!("flip", 3, "~";
        adjective: "~", "~per", "~pest";
    ),
    // Thin: 3 words (thin, tan, wan). Like Mid, except in 1 cell: adjective sup.
    declension!("thin", 3, "~", like Mid;
        adjective: _, _, "~nest";
    ),
    // Evil: 3 words (evil, mournful, mirthful).
    declension!("evil", 3, "~~";
        adjective: "~~", "~~ler", "~~lest";
    ),
    // Moronic: 3 words (morose, moronic, moronical). Like Morrisian, except in 1 cell: adjective comp.
    declension!("moronic", 3, "~~~~", like Morrisian;
        adjective: _, "~~e m~ro~", _;
    ),
    // Mellow: 3 words (mellow, reef, reem).
    declension!("mellow", 3, "~~~";
        adjective: "~~~", "~~~er|mo~ r~e~", "~~~est|most ~~~";
    ),
    // Damned: 3 words (damned, darned, doggoned).
    declension!("damned", 3, "~e~";
        adjective: "~e~", "~e~er", "~e~est";
    ),
    // Motty: 3 words (mottled, motty, mottolike). Like Motoric, except in 1 cell: adjective sup.
    declension!("motty", 3, "~~~~", like Motoric;
        adjective: _, _, "~s~ mo~t~";
    ),
    // Mommish: 3 words (mommyish, mommylike, mommish).
    declension!("mommish", 3, "~~~~";
        adjective: "~~~~", "~re ~o~m~", "~st ~o~m~";
    ),
    // Slim: 2 words (clam, glam).
    declension!("slim", 2, "~";
        adjective: "~", "~mer", "~mest";
    ),
];
