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
    /// Like `have`: 17,193 words.
    Have,
    /// Like `burn`: 15,607 words.
    Burn,
    /// Like `carry`: 1,476 words.
    Carry,
    /// Like `drop`: 493 words.
    Drop,
    /// Like `spot`: 398 words.
    Spot,
    /// Like `run`: 320 words.
    Run,
    /// Like `bag`: 313 words.
    Bag,
    /// Like `draw`: 220 words.
    Draw,
    /// Like `skin`: 213 words.
    Skin,
    /// Like `scrub`: 182 words.
    Scrub,
    /// Like `slam`: 177 words.
    Slam,
    /// Like `stir`: 130 words.
    Stir,
    /// Like `spring`: 116 words.
    Spring,
    /// Like `shred`: 114 words.
    Shred,
    /// Like `distil`: 87 words.
    Distil,
    /// Like `bend`: 68 words.
    Bend,
    /// Like `bleed`: 61 words.
    Bleed,
    /// Like `bind`: 60 words.
    Bind,
    /// Like `keep`: 56 words.
    Keep,
    /// Like `panic`: 55 words.
    Panic,
    /// Like `lay`: 46 words.
    Lay,
    /// Like `wear`: 44 words.
    Wear,
    /// Like `break`: 44 words.
    Break,
    /// Like `write`: 39 words.
    Write,
    /// Like `fall`: 38 words.
    Fall,
    /// Like `overgo`: 29 words.
    Overgo,
    /// Like `fix`: 28 words.
    Fix,
    /// Like `ride`: 28 words.
    Ride,
    /// Like `sit`: 27 words.
    Sit,
    /// Like `hold`: 25 words.
    Hold,
    /// Like `stand`: 23 words.
    Stand,
    /// Like `tell`: 23 words.
    Tell,
    /// Like `make`: 20 words.
    Make,
    /// Like `gas`: 18 words.
    Gas,
    /// Like `mislead`: 18 words.
    Mislead,
    /// Like `win`: 17 words.
    Win,
    /// Like `think`: 17 words.
    Think,
    /// Like `shoot`: 17 words.
    Shoot,
    /// Like `alight`: 17 words.
    Alight,
    /// Like `gild`: 17 words.
    Gild,
    /// Like `forbear`: 16 words.
    Forbear,
    /// Like `spill`: 14 words.
    Spill,
    /// Like `clepe`: 14 words.
    Clepe,
    /// Like `slide`: 13 words.
    Slide,
    /// Like `quiz`: 11 words.
    Quiz,
    /// Like `buy`: 11 words.
    Buy,
    /// Like `bring`: 11 words.
    Bring,
    /// Like `freeze`: 11 words.
    Freeze,
    /// Like `rec`: 11 words.
    Rec,
    /// Like `beteach`: 11 words.
    Beteach,
    /// Like `catch`: 10 words.
    Catch,
    /// Like `chef`: 10 words.
    Chef,
    /// Like `hang`: 9 words.
    Hang,
    /// Like `lie`: 9 words.
    Lie,
    /// Like `seek`: 9 words.
    Seek,
    /// Like `dive`: 8 words.
    Dive,
    /// Like `forget`: 8 words.
    Forget,
    /// Like `leave`: 8 words.
    Leave,
    /// Like `melt`: 8 words.
    Melt,
    /// Like `restrike`: 8 words.
    Restrike,
    /// Like `light`: 7 words.
    Light,
    /// Like `tread`: 7 words.
    Tread,
    /// Like `wind`: 7 words.
    Wind,
    /// Like `shiv`: 7 words.
    Shiv,
    /// Like `wake`: 6 words.
    Wake,
    /// Like `choose`: 6 words.
    Choose,
    /// Like `fly`: 6 words.
    Fly,
    /// Like `get`: 6 words.
    Get,
    /// Like `belay`: 6 words.
    Belay,
    /// Like `lose`: 5 words.
    Lose,
    /// Like `dwell`: 5 words.
    Dwell,
    /// Like `flee`: 5 words.
    Flee,
    /// Like `gale`: 5 words.
    Gale,
    /// Like `doe`: 5 words.
    Doe,
    /// Like `forbid`: 4 words.
    Forbid,
    /// Like `plead`: 4 words.
    Plead,
    /// Like `bite`: 3 words.
    Bite,
    /// Like `bid`: 3 words.
    Bid,
    /// Like `slay`: 3 words.
    Slay,
    /// Like `swell`: 3 words.
    Swell,
    /// Like `trolley`: 3 words.
    Trolley,
    /// Like `dress`: 3 words.
    Dress,
    /// Like `bide`: 3 words.
    Bide,
    /// Like `downstrike`: 3 words.
    Downstrike,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 84] = [
        Conjugation::Have,
        Conjugation::Burn,
        Conjugation::Carry,
        Conjugation::Drop,
        Conjugation::Spot,
        Conjugation::Run,
        Conjugation::Bag,
        Conjugation::Draw,
        Conjugation::Skin,
        Conjugation::Scrub,
        Conjugation::Slam,
        Conjugation::Stir,
        Conjugation::Spring,
        Conjugation::Shred,
        Conjugation::Distil,
        Conjugation::Bend,
        Conjugation::Bleed,
        Conjugation::Bind,
        Conjugation::Keep,
        Conjugation::Panic,
        Conjugation::Lay,
        Conjugation::Wear,
        Conjugation::Break,
        Conjugation::Write,
        Conjugation::Fall,
        Conjugation::Overgo,
        Conjugation::Fix,
        Conjugation::Ride,
        Conjugation::Sit,
        Conjugation::Hold,
        Conjugation::Stand,
        Conjugation::Tell,
        Conjugation::Make,
        Conjugation::Gas,
        Conjugation::Mislead,
        Conjugation::Win,
        Conjugation::Think,
        Conjugation::Shoot,
        Conjugation::Alight,
        Conjugation::Gild,
        Conjugation::Forbear,
        Conjugation::Spill,
        Conjugation::Clepe,
        Conjugation::Slide,
        Conjugation::Quiz,
        Conjugation::Buy,
        Conjugation::Bring,
        Conjugation::Freeze,
        Conjugation::Rec,
        Conjugation::Beteach,
        Conjugation::Catch,
        Conjugation::Chef,
        Conjugation::Hang,
        Conjugation::Lie,
        Conjugation::Seek,
        Conjugation::Dive,
        Conjugation::Forget,
        Conjugation::Leave,
        Conjugation::Melt,
        Conjugation::Restrike,
        Conjugation::Light,
        Conjugation::Tread,
        Conjugation::Wind,
        Conjugation::Shiv,
        Conjugation::Wake,
        Conjugation::Choose,
        Conjugation::Fly,
        Conjugation::Get,
        Conjugation::Belay,
        Conjugation::Lose,
        Conjugation::Dwell,
        Conjugation::Flee,
        Conjugation::Gale,
        Conjugation::Doe,
        Conjugation::Forbid,
        Conjugation::Plead,
        Conjugation::Bite,
        Conjugation::Bid,
        Conjugation::Slay,
        Conjugation::Swell,
        Conjugation::Trolley,
        Conjugation::Dress,
        Conjugation::Bide,
        Conjugation::Downstrike,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 84] = [
    // Have: 17,193 words (free, nonsense, pie, minute, trade, adjective).
    conjugation!("have", 17193, "~";
        verb: "~d";
    ),
    // Burn: 15,607 words (portmanteau, word, book, pound, pond, crow).
    conjugation!("burn", 15607, "~";
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
    // Spot: 398 words (cat, abet, abut, het, bit, bot).
    conjugation!("spot", 398, "~";
        verb: "~ted";
    ),
    // Run: 320 words (forecast, offset, yet, cast, read, become).
    conjugation!("run", 320, "~";
        verb: "~";
    ),
    // Bag: 313 words (big, gag, drug, twig, beg, leg).
    conjugation!("bag", 313, "~";
        verb: "~ged";
    ),
    // Draw: 220 words (take, know, give, see, shew, upblow).
    conjugation!("draw", 220, "~";
        verb: "~n";
    ),
    // Skin: 213 words (van, in, can, sun, man, thin).
    conjugation!("skin", 213, "~";
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
    // Spring: 116 words (spring, shrink, drink, wring, sing, begin).
    conjugation!("spring", 116, "~i~";
        verb: "~u~";
    ),
    // Shred: 114 words (cod, red, squid, god, bad, bed).
    conjugation!("shred", 114, "~";
        verb: "~ded";
    ),
    // Distil: 87 words (canal, control, appel, halal, compel, gel).
    conjugation!("distil", 87, "~";
        verb: "~led";
    ),
    // Bend: 68 words (send, bend, build, lend, rebuild, spend).
    conjugation!("bend", 68, "~d";
        verb: "~t";
    ),
    // Bleed: 61 words (upbreed, bleed, breed, feed, interbreed, meet).
    conjugation!("bleed", 61, "~e~";
        verb: "~~";
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
    // Wear: 44 words (abear, weave, tear, interweave, shear, swear).
    conjugation!("wear", 44, "~ea~";
        verb: "~o~n";
    ),
    // Break: 44 words (upbreak, break, speak, housebreak, misspeak, outbreak).
    conjugation!("break", 44, "~ea~";
        verb: "~o~en";
    ),
    // Write: 39 words (write, rewrite, underwrite, smite, frostbite, backbite).
    conjugation!("write", 39, "~~";
        verb: "~t~n";
    ),
    // Fall: 38 words (be, fall, beat, befall, browbeat, eat).
    conjugation!("fall", 38, "~";
        verb: "~en";
    ),
    // Overgo: 29 words (do, forego, forgo, go, outdo, overdo).
    conjugation!("overgo", 29, "~";
        verb: "~ne";
    ),
    // Fix: 28 words (deal, born, gar, fix, misdeal, daydream).
    conjugation!("fix", 28, "~";
        verb: "~t";
    ),
    // Ride: 28 words (hide, override, ride, stride, joyride, backslide).
    conjugation!("ride", 28, "~~";
        verb: "~d~n";
    ),
    // Sit: 27 words (shit, sit, babysit, resit, housesit, birdsit).
    conjugation!("sit", 27, "~i~";
        verb: "~a~";
    ),
    // Hold: 25 words (behold, hold, uphold, withhold, stranglehold, handhold).
    conjugation!("hold", 25, "~o~";
        verb: "~e~";
    ),
    // Stand: 23 words (stand, misunderstand, understand, withstand, handstand, headstand).
    conjugation!("stand", 23, "~an~";
        verb: "~oo~";
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
    // Win: 17 words (abide, win, shine, bestride, outshine, overstride).
    conjugation!("win", 17, "~i~";
        verb: "~o~";
    ),
    // Think: 17 words (think, rethink, doublethink, outthink, crimethink, misthink).
    conjugation!("think", 17, "~ink";
        verb: "~ought";
    ),
    // Shoot: 17 words (overshoot, shoot, troubleshoot, undershoot, reshoot, sharpshoot).
    conjugation!("shoot", 17, "~o~";
        verb: "~~";
    ),
    // Alight: 17 words (alight, relight, gaslight, spotlight, daylight, backlight).
    conjugation!("alight", 17, "~gh~";
        verb: "~gh~ed|~~";
    ),
    // Gild: 17 words (blend, undergird, gird, gild, backbend, regild).
    conjugation!("gild", 17, "~d";
        verb: "~ded|~t";
    ),
    // Forbear: 16 words (upbear, forebear, forbear, overbear, rebear, underbear).
    conjugation!("forbear", 16, "~ea~";
        verb: "~o~ne";
    ),
    // Spill: 14 words (smell, spill, respell, indwell, overspill, outspell).
    conjugation!("spill", 14, "~l";
        verb: "~led|~t";
    ),
    // Clepe: 14 words (horseshoe, glide, perse, hoise, twite, clepe).
    conjugation!("clepe", 14, "~e";
        verb: "~ed";
    ),
    // Slide: 13 words (slide, chide, landslide, rede, betide, slite).
    conjugation!("slide", 13, "~e";
        verb: "~";
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
    // Freeze: 11 words (freeze, unfreeze, refreeze, enfreeze, befreeze, prefreeze).
    conjugation!("freeze", 11, "~ee~";
        verb: "~o~n";
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
    // Chef: 10 words (ref, chef, crossref, coif, stuf, indef).
    conjugation!("chef", 10, "~";
        verb: "~fed";
    ),
    // Hang: 9 words (hang, overhang, uphang, cliffhang, behang, unhang).
    conjugation!("hang", 9, "~a~";
        verb: "~u~";
    ),
    // Lie: 9 words (lie, overlie, underlie, belie, superlie, forelie).
    conjugation!("lie", 9, "~~e";
        verb: "~a~n";
    ),
    // Seek: 9 words (seek, upseek, beseek, outseek, reseek, misseek).
    conjugation!("seek", 9, "~eek";
        verb: "~ought";
    ),
    // Dive: 8 words (dive, rive, gainstrive, clive, snithe, blike).
    conjugation!("dive", 8, "~i~";
        verb: "~i~d";
    ),
    // Forget: 8 words (forget, beget, unbeget, misget, unforget, rebeget).
    conjugation!("forget", 8, "~e~";
        verb: "~o~ten";
    ),
    // Leave: 8 words (leave, cleave, beleave, toreave, dreave, forleave).
    conjugation!("leave", 8, "~ave";
        verb: "~ft";
    ),
    // Melt: 8 words (melt, speedwalk, upswell, outswell, dispeed, formelt).
    conjugation!("melt", 8, "~e~";
        verb: "~e~ed";
    ),
    // Restrike: 8 words (airstrike, restrike, counterstrike, overstrike, outstrike, misstrike).
    conjugation!("restrike", 8, "~i~e";
        verb: "~uc~";
    ),
    // Light: 7 words (light, flashlight, twilight, floodlight, prelight, overlight).
    conjugation!("light", 7, "~gh~";
        verb: "~~|~gh~ed";
    ),
    // Tread: 7 words (tread, totread, fortread, betread, overtread, downtread).
    conjugation!("tread", 7, "~ea~";
        verb: "~o~den";
    ),
    // Wind: 7 words (wind, upclimb, heatsink, sidewind, shitpost, pigstick).
    conjugation!("wind", 7, "~i~";
        verb: "~i~ed";
    ),
    // Shiv: 7 words (shiv, rev, nav, improv, chiv, overrev).
    conjugation!("shiv", 7, "~";
        verb: "~ved";
    ),
    // Wake: 6 words (awake, wake, reawake, forcarve, rewake, outwake).
    conjugation!("wake", 6, "~a~";
        verb: "~o~n|~a~d";
    ),
    // Choose: 6 words (choose, forechoose, rechoose, mischoose, unchoose, prechoose).
    conjugation!("choose", 6, "~o~";
        verb: "~~n";
    ),
    // Fly: 6 words (fly, overfly, outfly, refly, befly, upfly).
    conjugation!("fly", 6, "~y";
        verb: "~own";
    ),
    // Get: 6 words (get, bede, reget, unget, overget, underget).
    conjugation!("get", 6, "~e~";
        verb: "~o~|~o~ten";
    ),
    // Belay: 6 words (why, belay, daresay, apay, beflay, appay).
    conjugation!("belay", 6, "~y";
        verb: "~yed|~id";
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
    // Gale: 5 words (gale, begrave, afare, forfare, overfare).
    conjugation!("gale", 5, "~a~";
        verb: "~a~d|~o~|~a~n";
    ),
    // Doe: 5 words (doe, forgoe, growe, blowe, undergoe).
    conjugation!("doe", 5, "~~";
        verb: "~n~";
    ),
    // Forbid: 4 words (forbid, misbid, downtrod, reforbid).
    conjugation!("forbid", 4, "~";
        verb: "~den";
    ),
    // Plead: 4 words (plead, rehang, implead, interplead).
    conjugation!("plead", 4, "~a~";
        verb: "~a~ed";
    ),
    // Bite: 3 words (bite, inwrite, gristbite).
    conjugation!("bite", 3, "~e";
        verb: "~ten|~";
    ),
    // Bid: 3 words (bid, beshit, unshit).
    conjugation!("bid", 3, "~i~";
        verb: "~i~|~a~";
    ),
    // Slay: 3 words (slay, foreslay, reslay).
    conjugation!("slay", 3, "~y";
        verb: "~in";
    ),
    // Swell: 3 words (swell, unswell, reswell).
    conjugation!("swell", 3, "~e~";
        verb: "~o~en|~e~ed";
    ),
    // Trolley: 3 words (monkey, bogey, trolley).
    conjugation!("trolley", 3, "~~y";
        verb: "~~yed|~i~d";
    ),
    // Dress: 3 words (dress, press, possess).
    conjugation!("dress", 3, "~s";
        verb: "~sed|~t";
    ),
    // Bide: 3 words (facsimile, bide, tole).
    conjugation!("bide", 3, "~~";
        verb: "~~d";
    ),
    // Downstrike: 3 words (swike, thunderstrike, downstrike).
    conjugation!("downstrike", 3, "~~";
        verb: "~c~n";
    ),
];
