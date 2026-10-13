//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the conjugation's it is like,
/// or, in a conjugation like no other, derived):
/// `verb` (inf, _3sg, past, prp, pp)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
     $(verb: $c0:tt, $c1:tt, $c2:tt, $c3:tt, $c4:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
            )),
        }
    };
}

/// The conjugation a conjugation is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Conjugation::$parent as u16)
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `have`: 16,893 words.
    Have,
    /// Like `burn`: 13,627 words.
    Burn,
    /// Like `pass`: 2,042 words.
    Pass,
    /// Like `drop`: 1,813 words.
    Drop,
    /// Like `carry`: 1,477 words.
    Carry,
    /// Like `hear`: 307 words.
    Hear,
    /// Like `bag`: 261 words.
    Bag,
    /// Like `set`: 149 words.
    Set,
    /// Like `spot`: 116 words.
    Spot,
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
    /// Like `write`: 49 words.
    Write,
    /// Like `spring`: 46 words.
    Spring,
    /// Like `lay`: 46 words.
    Lay,
    /// Like `throw`: 46 words.
    Throw,
    /// Like `break`: 46 words.
    Break,
    /// Like `cast`: 42 words.
    Cast,
    /// Like `take`: 41 words.
    Take,
    /// Like `show`: 40 words.
    Show,
    /// Like `wring`: 39 words.
    Wring,
    /// Like `drive`: 36 words.
    Drive,
    /// Like `wear`: 28 words.
    Wear,
    /// Like `sit`: 28 words.
    Sit,
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
    /// Like `backbite`: 21 words.
    Backbite,
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
    /// Like `forbear`: 16 words.
    Forbear,
    /// Like `overgo`: 16 words.
    Overgo,
    /// Like `should`: 16 words.
    Should,
    /// Like `see`: 15 words.
    See,
    /// Like `unbe`: 15 words.
    Unbe,
    /// Like `become`: 15 words.
    Become,
    /// Like `fall`: 14 words.
    Fall,
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
    /// Like `hang`: 9 words.
    Hang,
    /// Like `lie`: 9 words.
    Lie,
    /// Like `seek`: 9 words.
    Seek,
    /// Like `strike`: 9 words.
    Strike,
    /// Like `fix`: 8 words.
    Fix,
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
    /// Like `win`: 6 words.
    Win,
    /// Like `wake`: 6 words.
    Wake,
    /// Like `choose`: 6 words.
    Choose,
    /// Like `fly`: 6 words.
    Fly,
    /// Like `belay`: 6 words.
    Belay,
    /// Like `larn`: 6 words.
    Larn,
    /// Like `lose`: 5 words.
    Lose,
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
    pub const ALL: [Conjugation; 95] = [
        Conjugation::Have,
        Conjugation::Burn,
        Conjugation::Pass,
        Conjugation::Drop,
        Conjugation::Carry,
        Conjugation::Hear,
        Conjugation::Bag,
        Conjugation::Set,
        Conjugation::Spot,
        Conjugation::Bend,
        Conjugation::Die,
        Conjugation::Bleed,
        Conjugation::Bind,
        Conjugation::Keep,
        Conjugation::Write,
        Conjugation::Spring,
        Conjugation::Lay,
        Conjugation::Throw,
        Conjugation::Break,
        Conjugation::Cast,
        Conjugation::Take,
        Conjugation::Show,
        Conjugation::Wring,
        Conjugation::Drive,
        Conjugation::Wear,
        Conjugation::Sit,
        Conjugation::Hold,
        Conjugation::Stand,
        Conjugation::Draw,
        Conjugation::Tell,
        Conjugation::Queue,
        Conjugation::Backbite,
        Conjugation::Make,
        Conjugation::Give,
        Conjugation::Dive,
        Conjugation::Mislead,
        Conjugation::Run,
        Conjugation::Think,
        Conjugation::Shoot,
        Conjugation::Alight,
        Conjugation::Forbear,
        Conjugation::Overgo,
        Conjugation::Should,
        Conjugation::See,
        Conjugation::Unbe,
        Conjugation::Become,
        Conjugation::Fall,
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
        Conjugation::Hang,
        Conjugation::Lie,
        Conjugation::Seek,
        Conjugation::Strike,
        Conjugation::Fix,
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
        Conjugation::Win,
        Conjugation::Wake,
        Conjugation::Choose,
        Conjugation::Fly,
        Conjugation::Belay,
        Conjugation::Larn,
        Conjugation::Lose,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 95] = [
    // Have: 16,893 words (nonsense, minute, trade, adjective, substantive, patronage).
    conjugation!("have", 16893, "~e";
    verb: "~e", "~es", "~ed", "~ing", "~ed";
    ),
    // Burn: 13,627 words (portmanteau, word, book, pound, pond, crow). Like Have, except in 2 cells: verb inf, verb _3sg.
    conjugation!("burn", 13627, "~", like Have;
    verb: "~", "~s", _, _, _;
    ),
    // Pass: 2,042 words (march, abash, abolish, abroach, abscess, absciss). Like Have, except in 1 cell: verb inf.
    conjugation!("pass", 2042, "~", like Have;
    verb: "~", _, _, _, _;
    ),
    // Drop: 1,813 words (cat, swap, swop, ab, abet, abut).
    conjugation!("drop", 1813, "~";
    verb: "~", "~s", "~ped", "~ping", "~ped";
    ),
    // Carry: 1,477 words (dictionary, sky, vivify, nullify, treaty, policy).
    conjugation!("carry", 1477, "~y";
    verb: "~y", "~ies", "~ied", "~ying", "~ied";
    ),
    // Hear: 307 words (free, pie, deal, forecast, cast, thee). Like Burn, except in 2 cells: verb past, verb pp.
    conjugation!("hear", 307, "~", like Burn;
    verb: _, _, "~d", _, "~d";
    ),
    // Bag: 261 words (abhor, star, metric, music, war, defer).
    conjugation!("bag", 261, "~";
    verb: "~", "~s", "~ged", "~ging", "~ged";
    ),
    // Set: 149 words (offset, yet, bet, bid, cut, hit).
    conjugation!("set", 149, "~";
    verb: "~", "~s", "~", "~ting", "~";
    ),
    // Spot: 116 words (cod, red, squid, god, bad, bed). Like Set, except in 2 cells: verb past, verb pp.
    conjugation!("spot", 116, "~", like Set;
    verb: _, _, "~ted", _, "~ted";
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
    // Keep: 56 words (creep, feel, keep, kneel, oversleep, sleep). Like Bleed, except in 2 cells: verb past, verb pp.
    conjugation!("keep", 56, "~e~", like Bleed;
    verb: _, _, "~~t", _, "~~t";
    ),
    // Write: 49 words (write, override, rewrite, ride, stride, underwrite).
    conjugation!("write", 49, "~i~e";
    verb: "~i~e", "~i~es", "~o~e", "~i~ing", "~i~ten";
    ),
    // Spring: 46 words (spring, shrink, drink, sing, stink, sink). Like Bind, except in 2 cells: verb past, verb pp.
    conjugation!("spring", 46, "~i~", like Bind;
    verb: _, _, "~a~", _, "~u~";
    ),
    // Lay: 46 words (say, lay, mislay, overlay, overpay, pay).
    conjugation!("lay", 46, "~y";
    verb: "~y", "~ys", "~id", "~ying", "~id";
    ),
    // Throw: 46 words (know, upblow, blow, grow, outgrow, overthrow).
    conjugation!("throw", 46, "~o~";
    verb: "~o~", "~o~s", "~e~", "~o~ing", "~o~n";
    ),
    // Break: 46 words (upbreak, break, speak, housebreak, misspeak, outbreak).
    conjugation!("break", 46, "~ea~";
    verb: "~ea~", "~ea~s", "~o~e", "~ea~ing", "~o~en";
    ),
    // Cast: 42 words (read, misread, proofread, spread, worth, redd). Like Set, except in 1 cell: verb prp.
    conjugation!("cast", 42, "~", like Set;
    verb: _, _, _, "~ing", _;
    ),
    // Take: 41 words (take, forsake, mistake, overtake, partake, shake).
    conjugation!("take", 41, "~a~e";
    verb: "~a~e", "~a~es", "~oo~", "~a~ing", "~a~en";
    ),
    // Show: 40 words (shew, mow, sew, show, strew, sow). Like Burn, except in 1 cell: verb pp.
    conjugation!("show", 40, "~", like Burn;
    verb: _, _, _, _, "~n";
    ),
    // Wring: 39 words (wring, cling, fling, hamstring, sling, sting). Like Spring, except in 1 cell: verb past.
    conjugation!("wring", 39, "~i~", like Spring;
    verb: _, _, "~u~", _, _;
    ),
    // Drive: 36 words (arise, drive, rise, strive, uprise, shrive). Like Write, except in 1 cell: verb pp.
    conjugation!("drive", 36, "~i~e", like Write;
    verb: _, _, _, _, "~i~en";
    ),
    // Wear: 28 words (abear, tear, swear, wear, forswear, foreswear). Like Break, except in 1 cell: verb pp.
    conjugation!("wear", 28, "~ea~", like Break;
    verb: _, _, _, _, "~o~n";
    ),
    // Sit: 28 words (shit, sit, babysit, resit, housesit, birdsit). Like Spring, except in 2 cells: verb prp, verb pp.
    conjugation!("sit", 28, "~i~", like Spring;
    verb: _, _, _, "~i~ting", "~a~";
    ),
    // Hold: 25 words (behold, hold, uphold, withhold, stranglehold, handhold). Like Throw, except in 1 cell: verb pp.
    conjugation!("hold", 25, "~o~", like Throw;
    verb: _, _, _, _, "~e~";
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
    // Queue: 22 words (blue, true, horseshoe, coyote, meme, queue). Like Have, except in 1 cell: verb prp.
    conjugation!("queue", 22, "~e", like Have;
    verb: _, _, _, "~eing|~ing", _;
    ),
    // Backbite: 21 words (bite, hide, frostbite, backbite, backslide, overbite). Like Have, except in 2 cells: verb past, verb pp.
    conjugation!("backbite", 21, "~e", like Have;
    verb: _, _, "~", _, "~ten";
    ),
    // Make: 20 words (make, remake, matchmake, unmake, premake, merrymake).
    conjugation!("make", 20, "~ke";
    verb: "~ke", "~kes", "~de", "~king", "~de";
    ),
    // Give: 18 words (give, forgive, upgive, foregive, misgive, regive). Like Drive, except in 1 cell: verb past.
    conjugation!("give", 18, "~i~e", like Drive;
    verb: _, _, "~a~e", _, _;
    ),
    // Dive: 18 words (dive, updive, skydive, shite, rive, nosedive). Like Write, except in 2 cells: verb past, verb pp.
    conjugation!("dive", 18, "~i~e", like Write;
    verb: _, _, "~i~ed|~o~e", _, "~i~ed";
    ),
    // Mislead: 18 words (mislead, uplead, colead, cheerlead, underlead, aread). Like Draw, except in 2 cells: verb past, verb pp.
    conjugation!("mislead", 18, "~a~", like Draw;
    verb: _, _, "~~", _, "~~";
    ),
    // Run: 18 words (outrun, overrun, rerun, run, assrun, speedrun).
    conjugation!("run", 18, "~u~";
    verb: "~u~", "~u~s", "~a~", "~u~ning", "~u~";
    ),
    // Think: 17 words (think, rethink, doublethink, outthink, crimethink, misthink).
    conjugation!("think", 17, "~ink";
    verb: "~ink", "~inks", "~ought", "~inking", "~ought";
    ),
    // Shoot: 17 words (overshoot, shoot, troubleshoot, undershoot, reshoot, sharpshoot). Like Throw, except in 2 cells: verb past, verb pp.
    conjugation!("shoot", 17, "~o~", like Throw;
    verb: _, _, "~~", _, "~~";
    ),
    // Alight: 17 words (alight, relight, gaslight, spotlight, daylight, backlight).
    conjugation!("alight", 17, "~gh~";
    verb: "~gh~", "~gh~s", "~gh~ed|~~", "~gh~ing", "~gh~ed|~~";
    ),
    // Forbear: 16 words (upbear, forebear, forbear, overbear, rebear, underbear). Like Break, except in 1 cell: verb pp.
    conjugation!("forbear", 16, "~ea~", like Break;
    verb: _, _, _, _, "~o~ne";
    ),
    // Overgo: 16 words (forego, forgo, undergo, togo, outgo, misgo).
    conjugation!("overgo", 16, "~go";
    verb: "~go", "~goes", "~went", "~going", "~gone";
    ),
    // Should: 16 words (may, can, got, will, shall, should). Like Burn, except in 1 cell: verb _3sg.
    conjugation!("should", 16, "~", like Burn;
    verb: _, "~", _, _, _;
    ),
    // See: 15 words (see, foresee, oversee, forsee, sightsee, unsee).
    conjugation!("see", 15, "~ee";
    verb: "~ee", "~ees", "~aw", "~eeing", "~een";
    ),
    // Unbe: 15 words (be, beat, browbeat, wright, tyek, forbeat). Like Cast, except in 1 cell: verb pp.
    conjugation!("unbe", 15, "~", like Cast;
    verb: _, _, _, _, "~en";
    ),
    // Become: 15 words (become, come, overcome, misbecome, oncome, undercome).
    conjugation!("become", 15, "~o~e";
    verb: "~o~e", "~o~es", "~a~e", "~o~ing", "~o~e";
    ),
    // Fall: 14 words (fall, befall, eat, freefall, pratfall, downfall). Like Draw, except in 1 cell: verb pp.
    conjugation!("fall", 14, "~a~", like Draw;
    verb: _, _, _, _, "~a~en";
    ),
    // Spin: 14 words (spin, tailspin, backspin, topspin, overspin, wheelspin). Like Wring, except in 1 cell: verb prp.
    conjugation!("spin", 14, "~i~", like Wring;
    verb: _, _, _, "~i~ning", _;
    ),
    // Undo: 13 words (do, go, outdo, overdo, redo, undo).
    conjugation!("undo", 13, "~o";
    verb: "~o", "~oes", "~id", "~oing", "~one";
    ),
    // Slide: 13 words (slide, chide, landslide, rede, betide, slite). Like Backbite, except in 1 cell: verb pp.
    conjugation!("slide", 13, "~e", like Backbite;
    verb: _, _, _, _, "~";
    ),
    // Weave: 12 words (weave, interweave, reweave, forcleave, unweave, tocleave).
    conjugation!("weave", 12, "~~ave";
    verb: "~~ave", "~~aves", "~ov~", "~~aving", "~ov~n";
    ),
    // Abide: 11 words (abide, shine, bestride, outshine, overstride, beshine). Like Write, except in 1 cell: verb pp.
    conjugation!("abide", 11, "~i~e", like Write;
    verb: _, _, _, _, "~o~e";
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
    // Hang: 9 words (hang, overhang, uphang, cliffhang, behang, unhang). Like Draw, except in 2 cells: verb past, verb pp.
    conjugation!("hang", 9, "~a~", like Draw;
    verb: _, _, "~u~", _, "~u~";
    ),
    // Lie: 9 words (lie, overlie, underlie, belie, superlie, forelie).
    conjugation!("lie", 9, "~ie";
    verb: "~ie", "~ies", "~ay", "~ying", "~ain";
    ),
    // Seek: 9 words (seek, upseek, beseek, outseek, reseek, misseek).
    conjugation!("seek", 9, "~eek";
    verb: "~eek", "~eeks", "~ought", "~eeking", "~ought";
    ),
    // Strike: 9 words (strike, airstrike, restrike, counterstrike, overstrike, outstrike). Like Write, except in 2 cells: verb past, verb pp.
    conjugation!("strike", 9, "~i~e", like Write;
    verb: _, _, "~uc~", _, "~uc~";
    ),
    // Fix: 8 words (born, gar, fix, daydream, wis, lern). Like Burn, except in 2 cells: verb past, verb pp.
    conjugation!("fix", 8, "~", like Burn;
    verb: _, _, "~t", _, "~t";
    ),
    // Forget: 8 words (forget, beget, unbeget, misget, unforget, rebeget).
    conjugation!("forget", 8, "~e~";
    verb: "~e~", "~e~s", "~o~", "~e~ting", "~o~ten";
    ),
    // Leave: 8 words (leave, cleave, beleave, toreave, dreave, forleave).
    conjugation!("leave", 8, "~ave";
    verb: "~ave", "~aves", "~ft", "~aving", "~ft";
    ),
    // Overeat: 8 words (overeat, outeat, undereat, sleepeat, uneat, miseat). Like Bleed, except in 2 cells: verb past, verb pp.
    conjugation!("overeat", 8, "~e~", like Bleed;
    verb: _, _, "~~e", _, "~e~en";
    ),
    // Swim: 8 words (swim, nim, benim, outswim, overnim, overswim). Like Spring, except in 1 cell: verb prp.
    conjugation!("swim", 8, "~i~", like Spring;
    verb: _, _, _, "~i~ming", _;
    ),
    // Tread: 8 words (tread, totread, fortread, betread, untread, overtread). Like Break, except in 2 cells: verb past, verb pp.
    conjugation!("tread", 8, "~ea~", like Break;
    verb: _, _, "~o~", _, "~o~den";
    ),
    // Melt: 8 words (melt, speedwalk, upswell, outswell, dispeed, formelt). Like Bleed, except in 2 cells: verb past, verb pp.
    conjugation!("melt", 8, "~e~", like Bleed;
    verb: _, _, "~e~ed", _, "~e~ed";
    ),
    // Gale: 8 words (gale, handshake, bename, begrave, bedare, afare). Like Take, except in 2 cells: verb past, verb pp.
    conjugation!("gale", 8, "~a~e", like Take;
    verb: _, _, "~a~ed|~o~e", _, "~a~ed|~o~e|~a~en";
    ),
    // Gas: 7 words (gas, yes, teargas, outgas, degas, regas). Like Octopus, except in 1 cell: verb _3sg.
    conjugation!("gas", 7, "~", like Octopus;
    verb: _, "~es|~ses", _, _, _;
    ),
    // Dig: 7 words (dig, underdig, undig, redig, misdig, outdig). Like Wring, except in 1 cell: verb prp.
    conjugation!("dig", 7, "~i~", like Wring;
    verb: _, _, _, "~i~ging", _;
    ),
    // Light: 7 words (light, flashlight, twilight, floodlight, prelight, overlight). Like Alight, except in 2 cells: verb past, verb pp.
    conjugation!("light", 7, "~gh~", like Alight;
    verb: _, _, "~~|~gh~ed", _, "~~|~gh~ed";
    ),
    // Wind: 7 words (wind, upclimb, heatsink, sidewind, shitpost, pigstick). Like Bind, except in 2 cells: verb past, verb pp.
    conjugation!("wind", 7, "~i~", like Bind;
    verb: _, _, "~i~ed", _, "~i~ed";
    ),
    // Win: 6 words (win, overwin, rewin, outwin, bewin, electrowin). Like Spin, except in 2 cells: verb past, verb pp.
    conjugation!("win", 6, "~i~", like Spin;
    verb: _, _, "~o~", _, "~o~";
    ),
    // Wake: 6 words (awake, wake, reawake, forcarve, rewake, outwake). Like Take, except in 2 cells: verb past, verb pp.
    conjugation!("wake", 6, "~a~e", like Take;
    verb: _, _, "~o~e|~a~ed", _, "~o~en|~a~ed";
    ),
    // Choose: 6 words (choose, forechoose, rechoose, mischoose, unchoose, prechoose). Like Become, except in 2 cells: verb past, verb pp.
    conjugation!("choose", 6, "~o~e", like Become;
    verb: _, _, "~~e", _, "~~en";
    ),
    // Fly: 6 words (fly, overfly, outfly, refly, befly, upfly). Like Carry, except in 2 cells: verb past, verb pp.
    conjugation!("fly", 6, "~y", like Carry;
    verb: _, _, "~ew", _, "~own";
    ),
    // Belay: 6 words (why, belay, daresay, apay, beflay, appay). Like Lay, except in 2 cells: verb past, verb pp.
    conjugation!("belay", 6, "~y", like Lay;
    verb: _, _, "~yed|~id", _, "~yed|~id";
    ),
    // Larn: 6 words (de, dee, myek, deek, larn, stott). Like Burn, except in 1 cell: verb prp.
    conjugation!("larn", 6, "~", like Burn;
    verb: _, _, _, "~in", _;
    ),
    // Lose: 5 words (lose, kape, beclepe, unlose, relose). Like Have, except in 2 cells: verb past, verb pp.
    conjugation!("lose", 5, "~e", like Have;
    verb: _, _, "~t", _, "~t";
    ),
    // Flee: 5 words (flee, shoe, beflee, outflee, arede).
    conjugation!("flee", 5, "~e";
    verb: "~e", "~es", "~d", "~eing", "~d";
    ),
    // Get: 5 words (get, reget, unget, overget, underget). Like Forget, except in 1 cell: verb pp.
    conjugation!("get", 5, "~e~", like Forget;
    verb: _, _, _, _, "~o~|~o~ten";
    ),
    // Begin: 4 words (begin, rin, rebegin, atrin). Like Spring, except in 1 cell: verb prp.
    conjugation!("begin", 4, "~i~", like Spring;
    verb: _, _, _, "~i~ning", _;
    ),
    // Plead: 4 words (plead, rehang, implead, interplead). Like Draw, except in 2 cells: verb past, verb pp.
    conjugation!("plead", 4, "~a~", like Draw;
    verb: _, _, "~a~ed", _, "~a~ed";
    ),
    // Trolley: 4 words (monkey, bogey, curtsey, trolley). Like Buy, except in 2 cells: verb past, verb pp.
    conjugation!("trolley", 4, "~~y", like Buy;
    verb: _, _, "~~yed|~i~d", _, "~~yed|~i~d";
    ),
    // Birdie: 4 words (wheelie, stymie, birdie, assoilzie). Like Die, except in 1 cell: verb prp.
    conjugation!("birdie", 4, "~~e", like Die;
    verb: _, _, _, "~~eing|~y~ng", _;
    ),
    // Might: 4 words (might, must, gaf, wfh). Like Set, except in 2 cells: verb _3sg, verb prp.
    conjugation!("might", 4, "~", like Set;
    verb: _, "~", _, "~", _;
    ),
    // Slay: 3 words (slay, foreslay, reslay).
    conjugation!("slay", 3, "~ay";
    verb: "~ay", "~ays", "~ew", "~aying", "~ain";
    ),
    // Swell: 3 words (swell, unswell, reswell). Like Melt, except in 1 cell: verb pp.
    conjugation!("swell", 3, "~e~", like Melt;
    verb: _, _, _, _, "~o~en|~e~ed";
    ),
    // Dress: 3 words (dress, press, possess). Like Octopus, except in 2 cells: verb inf, verb pp.
    conjugation!("dress", 3, "~s", like Octopus;
    verb: "~s", _, _, _, "~sed|~t";
    ),
    // Braai: 3 words (braai, luigi, skibidi). Like Burn, except in 1 cell: verb prp.
    conjugation!("braai", 3, "~", like Burn;
    verb: _, _, _, "~ng", _;
    ),
    // Downstrike: 3 words (thunderstrike, downstrike, copystrike). Like Strike, except in 1 cell: verb pp.
    conjugation!("downstrike", 3, "~i~e", like Strike;
    verb: _, _, _, _, "~ic~en";
    ),
];
