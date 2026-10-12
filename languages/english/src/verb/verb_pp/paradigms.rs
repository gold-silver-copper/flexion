//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `verb` (pp)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     verb: $c0:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0)]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `drawn`: 36,922 words.
    Drawn,
    /// Like `told`: 525 words.
    Told,
    /// Like `burned`: 82 words.
    Burned,
    /// Like `bided`: 26 words.
    Bided,
    /// Like `wrung`: 21 words.
    Wrung,
    /// Like `fixt`: 21 words.
    Fixt,
    /// Like `shredded`: 20 words.
    Shredded,
    /// Like `fretted`: 17 words.
    Fretted,
    /// Like `alighted`: 17 words.
    Alighted,
    /// Like `laden`: 16 words.
    Laden,
    /// Like `shorn`: 15 words.
    Shorn,
    /// Like `skied`: 14 words.
    Skied,
    /// Like `spat`: 14 words.
    Spat,
    /// Like `spilled`: 14 words.
    Spilled,
    /// Like `fitted`: 12 words.
    Fitted,
    /// Like `dived`: 8 words.
    Dived,
    /// Like `curvetted`: 8 words.
    Curvetted,
    /// Like `winded`: 7 words.
    Winded,
    /// Like `grokked`: 6 words.
    Grokked,
    /// Like `miskenned`: 6 words.
    Miskenned,
    /// Like `belayed`: 6 words.
    Belayed,
    /// Like `pleaded`: 6 words.
    Pleaded,
    /// Like `melted`: 6 words.
    Melted,
    /// Like `recced`: 5 words.
    Recced,
    /// Like `galed`: 5 words.
    Galed,
    /// Like `speedwalked`: 5 words.
    Speedwalked,
    /// Like `crept`: 4 words.
    Crept,
    /// Like `got`: 4 words.
    Got,
    /// Like `octopussed`: 4 words.
    Octopussed,
    /// Like `lopped`: 4 words.
    Lopped,
    /// Like `delammed`: 4 words.
    Delammed,
    /// Like `fellowshipped`: 4 words.
    Fellowshipped,
    /// Like `abode`: 3 words.
    Abode,
    /// Like `bitten`: 3 words.
    Bitten,
    /// Like `bereaved`: 3 words.
    Bereaved,
    /// Like `trolleyed`: 3 words.
    Trolleyed,
    /// Like `dressed`: 3 words.
    Dressed,
    /// Like `skolled`: 3 words.
    Skolled,
    /// Like `dreft`: 3 words.
    Dreft,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 39] = [
        Conjugation::Drawn,
        Conjugation::Told,
        Conjugation::Burned,
        Conjugation::Bided,
        Conjugation::Wrung,
        Conjugation::Fixt,
        Conjugation::Shredded,
        Conjugation::Fretted,
        Conjugation::Alighted,
        Conjugation::Laden,
        Conjugation::Shorn,
        Conjugation::Skied,
        Conjugation::Spat,
        Conjugation::Spilled,
        Conjugation::Fitted,
        Conjugation::Dived,
        Conjugation::Curvetted,
        Conjugation::Winded,
        Conjugation::Grokked,
        Conjugation::Miskenned,
        Conjugation::Belayed,
        Conjugation::Pleaded,
        Conjugation::Melted,
        Conjugation::Recced,
        Conjugation::Galed,
        Conjugation::Speedwalked,
        Conjugation::Crept,
        Conjugation::Got,
        Conjugation::Octopussed,
        Conjugation::Lopped,
        Conjugation::Delammed,
        Conjugation::Fellowshipped,
        Conjugation::Abode,
        Conjugation::Bitten,
        Conjugation::Bereaved,
        Conjugation::Trolleyed,
        Conjugation::Dressed,
        Conjugation::Skolled,
        Conjugation::Dreft,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 39] = [
    // Drawn: 36,922 words (dictionaried, freed, portmanteaued, catted, worded, booked).
    conjugation!("drawn", 36922, "~";
        verb: "~";
    ),
    // Told: 525 words (oned, abuttaled, chairmaned, penciled, secreted, generaled).
    conjugation!("told", 525, "~~";
        verb: "~~|~l~";
    ),
    // Burned: 82 words (abraided, leaped, learned, chainsawed, hoisted, burned).
    conjugation!("burned", 82, "~ed";
        verb: "~ed|~t";
    ),
    // Bided: 26 words (loled, yet, taed, fleaed, microed, teaed).
    conjugation!("bided", 26, "~e~";
        verb: "~e~|~~";
    ),
    // Wrung: 21 words (drunk, wrung, hung, hamstrung, overhung, slung).
    conjugation!("wrung", 21, "~u~";
        verb: "~u~";
    ),
    // Fixt: 21 words (dwelt, misspelt, blest, fixt, rent, clipt).
    conjugation!("fixt", 21, "~t";
        verb: "~t|~ed";
    ),
    // Shredded: 20 words (blended, shredded, undergirded, girded, gilded, bikeshedded).
    conjugation!("shredded", 20, "~ded";
        verb: "~ded|~t";
    ),
    // Fretted: 17 words (proved, graved, disproved, fretted, shaved, betrothed).
    conjugation!("fretted", 17, "~d";
        verb: "~d|~n";
    ),
    // Alighted: 17 words (alighted, relighted, gaslighted, spotlighted, daylighted, backlighted).
    conjugation!("alighted", 17, "~gh~ed";
        verb: "~gh~ed|~~";
    ),
    // Laden: 16 words (shewn, mown, sewn, strewn, striven, shriven).
    conjugation!("laden", 16, "~n";
        verb: "~n|~ed";
    ),
    // Shorn: 15 words (woven, shorn, swollen, woken, unwoven, reawoken).
    conjugation!("shorn", 15, "~o~n";
        verb: "~o~n";
    ),
    // Skied: 14 words (skied, bid, flied, overlaid, vied, chid).
    conjugation!("skied", 14, "~i~";
        verb: "~i~";
    ),
    // Spat: 14 words (shat, spat, babysat, housesat, came, birdsat).
    conjugation!("spat", 14, "~a~";
        verb: "~a~";
    ),
    // Spilled: 14 words (smelled, spilled, respelled, indwelled, overspilled, outspelled).
    conjugation!("spilled", 14, "~led";
        verb: "~led|~t";
    ),
    // Fitted: 12 words (hetted, bullshitted, fitted, petted, shortcutted, screenshotted).
    conjugation!("fitted", 12, "~ted";
        verb: "~ted|~";
    ),
    // Dived: 8 words (dived, rived, gainstrived, clived, snithed, bliked).
    conjugation!("dived", 8, "~i~d";
        verb: "~i~d";
    ),
    // Curvetted: 8 words (duetted, doughnutted, eyeletted, curvetted, brevetted, telnetted).
    conjugation!("curvetted", 8, "~t~";
        verb: "~t~|~~";
    ),
    // Winded: 7 words (winded, upclimbed, shited, heatsinked, sidewinded, pigsticked).
    conjugation!("winded", 7, "~i~ed";
        verb: "~i~ed";
    ),
    // Grokked: 6 words (grokked, kekked, sacked, tarmacked, mosaicked, demosaicked).
    conjugation!("grokked", 6, "~k~";
        verb: "~k~|~~";
    ),
    // Miskenned: 6 words (penned, brenned, bekenned, miskenned, forekenned, outkenned).
    conjugation!("miskenned", 6, "~ned";
        verb: "~ned|~t";
    ),
    // Belayed: 6 words (payed, repayed, belayed, daresayed, apayed, appayed).
    conjugation!("belayed", 6, "~ye~";
        verb: "~ye~|~i~";
    ),
    // Pleaded: 6 words (pleaded, handshaked, rehanged, impleaded, bedared, interpleaded).
    conjugation!("pleaded", 6, "~a~ed";
        verb: "~a~ed";
    ),
    // Melted: 6 words (melted, shrieved, yeted, upswelled, outswelled, deswelled).
    conjugation!("melted", 6, "~e~d";
        verb: "~e~d|~o~n";
    ),
    // Recced: 5 words (zinced, vacced, recced, twocced, dezinced).
    conjugation!("recced", 5, "~c~";
        verb: "~c~";
    ),
    // Galed: 5 words (galed, begraved, afared, forfared, overfared).
    conjugation!("galed", 5, "~a~d";
        verb: "~a~d|~o~|~a~n";
    ),
    // Speedwalked: 5 words (speedwalked, weeped, dispeeded, formelted, bespeeded).
    conjugation!("speedwalked", 5, "~e~ed";
        verb: "~e~ed";
    ),
    // Crept: 4 words (crept, knelt, mineswept, powercrept).
    conjugation!("crept", 4, "~~t";
        verb: "~~t|~e~ed";
    ),
    // Got: 4 words (got, shone, outshone, backshone).
    conjugation!("got", 4, "~o~";
        verb: "~o~|~i~d";
    ),
    // Octopussed: 4 words (octopussed, nonplussed, ressed, dorcassed).
    conjugation!("octopussed", 4, "~s~";
        verb: "~s~|~~";
    ),
    // Lopped: 4 words (lopped, outstripped, reequipped, forstopped).
    conjugation!("lopped", 4, "~ped";
        verb: "~ped|~t";
    ),
    // Delammed: 4 words (intercommed, diademmed, delammed, undiademmed).
    conjugation!("delammed", 4, "~m~";
        verb: "~m~|~~";
    ),
    // Fellowshipped: 4 words (fellowshipped, disfellowshipped, tittupped, unfellowshipped).
    conjugation!("fellowshipped", 4, "~p~";
        verb: "~p~|~~";
    ),
    // Abode: 3 words (abode, come, bestrode).
    conjugation!("abode", 3, "~o~e";
        verb: "~o~e";
    ),
    // Bitten: 3 words (bitten, inwritten, gristbitten).
    conjugation!("bitten", 3, "~ten";
        verb: "~ten|~";
    ),
    // Bereaved: 3 words (leaved, reaved, bereaved).
    conjugation!("bereaved", 3, "~aved";
        verb: "~aved|~ft";
    ),
    // Trolleyed: 3 words (monkeyed, bogeyed, trolleyed).
    conjugation!("trolleyed", 3, "~~ye~";
        verb: "~~ye~|~i~~";
    ),
    // Dressed: 3 words (dressed, pressed, possessed).
    conjugation!("dressed", 3, "~sed";
        verb: "~sed|~t";
    ),
    // Skolled: 3 words (rappelled, skolled, escortcelled).
    conjugation!("skolled", 3, "~l~";
        verb: "~l~|~~";
    ),
    // Dreft: 3 words (toreft, torift, dreft).
    conjugation!("dreft", 3, "~ft";
        verb: "~ft|~aved";
    ),
];
