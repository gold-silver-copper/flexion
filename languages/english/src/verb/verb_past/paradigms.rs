//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `verb` (past, prp)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     verb: $c0:tt, $c1:tt;
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            forms: ::std::borrow::Cow::Borrowed(&[flexion::form!($c0), flexion::form!($c1)]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `dropped`: 34,351 words.
    Dropped,
    /// Like `carried`: 1,514 words.
    Carried,
    /// Like `went`: 230 words.
    Went,
    /// Like `heard`: 173 words.
    Heard,
    /// Like `set`: 128 words.
    Set,
    /// Like `drove`: 97 words.
    Drove,
    /// Like `blew`: 70 words.
    Blew,
    /// Like `bent`: 65 words.
    Bent,
    /// Like `bound`: 62 words.
    Bound,
    /// Like `bled`: 60 words.
    Bled,
    /// Like `kept`: 56 words.
    Kept,
    /// Like `sprang`: 48 words.
    Sprang,
    /// Like `laid`: 47 words.
    Laid,
    /// Like `bore`: 45 words.
    Bore,
    /// Like `wrung`: 39 words.
    Wrung,
    /// Like `took`: 38 words.
    Took,
    /// Like `broke`: 37 words.
    Broke,
    /// Like `drew`: 36 words.
    Drew,
    /// Like `fixt`: 29 words.
    Fixt,
    /// Like `sat`: 29 words.
    Sat,
    /// Like `dived`: 24 words.
    Dived,
    /// Like `stood`: 23 words.
    Stood,
    /// Like `told`: 23 words.
    Told,
    /// Like `rid`: 22 words.
    Rid,
    /// Like `queued`: 21 words.
    Queued,
    /// Like `made`: 20 words.
    Made,
    /// Like `gave`: 19 words.
    Gave,
    /// Like `gilded`: 19 words.
    Gilded,
    /// Like `fitted`: 18 words.
    Fitted,
    /// Like `led`: 18 words.
    Led,
    /// Like `ran`: 18 words.
    Ran,
    /// Like `thought`: 17 words.
    Thought,
    /// Like `spilled`: 17 words.
    Spilled,
    /// Like `shot`: 17 words.
    Shot,
    /// Like `alighted`: 17 words.
    Alighted,
    /// Like `overwent`: 16 words.
    Overwent,
    /// Like `saw`: 15 words.
    Saw,
    /// Like `got`: 14 words.
    Got,
    /// Like `became`: 14 words.
    Became,
    /// Like `spun`: 14 words.
    Spun,
    /// Like `wove`: 12 words.
    Wove,
    /// Like `struck`: 12 words.
    Struck,
    /// Like `brought`: 11 words.
    Brought,
    /// Like `froze`: 11 words.
    Froze,
    /// Like `undid`: 11 words.
    Undid,
    /// Like `overate`: 11 words.
    Overate,
    /// Like `pleaded`: 11 words.
    Pleaded,
    /// Like `betaught`: 11 words.
    Betaught,
    /// Like `bought`: 10 words.
    Bought,
    /// Like `caught`: 10 words.
    Caught,
    /// Like `lay`: 10 words.
    Lay,
    /// Like `stole`: 10 words.
    Stole,
    /// Like `hung`: 9 words.
    Hung,
    /// Like `left`: 9 words.
    Left,
    /// Like `sought`: 9 words.
    Sought,
    /// Like `lit`: 8 words.
    Lit,
    /// Like `swam`: 8 words.
    Swam,
    /// Like `trod`: 8 words.
    Trod,
    /// Like `lopped`: 8 words.
    Lopped,
    /// Like `skied`: 7 words.
    Skied,
    /// Like `grokked`: 7 words.
    Grokked,
    /// Like `woke`: 7 words.
    Woke,
    /// Like `dug`: 7 words.
    Dug,
    /// Like `belayed`: 7 words.
    Belayed,
    /// Like `melted`: 7 words.
    Melted,
    /// Like `recced`: 7 words.
    Recced,
    /// Like `won`: 6 words.
    Won,
    /// Like `miskenned`: 6 words.
    Miskenned,
    /// Like `chose`: 6 words.
    Chose,
    /// Like `flew`: 6 words.
    Flew,
    /// Like `larned`: 6 words.
    Larned,
    /// Like `dwelt`: 5 words.
    Dwelt,
    /// Like `pressed`: 5 words.
    Pressed,
    /// Like `fled`: 4 words.
    Fled,
    /// Like `birdied`: 4 words.
    Birdied,
    /// Like `delammed`: 4 words.
    Delammed,
    /// Like `began`: 3 words.
    Began,
    /// Like `bereaved`: 3 words.
    Bereaved,
    /// Like `sheared`: 3 words.
    Sheared,
    /// Like `slew`: 3 words.
    Slew,
    /// Like `trolleyed`: 3 words.
    Trolleyed,
    /// Like `braaied`: 3 words.
    Braaied,
    /// Like `coffed`: 3 words.
    Coffed,
    /// Like `had`: 3 words.
    Had,
    /// Like `blewe`: 3 words.
    Blewe,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 85] = [
        Conjugation::Dropped,
        Conjugation::Carried,
        Conjugation::Went,
        Conjugation::Heard,
        Conjugation::Set,
        Conjugation::Drove,
        Conjugation::Blew,
        Conjugation::Bent,
        Conjugation::Bound,
        Conjugation::Bled,
        Conjugation::Kept,
        Conjugation::Sprang,
        Conjugation::Laid,
        Conjugation::Bore,
        Conjugation::Wrung,
        Conjugation::Took,
        Conjugation::Broke,
        Conjugation::Drew,
        Conjugation::Fixt,
        Conjugation::Sat,
        Conjugation::Dived,
        Conjugation::Stood,
        Conjugation::Told,
        Conjugation::Rid,
        Conjugation::Queued,
        Conjugation::Made,
        Conjugation::Gave,
        Conjugation::Gilded,
        Conjugation::Fitted,
        Conjugation::Led,
        Conjugation::Ran,
        Conjugation::Thought,
        Conjugation::Spilled,
        Conjugation::Shot,
        Conjugation::Alighted,
        Conjugation::Overwent,
        Conjugation::Saw,
        Conjugation::Got,
        Conjugation::Became,
        Conjugation::Spun,
        Conjugation::Wove,
        Conjugation::Struck,
        Conjugation::Brought,
        Conjugation::Froze,
        Conjugation::Undid,
        Conjugation::Overate,
        Conjugation::Pleaded,
        Conjugation::Betaught,
        Conjugation::Bought,
        Conjugation::Caught,
        Conjugation::Lay,
        Conjugation::Stole,
        Conjugation::Hung,
        Conjugation::Left,
        Conjugation::Sought,
        Conjugation::Lit,
        Conjugation::Swam,
        Conjugation::Trod,
        Conjugation::Lopped,
        Conjugation::Skied,
        Conjugation::Grokked,
        Conjugation::Woke,
        Conjugation::Dug,
        Conjugation::Belayed,
        Conjugation::Melted,
        Conjugation::Recced,
        Conjugation::Won,
        Conjugation::Miskenned,
        Conjugation::Chose,
        Conjugation::Flew,
        Conjugation::Larned,
        Conjugation::Dwelt,
        Conjugation::Pressed,
        Conjugation::Fled,
        Conjugation::Birdied,
        Conjugation::Delammed,
        Conjugation::Began,
        Conjugation::Bereaved,
        Conjugation::Sheared,
        Conjugation::Slew,
        Conjugation::Trolleyed,
        Conjugation::Braaied,
        Conjugation::Coffed,
        Conjugation::Had,
        Conjugation::Blewe,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 85] = [
    // Dropped: 34,351 words (portmanteaued, catted, worded, booked, pounded, ponded).
    conjugation!("dropped", 34351, "~ed";
        verb: "~ed", "~ing";
    ),
    // Carried: 1,514 words (dictionaried, died, vivified, nullified, treatied, policied).
    conjugation!("carried", 1514, "~~ed";
        verb: "~~ed", "~y~ng";
    ),
    // Went: 230 words (might, abought, did, could, was, came).
    conjugation!("went", 230, "~";
        verb: "~", "~ing";
    ),
    // Heard: 173 words (freed, pied, theed, toed, cooeed, treed).
    conjugation!("heard", 173, "~d";
        verb: "~d", "~ing";
    ),
    // Set: 128 words (offset, yet, bet, cut, hit, input).
    conjugation!("set", 128, "~";
        verb: "~", "~ting";
    ),
    // Drove: 97 words (abode, wrote, arose, drove, overrode, rewrote).
    conjugation!("drove", 97, "~o~e";
        verb: "~o~e", "~i~ing";
    ),
    // Blew: 70 words (knew, upblew, beheld, blew, grew, held).
    conjugation!("blew", 70, "~e~";
        verb: "~e~", "~o~ing";
    ),
    // Bent: 65 words (sent, bent, built, lent, rebuilt, spent).
    conjugation!("bent", 65, "~t";
        verb: "~t", "~ding";
    ),
    // Bound: 62 words (upbound, bound, fought, found, ground, rebound).
    conjugation!("bound", 62, "~ou~";
        verb: "~ou~", "~i~ing";
    ),
    // Bled: 60 words (upbred, bled, bred, fed, interbred, met).
    conjugation!("bled", 60, "~~";
        verb: "~~", "~e~ing";
    ),
    // Kept: 56 words (crept, felt, kept, knelt, overslept, slept).
    conjugation!("kept", 56, "~~t";
        verb: "~~t", "~e~ing";
    ),
    // Sprang: 48 words (sprang, shrank, drank, sang, rang, stank).
    conjugation!("sprang", 48, "~a~";
        verb: "~a~", "~i~ing";
    ),
    // Laid: 47 words (said, laid, mislaid, overlaid, overpaid, paid).
    conjugation!("laid", 47, "~~d";
        verb: "~~d", "~y~ng";
    ),
    // Bore: 45 words (abore, upbore, tore, bore, swore, wore).
    conjugation!("bore", 45, "~or~";
        verb: "~or~", "~~aring";
    ),
    // Wrung: 39 words (wrung, clung, flung, hamstrung, slung, stuck).
    conjugation!("wrung", 39, "~u~";
        verb: "~u~", "~i~ing";
    ),
    // Took: 38 words (took, forsook, mistook, overtook, partook, shook).
    conjugation!("took", 38, "~oo~";
        verb: "~oo~", "~a~ing";
    ),
    // Broke: 37 words (upbroke, broke, spoke, housebroke, misspoke, outbroke).
    conjugation!("broke", 37, "~ok~";
        verb: "~ok~", "~~aking";
    ),
    // Drew: 36 words (fell, drew, befell, overdrew, withdrew, updrew).
    conjugation!("drew", 36, "~e~";
        verb: "~e~", "~a~ing";
    ),
    // Fixt: 29 words (dealt, lost, fixt, misdealt, daydreamt, upleant).
    conjugation!("fixt", 29, "~t";
        verb: "~t", "~ing";
    ),
    // Sat: 29 words (shat, sat, spat, babysat, resat, housesat).
    conjugation!("sat", 29, "~a~";
        verb: "~a~", "~i~ting";
    ),
    // Dived: 24 words (dived, winded, upclimbed, updived, skydived, shited).
    conjugation!("dived", 24, "~i~ed";
        verb: "~i~ed|~o~e", "~i~ing";
    ),
    // Stood: 23 words (stood, misunderstood, understood, withstood, handstood, headstood).
    conjugation!("stood", 23, "~oo~";
        verb: "~oo~", "~an~ing";
    ),
    // Told: 23 words (foretold, outsold, oversold, resold, sold, told).
    conjugation!("told", 23, "~o~d";
        verb: "~o~d", "~e~ling";
    ),
    // Rid: 22 words (bid, outbid, rid, shed, wed, clad).
    conjugation!("rid", 22, "~";
        verb: "~", "~ding";
    ),
    // Queued: 21 words (blued, trued, horseshoed, coyoted, memed, queued).
    conjugation!("queued", 21, "~ed";
        verb: "~ed", "~eing|~ing";
    ),
    // Made: 20 words (made, remade, matchmade, unmade, premade, merrymade).
    conjugation!("made", 20, "~de";
        verb: "~de", "~king";
    ),
    // Gave: 19 words (gave, forgave, upgave, sange, foregave, misgave).
    conjugation!("gave", 19, "~a~e";
        verb: "~a~e", "~i~ing";
    ),
    // Gilded: 19 words (blended, undergirded, girded, gilded, bikeshedded, testbedded).
    conjugation!("gilded", 19, "~ded";
        verb: "~ded|~t", "~ding";
    ),
    // Fitted: 18 words (hetted, bullshitted, fitted, duetted, shortcutted, doughnutted).
    conjugation!("fitted", 18, "~ted";
        verb: "~ted|~", "~ting";
    ),
    // Led: 18 words (led, misled, upled, coled, cheerled, underled).
    conjugation!("led", 18, "~~";
        verb: "~~", "~a~ing";
    ),
    // Ran: 18 words (outran, overran, reran, ran, assran, speedran).
    conjugation!("ran", 18, "~a~";
        verb: "~a~", "~u~ning";
    ),
    // Thought: 17 words (thought, rethought, doublethought, outthought, crimethought, misthought).
    conjugation!("thought", 17, "~ou~ht";
        verb: "~ou~ht", "~inkin~";
    ),
    // Spilled: 17 words (smelled, spilled, respelled, rappelled, skolled, indwelled).
    conjugation!("spilled", 17, "~led";
        verb: "~led|~t", "~ling";
    ),
    // Shot: 17 words (overshot, shot, troubleshot, undershot, reshot, sharpshot).
    conjugation!("shot", 17, "~~";
        verb: "~~", "~o~ing";
    ),
    // Alighted: 17 words (alighted, relighted, gaslighted, spotlighted, daylighted, backlighted).
    conjugation!("alighted", 17, "~gh~ed";
        verb: "~gh~ed|~~", "~gh~ing";
    ),
    // Overwent: 16 words (forewent, forwent, underwent, towent, outwent, miswent).
    conjugation!("overwent", 16, "~we~t";
        verb: "~we~t", "~goi~g";
    ),
    // Saw: 15 words (saw, foresaw, oversaw, forsaw, sightsaw, unsaw).
    conjugation!("saw", 15, "~aw";
        verb: "~aw", "~eeing";
    ),
    // Got: 14 words (yot, forgot, got, begot, regot, ungot).
    conjugation!("got", 14, "~o~";
        verb: "~o~", "~e~ting";
    ),
    // Became: 14 words (became, overcame, misbecame, oncame, undercame, upcame).
    conjugation!("became", 14, "~a~e";
        verb: "~a~e", "~o~ing";
    ),
    // Spun: 14 words (spun, tailspun, backspun, topspun, overspun, wheelspun).
    conjugation!("spun", 14, "~u~";
        verb: "~u~", "~i~ning";
    ),
    // Wove: 12 words (wove, interwove, rewove, forclove, unwove, toclove).
    conjugation!("wove", 12, "~ov~";
        verb: "~ov~", "~~aving";
    ),
    // Struck: 12 words (struck, airstruck, restruck, counterstruck, overstruck, outstruck).
    conjugation!("struck", 12, "~uc~";
        verb: "~uc~", "~i~ing";
    ),
    // Brought: 11 words (brought, upbrought, underbrought, forthbrought, inbrought, imbrought).
    conjugation!("brought", 11, "~ou~ht";
        verb: "~ou~ht", "~in~ing";
    ),
    // Froze: 11 words (froze, unfroze, refroze, foze, enfroze, befroze).
    conjugation!("froze", 11, "~oz~";
        verb: "~oz~", "~~ezing";
    ),
    // Undid: 11 words (outdid, overdid, redid, undid, bedid, fordid).
    conjugation!("undid", 11, "~~d";
        verb: "~~d", "~o~ng";
    ),
    // Overate: 11 words (overate, mighte, muste, outate, underate, fale).
    conjugation!("overate", 11, "~~e";
        verb: "~~e", "~e~ing";
    ),
    // Pleaded: 11 words (pleaded, galed, handshaked, rehanged, impleaded, begraved).
    conjugation!("pleaded", 11, "~a~ed";
        verb: "~a~ed", "~a~ing";
    ),
    // Betaught: 11 words (taught, retaught, overtaught, mistaught, betaught, untaught).
    conjugation!("betaught", 11, "~~ug~t";
        verb: "~~ug~t", "~e~c~ing";
    ),
    // Bought: 10 words (bought, rebought, overbought, underbought, prebought, forbought).
    conjugation!("bought", 10, "~o~~ht";
        verb: "~o~~ht", "~~yin~";
    ),
    // Caught: 10 words (caught, bycaught, recaught, overcaught, becaught, outcaught).
    conjugation!("caught", 10, "~ugh~";
        verb: "~ugh~", "~~ching";
    ),
    // Lay: 10 words (lay, overlay, underlay, belay, outlay, superlay).
    conjugation!("lay", 10, "~a~";
        verb: "~a~", "~~ing";
    ),
    // Stole: 10 words (stole, bestole, overhole, forstole, unstole, upstole).
    conjugation!("stole", 10, "~ol~";
        verb: "~ol~", "~~aling";
    ),
    // Hung: 9 words (hung, overhung, uphung, cliffhung, behung, unhung).
    conjugation!("hung", 9, "~u~";
        verb: "~u~", "~a~ing";
    ),
    // Left: 9 words (left, reft, cleft, beleft, toreft, dreft).
    conjugation!("left", 9, "~ft";
        verb: "~ft", "~aving";
    ),
    // Sought: 9 words (sought, upsought, besought, outsought, resought, missought).
    conjugation!("sought", 9, "~ou~ht";
        verb: "~ou~ht", "~eekin~";
    ),
    // Lit: 8 words (lit, flashlit, alit, twilit, floodlit, prelit).
    conjugation!("lit", 8, "~~";
        verb: "~~|~gh~ed", "~gh~ing";
    ),
    // Swam: 8 words (swam, nam, benam, outswam, overnam, overswam).
    conjugation!("swam", 8, "~a~";
        verb: "~a~", "~i~ming";
    ),
    // Trod: 8 words (trod, retrod, totrod, fortrod, betrod, untrod).
    conjugation!("trod", 8, "~o~";
        verb: "~o~", "~ea~ing";
    ),
    // Lopped: 8 words (lopped, outstripped, fellowshipped, reequipped, disfellowshipped, tittupped).
    conjugation!("lopped", 8, "~ped";
        verb: "~ped", "~ping";
    ),
    // Skied: 7 words (skied, flied, vied, epoxied, ennuied, miladied).
    conjugation!("skied", 7, "~ied";
        verb: "~ied|~yed", "~ying";
    ),
    // Grokked: 7 words (grokked, kekked, sacked, tarmacked, mosaicked, demosaicked).
    conjugation!("grokked", 7, "~ked";
        verb: "~ked|~ed", "~king|~ing";
    ),
    // Woke: 7 words (awoke, woke, wayfore, reawoke, forcorve, rewoke).
    conjugation!("woke", 7, "~o~e";
        verb: "~o~e|~a~ed", "~a~ing";
    ),
    // Dug: 7 words (dug, underdug, undug, redug, misdug, outdug).
    conjugation!("dug", 7, "~u~";
        verb: "~u~", "~i~ging";
    ),
    // Belayed: 7 words (whyed, payed, repayed, belayed, daresayed, apayed).
    conjugation!("belayed", 7, "~yed";
        verb: "~yed|~id", "~ying";
    ),
    // Melted: 7 words (melted, speedwalked, weeped, breeded, dispeeded, formelted).
    conjugation!("melted", 7, "~e~ed";
        verb: "~e~ed", "~e~ing";
    ),
    // Recced: 7 words (zinced, vacced, recced, twocced, renforced, glaunced).
    conjugation!("recced", 7, "~ced";
        verb: "~ced", "~cing";
    ),
    // Won: 6 words (won, overwon, rewon, outwon, bewon, electrowon).
    conjugation!("won", 6, "~o~";
        verb: "~o~", "~i~ning";
    ),
    // Miskenned: 6 words (penned, caravanned, bekenned, miskenned, forekenned, outkenned).
    conjugation!("miskenned", 6, "~ned";
        verb: "~ned|~t", "~ning";
    ),
    // Chose: 6 words (chose, forechose, rechose, mischose, unchose, prechose).
    conjugation!("chose", 6, "~~e";
        verb: "~~e", "~o~ing";
    ),
    // Flew: 6 words (flew, overflew, outflew, reflew, beflew, upflew).
    conjugation!("flew", 6, "~ew";
        verb: "~ew", "~ying";
    ),
    // Larned: 6 words (makked, radged, deeked, chorred, larned, dunshed).
    conjugation!("larned", 6, "~ed";
        verb: "~ed", "~in";
    ),
    // Dwelt: 5 words (dwelt, misspelt, foresmelt, outdwelt, bedwelt).
    conjugation!("dwelt", 5, "~t";
        verb: "~t|~led", "~ling";
    ),
    // Pressed: 5 words (octopussed, nonplussed, pressed, ressed, dorcassed).
    conjugation!("pressed", 5, "~sed";
        verb: "~sed|~ed", "~sing|~ing";
    ),
    // Fled: 4 words (fled, shod, befled, outfled).
    conjugation!("fled", 4, "~d";
        verb: "~d", "~eing";
    ),
    // Birdied: 4 words (wheelied, stymied, birdied, assoilzied).
    conjugation!("birdied", 4, "~~ed";
        verb: "~~ed", "~~eing|~y~ng";
    ),
    // Delammed: 4 words (intercommed, diademmed, delammed, undiademmed).
    conjugation!("delammed", 4, "~med";
        verb: "~med|~ed", "~ming|~ing";
    ),
    // Began: 3 words (began, gan, rebegan).
    conjugation!("began", 3, "~a~";
        verb: "~a~", "~i~ning";
    ),
    // Bereaved: 3 words (leaved, reaved, bereaved).
    conjugation!("bereaved", 3, "~aved";
        verb: "~aved|~ft", "~aving";
    ),
    // Sheared: 3 words (sheared, weared, resheared).
    conjugation!("sheared", 3, "~~ared";
        verb: "~~ared|~or~", "~~aring";
    ),
    // Slew: 3 words (slew, foreslew, reslew).
    conjugation!("slew", 3, "~ew";
        verb: "~ew", "~aying";
    ),
    // Trolleyed: 3 words (monkeyed, bogeyed, trolleyed).
    conjugation!("trolleyed", 3, "~~yed";
        verb: "~~yed|~i~d", "~~ying";
    ),
    // Braaied: 3 words (braaied, luigied, skibidied).
    conjugation!("braaied", 3, "~ed";
        verb: "~ed", "~ng";
    ),
    // Coffed: 3 words (coiffed, uncoiffed, coffed).
    conjugation!("coffed", 3, "~fed";
        verb: "~fed|~ed", "~fing|~ing";
    ),
    // Had: 3 words (had, hed, rehad).
    conjugation!("had", 3, "~d";
        verb: "~d", "~ving";
    ),
    // Blewe: 3 words (behelde, grewe, blewe).
    conjugation!("blewe", 3, "~e~e";
        verb: "~e~e", "~o~ing";
    ),
];
