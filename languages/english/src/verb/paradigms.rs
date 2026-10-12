//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, then a row of patterns per line (`referred`: the form
/// is another cell's, as the word's feature says):
/// `verb` (inf, _3sg, past, prp, pp)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal;
     verb: $c0:tt, $c1:tt, $c2:tt, $c3:tt, $c4:tt;
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
                flexion::form!($c4),
            ]),
        }
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `have`: 16,893 words.
    Have,
    /// Like `burn`: 13,595 words.
    Burn,
    /// Like `pass`: 2,042 words.
    Pass,
    /// Like `carry`: 1,477 words.
    Carry,
    /// Like `drop`: 493 words.
    Drop,
    /// Like `spot`: 402 words.
    Spot,
    /// Like `bag`: 313 words.
    Bag,
    /// Like `skin`: 214 words.
    Skin,
    /// Like `hear`: 183 words.
    Hear,
    /// Like `scrub`: 182 words.
    Scrub,
    /// Like `slam`: 179 words.
    Slam,
    /// Like `cast`: 141 words.
    Cast,
    /// Like `stir`: 130 words.
    Stir,
    /// Like `set`: 128 words.
    Set,
    /// Like `shred`: 116 words.
    Shred,
    /// Like `distil`: 89 words.
    Distil,
    /// Like `bend`: 68 words.
    Bend,
    /// Like `die`: 62 words.
    Die,
    /// Like `bleed`: 61 words.
    Bleed,
    /// Like `bind`: 60 words.
    Bind,
    /// Like `keep`: 56 words.
    Keep,
    /// Like `panic`: 55 words.
    Panic,
    /// Like `spring`: 46 words.
    Spring,
    /// Like `lay`: 46 words.
    Lay,
    /// Like `blow`: 46 words.
    Blow,
    /// Like `break`: 46 words.
    Break,
    /// Like `take`: 41 words.
    Take,
    /// Like `show`: 40 words.
    Show,
    /// Like `wring`: 39 words.
    Wring,
    /// Like `drive`: 36 words.
    Drive,
    /// Like `fix`: 28 words.
    Fix,
    /// Like `wear`: 28 words.
    Wear,
    /// Like `sit`: 28 words.
    Sit,
    /// Like `write`: 27 words.
    Write,
    /// Like `hold`: 25 words.
    Hold,
    /// Like `stand`: 23 words.
    Stand,
    /// Like `draw`: 23 words.
    Draw,
    /// Like `tell`: 23 words.
    Tell,
    /// Like `queue`: 22 words.
    Queue,
    /// Like `rid`: 22 words.
    Rid,
    /// Like `ride`: 22 words.
    Ride,
    /// Like `make`: 20 words.
    Make,
    /// Like `give`: 18 words.
    Give,
    /// Like `dive`: 18 words.
    Dive,
    /// Like `mislead`: 18 words.
    Mislead,
    /// Like `run`: 18 words.
    Run,
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
    /// Like `overgo`: 16 words.
    Overgo,
    /// Like `should`: 16 words.
    Should,
    /// Like `see`: 15 words.
    See,
    /// Like `beat`: 15 words.
    Beat,
    /// Like `bite`: 15 words.
    Bite,
    /// Like `come`: 15 words.
    Come,
    /// Like `fall`: 14 words.
    Fall,
    /// Like `spill`: 14 words.
    Spill,
    /// Like `spin`: 14 words.
    Spin,
    /// Like `undo`: 13 words.
    Undo,
    /// Like `slide`: 13 words.
    Slide,
    /// Like `weave`: 12 words.
    Weave,
    /// Like `abide`: 11 words.
    Abide,
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
    /// Like `octopus`: 11 words.
    Octopus,
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
    /// Like `strike`: 9 words.
    Strike,
    /// Like `forget`: 8 words.
    Forget,
    /// Like `leave`: 8 words.
    Leave,
    /// Like `overeat`: 8 words.
    Overeat,
    /// Like `swim`: 8 words.
    Swim,
    /// Like `tread`: 8 words.
    Tread,
    /// Like `melt`: 8 words.
    Melt,
    /// Like `gale`: 8 words.
    Gale,
    /// Like `gas`: 7 words.
    Gas,
    /// Like `dig`: 7 words.
    Dig,
    /// Like `light`: 7 words.
    Light,
    /// Like `wind`: 7 words.
    Wind,
    /// Like `shiv`: 7 words.
    Shiv,
    /// Like `win`: 6 words.
    Win,
    /// Like `wake`: 6 words.
    Wake,
    /// Like `choose`: 6 words.
    Choose,
    /// Like `fly`: 6 words.
    Fly,
    /// Like `backslide`: 6 words.
    Backslide,
    /// Like `belay`: 6 words.
    Belay,
    /// Like `larn`: 6 words.
    Larn,
    /// Like `lose`: 5 words.
    Lose,
    /// Like `dwell`: 5 words.
    Dwell,
    /// Like `flee`: 5 words.
    Flee,
    /// Like `get`: 5 words.
    Get,
    /// Like `begin`: 4 words.
    Begin,
    /// Like `plead`: 4 words.
    Plead,
    /// Like `trolley`: 4 words.
    Trolley,
    /// Like `birdie`: 4 words.
    Birdie,
    /// Like `might`: 4 words.
    Might,
    /// Like `slay`: 3 words.
    Slay,
    /// Like `swell`: 3 words.
    Swell,
    /// Like `dress`: 3 words.
    Dress,
    /// Like `braai`: 3 words.
    Braai,
    /// Like `downstrike`: 3 words.
    Downstrike,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 110] = [
        Conjugation::Have,
        Conjugation::Burn,
        Conjugation::Pass,
        Conjugation::Carry,
        Conjugation::Drop,
        Conjugation::Spot,
        Conjugation::Bag,
        Conjugation::Skin,
        Conjugation::Hear,
        Conjugation::Scrub,
        Conjugation::Slam,
        Conjugation::Cast,
        Conjugation::Stir,
        Conjugation::Set,
        Conjugation::Shred,
        Conjugation::Distil,
        Conjugation::Bend,
        Conjugation::Die,
        Conjugation::Bleed,
        Conjugation::Bind,
        Conjugation::Keep,
        Conjugation::Panic,
        Conjugation::Spring,
        Conjugation::Lay,
        Conjugation::Blow,
        Conjugation::Break,
        Conjugation::Take,
        Conjugation::Show,
        Conjugation::Wring,
        Conjugation::Drive,
        Conjugation::Fix,
        Conjugation::Wear,
        Conjugation::Sit,
        Conjugation::Write,
        Conjugation::Hold,
        Conjugation::Stand,
        Conjugation::Draw,
        Conjugation::Tell,
        Conjugation::Queue,
        Conjugation::Rid,
        Conjugation::Ride,
        Conjugation::Make,
        Conjugation::Give,
        Conjugation::Dive,
        Conjugation::Mislead,
        Conjugation::Run,
        Conjugation::Think,
        Conjugation::Shoot,
        Conjugation::Alight,
        Conjugation::Gild,
        Conjugation::Forbear,
        Conjugation::Overgo,
        Conjugation::Should,
        Conjugation::See,
        Conjugation::Beat,
        Conjugation::Bite,
        Conjugation::Come,
        Conjugation::Fall,
        Conjugation::Spill,
        Conjugation::Spin,
        Conjugation::Undo,
        Conjugation::Slide,
        Conjugation::Weave,
        Conjugation::Abide,
        Conjugation::Quiz,
        Conjugation::Buy,
        Conjugation::Bring,
        Conjugation::Freeze,
        Conjugation::Rec,
        Conjugation::Beteach,
        Conjugation::Octopus,
        Conjugation::Catch,
        Conjugation::Chef,
        Conjugation::Hang,
        Conjugation::Lie,
        Conjugation::Seek,
        Conjugation::Strike,
        Conjugation::Forget,
        Conjugation::Leave,
        Conjugation::Overeat,
        Conjugation::Swim,
        Conjugation::Tread,
        Conjugation::Melt,
        Conjugation::Gale,
        Conjugation::Gas,
        Conjugation::Dig,
        Conjugation::Light,
        Conjugation::Wind,
        Conjugation::Shiv,
        Conjugation::Win,
        Conjugation::Wake,
        Conjugation::Choose,
        Conjugation::Fly,
        Conjugation::Backslide,
        Conjugation::Belay,
        Conjugation::Larn,
        Conjugation::Lose,
        Conjugation::Dwell,
        Conjugation::Flee,
        Conjugation::Get,
        Conjugation::Begin,
        Conjugation::Plead,
        Conjugation::Trolley,
        Conjugation::Birdie,
        Conjugation::Might,
        Conjugation::Slay,
        Conjugation::Swell,
        Conjugation::Dress,
        Conjugation::Braai,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 110] = [
    // Have: 16,893 words (nonsense, minute, trade, adjective, substantive, patronage).
    conjugation!("have", 16893, "~e";
        verb: "~e", "~es", "~ed", "~ing", "~ed";
    ),
    // Burn: 13,595 words (portmanteau, word, book, pound, pond, crow).
    conjugation!("burn", 13595, "~";
        verb: "~", "~s", "~ed", "~ing", "~ed";
    ),
    // Pass: 2,042 words (march, abash, abolish, abroach, abscess, absciss).
    conjugation!("pass", 2042, "~";
        verb: "~", "~es", "~ed", "~ing", "~ed";
    ),
    // Carry: 1,477 words (dictionary, sky, vivify, nullify, treaty, policy).
    conjugation!("carry", 1477, "~y";
        verb: "~y", "~ies", "~ied", "~ying", "~ied";
    ),
    // Drop: 493 words (swap, swop, crap, map, pip, bap).
    conjugation!("drop", 493, "~";
        verb: "~", "~s", "~ped", "~ping", "~ped";
    ),
    // Spot: 402 words (cat, abet, abut, het, bit, bot).
    conjugation!("spot", 402, "~";
        verb: "~", "~s", "~ted", "~ting", "~ted";
    ),
    // Bag: 313 words (big, gag, drug, twig, beg, leg).
    conjugation!("bag", 313, "~";
        verb: "~", "~s", "~ged", "~ging", "~ged";
    ),
    // Skin: 214 words (van, in, sun, man, thin, son).
    conjugation!("skin", 214, "~";
        verb: "~", "~s", "~ned", "~ning", "~ned";
    ),
    // Hear: 183 words (free, pie, thee, toe, cooee, tree).
    conjugation!("hear", 183, "~";
        verb: "~", "~s", "~d", "~ing", "~d";
    ),
    // Scrub: 182 words (ab, lob, dub, bib, stab, pub).
    conjugation!("scrub", 182, "~";
        verb: "~", "~s", "~bed", "~bing", "~bed";
    ),
    // Slam: 179 words (dotcom, cum, um, stem, plum, om).
    conjugation!("slam", 179, "~";
        verb: "~", "~s", "~med", "~ming", "~med";
    ),
    // Cast: 141 words (forecast, cast, read, broadcast, burst, cost).
    conjugation!("cast", 141, "~";
        verb: "~", "~s", "~", "~ing", "~";
    ),
    // Stir: 130 words (abhor, star, war, defer, upbar, er).
    conjugation!("stir", 130, "~";
        verb: "~", "~s", "~red", "~ring", "~red";
    ),
    // Set: 128 words (offset, yet, bet, cut, hit, input).
    conjugation!("set", 128, "~";
        verb: "~", "~s", "~", "~ting", "~";
    ),
    // Shred: 116 words (cod, red, squid, god, bad, bed).
    conjugation!("shred", 116, "~";
        verb: "~", "~s", "~ded", "~ding", "~ded";
    ),
    // Distil: 89 words (canal, control, appel, halal, compel, gel).
    conjugation!("distil", 89, "~";
        verb: "~", "~s", "~led", "~ling", "~led";
    ),
    // Bend: 68 words (send, bend, build, lend, rebuild, spend).
    conjugation!("bend", 68, "~d";
        verb: "~d", "~ds", "~t", "~ding", "~t";
    ),
    // Die: 62 words (die, noogie, tie, sie, auntie, vie).
    conjugation!("die", 62, "~~e";
        verb: "~~e", "~~es", "~~ed", "~y~ng", "~~ed";
    ),
    // Bleed: 61 words (upbreed, bleed, breed, feed, interbreed, meet).
    conjugation!("bleed", 61, "~e~";
        verb: "~e~", "~e~s", "~~", "~e~ing", "~~";
    ),
    // Bind: 60 words (upbind, bind, fight, find, grind, rebind).
    conjugation!("bind", 60, "~i~";
        verb: "~i~", "~i~s", "~ou~", "~i~ing", "~ou~";
    ),
    // Keep: 56 words (creep, feel, keep, kneel, oversleep, sleep).
    conjugation!("keep", 56, "~e~";
        verb: "~e~", "~e~s", "~~t", "~e~ing", "~~t";
    ),
    // Panic: 55 words (metric, grok, music, hydraulic, yak, traffic).
    conjugation!("panic", 55, "~";
        verb: "~", "~s", "~ked", "~king", "~ked";
    ),
    // Spring: 46 words (spring, shrink, drink, sing, stink, sink).
    conjugation!("spring", 46, "~i~";
        verb: "~i~", "~i~s", "~a~", "~i~ing", "~u~";
    ),
    // Lay: 46 words (say, lay, mislay, overlay, overpay, pay).
    conjugation!("lay", 46, "~y";
        verb: "~y", "~ys", "~id", "~ying", "~id";
    ),
    // Blow: 46 words (know, upblow, blow, grow, outgrow, overthrow).
    conjugation!("blow", 46, "~o~";
        verb: "~o~", "~o~s", "~e~", "~o~ing", "~o~n";
    ),
    // Break: 46 words (upbreak, break, speak, housebreak, misspeak, outbreak).
    conjugation!("break", 46, "~ea~";
        verb: "~ea~", "~ea~s", "~o~e", "~ea~ing", "~o~en";
    ),
    // Take: 41 words (take, forsake, mistake, overtake, partake, shake).
    conjugation!("take", 41, "~a~e";
        verb: "~a~e", "~a~es", "~oo~", "~a~ing", "~a~en";
    ),
    // Show: 40 words (shew, mow, sew, show, strew, sow).
    conjugation!("show", 40, "~";
        verb: "~", "~s", "~ed", "~ing", "~n";
    ),
    // Wring: 39 words (wring, cling, fling, hamstring, sling, sting).
    conjugation!("wring", 39, "~i~";
        verb: "~i~", "~i~s", "~u~", "~i~ing", "~u~";
    ),
    // Drive: 36 words (arise, drive, rise, strive, uprise, shrive).
    conjugation!("drive", 36, "~i~e";
        verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~i~en";
    ),
    // Fix: 28 words (deal, born, gar, fix, misdeal, daydream).
    conjugation!("fix", 28, "~";
        verb: "~", "~s", "~t", "~ing", "~t";
    ),
    // Wear: 28 words (abear, tear, swear, wear, forswear, foreswear).
    conjugation!("wear", 28, "~ea~";
        verb: "~ea~", "~ea~s", "~o~e", "~ea~ing", "~o~n";
    ),
    // Sit: 28 words (shit, sit, babysit, resit, housesit, birdsit).
    conjugation!("sit", 28, "~i~";
        verb: "~i~", "~i~s", "~a~", "~i~ting", "~a~";
    ),
    // Write: 27 words (write, rewrite, underwrite, smite, ghostwrite, typewrite).
    conjugation!("write", 27, "~i~e";
        verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~i~ten";
    ),
    // Hold: 25 words (behold, hold, uphold, withhold, stranglehold, handhold).
    conjugation!("hold", 25, "~o~";
        verb: "~o~", "~o~s", "~e~", "~o~ing", "~e~";
    ),
    // Stand: 23 words (stand, misunderstand, understand, withstand, handstand, headstand).
    conjugation!("stand", 23, "~an~";
        verb: "~an~", "~an~s", "~oo~", "~an~ing", "~oo~";
    ),
    // Draw: 23 words (draw, overdraw, withdraw, updraw, redraw, outdraw).
    conjugation!("draw", 23, "~a~";
        verb: "~a~", "~a~s", "~e~", "~a~ing", "~a~n";
    ),
    // Tell: 23 words (foretell, outsell, oversell, resell, sell, tell).
    conjugation!("tell", 23, "~e~l";
        verb: "~e~l", "~e~ls", "~o~d", "~e~ling", "~o~d";
    ),
    // Queue: 22 words (blue, true, horseshoe, coyote, meme, queue).
    conjugation!("queue", 22, "~e";
        verb: "~e", "~es", "~ed", "~eing|~ing", "~ed";
    ),
    // Rid: 22 words (bid, outbid, rid, shed, wed, clad).
    conjugation!("rid", 22, "~";
        verb: "~", "~s", "~", "~ding", "~";
    ),
    // Ride: 22 words (override, ride, stride, joyride, hagride, outride).
    conjugation!("ride", 22, "~i~e";
        verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~i~den";
    ),
    // Make: 20 words (make, remake, matchmake, unmake, premake, merrymake).
    conjugation!("make", 20, "~ke";
        verb: "~ke", "~kes", "~de", "~king", "~de";
    ),
    // Give: 18 words (give, forgive, upgive, foregive, misgive, regive).
    conjugation!("give", 18, "~i~e";
        verb: "~i~e", "~i~es", "~a~e", "~i~ing", "~i~en";
    ),
    // Dive: 18 words (dive, updive, skydive, shite, rive, nosedive).
    conjugation!("dive", 18, "~i~e";
        verb: "~i~e", "~i~es", "~i~ed|~o~e", "~i~ing", "~i~ed";
    ),
    // Mislead: 18 words (mislead, uplead, colead, cheerlead, underlead, aread).
    conjugation!("mislead", 18, "~a~";
        verb: "~a~", "~a~s", "~~", "~a~ing", "~~";
    ),
    // Run: 18 words (outrun, overrun, rerun, run, assrun, speedrun).
    conjugation!("run", 18, "~u~";
        verb: "~u~", "~u~s", "~a~", "~u~ning", "~u~";
    ),
    // Think: 17 words (think, rethink, doublethink, outthink, crimethink, misthink).
    conjugation!("think", 17, "~ink";
        verb: "~ink", "~inks", "~ought", "~inking", "~ought";
    ),
    // Shoot: 17 words (overshoot, shoot, troubleshoot, undershoot, reshoot, sharpshoot).
    conjugation!("shoot", 17, "~o~";
        verb: "~o~", "~o~s", "~~", "~o~ing", "~~";
    ),
    // Alight: 17 words (alight, relight, gaslight, spotlight, daylight, backlight).
    conjugation!("alight", 17, "~gh~";
        verb: "~gh~", "~gh~s", "~gh~ed|~~", "~gh~ing", "~gh~ed|~~";
    ),
    // Gild: 17 words (blend, undergird, gird, gild, backbend, regild).
    conjugation!("gild", 17, "~d";
        verb: "~d", "~ds", "~ded|~t", "~ding", "~ded|~t";
    ),
    // Forbear: 16 words (upbear, forebear, forbear, overbear, rebear, underbear).
    conjugation!("forbear", 16, "~ea~";
        verb: "~ea~", "~ea~s", "~o~e", "~ea~ing", "~o~ne";
    ),
    // Overgo: 16 words (forego, forgo, undergo, togo, outgo, misgo).
    conjugation!("overgo", 16, "~go";
        verb: "~go", "~goes", "~went", "~going", "~gone";
    ),
    // Should: 16 words (may, can, got, will, shall, should).
    conjugation!("should", 16, "~";
        verb: "~", "~", "~ed", "~ing", "~ed";
    ),
    // See: 15 words (see, foresee, oversee, forsee, sightsee, unsee).
    conjugation!("see", 15, "~ee";
        verb: "~ee", "~ees", "~aw", "~eeing", "~een";
    ),
    // Beat: 15 words (be, beat, browbeat, wright, tyek, forbeat).
    conjugation!("beat", 15, "~";
        verb: "~", "~s", "~", "~ing", "~en";
    ),
    // Bite: 15 words (bite, frostbite, backbite, overbite, abite, forbite).
    conjugation!("bite", 15, "~e";
        verb: "~e", "~es", "~", "~ing", "~ten";
    ),
    // Come: 15 words (become, come, overcome, misbecome, oncome, undercome).
    conjugation!("come", 15, "~o~e";
        verb: "~o~e", "~o~es", "~a~e", "~o~ing", "~o~e";
    ),
    // Fall: 14 words (fall, befall, eat, freefall, pratfall, downfall).
    conjugation!("fall", 14, "~a~";
        verb: "~a~", "~a~s", "~e~", "~a~ing", "~a~en";
    ),
    // Spill: 14 words (smell, spill, respell, indwell, overspill, outspell).
    conjugation!("spill", 14, "~l";
        verb: "~l", "~ls", "~led|~t", "~ling", "~led|~t";
    ),
    // Spin: 14 words (spin, tailspin, backspin, topspin, overspin, wheelspin).
    conjugation!("spin", 14, "~i~";
        verb: "~i~", "~i~s", "~u~", "~i~ning", "~u~";
    ),
    // Undo: 13 words (do, go, outdo, overdo, redo, undo).
    conjugation!("undo", 13, "~o";
        verb: "~o", "~oes", "~id", "~oing", "~one";
    ),
    // Slide: 13 words (slide, chide, landslide, rede, betide, slite).
    conjugation!("slide", 13, "~e";
        verb: "~e", "~es", "~", "~ing", "~";
    ),
    // Weave: 12 words (weave, interweave, reweave, forcleave, unweave, tocleave).
    conjugation!("weave", 12, "~~ave";
        verb: "~~ave", "~~aves", "~ov~", "~~aving", "~ov~n";
    ),
    // Abide: 11 words (abide, shine, bestride, outshine, overstride, beshine).
    conjugation!("abide", 11, "~i~e";
        verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~o~e";
    ),
    // Quiz: 11 words (quiz, spaz, whiz, buz, wiz, diz).
    conjugation!("quiz", 11, "~";
        verb: "~", "~zes", "~zed", "~zing", "~zed";
    ),
    // Buy: 11 words (buy, rebuy, overbuy, underbuy, prebuy, forbuy).
    conjugation!("buy", 11, "~~y";
        verb: "~~y", "~~ys", "~o~ght", "~~ying", "~o~ght";
    ),
    // Bring: 11 words (bring, upbring, underbring, forthbring, inbring, imbring).
    conjugation!("bring", 11, "~in~";
        verb: "~in~", "~in~s", "~ou~ht", "~in~ing", "~ou~ht";
    ),
    // Freeze: 11 words (freeze, unfreeze, refreeze, enfreeze, befreeze, prefreeze).
    conjugation!("freeze", 11, "~~eze";
        verb: "~~eze", "~~ezes", "~oz~", "~~ezing", "~oz~n";
    ),
    // Rec: 11 words (sic, floc, tic, vac, proc, fanfic).
    conjugation!("rec", 11, "~";
        verb: "~", "~s", "~ced", "~cing", "~ced";
    ),
    // Beteach: 11 words (teach, reteach, overteach, misteach, beteach, unteach).
    conjugation!("beteach", 11, "~e~c~";
        verb: "~e~c~", "~e~c~es", "~~ug~t", "~e~c~ing", "~~ug~t";
    ),
    // Octopus: 11 words (octopus, nonplus, dis, pus, res, dorcas).
    conjugation!("octopus", 11, "~";
        verb: "~", "~ses", "~sed", "~sing", "~sed";
    ),
    // Catch: 10 words (catch, bycatch, recatch, overcatch, becatch, outcatch).
    conjugation!("catch", 10, "~tc~";
        verb: "~tc~", "~tc~es", "~ug~t", "~tc~ing", "~ug~t";
    ),
    // Chef: 10 words (ref, chef, crossref, coif, stuf, indef).
    conjugation!("chef", 10, "~";
        verb: "~", "~s", "~fed", "~fing", "~fed";
    ),
    // Hang: 9 words (hang, overhang, uphang, cliffhang, behang, unhang).
    conjugation!("hang", 9, "~a~";
        verb: "~a~", "~a~s", "~u~", "~a~ing", "~u~";
    ),
    // Lie: 9 words (lie, overlie, underlie, belie, superlie, forelie).
    conjugation!("lie", 9, "~ie";
        verb: "~ie", "~ies", "~ay", "~ying", "~ain";
    ),
    // Seek: 9 words (seek, upseek, beseek, outseek, reseek, misseek).
    conjugation!("seek", 9, "~eek";
        verb: "~eek", "~eeks", "~ought", "~eeking", "~ought";
    ),
    // Strike: 9 words (strike, airstrike, restrike, counterstrike, overstrike, outstrike).
    conjugation!("strike", 9, "~i~e";
        verb: "~i~e", "~i~es", "~uc~", "~i~ing", "~uc~";
    ),
    // Forget: 8 words (forget, beget, unbeget, misget, unforget, rebeget).
    conjugation!("forget", 8, "~e~";
        verb: "~e~", "~e~s", "~o~", "~e~ting", "~o~ten";
    ),
    // Leave: 8 words (leave, cleave, beleave, toreave, dreave, forleave).
    conjugation!("leave", 8, "~ave";
        verb: "~ave", "~aves", "~ft", "~aving", "~ft";
    ),
    // Overeat: 8 words (overeat, outeat, undereat, sleepeat, uneat, miseat).
    conjugation!("overeat", 8, "~e~";
        verb: "~e~", "~e~s", "~~e", "~e~ing", "~e~en";
    ),
    // Swim: 8 words (swim, nim, benim, outswim, overnim, overswim).
    conjugation!("swim", 8, "~i~";
        verb: "~i~", "~i~s", "~a~", "~i~ming", "~u~";
    ),
    // Tread: 8 words (tread, totread, fortread, betread, untread, overtread).
    conjugation!("tread", 8, "~ea~";
        verb: "~ea~", "~ea~s", "~o~", "~ea~ing", "~o~den";
    ),
    // Melt: 8 words (melt, speedwalk, upswell, outswell, dispeed, formelt).
    conjugation!("melt", 8, "~e~";
        verb: "~e~", "~e~s", "~e~ed", "~e~ing", "~e~ed";
    ),
    // Gale: 8 words (gale, handshake, bename, begrave, bedare, afare).
    conjugation!("gale", 8, "~a~e";
        verb: "~a~e", "~a~es", "~a~ed|~o~e", "~a~ing", "~a~ed|~o~e|~a~en";
    ),
    // Gas: 7 words (gas, yes, teargas, outgas, degas, regas).
    conjugation!("gas", 7, "~";
        verb: "~", "~es|~ses", "~sed", "~sing", "~sed";
    ),
    // Dig: 7 words (dig, underdig, undig, redig, misdig, outdig).
    conjugation!("dig", 7, "~i~";
        verb: "~i~", "~i~s", "~u~", "~i~ging", "~u~";
    ),
    // Light: 7 words (light, flashlight, twilight, floodlight, prelight, overlight).
    conjugation!("light", 7, "~gh~";
        verb: "~gh~", "~gh~s", "~~|~gh~ed", "~gh~ing", "~~|~gh~ed";
    ),
    // Wind: 7 words (wind, upclimb, heatsink, sidewind, shitpost, pigstick).
    conjugation!("wind", 7, "~i~";
        verb: "~i~", "~i~s", "~i~ed", "~i~ing", "~i~ed";
    ),
    // Shiv: 7 words (shiv, rev, nav, improv, chiv, overrev).
    conjugation!("shiv", 7, "~";
        verb: "~", "~s", "~ved", "~ving", "~ved";
    ),
    // Win: 6 words (win, overwin, rewin, outwin, bewin, electrowin).
    conjugation!("win", 6, "~i~";
        verb: "~i~", "~i~s", "~o~", "~i~ning", "~o~";
    ),
    // Wake: 6 words (awake, wake, reawake, forcarve, rewake, outwake).
    conjugation!("wake", 6, "~a~e";
        verb: "~a~e", "~a~es", "~o~e|~a~ed", "~a~ing", "~o~en|~a~ed";
    ),
    // Choose: 6 words (choose, forechoose, rechoose, mischoose, unchoose, prechoose).
    conjugation!("choose", 6, "~o~e";
        verb: "~o~e", "~o~es", "~~e", "~o~ing", "~~en";
    ),
    // Fly: 6 words (fly, overfly, outfly, refly, befly, upfly).
    conjugation!("fly", 6, "~y";
        verb: "~y", "~ies", "~ew", "~ying", "~own";
    ),
    // Backslide: 6 words (hide, backslide, unhide, rehide, autohide, hyde).
    conjugation!("backslide", 6, "~e";
        verb: "~e", "~es", "~", "~ing", "~den";
    ),
    // Belay: 6 words (why, belay, daresay, apay, beflay, appay).
    conjugation!("belay", 6, "~y";
        verb: "~y", "~ys", "~yed|~id", "~ying", "~yed|~id";
    ),
    // Larn: 6 words (de, dee, myek, deek, larn, stott).
    conjugation!("larn", 6, "~";
        verb: "~", "~s", "~ed", "~in", "~ed";
    ),
    // Lose: 5 words (lose, kape, beclepe, unlose, relose).
    conjugation!("lose", 5, "~e";
        verb: "~e", "~es", "~t", "~ing", "~t";
    ),
    // Dwell: 5 words (dwell, misspell, foresmell, outdwell, bedwell).
    conjugation!("dwell", 5, "~l";
        verb: "~l", "~ls", "~t|~led", "~ling", "~t|~led";
    ),
    // Flee: 5 words (flee, shoe, beflee, outflee, arede).
    conjugation!("flee", 5, "~e";
        verb: "~e", "~es", "~d", "~eing", "~d";
    ),
    // Get: 5 words (get, reget, unget, overget, underget).
    conjugation!("get", 5, "~e~";
        verb: "~e~", "~e~s", "~o~", "~e~ting", "~o~|~o~ten";
    ),
    // Begin: 4 words (begin, rin, rebegin, atrin).
    conjugation!("begin", 4, "~i~";
        verb: "~i~", "~i~s", "~a~", "~i~ning", "~u~";
    ),
    // Plead: 4 words (plead, rehang, implead, interplead).
    conjugation!("plead", 4, "~a~";
        verb: "~a~", "~a~s", "~a~ed", "~a~ing", "~a~ed";
    ),
    // Trolley: 4 words (monkey, bogey, curtsey, trolley).
    conjugation!("trolley", 4, "~~y";
        verb: "~~y", "~~ys", "~~yed|~i~d", "~~ying", "~~yed|~i~d";
    ),
    // Birdie: 4 words (wheelie, stymie, birdie, assoilzie).
    conjugation!("birdie", 4, "~~e";
        verb: "~~e", "~~es", "~~ed", "~~eing|~y~ng", "~~ed";
    ),
    // Might: 4 words (might, must, gaf, wfh).
    conjugation!("might", 4, "~";
        verb: "~", "~", "~", "~", "~";
    ),
    // Slay: 3 words (slay, foreslay, reslay).
    conjugation!("slay", 3, "~ay";
        verb: "~ay", "~ays", "~ew", "~aying", "~ain";
    ),
    // Swell: 3 words (swell, unswell, reswell).
    conjugation!("swell", 3, "~e~";
        verb: "~e~", "~e~s", "~e~ed", "~e~ing", "~o~en|~e~ed";
    ),
    // Dress: 3 words (dress, press, possess).
    conjugation!("dress", 3, "~s";
        verb: "~s", "~ses", "~sed", "~sing", "~sed|~t";
    ),
    // Braai: 3 words (braai, luigi, skibidi).
    conjugation!("braai", 3, "~";
        verb: "~", "~s", "~ed", "~ng", "~ed";
    ),
    // Downstrike: 3 words (thunderstrike, downstrike, copystrike).
    conjugation!("downstrike", 3, "~i~e";
        verb: "~i~e", "~i~es", "~uc~", "~i~ing", "~ic~en";
    ),
];
