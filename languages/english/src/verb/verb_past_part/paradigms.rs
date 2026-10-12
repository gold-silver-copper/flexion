//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `verb` (past)
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
    /// Like `have`: 17,193 words.
    Have,
    /// Like `burn`: 15,657 words.
    Burn,
    /// Like `carry`: 1,476 words.
    Carry,
    /// Like `drop`: 493 words.
    Drop,
    /// Like `spot`: 397 words.
    Spot,
    /// Like `bag`: 313 words.
    Bag,
    /// Like `set`: 302 words.
    Set,
    /// Like `skin`: 212 words.
    Skin,
    /// Like `scrub`: 182 words.
    Scrub,
    /// Like `slam`: 177 words.
    Slam,
    /// Like `stir`: 130 words.
    Stir,
    /// Like `shred`: 114 words.
    Shred,
    /// Like `spring`: 104 words.
    Spring,
    /// Like `drive`: 100 words.
    Drive,
    /// Like `break`: 90 words.
    Break,
    /// Like `distil`: 88 words.
    Distil,
    /// Like `blow`: 73 words.
    Blow,
    /// Like `bend`: 68 words.
    Bend,
    /// Like `bleed`: 61 words.
    Bleed,
    /// Like `wring`: 60 words.
    Wring,
    /// Like `bind`: 60 words.
    Bind,
    /// Like `keep`: 56 words.
    Keep,
    /// Like `panic`: 55 words.
    Panic,
    /// Like `lay`: 46 words.
    Lay,
    /// Like `take`: 38 words.
    Take,
    /// Like `draw`: 37 words.
    Draw,
    /// Like `bite`: 31 words.
    Bite,
    /// Like `fix`: 27 words.
    Fix,
    /// Like `tread`: 25 words.
    Tread,
    /// Like `stand`: 23 words.
    Stand,
    /// Like `shoot`: 23 words.
    Shoot,
    /// Like `tell`: 23 words.
    Tell,
    /// Like `make`: 20 words.
    Make,
    /// Like `gas`: 18 words.
    Gas,
    /// Like `mislead`: 18 words.
    Mislead,
    /// Like `run`: 18 words.
    Run,
    /// Like `think`: 17 words.
    Think,
    /// Like `dive`: 17 words.
    Dive,
    /// Like `alight`: 17 words.
    Alight,
    /// Like `gild`: 17 words.
    Gild,
    /// Like `overgo`: 16 words.
    Overgo,
    /// Like `see`: 15 words.
    See,
    /// Like `come`: 15 words.
    Come,
    /// Like `spill`: 14 words.
    Spill,
    /// Like `get`: 14 words.
    Get,
    /// Like `clepe`: 14 words.
    Clepe,
    /// Like `undo`: 12 words.
    Undo,
    /// Like `strike`: 12 words.
    Strike,
    /// Like `quiz`: 11 words.
    Quiz,
    /// Like `buy`: 11 words.
    Buy,
    /// Like `bring`: 11 words.
    Bring,
    /// Like `rec`: 11 words.
    Rec,
    /// Like `beteach`: 11 words.
    Beteach,
    /// Like `catch`: 10 words.
    Catch,
    /// Like `freeze`: 10 words.
    Freeze,
    /// Like `chef`: 10 words.
    Chef,
    /// Like `hang`: 9 words.
    Hang,
    /// Like `lie`: 9 words.
    Lie,
    /// Like `seek`: 9 words.
    Seek,
    /// Like `leave`: 8 words.
    Leave,
    /// Like `overeat`: 8 words.
    Overeat,
    /// Like `wake`: 7 words.
    Wake,
    /// Like `light`: 7 words.
    Light,
    /// Like `shiv`: 7 words.
    Shiv,
    /// Like `fly`: 6 words.
    Fly,
    /// Like `wind`: 6 words.
    Wind,
    /// Like `lose`: 5 words.
    Lose,
    /// Like `dwell`: 5 words.
    Dwell,
    /// Like `flee`: 5 words.
    Flee,
    /// Like `belay`: 5 words.
    Belay,
    /// Like `melt`: 5 words.
    Melt,
    /// Like `gale`: 5 words.
    Gale,
    /// Like `abide`: 4 words.
    Abide,
    /// Like `bid`: 4 words.
    Bid,
    /// Like `plead`: 4 words.
    Plead,
    /// Like `shear`: 3 words.
    Shear,
    /// Like `slay`: 3 words.
    Slay,
    /// Like `trolley`: 3 words.
    Trolley,
    /// Like `chide`: 3 words.
    Chide,
    /// Like `bede`: 3 words.
    Bede,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 80] = [
        Conjugation::Have,
        Conjugation::Burn,
        Conjugation::Carry,
        Conjugation::Drop,
        Conjugation::Spot,
        Conjugation::Bag,
        Conjugation::Set,
        Conjugation::Skin,
        Conjugation::Scrub,
        Conjugation::Slam,
        Conjugation::Stir,
        Conjugation::Shred,
        Conjugation::Spring,
        Conjugation::Drive,
        Conjugation::Break,
        Conjugation::Distil,
        Conjugation::Blow,
        Conjugation::Bend,
        Conjugation::Bleed,
        Conjugation::Wring,
        Conjugation::Bind,
        Conjugation::Keep,
        Conjugation::Panic,
        Conjugation::Lay,
        Conjugation::Take,
        Conjugation::Draw,
        Conjugation::Bite,
        Conjugation::Fix,
        Conjugation::Tread,
        Conjugation::Stand,
        Conjugation::Shoot,
        Conjugation::Tell,
        Conjugation::Make,
        Conjugation::Gas,
        Conjugation::Mislead,
        Conjugation::Run,
        Conjugation::Think,
        Conjugation::Dive,
        Conjugation::Alight,
        Conjugation::Gild,
        Conjugation::Overgo,
        Conjugation::See,
        Conjugation::Come,
        Conjugation::Spill,
        Conjugation::Get,
        Conjugation::Clepe,
        Conjugation::Undo,
        Conjugation::Strike,
        Conjugation::Quiz,
        Conjugation::Buy,
        Conjugation::Bring,
        Conjugation::Rec,
        Conjugation::Beteach,
        Conjugation::Catch,
        Conjugation::Freeze,
        Conjugation::Chef,
        Conjugation::Hang,
        Conjugation::Lie,
        Conjugation::Seek,
        Conjugation::Leave,
        Conjugation::Overeat,
        Conjugation::Wake,
        Conjugation::Light,
        Conjugation::Shiv,
        Conjugation::Fly,
        Conjugation::Wind,
        Conjugation::Lose,
        Conjugation::Dwell,
        Conjugation::Flee,
        Conjugation::Belay,
        Conjugation::Melt,
        Conjugation::Gale,
        Conjugation::Abide,
        Conjugation::Bid,
        Conjugation::Plead,
        Conjugation::Shear,
        Conjugation::Slay,
        Conjugation::Trolley,
        Conjugation::Chide,
        Conjugation::Bede,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 80] = [
    // Have: 17,193 words (free, nonsense, pie, may, minute, trade).
    conjugation!("have", 17193, "~";
        verb: "~d";
    ),
    // Burn: 15,657 words (portmanteau, word, book, pound, pond, crow).
    conjugation!("burn", 15657, "~";
        verb: "~ed";
    ),
    // Carry: 1,476 words (dictionary, sky, vivify, nullify, treaty, policy).
    conjugation!("carry", 1476, "~y";
        verb: "~ied";
    ),
    // Drop: 493 words (swap, swop, crap, map, pip, bap).
    conjugation!("drop", 493, "~";
        verb: "~ped";
    ),
    // Spot: 397 words (cat, abet, abut, het, bit, bot).
    conjugation!("spot", 397, "~";
        verb: "~ted";
    ),
    // Bag: 313 words (big, gag, drug, twig, beg, leg).
    conjugation!("bag", 313, "~";
        verb: "~ged";
    ),
    // Set: 302 words (forecast, offset, yet, cast, beat, read).
    conjugation!("set", 302, "~";
        verb: "~";
    ),
    // Skin: 212 words (van, in, sun, man, thin, son).
    conjugation!("skin", 212, "~";
        verb: "~ned";
    ),
    // Scrub: 182 words (ab, lob, dub, bib, stab, pub).
    conjugation!("scrub", 182, "~";
        verb: "~bed";
    ),
    // Slam: 177 words (dotcom, um, stem, plum, om, bam).
    conjugation!("slam", 177, "~";
        verb: "~med";
    ),
    // Stir: 130 words (abhor, star, war, defer, upbar, er).
    conjugation!("stir", 130, "~";
        verb: "~red";
    ),
    // Shred: 114 words (cod, red, squid, god, bad, bed).
    conjugation!("shred", 114, "~";
        verb: "~ded";
    ),
    // Spring: 104 words (spring, shrink, drink, give, shit, sing).
    conjugation!("spring", 104, "~i~";
        verb: "~a~";
    ),
    // Drive: 100 words (win, write, arise, drive, override, rewrite).
    conjugation!("drive", 100, "~i~";
        verb: "~o~";
    ),
    // Break: 90 words (abear, upbear, tear, upbreak, break, speak).
    conjugation!("break", 90, "~ea~";
        verb: "~o~e";
    ),
    // Distil: 88 words (canal, control, appel, halal, compel, gel).
    conjugation!("distil", 88, "~";
        verb: "~led";
    ),
    // Blow: 73 words (know, upblow, behold, blow, grow, hold).
    conjugation!("blow", 73, "~o~";
        verb: "~e~";
    ),
    // Bend: 68 words (send, bend, build, lend, rebuild, spend).
    conjugation!("bend", 68, "~d";
        verb: "~t";
    ),
    // Bleed: 61 words (upbreed, bleed, breed, feed, interbreed, meet).
    conjugation!("bleed", 61, "~e~";
        verb: "~~";
    ),
    // Wring: 60 words (wring, cling, dig, fling, hamstring, sling).
    conjugation!("wring", 60, "~i~";
        verb: "~u~";
    ),
    // Bind: 60 words (upbind, bind, fight, find, grind, rebind).
    conjugation!("bind", 60, "~i~";
        verb: "~ou~";
    ),
    // Keep: 56 words (creep, feel, keep, kneel, oversleep, sleep).
    conjugation!("keep", 56, "~e~";
        verb: "~~t";
    ),
    // Panic: 55 words (metric, grok, music, hydraulic, yak, traffic).
    conjugation!("panic", 55, "~";
        verb: "~ked";
    ),
    // Lay: 46 words (say, lay, mislay, overlay, overpay, pay).
    conjugation!("lay", 46, "~y";
        verb: "~id";
    ),
    // Take: 38 words (take, forsake, mistake, overtake, partake, shake).
    conjugation!("take", 38, "~a~e";
        verb: "~oo~";
    ),
    // Draw: 37 words (fall, draw, befall, overdraw, withdraw, updraw).
    conjugation!("draw", 37, "~a~";
        verb: "~e~";
    ),
    // Bite: 31 words (bite, hide, slide, frostbite, landslide, backbite).
    conjugation!("bite", 31, "~e";
        verb: "~";
    ),
    // Fix: 27 words (deal, born, gar, fix, misdeal, daydream).
    conjugation!("fix", 27, "~";
        verb: "~t";
    ),
    // Tread: 25 words (weave, interweave, tread, reweave, sweare, speake).
    conjugation!("tread", 25, "~ea~";
        verb: "~o~";
    ),
    // Stand: 23 words (stand, misunderstand, understand, withstand, handstand, headstand).
    conjugation!("stand", 23, "~an~";
        verb: "~oo~";
    ),
    // Shoot: 23 words (choose, overshoot, shoot, troubleshoot, undershoot, reshoot).
    conjugation!("shoot", 23, "~o~";
        verb: "~~";
    ),
    // Tell: 23 words (foretell, outsell, oversell, resell, sell, tell).
    conjugation!("tell", 23, "~e~l";
        verb: "~o~d";
    ),
    // Make: 20 words (make, remake, matchmake, unmake, premake, merrymake).
    conjugation!("make", 20, "~k~";
        verb: "~d~";
    ),
    // Gas: 18 words (gas, yes, octopus, nonplus, dis, pus).
    conjugation!("gas", 18, "~";
        verb: "~sed";
    ),
    // Mislead: 18 words (mislead, uplead, colead, cheerlead, underlead, aread).
    conjugation!("mislead", 18, "~a~";
        verb: "~~";
    ),
    // Run: 18 words (outrun, overrun, rerun, run, assrun, speedrun).
    conjugation!("run", 18, "~u~";
        verb: "~a~";
    ),
    // Think: 17 words (think, rethink, doublethink, outthink, crimethink, misthink).
    conjugation!("think", 17, "~ink";
        verb: "~ought";
    ),
    // Dive: 17 words (dive, updive, skydive, rive, nosedive, gainstrive).
    conjugation!("dive", 17, "~i~";
        verb: "~i~d|~o~";
    ),
    // Alight: 17 words (alight, relight, gaslight, spotlight, daylight, backlight).
    conjugation!("alight", 17, "~gh~";
        verb: "~gh~ed|~~";
    ),
    // Gild: 17 words (blend, undergird, gird, gild, backbend, regild).
    conjugation!("gild", 17, "~d";
        verb: "~ded|~t";
    ),
    // Overgo: 16 words (forego, forgo, undergo, togo, outgo, misgo).
    conjugation!("overgo", 16, "~go";
        verb: "~went";
    ),
    // See: 15 words (see, foresee, oversee, forsee, sightsee, unsee).
    conjugation!("see", 15, "~ee";
        verb: "~aw";
    ),
    // Come: 15 words (become, come, overcome, misbecome, oncome, undercome).
    conjugation!("come", 15, "~o~";
        verb: "~a~";
    ),
    // Spill: 14 words (smell, spill, respell, indwell, overspill, outspell).
    conjugation!("spill", 14, "~l";
        verb: "~led|~t";
    ),
    // Get: 14 words (forget, get, beget, reget, unget, forhele).
    conjugation!("get", 14, "~e~";
        verb: "~o~";
    ),
    // Clepe: 14 words (horseshoe, glide, perse, hoise, twite, clepe).
    conjugation!("clepe", 14, "~e";
        verb: "~ed";
    ),
    // Undo: 12 words (do, outdo, overdo, redo, undo, bedo).
    conjugation!("undo", 12, "~o";
        verb: "~id";
    ),
    // Strike: 12 words (strike, airstrike, restrike, counterstrike, overstrike, outstrike).
    conjugation!("strike", 12, "~i~e";
        verb: "~uc~";
    ),
    // Quiz: 11 words (quiz, spaz, whiz, buz, wiz, diz).
    conjugation!("quiz", 11, "~";
        verb: "~zed";
    ),
    // Buy: 11 words (buy, rebuy, overbuy, underbuy, prebuy, forbuy).
    conjugation!("buy", 11, "~~y";
        verb: "~o~ght";
    ),
    // Bring: 11 words (bring, upbring, underbring, forthbring, inbring, imbring).
    conjugation!("bring", 11, "~in~";
        verb: "~ou~ht";
    ),
    // Rec: 11 words (sic, floc, tic, vac, proc, fanfic).
    conjugation!("rec", 11, "~";
        verb: "~ced";
    ),
    // Beteach: 11 words (teach, reteach, overteach, misteach, beteach, unteach).
    conjugation!("beteach", 11, "~e~c~";
        verb: "~~ug~t";
    ),
    // Catch: 10 words (catch, bycatch, recatch, overcatch, becatch, outcatch).
    conjugation!("catch", 10, "~tc~";
        verb: "~ug~t";
    ),
    // Freeze: 10 words (freeze, unfreeze, refreeze, enfreeze, befreeze, prefreeze).
    conjugation!("freeze", 10, "~ee~";
        verb: "~o~";
    ),
    // Chef: 10 words (ref, chef, crossref, coif, stuf, indef).
    conjugation!("chef", 10, "~";
        verb: "~fed";
    ),
    // Hang: 9 words (hang, overhang, uphang, cliffhang, behang, unhang).
    conjugation!("hang", 9, "~a~";
        verb: "~u~";
    ),
    // Lie: 9 words (lie, overlie, underlie, belie, superlie, forelie).
    conjugation!("lie", 9, "~ie";
        verb: "~ay";
    ),
    // Seek: 9 words (seek, upseek, beseek, outseek, reseek, misseek).
    conjugation!("seek", 9, "~eek";
        verb: "~ought";
    ),
    // Leave: 8 words (leave, cleave, beleave, toreave, dreave, forleave).
    conjugation!("leave", 8, "~ave";
        verb: "~ft";
    ),
    // Overeat: 8 words (overeat, outeat, undereat, sleepeat, uneat, miseat).
    conjugation!("overeat", 8, "~e~";
        verb: "~~e";
    ),
    // Wake: 7 words (awake, wake, wayfare, reawake, forcarve, rewake).
    conjugation!("wake", 7, "~a~";
        verb: "~o~|~a~d";
    ),
    // Light: 7 words (light, flashlight, twilight, floodlight, prelight, overlight).
    conjugation!("light", 7, "~gh~";
        verb: "~~|~gh~ed";
    ),
    // Shiv: 7 words (shiv, rev, nav, improv, chiv, overrev).
    conjugation!("shiv", 7, "~";
        verb: "~ved";
    ),
    // Fly: 6 words (fly, overfly, outfly, refly, befly, upfly).
    conjugation!("fly", 6, "~y";
        verb: "~ew";
    ),
    // Wind: 6 words (wind, upclimb, sidewind, shitpost, pigstick, destink).
    conjugation!("wind", 6, "~i~";
        verb: "~i~ed";
    ),
    // Lose: 5 words (lose, kape, beclepe, unlose, relose).
    conjugation!("lose", 5, "~e";
        verb: "~t";
    ),
    // Dwell: 5 words (dwell, misspell, foresmell, outdwell, bedwell).
    conjugation!("dwell", 5, "~l";
        verb: "~t|~led";
    ),
    // Flee: 5 words (flee, shoe, beflee, outflee, arede).
    conjugation!("flee", 5, "~e";
        verb: "~d";
    ),
    // Belay: 5 words (why, belay, daresay, apay, appay).
    conjugation!("belay", 5, "~y";
        verb: "~yed|~id";
    ),
    // Melt: 5 words (melt, speedwalk, dispeed, formelt, bespeed).
    conjugation!("melt", 5, "~e~";
        verb: "~e~ed";
    ),
    // Gale: 5 words (gale, begrave, afare, forfare, overfare).
    conjugation!("gale", 5, "~a~";
        verb: "~a~d|~o~";
    ),
    // Abide: 4 words (abide, drite, overbide, bedrite).
    conjugation!("abide", 4, "~i~e";
        verb: "~o~e";
    ),
    // Bid: 4 words (bid, misbid, beshit, unshit).
    conjugation!("bid", 4, "~i~";
        verb: "~i~";
    ),
    // Plead: 4 words (plead, rehang, implead, interplead).
    conjugation!("plead", 4, "~a~";
        verb: "~a~ed";
    ),
    // Shear: 3 words (shear, sneak, reshear).
    conjugation!("shear", 3, "~ea~";
        verb: "~ea~ed|~o~e";
    ),
    // Slay: 3 words (slay, foreslay, reslay).
    conjugation!("slay", 3, "~ay";
        verb: "~ew";
    ),
    // Trolley: 3 words (monkey, bogey, trolley).
    conjugation!("trolley", 3, "~~y";
        verb: "~~yed|~i~d";
    ),
    // Chide: 3 words (chide, beshite, bechide).
    conjugation!("chide", 3, "~i~e";
        verb: "~i~|~i~ed|~o~e";
    ),
    // Bede: 3 words (bede, atren, overren).
    conjugation!("bede", 3, "~e~";
        verb: "~a~";
    ),
];
