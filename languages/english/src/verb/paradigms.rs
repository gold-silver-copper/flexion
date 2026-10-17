//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~e`, `settle`: the stem and `e`). A stem in two pieces (`~i~e`, `write`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A capital (`T`, `S`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~e- like settle\"`: the slot's cells are `settle`'s table's patterns
/// with `~e` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
/// `verb` (inf, _3sg, past, prp, pp)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem past: $s1:tt;)?
     $(stem prp: $s2:tt;)?
     $(verb: $c0:tt, $c1:tt, $c2:tt, $c3:tt, $c4:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// The cells a conjugation reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// What a conjugation adds after each pattern of the one it is like (`with "", "e" after "~"`: ``, or `e`
/// where that pattern ends in `~`).
macro_rules! affix {
    () => {
        None
    };
    ($affix:literal) => {
        affix!($affix, $affix, "")
    };
    ($affix:literal, $other:literal, $letters:literal) => {
        Some(flexion::Affix {
            pattern: ::std::borrow::Cow::Borrowed($affix),
            other: ::std::borrow::Cow::Borrowed($other),
            letters: ::std::borrow::Cow::Borrowed($letters),
        })
    };
}

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`inf_as_pp`: one value read as another).
const INF_AS_PP: [u16; 5] = [4, 1, 2, 3, 4];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `settle`: 16,918 words.
    Settle,
    /// Like `burn`: 13,672 words.
    Burn,
    /// Like `pass`: 2,045 words.
    Pass,
    /// Like `spot`: 1,522 words.
    Spot,
    /// Like `carry`: 1,477 words.
    Carry,
    /// Like `drop`: 446 words.
    Drop,
    /// Like `hear`: 398 words.
    Hear,
    /// Like `set`: 152 words.
    Set,
    /// Like `shred`: 114 words.
    Shred,
    /// Like `stir`: 100 words.
    Stir,
    /// Like `write`: 85 words.
    Write,
    /// Like `die`: 62 words.
    Die,
    /// Like `bleed`: 61 words.
    Bleed,
    /// Like `bind`: 60 words.
    Bind,
    /// Like `keep`: 56 words.
    Keep,
    /// Like `throw`: 46 words.
    Throw,
    /// Like `spring`: 46 words.
    Spring,
    /// Like `break`: 46 words.
    Break,
    /// Like `take`: 41 words.
    Take,
    /// Like `cast`: 40 words.
    Cast,
    /// Like `show`: 40 words.
    Show,
    /// Like `wring`: 39 words.
    Wring,
    /// Like `lay`: 37 words.
    Lay,
    /// Like `wear`: 28 words.
    Wear,
    /// Like `sit`: 28 words.
    Sit,
    /// Like `hold`: 25 words.
    Hold,
    /// Like `quiz`: 24 words.
    Quiz,
    /// Like `stand`: 23 words.
    Stand,
    /// Like `draw`: 23 words.
    Draw,
    /// Like `tell`: 23 words.
    Tell,
    /// Like `backbite`: 21 words.
    Backbite,
    /// Like `spin`: 21 words.
    Spin,
    /// Like `make`: 20 words.
    Make,
    /// Like `give`: 18 words.
    Give,
    /// Like `mislead`: 18 words.
    Mislead,
    /// Like `run`: 18 words.
    Run,
    /// Like `think`: 17 words.
    Think,
    /// Like `shoot`: 17 words.
    Shoot,
    /// Like `should`: 17 words.
    Should,
    /// Like `forbear`: 16 words.
    Forbear,
    /// Like `overgo`: 16 words.
    Overgo,
    /// Like `see`: 15 words.
    See,
    /// Like `unbe`: 15 words.
    Unbe,
    /// Like `become`: 15 words.
    Become,
    /// Like `fall`: 14 words.
    Fall,
    /// Like `undo`: 13 words.
    Undo,
    /// Like `slide`: 13 words.
    Slide,
    /// Like `weave`: 12 words.
    Weave,
    /// Like `swim`: 12 words.
    Swim,
    /// Like `abide`: 11 words.
    Abide,
    /// Like `buy`: 11 words.
    Buy,
    /// Like `bring`: 11 words.
    Bring,
    /// Like `rec`: 11 words.
    Rec,
    /// Like `beteach`: 11 words.
    Beteach,
    /// Like `outleap`: 10 words.
    Outleap,
    /// Like `catch`: 10 words.
    Catch,
    /// Like `freeze`: 10 words.
    Freeze,
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
    /// Like `tread`: 8 words.
    Tread,
    /// Like `distil`: 7 words.
    Distil,
    /// Like `gas`: 7 words.
    Gas,
    /// Like `light`: 7 words.
    Light,
    /// Like `panic`: 6 words.
    Panic,
    /// Like `win`: 6 words.
    Win,
    /// Like `wake`: 6 words.
    Wake,
    /// Like `choose`: 6 words.
    Choose,
    /// Like `fly`: 6 words.
    Fly,
    /// Like `larn`: 6 words.
    Larn,
    /// Like `lose`: 5 words.
    Lose,
    /// Like `flee`: 5 words.
    Flee,
    /// Like `get`: 5 words.
    Get,
    /// Like `might`: 4 words.
    Might,
    /// Like `chef`: 3 words.
    Chef,
    /// Like `slay`: 3 words.
    Slay,
    /// Like `swell`: 3 words.
    Swell,
    /// Like `braai`: 3 words.
    Braai,
    /// Like `downstrike`: 3 words.
    Downstrike,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 83] = [
        Conjugation::Settle,
        Conjugation::Burn,
        Conjugation::Pass,
        Conjugation::Spot,
        Conjugation::Carry,
        Conjugation::Drop,
        Conjugation::Hear,
        Conjugation::Set,
        Conjugation::Shred,
        Conjugation::Stir,
        Conjugation::Write,
        Conjugation::Die,
        Conjugation::Bleed,
        Conjugation::Bind,
        Conjugation::Keep,
        Conjugation::Throw,
        Conjugation::Spring,
        Conjugation::Break,
        Conjugation::Take,
        Conjugation::Cast,
        Conjugation::Show,
        Conjugation::Wring,
        Conjugation::Lay,
        Conjugation::Wear,
        Conjugation::Sit,
        Conjugation::Hold,
        Conjugation::Quiz,
        Conjugation::Stand,
        Conjugation::Draw,
        Conjugation::Tell,
        Conjugation::Backbite,
        Conjugation::Spin,
        Conjugation::Make,
        Conjugation::Give,
        Conjugation::Mislead,
        Conjugation::Run,
        Conjugation::Think,
        Conjugation::Shoot,
        Conjugation::Should,
        Conjugation::Forbear,
        Conjugation::Overgo,
        Conjugation::See,
        Conjugation::Unbe,
        Conjugation::Become,
        Conjugation::Fall,
        Conjugation::Undo,
        Conjugation::Slide,
        Conjugation::Weave,
        Conjugation::Swim,
        Conjugation::Abide,
        Conjugation::Buy,
        Conjugation::Bring,
        Conjugation::Rec,
        Conjugation::Beteach,
        Conjugation::Outleap,
        Conjugation::Catch,
        Conjugation::Freeze,
        Conjugation::Hang,
        Conjugation::Lie,
        Conjugation::Seek,
        Conjugation::Strike,
        Conjugation::Forget,
        Conjugation::Leave,
        Conjugation::Overeat,
        Conjugation::Tread,
        Conjugation::Distil,
        Conjugation::Gas,
        Conjugation::Light,
        Conjugation::Panic,
        Conjugation::Win,
        Conjugation::Wake,
        Conjugation::Choose,
        Conjugation::Fly,
        Conjugation::Larn,
        Conjugation::Lose,
        Conjugation::Flee,
        Conjugation::Get,
        Conjugation::Might,
        Conjugation::Chef,
        Conjugation::Slay,
        Conjugation::Swell,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 83] = [
    // Settle: 16,918 words (nonsense, minute, trade, adjective, substantive, patronage).
    conjugation!("settle", 16918, "~e";
    stem lemma: "~e- like settle";
    verb: _, _, "~ed", "~ing", _;
    ),
    // Burn: 13,672 words (portmanteau, word, book, pound, pond, crow). Like Settle, except in 2 cells: verb inf, verb _3sg.
    conjugation!("burn", 13672, "~", like Settle;
    verb: "~", "~S", _, _, _;
    ),
    // Pass: 2,045 words (march, abash, abolish, abroach, abscess, absciss). Like Settle, except in 1 cell: verb inf.
    conjugation!("pass", 2045, "~", like Settle;
    verb: "~", _, _, _, _;
    ),
    // Spot: 1,522 words (cat, ab, abet, abut, het, van). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("spot", 1522, "~", like Burn;
    verb: _, _, "~Ted", "~Ting", _;
    ),
    // Carry: 1,477 words (dictionary, sky, vivify, nullify, treaty, policy).
    conjugation!("carry", 1477, "~y";
    verb: "~y", "~ies", "~ied", "~ying", _;
    ),
    // Drop: 446 words (swap, swop, crap, pip, dub, bap). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("drop", 446, "~", like Burn;
    verb: _, _, "~ped", "~ping", _;
    ),
    // Hear: 398 words (free, pie, deal, send, forecast, blue). Like Burn, except in 1 cell: verb past.
    conjugation!("hear", 398, "~", like Burn;
    verb: _, _, "~d", _, _;
    ),
    // Set: 152 words (offset, yet, bet, bid, cut, hit). Like Spot, except in 1 cell: verb past.
    conjugation!("set", 152, "~", like Spot;
    verb: _, _, "~", _, _;
    ),
    // Shred: 114 words (cod, red, squid, god, bad, bed). Like Spot, except in 1 cell: verb pp.
    conjugation!("shred", 114, "~", like Spot;
    verb: _, _, _, _, "~TeT";
    ),
    // Stir: 100 words (abhor, star, war, defer, upbar, er). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("stir", 100, "~", like Burn;
    verb: _, _, "~red", "~ring", _;
    ),
    // Write: 85 words (write, arise, drive, override, rewrite, ride).
    conjugation!("write", 85, "~i~e";
    stem lemma: "~i~e- like settle";
    verb: _, _, "~o~e", "~i~ing", "~i~Ten";
    ),
    // Die: 62 words (die, noogie, tie, sie, auntie, vie).
    conjugation!("die", 62, "~~e";
    stem lemma: "~~e- like settle";
    verb: _, _, "~~ed", "~y~ng", _;
    ),
    // Bleed: 61 words (upbreed, bleed, breed, feed, interbreed, meet).
    conjugation!("bleed", 61, "~e~";
    stem lemma: "~e~- like settle";
    verb: _, _, "~~", "~e~ing", _;
    ),
    // Bind: 60 words (upbind, bind, fight, find, grind, rebind).
    conjugation!("bind", 60, "~i~";
    stem lemma: "~i~- like settle";
    verb: _, _, "~ou~", "~i~ing", _;
    ),
    // Keep: 56 words (creep, feel, keep, kneel, oversleep, sleep). Like Bleed, except in 1 cell: verb past.
    conjugation!("keep", 56, "~e~", like Bleed;
    verb: _, _, "~~t", _, _;
    ),
    // Throw: 46 words (know, upblow, blow, grow, outgrow, overthrow).
    conjugation!("throw", 46, "~o~";
    stem lemma: "~o~- like settle";
    verb: _, _, "~e~", "~o~ing", "~o~n";
    ),
    // Spring: 46 words (spring, shrink, drink, sing, stink, sink). Like Bind, except in 2 cells: verb past, verb pp.
    conjugation!("spring", 46, "~i~", like Bind;
    verb: _, _, "~a~", _, "~u~";
    ),
    // Break: 46 words (upbreak, break, speak, housebreak, misspeak, outbreak).
    conjugation!("break", 46, "~ea~";
    stem lemma: "~ea~- like settle";
    stem past: "~o~e- like break";
    verb: _, _, _, "~ea~ing", _;
    ),
    // Take: 41 words (take, forsake, mistake, overtake, partake, shake).
    conjugation!("take", 41, "~a~e";
    stem lemma: "~a~e- like settle";
    verb: _, _, "~oo~", "~a~ing", "~a~en";
    ),
    // Cast: 40 words (read, misread, proofread, spread, worth, redd). Like Burn, except in 1 cell: verb past.
    conjugation!("cast", 40, "~", like Burn;
    verb: _, _, "~", _, _;
    ),
    // Show: 40 words (shew, mow, sew, show, strew, sow). Like Burn, except in 1 cell: verb pp.
    conjugation!("show", 40, "~", like Burn;
    verb: _, _, _, _, "~n";
    ),
    // Wring: 39 words (wring, cling, fling, hamstring, sling, sting). Like Bind, except in 1 cell: verb past.
    conjugation!("wring", 39, "~i~", like Bind;
    verb: _, _, "~u~", _, _;
    ),
    // Lay: 37 words (say, lay, mislay, overlay, overpay, pay). Like Carry, except in 1 cell: verb past.
    conjugation!("lay", 37, "~y", like Carry;
    stem lemma: "~y- like settle";
    verb: _, _, "~id", _, _;
    ),
    // Wear: 28 words (abear, tear, swear, wear, forswear, foreswear). Like Break.
    conjugation!("wear", 28, "~ea~", like Break;
    stem past: "~o~- like wear";
    ),
    // Sit: 28 words (shit, sit, babysit, resit, housesit, birdsit). Like Bind, except in 2 cells: verb past, verb prp.
    conjugation!("sit", 28, "~i~", like Bind;
    verb: _, _, "~a~", "~i~Ting", _;
    ),
    // Hold: 25 words (behold, hold, uphold, withhold, stranglehold, handhold). Like Throw, except in 1 cell: verb pp.
    conjugation!("hold", 25, "~o~", like Throw;
    verb: _, _, _, _, derived;
    ),
    // Quiz: 24 words (quiz, octopus, nonplus, dis, pus, res).
    conjugation!("quiz", 24, "~";
    verb: "~", "~Ses", "~Sed", "~Sing", _;
    ),
    // Stand: 23 words (stand, misunderstand, understand, withstand, handstand, headstand).
    conjugation!("stand", 23, "~an~";
    stem lemma: "~an~- like settle";
    verb: _, _, "~oo~", "~an~ing", _;
    ),
    // Draw: 23 words (draw, overdraw, withdraw, updraw, redraw, outdraw).
    conjugation!("draw", 23, "~a~";
    stem lemma: "~a~- like settle";
    verb: _, _, "~e~", "~a~ing", "~a~n";
    ),
    // Tell: 23 words (foretell, outsell, oversell, resell, sell, tell).
    conjugation!("tell", 23, "~e~l";
    stem lemma: "~e~l- like settle";
    verb: _, _, "~o~d", "~e~ling", _;
    ),
    // Backbite: 21 words (bite, hide, frostbite, backbite, backslide, overbite). Like Settle, except in 2 cells: verb past, verb pp.
    conjugation!("backbite", 21, "~e", like Settle;
    verb: _, _, "~", _, "~Ten";
    ),
    // Spin: 21 words (dig, spin, tailspin, backspin, topspin, overspin). Like Wring, except in 1 cell: verb prp.
    conjugation!("spin", 21, "~i~", like Wring;
    verb: _, _, _, "~i~Ting", _;
    ),
    // Make: 20 words (make, remake, matchmake, unmake, premake, merrymake).
    conjugation!("make", 20, "~ke";
    stem lemma: "~ke- like settle";
    verb: _, _, "~de", "~king", _;
    ),
    // Give: 18 words (give, forgive, upgive, foregive, misgive, regive). Like Write, except in 2 cells: verb past, verb pp.
    conjugation!("give", 18, "~i~e", like Write;
    verb: _, _, "~a~e", _, "~i~en";
    ),
    // Mislead: 18 words (mislead, uplead, colead, cheerlead, underlead, aread). Like Draw, except in 2 cells: verb past, verb pp.
    conjugation!("mislead", 18, "~a~", like Draw;
    verb: _, _, "~~", _, derived;
    ),
    // Run: 18 words (outrun, overrun, rerun, run, assrun, speedrun). Like Spring, except in 1 cell: verb prp.
    conjugation!("run", 18, "~u~", like Spring as INF_AS_PP;
    stem lemma: "~u~- like settle";
    verb: _, _, _, "~u~Ting", _;
    ),
    // Think: 17 words (think, rethink, doublethink, outthink, crimethink, misthink).
    conjugation!("think", 17, "~ink";
    stem lemma: "~ink- like settle";
    verb: _, _, "~ought", "~inking", _;
    ),
    // Shoot: 17 words (overshoot, shoot, troubleshoot, undershoot, reshoot, sharpshoot). Like Hold, except in 1 cell: verb past.
    conjugation!("shoot", 17, "~o~", like Hold;
    verb: _, _, "~~", _, _;
    ),
    // Should: 17 words (may, can, got, will, shall, should). Like Burn, except in 1 cell: verb _3sg.
    conjugation!("should", 17, "~", like Burn;
    verb: _, "~", _, _, _;
    ),
    // Forbear: 16 words (upbear, forebear, forbear, overbear, rebear, underbear). Like Break, except in 1 cell: verb pp.
    conjugation!("forbear", 16, "~ea~", like Break;
    verb: _, _, _, _, "~o~ne";
    ),
    // Overgo: 16 words (forego, forgo, undergo, togo, outgo, misgo).
    conjugation!("overgo", 16, "~go";
    stem lemma: "~go- like pass";
    stem past: "~- like overgo";
    verb: _, _, _, "~going", _;
    ),
    // See: 15 words (see, foresee, oversee, forsee, sightsee, unsee).
    conjugation!("see", 15, "~ee";
    stem lemma: "~ee- like settle";
    verb: _, _, "~aw", "~eeing", "~een";
    ),
    // Unbe: 15 words (be, beat, browbeat, wright, tyek, forbeat). Like Cast, except in 1 cell: verb pp.
    conjugation!("unbe", 15, "~", like Cast;
    verb: _, _, _, _, "~en";
    ),
    // Become: 15 words (become, come, overcome, misbecome, oncome, undercome).
    conjugation!("become", 15, "~o~e";
    stem lemma: "~o~e- like settle";
    verb: _, _, "~a~e", "~o~ing", "~o~e";
    ),
    // Fall: 14 words (fall, befall, eat, freefall, pratfall, downfall). Like Draw, except in 1 cell: verb pp.
    conjugation!("fall", 14, "~a~", like Draw;
    verb: _, _, _, _, "~a~en";
    ),
    // Undo: 13 words (do, go, outdo, overdo, redo, undo).
    conjugation!("undo", 13, "~o";
    stem lemma: "~o- like pass";
    verb: _, _, "~iT", "~oing", "~one";
    ),
    // Slide: 13 words (slide, chide, landslide, rede, betide, slite). Like Pass with `` after each pattern (`e` after ~), except in 1 cell: verb past.
    conjugation!("slide", 13, "~e", like Pass with "", "e" after "~";
    verb: _, _, "~", _, _;
    ),
    // Weave: 12 words (weave, interweave, reweave, forcleave, unweave, tocleave).
    conjugation!("weave", 12, "~~ave";
    stem lemma: "~~ave- like settle";
    stem past: "~ov~- like break";
    verb: _, _, _, "~~aving", _;
    ),
    // Swim: 12 words (begin, swim, rin, nim, benim, rebegin). Like Spring, except in 1 cell: verb prp.
    conjugation!("swim", 12, "~i~", like Spring;
    verb: _, _, _, "~i~Ting", _;
    ),
    // Abide: 11 words (abide, shine, bestride, outshine, overstride, beshine). Like Write, except in 1 cell: verb pp.
    conjugation!("abide", 11, "~i~e", like Write;
    verb: _, _, _, _, derived;
    ),
    // Buy: 11 words (buy, rebuy, overbuy, underbuy, prebuy, forbuy).
    conjugation!("buy", 11, "~~y";
    stem lemma: "~~y- like settle";
    verb: _, _, "~o~ght", "~~ying", _;
    ),
    // Bring: 11 words (bring, upbring, underbring, forthbring, inbring, imbring).
    conjugation!("bring", 11, "~in~";
    stem lemma: "~in~- like settle";
    verb: _, _, "~ou~ht", "~in~ing", _;
    ),
    // Rec: 11 words (sic, floc, tic, vac, proc, fanfic). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("rec", 11, "~", like Burn;
    verb: _, _, "~ced", "~cing", _;
    ),
    // Beteach: 11 words (teach, reteach, overteach, misteach, beteach, unteach).
    conjugation!("beteach", 11, "~e~c~";
    stem lemma: "~e~c~- like pass";
    verb: _, _, "~~ug~t", "~e~c~ing", _;
    ),
    // Outleap: 10 words (gar, fix, daydream, wis, lowp, outleap). Like Burn, except in 1 cell: verb past.
    conjugation!("outleap", 10, "~", like Burn;
    verb: _, _, "~t", _, _;
    ),
    // Catch: 10 words (catch, bycatch, recatch, overcatch, becatch, outcatch).
    conjugation!("catch", 10, "~tc~";
    stem lemma: "~tc~- like pass";
    verb: _, _, "~ug~t", "~tc~ing", _;
    ),
    // Freeze: 10 words (freeze, unfreeze, refreeze, enfreeze, befreeze, prefreeze).
    conjugation!("freeze", 10, "~~eze";
    stem lemma: "~~eze- like settle";
    stem past: "~oz~- like break";
    verb: _, _, _, "~~ezing", _;
    ),
    // Hang: 9 words (hang, overhang, uphang, cliffhang, behang, unhang). Like Mislead, except in 1 cell: verb past.
    conjugation!("hang", 9, "~a~", like Mislead;
    verb: _, _, "~u~", _, _;
    ),
    // Lie: 9 words (lie, overlie, underlie, belie, superlie, forelie).
    conjugation!("lie", 9, "~ie";
    stem lemma: "~ie- like settle";
    stem past: "~a- like lie";
    verb: _, _, _, "~ying", _;
    ),
    // Seek: 9 words (seek, upseek, beseek, outseek, reseek, misseek).
    conjugation!("seek", 9, "~eek";
    stem lemma: "~eek- like settle";
    verb: _, _, "~ought", "~eeking", _;
    ),
    // Strike: 9 words (strike, airstrike, restrike, counterstrike, overstrike, outstrike). Like Abide, except in 1 cell: verb past.
    conjugation!("strike", 9, "~i~e", like Abide;
    verb: _, _, "~uc~", _, _;
    ),
    // Forget: 8 words (forget, beget, unbeget, misget, unforget, rebeget).
    conjugation!("forget", 8, "~e~";
    stem lemma: "~e~- like settle";
    verb: _, _, "~o~", "~e~Ting", "~o~ten";
    ),
    // Leave: 8 words (leave, cleave, beleave, toreave, dreave, forleave).
    conjugation!("leave", 8, "~ave";
    stem lemma: "~ave- like settle";
    verb: _, _, "~ft", "~aving", _;
    ),
    // Overeat: 8 words (overeat, outeat, undereat, sleepeat, uneat, miseat). Like Bleed, except in 2 cells: verb past, verb pp.
    conjugation!("overeat", 8, "~e~", like Bleed;
    verb: _, _, "~~e", _, "~e~en";
    ),
    // Tread: 8 words (tread, totread, fortread, betread, untread, overtread). Like Break, except in 2 cells: verb past, verb pp.
    conjugation!("tread", 8, "~ea~", like Break;
    verb: _, _, "~o~", _, "~o~den";
    ),
    // Distil: 7 words (expel, extol, excel, partial, reexpel, superexcel).
    conjugation!("distil", 7, "~";
    stem lemma: "~- like settle";
    verb: _, _, "~led", "~ling", _;
    ),
    // Gas: 7 words (gas, yes, teargas, outgas, degas, regas). Like Quiz, except in 1 cell: verb _3sg.
    conjugation!("gas", 7, "~", like Quiz;
    verb: _, "~es|~ses", _, _, _;
    ),
    // Light: 7 words (light, flashlight, twilight, floodlight, prelight, overlight).
    conjugation!("light", 7, "~gh~";
    stem lemma: "~gh~- like settle";
    verb: _, _, "~~|~gh~ed", "~gh~ing", _;
    ),
    // Panic: 6 words (hydraulic, bivouac, dak, wuk, dekek, hydrofrac). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("panic", 6, "~", like Burn;
    verb: _, _, "~ked", "~king", _;
    ),
    // Win: 6 words (win, overwin, rewin, outwin, bewin, electrowin). Like Sit, except in 1 cell: verb past.
    conjugation!("win", 6, "~i~", like Sit;
    verb: _, _, "~o~", _, _;
    ),
    // Wake: 6 words (awake, wake, reawake, forcarve, rewake, outwake). Like Take, except in 2 cells: verb past, verb pp.
    conjugation!("wake", 6, "~a~e", like Take;
    verb: _, _, "~o~e|~a~ed", _, "~o~en|~a~ed";
    ),
    // Choose: 6 words (choose, forechoose, rechoose, mischoose, unchoose, prechoose). Like Become.
    conjugation!("choose", 6, "~o~e", like Become;
    stem past: "~~e- like break";
    ),
    // Fly: 6 words (fly, overfly, outfly, refly, befly, upfly). Like Carry, except in 2 cells: verb past, verb pp.
    conjugation!("fly", 6, "~y", like Carry;
    verb: _, _, "~ew", _, "~own";
    ),
    // Larn: 6 words (de, dee, myek, deek, larn, stott). Like Burn, except in 1 cell: verb prp.
    conjugation!("larn", 6, "~", like Burn;
    verb: _, _, _, "~in", _;
    ),
    // Lose: 5 words (lose, kape, beclepe, unlose, relose). Like Pass with `` after each pattern (`e` after ~), except in 1 cell: verb past.
    conjugation!("lose", 5, "~e", like Pass with "", "e" after "~";
    verb: _, _, "~t", _, _;
    ),
    // Flee: 5 words (flee, shoe, beflee, outflee, arede). Like Settle, except in 2 cells: verb past, verb prp.
    conjugation!("flee", 5, "~e", like Settle;
    verb: _, _, "~d", "~eing", _;
    ),
    // Get: 5 words (get, reget, unget, overget, underget). Like Forget, except in 1 cell: verb pp.
    conjugation!("get", 5, "~e~", like Forget;
    verb: _, _, _, _, "~o~|~o~ten";
    ),
    // Might: 4 words (might, must, gaf, wfh). Like Set, except in 2 cells: verb _3sg, verb prp.
    conjugation!("might", 4, "~", like Set;
    verb: _, "~", _, "~", _;
    ),
    // Chef: 3 words (indef, uncoif, smof). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("chef", 3, "~", like Burn;
    verb: _, _, "~fed", "~fing", _;
    ),
    // Slay: 3 words (slay, foreslay, reslay).
    conjugation!("slay", 3, "~ay";
    stem lemma: "~ay- like settle";
    verb: _, _, "~ew", "~aying", "~ain";
    ),
    // Swell: 3 words (swell, unswell, reswell). Like Bleed, except in 2 cells: verb past, verb pp.
    conjugation!("swell", 3, "~e~", like Bleed;
    verb: _, _, "~e~ed", _, "~o~en|~e~ed";
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
