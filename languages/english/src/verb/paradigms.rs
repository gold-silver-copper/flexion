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
/// they share them), the conjugation it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the conjugation's it is like, or, in a conjugation like
/// no other, derived):
/// `verb` (inf, _3sg, past, prp, pp)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem verb: $s0:tt;)?
     $(stem verb_past: $s1:tt;)?
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

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`past_as_inf`: one value read as another).
const PAST_AS_INF: [u16; 5] = [0, 1, 0, 3, 4];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`inf_as_pp`: one value read as another).
const INF_AS_PP: [u16; 5] = [4, 1, 2, 3, 4];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`3sg_as_inf`: one value read as another).
const C3SG_AS_INF: [u16; 5] = [0, 0, 2, 3, 4];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`prp_as_past`: one value read as another).
const PRP_AS_PAST: [u16; 5] = [0, 1, 2, 2, 4];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `settle`: 16,919 words.
    Settle,
    /// Like `burn`: 13,673 words.
    Burn,
    /// Like `pass`: 2,045 words.
    Pass,
    /// Like `drop`: 1,909 words.
    Drop,
    /// Like `carry`: 1,477 words.
    Carry,
    /// Like `hear`: 333 words.
    Hear,
    /// Like `set`: 153 words.
    Set,
    /// Like `stir`: 130 words.
    Stir,
    /// Like `spot`: 114 words.
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
    /// Like `lay`: 46 words.
    Lay,
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
    /// Like `drive`: 36 words.
    Drive,
    /// Like `bag`: 35 words.
    Bag,
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
    /// Like `backbite`: 21 words.
    Backbite,
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
    /// Like `rec`: 11 words.
    Rec,
    /// Like `beteach`: 11 words.
    Beteach,
    /// Like `octopus`: 11 words.
    Octopus,
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
    /// Like `gas`: 7 words.
    Gas,
    /// Like `dig`: 7 words.
    Dig,
    /// Like `light`: 7 words.
    Light,
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
    /// Like `lose`: 6 words.
    Lose,
    /// Like `get`: 5 words.
    Get,
    /// Like `flee`: 4 words.
    Flee,
    /// Like `begin`: 4 words.
    Begin,
    /// Like `might`: 4 words.
    Might,
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
    pub const ALL: [Conjugation; 85] = [
        Conjugation::Settle,
        Conjugation::Burn,
        Conjugation::Pass,
        Conjugation::Drop,
        Conjugation::Carry,
        Conjugation::Hear,
        Conjugation::Set,
        Conjugation::Stir,
        Conjugation::Spot,
        Conjugation::Bend,
        Conjugation::Die,
        Conjugation::Bleed,
        Conjugation::Bind,
        Conjugation::Keep,
        Conjugation::Write,
        Conjugation::Lay,
        Conjugation::Throw,
        Conjugation::Spring,
        Conjugation::Break,
        Conjugation::Take,
        Conjugation::Cast,
        Conjugation::Show,
        Conjugation::Wring,
        Conjugation::Drive,
        Conjugation::Bag,
        Conjugation::Wear,
        Conjugation::Sit,
        Conjugation::Hold,
        Conjugation::Stand,
        Conjugation::Draw,
        Conjugation::Tell,
        Conjugation::Backbite,
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
        Conjugation::Spin,
        Conjugation::Undo,
        Conjugation::Slide,
        Conjugation::Weave,
        Conjugation::Abide,
        Conjugation::Quiz,
        Conjugation::Buy,
        Conjugation::Bring,
        Conjugation::Rec,
        Conjugation::Beteach,
        Conjugation::Octopus,
        Conjugation::Catch,
        Conjugation::Freeze,
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
        Conjugation::Gas,
        Conjugation::Dig,
        Conjugation::Light,
        Conjugation::Win,
        Conjugation::Wake,
        Conjugation::Choose,
        Conjugation::Fly,
        Conjugation::Larn,
        Conjugation::Lose,
        Conjugation::Get,
        Conjugation::Flee,
        Conjugation::Begin,
        Conjugation::Might,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 85] = [
    // Settle: 16,919 words (nonsense, minute, trade, adjective, substantive, patronage).
    conjugation!("settle", 16919, "~e";
    stem verb: "~- like settle";
    verb: _, _, "~ed", _, _;
    ),
    // Burn: 13,673 words (portmanteau, word, book, pound, pond, crow). Like Settle, except in 0 cells: .
    conjugation!("burn", 13673, "~", like Settle;
    stem verb: "~- like burn";
    ),
    // Pass: 2,045 words (march, abash, abolish, abroach, abscess, absciss). Like Settle, except in 1 cell: verb inf.
    conjugation!("pass", 2045, "~", like Settle;
    verb: "~", _, _, _, _;
    ),
    // Drop: 1,909 words (cat, swap, swop, ab, abet, abut). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("drop", 1909, "~", like Burn;
    verb: _, _, "~ped", "~ping", _;
    ),
    // Carry: 1,477 words (dictionary, sky, vivify, nullify, treaty, policy).
    conjugation!("carry", 1477, "~y";
    verb: "~y", "~ies", "~ied", "~ying", _;
    ),
    // Hear: 333 words (free, pie, deal, forecast, blue, cast). Like Burn, except in 1 cell: verb past.
    conjugation!("hear", 333, "~", like Burn;
    verb: _, _, "~d", _, _;
    ),
    // Set: 153 words (offset, yet, bet, bid, cut, forbid). Like Burn, except in 0 cells: .
    conjugation!("set", 153, "~", like Burn as PAST_AS_INF;
    stem verb: "~- like set";
    ),
    // Stir: 130 words (abhor, star, war, defer, upbar, er). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("stir", 130, "~", like Burn;
    verb: _, _, "~red", "~ring", _;
    ),
    // Spot: 114 words (cod, red, squid, god, bad, bed). Like Set, except in 1 cell: verb past.
    conjugation!("spot", 114, "~", like Set;
    verb: _, _, "~ted", _, _;
    ),
    // Bend: 68 words (send, bend, build, lend, rebuild, spend).
    conjugation!("bend", 68, "~d";
    stem verb: "~d- like burn";
    verb: _, _, "~t", _, _;
    ),
    // Die: 62 words (die, noogie, tie, sie, auntie, vie).
    conjugation!("die", 62, "~~e";
    verb: "~~e", "~~es", "~~ed", "~y~ng", _;
    ),
    // Bleed: 61 words (upbreed, bleed, breed, feed, interbreed, meet).
    conjugation!("bleed", 61, "~e~";
    stem verb: "~e~- like burn";
    verb: _, _, "~~", _, _;
    ),
    // Bind: 60 words (upbind, bind, fight, find, grind, rebind).
    conjugation!("bind", 60, "~i~";
    stem verb: "~i~- like burn";
    verb: _, _, "~ou~", _, _;
    ),
    // Keep: 56 words (creep, feel, keep, kneel, oversleep, sleep). Like Bleed, except in 1 cell: verb past.
    conjugation!("keep", 56, "~e~", like Bleed;
    verb: _, _, "~~t", _, _;
    ),
    // Write: 49 words (write, override, rewrite, ride, stride, underwrite).
    conjugation!("write", 49, "~i~e";
    stem verb: "~i~- like settle";
    verb: _, _, "~o~e", _, "~i~ten";
    ),
    // Lay: 46 words (say, lay, mislay, overlay, overpay, pay). Like Carry, except in 1 cell: verb past.
    conjugation!("lay", 46, "~y", like Carry;
    stem verb: "~y- like burn";
    verb: _, _, "~id", _, _;
    ),
    // Throw: 46 words (know, upblow, blow, grow, outgrow, overthrow).
    conjugation!("throw", 46, "~o~";
    stem verb: "~o~- like burn";
    verb: _, _, "~e~", _, "~o~n";
    ),
    // Spring: 46 words (spring, shrink, drink, sing, stink, sink). Like Bind, except in 2 cells: verb past, verb pp.
    conjugation!("spring", 46, "~i~", like Bind;
    verb: _, _, "~a~", _, "~u~";
    ),
    // Break: 46 words (upbreak, break, speak, housebreak, misspeak, outbreak).
    conjugation!("break", 46, "~ea~";
    stem verb: "~ea~- like burn";
    stem verb_past: "~o~e- like break";
    ),
    // Take: 41 words (take, forsake, mistake, overtake, partake, shake).
    conjugation!("take", 41, "~a~e";
    stem verb: "~a~- like settle";
    verb: _, _, "~oo~", _, "~a~en";
    ),
    // Cast: 40 words (read, misread, proofread, spread, worth, redd). Like Burn, except in 0 cells: .
    conjugation!("cast", 40, "~", like Burn as PAST_AS_INF;
    ),
    // Show: 40 words (shew, mow, sew, show, strew, sow). Like Burn, except in 1 cell: verb pp.
    conjugation!("show", 40, "~", like Burn;
    verb: _, _, _, _, "~n";
    ),
    // Wring: 39 words (wring, cling, fling, hamstring, sling, sting). Like Bind, except in 1 cell: verb past.
    conjugation!("wring", 39, "~i~", like Bind;
    verb: _, _, "~u~", _, _;
    ),
    // Drive: 36 words (arise, drive, rise, strive, uprise, shrive). Like Write, except in 1 cell: verb pp.
    conjugation!("drive", 36, "~i~e", like Write;
    verb: _, _, _, _, "~i~en";
    ),
    // Bag: 35 words (metric, music, hydraulic, traffic, logic, magic). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("bag", 35, "~", like Burn;
    verb: _, _, "~ged", "~ging", _;
    ),
    // Wear: 28 words (abear, tear, swear, wear, forswear, foreswear). Like Break, except in 0 cells: .
    conjugation!("wear", 28, "~ea~", like Break;
    stem verb_past: "~o~- like wear";
    ),
    // Sit: 28 words (shit, sit, babysit, resit, housesit, birdsit). Like Bind, except in 1 cell: verb past.
    conjugation!("sit", 28, "~i~", like Bind;
    stem verb: "~i~- like set";
    verb: _, _, "~a~", _, _;
    ),
    // Hold: 25 words (behold, hold, uphold, withhold, stranglehold, handhold). Like Throw, except in 1 cell: verb pp.
    conjugation!("hold", 25, "~o~", like Throw;
    verb: _, _, _, _, derived;
    ),
    // Stand: 23 words (stand, misunderstand, understand, withstand, handstand, headstand).
    conjugation!("stand", 23, "~an~";
    stem verb: "~an~- like burn";
    verb: _, _, "~oo~", _, _;
    ),
    // Draw: 23 words (draw, overdraw, withdraw, updraw, redraw, outdraw).
    conjugation!("draw", 23, "~a~";
    stem verb: "~a~- like burn";
    verb: _, _, "~e~", _, "~a~n";
    ),
    // Tell: 23 words (foretell, outsell, oversell, resell, sell, tell).
    conjugation!("tell", 23, "~e~l";
    stem verb: "~e~l- like burn";
    verb: _, _, "~o~d", _, _;
    ),
    // Backbite: 21 words (bite, hide, frostbite, backbite, backslide, overbite). Like Settle, except in 2 cells: verb past, verb pp.
    conjugation!("backbite", 21, "~e", like Settle;
    verb: _, _, "~", _, "~ten";
    ),
    // Make: 20 words (make, remake, matchmake, unmake, premake, merrymake).
    conjugation!("make", 20, "~ke";
    stem verb: "~k- like settle";
    verb: _, _, "~de", _, _;
    ),
    // Give: 18 words (give, forgive, upgive, foregive, misgive, regive). Like Drive, except in 1 cell: verb past.
    conjugation!("give", 18, "~i~e", like Drive;
    verb: _, _, "~a~e", _, _;
    ),
    // Mislead: 18 words (mislead, uplead, colead, cheerlead, underlead, aread). Like Draw, except in 2 cells: verb past, verb pp.
    conjugation!("mislead", 18, "~a~", like Draw;
    verb: _, _, "~~", _, derived;
    ),
    // Run: 18 words (outrun, overrun, rerun, run, assrun, speedrun). Like Spring, except in 2 cells: verb _3sg, verb prp.
    conjugation!("run", 18, "~u~", like Spring as INF_AS_PP;
    verb: _, "~u~s", _, "~u~ning", _;
    ),
    // Think: 17 words (think, rethink, doublethink, outthink, crimethink, misthink).
    conjugation!("think", 17, "~ink";
    stem verb: "~ink- like burn";
    verb: _, _, "~ought", _, _;
    ),
    // Shoot: 17 words (overshoot, shoot, troubleshoot, undershoot, reshoot, sharpshoot). Like Hold, except in 1 cell: verb past.
    conjugation!("shoot", 17, "~o~", like Hold;
    verb: _, _, "~~", _, _;
    ),
    // Should: 17 words (may, can, got, will, shall, should). Like Burn, except in 0 cells: .
    conjugation!("should", 17, "~", like Burn as C3SG_AS_INF;
    ),
    // Forbear: 16 words (upbear, forebear, forbear, overbear, rebear, underbear). Like Break, except in 1 cell: verb pp.
    conjugation!("forbear", 16, "~ea~", like Break;
    verb: _, _, _, _, "~o~ne";
    ),
    // Overgo: 16 words (forego, forgo, undergo, togo, outgo, misgo).
    conjugation!("overgo", 16, "~go";
    stem verb: "~go- like pass";
    stem verb_past: "~- like overgo";
    ),
    // See: 15 words (see, foresee, oversee, forsee, sightsee, unsee).
    conjugation!("see", 15, "~ee";
    stem verb: "~ee- like burn";
    verb: _, _, "~aw", _, "~een";
    ),
    // Unbe: 15 words (be, beat, browbeat, wright, tyek, forbeat). Like Cast, except in 1 cell: verb pp.
    conjugation!("unbe", 15, "~", like Cast;
    verb: _, _, _, _, "~en";
    ),
    // Become: 15 words (become, come, overcome, misbecome, oncome, undercome).
    conjugation!("become", 15, "~o~e";
    stem verb: "~o~- like settle";
    verb: _, _, "~a~e", _, "~o~e";
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
    stem verb: "~o- like pass";
    verb: _, _, "~id", _, "~one";
    ),
    // Slide: 13 words (slide, chide, landslide, rede, betide, slite). Like Settle, except in 1 cell: verb past.
    conjugation!("slide", 13, "~e", like Settle;
    verb: _, _, "~", _, _;
    ),
    // Weave: 12 words (weave, interweave, reweave, forcleave, unweave, tocleave).
    conjugation!("weave", 12, "~~ave";
    stem verb: "~~av- like settle";
    stem verb_past: "~ov~- like break";
    ),
    // Abide: 11 words (abide, shine, bestride, outshine, overstride, beshine). Like Write, except in 1 cell: verb pp.
    conjugation!("abide", 11, "~i~e", like Write;
    verb: _, _, _, _, derived;
    ),
    // Quiz: 11 words (quiz, spaz, whiz, buz, wiz, diz).
    conjugation!("quiz", 11, "~";
    verb: "~", "~zes", "~zed", "~zing", _;
    ),
    // Buy: 11 words (buy, rebuy, overbuy, underbuy, prebuy, forbuy).
    conjugation!("buy", 11, "~~y";
    stem verb: "~~y- like burn";
    verb: _, _, "~o~ght", _, _;
    ),
    // Bring: 11 words (bring, upbring, underbring, forthbring, inbring, imbring).
    conjugation!("bring", 11, "~in~";
    stem verb: "~in~- like burn";
    verb: _, _, "~ou~ht", _, _;
    ),
    // Rec: 11 words (sic, floc, tic, vac, proc, fanfic). Like Burn, except in 2 cells: verb past, verb prp.
    conjugation!("rec", 11, "~", like Burn;
    verb: _, _, "~ced", "~cing", _;
    ),
    // Beteach: 11 words (teach, reteach, overteach, misteach, beteach, unteach).
    conjugation!("beteach", 11, "~e~c~";
    stem verb: "~e~c~- like pass";
    verb: _, _, "~~ug~t", _, _;
    ),
    // Octopus: 11 words (octopus, nonplus, dis, pus, res, dorcas).
    conjugation!("octopus", 11, "~";
    verb: "~", "~ses", "~sed", "~sing", _;
    ),
    // Catch: 10 words (catch, bycatch, recatch, overcatch, becatch, outcatch).
    conjugation!("catch", 10, "~tc~";
    stem verb: "~tc~- like pass";
    verb: _, _, "~ug~t", _, _;
    ),
    // Freeze: 10 words (freeze, unfreeze, refreeze, enfreeze, befreeze, prefreeze).
    conjugation!("freeze", 10, "~~eze";
    stem verb: "~~ez- like settle";
    stem verb_past: "~oz~- like break";
    ),
    // Hang: 9 words (hang, overhang, uphang, cliffhang, behang, unhang). Like Mislead, except in 1 cell: verb past.
    conjugation!("hang", 9, "~a~", like Mislead;
    verb: _, _, "~u~", _, _;
    ),
    // Lie: 9 words (lie, overlie, underlie, belie, superlie, forelie).
    conjugation!("lie", 9, "~ie";
    stem verb_past: "~a- like lie";
    verb: "~ie", "~ies", _, "~ying", _;
    ),
    // Seek: 9 words (seek, upseek, beseek, outseek, reseek, misseek).
    conjugation!("seek", 9, "~eek";
    stem verb: "~eek- like burn";
    verb: _, _, "~ought", _, _;
    ),
    // Strike: 9 words (strike, airstrike, restrike, counterstrike, overstrike, outstrike). Like Abide, except in 1 cell: verb past.
    conjugation!("strike", 9, "~i~e", like Abide;
    verb: _, _, "~uc~", _, _;
    ),
    // Fix: 8 words (born, gar, fix, daydream, wis, lern). Like Burn, except in 1 cell: verb past.
    conjugation!("fix", 8, "~", like Burn;
    verb: _, _, "~t", _, _;
    ),
    // Forget: 8 words (forget, beget, unbeget, misget, unforget, rebeget).
    conjugation!("forget", 8, "~e~";
    stem verb: "~e~- like set";
    verb: _, _, "~o~", _, "~o~ten";
    ),
    // Leave: 8 words (leave, cleave, beleave, toreave, dreave, forleave).
    conjugation!("leave", 8, "~ave";
    stem verb: "~av- like settle";
    verb: _, _, "~ft", _, _;
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
    // Gas: 7 words (gas, yes, teargas, outgas, degas, regas). Like Octopus, except in 1 cell: verb _3sg.
    conjugation!("gas", 7, "~", like Octopus;
    verb: _, "~es|~ses", _, _, _;
    ),
    // Dig: 7 words (dig, underdig, undig, redig, misdig, outdig). Like Wring, except in 1 cell: verb prp.
    conjugation!("dig", 7, "~i~", like Wring;
    verb: _, _, _, "~i~ging", _;
    ),
    // Light: 7 words (light, flashlight, twilight, floodlight, prelight, overlight).
    conjugation!("light", 7, "~gh~";
    stem verb: "~gh~- like burn";
    verb: _, _, "~~|~gh~ed", _, _;
    ),
    // Win: 6 words (win, overwin, rewin, outwin, bewin, electrowin). Like Spin, except in 1 cell: verb past.
    conjugation!("win", 6, "~i~", like Spin;
    verb: _, _, "~o~", _, _;
    ),
    // Wake: 6 words (awake, wake, reawake, forcarve, rewake, outwake). Like Take, except in 2 cells: verb past, verb pp.
    conjugation!("wake", 6, "~a~e", like Take;
    verb: _, _, "~o~e|~a~ed", _, "~o~en|~a~ed";
    ),
    // Choose: 6 words (choose, forechoose, rechoose, mischoose, unchoose, prechoose). Like Become, except in 0 cells: .
    conjugation!("choose", 6, "~o~e", like Become;
    stem verb_past: "~~e- like break";
    ),
    // Fly: 6 words (fly, overfly, outfly, refly, befly, upfly). Like Carry, except in 2 cells: verb past, verb pp.
    conjugation!("fly", 6, "~y", like Carry;
    verb: _, _, "~ew", _, "~own";
    ),
    // Larn: 6 words (de, dee, myek, deek, larn, stott). Like Burn, except in 1 cell: verb prp.
    conjugation!("larn", 6, "~", like Burn;
    verb: _, _, _, "~in", _;
    ),
    // Lose: 6 words (lose, kape, beclepe, unlose, arede, relose). Like Settle, except in 1 cell: verb past.
    conjugation!("lose", 6, "~e", like Settle;
    verb: _, _, "~t", _, _;
    ),
    // Get: 5 words (get, reget, unget, overget, underget). Like Forget, except in 1 cell: verb pp.
    conjugation!("get", 5, "~e~", like Forget;
    verb: _, _, _, _, "~o~|~o~ten";
    ),
    // Flee: 4 words (flee, shoe, beflee, outflee). Like Settle, except in 1 cell: verb past.
    conjugation!("flee", 4, "~e", like Settle;
    stem verb: "~e- like burn";
    verb: _, _, "~d", _, _;
    ),
    // Begin: 4 words (begin, rin, rebegin, atrin). Like Spring, except in 1 cell: verb prp.
    conjugation!("begin", 4, "~i~", like Spring;
    verb: _, _, _, "~i~ning", _;
    ),
    // Might: 4 words (might, must, gaf, wfh). Like Set, except in 1 cell: verb _3sg.
    conjugation!("might", 4, "~", like Set as PRP_AS_PAST;
    verb: _, "~", _, _, _;
    ),
    // Slay: 3 words (slay, foreslay, reslay).
    conjugation!("slay", 3, "~ay";
    stem verb: "~ay- like burn";
    verb: _, _, "~ew", _, "~ain";
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
